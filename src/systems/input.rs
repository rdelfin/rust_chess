use crate::{input::ControlBindingTypes, resources::PiecePositioning};
use amethyst::{
    core::{
        geometry::Plane,
        //math::{Point2, Vector2, Vector3},
        transform::Transform,
    },
    derive::SystemDesc,
    ecs::{prelude::*, Entities, Read, ReadExpect, System},
    input::InputHandler,
    renderer::camera::{ActiveCamera, Camera},
    window::ScreenDimensions,
};
use nalgebra::{Point2, Vector2};

#[derive(SystemDesc)]
pub struct UserInputSystem;

impl<'s> System<'s> for UserInputSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputHandler<ControlBindingTypes>>,
        Read<'s, PiecePositioning>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (entities, input, piece_positioning, transforms, cameras, active_camera, screen_dimensions): Self::SystemData,
    ) {
        let mouse = match input.mouse_position() {
            Some((x, y)) => Point2::new(x, y),
            None => Point2::new(0.0, 0.0),
        };

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
            let mouse_world_position = ray.at_distance(distance);
            println!("Mouse: {}", mouse_world_position);
        }
    }
}
