use crate::menu::MenuComponent;
use crate::level_component::LevelComponent;
use crate::game_context::GameState::Level;
use crate::events::EventSystem;
use crate::rendering::renderer::Renderer;
use crate::system::System;
use crate::wad::{LumpStore, By};
use crate::page_component::PageComponent;

const MAX_NODES:usize = 8;
const BACKUPTICKS:i32 = 12;

pub struct GameContext {
    pub(crate) state: GameState,
    pub(crate) mode: GameMode,
    pub(crate) action: GameAction,
    pub(crate) old_enter_tics: i32,
    pub(crate) net_tics: [i32; MAX_NODES],
    pub(crate) node_in_game: [bool; MAX_NODES],
    pub(crate) game_tic: i32,
    pub(crate) game_time: i32,
    pub(crate) skip_tics: i32,
    pub(crate) make_tic: i32,

    pub(crate) menu: MenuComponent,
    pub(crate) level: LevelComponent,
    pub(crate) page: PageComponent,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            state: GameState::DemoScreen,
            mode: GameMode::Commercial, // TODO Hard coded for now
            action: GameAction::Nothing,
            old_enter_tics: 0,
            net_tics: [0i32; MAX_NODES],
            node_in_game: [false; MAX_NODES],
            game_tic: 0,
            game_time: 0,
            skip_tics: 0,
            make_tic: 0,
            menu: MenuComponent::new(),
            level: LevelComponent::new(),
            page: PageComponent::new(),
        }
    }

    pub fn game_loop(&mut self, events: &mut EventSystem, renderer: &mut dyn Renderer, system: &System, lumps: &LumpStore) {
        let map = 1;
        /*let map_lump_num = lumps.get_lump_number(&format!("map{:02}", map)).unwrap();

        let blockmap = crate::level::blockmap::load(lumps, map_lump_num);
        let vertices = crate::level::vertices::load(lumps, map_lump_num);
    */

        self.try_run_tics(system);

        // Below are contents of D_Display
        match self.state {
            GameState::ForceWipe => {}
            GameState::Level => {}
            GameState::Intermission => {}
            GameState::Finale => {}
            GameState::DemoScreen => {
                // D_PageDrawer
                self.page.draw(renderer, lumps);
            }
        }

        // TODO S_UpdateSounds(players[consoleplayer].mo);// move positional sounds

        self.menu.draw(renderer, lumps);

        events.process_events(&mut [
            &mut self.menu,
            &mut self.page,
            &mut self.level
        ]);

        if self.page.open_menu_requested() {
            self.menu.show();
        }

    }

    fn try_run_tics(&mut self, system: &System) {
        let ticdup = 1; // TODO: ticdup comes from "doomcom"
        let enter_tic = system.calculate_tics() as i32 / ticdup;
        let real_tics = enter_tic - self.old_enter_tics;
        self.old_enter_tics = enter_tic;

        // TODO: Skipping NetUpdate() for now

        let mut low_tic = i32::MAX;
        let mut num_playing = 0;
        for i in 0..1 { // TODO: Should be 0..doomcom->numnodes
            if true {
                // if game_context.node_in_game[i] {
                num_playing += 1;
                if self.net_tics[i] < low_tic {
                    low_tic = self.net_tics[i];
                }
            }
        }
        let available_tics = low_tic - self.game_tic / ticdup;

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

        while low_tic < self.game_tic / ticdup + counts {
            self.net_update(system);
            low_tic = i32::MAX;

            for i in 0..1 { // should be 0..num_nodes
                if self.node_in_game[i] && self.net_tics[i] < low_tic {
                    low_tic = self.net_tics[i];
                }
            }

            if low_tic < self.game_tic / ticdup {
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
                if self.game_tic / ticdup > low_tic {
                    panic!("gametic>lowtic")
                }

                if self.page.demo_state().advance_demo {
                    self.page.advance_demo(&self.mode)
                }

                // TODO:
                // M_Ticker
                // TODO G_Ticker is required to get any gameplay on screen
                //game_ticker( demo_state);
                self.game_tic += 1;

                // modify command for duplicated tics
                if i != ticdup -1 {
                    // TODO Fill this in
                }
            }
            self.net_update(system);
        }
    }

    fn net_update(&mut self, system: &System) {
        let ticdup = 1;
        let nowtime = system.calculate_tics() as i32 / ticdup;
        let mut newtics = nowtime - self.game_time;
        self.game_time = nowtime;

        if newtics <= 0 {
            // TODO GetPackets
            return;
        }

        if self.skip_tics <= newtics {
            newtics -= self.skip_tics;
            self.skip_tics = 0;
        }
        else {
            self.skip_tics -= newtics;
            newtics = 0;
        }

        // TODO netbuffer->player = consoleplayer;

        // build new ticcmds for console player
        let gameticdiv = self.game_tic / ticdup;
        for i in 0..newtics {
            // I_StartTic
            // D_ProcessEvents
            if self.game_tic - gameticdiv >= BACKUPTICKS / 2 - 1 {
                break; // can't hold any more
            }

            // G_BuildTiccmd(&localcmds[maketic % BACKUPTICS]);
            self.make_tic += 1;
        }

        /*
        TODO
        if singletics
            return;

         */

        // TODO: send the packet to the other nodes

        // TODO: GetPackets
    }
}

pub enum GameMode {
    Shareware,
    Registered,
    Commercial,
    Retail,
    Indetermined,
}

pub enum GameState {
    ForceWipe,
    Level,
    Intermission,
    Finale,
    DemoScreen,
}

#[derive(PartialOrd, PartialEq)]
pub enum GameAction {
    Nothing,
    LoadLevel,
    NewGame,
    LoadGame,
    SaveGame,
    PlayDemo,
    Completed,
    Victory,
    WorldDone,
    Screenshot,
}

pub struct DemoState {
    pub(crate) advance_demo: bool,
    pub(crate) demo_sequence: i32,
    pub(crate) page_name: Option<String>,
    pub(crate) page_tic: i32,
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

    pub fn page_name(&self) -> Option<&String> {
        self.page_name.as_ref()
    }
}