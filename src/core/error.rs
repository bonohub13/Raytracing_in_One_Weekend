use std::{
    error::Error,
    ffi::CString,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, thiserror::Error)]
pub enum RtError {
    DisplayHandle(winit::raw_window_handle::HandleError),
    WindowHandle(winit::raw_window_handle::HandleError),
    ReadFile(Box<dyn Error>),
    EnumerateInstanceLayer(Box<dyn Error>),
    ValidationLayerSupport(Vec<CString>),
    EnumerateRequiredExtensions(Option<Box<dyn Error>>),
    CreateInstance(Box<dyn Error>),
    EnumerateDeviceExtensions(Box<dyn Error>),
    #[allow(unused)]
    CreateDebugUtilsMessenger(Box<dyn Error>),
    CreateSurface(Box<dyn Error>),
    GetSurfaceSupport(Box<dyn Error>),
    GetSurfaceCapabilities(Box<dyn Error>),
    GetSurfaceFormats(Box<dyn Error>),
    GetSurfacePresentModes(Box<dyn Error>),
    EnumeratePhysicalDevices(Box<dyn Error>),
    NoSuitablePhysicalDevice,
    CreateDevice(Box<dyn Error>),
    DeviceWaitIdle(Box<dyn Error>),
    CreateSwapchain(Box<dyn Error>),
    GetSwapchainImages(Box<dyn Error>),
    CreateSwapchainImageViews(Box<dyn Error>),
    AcquiredNextImage(Box<dyn Error>),
    QueuePresent(Box<dyn Error>),
    CreateShaderModule(Box<dyn Error>),
    CreatePipelineLayout(Box<dyn Error>),
    CreateGraphicsPipeline(Option<Box<dyn Error>>),
    CreateComputePipeline(Option<Box<dyn Error>>),
    CreateRenderPass(Box<dyn Error>),
    CreateFramebuffer(Box<dyn Error>),
    CreateCommandPool(Box<dyn Error>),
    AllocateCommandBuffer(Option<Box<dyn Error>>),
    ResetCommandBuffer(Box<dyn Error>),
    BeginCommandBuffer(Box<dyn Error>),
    EndCommandBuffer(Box<dyn Error>),
    CreateSemaphore(Box<dyn Error>),
    CreateFence(Box<dyn Error>),
    WaitForFences(Box<dyn Error>),
    ResetFences(Box<dyn Error>),
    QueueSubmit(Box<dyn Error>),
    CreateBuffer(Box<dyn Error>),
    BindBufferMemory(Box<dyn Error>),
    BindImageMemory(Box<dyn Error>),
    CreateImage(Box<dyn Error>),
    CreateImageView(Box<dyn Error>),
    CreateSampler(Box<dyn Error>),
    CreateDescriptorSetLayout(Box<dyn Error>),
    CreateDescriptorPool(Box<dyn Error>),
    AllocateDescriptorSets(Option<Box<dyn Error>>),
    GpuAllocator(Box<dyn Error>),
}

pub type RtResult<T> = Result<T, RtError>;

impl Display for RtError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DisplayHandle(e) => write!(fmt, "Failed to get display handle ({e})"),
            Self::WindowHandle(e) => write!(fmt, "Failed to get window handle ({e})"),
            Self::ReadFile(e) => write!(fmt, "Failed to read file ({e})"),
            Self::EnumerateInstanceLayer(e) => {
                write!(fmt, "Failed to enumerate instance layer properties ({e})")
            }
            Self::EnumerateRequiredExtensions(e) => {
                if let Some(e) = e {
                    write!(fmt, "Failed to enumerate required extensions ({e})")
                } else {
                    write!(fmt, "Failed to enumerate required extensions")
                }
            }
            Self::ValidationLayerSupport(layers) => {
                write!(
                    fmt,
                    "Required validation layer is not supported ({layers:?})"
                )
            }
            Self::CreateInstance(e) => write!(fmt, "Failed to create Instance ({e})"),
            Self::EnumerateDeviceExtensions(e) => {
                write!(fmt, "Failed to enumerate device extension properties ({e})")
            }
            Self::CreateDebugUtilsMessenger(e) => {
                write!(fmt, "Failed to create DebugUtilsMessenger ({e})")
            }
            Self::CreateSurface(e) => write!(fmt, "Failed to create surface ({e})"),
            Self::GetSurfaceSupport(e) => {
                write!(fmt, "Failed to get physical device surface support ({e})")
            }
            Self::GetSurfaceCapabilities(e) => write!(
                fmt,
                "Failed to get physical device surface capabilities ({e})"
            ),
            Self::GetSurfaceFormats(e) => {
                write!(fmt, "Failed to get physical device surface formats ({e})")
            }
            Self::GetSurfacePresentModes(e) => {
                write!(
                    fmt,
                    "Failed to get physical device surface present modes ({e})"
                )
            }
            Self::EnumeratePhysicalDevices(e) => {
                write!(fmt, "Failed to enumerate physical devices ({e})")
            }
            Self::NoSuitablePhysicalDevice => {
                write!(fmt, "Failed to find any suitable physical device")
            }
            Self::CreateDevice(e) => write!(fmt, "Failed to create logical device ({e})"),
            Self::DeviceWaitIdle(e) => write!(fmt, "Device failed to wait idle ({e})"),
            Self::CreateSwapchain(e) => write!(fmt, "Failed to create swapchain ({e})"),
            Self::GetSwapchainImages(e) => write!(fmt, "Failed to get swapchain images ({e})"),
            Self::CreateSwapchainImageViews(e) => {
                write!(fmt, "Failed to create swapchain image views ({e})")
            }
            Self::AcquiredNextImage(e) => {
                write!(fmt, "Failed to acquire next image ({e})")
            }
            Self::QueuePresent(e) => {
                write!(fmt, "Failed to present queue ({e})")
            }
            Self::CreateShaderModule(e) => {
                write!(fmt, "Failed to create shader module ({e})")
            }
            Self::CreatePipelineLayout(e) => {
                write!(fmt, "Failed to create pipeline layout ({e})")
            }
            Self::CreateGraphicsPipeline(e) => {
                if let Some(err) = e {
                    write!(fmt, "Failed to create graphics pipeline(s) ({err})")
                } else {
                    write!(fmt, "Failed to create graphics pipeline")
                }
            }
            Self::CreateComputePipeline(e) => {
                if let Some(err) = e {
                    write!(fmt, "Failed to create compute pipeline(s) ({err})")
                } else {
                    write!(fmt, "Failed to create compute pipeline")
                }
            }
            Self::CreateRenderPass(e) => {
                write!(fmt, "Failed to create render pass ({e})")
            }
            Self::CreateFramebuffer(e) => {
                write!(fmt, "Failed to create framebuffer(s) ({e})")
            }
            Self::CreateCommandPool(e) => {
                write!(fmt, "Failed to create command pool ({e})")
            }
            Self::AllocateCommandBuffer(e) => {
                if let Some(err) = e {
                    write!(fmt, "Failed to allocate command buffer(s) ({err})")
                } else {
                    write!(fmt, "Failed to allocate command buffer(s)")
                }
            }
            Self::ResetCommandBuffer(e) => {
                write!(fmt, "Failed to reset command buffer ({e})")
            }
            Self::BeginCommandBuffer(e) => {
                write!(fmt, "Failed to begin command buffer ({e})")
            }
            Self::EndCommandBuffer(e) => {
                write!(fmt, "Failed to end command buffer ({e})")
            }
            Self::CreateSemaphore(e) => {
                write!(fmt, "Failed to create semaphore ({e})")
            }
            Self::CreateFence(e) => {
                write!(fmt, "Failed to create fence ({e})")
            }
            Self::WaitForFences(e) => {
                write!(fmt, "Failed to wait for fence(s) ({e})")
            }
            Self::ResetFences(e) => {
                write!(fmt, "Failed to reset fence(s) ({e})")
            }
            Self::QueueSubmit(e) => {
                write!(fmt, "Failed to submit queue ({e})")
            }
            Self::CreateBuffer(e) => {
                write!(fmt, "Failed to create buffer ({e})")
            }
            Self::BindBufferMemory(e) => {
                write!(fmt, "Failed to bind buffer to memory ({e})")
            }
            Self::BindImageMemory(e) => {
                write!(fmt, "Failed to bind image to memory ({e})")
            }
            Self::CreateImage(e) => {
                write!(fmt, "Failed to create image ({e})")
            }
            Self::CreateImageView(e) => {
                write!(fmt, "Failed to create image view ({e})")
            }
            Self::CreateSampler(e) => {
                write!(fmt, "Failed to create sampler ({e})")
            }
            Self::CreateDescriptorSetLayout(e) => {
                write!(fmt, "Failed to create descriptor set layout ({e})")
            }
            Self::CreateDescriptorPool(e) => {
                write!(fmt, "Failed to create descriptor pool ({e})")
            }
            Self::AllocateDescriptorSets(e) => {
                if let Some(err) = e.as_ref() {
                    write!(fmt, "Failed to allocate descriptor sets ({err})")
                } else {
                    write!(fmt, "Failed to allocate descriptor sets")
                }
            }
            Self::GpuAllocator(e) => {
                write!(fmt, "Error detected in GPU allocator ({e})")
            }
        }
    }
}
