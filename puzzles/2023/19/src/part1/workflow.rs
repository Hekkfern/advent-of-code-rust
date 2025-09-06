use crate::part1::rule;

pub enum RunResult {
    Accepted,
    Rejected,
    GoTo(String),
}
pub struct Workflow {
    name: String,
    rules: Vec<rule::Rule>,
}

impl Workflow {
    pub fn new(name: &str) -> Self {
        assert!(!name.is_empty(), "Workflow name cannot be empty");
        Self {
            name: name.into(),
            rules: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_rule(&mut self, rule: rule::Rule) {
        self.rules.push(rule);
    }

    pub fn run(&self, part: &crate::part1::part::Part) -> RunResult {
        for rule in &self.rules {
            match rule.run(part) {
                rule::RunResult::Accepted => return RunResult::Accepted,
                rule::RunResult::Rejected => return RunResult::Rejected,
                rule::RunResult::GoTo(destination) => return RunResult::GoTo(destination),
                rule::RunResult::Skip => continue,
            }
        }
        unreachable!()
    }
}
