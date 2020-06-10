use super::*;

//pub use self::sprite_program::Vertex;

pub struct SpriteSave {
    _ns: NotSend,
    pub(crate) buffer: vbo::StaticBuffer<sprite_program::Vertex>,
}
impl SpriteSave {
    pub fn uniforms<'a>(
        &'a self,
        sys: &'a mut SimpleCanvas,
        texture: &'a Texture,
        radius: f32,
    ) -> Uniforms<'a> {
        let sqrt2: f32 = 1.41421356237;
        let radius = radius * sqrt2;

        let common = UniformCommon {
            color: sys.color,
            offset: sys.offset,
        };
        let un = SpriteProgramUniformValues { radius, texture };
        Uniforms {
            sys,
            common,
            un: UniformVals::Sprite(un),
            buffer: self.buffer.get_info(),
        }
    }
}

pub struct SpriteSession {
    pub(crate) verts: Vec<sprite_program::Vertex>,
}

impl SpriteSession {
    pub fn new() -> Self {
        SpriteSession { verts: Vec::new() }
    }
    ///Add a point sprite.
    #[inline(always)]
    pub fn add(&mut self, point: PointType, index: u16, rotation: f32) -> &mut Self {
        let k = rotation.rem_euclid(core::f32::consts::PI * 2.);
        let k = k / (core::f32::consts::PI * 2.);
        let k = (k * (core::u16::MAX as f32)) as u16;

        self.verts.push(sprite_program::Vertex {
            pos: point,
            index: index as u16,
            rotation: k,
        });
        self
    }

    pub fn append(&mut self, other: &mut Self) {
        self.verts.append(&mut other.verts);
    }

    ///Save this sprite session to into its own static buffer to be drawn later.
    pub fn save(&mut self, _sys: &mut SimpleCanvas) -> SpriteSave {
        SpriteSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(&self.verts),
        }
    }

    pub fn send_and_uniforms<'a>(
        &'a mut self,
        sys: &'a mut SimpleCanvas,
        texture: &'a Texture,
        radius: f32,
    ) -> Uniforms<'a> {
        sys.sprite_buffer.send_to_gpu(&self.verts);

        let sqrt2: f32 = 1.41421356237;
        let radius = radius * sqrt2;

        let common = UniformCommon {
            color: sys.color,
            offset: sys.offset,
        };
        let un = SpriteProgramUniformValues { radius, texture };

        let buffer = sys.sprite_buffer.get_info(self.verts.len());
        Uniforms {
            common,
            sys,
            un: UniformVals::Sprite(un),
            buffer,
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    _ns: NotSend,
    pub(crate) grid_dim: [u8; 2],
    pub(crate) dim: [f32; 2],
    pub(crate) id: GLuint,
}

impl Texture {
    pub fn grid_dim(&self) -> [u8; 2] {
        self.grid_dim
    }
    pub fn dim(&self) -> [f32; 2] {
        self.dim
    }
    ///Create a texture index from a coordinate in the tile set.
    ///The top left time maps to 0,0.
    ///The x component grows to the right.
    ///The y component grows downwards.
    pub fn coord_to_index(&self, cell: [u8; 2]) -> u16 {
        let cell = [cell[0] as u16, cell[1] as u16];

        self.grid_dim[0] as u16 * cell[1] + cell[0]
    }

    pub unsafe fn new(textureid: GLuint, grid_dim: [u8; 2], dim: [f32; 2]) -> Texture {
        Texture {
            id: textureid,
            grid_dim,
            _ns: ns(),
            dim,
        }
    }
}
