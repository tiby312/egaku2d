use super::*;

pub struct SquareSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl SquareSave {
    
    pub fn uniforms<'a>(&'a self,sys:&'a mut SimpleCanvas,radius:f32)->StaticUniforms<'a>{
        let un=ProgramUniformValues{radius,color:sys.color,mode:gl::POINTS,rect:false,offset:vec2same(0.0)};
        StaticUniforms{sys,un,buffer:self.buffer.get_info()}
    }
}

pub struct SquareSession<'a> {
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
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn uniforms(&mut self,radius:f32)->Uniforms{
        let un=ProgramUniformValues{radius,color:self.sys.color,mode:gl::POINTS,rect:false,offset:vec2same(0.0)};
        Uniforms{sys:self.sys,un}
    }
}
impl<'a> Drop for SquareSession<'a> {
    fn drop(&mut self) {
        self.sys.circle_buffer.clear();
    }
}

pub struct CircleSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl CircleSave {
    pub fn uniforms<'a>(&'a self,sys:&'a mut SimpleCanvas,radius:f32)->StaticUniforms<'a>{
        let un=ProgramUniformValues{radius,color:sys.color,mode:gl::POINTS,rect:true,offset:vec2same(0.0)};
        StaticUniforms{sys,un,buffer:self.buffer.get_info()}
    }
}
pub struct CircleSession<'a> {
    pub(crate) sys: &'a mut SimpleCanvas,
}
impl<'a> Drop for CircleSession<'a> {
    fn drop(&mut self) {
        self.sys.circle_buffer.clear();
    }
}
impl<'a> CircleSession<'a> {
    pub fn save(&mut self) -> CircleSave {
        CircleSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn uniforms(&mut self,radius:f32)->Uniforms{
        let un=ProgramUniformValues{radius,color:self.sys.color,mode:gl::POINTS,rect:true,offset:vec2same(0.0)};
        Uniforms{sys:self.sys,un}
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
        self.sys.circle_buffer.clear();
    }
}



pub struct RectSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}

impl RectSave {
    pub fn uniforms<'a>(&'a self,sys:&'a mut SimpleCanvas)->StaticUniforms<'a>{
        let un=ProgramUniformValues{radius:0.0,color:sys.color,mode:gl::TRIANGLES,rect:false,offset:vec2same(0.0)};
        StaticUniforms{sys,un,buffer:self.buffer.get_info()}
    }
}

impl RectSession<'_> {
    pub fn save(&mut self) -> RectSave {
        RectSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn uniforms(&mut self)->Uniforms{
        let un=ProgramUniformValues{radius:0.0,color:self.sys.color,mode:gl::TRIANGLES,rect:false,offset:vec2same(0.0)};
        Uniforms{sys:self.sys,un}
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
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}
impl ArrowSave {
    pub fn uniforms<'a>(&'a self,sys:&'a mut SimpleCanvas)->StaticUniforms<'a>{
        let un=ProgramUniformValues{radius:0.0,color:sys.color,mode:gl::TRIANGLES,rect:false,offset:vec2same(0.0)};
        StaticUniforms{sys,un,buffer:self.buffer.get_info()}
    }
}
pub struct ArrowSession<'a> {
    pub(crate) sys: &'a mut SimpleCanvas,
    pub(crate) radius: f32,
}
impl Drop for ArrowSession<'_> {
    fn drop(&mut self) {
        self.sys.circle_buffer.clear();
    }
}

impl ArrowSession<'_> {
    pub fn save(&mut self) -> ArrowSave {
        ArrowSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn uniforms(&mut self)->Uniforms{
        let un=ProgramUniformValues{radius:0.0,color:self.sys.color,mode:gl::TRIANGLES,rect:false,offset:vec2same(0.0)};
        Uniforms{sys:self.sys,un}
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
    _ns: NotSend,
    buffer: vbo::StaticBuffer<circle_program::Vertex>,
}

impl LineSave {

    pub fn uniforms<'a>(&'a self,sys:&'a mut SimpleCanvas)->StaticUniforms<'a>{
        let un=ProgramUniformValues{radius:0.0,color:sys.color,mode:gl::TRIANGLES,rect:false,offset:vec2same(0.0)};
        StaticUniforms{sys,un,buffer:self.buffer.get_info()}
    }
}

pub struct LineSession<'a> {
    pub(crate) sys: &'a mut SimpleCanvas,
    pub(crate) radius: f32,
}
impl Drop for LineSession<'_> {
    fn drop(&mut self) {
        self.sys.circle_buffer.clear();
    }
}
impl LineSession<'_> {
    pub fn save(&mut self) -> LineSave {
        LineSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(self.sys.circle_buffer.get_verts()),
        }
    }

    pub fn uniforms(&mut self)->Uniforms{
        let un=ProgramUniformValues{radius:0.0,color:self.sys.color,mode:gl::TRIANGLES,rect:false,offset:vec2same(0.0)};
        Uniforms{sys:self.sys,un}
    }

    #[inline(always)]
    pub fn add(&mut self, start: PointType, end: PointType) -> &mut Self {
        let start=vec2(start[0],start[1]);  //TODO a program that detected bad uses like this would be cool
        let end=vec2(end[0],end[1]);

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
