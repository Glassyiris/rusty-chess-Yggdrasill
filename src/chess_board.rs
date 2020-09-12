use crate::chess::Camp::{GRAY, GREEN};
use crate::chess::*;
use crate::state::{Root, State};

#[derive(Debug)]
pub struct ChessBoard {
    pub board: Vec<Vec<State>>,
}

impl ChessBoard {
    pub fn new() -> Self {
        let mut board = vec![vec![State::default(); 8]; 8];
        for i in 2..6 {
            board[7][i].is_root = Root::GREEN;
            board[0][i].is_root = Root::GRAY;
        }
        for i in 3..5 {
            let chess_gray = Chess::new(Position { x: i, y: 0 }, ChessKind::LEAF, Camp::GRAY);
            let chess_green = Chess::new(Position { x: i, y: 7 }, ChessKind::LEAF, Camp::GREEN);
            board[i][1] = State::set_chess(Some(chess_gray));
            board[i][6] = State::set_chess(Some(chess_green));
        }
        ChessBoard { board }
    }

    fn is_gray_root(&self, position: Position) -> bool {
        self.board[position.y][position.x].is_root == Root::GRAY
    }

    fn is_green_root(&self, position: Position) -> bool {
        self.board[position.y][position.x].is_root == Root::GREEN
    }
}
