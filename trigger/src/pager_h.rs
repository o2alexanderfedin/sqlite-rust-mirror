use super::*;pub(crate) type Pgno = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Pager {
    pub(crate) _opaque: [u8; 0],
}
pub(crate) type DbPage = PgHdr;