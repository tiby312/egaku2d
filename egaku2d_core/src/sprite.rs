use super::*;



pub struct SpriteSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<sprite_program::Vertex>,
}
impl SpriteSave {

    pub fn uniforms<'a>(&'a self,sys:&'a mut SimpleCanvas,texture:&'a Texture,radius:f32)->StaticUniforms<'a>{
        let sqrt2:f32=1.41421356237;
        let radius=radius*sqrt2;

        let common=UniformCommon{color:sys.color,offset:vec2same(0.0)};
        let un=SpriteProgramUniformValues{radius,texture};
        StaticUniforms{sys,common,un:UniformVals::Sprite(un),buffer:self.buffer.get_info()}
    }
}

pub struct SpriteSession<'a> {
    pub(crate) sys: &'a mut SimpleCanvas,
}

impl SpriteSession<'_> {
    ///Add a point sprite.
    #[inline(always)]
    pub fn add(&mut self, point: PointType, index: u16,rotation:f32) -> &mut Self {
        
        let k=rotation.rem_euclid(core::f32::consts::PI*2.);
        let k=k/(core::f32::consts::PI*2.);
        let k= (k*(core::u16::MAX as f32)) as u16;

        self.sys.sprite_buffer.push(sprite_program::Vertex {
            pos: point,
            index: index as u16,
            rotation:k
        });
        self
    }

    ///Save this sprite session to into its own static buffer to be drawn later.
    pub fn save(&mut self) -> SpriteSave {
        SpriteSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(self.sys.sprite_buffer.get_verts()),
        }
    }

    pub fn uniforms<'a>(&'a mut self,texture: &'a Texture,radius:f32)->Uniforms<'a>{
        let sqrt2:f32=1.41421356237;
        let radius=radius*sqrt2;

        let common=UniformCommon{color:self.sys.color,offset:vec2same(0.0)};
        let un=SpriteProgramUniformValues{radius,texture};
        Uniforms{common,sys:self.sys,un:UniformVals::Sprite(un)}
    }
}



#[derive(Debug)]
pub struct Texture {
    _ns: NotSend,
    pub(crate) grid_dim: [u8;2],
    pub(crate) dim:[f32;2],
    pub(crate) id: GLuint,
}

impl Texture {
    ///Create a texture index from a coordinate in the tile set.
    ///The top left time maps to 0,0. 
    ///The x component grows to the right.
    ///The y component grows downwards.
    pub fn coord_to_index(&self, cell: [u8;2]) -> u16 {
        let cell=[cell[0] as u16,cell[1] as u16];
        self.grid_dim[0] as u16 * cell[0] + cell[1]
    }

    pub unsafe fn new(textureid:GLuint,grid_dim:[u8;2],dim:[f32;2])->Texture{
        Texture{id:textureid,grid_dim,_ns:ns(),dim}
    }

}

