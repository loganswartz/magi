pub struct MMU {
    // general RAM
    wram: Vec<u8>, // 8KB
    hram: Vec<u8>, // 128B
    // graphics RAM
    vram: Vec<u8>, // 8KB
    // I/O registers
    io: Vec<u8>,            // 128B
    cartridge: Vec<u8>,     // 16KB
    cartridge_mbc: Vec<u8>, // 16KB
    cartridge_ram: Vec<u8>, // 16KB
    oam: Vec<u8>,           // 160B
    ie: Vec<u8>,
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
    fn to_register(&self, mmu: &MMU) -> (&Vec<u8>, usize) {
        let register = match self {
            self::Cartridge(_) => &mmu.cartridge,
            self::CartridgeMBC(_) => &mmu.cartridge_mbc,
            self::VRAM(_) => &mmu.vram,
            self::CartridgeRAM(_) => &mmu.cartridge_ram,
            self::WRAM(_) => &mmu.wram,
            self::EchoRAM(_) => &mmu.wram,
            self::OAM(_) => &mmu.oam,
            self::HRAM(_) => &mmu.hram,
            self::IO(_) => &mmu.io,
            self::IE(_) => &mmu.ie,
        };

        (register, self.unwrap_value().into())
    }

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
use MemoryLocation::*;

impl MMU {
    pub fn new() -> Self {
        MMU {
            wram: vec![0; 8192],
            hram: vec![0; 128],
            vram: vec![0; 8192],
            io: vec![0; 128],
            cartridge: vec![0; 16384],
            cartridge_mbc: vec![0; 16384],
            cartridge_ram: vec![0; 16384],
            oam: vec![0; 160],
            ie: vec![0],
        }
    }

    fn map_register(&self, location: &MemoryLocation) -> &Vec<u8> {
        match location {
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
        }
    }

    /// Read a byte (u8) from a memory address.
    pub fn read_byte(&self, addr: u16) -> Option<&u8> {
        let location = self.get_location(addr);

        let (register, offset) = location.to_register(self);

        register.get(offset)
    }

    /// Read a 16-bit word (u16) from a memory address.
    pub fn read_word(&self, addr: u16) -> Option<&u16> {
        let location = self.get_location(addr);

        let (register, offset) = location.to_register(self);
        let first = *register.get(offset)?;
        let second = *register.get(offset + 1)?;

        Some(&u16::from_le_bytes([first, second]))
    }

    /// Write a byte (u8) to a memory address.
    pub fn write_byte(&self, addr: u16, value: u8) -> () {
        let location = self.get_location(addr);

        let (register, offset) = location.to_register(self);

        register[offset] = value;
    }

    /// Write a 16-bit word (u16) to a memory address.
    pub fn write_word(&self, addr: u16, value: u16) -> () {
        let location = self.get_location(addr);

        let (register, offset) = location.to_register(self);

        let bytes = value.to_le_bytes();
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
