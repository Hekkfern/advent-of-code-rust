pub type StackId = u32;

/// A stack of crates represented as a vector of characters
/// NOTE: Index 0 means the bottom of the stack, last index means the top of the stack
pub type CrateStack = Vec<char>;
