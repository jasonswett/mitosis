pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Cell {
    pub fn bounding_box(&self, buffer_width: usize, buffer_height: usize) -> (usize, usize, usize, usize) {
        let y_min = ((self.y - self.radius).floor() as isize).max(0) as usize;
        let y_max = ((self.y + self.radius).ceil() as isize).min(buffer_height as isize - 1) as usize;
        let x_min = ((self.x - self.radius).floor() as isize).max(0) as usize;
        let x_max = ((self.x + self.radius).ceil() as isize).min(buffer_width as isize - 1) as usize;
        (x_min, y_min, x_max, y_max)
    }

    pub fn draw(&self, buffer: &mut [u32], buffer_width: usize, buffer_height: usize) {
        let (x_min, y_min, x_max, y_max) = self.bounding_box(buffer_width, buffer_height);
        let radius_squared = self.radius * self.radius;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let distance_x = x as f32 - self.x;
                let distance_y = y as f32 - self.y;
                if distance_x * distance_x + distance_y * distance_y <= radius_squared {
                    buffer[y * buffer_width + x] = 0x00_40_FF;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bounding_box {
        use super::*;

        #[test]
        fn it_returns_the_bounding_box_of_the_cell() {
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0 };
            assert_eq!(cell.bounding_box(100, 100), (40, 40, 60, 60));
        }

        #[test]
        fn it_clamps_to_the_screen_edges() {
            let cell = Cell { x: 5.0, y: 5.0, radius: 10.0 };
            let (x_min, y_min, _, _) = cell.bounding_box(100, 100);
            assert_eq!(x_min, 0);
            assert_eq!(y_min, 0);
        }

        #[test]
        fn it_clamps_to_the_bottom_right_edge() {
            let cell = Cell { x: 95.0, y: 95.0, radius: 10.0 };
            let (_, _, x_max, y_max) = cell.bounding_box(100, 100);
            assert_eq!(x_max, 99);
            assert_eq!(y_max, 99);
        }
    }

    mod draw {
        use super::*;

        #[test]
        fn it_colors_the_center_pixel_blue() {
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0 };
            let mut buffer = vec![0u32; 100 * 100];
            cell.draw(&mut buffer, 100, 100);
            assert_eq!(buffer[50 * 100 + 50], 0x00_40_FF);
        }

        #[test]
        fn it_does_not_color_a_pixel_outside_the_circle() {
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0 };
            let mut buffer = vec![0u32; 100 * 100];
            cell.draw(&mut buffer, 100, 100);
            assert_eq!(buffer[0], 0x00_00_00);
        }

        #[test]
        fn it_colors_a_pixel_just_inside_the_edge() {
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0 };
            let mut buffer = vec![0u32; 100 * 100];
            cell.draw(&mut buffer, 100, 100);
            assert_eq!(buffer[50 * 100 + 59], 0x00_40_FF);
        }

        #[test]
        fn it_does_not_color_a_pixel_just_outside_the_edge() {
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0 };
            let mut buffer = vec![0u32; 100 * 100];
            cell.draw(&mut buffer, 100, 100);
            assert_eq!(buffer[39 * 100 + 39], 0x00_00_00);
        }
    }
}
