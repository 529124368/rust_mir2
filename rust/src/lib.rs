mod diablo_player;
mod man_base;
mod mosnter;
mod tools;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<diablo_player::DiabloPlayer>();
    handle.add_class::<mosnter::Mosnter>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
