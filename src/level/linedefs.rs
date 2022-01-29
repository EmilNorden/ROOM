use crate::level::parse_entity_vector;
use serde::Deserialize;
use crate::level::bounding_box::BoundingBox;
use crate::level::sidedefs::SideDef;
use crate::number::RealNumber;
use crate::rendering::types::Point2D;

pub enum SlopeType {
    Horizontal,
    Vertical,
    Positive,
    Negative,
}

#[derive(Deserialize)]
struct RawLineDef {
    v1: i16,
    v2: i16,
    flags: i16,
    special: i16,
    tag: i16,
    side_num: [i16; 2],
}

pub struct LineDef {
    /* Original Doom used pointers to vertices, but since I'm struggling a bit with the references
     that leaves me with two options: Store indices into the vertices array, or copies of the actual vertices.
     On 64-bit machines usize is 8 bytes, the same size as a Vertex (2x4bytes), and since the
     vertex data doesn't change after the level is loaded, it should be fine to just store copies of the vertices*/
    pub(crate) v1: Point2D,
    pub(crate) v2: Point2D,

    // Precalculated v2 - v1 for side checking.
    pub(crate) dx: RealNumber,
    pub(crate) dy: RealNumber,

    // Animation related
    pub(crate) flags: i16,
    pub(crate) special: i16,
    pub(crate) tag: i16,

    // Visual appearance: SideDefs.
    //  sidenum[1] will be -1 if one sided
    pub(crate) front_side_index: usize,
    pub(crate) back_side_index: Option<usize>,

    // Neat. Another bounding box, for the extent
    //  of the LineDef.
    pub(crate) bbox: BoundingBox,

    // To aid move clipping.
    pub(crate) slope_type: SlopeType,

    // Front and back sector.
    // Note: redundant? Can be retrieved from SideDefs.
    // I will go with redundant. SKipping these
    // TODO: Regarding my comment above. I have been needing to write some awkward logic to get around
    // not having these two fields. Perhaps I should reconsider.

    // if == validcount, already checked
    pub(crate) valid_count: i32,

    // TODO: void* specialdata?
}

impl LineDef {
    // ML_BLOCKING
    // Solid, is an obstacle.
    pub const FLAG_BLOCKING: i16 = 1;

    // ML_BLOCKMONSTERS
    // Blocks monsters only.
    pub const FLAG_BLOCK_MONSTERS: i16 = 2;

    // ML_TWOSIDED
    // Backside will not be present at all
    //  if not two sided.
    pub const FLAG_TWOSIDED: i16 = 4;

    // ML_DONTPEGTOP
    // upper texture unpegged
    pub const FLAG_DONT_PEG_TOP: i16 = 8;

    // ML_DONTPEGBOTTOM
    // lower texture unpegged
    pub const FLAG_DONT_PEG_BOTTOM: i16 = 16;

    // ML_SECRET
    // In AutoMap: don't map as two sided: IT'S A SECRET!
    pub const FLAG_SECRET: i16 = 32;

    // ML_SOUNDBLOCK
    // Sound rendering: don't let sound cross two of these.
    pub const FLAG_SOUND_BLOCK: i16 = 64;

    // ML_DONTDRAW
    // Don't draw on the automap at all.
    pub const FLAG_DONT_DRAW_ON_AUTOMAP: i16 = 128;

    // ML_MAPPED
    // Set if already seen, thus drawn in automap.
    pub const FLAG_MAPPED: i16 = 256;

    pub fn dont_peg_top_texture(&self) -> bool { (self.flags & Self::FLAG_DONT_PEG_TOP) > 0 }

    pub fn dont_peg_bottom_texture(&self) -> bool { (self.flags & Self::FLAG_DONT_PEG_BOTTOM) > 0 }

    pub fn is_adjacent_to_sector_index(&self, sector_index: usize, sides: &Vec<SideDef>) -> bool {
        if sides[self.front_side_index].sector_index == sector_index {
            return true;
        }

        match self.back_side_index {
            Some(back_side_index) => sides[back_side_index].sector_index == sector_index,
            None => false
        }
    }
}

pub fn load(data: &[u8], vertices: &Vec<Point2D>) -> Vec<LineDef> {
    parse_entity_vector(data, |x: RawLineDef| {
        let v1 = &vertices[x.v1 as usize];
        let v2 = &vertices[x.v2 as usize];
        let dx = v2.x - v1.x;
        let dy = v2.y - v1.y;

        let slope_type = if dx.is_zero() {
            SlopeType::Vertical
        } else if dy.is_zero() {
            SlopeType::Horizontal
        } else {
            if dy / dx > RealNumber::new(0) {
                SlopeType::Positive
            } else {
                SlopeType::Negative
            }
        };

        return LineDef {
            v1: v1.clone(),
            v2: v2.clone(),
            dx,
            dy,
            back_side_index: match x.side_num[1] {
                -1 => None,
                index => Some(index as usize)
            },
            bbox: BoundingBox::from_vertices(v1, v2),
            slope_type,
            front_side_index: x.side_num[0] as usize,
            flags: x.flags,
            special: x.special,
            tag: x.tag,
            valid_count: 0,
        };
    })
}