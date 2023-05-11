use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BoardGameState;

impl GameStateInteraction for BoardGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &StateInputEvent,
        author: &mut Person,
    ) -> Option<GameState> {
        match (event, author) {
            (StateInputEvent::SelectQuestion(question), Person::Player(player)) => {
                if let Some(leader_name) = context.lead_player.clone() {
                    //TODO: is question selectable?
                    //      (current round, not selected previously)
                    if *player.name() == leader_name {
                        return Some(GameState::QuestionAppearance(
                            QuestionAppearanceGameState::default(),
                        ));
                    }
                }

                None
            }
            _ => None,
        }
    }
}
