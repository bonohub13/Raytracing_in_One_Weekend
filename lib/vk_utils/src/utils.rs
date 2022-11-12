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
