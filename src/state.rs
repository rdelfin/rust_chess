use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};

use log::info;

/// A dummy game state that shows 3 sprites.
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        self.init_camera(world, &dimensions);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        // Keep going
        Trans::None
    }
}

impl GameState {
    fn init_camera(&self, world: &mut World, dimensions: &ScreenDimensions) {
        // Center the camera in the middle of the screen, and let it cover
        // the entire screen
        let mut transform = Transform::default();
        //transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);
        transform.set_translation_xyz(0., 0., 1.);
        //transform.set_scale(Vector3::new(0.2, 0.2, 1.));

        world
            .create_entity()
            .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
            .with(transform)
            .build();
    }
}
