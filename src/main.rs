pub mod body;
pub mod math;
pub mod physics;
pub mod render;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::{
    body::ball::Ball, math::vec3::Vec3, physics::world::PhysicsWorld, render::renderer::Renderer,
};

struct App {
    window: Option<Window>,
    renderer: Option<Renderer<'static>>,
    world: PhysicsWorld,
    last_time: std::time::Instant,
    spawn_pressed: bool,
}

impl App {
    fn new() -> Self {
        let grid = 5;
        let spacing = 2.2;

        let mut world = PhysicsWorld::new(
            2.5,
            10,
            Vec3 {
                x: 0.0,
                y: -9.8,
                z: 0.0,
            },
        );

        for y in 5..grid + 5 {
            for x in 0..grid {
                for z in 0..grid {
                    let pos = Vec3 {
                        x: x as f32 * spacing - (grid as f32 * spacing * 0.5),
                        y: y as f32 * spacing - (grid as f32 * spacing * 0.5),
                        z: z as f32 * spacing - (grid as f32 * spacing * 0.5),
                    };

                    let mut ball = Ball::new(pos);
                    ball.velocity = Vec3 {
                        x: rand::random_range(-2.0..2.0),
                        y: 0.0,
                        z: rand::random_range(-2.0..2.0),
                    };

                    world.add_ball(ball);
                }
            }
        }

        Self {
            window: None,
            renderer: None,
            world,
            last_time: std::time::Instant::now(),
            spawn_pressed: false,
        }
    }

    fn spawn_ball(&mut self) {
        let mut ball = Ball::new(Vec3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        });

        ball.velocity = Vec3 {
            x: rand::random_range(-3.0..3.0),
            y: rand::random_range(2.0..6.0),
            z: rand::random_range(-3.0..3.0),
        };

        self.world.add_ball(ball);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default().with_title("Physics Engine"))
            .unwrap();

        // 🔥 SAFETY TRICK: extend lifetime for wgpu surface
        let window_ref: &'static Window = Box::leak(Box::new(window));

        let renderer = pollster::block_on(Renderer::new(window_ref));

        self.window = Some(unsafe { std::ptr::read(window_ref) });
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(renderer) = &mut self.renderer {
            renderer
                .controller
                .process_events(&event, &mut renderer.camera);
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                let dt = (now - self.last_time).as_secs_f32();
                self.last_time = now;

                self.world.step(dt);

                if let Some(renderer) = &mut self.renderer {
                    renderer.update(&self.world, dt);
                    renderer.render();
                }
            }

            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                }
            }

            WindowEvent::KeyboardInput { ref event, .. } => {
                if let Some(key) = event.logical_key.to_text()
                    && (key == "b" || key == "B")
                {
                    match event.state {
                        ElementState::Pressed => {
                            if !self.spawn_pressed {
                                self.spawn_ball();
                                self.spawn_pressed = true;
                            }
                        }
                        ElementState::Released => self.spawn_pressed = false,
                    }
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();

    event_loop.run_app(&mut app).unwrap();
}
