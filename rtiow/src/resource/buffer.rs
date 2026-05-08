// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::Device;
use ash::vk;
use std::{marker::PhantomData, sync::Arc};

pub struct Buffer<T>
where
    T: Sized,
{
    buffers: Vec<vk::Buffer>,
    device: Arc<Device>,
    _data: PhantomData<T>,
}

impl<T> Buffer<T>
where
    T: Sized,
{
    #[inline]
    pub fn buffers(&self) -> &[vk::Buffer] {
        &self.buffers
    }

    #[inline]
    pub fn size(&self) -> usize {
        size_of::<T>()
    }
}

impl<T> Drop for Buffer<T>
where
    T: Sized,
{
    fn drop(&mut self) {
        let device = self.device.device();

        self.buffers
            .iter()
            .copied()
            .for_each(|buffer| unsafe { device.destroy_buffer(buffer, None) });
    }
}
