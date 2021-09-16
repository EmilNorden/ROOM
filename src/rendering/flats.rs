use crate::wad::LumpStore;

pub struct FlatData {
    first_flat: usize,
    last_flat: usize,
    num_flats: usize,
    flat_translation: Vec<i32>,
}

pub fn init_flats(lumps: &LumpStore) -> FlatData {
    let first_flat = lumps.get_lump_number("F_START")
        .unwrap()
        .into();

    let last_flat = lumps.get_lump_number("F_END")
        .unwrap()
        .into();

    let num_flats = last_flat - first_flat + 1;

    let mut flat_translation = vec![0i32; num_flats+1];
    for i in 0..num_flats {
        flat_translation[i] = i as i32;
    }

    FlatData {
        first_flat,
        last_flat,
        num_flats,
        flat_translation
    }
}