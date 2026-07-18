use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtreePayload {
    pub(crate) p_key: *const (),
    pub(crate) n_key: Sqlite3Int64,
    pub(crate) p_data: *const (),
    pub(crate) a_mem: *mut Sqlite3Value,
    pub(crate) n_mem: u16,
    pub(crate) n_data: i32,
    pub(crate) n_zero: i32,
}
