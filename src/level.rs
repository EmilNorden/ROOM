mod nodes;
mod segs;
pub mod blockmap;
pub mod vertices;
pub mod sectors;

use crate::wad::{LumpStore, By};
use crate::level::nodes::{Node, load_nodes};
use crate::level::blockmap::{Blockmap};

pub struct Level {
    nodes: Vec<Node>,
}

pub fn load(lumps: &LumpStore, episode: u32, map: u32) -> Level {
    /*let lump_num = lumps.get_lump_number(&format!("map{:02}", map)).unwrap();

    Level {
        blockmap: load_blockmap(lumps.get_lump(By::Number(lump_num + 10))),
        nodes: load_nodes(lumps.get_lump(By::Number(lump_num + 7)))
    }*/
    todo!();
}