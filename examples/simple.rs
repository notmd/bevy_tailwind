use bevy::prelude::*;
use bevy_tailwind::tw;
fn main() {
    // let _bundle = tw!("flex ", {
    //     "p-5 unknow": false
    // });
    fn get_node() -> Node {
        Node::default()
    }
    tw!("z-10");
    tw!(get_node(), "flex");
    let mut z_index = ZIndex(10);
    tw!(z_index, "-z-10");
}

fn my_system(mut node: Query<(Entity, &mut Node)>, mut commands: Commands) {
    for (entity, mut node) in node.iter_mut() {
        tw!(node, "flex");
    }

    commands.spawn((Marker, tw!("flex z-10")));
}

#[derive(Component)]
struct Marker;
