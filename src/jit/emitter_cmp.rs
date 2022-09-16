use crate::jit::context::{emit, Context};
use bad64::{Condition, Imm, Operand};
use dynasmrt::x64::Assembler;
use dynasmrt::{DynasmApi, DynasmLabelApi};

pub fn emit_cmp(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    let left_op = &operands[0];
    let right_op = &operands[1];
    match left_op {
        Operand::Reg { reg, .. } => match right_op {
            Operand::Imm64 { imm, .. } => match imm {
                Imm::Unsigned(imm) => {
                    emit::asm!(assembler
                        ;; emit::get_reg!(context, assembler, *reg, rax)
                        ; sub rax, *imm as _
                        ;; emit::nzcv::update!(context, assembler)
                    );
                }
                _ => panic!("Signed imm value not supported {}", imm),
            },
            _ => panic!("Unknown right cmp operand {}", right_op),
        },
        _ => panic!("Unknown left cmp operand {}", left_op),
    }
    true
}

pub fn emit_cmn(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 2);
    let left_op = &operands[0];
    let right_op = &operands[1];

    match left_op {
        Operand::Reg { reg, .. } => match right_op {
            Operand::Imm64 { imm, .. } => match imm {
                Imm::Unsigned(imm) => {
                    let negated_imm = -(*imm as i64);
                    emit::asm!(assembler
                        ;; emit::get_reg!(context, assembler, *reg, rax)
                        ; sub rax, negated_imm as _
                        ;; emit::nzcv::update!(context, assembler)
                    );
                }
                _ => panic!("Signed imm value not supported {}", imm),
            },
            _ => panic!("Unknown right cmp operand {}", right_op),
        },
        _ => panic!("Unknown left cmp operand {}", left_op),
    }
    true
}

pub fn emit_ccmn(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
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

    let set_nzcv_label = assembler.new_dynamic_label();
    emit::asm!(assembler
        ;; emit::nzcv::get_z!(context, assembler, rax)
        ; cmp rax, 1
    );

    match cond {
        Condition::NE => {
            emit::asm!(assembler
                ; je =>set_nzcv_label
            );
        }
        _ => panic!("Unsupported condition {}", cond),
    }

    emit_cmn(context, assembler, &[operands[0], operands[1]]);

    emit::asm!(assembler
        ; call >end
        ; =>set_nzcv_label
        ;; emit::nzcv::set!(context, assembler, ((*nzcv) << 28))
        ; end:
    );
    true
}
