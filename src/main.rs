mod algorithms;
mod grid;
mod print;

use crate::grid::Grid;
use algorithms::binary_tree;
use print::print_grid;

fn main() {
    // let mut grid = Grid::new(2, 3);
    //
    // let c_0_0 = grid.cell(0, 0).unwrap();
    // let c_0_1 = grid.cell(0, 1).unwrap();
    // let c_0_2 = grid.cell(0, 2).unwrap();
    //
    // let c_1_0 = grid.cell(1, 0).unwrap();
    // let c_1_1 = grid.cell(1, 1).unwrap();
    // let c_1_2 = grid.cell(1, 2).unwrap();
    //
    // grid.link(&c_0_0, &c_1_0);
    // grid.link(&c_1_0, &c_1_1);
    // grid.link(&c_1_1, &c_0_1);
    // grid.link(&c_0_1, &c_0_2);
    // grid.link(&c_0_2, &c_1_2);

    let mut grid = Grid::new(4, 4);
    binary_tree(&mut grid);
    print_grid(&grid)
}
