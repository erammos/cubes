use crate::app::{Texture, Vertex};
use crate::prelude::Graphics;
use glow::{HasContext, NativeBuffer, NativeVertexArray};
pub type MeshId = usize;
pub struct Mesh<'a> {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture: Vec<Texture>,
    pub vao: NativeVertexArray,
    pub vbo: NativeBuffer,
    pub ebo: NativeBuffer,
    gl: &'a glow::Context,
}
impl<'a> Mesh<'a> {
    pub fn new(
        renderer: &'a Graphics,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        texture: Vec<Texture>,
    ) -> Self {
        let (vao, vbo, ebo) = renderer.create_vertex_buffer(&vertices, &indices);
        Self {
            vertices,
            indices,
            texture,
            vao,
            vbo,
            ebo,
            gl: &renderer.gl,
        }
    }
}

impl Drop for Mesh<'_> {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_buffer(self.ebo);
        }
    }
}
