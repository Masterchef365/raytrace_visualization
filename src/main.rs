use nalgebra::{Point3, Vector3};
use rand::distributions::{Distribution, Uniform};

type Line = (Point3<f32>, Point3<f32>);

struct PathLine {
    dist_from_begin: f32,
    length: f32,
}

struct Path {
    points: Vec<Point3<f32>>,
    lines: Vec<PathLine>,
    total_length: f32,
}

impl Path {
    pub fn new(points: Vec<Point3<f32>>) -> Self {
        let mut lines = Vec::with_capacity(points.len() - 1);
        let mut dist_from_begin = 0.0;

        for pair in points.windows(2) {
            let length = (pair[1] - pair[0]).magnitude();
            lines.push(PathLine {
                dist_from_begin,
                length,
            });
            dist_from_begin += length;
        }

        Self {
            points,
            lines,
            total_length: dist_from_begin,
        }
    }

    pub fn lines_between(&self, begin: f32, end: f32) -> Option<Vec<Line>> {
        assert!(begin < end);

        let begin_idx = self.get_index(begin)?;
        let end_idx = self.get_index(end)?;

        let begin_pos = self.point_at_dist(begin)?;
        let end_pos = self.point_at_dist(end)?;

        if begin_idx == end_idx {
            Some(vec![(begin_pos, end_pos)])
        } else {
            let mut lines = Vec::new();
            for idx in begin_idx + 1..end_idx {
                lines.push((self.points[idx], self.points[idx + 1]));
            }
            lines.push((begin_pos, self.points[begin_idx + 1]));
            lines.push((self.points[end_idx], end_pos));
            Some(lines)
        }
    }

    pub fn length(&self) -> f32 {
        self.total_length
    }

    pub fn point_at_dist(&self, distance: f32) -> Option<Point3<f32>> {
        let index = self.get_index(distance)?;
        let line = &self.lines[index];
        let off = (distance - line.dist_from_begin) / line.length;
        let pt = self.points[index]
            .coords
            .lerp(&self.points[index + 1].coords, off);
        Some(pt.into())
    }

    /// Get the index of the point that begins the line to this index. Returns None if there is no
    /// corresponding line.
    pub fn get_index(&self, distance: f32) -> Option<usize> {
        if distance > self.total_length {
            return None;
        }

        let mut a = 0;
        let mut b = self.points.len() - 1;
        let mut current = (a + b) / 2;
        loop {
            let current_line = &self.lines[current];
            let begin = current_line.dist_from_begin;
            let end = begin + current_line.length;
            if distance > end {
                a = current + 1;
                current = (a + b) / 2;
            } else if distance < begin {
                b = current;
                current = (a + b) / 2;
            } else {
                break Some(current);
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut window = kiss3d::window::Window::new("Slow light ray trace");
    let step = 0.1;
    let delta = Uniform::new(-step, step);

    let steps = 300;
    let mut path = Vec::with_capacity(steps);
    let mut position = Point3::origin();
    for _ in 0..steps {
        path.push(position);
        position += Vector3::new(
            delta.sample(&mut rng),
            delta.sample(&mut rng),
            delta.sample(&mut rng),
        );
    }

    let path = Path::new(path);

    let mut begin = 0.0;
    while window.render() {
        begin += 0.01;

        if begin > path.length() {
            begin = 0.0;
        }

        for pair in path.points.windows(2) {
            window.draw_line(&pair[0], &pair[1], &Point3::new(0.3, 0.3, 0.3));
        }

        let mut end = begin + 5.0;
        if end > path.length() {
            end = path.length();
        }

        if let Some(lines) = path.lines_between(begin, end) {
            for (a, b) in lines {
                window.draw_line(&a, &b, &Point3::new(0.0, 1.0, 0.0));
            }
        }

        /*
        if let Some(pt) = path.point_at_dist(t) {
            window.draw_point(&pt, &Point3::new(0.0, 1.0, 0.0));
            //window.draw_line(&pt, &Point3::origin(), &Point3::new(0.0, 1.0, 0.0));
        }
        */
    }
}
