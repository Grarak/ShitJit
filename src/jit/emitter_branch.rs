use crate::jit::context::{emit, Context};
use bad64::{Imm, Operand};
use dynasmrt::x64::Assembler;
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

pub fn emit_beq(context: &mut Context, assembler: &mut Assembler, operands: &[Operand]) -> bool {
    assert_eq!(operands.len(), 1);

    let addr = match &operands[0] {
        Operand::Label(imm) => match imm {
            Imm::Unsigned(imm) => imm,
            _ => panic!(""),
        },
        _ => panic!("Branch must have a label"),
    };

    emit::nzcv::get_z!(context, assembler, rax);

    let context_addr = context.get_addr();
    emit::asm!(assembler
        ; cmp rax, 1
        ; jne >end
        ; mov rsi, QWORD *addr as _
        ; mov rdi, QWORD context_addr as _
        ;; emit::call_external!(assembler, Context::branch)
        ; end:
    );
    false
}
