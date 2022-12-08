mod buffer;
mod command;
mod debug;
mod descriptor;
mod device;
mod image;
mod instance;
mod pixel_color;
mod render;
mod semaphore;
mod surface;
mod swapchain;

pub use buffer::create_buffer;
pub use command::{create_command_buffer, create_command_pool};
pub use debug::DebugUtils;
pub use descriptor::{create_descriptor_pool, create_descriptor_set_layout};
pub use device::{create_fence, create_logical_device, get_device_queue};
pub use image::{create_image, create_image_view};
pub use instance::{create_instance, find_queue_families, pick_physical_device};
pub use pixel_color::create_summed_pixel_color_image;
pub use render::create_render_call_info_buffer;
pub use semaphore::create_semaphore;
pub use surface::create_surface;
pub use swapchain::{
    choose_swapchain_composite, choose_swapchain_format, choose_swapchain_present_mode,
    choose_swapchain_transform, create_swapchain,
};

fn find_memory_type_index(
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    memory_type_bits: u32,
    properties: ash::vk::MemoryPropertyFlags,
) -> Result<u32, String> {
    log::info!("finding suitable memory type");

    let memory_properties =
        unsafe { instance.get_physical_device_memory_properties(physical_device) };

    match (0..memory_properties.memory_type_count)
        .into_iter()
        .find(|&index| {
            memory_type_bits & (1 << index) != 0
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
