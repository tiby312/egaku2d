extern crate egaku2d;
use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;

const COL1: [f32; 4] = [0.0, 1.0, 0.1, 0.3];
const COL2: [f32; 4] = [0.8, 0.8, 1.0, 1.0];
const COL3: [f32; 4] = [1.0, 0.0, 1.0, 0.4];
const COL4: [f32; 4] = [0.5, 1.0, 0.5, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let mut sys = egaku2d::WindowedSystem::new([640, 480], &events_loop, "shapes example");
    //let mut sys=egaku2d::FullScreenSystem::new(&events_loop);

    //Make a bunch of textures
    let sky = sys.texture("day_sky.png", [1, 1]).unwrap();
    let food_tex = sys.texture("food.png", [8, 8]).unwrap();
    let adventurer = sys.texture("adventurer.png", [7, 11]).unwrap();
    let ascii_tex = sys.texture("ascii.png", [16, 14]).unwrap();
    let tall_tiles_tex = sys.texture("tall_tiles.png", [2, 3]).unwrap();
    let fat_tiles_tex = sys.texture("fat_tiles.png", [2, 3]).unwrap();
    let leaves = sys.texture("leaves.png", [1, 1]).unwrap();

    //Make a bunch of static vbos
    let canvas = sys.canvas_mut();
    let background = { canvas.rects().add([0.0, 640.0, 0.0, 480.0]).save(canvas) };
    let rect_save = {
        let mut k = canvas.rects();
        k.add([400., 420., 410., 420.]);
        k.add([50., 100., 60., 80.]);
        k.add([300., 500., 30., 50.]);
        k.add([300., 500., 300., 500.]);
        k.save(canvas)
    };
    let square_save = {
        let mut k = canvas.squares();
        for x in (0..1000).step_by(100).map(|a| a as f32) {
            for y in (0..1000).step_by(100).map(|a| a as f32) {
                k.add([x, y]);
            }
        }
        k.save(canvas)
    };
    let arrow_save = {
        canvas
            .arrows(5.0)
            .add([40., 40.], [40., 200.])
            .add([40., 40.], [200., 40.])
            .save(canvas)
    };
    let line_save = {
        canvas
            .lines(3.0)
            .add([400., 0.], [600., 400.])
            .add([10., 300.], [300., 400.])
            .save(canvas)
    };
    let sprite_save = {
        let mut k = canvas.sprites();
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
        k.save(canvas)
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

                //draw static VBOs already on the gpu.
                background
                    .uniforms(canvas)
                    .with_texture(&sky, 2.0, [0.0; 2])
                    .draw();
                sprite_save
                    .uniforms(canvas, &food_tex, 32.0)
                    .with_color(COL4)
                    .with_offset([-wobble[0], -wobble[1]])
                    .draw();
                arrow_save.uniforms(canvas).draw();
                line_save.uniforms(canvas).with_color(COL2).draw();
                square_save.uniforms(canvas, 10.0).with_color(COL3).draw();
                rect_save
                    .uniforms(canvas)
                    .with_texture(&fat_tiles_tex, 2.0, wobble)
                    .with_color(WHITE)
                    .with_offset(wobble)
                    .draw();

                //Draw a bunch of dyanmic shapes.
                let mut builder = canvas.circles();
                for x in (0..1000).step_by(12).map(|a| a as f32) {
                    for y in (0..1000).step_by(12).map(|a| a as f32) {
                        let c = (counter as f32 + x + y) * 0.01;
                        let x = x + c.sin() * y * 0.1;
                        let y = y + c.cos() * x * 0.1;
                        builder.add([x, y]);
                    }
                }
                builder
                    .send_and_uniforms(canvas, 8.0)
                    .with_color(COL1)
                    .draw();

                let mut builder = canvas.sprites();
                for y in (100..500).step_by(80).map(|a| a as f32) {
                    for x in (100..500).step_by(80).map(|a| a as f32) {
                        let c = (counter as f32 + x + y) * 0.01;
                        let cc = ((counter as f32 + x + y) * 0.1) as u32;
                        let x = x + c.sin() * 20.0;
                        let y = y + c.cos() * 20.0;
                        builder.add([x, y], (cc % 64) as u16, c);
                    }
                }
                builder
                    .send_and_uniforms(canvas, &adventurer, 100.0)
                    .with_color(WHITE)
                    .draw();

                let mut builder = canvas.sprites();
                add_ascii(
                    [100., 400.],
                    20.0,
                    cc.cos() * 0.5 - 0.2,
                    "testing? TESTING!",
                    &mut builder,
                );
                builder.add([100., 100.], ascii_tex.coord_to_index([2, 2]), 1.0);
                builder.send_and_uniforms(canvas, &ascii_tex, 20.0).draw();

                let c = ((counter as f32 * 0.06).sin() * 100.0).abs();
                canvas
                    .circles()
                    .add(cursor)
                    .send_and_uniforms(canvas, c)
                    .with_texture(&leaves, 1.0, [0.0; 2])
                    .draw();

                //draw a moving line
                let c = counter as f32 * 0.07;
                canvas
                    .lines(10.)
                    .add([50., 500.], [500., 50. + c.sin() * 50.])
                    .send_and_uniforms(canvas)
                    .with_texture(&leaves, 4.0, [0.0; 2])
                    .draw();

                //draw a rotating arrow
                let c = counter as f32 * 0.04;
                let center = [400., 400.];

                let other = [center[0] + c.cos() * 80., center[1] + c.sin() * 80.];
                canvas
                    .arrows(10.0)
                    .add(center, other)
                    .send_and_uniforms(canvas)
                    .with_color(COL4)
                    .draw();

                canvas
                    .sprites()
                    .add([500., 200.], c as u16, c)
                    .send_and_uniforms(canvas, &fat_tiles_tex, 100.)
                    .draw();
                canvas
                    .sprites()
                    .add([500., 50.], c as u16, c)
                    .send_and_uniforms(canvas, &tall_tiles_tex, 100.)
                    .draw();

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
    for (i, a) in st.chars().enumerate() {
        let ascii = a as u8;
        assert!(ascii >= 32);
        sprites.add(cc, (ascii - 32) as u16, rotation + (i as f32 * 0.1));
        cc[0] += width;
    }
}
