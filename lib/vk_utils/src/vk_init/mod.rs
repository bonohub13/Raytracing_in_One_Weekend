mod buffer;
mod command;
mod debug;
mod descriptor;
mod device;
mod framebuffer;
mod image;
mod instance;
mod pipeline;
mod render;
mod semaphore;
mod shader;
mod surface;
mod swapchain;

pub use buffer::{copy_buffer_to, copy_to_mapped_memory, create_buffer, map_buffer};
pub use command::{
    create_command_buffer, create_command_buffers, create_command_pool, create_command_pools,
};
pub use debug::DebugUtils;
pub use descriptor::{
    create_descriptor_pool, create_descriptor_set, create_descriptor_set_layout,
    update_descriptor_sets,
};
pub use device::{create_fence, create_logical_device, get_device_queue};
pub use framebuffer::create_framebuffers;
pub use image::{create_image, create_image_view, transition_image};
pub use instance::{
    create_instance, find_queue_families, get_physical_device_format_properties,
    get_physical_device_memory_properties, pick_physical_device,
};
pub use pipeline::{create_compute_pipeline, create_graphics_pipeline, create_pipeline_layout};
pub use render::create_render_pass;
pub use semaphore::create_semaphore;
pub use shader::create_shader_module;
pub use surface::create_surface;
pub use swapchain::{
    choose_swapchain_composite, choose_swapchain_format, choose_swapchain_present_mode,
    choose_swapchain_transform, create_swapchain,
};

fn find_memory_type_index(
    type_filter: u32,
    memory_properties: &ash::vk::PhysicalDeviceMemoryProperties,
    properties: ash::vk::MemoryPropertyFlags,
) -> Result<u32, String> {
    log::info!("finding suitable memory type");

    match (0..memory_properties.memory_type_count)
        .into_iter()
        .find(|&index| {
            type_filter & (1 << index) != 0
                && memory_properties.memory_types[index as usize]
                    .property_flags
                    .contains(properties)
        }) {
        Some(index) => {
            log::info!("found suitable memory type");

            Ok(index)
        }
        None => Err(String::from("unable to find suitable memory type")),
    }
}
