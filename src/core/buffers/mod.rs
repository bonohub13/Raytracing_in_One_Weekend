mod buffer;
mod descriptor;
mod texture;

pub use buffer::*;
pub use descriptor::*;
pub use texture::*;

use crate::core::{
    MAX_FRAMES_IN_FLIGHT,
    error::{RtError, RtResult},
    objects::Camera,
};
use ash::vk;
use gpu_allocator::vulkan as vk_alloc;
use std::sync::Arc;
use winit::window::Window;

pub struct Buffers {
    allocator: Option<vk_alloc::Allocator>,
    camera: Camera,
    camera_buffers: Vec<Buffer>,
    texture: Texture,
    sampler: vk::Sampler,
    descriptor: Descriptor,
}

impl Buffers {
    pub fn new(
        window: Arc<Window>,
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: &vk::PhysicalDevice,
        camera: &Camera,
    ) -> RtResult<Self> {
        let mut allocator = Self::create_allocator(instance, device, physical_device)?;
        let camera_buffers = {
            let mut camera_buffers: Vec<Buffer> = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);

            for _ in 0..MAX_FRAMES_IN_FLIGHT {
                camera_buffers.push(camera.create_uniform_buffer(&mut allocator, device)?);
            }

            camera_buffers
        };
        let texture = Texture::new(window, &mut allocator, device)?;
        let sampler = Self::create_sampler(instance, device, physical_device)?;
        let compute_bindings = Self::compute_bindings();
        let graphics_bindings = Self::graphics_bindings();
        let pool_sizes = Self::pool_sizes();
        let descriptor = Descriptor::new(
            device,
            &[&compute_bindings, &graphics_bindings],
            &pool_sizes,
            pool_sizes.len() as u32,
            MAX_FRAMES_IN_FLIGHT,
        )?;
        let per_frame_sets = descriptor.per_frame_sets();
        let global_sets = descriptor.global_sets();
        let buffer_infos: Vec<_> = camera_buffers
            .iter()
            .map(|buffer| buffer.buffer_info().offset(0))
            .collect();
        let compute_image_info = texture
            .image_info(vk::Sampler::null())
            .image_layout(vk::ImageLayout::GENERAL);
        let graphics_image_info = texture
            .image_info(sampler)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let compute_writes: Vec<_> = per_frame_sets
            .iter()
            .enumerate()
            .map(|(i, set)| {
                [
                    vk::WriteDescriptorSet::default()
                        .dst_set(*set)
                        .dst_binding(0)
                        .descriptor_count(1)
                        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                        .buffer_info(std::slice::from_ref(&buffer_infos[i])),
                    vk::WriteDescriptorSet::default()
                        .dst_set(*set)
                        .dst_binding(1)
                        .descriptor_count(1)
                        .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                        .image_info(std::slice::from_ref(&compute_image_info)),
                ]
            })
            .collect::<Vec<_>>()
            .concat();
        let graphics_writes = vec![
            // Graphics
            vk::WriteDescriptorSet::default()
                .dst_set(global_sets[0])
                .dst_binding(0)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(std::slice::from_ref(&graphics_image_info)),
        ];
        let writes = [compute_writes, graphics_writes].concat();

        unsafe { device.update_descriptor_sets(writes.as_slice(), &[]) };

        Ok(Self {
            allocator: Some(allocator),
            camera: *camera,
            camera_buffers,
            texture,
            sampler,
            descriptor,
        })
    }

    #[inline]
    pub const fn camera(&self) -> &Camera {
        &self.camera
    }

    #[inline]
    pub const fn texture(&self) -> &Texture {
        &self.texture
    }

    #[inline]
    pub fn compute_sets(&self) -> &[vk::DescriptorSet] {
        self.descriptor.per_frame_sets()
    }

    #[inline]
    pub fn graphics_sets(&self) -> &[vk::DescriptorSet] {
        self.descriptor.global_sets()
    }

    #[inline]
    pub fn descriptor_set_layouts(&self) -> &[vk::DescriptorSetLayout] {
        self.descriptor.set_layouts()
    }

    pub fn resize(&mut self, window: Arc<Window>, device: &ash::Device) -> RtResult<()> {
        let inner_size = window.inner_size();

        self.camera.width = inner_size.width;
        self.camera.height = inner_size.height;
        self.camera_buffers
            .iter_mut()
            .for_each(|buffer| buffer.write(self.camera));
        self.update_camera(device);
        self.texture.resize(window, &mut self.allocator, device)?;
        self.update_texture(device);

        Ok(())
    }

    pub unsafe fn destroy(&mut self, device: &ash::Device) -> RtResult<()> {
        unsafe {
            for camera_buffer in self.camera_buffers.iter_mut() {
                camera_buffer.destroy(&mut self.allocator, device)?;
            }
            self.texture.destroy(&mut self.allocator, device)?;
            device.destroy_sampler(self.sampler, None);
            self.descriptor.destroy(device);
        }
        if let Some(allocator) = self.allocator.take() {
            drop(allocator);
        }

        Ok(())
    }

    fn update_camera(&self, device: &ash::Device) {
        self.camera_buffers
            .iter()
            .enumerate()
            .for_each(|(i, buffer)| {
                let buffer_info = buffer.buffer_info();
                let write = vk::WriteDescriptorSet::default()
                    .dst_set(self.descriptor.per_frame_sets()[i])
                    .dst_binding(0)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(std::slice::from_ref(&buffer_info));

                unsafe { device.update_descriptor_sets(std::slice::from_ref(&write), &[]) }
            })
    }

    fn update_texture(&self, device: &ash::Device) {
        let compute_image_info = self
            .texture
            .image_info(vk::Sampler::null())
            .image_layout(vk::ImageLayout::GENERAL);
        let graphics_image_info = self
            .texture
            .image_info(self.sampler)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let compute_sets = self.descriptor.per_frame_sets();
        let graphics_sets = self.descriptor.global_sets();
        let writes = [
            // Compute
            vk::WriteDescriptorSet::default()
                .dst_set(compute_sets[0])
                .dst_binding(1)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .image_info(std::slice::from_ref(&compute_image_info)),
            // Compute
            vk::WriteDescriptorSet::default()
                .dst_set(compute_sets[1])
                .dst_binding(1)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .image_info(std::slice::from_ref(&compute_image_info)),
            // Graphics
            vk::WriteDescriptorSet::default()
                .dst_set(graphics_sets[0])
                .dst_binding(0)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(std::slice::from_ref(&graphics_image_info)),
        ];

        unsafe { device.update_descriptor_sets(&writes, &[]) }
    }

    #[inline]
    fn compute_bindings() -> [DescriptorSetLayoutBinding<'static>; 2] {
        [
            DescriptorSetLayoutBinding::FramePrivate(
                vk::DescriptorSetLayoutBinding::default()
                    .binding(0)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE),
            ),
            DescriptorSetLayoutBinding::Global(
                vk::DescriptorSetLayoutBinding::default()
                    .binding(1)
                    .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE),
            ),
        ]
    }

    #[inline]
    fn graphics_bindings() -> [DescriptorSetLayoutBinding<'static>; 1] {
        [DescriptorSetLayoutBinding::Global(
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        )]
    }

    fn pool_sizes() -> Vec<vk::DescriptorPoolSize> {
        let compute_pool_sizes = vec![
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(MAX_FRAMES_IN_FLIGHT as u32),
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(MAX_FRAMES_IN_FLIGHT as u32),
        ];
        let graphics_pool_sizes = vec![
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1),
        ];

        [compute_pool_sizes, graphics_pool_sizes].concat().to_vec()
    }

    fn create_allocator(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: &vk::PhysicalDevice,
    ) -> RtResult<vk_alloc::Allocator> {
        match vk_alloc::Allocator::new(&vk_alloc::AllocatorCreateDesc {
            instance: instance.clone(),
            device: device.clone(),
            physical_device: *physical_device,
            buffer_device_address: false,
            debug_settings: Default::default(),
            allocation_sizes: Default::default(),
        }) {
            Ok(allocator) => Ok(allocator),
            Err(err) => Err(RtError::GpuAllocator(err.into())),
        }
    }

    fn create_sampler(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: &vk::PhysicalDevice,
    ) -> RtResult<vk::Sampler> {
        let properties = unsafe { instance.get_physical_device_properties(*physical_device) };
        let create_info = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::NEAREST)
            .min_filter(vk::Filter::NEAREST)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
            .anisotropy_enable(true)
            .max_anisotropy(properties.limits.max_sampler_anisotropy)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mip_lod_bias(0f32)
            .min_lod(0f32)
            .max_lod(0f32);

        match unsafe { device.create_sampler(&create_info, None) } {
            Ok(sampler) => Ok(sampler),
            Err(err) => Err(RtError::CreateSampler(err.into())),
        }
    }
}
