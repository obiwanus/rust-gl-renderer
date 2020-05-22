// ==================================== Crates and modules ========================================

extern crate nalgebra_glm as glm;

mod buffers;
mod camera;
mod shader;
mod texture;

// ==================================== Imports ===================================================

use std::error::Error;
use std::f32::consts::PI;
use std::time::SystemTime;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use gl::types::*;
use gltf::scene::Node;
use gltf::{Material, Mesh};

// Local imports
use camera::Camera;
use camera::Movement::*;
use shader::Program;
use texture::Texture;

use buffers::{ElementBuffer, Model, VertexArray, VertexBuffer};

// ==================================== Functions =================================================

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
        .fullscreen_desktop()
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

    let light_pos = glm::vec3(-0.7, 0.2, 2.0);
    let light_color = glm::vec3(1.0, 1.0, 1.0);

    // Flat color shader
    let flatcolor_shader = Program::new()
        .vertex_shader("assets/shaders/flatcolor/flatcolor.vert")?
        .fragment_shader("assets/shaders/flatcolor/flatcolor.frag")?
        .link()?;
    flatcolor_shader.set_used();
    flatcolor_shader.set_vec3("point_light.ambient", &(0.2f32 * light_color))?;
    flatcolor_shader.set_vec3("point_light.diffuse", &(0.5f32 * light_color))?;
    flatcolor_shader.set_vec3("point_light.specular", &(1.0f32 * light_color))?;
    flatcolor_shader.set_float("point_light.attn_linear", 0.09)?;
    flatcolor_shader.set_float("point_light.attn_quadratic", 0.032)?;

    // Load model
    let (document, buffers, _images) = gltf::import("assets/models/culdesac/culdesac.glb")?;

    // Define opengl buffers
    assert_eq!(document.buffers().len(), 1);
    let buffer = document.buffers().next().unwrap();
    let mut buffer_id: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut buffer_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            buffer.length() as isize,
            buffers[0].as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }

    assert_eq!(document.scenes().len(), 1);
    let scene = document.scenes().next().unwrap();

    let models: Vec<Model> = scene.nodes().filter_map(Model::from).collect();

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

        // Set transforms
        flatcolor_shader.set_mat4("proj", &proj)?;
        flatcolor_shader.set_mat4("view", &view)?;

        // Light
        let p = light_pos;
        let light_pos = glm::vec4_to_vec3(&(view * glm::vec4(p.x, p.y, p.z, 1.0)));
        flatcolor_shader.set_vec3("point_light.position", &light_pos)?;

        for model in models.iter() {
            flatcolor_shader.set_mat4("model", &glm::Mat4x4::from(model.transform))?;

            model.vao.bind();
            model.draw_triangles(buffer_id);
        }

        window.gl_swap_window();
    }

    Ok(())
}
