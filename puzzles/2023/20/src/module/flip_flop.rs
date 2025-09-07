use crate::module::module_base::{ModuleBase, ModuleBaseTrait, ModuleTrait};
use crate::module::{ModuleName, ModuleType};
use crate::signal::Signal;
use crate::signal_value::SignalValue;

pub struct FlipFlop {
    module_base: ModuleBase,
    last_value: SignalValue,
}

impl FlipFlop {
    pub fn new(name: ModuleName) -> Self {
        Self {
            module_base: ModuleBase::new(name),
            last_value: SignalValue::Low,
        }
    }
}

impl ModuleBaseTrait for FlipFlop {
    fn name(&self) -> &ModuleName {
        self.module_base.name()
    }

    fn add_destination(&mut self, destination: &ModuleName) {
        self.module_base.add_destination(destination);
    }

    fn get_destinations(&self) -> &Vec<ModuleName> {
        self.module_base.get_destinations()
    }
}

fn flip(value: SignalValue) -> SignalValue {
    match value {
        SignalValue::High => SignalValue::Low,
        SignalValue::Low => SignalValue::High,
    }
}

impl ModuleTrait for FlipFlop {
    fn module_type(&self) -> ModuleType {
        ModuleType::FlipFlop
    }

    fn process(&mut self, input: &Signal) -> Vec<Signal> {
        if input.value() == SignalValue::High {
            vec![]
        } else {
            /* update the last value */
            self.last_value = flip(self.last_value);
            /* generate output */
            self.module_base
                .get_destinations()
                .iter()
                .map(|destination| {
                    Signal::new(
                        self.module_base.name().clone(),
                        destination.clone(),
                        self.last_value,
                    )
                })
                .collect()
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
