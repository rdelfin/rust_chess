mod movements;

use nalgebra::Vector2;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChessPiece {
    Pawn,
    Rook,
    Horse,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChessColor {
    White,
    Black,
}

impl Default for ChessColor {
    fn default() -> Self {
        ChessColor::White
    }
}

pub fn valid_piece_movements(
    pos: Vector2<i32>,
    piece_type: ChessPiece,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    // Piece being passed in is expected to be in the all_pieces map
    let color = all_pieces[&pos];

    match piece_type {
        ChessPiece::Pawn => movements::pawn_movements(pos, color, all_pieces),
        ChessPiece::Rook => movements::rook_movements(pos, color, all_pieces),
        ChessPiece::Horse => movements::horse_movements(pos, color, all_pieces),
        ChessPiece::Bishop => movements::bishop_movements(pos, color, all_pieces),
        ChessPiece::Queen => movements::queen_movements(pos, color, all_pieces),
        ChessPiece::King => movements::king_movements(pos, color, all_pieces),
    }
}
