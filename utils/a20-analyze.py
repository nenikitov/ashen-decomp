#!/bin/env python

import os


def format_hex(values: list[int]) -> str:
    return ' '.join([f'{v:0>2X}' for v in values]) if len(values) > 0 else 'NOTHING'

if __name__ == '__main__':
    # Get path to the file
    PATH_REPO = os.path.abspath(os.path.join(__file__, '..', '..'))
    PATH_A20 = os.path.join(PATH_REPO, 'output', 'A20.dat')
    PATH_A20_ANALYZED = os.path.join(PATH_REPO, 'output', 'A20_pattern.txt')
    SEQUENCE = list(range(0, 0xFF))

    # Check output
    if not os.path.isfile(PATH_A20):
        print('Run the extractor first')
        exit(1)

    # Analyze
    i = 0
    with open(PATH_A20, 'rb') as f, open(PATH_A20_ANALYZED, 'wt') as w:
        while DATA := list(f.read(256)):
            # Get removed and duplicated
            REMOVED = sorted(
                set(SEQUENCE) - set(DATA)
            )
            DUPLICATED = sorted(set(
                [ d for d in DATA if DATA.count(d) > 1]
            ))
            # Format
            print(i)
            print(f'    - Removed: {format_hex(REMOVED)}')
            print(f'    - Repeated: {format_hex(DUPLICATED)}')
            w.writelines([
                f'{i}\n',
                f'    - Removed: {format_hex(REMOVED)}\n',
                f'    - Repeated: {format_hex(DUPLICATED)}\n'
            ])
            i += 1


