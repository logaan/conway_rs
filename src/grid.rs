use std::env;
use std::collections::HashMap;

use cell::Cell;
use seeds;

pub struct Grid {
    pub cells: Vec<Cell>,
}

pub fn new(width: i16, height: i16, square_size: f32) -> Grid {
    let mut cells = Vec::new();
    for y in (0i16 .. height) {
        for x in (0i16 .. width) {
            cells.push(Cell {
                x: (x - width / 2) as f32 * square_size,
                y: (y - height / 2) as f32 * square_size,
                neighbours: [
                    (x-1, y-1), (x, y-1), (x+1, y-1),
                    (x-1, y  ),           (x+1, y  ),
                    (x-1, y+1), (x, y+1), (x+1, y+1)
                ].iter().map(|n| coords_to_index(*n, width, height)).collect(),
                alive: starts_alive(x, y),
            });
        }
    }
    Grid{ cells: cells }
}

fn coords_to_index(coords: (i16, i16), grid_width: i16, grid_height: i16) -> usize {
    let (x, y) = coords;
    let x_wrapped = (x + grid_width) % grid_width;
    let y_wrapped = (y + grid_height) % grid_height;
    (x_wrapped + (y_wrapped * grid_width)) as usize
}

fn starts_alive(x: i16, y: i16) -> bool {
    let mut seeds: HashMap<&str, fn(i16, i16) -> bool> = HashMap::new();
    seeds.insert("random", seeds::random);
    seeds.insert("diehard", seeds::diehard);
    seeds.insert("gosper_glider", seeds::gosper_glider);

    let arg = env::args().nth(1).unwrap();
    match seeds.get(&*arg) {
        Some(f) => f(x, y),
        None => false
    }
}

impl Grid {
    pub fn update(&mut self) {
        let mut alive_neighbours = Vec::new();
        for cell in self.cells.iter() {
            alive_neighbours.push(cell.neighbours.iter().filter(|n| self.cells[**n].alive).count())
        }

        for (cell, cell_alive_neighbours) in self.cells.iter_mut().zip(alive_neighbours.iter()) {
            cell.update(*cell_alive_neighbours)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::coords_to_index;

    #[test]
    fn it_returns_the_x_value_on_the_first_row() {
        assert!(coords_to_index((3, 0), 5, 3) == 3)
    }

    #[test]
    fn it_wraps_overflowing_x_values() {
        assert!(coords_to_index((6, 0), 5, 3) == 1)
    }

    #[test]
    fn it_wraps_underflowing_x_values() {
        assert!(coords_to_index((-1, 0), 5, 3) == 4)
    }

    #[test]
    fn it_adds_one_width_for_each_row() {
        assert!(coords_to_index((2, 2), 5, 3) == 12)
    }

    #[test]
    fn it_wraps_overflowing_y_values() {
        assert!(coords_to_index((1, 5), 5, 3) == 11)
    }

    #[test]
    fn it_wraps_underflowing_y_values() {
        assert!(coords_to_index((4, -2), 5, 3) == 9)
    }
}