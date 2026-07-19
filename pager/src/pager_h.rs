use super::*;
use crate::pcache_h::PgHdr;

///* The type used to represent a page number.  The first page in a file
///* is called page 1.  0 is used to represent "not a page".
pub(crate) type Pgno = u32;

///* Handle type for pages.
pub(crate) type DbPage = PgHdr;
