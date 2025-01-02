use crate::app;
use crate::prelude::*;
use std::mem::offset_of;

pub struct Graphics {
    pub gl: glow::Context,
    aspect_ratio: f32,
}

impl Graphics {
    pub fn new(app: &app::App) -> Graphics {
        unsafe {
            let gl = glow::Context::from_loader_function(|s| {
                app.video().gl_get_proc_address(s) as *const _
            });
            Self {
                gl,
                aspect_ratio: app.window_width as f32 / app.window_height as f32,
            }
        }
    }
    pub fn begin_frame(&self) {
        unsafe {
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.disable(glow::CULL_FACE);
        }
    }

    pub fn draw_mesh(&self, transform: &Mat4, shader: &crate::shader::Shader, mesh: &Mesh) {
        unsafe {
            for (i, texture) in mesh.texture.iter().enumerate() {
                self.gl.active_texture(glow::TEXTURE0 + i as u32);
                let texture_type_str = match texture.texture_type {
                    TextureType::DIFFUSE(_) => "texture_diffuse",
                    TextureType::SPECULAR(_) => "texture_specular",
                };
            }
            self.gl.bind_vertex_array(Some(mesh.vao));
            let camera_position = Vec3::new(0.0, 0.0, 5.0);
            let target = Vec3::new(0.0, 0.0, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);

            let projection =
                Mat4::perspective_rh_gl(45.0_f32.to_radians(), self.aspect_ratio, 0.1, 100.0);
            let view = Mat4::look_at_rh(camera_position, target, up);
            shader.set_uniform_mat4_f32("model", transform);
            shader.set_uniform_mat4_f32("projection", &projection);
            shader.set_uniform_mat4_f32("view", &view);
            self.gl.draw_elements(
                glow::TRIANGLES,
                mesh.indices.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );

            self.gl.bind_vertex_array(None);
        }
    }
    pub fn end_frame(&self, window: &sdl2::video::Window) {
        window.gl_swap_window();
    }
    pub fn create_vertex_buffer(
        &self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> (NativeVertexArray, NativeBuffer, NativeBuffer) {
        unsafe {
            let vertices_u8: &[u8] = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * core::mem::size_of::<Vertex>(),
            );
            let indices_u8: &[u8] = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * core::mem::size_of::<u32>(),
            );

            // We construct a buffer and upload the data
            let vbo = self.gl.create_buffer().unwrap();
            let vao = self.gl.create_vertex_array().unwrap();
            let ebo = self.gl.create_buffer().unwrap();
            self.gl.bind_vertex_array(Some(vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            self.gl
                .buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            self.gl
                .buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices_u8, glow::STATIC_DRAW);

            self.gl.enable_vertex_attrib_array(0);

            self.gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                size_of::<Vertex>() as i32,
                offset_of!(Vertex, position) as i32,
            );

            self.gl.enable_vertex_attrib_array(1);
            //
            self.gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                size_of::<Vertex>() as i32,
                offset_of!(Vertex, normal) as i32,
            );
            //
            self.gl.enable_vertex_attrib_array(2);
            //
            self.gl.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                size_of::<Vertex>() as i32,
                offset_of!(Vertex, uv) as i32,
            );

            self.gl.bind_vertex_array(None);

            (vao, vbo, ebo)
        }
    }
    fn create_triangle<'a>(&self) -> Mesh {
        // Define vertices of the triangle
        let vertices = vec![
            Vertex {
                position: Vec3::new(0.0, 0.5, 0.0), // Top vertex
                normal: Vec3::new(0.0, 0.0, 1.0),   // Normal pointing forward
                uv: Vec2::new(0.5, 1.0),            // UV for the top vertex
            },
            Vertex {
                position: Vec3::new(-0.5, -0.5, 0.0), // Bottom-left vertex
                normal: Vec3::new(0.0, 0.0, 1.0),     // Normal pointing forward
                uv: Vec2::new(0.0, 0.0),              // UV for the bottom-left vertex
            },
            Vertex {
                position: Vec3::new(0.5, -0.5, 0.0), // Bottom-right vertex
                normal: Vec3::new(0.0, 0.0, 1.0),    // Normal pointing forward
                uv: Vec2::new(1.0, 0.0),             // UV for the bottom-right vertex
            },
        ];
        let indices = vec![0, 1, 2];

        // No texture for now
        let texture = vec![];

        // Create and return the Mesh
        Mesh::new(&self, vertices, indices, texture)
    }
    pub fn create_cube<'a>(&self) -> Mesh {
        // Define vertices of the cube
        let vertices = vec![
            // Front face
            Vertex {
                position: Vec3::new(-0.5, -0.5, 0.5), // Bottom-left
                normal: Vec3::new(0.0, 0.0, 1.0),
                uv: Vec2::new(0.0, 0.0),
            },
            Vertex {
                position: Vec3::new(0.5, -0.5, 0.5), // Bottom-right
                normal: Vec3::new(0.0, 0.0, 1.0),
                uv: Vec2::new(1.0, 0.0),
            },
            Vertex {
                position: Vec3::new(0.5, 0.5, 0.5), // Top-right
                normal: Vec3::new(0.0, 0.0, 1.0),
                uv: Vec2::new(1.0, 1.0),
            },
            Vertex {
                position: Vec3::new(-0.5, 0.5, 0.5), // Top-left
                normal: Vec3::new(0.0, 0.0, 1.0),
                uv: Vec2::new(0.0, 1.0),
            },
            // Back face
            Vertex {
                position: Vec3::new(-0.5, -0.5, -0.5), // Bottom-left
                normal: Vec3::new(0.0, 0.0, -1.0),
                uv: Vec2::new(0.0, 0.0),
            },
            Vertex {
                position: Vec3::new(0.5, -0.5, -0.5), // Bottom-right
                normal: Vec3::new(0.0, 0.0, -1.0),
                uv: Vec2::new(1.0, 0.0),
            },
            Vertex {
                position: Vec3::new(0.5, 0.5, -0.5), // Top-right
                normal: Vec3::new(0.0, 0.0, -1.0),
                uv: Vec2::new(1.0, 1.0),
            },
            Vertex {
                position: Vec3::new(-0.5, 0.5, -0.5), // Top-left
                normal: Vec3::new(0.0, 0.0, -1.0),
                uv: Vec2::new(0.0, 1.0),
            },
        ];

        // Define indices for the cube (6 faces, 2 triangles per face)
        let indices = vec![
            // Front face
            0, 1, 2, 2, 3, 0, // Back face
            4, 5, 6, 6, 7, 4, // Left face
            4, 0, 3, 3, 7, 4, // Right face
            1, 5, 6, 6, 2, 1, // Top face
            3, 2, 6, 6, 7, 3, // Bottom face
            4, 5, 1, 1, 0, 4,
        ];

        // No texture for now
        let texture = vec![];

        // Create and return the Mesh
        Mesh::new(&self, vertices, indices, texture)
    }
}
