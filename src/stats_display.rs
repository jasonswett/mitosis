use crate::display;
use std::time::Instant;

const FPS_UPDATE_INTERVAL_MILLISECONDS: u128 = 200;

pub struct StatsDisplay {
    scale: usize,
    frame_count: usize,
    fps: usize,
    last_fps_update: Instant,
}

impl StatsDisplay {
    pub fn new(scale: usize) -> Self {
        StatsDisplay {
            scale,
            frame_count: 0,
            fps: 0,
            last_fps_update: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;

        let elapsed = self.last_fps_update.elapsed();
        if elapsed.as_millis() >= FPS_UPDATE_INTERVAL_MILLISECONDS {
            self.fps = self.frame_count * 1000 / elapsed.as_millis() as usize;
            self.frame_count = 0;
            self.last_fps_update = Instant::now();
        }
    }

    pub fn pixels(&self) -> Vec<(usize, usize, u32)> {
        display::fps_pixels(self.fps, self.scale)
    }
}
