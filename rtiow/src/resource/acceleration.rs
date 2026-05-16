use crate::{Buffer, Device, VkState};
use ash::{khr::acceleration_structure, vk};
use std::{mem::ManuallyDrop, sync::Arc};

#[derive(Clone)]
pub struct AsLoader {
    raw: acceleration_structure::Device,
}

#[derive(Clone)]
pub struct AccelerationStructure<T>
where
    T: Sized + Clone,
{
    handle: vk::AccelerationStructureKHR,
    buffer: ManuallyDrop<Buffer<T>>,
    scratch_buffer: Option<Buffer<T>>,
    loader: Arc<AsLoader>,
}

impl<T> AccelerationStructure<T> where T: Sized + Clone {}

impl<T> Drop for AccelerationStructure<T>
where
    T: Sized + Clone,
{
    fn drop(&mut self) {
        unsafe {
            self.loader
                .raw
                .destroy_acceleration_structure(self.handle, None);
        }
    }
}

impl AsLoader {
    pub fn new(state: &VkState) -> Self {
        let raw =
            acceleration_structure::Device::new(state.instance.instance(), state.device.device());

        Self { raw }
    }
}
