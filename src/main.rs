mod color;
mod framebuffer;
mod bmp;

use framebuffer::Framebuffer;

fn main() -> std::io::Result<()> {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0xADD8E6); // Light blue
    framebuffer.clear();

    framebuffer.set_current_color(0xFF0000); // Red
    framebuffer.point(400, 300);
    framebuffer.point(401, 300);
    framebuffer.point(400, 301);
    framebuffer.point(401, 301);

    framebuffer.set_current_color(0x00FF00); // Green
    framebuffer.point(200, 150);
    framebuffer.point(201, 150);
    framebuffer.point(200, 151);
    framebuffer.point(201, 151);

    framebuffer.set_current_color(0x0000FF); // Blue
    framebuffer.point(600, 450);
    framebuffer.point(601, 450);
    framebuffer.point(600, 451);
    framebuffer.point(601, 451);

    framebuffer.render_buffer("output.bmp")?;

    println!("Framebuffer rendered to output.bmp");
    Ok(())
}
