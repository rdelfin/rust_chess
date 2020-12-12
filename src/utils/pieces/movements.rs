use crate::utils::{ChessColor, ChessPiece};
use nalgebra::Vector2;
use std::collections::{HashMap, HashSet};

pub fn pawn_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let movements = HashSet::new();
    movements
}

pub fn rook_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let movements = HashSet::new();
    movements
}

pub fn horse_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let movements = HashSet::new();
    movements
}

pub fn bishop_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let movements = HashSet::new();
    movements
}

pub fn queen_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let movements = HashSet::new();
    movements
}

pub fn king_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let movements = HashSet::new();
    movements
}
