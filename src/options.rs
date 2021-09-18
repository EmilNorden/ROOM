pub enum DetailLevel {
    Low,
    High,
}

pub struct Options {
    pub(crate) detail: DetailLevel,
    pub(crate) show_messages: bool,
    pub(crate) mouse_sensitivity: i32,
    pub(crate) screen_size: i32,
}

impl Options {
    pub fn new() -> Self {
        Self {
            detail: DetailLevel::High,
            show_messages: true,
            mouse_sensitivity: 0,
            screen_size: 8,
        }
    }
}