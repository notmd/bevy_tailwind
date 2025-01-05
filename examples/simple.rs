use bevy::prelude::*;
use bevy_tailwind::tw;
fn main() {
    let mut node = tw!("flex", {
        "flex-grow shrink": false
    });

    tw!(node, "block",{
        "flex-grow flex-wrap": true,
        "shrink": false
    });

    fn get_node() -> Node {
        Node::default()
    }
    tw!("z-10");
    tw!(get_node(), "flex");
    let mut z_index = ZIndex(10);
    tw!(z_index, "-z-10");
    tw!(z_index, {
        "-z-20": true
    });
}

fn my_system(mut node: Query<(Entity, &mut Node)>) {
    for (entity, mut node) in node.iter_mut() {
        tw!(node, "flex");
    }
}
