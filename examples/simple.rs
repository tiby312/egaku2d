extern crate egaku2d;

use glutin::event::{Event,VirtualKeyCode,WindowEvent};
use glutin::event_loop::ControlFlow;


fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let mut sys = egaku2d::WindowedSystem::new([640, 480], &events_loop, "shapes example");

    //Draw 60 frames per second.
    let mut timer = egaku2d::RefreshTimer::new(16);

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


                canvas.clear_color([0.2; 3]);


                let mut circles=canvas.circles();

                for x in (20..400).step_by(20){
                    for y in (20..400).step_by(20){
                        circles.add([cursor[0]+(x as f32)*(cursor[0]*0.01),cursor[1]+y as f32]);
                    }
                }

                circles.uniforms(canvas,10.0).with_color([1.0,1.0,1.0,1.0]).send_and_draw();

                //display what we drew
                sys.swap_buffers();

            }
        }
        _ => {}
    });
}
