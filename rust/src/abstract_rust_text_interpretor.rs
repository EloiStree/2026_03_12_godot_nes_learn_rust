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
pub struct MarkdownParser {
    base: Base<Node>,
}

#[godot_api]
impl INode for MarkdownParser {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl MarkdownParser {
    #[func]
    fn parse(&self, text: String) -> String {
        format!("Parsed: {}", text)
    }
}
