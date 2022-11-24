pub struct DescriptorSets {
    binding_types: std::collections::HashMap<u32, ash::vk::DescriptorType>,
    pub descriptor_sets: Vec<ash::vk::DescriptorSet>,
}

impl DescriptorSets {
    pub fn new(
        device: &crate::Device,
        descriptor_pool: &crate::DescriptorPool,
        descriptor_set_layout: &crate::DescriptorSetLayout,
        binding_types: std::collections::HashMap<u32, ash::vk::DescriptorType>,
        size: usize,
    ) -> Result<Self, String> {
        use ash::vk;

        log::info!("creating DescriptorSets");

        let layouts: Vec<vk::DescriptorSetLayout> = (0..size)
            .into_iter()
            .map(|_| descriptor_set_layout.layout)
            .collect();

        let alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(descriptor_pool.descriptor_pool)
            .set_layouts(&layouts)
            .build();

        let descriptor_sets =
            device.allocate_descriptor_sets(descriptor_pool.descriptor_pool, &alloc_info)?;

        Ok(Self {
            descriptor_sets,
            binding_types,
        })
    }

    pub fn bind_from_buffer_info(
        &self,
        index: usize,
        binding: u32,
        buffer_info: &[ash::vk::DescriptorBufferInfo],
    ) -> Result<ash::vk::WriteDescriptorSet, String> {
        use ash::vk;

        let binding_type = self.get_binding_type(binding)?;
        let write_descriptor = vk::WriteDescriptorSet::builder()
            .dst_set(self.descriptor_sets[index])
            .dst_binding(binding)
            .dst_array_element(0)
            .descriptor_type(binding_type)
            .buffer_info(buffer_info)
            .build();

        Ok(write_descriptor)
    }

    pub fn bind_from_image_info(
        &self,
        index: usize,
        binding: u32,
        image_info: &[ash::vk::DescriptorImageInfo],
    ) -> Result<ash::vk::WriteDescriptorSet, String> {
        use ash::vk;

        let binding_type = self.get_binding_type(binding)?;
        let write_descriptor = vk::WriteDescriptorSet::builder()
            .dst_set(self.descriptor_sets[index])
            .dst_binding(binding)
            .dst_array_element(0)
            .descriptor_type(binding_type)
            .image_info(image_info)
            .build();

        Ok(write_descriptor)
    }

    pub fn bind_from_structure_info(
        &self,
        index: usize,
        binding: u32,
        structure_info: &mut ash::vk::WriteDescriptorSetAccelerationStructureKHR,
    ) -> Result<ash::vk::WriteDescriptorSet, String> {
        use ash::vk;

        let binding_type = self.get_binding_type(binding)?;
        let write_descriptor = vk::WriteDescriptorSet::builder()
            .dst_set(self.descriptor_sets[index])
            .dst_binding(binding)
            .dst_array_element(0)
            .descriptor_type(binding_type)
            .push_next(structure_info)
            .build();

        Ok(write_descriptor)
    }

    fn get_binding_type(&self, binding: u32) -> Result<ash::vk::DescriptorType, String> {
        match self
            .binding_types
            .iter()
            .find(|binding_type| *binding_type.0 == binding)
        {
            Some(binding_type) => Ok(binding_type.1.clone()),
            None => Err(String::from("binding not found")),
        }
    }

    pub fn cleanup(
        device: &crate::Device,
        descriptor_pool: &crate::DescriptorPool,
        descriptor_sets: &mut Self,
    ) -> Result<(), String> {
        log::info!("performing cleanup for DescriptorSets");

        device.free_descriptor_sets(
            descriptor_pool.descriptor_pool,
            &descriptor_sets.descriptor_sets,
        )
    }
}
