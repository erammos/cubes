use crate::graphics;
use crate::prelude::*;
use glow::{HasContext, NativeProgram};

pub struct Shader<'a> {
    program: NativeProgram,
    gl: &'a glow::Context,
}
impl<'a> Shader<'a> {
    pub fn new(
        renderer: &'a graphics::Graphics,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Self {
        let p = Shader::create_program(&renderer.gl, &vertex_source, &fragment_source);
        Shader {
            program: p,
            gl: &renderer.gl,
        }
    }
    pub fn use_program(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }
    pub fn set_uniform_1_f32(&self, name: &str, value: f32) {
        unsafe {
            let uniform_location = self.gl.get_uniform_location(self.program, name);
            // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
            self.gl.uniform_1_f32(uniform_location.as_ref(), value)
        }
    }
    pub fn set_uniform_1_i32(&self, name: &str, value: i32) {
        unsafe {
            let uniform_location = self.gl.get_uniform_location(self.program, name);
            self.gl.uniform_1_i32(uniform_location.as_ref(), value)
        }
    }
    pub fn set_uniform_mat4_f32(&self, name: &str, mat: &Mat4) {
        unsafe {
            let uniform_location = self.gl.get_uniform_location(self.program, name);
            self.gl.uniform_matrix_4_f32_slice(
                uniform_location.as_ref(),
                false,
                &mat.to_cols_array(),
            );
        }
    }
    fn create_program(
        gl: &glow::Context,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> NativeProgram {
        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];
        unsafe {
            let program = gl.create_program().expect("Cannot create program");
            let mut shaders = Vec::with_capacity(shader_sources.len());
            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }
            program
        }
    }
}
impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
        }
    }
}
