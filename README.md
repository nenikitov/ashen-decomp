# Ashen source port

A playable source port of an obscure NGage game Ashen

## How to run?

- Place the ROM
    ```sh
    mkdir ./rom/
    cp /path/to/packfile.dat ./rom/packfile.dat
    ```
- Parse files (output will be in `output/parsed/`)
    ```sh
    cargo test --release -- --ignored parse_rom_packfile
    cargo test --release -- --ignored parse_rom_asset
    ```

## File structure

### Overview

- File is composed out of 3 parts
    - Header
    - File declarations
    - Data
- Some data is compressed using `zlib` algorithm, some isn't
- All data is stored in Little-endian

### Header

| Size (bytes) | Purpose          |
| ------------ | ---------------- |
| `4`          | Signature `PMAN` |
| `4`          | Number of files  |
| `56`         | Copyright        |

### File declarations

- This structure is repeated for every file in the packfile

| Size (bytes) | Purpose                        |
| ------------ | ------------------------------ |
| `4`          | Padding? Always `00 00 00 00`  |
| `4`          | Offset of the file in packfile |
| `4`          | Size of the file               |
| `4`          | Padding? Always `00 00 00 00`  |

### Data

- If data is compressed using zlib

| Size (bytes) | Purpose                |
| ------------ | ---------------------- |
| `2`          | Signature `ZL`         |
| `3`          | Size when uncompressed |
| `*`          | Zlib stream            |

- If data is not compressed, just data stream

### Known file declarations

**⚠️ WARNING ⚠️**

I use the packfile that comes with Ashen 1.06.
Your packfile may have different offsets to files, I didn't test with different versions.

Notes:
- **⚠️** - Unknown file
- **🔎** - Unseen content

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

### Known file formats

### Text bank

- Starts with `9B 01 00 00`
- The rest is encoded in UTF16
- All text chunks are separated with `\r\r`
- There are weird characters like `20 20` which are probably to control in-game scripts

### Color palette

- Is a collection of 4 byte integers
- Every integer is the 12-bit color
- If transformed into 256x32 image, shows colored arcs similar to the Quake color palette

### Entity

- Groups texture, model, and possibly animation data together
- Textures are stored as 8 bit integers that are indeces of the color on the color palette

### Music

- Another collection of Zlib files concatinated together
- Music is probably stored as OGG, instruments separate from composition
- **TODO** Document this better
- Segments -> files
- 1st segment - OST
    - 7
    - 1
    - 8
    - 3
    - 5
    - 4
    - 2
    - 7
    - victory
    - concept
    - death
    - load
    - main menu

## Discoveries

- Test level for enemies
    - There is a pool with fish enemies, which I don't remember seeing in the game
- Test level ???
    - Collision by itself works with other level geometry, geometry by itself also works, but not together

## Resources

- [Post by HoRRoR](http://www.emu-land.net/forum/index.php?topic=49753.0)
- [Debug Windows build with symbols](https://archive.org/details/Nokia_N-Gage_Ashen_v1.0.6_Windows_Build)

