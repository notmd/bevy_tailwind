use bevy::prelude::*;
use bevy_tailwind::tw;
fn main() {
    let mut node = tw!("col-span-1", {
        "flex-grow shrink col-span-1": false
    });

    tw!(&mut node, "col-span-1");

    tw!(&mut node, "block pt-2",{
        "flex-grow flex-wrap col-span-1": true,
        "shrink": false
    });

    // fn get_node() -> Node {
    //     Node::default()
    // }
    // tw!("z-10");
    // tw!(get_node(), "flex");
    // let mut z_index = ZIndex(10);
    // tw!(z_index, "-z-10");
    // tw!(z_index, {
    //     "-z-20": trueclass_type
    // });

    // tw!(node, {"pt-2 pl-4 py-px": true});
}

fn my_system(mut node: Query<(Entity, &mut Node)>) {
    for (entity, mut node) in node.iter_mut() {
        tw!(node, "flex");
    }
}
