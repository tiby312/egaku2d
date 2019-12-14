
extern crate very_simple_2d;
extern crate axgeom;

use axgeom::*;
use very_simple_2d::{System};

use glutin::event::Event;
use glutin::event_loop::ControlFlow;
use glutin::event::WindowEvent;
use glutin::event::VirtualKeyCode;

fn main()
{
	let events_loop = glutin::event_loop::EventLoop::new();
	let mut glsys=System::new(rect(0.,1920.,0.,1080.),&events_loop);


  events_loop.run(move |event,_,control_flow| {
  	match event{

			Event::WindowEvent{ event, .. } => match event {
  			WindowEvent::KeyboardInput{input,..}=>{       
                  match input.virtual_keycode{
                      Some(VirtualKeyCode::Escape)=>{
                          *control_flow=ControlFlow::Exit;
                      },
                      _=>{}
                  }
              },
  			WindowEvent::CloseRequested => {
  				dbg!("close requested");
  				*control_flow = ControlFlow::Exit;
  			},
  			_=>{}
			},
			Event::EventsCleared=>{
				
				let mut sys=glsys.get_sys();

      	let mut k=sys.circles(100.0,[0.,1.,1.]);
        for x in (0..1000).step_by(100){
          for y in (0..1000).step_by(100){
            k.draw(vec2(x as f32,y as f32),0.1);
          }
        }
        k.finish();

        let mut k=sys.squares(100.0,[1.,0.,1.]);
        for x in (0..1000).step_by(100){
          for y in (0..1000).step_by(100){
            k.draw(vec2(x as f32,y as f32),0.1);
          }
        }
        k.finish();

        sys.lines(10.0,[0.,1.0,0.])
          .draw(vec2(0.,0.),vec2(500.,500.),0.3)
          .draw(vec2(40.,40.),vec2(500.,0.),0.3)
          .draw(vec2(700.,500.),vec2(100.,100.),0.3)
          .finish();

        sys.lines(50.,[1.,1.,0.2])
          .draw(vec2(50.,500.),vec2(500.,50.),0.2)
          .finish();

        let mut k=sys.rects([0.8,0.8,1.0]);
        k.draw(rect(50.,200.,700.,800.),0.2);

        k.draw(rect(800.,820.,700.,800.),0.2);
        k.finish();

        glsys.swap_buffers();

         
			},
			_=>{}
		}
	});
	

}