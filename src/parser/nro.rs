use crate::page_align;
use core::slice::SlicePattern;
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
    pub header: NroHeader,
}

impl Nro {
    fn new(mut file: File, header: &NroHeader) -> io::Result<Self> {
        let mut file_content = Vec::<u8>::new();
        file.read_to_end(&mut file_content)?;
        let nro = Nro {
            file_content,
            header: *header,
        };
        if &nro.get_header().magic == NRO_MAGIC {
            Ok(nro)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid nro file"))
        }
    }

    pub fn get_header(&self) -> Option<&NroHeader> {
        let header: &NroHeader = unsafe { mem::transmute(self.file_content.as_ptr()) };
        if &header.magic == NRO_MAGIC {
            Some(header)
        } else {
            None
        }
    }

    fn get_mod_header(&self) -> &ModHeader {
        let header = self.get_header().unwrap();
        let buf = &self.file_content;
        let header: &NroHeader = unsafe { mem::transmute(&buf[header.mod0_offset..]) };
        if &header.magic == NRO_MAGIC {
            Some(header)
        } else {
            None
        }
        match self.file.read_at(buf, self.header.mod0_offset as u64) {
            Ok(_) => {
                Some(*unsafe { transmute::<&mut [u8; size_of::<ModHeader>()], &ModHeader>(buf) })
            }
            Err(_) => None,
        }
    }

    pub fn get_segment(&self, segment: &NroSegmentHeader) -> io::Result<Vec<u8>> {
        let size = segment.size as usize;
        let offset = segment.memory_offset as usize;
        let mut buf = vec![0u8; size];

        let read_len = self
            .file
            .read_at(&mut buf, (size_of::<NroHeader>() + offset) as u64)?;
        if read_len == size {
            Ok(buf)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Can't read segment to the end",
            ))
        }
    }

    pub fn build_memory(&self) -> Vec<u8> {
        let mut memory_size = page_align(self.header.size);
        let bss_size = page_align(match self.get_mod_header() {
            None => self.header.bss_size,
            Some(mod_header) => (mod_header.bss_end_offset - mod_header.bss_start_offset) as u32,
        });

        memory_size += bss_size;

        Vec::new()
    }
}

pub fn parse(path: &String) -> io::Result<Nro> {
    let file = File::open(path)?;

    let buf = &mut [0u8; size_of::<NroHeader>()];
    let read_len = file.read_at(buf, 0)?;

    if read_len < size_of::<NroHeader>() {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid nro file"));
    }

    let header = unsafe { transmute::<&mut [u8; size_of::<NroHeader>()], &NroHeader>(buf) };
    if &header.magic == NRO_MAGIC {
        Ok(Nro::new(file, header))
    } else {
        Err(Error::new(ErrorKind::InvalidData, "Invalid nro file"))
    }
}
