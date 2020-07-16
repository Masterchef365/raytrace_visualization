use crate::ray::Ray;

pub trait Raycast {
    fn raycast(&self, ray: &Ray) -> Option<Ray>;
}
