use crate::{
    components::Position,
    entities::pieces,
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

pub fn new_board(world: &mut World) -> Result<Entity> {
    let piece_handle = {
        let sprite_cache = world
            .try_fetch::<SpriteCache>()
            .ok_or_else(|| anyhow!("Failed to fetch the sprite cache while creating player."))?;
        sprite_cache.fetch(SpriteKey::Board)?.clone()
    };

    Ok(world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: piece_handle,
            sprite_number: 0,
        })
        .with(Transparent)
        .with(Transform::default())
        .with(Position(Vector2::new(-0., -0.)))
        .build())
}

pub fn new_full_board(world: &mut World) -> Result<()> {
    new_board(world)?;

    // Setup all pieces of the board
    create_main_row(world, ChessColor::Black, 0)?;
    create_pawn_row(world, ChessColor::Black, 1)?;
    create_pawn_row(world, ChessColor::White, 6)?;
    create_main_row(world, ChessColor::White, 7)?;
    Ok(())
}

fn create_main_row(world: &mut World, color: ChessColor, row: i32) -> Result<()> {
    pieces::new_piece(world, ChessPiece::Rook, color, Vector2::new(0, row))?;
    pieces::new_piece(world, ChessPiece::Horse, color, Vector2::new(1, row))?;
    pieces::new_piece(world, ChessPiece::Bishop, color, Vector2::new(2, row))?;
    pieces::new_piece(world, ChessPiece::Queen, color, Vector2::new(3, row))?;
    pieces::new_piece(world, ChessPiece::King, color, Vector2::new(4, row))?;
    pieces::new_piece(world, ChessPiece::Bishop, color, Vector2::new(5, row))?;
    pieces::new_piece(world, ChessPiece::Horse, color, Vector2::new(6, row))?;
    pieces::new_piece(world, ChessPiece::Rook, color, Vector2::new(7, row))?;
    Ok(())
}

fn create_pawn_row(world: &mut World, color: ChessColor, row: i32) -> Result<()> {
    for c in 0..8 {
        pieces::new_piece(world, ChessPiece::Pawn, color, Vector2::new(c, row))?;
    }
    Ok(())
}
