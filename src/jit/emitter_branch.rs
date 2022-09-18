use crate::jit::assembler::instructions_assembler::{Inst, InstAssembler};
use crate::jit::context::Context;
use bad64::{Imm, Operand};
use iced_x86::{Code, Register};
use crate::jit::assembler::registers_handler::RegistersHandler;
use crate::jit::utils;

fn get_label(operand: &Operand) -> u64 {
    *match operand {
        Operand::Label(imm) => match imm {
            Imm::Unsigned(imm) => imm,
            _ => panic!(""),
        },
        _ => panic!("Branch must have a label"),
    }
}

fn cmp_and_branch(context: &mut Context, asm: &mut InstAssembler, code_cond: Code, addr: u64) {
    let mut regs_handler = RegistersHandler::new();
    let z_reg = regs_handler.get_free().unwrap();

    let context_addr = context.get_addr();
    let call_label = asm.create_label();
    let end_label = asm.create_label();

    context.registers.nzcv.emit_get_z(asm, z_reg);
    asm.uw_add(Inst::with2(Code::Cmp_rm64_imm8, z_reg, 1));
    asm.add_branch(code_cond, &call_label);
    asm.add_branch(Code::Jmp_rel32_64, &end_label);

    asm.uw_add_with_label(Inst::with2(Code::Mov_r64_imm64, Register::RSI, addr), &call_label);
    asm.uw_add(Inst::with2(Code::Mov_r64_imm64, Register::RDI, context_addr));
    asm.uw_add(Inst::with1(Code::Push_r64, Register::RAX));
    asm.uw_add(Inst::with2(Code::Mov_r64_imm64, Register::RAX, utils::get_fn_addr!(Context::branch)));
    asm.uw_add(Inst::with1(Code::Call_rm64, Register::RAX));
    asm.uw_add(Inst::with1(Code::Pop_r64, Register::RAX));
    asm.add_with_label(Inst::with(Code::Nopd), &end_label);
}

pub fn emit_beq(
    context: &mut Context,
    asm: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 1);
    let addr = get_label(&operands[0]);
    cmp_and_branch(context, asm, Code::Je_rel32_64, addr);
    false
}

pub fn emit_bne(
    context: &mut Context,
    asm: &mut InstAssembler,
    operands: &[Operand],
) -> bool {
    assert_eq!(operands.len(), 1);
    let addr = get_label(&operands[0]);
    cmp_and_branch(context, asm, Code::Jne_rel32_64, addr);
    false
}
