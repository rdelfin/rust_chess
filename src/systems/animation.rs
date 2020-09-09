use crate::components::AnimatedSprite;
use amethyst::{
    core::Time,
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{System, WriteStorage},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct SpriteAnimationSystem;

impl<'s> System<'s> for SpriteAnimationSystem {
    type SystemData = (
        WriteStorage<'s, AnimatedSprite>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut animations, mut sprite_renders, time): Self::SystemData) {
        let curr_time = time.absolute_time();
        for (animation, sprite_render) in (&mut animations, &mut sprite_renders).join() {
            let mut frames_passed = 0;
            while curr_time - animation.frame_start >= animation.frame_len {
                frames_passed += 1;
                animation.frame_start += animation.frame_len;
            }

            animation.curr_frame_idx = if animation.should_loop {
                (animation.curr_frame_idx + frames_passed) % animation.frames.len()
            } else {
                (animation.curr_frame_idx + frames_passed).min(animation.frames.len() - 1)
            };

            sprite_render.sprite_number = animation.frames[animation.curr_frame_idx];
        }
    }
}
