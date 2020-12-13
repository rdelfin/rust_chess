use crate::utils::{ChessColor, ChessPiece};
use nalgebra::Vector2;
use std::collections::{HashMap, HashSet};

pub fn pawn_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let mut movements = HashSet::new();

    let start_row = match color {
        ChessColor::White => 6,
        ChessColor::Black => 1,
    };

    let candidates = match color {
        ChessColor::White => (
            vec![pos + Vector2::new(0, -1)],
            vec![pos + Vector2::new(0, -2)],
            vec![pos + Vector2::new(1, -1), pos + Vector2::new(-1, -1)],
        ),
        ChessColor::Black => (
            vec![pos + Vector2::new(0, 1)],
            vec![pos + Vector2::new(0, 2)],
            vec![pos + Vector2::new(1, 1), pos + Vector2::new(-1, 1)],
        ),
    };

    // candidates.0 contains regular forwards movement
    movements.extend(
        candidates
            .0
            .iter()
            .cloned()
            .filter(|p| !all_pieces.contains_key(p)),
    );

    // candidates.1 contains initial double-jump
    movements.extend(
        candidates
            .1
            .iter()
            .cloned()
            .filter(|p| pos.y == start_row && empty_between(pos, *p, all_pieces, true, false)),
    );

    // candidates.2 contains om nom nom
    movements.extend(
        candidates
            .2
            .iter()
            .cloned()
            .filter(|p| match all_pieces.get(p) {
                None => false,
                Some(other_c) => color != *other_c,
            }),
    );

    movements
}

pub fn rook_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let mut movements = HashSet::new();

    movements.extend(
        (0..8)
            .map(|r| Vector2::new(r, pos.y))
            .filter(|p| *p != pos && empty_between(pos, *p, all_pieces, true, false)),
    );

    movements.extend(
        (0..8)
            .map(|r| Vector2::new(pos.x, r))
            .filter(|p| *p != pos && empty_between(pos, *p, all_pieces, true, false)),
    );

    movements
}

pub fn horse_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    [
        (2, 1),
        (1, 2),
        (2, -1),
        (1, -2),
        (-2, -1),
        (-1, -2),
        (-2, 1),
        (-1, 2),
    ]
    .iter()
    .map(|p| pos + Vector2::new(p.0, p.1))
    .filter(|p| in_board(p) && (!all_pieces.contains_key(p) || all_pieces[p] != color))
    .collect()
}

pub fn bishop_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    let mut movements = HashSet::new();

    movements.extend(
        (-8..8).map(|r| pos + Vector2::new(r, r)).filter(|p| {
            *p != pos && in_board(p) && empty_between(pos, *p, all_pieces, true, false)
        }),
    );
    movements.extend(
        (-8..8).map(|r| pos + Vector2::new(r, -r)).filter(|p| {
            *p != pos && in_board(p) && empty_between(pos, *p, all_pieces, true, false)
        }),
    );

    movements
}

pub fn queen_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    rook_movements(pos, color, all_pieces)
        .union(&bishop_movements(pos, color, all_pieces))
        .cloned()
        .collect()
}

pub fn king_movements(
    pos: Vector2<i32>,
    color: ChessColor,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
) -> HashSet<Vector2<i32>> {
    iproduct!((-1..2), (-1..2))
        .map(|(x, y)| pos + Vector2::new(x, y))
        .filter(|p| {
            *p != pos && in_board(p) && (!all_pieces.contains_key(p) || all_pieces[p] != color)
        })
        .collect()
}

// Checks if every tile between start and end is populated
fn empty_between(
    start: Vector2<i32>,
    end: Vector2<i32>,
    all_pieces: &HashMap<Vector2<i32>, ChessColor>,
    exclude_start: bool,
    exclude_end: bool,
) -> bool {
    let diff = end - start;
    let start_iter = if exclude_start { 1 } else { 0 };
    let end_iter_offset = if exclude_end { 0 } else { 1 };

    // Generates the set of tiles we'll need to check (vertical, horizontal and diagnonal)
    let tiles = if diff.x == 0 {
        (start_iter..(diff.y.abs() + end_iter_offset))
            .map(|i| start + i * Vector2::new(0, diff.y.signum()))
            .collect()
    } else if diff.y == 0 {
        (start_iter..(diff.x.abs() + end_iter_offset))
            .map(|i| start + i * Vector2::new(diff.x.signum(), 0))
            .collect()
    } else if diff.x.abs() == diff.y.abs() {
        (start_iter..(diff.x.abs() + end_iter_offset))
            .map(|i| start + i * Vector2::new(diff.x.signum(), diff.y.signum()))
            .collect()
    } else {
        vec![]
    };

    tiles.into_iter().all(|t| !all_pieces.contains_key(&t))
}

fn in_board(pos: &Vector2<i32>) -> bool {
    pos.x >= 0 && pos.x < 8 && pos.y >= 0 && pos.y < 8
}
