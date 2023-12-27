use std::ops::Range;

pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    pub size: usize,
    cells: Vec<u8>,
}

#[derive(Eq, PartialEq)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
}

const LINK_NORTH: u8 = 0x01;
const LINK_SOUTH: u8 = 0x02;
const LINK_EAST: u8 = 0x04;
const LINK_WEST: u8 = 0x08;

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Grid {
        let size = rows * columns;
        Grid {
            rows,
            columns,
            size,
            cells: vec![0u8; size],
        }
    }

    pub fn link(&mut self, from: &Cell, to: &Cell) {
        let index_from = self.to_index(from);
        let index_to = self.to_index(to);

        let mut value_from = self.cells[index_from];
        let mut value_to = self.cells[index_to];

        if self.is_north(from, to) {
            value_from |= LINK_NORTH;
            value_to |= LINK_SOUTH;
        } else if self.is_south(from, to) {
            value_from |= LINK_SOUTH;
            value_to |= LINK_NORTH;
        } else if self.is_east(from, to) {
            value_from |= LINK_EAST;
            value_to |= LINK_WEST;
        } else if self.is_west(from, to) {
            value_from |= LINK_WEST;
            value_to |= LINK_EAST;
        }

        self.cells[index_from] = value_from;
        self.cells[index_to] = value_to;
    }

    fn is_north(&self, from: &Cell, to: &Cell) -> bool {
        from.column == to.column && from.row > 0 && from.row - 1 == to.row
    }

    fn is_south(&self, from: &Cell, to: &Cell) -> bool {
        from.column == to.column && from.row < self.rows && from.row + 1 == to.row
    }

    fn is_east(&self, from: &Cell, to: &Cell) -> bool {
        from.row == to.row && from.column < self.columns && from.column + 1 == to.column
    }

    fn is_west(&self, from: &Cell, to: &Cell) -> bool {
        from.row == to.row && from.column > 0 && from.column - 1 == to.column
    }

    fn is_linked(&self, cell: &Cell, value: u8) -> bool {
        self.cells[self.to_index(cell)] & value == value
    }

    pub fn is_linked_north(&self, cell: &Cell) -> bool {
        self.is_linked(cell, LINK_NORTH)
    }

    pub fn is_linked_south(&self, cell: &Cell) -> bool {
        self.is_linked(cell, LINK_SOUTH)
    }

    pub fn is_linked_east(&self, cell: &Cell) -> bool {
        self.is_linked(cell, LINK_EAST)
    }

    pub fn is_linked_west(&self, cell: &Cell) -> bool {
        self.is_linked(cell, LINK_WEST)
    }

    pub fn north(&self, cell: &Cell) -> Option<Cell> {
        if cell.row == 0 {
            return None;
        }
        self.cell(cell.row - 1, cell.column)
    }

    pub fn south(&self, cell: &Cell) -> Option<Cell> {
        self.cell(cell.row + 1, cell.column)
    }

    pub fn east(&self, cell: &Cell) -> Option<Cell> {
        self.cell(cell.row, cell.column + 1)
    }

    pub fn west(&self, cell: &Cell) -> Option<Cell> {
        if cell.column == 0 {
            return None;
        }
        self.cell(cell.row, cell.column - 1)
    }

    pub fn cell(&self, row: usize, column: usize) -> Option<Cell> {
        if row >= self.rows {
            return None;
        }

        if column >= self.columns {
            return None;
        }
        Some(Cell { row, column })
    }

    pub fn iter(&self) -> CellsIterator {
        CellsIterator {
            rows_size: self.columns,
            range: (0..self.size),
            index: 0,
        }
    }

    pub fn to_index(&self, cell: &Cell) -> usize {
        cell.row * self.columns + cell.column
    }
}

pub struct CellsIterator {
    rows_size: usize,
    range: Range<usize>,
    index: usize,
}

impl Iterator for CellsIterator {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.range.contains(&self.index) {
            let cell = Cell {
                row: self.index / self.rows_size,
                column: self.index % self.rows_size,
            };
            self.index += 1;
            return Some(cell);
        }
        None
    }
}
