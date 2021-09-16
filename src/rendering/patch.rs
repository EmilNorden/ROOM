use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use std::mem::size_of;

pub struct Patch<'a> {
    data: &'a [u8],
}

impl<'a> From<&'a [u8]> for Patch<'a> {
    fn from(data: &'a [u8]) -> Self {
        Patch { data }
    }
}

impl<'a> Patch<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data
        }
    }
    pub fn width(&self) -> i32 {
        self.read_i16_at_offset(0) as i32
    }

    pub fn height(&self) -> i32 {
        self.read_i16_at_offset(2) as i32
    }

    pub fn left_offset(&self) -> i32 {
        self.read_i16_at_offset(4) as i32
    }

    pub fn top_offset(&self) -> i32 {
        self.read_i16_at_offset(6) as i32
    }

    pub fn get_column(&self, index: u64) -> PatchColumn {
        const BASE_COLUMN_OFFSET: u64 = 8;

        let column_offset = self.read_i32_at_offset(
            BASE_COLUMN_OFFSET + (index * size_of::<i32>() as u64)) as u64;

        PatchColumn::new(self.data, column_offset)
    }

    fn read_i16_at_offset(&self, offset: u64) -> i16 {
        let mut cursor = Cursor::new(self.data);
        cursor.seek(SeekFrom::Start(offset)).unwrap();
        cursor.read_i16::<LittleEndian>().unwrap()
    }

    fn read_i32_at_offset(&self, offset: u64) -> i32 {
        let mut cursor = Cursor::new(self.data);
        cursor.seek(SeekFrom::Start(offset)).unwrap();
        cursor.read_i32::<LittleEndian>().unwrap()
    }
}

pub struct PatchColumn<'a> {
    data: &'a [u8],
    column_offset: u64,
}


impl<'a> PatchColumn<'a> {
    pub fn new(data: &'a [u8], column_offset: u64) -> Self {
        Self { data, column_offset }
    }

    pub fn posts(&self) -> PostIterator {
        PostIterator { data: self.data, current_offset: self.column_offset as usize }
    }
}

pub struct Post<'a> {
    data: &'a [u8],
    offset: usize,
}

impl Post<'_> {
    pub fn top_delta(&self) -> i32 {
        self.data[self.offset] as i32
    }

    pub fn length(&self) -> i32 {
        self.data[self.offset + 1] as i32
    }

    pub fn data(&self) -> &[u8] {
        let length = self.length() as usize;
        &self.data[(self.offset + 3)..(self.offset + 3 + length)]
    }
}

pub struct PostIterator<'a> {
    data: &'a [u8],
    current_offset: usize,
}

impl<'a> Iterator for PostIterator<'a> {
    type Item = Post<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data[self.current_offset] == 0xff {
            return None;
        }

        let data_length = self.data[self.current_offset + 1];
        let total_length = data_length + 4; // post contains 4 bytes other than the actual data

        let current_offset = self.current_offset;
        self.current_offset += total_length as usize;

        Some(Post {
            data: self.data,
            offset: current_offset,
        })
    }
}