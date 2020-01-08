use super::*;
use core::marker::PhantomData;



#[derive(Copy,Clone,Debug)]
pub(crate) struct BufferInfo{
    pub id:u32,
    pub length:usize
}


#[derive(Debug)]
pub struct StaticBuffer<V> {
    info:BufferInfo,
    _p: PhantomData<V>,
}
impl<V> Drop for StaticBuffer<V> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.info.id);
        }
    }
}
impl<V: core::fmt::Debug + Copy + Clone> StaticBuffer<V> {
    pub(crate) fn get_info(&self)->BufferInfo{
        self.info
    }
    pub fn new(data: &[V]) -> StaticBuffer<V> {
        let mut vbo = 0;
        unsafe {
            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl_ok!();
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl_ok!();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<V>()) as GLsizeiptr,
                data.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
        }
        StaticBuffer {
            info : BufferInfo{
                id:vbo,
                length:data.len()
            },
            _p: PhantomData,
        }
    }
}

//TODO make this be composed of a static buffer!!!!!!!!!!
#[derive(Clone, Debug)]
pub struct GrowableBuffer<V> {
    vbo: u32,
    buffer: Vec<V>,
    vbo_size: Option<usize>,
}
impl<V> Drop for GrowableBuffer<V> {
    fn drop(&mut self) {
        //TODO make sure this is ok to do
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl<V: Default> GrowableBuffer<V> {
    pub(crate) fn get_info(&self)->BufferInfo{
        BufferInfo{id:self.vbo,length:self.buffer.len()}
    }
    #[inline(always)]
    pub fn get_verts(&self) -> &[V] {
        &self.buffer
    }

    pub fn new() -> GrowableBuffer<V> {
        let mut vbo = 0;

        let mut buffer = Vec::new();
        //TODO add a pre-set capacity???
        buffer.clear();

        unsafe {
            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl_ok!();
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl_ok!();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.capacity() * mem::size_of::<V>()) as GLsizeiptr,
                buffer.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
            gl_ok!();
        }

        GrowableBuffer {
            vbo,
            buffer,
            vbo_size: None,
        }
    }

    pub fn update(&mut self) {
        let vbo = self.vbo;

        match self.vbo_size {
            Some(l) => {
                if l < self.buffer.capacity() {
                    self.re_generate_buffer();
                }
                assert!(self.vbo_size.unwrap() >= self.buffer.capacity());
            }
            None => {
                self.re_generate_buffer();
            }
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl_ok!();

            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.buffer.len() * mem::size_of::<V>()) as GLsizeiptr,
                self.buffer.as_ptr() as *const _,
            );
            gl_ok!();
        }
    }

    #[inline(always)]
    pub fn push(&mut self, a: V) {
        self.buffer.push(a);
    }


    #[inline(always)]
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    fn re_generate_buffer(&mut self) {
        let vbo = &mut self.vbo;
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.buffer.capacity() * mem::size_of::<V>()) as GLsizeiptr,
                self.buffer.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
            gl_ok!()
        }

        //TODO first confirm the vbo resized??
        self.vbo_size = Some(self.buffer.capacity());
    }
}
