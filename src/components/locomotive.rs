use bevy::prelude::*;

#[derive(Component)]
pub struct Locomotive {
    pub location: Entity,
    pub target: u32,
    pub distance: f32,
    pub speed: f32,
}

impl Locomotive {
    pub fn new(speed: f32, location: Entity, target: u32) -> Locomotive {
        Locomotive {
            location,
            target,
            distance: 0.0,
            speed,
        }
    }

    pub fn log_location(&self) {
        println!(
            "track: {}, target: {}, distance: {}",
            self.location.index(),
            self.target,
            self.distance
        )
    }
}
