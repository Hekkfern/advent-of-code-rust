use crate::types::StackId;

pub struct CraneInstruction {
    num_crates: u32,
    origin: StackId,
    destination: StackId,
}

impl CraneInstruction {
    pub fn new(num_crates: u32, origin: StackId, destination: StackId) -> Self {
        Self {
            num_crates,
            origin,
            destination,
        }
    }

    pub fn num_crates(&self) -> u32 {
        self.num_crates
    }

    pub fn origin(&self) -> StackId {
        self.origin
    }

    pub fn destination(&self) -> StackId {
        self.destination
    }
}
