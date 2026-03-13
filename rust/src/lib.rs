use godot::prelude::*;

mod player;
mod double_console_game_code_edit;
mod abstract_rust_text_interpretor;

struct LearnRustInGodot;

#[gdextension]
unsafe impl ExtensionLibrary for LearnRustInGodot {}
