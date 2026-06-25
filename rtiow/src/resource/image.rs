// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Allocator, Device, Encoder, RtErr, RtError, VkState, util::lock_mutex};
use ash::vk;
use gpu_allocator::vulkan::{self as vk_alloc, Allocation};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum ImageType<'item> {
    Depth(vk::SampleCountFlags, &'item vk::Extent2D),
    Color(vk::SampleCountFlags, &'item vk::Extent2D, vk::Format),
    Storage(&'item vk::Extent2D),
}

pub struct AllocatedImage {
    image_view: vk::ImageView,
    allocation: Option<Allocation>,
    image: vk::Image,
    device: Arc<Device>,
    allocator: Arc<Mutex<Allocator>>,
}

impl AllocatedImage {
    pub fn new(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        encoder: &Encoder,
        ty: ImageType,
    ) -> RtErr<Self> {
        match ty {
            ImageType::Depth(samples, extent) => {
                Self::create_depth_image(state, allocator, encoder, samples, extent)
            }
            ImageType::Color(samples, extent, format) => {
                Self::create_color_image(state, allocator, encoder, samples, extent, format)
            }
            ImageType::Storage(extent) => {
                Self::create_storage_image(state, allocator, encoder, extent)
            }
            #[allow(unused)] // Keep for future implementation(s)
            _ => todo!(),
        }
    }

    #[inline]
    pub fn image(&self) -> vk::Image {
        self.image
    }

    #[inline]
    pub fn image_view(&self) -> vk::ImageView {
        self.image_view
    }

    fn create_depth_image(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        encoder: &Encoder,
        samples: vk::SampleCountFlags,
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
            .samples(samples)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let image = Self::create_image(state, &create_info)?;
        let allocation = Self::allocate_memory(state, allocator.clone(), image)?;
        let image_view = Self::create_image_view(
            state.device.clone(),
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
            allocation,
            image_view,
            device: state.device.clone(),
            allocator,
        })
    }

    fn create_color_image(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        encoder: &Encoder,
        samples: vk::SampleCountFlags,
        extent: &vk::Extent2D,
        format: vk::Format,
    ) -> RtErr<Self> {
        static SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        let create_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(samples)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(
                vk::ImageUsageFlags::TRANSIENT_ATTACHMENT | vk::ImageUsageFlags::COLOR_ATTACHMENT,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let image = Self::create_image(state, &create_info)?;
        let allocation = Self::allocate_memory(state, allocator.clone(), image)?;
        let image_view =
            Self::create_image_view(state.device.clone(), format, image, &SUBRESOURCE_RANGE)?;

        encoder.submit_single_command_buffer(|command_buffer| {
            let memory_barrier = vk::ImageMemoryBarrier2::default()
                .old_layout(vk::ImageLayout::UNDEFINED)
                .src_stage_mask(vk::PipelineStageFlags2::TOP_OF_PIPE)
                .src_access_mask(vk::AccessFlags2::NONE)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .image(image)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
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
            allocation,
            image_view,
            device: state.device.clone(),
            allocator,
        })
    }

    fn create_storage_image(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        encoder: &Encoder,
        extent: &vk::Extent2D,
    ) -> RtErr<Self> {
        static SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };
        const FORMAT: vk::Format = vk::Format::R16G16B16A16_SFLOAT;

        let create_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(FORMAT)
            .extent(vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);
        let image = Self::create_image(state, &create_info)?;
        let allocation = Self::allocate_memory(state, allocator.clone(), image)?;
        let image_view =
            Self::create_image_view(state.device.clone(), FORMAT, image, &SUBRESOURCE_RANGE)?;

        encoder.submit_single_command_buffer(|command_buffer| {
            let memory_barrier = vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::NONE)
                .src_access_mask(vk::AccessFlags2::NONE)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .dst_stage_mask(vk::PipelineStageFlags2::RAY_TRACING_SHADER_KHR)
                .dst_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE_KHR)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .new_layout(vk::ImageLayout::GENERAL)
                .image(image)
                .subresource_range(SUBRESOURCE_RANGE);
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
            allocation,
            image_view,
            device: state.device.clone(),
            allocator,
        })
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

    fn allocate_memory(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        image: vk::Image,
    ) -> RtErr<Option<Allocation>> {
        const IMAGE_MEMORY_NAME: &str = "Image bound memory";

        let device = state.device.device();
        let info = vk::ImageMemoryRequirementsInfo2::default().image(image);
        let requirements = {
            let mut requirements = vk::MemoryRequirements2::default();

            unsafe { device.get_image_memory_requirements2(&info, &mut requirements) };

            requirements.memory_requirements
        };
        let mut guard = lock_mutex!(allocator)?;
        let allocation = guard
            .allocator
            .allocate(&vk_alloc::AllocationCreateDesc {
                name: IMAGE_MEMORY_NAME,
                requirements,
                location: gpu_allocator::MemoryLocation::GpuOnly,
                linear: true,
                allocation_scheme: vk_alloc::AllocationScheme::DedicatedImage(image),
            })
            .map_err(|err| RtError::AllocateMemory(err.into()))?;
        let bind_info = vk::BindImageMemoryInfo::default()
            .image(image)
            .memory(unsafe { allocation.memory() })
            .memory_offset(allocation.offset());

        unsafe { device.bind_image_memory2(std::slice::from_ref(&bind_info)) }
            .map_err(|err| RtError::BindImageMemory(err.into()))?;

        Ok(Some(allocation))
    }

    fn create_image_view(
        device: Arc<Device>,
        format: vk::Format,
        image: vk::Image,
        subresource_range: &vk::ImageSubresourceRange,
    ) -> RtErr<vk::ImageView> {
        let create_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(*subresource_range);

        unsafe { device.device().create_image_view(&create_info, None) }
            .map_err(|err| RtError::CreateImageView(err.into()))
    }
}

impl Drop for AllocatedImage {
    fn drop(&mut self) {
        let device = self.device.device();

        unsafe {
            device.destroy_image(self.image, None);
        }
        if let Some(allocation) = self.allocation.take()
            && let Ok(mut guard) = self.allocator.lock()
            && let Err(err) = guard.free(allocation)
        {
            eprintln!("{err}")
        }
        unsafe {
            device.destroy_image_view(self.image_view, None);
        }
    }
}
