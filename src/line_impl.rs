use crate::framebuffer::Framebuffer;
use nalgebra_glm as glm;

pub trait Line {
    fn line(&mut self, start: glm::Vec3, end: glm::Vec3);
}

impl Line for Framebuffer {
    fn line(&mut self, start: glm::Vec3, end: glm::Vec3) {
        let mut x1 = start.x.round() as isize;
        let mut y1 = start.y.round() as isize;
        let x2 = end.x.round() as isize;
        let y2 = end.y.round() as isize;
        
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
