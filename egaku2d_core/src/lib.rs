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

use crate::circle_program::*;
use crate::sprite_program::*;

type PointType = [f32; 2];

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

#[derive(Debug)]
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

use self::uniforms::UniformCommon;
use self::uniforms::*;
///Contains the objects used for the uniform setting stage of the egaku2d drawing pipeline.
pub mod uniforms {
    use super::*;
    use vbo::BufferInfo;

    pub struct StaticUniforms<'a> {
        pub(crate) sys: &'a mut SimpleCanvas,
        pub(crate) un: UniformVals<'a>,
        pub(crate) common: UniformCommon,
        pub(crate) buffer: BufferInfo,
    }

    impl<'a> StaticUniforms<'a> {
        pub fn with_offset(&mut self, offset: [f32; 2]) -> &mut Self {
            self.common.offset = vec2(offset[0], offset[1]);
            self
        }

        pub fn with_color(&mut self, color: [f32; 4]) -> &mut Self {
            self.common.color = color;
            self
        }
/*
        pub fn with_texture(&mut self,texture:&'a sprite::Texture)->&mut Self{
            //add offset:[f32;2],scale:f32
            //println!("offset ignored");
            //println!("scale ignored");

            match &mut self.un{
                UniformVals::Sprite(s)=>{
                    println!("not implemented");
                },
                UniformVals::Regular(s)=>{
                    s.texture=Some(texture);
                },
                UniformVals::Circle(s)=>{
                    s.texture=Some(texture);
                }
            }   
            self
        }
*/
        pub fn draw(&mut self) {
            match &self.un {
                UniformVals::Sprite(a) => {
                    self.sys
                        .sprite_program
                        .set_buffer_and_draw(&self.common, a, self.buffer);
                }
                UniformVals::Regular(a) => {
                    self.sys
                        .regular_program
                        .set_buffer_and_draw(&self.common, a, self.buffer);
                }
                UniformVals::Circle(a) => {
                    self.sys
                        .circle_program
                        .set_buffer_and_draw(&self.common, a, self.buffer);
                }
            }
        }
    }

    pub struct UniformCommon {
        pub(crate) offset: Vec2<f32>,
        pub(crate) color: [f32; 4],
    }

    pub enum UniformVals<'a> {
        Sprite(SpriteProgramUniformValues<'a>),
        Regular(ProgramUniformValues<'a>),
        Circle(ProgramUniformValues<'a>),
    }

    pub struct Uniforms<'a> {
        pub(crate) sys: &'a mut SimpleCanvas,
        pub(crate) un: UniformVals<'a>,
        pub(crate) common: UniformCommon,
    }

    impl<'a> Uniforms<'a> {
        pub fn with_offset(&mut self, offset: [f32; 2]) -> &mut Self {
            self.common.offset = vec2(offset[0], offset[1]);
            self
        }
/*
        pub fn with_texture(&mut self,texture:&'a sprite::Texture)->&mut Self{
            //add offset:[f32;2],scale:f32
            //println!("offset ignored");
            //println!("scale ignored");

            match &mut self.un{
                UniformVals::Sprite(s)=>{
                    println!("not implemented");
                },
                UniformVals::Regular(s)=>{
                    s.texture=Some(texture);
                },
                UniformVals::Circle(s)=>{
                    s.texture=Some(texture);
                }
            }   
            self
        }
*/
        pub fn with_color(&mut self, color: [f32; 4]) -> &mut Self {
            self.common.color = color;
            self
        }

        pub fn send_and_draw(&mut self) {
            match &self.un {
                UniformVals::Sprite(a) => {
                    let verts=a.verts.unwrap();
                    self.sys.sprite_buffer.send_to_gpu(verts);
                    self.sys.sprite_program.set_buffer_and_draw(
                        &self.common,
                        a,
                        self.sys.sprite_buffer.get_info(),
                    );
                }
                UniformVals::Regular(a) => {
                    let verts=a.verts.unwrap();
                    self.sys.circle_buffer.send_to_gpu(verts);
                    self.sys.regular_program.set_buffer_and_draw(
                        &self.common,
                        a,
                        self.sys.circle_buffer.get_info(),
                    );
                }
                UniformVals::Circle(a) => {
                    //self.sys.circle_buffer.update();
                    let verts=a.verts.unwrap();
                    self.sys.circle_buffer.send_to_gpu(verts);
                    
                    self.sys.circle_program.set_buffer_and_draw(
                        &self.common,
                        a,
                        self.sys.circle_buffer.get_info(),
                    );
                }
            }
        }
    }
}

///Allows the user to start drawing shapes.
///The top left corner is the origin.
///y grows as you go down.
///x grows as you go right.
pub struct SimpleCanvas {
    _ns: NotSend,
    circle_program: CircleProgram,
    regular_program: CircleProgram,
    sprite_program: SpriteProgram,
    point_mul: PointMul,

    //It is important to note that this buffers might not be empty when a session object is dropped.
    //the buffers are cleared on creation of a session.
    //this allows us to not have to implement Drop for the session to make sure that the buffer is cleared.
    //if they were to implement drop, they would be slightly less egronomic to use.
    circle_buffer: vbo::GrowableBuffer<circle_program::Vertex>,
    sprite_buffer: vbo::GrowableBuffer<sprite_program::Vertex>,
    color: [f32; 4], //Default color used
}

impl SimpleCanvas {
    pub fn set_default_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn set_viewport(&mut self, window_dim: axgeom::FixedAspectVec2, game_width: f32) {
        self.point_mul = self.circle_program.set_viewport(window_dim, game_width);
        let _ = self.regular_program.set_viewport(window_dim, game_width);
        let _ = self.sprite_program.set_viewport(window_dim, game_width);
    }

    //Unsafe since user might create two instances, both of
    //which could make opengl calls simultaneously
    pub unsafe fn new(window_dim: axgeom::FixedAspectVec2) -> SimpleCanvas {
        let circle_buffer = vbo::GrowableBuffer::new();
        let sprite_buffer = vbo::GrowableBuffer::new();

        let mut circle_program = CircleProgram::new(circle_program::CIRCLE_FS_SRC);

        let mut regular_program = CircleProgram::new(circle_program::REGULAR_FS_SRC);

        let mut sprite_program = SpriteProgram::new();

        let point_mul = circle_program.set_viewport(window_dim, window_dim.width as f32);
        let _ = regular_program.set_viewport(window_dim, window_dim.width as f32);
        let _ = sprite_program.set_viewport(window_dim, window_dim.width as f32);

        gl::Enable(gl::BLEND);
        gl_ok!();
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl_ok!();

        SimpleCanvas {
            _ns: ns(),
            point_mul,
            sprite_program,
            regular_program,
            circle_program,
            circle_buffer,
            sprite_buffer,
            color: [1.0; 4],
        }
    }

    pub fn sprites(&mut self) -> sprite::SpriteSession {
        sprite::SpriteSession { sys: self,verts:Vec::new() }
    }

    pub fn circles(&mut self) -> CircleSession {
        CircleSession { sys: self ,verts:Vec::new()}
    }
    pub fn squares(&mut self) -> SquareSession {
        SquareSession { sys: self ,verts:Vec::new()}
    }
    pub fn rects(&mut self) -> RectSession {
        RectSession { sys: self ,verts:Vec::new()}
    }
    pub fn arrows(&mut self, radius: f32) -> ArrowSession {
        let kk = self.point_mul.0;

        ArrowSession {
            sys: self,
            radius: radius * kk,
            verts:Vec::new()
        }
    }

    pub fn lines(&mut self, radius: f32) -> LineSession {
        let kk = self.point_mul.0;
        LineSession {
            sys: self,
            radius: radius * kk,
            verts:Vec::new()
        }
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
