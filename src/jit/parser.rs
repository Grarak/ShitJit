use bad64::{Condition, Imm, Op, Operand, Reg};
use dynasmrt::x64::Assembler;
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use crate::jit::context::Context;

pub fn parse_inst(context: &mut Context, assembler: &mut Assembler, inst: &u32) {
    let inst_decoded = bad64::decode(*inst, 0).unwrap();
    println!("{}", inst_decoded);
    let operands = inst_decoded.operands();
    let parse = match inst_decoded.op() {
        Op::CMP => parse_cmp,
        Op::CCMN => parse_ccmn,
        _ => panic!("Unknown op {}", inst_decoded)
    };
    parse(context, assembler, operands);
}

fn parse_cmp(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) {
    assert_eq!(operands.len(), 2);
    let left_op = &operands[0];
    let right_op = &operands[1];
    match left_op {
        Operand::Reg { reg, .. } => {
            let reg_addr = context.registers.map_addr(reg);
            match right_op {
                Operand::Imm64 { imm, .. } => match imm {
                    Imm::Unsigned(imm) => {
                        dynasm!(assembler
                            ; .arch x64
                            ; mov r12, QWORD *imm as _
                            ; mov r13, QWORD reg_addr as _
                            ; mov r13, QWORD [r13]
                            ; cmp r13, r12
                        );
                    }
                    _ => panic!("Signed imm value not supported {}", imm)
                }
                _ => panic!("Unknown right cmp operand {}", right_op)
            }
        }
        _ => panic!("Unknown left cmp operand {}", left_op)
    }
}

fn parse_ccmn(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) {
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

    let left_reg_addr = context.registers.map_addr(left_op);

    match right_op {
        Operand::Imm64 { imm, .. } => match imm {
            Imm::Unsigned(imm) => {
                dynasm!(assembler
                    ; .arch x64
                    ; jne 0x24 // skip everything
                    ; mov r12, QWORD left_reg_addr as _
                    ; cmp QWORD [r12], *imm as _
                    ; jne 0x8 // set equal to zero
                );

                
            }
            _ => panic!("Signed ccmn imm not supported")
        },
        _ => panic!("Unknown right ccmn operand")
    }
}
