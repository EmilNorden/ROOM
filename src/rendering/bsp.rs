mod draw_seg;
mod planes;
mod solid_seg;

use crate::constants::FRAC_BITS;
use crate::graphics::flats::FlatNumber;
use crate::graphics::GraphicsData;
use crate::graphics::light_table::{LIGHT_SCALE_SHIFT, LIGHT_SEG_SHIFT, MAX_LIGHT_SCALE};
use crate::graphics::textures::{TextureData, TextureNumber};
use crate::level::bounding_box::BoundingBox;
use crate::rendering::View;
use crate::level::Level;
use crate::map_object::{Player, MapObject};
use crate::rendering::renderer::{RENDER_WIDTH, RENDER_HEIGHT};
use crate::level::nodes::Node;
use crate::level::sectors::Sector;
use crate::level::segs::Seg;
use crate::number::RealNumber;
use crate::rendering::bsp::draw_seg::{DrawSegs, SILHOUETTE_BOTTOM, SILHOUETTE_NONE, SILHOUETTE_TOP};
use crate::rendering::bsp::planes::{Planes, VisPlane};
use crate::rendering::bsp::solid_seg::SolidSegs;
use crate::rendering::types::{Angle, Point2D, Point3D};
use crate::rendering::types::tables::{FINEANGLES, FINE_TANGENT, get_sine_table};

pub struct BspRenderer {
    view_angle_to_x: [i32; FINEANGLES / 2],
    x_to_view_angle: [Angle; RENDER_WIDTH + 1],
    clip_angle: Angle,
    neg_one_array: [i16; RENDER_WIDTH],
    screen_height_array: [i16; RENDER_WIDTH],
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
        let abc = FINE_TANGENT[FINEANGLES / 4 + FIELD_OF_VIEW / 2];
        let def = FINE_TANGENT[3072];

        // TODO: Was it a mistake to introduce DoomRealNum?
        let focal_length = view.centerxfrac / RealNumber::new_from_bits(FINE_TANGENT[FINEANGLES / 4 + FIELD_OF_VIEW / 2]);
        // let focal_length = real(real_to_bits(view.centerxfrac) as f32 / FINE_TANGENT[FINEANGLES / 4 + FIELD_OF_VIEW / 2] as f32);
        let mut view_angle_to_x = [0i32; FINEANGLES / 2];
        for i in 0..(FINEANGLES / 2) {
            if i == 2048 {
                let asl = 32;
            }
            // What is this constant? No idea
            // The original code was FRACUNIT * 2, which expands to (1<<16) * 2, ie fixed point 2.0
            view_angle_to_x[i] = if RealNumber::new_from_bits(FINE_TANGENT[i]) > RealNumber::new(2) {
                -1
            } else if RealNumber::new_from_bits(FINE_TANGENT[i]) < RealNumber::new(-2) {
                view.width as i32 + 1
            } else {
                let t = RealNumber::new_from_bits(FINE_TANGENT[i]) * focal_length;
                let t = view.centerxfrac - t + RealNumber::new(1) - RealNumber::new_from_bits(1);

                t.to_int().clamp(-1, view.width as i32 + 1)
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
            // 4290248704
            x_to_view_angle[x] = Angle::from_fine_shift(i) - Angle::angle90();
        }

        // Take out the fencepost cases from viewangletox
        for i in 0..(FINEANGLES / 2) {
            if view_angle_to_x[i] == -1 {
                view_angle_to_x[i] = 0;
            } else if view_angle_to_x[i] == view.width as i32 + 1 {
                view_angle_to_x[i] = view.width as i32;
            }
        }

        let clip_angle = x_to_view_angle[0];

        let neg_one_array = [-1; RENDER_WIDTH];
        let screen_height_array = [view.height as i16; RENDER_WIDTH];
        Self {
            x_to_view_angle,
            view_angle_to_x,
            clip_angle,
            neg_one_array,
            screen_height_array,
        }
    }

    // R_RenderPlayerView
    pub fn render_player_view(&self, player: &Player, level: &Level, view: &View, graphics_data: &GraphicsData) {

        // R_SetupFrame
        let mut solid_segs = SolidSegs::new(view);
        let mut draw_segs = DrawSegs::new();
        let mut planes = Planes::new(view);

        let view_position = Point2D::new(player.x(), player.y());
        let view_angle = player.angle();
        let extra_light = player.extra_light();
        let viewz = player.viewz();

        let viewsin = view_angle.sine();
        let viewcos = view_angle.cosine();

        // TODO COLOR MAP, check r_main.cpp:772
        if player.fixed_color_map() > 0 {}
        // End R_SetupFrame

        //TODO: Call NetUpdate, apparently

        let hard_coded_view_position = Point3D::new(
            RealNumber::new(864),
            RealNumber::new(-96),
            RealNumber::new(41),
        );


        self.render_bsp_node(level, level.nodes().len() - 1, &hard_coded_view_position, view_angle, view, &mut planes, &mut solid_segs, &mut draw_segs, graphics_data);
    }

    fn render_bsp_node<'a, 'b, 'c>(
        &'c self,
        level: &'a Level,
        node_index: usize,
        view_position: &Point3D,
        view_angle: Angle,
        view: &View,
        planes: &mut Planes,
        solid_segs: &mut SolidSegs,
        draw_segs: &'b mut DrawSegs<'a, 'c>,
        graphics_data: &GraphicsData) {
        const NF_SUBSECTOR: usize = 0x8000;
        if (node_index & NF_SUBSECTOR) > 0 {
            // Skipping -1 check. not sure why its needed.
            self.render_subsector(
                level,
                node_index & !NF_SUBSECTOR,
                view_position,
                view_angle,
                view,
                planes,
                solid_segs,
                draw_segs,
                graphics_data);
            return;
        }

        let node = &level.nodes()[node_index];

        // Decide which side the view point is on.
        let side = point_on_side(view_position, node);

        // Recursively divide front space.
        self.render_bsp_node(
            level,
            node.children()[side],
            view_position,
            view_angle,
            view,
            planes,
            solid_segs,
            draw_segs,
            graphics_data);

        // Possibly divide back space.
    }

    fn render_subsector<'a, 'b, 'c>(
        &'c self,
        level: &'a Level,
        node_index: usize,
        view_position: &Point3D,
        view_angle: Angle,
        view: &View,
        planes: &mut Planes,
        solid_segs: &mut SolidSegs,
        draw_segs: &'b mut DrawSegs<'a, 'c>,
        graphics_data: &GraphicsData) {
        let subsector = level.sub_sectors().get(node_index)
            .expect(&*format!("R_Subsector: ss{} with numss = {}", node_index, level.sub_sectors().len()));

        // sscount++
        // TODO: I dont know why the original code counts the number of renderes subsectors.
        // From what I can see it is not used anywhere.

        let front_sector = &level.sectors()[subsector.sector_index];
        let mut count = subsector.num_segs;

        // TODO: This is the original location of find_plane calls for floor_plane and ceiling_plane

        // TODO: R_AddSprites

        let lines = &level.segs()[subsector.first_seg_index..(subsector.first_seg_index + count)];
        for line in lines {
            self.add_line(
                level,
                line,
                front_sector,
                view_position,
                view_angle,
                view,
                solid_segs,
                draw_segs,
                graphics_data,
                planes);
        }
    }

    //
// R_AddLine
// Clips the given segment
// and adds any visible pieces to the line list.
//
    fn add_line<'a, 'b, 'c>(
        &'c self,
        level: &Level,
        line: &'a Seg,
        front_sector: &Sector,
        view_position: &Point3D,
        view_angle: Angle,
        view: &View,
        solid_segs: &mut SolidSegs,
        draw_segs: &'b mut DrawSegs<'a, 'c>,
        graphics_data: &GraphicsData,
        planes: &mut Planes) {
        let vertices = level.vertices();

        let v1 = &vertices[line.vertex1_index];
        let v2 = &vertices[line.vertex2_index];
        let mut angle1 = Angle::from_points(v1, &view_position.into());
        let mut angle2 = Angle::from_points(v2, &view_position.into());

        // Clip to view edges.
        let span = angle1 - angle2;

        // Back side? I.e. backface culling
        if span >= Angle::angle180() {
            return;
        }

        // Global angle needed by segcalc
        let rw_angle1 = angle1;
        angle1 -= view_angle;
        angle2 -= view_angle;

        let mut tspan = angle1 + self.clip_angle;
        if tspan > 2 * self.clip_angle {
            tspan -= 2 * self.clip_angle;

            // Totally off the left edge?
            if tspan >= span {
                return;
            }

            angle1 = self.clip_angle;
        }

        tspan = self.clip_angle - angle2;
        if tspan > 2 * self.clip_angle {
            tspan -= 2 * self.clip_angle;

            // Totally off the left edge?
            if tspan >= span {
                return;
            }

            angle2 = -self.clip_angle;
        }

        // The seg is in the view range,
        // but not necessarily visible.
        angle1 = (angle1 + Angle::angle90()).fineshift();
        angle2 = (angle2 + Angle::angle90()).fineshift();

        let x1 = self.view_angle_to_x[angle1.to_u32() as usize];
        let x2 = self.view_angle_to_x[angle2.to_u32() as usize];

        // Does not cross a pixel?
        if x1 == x2 {
            return;
        }

        let optional_back_sector = line.back_sector_index.map(|x| &level.sectors()[x]);

        match optional_back_sector {
            // Single sided line?
            None => Self::clip_solid_wall_segment(x1, x2 - 1),

            Some(back_sector) if Self::is_closed_door(back_sector, front_sector)
            => Self::clip_solid_wall_segment(x1, x2 - 1),

            Some(back_sector) if Self::is_window(back_sector, front_sector)
            => self.clip_pass_wall_segment(
                level,
                line,
                front_sector,
                optional_back_sector,
                x1,
                x2 - 1,
                rw_angle1,
                view_position,
                view_angle,
                view,
                solid_segs,
                draw_segs,
                graphics_data,
                planes,
            ),

            Some(back_sector) => {
                // Reject empty lines used for triggers
                //  and special events.
                // Identical floor and ceiling on both sides,
                // identical light levels on both sides,
                // and no middle texture.
                if back_sector.ceiling_pic == front_sector.ceiling_pic &&
                    back_sector.floor_pic == front_sector.floor_pic &&
                    back_sector.light_level == front_sector.light_level &&
                    level.side_defs()[line.sidedef_index].mid_texture.is_zero() {
                    return;
                }

                self.clip_pass_wall_segment(
                    level,
                    line,
                    front_sector,
                    optional_back_sector,
                    x1,
                    x2 - 1,
                    rw_angle1,
                    view_position,
                    view_angle,
                    view,
                    solid_segs,
                    draw_segs,
                    graphics_data,
                    planes,
                );
            }
        }
    }

    fn is_closed_door(back_sector: &Sector, front_sector: &Sector) -> bool {
        back_sector.ceiling_height <= front_sector.floor_height ||
            back_sector.floor_height >= front_sector.ceiling_height
    }

    fn is_window(back_sector: &Sector, front_sector: &Sector) -> bool {
        back_sector.ceiling_height != front_sector.ceiling_height
            || back_sector.floor_height != front_sector.floor_height
    }

    fn clip_solid_wall_segment(x: i32, y: i32) {}

    fn clip_pass_wall_segment<'a, 'b, 'c>(
        &'c self,
        level: &Level,
        line: &'a Seg,
        front_sector: &Sector,
        back_sector: Option<&Sector>,
        first: i32,
        last: i32,
        rw_angle: Angle,
        view_position: &Point3D,
        view_angle: Angle,
        view: &View,
        solid_segs: &mut SolidSegs,
        draw_segs: &'b mut DrawSegs<'a, 'c>,
        graphics_data: &GraphicsData,
        planes: &mut Planes) {
        // Find the first range that touches the range
        //  (adjacent pixels are touching).
        let start_index = solid_segs.segs
            .iter()
            .position(|x| x.last >= first - 1)
            .unwrap();

        let start = &solid_segs.segs[start_index];

        if first < start.first {
            if last < start.first - 1 {
                // Post is entirely visible (above start).
                // TODO R_StoreWallRange(first, last);
                return;
            }

            // There is a fragment above *start.
            // TODO R_StoreWallRange(first, start->first - 1);
        }

        // Bottom contained in start?
        if last <= start.last {
            return;
        }

        for i in start_index..31 { // TODO 31 what? Introduce constant? HInt: MAXSEGS in original code
            if last < solid_segs.segs[i + 1].first - 1 {
                break;
            }

            self.store_wall_range(level,
                                  line,
                                  front_sector,
                                  back_sector,
                                  solid_segs.segs[i].last + 1,
                                  solid_segs.segs[i + 1].first - 1,
                                  rw_angle,
                                  view_position,
                                  view_angle,
                                  view,
                                  solid_segs,
                                  draw_segs,
                                  graphics_data,
                                  planes,
            );

            if last <= solid_segs.segs[i + 1].last {
                return;
            }
        }

        // TODO: ANOTHER CALL TO STORE_WALL_RANGE
        // Self::store_wall_range()
    }

    fn store_wall_range<'a, 'b, 'c>(&'c self,
                                    level: &Level,
                                    line: &'a Seg,
                                    front_sector: &Sector,
                                    optional_back_sector: Option<&Sector>,
                                    start: i32,
                                    stop: i32,
                                    rw_angle: Angle,
                                    view_position: &Point3D,
                                    view_angle: Angle,
                                    view: &View,
                                    solid_segs: &mut SolidSegs,
                                    draw_segs: &'b mut DrawSegs<'a, 'c>,
                                    graphics_data: &GraphicsData,
                                    planes: &mut Planes)
        where 'a: 'b {

        // Don't overflow and crash
        if draw_segs.current_index == draw_segs.segs.len() - 1 {
            return;
        }

        if start > stop { // TODO: Also check start >= viewwidth (check R_StoreWallRange)
            panic!("Bad R_StoreWallRange: {} to {}", start, stop);
        }

        let sidedef = &level.side_defs()[line.sidedef_index];
        let linedef = &level.line_defs()[line.linedef_index];

        // TODO: Fix this for automap
        // mark the segment as visible for auto map
        // linedef->flags |= ML_MAPPED;

        // calculate rw_distance for scale calculation
        let rw_normalangle = line.angle + Angle::angle180();
        let mut offset_angle = rw_normalangle - rw_angle;

        if offset_angle > Angle::angle90() {
            offset_angle = Angle::angle90();
        }

        let distangle = Angle::angle90() - offset_angle;
        let v1 = &level.vertices()[line.vertex1_index];
        let v2 = &level.vertices()[line.vertex2_index];
        let hyp = v1.distance(Point2D::new(view_position.x, view_position.y));
        let sineval = RealNumber::new(get_sine_table()[distangle.fineshift().to_u32() as usize]);

        let rw_distance = hyp * sineval;

        let rw_x = start;
        let current_seg = &mut draw_segs.segs[draw_segs.current_index];
        current_seg.x1 = start;
        current_seg.x2 = stop;
        current_seg.current_line = Some(line);
        let rw_stopx = stop + 1;

        // calculate scale at both ends and step
        let rw_scale = Self::scale_from_global_angle(
            view_angle + self.x_to_view_angle[start as usize],
            view_angle,
            rw_normalangle,
            rw_distance,
            view,
        );
        current_seg.scale1 = rw_scale;

        let mut rw_scalestep = RealNumber::new(0);

        if stop > start {
            current_seg.scale2 = Self::scale_from_global_angle(
                view_angle + self.x_to_view_angle[stop as usize],
                view_angle,
                rw_normalangle,
                rw_distance,
                view,
            );
            rw_scalestep = (current_seg.scale2 - rw_scale) / (stop - start);
            current_seg.scale_step = rw_scalestep;
        } else {
            current_seg.scale2 = current_seg.scale1;
        }
        // calculate texture boundaries
        //  and decide if floor / ceiling marks are needed
        let mut world_top = front_sector.ceiling_height - view_position.z;
        let mut world_bottom = front_sector.floor_height - view_position.z;

        let mid_texture = 0;
        let top_texture = 0;
        let bottom_texture = 0;
        let masked_texture = 0;

        current_seg.masked_texture_col = None;

        let mut mark_floor = false;
        let mut mark_ceiling = false;

        let mut world_high = RealNumber::new(0);
        let mut world_low = RealNumber::new(0);

        if let Some(back_sector) = optional_back_sector {
            // two sided line
            current_seg.sprite_top_clip = None;
            current_seg.sprite_bottom_clip = None;
            current_seg.silhouette = SILHOUETTE_NONE;

            if front_sector.floor_height > back_sector.floor_height {
                current_seg.silhouette = SILHOUETTE_BOTTOM;
                current_seg.bsilheight = front_sector.floor_height;
            } else if back_sector.floor_height > view_position.z {
                current_seg.silhouette = SILHOUETTE_BOTTOM;
                current_seg.bsilheight = RealNumber::new_from_bits(i32::MAX);
            }

            if front_sector.ceiling_height < back_sector.ceiling_height {
                current_seg.silhouette |= SILHOUETTE_TOP;
                current_seg.tsilheight = front_sector.ceiling_height;
            } else if back_sector.ceiling_height < view_position.z {
                current_seg.silhouette |= SILHOUETTE_TOP;
                current_seg.tsilheight = RealNumber::new_from_bits(i32::MIN);
            }

            if back_sector.ceiling_height <= front_sector.floor_height {
                // TODO r_segs.cpp:463
                current_seg.sprite_bottom_clip = Some(&self.neg_one_array);
                current_seg.bsilheight = RealNumber::new_from_bits(i32::MAX);
                current_seg.silhouette |= SILHOUETTE_BOTTOM;
            }

            if back_sector.floor_height >= front_sector.ceiling_height {
                current_seg.sprite_top_clip = Some(&self.screen_height_array);
                current_seg.tsilheight = RealNumber::new_from_bits(i32::MIN);
                current_seg.silhouette |= SILHOUETTE_TOP;
            }

            world_high = back_sector.ceiling_height - view_position.z;
            world_low = back_sector.ceiling_height - view_position.z;

            // hack to allow height changes in outdoor areas
            if front_sector.ceiling_pic == level.sky_flat_number() &&
                back_sector.ceiling_pic == level.sky_flat_number() {
                world_top = world_high;
            }

            mark_floor = world_low != world_bottom ||
                back_sector.floor_pic != front_sector.floor_pic ||
                back_sector.light_level != front_sector.light_level;

            mark_ceiling = world_high != world_top
                || back_sector.ceiling_pic != front_sector.ceiling_pic
                || back_sector.light_level != front_sector.light_level;

            if back_sector.ceiling_height <= front_sector.floor_height
                || back_sector.floor_height >= front_sector.ceiling_height {
                // closed door
                mark_floor = true;
                mark_ceiling = true;
            }

            let mut rw_top_texture_mid = RealNumber::new(0);
            let mut top_texture = TextureNumber(0);
            if world_high < world_top {
                // top texture
                top_texture =
                    graphics_data.textures().get_texture_translation(sidedef.top_texture);

                if linedef.dont_peg_top_texture() {
                    rw_top_texture_mid = world_top
                } else {
                    let vtop = back_sector.ceiling_height +
                        graphics_data.textures().get_texture_height(sidedef.top_texture);

                    // bottom of texture
                    rw_top_texture_mid = vtop - view_position.z;
                }
            }

            let mut rw_bottom_texture_mid = RealNumber::new(0);
            let mut bottom_texture = TextureNumber(0);
            if world_low > world_bottom {
                // bottom texture
                bottom_texture =
                    graphics_data.textures().get_texture_translation(sidedef.bottom_texture);

                if linedef.dont_peg_bottom_texture() {
                    // bottom of texture at bottom
                    // top of texture at top
                    rw_bottom_texture_mid = world_top;
                } else {
                    // top of texture at top
                    rw_bottom_texture_mid = world_low;
                }
            }

            rw_top_texture_mid += sidedef.row_offset;
            rw_bottom_texture_mid += sidedef.row_offset;

            // allocate space for masked texture tables
            let mut masked_texture = false;
            if sidedef.mid_texture.is_zero() {
                // TODO THIS ISNT NEEDED FOR NOW
                panic!("implement this! r_segs.cpp:542");
                // masked midtexture
                masked_texture = true;

                // current_seg.masked_texture_col
            }
        }

        // calculate rw_offset (only needed for textured lines)
        let seg_textured = mid_texture > 0 || top_texture > 0 || bottom_texture > 0 || masked_texture > 0;

        let mut rw_offset = RealNumber::new(0);
        let mut rw_center_angle = Angle::new(0);
        if seg_textured {
            let mut offset_angle = rw_normalangle - rw_angle;

            if offset_angle > Angle::angle180() {
                // TODO: Original code was angle = -angle. IS this equal?
                offset_angle = offset_angle.flip();
            }

            if offset_angle > Angle::angle90() {
                offset_angle = Angle::angle90();
            }

            let sineval = RealNumber::new_from_bits(get_sine_table()[offset_angle.fineshift().to_u32() as usize]);
            rw_offset = hyp * sineval;

            if rw_normalangle - rw_angle < Angle::angle180() {
                rw_offset = -rw_offset;
            }

            rw_offset += sidedef.texture_offset + line.offset;
            rw_center_angle = Angle::angle90() + view_angle + rw_normalangle;
            // calculate light table
            //  use different light tables
            //  for horizontal / vertical / diagonal
            // OPTIMIZE: get rid of LIGHTSEGSHIFT globally
            // TODO: Lets assume that fixedcolormap is always 0 (or None in our case)
            if true {
                let mut lightnum = (front_sector.light_level >> LIGHT_SEG_SHIFT);
                //TODO: extralight should be added to lightnum. Look at original code.

                if v1.y == v2.y {
                    lightnum -= 1;
                }
                else if v1.x == v2.x {
                    lightnum += 1;
                }

                if lightnum < 0 {

                }
            }
        }

        // if a floor / ceiling plane is on the wrong side
        //  of the view plane, it is definitely invisible
        //  and doesn't need to be marked.

        if front_sector.floor_height >= view_position.z {
            // above view plane
            mark_floor = false;
        }

        if front_sector.ceiling_height <= view_position.z &&
            front_sector.ceiling_pic != level.sky_flat_number() {
            // below view plane
            mark_ceiling = false;
        }

        // calculate incremental stepping values for texture edges
        world_top >>= 4;
        world_bottom >>= 4;

        let top_step = -rw_scalestep * world_top;
        let top_frac = (view.centeryfrac >> 4) - world_top * rw_scale;

        let bottom_step = -rw_scalestep * world_bottom;
        let bottom_frac = (view.centeryfrac >> 4) - world_bottom * rw_scale;

        let mut pix_high = RealNumber::new(0);
        let mut pix_high_step = RealNumber::new(0);
        let mut pix_low = RealNumber::new(0);
        let mut pix_low_step = RealNumber::new(0);

        if optional_back_sector.is_some() {
            world_high >>= 4;
            world_low >>= 4;

            if world_high < world_top {
                pix_high = (view.centeryfrac >> 4) - world_high * rw_scale;
                pix_high_step = -rw_scalestep * world_high;
            }

            if world_low > world_bottom {
                pix_low = (view.centeryfrac >> 4) - world_low * rw_scale;
                pix_low_step = -rw_scalestep * world_low;
            }
        }

        // render it
        // NOTE This is not the original location of these find_plane calls
        let floor_plane_index = if front_sector.floor_height < view_position.z() {
            Some(find_plane_index(level,
                                  front_sector.floor_height,
                                  front_sector.floor_pic,
                                  front_sector.light_level,
                                  planes))
        } else {
            None
        };

        let ceiling_plane_index = if front_sector.ceiling_height > view_position.z ||
            front_sector.ceiling_pic == level.sky_flat_number() {
            Some(find_plane_index(level,
                                  front_sector.ceiling_height,
                                  front_sector.ceiling_pic,
                                  front_sector.light_level,
                                  planes))
        } else {
            None
        };

        // TODO: IMPLEMENT THIS
        /*
        if (markceiling)
        ceilingplane = R_CheckPlane(ceilingplane, rw_x, rw_stopx - 1);

    if (markfloor)
        floorplane = R_CheckPlane(floorplane, rw_x, rw_stopx - 1);
        */

        self.render_seg_loop(rw_x, rw_stopx, rw_center_angle, rw_offset, rw_distance, rw_scale, top_frac, bottom_frac, mark_ceiling, mark_floor, floor_plane_index, ceiling_plane_index, planes, seg_textured);

        // TODO: IMPLEMENT THIS
        /*
            // save sprite clipping info
    if (((ds_p->silhouette & SIL_TOP) || maskedtexture)
        && !ds_p->sprtopclip) {
        memcpy(lastopening, ceilingclip + start, 2 * (rw_stopx - start));
        ds_p->sprtopclip = lastopening - start;
        lastopening += rw_stopx - start;
    }

    if (((ds_p->silhouette & SIL_BOTTOM) || maskedtexture)
        && !ds_p->sprbottomclip) {
        memcpy(lastopening, floorclip + start, 2 * (rw_stopx - start));
        ds_p->sprbottomclip = lastopening - start;
        lastopening += rw_stopx - start;
    }

    if (maskedtexture && !(ds_p->silhouette & SIL_TOP)) {
        ds_p->silhouette |= SIL_TOP;
        ds_p->tsilheight = MININT;
    }
    if (maskedtexture && !(ds_p->silhouette & SIL_BOTTOM)) {
        ds_p->silhouette |= SIL_BOTTOM;
        ds_p->bsilheight = MAXINT;
    }
    ds_p++;
        */
    }

    //
// R_RenderSegLoop
// Draws zero, one, or two textures (and possibly a masked
//  texture) for walls.
// Can draw or mark the starting pixel of floor and ceiling
//  textures.
// CALLED: CORE LOOPING ROUTINE.
//
    fn render_seg_loop(
        &self,
        rw_x: i32,
        rw_stopx: i32,
        rw_center_angle: Angle,
        rw_offset: RealNumber,
        rw_distance: RealNumber,
        rw_scale: RealNumber,
        top_frac: RealNumber,
        bottom_frac: RealNumber,
        mark_ceiling: bool,
        mark_floor: bool,
        floor_plane_index: Option<usize>,
        ceiling_plane_index: Option<usize>,
        planes: &mut Planes,
        seg_textured: bool) {
        pub const HEIGHT_BITS: i32 = 12;
        pub const HEIGHT_UNIT: i32 = (1 << HEIGHT_BITS);

        let rw_x = rw_x as usize;
        let rw_stopx = rw_stopx as usize;

        let mut top = 0i32;
        let mut bottom = 0i32;
        for x in rw_x..rw_stopx {
            // mark floor / ceiling areas
            let mut yl = (top_frac.to_bits() + HEIGHT_UNIT - 1) >> HEIGHT_BITS;

            // no space above wall?
            yl = yl.max(planes.ceiling_clip[x] as i32 + 1);

            if mark_ceiling {
                top = planes.ceiling_clip[x] as i32 + 1;
                bottom = yl - 1;

                bottom = bottom.min(planes.floor_clip[x] as i32 - 1);

                if top <= bottom {
                    let ceiling_plane_index = ceiling_plane_index.unwrap();
                    planes.visible_planes[ceiling_plane_index].top[rw_x] = top as u8;
                    planes.visible_planes[ceiling_plane_index].bottom[rw_x] = bottom as u8;
                    // In the original code, ceilingclip is an array of short, which is then read from and placed into an int. This int is then written to ceilingplane->top[x] which is a byte...
                    // Seems like they couldn't decide on a size and stick with it.
                    // planes.visible_planes[]
                }
            }

            let mut yh = bottom_frac.to_bits() >> HEIGHT_BITS;

            yh = yh.min(planes.floor_clip[rw_x as usize] as i32 - 1);

            if mark_floor {
                top = yh + 1;
                bottom = planes.floor_clip[rw_x] as i32 - 1;
                if top <= planes.ceiling_clip[rw_x] as i32 {
                    top = planes.ceiling_clip[rw_x] as i32 + 1;
                }
                if top <= bottom {
                    let floor_plane_index = floor_plane_index.unwrap();
                    planes.visible_planes[floor_plane_index].top[rw_x] = top as u8;
                    planes.visible_planes[floor_plane_index].bottom[rw_x] = bottom as u8;
                }
            }

            // texturecolumn and lighting are independent of wall tiers
            if seg_textured {
                // calculate texture offset
                let angle = (rw_center_angle + self.x_to_view_angle[rw_x]).fineshift();
                // TODO: This whole Angle -> Fineshift -> using the angle to index into the FINE_TANGENT array looks weird as hell. Figure it out :)
                let texture_column = rw_offset - RealNumber::new(FINE_TANGENT[angle.to_u32() as usize]) * rw_distance;
                let texture_column = texture_column.to_int();

                // calculate lighting
                let index = rw_scale >> LIGHT_SCALE_SHIFT;
                let mut index = index.to_bits() as usize;
                index = index.min(MAX_LIGHT_SCALE - 1);



            }
        }
    }

    //
// R_ScaleFromGlobalAngle
// Returns the texture mapping scale
//  for the current line (horizontal span)
//  at the given angle.
// rw_distance must be calculated first.
//
    fn scale_from_global_angle(visangle: Angle, view_angle: Angle, rw_normalangle: Angle, rw_distance: RealNumber, view: &View) -> RealNumber {
        let anglea = Angle::angle90() + (visangle - view_angle);
        let angleb = Angle::angle90() + (visangle - rw_normalangle);

        // both sines are always positive
        let sinea = RealNumber::new(get_sine_table()[anglea.fineshift().to_u32() as usize]);
        let sineb = RealNumber::new(get_sine_table()[angleb.fineshift().to_u32() as usize]);
        let num = (view.projection * sineb) << view.detail_shift;
        let den = rw_distance * sinea;

        if den > num >> 16 {
            let mut scale = num * den;

            if scale > RealNumber::new(64) {
                scale = RealNumber::new(64);
            } else if scale < RealNumber::new_from_bits(256) {
                scale = RealNumber::new_from_bits(256);
            }

            scale
        } else {
            RealNumber::new(64)
        }
    }
}

// R_CheckBBox
fn is_area_visible(bounds: &BoundingBox, view_position: &Point2D, view_angle: &Angle) -> bool {
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
    let mut angle1 = Angle::from_points(&Point2D::new(x1, y1), view_position) - *view_angle;
    let mut angle2 = Angle::from_points(&Point2D::new(x2, y2), view_position) - *view_angle;

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

fn find_plane_index<'a>(level: &Level, mut height: RealNumber, picnum: FlatNumber, mut light_level: i16, planes: &'a mut Planes) -> usize {
    if picnum == level.sky_flat_number() {
        height = RealNumber::new(0);
        light_level = 0;
    }

    let plane_index = planes.visible_planes.iter().position(|plane|
        plane.height == height &&
            plane.picnum == picnum &&
            plane.light_level == light_level)
        .filter(|x| *x < planes.last_visible_plane);

    if let Some(index) = plane_index {
        return index;
    }

    let current_index = planes.last_visible_plane;
    planes.last_visible_plane += 1;

    planes.visible_planes[current_index].height = height;
    planes.visible_planes[current_index].picnum = picnum;
    planes.visible_planes[current_index].light_level = light_level;
    planes.visible_planes[current_index].min_x = RENDER_WIDTH as i32;
    planes.visible_planes[current_index].max_x = -1;
    planes.visible_planes[current_index].top = [0xFF; RENDER_WIDTH];

    current_index
}

//
// R_PointOnSide
// Traverse BSP (sub) tree,
//  check point against partition plane.
// Returns side 0 (front) or 1 (back).
//
// TODO: Rewrite this later
fn point_on_side(point: &Point3D, node: &Node) -> usize {
    if node.dx().is_zero() {
        if point.x() <= node.x() {
            if node.dy() > RealNumber::new(0) {
                return 1;
            } else {
                return 0;
            }
        } else {
            if node.dy() < RealNumber::new(0) {
                return 1;
            } else {
                return 0;
            }
        }
    }
    if node.dy().is_zero() {
        if point.y() <= node.y() {
            if node.dx() < RealNumber::new(0) {
                return 1;
            } else {
                return 0;
            }
        } else {
            if node.dx() > RealNumber::new(0) {
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