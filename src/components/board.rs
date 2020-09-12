use amethyst::ecs::{Component, DenseVecStorage};
use nalgebra::Vector2;

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct PiecePlacement(pub Vector2<i32>);
