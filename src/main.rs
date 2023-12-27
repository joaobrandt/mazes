mod algorithms;
mod grid;
mod print;

use crate::grid::Grid;
use algorithms::binary_tree;
use print::print_grid;

fn main() {
    let mut grid = Grid::new(20, 20);
    binary_tree(&mut grid);
    print_grid(&grid)
}
