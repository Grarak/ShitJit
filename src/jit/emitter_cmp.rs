use crate::jit::assembler::instructions_assembler::{Inst, InstAssembler};
use crate::jit::assembler::registers_handler::RegistersHandler;
use crate::jit::context::Context;
use bad64::{Condition, Imm, Operand, Reg};
use iced_x86::Code;

pub fn emit_cmp(context: &mut Context, asm: &mut InstAssembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    let left_op = &operands[0];
    let right_op = &operands[1];
    match left_op {
        Operand::Reg { reg, .. } => match right_op {
            Operand::Imm64 { imm, .. } => match imm {
                Imm::Unsigned(imm) => {
                    let mut regs_handler = RegistersHandler::new();
                    let dest_register = regs_handler.get_free().unwrap();
                    let value_reg = regs_handler.get_free().unwrap();

                    context.emit_get_reg(asm, *reg, dest_register);
                    asm.uw_add(Inst::with2(Code::Mov_r64_imm64, value_reg, *imm));
                    asm.uw_add(Inst::with2(
                        Code::Sub_r64_rm64,
                        dest_register,
                        value_reg,
                    ));
                    context.registers.nzcv.emit_update(asm);
                }
                _ => panic!("Signed imm value not supported {}", imm),
            },
            _ => panic!("Unknown right cmp operand {}", right_op),
        },
        _ => panic!("Unknown left cmp operand {}", left_op),
    }
    true
}

pub fn emit_cmn(context: &mut Context, asm: &mut InstAssembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    let left_op = &operands[0];
    let right_op = &operands[1];

    match left_op {
        Operand::Reg { reg, .. } => match right_op {
            Operand::Imm64 { imm, .. } => match imm {
                Imm::Unsigned(imm) => {
                    let negated_imm = -(*imm as i64);
                    let mut regs_handler = RegistersHandler::new();
                    let dest_reg = regs_handler.get_free().unwrap();
                    let value_reg = regs_handler.get_free().unwrap();

                    context.emit_get_reg(asm, *reg, dest_reg);
                    asm.uw_add(Inst::with2(
                        Code::Mov_r64_imm64,
                        value_reg,
                        negated_imm,
                    ));
                    asm.uw_add(Inst::with2(Code::Sub_r64_rm64, dest_reg, value_reg));
                    context.registers.nzcv.emit_update(asm);
                }
                _ => panic!("Signed imm value not supported {}", imm),
            },
            _ => panic!("Unknown right cmp operand {}", right_op),
        },
        _ => panic!("Unknown left cmp operand {}", left_op),
    }
    true
}

pub fn emit_ccmn(context: &mut Context, asm: &mut InstAssembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 4);
    let nzcv = match &operands[2] {
        Operand::Imm32 { imm, .. } => match imm {
            Imm::Unsigned(imm) => imm,
            _ => panic!("nzcv ccmn must be unsigned"),
        },
        _ => panic!("nzcv ccmn must be immediate"),
    };
    let cond = match &operands[3] {
        Operand::Cond(cond) => cond,
        _ => panic!("ccmn should hold a cond"),
    };

    let mut regs_handler = RegistersHandler::new();
    let nzcv_reg = regs_handler.get_free().unwrap();

    context.registers.nzcv.emit_get_z(asm, nzcv_reg);
    asm.uw_add(Inst::with2(Code::Cmp_rm64_imm8, nzcv_reg, 1));

    let compare_label = asm.create_label();
    let end_label = asm.create_label();

    match cond {
        Condition::NE => asm.add_branch(Code::Jne_rel32_64, &compare_label),
        _ => panic!("Unsupported condition {}", cond),
    }

    context.registers.nzcv.emit_set(asm, (*nzcv) << 28);
    asm.add_branch(Code::Jmp_rel32_64, &end_label);

    asm.add_with_label(Inst::with(Code::Nopd), &compare_label);
    emit_cmn(context, asm, &[operands[0], operands[1]]);

    asm.add_with_label(Inst::with(Code::Nopd), &end_label);
    true
}
