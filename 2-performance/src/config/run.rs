use crate::config::error::ConfigError;
use crate::config::{Config, GeneratorConfig};
use crate::datastructure::bvh::KDTreeDataStructure;
// use crate::datastructure::DataStructure;
use crate::generator::basic::BasicGenerator;
use crate::generator::threaded::ThreadedGenerator;
use crate::generator::Generator;
use crate::raytracer::mstracer::MSTracer;

use crate::renderer::RendererBuilder;
use crate::scene::scene::SceneBuilder;
use crate::shader::mcshader::McShader;

use crate::util::camera::Camera;

use std::path::PathBuf;
use std::sync::{Arc};

impl Config {
    pub fn run(self) -> Result<(), ConfigError> {
        let (model, mtls) = tobj::load_obj(self.general.scenename.as_ref())?;

        let scene = SceneBuilder::default()
            .texturepath(PathBuf::from(&self.general.texturepath))
            .build_from_tobj((model, mtls))?;
        let generator: Box<dyn Generator> = match self.generator {
            GeneratorConfig::Basic => Box::new(BasicGenerator),
            GeneratorConfig::Threaded { threads } => {
                Box::new(ThreadedGenerator::new(threads.get_cores()))
            }
        };
        let raytracer = Box::new(MSTracer::new(self.raytracer.samples_per_pixel));
        let shader = Box::new(McShader);
        let datastructure =
            Box::new(KDTreeDataStructure::new(&scene));
        let renderer = RendererBuilder::new(generator.as_ref())
            .with_raytracer(raytracer.as_ref())
            .with_shader(shader.as_ref())
            .with_datastructure(datastructure.as_ref())
            .build();

        let camera = Camera::new(
            self.camera.position,
            self.camera.direction,
            self.camera.width,
            self.camera.height,
            self.camera.fov,
        );

        dbg!(&renderer);

        renderer
            .render(&camera)
            .to_bmp()
            .save(self.general.outputname)?;

        Ok(())
    }
}
