use nalgebra::Point3;

pub type Line = (Point3<f32>, Point3<f32>);

#[derive(Debug, Clone)]
struct PathEntry {
    position: f32,
    length: f32,
}

/// Implements odometry-based position queries with O(n) memory and O(log n) query time.
#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Point3<f32>>,
    lines: Vec<PathEntry>,
    length: f32,
}

impl Path {
    pub fn new(points: Vec<Point3<f32>>) -> Self {
        let mut lines = Vec::with_capacity(points.len() - 1);
        let mut position = 0.0;

        for line in points.windows(2) {
            let length = (line[1] - line[0]).magnitude();
            lines.push(PathEntry { position, length });
            position += length;
        }

        Self {
            points,
            lines,
            length: position,
        }
    }

    pub fn lines_between(&self, begin: f32, end: f32) -> Option<Vec<Line>> {
        if begin > end {
            return None;
        }

        let begin_idx = self.nearest_line_idx(begin)?;
        let end_idx = self.nearest_line_idx(end)?;

        let begin_pos = self.point_at_position(begin)?;
        let end_pos = self.point_at_position(end)?;

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

    pub fn point_at_position(&self, position: f32) -> Option<Point3<f32>> {
        let line_idx = self.nearest_line_idx(position)?;
        let line = &self.lines[line_idx];
        let off = (position - line.position) / line.length;
        let pt = self.points[line_idx]
            .coords
            .lerp(&self.points[line_idx + 1].coords, off);
        Some(pt.into())
    }

    /// Get the index of the point that begins the line to this index. Returns None if there is no
    /// corresponding line. It's a binary search underneath, so it should be O(log n)
    pub fn nearest_line_idx(&self, position: f32) -> Option<usize> {
        if position > self.length || self.lines.is_empty() {
            return None;
        }

        let mut a = 0;
        let mut b = self.points.len() - 1;
        let mut current_idx;
        loop {
            current_idx = (a + b) / 2;

            let current_line = &self.lines[current_idx];
            let begin = current_line.position;
            let end = begin + current_line.length;

            if position > end {
                a = current_idx + 1;
            } else if position < begin {
                b = current_idx;
            } else {
                break Some(current_idx);
            }
        }
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn points(&self) -> &[Point3<f32>] {
        &self.points
    }
}
