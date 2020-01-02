use super::*;


pub struct SpriteSession{
    pub(crate) radius: f32,
    pub(crate) sys: &'a mut SimpleCanvas,
}


impl SpriteSession{
	pub fn add(&mut self,pos:Vec2<f32>){
    self.sys.circle_buffer.add(pos);
	}

	pub fn send_and_draw(&mut self){
		unimplemented!()
	}
	pub fn save(&mut self){
		unimplemented!()
	}
}
impl Drop for SpriteSession{
	fn drop(&mut self){
		unimplemented!()
	}
}

pub struct Texture{
	id:GLuint
}


impl Texture {

	pub fn sprites(&mut self,canvas:&mut SimpleCanvas)->SpriteSession{
		unimplemented!()
	}
    pub fn new(file: String) -> image::ImageResult<Texture> {
        match image::open(file.clone()) {
            Err(err) => Err(err),
            Ok(img) => {
            	use image::GenericImageView;
                println!("Dimensions of image are {:?}", img.dimensions());

                let (width, height) = img.dimensions();

                let img = match img {
                    image::DynamicImage::ImageRgba8(img) => img,
                    img => img.to_rgba()
                };

                let id=build_opengl_mipmapped_texture(width, height, img);
            	Ok(Texture{id})
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
        

        let raw=image.into_raw();

        // FIXME of course not always RGBA
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGBA as i32,
                       width as i32,
                       height as i32,
                       0,
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       raw.as_ptr() as *const _);
        gl_ok!();
        
  
        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl_ok!();
        
        texture_id
    }
}
