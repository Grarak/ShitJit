use crate::jit::context::Context;
use bad64::Operand;
use dynasmrt::x64::Assembler;

pub fn emit_add(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 3);
    true
}

pub fn emit_adr(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    true
}

pub fn emit_sub(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 3);
    true
}

pub fn emit_subs(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 3);
    true
}
