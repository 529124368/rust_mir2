mod man_base;
mod other_player;
mod player;
mod tools;
mod websocket;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<websocket::Websocket>();
    handle.add_class::<player::Player>();
    handle.add_class::<other_player::OtherPlayer>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
