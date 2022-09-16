use crate::jit::context::{emit, Context};
use bad64::Operand;
use dynasmrt::x64::Assembler;
use dynasmrt::DynasmApi;

pub fn emit_ldp(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 3);
    true
}

pub fn emit_mov(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    let left_op = match &operands[0] {
        Operand::Reg { reg, .. } => reg,
        _ => panic!("Left operand must be register"),
    };
    let right_op = match &operands[1] {
        Operand::Reg { reg, .. } => reg,
        _ => panic!("Right operand must be register"),
    };
    let right_reg_addr = context.registers.map_addr(*right_op);
    emit::asm!(assembler
        ;; emit::get_reg!(context, assembler, *left_op, rax)
        ; mov rcx, QWORD right_reg_addr as _
        ; mov [rcx], rax
    );
    true
}

pub fn emit_str(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    true
}
