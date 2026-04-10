use std::ops::{Deref, DerefMut};

use crate::{
    body::rigid::RigidBody,
    math::{quat::Quat, vec3::Vec3},
};

pub struct Ball {
    pub body: RigidBody,
    pub radius: f32,
}

impl Deref for Ball {
    type Target = RigidBody;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for Ball {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}

impl Ball {
    pub fn new(pos: Vec3) -> Self {
        let mass = 1.0;
        let radius = 1.0;

        let inertia = (2.0 / 5.0) * mass * radius * radius;

        Self {
            body: RigidBody {
                position: pos,
                velocity: Vec3::ZERO,
                force: Vec3::ZERO,

                orientation: Quat::IDENTITY,

                angular_velocity: Vec3::ZERO,
                torque: Vec3::ZERO,

                mass,
                inv_mass: 1.0 / mass,

                inertia,
                inv_inertia: 1.0 / inertia,

                restitution: 0.2,
                friction: 0.6,
            },
            radius: 1.0,
        }
    }
    pub fn resolve_collision(&mut self, b: &mut Ball) {
        const BAUMGARTE: f32 = 0.2;

        let delta = b.position - self.position;
        let distance = delta.length();

        let radius_sum = self.radius + b.radius;

        if distance >= radius_sum || distance < 1e-6 {
            return;
        }

        let normal = delta / distance;

        let ra = normal * self.radius;
        let rb = -normal * b.radius;

        let ra_cross_n = ra.cross(normal);
        let rb_cross_n = rb.cross(normal);

        let denom = self.inv_mass
            + b.inv_mass
            + ra_cross_n.length_squared() * self.inv_inertia
            + rb_cross_n.length_squared() * b.inv_inertia;

        let va = self.velocity + self.angular_velocity.cross(ra);
        let vb = b.velocity + b.angular_velocity.cross(rb);

        let relative_velocity = vb - va;

        let vel_along_normal = relative_velocity.dot(normal);

        if vel_along_normal > 0.0 {
            return;
        }

        let tangent = (relative_velocity - normal * vel_along_normal)
            .try_normalize()
            .unwrap_or(Vec3::ZERO);
        let vel_along_tangent = relative_velocity.dot(tangent);

        let penetration = radius_sum - distance;

        const INV_DT: f32 = 60.0;
        let bias = BAUMGARTE * penetration * INV_DT;

        let j = -(vel_along_normal + bias) / denom;

        let impulse = normal * j;

        let jt = -vel_along_tangent / denom;
        let mu = (self.friction * b.friction).sqrt();

        let friction_impulse = if jt.abs() < j.abs() * mu {
            tangent * jt
        } else {
            tangent * -j * mu
        };

        let aim = self.inv_mass;
        let bim = b.inv_mass;

        self.velocity -= impulse * aim;
        b.velocity += impulse * bim;

        let aii = self.inv_inertia;
        let bii = b.inv_inertia;

        self.angular_velocity -= ra.cross(impulse) * aii;
        b.angular_velocity += rb.cross(impulse) * bii;

        self.velocity -= friction_impulse * aim;
        b.velocity += friction_impulse * bim;

        self.angular_velocity -= ra.cross(friction_impulse) * aii;
        b.angular_velocity += rb.cross(friction_impulse) * bii;

        let percent = 0.2;
        let correction = normal * penetration * percent * 0.5;

        self.position -= correction;
        b.position += correction;

        if self.velocity.length_squared() < 0.0001 {
            self.velocity = Vec3::ZERO;
        }
    }
}
