#[derive(Clone, Copy, Debug)]
pub enum ChessPiece {
    Pawn,
    Rook,
    Horse,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug)]
pub enum ChessColor {
    White,
    Black,
}

impl Default for ChessColor {
    fn default() -> Self {
        ChessColor::White
    }
}
