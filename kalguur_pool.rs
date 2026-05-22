//! Heroic Tragedy conquered notable pool (Kalguur).
//! Order matches [POE2DB Timeless Jewel Notable](https://poe2db.tw/us/Notable#TimelessJewelNotable):
//! **Scorched Earth** … **Spider's Lesson** (user wrote “Spider's Blessing”; wiki name is Lesson).

/// Spawn weight for every conquered notable in this pool (in-game / POE2DB for this band).
pub const KALGUUR_NOTABLE_SPAWN_WEIGHT: u32 = 500;

/// `kalguur_notableN` ids in ascending order (N = 1..=37).
pub const KALGUUR_NOTABLE_IDS: [u32; 37] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
];

/// First and last `kalguur_notable` index in the pool.
pub const KALGUUR_NOTABLE_MIN: u32 = 1;
pub const KALGUUR_NOTABLE_MAX: u32 = 37;

pub const KALGUUR_NOTABLE_NAMES: [&str; 37] = [
    "Scorched Earth",
    "War Tactics",
    "Kalguuran Forged",
    "Born of Middengard",
    "Force of Will",
    "Runic Flows",
    "Siege Mentality",
    "Rune Knight",
    "Fiery Leadership",
    "Druidic Alliance",
    "Steel and Sorcery",
    "Triskelion's Light",
    "Stormtossed Voyager",
    "Wrest Control",
    "Firedancer",
    "Mercenary's Lot",
    "Vorana's Fury",
    "Sensible Precautions",
    "Aim for the Jugular",
    "Survival Plan",
    "War of Attrition",
    "Guerilla Warfare",
    "One for the Road",
    "Targeted Strike",
    "Steel Bastion",
    "Forceful Energies",
    "Winter Forest",
    "Fight as One",
    "Summer Meadows",
    "Druidic Training",
    "Corrupted Vision",
    "Runic Tattoos",
    "Furs and Leather",
    "Sudden Hail",
    "Natural Energies",
    "Oaken Form",
    "Spider's Lesson",
];

pub fn name_for(id: u32) -> Option<&'static str> {
    if id < KALGUUR_NOTABLE_MIN || id > KALGUUR_NOTABLE_MAX {
        return None;
    }
    Some(KALGUUR_NOTABLE_NAMES[(id - 1) as usize])
}
