extern crate egaku2d;


use glutin::event::{Event,VirtualKeyCode,WindowEvent};
use glutin::event_loop::ControlFlow;

fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let mut sys = egaku2d::WindowedSystem::new([640, 480], &events_loop, "shapes example");
    //let mut sys=egaku2d::FullScreenSystem::new(&events_loop);
    let food_tex = sys.texture("food.png", [8, 8]).unwrap();
    let adventurer = sys.texture("adventurer.png", [7, 11]).unwrap();
    let ascii_tex = sys.texture("ascii.png", [16, 14]).unwrap();
    
    let tall_tiles_tex = sys.texture("tall_tiles.png",[2,3]).unwrap();
    let fat_tiles_tex = sys.texture("fat_tiles.png",[2,3]).unwrap();
    
    let rect_save = {
        let mut k = sys.canvas_mut().rects();
        k.add([400., 420., 410., 420.]);
        k.add([50., 100., 60., 80.]);
        k.add([300., 500., 30., 50.]);
        k.save()
    };

    let square_save = {
        //Draw some squares
        let mut k = sys.canvas_mut().squares();
        for x in (0..1000).step_by(100).map(|a| a as f32) {
            for y in (0..1000).step_by(100).map(|a| a as f32) {
                k.add([x, y]);
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

    let sprite_save = {
        let mut k = sys.canvas_mut().sprites();
        for (i, x) in (032..200)
            .step_by(32)
            .enumerate()
            .map(|(a, b)| (a as u8, b as f32))
        {
            for (j, y) in (032..200)
                .step_by(32)
                .enumerate()
                .map(|(a, b)| (a as u8, b as f32))
            {
                k.add([x, y], food_tex.coord_to_index([i, j]), 0.0);
            }
        }
        k.save()
    };

    //Draw 60 frames per second.
    let mut timer = egaku2d::RefreshTimer::new(16);

    let mut counter = 0;
    let mut cursor = [0.0; 2];
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
            WindowEvent::Resized(_dim) => {}
            _ => {}
        },

        Event::MainEventsCleared => {
            if timer.is_ready() {
                let canvas = sys.canvas_mut();

                let cc = counter as f32 * 0.1;
                let wobble = [cc.cos() * 10.0, cc.sin() * 10.0];

                canvas.clear_color([0.2; 3]);

                const COL1: [f32; 4] = [0.0, 1.0, 0.1, 0.1];
                const COL2: [f32; 4] = [0.8, 0.8, 1.0, 1.0];
                const COL3: [f32; 4] = [1.0, 0.0, 1.0, 0.4];
                const COL4: [f32; 4] = [0.5, 1.0, 0.5, 1.0];
                const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

                //draw static VBOs already on the gpu.
                sprite_save
                    .uniforms(canvas, &food_tex, 32.0)
                    .with_color(COL4)
                    .with_offset([-wobble[0], -wobble[1]])
                    .draw();
                arrow_save.uniforms(canvas).draw();
                line_save.uniforms(canvas).with_color(COL2).draw();
                square_save.uniforms(canvas, 10.0).with_color(COL3).draw();
                rect_save.uniforms(canvas).with_color(COL4).draw();

                //draw some moving circles
                let mut k = canvas.circles();
                for x in (0..1000).step_by(12).map(|a| a as f32) {
                    for y in (0..1000).step_by(12).map(|a| a as f32) {
                        let c = (counter as f32 + x + y) * 0.01;

                        let x = x + c.sin() * y * 0.1;
                        let y = y + c.cos() * x * 0.1;

                        k.add([x, y]);
                    }
                }
                k.uniforms(8.0).with_color(COL1).send_and_draw();

                //draw some moving sprites
                let mut k = canvas.sprites();

                for y in (100..500).step_by(80).map(|a| a as f32) {
                    for x in (100..500).step_by(80).map(|a| a as f32) {
                        let c = (counter as f32 + x + y) * 0.01;

                        let cc = ((counter as f32 + x + y) * 0.1) as u32;

                        let x = x + c.sin() * 20.0;
                        let y = y + c.cos() * 20.0;

                        k.add([x, y], (cc % 64) as u16, c);
                    }
                }

                k.uniforms(&adventurer, 100.0)
                    .with_color(WHITE)
                    .send_and_draw();

                let mut text = canvas.sprites();
                
                add_ascii([100., 400.], 20.0, cc.cos()*0.5-0.2, "testing? TESTING!", &mut text);
                text.add([100., 100.], ascii_tex.coord_to_index([2, 2]), 1.0);
                text.uniforms(&ascii_tex, 20.0).send_and_draw();

                //draw a growing circle
                let c = ((counter as f32 * 0.06).sin() * 100.0).abs();
                canvas
                    .circles()
                    .add(cursor)
                    .uniforms(c)
                    .with_color(WHITE)
                    .send_and_draw();

                //draw a moving line
                let c = counter as f32 * 0.07;
                canvas
                    .lines(10.)
                    .add([50., 500.], [500., 50. + c.sin() * 50.])
                    .uniforms()
                    .with_color(COL3)
                    .send_and_draw();

                //draw a rotating arrow
                let c = counter as f32 * 0.04;
                let center = [400., 400.];

                let other = [center[0] + c.cos() * 80., center[1] + c.sin() * 80.];
                canvas
                    .arrows(10.0)
                    .add(center, other)
                    .uniforms()
                    .with_color(COL4)
                    .send_and_draw();

                canvas.sprites().add([500.,200.],c as u16,c).uniforms(&fat_tiles_tex,100.).send_and_draw();

                canvas.sprites().add([500.,50.],c as u16,c).uniforms(&tall_tiles_tex,100.).send_and_draw();


                //display what we drew
                sys.swap_buffers();

                counter += 1;
            }
        }
        _ => {}
    });
}



fn add_ascii(
    start: [f32; 2],
    width: f32,
    rotation: f32,
    st: &str,
    sprites: &mut egaku2d::sprite::SpriteSession,
) {
    let mut cc = start;
    for (i,a) in st.chars().enumerate() {
        let ascii = a as u8;
        assert!(ascii >= 32);
        sprites.add(cc, (ascii - 32) as u16, rotation+(i as f32 * 0.1));
        cc[0] += width;
    }
}
