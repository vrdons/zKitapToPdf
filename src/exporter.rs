use image::RgbaImage;
use ruffle_core::{PlayerBuilder, limits::ExecutionLimit, tag_utils::SwfMovie};
use ruffle_render_wgpu::{
    backend::WgpuRenderBackend, backend::request_adapter_and_device, clap::GraphicsBackend,
    descriptors::Descriptors, target::TextureTarget, wgpu,
};
use std::sync::Arc;

use anyhow::{Result, anyhow};

#[derive(Copy, Clone)]
pub struct SizeOpt {
    pub scale: f64,

    pub width: u32,

    pub height: u32,
}

pub struct ExporterOpt {
    pub graphics: GraphicsBackend,
    pub scale: f64,
}

pub struct Exporter {
    descriptors: Arc<Descriptors>,
    scale: f64,
}

impl Exporter {
    pub fn new(opt: &ExporterOpt) -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: opt.graphics.into(),
            ..Default::default()
        });
        let (adapter, device, queue) = futures::executor::block_on(request_adapter_and_device(
            opt.graphics.into(),
            &instance,
            None,
            wgpu::PowerPreference::HighPerformance,
        ))
        .map_err(|e| anyhow!(e.to_string()))?;

        let descriptors = Arc::new(Descriptors::new(instance, adapter, device, queue));

        Ok(Self {
            descriptors,
            scale: opt.scale,
        })
    }

    pub fn capture_frames(&self, file: &mut [u8]) -> Result<Vec<RgbaImage>> {
        let movie = SwfMovie::from_data(file, "".to_string(), None)?;
        let total_frame = movie.num_frames();

        let width = movie.width().to_pixels();
        let width = (width * self.scale).round() as u32;

        let height = movie.height().to_pixels();
        let height = (height * self.scale).round() as u32;

        let target = TextureTarget::new(&self.descriptors.device, (width, height))
            .map_err(|e| anyhow!(e.to_string()))?;
        let player = PlayerBuilder::new()
            .with_renderer(
                WgpuRenderBackend::new(self.descriptors.clone(), target)
                    .map_err(|e| anyhow!(e.to_string()))?,
            )
            .with_movie(movie)
            .with_viewport_dimensions(width, height, self.scale)
            .build();
        let mut result = Vec::new();

        let mut locked_player = player.lock().unwrap();
        for i in 0..total_frame {
            locked_player.preload(&mut ExecutionLimit::none());

            locked_player.run_frame();
            locked_player.render();

            let image = {
                let renderer =
                    <dyn std::any::Any>::downcast_mut::<WgpuRenderBackend<TextureTarget>>(
                        locked_player.renderer_mut(),
                    )
                    .ok_or_else(|| anyhow!("Renderer type mismatch"))?;

                renderer.capture_frame()
            };

            match image {
                Some(img) => {
                    println!("Capturing frame: {}", i);
                    result.push(img);
                }
                None => return Err(anyhow!("No frame captured")),
            }
        }
        Ok(result)
    }
}
