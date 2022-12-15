pub fn create_framebuffers(
    device: &ash::Device,
    render_pass: ash::vk::RenderPass,
    color_attachments: &Vec<ash::vk::ImageView>,
    width: u32,
    height: u32,
) -> Result<Vec<ash::vk::Framebuffer>, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating framebuffers");

    let mut framebuffers: Vec<vk::Framebuffer> = Vec::new();
    for &color_attachment in color_attachments.iter() {
        let attachments = vec![color_attachment];
        let create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(width)
            .height(height)
            .layers(1)
            .build();

        let fb_sg = {
            let framebuffer = unsafe {
                device
                    .create_framebuffer(&create_info, None)
                    .map_err(|_| String::from("failed to create framebuffer"))?
            };

            guard(framebuffer, |fb| {
                log::warn!("framebuffer scopeguard");

                unsafe {
                    device.destroy_framebuffer(fb, None);
                }
            })
        };

        framebuffers.push(ScopeGuard::into_inner(fb_sg));
    }

    log::info!("created framebuffers");

    Ok(framebuffers)
}
