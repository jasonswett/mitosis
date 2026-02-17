use crate::Cell;

pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    buffer: Vec<u32>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let cell = Cell {
            x: width as f32 / 2.0,
            y: height as f32 / 2.0,
            radius: 30.0,
        };
        World {
            width,
            height,
            buffer: Self::rendered_buffer(&[cell], width, height),
            cells: vec![cell],
        }
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    fn rendered_buffer(cells: &[Cell], width: usize, height: usize) -> Vec<u32> {
        let mut buffer = vec![0x00_00_00u32; width * height];

        for cell in cells {
            for (x, y, color) in cell.pixels() {
                if x < width && y < height {
                    buffer[y * width + x] = color;
                }
            }
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_a_world_is_created {
        use super::*;

        #[test]
        fn it_has_one_cell() {
            let world = World::new(800, 600);
            assert_eq!(world.cells.len(), 1);
        }

        #[test]
        fn the_cell_is_centered() {
            let world = World::new(800, 600);
            let cell = &world.cells[0];
            assert_eq!(cell.x, 400.0);
            assert_eq!(cell.y, 300.0);
        }
    }

    mod when_the_world_renders {
        use super::*;

        #[test]
        fn the_cell_is_visible() {
            let world = World::new(800, 600);
            assert_eq!(world.buffer()[300 * 800 + 400], 0x00_40_FF);
        }

        #[test]
        fn the_background_is_black() {
            let world = World::new(800, 600);
            assert_eq!(world.buffer()[0], 0x00_00_00);
        }
    }

    mod when_a_cell_extends_past_the_buffer_boundary {
        use super::*;

        #[test]
        fn out_of_bounds_pixels_are_clipped() {
            let cells = vec![Cell { x: 49.0, y: 49.0, radius: 5.0 }];
            let buffer = World::rendered_buffer(&cells, 50, 50);

            assert_eq!(buffer.len(), 50 * 50);
            assert_eq!(buffer[49 * 50 + 49], 0x00_40_FF);
        }
    }
}
