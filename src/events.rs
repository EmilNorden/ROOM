use circular_queue::CircularQueue;
use winit::event::{ScanCode, VirtualKeyCode};

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

pub trait EventConsumer {
    fn consume(&mut self, event: &Event) -> bool;
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

    pub fn process_events(&mut self, consumers: &mut [&mut dyn EventConsumer]) {
        for event in self.events.asc_iter_mut() {
            for mut consumer in &mut *consumers {
                if consumer.consume(event) {
                    break;
                }
            }
        }

        self.events.clear();
    }
}
