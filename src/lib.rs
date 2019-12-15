//!
//! A library that lets you draw various simple 2d geometry primitives fast using a single 
//! shader program and a single vertex buffer object with a safe api.
//!

pub use glutin;
pub use very_simple_2d_core;
pub use very_simple_2d_core::DrawSession;
use very_simple_2d_core::MySys;
use axgeom::*;
use glutin::PossiblyCurrent;
use very_simple_2d_core::gl;


pub struct RefreshTimer{
    interval:usize,
    last_time:std::time::Instant
}
impl RefreshTimer{
    pub fn new(interval:usize)->RefreshTimer{
        RefreshTimer{interval,last_time:std::time::Instant::now()}
    }
    pub fn is_ready(&mut self)->bool{
        if self.last_time.elapsed().as_millis()>=self.interval as u128{
            self.last_time=std::time::Instant::now();        
            true
        }else{
            false
        }
    }
}


pub struct System2{

}
impl System2{
    pub fn new()->Self{
        unimplemented!()
    }
    pub fn set_viewport(&mut self,_rect:Rect<f32>){
        unimplemented!()
    }
    pub fn dim(&self)->Vec2<usize>{
        unimplemented!()
    }
}



pub struct System{
	inner:MySys,
	windowed_context:glutin::WindowedContext<PossiblyCurrent>,
}

impl System{

	
	pub fn new(game_world:Rect<f32>,events_loop:&glutin::event_loop::EventLoop<()>)->System{

        //use glutin::window::Fullscreen;
        //let fullscreen = Fullscreen::Borderless(prompt_for_monitor(events_loop));

        let width=game_world.x.distance() as f64;
        let height=game_world.y.distance() as f64;
        let gl_window = glutin::window::WindowBuilder::new()
           .with_inner_size(glutin::dpi::LogicalSize{width,height})
           .with_resizable(false)
           .with_title("very_simple_2d");
         
        //we are targeting only opengl 3.0 es. and glsl 300 es.
        
        let windowed_context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)))
        .with_vsync(true)
        .build_windowed(gl_window,&events_loop).unwrap();


        std::thread::sleep(std::time::Duration::from_millis(500));
        
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };



        // Load the OpenGL function pointers
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
        assert_eq!(unsafe{gl::GetError()},gl::NO_ERROR);

        //let glutin::dpi::LogicalSize{width,height}=windowed_context.window().inner_size();
        
        //dbg!(width,height);

        System{windowed_context,inner:MySys::new(game_world)}
	}
    /*
	pub fn set_viewport(&mut self,game_world:Rect<f32>){
		self.inner.set_viewport(game_world,self.get_dim().inner_as())
	}
    */
    pub fn get_dim(&self)->Vec2<usize>{
        let glutin::dpi::LogicalSize{width,height}=self.windowed_context.window().inner_size();
        vec2(width as usize,height as usize)
    }

    pub fn get_sys(&mut self)->DrawSession{
        self.inner.draw_sys()
    }
    pub fn swap_buffers(&mut self){
        self.windowed_context.swap_buffers().unwrap();
        assert_eq!(unsafe{gl::GetError()},gl::NO_ERROR);
        
    }
    /*
	pub fn draw(&mut self,mut func:impl FnMut(DrawSession)){
		func(self.inner.draw_sys());
	
		self.windowed_context.swap_buffers().unwrap();
        assert_eq!(unsafe{gl::GetError()},gl::NO_ERROR);
	}
    */

}

/*
use glutin::event_loop::{EventLoop};
use glutin::monitor::{MonitorHandle};

// Enumerate monitors and prompt user to choose one
fn prompt_for_monitor(el: &EventLoop<()>) -> MonitorHandle {
    let num =0;
    let monitor = el
        .available_monitors()
        .nth(num)
        .expect("Please enter a valid ID");

    monitor
}
*/