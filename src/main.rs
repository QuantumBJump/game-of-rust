mod grid;
use grid::*;
fn main() {
    let mut grid = new_grid(8);
    grid.toggle_cell(3, 2);

    grid.print();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    grid.delete();

    let input = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 0, 0],
    ];
    let res = read_grid(input);
    let grid2: Grid;
    match res {
        Ok(v) => {
            grid2 = v;

        }
        Err(e) => panic!("error parsing grid: {e:?}"),
    }
    grid2.print();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    grid2.delete();
    
    println!("Hello, world!");
}
