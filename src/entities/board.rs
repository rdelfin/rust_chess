use crate::{
    components::Position,
    resources::{SpriteCache, SpriteKey},
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
