pub struct DebugUtils {
    vk_set_debug_utils_object_name_ext: ash::vk::PFN_vkSetDebugUtilsObjectNameEXT,
    device: ash::vk::Device,
}

impl DebugUtils {
    pub fn new(entry: &ash::Entry, instance: ash::vk::Instance) -> Result<Self, String> {
        use std::mem::transmute;

        let vk_set_debug_utils_object_name_ext = unsafe {
            entry.get_instance_proc_addr(
                instance,
                "vkSetDebugUtilsObjectNameEXT".as_ptr() as *const i8,
            )
        };

        match vk_set_debug_utils_object_name_ext {
            Some(vk_set_debug_utils_obj_name_ext) => Ok(Self {
                vk_set_debug_utils_object_name_ext: unsafe {
                    transmute(vk_set_debug_utils_obj_name_ext)
                },
                device: ash::vk::Device::null(),
            }),
            None => Err(String::from(
                "failed to get address of 'vkSetDebugUtilsObjectNameEXT'",
            )),
        }
    }

    pub fn set_device(&mut self, device: ash::vk::Device) {
        self.device = device;
    }

    #[inline]
    fn vk_set_debug_utils_object_name_ext(
        &self,
        device: ash::vk::Device,
        info: &ash::vk::DebugUtilsObjectNameInfoEXT,
    ) -> ash::vk::Result {
        unsafe { (self.vk_set_debug_utils_object_name_ext)(device, info) }
    }

    #[inline]
    fn set_object_name<T>(
        &self,
        object: &T,
        name: &str,
        type_: ash::vk::ObjectType,
    ) -> Result<(), String> {
        use ash::vk;
        use std::ffi::CStr;
        use std::mem::transmute;

        let object_name = unsafe { CStr::from_bytes_with_nul_unchecked(name.as_bytes()) };
        let info = vk::DebugUtilsObjectNameInfoEXT::builder()
            .object_handle(unsafe { transmute(object) })
            .object_type(type_)
            .object_name(object_name)
            .build();

        let result = crate::check(
            self.vk_set_debug_utils_object_name_ext(self.device, &info),
            "set object name",
        );

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[macro_export]
macro_rules! set_object_name {
    ($object:expr, $name:expr) => {
        /* ash::vk::ObjectType (Members are all CAPS!)
         *  ACCELERATION_STRUCTURE_KHR
         *  BUFFER
         *  COMMAND_BUFFER
         *  DESCRIPTOR_SET
         *  DESCRIPTOR_SET_LAYOUT
         *  DEVICE_MEMORY
         *  FRAMEBUFFER
         *  IMAGE
         *  IMAGE_VIEW
         *  PIPELINE
         *  QUEUE
         *  RENDER_PASS
         *  SEMAPHORE
         *  SHADER_MODULE
         *  SWAPCHAIN_KHR
         */
        if let Some(_) =
            (&$object as &std::any::Any).downcast_ref::<ash::vk::AccelerationStructureKHR>()
        {
            DebugUtils::set_object_name(
                $object,
                $name,
                ash::vk::ObjectType::ACCELERATION_STRUCTURE_KHR,
            )
        } else if let Some(_) = (&$object as &std::any::Any).downcast_ref::<ash::vk::Buffer>() {
            DebugUtils::set_object_name($object, $name, ash::vk::ObjectType::BUFFER)
        }
    };
}
