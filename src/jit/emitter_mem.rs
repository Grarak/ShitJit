use crate::jit::assembler::instructions_assembler::InstAssembler;
use crate::jit::context::Context;
use bad64::Operand;

pub fn emit_ldp(
    context: &mut Context,
    assembler: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 3);
    true
}

pub fn emit_mov(
    context: &mut Context,
    assembler: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 2);
    let left_op = match &operands[0] {
        Operand::Reg { reg, .. } => reg,
        _ => panic!("Left operand must be register"),
    };
    let right_op = match &operands[1] {
        Operand::Reg { reg, .. } => reg,
        _ => panic!("Right operand must be register"),
    };
    //let right_reg_addr = context.registers.map_addr(*right_op);
    /*emit::asm!(assembler
        ;; emit::get_reg!(context, assembler, *left_op, rax)
        ; mov rcx, QWORD right_reg_addr as _
        ; mov [rcx], rax
    );*/
    true
}

pub fn emit_str(
    context: &mut Context,
    assembler: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 2);
    true
}
