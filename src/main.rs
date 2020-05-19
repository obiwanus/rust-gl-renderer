// ==================================== Crates and modules =======================================

extern crate nalgebra_glm as glm;

mod camera;
mod texture;

// ==================================== Imports ==================================================

use std::error::Error;
use std::f32::consts::PI;
use std::time::SystemTime;

use gl;
use sdl2;
use sdl2::keyboard::Scancode;

use camera::Camera;
use camera::Movement::*;

use texture::Texture;

// ==================================== Functions ================================================

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Open window
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    gl_attr.set_depth_size(16);
    gl_attr.set_double_buffer(true);

    let window = video_subsystem
        .window("Game 2", 1024, 768)
        .opengl()
        // .fullscreen_desktop()
        .build()?;
    let (window_width, window_height) = window.size();

    sdl.mouse().set_relative_mouse_mode(true);

    // Set up OpenGL
    let _gl_context = window.gl_create_context()?;
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
        gl::ClearColor(0.05, 0.05, 0.05, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    // Set up camera
    let mut camera = Camera::new();
    camera.aspect_ratio = (window_width as f32) / (window_height as f32);
    camera.position = glm::vec3(0.0, 2.0, 5.0);
    camera.look_at(glm::vec3(0.0, 2.0, 0.0));

    // TMP Cube
    #[rustfmt::skip]
    let cube_vertices: Vec<f32> = vec![
        // positions        // tex coords   // normals
        0.5, 0.5, 0.5,      1.0, 1.0,       0.0, 0.0, 1.0,      // 0
        0.5, -0.5, 0.5,     1.0, 0.0,       0.0, 0.0, 1.0,      // 1
       -0.5, 0.5, 0.5,      0.0, 1.0,       0.0, 0.0, 1.0,      // 3
        0.5, -0.5, 0.5,     1.0, 0.0,       0.0, 0.0, 1.0,      // 1
       -0.5, -0.5, 0.5,     0.0, 0.0,       0.0, 0.0, 1.0,      // 2
       -0.5, 0.5, 0.5,      0.0, 1.0,       0.0, 0.0, 1.0,      // 3

       -0.5, 0.5, -0.5,     1.0, 1.0,       0.0, 0.0, -1.0,     // 7
        0.5, 0.5, -0.5,     1.0, 0.0,       0.0, 0.0, -1.0,     // 4
       -0.5, -0.5, -0.5,    0.0, 1.0,       0.0, 0.0, -1.0,     // 6
       -0.5, -0.5, -0.5,    1.0, 0.0,       0.0, 0.0, -1.0,     // 6
        0.5, -0.5, -0.5,    0.0, 0.0,       0.0, 0.0, -1.0,     // 5
        0.5, 0.5, -0.5,     0.0, 1.0,       0.0, 0.0, -1.0,     // 4

        0.5, 0.5, -0.5,     1.0, 1.0,       1.0, 0.0, 0.0,      // 4
        0.5, -0.5, -0.5,    1.0, 0.0,       1.0, 0.0, 0.0,      // 5
        0.5, 0.5, 0.5,      0.0, 1.0,       1.0, 0.0, 0.0,      // 0
        0.5, -0.5, -0.5,    1.0, 0.0,       1.0, 0.0, 0.0,      // 5
        0.5, -0.5, 0.5,     0.0, 0.0,       1.0, 0.0, 0.0,      // 1
        0.5, 0.5, 0.5,      0.0, 1.0,       1.0, 0.0, 0.0,      // 0

       -0.5, 0.5, 0.5,      1.0, 1.0,      -1.0, 0.0, 0.0,      // 3
       -0.5, -0.5, 0.5,     1.0, 0.0,      -1.0, 0.0, 0.0,      // 2
       -0.5, 0.5, -0.5,     0.0, 1.0,      -1.0, 0.0, 0.0,      // 7
       -0.5, -0.5, 0.5,     1.0, 0.0,      -1.0, 0.0, 0.0,      // 2
       -0.5, -0.5, -0.5,    0.0, 0.0,      -1.0, 0.0, 0.0,      // 6
       -0.5, 0.5, -0.5,     0.0, 1.0,      -1.0, 0.0, 0.0,      // 7

        0.5, 0.5, -0.5,     1.0, 1.0,       0.0, 1.0, 0.0,      // 4
        0.5, 0.5, 0.5,      1.0, 0.0,       0.0, 1.0, 0.0,      // 0
       -0.5, 0.5, -0.5,     0.0, 1.0,       0.0, 1.0, 0.0,      // 7
        0.5, 0.5, 0.5,      1.0, 0.0,       0.0, 1.0, 0.0,      // 0
       -0.5, 0.5, 0.5,      0.0, 0.0,       0.0, 1.0, 0.0,      // 3
       -0.5, 0.5, -0.5,     0.0, 1.0,       0.0, 1.0, 0.0,      // 7

        0.5, -0.5, 0.5,     1.0, 1.0,       0.0, -1.0, 0.0,     // 1
        0.5, -0.5, -0.5,    1.0, 0.0,       0.0, -1.0, 0.0,     // 5
       -0.5, -0.5, 0.5,     0.0, 1.0,       0.0, -1.0, 0.0,     // 2
        0.5, -0.5, -0.5,    1.0, 0.0,       0.0, -1.0, 0.0,     // 5
       -0.5, -0.5, -0.5,    0.0, 0.0,       0.0, -1.0, 0.0,     // 6
       -0.5, -0.5, 0.5,     0.0, 1.0,       0.0, -1.0, 0.0,     // 2
    ];
    let cube_positions = vec![
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];
    let cube_model_transform = glm::rotation(-0.25 * PI, &glm::vec3(0.0, 0.0, 1.0));

    // Cube textures
    let cube_texture = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/crate/diffuse.png")?;
    let cube_specular_map = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/crate/specular.png")?;

    // Main loop
    let mut frame_start = SystemTime::now();
    let mut event_pump = sdl.event_pump()?;
    'main: loop {
        let now = SystemTime::now();
        let delta_time = now.duration_since(frame_start)?.as_secs_f32();
        frame_start = now;

        // General events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        let keyboard = event_pump.keyboard_state();
        if keyboard.is_scancode_pressed(Scancode::Escape) {
            break 'main;
        }

        // Look around
        let mouse_state = event_pump.relative_mouse_state();
        camera.rotate(mouse_state.x(), mouse_state.y());

        // Move camera
        if keyboard.is_scancode_pressed(Scancode::W) {
            camera.go(Forward, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::A) {
            camera.go(Left, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::S) {
            camera.go(Backward, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            camera.go(Right, delta_time);
        }

        // unsafe {
        //     gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        // }

        let proj = camera.get_projection_matrix();
        let view = camera.get_view_matrix();
    }

    Ok(())
}
