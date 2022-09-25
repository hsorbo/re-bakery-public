#!/usr/bin/env python3
# pip3 install pyusb
import usb.core
import usb.util
import struct
import time
import binascii

VID = 0x10c4
PID = 0xf5a0
CMD_READ = [0x11, 0x0a]
CMD_WRITE = [0x12, 0x0c]
CMD_DETECT = [0x15, 0x00]
CMD_VERSION = [0x17, 0x00]
CMD_SERIAL = [0x18, 0x00]
CMD_SELF_TEST = [0xf3, 0x00]
SLEEP = 0.01

rom_type = {
    'SPI': 0x01,
    '24XX': 0x02,
    '25XX': 0x03,
    '93XX': 0x04
}

voltage = {
    3: 0x00,
    5: 0x01
}


def db_read_entries(filename: str):
    f = open(filename, 'rb')
    f.seek(0x64)
    while True:
        data = f.read(0x6C)
        if len(data) == 0:
            break
        yield data
    f.close()


def mystisk(type, size, should_mystisk, eet, ee93_bits):
    if type == 0:
        return [0x03, 0x00]
    elif type == 1:
        if (should_mystisk != 0xfe):
            return [36 if size > 0x800 else 20, 0x00]
        else:
            return [0x04, 0x00]
    elif type == 2:
        return [17 if size > 0x200 else 1, 0x00]
    elif type == 3:
        return [16 * eet | 8, 3 if ee93_bits == 8 else 1]


def mystisk_from_entry(db_entry):
    return mystisk(db_entry['type'],
                   db_entry['size'],
                   db_entry['should_mystisk'],
                   db_entry['eet'],
                   db_entry['ee93_bits'])


def parse_entry(entry: bytes):
    (type, prod, vend, unk1, voltage, size, unk_write_1, unk_write_2, unk4, should_mystisk, eet, ee93_bits) = struct.unpack(
        'I 40s 20s c b 2x I I h B B B B 26x', entry)
    # volt: 0x55 -> 85 -> 65
    # size: 0x58 -> 88 -> 68
    # should_mystisk: 0x63 -> 99 -> 79
    # todo: page size
    # AT24C01 -> 8B
    # AT24C16 -> 16B
    # AT24C32 -> 32B
    # AT24C512 -> 128B
    # AT24C1024B -> 256B
    # W25P20 -> 4KB

    return {
        'type': type,
        'prod': prod.decode('ascii').replace('\0', ''),
        'vend': vend.decode('ascii').replace('\0', ''),
        'unk1': unk1,
        'voltage': voltage,
        'size': size,
        'unk_write_1': unk_write_1,
        'unk_write_2': unk_write_2,
        'unk4': unk4,
        'should_mystisk': should_mystisk,
        'eet': eet,
        'ee93_bits': ee93_bits
    }


entries = [parse_entry(x) for x in db_read_entries('database/DateBase.bin')]


def db_dump():
    for w in db_read_entries('database/DateBase.bin'):

        entry = parse_entry(w)
        chip_voltage_is5v = False if entry['type'] == 0 else entry['voltage'] > 0x28

        cats = ["spi ", "ee24", "ee25", 'ee93']
        chip_category = cats[entry['type']]
        mystisk_lo = mystisk_from_entry(entry)

        # region that isnt padding nor name/vendor/type
        flags = w[64:-26]
        # print(str(binascii.hexlify(flags)))
        # print(entry)
        print(str(binascii.hexlify(flags)) + " " + entry['prod'])


def usb_open(vid, pid):
    # https://stackoverflow.com/questions/15074394/pyusb-dev-set-configuration
    dev = usb.core.find(idVendor=vid, idProduct=pid)
    if not dev:
        raise ValueError('Device not found')
    dev.reset()
    dev.set_configuration()
    cfg = dev.get_active_configuration()
    intf = cfg[(0, 0)]

    def _find_fd(intf, direction):
        def _dir(e):
            x = usb.util.endpoint_direction(e.bEndpointAddress)
            return x == direction
        return usb.util.find_descriptor(intf, custom_match=_dir)

    return (
        _find_fd(intf, usb.util.ENDPOINT_OUT),
        _find_fd(intf, usb.util.ENDPOINT_IN))


def version(fin, fout):
    fout.write(CMD_VERSION)
    time.sleep(SLEEP)
    # b'\x17\x1eEZP2010 V2.1\x00\x00\xc2\x85\x7f\x05\x12j\xc7\x12j1\xd2\x85"\xc2\xa5\x7f'
    resp = fin.read(102)
    parts = bytes(resp[2:]).split(b'\x00')
    print(parts[0].decode('ascii'))


def detect(fin, fout, rom):
    fout.write(CMD_DETECT+[rom_type[rom]])
    #  Detect chip error! = b'\x15\x01\x02'
    fin.read(5)


def selftest(fin, fout):
    fout.write(CMD_SELF_TEST)
    time.sleep(SLEEP)
    fin.read(2)
    print(fin.read(1000).tobytes())


def serial(fin, fout):
    fout.write(CMD_SERIAL)
    time.sleep(SLEEP)
    resp = fin.read(20).tobytes()
    print(resp[2:])


def create_write_cmd(db_entry):
    m = mystisk_from_entry(db_entry)
    b = 0x01 if db_entry['unk_write_1'] == 0 else db_entry['unk_write_2']
    # TODO2 = [0x00, b]
    # last seen: 0040fd94
    x = CMD_WRITE +\
        [db_entry['type']+1] +\
        [0x00, 0x00, 0x00, 0x00] + \
        list(struct.pack('>i', db_entry['size'])) +\
        list(struct.pack('>h', b)) +\
        m +\
        [0x01 if db_get_is5v(db_entry) else 0x00] +\
        [0x00]
    return bytes(x)


def write(fin, fout, db_entry, data: bytes):
    fout.write(create_write_cmd(db_entry))
    time.sleep(SLEEP)
    resp = fin.read(3).tobytes()
    if resp == bytes([0x12, 0x01, 0x01]):
        pass


def db_get_is5v(db_entry):
    return False if db_entry['type'] == 0 else db_entry['voltage'] > 0x28


def create_read_cmd(db_entry):
    m = mystisk_from_entry(db_entry)
    # In EZP2010.exe byte at 0xc can get "stuck" to 0x01 (normally 0x00)
    # when selecting a 93eeprom and then another eeprom category.
    x = CMD_READ +\
        [db_entry['type']+1] +\
        [0x00, 0x00, 0x00, 0x00] + \
        list(struct.pack('>i', db_entry['size'])) +\
        m +\
        [0x01 if db_get_is5v(db_entry) else 0x00] +\
        [0x00]
    return bytes(x)


def read(fin, fout, db_entry):
    # resp: 11 01 00: OK
    # resp: 11 01 02: Read chip error!
    size = db_entry['size']
    fout.write(create_read_cmd(db_entry))
    time.sleep(SLEEP)
    resp = fin.read(3).tobytes()
    if resp == bytes([0x11, 0x01, 0x00]):
        read_bytes = bytes()
        chunks = [4096]*(size//4096)
        if size % 4096 > 0:
            chunks += [size % 4096]
        for d in chunks:
            time.sleep(SLEEP)
            print("Reading %i bytes" % d)
            read_bytes += fin.read(d).tobytes()
        f = open("file.bin", 'wb')
        f.write(read_bytes)
        f.close()
        print("wrote file.bin (%i bytes)" % len(read_bytes))
    else:
        print("NOPE")


if __name__ == '__main__':
    db_dump()
    # (fout, fin) = usb_open(VID, PID)
    # version(fin, fout)
    # serial(fin, fout)
    # selftest(fin, fout)
    # read(fin, fout, '24XX', 64*1024)
    # read(fin, fout, '24XX', 16)
