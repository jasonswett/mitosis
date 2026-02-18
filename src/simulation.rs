use crate::Cell;
use rand::Rng;

const GROWTH_RATE: f32 = 0.25;
const SPLIT_RADIUS: f32 = 20.0;
const DAUGHTER_RADIUS_FRACTION: f32 = 0.5;
const DAUGHTER_OFFSET: f32 = 12.0;
const DAUGHTER_Y_OFFSET_RANGE: f32 = 12.0;
const MIN_FPS_FOR_SPLIT: usize = 40;
const MIN_ENERGY_FOR_SPLIT: u32 = 2;

pub struct Simulation {
    cells: Vec<Cell>,
}

impl Simulation {
    pub fn new(cells: Vec<Cell>) -> Self {
        Simulation { cells }
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn tick(&mut self, fps: usize) {
        if fps < MIN_FPS_FOR_SPLIT {
            return;
        }

        let grown: Vec<Cell> = self.cells.iter().map(|cell| grown_cell(cell, GROWTH_RATE)).collect();
        let mut next = Vec::new();

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
    }
}

fn grown_cell(cell: &Cell, growth_rate: f32) -> Cell {
    Cell {
        x: cell.x,
        y: cell.y,
        radius: (cell.radius + growth_rate).min(SPLIT_RADIUS),
        energy: cell.energy,
    }
}

fn resolve_overlaps(cells: &mut Vec<Cell>) {
    const MAX_ITERATIONS: usize = 10;

    for _ in 0..MAX_ITERATIONS {
        for i in 1..cells.len() {
            for j in 0..i {
                let dx = cells[i].x - cells[j].x;
                let dy = cells[i].y - cells[j].y;
                let distance = (dx * dx + dy * dy).sqrt();
                let overlap = (cells[i].radius + cells[j].radius - distance).max(0.0);
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

    [
        Cell {
            x: cell.x - DAUGHTER_OFFSET,
            y: cell.y + rng.gen_range(-DAUGHTER_Y_OFFSET_RANGE..=DAUGHTER_Y_OFFSET_RANGE),
            radius: daughter_radius,
            energy: daughter_energy,
        },
        Cell {
            x: cell.x + DAUGHTER_OFFSET,
            y: cell.y + rng.gen_range(-DAUGHTER_Y_OFFSET_RANGE..=DAUGHTER_Y_OFFSET_RANGE),
            radius: daughter_radius,
            energy: daughter_energy,
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
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100 };
            let result = grown_cell(&cell, 0.5);
            assert_eq!(result.radius, 10.5);
        }

        #[test]
        fn it_preserves_position() {
            let cell = Cell { x: 30.0, y: 70.0, radius: 10.0, energy: 100 };
            let result = grown_cell(&cell, 0.5);
            assert_eq!(result.x, 30.0);
            assert_eq!(result.y, 70.0);
        }

        #[test]
        fn it_preserves_energy() {
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 42 };
            let result = grown_cell(&cell, 0.5);
            assert_eq!(result.energy, 42);
        }

        #[test]
        fn it_caps_radius_at_split_radius() {
            let cell = Cell { x: 50.0, y: 50.0, radius: SPLIT_RADIUS - 0.1, energy: 100 };
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
            let cell = Cell { x: 100.0, y: 100.0, radius: 60.0, energy: 100 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_eq!(daughters.len(), 2);
        }

        #[test]
        fn daughters_have_half_the_radius() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_eq!(daughters[0].radius, 10.0);
            assert_eq!(daughters[1].radius, 10.0);
        }

        #[test]
        fn daughters_are_offset_horizontally() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_eq!(daughters[0].x, 88.0);
            assert_eq!(daughters[1].x, 112.0);
        }

        #[test]
        fn daughters_are_not_touching() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            let distance = (daughters[1].x - daughters[0].x).abs();
            let sum_of_radii = daughters[0].radius + daughters[1].radius;
            assert!(distance > sum_of_radii);
        }

        #[test]
        fn daughters_get_distinct_y_offsets() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_ne!(daughters[0].y, daughters[1].y);
        }

        #[test]
        fn daughter_y_values_match_rng_output() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 100 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());

            let mut rng = seeded_rng();
            let expected_y_0 = cell.y + rng.gen_range(-DAUGHTER_Y_OFFSET_RANGE..=DAUGHTER_Y_OFFSET_RANGE);
            let expected_y_1 = cell.y + rng.gen_range(-DAUGHTER_Y_OFFSET_RANGE..=DAUGHTER_Y_OFFSET_RANGE);
            assert_eq!(daughters[0].y, expected_y_0);
            assert_eq!(daughters[1].y, expected_y_1);
        }

        #[test]
        fn each_daughter_gets_half_the_parent_energy() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0, energy: 80 };
            let daughters = daughter_cells(&cell, &mut seeded_rng());
            assert_eq!(daughters[0].energy, 40);
            assert_eq!(daughters[1].energy, 40);
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
                Cell { x: 0.0, y: 0.0, radius: 10.0, energy: 100 },
                Cell { x: 5.0, y: 0.0, radius: 10.0, energy: 100 },
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
                Cell { x: 0.0, y: 0.0, radius: 5.0, energy: 100 },
                Cell { x: 3.0, y: 4.0, radius: 5.0, energy: 100 },
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
                Cell { x: 0.0, y: 0.0, radius: 5.0, energy: 100 },
                Cell { x: 20.0, y: 0.0, radius: 5.0, energy: 100 },
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
                Cell { x: 0.0, y: 0.0, radius: 10.0, energy: 100 },
                Cell { x: 20.0, y: 0.0, radius: 10.0, energy: 100 },
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
                Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100 },
                Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100 },
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
                Cell { x: 0.0, y: 0.0, radius: 6.0, energy: 100 },
                Cell { x: 4.0, y: 0.0, radius: 8.0, energy: 100 },
            ];
            resolve_overlaps(&mut cells);
            assert!(approx_eq(cells[0].x, -5.0), "cell 0 x was {}", cells[0].x);
            assert!(approx_eq(cells[1].x, 9.0), "cell 1 x was {}", cells[1].x);
        }

        #[test]
        fn three_cells_in_a_line_all_resolve() {
            let mut cells = vec![
                Cell { x: 0.0, y: 0.0, radius: 10.0, energy: 100 },
                Cell { x: 10.0, y: 0.0, radius: 10.0, energy: 100 },
                Cell { x: 20.0, y: 0.0, radius: 10.0, energy: 100 },
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

    mod tick {
        use super::*;

        #[test]
        fn it_grows_cells_below_the_split_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100 },
            ]);
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 1);
            assert_eq!(simulation.cells()[0].radius, 10.0 + GROWTH_RATE);
        }

        #[test]
        fn it_splits_a_cell_that_reaches_the_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS - GROWTH_RATE, energy: 100 },
            ]);
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 2);
        }

        #[test]
        fn daughters_have_reduced_radius_after_split() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS - GROWTH_RATE, energy: 100 },
            ]);
            simulation.tick(60);
            let expected_radius = SPLIT_RADIUS * DAUGHTER_RADIUS_FRACTION;
            assert_eq!(simulation.cells()[0].radius, expected_radius);
            assert_eq!(simulation.cells()[1].radius, expected_radius);
        }

        #[test]
        fn it_skips_the_tick_when_fps_is_below_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100 },
            ]);
            simulation.tick(39);
            assert_eq!(simulation.cells()[0].radius, 10.0);
        }

        #[test]
        fn it_ticks_when_fps_is_exactly_at_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 10.0, energy: 100 },
            ]);
            simulation.tick(40);
            assert_eq!(simulation.cells()[0].radius, 10.0 + GROWTH_RATE);
        }

        #[test]
        fn a_cell_with_energy_1_grows_but_does_not_split() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS - GROWTH_RATE, energy: 1 },
            ]);
            simulation.tick(60);
            assert_eq!(simulation.cells().len(), 1);
            assert_eq!(simulation.cells()[0].radius, SPLIT_RADIUS);
        }
    }
}
