use super::*;
use crate::pager_h::{Pager, Pgno};
use crate::sqlite3_h::Sqlite3PcachePage;

///* Every page in the cache is controlled by an instance of the following
///* structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct PgHdr {
    pub(crate) p_page: *mut Sqlite3PcachePage,
    pub(crate) p_data: *mut (),
    pub(crate) p_extra: *mut (),
    pub(crate) p_cache: *mut PCache,
    pub(crate) p_dirty: *mut PgHdr,
    pub(crate) p_pager: *mut Pager,
    pub(crate) pgno: Pgno,
    pub(crate) flags: u16,
    pub(crate) n_ref: i64,
    pub(crate) p_dirty_next: *mut PgHdr,
    pub(crate) p_dirty_prev: *mut PgHdr,
}
