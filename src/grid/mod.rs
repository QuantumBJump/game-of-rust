
#[derive(Debug,PartialEq)]
pub struct Grid{
    grid_size: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid{
    pub fn print(self) {
        for row in self.cells{
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
        self.cells[y][x].value = !self.cells[y][x].value

    }
}

pub fn new_grid(grid_size: usize) -> Grid {
    let mut cells = vec![];
    for i in 0..grid_size {
        let mut row: Vec<Cell> = vec![];
        for j in 0..grid_size {
            row.push(Cell{x: j, y: i, value: false});
        }
        cells.push(row);
    }
    Grid{
        grid_size,
        cells: cells.to_vec(),
    }
}

pub fn read_grid(input: Vec<Vec<usize>>) -> Result<Grid, &'static str> {
    let mut cells = vec![];
    for (y, row) in input.iter().enumerate() {
        let mut contents = vec![];
        for (x, char) in row.iter().enumerate() {
            match char {
                0 => {contents.push(Cell{x, y, value: false});},
                1 => {contents.push(Cell{x, y, value: true});},
                _ => {return Err("invalid cell value");},
            }
        }
        cells.push(contents);
    }
    Ok(Grid{
        grid_size: input.len(),
        cells,
    })
}


#[derive(Clone,Debug, PartialEq)]
pub struct Cell{
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
        let expected = Grid{
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
}
