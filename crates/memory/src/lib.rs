// crates/memory/src/lib.rs

const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xFDFF;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF7F;

pub struct Bus {
    // Work RAM: 8KB
    wram: [u8; 0x2000],
    // High RAM: 127 bytes
    hram: [u8; 0x7F],
    // Boot ROM/Cartridge slot (Placeholder for now)
    // pub cartridge: Option<cartridge::Cartridge>,
    // I/O REGISTERS 128 bytes
    ioreg: [u8; 0x80],
}

impl Bus {
    #[must_use]
    pub const fn default() -> Self {
        Self { wram: [0; 0x2000], hram: [0; 0x7F], ioreg: [0; 0x80] }
    }

    #[must_use]
    pub const fn read8(&self, addr: u16) -> u8 {
        match addr {
            // WORK RAM & ECHO RAM
            WRAM_START..=WRAM_END => {
                let index: usize = (addr & 0x1FFF) as usize;
                self.wram[index]
            }

            // IO Register Stubs
            IO_START..=IO_END => {
                let index: usize = (addr - IO_START) as usize;
                self.ioreg[index]
            }

            // HIGH RAM
            HRAM_START..=HRAM_END => {
                let index: usize = (addr - HRAM_START) as usize;
                self.hram[index]
            }

            _ => 0xFF,
        }
    }

    pub const fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            // WORK RAM & Echo Ram
            WRAM_START..=WRAM_END => {
                let index: usize = (addr & 0x1FFF) as usize;
                self.wram[index] = val;
            }

            // IO Register Stubs
            IO_START..=IO_END => {
                let index: usize = (addr - IO_START) as usize;
                self.ioreg[index] = val;
            }

            // HIGH RAM
            HRAM_START..=HRAM_END => {
                let index: usize = (addr - HRAM_START) as usize;
                self.hram[index] = val;
            }

            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wram_bounds() {
        let mut bus: Bus = Bus::default();

        bus.write8(WRAM_START, 0xAA);
        let check: u8 = bus.read8(WRAM_START);
        assert_eq!(check, 0xAA);

        bus.write8(0xDFFF, 0xBB);
        let check: u8 = bus.read8(0xDFFF);
        assert_eq!(check, 0xBB);
    }

    #[test]
    fn test_echo_ram_mirroring() {
        let mut bus: Bus = Bus::default();

        bus.write8(0xC005, 0x55);
        assert_eq!(bus.read8(0xE005), 0x55);
    }

    #[test]
    fn test_hram_bounds() {
        let mut bus: Bus = Bus::default();

        bus.write8(HRAM_START, 0xCC);
        assert_eq!(bus.read8(HRAM_START), 0xCC);

        bus.write8(HRAM_END, 0xDD);
        assert_eq!(bus.read8(HRAM_END), 0xDD);
    }

    #[test]
    fn test_unmapped_and_readonly() {
        let mut bus: Bus = Bus::default();
        assert_eq!(bus.read8(0x0000), 0xFF);

        bus.write8(0x0000, 0x99);
        assert_eq!(bus.read8(0x0000), 0xFF);
    }

    #[test]
    fn test_ioreg_bounds() {
        let mut bus: Bus = Bus::default();
        bus.write8(IO_START, 0x5A);
        assert_eq!(bus.read8(IO_START), 0x5A);

        let mut bus: Bus = Bus::default();
        bus.write8(IO_END, 0x5A);
        assert_eq!(bus.read8(IO_END), 0x5A);
    }
}
