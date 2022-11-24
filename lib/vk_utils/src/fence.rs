pub struct Fence {
    pub fence: ash::vk::Fence,
}

impl Fence {
    pub fn new(device: &crate::Device, signaled: bool) -> Result<Self, String> {
        use ash::vk;

        let fence_info = vk::FenceCreateInfo::builder()
            .flags(if signaled {
                vk::FenceCreateFlags::SIGNALED
            } else {
                vk::FenceCreateFlags::empty()
            })
            .build();

        let fence = device.create_fence(&fence_info, None)?;

        Ok(Self { fence })
    }

    pub fn reset(&self, device: &crate::Device) -> Result<(), String> {
        device.reset_fences(&[self.fence])
    }

    pub fn wait(&self, device: &crate::Device, timeout: u64) -> Result<(), String> {
        device.wait_for_fences(&[self.fence], true, timeout)
    }

    pub fn cleanup(device: &crate::Device, fence: &mut Self) {
        log::info!("performing cleanup for Fence");

        device.destroy_fence(fence.fence, None);
    }
}
