mod grid;
use grid::*;
fn main() {
    let mut grid = new_grid(8);
    grid.toggle_cell(3, 2);
    grid.print();

    let input = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 0, 0],
    ];
    let grid2 = read_grid(input);
    match grid2 {
        Ok(v) => v.print(),
        Err(e) => panic!("error parsing grid: {e:?}"),
    }
    
    println!("Hello, world!");
}
