use std::io::Cursor;


#[macro_use]
extern crate glium;
use glium::Surface;

extern crate image;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex: [f32; 2],
}

implement_vertex!(Vertex, position, tex);

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_title(String::from("LearnOpenGL"))
        .build_glium()
        .expect("Failed to build glium window");

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex;
        out vec2 v_tex;

        uniform mat4 matrix;

        // Called once for each vertex in our geometry
        void main() {
            v_tex = tex;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex;
        out vec4 color;

        uniform sampler2D tex;

        // Called once per pixel
        void main() {
            color = texture(tex, v_tex);
        }
    "#;

    let bl = Vertex { position: [-0.5, -0.5], tex: [0.0, 0.0], };
    let top = Vertex { position: [0.0, 0.5], tex: [0.0, 1.0], };
    let br = Vertex { position: [0.5, -0.5], tex: [1.0, 0.0], };
    let shape = vec![bl, top, br];

    let vbo = glium::VertexBuffer::new(&display, &shape).expect("Failed to create vbo");
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .expect("Failed to link program");

    let image = image::load(Cursor::new(&include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tree.png"))[..]), image::PNG)
        .expect("Failed to read png")
        .to_rgba();

    let dims = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), dims);
    let tex = glium::texture::Texture2d::new(&display, raw_image).expect("Failed to create texture");

    let mut t: f32 = -0.5;

    loop {
        t += 0.0002;

        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ],
            tex: &tex
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vbo, &indices, &program, &uniforms, &Default::default())
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
