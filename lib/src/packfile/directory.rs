use super::PackFile;
use engine::{asset::Asset, directory::Directory};
use std::collections::HashMap;

macro_rules! insert_asset {
    ($hashmap:ident[$key:literal] = $entry:expr) => {
        let key = ::std::string::String::from($key);
        let entry = ::std::mem::take(&mut $entry);
        $hashmap.insert(key, entry.bytes);
    };
}

pub struct PackFileDirectory {
    assets: HashMap<String, Vec<u8>>,
}

impl PackFileDirectory {
    pub fn new(packfile: PackFile) -> Self {
        // TODO(Unavailable): with_capacity();
        let mut assets = HashMap::new();
        let mut entries = packfile.entries;

        insert_asset!(assets["gamma_table"] = entries[0x00]);

        Self { assets }
    }
}

impl Directory for PackFileDirectory {
    fn get<A: Asset>(&self, id: &str) -> A {
        let bytes = self.assets.get(id).expect("id was found");
        A::parse(bytes, Default::default())
    }

    fn get_all<A: Asset>(&self) -> Vec<A> {
        todo!()
    }
}

const ASSETS: [&'static str; 158] = [
    "gamma_table",
    "CREATURECOLORMAP",
    "G_CREATURECOLORMAP",
    "GHOSTCREATURECOLORMAP",
    "PICKUPCOLORMAP",
    "G_PICKUPCOLORMAP",
    "JACOBCOLORMAP",
    "G_LEVEL_COLORMAP",
    "PLAYERHANDSCOLORMAP",
    "G_PLAYERHANDSCOLORMAP",
    "AQUAGORE",
    "BROODMAW",
    "CRYPTCRAWLER",
    "FIREDEACON",
    "HUNTER",
    "PSISTALKER",
    "STORMFLUKE",
    "TENTACLE",
    "WRAITH",
    "PRIMEENTITY",
    "PLAYERMODEL",
    "VANESSA",
    "ROCKET",
    "GRENADE",
    "FXBLAST",
    "AQUAGORE_SHOT",
    "BROODMAW_SHOT",
    "CRYPTCRAWLER_SHOT",
    "FIREDEACON_SHOT",
    "GIB_GENERIC_1",
    "GIB_GENERIC_2",
    "GIB_GENERIC_3",
    "BLOOD_GENERIC_1",
    "CHARLES",
    "HUMAN_GIB_GENERIC_1",
    "HUMAN_GIB_GENERIC_2",
    "HUMAN_GIB_GENERIC_3",
    "PICKUP_AMMO_PISTOL",
    "PICKUP_AMMO_DOUBLE_PISTOL",
    "PICKUP_AMMO_SHOTGUN",
    "PICKUP_AMMO_MACHINEGUN",
    "PICKUP_AMMO_SNIPER",
    "PICKUP_AMMO_GRENADE",
    "PICKUP_AMMO_ROCKET",
    "PICKUP_AMMO_GATLINGGUN",
    "PICKUP_WEAPON_PISTOL",
    "PICKUP_WEAPON_DOUBLE_PISTOL",
    "PICKUP_WEAPON_SHOTGUN",
    "PICKUP_WEAPON_MACHINEGUN",
    "PICKUP_WEAPON_SNIPER",
    "PICKUP_WEAPON_GRENADE",
    "PICKUP_WEAPON_GATLINGGUN",
    "PICKUP_WEAPON_SHOCKWAVE",
    "PICKUP_GHOSTVISION",
    "PICKUP_FOCITALISMAN",
    "PICKUP_LETTER",
    "PICKUP_KEY1",
    "PICKUP_FLAKJACKET_25",
    "PICKUP_FLAKJACKET_50",
    "PICKUP_FLAKJACKET_100",
    "LEVEL1_SKY",
    "LEVEL2_SKY",
    "LEVEL3_SKY",
    "LEVEL4_SKY",
    "LEVEL5_SKY",
    "LEVEL6_SKY",
    "LEVEL1_SKY_GHOSTPALETTE",
    "LEVEL2_SKY_GHOSTPALETTE",
    "LEVEL3_SKY_GHOSTPALETTE",
    "LEVEL4_SKY_GHOSTPALETTE",
    "LEVEL5_SKY_GHOSTPALETTE",
    "LEVEL6_SKY_GHOSTPALETTE",
    "JACOB_SKIN_RED",
    "JACOB_SKIN_GREEN",
    "JACOB_SKIN_BLUE",
    "JACOB_SKIN_YELLOW",
    "LEVEL1A",
    "LEVEL1A_COLLISION",
    "LEVEL1A_WAYPOINTNAV",
    "LEVEL1A_COLORMAP",
    "LEVEL1B",
    "LEVEL1B_COLLISION",
    "LEVEL1B_WAYPOINTNAV",
    "LEVEL1B_COLORMAP",
    "LEVEL2A",
    "LEVEL2A_COLLISION",
    "LEVEL2A_WAYPOINTNAV",
    "LEVEL2A_COLORMAP",
    "Level2B",
    "LEVEL2B_COLLISION",
    "LEVEL2B_WAYPOINTNAV",
    "LEVEL2B_COLORMAP",
    "LEVEL3A",
    "LEVEL3A_COLLISION",
    "LEVEL3A_WAYPOINTNAV",
    "LEVEL3A_COLORMAP",
    "LEVEL3B",
    "LEVEL3B_COLLISION",
    "LEVEL3B_WAYPOINTNAV",
    "LEVEL3B_COLORMAP",
    "LEVEL4A",
    "LEVEL4A_COLLISION",
    "LEVEL4A_WAYPOINTNAV",
    "LEVEL4A_COLORMAP",
    "LEVEL4B",
    "LEVEL4B_COLLISION",
    "LEVEL4B_WAYPOINTNAV",
    "LEVEL4B_COLORMAP",
    "LEVEL5A",
    "LEVEL5A_COLLISION",
    "LEVEL5A_WAYPOINTNAV",
    "LEVEL5A_COLORMAP",
    "LEVEL5B",
    "LEVEL5B_COLLISION",
    "LEVEL5B_WAYPOINTNAV",
    "LEVEL5B_COLORMAP",
    "LEVEL6",
    "LEVEL6_COLLISION",
    "LEVEL6_WAYPOINTNAV",
    "LEVEL6_COLORMAP",
    "LEVEL7",
    "LEVEL7_COLLISION",
    "LEVEL7_WAYPOINTNAV",
    "LEVEL7_COLORMAP",
    "LEVEL8",
    "LEVEL8_COLLISION",
    "LEVEL8_WAYPOINTNAV",
    "LEVEL8_COLORMAP",
    "LEVELDM1",
    "LEVELDM1_COLLISION",
    "LEVELDM1_COLORMAP",
    "LEVELDM2",
    "LEVELDM2_COLLISION",
    "LEVELDM2_COLORMAP",
    "LEVELDM3",
    "LEVELDM3_COLLISION",
    "LEVELDM3_COLORMAP",
    "LEVELDM4",
    "LEVELDM4_COLLISION",
    "LEVELDM4_COLORMAP",
    "LEVELMONSTERS",
    "LEVELMONSTERS_COLLISION",
    "LEVELMONSTERS_WAYPOINTNAV",
    "LEVELDOORS",
    "LEVELDOORS_COLLISION",
    "LEVELDOORS_WAYPOINTNAV",
    "SPRITES",
    "TEXTURE_INFO",
    "SPRITE_TEXTURE_INFO",
    "TEXTURES",
    "SPRITE_TEXTURES",
    "SOUND_DATA",
    "STRINGTABLE_ENGLISH_UK",
    "STRINGTABLE_ENGLISH_US",
    "STRINGTABLE_FRENCH",
    "STRINGTABLE_ITALIAN",
    "STRINGTABLE_GERMAN",
    "STRINGTABLE_SPANISH",
];
