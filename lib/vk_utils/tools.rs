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
