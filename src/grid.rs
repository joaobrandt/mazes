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
    limit_rows: usize,
    limit_colums: usize,
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

        if from.is_north(to) {
            value_from |= LINK_NORTH;
            value_to |= LINK_SOUTH;
        } else if from.is_south(to) {
            value_from |= LINK_SOUTH;
            value_to |= LINK_NORTH;
        } else if from.is_east(to) {
            value_from |= LINK_EAST;
            value_to |= LINK_WEST;
        } else if from.is_west(to) {
            value_from |= LINK_WEST;
            value_to |= LINK_EAST;
        }

        self.cells[index_from] = value_from;
        self.cells[index_to] = value_to;
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

    pub fn iter(&self) -> CellsIterator {
        CellsIterator {
            rows: self.rows,
            columns: self.columns,
            range: (0..self.size),
            index: 0,
        }
    }

    pub fn to_index(&self, cell: &Cell) -> usize {
        cell.row * self.columns + cell.column
    }
}

impl Cell {
    pub fn north(&self) -> Option<Cell> {
        self.neighbor(self.row.checked_sub(1), Some(self.column))
    }

    pub fn south(&self) -> Option<Cell> {
        self.neighbor(Some(self.row + 1), Some(self.column))
    }

    pub fn east(&self) -> Option<Cell> {
        self.neighbor(Some(self.row), Some(self.column + 1))
    }

    pub fn west(&self) -> Option<Cell> {
        self.neighbor(Some(self.row), self.column.checked_sub(1))
    }

    fn neighbor(&self, row: Option<usize>, column: Option<usize>) -> Option<Cell> {
        row.filter(|n| *n < self.limit_rows)
            .zip(column.filter(|n| *n < self.limit_colums))
            .map(|s| Cell {
                row: s.0,
                column: s.1,
                limit_rows: self.limit_rows,
                limit_colums: self.limit_colums,
            })
    }

    fn is_north(&self, to: &Cell) -> bool {
        self.column == to.column && self.row > 0 && self.row - 1 == to.row
    }

    fn is_south(&self, to: &Cell) -> bool {
        self.column == to.column && self.row + 1 == to.row
    }

    fn is_east(&self, to: &Cell) -> bool {
        self.row == to.row && self.column + 1 == to.column
    }

    fn is_west(&self, to: &Cell) -> bool {
        self.row == to.row && self.column > 0 && self.column - 1 == to.column
    }
}

pub struct CellsIterator {
    rows: usize,
    columns: usize,
    range: Range<usize>,
    index: usize,
}

impl Iterator for CellsIterator {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.range.contains(&self.index) {
            let cell = Cell {
                row: self.index / self.rows,
                column: self.index % self.rows,
                limit_rows: self.rows,
                limit_colums: self.columns,
            };
            self.index += 1;
            return Some(cell);
        }
        None
    }
}
