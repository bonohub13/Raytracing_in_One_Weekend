// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::core::{RtError, RtResult, device::Device, params, surface::Surface, util::lock_mutex};
use ash::vk;
use gpu_allocator::vulkan::{self, Allocation};
use std::sync::Arc;

pub struct TextureDescriptor<'desc> {
    pub name: &'desc str,
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
}

pub struct Texture {
    surface: Arc<Surface>,
    device: Arc<Device>,
    images: Vec<vk::Image>,
    allocations: Vec<Option<Allocation>>,
    image_views: Vec<vk::ImageView>,
    samplers: Vec<vk::Sampler>,
}

impl Texture {
    pub fn new(desc: &TextureDescriptor) -> RtResult<Self> {
        let images: Vec<vk::Image> = (0..params::MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_image(desc.surface.clone(), desc.device.clone()))
            .collect::<RtResult<_>>()?;
        let allocations: Vec<Option<Allocation>> = images
            .iter()
            .map(|image| {
                Ok(Some(Self::create_allocation(
                    desc.name,
                    desc.device.clone(),
                    *image,
                )?))
            })
            .collect::<RtResult<_>>()?;
        let image_views: Vec<vk::ImageView> = images
            .iter()
            .map(|image| Self::create_image_view(desc.device.clone(), *image))
            .collect::<RtResult<_>>()?;
        let samplers: Vec<vk::Sampler> = (0..params::MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_sampler())
            .collect::<RtResult<_>>()?;

        Ok(Self {
            surface: desc.surface.clone(),
            device: desc.device.clone(),
            images,
            allocations,
            image_views,
            samplers,
        })
    }

    #[inline]
    pub const fn images(&self) -> &[vk::Image] {
        self.images.as_slice()
    }

    #[inline]
    pub const fn image_views(&self) -> &[vk::ImageView] {
        self.image_views.as_slice()
    }

    #[inline]
    pub const fn samplers(&self) -> &[vk::Sampler] {
        self.samplers.as_slice()
    }

    fn create_image(surface: Arc<Surface>, device: Arc<Device>) -> RtResult<vk::Image> {
        let size = surface.inner_size();
        let create_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .extent(vk::Extent3D {
                width: size.width,
                height: size.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(
                vk::ImageUsageFlags::STORAGE
                    | vk::ImageUsageFlags::SAMPLED
                    | vk::ImageUsageFlags::TRANSFER_SRC
                    | vk::ImageUsageFlags::TRANSFER_DST,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);

        match unsafe { device.raw().create_image(&create_info, None) } {
            Ok(image) => Ok(image),
            Err(err) => Err(RtError::CreateImage(err.into())),
        }
    }

    fn create_allocation(
        name: &str,
        device: Arc<Device>,
        image: vk::Image,
    ) -> RtResult<Allocation> {
        let mut guard = lock_mutex!(device.allocator())?;
        let mut dedicated_requirements = vk::MemoryDedicatedRequirements::default();
        let mem_requirements = {
            let mut mem_requirements =
                vk::MemoryRequirements2::default().push_next(&mut dedicated_requirements);
            let info = vk::ImageMemoryRequirementsInfo2::default().image(image);

            unsafe {
                device
                    .raw()
                    .get_image_memory_requirements2(&info, &mut mem_requirements)
            };

            mem_requirements
        };
        let alloc_desc = vulkan::AllocationCreateDesc {
            name,
            requirements: mem_requirements.memory_requirements,
            location: gpu_allocator::MemoryLocation::GpuOnly,
            linear: false,
            allocation_scheme: if dedicated_requirements.requires_dedicated_allocation == vk::TRUE {
                vulkan::AllocationScheme::DedicatedImage(image)
            } else {
                vulkan::AllocationScheme::GpuAllocatorManaged
            },
        };
        let allocation = guard
            .allocate(&alloc_desc)
            .map_err(|err| RtError::CreateAllocation(err.into()))?;
        let bind_info = vk::BindImageMemoryInfo::default()
            .image(image)
            .memory(unsafe { allocation.memory() })
            .memory_offset(allocation.offset());

        drop(guard);
        unsafe {
            device
                .raw()
                .bind_image_memory2(std::slice::from_ref(&bind_info))
        }
        .map_err(|err| RtError::BindImageMemory(err.into()))?;

        Ok(allocation)
    }

    fn create_image_view(device: Arc<Device>, image: vk::Image) -> RtResult<vk::ImageView> {
        const SUBRESOUCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            level_count: 1,
            layer_count: 1,
            base_mip_level: 0,
            base_array_layer: 0,
        };

        let create_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .subresource_range(SUBRESOUCE_RANGE);

        match unsafe { device.raw().create_image_view(&create_info, None) } {
            Ok(image_view) => Ok(image_view),
            Err(err) => Err(RtError::CreateImageView(err.into())),
        }
    }

    fn create_sampler() -> RtResult<vk::Sampler> {
        todo!()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        let device = self.device.raw();
        let mut guard = match self.device.allocator().lock() {
            Ok(guard) => guard,
            Err(err) => {
                eprintln!("{}", RtError::MutexLock(err.to_string()));

                err.into_inner()
            }
        };

        self.samplers
            .iter()
            .for_each(|sampler| unsafe { device.destroy_sampler(*sampler, None) });
        self.image_views
            .iter()
            .for_each(|image_view| unsafe { device.destroy_image_view(*image_view, None) });
        if let Err(err) = self.allocations.iter_mut().try_for_each(|allocation| {
            if let Some(allocation) = allocation.take() {
                guard
                    .free(allocation)
                    .map_err(|err| RtError::FreeAllocation(err.into()))
            } else {
                eprintln!("Allocation for Texture object already freed");

                Ok(())
            }
        }) {
            eprintln!("{err}");
        }
        self.images
            .iter()
            .for_each(|image| unsafe { device.destroy_image(*image, None) });
    }
}
