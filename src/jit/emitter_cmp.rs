use bad64::{Condition, Imm, Operand};
use dynasmrt::{DynamicLabel, dynasm, DynasmApi, DynasmLabelApi};
use dynasmrt::x64::Assembler;
use crate::jit::context::Context;

pub fn emit_cmp(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    let left_op = &operands[0];
    let right_op = &operands[1];
    match left_op {
        Operand::Reg { reg, .. } => {
            match right_op {
                Operand::Imm64 { imm, .. } => match imm {
                    Imm::Unsigned(imm) => {
                        context.emit_get_reg(assembler, *reg);

                        dynasm!(assembler
                            ; .arch x64
                            ; sub rax, *imm as _
                        );

                        context.registers.get_nzcv().emit_update(assembler);
                    }
                    _ => panic!("Signed imm value not supported {}", imm)
                }
                _ => panic!("Unknown right cmp operand {}", right_op)
            }
        }
        _ => panic!("Unknown left cmp operand {}", left_op)
    }
    true
}

pub fn emit_cmn(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    let left_op = &operands[0];
    let right_op = &operands[1];
    match left_op {
        Operand::Reg { reg, .. } => {
            match right_op {
                Operand::Imm64 { imm, .. } => match imm {
                    Imm::Unsigned(imm) => {
                        context.emit_get_reg(assembler, *reg);

                        let negated_imm = -(*imm as i64);
                        dynasm!(assembler
                            ; .arch x64
                            ; sub rax, negated_imm as _
                        );

                        context.registers.get_nzcv().emit_update(assembler);
                    }
                    _ => panic!("Signed imm value not supported {}", imm)
                }
                _ => panic!("Unknown right cmp operand {}", right_op)
            }
        }
        _ => panic!("Unknown left cmp operand {}", left_op)
    }
    true
}

pub fn emit_ccmn(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 4);
    let left_op = match &operands[0] {
        Operand::Reg { reg, .. } => reg,
        _ => panic!("Left ccmn operand must be reg")
    };
    let right_op = &operands[1];
    let nzcv = match &operands[2] {
        Operand::Imm32 { imm, .. } => match imm {
            Imm::Unsigned(imm) => imm,
            _ => panic!("nzcv ccmn must be unsigned")
        },
        _ => panic!("nzcv ccmn must be immediate")
    };
    let cond = match &operands[3] {
        Operand::Cond(cond) => cond,
        _ => panic!("ccmn should hold a cond")
    };

    let left_reg_addr = context.registers.map_addr(*left_op);

    match right_op {
        Operand::Imm64 { imm, .. } => match imm {
            Imm::Unsigned(imm) => {
                match cond {
                    Condition::NE => {
                        context.registers.get_nzcv().emit_get_z(assembler);
                        let set_nzcv_label = assembler.new_dynamic_label();
                        let end_label = assembler.new_dynamic_label();

                        dynasm!(
                            assembler
                            ; .arch x64
                            ; cmp rax, 1
                            ; jne =>set_nzcv_label
                        );

                        emit_cmn(context, assembler, &[operands[0], operands[1]]);

                        dynasm!(
                            assembler
                            ; .arch x64
                            ; call =>end_label
                            ; =>set_nzcv_label
                            ; mov rax, *nzcv as _
                            ; sal rax, 28
                            ; =>end_label
                        );

                        context.registers.get_nzcv().emit_set(assembler);
                    }
                    _ => panic!("Unsupported condition {}", cond)
                }

                dynasm!(assembler
                    ; .arch x64
                );
            }
            _ => panic!("Signed ccmn imm not supported")
        },
        _ => panic!("Unknown right ccmn operand")
    }
    true
}
