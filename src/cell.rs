const CELL_COLOR: u32 = 0x00_40_FF;

#[derive(Clone)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub energy: u32,
}

impl Cell {
    fn bounding_box(&self) -> (usize, usize, usize, usize) {
        let x_min = ((self.x - self.radius).floor() as isize).max(0) as usize;
        let y_min = ((self.y - self.radius).floor() as isize).max(0) as usize;
        let x_max = (self.x + self.radius).ceil() as usize;
        let y_max = (self.y + self.radius).ceil() as usize;
        (x_min, y_min, x_max, y_max)
    }

    pub fn draw(&self, buffer: &mut [u32], width: usize, height: usize) {
        let (x_min, y_min, x_max, y_max) = self.bounding_box();
        let radius_squared = self.radius * self.radius;

        for y in y_min..=y_max.min(height - 1) {
            for x in x_min..=x_max.min(width - 1) {
                let distance_x = x as f32 - self.x;
                let distance_y = y as f32 - self.y;
                if distance_x * distance_x + distance_y * distance_y <= radius_squared {
                    buffer[y * width + x] = CELL_COLOR;
                }
            }
        }
    }

    pub fn pixels(&self) -> Vec<(usize, usize, u32)> {
        let (_, _, x_max, y_max) = self.bounding_box();
        let width = x_max + 1;
        let height = y_max + 1;
        let mut buffer = vec![0u32; width * height];
        self.draw(&mut buffer, width, height);

        let (x_min, y_min, _, _) = self.bounding_box();
        let mut pixels = Vec::new();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if buffer[y * width + x] != 0 {
                    pixels.push((x, y, buffer[y * width + x]));
                }
            }
        }
        pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bounding_box {
        use super::*;

        #[test]
        fn it_spans_from_center_minus_radius_to_center_plus_radius() {
            let cell = Cell { x: 50.0, y: 30.0, radius: 10.0, energy: 100 };
            assert_eq!(cell.bounding_box(), (40, 20, 60, 40));
        }

        #[test]
        fn when_the_cell_is_near_the_top_left_edge_it_clamps_to_zero() {
            let cell = Cell { x: 5.0, y: 5.0, radius: 10.0, energy: 100 };
            let (x_min, y_min, _, _) = cell.bounding_box();
            assert_eq!(x_min, 0);
            assert_eq!(y_min, 0);
        }

        #[test]
        fn when_the_cell_is_near_the_bottom_right_edge_it_is_unclamped() {
            let cell = Cell { x: 95.0, y: 95.0, radius: 10.0, energy: 100 };
            let (_, _, x_max, y_max) = cell.bounding_box();
            assert_eq!(x_max, 105);
            assert_eq!(y_max, 105);
        }
    }

    mod pixels {
        use super::*;

        #[test]
        fn a_point_inside_the_circle_is_included() {
            let cell = Cell { x: 10.0, y: 30.0, radius: 10.0, energy: 100 };
            let pixels = cell.pixels();
            // (15, 33): distance_squared = 5*5 + 3*3 = 34 <= 100
            assert!(pixels.contains(&(15, 33, 0x00_40_FF)));
        }

        #[test]
        fn a_point_outside_the_circle_is_not_included() {
            let cell = Cell { x: 10.0, y: 30.0, radius: 10.0, energy: 100 };
            let pixels = cell.pixels();
            // (18, 37): distance_squared = 8*8 + 7*7 = 113 > 100
            assert!(!pixels.iter().any(|&(x, y, _)| x == 18 && y == 37));
        }

        #[test]
        fn a_cell_near_the_origin_produces_correct_pixels() {
            let cell = Cell { x: 5.0, y: 2.0, radius: 3.0, energy: 100 };
            let pixels = cell.pixels();
            assert!(pixels.contains(&(5, 2, 0x00_40_FF)));
        }
    }
}
