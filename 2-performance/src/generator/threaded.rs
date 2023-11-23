use crate::generator::{Callback, Generator};
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use log::info;

#[derive(Debug)]
pub struct ThreadedGenerator {
    threads: usize,
}

impl ThreadedGenerator {
    pub fn new(threads: usize) -> Self {
        Self { threads }
    }
}

impl Generator for ThreadedGenerator {
    fn generate(&self, camera: &Camera, callback: &Callback) -> OutputBuffer {
        let stored_location = "backup.rgb";
        let output = Arc::new(Mutex::new(OutputBuffer::with_size(
            camera.width,
            camera.height,
            "backup.rgb",
        )));

        thread::scope(|s| {
            let rows_per_thread = (camera.height / self.threads)
                + if camera.height % self.threads == 0 {
                    0
                } else {
                    1
                };

            // ceiling division
            let chunks = (camera.height + rows_per_thread - 1) / rows_per_thread;
            let file = File::create(stored_location).unwrap();
            for index in 0..chunks {
                let start_y = index * rows_per_thread;

                let local_output = Arc::clone(&output);
                let mut file = file.try_clone().unwrap();
                s.spawn(move || {
                    for y in start_y..(start_y + rows_per_thread) {
                        if y >= camera.height {
                            continue;
                        }

                        for x in 0..camera.width {
                            let mut guard = local_output.lock().unwrap();
                            guard.set_at_thread(x, y, callback(x, y), &mut file);
                        }

                        info!("Finished row {}", y);
                    }
                });
            }
        });

        let output = output.lock().unwrap().clone();
        output
    }
}
