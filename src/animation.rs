use crate::path::Path;
use kiss3d::window::Window;
use nalgebra::Point3;

pub struct RayAnimation {
    path: Path,
    line_len: f32,
    beginning: f32,
}

impl RayAnimation {
    pub fn new(path: Path, line_len: f32) -> Self {
        RayAnimation {
            path,
            line_len,
            beginning: 0.0,
        }
    }

    /// Return false when finished
    pub fn step(&mut self, distance: f32) -> bool {
        if self.beginning >= self.path.length() + self.line_len {
            false
        } else {
            self.beginning += distance;
            true
        }
    }

    pub fn reset(&mut self) {
        self.beginning = 0.0;
    }

    pub fn draw(&self, window: &mut Window, color: &Point3<f32>) {
        let begin = if self.beginning > self.path.length() {
            self.path.length()
        } else {
            self.beginning
        };

        let candidate = self.beginning - self.line_len;
        let end = if candidate < 0.0 { 0.0 } else { candidate };

        if let Some(lines) = self.path.lines_between(end, begin) {
            for (a, b) in &lines {
                window.draw_line(&a, &b, color);
            }
        }
    }
}
