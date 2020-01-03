use crate::gl;
use crate::gl::types::*;
use crate::shader::*;
use axgeom;
use std::ffi::CString;
use std::str;

// Shader sources
static VS_SRC: &'static str = "
#version 300 es
in vec3 position;
out mat2 rotation;
uniform mat3 mmatrix;
uniform float point_size;
void main() {
    gl_PointSize = point_size;
    vec3 pp=vec3(position.xy,1.0);
    gl_Position = vec4(mmatrix*pp.xyz, 1.0);

    float c=cos(position.z);
    float s=sin(position.z);

    rotation[0]=vec2(c,-s);
    rotation[1]=vec2(s,c);
}";

static FS_SRC: &'static str = "
#version 300 es
precision mediump float;

uniform sampler2D tex0;
in mat2 rotation;
out vec4 out_color;

void main() 
{
    vec2 foo=(gl_PointCoord-vec2(0.5,0.5))*1.5;

    vec2 foo2=(rotation * foo);
    
    if (foo2.x<-0.5 || foo2.x>0.5){
        discard;
    }
    if (foo2.y<-0.5 || foo2.y>0.5){
        discard;
    }
    
    
    vec2 texCoord = foo2 + vec2(0.5,0.5);
    out_color = texture(tex0, texCoord) ;
}
";

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex(pub [f32; 3]);

#[derive(Debug)]
pub struct SpriteProgram {
    pub program: GLuint,
    pub matrix_uniform: GLint,
    pub square_uniform: GLint,
    pub point_size_uniform: GLint,
    pub bcol_uniform: GLint,
    pub pos_attr: GLint,
    pub sample_location: GLint
}

#[derive(Debug)]
pub struct PointMul(pub f32);

impl SpriteProgram {
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

    pub fn set_buffer_and_draw(
        &mut self,
        point_size: f32,
        col: [f32; 4],
        square: usize,
        buffer_id: u32,
        mode: GLenum,
        length: usize,
        texture_id:u32
    ) {

        //TODO NO IDEA WHY THIS IS NEEDED ON LINUX.
        //Without this function call, on linux not every shape gets drawn.
        //gl_PointCoord will always return zero if you you try
        //and draw some circles after drawing a rect save.
        //It is something to do with changing between gl::TRIANGLES to gl::POINTS.
        //but this shouldnt be a problem since they are seperate vbos.
        unsafe {
            gl::UseProgram(self.program);
            gl_ok!();

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl_ok!();

            let mut data = 0i32;
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING,&mut data);
            assert_eq!(data as u32,buffer_id);

            //gl::DrawArrays(mode, 0, length as i32);
            //gl_ok!();

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl_ok!();
        }

        unsafe {
            
            gl::Uniform1f(self.point_size_uniform, point_size);
            gl_ok!();

            gl::Uniform4fv(self.bcol_uniform, 1, col.as_ptr() as *const _);
            gl_ok!();

            gl::Uniform1i(self.square_uniform, square as i32);
            gl_ok!();

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl_ok!();



            gl::ActiveTexture(gl::TEXTURE0);
            gl_ok!();

            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl_ok!();            
            

            gl::Uniform1i(self.sample_location,0);   
            gl_ok!();


        



            gl::EnableVertexAttribArray(self.pos_attr as GLuint);
            gl_ok!();

            gl::VertexAttribPointer(
                self.pos_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0 as i32,
                core::ptr::null(),
            );
            gl_ok!();

            gl::DrawArrays(mode, 0 as i32, length as i32);

            gl_ok!();

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl_ok!();

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl_ok!();            
            
        }
    }

    pub fn new() -> SpriteProgram {
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

            let sample_location = 
                gl::GetAttribLocation(program, CString::new("tex0").unwrap().as_ptr());
            gl_ok!();

            SpriteProgram {
                sample_location,
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

impl Drop for SpriteProgram {
    fn drop(&mut self) {
        // Cleanup
        unsafe {
            gl::DeleteProgram(self.program);
            gl_ok!();
        }
    }
}
