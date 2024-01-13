use crate::{cpu::sm83::SM83, mmu::MMU};

pub struct GB {
    cpu: SM83,
    pub mmu: MMU,
}

impl GB {
    fn run(&self) {
        self.cpu.run(self)
    }
}
