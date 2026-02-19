use crate::Cell;
use crate::energy_ball::EnergyBall;

const BACKGROUND_COLOR: u32 = 0x00_00_00;

pub struct WorldBuffer {
    pixels: Vec<u32>,
}

impl WorldBuffer {
    pub fn new(cells: &[Cell], energy_balls: &[EnergyBall], width: usize, height: usize) -> Self {
        let mut pixels = vec![BACKGROUND_COLOR; width * height];
        Self::draw_into(cells, energy_balls, &mut pixels, width, height);
        WorldBuffer { pixels }
    }

    pub fn draw_into(cells: &[Cell], energy_balls: &[EnergyBall], buffer: &mut [u32], width: usize, height: usize) {
        buffer.fill(BACKGROUND_COLOR);

        for ball in energy_balls {
            ball.draw(buffer, width, height);
        }

        for cell in cells {
            cell.draw(buffer, width, height);
        }
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
            let cells = vec![Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 }];
            let world_buffer = WorldBuffer::new(&cells, &[], 100, 100);
            assert!(world_buffer.pixels().iter().any(|&pixel| pixel != 0x00_00_00));
        }

        #[test]
        fn the_background_is_black() {
            let cells = vec![Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 }];
            let world_buffer = WorldBuffer::new(&cells, &[], 100, 100);
            assert_eq!(world_buffer.pixels()[0], 0x00_00_00);
        }
    }

    mod when_a_cell_extends_past_the_buffer_boundary {
        use super::*;

        #[test]
        fn out_of_bounds_pixels_are_clipped() {
            let cells = vec![Cell { x: 49.0, y: 49.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 }];
            let world_buffer = WorldBuffer::new(&cells, &[], 50, 50);

            assert_eq!(world_buffer.pixels().len(), 50 * 50);
            assert_eq!(world_buffer.pixels()[49 * 50 + 49], 0x00_40_FF);
        }
    }

    mod when_an_energy_ball_extends_past_the_buffer_boundary {
        use super::*;
        use crate::energy_ball::EnergyBall;

        #[test]
        fn out_of_bounds_pixels_are_clipped() {
            let balls = vec![EnergyBall { x: 49.0, y: 49.0 }];
            let world_buffer = WorldBuffer::new(&[], &balls, 50, 50);

            assert_eq!(world_buffer.pixels().len(), 50 * 50);
            assert_eq!(world_buffer.pixels()[49 * 50 + 49], 0x00_FF_00);
        }
    }
}
