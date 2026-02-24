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
