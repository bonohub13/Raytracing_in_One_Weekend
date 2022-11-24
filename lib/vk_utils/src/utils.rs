pub fn vk_to_string(raw_string_array: &[std::os::raw::c_char]) -> Result<String, String> {
    use std::ffi::CStr;

    let raw_string = {
        let pointer = raw_string_array.as_ptr();

        unsafe { CStr::from_ptr(pointer) }
    };

    match raw_string.to_str() {
        Ok(string) => Ok(string.to_owned()),
        Err(_) => Err(String::from("failed to convert raw c_char array to String")),
    }
}

pub struct SingleTimeCommands;

impl SingleTimeCommands {
    pub fn submit(
        device: &crate::Device,
        command_pool: &crate::CommandPool,
        action: impl FnOnce(ash::vk::CommandBuffer) -> Result<(), String>,
    ) -> Result<(), String> {
        use ash::vk;

        let command_buffer = crate::CommandBuffers::new(device, command_pool, 1)?;

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();

        device.begin_command_buffer(command_buffer.at(0), &begin_info);
        action(command_buffer.at(0))?;
        device.end_command_buffer(command_buffer.at(0));

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&[command_buffer.at(0)])
            .build();

        let graphics_queue = device.graphics_queue();

        device.queue_submit(graphics_queue, &[submit_info], vk::Fence::null())?;
        device.queue_wait_idle(graphics_queue)?;

        Ok(())
    }
}
