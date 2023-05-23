use bevy::prelude::*;

struct Joint {
    id: u32,
    connected: bool,
    connected_entity: Option<Entity>,
}

struct Connection {
    distance: f32,
    l1: u32,
    l2: u32,
}

#[derive(Component)]
pub struct Track {
    joints: Vec<Joint>,
    connections: Vec<Connection>,
}
