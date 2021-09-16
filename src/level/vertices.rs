use crate::types::{DoomRealNum, real};
use crate::wad::{LumpStore, By, LumpNumber};
use std::mem::size_of;
use std::io::Seek;
use serde::Deserialize;

#[derive(Deserialize)]
struct VertexRaw {
    x: i16,
    y: i16
}

pub struct Vertex {
    x: DoomRealNum,
    y: DoomRealNum
}

pub fn load(lumps: &LumpStore, map_lump: LumpNumber) -> Vec<Vertex> {
    let mut data = lumps.get_lump_cursor(By::Number(map_lump.offset(4)));

    let vertex_count = data.stream_len().unwrap() as usize / size_of::<VertexRaw>();
    let mut vertices = Vec::new();

    for _ in 0..vertex_count {
        let raw_vertex: VertexRaw = bincode::deserialize_from(&mut data).unwrap();

        vertices.push(Vertex {
            x: real(raw_vertex.x),
            y: real(raw_vertex.y),
        });
    }

    vertices
}