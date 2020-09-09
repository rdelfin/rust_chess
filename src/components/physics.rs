use amethyst::ecs::{Component, DenseVecStorage, VecStorage};
use nalgebra::Vector2;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Vector2<f32>);

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct Velocity(pub Vector2<f32>);
