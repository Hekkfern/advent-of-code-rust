use crate::module::conjunction::Conjunction;
use crate::module::module_base::ModuleTrait;
use crate::module::{ModuleName, ModuleType};
use crate::signal::Signal;
use std::collections::HashMap;

pub struct Mesh {
    modules: HashMap<ModuleName, Box<dyn ModuleTrait>>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: Box<dyn ModuleTrait>) {
        let name = module.name().to_string();
        self.modules.insert(name, module);
    }

    pub fn setup(&mut self) {
        // First, collect all conjunctions and their connected modules
        let conjunction_inputs: Vec<(String, Vec<String>)> = self
            .modules
            .iter()
            .filter(|(_, module)| module.module_type() == ModuleType::Conjunction)
            .map(|(conjunction_name, _)| {
                (
                    conjunction_name.clone(),
                    self.modules
                        .iter()
                        .filter_map(|(connected_module_name, connected_module)| {
                            if connected_module
                                .get_destinations()
                                .contains(&conjunction_name)
                            {
                                Some(connected_module_name.clone())
                            } else {
                                None
                            }
                        })
                        .collect(),
                )
            })
            .collect();
        // Now, for each conjunction, add its inputs
        for (conjunction_name, connected_modules) in conjunction_inputs {
            let conjunction = self
                .modules
                .get_mut(&conjunction_name)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<Conjunction>()
                .unwrap();
            for connected_module_name in connected_modules {
                conjunction.add_input(&connected_module_name);
            }
        }
    }

    pub fn process(&mut self, input: &Signal) -> Vec<Signal> {
        if let Some(module) = self.modules.get_mut(input.destination()) {
            module.process(input)
        } else {
            vec![]
        }
    }

    pub fn get_modules_connected_to(&self, module_name: &ModuleName) -> Vec<ModuleName> {
        self.modules
            .iter()
            .filter_map(|(name, module)| {
                if module.get_destinations().contains(&module_name) {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
