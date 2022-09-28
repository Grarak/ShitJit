use crate::page_align;
use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::mem::size_of;
use std::os::unix::fs::FileExt;
use std::{io, mem};

const NRO_MAGIC: &[u8; 4] = b"NRO0";
const MOD_MAGIC: &[u8; 4] = b"MOD0";

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ModHeader {
    reserved: u32,
    offset: u32,
    magic: [u8; 0x4],
    dynamic_offset: i32,
    bss_start_offset: i32,
    bss_end_offset: i32,
    eh_frame_hdr_start_offset: i32,
    eh_frame_hdr_end_offset: i32,
    bss_base_offset: i32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct NroSegmentHeader {
    pub memory_offset: u32,
    pub size: u32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct NroHeader {
    unused: u32,
    pub mod0_offset: u32,
    padding: u64,
    pub magic: [u8; 4],
    version: u32,
    pub size: u32,
    flags: u32,
    pub text_segment_header: NroSegmentHeader,
    pub ro_segment_header: NroSegmentHeader,
    pub data_segment_header: NroSegmentHeader,
    pub bss_size: u32,
    reserved: u32,
    pub module_id: [u8; 0x20],
    dso_handle_offset: u32,
    reserved2: u32,
    pub api_info_segment_header: NroSegmentHeader,
    pub dynstr_segment_header: NroSegmentHeader,
    pub dynsym_segment_header: NroSegmentHeader,
}

pub struct Nro {
    file_content: Vec<u8>,
}

impl Nro {
    fn new(mut file: File) -> io::Result<Self> {
        let mut file_content = Vec::<u8>::new();
        let len_read = file.read_to_end(&mut file_content)?;
        let nro = Nro { file_content };
        if len_read >= size_of::<NroHeader>() && &nro.get_header().magic == NRO_MAGIC {
            Ok(nro)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid nro file"))
        }
    }

    pub fn get_header(&self) -> &NroHeader {
        unsafe { mem::transmute(self.file_content.as_ptr()) }
    }

    fn get_mod_header(&self) -> Option<&ModHeader> {
        let header = self.get_header();
        let buf = &self.file_content;
        let header: &ModHeader =
            unsafe { mem::transmute(&buf[header.mod0_offset as usize..].as_ptr()) };
        if &header.magic == MOD_MAGIC {
            Some(header)
        } else {
            None
        }
    }

    pub fn get_segment(&self, segment: &NroSegmentHeader) -> &[u8] {
        let size = segment.size as usize;
        let offset = size_of::<NroHeader>() + segment.memory_offset as usize;
        &self.file_content[offset..offset + size]
    }

    pub fn build_memory(&self) -> Vec<u8> {
        let header = self.get_header();
        let mut memory_size = page_align(header.size);
        let bss_size = page_align(match self.get_mod_header() {
            None => header.bss_size,
            Some(mod_header) => (mod_header.bss_end_offset - mod_header.bss_start_offset) as u32,
        });

        println!("bss_size {:x}", bss_size);

        let mut mem = Vec::<u8>::new();
        mem.resize((memory_size + bss_size) as usize, 0);
        mem[..memory_size as usize].copy_from_slice(&self.file_content[..memory_size as usize]);
        mem
    }

    pub fn parse(path: &String) -> io::Result<Self> {
        let file = File::open(path)?;
        Self::new(file)
    }
}
