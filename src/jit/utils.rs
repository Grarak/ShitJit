use crate::jit::assembler::instructions_assembler::InstAssembler;
use crate::jit::assembler::registers_handler::{map_reg_32, RegistersHandler};
use bad64::Reg;
use iced_x86::{Code, Instruction, MemoryOperand, Register};
use std::ptr::NonNull;

macro_rules! get_fn_addr {
    ($fun:expr) => {
        ($fun as *const ()) as u64
    };
}
use crate::jit::utils::private::{EmitSetVar, EmitVarToReg};
pub(crate) use get_fn_addr;

pub fn get_var_addr<T>(var: &T) -> u64 {
    (var as *const T) as u64
}

mod private {
    pub trait EmitVarToReg<S, D> {
        fn emit_var_to_reg(&mut self, src: S, dest: D);
    }

    pub trait EmitSetVar<T> {
        fn emit_set_var(&mut self, src: T, dest: &mut u64);
    }
}

impl InstAssembler {
    pub fn emit_var_to_reg(&mut self, var: &u64, reg: Register) {
        self.uw_add(Instruction::with2(
            Code::Mov_r64_imm64,
            reg,
            get_var_addr(var),
        ));
        self.uw_add(Instruction::with2(
            Code::Mov_r64_rm64,
            reg,
            MemoryOperand::with_base(reg),
        ));
    }

    #[inline]
    pub fn emit_set_var<T>(&mut self, src: T, dest: &mut u64)
    where
        Self: EmitSetVar<T>,
    {
        (self as &mut dyn EmitSetVar<T>).emit_set_var(src, dest);
    }
}

impl EmitVarToReg<Register, u64> for InstAssembler {
    fn emit_var_to_reg(&mut self, src: Register, dest: u64) {

    }
}

impl EmitSetVar<u64> for InstAssembler {
    fn emit_set_var(&mut self, src: u64, dest: &mut u64) {
        let mut regs_handler = RegistersHandler::new();
        let addr_reg = regs_handler.get_free().unwrap();
        let value_reg = regs_handler.get_free().unwrap();

        self.uw_add(Instruction::with2(
            Code::Mov_r64_imm64,
            addr_reg,
            get_var_addr(dest),
        ));
        self.uw_add(Instruction::with2(Code::Mov_r64_imm64, value_reg, src));
        self.uw_add(Instruction::with2(
            Code::Mov_rm64_r64,
            MemoryOperand::with_base(addr_reg),
            value_reg,
        ));
    }
}

impl EmitSetVar<Register> for InstAssembler {
    fn emit_set_var(&mut self, src: Register, dest: &mut u64) {
        let mut regs_handler = RegistersHandler::new();
        regs_handler.reserve(src);

        let addr_reg = regs_handler.get_free().unwrap();

        self.uw_add(Instruction::with2(
            Code::Mov_r64_imm64,
            addr_reg,
            get_var_addr(dest),
        ));
        self.uw_add(Instruction::with2(
            Code::Mov_rm64_r64,
            MemoryOperand::with_base(addr_reg),
            src,
        ));
    }
}

fn is_aarch64_half_reg(reg: Reg) -> bool {
    match reg {
        Reg::W0
        | Reg::W1
        | Reg::W2
        | Reg::W3
        | Reg::W4
        | Reg::W5
        | Reg::W6
        | Reg::W7
        | Reg::W8
        | Reg::W9
        | Reg::W10
        | Reg::W11
        | Reg::W12
        | Reg::W13
        | Reg::W14
        | Reg::W15
        | Reg::W16
        | Reg::W17
        | Reg::W18
        | Reg::W19
        | Reg::W20
        | Reg::W21
        | Reg::W22
        | Reg::W23
        | Reg::W24
        | Reg::W25
        | Reg::W26
        | Reg::W27
        | Reg::W28
        | Reg::W29
        | Reg::W30
        | Reg::WZR
        | Reg::WSP => true,
        _ => false,
    }
}

pub fn map_x64_to_half_if(x64_reg: Register, aarch64_reg: Reg) -> Register {
    if is_aarch64_half_reg(aarch64_reg) {
        map_reg_32(&x64_reg)
    } else {
        x64_reg
    }
}
