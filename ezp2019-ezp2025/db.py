#!/usr/bin/env python
import argparse
import struct
import csv
import os


def parse_file(filename):
    with open(filename, 'r+b') as f:
        while True:
            entry = f.read(0x44) # 0x4c on ezp2020?
            if not entry:
                break
            (info, chip_id, size, flash_page, chip_class, algo, delay, eeprom_extend, eeprom, unk2, eeprom_page, voltage) = struct.unpack('48s I I h B B h H B B B B', entry)
            try:
                info = info.decode('utf-8').split("\x00")[0].split(',')
                yield {
                    'type' : info[0],
                    'manufacturer' : info[1],
                    'name' : info[2],
                    'chip_id' : chip_id,
                    'size': size,
                    'flash_page': flash_page,
                    'chip_class': chip_class,
                    'algo': algo,
                    'delay': delay,
                    'eeprom_extend': eeprom_extend,
                    'eeprom': eeprom,
                    'unk2': unk2,
                    'eeprom_page': eeprom_page,
                    'voltage': voltage
                }
            except:
                print('error')

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('file', type=str)
    args = parser.parse_args()
    csvfile =  os.path.splitext(args.file )[0]+'.csv' 
    with open(csvfile, 'w') as csvfile:
        writer = None
        for x in parse_file(args.file):
            if not writer:
                writer = csv.DictWriter(csvfile, fieldnames=x.keys())
                writer.writeheader()
            writer.writerow(x)
