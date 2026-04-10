use std::ops::Mul;

use crate::math::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mat4 {
    pub fn identity() -> Self {
        Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let f = (target - eye).normalize();
    let s = f.cross(up).normalize();
    let u = s.cross(f);

    Mat4([
        [s.x, s.y, s.z, 0.0],
        [u.x, u.y, u.z, 0.0],
        [-f.x, -f.y, -f.z, 0.0],
        [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0],
    ])
}

pub fn perspective(fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
    let f = 1.0 / (fovy * 0.5).tan();

    Mat4([
        [f / aspect, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, zfar / (znear - zfar), -1.0],
        [0.0, 0.0, (znear * zfar) / (znear - zfar), 0.0],
    ])
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.0[k][j] * rhs.0[i][k];
                }
            }
        }

        Mat4(result)
    }
}
