pub fn create_logical_device(
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    queue_family_indices: &crate::QueueFamilyIndices,
    required_extensions: &[*const i8],
) -> Result<(ash::Device, crate::ExpectedQueues), String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating logical device");

    let mut count_queue_types = 0;
    let mut queue_indices: Vec<u32> = vec![];
    for queue_index in vec![
        queue_family_indices.compute_family_index,
        queue_family_indices.graphics_family_index,
        queue_family_indices.present_family_index,
    ]
    .iter()
    {
        if !queue_indices.contains(queue_index) {
            queue_indices.push(*queue_index)
        }
    }
    let queue_priority = 1.0f32;
    let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = queue_indices
        .iter()
        .map(|&queue_family_index| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family_index)
                .queue_priorities(&[queue_priority])
                .build()
        })
        .collect();

    let device_features = vk::PhysicalDeviceFeatures::default();

    let create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(required_extensions)
        .enabled_features(&device_features)
        .build();

    let device_sg = {
        let device = unsafe {
            instance
                .create_device(physical_device, &create_info, None)
                .map_err(|err| {
                    log::error!("{}", err);

                    String::from("failed to create logical device")
                })?
        };

        guard(device, |device| {
            log::warn!("device scopeguard");

            unsafe {
                device.destroy_device(None);
            }
        })
    };

    log::info!("created logical device");

    log::info!("creating queues (compute, graphics present)");

    let compute_queue = get_device_queue(&device_sg, queue_family_indices.compute_family_index);
    let graphics_queue = get_device_queue(&device_sg, queue_family_indices.graphics_family_index);
    let present_queue = get_device_queue(&device_sg, queue_family_indices.present_family_index);

    Ok((
        ScopeGuard::into_inner(device_sg),
        crate::ExpectedQueues {
            compute: compute_queue,
            graphics: graphics_queue,
            present: present_queue,
        },
    ))
}

pub fn get_device_queue(device: &ash::Device, queue_family: u32) -> ash::vk::Queue {
    log::info!("creating device queue");

    let queue = unsafe { device.get_device_queue(queue_family, 0) };

    log::info!("created device queue");

    queue
}
