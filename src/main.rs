use kiss3d::window::Window;
use nalgebra::{Matrix4, Point3, Vector3, Vector4};
use rand::distributions::{Distribution, Uniform};
use visible_raytrace::{camera::Camera, plane::Plane};


fn draw_plane(window: &mut Window, plane: &Plane, size: f32, color: &Point3<f32>) {
    let cross_x = plane.normal.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize() * size;
    let cross_y = plane.normal.cross(&cross_x).normalize() * size;

    let tl = plane.origin + cross_x;
    let tr = plane.origin - cross_y;
    let bl = plane.origin + cross_y;
    let br = plane.origin - cross_x;

    window.draw_line(&plane.origin, &(plane.origin + plane.normal * size), color);
    window.draw_line(&tl, &tr, color);
    window.draw_line(&tr, &br, color);
    window.draw_line(&br, &bl, color);
    window.draw_line(&bl, &tl, color);
}

fn draw_camera(window: &mut Window, camera: &Camera, size: f32, color: &Point3<f32>) {
    let tl = camera.pixel_tl(0, 0, size);
    let tr = camera.pixel_tl(camera.width, 0, size);
    let bl = camera.pixel_tl(0, camera.height, size);
    let br = camera.pixel_tl(camera.width, camera.height, size);

    window.draw_line(&tl, &tr, color);
    window.draw_line(&tr, &br, color);
    window.draw_line(&br, &bl, color);
    window.draw_line(&bl, &tl, color);

    window.draw_line(&camera.eye, &tl, color);
    window.draw_line(&camera.eye, &tr, color);
    window.draw_line(&camera.eye, &br, color);
    window.draw_line(&camera.eye, &bl, color);
}

fn reflect(ray: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    ray - 2.0 * normal * normal.dot(&ray)
}

fn main() {
    let mut window = Window::new("Slow light ray trace");
    let white = Point3::new(1.0, 1.0, 1.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let red = Point3::new(1.0, 0.0, 0.0);

    let cam = Camera {
        width: 6,
        height: 6,
        eye: Point3::origin(),
        at: Point3::new(0.0, 0.0, -10.0),
        fov: 0.3,
        near: 0.1,
        far: 1000.0,
    };

    let plane = Plane {
        origin: Point3::new(0.0, 0.0, 5.0),
        normal: Vector3::new(0.3, 1.0, -1.0),
    };

    while window.render() {
        draw_camera(&mut window, &cam, 0.1, &white);
        draw_plane(&mut window, &plane, 1.0, &white);
        for y in 1..cam.height {
            for x in 1..cam.width {
                let ray = cam.ray(x, y);
                let intersect = plane.intersect(&ray).unwrap();
                window.draw_line(&intersect, &cam.eye, &green);
                let away = intersect + reflect(&ray.direction, &plane.normal);
                window.draw_line(&intersect, &away, &red);
            }
        }
    }
}
