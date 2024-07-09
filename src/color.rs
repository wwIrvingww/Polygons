#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor que recibe valores RGB
    pub fn new(r: i32, g: i32, b: i32) -> Color {
        Color {
            r: Color::clamp(r),
            g: Color::clamp(g),
            b: Color::clamp(b),
        }
    }

    // Constructor que recibe un valor HEX
    pub fn from_hex(hex: &str) -> Result<Color, &'static str> {
        if hex.len() != 6 {
            return Err("Hex color must be 6 characters long.");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex value")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex value")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex value")?;

        Ok(Color { r, g, b })
    }

    // Clamping de valores RGB entre 0 y 255
    fn clamp(value: i32) -> u8 {
        if value < 0 {
            0
        } else if value > 255 {
            255
        } else {
            value as u8
        }
    }

    // Sumar dos colores sin sobrepasar el valor de 255
    pub fn add(&self, other: &Color) -> Color {
        Color {
            r: Color::clamp(self.r as i32 + other.r as i32),
            g: Color::clamp(self.g as i32 + other.g as i32),
            b: Color::clamp(self.b as i32 + other.b as i32),
        }
    }

    // Multiplicar un color por un número
    pub fn multiply(&self, scalar: f32) -> Color {
        Color {
            r: Color::clamp((self.r as f32 * scalar) as i32),
            g: Color::clamp((self.g as f32 * scalar) as i32),
            b: Color::clamp((self.b as f32 * scalar) as i32),
        }
    }
}

// Implementar el trait Display para la estructura Color
use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

// Test para verificar el comportamiento de la clase Color
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_colors() {
        let color1 = Color::new(200, 100, 50);
        let color2 = Color::new(60, 160, 220);
        let result = color1.add(&color2);
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 255);
        assert_eq!(result.b, 255);
    }

    #[test]
    fn test_multiply_color() {
        let color = Color::new(100, 50, 25);
        let result = color.multiply(2.0);
        assert_eq!(result.r, 200);
        assert_eq!(result.g, 100);
        assert_eq!(result.b, 50);
    }

    #[test]
    fn test_clamp_values() {
        let color = Color::new(300, -50, 255);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex("FF5733").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 87);
        assert_eq!(color.b, 51);
    }

    #[test]
    fn test_display() {
        let color = Color::new(255, 87, 51);
        assert_eq!(format!("{}", color), "Color(r: 255, g: 87, b: 51)");
    }
}