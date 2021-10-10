use crate::events::{Event, EventConsumer, ConsumeResult};
use crate::level::Level;
use crate::game_context::{Skill, GameAction};
use crate::random::{PRNG, create_random_generator};
use crate::wad::LumpStore;
use crate::graphics::textures::{TextureData, init_textures};
use crate::graphics::flats::{FlatData, init_flats};
use crate::map_object::{PlayerState, Player};

pub const MAX_PLAYERS: usize = 4;

pub struct GameComponent<'a> {
    textures: TextureData,
    flats: FlatData,
    loaded_level: Option<Level<'a>>,
    action: GameAction,
    demo_playback: bool,
    net_demo: bool,
    net_game: bool,
    deathmatch: bool,
    respawnparm: bool,
    respawn_monsters: bool,
    paused: bool,
    random: Box<dyn PRNG>,
    players: [Player; MAX_PLAYERS],
}

impl GameComponent<'_> {
    pub fn new(lumps: &LumpStore) -> Self {
        Self {
            textures: init_textures(lumps),
            flats: init_flats(lumps),
            loaded_level: None,
            action: GameAction::Nothing,

            demo_playback: false,
            net_demo: false,
            net_game: false,
            deathmatch: false,
            respawnparm: false,
            paused: false,
            random: create_random_generator(),
            players: [Player::default(); 4],
            respawn_monsters: false,
        }
    }

    pub fn new_game(&mut self, skill: Skill, episode: i32, map: i32) {
        self.action = GameAction::NewGame { skill, episode, map };
    }

    // G_Ticker (ish)
    pub fn tick(&mut self, game_tics: i32, lumps: &LumpStore) {
        // TODO: Player reborn logic

        while self.action != GameAction::Nothing {
            match self.action {
                GameAction::Nothing => {}
                GameAction::NewGame { skill, episode, map } => {
                    self.demo_playback = false;
                    self.net_demo = false;
                    self.deathmatch = false;
                    self.respawnparm = false;
                    self.init_new(skill, game_tics, episode, map, lumps);
                    self.action = GameAction::Nothing;
                }
                _ => println!("Unhandled action in GameComponent::ticker")
            }
        }
    }

    fn init_new(&mut self, skill: Skill, game_tics: i32, mut episode: i32, mut map: i32, lumps: &LumpStore) {
        if self.paused {
            self.paused = false;
            // TODO S_ResumeSound
        }

        if episode < 1 { // TODO Why not make episode unsigned?
            episode = 1
        }

        if map < 1 { // TODO Same here
            map = 1;
        }

        self.random.reset();

        self.respawn_monsters = skill == Skill::Nightmare; // TODO: Implement respawnparam, look at original

        for player in &mut self.players {
            player.state = PlayerState::Reborn
        }

        /*
            usergame = true;                // will be set false if a demo
    paused = false;
    demoplayback = false;
    automapactive = false;
    viewactive = true;
    gameepisode = episode;
    gamemap = map;
    gameskill = skill;

    viewactive = true;
         */

        self.loaded_level = Some(Level::load(lumps, &self.textures, &self.flats, game_tics, episode, map));
    }
}

impl EventConsumer for GameComponent<'_> {
    // G_Responder (ish)
    fn consume(&mut self, event: &Event) -> ConsumeResult {
        if self.loaded_level.is_none() {
            return ConsumeResult::NotHandled;
        }

        ConsumeResult::NotHandled
    }
}