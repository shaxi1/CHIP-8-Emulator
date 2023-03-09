use std::fs::File;
use std::io::{self, Read};

pub struct RomReader {
    pub rom : Vec<u8>,
    pub file_path : String
}

impl RomReader {
    pub fn new(file_path: String) -> RomReader {
        RomReader {
            rom: Vec::new(),
            file_path
        }
    }

    pub fn load_rom(&mut self) -> io::Result<()> {
        print!("Loading rom from: {}", self.file_path);

        let mut file = File::open(&self.file_path)?;
        file.read_to_end(&mut self.rom)?;

        Ok(())
    }
}