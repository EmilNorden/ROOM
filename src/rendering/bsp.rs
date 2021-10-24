use crate::level::bounding_box::BoundingBox;
use crate::rendering::View;
use crate::level::Level;
use crate::types::{real, DoomRealNum};
use crate::map_object::{Player, MapObject};
use crate::rendering::renderer::{RENDER_WIDTH, RENDER_HEIGHT};
use crate::level::nodes::Node;

const MAXSEGS: usize = 32;

#[derive(Copy, Clone)]
struct ClipRange {
    first: i32,
    last: i32,
}

impl Default for ClipRange {
    fn default() -> Self {
        Self { first: 0, last: 0 }
    }
}

struct SolidSegs {
    segs: [ClipRange; MAXSEGS],
    valid_segs_count: usize,
}

impl SolidSegs {
    pub fn new(view: &View) -> Self {
        let mut segs = [ClipRange::default(); MAXSEGS];
        segs[0].first = -0x7fffffff;
        segs[0].last = -1;
        segs[1].first = view.width as i32;
        segs[1].last = 0x7fffffff;

        Self {
            segs,
            valid_segs_count: 2,
        }
    }
}


struct Planes {
    floor_clip: [i16; RENDER_WIDTH],
    ceiling_clip: [i16; RENDER_WIDTH],
    visible_planes: [VisPlane; 128],
    last_visible_plane: usize,
    openings: [i16; RENDER_WIDTH * 64],
    last_opening: usize,
    cached_height: [DoomRealNum; RENDER_HEIGHT],
}

impl Planes {
    pub fn new(view: &View) -> Self {
        let floor = [view.height as i16; RENDER_WIDTH];
        let ceiling = [-1; RENDER_WIDTH];

        Self {
            floor_clip: floor,
            ceiling_clip: ceiling,
            visible_planes: [VisPlane::default(); 128],
            last_visible_plane: 0,
            openings: [0i16; RENDER_WIDTH * 64],
            last_opening: 0,
            cached_height: [real(0); RENDER_HEIGHT],
        }
    }
}

#[derive(Copy, Clone)]
struct VisPlane {
    height: DoomRealNum,
    picnum: i32,
    light_level: i32,
    min_x: i32,
    max_x: i32,

    pad1: u8,
    top: [u8; RENDER_WIDTH],
    pad2: u8,
    pad3: u8,
    bottom: [u8; RENDER_WIDTH],
    pad4: u8,
}

impl Default for VisPlane {
    fn default() -> Self {
        Self {
            height: Default::default(),
            picnum: 0,
            light_level: 0,
            min_x: 0,
            max_x: 0,
            pad1: 0,
            top: [0u8; RENDER_WIDTH],
            pad2: 0,
            pad3: 0,
            bottom: [0u8; RENDER_WIDTH],
            pad4: 0,
        }
    }
}

// R_RenderPlayerView
pub fn render_player_view(player: &Player, level: &Level, view: &View) {

    // R_SetupFrame
    let mut solid_segs = SolidSegs::new(view);
    let mut planes = Planes::new(view);

    let viewx = player.x();
    let viewy = player.y();
    let view_angle = player.angle();
    let extra_light = player.extra_light();
    let viewz = player.viewz();

    let viewsin = cordic::sin(view_angle);
    let viewcos = cordic::cos(view_angle);

    // TODO COLOR MAP, check r_main.cpp:772
    if player.fixed_color_map() > 0 {}


    //TODO: Call NetUpdate, apparently

    render_bsp_node(level, level.nodes().len() - 1, viewx, viewy);
}

fn render_bsp_node(level: &Level, node_index: usize, viewx: DoomRealNum, viewy: DoomRealNum) {
    const NF_SUBSECTOR: usize = 0x8000;
    if (node_index & NF_SUBSECTOR) > 0 {
        // Skipping -1 check. not sure why its needed.
        render_subsector(level, node_index & !NF_SUBSECTOR);
        return;
    }

    let node = &level.nodes()[node_index];

    // Decide which side the view point is on.
    let side = point_on_side(viewx, viewy, node);

    // Recursively divide front space.
    render_bsp_node(level, node.children()[side], viewx, viewy);

    // Possibly divide back space.
}

// R_CheckBBox
fn is_area_visible(bounds: &BoundingBox, viewx: DoomRealNum, viewy: DoomRealNum, view_angle: DoomRealNum) -> bool {
    // Find the corners of the box
    // that define the edges from current viewpoint.
    let box_x = if viewx <= bounds.left() {
        0
    } else if viewx < bounds.right() {
        1
    }
    else {
        2
    };

    let box_y = if viewy >= bounds.top() {
        0
    }
    else if viewy > bounds.bottom() {
        1
    }
    else {
        2
    };

    let box_position = (box_y << 2) + box_x;

    if box_position == 5 {
        return true;
    }

    const CHECK_COORD: [[usize; 4]; 11] = [
        [3, 0, 2, 1],
        [3, 0, 2, 0],
        [3, 1, 2, 0],
        [0, 0, 0, 0],
        [2, 0, 2, 1],
        [0, 0, 0, 0],
        [3, 1, 3, 0],
        [0, 0, 0, 0],
        [2, 0, 3, 1],
        [2, 1, 3, 1],
        [2, 1, 3, 0],
    ];

    let x1 = bounds[CHECK_COORD[box_position][0]];
    let y1 = bounds[CHECK_COORD[box_position][1]];
    let x2 = bounds[CHECK_COORD[box_position][2]];
    let y2 = bounds[CHECK_COORD[box_position][3]];

    // check clip list for an open space

    false
}



fn render_subsector(level: &Level, node_index: usize) {

}

//
// R_PointOnSide
// Traverse BSP (sub) tree,
//  check point against partition plane.
// Returns side 0 (front) or 1 (back).
//
// TODO: Rewrite this later
fn point_on_side(x: DoomRealNum, y: DoomRealNum, node: &Node) -> usize {
    if node.dx().is_zero() {
        if x <= node.x() {
            if node.dy() > 0 {
                return 1;
            }
            else {
                return 0;
            }
        }
        else {
            if node.dy() < 0 {
                return 1;
            }
            else {
                return 0;
            }
        }
    }
    if node.dy().is_zero() {
        if y <= node.y() {
            if node.dx() < 0 {
                return 1;
            }
            else {
                return 0;
            }
        }
        else {
            if node.dx() > 0 {
               return 1;
            }
            else {
                return 0;
            }
        }
    }

    let dx = x - node.x();
    let dy = y - node.y();

    if (node.dy() ^ node.dx() ^ dx ^ dy).is_negative() {
        if (node.dy() ^ dx).is_negative() {
            // left is negative
            return 1;
        }
        return 0;
    }

    let left = node.dy() * dx;
    let right = dy * node.dx();

    if right < left {
        return 0;
    }

    1
}