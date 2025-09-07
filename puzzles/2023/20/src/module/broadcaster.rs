use crate::module::module_base::{ModuleBase, ModuleBaseTrait, ModuleTrait};
use crate::module::{ModuleName, ModuleType};
use crate::signal::Signal;

pub struct Broadcaster {
    module_base: ModuleBase,
}

impl Broadcaster {
    pub fn new(name: ModuleName) -> Self {
        Self {
            module_base: ModuleBase::new(name),
        }
    }
}

impl ModuleBaseTrait for Broadcaster {
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

impl ModuleTrait for Broadcaster {
    fn module_type(&self) -> ModuleType {
        ModuleType::Broadcaster
    }

    fn process(&mut self, input: &Signal) -> Vec<Signal> {
        self.module_base
            .get_destinations()
            .iter()
            .map(|destination| {
                Signal::new(
                    self.module_base.name().clone(),
                    destination.clone(),
                    input.value(),
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
