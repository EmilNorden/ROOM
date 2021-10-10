use crate::level::parse_entity_vector;
use crate::level::segs::Seg;
use crate::level::sidedefs::SideDef;
use serde::Deserialize;

#[derive(Deserialize)]
struct RawSubSector {
    num_segs: i16,
    // Index of first one, segs are stored sequentially.
    first_seg_index: i16,
}

pub(crate) struct SubSector {
    pub(crate) num_segs: usize,
    pub(crate) first_seg_index: usize,
    pub(crate) sector_index: usize,
}

pub(crate) fn load(data: &[u8], segs: &Vec<Seg>, side_defs: &Vec<SideDef>) -> Vec<SubSector> {
    parse_entity_vector(data, |x: RawSubSector| {
        let seg = &segs[x.first_seg_index as usize];
        let side_def = &side_defs[seg.sidedef_index];

        SubSector {
            first_seg_index: x.first_seg_index as usize,
            num_segs: x.num_segs as usize,
            sector_index: side_def.sector_index,
        }
    })
}