use std::iter::Map;
use memmap::{MmapMut, MmapOptions};
use std::collections::HashMap;
use std::slice::from_raw_parts;
use dynasmrt::{AssemblyOffset, dynasm, DynasmApi, DynasmLabelApi};
use iced_x86::{Decoder, DecoderOptions, Instruction};
use crate::jit::parser::parse_inst;
use std::{io, slice, mem};
use std::borrow::{Borrow, BorrowMut};
use std::io::{Write};
use bad64::Reg;

const JIT_CACHE_SIZE: usize = 1 * 1024 * 1024;

#[derive(Default)]
pub struct Registers {
    x0: u64,
    x1: u64,
}

impl Registers {
    pub fn map(&self, reg: &Reg) -> &u64 {
        match reg {
            Reg::X0 => &self.x0,
            Reg::X1 => &self.x1,
            _ => panic!("Unmapped register {}", reg)
        }
    }

    pub fn map_addr(&self, reg: &Reg) -> usize {
        (self.map(reg) as *const u64) as usize
    }
}

pub struct Context {
    text: Vec<u32>,
    mem: MmapMut,
    cached_functions: HashMap<usize, usize>,
    jit_pc: usize,
    pub registers: Registers,
}

impl Context {
    pub fn new(text: Vec<u32>) -> Self {
        let mem = MmapMut::map_anon(JIT_CACHE_SIZE).unwrap();
        Context { text, mem, cached_functions: HashMap::<usize, usize>::new(), jit_pc: 0, registers: Registers::default() }
    }

    pub fn run(&mut self) {
        self.execute_fn(0);
    }

    fn execute_fn(&mut self, addr: usize) {
        if self.cached_functions.contains_key(&addr) {
            // TODO call function at value
        } else {
            let mut ops = dynasmrt::x64::Assembler::new().unwrap();
            let fun_offset = ops.offset();

            for inst_index in addr..self.text.len() {
                let inst = self.text[inst_index];
                parse_inst(self, &mut ops, &inst);
            }

            dynasm!(ops
                ; .arch x64
                ; ret
            );

            let buf = ops.finalize().unwrap();

            let mut decoder = Decoder::with_ip(64, &buf, 0, DecoderOptions::NONE);
            let mut instruction = Instruction::default();
            while decoder.can_decode() {
                decoder.decode_out(&mut instruction);
                println!("{}", instruction);
            }

            let fun: extern "C" fn() = unsafe { mem::transmute(buf.ptr(fun_offset)) };
            fun();
        }
    }
}
