use crate::engine::Raycast;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Sphere {
    /// Returns the closest intersection point, if any
    pub fn intersect(&self, ray: &Ray) -> Option<Point3<f32>> {
        // Gaurd against reversed rays
        if (self.center - ray.origin).dot(&ray.direction) < 0.0 {
            return None;
        }

        // Calculate the intersections
        let o = ray.origin - self.center;
        let d = ray.direction;
        let (a, b) = solve_quadratic(
            d.dot(&d),
            2.0 * d.dot(&o),
            o.dot(&o) - self.radius.powf(2.0),
        )?;

        let along = |v: f32| ray.origin + ray.direction * v;
        let a = along(a);
        let b = along(b);

        // Choose the closest intersection
        if (a - ray.origin).magnitude_squared() < (b - ray.origin).magnitude_squared() {
            Some(a)
        } else {
            Some(b)
        }
    }

    /// The normal at this point. Doesn't have to be on the sphere, necessarily.
    pub fn normal(&self, pt: &Point3<f32>) -> Vector3<f32> {
        (self.center - pt).normalize()
    }
}

impl Raycast for Sphere {
    fn raycast(&self, ray: &Ray) -> Option<Ray> {
        self.intersect(ray)
            .map(|p| ray.reflect(&p, &self.normal(&p)))
    }
}

fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let inside = b.powf(2.0) - 4.0 * a * c;
    if inside < 0.0 {
        return None;
    }
    let rest = inside.sqrt();
    let a2 = a * 2.0;
    Some(((-b + rest) / a2, (-b - rest) / a2))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn within_tol(sample: Option<(f32, f32)>, example: Option<(f32, f32)>) {
        fn tol(sample: f32, example: f32) {
            assert!(sample - example < std::f32::EPSILON * 2.0);
        }

        match (sample, example) {
            (Some((a, b)), Some((c, d))) => {
                tol(a, c);
                tol(b, d);
            }
            (None, None) => (),
            _ => panic!("Mismatched fails"),
        }
    }

    #[test]
    fn test_solve_quadratic() {
        within_tol(solve_quadratic(6.0, -7.0, 3.0), None);
        within_tol(solve_quadratic(8.0, 2.0, 0.0), Some((0.0, -0.25)));
        within_tol(solve_quadratic(6.0, 3.0, 3.0), None);
        within_tol(
            solve_quadratic(-6.0, 3.0, 8.0),
            Some((-0.93145390656315, 1.4314539065632)),
        );
    }
}
