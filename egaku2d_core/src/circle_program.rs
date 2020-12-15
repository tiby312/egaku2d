use super::*;
use crate::gl;
use crate::shader::*;
use crate::vbo::BufferInfo;
use std::ffi::CString;
use std::str;

#[derive(Copy, Clone, Debug)]
pub struct ProgramUniformValues<'a> {
    pub radius: f32,
    pub mode: u32,
    pub stride: i32,
    pub texture: Option<(&'a sprite::Texture, f32, [f32; 2])>,
}
impl<'a> ProgramUniformValues<'a> {
    pub fn new(radius: f32, mode: u32) -> Self {
        ProgramUniformValues {
            mode,
            radius,
            texture: None,
            stride: 0,
        }
    }
}

// Shader sources
pub static VS_SRC: &'static str = "
#version 300 es
in vec2 position;
out vec2 pos;
uniform vec2 offset;
uniform mat3 mmatrix;
uniform float point_size;
void main() {
    gl_PointSize = point_size;
    vec3 pp=vec3(position+offset,1.0);
    pos=position*0.005;
    gl_Position = vec4(mmatrix*pp.xyz, 1.0);
}";

//https://blog.lapingames.com/draw-circle-glsl-shader/
pub static CIRCLE_FS_SRC: &'static str = "
#version 300 es
precision mediump float;
uniform vec4 bcol;
out vec4 out_color;
in vec2 pos;
in float ps;

void main() {

    vec2 coord = gl_PointCoord - vec2(0.5,0.5);
    float dis=dot(coord,coord);
    if(dis > 0.25){                  //outside of circle radius?
        discard;
    }

    out_color = bcol;
}";

pub static REGULAR_FS_SRC: &'static str = "
#version 300 es
precision mediump float;
uniform vec4 bcol;
in vec2 pos;
out vec4 out_color;

void main() {
    out_color=bcol;
}";

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex(pub [f32; 2]);

#[derive(Debug)]
pub struct CircleProgram {
    pub program: GLuint,
    pub matrix_uniform: GLint,
    pub offset_uniform: GLint,
    pub point_size_uniform: GLint,
    pub bcol_uniform: GLint,
    pub pos_attr: GLint,
}

#[derive(Debug)]
pub struct PointMul(pub f32);

impl CircleProgram {
    pub fn set_viewport(
        &mut self,
        window_dim: FixedAspectVec2,
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
        //let square=un.rect;
        let buffer_id = buffer_info.id;
        let offset = common.offset;
        let length = buffer_info.length;
        let stride = un.stride;

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

            gl::EnableVertexAttribArray(self.pos_attr as GLuint);
            gl_ok!();

            gl::VertexAttribPointer(
                self.pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                stride as i32,
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

    pub fn new(frag: &str) -> CircleProgram {
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

            let temp=CString::new("point_size").unwrap();
            let point_size_uniform: GLint =
                gl::GetUniformLocation(program, temp.as_ptr());
            gl_ok!();

            let temp= CString::new("mmatrix").unwrap();
            let matrix_uniform: GLint =
                gl::GetUniformLocation(program,temp.as_ptr());
            gl_ok!();

            let temp=CString::new("bcol").unwrap();
            let bcol_uniform: GLint =
                gl::GetUniformLocation(program, temp.as_ptr());
            gl_ok!();

            let temp=CString::new("offset").unwrap();
            let offset_uniform: GLint =
                gl::GetUniformLocation(program, temp.as_ptr());
            gl_ok!();

            let temp=CString::new("position").unwrap();
            let pos_attr =
                gl::GetAttribLocation(program, temp.as_ptr());
            gl_ok!();

            CircleProgram {
                program,
                offset_uniform,
                point_size_uniform,
                matrix_uniform,
                bcol_uniform,
                pos_attr,
            }
        }
    }
}

impl Drop for CircleProgram {
    fn drop(&mut self) {
        // Cleanup
        unsafe {
            gl::DeleteProgram(self.program);
            gl_ok!();
        }
    }
}
