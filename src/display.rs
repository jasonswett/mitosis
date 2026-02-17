const DIGIT_GLYPHS: [[u8; 5]; 10] = [
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

pub fn fps_pixels(fps: usize) -> Vec<(usize, usize, u32)> {
    let text = fps.to_string();
    let mut pixels = Vec::new();
    let mut cursor_x = 0;

    for ch in text.chars() {
        if let Some(digit) = ch.to_digit(10) {
            let glyph = &DIGIT_GLYPHS[digit as usize];
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..3 {
                    if bits & (1 << (2 - col)) != 0 {
                        pixels.push((cursor_x + col, row, 0xFFFFFF));
                    }
                }
            }
            cursor_x += 4;
        }
    }

    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_fps_is_displayed {
        use super::*;

        #[test]
        fn the_fps_value_appears_as_non_black_pixels() {
            let pixels = fps_pixels(60);
            assert!(!pixels.is_empty());
        }

        #[test]
        fn different_fps_values_produce_different_pixels() {
            let pixels_30 = fps_pixels(30);
            let pixels_60 = fps_pixels(60);
            assert_ne!(pixels_30, pixels_60);
        }

        #[test]
        fn the_digit_1_renders_at_the_correct_positions() {
            let pixels = fps_pixels(1);
            // Glyph for "1": .#. / ##. / .#. / .#. / ###
            assert!(pixels.contains(&(1, 0, 0xFFFFFF)));
            assert!(pixels.contains(&(0, 1, 0xFFFFFF)));
            assert!(pixels.contains(&(1, 1, 0xFFFFFF)));
            assert!(pixels.contains(&(1, 2, 0xFFFFFF)));
            assert!(pixels.contains(&(1, 3, 0xFFFFFF)));
            assert!(pixels.contains(&(0, 4, 0xFFFFFF)));
            assert!(pixels.contains(&(1, 4, 0xFFFFFF)));
            assert!(pixels.contains(&(2, 4, 0xFFFFFF)));
            assert_eq!(pixels.len(), 8);
        }

        #[test]
        fn the_second_digit_is_offset_by_four_pixels() {
            let pixels = fps_pixels(11);
            // Second "1" starts at x=4
            assert!(pixels.contains(&(5, 0, 0xFFFFFF)));
            assert!(pixels.contains(&(4, 1, 0xFFFFFF)));
        }
    }
}
