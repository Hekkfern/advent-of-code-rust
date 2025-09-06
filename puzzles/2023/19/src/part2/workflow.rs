use crate::part2::rule::Rule;

pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rules: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }
}
