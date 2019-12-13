
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
  				
  				glsys.draw(|mut sys|{
  					let mut k=sys.new_circle(100.0,[0.,1.,1.]);
  					k.draw_circle(vec2(0.,0.),1.0);
  					k.draw_circle(vec2(1000.,1000.),1.0);
  					k.draw_circle(vec2(-100.,-100.),1.0);
  					k.draw_circle(vec2(100.,100.),1.0);
  					k.finish();

  				});
				
  			},
  			_=>{}
  		}
  	});
	

}