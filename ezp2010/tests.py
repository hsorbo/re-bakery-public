#!/usr/bin/env python
import unittest
import ezp
import binascii


class Tests(unittest.TestCase):

    def test_create_read_cmd(self):
        known = [
            #  0 1 2 3 4 5 6 7 8 9 a b c d e
            # 24XX
            ('110a02000000000000200024000000', '24C64 3V'),
            ('110a02000000000000200024000100', '24C64 5V'),
            ('110a02000000000000001014000100', 'MIC24LC00'),
            ('110a02000000000000010014000100', 'AT24C02'),
            ('110a02000000000000080014000100', 'MIC24LC16B'),
            ('110a02000000000000080014000100', 'AT24C16'),
            ('110a02000000000000100024000000', '24C32 3V'),
            ('110a02000000000000200024000100', 'ST24C64'),
            ('110a02000000000000400024000100', 'MIC24LC128'),
            ('110a02000000000000800024000100', 'MIC24LC256'),
            ('110a02000000000001000024000100', 'AT24C512A'),
            ('110a02000000000002000024000100', 'MIC24AA1024'),
            # 25XX
            ('110a03000000000000008001000100', '25010'),
            ('110a03000000000000020001000100', '25040'),
            ('110a03000000000000040011000100', '25080'),
            ('110a03000000000000200011000100', '25640'),
            ('110a03000000000000400011000100', '25128'),
            # 93XX
            ('110a04000000000000008068010100', 'AK93C45AV'),
            ('110a04000000000000008078030100', '93C46(8bit)'),
            ('110a04000000000000008068010100', '93C46(16bit)'),
            # SPI
            ('110a01000000000001000003000000', '25X512'),
            ('110a01000000000010000003000000', 'W25X80L'),
            ('110a01000000000040000003000000', 'AT25DF321'),
            ('110a01000000000100000003000000', 'W25Q128BV')
        ]

        for (expected_str, prod) in known:
            entries = list(filter(lambda x: x['prod'] == prod, ezp.entries))
            self.assertEqual(len(entries), 1)
            entry = entries[0]
            got = ezp.create_read_cmd(entry)
            expected = bytearray.fromhex(expected_str)
            self.assertEqual(got, expected)

    def test_create_write_cmd(self):
        known = [
            #  0 1 2 3 4 5 6 7 8 9 a b c d e
            # 24XX
            ('120c020000000000000010000114000000', '24C00 3V'),
            ('120c020000000000020000008024000000', '24C1024 3V'),
            ('120c020000000000020000008024000100', '24C1024 5V'),
            # 93XX
            ('120c030000000000000080000401000100', 'AT25010'),
            ('120c030000000000000800001011000100', 'MIC25LC160'),
            ('120c030000000000000400000811000100', 'ST25W08'),
            # 25XX
            ('120c040000000000000080000168010100', 'AK93C45AV'),
            ('120c0400000000000008000001b8030100', 'AT93C86(8bit)-SOP8'),
            ('120c040000000000000020000168010100', 'FM93C06AM8(16bit)'),
            ('120c0400000000000008000001a8010100', 'NSC93C86'),
            # SPI
            ('120c010000000000100000010003000000', 'W25X80L'),
            ('120c010000000000400000010003000000', 'AT25DF321'),
            ('120c010000000001000000010003000000', 'W25Q128BV')
        ]

        for (expected_str, prod) in known:
            entries = list(filter(lambda x: x['prod'] == prod, ezp.entries))
            self.assertEqual(len(entries), 1)
            entry = entries[0]
            got = ezp.create_write_cmd(entry)
            expected = bytearray.fromhex(expected_str)
            self.assertEqual(got, expected)

    def test_page_size(self):
        known = [
            ('AT24C01', 8),
            ('AT24C16', 16),
            ('AT24C32', 32),
            ('AT24C512', 128),
            ('AT24C1024B', 256),
            ('W25P20', 4096)
        ]
        print()
        for (prod, expected) in known:
            entries = list(filter(lambda x: x['prod'] == prod, ezp.entries))
            self.assertEqual(len(entries), 1)
            entry = entries[0]
            print(entry)


if __name__ == '__main__':
    unittest.main()
