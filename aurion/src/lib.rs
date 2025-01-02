mod app;
mod graphics;
mod mesh;
mod scene_graph;
mod shader;
mod utils;
pub mod prelude {
    pub use crate::utils::*;
    pub use crate::app::*;
    pub use crate::graphics::*;
    pub use crate::mesh::*;
    pub use crate::scene_graph::*;
    pub use crate::shader::Shader;
    pub use glam::*;
    pub use glow::*;
    pub use sdl2::video::GLContext;
    pub use sdl2::*;
    pub use std::io::Read;
    pub use std::time::Instant;
}

