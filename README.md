# Ashen reverse engineering

Reverse engineering of an N-Gage game [Ashen](https://en.wikipedia.org/wiki/Ashen_(2004_video_game))
- Can extract game resources

## Usage

### Setup

1. Place the ROM
    ```sh
    mkdir rom/
    cp /path/to/packfile.dat rom/packfile.dat
    ```

### Extracting files

File parsing is in test suite only, for now.

- Unpack game resources
    1. Run deflate test
        ```sh
        cargo test --release -- --ignored parse_rom_packfile
        ```
    - This will split and deflate game files into `output/deflated/` directory.
    - Files are named with the address as they appear in the [asset table](#file-structure).
    - Files that begin with decompression signature are automatically decompressed and named with `-deflated` suffix.
        This automatic decompression doesn't work well for collection assets such as textures.
- Parse resources (make sure to unpack first)
    1. Run parsing tests
        ```sh
        cargo test --release -- --ignored parse_rom_asset
        ```
    - This will parse select few game files into `output/parsed/` directory
    - Directions on how to opened parsed files are available [here](#file-formats-and-extraction)

## Roadmap

### File formats and extraction

- [x] Gamma table
    - **Purpose**
        - Look up table for brightness when changing gamma in settings
    - **Output format**
        - PNG image
- [x] Color map
    - **Purpose**
        - Look up table for 12-bit RGB colors, because textures are stored in 256 indexed color format
        - There is also ghost version for when you use Ghost Vision Goggles in game
        - There are unique color maps for monsters, pick-ups (and some other models), UI sprites, and levels
    - **Output format**
        - PNG image
- [x] Model
    - **Purpose**
        - Texture, geometry, and model data
    - **TO DO**
        - Figure out what to do with pre-calculated face and vertex normals
    - **Output format**
        - Blender Python script that can be executed with
            ```sh
            blender -P /path/to/script.py
            ```
        - Automatically sets up Cycles material and animations as shape keys in dope sheet's shape key editor
- [x] Sky
    - **Purpose**
        - Color map and skybox texture
    - **Output format**
        - PNG image
- [ ] Skins
- [ ] Level geometry
- [ ] Level collision
- [ ] Level waypoints
- [ ] Sprite
- [x] Texture
    - **Purpose**
        - Texture info - offsets, texture dimensions
        - Texture data - color indeces
    - **Output format**
        - PNG image
        - GIF image for animated textures
- [x] Music and sound effects
    - **Purpose**
        - Sound effects
        - Music
    - **TO DO**
        - Improve mixer with pitch, pan, and other sound effects
        - Support sustained instruments
        - Set correct tempo
    - **Output format**
        - WAV audio file
- [x] String table
    - **Purpose**
        - Printable strings in UI
    - **TO DO**
        - Figure out weird non-text characters (probably for controlling in-game events)
    - **Output format**
        - Plain text

## File structure

> [!IMPORTANT]
>
> I use the packfile that comes with Ashen 1.06.
> Your packfile may have different order, I didn't test with different versions.

| Address (HEX) | Asset                           |
|---------------|---------------------------------|
| `00`          | gamma table                     |
| `01`          | creature colormap               |
| `02`          | creature colormap (ghost)       |
| `03`          | ghost creature colormap (ghost) |
| `04`          | pickup colormap                 |
| `05`          | pickup colormap (ghost)         |
| `06`          | jacob colormap                  |
| `07`          | level colormap (ghost)          |
| `08`          | player hands colormap           |
| `09`          | player hands colormap (ghost)   |
| `0A`          | aquagore                        |
| `0B`          | broodmaw                        |
| `0C`          | cryptcrawler                    |
| `0D`          | firedeacon                      |
| `0E`          | hunter                          |
| `0F`          | psistalker                      |
| `10`          | stormfluke                      |
| `11`          | tentacle                        |
| `12`          | wraith                          |
| `13`          | primeentity                     |
| `14`          | playermodel                     |
| `15`          | vanessa                         |
| `16`          | rocket                          |
| `17`          | grenade                         |
| `18`          | fxblast                         |
| `19`          | aquagore shot                   |
| `1A`          | broodmaw shot                   |
| `1B`          | cryptcrawler shot               |
| `1C`          | firedeacon shot                 |
| `1D`          | gib generic 1                   |
| `1E`          | gib generic 2                   |
| `1F`          | gib generic 3                   |
| `20`          | blood generic 1                 |
| `21`          | charles                         |
| `22`          | human gib generic 1             |
| `23`          | human gib generic 2             |
| `24`          | human gib generic 3             |
| `25`          | pickup ammo pistol              |
| `26`          | pickup ammo double pistol       |
| `27`          | pickup ammo shotgun             |
| `28`          | pickup ammo machinegun          |
| `29`          | pickup ammo sniper              |
| `2A`          | pickup ammo grenade             |
| `2B`          | pickup ammo rocket              |
| `2C`          | pickup ammo gatlinggun          |
| `2D`          | pickup weapon pistol            |
| `2E`          | pickup weapon double pistol     |
| `2F`          | pickup weapon shotgun           |
| `30`          | pickup weapon machinegun        |
| `31`          | pickup weapon sniper            |
| `32`          | pickup weapon grenade           |
| `33`          | pickup weapon gatlinggun        |
| `34`          | pickup weapon shockwave         |
| `35`          | pickup ghostvision              |
| `36`          | pickup focitalisman             |
| `37`          | pickup letter                   |
| `38`          | pickup key1                     |
| `39`          | pickup flakjacket 25            |
| `3A`          | pickup flakjacket 50            |
| `3B`          | pickup flakjacket 100           |
| `3C`          | level1 sky                      |
| `3D`          | level2 sky                      |
| `3E`          | level3 sky                      |
| `3F`          | level4 sky                      |
| `40`          | level5 sky                      |
| `41`          | level6 sky                      |
| `42`          | level1 sky palette (ghost)      |
| `43`          | level2 sky palette (ghost)      |
| `44`          | level3 sky palette (ghost)      |
| `45`          | level4 sky palette (ghost)      |
| `46`          | level5 sky palette (ghost)      |
| `47`          | level6 sky palette (ghost)      |
| `48`          | jacob skin red                  |
| `49`          | jacob skin green                |
| `4A`          | jacob skin blue                 |
| `4B`          | jacob skin yellow               |
| `4C`          | level1a                         |
| `4D`          | level1a collision               |
| `4E`          | level1a waypointnav             |
| `4F`          | level1a colormap                |
| `50`          | level1b                         |
| `51`          | level1b collision               |
| `52`          | level1b waypointnav             |
| `53`          | level1b colormap                |
| `54`          | level2a                         |
| `55`          | level2a collision               |
| `56`          | level2a waypointnav             |
| `57`          | level2a colormap                |
| `58`          | level2b                         |
| `59`          | level2b collision               |
| `5A`          | level2b waypointnav             |
| `5B`          | level2b colormap                |
| `5C`          | level3a                         |
| `5D`          | level3a collision               |
| `5E`          | level3a waypointnav             |
| `5F`          | level3a colormap                |
| `60`          | level3b                         |
| `61`          | level3b collision               |
| `62`          | level3b waypointnav             |
| `63`          | level3b colormap                |
| `64`          | level4a                         |
| `65`          | level4a collision               |
| `66`          | level4a waypointnav             |
| `67`          | level4a colormap                |
| `68`          | level4b                         |
| `69`          | level4b collision               |
| `6A`          | level4b waypointnav             |
| `6B`          | level4b colormap                |
| `6C`          | level5a                         |
| `6D`          | level5a collision               |
| `6E`          | level5a waypointnav             |
| `6F`          | level5a colormap                |
| `70`          | level5b                         |
| `71`          | level5b collision               |
| `72`          | level5b waypointnav             |
| `73`          | level5b colormap                |
| `74`          | level6                          |
| `75`          | level6 collision                |
| `76`          | level6 waypointnav              |
| `77`          | level6 colormap                 |
| `78`          | level7                          |
| `79`          | level7 collision                |
| `7A`          | level7 waypointnav              |
| `7B`          | level7 colormap                 |
| `7C`          | level8                          |
| `7D`          | level8 collision                |
| `7E`          | level8 waypointnav              |
| `7F`          | level8 colormap                 |
| `80`          | leveldm1                        |
| `81`          | leveldm1 collision              |
| `82`          | leveldm1 colormap               |
| `83`          | leveldm2                        |
| `84`          | leveldm2 collision              |
| `85`          | leveldm2 colormap               |
| `86`          | leveldm3                        |
| `87`          | leveldm3 collision              |
| `88`          | leveldm3 colormap               |
| `89`          | leveldm4                        |
| `8A`          | leveldm4 collision              |
| `8B`          | leveldm4 colormap               |
| `8C`          | levelmonsters                   |
| `8D`          | levelmonsters collision         |
| `8E`          | levelmonsters waypointnav       |
| `8F`          | leveldoors                      |
| `90`          | leveldoors collision            |
| `91`          | leveldoors waypointnav          |
| `92`          | sprites                         |
| `93`          | texture info                    |
| `94`          | sprite texture info             |
| `95`          | textures                        |
| `96`          | sprite textures                 |
| `97`          | sound data                      |
| `98`          | stringtable english uk          |
| `99`          | stringtable english us          |
| `9A`          | stringtable french              |
| `9B`          | stringtable italian             |
| `9C`          | stringtable german              |
| `9D`          | stringtable spanish             |

## Discoveries

- Test level for enemies
    - There is a pool with fish enemies, which I don't remember seeing in the game
- Test level for doors
    - Collision by itself works with other level geometry, geometry by itself also works, but not together

## Resources

- [Post by HoRRoR](http://www.emu-land.net/forum/index.php?topic=49753.0)
- [Debug Windows build with symbols](https://archive.org/details/Nokia_N-Gage_Ashen_v1.0.6_Windows_Build)
- [Manual](https://ia804704.us.archive.org/25/items/n-gage-user-manuals/User%20Manual%20Ashen.pdf)
