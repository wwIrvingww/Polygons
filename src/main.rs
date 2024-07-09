mod color;
mod framebuffer;
mod bmp;
mod line_impl;
mod polygon_impl;

use framebuffer::FrameBuffer;
use line_impl::Line;
use polygon_impl::Polygon;
use std::io::Result;

fn main() -> Result<()> {
    let mut framebuffer = FrameBuffer::new(800, 600);

    framebuffer.set_background_color(0xADD8E6);
    framebuffer.clear();

    framebuffer.set_current_color(0xFF0000);
    framebuffer.point(400, 300);
    framebuffer.point(401, 300);
    framebuffer.point(400, 301);
    framebuffer.point(401, 301);

    framebuffer.set_current_color(0x00FF00);
    framebuffer.point(200, 150);
    framebuffer.point(201, 150);
    framebuffer.point(200, 151);
    framebuffer.point(201, 151);

    framebuffer.set_current_color(0x0000FF);
    framebuffer.point(600, 450);
    framebuffer.point(601, 450);
    framebuffer.point(600, 451);
    framebuffer.point(601, 451);

    // Draw lines using Bresenham's algorithm
    framebuffer.set_current_color(0x000000); // Black
    framebuffer.line(100, 100, 700, 500);
    framebuffer.line(700, 100, 100, 500);
    framebuffer.line(400, 0, 400, 600);
    framebuffer.line(0, 300, 800, 300);

    // Draw polygon
    let points = vec![(100, 100), (700, 100), (700, 500), (100, 500)];
    framebuffer.set_current_color(0xFF00FF); // Magenta
    framebuffer.draw_polygon(&points);

    framebuffer.render_buffer("output.bmp")?;

    println!("Framebuffer rendered to output.bmp");
    Ok(())
}
