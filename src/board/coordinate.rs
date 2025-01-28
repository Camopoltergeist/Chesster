pub struct Coordinate {
    column: u32,
    row: u32
}

impl Coordinate {
    pub fn new(column: u32, row: u32) -> Self {
        Self {
            column,
            row
        }
    }
}

