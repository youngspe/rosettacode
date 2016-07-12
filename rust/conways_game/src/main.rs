use std::io::BufRead;

use Cell::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn next(self, n: i32) -> Self {
        match (self, n) {
            (Dead, 3) => Alive, // It takes three to give birth!
            (Alive, 0...1) => Dead, // Lonely
            (Alive, 4...8) => Dead, // Overcrowded
            _ => self, // No change
        }
    }
}

#[derive(Clone)]
struct Grid {
    pub height: i32,
    pub width: i32,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(h: i32, w: i32) -> Self {
        let size = h * w;
        Grid {
            height: h,
            width: w,
            cells: (0..size).map(|_| Dead).collect(),
        }
    }
    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.cells[(y * self.width + x) as usize]
        } else {
            Dead // out of bounds cells are considered Dead
        }
    }
    pub fn set_cell(&mut self, x: i32, y: i32, cell: Cell) {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.cells[(y * self.width + x) as usize] = cell;
        } else {
            panic!("Index out of range.");
        }
    }
    pub fn get_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for i in -1..2 {
            for j in -1..2 {
                if self.get_cell(x + i, y + j) == Alive && (i != 0 || j != 0) {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn next(&mut self) {
        let orig = self.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                self.set_cell(x, y, orig.get_cell(x, y).next(orig.get_neighbors(x, y)));
            }
        }
    }
}

fn print_grid(grid: &Grid) {
    for y in 0..grid.height {
        let mut line = String::new();
        for x in 0..grid.width {
            line.push(match grid.get_cell(x, y) {
                Dead => '.',
                Alive => '#',
            });
        }
        println!("{}", line);
    }
    println!("");
}

fn main() {
    let mut grid = Grid::new(5, 5);

    grid.set_cell(2, 1, Alive);
    grid.set_cell(2, 2, Alive);
    grid.set_cell(2, 3, Alive);

    print_grid(&grid);

    // repeat every time the user enters a new line but quit when that line is "q"
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(ref line) if line == "q" => break,
            Ok(_) => {
                grid.next();
                print_grid(&grid);
            }
            Err(err) => panic!("IO error: {}", err),
        }
    }
}
