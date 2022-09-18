use crate::jit::assembler::{Error, Result};
use iced_x86::Register;
use std::collections::HashSet;

const REGS_64: [Register; 16] = [
    Register::RAX,
    Register::RBX,
    Register::RCX,
    Register::RDX,
    Register::RSI,
    Register::RDI,
    Register::RBP,
    Register::RSP,
    Register::R8,
    Register::R9,
    Register::R10,
    Register::R11,
    Register::R12,
    Register::R13,
    Register::R14,
    Register::R15,
];

const REGS_32: [Register; 16] = [
    Register::EAX,
    Register::EBX,
    Register::ECX,
    Register::EDX,
    Register::ESI,
    Register::EDI,
    Register::EBP,
    Register::ESP,
    Register::R8D,
    Register::R9D,
    Register::R10D,
    Register::R11D,
    Register::R12D,
    Register::R13D,
    Register::R14D,
    Register::R15D,
];

const REGS_16: [Register; 16] = [
    Register::AX,
    Register::BX,
    Register::CX,
    Register::DX,
    Register::SI,
    Register::DI,
    Register::BP,
    Register::SP,
    Register::R8W,
    Register::R9W,
    Register::R10W,
    Register::R11W,
    Register::R12W,
    Register::R13W,
    Register::R14W,
    Register::R15W,
];

const REGS_8: [Register; 16] = [
    Register::AL,
    Register::BL,
    Register::CL,
    Register::DL,
    Register::SIL,
    Register::DIL,
    Register::BPL,
    Register::SPL,
    Register::R8L,
    Register::R9L,
    Register::R10L,
    Register::R11L,
    Register::R12L,
    Register::R13L,
    Register::R14L,
    Register::R15L,
];

const REGS_8_H: [Register; 4] = [Register::AH, Register::BH, Register::CH, Register::DH];

const CALLER_SAVED_REGISTERS: [Register; 9] = [
    Register::RAX,
    Register::RCX,
    Register::RDX,
    Register::RSI,
    Register::RDI,
    Register::R8,
    Register::R9,
    Register::R10,
    Register::R11,
];

pub struct RegistersHandler {
    used_registers: HashSet<Register>,
}

impl RegistersHandler {
    pub fn new() -> Self {
        RegistersHandler {
            used_registers: HashSet::new(),
        }
    }

    pub fn reserve(&mut self, reg: Register) {
        self.used_registers.insert(reg.full_register());
    }

    pub fn get_free(&mut self) -> Result<Register> {
        for register in CALLER_SAVED_REGISTERS {
            if !self.used_registers.contains(&register) {
                self.used_registers.insert(register);
                return Ok(register);
            }
        }
        Err(Error::new("No free registers remaining".to_string()))
    }
}

fn map_reg(reg: &Register, array: &[Register]) -> Register {
    let full_reg = reg.full_register();
    let index = REGS_64.iter().position(|&r| r == full_reg).unwrap();
    array[index]
}

pub fn map_reg_32(reg: &Register) -> Register {
    map_reg(reg, &REGS_32)
}

pub fn map_reg_16(reg: &Register) -> Register {
    map_reg(reg, &REGS_16)
}

pub fn map_reg_8(reg: &Register) -> Register {
    map_reg(reg, &REGS_8)
}
