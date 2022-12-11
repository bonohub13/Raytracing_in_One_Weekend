#[repr(C)]
#[derive(Copy, Clone)]
pub struct UniformBufferObject {
    pub image_width: f32,
    pub image_height: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub focal_length: f32,
}
