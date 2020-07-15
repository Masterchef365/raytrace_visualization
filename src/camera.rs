use nalgebra::{Matrix4, Point3, Vector3, Vector4};
use crate::ray::Ray;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub eye: Point3<f32>,
    pub at: Point3<f32>,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn matrix(&self) -> Matrix4<f32> {
        let aspect = self.width as f32 / self.height as f32;
        Matrix4::new_perspective(aspect, self.fov, self.near, self.far)
            * Matrix4::face_towards(&self.eye, &self.at, &Vector3::new(0.0, 1.0, 0.0))
    }

    pub fn pixel_tl(&self, x: u32, y: u32, mag: f32) -> Point3<f32> {
        let inv = self.matrix().try_inverse().unwrap();
        let x = x as f32;
        let y = y as f32;
        let width = self.width as f32;
        let height = self.height as f32;
        let v = Vector4::new(
            ((x / width) * 2.0) - 1.0,
            (((height - y) / height) * 2.0) - 1.0,
            mag,
            1.0,
        );
        (inv * v).xyz().into()
    }

    pub fn ray(&self, x: u32, y: u32) -> Ray {
        Ray {
            origin: self.eye,
            direction: self.pixel_tl(x, y, 1.0) - self.eye,
        }
    }
}
