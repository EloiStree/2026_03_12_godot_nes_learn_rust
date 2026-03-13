use godot::prelude::*;

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
pub struct CeasarOffsetParser {
    base: Base<Node>,
    #[export]
    ceasar_offset: i32,
}

#[godot_api]
impl INode for CeasarOffsetParser {
    fn init(base: Base<Node>) -> Self {
        Self { base, ceasar_offset: 0 }
    }
}

#[godot_api]
impl CeasarOffsetParser {
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
