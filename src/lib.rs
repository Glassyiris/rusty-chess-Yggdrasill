mod chess;
mod chess_board;
mod message;
mod state;
mod game;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::Camp::GRAY;
    use crate::chess::ChessKind;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    fn board() {
        let board = chess_board::ChessBoard::new();
        println!("{:?}", board.board[0][3].is_root);
    }
}
