use crate::{
    input::{ActionBinding, ControlBindingTypes},
    resources::{PiecePositioning, Selected},
};
use amethyst::{
    core::{
        geometry::Plane,
        //math::{Point2, Vector2, Vector3},
        transform::Transform,
    },
    derive::SystemDesc,
    ecs::{prelude::*, Entities, Read, ReadExpect, System, Write},
    input::InputHandler,
    renderer::camera::{ActiveCamera, Camera},
    window::ScreenDimensions,
};
use nalgebra::{Point2, Vector2};

#[derive(SystemDesc, Default)]
pub struct UserInputSystem {
    select_prev_pressed: bool,
}

impl<'s> System<'s> for UserInputSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputHandler<ControlBindingTypes>>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        Write<'s, Selected>,
    );

    fn run(
        &mut self,
        (
            entities,
            input,
            transforms,
            cameras,
            active_camera,
            screen_dimensions,
            mut selected,
        ): Self::SystemData,
    ) {
        let mouse = match input.mouse_position() {
            Some((x, y)) => Point2::new(x, y),
            None => Point2::new(0.0, 0.0),
        };
        let selected_pressed = input
            .action_is_down(&ActionBinding::Select)
            .unwrap_or(false);

        let mut camera_join = (&cameras, &transforms).join();
        if let Some((camera, camera_transform)) = active_camera
            .entity
            .and_then(|a| camera_join.get(a, &entities))
            .or_else(|| camera_join.next())
        {
            // Project a ray from the camera to the 0z axis
            let ray = camera.screen_ray(
                mouse,
                Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                camera_transform,
            );
            let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
            let mouse_pos = ray.at_distance(distance);
            let chess_pos = Vector2::new(
                ((mouse_pos.x + 192.) / 64.).ceil() as i32,
                ((-mouse_pos.y + 192.) / 64.).ceil() as i32,
            );

            if !selected_pressed && self.select_prev_pressed && inside_board(chess_pos) {
                selected.0 = Some(chess_pos);
            }
        }

        self.select_prev_pressed = selected_pressed;
    }
}

fn inside_board(p: Vector2<i32>) -> bool {
    p.x >= 0 && p.y >= 0 && p.x < 8 && p.y < 8
}
