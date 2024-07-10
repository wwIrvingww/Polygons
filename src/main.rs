mod color;
mod framebuffer;
mod line_impl;
mod polygon_impl;
mod bmp;
mod lab_1;
use framebuffer::Framebuffer;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    // Llama a la función draw_polygons para dibujar y rellenar ambos polígonos
    lab_1::draw_polygons(&mut framebuffer);

    // Renderiza el framebuffer en un archivo BMP
    framebuffer.render_buffer("polygons_output.bmp").unwrap();
}
