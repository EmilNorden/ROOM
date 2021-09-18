use crate::menu::MenuComponent;

pub type ActionCallback = fn(&mut MenuComponent, i16);

pub struct MenuItem {
    status: i16,
    name: String,
    routine: Option<ActionCallback>,
    alpha_key: char,
}

impl MenuItem {
    pub fn new<S: Into<String>>(status: i16, name: S, routine: Option<ActionCallback>, alpha_key: char) -> Self {
        MenuItem {
            status,
            name: name.into(),
            routine,
            alpha_key,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn status(&self) -> i16 { self.status }
    pub fn routine(&self) -> &Option<ActionCallback> { &self.routine }
}