extern crate glfw;
use self::glfw::Context;
use std::sync::mpsc::Receiver;

extern crate gl;

mod renderer;
mod utils;
use crate::renderer::Renderer;
use crate::utils::math;

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
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut renderer = Renderer::new().expect("Cannot create renderer");

    renderer.load_cubes(
        math::Vec3::new(0.0, 0.0, 0.0),
        math::Vec3::new(16.0, 16.0, 16.0),
    );

    renderer.load_crosshair();

    let mut delta_time: f64 = 0.01;
    let mut last_frame: f64 = 0.0;

    unsafe{
    //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }
    while !window.should_close() {
        process_input(&mut window, &mut renderer, delta_time);
        process_events(&events, &mut renderer, delta_time);

        renderer.clear_screen();
        //renderer.draw_crosshair();
        renderer.draw_cubes();

        let current_frame: f64 = glfw::Glfw::get_time(&glfw);
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut glfw::Window, renderer: &mut Renderer, delta_time: f64) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true)
    } else {
        renderer.process_input(&window, delta_time);
    }
}

fn process_events(
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
