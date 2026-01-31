use godot::prelude::*;

struct HelloWorldExtension;

#[gdextension]
unsafe impl ExtensionLibrary for HelloWorldExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct HelloWorld {
    base: Base<Node>,
}

#[godot_api]
impl INode for HelloWorld {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("Hello, World from Rust!");
    }
}

#[godot_api]
impl HelloWorld {
    #[func]
    pub fn greet(&self, name: GString) -> GString {
        GString::from(format!("Hello, {}!", name))
    }
}
