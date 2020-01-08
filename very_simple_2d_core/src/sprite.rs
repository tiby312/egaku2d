use super::*;



///The texture index is the other piece of data every sprite has besides
///its position. It tells the gpu which part of a texture to draw.
///Each texture object has functions to create this index from a x and y coordinate. 
///On the gpu, the index will be split into a x and y coordinate.
///If the index is larger than texture.dim.x*texture.dim.y then it will be modded so that
///it can be mapped to a tile set. But obviously, the user should be picking an index
///that maps to a valid tile in the tile set to begin with.
pub struct TexIndex(pub u32);

pub struct SpriteSave {
    _ns: NotSend,
    buffer: vbo::StaticBuffer<sprite_program::Vertex>,
}
impl SpriteSave {

    /*
    pub fn draw(
        &self,
        session: &mut SimpleCanvas,
        texture: &Texture,
        color: [f32; 4],
        point_size: f32,
    ) {
        session.sprite_program.set_buffer_and_draw(
            point_size * GL_POINT_COMP * session.point_mul.0,
            color,
            self.buffer.get_id(),
            self.buffer.len(),
            texture,
        );
    }
    */
    pub fn uniforms<'a>(&'a self,sys:&'a mut SimpleCanvas,texture:&'a Texture,radius:f32)->StaticSpriteUniforms<'a>{
        let un=SpriteProgramUniformValues{radius,color:sys.color,texture,offset:vec2same(0.0)};
        StaticSpriteUniforms{sys,un,buffer:self.buffer.get_info()}
    }
}

pub struct SpriteSession<'a> {
    pub(crate) sys: &'a mut SimpleCanvas,
}

impl SpriteSession<'_> {
    ///Add a point sprite.
    #[inline(always)]
    pub fn add(&mut self, point: PointType, index: TexIndex) -> &mut Self {
        self.sys.sprite_buffer.push(sprite_program::Vertex {
            pos: point,
            index: index.0 as f32,
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

    pub fn uniforms<'a>(&'a mut self,texture: &'a Texture,radius:f32)->SpriteUniforms<'a>{
        let un=SpriteProgramUniformValues{radius,color:self.sys.color,texture,offset:vec2same(0.0)};
        SpriteUniforms{sys:self.sys,un}
    }
}

impl Drop for SpriteSession<'_> {
    fn drop(&mut self) {
        self.sys.sprite_buffer.clear();
    }
}

pub struct Texture {
    _ns: NotSend,
    pub(crate) grid_dim: [u32;2],
    pub(crate) id: GLuint,
}

impl Texture {
    ///Create a texture index from a coordinate in the tile set.
    ///The top left time maps to 0,0. 
    ///The x component grows to the right.
    ///The y component grows downwards.
    pub fn coord_to_index(&self, cell: [u32;2]) -> TexIndex {
        TexIndex(self.grid_dim[0] * cell[0] + cell[1])
    }

    pub fn new(file: &str, grid_dim: [u32;2]) -> image::ImageResult<Texture> {
        match image::open(&file.to_string()) {
            Err(err) => Err(err),
            Ok(img) => {
                use image::GenericImageView;

                let (width, height) = img.dimensions();

                let img = match img {
                    image::DynamicImage::ImageRgba8(img) => img,
                    img => img.to_rgba(),
                };

                let id = build_opengl_mipmapped_texture(width, height, img);
                Ok(Texture {_ns: ns(), id, grid_dim })
            }
        }
    }
}

fn build_opengl_mipmapped_texture(width: u32, height: u32, image: image::RgbaImage) -> GLuint {
    unsafe {
        let mut texture_id: GLuint = 0;
        gl::GenTextures(1, &mut texture_id);
        gl_ok!();

        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl_ok!();

        let raw = image.into_raw();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            raw.as_ptr() as *const _,
        );
        gl_ok!();

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl_ok!();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl_ok!();

        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl_ok!();

        texture_id
    }
}
