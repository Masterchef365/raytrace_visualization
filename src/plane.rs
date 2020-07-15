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

    pub fn intersect(&self, ray: &Ray) -> Option<Point3<f32>> {
        let denom = (ray.origin.coords + self.origin.coords).dot(&self.normal);
        let numer = ray.direction.dot(&self.normal);
        let l = denom / numer;
        Some(ray.origin + ray.direction * l)
        /*
        let above = (ray.origin - self.origin).dot(&ray.direction);
        let with = ray.direction.dot(&self.normal) > 0.0;
        if above != with {
        } else {
            None
        }
                */
    }
}
