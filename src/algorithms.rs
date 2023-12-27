use fastrand::Rng;

use crate::grid::Grid;

pub fn binary_tree(grid: &mut Grid) {
    let mut rng = Rng::new();

    for cell in grid.iter() {
        let carve_to = if rng.bool() {
            cell.north().or(cell.east())
        } else {
            cell.east().or(cell.north())
        };
        if let Some(neighbor) = carve_to {
            grid.link(&cell, &neighbor);
        }
    }
}
