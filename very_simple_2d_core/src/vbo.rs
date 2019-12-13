use super::*;

#[derive(Clone,Debug)]
pub struct GrowableBuffer<V>{
    vbo:u32,
    buffer:Vec<V>
}
impl<V> Drop for GrowableBuffer<V>{
    fn drop(&mut self){
        //TODO make sure this is ok to do
        unsafe{
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl<V:Default> GrowableBuffer<V>{

    pub fn new()->GrowableBuffer<V>{
        let mut vbo = 0;
        
        let buffer=Vec::new();
        
        
        unsafe {

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.len() *mem::size_of::<V>()) as GLsizeiptr,
                mem::transmute(buffer.as_ptr()),
                gl::DYNAMIC_DRAW,
            );

            
        }


        let mut b = GrowableBuffer{vbo,buffer};
        b.re_generate_buffer();
        b
    }
    pub fn get_id(&self)->u32{
        self.vbo
    }

    pub fn push(&mut self,a:V){
        if self.buffer.len()==self.buffer.capacity(){
            self.push(a);
            self.re_generate_buffer();

        }
    }

    pub fn len(&self)->usize{
        self.buffer.len()
    }

    pub fn capacity(&self)->usize{
        self.buffer.capacity()
    }

    pub fn clear(&mut self){
        self.buffer.clear();
    }
    
    /*
    pub fn update(&mut self){
        let vbo=&mut self.vbo;
        
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
            
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.buffer.len()*mem::size_of::<V>()) as GLsizeiptr,
                mem::transmute(self.buffer.as_ptr()),
            );
        }
        assert_eq!(unsafe{gl::GetError()},gl::NO_ERROR);   
    }
    */
    
    pub fn get_num_verticies(&self)->usize{
        self.buffer.len()
    }


    fn re_generate_buffer(&mut self){
        
         
        //self.buffer.resize_with(num_verticies,Default::default);
        let vbo=&mut self.vbo;
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.buffer.capacity() *mem::size_of::<V>()) as GLsizeiptr,
                mem::transmute(self.buffer.as_ptr()),
                gl::DYNAMIC_DRAW,
            );
        }
        assert_eq!(unsafe{gl::GetError()},gl::NO_ERROR);
        
    }
}