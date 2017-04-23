#[macro_use]
extern crate glium;
use glium::Surface;

mod teapot;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_title(String::from("LearnOpenGL"))
        .with_dimensions(600, 600)
        .build_glium()
        .expect("Failed to build glium window");

    let vs = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        // Called once for each vertex in our geometry
        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fs = r#"
        #version 140

        out vec4 color;

        // Called once per pixel
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();
    let program = glium::Program::from_source(&display, vs, fs, None).expect("Failed to link program");

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
        };

        target.draw((&positions, &normals), &indices, &program, &uniforms, &Default::default())
            .expect("Failed to draw");

        target.finish().expect("Failed to clear target");

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
