use crate::jit::assembler::instructions_assembler::{Inst, InstAssembler};
use crate::jit::assembler::registers_handler::{map_reg_16, RegistersHandler};
use crate::jit::parser::parse_inst;
use crate::jit::utils;
use bad64::Reg;
use iced_x86::{Code, Decoder, DecoderOptions, Register};
use memmap::Mmap;
use std::collections::HashMap;
use std::mem;

const TEXT_OFFSET: u64 = 0x10000;

pub struct NZCV {
    value: u64,
}

impl Default for NZCV {
    fn default() -> Self {
        NZCV { value: 0x4 << 28 }
    }
}

impl NZCV {
    pub fn borrow_mut_value(&mut self) -> &mut u64 {
        &mut self.value
    }

    pub fn emit_update(&mut self, asm: &mut InstAssembler) {
        asm.add(Inst::with(Code::Lahf));
        asm.uw_add(Inst::with1(Code::Seto_rm8, Register::AL));

        let mut regs_handler = RegistersHandler::new();
        regs_handler.reserve(Register::RAX);
        let nzcv_reg = regs_handler.get_free().unwrap();
        let process_reg = regs_handler.get_free().unwrap().full_register();
        let process_reg_16 = map_reg_16(&process_reg);

        asm.uw_add(Inst::with2(Code::Xor_r64_rm64, nzcv_reg, nzcv_reg));

        // NZ
        asm.uw_add(Inst::with2(Code::Xor_r64_rm64, process_reg, process_reg));
        asm.uw_add(Inst::with2(
            Code::Mov_r16_rm16,
            process_reg_16,
            Register::AX,
        ));
        asm.uw_add(Inst::with2(Code::And_rm64_imm32, process_reg, 0xC000u32));
        asm.uw_add(Inst::with2(Code::Sal_rm64_imm8, process_reg, 16));
        asm.uw_add(Inst::with2(Code::Or_r64_rm64, nzcv_reg, process_reg));

        // C
        asm.uw_add(Inst::with2(Code::Xor_r64_rm64, process_reg, process_reg));
        asm.uw_add(Inst::with2(
            Code::Mov_r16_rm16,
            process_reg_16,
            Register::AX,
        ));
        // CF is inverted in ARM
        asm.uw_add(Inst::with2(Code::Xor_rm16_imm8, process_reg_16, -1));
        asm.uw_add(Inst::with2(Code::And_rm64_imm32, process_reg, 0x100));
        asm.uw_add(Inst::with2(Code::Sal_rm64_imm8, process_reg, 21));
        asm.uw_add(Inst::with2(Code::Or_r64_rm64, nzcv_reg, process_reg));

        // V
        asm.uw_add(Inst::with2(Code::Xor_r64_rm64, process_reg, process_reg));
        asm.uw_add(Inst::with2(
            Code::Mov_r16_rm16,
            process_reg_16,
            Register::AX,
        ));
        asm.uw_add(Inst::with2(Code::And_rm64_imm32, process_reg, 1));
        asm.uw_add(Inst::with2(Code::Sal_rm64_imm8, process_reg, 28));
        asm.uw_add(Inst::with2(Code::Or_r64_rm64, nzcv_reg, process_reg));

        asm.emit_set_var(nzcv_reg, self.borrow_mut_value());
    }

    pub fn emit_get_z(&self, asm: &mut InstAssembler, dest: Register) {
        asm.emit_var_to_reg(&self.value, dest);
        asm.uw_add(Inst::with2(Code::Sar_rm64_imm8, dest, 30));
        asm.uw_add(Inst::with2(Code::And_rm64_imm8, dest, 1));
    }

    pub fn emit_set(&mut self, asm: &mut InstAssembler, value: u64) {
        asm.emit_set_var(value, self.borrow_mut_value());
    }
}

#[derive(Default)]
pub struct Registers {
    x0: u64,
    x1: u64,
    pc: u64,
    pub nzcv: NZCV,
}

impl Registers {
    pub fn borrow_mut_reg(&mut self, reg: Reg) -> &mut u64 {
        match reg {
            Reg::X0 => &mut self.x0,
            Reg::X1 => &mut self.x1,
            _ => panic!("Unmapped register {}", reg),
        }
    }

    pub fn borrow_mut_pc(&mut self) -> &mut u64 {
        &mut self.pc
    }
}

pub struct Context {
    text: Vec<u32>,
    cached_functions: HashMap<usize, Mmap>,
    pub registers: Registers,
}

impl Context {
    pub fn new(text: Vec<u32>) -> Self {
        Context {
            text,
            cached_functions: HashMap::new(),
            registers: Registers::default(),
        }
    }

    pub fn run(&mut self) {
        self.execute_fn(0);
    }

    pub fn get_addr(&self) -> u64 {
        utils::get_var_addr(self)
    }

    pub extern "C" fn branch(&mut self, relative_addr: i64) {
        let addr = (self.registers.pc - TEXT_OFFSET) as i64 + relative_addr;
        self.execute_fn(addr as usize);
    }

    fn execute_fn(&mut self, addr: usize) {
        println!("Executing 0x{:x}", addr);

        if self.cached_functions.contains_key(&addr) {
            println!("0x{:x} is cached", addr);
        } else {
            let mut asm = InstAssembler::new();

            for inst_index in (addr / 4)..self.text.len() {
                let pc = TEXT_OFFSET + inst_index as u64 * 4;
                asm.emit_set_var(pc, self.registers.borrow_mut_pc());

                let inst = self.text[inst_index];
                let should_continue = parse_inst(self, &mut asm, &inst);
                if !should_continue {
                    break;
                }
            }

            asm.add(Inst::with(Code::Retnq));

            println!();

            let mem = asm.finalize().unwrap();

            let mut decoder = Decoder::new(64, &mem, DecoderOptions::NONE);
            for inst in &mut decoder {
                println!("{:016X} {}", inst.ip(), inst);
            }

            let fun: extern "C" fn() = unsafe { mem::transmute(mem.as_ptr()) };
            self.cached_functions.insert(addr, mem);
            fun();
        }

        self.print_regs();
    }

    fn print_regs(&self) {
        println!();
        println!("x0: {:#016x}", self.registers.x0);
        println!("x1: {:#016x}", self.registers.x1);
        println!("pc: {:#016x}", self.registers.pc);
        println!("nzcv: {:#016x}", self.registers.nzcv.value);
        println!("n: {}", (self.registers.nzcv.value >> 31) & 1);
        println!("z: {}", (self.registers.nzcv.value >> 30) & 1);
        println!("c: {}", (self.registers.nzcv.value >> 29) & 1);
        println!("v: {}", (self.registers.nzcv.value >> 28) & 1);
        println!();
    }

    pub fn emit_get_reg(&mut self, assembler: &mut InstAssembler, src: Reg, dest: Register) {
        assembler.emit_var_to_reg(self.registers.borrow_mut_reg(src), dest);
    }
}
