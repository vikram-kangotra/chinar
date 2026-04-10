use crate::math::{
    mat4::{Mat4, look_at, perspective},
    vec3::Vec3,
};

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,

    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn forward(&self) -> Vec3 {
        Vec3 {
            x: self.yaw.cos() * self.pitch.cos(),
            y: self.pitch.sin(),
            z: self.yaw.sin() * self.pitch.cos(),
        }
        .normalize()
    }

    pub fn right(&self) -> Vec3 {
        Vec3::Y.cross(self.forward()).normalize()
    }

    pub fn build_vp_matrix(&self) -> Mat4 {
        let target = self.position + self.forward();
        let view = look_at(self.position, target, Vec3::Y);
        let proj = perspective(self.fov_y, self.aspect, self.z_near, self.z_far);

        proj * view
    }
}
