use minifb::{Key, Window, WindowOptions};
use mitosis::{Cell, StatsDisplay, WorldBuffer};
use std::time::Instant;

const FRAME_DURATION_MICROSECONDS: u64 = 16_667;
const FPS_DISPLAY_SCALE: usize = 4;

#[repr(C)]
struct CGSize {
    width: f64,
    height: f64,
}

#[repr(C)]
struct CGRect {
    _origin_x: f64,
    _origin_y: f64,
    size: CGSize,
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGMainDisplayID() -> u32;
    fn CGDisplayBounds(display: u32) -> CGRect;
}

fn main() {
    let (width, height) = unsafe {
        let display = CGMainDisplayID();
        let bounds = CGDisplayBounds(display);
        (bounds.size.width as usize, bounds.size.height as usize)
    };

    let mut window = Window::new(
        "Mitosis",
        width,
        height,
        WindowOptions {
            borderless: true,
            title: false,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(FRAME_DURATION_MICROSECONDS)));

    let cell = Cell { x: width as f32 / 2.0, y: height as f32 / 2.0, radius: 30.0 };
    let world_buffer = WorldBuffer::new(&[cell], width, height);
    let mut frame_pixels = world_buffer.pixels().to_vec();
    let mut stats_display = StatsDisplay::new(FPS_DISPLAY_SCALE, Instant::now());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        stats_display.tick(Instant::now());

        frame_pixels.copy_from_slice(world_buffer.pixels());
        for (x, y, color) in stats_display.pixels() {
            if x < width && y < height {
                frame_pixels[y * width + x] = color;
            }
        }

        window
            .update_with_buffer(&frame_pixels, width, height)
            .expect("Unable to update window");
    }
}
