use crate::jit::assembler::{Error, Result};
use iced_x86::{BlockEncoder, BlockEncoderOptions, Code, IcedError, Instruction, InstructionBlock};
use memmap::{Mmap, MmapMut};
use std::result;

pub type Inst = Instruction;

pub struct Label {
    id: u64,
}

impl Label {
    fn new(id: u64) -> Self {
        Label { id }
    }
}

pub struct InstAssembler {
    insts: Vec<Inst>,
    label_counter: u64,
}

impl InstAssembler {
    pub fn new() -> Self {
        InstAssembler {
            insts: Vec::new(),
            label_counter: 0,
        }
    }

    #[inline]
    pub fn add(&mut self, inst: Inst) {
        self.insts.push(inst);
    }

    pub fn add_with_label(&mut self, mut inst: Inst, label: &Label) {
        inst.set_ip(label.id);
        self.add(inst);
    }

    #[inline]
    pub fn uw_add(&mut self, inst: result::Result<Inst, IcedError>) {
        self.add(inst.unwrap());
    }

    pub fn uw_add_with_label(&mut self, inst: result::Result<Inst, IcedError>, label: &Label) {
        let mut inst = inst.unwrap();
        inst.set_ip(label.id);
        self.add(inst);
    }

    #[inline]
    pub fn add_branch(&mut self, code: Code, label: &Label) {
        self.add(Inst::with_branch(code, label.id).unwrap());
    }

    pub fn create_label(&mut self) -> Label {
        self.label_counter += 1;
        Label::new(self.label_counter)
    }

    pub fn finalize(self) -> Result<Mmap> {
        let inst_block = InstructionBlock::new(&self.insts, 0);
        let block_encoder = match BlockEncoder::encode(64, inst_block, BlockEncoderOptions::NONE) {
            Ok(encoder) => Ok(encoder),
            Err(err) => Err(Error::new(err.to_string())),
        }?;
        let buf = block_encoder.code_buffer;

        let mut mem = match MmapMut::map_anon(buf.len()) {
            Ok(map) => Ok(map),
            Err(err) => Err(Error::new(err.to_string())),
        }?;
        mem.copy_from_slice(&buf);
        match mem.make_exec() {
            Ok(map) => Ok(map),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}
