use crate::jit::assembler::instructions_assembler::InstAssembler;
use crate::jit::assembler::registers_handler::RegistersHandler;
use iced_x86::{Code, Instruction, MemoryOperand, Register};
use std::ptr::NonNull;

macro_rules! get_fn_addr {
    ($fun:expr) => {
        ($fun as *const ()) as u64
    };
}
use crate::jit::utils::private::EmitSetVar;
pub(crate) use get_fn_addr;

pub fn get_var_addr<T>(var: &T) -> u64 {
    (var as *const T) as u64
}

mod private {
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
