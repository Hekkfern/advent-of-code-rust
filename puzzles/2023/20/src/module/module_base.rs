use crate::module::{ModuleName, ModuleType};
use crate::signal::Signal;
use std::any::Any;

pub trait ModuleTrait: ModuleBaseTrait + Any {
    fn module_type(&self) -> ModuleType;
    fn process(&mut self, input: &Signal) -> Vec<Signal>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ModuleBaseTrait {
    fn name(&self) -> &ModuleName;
    fn add_destination(&mut self, destination: &ModuleName);
    fn get_destinations(&self) -> &Vec<ModuleName>;
}

pub struct ModuleBase {
    name: ModuleName,
    destinations: Vec<ModuleName>,
}

impl ModuleBase {
    pub fn new(name: ModuleName) -> Self {
        Self {
            name,
            destinations: Vec::new(),
        }
    }
}

impl ModuleBaseTrait for ModuleBase {
    fn name(&self) -> &ModuleName {
        &self.name
    }

    fn add_destination(&mut self, destination: &ModuleName) {
        self.destinations.push(destination.clone());
    }

    fn get_destinations(&self) -> &Vec<ModuleName> {
        &self.destinations
    }
}
