<style>
.warning {
    color: orange;
    font-weight: bold;
}
</style>


# Ashen PackFile unpacker

An unarchiver of an obscure NGage game Ashen

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

| Size (bytes) | Purpose                        |
| ------------ | ------------------------------ |
| `2`          | Signature `ZL`                 |
| `3`          | Size when uncompressed         |
| `*`          | Zlib stream                    |

- If data is not compressed, just data stream

### Known file declarations

<div class="warning">!!! WARNING !!!</div>

I use the packfile that comes with Ashen 1.06. Your packfile may have different offsets to files, I didn't test with different versions.

| Address (HEX) | Output file | Purpose                                         |
| ------------- | ----------- | ----------------------------------------------- |
| `030`         |             | <span class="warning">???</span>                |
| `040`         |             | <span class="warning">???</span>                |
| `050`         |             | <span class="warning">???</span>                |
| `060`         |             | <span class="warning">???</span>                |
| `070`         |             | <span class="warning">???</span>                |
| `080`         |             | <span class="warning">???</span>                |
| `090`         |             | <span class="warning">???</span>                |
| `0A0`         |             | <span class="warning">???</span>                |
| `0B0`         |             | <span class="warning">???</span>                |
| `0C0`         |             | <span class="warning">???</span>                |
| `0D0`         |             | <span class="warning">???</span>                |
| `0E0`         |             | <span class="warning">???</span>                |
| `0F0`         |             | <span class="warning">???</span>                |
| `100`         |             | <span class="warning">???</span>                |
| `110`         |             | <span class="warning">???</span>                |
| `120`         |             | <span class="warning">???</span>                |
| `130`         |             | <span class="warning">???</span>                |
| `140`         |             | <span class="warning">???</span>                |
| `150`         |             | <span class="warning">???</span>                |
| `160`         |             | <span class="warning">???</span>                |
| `170`         |             | <span class="warning">???</span>                |
| `180`         |             | <span class="warning">???</span>                |
| `190`         |             | <span class="warning">???</span>                |
| `1A0`         |             | <span class="warning">???</span>                |
| `1B0`         |             | <span class="warning">???</span>                |
| `1C0`         |             | <span class="warning">???</span>                |
| `1D0`         |             | <span class="warning">???</span>                |
| `1E0`         |             | <span class="warning">???</span>                |
| `1F0`         |             | <span class="warning">???</span>                |
| `200`         |             | <span class="warning">???</span>                |
| `210`         |             | <span class="warning">???</span>                |
| `220`         |             | <span class="warning">???</span>                |
| `230`         |             | <span class="warning">???</span>                |
| `240`         |             | <span class="warning">???</span>                |
| `250`         |             | <span class="warning">???</span>                |
| `260`         |             | <span class="warning">???</span>                |
| `270`         |             | <span class="warning">???</span>                |
| `280`         |             | <span class="warning">???</span>                |
| `290`         |             | <span class="warning">???</span>                |
| `2A0`         |             | <span class="warning">???</span>                |
| `2B0`         |             | <span class="warning">???</span>                |
| `2C0`         |             | <span class="warning">???</span>                |
| `2D0`         |             | <span class="warning">???</span>                |
| `2E0`         |             | <span class="warning">???</span>                |
| `2F0`         |             | <span class="warning">???</span>                |
| `300`         |             | <span class="warning">???</span>                |
| `310`         |             | <span class="warning">???</span>                |
| `320`         |             | <span class="warning">???</span>                |
| `330`         |             | <span class="warning">???</span>                |
| `340`         |             | <span class="warning">???</span>                |
| `350`         |             | <span class="warning">???</span>                |
| `360`         |             | <span class="warning">???</span>                |
| `370`         |             | <span class="warning">???</span>                |
| `380`         |             | <span class="warning">???</span>                |
| `390`         |             | <span class="warning">???</span>                |
| `3A0`         |             | <span class="warning">???</span>                |
| `3B0`         |             | <span class="warning">???</span>                |
| `3C0`         |             | <span class="warning">???</span>                |
| `3D0`         |             | <span class="warning">???</span>                |
| `3E0`         |             | <span class="warning">???</span>                |
| `3F0`         |             | <span class="warning">???</span>                |
| `400`         |             | <span class="warning">???</span>                |
| `410`         |             | <span class="warning">???</span>                |
| `420`         |             | <span class="warning">???</span>                |
| `430`         |             | <span class="warning">???</span>                |
| `440`         |             | <span class="warning">???</span>                |
| `450`         |             | <span class="warning">???</span>                |
| `460`         |             | <span class="warning">???</span>                |
| `470`         |             | <span class="warning">???</span>                |
| `480`         |             | <span class="warning">???</span>                |
| `490`         |             | <span class="warning">???</span>                |
| `4A0`         |             | <span class="warning">???</span>                |
| `4B0`         |             | <span class="warning">???</span>                |
| `4C0`         |             | <span class="warning">???</span>                |
| `4D0`         |             | <span class="warning">???</span>                |
| `4E0`         |             | <span class="warning">???</span>                |
| `4F0`         |             | <span class="warning">???</span>                |
| `500`         |             | <span class="warning">Chapter 1.1 geometry      |
| `510`         |             | <span class="warning">Chapter 1.1 collision     |
| `520`         |             | <span class="warning">Chapter 1.1 waypoint      |
| `530`         |             | <span class="warning">Chapter 1.1 color palette |
| `540`         |             | <span class="warning">Chapter 1.1 </span>       |
| `550`         |             | <span class="warning">???</span>                |
| `560`         |             | <span class="warning">???</span>                |
| `570`         |             | <span class="warning">???</span>                |
| `580`         |             | <span class="warning">???</span>                |
| `590`         |             | <span class="warning">???</span>                |
| `5A0`         |             | <span class="warning">???</span>                |
| `5B0`         |             | <span class="warning">???</span>                |
| `5C0`         |             | <span class="warning">???</span>                |
| `5D0`         |             | <span class="warning">???</span>                |
| `5E0`         |             | <span class="warning">???</span>                |
| `5F0`         |             | <span class="warning">???</span>                |
| `600`         |             | <span class="warning">???</span>                |
| `610`         |             | <span class="warning">???</span>                |
| `620`         |             | <span class="warning">???</span>                |
| `630`         |             | <span class="warning">???</span>                |
| `640`         |             | <span class="warning">???</span>                |
| `650`         |             | <span class="warning">???</span>                |
| `660`         |             | <span class="warning">???</span>                |
| `670`         |             | <span class="warning">???</span>                |
| `680`         |             | <span class="warning">???</span>                |
| `690`         |             | <span class="warning">???</span>                |
| `6A0`         |             | <span class="warning">???</span>                |
| `6B0`         |             | <span class="warning">???</span>                |
| `6C0`         |             | <span class="warning">???</span>                |
| `6D0`         |             | <span class="warning">???</span>                |
| `6E0`         |             | <span class="warning">???</span>                |
| `6F0`         |             | <span class="warning">???</span>                |
| `700`         |             | <span class="warning">???</span>                |
| `710`         |             | <span class="warning">???</span>                |
| `720`         |             | <span class="warning">???</span>                |
| `730`         |             | <span class="warning">???</span>                |
| `740`         |             | <span class="warning">???</span>                |
| `750`         |             | <span class="warning">???</span>                |
| `760`         |             | <span class="warning">???</span>                |
| `770`         |             | <span class="warning">???</span>                |
| `780`         |             | <span class="warning">???</span>                |
| `790`         |             | <span class="warning">???</span>                |
| `7A0`         |             | <span class="warning">???</span>                |
| `7B0`         |             | <span class="warning">???</span>                |
| `7C0`         |             | <span class="warning">???</span>                |
| `7D0`         |             | <span class="warning">???</span>                |
| `7E0`         |             | <span class="warning">???</span>                |
| `7F0`         |             | <span class="warning">???</span>                |
| `800`         |             | <span class="warning">???</span>                |
| `810`         |             | <span class="warning">???</span>                |
| `820`         |             | <span class="warning">???</span>                |
| `830`         |             | <span class="warning">???</span>                |
| `840`         |             | <span class="warning">???</span>                |
| `850`         |             | <span class="warning">???</span>                |
| `860`         |             | <span class="warning">???</span>                |
| `870`         |             | <span class="warning">???</span>                |
| `880`         |             | <span class="warning">???</span>                |
| `890`         |             | <span class="warning">???</span>                |
| `8A0`         |             | <span class="warning">???</span>                |
| `8B0`         |             | <span class="warning">???</span>                |
| `8C0`         |             | <span class="warning">???</span>                |
| `8D0`         |             | <span class="warning">???</span>                |
| `8E0`         |             | <span class="warning">???</span>                |
| `8F0`         |             | <span class="warning">???</span>                |
| `900`         |             | <span class="warning">???</span>                |
| `910`         |             | <span class="warning">???</span>                |
| `920`         |             | <span class="warning">???</span>                |
| `930`         |             | <span class="warning">???</span>                |
| `940`         |             | <span class="warning">???</span>                |
| `950`         |             | <span class="warning">???</span>                |
| `960`         |             | <span class="warning">???</span>                |
| `970`         |             | <span class="warning">???</span>                |
| `980`         |             | <span class="warning">???</span>                |
| `990`         |             | <span class="warning">???</span>                |
| `9A0`         |             | <span class="warning">???</span>                |
| `9B0`         |             | <span class="warning">???</span>                |
| `9C0`         |             | Text - English UK                               |
| `9D0`         |             | Text - English US                               |
| `9E0`         |             | Text - French                                   |
| `9F0`         |             | Text - Italian                                  |
| `A00`         |             | Text - German                                   |
| `A10`         |             | Text - Spanish                                  |

## List of files that I made sense of

### Text banks

- `D9924C.zlib` - English
- `D9B808.zlib` - English
- `D9DDC4.zlib` - French
- `DA0904.zlib` - Italian
- `DA34B8.zlib` - German
- `DA5B9C.zlib` - Spanish

## Level data

- For level 1
    - `500` - Map data
    - `510` - Collisions
    - `520` - Waypoints? Objectives
    - `530` - Color palette
    - `540` - ?
    - `550` - ?
- For DM 1
    - Map data
    - Collisions
    - Color palette
    - ...

### Collision files

- `294D84.zlib` - e1m1
- `2E9D88.zlib` - e1m2
- `357648.zlib` - e2m1
- `3DF9DC.zlib` - e2m2
- `458220.zlib` - e3m1
- `4BFCAC.zlib` - e3m2
- `52EAB4.zlib` - e4m1
- `58EFC8.zlib` - e4m2
- `5F60B8.zlib` - e5m1
- `683FD4.zlib` - e5m2
- `6F23F4.zlib` - e6m1
- `775A2C.zlib` - e7m1
- `7E3990.zlib` - e8m1
- `823F54.zlib` - dm1**?**
- `84F7D0.zlib` - dm2**?**
- `870414.zlib` - dm3**?**
- `8A8F2C.zlib` - dm4**?**
- `8CFF88.zlib` - ???? (no color palette, only collision data)

