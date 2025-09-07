use crate::module::{ModuleName, ModuleType};
use crate::module::module_base::{ModuleBase, ModuleBaseTrait, ModuleTrait};
use crate::signal::Signal;
use crate::signal_value::SignalValue;
use std::collections::HashMap;

pub struct Conjunction {
    module_base: ModuleBase,
    last_values: HashMap<ModuleName, SignalValue>,
}

impl Conjunction {
    pub fn new(name: ModuleName) -> Self {
        Self {
            module_base: ModuleBase::new(name),
            last_values: HashMap::new(),
        }
    }

    pub fn add_input(&mut self, input: &ModuleName) {
        self.last_values
            .entry(input.clone())
            .or_insert(SignalValue::Low);
    }
}

impl ModuleBaseTrait for Conjunction {
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

impl ModuleTrait for Conjunction {
    fn module_type(&self) -> ModuleType {
        ModuleType::Conjunction
    }

    fn process(&mut self, input: &Signal) -> Vec<Signal> {
        /* update the last value for the input */
        self.last_values
            .insert(input.origin().clone(), input.value());
        /* generate output */
        self.module_base
            .get_destinations()
            .iter()
            .map(|destination| {
                let output_value = if self.last_values.values().all(|&v| v == SignalValue::High) {
                    SignalValue::Low
                } else {
                    SignalValue::High
                };
                Signal::new(
                    self.module_base.name().clone(),
                    destination.clone(),
                    output_value,
                )
            })
            .collect()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
