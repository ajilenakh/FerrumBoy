use std::fs;
use std::io;

// RAW Cartridge DATA
pub struct Cartridge {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
}

// Cartridge TYPE
#[derive(Debug, PartialEq, Eq)]
pub enum CartridgeType {
    RomOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBattery,
    Unknown(u8),
}

// ROM SIZE & No.of ROM BANKS
#[derive(Debug, PartialEq, Eq)]
pub enum RomSize {
    KiB32,
    KiB64,
    KiB128,
    KiB256,
    KiB512,
    MiB1,
    MiB2,
    MiB4,
    MiB8,
}

// RAM SIZE
#[derive(Debug, PartialEq, Eq)]
pub enum RamSize {
    Zero,
    KiB8,
    KiB32,
    KiB64,
    KiB128,
}

impl Cartridge {
    /// Loads a cartridge from a ROM file.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file cannot be read
    /// (e.g. file does not exist or permission denied).
    pub fn load(path: &str) -> io::Result<Self> {
        let rom = fs::read(path)?;
        let mut cart = Self { rom, ram: None };
        cart.ram = match ram_size(&cart) {
            RamSize::Zero => None,
            RamSize::KiB8 => Some(vec![0; 8 * 1024]),
            RamSize::KiB32 => Some(vec![0; 32 * 1024]),
            RamSize::KiB64 => Some(vec![0; 64 * 1024]),
            RamSize::KiB128 => Some(vec![0; 128 * 1024]),
        };
        Ok(cart)
    }

    // Reads the cartridge
    pub fn read() {
        //todo
    }

    // Writes Saves on cartridge
    pub fn write() {
        //todo
    }
}

// Extract rom title
#[must_use]
pub fn title(cartridge: &Cartridge) -> String {
    let title_bytes = &cartridge.rom[0x0134..0x0144];
    // Convert Bytes to ASCII
    String::from_utf8_lossy(title_bytes).trim_end_matches('\0').to_string()
}

// Extract cartridge type
#[must_use]
pub fn cartridge_type(cartridge: &Cartridge) -> CartridgeType {
    match cartridge.rom[0x0147] {
        0x00 => CartridgeType::RomOnly,
        0x01 => CartridgeType::MBC1,
        0x02 => CartridgeType::MBC1Ram,
        0x03 => CartridgeType::MBC1RamBattery,
        other => CartridgeType::Unknown(other),
    }
}

// Extract rom size
#[must_use]
pub fn rom_size(cartridge: &Cartridge) -> (RomSize, i16) {
    match cartridge.rom[0x0148] {
        0x00 => (RomSize::KiB32, 2),
        0x01 => (RomSize::KiB64, 4),
        0x02 => (RomSize::KiB128, 8),
        0x03 => (RomSize::KiB256, 16),
        0x04 => (RomSize::KiB512, 32),
        0x05 => (RomSize::MiB1, 64),
        0x06 => (RomSize::MiB2, 128),
        0x07 => (RomSize::MiB4, 256),
        0x08 => (RomSize::MiB8, 512),
        _ => unreachable!(),
    }
}

//Extract ram size
#[must_use]
pub fn ram_size(cartridge: &Cartridge) -> RamSize {
    match cartridge.rom[0x0149] {
        0x00 => RamSize::Zero, // No RAM
        //0x01 => ,                 // Unused
        0x02 => RamSize::KiB8,   // 1 bank
        0x03 => RamSize::KiB32,  // 4 banks of 8 KiB each
        0x04 => RamSize::KiB128, // 16 banks of 8 KiB each
        0x05 => RamSize::KiB64,  // 8 banks of 8 KiB each
        _ => unreachable!(),
    }
}
