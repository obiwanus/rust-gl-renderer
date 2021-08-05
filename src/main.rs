// ==================================== Crates and modules ========================================

extern crate gl as opengl;

mod utils;

mod buffers;
mod camera;
mod scene;
mod shader;
mod skybox;
mod texture;

// ==================================== Imports ===================================================

use gl::types::*;
use std::error::Error;
use std::ffi::CStr;
use std::time::Instant;

use glutin::event::{
    DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::{Api, GlProfile, GlRequest};
use glutin::{PossiblyCurrent, WindowedContext};

use glam::{Vec3, Vec4};

// Local imports
use camera::Camera;
use camera::Movement::*;
use scene::Scene;
use shader::Program;
use skybox::Skybox;

// ==================================== Types =====================================================

struct Game {
    windowed_context: WindowedContext<PossiblyCurrent>,
    input: Input,
    camera: Camera,
    in_focus: bool,
    frame_start: Instant,

    // @tmp
    scene: Scene,
    shader: Program,
    skybox: Skybox,
    light: DirectionalLight,
}

#[derive(Default)]
struct Input {
    forward: bool,
    back: bool,
    left: bool,
    right: bool,
}

struct DirectionalLight {
    color: Vec3,
    direction: Vec3,
}

// ==================================== Functions =================================================

fn main() {
    let event_loop = EventLoop::new();
    let mut game = Game::new(&event_loop).unwrap_or_else(|error| {
        eprintln!("{}", error);
        std::process::exit(1);
    });

    // Run winit event loop (which is used as the game loop)
    event_loop.run(move |event, _, control_flow| {
        if let Err(error) = game.main_loop(event, control_flow) {
            eprint!("{}", error);
            std::process::exit(1);
        };
    });
}

impl Game {
    /// Creates a window and inits a new game
    fn new(event_loop: &EventLoop<()>) -> Result<Self, Box<dyn Error>> {
        // Create window
        let window_builder = WindowBuilder::new()
            .with_title("Game 2")
            .with_resizable(false)
            .with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
            .with_inner_size(glutin::dpi::LogicalSize::new(1366.0, 768.0));
        let gl_request = GlRequest::Specific(Api::OpenGl, (3, 3));
        let gl_profile = GlProfile::Core;
        let windowed_context = glutin::ContextBuilder::new()
            .with_gl(gl_request)
            .with_gl_profile(gl_profile)
            .with_double_buffer(Some(true))
            .with_depth_buffer(16)
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)?;

        // Set up OpenGL
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);
        let window = windowed_context.window();
        window.set_cursor_grab(true)?;
        window.set_cursor_visible(false);
        let window_size = window.inner_size();
        unsafe {
            gl::Viewport(0, 0, window_size.width as i32, window_size.height as i32);
            gl::ClearColor(0.05, 0.05, 0.05, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::FRAMEBUFFER_SRGB);

            #[cfg(not(target_os = "macos"))]
            {
                // MacOS deprecated OpenGL, which is stuck at 4.1 so no debug callbacks here :(
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
            }
        }

        // Set up camera
        let camera = Camera::new(
            Vec3::new(0.0, 0.5, -23.0),
            Vec3::new(0.0, 0.5, 0.0),
            window_size.width,
            window_size.height,
        );

        let light = DirectionalLight {
            color: Vec3::new(1.0, 0.7, 0.7),
            direction: Vec3::new(0.37f32, -0.56, 0.75),
        };

        // Flat color shader
        let shader = Program::new()
            .vertex_shader("assets/shaders/flatcolor/flatcolor.vert")?
            .fragment_shader("assets/shaders/flatcolor/flatcolor.frag")?
            .link()?;
        shader.set_used();

        // Directional light
        shader.set_vec3("directional_light.ambient", &(0.2f32 * light.color))?;
        shader.set_vec3("directional_light.diffuse", &(0.5f32 * light.color))?;
        shader.set_vec3("directional_light.specular", &(1.0f32 * light.color))?;

        // Set default material
        shader.set_vec3("material.specular", &Vec3::new(0.4, 0.4, 0.4))?;
        shader.set_float("material.shininess", 10.0)?;

        let scene = Scene::from("assets/models/culdesac/culdesac.glb")?;
        let skybox = Skybox::from([
            "assets/textures/skybox/right.jpg",
            "assets/textures/skybox/left.jpg",
            "assets/textures/skybox/top.jpg",
            "assets/textures/skybox/bottom.jpg",
            "assets/textures/skybox/front.jpg",
            "assets/textures/skybox/back.jpg",
        ])?;

        Ok(Game {
            windowed_context,
            input: Input::default(),
            camera,
            in_focus: true,
            frame_start: Instant::now(),

            scene,
            shader,
            skybox,
            light,
        })
    }

    fn main_loop(
        &mut self,
        event: Event<()>,
        control_flow: &mut ControlFlow,
    ) -> Result<(), Box<dyn Error>> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                } => {
                    // Mouse button click

                    println!("camera: {:?}", self.camera);
                }
                WindowEvent::Focused(focused) => {
                    self.in_focus = focused;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    // Key press
                    match key {
                        VirtualKeyCode::W => self.input.forward = state == ElementState::Pressed,
                        VirtualKeyCode::A => self.input.left = state == ElementState::Pressed,
                        VirtualKeyCode::S => self.input.back = state == ElementState::Pressed,
                        VirtualKeyCode::D => self.input.right = state == ElementState::Pressed,
                        VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                        _ => {}
                    }
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } if self.in_focus => {
                    let (yaw_delta, pitch_delta) = delta;
                    self.camera.rotate(yaw_delta as f32, pitch_delta as f32);
                }
                _ => {}
            },
            Event::MainEventsCleared => self.update_and_render()?,
            _ => {}
        };
        Ok(())
    }

    fn update_and_render(&mut self) -> Result<(), Box<dyn Error>> {
        // Application code
        let now = Instant::now();
        let delta_time = now.duration_since(self.frame_start).as_secs_f32();
        self.frame_start = now;

        // Move camera
        if self.input.forward {
            self.camera.go(Forward, delta_time);
        }
        if self.input.left {
            self.camera.go(Left, delta_time);
        }
        if self.input.back {
            self.camera.go(Backward, delta_time);
        }
        if self.input.right {
            self.camera.go(Right, delta_time);
        }

        let proj = self.camera.get_projection_matrix();
        let view = self.camera.get_view_matrix();

        // Draw
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.shader.set_used();
        self.shader.set_mat4("proj", &proj)?;
        self.shader.set_mat4("view", &view)?;
        let light_direction: Vec3 = (view
            * Vec4::new(
                self.light.direction.x,
                self.light.direction.y,
                self.light.direction.z,
                0.0,
            ))
        .into();
        self.shader
            .set_vec3("directional_light.direction", &light_direction)?;
        self.scene.draw(&self.shader)?;
        self.skybox.draw(&proj, &view)?; // draw skybox last

        self.windowed_context.swap_buffers()?;

        #[cfg(feature = "debug")]
        {
            // Display frame time
            let frame_ms =
                Instant::now().duration_since(self.frame_start).as_micros() as f32 / 1000.0;
            println!("frame: {} ms", frame_ms);
        }

        Ok(())
    }
}

extern "system" fn debug_callback(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut std::os::raw::c_void,
) {
    let msg_type = if gltype == gl::DEBUG_TYPE_ERROR {
        "** GL ERROR ** "
    } else {
        "** GL DEBUG **"
    };
    let msg = unsafe { CStr::from_ptr(message) };
    eprintln!("{} {}", msg_type, msg.to_str().unwrap().to_owned());
}
