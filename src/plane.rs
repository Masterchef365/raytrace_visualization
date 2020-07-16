use crate::engine::Raycast;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub origin: Point3<f32>,
    pub normal: Vector3<f32>,
}

impl Plane {
    pub fn facing(&self, pt: &Point3<f32>) -> bool {
        (*pt - self.origin).dot(&self.normal) > 0.0
    }

    pub fn project(&self, pt: &Point3<f32>) -> Point3<f32> {
        *pt - self.normal * self.distance(pt)
    }

    pub fn distance(&self, pt: &Point3<f32>) -> f32 {
        self.normal.dot(&(*pt - self.origin))
    }

    pub fn normal(&self) -> Vector3<f32> {
        self.normal.normalize()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Point3<f32>> {
        let diff = ray.origin - self.origin;
        let above = diff.dot(&self.normal) > 0.0;
        let with = ray.direction.dot(&self.normal) > 0.0;
        if above && !with {
            let l = -self.normal.dot(&diff) / (self.normal.dot(&ray.direction));
            Some(ray.origin + ray.direction * l)
        } else {
            None
        }
    }
}

impl Raycast for Plane {
    fn raycast(&self, ray: &Ray) -> Option<Ray> {
        let intersection = self.intersect(ray)?;
        Some(ray.reflect(&intersection, &self.normal()))
    }
}
