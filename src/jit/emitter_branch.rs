use crate::jit::context::{emit, Context};
use bad64::{Imm, Operand};
use dynasmrt::x64::Assembler;
use dynasmrt::{DynasmApi, DynasmLabelApi};

fn get_label(operand: &Operand) -> u64 {
    *match operand {
        Operand::Label(imm) => match imm {
            Imm::Unsigned(imm) => imm,
            _ => panic!(""),
        },
        _ => panic!("Branch must have a label"),
    }
}

macro_rules! cmp_and_branch {
    ($context:ident, $assembler:ident, $addr:expr, $cond:tt) => {
        let context_addr = $context.get_addr();
        emit::asm!($assembler
            ;; emit::nzcv::get_z!($context, $assembler, rax)
            ; cmp rax, 1
            ; $cond >branch
            ; call >end
            ; branch:
            ; mov rsi, QWORD $addr as _
            ; mov rdi, QWORD context_addr as _
            ;; emit::call_external!($assembler, Context::branch)
            ; end:
        );
    };
}

pub fn emit_beq(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 1);
    let addr = get_label(&operands[0]);
    cmp_and_branch!(context, assembler, addr, je);
    false
}

pub fn emit_bne(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 1);
    let addr = get_label(&operands[0]);
    cmp_and_branch!(context, assembler, addr, jne);
    false
}
