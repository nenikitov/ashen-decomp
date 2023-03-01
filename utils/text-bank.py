#!/bin/env python

import pathlib

class AshenString:
    SEPARATORS = {
        'chunk': '=====',
        'screen': '---'
    }

    def __init__(self, text: str):
        self.__text = (
            text
                .replace('\r\r', f'\n{AshenString.SEPARATORS["screen"]}\n')
                .replace('\r', '\n')
                # TODO find what \x20\x20 means
        )

    def __repr__(self) -> str:
        return self.__text

NAME_TEXT_BANK = 'D9924C.dat'
PATH_REPO = pathlib.Path(__file__).parent.parent.resolve()
PATH_OUTPUT = PATH_REPO.joinpath('output', 'deflated')

with open(PATH_OUTPUT.joinpath(NAME_TEXT_BANK), 'rb') as f:
    # Check if text file
    if f.read(4) != b'\x9b\1\0\0':
        print(f'File {NAME_TEXT_BANK} is not a text bank')
        exit(1)

    # Read text
    TEXT = f.read().decode('utf-16le')
    STRIGNS = [ AshenString(s) for s in TEXT.split('\0') ]

    # Output
    for s in STRIGNS:
        print(f'{s}\n{AshenString.SEPARATORS["chunk"]}')

