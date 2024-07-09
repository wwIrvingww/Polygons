use crate::framebuffer::FrameBuffer;
use crate::line_impl::Line;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[(isize, isize)]);
}

impl Polygon for FrameBuffer {
    fn draw_polygon(&mut self, points: &[(isize, isize)]) {
        if points.len() < 2 {
            return;
        }

        for i in 0..points.len() - 1 {
            let (x1, y1) = points[i];
            let (x2, y2) = points[i + 1];
            self.line(x1, y1, x2, y2);
        }

        // Draw line from last point to the first point
        let (x1, y1) = points[points.len() - 1];
        let (x2, y2) = points[0];
        self.line(x1, y1, x2, y2);
    }
}
