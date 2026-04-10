use crate::{body::ball::Ball, math::vec3::Vec3, physics::grid::SpatialGrid};

pub struct PhysicsWorld {
    pub balls: Vec<Ball>,
    pub gravity: Vec3,
    pub grid: SpatialGrid,
    pub solver_iterations: usize,
}

impl PhysicsWorld {
    pub fn new(cell_size: f32, solver_iterations: usize, gravity: Vec3) -> Self {
        Self {
            balls: Vec::new(),
            grid: SpatialGrid::new(cell_size),
            gravity,
            solver_iterations,
        }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    fn apply_forces(&mut self) {
        for ball in &mut self.balls {
            let mass = ball.mass;
            ball.apply_force(self.gravity * mass);
        }
    }

    fn integrate(&mut self, dt: f32) {
        for ball in &mut self.balls {
            ball.integrate(dt);
        }
    }

    fn board_phase(&mut self) {
        self.grid.clear();
        for (i, ball) in self.balls.iter().enumerate() {
            self.grid.insert(i, ball.position);
        }
    }

    fn solve_floor_collision(ball: &mut Ball, dt: f32) {
        if ball.position.y - ball.radius <= 0.0 {
            ball.position.y = ball.radius;

            let normal = Vec3::Y;

            // --- Reflect (bounce)
            ball.velocity = ball.velocity.reflect(normal) * ball.restitution;

            // --- Tangential velocity (horizontal)
            let tangent = Vec3 {
                x: ball.velocity.x,
                y: 0.0,
                z: ball.velocity.z,
            };

            let radius = ball.radius;

            // v = ω × r → approximate
            ball.angular_velocity = Vec3 {
                x: -tangent.z / radius,
                y: 0.0,
                z: tangent.x / radius,
            };

            let speed = tangent.length();

            if speed > 1e-4 {
                let dir = tangent / speed;

                // Coulomb friction
                let friction_force = ball.friction * 9.8;

                let dv = friction_force * dt;

                if dv > speed {
                    // STATIC FRICTION → fully stop
                    ball.velocity.x = 0.0;
                    ball.velocity.z = 0.0;
                } else {
                    // DYNAMIC FRICTION → slow down
                    let new_speed = speed - dv;
                    ball.velocity.x = dir.x * new_speed;
                    ball.velocity.z = dir.z * new_speed;
                }
            }

            // Kill tiny bounce
            if ball.velocity.y.abs() < 0.05 {
                ball.velocity.y = 0.0;
            }
        }
    }

    fn resolve_collision(&mut self, dt: f32) {
        let mut neighbors = Vec::new();

        for _ in 0..self.solver_iterations {
            for i in 0..self.balls.len() {
                self.grid.neighbors(self.balls[i].position, &mut neighbors);

                for &j in &neighbors {
                    if i >= j {
                        continue;
                    }
                    let (left, right) = self.balls.split_at_mut(j);
                    let a = &mut left[i];
                    let b = &mut right[0];

                    a.resolve_collision(b);
                }
            }

            for ball in &mut self.balls {
                Self::solve_floor_collision(ball, dt);
            }
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.apply_forces();
        self.integrate(dt);
        self.board_phase();
        self.resolve_collision(dt);
    }
}
