pub mod broadcaster;
pub mod conjunction;
pub mod flip_flop;
pub mod module_base;

pub type ModuleName = String;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModuleType {
    Broadcaster,
    Conjunction,
    FlipFlop,
}
