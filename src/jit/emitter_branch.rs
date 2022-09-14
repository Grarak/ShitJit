use bad64::Operand;
use dynasmrt::x64::Assembler;
use crate::jit::context::Context;

pub fn emit_beq(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    false
}
