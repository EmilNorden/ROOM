use crate::types::DoomRealNum;
use crate::level::vertices::Vertex;

pub struct BoundingBox {
    left: DoomRealNum,
    right: DoomRealNum,
    top: DoomRealNum,
    bottom: DoomRealNum
}

impl BoundingBox {
    pub fn from_vertices(v1: &Vertex, v2: &Vertex) -> Self {
        Self {
            left: v1.x.min(v2.x),
            right: v1.x.max(v2.x),
            bottom: v1.y.min(v2.y),
            top: v1.y.max(v2.y),
        }
    }

    pub fn new(left: DoomRealNum, right: DoomRealNum, top: DoomRealNum, bottom: DoomRealNum) -> Self {
        Self {
            left,
            right,
            top,
            bottom
        }
    }
}