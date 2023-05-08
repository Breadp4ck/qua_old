use super::{PersonName, Personality};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Host {
    name: PersonName,
}

impl Personality for Host {
    fn name(&self) -> &PersonName {
        &self.name
    }

    fn with_name(name: PersonName) -> Self {
        Self {
            name,
        }
    }
}
