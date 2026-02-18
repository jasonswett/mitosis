#[derive(Clone, Copy)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Cell {
    fn bounding_box(&self) -> (usize, usize, usize, usize) {
        let x_min = ((self.x - self.radius).floor() as isize).max(0) as usize;
        let y_min = ((self.y - self.radius).floor() as isize).max(0) as usize;
        let x_max = (self.x + self.radius).ceil() as usize;
        let y_max = (self.y + self.radius).ceil() as usize;
        (x_min, y_min, x_max, y_max)
    }

    pub fn pixels(&self) -> Vec<(usize, usize, u32)> {
        let (x_min, y_min, x_max, y_max) = self.bounding_box();
        let radius_squared = self.radius * self.radius;
        let mut pixels = Vec::new();

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let distance_x = x as f32 - self.x;
                let distance_y = y as f32 - self.y;
                if distance_x * distance_x + distance_y * distance_y <= radius_squared {
                    pixels.push((x, y, 0x00_40_FF));
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
            let cell = Cell { x: 50.0, y: 30.0, radius: 10.0 };
            assert_eq!(cell.bounding_box(), (40, 20, 60, 40));
        }

        #[test]
        fn when_the_cell_is_near_the_top_left_edge_it_clamps_to_zero() {
            let cell = Cell { x: 5.0, y: 5.0, radius: 10.0 };
            let (x_min, y_min, _, _) = cell.bounding_box();
            assert_eq!(x_min, 0);
            assert_eq!(y_min, 0);
        }

        #[test]
        fn when_the_cell_is_near_the_bottom_right_edge_it_is_unclamped() {
            let cell = Cell { x: 95.0, y: 95.0, radius: 10.0 };
            let (_, _, x_max, y_max) = cell.bounding_box();
            assert_eq!(x_max, 105);
            assert_eq!(y_max, 105);
        }
    }

    mod pixels {
        use super::*;

        #[test]
        fn a_point_inside_the_circle_is_included() {
            let cell = Cell { x: 10.0, y: 30.0, radius: 10.0 };
            let pixels = cell.pixels();
            // (15, 33): distance_squared = 5*5 + 3*3 = 34 <= 100
            assert!(pixels.contains(&(15, 33, 0x00_40_FF)));
        }

        #[test]
        fn a_point_outside_the_circle_is_not_included() {
            let cell = Cell { x: 10.0, y: 30.0, radius: 10.0 };
            let pixels = cell.pixels();
            // (18, 37): distance_squared = 8*8 + 7*7 = 113 > 100
            assert!(!pixels.iter().any(|&(x, y, _)| x == 18 && y == 37));
        }

    }
}
