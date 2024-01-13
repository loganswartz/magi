use std::cell::RefCell;

use MemoryLocation::*;

pub struct MMU {
    // general RAM
    wram: RefCell<Vec<u8>>, // 8KB
    hram: RefCell<Vec<u8>>, // 128B
    // graphics RAM
    vram: RefCell<Vec<u8>>, // 8KB
    // I/O registers
    io: RefCell<Vec<u8>>,            // 128B
    cartridge: RefCell<Vec<u8>>,     // 16KB
    cartridge_mbc: RefCell<Vec<u8>>, // 16KB
    cartridge_ram: RefCell<Vec<u8>>, // 16KB
    oam: RefCell<Vec<u8>>,           // 160B
    ie: RefCell<Vec<u8>>,
}

// pub struct Cartridge {
//     pub rom: [u8; 16384], // 16KB
//     pub mbc: [u8; 16384], // 16KB
//     pub ram: [u8; 16384], // 16KB
// }

pub enum MemoryLocation {
    Cartridge(u16),
    CartridgeMBC(u16),
    CartridgeRAM(u16),
    VRAM(u16),
    WRAM(u16),
    EchoRAM(u16),
    OAM(u16),
    IO(u16),
    HRAM(u16),
    IE(u16),
}

impl MemoryLocation {
    fn unwrap_value(&self) -> u16 {
        match self {
            self::Cartridge(addr) => *addr,
            self::CartridgeMBC(addr) => *addr,
            self::VRAM(addr) => *addr,
            self::CartridgeRAM(addr) => *addr,
            self::WRAM(addr) => *addr,
            self::EchoRAM(addr) => *addr,
            self::OAM(addr) => *addr,
            self::HRAM(addr) => *addr,
            self::IO(addr) => *addr,
            self::IE(addr) => *addr,
        }
    }
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            wram: vec![0; 8192].into(),
            hram: vec![0; 128].into(),
            vram: vec![0; 8192].into(),
            io: vec![0; 128].into(),
            cartridge: vec![0; 16384].into(),
            cartridge_mbc: vec![0; 16384].into(),
            cartridge_ram: vec![0; 16384].into(),
            oam: vec![0; 160].into(),
            ie: vec![0].into(),
        }
    }

    fn map_register(&self, location: MemoryLocation) -> (&RefCell<Vec<u8>>, usize) {
        let register = match location {
            Cartridge(_) => &self.cartridge,
            CartridgeMBC(_) => &self.cartridge_mbc,
            VRAM(_) => &self.vram,
            CartridgeRAM(_) => &self.cartridge_ram,
            WRAM(_) => &self.wram,
            EchoRAM(_) => &self.wram,
            OAM(_) => &self.oam,
            HRAM(_) => &self.hram,
            IO(_) => &self.io,
            IE(_) => &self.ie,
        };

        (register, location.unwrap_value().into())
    }

    /// Read a byte (u8) from a memory address.
    pub fn read_byte(&self, addr: u16) -> Option<u8> {
        let location = self.get_location(addr);

        let (register, offset) = self.map_register(location);

        register.borrow().get(offset).map(|byte| *byte)
    }

    /// Read a 16-bit word (u16) from a memory address.
    pub fn read_word(&self, addr: u16) -> Option<u16> {
        let location = self.get_location(addr);

        let (register, offset) = self.map_register(location);
        let first = *register.borrow().get(offset)?;
        let second = *register.borrow().get(offset + 1)?;

        Some(u16::from_le_bytes([first, second]))
    }

    /// Write a byte (u8) to a memory address.
    pub fn write_byte(&self, addr: u16, value: u8) -> () {
        let location = self.get_location(addr);

        let (register, offset) = self.map_register(location);

        register.borrow_mut()[offset] = value;
    }

    /// Write a 16-bit word (u16) to a memory address.
    pub fn write_word(&self, addr: u16, value: u16) -> () {
        let location = self.get_location(addr);

        let (register, offset) = self.map_register(location);

        let bytes = value.to_le_bytes();

        let mut register = register.borrow_mut();
        register[offset] = bytes[0];
        register[offset + 1] = bytes[1];
    }

    pub fn get_location(&self, addr: u16) -> MemoryLocation {
        use MemoryLocation::*;

        match addr {
            0x0000..=0x3FFF => Cartridge(addr),
            0x4000..=0x7FFF => CartridgeMBC(addr ^ 0x4000),
            0x8000..=0x9FFF => VRAM(addr ^ 0x8000),
            0xA000..=0xBFFF => CartridgeRAM(addr ^ 0xA000),
            0xC000..=0xDFFF => WRAM(addr ^ 0xC000),
            0xE000..=0xFDFF => EchoRAM(addr ^ 0xE000), // echos 0xC000-0xDDFF
            0xFE00..=0xFE9F => OAM(addr ^ 0xFE00),
            0xFEA0..=0xFEFF => panic!("Invalid memory address: {:04X}", addr),
            0xFF00..=0xFF7F => IO(addr ^ 0xFF00),
            0xFF80..=0xFFFE => HRAM(addr ^ 0xFF80),
            0xFFFF => IE(0xFFFF),
        }
    }
}
