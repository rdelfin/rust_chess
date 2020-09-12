use crate::components::{PiecePlacement, Position};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{ReadStorage, System, WriteStorage},
};
use nalgebra::Vector2;

#[derive(SystemDesc)]
pub struct PiecePlacementSystem;

impl<'s> System<'s> for PiecePlacementSystem {
    type SystemData = (ReadStorage<'s, PiecePlacement>, WriteStorage<'s, Position>);

    fn run(&mut self, (piece_placements, mut positions): Self::SystemData) {
        for (piece_placement, position) in (&piece_placements, &mut positions).join() {
            position.0 = Vector2::new(-224., 224.)
                + 64. * Vector2::new(piece_placement.0.x as f32, -piece_placement.0.y as f32);
        }
    }
}
