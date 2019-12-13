pub use glutin;
use axgeom::*;
use very_simple_2d_core::DrawSession;
use very_simple_2d_core::MySys;
use glutin::PossiblyCurrent;
use very_simple_2d_core::gl;

pub struct System{
	inner:MySys,
	windowed_context:glutin::WindowedContext<PossiblyCurrent>,
}

impl System{

	
	pub fn new(game_world:Rect<f32>,events_loop:&glutin::event_loop::EventLoop<()>)->System{

        use glutin::window::Fullscreen;
        let fullscreen = Fullscreen::Borderless(prompt_for_monitor(events_loop));

        let gl_window = glutin::window::WindowBuilder::new()
            .with_fullscreen(Some(fullscreen));
         
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

        let glutin::dpi::LogicalSize{width,height}=windowed_context.window().inner_size();
        
        //dbg!(width,height);

        System{windowed_context,inner:MySys::new(game_world,vec2(width as f32,height as f32))}
	}

	pub fn set_viewport(&mut self,game_world:Rect<f32>){
		self.inner.set_viewport(game_world,self.get_dim().inner_as())
	}
    
    pub fn get_dim(&self)->Vec2<usize>{
        let glutin::dpi::LogicalSize{width,height}=self.windowed_context.window().inner_size();
        vec2(width as usize,height as usize)
    }
	pub fn draw(&mut self,mut func:impl FnMut(DrawSession)){
		func(self.inner.draw_sys());
	
		self.windowed_context.swap_buffers().unwrap();
        assert_eq!(unsafe{gl::GetError()},gl::NO_ERROR);
	}

}

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