use crate::core::error::{RtError, RtResult};
use ash::vk;

pub enum DescriptorSetLayoutBinding<'binding> {
    FramePrivate(vk::DescriptorSetLayoutBinding<'binding>),
    Global(vk::DescriptorSetLayoutBinding<'binding>),
}

pub struct Descriptor {
    set_layouts: Vec<vk::DescriptorSetLayout>,
    pool: vk::DescriptorPool,
    per_frame_sets: Vec<vk::DescriptorSet>,
    global_sets: Vec<vk::DescriptorSet>,
}

impl Descriptor {
    pub fn new(
        device: &ash::Device,
        bindings: &[&[DescriptorSetLayoutBinding]],
        pool_sizes: &[vk::DescriptorPoolSize],
        max_sets: u32,
        frames_in_flight: usize,
    ) -> RtResult<Self> {
        let pool = Self::create_pool(device, pool_sizes, max_sets)?;
        let mut set_layouts: Vec<vk::DescriptorSetLayout> = Vec::with_capacity(bindings.len());
        let mut per_frame_sets: Vec<vk::DescriptorSet> =
            Vec::with_capacity(set_layouts.len() * frames_in_flight);
        let mut global_sets: Vec<vk::DescriptorSet> = Vec::with_capacity(set_layouts.len());
        let mut start_index: usize;

        for binding in bindings.iter() {
            let (set_layout, frame_private_bindings) = Self::create_set_layout(device, binding)?;

            start_index = if frame_private_bindings.is_empty() {
                frames_in_flight - 1
            } else {
                0
            };
            set_layouts.push(set_layout);
            for current_frame in start_index..frames_in_flight {
                let sets_inner = Self::create_descriptor_sets(device, &pool, &set_layout)?;

                sets_inner.iter().enumerate().for_each(|(i, set)| {
                    if frame_private_bindings.contains(&i) {
                        per_frame_sets.push(*set)
                    } else if current_frame == start_index {
                        global_sets.push(*set)
                    }
                });
            }
        }

        Ok(Self {
            set_layouts,
            pool,
            global_sets,
            per_frame_sets,
        })
    }

    #[inline]
    pub fn per_frame_sets(&self) -> &[vk::DescriptorSet] {
        &self.per_frame_sets
    }

    #[inline]
    pub fn global_sets(&self) -> &[vk::DescriptorSet] {
        &self.global_sets
    }

    #[inline]
    pub fn set_layouts(&self) -> &[vk::DescriptorSetLayout] {
        &self.set_layouts
    }

    #[inline]
    pub unsafe fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            self.set_layouts
                .iter_mut()
                .for_each(|set_layout| device.destroy_descriptor_set_layout(*set_layout, None));
        }
    }

    fn create_set_layout(
        device: &ash::Device,
        bindings: &[DescriptorSetLayoutBinding],
    ) -> RtResult<(vk::DescriptorSetLayout, Vec<usize>)> {
        let mut frame_private_bindings: Vec<usize> = Vec::with_capacity(bindings.len());
        let bindings: Vec<vk::DescriptorSetLayoutBinding> = bindings
            .iter()
            .enumerate()
            .map(|(i, binding)| match binding {
                DescriptorSetLayoutBinding::FramePrivate(binding) => {
                    frame_private_bindings.push(i);

                    *binding
                }
                DescriptorSetLayoutBinding::Global(binding) => *binding,
            })
            .collect();
        let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);

        match unsafe { device.create_descriptor_set_layout(&create_info, None) } {
            Ok(descriptor_set_layout) => Ok((descriptor_set_layout, frame_private_bindings)),
            Err(err) => Err(RtError::CreateDescriptorSetLayout(err.into())),
        }
    }

    fn create_pool(
        device: &ash::Device,
        pool_sizes: &[vk::DescriptorPoolSize],
        max_sets: u32,
    ) -> RtResult<vk::DescriptorPool> {
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(pool_sizes)
            .max_sets(max_sets);

        match unsafe { device.create_descriptor_pool(&create_info, None) } {
            Ok(pool) => Ok(pool),
            Err(err) => Err(RtError::CreateDescriptorPool(err.into())),
        }
    }

    fn create_descriptor_sets(
        device: &ash::Device,
        pool: &vk::DescriptorPool,
        set_layout: &vk::DescriptorSetLayout,
    ) -> RtResult<Vec<vk::DescriptorSet>> {
        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(*pool)
            .set_layouts(std::slice::from_ref(set_layout));

        match unsafe { device.allocate_descriptor_sets(&alloc_info) } {
            Ok(sets) => {
                if sets.is_empty() {
                    Err(RtError::AllocateDescriptorSets(None))
                } else {
                    Ok(sets)
                }
            }
            Err(err) => Err(RtError::AllocateDescriptorSets(Some(err.into()))),
        }
    }
}
