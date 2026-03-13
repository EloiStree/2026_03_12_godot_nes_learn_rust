use godot::prelude::*;

/// I wanted to do inheritance here.
/// But I did not find out how to do it in godot-rust.
/// So I did not for now.

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AbstractParserNode {
    base: Base<Node>,
}

#[godot_api]
impl INode for AbstractParserNode {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl AbstractParserNode {
    #[func]
    fn parse(&self, text: String) -> String {
        text
    }
}


#[derive(GodotClass)]
#[class(base=Node)]
pub struct RustCeasarOffsetParser {
    base: Base<Node>,
    #[export]
    ceasar_offset: i32,
}

#[godot_api]
impl INode for RustCeasarOffsetParser {
    fn init(base: Base<Node>) -> Self {
        Self { base, ceasar_offset: 1 }
    }
}

#[godot_api]
impl RustCeasarOffsetParser {
    #[func]
    fn parse(&self, text: String) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = (c as u8 - first + self.ceasar_offset as u8) % 26;
                    (first + offset) as char
                } else {
                    c
                }
            })
            .collect()
    }
}


#[derive(GodotClass)]
#[class(base=Node)]
pub struct RustUpperCaseParser {
    base: Base<Node>,
}

#[godot_api]
impl INode for RustUpperCaseParser {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl RustUpperCaseParser {
    #[func]
    fn parse(&self, text: String) -> String {
        text.to_uppercase()
    }
}