use crate::TextDisplay;
use std::time::Instant;

const FPS_UPDATE_INTERVAL_MILLISECONDS: u128 = 200;

pub struct StatsDisplay {
    scale: usize,
    frame_count: usize,
    fps: usize,
    last_fps_update: Instant,
}

impl StatsDisplay {
    pub fn new(scale: usize, now: Instant) -> Self {
        StatsDisplay {
            scale,
            frame_count: 0,
            fps: 0,
            last_fps_update: now,
        }
    }

    pub fn tick(&mut self, now: Instant) {
        self.frame_count += 1;

        let elapsed = now.duration_since(self.last_fps_update);
        if elapsed.as_millis() >= FPS_UPDATE_INTERVAL_MILLISECONDS {
            self.fps = self.frame_count * 1000 / elapsed.as_millis() as usize;
            self.frame_count = 0;
            self.last_fps_update = now;
        }
    }

    pub fn pixels(&self) -> Vec<(usize, usize, u32)> {
        TextDisplay::new(&format!("FPS: {}", self.fps), self.scale).pixels()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    mod when_enough_time_has_elapsed {
        use super::*;

        #[test]
        fn the_pixels_reflect_the_calculated_fps() {
            let start = Instant::now();
            let mut stats_display = StatsDisplay::new(1, start);

            // 11 ticks over 200ms = 55 FPS
            for _ in 0..10 {
                stats_display.tick(start);
            }
            stats_display.tick(start + Duration::from_millis(200));

            assert_eq!(stats_display.pixels(), TextDisplay::new("FPS: 55", 1).pixels());
        }
    }

    mod when_not_enough_time_has_elapsed {
        use super::*;

        #[test]
        fn the_pixels_show_zero_fps() {
            let start = Instant::now();
            let mut stats_display = StatsDisplay::new(1, start);

            stats_display.tick(start + Duration::from_millis(100));

            assert_eq!(stats_display.pixels(), TextDisplay::new("FPS: 0", 1).pixels());
        }
    }
}
