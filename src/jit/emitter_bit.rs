use crate::jit::assembler::instructions_assembler::InstAssembler;
use crate::jit::context::Context;
use bad64::Operand;

pub fn emit_and(
    context: &mut Context,
    assembler: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 3);
    true
}
