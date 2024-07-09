use std::fs::File;
use std::io::{Write, BufWriter};

const BMP_HEADER_SIZE: usize = 14;
const DIB_HEADER_SIZE: usize = 40;
const BMP_PIXEL_OFFSET: usize = BMP_HEADER_SIZE + DIB_HEADER_SIZE;
const BMP_BITS_PER_PIXEL: usize = 24;

pub fn write_bmp_file(
    file_path: &str,         // Path to the output BMP file
    buffer: &[u32],          // Framebuffer pixel data
    width: usize,            // Width of the image
    height: usize,           // Height of the image
) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    write_bmp_header(&mut writer, width, height)?;
    write_pixel_data(&mut writer, buffer, width, height)?;

    Ok(())
}

fn write_bmp_header(
    writer: &mut BufWriter<File>, // Buffered writer for the file
    width: usize,                 // Width of the image
    height: usize,                // Height of the image
) -> std::io::Result<()> {
    let row_size = ((BMP_BITS_PER_PIXEL * width + 31) / 32) * 4;
    let pixel_data_size = row_size * height;
    let file_size = BMP_HEADER_SIZE + DIB_HEADER_SIZE + pixel_data_size;
    let reserved: u32 = 0;
    let offset: u32 = BMP_PIXEL_OFFSET as u32;

    // BMP Header
    writer.write_all(b"BM")?;
    writer.write_all(&(file_size as u32).to_le_bytes())?;
    writer.write_all(&reserved.to_le_bytes())?;
    writer.write_all(&offset.to_le_bytes())?;

    // DIB Header (BITMAPINFOHEADER)
    let dib_header_size: u32 = DIB_HEADER_SIZE as u32;
    let planes: u16 = 1;
    let bpp: u16 = BMP_BITS_PER_PIXEL as u16;
    let compression: u32 = 0;
    let image_size: u32 = pixel_data_size as u32;
    let resolution_x: i32 = 2835; // 72 DPI
    let resolution_y: i32 = 2835; // 72 DPI
    let colors_used: u32 = 0;
    let important_colors: u32 = 0;

    writer.write_all(&dib_header_size.to_le_bytes())?;
    writer.write_all(&(width as i32).to_le_bytes())?;
    writer.write_all(&(height as i32).to_le_bytes())?;
    writer.write_all(&planes.to_le_bytes())?;
    writer.write_all(&bpp.to_le_bytes())?;
    writer.write_all(&compression.to_le_bytes())?;
    writer.write_all(&image_size.to_le_bytes())?;
    writer.write_all(&resolution_x.to_le_bytes())?;
    writer.write_all(&resolution_y.to_le_bytes())?;
    writer.write_all(&colors_used.to_le_bytes())?;
    writer.write_all(&important_colors.to_le_bytes())?;

    Ok(())
}

fn write_pixel_data(
    writer: &mut BufWriter<File>, // Buffered writer for the file
    buffer: &[u32],               // Framebuffer pixel data
    width: usize,                 // Width of the image
    height: usize,                // Height of the image
) -> std::io::Result<()> {
    let row_padding = (4 - (width * 3 % 4)) % 4;
    let padding: [u8; 3] = [0, 0, 0]; // Padding bytes

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let blue = (pixel & 0xFF) as u8;
            let green = ((pixel >> 8) & 0xFF) as u8;
            let red = ((pixel >> 16) & 0xFF) as u8;
            writer.write_all(&[blue, green, red])?; // BMP uses BGR format
        }
        if row_padding > 0 {
            writer.write_all(&padding[..row_padding])?; // Add padding bytes if needed
        }
    }

    Ok(())
}
