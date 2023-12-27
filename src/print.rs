use crate::grid::Grid;

const SMALL_TOP: &str = "\u{2579}";
const SMALL_RIGHT: &str = "\u{257A}";
const SMALL_BOTTOM: &str = "\u{257B}";
const SMALL_LEFT: &str = "\u{2578}";
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

const TOP: u8 = 0x01;
const RIGHT: u8 = 0x02;
const BOTTOM: u8 = 0x04;
const LEFT: u8 = 0x08;

const TILE_ROWS_BY_CELL: usize = 3;
const TILE_COLUMNS_BY_CELL: usize = 5;

pub fn print_grid(grid: &Grid) {
    // tiles rows and columns are arithmetic progressions
    let tiles_rows = TILE_ROWS_BY_CELL + (grid.rows - 1) * (TILE_ROWS_BY_CELL - 1);
    let tiles_columns = TILE_COLUMNS_BY_CELL + (grid.columns - 1) * (TILE_COLUMNS_BY_CELL - 1);

    let mut tiles = vec![0u8; tiles_columns * tiles_rows];

    for cell in grid.iter() {
        let row_offset = cell.row * tiles_columns * (TILE_ROWS_BY_CELL - 1);
        let column_offset = cell.column * (TILE_COLUMNS_BY_CELL - 1);
        let index = row_offset + column_offset;

        if !grid.is_linked_north(&cell) {
            tiles[index] |= RIGHT;
            for j in 1..TILE_COLUMNS_BY_CELL - 1 {
                tiles[index + j] |= RIGHT | LEFT;
            }
            tiles[index + TILE_COLUMNS_BY_CELL - 1] |= LEFT;
        }

        if !grid.is_linked_south(&cell) {
            let last_row_modifier = index + tiles_columns * (TILE_ROWS_BY_CELL - 1);

            tiles[last_row_modifier] |= RIGHT;
            for j in 1..TILE_COLUMNS_BY_CELL - 1 {
                tiles[last_row_modifier + j] |= RIGHT | LEFT;
            }
            tiles[last_row_modifier + TILE_COLUMNS_BY_CELL - 1] |= LEFT;
        }

        if !grid.is_linked_east(&cell) {
            let last_column_modifier = index + (TILE_COLUMNS_BY_CELL - 1);
            tiles[last_column_modifier] |= BOTTOM;
            for i in 1..TILE_ROWS_BY_CELL - 1 {
                tiles[last_column_modifier + tiles_columns * i] |= BOTTOM | TOP;
            }
            tiles[last_column_modifier + tiles_columns * (TILE_ROWS_BY_CELL - 1)] |= TOP;
        }

        if !grid.is_linked_west(&cell) {
            tiles[index] |= BOTTOM;
            for i in 1..TILE_ROWS_BY_CELL - 1 {
                tiles[index + tiles_columns * i] |= BOTTOM | TOP;
            }
            tiles[index + tiles_columns * (TILE_ROWS_BY_CELL - 1)] |= TOP;
        }
    }

    for j in 0..tiles.len() {
        if j % tiles_columns == 0 {
            println!();
        }
        let string = match tiles[j] {
            1 => SMALL_TOP,
            2 => SMALL_RIGHT,
            3 => CORNER_LB,
            4 => SMALL_BOTTOM,
            5 => BAR_V,
            6 => CORNER_LT,
            7 => SPLIT_R,
            8 => SMALL_LEFT,
            9 => CORNER_RB,
            10 => BAR_H,
            11 => SPLIT_T,
            12 => CORNER_RT,
            13 => SPLIT_L,
            14 => SPLIT_B,
            15 => CROSS,
            _ => " ",
        };
        print!("{}", string);
    }
    println!();
}
