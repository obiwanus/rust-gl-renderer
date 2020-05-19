// ==================================== Crates and modules =======================================

extern crate nalgebra_glm as glm;

mod buffers;
mod camera;
mod shader;
mod texture;

// ==================================== Imports ==================================================

use std::error::Error;
use std::f32::consts::PI;
use std::time::SystemTime;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use camera::Camera;
use camera::Movement::*;
use shader::Program;
use texture::Texture;

use buffers::{VertexArray, VertexBuffer};

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
    let gl_context = window.gl_create_context()?;
    window.gl_make_current(&gl_context)?;
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
    let light_pos = glm::vec3(-0.7, 0.2, 2.0);
    let light_color = glm::vec3(1.0, 1.0, 1.0);

    // Cube textures
    let cube_texture = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/crate/diffuse.png")?;
    let cube_specular_map = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/crate/specular.png")?;

    // Cube buffers
    let stride = 8;
    let mut cube = VertexBuffer::new();
    cube.bind();
    cube.set_static_data(&cube_vertices, stride);
    let cube_vao = VertexArray::new();
    cube_vao.bind();
    cube_vao.set_attrib(0, 3, stride, 0); // positions
    cube_vao.set_attrib(1, 2, stride, 3); // texture coords
    cube_vao.set_attrib(2, 3, stride, 5); // normals
    cube.unbind();

    // Cube shaders
    let cube_shader = Program::new()
        .vertex_shader("assets/shaders/cube/cube.vert")?
        .fragment_shader("assets/shaders/cube/cube.frag")?
        .link()?;
    cube_shader.set_used();
    // Set default material
    cube_shader.set_texture_unit("material.diffuse", 0)?;
    cube_shader.set_texture_unit("material.specular", 1)?;
    cube_shader.set_float("material.shininess", 32.0)?;

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
                Event::Quit { .. } => break 'main,
                Event::MouseWheel { y, .. } => camera.adjust_zoom(y),
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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let proj = camera.get_projection_matrix();
        let view = camera.get_view_matrix();

        // Cube transforms
        cube_shader.set_used();
        cube_shader.set_mat4("proj", &proj)?;
        cube_shader.set_mat4("view", &view)?;
        cube_shader.set_mat4("model", &cube_model_transform)?;

        // Lights
        let p = light_pos;
        let light_pos = glm::vec4_to_vec3(&(view * glm::vec4(p.x, p.y, p.z, 1.0)));
        cube_shader.set_vec3("light.position", &light_pos)?;
        cube_shader.set_vec3("light.ambient", &(0.2f32 * light_color))?;
        cube_shader.set_vec3("light.diffuse", &(0.5f32 * light_color))?;
        cube_shader.set_vec3("light.specular", &(1.0f32 * light_color))?;
        cube_shader.set_float("light.attn_linear", 0.09)?;
        cube_shader.set_float("light.attn_quadratic", 0.032)?;

        cube_vao.bind();
        cube.draw_triangles();

        window.gl_swap_window();
    }

    Ok(())
}
