pub fn vk_to_string(raw_string_array: &[std::os::raw::c_char]) -> Result<String, String> {
    use std::ffi::CStr;

    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();

        CStr::from_ptr(pointer)
    };

    let std_string = raw_string.to_str();

    if std_string.is_err() {
        Err(String::from("Failed to convert raw_char_array to String!"))
    } else {
        Ok(std_string.unwrap().to_owned())
    }
}

// Reads a shader code (.spv file) and retrieves data
//
// Param:
//  shader_path: Path to where the shader code (.spv file) is
//               If using relative path instead of absolute path, enter the
//               relative path to where the current directory is.
//               Not where the compiled binary is.
//
// Return:
//  Result<Vec<u8>, String>
//  Returns an error if the file doesn't exist
pub fn read_shader_code(shader_path: &std::path::Path) -> Result<Vec<u8>, String> {
    use std::{fs::File, io::Read};

    if !shader_path.to_str().unwrap().to_string().ends_with(".spv") {
        return Err(String::from("Invalid file! File must be a .spv file"));
    }

    let result = File::open(shader_path);

    match result {
        Err(_) => Err(format!("Failed to find spv file at {:?}", shader_path)),
        // spv file is compiled in the first place so any error in byte can be
        // ignored. (An error would've resulted in compile error in the shader
        // code compilation)
        Ok(spv_file) => Ok(spv_file.bytes().filter_map(|byte| byte.ok()).collect()),
    }
}

pub fn create_shader_module(
    device: &ash::Device,
    code: &Vec<u8>,
) -> Result<ash::vk::ShaderModule, String> {
    use ash::vk;

    let create_info = vk::ShaderModuleCreateInfo {
        code_size: code.len(),
        p_code: code.as_ptr() as *const u32,
        ..Default::default()
    };

    let result = unsafe { device.create_shader_module(&create_info, None) };

    match result {
        Ok(shader_module) => Ok(shader_module),
        Err(_) => Err(String::from("failed to create shader module!")),
    }
}
