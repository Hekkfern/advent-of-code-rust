use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Packet {
    Term(u8),
    Nest(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Packet {
    fn cmp(&self, rhs: &Self) -> Ordering {
        use Packet::*;
        match (self, rhs) {
            (Term(l), Term(r)) => l.cmp(r),
            (Nest(l), Nest(r)) => l.cmp(r),
            (Term(l), Nest(r)) => [Term(*l)][..].cmp(r),
            (Nest(l), Term(r)) => l.as_slice().cmp(&[Term(*r)]),
        }
    }
}
