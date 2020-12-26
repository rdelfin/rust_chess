use crate::{
    components::{ChessPieceInfo, PiecePlacement, Position, PotentialMovement},
    entities::display as display_entities,
    resources::{Displayed, PiecePositioning, Play, Selected, SpriteCache},
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
        Write<'s, Play>,
        Read<'s, PiecePositioning>,
        ReadStorage<'s, ChessPieceInfo>,
        ReadStorage<'s, PotentialMovement>,
        WriteStorage<'s, PiecePlacement>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, SpriteCache>,
    );

    fn run(
        &mut self,
        (
            mut selected,
            mut displayed,
            mut play,
            piece_positioning,
            piece_infos,
            potential_movements,
            mut piece_placements,
            entities,
            lazy_update,
            sprite_cache,
        ): Self::SystemData,
    ) {
        // There's two resources, selected and displayed. The former denotes a piece
        // that we just clicked on. The latter denotes a piece that we're currently
        // showing the valid positions of.
        //
        // We only act when selected is Some(_), and depending on whether we have a
        // piece already displayed we choose whether to display potential moves, move
        // the piece, or do nothing

        if let Some(s) = selected.0 {
            let piece_info = match piece_positioning.map.get(&s) {
                Some(position) => piece_infos.join().get(*position, &entities),
                None => None,
            };

            // Creates a map from piece positions to colour of the piece
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

            displayed.0 = match displayed.0 {
                // If we're already displaying a piece, move the piece (if a valid
                // movement is selected) and clear out the displayed potential moves.
                Some(d) => {
                    // We need to compute the current position first
                    let displayed_position = piece_placements.get(d).unwrap().0;

                    // This computes the valid movements for the *displayed* piece
                    let valid_moves = match piece_infos.join().get(d, &entities) {
                        Some(piece_info) => valid_piece_movements(
                            displayed_position,
                            piece_info.piece,
                            &colour_mappings,
                        ),
                        None => HashSet::new(),
                    };

                    // If we selected a valid move for the piece being displayed, move
                    // it and switch colours
                    if valid_moves.contains(&s) {
                        if let Some(placement) = piece_placements.get_mut(d) {
                            placement.0 = s;

                            // Finally, remove pieces you "ate"
                            if let Some(piece) = piece_positioning.map.get(&s) {
                                entities.delete(*piece).unwrap();
                            }
                        }

                        play.turn = !play.turn;
                    }

                    // Clear out the displayed red squares
                    for (_, entity) in (&potential_movements, &entities).join() {
                        entities.delete(entity).unwrap();
                    }
                    None
                }

                // If we're not displaying anything, try to display the valid moves for
                // the current piece
                None => {
                    // Only allow movement from current colour
                    if let Some(selected_colour) = colour_mappings.get(&s) {
                        if *selected_colour == play.turn {
                            // The relevant valid moves
                            let valid_moves = match piece_info {
                                Some(piece_info) => {
                                    valid_piece_movements(s, piece_info.piece, &colour_mappings)
                                }
                                None => HashSet::new(),
                            };

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
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            };
        }

        selected.0 = None
    }
}
