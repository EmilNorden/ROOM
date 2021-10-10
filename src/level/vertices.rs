use crate::types::{DoomRealNum, real};
use crate::wad::{LumpStore, By, LumpNumber};
use std::mem::size_of;
use std::io::Seek;
use serde::Deserialize;
use crate::level::parse_entity_vector;

#[derive(Deserialize)]
struct RawVertex {
    x: i16,
    y: i16
}

#[derive(Clone)]
pub struct Vertex {
    pub(crate) x: DoomRealNum,
    pub(crate) y: DoomRealNum
}

pub fn load(data: &[u8]) -> Vec<Vertex> {
    parse_entity_vector(data, |raw_vertex: RawVertex| Vertex {
        x: real(raw_vertex.x),
        y: real(raw_vertex.y),
    })
}