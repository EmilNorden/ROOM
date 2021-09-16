#[derive(Copy, Clone)]
pub enum Event {
    KeyDown(i32),
    KeyUp(i32),
    Mouse { buttons: i32, x: i32, y: i32 },
    Joystick { buttons: i32, x: i32, y: i32 },
}

const MAX_EVENTS: usize = 64;

pub struct EventSystem {
    events: [Option<Event>; MAX_EVENTS],
    event_head: usize,
    event_tail: usize,
}

pub trait EventConsumer {
    fn consume(&mut self, event: &Event) -> bool;
}

impl EventSystem {
    pub fn new() -> Self {
        EventSystem {
            events: [None; MAX_EVENTS],
            event_head: 0,
            event_tail: 0,
        }
    }

    pub fn post_event(&mut self, event: Event) {
        self.events[self.event_head] = Some(event);
        self.event_head = (self.event_head + 1) & (MAX_EVENTS - 1);
    }

    pub fn process_events(&mut self, consumers: &Vec<&mut dyn EventConsumer>) {
        // The original code has some checks for ignoring input
        // in store demos. Im skipping that part.

        while self.event_tail != self.event_head {

            self.event_tail = (self.event_tail + 1) & (MAX_EVENTS - 1);
        }
    }
}