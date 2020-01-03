use super::*;


pub struct SpriteSave{
    _ns: NotSend,
    buffer: vbo::StaticBuffer<sprite_program::Vertex>,
}
impl SpriteSave {
    pub fn draw(&self, session: &mut SimpleCanvas, texture:&mut Texture) {
        session.sprite_program.set_buffer_and_draw(
            texture.radius * GL_POINT_COMP * session.point_mul.0,
            [1.0,1.0,1.0,1.0],
            0,
            self.buffer.get_id(),
            gl::POINTS,
            self.buffer.len(),
            texture.id
        );
    }
}


pub struct SpriteSession<'a> {
    pub(crate) sys: &'a mut SimpleCanvas,
}

impl SpriteSession<'_> {
    pub fn add(&mut self, point: Vec2<f32>,rotation:f32) -> &mut Self {
        self.sys.sprite_buffer.push(sprite_program::Vertex([point.x, point.y,rotation]));
        self
    }

    pub fn addp(&mut self, x:f32,y:f32,rotation:f32) -> &mut Self{
        self.sys.sprite_buffer.push(sprite_program::Vertex([x, y,rotation]));
        self
    }
    pub fn save(&mut self) -> SpriteSave {
        SpriteSave {
            _ns: ns(),
            buffer: vbo::StaticBuffer::new(self.sys.sprite_buffer.get_verts()),
        }
    }
    pub fn send_and_draw(&mut self,texture:&mut Texture) {
        self.sys.sprite_buffer.update();


        self.sys.sprite_program.set_buffer_and_draw(
            texture.radius * GL_POINT_COMP * self.sys.point_mul.0,
            [1.0,1.0,1.0,1.0],
            0,
            self.sys.sprite_buffer.get_id(),
            gl::POINTS,
            self.sys.sprite_buffer.len(),
            texture.id
        );
    }
}
impl Drop for SpriteSession<'_> {
    fn drop(&mut self) {
        self.sys.sprite_buffer.clear();
    }
}

pub struct Texture {
    radius:f32,
    id: GLuint,
}

impl Texture {
    
    pub fn new(file: String) -> image::ImageResult<Texture> {
        match image::open(file.clone()) {
            Err(err) => Err(err),
            Ok(img) => {
                use image::GenericImageView;
                
                let (width, height) = img.dimensions();

                let img = match img {
                    image::DynamicImage::ImageRgba8(img) => img,
                    img => img.to_rgba(),
                };

                let id = build_opengl_mipmapped_texture(width, height, img);
                Ok(Texture { id ,radius:(width.max(height) as f32)*2.0/2.0})
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

        // FIXME of course not always RGBA
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
