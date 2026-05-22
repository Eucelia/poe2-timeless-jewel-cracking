//! Faithful port of Vilsol/timeless-jewels `random` package (TinyMT32 variant).
//! POE2 initializes TinyMT with three seeds: `[passive_skills_hash, jewel_seed, hidden_salt]`.
//!
//! Jewel type is fixed to Heroic Tragedy (POE2 timeless jewel). Scope is conquered **notables**
//! only (keystones / conqueror mods are not modeled).

pub mod fixtures;
pub mod kalguur_pool;
pub use fixtures::{NotableReplacementCase, JEWEL_SEED_6790, SEED_6790_CASES};
pub use kalguur_pool::{
    name_for, spawn_weight_for, KALGUUR_NOTABLE_IDS, KALGUUR_NOTABLE_MAX, KALGUUR_NOTABLE_MIN,
    KALGUUR_NOTABLE_NAMES, KALGUUR_NOTABLE_SPAWN_WEIGHTS,
};

/// Vilsol `data.JewelType` values (`iota + 1` in Go).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum JewelType {
    HeroicTragedy = 6,
}

/// Only jewel type used by this crate.
pub const JEWEL_TYPE: JewelType = JewelType::HeroicTragedy;

/// `TimelessJewelSeedRanges[HeroicTragedy]` from Vilsol timeless-jewels.
pub const JEWEL_SEED_MIN: u32 = 100;
pub const JEWEL_SEED_MAX: u32 = 8000;

/// `GetAlternateTreeVersionIndex(uint32(JEWEL_TYPE))` → `AlternateTreeVersions` row `_key`.
pub const ALTERNATE_TREE_VERSION_INDEX: u32 = JEWEL_TYPE as u32;

const INITIAL_STATE: [u32; 4] = [
    0x4033_6050,
    0xCFA3_723C,
    0x3CAC_5F6F,
    0x3793_FDFF,
];

const TINYMT32_SH0: u32 = 1;
const TINYMT32_SH1: u32 = 10;
const TINYMT32_MASK: u32 = 0x7FFF_FFFF;
const TINYMT32_ALPHA: u32 = 0x0019_660D;
const TINYMT32_BRAVO: u32 = 0x5D58_8B65;

const MAT1: u32 = 0x8F70_11EE;
const MAT2: u32 = 0xFC78_FF1F;
const TMAT: u32 = 0x3793_FDFF;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TinyMt32 {
    state: [u32; 4],
}

impl Default for TinyMt32 {
    fn default() -> Self {
        Self {
            state: INITIAL_STATE,
        }
    }
}

impl TinyMt32 {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Full `Initialize` with an arbitrary seed list (game may use 3+ values).
    pub fn initialize(&mut self, seeds: &[u32]) {
        self.state = INITIAL_STATE;
        let mut index: u32 = 1;
        for &seed in seeds {
            alpha_round_with_seed(&mut self.state, &mut index, seed);
        }
        for _ in 0..5 {
            alpha_round_index_only(&mut self.state, &mut index);
        }
        for _ in 0..4 {
            bravo_round(&mut self.state, &mut index);
        }
        for _ in 0..8 {
            self.generate_next_state();
        }
    }

    /// Known graph id + jewel seed; third value is the hidden salt under test.
    #[inline(always)]
    pub fn initialize_with_salt(&mut self, graph_id: u32, jewel_seed: u32, hidden_salt: u32) {
        let mut s = PartialInit::new();
        s.feed_seed(graph_id);
        s.feed_seed(jewel_seed);
        s.finish(hidden_salt, &mut self.state);
        for _ in 0..8 {
            self.generate_next_state();
        }
    }

    #[inline(always)]
    pub fn generate_next_state(&mut self) {
        let a = self.state[3];
        let b = (self.state[0] & TINYMT32_MASK) ^ self.state[1] ^ self.state[2];

        let a = a ^ (a << TINYMT32_SH0);
        let b = b ^ (b >> TINYMT32_SH0) ^ a;

        let old1 = self.state[1];
        self.state[0] = old1;
        self.state[1] = self.state[2];
        self.state[2] = a ^ (b << TINYMT32_SH1);
        self.state[3] = b;

        let mask = b & 1;
        self.state[1] ^= mask.wrapping_neg() & MAT1;
        self.state[2] ^= mask.wrapping_neg() & MAT2;
    }

    #[inline(always)]
    pub fn temper(&self) -> u32 {
        let b = self.state[0].wrapping_add(self.state[2] >> 8);
        let a = self.state[3] ^ b;
        a ^ (b & 1).wrapping_neg() & TMAT
    }

    #[inline(always)]
    pub fn generate_uint(&mut self) -> u32 {
        self.generate_next_state();
        self.temper()
    }

    #[inline(always)]
    pub fn generate_single(&mut self, exclusive_maximum: u32) -> u32 {
        debug_assert!(exclusive_maximum > 0);
        self.generate_uint() % exclusive_maximum
    }

    /// Vilsol `Generate(min, max)` inclusive on both ends.
    #[inline(always)]
    pub fn generate_range(&mut self, min_value: u32, max_value: u32) -> u32 {
        let a = min_value.wrapping_add(0x8000_0000);
        let b = max_value.wrapping_add(0x8000_0000);
        let roll = self.generate_single(b - a + 1);
        roll.wrapping_add(a).wrapping_add(0x8000_0000)
    }
}

/// POE2 TinyMT init: `[passive_skills_hash, jewel_seed, hidden_salt]`.
#[inline(always)]
pub fn reset_rng(
    rng: &mut TinyMt32,
    passive_skills_hash: u32,
    jewel_seed: u32,
    hidden_salt: u32,
) {
    rng.initialize_with_salt(passive_skills_hash, jewel_seed, hidden_salt);
}

/// Roll which conquered notable replaces a radius notable (ascending `kalguur_notable` ids).
pub fn roll_kalguur_notable(rng: &mut TinyMt32) -> u32 {
    let mut rolled = KALGUUR_NOTABLE_MIN;
    let mut cumulative = 0u32;
    for id in KALGUUR_NOTABLE_IDS {
        let weight = KALGUUR_NOTABLE_SPAWN_WEIGHTS[(id - 1) as usize];
        cumulative = cumulative.saturating_add(weight);
        if rng.generate_single(cumulative) < weight {
            rolled = id;
        }
    }
    rolled
}

/// Full notable replacement: 3-seed init → weighted pick.
pub fn roll_notable_replacement(
    passive_skills_hash: u32,
    jewel_seed: u32,
    hidden_salt: u32,
) -> u32 {
    let mut rng = TinyMt32::new();
    reset_rng(&mut rng, passive_skills_hash, jewel_seed, hidden_salt);
    // rng.generate_range(0, 100);
    roll_kalguur_notable(&mut rng)
}

/// True when `hidden_salt` reproduces every `cases` observation for `jewel_seed`.
pub fn hidden_salt_matches_cases(
    jewel_seed: u32,
    hidden_salt: u32,
    cases: &[NotableReplacementCase],
) -> bool {
    cases.iter().all(|case| {
        roll_notable_replacement(case.old_hash, jewel_seed, hidden_salt) == case.new_kalguur_notable
    })
}

/// Total candidate third-seed values (`0..=u32::MAX`).
pub const HIDDEN_SALT_SEARCH_SPACE: u64 = u32::MAX as u64 + 1;

/// Format a count or rate with `K` / `M` / `B` suffixes (decimal thousands).
fn format_compact(n: f64) -> String {
    const K: f64 = 1_000.0;
    const M: f64 = 1_000_000.0;
    const B: f64 = 1_000_000_000.0;

    fn scaled(v: f64, suffix: char) -> String {
        if v >= 100.0 {
            format!("{v:.0}{suffix}")
        } else if v >= 10.0 {
            format!("{v:.1}{suffix}")
        } else {
            format!("{v:.2}{suffix}")
        }
    }

    if n >= B {
        scaled(n / B, 'B')
    } else if n >= M {
        scaled(n / M, 'M')
    } else if n >= K {
        scaled(n / K, 'K')
    } else {
        format!("{n:.0}")
    }
}

/// Bruteforce the jewel-wide third seed that satisfies all replacement observations.
///
/// Prints progress to stderr about once per second (checked count, %, rate).
pub fn find_hidden_salt_for_cases(
    jewel_seed: u32,
    cases: &[NotableReplacementCase],
) -> Option<u32> {
    find_hidden_salt_for_cases_with_progress(jewel_seed, cases, std::time::Duration::from_secs(1))
}

/// Same as [`find_hidden_salt_for_cases`] with a custom progress print interval.
pub fn find_hidden_salt_for_cases_with_progress(
    jewel_seed: u32,
    cases: &[NotableReplacementCase],
    progress_interval: std::time::Duration,
) -> Option<u32> {
    use std::io::Write;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
    use std::sync::Arc;
    use std::time::Instant;

    let found = AtomicU32::new(u32::MAX);
    let done = Arc::new(AtomicBool::new(false));
    let checked = Arc::new(AtomicU64::new(0));
    let started = Instant::now();

    let progress_checked = Arc::clone(&checked);
    let progress_done = Arc::clone(&done);
    let progress_handle = std::thread::spawn(move || {
        while !progress_done.load(Ordering::Relaxed) {
            std::thread::sleep(progress_interval);
            if progress_done.load(Ordering::Relaxed) {
                break;
            }
            let n = progress_checked.load(Ordering::Relaxed);
            let elapsed = started.elapsed().as_secs_f64().max(1e-9);
            let rate = n as f64 / elapsed;
            let pct = (n as f64 / HIDDEN_SALT_SEARCH_SPACE as f64) * 100.0;
            let _ = writeln!(
                std::io::stderr(),
                "hidden salt search: checked {} / {} ({pct:.4}%) — {}/s",
                format_compact(n as f64),
                format_compact(HIDDEN_SALT_SEARCH_SPACE as f64),
                format_compact(rate),
            );
        }
    });

    let done_worker = Arc::clone(&done);
    (0u32..=u32::MAX).into_par_iter().for_each(|salt| {
        checked.fetch_add(1, Ordering::Relaxed);
        if done_worker.load(Ordering::Relaxed) {
            return;
        }
        if hidden_salt_matches_cases(jewel_seed, salt, cases) {
            found.store(salt, Ordering::Relaxed);
            done_worker.store(true, Ordering::Relaxed);
        }
    });

    done.store(true, Ordering::Relaxed);
    let _ = progress_handle.join();

    let n = checked.load(Ordering::Relaxed);
    let elapsed = started.elapsed().as_secs_f64();
    let v = found.load(Ordering::Relaxed);
    if v == u32::MAX {
        eprintln!(
            "hidden salt search: finished — no match ({} checked in {elapsed:.1}s)",
            format_compact(n as f64),
        );
        None
    } else {
        eprintln!(
            "hidden salt search: found {v} ({} checked in {elapsed:.1}s)",
            format_compact(n as f64),
        );
        Some(v)
    }
}

/// State after the first two seeds — reuse across all third-seed candidates.
#[derive(Clone, Copy)]
pub struct PartialInit {
    state: [u32; 4],
    index: u32,
}

impl PartialInit {
    pub fn new() -> Self {
        Self {
            state: INITIAL_STATE,
            index: 1,
        }
    }

    pub fn feed_seed(&mut self, seed: u32) {
        alpha_round_with_seed(&mut self.state, &mut self.index, seed);
    }

    /// Third seed + closing alpha/bravo rounds (not the final 8× `generate_next_state`).
    pub fn finish(&self, hidden_salt: u32, out: &mut [u32; 4]) {
        let mut state = self.state;
        let mut index = self.index;
        alpha_round_with_seed(&mut state, &mut index, hidden_salt);
        for _ in 0..5 {
            alpha_round_index_only(&mut state, &mut index);
        }
        for _ in 0..4 {
            bravo_round(&mut state, &mut index);
        }
        *out = state;
    }
}

#[inline(always)]
fn manipulate_alpha(value: u32) -> u32 {
    (value ^ (value >> 27)).wrapping_mul(TINYMT32_ALPHA)
}

#[inline(always)]
fn manipulate_bravo(value: u32) -> u32 {
    (value ^ (value >> 27)).wrapping_mul(TINYMT32_BRAVO)
}

#[inline(always)]
fn alpha_round_with_seed(state: &mut [u32; 4], index: &mut u32, seed: u32) {
    let i = *index % 4;
    let i1 = (*index + 1) % 4;
    let im1 = (*index + 3) % 4; // (index + 4 - 1) % 4

    let mut round_state = manipulate_alpha(state[i as usize] ^ state[i1 as usize] ^ state[im1 as usize]);
    state[i1 as usize] = state[i1 as usize].wrapping_add(round_state);
    round_state = round_state.wrapping_add(seed).wrapping_add(*index);
    state[(((*index + 1) + 1) % 4) as usize] =
        state[(((*index + 1) + 1) % 4) as usize].wrapping_add(round_state);
    state[i as usize] = round_state;
    *index = (*index + 1) % 4;
}

#[inline(always)]
fn alpha_round_index_only(state: &mut [u32; 4], index: &mut u32) {
    alpha_round_with_seed(state, index, 0);
    // Go adds only `index`, not seed — fix by subtracting the 0 we added as seed:
    // roundState += index  (seed term absent). Our helper added seed=0, which is correct.
}

#[inline(always)]
fn bravo_round(state: &mut [u32; 4], index: &mut u32) {
    let i = *index % 4;
    let i1 = (*index + 1) % 4;
    let im1 = (*index + 3) % 4;

    let mut round_state = manipulate_bravo(
        state[i as usize]
            .wrapping_add(state[i1 as usize])
            .wrapping_add(state[im1 as usize]),
    );
    state[i1 as usize] ^= round_state;
    round_state = round_state.wrapping_sub(*index);
    state[(((*index + 1) + 1) % 4) as usize] ^= round_state;
    state[i as usize] = round_state;
    *index = (*index + 1) % 4;
}

use rayon::prelude::*;

/// Match the first tempered uint32 after initialization (one `generate_uint` call).
pub fn find_salt_by_first_roll(
    graph_id: u32,
    jewel_seed: u32,
    expected_first_roll: u32,
) -> Option<u32> {
    let partial = {
        let mut p = PartialInit::new();
        p.feed_seed(graph_id);
        p.feed_seed(jewel_seed);
        p
    };

    let found = std::sync::atomic::AtomicU32::new(u32::MAX);
    let done = std::sync::atomic::AtomicBool::new(false);

    (0u32..=u32::MAX).into_par_iter().for_each(|salt| {
        if done.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let mut state = INITIAL_STATE;
        partial.finish(salt, &mut state);
        let mut rng = TinyMt32 { state };
        for _ in 0..8 {
            rng.generate_next_state();
        }
        rng.generate_next_state();
        if rng.temper() == expected_first_roll {
            found.store(salt, std::sync::atomic::Ordering::Relaxed);
            done.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    });

    let v = found.load(std::sync::atomic::Ordering::Relaxed);
    if v == u32::MAX {
        None
    } else {
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_6790_fixture() {
        assert_eq!(JEWEL_SEED_6790, 6790);
        assert_eq!(SEED_6790_CASES.len(), 9);
        let bone = &SEED_6790_CASES[1];
        assert_eq!(bone.old_name, "Bone Chains");
        assert_eq!(bone.old_hash, 26563);
        assert_eq!(bone.new_name, "Steel Bastion");
        assert_eq!(bone.new_kalguur_notable, 25);
        // Two different originals roll the same conquered notable.
        let fiery: Vec<_> = SEED_6790_CASES
            .iter()
            .filter(|c| c.new_kalguur_notable == 9)
            .map(|c| c.old_hash)
            .collect();
        assert_eq!(fiery.len(), 2);
        assert!(fiery.contains(&41394)); // Invigorating Archon
        assert!(fiery.contains(&36623)); // Convalescence
    }

    #[test]
    #[ignore = "bruteforces u32 hidden salt; run: cargo test find_seed_6790_hidden_salt -- --ignored --nocapture"]
    fn find_seed_6790_hidden_salt() {
        let salt = find_hidden_salt_for_cases(JEWEL_SEED_6790, SEED_6790_CASES)
            .expect("no hidden salt satisfies all SEED_6790_CASES");
        println!("JEWEL_SEED_6790 hidden_salt = {salt}");
        assert!(hidden_salt_matches_cases(JEWEL_SEED_6790, salt, SEED_6790_CASES));
    }

    #[test]
    #[ignore = "set fixtures::JEWEL_SEED_6790_HIDDEN_SALT after find_seed_6790_hidden_salt"]
    fn seed_6790_three_seed_rolls() {
        let salt = fixtures::JEWEL_SEED_6790_HIDDEN_SALT
            .expect("run find_seed_6790_hidden_salt and paste salt into fixtures.rs");
        for case in SEED_6790_CASES {
            let rolled = roll_notable_replacement(case.old_hash, JEWEL_SEED_6790, salt);
            assert_eq!(
                rolled,
                case.new_kalguur_notable,
                "{} (hash {}) expected {} got {} ({})",
                case.old_name,
                case.old_hash,
                case.new_name,
                rolled,
                name_for(rolled).unwrap_or("?"),
            );
        }
    }
}
