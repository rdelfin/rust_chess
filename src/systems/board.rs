use crate::{
    components::{ChessPieceInfo, PiecePlacement, Position, PotentialMovement},
    entities::display as display_entities,
    resources::{Displayed, PiecePositioning, Selected, SpriteCache},
    utils::valid_piece_movements,
};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{Entities, ReadStorage, System, Write, WriteStorage},
};
use nalgebra::Vector2;
use std::collections::HashSet;

#[derive(SystemDesc)]
pub struct PiecePlacementSystem;

impl<'s> System<'s> for PiecePlacementSystem {
    type SystemData = (
        ReadStorage<'s, PiecePlacement>,
        ReadStorage<'s, ChessPieceInfo>,
        WriteStorage<'s, Position>,
        Write<'s, PiecePositioning>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (piece_placements, piece_infos, mut positions, mut piece_positioning, entities): Self::SystemData,
    ) {
        piece_positioning.map.clear();
        for (piece_placement, _, position, entity) in
            (&piece_placements, &piece_infos, &mut positions, &entities).join()
        {
            position.0 = Vector2::new(-224., 224.)
                + 64. * Vector2::new(piece_placement.0.x as f32, -piece_placement.0.y as f32);
            piece_positioning.map.insert(piece_placement.0, entity);
        }
    }
}

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'s, Selected>,
        Write<'s, Displayed>,
        Read<'s, PiecePositioning>,
        ReadStorage<'s, ChessPieceInfo>,
        ReadStorage<'s, PotentialMovement>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, SpriteCache>,
    );

    fn run(
        &mut self,
        (
            mut selected,
            mut displayed,
            piece_positioning,
            piece_infos,
            potential_movements,
            entities,
            lazy_update,
            sprite_cache,
        ): Self::SystemData,
    ) {
        if let Some(s) = selected.0 {
            let piece_info = match piece_positioning.map.get(&s) {
                Some(position) => piece_infos.join().get(*position, &entities),
                None => None,
            };

            let mut piece_infos_join = piece_infos.join();
            let colour_mappings = piece_positioning
                .map
                .iter()
                .map(|(position, entity)| {
                    (
                        *position,
                        piece_infos_join.get(*entity, &entities).unwrap().color,
                    )
                })
                .collect();
            let valid_moves = match piece_info {
                Some(piece_info) => valid_piece_movements(s, piece_info.piece, &colour_mappings),
                None => HashSet::new(),
            };

            displayed.0 = match displayed.0 {
                Some(_) => {
                    for (_, entity) in (&potential_movements, &entities).join() {
                        entities.delete(entity).unwrap();
                    }
                    None
                }
                None => {
                    // If we're not already showing a move, add the selected piece's potential
                    // moves
                    for movement in valid_moves {
                        display_entities::fill_potential_move(
                            &entities,
                            &lazy_update,
                            &sprite_cache,
                            movement,
                        )
                        .unwrap();
                    }
                    piece_positioning.map.get(&s).cloned()
                }
            };
        }

        selected.0 = None
    }
}
