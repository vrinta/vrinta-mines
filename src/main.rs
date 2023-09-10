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

fn main() {
    let mut c: Cell = Cell {
        x_position: 0,
        y_position: 0,
        value: CellKind::Empty,
    };
}
