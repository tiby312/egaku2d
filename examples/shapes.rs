extern crate axgeom;
extern crate very_simple_2d;

use axgeom::*;
use glutin::event::Event;
use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use very_simple_2d::*;

fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let mut sys = WindowedSystem::newp(640, 480, &events_loop, "shapes example");
    //let mut sys=FullScreenSystem::new(&events_loop);


    let rect_save = {
        let mut k = sys.canvas_mut().rects();
        k.addp(400., 420., 300., 400.);
        k.addp(50., 100., 300., 350.);
        k.addp(5., 100., 30., 350.);
        k.save()
    };

    let square_save = {
        //Draw some squares
        let mut k = sys.canvas_mut().squares(10.0);
        for x in (0..1000).step_by(100) {
            for y in (0..1000).step_by(100) {
                k.addp(x as f32, y as f32);
            }
        }
        k.save()
    };

    let arrow_save = {
        //Draw some arrows
        sys.canvas_mut()
            .arrows(5.0)
            .add(vec2(40., 40.), vec2(40., 200.))
            .add(vec2(40., 40.), vec2(200., 40.))
            .save()
    };

    let line_save = {
        //Draw some lines
        sys.canvas_mut()
            .lines(3.0)
            .add(vec2(400., 0.), vec2(300., 10.))
            .add(vec2(10., 300.), vec2(300., 400.))
            .save()
    };

    let mut texture = sys.canvas_mut().texture("test1.png".to_string()).unwrap();
    
    let mut k=texture.sprites(sys.canvas_mut());
    for x in (0..600).step_by(20){
        for y in (0..600).step_by(20){
            let (x,y)=(x as f32,y as f32);
            let l=0 as f32 * 0.06;
            k.addp(x+l.cos()*5.,y+l.sin()*5.);
        }
    }
    
    let sprite_save=k.save();
    drop(k);
    //k.send_and_draw();
    //drop(k);


    let mut timer = very_simple_2d::RefreshTimer::new(16);

    let mut counter = 0;
    let mut cursor = vec2same(0.0);
    events_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            WindowEvent::CursorMoved {
                modifiers: _,
                device_id: _,
                position: logical_position,
            } => {
                let dpi = sys.get_hidpi_factor();
                let p = logical_position.to_physical(dpi);
                cursor = vec2(p.x, p.y).inner_as();
            }
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(_logical_size) => {}
            _ => {}
        },

        Event::EventsCleared => {
            if timer.is_ready() {
                let mut canvas = sys.canvas_mut();

                canvas.clear_color([0.2, 0.2, 0.2]);


              
                //Use this instead of clear_color for an interesting fade effect.
                //canvas.rects().addp(0.0,640.0,0.0,480.0).send_and_draw([0.2,0.2,0.2,0.3]);

                //draw static VBOs already on the gpu.
                arrow_save.draw(&mut canvas, [0.0, 1.0, 0.1, 0.5]);
                line_save.draw(&mut canvas, [0., 1.0, 1., 0.3]);
                square_save.draw(&mut canvas, [1., 0., 1., 0.1]);

                rect_save.draw(&mut canvas, [0.8, 0.8, 1.0, 0.2]);

                {
                    //Draw some moving circles
                    let mut k = canvas.circles(8.0);
                    for x in (0..1000).step_by(12) {
                        for y in (0..1000).step_by(12) {
                            let c = (counter + x + y) as f32 * 0.01;

                            let pos = vec2(x, y).inner_as();

                            k.add(pos + vec2(c.sin() * y as f32 * 0.1, c.cos() * x as f32 * 0.1));
                        }
                    }
                    k.send_and_draw([1., 1., 1., 0.1]);
                }

                {
                    //Draw a growing circle
                    let c = ((counter as f32 * 0.06).sin() * 40.0).abs();
                    canvas
                        .circles(c)
                        .add(cursor)
                        .send_and_draw([1.0, 1.0, 1.0, 1.0]);
                }

                {
                    //Draw a moving line
                    let c = counter as f32 * 0.07;
                    canvas
                        .lines(10.)
                        .add(vec2(50., 500.), vec2(500., 50. + c.sin() * 50.))
                        .send_and_draw([1., 1., 0.2, 0.2]);
                }
                {
                    //Draw a rotating arrow
                    let c = counter as f32 * 0.04;
                    let center = vec2(400., 400.);
                    canvas
                        .arrows(10.0)
                        .add(center, center + vec2(c.cos() * 80., c.sin() * 80.))
                        .send_and_draw([1.0, 0.1, 0.5, 0.5]);
                }

                sprite_save.draw(canvas,&mut texture);
                

                //Display what we drew
                sys.swap_buffers();

                counter += 1;
            }
        }
        _ => {}
    });
}
