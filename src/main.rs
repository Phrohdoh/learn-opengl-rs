#[macro_use]
extern crate glium;
use glium::Surface;

mod teapot;

fn calc_view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_title(String::from("LearnOpenGL"))
        .with_dimensions(600, 600)
        .with_depth_buffer(24)
        .build_glium()
        .expect("Failed to build glium window");

    let vs = r#"
        #version 330

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 view_matrix;
        uniform mat4 model_matrix;

        // Called once for each vertex in our geometry
        void main() {
            mat4 mv = view_matrix * model_matrix;
            v_normal = transpose(inverse(mat3(mv))) * normal;
            gl_Position = perspective * mv * vec4(position, 1.0);
        }
    "#;

    let fs = r#"
        #version 330

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        // Called once per pixel
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display,
                                          glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    let program = glium::Program::from_source(&display, vs, fs, None)
        .expect("Failed to link program");

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        // NOTE: Intentionally disabled because the teapot isn't a closed model.
        //       Left here so I don't forget about this.
        // backface_culling: glium::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let model_matrix = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 2.0, 1.0f32]
    ];

    let view_matrix = calc_view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

    let light = [-1.0, 0.4, 0.8f32];

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0,              0.0              , 0.0],
                [       0.0      ,  f ,              0.0              , 0.0],
                [       0.0      , 0.0,  (zfar+znear)/(zfar-znear)    , 1.0],
                [       0.0      , 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
            ]
        };

        target.draw((&positions, &normals),
                  &indices,
                  &program,
                  &uniform! {
                    model_matrix: model_matrix,
                    view_matrix: view_matrix,
                    u_light: light,
                    perspective: perspective,
                  },
                  &params)
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
