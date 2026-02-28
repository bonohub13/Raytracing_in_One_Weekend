// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum RtError {
    #[error("Failed to lock mutex: `{0}`")]
    MutexLock(String),
    #[error("Failed to query raw display handle: `{0}`")]
    DisplayHandle(Box<dyn Error>),
    #[error("Failed to query raw window handle: `{0}`")]
    WindowHandle(Box<dyn Error>),
    #[error("Failed to enumerate required extensions: `{0}`")]
    EnumerateRequiredExtensions(Box<dyn Error>),
    #[error("Debug Loader is not initialized")]
    DebugLoaderUninitialized,
    #[error("Failed to create debug messenger")]
    CreateDebugUtilsmessenger(Box<dyn Error>),
    #[error("Failed to create instance: `{0}`")]
    CreateInstance(Box<dyn Error>),
    #[error("Failed to enumerate instance extension properties: `{0}`")]
    EnumerateInstanceExtensionProperties(Box<dyn Error>),
    #[error("Failed to enumerate instance layer properties: `{0}`")]
    EnumerateInstanceLayerProperties(Box<dyn Error>),
    #[error("Failed to enumerate device extension properties: `{0}`")]
    EnumerateDeviceExtensionProperties(Box<dyn Error>),
    #[error("Validation layers are not supported")]
    RequiredValidationLayersNotSupported,
    #[error("Failed to enumerate physical devices: `{0}`")]
    EnumeratePhyicalDevices(Box<dyn Error>),
    #[error("Failed to create surface")]
    CreateSurface(Box<dyn Error>),
    #[error("Failed to get surface support for physical device: `{0}`")]
    GetPhysicalDeviceSurfaceSupport(Box<dyn Error>),
    #[error("Failed to get surface capabilities for physical device: `{0}`")]
    GetPhysicalDeviceSurfaceCapabilities(Box<dyn Error>),
    #[error("Failed to get surface formats for physical device: `{0}`")]
    GetPhysicalDeviceSurfaceFormats(Box<dyn Error>),
    #[error("Failed to get surface present modes for physical device: `{0}`")]
    GetPhysicalDeviceSurfacePresentModes(Box<dyn Error>),
    #[error("Failed to find any suitable GPU")]
    NoSuitableDevice,
    #[error("Failed to create logical device: `{0}`")]
    CreateDevice(Box<dyn Error>),
    #[error("Failed to wait logical device to idle state: `{0}`")]
    DeviceWaitIdle(Box<dyn Error>),
    #[error("Failed to create GPU memory allocator: `{0}`")]
    CreateAllocator(Box<dyn Error>),
    #[error("Failed to create allocation in GPU memory: `{0}`")]
    CreateAllocation(Box<dyn Error>),
    #[error("Allocated GPU memory has been freed")]
    NoAllocation,
    #[error("Failed to free allocation in GPU memory: `{0}`")]
    FreeAllocation(Box<dyn Error>),
    #[error("Failed to create swapchain: `{0}`")]
    CreateSwapchain(Box<dyn Error>),
    #[error("Failed to get swapchain images: `{0}`")]
    GetSwapchainImages(Box<dyn Error>),
    #[error("Failed to create image: `{0}`")]
    CreateImage(Box<dyn Error>),
    #[error("Failed to create image view: `{0}`")]
    CreateImageView(Box<dyn Error>),
    #[error("Failed to create sampler: `{0}`")]
    CreateSampler(Box<dyn Error>),
    #[error("Failed to load shader: `{0}`")]
    LoadShader(Box<dyn Error>),
    #[error("Failed to create shader module: `{0}`")]
    CreateShaderModule(Box<dyn Error>),
    #[error("Failed to create pipeline layout: `{0}`")]
    CreatePipelineLayout(Box<dyn Error>),
    #[error("Failed to create pipeline: `{0:?}`")]
    CreatePipeline(Option<Box<dyn Error>>),
    #[error("Failed to create command pool: `{0}`")]
    CreateCommandPool(Box<dyn Error>),
    #[error("Failed to create command buffer: `{0:?}`")]
    AllocateCommandBuffers(Option<Box<dyn Error>>),
    #[error("Failed to reset command buffer: `{0}`")]
    ResetCommandBuffer(Box<dyn Error>),
    #[error("Failed to begin command buffer: `{0}`")]
    BeginCommandBuffer(Box<dyn Error>),
    #[error("Failed to end command buffer: `{0}`")]
    EndCommandBuffer(Box<dyn Error>),
    #[error("Failed to create semaphore: `{0}`")]
    CreateSemaphore(Box<dyn Error>),
    #[error("Failed to create fence: `{0}`")]
    CreateFence(Box<dyn Error>),
    #[error("Failed to wait for fences: `{0}`")]
    WaitForFences(Box<dyn Error>),
    #[error("Failed to reset fences: `{0}`")]
    ResetFences(Box<dyn Error>),
    #[error("Failed to acquire next image: `{0}`")]
    AcquireNextImage(Box<dyn Error>),
    #[error("Failed to submit queue: `{0}`")]
    SubmitQueue(Box<dyn Error>),
    #[error("Failed to present queue: `{0}`")]
    QueuePresent(Box<dyn Error>),
    #[error("Failed to create descriptor set layout: `{0}`")]
    CreateDescriptorSetLayout(Box<dyn Error>),
    #[error("Failed to create descriptor pool: `{0}`")]
    CreateDescriptorPool(Box<dyn Error>),
    #[error("Failed to allocate descriptor sets: `{0}`")]
    AllocateDescriptorSets(Box<dyn Error>),
    #[error("Failed to create buffer: `{0}`")]
    CreateBuffer(Box<dyn Error>),
    #[error("Failed to bind image memory: `{0}`")]
    BindImageMemory(Box<dyn Error>),
    #[error("Failed to bind buffer memory: `{0}`")]
    BindBufferMemory(Box<dyn Error>),
    #[error("Failed to flush mapped memory ranges: `{0}`")]
    FlushMappedMemoryRanges(Box<dyn Error>),
    #[error("No data were passed for AS geometry")]
    NoGemoetryData,
}

pub type RtResult<T> = Result<T, RtError>;
