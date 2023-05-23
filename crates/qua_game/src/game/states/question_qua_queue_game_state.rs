use std::{collections::HashMap, time::Duration};

use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct QuestionQuaQueueGameState {
    persons_attracted: HashMap<PersonName, Duration>,
}

impl QuestionQuaQueueGameState {
    pub fn new(name: &PersonName, duration: &Duration) -> Self {
        let mut persons_attracted = HashMap::new();
        persons_attracted.insert(name.clone(), duration.clone());

        Self { persons_attracted }
    }

    fn get_first(&self) -> PersonName {
        self.persons_attracted
            .iter()
            .min_by_key(|(_, d)| d.clone())
            .unwrap()
            .0
            .clone()
    }
}

impl GameStateInteraction for QuestionQuaQueueGameState {
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
            (InputEvent::Timeout, Person::Host(_)) => Some(GameState::QuestionQuaAnswering(
                QuestionQuaAnsweringGameState::new(self.get_first()),
            )),
            _ => None,
        }
    }
}