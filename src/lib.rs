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
//! Lines                     | `(point,point)`               
//! Arrows                    | `(point_start,point_end)`               
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
//! Additionally there are the following functions:
//!
//! * `save()` - **Slow** function that creates a static VBO contains the shapes added from `add()`.
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
//! In windowed and fullscreen mode, the dimenions of the window defaults to scale exactly to the world.
//! For example, if the user made a window of size 800,600, and then drew a circle at 400,300, the 
//! circle would appear in the center of the window.
//! Similarily, if the user had a monitor with a resolution of 800,600 and started in fullscreen mode,
//! and drew a circle at 400,300, it would also appear in the center of the screen.
//!
//! The ratio between the scale of x and y are fixed to be 1:1 so that there is no distortion in the
//! shapes. The user can manually set the scale either by x or y and the other axis is automaically inferred
//! so that to keep a 1:1 ratio.
//!
//! # Example
//!
//! ```rust,no_run
//! use axgeom::*;
//! let events_loop = glutin::event_loop::EventLoop::new();
//! let mut glsys = very_simple_2d::WindowedSystem::new(vec2(600., 480.), &events_loop);
//!
//! let mut sys = glsys.inner_mut();
//!
//! //Make the background dark gray.
//! sys.clear_color([0.2,0.2,0.2]);
//! 
//! //Push some squares to a static vertex buffer object on the gpu.
//! let rect_save = sys.squares([0.0, 1.0, 0.1, 0.5], 5.0)
//!   .addp(40., 40.)
//!   .addp(40., 40.)
//!   .save();
//!
//! //Draw the squares we saved.
//! rect_save.draw(&mut sys);
//!
//! //Draw some arrows.
//! sys.arrows([0.0, 1.0, 0.1, 0.5], 5.0)
//!   .add(vec2(40., 40.), vec2(40., 200.))
//!   .add(vec2(40., 40.), vec2(200., 40.))
//!   .send_and_draw();
//!
//! //Draw some circles.
//! sys.circles([0., 1., 1., 0.1], 4.0)
//!   .add(vec2(5.,6.))
//!   .add(vec2(7.,8.))
//!   .add(vec2(9.,5.))
//!   .send_and_draw();
//!
//! //Draw some circles from f32 primitives.
//! sys.circles([0., 1., 1., 0.1], 4.0)
//!   .addp(5.,6.)
//!   .addp(7.,8.)
//!   .addp(9.,5.)
//!   .send_and_draw();
//!
//! //Swap buffers on the opengl context.
//! glsys.swap_buffers();
//! ```

use axgeom::*;
pub use glutin;
use glutin::PossiblyCurrent;
pub use very_simple_2d_core;
use very_simple_2d_core::gl;
use very_simple_2d_core::MySys;

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
pub struct FullScreenSystem {
    inner: MySys,
    windowed_context: glutin::WindowedContext<PossiblyCurrent>,
}
impl FullScreenSystem {
    pub fn new(events_loop: &glutin::event_loop::EventLoop<()>) -> Self {
        use glutin::window::Fullscreen;
        let fullscreen = Fullscreen::Borderless(prompt_for_monitor(events_loop));

        let gl_window = glutin::window::WindowBuilder::new().with_fullscreen(Some(fullscreen));

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

        let glutin::dpi::LogicalSize { width, height } = windowed_context.window().inner_size();
        let game_world = Rect::new(0.0, width as f32, 0.0, height as f32);

        let mut f = FullScreenSystem {
            windowed_context,
            inner: MySys::new(game_world),
        };

        f.set_viewport_from_width(width as f32);

        f
    }

    pub fn set_viewport_from_width(&mut self, width: f32) {
        let dim = self.get_dim().inner_as::<f32>();
        let aspect_ratio = dim.y / dim.x;

        let height = aspect_ratio * width;
        self.inner
            .set_viewport(dim.x, rect(0.0, width, 0.0, height));
    }

    pub fn set_viewport_min(&mut self, d: f32) {
        if self.get_dim().x < self.get_dim().y {
            self.set_viewport_from_width(d);
        } else {
            self.set_viewport_from_height(d);
        }
    }

    pub fn set_viewport_from_height(&mut self, height: f32) {
        let dim = self.get_dim().inner_as::<f32>();
        let aspect_ratio = dim.x / dim.y;

        let width = aspect_ratio * height;
        self.inner
            .set_viewport(dim.x, rect(0.0, width, 0.0, height));
    }

    pub fn inner_mut(&mut self) -> &mut MySys {
        &mut self.inner
    }

    pub fn get_dim(&self) -> Vec2<usize> {
        let glutin::dpi::LogicalSize { width, height } =
            self.windowed_context.window().inner_size();
        vec2(width as usize, height as usize)
    }
    pub fn swap_buffers(&mut self) {
        self.windowed_context.swap_buffers().unwrap();
        assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    }
}

///A version where the user can control the size of the window.
pub struct WindowedSystem {
    inner: MySys,
    windowed_context: glutin::WindowedContext<PossiblyCurrent>,
}

impl WindowedSystem {
    pub fn new(dim: Vec2<f32>, events_loop: &glutin::event_loop::EventLoop<()>) -> WindowedSystem {
        let game_world = Rect::new(0.0, dim.x, 0.0, dim.y);
        //use glutin::window::Fullscreen;
        //let fullscreen = Fullscreen::Borderless(prompt_for_monitor(events_loop));

        let width = game_world.x.distance() as f64;
        let height = game_world.y.distance() as f64;
        let gl_window = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize { width, height })
            .with_resizable(false)
            .with_title("very_simple_2d");

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

        WindowedSystem {
            windowed_context,
            inner: MySys::new(game_world),
        }
    }

    pub fn set_viewport_from_height(&mut self, height: f32) {
        let dim = self.get_dim().inner_as::<f32>();
        let aspect_ratio = dim.x / dim.y;

        let width = aspect_ratio * height;
        self.inner
            .set_viewport(dim.x, rect(0.0, width, 0.0, height));
    }

    pub fn set_viewport_from_width(&mut self, width: f32) {
        let dim = self.get_dim().inner_as::<f32>();
        let aspect_ratio = dim.y / dim.x;

        let height = aspect_ratio * width;
        self.inner
            .set_viewport(dim.x, rect(0.0, width, 0.0, height));
    }

    pub fn set_viewport_min(&mut self, d: f32) {
        if self.get_dim().x < self.get_dim().y {
            self.set_viewport_from_width(d);
        } else {
            self.set_viewport_from_height(d);
        }
    }

    pub fn get_dim(&self) -> Vec2<usize> {
        let glutin::dpi::LogicalSize { width, height } =
            self.windowed_context.window().inner_size();
        vec2(width as usize, height as usize)
    }

    pub fn inner_mut(&mut self) -> &mut MySys {
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
