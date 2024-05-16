mod classes;

use godot::prelude::*;

struct RocketJump;

#[gdextension]
unsafe impl ExtensionLibrary for RocketJump {}

#[derive(GodotClass)]
#[class(base = Node)]
struct Main {
    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Main { base }
    }
}
