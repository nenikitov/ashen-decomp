#!/bin/env python

import os
import pathlib
import zlib

PATH_REPO = pathlib.Path(__file__).parent.parent.resolve()
PATH_OUTPUT = PATH_REPO.joinpath('output')
PATH_OUTPUT_DEFLATED = PATH_OUTPUT.joinpath('deflated').joinpath('BBC974.dat')
PATH_OUTPUT_SOUND = PATH_OUTPUT.joinpath('parsed').joinpath('sound')



# D4 00 00 00 - offset
# 03 71 01 00 - size
# 00 00 00 00 - padding


SPLIT_BYTES = b'ZL'

if __name__ == "__main__":
    with open(PATH_OUTPUT_DEFLATED, 'rb') as f:
        FILE_BYTES = f.read()

        f.seek(0x34)
        NUM_FILES = int.from_bytes(f.read(4), 'little')

        for i in range(0, NUM_FILES):
            OFFSET = int.from_bytes(f.read(4), 'little')
            SIZE = int.from_bytes(f.read(4), 'little')
            f.read(4)

            DATA = FILE_BYTES[OFFSET: OFFSET + SIZE]
            DATA = zlib.decompress(DATA[5:])
            with open(PATH_OUTPUT_SOUND.joinpath(f'{OFFSET:03X}.dat'), 'wb') as o:
                o.write(DATA)

