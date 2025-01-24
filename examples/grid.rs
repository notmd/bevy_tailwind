//! An example that rewrite the [grid](https://github.com/bevyengine/bevy/blob/main/examples/ui/grid.rs) example from `bevy` with `bevy_tailwind`
use bevy::{color::palettes::css::*, prelude::*};
use bevy_tailwind::tw;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [800., 600.].into(),
                title: "Bevy CSS Grid Layout Example".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_layout)
        .run();
}

fn spawn_layout(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Top-level grid (app frame)
    commands
        .spawn((
            tw!(
                Node {
                    grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                    grid_template_rows: vec![GridTrack::auto(), GridTrack::flex(1.0), GridTrack::px(20.)],
                    ..Default::default()
                },
                "grid size-full"
            ),
           tw!("bg-white") 
        ))
        .with_children(|builder| {
            // Header
            builder
                .spawn(tw!("grid col-span-2 p-1.5"))
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder,  "Bevy CSS Grid Layout Example");
                });

            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn((
                    tw!("h-full aspect-square grid p-6 grid-cols-4 grid-rows-4 gap-3"),
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                ))
                .with_children(|builder| {
                    // Note there is no need to specify the position for each grid item. Grid items that are
                    // not given an explicit position will be automatically positioned into the next available
                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
                    // style property.

                    item_rect(builder, ORANGE);
                    item_rect(builder, BISQUE);
                    item_rect(builder, BLUE);
                    item_rect(builder, CRIMSON);
                    item_rect(builder, AQUA);
                    item_rect(builder, ORANGE_RED);
                    item_rect(builder, DARK_GREEN);
                    item_rect(builder, FUCHSIA);
                    item_rect(builder, TEAL);
                    item_rect(builder, ALICE_BLUE);
                    item_rect(builder, CRIMSON);
                    item_rect(builder, ANTIQUE_WHITE);
                    item_rect(builder, YELLOW);
                    item_rect(builder, DEEP_PINK);
                    item_rect(builder, YELLOW_GREEN);
                    item_rect(builder, SALMON);
                });

            // Right side bar (auto placed in row 2, column 2)
            builder
                .spawn((
                    tw!(Node {
                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
                        ..default()
                    }, "grid items-center justify-center p-2.5 gap-y-2.5"),
                    tw!("bg-black")
                ))
                .with_children(|builder| {
                    builder.spawn(Text::new("Sidebar"));
                    builder.spawn((Text::new("A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely."),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                    ));
                    builder.spawn(Node::default());
                });

            // Footer / status bar
            builder.spawn(tw!("col-span-2 bg-white"));

            // Modal (absolutely positioned on top of content - currently hidden: to view it, change its visibility)
            builder.spawn((
                tw!("absolute mt-25 w-60/100 h-[300px] max-w-[600px] bg-white/80"),
                Visibility::Hidden
            ));
        });
}

/// Create a colored rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take its size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Srgba) {
    builder
        .spawn(tw!("grid p-0.75 bg-black"))
        .with_children(|builder| {
            builder.spawn((Node::default(), BackgroundColor(color.into())));
        });
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, text: &str) {
    builder.spawn((Text::new(text), tw!("text-black")));
}
