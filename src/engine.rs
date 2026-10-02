// Fast board: 16 cells x 5 bits packed in a u128 (exponents up to 31, the game needs 17).
// Cell (row, col) lives at bit 20 * row + 5 * col, so a row is a contiguous 20-bit key
// and left/right moves are 4 table lookups. Up/down go through a transposition.
//
// Spawn rule (identical to the referee): free cells are listed column by column
// (col outer, row inner), the tile goes to free[seed % count], value is 2 if
// seed & 0x10 == 0 else 4, then seed = seed^2 mod 50515093.
// Column-major order of a board is the row-major order of its transpose.

pub type B = u128;

pub const ROW_BITS: u32 = 20;
pub const ROW_MASK: u128 = (1 << ROW_BITS) - 1;
pub const LINE_COUNT: usize = 1 << ROW_BITS;
pub const MODULO: u64 = 50515093;

// moves are encoded with the output letters order
pub const UP: u8 = 0;
pub const RIGHT: u8 = 1;
pub const DOWN: u8 = 2;
pub const LEFT: u8 = 3;
pub const MOVE_CHARS: [u8; 4] = *b"URDL";

// selects the cells (r, c) taking part in a step of the transposition
const fn cell_mask(kind: u8) -> u128 {
    let mut m = 0u128;
    let mut i = 0;
    while i < 16 {
        let (r, c) = (i >> 2, i & 3);
        let take = match kind {
            0 => (r & 1) == (c & 1),
            1 => (r & 1) == 0 && (c & 1) == 1,
            2 => (r & 1) == 1 && (c & 1) == 0,
            3 => (r < 2) == (c < 2),
            4 => r < 2 && c >= 2,
            _ => r >= 2 && c < 2,
        };
        if take {
            m |= 0x1f << (5 * i);
        }
        i += 1;
    }
    m
}

// step 1 transposes each 2x2 block, step 2 swaps the two off-diagonal blocks
const T1_KEEP: u128 = cell_mask(0);
const T1_FWD: u128 = cell_mask(1); // +3 cells
const T1_BWD: u128 = cell_mask(2); // -3 cells
const T2_KEEP: u128 = cell_mask(3);
const T2_FWD: u128 = cell_mask(4); // +6 cells
const T2_BWD: u128 = cell_mask(5); // -6 cells
// lowest bit of every cell
pub const LOW_BITS: u128 = low_bits();

const fn low_bits() -> u128 {
    let mut m = 0u128;
    let mut i = 0;
    while i < 16 {
        m |= 1 << (5 * i);
        i += 1;
    }
    m
}

#[inline(always)]
pub fn transpose(x: B) -> B {
    let a = (x & T1_KEEP) | ((x & T1_FWD) << 15) | ((x & T1_BWD) >> 15);
    (a & T2_KEEP) | ((a & T2_FWD) << 30) | ((a & T2_BWD) >> 30)
}

#[inline(always)]
pub fn get(b: B, idx: usize) -> u8 {
    ((b >> (5 * idx)) & 0x1f) as u8
}

#[inline(always)]
pub fn row(b: B, r: u32) -> usize {
    ((b >> (ROW_BITS * r)) & ROW_MASK) as usize
}

pub fn from_cells(cells: &[u8; 16]) -> B {
    let mut b = 0u128;
    for (i, &v) in cells.iter().enumerate() {
        b |= (v as u128) << (5 * i);
    }
    b
}

pub fn to_cells(b: B) -> [u8; 16] {
    let mut cells = [0u8; 16];
    for (i, c) in cells.iter_mut().enumerate() {
        *c = get(b, i);
    }
    cells
}

#[inline(always)]
pub fn next_seed(seed: u64) -> u64 {
    seed * seed % MODULO
}

#[inline(always)]
pub fn spawn_value(seed: u64) -> u128 {
    if seed & 0x10 == 0 {
        1
    } else {
        2
    }
}

/// Bit mask (one bit per cell, at its lowest bit) of the empty cells.
#[inline(always)]
pub fn empty_bits(b: B) -> u128 {
    let nz = b | (b >> 1) | (b >> 2) | (b >> 3) | (b >> 4);
    !nz & LOW_BITS
}

/// Spawn a tile on board `b` given its transpose `t`; both are updated.
/// Returns false if the board is full (cannot happen after a valid move).
#[inline(always)]
pub fn spawn(b: &mut B, t: &mut B, seed: u64) -> bool {
    let mut e = empty_bits(*t);
    let count = e.count_ones() as u64;
    if count == 0 {
        return false;
    }
    let mut k = seed % count;
    while k > 0 {
        e &= e - 1;
        k -= 1;
    }
    let pos = e.trailing_zeros();
    let v = spawn_value(seed);
    *t |= v << pos;
    let i = pos / 5; // index in the transpose = 4 * col + row
    let (col, r) = (i >> 2, i & 3);
    *b |= v << (5 * (4 * r + col));
    true
}

/// Score contribution of the tiles: a tile 2^k built from 2s scored (k-1) * 2^k.
/// real score = tile_score(board) - 4 * (number of spawned 4s)
pub fn tile_score(b: B) -> u64 {
    let mut s = 0u64;
    for i in 0..16 {
        let v = get(b, i) as u64;
        if v >= 2 {
            s += (v - 1) << v;
        }
    }
    s
}

/// Sum of the tiles.
pub fn mass(b: B) -> u64 {
    (0..16).map(|i| get(b, i)).filter(|&v| v > 0).map(|v| 1u64 << v).sum()
}

/// Number of moves before the game is necessarily over: the mass only depends on the
/// spawned values (so on the seed), and a mass with 16 bits set needs 16 distinct tiles,
/// i.e. a full board without any merge.
pub fn moves_until_forced_death(mut mass: u64, mut seed: u64) -> u64 {
    let mut moves = 0;
    while mass.count_ones() < 16 && moves < 1_000_000 {
        mass += 1 << spawn_value(seed);
        seed = next_seed(seed);
        moves += 1;
    }
    moves
}

pub fn max_tile(b: B) -> u8 {
    (0..16).map(|i| get(b, i)).max().unwrap()
}

pub struct Tables {
    /// row moved towards cell 0 ("left" on rows, "up" on the transpose)
    pub left: Vec<u32>,
    /// row moved towards cell 3
    pub right: Vec<u32>,
}

#[inline(always)]
fn decode(key: usize) -> [u32; 4] {
    [
        (key & 0x1f) as u32,
        ((key >> 5) & 0x1f) as u32,
        ((key >> 10) & 0x1f) as u32,
        ((key >> 15) & 0x1f) as u32,
    ]
}

#[inline(always)]
fn encode(line: [u32; 4]) -> u32 {
    line[0] | (line[1] << 5) | (line[2] << 10) | (line[3] << 15)
}

pub fn decode_line(key: usize) -> [u32; 4] {
    decode(key)
}

fn slide_left(line: [u32; 4]) -> [u32; 4] {
    let mut out = [0u32; 4];
    let mut n = 0;
    let mut can_merge = false;
    for &v in line.iter() {
        if v == 0 {
            continue;
        }
        if can_merge && out[n - 1] == v {
            out[n - 1] = (v + 1).min(31);
            can_merge = false;
        } else {
            out[n] = v;
            n += 1;
            can_merge = true;
        }
    }
    out
}

impl Tables {
    pub fn new() -> Tables {
        let mut left = vec![0u32; LINE_COUNT];
        let mut right = vec![0u32; LINE_COUNT];
        for key in 0..LINE_COUNT {
            let line = decode(key);
            left[key] = encode(slide_left(line));
            let rev = [line[3], line[2], line[1], line[0]];
            let r = slide_left(rev);
            right[key] = encode([r[3], r[2], r[1], r[0]]);
        }
        Tables { left, right }
    }

    #[inline(always)]
    pub fn move_rows(&self, b: B, table: &[u32]) -> B {
        (table[row(b, 0)] as u128)
            | ((table[row(b, 1)] as u128) << 20)
            | ((table[row(b, 2)] as u128) << 40)
            | ((table[row(b, 3)] as u128) << 60)
    }

    /// Apply a move without spawning. Returns the new board (equal to `b` if invalid).
    pub fn apply(&self, b: B, mv: u8) -> B {
        match mv {
            LEFT => self.move_rows(b, &self.left),
            RIGHT => self.move_rows(b, &self.right),
            UP => transpose(self.move_rows(transpose(b), &self.left)),
            _ => transpose(self.move_rows(transpose(b), &self.right)),
        }
    }

    pub fn is_game_over(&self, b: B) -> bool {
        (0..4).all(|m| self.apply(b, m) == b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim;

    fn sim_action(mv: u8) -> u8 {
        // sim: 1 = U, 2 = L, 3 = D, 4 = R
        match mv {
            UP => 1,
            LEFT => 2,
            DOWN => 3,
            _ => 4,
        }
    }

    #[test]
    fn transpose_roundtrip() {
        let cells: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17];
        let b = from_cells(&cells);
        let t = transpose(b);
        for r in 0..4 {
            for c in 0..4 {
                assert_eq!(get(t, 4 * c + r), cells[4 * r + c]);
            }
        }
        assert_eq!(transpose(t), b);
    }

    #[test]
    fn matches_reference_simulator() {
        let tables = Tables::new();
        let mut rng = 12345u64;
        for seed in [42u64, 290797, 10682358, 38333962, 47049887] {
            let mut reference = sim::Board::new(seed);
            let mut b = from_cells(&reference.board);
            let mut t = transpose(b);
            let mut s = reference.seed;
            let mut n4 = reference.board.iter().filter(|&&v| v == 2).count() as u64;
            let mut steps = 0;
            while !reference.is_game_over() && steps < 5000 {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                let mv = (rng % 4) as u8;
                let nb = tables.apply(b, mv);
                let moved = reference.play(sim_action(mv));
                assert_eq!(moved, nb != b);
                if moved {
                    b = nb;
                    t = transpose(b);
                    if spawn_value(s) == 2 {
                        n4 += 1;
                    }
                    assert!(spawn(&mut b, &mut t, s));
                    s = next_seed(s);
                    steps += 1;
                }
                assert_eq!(to_cells(b), reference.board);
                assert_eq!(t, transpose(b));
                assert_eq!(s, reference.seed);
                assert_eq!(tile_score(b) - 4 * n4, reference.score as u64);
            }
            assert_eq!(tables.is_game_over(b), reference.is_game_over());
        }
    }
}
