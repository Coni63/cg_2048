use rusqlite::{params, Connection, Result};

use crate::agent::node::Node;
use crate::board::Board;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(conn_str: String) -> Database {
        let conn = Connection::open(conn_str).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                id INTEGER PRIMARY KEY,
                board TEXT NOT NULL,
                action TEXT NOT NULL,
                count_action INTEGER NOT NULL DEFAULT 0,
                score INTEGER NOT NULL,
                initial_seed INTEGER NOT NULL,
                seed INTEGER NOT NULL,
                finished INTEGER NOT NULL DEFAULT 0,
                solver TEXT NOT NULL,
                evaluator TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
        Database { conn }
    }

    pub fn insert(&self, node: &Node, solver: &str, evaluator: &str, initial_seed: u64) {
        let board = node.board.export();
        println!("{}", board);
        let action = node.action.to_string();
        let count_action = node.action.len();
        let score = node.board.score;
        let seed = node.board.seed;
        let finished = node.board.is_game_over();
        self.conn
            .execute(
                "INSERT INTO data (board, action, count_action, score, initial_seed, seed, finished, solver, evaluator) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![board, action, count_action, score, initial_seed, seed, finished, solver, evaluator],
            )
            .unwrap();
    }

    pub fn select(&self, id: i32) -> Option<Node> {
        let mut stmt = self
            .conn
            .prepare("SELECT board, action, score, seed FROM data WHERE id = ?1")
            .unwrap();
        let mut rows = stmt.query(params![id]).unwrap();

        match rows.next() {
            Ok(Some(row)) => {
                let board = Board::from_string(row.get(0).unwrap());
                let action = row.get(1).unwrap();
                let score = row.get(2).unwrap();
                let seed = row.get(3).unwrap();

                let mut node = Node::new(&board);
                node.action = action;
                node.board.score = score;
                node.board.seed = seed;
                Some(node)
            }
            Ok(None) => None,
            Err(err) => {
                println!("Error: {}", err);
                None
            }
        }
    }
}
