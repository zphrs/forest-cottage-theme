// rust example code
pub struct example {
    x: int,
    y: int,
    s: String,
}

impl example {
    pub fn new() -> example {
        example { x: 1, y: 2 }
    }
    pub fn x(&self) -> int {
        self.x
    }
    pub fn y(&self) -> int {
        self.y
    }
    pub fn s(&self) -> String {
        self.s.clone()
    }
}

impl Display for example {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {}, y: {}, s: {}", self.x(), self.y(), self.s())
    }
}
