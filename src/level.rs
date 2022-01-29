pub mod nodes;
pub mod segs;
pub mod blockmap;
pub mod vertices;
pub mod sectors;
mod sidedefs;
mod linedefs;
pub mod bounding_box;
mod sub_sectors;

use crate::wad::{LumpStore, By};
use crate::level::blockmap::{Blockmap};
use crate::graphics::textures::{TextureNumber, TextureData};
use crate::graphics::flats::{FlatData, FlatNumber};
use crate::game_component::MAX_PLAYERS;
use crate::level::sectors::Sector;
use crate::level::segs::Seg;
use std::mem::size_of;
use std::io::Cursor;
use crate::constants::{MAP_BLOCK_SHIFT, MAX_RADIUS};
use crate::level::sidedefs::SideDef;
use crate::level::linedefs::LineDef;
use crate::level::sub_sectors::SubSector;
use crate::level::nodes::Node;
use crate::level::bounding_box::BoundingBox;
use crate::number::RealNumber;
use crate::rendering::types::Point2D;
use crate::types::{Vector3};

pub struct Level {
    level_start_tic: i32,
    sky_texture: TextureNumber,
    player_in_game: [bool; MAX_PLAYERS],
    blockmap: Blockmap,
    vertices: Vec<Point2D>,
    sectors: Vec<Sector>,
    side_defs: Vec<SideDef>,
    line_defs: Vec<LineDef>,
    sub_sectors: Vec<SubSector>,
    nodes: Vec<Node>,
    segs: Vec<Seg>,
    sky_flat_number: FlatNumber,
}

impl Level {
    pub fn nodes(&self) -> &Vec<Node> { &self.nodes }
    pub fn sub_sectors(&self) -> &Vec<SubSector> { &self.sub_sectors }
    pub fn sectors(&self) -> &Vec<Sector> { &self.sectors }
    pub fn segs(&self) -> &Vec<Seg> { &self.segs }
    pub fn vertices(&self) -> &Vec<Point2D> { &self.vertices }
    pub fn side_defs(&self) -> &Vec<SideDef> { &self.side_defs }
    pub fn line_defs(&self) -> &Vec<LineDef> { &self.line_defs }
    pub fn sky_flat_number(&self) -> FlatNumber { self.sky_flat_number }

    pub fn load(lumps: &LumpStore, textures: &TextureData, flats: &FlatData, game_tics: i32, episode: i32, map: i32) -> Self {
        // TODO: Different game modes uses different sky textures.
        // TODO Hard code for commercial for now

        let sky_flat_number = flats.get_flat_number("F_SKY1", lumps).unwrap();

        let sky_texture = match map {
            0..=11 => textures.get_texture_number("SKY1"),
            12..=20 => textures.get_texture_number("SKY2"),
            _ => textures.get_texture_number("SKY3")
        }.unwrap();

        /*if (wipegamestate == GS_LEVEL)
            wipegamestate = GS_FORCE_WIPE;             // force a wipe

        gamestate = GS_LEVEL;*/


        // TODO BELOW
        /*
            for (i = 0; i < MAXPLAYERS; i++) {
        if (playeringame[i] && players[i].playerstate == PST_DEAD)
            players[i].playerstate = PST_REBORN;
        memset(players[i].frags, 0, sizeof(players[i].frags));
    }
    */
        // BELOW is P_SetupLevel
        // totalkills = totalitems = totalsecret = wminfo.maxfrags = 0;

        let lump_num = lumps.get_lump_number(&format!("map{:02}", map)).unwrap();

        let blockmap = blockmap::load(lumps.get_lump(By::Number(lump_num.offset(10))));
        let vertices = vertices::load(lumps.get_lump(By::Number(lump_num.offset(4))));
        let mut sectors = sectors::load(lumps.get_lump(By::Number(lump_num.offset(8))));
        let side_defs = sidedefs::load(lumps.get_lump(By::Number(lump_num.offset(3))), textures);
        let line_defs = linedefs::load(lumps.get_lump(By::Number(lump_num.offset(2))), &vertices);
        // TODO In the original code segs is loaded last. Im changing that so sub_sectors can be initialized properly.
        let segs = segs::load(lumps.get_lump(By::Number(lump_num.offset(5))));
        let sub_sectors = sub_sectors::load(lumps.get_lump(By::Number(lump_num.offset(6))), &segs, &side_defs);
        let nodes = nodes::load(lumps.get_lump(By::Number(lump_num.offset(7))));


        // TODO rejectmatrix - is it used?

        // P_GroupLines

        // Count number of lines in each sector
        let mut total = 0;
        let line_count = line_defs.len();
        for line in &line_defs {
            total += 1;

            if total == 97 {
                println!("asdasda");
            }

            let front_sector_index = side_defs[line.front_side_index].sector_index;
            sectors[front_sector_index].line_count += 1;

            if let Some(back_side_index) = line.back_side_index {
                let back_sector_index = side_defs[back_side_index].sector_index;
                if back_sector_index != front_sector_index {
                    sectors[back_sector_index].line_count += 1;
                    total += 1;
                }
            }
        }

        // build line tables for each sector

        for (sector_index, sector) in sectors.iter_mut().enumerate() {
            let mut line_buffer = Vec::new();
            let mut bounds = BoundingBox::new_empty();

            for line in &line_defs {
                if line.is_adjacent_to_sector_index(sector_index, &side_defs) {
                    line_buffer.push(line);
                    bounds.expand(&line.v1);
                    bounds.expand(&line.v2);
                    // Should I store
                    // sector.lines.push(line.clone());
                }
            }

            if line_buffer.len() != sector.line_count as usize {
                panic!("P_GroupLines: miscounted");
            }

            // set the degenmobj_t to the middle of the bounding box
            sector.soundorg = Vector3::new(
                (bounds.right() + bounds.left()) / RealNumber::new(2), // TODO: Will this really work? I really just want to divide bits by 2
                (bounds.top() + bounds.bottom()) / RealNumber::new(2),
                RealNumber::new(0),
            );

            // adjust bounding box to map blocks
            let mut block_top = (bounds.top() - blockmap.origin_y() + RealNumber::new(MAX_RADIUS)) >> MAP_BLOCK_SHIFT;
            block_top = if block_top >= blockmap.height() { blockmap.height() - RealNumber::new_from_bits(1) } else { block_top };

            let mut block_bottom = (bounds.bottom() - blockmap.origin_y() - RealNumber::new(MAX_RADIUS)) >> MAP_BLOCK_SHIFT;
            block_bottom = if block_bottom < RealNumber::new(0) { RealNumber::new(0) } else { block_bottom };

            let mut block_right = (bounds.right() - blockmap.origin_x() + RealNumber::new(MAX_RADIUS)) >> MAP_BLOCK_SHIFT;
            block_right = if block_right >= blockmap.width() { blockmap.width() - RealNumber::new_from_bits(1) } else { block_right };

            let mut block_left = (bounds.left() - blockmap.origin_x() - RealNumber::new(MAX_RADIUS)) >> MAP_BLOCK_SHIFT;
            block_left = if block_left < RealNumber::new(0) { RealNumber::new(0) } else { block_left };

            sector.blockbox = BoundingBox::new(block_left, block_right, block_top, block_bottom);
        }
        /*Self {
            level_start_tic: game_tics,
            sky_texture,
            nodes: Vec::new(),
        }*/
        
        Self {
            level_start_tic: game_tics,
            sky_texture,
            player_in_game: [false; MAX_PLAYERS],
            blockmap,
            vertices,
            sectors,
            side_defs,
            line_defs,
            sub_sectors,
            nodes,
            segs,
            sky_flat_number,
        }
    }
}

pub fn load(lumps: &LumpStore, episode: u32, map: u32) -> Level {
    /*let lump_num = lumps.get_lump_number(&format!("map{:02}", map)).unwrap();

    Level {
        blockmap: load_blockmap(lumps.get_lump(By::Number(lump_num + 10))),
        nodes: load_nodes(lumps.get_lump(By::Number(lump_num + 7)))
    }*/
    todo!();
}

fn parse_entity_vector<TIn, TOut, F: Fn(TIn) -> TOut>(data: &[u8], factory: F) -> Vec<TOut>
    where TIn: for<'de> serde::Deserialize<'de> {
    let num_entities = data.len() / size_of::<TIn>();

    let mut result = Vec::new();
    let mut cursor = Cursor::new(data);
    for _ in 0..num_entities {
        let raw_entity: TIn = bincode::deserialize_from(&mut cursor).unwrap();
        result.push(factory(raw_entity));
    }

    result
}