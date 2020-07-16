use crate::camera::Camera;
use crate::path::Path;
use crate::ray::Ray;

pub type Scene = Vec<Box<dyn Raycast>>;

pub fn trace_scene(scene: &Scene, camera: &Camera, max_bounces: usize) -> Vec<Path> {
    let mut paths = Vec::with_capacity((camera.width as usize - 1) * (camera.height as usize - 1));
    for y in 1..camera.height {
        for x in 1..camera.width {
            let ray = camera.ray(x, y);
            paths.push(trace_ray(&scene, &ray, max_bounces));
        }
    }
    paths
}

fn trace_ray(scene: &Scene, ray: &Ray, max_bounces: usize) -> Path {
    let mut points = Vec::new();
    let mut cur_ray = *ray;
    for _ in 0..max_bounces {
        points.push(cur_ray.origin);
        if let Some(ray) = intersect_scene(scene, &cur_ray) {
            cur_ray = ray;
        } else {
            break;
        }
    }
    points.push(cur_ray.origin + cur_ray.direction);
    Path::new(points)
}

fn intersect_scene(scene: &Scene, ray: &Ray) -> Option<Ray> {
    let mut best = None;
    let mut best_mag = std::f32::INFINITY;
    for object in scene {
        if let Some(candidate) = object.raycast(ray) {
            let dist = (ray.origin - candidate.origin).magnitude_squared();
            if dist < best_mag {
                best_mag = dist;
                best = Some(candidate);
            }
        }
    }
    best
}

pub trait Raycast {
    fn raycast(&self, ray: &Ray) -> Option<Ray>;
}
