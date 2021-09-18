use crate::menu::menu_item::MenuItem;
use crate::rendering::renderer::Renderer;
use std::thread::current;
use crate::wad::{LumpStore, By};
use crate::events::{EventConsumer, Event};
use winit::event::{ScanCode, VirtualKeyCode};
use crate::options::{Options, DetailLevel};

mod menu_item;
mod definitions;

const LINE_HEIGHT: i32 = 16;
const SKULL_X_OFFSET: i32 = -32;

pub type DrawMenuCallback = fn(&MenuComponent, &mut dyn Renderer, lumps: &LumpStore);

pub struct Menu {
    menu_items: Vec<MenuItem>,
    previous_menu_index: Option<usize>,
    draw_routine: DrawMenuCallback,
    x: i32,
    y: i32,
    last_on: usize,
}

pub struct MenuComponent {
    is_active: bool,
    menus: Vec<Menu>,
    current_menu_index: usize,
    item_on: usize,
    which_skull: usize,
    skull_animation_counter: i32,
    options: Options,
}

impl MenuComponent {
    pub fn new() -> Self {
        let mut menus = Vec::new();
        menus.push(Self::create_main_menu());
        menus.push(Self::create_episodes_menu());
        menus.push(Self::create_options_menu());

        MenuComponent {
            is_active: false,
            menus,
            item_on: 0,
            current_menu_index: 0,
            which_skull: 0,
            skull_animation_counter: 10,
            options: Options::new(),
        }
    }

    fn create_main_menu() -> Menu {
        Menu {
            x: 97,
            y: 64,
            last_on: 0,
            draw_routine: Self::draw_main_menu,
            previous_menu_index: None,
            menu_items: vec![
                MenuItem::new(1, "M_NGAME", Some(Self::new_game), 'n'),
                MenuItem::new(1, "M_OPTION", Some(Self::options), 'o'),
                MenuItem::new(1, "M_LOADG", Some(Self::load_game), 'l'),
                MenuItem::new(1, "M_SAVEG", Some(Self::save_game), 's'),
                // Another hickup with Special edition.
                MenuItem::new(1, "M_RDTHIS", Some(Self::read_this), 'r'),
                MenuItem::new(1, "M_QUITG", Some(Self::quit), 'q'),
            ],
        }
    }

    fn create_episodes_menu() -> Menu {
        Menu {
            x: 48,
            y: 63,
            last_on: 0,
            draw_routine: Self::draw_main_menu,
            previous_menu_index: Some(0),
            menu_items: vec![
                MenuItem::new(1, "M_EPI1", Some(Self::episode), 'k')
            ],
        }
    }

    fn create_options_menu() -> Menu {
        Menu {
            x: 60,
            y: 37,
            last_on: 0,
            draw_routine: Self::draw_options_menu,
            previous_menu_index: Some(0),
            menu_items: vec![
                MenuItem::new(1, "M_ENDGAM", Some(Self::episode), 'e'),
                MenuItem::new(1, "M_MESSG", Some(Self::change_messages), 'm'),
                MenuItem::new(1, "M_DETAIL", Some(Self::change_detail), 'g'),
                MenuItem::new(2, "M_SCRNSZ", Some(Self::size_display), 's'),
                MenuItem::new(-1, "", None, ' '), // TODO Alpha key should be Option<char>?
                MenuItem::new(2, "M_MSENS", Some(Self::change_sensitivity), 'm'),
                MenuItem::new(-1, "", None, ' '),
                MenuItem::new(1, "M_SVOL", Some(Self::sound), 's'),
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

        let current_menu = &self.menus[self.current_menu_index];
        (current_menu.draw_routine)(self, renderer, lumps);

        let x = current_menu.x;
        let mut y = current_menu.y;
        let max = current_menu.menu_items.len();

        for item in &current_menu.menu_items {
            if !item.name().is_empty() {
                renderer.draw_patch(x, y, 0,
                                    &lumps.get_lump(By::Name(item.name())).into());
            }

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
            &lumps.get_lump(By::Name(SKULL_NAMES[self.which_skull])).into(),
        );
    }

    fn new_game(menu_component: &mut MenuComponent, choice: i16) {}

    fn options(menu_component: &mut MenuComponent, choice: i16) {
        menu_component.current_menu_index = 2;
        menu_component.item_on = menu_component.current_menu().last_on;
    }

    fn load_game(menu_component: &mut MenuComponent, choice: i16) {}

    fn save_game(menu_component: &mut MenuComponent, choice: i16) {}

    fn read_this(menu_component: &mut MenuComponent, choice: i16) {}

    fn quit(menu_component: &mut MenuComponent, choice: i16) {}

    fn episode(menu_component: &mut MenuComponent, choice: i16) {}

    fn end_game(menu_component: &mut MenuComponent, choice: i16) {}

    fn change_messages(menu_component: &mut MenuComponent, choice: i16) {}

    fn change_detail(menu_component: &mut MenuComponent, choice: i16) {}

    fn size_display(menu_component: &mut MenuComponent, choice: i16) {}

    fn change_sensitivity(menu_component: &mut MenuComponent, choice: i16) {}

    fn sound(menu_component: &mut MenuComponent, choice: i16) {}


    fn draw_main_menu(menu_component: &MenuComponent, renderer: &mut dyn Renderer, lumps: &LumpStore) {
        renderer.draw_patch(94, 2, 0, &lumps.get_lump(By::Name("M_DOOM")).into());
    }

    fn draw_options_menu(menu_component: &MenuComponent, renderer: &mut dyn Renderer, lumps: &LumpStore) {
        const MESSAGES_LINE_INDEX: i32 = 1;
        const DETAIL_LINE_INDEX: i32 = 2;
        const MOUSE_SENS_LINE_INDEX: i32 = 6;
        const SCREEN_SIZE_LINE_INDEX: i32 = 4;
        renderer.draw_patch(108, 15, 0, &lumps.get_lump(By::Name("M_OPTTTL")).into());

        let detail_text = match menu_component.options.detail {
            DetailLevel::Low => "M_GDLOW",
            DetailLevel::High => "M_GDHIGH"
        };

        renderer.draw_patch(175 + 60, 37 + LINE_HEIGHT * DETAIL_LINE_INDEX, 0,
                            &lumps.get_lump(By::Name(detail_text)).into());

        let messages_text = match menu_component.options.show_messages {
            true => "M_MSGON",
            false => "M_MSGOFF"
        };

        renderer.draw_patch(120 + 60, 37 + LINE_HEIGHT * MESSAGES_LINE_INDEX, 0,
                            &lumps.get_lump(By::Name(messages_text)).into());

        Self::draw_slider(menu_component, renderer, lumps,
                          60, 37 + LINE_HEIGHT * MOUSE_SENS_LINE_INDEX,
                          10, menu_component.options.mouse_sensitivity);

        Self::draw_slider(menu_component, renderer, lumps,
                          60, 37 + LINE_HEIGHT * SCREEN_SIZE_LINE_INDEX,
                          9, menu_component.options.screen_size);
    }

    fn draw_slider(menu_component: &MenuComponent, renderer: &mut dyn Renderer, lumps: &LumpStore,
                   x: i32, y: i32, width: i32, value: i32) {
        renderer.draw_patch(x, y, 0,
                            &lumps.get_lump(By::Name("M_THERML")).into(),
        );

        for i in 0..width {
            renderer.draw_patch(x + 8 + (i * 8), y, 0,
                                &lumps.get_lump(By::Name("M_THERMM")).into(),
            );
        }

        renderer.draw_patch(x + (width+1) * 8, y, 0,
                            &lumps.get_lump(By::Name("M_THERMR")).into(),
        );

        renderer.draw_patch(x + 8 + value * 8, y, 0,
                            &lumps.get_lump(By::Name("M_THERMO")).into(),
        );
    }

    fn draw_episode_menu() {}

    fn current_menu(&self) -> &Menu {
        &self.menus[self.current_menu_index]
    }

    fn current_menu_mut(&mut self) -> &mut Menu {
        &mut self.menus[self.current_menu_index]
    }

    fn current_menu_item(&self) -> &MenuItem {
        &self.current_menu().menu_items[self.item_on]
    }

    fn hide(&mut self) {
        self.is_active = false;
    }

    pub fn show(&mut self) {
        self.is_active = true;
    }
}

impl EventConsumer for MenuComponent {
    // M_Responder
    fn consume(&mut self, event: &Event) -> bool {
        if !self.is_active {
            return false;
        }

        // TODO: Im skipping support for navigating menu with joystick/mouse. Atleast for now
        let key = match event {
            Event::KeyDown { virtual_keycode: Some(key), .. } => {
                key
            }
            _ => return false,
        };

        // Keys usable within menu
        match key {
            VirtualKeyCode::Down => {
                loop {
                    self.item_on = (self.item_on + 1) % self.current_menu().menu_items.len();

                    //TODO: Play sound!

                    if self.current_menu_item().status() != -1 {
                        break;
                    }
                }
            }
            VirtualKeyCode::Up => {
                loop {
                    if self.item_on == 0 {
                        self.item_on = self.current_menu().menu_items.len() - 1;
                    } else {
                        self.item_on -= 1;
                    }

                    //TODO: Play sound!

                    if self.current_menu_item().status() != -1 {
                        break;
                    }
                }
            }
            VirtualKeyCode::Return => {
                if self.current_menu_item().status() != 0 {
                    self.current_menu_mut().last_on = self.item_on;

                    if let Some(routine) = self.current_menu_item().routine() {
                        if self.current_menu_item().status() == 2 {
                            routine(self, 1); // right arrow
                            // TODO Play sound!
                        } else {
                            routine(self, self.item_on as i16);
                            // TODO Play sound!
                        }
                    }
                }
            }
            VirtualKeyCode::Escape => {
                self.current_menu_mut().last_on = self.item_on;
                self.hide();
            }
            _ => return false,
        }
        true
    }
}