use crate::math::{quat::Quat, vec3::Vec3};

pub struct RigidBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub force: Vec3,

    pub orientation: Quat,

    pub angular_velocity: Vec3,
    pub torque: Vec3,

    pub mass: f32,
    pub inv_mass: f32,

    pub inertia: f32,
    pub inv_inertia: f32,

    pub restitution: f32,
    pub friction: f32,
}

impl RigidBody {
    pub fn apply_force(&mut self, f: Vec3) {
        self.force += f;
    }

    fn integrate_force(&mut self, dt: f32) {
        let acceleration = self.force / self.mass;
        self.velocity += acceleration * dt;
        self.force = Vec3::ZERO;
    }

    fn integrate_velocity(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    fn integrate_torque(&mut self, dt: f32) {
        let angular_acc = self.torque * self.inv_inertia;
        self.angular_velocity += angular_acc * dt;

        self.torque = Vec3::ZERO;
    }

    fn integrate_orientation(&mut self, dt: f32) {
        let w = self.angular_velocity;

        if w.length_squared() > 0.0 {
            let axis = w.normalize();
            let angle = w.length() * dt;

            let dq = Quat::from_axis_angle(axis, angle);

            self.orientation = (dq * self.orientation).normalize();
        }
    }

    pub fn integrate(&mut self, dt: f32) {
        self.integrate_force(dt);
        self.integrate_torque(dt);
        self.integrate_velocity(dt);
        self.integrate_orientation(dt);
    }
}
