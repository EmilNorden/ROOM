use crate::events::{EventConsumer, Event};
use crate::game_context::{DemoState, GameMode};
use crate::rendering::renderer::Renderer;
use crate::wad::{LumpStore, By};

pub struct PageComponent {
    demo_state: DemoState,
    open_menu_requested: bool,
}

impl PageComponent {
    pub fn new() -> Self {
        Self {
            demo_state: DemoState::new(),
            open_menu_requested: false,
        }
    }

    pub fn demo_state(&self) -> &DemoState { &self.demo_state }
    pub fn demo_state_mut(&mut self) -> &mut DemoState { &mut self.demo_state }

    pub fn open_menu_requested(&mut self) -> bool {
        let result = self.open_menu_requested;
        self.open_menu_requested = false;

        result
    }

    pub fn draw(&self, renderer: &mut dyn Renderer, lumps: &LumpStore) {
        if self.demo_state.page_name().is_none() {
            return;
        }
        renderer.draw_patch(
            0,
            0,
            0,
            &lumps.get_lump(By::Name(self.demo_state.page_name().unwrap())).into())
    }

    pub fn advance_demo(&mut self, mode: &GameMode) {
        self.demo_state.advance_demo = false;
        let max_sequence = match mode {
            GameMode::Retail => 7i32,
            _ => 6i32
        };

        self.demo_state.demo_sequence = (self.demo_state.demo_sequence + 1) % max_sequence;

        match self.demo_state.demo_sequence {
            0 => {
                self.demo_state.page_tic = match mode {
                    GameMode::Commercial => 35 * 11,
                    _ => 170,
                };
                // TODO gamestate = GS_DEMOSCREEN
                self.demo_state.page_name = Some("TITLEPIC".to_string());
                match mode {
                    GameMode::Commercial => {
                        // TODO S_StartMusic(mus_dm2ttl);
                    }
                    _ => {
                        // S_StartMusic(mus_intro);
                    }
                }
            },
            2 => {
                self.demo_state.page_tic = 200;
                self.demo_state.page_name = Some("CREDIT".to_string());
            }
            _ => {
                println!("Demosequences not handled");
            }
        }
    }
}

impl EventConsumer for PageComponent {
    fn consume(&mut self, event: &Event) -> bool {
        match event {
            Event::KeyDown { .. } => {
                self.open_menu_requested = true;
                // Dont consume the event
            },
            _ => {}
        }

        false
    }
}