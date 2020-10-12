//! # Overview
//!
//! A library that lets you draw various simple 2d geometry primitives and sprites fast using
//! vertex buffer objects with a safe api. Uses the builder pattern for a convinient api.
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
//! * 3. Send the vertex data to the gpu and set mandatory shader uniform values bt calling **`send_and_uniforms()`**
//!     * 3.1 Set optional uniform values e.g. **`with_color()`**.
//! * 4. Draw the verticies by calling **`draw()`**
//!
//! Additionally, there is a way to draw the vertices we saved off to the gpu.
//! To do that, instead of steps 1 and 2, we use the saved off verticies,
//! and then set the uniform values by valling **`uniforms()`** and then draw by calling **`draw()`**.
//!
//! Using this pipeline, the user can efficiently draw thousands of circles, for example, with the caveat that
//! they all will be the same radius and color/transparency values. This api does not allow the user
//! to efficiently draw thousands of circles where each circle has a different color or radius.
//! This was a design decision to make each vertex as lightweight as possible (just a x and y position),
//! making it more efficient to set and send to the gpu.
//!
//! # Key Design Goals
//!
//! The main goal was to make a very performat simple 2d graphics library.
//! There is special focus on reducing traffic between the cpu and the gpu by using compact vertices,
//! point sprites, and by allowing the user to save vertex data to the gpu on their own.
//!
//! Providing a safe api is also a goal. All draw functions require a mutable version to the canvas, ensuring
//! they happen sequentially. The user is prevented from making multiple instances of the system using an atomic counter.
//! The system also does not implement Send so that the drop calls from vertex buffers going out of scope happen sequentially
//! as well. If the user were to call opengl functions on their own, then some safety guarentees might be lost.
//! However, if the user does not, this api should be completely safe.
//!
//! Writing fast shader programs is a seconady goal. This is a 2d drawing library even though most of the hardware out there
//! is made to handle 3d. This means that the gpu is most likely under-utilized with this library.
//! Because of this, it was decided there is little point to make a non-rotatable sprite shader to save
//! on gpu time, for example. Especially since the vertex layout is the same size (with 32bit alignment) (`[f32;2],i16,i16` vs `[f32;2],i16`),
//! so there are no gains from having to send less data to the gpu.
//!
//! # Using Shapes
//!
//! The user can draw the following:
//!
//! Shape                     | Representation                        | Opengl Primitive Type
//! --------------------------|---------------------------------------|-----------------
//! Circles                   | `(point,radius)`                      | POINTS
//! Axis Aligned Rectangles   | `(startx,endx,starty,endy)`           | TRIANGLES
//! Axis Aligned Squares      | `(point,radius)`                      | POINTS
//! Lines                     | `(point,point,thickness)`             | TRIANGLES
//! Arrows                    | `(point_start,point_end,thickness)`   | TRIANGLES 
//!   
//! # Using Sprites
//!
//! This crate also allows the user to draw sprites. You can upload a tileset texture to the gpu and then draw thousands of sprites
//! using a similar api to the shape drawing api.
//! The sprites are point sprites drawn using the opengl POINTS primitive in order to cut down on the data
//! that needs to be sent to the gpu.
//!
//! Each sprite vertex is composed of the following:
//!
//! * position:`[f32;2]`
//! * index:`u16` - the user can index up to 256*256 different sprites in a tile set.
//! * rotation:`u16` - this gets normalized to a float internally. The user passes a f32 float in radians.
//!
//! So each sprite vertex is compact at 4*3=12 bytes.
//!
//! Each texture object has functions to create this index from a x and y coordinate.
//! On the gpu, the index will be split into a x and y coordinate.
//! If the index is larger than texture.dim.x*texture.dim.y then it will be modded so that
//! it can be mapped to a tile set. Therefore it is impossible for the index
//! to have a 'invalid' value. But obviously, the user should be picking an index
//! that maps to a valid tile in the tile set to begin with. 
//!
//! The rotation is normalized to a float on the gpu. The fact that the tile index has size u16,
//! means you can have a texture with a mamimum of 256x256 tiles. The user simply passes a f32 through
//! the api. The rotation is in radians with 0 being no rotation and grows with a clockwise rotation.
//! 
//!
//! # Batch drawing
//!
//! While you can pretty efficiently draw thousands of objects by calling add() a bunch of times,
//! you might already have all of the vertex data embeded somewhere, in which case it can seem
//! wasteful to iterate through your data structure to just build up another list that is then sent
//! to the gpu. egaku2d has `Batches` that lets you map verticies to an existing data structure that you might have.
//! This lets us skip building up a new verticies list by sending your entire data structure to the gpu.
//!
//! The downside to this approach is that you might have the vertex data in a list, but it might not be
//! tightly packed since you have a bunch of other  data associated with each element,
//! in which case we might end up sending a lot of useless data to the gpu.
//!
//! Currently this is only supported for circle drawing.
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
//!   .save(canvas);
//!
//! //Draw the squares we saved.
//! rect_save.uniforms(canvas,5.0).with_color([0.0, 1.0, 0.1, 0.5]).draw();
//!
//! //Draw some arrows.
//! canvas.arrows(5.0)
//!   .add([40., 40.], [40., 200.])
//!   .add([40., 40.], [200., 40.])
//!   .send_and_uniforms(canvas).draw();
//!
//! //Draw some circles.
//! canvas.circles()
//!   .add([5.,6.])
//!   .add([7.,8.])
//!   .add([9.,5.])
//!   .send_and_uniforms(canvas,4.0).with_color([0., 1., 1., 0.1]).draw();
//!
//! //Draw some circles from f32 primitives.
//! canvas.circles()
//!   .add([5.,6.])
//!   .add([7.,8.])
//!   .add([9.,5.])
//!   .send_and_uniforms(canvas,4.0).with_color([0., 1., 1., 0.1]).draw();
//!
//! //Draw the first tile in the top left corder of the texture.
//! canvas.sprites().add([100.,100.],food_texture.coord_to_index([0,0]),3.14).send_and_uniforms(canvas,&food_texture,4.0).draw();
//!
//! //Swap buffers on the opengl context.
//! glsys.swap_buffers();
//! ```

use egaku2d_core::axgeom;
pub use glutin;
use glutin::PossiblyCurrent;

use egaku2d_core;
use egaku2d_core::gl;

pub use egaku2d_core::batch;
pub use egaku2d_core::shapes;
pub use egaku2d_core::sprite;
pub use egaku2d_core::uniforms;
pub use egaku2d_core::SimpleCanvas;

mod onein {
    use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
    static INSTANCES: AtomicUsize = AtomicUsize::new(0);

    pub fn assert_only_one_instance() {
        assert_eq!(
            INSTANCES.fetch_add(1, SeqCst),
            0,
            "Cannot have multiple instances of the egaku2d system at the same time!"
        );
    }
    pub fn decrement_one_instance() {
        assert_eq!(
            INSTANCES.fetch_sub(1, SeqCst),
            1,
            "The last egaku2d system object was not properly destroyed"
        );
    }
}

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

    impl Drop for FullScreenSystem {
        fn drop(&mut self) {
            onein::decrement_one_instance();
        }
    }

    pub struct FullScreenSystem {
        inner: SimpleCanvas,
        window_dim: FixedAspectVec2,
        windowed_context: Option<glutin::WindowedContext<PossiblyCurrent>>,
    }
    impl FullScreenSystem {
        pub fn new(events_loop: &glutin::event_loop::EventLoop<()>) -> Self {
            onein::assert_only_one_instance();

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

            let glutin::dpi::PhysicalSize { width, height } =
                windowed_context.window().inner_size();

            // Load the OpenGL function pointers
            gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
            assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);

            let window_dim = axgeom::FixedAspectVec2 {
                ratio: AspectRatio(vec2(width as f64, height as f64)),
                width: width as f64,
            };

            let windowed_context = Some(windowed_context);

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

        ///Creates a new texture from the specified file.
        ///The fact that we need a mutable reference to this object
        ///Ensures that we make the texture in the same thread.
        ///The grid dimensions passed are the tile dimensions is
        ///the texture is a tile set.
        pub fn texture(
            &mut self,
            file: &str,
            grid_dim: [u8; 2],
        ) -> image::ImageResult<sprite::Texture> {
            crate::texture(file, grid_dim)
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
    window_dim: axgeom::FixedAspectVec2,
    windowed_context: glutin::WindowedContext<PossiblyCurrent>,
}

impl Drop for WindowedSystem {
    fn drop(&mut self) {
        onein::decrement_one_instance();
    }
}

impl WindowedSystem {
    pub fn new(
        dim: [usize; 2],
        events_loop: &glutin::event_loop::EventLoop<()>,
        title: &str,
    ) -> WindowedSystem {
        onein::assert_only_one_instance();

        let dim = axgeom::vec2(dim[0], dim[1]);
        let dim = dim.inner_as::<f32>();

        let game_world = axgeom::Rect::new(0.0, dim.x, 0.0, dim.y);

        let width = game_world.x.distance() as f64;
        let height = game_world.y.distance() as f64;

        let monitor = prompt_for_monitor(events_loop);
        let dpi = monitor.scale_factor();
        let p: glutin::dpi::LogicalSize<f64> =
            glutin::dpi::PhysicalSize { width, height }.to_logical(dpi);

        let gl_window = glutin::window::WindowBuilder::new()
            .with_inner_size(p)
            .with_resizable(false)
            .with_title(title);

        //we are targeting only opengl 3.0 es. and glsl 300 es.

        let windowed_context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (2, 0)))
            .with_vsync(true)
            .build_windowed(gl_window, &events_loop)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        // Load the OpenGL function pointers
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
        assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);

        //let dpi = windowed_context.window().scale_factor();
        let glutin::dpi::PhysicalSize { width, height } = windowed_context.window().inner_size();
        assert_eq!(width as usize, dim.x as usize);
        assert_eq!(height as usize, dim.y as usize);

        let window_dim = axgeom::FixedAspectVec2 {
            ratio: axgeom::AspectRatio(axgeom::vec2(width as f64, height as f64)),
            width: width as f64,
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

    pub fn get_dim(&self) -> [usize; 2] {
        let v = self.window_dim.as_vec().inner_as();
        [v.x, v.y]
    }

    ///Creates a new texture from the specified file.
    ///The fact that we need a mutable reference to this object
    ///Ensures that we make the texture in the same thread.
    ///The grid dimensions passed are the tile dimensions is
    ///the texture is a tile set.
    pub fn texture(
        &mut self,
        file: &str,
        grid_dim: [u8; 2],
    ) -> image::ImageResult<sprite::Texture> {
        crate::texture(file, grid_dim)
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

use egaku2d_core::gl::types::GLuint;
use egaku2d_core::gl_ok;
use egaku2d_core::sprite::*;

///Creates a new texture from the specified file.
///The fact that we need a mutable reference to this object
///Ensures that we make the texture in the same thread.
///The grid dimensions passed are the tile dimensions is
///the texture is a tile set.
fn texture(file: &str, grid_dim: [u8; 2]) -> image::ImageResult<sprite::Texture> {
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
            Ok(unsafe { Texture::new(id, grid_dim, [width as f32, height as f32]) })
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

        //TODO convert these into options? with_linear() with_nearest() ??
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl_ok!();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl_ok!();

        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl_ok!();

        texture_id
    }
}
