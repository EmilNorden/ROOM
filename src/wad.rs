use std::fs::File;
use serde::{Serialize, Deserialize};
use bincode::Options;
use std::io::{SeekFrom, Seek, Read, Cursor};
use std::collections::HashMap;
use std::path::Path;
use std::fmt::Display;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct LumpNumber(usize);

impl LumpNumber {
    pub fn offset(self, value: usize) -> Self {
        Self { 0: self.0 + value }
    }
}

impl Into<usize> for LumpNumber {
    fn into(self) -> usize { self.0 }
}

impl Sub for LumpNumber {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

pub trait ReadWadString {
    fn read_wad_string(&mut self) -> std::io::Result<String>;
}

impl<T: AsRef<[u8]>> ReadWadString for Cursor<T> {
    fn read_wad_string(&mut self) -> std::io::Result<String> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer.into())
            .unwrap()
            .trim_matches(char::from(0)).to_string())
    }
}

pub fn from_wad_string(chars: [u8; 8]) -> String {
    String::from_utf8(chars.into())
        .unwrap()
        .trim_matches(char::from(0))
        .to_string()
}



#[derive(Deserialize)]
pub struct WadHeader {
    identification: [u8; 4],
    num_lumps: i32,
    directory_offset: i32,
}

#[derive(Deserialize)]
struct FileLump {
    offset: i32,
    size: i32,
    name: [u8; 8],
}

pub struct LumpInfo {
    name: String,
    offset: usize,
    size: usize,
}

pub struct LumpStore {
    lumps: Vec<LumpInfo>,
    // offsets: HashMap<String, LumpOffset>,
    data: Vec<u8>,
}

pub enum By<'a> {
    Name(&'a str),
    Number(LumpNumber),
}

impl LumpStore {
    pub fn new() -> Self {
        Self {
            lumps: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_file<P: AsRef<Path> + Display>(&mut self, path: P) {
        let mut file = File::open(&path).expect(&*format!("Unable to open file {}", path));
        let header: WadHeader =
            bincode::deserialize_from(&file).expect(&*format!("Unable to read file {}", path));
        if header.identification != ['I' as u8, 'W' as u8, 'A' as u8, 'D' as u8] {
            // Homebrew levels?
            if header.identification != ['P' as u8, 'W' as u8, 'A' as u8, 'D' as u8] {
                panic!("Wad file {} does not have IWAD or PWAD id", path);
            }
        }

        file.seek(SeekFrom::Start(header.directory_offset as u64)).expect("Unable to read file");

        let base_offset = self.data.len();
        for _ in 0..header.num_lumps {
            let file_lump: FileLump = bincode::deserialize_from(&file).expect("Unable to read file");

            let lump_name =
                String::from_utf8(Vec::from(file_lump.name)).unwrap()
                    .trim_matches(char::from(0))
                    .to_uppercase().to_string();

            self.lumps.push(LumpInfo {
                name: lump_name,
                offset: base_offset + file_lump.offset as usize,
                size: file_lump.size as usize,
            });
            /*self.offsets.insert(lump_name, LumpOffset {
                offset: base_offset + file_lump.offset as usize,
                size: file_lump.size as usize
            });*/
        }

        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_to_end(&mut self.data).expect("Unable to read file");
    }

    pub fn get_lump_number(&self, name: &str) -> Option<LumpNumber> {
        let upper_case_name = name.to_uppercase();
        Some(LumpNumber { 0: self.lumps.iter().position(|x| x.name == upper_case_name)? })
    }

    pub fn get_lump(&self, by: By) -> &[u8] {
        let lump = match by {
            By::Name(name) => {
                let upper_case_name = name.to_uppercase();
                self.lumps.iter().rfind(|x| x.name == upper_case_name)
                    .expect(&*format!("Lump {} not found", name))
            }
            By::Number(number) => &self.lumps[number.0]
        };

        &self.data[lump.offset..(lump.offset + lump.size)]
    }

    pub fn get_lump_cursor(&self, by: By) -> Cursor<&[u8]> {
        Cursor::new(self.get_lump(by))
    }
}