use crate::gl;

use crate::shader::*;
use axgeom;
use std::ffi::CString;
use std::str;

use super::*;
use crate::vbo::BufferInfo;

// Shader sources
static VS_SRC: &'static str = "
#version 100
attribute vec2 position;
attribute float rotation;
attribute float cellindex;

varying vec2 texture_offset;
varying mat2 rot_matrix;

uniform vec2 offset;
uniform vec2 grid_dim;
uniform vec2 sprite_dim;


uniform mat3 mmatrix;
uniform float point_size;



const float PI = 3.1415926535897932384626433832795;

void main() {
    gl_PointSize = point_size;
    vec3 pp = vec3(position.xy+offset,1.0);
    gl_Position = vec4(mmatrix*pp.xyz, 1.0);

    float rot=rotation*(PI*2.0);
    float c=cos(rot);
    float s=sin(rot);

    rot_matrix[0]=vec2(c,-s);
    rot_matrix[1]=vec2(s,c);

    int cellindex_i = int(cellindex);

    int grid_y = cellindex_i / int(grid_dim.x);
    int grid_x = cellindex_i - (grid_y * int(grid_dim.x));
    
    ivec2 ce=ivec2(grid_x, grid_y);

    texture_offset.x=float(ce.x);
    texture_offset.y=float(ce.y);
}";

static FS_SRC: &'static str = "
#version 100
precision mediump float;

varying vec2 texture_offset;
varying mat2 rot_matrix;


uniform highp vec2 grid_dim;
uniform highp vec2 sprite_dim;
uniform sampler2D tex0;
uniform vec4 bcol;


const float SQRT2=1.41421356237;

//This is the offset from the outer rectangle to the inner rectangle.
//We need a larger outer rectangle since if the sprite rotates, its corners would clip.
//The width of the outer rectangle needs to be sqrt(2)*normal rectangle.
const float s2=(SQRT2-1.0)/(2.0*SQRT2);

const vec2 mid=vec2(0.5,0.5);

void main() 
{
    vec2 dim=vec2(float(grid_dim.x),float(grid_dim.y));
    mat2 grid_dim2=mat2(1.0/dim.x,0.0,0.0,1.0/dim.y);
    
    //Handle rotation before we do anything.`
    vec2 pos=  (rot_matrix*( (gl_PointCoord.xy-mid)) + mid);
    
    vec2 extra=vec2(max(0.0,(sprite_dim.y-sprite_dim.x)/3.0),max(0.0,(sprite_dim.x-sprite_dim.y)/3.0)) ;
    extra.x+=0.01; //TODO why is this needed?
    extra.y+=0.01; // I think some of the math needs to be simplified. floating point loss ofprecision??
    //Now we make sure we don't draw anything in the wasted areas of the outer
    //rectangle.
    if (pos.x>=(1.0-s2-extra.x) || pos.x<(0.0+s2+extra.x) || pos.y>=(1.0-s2-extra.y) || pos.y<(0.0+s2+extra.y)){
        discard;
    }else{     
    
        vec2 pp1=(pos-mid)/sprite_dim+mid;
        
        //We must start drawing the sprite at the inner rectangle top left corder,
        //instead of the default 0,0 since that would be the start of the
        //outer rectangle.
        //Here we also make sure we draw the right tile in the tileset
        vec2 pp2=(pp1-vec2(s2,s2))*SQRT2;
                
        vec2 foo =  (pp2+ texture_offset)*grid_dim2;
        // vec4 bcol_1 = bcol + vec4(0.5,0.5,0.5,0.5); 
        gl_FragColor=texture2D(tex0,foo)*bcol;
    }
}
";

#[repr(packed(4))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub pos: [f32; 2], //TODO use half floats??
    pub index: u16,
    pub rotation: u16,
}

#[derive(Debug)]
pub struct SpriteProgram {
    pub program: GLuint,
    pub matrix_uniform: GLint,
    pub square_uniform: GLint,
    pub offset_uniform: GLint,
    pub point_size_uniform: GLint,
    pub grid_dim_uniform: GLint,
    pub sprite_dim_uniform: GLint,
    pub bcol_uniform: GLint,
    pub pos_attr: GLint,
    pub rotation_attr: GLint,
    pub index_attr: GLint,
    pub sample_location: GLint,
}

#[derive(Debug)]
pub struct PointMul(pub f32);

#[derive(Copy, Clone, Debug)]
pub struct SpriteProgramUniformValues<'a> {
    pub texture: &'a crate::sprite::Texture,
    pub radius: f32,
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
        common: &UniformCommon,
        un: &SpriteProgramUniformValues,
        buffer_info: BufferInfo,
    ) {
        let col = common.color;
        let buffer_id = buffer_info.id;
        let length = buffer_info.length;
        let point_size = un.radius;
        let mode = gl::POINTS;
        let texture = un.texture;
        let texture_id = un.texture.id;
        let offset = common.offset;

        unsafe {
            gl::UseProgram(self.program);
            gl_ok!();

            gl::Uniform1f(self.point_size_uniform, point_size);
            gl_ok!();

            gl::Uniform2f(self.offset_uniform, offset.x, offset.y);
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

            assert_eq!(core::mem::size_of::<Vertex>(), 12);

            let sx = texture.dim[0] / (texture.grid_dim[0] as f32);
            let sy = texture.dim[1] / (texture.grid_dim[1] as f32);

            let sprite_dim = if sx > sy {
                [1.0, sy / sx]
            } else {
                [sx / sy, 1.0]
            };

            //Either the x or y component MUST be 1.0
            //The other component must be less than or equal to 1.0.
            gl::Uniform2f(self.sprite_dim_uniform, sprite_dim[0], sprite_dim[1]);
            gl_ok!();

            gl::Uniform2f(
                self.grid_dim_uniform,
                texture.grid_dim[0] as f32,
                texture.grid_dim[1] as f32,
            );
            gl_ok!();

            gl::EnableVertexAttribArray(self.pos_attr as GLuint);
            gl_ok!();

            gl::VertexAttribPointer(
                self.pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                12 as i32,
                0 as *const _,
            );
            gl_ok!();

            gl::EnableVertexAttribArray(self.index_attr as GLuint);
            gl_ok!();

            gl::VertexAttribPointer(
                self.index_attr as GLuint,
                1,
                gl::UNSIGNED_SHORT,
                gl::FALSE as GLboolean,
                12 as i32,
                (4 * 2) as *const _,
            );
            gl_ok!();

            gl::EnableVertexAttribArray(self.rotation_attr as GLuint);
            gl_ok!();

            gl::VertexAttribPointer(
                self.rotation_attr as GLuint,
                1,
                gl::UNSIGNED_SHORT,
                gl::TRUE as GLboolean,
                12 as i32,
                ((4 * 2) + 2) as *const _,
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

            let sprite_dim_uniform: GLint =
                gl::GetUniformLocation(program, CString::new("sprite_dim").unwrap().as_ptr());
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

            let index_attr = gl::GetAttribLocation(program, CString::new("cellindex").unwrap().as_ptr());
            gl_ok!();

            let rotation_attr = gl::GetAttribLocation(program, CString::new("rotation").unwrap().as_ptr());
            gl_ok!();

            let sample_location =
                gl::GetAttribLocation(program, CString::new("tex0").unwrap().as_ptr());
            gl_ok!();

            SpriteProgram {
                sample_location,
                program,
                square_uniform,
                rotation_attr,
                offset_uniform,
                point_size_uniform,
                grid_dim_uniform,
                sprite_dim_uniform,
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
