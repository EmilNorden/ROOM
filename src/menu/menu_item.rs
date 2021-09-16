pub type ActionCallback = fn(i16);

pub struct MenuItem {
    status: i16,
    name: String,
    routine: ActionCallback,
    alpha_key: char,
}

impl MenuItem {
    pub fn new<S: Into<String>>(status: i16, name: S, routine: ActionCallback, alpha_key: char) -> Self {
        MenuItem {
            status,
            name: name.into(),
            routine,
            alpha_key,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}