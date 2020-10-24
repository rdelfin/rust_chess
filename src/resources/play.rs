use crate::utils::{ChessColor, ChessPiece};
use nalgebra::Vector2;
use std::collections::HashMap;

#[derive(Default)]
pub struct PiecePositioning {
    pub map: HashMap<Vector2<i32>, (ChessColor, ChessPiece)>,
}
