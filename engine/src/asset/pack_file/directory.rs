use super::PackFile;
use crate::{
    asset::{
        color_map::ColorMap, extension::Wildcard, gamma_table::GammaTable, model::Model,
        skybox::Skybox, string_table::StringTable, texture::dat::texture::Texture, AssetParser,
    },
    directory::Directory,
};

use std::{
    any::TypeId,
    io::{ErrorKind as IoErrorKind, Result as IoResult},
    path::Path,
};

struct Level;
struct Sound;

pub struct PackFileDirectory {
    gamma_table: GammaTable,
    color_maps: Box<[ColorMap]>,
    models: Box<[Model]>,
    levels: Box<[Level]>,
    textures: Box<[Texture]>,
    sounds: Box<[Sound]>,
    strings: Box<[StringTable]>,
}

impl PackFileDirectory {
    pub fn new(packfile: PackFile) -> Self {
        todo!()
    }
}

impl Directory for PackFileDirectory {
    fn get<A, P>(&self, path: P) -> IoResult<A>
    where
        A: AssetParser<Wildcard>,
        P: AsRef<Path>,
    {
        // let entry = path.as_ref().to_str().ok_or(IoErrorKind::InvalidFilename)?;
        // let entry = self.assets.get(entry).ok_or(IoErrorKind::NotFound)?;
        //
        // match A::parse(entry, Extension::Dat) {
        //     Ok((_input, asset)) => Ok(asset),
        //     Err(err) => Err(IoError::new(IoErrorKind::InvalidData, err)),
        // }

        todo!()
    }

    // TODO(Unavailable): Return something that implements `IntoIterator<Item = A>` so wether a user
    // passes `Sound` or `SoundCollection` it could directly return a `SoundCollection` regardless.
    fn all<A>(&self) -> IoResult<Vec<A>>
    where
        A: AssetParser<Wildcard> + 'static,
    {
        match TypeId::of::<A>() {
            id if TypeId::of::<GammaTable>() == id => todo!(),
            id if TypeId::of::<ColorMap>() == id => todo!(),
            // id if TypeId::of::<Sound>() == id => todo!(),
            id if TypeId::of::<StringTable>() == id => todo!(),
            id if TypeId::of::<Skybox>() == id => todo!(),
            id if TypeId::of::<Model>() == id => todo!(),
            // id if TypeId::of::<Texture>() == id => todo!(),
            // id if TypeId::of::<Level>() == id => todo!(),
            _ => todo!(),
        }
    }
}

const ASSETS: [&'static str; 158] = [
    "misc/gamma_table",
    "color_map/creature",
    "color_map/creature.ghost",
    "color_map/wraith",
    "color_map/pick_up",
    "color_map/pick_up.ghost",
    "color_map/jacob",
    "color_map/level.ghost",
    "color_map/player_hands",
    "color_map/player_hands.ghost",
    "model/enemies/aquagore",
    "model/enemies/brood_maw",
    "model/enemies/crypt_crawler",
    "model/enemies/fire_deacon",
    "model/enemies/hunter",
    "model/enemies/psi_stalker",
    "model/enemies/storm_fluke",
    "model/enemies/tentacle",
    "model/enemies/wraith",
    "model/enemies/prime_entity",
    "model/characters/jacob",
    "model/characters/vanessa",
    "model/projectiles/rocket",
    "model/projectiles/grenade",
    "model/projectiles/fx_blast", // TODO(nenikitov): use a better name. Is this a puse gun projectile?
    "model/projectiles/aquagore_shot",
    "model/projectiles/brood_maw_shot",
    "model/projectiles/crypt_crawler_shot",
    "model/projectiles/fire_deacon_shot",
    "model/gibs/gib_1",
    "model/gibs/gib_2",
    "model/gibs/gib_3",
    "model/gibs/blood_1",
    "model/gibs/charles",
    "model/gibs/human_gib_1",
    "model/gibs/human_gib_2",
    "model/gibs/human_gib_3",
    "model/pickup/ammo_pistol",
    "model/pickup/ammo_double_pistol",
    "model/pickup/ammo_shotgun",
    "model/pickup/ammo_machinegun",
    "model/pickup/ammo_sniper",
    "model/pickup/ammo_grenade",
    "model/pickup/ammo_rocket",
    "model/pickup/ammo_gatling_gun",
    "model/pickup/weapon_pistol",
    "model/pickup/weapon_double_pistol",
    "model/pickup/weapon_shotgun",
    "model/pickup/weapon_machinegun",
    "model/pickup/weapon_sniper",
    "model/pickup/weapon_grenade",
    "model/pickup/weapon_gatling_gun",
    "model/pickup/weapon_alien_pulse_gun",
    "model/pickup/ghost_vision_goggles",
    "model/pickup/talisman",
    "model/pickup/letter",
    "model/pickup/alien_key",
    "model/pickup/flak_jacket_25",
    "model/pickup/flak_jacket_50",
    "model/pickup/flak_jacket_100",
    "level/1/sky",
    "level/2/sky",
    "level/3/sky",
    "level/4/sky",
    "level/5/sky",
    "level/6/sky",
    "level/1/sky.color_map.ghost",
    "level/2/sky.color_map.ghost",
    "level/3/sky.color_map.ghost",
    "level/4/sky.color_map.ghost",
    "level/5/sky.color_map.ghost",
    "level/6/sky.color_map.ghost",
    "model/characters/jacob.color_map.red",
    "model/characters/jacob.color_map.green",
    "model/characters/jacob.color_map.blue",
    "model/characters/jacob.color_map.yellow",
    "level/1a/geometry",
    "level/1a/collision",
    "level/1a/waypoint",
    "level/1a/color_map",
    "level/1b/geometry",
    "level/1b/collision",
    "level/1b/waypoint",
    "level/1b/color_map",
    "level/2a/geometry",
    "level/2a/collision",
    "level/2a/waypoint",
    "level/2a/color_map",
    "level/2b/geometry",
    "level/2b/collision",
    "level/2b/waypoint",
    "level/2b/color_map",
    "level/3a/geometry",
    "level/3a/collision",
    "level/3a/waypoint",
    "level/3a/color_map",
    "level/3b/geometry",
    "level/3b/collision",
    "level/3b/waypoint",
    "level/3b/color_map",
    "level/4a/geometry",
    "level/4a/collision",
    "level/4a/waypoint",
    "level/4a/color_map",
    "level/4b/geometry",
    "level/4b/collision",
    "level/4b/waypoint",
    "level/4b/color_map",
    "level/5a/geometry",
    "level/5a/collision",
    "level/5a/waypoint",
    "level/5a/color_map",
    "level/5b/geometry",
    "level/5b/collision",
    "level/5b/waypoint",
    "level/5b/color_map",
    "level/6/geometry",
    "level/6/collision",
    "level/6/waypoint",
    "level/6/color_map",
    "level/7/geometry",
    "level/7/collision",
    "level/7/waypoint",
    "level/7/color_map",
    "level/8/geometry",
    "level/8/collision",
    "level/8/waypoint",
    "level/8/color_map",
    "level/dm_1/geometry",
    "level/dm_1/collison",
    "level/dm_1/color_map",
    "level/dm_2/geometry",
    "level/dm_2/collison",
    "level/dm_2/color_map",
    "level/dm_3/geometry",
    "level/dm_3/collison",
    "level/dm_3/color_map",
    "level/dm_4/geometry",
    "level/dm_4/collison",
    "level/dm_4/color_map",
    "level/test_monsters/geometry",
    "level/test_monsters/collison",
    "level/test_monsters/color_map",
    "level/test_doors/geometry",
    "level/test_doors/collison",
    "level/test_doors/color_map",
    "SPRITES", // TODO(nenikitov): Figure out what exactly those files are and how to name them
    "TEXTURE_INFO",
    "SPRITE_TEXTURE_INFO",
    "TEXTURES",
    "SPRITE_TEXTURES",
    "sound/sound.collection", // Should be separated into multiple files
    "string_table/english_uk",
    "string_table/english_us",
    "string_table/french",
    "string_table/italian",
    "string_table/german",
    "string_table/spanish",
];
