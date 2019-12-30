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
mod vbo;

///Macro that asserts that there are no opengl errors.
#[macro_export]
macro_rules! gl_ok {
    () => {
        assert_eq!(gl::GetError(), gl::NO_ERROR);
    };
}

use circle_program::CircleProgram;
use circle_program::PointMul;
mod circle_program;

///All the opengl functions generated from the gl_generator crate.
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

///Contains all the shape drawing session and save objects
///They all follow the same api outlined in the crate documentation.
pub mod shapes;

const GL_POINT_COMP: f32 = 2.5;
//const GL_POINT_COMP:f32=2.0;

///Allows the user to start drawing shapes.
///The top left corner is the origin.
///y grows as you go down.
///x grows as you go right.
pub struct SimpleCanvas {
    circle_program: CircleProgram,
    point_mul: PointMul,
    circle_buffer: vbo::GrowableBuffer<circle_program::Vertex>,
}

impl SimpleCanvas {
    fn reset(&mut self) {
        self.circle_buffer.clear();
    }
    pub fn set_viewport(&mut self, window_dim: axgeom::FixedAspectVec2, game_width: f32) {
        self.point_mul = self.circle_program.set_viewport(window_dim, game_width);
    }

    //Unsafe since user might create two instances, both of
    //which could make opengl calls simultaneously
    pub unsafe fn new(window_dim: axgeom::FixedAspectVec2) -> SimpleCanvas {
        let circle_buffer = vbo::GrowableBuffer::new();
        let mut circle_program = CircleProgram::new();

        let point_mul = circle_program.set_viewport(window_dim, window_dim.width as f32);

        gl::Enable(gl::BLEND);
        gl_ok!();
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl_ok!();

        SimpleCanvas {
            point_mul,
            circle_program,
            circle_buffer,
        }
    }

    pub fn circles(&mut self, radius: f32) -> CircleSession {
        CircleSession { radius, sys: self }
    }
    pub fn squares(&mut self, radius: f32) -> SquareSession {
        SquareSession { radius, sys: self }
    }
    pub fn rects(&mut self) -> RectSession {
        RectSession { sys: self }
    }
    pub fn arrows(&mut self, radius: f32) -> ArrowSession {
        let kk = self.point_mul.0;

        ArrowSession {
            sys: self,
            radius: radius * kk,
        }
    }

    pub fn lines(&mut self, radius: f32) -> LineSession {
        let kk = self.point_mul.0;
        LineSession {
            sys: self,
            radius: radius * kk,
        }
    }

    pub fn clear_color(&mut self, back_color: [f32; 3]) {
        unsafe {
            gl::ClearColor(back_color[0], back_color[1], back_color[2], 1.0);
            gl_ok!();

            //self.rects()
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl_ok!();
        }
    }
}
