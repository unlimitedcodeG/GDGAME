use godot::prelude::*;

mod player;

struct HelloWorldExtension;

#[gdextension]
unsafe impl ExtensionLibrary for HelloWorldExtension {}
