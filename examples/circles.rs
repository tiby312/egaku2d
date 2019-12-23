extern crate axgeom;
extern crate very_simple_2d;

use axgeom::*;
use very_simple_2d::*;

use glutin::event::Event;
use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;

fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let mut glsys = WindowedSystem::new(vec2(600., 480.), &events_loop);
    //let mut glsys=FullScreenSystem::new(&events_loop);
    //glsys.set_viewport_min(600.0);

    let mut timer = very_simple_2d::RefreshTimer::new(16);

    let mut counter = 0;
    events_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        },
        Event::EventsCleared => {
            if timer.is_ready() {
                let mut sys = glsys.session();

                //Draw some arrows
                sys.arrows([0.0, 1.0, 0.1, 0.5], 5.0)
                    .add(vec2(40., 40.), vec2(40., 200.))
                    .add(vec2(40., 40.), vec2(200., 40.))
                    .draw();

                {
                    //Draw some moving circles
                    let mut k = sys.circles([0., 1., 1., 0.1], 4.0);
                    for x in (0..1000).step_by(12) {
                        for y in (0..1000).step_by(12) {
                            let c = (counter + x + y) as f32 * 0.01;

                            let pos = vec2(x, y).inner_as();
                            k.add(pos + vec2(c.sin() * y as f32 * 0.1, c.cos() * x as f32 * 0.1));
                        }
                    }
                    k.draw();
                }

                {
                    //Draw some squares
                    let mut k = sys.squares([1., 0., 1., 0.1], 10.0);
                    for x in (0..1000).step_by(100) {
                        for y in (0..1000).step_by(100) {
                            k.addp(x as f32, y as f32);
                        }
                    }
                    k.draw();
                }

                //Draw some lines
                sys.lines([0., 1.0, 1., 0.3], 3.0)
                    .add(vec2(400., 0.), vec2(300., 10.))
                    .add(vec2(10., 300.), vec2(300., 400.))
                    .draw();

                {
                    //Draw a moving line
                    let c = counter as f32 * 0.07;
                    sys.lines([1., 1., 0.2, 0.2], 10.)
                        .add(vec2(50., 500.), vec2(500., 50. + c.sin() * 50.))
                        .draw();
                }

                {
                    //Draw a rotating arrow
                    let c = counter as f32 * 0.04;
                    let center = vec2(400., 400.);
                    sys.arrows([1.0, 0.1, 0.5, 0.5], 10.0)
                        .add(center, center + vec2(c.cos() * 80., c.sin() * 80.))
                        .draw();
                }

                //Draw some rectangles
                let mut k = sys.rects([0.8, 0.8, 1.0, 0.2]);
                k.addp(50., 100., 300., 350.);
                k.addp(400., 420., 300., 400.);
                k.draw();
                drop(k);

                {
                    //Draw a growing circle
                    let c = ((counter as f32 * 0.06).sin() * 40.0).abs();
                    sys.circles([1.0, 1.0, 1.0, 1.0], c)
                        .addp(520., 400.)
                        .draw();
                }

                //Display what we drew
                glsys.swap_buffers();

                counter += 1;
            }
        }
        _ => {}
    });
}
