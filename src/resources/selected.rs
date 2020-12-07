use amethyst::ecs::Entity;

#[derive(Default)]
pub struct Selected(pub Option<Entity>);
