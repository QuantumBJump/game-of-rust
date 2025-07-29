use std::io::{Write, stdout};
#[derive(Debug, PartialEq)]
pub struct Grid {
    grid_size: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn print(&self) {
        for row in &self.cells {
            for cell in row {
                if cell.value {
                    print!("X")
                } else {
                    print!("·")
                }
            }
            print!("\n")
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x].value = !self.cells[y][x].value;
    }

    pub fn delete(self) {
        print!("\x1b[{};A", &self.cells.len());
        if let Err(res) = stdout().flush() {
            panic!("{res:?}")
        }
        
        for row in &self.cells {
            println!("{}", " ".repeat(row.len()));
        }
        print!("\x1b[{};A", self.cells.len());
        if let Err(res) = stdout().flush() {
            panic!("{res:?}")
        }
    }
    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x < self.cells[0].len() as isize && y < self.cells.len() as isize && x >= 0 && y >= 0
    }

    pub fn wrap(&self, x: isize, y: isize) -> (isize, isize) {
        let xret: isize;
        let yret: isize;
        if x < 0 {
            xret = self.cells[0].len() as isize - 1;
        } else if x >= self.cells[0].len() as isize {
            xret = 0;
        } else {
            xret = x;
        }

        if y < 0 {
            yret = self.cells.len() as isize - 1;
        } else if y >=self.cells.len() as isize {
            yret = 0;
        } else {
            yret = y;
        }
        (xret, yret)
    }
}

pub fn new_grid(grid_size: usize) -> Grid {
    let mut cells = vec![];
    for i in 0..grid_size {
        let mut row: Vec<Cell> = vec![];
        for j in 0..grid_size {
            row.push(Cell {
                x: j,
                y: i,
                value: false,
            });
        }
        cells.push(row);
    }
    Grid {
        grid_size,
        cells: cells.to_vec(),
    }
}

pub fn read_grid(input: Vec<Vec<usize>>) -> Result<Grid, &'static str> {
    if input.len() == 0 {
        return Err("no values in input");
    }
    let width = input[0].len();
    let mut cells = vec![];
    for (y, row) in input.iter().enumerate() {
        if row.len() != width {
            return Err("mismatched row widths");
        }
        let mut contents = vec![];
        for (x, char) in row.iter().enumerate() {
            match char {
                0 => {
                    contents.push(Cell { x, y, value: false });
                }
                1 => {
                    contents.push(Cell { x, y, value: true });
                }
                _ => {
                    return Err("invalid cell value");
                }
            }
        }
        cells.push(contents);
    }
    Ok(Grid {
        grid_size: input.len(),
        cells,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    x: usize,
    y: usize,
    value: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_grid_valid() {
        let input = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let result = read_grid(input).unwrap();
        let expected = Grid {
            grid_size: 4,
            cells: vec![
                vec![Cell{x: 0, y: 0, value: false}, Cell{x: 1, y: 0, value: true}, Cell{x: 2, y: 0, value: false}, Cell{x: 3, y: 0, value: false}],
                vec![Cell{x: 0, y: 1, value: false}, Cell{x: 1, y: 1, value: false}, Cell{x: 2, y: 1, value: false}, Cell{x: 3, y: 1, value: false}],
                vec![Cell{x: 0, y: 2, value: false}, Cell{x: 1, y: 2, value: false}, Cell{x: 2, y: 2, value: false}, Cell{x: 3, y: 2, value: false}],
                vec![Cell{x: 0, y: 3, value: false}, Cell{x: 1, y: 3, value: false}, Cell{x: 2, y: 3, value: false}, Cell{x: 3, y: 3, value: false}],
            ],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn test_read_grid_empty_input() {
        let input = vec![];
        let result = read_grid(input);
        assert_eq!(result, Err("no values in input"));
    }

    #[test]
    fn test_read_grid_not_square() {
        let input = vec![
            vec![0, 1],
            vec![0],
            vec![0, 0],
        ];
        let result = read_grid(input);
        assert_eq!(result, Err("mismatched row widths"));
    }

    #[test]
    fn test_read_grid_invalid_value() {
        let input = vec![
            vec![0, 2],
        ];
        let result = read_grid(input);
        assert_eq!(result, Err("invalid cell value"))
    }

    #[test]
    fn test_wrap() {
        let grid = read_grid(vec![vec![0, 1, 1], vec![0, 1, 0], vec![0, 1, 1]]).unwrap();
        assert_eq!(grid.wrap(-1, -1), (2, 2));
        assert_eq!(grid.wrap(3, 3), (0, 0));
    }

    #[test]
    fn test_in_bounds() {
        let grid = read_grid(vec![
            vec![0, 1, 1],
            vec![0, 1, 0],
            vec![0, 1, 1]]).unwrap();
        assert_eq!(grid.in_bounds(-1, -1), false);
        assert_eq!(grid.in_bounds(1, 3), false);
        assert_eq!(grid.in_bounds(-1, 2), false);
        assert_eq!(grid.in_bounds(3, 3), false);
        assert_eq!(grid.in_bounds(0, 0), true);
    }
}
