use super::*;
use crate::pcache_h::PgHdr;

///* Each open file is managed by a separate instance of the "Pager" structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Pager {
    pub(crate) _opaque: [u8; 0],
}

///* The type used to represent a page number.  The first page in a file
///* is called page 1.  0 is used to represent "not a page".
pub(crate) type Pgno = u32;

///* Handle type for pages.
pub(crate) type DbPage = PgHdr;
