use std::fs;
use std::io;

// RAW Cartridge DATA
pub struct Cartridge {
    rom: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum CartridgeType {
    RomOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBattery,
    Unknown(u8),
}

impl Cartridge {
    // Load cartridge from rom
    pub fn load(path: &str) -> io::Result<Self> {
        let rom = fs::read(path)?;
        Ok(Self { rom })
    }

    // Extrat rom title
    pub fn title(&self) -> String {
        let title_bytes = &self.rom[0x0134..0x0144];
        // Convert Bytes to ASCII
        let title = String::from_utf8_lossy(title_bytes).trim_end_matches('\0').to_string();
        title
    }

    // Extract cartridge type
    pub fn cartridge_type(&self) -> CartridgeType {
        match self.rom[0x0147] {
            0x00 => CartridgeType::RomOnly,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1Ram,
            0x03 => CartridgeType::MBC1RamBattery,
            other => CartridgeType::Unknown(other),
        }
    }
}
