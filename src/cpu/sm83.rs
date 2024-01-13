use crate::{
    cpu::{
        opcodes::{Opcode, OperationsMap, SM83_OPERATIONS},
        registers::SM83RegisterBank,
    },
    mmu::MMU,
};

/// The CPU of the GameBoy, a Sharp SM83.
#[derive(Debug, Clone)]
pub struct SM83 {
    // CPU clock
    m: u8,
    t: u8,

    pub registers: SM83RegisterBank,
    operations: OperationsMap<Self>,
}

impl SM83 {
    pub fn new() -> Self {
        SM83 {
            m: 0,
            t: 0,
            registers: SM83RegisterBank::new(),
            operations: SM83_OPERATIONS.clone(),
        }
    }

    pub fn step(&mut self) {}

    pub fn reset(&mut self) {}

    pub fn run(&mut self, mmu: &MMU) {
        loop {
            let Some(code) = mmu.read_byte(self.registers.pc) else {
                println!("Failed to read byte at address: {:04X}", self.registers.pc);
                continue;
            };

            let Some(opcode) = self.operations.get(&code) else {
                panic!("Unknown opcode: {:02X}", code);
            };
            let cycles = opcode.cycle_count();

            match opcode {
                Opcode::Unary(operation, _) => {
                    operation(self, mmu);
                }
                Opcode::Binary(operation, _) => {
                    let addr = self.registers.pc + 1;
                    let Some(immediate) = mmu.read_byte(addr) else {
                        println!("Failed to read byte at address: {:04X}", addr);
                        continue;
                    };

                    operation(self, mmu, immediate);
                }
                Opcode::Ternary(operation, _) => {
                    let addr = self.registers.pc + 1;
                    let Some(immediate_a) = mmu.read_byte(addr) else {
                        println!("Failed to read byte at address: {:04X}", addr);
                        continue;
                    };
                    let Some(immediate_b) = mmu.read_byte(addr + 1) else {
                        println!("Failed to read byte at address: {:04X}", addr);
                        continue;
                    };

                    operation(self, mmu, immediate_a, immediate_b);
                }
            };

            // increment our clock registers
            self.registers.m = cycles;
            self.registers.t = cycles * 4;
        }
    }
}
