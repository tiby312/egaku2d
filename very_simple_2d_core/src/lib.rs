use axgeom::*;

use gl::types::*;
use core::mem;

mod vbo;
mod shader;


macro_rules! gl_ok {
    () => {
        assert_eq!(gl::GetError(),gl::NO_ERROR);
    };
}

use circle_program::CircleProgram;
use circle_program::PointMul;
mod circle_program;


pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
} 

pub struct CircleSession<'a>{
    sys:&'a mut MySys
}
impl<'a> CircleSession<'a>{

    pub fn draw_circle(&mut self,point:Vec2<f32>,alpha:f32)->&mut Self{
    	self.sys.circle_buffer.push(circle_program::Vertex([point.x,point.y,alpha]));
    	self
    }

    pub fn finish(&mut self){
    	unsafe{
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();
        
	        //TODO move this down more?
	        gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
	        gl_ok!();
	  

            self.sys.circle_buffer.update();
	        
            /////
	        gl::EnableVertexAttribArray(self.sys.circle_program.pos_attr as GLuint);
            gl_ok!();
	        gl::VertexAttribPointer(
	            self.sys.circle_program.pos_attr as GLuint,
	            2,
	            gl::FLOAT,
	            gl::FALSE as GLboolean,
	            3*mem::size_of::<f32>() as i32,
	            core::ptr::null(),
	        );
            gl_ok!();
	        /////
	        
	        gl::EnableVertexAttribArray(self.sys.circle_program.pos_attr as GLuint);
            gl_ok!();
	        gl::VertexAttribPointer(
	            self.sys.circle_program.pos_attr as GLuint,
	            1,
	            gl::FLOAT,
	            gl::FALSE as GLboolean,
	            3*mem::size_of::<f32>() as i32,
	            (2*mem::size_of::<f32>()) as *const std::ffi::c_void,
	        );
            gl_ok!();
	        
	        //////
            gl::DrawArrays(gl::POINTS,0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
    	}
        
        self.sys.circle_buffer.clear();
    }
}

pub struct LineSession<'a>{
	_sys:&'a mut MySys
}
impl LineSession<'_>{
	pub fn draw_line(&mut self,start:Vec2<f32>,end:Vec2<f32>){
		unimplemented!("{:?}",(start,end))
	}
}


pub struct DrawSession<'a>{
	sys:&'a mut MySys
}
impl DrawSession<'_>{
    pub fn new_circle(&mut self,radius:f32,color:[f32;3])->CircleSession{
    	unsafe{

            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

    		gl::Uniform1f(self.sys.circle_program.point_size_uniform,radius*self.sys.point_mul.0);
        	gl_ok!();
            gl::Uniform3fv(self.sys.circle_program.bcol_uniform,1,std::mem::transmute(&color[0]));
            gl_ok!();

            let square=0;
            gl::Uniform1i(self.sys.circle_program.square_uniform,square);
            gl_ok!();
            

    	}
    	
        CircleSession{sys:self.sys}
    }

    pub fn new_line(&mut self,radius:f32,color:[f32;4])->LineSession{
    	unimplemented!("{:?}",(radius,color))
    }
    pub fn finish(self){
    	//TODO swap buffers
    	unimplemented!()
    }
}

pub struct MySys{
	back_color:[f32;3],
	circle_program:CircleProgram,
    point_mul:PointMul,
	circle_buffer:vbo::GrowableBuffer<circle_program::Vertex>,
}
impl MySys{
    pub fn new(dim:Rect<f32>,window_dim:Vec2<f32>)->MySys{

    	let circle_buffer=vbo::GrowableBuffer::new();
    	let mut circle_program=CircleProgram::new();
        let point_mul=circle_program.set_viewport(dim,window_dim);

        dbg!(&point_mul);
        dbg!(&circle_program);
        dbg!(&circle_buffer);
    	let back_color=[0.2;3];

        MySys{point_mul,back_color,circle_program,circle_buffer}
    }
    pub fn set_viewport(&mut self,dim:Rect<f32>,window_dim:Vec2<f32>){
        self.circle_program.set_viewport(dim,window_dim);
    	//TODO add line program
    }
    pub fn draw_sys(&mut self)->DrawSession{

        let back_color=&self.back_color;
        unsafe{
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl_ok!();
            gl::Enable( gl::BLEND );
            gl_ok!();
            gl::ClearColor(back_color[0], back_color[1], back_color[2], 1.0);
            gl_ok!();
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl_ok!();
        }
        DrawSession{sys:self}
    }
}


