//!
//! ## Overview
//!
//! This crate provides the opengl internals without the opengl context creation functionality.
//! So this crate does not depend on glutin.
//!
use axgeom::*;

use core::mem;
use gl::types::*;

mod shader;
mod vbo;

#[macro_export]
macro_rules! gl_ok {
    () => {
        assert_eq!(gl::GetError(), gl::NO_ERROR);
    };
}

use circle_program::CircleProgram;
use circle_program::PointMul;
mod circle_program;

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct SquareSession<'a> {
    sys: &'a mut MySys,
}
impl<'a> SquareSession<'a> {
    #[inline(always)]
    pub fn add(&mut self, point: Vec2<f32>) -> &mut Self {
        self.sys
            .circle_buffer
            .push(circle_program::Vertex([point.x, point.y]));
        self
    }

    #[inline(always)]
    pub fn addp(&mut self, x:f32,y:f32)->&mut Self{
        self.add(vec2(x,y))
    }

    pub fn draw(&mut self) {
        self.sys.circle_buffer.update();

        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();

            //////
            gl::DrawArrays(gl::POINTS, 0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
    }
}
impl<'a> Drop for SquareSession<'a> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}
pub struct CircleSession<'a> {
    sys: &'a mut MySys,
}
impl<'a> Drop for CircleSession<'a> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}
impl<'a> CircleSession<'a> {
    pub fn draw(&mut self) {
        self.sys.circle_buffer.update();

        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();

            //////
            gl::DrawArrays(gl::POINTS, 0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
    }

    #[inline(always)]
    pub fn addp(&mut self, x:f32,y:f32)->&mut Self{
        self.add(vec2(x,y))
    }
    #[inline(always)]
    pub fn add(&mut self, point: Vec2<f32>) -> &mut Self {
        self.sys
            .circle_buffer
            .push(circle_program::Vertex([point.x, point.y]));
        self
    }
}

pub struct RectSession<'a> {
    sys: &'a mut MySys,
}
impl Drop for RectSession<'_> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}

pub struct RectSave{
    buffer:vbo::StaticBuffer<circle_program::Vertex>
}
impl RectSave{
    pub fn display(&self,session:&DrawSession){
         unsafe {
            gl::UseProgram(session.sys.circle_program.program);
            gl_ok!();

            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer.get_id());
            gl_ok!();

            gl::DrawArrays(gl::TRIANGLES, 0 as i32, self.buffer.len() as i32);
            gl_ok!();
        }
    }
}

impl RectSession<'_> {
    pub fn save(&mut self)->RectSave{
        RectSave{buffer:vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts())}
    }
    pub fn draw(&mut self) {
        self.sys.circle_buffer.update();

        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();

            //////
            gl::DrawArrays(gl::TRIANGLES, 0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
    }

    ///NOTE The argument positions
    ///It is x1,x2,y1,y2  not  x1,y1,x2,y2.
    #[inline(always)]
    pub fn addp(&mut self, x1:f32,x2:f32,y1:f32,y2:f32)->&mut Self{
        self.add(rect(x1,x2,y1,y2))
    }
    #[inline(always)]
    pub fn add(&mut self, rect: Rect<f32>) -> &mut Self {
        let [a, b, c, d] = rect.get_corners();
        let arr = [a, b, c, c, d, a];

        for a in arr.iter() {
            self.sys
                .circle_buffer
                .push(circle_program::Vertex([a.x, a.y]));
        }

        self
    }
}

pub struct ArrowSession<'a> {
    sys: &'a mut MySys,
    radius: f32,
}
impl Drop for ArrowSession<'_> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}

impl ArrowSession<'_> {
    pub fn draw(&mut self) {
        self.sys.circle_buffer.update();

        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();

            //////
            gl::DrawArrays(gl::TRIANGLES, 0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
    }

    #[inline(always)]
    pub fn addp(&mut self, x1:f32,y1:f32,x2:f32,y2:f32)->&mut Self{
        self.add(vec2(x1,y1),vec2(x2,y2))
    }
    #[inline(always)]
    pub fn add(&mut self, start: Vec2<f32>, end: Vec2<f32>) -> &mut Self {
        let radius = self.radius;
        let offset = end - start;

        let arrow_head = start + offset * 0.8;

        let k = offset.rotate_90deg_right().normalize_to(1.0);
        let start1 = start + k * radius;
        let start2 = start - k * radius;

        let end1 = arrow_head + k * radius;
        let end2 = arrow_head - k * radius;

        let end11 = arrow_head + k * radius * 2.5;
        let end22 = arrow_head - k * radius * 2.5;
        let arr = [start1, start2, end1, start2, end1, end2, end, end11, end22];

        for a in arr.iter() {
            self.sys
                .circle_buffer
                .push(circle_program::Vertex([a.x, a.y]));
        }
        self
    }
}

pub struct LineSession<'a> {
    sys: &'a mut MySys,
    radius: f32,
}
impl Drop for LineSession<'_> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}
impl LineSession<'_> {
    pub fn draw(&mut self) {
        self.sys.circle_buffer.update();

        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            //TODO move this down more?
            gl::BindBuffer(gl::ARRAY_BUFFER, self.sys.circle_buffer.get_id());
            gl_ok!();

            //////
            gl::DrawArrays(gl::TRIANGLES, 0 as i32, self.sys.circle_buffer.len() as i32);
            gl_ok!();
        }
    }

    #[inline(always)]
    pub fn addp(&mut self, x1:f32,y1:f32,x2:f32,y2:f32)->&mut Self{
        self.add(vec2(x1,y1),vec2(x2,y2))
    }
    #[inline(always)]
    pub fn add(&mut self, start: Vec2<f32>, end: Vec2<f32>) -> &mut Self {
        let radius = self.radius;
        let offset = end - start;
        let k = offset.rotate_90deg_right().normalize_to(1.0);
        let start1 = start + k * radius;
        let start2 = start - k * radius;

        let end1 = end + k * radius;
        let end2 = end - k * radius;

        let arr = [start1, start2, end1, start2, end1, end2];

        for a in arr.iter() {
            self.sys
                .circle_buffer
                .push(circle_program::Vertex([a.x, a.y]));
        }
        self
    }
}

///Allows the user to start drawing shapes.
///The top left corner is the origin.
///y grows as you go down.
///x grows as you go right.
pub struct DrawSession<'a> {
    sys: &'a mut MySys,
}
impl DrawSession<'_> {
    pub fn circles(&mut self, color: [f32; 4], radius: f32) -> CircleSession {
        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            gl::Uniform1f(
                self.sys.circle_program.point_size_uniform,
                radius * 2.5 * self.sys.point_mul.0,
            );
            gl_ok!();
            gl::Uniform4fv(
                self.sys.circle_program.bcol_uniform,
                1,
                std::mem::transmute(&color[0]),
            );
            gl_ok!();

            let square = 1;
            gl::Uniform1i(self.sys.circle_program.square_uniform, square);
            gl_ok!();
        }

        CircleSession { sys: self.sys }
    }
    pub fn squares(&mut self, color: [f32; 4], radius: f32) -> SquareSession {
        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            gl::Uniform1f(
                self.sys.circle_program.point_size_uniform,
                radius * 2.5 * self.sys.point_mul.0,
            );
            gl_ok!();
            gl::Uniform4fv(
                self.sys.circle_program.bcol_uniform,
                1,
                std::mem::transmute(&color[0]),
            );
            gl_ok!();

            let square = 0;
            gl::Uniform1i(self.sys.circle_program.square_uniform, square);
            gl_ok!();
        }

        SquareSession { sys: self.sys }
    }

    pub fn rects(&mut self, color: [f32; 4]) -> RectSession {
        let _kk = self.sys.point_mul.0;

        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            gl::Uniform1f(self.sys.circle_program.point_size_uniform, 0.0);
            gl_ok!();
            gl::Uniform4fv(
                self.sys.circle_program.bcol_uniform,
                1,
                std::mem::transmute(&color[0]),
            );
            gl_ok!();

            let square = 0;
            gl::Uniform1i(self.sys.circle_program.square_uniform, square);
            gl_ok!();
        }

        RectSession { sys: self.sys }
    }

    pub fn arrows(&mut self, color: [f32; 4], radius: f32) -> ArrowSession {
        let kk = self.sys.point_mul.0;

        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            gl::Uniform1f(self.sys.circle_program.point_size_uniform, radius * kk);
            gl_ok!();
            gl::Uniform4fv(
                self.sys.circle_program.bcol_uniform,
                1,
                std::mem::transmute(&color[0]),
            );
            gl_ok!();

            let square = 0;
            gl::Uniform1i(self.sys.circle_program.square_uniform, square);
            gl_ok!();
        }

        ArrowSession {
            sys: self.sys,
            radius: radius * kk,
        }
    }

    pub fn lines(&mut self, color: [f32; 4], radius: f32) -> LineSession {
        let kk = self.sys.point_mul.0;

        unsafe {
            gl::UseProgram(self.sys.circle_program.program);
            gl_ok!();

            gl::Uniform1f(self.sys.circle_program.point_size_uniform, radius * kk);
            gl_ok!();
            gl::Uniform4fv(
                self.sys.circle_program.bcol_uniform,
                1,
                std::mem::transmute(&color[0]),
            );
            gl_ok!();

            let square = 0;
            gl::Uniform1i(self.sys.circle_program.square_uniform, square);
            gl_ok!();
        }

        LineSession {
            sys: self.sys,
            radius: radius * kk,
        }
    }
}

pub struct MySys {
    circle_program: CircleProgram,
    point_mul: PointMul,
    circle_buffer: vbo::GrowableBuffer<circle_program::Vertex>,
}
impl MySys {
    fn reset(&mut self) {
        self.circle_buffer.clear();
    }
    pub fn set_viewport(&mut self, width: f32, rect: Rect<f32>) {
        self.point_mul = self.circle_program.set_viewport(width, rect);
    }
    pub fn new(dim: Rect<f32>) -> MySys {
        let circle_buffer = vbo::GrowableBuffer::new();
        let mut circle_program = CircleProgram::new();
        let point_mul = circle_program.set_viewport(dim.x.distance(), dim);

        MySys {
            point_mul,
            circle_program,
            circle_buffer,
        }
    }

    pub fn draw_sys(&mut self,back_color:[f32;3]) -> DrawSession {
        unsafe {

            gl::ClearColor(back_color[0], back_color[1], back_color[2], 1.0);
            gl_ok!();
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl_ok!();
            gl::Enable(gl::BLEND);
            gl_ok!();
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl_ok!();
        }
        DrawSession { sys: self }
    }
}
