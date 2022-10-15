pub fn create_framebuffers(
    device: &ash::Device,
    render_pass: ash::vk::RenderPass,
    image_views: &Vec<ash::vk::ImageView>,
    color_image_view: Option<ash::vk::ImageView>,
    depth_image_view: Option<ash::vk::ImageView>,
    swapchain_extent: &ash::vk::Extent2D,
) -> Result<Vec<ash::vk::Framebuffer>, String> {
    use ash::vk;

    let mut init_attachments: Vec<vk::ImageView> = vec![];
    let mut framebuffers: Vec<vk::Framebuffer> = vec![];

    if color_image_view.is_some() {
        init_attachments.push(color_image_view.unwrap());
    }
    if depth_image_view.is_some() {
        init_attachments.push(depth_image_view.unwrap());
    }

    for &image_view in image_views.iter() {
        let mut attachments = init_attachments.clone();
        attachments.push(image_view);

        let framebuffer_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(swapchain_extent.width)
            .height(swapchain_extent.height)
            .layers(1);

        let result = unsafe { device.create_framebuffer(&framebuffer_info, None) };

        if result.is_err() {
            return Err(String::from("failed to create framebuffer!"));
        } else {
            framebuffers.push(result.unwrap());
        }
    }

    Ok(framebuffers)
}
