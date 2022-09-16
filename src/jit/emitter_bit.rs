use crate::jit::context::Context;
use bad64::Operand;
use dynasmrt::x64::Assembler;

pub fn emit_and(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 3);
    true
}
