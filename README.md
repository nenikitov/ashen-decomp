# Ashen PackFile unpacker

An unarchiver of an obscure NGage game Ashen

## How to run?

- Place the ROM
    ```bash
    mkdir ./rom/
    cp /path/to/packfile.dat ./rom/packfile.dat
    ```
- Run
    ```bash
    cargo run --release
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

I use the packfile that comes with Ashen 1.06. Your packfile may have different offsets to files, I didn't test with different versions.

| Address (HEX) | Output file | Purpose                                   |
| ------------- | ----------- | ----------------------------------------- |
| `030`         |             | **⚠️ ??? ⚠️**                               |
| `040`         |             | **⚠️ ??? ⚠️**                               |
| `050`         |             | **⚠️ ??? ⚠️**                               |
| `060`         |             | **⚠️ ??? ⚠️**                               |
| `070`         |             | **⚠️ ??? ⚠️**                               |
| `080`         |             | **⚠️ ??? ⚠️**                               |
| `090`         |             | **⚠️ ??? ⚠️**                               |
| `0A0`         |             | **⚠️ ??? ⚠️**                               |
| `0B0`         |             | **⚠️ ??? ⚠️**                               |
| `0C0`         |             | **⚠️ ??? ⚠️**                               |
| `0D0`         |             | **⚠️ ??? ⚠️**                               |
| `0E0`         |             | **⚠️ ??? ⚠️**                               |
| `0F0`         |             | **⚠️ ??? ⚠️**                               |
| `100`         |             | **⚠️ ??? ⚠️**                               |
| `110`         |             | **⚠️ ??? ⚠️**                               |
| `120`         |             | **⚠️ ??? ⚠️**                               |
| `130`         |             | **⚠️ ??? ⚠️**                               |
| `140`         |             | **⚠️ ??? ⚠️**                               |
| `150`         |             | **⚠️ ??? ⚠️**                               |
| `160`         |             | **⚠️ ??? ⚠️**                               |
| `170`         |             | **⚠️ ??? ⚠️**                               |
| `180`         |             | **⚠️ ??? ⚠️**                               |
| `190`         |             | **⚠️ ??? ⚠️**                               |
| `1A0`         |             | **⚠️ ??? ⚠️**                               |
| `1B0`         |             | **⚠️ ??? ⚠️**                               |
| `1C0`         |             | **⚠️ ??? ⚠️**                               |
| `1D0`         |             | **⚠️ ??? ⚠️**                               |
| `1E0`         |             | **⚠️ ??? ⚠️**                               |
| `1F0`         |             | **⚠️ ??? ⚠️**                               |
| `200`         |             | **⚠️ ??? ⚠️**                               |
| `210`         |             | **⚠️ ??? ⚠️**                               |
| `220`         |             | **⚠️ ??? ⚠️**                               |
| `230`         |             | **⚠️ ??? ⚠️**                               |
| `240`         |             | **⚠️ ??? ⚠️**                               |
| `250`         |             | **⚠️ ??? ⚠️**                               |
| `260`         |             | **⚠️ ??? ⚠️**                               |
| `270`         |             | **⚠️ ??? ⚠️**                               |
| `280`         |             | **⚠️ ??? ⚠️**                               |
| `290`         |             | **⚠️ ??? ⚠️**                               |
| `2A0`         |             | **⚠️ ??? ⚠️**                               |
| `2B0`         |             | **⚠️ ??? ⚠️**                               |
| `2C0`         |             | **⚠️ ??? ⚠️**                               |
| `2D0`         |             | **⚠️ ??? ⚠️**                               |
| `2E0`         |             | **⚠️ ??? ⚠️**                               |
| `2F0`         |             | **⚠️ ??? ⚠️**                               |
| `300`         |             | **⚠️ ??? ⚠️**                               |
| `310`         |             | **⚠️ ??? ⚠️**                               |
| `320`         |             | **⚠️ ??? ⚠️**                               |
| `330`         |             | **⚠️ ??? ⚠️**                               |
| `340`         |             | **⚠️ ??? ⚠️**                               |
| `350`         |             | **⚠️ ??? ⚠️**                               |
| `360`         |             | **⚠️ ??? ⚠️**                               |
| `370`         |             | **⚠️ ??? ⚠️**                               |
| `380`         |             | **⚠️ ??? ⚠️**                               |
| `390`         |             | **⚠️ ??? ⚠️**                               |
| `3A0`         |             | **⚠️ ??? ⚠️**                               |
| `3B0`         |             | **⚠️ ??? ⚠️**                               |
| `3C0`         |             | **⚠️ ??? ⚠️**                               |
| `3D0`         |             | **⚠️ ??? ⚠️**                               |
| `3E0`         |             | **⚠️ ??? ⚠️**                               |
| `3F0`         |             | **⚠️ ??? ⚠️**                               |
| `400`         |             | **⚠️ ??? ⚠️**                               |
| `410`         |             | **⚠️ ??? ⚠️**                               |
| `420`         |             | **⚠️ ??? ⚠️**                               |
| `430`         |             | **⚠️ ??? ⚠️**                               |
| `440`         |             | **⚠️ ??? ⚠️**                               |
| `450`         |             | **⚠️ ??? ⚠️**                               |
| `460`         |             | **⚠️ ??? ⚠️**                               |
| `470`         |             | **⚠️ ??? ⚠️**                               |
| `480`         |             | **⚠️ ??? ⚠️**                               |
| `490`         |             | **⚠️ ??? ⚠️**                               |
| `4A0`         |             | **⚠️ ??? ⚠️**                               |
| `4B0`         |             | **⚠️ ??? ⚠️**                               |
| `4C0`         |             | **⚠️ ??? ⚠️**                               |
| `4D0`         |             | **⚠️ ??? ⚠️**                               |
| `4E0`         |             | **⚠️ ??? ⚠️**                               |
| `4F0`         |             | **⚠️ ??? ⚠️**                               |
| `500`         |             | Chapter 1.1 geometry                      |
| `510`         |             | Chapter 1.1 collision                     |
| `520`         |             | Chapter 1.1 waypoint                      |
| `530`         |             | Chapter 1.1 color palette                 |
| `540`         |             | Chapter 1.2 geometry                      |
| `550`         |             | Chapter 1.2 collision                     |
| `560`         |             | Chapter 1.2 waypoint                      |
| `570`         |             | Chapter 1.2 color palette                 |
| `580`         |             | Chapter 2.1 geometry                      |
| `590`         |             | Chapter 2.1 collision                     |
| `5A0`         |             | Chapter 2.1 waypoint                      |
| `5B0`         |             | Chapter 2.1 color palette                 |
| `5C0`         |             | Chapter 2.2 geometry                      |
| `5D0`         |             | Chapter 2.2 collision                     |
| `5E0`         |             | Chapter 2.2 waypoint                      |
| `5F0`         |             | Chapter 2.2 color palette                 |
| `600`         |             | Chapter 3.1 geometry                      |
| `610`         |             | Chapter 3.1 collision                     |
| `620`         |             | Chapter 3.1 waypoint                      |
| `630`         |             | Chapter 3.1 color palette                 |
| `640`         |             | Chapter 3.2 geometry                      |
| `650`         |             | Chapter 3.2 collision                     |
| `660`         |             | Chapter 3.2 waypoint                      |
| `670`         |             | Chapter 3.2 color palette                 |
| `680`         |             | Chapter 4.1 geometry                      |
| `690`         |             | Chapter 4.1 collision                     |
| `6A0`         |             | Chapter 4.1 waypoint                      |
| `6B0`         |             | Chapter 4.1 color palette                 |
| `6C0`         |             | Chapter 4.2 geometry                      |
| `6D0`         |             | Chapter 4.2 collision                     |
| `6E0`         |             | Chapter 4.2 waypoint                      |
| `6F0`         |             | Chapter 4.2 color palette                 |
| `700`         |             | Chapter 5.1 geometry                      |
| `710`         |             | Chapter 5.1 collision                     |
| `720`         |             | Chapter 5.1 waypoint                      |
| `730`         |             | Chapter 5.1 color palette                 |
| `740`         |             | Chapter 5.2 geometry                      |
| `750`         |             | Chapter 5.2 collision                     |
| `760`         |             | Chapter 5.2 waypoint                      |
| `770`         |             | Chapter 5.2 color palette                 |
| `780`         |             | Chapter 6.1 geometry                      |
| `790`         |             | Chapter 6.1 collision                     |
| `7A0`         |             | Chapter 6.1 waypoint                      |
| `7B0`         |             | Chapter 6.1 color palette                 |
| `7C0`         |             | Chapter 7.1 geometry                      |
| `7D0`         |             | Chapter 7.1 collision                     |
| `7E0`         |             | Chapter 7.1 waypoint                      |
| `7F0`         |             | Chapter 7.1 color palette                 |
| `800`         |             | Chapter 8.1 geometry                      |
| `810`         |             | Chapter 8.1 collision                     |
| `820`         |             | Chapter 8.1 waypoint                      |
| `830`         |             | Chapter 8.1 color palette                 |
| `840`         |             | Deathmatch 1 geometry                     |
| `850`         |             | Deathmatch 1 collision                    |
| `860`         |             | Deathmatch 1 color palette                |
| `870`         |             | Deathmatch 2 geometry                     |
| `880`         |             | Deathmatch 2 collision                    |
| `890`         |             | Deathmatch 2 color palette                |
| `8A0`         |             | Deathmatch 3 geometry                     |
| `8B0`         |             | Deathmatch 3 collision                    |
| `8C0`         |             | Deathmatch 3 color palette                |
| `8D0`         |             | Deathmatch 4 geometry                     |
| `8E0`         |             | Deathmatch 4 collision                    |
| `8F0`         |             | Deathmatch 4 color palette                |
| `900`         |             | 💡 Test level for enemies geometry        |
| `910`         |             | 💡 Test level for enemies collision       |
| `920`         |             | **💡⚠️ Test level for enemies waypoint ⚠️** |
| `930`         |             | 💡 Test level ??? geometry                |
| `940`         |             | **💡⚠️ Test level ??? collision ⚠️**        |
| `950`         |             | **💡⚠️ Test level ??? waypoint ⚠️**         |
| `960`         |             | **⚠️ ??? ⚠️**                               |
| `970`         |             | **⚠️ ??? ⚠️**                               |
| `980`         |             | **⚠️ ??? ⚠️**                               |
| `990`         |             | **⚠️ ??? ⚠️**                               |
| `9A0`         |             | **⚠️ ??? ⚠️**                               |
| `9B0`         |             | **⚠️ ??? ⚠️**                               |
| `9C0`         |             | Text - English UK                         |
| `9D0`         |             | Text - English US                         |
| `9E0`         |             | Text - French                             |
| `9F0`         |             | Text - Italian                            |
| `A00`         |             | Text - German                             |
| `A10`         |             | Text - Spanish                            |

## Discoveries

- Test level for enemies
    - There is a pool with fish enemies, which I don't remember seeing in the game
- Test level ???
    - Collision by itself works with other level geometry, geometry by itself also works, but not together

