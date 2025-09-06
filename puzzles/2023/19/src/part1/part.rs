#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    pub fn new(x: u32, m: u32, a: u32, s: u32) -> Self {
        Self { x, m, a, s }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn m(&self) -> u32 {
        self.m
    }

    pub fn a(&self) -> u32 {
        self.a
    }

    pub fn s(&self) -> u32 {
        self.s
    }

    /// Sum of all the categories, for the final result.
    pub fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}
