use godot::prelude::*;

mod player;
mod double_console_game_code_edit;

struct LearnRustInGodot;

#[gdextension]
unsafe impl ExtensionLibrary for LearnRustInGodot {}
