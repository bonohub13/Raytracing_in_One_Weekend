pub(crate) unsafe fn data_into_bytes<T>(data: &[T]) -> &[u8]
where
    T: Sized,
{
    let p_data = &data[0] as *const T;

    unsafe { std::slice::from_raw_parts(p_data as *const u8, size_of_val(data)) }
}
