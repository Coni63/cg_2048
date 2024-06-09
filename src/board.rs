use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};

pub struct Board {
    pub board: [u8; 16],
    pub score: u32,
    pub seed: u64,
}

impl Board {
    pub fn new(seed: u64) -> Board {
        let mut board = Board {
            board: [0; 16],
            score: 0,
            seed,
        };

        board.add_random_tile();
        board.add_random_tile();

        board
    }

    pub fn play(&mut self, action: u8) -> bool {
        let moved = self.apply_action(action);

        self.add_random_tile();

        moved
    }

    #[allow(dead_code)]
    pub fn is_game_over(&self) -> bool {
        let mut moved: bool;
        let mut copy: Board;
        for dir in 1..5 {
            copy = self.clone();
            moved = copy.apply_action(dir);
            if moved {
                return false;
            }
        }
        true
    }

    fn apply_action(&mut self, action: u8) -> bool {
        match action {
            1 => self.move_up(),
            2 => self.move_left(),
            3 => self.move_down(),
            4 => self.move_right(),
            _ => false,
        }
    }

    fn add_random_tile(&mut self) {
        let mut empty_tiles = Vec::new();
        for col in 0..4 {
            for row in 0..4 {
                let idx = row * 4 + col;
                if self.board[idx] == 0 {
                    empty_tiles.push(idx);
                }
            }
        }

        if empty_tiles.is_empty() {
            return;
        }

        let idx = self.seed.wrapping_rem(empty_tiles.len() as u64);
        let spawn_index = empty_tiles[idx as usize];
        let value = if self.seed & 0x10 == 0 { 1 } else { 2 };
        self.board[spawn_index] = value;

        self.seed = self.seed.wrapping_mul(self.seed).wrapping_rem(50515093);
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;
        let mut ptr1: usize;
        let mut ptr2: usize;
        for i in 0..4 {
            // for each column
            ptr1 = i;
            ptr2 = i + 4;

            while ptr2 < 16 {
                if self.board[ptr2] != 0 {
                    if self.board[ptr1] == 0 {
                        // move to empty tile
                        self.board[ptr1] = self.board[ptr2];
                        self.board[ptr2] = 0;
                        ptr2 += 4;
                        moved = true;
                    } else if self.board[ptr1] == self.board[ptr2] {
                        // 2 same values -> merge
                        self.board[ptr1] += 1;
                        self.board[ptr2] = 0;
                        self.score += 1 << self.board[ptr1] as u32;
                        ptr1 += 4;
                        ptr2 = ptr1 + 4;
                        moved = true;
                    } else {
                        // 2 different values
                        ptr1 += 4;
                        ptr2 = ptr1 + 4;
                    }
                } else {
                    ptr2 += 4;
                }
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        let mut ptr1: usize;
        let mut ptr2: usize;

        for i in 0..4 {
            // for each row
            ptr1 = 4 * i;
            ptr2 = 4 * i + 1;

            while ptr2 < 4 * (i + 1) {
                if self.board[ptr2] != 0 {
                    if self.board[ptr1] == 0 {
                        // move to empty tile
                        self.board[ptr1] = self.board[ptr2];
                        self.board[ptr2] = 0;
                        ptr2 += 1;
                        moved = true;
                    } else if self.board[ptr1] == self.board[ptr2] {
                        // 2 same values -> merge
                        self.board[ptr1] += 1;
                        self.board[ptr2] = 0;
                        self.score += 1 << self.board[ptr1] as u32;
                        ptr1 += 1;
                        ptr2 = ptr1 + 1;
                        moved = true;
                    } else {
                        // 2 different values
                        ptr1 += 1;
                        ptr2 = ptr1 + 1;
                    }
                } else {
                    ptr2 += 1;
                }
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;
        let mut ptr1: i32;
        let mut ptr2: i32;
        for i in 0..4 {
            // for each column
            ptr1 = 12 + i; // bottom pointer
            ptr2 = 8 + i; // top pointer

            while ptr2 >= 0 {
                if self.board[ptr2 as usize] != 0 {
                    if self.board[ptr1 as usize] == 0 {
                        // move to empty tile
                        self.board[ptr1 as usize] = self.board[ptr2 as usize];
                        self.board[ptr2 as usize] = 0;
                        ptr2 -= 4;
                        moved = true;
                    } else if self.board[ptr1 as usize] == self.board[ptr2 as usize] {
                        // 2 same values -> merge
                        self.board[ptr1 as usize] += 1;
                        self.board[ptr2 as usize] = 0;
                        self.score += 1 << self.board[ptr1 as usize] as u32;
                        ptr1 -= 4;
                        ptr2 = ptr1 - 4;
                        moved = true;
                    } else {
                        // 2 different values
                        ptr1 -= 4;
                        ptr2 = ptr1 - 4;
                    }
                } else {
                    ptr2 -= 4;
                }
            }
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        let mut moved = false;
        let mut ptr1: i32;
        let mut ptr2: i32;
        for i in 0..4 {
            // for each row
            ptr1 = 4 * (i + 1) - 1; // right pointer
            ptr2 = 4 * (i + 1) - 2; // left pointer

            while ptr2 >= 4 * i {
                if self.board[ptr2 as usize] != 0 {
                    if self.board[ptr1 as usize] == 0 {
                        // move to empty tile
                        self.board[ptr1 as usize] = self.board[ptr2 as usize];
                        self.board[ptr2 as usize] = 0;
                        ptr2 -= 1;
                        moved = true;
                    } else if self.board[ptr1 as usize] == self.board[ptr2 as usize] {
                        // 2 same values -> merge
                        self.board[ptr1 as usize] += 1;
                        self.board[ptr2 as usize] = 0;
                        self.score += 1 << self.board[ptr1 as usize] as u32;
                        ptr1 -= 1;
                        ptr2 = ptr1 - 1;
                        moved = true;
                    } else {
                        // 2 different values
                        ptr1 -= 1;
                        ptr2 = ptr1 - 1;
                    }
                } else {
                    ptr2 -= 1;
                }
            }
        }
        moved
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Score: {}", self.score)?;
        writeln!(f, "Seed: {}", self.seed)?;
        for y in 0..4 {
            for x in 0..4 {
                let val = self.board[(y * 4 + x) as usize];
                write!(f, "{:2} ", if val > 0 { 1 << val } else { 0 })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Score: {}", self.score)?;
        writeln!(f, "Seed: {}", self.seed)?;
        for y in 0..4 {
            for x in 0..4 {
                let val = self.board[(y * 4 + x) as usize];
                write!(f, "{:2} ", if val > 0 { 1 << val } else { 0 })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

// Automatically implementing Eq since we implemented PartialEq
impl Eq for Board {}

impl Hash for Board {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.board.hash(state)
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board {
            board: self.board,
            score: self.score,
            seed: self.seed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let mut game = Board::new(0);
        game.board = [0, 0, 4, 4, 0, 2, 2, 4, 0, 0, 2, 2, 0, 0, 0, 0];
        game.score = 123;
        game.seed = 456;

        let mut copy = game.clone();
        copy.play(4);

        assert!(game.board == [0, 0, 4, 4, 0, 2, 2, 4, 0, 0, 2, 2, 0, 0, 0, 0]);
        assert!(copy.board == [1, 0, 0, 5, 0, 0, 3, 4, 0, 0, 0, 3, 0, 0, 0, 0]);

        assert!(game.score == 123);
        assert!(copy.score == 123 + 48);

        assert!(game.seed == 456);
        assert!(copy.seed == 456 * 456);
    }

    #[test]
    fn test_move_up() {
        let board_start = [0, 4, 0, 2, 0, 0, 4, 2, 2, 4, 2, 2, 2, 2, 8, 2];
        let board_end = [3, 5, 4, 3, 0, 2, 2, 3, 0, 0, 8, 0, 0, 0, 0, 0];

        let mut game = Board::new(0);
        game.board = board_start;

        let moved = game.move_up();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(moved);
    }

    #[test]
    fn test_no_move_up() {
        let board_end = [4, 8, 4, 4, 0, 2, 2, 0, 0, 0, 8, 0, 0, 0, 0, 0];

        let mut game = Board::new(0);
        game.board = board_end;

        let moved = game.move_up();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(!moved);
    }

    #[test]
    fn test_move_left() {
        let board_start = [2, 2, 2, 2, 2, 0, 4, 2, 0, 4, 8, 2, 0, 4, 0, 2];
        let board_end = [3, 3, 0, 0, 2, 4, 2, 0, 4, 8, 2, 0, 4, 2, 0, 0];

        let mut game = Board::new(0);
        game.board = board_start;

        let moved = game.move_left();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(moved);
    }

    #[test]
    fn test_no_move_left() {
        let board_end = [4, 0, 0, 0, 2, 4, 2, 0, 4, 8, 2, 0, 4, 2, 0, 0];

        let mut game = Board::new(0);
        game.board = board_end;

        let moved = game.move_left();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(!moved);
    }

    #[test]
    fn test_move_down() {
        let board_start = [2, 2, 4, 2, 2, 0, 4, 2, 0, 4, 5, 2, 0, 4, 0, 2];
        let board_end = [0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 5, 3, 3, 5, 5, 3];

        let mut game = Board::new(0);
        game.board = board_start;

        let moved = game.move_down();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(moved);
    }

    #[test]
    fn test_no_move_down() {
        let board_end = [0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 4, 8, 8, 4];

        let mut game = Board::new(0);
        game.board = board_end;

        let moved = game.move_down();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(!moved);
    }

    #[test]
    fn test_move_right() {
        let board_start = [2, 2, 2, 2, 2, 0, 4, 2, 2, 4, 8, 0, 2, 2, 3, 0];
        let board_end = [0, 0, 3, 3, 0, 2, 4, 2, 0, 2, 4, 8, 0, 0, 3, 3];

        let mut game = Board::new(0);
        game.board = board_start;

        let moved = game.move_right();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(moved);
    }

    #[test]
    fn test_no_move_right() {
        let board_end = [0, 0, 0, 4, 0, 0, 8, 2, 2, 4, 2, 4, 0, 0, 4, 8];

        let mut game = Board::new(0);
        game.board = board_end;

        let moved = game.move_right();

        // println!("{:?}", game.board);

        assert!(game.board == board_end);
        assert!(!moved);
    }

    #[test]
    fn test_hash_same_board() {
        let game1 = Board::new(0);
        let game2 = Board::new(0);

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        game1.hash(&mut hasher);
        let h1 = hasher.finish();

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        game2.hash(&mut hasher);
        let h2 = hasher.finish();

        assert!(h1 == h2);
    }

    #[test]
    fn test_hash_different_board() {
        let game1 = Board::new(0);
        let game2 = Board::new(42);

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        game1.hash(&mut hasher);
        let h1 = hasher.finish();

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        game2.hash(&mut hasher);
        let h2 = hasher.finish();

        assert!(h1 != h2);
    }

    #[test]
    fn test_board() {
        let mut game = Board::new(290797);

        assert!(game.seed == 13339144);
        assert!(game.board == [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 0, 0]);

        for letter in "DRDURRLRDUURUDRRDDRDLULRDULUDRULLUDDLRDLDLL".chars() {
            let action = match letter {
                'U' => 1,
                'L' => 2,
                'D' => 3,
                'R' => 4,
                _ => 0,
            };

            let moved = game.play(action);
            assert!(moved);
        }

        assert!(game.board == [3, 1, 3, 2, 1, 4, 5, 3, 4, 1, 3, 4, 2, 4, 2, 1]);
        assert!(game.score == 280);
        assert!(game.is_game_over());
    }
}
