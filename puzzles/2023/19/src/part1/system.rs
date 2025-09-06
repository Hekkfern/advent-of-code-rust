use crate::part1::workflow;
use std::collections::HashMap;

pub enum RunResult {
    Accepted,
    Rejected,
}

pub struct System {
    workflows: HashMap<String, workflow::Workflow>,
}

impl System {
    const INITIAL_WORKFLOW: &'static str = "in";

    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    pub fn add_workflow(&mut self, workflow: workflow::Workflow) {
        let name = workflow.name().to_string();
        self.workflows.insert(name, workflow);
    }

    pub fn run(&self, part: &crate::part1::part::Part) -> RunResult {
        let mut current_workflow_name = Self::INITIAL_WORKFLOW.to_string();
        loop {
            let workflow = self.workflows.get(&current_workflow_name).unwrap();
            match workflow.run(part) {
                workflow::RunResult::Accepted => return RunResult::Accepted,
                workflow::RunResult::Rejected => return RunResult::Rejected,
                workflow::RunResult::GoTo(destination) => {
                    current_workflow_name = destination;
                }
            }
        }
    }
}
