use crate::framebuffer::Framebuffer;
use crate::line_impl::Line;
use nalgebra_glm as glm;

pub fn draw_fill_polygon(framebuffer: &mut Framebuffer, points: &[glm::Vec2], border_color: u32, fill_color: u32) {
    if points.len() < 3 {
        return; // Necesitamos al menos tres puntos para dibujar un polígono
    }

    // Dibuja los bordes del polígono con un grosor mayor
    for i in -1..=1 {
        for j in -1..=1 {
            framebuffer.set_foreground_color(border_color);
            for k in 0..points.len() {
                let start = glm::vec3(points[k].x + i as f32, points[k].y + j as f32, 0.0);
                let end = if k == points.len() - 1 {
                    glm::vec3(points[0].x + i as f32, points[0].y + j as f32, 0.0)
                } else {
                    glm::vec3(points[k + 1].x + i as f32, points[k + 1].y + j as f32, 0.0)
                };
                framebuffer.line(start, end);
            }
        }
    }

    // Rellena el polígono usando el algoritmo de escaneo de líneas
    fill_polygon(framebuffer, points, fill_color);
}

fn fill_polygon(framebuffer: &mut Framebuffer, points: &[glm::Vec2], fill_color: u32) {
    let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min) as isize;
    let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max) as isize;

    framebuffer.set_foreground_color(fill_color);
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
                    framebuffer.point(x, y);
                }
            }
        }
    }
}
