// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

mod debug;
mod device;
mod error;
mod instance;
pub mod params;
mod renderer;
mod state;
mod surface;

use error::*;
pub use state::{State, StateDescriptor};

pub mod util {
    macro_rules! lock_mutex {
        ($mutex:expr) => {{
            $mutex
                .lock()
                .map_err(|err| RtError::MutexLock(err.to_string()))
        }};
    }

    macro_rules! lock_mutex_with_fallback {
        ($mutex:expr) => {{
            $mutex
                .lock()
                .map_err(|err| {
                    eprintln!("{}", RtError::MutexLock(err.to_string()));

                    err.into_inner()
                })
                .expect("Failed to get fallback poisened mutex")
        }};
    }

    pub(crate) use lock_mutex;
    pub(crate) use lock_mutex_with_fallback;
}
