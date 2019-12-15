use axgeom::*;

use gl::types::*;
use core::mem;

mod vbo;
mod shader;

#[macro_export]
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


pub struct SquareSession<'a>{
    sys:&'a mut MySys
}
impl<'a> SquareSession<'a>{
    #[inline(always)]
    pub fn add(&mut self,point:Vec2<f32>)->&mut Self{
        self.sys.circle_buffer.push(circle_program::Vertex([point.x,point.y]));
        self
    }
    pub fn draw(&mut self){

        self.sys.circle_buffer.update();
        
        unsafe{
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();
        
            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();
      

            
            
            //////
            gl::DrawArrays(gl::POINTS,0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
        
    }
}
impl<'a> Drop for SquareSession<'a>{
    fn drop(&mut self){
        self.sys.reset();
    }
}
pub struct CircleSession<'a>{
    sys:&'a mut MySys
}
impl<'a> Drop for CircleSession<'a>{
    fn drop(&mut self){
        self.sys.reset();
       
    }
}
impl<'a> CircleSession<'a>{
    pub fn draw(&mut self){
         self.sys.circle_buffer.update();
        
        unsafe{
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();
        
            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();
      

            
            
            //////
            gl::DrawArrays(gl::POINTS,0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
        
    }

    #[inline(always)]
    pub fn add(&mut self,point:Vec2<f32>)->&mut Self{
    	self.sys.circle_buffer.push(circle_program::Vertex([point.x,point.y]));
    	self
    }
}


pub struct RectSession<'a>{
    sys:&'a mut MySys
}
impl Drop for RectSession<'_>{
    fn drop(&mut self){
        self.sys.reset();
    }
}

impl RectSession<'_>{
    pub fn draw(&mut self){

        self.sys.circle_buffer.update();
        
        unsafe{
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();
        
            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();
      
            //////
            gl::DrawArrays(gl::TRIANGLES,0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
        
    }

    #[inline(always)]
    pub fn add(&mut self,rect:Rect<f32>)->&mut Self{
        let [a,b,c,d] = rect.get_corners();
        let arr=[a,b,c,c,d,a];

        for a in arr.iter(){
            self.sys.circle_buffer.push(circle_program::Vertex([a.x,a.y]));    
        }

        self
    }
}
pub struct LineSession<'a>{
	sys:&'a mut MySys,
    radius:f32
}
impl Drop for LineSession<'_>{
    fn drop(&mut self){
        self.sys.reset();
    }
}
impl LineSession<'_>{
    pub fn draw(&mut self){

        self.sys.circle_buffer.update();
        
        unsafe{
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();
        
            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();
      
            //////
            gl::DrawArrays(gl::TRIANGLES,0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
        
    }

    #[inline(always)]
	pub fn add(&mut self,start:Vec2<f32>,end:Vec2<f32>)->&mut Self{
        let radius=self.radius;
        let offset=end-start;
        let k=offset.rotate_90deg_right().normalize_to(1.0);
        let start1=start+k*radius;
        let start2=start-k*radius;

        let end1=end+k*radius;
        let end2=end-k*radius;

        let arr=[start1,start2,end1,start2,end1,end2];

        for a in arr.iter(){
            self.sys.circle_buffer.push(circle_program::Vertex([a.x,a.y]));    
        }
        self
	}
}


pub struct DrawSession<'a>{
	sys:&'a mut MySys
}
impl DrawSession<'_>{
    pub fn circles(&mut self,radius:f32,color:[f32;4])->CircleSession{
    	unsafe{

            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();


    		gl::Uniform1f(self.sys.circle_program.point_size_uniform,radius*2.5);
        	gl_ok!();
            gl::Uniform4fv(self.sys.circle_program.bcol_uniform,1,std::mem::transmute(&color[0]));
            gl_ok!();

            let square=1;
            gl::Uniform1i(self.sys.circle_program.square_uniform,square);
            gl_ok!();
            

    	}
    	
        CircleSession{sys:self.sys}
    }
    pub fn squares(&mut self,radius:f32,color:[f32;4])->SquareSession{
        unsafe{

            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            gl::Uniform1f(self.sys.circle_program.point_size_uniform,radius*2.5);
            gl_ok!();
            gl::Uniform4fv(self.sys.circle_program.bcol_uniform,1,std::mem::transmute(&color[0]));
            gl_ok!();

            let square=0;
            gl::Uniform1i(self.sys.circle_program.square_uniform,square);
            gl_ok!();
            

        }
        
        SquareSession{sys:self.sys}
    }


    pub fn rects(&mut self,color:[f32;4])->RectSession{
        let _kk=self.sys.point_mul.0;
            
        unsafe{
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            gl::Uniform1f(self.sys.circle_program.point_size_uniform,0.0);
            gl_ok!();
            gl::Uniform4fv(self.sys.circle_program.bcol_uniform,1,std::mem::transmute(&color[0]));
            gl_ok!();

            let square=0;
            gl::Uniform1i(self.sys.circle_program.square_uniform,square);
            gl_ok!();
            

        }

        RectSession{sys:self.sys}
    }

    pub fn lines(&mut self,radius:f32,color:[f32;4])->LineSession{
        let kk=self.sys.point_mul.0;
            
        unsafe{
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            gl::Uniform1f(self.sys.circle_program.point_size_uniform,radius*kk);
            gl_ok!();
            gl::Uniform4fv(self.sys.circle_program.bcol_uniform,1,std::mem::transmute(&color[0]));
            gl_ok!();

            let square=0;
            gl::Uniform1i(self.sys.circle_program.square_uniform,square);
            gl_ok!();
            

        }

        LineSession{sys:self.sys,radius:radius*kk}
    }
}

pub struct MySys{
	back_color:[f32;3],
	circle_program:CircleProgram,
    point_mul:PointMul,
	circle_buffer:vbo::GrowableBuffer<circle_program::Vertex>,
}
impl MySys{
    fn reset(&mut self){       
        self.circle_buffer.clear();
    }
    pub fn new(dim:Rect<f32>)->MySys{

    	let circle_buffer=vbo::GrowableBuffer::new();
    	let mut circle_program=CircleProgram::new();
        let point_mul=circle_program.set_viewport(dim);

    	let back_color=[0.2;3];

        MySys{point_mul,back_color,circle_program,circle_buffer}
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



