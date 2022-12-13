pub fn create_render_pass(
    device: &ash::Device,
    color_format: ash::vk::Format,
) -> Result<ash::vk::RenderPass, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating render pass");

    let color_attachment = vk::AttachmentDescription::builder()
        .format(color_format)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .build();

    let color_ref = vk::AttachmentReference::builder()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build();
    let subpass = vk::SubpassDescription::builder()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&[color_ref])
        .build();

    let dependency = vk::SubpassDependency::builder()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk::AccessFlags::empty())
        .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .build();

    let create_info = vk::RenderPassCreateInfo::builder()
        .attachments(&[color_attachment])
        .subpasses(&[subpass])
        .dependencies(&[dependency])
        .build();

    let rp_sg = {
        let render_pass = unsafe {
            device
                .create_render_pass(&create_info, None)
                .map_err(|_| String::from("failed to create render pass"))?
        };

        guard(render_pass, |render_pass| {
            log::info!("render pass scopeguard");

            unsafe {
                device.destroy_render_pass(render_pass, None);
            }
        })
    };

    log::info!("created render pass");

    Ok(ScopeGuard::into_inner(rp_sg))
}
