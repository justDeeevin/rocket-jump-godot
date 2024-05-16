mod classes;

use classes::{Explosion, Player};
use godot::prelude::*;

struct RocketJump;

#[gdextension]
unsafe impl ExtensionLibrary for RocketJump {}

#[derive(GodotClass)]
#[class(base = Node)]
struct Main {
    #[export]
    explosion_scene: Gd<PackedScene>,

    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            explosion_scene: PackedScene::new_gd(),
            base,
        }
    }
}

#[godot_api]
impl Main {
    #[func]
    fn on_hit(&mut self, point: Vector3) {
        let mut explosion = self.explosion_scene.instantiate_as::<Explosion>();
        explosion.set_position(point);
        self.base_mut()
            .add_child(explosion.clone().upcast::<Node>());
        let explosion_props = explosion.bind();

        let mut player = self.base().get_node_as::<Player>("Player");

        let difference = player.get_position() - point;
        let direction = difference.normalized();
        let distance = difference.length();

        if distance < explosion_props.radius {
            player
                .bind_mut()
                .add_velocity(direction * (explosion_props.explosion_force / (distance)));
        }
    }
}
