use crate::engine::{decode_line, row, B, LINE_COUNT};

// Board evaluation = snake * snake_gradient / scale + sum of line heuristics (4 rows + 4 cols).
//
// - snake gradient: the C# ScoreOrder, sum of 2^(weight + exponent) along a snake ending in
//   the bottom-left corner. Divided by 2^30 * (tile mass of the layer) so it stays in ~[0, 1]
//   whatever the stage of the game (all nodes of a layer share the same mass).
// - line heuristic (nneonneo's 2048 AI): empty cells, merge opportunities, weighted
//   monotonicity and a sum penalty, precomputed for every 20-bit line.

const SNAKE_WEIGHTS: [[u32; 4]; 4] = [
    [1, 2, 4, 6],
    [14, 12, 10, 8],
    [16, 18, 20, 22],
    [30, 28, 26, 24],
];

#[derive(Clone, Debug)]
pub struct EvalParams {
    pub snake: f64,
    pub empty: f64,
    pub merges: f64,
    pub mono: f64,
    pub mono_pow: f64,
    pub sum: f64,
    pub sum_pow: f64,
    /// 0: C# weights, otherwise weight = step * (position along the snake from the tail)
    pub snake_step: f64,
    /// deterministic per-board noise added to the eval (diversity), 0 = off
    pub noise: f64,
    pub noise_seed: f64,
}

impl Default for EvalParams {
    fn default() -> Self {
        EvalParams {
            snake: 1.0,
            empty: 0.0,
            merges: 0.0,
            mono: 0.0,
            mono_pow: 4.0,
            sum: 0.0,
            sum_pow: 3.5,
            snake_step: 0.0,
            noise: 0.0,
            noise_seed: 1.0,
        }
    }
}

impl EvalParams {
    pub fn set(&mut self, name: &str, value: f64) -> bool {
        match name {
            "snake" => self.snake = value,
            "empty" => self.empty = value,
            "merges" => self.merges = value,
            "mono" => self.mono = value,
            "mono_pow" => self.mono_pow = value,
            "sum" => self.sum = value,
            "sum_pow" => self.sum_pow = value,
            "snake_step" => self.snake_step = value,
            "noise" => self.noise = value,
            "noise_seed" => self.noise_seed = value,
            _ => return false,
        }
        true
    }

    fn uses_lines(&self) -> bool {
        self.empty != 0.0 || self.merges != 0.0 || self.mono != 0.0 || self.sum != 0.0
    }
}

pub struct Evaluator {
    pub params: EvalParams,
    use_lines: bool,
    lines: Vec<f32>,
    // [row][half][10-bit key]
    snake: Vec<[[f64; 1024]; 2]>,
    noise_key: u64,
}

fn line_heuristic(p: &EvalParams, line: [u32; 4]) -> f64 {
    let mut sum = 0.0;
    let mut empty = 0.0;
    let mut merges = 0.0;
    let mut prev = 0;
    let mut counter = 0.0;
    for &v in line.iter() {
        sum += (v as f64).powf(p.sum_pow);
        if v == 0 {
            empty += 1.0;
        } else {
            if prev == v {
                counter += 1.0;
            } else if counter > 0.0 {
                merges += 1.0 + counter;
                counter = 0.0;
            }
            prev = v;
        }
    }
    if counter > 0.0 {
        merges += 1.0 + counter;
    }
    let mut mono_left = 0.0;
    let mut mono_right = 0.0;
    for i in 1..4 {
        let a = (line[i - 1] as f64).powf(p.mono_pow);
        let b = (line[i] as f64).powf(p.mono_pow);
        if line[i - 1] > line[i] {
            mono_left += a - b;
        } else {
            mono_right += b - a;
        }
    }
    p.empty * empty + p.merges * merges - p.mono * mono_left.min(mono_right) - p.sum * sum
}

impl Evaluator {
    pub fn new(params: EvalParams) -> Evaluator {
        let use_lines = params.uses_lines();
        let lines = if use_lines {
            (0..LINE_COUNT)
                .map(|k| line_heuristic(&params, decode_line(k)) as f32)
                .collect()
        } else {
            Vec::new()
        };
        let mut weights = SNAKE_WEIGHTS;
        if params.snake_step > 0.0 {
            // position along the snake from the tail: C# weights are already sorted that way
            let mut order: Vec<(u32, usize)> = (0..16).map(|i| (SNAKE_WEIGHTS[i / 4][i % 4], i)).collect();
            order.sort();
            for (pos, &(_, i)) in order.iter().enumerate() {
                weights[i / 4][i % 4] = (params.snake_step * pos as f64).round() as u32;
            }
        }
        let mut snake = vec![[[0f64; 1024]; 2]; 4];
        for (r, halves) in snake.iter_mut().enumerate() {
            for (h, table) in halves.iter_mut().enumerate() {
                for (key, value) in table.iter_mut().enumerate() {
                    for k in 0..2 {
                        let v = ((key >> (5 * k)) & 0x1f) as u32;
                        if v > 0 {
                            *value += (weights[r][2 * h + k] as f64 + v as f64).exp2();
                        }
                    }
                }
            }
        }
        let noise_key = (params.noise_seed as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        Evaluator {
            params,
            use_lines,
            lines,
            snake,
            noise_key,
        }
    }

    #[inline(always)]
    pub fn snake(&self, b: B) -> f64 {
        let mut s = 0.0;
        for r in 0..4 {
            let key = row(b, r as u32);
            s += self.snake[r][0][key & 0x3ff] + self.snake[r][1][key >> 10];
        }
        s
    }

    /// `b` the board, `t` its transpose, `inv_mass` = 1 / (2^30 * tile mass of the layer)
    #[inline(always)]
    pub fn eval(&self, b: B, t: B, inv_mass: f64) -> f64 {
        let mut e = 0.0;
        if self.params.snake != 0.0 {
            e += self.params.snake * self.snake(b) * inv_mass;
        }
        if self.use_lines {
            let l = &self.lines;
            e += (l[row(b, 0)] + l[row(b, 1)] + l[row(b, 2)] + l[row(b, 3)]) as f64;
            e += (l[row(t, 0)] + l[row(t, 1)] + l[row(t, 2)] + l[row(t, 3)]) as f64;
        }
        if self.params.noise != 0.0 {
            let h = ((b as u64) ^ ((b >> 64) as u64).rotate_left(29)).wrapping_mul(self.noise_key);
            e += self.params.noise * ((h >> 11) as f64 / (1u64 << 53) as f64);
        }
        e
    }
}
