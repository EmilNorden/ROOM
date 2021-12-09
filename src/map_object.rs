use crate::types::DoomRealNum;
use crate::rendering::types::Angle;

pub trait MapObject {
    // Info for drawing: position
    fn x(&self) -> DoomRealNum;
    fn y(&self) -> DoomRealNum;
    fn z(&self) -> DoomRealNum;

    //More drawing info: to determine current sprite.
    fn angle(&self) -> Angle;
}

#[derive(Copy, Clone)]
pub struct Player {
    pub(crate) state: PlayerState,
    x: DoomRealNum,
    y: DoomRealNum,
    z: DoomRealNum,
    angle: Angle,
    extra_light: i32,
    viewz: DoomRealNum,
    fixed_color_map: i32,
}

impl MapObject for Player {
    fn x(&self) -> DoomRealNum { self.x } // real(-864)

    fn y(&self) -> DoomRealNum { self.y } // real(-96)

    fn z(&self) -> DoomRealNum { self.z }

    fn angle(&self) -> Angle { self.angle }
}

impl Player {
    pub fn extra_light(&self) -> i32 { self.extra_light }

    pub fn viewz(&self) -> DoomRealNum { self.viewz } // real(41);

    pub fn fixed_color_map(&self) -> i32 { self.fixed_color_map }
}

/*impl Player {
    pub fn x(&self) -> DoomRealNum { real(-864) }
    pub fn y(&self) -> DoomRealNum { real(-96) }
}*/

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::Dead,
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            angle: Angle::default(),
            extra_light: 0,
            viewz: Default::default(),
            fixed_color_map: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum PlayerState {
    // Playing or camping.
    Alive,
    // Dead on the ground, view follows killer.
    Dead,
    // Ready to restart/respawn???
    Reborn
}