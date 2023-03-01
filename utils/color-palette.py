#!/bin/env python

import typing as t
import os
import pathlib
from PIL import Image

HEIGHT = 256
NAME_COLOR_PALETTE = '7F5464.dat'
PATH_REPO = pathlib.Path(__file__).parent.parent.resolve()
PATH_OUTPUT = PATH_REPO.joinpath('output', 'deflated')

def transform_color(color: int) -> t.Tuple[int, int, int]:
    R = (color & 0xF00) >> 8
    G = (color & 0x0F0) >> 4
    B = (color & 0x00F)
    return (
        R << 4 | R,
        G << 4 | G,
        B << 4 | B
    )

if __name__ == "__main__":
    with open(PATH_OUTPUT.joinpath(NAME_COLOR_PALETTE), 'rb') as f:
        SIZE = os.path.getsize(PATH_OUTPUT.joinpath(NAME_COLOR_PALETTE)) // 4
        image = Image.new(
            'RGB',
            (HEIGHT, SIZE // HEIGHT)
        )
        # Get pixel colors
        for i in range(SIZE):
            COLOR = f.read(4)
            COLOR = int.from_bytes(COLOR, 'little')
            X = i % HEIGHT
            Y = i // HEIGHT
            image.putpixel(
                (X, Y),
                transform_color(COLOR)
            )

        # Save
        image.save(PATH_OUTPUT.parent.joinpath('test').joinpath('color_palette.png'))

