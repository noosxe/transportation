use bevy::prelude::*;

#[derive(Component)]
pub struct Locomotive {
    target: Entity,
    distance: f32,
    speed: f32,
}
