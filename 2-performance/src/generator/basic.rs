use crate::generator::{Callback, Generator};
use std::fs::File;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;

#[derive(Debug)]
pub struct BasicGenerator;

impl Generator for BasicGenerator {
    fn generate(&self, camera: &Camera, callback: &Callback) -> OutputBuffer {
        let stored_location = "backup.rgb";
        let mut output = OutputBuffer::with_size(camera.width, camera.height, stored_location);

        let mut stored_file = File::create(stored_location).unwrap();
        for x in 0..camera.width {
            for y in 0..camera.height {
                let res = callback(x, y);
                output.set_at_base(x, y, res, &mut stored_file);
            }
        }

        output
    }
}
