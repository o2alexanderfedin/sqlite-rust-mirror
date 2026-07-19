use super::*;

/// Connection to a write-ahead log (WAL) file. 
///* There is one object of this type for each pager.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Wal {
    pub(crate) _opaque: [u8; 0],
}
