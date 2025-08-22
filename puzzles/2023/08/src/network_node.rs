use crate::instruction::Instruction;

pub type NodeId = String;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct NetworkNode {
    next_nodes: [NodeId; 2],
}

impl NetworkNode {
    pub fn new(left_node: NodeId, right_node: NodeId) -> Self {
        Self {
            next_nodes: [left_node, right_node],
        }
    }

    pub fn navigate(&self, instruction: &Instruction) -> &NodeId {
        match instruction {
            Instruction::Left => &self.next_nodes[0],
            Instruction::Right => &self.next_nodes[1],
        }
    }
}
