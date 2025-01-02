use crate::prelude::*;

use sdl2::video::Window;

#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}
pub enum TextureType {
    DIFFUSE(i32),
    SPECULAR(i32),
}
pub struct Texture {
    pub texture_type: TextureType,
}

pub struct App {
    video: VideoSubsystem,
    window: sdl2::video::Window,
    event_loop: sdl2::EventPump,
    gl_context: sdl2::video::GLContext,
    pub window_width: u32,
    pub window_height: u32,
}

impl App {
    pub fn new(title: &str, window_width: u32, window_height: u32) -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_flags().forward_compatible().set();
        let window = video
            .window(title, window_width, window_height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window
            .gl_create_context()
            .expect("Can't create OpenGL context!!!");
        Self {
            video,
            window,
            event_loop: sdl.event_pump().unwrap(),
            gl_context,
            window_width,
            window_height,
        }
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn event_loop(&mut self) -> &mut EventPump {
        &mut self.event_loop
    }
    pub fn gl_context(&self) -> &GLContext {
        &self.gl_context
    }
    pub fn video(&self) -> &VideoSubsystem {
        &self.video
    }
}
