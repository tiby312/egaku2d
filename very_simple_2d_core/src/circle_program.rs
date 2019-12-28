use crate::gl;
use crate::gl::types::*;
use crate::shader::*;
use axgeom;
use axgeom::*;
use std::ffi::CString;
use std::str;

// Shader sources
static VS_SRC: &'static str = "
#version 300 es
in vec2 position;
uniform mat3 mmatrix;
uniform float point_size;
void main() {
    gl_PointSize = point_size;
    vec3 pp=vec3(position,1.0);
    gl_Position = vec4(mmatrix*pp.xyz, 1.0);
}";

//https://blog.lapingames.com/draw-circle-glsl-shader/
static FS_SRC: &'static str = "
#version 300 es
precision mediump float;
uniform vec4 bcol;
out vec4 out_color;
uniform bool square;
void main() {

    //This is inefficient, but it does allow us to use only one shader program to
    //do many things. In most use-cases of a 2d graphics library, the cpu is the bottle
    //neck not the gpu anyway.
    if (square){
        vec2 coord = gl_PointCoord - vec2(0.5);
        float dis=dot(coord,coord);
        if(dis > 0.25)                  //outside of circle radius?
            discard;
    }

    out_color = bcol;
}";

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex(pub [f32; 2]);

#[derive(Debug)]
pub struct CircleProgram {
    pub program: GLuint,
    pub matrix_uniform: GLint,
    pub square_uniform: GLint,
    pub point_size_uniform: GLint,
    pub bcol_uniform: GLint,
    pub pos_attr: GLint,
}

#[derive(Debug)]
pub struct PointMul(pub f32);

impl CircleProgram {
    pub fn set_viewport(&mut self, width: f32, game_world: Rect<f32>) -> PointMul {
        let ((x1, x2), (y1, y2)) = game_world.get();
        let w = x2 - x1;
        let h = y2 - y1;

        let scalex = 2.0 / w;
        let scaley = 2.0 / h;

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

        PointMul(width / w)
    }

    pub fn set_uniforms(&mut self,point_size:f32,col:[f32;4],square:usize){
        
        unsafe {
            gl::UseProgram(self.program);
            gl_ok!();

            gl::Uniform1f(self.point_size_uniform, 0.0);
            gl_ok!();
            gl::Uniform4fv(
                self.bcol_uniform,
                1,
                col.as_ptr() as *const _
            );
            gl_ok!();

            let square = 0;
            gl::Uniform1i(self.square_uniform, square);
            gl_ok!();
        }
    }
    pub fn set_buffer_and_draw(&mut self,buffer_id:u32,mode:GLenum,length:usize){
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl_ok!();
                
            gl::VertexAttribPointer(
                    self.pos_attr as GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE as GLboolean,
                    /*2 * core::mem::size_of::<f32>() as i32*/ 0 as i32,
                    core::ptr::null(),
                );
            gl_ok!();

            gl::DrawArrays(mode, 0 as i32, length as i32);

            gl_ok!();
        }
    }

    pub fn new() -> CircleProgram {
        unsafe {
            // Create GLSL shaders
            let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
            gl_ok!();

            let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
            gl_ok!();

            let program = link_program(vs, fs);
            gl_ok!();

            gl::DeleteShader(fs);
            gl_ok!();

            gl::DeleteShader(vs);
            gl_ok!();

            gl::UseProgram(program);
            gl_ok!();

            let square_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("square").unwrap().as_ptr());
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

            let pos_attr =
                gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
            gl_ok!();

            /////
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl_ok!();
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                /*2 * core::mem::size_of::<f32>() as i32*/ 0 as i32,
                core::ptr::null(),
            );
            gl_ok!();

            CircleProgram {
                program,
                square_uniform,
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
