use serde::{Serialize, Deserialize};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct RoomId {
    id: u64,
}

impl RoomId {
    pub fn min() -> Self {
        Self { id: 0 }
    }

    pub fn next(&mut self) {
        self.id += 1;
    }
}
