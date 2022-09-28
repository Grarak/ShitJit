use crate::jit::assembler::instructions_assembler::{Inst, InstAssembler};
use crate::jit::assembler::registers_handler::RegistersHandler;
use crate::jit::context::Context;
use bad64::{Imm, Operand};
use iced_x86::Code;
use crate::jit::utils::map_x64_to_half_if;

pub fn emit_add(context: &mut Context, asm: &mut InstAssembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 3);
    true
}

pub fn emit_adr(context: &mut Context, asm: &mut InstAssembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);

    let reg = match &operands[0] {
        Operand::Reg { reg, .. } => reg,
        _ => panic!("Adr only supports reg as left operand"),
    };

    let relative_addr = match &operands[1] {
        Operand::Label(imm) => match imm {
            Imm::Unsigned(imm) => imm,
            _ => panic!("Only unsigned label supported"),
        },
        _ => panic!("Adr only supports label as right operand"),
    };

    let mut regs_handler = RegistersHandler::new();
    let pc_reg = map_x64_to_half_if(regs_handler.get_free().unwrap(), *reg);
    let addr_reg = regs_handler.get_free().unwrap();

    asm.emit_var_to_reg(context.registers.borrow_mut_pc(), pc_reg);
    asm.uw_add(Inst::with2(Code::Mov_r64_imm64, addr_reg, *relative_addr));
    asm.uw_add(Inst::with2(Code::Add_r64_rm64, pc_reg, addr_reg));
    asm.emit_set_var(pc_reg, context.registers.borrow_mut_reg(*reg));

    true
}

pub fn emit_sub(context: &mut Context, asm: &mut InstAssembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 3);
    true
}

pub fn emit_subs(context: &mut Context, asm: &mut InstAssembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 3);
    true
}
