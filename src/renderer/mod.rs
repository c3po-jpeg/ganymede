pub mod core;

use core::Core;

pub fn render(core: &mut Core) -> anyhow::Result<()> {
    core.window().request_redraw();

    if !core.is_surface_configured() {
        return Ok(());
    }

    let output = match core.surface().get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
        wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
            log::warn!("Surface is suboptimal");
            surface_texture
        }
        wgpu::CurrentSurfaceTexture::Timeout
        | wgpu::CurrentSurfaceTexture::Occluded
        | wgpu::CurrentSurfaceTexture::Validation => {
            return Ok(());
        }
        wgpu::CurrentSurfaceTexture::Outdated => {
            core.surface()
                .configure(core.device(), core.surface_config());
            return Ok(());
        }
        wgpu::CurrentSurfaceTexture::Lost => {
            anyhow::bail!("Lost device");
        }
    };

    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = core
        .device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Graphics render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.5,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }

    core.queue().submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}
