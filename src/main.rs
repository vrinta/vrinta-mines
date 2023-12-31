use crate::CellKind::Number;
use rand::Rng;
use std::ptr;

#[derive(Debug, Clone)]
struct Cell {
    x_position: u16,
    y_position: u16,
    value: CellKind,
}

impl Cell {
    fn new(x_position: u16, y_position: u16, value: CellKind) -> Self {
        Cell {
            x_position,
            y_position,
            value,
        }
    }

    pub fn is_bomb(&self) -> bool {
        self.value == CellKind::Bomb
    }

    pub fn is_number(&self) -> bool {
        match self.value {
            CellKind::Number(_) => true,
            _ => false,
        }
    }

    pub fn increment_number(&mut self) {
        match self.value {
            CellKind::Empty => self.value = Number(1),
            Number(num) => {
                let new_value = num + 1;
                self.value = Number(new_value);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CellKind {
    Empty,
    Bomb,
    Number(u8),
}

#[derive(Debug)]
struct Board {
    size: u16,
    elements: Vec<Cell>,
    side_size: u16,
}

impl Board {
    pub fn new(size: u16) -> Self {
        let side_size = f64::sqrt(size as f64);
        if side_size != side_size.trunc() {
            panic!("Did not provide square board size (e.g. 4, 16, 32)");
        }
        let empty_vec = Board::init_empty_vector(size);
        let mut board = Board {
            size,
            elements: empty_vec,
            side_size: side_size as u16,
        };
        let board_ref: &mut Board = &mut board;
        board_ref.insert_bombs(0.3);
        return board;
    }

    fn init_empty_vector(size: u16) -> Vec<Cell> {
        let mut empty_items: Vec<Cell> = vec![];
        for i in 0..size {
            let (x, y) = Board::from_vector_index_to_coordinates(size, i as usize);
            let c = Cell {
                x_position: x,
                y_position: y,
                value: CellKind::Empty,
            };
            empty_items.insert(to_usize(i), c);
        }
        empty_items
    }

    pub fn set_element(&mut self, x: u16, y: u16, cell_kind: CellKind) {
        let c = Cell {
            x_position: x,
            y_position: y,
            value: cell_kind,
        };
        let index = Board::from_coordinates_to_vector_index(self.side_size, x, y);
        self.elements.insert(index, c);
    }

    pub fn get_element(&mut self, x: u16, y: u16) -> &Cell {
        let index = Board::from_coordinates_to_vector_index(self.side_size, x, y);
        self.elements.get(index).unwrap()
    }

    fn insert_bombs(&mut self, bombs_percentage: f32) {
        let size = self.size;
        let target_number_of_bombs_rounded: u16 =
            Board::target_number_of_bombs(size, bombs_percentage);
        let mut random_range = rand::thread_rng();
        let mut current_number_of_bombs = 0;
        while current_number_of_bombs < target_number_of_bombs_rounded {
            let random_index_with_bomb: usize = to_usize(random_range.gen_range(0..size));
            let current_cell: &mut Cell = self.elements.get_mut(random_index_with_bomb).unwrap();
            if !current_cell.is_bomb() {
                current_cell.value = CellKind::Bomb;
                current_number_of_bombs += 1;
            }
        }
    }
    fn target_number_of_bombs(size: u16, bombs_percentage: f32) -> u16 {
        let target_number_of_bombs: f32 = size as f32 * bombs_percentage;
        target_number_of_bombs.round() as u16
    }

    fn from_coordinates_to_vector_index(side_size: u16, x: u16, y: u16) -> usize {
        if x >= side_size || y >= side_size {
            panic!("Row or column out of index");
        }
        (x * side_size + y) as usize
    }

    fn from_vector_index_to_coordinates(size: u16, index: usize) -> (u16, u16) {
        let cast_index = to_u16(index);
        let row = cast_index / size;
        let column = cast_index % size;
        (row, column)
    }
}

// Utility funcitons
fn to_usize(value: u16) -> usize {
    value as usize
}

fn to_u16(value: usize) -> u16 {
    value as u16
}

fn main() {
    let mut c: Cell = Cell {
        x_position: 0,
        y_position: 0,
        value: CellKind::Empty,
    };

    let mut b = Board::new(25);
    println!("{:?}", ptr::addr_of!(b));
    println!("{:?}", b);
    let mut b_ref = &mut b;
    // b_ref.set_element(0, 0, CellKind::Bomb);
    // println!("{:?}", ptr::addr_of!(b_ref));
    // b_ref.set_element(0, 1, CellKind::Number(1));
    // println!("{:?}", ptr::addr_of!(b_ref));

    // let mut bboard_it = &mut b_ref.elements;
    //
    // let mut bombs: u16 = 0;
    // for c in bboard_it{
    //     if (c.is_bomb()) {
    //         bombs += 1;
    //     }
    // }

    // println!("{:?}", bombs);

    println!("{:?}", b_ref);

    for x in 0..b_ref.side_size {
        for y in 0..b_ref.side_size {
            let mut c = b_ref.get_element(x, y).clone();
            if (c.is_bomb()) {
                println!("{:?}", c);
            } else if (c.value == CellKind::Empty) {
                c.increment_number();
                c.increment_number();
            }
            println!("{:?}", c);
        }
    }
}
