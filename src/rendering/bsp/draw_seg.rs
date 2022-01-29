use crate::level::segs::Seg;
use crate::number::RealNumber;
use crate::rendering::renderer::RENDER_WIDTH;

pub const SILHOUETTE_NONE: u8 = 0;
pub const SILHOUETTE_TOP: u8 = 1;
pub const SILHOUETTE_BOTTOM: u8 = 2;
pub const SILHOUETTE_BOTH: u8 = 3;

#[derive(Copy, Clone)]
pub struct DrawSeg<'a, 'b> {
    pub current_line: Option<&'a Seg>,
    pub x1: i32,
    pub x2: i32,

    pub scale1: RealNumber,
    pub scale2: RealNumber,
    pub scale_step: RealNumber,
    pub silhouette: u8,

    // do not clip sprites above this
    pub bsilheight: RealNumber,

    // do not clip sprites below this
    pub tsilheight: RealNumber,

    // Pointers to lists for sprite clipping,
    //  all three adjusted so [x1] is first value.
    /*short*		sprtopclip;
    short*		sprbottomclip;
    short*		maskedtexturecol;*/
    pub sprite_top_clip: Option<&'b [i16]>,
    pub sprite_bottom_clip: Option<&'b [i16]>,
    pub masked_texture_col: Option<&'b [u16]>,
}

impl Default for DrawSeg<'_, '_> {
    fn default() -> Self {
        Self {
            silhouette: SILHOUETTE_NONE,
            ..Default::default()
        }
    }
}

pub struct DrawSegs<'a, 'b> {
    pub segs: [DrawSeg<'a, 'b>; 256],
    pub current_index: usize,
}

impl DrawSegs<'_, '_> {
    pub fn new() -> Self {
        Self {
            segs: [DrawSeg::default(); 256],
            current_index: 0,
        }
    }
}