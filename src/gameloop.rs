use crate::wad::{LumpStore, By};
use crate::system::System;
use crate::game_context::{GameContext, GameMode, GameState};
use crate::rendering::renderer::{Renderer};
use crate::menu::MenuComponent;
use crate::events::{EventSystem, EventConsumer};

const BACKUPTICKS:i32 = 12;

pub fn game_loop(events: &mut EventSystem, renderer: &mut dyn Renderer, system: &System, lumps: &LumpStore, game_context: &mut GameContext, demo_state: &mut DemoState) {
    let map = 1;
    /*let map_lump_num = lumps.get_lump_number(&format!("map{:02}", map)).unwrap();

    let blockmap = crate::level::blockmap::load(lumps, map_lump_num);
    let vertices = crate::level::vertices::load(lumps, map_lump_num);
*/

    try_run_tics(system, game_context, demo_state);

    // Below are contents of D_Display
    match game_context.state {
        GameState::ForceWipe => {}
        GameState::Level => {}
        GameState::Intermission => {}
        GameState::Finale => {}
        GameState::DemoScreen => {
            // D_PageDrawer
            if demo_state.page_name.is_none() {
                return;
            }
            renderer.draw_patch(
                0,
                0,
                0,
                &lumps.get_lump(By::Name(demo_state.page_name.as_ref().unwrap())).into())
        }
    }

    // TODO S_UpdateSounds(players[consoleplayer].mo);// move positional sounds

    let mut mc = MenuComponent::new();
    mc.draw(renderer, lumps);

    let mut consumers = Vec::<&mut dyn EventConsumer>::new();
    consumers.push(&mut mc);
    events.process_events(&consumers);
}


fn try_run_tics(system: &System, game_context: &mut GameContext, demo_state: &mut DemoState) {
    let ticdup = 1; // TODO: ticdup comes from "doomcom"
    let enter_tic = system.calculate_tics() as i32 / ticdup;
    let real_tics = enter_tic - game_context.old_enter_tics;
    game_context.old_enter_tics = enter_tic;

    // TODO: Skipping NetUpdate() for now

    let mut low_tic = i32::MAX;
    let mut num_playing = 0;
    for i in 0..1 { // TODO: Should be 0..doomcom->numnodes
        if true {
        // if game_context.node_in_game[i] {
            num_playing += 1;
            if game_context.net_tics[i] < low_tic {
                low_tic = game_context.net_tics[i];
            }
        }
    }
    let available_tics = low_tic - game_context.game_tic / ticdup;

    // decide how many tics to run
    let mut counts = if real_tics < available_tics - 1 {
        real_tics + 1
    } else if real_tics < available_tics {
        real_tics
    } else {
        available_tics
    };

    if counts < 1 {
        counts = 1;
    }

    // frameon++ TODO Skip this

    // TODO Skipping conditional statement !demoplayback

    while low_tic < game_context.game_tic / ticdup + counts {
        net_update(system, game_context);
        low_tic = i32::MAX;

        for i in 0..1 { // should be 0..num_nodes
            if game_context.node_in_game[i] && game_context.net_tics[i] < low_tic {
                low_tic = game_context.net_tics[i];
            }
        }

        if low_tic < game_context.game_tic / ticdup {
            panic!("try_run_tics: lowtic < gametic");
        }

        // don't stay in here forever -- give the menu a chance to work
        if system.calculate_tics() as i32 / ticdup - enter_tic >= 20 {
            // TODO M_Ticker
            return;
        }
    }

    // run the count * ticdup dics
    while counts > 0 {
        counts -= 1;

        for i in 0..ticdup {
            if game_context.game_tic / ticdup > low_tic {
                panic!("gametic>lowtic")
            }

            if demo_state.advance_demo {
                do_advance_demo(game_context, demo_state);
            }



            // TODO:
            // M_Ticker
            // TODO G_Ticker is required to get any gameplay on screen
            game_ticker(game_context, demo_state);
            game_context.game_tic += 1;

            // modify command for duplicated tics
            if i != ticdup -1 {
                // TODO Fill this in
            }
        }
        net_update(system, game_context);
    }

}

fn net_update(system: &System, game_context: &mut GameContext) {
    let ticdup = 1;
    let nowtime = system.calculate_tics() as i32 / ticdup;
    let mut newtics = nowtime - game_context.game_time;
    game_context.game_time = nowtime;

    if newtics <= 0 {
        // TODO GetPackets
        return;
    }

    if game_context.skip_tics <= newtics {
        newtics -= game_context.skip_tics;
        game_context.skip_tics = 0;
    }
    else {
        game_context.skip_tics -= newtics;
        newtics = 0;
    }

    // TODO netbuffer->player = consoleplayer;

    // build new ticcmds for console player
    let gameticdiv = game_context.game_tic / ticdup;
    for i in 0..newtics {
        // I_StartTic
        // D_ProcessEvents
        if game_context.game_tic - gameticdiv >= BACKUPTICKS / 2 - 1 {
            break; // can't hold any more
        }

        // G_BuildTiccmd(&localcmds[maketic % BACKUPTICS]);
        game_context.make_tic += 1;
    }

    /*
    TODO
    if singletics
        return;

     */

    // TODO: send the packet to the other nodes

    // TODO: GetPackets
}

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

pub struct DemoState {
    advance_demo: bool,
    demo_sequence: i32,
    page_name: Option<String>,
    page_tic: i32,
}

impl DemoState {
    pub fn new() -> Self {
        Self {
            advance_demo: true, // Should be false and set to true in D_StartTitle, but seems redundant
            demo_sequence: -1,
            page_name: None,
            page_tic: 0
        }
    }
}

fn page_ticker(state: &mut DemoState) {
    state.page_tic -= 1;
    if state.page_tic < 0 {
        state.advance_demo = true;
    }
}

fn do_advance_demo(game_context: &GameContext, state: &mut DemoState) {
    state.advance_demo = false;
    let max_sequence = match game_context.mode {
        GameMode::Retail => 7i32,
        _ => 6i32
    };

    state.demo_sequence = (state.demo_sequence + 1) % max_sequence;

    match state.demo_sequence {
        0 => {
            state.page_tic = match game_context.mode {
                GameMode::Commercial => 35 * 11,
                _ => 170,
            };
            // TODO gamestate = GS_DEMOSCREEN
            state.page_name = Some("TITLEPIC".to_string());
            match game_context.mode {
                GameMode::Commercial => {
                    // TODO S_StartMusic(mus_dm2ttl);
                }
                _ => {
                    // S_StartMusic(mus_intro);
                }
            }
        },
        2 => {
            state.page_tic = 200;
            state.page_name = Some("CREDIT".to_string());
        }
        _ => {
            println!("Demosequences not handled");
        }
    }
}