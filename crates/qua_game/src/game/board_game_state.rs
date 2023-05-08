use super::*;

#[derive(Default)]
pub struct BoardGameState;

impl GameStateInteraction for BoardGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState> {
        match (event, author) {
            (GameEventLocal::SelectQuestion(question), Person::Player(player)) => {
                if let Some(leader_name) = context.lead_player.clone() {
                    //TODO: is question selectable?
                    //      (current round, not selected previously)
                    if *player.name() == leader_name {
                        return Some(GameState::Greet(GreetGameState::default()))
                    }
                }

                None
            }
            _ => None,
        }
    }
}
