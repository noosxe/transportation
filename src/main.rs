mod components;

use bevy::prelude::*;
use components::{locomotive::Locomotive, track::Track};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NodePlugin)
        .run();
}

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(boot)
            .add_startup_system(add_tracks)
            .add_system(advance_logomotive);
    }
}

fn boot() {
    println!("booting...")
}

#[derive(Component)]
struct Node;

fn add_tracks(mut commands: Commands) {
    let mut track1 = Track::simple(10.0);
    let mut track2 = Track::simple(10.0);
    let mut track3 = Track::simple(10.0);

    let tr1 = commands.spawn_empty().id();
    let tr2 = commands.spawn_empty().id();
    let tr3 = commands.spawn_empty().id();

    track1.attach_at(0, tr3);
    track1.attach_at(1, tr2);

    track2.attach_at(0, tr1);
    track2.attach_at(1, tr3);

    track3.attach_at(0, tr2);
    track3.attach_at(1, tr1);

    commands.entity(tr1).insert(track1);
    commands.entity(tr2).insert(track2);
    commands.entity(tr3).insert(track3);

    commands.spawn(Locomotive::new(2.0, tr1, 1));
}

fn log_tracks(query: Query<&Track>) {
    for track in &query {
        track.log_joints();
    }
}

fn advance_logomotive(
    time: Res<Time>,
    mut loco_query: Query<&mut Locomotive>,
    track_query: Query<&Track>,
) {
    for mut loco in loco_query.iter_mut() {
        let dist_delta = loco.speed * time.delta_seconds();
        loco.distance -= dist_delta;

        if loco.distance < 0.0 {
            if let Ok(current) = track_query.get(loco.location) {
                let next_track_entity = current
                    .find_joint(loco.target)
                    .and_then(|joint| joint.connected_entity);
                let next_track = next_track_entity.and_then(|next| track_query.get(next).ok());

                assert!(next_track.is_some(), "next_track not found");

                let connection_joint =
                    next_track.and_then(|track| track.find_joint_by_entity(loco.location));

                assert!(connection_joint.is_some(), "connection_joint not found");

                if next_track.is_some() && connection_joint.is_some() {
                    let connections = next_track
                        .unwrap()
                        .find_connections_for_joint(connection_joint.unwrap().id);

                    let next_connection = connections
                        .iter()
                        .find(|&&x| x.j1 == connection_joint.unwrap().id);

                    assert!(next_connection.is_some(), "next_connection not found");

                    let target_joint = next_connection.map(|x| x.j2);

                    assert!(target_joint.is_some(), "target_joint not found");

                    if target_joint.is_some() {
                        println!(
                            "assigning location: {}, target: {}",
                            next_track_entity.unwrap().index(),
                            target_joint.unwrap(),
                        );

                        loco.target = target_joint.unwrap();
                        loco.location = next_track_entity.unwrap();
                        loco.distance = next_connection.unwrap().distance + loco.distance;
                    }
                }
            }
        }
    }
}

fn log_locomotive(query: Query<&Locomotive>) {
    for loco in &query {
        loco.log_location();
    }
}
