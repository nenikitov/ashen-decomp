use std::{
    borrow::Cow,
    io,
    path::{Component, Path},
};

use either::{Either, Left};

use super::PackFile;
use crate::utils::iterator::IterMore;

type Handle = Either<FileHandle, DirHandle>;

#[derive(Clone, Debug)]
pub struct FileHandle {
    // PERF(Unavailable): Either<&'static str, Box<str>>
    name: Box<str>,
    bytes: Box<[u8]>,
}

impl FileHandle {
    fn new<N, B>(name: N, bytes: B) -> Self
    where
        N: Into<Box<str>>,
        B: Into<Box<[u8]>>,
    {
        Self {
            name: name.into(),
            bytes: bytes.into(),
        }
    }

    /// The file's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The file's bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Clone, Debug)]
pub struct DirHandle {
    // Either keep `&'static Path` or `PathBuf`.
    path: Cow<'static, Path>,
    children: Vec<Handle>,
}

impl DirHandle {
    fn traverse_root_from_path(root: &DirHandle, path: &Path) -> io::Result<Self> {
        if Path::new("/") == path || Path::new("./") == path {
            return Ok(root.clone());
        };

        let mut head = root;
        for component in path.components() {
            match component {
                Component::RootDir | Component::CurDir => {
                    // These components can only appear once (at the start of
                    // the path [a/./b gets normalized]), so they can be ignored,
                    // because `head` is already pointing at `root`.
                }
                Component::Normal(component) => {
                    match head.children.iter().find_map(|handle| {
                        handle.as_ref().right().and_then(|handle| {
                            handle
                                .path
                                .file_name()
                                .is_some_and(|n| n == component)
                                .then_some(handle)
                        })
                    }) {
                        Some(handle) => head = handle,
                        None => {
                            return Err(io::Error::new(
                                io::ErrorKind::NotFound,
                                format!("{:?} was not found", component),
                            ));
                        }
                    }
                }
                Component::Prefix(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "the use of path prefixes (i.e 'C:' on windows) is unsupported",
                    ))
                }
                Component::ParentDir => {
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "the use of '..' to refer to the parent directory is unsupported",
                    ))
                }
            }
        }

        Ok(head.clone())
    }

    /// The directory's name.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl IntoIterator for DirHandle {
    type Item = FileHandle;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter().flat_map(|handle| {
            handle.map_either(std::iter::once, |dir| dir.into_iter().into_box_dyn())
        })
    }
}

pub struct PackFileDirectory {
    root: DirHandle,
}

impl PackFileDirectory {
    pub fn from_106_packfile(pack: PackFile) -> Option<Self> {
        let copyright = pack.copyright.into_bytes();
        let mut pack = pack.entries.into_iter();

        let mut touch = |str: &'static str| -> Option<Handle> {
            Some(Left(FileHandle::new(str.to_string(), pack.next()?.bytes)))
        };

        let children = vec![
            Left(FileHandle::new("copyright".to_string(), copyright)),
            touch("color_map")?,
        ];

        Some(Self {
            root: DirHandle {
                path: Cow::Borrowed(Path::new("/")),
                children,
            },
        })
    }
}

impl PackFileDirectory {
    pub fn walk<P>(&self, path: P) -> io::Result<DirHandle>
    where
        P: AsRef<Path>,
    {
        // PERF(Unavailable): inner fn
        let path = path.as_ref();
        DirHandle::traverse_root_from_path(&self.root, path)
    }

    pub fn get<P>(&self, path: P) -> io::Result<FileHandle>
    where
        P: AsRef<Path>,
    {
        fn is_relative_to_root(path: &Path) -> Option<&Path> {
            // PERF(Unavailable): Not that it matters just curious :)
            //
            // [Path::new("."), Path::new("")]
            //     .contains(&path)
            //     .not()
            //     .then_some(path)
            (Path::new(".") != path && Path::new("") != path).then_some(path)
        }

        // PERF(Unavailable): inner fn
        let path = path.as_ref();
        // NOTE: This needs to go before getting the `parent`, because this will
        // error out if the file path terminates in `root`, a `prefix`, or if
        // it's the empty string.
        let name = path.file_name().ok_or(io::ErrorKind::InvalidFilename)?;

        let parent = path
            .parent()
            .and_then(is_relative_to_root)
            .unwrap_or(Path::new("/"));

        Ok(self
            .walk(parent)?
            .into_iter()
            .find(|file| file.name() == name)
            .ok_or(io::ErrorKind::NotFound)?)
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

        let dir = PackFileDirectory::from_106_packfile(pack_file).unwrap();
        let dir = dir.walk("/")?;

        for file in dir {
            println!("{file:?}");
        }

        Ok(())
    }
}

// const ASSETS: [&'static str; 158] = [
//     "misc/gamma_table",
//     "color_map/creature",
//     "color_map/creature.ghost",
//     "color_map/wraith",
//     "color_map/pick_up",
//     "color_map/pick_up.ghost",
//     "color_map/jacob",
//     "color_map/level.ghost",
//     "color_map/player_hands",
//     "color_map/player_hands.ghost",
//     "model/enemies/aquagore",
//     "model/enemies/brood_maw",
//     "model/enemies/crypt_crawler",
//     "model/enemies/fire_deacon",
//     "model/enemies/hunter",
//     "model/enemies/psi_stalker",
//     "model/enemies/storm_fluke",
//     "model/enemies/tentacle",
//     "model/enemies/wraith",
//     "model/enemies/prime_entity",
//     "model/characters/jacob",
//     "model/characters/vanessa",
//     "model/projectiles/rocket",
//     "model/projectiles/grenade",
//     "model/projectiles/fx_blast", // TODO(nenikitov): use a better name. Is this a puse gun projectile?
//     "model/projectiles/aquagore_shot",
//     "model/projectiles/brood_maw_shot",
//     "model/projectiles/crypt_crawler_shot",
//     "model/projectiles/fire_deacon_shot",
//     "model/gibs/gib_1",
//     "model/gibs/gib_2",
//     "model/gibs/gib_3",
//     "model/gibs/blood_1",
//     "model/gibs/charles",
//     "model/gibs/human_gib_1",
//     "model/gibs/human_gib_2",
//     "model/gibs/human_gib_3",
//     "model/pickup/ammo_pistol",
//     "model/pickup/ammo_double_pistol",
//     "model/pickup/ammo_shotgun",
//     "model/pickup/ammo_machinegun",
//     "model/pickup/ammo_sniper",
//     "model/pickup/ammo_grenade",
//     "model/pickup/ammo_rocket",
//     "model/pickup/ammo_gatling_gun",
//     "model/pickup/weapon_pistol",
//     "model/pickup/weapon_double_pistol",
//     "model/pickup/weapon_shotgun",
//     "model/pickup/weapon_machinegun",
//     "model/pickup/weapon_sniper",
//     "model/pickup/weapon_grenade",
//     "model/pickup/weapon_gatling_gun",
//     "model/pickup/weapon_alien_pulse_gun",
//     "model/pickup/ghost_vision_goggles",
//     "model/pickup/talisman",
//     "model/pickup/letter",
//     "model/pickup/alien_key",
//     "model/pickup/flak_jacket_25",
//     "model/pickup/flak_jacket_50",
//     "model/pickup/flak_jacket_100",
//     "level/1/sky",
//     "level/2/sky",
//     "level/3/sky",
//     "level/4/sky",
//     "level/5/sky",
//     "level/6/sky",
//     "level/1/sky.color_map.ghost",
//     "level/2/sky.color_map.ghost",
//     "level/3/sky.color_map.ghost",
//     "level/4/sky.color_map.ghost",
//     "level/5/sky.color_map.ghost",
//     "level/6/sky.color_map.ghost",
//     "model/characters/jacob.color_map.red",
//     "model/characters/jacob.color_map.green",
//     "model/characters/jacob.color_map.blue",
//     "model/characters/jacob.color_map.yellow",
//     "level/1a/geometry",
//     "level/1a/collision",
//     "level/1a/waypoint",
//     "level/1a/color_map",
//     "level/1b/geometry",
//     "level/1b/collision",
//     "level/1b/waypoint",
//     "level/1b/color_map",
//     "level/2a/geometry",
//     "level/2a/collision",
//     "level/2a/waypoint",
//     "level/2a/color_map",
//     "level/2b/geometry",
//     "level/2b/collision",
//     "level/2b/waypoint",
//     "level/2b/color_map",
//     "level/3a/geometry",
//     "level/3a/collision",
//     "level/3a/waypoint",
//     "level/3a/color_map",
//     "level/3b/geometry",
//     "level/3b/collision",
//     "level/3b/waypoint",
//     "level/3b/color_map",
//     "level/4a/geometry",
//     "level/4a/collision",
//     "level/4a/waypoint",
//     "level/4a/color_map",
//     "level/4b/geometry",
//     "level/4b/collision",
//     "level/4b/waypoint",
//     "level/4b/color_map",
//     "level/5a/geometry",
//     "level/5a/collision",
//     "level/5a/waypoint",
//     "level/5a/color_map",
//     "level/5b/geometry",
//     "level/5b/collision",
//     "level/5b/waypoint",
//     "level/5b/color_map",
//     "level/6/geometry",
//     "level/6/collision",
//     "level/6/waypoint",
//     "level/6/color_map",
//     "level/7/geometry",
//     "level/7/collision",
//     "level/7/waypoint",
//     "level/7/color_map",
//     "level/8/geometry",
//     "level/8/collision",
//     "level/8/waypoint",
//     "level/8/color_map",
//     "level/dm_1/geometry",
//     "level/dm_1/collison",
//     "level/dm_1/color_map",
//     "level/dm_2/geometry",
//     "level/dm_2/collison",
//     "level/dm_2/color_map",
//     "level/dm_3/geometry",
//     "level/dm_3/collison",
//     "level/dm_3/color_map",
//     "level/dm_4/geometry",
//     "level/dm_4/collison",
//     "level/dm_4/color_map",
//     "level/test_monsters/geometry",
//     "level/test_monsters/collison",
//     "level/test_monsters/color_map",
//     "level/test_doors/geometry",
//     "level/test_doors/collison",
//     "level/test_doors/color_map",
//     "SPRITES", // TODO(nenikitov): Figure out what exactly those files are and how to name them
//     "TEXTURE_INFO",
//     "SPRITE_TEXTURE_INFO",
//     "TEXTURES",
//     "SPRITE_TEXTURES",
//     "sound/sound.collection", // Should be separated into multiple files
//     "string_table/english_uk",
//     "string_table/english_us",
//     "string_table/french",
//     "string_table/italian",
//     "string_table/german",
//     "string_table/spanish",
// ]
