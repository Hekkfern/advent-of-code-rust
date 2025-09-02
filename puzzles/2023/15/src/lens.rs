#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Lens {
    label: String,
    focal_length: u8,
}

impl Lens {
    pub fn new(label: &str, focal_length: u8) -> Self {
        Self {
            label: label.to_string(),
            focal_length,
        }
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn get_focal_length(&self) -> u8 {
        self.focal_length
    }

    pub fn set_focal_length(&mut self, focal_length: u8) {
        self.focal_length = focal_length;
    }
}
