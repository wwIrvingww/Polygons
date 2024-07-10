use crate::framebuffer::Framebuffer;
use crate::line_impl::Line;
use nalgebra_glm as glm;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[glm::Vec3]);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, points: &[glm::Vec3]) {
        if points.len() < 3 {
            return; // Necesitamos al menos tres puntos para dibujar un polígono
        }

        // Dibuja los bordes del polígono
        for i in 0..points.len() {
            let start = points[i];
            let end = if i == points.len() - 1 {
                points[0] // Conectar el último punto con el primero
            } else {
                points[i + 1]
            };
            self.line(start, end);
        }

        // Rellena el polígono usando el algoritmo de escaneo de líneas
        self.fill_polygon(points);
    }
}

impl Framebuffer {
    fn fill_polygon(&mut self, points: &[glm::Vec3]) {
        let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min) as isize;
        let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max) as isize;

        for y in min_y..=max_y {
            let mut intersections = Vec::new();
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = if i == points.len() - 1 {
                    points[0]
                } else {
                    points[i + 1]
                };

                if (p1.y <= y as f32 && p2.y > y as f32) || (p2.y <= y as f32 && p1.y > y as f32) {
                    let t = (y as f32 - p1.y) / (p2.y - p1.y);
                    let x = p1.x + t * (p2.x - p1.x);
                    intersections.push(x);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x1 = intersections[i].round() as isize;
                    let x2 = intersections[i + 1].round() as isize;
                    for x in x1..=x2 {
                        self.point(x, y);
                    }
                }
            }
        }
    }
}
