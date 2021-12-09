use crate::level::bounding_box::BoundingBox;
use crate::rendering::View;
use crate::level::Level;
use crate::types::{real, DoomRealNum, real_to_int};
use crate::map_object::{Player, MapObject};
use crate::rendering::renderer::{RENDER_WIDTH, RENDER_HEIGHT};
use crate::level::nodes::Node;
use crate::rendering::types::{Angle, Point};
use crate::rendering::types::tables::{FINEANGLES, FINE_TANGENT};

struct BspRenderer {
    view_angle_to_x: [i32; FINEANGLES / 2],
    x_to_view_angle: [Angle; RENDER_WIDTH + 1],
    clip_angle: Angle,
}

impl BspRenderer {
    pub fn new(view: &View) -> Self {
        const FIELD_OF_VIEW: usize = 2048; // Fineangles in the SCREENWIDTH wide window.
        // R_InitTextureMapping

        // Use tangent table to generate viewangletox:
        //  viewangletox will give the next greatest x
        //  after the view angle.
        //
        // Calc focallength
        //  so FIELDOFVIEW angles covers SCREENWIDTH.
        let focal_length = view.centerxfrac / FINE_TANGENT[FINEANGLES / 4 + FIELD_OF_VIEW / 2];
        let mut view_angle_to_x = [0i32; FINEANGLES / 2];
        for i in 0..(FINEANGLES / 2) {
            // What is this constant? No idea
            // The original code was FRACUNIT * 2, which expands to (1<<16) * 2, ie fixed point 2.0
            view_angle_to_x[i] = if FINE_TANGENT[i] > real(2) {
                -1
            } else if FINE_TANGENT[i] < real(-2) {
                view.width as i32
            } else {
                let a = real_to_int(view.centerxfrac -
                    (FINE_TANGENT[i] * focal_length) + real(1) - real(0.00002));

                a.clamp(-1, view.width as i32 + 1)
            };
        }

        // Scan viewangletox[] to generate xtoviewangle[]:
        //  xtoviewangle will give the smallest view angle
        //  that maps to x.
        let mut x_to_view_angle = [Angle::default(); RENDER_WIDTH + 1];
        for x in 0..=view.width {
            let mut i = view_angle_to_x.iter().position(|angle| *angle <= x as i32)
                .expect("Unable to find smallest view angle for x. This shouldnt fail.");
            // x_to_view_angle[x] =
            x_to_view_angle[x] = Angle::from_fine_shift(i) - Angle::angle90();
        }

        // Take out the fencepost cases from viewangletox
        for i in 0..(FINEANGLES / 2) {
            if view_angle_to_x[i] == -1 {
                view_angle_to_x[i] = 0;
            }
            else if view_angle_to_x[i] == view.width as i32 + 1 {
                view_angle_to_x[i] = view.width as i32;
            }
        }

        let clip_angle = x_to_view_angle[0];
        Self {
            x_to_view_angle,
            view_angle_to_x,
            clip_angle
        }
    }
}

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

    let view_position = Point::new(player.x(), player.y());
    let view_angle = player.angle();
    let extra_light = player.extra_light();
    let viewz = player.viewz();

    let viewsin = view_angle.sine();
    let viewcos = view_angle.cosine();

    // TODO COLOR MAP, check r_main.cpp:772
    if player.fixed_color_map() > 0 {}
    // End R_SetupFrame

    //TODO: Call NetUpdate, apparently

    render_bsp_node(level, level.nodes().len() - 1, &view_position);
}

fn render_bsp_node(level: &Level, node_index: usize, view_position: &Point) {
    const NF_SUBSECTOR: usize = 0x8000;
    if (node_index & NF_SUBSECTOR) > 0 {
        // Skipping -1 check. not sure why its needed.
        render_subsector(level, node_index & !NF_SUBSECTOR);
        return;
    }

    let node = &level.nodes()[node_index];

    // Decide which side the view point is on.
    let side = point_on_side(view_position, node);

    // Recursively divide front space.
    render_bsp_node(level, node.children()[side], view_position);

    // Possibly divide back space.
}

// R_CheckBBox
fn is_area_visible(bounds: &BoundingBox, view_position: &Point, view_angle: &Angle) -> bool {
    // Find the corners of the box
    // that define the edges from current viewpoint.
    let box_x = if view_position.x() <= bounds.left() {
        0
    } else if view_position.x() < bounds.right() {
        1
    } else {
        2
    };

    let box_y = if view_position.y() >= bounds.top() {
        0
    } else if view_position.y() > bounds.bottom() {
        1
    } else {
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
    let mut angle1 = Angle::from_points(&Point::new(x1, y1), view_position) - *view_angle;
    let mut angle2 = Angle::from_points(&Point::new(x2, y2), view_position) - *view_angle;

    let span = angle1 - angle2;

    if span >= Angle::angle180() {
        return true;
    }

    let clip_angle = Angle::new(537395200); // Should come from R_InitTextureMapping, hard-coding for now.
    let mut tspan = angle1 + clip_angle;

    if tspan > 2 * clip_angle {
        tspan -= 2 * clip_angle;

        // Totally off the left edge?
        if tspan >= span {
            return false;
        }

        angle1 = clip_angle;
    }

    tspan = clip_angle - angle2;
    if tspan > 2 * clip_angle {
        tspan -= 2 * clip_angle;

        // Totally off the left edge?
        if tspan >= span {
            return false;
        }

        // Original code flipped angle by negating, but Rust wont allow negating an unsigned value.
        // I will XOR with the MSB instead.
        // In Binary Angular Measurement, MSB represents 180 degrees.
        angle2 = clip_angle.flip();
    }

    // Find the first clippost
    //  that touches the source post
    //  (adjacent pixels are touching).

    angle1 = angle1.rotate(Angle::angle90()).fineshift();
    angle2 = angle2.rotate(Angle::angle90()).fineshift();


    // Sitting on a line?
    //  if spanfalse
    false
}


fn render_subsector(level: &Level, node_index: usize) {}

//
// R_PointOnSide
// Traverse BSP (sub) tree,
//  check point against partition plane.
// Returns side 0 (front) or 1 (back).
//
// TODO: Rewrite this later
fn point_on_side(point: &Point, node: &Node) -> usize {
    if node.dx().is_zero() {
        if point.x() <= node.x() {
            if node.dy() > 0 {
                return 1;
            } else {
                return 0;
            }
        } else {
            if node.dy() < 0 {
                return 1;
            } else {
                return 0;
            }
        }
    }
    if node.dy().is_zero() {
        if point.y() <= node.y() {
            if node.dx() < 0 {
                return 1;
            } else {
                return 0;
            }
        } else {
            if node.dx() > 0 {
                return 1;
            } else {
                return 0;
            }
        }
    }

    let dx = point.x() - node.x();
    let dy = point.y() - node.y();

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