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
}

#[derive(Debug, Clone)]
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
        return Board {
            size,
            elements: vec![],
            side_size: side_size as u16,
        };
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

    let mut b = Board::new(9);
    println!("{:?}", ptr::addr_of!(b));
    let mut b_ref = &mut b;
    b_ref.set_element(0, 0, CellKind::Bomb);
    println!("{:?}", ptr::addr_of!(b_ref));
    b_ref.set_element(0, 1, CellKind::Number(1));
    println!("{:?}", ptr::addr_of!(b_ref));

    println!("{:?}", b_ref);
    println!("{:?}", b);
}
