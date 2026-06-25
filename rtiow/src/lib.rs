// Copyright 2026 Kensuke Saitolibrs
// SPDX-License-Identifier: MIT

pub(crate) mod debug;
pub(crate) mod device;
pub(crate) mod instance;
mod pipeline;
mod resource;
pub(crate) mod state;
pub(crate) mod surface;
pub(crate) mod swapchain;
pub mod util;

pub(crate) use crate::{debug::*, device::*, instance::*, surface::*};
pub use crate::{pipeline::*, resource::*, state::*, swapchain::*};
use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum RtError {
    // Generic Rust operation error
    #[error("Failed to parse &str into u32. ({0})")]
    StrToU32Conv(Box<str>),
    #[error("Failed to upgrade Weak pointer into Arc.")]
    ArcWeakUpgrade,
    #[error("Failed to read from file. ({0})")]
    ReadFile(Box<dyn Error>),
    #[error("Mutex was poisened by a panicked thread")]
    LockMutex,
    // Display Handle/Window Handle errors
    #[error("Failed to get display handle from window. ({0})")]
    DisplayHandle(Box<dyn Error>),
    #[error("Failed to get window handle from window. ({0})")]
    WindowHandle(Box<dyn Error>),
    #[error("Failed to enumerate required extensions. ({0})")]
    EnumerateRequiredExtensions(Box<dyn Error>),
    // Vulkan related errors
    #[error("Failed to enumerate instance extension properties. ({0})")]
    EnumerateInstanceExtensionProperties(Box<dyn Error>),
    #[error("Failed to enumerate instance layer properties. ({0})")]
    EnumerateInstanceLayerProperties(Box<dyn Error>),
    #[error("Validation layers requested, but not available")]
    ValidationLayersNotSupported,
    #[error("Extensions requested, but not available")]
    ExtensionNotSupported,
    #[error("Failed to create instance. ({0})")]
    CreateInstance(Box<dyn Error>),
    #[error("Debug loader isn't initialized.")]
    DebugLoaderUninitialized,
    #[error("Failed to create debug utils messenger. ({0})")]
    CreateDebugUtilsMessenger(Box<dyn Error>),
    #[error("Failed to create surface. ({0})")]
    CreateSurface(Box<dyn Error>),
    #[error("Failed to get physical device surface support. ({0})")]
    GetPhysicalDeviceSurfaceSupport(Box<dyn Error>),
    #[error("Failed to get physical device surface capabilities. ({0:?})")]
    GetPhysicalDeviceSurfaceCapabilities(Option<Box<dyn Error>>),
    #[error("Failed to get physical device surface formats. ({0})")]
    GetPhysicalDeviceSurfaceFormats(Box<dyn Error>),
    #[error("Failed to get physical device surface present modes. ({0})")]
    GetPhysicalDeviceSurfacePresentModes(Box<dyn Error>),
    #[error("Failed to enumerate physical devices. ({0})")]
    EnumeratePhysicalDevices(Box<dyn Error>),
    #[error("Failed to enumerate device extension properties. ({0})")]
    EnumerateDeviceExtensionProperties(Box<dyn Error>),
    #[error("Failed to find suitable physical devices.")]
    FindSuitableDevice,
    #[error("Failed to create logical device. ({0})")]
    CreateDevice(Box<dyn Error>),
    #[error("Device failed to wait idle. ({0})")]
    DeviceWaitIdle(Box<dyn Error>),
    #[error("Queue failed to wait idle. ({0})")]
    QueueWaitIdle(Box<dyn Error>),
    #[error("Failed to create swapchain. ({0})")]
    CreateSwapchain(Box<dyn Error>),
    #[error("Failed to acquire next image. ({0})")]
    AcquireNextImage(Box<dyn Error>),
    #[error("Failed to present queue. ({0})")]
    QueuePresent(Box<dyn Error>),
    #[error("Failed to initialize GPU allocator")]
    InitAllocator(Box<dyn Error>),
    #[error("Failed to allocate memory. ({0})")]
    AllocateMemory(Box<dyn Error>),
    #[error("Failed to free allocation. ({0})")]
    FreeAllocation(Box<dyn Error>),
    #[error("Allocation doesn't exist")]
    NoAllocation,
    #[error("Buffer is not visible to host")]
    BufferNonHostVisible,
    #[error("Failed to create images. ({0})")]
    CreateImages(Box<dyn Error>),
    #[error("Failed to bind image to memory. ({0})")]
    BindImageMemory(Box<dyn Error>),
    #[error("Failed to create image view. ({0})")]
    CreateImageView(Box<dyn Error>),
    #[error("Failed to create shader module. ({0})")]
    CreateBuffer(Box<dyn Error>),
    #[error("Failed to bind buffer to memory. ({0})")]
    BindBufferMemory(Box<dyn Error>),
    #[error("Failed to create buffer. ({0})")]
    CreateShaderModule(Box<dyn Error>),
    #[error("Failed to create descriptor set layout. ({0})")]
    CreateDescriptorSetLayout(Box<dyn Error>),
    #[error("Failed to create descriptor pool. ({0})")]
    CreateDescriptorPool(Box<dyn Error>),
    #[error("Failed to allocate descriptor sets. ({0})")]
    AllocateDescriptorSets(Box<dyn Error>),
    #[error("Failed to create pipeline layout. ({0})")]
    CreatePipelineLayout(Box<dyn Error>),
    #[error("Failed to create pipeline. ({0})")]
    CreatePipeline(Box<dyn Error>),
    #[error("Failed to create command pool. ({0})")]
    CreateCommandPool(Box<dyn Error>),
    #[error("Failed to allocate command buffers. ({0})")]
    AllocateCommandBuffers(Box<dyn Error>),
    #[error("Failed to begin command buffer. ({0})")]
    BeginCommandBuffer(Box<dyn Error>),
    #[error("Failed to end command buffer. ({0})")]
    EndCommandBuffer(Box<dyn Error>),
    #[error("Failed to reset command buffer. ({0})")]
    ResetCommandBuffer(Box<dyn Error>),
    #[error("Failed to create semaphore. ({0})")]
    CreateSemaphore(Box<dyn Error>),
    #[error("Failed to create fence. ({0})")]
    CreateFence(Box<dyn Error>),
    #[error("Failed to wait for fence(s). ({0})")]
    WaitForFences(Box<dyn Error>),
    #[error("Failed to reset fence(s). ({0})")]
    ResetFences(Box<dyn Error>),
    #[error("Failed to submit queue. ({0})")]
    QueueSubmit(Box<dyn Error>),
    #[error("Invalid buffer type")]
    InvalidBufferType,
    #[error("Failed to create acceleration structure. ({0})")]
    CreateAccelerationStructure(Box<dyn Error>),
    #[error("Failed to get ray tracing shader group handles. ({0})")]
    GetRayTracingShaderGroupHandles(Box<dyn Error>),
    #[error("Failed to create sampler. ({0})")]
    CreateSampler(Box<dyn Error>),
}

pub type RtErr<T> = Result<T, RtError>;
