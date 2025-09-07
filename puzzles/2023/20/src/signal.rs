use crate::module::ModuleName;
use crate::signal_value::SignalValue;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Signal {
    origin: ModuleName,
    destination: ModuleName,
    value: SignalValue,
}

impl Signal {
    pub fn new(origin: ModuleName, destination: ModuleName, value: SignalValue) -> Self {
        Self {
            origin,
            destination,
            value,
        }
    }

    pub fn origin(&self) -> &ModuleName {
        &self.origin
    }

    pub fn destination(&self) -> &ModuleName {
        &self.destination
    }

    pub fn value(&self) -> SignalValue {
        self.value
    }
}
