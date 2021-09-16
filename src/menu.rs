use crate::menu::menu_item::MenuItem;
use crate::rendering::renderer::Renderer;
use std::thread::current;
use crate::wad::{LumpStore, By};
use crate::events::{EventConsumer, Event};

mod menu_item;
mod definitions;

const LINE_HEIGHT:i32 = 16;
const SKULL_X_OFFSET:i32 = -32;

pub struct Menu {
    menu_items: Vec<MenuItem>,
    previous_menu_index: Option<usize>,
    draw_routine: Box<dyn Fn()>,
    x: i32,
    y: i32,
    last_on: usize,
}

pub struct MenuComponent {
    is_active: bool,
    menus: Vec<Menu>,
    current_menu: usize,
    item_on: usize,
    which_skull: usize,
    skull_animation_counter: i32,
}

impl MenuComponent {
    pub fn new() -> Self {
        let mut menus = Vec::new();
        menus.push(Self::create_main_menu());
        menus.push(Self::create_episodes_menu());

        MenuComponent {
            is_active: true,
            menus,
            item_on: 0,
            current_menu: 0,
            which_skull: 0,
            skull_animation_counter: 10,
        }
    }

    fn create_main_menu() -> Menu {
        Menu {
            x: 97,
            y: 64,
            last_on: 0,
            draw_routine: Box::new(Self::draw_main_menu),
            previous_menu_index: None,
            menu_items: vec![
                MenuItem::new(1, "M_NGAME", Self::new_game, 'n'),
                MenuItem::new(1, "M_OPTION", Self::options, 'o'),
                MenuItem::new(1, "M_LOADG", Self::load_game, 'l'),
                MenuItem::new(1, "M_SAVEG", Self::save_game, 's'),
                // Another hickup with Special edition.
                MenuItem::new(1, "M_RDTHIS", Self::read_this, 'r'),
                MenuItem::new(1, "M_QUITG", Self::quit, 'q'),
            ],
        }
    }

    fn create_episodes_menu() -> Menu {
        Menu {
            x: 48,
            y: 63,
            last_on: 0,
            draw_routine: Box::new(Self::draw_main_menu),
            previous_menu_index: Some(0),
            menu_items: vec![
                MenuItem::new(1, "M_EPI1", Self::episode, 'k')
            ],
        }
    }

    pub fn tick(&mut self) {
        self.skull_animation_counter -= 1;
        if self.skull_animation_counter <= 0 {
            self.which_skull ^= 1;
            self.skull_animation_counter = 8;
        }
    }

    pub fn draw(&self, renderer: &mut dyn Renderer, lumps: &LumpStore) {
        if !self.is_active {
            return;
        }

        let current_menu = &self.menus[self.current_menu];
        (current_menu.draw_routine)();

        let x = current_menu.x;
        let mut y = current_menu.y;
        let max = current_menu.menu_items.len();

        for item in &current_menu.menu_items {
            renderer.draw_patch(x, y, 0,
                                &lumps.get_lump(By::Name(item.name())).into());

            y += LINE_HEIGHT;
        }

        // DRAW SKULL
        const SKULL_NAMES: [&'static str; 2] = [
            "M_SKULL1", "M_SKUll2"
        ];

        renderer.draw_patch(
            x + SKULL_X_OFFSET,
            current_menu.y - 5 + self.item_on as i32 * LINE_HEIGHT,
            0,
            &lumps.get_lump(By::Name(SKULL_NAMES[self.which_skull])).into()
        );

    }

    fn new_game(choice: i16) {}

    fn options(choice: i16) {}

    fn load_game(choice: i16) {}

    fn save_game(choice: i16) {}

    fn read_this(choice: i16) {}

    fn quit(choice: i16) {}

    fn draw_main_menu() {}

    fn draw_episode_menu() {}

    fn episode(choice: i16) {}
}

impl EventConsumer for MenuComponent {
    fn consume(&mut self, event: &Event) -> bool {
        false
    }
}