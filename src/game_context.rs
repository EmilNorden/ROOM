const MAX_NODES:usize = 8;

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
        }
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