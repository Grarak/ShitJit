extern crate core;
extern crate core;

mod jit;
mod parser;
pub(crate) mod tests;
mod utils;

use crate::utils::memory::page_align;
use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: {} <path-to-nro>", args[0]);
        exit(1);
    }

    let nro = parser::nro::parse(&args[1]).unwrap();

    let text_segment = nro.get_segment(&nro.header.text_segment_header).unwrap();
    let (_, text_content, _) = unsafe { text_segment.align_to::<u32>() };

    let mut jit = jit::context::Context::new(text_content.to_vec());
    jit.run();
}
