extern crate glfw;
use self::glfw::{Context, Key, Action, MouseButton};
use std::sync::mpsc::Receiver;

extern crate gl;
use self::gl::types::*;

mod utils;
mod renderer;
use crate::renderer::Renderer;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(800, 600, "Rust rotating cube!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_all_polling(true);
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    #[rustfmt::skip]
    let vertices: [f32; 180] = [
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
   ];

    /*
    #[rustfmt::skip]
    let indices: [u32; 6] = [
        2, 0, 1,
        2, 3, 1,
    ];
    */
    let renderer = Renderer::new().expect("Cannot create renderer");
    renderer.upload_vbo_data(&vertices);
    //renderer.upload_ibo_data(&indices);

    renderer.set_vao_attrib(0, 3, 5, 0);
    renderer.set_vao_attrib(1, 2, 5, 3);
    while !window.should_close() {
        process_events(&mut window, &events);

        renderer.draw();
        window.swap_buffers();
        glfw.poll_events();
    }
}


fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
for (_, event) in glfw::flush_messages(&events) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe {gl::Viewport(0, 0, width, height)}
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => println!("Works"),
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => println!("yep"),
        glfw::WindowEvent::Key(Key::H, _, Action::Press, _) => println!("It's really working"),
        glfw::WindowEvent::CursorPos(x, y) => println!("x: {}, y: {}", x, y),
        glfw::WindowEvent::Scroll(x, y) => println!("Scrolling on {} and {}", x, y),
        _ => {}
    }
}
}
