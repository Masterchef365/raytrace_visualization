use kiss3d::window::Window;
use nalgebra::{Point3, Vector3};
use visible_raytrace::{
    animation::RayAnimation, camera::Camera, engine, path::Path, plane::Plane, sphere::Sphere,
};

fn draw_sphere(window: &mut Window, sphere: &Sphere, rows: u32, cols: u32, color: &Point3<f32>) {
    let pos = |row: u32, col: u32| {
        let row = row as f32 * std::f32::consts::PI / rows as f32;
        let row = row - (std::f32::consts::PI / 2.0);
        let col = col as f32 * (std::f32::consts::PI * 2.0) / cols as f32;
        sphere.center + Vector3::new(row.cos() * col.cos(), row.sin(), row.cos() * col.sin())
    };

    for row in 0..rows {
        let mut last = pos(row, 0);
        for col in 1..=cols {
            let cur = pos(row, col);
            window.draw_line(&cur, &last, color);
            last = cur;
        }
    }

    for col in 0..cols {
        let mut last = pos(0, col);
        for row in 1..=rows {
            let cur = pos(row, col);
            window.draw_line(&cur, &last, color);
            last = cur;
        }
    }
}

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

fn main() {
    let mut window = Window::new("Slow light ray trace");
    let white = Point3::new(1.0, 1.0, 1.0);
    let green = Point3::new(0.4, 1.0, 0.0);

    let cam = Camera {
        width: 80,
        height: 80,
        eye: Point3::origin(),
        at: Point3::new(0.0, 0.0, -10.0),
        fov: 0.3,
        near: 0.1,
        far: 1000.0,
    };

    let mut scene = engine::Scene::new();

    let sphere = Sphere {
        center: Point3::new(0.0, -1.0, 3.0),
        radius: 1.0,
    };

    let plane = Plane {
        origin: Point3::new(0.0, 0.0, 5.0),
        normal: Vector3::new(0.0, 1.0, -2.5),
    };

    scene.push(Box::new(sphere.clone()));
    scene.push(Box::new(plane.clone()));

    let mut animations = engine::trace_scene(&scene, &cam, 20)
        .into_iter()
        .map(|path| RayAnimation::new(path, 0.5))
        .collect::<Vec<_>>();

    while window.render() {
        draw_camera(&mut window, &cam, 0.1, &white);
        draw_sphere(&mut window, &sphere, 20, 68, &white);
        draw_plane(&mut window, &plane, 5.0, &white);
        let mut reset = true;
        for anim in &mut animations {
            anim.draw(&mut window, &green);
            if anim.step(0.01) {
                reset = false;
            }
        }

        if reset {
            for anim in &mut animations {
                anim.reset();
            }
        }
    }
}
