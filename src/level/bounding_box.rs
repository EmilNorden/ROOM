use std::ops::Index;
use crate::types::DoomRealNum;
use crate::level::vertices::Vertex;

pub struct BoundingBox {
    coordinates: [DoomRealNum; 4],
}

impl BoundingBox {
    pub fn left(&self) -> DoomRealNum { self.coordinates[2] }
    pub fn right(&self) -> DoomRealNum { self.coordinates[3] }
    pub fn top(&self) -> DoomRealNum { self.coordinates[0] }
    pub fn bottom(&self) -> DoomRealNum { self.coordinates[1] }
}

impl BoundingBox {
    pub fn from_vertices(v1: &Vertex, v2: &Vertex) -> Self {
        Self {
            coordinates: [
                v1.y.max(v2.y),
                v1.y.min(v2.y),
                v1.x.min(v2.x),
                v1.x.max(v2.x)
            ]
        }
    }

    pub fn new(left: DoomRealNum, right: DoomRealNum, top: DoomRealNum, bottom: DoomRealNum) -> Self {
        Self {
            coordinates: [
                top,
                bottom,
                left,
                right
            ]
        }
    }
}

impl Index<usize> for BoundingBox {
    type Output = DoomRealNum;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}