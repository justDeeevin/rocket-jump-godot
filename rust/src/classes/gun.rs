use godot::{
    engine::{IMeshInstance3D, MeshInstance3D, RayCast3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base = MeshInstance3D)]
pub struct Gun {
    raycast: Gd<RayCast3D>,

    base: Base<MeshInstance3D>,
}

#[godot_api]
impl IMeshInstance3D for Gun {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Gun {
            base,
            raycast: RayCast3D::new_alloc(),
        }
    }

    fn ready(&mut self) {
        self.raycast = self.base().get_node_as("RayCast");
    }
}

#[godot_api]
impl Gun {
    pub fn shoot(&self) -> Option<Vector3> {
        if self.raycast.is_colliding() {
            let point = self.raycast.get_collision_point();
            return Some(point);
        }
        None
    }
}
