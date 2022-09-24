use crate::jit::assembler::instructions_assembler::InstAssembler;
use crate::jit::context::Context;
use bad64::Operand;

pub fn emit_add(
    context: &mut Context,
    asm: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 3);
    true
}

pub fn emit_adr(
    context: &mut Context,
    asm: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 2);
    true
}

pub fn emit_sub(
    context: &mut Context,
    asm: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 3);
    true
}

pub fn emit_subs(
    context: &mut Context,
    asm: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 3);
    true
}
