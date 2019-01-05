#[derive(Copy, Clone)]
pub struct Dimensions {
    pub width: i8,
    pub height: i8
}

impl Dimensions {
    pub fn new(width: i8, height: i8) -> Dimensions {
        Dimensions { width, height }
    }
}
