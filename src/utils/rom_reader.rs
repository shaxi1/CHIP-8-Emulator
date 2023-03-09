use std::fs::File;
use std::io::{self, Read};

pub struct RomReader {
    pub rom : Vec<u8>,
    pub file_path : String,
    pub rom_size : usize
}

impl RomReader {
    pub fn new(file_path: String) -> RomReader {
        RomReader {
            rom: Vec::new(),
            file_path,
            rom_size: 0
        }
    }

    pub fn load_rom(&mut self) -> io::Result<()> {
        print!("Loading rom from: {}\n", self.file_path);

        let mut file = File::open(&self.file_path)?;
        file.read_to_end(&mut self.rom)?;

        self.rom_size = self.rom.len();

        Ok(())
    }
}