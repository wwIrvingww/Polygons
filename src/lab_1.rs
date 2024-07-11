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

pub fn draw_polygons(framebuffer: &mut Framebuffer) {
    // Define los puntos del polígono 1
    let polygon1_points = vec![
        glm::vec2(165.0, 380.0),
        glm::vec2(185.0, 360.0),
        glm::vec2(180.0, 330.0),
        glm::vec2(207.0, 345.0),
        glm::vec2(233.0, 330.0),
        glm::vec2(230.0, 360.0),
        glm::vec2(250.0, 380.0),
        glm::vec2(220.0, 385.0),
        glm::vec2(205.0, 410.0),
        glm::vec2(193.0, 383.0),
    ];

    // Define los puntos del polígono 2
    let polygon2_points = vec![
        glm::vec2(321.0, 335.0),
        glm::vec2(288.0, 286.0),
        glm::vec2(339.0, 251.0),
        glm::vec2(374.0, 302.0),
    ];

    // Define los puntos del polígono 3
    let polygon3_points = vec![
        glm::vec2(377.0, 249.0),
        glm::vec2(411.0, 197.0),
        glm::vec2(436.0, 249.0),
    ];

    // Define los puntos del polígono 4
    let polygon4_points = vec![
        glm::vec2(413.0, 177.0),
        glm::vec2(448.0, 159.0),
        glm::vec2(502.0, 88.0),
        glm::vec2(553.0, 53.0),
        glm::vec2(535.0, 36.0),
        glm::vec2(676.0, 37.0),
        glm::vec2(660.0, 52.0),
        glm::vec2(750.0, 145.0),
        glm::vec2(761.0, 179.0),
        glm::vec2(672.0, 192.0),
        glm::vec2(659.0, 214.0),
        glm::vec2(615.0, 214.0),
        glm::vec2(632.0, 230.0),
        glm::vec2(580.0, 230.0),
        glm::vec2(597.0, 215.0),
        glm::vec2(552.0, 214.0),
        glm::vec2(517.0, 144.0),
        glm::vec2(466.0, 180.0),
    ];

    // Define los puntos del polígono 5
    let polygon5_points = vec![
        glm::vec2(682.0, 175.0),
        glm::vec2(708.0, 120.0),
        glm::vec2(735.0, 148.0),
        glm::vec2(739.0, 170.0),
    ];

    // Dibuja y rellena el polígono 1
    draw_fill_polygon(framebuffer, &polygon1_points, 0xFFFFFF, 0xFFFF00);
    
    // Dibuja y rellena el polígono 2
    draw_fill_polygon(framebuffer, &polygon2_points, 0xFFFFFF, 0x0000FF);

    // Dibuja y rellena el polígono 3
    draw_fill_polygon(framebuffer, &polygon3_points, 0xFFFFFF, 0xFF0000);

    // Dibuja y rellena el polígono 4
    draw_fill_polygon(framebuffer, &polygon4_points, 0xFFFFFF, 0x00FF00);

    // Dibuja el agujero (polígono 5)
    framebuffer.set_foreground_color(0x000000); // Negro para el fondo
    fill_polygon(framebuffer, &polygon5_points, 0x000000); // Rellenar el polígono 5 con negro
    draw_fill_polygon(framebuffer, &polygon5_points, 0xFFFFFF, 0x000000); // Dibuja solo el borde blanco del polígono 5
}
