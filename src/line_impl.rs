use crate::framebuffer::Framebuffer;

pub trait Line {
    fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize);
}

impl Line for Framebuffer {
    fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        let mut x1 = x1;
        let mut y1 = y1;
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            self.point(x1, y1);
            if x1 == x2 && y1 == y2 { break; }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x1 += sx;
            }
            if e2 < dy {
                err += dx;
                y1 += sy;
            }
        }
    }
}
