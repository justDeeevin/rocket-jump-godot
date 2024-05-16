use godot::{
    engine::{IMeshInstance3D, MeshInstance3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base = MeshInstance3D)]
pub struct Explosion {
    #[export]
    decay_speed: f32,
    #[export]
    pub explosion_force: f32,
    #[export]
    pub radius: f32,
    base: Base<MeshInstance3D>,
}

#[godot_api]
impl IMeshInstance3D for Explosion {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Explosion {
            decay_speed: 0.5,
            explosion_force: 10.0,
            radius: 10.0,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut scale = self.base().get_scale();
        scale -= Vector3::ONE * self.decay_speed * delta as f32;
        self.base_mut().set_scale(scale);
        if scale.x <= 0.01 {
            self.base_mut().queue_free();
        }
    }
}
