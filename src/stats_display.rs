use crate::TextDisplay;
use std::time::Instant;

const FPS_UPDATE_INTERVAL_MILLISECONDS: u128 = 200;
const GLYPH_HEIGHT: usize = 5;
const LINE_SPACING: usize = 1;

pub struct StatsDisplay {
    scale: usize,
    starting_energy: u32,
    frame_count: usize,
    fps: usize,
    last_fps_update: Instant,
}

impl StatsDisplay {
    pub fn new(scale: usize, starting_energy: u32, now: Instant) -> Self {
        StatsDisplay {
            scale,
            starting_energy,
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

    pub fn fps(&self) -> usize {
        self.fps
    }

    pub fn pixels(&self, population: usize, total_energy: u64) -> Vec<(usize, usize, u32)> {
        let line_height = (GLYPH_HEIGHT + LINE_SPACING) * self.scale;
        let lines = [
            format!("FPS: {}", self.fps),
            format!("POP: {}", population),
            format!("E: {}/{}", total_energy, self.starting_energy),
        ];

        let mut pixels = Vec::new();
        for (line_index, text) in lines.iter().enumerate() {
            let y_offset = line_index * line_height;
            for (x, y, color) in TextDisplay::new(text, self.scale).pixels() {
                pixels.push((x, y + y_offset, color));
            }
        }

        pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    mod when_enough_time_has_elapsed {
        use super::*;

        #[test]
        fn the_fps_line_reflects_the_calculated_fps() {
            let start = Instant::now();
            let mut stats_display = StatsDisplay::new(1, 1000, start);

            for _ in 0..10 {
                stats_display.tick(start);
            }
            stats_display.tick(start + Duration::from_millis(200));

            let pixels = stats_display.pixels(1, 1000);
            let fps_pixels = TextDisplay::new("FPS: 55", 1).pixels();
            for pixel in &fps_pixels {
                assert!(pixels.contains(pixel));
            }
        }

        #[test]
        fn fps_returns_the_calculated_value() {
            let start = Instant::now();
            let mut stats_display = StatsDisplay::new(1, 1000, start);

            for _ in 0..10 {
                stats_display.tick(start);
            }
            stats_display.tick(start + Duration::from_millis(200));

            assert_eq!(stats_display.fps(), 55);
        }
    }

    mod when_not_enough_time_has_elapsed {
        use super::*;

        #[test]
        fn the_fps_line_shows_zero() {
            let start = Instant::now();
            let mut stats_display = StatsDisplay::new(1, 1000, start);

            stats_display.tick(start + Duration::from_millis(100));

            let pixels = stats_display.pixels(1, 1000);
            let fps_pixels = TextDisplay::new("FPS: 0", 1).pixels();
            for pixel in &fps_pixels {
                assert!(pixels.contains(pixel));
            }
        }
    }

    mod population_and_energy {
        use super::*;

        #[test]
        fn it_displays_the_population() {
            let start = Instant::now();
            let stats_display = StatsDisplay::new(1, 1000, start);
            let pixels = stats_display.pixels(42, 5000);

            let line_height = (GLYPH_HEIGHT + LINE_SPACING) * 1;
            let pop_pixels: Vec<(usize, usize, u32)> = TextDisplay::new("POP: 42", 1)
                .pixels()
                .iter()
                .map(|&(x, y, color)| (x, y + line_height, color))
                .collect();

            for pixel in &pop_pixels {
                assert!(pixels.contains(pixel));
            }
        }

        #[test]
        fn it_offsets_lines_by_scale() {
            let start = Instant::now();
            let stats_display = StatsDisplay::new(2, 1000, start);
            let pixels = stats_display.pixels(5, 2000);

            let line_height = (GLYPH_HEIGHT + LINE_SPACING) * 2;
            let pop_pixels: Vec<(usize, usize, u32)> = TextDisplay::new("POP: 5", 2)
                .pixels()
                .iter()
                .map(|&(x, y, color)| (x, y + line_height, color))
                .collect();

            for pixel in &pop_pixels {
                assert!(pixels.contains(pixel), "missing pixel {:?}", pixel);
            }
        }

        #[test]
        fn it_displays_total_and_starting_energy() {
            let start = Instant::now();
            let stats_display = StatsDisplay::new(1, 1000, start);
            let pixels = stats_display.pixels(1, 5000);

            let line_height = (GLYPH_HEIGHT + LINE_SPACING) * 1;
            let energy_pixels: Vec<(usize, usize, u32)> = TextDisplay::new("E: 5000/1000", 1)
                .pixels()
                .iter()
                .map(|&(x, y, color)| (x, y + 2 * line_height, color))
                .collect();

            for pixel in &energy_pixels {
                assert!(pixels.contains(pixel));
            }
        }
    }
}
