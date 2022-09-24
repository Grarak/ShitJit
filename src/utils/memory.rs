const PAGE_SIZE: u32 = 4096;

#[inline]
pub fn page_align(size: u32) -> u32 {
    (PAGE_SIZE - 1 + size) & !(PAGE_SIZE - 1)
}
