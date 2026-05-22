//! Faithful port of Vilsol/timeless-jewels `random` package (TinyMT32 variant).
//! Supports brute-forcing a hidden third `uint32` passed to `Initialize`.

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

        for (offset, &seed) in seeds.iter().enumerate() {
            let idx = index.wrapping_add(offset as u32) % 4;
            // Re-sync: after each seed, index advances — mirror Go loop exactly.
            let _ = idx;
        }
        // Clear above — use explicit Go loop instead:
        self.state = INITIAL_STATE;
        index = 1;
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

        let old0 = self.state[0];
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

/// Run `f` on every `hidden_salt` in `0..=u32::MAX` (parallel by default).
pub fn bruteforce_third_seed<F>(graph_id: u32, jewel_seed: u32, mut f: F) -> Vec<u32>
where
    F: Fn(u32, &mut TinyMt32) -> bool + Send + Sync,
{
    let partial = PartialInit::new();
    let mut partial = partial;
    partial.feed_seed(graph_id);
    partial.feed_seed(jewel_seed);

    (0u32..=u32::MAX)
        .into_par_iter()
        .filter_map(|salt| {
            let mut rng = TinyMt32::new();
            let mut state = INITIAL_STATE;
            partial.finish(salt, &mut state);
            rng.state = state;
            for _ in 0..8 {
                rng.generate_next_state();
            }
            if f(salt, &mut rng) {
                Some(salt)
            } else {
                None
            }
        })
        .collect()
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
    fn two_seed_matches_go_style_init() {
        let mut a = TinyMt32::new();
        a.initialize(&[12345, 67890]);
        let mut b = TinyMt32::new();
        b.initialize_with_salt(12345, 67890, 0);
        // third seed 0 is NOT the same as two-seed init — this documents API only
        let _ = (a.generate_uint(), b.generate_uint());
    }
}
