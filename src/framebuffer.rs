use crate::bmp::write_bmp_file;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let buffer = vec![0; width * height];
        FrameBuffer {
            width,
            height,
            buffer,
            background_color: 0xFFFFFF, // Default background color is white
            current_color: 0x000000, // Default current color is black
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            self.buffer[y as usize * self.width + x as usize] = self.current_color;
        }
    }

    pub fn get_color(&self, x: isize, y: isize) -> Option<u32> {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            Some(self.buffer[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }
}
