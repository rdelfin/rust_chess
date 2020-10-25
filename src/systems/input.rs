use crate::input::{AxisBinding, ControlBindingTypes};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    ecs::{Read, System},
    input::InputHandler,
};
use nalgebra::Vector2;

#[derive(SystemDesc)]
pub struct UserInputSystem;

impl<'s> System<'s> for UserInputSystem {
    type SystemData = Read<'s, InputHandler<ControlBindingTypes>>;

    fn run(&mut self, input: Self::SystemData) {
        let mouse = match input.mouse_position() {
            Some((x, y)) => Vector2::new(x, y),
            None => Vector2::new(0.0, 0.0),
        };
        println!("Mouse: {}", mouse);
    }
}
