mod grid;

use crate::grid::Grid;

const BAR_H: &str = "\u{2501}";
const BAR_V: &str = "\u{2503}";
const CORNER_LT: &str = "\u{250F}";
const CORNER_RT: &str = "\u{2513}";
const CORNER_LB: &str = "\u{2517}";
const CORNER_RB: &str = "\u{251B}";
const SPLIT_R: &str = "\u{2523}";
const SPLIT_L: &str = "\u{252B}";
const SPLIT_B: &str = "\u{2533}";
const SPLIT_T: &str = "\u{253B}";
const CROSS: &str = "\u{254B}";

fn main() {
    println!("\u{250F}\u{2501}\u{2501}\u{2501}\u{2501}\u{2533}\u{254B}");
    let grid = Grid::new(4, 4);

    for cell in grid.iter() {
        println!("Cell in row {}, colum {}!", cell.row, cell.column);
    }

    for row in 0..grid.rows {
        println!("Row {}", row);

        for cell in grid.iter_row(row) {
            println!("Cell in row {}, colum {}!", cell.row, cell.column);
        }
    }
}

fn print_grid(grid: &Grid) {
    let chars_horizontal = grid.columns * 3 - 1;
    let chars_vertical = grid.rows * 3 - 1;

    let mut lines = vec![String::with_capacity(chars_horizontal); chars_vertical];

    for row in 0..grid.rows {
        if first_row {
            top.push_str(CORNER_LT);
        } else if last_row {
            top.push_str(CORNER_LB);
        } else {
            top.push_str(SPLIT_R);
        }

        println!("{}", top);
    }
}

/*
* 01 =   kkkk
*
*/
