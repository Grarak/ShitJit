use crate::jit::assembler::instructions_assembler::Inst;
use iced_x86::{Code, Instruction, Register};

pub struct ShortInst;

trait Mov<D, S> {
    fn mov(dest: D, src: S) -> Inst;
}

trait VarToReg<D, S> {
    fn var_to_reg(dest: D, src: S) -> Inst;
}

impl ShortInst {
    fn mov<D, S>(dest: D, src: S) -> Inst
        where
            Self: Mov<D, S>,
    {
        <Self as Mov<D, S>>::mov(dest, src)
    }
}

impl Mov<Register, Register> for ShortInst {
    fn mov(dest: Register, src: Register) -> Inst {
        assert_eq!(dest.size(), src.size());
        let code = if dest.is_gpr64() {
            Code::Mov_r64_rm64
        } else if dest.is_gpr32() {
            Code::Mov_r32_rm32
        } else if dest.is_gpr16() {
            Code::Mov_r16_rm16
        } else if dest.is_gpr8() {
            Code::Mov_r8_rm8
        } else {
            panic!("Unsupported register size")
        };

        Inst::with2(code, dest, src).unwrap()
    }
}

impl Mov<Register, u64> for ShortInst {
    fn mov(dest: Register, src: u64) -> Inst {
        assert!(dest.is_gpr64());
        Inst::with2(Code::Mov_r64_imm64, dest, src).unwrap()
    }
}

impl VarToReg<Register, &u64> for ShortInst {
    fn var_to_reg(dest: Register, src: &u64) -> [Inst; 2] {

    }
}
