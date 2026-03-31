// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{RtErr, RtError};
use ash::vk;

pub fn parse_version_from_str(version: &str) -> RtErr<u32> {
    let mut parts = version
        .split('.')
        .filter_map(|part| part.parse::<u32>().ok());

    if let Some(major) = parts.next() {
        let minor = parts.next().unwrap_or(0);
        let patch = parts.next().unwrap_or(0);

        Ok(vk::make_api_version(0, major, minor, patch))
    } else {
        Err(RtError::StrToU32Conv(version.into()))
    }
}
