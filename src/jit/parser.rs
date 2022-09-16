use crate::jit::context::Context;
use crate::jit::emitter_arithmetic::{emit_add, emit_adr, emit_sub, emit_subs};
use crate::jit::emitter_bit::emit_and;
use crate::jit::emitter_branch::{emit_beq, emit_bne};
use crate::jit::emitter_cmp::{emit_ccmn, emit_cmn, emit_cmp};
use crate::jit::emitter_mem::{emit_ldp, emit_mov, emit_str};
use bad64::Op;
use dynasmrt::x64::Assembler;

pub fn parse_inst(context: &mut Context, assembler: &mut Assembler, inst: &u32) -> bool {
    let inst_decoded = bad64::decode(*inst, 0).unwrap();
    println!("{}", inst_decoded);
    let operands = inst_decoded.operands();
    let parse = match inst_decoded.op() {
        Op::ADD => emit_add,
        Op::ADR => emit_adr,
        Op::SUB => emit_sub,
        Op::SUBS => emit_subs,

        Op::AND => emit_and,

        Op::B_EQ => emit_beq,
        Op::B_NE => emit_bne,

        Op::CMP => emit_cmp,
        Op::CMN => emit_cmn,
        Op::CCMN => emit_ccmn,

        Op::LDP => emit_ldp,
        Op::MOV => emit_mov,
        Op::STR => emit_str,
        _ => panic!("Unknown op {}", inst_decoded),
    };
    parse(context, assembler, operands)
}
