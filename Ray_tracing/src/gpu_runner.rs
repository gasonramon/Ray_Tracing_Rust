extern crate ocl;

use ocl::builders::KernelBuilder;
use ocl::ProQue;

pub struct GpuRunner{

}
impl GpuRunner {
    pub fn trivial() -> ocl::Result<()> {
        let src = r#"
        __kernel void add(__global float* buffer, float scalar) {
            buffer[get_global_id(0)] += scalar;
        }"#;
        let pro_que = ProQue::builder()
            .src(src)
            .dims(1 << 20)
            .build()?;
        let buffer = pro_que.create_buffer::<f32>()?;

        let kernal = pro_que.kernel_builder("add").arg(&buffer).arg(10.0f32).build()?;

        unsafe { kernal.enq()?;}

        let mut vec = vec![0.0f32; buffer.len()];
        buffer.read(&mut vec).enq()?;

        println!("The value at index [{}] is now '{}'!", 200007, vec[200007]);
        Ok(())
    }


}
