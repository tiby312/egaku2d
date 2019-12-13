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
        
        let mut buffer=Vec::new();
        buffer.resize_with(1000,core::default::Default::default);
        buffer.clear();
        
        
        unsafe {

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.capacity() *mem::size_of::<V>()) as GLsizeiptr,
                mem::transmute(buffer.as_ptr()),
                gl::DYNAMIC_DRAW,
            );

            
        }


        GrowableBuffer{vbo,buffer}
    }


    pub fn update(&mut self){
        let vbo=&mut self.vbo;
        
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
            
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.buffer.capacity()*mem::size_of::<V>()) as GLsizeiptr,
                mem::transmute(self.buffer.as_ptr()),
            );
        }
        assert_eq!(unsafe{gl::GetError()},gl::NO_ERROR);   
    }

    pub fn get_id(&self)->u32{
        self.vbo
    }

    pub fn push(&mut self,a:V){
        if self.buffer.len() == self.buffer.capacity(){
            panic!("fail");
            /*
            self.buffer.push(a);
            assert!(self.buffer.len()!=self.buffer.capacity(),"vec did not grow:{:?}",(self.buffer.len(),self.buffer.capacity()));
            self.re_generate_buffer();
            */
        }else{
            self.buffer.push(a);
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
    
    
    pub fn get_num_verticies(&self)->usize{
        self.buffer.len()
    }


    fn re_generate_buffer(&mut self){
        unimplemented!();
            
         
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