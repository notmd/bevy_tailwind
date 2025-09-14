#![allow(unused_imports)]
use bevy::{color::palettes::css::*, prelude::*};
use bevy_tailwind::tw;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

fn test() {
    // tw!("border-transparent hover:border-black focus:border-white");
    // tw!("border-transparent/50");
    // tw!("border-black");
    // tw!("border-black/50");
    // tw!("border-white");
    // tw!("border-white/50");
    // tw!("border-gray-100");
    // tw!("border-gray-100/50");
    // tw!("border-color": Color::WHITE);

    // tw!("border-x-transparent hover:border-x-black focus:border-x-white");
    // tw!("border-x-transparent/50");
    // tw!("border-x-black");
    // tw!("border-x-black/50");
    // tw!("border-x-white");
    // tw!("border-x-white/50");
    // tw!("border-x-gray-100");
    // tw!("border-x-gray-100/50");
    tw!("border-x-color": Color::WHITE);
    // tw!("hover:bg": Color::WHITE, "hover:rounded");

    // tw!("border-y-transparent hover:border-y-black focus:border-y-white");
    // tw!("border-y-transparent/50");
    // tw!("border-y-black");
    // tw!("border-y-black/50");
    // tw!("border-y-white");
    // tw!("border-y-white/50");
    // tw!("border-y-gray-100");
    // tw!("border-y-gray-100/50");
    // tw!("border-y-color": Color::WHITE);

    // tw!("border-t-transparent hover:border-t-black focus:border-t-white");
    // tw!("border-t-transparent/50");
    // tw!("border-t-black");
    // tw!("border-t-black/50");
    // tw!("border-t-white");
    // tw!("border-t-white/50");
    // tw!("border-t-gray-100");
    // tw!("border-t-gray-100/50");
    // tw!("border-t-color": Color::WHITE);

    // tw!("border-r-transparent hover:border-r-black focus:border-r-white");
    // tw!("border-r-transparent/50");
    // tw!("border-r-black");
    // tw!("border-r-black/50");
    // tw!("border-r-white");
    // tw!("border-r-white/50");
    // tw!("border-r-gray-100");
    // tw!("border-r-gray-100/50");
    // tw!("border-r-color": Color::WHITE);

    // tw!("border-b-transparent hover:border-b-black focus:border-b-white");
    // tw!("border-b-transparent/50");
    // tw!("border-b-black");
    // tw!("border-b-black/50");
    // tw!("border-b-white");
    // tw!("border-b-white/50");
    // tw!("border-b-gray-100");
    // tw!("border-b-gray-100/50");
    // tw!("border-b-color": Color::WHITE);

    // tw!("border-l-transparent hover:border-t-black focus:border-t-white");
    // tw!("border-l-transparent/50");
    // tw!("border-l-black");
    // tw!("border-l-black/50");
    // tw!("border-l-white");
    // tw!("border-l-white/50");
    // tw!("border-l-gray-100");
    // tw!("border-l-gray-100/50");
    // tw!("border-t-color": Color::WHITE);
}
