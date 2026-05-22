//! Observed Heroic Tragedy replacements for jewel seed 6790.
//! Original notables: `PassiveSkillsHash` from POE2DB (e.g. [Bone Chains](https://poe2db.tw/us/Bone_Chains)).
//! Conquered notables: `kalguur_notable` code suffix from POE2DB (no PassiveSkillsHash on those pages).

/// Jewel seed shared by every case in this fixture.
pub const JEWEL_SEED_6790: u32 = 6790;

/// Third RNG seed for `JEWEL_SEED_6790`, once discovered via `find_hidden_salt_for_cases`.
/// Run: `cargo test find_seed_6790_hidden_salt -- --ignored --nocapture`
pub const JEWEL_SEED_6790_HIDDEN_SALT: Option<u32> = None;

/// One notable replacement observation used to constrain seed / hidden-salt search.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotableReplacementCase {
    pub old_name: &'static str,
    /// POE2DB `PassiveSkillsHash` for the original notable (RNG graph id).
    pub old_hash: u32,
    pub new_name: &'static str,
    /// POE2DB `kalguur_notableN` code suffix for the conquered notable.
    pub new_kalguur_notable: u32,
}

/// Heroic Tragedy seed **6790** — nine radius notables from in-game observation.
pub const SEED_6790_CASES: &[NotableReplacementCase] = &[
    NotableReplacementCase {
        old_name: "Sanguine Tolerance",
        old_hash: 4810,
        new_name: "Survival Plan",
        new_kalguur_notable: 20,
    },
    NotableReplacementCase {
        old_name: "Bone Chains",
        old_hash: 26563,
        new_name: "Steel Bastion",
        new_kalguur_notable: 25,
    },
    NotableReplacementCase {
        old_name: "Energising Archon",
        old_hash: 43633,
        new_name: "Oaken Form",
        new_kalguur_notable: 36,
    },
    NotableReplacementCase {
        old_name: "Invigorating Archon",
        old_hash: 41394,
        new_name: "Fiery Leadership",
        new_kalguur_notable: 9,
    },
    NotableReplacementCase {
        old_name: "Convalescence",
        old_hash: 36623,
        new_name: "Fiery Leadership",
        new_kalguur_notable: 9,
    },
    NotableReplacementCase {
        old_name: "Turn the Clock Back",
        old_hash: 35564,
        new_name: "Born of Middengard",
        new_kalguur_notable: 4,
    },
    NotableReplacementCase {
        old_name: "Turn the Clock Forward",
        old_hash: 2335,
        new_name: "Natural Energies",
        new_kalguur_notable: 35,
    },
    NotableReplacementCase {
        old_name: "Mental Alacrity",
        old_hash: 16466,
        new_name: "Fight as One",
        new_kalguur_notable: 28,
    },
    NotableReplacementCase {
        old_name: "Tempered Mind",
        old_hash: 8831,
        new_name: "Wrest Control",
        new_kalguur_notable: 14,
    },
];
