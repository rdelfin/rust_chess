use crate::{
    components::{ChessPieceInfo, PiecePlacement, Position},
    resources::PiecePositioning,
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{ReadStorage, System, Write, WriteStorage},
};
use nalgebra::Vector2;

#[derive(SystemDesc)]
pub struct PiecePlacementSystem;

impl<'s> System<'s> for PiecePlacementSystem {
    type SystemData = (
        ReadStorage<'s, PiecePlacement>,
        ReadStorage<'s, ChessPieceInfo>,
        WriteStorage<'s, Position>,
        Write<'s, PiecePositioning>,
    );

    fn run(
        &mut self,
        (piece_placements, piece_infos, mut positions, mut piece_positioning): Self::SystemData,
    ) {
        piece_positioning.map.clear();
        for (piece_placement, piece_info, position) in
            (&piece_placements, &piece_infos, &mut positions).join()
        {
            position.0 = Vector2::new(-224., 224.)
                + 64. * Vector2::new(piece_placement.0.x as f32, -piece_placement.0.y as f32);
            piece_positioning
                .map
                .insert(piece_placement.0, (piece_info.color, piece_info.piece));
        }
    }
}
