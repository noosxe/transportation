use bevy::prelude::*;

pub struct Joint {
    pub id: u32,
    connected: bool,
    pub connected_entity: Option<Entity>,
}

impl Joint {
    fn empty(id: u32) -> Joint {
        Joint {
            id,
            connected: false,
            connected_entity: None,
        }
    }
}

pub struct Connection {
    pub distance: f32,
    pub j1: u32,
    pub j2: u32,
}

impl Connection {
    fn new(distance: f32, j1: u32, j2: u32) -> Connection {
        Connection { distance, j1, j2 }
    }
}

#[derive(Component)]
pub struct Track {
    pub joints: Vec<Joint>,
    pub connections: Vec<Connection>,
}

impl Track {
    pub fn simple(distance: f32) -> Track {
        let joints = vec![Joint::empty(0), Joint::empty(1)];
        let connections = vec![
            Connection::new(distance, 0, 1),
            Connection::new(distance, 1, 0),
        ];

        Track {
            joints,
            connections,
        }
    }

    pub fn attach_at(&mut self, id: u32, entiy: Entity) {
        let f = self.joints.iter_mut().find(|x| x.id == id);

        match f {
            Some(joint) => {
                joint.connected_entity = Some(entiy);
                joint.connected = true;
            }
            None => println!("Joint {} not found", id),
        }
    }

    pub fn find_joint(&self, id: u32) -> Option<&Joint> {
        return self.joints.iter().find(|x| x.id == id);
    }

    pub fn find_joint_by_entity(&self, entiy: Entity) -> Option<&Joint> {
        return self
            .joints
            .iter()
            .find(|&x| x.connected_entity.is_some() && x.connected_entity.unwrap() == entiy);
    }

    pub fn find_connections_for_joint(&self, id: u32) -> Vec<&Connection> {
        return self
            .connections
            .iter()
            .filter(|&x| x.j1 == id || x.j2 == id)
            .collect();
    }

    pub fn log_joints(&self) {
        for joint in self.joints.iter() {
            println!("joint id {}, connected {}", joint.id, joint.connected);
        }
    }
}
