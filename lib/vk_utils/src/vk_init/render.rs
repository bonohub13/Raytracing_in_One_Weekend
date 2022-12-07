pub fn create_render_call_info_buffer(
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    device: &ash::Device,
) -> Result<crate::VkBuffer, String> {
    use ash::vk;
    use std::mem::size_of;

    log::info!("creating VkRenderCallInfo buffer");

    let render_call_info_buffer = crate::vk_init::create_buffer(
        instance,
        physical_device,
        device,
        size_of::<crate::VkRenderCallInfo>() as u64,
        vk::BufferUsageFlags::UNIFORM_BUFFER,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    )
    .map_err(|_| String::from("failed to create VkRenderCallInfo buffer"))?;

    log::info!("created VkRenderCallInfo buffer");

    Ok(render_call_info_buffer)
}
