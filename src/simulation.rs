use crate::Cell;

const GROWTH_RATE: f32 = 0.25;
const SPLIT_RADIUS: f32 = 20.0;
const DAUGHTER_RADIUS_FRACTION: f32 = 0.5;
const DAUGHTER_OFFSET: f32 = 12.0;

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

    pub fn tick(&mut self) {
        let grown: Vec<Cell> = self.cells.iter().map(|cell| grown_cell(cell, GROWTH_RATE)).collect();
        let mut next = Vec::new();

        for cell in &grown {
            if cell.radius >= SPLIT_RADIUS {
                let [a, b] = daughter_cells(cell);
                next.push(a);
                next.push(b);
            } else {
                next.push(cell.clone());
            }
        }

        self.cells = next;
    }
}

fn grown_cell(cell: &Cell, growth_rate: f32) -> Cell {
    Cell {
        x: cell.x,
        y: cell.y,
        radius: cell.radius + growth_rate,
    }
}

fn daughter_cells(cell: &Cell) -> [Cell; 2] {
    let daughter_radius = cell.radius * DAUGHTER_RADIUS_FRACTION;

    [
        Cell {
            x: cell.x - DAUGHTER_OFFSET,
            y: cell.y,
            radius: daughter_radius,
        },
        Cell {
            x: cell.x + DAUGHTER_OFFSET,
            y: cell.y,
            radius: daughter_radius,
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
            let cell = Cell { x: 50.0, y: 50.0, radius: 10.0 };
            let result = grown_cell(&cell, 0.5);
            assert_eq!(result.radius, 10.5);
        }

        #[test]
        fn it_preserves_position() {
            let cell = Cell { x: 30.0, y: 70.0, radius: 10.0 };
            let result = grown_cell(&cell, 0.5);
            assert_eq!(result.x, 30.0);
            assert_eq!(result.y, 70.0);
        }
    }

    mod daughter_cells {
        use super::*;

        #[test]
        fn it_returns_two_cells() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 60.0 };
            let daughters = daughter_cells(&cell);
            assert_eq!(daughters.len(), 2);
        }

        #[test]
        fn daughters_have_half_the_radius() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0 };
            let daughters = daughter_cells(&cell);
            assert_eq!(daughters[0].radius, 10.0);
            assert_eq!(daughters[1].radius, 10.0);
        }

        #[test]
        fn daughters_are_offset_horizontally() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0 };
            let daughters = daughter_cells(&cell);
            assert_eq!(daughters[0].x, 88.0);
            assert_eq!(daughters[1].x, 112.0);
        }

        #[test]
        fn daughters_are_not_touching() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0 };
            let daughters = daughter_cells(&cell);
            let distance = (daughters[1].x - daughters[0].x).abs();
            let sum_of_radii = daughters[0].radius + daughters[1].radius;
            assert!(distance > sum_of_radii);
        }

        #[test]
        fn daughters_share_the_parent_y() {
            let cell = Cell { x: 100.0, y: 100.0, radius: 20.0 };
            let daughters = daughter_cells(&cell);
            assert_eq!(daughters[0].y, 100.0);
            assert_eq!(daughters[1].y, 100.0);
        }
    }

    mod tick {
        use super::*;

        #[test]
        fn it_grows_cells_below_the_split_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 50.0, y: 50.0, radius: 10.0 },
            ]);
            simulation.tick();
            assert_eq!(simulation.cells().len(), 1);
            assert_eq!(simulation.cells()[0].radius, 10.0 + GROWTH_RATE);
        }

        #[test]
        fn it_splits_a_cell_that_reaches_the_threshold() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS - GROWTH_RATE },
            ]);
            simulation.tick();
            assert_eq!(simulation.cells().len(), 2);
        }

        #[test]
        fn daughters_have_reduced_radius_after_split() {
            let mut simulation = Simulation::new(vec![
                Cell { x: 100.0, y: 100.0, radius: SPLIT_RADIUS - GROWTH_RATE },
            ]);
            simulation.tick();
            let expected_radius = SPLIT_RADIUS * DAUGHTER_RADIUS_FRACTION;
            assert_eq!(simulation.cells()[0].radius, expected_radius);
            assert_eq!(simulation.cells()[1].radius, expected_radius);
        }
    }
}
