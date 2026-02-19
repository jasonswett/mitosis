pub const ENERGY_BALL_RADIUS: f32 = 3.0;
const ENERGY_BALL_COLOR: u32 = 0x00_FF_00;

pub struct EnergyBall {
    pub x: f32,
    pub y: f32,
}

impl EnergyBall {
    pub fn pixels(&self) -> Vec<(usize, usize, u32)> {
        let x_min = ((self.x - ENERGY_BALL_RADIUS).floor() as isize).max(0) as usize;
        let y_min = ((self.y - ENERGY_BALL_RADIUS).floor() as isize).max(0) as usize;
        let x_max = (self.x + ENERGY_BALL_RADIUS).ceil() as usize;
        let y_max = (self.y + ENERGY_BALL_RADIUS).ceil() as usize;
        let radius_squared = ENERGY_BALL_RADIUS * ENERGY_BALL_RADIUS;
        let mut pixels = Vec::new();

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let distance_x = x as f32 - self.x;
                let distance_y = y as f32 - self.y;
                if distance_x * distance_x + distance_y * distance_y <= radius_squared {
                    pixels.push((x, y, ENERGY_BALL_COLOR));
                }
            }
        }

        pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod pixels {
        use super::*;

        #[test]
        fn it_produces_green_pixels_at_the_ball_position() {
            let ball = EnergyBall { x: 10.0, y: 10.0 };
            let pixels = ball.pixels();
            assert!(pixels.contains(&(10, 10, 0x00_FF_00)));
        }

        #[test]
        fn a_point_within_radius_is_included() {
            let ball = EnergyBall { x: 10.0, y: 10.0 };
            let pixels = ball.pixels();
            // (12, 10): distance = 2.0 <= 3.0
            assert!(pixels.contains(&(12, 10, 0x00_FF_00)));
        }

        #[test]
        fn a_point_outside_radius_is_not_included() {
            let ball = EnergyBall { x: 10.0, y: 10.0 };
            let pixels = ball.pixels();
            // (14, 10): distance = 4.0 > 3.0
            assert!(!pixels.iter().any(|&(x, y, _)| x == 14 && y == 10));
        }

        #[test]
        fn the_pixel_count_for_a_radius_three_circle_is_29() {
            let ball = EnergyBall { x: 10.0, y: 10.0 };
            assert_eq!(ball.pixels().len(), 29);
        }

        #[test]
        fn boundary_pixel_at_x_min_is_included() {
            // Ball at (3, 3): x_min should be 0. Pixel (0, 3) is at distance 3.0 = radius.
            let ball = EnergyBall { x: 3.0, y: 3.0 };
            let pixels = ball.pixels();
            assert!(pixels.contains(&(0, 3, 0x00_FF_00)));
        }

        #[test]
        fn boundary_pixel_at_y_min_is_included() {
            // Ball at (3, 3): y_min should be 0. Pixel (3, 0) is at distance 3.0 = radius.
            let ball = EnergyBall { x: 3.0, y: 3.0 };
            let pixels = ball.pixels();
            assert!(pixels.contains(&(3, 0, 0x00_FF_00)));
        }

        #[test]
        fn boundary_pixel_at_x_max_is_included() {
            // Ball at (1, 1): x_max should be 4. Pixel (4, 1) is at distance 3.0 = radius.
            let ball = EnergyBall { x: 1.0, y: 1.0 };
            let pixels = ball.pixels();
            assert!(pixels.contains(&(4, 1, 0x00_FF_00)));
        }

        #[test]
        fn boundary_pixel_at_y_max_is_included() {
            // Ball at (1, 1): y_max should be 4. Pixel (1, 4) is at distance 3.0 = radius.
            let ball = EnergyBall { x: 1.0, y: 1.0 };
            let pixels = ball.pixels();
            assert!(pixels.contains(&(1, 4, 0x00_FF_00)));
        }

        #[test]
        fn a_diagonal_point_just_outside_radius_is_not_included() {
            // Ball at (10, 10). Pixel (12, 12): distance = sqrt(8) ≈ 2.83, included.
            // Pixel (13, 12): distance = sqrt(9+4) = sqrt(13) ≈ 3.61, excluded.
            let ball = EnergyBall { x: 10.0, y: 10.0 };
            let pixels = ball.pixels();
            assert!(pixels.contains(&(12, 12, 0x00_FF_00)));
            assert!(!pixels.iter().any(|&(x, y, _)| x == 13 && y == 12));
        }
    }
}
