use crate::components::{Position, PotentialMovement};
use amethyst::{
    derive::SystemDesc,
    ecs::{prelude::*, Join, System},
};
use nalgebra::Vector2;

#[derive(SystemDesc)]
pub struct DisplayMovesSystem;

impl<'s> System<'s> for DisplayMovesSystem {
    type SystemData = (
        WriteStorage<'s, Position>,
        ReadStorage<'s, PotentialMovement>,
    );

    fn run(&mut self, (mut positions, potential_moves): Self::SystemData) {
        for (position, potential_moves) in (&mut positions, &potential_moves).join() {
            let chess_pos = potential_moves.pos;
            position.0 = Vector2::new(-224., 224.)
                + 64. * Vector2::new(chess_pos.x as f32, -chess_pos.y as f32);
        }
    }
}
