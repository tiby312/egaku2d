extern crate very_simple_2d;

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
        k.add([400., 420.], [300., 400.]);
        k.add([50., 100.], [300., 350.]);
        k.add([5., 100.], [30., 350.]);
        k.save()
    };

    let square_save = {
        //Draw some squares
        let mut k = sys.canvas_mut().squares();
        for x in (0..1000).step_by(100).map(|a|a as f32) {
            for y in (0..1000).step_by(100).map(|a|a as f32) {
                k.add([x,y]);
            }
        }
        k.save()
    };

    let arrow_save = {
        //Draw some arrows
        sys.canvas_mut()
            .arrows(5.0)
            .add([40., 40.], [40., 200.])
            .add([40., 40.], [200., 40.])
            .save()
    };

    let line_save = {
        //Draw some lines
        sys.canvas_mut()
            .lines(3.0)
            .add([400., 0.], [600., 400.])
            .add([10., 300.], [300., 400.])
            .save()
    };

    let food_tex = sys.canvas_mut().texture("food.png", [8, 8]).unwrap();

    let sprite_save = {
        let mut k = sys.canvas_mut().sprites();
        for (i, x) in (032..200).step_by(32).enumerate().map(|(a,b)|(a as u32,b as f32)) {
            for (j, y) in (032..200).step_by(32).enumerate().map(|(a,b)|(a as u32,b as f32)) {
                k.add(
                    [x, y],
                    food_tex.coord_to_index([i, j]),
                );
            }
        }
        k.save()
    };

    //Draw 60 frames per second.
    let mut timer = very_simple_2d::RefreshTimer::new(16);

    let mut counter = 0;
    let mut cursor = [0.0;2];
    events_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            WindowEvent::CursorMoved {
                device_id: _,
                position: p,
                ..
            } => {
                cursor = [p.x as f32, p.y as f32];
            }
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(_logical_size) => {}
            _ => {}
        },

        Event::MainEventsCleared => {
            if timer.is_ready() {
                let canvas = sys.canvas_mut();

                canvas.clear_color([0.2; 3]);

                const COL1: [f32; 4] = [0.0, 1.0, 0.1, 0.1];
                const COL2: [f32; 4] = [0.8, 0.8, 1.0, 0.4];
                const COL3: [f32; 4] = [1.0, 0.0, 1.0, 0.4];
                const COL4: [f32; 4] = [0.5, 1.0, 0.5, 0.6];
                const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 0.8];

                //draw static VBOs already on the gpu.
                sprite_save.uniforms(canvas, &food_tex,16.0).with_color(COL4).draw();
                arrow_save.uniforms(canvas).with_color(COL1).draw();
                line_save.uniforms(canvas).with_color(COL2).draw();
                square_save.uniforms(canvas,10.0).with_color(COL3).draw();
                rect_save.uniforms(canvas).with_color(COL4);



                /*
                either a or b happens:

                case A:
                    1   : build up verticies 
                    1.5 : *optional*  save verticies
                case B:
                    1   : invoke saved off verticies
                
                2   :  set mandatory uniforms and set optional uniforms if desired
                3   :  draw
                */

                //sprite_save.uniforms(canvas,&food_tex,16.0).with_color(COL1).with_offset(vec2(5.,4.)).draw();

                {
                    //draw some moving circles
                    let mut k = canvas.circles();
                    for x in (0..1000).step_by(12).map(|a|a as f32) {
                        for y in (0..1000).step_by(12).map(|a|a as f32) {
                            let c = (counter as f32 + x + y) * 0.01;

                            let x=x+c.sin() * y * 0.1;
                            let y=y+c.cos() * x * 0.1;
                            
                            k.add([x,y]);
                        }
                    }
                    k.uniforms(8.0).with_color(COL1).send_and_draw();
                }

                {
                    //draw some moving sprites
                    let mut k = canvas.sprites();

                    for y in (100..500).step_by(40).map(|a|a as f32) {
                        for x in (100..500).step_by(40).map(|a|a as f32) {
                            let c = (counter as f32 + x + y) * 0.01;
                            
                            let cc = ((counter as f32 + x + y) * 0.1) as u32;

                            let x=x+c.sin() * 20.0;
                            let y=y+c.cos() * 20.0;

                            k.add(
                                [x,y],
                                sprite::TexIndex(cc % 64),
                            );
                        }
                    }

                    k.uniforms(&food_tex,20.0).with_color(WHITE).send_and_draw();
                }

                {
                    //draw a growing circle
                    let c = ((counter as f32 * 0.06).sin() * 40.0).abs();
                    canvas.circles().add(cursor).uniforms(c).with_color(COL2).send_and_draw();
                }

                {
                    //draw a moving line
                    let c = counter as f32 * 0.07;
                    canvas
                        .lines(10.)
                        .add([50., 500.], [500., 50. + c.sin() * 50.])
                        .uniforms()
                        .with_color(COL3)
                        .send_and_draw();
                }

                {
                    //draw a rotating arrow
                    let c = counter as f32 * 0.04;
                    let center = [400., 400.];

                    let other=[center[0]+c.cos() * 80.,center[1]+c.sin() * 80.];
                    canvas
                        .arrows(10.0)
                        .add(center, other)
                        .uniforms()
                        .with_color(COL4)
                        .send_and_draw();
                }

                //display what we drew
                sys.swap_buffers();

                counter += 1;
            }
        }
        _ => {}
    });
}
