use winit::{
    event::{ElementState, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::math::vec3::Vec3;

use super::camera::Camera;

pub struct CameraController {
    speed: f32,
    sensitivity: f32,

    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,

    fast: bool,

    mouse_pressed: bool,
    pub last_mouse_pos: Option<(f64, f64)>,

    mouse_delta: (f32, f32),
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,

            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
            fast: false,

            mouse_pressed: false,
            last_mouse_pos: None,

            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent, camera: &mut Camera) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => self.forward = pressed,
                    PhysicalKey::Code(KeyCode::KeyS) => self.backward = pressed,
                    PhysicalKey::Code(KeyCode::KeyA) => self.left = pressed,
                    PhysicalKey::Code(KeyCode::KeyD) => self.right = pressed,
                    PhysicalKey::Code(KeyCode::Space) => self.up = pressed,
                    PhysicalKey::Code(KeyCode::ShiftLeft)
                    | PhysicalKey::Code(KeyCode::ShiftRight) => self.fast = pressed,
                    PhysicalKey::Code(KeyCode::KeyC) => self.down = pressed,
                    _ => {}
                }
            }

            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
            }

            WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_pressed {
                    if let Some((lx, ly)) = self.last_mouse_pos {
                        let dx = (position.x - lx) as f32;
                        let dy = (position.y - ly) as f32;

                        self.mouse_delta.0 += dx;
                        self.mouse_delta.1 += dy;
                    }

                    self.last_mouse_pos = Some((position.x, position.y));
                }
            }

            _ => {}
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32) {
        let forward = camera.forward();
        let right = camera.right();

        let mut speed = self.speed * dt;

        if self.fast {
            speed *= 3.0;
        }

        if self.forward {
            camera.position += forward * speed;
        }
        if self.backward {
            camera.position -= forward * speed;
        }
        if self.right {
            camera.position += right * speed;
        }
        if self.left {
            camera.position -= right * speed;
        }
        if self.up {
            camera.position += Vec3::Y * speed;
        }
        if self.down {
            camera.position -= Vec3::Y * speed;
        }

        self.mouse_look(camera, self.mouse_delta.0, self.mouse_delta.1);
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn mouse_look(&mut self, camera: &mut Camera, dx: f32, dy: f32) {
        camera.yaw += dx * self.sensitivity * 0.01;
        camera.pitch -= dy * self.sensitivity * 0.01;

        camera.pitch = camera.pitch.clamp(-1.5, 1.5);
    }
}
