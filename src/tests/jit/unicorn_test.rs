use crate::{page_align, Nro};
use std::ptr::NonNull;

#[test]
fn check_instructions() {
    use unicorn_engine::unicorn_const::{Arch, Mode, Permission, SECOND_SCALE};
    use unicorn_engine::{RegisterARM, Unicorn};

    let nro = Nro::parse(&"hello.nro".to_string()).unwrap();
    nro.get_segment(&nro.get_header().text_segment_header);
    let mem = nro.build_memory();

    let mem_offset = &mem[0x4000..];
    let (_, text_content, _) = unsafe { mem_offset.align_to::<u32>() };

    for i in 0..20 {
        let inst_decoded = bad64::decode(text_content[i], 0).unwrap();
        println!("{}", inst_decoded);
    }

    // let mut unicorn =
    //     Unicorn::new(Arch::ARM64, Mode::ARM).expect("failed to initialize Unicorn instance");
    // let emu = &mut unicorn;
    //
    // println!("{}", text_segment.len());
    // emu.mem_map(BASE, (BASE + 0x30000) as usize, Permission::ALL)
    //     .expect("failed to map code page");
    // emu.mem_write(BASE, &text_segment)
    //     .expect("failed to write instructions");
    //
    // emu.add_code_hook(BASE, BASE + 0x30000, |unicorn, address, size| {
    //     println!("{} {:x} {:x}", (address - BASE) / 4, address, size);
    // })
    // .unwrap();
    //
    // emu.add_intr_hook(|unicorn, address| {
    //     println!("{} {:x}", (address as u64 - BASE) / 4, address);
    // })
    // .unwrap();
    //
    // emu.add_insn_invalid_hook(|unicorn| {
    //     println!("insn invalid");
    //     true
    // })
    // .unwrap();
    //
    // let _ = emu.emu_start(
    //     BASE,
    //     (BASE + text_segment.len() as u64),
    //     1000 * SECOND_SCALE,
    //     1000,
    // );
}
