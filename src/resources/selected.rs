use amethyst::ecs::Entity;
use nalgebra::Vector2;

#[derive(Default)]
pub struct Selected(pub Option<Vector2<i32>>);

#[derive(Default)]
pub struct Displayed(pub Option<Entity>);
