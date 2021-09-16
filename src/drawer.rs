use crate::rendering::renderer::Renderer;
use crate::wad::{LumpStore, By};
use crate::rendering::patch::Patch;

// Do we even need this?
pub trait Drawer {
    fn draw(&self, renderer: &mut dyn Renderer, lumps: &LumpStore);
}

pub struct PageDrawer {
    page_name: Option<String>,
}

impl PageDrawer {
    fn draw(&self, renderer: &mut dyn Renderer, lumps: &LumpStore) {
        let patch = lumps.get_lump(By::Name(self.page_name.as_ref().expect("Page name not set"))).into();
        renderer.draw_patch(0, 0, 0, &patch);
    }
}