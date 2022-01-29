use crate::wad::{LumpStore, By, LumpNumber};
use std::mem::size_of;
use std::io::Seek;
use serde::Deserialize;
use crate::level::parse_entity_vector;
use crate::number::RealNumber;
use crate::rendering::types::Point2D;

#[derive(Deserialize)]
struct RawVertex {
    x: i16,
    y: i16
}

pub fn load(data: &[u8]) -> Vec<Point2D> {
    parse_entity_vector(data, |raw_vertex: RawVertex| Point2D::new(RealNumber::new(raw_vertex.x), RealNumber::new(raw_vertex.y)))
}