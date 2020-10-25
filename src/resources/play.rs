use crate::utils::ChessColor;
use amethyst::ecs::Entity;
use nalgebra::Vector2;
use std::collections::HashMap;

#[derive(Default)]
pub struct Play {
    pub turn: ChessColor,
    pub selected_piece: Option<Entity>,
}

#[derive(Default)]
pub struct PiecePositioning {
    pub map: HashMap<Vector2<i32>, Entity>,
}
