use std::ops::Index;
use crate::number::RealNumber;
use crate::rendering::types::Point2D;

pub struct BoundingBox {
    coordinates: [RealNumber; 4],
}

impl BoundingBox {
    const LEFT_INDEX: usize = 2;
    const RIGHT_INDEX: usize = 3;
    const TOP_INDEX: usize = 0;
    const BOTTOM_INDEX: usize = 1;

    pub fn left(&self) -> RealNumber { self.coordinates[Self::LEFT_INDEX] }
    pub fn right(&self) -> RealNumber { self.coordinates[Self::RIGHT_INDEX] }
    pub fn top(&self) -> RealNumber { self.coordinates[Self::TOP_INDEX] }
    pub fn bottom(&self) -> RealNumber { self.coordinates[Self::BOTTOM_INDEX] }

    pub fn from_vertices(v1: &Point2D, v2: &Point2D) -> Self {
        Self {
            coordinates: [
                v1.y.max(v2.y),
                v1.y.min(v2.y),
                v1.x.min(v2.x),
                v1.x.max(v2.x)
            ]
        }
    }

    pub fn new(left: RealNumber, right: RealNumber, top: RealNumber, bottom: RealNumber) -> Self {
        Self {
            coordinates: [
                top,
                bottom,
                left,
                right
            ]
        }
    }

    pub fn new_empty() -> Self {
        Self::new(
            RealNumber::new(i16::MAX),
            RealNumber::new(i16::MIN),
            RealNumber::new(i16::MIN),
            RealNumber::new(i16::MAX),
        )
    }

    pub fn expand(&mut self, vertex: &Point2D) {
        if vertex.x < self.coordinates[Self::LEFT_INDEX] {
            self.coordinates[Self::LEFT_INDEX] = vertex.x;
        }

        if vertex.x > self.coordinates[Self::RIGHT_INDEX] {
            self.coordinates[Self::RIGHT_INDEX] = vertex.x;
        }

        if vertex.y < self.coordinates[Self::BOTTOM_INDEX] {
            self.coordinates[Self::BOTTOM_INDEX] = vertex.y;
        }

        if vertex.y > self.coordinates[Self::TOP_INDEX] {
            self.coordinates[Self::TOP_INDEX] = vertex.y;
        }
    }
}

impl Index<usize> for BoundingBox {
    type Output = RealNumber;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}