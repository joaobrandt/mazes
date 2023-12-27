use fastrand::Rng;

use crate::grid::{Cell, Grid};

pub fn binary_tree(grid: &mut Grid) {
    let mut rng = Rng::new();

    for cell in grid.iter() {
        let north_cell = grid.north(&cell);
        let east_cell = grid.east(&cell);

        let carve_to = if rng.bool() {
            north_cell.or(east_cell)
        } else {
            east_cell.or(north_cell)
        };

        if let Some(neighbor) = carve_to {
            grid.link(&cell, &neighbor);
        }
    }
}
