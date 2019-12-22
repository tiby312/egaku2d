//! ## Overview
//!
//! A library that lets you draw various simple 2d geometry primitives fast using a single
//! shader program and a single vertex buffer object with a safe api (provided no other libray
//! is calling opengl functions). Uses the builder pattern for a convinient and expressive api.
//! Uses glutin and opengl es 3.0.
//!
//! ## Screenshot
//!
//! ![](https://raw.githubusercontent.com/tiby312/very_simple_2d/master/assets/screenshot.gif)
//!
//!
//! ## Example
//!
//! ```rust,no_run
//! use axgeom::*;
//! let events_loop = glutin::event_loop::EventLoop::new();
//! let mut glsys = very_simple_2d::WindowedSystem::new(vec2(600., 480.), &events_loop);
//!
//! let mut sys = glsys.session();
//!
//! //Draw some arrows.
//! sys.arrows([0.0, 1.0, 0.1, 0.5], 5.0)
//!   .add(vec2(40., 40.), vec2(40., 200.))
//!   .add(vec2(40., 40.), vec2(200., 40.))
//!   .draw();
//!
//! //Draw some circles.
//! sys.circles([0., 1., 1., 0.1], 4.0)
//!   .add(vec2(5.,6.))
//!   .add(vec2(7.,8.))
//!   .add(vec2(9.,5.))
//!   .draw();
//!
//! glsys.swap_buffers();
//! ```

use axgeom::*;
pub use glutin;
use glutin::PossiblyCurrent;
pub use very_simple_2d_core;
use very_simple_2d_core::gl;
pub use very_simple_2d_core::DrawSession;
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
    view_port_set: bool,
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

        std::thread::sleep(std::time::Duration::from_millis(500));

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        // Load the OpenGL function pointers
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
        assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);

        let glutin::dpi::LogicalSize { width, height } = windowed_context.window().inner_size();
        let game_world = Rect::new(0.0, width as f32, 0.0, height as f32);

        FullScreenSystem {
            windowed_context,
            inner: MySys::new(game_world),
            view_port_set: false,
        }
    }

    pub fn set_viewport_from_width(&mut self, width: f32) {
        let dim = self.get_dim().inner_as::<f32>();
        let aspect_ratio = dim.y / dim.x;

        let height = aspect_ratio * width;
        self.inner
            .set_viewport(dim.x, rect(0.0, width, 0.0, height));
        self.view_port_set = true;
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
        self.view_port_set = true;
    }

    pub fn session(&mut self) -> DrawSession {
        assert!(self.view_port_set);
        self.inner.draw_sys()
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

        std::thread::sleep(std::time::Duration::from_millis(500));

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        // Load the OpenGL function pointers
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
        assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);

        //let glutin::dpi::LogicalSize{width,height}=windowed_context.window().inner_size();

        //dbg!(width,height);

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

    pub fn session(&mut self) -> DrawSession {
        self.inner.draw_sys()
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
