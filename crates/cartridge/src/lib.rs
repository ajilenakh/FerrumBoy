// crates/cartridge/src/lib.rs

use std::fs;
use std::io;

// Cartridge TYPE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartridgeType {
    RomOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBattery,
    Unknown(u8),
}
// ROM SIZE & No.of ROM BANKS
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RamSize {
    Zero,
    KiB8,
    KiB32,
    KiB64,
    KiB128,
}

// RAW Cartridge DATA
pub struct Cartridge {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    cart_type: CartridgeType,
}

impl Cartridge {
    /// Loads a cartridge from a ROM file.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file cannot be read
    /// (e.g. file does not exist or permission denied).
    pub fn load(path: &str) -> io::Result<Self> {
        let rom_bytes: Vec<u8> = fs::read(path)?;
        let cart_type: CartridgeType = cartridge_type(&rom_bytes);
        let ram: Option<Vec<u8>> = match ram_size(&rom_bytes) {
            RamSize::Zero => None,
            RamSize::KiB8 => Some(vec![0; 8 * 1024]),
            RamSize::KiB32 => Some(vec![0; 32 * 1024]),
            RamSize::KiB64 => Some(vec![0; 64 * 1024]),
            RamSize::KiB128 => Some(vec![0; 128 * 1024]),
        };
        Ok(Self { rom: rom_bytes, ram, cart_type })
    }

    // Reads the cartridge
    #[must_use]
    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            // ROM Banks Region (0x0000 - 0x7FFF)
            0x0000..=0x7FFF => {
                match self.cart_type {
                    CartridgeType::RomOnly => self.rom[addr as usize],
                    _ => {
                        // Placeholder for MBC1, MBC1 Ram etc...
                        0xFF
                    }
                }
            }

            // External Save Ram Region (0xA000 - 0xBFFF)
            0xA000..=0xBFFF => self.ram.as_ref().map_or(0xFF, |ram_vec| {
                let index: usize = (addr - 0xA000) as usize;
                ram_vec[index]
            }),
            // Fallback for any unexpected addresses
            _ => 0xFF,
        }
    }

    // Writes Saves on cartridge
    pub fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            // ROM Banks Region (0x0000 - 0x7FFF)
            0x0000..=0x7FFF => match self.cart_type {
                CartridgeType::RomOnly => {}
                CartridgeType::MBC1 => todo!(),
                CartridgeType::MBC1Ram => todo!(),
                CartridgeType::MBC1RamBattery => todo!(),
                CartridgeType::Unknown(_) => todo!(),
            },

            // External Save RAM Region (0xA000 - 0xBFFF)
            0xA000..=0xBFFF => {
                if let Some(ref mut ram_vec) = self.ram {
                    let index = (addr - 0xA000) as usize;
                    ram_vec[index] = val;
                }
                // If self.ram is None, the write is silently discarded
            }
            _ => {}
        }
    }
}

// --- Placeholder metadata parser functions updated to take raw byte slices ---
// Extract rom title
#[must_use]
pub fn title(rom_bytes: &[u8]) -> String {
    let title_bytes: &[u8] = &rom_bytes[0x0134..0x0144];
    // Convert Bytes to ASCII
    String::from_utf8_lossy(title_bytes).trim_end_matches('\0').to_string()
}

// Extract cartridge type
#[must_use]
pub fn cartridge_type(rom_bytes: &[u8]) -> CartridgeType {
    match &rom_bytes[0x0147] {
        0x00 => CartridgeType::RomOnly,
        0x01 => CartridgeType::MBC1,
        0x02 => CartridgeType::MBC1Ram,
        0x03 => CartridgeType::MBC1RamBattery,
        other => CartridgeType::Unknown(*other),
    }
}

// Extract rom size
#[must_use]
pub fn rom_size(rom_bytes: &[u8]) -> (RomSize, i16) {
    match rom_bytes[0x0148] {
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
pub fn ram_size(rom_bytes: &[u8]) -> RamSize {
    match rom_bytes[0x0149] {
        0x00 => RamSize::Zero, // No RAM
        //0x01 => ,                 // Unused
        0x02 => RamSize::KiB8,   // 1 bank
        0x03 => RamSize::KiB32,  // 4 banks of 8 KiB each
        0x04 => RamSize::KiB128, // 16 banks of 8 KiB each
        0x05 => RamSize::KiB64,  // 8 banks of 8 KiB each
        _ => unreachable!(),
    }
}
