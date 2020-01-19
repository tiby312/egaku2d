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


mod textured_shape_program;


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

    pub struct Uniforms<'a> {
        pub(crate) sys: &'a mut SimpleCanvas,
        pub(crate) un: UniformVals<'a>,
        pub(crate) common: UniformCommon,
        pub(crate) buffer: BufferInfo,
    }

    impl<'a> Uniforms<'a> {
        pub fn with_offset(&mut self, offset: [f32; 2]) -> &mut Self {
            self.common.offset = vec2(offset[0], offset[1]);
            self
        }

        pub fn with_color(&mut self, color: [f32; 4]) -> &mut Self {
            self.common.color = color;
            self
        }

        pub fn with_texture(&mut self,texture:&'a sprite::Texture,scale:f32,offset:[f32;2])->&mut Self{
            //add offset:[f32;2],scale:f32
            //println!("offset ignored");
            //println!("scale ignored");

            match &mut self.un{
                UniformVals::Sprite(s)=>{
                    s.texture=texture;
                    //println!("not implemented");
                },
                UniformVals::Regular(s)=>{
                    s.texture=Some((texture,scale,offset));
                },
                UniformVals::Circle(s)=>{
                    s.texture=Some((texture,scale,offset));
                }
            }   
            self
        }

        pub fn draw(&mut self) {
            match &self.un {
                UniformVals::Sprite(a) => {
                    self.sys
                        .sprite_program
                        .set_buffer_and_draw(&self.common, a, self.buffer);
                }
                UniformVals::Regular(a) => {
                    if a.texture.is_some(){
                        self.sys.textured_shape_program.set_buffer_and_draw(&self.common,a,self.buffer);
                    }else{
                        self.sys
                            .regular_program
                            .set_buffer_and_draw(&self.common, a, self.buffer);                        
                    }

                }
                UniformVals::Circle(a) => {
                    if a.texture.is_some(){
                        self.sys.textured_circle_program.set_buffer_and_draw(&self.common,a,self.buffer);
                    }else{
                        self.sys
                            .circle_program
                            .set_buffer_and_draw(&self.common, a, self.buffer);   
                    }
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

    /*
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

        pub fn with_texture(&mut self,texture:&'a sprite::Texture,scale:f32,offset:[f32;2])->&mut Self{
            //add offset:[f32;2],scale:f32
            //println!("offset ignored");
            //println!("scale ignored");

            match &mut self.un{
                UniformVals::Sprite(s)=>{
                    //println!("not implemented");
                    s.texture=texture;
                },
                UniformVals::Regular(s)=>{
                    s.texture=Some((texture,scale,offset));
                },
                UniformVals::Circle(s)=>{
                    s.texture=Some((texture,scale,offset));
                }
            }   
            self
        }

        pub fn with_color(&mut self, color: [f32; 4]) -> &mut Self {
            self.common.color = color;
            self
        }

        pub fn draw(&mut self) {
            match &self.un {
                UniformVals::Sprite(a) => {
                    self.sys.sprite_program.set_buffer_and_draw(
                        &self.common,
                        a,
                        self.sys.sprite_buffer.get_info(),
                    );
                }
                UniformVals::Regular(a) => {
                    if a.texture.is_some(){
                        self.sys.textured_shape_program.set_buffer_and_draw(
                            &self.common,
                            a,
                            self.sys.circle_buffer.get_info(),
                        );        
                    }else{
                        self.sys.regular_program.set_buffer_and_draw(
                            &self.common,
                            a,
                            self.sys.circle_buffer.get_info(),
                        );        
                    
                    }
                    
                }
                UniformVals::Circle(a) => {
                    
                    if a.texture.is_some(){
                        self.sys.textured_circle_program.set_buffer_and_draw(
                            &self.common,
                            a,
                            self.sys.circle_buffer.get_info(),
                        );
                    }else{
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
    */
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
    textured_shape_program: textured_shape_program::TexturedShapeProgram,
    textured_circle_program: textured_shape_program::TexturedShapeProgram,
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
        let _ = self.textured_shape_program.set_viewport(window_dim,game_width);
        let _ = self.textured_circle_program.set_viewport(window_dim,game_width);
    }

    //Unsafe since user might create two instances, both of
    //which could make opengl calls simultaneously
    pub unsafe fn new(window_dim: axgeom::FixedAspectVec2) -> SimpleCanvas {
        let circle_buffer = vbo::GrowableBuffer::new();
        let sprite_buffer = vbo::GrowableBuffer::new();

        let mut circle_program = CircleProgram::new(circle_program::CIRCLE_FS_SRC);

        let mut regular_program = CircleProgram::new(circle_program::REGULAR_FS_SRC);

        let mut textured_shape_program = textured_shape_program::TexturedShapeProgram::new(textured_shape_program::REGULAR_FS_SRC);
        let mut textured_circle_program = textured_shape_program::TexturedShapeProgram::new(textured_shape_program::CIRCLE_FS_SRC);


        let mut sprite_program = SpriteProgram::new();

        let point_mul = circle_program.set_viewport(window_dim, window_dim.width as f32);
        let _ = regular_program.set_viewport(window_dim, window_dim.width as f32);
        let _ = sprite_program.set_viewport(window_dim, window_dim.width as f32);
        let _ = textured_shape_program.set_viewport(window_dim,window_dim.width as f32);
        let _ = textured_circle_program.set_viewport(window_dim,window_dim.width as f32);

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
            textured_shape_program,
            textured_circle_program,
            color: [1.0; 4],
        }
    }

    pub fn sprites(&mut self) -> sprite::SpriteSession {
        sprite::SpriteSession { verts:Vec::new() }
    }

    pub fn circles(&mut self) -> CircleSession {
        CircleSession { verts:Vec::new()}
    }

    pub fn squares(&mut self) -> SquareSession {
        SquareSession { verts:Vec::new()}
    }
    pub fn rects(&mut self) -> RectSession {
        RectSession { verts:Vec::new()}
    }
    pub fn arrows(&mut self, radius: f32) -> ArrowSession {
        let kk = self.point_mul.0;

        ArrowSession {
            radius: radius * kk,
            verts:Vec::new()
        }
    }

    pub fn lines(&mut self, radius: f32) -> LineSession {
        let kk = self.point_mul.0;
        LineSession {
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

    //The reference returned by the closure must be a pointer into a member of T.
    pub unsafe fn circles_from_generic_slice<T,F:Fn(&T)->&[f32;2]>(&mut self,bots:&[T],func:F)->BatchCircle<T,F>{
        BatchCircle::new(bots,func)
    }
}



use core::marker::PhantomData;
pub struct BatchCircle<T,F>{
    buffer:vbo::GrowableBuffer<T>,
    func:F,
    _p:PhantomData<T>,
    _ns:NotSend
}

impl<T,F:Fn(&T)->&[f32;2]> BatchCircle<T,F>{
    pub fn new(bots:&[T],func:F)->BatchCircle<T,F>{
        let mut b=BatchCircle{
            buffer:vbo::GrowableBuffer::new(),
            func,
            _p:PhantomData,
            _ns:ns()
        };
        b.buffer.send_to_gpu(bots);
        b
    }

    pub fn send_and_uniforms<'a>(&'a mut self,sys:&'a mut SimpleCanvas,bots:&[T],radius:f32)->Uniforms<'a>{
        let stride=0;
        /*
        if bots.is_empty(){

        }else{
            self.buffer.send_to_gpu(bots);    
        }
        */

        
        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues{
            mode:gl::POINTS,
            radius,
            stride,
            texture:None
        };

        Uniforms {
            sys,
            common,
            un: UniformVals::Circle(un),
            buffer:self.buffer.get_info(bots.len())
        }
    }
}
