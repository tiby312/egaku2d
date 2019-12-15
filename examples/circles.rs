
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
  let mut glsys=System::new(rect(0.,600.,0.,600.),&events_loop);


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
        {
          	let mut k=sys.circles(100.0,[0.,1.,1.,0.1]);
            for x in (0..1000).step_by(100){
              for y in (0..1000).step_by(100){
                k.add(vec2(x as f32,y as f32));
              }
            }
            k.draw();
        }
        {
          let mut k=sys.squares(100.0,[1.,0.,1.,0.1]);
          for x in (0..1000).step_by(100){
            for y in (0..1000).step_by(100){
              k.add(vec2(x as f32,y as f32));
            }
          }
          k.draw();
        }

        sys.lines(10.0,[0.,1.0,0.,0.3])
          .add(vec2(0.,0.),vec2(500.,500.))
          .add(vec2(40.,40.),vec2(500.,0.))
          .add(vec2(700.,500.),vec2(100.,100.)).draw();

        sys.lines(50.,[1.,1.,0.2,0.2])
          .add(vec2(50.,500.),vec2(500.,50.)).draw();

        let mut k=sys.rects([0.8,0.8,1.0,0.2]);
        k.add(rect(50.,200.,700.,800.));

        k.add(rect(800.,820.,700.,800.));
        k.draw();
        drop(k);

        glsys.swap_buffers();

         
			},
			_=>{}
		}
	});
	

}