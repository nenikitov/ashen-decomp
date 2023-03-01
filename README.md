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

Notes:
- **⚠️** - Unknown file
- **🔎** - Unseen content

| Address (HEX) | Output file  | Type            | Purpose                     | Notes   |
| ------------- | ------------ | --------------- | --------------------------- | ------- |
| `040`         | `A20.dat`    |                 | **???**                     | **⚠️**   |
| `050`         | `6F20.dat`   |                 | **???**                     | **⚠️**   |
| `060`         | `EF20.dat`   |                 | **???**                     | **⚠️**   |
| `070`         | `16F20.dat`  |                 | **???**                     | **⚠️**   |
| `080`         | `1EF20.dat`  |                 | **???**                     | **⚠️**   |
| `090`         | `26F20.dat`  |                 | **???**                     | **⚠️**   |
| `0A0`         | `2EF20.dat`  |                 | **???**                     | **⚠️**   |
| `0B0`         | `36F20.dat`  |                 | **???**                     | **⚠️**   |
| `0C0`         | `3EF20.dat`  |                 | **???**                     | **⚠️**   |
| `0D0`         | `46F20.dat`  |                 | **???**                     | **⚠️**   |
| `0E0`         | `4EF20.dat`  |                 | **???**                     | **⚠️**   |
| `0F0`         | `7DE80.dat`  |                 | **???**                     | **⚠️**   |
| `100`         | `93CF4.dat`  |                 | **???**                     | **⚠️**   |
| `110`         | `AE3D0.dat`  |                 | **???**                     | **⚠️**   |
| `120`         | `D0A90.dat`  |                 | **???**                     | **⚠️**   |
| `130`         | `EC6BC.dat`  |                 | **???**                     | **⚠️**   |
| `140`         | `10E58C.dat` |                 | **???**                     | **⚠️**   |
| `150`         | `127C58.dat` |                 | **???**                     | **⚠️**   |
| `160`         | `12B2F8.dat` |                 | **???**                     | **⚠️**   |
| `170`         | `142920.dat` |                 | **???**                     | **⚠️**   |
| `180`         | `1574A4.dat` |                 | **???**                     | **⚠️**   |
| `190`         | `166DDC.dat` |                 | **???**                     | **⚠️**   |
| `1A0`         | `17A464.dat` |                 | **???**                     | **⚠️**   |
| `1B0`         | `17A688.dat` |                 | **???**                     | **⚠️**   |
| `1C0`         | `17A7AC.dat` |                 | **???**                     | **⚠️**   |
| `1D0`         | `17A948.dat` |                 | **???**                     | **⚠️**   |
| `1E0`         | `17AB00.dat` |                 | **???**                     | **⚠️**   |
| `1F0`         | `17ACD4.dat` |                 | **???**                     | **⚠️**   |
| `200`         | `17ADBC.dat` |                 | **???**                     | **⚠️**   |
| `210`         | `17AEFC.dat` |                 | **???**                     | **⚠️**   |
| `220`         | `17BA3C.dat` |                 | **???**                     | **⚠️**   |
| `230`         | `17C574.dat` |                 | **???**                     | **⚠️**   |
| `240`         | `17D05C.dat` |                 | **???**                     | **⚠️**   |
| `250`         | `17D388.dat` |                 | **???**                     | **⚠️**   |
| `260`         | `17FF8C.dat` |                 | **???**                     | **⚠️**   |
| `270`         | `180664.dat` |                 | **???**                     | **⚠️**   |
| `280`         | `180D38.dat` |                 | **???**                     | **⚠️**   |
| `290`         | `1813C4.dat` |                 | **???**                     | **⚠️**   |
| `2A0`         | `181A24.dat` |                 | **???**                     | **⚠️**   |
| `2B0`         | `1820B4.dat` |                 | **???**                     | **⚠️**   |
| `2C0`         | `1827C4.dat` |                 | **???**                     | **⚠️**   |
| `2D0`         | `182E18.dat` |                 | **???**                     | **⚠️**   |
| `2E0`         | `1832DC.dat` |                 | **???**                     | **⚠️**   |
| `2F0`         | `1839F0.dat` |                 | **???**                     | **⚠️**   |
| `300`         | `184020.dat` |                 | **???**                     | **⚠️**   |
| `310`         | `18475C.dat` |                 | **???**                     | **⚠️**   |
| `320`         | `184F4C.dat` |                 | **???**                     | **⚠️**   |
| `330`         | `1858F0.dat` |                 | **???**                     | **⚠️**   |
| `340`         | `18602C.dat` |                 | **???**                     | **⚠️**   |
| `350`         | `1868D4.dat` |                 | **???**                     | **⚠️**   |
| `360`         | `1871B4.dat` |                 | **???**                     | **⚠️**   |
| `370`         | `187A4C.dat` |                 | **???**                     | **⚠️**   |
| `380`         | `1882D0.dat` |                 | **???**                     | **⚠️**   |
| `390`         | `188B7C.dat` |                 | **???**                     | **⚠️**   |
| `3A0`         | `189574.dat` |                 | **???**                     | **⚠️**   |
| `3B0`         | `189AF0.dat` |                 | **???**                     | **⚠️**   |
| `3C0`         | `18A08C.dat` |                 | **???**                     | **⚠️**   |
| `3D0`         | `18AB20.dat` |                 | **???**                     | **⚠️**   |
| `3E0`         | `18B2BC.dat` |                 | **???**                     | **⚠️**   |
| `3F0`         | `18BB20.dat` |                 | **???**                     | **⚠️**   |
| `400`         | `18C374.dat` |                 | **???**                     | **⚠️**   |
| `410`         | `19C57C.dat` |                 | **???**                     | **⚠️**   |
| `420`         | `1AC784.dat` |                 | **???**                     | **⚠️**   |
| `430`         | `1BC98C.dat` |                 | **???**                     | **⚠️**   |
| `440`         | `1CCB94.dat` |                 | **???**                     | **⚠️**   |
| `450`         | `1DCD9C.dat` |                 | **???**                     | **⚠️**   |
| `460`         | `1ECFA4.dat` |                 | **???**                     | **⚠️**   |
| `470`         | `1ED1A4.dat` |                 | **???**                     | **⚠️**   |
| `480`         | `1ED3A4.dat` |                 | **???**                     | **⚠️**   |
| `490`         | `1ED5A4.dat` |                 | **???**                     | **⚠️**   |
| `4A0`         | `1ED7A4.dat` |                 | **???**                     | **⚠️**   |
| `4B0`         | `1ED9A4.dat` |                 | **???**                     | **⚠️**   |
| `4C0`         | `1EDBA4.dat` |                 | **???**                     | **⚠️**   |
| `4D0`         | `1FDBA4.dat` |                 | **???**                     | **⚠️**   |
| `4E0`         | `20DBA4.dat` |                 | **???**                     | **⚠️**   |
| `4F0`         | `21DBA4.dat` |                 | **???**                     | **⚠️**   |
| `500`         | `22DBA4.dat` | Level geometry  | Chapter 1 Part 1            |         |
| `510`         | `294D84.dat` | Level collision | Chapter 1 Part 1            |         |
| `520`         | `2A710C.dat` | Level waypoint  | Chapter 1 Part 1            |         |
| `530`         | `2A89A4.dat` | Color palette   | Chapter 1 Part 1            |         |
| `540`         | `2B09A4.dat` | Level geometry  | Chapter 1 Part 2            |         |
| `550`         | `2E9D88.dat` | Level collision | Chapter 1 Part 2            |         |
| `560`         | `2F56B0.dat` | Level waypoint  | Chapter 1 Part 2            |         |
| `570`         | `2F5E40.dat` | Color palette   | Chapter 1 Part 2            |         |
| `580`         | `2FDE40.dat` | Level geometry  | Chapter 2 Part 1            |         |
| `590`         | `357648.dat` | Level collision | Chapter 2 Part 1            |         |
| `5A0`         | `36892C.dat` | Level waypoint  | Chapter 2 Part 1            |         |
| `5B0`         | `368F28.dat` | Color palette   | Chapter 2 Part 1            |         |
| `5C0`         | `370F28.dat` | Level geometry  | Chapter 2 Part 2            |         |
| `5D0`         | `3DF9DC.dat` | Level collision | Chapter 2 Part 2            |         |
| `5E0`         | `3F5974.dat` | Level waypoint  | Chapter 2 Part 2            |         |
| `5F0`         | `3F5AFC.dat` | Color palette   | Chapter 2 Part 2            |         |
| `600`         | `3FDAFC.dat` | Level geometry  | Chapter 3 Part 1            |         |
| `610`         | `458220.dat` | Level collision | Chapter 3 Part 1            |         |
| `620`         | `469B04.dat` | Level waypoint  | Chapter 3 Part 1            |         |
| `630`         | `46A300.dat` | Color palette   | Chapter 3 Part 1            |         |
| `640`         | `472300.dat` | Level geometry  | Chapter 3 Part 2            |         |
| `650`         | `4BFCAC.dat` | Level collision | Chapter 3 Part 2            |         |
| `660`         | `4CD11C.dat` | Level waypoint  | Chapter 3 Part 2            |         |
| `670`         | `4CD998.dat` | Color palette   | Chapter 3 Part 2            |         |
| `680`         | `4D5998.dat` | Level geometry  | Chapter 4 Part 1            |         |
| `690`         | `52EAB4.dat` | Level collision | Chapter 4 Part 1            |         |
| `6A0`         | `53DFAC.dat` | Level waypoint  | Chapter 4 Part 1            |         |
| `6B0`         | `53EBA8.dat` | Color palette   | Chapter 4 Part 1            |         |
| `6C0`         | `546BA8.dat` | Level geometry  | Chapter 4 Part 2            |         |
| `6D0`         | `58EFC8.dat` | Level collision | Chapter 4 Part 2            |         |
| `6E0`         | `59A818.dat` | Level waypoint  | Chapter 4 Part 2            |         |
| `6F0`         | `59B7B8.dat` | Color palette   | Chapter 4 Part 2            |         |
| `700`         | `5A37B8.dat` | Level geometry  | Chapter 5 Part 1            |         |
| `710`         | `5F60B8.dat` | Level collision | Chapter 5 Part 1            |         |
| `720`         | `605B3C.dat` | Level waypoint  | Chapter 5 Part 1            |         |
| `730`         | `6069B4.dat` | Color palette   | Chapter 5 Part 1            |         |
| `740`         | `60E9B4.dat` | Level geometry  | Chapter 5 Part 2            |         |
| `750`         | `683FD4.dat` | Level collision | Chapter 5 Part 2            |         |
| `760`         | `698898.dat` | Level waypoint  | Chapter 5 Part 2            |         |
| `770`         | `69A6A0.dat` | Color palette   | Chapter 5 Part 2            |         |
| `780`         | `6A26A0.dat` | Level geometry  | Chapter 6                   |         |
| `790`         | `6F23F4.dat` | Level collision | Chapter 6                   |         |
| `7A0`         | `6FF774.dat` | Level waypoint  | Chapter 6                   |         |
| `7B0`         | `6FFAA0.dat` | Color palette   | Chapter 6                   |         |
| `7C0`         | `707AA0.dat` | Level geometry  | Chapter 7                   |         |
| `7D0`         | `775A2C.dat` | Level collision | Chapter 7                   |         |
| `7E0`         | `7882C0.dat` | Level waypoint  | Chapter 7                   |         |
| `7F0`         | `788D48.dat` | Color palette   | Chapter 7                   |         |
| `800`         | `790D48.dat` | Level geometry  | Chapter 8                   |         |
| `810`         | `7E3990.dat` | Level collision | Chapter 8                   |         |
| `820`         | `7F5028.dat` | Level waypoint  | Chapter 8                   |         |
| `830`         | `7F5464.dat` | Color palette   | Chapter 8                   |         |
| `840`         | `7FD464.dat` | Level geometry  | Deathmatch 1                |         |
| `850`         | `823F54.dat` | Level collision | Deathmatch 1                |         |
| `860`         | `82A85C.dat` | Color palette   | Deathmatch 1                |         |
| `870`         | `83285C.dat` | Level geometry  | Deathmatch 2                |         |
| `880`         | `84F7D0.dat` | Level collision | Deathmatch 2                |         |
| `890`         | `856108.dat` | Color palette   | Deathmatch 2                |         |
| `8A0`         | `85E108.dat` | Level geometry  | Deathmatch 3                |         |
| `8B0`         | `870414.dat` | Level collision | Deathmatch 3                |         |
| `8C0`         | `87352C.dat` | Color palette   | Deathmatch 3                |         |
| `8D0`         | `87B52C.dat` | Level geometry  | Deathmatch 4                |         |
| `8E0`         | `8A8F2C.dat` | Level collision | Deathmatch 4                |         |
| `8F0`         | `8B2508.dat` | Color palette   | Deathmatch 4                |         |
| `900`         | `8BA508.dat` | Level geometry  | Test level for enemies      | 🔎      |
| `910`         | `8C4188.dat` | Level collision | Test level for enemies      | 🔎      |
| `920`         | `8C4F64.dat` | Level waypoint  | Test level for enemies      | **⚠️🔎** |
| `930`         | `8C4F84.dat` | Level geometry  | Test level **???** geometry | 🔎      |
| `940`         | `8CFF88.dat` | Level collision | Test level **???**          | **⚠️🔎** |
| `950`         | `8D1000.dat` | Level waypoint  | Test level **???**          | **⚠️🔎** |
| `960`         | `8D102C.dat` |                 | **???**                     | **⚠️**   |
| `970`         | `8DB36C.dat` |                 | **???**                     | **⚠️**   |
| `980`         | `8DCC74.dat` |                 | **???**                     | **⚠️**   |
| `990`         | `8E160C.dat` |                 | **???**                     | **⚠️**   |
| `9A0`         | `A25878.dat` |                 | **???**                     | **⚠️**   |
| `9B0`         | `BBC974.dat` |                 | **???**                     | **⚠️**   |
| `9C0`         | `D9924C.dat` | Text bank       | English UK                  |         |
| `9D0`         | `D9B808.dat` | Text bank       | English US                  |         |
| `9E0`         | `D9DDC4.dat` | Text bank       | French                      |         |
| `9F0`         | `DA0904.dat` | Text bank       | Italian                     |         |
| `A00`         | `DA34B8.dat` | Text bank       | German                      |         |
| `A10`         | `DA5B9C.dat` | Text bank       | Spanish                     |         |

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

## Discoveries

- Test level for enemies
    - There is a pool with fish enemies, which I don't remember seeing in the game
- Test level ???
    - Collision by itself works with other level geometry, geometry by itself also works, but not together

