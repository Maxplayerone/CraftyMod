extern crate glfw;
use self::glfw::Context;
use std::sync::mpsc::Receiver;

extern crate gl;

mod renderer;
mod utils;
use crate::renderer::Renderer;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "Rust rotating cube!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_all_polling(true);
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    #[rustfmt::skip]
    //X-Y-Z
    let vertices: [f32; 120] = [
        -0.5, -0.5, -0.5,  0.0, 0.0, //left-bottom-back
         0.5, -0.5, -0.5,  1.0, 0.0, //right-bottom-back
         0.5,  0.5, -0.5,  1.0, 1.0, //right-top-back
        -0.5,  0.5, -0.5,  0.0, 1.0, //left-top-back

        -0.5, -0.5,  0.5,  0.0, 0.0, //left-bottom-front
         0.5, -0.5,  0.5,  1.0, 0.0, //right-bottom-front
         0.5,  0.5,  0.5,  1.0, 1.0, //right-top-front
        -0.5,  0.5,  0.5,  0.0, 1.0, //left-top-front

        -0.5,  0.5,  0.5,  1.0, 0.0, //left-top-front
        -0.5,  0.5, -0.5,  1.0, 1.0, //left-top-back
        -0.5, -0.5, -0.5,  0.0, 1.0, //left-bottom-back
        -0.5, -0.5,  0.5,  0.0, 0.0, //left-bottom-front

         0.5,  0.5,  0.5,  1.0, 0.0, //right-top-front
         0.5,  0.5, -0.5,  1.0, 1.0, //right-top-back
         0.5, -0.5, -0.5,  0.0, 1.0, //right-bottom-back
         0.5, -0.5,  0.5,  0.0, 0.0, //right-bottom-front

        -0.5, -0.5, -0.5,  0.0, 1.0, //left-bottom-back
         0.5, -0.5, -0.5,  1.0, 1.0, //right-bottom-back
         0.5, -0.5,  0.5,  1.0, 0.0, //right-bottom-front
        -0.5, -0.5,  0.5,  0.0, 0.0, //left-bottom-front

        -0.5,  0.5, -0.5,  0.0, 1.0, //left-top-back
         0.5,  0.5, -0.5,  1.0, 1.0, //right-top-back
         0.5,  0.5,  0.5,  1.0, 0.0, //right-top-front
        -0.5,  0.5,  0.5,  0.0, 0.0, //left-top-front
   ];

    
    #[rustfmt::skip]
    let indices: [u32; 36] = [
        0, 1, 2, 2, 3, 0,
        4, 5, 6, 6, 7, 4,
        8, 9, 10, 10, 11, 8,
        12, 13, 14, 14, 15, 12,
        16, 17, 18, 18, 19, 16, 
        20, 21, 22, 22, 23, 20,
    ];
    
    let mut renderer = Renderer::new().expect("Cannot create renderer");
    renderer.upload_vbo_data(&vertices);
    renderer.upload_ibo_data(&indices);

    renderer.set_vao_attrib(0, 3, 5, 0);
    renderer.set_vao_attrib(1, 2, 5, 3);

    let mut delta_time: f64 = 0.01;
    let mut last_frame: f64 = 0.0;
    while !window.should_close() {
        process_events(&mut window, &events, &mut renderer, delta_time);

        renderer.draw();

        let current_frame: f64 = glfw::Glfw::get_time(&glfw);
        delta_time = current_frame - last_frame;
        last_frame = current_frame;
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(
    _window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>,
    renderer: &mut Renderer,
    delta_time: f64,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            _ => {
                renderer.process_events(event, delta_time);
            }
        }
    }
}
