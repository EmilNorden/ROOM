use crate::types::{DoomRealNum, real};
use crate::wad::LumpStore;
use std::mem::size_of;
use std::io::Cursor;
use fixed::{FixedI32, FixedU32};
use fixed::types::{U16F16, I16F16, U32F0};
use serde::Deserialize;
use crate::level::parse_entity_vector;
use crate::level::bounding_box::BoundingBox;

#[derive(Deserialize)]
struct RawNode {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,

    bbox: [[i16; 4]; 2],

    children: [u16; 2],
}

pub struct Node {
    // Partition line
    x: DoomRealNum,
    y: DoomRealNum,
    dx: DoomRealNum,
    dy: DoomRealNum,

    // Bounding box for each child
    bbox: [BoundingBox; 2],

    // If NF_SUBSECTOR its a subsector.
    children: [usize; 2],
}

impl Node {
    pub fn x(&self) -> DoomRealNum { self. x }
    pub fn y(&self) -> DoomRealNum { self. y }
    pub fn dx(&self) -> DoomRealNum { self.dx }
    pub fn dy(&self) -> DoomRealNum { self.dy }
    pub fn children(&self) -> &[usize; 2] { & self.children }
}

pub fn load(data: &[u8]) -> Vec<Node> {
    parse_entity_vector(data, |raw_node: RawNode| Node {
        x: real(raw_node.x),
        y: real(raw_node.y),
        dx: real(raw_node.dx),
        dy: real(raw_node.dy),
        bbox: [
            [
                real(raw_node.bbox[0][0]),
                real(raw_node.bbox[0][1]),
                real(raw_node.bbox[0][2]),
                real(raw_node.bbox[0][3]),
            ],
            [
                real(raw_node.bbox[1][0]),
                real(raw_node.bbox[1][1]),
                real(raw_node.bbox[1][2]),
                real(raw_node.bbox[1][3]),
            ]
        ],
        children: [
            raw_node.children[0] as usize,
            raw_node.children[1] as usize,
        ]
    })
}