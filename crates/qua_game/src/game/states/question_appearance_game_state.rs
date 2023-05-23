use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionAppearanceGameState;

impl GameStateInteraction for QuestionAppearanceGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        event: &InputEvent,
        author: &PersonName,
        persons: &mut Persons,
        _: &mut PackageState,
    ) -> Option<GameState> {
        let person = persons.get(author).unwrap();
        match (event, person) {
            (InputEvent::Timeout, Person::Host(_)) => {
                Some(GameState::QuestionMatter(QuestionMatterGameState::default()))
            }
            _ => None,
        }
    }
}
