use circular_queue::CircularQueue;
use winit::event::{ScanCode, VirtualKeyCode};
use crate::game_context::StateChange;

#[derive(Copy, Clone)]
pub enum Event {
    KeyDown { scancode: ScanCode, virtual_keycode: Option<VirtualKeyCode> },
    KeyUp { scancode: ScanCode, virtual_keycode: Option<VirtualKeyCode> },
    Mouse { buttons: i32, x: i32, y: i32 },
    Joystick { buttons: i32, x: i32, y: i32 },
}

const MAX_EVENTS: usize = 64;

pub struct EventSystem {
    events: CircularQueue<Event>,
}

pub enum ConsumeResult {
    Handled(Option<StateChange>),
    NotHandled,
}

pub trait EventConsumer {
    fn consume(&mut self, event: &Event) -> ConsumeResult;
}

impl EventSystem {
    pub fn new() -> Self {
        EventSystem {
            events: CircularQueue::with_capacity(MAX_EVENTS),
        }
    }

    pub fn post_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn process_events(&mut self, consumers: &mut [&mut dyn EventConsumer]) -> Option<StateChange> {
        let mut state_change = None;
        for event in self.events.asc_iter_mut() {
            for mut consumer in &mut *consumers {
                match consumer.consume(event) {
                    ConsumeResult::Handled(x) => {
                        if x.is_some() {
                            state_change = x;
                        }
                        break;
                    }
                    ConsumeResult::NotHandled => {}
                }
            }
        }

        self.events.clear();

        state_change
    }
}
