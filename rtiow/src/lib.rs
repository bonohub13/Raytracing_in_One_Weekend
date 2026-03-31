// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

pub(crate) mod instance;
pub(crate) mod state;
pub mod util;

pub use instance::Instance;
pub(crate) use instance::InstanceDesc;
pub use state::*;
use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum RtError {
    // Generic Rust operation error
    #[error("Failed to parse &str into u32. ({0})")]
    StrToU32Conv(Box<str>),
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
    #[error("Failed to create instance. ({0})")]
    CreateInstance(Box<dyn Error>),
}

pub type RtErr<T> = Result<T, RtError>;
