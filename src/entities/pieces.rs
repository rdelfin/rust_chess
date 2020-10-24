use crate::{
    components::{ChessPieceInfo, PiecePlacement, Position, Velocity},
    resources::{SpriteCache, SpriteKey},
    utils::{ChessColor, ChessPiece},
};
use amethyst::{
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use anyhow::{anyhow, Result};
use nalgebra::Vector2;

pub fn new_piece(world: &mut World, piece: ChessPiece, color: ChessColor) -> Result<Entity> {
    let piece_handle = {
        let sprite_cache = world
            .try_fetch::<SpriteCache>()
            .ok_or_else(|| anyhow!("Failed to fetch the sprite cache while creating player."))?;
        sprite_cache.fetch(SpriteKey::Pieces)?.clone()
    };

    Ok(world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: piece_handle,
            sprite_number: chess_piece_to_frame(piece, color),
        })
        .with(Transparent)
        .with(Transform::default())
        .with(Position(Vector2::new(0., 0.)))
        .with(Velocity(Vector2::new(0., 0.)))
        .with(PiecePlacement(Vector2::new(2, 3)))
        .with(ChessPieceInfo { color, piece })
        .build())
}

fn chess_piece_to_frame(chess_piece: ChessPiece, color: ChessColor) -> usize {
    let piece_pos = match chess_piece {
        ChessPiece::Pawn => 0,
        ChessPiece::Rook => 1,
        ChessPiece::Horse => 2,
        ChessPiece::Bishop => 3,
        ChessPiece::Queen => 4,
        ChessPiece::King => 5,
    };
    let color_offset = match color {
        ChessColor::White => 0,
        ChessColor::Black => 6,
    };

    (piece_pos + color_offset) as usize
}
