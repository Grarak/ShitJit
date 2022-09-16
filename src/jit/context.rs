use crate::jit::parser::parse_inst;
use bad64::Reg;
use dynasmrt::x64::Assembler;
use dynasmrt::{DynasmApi, ExecutableBuffer};
use iced_x86::{Decoder, DecoderOptions, Instruction};
use std::collections::HashMap;
use std::mem;

pub mod emit {
    macro_rules! asm {
        ($assembler:ident $($t:tt)*) => {
            dynasmrt::dynasm!($assembler
                $($t)*
            )
        }
    }
    pub(crate) use asm;

    macro_rules! get_var {
        ($assembler:ident, $addr:expr, $retreg:tt) => {
            emit::asm!($assembler
                ; mov $retreg, QWORD $addr as _
                ; mov $retreg, QWORD [$retreg]
            );
        };
    }
    pub(crate) use get_var;

    macro_rules! set_var {
        ($assembler:ident, $addr:expr, $value:expr) => {
            emit::asm!($assembler
                ; mov rax, QWORD $addr as _
                ; mov QWORD [rax], $value as _
            );
        };
    }
    pub(crate) use set_var;

    macro_rules! get_reg {
        ($context:ident, $assembler:ident, $reg:expr, $retreg:tt) => {
            let reg_addr = $context.registers.map_addr($reg);
            emit::get_var!($assembler, reg_addr, $retreg);
        };
    }
    pub(crate) use get_reg;

    macro_rules! call_external {
        ($assembler:ident, $fun:expr) => {
            emit::asm!($assembler
                ; mov rax, QWORD $fun as _
                ; push rax
                ; call rax
                ; pop rax
            );
        };
    }
    pub(crate) use call_external;

    pub mod nzcv {
        macro_rules! get_z {
            ($context:ident, $assembler:ident, $retreg:tt) => {
                let addr = $context.registers.nzcv.get_addr();
                emit::asm!($assembler
                    ;; emit::get_var!($assembler, addr, $retreg)
                    ; sar $retreg, 30
                    ; and $retreg, 1
                )
            };
        }
        pub(crate) use get_z;

        macro_rules! update {
            ($context:ident, $assembler:ident) => {
                let addr = $context.registers.nzcv.get_addr();
                emit::asm!($assembler
                    ; lahf
                    ; seto al

                    ; xor rcx, rcx
                    // nz
                    ; xor rsi, rsi
                    ; mov si, ax
                    ; and rsi, 0xC000
                    ; sal rsi, 16
                    ; or rcx, rsi
                    // c
                    ; xor rsi, rsi
                    ; mov si, ax
                    ; xor si, -1 // CF is inverted in ARM
                    ; and rsi, 0x100
                    ; sal rsi, 21
                    ; or rcx, rsi
                    // v
                    ; xor rsi, rsi
                    ; mov si, ax
                    ; and rsi, 1
                    ; sal rsi, 28
                    ; or rcx, rsi

                    ; mov rsi, QWORD addr as _
                    ; mov QWORD [rsi], rcx
                )
            };
        }
        pub(crate) use update;

        macro_rules! set {
            ($context:ident, $assembler:ident, $value:expr) => {
                let addr = $context.registers.nzcv.get_addr();
                emit::set_var!($assembler, addr, $value)
            };
        }
        pub(crate) use set;
    }
}

const TEXT_OFFSET: u64 = 0x10000;

fn get_addr<T>(var: &T) -> usize {
    (var as *const T) as usize
}

pub struct NZCV {
    value: u64,
}

impl Default for NZCV {
    fn default() -> Self {
        NZCV { value: 0x4 << 28 }
    }
}

impl NZCV {
    pub fn get_addr(&self) -> usize {
        get_addr(&self.value)
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
    pub fn map(&self, reg: Reg) -> &u64 {
        match reg {
            Reg::X0 => &self.x0,
            Reg::X1 => &self.x1,
            _ => panic!("Unmapped register {}", reg),
        }
    }

    pub fn map_addr(&self, reg: Reg) -> usize {
        get_addr(self.map(reg))
    }

    pub fn get_pc_addr(&self) -> usize {
        get_addr(&self.pc)
    }
}

pub struct Context {
    text: Vec<u32>,
    cached_functions: HashMap<usize, ExecutableBuffer>,
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

    pub fn get_addr(&self) -> usize {
        get_addr(self)
    }

    pub extern "C" fn branch(&mut self, relative_addr: i64) {
        let addr = (self.registers.pc - TEXT_OFFSET) as i64 + relative_addr;
        self.execute_fn(addr as usize);
    }

    fn execute_fn(&mut self, addr: usize) {
        println!();
        println!("Executing 0x{:x}", addr);
        self.print_regs();

        if self.cached_functions.contains_key(&addr) {
            println!("0x{:x} is cached", addr);
        } else {
            let mut ops = Assembler::new().unwrap();
            let fun_offset = ops.offset();

            for inst_index in (addr / 4)..self.text.len() {
                let pc_addr = self.registers.get_pc_addr();
                let pc = (TEXT_OFFSET as usize) + inst_index * 4;
                emit::set_var!(ops, pc_addr, pc);

                let inst = self.text[inst_index];
                let should_continue = parse_inst(self, &mut ops, &inst);
                if !should_continue {
                    break;
                }
            }

            emit::asm!(ops
                ; ret
            );

            println!();

            let buf = ops.finalize().unwrap();

            let mut decoder = Decoder::with_ip(64, &buf, 0, DecoderOptions::NONE);
            let mut instruction = Instruction::default();
            while decoder.can_decode() {
                decoder.decode_out(&mut instruction);
                println!("{}", instruction);
            }

            let fun: extern "C" fn() = unsafe { mem::transmute(buf.ptr(fun_offset)) };
            self.cached_functions.insert(addr, buf);
            fun();
        }
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
}
