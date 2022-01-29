use crate::wad::LumpStore;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlatNumber(pub usize);

pub struct FlatData {
    first_flat: usize,
    last_flat: usize,
    num_flats: usize,
    flat_translation: Vec<i32>,
}

impl FlatData {
    pub fn init(lumps: &LumpStore) -> Self {
        let first_flat: usize = lumps.get_lump_number("F_START")
            .unwrap()
            .into();

        let last_flat: usize = lumps.get_lump_number("F_END")
            .unwrap()
            .into();

        let num_flats = last_flat - first_flat + 1;

        let mut flat_translation = vec![0i32; num_flats+1];
        for i in 0..num_flats {
            flat_translation[i] = i as i32;
        }

        Self {
            first_flat: first_flat + 1,
            last_flat: last_flat - 1,
            num_flats,
            flat_translation
        }
    }
    pub fn get_flat_number(&self, name: &str, lumps: &LumpStore) -> Option<FlatNumber> {
        let lump_number: usize = lumps.get_lump_number(name)?.into();

        Some(FlatNumber(lump_number - self.first_flat))
    }
}