pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    cells: Vec<Cell>,
}

pub struct Cell {
    pub row: usize,
    pub column: usize,
    links: u8,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Grid {
        let size = rows * columns;
        let mut grid = Grid {
            rows,
            columns,
            cells: Vec::with_capacity(size),
        };
        for i in 0..size {
            grid.cells.push(Cell {
                row: i / columns,
                column: i % columns,
                links: 0u8,
            })
        }
        grid
    }

    pub fn north(&self, cell: Cell) -> Option<&Cell> {
        self.cell(cell.row - 1, cell.column)
    }

    pub fn east(&self, cell: Cell) -> Option<&Cell> {
        self.cell(cell.row, cell.column + 1)
    }

    pub fn cell(&self, row: usize, column: usize) -> Option<&Cell> {
        if row >= self.rows {
            return None;
        }

        if column >= self.columns {
            return None;
        }

        let index = row + self.columns * column;
        Some(&self.cells[index])
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Cell> {
        self.cells.iter()
    }
}
