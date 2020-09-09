use amethyst::ecs::{Component, DenseVecStorage};
use std::time::Duration;

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct AnimatedSprite {
    pub frames: Vec<usize>,
    pub curr_frame_idx: usize,
    pub frame_len: Duration,
    pub frame_start: Duration,
    pub should_loop: bool,
}
