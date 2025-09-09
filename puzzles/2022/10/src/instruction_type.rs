pub enum InstructionType {
    Noop,
    Addx(i32),
}

impl From<&str> for InstructionType {
    fn from(s: &str) -> Self {
        if s == "noop" {
            InstructionType::Noop
        } else if let Some(rest) = s.strip_prefix("addx ") {
            let value = rest.parse::<i32>().unwrap();
            InstructionType::Addx(value)
        } else {
            unreachable!()
        }
    }
}
