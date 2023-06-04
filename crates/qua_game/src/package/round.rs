use serde::{Deserialize, Serialize};

use super::theme::ThemeState;

pub type RoundIndex = usize;

#[derive(Clone, Serialize, Deserialize)]
pub struct RoundState {
    pub themes: Vec<ThemeState>,
}
