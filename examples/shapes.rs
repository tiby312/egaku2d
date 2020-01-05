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
        let mut k = sys.canvas_mut().squares();
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

    /*
    let mut texture = sys.canvas_mut().texture("test1.png".to_string()).unwrap();
    
    let mut k=sys.canvas_mut().sprites();
    for x in (0..600).step_by(60){
        for y in (0..600).step_by(60){
            let (x,y)=(x as f32,y as f32);
            let l=0 as f32 * 0.06;
            k.addp(x+l.cos()*5.,y+l.sin()*5.,(x+y) as f32);
        }
    }

    let sprite_save=k.save();
    drop(k);
    */
    let mut food_tex = sys.canvas_mut().texture("food.png",vec2(8,8)).unwrap();
    let mut adventurer_tex = sys.canvas_mut().texture("adventurer-sheet.png",vec2(7,11)).unwrap();


    let mut k=sys.canvas_mut().sprites();
    let mut cc=0;
    for y in (032..200).step_by(32){
        for x in (032..200).step_by(32){
            k.add(vec2(x,y).inner_as(),cc % 64);
            cc+=1;
        }
    }
    
    let sprite_save=k.save();
    
    drop(k);

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

                canvas.clear_color([0.2;3]);

                const COL1:[f32;4]=[0.0,1.0,0.1,0.1];
                const COL2:[f32;4]=[0.8,0.8,1.0,0.4];
                const COL3:[f32;4]=[1.0,0.0,1.0,0.4];
                const COL4:[f32;4]=[0.5,1.0,0.5,0.6];
                const WHITE:[f32;4]=[1.0,1.0,1.0,0.8];
              
                //draw static VBOs already on the gpu.
                sprite_save.draw(&mut canvas,&mut food_tex,COL1,32.0);
                arrow_save.draw(&mut canvas, COL1);
                line_save.draw(&mut canvas, COL2);
                square_save.draw(&mut canvas, COL3,10.0);
                rect_save.draw(&mut canvas, COL4);
                                
                {
                    //Draw some moving circles
                    let mut k = canvas.circles();
                    for x in (0..1000).step_by(12) {
                        for y in (0..1000).step_by(12) {
                            let c = (counter + x + y) as f32 * 0.01;

                            let pos = vec2(x, y).inner_as();

                            k.add(pos + vec2(c.sin() * y as f32 * 0.1, c.cos() * x as f32 * 0.1));
                        }
                    }
                    k.send_and_draw(COL1,8.0);
                }
                
                
                canvas.sprites().addp(550.0,200.0,(counter as f32*0.05) as u32 % 40).send_and_draw(&mut adventurer_tex,WHITE,200.0);
                
                
                let mut k=canvas.sprites();
                
                for y in (100..500).step_by(40){
                    
                    for x in (100..500).step_by(40){
                        let c=(counter+x+y) as f32*0.01;
                        let pos = vec2(x, y).inner_as();

                        let cc=((counter+x+y) as f32*0.1 ) as u32;
                        k.add(pos+vec2(c.sin()*20.0,c.cos()*20.0),cc % 64);
                        
                    }
                }
                
                k.send_and_draw(&mut food_tex,WHITE,20.0);
                
                drop(k);
                

                
                {
                    //Draw a growing circle
                    let c = ((counter as f32 * 0.06).sin() * 40.0).abs();
                    canvas
                        .circles()
                        .add(cursor)
                        .send_and_draw(COL2,c);
                }


                {
                    //Draw a moving line
                    let c = counter as f32 * 0.07;
                    canvas
                        .lines(10.)
                        .add(vec2(50., 500.), vec2(500., 50. + c.sin() * 50.))
                        .send_and_draw(COL3);
                }

                
                {
                    //Draw a rotating arrow
                    let c = counter as f32 * 0.04;
                    let center = vec2(400., 400.);
                    canvas
                        .arrows(10.0)
                        .add(center, center + vec2(c.cos() * 80., c.sin() * 80.))
                        .send_and_draw(COL4);
                }
                
                
                
                //sprite_save.draw(canvas,&mut texture);
                
                //let sprite_save=k.save();
                //drop(k);


                //Display what we drew
                sys.swap_buffers();

                counter += 1;
            }
        }
        _ => {}
    });
}
