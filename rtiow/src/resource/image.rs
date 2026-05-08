use crate::{Device, Encoder, RtErr, RtError, VkState};
use ash::vk;
use std::sync::Arc;

pub struct AllocatedImage {
    image_view: vk::ImageView,
    memory: vk::DeviceMemory,
    image: vk::Image,
    device: Arc<Device>,
}

impl AllocatedImage {
    pub(crate) fn create_depth_image(
        state: &VkState,
        encoder: &Encoder,
        extent: &vk::Extent2D,
    ) -> RtErr<Self> {
        let depth_format = Self::find_depth_format(state);
        let create_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(depth_format)
            .extent(vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let image = Self::create_image(state, &create_info)?;
        let memory = Self::allocate_memory(state, image)?;
        let image_view = Self::create_image_view(
            state,
            depth_format,
            image,
            &vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::DEPTH,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        )?;

        encoder.submit_single_command_buffer(|command_buffer| {
            let memory_barrier = vk::ImageMemoryBarrier2::default()
                .old_layout(vk::ImageLayout::UNDEFINED)
                .src_stage_mask(vk::PipelineStageFlags2::TOP_OF_PIPE)
                .src_access_mask(vk::AccessFlags2::NONE)
                .new_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                .dst_stage_mask(vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS)
                .dst_access_mask(vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE)
                .image(image)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::DEPTH,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });
            let dependency_info = vk::DependencyInfo::default()
                .image_memory_barriers(std::slice::from_ref(&memory_barrier));

            unsafe {
                state
                    .device
                    .device()
                    .cmd_pipeline_barrier2(command_buffer, &dependency_info);
            }
        })?;

        Ok(Self {
            image,
            memory,
            image_view,
            device: state.device.clone(),
        })
    }

    #[inline]
    pub fn image(&self) -> vk::Image {
        self.image
    }

    #[inline]
    pub fn image_view(&self) -> vk::ImageView {
        self.image_view
    }

    fn find_depth_format(state: &VkState) -> vk::Format {
        Self::find_supported_format(
            state,
            &[
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        )
    }

    fn has_stencil_component(format: vk::Format) -> bool {
        format == vk::Format::D32_SFLOAT_S8_UINT || format == vk::Format::D24_UNORM_S8_UINT
    }

    fn find_supported_format(
        state: &VkState,
        candidates: &[vk::Format],
        tiling: vk::ImageTiling,
        features: vk::FormatFeatureFlags,
    ) -> vk::Format {
        candidates
            .iter()
            .find_map(|format| {
                let mut properties = vk::FormatProperties2::default();

                unsafe {
                    state
                        .instance
                        .instance()
                        .get_physical_device_format_properties2(
                            state.device.physical_device(),
                            *format,
                            &mut properties,
                        );
                }

                if (tiling == vk::ImageTiling::LINEAR
                    && properties
                        .format_properties
                        .linear_tiling_features
                        .contains(features))
                    || (tiling == vk::ImageTiling::OPTIMAL
                        && properties
                            .format_properties
                            .optimal_tiling_features
                            .contains(features))
                {
                    Some(*format)
                } else {
                    None
                }
            })
            .expect("Failed to find supported format")
    }

    fn create_image(state: &VkState, create_info: &vk::ImageCreateInfo) -> RtErr<vk::Image> {
        unsafe { state.device.device().create_image(create_info, None) }
            .map_err(|err| RtError::CreateImages(err.into()))
    }

    fn allocate_memory(state: &VkState, image: vk::Image) -> RtErr<vk::DeviceMemory> {
        let device = state.device.device();
        let info = vk::ImageMemoryRequirementsInfo2::default().image(image);
        let requirements = {
            let mut requirements = vk::MemoryRequirements2::default();

            unsafe { device.get_image_memory_requirements2(&info, &mut requirements) };

            requirements.memory_requirements
        };
        let type_index = Self::find_memory_type(
            requirements.memory_type_bits,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            state.device.memory_properties(),
        );
        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(type_index);
        let memory = unsafe { device.allocate_memory(&alloc_info, None) }
            .map_err(|err| RtError::AllocateMemory(err.into()))?;
        let bind_info = vk::BindImageMemoryInfo::default()
            .image(image)
            .memory(memory)
            .memory_offset(0);

        unsafe { device.bind_image_memory2(std::slice::from_ref(&bind_info)) }
            .map_err(|err| RtError::BindImageMemory(err.into()))?;

        Ok(memory)
    }

    fn create_image_view(
        state: &VkState,
        format: vk::Format,
        image: vk::Image,
        subresource_range: &vk::ImageSubresourceRange,
    ) -> RtErr<vk::ImageView> {
        let create_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(*subresource_range);

        unsafe { state.device.device().create_image_view(&create_info, None) }
            .map_err(|err| RtError::CreateImageView(err.into()))
    }

    fn find_memory_type(
        type_filter: u32,
        properties: vk::MemoryPropertyFlags,
        mem_properties: &vk::PhysicalDeviceMemoryProperties,
    ) -> u32 {
        (0..mem_properties.memory_type_count)
            .find(|i| {
                (type_filter & (1 << *i)) != 0
                    && mem_properties.memory_types[*i as usize]
                        .property_flags
                        .contains(properties)
            })
            .expect("Failed to find suitable memory type")
    }
}

impl Drop for AllocatedImage {
    fn drop(&mut self) {
        let device = self.device.device();

        unsafe {
            device.destroy_image(self.image, None);
            device.free_memory(self.memory, None);
            device.destroy_image_view(self.image_view, None);
        }
    }
}
