mod grid;

use crate::grid::Grid;

fn main() {
    println!("\u{250F}\u{2501}\u{2501}\u{2501}\u{2501}\u{2533}\u{254B}");
    let grid = Grid::new(4, 4);

    for cell in grid.iter() {
        println!("Cell in row {}, colum {}!", cell.row, cell.column);
    }
}

// const LINK_NORTH: u8 = 0x01;
// const LINK_SOUTH: u8 = 0x02;
// const LINK_EAST: u8 = 0x03;
// const LINK_WEST: u8 = 0x04;
