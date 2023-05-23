mod components;

use bevy::prelude::*;

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
            .add_startup_system(add_nodes)
            .add_system(log_nodes);
    }
}

fn boot() {
    println!("booting...")
}

#[derive(Component)]
struct Node;

fn add_nodes(mut commands: Commands) {
    let node_1 = commands.spawn(Node).id();
    let node_2 = commands.spawn(Node).id();
    let node_3 = commands.spawn(Node).id();

    commands.entity(node_1).add_child(node_2);
    commands.entity(node_2).add_child(node_3);
    commands.entity(node_3).add_child(node_1);
}

fn log_nodes(query: Query<(Entity, &Children), With<Node>>) {
    for (_, children) in &query {
        println!("children {}", children.len());
    }
}
