mod color;
mod framebuffer;
mod line_impl;
mod polygon_impl;
mod bmp;
mod lab_1;
use nalgebra_glm as glm;
use framebuffer::Framebuffer;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

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

    // Llama a la función draw_fill_polygon para dibujar y rellenar el polígono 1
    lab_1::draw_fill_polygon(&mut framebuffer, &polygon1_points, 0xFFFFFF, 0xFFFF00);

    // Renderiza el framebuffer en un archivo BMP
    framebuffer.render_buffer("polygon1_output.bmp").unwrap();
}
