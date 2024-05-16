use crate::classes::Gun;
use godot::{
    engine::{
        input::MouseMode, utilities::clamp, CharacterBody3D, ICharacterBody3D, InputEvent,
        InputEventMouseMotion, Marker3D,
    },
    prelude::*,
};

#[derive(GodotClass)]
#[class(base = CharacterBody3D)]
pub struct Player {
    /// How fast the player moves, m/s
    #[export]
    speed: f32,
    /// The downward acceleration when in the air, m/s^2
    #[export]
    fall_acceleration: f32,
    /// Vertical impulse applied to the character upon jumping, m/s
    #[export]
    jump_impulse: f32,
    #[export]
    look_sensitivity: f32,
    #[export]
    friction: f32,
    gun: Gd<Gun>,
    camera_pivot: Gd<Marker3D>,
    camera: Gd<Camera3D>,
    target_velocity: Vector3,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Player {
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            look_sensitivity: 0.03,
            camera_pivot: Marker3D::new_alloc(),
            camera: Camera3D::new_alloc(),
            gun: Gun::new_alloc(),
            target_velocity: Vector3::ZERO,
            friction: 10.0,
            base,
        }
    }

    fn ready(&mut self) {
        self.camera_pivot = self.base().get_node_as("CameraPivot");
        self.camera = self.camera_pivot.get_node_as("Camera");
        self.gun = self.camera.get_node_as("Gun");

        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let input_direction = input.get_vector(
            "move_left".into(),
            "move_right".into(),
            "move_forward".into(),
            "move_back".into(),
        );
        let direction = (self.camera_pivot.get_transform().basis
            * Vector3::new(input_direction.x, 0.0, input_direction.y))
        .normalized();

        if direction != Vector3::ZERO {
            self.target_velocity.x = direction.x * self.speed;
            self.target_velocity.z = direction.z * self.speed;
        }

        if !self.base().is_on_floor() {
            self.target_velocity.y -= self.fall_acceleration * delta as f32;
        }

        if self.base().is_on_floor() {
            if direction == Vector3::ZERO
                && self.target_velocity.x != 0.0
                && self.target_velocity.z != 0.0
            {
                self.target_velocity.x *= 1.0 / self.friction;
                self.target_velocity.z *= 1.0 / self.friction;
            }

            if input.is_action_just_pressed("jump".into()) {
                self.target_velocity.y = self.jump_impulse;
            }
        }

        let target_velocity = self.target_velocity;
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let mut input = Input::singleton();

        if event.is_action_pressed("ui_cancel".into())
            && input.get_mouse_mode() == MouseMode::CAPTURED
        {
            input.set_mouse_mode(MouseMode::VISIBLE);
        }
        if event.is_action_pressed("left_click".into()) {
            if input.get_mouse_mode() == MouseMode::CAPTURED {
                let point = self.gun.bind().shoot();
                if let Some(point) = point {
                    self.base_mut()
                        .emit_signal("hit".into(), &[Variant::from(point)]);
                }
            } else {
                input.set_mouse_mode(MouseMode::CAPTURED);
            }
        }

        if let Ok(mouse_motion) = event.try_cast::<InputEventMouseMotion>() {
            if input.get_mouse_mode() != MouseMode::CAPTURED {
                return;
            }

            self.camera_pivot
                .rotate_y(-mouse_motion.get_relative().x * self.look_sensitivity);
            self.camera
                .rotate_x(-mouse_motion.get_relative().y * self.look_sensitivity);

            let camera_rotation = self.camera.get_rotation();
            self.camera.set_rotation(Vector3::new(
                clamp(
                    Variant::from(camera_rotation.x),
                    Variant::from(-90_f32.to_radians()),
                    Variant::from(90_f32.to_radians()),
                )
                .to(),
                camera_rotation.y,
                camera_rotation.z,
            ));
        }
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn hit(point: Vector3);

    pub fn add_velocity(&mut self, velocity: Vector3) {
        self.target_velocity += velocity;
    }
}
