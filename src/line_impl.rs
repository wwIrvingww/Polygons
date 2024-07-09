use crate::framebuffer::FrameBuffer;

pub trait Line {
    fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize);
}

impl Line for FrameBuffer {
    fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.point(x, y);

            if x == x2 && y == y2 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}
