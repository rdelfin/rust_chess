use crate::utils::{ChessColor, ChessPiece};
use amethyst::ecs::{Component, DenseVecStorage};
use nalgebra::Vector2;

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct PiecePlacement(pub Vector2<i32>);

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct ChessPieceInfo {
    pub color: ChessColor,
    pub piece: ChessPiece,
}

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct PotentialMovement {
    pub pos: Vector2<i32>,
}
