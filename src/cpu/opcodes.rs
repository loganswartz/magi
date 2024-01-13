use crate::{
    cpu::{registers::Flag, sm83::SM83},
    mmu::MMU,
};

use lazy_static::lazy_static;
use paste::paste;
use std::collections::HashMap;

/// An individual instruction for the SM83.
///
/// The first element of the enum is the handler function for the operation,
/// and the second element is the number of cycles the operation takes.
#[derive(Debug, Clone, Copy)]
pub enum Opcode<CPU> {
    Unary(fn(&mut CPU, &MMU), u8),
    Binary(fn(&mut CPU, &MMU, u8), u8),
    Ternary(fn(&mut CPU, &MMU, u8, u8), u8),
}

impl<T> Opcode<T> {
    pub fn cycle_count(&self) -> u8 {
        match self {
            Opcode::Unary(_, cycles) => *cycles,
            Opcode::Binary(_, cycles) => *cycles,
            Opcode::Ternary(_, cycles) => *cycles,
        }
    }
}

pub type OperationsMap<CPU> = HashMap<u8, Opcode<CPU>>;

// trait Op<'a, CPU, const OPERANDS: usize> {
//     fn execute(&self, cpu: &CPU, operands: [u8; OPERANDS]) -> ();
// }
//
// struct Nop;
//
// impl Op<'static, SM83, 0> for Nop {
//     fn execute(&self, cpu: &SM83, _: [u8; 0]) -> () {
//         //
//     }
// }

fn nop(_: &mut SM83, _: &MMU) {
    //
}

fn increment_hl_addr(cpu: &mut SM83, mmu: &MMU) {
    cpu.registers.flags.clear();
    let addr = cpu.registers.hl();

    mmu.write_byte(
        addr,
        match mmu
            .read_byte(addr)
            .expect("should be able to read byte")
            .checked_add(1)
        {
            Some(0) => {
                cpu.registers.flags.set(Flag::Zero);
                0
            }
            Some(val) => val,
            None => {
                cpu.registers.flags.set(Flag::Carry);
                return;
            }
        },
    )
}

macro_rules! increment8 {
    ($reg:ident) => {
        paste! {
            fn [<increment_ $reg>](cpu: &mut SM83, _: &MMU) {
                cpu.registers.flags.clear();

                cpu.registers.$reg = match cpu.registers.$reg.checked_add(1) {
                    Some(0) => {
                        cpu.registers.flags.set(Flag::Zero);
                        0
                    },
                    Some(val) => val,
                    None => {
                        cpu.registers.flags.set(Flag::Carry);
                        return;
                    }
                }
            }
        }
    };
}

macro_rules! increment16 {
    ($regA:ident, $regB:ident) => {
        paste! {
            fn [<increment_ $regA $regB>](cpu: &mut SM83, _: &MMU) {
                let combined = cpu.registers.combined(cpu.registers.$regA, cpu.registers.$regB);
                let result = match combined.checked_add(1) {
                    Some(0) => {
                        cpu.registers.flags.set(Flag::Zero);
                        0
                    },
                    Some(val) => val,
                    None => {
                        cpu.registers.flags.set(Flag::Carry);
                        return;
                    }
                };

                let [a, b] = cpu.registers.split(result);
                cpu.registers.$regA = a;
                cpu.registers.$regB = b;
            }
        }
    };
}

macro_rules! add_to_hl {
    ($regA:ident, $regB:ident) => {
        paste! {
            fn [<add_ $regA $regB _to_hl>](cpu: &mut SM83, _: &MMU) {
                let value = match cpu.registers.hl().checked_add(cpu.registers.hl()) {
                    Some(0) => {
                        cpu.registers.flags.set(Flag::Zero);
                        0
                    },
                    Some(val) => val,
                    None => {
                        cpu.registers.flags.set(Flag::Carry);
                        return;
                    }
                };

                cpu.registers.set_hl(value);
            }
        }
    };
}

macro_rules! decrement8 {
    ($reg:ident) => {
        paste! {
            fn [<decrement_ $reg>](cpu: &mut SM83, _: &MMU) {
                cpu.registers.flags.clear();

                cpu.registers.$reg = match cpu.registers.$reg.checked_sub(1) {
                    Some(0) => {
                        cpu.registers.flags.set(Flag::Zero);
                        0
                    },
                    Some(val) => val,
                    None => {
                        cpu.registers.flags.set(Flag::Carry);
                        return;
                    }
                }
            }
        }
    };
}

macro_rules! decrement16 {
    ($regA:ident, $regB:ident) => {
        paste! {
            fn [<decrement_ $regA $regB>](cpu: &mut SM83, _: &MMU) {
                let combined = cpu.registers.combined(cpu.registers.$regA, cpu.registers.$regB);
                let result = match combined.checked_sub(1) {
                    Some(0) => {
                        cpu.registers.flags.set(Flag::Zero);
                        0
                    },
                    Some(val) => val,
                    None => {
                        cpu.registers.flags.set(Flag::Carry);
                        return;
                    }
                };

                let [a, b] = cpu.registers.split(result);
                cpu.registers.$regA = a;
                cpu.registers.$regB = b;
            }
        }
    };
}

macro_rules! load_immediate8 {
    ($reg:ident) => {
        paste! {
            fn [<load_immediate_into_ $reg>](cpu: &mut SM83, _: &MMU, immediate: u8) {
                cpu.registers.$reg = immediate;
            }
        }
    };
}

macro_rules! load_immediate16 {
    ($regA:ident,$regB:ident) => {
        paste! {
            fn [<load_immediate_into_ $regA $regB>](cpu: &mut SM83, _: &MMU, a: u8, b: u8) {
                cpu.registers.$regA = a;
                cpu.registers.$regB = b;
            }
        }
    };
}

macro_rules! load_reg_into_reg16_addr {
    ($source:ident,$destA:ident,$destB:ident) => {
        paste! {
            fn [<load_ $source _into_ $destA $destB _address>](cpu: &mut SM83, mmu: &MMU) {
                let addr = cpu.registers.combined(cpu.registers.$destA, cpu.registers.$destB);

                mmu.write_byte(addr, cpu.registers.$source);
            }
        }
    };
}

increment8!(a);
increment8!(b);
increment8!(c);
increment8!(d);
increment8!(e);
increment8!(h);
increment8!(l);

increment16!(b, c);
increment16!(d, e);
increment16!(h, l);
// increment16!(s, p);

decrement8!(b);

decrement16!(b, c);
decrement16!(d, e);
decrement16!(h, l);

add_to_hl!(b, c);
add_to_hl!(d, e);
add_to_hl!(h, l);
// add_to_hl!(s, p);

load_immediate8!(b);
load_immediate16!(b, c);
load_reg_into_reg16_addr!(a, b, c);

fn rotate_a_left_with_carry(_cpu: &mut SM83, _mmu: &MMU) {
    //
}

fn load_sp_into_immediate_address(_cpu: &mut SM83, _mmu: &MMU, _a: u8, _b: u8) {
    //
}

fn stop(_cpu: &mut SM83, _mmu: &MMU, _: u8) {
    //
}

lazy_static! {
    pub static ref SM83_OPERATIONS: OperationsMap<SM83> = HashMap::from([
        (0x00u8, Opcode::Unary(nop, 1)),
        (0x01u8, Opcode::Ternary(load_immediate_into_bc, 1)),
        (0x02u8, Opcode::Unary(load_a_into_bc_address, 1)),
        (0x03u8, Opcode::Unary(increment_bc, 1)),
        (0x04u8, Opcode::Unary(increment_b, 1)),
        (0x05u8, Opcode::Unary(decrement_b, 1)),
        (0x06u8, Opcode::Binary(load_immediate_into_b, 1)),
        (0x07u8, Opcode::Unary(rotate_a_left_with_carry, 1)),
        (0x08u8, Opcode::Ternary(load_sp_into_immediate_address, 1)),
        (0x09u8, Opcode::Unary(add_bc_to_hl, 1)),
        (0x0Au8, Opcode::Unary(nop, 1)),
        (0x0Bu8, Opcode::Unary(decrement_bc, 1)),
        (0x0Cu8, Opcode::Unary(increment_c, 1)),
        (0x0Du8, Opcode::Unary(nop, 1)),
        (0x0Eu8, Opcode::Unary(nop, 1)),
        (0x0Fu8, Opcode::Unary(nop, 1)),
        (0x10u8, Opcode::Binary(stop, 1)),
        (0x11u8, Opcode::Unary(nop, 1)),
        (0x12u8, Opcode::Unary(nop, 1)),
        (0x13u8, Opcode::Unary(increment_de, 1)),
        (0x14u8, Opcode::Unary(increment_d, 1)),
        (0x15u8, Opcode::Unary(nop, 1)),
        (0x16u8, Opcode::Unary(nop, 1)),
        (0x17u8, Opcode::Unary(nop, 1)),
        (0x18u8, Opcode::Unary(nop, 1)),
        (0x19u8, Opcode::Unary(add_de_to_hl, 1)),
        (0x1Au8, Opcode::Unary(nop, 1)),
        (0x1Bu8, Opcode::Unary(decrement_de, 1)),
        (0x1Cu8, Opcode::Unary(increment_e, 1)),
        (0x1Du8, Opcode::Unary(nop, 1)),
        (0x1Eu8, Opcode::Unary(nop, 1)),
        (0x1Fu8, Opcode::Unary(nop, 1)),
        (0x20u8, Opcode::Unary(nop, 1)),
        (0x21u8, Opcode::Unary(nop, 1)),
        (0x22u8, Opcode::Unary(nop, 1)),
        (0x23u8, Opcode::Unary(increment_hl, 1)),
        (0x24u8, Opcode::Unary(increment_h, 1)),
        (0x25u8, Opcode::Unary(nop, 1)),
        (0x26u8, Opcode::Unary(nop, 1)),
        (0x27u8, Opcode::Unary(nop, 1)),
        (0x28u8, Opcode::Unary(nop, 1)),
        (0x29u8, Opcode::Unary(add_hl_to_hl, 1)),
        (0x2Au8, Opcode::Unary(nop, 1)),
        (0x2Bu8, Opcode::Unary(decrement_hl, 1)),
        (0x2Cu8, Opcode::Unary(increment_l, 1)),
        (0x2Du8, Opcode::Unary(nop, 1)),
        (0x2Eu8, Opcode::Unary(nop, 1)),
        (0x2Fu8, Opcode::Unary(nop, 1)),
        (0x30u8, Opcode::Unary(nop, 1)),
        (0x31u8, Opcode::Unary(nop, 1)),
        (0x32u8, Opcode::Unary(nop, 1)),
        (0x33u8, Opcode::Unary(nop, 1)),
        (0x34u8, Opcode::Unary(increment_hl_addr, 1)),
        (0x35u8, Opcode::Unary(nop, 1)),
        (0x36u8, Opcode::Unary(nop, 1)),
        (0x37u8, Opcode::Unary(nop, 1)),
        (0x38u8, Opcode::Unary(nop, 1)),
        (0x39u8, Opcode::Unary(nop, 1)),
        (0x3Au8, Opcode::Unary(nop, 1)),
        (0x3Bu8, Opcode::Unary(nop, 1)),
        (0x3Cu8, Opcode::Unary(increment_a, 1)),
        (0x3Du8, Opcode::Unary(nop, 1)),
        (0x3Eu8, Opcode::Unary(nop, 1)),
        (0x3Fu8, Opcode::Unary(nop, 1)),
        (0x40u8, Opcode::Unary(nop, 1)),
        (0x41u8, Opcode::Unary(nop, 1)),
        (0x42u8, Opcode::Unary(nop, 1)),
        (0x43u8, Opcode::Unary(nop, 1)),
        (0x44u8, Opcode::Unary(nop, 1)),
        (0x45u8, Opcode::Unary(nop, 1)),
        (0x46u8, Opcode::Unary(nop, 1)),
        (0x47u8, Opcode::Unary(nop, 1)),
        (0x48u8, Opcode::Unary(nop, 1)),
        (0x49u8, Opcode::Unary(nop, 1)),
        (0x4Au8, Opcode::Unary(nop, 1)),
        (0x4Bu8, Opcode::Unary(nop, 1)),
        (0x4Cu8, Opcode::Unary(nop, 1)),
        (0x4Du8, Opcode::Unary(nop, 1)),
        (0x4Eu8, Opcode::Unary(nop, 1)),
        (0x4Fu8, Opcode::Unary(nop, 1)),
        (0x50u8, Opcode::Unary(nop, 1)),
        (0x51u8, Opcode::Unary(nop, 1)),
        (0x52u8, Opcode::Unary(nop, 1)),
        (0x53u8, Opcode::Unary(nop, 1)),
        (0x54u8, Opcode::Unary(nop, 1)),
        (0x55u8, Opcode::Unary(nop, 1)),
        (0x56u8, Opcode::Unary(nop, 1)),
        (0x57u8, Opcode::Unary(nop, 1)),
        (0x58u8, Opcode::Unary(nop, 1)),
        (0x59u8, Opcode::Unary(nop, 1)),
        (0x5Au8, Opcode::Unary(nop, 1)),
        (0x5Bu8, Opcode::Unary(nop, 1)),
        (0x5Cu8, Opcode::Unary(nop, 1)),
        (0x5Du8, Opcode::Unary(nop, 1)),
        (0x5Eu8, Opcode::Unary(nop, 1)),
        (0x5Fu8, Opcode::Unary(nop, 1)),
        (0x60u8, Opcode::Unary(nop, 1)),
        (0x61u8, Opcode::Unary(nop, 1)),
        (0x62u8, Opcode::Unary(nop, 1)),
        (0x63u8, Opcode::Unary(nop, 1)),
        (0x64u8, Opcode::Unary(nop, 1)),
        (0x65u8, Opcode::Unary(nop, 1)),
        (0x66u8, Opcode::Unary(nop, 1)),
        (0x67u8, Opcode::Unary(nop, 1)),
        (0x68u8, Opcode::Unary(nop, 1)),
        (0x69u8, Opcode::Unary(nop, 1)),
        (0x6Au8, Opcode::Unary(nop, 1)),
        (0x6Bu8, Opcode::Unary(nop, 1)),
        (0x6Cu8, Opcode::Unary(nop, 1)),
        (0x6Du8, Opcode::Unary(nop, 1)),
        (0x6Eu8, Opcode::Unary(nop, 1)),
        (0x6Fu8, Opcode::Unary(nop, 1)),
        (0x70u8, Opcode::Unary(nop, 1)),
        (0x71u8, Opcode::Unary(nop, 1)),
        (0x72u8, Opcode::Unary(nop, 1)),
        (0x73u8, Opcode::Unary(nop, 1)),
        (0x74u8, Opcode::Unary(nop, 1)),
        (0x75u8, Opcode::Unary(nop, 1)),
        (0x76u8, Opcode::Unary(nop, 1)),
        (0x77u8, Opcode::Unary(nop, 1)),
        (0x78u8, Opcode::Unary(nop, 1)),
        (0x79u8, Opcode::Unary(nop, 1)),
        (0x7Au8, Opcode::Unary(nop, 1)),
        (0x7Bu8, Opcode::Unary(nop, 1)),
        (0x7Cu8, Opcode::Unary(nop, 1)),
        (0x7Du8, Opcode::Unary(nop, 1)),
        (0x7Eu8, Opcode::Unary(nop, 1)),
        (0x7Fu8, Opcode::Unary(nop, 1)),
        (0x80u8, Opcode::Unary(nop, 1)),
        (0x81u8, Opcode::Unary(nop, 1)),
        (0x82u8, Opcode::Unary(nop, 1)),
        (0x83u8, Opcode::Unary(nop, 1)),
        (0x84u8, Opcode::Unary(nop, 1)),
        (0x85u8, Opcode::Unary(nop, 1)),
        (0x86u8, Opcode::Unary(nop, 1)),
        (0x87u8, Opcode::Unary(nop, 1)),
        (0x88u8, Opcode::Unary(nop, 1)),
        (0x89u8, Opcode::Unary(nop, 1)),
        (0x8Au8, Opcode::Unary(nop, 1)),
        (0x8Bu8, Opcode::Unary(nop, 1)),
        (0x8Cu8, Opcode::Unary(nop, 1)),
        (0x8Du8, Opcode::Unary(nop, 1)),
        (0x8Eu8, Opcode::Unary(nop, 1)),
        (0x8Fu8, Opcode::Unary(nop, 1)),
        (0x90u8, Opcode::Unary(nop, 1)),
        (0x91u8, Opcode::Unary(nop, 1)),
        (0x92u8, Opcode::Unary(nop, 1)),
        (0x93u8, Opcode::Unary(nop, 1)),
        (0x94u8, Opcode::Unary(nop, 1)),
        (0x95u8, Opcode::Unary(nop, 1)),
        (0x96u8, Opcode::Unary(nop, 1)),
        (0x97u8, Opcode::Unary(nop, 1)),
        (0x98u8, Opcode::Unary(nop, 1)),
        (0x99u8, Opcode::Unary(nop, 1)),
        (0x9Au8, Opcode::Unary(nop, 1)),
        (0x9Bu8, Opcode::Unary(nop, 1)),
        (0x9Cu8, Opcode::Unary(nop, 1)),
        (0x9Du8, Opcode::Unary(nop, 1)),
        (0x9Eu8, Opcode::Unary(nop, 1)),
        (0x9Fu8, Opcode::Unary(nop, 1)),
        (0xA0u8, Opcode::Unary(nop, 1)),
        (0xA1u8, Opcode::Unary(nop, 1)),
        (0xA2u8, Opcode::Unary(nop, 1)),
        (0xA3u8, Opcode::Unary(nop, 1)),
        (0xA4u8, Opcode::Unary(nop, 1)),
        (0xA5u8, Opcode::Unary(nop, 1)),
        (0xA6u8, Opcode::Unary(nop, 1)),
        (0xA7u8, Opcode::Unary(nop, 1)),
        (0xA8u8, Opcode::Unary(nop, 1)),
        (0xA9u8, Opcode::Unary(nop, 1)),
        (0xAAu8, Opcode::Unary(nop, 1)),
        (0xABu8, Opcode::Unary(nop, 1)),
        (0xACu8, Opcode::Unary(nop, 1)),
        (0xADu8, Opcode::Unary(nop, 1)),
        (0xAEu8, Opcode::Unary(nop, 1)),
        (0xAFu8, Opcode::Unary(nop, 1)),
        (0xB0u8, Opcode::Unary(nop, 1)),
        (0xB1u8, Opcode::Unary(nop, 1)),
        (0xB2u8, Opcode::Unary(nop, 1)),
        (0xB3u8, Opcode::Unary(nop, 1)),
        (0xB4u8, Opcode::Unary(nop, 1)),
        (0xB5u8, Opcode::Unary(nop, 1)),
        (0xB6u8, Opcode::Unary(nop, 1)),
        (0xB7u8, Opcode::Unary(nop, 1)),
        (0xB8u8, Opcode::Unary(nop, 1)),
        (0xB9u8, Opcode::Unary(nop, 1)),
        (0xBAu8, Opcode::Unary(nop, 1)),
        (0xBBu8, Opcode::Unary(nop, 1)),
        (0xBCu8, Opcode::Unary(nop, 1)),
        (0xBDu8, Opcode::Unary(nop, 1)),
        (0xBEu8, Opcode::Unary(nop, 1)),
        (0xBFu8, Opcode::Unary(nop, 1)),
        (0xC0u8, Opcode::Unary(nop, 1)),
        (0xC1u8, Opcode::Unary(nop, 1)),
        (0xC2u8, Opcode::Unary(nop, 1)),
        (0xC3u8, Opcode::Unary(nop, 1)),
        (0xC4u8, Opcode::Unary(nop, 1)),
        (0xC5u8, Opcode::Unary(nop, 1)),
        (0xC6u8, Opcode::Unary(nop, 1)),
        (0xC7u8, Opcode::Unary(nop, 1)),
        (0xC8u8, Opcode::Unary(nop, 1)),
        (0xC9u8, Opcode::Unary(nop, 1)),
        (0xCAu8, Opcode::Unary(nop, 1)),
        (0xCBu8, Opcode::Unary(nop, 1)),
        (0xCCu8, Opcode::Unary(nop, 1)),
        (0xCDu8, Opcode::Unary(nop, 1)),
        (0xCEu8, Opcode::Unary(nop, 1)),
        (0xCFu8, Opcode::Unary(nop, 1)),
        (0xD0u8, Opcode::Unary(nop, 1)),
        (0xD1u8, Opcode::Unary(nop, 1)),
        (0xD2u8, Opcode::Unary(nop, 1)),
        (0xD3u8, Opcode::Unary(nop, 1)),
        (0xD4u8, Opcode::Unary(nop, 1)),
        (0xD5u8, Opcode::Unary(nop, 1)),
        (0xD6u8, Opcode::Unary(nop, 1)),
        (0xD7u8, Opcode::Unary(nop, 1)),
        (0xD8u8, Opcode::Unary(nop, 1)),
        (0xD9u8, Opcode::Unary(nop, 1)),
        (0xDAu8, Opcode::Unary(nop, 1)),
        (0xDBu8, Opcode::Unary(nop, 1)),
        (0xDCu8, Opcode::Unary(nop, 1)),
        (0xDDu8, Opcode::Unary(nop, 1)),
        (0xDEu8, Opcode::Unary(nop, 1)),
        (0xDFu8, Opcode::Unary(nop, 1)),
        (0xE0u8, Opcode::Unary(nop, 1)),
        (0xE1u8, Opcode::Unary(nop, 1)),
        (0xE2u8, Opcode::Unary(nop, 1)),
        (0xE3u8, Opcode::Unary(nop, 1)),
        (0xE4u8, Opcode::Unary(nop, 1)),
        (0xE5u8, Opcode::Unary(nop, 1)),
        (0xE6u8, Opcode::Unary(nop, 1)),
        (0xE7u8, Opcode::Unary(nop, 1)),
        (0xE8u8, Opcode::Unary(nop, 1)),
        (0xE9u8, Opcode::Unary(nop, 1)),
        (0xEAu8, Opcode::Unary(nop, 1)),
        (0xEBu8, Opcode::Unary(nop, 1)),
        (0xECu8, Opcode::Unary(nop, 1)),
        (0xEDu8, Opcode::Unary(nop, 1)),
        (0xEEu8, Opcode::Unary(nop, 1)),
        (0xEFu8, Opcode::Unary(nop, 1)),
        (0xF0u8, Opcode::Unary(nop, 1)),
        (0xF1u8, Opcode::Unary(nop, 1)),
        (0xF2u8, Opcode::Unary(nop, 1)),
        (0xF3u8, Opcode::Unary(nop, 1)),
        (0xF4u8, Opcode::Unary(nop, 1)),
        (0xF5u8, Opcode::Unary(nop, 1)),
        (0xF6u8, Opcode::Unary(nop, 1)),
        (0xF7u8, Opcode::Unary(nop, 1)),
        (0xF8u8, Opcode::Unary(nop, 1)),
        (0xF9u8, Opcode::Unary(nop, 1)),
        (0xFAu8, Opcode::Unary(nop, 1)),
        (0xFBu8, Opcode::Unary(nop, 1)),
        (0xFCu8, Opcode::Unary(nop, 1)),
        (0xFDu8, Opcode::Unary(nop, 1)),
        (0xFEu8, Opcode::Unary(nop, 1)),
        (0xFFu8, Opcode::Unary(nop, 1)),
    ]);
}

#[cfg(test)]
mod tests {
    use crate::mmu::MMU;

    use super::*;

    #[test]
    fn test_nop() {
        let mut cpu = SM83::new();
        let mmu = MMU::new();
        let opcode = 0x00u8;
        let operation = SM83_OPERATIONS.get(&opcode).unwrap();

        match operation {
            Opcode::Unary(op, _) => op(&mut cpu, &mmu),
            _ => panic!("Expected unary operation"),
        }
    }
}
