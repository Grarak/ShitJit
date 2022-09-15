use crate::jit::context::Context;
use crate::jit::emitter_branch::emit_beq;
use crate::jit::emitter_cmp::{emit_ccmn, emit_cmn, emit_cmp};
use bad64::Op;
use dynasmrt::x64::Assembler;

pub fn parse_inst(context: &mut Context, assembler: &mut Assembler, inst: &u32) -> bool {
    let inst_decoded = bad64::decode(*inst, 0).unwrap();
    println!("{}", inst_decoded);
    let operands = inst_decoded.operands();
    let parse = match inst_decoded.op() {
        Op::CMP => emit_cmp,
        Op::CMN => emit_cmn,
        Op::CCMN => emit_ccmn,
        Op::B_EQ => emit_beq,
        _ => panic!("Unknown op {}", inst_decoded),
    };
    parse(context, assembler, operands)
}
