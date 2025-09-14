use crate::part2::part_range::PartRange;
use crate::part2::rule::ActionType;
use crate::part2::workflow::Workflow;
use std::collections::HashMap;

pub struct System {
    workflows: HashMap<String, Workflow>,
}

fn search_accepted(
    workflows: &HashMap<String, Workflow>,
    accepted: &mut Vec<PartRange>,
    action_type: &ActionType,
    value: &mut PartRange,
) {
    match action_type {
        ActionType::Accepted => {
            accepted.push(value.clone());
        }
        ActionType::Rejected => {
            // do nothing
        }
        ActionType::GoTo(destination) => {
            let workflow = workflows.get(destination).unwrap();
            for rule in workflow.get_rules() {
                // No condition means that we are jumping to a new label
                if !rule.has_condition() {
                    search_accepted(workflows, accepted, rule.get_action(), value);
                } else {
                    // Get the "true" and "false" chunks
                    let (jump, next) = rule.process(value);
                    if let Some(mut jump_value) = jump {
                        //recurse
                        search_accepted(workflows, accepted, rule.get_action(), &mut jump_value);
                    }
                    // If the "false" chunk doesn't contain any values, we are done
                    if next.is_none() {
                        // dead end
                        break;
                    }
                    // update the relevant member with the "false" values and continue
                    *value = next.unwrap();
                }
            }
        }
    }
}

impl System {
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    pub fn add_workflow(&mut self, workflow: Workflow) {
        let name = workflow.name().to_string();
        self.workflows.insert(name, workflow);
    }

    pub fn search(&self) -> Vec<PartRange> {
        let mut accepted: Vec<PartRange> = Vec::new();
        search_accepted(
            &self.workflows,
            &mut accepted,
            &ActionType::GoTo(String::from("in")),
            &mut PartRange::new(),
        );
        accepted
    }
}
