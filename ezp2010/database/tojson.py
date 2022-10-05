#!/usr/bin/env python3
import struct
import json


def db_read_entries(filename: str):
    f = open(filename, 'rb')
    (entry_count) = struct.unpack('H', f.read(2))
    f.seek(0x64)
    while True:
        data = f.read(0x6C)
        if len(data) == 0:
            break
        yield data
    f.close()


def parse_entry(entry: bytes):
    (type, prod, vend, unk1, voltage, size, unk_write_1, unk_write_2, unk2, mystisk, ee93_flags, ee93_bits) = struct.unpack(
        'I 40s 20s b b 2x I I h B B B B 26x', entry)
    # volt: 0x55 -> 85 -> 65
    # size: 0x58 -> 88 -> 68
    # should_mystisk: 0x63 -> 99 -> 79
    types = ['SPI', '24XX', '25XX', '93XX']
    return {
        'type': types[type],
        'prod': prod.decode('ascii').replace('\0', ''),
        'vend': vend.decode('ascii').replace('\0', ''),
        'unk1': unk1,
        'voltage': voltage,
        'size': size,
        'unk_write_1': unk_write_1,
        'unk_write_2': unk_write_2,
        'unk2': unk2,
        'mystisk': mystisk,
        'ee93_flags': ee93_flags,
        'ee93_bits': ee93_bits
    }


foo = [parse_entry(x) for x in db_read_entries('DateBase.bin')]
print(json.dumps(foo, indent=2))
