
use super::*;
use core::marker::PhantomData;
pub struct BatchCircle<T,F>{
    buffer:vbo::GrowableBuffer<T>,
    func:F,
    _p:PhantomData<T>,
    _ns:NotSend
}

impl<T,F:Fn(&T)->&[f32;2]> BatchCircle<T,F>{
    pub(crate) fn new(bots:&[T],func:F)->BatchCircle<T,F>{
        let mut b=BatchCircle{
            buffer:vbo::GrowableBuffer::new(),
            func,
            _p:PhantomData,
            _ns:ns()
        };
        b.buffer.send_to_gpu(bots);
        b
    }

    pub fn send_and_uniforms<'a>(&'a mut self,sys:&'a mut SimpleCanvas,bots:&[T],radius:f32)->Uniforms<'a>{
    
        self.buffer.send_to_gpu(bots);

        let stride=if bots.len()<2{
            0i32
        }else{

            let first=(self.func)(&bots[0]);
            let second=(self.func)(&bots[1]);
            
            let a=first as *const _ as usize;
            let b=second as *const _ as usize;

            assert!(b>a);

            let tsize=core::mem::size_of::<T>();
            let diff=b-a;
            assert!(diff>=tsize);

            (diff) as i32
        };
        
        let common = UniformCommon {
            color: sys.color,
            offset: vec2same(0.0),
        };
        let un = ProgramUniformValues{
            mode:gl::POINTS,
            radius,
            stride,
            texture:None
        };

        Uniforms {
            sys,
            common,
            un: UniformVals::Circle(un),
            buffer:self.buffer.get_info(bots.len())
        }
    }
}
