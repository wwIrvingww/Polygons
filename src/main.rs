mod color;
mod framebuffer;
mod line_impl;
mod polygon_impl;
mod bmp;
use nalgebra_glm as glm;
use framebuffer::Framebuffer;
use line_impl::Line;
use polygon_impl::Polygon;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    // Define los vértices del cubo
    let vertices = vec![
        glm::vec3(0.5, 0.5, 0.5),
        glm::vec3(0.5, 0.5, -0.5),
        glm::vec3(0.5, -0.5, 0.5),
        glm::vec3(0.5, -0.5, -0.5),
        glm::vec3(-0.5, 0.5, 0.5),
        glm::vec3(-0.5, 0.5, -0.5),
        glm::vec3(-0.5, -0.5, 0.5),
        glm::vec3(-0.5, -0.5, -0.5),
    ];

    // Define la matriz de rotación (rotación alrededor de los ejes X e Y)
    let rotation_matrix_x = glm::rotation(30.0_f32.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
    let rotation_matrix_y = glm::rotation(30.0_f32.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
    let rotation_matrix = rotation_matrix_y * rotation_matrix_x;

    // Crear representación homogénea
    let homogeneous_vertices: Vec<glm::Vec4> = vertices.iter()
        .map(|v| glm::vec4(v.x, v.y, v.z, 1.0))
        .collect();

    // Aplica la rotación
    let rotated_vertices: Vec<glm::Vec4> = homogeneous_vertices.iter()
        .map(|v| rotation_matrix * v)
        .collect();

    // Extrae posición para la proyección
    let projected_vertices: Vec<glm::Vec3> = rotated_vertices.iter()
        .map(|v| glm::vec3(v.x / v.w, v.y / v.w, v.z / v.w))
        .collect();

    // Escalamos y trasladamos los vértices para que sean visibles en la pantalla
    let scale = 200.0;
    let translate = glm::vec3(400.0, 300.0, 0.0);

    let final_vertices: Vec<glm::Vec3> = projected_vertices.iter()
        .map(|v| glm::vec3(
            v.x * scale + translate.x,
            v.y * scale + translate.y,
            v.z * scale + translate.z
        ))
        .collect();

    // Define los colores para las líneas del cubo
    let colors = vec![
        0xFF0000, // Rojo
        0x00FF00, // Verde
        0x0000FF, // Azul
        0xFFFF00, // Amarillo
        0xFF00FF, // Magenta
        0x00FFFF, // Cian
    ];

    // Dibujamos las líneas que conectan los vértices y rellenamos las caras del cubo
    let faces = vec![
        vec![0, 1, 3, 2], // Front face
        vec![4, 5, 7, 6], // Back face
        vec![0, 1, 5, 4], // Top face
        vec![2, 3, 7, 6], // Bottom face
        vec![0, 2, 6, 4], // Left face
        vec![1, 3, 7, 5], // Right face
    ];

    for (i, face) in faces.iter().enumerate() {
        framebuffer.set_foreground_color(colors[i % colors.len()]);
        let face_vertices: Vec<glm::Vec3> = face.iter().map(|&index| final_vertices[index]).collect();
        framebuffer.draw_polygon(&face_vertices);
    }

    // Renderiza el framebuffer en un archivo BMP
    framebuffer.render_buffer("cube_output.bmp").unwrap();
}
