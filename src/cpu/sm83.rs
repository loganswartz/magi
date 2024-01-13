use crate::{
    cpu::{
        opcodes::{Opcode, OperationsMap, SM83_OPERATIONS},
        registers::SM83RegisterBank,
    },
    gb::GB,
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

    pub fn run(&mut self, gb: &GB) {
        loop {
            let Some(code) = gb.mmu.read_byte(self.registers.pc) else {
                println!("Failed to read byte at address: {:04X}", self.registers.pc);
                continue;
            };

            let Some(opcode) = self.operations.get(&code) else {
                panic!("Unknown opcode: {:02X}", code);
            };

            match opcode {
                Opcode::Unary(operation, _) => {
                    operation(&mut self);
                }
                Opcode::Binary(operation, _) => {
                    let addr = self.registers.pc + 1;
                    let Some(immediate) = gb.mmu.read_byte(addr) else {
                        println!("Failed to read byte at address: {:04X}", addr);
                        continue;
                    };

                    operation(&mut self, *immediate);
                }
                Opcode::Ternary(operation, _) => {
                    let addr = self.registers.pc + 1;
                    let Some(immediateA) = gb.mmu.read_byte(addr) else {
                        println!("Failed to read byte at address: {:04X}", addr);
                        continue;
                    };
                    let Some(immediateB) = gb.mmu.read_byte(addr + 1) else {
                        println!("Failed to read byte at address: {:04X}", addr);
                        continue;
                    };

                    operation(&mut self, *immediateA, *immediateB);
                }
            };

            // increment our clock registers
            let cycles = opcode.cycle_count();
            self.registers.m = cycles;
            self.registers.t = cycles * 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let mut cpu = SM83::new();
        let opcode = 0x00u8;
        let operation = SM83_OPERATIONS.get(&opcode).unwrap();
        match operation {
            Opcode::Unary(op, _) => op(&mut cpu),
            _ => panic!("Expected unary operation"),
        }
    }
}
