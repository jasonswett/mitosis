use crate::Cell;

const BACKGROUND_COLOR: u32 = 0x00_00_00;

pub struct WorldBuffer {
    pixels: Vec<u32>,
}

impl WorldBuffer {
    pub fn new(cells: &[Cell], width: usize, height: usize) -> Self {
        let mut pixels = vec![BACKGROUND_COLOR; width * height];

        for cell in cells {
            for (x, y, color) in cell.pixels() {
                if x < width && y < height {
                    pixels[y * width + x] = color;
                }
            }
        }

        WorldBuffer { pixels }
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_cells_are_present {
        use super::*;

        #[test]
        fn the_buffer_contains_visible_pixels() {
            let cells = vec![Cell { x: 50.0, y: 50.0, radius: 10.0 }];
            let world_buffer = WorldBuffer::new(&cells, 100, 100);
            assert!(world_buffer.pixels().iter().any(|&pixel| pixel != 0x00_00_00));
        }

        #[test]
        fn the_background_is_black() {
            let cells = vec![Cell { x: 50.0, y: 50.0, radius: 10.0 }];
            let world_buffer = WorldBuffer::new(&cells, 100, 100);
            assert_eq!(world_buffer.pixels()[0], 0x00_00_00);
        }
    }

    mod when_a_cell_extends_past_the_buffer_boundary {
        use super::*;

        #[test]
        fn out_of_bounds_pixels_are_clipped() {
            let cells = vec![Cell { x: 49.0, y: 49.0, radius: 5.0 }];
            let world_buffer = WorldBuffer::new(&cells, 50, 50);

            assert_eq!(world_buffer.pixels().len(), 50 * 50);
            assert_eq!(world_buffer.pixels()[49 * 50 + 49], 0x00_40_FF);
        }
    }
}
