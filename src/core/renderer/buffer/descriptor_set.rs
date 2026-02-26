use crate::core::{RtError, RtResult, device::Device, params, renderer::buffer::GraphicsBuffer};
use ash::vk;
use std::sync::Arc;

pub struct DescriptorSetDescriptor<'desc> {
    pub device: Arc<Device>,
    pub bindings: &'desc [vk::DescriptorSetLayoutBinding<'desc>],
}

pub struct DescriptorSet {
    device: Arc<Device>,
    layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    graphics_sets: Vec<vk::DescriptorSet>,
}

impl DescriptorSet {
    pub fn new(desc: &DescriptorSetDescriptor) -> RtResult<Self> {
        let layout = Self::create_layout(desc.device.clone(), desc.bindings)?;
        let pool = Self::create_pool(desc.device.clone())?;
        let graphics_sets = Self::allocate_sets(desc.device.clone(), layout, pool)?;

        Ok(Self {
            device: desc.device.clone(),
            layout,
            pool,
            graphics_sets,
        })
    }

    #[inline]
    pub const fn layout(&self) -> vk::DescriptorSetLayout {
        self.layout
    }

    #[inline]
    pub const fn graphics_sets(&self) -> &[vk::DescriptorSet] {
        self.graphics_sets.as_slice()
    }

    fn create_layout(
        device: Arc<Device>,
        bindings: &[vk::DescriptorSetLayoutBinding],
    ) -> RtResult<vk::DescriptorSetLayout> {
        let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(bindings);

        unsafe {
            device
                .raw()
                .create_descriptor_set_layout(&create_info, None)
        }
        .map_err(|err| RtError::CreateDescriptorSetLayout(err.into()))
    }

    fn create_pool(device: Arc<Device>) -> RtResult<vk::DescriptorPool> {
        const PER_PIPELINE_DESCRIPTOR_SET_COUNT: u32 = params::MAX_FRAMES_IN_FLIGHT as u32;
        const DESCRIPTOR_SET_TYPES: u32 = 2;

        let pool_sizes = [GraphicsBuffer::pool_sizes()].concat();
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(PER_PIPELINE_DESCRIPTOR_SET_COUNT * DESCRIPTOR_SET_TYPES);

        unsafe { device.raw().create_descriptor_pool(&create_info, None) }
            .map_err(|err| RtError::CreateDescriptorPool(err.into()))
    }

    fn allocate_sets(
        device: Arc<Device>,
        set_layout: vk::DescriptorSetLayout,
        pool: vk::DescriptorPool,
    ) -> RtResult<Vec<vk::DescriptorSet>> {
        let set_layouts = vec![set_layout; params::MAX_FRAMES_IN_FLIGHT];
        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(&set_layouts);

        unsafe { device.raw().allocate_descriptor_sets(&alloc_info) }
            .map_err(|err| RtError::AllocateDescriptorSets(err.into()))
    }
}

impl Drop for DescriptorSet {
    fn drop(&mut self) {
        let device = self.device.raw();

        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}
