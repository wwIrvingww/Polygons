# Computer Graphics Project in Rust

This repository contains a set of Rust programs for drawing and filling polygons in a BMP image. 
Several functionalities are included such as drawing lines, filling polygons, and handling the writing of BMP files. 
This project is geared towards practicing computer graphics concepts.

## Project Structure

- `src/`
  - `color.rs`: Defines the `Color` structure to handle RGB colors.
  - framebuffer.rs`: Handles the framebuffer where pixels are drawn.
  - line_impl.rs`: Implements the function to draw lines in the framebuffer.
  - polygon_impl.rs`: Defines the structure and functions to handle polygons.
  - bmp.rs`: Implements the function to write BMP files.
  - `lab_1.rs`: Contains the functions to draw and fill the polygons specified in the laboratory.
  - `main.rs`: Entry point of the program, calls the functions of `lab_1.rs`.

## Project Execution

To compile and run the project, use the following command:

```bash
cargo run
```

## Notes
  - The generated BMP file will be in the root folder of the project with the name polygons_output.bmp.
  - The generated BMP file will be correctly oriented, after correcting the image inversion problem.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

