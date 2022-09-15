use std::borrow::BorrowMut;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind};
use std::mem::{size_of, transmute};
use std::os::unix::fs::FileExt;

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
    pub version: u32,
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
    file: File,
    pub header: NroHeader,
}

impl Nro {
    fn new(file: File, header: &NroHeader) -> Self {
        Nro {
            file,
            header: *header,
        }
    }

    pub fn get_segment(&self, segment: &NroSegmentHeader) -> io::Result<Vec<u8>> {
        let size = segment.size as usize;
        let offset = segment.memory_offset as usize;
        let mut buf = vec![0u8; size];

        let read_len = self
            .file
            .read_at(buf.borrow_mut(), (size_of::<NroHeader>() + offset) as u64)?;
        if read_len == size {
            Ok(buf)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Can't read segment to the end",
            ))
        }
    }
}

const NRO_MAGIC: &[u8; 4] = b"NRO0";

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
