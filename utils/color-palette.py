#!/bin/env python

import typing as t
import os
import pathlib
from PIL import Image

HEIGHT = 256
PATH_REPO = pathlib.Path(__file__).parent.parent.resolve()
PATH_OUTPUT = PATH_REPO.joinpath('output')
PATH_OUTPUT_DEFLATED = PATH_OUTPUT.joinpath('deflated')
PATH_OUTPUT_COLOR_PALETTE = PATH_OUTPUT.joinpath('parsed').joinpath('color-palettes')

def transform_color(color: int) -> t.Tuple[int, int, int]:
    R = (color & 0xF00) >> 8
    G = (color & 0x0F0) >> 4
    B = (color & 0x00F)
    return (
        R << 4 | R,
        G << 4 | G,
        B << 4 | B
    )

def generate_color_palette(path_palette_file: pathlib.Path, path_output_image: pathlib.Path) -> None:
    with open(path_palette_file, 'rb') as f:
        SIZE = os.path.getsize(path_palette_file) // 4
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
        image.save(path_output_image)

if __name__ == "__main__":
    os.makedirs(PATH_OUTPUT_COLOR_PALETTE, exist_ok=True)

    for f in os.listdir(PATH_OUTPUT_DEFLATED):
        PATH_FILE = PATH_OUTPUT_DEFLATED.joinpath(f)
        if os.path.getsize(PATH_FILE) == 32 * 1024:
            # May be a color palette file
            generate_color_palette(
                PATH_FILE,
                PATH_OUTPUT_COLOR_PALETTE.joinpath(os.path.splitext(f)[0] + '.png')
            )
