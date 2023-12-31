use rand::{Rng, distributions::Alphanumeric};
use serde::{Serialize, Deserialize};

use crate::MAX_ROOM_CODE_LENGTH;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoomCode {
    code: String,
}

impl From<String> for RoomCode {
    fn from(code: String) -> Self {
        Self { code }
    }
}

impl RoomCode {
    pub fn random() -> Self {
        let rng = rand::thread_rng();
        let code = rng
            .sample_iter(Alphanumeric)
            .take(MAX_ROOM_CODE_LENGTH)
            .map(char::from)
            .collect::<String>()
            .to_ascii_uppercase();

        Self { code }
    }
}

impl ToString for RoomCode {
    fn to_string(&self) -> String {
        self.code.clone()
    }
}
