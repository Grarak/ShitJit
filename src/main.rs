extern crate core;

mod jit;
mod parser;
pub(crate) mod tests;
mod utils;

use crate::parser::nro::Nro;
use crate::utils::memory::page_align;
use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: {} <path-to-nro>", args[0]);
        exit(1);
    }

    let nro = Nro::parse(&args[1]).unwrap();
    let memory = nro.build_memory();

    let text_segment = nro.get_segment(&nro.get_header().text_segment_header);
    let (_, text_content, _) = unsafe { text_segment.align_to::<u32>() };

    let mut jit = jit::context::Context::new(text_content.to_vec());
    jit.run();
}
