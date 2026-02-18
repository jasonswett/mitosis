use minifb::{Key, Window, WindowOptions};
use mitosis::{display, World};
use std::time::Instant;

const FRAME_DURATION_MICROSECONDS: u64 = 16_667;
const FPS_UPDATE_INTERVAL_MILLISECONDS: u128 = 200;
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

    let world = World::new(width, height);
    let mut buffer = world.buffer().to_vec();
    let mut frame_count: usize = 0;
    let mut last_fps_update = Instant::now();
    let mut fps: usize = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        frame_count += 1;

        let elapsed = last_fps_update.elapsed();
        if elapsed.as_millis() >= FPS_UPDATE_INTERVAL_MILLISECONDS {
            fps = frame_count * 1000 / elapsed.as_millis() as usize;
            frame_count = 0;
            last_fps_update = Instant::now();
        }

        buffer.copy_from_slice(world.buffer());
        for (x, y, color) in display::fps_pixels(fps, FPS_DISPLAY_SCALE) {
            if x < width && y < height {
                buffer[y * width + x] = color;
            }
        }

        window
            .update_with_buffer(&buffer, width, height)
            .expect("Unable to update window");
    }
}
