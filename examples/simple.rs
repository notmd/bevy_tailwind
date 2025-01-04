use bevy::prelude::*;
use bevy_tailwind::tw;
fn main() {
    // let _bundle = tw!("flex ", {
    //     "p-5 unknow": false
    // });
    let mut node = Node::default();
    fn get_node() -> Node {
        Node::default()
    }
    tw!(get_node(), "flex");
    tw!("flex");
}

fn my_system(mut node: Query<(Entity, &mut Node)>) {
    for (entity, mut node) in node.iter_mut() {
        tw!(node, "flex");
    }
}
