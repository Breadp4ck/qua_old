use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct EndingGameState;

impl GameStateInteraction for EndingGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        _: &InputEvent,
        _: &PersonName,
        _: &mut Persons,
        _: &mut PackageState,
    ) -> Option<GameState> {
        None
    }
}

