#!/usr/bin/env python3
# pip3 install pyusb
import usb.core
import usb.util
import struct
import time

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


def write(fin, fout, rom, data: bytes):
    # b'1b240000' b'010000000001ef1300000000' type: 0 size: 1048576    name: WINBOND/W25X80A
    # b'17240000' b'010000000001ef1300000000' type: 0 size: 1048576    name: WINBOND/W25X80AL
    # b'17210000' b'010000000001ef1300000000' type: 0 size: 1048576    name: WINBOND/W25X80L
    # r 11 0a 01 00 00 00 00 00 10 00 00 03 00 00 00 A
    # r 11 0a 01 00 00 00 00 00 10 00 00 03 00 00 00 AL
    # r 11 0a 01 00 00 00 00 00 10 00 00 03 00 00 00 L
    # w 12 0c 01 00 00 00 00 00 10 00 00 01 00 03 00 00 00 A
    # w 12 0c 01 00 00 00 00 00 10 00 00 01 00 03 00 00 00 AL
    # w 12 0c 01 00 00 00 00 00 10 00 00 01 00 03 00 00 00 L

    # write: 12 0c 02 00 00 00 00 00 00 00 10 00 01 14 00 01 00
    # read: 12 01 01
    # 12 0c 02 00 00 00 00 00 00 00 10 00 01 14 00 00 00 <-  16b 24C00 3V
    # 12 0c 02 00 00 00 00 00 02 00 00 00 80 24 00 00 00 <- 128k 24C1024 3V
    # 12 0c 02 00 00 00 00 00 02 00 00 00 80 24 00 01 00 <- 128k 24C1024 5V
    # 12 0c 01 00 00 00 00 00 10 00 00 01 00 03 00 00 00 <-   1M W25X80L
    # 12 0c 01 00 00 00 00 00 40 00 00 01 00 03 00 00 00 <-   4M AT25DF321
    # 12 0c 01 00 00 00 00 01 00 00 00 01 00 03 00 00 00 <-  16M W25Q128BV
    TODO = [0x24, 0x00]
    TODO2 = [0x00, 0x01]
    cmd = CMD_WRITE +\
        [rom_type[rom]] +\
        [0x00, 0x00, 0x00, 0x00] + \
        list(struct.pack('>i', len(data))) +\
        TODO2 +\
        TODO +\
        [voltage[3]] +\
        [0x00]
    fout.write(cmd)
    time.sleep(SLEEP)

    pass


def read(fin, fout, rom, size):
    # 24XX
    #  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e
    # 11 0a 02 00 00 00 00 00 00 00 10 14 00 01 00 <-  16b MIC24LC00
    # 11 0a 02 00 00 00 00 00 00 01 00 14 00 01 00 <- 256b AT24C02
    # 11 0a 02 00 00 00 00 00 00 04 00 14 01 00 00 <-   1k 24C08 3V
    # 11 0a 02 00 00 00 00 00 00 08 00 14 00 01 00 <-   2k MIC24LC16B
    # 11 0a 02 00 00 00 00 00 00 08 00 14 00 01 00 <-   2k AT24C16
    # 11 0a 02 00 00 00 00 00 00 10 00 24 01 00 00 <-   4k 24C32 3V
    # 11 0a 02 00 00 00 00 00 00 20 00 24 00 01 00 <-   8k ST24C64
    # 11 0a 02 00 00 00 00 00 00 20 00 24 00 00 00 <-   8k 24C64 3V
    # 11 0a 02 00 00 00 00 00 00 20 00 24 00 01 00 <-   8k 24C64 5V
    # 11 0a 02 00 00 00 00 00 00 40 00 24 00 01 00 <-  16k MIC24LC128
    # 11 0a 02 00 00 00 00 00 00 80 00 24 00 01 00 <-  32k MIC24LC256
    # 11 0a 02 00 00 00 00 00 01 00 00 24 00 01 00 <-  64k MIC24LC512
    # 11 0a 02 00 00 00 00 00 01 00 00 24 00 01 00 <-  64k MIC24FC512
    # 11 0a 02 00 00 00 00 00 01 00 00 24 00 01 00 <-  64k AT24C512A
    # 11 0a 02 00 00 00 00 00 02 00 00 24 00 01 00 <- 128k MIC24AA1024
    # 25XX
    # 11 0a 03 00 00 00 00 00 00 00 80 01 00 01 00 <- 128b 25010
    # 11 0a 03 00 00 00 00 00 00 02 00 01 00 01 00 <- 512b 25040
    # 11 0a 03 00 00 00 00 00 00 04 00 11 00 01 00 <-   1k 25080
    # 11 0a 03 00 00 00 00 00 00 20 00 11 00 01 00 <-   8k 25640
    # 11 0a 03 00 00 00 00 00 00 40 00 11 00 01 00 <-  16k 25128
    # 93XX
    # 11 0a 04 00 00 00 00 00 00 08 00 a8 01 01 00 <- 1Kx16b 93C86
    # SPI
    # 11 0a 01 00 00 00 00 00 01 00 00 03 01 00 00 <-  64k 25X512
    # 11 0a 01 00 00 00 00 00 10 00 00 03 00 00 00 <-   1M W25X80L
    # 11 0a 01 00 00 00 00 00 40 00 00 03 00 00 00 <-   4M AT25DF321
    # 11 0a 01 00 00 00 00 01 00 00 00 03 00 00 00 <-  16M W25Q128BV

    # resp: 11 01 00: OK
    # resp: 11 01 02: Read chip error!
    TODO = [0x24, 0x00]
    cmd = CMD_READ +\
        [rom_type[rom]] +\
        [0x00, 0x00, 0x00, 0x00] + \
        list(struct.pack('>i', size)) +\
        TODO +\
        [voltage[3]] +\
        [0x00]
    fout.write(cmd)
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
    (fout, fin) = usb_open(VID, PID)
    # version(fin, fout)
    # serial(fin, fout)
    # selftest(fin, fout)
    read(fin, fout, '24XX', 64*1024)
    # read(fin, fout, '24XX', 16)
