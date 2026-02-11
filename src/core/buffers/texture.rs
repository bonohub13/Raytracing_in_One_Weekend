use crate::core::error::{RtError, RtResult};
use ash::vk;
use gpu_allocator::vulkan as vk_alloc;
use std::sync::Arc;
use winit::window::Window;

pub struct Texture {
    image: vk::Image,
    allocation: Option<vk_alloc::Allocation>,
    pub(super) image_view: vk::ImageView,
}

impl Texture {
    const TEXTURE_FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
    pub fn new(
        window: Arc<Window>,
        allocator: &mut vk_alloc::Allocator,
        device: &ash::Device,
    ) -> RtResult<Self> {
        let image = Self::create_texture_image(window, device)?;
        let allocation = Self::create_allocation(allocator, device, &image)?;
        let image_view = Self::create_image_view(device, &image)?;

        Ok(Self {
            image,
            allocation: Some(allocation),
            image_view,
        })
    }

    #[inline]
    pub fn image(&self) -> &vk::Image {
        &self.image
    }

    pub fn resize(
        &mut self,
        window: Arc<Window>,
        allocator: &mut Option<vk_alloc::Allocator>,
        device: &ash::Device,
    ) -> RtResult<()> {
        unsafe { self.destroy(allocator, device) }?;
        if let Some(allocator) = allocator.as_mut() {
            self.image = Self::create_texture_image(window, device)?;
            self.allocation = Some(Self::create_allocation(allocator, device, &self.image)?);
            self.image_view = Self::create_image_view(device, &self.image)?;
        }

        Ok(())
    }

    pub unsafe fn destroy(
        &mut self,
        allocator: &mut Option<vk_alloc::Allocator>,
        device: &ash::Device,
    ) -> RtResult<()> {
        if let Some(allocator) = allocator.as_mut() {
            unsafe {
                device.destroy_image_view(self.image_view, None);
            }

            if let Some(allocation) = self.allocation.take()
                && let Err(err) = allocator.free(allocation)
            {
                Err(RtError::GpuAllocator(err.into()))
            } else {
                Ok(())
            }?;

            unsafe {
                device.destroy_image(self.image, None);
            }
        }

        Ok(())
    }

    #[inline]
    pub fn image_info(&self, sampler: vk::Sampler) -> vk::DescriptorImageInfo {
        vk::DescriptorImageInfo::default()
            .image_view(self.image_view)
            .sampler(sampler)
    }

    fn create_texture_image(window: Arc<Window>, device: &ash::Device) -> RtResult<vk::Image> {
        let inner_size = window.inner_size();
        let create_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .extent(vk::Extent3D {
                width: inner_size.width,
                height: inner_size.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .format(Self::TEXTURE_FORMAT)
            .tiling(vk::ImageTiling::OPTIMAL)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .samples(vk::SampleCountFlags::TYPE_1);

        match unsafe { device.create_image(&create_info, None) } {
            Ok(image) => Ok(image),
            Err(err) => Err(RtError::CreateImage(err.into())),
        }
    }

    fn create_allocation(
        allocator: &mut vk_alloc::Allocator,
        device: &ash::Device,
        image: &vk::Image,
    ) -> RtResult<vk_alloc::Allocation> {
        const TEXTURE_NAME: &str = "Compute Texture";

        let requirements = unsafe { device.get_image_memory_requirements(*image) };
        let desc = vk_alloc::AllocationCreateDesc {
            name: TEXTURE_NAME,
            requirements,
            linear: true,
            location: gpu_allocator::MemoryLocation::GpuOnly,
            allocation_scheme: vk_alloc::AllocationScheme::GpuAllocatorManaged,
        };

        match allocator.allocate(&desc) {
            Ok(allocation) => {
                if let Err(err) = unsafe {
                    device.bind_image_memory(*image, allocation.memory(), allocation.offset())
                } {
                    Err(RtError::BindImageMemory(err.into()))
                } else {
                    Ok(allocation)
                }
            }
            Err(err) => Err(RtError::GpuAllocator(err.into())),
        }
    }

    fn create_image_view(device: &ash::Device, image: &vk::Image) -> RtResult<vk::ImageView> {
        const SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        let create_info = vk::ImageViewCreateInfo::default()
            .image(*image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(Self::TEXTURE_FORMAT)
            .subresource_range(SUBRESOURCE_RANGE);

        match unsafe { device.create_image_view(&create_info, None) } {
            Ok(image_view) => Ok(image_view),
            Err(err) => Err(RtError::CreateImageView(err.into())),
        }
    }
}
