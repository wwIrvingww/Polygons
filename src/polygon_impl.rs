use crate::framebuffer::Framebuffer;
use crate::line_impl::Line;
use nalgebra_glm as glm;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[glm::Vec3]);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, points: &[glm::Vec3]) {
        if points.len() < 2 {
            return; // Necesitamos al menos dos puntos para dibujar una línea
        }

        for i in 0..points.len() {
            let start = points[i];
            let end = if i == points.len() - 1 {
                points[0] // Conectar el último punto con el primero
            } else {
                points[i + 1]
            };

            // Dibuja la línea entre los puntos start y end
            self.line(start, end);
        }
    }
}
