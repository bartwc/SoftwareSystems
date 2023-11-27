use rand::rngs::ThreadRng;


pub mod camera;
pub mod color;
pub mod consts;
pub mod outputbuffer;
pub mod ray;
pub mod vector;

pub fn get_rng() -> ThreadRng {
    // Best random distribution
    rand::thread_rng()
}
