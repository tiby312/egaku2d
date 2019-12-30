use super::*;

pub struct SquareSave {
    radius: f32,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl SquareSave {
    pub fn draw(&self, session: &mut SimpleCanvas, col: [f32; 4]) {
        session.circle_program.set_buffer_and_draw(
            self.radius * GL_POINT_COMP * session.point_mul.0,
            col,
            0,
            self.buffer.get_id(),
            gl::POINTS,
            self.buffer.len(),
        );
    }
}
pub struct SquareSession<'a> {
    pub(crate) radius: f32,
    pub(crate) sys: &'a mut SimpleCanvas,
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
    pub fn addp(&mut self, x: f32, y: f32) -> &mut Self {
        self.add(vec2(x, y))
    }
    pub fn save(&mut self) -> SquareSave {
        SquareSave {
            radius: self.radius,
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn send_and_draw(&mut self, col: [f32; 4]) {
        //self.sys.circle_buffer.update();
        self.sys.circle_buffer.update();
        self.sys.circle_program.set_buffer_and_draw(
            self.radius * GL_POINT_COMP * self.sys.point_mul.0,
            col,
            0,
            self.sys.circle_buffer.get_id(),
            gl::POINTS,
            self.sys.circle_buffer.len(),
        );
    }
}
impl<'a> Drop for SquareSession<'a> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}

pub struct CircleSave {
    radius: f32,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl CircleSave {
    pub fn draw(&self, session: &mut SimpleCanvas, col: [f32; 4]) {
        session.circle_program.set_buffer_and_draw(
            self.radius * GL_POINT_COMP * session.point_mul.0,
            col,
            1,
            self.buffer.get_id(),
            gl::POINTS,
            self.buffer.len(),
        );
    }
}
pub struct CircleSession<'a> {
    pub(crate) radius: f32,
    pub(crate) sys: &'a mut SimpleCanvas,
}
impl<'a> Drop for CircleSession<'a> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}
impl<'a> CircleSession<'a> {
    pub fn save(&mut self) -> CircleSave {
        CircleSave {
            radius: self.radius,
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn send_and_draw(&mut self, col: [f32; 4]) {
        
        //TODO NO IDEA WHY THIS IS NEEDED ON LINUX.
        //Without this function call, on linux not every shape gets drawn.
        //gl_PointCoord will always return zero if you you try 
        //and draw some circles after drawing a rect save.
        //It is something to do with changing between gl::TRIANGLES to gl::POINTS.
        //but this shouldnt be a problem since they are seperate vbos.
        self.sys.circle_program.set_buffer_and_draw(
            0.,
            col,
            1,
            self.sys.circle_buffer.get_id(),
            gl::POINTS,
            1,
        );
        self.sys.circle_buffer.update();
        
        self.sys.circle_program.set_buffer_and_draw(
            self.radius * GL_POINT_COMP * self.sys.point_mul.0,
            col,
            1,
            self.sys.circle_buffer.get_id(),
            gl::POINTS,
            self.sys.circle_buffer.len(),
        );
    }

    #[inline(always)]
    pub fn addp(&mut self, x: f32, y: f32) -> &mut Self {
        self.add(vec2(x, y))
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
    pub(crate) sys: &'a mut SimpleCanvas,
}
impl Drop for RectSession<'_> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}

pub struct RectSave {
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}

impl RectSave {
    pub fn draw(&self, session: &mut SimpleCanvas, col: [f32; 4]) {
        assert_eq!(self.buffer.len() % 3, 0);
        session.circle_program.set_buffer_and_draw(
            0.0,
            col,
            0,
            self.buffer.get_id(),
            gl::TRIANGLES,
            self.buffer.len(),
        );
    }
}

impl RectSession<'_> {
    pub fn save(&mut self) -> RectSave {
        RectSave {
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn send_and_draw(&mut self, col: [f32; 4]) {
        assert_eq!(self.sys.circle_buffer.len() % 3, 0);
        self.sys.circle_buffer.update();
        self.sys.circle_program.set_buffer_and_draw(
            0.0,
            col,
            0,
            self.sys.circle_buffer.get_id(),
            gl::TRIANGLES,
            self.sys.circle_buffer.len(),
        );
    }

    ///NOTE The argument positions
    ///It is x1,x2,y1,y2  not  x1,y1,x2,y2.
    #[inline(always)]
    pub fn addp(&mut self, x1: f32, x2: f32, y1: f32, y2: f32) -> &mut Self {
        self.add(rect(x1, x2, y1, y2))
    }
    #[inline(always)]
    pub fn add(&mut self, rect: Rect<f32>) -> &mut Self {
        let [tl, tr, br, bl] = rect.get_corners();
        //let arr = [a, b, c, c, d, a];
        let arr = [tr, tl, bl, bl, br, tr];
        for a in arr.iter() {
            self.sys
                .circle_buffer
                .push(circle_program::Vertex([a.x, a.y]));
        }

        self
    }
}

pub struct ArrowSave {
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl ArrowSave {
    pub fn draw(&self, session: &mut SimpleCanvas, col: [f32; 4]) {
        session.circle_program.set_buffer_and_draw(
            0.0,
            col,
            0,
            self.buffer.get_id(),
            gl::TRIANGLES,
            self.buffer.len(),
        );
    }
}
pub struct ArrowSession<'a> {
    pub(crate) sys: &'a mut SimpleCanvas,
    pub(crate) radius: f32,
}
impl Drop for ArrowSession<'_> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}

impl ArrowSession<'_> {
    pub fn save(&mut self) -> ArrowSave {
        ArrowSave {
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn send_and_draw(&mut self, col: [f32; 4]) {
        self.sys.circle_buffer.update();
        self.sys.circle_program.set_buffer_and_draw(
            0.0,
            col,
            0,
            self.sys.circle_buffer.get_id(),
            gl::TRIANGLES,
            self.sys.circle_buffer.len(),
        );
    }

    #[inline(always)]
    pub fn addp(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> &mut Self {
        self.add(vec2(x1, y1), vec2(x2, y2))
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

pub struct LineSave {
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}

impl LineSave {
    pub fn draw(&self, session: &mut SimpleCanvas, col: [f32; 4]) {
        let _kk = session.point_mul.0;
        session.circle_program.set_buffer_and_draw(
            0.0,
            col,
            0,
            self.buffer.get_id(),
            gl::TRIANGLES,
            self.buffer.len(),
        );
    }
}

pub struct LineSession<'a> {
    pub(crate) sys: &'a mut SimpleCanvas,
    pub(crate) radius: f32,
}
impl Drop for LineSession<'_> {
    fn drop(&mut self) {
        self.sys.reset();
    }
}
impl LineSession<'_> {
    pub fn save(&mut self) -> LineSave {
        LineSave {
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn send_and_draw(&mut self, col: [f32; 4]) {
        self.sys.circle_buffer.update();
        self.sys.circle_program.set_buffer_and_draw(
            0.0,
            col,
            0,
            self.sys.circle_buffer.get_id(),
            gl::TRIANGLES,
            self.sys.circle_buffer.len(),
        );
    }

    #[inline(always)]
    pub fn addp(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> &mut Self {
        self.add(vec2(x1, y1), vec2(x2, y2))
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
