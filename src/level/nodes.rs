use crate::types::{DoomRealNum, real};
use crate::wad::LumpStore;
use std::mem::size_of;
use std::io::Cursor;
use fixed::{FixedI32, FixedU32};
use fixed::types::{U16F16, I16F16, U32F0};
use serde::Deserialize;

#[derive(Deserialize)]
struct NodeRaw {
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
    bbox: [[DoomRealNum; 4]; 2],

    // If NF_SUBSECTOR its a subsector.
    children: [usize; 2],
}

pub fn load_nodes(data: &[u8]) -> Vec<Node> {
    let num_nodes = data.len() / size_of::<NodeRaw>();

    let mut nodes = Vec::new();
    let mut nodes_cursor = Cursor::new(data);
    for i in 0..num_nodes {
        let raw_node: NodeRaw = bincode::deserialize_from(&mut nodes_cursor)
            .expect("Load Nodes failed");

        let node = Node {
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
        };
        nodes.push(node);
    }
    nodes
}