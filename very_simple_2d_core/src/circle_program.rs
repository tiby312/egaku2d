use axgeom;
use axgeom::*;
use crate::shader::*;
use crate::gl;
use crate::gl::types::*;
use std::str;
use std::ffi::CString;




// Shader sources
static VS_SRC: &'static str = "
#version 300 es
in vec2 position;
uniform mat3 mmatrix;
uniform float point_size;
in float alpha;
out float alpha2;
void main() {
    gl_PointSize = point_size;
    vec3 pp=vec3(position,1.0);
    gl_Position = vec4(mmatrix*pp.xyz, 1.0);
    alpha2=alpha;
}";



//https://blog.lapingames.com/draw-circle-glsl-shader/
static FS_SRC: &'static str = "
#version 300 es
precision mediump float;
in float alpha2;
uniform vec3 bcol;
out vec4 out_color;
uniform bool square;
void main() {

    vec2 coord = gl_PointCoord - vec2(0.5);
    
    if (square){
        float dis=dot(coord,coord);
        if(dis > 0.25)                  //outside of circle radius?
            discard;
    }

    out_color = vec4(bcol,alpha2);
}";


#[repr(transparent)]
#[derive(Copy,Clone,Debug,Default)]
pub struct Vertex(pub [f32;3]);


pub struct CircleProgram{
    pub program:GLuint,
    pub matrix_uniform:GLint,
    pub square_uniform:GLint,
    pub point_size_uniform:GLint,
    pub bcol_uniform:GLint,
    pub pos_attr:GLint,
    pub alpha_attr:GLint,
}

impl CircleProgram{
    pub fn set_viewport(&mut self,game_world:Rect<f32>,window_dim:Vec2<f32>){
        let _width=window_dim.x as f32;
        let _height=window_dim.y as f32;

        let ((x1,x2),(y1,y2))=game_world.get();
        let w=x2-x1;
        let h=y2-y1;

        let scalex=2.0/w;
        let scaley=2.0/h;

        let tx=-(1.+x1/(w/2.0));
        let ty=1.+y1/(h/2.0);
        
        let matrix= [
                    [scalex, 0.0, 0.0],
                    [0.0, -scaley,0.0],
                    [tx,ty,1.0]
                ];  

        unsafe{
            gl::UniformMatrix3fv(self.matrix_uniform,1, 0,std::mem::transmute(&matrix[0][0]));
            gl_ok!();
        }
    }


    pub fn new()->CircleProgram{
        unsafe{
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

            let square_uniform:GLint = gl::GetUniformLocation(program, CString::new("square").unwrap().as_ptr());
            gl_ok!();
        
            
            let point_size_uniform:GLint = gl::GetUniformLocation(program, CString::new("point_size").unwrap().as_ptr());
            gl_ok!();
            
            let matrix_uniform:GLint = gl::GetUniformLocation(program, CString::new("mmatrix").unwrap().as_ptr());
            gl_ok!();
            
            let bcol_uniform:GLint = gl::GetUniformLocation(program, CString::new("bcol").unwrap().as_ptr());
            gl_ok!();

            let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
            gl_ok!();

            let alpha_attr = gl::GetAttribLocation(program, CString::new("alpha").unwrap().as_ptr());
            gl_ok!();   

            CircleProgram{program,square_uniform,point_size_uniform,matrix_uniform,bcol_uniform,pos_attr,alpha_attr}
        }
    }
}

impl Drop for CircleProgram{
    fn drop(&mut self){
        // Cleanup
        unsafe {
            gl::DeleteProgram(self.program);
            gl_ok!();
        }
    }
}










