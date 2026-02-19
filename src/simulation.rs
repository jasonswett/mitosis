use crate::Cell;
use crate::energy_ball::{EnergyBall, ENERGY_BALL_RADIUS};
use rand::Rng;
use std::f32::consts::TAU;

const GROWTH_RATE: f32 = 0.25;
const SPLIT_RADIUS: f32 = 10.0;
const DAUGHTER_RADIUS_FRACTION: f32 = 0.5;
const DAUGHTER_OFFSET: f32 = 6.0;
const DAUGHTER_Y_OFFSET_RANGE: f32 = 6.0;
const MIN_FPS_FOR_SPLIT: usize = 40;
const MIN_ENERGY_FOR_SPLIT: u32 = 10;
const ENERGY_BALL_VALUE: u32 = 50;
const ENERGY_BALL_SPAWN_INTERVAL_TICKS: usize = 100;
const ENERGY_BALL_FALL_SPEED: f32 = 4.0;
const REAPER_INTERVAL_TICKS: usize = 30;
const CELL_SPEED: f32 = 0.5;

pub struct Simulation {
    cells: Vec<Cell>,
    energy_balls: Vec<EnergyBall>,
    starting_energy: u64,
    ticks_since_last_spawn: usize,
    ticks_since_last_reap: usize,
    width: f32,
    height: f32,
}

impl Simulation {
    pub fn new(cells: Vec<Cell>, starting_energy: u32, width: usize, height: usize) -> Self {
        Simulation {
            cells,
            energy_balls: Vec::new(),
            starting_energy: starting_energy as u64,
            ticks_since_last_spawn: 0,
            ticks_since_last_reap: 0,
            width: width as f32,
            height: height as f32,
        }
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn energy_balls(&self) -> &[EnergyBall] {
        &self.energy_balls
    }

    pub fn total_energy(&self) -> u64 {
        self.cells.iter().map(|cell| cell.energy as u64).sum()
    }

    pub fn tick(&mut self, fps: usize) {
        if fps < MIN_FPS_FOR_SPLIT {
            return;
        }

        let grown: Vec<Cell> = self.cells.iter().map(|cell| grown_cell(cell, GROWTH_RATE)).collect();
        let mut next = Vec::with_capacity(self.cells.len() * 2);

        let mut rng = rand::thread_rng();

        for cell in &grown {
            if cell.radius >= SPLIT_RADIUS && cell.energy >= MIN_ENERGY_FOR_SPLIT {
                let [a, b] = daughter_cells(cell, &mut rng);
                next.push(a);
                next.push(b);
            } else {
                next.push(cell.clone());
            }
        }

        self.cells = next;
        resolve_overlaps(&mut self.cells);

        for cell in &mut self.cells {
            cell.x = (cell.x + cell.vx).rem_euclid(self.width);
            cell.y = (cell.y + cell.vy).rem_euclid(self.height);
        }

        for ball in &mut self.energy_balls {
            ball.y += ENERGY_BALL_FALL_SPEED;
        }
        self.energy_balls.retain(|ball| ball.y < self.height);

        self.absorb_energy_balls();

        self.ticks_since_last_spawn += 1;
        if self.ticks_since_last_spawn >= ENERGY_BALL_SPAWN_INTERVAL_TICKS
            && self.total_energy() + ENERGY_BALL_VALUE as u64 <= self.starting_energy * 2
        {
            self.energy_balls.push(EnergyBall { x: self.width / 2.0, y: 0.0 });
            self.ticks_since_last_spawn = 0;
        }

        self.ticks_since_last_reap += 1;
        if self.ticks_since_last_reap >= REAPER_INTERVAL_TICKS && self.cells.len() > 1 {
            let index = rng.gen_range(0..self.cells.len());
            self.cells.remove(index);
            self.ticks_since_last_reap = 0;
        }
    }

    fn absorb_energy_balls(&mut self) {
        let mut absorbed = vec![false; self.energy_balls.len()];
        let energy_cap = self.starting_energy * 2;
        let mut current_energy: u64 = self.cells.iter().map(|cell| cell.energy as u64).sum();

        for cell in &mut self.cells {
            for (i, ball) in self.energy_balls.iter().enumerate() {
                if absorbed[i] {
                    continue;
                }
                if current_energy + ENERGY_BALL_VALUE as u64 > energy_cap {
                    continue;
                }
                let dx = cell.x - ball.x;
                let dy = cell.y - ball.y;
                let distance_squared = dx * dx + dy * dy;
                let threshold = cell.radius + ENERGY_BALL_RADIUS;
                if distance_squared < threshold * threshold {
                    cell.energy += ENERGY_BALL_VALUE;
                    current_energy += ENERGY_BALL_VALUE as u64;
                    absorbed[i] = true;
                }
            }
        }

        let mut i = 0;
        self.energy_balls.retain(|_| {
            let keep = !absorbed[i];
            i += 1;
            keep
        });
    }
}

fn grown_cell(cell: &Cell, growth_rate: f32) -> Cell {
    Cell {
        x: cell.x,
        y: cell.y,
        radius: (cell.radius + growth_rate).min(SPLIT_RADIUS),
        energy: cell.energy,
        vx: cell.vx,
        vy: cell.vy,
    }
}

fn resolve_overlaps(cells: &mut Vec<Cell>) {
    const MAX_ITERATIONS: usize = 10;

    for _ in 0..MAX_ITERATIONS {
        for i in 1..cells.len() {
            for j in 0..i {
                let dx = cells[i].x - cells[j].x;
                let dy = cells[i].y - cells[j].y;
                let distance_squared = dx * dx + dy * dy;
                let min_distance = cells[i].radius + cells[j].radius;
                if distance_squared >= min_distance * min_distance {
                    continue;
                }
                let distance = distance_squared.sqrt();
                let overlap = min_distance - distance;
                let half_push = overlap / 2.0;

                let (nx, ny) = if distance == 0.0 {
                    (1.0, 0.0)
                } else {
                    (dx / distance, dy / distance)
                };

                cells[i].x += nx * half_push;
                cells[i].y += ny * half_push;
                cells[j].x -= nx * half_push;
                cells[j].y -= ny * half_push;
            }
        }
    }
}

fn daughter_cells(cell: &Cell, rng: &mut impl Rng) -> [Cell; 2] {
    let daughter_radius = cell.radius * DAUGHTER_RADIUS_FRACTION;
    let daughter_energy = cell.energy / 2;

    let angle_a = rng.gen_range(0.0..TAU);
    let angle_b = rng.gen_range(0.0..TAU);

    [
        Cell {
            x: cell.x - DAUGHTER_OFFSET,
            y: cell.y + rng.gen_range(-DAUGHTER_Y_OFFSET_RANGE..=DAUGHTER_Y_OFFSET_RANGE),
            radius: daughter_radius,
            energy: daughter_energy,
            vx: CELL_SPEED * angle_a.cos(),
            vy: CELL_SPEED * angle_a.sin(),
        },
        Cell {
            x: cell.x + DAUGHTER_OFFSET,
            y: cell.y + rng.gen_range(-DAUGHTER_Y_OFFSET_RANGE..=DAUGHTER_Y_OFFSET_RANGE),
            radius: daughter_radius,
            energy: daughter_energy,
            vx: CELL_SPEED * angle_b.cos(),
            vy: CELL_SPEED * angle_b.sin(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    mod grown_cell {
        use super::*;

        #[test]
        fn it_increases_the_radius_by_the_growth_rate() {
            let cell = Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 };
            let result = grown_cell(&cell, 0.5);
            assert_eq!(result.radius, 5.5);
        }

        #[test]
        fn it_preserves_position() {
            let cell = Cell { x: 30.0, y: 70.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 };
            let result = grown_cell(&cell, 0.5);
            assert_eq!(result.x, 30.0);
            assert_eq!(result.y, 70.0);
        }

        #[test]
        fn it_preserves_energy() {
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 42, vx: 0.0, vy: 0.0 };
            let result = grown_cell(&cell, 0.5);
            assert_eq!(result.energy, 42);
        }

        #[test]
        fn it_caps_radius_at_split_radius() {
            let cell = Cell { x: 50.0, y: 50.0, radius: SPLIT_RADIUS - 0.1, energy: 100, vx: 0.0, vy: 0.0 };
            let result = grown_cell(&cell, 1.0);
            assert_eq!(result.radius, SPLIT_RADIUS);
        }
    }

    mod daughter_cells {
        use super::*;
        use rand::Rng;
        use rand::rngs::StdRng;
        use rand::SeedableRng;

        fn seeded_rng() -> StdRng {
            StdRng::seed_from_u64(42)
        }

        #[test]
        fn it_returns_two_cells() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 60.0, energy: 100, vx: 0.0, vy: 0.0 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_eq!(daughters.len(), 2);
        }

        #[test]
        fn daughters_have_half_the_radius() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100, vx: 0.0, vy: 0.0 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_eq!(daughters[0].radius, 10.0);
            assert_eq!(daughters[1].radius, 10.0);
        }

        #[test]
        fn daughters_are_offset_horizontally() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100, vx: 0.0, vy: 0.0 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_eq!(daughters[0].x, 94.0);
            assert_eq!(daughters[1].x, 106.0);
        }

        #[test]
        fn daughters_are_not_touching() {
            let cell = Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS, energy: 100, vx: 0.0, vy: 0.0 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            let distance = (daughters[1].x - daughters[0].x).abs();
            let sum_of_radii = daughters[0].radius + daughters[1].radius;
            assert!(distance > sum_of_radii);
        }

        #[test]
        fn daughters_get_distinct_y_offsets() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100, vx: 0.0, vy: 0.0 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_ne!(daughters[0].y, daughters[1].y);
        }

        #[test]
        fn daughter_y_values_match_rng_output() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100, vx: 0.0, vy: 0.0 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());

            let mut rng = seeded_rng();
            let _angle_a: f32 = rng.gen_range(0.0..TAU);
            let _angle_b: f32 = rng.gen_range(0.0..TAU);
            let expected_y_0 = cell.y + rng.gen_range(-DAUGHTER_Y_OFFSET_RANGE..=DAUGHTER_Y_OFFSET_RANGE);
            let expected_y_1 = cell.y + rng.gen_range(-DAUGHTER_Y_OFFSET_RANGE..=DAUGHTER_Y_OFFSET_RANGE);
            assert_eq!(daughters[0].y, expected_y_0);
            assert_eq!(daughters[1].y, expected_y_1);
        }

        #[test]
        fn each_daughter_gets_half_the_parent_energy() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 80, vx: 0.0, vy: 0.0 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_eq!(daughters[0].energy, 40);
            assert_eq!(daughters[1].energy, 40);
        }

        #[test]
        fn daughter_cells_have_nonzero_velocity() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100, vx: 0.0, vy: 0.0 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            let speed_0 = (daughters[0].vx * daughters[0].vx + daughters[0].vy * daughters[0].vy).sqrt();
            let speed_1 = (daughters[1].vx * daughters[1].vx + daughters[1].vy * daughters[1].vy).sqrt();
            assert!((speed_0 - CELL_SPEED).abs() < 0.001);
            assert!((speed_1 - CELL_SPEED).abs() < 0.001);
        }
    }

    mod resolve_overlaps {
        use super::*;

        fn approx_eq(a: f32, b: f32) -> bool {
            (a - b).abs() < 0.001
        }

        #[test]
        fn horizontal_overlap_pushes_cells_to_exact_positions() {
            // Cells at (0,0) and (5,0), both radius 10.
            // Distance=5, min_distance=20, overlap=15, half_push=7.5.
            // nx=1, ny=0. Cell 0: x=-7.5. Cell 1: x=12.5.
            let mut cells = vec![
                Cell { x: 0.0, y: 0.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 5.0, y: 0.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
            ];
            resolve_overlaps(&mut cells);
            assert!(approx_eq(cells[0].x, -7.5), "cell 0 x was {}", cells[0].x);
            assert!(approx_eq(cells[0].y, 0.0), "cell 0 y was {}", cells[0].y);
            assert!(approx_eq(cells[1].x, 12.5), "cell 1 x was {}", cells[1].x);
            assert!(approx_eq(cells[1].y, 0.0), "cell 1 y was {}", cells[1].y);
        }

        #[test]
        fn diagonal_overlap_pushes_along_correct_axis() {
            // Cells at (0,0) and (3,4), both radius 5.
            // Distance=5, min_distance=10, overlap=5, half_push=2.5.
            // nx=3/5=0.6, ny=4/5=0.8.
            // Cell 0: (-1.5, -2.0). Cell 1: (4.5, 6.0).
            let mut cells = vec![
                Cell { x: 0.0, y: 0.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 3.0, y: 4.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ];
            resolve_overlaps(&mut cells);
            assert!(approx_eq(cells[0].x, -1.5), "cell 0 x was {}", cells[0].x);
            assert!(approx_eq(cells[0].y, -2.0), "cell 0 y was {}", cells[0].y);
            assert!(approx_eq(cells[1].x, 4.5), "cell 1 x was {}", cells[1].x);
            assert!(approx_eq(cells[1].y, 6.0), "cell 1 y was {}", cells[1].y);
        }

        #[test]
        fn non_overlapping_cells_are_unchanged() {
            let mut cells = vec![
                Cell { x: 0.0, y: 0.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 20.0, y: 0.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ];
            resolve_overlaps(&mut cells);
            assert_eq!(cells[0].x, 0.0);
            assert_eq!(cells[0].y, 0.0);
            assert_eq!(cells[1].x, 20.0);
            assert_eq!(cells[1].y, 0.0);
        }

        #[test]
        fn exactly_touching_cells_are_unchanged() {
            // Distance = sum of radii, so distance < min_distance is false.
            let mut cells = vec![
                Cell { x: 0.0, y: 0.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 20.0, y: 0.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
            ];
            resolve_overlaps(&mut cells);
            assert_eq!(cells[0].x, 0.0);
            assert_eq!(cells[1].x, 20.0);
        }

        #[test]
        fn cells_at_the_same_position_are_pushed_apart_along_x() {
            // Distance=0, min_distance=20, overlap=20, half_push=10.
            // Fallback nx=1, ny=0. Cell 0: x=40. Cell 1: x=60.
            let mut cells = vec![
                Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
            ];
            resolve_overlaps(&mut cells);
            assert!(approx_eq(cells[0].x, 40.0), "cell 0 x was {}", cells[0].x);
            assert!(approx_eq(cells[0].y, 50.0), "cell 0 y was {}", cells[0].y);
            assert!(approx_eq(cells[1].x, 60.0), "cell 1 x was {}", cells[1].x);
            assert!(approx_eq(cells[1].y, 50.0), "cell 1 y was {}", cells[1].y);
        }

        #[test]
        fn cells_with_different_radii_are_pushed_apart_correctly() {
            // Cells at (0,0) radius 6 and (4,0) radius 8.
            // Distance=4, min_distance=14, overlap=10, half_push=5.
            // nx=1, ny=0. Cell 0: x=-5. Cell 1: x=9.
            let mut cells = vec![
                Cell { x: 0.0, y: 0.0, radius: 6.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 4.0, y: 0.0, radius: 8.0, energy: 100, vx: 0.0, vy: 0.0 },
            ];
            resolve_overlaps(&mut cells);
            assert!(approx_eq(cells[0].x, -5.0), "cell 0 x was {}", cells[0].x);
            assert!(approx_eq(cells[1].x, 9.0), "cell 1 x was {}", cells[1].x);
        }

        #[test]
        fn three_cells_in_a_line_all_resolve() {
            let mut cells = vec![
                Cell { x: 0.0, y: 0.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 10.0, y: 0.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 20.0, y: 0.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
            ];
            resolve_overlaps(&mut cells);
            for i in 0..cells.len() {
                for j in (i + 1)..cells.len() {
                    let distance = ((cells[j].x - cells[i].x).powi(2) + (cells[j].y - cells[i].y).powi(2)).sqrt();
                    assert!(
                        distance >= cells[i].radius + cells[j].radius - 0.01,
                        "cells {} and {} still overlap: distance={}, min={}",
                        i, j, distance, cells[i].radius + cells[j].radius
                    );
                }
            }
        }
    }

    mod total_energy {
        use super::*;

        #[test]
        fn it_sums_energy_across_all_cells() {
            let simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 100.0, y: 100.0, radius: 5.0, energy: 250, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            assert_eq!(simulation.total_energy(), 350);
        }

        #[test]
        fn it_returns_zero_for_no_cells() {
            let simulation = Simulation::new(vec![], 10000, 200, 200);
            assert_eq!(simulation.total_energy(), 0);
        }
    }

    mod tick {
        use super::*;

        #[test]
        fn it_grows_cells_below_the_split_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 1);
            assert_eq!(simulation.cells()[0].radius, 5.0 + GROWTH_RATE);
        }

        #[test]
        fn it_splits_a_cell_that_reaches_the_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS - GROWTH_RATE, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 2);
        }

        #[test]
        fn daughters_have_reduced_radius_after_split() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS - GROWTH_RATE, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(60);
            let expected_radius = SPLIT_RADIUS * DAUGHTER_RADIUS_FRACTION;
            assert_eq!(simulation.cells()[0].radius, expected_radius);
            assert_eq!(simulation.cells()[1].radius, expected_radius);
        }

        #[test]
        fn it_skips_the_tick_when_fps_is_below_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(39);
            assert_eq!(simulation.cells()[0].radius, 10.0);
        }

        #[test]
        fn it_ticks_when_fps_is_exactly_at_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(40);
            assert_eq!(simulation.cells()[0].radius, 5.0 + GROWTH_RATE);
        }

        #[test]
        fn a_cell_with_energy_1_grows_but_does_not_split() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS - GROWTH_RATE, energy: 1, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 1);
            assert_eq!(simulation.cells()[0].radius, SPLIT_RADIUS);
        }

        #[test]
        fn an_energy_ball_is_spawned_after_the_interval() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            assert_eq!(simulation.energy_balls().len(), 0);
            simulation.ticks_since_last_spawn = ENERGY_BALL_SPAWN_INTERVAL_TICKS - 1;
            simulation.tick(60);
            assert_eq!(simulation.energy_balls().len(), 1);
        }

        #[test]
        fn no_energy_ball_is_spawned_before_the_interval() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(60);
            assert_eq!(simulation.energy_balls().len(), 0);
        }

        #[test]
        fn an_energy_ball_overlapping_a_cell_is_absorbed() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 50.0 });
            let energy_before = simulation.cells()[0].energy;
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, energy_before + ENERGY_BALL_VALUE);
            assert_eq!(simulation.energy_balls().len(), 0);
        }

        #[test]
        fn an_energy_ball_not_touching_any_cell_persists() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 150.0, y: 100.0 });
            simulation.tick(60);
            assert_eq!(simulation.energy_balls().len(), 1);
        }

        #[test]
        fn a_ball_at_nonzero_horizontal_distance_within_range_is_absorbed() {
            // Cell at (50, 50) grows to radius 5.25. Threshold = 5.25 + 3.0 = 8.25.
            // Ball at (57, 50): distance = 7.0 < 8.25. Absorbed.
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 57.0, y: 50.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 150);
        }

        #[test]
        fn a_ball_at_nonzero_diagonal_distance_within_range_is_absorbed() {
            // Cell at (50, 100) grows to radius 5.25. Threshold = 8.25.
            // Ball at (55, 101) falls to (55, 105): distance = sqrt(25+25) ≈ 7.07 < 8.25. Absorbed.
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 100.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 55.0, y: 101.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 150);
        }

        #[test]
        fn a_ball_at_exactly_the_collision_boundary_is_not_absorbed() {
            // Cell at (50, 50) grows to radius 5.25. Threshold = 5.25 + 3.0 = 8.25.
            // Ball at (58.25, 46) falls to (58.25, 50): distance = 8.25, not < 8.25. Not absorbed.
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 58.25, y: 46.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 100);
        }

        #[test]
        fn a_ball_beyond_range_vertically_is_not_absorbed() {
            // Cell at (50, 50) grows to radius 5.25. Threshold = 8.25.
            // Ball at (50, 36) falls to (50, 40): distance = 10 > 8.25. Not absorbed.
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 36.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 100);
        }

        #[test]
        fn only_the_overlapping_ball_is_removed_when_multiple_exist() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 150.0, y: 150.0 }); // far away
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 50.0 });   // overlapping
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 150);
            assert_eq!(simulation.energy_balls().len(), 1);
        }

        #[test]
        fn absorption_adds_exactly_50_energy() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 37, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 50.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 87);
        }

        #[test]
        fn no_ball_spawns_when_total_energy_plus_ball_value_exceeds_twice_starting_energy() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 2000, vx: 0.0, vy: 0.0 },
            ], 1000, 200, 200);
            simulation.ticks_since_last_spawn = ENERGY_BALL_SPAWN_INTERVAL_TICKS - 1;
            simulation.tick(60);
            assert_eq!(simulation.energy_balls().len(), 0);
        }

        #[test]
        fn a_ball_spawns_when_total_energy_plus_ball_value_equals_twice_starting_energy() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 1950, vx: 0.0, vy: 0.0 },
            ], 1000, 200, 200);
            simulation.ticks_since_last_spawn = ENERGY_BALL_SPAWN_INTERVAL_TICKS - 1;
            simulation.tick(60);
            assert_eq!(simulation.energy_balls().len(), 1);
        }

        #[test]
        fn absorption_is_skipped_when_it_would_exceed_the_energy_cap() {
            // Cell has 1960 energy, cap is 2000. Absorbing a 50-energy ball would exceed cap.
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 1960, vx: 0.0, vy: 0.0 },
            ], 1000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 50.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 1960);
            // Ball was not absorbed, so it persists. No new ball spawns either (also over cap).
            assert_eq!(simulation.energy_balls().len(), 1);
        }

        #[test]
        fn absorption_succeeds_when_energy_is_below_the_cap() {
            // Cell energy=1050, starting_energy=1000, cap=2000. 1050+50=1100 <= 2000.
            // Catches * → + (cap=1002, 1100>1002) and * → / (cap=500, 1100>500).
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 1050, vx: 0.0, vy: 0.0 },
            ], 1000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 50.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 1100);
        }

        #[test]
        fn absorption_succeeds_when_energy_exactly_reaches_the_cap() {
            // Cell energy=1950, starting_energy=1000, cap=2000. 1950+50=2000, NOT > 2000.
            // Catches > → >= (2000 >= 2000 would skip).
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 1950, vx: 0.0, vy: 0.0 },
            ], 1000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 50.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 2000);
        }

        #[test]
        fn two_overlapping_balls_are_both_absorbed_when_under_the_cap() {
            // Catches += → *= on current_energy tracking.
            // With +=: current goes 100→150→200, both under cap 400.
            // With *=: current goes 100→5000, second ball blocked.
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 200, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 50.0 });
            simulation.energy_balls.push(EnergyBall { x: 50.0, y: 50.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 200);
            assert_eq!(simulation.energy_balls().len(), 0);
        }

        #[test]
        fn a_ball_spawns_when_total_energy_is_below_twice_starting_energy() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 1000, 200, 200);
            simulation.ticks_since_last_spawn = ENERGY_BALL_SPAWN_INTERVAL_TICKS - 1;
            simulation.tick(60);
            assert_eq!(simulation.energy_balls().len(), 1);
        }

        #[test]
        fn energy_balls_fall_downward_each_tick() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 150.0, y: 10.0 });
            simulation.tick(60);
            assert_eq!(simulation.energy_balls()[0].y, 10.0 + ENERGY_BALL_FALL_SPEED);
        }

        #[test]
        fn energy_balls_that_fall_past_the_bottom_are_removed() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 150.0, y: 199.0 });
            simulation.tick(60);
            assert_eq!(simulation.energy_balls().len(), 0);
        }

        #[test]
        fn an_energy_ball_exactly_at_the_bottom_is_removed() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            // Ball at y=196 falls to y=200, which is == height. Should be removed.
            simulation.energy_balls.push(EnergyBall { x: 150.0, y: 196.0 });
            simulation.tick(60);
            assert_eq!(simulation.energy_balls().len(), 0);
        }

        #[test]
        fn spawned_energy_balls_start_at_the_top() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.ticks_since_last_spawn = ENERGY_BALL_SPAWN_INTERVAL_TICKS - 1;
            simulation.tick(60);
            assert_eq!(simulation.energy_balls()[0].y, 0.0);
        }

        #[test]
        fn spawned_energy_balls_start_at_the_horizontal_center() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.ticks_since_last_spawn = ENERGY_BALL_SPAWN_INTERVAL_TICKS - 1;
            simulation.tick(60);
            assert_eq!(simulation.energy_balls()[0].x, 100.0);
        }

        #[test]
        fn a_ball_to_the_left_just_outside_range_is_not_absorbed() {
            // Cell at (100, 50) grows to radius 5.25. Threshold = 5.25 + 3.0 = 8.25.
            // Ball at (91, 46) falls to (91, 50). dx = 100 - 91 = 9, dy = 0.
            // Real distance = sqrt(81) = 9 >= 8.25. Not absorbed.
            // Catches dx*dx → dx+dx: sqrt(9+9) = sqrt(18) ≈ 4.24 < 8.25, wrongly absorbed.
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.energy_balls.push(EnergyBall { x: 91.0, y: 46.0 });
            simulation.tick(60);
            assert_eq!(simulation.cells()[0].energy, 100);
        }

        #[test]
        fn the_reaper_removes_a_cell_after_the_interval() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 100.0, y: 100.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 150.0, y: 150.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.ticks_since_last_reap = REAPER_INTERVAL_TICKS - 1;
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 2);
        }

        #[test]
        fn the_reaper_does_not_remove_a_cell_before_the_interval() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
                Cell { x: 100.0, y: 100.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 2);
        }

        #[test]
        fn the_reaper_does_not_remove_the_last_cell() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 5.0, energy: 100, vx: 0.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.ticks_since_last_reap = REAPER_INTERVAL_TICKS - 1;
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 1);
        }

        #[test]
        fn a_cell_moves_by_its_velocity_each_tick() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: 5.0, energy: 100, vx: 1.0, vy: -0.5 },
            ], 10000, 200, 200);
            simulation.tick(60);
            let cell = &simulation.cells()[0];
            assert_eq!(cell.x, 101.0);
            assert_eq!(cell.y, 99.5);
        }

        #[test]
        fn a_cell_wraps_around_the_right_edge() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 199.0, y: 100.0, radius: 5.0, energy: 100, vx: 2.0, vy: 0.0 },
            ], 10000, 200, 200);
            simulation.tick(60);
            let cell = &simulation.cells()[0];
            assert_eq!(cell.x, 1.0);
        }

        #[test]
        fn a_cell_wraps_around_the_bottom_edge() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 199.0, radius: 5.0, energy: 100, vx: 0.0, vy: 2.0 },
            ], 10000, 200, 200);
            simulation.tick(60);
            let cell = &simulation.cells()[0];
            assert_eq!(cell.y, 1.0);
        }
    }
}
