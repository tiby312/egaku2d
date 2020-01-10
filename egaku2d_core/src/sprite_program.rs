use crate::gl;

use crate::shader::*;
use axgeom;
use std::ffi::CString;
use std::str;

use crate::vbo::BufferInfo;
use super::*;

// Shader sources
static VS_SRC: &'static str = "
#version 300 es
in vec2 position;
in uint cellindex;

out vec2 texture_offset;
uniform vec2 offset;
uniform ivec2 grid_dim;
uniform float cell_size;

uniform mat3 mmatrix;
uniform float point_size;

void main() {
    gl_PointSize = point_size;
    vec3 pp = vec3(position.xy+offset,1.0);
    gl_Position = vec4(mmatrix*pp.xyz, 1.0);

    //float c=cos(position.z);
    //float s=sin(position.z);

    //rotation[0]=vec2(c,-s);
    //rotation[1]=vec2(s,c);

    int cellindex = int(cellindex);

    //Force cellindex to be in a valid range
    cellindex = cellindex % (grid_dim.x * grid_dim.y);

    
    //TODO optimize
    ivec2 ce=ivec2(cellindex / (grid_dim.x), cellindex % (grid_dim.x));


    //texture_offset.x=float(ce.x)/float(grid_dim.x);
    //texture_offset.y=float(ce.y)/float(grid_dim.y);
    texture_offset.x=float(ce.x);
    texture_offset.y=float(ce.y);
}";

static FS_SRC: &'static str = "
#version 300 es
precision mediump float;
in vec2 texture_offset;
uniform highp ivec2 grid_dim;
uniform sampler2D tex0;
uniform vec4 bcol;
out vec4 out_color;

void main() 
{
    vec2 k=vec2((gl_PointCoord.x+texture_offset.x)/float(grid_dim.x),(gl_PointCoord.y+texture_offset.y)/float(grid_dim.y));
    
    vec2 foo=k;
    out_color=texture(tex0,foo)*bcol;
}
";

//#[repr(transparent)]
#[repr(packed(4))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub pos: [f32; 2], //TODO use half floats??
    pub index: u16,
    pub rotation: u16
}
//pub struct Vertex(pub ([f32; 3],u32));

#[derive(Debug)]
pub struct SpriteProgram {
    pub program: GLuint,
    pub matrix_uniform: GLint,
    pub square_uniform: GLint,
    pub offset_uniform: GLint,
    pub point_size_uniform: GLint,
    pub grid_dim_uniform: GLint,
    pub cell_size_uniform: GLint,
    pub bcol_uniform: GLint,
    pub pos_attr: GLint,
    pub index_attr: GLint,
    pub sample_location: GLint,
}

#[derive(Debug)]
pub struct PointMul(pub f32);

#[derive(Copy,Clone,Debug)]
pub struct SpriteProgramUniformValues<'a>{
    pub texture:&'a crate::sprite::Texture,
    pub radius:f32
}


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

    pub(crate) fn set_buffer_and_draw(
        &mut self,
        common:&UniformCommon,
        un:&SpriteProgramUniformValues,
        buffer_info:BufferInfo,
    ) {
        let col=common.color;
        let buffer_id=buffer_info.id;
        let length=buffer_info.length;
        let point_size=un.radius;
        let mode = gl::POINTS;
        let texture=un.texture;
        let texture_id = un.texture.id;
        let offset=common.offset;
        
        unsafe {
            gl::UseProgram(self.program);
            gl_ok!();

            gl::Uniform1f(self.point_size_uniform, point_size);
            gl_ok!();
            
            gl::Uniform2f(self.offset_uniform, offset.x,offset.y);
            gl_ok!();

            gl::Uniform4fv(self.bcol_uniform, 1, col.as_ptr() as *const _);
            gl_ok!();
        
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl_ok!();

            gl::ActiveTexture(gl::TEXTURE0);
            gl_ok!();

            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl_ok!();

            gl::Uniform1i(self.sample_location, 0);
            gl_ok!();

            gl::Uniform2i(
                self.grid_dim_uniform,
                texture.grid_dim[0] as i32,
                texture.grid_dim[1] as i32,
            );
            gl_ok!();

            gl::EnableVertexAttribArray(self.pos_attr as GLuint);
            gl_ok!();

            gl::VertexAttribPointer(
                self.pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                3 * 4 as i32,
                0 as *const _,
            );
            gl_ok!();

            gl::EnableVertexAttribArray(self.index_attr as GLuint);
            gl_ok!();

            gl::VertexAttribIPointer(
                self.index_attr as GLuint,
                1,
                gl::UNSIGNED_SHORT,
                (3 * 4) as i32,
                (4 * 2) as *const _,
            );
            gl_ok!();

            gl::DrawArrays(mode, 0 as i32, length as i32);

            gl_ok!();

            gl::DisableVertexAttribArray(self.pos_attr as GLuint);
            gl_ok!();

            gl::DisableVertexAttribArray(self.index_attr as GLuint);
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

            let grid_dim_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("grid_dim").unwrap().as_ptr());
            gl_ok!();

            let cell_size_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("cell_size").unwrap().as_ptr());
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

            let offset_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("offset").unwrap().as_ptr());
            gl_ok!();

            let pos_attr =
                gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
            gl_ok!();

            let index_attr =
                gl::GetAttribLocation(program, CString::new("cellindex").unwrap().as_ptr());
            gl_ok!();

            let sample_location =
                gl::GetAttribLocation(program, CString::new("tex0").unwrap().as_ptr());
            gl_ok!();

            SpriteProgram {
                sample_location,
                program,
                square_uniform,
                offset_uniform,
                point_size_uniform,
                grid_dim_uniform,
                cell_size_uniform,
                matrix_uniform,
                bcol_uniform,
                pos_attr,
                index_attr,
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
