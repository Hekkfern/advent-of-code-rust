use crate::instruction::Instruction;
use crate::network_node::{NetworkNode, NodeId};
use num_integer::Integer;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<NodeId, NetworkNode>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }

    pub fn add_node(&mut self, node_id: NodeId, node: NetworkNode) {
        self.nodes.insert(node_id, node);
    }

    pub fn navigate_from_aaa_to_zzz(&self) -> u64 {
        const INITIAL_NODE_ID: &str = "AAA";
        const FINAL_NODE_ID: &str = "ZZZ";

        let mut instruction_iter = self.instructions.iter().cycle();
        let mut step_counter: u64 = 0;
        let mut current_node_id: &str = INITIAL_NODE_ID;

        while current_node_id != FINAL_NODE_ID {
            let instruction = instruction_iter.next().unwrap();
            current_node_id = self.nodes[current_node_id].navigate(instruction);
            step_counter += 1;
        }

        step_counter
    }

    pub fn navigate_from_all_xxa_to_all_xxz(&self) -> u64 {
        let is_final_node = |node_id: &NodeId| node_id.ends_with("Z");
        let is_initial_node = |node_id: &NodeId| node_id.ends_with("A");

        let mut step_result: u64 = 1;
        for initial_node_id in self.nodes.keys().filter(|node_id| is_initial_node(node_id)) {
            let mut step_counter: u64 = 0;
            let mut current_node_id: &NodeId = initial_node_id;
            let mut instruction_iter = self.instructions.iter().cycle();

            while !is_final_node(current_node_id) {
                let instruction = instruction_iter.next().unwrap();
                current_node_id = self.nodes[current_node_id].navigate(instruction);
                step_counter += 1;
            }

            step_result = step_result.lcm(&step_counter);
        }

        step_result
    }
}
