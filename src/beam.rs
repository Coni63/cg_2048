use std::collections::VecDeque;
use std::time::Instant;

use crate::engine::*;
use crate::eval::Evaluator;

pub const MAX_TURNS: usize = 600;

// Persistent beam search.
//
// The beam is never restarted: layer 0 is the real current state, the last layer is the
// frontier. Each turn the frontier is pushed forward (fixed number of layers, or until the
// time budget is spent), then the best frontier node is chosen and only the first
// `ahead - horizon` moves of its path are played. Nodes that do not descend from the
// played prefix are pruned, the remaining tree is kept for the next turn.
//
// All nodes of a layer share the same seed (the seed only depends on the number of moves),
// so identical boards in a layer are true duplicates and are merged.

#[derive(Clone, Copy)]
struct Node {
    b: B,
    eval: f64,
    parent: u32,
    mv: u8,
    dead: bool, // no legal move: game over on this node
}

struct Layer {
    nodes: Vec<Node>,
    /// seed used for the spawn that creates the next layer
    seed: u64,
    /// spawned 4s since the root layer was created (for terminal scores)
    n4: u64,
}

#[derive(Clone, Debug)]
pub struct BeamParams {
    /// beam width (initial width in time mode)
    pub width: usize,
    /// number of layers kept unplayed in front of the played moves
    pub horizon: usize,
    /// fixed mode: layers expanded per turn; time mode: target used to adapt the width.
    /// 0 = auto: the moves left before the forced game over spread over the turns left
    pub layers_per_turn: usize,
    /// time budget per turn in ms, 0 = fixed mode
    pub time_ms: f64,
    pub first_time_ms: f64,
    /// false: UP is only tried when no other move is legal (smaller tree, but loses games)
    pub always_up: bool,
    pub min_width: usize,
    pub max_width: usize,
}

impl Default for BeamParams {
    fn default() -> Self {
        BeamParams {
            width: 1000,
            horizon: 200,
            layers_per_turn: 0,
            time_ms: 40.0,
            first_time_ms: 900.0,
            always_up: true,
            min_width: 20,
            max_width: 200_000,
        }
    }
}

struct DedupSet {
    keys: Vec<B>,
    stamps: Vec<u32>,
    gen: u32,
    shift: u32,
}

impl DedupSet {
    fn new() -> DedupSet {
        DedupSet {
            keys: Vec::new(),
            stamps: Vec::new(),
            gen: 0,
            shift: 64,
        }
    }

    fn reset(&mut self, expected: usize) {
        let mut bits = 4;
        while (1usize << bits) < 2 * expected {
            bits += 1;
        }
        if self.keys.len() < (1 << bits) {
            self.keys = vec![0; 1 << bits];
            self.stamps = vec![0; 1 << bits];
            self.gen = 0;
        }
        self.shift = 64 - self.keys.len().trailing_zeros();
        self.gen = self.gen.wrapping_add(1);
        if self.gen == 0 {
            self.stamps.iter_mut().for_each(|s| *s = 0);
            self.gen = 1;
        }
    }

    /// true if the key was not present
    #[inline(always)]
    fn insert(&mut self, k: B) -> bool {
        let x = (k as u64) ^ ((k >> 64) as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let mask = self.keys.len() - 1;
        let mut h = (x.wrapping_mul(0xBF58_476D_1CE4_E5B9) >> self.shift) as usize;
        loop {
            if self.stamps[h] != self.gen {
                self.stamps[h] = self.gen;
                self.keys[h] = k;
                return true;
            }
            if self.keys[h] == k {
                return false;
            }
            h = (h + 1) & mask;
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct TurnStats {
    pub layers: usize,
    pub width: usize,
    pub played: usize,
    pub ahead: usize,
    pub frontier: usize,
    pub resync: bool,
}

pub struct Agent<'a> {
    tables: &'a Tables,
    eval: &'a Evaluator,
    pub params: BeamParams,
    layers: VecDeque<Layer>,
    cand: Vec<Node>,
    dedup: DedupSet,
    width: usize,
    /// index of the current state in layer 0
    root: usize,
    /// time reserved for the end of turn (choice of the path + rebase), decaying max
    commit_ms: f64,
    /// moves left before the forced game over
    moves_left: u64,
    turn: usize,
    pub stats: TurnStats,
}

impl<'a> Agent<'a> {
    pub fn new(tables: &'a Tables, eval: &'a Evaluator, params: BeamParams) -> Agent<'a> {
        let width = params.width;
        Agent {
            tables,
            eval,
            params,
            layers: VecDeque::new(),
            cand: Vec::new(),
            dedup: DedupSet::new(),
            width,
            root: 0,
            commit_ms: 1.0,
            moves_left: 0,
            turn: 0,
            stats: TurnStats::default(),
        }
    }

    fn reset(&mut self, b: B, seed: u64) {
        self.layers.clear();
        self.layers.push_back(Layer {
            nodes: vec![Node {
                b,
                eval: 0.0,
                parent: 0,
                mv: 0,
                dead: false,
            }],
            seed,
            n4: 0,
        });
        self.root = 0;
        self.moves_left = moves_until_forced_death(mass(b), seed);
    }

    /// Moves to play per turn to reach the forced game over within the turn limit.
    fn pace(&self) -> usize {
        let turns_left = MAX_TURNS.saturating_sub(self.turn + 3).max(1) as u64;
        ((self.moves_left * 21 / 20).div_ceil(turns_left)).max(1) as usize
    }

    fn layers_target(&self) -> usize {
        if self.params.layers_per_turn > 0 {
            self.params.layers_per_turn
        } else {
            self.pace()
        }
    }

    /// Expand the frontier by one layer. Returns the size of the new layer (0: beam is dead).
    fn expand(&mut self) -> usize {
        let Agent {
            tables,
            eval,
            params,
            layers,
            cand,
            dedup,
            width,
            ..
        } = self;
        let tables: &Tables = tables;
        let eval: &Evaluator = eval;
        let last = layers.back_mut().unwrap();
        let seed = last.seed;
        let value = spawn_value(seed);
        let first = last.nodes[0].b;
        let mut mass: u64 = 1 << value;
        for i in 0..16 {
            let v = get(first, i);
            if v > 0 {
                mass += 1 << v;
            }
        }
        let inv_mass = 1.0 / ((1u64 << 30) as f64 * mass as f64);

        cand.clear();
        dedup.reset(last.nodes.len() * 3);

        #[inline(always)]
        fn add(
            cand: &mut Vec<Node>,
            dedup: &mut DedupSet,
            eval: &Evaluator,
            mut c: B,
            mut ct: B,
            seed: u64,
            inv_mass: f64,
            parent: usize,
            mv: u8,
        ) {
            spawn(&mut c, &mut ct, seed);
            if dedup.insert(c) {
                cand.push(Node {
                    b: c,
                    eval: eval.eval(c, ct, inv_mass),
                    parent: parent as u32,
                    mv,
                    dead: false,
                });
            }
        }

        for i in 0..last.nodes.len() {
            let b = last.nodes[i].b;
            let t = transpose(b);
            let mut legal = false;

            let ct = tables.move_rows(t, &tables.right);
            if ct != t {
                legal = true;
                add(cand, dedup, eval, transpose(ct), ct, seed, inv_mass, i, DOWN);
            }
            let c = tables.move_rows(b, &tables.left);
            if c != b {
                legal = true;
                add(cand, dedup, eval, c, transpose(c), seed, inv_mass, i, LEFT);
            }
            let c = tables.move_rows(b, &tables.right);
            if c != b {
                legal = true;
                add(cand, dedup, eval, c, transpose(c), seed, inv_mass, i, RIGHT);
            }
            if params.always_up || !legal {
                let ct = tables.move_rows(t, &tables.left);
                if ct != t {
                    legal = true;
                    add(cand, dedup, eval, transpose(ct), ct, seed, inv_mass, i, UP);
                }
            }
            if !legal {
                last.nodes[i].dead = true;
            }
        }

        if cand.is_empty() {
            return 0;
        }
        let w = *width;
        if cand.len() > w {
            cand.select_nth_unstable_by(w - 1, |a, b| b.eval.partial_cmp(&a.eval).unwrap());
            cand.truncate(w);
        }
        let n4 = last.n4 + (value == 2) as u64;
        layers.push_back(Layer {
            nodes: cand.clone(),
            seed: next_seed(seed),
            n4,
        });
        cand.len()
    }

    /// Moves from the root to node `idx` of layer `depth`.
    fn path(&self, depth: usize, mut idx: usize) -> Vec<(u8, usize)> {
        let mut path = Vec::with_capacity(depth);
        for d in (1..=depth).rev() {
            let n = &self.layers[d].nodes[idx];
            path.push((n.mv, idx));
            idx = n.parent as usize;
        }
        path.reverse();
        path
    }

    /// alive[i] for the nodes of layer `to`, given the alive mask of layer `from`
    fn propagate(&self, from: usize, mut alive: Vec<bool>, to: usize) -> Vec<bool> {
        for j in from + 1..=to {
            alive = self.layers[j]
                .nodes
                .iter()
                .map(|n| alive[n.parent as usize])
                .collect();
        }
        alive
    }

    /// Make node `idx` of layer `k` the new root and drop the frontier nodes not descending
    /// from it. Intermediate layers are left untouched (their unreachable nodes are inert),
    /// only the frontier is compacted since it is the only layer that gets expanded.
    fn rebase(&mut self, k: usize, idx: usize) {
        let last = self.layers.len() - 1;
        if last > k {
            let mut alive = vec![false; self.layers[k].nodes.len()];
            alive[idx] = true;
            let alive = self.propagate(k, alive, last - 1);
            self.layers[last].nodes.retain(|n| alive[n.parent as usize]);
            self.root = idx;
        } else {
            let root = self.layers[k].nodes[idx];
            self.layers[k].nodes = vec![root];
            self.root = 0;
        }
        self.layers.drain(..k);
    }

    /// Best game-over node of the tree (used when the whole beam is dead).
    fn best_terminal(&self) -> (usize, usize) {
        let mut best = (0, self.root);
        let mut best_score = i64::MIN;
        let mut alive = vec![false; self.layers[0].nodes.len()];
        alive[self.root] = true;
        for (d, layer) in self.layers.iter().enumerate() {
            if d > 0 {
                alive = self.propagate(d - 1, alive, d);
            }
            for (i, n) in layer.nodes.iter().enumerate() {
                if n.dead && alive[i] {
                    let s = tile_score(n.b) as i64 - 4 * layer.n4 as i64;
                    if s > best_score {
                        best_score = s;
                        best = (d, i);
                    }
                }
            }
        }
        best
    }

    /// Play one turn: `seed` and `cells` (exponents) are the referee input, `start` the
    /// instant the input was received.
    pub fn turn(&mut self, seed: u64, cells: [u8; 16], start: Instant) -> String {
        let b = from_cells(&cells);
        let resync = self.layers.is_empty()
            || self.layers[0].nodes[self.root].b != b
            || self.layers[0].seed != seed;
        if resync {
            self.reset(b, seed);
        }
        let first = self.turn == 0;
        let fixed = self.params.time_ms <= 0.0;
        let budget = if first {
            self.params.first_time_ms
        } else {
            self.params.time_ms
        };
        let layers_target = self.layers_target();
        let target = if first {
            self.params.horizon + layers_target
        } else {
            layers_target
        };

        let mut expanded = 0;
        let mut dead = false;
        let mut last_layer_ms = 0.0;
        loop {
            if fixed || first {
                if expanded >= target {
                    break;
                }
            }
            if !fixed {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                // always keep at least one move to play
                if elapsed + 1.5 * last_layer_ms + self.commit_ms > budget && self.layers.len() > 1 {
                    break;
                }
            }
            let t0 = Instant::now();
            if self.expand() == 0 {
                dead = true;
                break;
            }
            last_layer_ms = t0.elapsed().as_secs_f64() * 1000.0;
            expanded += 1;
        }

        let commit_start = Instant::now();
        let ahead = self.layers.len() - 1;
        let (depth, idx, play) = if dead {
            let (d, i) = self.best_terminal();
            (d, i, d)
        } else {
            let frontier = &self.layers[ahead].nodes;
            let mut best = 0;
            for (i, n) in frontier.iter().enumerate() {
                if n.eval > frontier[best].eval {
                    best = i;
                }
            }
            let play = ahead
                .saturating_sub(self.params.horizon)
                .max(self.pace())
                .min(ahead);
            (ahead, best, play)
        };

        let mut out = String::new();
        if play > 0 {
            let path = self.path(depth, idx);
            for &(mv, _) in &path[..play] {
                out.push(MOVE_CHARS[mv as usize] as char);
            }
            let new_root = path[play - 1].1;
            self.rebase(play, new_root);
        } else {
            // root without legal move: the game is already over
            out.push('U');
        }

        let commit_ms = commit_start.elapsed().as_secs_f64() * 1000.0;
        self.commit_ms = commit_ms.max(self.commit_ms * 0.9);

        self.stats = TurnStats {
            layers: expanded,
            width: self.width,
            played: play,
            ahead,
            frontier: self.layers.back().map_or(0, |l| l.nodes.len()),
            resync,
        };

        if !fixed && first && expanded > 0 {
            // calibrate the width from the cost of a layer during the first turn
            let ms_per_layer = start.elapsed().as_secs_f64() * 1000.0 / expanded as f64;
            let wanted = self.params.time_ms / (layers_target as f64 * ms_per_layer);
            self.width = ((self.width as f64 * wanted.clamp(0.1, 10.0)) as usize)
                .clamp(self.params.min_width, self.params.max_width);
        }
        if !fixed && !first && expanded > 0 {
            let ratio = (expanded as f64 / layers_target as f64).clamp(0.7, 1.3);
            self.width = ((self.width as f64 * ratio) as usize)
                .clamp(self.params.min_width, self.params.max_width);
        }
        self.moves_left = self.moves_left.saturating_sub(play as u64);
        self.turn += 1;
        out
    }
}
