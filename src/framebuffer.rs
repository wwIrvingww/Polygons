use crate::color::Color;
use crate::bmp;
use nalgebra_glm as glm;

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
    background_color: Color,
    foreground_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let background_color = Color::new(0, 0, 0); // Default background color black
        let buffer = vec![background_color; width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            foreground_color: Color::new(255, 255, 255), // Default foreground color white
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.foreground_color;
        }
    }

    pub fn point_vertex(&mut self, vertex: &glm::Vec3) {
        let x = vertex.x.round() as isize;
        let y = vertex.y.round() as isize;
        self.point(x, y);
    }

    pub fn get_color(&self, x: isize, y: isize) -> Option<Color> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = Color::new(
            ((color >> 16) & 0xFF) as i32,
            ((color >> 8) & 0xFF) as i32,
            (color & 0xFF) as i32,
        );
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        self.foreground_color = Color::new(
            ((color >> 16) & 0xFF) as i32,
            ((color >> 8) & 0xFF) as i32,
            (color & 0xFF) as i32,
        );
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        bmp::write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }
}
