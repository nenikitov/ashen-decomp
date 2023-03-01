#!/bin/env python

import typing as t
import math
import os
import pathlib
from PIL import Image

PATH_REPO = pathlib.Path(__file__).parent.parent.resolve()
PATH_OUTPUT = PATH_REPO.joinpath('output')
PATH_OUTPUT_DEFLATED = PATH_OUTPUT.joinpath('deflated')
PATH_OUTPUT_COLOR_PALETTES = PATH_OUTPUT.joinpath('parsed').joinpath('color-palettes')
PATH_OUTPUT_TEXTURES = PATH_OUTPUT.joinpath('parsed').joinpath('textures')

def generate_texture(palette: t.List[t.Tuple[int, int, int]], width: int, path_input: pathlib.Path, path_output_image: pathlib.Path):
    with open(path_input, 'rb') as f:
        SIZE = os.path.getsize(path_input)
        HEIGHT = math.ceil(SIZE / width)
        image = Image.new('RGB', (width, HEIGHT))
        for i in range(SIZE):
            COLOR = palette[int.from_bytes(f.read(1), 'little')]
            X = i % width
            Y = i // width
            image.putpixel(
                (X, Y),
                COLOR
            )
        image.save(path_output_image)

def get_palette(path_color_palette: pathlib.Path) -> t.List[t.Tuple[int, int, int]]:
    COLOR_PALETTE = Image.open(path_color_palette)
    return [
        COLOR_PALETTE.getpixel((i, 15))
        for i in range(0, COLOR_PALETTE.size[0])
    ]


if __name__ == "__main__":
    os.makedirs(PATH_OUTPUT_TEXTURES, exist_ok=True)

    for c in [ '1EF20.png', '2A89A4.png', '2EF20.png', '3EF20.png', '6F20.png' ]:
        # Get color palette
        COLOR_PALETTE = get_palette(PATH_OUTPUT_COLOR_PALETTES.joinpath(c))

        for f in os.listdir(PATH_OUTPUT_DEFLATED):
            # Read as texture
            PATH_FILE = PATH_OUTPUT_DEFLATED.joinpath(f)
            for s in [8, 16, 32, 64, 128, 256, 512]:
                generate_texture(
                    COLOR_PALETTE,
                    s,
                    PATH_FILE,
                    PATH_OUTPUT_TEXTURES.joinpath(
                        os.path.splitext(f)[0] + '_'
                            + str(s) + '_'
                            + os.path.splitext(c)[0] + '.png'
                    )
                )

