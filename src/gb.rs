use crate::{cpu::sm83::SM83, mmu::MMU};

pub struct GB {
    cpu: SM83,
    pub mmu: MMU,
}

impl GB {
    fn new() -> Self {
        GB {
            cpu: SM83::new(),
            mmu: MMU::new(),
        }
    }

    fn run(&mut self) {
        self.cpu.run(&self.mmu)
    }
}
