pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(version: u32) -> Self {
        use ash::vk;

        Self {
            major: vk::api_version_major(version),
            minor: vk::api_version_minor(version),
            patch: vk::api_version_patch(version),
        }
    }

    pub fn nv_new(version: u32, vendor_id: u32) -> Self {
        use ash::vk;

        Self {
            major: vk::api_version_major(version) >> (if vendor_id == 0x10DE { 0 } else { 0 }),
            minor: vk::api_version_minor(version) >> (if vendor_id == 0x10DE { 2 } else { 0 }),
            patch: vk::api_version_patch(version) >> (if vendor_id == 0x10DE { 4 } else { 0 }),
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn check(result: ash::vk::Result, operation: &str) -> Result<(), String> {
    match result {
        ash::vk::Result::SUCCESS => Ok(()),
        _ => match result.result() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("failed to {} ({})", operation, err.to_string())),
        },
    }
}
