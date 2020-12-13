use crate::{
    components::{Position, PotentialMovement},
    resources::{SpriteCache, SpriteKey},
};
use amethyst::{
    core::Transform,
    ecs::{Entities, Entity, LazyUpdate},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use anyhow::Result;
use nalgebra::Vector2;

pub fn fill_potential_move(
    entities: &Entities<'_>,
    updater: &LazyUpdate,
    sprite_cache: &SpriteCache,
    pos: Vector2<i32>,
) -> Result<Entity> {
    let piece_handle = sprite_cache.fetch(SpriteKey::OptionSquare)?.clone();

    Ok(updater
        .create_entity(entities)
        .with(Transform::default())
        .with(Position(Vector2::new(0., 0.)))
        .with(Transparent)
        .with(SpriteRender {
            sprite_sheet: piece_handle,
            sprite_number: 0,
        })
        .with(PotentialMovement { pos })
        .build())
}
