use super::*;

//pub use self::circle_program::Vertex;

pub struct SquareSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl SquareSave {
    pub fn uniforms<'a>(&'a self, sys: &'a mut SimpleCanvas, radius: f32) -> Uniforms<'a> {
        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(radius, gl::POINTS);

        Uniforms {
            sys,
            un: UniformVals::Regular(un),
            common,
            buffer: self.buffer.get_info(),
        }
    }
}

pub struct SquareSession {
    pub(crate) verts: Vec<circle_program::Vertex>,
}
impl SquareSession {
    pub fn new() -> Self {
        SquareSession { verts: Vec::new() }
    }
    #[inline(always)]
    pub fn add(&mut self, point: [f32; 2]) -> &mut Self {
        self.verts.push(circle_program::Vertex(point));
        self
    }

    pub fn append(&mut self, other: &mut Self) {
        self.verts.append(&mut other.verts);
    }

    pub fn save(&mut self, _sys: &mut SimpleCanvas) -> SquareSave {
        SquareSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(&self.verts),
        }
    }

    pub fn send_and_uniforms<'a>(
        &'a mut self,
        sys: &'a mut SimpleCanvas,
        radius: f32,
    ) -> Uniforms<'a> {
        sys.circle_buffer.send_to_gpu(&self.verts);

        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(radius, gl::POINTS);

        let buffer = sys.circle_buffer.get_info(self.verts.len());
        Uniforms {
            sys,
            common,
            un: UniformVals::Regular(un),
            buffer,
        }
    }
}

pub struct CircleSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl CircleSave {
    pub fn uniforms<'a>(&'a self, sys: &'a mut SimpleCanvas, radius: f32) -> Uniforms<'a> {
        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(radius, gl::POINTS);

        let buffer = self.buffer.get_info();
        Uniforms {
            common,
            sys,
            un: UniformVals::Circle(un),
            buffer,
        }
    }
}
pub struct CircleSession {
    pub(crate) verts: Vec<circle_program::Vertex>,
}

impl CircleSession {
    pub fn new() -> Self {
        CircleSession { verts: Vec::new() }
    }
    pub fn save(&mut self, _sys: &mut SimpleCanvas) -> CircleSave {
        CircleSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(&self.verts),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.verts.append(&mut other.verts);
    }
    pub fn send_and_uniforms<'a>(
        &'a mut self,
        sys: &'a mut SimpleCanvas,
        radius: f32,
    ) -> Uniforms<'a> {
        sys.circle_buffer.send_to_gpu(&self.verts);

        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(radius, gl::POINTS);

        let buffer = sys.circle_buffer.get_info(self.verts.len());
        Uniforms {
            sys,
            common,
            un: UniformVals::Circle(un),
            buffer,
        }
    }

    #[inline(always)]
    pub fn add(&mut self, point: [f32; 2]) -> &mut Self {
        self.verts.push(circle_program::Vertex(point));
        self
    }
}

pub struct RectSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}

impl RectSave {
    pub fn uniforms<'a>(&'a self, sys: &'a mut SimpleCanvas) -> Uniforms<'a> {
        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(0.0, gl::TRIANGLES);
        let buffer = self.buffer.get_info();
        Uniforms {
            sys,
            common,
            un: UniformVals::Regular(un),
            buffer,
        }
    }
}

pub struct RectSession {
    pub(crate) verts: Vec<circle_program::Vertex>,
}

impl RectSession {
    pub fn new() -> Self {
        RectSession { verts: Vec::new() }
    }

    pub fn save(&mut self, _sys: &mut SimpleCanvas) -> RectSave {
        RectSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(&self.verts),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.verts.append(&mut other.verts);
    }
    pub fn send_and_uniforms<'a>(&'a mut self, sys: &'a mut SimpleCanvas) -> Uniforms<'a> {
        sys.circle_buffer.send_to_gpu(&self.verts);

        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(0.0, gl::TRIANGLES);
        let buffer = sys.circle_buffer.get_info(self.verts.len());
        Uniforms {
            sys,
            common,
            un: UniformVals::Regular(un),
            buffer,
        }
    }

    #[inline(always)]
    fn create_rect(rect: [f32; 4]) -> [circle_program::Vertex; 6] {
        let rect:Rect<f32> = core::convert::From::from(rect);
        let [tl, tr, br, bl] = rect.get_corners();
        //let arr = [tr, tl, bl, bl, br, tr];

        fn doop(a: Vec2<f32>) -> circle_program::Vertex {
            circle_program::Vertex([a.x, a.y])
        }
        [doop(tr), doop(tl), doop(bl), doop(bl), doop(br), doop(tr)]
    }
    #[inline(always)]
    pub fn add(&mut self, rect: [f32; 4]) -> &mut Self {
        let arr = Self::create_rect(rect);
        self.verts.extend_from_slice(&arr);
        self
    }
}

pub struct ArrowSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl ArrowSave {
    pub fn uniforms<'a>(&'a self, sys: &'a mut SimpleCanvas) -> Uniforms<'a> {
        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(0.0, gl::TRIANGLES);
        Uniforms {
            sys,
            common,
            un: UniformVals::Regular(un),
            buffer: self.buffer.get_info(),
        }
    }
}
pub struct ArrowSession {
    pub(crate) radius: f32,
    pub(crate) verts: Vec<circle_program::Vertex>,
}

impl ArrowSession {
    pub fn new(radius: f32) -> Self {
        ArrowSession {
            radius,
            verts: Vec::new(),
        }
    }
    pub fn save(&mut self, _sys: &mut SimpleCanvas) -> ArrowSave {
        ArrowSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(&self.verts),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.verts.append(&mut other.verts);
    }
    pub fn send_and_uniforms<'a>(&'a mut self, sys: &'a mut SimpleCanvas) -> Uniforms<'a> {
        sys.circle_buffer.send_to_gpu(&self.verts);

        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(0.0, gl::TRIANGLES);
        let buffer = sys.circle_buffer.get_info(self.verts.len());
        Uniforms {
            sys,
            common,
            un: UniformVals::Regular(un),
            buffer,
        }
    }

    #[inline(always)]
    fn create_arrow(radius: f32, start: PointType, end: PointType) -> [circle_program::Vertex; 9] {
        let start = vec2(start[0], start[1]);
        let end = vec2(end[0], end[1]);
        let offset = end - start;

        let arrow_head = start + offset * 0.8;

        let k = offset.rotate_90deg_right().normalize_to(1.0);
        let start1 = start + k * radius;
        let start2 = start - k * radius;

        let end1 = arrow_head + k * radius;
        let end2 = arrow_head - k * radius;

        let end11 = arrow_head + k * radius * 2.5;
        let end22 = arrow_head - k * radius * 2.5;
        //let arr = [start1, start2, end1, start2, end1, end2, end, end11, end22];

        fn doop(a: Vec2<f32>) -> circle_program::Vertex {
            circle_program::Vertex([a.x, a.y])
        }

        [
            doop(start1),
            doop(start2),
            doop(end1),
            doop(start2),
            doop(end1),
            doop(end2),
            doop(end),
            doop(end11),
            doop(end22),
        ]
    }

    #[inline(always)]
    pub fn add(&mut self, start: PointType, end: PointType) -> &mut Self {
        let arr = Self::create_arrow(self.radius, start, end);
        self.verts.extend_from_slice(&arr);
        self
    }
}

pub struct LineSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}

impl LineSave {
    pub fn uniforms<'a>(&'a self, sys: &'a mut SimpleCanvas) -> Uniforms<'a> {
        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues::new(0.0, gl::TRIANGLES);
        Uniforms {
            sys,
            common,
            un: UniformVals::Regular(un),
            buffer: self.buffer.get_info(),
        }
    }
}

pub struct LineSession {
    pub(crate) radius: f32,
    pub(crate) verts: Vec<circle_program::Vertex>,
}

impl LineSession {
    pub fn new(radius: f32) -> Self {
        LineSession {
            radius,
            verts: Vec::new(),
        }
    }

    pub fn save(&mut self, _sys: &mut SimpleCanvas) -> LineSave {
        LineSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(&self.verts),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.verts.append(&mut other.verts);
    }
    pub fn send_and_uniforms<'a>(&'a mut self, sys: &'a mut SimpleCanvas) -> Uniforms<'a> {
        sys.circle_buffer.send_to_gpu(&self.verts);

        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };

        let un = ProgramUniformValues::new(0.0, gl::TRIANGLES);
        let buffer = sys.circle_buffer.get_info(self.verts.len());
        Uniforms {
            sys,
            common,
            un: UniformVals::Regular(un),
            buffer,
        }
    }

    #[inline(always)]
    fn create_line(radius: f32, start: PointType, end: PointType) -> [circle_program::Vertex; 6] {
        let start = vec2(start[0], start[1]); //TODO a program that detected bad uses like this would be cool
        let end = vec2(end[0], end[1]);

        let offset = end - start;
        let k = offset.rotate_90deg_right().normalize_to(1.0);
        let start1 = start + k * radius;
        let start2 = start - k * radius;

        let end1 = end + k * radius;
        let end2 = end - k * radius;

        //let arr = [start1, start2, end1, start2, end1, end2];

        fn doop(a: Vec2<f32>) -> circle_program::Vertex {
            circle_program::Vertex([a.x, a.y])
        }

        [
            doop(start1),
            doop(start2),
            doop(end1),
            doop(start2),
            doop(end1),
            doop(end2),
        ]
    }

    #[inline(always)]
    pub fn add(&mut self, start: PointType, end: PointType) -> &mut Self {
        let a = Self::create_line(self.radius, start, end);
        self.verts.extend_from_slice(&a);
        self
    }
}
