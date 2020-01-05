//!
//! ## Overview
//!
//! This crate provides the opengl internals without the opengl context creation functionality.
//! So this crate does not depend on glutin.
//!
use crate::shapes::*;
use axgeom::*;

use core::mem;
use gl::types::*;

mod shader;

///Contains all the texture/sprite drawing code.
///The api is described in the crate documentation.
pub mod sprite;
mod vbo;

///Macro that asserts that there are no opengl errors.
#[macro_export]
macro_rules! gl_ok {
    () => {
        assert_eq!(gl::GetError(), gl::NO_ERROR);
    };
}

struct NotSend(*mut usize);

fn ns() -> NotSend {
    NotSend(core::ptr::null_mut())
}

use circle_program::CircleProgram;
use circle_program::PointMul;
mod circle_program;
use sprite_program::SpriteProgram;
mod sprite_program;

///All the opengl functions generated from the gl_generator crate.
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

///Contains all the shape drawing session and save objects
///They all follow the same api outlined in the crate documentation.
pub mod shapes;

const GL_POINT_COMP: f32 = 2.0;
//const GL_POINT_COMP:f32=2.0;

///Allows the user to start drawing shapes.
///The top left corner is the origin.
///y grows as you go down.
///x grows as you go right.
pub struct SimpleCanvas {
    circle_program: CircleProgram,
    sprite_program: SpriteProgram,
    point_mul: PointMul,
    circle_buffer: vbo::GrowableBuffer<circle_program::Vertex>,
    sprite_buffer: vbo::GrowableBuffer<sprite_program::Vertex>,
}

impl SimpleCanvas {
    pub fn set_viewport(&mut self, window_dim: axgeom::FixedAspectVec2, game_width: f32) {
        self.point_mul = self.circle_program.set_viewport(window_dim, game_width);

        let _ = self.sprite_program.set_viewport(window_dim, game_width);
    }

    //Unsafe since user might create two instances, both of
    //which could make opengl calls simultaneously
    pub unsafe fn new(window_dim: axgeom::FixedAspectVec2) -> SimpleCanvas {
        let circle_buffer = vbo::GrowableBuffer::new();
        let sprite_buffer = vbo::GrowableBuffer::new();

        let mut circle_program = CircleProgram::new();

        let mut sprite_program = SpriteProgram::new();

        let point_mul = circle_program.set_viewport(window_dim, window_dim.width as f32);
        let _ = sprite_program.set_viewport(window_dim, window_dim.width as f32);

        gl::Enable(gl::BLEND);
        gl_ok!();
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl_ok!();

        SimpleCanvas {
            point_mul,
            sprite_program,
            circle_program,
            circle_buffer,
            sprite_buffer,
        }
    }

    pub fn sprites(&mut self) -> sprite::SpriteSession {
        sprite::SpriteSession { sys: self }
    }

    pub fn circles(&mut self) -> CircleSession {
        assert_eq!(self.circle_buffer.len(), 0);
        CircleSession { sys: self }
    }
    pub fn squares(&mut self) -> SquareSession {
        assert_eq!(self.circle_buffer.len(), 0);
        SquareSession { sys: self }
    }
    pub fn rects(&mut self) -> RectSession {
        assert_eq!(self.circle_buffer.len(), 0);
        RectSession { sys: self }
    }
    pub fn arrows(&mut self, radius: f32) -> ArrowSession {
        assert_eq!(self.circle_buffer.len(), 0);
        let kk = self.point_mul.0;

        ArrowSession {
            sys: self,
            radius: radius * kk,
        }
    }

    pub fn lines(&mut self, radius: f32) -> LineSession {
        assert_eq!(self.circle_buffer.len(), 0);
        let kk = self.point_mul.0;
        LineSession {
            sys: self,
            radius: radius * kk,
        }
    }

    ///Creates a new texture from the specified file.
    ///The fact that we need a mutable reference to this object
    ///Ensures that we make the texture in the same thread.
    ///The grid dimensions passed are the tile dimensions is
    ///the texture is a tile set.
    pub fn texture(
        &mut self,
        file: &str,
        grid_dim: Vec2<u32>,
    ) -> image::ImageResult<sprite::Texture> {
        sprite::Texture::new(file, grid_dim)
    }

    pub fn clear_color(&mut self, back_color: [f32; 3]) {
        unsafe {
            gl::ClearColor(back_color[0], back_color[1], back_color[2], 1.0);
            gl_ok!();

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl_ok!();
        }
    }
}
