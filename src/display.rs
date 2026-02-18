const TEXT_COLOR: u32 = 0xFFFFFF;
const GLYPH_WIDTH: usize = 3;
const GLYPH_HEIGHT: usize = 5;

const DIGIT_GLYPHS: [[u8; GLYPH_HEIGHT]; 10] = [
    [0b111, 0b101, 0b101, 0b101, 0b111], // 0
    [0b010, 0b110, 0b010, 0b010, 0b111], // 1
    [0b111, 0b001, 0b111, 0b100, 0b111], // 2
    [0b111, 0b001, 0b111, 0b001, 0b111], // 3
    [0b101, 0b101, 0b111, 0b001, 0b001], // 4
    [0b111, 0b100, 0b111, 0b001, 0b111], // 5
    [0b111, 0b100, 0b111, 0b101, 0b111], // 6
    [0b111, 0b001, 0b010, 0b010, 0b010], // 7
    [0b111, 0b101, 0b111, 0b101, 0b111], // 8
    [0b111, 0b101, 0b111, 0b001, 0b111], // 9
];

fn letter_glyph(ch: char) -> Option<[u8; GLYPH_HEIGHT]> {
    match ch {
        'F' => Some([0b111, 0b100, 0b111, 0b100, 0b100]),
        'P' => Some([0b111, 0b101, 0b111, 0b100, 0b100]),
        'S' => Some([0b111, 0b100, 0b111, 0b001, 0b111]),
        ':' => Some([0b000, 0b010, 0b000, 0b010, 0b000]),
        _ => None,
    }
}

fn glyph_for(ch: char) -> Option<[u8; GLYPH_HEIGHT]> {
    if let Some(digit) = ch.to_digit(10) {
        Some(DIGIT_GLYPHS[digit as usize])
    } else {
        letter_glyph(ch)
    }
}

fn text_pixels(text: &str, scale: usize) -> Vec<(usize, usize, u32)> {
    let mut pixels = Vec::new();
    let mut cursor_x = 0;

    for ch in text.chars() {
        if ch == ' ' {
            cursor_x += (GLYPH_WIDTH + 1) * scale;
            continue;
        }
        if let Some(glyph) = glyph_for(ch) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..GLYPH_WIDTH {
                    if bits & (1 << (GLYPH_WIDTH - 1 - col)) != 0 {
                        for sy in 0..scale {
                            for sx in 0..scale {
                                pixels.push((cursor_x + col * scale + sx, row * scale + sy, TEXT_COLOR));
                            }
                        }
                    }
                }
            }
            cursor_x += (GLYPH_WIDTH + 1) * scale;
        }
    }

    pixels
}

pub fn fps_pixels(fps: usize, scale: usize) -> Vec<(usize, usize, u32)> {
    text_pixels(&format!("FPS: {}", fps), scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_fps_is_displayed {
        use super::*;

        #[test]
        fn different_fps_values_produce_different_pixels() {
            let pixels_30 = fps_pixels(30, 1);
            let pixels_60 = fps_pixels(60, 1);
            assert_ne!(pixels_30, pixels_60);
        }

        #[test]
        fn it_includes_the_fps_label() {
            let pixels = fps_pixels(60, 1);
            // "F" glyph top row is 0b111 -> pixels at (0,0), (1,0), (2,0)
            assert!(pixels.contains(&(0, 0, 0xFFFFFF)));
            assert!(pixels.contains(&(1, 0, 0xFFFFFF)));
            assert!(pixels.contains(&(2, 0, 0xFFFFFF)));
        }

        #[test]
        fn scale_multiplies_pixel_positions() {
            let pixels_1x = text_pixels("0", 1);
            let pixels_2x = text_pixels("0", 2);
            assert!(pixels_2x.len() > pixels_1x.len());
        }
    }
}
