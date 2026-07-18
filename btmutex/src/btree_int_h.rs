use super::*;#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Btree {
    pub(crate) db: *mut Sqlite3,
    pub(crate) p_bt: *mut BtShared,
    pub(crate) in_trans: u8,
    pub(crate) sharable: u8,
    pub(crate) locked: u8,
    pub(crate) has_incrblob_cur: u8,
    pub(crate) want_to_lock: i32,
    pub(crate) n_backup: i32,
    pub(crate) i_b_data_version: u32,
    pub(crate) p_next: *mut Btree,
    pub(crate) p_prev: *mut Btree,
    pub(crate) lock: BtLock,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtShared {
    pub(crate) p_pager: *mut Pager,
    pub(crate) db: *mut Sqlite3,
    pub(crate) p_cursor: *mut BtCursor,
    pub(crate) p_page1: *mut MemPage,
    pub(crate) open_flags: u8,
    pub(crate) auto_vacuum: u8,
    pub(crate) incr_vacuum: u8,
    pub(crate) b_do_truncate: u8,
    pub(crate) in_transaction: u8,
    pub(crate) max1byte_payload: u8,
    pub(crate) n_reserve_wanted: u8,
    pub(crate) bts_flags: u16,
    pub(crate) max_local: u16,
    pub(crate) min_local: u16,
    pub(crate) max_leaf: u16,
    pub(crate) min_leaf: u16,
    pub(crate) page_size: u32,
    pub(crate) usable_size: u32,
    pub(crate) n_transaction: i32,
    pub(crate) n_page: u32,
    pub(crate) p_schema: *mut (),
    pub(crate) x_free_schema: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) mutex: *mut Sqlite3Mutex,
    pub(crate) p_has_content: *mut Bitvec,
    pub(crate) n_ref: i32,
    pub(crate) p_next: *mut BtShared,
    pub(crate) p_lock: *mut BtLock,
    pub(crate) p_writer: *mut Btree,
    pub(crate) p_tmp_space: *mut u8,
    pub(crate) n_preformat_size: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtCursor {
    pub(crate) e_state: u8,
    pub(crate) cur_flags: u8,
    pub(crate) cur_pager_flags: u8,
    pub(crate) hints: u8,
    pub(crate) skip_next: i32,
    pub(crate) p_btree: *mut Btree,
    pub(crate) a_overflow: *mut Pgno,
    pub(crate) p_key: *mut (),
    pub(crate) p_bt: *mut BtShared,
    pub(crate) p_next: *mut BtCursor,
    pub(crate) info: CellInfo,
    pub(crate) n_key: i64,
    pub(crate) pgno_root: Pgno,
    pub(crate) i_page: i8,
    pub(crate) cur_int_key: u8,
    pub(crate) ix: u16,
    pub(crate) ai_idx: [u16; 19],
    pub(crate) p_key_info: *mut KeyInfo,
    pub(crate) p_page: *mut MemPage,
    pub(crate) ap_page: [*mut MemPage; 19],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CellInfo {
    pub(crate) n_key: i64,
    pub(crate) p_payload: *mut u8,
    pub(crate) n_payload: u32,
    pub(crate) n_local: u16,
    pub(crate) n_size: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct MemPage {
    pub(crate) is_init: u8,
    pub(crate) int_key: u8,
    pub(crate) int_key_leaf: u8,
    pub(crate) pgno: Pgno,
    pub(crate) leaf: u8,
    pub(crate) hdr_offset: u8,
    pub(crate) child_ptr_size: u8,
    pub(crate) max1byte_payload: u8,
    pub(crate) n_overflow: u8,
    pub(crate) max_local: u16,
    pub(crate) min_local: u16,
    pub(crate) cell_offset: u16,
    pub(crate) n_free: i32,
    pub(crate) n_cell: u16,
    pub(crate) mask_page: u16,
    pub(crate) ai_ovfl: [u16; 4],
    pub(crate) ap_ovfl: [*mut u8; 4],
    pub(crate) p_bt: *mut BtShared,
    pub(crate) a_data: *mut u8,
    pub(crate) a_data_end: *mut u8,
    pub(crate) a_cell_idx: *mut u8,
    pub(crate) a_data_ofst: *mut u8,
    pub(crate) p_db_page: *mut DbPage,
    pub(crate) x_cell_size: Option<unsafe extern "C" fn(*mut MemPage, *mut u8)
        -> u16>,
    pub(crate) x_parse_cell: Option<unsafe extern "C" fn(*mut MemPage,
        *mut u8, *mut CellInfo) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BtLock {
    pub(crate) p_btree: *mut Btree,
    pub(crate) i_table: Pgno,
    pub(crate) e_lock: u8,
    pub(crate) p_next: *mut BtLock,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IntegrityCk {
    pub(crate) p_bt: *mut BtShared,
    pub(crate) p_pager: *mut Pager,
    pub(crate) a_pg_ref: *mut u8,
    pub(crate) n_ck_page: Pgno,
    pub(crate) mx_err: i32,
    pub(crate) n_err: i32,
    pub(crate) rc: i32,
    pub(crate) n_step: u32,
    pub(crate) z_pfx: *const i8,
    pub(crate) v0: Pgno,
    pub(crate) v1: Pgno,
    pub(crate) v2: i32,
    pub(crate) err_msg: StrAccum,
    pub(crate) heap: *mut u32,
    pub(crate) db: *mut Sqlite3,
    pub(crate) n_row: i64,
}