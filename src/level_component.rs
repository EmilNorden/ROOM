use crate::events::{Event, EventConsumer};

pub struct LevelComponent {

}

impl LevelComponent {
    pub fn new() -> Self {
        Self{}
    }
    // G_Responder
    pub fn handle_event(event: &Event) -> bool {
        false
    }
}

impl EventConsumer for LevelComponent {
    fn consume(&mut self, event: &Event) -> bool {
        false
    }
}