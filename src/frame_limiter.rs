// Copyright 2026 Kensuke
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FrameLimiter {
    next_frame: Instant,
    fps_limit: Option<Duration>,
    fps_update: Instant,
    fps_update_limit: Duration,
}

impl FrameLimiter {
    pub fn new(fps: Option<u32>) -> Self {
        let frame_time = Instant::now();
        let fps_limit = fps.map(|limit| Duration::from_secs_f64(1f64 / limit as f64));

        Self {
            next_frame: frame_time,
            fps_limit,
            fps_update: frame_time,
            fps_update_limit: Duration::from_secs_f64(1f64),
        }
    }

    pub fn wait_frame(&mut self) {
        if let Some(fps_limit) = self.fps_limit {
            let frame_time = Instant::now();

            self.next_frame += fps_limit;
            if (self.fps_update - frame_time) > self.fps_update_limit {
                self.fps_update = frame_time;
            }

            std::thread::sleep(self.next_frame - frame_time);
        }
    }
}
