use crate::part::Part;
use crate::symbol::Symbol;

#[derive(Eq, PartialEq, Debug, Default)]
pub struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    pub fn new(parts: Vec<Part>, symbols: Vec<Symbol>) -> Self {
        Self { parts, symbols }
    }

    pub fn merge(&mut self, other: Schematic) {
        self.parts.extend(other.parts);
        self.symbols.extend(other.symbols);
    }

    pub fn parts(&self) -> &[Part] {
        &self.parts
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }
}
