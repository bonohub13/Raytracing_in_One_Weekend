// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

pub(crate) mod debug;
pub(crate) mod device;
pub(crate) mod instance;
mod pipeline;
pub(crate) mod state;
pub(crate) mod surface;
pub(crate) mod swapchain;
pub mod util;

pub(crate) use crate::{debug::*, device::*, instance::*, surface::*, swapchain::*};
pub use crate::{pipeline::*, state::*};
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
    #[error("Failed to get physical device surface capabilities. ({0})")]
    GetPhysicalDeviceSurfaceCapabilities(Box<dyn Error>),
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
    #[error("Failed to create swapchain. ({0})")]
    CreateSwapchain(Box<dyn Error>),
    #[error("Failed to acquire next image. ({0})")]
    AcquireNextImage(Box<dyn Error>),
    #[error("Failed to create images. ({0})")]
    CreateImages(Box<dyn Error>),
    #[error("Failed to create image view. ({0})")]
    CreateImageView(Box<dyn Error>),
    #[error("Failed to create shader module. ({0})")]
    CreateShaderModule(Box<dyn Error>),
    #[error("Failed to create pipeline layout. ({0})")]
    CreatePipelineLayout(Box<dyn Error>),
    #[error("Failed to create pipeline. ({0})")]
    CreatePipeline(Box<dyn Error>),
}

pub type RtErr<T> = Result<T, RtError>;
