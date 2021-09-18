use crate::wad::{LumpStore, By};
use crate::system::System;
use crate::game_context::{GameContext, GameMode, GameState};
use crate::rendering::renderer::{Renderer};
use crate::menu::MenuComponent;
use crate::events::{EventSystem, EventConsumer};

/*
fn game_ticker(context: &GameContext, demo_state: &mut DemoState) {
    let ticdup = 1;
    // do player reborns if needed
    // TODO Add this later

    // do things to change the game state
    // TODO Add this later

    let buf = context.game_tic / ticdup % BACKUPTICKS;
    // TODO Add this loop later

    match context.state {
        GameState::ForceWipe => {}
        GameState::Level => {}
        GameState::Intermission => {}
        GameState::Finale => {}
        GameState::DemoScreen => {
            page_ticker(demo_state);
        }
    }
}

fn page_ticker(state: &mut DemoState) {
    state.page_tic -= 1;
    if state.page_tic < 0 {
        state.advance_demo = true;
    }
}


*/
