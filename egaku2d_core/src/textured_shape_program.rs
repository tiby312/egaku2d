use super::*;
use crate::gl;
use crate::shader::*;
use crate::vbo::BufferInfo;
use axgeom;
use std::ffi::CString;
use std::str;

// Shader sources
pub static VS_SRC: &'static str = "
#version 300 es
in vec2 position;
out float ps;

uniform vec2 offset;
uniform mat3 mmatrix;
uniform float point_size;
void main() {
    gl_PointSize = point_size;
    vec3 pp=vec3(position+offset,1.0);
    ps=gl_PointSize;
    gl_Position = vec4(mmatrix*pp.xyz, 1.0);
}";

//https://blog.lapingames.com/draw-circle-glsl-shader/
pub static CIRCLE_FS_SRC: &'static str = "
#version 300 es
precision mediump float;
uniform vec4 bcol;
out vec4 out_color;
in float ps;
uniform sampler2D tex0;
uniform vec2 texture_dim;
uniform float texture_scale;
uniform vec2 texture_offset;
void main() {

    vec2 coord = gl_PointCoord - vec2(0.5,0.5);
    float dis=dot(coord,coord);
    if(dis > 0.25){                  //outside of circle radius?
        discard;
    }

    vec2 pos;
    pos.x=gl_FragCoord.x;
    pos.y=-gl_FragCoord.y;
    
    out_color = texture(tex0,( ((pos-texture_offset)/texture_dim)/texture_scale))*bcol;
}";

pub static REGULAR_FS_SRC: &'static str = "
#version 300 es
precision mediump float;
uniform vec4 bcol;
out vec4 out_color;

uniform vec2 texture_dim;
uniform float texture_scale;
uniform vec2 texture_offset;
uniform sampler2D tex0;

void main() {
    vec2 pos;
    pos.x=gl_FragCoord.x;
    pos.y=-gl_FragCoord.y;
    out_color = texture(tex0, ((pos-texture_offset)/texture_dim)/texture_scale)*bcol;

}";

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex(pub [f32; 2]);

#[derive(Debug)]
pub struct TexturedShapeProgram {
    pub program: GLuint,
    pub matrix_uniform: GLint,
    pub offset_uniform: GLint,
    pub texture_dim_uniform: GLint,
    pub texture_offset_uniform: GLint,
    pub texture_scale_uniform: GLint,
    pub point_size_uniform: GLint,
    pub bcol_uniform: GLint,
    pub pos_attr: GLint,
    pub sample_location: GLint,
}

#[derive(Debug)]
pub struct PointMul(pub f32);

impl TexturedShapeProgram {
    pub fn set_viewport(
        &mut self,
        window_dim: axgeom::FixedAspectVec2,
        game_width: f32,
    ) -> PointMul {
        let game_height = window_dim.ratio.height_over_width() as f32 * game_width;

        let scalex = 2.0 / game_width;
        let scaley = 2.0 / game_height;

        let tx = -1.0;
        let ty = 1.0;

        let matrix = [[scalex, 0.0, 0.0], [0.0, -scaley, 0.0], [tx, ty, 1.0]];

        unsafe {
            gl::UseProgram(self.program);
            gl_ok!();
            gl::UniformMatrix3fv(
                self.matrix_uniform,
                1,
                0,
                std::mem::transmute(&matrix[0][0]),
            );
            gl_ok!();
        }

        PointMul(window_dim.width as f32 / game_width)
    }

    pub(crate) fn set_buffer_and_draw(
        &mut self,
        common: &UniformCommon,
        un: &ProgramUniformValues,
        buffer_info: BufferInfo,
    ) {
        let mode = un.mode;
        let point_size = un.radius;
        let col = common.color;
        let buffer_id = buffer_info.id;
        let offset = common.offset;
        let length = buffer_info.length;

        unsafe {
            gl::UseProgram(self.program);
            gl_ok!();

            gl::Uniform2f(self.offset_uniform, offset.x, offset.y);
            gl_ok!();

            gl::Uniform1f(self.point_size_uniform, point_size);
            gl_ok!();

            gl::Uniform4fv(self.bcol_uniform, 1, col.as_ptr() as *const _);
            gl_ok!();

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl_ok!();

            match un.texture {
                Some((t, scale, offset)) => {
                    let texture_id = t.id;

                    gl::ActiveTexture(gl::TEXTURE0);
                    gl_ok!();

                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                    gl_ok!();

                    gl::Uniform1i(self.sample_location, 0);
                    gl_ok!();

                    //dbg!(t.dim);
                    gl::Uniform2f(self.texture_dim_uniform, t.dim[0] as f32, t.dim[1] as f32);
                    gl_ok!();

                    gl::Uniform2f(self.texture_offset_uniform, offset[0], offset[1]);
                    gl_ok!();

                    gl::Uniform1f(self.texture_scale_uniform, scale);
                    gl_ok!();
                }
                None => {
                    unreachable!();
                }
            }

            gl::EnableVertexAttribArray(self.pos_attr as GLuint);
            gl_ok!();

            gl::VertexAttribPointer(
                self.pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0 as i32,
                core::ptr::null(),
            );
            gl_ok!();

            gl::DrawArrays(mode, 0 as i32, length as i32);

            gl_ok!();

            gl::DisableVertexAttribArray(self.pos_attr as GLuint);
            gl_ok!();

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl_ok!();
        }
    }

    pub fn new(frag: &str) -> TexturedShapeProgram {
        unsafe {
            // Create GLSL shaders
            let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
            gl_ok!();

            let fs = compile_shader(frag, gl::FRAGMENT_SHADER);
            gl_ok!();

            let program = link_program(vs, fs);
            gl_ok!();

            gl::DeleteShader(fs);
            gl_ok!();

            gl::DeleteShader(vs);
            gl_ok!();

            gl::UseProgram(program);
            gl_ok!();

            let texture_scale_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("texture_scale").unwrap().as_ptr());
            gl_ok!();

            let texture_dim_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("texture_dim").unwrap().as_ptr());
            gl_ok!();

            let texture_offset_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("texture_offset").unwrap().as_ptr());
            gl_ok!();

            let point_size_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("point_size").unwrap().as_ptr());
            gl_ok!();

            let matrix_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("mmatrix").unwrap().as_ptr());
            gl_ok!();

            let bcol_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("bcol").unwrap().as_ptr());
            gl_ok!();

            let offset_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("offset").unwrap().as_ptr());
            gl_ok!();

            let pos_attr =
                gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
            gl_ok!();

            let sample_location =
                gl::GetAttribLocation(program, CString::new("tex0").unwrap().as_ptr());
            gl_ok!();

            TexturedShapeProgram {
                program,
                offset_uniform,
                texture_dim_uniform,
                texture_offset_uniform,
                texture_scale_uniform,
                point_size_uniform,
                matrix_uniform,
                bcol_uniform,
                pos_attr,
                sample_location,
            }
        }
    }
}

impl Drop for TexturedShapeProgram {
    fn drop(&mut self) {
        // Cleanup
        unsafe {
            gl::DeleteProgram(self.program);
            gl_ok!();
        }
    }
}
