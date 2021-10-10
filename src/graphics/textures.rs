use crate::wad::{LumpStore, ReadWadString, By};
use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Deserialize;
use std::mem::size_of;
use std::convert::TryInto;

#[derive(Deserialize)]
struct MapTexturePatchRaw {
    originx: i16,
    originy: i16,
    patch_index: i16,
    stepdir: i16,
    colormap: i16,
}

#[derive(Deserialize)]
struct MapTextureRaw {
    name: [u8; 8],
    masked: u32,
    width: i16,
    height: i16,
    dummy: i32,
    // needed to keep struct the correct size,
    patch_count: i16,
}

struct TexturePatch {
    originx: i32,
    originy: i32,
    patch_index: usize,
}

struct Texture {
    // Keep name for switch changing, etc.
    name: String,
    width: u32,
    height: u32,

    // All the patches[patchcount]
    //  are drawn back to front into the cached texture.

    patches: Vec<TexturePatch>,
}

impl Texture {
    pub fn name(&self) -> &str { &self.name }
}

#[derive(Copy, Clone, Debug)]
pub struct TextureNumber(usize);

impl<T> From<T> for TextureNumber
    where T: TryInto<usize> {
    fn from(number: T) -> Self {
        TextureNumber(number.try_into().ok().unwrap())
    }
}

pub struct TextureData {
    patch_names: Vec<String>,
    textures: Vec<Texture>,
}

impl TextureData {
    pub fn get_texture_number<S>(&self, name: S) -> Option<TextureNumber>
        where S: AsRef<str> {
        let name_ref = name.as_ref();
        // "NoTexture" marker.
        if name_ref.starts_with("-") {
            return Some(TextureNumber(0));
        }

        for i in 0..self.textures.len() {
            if self.textures[i].name() == name_ref {
                return Some(TextureNumber(i));
            }
        }

        None
    }
    /*pub fn get_texture_number(&self, name: &str) -> Option<TextureNumber> {
        // "NoTexture" marker.
        if name.starts_with("-") {
            return Some(TextureNumber(0));
        }

        for i in 0..self.textures.len() {
            if self.textures[i].name() == name {
                return Some(TextureNumber(i));
            }
        }

        None
    }*/
}

fn read_texture<R: Read>(mut data: R) -> Texture {
    let map_texture_raw: MapTextureRaw = bincode::deserialize_from(&mut data).unwrap();
    let mut patches = Vec::<MapTexturePatchRaw>::new();
    for _ in 0..map_texture_raw.patch_count {
        patches.push(bincode::deserialize_from(&mut data).unwrap());
    }

    let texture_name =
        String::from_utf8(map_texture_raw.name.into()).unwrap().trim_matches(char::from(0)).to_string();

    Texture {
        name: texture_name,
        width: map_texture_raw.width as u32,
        height: map_texture_raw.height as u32,
        patches: patches.iter().map(|x| TexturePatch {
            originx: x.originx as i32,
            originy: x.originy as i32,
            patch_index: x.patch_index as usize,
        }).collect(),
    }
}

pub fn init_textures(lumps: &LumpStore) -> TextureData {
    let mut names = lumps.get_lump_cursor(By::Name("PNAMES"));

    let map_patch_count = names.read_u32::<LittleEndian>().unwrap();

    let mut patch_names = Vec::new();
    for _ in 0..map_patch_count {
        patch_names.push(names.read_wad_string().unwrap());
    }

    // Load the map texture definitions from textures.lmp.
    // The data is contained in one or two lumps,
    //  TEXTURE1 for shareware, plus TEXTURE2 for commercial.
    let mut map_textures = lumps.get_lump_cursor(By::Name("TEXTURE1"));

    let numtextures1 = map_textures.read_u32::<LittleEndian>().unwrap() as usize;
    let max_offset = map_textures.stream_len().unwrap();

    // TODO: Add support for TEXTURE2 (check r_data.c:468)

    let numtextures = numtextures1;

    let mut textures = Vec::new();
    let mut texturecolumnlump = Vec::new();
    let mut texturecolumnofs = Vec::new();
    let mut texturewidthmask = vec![0u32; numtextures];
    let mut textureheight = vec![0u32; numtextures];

    let mut texture_offsets = vec![0u32; numtextures];
    map_textures.read_u32_into::<LittleEndian>(&mut texture_offsets).unwrap();

    for i in 0..numtextures {
        let offset = texture_offsets[i] as u64;

        if offset > max_offset {
            panic!("Bad texture directory");
        }

        map_textures.seek(SeekFrom::Start(offset)).unwrap();

        let texture = read_texture(&mut map_textures);

        texturecolumnlump.push(Vec::<i16>::with_capacity(texture.width as usize));
        texturecolumnofs.push(Vec::<u16>::with_capacity(texture.width as usize));
        textureheight[i] = texture.height << 16;

        let mut j = 1u32;
        while j * 2 <= texture.width {
            j <<= 1;
        }

        texturewidthmask[i] = j - 1;

        textures.push(texture);
    }

    TextureData {
        patch_names,
        textures,
    }
}