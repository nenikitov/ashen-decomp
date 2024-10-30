use std::{
    collections::BTreeMap,
    io,
    path::{Path, PathBuf},
};

use super::PackFile;

#[derive(Clone, Debug)]
pub struct FileHandle {
    bytes: Box<[u8]>,
}

impl FileHandle {
    /// The file's bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Default, Debug)]
pub struct VirtualFileSystem {
    files: BTreeMap<PathBuf, FileHandle>,
}

impl VirtualFileSystem {
    pub fn from_106_packfile(pack: PackFile) -> Option<Self> {
        let mut files = BTreeMap::new();

        let copyright = pack.copyright.into_bytes().into_boxed_slice();
        let mut pack = pack.entries.into_iter();

        macro_rules! touch {
            ($path:literal) => {
                touch!($path, pack.next()?.bytes.into())
            };
            ($path:literal, $bytes:expr) => {
                files.insert(PathBuf::from($path), FileHandle { bytes: $bytes });
            };
        }

        touch!("/copyright", copyright);
        touch!("/color_map/creature");
        touch!("/color_map/creature.ghost");
        touch!("/color_map/wraith");
        touch!("/color_map/pick_up");
        touch!("/color_map/pick_up.ghost");
        touch!("/color_map/jacob");
        touch!("/color_map/level.ghost");
        touch!("/color_map/player_hands");
        touch!("/color_map/player_hands.ghost");
        touch!("/model/enemies/aquagore");
        touch!("/model/enemies/brood_maw");
        touch!("/model/enemies/crypt_crawler");
        touch!("/model/enemies/fire_deacon");
        touch!("/model/enemies/hunter");
        touch!("/model/enemies/psi_stalker");
        touch!("/model/enemies/storm_fluke");
        touch!("/model/enemies/tentacle");
        touch!("/model/enemies/wraith");
        touch!("/model/enemies/prime_entity");
        touch!("/model/characters/jacob");
        touch!("/model/characters/vanessa");
        touch!("/model/projectiles/rocket");
        touch!("/model/projectiles/grenade");
        touch!("/model/projectiles/fx_blast");
        touch!("/model/projectiles/aquagore_shot");
        touch!("/model/projectiles/brood_maw_shot");
        touch!("/model/projectiles/crypt_crawler_shot");
        touch!("/model/projectiles/fire_deacon_shot");
        touch!("/model/gibs/gib_1");
        touch!("/model/gibs/gib_2");
        touch!("/model/gibs/gib_3");
        touch!("/model/gibs/blood_1");
        touch!("/model/gibs/charles");
        touch!("/model/gibs/human_gib_1");
        touch!("/model/gibs/human_gib_2");
        touch!("/model/gibs/human_gib_3");
        touch!("/model/pickup/ammo_pistol");
        touch!("/model/pickup/ammo_double_pistol");
        touch!("/model/pickup/ammo_shotgun");
        touch!("/model/pickup/ammo_machinegun");
        touch!("/model/pickup/ammo_sniper");
        touch!("/model/pickup/ammo_grenade");
        touch!("/model/pickup/ammo_rocket");
        touch!("/model/pickup/ammo_gatling_gun");
        touch!("/model/pickup/weapon_pistol");
        touch!("/model/pickup/weapon_double_pistol");
        touch!("/model/pickup/weapon_shotgun");
        touch!("/model/pickup/weapon_machinegun");
        touch!("/model/pickup/weapon_sniper");
        touch!("/model/pickup/weapon_grenade");
        touch!("/model/pickup/weapon_gatling_gun");
        touch!("/model/pickup/weapon_alien_pulse_gun");
        touch!("/model/pickup/ghost_vision_goggles");
        touch!("/model/pickup/talisman");
        touch!("/model/pickup/letter");
        touch!("/model/pickup/alien_key");
        touch!("/model/pickup/flak_jacket_25");
        touch!("/model/pickup/flak_jacket_50");
        touch!("/model/pickup/flak_jacket_100");
        touch!("/level/1/sky");
        touch!("/level/2/sky");
        touch!("/level/3/sky");
        touch!("/level/4/sky");
        touch!("/level/5/sky");
        touch!("/level/6/sky");
        touch!("/level/1/sky.color_map.ghost");
        touch!("/level/2/sky.color_map.ghost");
        touch!("/level/3/sky.color_map.ghost");
        touch!("/level/4/sky.color_map.ghost");
        touch!("/level/5/sky.color_map.ghost");
        touch!("/level/6/sky.color_map.ghost");
        touch!("/model/characters/jacob.color_map.red");
        touch!("/model/characters/jacob.color_map.green");
        touch!("/model/characters/jacob.color_map.blue");
        touch!("/model/characters/jacob.color_map.yellow");
        touch!("/level/1a/geometry");
        touch!("/level/1a/collision");
        touch!("/level/1a/waypoint");
        touch!("/level/1a/color_map");
        touch!("/level/1b/geometry");
        touch!("/level/1b/collision");
        touch!("/level/1b/waypoint");
        touch!("/level/1b/color_map");
        touch!("/level/2a/geometry");
        touch!("/level/2a/collision");
        touch!("/level/2a/waypoint");
        touch!("/level/2a/color_map");
        touch!("/level/2b/geometry");
        touch!("/level/2b/collision");
        touch!("/level/2b/waypoint");
        touch!("/level/2b/color_map");
        touch!("/level/3a/geometry");
        touch!("/level/3a/collision");
        touch!("/level/3a/waypoint");
        touch!("/level/3a/color_map");
        touch!("/level/3b/geometry");
        touch!("/level/3b/collision");
        touch!("/level/3b/waypoint");
        touch!("/level/3b/color_map");
        touch!("/level/4a/geometry");
        touch!("/level/4a/collision");
        touch!("/level/4a/waypoint");
        touch!("/level/4a/color_map");
        touch!("/level/4b/geometry");
        touch!("/level/4b/collision");
        touch!("/level/4b/waypoint");
        touch!("/level/4b/color_map");
        touch!("/level/5a/geometry");
        touch!("/level/5a/collision");
        touch!("/level/5a/waypoint");
        touch!("/level/5a/color_map");
        touch!("/level/5b/geometry");
        touch!("/level/5b/collision");
        touch!("/level/5b/waypoint");
        touch!("/level/5b/color_map");
        touch!("/level/6/geometry");
        touch!("/level/6/collision");
        touch!("/level/6/waypoint");
        touch!("/level/6/color_map");
        touch!("/level/7/geometry");
        touch!("/level/7/collision");
        touch!("/level/7/waypoint");
        touch!("/level/7/color_map");
        touch!("/level/8/geometry");
        touch!("/level/8/collision");
        touch!("/level/8/waypoint");
        touch!("/level/8/color_map");
        touch!("/level/dm_1/geometry");
        touch!("/level/dm_1/collison");
        touch!("/level/dm_1/color_map");
        touch!("/level/dm_2/geometry");
        touch!("/level/dm_2/collison");
        touch!("/level/dm_2/color_map");
        touch!("/level/dm_3/geometry");
        touch!("/level/dm_3/collison");
        touch!("/level/dm_3/color_map");
        touch!("/level/dm_4/geometry");
        touch!("/level/dm_4/collison");
        touch!("/level/dm_4/color_map");
        touch!("/level/test_monsters/geometry");
        touch!("/level/test_monsters/collison");
        touch!("/level/test_monsters/color_map");
        touch!("/level/test_doors/geometry");
        touch!("/level/test_doors/collison");
        touch!("/level/test_doors/color_map");

        // TODO(Unavailable):
        // "SPRITES", // TODO(nenikitov): Figure out what exactly those files are and how to name them
        // "TEXTURE_INFO",
        // "SPRITE_TEXTURE_INFO",
        // "TEXTURES",
        // "SPRITE_TEXTURES",
        // "sound/sound.collection", // Should be separated into multiple files
        pack.by_ref().skip(6);

        touch!("/string_table/english_uk");
        touch!("/string_table/english_us");
        touch!("/string_table/french");
        touch!("/string_table/italian");
        touch!("/string_table/german");
        touch!("/string_table/spanish");

        Some(Self { files })
    }

    pub fn read<P>(&self, path: P) -> io::Result<&FileHandle>
    where
        P: AsRef<Path>,
    {
        self.files
            .get(path.as_ref())
            .ok_or(io::Error::from(io::ErrorKind::NotFound))
    }

    pub fn walk<P>(&self, path: P, recursive: bool) -> io::Result<Vec<(&Path, &FileHandle)>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        if self.files.contains_key(path) {
            return Err(io::Error::from(io::ErrorKind::NotADirectory));
        }

        Ok(self
            .files
            .iter()
            .filter(|(f, _)| f.starts_with(&path))
            .filter(|(f, _)| recursive || f.parent() == Some(&path))
            .map(|(f, h)| (f.as_path(), h))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use super::*;
    use crate::utils::test::*;

    const ROM_DATA: LazyCell<Vec<u8>> = std::cell::LazyCell::new(|| {
        std::fs::read(workspace_file_path!("rom/packfile.dat")).expect("ROM is present")
    });

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn packfile_directory_works() -> eyre::Result<()> {
        let (_, pack_file) = PackFile::new(&ROM_DATA)?;

        let dir = VirtualFileSystem::from_106_packfile(pack_file).unwrap();
        let dir = dir.walk("/", true)?;

        for file in dir {
            println!("{file:?}");
        }

        Ok(())
    }
}
