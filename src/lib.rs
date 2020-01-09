//! # Overview
//!
//! A library that lets you draw various simple 2d geometry primitives and sprites fast using 
//! vertex buffer objects with a safe api (provided no other libray
//! is calling opengl functions). Uses the builder pattern for a convinient api.
//! The main design goal is to be able to draw thousands of shapes efficiently.
//! Uses glutin and opengl es 3.0.
//!
//! ![](https://raw.githubusercontent.com/tiby312/egaku2d/master/assets/screenshot.gif)
//!
//! # Pipeline 
//!
//! The egaku2d drawing pipeline works as follows:
//!
//! * 1. Pick a drawing type (a particular shape or a sprite) and set mandatory values for the particular shape or sprite.
//! * 2. Build up a large group of verticies by calling **`add()`**
//!     * 2.1 Optionally save off verticies to a static vbo on the gpu for fast drawing at a later time by calling **`save()`**.
//! * 3. Set mandatory shader uniform values bt calling **`uniforms()`**
//!     * 3.1 Set optional uniform values e.g. **`with_color()`**.
//! * 4. Send the vertex data to the gpu and draw by calling **`send_and_draw()`**
//!
//! Additionally, there is a way to draw the vertices we saved off to the gpu.
//! To do that, instead of steps 1 and 2, we use the saved off verticies,
//! and then set the uniform values and then draw by calling **`draw()`**. Drawing in this case is faster
//! since the vertex data already exists on the gpu.
//!
//!
//! Using this pipeline, the user can efficiently draw thousands of circles, for example, with the caveat that
//! they all will be the same radius and color/transparency values. This api does not allow the user
//! to efficiently draw thousands of circles where each circle has a different color or radius.
//! This was a design decision to make each vertex as lightweight as possible (just a x and y position),
//! making it more efficient to set and send to the gpu.
//!
//!
//!
//! # Using Shapes
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
//! # Using Sprites
//!
//! You can also draw sprites! You can upload a tileset texture to the gpu and then draw thousands of sprites
//! using a similar api to the shape drawing api. 
//! The sprites are point sprites drawn using the opengl POINTS primitive in order to cut down on the data
//! that needs to be sent to the gpu. The only information that is sent to the gpu on a sprite by sprite basis
//! is its position, and its tile index.
//!
//! While the user can pick different tile
//! coorinates to draw different sprints within the texture they upload to the gpu, they cannot rotate the sprite.
//! The sprite is drawn centered at the position the user specifies. They can change the size of all sprites in the 
//! draw session by changing its radius. See the example below.
//!
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
//!  
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
//! let mut glsys = egaku2d::WindowedSystem::new([600, 480], &events_loop,"test window");
//!
//! //Make a tileset texture from a png that has 64 different tiles.
//! let food_texture = glsys.texture("food.png",[8,8]).unwrap();
//!
//! let canvas = glsys.canvas_mut();
//!
//! //Make the background dark gray.
//! canvas.clear_color([0.2,0.2,0.2]);
//!
//! //Push some squares to a static vertex buffer object on the gpu.
//! let rect_save = canvas.squares()
//!   .add([40., 40.])
//!   .add([40., 40.])
//!   .save();
//!
//! //Draw the squares we saved.
//! rect_save.uniforms(canvas,5.0).with_color([0.0, 1.0, 0.1, 0.5]).draw();
//!
//! //Draw some arrows.
//! canvas.arrows(5.0)
//!   .add([40., 40.], [40., 200.])
//!   .add([40., 40.], [200., 40.])
//!   .uniforms().send_and_draw();
//!
//! //Draw some circles.
//! canvas.circles()
//!   .add([5.,6.])
//!   .add([7.,8.])
//!   .add([9.,5.])
//!   .uniforms(4.0).with_color([0., 1., 1., 0.1]).send_and_draw();
//!
//! //Draw some circles from f32 primitives.
//! canvas.circles()
//!   .add([5.,6.])
//!   .add([7.,8.])
//!   .add([9.,5.])
//!   .uniforms(4.0).with_color([0., 1., 1., 0.1]).send_and_draw();
//!
//! //Draw the first tile in the top left corder of the texture.
//! canvas.sprites().add([100.,100.],food_texture.coord_to_index([0,0])).uniforms(&food_texture,4.0).send_and_draw();
//!
//! //Swap buffers on the opengl context.
//! glsys.swap_buffers();
//! ```




use axgeom::*;
pub use glutin;
use glutin::PossiblyCurrent;

use egaku2d_core;
use egaku2d_core::gl;


pub use egaku2d_core::uniforms;
pub use egaku2d_core::shapes;
pub use egaku2d_core::sprite;
pub use egaku2d_core::SimpleCanvas;

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
pub mod fullscreen {
    use super::*;
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

            //let dpi = windowed_context.window().scale_factor();
            let glutin::dpi::PhysicalSize { width, height } =
                windowed_context.window().inner_size();

            dbg!(width, height);

            // Load the OpenGL function pointers
            gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
            assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);

            let window_dim = axgeom::FixedAspectVec2 {
                ratio: AspectRatio(vec2(width as f64, height as f64)),
                width:width as f64,
            };

            let windowed_context = Some(windowed_context);

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
        pub fn update_window_dim(&mut self) {
            let dpi = self
                .windowed_context
                .as_ref()
                .unwrap()
                .window()
                .scale_factor();

            let size = self
                .windowed_context
                .as_ref()
                .unwrap()
                .window()
                .inner_size();

            println!("resizing context!!! {:?}", (dpi, size));

            self.windowed_context.as_mut().unwrap().resize(size);
            self.window_dim = axgeom::FixedAspectVec2 {
                ratio: AspectRatio(vec2(size.width as f64, size.height as f64)),
                width: size.width as f64,
            };

            let ctx = unsafe {
                self.windowed_context
                    .take()
                    .unwrap()
                    .make_not_current()
                    .unwrap()
            };

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

        ///Creates a new texture from the specified file.
        ///The fact that we need a mutable reference to this object
        ///Ensures that we make the texture in the same thread.
        ///The grid dimensions passed are the tile dimensions is
        ///the texture is a tile set.
        pub fn texture(
            &mut self,
            file: &str,
            grid_dim: [u32;2],
        ) -> image::ImageResult<sprite::Texture> {
            crate::texture(file,grid_dim)
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
            self.windowed_context
                .as_mut()
                .unwrap()
                .swap_buffers()
                .unwrap();
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
    pub fn new(
        dim: [usize;2],
        events_loop: &glutin::event_loop::EventLoop<()>,
        title: &str,
    ) -> WindowedSystem {
        let dim=vec2(dim[0],dim[1]);
        let dim = dim.inner_as::<f32>();

        let game_world = Rect::new(0.0, dim.x, 0.0, dim.y);

        let width = game_world.x.distance() as f64;
        let height = game_world.y.distance() as f64;

        let monitor = prompt_for_monitor(events_loop);
        let dpi = monitor.scale_factor();
        let p : glutin::dpi::LogicalSize<f64>= glutin::dpi::PhysicalSize { width, height }.to_logical(dpi);

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

        //let dpi = windowed_context.window().scale_factor();
        let glutin::dpi::PhysicalSize { width, height } =
            windowed_context.window().inner_size();
        assert_eq!(width as usize, dim.x as usize);
        assert_eq!(height as usize, dim.y as usize);

        let window_dim = axgeom::FixedAspectVec2 {
            ratio: AspectRatio(vec2(width as f64, height as f64)),
            width:width as f64,
        };

        WindowedSystem {
            windowed_context,
            window_dim,
            inner: unsafe { SimpleCanvas::new(window_dim) },
        }
    }


    pub fn set_viewport_from_width(&mut self, width: f32) {
        self.inner.set_viewport(self.window_dim, width);
    }

    pub fn set_viewport_min(&mut self, d: f32) {
        if self.get_dim()[0] < self.get_dim()[1] {
            self.set_viewport_from_width(d);
        } else {
            self.set_viewport_from_height(d);
        }
    }

    pub fn set_viewport_from_height(&mut self, height: f32) {
        let width = self.window_dim.ratio.width_over_height() as f32 * height;
        self.inner.set_viewport(self.window_dim, width);
    }

    pub fn get_dim(&self) -> [usize;2] {
        let v=self.window_dim.as_vec().inner_as();
        [v.x,v.y]
    }

    ///Creates a new texture from the specified file.
    ///The fact that we need a mutable reference to this object
    ///Ensures that we make the texture in the same thread.
    ///The grid dimensions passed are the tile dimensions is
    ///the texture is a tile set.
    pub fn texture(
        &mut self,
        file: &str,
        grid_dim: [u32;2],
    ) -> image::ImageResult<sprite::Texture> {
        crate::texture(file,grid_dim)
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




use egaku2d_core::gl_ok;
use egaku2d_core::sprite::*;
use egaku2d_core::gl::types::GLuint;

///Creates a new texture from the specified file.
///The fact that we need a mutable reference to this object
///Ensures that we make the texture in the same thread.
///The grid dimensions passed are the tile dimensions is
///the texture is a tile set.
fn texture(
    file: &str,
    grid_dim: [u32;2],
) -> image::ImageResult<sprite::Texture> {

    match image::open(&file.to_string()) {
        Err(err) => Err(err),
        Ok(img) => {
            use image::GenericImageView;

            let (width, height) = img.dimensions();

            let img = match img {
                image::DynamicImage::ImageRgba8(img) => img,
                img => img.to_rgba(),
            };

            let id = build_opengl_mipmapped_texture(width, height, img);
            Ok(unsafe{Texture::new(id,grid_dim)})
        }
    }
}

fn build_opengl_mipmapped_texture(width: u32, height: u32, image: image::RgbaImage) -> GLuint {
    unsafe {
        let mut texture_id: GLuint = 0;
        gl::GenTextures(1, &mut texture_id);
        gl_ok!();

        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl_ok!();

        let raw = image.into_raw();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            raw.as_ptr() as *const _,
        );
        gl_ok!();

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl_ok!();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl_ok!();

        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl_ok!();

        texture_id
    }
}
