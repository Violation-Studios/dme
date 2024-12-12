type Name = String;
type Description = String;

pub struct Zone {
    name: Name,
    description: Description,
}

impl Zone {
    pub fn new(name: &str, description: &str) -> Self {
        let owned_name = name.to_owned();
        let owned_description = description.to_owned();
        Self {
            name: owned_name,
            description: owned_description,
        }
    }
    pub const fn name(&self) -> &Name {
        &self.name
    }
    pub const fn description(&self) -> &Description {
        &self.description
    }
}
