use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use wgpu::{CreateSurfaceError, RequestDeviceError, SurfaceError};

#[derive(Debug, thiserror::Error)]
pub enum RtError {
    CreateSurface(CreateSurfaceError),
    RequestAdapter,
    RequestDevice(RequestDeviceError),
    GetCurrentTexture(SurfaceError),
    ReadFile(Box<dyn Error>),
}

pub type RtResult<T> = Result<T, RtError>;

impl Display for RtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RtError::CreateSurface(e) => {
                write!(f, "[ERROR]: Failed to create WGPU surface ({e})")
            }
            RtError::RequestAdapter => {
                write!(f, "[ERROR]: Failed to request WGPU adapter")
            }
            RtError::RequestDevice(e) => {
                write!(f, "[ERROR]: Failed to request WGPU device ({e})")
            }
            RtError::GetCurrentTexture(e) => {
                write!(f, "[ERROR]: Failed to get current texture ({e})")
            }
            RtError::ReadFile(e) => {
                write!(f, "[Error]: Failed to read file ({e})")
            }
        }
    }
}

unsafe impl Send for RtError {}
unsafe impl Sync for RtError {}
