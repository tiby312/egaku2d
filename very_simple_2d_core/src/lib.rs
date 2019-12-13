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

    pub fn finish(self){
    	unsafe{
	        //TODO move this down more?
	        gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
	        
	  
	        /////
	        gl::EnableVertexAttribArray(self.sys.circle_program.pos_attr as GLuint);
	        gl::VertexAttribPointer(
	            self.sys.circle_program.pos_attr as GLuint,
	            2,
	            gl::FLOAT,
	            gl::FALSE as GLboolean,
	            3*mem::size_of::<f32>() as i32,
	            core::ptr::null(),
	        );
	        /////
	        
	        gl::EnableVertexAttribArray(self.sys.circle_program.pos_attr as GLuint);
	        gl::VertexAttribPointer(
	            self.sys.circle_program.pos_attr as GLuint,
	            1,
	            gl::FLOAT,
	            gl::FALSE as GLboolean,
	            3*mem::size_of::<f32>() as i32,
	            (2*mem::size_of::<f32>()) as *const std::ffi::c_void,
	        );
	        
	        //////
	        gl::DrawArrays(gl::POINTS,0 as i32, self.sys.circle_buffer.len() as i32);
    	}
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
    		gl::Uniform1f(self.sys.circle_program.point_size_uniform,radius);
        	gl_ok!();
            gl::Uniform3fv(self.sys.circle_program.bcol_uniform,1,std::mem::transmute(&color[0]));
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
	circle_buffer:vbo::GrowableBuffer<circle_program::Vertex>,
    //pub buffer:vbo::GrowableBuffer<Vertex>
}
impl MySys{
    pub fn new(dim:Rect<f32>,window_dim:Vec2<f32>)->MySys{

    	let circle_buffer=vbo::GrowableBuffer::new();
    	let circle_program=CircleProgram::new();
    	let back_color=[0.0;3];

    	let mut k = MySys{back_color,circle_program,circle_buffer};
    	k.set_viewport(dim,window_dim);
    	k
    }
    pub fn set_viewport(&mut self,dim:Rect<f32>,window_dim:Vec2<f32>){
        self.circle_program.set_viewport(dim,window_dim);
    	//TODO add line program
    }
    pub fn draw_sys(&mut self)->DrawSession{

        let back_color=&self.back_color;
        unsafe{
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable( gl::BLEND );

            gl::ClearColor(back_color[0], back_color[1], back_color[2], 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        DrawSession{sys:self}
    }
}


