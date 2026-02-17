pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
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
            cells: vec![cell],
        }
    }

    pub fn render(&self, buffer: &mut [u32]) {
        for pixel in buffer.iter_mut() {
            *pixel = 0x00_00_00;
        }

        for cell in &self.cells {
            let cx = cell.x;
            let cy = cell.y;
            let r = cell.radius;
            let r_sq = r * r;

            let y_min = ((cy - r).floor() as isize).max(0) as usize;
            let y_max = ((cy + r).ceil() as isize).min(self.height as isize - 1) as usize;
            let x_min = ((cx - r).floor() as isize).max(0) as usize;
            let x_max = ((cx + r).ceil() as isize).min(self.width as isize - 1) as usize;

            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    if dx * dx + dy * dy <= r_sq {
                        buffer[y * self.width + x] = 0x00_40_FF;
                    }
                }
            }
        }
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
        fn the_center_pixel_is_blue() {
            let world = World::new(800, 600);
            let mut buffer = vec![0u32; 800 * 600];
            world.render(&mut buffer);
            assert_eq!(buffer[300 * 800 + 400], 0x00_40_FF);
        }

        #[test]
        fn a_pixel_outside_the_circle_is_black() {
            let world = World::new(800, 600);
            let mut buffer = vec![0u32; 800 * 600];
            world.render(&mut buffer);
            assert_eq!(buffer[0], 0x00_00_00);
        }
    }
}
