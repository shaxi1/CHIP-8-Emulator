use std::env;
#[path = "utils/rom_reader.rs"] mod rom_reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut rom_reader = rom_reader::RomReader::new(file_path.to_string());
    rom_reader.load_rom().unwrap();
    println!("Rom: {} loaded!\n", file_path);

}