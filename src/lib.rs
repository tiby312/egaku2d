//! # Overview
//!
//! A library that lets you draw various simple 2d geometry primitives fast using a single
//! shader program and a single vertex buffer object with a safe api (provided no other libray
//! is calling opengl functions). Uses the builder pattern for a convinient api.
//! The main design goal is to be able to draw thousands of shapes efficiently.
//! Uses glutin and opengl es 3.0.
//!
//! ![](https://raw.githubusercontent.com/tiby312/very_simple_2d/master/assets/screenshot.gif)
//!
//! # User Guide
//!
//! The user can draw the following:
//!
//! Shape                     | Representation       
//! --------------------------|-----------------------------
//! Circles                   | `(point,radius)`              
//! Axis Aligned Rectangles   | `(startx,endx,starty,endy)`   
//! Axis Aligned Squares      | `(point,radius)`              
//! Lines                     | `(point,point,thickness)`               
//! Arrows                    | `(point_start,point_end,thickness)`               
//!   
//!
//! Each one of these follows the same simple api for drawing:
//!
//! * `add()` - **Fast** function that adds one shape to a Vec.
//! * `send_and_draw()` - **Slow** function that sends the Vec to the one vertex buffer object on the gpu and then draws them using DrawArrays.
//!
//! Using this api, the user can efficiently draw thousands of circles, for example, with the caveat that
//! they all will be the same radius and color/transparency values. This api does not allow the user
//! to efficiently draw thousands of circles where each circle has a different color or radius.
//! This was a design decision to make each vertex as lightweight as possible (just a x and y position),
//! making it more efficient to set and send to the gpu.
//!
//! Additionally there are the following functions to optionally save off verticies onto the gpu:
//!
//! * `save()` - **Slow** function that creates a new static VBO containing the shapes added from `add()`.
//! * `draw()` - **Fast** function that draws a static VBO. This is fast since the data already exists on the gpu.
//!
//! These functions allow the user to efficiently draw thousands of static objects by uploading all
//! of their shape data just once to the gpu. For dynamic shapes that move every step,
//! the user should use send_and_draw() every step.
//!
//! # View
//!
//! The top left corner is the origin (0,0) and x and y grow to the right and downwards respectively.
//!
//! In windowed mode, the dimenions of the window defaults to scale exactly to the world.
//! For example, if the user made a window of size 800,600, and then drew a circle at 400,300, the
//! circle would appear in the center of the window.
//! Similarily, if the user had a monitor with a resolution of 800,600 and started in fullscreen mode,
//! and drew a circle at 400,300, it would also appear in the center of the screen.
//!
//! The ratio between the scale of x and y are fixed to be 1:1 so that there is no distortion in the
//! shapes. The user can manually set the scale either by x or y and the other axis is automaically inferred
//! so that to keep a 1:1 ratio.
//!
//! # Fullscreen
//!
//! Fullscreen is kept behind a feature gate since on certain platforms like wayland linux it does not work.
//! I suspect this is a problem with glutin, so I have just disabled it for the time behing in the hope that
//! once glutin leaves alpha it will work. I think the problem is that when the window is resized, I can't manually change
//! the size of the context to match using resize().
//!
//! # Example
//!
//! ```rust,no_run
//! use axgeom::*;
//! let events_loop = glutin::event_loop::EventLoop::new();
//! let mut glsys = very_simple_2d::WindowedSystem::newp(600, 480, &events_loop,"test window");
//!
//! let mut canvas = glsys.canvas_mut();
//!
//! //Make the background dark gray.
//! canvas.clear_color([0.2,0.2,0.2]);
//!
//! //Push some squares to a static vertex buffer object on the gpu.
//! let rect_save = canvas.squares(5.0)
//!   .addp(40., 40.)
//!   .addp(40., 40.)
//!   .save();
//!
//! //Draw the squares we saved.
//! rect_save.draw(&mut canvas,[0.0, 1.0, 0.1, 0.5]);
//!
//! //Draw some arrows.
//! canvas.arrows(5.0)
//!   .add(vec2(40., 40.), vec2(40., 200.))
//!   .add(vec2(40., 40.), vec2(200., 40.))
//!   .send_and_draw([0.0, 1.0, 0.1, 0.5]);
//!
//! //Draw some circles.
//! canvas.circles(4.0)
//!   .add(vec2(5.,6.))
//!   .add(vec2(7.,8.))
//!   .add(vec2(9.,5.))
//!   .send_and_draw([0., 1., 1., 0.1]);
//!
//! //Draw some circles from f32 primitives.
//! canvas.circles(4.0)
//!   .addp(5.,6.)
//!   .addp(7.,8.)
//!   .addp(9.,5.)
//!   .send_and_draw([0., 1., 1., 0.1]);
//!
//! //Swap buffers on the opengl context.
//! glsys.swap_buffers();
//! ```

use axgeom::*;
pub use glutin;
use glutin::PossiblyCurrent;

use very_simple_2d_core;
use very_simple_2d_core::gl;

pub use very_simple_2d_core::shapes;
pub use very_simple_2d_core::SimpleCanvas;

///A timer to determine how often to refresh the screen.
///You pass it the desired refresh rate, then you can poll
///with is_ready() to determine if it is time to refresh.
pub struct RefreshTimer {
    interval: usize,
    last_time: std::time::Instant,
}
impl RefreshTimer {
    pub fn new(interval: usize) -> RefreshTimer {
        RefreshTimer {
            interval,
            last_time: std::time::Instant::now(),
        }
    }
    pub fn is_ready(&mut self) -> bool {
        if self.last_time.elapsed().as_millis() >= self.interval as u128 {
            self.last_time = std::time::Instant::now();
            true
        } else {
            false
        }
    }
}

///Unlike a windowed system, we do not have control over the dimensions of the
///window we end up with.
///After construction, the user must set the viewport using the window dimension
///information.
#[cfg(feature = "fullscreen")]
pub use self::fullscreen::FullScreenSystem;
#[cfg(feature = "fullscreen")]
pub mod fullscreen{
    pub struct FullScreenSystem {
        inner: SimpleCanvas,
        window_dim: FixedAspectVec2,
        windowed_context: Option<glutin::WindowedContext<PossiblyCurrent>>,
    }
    impl FullScreenSystem {
        pub fn new(events_loop: &glutin::event_loop::EventLoop<()>) -> Self {
            use glutin::window::Fullscreen;
            let fullscreen = Fullscreen::Borderless(prompt_for_monitor(events_loop));

            let gl_window = glutin::window::WindowBuilder::new().with_fullscreen(Some(fullscreen));


            //std::thread::sleep(std::time::Duration::from_millis(5000));

            //we are targeting only opengl 3.0 es. and glsl 300 es.

            let windowed_context = glutin::ContextBuilder::new()
                .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)))
                .with_vsync(true)
                .build_windowed(gl_window, &events_loop)
                .unwrap();


            

            let windowed_context = unsafe { windowed_context.make_current().unwrap() };




            let dpi = windowed_context.window().hidpi_factor();
            let glutin::dpi::PhysicalSize { width, height } =
                windowed_context.window().inner_size().to_physical(dpi);

            dbg!(width,height);


            // Load the OpenGL function pointers
            gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
            assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);




            let window_dim = axgeom::FixedAspectVec2 {
                ratio: AspectRatio(vec2(width, height)),
                width,
            };

            let windowed_context=Some(windowed_context);

            //let game_world = Rect::new(0.0, width as f32, 0.0, height as f32);
            let mut f = FullScreenSystem {
                windowed_context,
                window_dim,
                inner: unsafe { SimpleCanvas::new(window_dim) },
            };

            f.set_viewport_from_width(width as f32);

            f
        }

        //After this is called, you should update the viewport!!!!
        pub fn update_window_dim(&mut self){


            let dpi = self.windowed_context.as_ref().unwrap().window().hidpi_factor();
            
            let size =
                self.windowed_context.as_ref().unwrap().window().inner_size().to_physical(dpi);

            println!("resizing context!!! {:?}",(dpi,size));

            self.windowed_context.as_mut().unwrap().resize(size);
            self.window_dim=axgeom::FixedAspectVec2{ratio:AspectRatio(vec2(size.width,size.height)),width:size.width};

            let ctx = unsafe { self.windowed_context.take().unwrap().make_not_current().unwrap() };

            self.windowed_context = Some(unsafe { ctx.make_current().unwrap() });

        }


        pub fn set_viewport_from_width(&mut self, width: f32) {
            //let dim = self.get_dim().inner_as::<f32>();
            //let aspect_ratio = dim.y / dim.x;

            //let height = aspect_ratio * width;
            self.inner.set_viewport(self.window_dim, width);
        }

        pub fn set_viewport_min(&mut self, d: f32) {
            if self.get_dim().x < self.get_dim().y {
                self.set_viewport_from_width(d);
            } else {
                self.set_viewport_from_height(d);
            }
        }

        pub fn set_viewport_from_height(&mut self, height: f32) {
            //let dim = self.get_dim().inner_as::<f32>();
            //let aspect_ratio = dim.x / dim.y;

            //let width = aspect_ratio * height;
            let width = self.window_dim.ratio.width_over_height() as f32 * height;
            self.inner.set_viewport(self.window_dim, width);
        }

        pub fn canvas(&self) -> &SimpleCanvas {
            &self.inner
        }
        pub fn canvas_mut(&mut self) -> &mut SimpleCanvas {
            &mut self.inner
        }

        pub fn get_dim(&self) -> Vec2<usize> {
            self.window_dim.as_vec().inner_as()
        }
        pub fn swap_buffers(&mut self) {
            self.windowed_context.as_mut().unwrap().swap_buffers().unwrap();
            assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
        }
    }
    
    

}

///A version where the user can control the size of the window.
pub struct WindowedSystem {
    inner: SimpleCanvas,
    window_dim: FixedAspectVec2,
    windowed_context: glutin::WindowedContext<PossiblyCurrent>,
}

impl WindowedSystem {
    pub fn newp(
        dimx: usize,
        dimy: usize,
        events_loop: &glutin::event_loop::EventLoop<()>,
        title:&str
    ) -> WindowedSystem {
        Self::new(vec2(dimx, dimy), events_loop, title)
    }
    pub fn new(dim: Vec2<usize>, events_loop: &glutin::event_loop::EventLoop<()>,title:&str) -> WindowedSystem {
        let dim=dim.inner_as::<f32>();

        let game_world = Rect::new(0.0, dim.x, 0.0, dim.y);
        
        let width = game_world.x.distance() as f64;
        let height = game_world.y.distance() as f64;

        let monitor=prompt_for_monitor(events_loop);
        let dpi=monitor.hidpi_factor();
        let p=glutin::dpi::PhysicalSize{width,height}.to_logical(dpi);

        let gl_window = glutin::window::WindowBuilder::new()
            .with_inner_size(p)
            .with_resizable(false)
            .with_title(title);

        //we are targeting only opengl 3.0 es. and glsl 300 es.

        let windowed_context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)))
            .with_vsync(true)
            .build_windowed(gl_window, &events_loop)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        
        // Load the OpenGL function pointers
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
        assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);


        let dpi = windowed_context.window().hidpi_factor();
        let glutin::dpi::PhysicalSize { width, height } =
            windowed_context.window().inner_size().to_physical(dpi);
        assert_eq!(width as usize,dim.x as usize);
        assert_eq!(height as usize,dim.y as usize);


        let window_dim = axgeom::FixedAspectVec2 {
            ratio: AspectRatio(vec2(width, height)),
            width,
        };

        WindowedSystem {
            windowed_context,
            window_dim,
            inner: unsafe { SimpleCanvas::new(window_dim) },
        }
    }

    pub fn get_hidpi_factor(&self)->f64{
        self.windowed_context.window().hidpi_factor()
    }

    pub fn set_viewport_from_width(&mut self, width: f32) {
        self.inner.set_viewport(self.window_dim, width);
    }

    pub fn set_viewport_min(&mut self, d: f32) {
        if self.get_dim().x < self.get_dim().y {
            self.set_viewport_from_width(d);
        } else {
            self.set_viewport_from_height(d);
        }
    }

    pub fn set_viewport_from_height(&mut self, height: f32) {
        let width = self.window_dim.ratio.width_over_height() as f32 * height;
        self.inner.set_viewport(self.window_dim, width);
    }

    pub fn get_dimp(&self) -> [usize; 2] {
        let k=self.window_dim.as_vec().inner_as();
        [k.x,k.y]
    }
    pub fn get_dim(&self) -> Vec2<usize> {
        self.window_dim.as_vec().inner_as()
    }

    pub fn canvas(&self) -> &SimpleCanvas {
        &self.inner
    }
    pub fn canvas_mut(&mut self) -> &mut SimpleCanvas {
        &mut self.inner
    }
    pub fn swap_buffers(&mut self) {
        self.windowed_context.swap_buffers().unwrap();
        assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    }
}

use glutin::event_loop::EventLoop;
use glutin::monitor::MonitorHandle;

// Enumerate monitors and prompt user to choose one
fn prompt_for_monitor(el: &EventLoop<()>) -> MonitorHandle {
    let num = 0;
    let monitor = el
        .available_monitors()
        .nth(num)
        .expect("Please enter a valid ID");

    monitor
}