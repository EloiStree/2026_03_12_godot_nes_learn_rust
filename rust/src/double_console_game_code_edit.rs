use std::io::SeekFrom;

use godot::global::Error;
use godot::prelude::*;
use godot::classes::{CodeEdit, Button, Node};
use crate::abstract_rust_text_interpretor::AbstractParserNode;

#[derive(GodotClass)]
#[class(base=Node)]
struct DoubleConsoleGameCodeEdit {
    base: Base<Node>,

    #[export]
    input_codeedit: Option<Gd<CodeEdit>>,

    #[export]
    output_codeedit: Option<Gd<CodeEdit>>,

    #[export]
    submit_button: Option<Gd<Button>>,

    #[export]
    godot_interpretor: Option<Gd<Node>>,

    #[export]
    rust_interpretor: Option<Gd<Node>>,
}

#[godot_api]
impl DoubleConsoleGameCodeEdit {
    #[signal]
    fn on_submited_text(givent_text: String);
    #[signal]
    fn on_parsed_text(givent_text: String);

    #[func]
    fn submit_code(&mut self) {
        let Some(input) = self.input_codeedit.as_mut() else {
            godot_warn!("input_codeedit is not assigned!");
            return;
        };

        let Some(output) = self.output_codeedit.as_mut() else {
            godot_warn!("output_codeedit is not assigned!");
            return;
        };

        let code = input.get_text();
        output.set_text(&code);

        if let Some(mut interpretor) = self.godot_interpretor.as_mut() {
            let result = interpretor.call("parse", &[code.to_variant()]);
            let parsed_text: String = result.to();
            output.set_text(&parsed_text);
        }

        if let Some(mut rust_interpretor) = self.rust_interpretor.as_mut() {
            let result = rust_interpretor.call("parse", &[code.to_variant()]);
            let parsed_text: String = result.to();
            output.set_text(&parsed_text);
        }



        self.base.to_gd().emit_signal("on_submited_text", &[code.to_variant()]);
        self.base.to_gd().emit_signal("on_parsed_text", &[output.get_text().to_variant()]); 
    }
}

#[godot_api]
impl INode for DoubleConsoleGameCodeEdit {
    fn init(base: Base<Node>) -> Self {
        godot_print!("DoubleConsoleGameCodeEdit initialized");
        
        Self {
            base,
            input_codeedit: None,
            output_codeedit: None,
            submit_button: None,
            godot_interpretor: None,
            rust_interpretor: None,
        }
    }

    fn ready(&mut self) {
        let Some(button) = self.submit_button.as_mut() else {
            godot_warn!("submit_button is not assigned in editor!");
            return;
        };

        // Modern, clean way to connect in gdext 0.2.x / Godot 4.2+
        let callable = self.base.to_gd().callable("submit_code");
        let err = button.connect("pressed", &callable);
        if err != Error::OK {
            godot_error!("Failed to connect 'pressed' signal: {err:?}");
        }
    }
}