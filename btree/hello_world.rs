#![feature(c_variadic)]
type __darwin_size_t = u64;
type __darwin_intptr_t = i64;
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3 {
    p_vfs: *mut sqlite3_vfs,
    p_vdbe: *mut Vdbe,
    p_dflt_coll: *mut CollSeq,
    mutex: *mut sqlite3_mutex,
    a_db: *mut Db,
    n_db: i32,
    m_db_flags: u32,
    flags: u64,
    last_rowid: i64,
    sz_mmap: i64,
    n_schema_lock: u32,
    open_flags: u32,
    err_code: i32,
    err_byte_offset: i32,
    err_mask: i32,
    i_sys_errno: i32,
    db_opt_flags: u32,
    enc: u8,
    auto_commit: u8,
    temp_store: u8,
    malloc_failed: u8,
    b_benign_malloc: u8,
    dflt_lock_mode: u8,
    next_autovac: i8,
    suppress_err: u8,
    vtab_on_conflict: u8,
    is_transaction_savepoint: u8,
    m_trace: u8,
    no_shared_cache: u8,
    n_sql_exec: u8,
    e_open_state: u8,
    n_fp_digit: u8,
    next_pagesize: i32,
    n_change: i64,
    n_total_change: i64,
    a_limit: [i32; 13],
    n_max_sorter_mmap: i32,
    init: sqlite3InitInfo,
    n_vdbe_active: i32,
    n_vdbe_read: i32,
    n_vdbe_write: i32,
    n_vdbe_exec: i32,
    n_v_destroy: i32,
    n_extension: i32,
    a_extension: *mut *mut (),
    trace: sqlite3_u0,
    p_trace_arg: *mut (),
    x_profile: Option<unsafe extern "C" fn(*mut (), *const i8, u64) -> ()>,
    p_profile_arg: *mut (),
    p_commit_arg: *mut (),
    x_commit_callback: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    p_rollback_arg: *mut (),
    x_rollback_callback: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_update_arg: *mut (),
    x_update_callback: Option<unsafe extern "C" fn(*mut (), i32, *const i8,
        *const i8, i64) -> ()>,
    p_autovac_pages_arg: *mut (),
    x_autovac_destr: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    x_autovac_pages: Option<unsafe extern "C" fn(*mut (), *const i8, u32, u32,
        u32) -> u32>,
    p_parse: *mut Parse,
    x_wal_callback: Option<unsafe extern "C" fn(*mut (), *mut sqlite3,
        *const i8, i32) -> i32>,
    p_wal_arg: *mut (),
    x_coll_needed: Option<unsafe extern "C" fn(*mut (), *mut sqlite3, i32,
        *const i8) -> ()>,
    x_coll_needed16: Option<unsafe extern "C" fn(*mut (), *mut sqlite3, i32,
        *const ()) -> ()>,
    p_coll_needed_arg: *mut (),
    p_err: *mut sqlite3_value,
    u1: sqlite3_u1,
    lookaside: Lookaside,
    x_auth: Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
        *const i8, *const i8) -> i32>,
    p_auth_arg: *mut (),
    x_progress: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    p_progress_arg: *mut (),
    n_progress_ops: u32,
    n_v_trans: i32,
    a_module: Hash,
    p_vtab_ctx: *mut VtabCtx,
    a_v_trans: *mut *mut VTable,
    p_disconnect: *mut VTable,
    a_func: Hash,
    a_coll_seq: Hash,
    busy_handler: BusyHandler,
    a_db_static: [Db; 2],
    p_savepoint: *mut Savepoint,
    n_analysis_limit: i32,
    busy_timeout: i32,
    n_savepoint: i32,
    n_statement: i32,
    n_deferred_cons: i64,
    n_deferred_imm_cons: i64,
    pn_bytes_freed: *mut i32,
    p_db_data: *mut DbClientData,
    n_spill: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_vfs {
    i_version: i32,
    sz_os_file: i32,
    mx_pathname: i32,
    p_next: *mut sqlite3_vfs,
    z_name: *const i8,
    p_app_data: *mut (),
    x_open: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8,
        *mut sqlite3_file, i32, *mut i32) -> i32>,
    x_delete: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8, i32)
        -> i32>,
    x_access: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8, i32,
        *mut i32) -> i32>,
    x_full_pathname: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8,
        i32, *mut i8) -> i32>,
    x_dl_open: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *const i8)
        -> *mut ()>,
    x_dl_error: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32, *mut i8)
        -> ()>,
    x_dl_sym: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *mut (),
        *const i8) -> unsafe extern "C" fn() -> ()>,
    x_dl_close: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *mut ()) -> ()>,
    x_randomness: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32, *mut i8)
        -> i32>,
    x_sleep: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32) -> i32>,
    x_current_time: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *mut f64)
        -> i32>,
    x_get_last_error: Option<unsafe extern "C" fn(*mut sqlite3_vfs, i32,
        *mut i8) -> i32>,
    x_current_time_int64: Option<unsafe extern "C" fn(*mut sqlite3_vfs,
        *mut i64) -> i32>,
    x_set_system_call: Option<unsafe extern "C" fn(*mut sqlite3_vfs,
        *const i8, unsafe extern "C" fn() -> ()) -> i32>,
    x_get_system_call: Option<unsafe extern "C" fn(*mut sqlite3_vfs,
        *const i8) -> unsafe extern "C" fn() -> ()>,
    x_next_system_call: Option<unsafe extern "C" fn(*mut sqlite3_vfs,
        *const i8) -> *const i8>,
}
type sqlite3_filename = *const i8;
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_file {
    p_methods: *const sqlite3_io_methods,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_io_methods {
    i_version: i32,
    x_close: Option<unsafe extern "C" fn(*mut sqlite3_file) -> i32>,
    x_read: Option<unsafe extern "C" fn(*mut sqlite3_file, *mut (), i32, i64)
        -> i32>,
    x_write: Option<unsafe extern "C" fn(*mut sqlite3_file, *const (), i32,
        i64) -> i32>,
    x_truncate: Option<unsafe extern "C" fn(*mut sqlite3_file, i64) -> i32>,
    x_sync: Option<unsafe extern "C" fn(*mut sqlite3_file, i32) -> i32>,
    x_file_size: Option<unsafe extern "C" fn(*mut sqlite3_file, *mut i64)
        -> i32>,
    x_lock: Option<unsafe extern "C" fn(*mut sqlite3_file, i32) -> i32>,
    x_unlock: Option<unsafe extern "C" fn(*mut sqlite3_file, i32) -> i32>,
    x_check_reserved_lock: Option<unsafe extern "C" fn(*mut sqlite3_file,
        *mut i32) -> i32>,
    x_file_control: Option<unsafe extern "C" fn(*mut sqlite3_file, i32,
        *mut ()) -> i32>,
    x_sector_size: Option<unsafe extern "C" fn(*mut sqlite3_file) -> i32>,
    x_device_characteristics: Option<unsafe extern "C" fn(*mut sqlite3_file)
        -> i32>,
    x_shm_map: Option<unsafe extern "C" fn(*mut sqlite3_file, i32, i32, i32,
        *mut *mut ()) -> i32>,
    x_shm_lock: Option<unsafe extern "C" fn(*mut sqlite3_file, i32, i32, i32)
        -> i32>,
    x_shm_barrier: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ()>,
    x_shm_unmap: Option<unsafe extern "C" fn(*mut sqlite3_file, i32) -> i32>,
    x_fetch: Option<unsafe extern "C" fn(*mut sqlite3_file, i64, i32,
        *mut *mut ()) -> i32>,
    x_unfetch: Option<unsafe extern "C" fn(*mut sqlite3_file, i64, *mut ())
        -> i32>,
}
type sqlite_int64 = i64;
type sqlite3_int64 = sqlite_int64;
type sqlite3_syscall_ptr = unsafe extern "C" fn() -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct CollSeq {
    z_name: *mut i8,
    enc: u8,
    p_user: *mut (),
    x_cmp: Option<unsafe extern "C" fn(*mut (), i32, *const (), i32,
        *const ()) -> i32>,
    x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_mutex {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Db {
    z_db_s_name: *mut i8,
    p_bt: *mut Btree,
    safety_level: u8,
    b_sync_set: u8,
    p_schema: *mut Schema,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Btree {
    db: *mut sqlite3,
    p_bt: *mut BtShared,
    in_trans: u8,
    sharable: u8,
    locked: u8,
    has_incrblob_cur: u8,
    want_to_lock: i32,
    n_backup: i32,
    i_b_data_version: u32,
    p_next: *mut Btree,
    p_prev: *mut Btree,
    lock: BtLock,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct BtShared {
    p_pager: *mut Pager,
    db: *mut sqlite3,
    p_cursor: *mut BtCursor,
    p_page1: *mut MemPage,
    open_flags: u8,
    auto_vacuum: u8,
    incr_vacuum: u8,
    b_do_truncate: u8,
    in_transaction: u8,
    max1byte_payload: u8,
    n_reserve_wanted: u8,
    bts_flags: u16,
    max_local: u16,
    min_local: u16,
    max_leaf: u16,
    min_leaf: u16,
    page_size: u32,
    usable_size: u32,
    n_transaction: i32,
    n_page: u32,
    p_schema: *mut (),
    x_free_schema: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    mutex: *mut sqlite3_mutex,
    p_has_content: *mut Bitvec,
    n_ref: i32,
    p_next: *mut BtShared,
    p_lock: *mut BtLock,
    p_writer: *mut Btree,
    p_tmp_space: *mut u8,
    n_preformat_size: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Pager {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct BtCursor {
    e_state: u8,
    cur_flags: u8,
    cur_pager_flags: u8,
    hints: u8,
    skip_next: i32,
    p_btree: *mut Btree,
    a_overflow: *mut Pgno,
    p_key: *mut (),
    p_bt: *mut BtShared,
    p_next: *mut BtCursor,
    info: CellInfo,
    n_key: i64,
    pgno_root: Pgno,
    i_page: i8,
    cur_int_key: u8,
    ix: u16,
    ai_idx: [u16; 19],
    p_key_info: *mut KeyInfo,
    p_page: *mut MemPage,
    ap_page: [*mut MemPage; 19],
}
type Pgno = u32;
#[repr(C)]
#[derive(Copy, Clone)]
struct CellInfo {
    n_key: i64,
    p_payload: *mut u8,
    n_payload: u32,
    n_local: u16,
    n_size: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct KeyInfo {
    n_ref: u32,
    enc: u8,
    n_key_field: u16,
    n_all_field: u16,
    db: *mut sqlite3,
    a_sort_flags: *mut u8,
    a_coll: [*mut CollSeq; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct MemPage {
    is_init: u8,
    int_key: u8,
    int_key_leaf: u8,
    pgno: Pgno,
    leaf: u8,
    hdr_offset: u8,
    child_ptr_size: u8,
    max1byte_payload: u8,
    n_overflow: u8,
    max_local: u16,
    min_local: u16,
    cell_offset: u16,
    n_free: i32,
    n_cell: u16,
    mask_page: u16,
    ai_ovfl: [u16; 4],
    ap_ovfl: [*mut u8; 4],
    p_bt: *mut BtShared,
    a_data: *mut u8,
    a_data_end: *mut u8,
    a_cell_idx: *mut u8,
    a_data_ofst: *mut u8,
    p_db_page: *mut DbPage,
    x_cell_size: Option<unsafe extern "C" fn(*mut MemPage, *mut u8) -> u16>,
    x_parse_cell: Option<unsafe extern "C" fn(*mut MemPage, *mut u8,
        *mut CellInfo) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PgHdr {
    p_page: *mut sqlite3_pcache_page,
    p_data: *mut (),
    p_extra: *mut (),
    p_cache: *mut PCache,
    p_dirty: *mut PgHdr,
    p_pager: *mut Pager,
    pgno: Pgno,
    flags: u16,
    n_ref: i64,
    p_dirty_next: *mut PgHdr,
    p_dirty_prev: *mut PgHdr,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache_page {
    p_buf: *mut (),
    p_extra: *mut (),
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PCache {
    _opaque: [u8; 0],
}
type DbPage = PgHdr;
#[repr(C)]
#[derive(Copy, Clone)]
struct Bitvec {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct BtLock {
    p_btree: *mut Btree,
    i_table: Pgno,
    e_lock: u8,
    p_next: *mut BtLock,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Schema {
    schema_cookie: i32,
    i_generation: i32,
    tbl_hash: Hash,
    idx_hash: Hash,
    trig_hash: Hash,
    fkey_hash: Hash,
    p_seq_tab: *mut Table,
    file_format: u8,
    enc: u8,
    schema_flags: u16,
    cache_size: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Hash {
    htsize: u32,
    count: u32,
    first: *mut HashElem,
    ht: *mut _ht,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct HashElem {
    next: *mut HashElem,
    prev: *mut HashElem,
    data: *mut (),
    p_key: *const i8,
    h: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct _ht {
    count: u32,
    chain: *mut HashElem,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Table {
    z_name: *mut i8,
    a_col: *mut Column,
    p_index: *mut Index,
    z_col_aff: *mut i8,
    p_check: *mut ExprList,
    tnum: Pgno,
    n_tab_ref: u32,
    tab_flags: u32,
    i_p_key: i16,
    n_col: i16,
    n_nv_col: i16,
    n_row_log_est: LogEst,
    sz_tab_row: LogEst,
    key_conf: u8,
    e_tab_type: u8,
    u: Table_u0,
    p_trigger: *mut Trigger,
    p_schema: *mut Schema,
    a_hx: [u8; 16],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Column {
    z_cn_name: *mut i8,
    _bitfield_1: u32,
    affinity: i8,
    sz_est: u8,
    h_name: u8,
    i_dflt: u16,
    col_flags: u16,
}
impl Column {
    fn not_null(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0xfu32) as i32 }
    fn set_not_null(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0xfu32) | ((val & 0xfu32) << 0u32);
    }
    fn e_c_type(&self) -> i32 { ((self._bitfield_1 >> 4u32) & 0xfu32) as i32 }
    fn set_e_c_type(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0xfu32 << 4u32)) | ((val & 0xfu32) << 4u32);
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Index {
    z_name: *mut i8,
    ai_column: *mut i16,
    ai_row_log_est: *mut LogEst,
    p_table: *mut Table,
    z_col_aff: *mut i8,
    p_next: *mut Index,
    p_schema: *mut Schema,
    a_sort_order: *mut u8,
    az_coll: *mut *const i8,
    p_part_idx_where: *mut Expr,
    a_col_expr: *mut ExprList,
    tnum: Pgno,
    sz_idx_row: LogEst,
    n_key_col: u16,
    n_column: u16,
    on_error: u8,
    _bitfield_1: u32,
    col_not_idxed: Bitmask,
}
impl Index {
    fn idx_type(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0x3u32) as i32 }
    fn set_idx_type(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x3u32) | ((val & 0x3u32) << 0u32);
    }
    fn b_unordered(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_b_unordered(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn uniq_not_null(&self) -> i32 {
        ((self._bitfield_1 >> 3u32) & 0x1u32) as i32
    }
    fn set_uniq_not_null(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn is_resized(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_is_resized(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn is_covering(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_is_covering(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn no_skip_scan(&self) -> i32 {
        ((self._bitfield_1 >> 6u32) & 0x1u32) as i32
    }
    fn set_no_skip_scan(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn has_stat1(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_has_stat1(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn b_no_query(&self) -> i32 {
        ((self._bitfield_1 >> 8u32) & 0x1u32) as i32
    }
    fn set_b_no_query(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
    fn b_asc_key_bug(&self) -> i32 {
        ((self._bitfield_1 >> 9u32) & 0x1u32) as i32
    }
    fn set_b_asc_key_bug(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 9u32)) | ((val & 0x1u32) << 9u32);
    }
    fn b_has_v_col(&self) -> i32 {
        ((self._bitfield_1 >> 10u32) & 0x1u32) as i32
    }
    fn set_b_has_v_col(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 10u32)) |
                ((val & 0x1u32) << 10u32);
    }
    fn b_has_expr(&self) -> i32 {
        ((self._bitfield_1 >> 11u32) & 0x1u32) as i32
    }
    fn set_b_has_expr(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 11u32)) |
                ((val & 0x1u32) << 11u32);
    }
}
type LogEst = i16;
#[repr(C)]
#[derive(Copy, Clone)]
struct Expr {
    op: u8,
    aff_expr: i8,
    op2: u8,
    flags: u32,
    u: Expr_u0,
    p_left: *mut Expr,
    p_right: *mut Expr,
    x: Expr_u1,
    n_height: i32,
    i_table: i32,
    i_column: ynVar,
    i_agg: i16,
    w: Expr_u2,
    p_agg_info: *mut AggInfo,
    y: Expr_u3,
}
#[repr(C)]
#[derive(Copy, Clone)]
union Expr_u0 {
    z_token: *mut i8,
    i_value: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
union Expr_u1 {
    p_list: *mut ExprList,
    p_select: *mut Select,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ExprList {
    n_expr: i32,
    n_alloc: i32,
    a: [ExprList_item; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ExprList_item {
    p_expr: *mut Expr,
    z_e_name: *mut i8,
    fg: ExprList_item_s0,
    u: ExprList_item_u1,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ExprList_item_s0 {
    sort_flags: u8,
    _bitfield_1: u32,
}
impl ExprList_item_s0 {
    fn e_e_name(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0x3u32) as i32 }
    fn set_e_e_name(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x3u32) | ((val & 0x3u32) << 0u32);
    }
    fn done(&self) -> i32 { ((self._bitfield_1 >> 2u32) & 0x1u32) as i32 }
    fn set_done(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn reusable(&self) -> i32 { ((self._bitfield_1 >> 3u32) & 0x1u32) as i32 }
    fn set_reusable(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn b_sorter_ref(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_b_sorter_ref(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn b_nulls(&self) -> i32 { ((self._bitfield_1 >> 5u32) & 0x1u32) as i32 }
    fn set_b_nulls(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn b_used(&self) -> i32 { ((self._bitfield_1 >> 6u32) & 0x1u32) as i32 }
    fn set_b_used(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn b_using_term(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_b_using_term(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn b_no_expand(&self) -> i32 {
        ((self._bitfield_1 >> 8u32) & 0x1u32) as i32
    }
    fn set_b_no_expand(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
union ExprList_item_u1 {
    x: ExprList_item_u1_s0,
    i_const_expr_reg: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ExprList_item_u1_s0 {
    i_order_by_col: u16,
    i_alias: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Select {
    op: u8,
    n_select_row: LogEst,
    sel_flags: u32,
    i_limit: i32,
    i_offset: i32,
    sel_id: u32,
    p_e_list: *mut ExprList,
    p_src: *mut SrcList,
    p_where: *mut Expr,
    p_group_by: *mut ExprList,
    p_having: *mut Expr,
    p_order_by: *mut ExprList,
    p_prior: *mut Select,
    p_next: *mut Select,
    p_limit: *mut Expr,
    p_with: *mut With,
    p_win: *mut Window,
    p_win_defn: *mut Window,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SrcList {
    n_src: i32,
    n_alloc: u32,
    a: [SrcItem; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SrcItem {
    z_name: *mut i8,
    z_alias: *mut i8,
    p_s_tab: *mut Table,
    fg: SrcItem_s0,
    i_cursor: i32,
    col_used: Bitmask,
    u1: SrcItem_u1,
    u2: SrcItem_u2,
    u3: SrcItem_u3,
    u4: SrcItem_u4,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SrcItem_s0 {
    jointype: u8,
    _bitfield_1: u32,
}
impl SrcItem_s0 {
    fn not_indexed(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_not_indexed(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn is_indexed_by(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_is_indexed_by(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn is_subquery(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_is_subquery(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn is_tab_func(&self) -> i32 {
        ((self._bitfield_1 >> 3u32) & 0x1u32) as i32
    }
    fn set_is_tab_func(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn is_correlated(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_is_correlated(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn is_materialized(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_is_materialized(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn via_coroutine(&self) -> i32 {
        ((self._bitfield_1 >> 6u32) & 0x1u32) as i32
    }
    fn set_via_coroutine(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn is_recursive(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_is_recursive(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn from_ddl(&self) -> i32 { ((self._bitfield_1 >> 8u32) & 0x1u32) as i32 }
    fn set_from_ddl(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
    fn is_cte(&self) -> i32 { ((self._bitfield_1 >> 9u32) & 0x1u32) as i32 }
    fn set_is_cte(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 9u32)) | ((val & 0x1u32) << 9u32);
    }
    fn not_cte(&self) -> i32 { ((self._bitfield_1 >> 10u32) & 0x1u32) as i32 }
    fn set_not_cte(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 10u32)) |
                ((val & 0x1u32) << 10u32);
    }
    fn is_using(&self) -> i32 {
        ((self._bitfield_1 >> 11u32) & 0x1u32) as i32
    }
    fn set_is_using(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 11u32)) |
                ((val & 0x1u32) << 11u32);
    }
    fn is_on(&self) -> i32 { ((self._bitfield_1 >> 12u32) & 0x1u32) as i32 }
    fn set_is_on(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 12u32)) |
                ((val & 0x1u32) << 12u32);
    }
    fn is_synth_using(&self) -> i32 {
        ((self._bitfield_1 >> 13u32) & 0x1u32) as i32
    }
    fn set_is_synth_using(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 13u32)) |
                ((val & 0x1u32) << 13u32);
    }
    fn is_nested_from(&self) -> i32 {
        ((self._bitfield_1 >> 14u32) & 0x1u32) as i32
    }
    fn set_is_nested_from(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 14u32)) |
                ((val & 0x1u32) << 14u32);
    }
    fn rowid_used(&self) -> i32 {
        ((self._bitfield_1 >> 15u32) & 0x1u32) as i32
    }
    fn set_rowid_used(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 15u32)) |
                ((val & 0x1u32) << 15u32);
    }
    fn fixed_schema(&self) -> i32 {
        ((self._bitfield_1 >> 16u32) & 0x1u32) as i32
    }
    fn set_fixed_schema(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 16u32)) |
                ((val & 0x1u32) << 16u32);
    }
    fn had_schema(&self) -> i32 {
        ((self._bitfield_1 >> 17u32) & 0x1u32) as i32
    }
    fn set_had_schema(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 17u32)) |
                ((val & 0x1u32) << 17u32);
    }
    fn from_exists(&self) -> i32 {
        ((self._bitfield_1 >> 18u32) & 0x1u32) as i32
    }
    fn set_from_exists(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 18u32)) |
                ((val & 0x1u32) << 18u32);
    }
}
type sqlite_uint64 = u64;
type Bitmask = u64;
#[repr(C)]
#[derive(Copy, Clone)]
union SrcItem_u1 {
    z_indexed_by: *mut i8,
    p_func_arg: *mut ExprList,
    n_row: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
union SrcItem_u2 {
    p_ib_index: *mut Index,
    p_cte_use: *mut CteUse,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CteUse {
    n_use: i32,
    addr_m9e: i32,
    reg_rtn: i32,
    i_cur: i32,
    n_row_est: LogEst,
    e_m10d: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
union SrcItem_u3 {
    p_on: *mut Expr,
    p_using: *mut IdList,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IdList {
    n_id: i32,
    a: [IdList_item; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IdList_item {
    z_name: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
union SrcItem_u4 {
    p_schema: *mut Schema,
    z_database: *mut i8,
    p_subq: *mut Subquery,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Subquery {
    p_select: *mut Select,
    addr_fill_sub: i32,
    reg_return: i32,
    reg_result: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct With {
    n_cte: i32,
    b_view: i32,
    p_outer: *mut With,
    a: [Cte; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Cte {
    z_name: *mut i8,
    p_cols: *mut ExprList,
    p_select: *mut Select,
    z_cte_err: *const i8,
    p_use: *mut CteUse,
    e_m10d: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Window {
    z_name: *mut i8,
    z_base: *mut i8,
    p_partition: *mut ExprList,
    p_order_by: *mut ExprList,
    e_frm_type: u8,
    e_start: u8,
    e_end: u8,
    b_implicit_frame: u8,
    e_exclude: u8,
    p_start: *mut Expr,
    p_end: *mut Expr,
    pp_this: *mut *mut Window,
    p_next_win: *mut Window,
    p_filter: *mut Expr,
    p_w_func: *mut FuncDef,
    i_eph_csr: i32,
    reg_accum: i32,
    reg_result: i32,
    csr_app: i32,
    reg_app: i32,
    reg_part: i32,
    p_owner: *mut Expr,
    n_buffer_col: i32,
    i_arg_col: i32,
    reg_one: i32,
    reg_start_rowid: i32,
    reg_end_rowid: i32,
    b_expr_args: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FuncDef {
    n_arg: i16,
    func_flags: u32,
    p_user_data: *mut (),
    p_next: *mut FuncDef,
    x_s_func: Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
        *mut *mut sqlite3_value) -> ()>,
    x_finalize: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    x_value: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    x_inverse: Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
        *mut *mut sqlite3_value) -> ()>,
    z_name: *const i8,
    u: FuncDef_u0,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_context {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_value {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
union FuncDef_u0 {
    p_hash: *mut FuncDef,
    p_destructor: *mut FuncDestructor,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FuncDestructor {
    n_ref: i32,
    x_destroy: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_user_data: *mut (),
}
type ynVar = i16;
#[repr(C)]
#[derive(Copy, Clone)]
union Expr_u2 {
    i_join: i32,
    i_ofst: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct AggInfo {
    direct_mode: u8,
    use_sorting_idx: u8,
    n_sorting_column: u32,
    sorting_idx: i32,
    sorting_idx_p_tab: i32,
    i_first_reg: i32,
    p_group_by: *mut ExprList,
    a_col: *mut AggInfo_col,
    n_column: i32,
    n_accumulator: i32,
    a_func: *mut AggInfo_func,
    n_func: i32,
    sel_id: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct AggInfo_col {
    p_tab: *mut Table,
    p_c_expr: *mut Expr,
    i_table: i32,
    i_column: i32,
    i_sorter_column: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct AggInfo_func {
    p_f_expr: *mut Expr,
    p_func: *mut FuncDef,
    i_distinct: i32,
    i_dist_addr: i32,
    i_ob_tab: i32,
    b_ob_payload: u8,
    b_ob_unique: u8,
    b_use_subtype: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
union Expr_u3 {
    p_tab: *mut Table,
    p_win: *mut Window,
    n_reg: i32,
    sub: Expr_u3_s0,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Expr_u3_s0 {
    i_addr: i32,
    reg_return: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
union Table_u0 {
    tab: Table_u0_s0,
    view: Table_u0_s1,
    vtab: Table_u0_s2,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Table_u0_s0 {
    add_col_offset: i32,
    p_f_key: *mut FKey,
    p_dflt_list: *mut ExprList,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FKey {
    p_from: *mut Table,
    p_next_from: *mut FKey,
    z_to: *mut i8,
    p_next_to: *mut FKey,
    p_prev_to: *mut FKey,
    n_col: i32,
    is_deferred: u8,
    a_action: [u8; 2],
    ap_trigger: [*mut Trigger; 2],
    a_col: [sColMap; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Trigger {
    z_name: *mut i8,
    table: *mut i8,
    op: u8,
    tr_tm: u8,
    b_returning: u8,
    p_when: *mut Expr,
    p_columns: *mut IdList,
    p_schema: *mut Schema,
    p_tab_schema: *mut Schema,
    step_list: *mut TriggerStep,
    p_next: *mut Trigger,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct TriggerStep {
    op: u8,
    orconf: u8,
    p_trig: *mut Trigger,
    p_select: *mut Select,
    p_src: *mut SrcList,
    p_where: *mut Expr,
    p_expr_list: *mut ExprList,
    p_id_list: *mut IdList,
    p_upsert: *mut Upsert,
    z_span: *mut i8,
    p_next: *mut TriggerStep,
    p_last: *mut TriggerStep,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Upsert {
    p_upsert_target: *mut ExprList,
    p_upsert_target_where: *mut Expr,
    p_upsert_set: *mut ExprList,
    p_upsert_where: *mut Expr,
    p_next_upsert: *mut Upsert,
    is_do_update: u8,
    is_dup: u8,
    p_to_free: *mut (),
    p_upsert_idx: *mut Index,
    p_upsert_src: *mut SrcList,
    reg_data: i32,
    i_data_cur: i32,
    i_idx_cur: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sColMap {
    i_from: i32,
    z_col: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Table_u0_s1 {
    p_select: *mut Select,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Table_u0_s2 {
    n_arg: i32,
    az_arg: *mut *mut i8,
    p: *mut VTable,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VTable {
    db: *mut sqlite3,
    p_mod: *mut Module,
    p_vtab: *mut sqlite3_vtab,
    n_ref: i32,
    b_constraint: u8,
    b_all_schemas: u8,
    e_vtab_risk: u8,
    i_savepoint: i32,
    p_next: *mut VTable,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Module {
    p_module: *const sqlite3_module,
    z_name: *const i8,
    n_ref_module: i32,
    p_aux: *mut (),
    x_destroy: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_epo_tab: *mut Table,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_module {
    i_version: i32,
    x_create: Option<unsafe extern "C" fn(*mut sqlite3, *mut (), i32,
        *const *const i8, *mut *mut sqlite3_vtab, *mut *mut i8) -> i32>,
    x_connect: Option<unsafe extern "C" fn(*mut sqlite3, *mut (), i32,
        *const *const i8, *mut *mut sqlite3_vtab, *mut *mut i8) -> i32>,
    x_best_index: Option<unsafe extern "C" fn(*mut sqlite3_vtab,
        *mut sqlite3_index_info) -> i32>,
    x_disconnect: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_destroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_open: Option<unsafe extern "C" fn(*mut sqlite3_vtab,
        *mut *mut sqlite3_vtab_cursor) -> i32>,
    x_close: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> i32>,
    x_filter: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor, i32,
        *const i8, i32, *mut *mut sqlite3_value) -> i32>,
    x_next: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> i32>,
    x_eof: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> i32>,
    x_column: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor,
        *mut sqlite3_context, i32) -> i32>,
    x_rowid: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor, *mut i64)
        -> i32>,
    x_update: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32,
        *mut *mut sqlite3_value, *mut i64) -> i32>,
    x_begin: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_sync: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_commit: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_rollback: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> i32>,
    x_find_function: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32,
        *const i8,
        *mut unsafe extern "C" fn(*mut sqlite3_context, i32,
                *mut *mut sqlite3_value) -> (), *mut *mut ()) -> i32>,
    x_rename: Option<unsafe extern "C" fn(*mut sqlite3_vtab, *const i8)
        -> i32>,
    x_savepoint: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32) -> i32>,
    x_release: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32) -> i32>,
    x_rollback_to: Option<unsafe extern "C" fn(*mut sqlite3_vtab, i32)
        -> i32>,
    x_shadow_name: Option<unsafe extern "C" fn(*const i8) -> i32>,
    x_integrity: Option<unsafe extern "C" fn(*mut sqlite3_vtab, *const i8,
        *const i8, i32, *mut *mut i8) -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_vtab {
    p_module: *const sqlite3_module,
    n_ref: i32,
    z_err_msg: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_index_info {
    n_constraint: i32,
    a_constraint: *mut sqlite3_index_constraint,
    n_order_by: i32,
    a_order_by: *mut sqlite3_index_orderby,
    a_constraint_usage: *mut sqlite3_index_constraint_usage,
    idx_num: i32,
    idx_str: *mut i8,
    need_to_free_idx_str: i32,
    order_by_consumed: i32,
    estimated_cost: f64,
    estimated_rows: sqlite3_int64,
    idx_flags: i32,
    col_used: sqlite3_uint64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_index_constraint {
    i_column: i32,
    op: u8,
    usable: u8,
    i_term_offset: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_index_orderby {
    i_column: i32,
    desc: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_index_constraint_usage {
    argv_index: i32,
    omit: u8,
}
type sqlite3_uint64 = sqlite_uint64;
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_vtab_cursor {
    p_vtab: *mut sqlite3_vtab,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3InitInfo {
    new_tnum: Pgno,
    i_db: u8,
    busy: u8,
    _bitfield_1: u32,
    az_init: *mut *const i8,
}
impl sqlite3InitInfo {
    fn orphan_trigger(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_orphan_trigger(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn imposter_table(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x3u32) as i32
    }
    fn set_imposter_table(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x3u32 << 1u32)) | ((val & 0x3u32) << 1u32);
    }
    fn reopen_memdb(&self) -> i32 {
        ((self._bitfield_1 >> 3u32) & 0x1u32) as i32
    }
    fn set_reopen_memdb(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
union sqlite3_u0 {
    x_legacy: Option<unsafe extern "C" fn(*mut (), *const i8) -> ()>,
    x_v2: Option<unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Parse {
    db: *mut sqlite3,
    z_err_msg: *mut i8,
    p_vdbe: *mut Vdbe,
    rc: i32,
    n_query_loop: LogEst,
    nested: u8,
    n_temp_reg: u8,
    is_multi_write: u8,
    disable_lookaside: u8,
    prep_flags: u8,
    within_rj_subrtn: u8,
    m_subrtn_sig: u8,
    e_trigger_op: u8,
    e_orconf: u8,
    _bitfield_1: u32,
    n_range_reg: i32,
    i_range_reg: i32,
    n_err: i32,
    n_tab: i32,
    n_mem: i32,
    i_self_tab: i32,
    n_nest_sel: i32,
    n_label: i32,
    n_label_alloc: i32,
    a_label: *mut i32,
    p_const_expr: *mut ExprList,
    p_idx_epr: *mut IndexedExpr,
    p_idx_part_expr: *mut IndexedExpr,
    write_mask: yDbMask,
    cookie_mask: yDbMask,
    n_max_arg: i32,
    n_select: i32,
    n_progress_steps: u32,
    n_table_lock: i32,
    p_toplevel: *mut Parse,
    p_trigger_tab: *mut Table,
    p_trigger_prg: *mut TriggerPrg,
    p_cleanup: *mut ParseCleanup,
    a_temp_reg: [i32; 8],
    p_outer_parse: *mut Parse,
    s_name_token: Token,
    oldmask: u32,
    newmask: u32,
    u1: Parse_u0,
    p_ainc: *mut AutoincInfo,
    a_table_lock: *mut TableLock,
    s_last_token: Token,
    n_var: ynVar,
    i_pk_sort_order: u8,
    explain: u8,
    e_parse_mode: u8,
    n_vtab_lock: i32,
    n_height: i32,
    addr_explain: i32,
    p_v_list: *mut VList,
    p_reprepare: *mut Vdbe,
    z_tail: *const i8,
    p_new_table: *mut Table,
    p_new_index: *mut Index,
    p_new_trigger: *mut Trigger,
    z_auth_context: *const i8,
    s_arg: Token,
    ap_vtab_lock: *mut *mut Table,
    p_with: *mut With,
    p_rename: *mut RenameToken,
}
impl Parse {
    fn disable_triggers(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_disable_triggers(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn may_abort(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_may_abort(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn has_compound(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_has_compound(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn b_returning(&self) -> i32 {
        ((self._bitfield_1 >> 3u32) & 0x1u32) as i32
    }
    fn set_b_returning(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn b_has_exists(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_b_has_exists(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn col_names_set(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_col_names_set(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn b_has_with(&self) -> i32 {
        ((self._bitfield_1 >> 6u32) & 0x1u32) as i32
    }
    fn set_b_has_with(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn ok_const_factor(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_ok_const_factor(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn check_schema(&self) -> i32 {
        ((self._bitfield_1 >> 8u32) & 0x1u32) as i32
    }
    fn set_check_schema(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
    fn uses_ainc(&self) -> i32 {
        ((self._bitfield_1 >> 9u32) & 0x1u32) as i32
    }
    fn set_uses_ainc(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 9u32)) | ((val & 0x1u32) << 9u32);
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Vdbe {
    _opaque: [u8; 0],
}
type bft = u32;
#[repr(C)]
#[derive(Copy, Clone)]
struct IndexedExpr {
    p_expr: *mut Expr,
    i_data_cur: i32,
    i_idx_cur: i32,
    i_idx_col: i32,
    b_maybe_null_row: u8,
    aff: u8,
    p_ie_next: *mut IndexedExpr,
}
type yDbMask = u32;
#[repr(C)]
#[derive(Copy, Clone)]
struct TriggerPrg {
    p_trigger: *mut Trigger,
    p_next: *mut TriggerPrg,
    p_program: *mut SubProgram,
    orconf: i32,
    a_colmask: [u32; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SubProgram {
    a_op: *mut VdbeOp,
    n_op: i32,
    n_mem: i32,
    n_csr: i32,
    a_once: *mut u8,
    token: *mut (),
    p_next: *mut SubProgram,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VdbeOp {
    opcode: u8,
    p4type: i8,
    p5: u16,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: p4union,
}
#[repr(C)]
#[derive(Copy, Clone)]
union p4union {
    i: i32,
    p: *mut (),
    z: *mut i8,
    p_i64: *mut i64,
    p_real: *mut f64,
    p_func: *mut FuncDef,
    p_ctx: *mut sqlite3_context,
    p_coll: *mut CollSeq,
    p_mem: *mut Mem,
    p_vtab: *mut VTable,
    p_key_info: *mut KeyInfo,
    ai: *mut u32,
    p_program: *mut SubProgram,
    p_tab: *mut Table,
    p_subrtn_sig: *mut SubrtnSig,
    p_idx: *mut Index,
}
type Mem = sqlite3_value;
#[repr(C)]
#[derive(Copy, Clone)]
struct SubrtnSig {
    sel_id: i32,
    b_complete: u8,
    z_aff: *mut i8,
    i_table: i32,
    i_addr: i32,
    reg_return: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ParseCleanup {
    p_next: *mut ParseCleanup,
    p_ptr: *mut (),
    x_cleanup: Option<unsafe extern "C" fn(*mut sqlite3, *mut ()) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Token {
    z: *const i8,
    n: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
union Parse_u0 {
    cr: Parse_u0_s0,
    d: Parse_u0_s1,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Parse_u0_s0 {
    addr_cr_tab: i32,
    reg_rowid: i32,
    reg_root: i32,
    constraint_name: Token,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Parse_u0_s1 {
    p_returning: *mut Returning,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Returning {
    p_parse: *mut Parse,
    p_return_el: *mut ExprList,
    ret_trig: Trigger,
    ret_t_step: TriggerStep,
    i_ret_cur: i32,
    n_ret_col: i32,
    i_ret_reg: i32,
    z_name: [i8; 40],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct AutoincInfo {
    p_next: *mut AutoincInfo,
    p_tab: *mut Table,
    i_db: i32,
    reg_ctr: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct TableLock {
    _opaque: [u8; 0],
}
type VList = i32;
#[repr(C)]
#[derive(Copy, Clone)]
struct RenameToken {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
union sqlite3_u1 {
    is_interrupted: i32,
    not_used1: f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Lookaside {
    b_disable: u32,
    sz: u16,
    sz_true: u16,
    b_malloced: u8,
    n_slot: u32,
    an_stat: [u32; 3],
    p_init: *mut LookasideSlot,
    p_free: *mut LookasideSlot,
    p_small_init: *mut LookasideSlot,
    p_small_free: *mut LookasideSlot,
    p_middle: *mut (),
    p_start: *mut (),
    p_end: *mut (),
    p_true_end: *mut (),
}
#[repr(C)]
#[derive(Copy, Clone)]
struct LookasideSlot {
    p_next: *mut LookasideSlot,
}
type sqlite3_xauth =
    unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, *const i8,
        *const i8) -> i32;
#[repr(C)]
#[derive(Copy, Clone)]
struct VtabCtx {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct BusyHandler {
    x_busy_handler: Option<unsafe extern "C" fn(*mut (), i32) -> i32>,
    p_busy_arg: *mut (),
    n_busy: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Savepoint {
    z_name: *mut i8,
    n_deferred_cons: i64,
    n_deferred_imm_cons: i64,
    p_next: *mut Savepoint,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct DbClientData {
    p_next: *mut DbClientData,
    p_data: *mut (),
    x_destructor: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    z_name: [i8; 0],
}
type sqlite3_callback =
    unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32;
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_api_routines {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_mem_methods {
    x_malloc: Option<unsafe extern "C" fn(i32) -> *mut ()>,
    x_free: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    x_realloc: Option<unsafe extern "C" fn(*mut (), i32) -> *mut ()>,
    x_size: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    x_roundup: Option<unsafe extern "C" fn(i32) -> i32>,
    x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_app_data: *mut (),
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_stmt {
    _opaque: [u8; 0],
}
type sqlite3_destructor_type = unsafe extern "C" fn(*mut ()) -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlite3Config {
    b_memstat: i32,
    b_core_mutex: u8,
    b_full_mutex: u8,
    b_open_uri: u8,
    b_use_cis: u8,
    b_small_malloc: u8,
    b_extra_schema_checks: u8,
    mx_strlen: i32,
    never_corrupt: i32,
    sz_lookaside: i32,
    n_lookaside: i32,
    n_stmt_spill: i32,
    m: sqlite3_mem_methods,
    mutex: sqlite3_mutex_methods,
    pcache2: sqlite3_pcache_methods2,
    p_heap: *mut (),
    n_heap: i32,
    mn_req: i32,
    mx_req: i32,
    sz_mmap: sqlite3_int64,
    mx_mmap: sqlite3_int64,
    p_page: *mut (),
    sz_page: i32,
    n_page: i32,
    mx_parser_stack: i32,
    shared_cache_enabled: i32,
    sz_pma: u32,
    is_init: i32,
    in_progress: i32,
    is_mutex_init: i32,
    is_malloc_init: i32,
    is_p_cache_init: i32,
    n_ref_init_mutex: i32,
    p_init_mutex: *mut sqlite3_mutex,
    x_log: Option<unsafe extern "C" fn(*mut (), i32, *const i8) -> ()>,
    p_log_arg: *mut (),
    mx_memdb_size: sqlite3_int64,
    x_test_callback: Option<unsafe extern "C" fn(i32) -> i32>,
    b_localtime_fault: i32,
    x_alt_localtime: Option<unsafe extern "C" fn(*const (), *mut ()) -> i32>,
    i_once_reset_threshold: i32,
    sz_sorter_ref: u32,
    i_prng_seed: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_mutex_methods {
    x_mutex_init: Option<unsafe extern "C" fn() -> i32>,
    x_mutex_end: Option<unsafe extern "C" fn() -> i32>,
    x_mutex_alloc: Option<unsafe extern "C" fn(i32) -> *mut sqlite3_mutex>,
    x_mutex_free: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    x_mutex_enter: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    x_mutex_try: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> i32>,
    x_mutex_leave: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    x_mutex_held: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> i32>,
    x_mutex_notheld: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache_methods2 {
    i_version: i32,
    p_arg: *mut (),
    x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    x_create: Option<unsafe extern "C" fn(i32, i32, i32)
        -> *mut sqlite3_pcache>,
    x_cachesize: Option<unsafe extern "C" fn(*mut sqlite3_pcache, i32) -> ()>,
    x_pagecount: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> i32>,
    x_fetch: Option<unsafe extern "C" fn(*mut sqlite3_pcache, u32, i32)
        -> *mut sqlite3_pcache_page>,
    x_unpin: Option<unsafe extern "C" fn(*mut sqlite3_pcache,
        *mut sqlite3_pcache_page, i32) -> ()>,
    x_rekey: Option<unsafe extern "C" fn(*mut sqlite3_pcache,
        *mut sqlite3_pcache_page, u32, u32) -> ()>,
    x_truncate: Option<unsafe extern "C" fn(*mut sqlite3_pcache, u32) -> ()>,
    x_destroy: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
    x_shrink: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache {
    _opaque: [u8; 0],
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_enable_shared_cache(enable: i32) -> i32 {
    unsafe { sqlite3Config.shared_cache_enabled = enable; return 0; }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_blob {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_str {
    db: *mut sqlite3,
    z_text: *mut i8,
    n_alloc: u32,
    mx_alloc: u32,
    n_char: u32,
    acc_error: u8,
    printf_flags: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache_methods {
    p_arg: *mut (),
    x_init: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    x_shutdown: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    x_create: Option<unsafe extern "C" fn(i32, i32) -> *mut sqlite3_pcache>,
    x_cachesize: Option<unsafe extern "C" fn(*mut sqlite3_pcache, i32) -> ()>,
    x_pagecount: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> i32>,
    x_fetch: Option<unsafe extern "C" fn(*mut sqlite3_pcache, u32, i32)
        -> *mut ()>,
    x_unpin: Option<unsafe extern "C" fn(*mut sqlite3_pcache, *mut (), i32)
        -> ()>,
    x_rekey: Option<unsafe extern "C" fn(*mut sqlite3_pcache, *mut (), u32,
        u32) -> ()>,
    x_truncate: Option<unsafe extern "C" fn(*mut sqlite3_pcache, u32) -> ()>,
    x_destroy: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_backup {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_snapshot {
    hidden: [u8; 48],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_rtree_geometry {
    p_context: *mut (),
    n_param: i32,
    a_param: *mut sqlite3_rtree_dbl,
    p_user: *mut (),
    x_del_user: Option<unsafe extern "C" fn(*mut ()) -> ()>,
}
type sqlite3_rtree_dbl = f64;
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_rtree_query_info {
    p_context: *mut (),
    n_param: i32,
    a_param: *mut sqlite3_rtree_dbl,
    p_user: *mut (),
    x_del_user: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    a_coord: *mut sqlite3_rtree_dbl,
    an_queue: *mut u32,
    n_coord: i32,
    i_level: i32,
    mx_level: i32,
    i_rowid: sqlite3_int64,
    r_parent_score: sqlite3_rtree_dbl,
    e_parent_within: i32,
    e_within: i32,
    r_score: sqlite3_rtree_dbl,
    ap_sql_param: *mut *mut sqlite3_value,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5ExtensionApi {
    i_version: i32,
    x_user_data: Option<unsafe extern "C" fn(*mut Fts5Context) -> *mut ()>,
    x_column_count: Option<unsafe extern "C" fn(*mut Fts5Context) -> i32>,
    x_row_count: Option<unsafe extern "C" fn(*mut Fts5Context, *mut i64)
        -> i32>,
    x_column_total_size: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut i64) -> i32>,
    x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Context, *const i8, i32,
        *mut (),
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
    x_phrase_count: Option<unsafe extern "C" fn(*mut Fts5Context) -> i32>,
    x_phrase_size: Option<unsafe extern "C" fn(*mut Fts5Context, i32) -> i32>,
    x_inst_count: Option<unsafe extern "C" fn(*mut Fts5Context, *mut i32)
        -> i32>,
    x_inst: Option<unsafe extern "C" fn(*mut Fts5Context, i32, *mut i32,
        *mut i32, *mut i32) -> i32>,
    x_rowid: Option<unsafe extern "C" fn(*mut Fts5Context) -> i64>,
    x_column_text: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut *const i8, *mut i32) -> i32>,
    x_column_size: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut i32) -> i32>,
    x_query_phrase: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut (),
        unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
                *mut ()) -> i32) -> i32>,
    x_set_auxdata: Option<unsafe extern "C" fn(*mut Fts5Context, *mut (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    x_get_auxdata: Option<unsafe extern "C" fn(*mut Fts5Context, i32)
        -> *mut ()>,
    x_phrase_first: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut Fts5PhraseIter, *mut i32, *mut i32) -> i32>,
    x_phrase_next: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut Fts5PhraseIter, *mut i32, *mut i32) -> ()>,
    x_phrase_first_column: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut Fts5PhraseIter, *mut i32) -> i32>,
    x_phrase_next_column: Option<unsafe extern "C" fn(*mut Fts5Context,
        *mut Fts5PhraseIter, *mut i32) -> ()>,
    x_query_token: Option<unsafe extern "C" fn(*mut Fts5Context, i32, i32,
        *mut *const i8, *mut i32) -> i32>,
    x_inst_token: Option<unsafe extern "C" fn(*mut Fts5Context, i32, i32,
        *mut *const i8, *mut i32) -> i32>,
    x_column_locale: Option<unsafe extern "C" fn(*mut Fts5Context, i32,
        *mut *const i8, *mut i32) -> i32>,
    x_tokenize_v2: Option<unsafe extern "C" fn(*mut Fts5Context, *const i8,
        i32, *const i8, i32, *mut (),
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Context {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5PhraseIter {
    a: *const u8,
    b: *const u8,
}
type fts5_extension_function =
    unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
        *mut sqlite3_context, i32, *mut *mut sqlite3_value) -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct Fts5Tokenizer {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fts5_tokenizer_v2 {
    i_version: i32,
    x_create: Option<unsafe extern "C" fn(*mut (), *mut *const i8, i32,
        *mut *mut Fts5Tokenizer) -> i32>,
    x_delete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>,
    x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Tokenizer, *mut (), i32,
        *const i8, i32, *const i8, i32,
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fts5_tokenizer {
    x_create: Option<unsafe extern "C" fn(*mut (), *mut *const i8, i32,
        *mut *mut Fts5Tokenizer) -> i32>,
    x_delete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>,
    x_tokenize: Option<unsafe extern "C" fn(*mut Fts5Tokenizer, *mut (), i32,
        *const i8, i32,
        unsafe extern "C" fn(*mut (), i32, *const i8, i32, i32, i32) -> i32)
        -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct fts5_api {
    i_version: i32,
    x_create_tokenizer: Option<unsafe extern "C" fn(*mut fts5_api, *const i8,
        *mut (), *mut fts5_tokenizer, unsafe extern "C" fn(*mut ()) -> ())
        -> i32>,
    x_find_tokenizer: Option<unsafe extern "C" fn(*mut fts5_api, *const i8,
        *mut *mut (), *mut fts5_tokenizer) -> i32>,
    x_create_function: Option<unsafe extern "C" fn(*mut fts5_api, *const i8,
        *mut (),
        unsafe extern "C" fn(*const Fts5ExtensionApi, *mut Fts5Context,
                *mut sqlite3_context, i32, *mut *mut sqlite3_value) -> (),
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    x_create_tokenizer_v2: Option<unsafe extern "C" fn(*mut fts5_api,
        *const i8, *mut (), *mut fts5_tokenizer_v2,
        unsafe extern "C" fn(*mut ()) -> ()) -> i32>,
    x_find_tokenizer_v2: Option<unsafe extern "C" fn(*mut fts5_api, *const i8,
        *mut *mut (), *mut *mut fts5_tokenizer_v2) -> i32>,
}
type tRowcnt = u64;
type uptr = u64;
#[repr(C)]
#[derive(Copy, Clone)]
struct AuthContext {
    z_auth_context: *const i8,
    p_parse: *mut Parse,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct DbFixer {
    p_parse: *mut Parse,
    w: Walker,
    p_schema: *mut Schema,
    b_temp: u8,
    z_db: *const i8,
    z_type: *const i8,
    p_name: *const Token,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Walker {
    p_parse: *mut Parse,
    x_expr_callback: Option<unsafe extern "C" fn(*mut Walker, *mut Expr)
        -> i32>,
    x_select_callback: Option<unsafe extern "C" fn(*mut Walker, *mut Select)
        -> i32>,
    x_select_callback2: Option<unsafe extern "C" fn(*mut Walker, *mut Select)
        -> ()>,
    walker_depth: i32,
    e_code: u16,
    m_w_flags: u16,
    u: Walker_u0,
}
#[repr(C)]
#[derive(Copy, Clone)]
union Walker_u0 {
    p_nc: *mut NameContext,
    n: i32,
    i_cur: i32,
    sz: i32,
    p_src_list: *mut SrcList,
    p_c_cur_hint: *mut CCurHint,
    p_ref_src_list: *mut RefSrcList,
    ai_col: *mut i32,
    p_idx_cover: *mut IdxCover,
    p_group_by: *mut ExprList,
    p_select: *mut Select,
    p_rewrite: *mut WindowRewrite,
    p_const: *mut WhereConst,
    p_rename: *mut RenameCtx,
    p_tab: *mut Table,
    p_cov_idx_ck: *mut CoveringIndexCheck,
    p_src_item: *mut SrcItem,
    p_fix: *mut DbFixer,
    a_mem: *mut Mem,
    p_check_on_ctx: *mut CheckOnCtx,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct NameContext {
    p_parse: *mut Parse,
    p_src_list: *mut SrcList,
    u_nc: NameContext_u0,
    p_next: *mut NameContext,
    n_ref: i32,
    n_nc_err: i32,
    nc_flags: i32,
    n_nested_select: u32,
    p_win_select: *mut Select,
}
#[repr(C)]
#[derive(Copy, Clone)]
union NameContext_u0 {
    p_e_list: *mut ExprList,
    p_agg_info: *mut AggInfo,
    p_upsert: *mut Upsert,
    i_base_reg: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FpDecode {
    n: i32,
    i_dp: i32,
    z: *mut i8,
    z_buf: [i8; 21],
    sign: i8,
    is_special: i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct FuncDefHash {
    a: [*mut FuncDef; 23],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IndexSample {
    p: *mut (),
    n: i32,
    an_eq: *mut tRowcnt,
    an_lt: *mut tRowcnt,
    an_d_lt: *mut tRowcnt,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct KeyClass {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct OnOrUsing {
    p_on: *mut Expr,
    p_using: *mut IdList,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PreUpdate {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PrintfArguments {
    n_arg: i32,
    n_used: i32,
    ap_arg: *mut *mut sqlite3_value,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct RCStr {
    n_rc_ref: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct RowSet {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SQLiteThread {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SelectDest {
    e_dest: u8,
    i_sd_parm: i32,
    i_sd_parm2: i32,
    i_sdst: i32,
    n_sdst: i32,
    z_aff_sdst: *mut i8,
    p_order_by: *mut ExprList,
}
type StrAccum = sqlite3_str;
#[repr(C)]
#[derive(Copy, Clone)]
struct TreeView {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct UnpackedRecord {
    p_key_info: *mut KeyInfo,
    a_mem: *mut Mem,
    u: UnpackedRecord_u0,
    n: i32,
    n_field: u16,
    default_rc: i8,
    err_code: u8,
    r1: i8,
    r2: i8,
    eq_seen: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
union UnpackedRecord_u0 {
    z: *mut i8,
    i: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereInfo {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct BtreePayload {
    p_key: *const (),
    n_key: sqlite3_int64,
    p_data: *const (),
    a_mem: *mut sqlite3_value,
    n_mem: u16,
    n_data: i32,
    n_zero: i32,
}
static mut sqlite3_shared_cache_list: *mut BtShared = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_schema(p: *mut Btree, n_bytes_1: i32,
    x_free_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> *mut () {
    unsafe {
        let p_bt: *mut BtShared = unsafe { (*p).p_bt };
        { let _ = 0; };
        unsafe { sqlite3_btree_enter(p) };
        if (unsafe { (*p_bt).p_schema }).is_null() as i32 != 0 &&
                n_bytes_1 != 0 {
            unsafe {
                (*p_bt).p_schema =
                    unsafe {
                        sqlite3_db_malloc_zero(core::ptr::null_mut(),
                            n_bytes_1 as u64)
                    }
            };
            unsafe { (*p_bt).x_free_schema = x_free_1 };
        }
        unsafe { sqlite3_btree_leave(p) };
        return unsafe { (*p_bt).p_schema };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_set_cache_size(p: *mut Btree, mx_page_1: i32)
    -> i32 {
    let p_bt: *const BtShared = unsafe { (*p).p_bt } as *const BtShared;
    { let _ = 0; };
    unsafe { sqlite3_btree_enter(p) };
    unsafe {
        sqlite3_pager_set_cachesize(unsafe { (*p_bt).p_pager }, mx_page_1)
    };
    unsafe { sqlite3_btree_leave(p) };
    return 0;
}
extern "C" fn cell_size_ptr_table_leaf(p_page_1: *mut MemPage,
    p_cell_1: *mut u8) -> u16 {
    let mut p_iter: *mut u8 = p_cell_1;
    let mut p_end: *mut u8 = core::ptr::null_mut();
    let mut n_size: u32 = 0 as u32;
    n_size = unsafe { *p_iter } as u32;
    if n_size >= 128 as u32 {
        p_end = unsafe { p_iter.offset(8 as isize) };
        n_size &= 127 as u32;
        '__b0: loop {
            '__c0: loop {
                n_size =
                    n_size << 7 |
                        (unsafe {
                                        *{
                                                let __p = &mut p_iter;
                                                *__p = unsafe { (*__p).offset(1) };
                                                *__p
                                            }
                                    } as i32 & 127) as u32;
                break '__c0;
            }
            if !(unsafe { *p_iter } as i32 >= 128 && p_iter < p_end) {
                break '__b0;
            }
        }
    }
    {
        let __p = &mut p_iter;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    if unsafe {
                                                    *{
                                                            let __p = &mut p_iter;
                                                            let __t = *__p;
                                                            *__p = unsafe { (*__p).offset(1) };
                                                            __t
                                                        }
                                                } as i32 & 128 != 0 &&
                                    unsafe {
                                                    *{
                                                            let __p = &mut p_iter;
                                                            let __t = *__p;
                                                            *__p = unsafe { (*__p).offset(1) };
                                                            __t
                                                        }
                                                } as i32 & 128 != 0 &&
                                unsafe {
                                                *{
                                                        let __p = &mut p_iter;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            } as i32 & 128 != 0 &&
                            unsafe {
                                            *{
                                                    let __p = &mut p_iter;
                                                    let __t = *__p;
                                                    *__p = unsafe { (*__p).offset(1) };
                                                    __t
                                                }
                                        } as i32 & 128 != 0 &&
                        unsafe {
                                        *{
                                                let __p = &mut p_iter;
                                                let __t = *__p;
                                                *__p = unsafe { (*__p).offset(1) };
                                                __t
                                            }
                                    } as i32 & 128 != 0 &&
                    unsafe {
                                    *{
                                            let __p = &mut p_iter;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                } as i32 & 128 != 0 &&
                unsafe {
                                *{
                                        let __p = &mut p_iter;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as i32 & 128 != 0 &&
            unsafe {
                            *{
                                    let __p = &mut p_iter;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as i32 & 128 != 0 {
        {
            let __p = &mut p_iter;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if n_size <= unsafe { (*p_page_1).max_local } as u32 {
        n_size += unsafe { p_iter.offset_from(p_cell_1) } as i64 as u32;
        if n_size < 4 as u32 { n_size = 4 as u32; }
    } else {
        let min_local: i32 = unsafe { (*p_page_1).min_local } as i32;
        n_size =
            min_local as u32 +
                (n_size - min_local as u32) %
                    (unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } -
                        4 as u32);
        if n_size > unsafe { (*p_page_1).max_local } as u32 {
            n_size = min_local as u32;
        }
        n_size +=
            (4 + unsafe { p_iter.offset_from(p_cell_1) } as i64 as u16 as i32)
                as u32;
    }
    { let _ = 0; };
    return n_size as u16;
}
extern "C" fn btree_parse_cell_adjust_size_for_overflow(p_page_1: &MemPage,
    p_cell_1: *const u8, p_info_1: &mut CellInfo) -> () {
    let mut min_local: i32 = 0;
    let mut max_local: i32 = 0;
    let mut surplus: i32 = 0;
    min_local = (*p_page_1).min_local as i32;
    max_local = (*p_page_1).max_local as i32;
    surplus =
        (min_local as u32 +
                ((*p_info_1).n_payload - min_local as u32) %
                    (unsafe { (*(*p_page_1).p_bt).usable_size } - 4 as u32)) as
            i32;
    if surplus <= max_local {
        (*p_info_1).n_local = surplus as u16;
    } else { (*p_info_1).n_local = min_local as u16; }
    (*p_info_1).n_size =
        (unsafe {
                                unsafe {
                                    (*p_info_1).p_payload.add((*p_info_1).n_local as
                                                usize).offset_from(p_cell_1)
                                }
                            } as i64 as u16 as i32 + 4) as u16;
}
extern "C" fn btree_parse_cell_ptr(p_page_1: *mut MemPage, p_cell_1: *mut u8,
    p_info_1: *mut CellInfo) -> () {
    let mut p_iter: *mut u8 = core::ptr::null_mut();
    let mut n_payload: u64 = 0 as u64;
    let mut i_key: u64 = 0 as u64;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    p_iter = p_cell_1;
    n_payload = unsafe { *p_iter } as u64;
    if n_payload >= 128 as u64 {
        let p_end: *mut u8 = unsafe { &mut *p_iter.offset(8 as isize) };
        n_payload &= 127 as u64;
        '__b1: loop {
            '__c1: loop {
                n_payload =
                    n_payload << 7 |
                        (unsafe {
                                        *{
                                                let __p = &mut p_iter;
                                                *__p = unsafe { (*__p).offset(1) };
                                                *__p
                                            }
                                    } as i32 & 127) as u64;
                break '__c1;
            }
            if !(unsafe { *p_iter } as i32 >= 128 && p_iter < p_end) {
                break '__b1;
            }
        }
        n_payload &= 4294967295u32 as u64;
    }
    {
        let __p = &mut p_iter;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    i_key = unsafe { *p_iter } as u64;
    if i_key >= 128 as u64 {
        let mut x: u8 = 0 as u8;
        i_key =
            i_key << 7 ^
                {
                        x =
                            unsafe {
                                *{
                                        let __p = &mut p_iter;
                                        *__p = unsafe { (*__p).offset(1) };
                                        *__p
                                    }
                            };
                        x
                    } as u64;
        if x as i32 >= 128 {
            i_key =
                i_key << 7 ^
                    {
                            x =
                                unsafe {
                                    *{
                                            let __p = &mut p_iter;
                                            *__p = unsafe { (*__p).offset(1) };
                                            *__p
                                        }
                                };
                            x
                        } as u64;
            if x as i32 >= 128 {
                i_key =
                    i_key << 7 ^ 270548992 as u64 ^
                        {
                                x =
                                    unsafe {
                                        *{
                                                let __p = &mut p_iter;
                                                *__p = unsafe { (*__p).offset(1) };
                                                *__p
                                            }
                                    };
                                x
                            } as u64;
                if x as i32 >= 128 {
                    i_key =
                        i_key << 7 ^ 16384 as u64 ^
                            {
                                    x =
                                        unsafe {
                                            *{
                                                    let __p = &mut p_iter;
                                                    *__p = unsafe { (*__p).offset(1) };
                                                    *__p
                                                }
                                        };
                                    x
                                } as u64;
                    if x as i32 >= 128 {
                        i_key =
                            i_key << 7 ^ 16384 as u64 ^
                                {
                                        x =
                                            unsafe {
                                                *{
                                                        let __p = &mut p_iter;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        *__p
                                                    }
                                            };
                                        x
                                    } as u64;
                        if x as i32 >= 128 {
                            i_key =
                                i_key << 7 ^ 16384 as u64 ^
                                    {
                                            x =
                                                unsafe {
                                                    *{
                                                            let __p = &mut p_iter;
                                                            *__p = unsafe { (*__p).offset(1) };
                                                            *__p
                                                        }
                                                };
                                            x
                                        } as u64;
                            if x as i32 >= 128 {
                                i_key =
                                    i_key << 7 ^ 16384 as u64 ^
                                        {
                                                x =
                                                    unsafe {
                                                        *{
                                                                let __p = &mut p_iter;
                                                                *__p = unsafe { (*__p).offset(1) };
                                                                *__p
                                                            }
                                                    };
                                                x
                                            } as u64;
                                if x as i32 >= 128 {
                                    i_key =
                                        i_key << 8 ^ 32768 as u64 ^
                                            unsafe {
                                                    *{
                                                            let __p = &mut p_iter;
                                                            *__p = unsafe { (*__p).offset(1) };
                                                            *__p
                                                        }
                                                } as u64;
                                }
                            }
                        }
                    }
                }
            } else { i_key ^= 2113536 as u64; }
        } else { i_key ^= 16384 as u64; }
    }
    {
        let __p = &mut p_iter;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    unsafe { (*p_info_1).n_key = unsafe { *(&raw mut i_key as *mut i64) } };
    unsafe { (*p_info_1).n_payload = n_payload as u32 };
    unsafe { (*p_info_1).p_payload = p_iter };
    { let _ = 0; };
    if n_payload <= unsafe { (*p_page_1).max_local } as u64 {
        unsafe {
            (*p_info_1).n_size =
                (n_payload as u16 as i32 +
                        unsafe { p_iter.offset_from(p_cell_1) } as i64 as u16 as
                            i32) as u16
        };
        if (unsafe { (*p_info_1).n_size } as i32) < 4 {
            unsafe { (*p_info_1).n_size = 4 as u16 };
        }
        unsafe { (*p_info_1).n_local = n_payload as u16 };
    } else {
        btree_parse_cell_adjust_size_for_overflow(unsafe { &*p_page_1 },
            p_cell_1 as *const u8, unsafe { &mut *p_info_1 });
    }
}
extern "C" fn cell_size_ptr_idx_leaf(p_page_1: *mut MemPage,
    p_cell_1: *mut u8) -> u16 {
    let mut p_iter: *mut u8 = p_cell_1;
    let mut p_end: *mut u8 = core::ptr::null_mut();
    let mut n_size: u32 = 0 as u32;
    { let _ = 0; };
    n_size = unsafe { *p_iter } as u32;
    if n_size >= 128 as u32 {
        p_end = unsafe { p_iter.offset(8 as isize) };
        n_size &= 127 as u32;
        '__b2: loop {
            '__c2: loop {
                n_size =
                    n_size << 7 |
                        (unsafe {
                                        *{
                                                let __p = &mut p_iter;
                                                *__p = unsafe { (*__p).offset(1) };
                                                *__p
                                            }
                                    } as i32 & 127) as u32;
                break '__c2;
            }
            if !(unsafe { *p_iter } as i32 >= 128 && p_iter < p_end) {
                break '__b2;
            }
        }
    }
    {
        let __p = &mut p_iter;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    if n_size <= unsafe { (*p_page_1).max_local } as u32 {
        n_size += unsafe { p_iter.offset_from(p_cell_1) } as i64 as u32;
        if n_size < 4 as u32 { n_size = 4 as u32; }
    } else {
        let min_local: i32 = unsafe { (*p_page_1).min_local } as i32;
        n_size =
            min_local as u32 +
                (n_size - min_local as u32) %
                    (unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } -
                        4 as u32);
        if n_size > unsafe { (*p_page_1).max_local } as u32 {
            n_size = min_local as u32;
        }
        n_size +=
            (4 + unsafe { p_iter.offset_from(p_cell_1) } as i64 as u16 as i32)
                as u32;
    }
    { let _ = 0; };
    return n_size as u16;
}
extern "C" fn btree_parse_cell_ptr_index(p_page_1: *mut MemPage,
    p_cell_1: *mut u8, p_info_1: *mut CellInfo) -> () {
    let mut p_iter: *mut u8 = core::ptr::null_mut();
    let mut n_payload: u32 = 0 as u32;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    p_iter =
        unsafe {
            p_cell_1.add(unsafe { (*p_page_1).child_ptr_size } as usize)
        };
    n_payload = unsafe { *p_iter } as u32;
    if n_payload >= 128 as u32 {
        let p_end: *mut u8 = unsafe { &mut *p_iter.offset(8 as isize) };
        n_payload &= 127 as u32;
        '__b3: loop {
            '__c3: loop {
                n_payload =
                    n_payload << 7 |
                        (unsafe {
                                        *{
                                                let __p = &mut p_iter;
                                                *__p = unsafe { (*__p).offset(1) };
                                                *__p
                                            }
                                    } as i32 & 127) as u32;
                break '__c3;
            }
            if !(unsafe { *p_iter } as i32 >= 128 && p_iter < p_end) {
                break '__b3;
            }
        }
    }
    {
        let __p = &mut p_iter;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    unsafe { (*p_info_1).n_key = n_payload as i64 };
    unsafe { (*p_info_1).n_payload = n_payload };
    unsafe { (*p_info_1).p_payload = p_iter };
    { let _ = 0; };
    { let _ = 0; };
    if n_payload <= unsafe { (*p_page_1).max_local } as u32 {
        unsafe {
            (*p_info_1).n_size =
                (n_payload as u16 as i32 +
                        unsafe { p_iter.offset_from(p_cell_1) } as i64 as u16 as
                            i32) as u16
        };
        if (unsafe { (*p_info_1).n_size } as i32) < 4 {
            unsafe { (*p_info_1).n_size = 4 as u16 };
        }
        unsafe { (*p_info_1).n_local = n_payload as u16 };
    } else {
        btree_parse_cell_adjust_size_for_overflow(unsafe { &*p_page_1 },
            p_cell_1 as *const u8, unsafe { &mut *p_info_1 });
    }
}
extern "C" fn cell_size_ptr(p_page_1: *mut MemPage, p_cell_1: *mut u8)
    -> u16 {
    let mut p_iter: *mut u8 = unsafe { p_cell_1.offset(4 as isize) };
    let mut p_end: *mut u8 = core::ptr::null_mut();
    let mut n_size: u32 = 0 as u32;
    { let _ = 0; };
    n_size = unsafe { *p_iter } as u32;
    if n_size >= 128 as u32 {
        p_end = unsafe { p_iter.offset(8 as isize) };
        n_size &= 127 as u32;
        '__b4: loop {
            '__c4: loop {
                n_size =
                    n_size << 7 |
                        (unsafe {
                                        *{
                                                let __p = &mut p_iter;
                                                *__p = unsafe { (*__p).offset(1) };
                                                *__p
                                            }
                                    } as i32 & 127) as u32;
                break '__c4;
            }
            if !(unsafe { *p_iter } as i32 >= 128 && p_iter < p_end) {
                break '__b4;
            }
        }
    }
    {
        let __p = &mut p_iter;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    if n_size <= unsafe { (*p_page_1).max_local } as u32 {
        n_size += unsafe { p_iter.offset_from(p_cell_1) } as i64 as u32;
        { let _ = 0; };
    } else {
        let min_local: i32 = unsafe { (*p_page_1).min_local } as i32;
        n_size =
            min_local as u32 +
                (n_size - min_local as u32) %
                    (unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } -
                        4 as u32);
        if n_size > unsafe { (*p_page_1).max_local } as u32 {
            n_size = min_local as u32;
        }
        n_size +=
            (4 + unsafe { p_iter.offset_from(p_cell_1) } as i64 as u16 as i32)
                as u32;
    }
    { let _ = 0; };
    return n_size as u16;
}
extern "C" fn cell_size_ptr_no_payload(p_page_1: *mut MemPage,
    p_cell_1: *mut u8) -> u16 {
    let mut p_iter: *mut u8 = unsafe { p_cell_1.offset(4 as isize) };
    let mut p_end: *mut u8 = core::ptr::null_mut();
    { let _ = p_page_1; };
    { let _ = 0; };
    p_end = unsafe { p_iter.offset(9 as isize) };
    while unsafe {
                            *{
                                    let __p = &mut p_iter;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                        } as i32 & 128 != 0 && p_iter < p_end {}
    { let _ = 0; };
    return unsafe { p_iter.offset_from(p_cell_1) } as i64 as u16;
}
extern "C" fn btree_parse_cell_ptr_no_payload(p_page_1: *mut MemPage,
    p_cell_1: *mut u8, p_info_1: *mut CellInfo) -> () {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = p_page_1; };
    unsafe {
        (*p_info_1).n_size =
            (4 +
                    unsafe {
                            sqlite3_get_varint(unsafe {
                                        &raw mut *p_cell_1.offset(4 as isize)
                                    } as *const u8,
                                unsafe { &raw mut (*p_info_1).n_key } as *mut u64)
                        } as i32) as u16
    };
    unsafe { (*p_info_1).n_payload = 0 as u32 };
    unsafe { (*p_info_1).n_local = 0 as u16 };
    unsafe { (*p_info_1).p_payload = core::ptr::null_mut() };
    return;
}
extern "C" fn decode_flags(p_page_1: &mut MemPage, flag_byte_1: i32) -> i32 {
    let mut p_bt: *const BtShared = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    p_bt = (*p_page_1).p_bt;
    (*p_page_1).max1byte_payload = unsafe { (*p_bt).max1byte_payload };
    if flag_byte_1 >= 2 | 8 {
        (*p_page_1).child_ptr_size = 0 as u8;
        (*p_page_1).leaf = 1 as u8;
        if flag_byte_1 == 4 | 1 | 8 {
            (*p_page_1).int_key_leaf = 1 as u8;
            (*p_page_1).x_cell_size = Some(cell_size_ptr_table_leaf);
            (*p_page_1).x_parse_cell = Some(btree_parse_cell_ptr);
            (*p_page_1).int_key = 1 as u8;
            (*p_page_1).max_local = unsafe { (*p_bt).max_leaf };
            (*p_page_1).min_local = unsafe { (*p_bt).min_leaf };
        } else if flag_byte_1 == 2 | 8 {
            (*p_page_1).int_key = 0 as u8;
            (*p_page_1).int_key_leaf = 0 as u8;
            (*p_page_1).x_cell_size = Some(cell_size_ptr_idx_leaf);
            (*p_page_1).x_parse_cell = Some(btree_parse_cell_ptr_index);
            (*p_page_1).max_local = unsafe { (*p_bt).max_local };
            (*p_page_1).min_local = unsafe { (*p_bt).min_local };
        } else {
            (*p_page_1).int_key = 0 as u8;
            (*p_page_1).int_key_leaf = 0 as u8;
            (*p_page_1).x_cell_size = Some(cell_size_ptr_idx_leaf);
            (*p_page_1).x_parse_cell = Some(btree_parse_cell_ptr_index);
            return unsafe { sqlite3_corrupt_error(2084) };
        }
    } else {
        (*p_page_1).child_ptr_size = 4 as u8;
        (*p_page_1).leaf = 0 as u8;
        if flag_byte_1 == 2 {
            (*p_page_1).int_key = 0 as u8;
            (*p_page_1).int_key_leaf = 0 as u8;
            (*p_page_1).x_cell_size = Some(cell_size_ptr);
            (*p_page_1).x_parse_cell = Some(btree_parse_cell_ptr_index);
            (*p_page_1).max_local = unsafe { (*p_bt).max_local };
            (*p_page_1).min_local = unsafe { (*p_bt).min_local };
        } else if flag_byte_1 == 4 | 1 {
            (*p_page_1).int_key_leaf = 0 as u8;
            (*p_page_1).x_cell_size = Some(cell_size_ptr_no_payload);
            (*p_page_1).x_parse_cell = Some(btree_parse_cell_ptr_no_payload);
            (*p_page_1).int_key = 1 as u8;
            (*p_page_1).max_local = unsafe { (*p_bt).max_leaf };
            (*p_page_1).min_local = unsafe { (*p_bt).min_leaf };
        } else {
            (*p_page_1).int_key = 0 as u8;
            (*p_page_1).int_key_leaf = 0 as u8;
            (*p_page_1).x_cell_size = Some(cell_size_ptr);
            (*p_page_1).x_parse_cell = Some(btree_parse_cell_ptr_index);
            return unsafe { sqlite3_corrupt_error(2108) };
        }
    }
    return 0;
}
extern "C" fn btree_cell_size_check(p_page_1: *mut MemPage) -> i32 {
    let mut i_cell_first: i32 = 0;
    let mut i_cell_last: i32 = 0;
    let mut i: i32 = 0;
    let mut sz: i32 = 0;
    let mut pc: i32 = 0;
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut usable_size: i32 = 0;
    let mut cell_offset: i32 = 0;
    i_cell_first =
        unsafe { (*p_page_1).cell_offset } as i32 +
            2 * unsafe { (*p_page_1).n_cell } as i32;
    usable_size =
        unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } as i32;
    i_cell_last = usable_size - 4;
    data = unsafe { (*p_page_1).a_data };
    cell_offset = unsafe { (*p_page_1).cell_offset } as i32;
    if (unsafe { (*p_page_1).leaf } == 0) as i32 != 0 {
        { let __p = &mut i_cell_last; let __t = *__p; *__p -= 1; __t };
    }
    {
        i = 0;
        '__b6: loop {
            if !(i < unsafe { (*p_page_1).n_cell } as i32) { break '__b6; }
            '__c6: loop {
                pc =
                    (unsafe {
                                        *unsafe {
                                                data.offset((cell_offset + i * 2) as
                                                            isize).offset(0 as isize)
                                            }
                                    } as i32) << 8 |
                        unsafe {
                                *unsafe {
                                        data.offset((cell_offset + i * 2) as
                                                    isize).offset(1 as isize)
                                    }
                            } as i32;
                if pc < i_cell_first || pc > i_cell_last {
                    return unsafe { sqlite3_corrupt_error(2228) };
                }
                sz =
                    unsafe {
                            (unsafe {
                                    (*p_page_1).x_cell_size.unwrap()
                                })(p_page_1, unsafe { &mut *data.offset(pc as isize) })
                        } as i32;
                if pc + sz > usable_size {
                    return unsafe { sqlite3_corrupt_error(2233) };
                }
                break '__c6;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}
extern "C" fn btree_init_page(p_page_1: *mut MemPage) -> i32 {
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut p_bt: *const BtShared = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    p_bt = unsafe { (*p_page_1).p_bt };
    data =
        unsafe {
            unsafe {
                (*p_page_1).a_data.add(unsafe { (*p_page_1).hdr_offset } as
                        usize)
            }
        };
    if decode_flags(unsafe { &mut *p_page_1 },
                unsafe { *data.offset(0 as isize) } as i32) != 0 {
        return unsafe { sqlite3_corrupt_error(2265) };
    }
    { let _ = 0; };
    unsafe {
        (*p_page_1).mask_page =
            (unsafe { (*p_bt).page_size } - 1 as u32) as u16
    };
    unsafe { (*p_page_1).n_overflow = 0 as u8 };
    unsafe {
        (*p_page_1).cell_offset =
            (unsafe { (*p_page_1).hdr_offset } as i32 + 8 +
                    unsafe { (*p_page_1).child_ptr_size } as i32) as u16
    };
    unsafe {
        (*p_page_1).a_cell_idx =
            unsafe {
                unsafe {
                    data.add(unsafe { (*p_page_1).child_ptr_size } as
                                usize).offset(8 as isize)
                }
            }
    };
    unsafe {
        (*p_page_1).a_data_end =
            unsafe {
                unsafe {
                    (*p_page_1).a_data.add(unsafe { (*p_bt).page_size } as
                            usize)
                }
            }
    };
    unsafe {
        (*p_page_1).a_data_ofst =
            unsafe {
                unsafe {
                    (*p_page_1).a_data.add(unsafe { (*p_page_1).child_ptr_size }
                            as usize)
                }
            }
    };
    unsafe {
        (*p_page_1).n_cell =
            ((unsafe {
                                    *unsafe { data.offset(3 as isize).offset(0 as isize) }
                                } as i32) << 8 |
                    unsafe {
                            *unsafe { data.offset(3 as isize).offset(1 as isize) }
                        } as i32) as u16
    };
    if unsafe { (*p_page_1).n_cell } as u32 >
            (unsafe { (*p_bt).page_size } - 8 as u32) / 6 as u32 {
        return unsafe { sqlite3_corrupt_error(2279) };
    }
    { let _ = 0; };
    unsafe { (*p_page_1).n_free = -1 };
    unsafe { (*p_page_1).is_init = 1 as u8 };
    if unsafe { (*unsafe { (*p_bt).db }).flags } & 2097152 as u64 != 0 {
        return btree_cell_size_check(p_page_1);
    }
    return 0;
}
extern "C" fn page_reinit(p_data_1: *mut DbPage) -> () {
    let mut p_page: *mut MemPage = core::ptr::null_mut();
    p_page = unsafe { sqlite3_pager_get_extra(p_data_1) } as *mut MemPage;
    { let _ = 0; };
    if unsafe { (*p_page).is_init } != 0 {
        { let _ = 0; };
        unsafe { (*p_page).is_init = 0 as u8 };
        if unsafe { sqlite3_pager_page_refcount(p_data_1) } > 1 {
            btree_init_page(p_page);
        }
    }
}
extern "C" fn btree_invoke_busy_handler(p_arg_1: *mut ()) -> i32 {
    let p_bt: *const BtShared = p_arg_1 as *mut BtShared as *const BtShared;
    { let _ = 0; };
    { let _ = 0; };
    return unsafe {
            sqlite3_invoke_busy_handler(unsafe {
                    &mut (*unsafe { (*p_bt).db }).busy_handler
                })
        };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_open(p_vfs_1: *mut sqlite3_vfs,
    z_filename_1: *const i8, db: *mut sqlite3, pp_btree_1: &mut *mut Btree,
    mut flags: i32, mut vfs_flags_1: i32) -> i32 {
    unsafe {
        let mut p_bt: *mut BtShared = core::ptr::null_mut();
        let mut p: *mut Btree = core::ptr::null_mut();
        let mut mutex_open: *mut sqlite3_mutex = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut n_reserve: u8 = 0 as u8;
        let mut z_db_header: [u8; 100] = [0; 100];
        let mut is_temp_db: i32 = 0;
        let mut is_memdb: i32 = 0;
        let mut n_filename: i32 = 0;
        let mut n_full_pathname: i32 = 0;
        let mut z_full_pathname: *mut i8 = core::ptr::null_mut();
        let mut mutex_shared: *mut sqlite3_mutex = core::ptr::null_mut();
        let mut i_db: i32 = 0;
        let mut p_existing: *const Btree = core::ptr::null();
        let mut mutex_shared_1: *mut sqlite3_mutex = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut p_sib: *mut Btree = core::ptr::null_mut();
        let mut p_file: *mut sqlite3_file = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s8:
                {
                match __state {
                    0 => { p_bt = core::ptr::null_mut(); __state = 3; }
                    2 => {
                        if rc != 0 { __state = 149; } else { __state = 150; }
                    }
                    3 => { __state = 4; }
                    4 => { mutex_open = core::ptr::null_mut(); __state = 5; }
                    5 => { rc = 0; __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => {
                        is_temp_db =
                            (z_filename_1 == core::ptr::null() ||
                                    unsafe { *z_filename_1.offset(0 as isize) } as i32 == 0) as
                                i32;
                        __state = 9;
                    }
                    9 => {
                        is_memdb =
                            (!(z_filename_1).is_null() &&
                                            unsafe {
                                                    strcmp(z_filename_1,
                                                        c":memory:".as_ptr() as *mut i8 as *const i8)
                                                } == 0 ||
                                        is_temp_db != 0 &&
                                            unsafe { sqlite3_temp_in_memory(db as *const sqlite3) } != 0
                                    || vfs_flags_1 & 128 != 0) as i32;
                        __state = 10;
                    }
                    10 => { { let _ = 0; }; __state = 11; }
                    11 => { { let _ = 0; }; __state = 12; }
                    12 => { { let _ = 0; }; __state = 13; }
                    13 => { { let _ = 0; }; __state = 14; }
                    14 => { { let _ = 0; }; __state = 15; }
                    15 => { { let _ = 0; }; __state = 16; }
                    16 => {
                        if is_memdb != 0 { __state = 18; } else { __state = 17; }
                    }
                    17 => {
                        if vfs_flags_1 & 256 != 0 &&
                                (is_memdb != 0 || is_temp_db != 0) {
                            __state = 20;
                        } else { __state = 19; }
                    }
                    18 => { flags |= 2; __state = 17; }
                    19 => {
                        p =
                            unsafe {
                                    sqlite3_malloc_zero(core::mem::size_of::<Btree>() as u64)
                                } as *mut Btree;
                        __state = 21;
                    }
                    20 => {
                        vfs_flags_1 = vfs_flags_1 & !256 | 512;
                        __state = 19;
                    }
                    21 => {
                        if (p).is_null() as i32 != 0 {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    22 => { unsafe { (*p).in_trans = 0 as u8 }; __state = 24; }
                    23 => { return 7; }
                    24 => { unsafe { (*p).db = db }; __state = 25; }
                    25 => { unsafe { (*p).lock.p_btree = p }; __state = 26; }
                    26 => {
                        unsafe { (*p).lock.i_table = 1 as Pgno };
                        __state = 27;
                    }
                    27 => {
                        if is_temp_db as i32 == 0 &&
                                (is_memdb as i32 == 0 || vfs_flags_1 & 64 != 0) {
                            __state = 29;
                        } else { __state = 28; }
                    }
                    28 => {
                        if p_bt == core::ptr::null_mut() {
                            __state = 73;
                        } else { __state = 72; }
                    }
                    29 => {
                        if vfs_flags_1 & 131072 != 0 {
                            __state = 30;
                        } else { __state = 28; }
                    }
                    30 => {
                        n_filename = unsafe { sqlite3_strlen30(z_filename_1) } + 1;
                        __state = 31;
                    }
                    31 => {
                        n_full_pathname = unsafe { (*p_vfs_1).mx_pathname } + 1;
                        __state = 32;
                    }
                    32 => {
                        z_full_pathname =
                            unsafe {
                                    sqlite3Malloc(if n_full_pathname > n_filename {
                                                n_full_pathname
                                            } else { n_filename } as u64)
                                } as *mut i8;
                        __state = 33;
                    }
                    33 => { __state = 34; }
                    34 => { unsafe { (*p).sharable = 1 as u8 }; __state = 35; }
                    35 => {
                        if (z_full_pathname).is_null() as i32 != 0 {
                            __state = 37;
                        } else { __state = 36; }
                    }
                    36 => {
                        if is_memdb != 0 { __state = 40; } else { __state = 41; }
                    }
                    37 => {
                        unsafe { sqlite3_free(p as *mut ()) };
                        __state = 38;
                    }
                    38 => { return 7; }
                    39 => {
                        mutex_open = unsafe { sqlite3MutexAlloc(4) };
                        __state = 48;
                    }
                    40 => {
                        unsafe {
                            memcpy(z_full_pathname as *mut (),
                                z_filename_1 as *const (), n_filename as u64)
                        };
                        __state = 39;
                    }
                    41 => {
                        rc =
                            unsafe {
                                sqlite3_os_full_pathname(p_vfs_1, z_filename_1,
                                    n_full_pathname, z_full_pathname)
                            };
                        __state = 42;
                    }
                    42 => {
                        if rc != 0 { __state = 43; } else { __state = 39; }
                    }
                    43 => {
                        if rc == 0 | 2 << 8 { __state = 44; } else { __state = 45; }
                    }
                    44 => { rc = 0; __state = 39; }
                    45 => {
                        unsafe { sqlite3_free(z_full_pathname as *mut ()) };
                        __state = 46;
                    }
                    46 => {
                        unsafe { sqlite3_free(p as *mut ()) };
                        __state = 47;
                    }
                    47 => { return rc; }
                    48 => {
                        unsafe { sqlite3_mutex_enter(mutex_open) };
                        __state = 49;
                    }
                    49 => {
                        mutex_shared = unsafe { sqlite3MutexAlloc(2) };
                        __state = 50;
                    }
                    50 => {
                        unsafe { sqlite3_mutex_enter(mutex_shared) };
                        __state = 51;
                    }
                    51 => { p_bt = sqlite3_shared_cache_list; __state = 53; }
                    52 => {
                        unsafe { sqlite3_mutex_leave(mutex_shared) };
                        __state = 71;
                    }
                    53 => {
                        if !(p_bt).is_null() {
                            __state = 54;
                        } else { __state = 52; }
                    }
                    54 => { { let _ = 0; }; __state = 56; }
                    55 => { p_bt = unsafe { (*p_bt).p_next }; __state = 53; }
                    56 => {
                        if 0 ==
                                    unsafe {
                                        strcmp(z_full_pathname as *const i8,
                                            unsafe {
                                                sqlite3_pager_filename(unsafe { (*p_bt).p_pager } as
                                                        *const Pager, 0)
                                            })
                                    } &&
                                unsafe { sqlite3_pager_vfs(unsafe { (*p_bt).p_pager }) } ==
                                    p_vfs_1 {
                            __state = 57;
                        } else { __state = 55; }
                    }
                    57 => { __state = 58; }
                    58 => { i_db = unsafe { (*db).n_db } - 1; __state = 60; }
                    59 => { unsafe { (*p).p_bt = p_bt }; __state = 69; }
                    60 => {
                        if i_db >= 0 { __state = 61; } else { __state = 59; }
                    }
                    61 => {
                        p_existing =
                            unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).p_bt
                                } as *const Btree;
                        __state = 63;
                    }
                    62 => {
                        { let __p = &mut i_db; let __t = *__p; *__p -= 1; __t };
                        __state = 60;
                    }
                    63 => {
                        if !(p_existing).is_null() &&
                                unsafe { (*p_existing).p_bt } == p_bt {
                            __state = 64;
                        } else { __state = 62; }
                    }
                    64 => {
                        unsafe { sqlite3_mutex_leave(mutex_shared) };
                        __state = 65;
                    }
                    65 => {
                        unsafe { sqlite3_mutex_leave(mutex_open) };
                        __state = 66;
                    }
                    66 => {
                        unsafe { sqlite3_free(z_full_pathname as *mut ()) };
                        __state = 67;
                    }
                    67 => {
                        unsafe { sqlite3_free(p as *mut ()) };
                        __state = 68;
                    }
                    68 => { return 19; }
                    69 => {
                        {
                            let __p = unsafe { &mut (*p_bt).n_ref };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 70;
                    }
                    70 => { __state = 52; }
                    71 => {
                        unsafe { sqlite3_free(z_full_pathname as *mut ()) };
                        __state = 28;
                    }
                    72 => {
                        if unsafe { (*p).sharable } != 0 {
                            __state = 127;
                        } else { __state = 126; }
                    }
                    73 => { { let _ = 0; }; __state = 74; }
                    74 => { { let _ = 0; }; __state = 75; }
                    75 => { { let _ = 0; }; __state = 76; }
                    76 => { { let _ = 0; }; __state = 77; }
                    77 => { { let _ = 0; }; __state = 78; }
                    78 => {
                        unsafe {
                            memset(&raw mut z_db_header[16 as usize] as *mut (), 0,
                                8 as u64)
                        };
                        __state = 79;
                    }
                    79 => {
                        p_bt =
                            unsafe {
                                    sqlite3_malloc_zero(core::mem::size_of::<BtShared>() as u64)
                                } as *mut BtShared;
                        __state = 80;
                    }
                    80 => {
                        if p_bt == core::ptr::null_mut() {
                            __state = 82;
                        } else { __state = 81; }
                    }
                    81 => {
                        rc =
                            unsafe {
                                sqlite3_pager_open(p_vfs_1, unsafe { &mut (*p_bt).p_pager },
                                    z_filename_1, core::mem::size_of::<MemPage>() as i32, flags,
                                    vfs_flags_1, Some(page_reinit))
                            };
                        __state = 84;
                    }
                    82 => { rc = 7; __state = 83; }
                    83 => { __state = 2; }
                    84 => {
                        if rc == 0 { __state = 86; } else { __state = 85; }
                    }
                    85 => {
                        if rc != 0 { __state = 89; } else { __state = 88; }
                    }
                    86 => {
                        unsafe {
                            sqlite3_pager_set_mmap_limit(unsafe { (*p_bt).p_pager },
                                unsafe { (*db).sz_mmap })
                        };
                        __state = 87;
                    }
                    87 => {
                        rc =
                            unsafe {
                                sqlite3_pager_read_fileheader(unsafe { (*p_bt).p_pager },
                                    core::mem::size_of::<[u8; 100]>() as i32,
                                    &raw mut z_db_header[0 as usize] as *mut u8)
                            };
                        __state = 85;
                    }
                    88 => {
                        unsafe { (*p_bt).open_flags = flags as u8 };
                        __state = 90;
                    }
                    89 => { __state = 2; }
                    90 => { unsafe { (*p_bt).db = db }; __state = 91; }
                    91 => {
                        unsafe {
                            sqlite3_pager_set_busy_handler(unsafe { (*p_bt).p_pager },
                                Some(btree_invoke_busy_handler), p_bt as *mut ())
                        };
                        __state = 92;
                    }
                    92 => { unsafe { (*p).p_bt = p_bt }; __state = 93; }
                    93 => {
                        unsafe { (*p_bt).p_cursor = core::ptr::null_mut() };
                        __state = 94;
                    }
                    94 => {
                        unsafe { (*p_bt).p_page1 = core::ptr::null_mut() };
                        __state = 95;
                    }
                    95 => {
                        if unsafe {
                                    sqlite3_pager_isreadonly(unsafe { (*p_bt).p_pager })
                                } != 0 {
                            __state = 97;
                        } else { __state = 96; }
                    }
                    96 => {
                        unsafe {
                            (*p_bt).page_size =
                                ((z_db_header[16 as usize] as i32) << 8 |
                                        (z_db_header[17 as usize] as i32) << 16) as u32
                        };
                        __state = 98;
                    }
                    97 => {
                        unsafe { (*p_bt).bts_flags |= 1 as u16 };
                        __state = 96;
                    }
                    98 => {
                        if unsafe { (*p_bt).page_size } < 512 as u32 ||
                                    unsafe { (*p_bt).page_size } > 65536 as u32 ||
                                unsafe { (*p_bt).page_size } - 1 as u32 &
                                        unsafe { (*p_bt).page_size } != 0 as u32 {
                            __state = 100;
                        } else { __state = 101; }
                    }
                    99 => {
                        rc =
                            unsafe {
                                sqlite3_pager_set_pagesize(unsafe { (*p_bt).p_pager },
                                    unsafe { &mut (*p_bt).page_size }, n_reserve as i32)
                            };
                        __state = 109;
                    }
                    100 => {
                        unsafe { (*p_bt).page_size = 0 as u32 };
                        __state = 102;
                    }
                    101 => {
                        n_reserve = z_db_header[20 as usize];
                        __state = 106;
                    }
                    102 => {
                        if !(z_filename_1).is_null() && (is_memdb == 0) as i32 != 0
                            {
                            __state = 104;
                        } else { __state = 103; }
                    }
                    103 => { n_reserve = 0 as u8; __state = 99; }
                    104 => {
                        unsafe {
                            (*p_bt).auto_vacuum = if 0 != 0 { 1 } else { 0 } as u8
                        };
                        __state = 105;
                    }
                    105 => {
                        unsafe {
                            (*p_bt).incr_vacuum = if 0 == 2 { 1 } else { 0 } as u8
                        };
                        __state = 103;
                    }
                    106 => {
                        unsafe { (*p_bt).bts_flags |= 2 as u16 };
                        __state = 107;
                    }
                    107 => {
                        unsafe {
                            (*p_bt).auto_vacuum =
                                if unsafe {
                                                sqlite3_get4byte(&raw mut z_db_header[(36 + 4 * 4) as usize]
                                                        as *const u8)
                                            } != 0 {
                                        1
                                    } else { 0 } as u8
                        };
                        __state = 108;
                    }
                    108 => {
                        unsafe {
                            (*p_bt).incr_vacuum =
                                if unsafe {
                                                sqlite3_get4byte(&raw mut z_db_header[(36 + 7 * 4) as usize]
                                                        as *const u8)
                                            } != 0 {
                                        1
                                    } else { 0 } as u8
                        };
                        __state = 99;
                    }
                    109 => {
                        if rc != 0 { __state = 111; } else { __state = 110; }
                    }
                    110 => {
                        unsafe {
                            (*p_bt).usable_size =
                                unsafe { (*p_bt).page_size } - n_reserve as u32
                        };
                        __state = 112;
                    }
                    111 => { __state = 2; }
                    112 => { { let _ = 0; }; __state = 113; }
                    113 => { unsafe { (*p_bt).n_ref = 1 }; __state = 114; }
                    114 => {
                        if unsafe { (*p).sharable } != 0 {
                            __state = 115;
                        } else { __state = 72; }
                    }
                    115 => { __state = 116; }
                    116 => {
                        mutex_shared_1 = unsafe { sqlite3MutexAlloc(2) };
                        __state = 117;
                    }
                    117 => {
                        if 1 != 0 && sqlite3Config.b_core_mutex != 0 {
                            __state = 119;
                        } else { __state = 118; }
                    }
                    118 => {
                        unsafe { sqlite3_mutex_enter(mutex_shared_1) };
                        __state = 123;
                    }
                    119 => {
                        unsafe { (*p_bt).mutex = unsafe { sqlite3MutexAlloc(0) } };
                        __state = 120;
                    }
                    120 => {
                        if unsafe { (*p_bt).mutex } == core::ptr::null_mut() {
                            __state = 121;
                        } else { __state = 118; }
                    }
                    121 => { rc = 7; __state = 122; }
                    122 => { __state = 2; }
                    123 => {
                        unsafe { (*p_bt).p_next = sqlite3_shared_cache_list };
                        __state = 124;
                    }
                    124 => { sqlite3_shared_cache_list = p_bt; __state = 125; }
                    125 => {
                        unsafe { sqlite3_mutex_leave(mutex_shared_1) };
                        __state = 72;
                    }
                    126 => { *pp_btree_1 = p; __state = 147; }
                    127 => { __state = 128; }
                    128 => { __state = 129; }
                    129 => { i = 0; __state = 130; }
                    130 => {
                        if i < unsafe { (*db).n_db } {
                            __state = 131;
                        } else { __state = 126; }
                    }
                    131 => {
                        if {
                                        p_sib =
                                            unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                                        p_sib
                                    } != core::ptr::null_mut() &&
                                unsafe { (*p_sib).sharable } != 0 {
                            __state = 133;
                        } else { __state = 132; }
                    }
                    132 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 130;
                    }
                    133 => {
                        if !(unsafe { (*p_sib).p_prev }).is_null() {
                            __state = 135;
                        } else { __state = 134; }
                    }
                    134 => {
                        if (unsafe { (*p).p_bt } as uptr) <
                                unsafe { (*p_sib).p_bt } as uptr {
                            __state = 137;
                        } else { __state = 138; }
                    }
                    135 => {
                        p_sib = unsafe { (*p_sib).p_prev };
                        __state = 133;
                    }
                    136 => { __state = 126; }
                    137 => { unsafe { (*p).p_next = p_sib }; __state = 139; }
                    138 => {
                        if !(unsafe { (*p_sib).p_next }).is_null() &&
                                (unsafe { (*unsafe { (*p_sib).p_next }).p_bt } as uptr) <
                                    unsafe { (*p).p_bt } as uptr {
                            __state = 142;
                        } else { __state = 141; }
                    }
                    139 => {
                        unsafe { (*p).p_prev = core::ptr::null_mut() };
                        __state = 140;
                    }
                    140 => { unsafe { (*p_sib).p_prev = p }; __state = 136; }
                    141 => {
                        unsafe { (*p).p_next = unsafe { (*p_sib).p_next } };
                        __state = 143;
                    }
                    142 => {
                        p_sib = unsafe { (*p_sib).p_next };
                        __state = 138;
                    }
                    143 => { unsafe { (*p).p_prev = p_sib }; __state = 144; }
                    144 => {
                        if !(unsafe { (*p).p_next }).is_null() {
                            __state = 146;
                        } else { __state = 145; }
                    }
                    145 => { unsafe { (*p_sib).p_next = p }; __state = 136; }
                    146 => {
                        unsafe { (*unsafe { (*p).p_next }).p_prev = p };
                        __state = 145;
                    }
                    147 => { __state = 2; }
                    148 => {
                        if !(mutex_open).is_null() {
                            __state = 161;
                        } else { __state = 160; }
                    }
                    149 => {
                        if !(p_bt).is_null() &&
                                !(unsafe { (*p_bt).p_pager }).is_null() {
                            __state = 152;
                        } else { __state = 151; }
                    }
                    150 => { __state = 155; }
                    151 => {
                        unsafe { sqlite3_free(p_bt as *mut ()) };
                        __state = 153;
                    }
                    152 => {
                        unsafe {
                            sqlite3_pager_close(unsafe { (*p_bt).p_pager },
                                core::ptr::null_mut())
                        };
                        __state = 151;
                    }
                    153 => {
                        unsafe { sqlite3_free(p as *mut ()) };
                        __state = 154;
                    }
                    154 => {
                        *pp_btree_1 = core::ptr::null_mut();
                        __state = 148;
                    }
                    155 => {
                        if unsafe { sqlite3_btree_schema(p, 0, None) } ==
                                core::ptr::null_mut() {
                            __state = 157;
                        } else { __state = 156; }
                    }
                    156 => {
                        p_file =
                            unsafe { sqlite3_pager_file(unsafe { (*p_bt).p_pager }) };
                        __state = 158;
                    }
                    157 => {
                        unsafe { sqlite3_btree_set_cache_size(p, -2000) };
                        __state = 156;
                    }
                    158 => {
                        if !(unsafe { (*p_file).p_methods }).is_null() {
                            __state = 159;
                        } else { __state = 148; }
                    }
                    159 => {
                        unsafe {
                            sqlite3_os_file_control_hint(p_file, 30,
                                unsafe { &raw mut (*p_bt).db } as *mut ())
                        };
                        __state = 148;
                    }
                    160 => { { let _ = 0; }; __state = 163; }
                    161 => { { let _ = 0; }; __state = 162; }
                    162 => {
                        unsafe { sqlite3_mutex_leave(mutex_open) };
                        __state = 160;
                    }
                    163 => { return rc; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn btree_parse_cell(p_page_1: *mut MemPage, i_cell_1: i32,
    p_info_1: *mut CellInfo) -> () {
    unsafe {
        (unsafe {
                (*p_page_1).x_parse_cell.unwrap()
            })(p_page_1,
            unsafe {
                unsafe {
                    (*p_page_1).a_data.offset((unsafe { (*p_page_1).mask_page }
                                    as i32 &
                                ((unsafe {
                                                    *unsafe {
                                                            unsafe {
                                                                (*p_page_1).a_cell_idx.offset((2 * i_cell_1) as
                                                                            isize).offset(0 as isize)
                                                            }
                                                        }
                                                } as i32) << 8 |
                                    unsafe {
                                            *unsafe {
                                                    unsafe {
                                                        (*p_page_1).a_cell_idx.offset((2 * i_cell_1) as
                                                                    isize).offset(1 as isize)
                                                    }
                                                }
                                        } as i32)) as isize)
                }
            }, p_info_1)
    };
}
extern "C" fn get_cell_info(p_cur_1: &mut BtCursor) -> () {
    if (*p_cur_1).info.n_size as i32 == 0 {
        (*p_cur_1).cur_flags |= 2 as u8;
        btree_parse_cell((*p_cur_1).p_page, (*p_cur_1).ix as i32,
            &mut (*p_cur_1).info);
    } else {}
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_integer_key(p_cur_1: *mut BtCursor) -> i64 {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    get_cell_info(unsafe { &mut *p_cur_1 });
    return unsafe { (*p_cur_1).info.n_key };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_payload_size(p_cur_1: *mut BtCursor) -> u32 {
    { let _ = 0; };
    { let _ = 0; };
    get_cell_info(unsafe { &mut *p_cur_1 });
    return unsafe { (*p_cur_1).info.n_payload };
}
extern "C" fn copy_payload(p_payload_1: *mut (), p_buf_1: &mut [u8],
    e_op_1: i32, p_db_page_1: *mut DbPage) -> i32 {
    if e_op_1 != 0 {
        let rc: i32 = unsafe { sqlite3_pager_write(p_db_page_1) };
        if rc != 0 { return rc; }
        unsafe {
            memcpy(p_payload_1, p_buf_1.as_ptr() as *const (),
                p_buf_1.len() as i32 as u64)
        };
    } else {
        unsafe {
            memcpy(p_buf_1.as_ptr() as *mut (), p_payload_1 as *const (),
                p_buf_1.len() as i32 as u64)
        };
    }
    return 0;
}
extern "C" fn ptrmap_pageno(p_bt_1: &BtShared, pgno: Pgno) -> Pgno {
    unsafe {
        let mut n_pages_per_map_page: i32 = 0;
        let mut i_ptr_map: Pgno = 0 as Pgno;
        let mut ret: Pgno = 0 as Pgno;
        { let _ = 0; };
        if pgno < 2 as u32 { return 0 as Pgno; }
        n_pages_per_map_page =
            ((*p_bt_1).usable_size / 5 as u32 + 1 as u32) as i32;
        i_ptr_map = (pgno - 2 as Pgno) / n_pages_per_map_page as Pgno;
        ret = i_ptr_map * n_pages_per_map_page as Pgno + 2 as Pgno;
        if ret ==
                (sqlite3_pending_byte as u32 / (*p_bt_1).page_size + 1 as u32)
                    as Pgno {
            { let __p = &mut ret; let __t = *__p; *__p += 1; __t };
        }
        return ret;
    }
}
extern "C" fn btree_pagecount(p_bt_1: &BtShared) -> Pgno {
    return (*p_bt_1).n_page;
}
extern "C" fn ptrmap_get(p_bt_1: *mut BtShared, key: Pgno,
    p_e_type_1: &mut u8, p_pgno_1: *mut Pgno) -> i32 {
    let mut p_db_page: *mut DbPage = core::ptr::null_mut();
    let mut i_ptrmap: i32 = 0;
    let mut p_ptrmap: *mut u8 = core::ptr::null_mut();
    let mut offset: i32 = 0;
    let mut rc: i32 = 0;
    { let _ = 0; };
    i_ptrmap = ptrmap_pageno(unsafe { &*p_bt_1 }, key) as i32;
    rc =
        unsafe {
            sqlite3_pager_get(unsafe { (*p_bt_1).p_pager }, i_ptrmap as Pgno,
                &mut p_db_page, 0)
        };
    if rc != 0 { return rc; }
    p_ptrmap = unsafe { sqlite3_pager_get_data(p_db_page) } as *mut u8;
    offset = (5 as Pgno * (key - i_ptrmap as Pgno - 1 as Pgno)) as i32;
    if offset < 0 {
        unsafe { sqlite3_pager_unref(p_db_page) };
        return unsafe { sqlite3_corrupt_error(1165) };
    }
    { let _ = 0; };
    { let _ = 0; };
    *p_e_type_1 = unsafe { *p_ptrmap.offset(offset as isize) };
    if !(p_pgno_1).is_null() {
        unsafe {
            *p_pgno_1 =
                unsafe {
                    sqlite3_get4byte(unsafe {
                                &raw mut *p_ptrmap.offset((offset + 1) as isize)
                            } as *const u8)
                }
        };
    }
    unsafe { sqlite3_pager_unref(p_db_page) };
    if (*p_e_type_1 as i32) < 1 || *p_e_type_1 as i32 > 5 {
        return unsafe { sqlite3_corrupt_error(1173) };
    }
    return 0;
}
extern "C" fn btree_page_from_db_page(p_db_page_1: *mut DbPage, pgno: Pgno,
    p_bt_1: *mut BtShared) -> *mut MemPage {
    let p_page: *mut MemPage =
        unsafe { sqlite3_pager_get_extra(p_db_page_1) } as *mut MemPage;
    if pgno != unsafe { (*p_page).pgno } {
        unsafe {
            (*p_page).a_data =
                unsafe { sqlite3_pager_get_data(p_db_page_1) } as *mut u8
        };
        unsafe { (*p_page).p_db_page = p_db_page_1 };
        unsafe { (*p_page).p_bt = p_bt_1 };
        unsafe { (*p_page).pgno = pgno };
        unsafe {
            (*p_page).hdr_offset =
                if pgno == 1 as u32 { 100 } else { 0 } as u8
        };
    }
    { let _ = 0; };
    return p_page;
}
extern "C" fn btree_get_page(p_bt_1: *mut BtShared, pgno: Pgno,
    pp_page_1: &mut *mut MemPage, flags: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut p_db_page: *mut DbPage = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    rc =
        unsafe {
            sqlite3_pager_get(unsafe { (*p_bt_1).p_pager }, pgno,
                &raw mut p_db_page as *mut *mut DbPage, flags)
        };
    if rc != 0 { return rc; }
    *pp_page_1 = btree_page_from_db_page(p_db_page, pgno, p_bt_1);
    return 0;
}
extern "C" fn release_page_not_null(p_page_1: &MemPage) -> () {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { sqlite3_pager_unref_not_null((*p_page_1).p_db_page) };
}
extern "C" fn release_page(p_page_1: *mut MemPage) -> () {
    if !(p_page_1).is_null() { release_page_not_null(unsafe { &*p_page_1 }); }
}
extern "C" fn get_overflow_page(p_bt_1: *mut BtShared, ovfl: Pgno,
    pp_page_1: *mut *mut MemPage, p_pgno_next_1: *mut Pgno) -> i32 {
    unsafe {
        let mut next: Pgno = 0 as Pgno;
        let mut p_page: *mut MemPage = core::ptr::null_mut();
        let mut rc: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_bt_1).auto_vacuum } != 0 {
            let mut pgno: Pgno = 0 as Pgno;
            let mut i_guess: Pgno = ovfl + 1 as Pgno;
            let mut e_type: u8 = 0 as u8;
            while ptrmap_pageno(unsafe { &*p_bt_1 }, i_guess) == i_guess ||
                    i_guess ==
                        (sqlite3_pending_byte as u32 /
                                    unsafe { (*p_bt_1).page_size } + 1 as u32) as Pgno {
                { let __p = &mut i_guess; let __t = *__p; *__p += 1; __t };
            }
            if i_guess <= btree_pagecount(unsafe { &*p_bt_1 }) {
                rc = ptrmap_get(p_bt_1, i_guess, &mut e_type, &mut pgno);
                if rc == 0 && e_type as i32 == 4 && pgno == ovfl {
                    next = i_guess;
                    rc = 101;
                }
            }
        }
        { let _ = 0; };
        if rc == 0 {
            rc =
                btree_get_page(p_bt_1, ovfl, &mut p_page,
                    if pp_page_1 == core::ptr::null_mut() { 2 } else { 0 });
            { let _ = 0; };
            if rc == 0 {
                next =
                    unsafe {
                        sqlite3_get4byte(unsafe { (*p_page).a_data } as *const u8)
                    };
            }
        }
        unsafe { *p_pgno_next_1 = next };
        if !(pp_page_1).is_null() {
            unsafe { *pp_page_1 = p_page };
        } else { release_page(p_page); }
        return if rc == 101 { 0 } else { rc };
    }
}
extern "C" fn access_payload(p_cur_1: *mut BtCursor, mut offset: u32,
    mut amt: u32, mut p_buf_1: *mut u8, e_op_1: i32) -> i32 {
    let mut a_payload: *mut u8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut i_idx: i32 = 0;
    let p_page: *const MemPage =
        unsafe { (*p_cur_1).p_page } as *const MemPage;
    let p_bt: *mut BtShared = unsafe { (*p_cur_1).p_bt };
    let p_buf_start: *mut u8 = p_buf_1;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_cur_1).ix } as i32 >= unsafe { (*p_page).n_cell } as i32 {
        return unsafe { sqlite3_corrupt_error(5175) };
    }
    { let _ = 0; };
    get_cell_info(unsafe { &mut *p_cur_1 });
    a_payload = unsafe { (*p_cur_1).info.p_payload };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { a_payload.offset_from(unsafe { (*p_page).a_data }) } as i64 as
                uptr >
            (unsafe { (*p_bt).usable_size } -
                    unsafe { (*p_cur_1).info.n_local } as u32) as u64 {
        return unsafe { sqlite3_corrupt_error(5190) };
    }
    if offset < unsafe { (*p_cur_1).info.n_local } as u32 {
        let mut a: i32 = amt as i32;
        if a as u32 + offset > unsafe { (*p_cur_1).info.n_local } as u32 {
            a = (unsafe { (*p_cur_1).info.n_local } as u32 - offset) as i32;
        }
        rc =
            copy_payload(unsafe { &raw mut *a_payload.add(offset as usize) }
                    as *mut (),
                unsafe {
                    let __p = p_buf_1 as *mut u8 as *mut u8;
                    if __p.is_null() {
                        &mut []
                    } else { core::slice::from_raw_parts_mut(__p, a as usize) }
                }, e_op_1, unsafe { (*p_page).p_db_page });
        offset = 0 as u32;
        {
            let __n = a;
            let __p = &mut p_buf_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        amt -= a as u32;
    } else { offset -= unsafe { (*p_cur_1).info.n_local } as u32; }
    if rc == 0 && amt > 0 as u32 {
        let ovfl_size: u32 =
            (unsafe { (*p_bt).usable_size } - 4 as u32) as u32;
        let mut next_page: Pgno = 0 as Pgno;
        next_page =
            unsafe {
                sqlite3_get4byte(unsafe {
                            &raw mut *a_payload.add(unsafe { (*p_cur_1).info.n_local }
                                            as usize)
                        } as *const u8)
            };
        if unsafe { (*p_cur_1).cur_flags } as i32 & 4 == 0 {
            let mut n_ovfl: i64 = unsafe { (*p_cur_1).info.n_payload } as i64;
            n_ovfl =
                (n_ovfl - unsafe { (*p_cur_1).info.n_local } as i64 +
                            ovfl_size as i64 - 1 as i64) / ovfl_size as i64;
            if unsafe { (*p_cur_1).a_overflow } == core::ptr::null_mut() ||
                    n_ovfl * core::mem::size_of::<Pgno>() as i32 as i64 >
                        unsafe {
                                sqlite3_malloc_size(unsafe { (*p_cur_1).a_overflow } as
                                        *const ())
                            } as i64 {
                let mut a_new: *mut Pgno = core::ptr::null_mut();
                if unsafe { sqlite3_fault_sim(413) } != 0 {
                    a_new = core::ptr::null_mut();
                } else {
                    a_new =
                        unsafe {
                                sqlite3Realloc(unsafe { (*p_cur_1).a_overflow } as *mut (),
                                    (n_ovfl * 2 as i64) as u64 *
                                        core::mem::size_of::<Pgno>() as u64)
                            } as *mut Pgno;
                }
                if a_new == core::ptr::null_mut() {
                    return 7;
                } else { unsafe { (*p_cur_1).a_overflow = a_new }; }
            }
            unsafe {
                memset(unsafe { (*p_cur_1).a_overflow } as *mut (), 0,
                    n_ovfl as u64 * core::mem::size_of::<Pgno>() as u64)
            };
            unsafe { (*p_cur_1).cur_flags |= 4 as u8 };
        } else {
            { let _ = 0; };
            { let _ = 0; };
            if unsafe {
                        *unsafe {
                                (*p_cur_1).a_overflow.add((offset / ovfl_size as u32) as
                                        usize)
                            }
                    } != 0 {
                i_idx = (offset / ovfl_size as u32) as i32;
                next_page =
                    unsafe {
                        *unsafe { (*p_cur_1).a_overflow.offset(i_idx as isize) }
                    };
                offset = offset % ovfl_size as u32;
            }
        }
        { let _ = 0; };
        while next_page != 0 {
            if next_page > unsafe { (*p_bt).n_page } {
                return unsafe { sqlite3_corrupt_error(5263) };
            }
            { let _ = 0; };
            unsafe {
                *unsafe { (*p_cur_1).a_overflow.offset(i_idx as isize) } =
                    next_page
            };
            if offset >= ovfl_size {
                { let _ = 0; };
                { let _ = 0; };
                if unsafe {
                            *unsafe {
                                    (*p_cur_1).a_overflow.offset((i_idx + 1) as isize)
                                }
                        } != 0 {
                    next_page =
                        unsafe {
                            *unsafe {
                                    (*p_cur_1).a_overflow.offset((i_idx + 1) as isize)
                                }
                        };
                } else {
                    rc =
                        get_overflow_page(p_bt, next_page, core::ptr::null_mut(),
                            &mut next_page);
                }
                offset -= ovfl_size as u32;
            } else {
                let mut a: i32 = amt as i32;
                if a as u32 + offset > ovfl_size {
                    a = (ovfl_size - offset) as i32;
                }
                if e_op_1 == 0 && offset == 0 as u32 &&
                            unsafe {
                                    sqlite3_pager_direct_read_ok(unsafe { (*p_bt).p_pager },
                                        next_page)
                                } != 0 &&
                        unsafe { p_buf_1.offset(-4 as isize) } >= p_buf_start {
                    let fd: *mut sqlite3_file =
                        unsafe { sqlite3_pager_file(unsafe { (*p_bt).p_pager }) };
                    let mut a_save: [u8; 4] = [0; 4];
                    let a_write: *mut u8 =
                        unsafe { &mut *p_buf_1.offset(-4 as isize) };
                    { let _ = 0; };
                    unsafe {
                        memcpy(&raw mut a_save[0 as usize] as *mut u8 as *mut (),
                            a_write as *const (), 4 as u64)
                    };
                    rc =
                        unsafe {
                            sqlite3_os_read(fd, a_write as *mut (), a + 4,
                                unsafe { (*p_bt).page_size } as i64 *
                                    (next_page - 1 as Pgno) as i64)
                        };
                    next_page =
                        unsafe { sqlite3_get4byte(a_write as *const u8) };
                    unsafe {
                        memcpy(a_write as *mut (),
                            &raw mut a_save[0 as usize] as *mut u8 as *const (),
                            4 as u64)
                    };
                } else {
                    let mut p_db_page: *mut DbPage = core::ptr::null_mut();
                    rc =
                        unsafe {
                            sqlite3_pager_get(unsafe { (*p_bt).p_pager }, next_page,
                                &mut p_db_page, if e_op_1 == 0 { 2 } else { 0 })
                        };
                    if rc == 0 {
                        if e_op_1 != 0 &&
                                    (unsafe { sqlite3_pager_page_refcount(p_db_page) } != 1 ||
                                        unsafe {
                                                (*(unsafe { sqlite3_pager_get_extra(p_db_page) } as
                                                                *mut MemPage)).is_init
                                            } != 0) && unsafe { sqlite3_fault_sim(411) } == 0 {
                            unsafe { sqlite3_pager_unref(p_db_page) };
                            return unsafe { sqlite3_corrupt_error(5335) };
                        }
                        a_payload =
                            unsafe { sqlite3_pager_get_data(p_db_page) } as *mut u8;
                        next_page =
                            unsafe { sqlite3_get4byte(a_payload as *const u8) };
                        rc =
                            copy_payload(unsafe {
                                        &raw mut *a_payload.add((offset + 4 as u32) as usize)
                                    } as *mut (),
                                unsafe {
                                    let __p = p_buf_1 as *mut u8 as *mut u8;
                                    if __p.is_null() {
                                        &mut []
                                    } else { core::slice::from_raw_parts_mut(__p, a as usize) }
                                }, e_op_1, p_db_page);
                        unsafe { sqlite3_pager_unref(p_db_page) };
                        offset = 0 as u32;
                    }
                }
                amt -= a as u32;
                if amt == 0 as u32 { return rc; }
                {
                    let __n = a;
                    let __p = &mut p_buf_1;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            }
            if rc != 0 { break; }
            { let __p = &mut i_idx; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 && amt > 0 as u32 {
        return unsafe { sqlite3_corrupt_error(5355) };
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_payload(p_cur_1: *mut BtCursor, offset: u32,
    amt: u32, p_buf_1: *mut ()) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    return access_payload(p_cur_1, offset, amt, p_buf_1 as *mut u8, 0);
}
extern "C" fn save_cursor_key(p_cur_1: *mut BtCursor) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_cur_1).cur_int_key } != 0 {
        unsafe {
            (*p_cur_1).n_key = unsafe { sqlite3_btree_integer_key(p_cur_1) }
        };
    } else {
        let mut p_key: *mut () = core::ptr::null_mut();
        unsafe {
            (*p_cur_1).n_key =
                unsafe { sqlite3_btree_payload_size(p_cur_1) } as i64
        };
        p_key =
            unsafe {
                sqlite3Malloc((unsafe { (*p_cur_1).n_key } as i64 + 9 as i64 +
                            8 as i64) as u64)
            };
        if !(p_key).is_null() {
            rc =
                unsafe {
                    sqlite3_btree_payload(p_cur_1, 0 as u32,
                        unsafe { (*p_cur_1).n_key } as i32 as u32, p_key)
                };
            if rc == 0 {
                unsafe {
                    memset(unsafe {
                                (p_key as
                                        *mut u8).offset(unsafe { (*p_cur_1).n_key } as isize)
                            } as *mut (), 0, (9 + 8) as u64)
                };
                unsafe { (*p_cur_1).p_key = p_key };
            } else { unsafe { sqlite3_free(p_key) }; }
        } else { rc = 7; }
    }
    { let _ = 0; };
    return rc;
}
extern "C" fn btree_release_all_cursor_pages(p_cur_1: &mut BtCursor) -> () {
    let mut i: i32 = 0;
    if (*p_cur_1).i_page as i32 >= 0 {
        {
            i = 0;
            '__b11: loop {
                if !(i < (*p_cur_1).i_page as i32) { break '__b11; }
                '__c11: loop {
                    unsafe {
                        release_page_not_null(unsafe {
                                &*(*p_cur_1).ap_page[i as usize]
                            })
                    };
                    break '__c11;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { release_page_not_null(unsafe { &*(*p_cur_1).p_page }) };
        (*p_cur_1).i_page = -1 as i8;
    }
}
extern "C" fn save_cursor_position(p_cur_1: *mut BtCursor) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_cur_1).cur_flags } as i32 & 64 != 0 {
        return 19 | 11 << 8;
    }
    if unsafe { (*p_cur_1).e_state } as i32 == 2 {
        unsafe { (*p_cur_1).e_state = 0 as u8 };
    } else { unsafe { (*p_cur_1).skip_next = 0 }; }
    rc = save_cursor_key(p_cur_1);
    if rc == 0 {
        btree_release_all_cursor_pages(unsafe { &mut *p_cur_1 });
        unsafe { (*p_cur_1).e_state = 3 as u8 };
    }
    unsafe { (*p_cur_1).cur_flags &= !(2 | 4 | 8) as u8 };
    return rc;
}
extern "C" fn save_cursors_on_list(mut p: *mut BtCursor, i_root_1: Pgno,
    p_except_1: *mut BtCursor) -> i32 {
    '__b12: loop {
        '__c12: loop {
            if p != p_except_1 &&
                    (0 as Pgno == i_root_1 ||
                        unsafe { (*p).pgno_root } == i_root_1) {
                if unsafe { (*p).e_state } as i32 == 0 ||
                        unsafe { (*p).e_state } as i32 == 2 {
                    let rc: i32 = save_cursor_position(p);
                    if 0 != rc { return rc; }
                } else { btree_release_all_cursor_pages(unsafe { &mut *p }); }
            }
            p = unsafe { (*p).p_next };
            break '__c12;
        }
        if !(!(p).is_null()) { break '__b12; }
    }
    return 0;
}
extern "C" fn save_all_cursors(p_bt_1: &BtShared, i_root_1: Pgno,
    p_except_1: *mut BtCursor) -> i32 {
    let mut p: *mut BtCursor = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    {
        p = (*p_bt_1).p_cursor;
        '__b13: loop {
            if !(!(p).is_null()) { break '__b13; }
            '__c13: loop {
                if p != p_except_1 &&
                        (0 as Pgno == i_root_1 ||
                            unsafe { (*p).pgno_root } == i_root_1) {
                    break '__b13;
                }
                break '__c13;
            }
            p = unsafe { (*p).p_next };
        }
    }
    if !(p).is_null() {
        return unsafe { save_cursors_on_list(p, i_root_1, p_except_1) };
    }
    if !(p_except_1).is_null() {
        unsafe { (*p_except_1).cur_flags &= !32 as u8 };
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_clear_cursor(p_cur_1: &mut BtCursor) -> () {
    { let _ = 0; };
    unsafe { sqlite3_free((*p_cur_1).p_key) };
    (*p_cur_1).p_key = core::ptr::null_mut();
    (*p_cur_1).e_state = 1 as u8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_trip_all_cursors(p_btree_1: *mut Btree,
    err_code_1: i32, write_only_1: i32) -> i32 {
    let mut p: *mut BtCursor = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = 0; };
    if !(p_btree_1).is_null() {
        unsafe { sqlite3_btree_enter(p_btree_1) };
        {
            p = unsafe { (*unsafe { (*p_btree_1).p_bt }).p_cursor };
            '__b14: loop {
                if !(!(p).is_null()) { break '__b14; }
                '__c14: loop {
                    if write_only_1 != 0 &&
                            unsafe { (*p).cur_flags } as i32 & 1 == 0 {
                        if unsafe { (*p).e_state } as i32 == 0 ||
                                unsafe { (*p).e_state } as i32 == 2 {
                            rc = save_cursor_position(p);
                            if rc != 0 {
                                {
                                    let _ = sqlite3_btree_trip_all_cursors(p_btree_1, rc, 0);
                                };
                                break '__b14;
                            }
                        }
                    } else {
                        sqlite3_btree_clear_cursor(unsafe { &mut *p });
                        unsafe { (*p).e_state = 4 as u8 };
                        unsafe { (*p).skip_next = err_code_1 };
                    }
                    btree_release_all_cursor_pages(unsafe { &mut *p });
                    break '__c14;
                }
                p = unsafe { (*p).p_next };
            }
        }
        unsafe { sqlite3_btree_leave(p_btree_1) };
    }
    return rc;
}
extern "C" fn btree_set_n_page(p_bt_1: &mut BtShared, p_page1_1: &MemPage)
    -> () {
    let mut n_page: i32 =
        unsafe {
                sqlite3_get4byte(unsafe {
                            &raw mut *(*p_page1_1).a_data.offset(28 as isize)
                        } as *const u8)
            } as i32;
    if n_page == 0 {
        unsafe { sqlite3_pager_pagecount((*p_bt_1).p_pager, &mut n_page) };
    }
    (*p_bt_1).n_page = n_page as u32;
}
extern "C" fn release_page_one(p_page_1: &MemPage) -> () {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { sqlite3_pager_unref_page_one((*p_page_1).p_db_page) };
}
extern "C" fn btree_clear_has_content(p_bt_1: &mut BtShared) -> () {
    unsafe { sqlite3_bitvec_destroy((*p_bt_1).p_has_content) };
    (*p_bt_1).p_has_content = core::ptr::null_mut();
}
extern "C" fn downgrade_all_shared_cache_table_locks(p: *mut Btree) -> () {
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    if unsafe { (*p_bt).p_writer } == p {
        let mut p_lock: *mut BtLock = core::ptr::null_mut();
        unsafe { (*p_bt).p_writer = core::ptr::null_mut() };
        unsafe { (*p_bt).bts_flags &= !(64 | 128) as u16 };
        {
            p_lock = unsafe { (*p_bt).p_lock };
            '__b15: loop {
                if !(!(p_lock).is_null()) { break '__b15; }
                '__c15: loop {
                    { let _ = 0; };
                    unsafe { (*p_lock).e_lock = 1 as u8 };
                    break '__c15;
                }
                p_lock = unsafe { (*p_lock).p_next };
            }
        }
    }
}
extern "C" fn clear_all_shared_cache_table_locks(p: *mut Btree) -> () {
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    let mut pp_iter: *mut *mut BtLock = unsafe { &mut (*p_bt).p_lock };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    while !(unsafe { *pp_iter }).is_null() {
        let p_lock: *mut BtLock = unsafe { *pp_iter };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_lock).p_btree } == p {
            unsafe { *pp_iter = unsafe { (*p_lock).p_next } };
            { let _ = 0; };
            if unsafe { (*p_lock).i_table } != 1 as u32 {
                unsafe { sqlite3_free(p_lock as *mut ()) };
            }
        } else { pp_iter = unsafe { &mut (*p_lock).p_next }; }
    }
    { let _ = 0; };
    if unsafe { (*p_bt).p_writer } == p {
        unsafe { (*p_bt).p_writer = core::ptr::null_mut() };
        unsafe { (*p_bt).bts_flags &= !(64 | 128) as u16 };
    } else if unsafe { (*p_bt).n_transaction } == 2 {
        unsafe { (*p_bt).bts_flags &= !128 as u16 };
    }
}
extern "C" fn unlock_btree_if_unused(p_bt_1: &mut BtShared) -> () {
    { let _ = 0; };
    { let _ = 0; };
    if (*p_bt_1).in_transaction as i32 == 0 &&
            (*p_bt_1).p_page1 != core::ptr::null_mut() {
        let p_page1: *mut MemPage = (*p_bt_1).p_page1;
        { let _ = 0; };
        { let _ = 0; };
        (*p_bt_1).p_page1 = core::ptr::null_mut();
        release_page_one(unsafe { &*p_page1 });
    }
}
extern "C" fn btree_end_transaction(p: *mut Btree) -> () {
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    let db: *const sqlite3 = unsafe { (*p).db } as *const sqlite3;
    { let _ = 0; };
    unsafe { (*p_bt).b_do_truncate = 0 as u8 };
    if unsafe { (*p).in_trans } as i32 > 0 && unsafe { (*db).n_vdbe_read } > 1
        {
        downgrade_all_shared_cache_table_locks(p);
        unsafe { (*p).in_trans = 1 as u8 };
    } else {
        if unsafe { (*p).in_trans } as i32 != 0 {
            clear_all_shared_cache_table_locks(p);
            {
                let __p = unsafe { &mut (*p_bt).n_transaction };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            if 0 == unsafe { (*p_bt).n_transaction } {
                unsafe { (*p_bt).in_transaction = 0 as u8 };
            }
        }
        unsafe { (*p).in_trans = 0 as u8 };
        unlock_btree_if_unused(unsafe { &mut *p_bt });
    }
    { let _ = 0; };
    { let _ = 0; };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_rollback(p: *mut Btree, mut trip_code_1: i32,
    mut write_only_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    let mut p_page1: *mut MemPage = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    unsafe { sqlite3_btree_enter(p) };
    if trip_code_1 == 0 {
        rc =
            {
                trip_code_1 =
                    save_all_cursors(unsafe { &*p_bt }, 0 as Pgno,
                        core::ptr::null_mut());
                trip_code_1
            };
        if rc != 0 { write_only_1 = 0; }
    } else { rc = 0; }
    if trip_code_1 != 0 {
        let mut rc2: i32 =
            sqlite3_btree_trip_all_cursors(p, trip_code_1, write_only_1);
        { let _ = 0; };
        if rc2 != 0 { rc = rc2; }
    }
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p).in_trans } as i32 == 2 {
        let mut rc2: i32 = 0;
        { let _ = 0; };
        rc2 = unsafe { sqlite3_pager_rollback(unsafe { (*p_bt).p_pager }) };
        if rc2 != 0 { rc = rc2; }
        if btree_get_page(p_bt, 1 as Pgno, &mut p_page1, 0) == 0 {
            btree_set_n_page(unsafe { &mut *p_bt }, unsafe { &*p_page1 });
            release_page_one(unsafe { &*p_page1 });
        }
        { let _ = 0; };
        unsafe { (*p_bt).in_transaction = 1 as u8 };
        btree_clear_has_content(unsafe { &mut *p_bt });
    }
    btree_end_transaction(p);
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
extern "C" fn remove_from_sharing_list(p_bt_1: *mut BtShared) -> i32 {
    unsafe {
        let mut p_main_mtx: *mut sqlite3_mutex = core::ptr::null_mut();
        let mut p_list: *mut BtShared = core::ptr::null_mut();
        let mut removed: i32 = 0;
        { let _ = 0; };
        p_main_mtx = unsafe { sqlite3MutexAlloc(2) };
        unsafe { sqlite3_mutex_enter(p_main_mtx) };
        {
            let __p = unsafe { &mut (*p_bt_1).n_ref };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p_bt_1).n_ref } <= 0 {
            if sqlite3_shared_cache_list == p_bt_1 {
                sqlite3_shared_cache_list = unsafe { (*p_bt_1).p_next };
            } else {
                p_list = sqlite3_shared_cache_list;
                while !(p_list).is_null() &&
                        unsafe { (*p_list).p_next } != p_bt_1 {
                    p_list = unsafe { (*p_list).p_next };
                }
                if !(p_list).is_null() {
                    unsafe { (*p_list).p_next = unsafe { (*p_bt_1).p_next } };
                }
            }
            if 1 != 0 {
                unsafe { sqlite3_mutex_free(unsafe { (*p_bt_1).mutex }) };
            }
            removed = 1;
        }
        unsafe { sqlite3_mutex_leave(p_main_mtx) };
        return removed;
    }
}
extern "C" fn free_temp_space(p_bt_1: &mut BtShared) -> () {
    if !((*p_bt_1).p_tmp_space).is_null() {
        {
            let __n = 4;
            let __p = &mut (*p_bt_1).p_tmp_space;
            *__p = unsafe { (*__p).offset(-(__n as isize)) };
        };
        unsafe { sqlite3_page_free((*p_bt_1).p_tmp_space as *mut ()) };
        (*p_bt_1).p_tmp_space = core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_close(p: *mut Btree) -> i32 {
    unsafe {
        let p_bt: *mut BtShared = unsafe { (*p).p_bt };
        { let _ = 0; };
        unsafe { sqlite3_btree_enter(p) };
        unsafe { sqlite3_btree_rollback(p, 0, 0) };
        unsafe { sqlite3_btree_leave(p) };
        { let _ = 0; };
        if (unsafe { (*p).sharable } == 0) as i32 != 0 ||
                remove_from_sharing_list(p_bt) != 0 {
            { let _ = 0; };
            unsafe {
                sqlite3_pager_close(unsafe { (*p_bt).p_pager },
                    unsafe { (*p).db })
            };
            if unsafe { (*p_bt).x_free_schema.is_some() } &&
                    !(unsafe { (*p_bt).p_schema }).is_null() {
                unsafe {
                    (unsafe {
                            (*p_bt).x_free_schema.unwrap()
                        })(unsafe { (*p_bt).p_schema })
                };
            }
            unsafe {
                sqlite3_db_free(core::ptr::null_mut(),
                    unsafe { (*p_bt).p_schema })
            };
            free_temp_space(unsafe { &mut *p_bt });
            unsafe { sqlite3_free(p_bt as *mut ()) };
        }
        { let _ = 0; };
        { let _ = 0; };
        if !(unsafe { (*p).p_prev }).is_null() {
            unsafe {
                (*unsafe { (*p).p_prev }).p_next = unsafe { (*p).p_next }
            };
        }
        if !(unsafe { (*p).p_next }).is_null() {
            unsafe {
                (*unsafe { (*p).p_next }).p_prev = unsafe { (*p).p_prev }
            };
        }
        unsafe { sqlite3_free(p as *mut ()) };
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_set_spill_size(p: *mut Btree, mx_page_1: i32)
    -> i32 {
    let p_bt: *const BtShared = unsafe { (*p).p_bt } as *const BtShared;
    let mut res: i32 = 0;
    { let _ = 0; };
    unsafe { sqlite3_btree_enter(p) };
    res =
        unsafe {
            sqlite3_pager_set_spillsize(unsafe { (*p_bt).p_pager }, mx_page_1)
        };
    unsafe { sqlite3_btree_leave(p) };
    return res;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_set_mmap_limit(p: *mut Btree,
    sz_mmap_1: sqlite3_int64) -> i32 {
    let p_bt: *const BtShared = unsafe { (*p).p_bt } as *const BtShared;
    { let _ = 0; };
    unsafe { sqlite3_btree_enter(p) };
    unsafe {
        sqlite3_pager_set_mmap_limit(unsafe { (*p_bt).p_pager }, sz_mmap_1)
    };
    unsafe { sqlite3_btree_leave(p) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_set_pager_flags(p: *mut Btree,
    pg_flags_1: u32) -> i32 {
    let p_bt: *const BtShared = unsafe { (*p).p_bt } as *const BtShared;
    { let _ = 0; };
    unsafe { sqlite3_btree_enter(p) };
    unsafe {
        sqlite3_pager_set_flags(unsafe { (*p_bt).p_pager }, pg_flags_1)
    };
    unsafe { sqlite3_btree_leave(p) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_set_page_size(p: *mut Btree,
    mut page_size_1: i32, mut n_reserve_1: i32, i_fix_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut x: i32 = 0;
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    { let _ = 0; };
    unsafe { sqlite3_btree_enter(p) };
    unsafe { (*p_bt).n_reserve_wanted = n_reserve_1 as u8 };
    x =
        (unsafe { (*p_bt).page_size } - unsafe { (*p_bt).usable_size }) as
            i32;
    if x == n_reserve_1 &&
            (page_size_1 == 0 ||
                page_size_1 as u32 == unsafe { (*p_bt).page_size }) {
        unsafe { sqlite3_btree_leave(p) };
        return 0;
    }
    if n_reserve_1 < x { n_reserve_1 = x; }
    if unsafe { (*p_bt).bts_flags } as i32 & 2 != 0 {
        unsafe { sqlite3_btree_leave(p) };
        return 8;
    }
    { let _ = 0; };
    if page_size_1 >= 512 && page_size_1 <= 65536 &&
            page_size_1 - 1 & page_size_1 == 0 {
        { let _ = 0; };
        { let _ = 0; };
        if n_reserve_1 > 32 && page_size_1 == 512 { page_size_1 = 1024; }
        unsafe { (*p_bt).page_size = page_size_1 as u32 };
        free_temp_space(unsafe { &mut *p_bt });
    }
    rc =
        unsafe {
            sqlite3_pager_set_pagesize(unsafe { (*p_bt).p_pager },
                unsafe { &mut (*p_bt).page_size }, n_reserve_1)
        };
    unsafe {
        (*p_bt).usable_size =
            unsafe { (*p_bt).page_size } - n_reserve_1 as u16 as u32
    };
    if i_fix_1 != 0 { unsafe { (*p_bt).bts_flags |= 2 as u16 }; }
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_get_page_size(p: &Btree) -> i32 {
    return unsafe { (*(*p).p_bt).page_size } as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_max_page_count(p: *mut Btree, mx_page_1: Pgno)
    -> Pgno {
    let mut n: Pgno = 0 as Pgno;
    unsafe { sqlite3_btree_enter(p) };
    n =
        unsafe {
            sqlite3_pager_max_page_count(unsafe {
                    (*unsafe { (*p).p_bt }).p_pager
                }, mx_page_1)
        };
    unsafe { sqlite3_btree_leave(p) };
    return n;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_last_page(p: &Btree) -> Pgno {
    { let _ = 0; };
    return btree_pagecount(unsafe { &*(*p).p_bt });
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_secure_delete(p: *mut Btree, new_flag_1: i32)
    -> i32 {
    let mut b: i32 = 0;
    if p == core::ptr::null_mut() { return 0; }
    unsafe { sqlite3_btree_enter(p) };
    { let _ = 0; };
    { let _ = 0; };
    if new_flag_1 >= 0 {
        unsafe { (*unsafe { (*p).p_bt }).bts_flags &= !12 as u16 };
        unsafe {
            (*unsafe { (*p).p_bt }).bts_flags |=
                (4 * new_flag_1) as u16 as i32 as u16
        };
    }
    b = (unsafe { (*unsafe { (*p).p_bt }).bts_flags } as i32 & 12) / 4;
    unsafe { sqlite3_btree_leave(p) };
    return b;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_get_reserve_no_mutex(p: &Btree) -> i32 {
    let mut n: i32 = 0;
    { let _ = 0; };
    n =
        (unsafe { (*(*p).p_bt).page_size } -
                unsafe { (*(*p).p_bt).usable_size }) as i32;
    return n;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_get_requested_reserve(p: *mut Btree) -> i32 {
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    unsafe { sqlite3_btree_enter(p) };
    n1 = unsafe { (*unsafe { (*p).p_bt }).n_reserve_wanted } as i32;
    n2 = sqlite3_btree_get_reserve_no_mutex(unsafe { &*p });
    unsafe { sqlite3_btree_leave(p) };
    return if n1 > n2 { n1 } else { n2 };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_set_auto_vacuum(p: *mut Btree,
    auto_vacuum_1: i32) -> i32 {
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    let mut rc: i32 = 0;
    let av: u8 = auto_vacuum_1 as u8;
    unsafe { sqlite3_btree_enter(p) };
    if unsafe { (*p_bt).bts_flags } as i32 & 2 != 0 &&
            if av != 0 { 1 } else { 0 } !=
                unsafe { (*p_bt).auto_vacuum } as i32 {
        rc = 8;
    } else {
        unsafe { (*p_bt).auto_vacuum = if av != 0 { 1 } else { 0 } as u8 };
        unsafe {
            (*p_bt).incr_vacuum = if av as i32 == 2 { 1 } else { 0 } as u8
        };
    }
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_get_auto_vacuum(p: *mut Btree) -> i32 {
    let mut rc: i32 = 0;
    unsafe { sqlite3_btree_enter(p) };
    rc =
        if (unsafe { (*unsafe { (*p).p_bt }).auto_vacuum } == 0) as i32 != 0 {
            0
        } else {
            if (unsafe { (*unsafe { (*p).p_bt }).incr_vacuum } == 0) as i32 !=
                    0 {
                1
            } else { 2 }
        };
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
extern "C" fn query_shared_cache_table_lock(p: *mut Btree, i_tab_1: Pgno,
    e_lock_1: u8) -> i32 {
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    let mut p_iter: *const BtLock = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if (unsafe { (*p).sharable } == 0) as i32 != 0 { return 0; }
    if unsafe { (*p_bt).p_writer } != p &&
            unsafe { (*p_bt).bts_flags } as i32 & 64 != 0 {
        return 6 | 1 << 8;
    }
    {
        p_iter = unsafe { (*p_bt).p_lock };
        '__b18: loop {
            if !(!(p_iter).is_null()) { break '__b18; }
            '__c18: loop {
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p_iter).p_btree } != p &&
                            unsafe { (*p_iter).i_table } == i_tab_1 &&
                        unsafe { (*p_iter).e_lock } as i32 != e_lock_1 as i32 {
                    if e_lock_1 as i32 == 2 {
                        { let _ = 0; };
                        unsafe { (*p_bt).bts_flags |= 128 as u16 };
                    }
                    return 6 | 1 << 8;
                }
                break '__c18;
            }
            p_iter = unsafe { (*p_iter).p_next };
        }
    }
    return 0;
}
static z_magic_header: [i8; 16] =
    [83 as i8, 81 as i8, 76 as i8, 105 as i8, 116 as i8, 101 as i8, 32 as i8,
            102 as i8, 111 as i8, 114 as i8, 109 as i8, 97 as i8, 116 as i8,
            32 as i8, 51 as i8, 0 as i8];
extern "C" fn lock_btree(p_bt_1: *mut BtShared) -> i32 {
    let mut rc: i32 = 0;
    let mut p_page1: *mut MemPage = core::ptr::null_mut();
    '__b19: loop {
        '__c19: loop {
            let mut n_page: u32 = 0 as u32;
            let mut n_page_file: u32 = 0 as u32;
            { let _ = 0; };
            { let _ = 0; };
            rc =
                unsafe {
                    sqlite3_pager_shared_lock(unsafe { (*p_bt_1).p_pager })
                };
            if rc != 0 { return rc; }
            rc = btree_get_page(p_bt_1, 1 as Pgno, &mut p_page1, 0);
            if rc != 0 { return rc; }
            n_page =
                unsafe {
                    sqlite3_get4byte(unsafe {
                                (unsafe { (*p_page1).a_data } as
                                        *mut u8).offset(28 as isize)
                            } as *const u8)
                };
            unsafe {
                sqlite3_pager_pagecount(unsafe { (*p_bt_1).p_pager },
                    &raw mut n_page_file as *mut i32)
            };
            if n_page == 0 as u32 ||
                    unsafe {
                            memcmp(unsafe {
                                        (unsafe { (*p_page1).a_data } as
                                                *mut u8).offset(24 as isize)
                                    } as *const (),
                                unsafe {
                                        (unsafe { (*p_page1).a_data } as
                                                *mut u8).offset(92 as isize)
                                    } as *const (), 4 as u64)
                        } != 0 {
                n_page = n_page_file;
            }
            if unsafe { (*unsafe { (*p_bt_1).db }).flags } & 33554432 as u64
                    != 0 as u64 {
                n_page = 0 as u32;
            }
            if n_page > 0 as u32 {
                let mut page_size: u32 = 0 as u32;
                let mut usable_size: u32 = 0 as u32;
                let page1: *mut u8 = unsafe { (*p_page1).a_data };
                rc = 26;
                if unsafe {
                            memcmp(page1 as *const (),
                                &raw const z_magic_header[0 as usize] as *const i8 as
                                    *const (), 16 as u64)
                        } != 0 {
                    break '__b19;
                }
                if unsafe { *page1.offset(18 as isize) } as i32 > 2 {
                    unsafe { (*p_bt_1).bts_flags |= 1 as u16 };
                }
                if unsafe { *page1.offset(19 as isize) } as i32 > 2 {
                    break '__b19;
                }
                if unsafe { *page1.offset(19 as isize) } as i32 == 2 &&
                        unsafe { (*p_bt_1).bts_flags } as i32 & 32 == 0 {
                    let mut is_open: i32 = 0;
                    rc =
                        unsafe {
                            sqlite3_pager_open_wal(unsafe { (*p_bt_1).p_pager },
                                &mut is_open)
                        };
                    if rc != 0 {
                        break '__b19;
                    } else {
                        if is_open == 0 {
                            release_page_one(unsafe { &*p_page1 });
                            return 0;
                        }
                    }
                    rc = 26;
                } else {}
                if unsafe {
                            memcmp(unsafe { &raw mut *page1.offset(21 as isize) } as
                                    *const (), c"@  ".as_ptr() as *mut i8 as *const (),
                                3 as u64)
                        } != 0 {
                    break '__b19;
                }
                page_size =
                    ((unsafe { *page1.offset(16 as isize) } as i32) << 8 |
                            (unsafe { *page1.offset(17 as isize) } as i32) << 16) as
                        u32;
                if page_size - 1 as u32 & page_size != 0 as u32 ||
                            page_size > 65536 as u32 || page_size <= 256 as u32 {
                    break '__b19;
                }
                { let _ = 0; };
                usable_size =
                    page_size - unsafe { *page1.offset(20 as isize) } as u32;
                if page_size as u32 != unsafe { (*p_bt_1).page_size } {
                    release_page_one(unsafe { &*p_page1 });
                    unsafe { (*p_bt_1).usable_size = usable_size };
                    unsafe { (*p_bt_1).page_size = page_size };
                    unsafe { (*p_bt_1).bts_flags |= 2 as u16 };
                    free_temp_space(unsafe { &mut *p_bt_1 });
                    rc =
                        unsafe {
                            sqlite3_pager_set_pagesize(unsafe { (*p_bt_1).p_pager },
                                unsafe { &mut (*p_bt_1).page_size },
                                (page_size - usable_size) as i32)
                        };
                    return rc;
                }
                if n_page > n_page_file {
                    if unsafe {
                                sqlite3_writable_schema(unsafe { (*p_bt_1).db })
                            } == 0 {
                        rc = unsafe { sqlite3_corrupt_error(3437) };
                        break '__b19;
                    } else { n_page = n_page_file; }
                }
                if usable_size < 480 as u32 { break '__b19; }
                unsafe { (*p_bt_1).bts_flags |= 2 as u16 };
                unsafe { (*p_bt_1).page_size = page_size };
                unsafe { (*p_bt_1).usable_size = usable_size };
                unsafe {
                    (*p_bt_1).auto_vacuum =
                        if unsafe {
                                        sqlite3_get4byte(unsafe {
                                                    &raw mut *page1.offset((36 + 4 * 4) as isize)
                                                } as *const u8)
                                    } != 0 {
                                1
                            } else { 0 } as u8
                };
                unsafe {
                    (*p_bt_1).incr_vacuum =
                        if unsafe {
                                        sqlite3_get4byte(unsafe {
                                                    &raw mut *page1.offset((36 + 7 * 4) as isize)
                                                } as *const u8)
                                    } != 0 {
                                1
                            } else { 0 } as u8
                };
            }
            unsafe {
                (*p_bt_1).max_local =
                    ((unsafe { (*p_bt_1).usable_size } - 12 as u32) * 64 as u32
                                / 255 as u32 - 23 as u32) as u16
            };
            unsafe {
                (*p_bt_1).min_local =
                    ((unsafe { (*p_bt_1).usable_size } - 12 as u32) * 32 as u32
                                / 255 as u32 - 23 as u32) as u16
            };
            unsafe {
                (*p_bt_1).max_leaf =
                    (unsafe { (*p_bt_1).usable_size } - 35 as u32) as u16
            };
            unsafe {
                (*p_bt_1).min_leaf =
                    ((unsafe { (*p_bt_1).usable_size } - 12 as u32) * 32 as u32
                                / 255 as u32 - 23 as u32) as u16
            };
            if unsafe { (*p_bt_1).max_local } as i32 > 127 {
                unsafe { (*p_bt_1).max1byte_payload = 127 as u8 };
            } else {
                unsafe {
                    (*p_bt_1).max1byte_payload =
                        unsafe { (*p_bt_1).max_local } as u8
                };
            }
            { let _ = 0; };
            unsafe { (*p_bt_1).p_page1 = p_page1 };
            unsafe { (*p_bt_1).n_page = n_page };
            return 0;
            break '__c19;
        }
        if !(false) { break '__b19; }
    }
    release_page_one(unsafe { &*p_page1 });
    unsafe { (*p_bt_1).p_page1 = core::ptr::null_mut() };
    return rc;
}
extern "C" fn zero_page(p_page_1: *mut MemPage, flags: i32) -> () {
    let data: *mut u8 = unsafe { (*p_page_1).a_data };
    let p_bt: *const BtShared =
        unsafe { (*p_page_1).p_bt } as *const BtShared;
    let hdr: i32 = unsafe { (*p_page_1).hdr_offset } as i32;
    let mut first: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_bt).bts_flags } as i32 & 12 != 0 {
        unsafe {
            memset(unsafe { &raw mut *data.offset(hdr as isize) } as *mut (),
                0, (unsafe { (*p_bt).usable_size } - hdr as u32) as u64)
        };
    }
    unsafe { *data.offset(hdr as isize) = flags as i8 as u8 };
    first = hdr + if flags & 8 == 0 { 12 } else { 8 };
    unsafe {
        memset(unsafe { &raw mut *data.offset((hdr + 1) as isize) } as
                *mut (), 0, 4 as u64)
    };
    unsafe { *data.offset((hdr + 7) as isize) = 0 as u8 };
    {
        unsafe {
            *unsafe { data.offset((hdr + 5) as isize).offset(0 as isize) } =
                (unsafe { (*p_bt).usable_size } >> 8) as u8
        };
        unsafe {
            *unsafe { data.offset((hdr + 5) as isize).offset(1 as isize) } =
                unsafe { (*p_bt).usable_size } as u8
        }
    };
    unsafe {
        (*p_page_1).n_free =
            (unsafe { (*p_bt).usable_size } - first as u32) as u16 as i32
    };
    decode_flags(unsafe { &mut *p_page_1 }, flags);
    unsafe { (*p_page_1).cell_offset = first as u16 };
    unsafe {
        (*p_page_1).a_data_end =
            unsafe { data.add(unsafe { (*p_bt).page_size } as usize) }
    };
    unsafe {
        (*p_page_1).a_cell_idx = unsafe { data.offset(first as isize) }
    };
    unsafe {
        (*p_page_1).a_data_ofst =
            unsafe {
                data.add(unsafe { (*p_page_1).child_ptr_size } as usize)
            }
    };
    unsafe { (*p_page_1).n_overflow = 0 as u8 };
    { let _ = 0; };
    unsafe {
        (*p_page_1).mask_page =
            (unsafe { (*p_bt).page_size } - 1 as u32) as u16
    };
    unsafe { (*p_page_1).n_cell = 0 as u16 };
    unsafe { (*p_page_1).is_init = 1 as u8 };
}
extern "C" fn new_database(p_bt_1: &mut BtShared) -> i32 {
    let mut p_p1: *mut MemPage = core::ptr::null_mut();
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = 0; };
    if (*p_bt_1).n_page > 0 as u32 { return 0; }
    p_p1 = (*p_bt_1).p_page1;
    { let _ = 0; };
    data = unsafe { (*p_p1).a_data };
    rc = unsafe { sqlite3_pager_write(unsafe { (*p_p1).p_db_page }) };
    if rc != 0 { return rc; }
    unsafe {
        memcpy(data as *mut (),
            &raw const z_magic_header[0 as usize] as *const i8 as *const (),
            core::mem::size_of::<[i8; 16]>() as u64)
    };
    { let _ = 0; };
    unsafe {
        *data.offset(16 as isize) =
            ((*p_bt_1).page_size >> 8 & 255 as u32) as u8
    };
    unsafe {
        *data.offset(17 as isize) =
            ((*p_bt_1).page_size >> 16 & 255 as u32) as u8
    };
    unsafe { *data.offset(18 as isize) = 1 as u8 };
    unsafe { *data.offset(19 as isize) = 1 as u8 };
    { let _ = 0; };
    unsafe {
        *data.offset(20 as isize) =
            ((*p_bt_1).page_size - (*p_bt_1).usable_size) as u8
    };
    unsafe { *data.offset(21 as isize) = 64 as u8 };
    unsafe { *data.offset(22 as isize) = 32 as u8 };
    unsafe { *data.offset(23 as isize) = 32 as u8 };
    unsafe {
        memset(unsafe { &raw mut *data.offset(24 as isize) } as *mut (), 0,
            (100 - 24) as u64)
    };
    zero_page(p_p1, 1 | 8 | 4);
    (*p_bt_1).bts_flags |= 2 as u16;
    { let _ = 0; };
    { let _ = 0; };
    unsafe {
        sqlite3_put4byte(unsafe { &mut *data.offset((36 + 4 * 4) as isize) },
            (*p_bt_1).auto_vacuum as u32)
    };
    unsafe {
        sqlite3_put4byte(unsafe { &mut *data.offset((36 + 7 * 4) as isize) },
            (*p_bt_1).incr_vacuum as u32)
    };
    (*p_bt_1).n_page = 1 as u32;
    unsafe { *data.offset(31 as isize) = 1 as u8 };
    return 0;
}
extern "C" fn btree_begin_trans(p: *mut Btree, wrflag: i32,
    p_schema_version_1: *mut i32) -> i32 {
    let mut p_bt: *mut BtShared = core::ptr::null_mut();
    let mut p_pager: *mut Pager = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_block: *const sqlite3 = core::ptr::null();
    let mut p_iter: *const BtLock = core::ptr::null();
    let mut p_page1: *const MemPage = core::ptr::null();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s21:
            {
            match __state {
                0 => { p_bt = unsafe { (*p).p_bt }; __state = 3; }
                2 => { if rc == 0 { __state = 78; } else { __state = 77; } }
                3 => { p_pager = unsafe { (*p_bt).p_pager }; __state = 4; }
                4 => { rc = 0; __state = 5; }
                5 => { unsafe { sqlite3_btree_enter(p) }; __state = 6; }
                6 => { { let _ = 0; }; __state = 7; }
                7 => { { let _ = 0; }; __state = 8; }
                8 => { __state = 9; }
                9 => {
                    if unsafe { (*p).in_trans } as i32 == 2 ||
                            unsafe { (*p).in_trans } as i32 == 1 &&
                                (wrflag == 0) as i32 != 0 {
                        __state = 11;
                    } else { __state = 10; }
                }
                10 => { { let _ = 0; }; __state = 12; }
                11 => { __state = 2; }
                12 => {
                    if unsafe { (*unsafe { (*p).db }).flags } & 33554432 as u64
                                != 0 &&
                            unsafe { sqlite3_pager_isreadonly(p_pager) } as i32 == 0 {
                        __state = 14;
                    } else { __state = 13; }
                }
                13 => {
                    if unsafe { (*p_bt).bts_flags } as i32 & 1 != 0 &&
                            wrflag != 0 {
                        __state = 16;
                    } else { __state = 15; }
                }
                14 => {
                    unsafe { (*p_bt).bts_flags &= !1 as u16 };
                    __state = 13;
                }
                15 => { p_block = core::ptr::null(); __state = 19; }
                16 => { rc = 8; __state = 17; }
                17 => { __state = 2; }
                18 => {
                    rc = query_shared_cache_table_lock(p, 1 as Pgno, 1 as u8);
                    __state = 33;
                }
                19 => {
                    if wrflag != 0 &&
                                unsafe { (*p_bt).in_transaction } as i32 == 2 ||
                            unsafe { (*p_bt).bts_flags } as i32 & 128 != 0 {
                        __state = 21;
                    } else { __state = 22; }
                }
                20 => {
                    if !(p_block).is_null() {
                        __state = 30;
                    } else { __state = 18; }
                }
                21 => {
                    p_block = unsafe { (*unsafe { (*p_bt).p_writer }).db };
                    __state = 20;
                }
                22 => {
                    if wrflag > 1 { __state = 23; } else { __state = 20; }
                }
                23 => { __state = 24; }
                24 => { p_iter = unsafe { (*p_bt).p_lock }; __state = 25; }
                25 => {
                    if !(p_iter).is_null() {
                        __state = 26;
                    } else { __state = 20; }
                }
                26 => {
                    if unsafe { (*p_iter).p_btree } != p {
                        __state = 28;
                    } else { __state = 27; }
                }
                27 => { p_iter = unsafe { (*p_iter).p_next }; __state = 25; }
                28 => {
                    p_block = unsafe { (*unsafe { (*p_iter).p_btree }).db };
                    __state = 29;
                }
                29 => { __state = 20; }
                30 => { __state = 31; }
                31 => { rc = 6 | 1 << 8; __state = 32; }
                32 => { __state = 2; }
                33 => { if 0 != rc { __state = 35; } else { __state = 34; } }
                34 => {
                    unsafe { (*p_bt).bts_flags &= !16 as u16 };
                    __state = 36;
                }
                35 => { __state = 2; }
                36 => {
                    if unsafe { (*p_bt).n_page } == 0 as u32 {
                        __state = 38;
                    } else { __state = 37; }
                }
                37 => { __state = 41; }
                38 => {
                    unsafe { (*p_bt).bts_flags |= 16 as u16 };
                    __state = 37;
                }
                39 => { __state = 54; }
                40 => {
                    if rc & 255 == 5 &&
                                unsafe { (*p_bt).in_transaction } as i32 == 0 &&
                            btree_invoke_busy_handler(p_bt as *mut ()) != 0 {
                        __state = 37;
                    } else { __state = 39; }
                }
                41 => {
                    if unsafe { (*p_bt).p_page1 } == core::ptr::null_mut() &&
                            0 == { rc = lock_btree(p_bt); rc } {
                        __state = 43;
                    } else { __state = 42; }
                }
                42 => {
                    if rc == 0 && wrflag != 0 {
                        __state = 45;
                    } else { __state = 44; }
                }
                43 => { __state = 41; }
                44 => { if rc != 0 { __state = 52; } else { __state = 40; } }
                45 => {
                    if unsafe { (*p_bt).bts_flags } as i32 & 1 != 0 {
                        __state = 46;
                    } else { __state = 47; }
                }
                46 => { rc = 8; __state = 44; }
                47 => {
                    rc =
                        unsafe {
                            sqlite3_pager_begin(p_pager, (wrflag > 1) as i32,
                                unsafe {
                                    sqlite3_temp_in_memory(unsafe { (*p).db } as *const sqlite3)
                                })
                        };
                    __state = 48;
                }
                48 => { if rc == 0 { __state = 49; } else { __state = 50; } }
                49 => {
                    rc = new_database(unsafe { &mut *p_bt });
                    __state = 44;
                }
                50 => {
                    if rc == 5 | 2 << 8 &&
                            unsafe { (*p_bt).in_transaction } as i32 == 0 {
                        __state = 51;
                    } else { __state = 44; }
                }
                51 => { rc = 5; __state = 44; }
                52 => { { let _ = 0; }; __state = 53; }
                53 => {
                    unlock_btree_if_unused(unsafe { &mut *p_bt });
                    __state = 40;
                }
                54 => { if rc == 0 { __state = 56; } else { __state = 55; } }
                55 => { __state = 2; }
                56 => {
                    if unsafe { (*p).in_trans } as i32 == 0 {
                        __state = 58;
                    } else { __state = 57; }
                }
                57 => {
                    unsafe {
                        (*p).in_trans = if wrflag != 0 { 2 } else { 1 } as u8
                    };
                    __state = 64;
                }
                58 => {
                    {
                        let __p = unsafe { &mut (*p_bt).n_transaction };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 59;
                }
                59 => {
                    if unsafe { (*p).sharable } != 0 {
                        __state = 60;
                    } else { __state = 57; }
                }
                60 => { { let _ = 0; }; __state = 61; }
                61 => { unsafe { (*p).lock.e_lock = 1 as u8 }; __state = 62; }
                62 => {
                    unsafe { (*p).lock.p_next = unsafe { (*p_bt).p_lock } };
                    __state = 63;
                }
                63 => {
                    unsafe { (*p_bt).p_lock = unsafe { &mut (*p).lock } };
                    __state = 57;
                }
                64 => {
                    if unsafe { (*p).in_trans } as i32 >
                            unsafe { (*p_bt).in_transaction } as i32 {
                        __state = 66;
                    } else { __state = 65; }
                }
                65 => {
                    if wrflag != 0 { __state = 67; } else { __state = 55; }
                }
                66 => {
                    unsafe {
                        (*p_bt).in_transaction = unsafe { (*p).in_trans }
                    };
                    __state = 65;
                }
                67 => {
                    p_page1 = unsafe { (*p_bt).p_page1 } as *const MemPage;
                    __state = 68;
                }
                68 => { { let _ = 0; }; __state = 69; }
                69 => { unsafe { (*p_bt).p_writer = p }; __state = 70; }
                70 => {
                    unsafe { (*p_bt).bts_flags &= !64 as u16 };
                    __state = 71;
                }
                71 => {
                    if wrflag > 1 { __state = 73; } else { __state = 72; }
                }
                72 => {
                    if unsafe { (*p_bt).n_page } !=
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe { (*p_page1).a_data.offset(28 as isize) }
                                        } as *const u8)
                            } {
                        __state = 74;
                    } else { __state = 55; }
                }
                73 => {
                    unsafe { (*p_bt).bts_flags |= 64 as u16 };
                    __state = 72;
                }
                74 => {
                    rc =
                        unsafe {
                            sqlite3_pager_write(unsafe { (*p_page1).p_db_page })
                        };
                    __state = 75;
                }
                75 => { if rc == 0 { __state = 76; } else { __state = 55; } }
                76 => {
                    unsafe {
                        sqlite3_put4byte(unsafe {
                                &mut *unsafe { (*p_page1).a_data.offset(28 as isize) }
                            }, unsafe { (*p_bt).n_page })
                    };
                    __state = 55;
                }
                77 => { { let _ = 0; }; __state = 82; }
                78 => {
                    if !(p_schema_version_1).is_null() {
                        __state = 80;
                    } else { __state = 79; }
                }
                79 => {
                    if wrflag != 0 { __state = 81; } else { __state = 77; }
                }
                80 => {
                    unsafe {
                        *p_schema_version_1 =
                            unsafe {
                                    sqlite3_get4byte(unsafe {
                                                &raw mut *unsafe {
                                                            (*unsafe { (*p_bt).p_page1 }).a_data.offset(40 as isize)
                                                        }
                                            } as *const u8)
                                } as i32
                    };
                    __state = 79;
                }
                81 => {
                    rc =
                        unsafe {
                            sqlite3_pager_open_savepoint(p_pager,
                                unsafe { (*unsafe { (*p).db }).n_savepoint })
                        };
                    __state = 77;
                }
                82 => { { let _ = 0; }; __state = 83; }
                83 => { __state = 84; }
                84 => { unsafe { sqlite3_btree_leave(p) }; __state = 85; }
                85 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_begin_trans(p: *mut Btree, wrflag: i32,
    p_schema_version_1: *mut i32) -> i32 {
    let mut p_bt: *const BtShared = core::ptr::null();
    if unsafe { (*p).sharable } != 0 || unsafe { (*p).in_trans } as i32 == 0
            || unsafe { (*p).in_trans } as i32 == 1 && wrflag != 0 {
        return btree_begin_trans(p, wrflag, p_schema_version_1);
    }
    p_bt = unsafe { (*p).p_bt };
    if !(p_schema_version_1).is_null() {
        unsafe {
            *p_schema_version_1 =
                unsafe {
                        sqlite3_get4byte(unsafe {
                                    &raw mut *unsafe {
                                                (*unsafe { (*p_bt).p_page1 }).a_data.offset(40 as isize)
                                            }
                                } as *const u8)
                    } as i32
        };
    }
    if wrflag != 0 {
        return unsafe {
                sqlite3_pager_open_savepoint(unsafe { (*p_bt).p_pager },
                    unsafe { (*unsafe { (*p).db }).n_savepoint })
            };
    } else { return 0; }
}
extern "C" fn invalidate_all_overflow_cache(p_bt_1: &BtShared) -> () {
    let mut p: *mut BtCursor = core::ptr::null_mut();
    { let _ = 0; };
    {
        p = (*p_bt_1).p_cursor;
        '__b22: loop {
            if !(!(p).is_null()) { break '__b22; }
            '__c22: loop {
                unsafe { (*p).cur_flags &= !4 as u8 };
                break '__c22;
            }
            p = unsafe { (*p).p_next };
        }
    }
}
extern "C" fn final_db_size(p_bt_1: *mut BtShared, n_orig_1: Pgno,
    n_free_1: Pgno) -> Pgno {
    unsafe {
        let mut n_entry: i32 = 0;
        let mut n_ptrmap: Pgno = 0 as Pgno;
        let mut n_fin: Pgno = 0 as Pgno;
        n_entry = (unsafe { (*p_bt_1).usable_size } / 5 as u32) as i32;
        n_ptrmap =
            (n_free_1 - n_orig_1 +
                        ptrmap_pageno(unsafe { &*p_bt_1 }, n_orig_1) +
                    n_entry as Pgno) / n_entry as Pgno;
        n_fin = n_orig_1 - n_free_1 - n_ptrmap;
        if n_orig_1 >
                    (sqlite3_pending_byte as u32 /
                                unsafe { (*p_bt_1).page_size } + 1 as u32) as Pgno &&
                n_fin <
                    (sqlite3_pending_byte as u32 /
                                unsafe { (*p_bt_1).page_size } + 1 as u32) as Pgno {
            { let __p = &mut n_fin; let __t = *__p; *__p -= 1; __t };
        }
        while ptrmap_pageno(unsafe { &*p_bt_1 }, n_fin) == n_fin ||
                n_fin ==
                    (sqlite3_pending_byte as u32 /
                                unsafe { (*p_bt_1).page_size } + 1 as u32) as Pgno {
            { let __p = &mut n_fin; let __t = *__p; *__p -= 1; __t };
        }
        return n_fin;
    }
}
extern "C" fn btree_get_unused_page(p_bt_1: *mut BtShared, pgno: Pgno,
    pp_page_1: *mut *mut MemPage, flags: i32) -> i32 {
    let rc: i32 =
        btree_get_page(p_bt_1, pgno, unsafe { &mut *pp_page_1 }, flags);
    if rc == 0 {
        if unsafe {
                    sqlite3_pager_page_refcount(unsafe {
                            (*unsafe { *pp_page_1 }).p_db_page
                        })
                } > 1 {
            release_page(unsafe { *pp_page_1 });
            unsafe { *pp_page_1 = core::ptr::null_mut() };
            return unsafe { sqlite3_corrupt_error(2494) };
        }
        unsafe { (*unsafe { *pp_page_1 }).is_init = 0 as u8 };
    } else { unsafe { *pp_page_1 = core::ptr::null_mut() }; }
    return rc;
}
extern "C" fn btree_get_has_content(p_bt_1: &BtShared, pgno: Pgno) -> i32 {
    let p: *mut Bitvec = (*p_bt_1).p_has_content;
    return (!(p).is_null() &&
                (pgno > unsafe { sqlite3_bitvec_size(p) } ||
                    unsafe { sqlite3_bitvec_test_not_null(p, pgno) } != 0)) as
            i32;
}
extern "C" fn allocate_btree_page(p_bt_1: *mut BtShared,
    pp_page_1: *mut *mut MemPage, p_pgno_1: *mut Pgno, nearby: Pgno,
    e_mode_1: u8) -> i32 {
    unsafe {
        let mut p_page1: *const MemPage = core::ptr::null();
        let mut rc: i32 = 0;
        let mut n: u32 = 0 as u32;
        let mut k: u32 = 0 as u32;
        let mut p_trunk: *mut MemPage = core::ptr::null_mut();
        let mut p_prev_trunk: *mut MemPage = core::ptr::null_mut();
        let mut mx_page: Pgno = 0 as Pgno;
        let mut i_trunk: Pgno = 0 as Pgno;
        let mut search_list: u8 = 0 as u8;
        let mut n_search: u32 = 0 as u32;
        let mut e_type: u8 = 0 as u8;
        let mut p_new_trunk: *mut MemPage = core::ptr::null_mut();
        let mut i_new_trunk: Pgno = 0 as Pgno;
        let mut closest: u32 = 0 as u32;
        let mut i_page: Pgno = 0 as Pgno;
        let mut a_data: *mut u8 = core::ptr::null_mut();
        let mut i: u32 = 0 as u32;
        let mut dist: i32 = 0;
        let mut d2: i32 = 0;
        let mut no_content: i32 = 0;
        let mut b_no_content: i32 = 0;
        let mut p_pg: *mut MemPage = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s25:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => { release_page(p_trunk); __state = 189; }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { p_trunk = core::ptr::null_mut(); __state = 7; }
                    7 => { p_prev_trunk = core::ptr::null_mut(); __state = 8; }
                    8 => { __state = 9; }
                    9 => { { let _ = 0; }; __state = 10; }
                    10 => { { let _ = 0; }; __state = 11; }
                    11 => {
                        p_page1 = unsafe { (*p_bt_1).p_page1 };
                        __state = 12;
                    }
                    12 => {
                        mx_page = btree_pagecount(unsafe { &*p_bt_1 });
                        __state = 13;
                    }
                    13 => {
                        n =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe { (*p_page1).a_data.offset(36 as isize) }
                                        } as *const u8)
                            };
                        __state = 14;
                    }
                    14 => { __state = 15; }
                    15 => {
                        if n >= mx_page { __state = 17; } else { __state = 16; }
                    }
                    16 => {
                        if n > 0 as u32 { __state = 19; } else { __state = 20; }
                    }
                    17 => { return unsafe { sqlite3_corrupt_error(6570) }; }
                    18 => { { let _ = 0; }; __state = 188; }
                    19 => { __state = 21; }
                    20 => {
                        b_no_content =
                            if 0 == unsafe { (*p_bt_1).b_do_truncate } as i32 {
                                1
                            } else { 0 };
                        __state = 158;
                    }
                    21 => { search_list = 0 as u8; __state = 22; }
                    22 => { n_search = 0 as u32; __state = 23; }
                    23 => {
                        if e_mode_1 as i32 == 1 {
                            __state = 25;
                        } else { __state = 26; }
                    }
                    24 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_page1).p_db_page })
                            };
                        __state = 36;
                    }
                    25 => {
                        if nearby <= mx_page {
                            __state = 27;
                        } else { __state = 24; }
                    }
                    26 => {
                        if e_mode_1 as i32 == 2 {
                            __state = 35;
                        } else { __state = 24; }
                    }
                    27 => { __state = 28; }
                    28 => { { let _ = 0; }; __state = 29; }
                    29 => { { let _ = 0; }; __state = 30; }
                    30 => {
                        rc =
                            ptrmap_get(p_bt_1, nearby, &mut e_type,
                                core::ptr::null_mut());
                        __state = 31;
                    }
                    31 => {
                        if rc != 0 { __state = 33; } else { __state = 32; }
                    }
                    32 => {
                        if e_type as i32 == 2 {
                            __state = 34;
                        } else { __state = 24; }
                    }
                    33 => { return rc; }
                    34 => { search_list = 1 as u8; __state = 24; }
                    35 => { search_list = 1 as u8; __state = 24; }
                    36 => {
                        if rc != 0 { __state = 38; } else { __state = 37; }
                    }
                    37 => {
                        unsafe {
                            sqlite3_put4byte(unsafe {
                                    &mut *unsafe { (*p_page1).a_data.offset(36 as isize) }
                                }, n - 1 as u32)
                        };
                        __state = 39;
                    }
                    38 => { return rc; }
                    39 => { p_prev_trunk = p_trunk; __state = 41; }
                    40 => {
                        if search_list != 0 { __state = 39; } else { __state = 18; }
                    }
                    41 => {
                        if !(p_prev_trunk).is_null() {
                            __state = 43;
                        } else { __state = 44; }
                    }
                    42 => { __state = 45; }
                    43 => {
                        i_trunk =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe {
                                                        (*p_prev_trunk).a_data.offset(0 as isize)
                                                    }
                                        } as *const u8)
                            };
                        __state = 42;
                    }
                    44 => {
                        i_trunk =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe { (*p_page1).a_data.offset(32 as isize) }
                                        } as *const u8)
                            };
                        __state = 42;
                    }
                    45 => {
                        if i_trunk > mx_page ||
                                { let __p = &mut n_search; let __t = *__p; *__p += 1; __t }
                                    > n {
                            __state = 47;
                        } else { __state = 48; }
                    }
                    46 => {
                        if rc != 0 { __state = 50; } else { __state = 49; }
                    }
                    47 => {
                        rc = unsafe { sqlite3_corrupt_error(6626) };
                        __state = 46;
                    }
                    48 => {
                        rc =
                            btree_get_unused_page(p_bt_1, i_trunk, &mut p_trunk, 0);
                        __state = 46;
                    }
                    49 => { { let _ = 0; }; __state = 52; }
                    50 => { p_trunk = core::ptr::null_mut(); __state = 51; }
                    51 => { __state = 2; }
                    52 => { { let _ = 0; }; __state = 53; }
                    53 => {
                        k =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe { (*p_trunk).a_data.offset(4 as isize) }
                                        } as *const u8)
                            };
                        __state = 54;
                    }
                    54 => {
                        if k == 0 as u32 && (search_list == 0) as i32 != 0 {
                            __state = 56;
                        } else { __state = 57; }
                    }
                    55 => { release_page(p_prev_trunk); __state = 157; }
                    56 => { { let _ = 0; }; __state = 58; }
                    57 => {
                        if k >
                                (unsafe { (*p_bt_1).usable_size } / 4 as u32 - 2 as u32) as
                                    u32 {
                            __state = 66;
                        } else { __state = 67; }
                    }
                    58 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_trunk).p_db_page })
                            };
                        __state = 59;
                    }
                    59 => {
                        if rc != 0 { __state = 61; } else { __state = 60; }
                    }
                    60 => { unsafe { *p_pgno_1 = i_trunk }; __state = 62; }
                    61 => { __state = 2; }
                    62 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe { (*p_page1).a_data.offset(32 as isize) }
                                    } as *mut (),
                                unsafe {
                                        &raw mut *unsafe { (*p_trunk).a_data.offset(0 as isize) }
                                    } as *const (), 4 as u64)
                        };
                        __state = 63;
                    }
                    63 => { unsafe { *pp_page_1 = p_trunk }; __state = 64; }
                    64 => { p_trunk = core::ptr::null_mut(); __state = 65; }
                    65 => { __state = 55; }
                    66 => {
                        rc = unsafe { sqlite3_corrupt_error(6655) };
                        __state = 68;
                    }
                    67 => {
                        if search_list != 0 &&
                                (nearby == i_trunk ||
                                    i_trunk < nearby && e_mode_1 as i32 == 2) {
                            __state = 69;
                        } else { __state = 70; }
                    }
                    68 => { __state = 2; }
                    69 => { unsafe { *p_pgno_1 = i_trunk }; __state = 71; }
                    70 => {
                        if k > 0 as u32 { __state = 109; } else { __state = 55; }
                    }
                    71 => { unsafe { *pp_page_1 = p_trunk }; __state = 72; }
                    72 => { search_list = 0 as u8; __state = 73; }
                    73 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_trunk).p_db_page })
                            };
                        __state = 74;
                    }
                    74 => {
                        if rc != 0 { __state = 76; } else { __state = 75; }
                    }
                    75 => {
                        if k == 0 as u32 { __state = 78; } else { __state = 79; }
                    }
                    76 => { __state = 2; }
                    77 => { p_trunk = core::ptr::null_mut(); __state = 108; }
                    78 => {
                        if (p_prev_trunk).is_null() as i32 != 0 {
                            __state = 80;
                        } else { __state = 81; }
                    }
                    79 => { __state = 85; }
                    80 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe { (*p_page1).a_data.offset(32 as isize) }
                                    } as *mut (),
                                unsafe {
                                        &raw mut *unsafe { (*p_trunk).a_data.offset(0 as isize) }
                                    } as *const (), 4 as u64)
                        };
                        __state = 77;
                    }
                    81 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_prev_trunk).p_db_page })
                            };
                        __state = 82;
                    }
                    82 => {
                        if rc != 0 { __state = 84; } else { __state = 83; }
                    }
                    83 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_prev_trunk).a_data.offset(0 as isize)
                                                }
                                    } as *mut (),
                                unsafe {
                                        &raw mut *unsafe { (*p_trunk).a_data.offset(0 as isize) }
                                    } as *const (), 4 as u64)
                        };
                        __state = 77;
                    }
                    84 => { __state = 2; }
                    85 => {
                        i_new_trunk =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe { (*p_trunk).a_data.offset(8 as isize) }
                                        } as *const u8)
                            };
                        __state = 86;
                    }
                    86 => {
                        if i_new_trunk > mx_page {
                            __state = 88;
                        } else { __state = 87; }
                    }
                    87 => { __state = 90; }
                    88 => {
                        rc = unsafe { sqlite3_corrupt_error(6689) };
                        __state = 89;
                    }
                    89 => { __state = 2; }
                    90 => {
                        rc =
                            btree_get_unused_page(p_bt_1, i_new_trunk, &mut p_new_trunk,
                                0);
                        __state = 91;
                    }
                    91 => {
                        if rc != 0 { __state = 93; } else { __state = 92; }
                    }
                    92 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_new_trunk).p_db_page })
                            };
                        __state = 94;
                    }
                    93 => { __state = 2; }
                    94 => {
                        if rc != 0 { __state = 96; } else { __state = 95; }
                    }
                    95 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_new_trunk).a_data.offset(0 as isize)
                                                }
                                    } as *mut (),
                                unsafe {
                                        &raw mut *unsafe { (*p_trunk).a_data.offset(0 as isize) }
                                    } as *const (), 4 as u64)
                        };
                        __state = 98;
                    }
                    96 => { release_page(p_new_trunk); __state = 97; }
                    97 => { __state = 2; }
                    98 => {
                        unsafe {
                            sqlite3_put4byte(unsafe {
                                    &mut *unsafe { (*p_new_trunk).a_data.offset(4 as isize) }
                                }, k - 1 as u32)
                        };
                        __state = 99;
                    }
                    99 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_new_trunk).a_data.offset(8 as isize)
                                                }
                                    } as *mut (),
                                unsafe {
                                        &raw mut *unsafe { (*p_trunk).a_data.offset(12 as isize) }
                                    } as *const (), ((k - 1 as u32) * 4 as u32) as u64)
                        };
                        __state = 100;
                    }
                    100 => { release_page(p_new_trunk); __state = 101; }
                    101 => {
                        if (p_prev_trunk).is_null() as i32 != 0 {
                            __state = 102;
                        } else { __state = 103; }
                    }
                    102 => { { let _ = 0; }; __state = 104; }
                    103 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_prev_trunk).p_db_page })
                            };
                        __state = 105;
                    }
                    104 => {
                        unsafe {
                            sqlite3_put4byte(unsafe {
                                    &mut *unsafe { (*p_page1).a_data.offset(32 as isize) }
                                }, i_new_trunk)
                        };
                        __state = 77;
                    }
                    105 => {
                        if rc != 0 { __state = 107; } else { __state = 106; }
                    }
                    106 => {
                        unsafe {
                            sqlite3_put4byte(unsafe {
                                    &mut *unsafe { (*p_prev_trunk).a_data.offset(0 as isize) }
                                }, i_new_trunk)
                        };
                        __state = 77;
                    }
                    107 => { __state = 2; }
                    108 => { __state = 55; }
                    109 => { __state = 110; }
                    110 => { __state = 111; }
                    111 => {
                        a_data = unsafe { (*p_trunk).a_data };
                        __state = 112;
                    }
                    112 => {
                        if nearby > 0 as u32 {
                            __state = 114;
                        } else { __state = 115; }
                    }
                    113 => {
                        i_page =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *a_data.add((8 as u32 + closest * 4 as u32) as
                                                            usize)
                                        } as *const u8)
                            };
                        __state = 134;
                    }
                    114 => { __state = 116; }
                    115 => { closest = 0 as u32; __state = 113; }
                    116 => { closest = 0 as u32; __state = 117; }
                    117 => {
                        if e_mode_1 as i32 == 2 {
                            __state = 118;
                        } else { __state = 119; }
                    }
                    118 => { i = 0 as u32; __state = 120; }
                    119 => { __state = 126; }
                    120 => {
                        if i < k { __state = 121; } else { __state = 113; }
                    }
                    121 => {
                        i_page =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *a_data.add((8 as u32 + i * 4 as u32) as usize)
                                        } as *const u8)
                            };
                        __state = 123;
                    }
                    122 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 120;
                    }
                    123 => {
                        if i_page <= nearby {
                            __state = 124;
                        } else { __state = 122; }
                    }
                    124 => { closest = i; __state = 125; }
                    125 => { __state = 113; }
                    126 => {
                        dist =
                            unsafe {
                                sqlite3_abs_int32((unsafe {
                                                sqlite3_get4byte(unsafe {
                                                            &raw mut *a_data.offset(8 as isize)
                                                        } as *const u8)
                                            } - nearby) as i32)
                            };
                        __state = 127;
                    }
                    127 => { i = 1 as u32; __state = 128; }
                    128 => {
                        if i < k { __state = 129; } else { __state = 113; }
                    }
                    129 => {
                        d2 =
                            unsafe {
                                sqlite3_abs_int32((unsafe {
                                                sqlite3_get4byte(unsafe {
                                                            &raw mut *a_data.add((8 as u32 + i * 4 as u32) as usize)
                                                        } as *const u8)
                                            } - nearby) as i32)
                            };
                        __state = 131;
                    }
                    130 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 128;
                    }
                    131 => {
                        if d2 < dist { __state = 132; } else { __state = 130; }
                    }
                    132 => { closest = i; __state = 133; }
                    133 => { dist = d2; __state = 130; }
                    134 => { __state = 135; }
                    135 => {
                        if i_page > mx_page || i_page < 2 as u32 {
                            __state = 137;
                        } else { __state = 136; }
                    }
                    136 => { __state = 139; }
                    137 => {
                        rc = unsafe { sqlite3_corrupt_error(6754) };
                        __state = 138;
                    }
                    138 => { __state = 2; }
                    139 => {
                        if (search_list == 0) as i32 != 0 ||
                                (i_page == nearby ||
                                    i_page < nearby && e_mode_1 as i32 == 2) {
                            __state = 140;
                        } else { __state = 55; }
                    }
                    140 => { __state = 141; }
                    141 => { unsafe { *p_pgno_1 = i_page }; __state = 142; }
                    142 => { __state = 143; }
                    143 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_trunk).p_db_page })
                            };
                        __state = 144;
                    }
                    144 => {
                        if rc != 0 { __state = 146; } else { __state = 145; }
                    }
                    145 => {
                        if closest < k - 1 as u32 {
                            __state = 148;
                        } else { __state = 147; }
                    }
                    146 => { __state = 2; }
                    147 => {
                        unsafe {
                            sqlite3_put4byte(unsafe { &mut *a_data.offset(4 as isize) },
                                k - 1 as u32)
                        };
                        __state = 149;
                    }
                    148 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_data.add((8 as u32 + closest * 4 as u32) as
                                                        usize)
                                    } as *mut (),
                                unsafe {
                                        &raw mut *a_data.add((4 as u32 + k * 4 as u32) as usize)
                                    } as *const (), 4 as u64)
                        };
                        __state = 147;
                    }
                    149 => {
                        no_content =
                            if (btree_get_has_content(unsafe { &*p_bt_1 },
                                                unsafe { *p_pgno_1 }) == 0) as i32 != 0 {
                                1
                            } else { 0 };
                        __state = 150;
                    }
                    150 => {
                        rc =
                            btree_get_unused_page(p_bt_1, unsafe { *p_pgno_1 },
                                pp_page_1, no_content);
                        __state = 151;
                    }
                    151 => {
                        if rc == 0 { __state = 153; } else { __state = 152; }
                    }
                    152 => { search_list = 0 as u8; __state = 55; }
                    153 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe {
                                        (*unsafe { *pp_page_1 }).p_db_page
                                    })
                            };
                        __state = 154;
                    }
                    154 => {
                        if rc != 0 { __state = 155; } else { __state = 152; }
                    }
                    155 => {
                        release_page(unsafe { *pp_page_1 });
                        __state = 156;
                    }
                    156 => {
                        unsafe { *pp_page_1 = core::ptr::null_mut() };
                        __state = 152;
                    }
                    157 => {
                        p_prev_trunk = core::ptr::null_mut();
                        __state = 40;
                    }
                    158 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe {
                                        (*unsafe { (*p_bt_1).p_page1 }).p_db_page
                                    })
                            };
                        __state = 159;
                    }
                    159 => {
                        if rc != 0 { __state = 161; } else { __state = 160; }
                    }
                    160 => {
                        {
                            let __p = unsafe { &mut (*p_bt_1).n_page };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 162;
                    }
                    161 => { return rc; }
                    162 => {
                        if unsafe { (*p_bt_1).n_page } ==
                                (sqlite3_pending_byte as u32 /
                                            unsafe { (*p_bt_1).page_size } + 1 as u32) as Pgno {
                            __state = 164;
                        } else { __state = 163; }
                    }
                    163 => {
                        if unsafe { (*p_bt_1).auto_vacuum } != 0 &&
                                ptrmap_pageno(unsafe { &*p_bt_1 },
                                        unsafe { (*p_bt_1).n_page }) == unsafe { (*p_bt_1).n_page }
                            {
                            __state = 166;
                        } else { __state = 165; }
                    }
                    164 => {
                        {
                            let __p = unsafe { &mut (*p_bt_1).n_page };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 163;
                    }
                    165 => {
                        unsafe {
                            sqlite3_put4byte(unsafe {
                                    (unsafe { (*unsafe { (*p_bt_1).p_page1 }).a_data } as
                                            *mut u8).offset(28 as isize)
                                }, unsafe { (*p_bt_1).n_page })
                        };
                        __state = 178;
                    }
                    166 => { p_pg = core::ptr::null_mut(); __state = 167; }
                    167 => { __state = 168; }
                    168 => { { let _ = 0; }; __state = 169; }
                    169 => {
                        rc =
                            btree_get_unused_page(p_bt_1, unsafe { (*p_bt_1).n_page },
                                &mut p_pg, b_no_content);
                        __state = 170;
                    }
                    170 => {
                        if rc == 0 { __state = 172; } else { __state = 171; }
                    }
                    171 => {
                        if rc != 0 { __state = 175; } else { __state = 174; }
                    }
                    172 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_pg).p_db_page })
                            };
                        __state = 173;
                    }
                    173 => { release_page(p_pg); __state = 171; }
                    174 => {
                        {
                            let __p = unsafe { &mut (*p_bt_1).n_page };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 176;
                    }
                    175 => { return rc; }
                    176 => {
                        if unsafe { (*p_bt_1).n_page } ==
                                (sqlite3_pending_byte as u32 /
                                            unsafe { (*p_bt_1).page_size } + 1 as u32) as Pgno {
                            __state = 177;
                        } else { __state = 165; }
                    }
                    177 => {
                        {
                            let __p = unsafe { &mut (*p_bt_1).n_page };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 165;
                    }
                    178 => {
                        unsafe { *p_pgno_1 = unsafe { (*p_bt_1).n_page } };
                        __state = 179;
                    }
                    179 => { { let _ = 0; }; __state = 180; }
                    180 => {
                        rc =
                            btree_get_unused_page(p_bt_1, unsafe { *p_pgno_1 },
                                pp_page_1, b_no_content);
                        __state = 181;
                    }
                    181 => {
                        if rc != 0 { __state = 183; } else { __state = 182; }
                    }
                    182 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe {
                                        (*unsafe { *pp_page_1 }).p_db_page
                                    })
                            };
                        __state = 184;
                    }
                    183 => { return rc; }
                    184 => {
                        if rc != 0 { __state = 186; } else { __state = 185; }
                    }
                    185 => { __state = 18; }
                    186 => {
                        release_page(unsafe { *pp_page_1 });
                        __state = 187;
                    }
                    187 => {
                        unsafe { *pp_page_1 = core::ptr::null_mut() };
                        __state = 185;
                    }
                    188 => { __state = 2; }
                    189 => { release_page(p_prev_trunk); __state = 190; }
                    190 => { { let _ = 0; }; __state = 191; }
                    191 => { { let _ = 0; }; __state = 192; }
                    192 => { return rc; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn ptrmap_put(p_bt_1: *mut BtShared, key: Pgno, e_type_1: u8,
    parent: Pgno, p_rc_1: &mut i32) -> () {
    let mut p_db_page: *mut DbPage = core::ptr::null_mut();
    '__b26: loop {
        '__c26: loop {
            let mut p_ptrmap: *mut u8 = core::ptr::null_mut();
            let mut i_ptrmap: Pgno = 0 as Pgno;
            let mut offset: i32 = 0;
            let mut rc: i32 = 0;
            if *p_rc_1 != 0 { return; }
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if key == 0 as u32 {
                *p_rc_1 = unsafe { sqlite3_corrupt_error(1102) };
                return;
            }
            i_ptrmap = ptrmap_pageno(unsafe { &*p_bt_1 }, key);
            rc =
                unsafe {
                    sqlite3_pager_get(unsafe { (*p_bt_1).p_pager }, i_ptrmap,
                        &mut p_db_page, 0)
                };
            if rc != 0 { *p_rc_1 = rc; return; }
            if unsafe {
                            *(unsafe { sqlite3_pager_get_extra(p_db_page) } as
                                        *mut i8).offset(0 as isize)
                        } as i32 != 0 {
                *p_rc_1 = unsafe { sqlite3_corrupt_error(1115) };
                break '__b26;
            }
            offset = (5 as Pgno * (key - i_ptrmap - 1 as Pgno)) as i32;
            if offset < 0 {
                *p_rc_1 = unsafe { sqlite3_corrupt_error(1120) };
                break '__b26;
            }
            { let _ = 0; };
            p_ptrmap =
                unsafe { sqlite3_pager_get_data(p_db_page) } as *mut u8;
            if e_type_1 as i32 !=
                        unsafe { *p_ptrmap.offset(offset as isize) } as i32 ||
                    unsafe {
                            sqlite3_get4byte(unsafe {
                                        &raw mut *p_ptrmap.offset((offset + 1) as isize)
                                    } as *const u8)
                        } != parent {
                *p_rc_1 =
                    { rc = unsafe { sqlite3_pager_write(p_db_page) }; rc };
                if rc == 0 {
                    unsafe { *p_ptrmap.offset(offset as isize) = e_type_1 };
                    unsafe {
                        sqlite3_put4byte(unsafe {
                                &mut *p_ptrmap.offset((offset + 1) as isize)
                            }, parent)
                    };
                }
            }
            break '__c26;
        }
        if !(false) { break '__b26; }
    }
    unsafe { sqlite3_pager_unref(p_db_page) };
}
extern "C" fn ptrmap_put_ovfl_ptr(p_page_1: *mut MemPage, p_src_1: &MemPage,
    p_cell_1: *mut u8, p_rc_1: *mut i32) -> () {
    let mut info: CellInfo = unsafe { core::mem::zeroed() };
    if unsafe { *p_rc_1 } != 0 { return; }
    { let _ = 0; };
    unsafe {
        (unsafe {
                (*p_page_1).x_parse_cell.unwrap()
            })(p_page_1, p_cell_1, &mut info)
    };
    if (info.n_local as u32) < info.n_payload {
        let mut ovfl: Pgno = 0 as Pgno;
        if (p_cell_1 as uptr) < (*p_src_1).a_data_end as uptr &&
                unsafe { p_cell_1.add(info.n_local as usize) } as uptr >
                    (*p_src_1).a_data_end as uptr {
            unsafe { *p_rc_1 = unsafe { sqlite3_corrupt_error(1618) } };
            return;
        }
        ovfl =
            unsafe {
                sqlite3_get4byte(unsafe {
                            &raw mut *p_cell_1.offset((info.n_size as i32 - 4) as isize)
                        } as *const u8)
            };
        ptrmap_put(unsafe { (*p_page_1).p_bt }, ovfl, 3 as u8,
            unsafe { (*p_page_1).pgno }, unsafe { &mut *p_rc_1 });
    }
}
extern "C" fn set_child_ptrmaps(p_page_1: *mut MemPage) -> i32 {
    let mut i: i32 = 0;
    let mut n_cell: i32 = 0;
    let mut rc: i32 = 0;
    let p_bt: *mut BtShared = unsafe { (*p_page_1).p_bt };
    let pgno: Pgno = unsafe { (*p_page_1).pgno };
    { let _ = 0; };
    rc =
        if unsafe { (*p_page_1).is_init } != 0 {
            0
        } else { btree_init_page(p_page_1) };
    if rc != 0 { return rc; }
    n_cell = unsafe { (*p_page_1).n_cell } as i32;
    {
        i = 0;
        '__b27: loop {
            if !(i < n_cell) { break '__b27; }
            '__c27: loop {
                let p_cell: *mut u8 =
                    unsafe {
                        unsafe {
                            (*p_page_1).a_data.offset((unsafe { (*p_page_1).mask_page }
                                            as i32 &
                                        ((unsafe {
                                                            *unsafe {
                                                                    unsafe {
                                                                        (*p_page_1).a_cell_idx.offset((2 * i) as
                                                                                    isize).offset(0 as isize)
                                                                    }
                                                                }
                                                        } as i32) << 8 |
                                            unsafe {
                                                    *unsafe {
                                                            unsafe {
                                                                (*p_page_1).a_cell_idx.offset((2 * i) as
                                                                            isize).offset(1 as isize)
                                                            }
                                                        }
                                                } as i32)) as isize)
                        }
                    };
                ptrmap_put_ovfl_ptr(p_page_1, unsafe { &*p_page_1 }, p_cell,
                    &mut rc);
                if (unsafe { (*p_page_1).leaf } == 0) as i32 != 0 {
                    let child_pgno: Pgno =
                        unsafe { sqlite3_get4byte(p_cell as *const u8) };
                    ptrmap_put(p_bt, child_pgno, 5 as u8, pgno, &mut rc);
                }
                break '__c27;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if (unsafe { (*p_page_1).leaf } == 0) as i32 != 0 {
        let child_pgno_1: Pgno =
            unsafe {
                sqlite3_get4byte(unsafe {
                            &raw mut *unsafe {
                                        (*p_page_1).a_data.offset((unsafe { (*p_page_1).hdr_offset }
                                                        as i32 + 8) as isize)
                                    }
                        } as *const u8)
            };
        ptrmap_put(p_bt, child_pgno_1, 5 as u8, pgno, &mut rc);
    }
    return rc;
}
extern "C" fn modify_page_pointer(p_page_1: *mut MemPage, i_from_1: Pgno,
    i_to_1: Pgno, e_type_1: u8) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    if e_type_1 as i32 == 4 {
        if unsafe {
                    sqlite3_get4byte(unsafe { (*p_page_1).a_data } as *const u8)
                } != i_from_1 {
            return unsafe { sqlite3_corrupt_error(3916) };
        }
        unsafe { sqlite3_put4byte(unsafe { (*p_page_1).a_data }, i_to_1) };
    } else {
        let mut i: i32 = 0;
        let mut n_cell: i32 = 0;
        let mut rc: i32 = 0;
        rc =
            if unsafe { (*p_page_1).is_init } != 0 {
                0
            } else { btree_init_page(p_page_1) };
        if rc != 0 { return rc; }
        n_cell = unsafe { (*p_page_1).n_cell } as i32;
        {
            i = 0;
            '__b28: loop {
                if !(i < n_cell) { break '__b28; }
                '__c28: loop {
                    let p_cell: *mut u8 =
                        unsafe {
                            unsafe {
                                (*p_page_1).a_data.offset((unsafe { (*p_page_1).mask_page }
                                                as i32 &
                                            ((unsafe {
                                                                *unsafe {
                                                                        unsafe {
                                                                            (*p_page_1).a_cell_idx.offset((2 * i) as
                                                                                        isize).offset(0 as isize)
                                                                        }
                                                                    }
                                                            } as i32) << 8 |
                                                unsafe {
                                                        *unsafe {
                                                                unsafe {
                                                                    (*p_page_1).a_cell_idx.offset((2 * i) as
                                                                                isize).offset(1 as isize)
                                                                }
                                                            }
                                                    } as i32)) as isize)
                            }
                        };
                    if e_type_1 as i32 == 3 {
                        let mut info: CellInfo = unsafe { core::mem::zeroed() };
                        unsafe {
                            (unsafe {
                                    (*p_page_1).x_parse_cell.unwrap()
                                })(p_page_1, p_cell, &mut info)
                        };
                        if (info.n_local as u32) < info.n_payload {
                            if unsafe { p_cell.add(info.n_size as usize) } >
                                    unsafe {
                                        unsafe {
                                            (*p_page_1).a_data.add(unsafe {
                                                        (*unsafe { (*p_page_1).p_bt }).usable_size
                                                    } as usize)
                                        }
                                    } {
                                return unsafe { sqlite3_corrupt_error(3935) };
                            }
                            if i_from_1 ==
                                    unsafe {
                                        sqlite3_get4byte(unsafe {
                                                    unsafe {
                                                        p_cell.add(info.n_size as usize).offset(-(4 as isize))
                                                    }
                                                } as *const u8)
                                    } {
                                unsafe {
                                    sqlite3_put4byte(unsafe {
                                            unsafe {
                                                p_cell.add(info.n_size as usize).offset(-(4 as isize))
                                            }
                                        }, i_to_1)
                                };
                                break '__b28;
                            }
                        }
                    } else {
                        if unsafe { p_cell.offset(4 as isize) } >
                                unsafe {
                                    unsafe {
                                        (*p_page_1).a_data.add(unsafe {
                                                    (*unsafe { (*p_page_1).p_bt }).usable_size
                                                } as usize)
                                    }
                                } {
                            return unsafe { sqlite3_corrupt_error(3944) };
                        }
                        if unsafe { sqlite3_get4byte(p_cell as *const u8) } ==
                                i_from_1 {
                            unsafe { sqlite3_put4byte(p_cell, i_to_1) };
                            break '__b28;
                        }
                    }
                    break '__c28;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if i == n_cell {
            if e_type_1 as i32 != 5 ||
                    unsafe {
                            sqlite3_get4byte(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_page_1).a_data.offset((unsafe { (*p_page_1).hdr_offset }
                                                                    as i32 + 8) as isize)
                                                }
                                    } as *const u8)
                        } != i_from_1 {
                return unsafe { sqlite3_corrupt_error(3956) };
            }
            unsafe {
                sqlite3_put4byte(unsafe {
                        &mut *unsafe {
                                    (*p_page_1).a_data.offset((unsafe { (*p_page_1).hdr_offset }
                                                    as i32 + 8) as isize)
                                }
                    }, i_to_1)
            };
        }
    }
    return 0;
}
extern "C" fn relocate_page(p_bt_1: *mut BtShared, p_db_page_1: *mut MemPage,
    e_type_1: u8, i_ptr_page_1: Pgno, i_free_page_1: Pgno, is_commit_1: i32)
    -> i32 {
    let mut p_ptr_page: *mut MemPage = core::ptr::null_mut();
    let i_db_page: Pgno = unsafe { (*p_db_page_1).pgno };
    let p_pager: *mut Pager = unsafe { (*p_bt_1).p_pager };
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if i_db_page < 3 as u32 { return unsafe { sqlite3_corrupt_error(3991) }; }
    rc =
        unsafe {
            sqlite3_pager_movepage(p_pager,
                unsafe { (*p_db_page_1).p_db_page }, i_free_page_1,
                is_commit_1)
        };
    if rc != 0 { return rc; }
    unsafe { (*p_db_page_1).pgno = i_free_page_1 };
    if e_type_1 as i32 == 5 || e_type_1 as i32 == 1 {
        rc = set_child_ptrmaps(p_db_page_1);
        if rc != 0 { return rc; }
    } else {
        let next_ovfl: Pgno =
            unsafe {
                sqlite3_get4byte(unsafe { (*p_db_page_1).a_data } as
                        *const u8)
            };
        if next_ovfl != 0 as u32 {
            ptrmap_put(p_bt_1, next_ovfl, 4 as u8, i_free_page_1, &mut rc);
            if rc != 0 { return rc; }
        }
    }
    if e_type_1 as i32 != 1 {
        rc = btree_get_page(p_bt_1, i_ptr_page_1, &mut p_ptr_page, 0);
        if rc != 0 { return rc; }
        rc =
            unsafe {
                sqlite3_pager_write(unsafe { (*p_ptr_page).p_db_page })
            };
        if rc != 0 { release_page(p_ptr_page); return rc; }
        rc =
            modify_page_pointer(p_ptr_page, i_db_page, i_free_page_1,
                e_type_1);
        release_page(p_ptr_page);
        if rc == 0 {
            ptrmap_put(p_bt_1, i_free_page_1, e_type_1, i_ptr_page_1,
                &mut rc);
        }
    }
    return rc;
}
extern "C" fn incr_vacuum_step(p_bt_1: *mut BtShared, n_fin_1: Pgno,
    mut i_last_pg_1: Pgno, b_commit_1: i32) -> i32 {
    unsafe {
        let mut n_free_list: Pgno = 0 as Pgno;
        let mut rc: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        if !(ptrmap_pageno(unsafe { &*p_bt_1 }, i_last_pg_1) == i_last_pg_1)
                        as i32 != 0 &&
                i_last_pg_1 !=
                    (sqlite3_pending_byte as u32 /
                                unsafe { (*p_bt_1).page_size } + 1 as u32) as Pgno {
            let mut e_type: u8 = 0 as u8;
            let mut i_ptr_page: Pgno = 0 as Pgno;
            n_free_list =
                unsafe {
                    sqlite3_get4byte(unsafe {
                                &raw mut *unsafe {
                                            (*unsafe { (*p_bt_1).p_page1 }).a_data.offset(36 as isize)
                                        }
                            } as *const u8)
                };
            if n_free_list == 0 as u32 { return 101; }
            rc =
                ptrmap_get(p_bt_1, i_last_pg_1, &mut e_type, &mut i_ptr_page);
            if rc != 0 { return rc; }
            if e_type as i32 == 1 {
                return unsafe { sqlite3_corrupt_error(4089) };
            }
            if e_type as i32 == 2 {
                if b_commit_1 == 0 {
                    let mut i_free_pg: Pgno = 0 as Pgno;
                    let mut p_free_pg: *mut MemPage = core::ptr::null_mut();
                    rc =
                        unsafe {
                            allocate_btree_page(p_bt_1, &mut p_free_pg, &mut i_free_pg,
                                i_last_pg_1, 1 as u8)
                        };
                    if rc != 0 { return rc; }
                    { let _ = 0; };
                    release_page(p_free_pg);
                }
            } else {
                let mut i_free_pg_1: Pgno = 0 as Pgno;
                let mut p_last_pg: *mut MemPage = core::ptr::null_mut();
                let mut e_mode: u8 = 0 as u8;
                let mut i_near: Pgno = 0 as Pgno;
                rc = btree_get_page(p_bt_1, i_last_pg_1, &mut p_last_pg, 0);
                if rc != 0 { return rc; }
                if b_commit_1 == 0 { e_mode = 2 as u8; i_near = n_fin_1; }
                '__b29: loop {
                    '__c29: loop {
                        let mut p_free_pg_1: *mut MemPage = core::ptr::null_mut();
                        let db_size: Pgno = btree_pagecount(unsafe { &*p_bt_1 });
                        rc =
                            unsafe {
                                allocate_btree_page(p_bt_1, &mut p_free_pg_1,
                                    &mut i_free_pg_1, i_near, e_mode)
                            };
                        if rc != 0 { release_page(p_last_pg); return rc; }
                        release_page(p_free_pg_1);
                        if i_free_pg_1 > db_size {
                            release_page(p_last_pg);
                            return unsafe { sqlite3_corrupt_error(4141) };
                        }
                        break '__c29;
                    }
                    if !(b_commit_1 != 0 && i_free_pg_1 > n_fin_1) {
                        break '__b29;
                    }
                }
                { let _ = 0; };
                rc =
                    relocate_page(p_bt_1, p_last_pg, e_type, i_ptr_page,
                        i_free_pg_1, b_commit_1);
                release_page(p_last_pg);
                if rc != 0 { return rc; }
            }
        }
        if b_commit_1 == 0 {
            '__b30: loop {
                '__c30: loop {
                    {
                        let __p = &mut i_last_pg_1;
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    break '__c30;
                }
                if !(i_last_pg_1 ==
                                    (sqlite3_pending_byte as u32 /
                                                unsafe { (*p_bt_1).page_size } + 1 as u32) as Pgno ||
                                ptrmap_pageno(unsafe { &*p_bt_1 }, i_last_pg_1) ==
                                    i_last_pg_1) {
                    break '__b30;
                }
            }
            unsafe { (*p_bt_1).b_do_truncate = 1 as u8 };
            unsafe { (*p_bt_1).n_page = i_last_pg_1 };
        }
        return 0;
    }
}
extern "C" fn auto_vacuum_commit(p: *mut Btree) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_pager: *mut Pager = core::ptr::null_mut();
        let mut p_bt: *mut BtShared = core::ptr::null_mut();
        let mut db: *const sqlite3 = core::ptr::null();
        { let _ = 0; };
        p_bt = unsafe { (*p).p_bt };
        p_pager = unsafe { (*p_bt).p_pager };
        { let _ = 0; };
        invalidate_all_overflow_cache(unsafe { &*p_bt });
        { let _ = 0; };
        if (unsafe { (*p_bt).incr_vacuum } == 0) as i32 != 0 {
            let mut n_fin: Pgno = 0 as Pgno;
            let mut n_free: Pgno = 0 as Pgno;
            let mut n_vac: Pgno = 0 as Pgno;
            let mut i_free: Pgno = 0 as Pgno;
            let mut n_orig: Pgno = 0 as Pgno;
            n_orig = btree_pagecount(unsafe { &*p_bt });
            if ptrmap_pageno(unsafe { &*p_bt }, n_orig) == n_orig ||
                    n_orig ==
                        (sqlite3_pending_byte as u32 / unsafe { (*p_bt).page_size }
                                + 1 as u32) as Pgno {
                return unsafe { sqlite3_corrupt_error(4260) };
            }
            n_free =
                unsafe {
                    sqlite3_get4byte(unsafe {
                                &raw mut *unsafe {
                                            (*unsafe { (*p_bt).p_page1 }).a_data.offset(36 as isize)
                                        }
                            } as *const u8)
                };
            db = unsafe { (*p).db };
            if unsafe { (*db).x_autovac_pages.is_some() } {
                let mut i_db: i32 = 0;
                {
                    i_db = 0;
                    '__b31: loop {
                        if !(i_db < unsafe { (*db).n_db }) { break '__b31; }
                        '__c31: loop {
                            if unsafe {
                                        (*unsafe { (*db).a_db.offset(i_db as isize) }).p_bt
                                    } == p {
                                break '__b31;
                            }
                            break '__c31;
                        }
                        { let __p = &mut i_db; let __t = *__p; *__p += 1; __t };
                    }
                }
                n_vac =
                    unsafe {
                        (unsafe {
                                (*db).x_autovac_pages.unwrap()
                            })(unsafe { (*db).p_autovac_pages_arg },
                            unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                } as *const i8, n_orig, n_free,
                            unsafe { (*p_bt).page_size })
                    };
                if n_vac > n_free { n_vac = n_free; }
                if n_vac == 0 as u32 { return 0; }
            } else { n_vac = n_free; }
            n_fin = final_db_size(p_bt, n_orig, n_vac);
            if n_fin > n_orig {
                return unsafe { sqlite3_corrupt_error(4287) };
            }
            if n_fin < n_orig {
                rc =
                    save_all_cursors(unsafe { &*p_bt }, 0 as Pgno,
                        core::ptr::null_mut());
            }
            {
                i_free = n_orig;
                '__b32: loop {
                    if !(i_free > n_fin && rc == 0) { break '__b32; }
                    '__c32: loop {
                        rc =
                            incr_vacuum_step(p_bt, n_fin, i_free,
                                (n_vac == n_free) as i32);
                        break '__c32;
                    }
                    { let __p = &mut i_free; let __t = *__p; *__p -= 1; __t };
                }
            }
            if (rc == 101 || rc == 0) && n_free > 0 as u32 {
                rc =
                    unsafe {
                        sqlite3_pager_write(unsafe {
                                (*unsafe { (*p_bt).p_page1 }).p_db_page
                            })
                    };
                if n_vac == n_free {
                    unsafe {
                        sqlite3_put4byte(unsafe {
                                &mut *unsafe {
                                            (*unsafe { (*p_bt).p_page1 }).a_data.offset(32 as isize)
                                        }
                            }, 0 as u32)
                    };
                    unsafe {
                        sqlite3_put4byte(unsafe {
                                &mut *unsafe {
                                            (*unsafe { (*p_bt).p_page1 }).a_data.offset(36 as isize)
                                        }
                            }, 0 as u32)
                    };
                }
                unsafe {
                    sqlite3_put4byte(unsafe {
                            &mut *unsafe {
                                        (*unsafe { (*p_bt).p_page1 }).a_data.offset(28 as isize)
                                    }
                        }, n_fin)
                };
                unsafe { (*p_bt).b_do_truncate = 1 as u8 };
                unsafe { (*p_bt).n_page = n_fin };
            }
            if rc != 0 { unsafe { sqlite3_pager_rollback(p_pager) }; }
        }
        { let _ = 0; };
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_commit_phase_one(p: *mut Btree,
    z_super_jrnl_1: *const i8) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*p).in_trans } as i32 == 2 {
        let p_bt: *const BtShared = unsafe { (*p).p_bt } as *const BtShared;
        unsafe { sqlite3_btree_enter(p) };
        if unsafe { (*p_bt).auto_vacuum } != 0 {
            rc = auto_vacuum_commit(p);
            if rc != 0 { unsafe { sqlite3_btree_leave(p) }; return rc; }
        }
        if unsafe { (*p_bt).b_do_truncate } != 0 {
            unsafe {
                sqlite3_pager_truncate_image(unsafe { (*p_bt).p_pager },
                    unsafe { (*p_bt).n_page })
            };
        }
        rc =
            unsafe {
                sqlite3_pager_commit_phase_one(unsafe { (*p_bt).p_pager },
                    z_super_jrnl_1, 0)
            };
        unsafe { sqlite3_btree_leave(p) };
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_commit_phase_two(p: *mut Btree,
    b_cleanup_1: i32) -> i32 {
    if unsafe { (*p).in_trans } as i32 == 0 { return 0; }
    unsafe { sqlite3_btree_enter(p) };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p).in_trans } as i32 == 2 {
        let mut rc: i32 = 0;
        let p_bt: *mut BtShared = unsafe { (*p).p_bt };
        { let _ = 0; };
        { let _ = 0; };
        rc =
            unsafe {
                sqlite3_pager_commit_phase_two(unsafe { (*p_bt).p_pager })
            };
        if rc != 0 && b_cleanup_1 == 0 {
            unsafe { sqlite3_btree_leave(p) };
            return rc;
        }
        {
            let __p = unsafe { &mut (*p).i_b_data_version };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        unsafe { (*p_bt).in_transaction = 1 as u8 };
        btree_clear_has_content(unsafe { &mut *p_bt });
    }
    btree_end_transaction(p);
    unsafe { sqlite3_btree_leave(p) };
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_commit(p: *mut Btree) -> i32 {
    let mut rc: i32 = 0;
    unsafe { sqlite3_btree_enter(p) };
    rc = sqlite3_btree_commit_phase_one(p, core::ptr::null());
    if rc == 0 { rc = sqlite3_btree_commit_phase_two(p, 0); }
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_begin_stmt(p: *mut Btree, i_statement_1: i32)
    -> i32 {
    let mut rc: i32 = 0;
    let p_bt: *const BtShared = unsafe { (*p).p_bt } as *const BtShared;
    unsafe { sqlite3_btree_enter(p) };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    rc =
        unsafe {
            sqlite3_pager_open_savepoint(unsafe { (*p_bt).p_pager },
                i_statement_1)
        };
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_get_meta(p: *mut Btree, idx: i32,
    p_meta_1: &mut u32) -> () {
    let p_bt: *const BtShared = unsafe { (*p).p_bt } as *const BtShared;
    unsafe { sqlite3_btree_enter(p) };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if idx == 15 {
        *p_meta_1 =
            unsafe { sqlite3_pager_data_version(unsafe { (*p_bt).p_pager }) }
                + unsafe { (*p).i_b_data_version };
    } else {
        *p_meta_1 =
            unsafe {
                sqlite3_get4byte(unsafe {
                            &raw mut *unsafe {
                                        (*unsafe {
                                                            (*p_bt).p_page1
                                                        }).a_data.offset((36 + idx * 4) as isize)
                                    }
                        } as *const u8)
            };
    }
    unsafe { sqlite3_btree_leave(p) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_update_meta(p: *mut Btree, idx: i32,
    i_meta_1: u32) -> i32 {
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    let mut p_p1: *mut u8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = 0; };
    unsafe { sqlite3_btree_enter(p) };
    { let _ = 0; };
    { let _ = 0; };
    p_p1 = unsafe { (*unsafe { (*p_bt).p_page1 }).a_data };
    rc =
        unsafe {
            sqlite3_pager_write(unsafe {
                    (*unsafe { (*p_bt).p_page1 }).p_db_page
                })
        };
    if rc == 0 {
        unsafe {
            sqlite3_put4byte(unsafe {
                    &mut *p_p1.offset((36 + idx * 4) as isize)
                }, i_meta_1)
        };
        if idx == 7 {
            { let _ = 0; };
            { let _ = 0; };
            unsafe { (*p_bt).incr_vacuum = i_meta_1 as u8 };
        }
    }
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
extern "C" fn btree_create_table(p: *mut Btree, pi_table_1: &mut Pgno,
    create_tab_flags_1: i32) -> i32 {
    unsafe {
        let p_bt: *mut BtShared = unsafe { (*p).p_bt };
        let mut p_root: *mut MemPage = core::ptr::null_mut();
        let mut pgno_root: Pgno = 0 as Pgno;
        let mut rc: i32 = 0;
        let mut ptf_flags: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_bt).auto_vacuum } != 0 {
            let mut pgno_move: Pgno = 0 as Pgno;
            let mut p_page_move: *mut MemPage = core::ptr::null_mut();
            invalidate_all_overflow_cache(unsafe { &*p_bt });
            unsafe { sqlite3_btree_get_meta(p, 4, &mut pgno_root) };
            if pgno_root > btree_pagecount(unsafe { &*p_bt }) {
                return unsafe { sqlite3_corrupt_error(10120) };
            }
            { let __p = &mut pgno_root; let __t = *__p; *__p += 1; __t };
            while pgno_root == ptrmap_pageno(unsafe { &*p_bt }, pgno_root) ||
                    pgno_root ==
                        (sqlite3_pending_byte as u32 / unsafe { (*p_bt).page_size }
                                + 1 as u32) as Pgno {
                { let __p = &mut pgno_root; let __t = *__p; *__p += 1; __t };
            }
            { let _ = 0; };
            rc =
                allocate_btree_page(p_bt, &mut p_page_move, &mut pgno_move,
                    pgno_root, 1 as u8);
            if rc != 0 { return rc; }
            if pgno_move != pgno_root {
                let mut e_type: u8 = 0 as u8;
                let mut i_ptr_page: Pgno = 0 as Pgno;
                rc =
                    save_all_cursors(unsafe { &*p_bt }, 0 as Pgno,
                        core::ptr::null_mut());
                release_page(p_page_move);
                if rc != 0 { return rc; }
                rc = btree_get_page(p_bt, pgno_root, &mut p_root, 0);
                if rc != 0 { return rc; }
                rc =
                    ptrmap_get(p_bt, pgno_root, &mut e_type, &mut i_ptr_page);
                if e_type as i32 == 1 || e_type as i32 == 2 {
                    rc = unsafe { sqlite3_corrupt_error(10168) };
                }
                if rc != 0 { release_page(p_root); return rc; }
                { let _ = 0; };
                { let _ = 0; };
                rc =
                    relocate_page(p_bt, p_root, e_type, i_ptr_page, pgno_move,
                        0);
                release_page(p_root);
                if rc != 0 { return rc; }
                rc = btree_get_page(p_bt, pgno_root, &mut p_root, 0);
                if rc != 0 { return rc; }
                rc =
                    unsafe {
                        sqlite3_pager_write(unsafe { (*p_root).p_db_page })
                    };
                if rc != 0 { release_page(p_root); return rc; }
            } else { p_root = p_page_move; }
            ptrmap_put(p_bt, pgno_root, 1 as u8, 0 as Pgno, &mut rc);
            if rc != 0 { release_page(p_root); return rc; }
            { let _ = 0; };
            rc = unsafe { sqlite3_btree_update_meta(p, 4, pgno_root) };
            if rc != 0 { release_page(p_root); return rc; }
        } else {
            rc =
                allocate_btree_page(p_bt, &mut p_root, &mut pgno_root,
                    1 as Pgno, 0 as u8);
            if rc != 0 { return rc; }
        }
        { let _ = 0; };
        if create_tab_flags_1 & 1 != 0 {
            ptf_flags = 1 | 4 | 8;
        } else { ptf_flags = 2 | 8; }
        zero_page(p_root, ptf_flags);
        unsafe { sqlite3_pager_unref(unsafe { (*p_root).p_db_page }) };
        { let _ = 0; };
        *pi_table_1 = pgno_root;
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_create_table(p: *mut Btree,
    pi_table_1: *mut Pgno, flags: i32) -> i32 {
    let mut rc: i32 = 0;
    unsafe { sqlite3_btree_enter(p) };
    rc = btree_create_table(p, unsafe { &mut *pi_table_1 }, flags);
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_txn_state(p: *const Btree) -> i32 {
    { let _ = 0; };
    return if !(p).is_null() { (unsafe { (*p).in_trans }) as i32 } else { 0 };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_is_in_backup(p: &Btree) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    return ((*p).n_backup != 0) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_schema_locked(p: *mut Btree) -> i32 {
    let mut rc: i32 = 0;
    { let _ = p; };
    { let _ = 0; };
    unsafe { sqlite3_btree_enter(p) };
    rc = query_shared_cache_table_lock(p, 1 as Pgno, 1 as u8);
    { let _ = 0; };
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
extern "C" fn set_shared_cache_table_lock(p: *mut Btree, i_table_1: Pgno,
    e_lock_1: u8) -> i32 {
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    let mut p_lock: *mut BtLock = core::ptr::null_mut();
    let mut p_iter: *mut BtLock = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    {
        p_iter = unsafe { (*p_bt).p_lock };
        '__b34: loop {
            if !(!(p_iter).is_null()) { break '__b34; }
            '__c34: loop {
                if unsafe { (*p_iter).i_table } == i_table_1 &&
                        unsafe { (*p_iter).p_btree } == p {
                    p_lock = p_iter;
                    break '__b34;
                }
                break '__c34;
            }
            p_iter = unsafe { (*p_iter).p_next };
        }
    }
    if (p_lock).is_null() as i32 != 0 {
        p_lock =
            unsafe {
                    sqlite3_malloc_zero(core::mem::size_of::<BtLock>() as u64)
                } as *mut BtLock;
        if (p_lock).is_null() as i32 != 0 { return 7; }
        unsafe { (*p_lock).i_table = i_table_1 };
        unsafe { (*p_lock).p_btree = p };
        unsafe { (*p_lock).p_next = unsafe { (*p_bt).p_lock } };
        unsafe { (*p_bt).p_lock = p_lock };
    }
    { let _ = 0; };
    if e_lock_1 as i32 > unsafe { (*p_lock).e_lock } as i32 {
        unsafe { (*p_lock).e_lock = e_lock_1 };
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_lock_table(p: *mut Btree, i_tab_1: i32,
    is_write_lock_1: u8) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    if unsafe { (*p).sharable } != 0 {
        let lock_type: u8 = (1 + is_write_lock_1 as i32) as u8;
        { let _ = 0; };
        { let _ = 0; };
        unsafe { sqlite3_btree_enter(p) };
        rc = query_shared_cache_table_lock(p, i_tab_1 as Pgno, lock_type);
        if rc == 0 {
            rc = set_shared_cache_table_lock(p, i_tab_1 as Pgno, lock_type);
        }
        unsafe { sqlite3_btree_leave(p) };
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_savepoint(p: *mut Btree, op: i32,
    i_savepoint_1: i32) -> i32 {
    let mut rc: i32 = 0;
    if !(p).is_null() && unsafe { (*p).in_trans } as i32 == 2 {
        let p_bt: *mut BtShared = unsafe { (*p).p_bt };
        { let _ = 0; };
        { let _ = 0; };
        unsafe { sqlite3_btree_enter(p) };
        if op == 2 {
            rc =
                save_all_cursors(unsafe { &*p_bt }, 0 as Pgno,
                    core::ptr::null_mut());
        }
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_pager_savepoint(unsafe { (*p_bt).p_pager }, op,
                        i_savepoint_1)
                };
        }
        if rc == 0 {
            if i_savepoint_1 < 0 &&
                    unsafe { (*p_bt).bts_flags } as i32 & 16 != 0 {
                unsafe { (*p_bt).n_page = 0 as u32 };
            }
            rc = new_database(unsafe { &mut *p_bt });
            btree_set_n_page(unsafe { &mut *p_bt },
                unsafe { &*unsafe { (*p_bt).p_page1 } });
            { let _ = 0; };
        }
        unsafe { sqlite3_btree_leave(p) };
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_checkpoint(p: *mut Btree, e_mode_1: i32,
    pn_log_1: *mut i32, pn_ckpt_1: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    if !(p).is_null() {
        let p_bt: *const BtShared = unsafe { (*p).p_bt } as *const BtShared;
        unsafe { sqlite3_btree_enter(p) };
        if unsafe { (*p_bt).in_transaction } as i32 != 0 {
            rc = 6;
        } else {
            rc =
                unsafe {
                    sqlite3_pager_checkpoint(unsafe { (*p_bt).p_pager },
                        unsafe { (*p).db }, e_mode_1, pn_log_1, pn_ckpt_1)
                };
        }
        unsafe { sqlite3_btree_leave(p) };
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_get_filename(p: &Btree) -> *const i8 {
    { let _ = 0; };
    return unsafe {
            sqlite3_pager_filename(unsafe { (*(*p).p_bt).p_pager } as
                    *const Pager, 1)
        };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_get_journalname(p: &Btree) -> *const i8 {
    { let _ = 0; };
    return unsafe {
            sqlite3_pager_journalname(unsafe { (*(*p).p_bt).p_pager })
        };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_incr_vacuum(p: *mut Btree) -> i32 {
    let mut rc: i32 = 0;
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    unsafe { sqlite3_btree_enter(p) };
    { let _ = 0; };
    if (unsafe { (*p_bt).auto_vacuum } == 0) as i32 != 0 {
        rc = 101;
    } else {
        let n_orig: Pgno = btree_pagecount(unsafe { &*p_bt });
        let n_free: Pgno =
            unsafe {
                sqlite3_get4byte(unsafe {
                            &raw mut *unsafe {
                                        (*unsafe { (*p_bt).p_page1 }).a_data.offset(36 as isize)
                                    }
                        } as *const u8)
            };
        let n_fin: Pgno = final_db_size(p_bt, n_orig, n_free);
        if n_orig < n_fin || n_free >= n_orig {
            rc = unsafe { sqlite3_corrupt_error(4209) };
        } else if n_free > 0 as u32 {
            rc =
                save_all_cursors(unsafe { &*p_bt }, 0 as Pgno,
                    core::ptr::null_mut());
            if rc == 0 {
                invalidate_all_overflow_cache(unsafe { &*p_bt });
                rc = incr_vacuum_step(p_bt, n_fin, n_orig, 0);
            }
            if rc == 0 {
                rc =
                    unsafe {
                        sqlite3_pager_write(unsafe {
                                (*unsafe { (*p_bt).p_page1 }).p_db_page
                            })
                    };
                unsafe {
                    sqlite3_put4byte(unsafe {
                            &mut *unsafe {
                                        (*unsafe { (*p_bt).p_page1 }).a_data.offset(28 as isize)
                                    }
                        }, unsafe { (*p_bt).n_page })
                };
            }
        } else { rc = 101; }
    }
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
extern "C" fn invalidate_incrblob_cursors(p_btree_1: &mut Btree,
    pgno_root_1: Pgno, i_row_1: i64, is_clear_table_1: i32) -> () {
    let mut p: *mut BtCursor = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    (*p_btree_1).has_incrblob_cur = 0 as u8;
    {
        p = unsafe { (*(*p_btree_1).p_bt).p_cursor };
        '__b35: loop {
            if !(!(p).is_null()) { break '__b35; }
            '__c35: loop {
                if unsafe { (*p).cur_flags } as i32 & 16 != 0 {
                    (*p_btree_1).has_incrblob_cur = 1 as u8;
                    if unsafe { (*p).pgno_root } == pgno_root_1 &&
                            (is_clear_table_1 != 0 ||
                                unsafe { (*p).info.n_key } == i_row_1) {
                        unsafe { (*p).e_state = 1 as u8 };
                    }
                }
                break '__c35;
            }
            p = unsafe { (*p).p_next };
        }
    }
}
extern "C" fn get_and_init_page(p_bt_1: *mut BtShared, pgno: Pgno,
    pp_page_1: &mut *mut MemPage, b_read_only_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut p_db_page: *mut DbPage = core::ptr::null_mut();
    let mut p_page: *mut MemPage = core::ptr::null_mut();
    { let _ = 0; };
    if pgno > btree_pagecount(unsafe { &*p_bt_1 }) {
        *pp_page_1 = core::ptr::null_mut();
        return unsafe { sqlite3_corrupt_error(2422) };
    }
    rc =
        unsafe {
            sqlite3_pager_get(unsafe { (*p_bt_1).p_pager }, pgno,
                &raw mut p_db_page as *mut *mut DbPage, b_read_only_1)
        };
    if rc != 0 { *pp_page_1 = core::ptr::null_mut(); return rc; }
    p_page = unsafe { sqlite3_pager_get_extra(p_db_page) } as *mut MemPage;
    if unsafe { (*p_page).is_init } as i32 == 0 {
        btree_page_from_db_page(p_db_page, pgno, p_bt_1);
        rc = btree_init_page(p_page);
        if rc != 0 {
            unsafe { release_page(p_page) };
            *pp_page_1 = core::ptr::null_mut();
            return rc;
        }
    }
    { let _ = 0; };
    { let _ = 0; };
    *pp_page_1 = p_page;
    return 0;
}
extern "C" fn btree_page_lookup(p_bt_1: *mut BtShared, pgno: Pgno)
    -> *mut MemPage {
    let mut p_db_page: *mut DbPage = core::ptr::null_mut();
    { let _ = 0; };
    p_db_page =
        unsafe { sqlite3_pager_lookup(unsafe { (*p_bt_1).p_pager }, pgno) };
    if !(p_db_page).is_null() {
        return btree_page_from_db_page(p_db_page, pgno, p_bt_1);
    }
    return core::ptr::null_mut();
}
extern "C" fn btree_set_has_content(p_bt_1: &mut BtShared, pgno: Pgno)
    -> i32 {
    let mut rc: i32 = 0;
    if ((*p_bt_1).p_has_content).is_null() as i32 != 0 {
        { let _ = 0; };
        (*p_bt_1).p_has_content =
            unsafe { sqlite3_bitvec_create((*p_bt_1).n_page) };
        if ((*p_bt_1).p_has_content).is_null() as i32 != 0 { rc = 7; }
    }
    if rc == 0 &&
            pgno <= unsafe { sqlite3_bitvec_size((*p_bt_1).p_has_content) } {
        rc = unsafe { sqlite3_bitvec_set((*p_bt_1).p_has_content, pgno) };
    }
    return rc;
}
extern "C" fn free_page2(p_bt_1: *mut BtShared, p_mem_page_1: *mut MemPage,
    i_page_1: Pgno) -> i32 {
    let mut p_trunk: *mut MemPage = core::ptr::null_mut();
    let mut p_page: *mut MemPage = core::ptr::null_mut();
    let mut rc: i32 = 0;
    '__b36: loop {
        '__c36: loop {
            let mut i_trunk: Pgno = 0 as Pgno;
            let p_page1: *const MemPage =
                unsafe { (*p_bt_1).p_page1 } as *const MemPage;
            let mut n_free: u32 = 0 as u32;
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if i_page_1 < 2 as u32 || i_page_1 > unsafe { (*p_bt_1).n_page } {
                return unsafe { sqlite3_corrupt_error(6881) };
            }
            if !(p_mem_page_1).is_null() {
                p_page = p_mem_page_1;
                unsafe { sqlite3_pager_ref(unsafe { (*p_page).p_db_page }) };
            } else { p_page = btree_page_lookup(p_bt_1, i_page_1); }
            rc =
                unsafe {
                    sqlite3_pager_write(unsafe { (*p_page1).p_db_page })
                };
            if rc != 0 { break '__b36; }
            n_free =
                unsafe {
                    sqlite3_get4byte(unsafe {
                                &raw mut *unsafe { (*p_page1).a_data.offset(36 as isize) }
                            } as *const u8)
                };
            unsafe {
                sqlite3_put4byte(unsafe {
                        &mut *unsafe { (*p_page1).a_data.offset(36 as isize) }
                    }, n_free + 1 as u32)
            };
            if unsafe { (*p_bt_1).bts_flags } as i32 & 4 != 0 {
                if (p_page).is_null() as i32 != 0 &&
                            {
                                    rc = btree_get_page(p_bt_1, i_page_1, &mut p_page, 0);
                                    rc
                                } != 0 ||
                        {
                                rc =
                                    unsafe {
                                        sqlite3_pager_write(unsafe { (*p_page).p_db_page })
                                    };
                                rc
                            } != 0 {
                    break '__b36;
                }
                unsafe {
                    memset(unsafe { (*p_page).a_data } as *mut (), 0,
                        unsafe { (*unsafe { (*p_page).p_bt }).page_size } as u64)
                };
            }
            if unsafe { (*p_bt_1).auto_vacuum } != 0 {
                ptrmap_put(p_bt_1, i_page_1, 2 as u8, 0 as Pgno, &mut rc);
                if rc != 0 { break '__b36; }
            }
            if n_free != 0 as u32 {
                let mut n_leaf: u32 = 0 as u32;
                i_trunk =
                    unsafe {
                        sqlite3_get4byte(unsafe {
                                    &raw mut *unsafe { (*p_page1).a_data.offset(32 as isize) }
                                } as *const u8)
                    };
                if i_trunk > btree_pagecount(unsafe { &*p_bt_1 }) {
                    rc = unsafe { sqlite3_corrupt_error(6928) };
                    break '__b36;
                }
                rc = btree_get_page(p_bt_1, i_trunk, &mut p_trunk, 0);
                if rc != 0 { break '__b36; }
                n_leaf =
                    unsafe {
                        sqlite3_get4byte(unsafe {
                                    &raw mut *unsafe { (*p_trunk).a_data.offset(4 as isize) }
                                } as *const u8)
                    };
                { let _ = 0; };
                if n_leaf >
                        unsafe { (*p_bt_1).usable_size } as u32 / 4 as u32 -
                            2 as u32 {
                    rc = unsafe { sqlite3_corrupt_error(6939) };
                    break '__b36;
                }
                if n_leaf <
                        unsafe { (*p_bt_1).usable_size } as u32 / 4 as u32 -
                            8 as u32 {
                    rc =
                        unsafe {
                            sqlite3_pager_write(unsafe { (*p_trunk).p_db_page })
                        };
                    if rc == 0 {
                        unsafe {
                            sqlite3_put4byte(unsafe {
                                    &mut *unsafe { (*p_trunk).a_data.offset(4 as isize) }
                                }, n_leaf + 1 as u32)
                        };
                        unsafe {
                            sqlite3_put4byte(unsafe {
                                    &mut *unsafe {
                                                (*p_trunk).a_data.add((8 as u32 + n_leaf * 4 as u32) as
                                                        usize)
                                            }
                                }, i_page_1)
                        };
                        if !(p_page).is_null() &&
                                unsafe { (*p_bt_1).bts_flags } as i32 & 4 == 0 {
                            unsafe {
                                sqlite3_pager_dont_write(unsafe { (*p_page).p_db_page })
                            };
                        }
                        rc =
                            btree_set_has_content(unsafe { &mut *p_bt_1 }, i_page_1);
                    }
                    break '__b36;
                }
            }
            if p_page == core::ptr::null_mut() &&
                    0 !=
                        {
                            rc = btree_get_page(p_bt_1, i_page_1, &mut p_page, 0);
                            rc
                        } {
                break '__b36;
            }
            rc =
                unsafe {
                    sqlite3_pager_write(unsafe { (*p_page).p_db_page })
                };
            if rc != 0 { break '__b36; }
            unsafe { sqlite3_put4byte(unsafe { (*p_page).a_data }, i_trunk) };
            unsafe {
                sqlite3_put4byte(unsafe {
                        &mut *unsafe { (*p_page).a_data.offset(4 as isize) }
                    }, 0 as u32)
            };
            unsafe {
                sqlite3_put4byte(unsafe {
                        &mut *unsafe { (*p_page1).a_data.offset(32 as isize) }
                    }, i_page_1)
            };
            break '__c36;
        }
        if !(false) { break '__b36; }
    }
    if !(p_page).is_null() { unsafe { (*p_page).is_init = 0 as u8 }; }
    release_page(p_page);
    release_page(p_trunk);
    return rc;
}
extern "C" fn clear_cell_overflow(p_page_1: &MemPage, p_cell_1: *const u8,
    p_info_1: &CellInfo) -> i32 {
    let mut p_bt: *mut BtShared = core::ptr::null_mut();
    let mut ovfl_pgno: Pgno = 0 as Pgno;
    let mut rc: i32 = 0;
    let mut n_ovfl: i32 = 0;
    let mut ovfl_page_size: u32 = 0 as u32;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { p_cell_1.add((*p_info_1).n_size as usize) } >
            (*p_page_1).a_data_end {
        return unsafe { sqlite3_corrupt_error(7028) };
    }
    ovfl_pgno =
        unsafe {
            sqlite3_get4byte(unsafe {
                        unsafe {
                            p_cell_1.add((*p_info_1).n_size as
                                        usize).offset(-(4 as isize))
                        }
                    } as *const u8)
        };
    p_bt = (*p_page_1).p_bt;
    { let _ = 0; };
    ovfl_page_size = unsafe { (*p_bt).usable_size } - 4 as u32;
    n_ovfl =
        (((*p_info_1).n_payload - (*p_info_1).n_local as u32 + ovfl_page_size
                    - 1 as u32) / ovfl_page_size) as i32;
    { let _ = 0; };
    while { let __p = &mut n_ovfl; let __t = *__p; *__p -= 1; __t } != 0 {
        let mut i_next: Pgno = 0 as Pgno;
        let mut p_ovfl: *mut MemPage = core::ptr::null_mut();
        if ovfl_pgno < 2 as u32 ||
                ovfl_pgno > btree_pagecount(unsafe { &*p_bt }) {
            return unsafe { sqlite3_corrupt_error(7045) };
        }
        if n_ovfl != 0 {
            rc = get_overflow_page(p_bt, ovfl_pgno, &mut p_ovfl, &mut i_next);
            if rc != 0 { return rc; }
        }
        if (!(p_ovfl).is_null() ||
                    { p_ovfl = btree_page_lookup(p_bt, ovfl_pgno); p_ovfl } !=
                        core::ptr::null_mut()) &&
                unsafe {
                        sqlite3_pager_page_refcount(unsafe { (*p_ovfl).p_db_page })
                    } != 1 {
            rc = unsafe { sqlite3_corrupt_error(7065) };
        } else { rc = free_page2(p_bt, p_ovfl, ovfl_pgno); }
        if !(p_ovfl).is_null() {
            unsafe { sqlite3_pager_unref(unsafe { (*p_ovfl).p_db_page }) };
        }
        if rc != 0 { return rc; }
        ovfl_pgno = i_next;
    }
    return 0;
}
extern "C" fn free_page(p_page_1: *mut MemPage, p_rc_1: &mut i32) -> () {
    if *p_rc_1 == 0 {
        *p_rc_1 =
            free_page2(unsafe { (*p_page_1).p_bt }, p_page_1,
                unsafe { (*p_page_1).pgno });
    }
}
extern "C" fn clear_database_page(p_bt_1: *mut BtShared, pgno: Pgno,
    free_page_flag_1: i32, mut pn_change_1: *mut i64) -> i32 {
    let mut p_page: *mut MemPage = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut p_cell: *mut u8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut hdr: i32 = 0;
    let mut info: CellInfo = unsafe { core::mem::zeroed() };
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s39:
            {
            match __state {
                0 => { __state = 3; }
                2 => { release_page(p_page); __state = 47; }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { { let _ = 0; }; __state = 9; }
                9 => {
                    if pgno > btree_pagecount(unsafe { &*p_bt_1 }) {
                        __state = 11;
                    } else { __state = 10; }
                }
                10 => {
                    rc = get_and_init_page(p_bt_1, pgno, &mut p_page, 0);
                    __state = 12;
                }
                11 => { return unsafe { sqlite3_corrupt_error(10258) }; }
                12 => { if rc != 0 { __state = 14; } else { __state = 13; } }
                13 => {
                    if unsafe { (*p_bt_1).open_flags } as i32 & 4 == 0 &&
                            unsafe {
                                    sqlite3_pager_page_refcount(unsafe { (*p_page).p_db_page })
                                } != 1 + (pgno == 1 as u32) as i32 {
                        __state = 16;
                    } else { __state = 15; }
                }
                14 => { return rc; }
                15 => {
                    hdr = unsafe { (*p_page).hdr_offset } as i32;
                    __state = 18;
                }
                16 => {
                    rc = unsafe { sqlite3_corrupt_error(10265) };
                    __state = 17;
                }
                17 => { __state = 2; }
                18 => { i = 0; __state = 20; }
                19 => {
                    if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
                        __state = 35;
                    } else { __state = 34; }
                }
                20 => {
                    if i < unsafe { (*p_page).n_cell } as i32 {
                        __state = 21;
                    } else { __state = 19; }
                }
                21 => {
                    p_cell =
                        unsafe {
                            unsafe {
                                (*p_page).a_data.offset((unsafe { (*p_page).mask_page } as
                                                i32 &
                                            ((unsafe {
                                                                *unsafe {
                                                                        unsafe {
                                                                            (*p_page).a_cell_idx.offset((2 * i) as
                                                                                        isize).offset(0 as isize)
                                                                        }
                                                                    }
                                                            } as i32) << 8 |
                                                unsafe {
                                                        *unsafe {
                                                                unsafe {
                                                                    (*p_page).a_cell_idx.offset((2 * i) as
                                                                                isize).offset(1 as isize)
                                                                }
                                                            }
                                                    } as i32)) as isize)
                            }
                        };
                    __state = 23;
                }
                22 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 20;
                }
                23 => {
                    if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
                        __state = 25;
                    } else { __state = 24; }
                }
                24 => {
                    unsafe {
                        (unsafe {
                                (*p_page).x_parse_cell.unwrap()
                            })(p_page, p_cell, &mut info)
                    };
                    __state = 28;
                }
                25 => {
                    rc =
                        clear_database_page(p_bt_1,
                            unsafe { sqlite3_get4byte(p_cell as *const u8) }, 1,
                            pn_change_1);
                    __state = 26;
                }
                26 => { if rc != 0 { __state = 27; } else { __state = 24; } }
                27 => { __state = 2; }
                28 => {
                    if info.n_local as u32 != info.n_payload {
                        __state = 30;
                    } else { __state = 31; }
                }
                29 => { __state = 32; }
                30 => {
                    rc =
                        clear_cell_overflow(unsafe { &*p_page },
                            p_cell as *const u8, &info);
                    __state = 29;
                }
                31 => { rc = 0; __state = 29; }
                32 => { if rc != 0 { __state = 33; } else { __state = 22; } }
                33 => { __state = 2; }
                34 => {
                    if !(pn_change_1).is_null() {
                        __state = 41;
                    } else { __state = 40; }
                }
                35 => {
                    rc =
                        clear_database_page(p_bt_1,
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe {
                                                        (*p_page).a_data.offset((hdr + 8) as isize)
                                                    }
                                        } as *const u8)
                            }, 1, pn_change_1);
                    __state = 36;
                }
                36 => { if rc != 0 { __state = 38; } else { __state = 37; } }
                37 => {
                    if unsafe { (*p_page).int_key } != 0 {
                        __state = 39;
                    } else { __state = 34; }
                }
                38 => { __state = 2; }
                39 => { pn_change_1 = core::ptr::null_mut(); __state = 34; }
                40 => {
                    if free_page_flag_1 != 0 {
                        __state = 44;
                    } else { __state = 45; }
                }
                41 => { __state = 42; }
                42 => {
                    unsafe {
                        *pn_change_1 += unsafe { (*p_page).n_cell } as i64
                    };
                    __state = 40;
                }
                43 => { __state = 2; }
                44 => { free_page(p_page, &mut rc); __state = 43; }
                45 => {
                    if {
                                rc =
                                    unsafe {
                                        sqlite3_pager_write(unsafe { (*p_page).p_db_page })
                                    };
                                rc
                            } == 0 {
                        __state = 46;
                    } else { __state = 43; }
                }
                46 => {
                    zero_page(p_page,
                        unsafe { *unsafe { (*p_page).a_data.offset(hdr as isize) } }
                                as i32 | 8);
                    __state = 43;
                }
                47 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_clear_table(p: *mut Btree, i_table_1: i32,
    pn_change_1: *mut i64) -> i32 {
    let mut rc: i32 = 0;
    let p_bt: *mut BtShared = unsafe { (*p).p_bt };
    unsafe { sqlite3_btree_enter(p) };
    { let _ = 0; };
    rc =
        save_all_cursors(unsafe { &*p_bt }, i_table_1 as Pgno,
            core::ptr::null_mut());
    if 0 == rc {
        if unsafe { (*p).has_incrblob_cur } != 0 {
            invalidate_incrblob_cursors(unsafe { &mut *p }, i_table_1 as Pgno,
                0 as i64, 1);
        }
        rc = clear_database_page(p_bt, i_table_1 as Pgno, 0, pn_change_1);
    }
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
extern "C" fn btree_drop_table(p: *mut Btree, i_table_1: Pgno,
    pi_moved_1: &mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_page: *mut MemPage = core::ptr::null_mut();
        let p_bt: *mut BtShared = unsafe { (*p).p_bt };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if i_table_1 > btree_pagecount(unsafe { &*p_bt }) {
            return unsafe { sqlite3_corrupt_error(10369) };
        }
        rc =
            sqlite3_btree_clear_table(p, i_table_1 as i32,
                core::ptr::null_mut());
        if rc != 0 { return rc; }
        rc = btree_get_page(p_bt, i_table_1 as Pgno, &mut p_page, 0);
        if rc != 0 { release_page(p_page); return rc; }
        *pi_moved_1 = 0;
        if unsafe { (*p_bt).auto_vacuum } != 0 {
            let mut max_root_pgno: Pgno = 0 as Pgno;
            unsafe { sqlite3_btree_get_meta(p, 4, &mut max_root_pgno) };
            if i_table_1 == max_root_pgno {
                free_page(p_page, &mut rc);
                release_page(p_page);
                if rc != 0 { return rc; }
            } else {
                let mut p_move: *mut MemPage = core::ptr::null_mut();
                release_page(p_page);
                rc = btree_get_page(p_bt, max_root_pgno, &mut p_move, 0);
                if rc != 0 { return rc; }
                rc =
                    relocate_page(p_bt, p_move, 1 as u8, 0 as Pgno, i_table_1,
                        0);
                release_page(p_move);
                if rc != 0 { return rc; }
                p_move = core::ptr::null_mut();
                rc = btree_get_page(p_bt, max_root_pgno, &mut p_move, 0);
                free_page(p_move, &mut rc);
                release_page(p_move);
                if rc != 0 { return rc; }
                *pi_moved_1 = max_root_pgno as i32;
            }
            { let __p = &mut max_root_pgno; let __t = *__p; *__p -= 1; __t };
            while max_root_pgno ==
                        (sqlite3_pending_byte as u32 / unsafe { (*p_bt).page_size }
                                + 1 as u32) as Pgno ||
                    ptrmap_pageno(unsafe { &*p_bt }, max_root_pgno) ==
                        max_root_pgno {
                {
                    let __p = &mut max_root_pgno;
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
            { let _ = 0; };
            rc = unsafe { sqlite3_btree_update_meta(p, 4, max_root_pgno) };
        } else { free_page(p_page, &mut rc); release_page(p_page); }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_drop_table(p: *mut Btree, i_table_1: i32,
    pi_moved_1: *mut i32) -> i32 {
    let mut rc: i32 = 0;
    unsafe { sqlite3_btree_enter(p) };
    rc = btree_drop_table(p, i_table_1 as Pgno, unsafe { &mut *pi_moved_1 });
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_clear_table_of_cursor(p_cur_1: &BtCursor)
    -> i32 {
    return sqlite3_btree_clear_table((*p_cur_1).p_btree,
            (*p_cur_1).pgno_root as i32, core::ptr::null_mut());
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_new_db(p: *mut Btree) -> i32 {
    let mut rc: i32 = 0;
    unsafe { sqlite3_btree_enter(p) };
    unsafe { (*unsafe { (*p).p_bt }).n_page = 0 as u32 };
    rc = new_database(unsafe { &mut *unsafe { (*p).p_bt } });
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
extern "C" fn allocate_temp_space(p_bt_1: &mut BtShared) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    (*p_bt_1).p_tmp_space =
        unsafe { sqlite3_page_malloc((*p_bt_1).page_size as i32) } as *mut u8;
    if (*p_bt_1).p_tmp_space == core::ptr::null_mut() {
        let p_cur: *mut BtCursor = (*p_bt_1).p_cursor;
        (*p_bt_1).p_cursor = unsafe { (*p_cur).p_next };
        unsafe {
            memset(p_cur as *mut (), 0,
                core::mem::size_of::<BtCursor>() as u64)
        };
        return 7;
    }
    unsafe { memset((*p_bt_1).p_tmp_space as *mut (), 0, 8 as u64) };
    {
        let __n = 4;
        let __p = &mut (*p_bt_1).p_tmp_space;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    return 0;
}
extern "C" fn btree_cursor(p: *mut Btree, mut i_table_1: Pgno, wr_flag_1: i32,
    p_key_info_1: *mut KeyInfo, p_cur_1: *mut BtCursor) -> i32 {
    unsafe {
        let p_bt: *mut BtShared = unsafe { (*p).p_bt };
        let mut p_x: *mut BtCursor = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if i_table_1 <= 1 as u32 {
            if i_table_1 < 1 as u32 {
                return unsafe { sqlite3_corrupt_error(4751) };
            } else if btree_pagecount(unsafe { &*p_bt }) == 0 as u32 {
                { let _ = 0; };
                i_table_1 = 0 as Pgno;
            }
        }
        unsafe { (*p_cur_1).pgno_root = i_table_1 };
        unsafe { (*p_cur_1).i_page = -1 as i8 };
        unsafe { (*p_cur_1).p_key_info = p_key_info_1 };
        unsafe { (*p_cur_1).p_btree = p };
        unsafe { (*p_cur_1).p_bt = p_bt };
        unsafe { (*p_cur_1).cur_flags = 0 as u8 };
        {
            p_x = unsafe { (*p_bt).p_cursor };
            '__b41: loop {
                if !(!(p_x).is_null()) { break '__b41; }
                '__c41: loop {
                    if unsafe { (*p_x).pgno_root } == i_table_1 {
                        unsafe { (*p_x).cur_flags |= 32 as u8 };
                        unsafe { (*p_cur_1).cur_flags = 32 as u8 };
                    }
                    break '__c41;
                }
                p_x = unsafe { (*p_x).p_next };
            }
        }
        unsafe { (*p_cur_1).e_state = 1 as u8 };
        unsafe { (*p_cur_1).p_next = unsafe { (*p_bt).p_cursor } };
        unsafe { (*p_bt).p_cursor = p_cur_1 };
        if wr_flag_1 != 0 {
            unsafe { (*p_cur_1).cur_flags |= 1 as u8 };
            unsafe { (*p_cur_1).cur_pager_flags = 0 as u8 };
            if unsafe { (*p_bt).p_tmp_space } == core::ptr::null_mut() {
                return allocate_temp_space(unsafe { &mut *p_bt });
            }
        } else { unsafe { (*p_cur_1).cur_pager_flags = 2 as u8 }; }
        return 0;
    }
}
extern "C" fn btree_cursor_with_lock(p: *mut Btree, i_table_1: Pgno,
    wr_flag_1: i32, p_key_info_1: *mut KeyInfo, p_cur_1: *mut BtCursor)
    -> i32 {
    let mut rc: i32 = 0;
    unsafe { sqlite3_btree_enter(p) };
    rc = btree_cursor(p, i_table_1, wr_flag_1, p_key_info_1, p_cur_1);
    unsafe { sqlite3_btree_leave(p) };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor(p: *mut Btree, i_table_1: Pgno,
    wr_flag_1: i32, p_key_info_1: *mut KeyInfo, p_cur_1: *mut BtCursor)
    -> i32 {
    if unsafe { (*p).sharable } != 0 {
        return btree_cursor_with_lock(p, i_table_1, wr_flag_1, p_key_info_1,
                p_cur_1);
    } else {
        return btree_cursor(p, i_table_1, wr_flag_1, p_key_info_1, p_cur_1);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_fake_valid_cursor() -> *mut BtCursor {
    unsafe { { let _ = 0; }; return &raw mut fake_cursor as *mut BtCursor; }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_size() -> i32 {
    return (core::mem::size_of::<BtCursor>() as u64 + 7 as u64 & !7 as u64) as
            i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_zero(p: *mut BtCursor) -> () {
    unsafe {
        memset(p as *mut (), 0, core::mem::offset_of!(BtCursor, p_bt) as u64)
    };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_hint_flags(p_cur_1: &mut BtCursor,
    x: u32) -> () {
    { let _ = 0; };
    (*p_cur_1).hints = x as u8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_close_cursor(p_cur_1: *mut BtCursor) -> i32 {
    let p_btree: *mut Btree = unsafe { (*p_cur_1).p_btree };
    if !(p_btree).is_null() {
        let p_bt: *mut BtShared = unsafe { (*p_cur_1).p_bt };
        unsafe { sqlite3_btree_enter(p_btree) };
        { let _ = 0; };
        if unsafe { (*p_bt).p_cursor } == p_cur_1 {
            unsafe { (*p_bt).p_cursor = unsafe { (*p_cur_1).p_next } };
        } else {
            let mut p_prev: *mut BtCursor = unsafe { (*p_bt).p_cursor };
            '__b42: loop {
                '__c42: loop {
                    if unsafe { (*p_prev).p_next } == p_cur_1 {
                        unsafe { (*p_prev).p_next = unsafe { (*p_cur_1).p_next } };
                        break '__b42;
                    }
                    p_prev = unsafe { (*p_prev).p_next };
                    break '__c42;
                }
                if !(!(p_prev).is_null()) { break '__b42; }
            }
        }
        btree_release_all_cursor_pages(unsafe { &mut *p_cur_1 });
        unlock_btree_if_unused(unsafe { &mut *p_bt });
        unsafe { sqlite3_free(unsafe { (*p_cur_1).a_overflow } as *mut ()) };
        unsafe { sqlite3_free(unsafe { (*p_cur_1).p_key }) };
        if unsafe { (*p_bt).open_flags } as i32 & 4 != 0 &&
                unsafe { (*p_bt).p_cursor } == core::ptr::null_mut() {
            { let _ = 0; };
            sqlite3_btree_close(p_btree);
        } else { unsafe { sqlite3_btree_leave(p_btree) }; }
        unsafe { (*p_cur_1).p_btree = core::ptr::null_mut() };
    }
    return 0;
}
type RecordCompare =
    unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord) -> i32;
extern "C" fn cursor_on_last_page(p_cur_1: &BtCursor) -> i32 {
    let mut i: i32 = 0;
    { let _ = 0; };
    {
        i = 0;
        '__b43: loop {
            if !(i < (*p_cur_1).i_page as i32) { break '__b43; }
            '__c43: loop {
                let p_page: *const MemPage =
                    (*p_cur_1).ap_page[i as usize] as *const MemPage;
                if ((*p_cur_1).ai_idx[i as usize] as i32) <
                        unsafe { (*p_page).n_cell } as i32 {
                    return 0;
                }
                break '__c43;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}
extern "C" fn index_cell_compare(p_page_1: &MemPage, idx: i32,
    p_idx_key_1: *mut UnpackedRecord,
    x_record_compare_1:
        Option<unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord)
            -> i32>) -> i32 {
    let mut c: i32 = 0;
    let mut n_cell: i32 = 0;
    let p_cell: *mut u8 =
        unsafe {
            (*p_page_1).a_data_ofst.offset(((*p_page_1).mask_page as i32 &
                        ((unsafe {
                                            *unsafe {
                                                    (*p_page_1).a_cell_idx.offset((2 * idx) as
                                                                isize).offset(0 as isize)
                                                }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe {
                                            (*p_page_1).a_cell_idx.offset((2 * idx) as
                                                        isize).offset(1 as isize)
                                        }
                                } as i32)) as isize)
        };
    n_cell = unsafe { *p_cell.offset(0 as isize) } as i32;
    if n_cell <= (*p_page_1).max1byte_payload as i32 {
        if unsafe { p_cell.offset(n_cell as isize) } >= (*p_page_1).a_data_end
            {
            return 99;
        }
        c =
            unsafe {
                x_record_compare_1.unwrap()(n_cell,
                    unsafe { &raw mut *p_cell.offset(1 as isize) } as *mut () as
                        *const (), p_idx_key_1)
            };
    } else if (unsafe { *p_cell.offset(1 as isize) } as i32 & 128 == 0) as i32
                != 0 &&
            {
                    n_cell =
                        ((n_cell & 127) << 7) +
                            unsafe { *p_cell.offset(1 as isize) } as i32;
                    n_cell
                } <= (*p_page_1).max_local as i32 {
        if unsafe { p_cell.offset(n_cell as isize) } >= (*p_page_1).a_data_end
            {
            return 99;
        }
        c =
            unsafe {
                x_record_compare_1.unwrap()(n_cell,
                    unsafe { &raw mut *p_cell.offset(2 as isize) } as *mut () as
                        *const (), p_idx_key_1)
            };
    } else { c = 99; }
    return c;
}
extern "C" fn move_to_child(p_cur_1: &mut BtCursor, new_pgno_1: u32) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if (*p_cur_1).i_page as i32 >= 20 - 1 {
        return unsafe { sqlite3_corrupt_error(5493) };
    }
    (*p_cur_1).info.n_size = 0 as u16;
    (*p_cur_1).cur_flags &= !(2 | 4) as u8;
    (*p_cur_1).ai_idx[(*p_cur_1).i_page as usize] = (*p_cur_1).ix;
    (*p_cur_1).ap_page[(*p_cur_1).i_page as usize] = (*p_cur_1).p_page;
    (*p_cur_1).ix = 0 as u16;
    { let __p = &mut (*p_cur_1).i_page; let __t = *__p; *__p += 1; __t };
    rc =
        get_and_init_page((*p_cur_1).p_bt, new_pgno_1, &mut (*p_cur_1).p_page,
            (*p_cur_1).cur_pager_flags as i32);
    { let _ = 0; };
    if rc == 0 &&
            ((unsafe { (*(*p_cur_1).p_page).n_cell } as i32) < 1 ||
                unsafe { (*(*p_cur_1).p_page).int_key } as i32 !=
                    (*p_cur_1).cur_int_key as i32) {
        release_page((*p_cur_1).p_page);
        rc = unsafe { sqlite3_corrupt_error(5507) };
    }
    if rc != 0 {
        (*p_cur_1).p_page =
            (*p_cur_1).ap_page[{
                        let __p = &mut (*p_cur_1).i_page;
                        *__p -= 1;
                        *__p
                    } as usize];
    }
    return rc;
}
extern "C" fn move_to_root(p_cur_1: *mut BtCursor) -> i32 {
    unsafe {
        let mut p_root: *const MemPage = core::ptr::null();
        let mut rc: i32 = 0;
        let mut subpage: Pgno = 0 as Pgno;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s45:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => { unsafe { (*p_cur_1).ix = 0 as u16 }; __state = 38; }
                    3 => { rc = 0; __state = 4; }
                    4 => { { let _ = 0; }; __state = 5; }
                    5 => { { let _ = 0; }; __state = 6; }
                    6 => { { let _ = 0; }; __state = 7; }
                    7 => { { let _ = 0; }; __state = 8; }
                    8 => { { let _ = 0; }; __state = 9; }
                    9 => { { let _ = 0; }; __state = 10; }
                    10 => {
                        if unsafe { (*p_cur_1).i_page } as i32 >= 0 {
                            __state = 12;
                        } else { __state = 13; }
                    }
                    11 => {
                        p_root = unsafe { (*p_cur_1).p_page };
                        __state = 33;
                    }
                    12 => {
                        if unsafe { (*p_cur_1).i_page } != 0 {
                            __state = 14;
                        } else { __state = 11; }
                    }
                    13 => {
                        if unsafe { (*p_cur_1).pgno_root } == 0 as u32 {
                            __state = 19;
                        } else { __state = 20; }
                    }
                    14 => {
                        release_page_not_null(unsafe {
                                &*unsafe { (*p_cur_1).p_page }
                            });
                        __state = 15;
                    }
                    15 => {
                        if {
                                    let __p = unsafe { &mut (*p_cur_1).i_page };
                                    *__p -= 1;
                                    *__p
                                } != 0 {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    16 => {
                        p_root =
                            {
                                unsafe {
                                    (*p_cur_1).p_page =
                                        unsafe { (*p_cur_1).ap_page[0 as usize] }
                                };
                                unsafe { (*p_cur_1).p_page }
                            };
                        __state = 18;
                    }
                    17 => {
                        release_page_not_null(unsafe {
                                &*unsafe {
                                            (*p_cur_1).ap_page[unsafe { (*p_cur_1).i_page } as usize]
                                        }
                            });
                        __state = 15;
                    }
                    18 => { __state = 2; }
                    19 => {
                        unsafe { (*p_cur_1).e_state = 1 as u8 };
                        __state = 21;
                    }
                    20 => { { let _ = 0; }; __state = 22; }
                    21 => { return 16; }
                    22 => {
                        if unsafe { (*p_cur_1).e_state } as i32 >= 3 {
                            __state = 24;
                        } else { __state = 23; }
                    }
                    23 => {
                        rc =
                            get_and_init_page(unsafe { (*p_cur_1).p_bt },
                                unsafe { (*p_cur_1).pgno_root },
                                unsafe { &mut (*p_cur_1).p_page },
                                unsafe { (*p_cur_1).cur_pager_flags } as i32);
                        __state = 28;
                    }
                    24 => {
                        if unsafe { (*p_cur_1).e_state } as i32 == 4 {
                            __state = 26;
                        } else { __state = 25; }
                    }
                    25 => {
                        sqlite3_btree_clear_cursor(unsafe { &mut *p_cur_1 });
                        __state = 23;
                    }
                    26 => { { let _ = 0; }; __state = 27; }
                    27 => { return unsafe { (*p_cur_1).skip_next }; }
                    28 => {
                        if rc != 0 { __state = 30; } else { __state = 29; }
                    }
                    29 => {
                        unsafe { (*p_cur_1).i_page = 0 as i8 };
                        __state = 32;
                    }
                    30 => {
                        unsafe { (*p_cur_1).e_state = 1 as u8 };
                        __state = 31;
                    }
                    31 => { return rc; }
                    32 => {
                        unsafe {
                            (*p_cur_1).cur_int_key =
                                unsafe { (*unsafe { (*p_cur_1).p_page }).int_key }
                        };
                        __state = 11;
                    }
                    33 => { { let _ = 0; }; __state = 34; }
                    34 => { { let _ = 0; }; __state = 35; }
                    35 => {
                        if unsafe { (*p_root).is_init } as i32 == 0 ||
                                (unsafe { (*p_cur_1).p_key_info } == core::ptr::null_mut())
                                        as i32 != unsafe { (*p_root).int_key } as i32 {
                            __state = 37;
                        } else { __state = 36; }
                    }
                    36 => { __state = 2; }
                    37 => { return unsafe { sqlite3_corrupt_error(5642) }; }
                    38 => {
                        unsafe { (*p_cur_1).info.n_size = 0 as u16 };
                        __state = 39;
                    }
                    39 => {
                        unsafe { (*p_cur_1).cur_flags &= !(8 | 2 | 4) as u8 };
                        __state = 40;
                    }
                    40 => {
                        if unsafe { (*p_root).n_cell } as i32 > 0 {
                            __state = 42;
                        } else { __state = 43; }
                    }
                    41 => { return rc; }
                    42 => {
                        unsafe { (*p_cur_1).e_state = 0 as u8 };
                        __state = 41;
                    }
                    43 => {
                        if (unsafe { (*p_root).leaf } == 0) as i32 != 0 {
                            __state = 44;
                        } else { __state = 45; }
                    }
                    44 => { __state = 46; }
                    45 => {
                        unsafe { (*p_cur_1).e_state = 1 as u8 };
                        __state = 51;
                    }
                    46 => {
                        if unsafe { (*p_root).pgno } != 1 as u32 {
                            __state = 48;
                        } else { __state = 47; }
                    }
                    47 => {
                        subpage =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe {
                                                        (*p_root).a_data.offset((unsafe { (*p_root).hdr_offset } as
                                                                        i32 + 8) as isize)
                                                    }
                                        } as *const u8)
                            };
                        __state = 49;
                    }
                    48 => { return unsafe { sqlite3_corrupt_error(5654) }; }
                    49 => {
                        unsafe { (*p_cur_1).e_state = 0 as u8 };
                        __state = 50;
                    }
                    50 => {
                        rc = move_to_child(unsafe { &mut *p_cur_1 }, subpage);
                        __state = 41;
                    }
                    51 => { rc = 16; __state = 41; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_index_moveto(p_cur_1: *mut BtCursor,
    p_idx_key_1: *mut UnpackedRecord, p_res_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut x_record_compare:
            Option<unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord)
                -> i32> = None;
    let mut c: i32 = 0;
    let mut lwr: i32 = 0;
    let mut upr: i32 = 0;
    let mut idx: i32 = 0;
    let mut c__1: i32 = 0;
    let mut chld_pg: Pgno = 0 as Pgno;
    let mut p_page: *mut MemPage = core::ptr::null_mut();
    let mut p_cell: *mut u8 = core::ptr::null_mut();
    let mut n_cell: i32 = 0;
    let mut p_cell_key: *mut () = core::ptr::null_mut();
    let mut p_cell_body: *mut u8 = core::ptr::null_mut();
    let mut n_overrun: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s47:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    unsafe { (*p_cur_1).info.n_size = 0 as u16 };
                    __state = 129;
                }
                3 => { __state = 4; }
                4 => { { let _ = 0; }; __state = 7; }
                5 => { { let _ = 0; }; __state = 31; }
                6 => { if false { __state = 4; } else { __state = 5; } }
                7 => { { let _ = 0; }; __state = 8; }
                8 => { { let _ = 0; }; __state = 9; }
                9 => { { let _ = 0; }; __state = 10; }
                10 => {
                    x_record_compare =
                        Some(unsafe { sqlite3_vdbe_find_compare(p_idx_key_1) });
                    __state = 11;
                }
                11 => {
                    unsafe { (*p_idx_key_1).err_code = 0 as u8 };
                    __state = 12;
                }
                12 => { { let _ = 0; }; __state = 13; }
                13 => {
                    if unsafe { (*p_cur_1).e_state } as i32 == 0 &&
                                unsafe { (*unsafe { (*p_cur_1).p_page }).leaf } != 0 &&
                            cursor_on_last_page(unsafe { &*p_cur_1 }) != 0 {
                        __state = 15;
                    } else { __state = 14; }
                }
                14 => { rc = move_to_root(p_cur_1); __state = 25; }
                15 => { __state = 16; }
                16 => {
                    if unsafe { (*p_cur_1).ix } as i32 ==
                                    unsafe { (*unsafe { (*p_cur_1).p_page }).n_cell } as i32 - 1
                                &&
                                {
                                        c =
                                            index_cell_compare(unsafe {
                                                    &*unsafe { (*p_cur_1).p_page }
                                                }, unsafe { (*p_cur_1).ix } as i32, p_idx_key_1,
                                                x_record_compare);
                                        c
                                    } <= 0 && unsafe { (*p_idx_key_1).err_code } as i32 == 0 {
                        __state = 18;
                    } else { __state = 17; }
                }
                17 => {
                    if unsafe { (*p_cur_1).i_page } as i32 > 0 &&
                                index_cell_compare(unsafe {
                                            &*unsafe { (*p_cur_1).p_page }
                                        }, 0, p_idx_key_1, x_record_compare) <= 0 &&
                            unsafe { (*p_idx_key_1).err_code } as i32 == 0 {
                        __state = 21;
                    } else { __state = 20; }
                }
                18 => { *p_res_1 = c; __state = 19; }
                19 => { return 0; }
                20 => {
                    unsafe { (*p_idx_key_1).err_code = 0 as u8 };
                    __state = 14;
                }
                21 => {
                    unsafe { (*p_cur_1).cur_flags &= !(4 | 8) as u8 };
                    __state = 22;
                }
                22 => {
                    if (unsafe { (*unsafe { (*p_cur_1).p_page }).is_init } == 0)
                                as i32 != 0 {
                        __state = 24;
                    } else { __state = 23; }
                }
                23 => { __state = 5; }
                24 => { return unsafe { sqlite3_corrupt_error(6122) }; }
                25 => { if rc != 0 { __state = 26; } else { __state = 6; } }
                26 => { if rc == 16 { __state = 28; } else { __state = 27; } }
                27 => { return rc; }
                28 => { { let _ = 0; }; __state = 29; }
                29 => { *p_res_1 = -1; __state = 30; }
                30 => { return 0; }
                31 => { { let _ = 0; }; __state = 32; }
                32 => { { let _ = 0; }; __state = 33; }
                33 => { { let _ = 0; }; __state = 34; }
                34 => { { let _ = 0; }; __state = 35; }
                35 => { { let _ = 0; }; __state = 36; }
                36 => { __state = 38; }
                37 => { __state = 2; }
                38 => { __state = 39; }
                39 => { __state = 41; }
                40 => { __state = 38; }
                41 => { __state = 42; }
                42 => { p_page = unsafe { (*p_cur_1).p_page }; __state = 43; }
                43 => { __state = 44; }
                44 => { { let _ = 0; }; __state = 45; }
                45 => { { let _ = 0; }; __state = 46; }
                46 => { lwr = 0; __state = 47; }
                47 => {
                    upr = unsafe { (*p_page).n_cell } as i32 - 1;
                    __state = 48;
                }
                48 => { idx = upr >> 1; __state = 49; }
                49 => { __state = 51; }
                50 => { { let _ = 0; }; __state = 104; }
                51 => { __state = 52; }
                52 => { __state = 54; }
                53 => { __state = 51; }
                54 => {
                    p_cell =
                        unsafe {
                            unsafe {
                                (*p_page).a_data_ofst.offset((unsafe { (*p_page).mask_page }
                                                as i32 &
                                            ((unsafe {
                                                                *unsafe {
                                                                        unsafe {
                                                                            (*p_page).a_cell_idx.offset((2 * idx) as
                                                                                        isize).offset(0 as isize)
                                                                        }
                                                                    }
                                                            } as i32) << 8 |
                                                unsafe {
                                                        *unsafe {
                                                                unsafe {
                                                                    (*p_page).a_cell_idx.offset((2 * idx) as
                                                                                isize).offset(1 as isize)
                                                                }
                                                            }
                                                    } as i32)) as isize)
                            }
                        };
                    __state = 55;
                }
                55 => {
                    n_cell = unsafe { *p_cell.offset(0 as isize) } as i32;
                    __state = 56;
                }
                56 => {
                    if n_cell <= unsafe { (*p_page).max1byte_payload } as i32 {
                        __state = 58;
                    } else { __state = 59; }
                }
                57 => { { let _ = 0; }; __state = 89; }
                58 => {
                    if unsafe { p_cell.offset(n_cell as isize) } >=
                            unsafe { (*p_page).a_data_end } {
                        __state = 61;
                    } else { __state = 60; }
                }
                59 => {
                    if (unsafe { *p_cell.offset(1 as isize) } as i32 & 128 == 0)
                                        as i32 != 0 &&
                                {
                                        n_cell =
                                            ((n_cell & 127) << 7) +
                                                unsafe { *p_cell.offset(1 as isize) } as i32;
                                        n_cell
                                    } <= unsafe { (*p_page).max_local } as i32 &&
                            unsafe { p_cell.offset(n_cell as isize) } <
                                unsafe { (*p_page).a_data_end } {
                        __state = 63;
                    } else { __state = 64; }
                }
                60 => {
                    c__1 =
                        unsafe {
                            x_record_compare.unwrap()(n_cell,
                                unsafe { &raw mut *p_cell.offset(1 as isize) } as *mut () as
                                    *const (), p_idx_key_1)
                        };
                    __state = 57;
                }
                61 => {
                    rc = unsafe { sqlite3_corrupt_error(6181) };
                    __state = 62;
                }
                62 => { __state = 2; }
                63 => {
                    c__1 =
                        unsafe {
                            x_record_compare.unwrap()(n_cell,
                                unsafe { &raw mut *p_cell.offset(2 as isize) } as *mut () as
                                    *const (), p_idx_key_1)
                        };
                    __state = 57;
                }
                64 => { __state = 65; }
                65 => {
                    p_cell_body =
                        unsafe {
                            p_cell.sub(unsafe { (*p_page).child_ptr_size } as usize)
                        };
                    __state = 66;
                }
                66 => { n_overrun = 18 as i32; __state = 67; }
                67 => {
                    unsafe {
                        (unsafe {
                                (*p_page).x_parse_cell.unwrap()
                            })(p_page, p_cell_body, unsafe { &mut (*p_cur_1).info })
                    };
                    __state = 68;
                }
                68 => {
                    n_cell = unsafe { (*p_cur_1).info.n_key } as i32;
                    __state = 69;
                }
                69 => { __state = 70; }
                70 => { __state = 71; }
                71 => { __state = 72; }
                72 => { __state = 73; }
                73 => {
                    if n_cell < 2 ||
                            n_cell as u32 /
                                    unsafe { (*unsafe { (*p_cur_1).p_bt }).usable_size } >
                                unsafe { (*unsafe { (*p_cur_1).p_bt }).n_page } {
                        __state = 75;
                    } else { __state = 74; }
                }
                74 => {
                    p_cell_key =
                        unsafe { sqlite3Malloc(n_cell as u64 + n_overrun as u64) };
                    __state = 77;
                }
                75 => {
                    rc = unsafe { sqlite3_corrupt_error(6212) };
                    __state = 76;
                }
                76 => { __state = 2; }
                77 => {
                    if p_cell_key == core::ptr::null_mut() {
                        __state = 79;
                    } else { __state = 78; }
                }
                78 => { unsafe { (*p_cur_1).ix = idx as u16 }; __state = 81; }
                79 => { rc = 7; __state = 80; }
                80 => { __state = 2; }
                81 => {
                    rc =
                        access_payload(p_cur_1, 0 as u32, n_cell as u32,
                            p_cell_key as *mut u8, 0);
                    __state = 82;
                }
                82 => {
                    unsafe {
                        memset(unsafe {
                                    (p_cell_key as *mut u8).offset(n_cell as isize)
                                } as *mut (), 0, n_overrun as u64)
                    };
                    __state = 83;
                }
                83 => {
                    unsafe { (*p_cur_1).cur_flags &= !4 as u8 };
                    __state = 84;
                }
                84 => { if rc != 0 { __state = 86; } else { __state = 85; } }
                85 => {
                    c__1 =
                        unsafe {
                            sqlite3_vdbe_record_compare(n_cell, p_cell_key as *const (),
                                p_idx_key_1)
                        };
                    __state = 88;
                }
                86 => { unsafe { sqlite3_free(p_cell_key) }; __state = 87; }
                87 => { __state = 2; }
                88 => { unsafe { sqlite3_free(p_cell_key) }; __state = 57; }
                89 => { if c__1 < 0 { __state = 91; } else { __state = 92; } }
                90 => {
                    if lwr > upr { __state = 102; } else { __state = 101; }
                }
                91 => { lwr = idx + 1; __state = 90; }
                92 => { if c__1 > 0 { __state = 93; } else { __state = 94; } }
                93 => { upr = idx - 1; __state = 90; }
                94 => { { let _ = 0; }; __state = 95; }
                95 => { *p_res_1 = 0; __state = 96; }
                96 => { rc = 0; __state = 97; }
                97 => { unsafe { (*p_cur_1).ix = idx as u16 }; __state = 98; }
                98 => {
                    if unsafe { (*p_idx_key_1).err_code } != 0 {
                        __state = 100;
                    } else { __state = 99; }
                }
                99 => { __state = 2; }
                100 => {
                    rc = unsafe { sqlite3_corrupt_error(6244) };
                    __state = 99;
                }
                101 => { { let _ = 0; }; __state = 103; }
                102 => { __state = 50; }
                103 => { idx = lwr + upr >> 1; __state = 53; }
                104 => { { let _ = 0; }; __state = 105; }
                105 => {
                    if unsafe { (*p_page).leaf } != 0 {
                        __state = 107;
                    } else { __state = 106; }
                }
                106 => {
                    if lwr >= unsafe { (*p_page).n_cell } as i32 {
                        __state = 113;
                    } else { __state = 114; }
                }
                107 => { { let _ = 0; }; __state = 108; }
                108 => {
                    unsafe { (*p_cur_1).ix = idx as u16 };
                    __state = 109;
                }
                109 => { *p_res_1 = c__1; __state = 110; }
                110 => { rc = 0; __state = 111; }
                111 => { __state = 2; }
                112 => {
                    unsafe { (*p_cur_1).info.n_size = 0 as u16 };
                    __state = 115;
                }
                113 => {
                    chld_pg =
                        unsafe {
                            sqlite3_get4byte(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_page).a_data.offset((unsafe { (*p_page).hdr_offset } as
                                                                    i32 + 8) as isize)
                                                }
                                    } as *const u8)
                        };
                    __state = 112;
                }
                114 => {
                    chld_pg =
                        unsafe {
                            sqlite3_get4byte(unsafe {
                                        unsafe {
                                            (*p_page).a_data.offset((unsafe { (*p_page).mask_page } as
                                                            i32 &
                                                        ((unsafe {
                                                                            *unsafe {
                                                                                    unsafe {
                                                                                        (*p_page).a_cell_idx.offset((2 * lwr) as
                                                                                                    isize).offset(0 as isize)
                                                                                    }
                                                                                }
                                                                        } as i32) << 8 |
                                                            unsafe {
                                                                    *unsafe {
                                                                            unsafe {
                                                                                (*p_page).a_cell_idx.offset((2 * lwr) as
                                                                                            isize).offset(1 as isize)
                                                                            }
                                                                        }
                                                                } as i32)) as isize)
                                        }
                                    } as *const u8)
                        };
                    __state = 112;
                }
                115 => {
                    unsafe { (*p_cur_1).cur_flags &= !(2 | 4) as u8 };
                    __state = 116;
                }
                116 => {
                    if unsafe { (*p_cur_1).i_page } as i32 >= 20 - 1 {
                        __state = 118;
                    } else { __state = 117; }
                }
                117 => {
                    unsafe {
                        (*p_cur_1).ai_idx[unsafe { (*p_cur_1).i_page } as usize] =
                            lwr as u16
                    };
                    __state = 119;
                }
                118 => { return unsafe { sqlite3_corrupt_error(6275) }; }
                119 => {
                    unsafe {
                        (*p_cur_1).ap_page[unsafe { (*p_cur_1).i_page } as usize] =
                            unsafe { (*p_cur_1).p_page }
                    };
                    __state = 120;
                }
                120 => { unsafe { (*p_cur_1).ix = 0 as u16 }; __state = 121; }
                121 => {
                    {
                        let __p = unsafe { &mut (*p_cur_1).i_page };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 122;
                }
                122 => {
                    rc =
                        get_and_init_page(unsafe { (*p_cur_1).p_bt }, chld_pg,
                            unsafe { &mut (*p_cur_1).p_page },
                            unsafe { (*p_cur_1).cur_pager_flags } as i32);
                    __state = 123;
                }
                123 => {
                    if rc == 0 &&
                            ((unsafe { (*unsafe { (*p_cur_1).p_page }).n_cell } as i32)
                                    < 1 ||
                                unsafe { (*unsafe { (*p_cur_1).p_page }).int_key } as i32 !=
                                    unsafe { (*p_cur_1).cur_int_key } as i32) {
                        __state = 125;
                    } else { __state = 124; }
                }
                124 => {
                    if rc != 0 { __state = 127; } else { __state = 40; }
                }
                125 => {
                    release_page(unsafe { (*p_cur_1).p_page });
                    __state = 126;
                }
                126 => {
                    rc = unsafe { sqlite3_corrupt_error(6286) };
                    __state = 124;
                }
                127 => {
                    unsafe {
                        (*p_cur_1).p_page =
                            unsafe {
                                (*p_cur_1).ap_page[{
                                            let __p = unsafe { &mut (*p_cur_1).i_page };
                                            *__p -= 1;
                                            *__p
                                        } as usize]
                            }
                    };
                    __state = 128;
                }
                128 => { __state = 37; }
                129 => { { let _ = 0; }; __state = 130; }
                130 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn btree_moveto(p_cur_1: *mut BtCursor, p_key_1: *const (),
    n_key_1: i64, bias: i32, p_res_1: *mut i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_idx_key: *mut UnpackedRecord = core::ptr::null_mut();
        if !(p_key_1).is_null() {
            let p_key_info: *mut KeyInfo = unsafe { (*p_cur_1).p_key_info };
            { let _ = 0; };
            p_idx_key =
                unsafe { sqlite3_vdbe_alloc_unpacked_record(p_key_info) };
            if p_idx_key == core::ptr::null_mut() { return 7; }
            unsafe {
                sqlite3_vdbe_record_unpack(n_key_1 as i32, p_key_1, p_idx_key)
            };
            if unsafe { (*p_idx_key).n_field } as i32 == 0 ||
                    unsafe { (*p_idx_key).n_field } as i32 >
                        unsafe { (*p_key_info).n_all_field } as i32 {
                rc = unsafe { sqlite3_corrupt_error(887) };
            } else {
                rc =
                    unsafe {
                        sqlite3_btree_index_moveto(p_cur_1, p_idx_key,
                            unsafe { &mut *p_res_1 })
                    };
            }
            unsafe {
                sqlite3_db_free(unsafe {
                        (*unsafe { (*p_cur_1).p_key_info }).db
                    }, p_idx_key as *mut ())
            };
        } else {
            p_idx_key = core::ptr::null_mut();
            rc =
                unsafe {
                    sqlite3_btree_table_moveto(p_cur_1, n_key_1, bias,
                        unsafe { &mut *p_res_1 })
                };
        }
        return rc;
    }
}
extern "C" fn btree_restore_cursor_position(p_cur_1: *mut BtCursor) -> i32 {
    let mut rc: i32 = 0;
    let mut skip_next: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_cur_1).e_state } as i32 == 4 {
        return unsafe { (*p_cur_1).skip_next };
    }
    unsafe { (*p_cur_1).e_state = 1 as u8 };
    if unsafe { sqlite3_fault_sim(410) } != 0 {
        rc = 10;
    } else {
        rc =
            btree_moveto(p_cur_1, unsafe { (*p_cur_1).p_key } as *const (),
                unsafe { (*p_cur_1).n_key }, 0, &mut skip_next);
    }
    if rc == 0 {
        unsafe { sqlite3_free(unsafe { (*p_cur_1).p_key }) };
        unsafe { (*p_cur_1).p_key = core::ptr::null_mut() };
        { let _ = 0; };
        if skip_next != 0 { unsafe { (*p_cur_1).skip_next = skip_next }; }
        if unsafe { (*p_cur_1).skip_next } != 0 &&
                unsafe { (*p_cur_1).e_state } as i32 == 0 {
            unsafe { (*p_cur_1).e_state = 2 as u8 };
        }
    }
    return rc;
}
extern "C" fn move_to_leftmost(p_cur_1: *mut BtCursor) -> i32 {
    let mut pgno: Pgno = 0 as Pgno;
    let mut rc: i32 = 0;
    let mut p_page: *const MemPage = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    while rc == 0 &&
            (unsafe {
                            (*{ p_page = unsafe { (*p_cur_1).p_page }; p_page }).leaf
                        } == 0) as i32 != 0 {
        { let _ = 0; };
        pgno =
            unsafe {
                sqlite3_get4byte(unsafe {
                            unsafe {
                                (*p_page).a_data.offset((unsafe { (*p_page).mask_page } as
                                                i32 &
                                            ((unsafe {
                                                                *unsafe {
                                                                        unsafe {
                                                                            (*p_page).a_cell_idx.offset((2 *
                                                                                            unsafe { (*p_cur_1).ix } as i32) as
                                                                                        isize).offset(0 as isize)
                                                                        }
                                                                    }
                                                            } as i32) << 8 |
                                                unsafe {
                                                        *unsafe {
                                                                unsafe {
                                                                    (*p_page).a_cell_idx.offset((2 *
                                                                                    unsafe { (*p_cur_1).ix } as i32) as
                                                                                isize).offset(1 as isize)
                                                                }
                                                            }
                                                    } as i32)) as isize)
                            }
                        } as *const u8)
            };
        rc = move_to_child(unsafe { &mut *p_cur_1 }, pgno);
    }
    return rc;
}
extern "C" fn move_to_parent(p_cur_1: &mut BtCursor) -> () {
    let mut p_leaf: *mut MemPage = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    (*p_cur_1).info.n_size = 0 as u16;
    (*p_cur_1).cur_flags &= !(2 | 4) as u8;
    (*p_cur_1).ix =
        (*p_cur_1).ai_idx[((*p_cur_1).i_page as i32 - 1) as usize];
    p_leaf = (*p_cur_1).p_page;
    (*p_cur_1).p_page =
        (*p_cur_1).ap_page[{
                    let __p = &mut (*p_cur_1).i_page;
                    *__p -= 1;
                    *__p
                } as usize];
    release_page_not_null(unsafe { &*p_leaf });
}
extern "C" fn btree_next(p_cur_1: *mut BtCursor) -> i32 {
    let mut rc: i32 = 0;
    let mut idx: i32 = 0;
    let mut p_page: *mut MemPage = core::ptr::null_mut();
    { let _ = 0; };
    if unsafe { (*p_cur_1).e_state } as i32 != 0 {
        { let _ = 0; };
        rc =
            if unsafe { (*p_cur_1).e_state } as i32 >= 3 {
                btree_restore_cursor_position(p_cur_1)
            } else { 0 };
        if rc != 0 { return rc; }
        if 1 == unsafe { (*p_cur_1).e_state } as i32 { return 101; }
        if unsafe { (*p_cur_1).e_state } as i32 == 2 {
            unsafe { (*p_cur_1).e_state = 0 as u8 };
            if unsafe { (*p_cur_1).skip_next } > 0 { return 0; }
        }
    }
    p_page = unsafe { (*p_cur_1).p_page };
    idx = { let __p = unsafe { &mut (*p_cur_1).ix }; *__p += 1; *__p } as i32;
    if unsafe { sqlite3_fault_sim(412) } != 0 {
        unsafe { (*p_page).is_init = 0 as u8 };
    }
    if (unsafe { (*p_page).is_init } == 0) as i32 != 0 {
        return unsafe { sqlite3_corrupt_error(6387) };
    }
    if idx >= unsafe { (*p_page).n_cell } as i32 {
        if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
            rc =
                move_to_child(unsafe { &mut *p_cur_1 },
                    unsafe {
                        sqlite3_get4byte(unsafe {
                                    &raw mut *unsafe {
                                                (*p_page).a_data.offset((unsafe { (*p_page).hdr_offset } as
                                                                i32 + 8) as isize)
                                            }
                                } as *const u8)
                    });
            if rc != 0 { return rc; }
            return move_to_leftmost(p_cur_1);
        }
        '__b49: loop {
            '__c49: loop {
                if unsafe { (*p_cur_1).i_page } as i32 == 0 {
                    unsafe { (*p_cur_1).e_state = 1 as u8 };
                    return 101;
                }
                move_to_parent(unsafe { &mut *p_cur_1 });
                p_page = unsafe { (*p_cur_1).p_page };
                break '__c49;
            }
            if !(unsafe { (*p_cur_1).ix } as i32 >=
                            unsafe { (*p_page).n_cell } as i32) {
                break '__b49;
            }
        }
        if unsafe { (*p_page).int_key } != 0 {
            return unsafe { sqlite3_btree_next(p_cur_1, 0) };
        } else { return 0; }
    }
    if unsafe { (*p_page).leaf } != 0 {
        return 0;
    } else { return move_to_leftmost(p_cur_1); }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_next(p_cur_1: *mut BtCursor, flags: i32)
    -> i32 {
    let mut p_page: *const MemPage = core::ptr::null();
    { let _ = flags; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { (*p_cur_1).info.n_size = 0 as u16 };
    unsafe { (*p_cur_1).cur_flags &= !(2 | 4) as u8 };
    if unsafe { (*p_cur_1).e_state } as i32 != 0 {
        return btree_next(p_cur_1);
    }
    p_page = unsafe { (*p_cur_1).p_page };
    if { let __p = unsafe { &mut (*p_cur_1).ix }; *__p += 1; *__p } as i32 >=
            unsafe { (*p_page).n_cell } as i32 {
        {
            let __p = unsafe { &mut (*p_cur_1).ix };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        return btree_next(p_cur_1);
    }
    if unsafe { (*p_page).leaf } != 0 {
        return 0;
    } else { return move_to_leftmost(p_cur_1); }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_table_moveto(p_cur_1: *mut BtCursor,
    int_key_1: i64, bias_right_1: i32, p_res_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    let mut lwr: i32 = 0;
    let mut upr: i32 = 0;
    let mut idx: i32 = 0;
    let mut c: i32 = 0;
    let mut chld_pg: Pgno = 0 as Pgno;
    let mut p_page: *const MemPage = core::ptr::null();
    let mut p_cell: *mut u8 = core::ptr::null_mut();
    let mut n_cell_key: i64 = 0 as i64;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s51:
            {
            match __state {
                0 => { __state = 4; }
                2 => {
                    if lwr >= unsafe { (*p_page).n_cell } as i32 {
                        __state = 96;
                    } else { __state = 97; }
                }
                3 => {
                    unsafe { (*p_cur_1).info.n_size = 0 as u16 };
                    __state = 101;
                }
                4 => { { let _ = 0; }; __state = 5; }
                5 => { { let _ = 0; }; __state = 6; }
                6 => { { let _ = 0; }; __state = 7; }
                7 => { { let _ = 0; }; __state = 8; }
                8 => { { let _ = 0; }; __state = 9; }
                9 => {
                    if unsafe { (*p_cur_1).e_state } as i32 == 0 &&
                            unsafe { (*p_cur_1).cur_flags } as i32 & 2 != 0 {
                        __state = 11;
                    } else { __state = 10; }
                }
                10 => { rc = move_to_root(p_cur_1); __state = 28; }
                11 => {
                    if unsafe { (*p_cur_1).info.n_key } == int_key_1 {
                        __state = 13;
                    } else { __state = 12; }
                }
                12 => {
                    if unsafe { (*p_cur_1).info.n_key } < int_key_1 {
                        __state = 15;
                    } else { __state = 10; }
                }
                13 => { *p_res_1 = 0; __state = 14; }
                14 => { return 0; }
                15 => {
                    if unsafe { (*p_cur_1).cur_flags } as i32 & 8 != 0 {
                        __state = 17;
                    } else { __state = 16; }
                }
                16 => {
                    if unsafe { (*p_cur_1).info.n_key } + 1 as i64 == int_key_1
                        {
                        __state = 20;
                    } else { __state = 10; }
                }
                17 => { { let _ = 0; }; __state = 18; }
                18 => { *p_res_1 = -1; __state = 19; }
                19 => { return 0; }
                20 => { *p_res_1 = 0; __state = 21; }
                21 => {
                    rc = unsafe { sqlite3_btree_next(p_cur_1, 0) };
                    __state = 22;
                }
                22 => { if rc == 0 { __state = 23; } else { __state = 24; } }
                23 => {
                    get_cell_info(unsafe { &mut *p_cur_1 });
                    __state = 25;
                }
                24 => {
                    if rc != 101 { __state = 27; } else { __state = 10; }
                }
                25 => {
                    if unsafe { (*p_cur_1).info.n_key } == int_key_1 {
                        __state = 26;
                    } else { __state = 10; }
                }
                26 => { return 0; }
                27 => { return rc; }
                28 => { if rc != 0 { __state = 30; } else { __state = 29; } }
                29 => { { let _ = 0; }; __state = 35; }
                30 => { if rc == 16 { __state = 32; } else { __state = 31; } }
                31 => { return rc; }
                32 => { { let _ = 0; }; __state = 33; }
                33 => { *p_res_1 = -1; __state = 34; }
                34 => { return 0; }
                35 => { { let _ = 0; }; __state = 36; }
                36 => { { let _ = 0; }; __state = 37; }
                37 => { { let _ = 0; }; __state = 38; }
                38 => { { let _ = 0; }; __state = 39; }
                39 => { { let _ = 0; }; __state = 40; }
                40 => { __state = 42; }
                41 => { __state = 3; }
                42 => { __state = 43; }
                43 => { __state = 45; }
                44 => { __state = 42; }
                45 => { __state = 46; }
                46 => {
                    p_page = unsafe { (*p_cur_1).p_page } as *const MemPage;
                    __state = 47;
                }
                47 => { __state = 48; }
                48 => { { let _ = 0; }; __state = 49; }
                49 => { { let _ = 0; }; __state = 50; }
                50 => { lwr = 0; __state = 51; }
                51 => {
                    upr = unsafe { (*p_page).n_cell } as i32 - 1;
                    __state = 52;
                }
                52 => { { let _ = 0; }; __state = 53; }
                53 => { idx = upr >> 1 - bias_right_1; __state = 54; }
                54 => { __state = 56; }
                55 => { { let _ = 0; }; __state = 87; }
                56 => { __state = 57; }
                57 => { __state = 59; }
                58 => { __state = 56; }
                59 => {
                    p_cell =
                        unsafe {
                            unsafe {
                                (*p_page).a_data_ofst.offset((unsafe { (*p_page).mask_page }
                                                as i32 &
                                            ((unsafe {
                                                                *unsafe {
                                                                        unsafe {
                                                                            (*p_page).a_cell_idx.offset((2 * idx) as
                                                                                        isize).offset(0 as isize)
                                                                        }
                                                                    }
                                                            } as i32) << 8 |
                                                unsafe {
                                                        *unsafe {
                                                                unsafe {
                                                                    (*p_page).a_cell_idx.offset((2 * idx) as
                                                                                isize).offset(1 as isize)
                                                                }
                                                            }
                                                    } as i32)) as isize)
                            }
                        };
                    __state = 60;
                }
                60 => {
                    if unsafe { (*p_page).int_key_leaf } != 0 {
                        __state = 62;
                    } else { __state = 61; }
                }
                61 => {
                    unsafe {
                        sqlite3_get_varint(p_cell as *const u8,
                            &raw mut n_cell_key as *mut u64)
                    };
                    __state = 65;
                }
                62 => {
                    if 128 <=
                            unsafe {
                                    *{
                                            let __p = &mut p_cell;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                } as i32 {
                        __state = 63;
                    } else { __state = 61; }
                }
                63 => {
                    if p_cell >= unsafe { (*p_page).a_data_end } {
                        __state = 64;
                    } else { __state = 62; }
                }
                64 => { return unsafe { sqlite3_corrupt_error(5927) }; }
                65 => {
                    if n_cell_key < int_key_1 {
                        __state = 67;
                    } else { __state = 68; }
                }
                66 => { { let _ = 0; }; __state = 86; }
                67 => { lwr = idx + 1; __state = 69; }
                68 => {
                    if n_cell_key > int_key_1 {
                        __state = 72;
                    } else { __state = 73; }
                }
                69 => {
                    if lwr > upr { __state = 70; } else { __state = 66; }
                }
                70 => { c = -1; __state = 71; }
                71 => { __state = 55; }
                72 => { upr = idx - 1; __state = 74; }
                73 => { { let _ = 0; }; __state = 77; }
                74 => {
                    if lwr > upr { __state = 75; } else { __state = 66; }
                }
                75 => { c = 1; __state = 76; }
                76 => { __state = 55; }
                77 => { unsafe { (*p_cur_1).ix = idx as u16 }; __state = 78; }
                78 => {
                    if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
                        __state = 79;
                    } else { __state = 80; }
                }
                79 => { lwr = idx; __state = 81; }
                80 => {
                    unsafe { (*p_cur_1).cur_flags |= 2 as u8 };
                    __state = 82;
                }
                81 => { __state = 2; }
                82 => {
                    unsafe { (*p_cur_1).info.n_key = n_cell_key };
                    __state = 83;
                }
                83 => {
                    unsafe { (*p_cur_1).info.n_size = 0 as u16 };
                    __state = 84;
                }
                84 => { *p_res_1 = 0; __state = 85; }
                85 => { return 0; }
                86 => { idx = lwr + upr >> 1; __state = 58; }
                87 => { { let _ = 0; }; __state = 88; }
                88 => {
                    if unsafe { (*p_page).leaf } != 0 {
                        __state = 90;
                    } else { __state = 89; }
                }
                89 => { __state = 2; }
                90 => { { let _ = 0; }; __state = 91; }
                91 => { unsafe { (*p_cur_1).ix = idx as u16 }; __state = 92; }
                92 => { *p_res_1 = c; __state = 93; }
                93 => { rc = 0; __state = 94; }
                94 => { __state = 3; }
                95 => { unsafe { (*p_cur_1).ix = lwr as u16 }; __state = 98; }
                96 => {
                    chld_pg =
                        unsafe {
                            sqlite3_get4byte(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_page).a_data.offset((unsafe { (*p_page).hdr_offset } as
                                                                    i32 + 8) as isize)
                                                }
                                    } as *const u8)
                        };
                    __state = 95;
                }
                97 => {
                    chld_pg =
                        unsafe {
                            sqlite3_get4byte(unsafe {
                                        unsafe {
                                            (*p_page).a_data.offset((unsafe { (*p_page).mask_page } as
                                                            i32 &
                                                        ((unsafe {
                                                                            *unsafe {
                                                                                    unsafe {
                                                                                        (*p_page).a_cell_idx.offset((2 * lwr) as
                                                                                                    isize).offset(0 as isize)
                                                                                    }
                                                                                }
                                                                        } as i32) << 8 |
                                                            unsafe {
                                                                    *unsafe {
                                                                            unsafe {
                                                                                (*p_page).a_cell_idx.offset((2 * lwr) as
                                                                                            isize).offset(1 as isize)
                                                                            }
                                                                        }
                                                                } as i32)) as isize)
                                        }
                                    } as *const u8)
                        };
                    __state = 95;
                }
                98 => {
                    rc = move_to_child(unsafe { &mut *p_cur_1 }, chld_pg);
                    __state = 99;
                }
                99 => { if rc != 0 { __state = 100; } else { __state = 44; } }
                100 => { __state = 41; }
                101 => { { let _ = 0; }; __state = 102; }
                102 => { return rc; }
                _ => {}
            }
        }
    }
    unreachable!();
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_has_moved(p_cur_1: *mut BtCursor)
    -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    return (0 != unsafe { *(p_cur_1 as *mut u8) } as i32) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_restore(p_cur_1: *mut BtCursor,
    p_different_row_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    rc =
        if unsafe { (*p_cur_1).e_state } as i32 >= 3 {
            btree_restore_cursor_position(p_cur_1)
        } else { 0 };
    if rc != 0 { *p_different_row_1 = 1; return rc; }
    if unsafe { (*p_cur_1).e_state } as i32 != 0 {
        *p_different_row_1 = 1;
    } else { *p_different_row_1 = 0; }
    return 0;
}
extern "C" fn btree_compute_free_space(p_page_1: &mut MemPage) -> i32 {
    let mut pc: i32 = 0;
    let mut hdr: u8 = 0 as u8;
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut usable_size: i32 = 0;
    let mut n_free: i32 = 0;
    let mut top: i32 = 0;
    let mut i_cell_first: i32 = 0;
    let mut i_cell_last: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    usable_size = unsafe { (*(*p_page_1).p_bt).usable_size } as i32;
    hdr = (*p_page_1).hdr_offset;
    data = (*p_page_1).a_data;
    top =
        (((unsafe {
                                            *unsafe {
                                                    data.offset((hdr as i32 + 5) as isize).offset(0 as isize)
                                                }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe {
                                            data.offset((hdr as i32 + 5) as isize).offset(1 as isize)
                                        }
                                } as i32) as i32 - 1 & 65535) + 1;
    i_cell_first =
        hdr as i32 + 8 + (*p_page_1).child_ptr_size as i32 +
            2 * (*p_page_1).n_cell as i32;
    i_cell_last = usable_size - 4;
    pc =
        (unsafe {
                            *unsafe {
                                    data.offset((hdr as i32 + 1) as isize).offset(0 as isize)
                                }
                        } as i32) << 8 |
            unsafe {
                    *unsafe {
                            data.offset((hdr as i32 + 1) as isize).offset(1 as isize)
                        }
                } as i32;
    n_free = unsafe { *data.offset((hdr as i32 + 7) as isize) } as i32 + top;
    if pc > 0 {
        let mut next: u32 = 0 as u32;
        let mut size: u32 = 0 as u32;
        if pc < top { return unsafe { sqlite3_corrupt_error(2159) }; }
        loop {
            if pc > i_cell_last {
                return unsafe { sqlite3_corrupt_error(2164) };
            }
            next =
                ((unsafe {
                                        *unsafe { data.offset(pc as isize).offset(0 as isize) }
                                    } as i32) << 8 |
                        unsafe {
                                *unsafe { data.offset(pc as isize).offset(1 as isize) }
                            } as i32) as u32;
            size =
                ((unsafe {
                                        *unsafe {
                                                data.offset((pc + 2) as isize).offset(0 as isize)
                                            }
                                    } as i32) << 8 |
                        unsafe {
                                *unsafe {
                                        data.offset((pc + 2) as isize).offset(1 as isize)
                                    }
                            } as i32) as u32;
            if size < 4 as u32 && unsafe { sqlite3_fault_sim(422) } == 0 {
                return unsafe { sqlite3_corrupt_error(2173) };
            }
            n_free = (n_free as u32 + size) as i32;
            if next < pc as u32 + size + 4 as u32 { break; }
            pc = next as i32;
        }
        if next > 0 as u32 { return unsafe { sqlite3_corrupt_error(2181) }; }
        if pc as u32 + size > usable_size as u32 {
            return unsafe { sqlite3_corrupt_error(2185) };
        }
    }
    if n_free > usable_size || n_free < i_cell_first {
        return unsafe { sqlite3_corrupt_error(2197) };
    }
    (*p_page_1).n_free = (n_free - i_cell_first) as u16 as i32;
    return 0;
}
extern "C" fn move_to_rightmost(p_cur_1: *mut BtCursor) -> i32 {
    let mut pgno: Pgno = 0 as Pgno;
    let mut rc: i32 = 0;
    let mut p_page: *const MemPage = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    while (unsafe {
                        (*{ p_page = unsafe { (*p_cur_1).p_page }; p_page }).leaf
                    } == 0) as i32 != 0 {
        pgno =
            unsafe {
                sqlite3_get4byte(unsafe {
                            &raw mut *unsafe {
                                        (*p_page).a_data.offset((unsafe { (*p_page).hdr_offset } as
                                                        i32 + 8) as isize)
                                    }
                        } as *const u8)
            };
        unsafe { (*p_cur_1).ix = unsafe { (*p_page).n_cell } };
        rc = move_to_child(unsafe { &mut *p_cur_1 }, pgno);
        if rc != 0 { return rc; }
    }
    unsafe {
        (*p_cur_1).ix = (unsafe { (*p_page).n_cell } as i32 - 1) as u16
    };
    { let _ = 0; };
    { let _ = 0; };
    return 0;
}
extern "C" fn btree_previous(p_cur_1: *mut BtCursor) -> i32 {
    let mut rc: i32 = 0;
    let mut p_page: *mut MemPage = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_cur_1).e_state } as i32 != 0 {
        rc =
            if unsafe { (*p_cur_1).e_state } as i32 >= 3 {
                btree_restore_cursor_position(p_cur_1)
            } else { 0 };
        if rc != 0 { return rc; }
        if 1 == unsafe { (*p_cur_1).e_state } as i32 { return 101; }
        if 2 == unsafe { (*p_cur_1).e_state } as i32 {
            unsafe { (*p_cur_1).e_state = 0 as u8 };
            if unsafe { (*p_cur_1).skip_next } < 0 { return 0; }
        }
    }
    p_page = unsafe { (*p_cur_1).p_page };
    if unsafe { sqlite3_fault_sim(412) } != 0 {
        unsafe { (*p_page).is_init = 0 as u8 };
    }
    if (unsafe { (*p_page).is_init } == 0) as i32 != 0 {
        return unsafe { sqlite3_corrupt_error(6480) };
    }
    if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
        let idx: i32 = unsafe { (*p_cur_1).ix } as i32;
        rc =
            move_to_child(unsafe { &mut *p_cur_1 },
                unsafe {
                    sqlite3_get4byte(unsafe {
                                unsafe {
                                    (*p_page).a_data.offset((unsafe { (*p_page).mask_page } as
                                                    i32 &
                                                ((unsafe {
                                                                    *unsafe {
                                                                            unsafe {
                                                                                (*p_page).a_cell_idx.offset((2 * idx) as
                                                                                            isize).offset(0 as isize)
                                                                            }
                                                                        }
                                                                } as i32) << 8 |
                                                    unsafe {
                                                            *unsafe {
                                                                    unsafe {
                                                                        (*p_page).a_cell_idx.offset((2 * idx) as
                                                                                    isize).offset(1 as isize)
                                                                    }
                                                                }
                                                        } as i32)) as isize)
                                }
                            } as *const u8)
                });
        if rc != 0 { return rc; }
        rc = move_to_rightmost(p_cur_1);
    } else {
        while unsafe { (*p_cur_1).ix } as i32 == 0 {
            if unsafe { (*p_cur_1).i_page } as i32 == 0 {
                unsafe { (*p_cur_1).e_state = 1 as u8 };
                return 101;
            }
            move_to_parent(unsafe { &mut *p_cur_1 });
        }
        { let _ = 0; };
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p_cur_1).ix };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        p_page = unsafe { (*p_cur_1).p_page };
        if unsafe { (*p_page).int_key } != 0 &&
                (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
            rc = unsafe { sqlite3_btree_previous(p_cur_1, 0) };
        } else { rc = 0; }
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_previous(p_cur_1: *mut BtCursor, flags: i32)
    -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = flags; };
    unsafe { (*p_cur_1).cur_flags &= !(8 | 4 | 2) as u8 };
    unsafe { (*p_cur_1).info.n_size = 0 as u16 };
    if unsafe { (*p_cur_1).e_state } as i32 != 0 ||
                unsafe { (*p_cur_1).ix } as i32 == 0 ||
            unsafe { (*unsafe { (*p_cur_1).p_page }).leaf } as i32 == 0 {
        return btree_previous(p_cur_1);
    }
    {
        let __p = unsafe { &mut (*p_cur_1).ix };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    return 0;
}
extern "C" fn free_space(p_page_1: &mut MemPage, mut i_start_1: i32,
    mut i_size_1: i32) -> i32 {
    let mut i_ptr: i32 = 0;
    let mut i_free_blk: i32 = 0;
    let mut hdr: u8 = 0 as u8;
    let mut n_frag: i32 = 0;
    let i_orig_size: i32 = i_size_1;
    let mut x: i32 = 0;
    let mut i_end: i32 = i_start_1 + i_size_1;
    let data: *mut u8 = (*p_page_1).a_data;
    let mut p_tmp: *const u8 = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    hdr = (*p_page_1).hdr_offset;
    i_ptr = hdr as i32 + 1;
    if unsafe { *data.offset((i_ptr + 1) as isize) } as i32 == 0 &&
            unsafe { *data.offset(i_ptr as isize) } as i32 == 0 {
        i_free_blk = 0;
    } else {
        while {
                    i_free_blk =
                        (unsafe {
                                            *unsafe { data.offset(i_ptr as isize).offset(0 as isize) }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe { data.offset(i_ptr as isize).offset(1 as isize) }
                                } as i32;
                    i_free_blk
                } < i_start_1 {
            if i_free_blk <= i_ptr {
                if i_free_blk == 0 { break; }
                return unsafe { sqlite3_corrupt_error(1975) };
            }
            i_ptr = i_free_blk;
        }
        if i_free_blk > unsafe { (*(*p_page_1).p_bt).usable_size } as i32 - 4
            {
            return unsafe { sqlite3_corrupt_error(1980) };
        }
        { let _ = 0; };
        if i_free_blk != 0 && i_end + 3 >= i_free_blk {
            n_frag = i_free_blk - i_end;
            if i_end > i_free_blk {
                return unsafe { sqlite3_corrupt_error(1992) };
            }
            i_end =
                i_free_blk +
                    ((unsafe {
                                        *unsafe {
                                                data.offset((i_free_blk + 2) as isize).offset(0 as isize)
                                            }
                                    } as i32) << 8 |
                        unsafe {
                                *unsafe {
                                        data.offset((i_free_blk + 2) as isize).offset(1 as isize)
                                    }
                            } as i32);
            if i_end > unsafe { (*(*p_page_1).p_bt).usable_size } as i32 {
                return unsafe { sqlite3_corrupt_error(1995) };
            }
            i_size_1 = i_end - i_start_1;
            i_free_blk =
                (unsafe {
                                    *unsafe {
                                            data.offset(i_free_blk as isize).offset(0 as isize)
                                        }
                                } as i32) << 8 |
                    unsafe {
                            *unsafe {
                                    data.offset(i_free_blk as isize).offset(1 as isize)
                                }
                        } as i32;
        }
        if i_ptr > hdr as i32 + 1 {
            let i_ptr_end: i32 =
                i_ptr +
                    ((unsafe {
                                        *unsafe {
                                                data.offset((i_ptr + 2) as isize).offset(0 as isize)
                                            }
                                    } as i32) << 8 |
                        unsafe {
                                *unsafe {
                                        data.offset((i_ptr + 2) as isize).offset(1 as isize)
                                    }
                            } as i32);
            if i_ptr_end + 3 >= i_start_1 {
                if i_ptr_end > i_start_1 {
                    return unsafe { sqlite3_corrupt_error(2008) };
                }
                n_frag += i_start_1 - i_ptr_end;
                i_size_1 = i_end - i_ptr;
                i_start_1 = i_ptr;
            }
        }
        if n_frag > unsafe { *data.offset((hdr as i32 + 7) as isize) } as i32
            {
            return unsafe { sqlite3_corrupt_error(2014) };
        }
        unsafe {
            *data.offset((hdr as i32 + 7) as isize) -=
                n_frag as u8 as i32 as u8
        };
    }
    p_tmp = unsafe { data.offset((hdr as i32 + 5) as isize) };
    x =
        (unsafe { *p_tmp.offset(0 as isize) } as i32) << 8 |
            unsafe { *p_tmp.offset(1 as isize) } as i32;
    if unsafe { (*(*p_page_1).p_bt).bts_flags } as i32 & 12 != 0 {
        unsafe {
            memset(unsafe { &raw mut *data.offset(i_start_1 as isize) } as
                    *mut (), 0, i_size_1 as u64)
        };
    }
    if i_start_1 <= x {
        if i_start_1 < x { return unsafe { sqlite3_corrupt_error(2028) }; }
        if i_ptr != hdr as i32 + 1 {
            return unsafe { sqlite3_corrupt_error(2029) };
        }
        {
            unsafe {
                *unsafe {
                            data.offset((hdr as i32 + 1) as isize).offset(0 as isize)
                        } = (i_free_blk >> 8) as u8
            };
            unsafe {
                *unsafe {
                            data.offset((hdr as i32 + 1) as isize).offset(1 as isize)
                        } = i_free_blk as u8
            }
        };
        {
            unsafe {
                *unsafe {
                            data.offset((hdr as i32 + 5) as isize).offset(0 as isize)
                        } = (i_end >> 8) as u8
            };
            unsafe {
                *unsafe {
                            data.offset((hdr as i32 + 5) as isize).offset(1 as isize)
                        } = i_end as u8
            }
        };
    } else {
        {
            unsafe {
                *unsafe { data.offset(i_ptr as isize).offset(0 as isize) } =
                    (i_start_1 >> 8) as u8
            };
            unsafe {
                *unsafe { data.offset(i_ptr as isize).offset(1 as isize) } =
                    i_start_1 as u8
            }
        };
        {
            unsafe {
                *unsafe { data.offset(i_start_1 as isize).offset(0 as isize) }
                    = (i_free_blk >> 8) as u8
            };
            unsafe {
                *unsafe { data.offset(i_start_1 as isize).offset(1 as isize) }
                    = i_free_blk as u8
            }
        };
        { let _ = 0; };
        {
            unsafe {
                *unsafe {
                            data.offset((i_start_1 + 2) as isize).offset(0 as isize)
                        } = (i_size_1 as u16 as i32 >> 8) as u8
            };
            unsafe {
                *unsafe {
                            data.offset((i_start_1 + 2) as isize).offset(1 as isize)
                        } = i_size_1 as u16 as u8
            }
        };
    }
    (*p_page_1).n_free += i_orig_size;
    return 0;
}
extern "C" fn drop_cell(p_page_1: *mut MemPage, idx: i32, sz: i32,
    p_rc_1: &mut i32) -> () {
    let mut pc: u32 = 0 as u32;
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut ptr: *mut u8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut hdr: i32 = 0;
    if *p_rc_1 != 0 { return; }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    data = unsafe { (*p_page_1).a_data };
    ptr =
        unsafe {
            unsafe { (*p_page_1).a_cell_idx.offset((2 * idx) as isize) }
        };
    { let _ = 0; };
    pc =
        ((unsafe { *ptr.offset(0 as isize) } as i32) << 8 |
                unsafe { *ptr.offset(1 as isize) } as i32) as u32;
    hdr = unsafe { (*p_page_1).hdr_offset } as i32;
    if pc + sz as u32 > unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size }
        {
        *p_rc_1 = unsafe { sqlite3_corrupt_error(7321) };
        return;
    }
    rc = free_space(unsafe { &mut *p_page_1 }, pc as i32, sz);
    if rc != 0 { *p_rc_1 = rc; return; }
    {
        let __p = unsafe { &mut (*p_page_1).n_cell };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    if unsafe { (*p_page_1).n_cell } as i32 == 0 {
        unsafe {
            memset(unsafe { &raw mut *data.offset((hdr + 1) as isize) } as
                    *mut (), 0, 4 as u64)
        };
        unsafe { *data.offset((hdr + 7) as isize) = 0 as u8 };
        {
            unsafe {
                *unsafe { data.offset((hdr + 5) as isize).offset(0 as isize) }
                    =
                    (unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } >> 8)
                        as u8
            };
            unsafe {
                *unsafe { data.offset((hdr + 5) as isize).offset(1 as isize) }
                    =
                    unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } as u8
            }
        };
        unsafe {
            (*p_page_1).n_free =
                (unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } -
                                unsafe { (*p_page_1).hdr_offset } as u32 -
                            unsafe { (*p_page_1).child_ptr_size } as u32 - 8 as u32) as
                    i32
        };
    } else {
        unsafe {
            memmove(ptr as *mut (),
                unsafe { ptr.offset(2 as isize) } as *const (),
                (2 * (unsafe { (*p_page_1).n_cell } as i32 - idx)) as u64)
        };
        {
            unsafe {
                *unsafe { data.offset((hdr + 3) as isize).offset(0 as isize) }
                    = (unsafe { (*p_page_1).n_cell } as i32 >> 8) as u8
            };
            unsafe {
                *unsafe { data.offset((hdr + 3) as isize).offset(1 as isize) }
                    = unsafe { (*p_page_1).n_cell } as u8
            }
        };
        unsafe { (*p_page_1).n_free += 2 };
    }
}
extern "C" fn page_find_slot(p_pg_1: &MemPage, n_byte_1: i32,
    p_rc_1: &mut i32) -> *mut u8 {
    let hdr: i32 = (*p_pg_1).hdr_offset as i32;
    let a_data: *mut u8 = (*p_pg_1).a_data;
    let mut i_addr: i32 = hdr + 1;
    let mut p_tmp: *const u8 =
        unsafe { &raw mut *a_data.offset(i_addr as isize) } as *const u8;
    let mut pc: i32 =
        (unsafe { *p_tmp.offset(0 as isize) } as i32) << 8 |
            unsafe { *p_tmp.offset(1 as isize) } as i32;
    let mut x: i32 = 0;
    let max_pc: i32 =
        (unsafe { (*(*p_pg_1).p_bt).usable_size } - n_byte_1 as u32) as i32;
    let mut size: i32 = 0;
    { let _ = 0; };
    while pc <= max_pc {
        p_tmp = unsafe { a_data.offset((pc + 2) as isize) };
        size =
            (unsafe { *p_tmp.offset(0 as isize) } as i32) << 8 |
                unsafe { *p_tmp.offset(1 as isize) } as i32;
        if { x = size - n_byte_1; x } >= 0 {
            if x < 4 {
                if unsafe { *a_data.offset((hdr + 7) as isize) } as i32 > 57 {
                    return core::ptr::null_mut();
                }
                unsafe {
                    memcpy(unsafe { &raw mut *a_data.offset(i_addr as isize) }
                            as *mut (),
                        unsafe { &raw mut *a_data.offset(pc as isize) } as
                            *const (), 2 as u64)
                };
                unsafe {
                    *a_data.offset((hdr + 7) as isize) += x as u8 as i32 as u8
                };
                return unsafe { &mut *a_data.offset(pc as isize) };
            } else if x + pc > max_pc {
                *p_rc_1 = unsafe { sqlite3_corrupt_error(1806) };
                return core::ptr::null_mut();
            } else {
                {
                    unsafe {
                        *unsafe {
                                    a_data.offset((pc + 2) as isize).offset(0 as isize)
                                } = (x >> 8) as u8
                    };
                    unsafe {
                        *unsafe {
                                    a_data.offset((pc + 2) as isize).offset(1 as isize)
                                } = x as u8
                    }
                };
            }
            return unsafe { &mut *a_data.offset((pc + x) as isize) };
        }
        i_addr = pc;
        p_tmp = unsafe { a_data.offset(pc as isize) };
        pc =
            (unsafe { *p_tmp.offset(0 as isize) } as i32) << 8 |
                unsafe { *p_tmp.offset(1 as isize) } as i32;
        if pc <= i_addr {
            if pc != 0 { *p_rc_1 = unsafe { sqlite3_corrupt_error(1821) }; }
            return core::ptr::null_mut();
        }
    }
    if pc > max_pc + n_byte_1 - 4 {
        *p_rc_1 = unsafe { sqlite3_corrupt_error(1828) };
    }
    return core::ptr::null_mut();
}
extern "C" fn defragment_page(p_page_1: *mut MemPage, n_max_frag_1: i32)
    -> i32 {
    let mut i: i32 = 0;
    let mut pc: i32 = 0;
    let mut hdr: i32 = 0;
    let mut size: i32 = 0;
    let mut usable_size: i32 = 0;
    let mut cell_offset: i32 = 0;
    let mut cbrk: i32 = 0;
    let mut n_cell: i32 = 0;
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut temp: *mut u8 = core::ptr::null_mut();
    let mut src: *mut u8 = core::ptr::null_mut();
    let mut i_cell_first: i32 = 0;
    let mut i_cell_last: i32 = 0;
    let mut i_cell_start: i32 = 0;
    let mut i_free: i32 = 0;
    let mut i_free2: i32 = 0;
    let mut p_end: *mut u8 = core::ptr::null_mut();
    let mut p_addr: *mut u8 = core::ptr::null_mut();
    let mut sz2: i32 = 0;
    let mut sz: i32 = 0;
    let mut top: i32 = 0;
    let mut p_addr_1: *mut u8 = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s58:
            {
            match __state {
                0 => { __state = 3; }
                2 => { { let _ = 0; }; __state = 95; }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { __state = 9; }
                9 => { __state = 10; }
                10 => { __state = 11; }
                11 => { __state = 12; }
                12 => { __state = 13; }
                13 => { __state = 14; }
                14 => { __state = 15; }
                15 => { __state = 16; }
                16 => { { let _ = 0; }; __state = 17; }
                17 => { { let _ = 0; }; __state = 18; }
                18 => { { let _ = 0; }; __state = 19; }
                19 => { { let _ = 0; }; __state = 20; }
                20 => { { let _ = 0; }; __state = 21; }
                21 => { data = unsafe { (*p_page_1).a_data }; __state = 22; }
                22 => {
                    hdr = unsafe { (*p_page_1).hdr_offset } as i32;
                    __state = 23;
                }
                23 => {
                    cell_offset = unsafe { (*p_page_1).cell_offset } as i32;
                    __state = 24;
                }
                24 => {
                    n_cell = unsafe { (*p_page_1).n_cell } as i32;
                    __state = 25;
                }
                25 => { { let _ = 0; }; __state = 26; }
                26 => {
                    i_cell_first = cell_offset + 2 * n_cell;
                    __state = 27;
                }
                27 => {
                    usable_size =
                        unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } as
                            i32;
                    __state = 28;
                }
                28 => {
                    if unsafe { *data.offset((hdr + 7) as isize) } as i32 <=
                            n_max_frag_1 {
                        __state = 30;
                    } else { __state = 29; }
                }
                29 => { cbrk = usable_size; __state = 67; }
                30 => {
                    i_free =
                        (unsafe {
                                            *unsafe {
                                                    data.offset((hdr + 1) as isize).offset(0 as isize)
                                                }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe {
                                            data.offset((hdr + 1) as isize).offset(1 as isize)
                                        }
                                } as i32;
                    __state = 31;
                }
                31 => {
                    if i_free > usable_size - 4 {
                        __state = 33;
                    } else { __state = 32; }
                }
                32 => {
                    if i_free != 0 { __state = 34; } else { __state = 29; }
                }
                33 => { return unsafe { sqlite3_corrupt_error(1676) }; }
                34 => {
                    i_free2 =
                        (unsafe {
                                            *unsafe { data.offset(i_free as isize).offset(0 as isize) }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe { data.offset(i_free as isize).offset(1 as isize) }
                                } as i32;
                    __state = 35;
                }
                35 => {
                    if i_free2 > usable_size - 4 {
                        __state = 37;
                    } else { __state = 36; }
                }
                36 => {
                    if 0 == i_free2 ||
                            unsafe { *data.offset(i_free2 as isize) } as i32 == 0 &&
                                unsafe { *data.offset((i_free2 + 1) as isize) } as i32 == 0
                        {
                        __state = 38;
                    } else { __state = 29; }
                }
                37 => { return unsafe { sqlite3_corrupt_error(1679) }; }
                38 => {
                    p_end =
                        unsafe { data.offset((cell_offset + n_cell * 2) as isize) };
                    __state = 39;
                }
                39 => { __state = 40; }
                40 => { sz2 = 0; __state = 41; }
                41 => {
                    sz =
                        (unsafe {
                                            *unsafe {
                                                    data.offset((i_free + 2) as isize).offset(0 as isize)
                                                }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe {
                                            data.offset((i_free + 2) as isize).offset(1 as isize)
                                        }
                                } as i32;
                    __state = 42;
                }
                42 => {
                    top =
                        (unsafe {
                                            *unsafe {
                                                    data.offset((hdr + 5) as isize).offset(0 as isize)
                                                }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe {
                                            data.offset((hdr + 5) as isize).offset(1 as isize)
                                        }
                                } as i32;
                    __state = 43;
                }
                43 => {
                    if top >= i_free { __state = 45; } else { __state = 44; }
                }
                44 => {
                    if i_free2 != 0 { __state = 47; } else { __state = 48; }
                }
                45 => { return unsafe { sqlite3_corrupt_error(1687) }; }
                46 => { cbrk = top + sz; __state = 56; }
                47 => {
                    if i_free + sz > i_free2 {
                        __state = 50;
                    } else { __state = 49; }
                }
                48 => {
                    if i_free + sz > usable_size {
                        __state = 55;
                    } else { __state = 46; }
                }
                49 => {
                    sz2 =
                        (unsafe {
                                            *unsafe {
                                                    data.offset((i_free2 + 2) as isize).offset(0 as isize)
                                                }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe {
                                            data.offset((i_free2 + 2) as isize).offset(1 as isize)
                                        }
                                } as i32;
                    __state = 51;
                }
                50 => { return unsafe { sqlite3_corrupt_error(1690) }; }
                51 => {
                    if i_free2 + sz2 > usable_size {
                        __state = 53;
                    } else { __state = 52; }
                }
                52 => {
                    unsafe {
                        memmove(unsafe {
                                    &raw mut *data.offset((i_free + sz + sz2) as isize)
                                } as *mut (),
                            unsafe { &raw mut *data.offset((i_free + sz) as isize) } as
                                *const (), (i_free2 - (i_free + sz)) as u64)
                    };
                    __state = 54;
                }
                53 => { return unsafe { sqlite3_corrupt_error(1692) }; }
                54 => { sz += sz2; __state = 46; }
                55 => { return unsafe { sqlite3_corrupt_error(1696) }; }
                56 => { { let _ = 0; }; __state = 57; }
                57 => {
                    unsafe {
                        memmove(unsafe { &raw mut *data.offset(cbrk as isize) } as
                                *mut (),
                            unsafe { &raw mut *data.offset(top as isize) } as *const (),
                            (i_free - top) as u64)
                    };
                    __state = 58;
                }
                58 => {
                    p_addr = unsafe { data.offset(cell_offset as isize) };
                    __state = 60;
                }
                59 => { __state = 2; }
                60 => {
                    if p_addr < p_end { __state = 61; } else { __state = 59; }
                }
                61 => {
                    pc =
                        (unsafe { *p_addr.offset(0 as isize) } as i32) << 8 |
                            unsafe { *p_addr.offset(1 as isize) } as i32;
                    __state = 63;
                }
                62 => {
                    {
                        let __n = 2;
                        let __p = &mut p_addr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 60;
                }
                63 => {
                    if pc < i_free { __state = 64; } else { __state = 65; }
                }
                64 => {
                    {
                        unsafe {
                            *p_addr.offset(0 as isize) = (pc + sz >> 8) as u8
                        };
                        unsafe { *p_addr.offset(1 as isize) = (pc + sz) as u8 }
                    };
                    __state = 62;
                }
                65 => {
                    if pc < i_free2 { __state = 66; } else { __state = 62; }
                }
                66 => {
                    {
                        unsafe {
                            *p_addr.offset(0 as isize) = (pc + sz2 >> 8) as u8
                        };
                        unsafe { *p_addr.offset(1 as isize) = (pc + sz2) as u8 }
                    };
                    __state = 62;
                }
                67 => { i_cell_last = usable_size - 4; __state = 68; }
                68 => {
                    i_cell_start =
                        (unsafe {
                                            *unsafe {
                                                    data.offset((hdr + 5) as isize).offset(0 as isize)
                                                }
                                        } as i32) << 8 |
                            unsafe {
                                    *unsafe {
                                            data.offset((hdr + 5) as isize).offset(1 as isize)
                                        }
                                } as i32;
                    __state = 69;
                }
                69 => {
                    if n_cell > 0 { __state = 71; } else { __state = 70; }
                }
                70 => {
                    unsafe { *data.offset((hdr + 7) as isize) = 0 as u8 };
                    __state = 94;
                }
                71 => {
                    temp =
                        unsafe {
                                sqlite3_pager_temp_space(unsafe {
                                        (*unsafe { (*p_page_1).p_bt }).p_pager
                                    })
                            } as *mut u8;
                    __state = 72;
                }
                72 => {
                    unsafe {
                        memcpy(temp as *mut (), data as *const (),
                            usable_size as u64)
                    };
                    __state = 73;
                }
                73 => { src = temp; __state = 74; }
                74 => { i = 0; __state = 75; }
                75 => {
                    if i < n_cell { __state = 76; } else { __state = 70; }
                }
                76 => { __state = 78; }
                77 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 75;
                }
                78 => {
                    p_addr_1 =
                        unsafe { data.offset((cell_offset + i * 2) as isize) };
                    __state = 79;
                }
                79 => {
                    pc =
                        (unsafe { *p_addr_1.offset(0 as isize) } as i32) << 8 |
                            unsafe { *p_addr_1.offset(1 as isize) } as i32;
                    __state = 80;
                }
                80 => { __state = 81; }
                81 => { __state = 82; }
                82 => {
                    if pc > i_cell_last { __state = 84; } else { __state = 83; }
                }
                83 => { { let _ = 0; }; __state = 85; }
                84 => { return unsafe { sqlite3_corrupt_error(1729) }; }
                85 => {
                    size =
                        unsafe {
                                (unsafe {
                                        (*p_page_1).x_cell_size.unwrap()
                                    })(p_page_1, unsafe { &mut *src.offset(pc as isize) })
                            } as i32;
                    __state = 86;
                }
                86 => { cbrk -= size; __state = 87; }
                87 => {
                    if cbrk < i_cell_start || pc + size > usable_size {
                        __state = 89;
                    } else { __state = 88; }
                }
                88 => { { let _ = 0; }; __state = 90; }
                89 => { return unsafe { sqlite3_corrupt_error(1735) }; }
                90 => { __state = 91; }
                91 => { __state = 92; }
                92 => {
                    {
                        unsafe { *p_addr_1.offset(0 as isize) = (cbrk >> 8) as u8 };
                        unsafe { *p_addr_1.offset(1 as isize) = cbrk as u8 }
                    };
                    __state = 93;
                }
                93 => {
                    unsafe {
                        memcpy(unsafe { &raw mut *data.offset(cbrk as isize) } as
                                *mut (),
                            unsafe { &raw mut *src.offset(pc as isize) } as *const (),
                            size as u64)
                    };
                    __state = 77;
                }
                94 => { __state = 2; }
                95 => {
                    if unsafe { *data.offset((hdr + 7) as isize) } as i32 + cbrk
                                - i_cell_first != unsafe { (*p_page_1).n_free } {
                        __state = 97;
                    } else { __state = 96; }
                }
                96 => { { let _ = 0; }; __state = 98; }
                97 => { return unsafe { sqlite3_corrupt_error(1749) }; }
                98 => {
                    {
                        unsafe {
                            *unsafe {
                                        data.offset((hdr + 5) as isize).offset(0 as isize)
                                    } = (cbrk >> 8) as u8
                        };
                        unsafe {
                            *unsafe {
                                        data.offset((hdr + 5) as isize).offset(1 as isize)
                                    } = cbrk as u8
                        }
                    };
                    __state = 99;
                }
                99 => {
                    unsafe { *data.offset((hdr + 1) as isize) = 0 as u8 };
                    __state = 100;
                }
                100 => {
                    unsafe { *data.offset((hdr + 2) as isize) = 0 as u8 };
                    __state = 101;
                }
                101 => {
                    unsafe {
                        memset(unsafe {
                                    &raw mut *data.offset(i_cell_first as isize)
                                } as *mut (), 0, (cbrk - i_cell_first) as u64)
                    };
                    __state = 102;
                }
                102 => { { let _ = 0; }; __state = 103; }
                103 => { return 0; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn allocate_space(p_page_1: *mut MemPage, n_byte_1: i32,
    p_idx_1: &mut i32) -> i32 {
    let hdr: i32 = unsafe { (*p_page_1).hdr_offset } as i32;
    let data: *mut u8 = unsafe { (*p_page_1).a_data };
    let mut top: i32 = 0;
    let mut rc: i32 = 0;
    let mut p_tmp: *const u8 = core::ptr::null();
    let mut gap: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    gap =
        unsafe { (*p_page_1).cell_offset } as i32 +
            2 * unsafe { (*p_page_1).n_cell } as i32;
    { let _ = 0; };
    p_tmp = unsafe { data.offset((hdr + 5) as isize) };
    top =
        (unsafe { *p_tmp.offset(0 as isize) } as i32) << 8 |
            unsafe { *p_tmp.offset(1 as isize) } as i32;
    if gap > top {
        if top == 0 &&
                unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } ==
                    65536 as u32 {
            top = 65536;
        } else { return unsafe { sqlite3_corrupt_error(1876) }; }
    } else if top >
            unsafe { (*unsafe { (*p_page_1).p_bt }).usable_size } as i32 {
        return unsafe { sqlite3_corrupt_error(1879) };
    }
    if (unsafe { *data.offset((hdr + 2) as isize) } != 0 ||
                unsafe { *data.offset((hdr + 1) as isize) } != 0) &&
            gap + 2 <= top {
        let p_space: *const u8 =
            page_find_slot(unsafe { &*p_page_1 }, n_byte_1, &mut rc) as
                *const u8;
        if !(p_space).is_null() {
            let mut g2: i32 = 0;
            { let _ = 0; };
            *p_idx_1 =
                {
                    g2 = unsafe { p_space.offset_from(data) } as i64 as i32;
                    g2
                };
            if g2 <= gap {
                return unsafe { sqlite3_corrupt_error(1896) };
            } else { return 0; }
        } else if rc != 0 { return rc; }
    }
    if gap + 2 + n_byte_1 > top {
        { let _ = 0; };
        { let _ = 0; };
        rc =
            defragment_page(p_page_1,
                if 4 < unsafe { (*p_page_1).n_free } - (2 + n_byte_1) {
                    4
                } else { (unsafe { (*p_page_1).n_free }) - (2 + n_byte_1) });
        if rc != 0 { return rc; }
        top =
            (((unsafe {
                                                *unsafe {
                                                        data.offset((hdr + 5) as isize).offset(0 as isize)
                                                    }
                                            } as i32) << 8 |
                                unsafe {
                                        *unsafe {
                                                data.offset((hdr + 5) as isize).offset(1 as isize)
                                            }
                                    } as i32) as i32 - 1 & 65535) + 1;
        { let _ = 0; };
    }
    top -= n_byte_1;
    {
        unsafe {
            *unsafe { data.offset((hdr + 5) as isize).offset(0 as isize) } =
                (top >> 8) as u8
        };
        unsafe {
            *unsafe { data.offset((hdr + 5) as isize).offset(1 as isize) } =
                top as u8
        }
    };
    { let _ = 0; };
    *p_idx_1 = top;
    return 0;
}
extern "C" fn insert_cell(p_page_1: *mut MemPage, i: i32,
    mut p_cell_1: *mut u8, sz: i32, p_temp_1: *mut u8, i_child_1: Pgno)
    -> i32 {
    let mut idx: i32 = 0;
    let mut j: i32 = 0;
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut p_ins: *mut u8 = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_page_1).n_overflow } != 0 ||
            sz + 2 > unsafe { (*p_page_1).n_free } {
        if !(p_temp_1).is_null() {
            unsafe {
                memcpy(p_temp_1 as *mut (), p_cell_1 as *const (), sz as u64)
            };
            p_cell_1 = p_temp_1;
        }
        unsafe { sqlite3_put4byte(p_cell_1, i_child_1) };
        j =
            {
                    let __p = unsafe { &mut (*p_page_1).n_overflow };
                    let __t = *__p;
                    *__p += 1;
                    __t
                } as i32;
        { let _ = 0; };
        unsafe { (*p_page_1).ap_ovfl[j as usize] = p_cell_1 };
        unsafe { (*p_page_1).ai_ovfl[j as usize] = i as u16 };
        { let _ = 0; };
        { let _ = 0; };
    } else {
        let mut rc: i32 =
            unsafe { sqlite3_pager_write(unsafe { (*p_page_1).p_db_page }) };
        if rc != 0 { return rc; }
        { let _ = 0; };
        data = unsafe { (*p_page_1).a_data };
        { let _ = 0; };
        rc = allocate_space(p_page_1, sz, &mut idx);
        if rc != 0 { return rc; }
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_page_1).n_free -= (2 + sz) as u16 as i32 };
        unsafe {
            memcpy(unsafe { &raw mut *data.offset((idx + 4) as isize) } as
                    *mut (),
                unsafe { p_cell_1.offset(4 as isize) } as *const (),
                (sz - 4) as u64)
        };
        unsafe {
            sqlite3_put4byte(unsafe { &mut *data.offset(idx as isize) },
                i_child_1)
        };
        p_ins =
            unsafe {
                unsafe { (*p_page_1).a_cell_idx.offset((i * 2) as isize) }
            };
        unsafe {
            memmove(unsafe { p_ins.offset(2 as isize) } as *mut (),
                p_ins as *const (),
                (2 * (unsafe { (*p_page_1).n_cell } as i32 - i)) as u64)
        };
        {
            unsafe { *p_ins.offset(0 as isize) = (idx >> 8) as u8 };
            unsafe { *p_ins.offset(1 as isize) = idx as u8 }
        };
        {
            let __p = unsafe { &mut (*p_page_1).n_cell };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if {
                        let __p =
                            unsafe {
                                &mut *data.offset((unsafe { (*p_page_1).hdr_offset } as i32
                                                    + 4) as isize)
                            };
                        *__p += 1;
                        *__p
                    } as i32 == 0 {
            {
                let __p =
                    unsafe {
                        &mut *data.offset((unsafe { (*p_page_1).hdr_offset } as i32
                                            + 3) as isize)
                    };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        { let _ = 0; };
        if unsafe { (*unsafe { (*p_page_1).p_bt }).auto_vacuum } != 0 {
            let mut rc2: i32 = 0;
            ptrmap_put_ovfl_ptr(p_page_1, unsafe { &*p_page_1 }, p_cell_1,
                &mut rc2);
            if rc2 != 0 { return rc2; }
        }
    }
    return 0;
}
extern "C" fn another_valid_cursor(p_cur_1: *mut BtCursor) -> i32 {
    let mut p_other: *mut BtCursor = core::ptr::null_mut();
    {
        p_other = unsafe { (*unsafe { (*p_cur_1).p_bt }).p_cursor };
        '__b59: loop {
            if !(!(p_other).is_null()) { break '__b59; }
            '__c59: loop {
                if p_other != p_cur_1 &&
                            unsafe { (*p_other).e_state } as i32 == 0 &&
                        unsafe { (*p_other).p_page } == unsafe { (*p_cur_1).p_page }
                    {
                    return unsafe { sqlite3_corrupt_error(9146) };
                }
                break '__c59;
            }
            p_other = unsafe { (*p_other).p_next };
        }
    }
    return 0;
}
extern "C" fn copy_node_content(p_from_1: &MemPage, p_to_1: *mut MemPage,
    p_rc_1: &mut i32) -> () {
    if *p_rc_1 == 0 {
        let p_bt: *const BtShared = (*p_from_1).p_bt as *const BtShared;
        let a_from: *mut u8 = (*p_from_1).a_data;
        let a_to: *mut u8 = unsafe { (*p_to_1).a_data };
        let i_from_hdr: i32 = (*p_from_1).hdr_offset as i32;
        let i_to_hdr: i32 =
            if unsafe { (*p_to_1).pgno } == 1 as u32 { 100 } else { 0 } as
                i32;
        let mut rc: i32 = 0;
        let mut i_data: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        i_data =
            (unsafe {
                                *unsafe {
                                        a_from.offset((i_from_hdr + 5) as isize).offset(0 as isize)
                                    }
                            } as i32) << 8 |
                unsafe {
                        *unsafe {
                                a_from.offset((i_from_hdr + 5) as isize).offset(1 as isize)
                            }
                    } as i32;
        unsafe {
            memcpy(unsafe { &raw mut *a_to.offset(i_data as isize) } as
                    *mut (),
                unsafe { &raw mut *a_from.offset(i_data as isize) } as
                    *const (),
                (unsafe { (*p_bt).usable_size } - i_data as u32) as u64)
        };
        unsafe {
            memcpy(unsafe { &raw mut *a_to.offset(i_to_hdr as isize) } as
                    *mut (),
                unsafe { &raw mut *a_from.offset(i_from_hdr as isize) } as
                    *const (),
                ((*p_from_1).cell_offset as i32 +
                        2 * (*p_from_1).n_cell as i32) as u64)
        };
        unsafe { (*p_to_1).is_init = 0 as u8 };
        rc = btree_init_page(p_to_1);
        if rc == 0 { rc = btree_compute_free_space(unsafe { &mut *p_to_1 }); }
        if rc != 0 { *p_rc_1 = rc; return; }
        if unsafe { (*p_bt).auto_vacuum } != 0 {
            *p_rc_1 = set_child_ptrmaps(p_to_1);
        }
    }
}
extern "C" fn balance_deeper(p_root_1: *mut MemPage,
    pp_child_1: &mut *mut MemPage) -> i32 {
    let mut rc: i32 = 0;
    let mut p_child: *mut MemPage = core::ptr::null_mut();
    let mut pgno_child: Pgno = 0 as Pgno;
    let p_bt: *mut BtShared = unsafe { (*p_root_1).p_bt };
    { let _ = 0; };
    { let _ = 0; };
    rc = unsafe { sqlite3_pager_write(unsafe { (*p_root_1).p_db_page }) };
    if rc == 0 {
        rc =
            allocate_btree_page(p_bt, &mut p_child, &mut pgno_child,
                unsafe { (*p_root_1).pgno }, 0 as u8);
        copy_node_content(unsafe { &*p_root_1 }, p_child, &mut rc);
        if unsafe { (*p_bt).auto_vacuum } != 0 {
            ptrmap_put(p_bt, pgno_child, 5 as u8, unsafe { (*p_root_1).pgno },
                &mut rc);
        }
    }
    if rc != 0 {
        *pp_child_1 = core::ptr::null_mut();
        release_page(p_child);
        return rc;
    }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe {
        memcpy(unsafe { &raw mut (*p_child).ai_ovfl[0 as usize] } as *mut u16
                as *mut (),
            unsafe { &raw mut (*p_root_1).ai_ovfl[0 as usize] } as *mut u16 as
                *const (),
            unsafe { (*p_root_1).n_overflow } as u64 *
                core::mem::size_of::<u16>() as u64)
    };
    unsafe {
        memcpy(unsafe { &raw mut (*p_child).ap_ovfl[0 as usize] } as
                    *mut *mut u8 as *mut (),
            unsafe { &raw mut (*p_root_1).ap_ovfl[0 as usize] } as
                    *mut *mut u8 as *const (),
            unsafe { (*p_root_1).n_overflow } as u64 *
                core::mem::size_of::<*mut u8>() as u64)
    };
    unsafe { (*p_child).n_overflow = unsafe { (*p_root_1).n_overflow } };
    zero_page(p_root_1,
        unsafe { *unsafe { (*p_child).a_data.offset(0 as isize) } } as i32 &
            !8);
    unsafe {
        sqlite3_put4byte(unsafe {
                &mut *unsafe {
                            (*p_root_1).a_data.offset((unsafe { (*p_root_1).hdr_offset }
                                            as i32 + 8) as isize)
                        }
            }, pgno_child)
    };
    *pp_child_1 = p_child;
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CellArray {
    n_cell: i32,
    p_ref: *mut MemPage,
    ap_cell: *mut *mut u8,
    sz_cell: *mut u16,
    ap_end: [*mut u8; 6],
    ix_nx: [i32; 6],
}
extern "C" fn rebuild_page(p_c_array_1: &CellArray, i_first_1: i32,
    n_cell_1: i32, p_pg_1: &mut MemPage) -> i32 {
    let hdr: i32 = (*p_pg_1).hdr_offset as i32;
    let a_data: *mut u8 = (*p_pg_1).a_data;
    let usable_size: i32 = unsafe { (*(*p_pg_1).p_bt).usable_size } as i32;
    let p_end: *mut u8 = unsafe { &mut *a_data.offset(usable_size as isize) };
    let mut i: i32 = i_first_1;
    let mut j: u32 = 0 as u32;
    let i_end: i32 = i + n_cell_1;
    let mut p_cellptr: *mut u8 = (*p_pg_1).a_cell_idx;
    let p_tmp: *mut u8 =
        unsafe {
                sqlite3_pager_temp_space(unsafe { (*(*p_pg_1).p_bt).p_pager })
            } as *mut u8;
    let mut p_data: *mut u8 = core::ptr::null_mut();
    let mut k: i32 = 0;
    let mut p_src_end: *const u8 = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    j =
        ((unsafe {
                                *unsafe {
                                        a_data.offset((hdr + 5) as isize).offset(0 as isize)
                                    }
                            } as i32) << 8 |
                unsafe {
                        *unsafe {
                                a_data.offset((hdr + 5) as isize).offset(1 as isize)
                            }
                    } as i32) as u32;
    if j > usable_size as u32 { j = 0 as u32; }
    unsafe {
        memcpy(unsafe { &raw mut *p_tmp.add(j as usize) } as *mut (),
            unsafe { &raw mut *a_data.add(j as usize) } as *const (),
            (usable_size as u32 - j) as u64)
    };
    { let _ = 0; };
    {
        k = 0;
        '__b60: loop {
            if !((*p_c_array_1).ix_nx[k as usize] <= i) { break '__b60; }
            '__c60: loop { break '__c60; }
            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
        }
    }
    p_src_end = (*p_c_array_1).ap_end[k as usize];
    p_data = p_end;
    loop {
        let mut p_cell: *const u8 =
            unsafe { *(*p_c_array_1).ap_cell.offset(i as isize) } as
                *const u8;
        let sz: u16 = unsafe { *(*p_c_array_1).sz_cell.offset(i as isize) };
        { let _ = 0; };
        if p_cell as uptr >= unsafe { a_data.add(j as usize) } as uptr &&
                (p_cell as uptr) < p_end as uptr {
            if unsafe { p_cell.add(sz as usize) } as uptr > p_end as uptr {
                return unsafe { sqlite3_corrupt_error(7711) };
            }
            p_cell =
                unsafe {
                    p_tmp.offset(unsafe { p_cell.offset_from(a_data) } as i64 as
                            isize)
                };
        } else if unsafe { p_cell.add(sz as usize) } as uptr >
                    p_src_end as uptr && (p_cell as uptr) < p_src_end as uptr {
            return unsafe { sqlite3_corrupt_error(7716) };
        }
        {
            let __n = sz;
            let __p = &mut p_data;
            *__p = unsafe { (*__p).sub(__n as usize) };
        };
        {
            unsafe {
                *p_cellptr.offset(0 as isize) =
                    (unsafe { p_data.offset_from(a_data) } as i64 >> 8) as u8
            };
            unsafe {
                *p_cellptr.offset(1 as isize) =
                    unsafe { p_data.offset_from(a_data) } as i64 as u8
            }
        };
        {
            let __n = 2;
            let __p = &mut p_cellptr;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        if p_data < p_cellptr {
            return unsafe { sqlite3_corrupt_error(7722) };
        }
        unsafe { memmove(p_data as *mut (), p_cell as *const (), sz as u64) };
        { let _ = 0; };
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        if i >= i_end { break; }
        if (*p_c_array_1).ix_nx[k as usize] <= i {
            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
            p_src_end = (*p_c_array_1).ap_end[k as usize];
        }
    }
    { let _ = 0; };
    (*p_pg_1).n_cell = n_cell_1 as u16;
    (*p_pg_1).n_overflow = 0 as u8;
    {
        unsafe {
            *unsafe { a_data.offset((hdr + 1) as isize).offset(0 as isize) } =
                (0 >> 8) as u8
        };
        unsafe {
            *unsafe { a_data.offset((hdr + 1) as isize).offset(1 as isize) } =
                0 as u8
        }
    };
    {
        unsafe {
            *unsafe { a_data.offset((hdr + 3) as isize).offset(0 as isize) } =
                ((*p_pg_1).n_cell as i32 >> 8) as u8
        };
        unsafe {
            *unsafe { a_data.offset((hdr + 3) as isize).offset(1 as isize) } =
                (*p_pg_1).n_cell as u8
        }
    };
    {
        unsafe {
            *unsafe { a_data.offset((hdr + 5) as isize).offset(0 as isize) } =
                (unsafe { p_data.offset_from(a_data) } as i64 >> 8) as u8
        };
        unsafe {
            *unsafe { a_data.offset((hdr + 5) as isize).offset(1 as isize) } =
                unsafe { p_data.offset_from(a_data) } as i64 as u8
        }
    };
    unsafe { *a_data.offset((hdr + 7) as isize) = 0 as u8 };
    return 0;
}
extern "C" fn balance_quick(p_parent_1: *mut MemPage, p_page_1: *mut MemPage,
    p_space_1: *mut u8) -> i32 {
    let p_bt: *mut BtShared = unsafe { (*p_page_1).p_bt };
    let mut p_new: *mut MemPage = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut pgno_new: Pgno = 0 as Pgno;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_page_1).n_cell } as i32 == 0 {
        return unsafe { sqlite3_corrupt_error(8049) };
    }
    { let _ = 0; };
    { let _ = 0; };
    rc =
        allocate_btree_page(p_bt, &mut p_new, &mut pgno_new, 0 as Pgno,
            0 as u8);
    if rc == 0 {
        let mut p_out: *mut u8 =
            unsafe { &mut *p_space_1.offset(4 as isize) };
        let mut p_cell: *mut u8 = unsafe { (*p_page_1).ap_ovfl[0 as usize] };
        let mut sz_cell: u16 =
            unsafe {
                (unsafe {
                        (*p_page_1).x_cell_size.unwrap()
                    })(p_page_1, p_cell)
            };
        let mut p_stop: *mut u8 = core::ptr::null_mut();
        let mut b: CellArray = unsafe { core::mem::zeroed() };
        { let _ = 0; };
        { let _ = 0; };
        zero_page(p_new, 1 | 4 | 8);
        b.n_cell = 1;
        b.p_ref = p_page_1;
        b.ap_cell = &mut p_cell;
        b.sz_cell = &mut sz_cell;
        b.ap_end[0 as usize] = unsafe { (*p_page_1).a_data_end };
        b.ix_nx[0 as usize] = 2;
        b.ix_nx[(3 * 2 - 1) as usize] = 2147483647;
        rc = rebuild_page(&b, 0, 1, unsafe { &mut *p_new });
        if rc != 0 { release_page(p_new); return rc; }
        unsafe {
            (*p_new).n_free =
                (unsafe { (*p_bt).usable_size } -
                                unsafe { (*p_new).cell_offset } as u32 - 2 as u32 -
                        sz_cell as u32) as i32
        };
        if unsafe { (*p_bt).auto_vacuum } != 0 {
            ptrmap_put(p_bt, pgno_new, 5 as u8, unsafe { (*p_parent_1).pgno },
                &mut rc);
            if sz_cell as i32 > unsafe { (*p_new).min_local } as i32 {
                ptrmap_put_ovfl_ptr(p_new, unsafe { &*p_new }, p_cell,
                    &mut rc);
            }
        }
        p_cell =
            unsafe {
                unsafe {
                    (*p_page_1).a_data.offset((unsafe { (*p_page_1).mask_page }
                                    as i32 &
                                ((unsafe {
                                                    *unsafe {
                                                            unsafe {
                                                                (*p_page_1).a_cell_idx.offset((2 *
                                                                                (unsafe { (*p_page_1).n_cell } as i32 - 1)) as
                                                                            isize).offset(0 as isize)
                                                            }
                                                        }
                                                } as i32) << 8 |
                                    unsafe {
                                            *unsafe {
                                                    unsafe {
                                                        (*p_page_1).a_cell_idx.offset((2 *
                                                                        (unsafe { (*p_page_1).n_cell } as i32 - 1)) as
                                                                    isize).offset(1 as isize)
                                                    }
                                                }
                                        } as i32)) as isize)
                }
            };
        p_stop = unsafe { p_cell.offset(9 as isize) };
        while unsafe {
                                *{
                                        let __p = &mut p_cell;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } as i32 & 128 != 0 && p_cell < p_stop {}
        p_stop = unsafe { p_cell.offset(9 as isize) };
        while {
                                let __v =
                                    unsafe {
                                        *{
                                                let __p = &mut p_cell;
                                                let __t = *__p;
                                                *__p = unsafe { (*__p).offset(1) };
                                                __t
                                            }
                                    };
                                unsafe {
                                    *{
                                                let __p = &mut p_out;
                                                let __t = *__p;
                                                *__p = unsafe { (*__p).offset(1) };
                                                __t
                                            } = __v
                                };
                                __v
                            } as i32 & 128 != 0 && p_cell < p_stop {}
        if rc == 0 {
            rc =
                insert_cell(p_parent_1,
                    unsafe { (*p_parent_1).n_cell } as i32, p_space_1,
                    unsafe { p_out.offset_from(p_space_1) } as i64 as i32,
                    core::ptr::null_mut(), unsafe { (*p_page_1).pgno });
        }
        unsafe {
            sqlite3_put4byte(unsafe {
                    &mut *unsafe {
                                (*p_parent_1).a_data.offset((unsafe {
                                                    (*p_parent_1).hdr_offset
                                                } as i32 + 8) as isize)
                            }
                }, pgno_new)
        };
        release_page(p_new);
    }
    return rc;
}
extern "C" fn compute_cell_size(p: &CellArray, n_1: i32) -> u16 {
    { let _ = 0; };
    { let _ = 0; };
    unsafe {
        *(*p).sz_cell.offset(n_1 as isize) =
            unsafe {
                (unsafe {
                        (*(*p).p_ref).x_cell_size.unwrap()
                    })((*p).p_ref,
                    unsafe { *(*p).ap_cell.offset(n_1 as isize) })
            }
    };
    return unsafe { *(*p).sz_cell.offset(n_1 as isize) };
}
extern "C" fn cached_cell_size(p: *mut CellArray, n_1: i32) -> u16 {
    { let _ = 0; };
    if unsafe { *unsafe { (*p).sz_cell.offset(n_1 as isize) } } != 0 {
        return unsafe { *unsafe { (*p).sz_cell.offset(n_1 as isize) } };
    }
    return compute_cell_size(unsafe { &*p }, n_1);
}
extern "C" fn page_free_array(p_pg_1: *mut MemPage, i_first_1: i32,
    n_cell_1: i32, p_c_array_1: &CellArray) -> i32 {
    let a_data: *mut u8 = unsafe { (*p_pg_1).a_data };
    let p_end: *mut u8 =
        unsafe {
            &mut *a_data.add(unsafe {
                                (*unsafe { (*p_pg_1).p_bt }).usable_size
                            } as usize)
        };
    let p_start: *const u8 =
        unsafe {
                &raw mut *a_data.offset((unsafe { (*p_pg_1).hdr_offset } as
                                            i32 + 8 + unsafe { (*p_pg_1).child_ptr_size } as i32) as
                                isize)
            } as *const u8;
    let mut n_ret: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let i_end: i32 = i_first_1 + n_cell_1;
    let mut n_free: i32 = 0;
    let mut a_ofst: [i32; 10] = [0; 10];
    let mut a_after: [i32; 10] = [0; 10];
    {
        i = i_first_1;
        '__b64: loop {
            if !(i < i_end) { break '__b64; }
            '__c64: loop {
                let p_cell: *const u8 =
                    unsafe { *(*p_c_array_1).ap_cell.offset(i as isize) } as
                        *const u8;
                if p_cell as uptr >= p_start as uptr &&
                        (p_cell as uptr) < p_end as uptr {
                    let mut sz: i32 = 0;
                    let mut i_after: i32 = 0;
                    let mut i_ofst: i32 = 0;
                    sz =
                        unsafe { *(*p_c_array_1).sz_cell.offset(i as isize) } as
                            i32;
                    { let _ = 0; };
                    i_ofst =
                        unsafe { p_cell.offset_from(a_data) } as i64 as u16 as i32;
                    i_after = i_ofst + sz;
                    {
                        j = 0;
                        '__b65: loop {
                            if !(j < n_free) { break '__b65; }
                            '__c65: loop {
                                if a_ofst[j as usize] == i_after {
                                    a_ofst[j as usize] = i_ofst;
                                    break '__b65;
                                } else if a_after[j as usize] == i_ofst {
                                    a_after[j as usize] = i_after;
                                    break '__b65;
                                }
                                break '__c65;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if j >= n_free {
                        if n_free >=
                                (core::mem::size_of::<[i32; 10]>() as u64 /
                                        core::mem::size_of::<i32>() as u64) as i32 {
                            {
                                j = 0;
                                '__b66: loop {
                                    if !(j < n_free) { break '__b66; }
                                    '__c66: loop {
                                        free_space(unsafe { &mut *p_pg_1 }, a_ofst[j as usize],
                                            a_after[j as usize] - a_ofst[j as usize]);
                                        break '__c66;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            n_free = 0;
                        }
                        a_ofst[n_free as usize] = i_ofst;
                        a_after[n_free as usize] = i_after;
                        if unsafe { a_data.offset(i_after as isize) } > p_end {
                            return 0;
                        }
                        { let __p = &mut n_free; let __t = *__p; *__p += 1; __t };
                    }
                    { let __p = &mut n_ret; let __t = *__p; *__p += 1; __t };
                }
                break '__c64;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        j = 0;
        '__b67: loop {
            if !(j < n_free) { break '__b67; }
            '__c67: loop {
                free_space(unsafe { &mut *p_pg_1 }, a_ofst[j as usize],
                    a_after[j as usize] - a_ofst[j as usize]);
                break '__c67;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    return n_ret;
}
extern "C" fn page_insert_array(p_pg_1: *mut MemPage, p_begin_1: *const u8,
    pp_data_1: &mut *mut u8, mut p_cellptr_1: *mut u8, i_first_1: i32,
    n_cell_1: i32, p_c_array_1: &CellArray) -> i32 {
    let mut i: i32 = i_first_1;
    let a_data: *const u8 = unsafe { (*p_pg_1).a_data } as *const u8;
    let mut p_data: *mut u8 = *pp_data_1;
    let i_end: i32 = i_first_1 + n_cell_1;
    let mut k: i32 = 0;
    let mut p_end: *const u8 = core::ptr::null();
    { let _ = 0; };
    if i_end <= i_first_1 { return 0; }
    { let _ = 0; };
    {
        k = 0;
        '__b68: loop {
            if !((*p_c_array_1).ix_nx[k as usize] <= i) { break '__b68; }
            '__c68: loop { break '__c68; }
            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
        }
    }
    p_end = (*p_c_array_1).ap_end[k as usize];
    loop {
        let mut sz: i32 = 0;
        let mut rc: i32 = 0;
        let mut p_slot: *mut u8 = core::ptr::null_mut();
        { let _ = 0; };
        sz = unsafe { *(*p_c_array_1).sz_cell.offset(i as isize) } as i32;
        if unsafe { *a_data.offset(1 as isize) } as i32 == 0 &&
                    unsafe { *a_data.offset(2 as isize) } as i32 == 0 ||
                {
                        p_slot = page_find_slot(unsafe { &*p_pg_1 }, sz, &mut rc);
                        p_slot
                    } == core::ptr::null_mut() {
            if (unsafe { p_data.offset_from(p_begin_1) } as i64) < sz as i64 {
                return 1;
            }
            {
                let __n = sz;
                let __p = &mut p_data;
                *__p = unsafe { (*__p).offset(-(__n as isize)) };
            };
            p_slot = p_data;
        }
        { let _ = 0; };
        if unsafe {
                            unsafe {
                                (*(*p_c_array_1).ap_cell.offset(i as
                                                isize)).offset(sz as isize)
                            }
                        } as uptr > p_end as uptr &&
                (unsafe { *(*p_c_array_1).ap_cell.offset(i as isize) } as
                            uptr) < p_end as uptr {
            { let _ = 0; };
            { let _ = unsafe { sqlite3_corrupt_error(7809) }; };
            return 1;
        }
        unsafe {
            memmove(p_slot as *mut (),
                unsafe { *(*p_c_array_1).ap_cell.offset(i as isize) } as
                    *const (), sz as u64)
        };
        {
            unsafe {
                *p_cellptr_1.offset(0 as isize) =
                    (unsafe { p_slot.offset_from(a_data) } as i64 >> 8) as u8
            };
            unsafe {
                *p_cellptr_1.offset(1 as isize) =
                    unsafe { p_slot.offset_from(a_data) } as i64 as u8
            }
        };
        {
            let __n = 2;
            let __p = &mut p_cellptr_1;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        if i >= i_end { break; }
        if (*p_c_array_1).ix_nx[k as usize] <= i {
            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
            p_end = (*p_c_array_1).ap_end[k as usize];
        }
    }
    *pp_data_1 = p_data;
    return 0;
}
extern "C" fn populate_cell_cache(p: &CellArray, mut idx: i32, mut n_1: i32)
    -> () {
    let p_ref: *mut MemPage = (*p).p_ref;
    let sz_cell: *mut u16 = (*p).sz_cell;
    { let _ = 0; };
    while n_1 > 0 {
        { let _ = 0; };
        if unsafe { *sz_cell.offset(idx as isize) } as i32 == 0 {
            unsafe {
                *sz_cell.offset(idx as isize) =
                    unsafe {
                        (unsafe {
                                (*p_ref).x_cell_size.unwrap()
                            })(p_ref, unsafe { *(*p).ap_cell.offset(idx as isize) })
                    }
            };
        } else { { let _ = 0; }; }
        { let __p = &mut idx; let __t = *__p; *__p += 1; __t };
        { let __p = &mut n_1; let __t = *__p; *__p -= 1; __t };
    }
}
extern "C" fn edit_page(p_pg_1: *mut MemPage, i_old_1: i32, i_new_1: i32,
    n_new_1: i32, p_c_array_1: *mut CellArray) -> i32 {
    let mut a_data: *mut u8 = core::ptr::null_mut();
    let mut hdr: i32 = 0;
    let mut p_begin: *mut u8 = core::ptr::null_mut();
    let mut n_cell: i32 = 0;
    let mut p_data: *mut u8 = core::ptr::null_mut();
    let mut p_cellptr: *mut u8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut i_old_end: i32 = 0;
    let mut i_new_end: i32 = 0;
    let mut n_shift: i32 = 0;
    let mut n_tail: i32 = 0;
    let mut n_add: i32 = 0;
    let mut i_cell: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s72:
            {
            match __state {
                0 => { a_data = unsafe { (*p_pg_1).a_data }; __state = 3; }
                2 => {
                    if n_new_1 < 1 { __state = 60; } else { __state = 59; }
                }
                3 => {
                    hdr = unsafe { (*p_pg_1).hdr_offset } as i32;
                    __state = 4;
                }
                4 => {
                    p_begin =
                        unsafe {
                            unsafe {
                                (*p_pg_1).a_cell_idx.offset((n_new_1 * 2) as isize)
                            }
                        };
                    __state = 5;
                }
                5 => {
                    n_cell = unsafe { (*p_pg_1).n_cell } as i32;
                    __state = 6;
                }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { __state = 9; }
                9 => {
                    i_old_end =
                        i_old_1 + unsafe { (*p_pg_1).n_cell } as i32 +
                            unsafe { (*p_pg_1).n_overflow } as i32;
                    __state = 10;
                }
                10 => { i_new_end = i_new_1 + n_new_1; __state = 11; }
                11 => { { let _ = 0; }; __state = 12; }
                12 => {
                    if i_old_1 < i_new_1 {
                        __state = 14;
                    } else { __state = 13; }
                }
                13 => {
                    if i_new_end < i_old_end {
                        __state = 20;
                    } else { __state = 19; }
                }
                14 => {
                    n_shift =
                        page_free_array(p_pg_1, i_old_1, i_new_1 - i_old_1,
                            unsafe { &*p_c_array_1 });
                    __state = 15;
                }
                15 => {
                    if n_shift > n_cell { __state = 17; } else { __state = 16; }
                }
                16 => {
                    unsafe {
                        memmove(unsafe { (*p_pg_1).a_cell_idx } as *mut (),
                            unsafe {
                                    &raw mut *unsafe {
                                                (*p_pg_1).a_cell_idx.offset((n_shift * 2) as isize)
                                            }
                                } as *const (), (n_cell * 2) as u64)
                    };
                    __state = 18;
                }
                17 => { return unsafe { sqlite3_corrupt_error(7931) }; }
                18 => { n_cell -= n_shift; __state = 13; }
                19 => {
                    p_data =
                        unsafe {
                            a_data.offset(((unsafe {
                                                        *unsafe {
                                                                a_data.offset((hdr + 5) as isize).offset(0 as isize)
                                                            }
                                                    } as i32) << 8 |
                                        unsafe {
                                                *unsafe {
                                                        a_data.offset((hdr + 5) as isize).offset(1 as isize)
                                                    }
                                            } as i32) as isize)
                        };
                    __state = 23;
                }
                20 => {
                    n_tail =
                        page_free_array(p_pg_1, i_new_end, i_old_end - i_new_end,
                            unsafe { &*p_c_array_1 });
                    __state = 21;
                }
                21 => { { let _ = 0; }; __state = 22; }
                22 => { n_cell -= n_tail; __state = 19; }
                23 => {
                    if p_data < p_begin { __state = 25; } else { __state = 24; }
                }
                24 => {
                    if p_data > unsafe { (*p_pg_1).a_data_end } {
                        __state = 27;
                    } else { __state = 26; }
                }
                25 => { __state = 2; }
                26 => {
                    if i_new_1 < i_old_1 {
                        __state = 29;
                    } else { __state = 28; }
                }
                27 => { __state = 2; }
                28 => { i = 0; __state = 38; }
                29 => {
                    n_add =
                        if n_new_1 < i_old_1 - i_new_1 {
                            n_new_1
                        } else { i_old_1 - i_new_1 };
                    __state = 30;
                }
                30 => { { let _ = 0; }; __state = 31; }
                31 => { { let _ = 0; }; __state = 32; }
                32 => {
                    p_cellptr = unsafe { (*p_pg_1).a_cell_idx };
                    __state = 33;
                }
                33 => {
                    unsafe {
                        memmove(unsafe {
                                    &raw mut *p_cellptr.offset((n_add * 2) as isize)
                                } as *mut (), p_cellptr as *const (), (n_cell * 2) as u64)
                    };
                    __state = 34;
                }
                34 => {
                    if page_insert_array(p_pg_1, p_begin as *const u8,
                                &mut p_data, p_cellptr, i_new_1, n_add,
                                unsafe { &*p_c_array_1 }) != 0 {
                        __state = 36;
                    } else { __state = 35; }
                }
                35 => { n_cell += n_add; __state = 28; }
                36 => { __state = 2; }
                37 => { { let _ = 0; }; __state = 49; }
                38 => {
                    if i < unsafe { (*p_pg_1).n_overflow } as i32 {
                        __state = 39;
                    } else { __state = 37; }
                }
                39 => {
                    i_cell =
                        i_old_1 + unsafe { (*p_pg_1).ai_ovfl[i as usize] } as i32 -
                            i_new_1;
                    __state = 41;
                }
                40 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 38;
                }
                41 => {
                    if i_cell >= 0 && i_cell < n_new_1 {
                        __state = 42;
                    } else { __state = 40; }
                }
                42 => {
                    p_cellptr =
                        unsafe {
                            unsafe {
                                (*p_pg_1).a_cell_idx.offset((i_cell * 2) as isize)
                            }
                        };
                    __state = 43;
                }
                43 => {
                    if n_cell > i_cell { __state = 45; } else { __state = 44; }
                }
                44 => {
                    { let __p = &mut n_cell; let __t = *__p; *__p += 1; __t };
                    __state = 46;
                }
                45 => {
                    unsafe {
                        memmove(unsafe { &raw mut *p_cellptr.offset(2 as isize) } as
                                *mut (), p_cellptr as *const (),
                            ((n_cell - i_cell) * 2) as u64)
                    };
                    __state = 44;
                }
                46 => {
                    cached_cell_size(p_c_array_1, i_cell + i_new_1);
                    __state = 47;
                }
                47 => {
                    if page_insert_array(p_pg_1, p_begin as *const u8,
                                &mut p_data, p_cellptr, i_cell + i_new_1, 1,
                                unsafe { &*p_c_array_1 }) != 0 {
                        __state = 48;
                    } else { __state = 40; }
                }
                48 => { __state = 2; }
                49 => {
                    p_cellptr =
                        unsafe {
                            unsafe {
                                (*p_pg_1).a_cell_idx.offset((n_cell * 2) as isize)
                            }
                        };
                    __state = 50;
                }
                50 => {
                    if page_insert_array(p_pg_1, p_begin as *const u8,
                                &mut p_data, p_cellptr, i_new_1 + n_cell, n_new_1 - n_cell,
                                unsafe { &*p_c_array_1 }) != 0 {
                        __state = 52;
                    } else { __state = 51; }
                }
                51 => { { let _ = 0; }; __state = 53; }
                52 => { __state = 2; }
                53 => {
                    unsafe { (*p_pg_1).n_cell = n_new_1 as u16 };
                    __state = 54;
                }
                54 => {
                    unsafe { (*p_pg_1).n_overflow = 0 as u8 };
                    __state = 55;
                }
                55 => {
                    {
                        unsafe {
                            *unsafe {
                                        a_data.offset((hdr + 3) as isize).offset(0 as isize)
                                    } = (unsafe { (*p_pg_1).n_cell } as i32 >> 8) as u8
                        };
                        unsafe {
                            *unsafe {
                                        a_data.offset((hdr + 3) as isize).offset(1 as isize)
                                    } = unsafe { (*p_pg_1).n_cell } as u8
                        }
                    };
                    __state = 56;
                }
                56 => {
                    {
                        unsafe {
                            *unsafe {
                                        a_data.offset((hdr + 5) as isize).offset(0 as isize)
                                    } =
                                (unsafe { p_data.offset_from(a_data) } as i64 >> 8) as u8
                        };
                        unsafe {
                            *unsafe {
                                        a_data.offset((hdr + 5) as isize).offset(1 as isize)
                                    } = unsafe { p_data.offset_from(a_data) } as i64 as u8
                        }
                    };
                    __state = 57;
                }
                57 => { return 0; }
                58 => { __state = 2; }
                59 => {
                    populate_cell_cache(unsafe { &*p_c_array_1 }, i_new_1,
                        n_new_1);
                    __state = 61;
                }
                60 => { return unsafe { sqlite3_corrupt_error(8009) }; }
                61 => {
                    return rebuild_page(unsafe { &*p_c_array_1 }, i_new_1,
                            n_new_1, unsafe { &mut *p_pg_1 });
                }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn balance_nonroot(p_parent_1: *mut MemPage, i_parent_idx_1: i32,
    a_ovfl_space_1: *mut u8, is_root_1: i32, b_bulk_1: i32) -> i32 {
    unsafe {
        let mut p_bt: *mut BtShared = core::ptr::null_mut();
        let mut n_max_cells: i32 = 0;
        let mut n_new: i32 = 0;
        let mut n_old: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut nx_div: i32 = 0;
        let mut rc: i32 = 0;
        let mut leaf_correction: u16 = 0 as u16;
        let mut leaf_data: i32 = 0;
        let mut usable_space: i32 = 0;
        let mut page_flags: i32 = 0;
        let mut i_space1: i32 = 0;
        let mut i_ovfl_space: i32 = 0;
        let mut sz_scratch: u64 = 0 as u64;
        let mut ap_old: [*mut MemPage; 3] = [core::ptr::null_mut(); 3];
        let mut ap_new: [*mut MemPage; 5] = [core::ptr::null_mut(); 5];
        let mut p_right: *mut u8 = core::ptr::null_mut();
        let mut ap_div: [*mut u8; 2] = [core::ptr::null_mut(); 2];
        let mut cnt_new: [i32; 5] = [0; 5];
        let mut cnt_old: [i32; 5] = [0; 5];
        let mut sz_new: [i32; 5] = [0; 5];
        let mut a_space1: *mut u8 = core::ptr::null_mut();
        let mut pgno: Pgno = 0 as Pgno;
        let mut ab_done: [u8; 5] = [0; 5];
        let mut a_pgno: [u32; 5] = [0; 5];
        let mut b: CellArray = unsafe { core::mem::zeroed() };
        let mut i_off: i32 = 0;
        let mut p_old: *const MemPage = core::ptr::null();
        let mut limit: i32 = 0;
        let mut a_data: *mut u8 = core::ptr::null_mut();
        let mut mask_page: u16 = 0 as u16;
        let mut pi_cell: *mut u8 = core::ptr::null_mut();
        let mut pi_end: *mut u8 = core::ptr::null_mut();
        let mut sz: u16 = 0 as u16;
        let mut p_temp: *mut u8 = core::ptr::null_mut();
        let mut p: *mut MemPage = core::ptr::null_mut();
        let mut sz__1: i32 = 0;
        let mut sz_right: i32 = 0;
        let mut sz_left: i32 = 0;
        let mut r: i32 = 0;
        let mut d: i32 = 0;
        let mut sz_r: i32 = 0;
        let mut sz_d: i32 = 0;
        let mut p_new: *mut MemPage = core::ptr::null_mut();
        let mut i_b: i32 = 0;
        let mut pgno_a: Pgno = 0 as Pgno;
        let mut pgno_b: Pgno = 0 as Pgno;
        let mut pgno_temp: Pgno = 0 as Pgno;
        let mut fg_a: u16 = 0 as u16;
        let mut fg_b: u16 = 0 as u16;
        let mut p_old_1: *const MemPage = core::ptr::null();
        let mut p_old_2: *mut MemPage = core::ptr::null_mut();
        let mut p_new_1: *mut MemPage = core::ptr::null_mut();
        let mut cnt_old_next: i32 = 0;
        let mut i_new: i32 = 0;
        let mut i_old: i32 = 0;
        let mut p_cell: *mut u8 = core::ptr::null_mut();
        let mut p_cell_1: *mut u8 = core::ptr::null_mut();
        let mut p_temp_1: *mut u8 = core::ptr::null_mut();
        let mut sz__2: i32 = 0;
        let mut p_src_end: *const u8 = core::ptr::null();
        let mut p_new_2: *mut MemPage = core::ptr::null_mut();
        let mut info: CellInfo = unsafe { core::mem::zeroed() };
        let mut i_pg: i32 = 0;
        let mut i_new_1: i32 = 0;
        let mut i_old_1: i32 = 0;
        let mut n_new_cell: i32 = 0;
        let mut key: u32 = 0 as u32;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s74:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        unsafe {
                            sqlite3_db_free(core::ptr::null_mut(), b.ap_cell as *mut ())
                        };
                        __state = 430;
                    }
                    3 => { n_max_cells = 0; __state = 4; }
                    4 => { n_new = 0; __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { rc = 0; __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { i_space1 = 0; __state = 14; }
                    14 => { i_ovfl_space = 0; __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => { __state = 18; }
                    18 => { __state = 19; }
                    19 => { __state = 20; }
                    20 => { __state = 21; }
                    21 => { __state = 22; }
                    22 => { __state = 23; }
                    23 => { __state = 24; }
                    24 => { __state = 25; }
                    25 => { __state = 26; }
                    26 => { __state = 27; }
                    27 => { __state = 28; }
                    28 => {
                        unsafe {
                            memset(&raw mut ab_done[0 as usize] as *mut u8 as *mut (),
                                0, core::mem::size_of::<[u8; 5]>() as u64)
                        };
                        __state = 29;
                    }
                    29 => { { let _ = 0; }; __state = 30; }
                    30 => {
                        unsafe {
                            memset(&raw mut b as *mut (), 0,
                                core::mem::size_of::<CellArray>() as u64 -
                                    core::mem::size_of::<i32>() as u64)
                        };
                        __state = 31;
                    }
                    31 => {
                        b.ix_nx[(3 * 2 - 1) as usize] = 2147483647;
                        __state = 32;
                    }
                    32 => {
                        p_bt = unsafe { (*p_parent_1).p_bt };
                        __state = 33;
                    }
                    33 => { { let _ = 0; }; __state = 34; }
                    34 => { { let _ = 0; }; __state = 35; }
                    35 => { { let _ = 0; }; __state = 36; }
                    36 => { { let _ = 0; }; __state = 37; }
                    37 => {
                        if (a_ovfl_space_1).is_null() as i32 != 0 {
                            __state = 39;
                        } else { __state = 38; }
                    }
                    38 => { { let _ = 0; }; __state = 40; }
                    39 => { return 7; }
                    40 => {
                        i =
                            unsafe { (*p_parent_1).n_overflow } as i32 +
                                unsafe { (*p_parent_1).n_cell } as i32;
                        __state = 41;
                    }
                    41 => { if i < 2 { __state = 43; } else { __state = 44; } }
                    42 => { n_old = i + 1; __state = 51; }
                    43 => { nx_div = 0; __state = 42; }
                    44 => { { let _ = 0; }; __state = 45; }
                    45 => {
                        if i_parent_idx_1 == 0 {
                            __state = 47;
                        } else { __state = 48; }
                    }
                    46 => { i = 2 - b_bulk_1; __state = 42; }
                    47 => { nx_div = 0; __state = 46; }
                    48 => {
                        if i_parent_idx_1 == i {
                            __state = 49;
                        } else { __state = 50; }
                    }
                    49 => { nx_div = i - 2 + b_bulk_1; __state = 46; }
                    50 => { nx_div = i_parent_idx_1 - 1; __state = 46; }
                    51 => {
                        if i + nx_div - unsafe { (*p_parent_1).n_overflow } as i32
                                == unsafe { (*p_parent_1).n_cell } as i32 {
                            __state = 53;
                        } else { __state = 54; }
                    }
                    52 => {
                        pgno = unsafe { sqlite3_get4byte(p_right as *const u8) };
                        __state = 55;
                    }
                    53 => {
                        p_right =
                            unsafe {
                                unsafe {
                                    (*p_parent_1).a_data.offset((unsafe {
                                                        (*p_parent_1).hdr_offset
                                                    } as i32 + 8) as isize)
                                }
                            };
                        __state = 52;
                    }
                    54 => {
                        p_right =
                            unsafe {
                                unsafe {
                                    (*p_parent_1).a_data.offset((unsafe {
                                                        (*p_parent_1).mask_page
                                                    } as i32 &
                                                ((unsafe {
                                                                    *unsafe {
                                                                            unsafe {
                                                                                (*p_parent_1).a_cell_idx.offset((2 *
                                                                                                (i + nx_div - unsafe { (*p_parent_1).n_overflow } as i32))
                                                                                            as isize).offset(0 as isize)
                                                                            }
                                                                        }
                                                                } as i32) << 8 |
                                                    unsafe {
                                                            *unsafe {
                                                                    unsafe {
                                                                        (*p_parent_1).a_cell_idx.offset((2 *
                                                                                        (i + nx_div - unsafe { (*p_parent_1).n_overflow } as i32))
                                                                                    as isize).offset(1 as isize)
                                                                    }
                                                                }
                                                        } as i32)) as isize)
                                }
                            };
                        __state = 52;
                    }
                    55 => { if 1 != 0 { __state = 57; } else { __state = 56; } }
                    56 => { n_max_cells = n_max_cells + 3 & !3; __state = 85; }
                    57 => {
                        if rc == 0 { __state = 59; } else { __state = 58; }
                    }
                    58 => {
                        if rc != 0 { __state = 61; } else { __state = 60; }
                    }
                    59 => {
                        rc =
                            get_and_init_page(p_bt, pgno, &mut ap_old[i as usize], 0);
                        __state = 58;
                    }
                    60 => {
                        if unsafe { (*ap_old[i as usize]).n_free } < 0 {
                            __state = 64;
                        } else { __state = 63; }
                    }
                    61 => {
                        unsafe {
                            memset(&raw mut ap_old[0 as usize] as *mut *mut MemPage as
                                    *mut (), 0,
                                (i + 1) as u64 *
                                    core::mem::size_of::<*mut MemPage>() as u64)
                        };
                        __state = 62;
                    }
                    62 => { __state = 2; }
                    63 => {
                        n_max_cells +=
                            unsafe { (*ap_old[i as usize]).n_cell } as i32 +
                                (core::mem::size_of::<[*mut u8; 4]>() as u64 /
                                        core::mem::size_of::<*mut u8>() as u64) as i32;
                        __state = 68;
                    }
                    64 => {
                        rc =
                            btree_compute_free_space(unsafe {
                                    &mut *ap_old[i as usize]
                                });
                        __state = 65;
                    }
                    65 => {
                        if rc != 0 { __state = 66; } else { __state = 63; }
                    }
                    66 => {
                        unsafe {
                            memset(&raw mut ap_old[0 as usize] as *mut *mut MemPage as
                                    *mut (), 0,
                                i as u64 * core::mem::size_of::<*mut MemPage>() as u64)
                        };
                        __state = 67;
                    }
                    67 => { __state = 2; }
                    68 => {
                        if { let __p = &mut i; let __t = *__p; *__p -= 1; __t } == 0
                            {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    69 => {
                        if unsafe { (*p_parent_1).n_overflow } != 0 &&
                                i + nx_div ==
                                    unsafe { (*p_parent_1).ai_ovfl[0 as usize] } as i32 {
                            __state = 71;
                        } else { __state = 72; }
                    }
                    70 => { __state = 56; }
                    71 => {
                        ap_div[i as usize] =
                            unsafe { (*p_parent_1).ap_ovfl[0 as usize] };
                        __state = 73;
                    }
                    72 => {
                        ap_div[i as usize] =
                            unsafe {
                                unsafe {
                                    (*p_parent_1).a_data.offset((unsafe {
                                                        (*p_parent_1).mask_page
                                                    } as i32 &
                                                ((unsafe {
                                                                    *unsafe {
                                                                            unsafe {
                                                                                (*p_parent_1).a_cell_idx.offset((2 *
                                                                                                (i + nx_div - unsafe { (*p_parent_1).n_overflow } as i32))
                                                                                            as isize).offset(0 as isize)
                                                                            }
                                                                        }
                                                                } as i32) << 8 |
                                                    unsafe {
                                                            *unsafe {
                                                                    unsafe {
                                                                        (*p_parent_1).a_cell_idx.offset((2 *
                                                                                        (i + nx_div - unsafe { (*p_parent_1).n_overflow } as i32))
                                                                                    as isize).offset(1 as isize)
                                                                    }
                                                                }
                                                        } as i32)) as isize)
                                }
                            };
                        __state = 76;
                    }
                    73 => {
                        pgno =
                            unsafe {
                                sqlite3_get4byte(ap_div[i as usize] as *const u8)
                            };
                        __state = 74;
                    }
                    74 => {
                        sz_new[i as usize] =
                            unsafe {
                                    (unsafe {
                                            (*p_parent_1).x_cell_size.unwrap()
                                        })(p_parent_1, ap_div[i as usize])
                                } as i32;
                        __state = 75;
                    }
                    75 => {
                        unsafe { (*p_parent_1).n_overflow = 0 as u8 };
                        __state = 55;
                    }
                    76 => {
                        pgno =
                            unsafe {
                                sqlite3_get4byte(ap_div[i as usize] as *const u8)
                            };
                        __state = 77;
                    }
                    77 => {
                        sz_new[i as usize] =
                            unsafe {
                                    (unsafe {
                                            (*p_parent_1).x_cell_size.unwrap()
                                        })(p_parent_1, ap_div[i as usize])
                                } as i32;
                        __state = 78;
                    }
                    78 => {
                        if unsafe { (*p_bt).bts_flags } as i32 & 12 != 0 {
                            __state = 80;
                        } else { __state = 79; }
                    }
                    79 => {
                        drop_cell(p_parent_1,
                            i + nx_div - unsafe { (*p_parent_1).n_overflow } as i32,
                            sz_new[i as usize], &mut rc);
                        __state = 55;
                    }
                    80 => { __state = 81; }
                    81 => {
                        i_off =
                            ap_div[i as usize] as i64 as i32 -
                                unsafe { (*p_parent_1).a_data } as i64 as i32;
                        __state = 82;
                    }
                    82 => {
                        if i_off + sz_new[i as usize] <=
                                unsafe { (*p_bt).usable_size } as i32 {
                            __state = 83;
                        } else { __state = 79; }
                    }
                    83 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *a_ovfl_space_1.offset(i_off as isize)
                                    } as *mut (), ap_div[i as usize] as *const (),
                                sz_new[i as usize] as u64)
                        };
                        __state = 84;
                    }
                    84 => {
                        ap_div[i as usize] =
                            unsafe {
                                a_ovfl_space_1.offset(unsafe {
                                                ap_div[i as
                                                            usize].offset_from(unsafe { (*p_parent_1).a_data })
                                            } as i64 as isize)
                            };
                        __state = 79;
                    }
                    85 => {
                        sz_scratch =
                            (n_max_cells as u64 * core::mem::size_of::<*mut u8>() as u64
                                        + n_max_cells as u64 * core::mem::size_of::<u16>() as u64 +
                                    unsafe { (*p_bt).page_size } as u64) as u64;
                        __state = 86;
                    }
                    86 => { { let _ = 0; }; __state = 87; }
                    87 => {
                        b.ap_cell =
                            unsafe {
                                    sqlite3_db_malloc_raw(core::ptr::null_mut(), sz_scratch)
                                } as *mut *mut u8;
                        __state = 88;
                    }
                    88 => {
                        if b.ap_cell == core::ptr::null_mut() {
                            __state = 90;
                        } else { __state = 89; }
                    }
                    89 => {
                        b.sz_cell =
                            unsafe { &raw mut *b.ap_cell.offset(n_max_cells as isize) }
                                as *mut u16;
                        __state = 92;
                    }
                    90 => { rc = 7; __state = 91; }
                    91 => { __state = 2; }
                    92 => {
                        a_space1 =
                            unsafe { &raw mut *b.sz_cell.offset(n_max_cells as isize) }
                                as *mut u8;
                        __state = 93;
                    }
                    93 => { { let _ = 0; }; __state = 94; }
                    94 => { b.p_ref = ap_old[0 as usize]; __state = 95; }
                    95 => {
                        leaf_correction =
                            (unsafe { (*b.p_ref).leaf } as i32 * 4) as u16;
                        __state = 96;
                    }
                    96 => {
                        leaf_data = unsafe { (*b.p_ref).int_key_leaf } as i32;
                        __state = 97;
                    }
                    97 => { i = 0; __state = 99; }
                    98 => {
                        usable_space =
                            (unsafe { (*p_bt).usable_size } - 12 as u32 +
                                    leaf_correction as u32) as i32;
                        __state = 160;
                    }
                    99 => {
                        if i < n_old { __state = 100; } else { __state = 98; }
                    }
                    100 => {
                        p_old = ap_old[i as usize] as *const MemPage;
                        __state = 102;
                    }
                    101 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 99;
                    }
                    102 => {
                        limit = unsafe { (*p_old).n_cell } as i32;
                        __state = 103;
                    }
                    103 => {
                        a_data = unsafe { (*p_old).a_data };
                        __state = 104;
                    }
                    104 => {
                        mask_page = unsafe { (*p_old).mask_page };
                        __state = 105;
                    }
                    105 => {
                        pi_cell =
                            unsafe {
                                a_data.add(unsafe { (*p_old).cell_offset } as usize)
                            };
                        __state = 106;
                    }
                    106 => { __state = 107; }
                    107 => {
                        if unsafe { *unsafe { (*p_old).a_data.offset(0 as isize) } }
                                    as i32 !=
                                unsafe {
                                        *unsafe { (*ap_old[0 as usize]).a_data.offset(0 as isize) }
                                    } as i32 {
                            __state = 109;
                        } else { __state = 108; }
                    }
                    108 => {
                        unsafe {
                            memset(unsafe {
                                        &raw mut *b.sz_cell.offset(b.n_cell as isize)
                                    } as *mut (), 0,
                                core::mem::size_of::<u16>() as u64 *
                                    (limit + unsafe { (*p_old).n_overflow } as i32) as u64)
                        };
                        __state = 111;
                    }
                    109 => {
                        rc = unsafe { sqlite3_corrupt_error(8473) };
                        __state = 110;
                    }
                    110 => { __state = 2; }
                    111 => {
                        if unsafe { (*p_old).n_overflow } as i32 > 0 {
                            __state = 113;
                        } else { __state = 112; }
                    }
                    112 => {
                        pi_end =
                            unsafe {
                                unsafe {
                                    a_data.add(unsafe { (*p_old).cell_offset } as
                                                usize).offset((2 * unsafe { (*p_old).n_cell } as i32) as
                                            isize)
                                }
                            };
                        __state = 129;
                    }
                    113 => {
                        if limit < unsafe { (*p_old).ai_ovfl[0 as usize] } as i32 {
                            __state = 115;
                        } else { __state = 114; }
                    }
                    114 => {
                        limit = unsafe { (*p_old).ai_ovfl[0 as usize] } as i32;
                        __state = 117;
                    }
                    115 => {
                        rc = unsafe { sqlite3_corrupt_error(8497) };
                        __state = 116;
                    }
                    116 => { __state = 2; }
                    117 => { j = 0; __state = 119; }
                    118 => { k = 0; __state = 124; }
                    119 => {
                        if j < limit { __state = 120; } else { __state = 118; }
                    }
                    120 => {
                        unsafe {
                            *b.ap_cell.offset(b.n_cell as isize) =
                                unsafe {
                                    a_data.offset((mask_page as i32 &
                                                ((unsafe { *pi_cell.offset(0 as isize) } as i32) << 8 |
                                                    unsafe { *pi_cell.offset(1 as isize) } as i32)) as isize)
                                }
                        };
                        __state = 122;
                    }
                    121 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 119;
                    }
                    122 => {
                        {
                            let __n = 2;
                            let __p = &mut pi_cell;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 123;
                    }
                    123 => {
                        { let __p = &mut b.n_cell; let __t = *__p; *__p += 1; __t };
                        __state = 121;
                    }
                    124 => {
                        if k < unsafe { (*p_old).n_overflow } as i32 {
                            __state = 125;
                        } else { __state = 112; }
                    }
                    125 => { { let _ = 0; }; __state = 127; }
                    126 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 124;
                    }
                    127 => {
                        unsafe {
                            *b.ap_cell.offset(b.n_cell as isize) =
                                unsafe { (*p_old).ap_ovfl[k as usize] }
                        };
                        __state = 128;
                    }
                    128 => {
                        { let __p = &mut b.n_cell; let __t = *__p; *__p += 1; __t };
                        __state = 126;
                    }
                    129 => {
                        if pi_cell < pi_end {
                            __state = 131;
                        } else { __state = 130; }
                    }
                    130 => { { let _ = 0; }; __state = 135; }
                    131 => { { let _ = 0; }; __state = 132; }
                    132 => {
                        unsafe {
                            *b.ap_cell.offset(b.n_cell as isize) =
                                unsafe {
                                    a_data.offset((mask_page as i32 &
                                                ((unsafe { *pi_cell.offset(0 as isize) } as i32) << 8 |
                                                    unsafe { *pi_cell.offset(1 as isize) } as i32)) as isize)
                                }
                        };
                        __state = 133;
                    }
                    133 => {
                        {
                            let __n = 2;
                            let __p = &mut pi_cell;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 134;
                    }
                    134 => {
                        { let __p = &mut b.n_cell; let __t = *__p; *__p += 1; __t };
                        __state = 129;
                    }
                    135 => { cnt_old[i as usize] = b.n_cell; __state = 136; }
                    136 => {
                        if i < n_old - 1 && (leaf_data == 0) as i32 != 0 {
                            __state = 137;
                        } else { __state = 101; }
                    }
                    137 => { sz = sz_new[i as usize] as u16; __state = 138; }
                    138 => { __state = 139; }
                    139 => { { let _ = 0; }; __state = 140; }
                    140 => {
                        unsafe { *b.sz_cell.offset(b.n_cell as isize) = sz };
                        __state = 141;
                    }
                    141 => {
                        p_temp = unsafe { a_space1.offset(i_space1 as isize) };
                        __state = 142;
                    }
                    142 => { i_space1 += sz as i32; __state = 143; }
                    143 => { { let _ = 0; }; __state = 144; }
                    144 => { { let _ = 0; }; __state = 145; }
                    145 => {
                        unsafe {
                            memcpy(p_temp as *mut (), ap_div[i as usize] as *const (),
                                sz as u64)
                        };
                        __state = 146;
                    }
                    146 => {
                        unsafe {
                            *b.ap_cell.offset(b.n_cell as isize) =
                                unsafe { p_temp.add(leaf_correction as usize) }
                        };
                        __state = 147;
                    }
                    147 => { { let _ = 0; }; __state = 148; }
                    148 => {
                        unsafe {
                            *b.sz_cell.offset(b.n_cell as isize) =
                                (unsafe { *b.sz_cell.offset(b.n_cell as isize) } as i32 -
                                        leaf_correction as i32) as u16
                        };
                        __state = 149;
                    }
                    149 => {
                        if (unsafe { (*p_old).leaf } == 0) as i32 != 0 {
                            __state = 151;
                        } else { __state = 152; }
                    }
                    150 => {
                        { let __p = &mut b.n_cell; let __t = *__p; *__p += 1; __t };
                        __state = 101;
                    }
                    151 => { { let _ = 0; }; __state = 153; }
                    152 => { { let _ = 0; }; __state = 155; }
                    153 => { { let _ = 0; }; __state = 154; }
                    154 => {
                        unsafe {
                            memcpy(unsafe { *b.ap_cell.offset(b.n_cell as isize) } as
                                    *mut (),
                                unsafe {
                                        &raw mut *unsafe { (*p_old).a_data.offset(8 as isize) }
                                    } as *const (), 4 as u64)
                        };
                        __state = 150;
                    }
                    155 => {
                        if (unsafe { *b.sz_cell.offset(b.n_cell as isize) } as i32)
                                < 4 {
                            __state = 156;
                        } else { __state = 150; }
                    }
                    156 => { { let _ = 0; }; __state = 157; }
                    157 => { { let _ = 0; }; __state = 158; }
                    158 => {
                        unsafe {
                            *a_space1.offset({
                                                let __p = &mut i_space1;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = 0 as u8
                        };
                        __state = 159;
                    }
                    159 => {
                        {
                            let __p =
                                unsafe { &mut *b.sz_cell.offset(b.n_cell as isize) };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 155;
                    }
                    160 => { i = { k = 0; k }; __state = 162; }
                    161 => { k = n_old; __state = 180; }
                    162 => {
                        if i < n_old { __state = 163; } else { __state = 161; }
                    }
                    163 => { p = ap_old[i as usize]; __state = 165; }
                    164 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t }
                        };
                        __state = 162;
                    }
                    165 => {
                        b.ap_end[k as usize] = unsafe { (*p).a_data_end };
                        __state = 166;
                    }
                    166 => {
                        b.ix_nx[k as usize] = cnt_old[i as usize];
                        __state = 167;
                    }
                    167 => {
                        if k != 0 &&
                                b.ix_nx[k as usize] == b.ix_nx[(k - 1) as usize] {
                            __state = 169;
                        } else { __state = 168; }
                    }
                    168 => {
                        if (leaf_data == 0) as i32 != 0 {
                            __state = 171;
                        } else { __state = 170; }
                    }
                    169 => {
                        { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                        __state = 168;
                    }
                    170 => { { let _ = 0; }; __state = 174; }
                    171 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 172;
                    }
                    172 => {
                        b.ap_end[k as usize] = unsafe { (*p_parent_1).a_data_end };
                        __state = 173;
                    }
                    173 => {
                        b.ix_nx[k as usize] = cnt_old[i as usize] + 1;
                        __state = 170;
                    }
                    174 => {
                        sz_new[i as usize] = usable_space - unsafe { (*p).n_free };
                        __state = 175;
                    }
                    175 => { j = 0; __state = 177; }
                    176 => {
                        cnt_new[i as usize] = cnt_old[i as usize];
                        __state = 164;
                    }
                    177 => {
                        if j < unsafe { (*p).n_overflow } as i32 {
                            __state = 178;
                        } else { __state = 176; }
                    }
                    178 => {
                        sz_new[i as usize] +=
                            2 +
                                unsafe {
                                        (unsafe {
                                                (*p).x_cell_size.unwrap()
                                            })(p, unsafe { (*p).ap_ovfl[j as usize] })
                                    } as i32;
                        __state = 179;
                    }
                    179 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 177;
                    }
                    180 => { i = 0; __state = 182; }
                    181 => { i = k - 1; __state = 218; }
                    182 => {
                        if i < k { __state = 183; } else { __state = 181; }
                    }
                    183 => { __state = 185; }
                    184 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 182;
                    }
                    185 => {
                        if sz_new[i as usize] > usable_space {
                            __state = 187;
                        } else { __state = 186; }
                    }
                    186 => {
                        if cnt_new[i as usize] < b.n_cell {
                            __state = 203;
                        } else { __state = 202; }
                    }
                    187 => {
                        if i + 1 >= k { __state = 189; } else { __state = 188; }
                    }
                    188 => {
                        sz__1 =
                            2 +
                                cached_cell_size(&mut b, cnt_new[i as usize] - 1) as i32;
                        __state = 195;
                    }
                    189 => { k = i + 2; __state = 190; }
                    190 => {
                        if k > 3 + 2 { __state = 192; } else { __state = 191; }
                    }
                    191 => { sz_new[(k - 1) as usize] = 0; __state = 194; }
                    192 => {
                        rc = unsafe { sqlite3_corrupt_error(8598) };
                        __state = 193;
                    }
                    193 => { __state = 2; }
                    194 => {
                        cnt_new[(k - 1) as usize] = b.n_cell;
                        __state = 188;
                    }
                    195 => { sz_new[i as usize] -= sz__1; __state = 196; }
                    196 => {
                        if (leaf_data == 0) as i32 != 0 {
                            __state = 198;
                        } else { __state = 197; }
                    }
                    197 => { sz_new[(i + 1) as usize] += sz__1; __state = 201; }
                    198 => {
                        if cnt_new[i as usize] < b.n_cell {
                            __state = 199;
                        } else { __state = 200; }
                    }
                    199 => {
                        sz__1 =
                            2 + cached_cell_size(&mut b, cnt_new[i as usize]) as i32;
                        __state = 197;
                    }
                    200 => { sz__1 = 0; __state = 197; }
                    201 => {
                        {
                            let __p = &mut cnt_new[i as usize];
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 185;
                    }
                    202 => {
                        if cnt_new[i as usize] >= b.n_cell {
                            __state = 213;
                        } else { __state = 214; }
                    }
                    203 => {
                        sz__1 =
                            2 + cached_cell_size(&mut b, cnt_new[i as usize]) as i32;
                        __state = 204;
                    }
                    204 => {
                        if sz_new[i as usize] + sz__1 > usable_space {
                            __state = 206;
                        } else { __state = 205; }
                    }
                    205 => { sz_new[i as usize] += sz__1; __state = 207; }
                    206 => { __state = 202; }
                    207 => {
                        {
                            let __p = &mut cnt_new[i as usize];
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 208;
                    }
                    208 => {
                        if (leaf_data == 0) as i32 != 0 {
                            __state = 210;
                        } else { __state = 209; }
                    }
                    209 => { sz_new[(i + 1) as usize] -= sz__1; __state = 186; }
                    210 => {
                        if cnt_new[i as usize] < b.n_cell {
                            __state = 211;
                        } else { __state = 212; }
                    }
                    211 => {
                        sz__1 =
                            2 + cached_cell_size(&mut b, cnt_new[i as usize]) as i32;
                        __state = 209;
                    }
                    212 => { sz__1 = 0; __state = 209; }
                    213 => { k = i + 1; __state = 184; }
                    214 => {
                        if cnt_new[i as usize] <=
                                if i > 0 { cnt_new[(i - 1) as usize] } else { 0 } {
                            __state = 215;
                        } else { __state = 184; }
                    }
                    215 => {
                        rc = unsafe { sqlite3_corrupt_error(8631) };
                        __state = 216;
                    }
                    216 => { __state = 2; }
                    217 => { { let _ = 0; }; __state = 245; }
                    218 => {
                        if i > 0 { __state = 219; } else { __state = 217; }
                    }
                    219 => { sz_right = sz_new[i as usize]; __state = 221; }
                    220 => {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        __state = 218;
                    }
                    221 => {
                        sz_left = sz_new[(i - 1) as usize];
                        __state = 222;
                    }
                    222 => { __state = 223; }
                    223 => { __state = 224; }
                    224 => { r = cnt_new[(i - 1) as usize] - 1; __state = 225; }
                    225 => { d = r + 1 - leaf_data; __state = 226; }
                    226 => {
                        { let _ = cached_cell_size(&mut b, d); };
                        __state = 227;
                    }
                    227 => { __state = 230; }
                    228 => { sz_new[i as usize] = sz_right; __state = 241; }
                    229 => {
                        if r >= 0 { __state = 227; } else { __state = 228; }
                    }
                    230 => { { let _ = 0; }; __state = 231; }
                    231 => { { let _ = 0; }; __state = 232; }
                    232 => {
                        sz_r = cached_cell_size(&mut b, r) as i32;
                        __state = 233;
                    }
                    233 => {
                        sz_d = unsafe { *b.sz_cell.offset(d as isize) } as i32;
                        __state = 234;
                    }
                    234 => {
                        if sz_right != 0 &&
                                (b_bulk_1 != 0 ||
                                    sz_right + sz_d + 2 >
                                        sz_left - (sz_r + if i == k - 1 { 0 } else { 2 })) {
                            __state = 236;
                        } else { __state = 235; }
                    }
                    235 => { sz_right += sz_d + 2; __state = 237; }
                    236 => { __state = 228; }
                    237 => { sz_left -= sz_r + 2; __state = 238; }
                    238 => { cnt_new[(i - 1) as usize] = r; __state = 239; }
                    239 => {
                        { let __p = &mut r; let __t = *__p; *__p -= 1; __t };
                        __state = 240;
                    }
                    240 => {
                        { let __p = &mut d; let __t = *__p; *__p -= 1; __t };
                        __state = 229;
                    }
                    241 => {
                        sz_new[(i - 1) as usize] = sz_left;
                        __state = 242;
                    }
                    242 => {
                        if cnt_new[(i - 1) as usize] <=
                                if i > 1 { cnt_new[(i - 2) as usize] } else { 0 } {
                            __state = 243;
                        } else { __state = 220; }
                    }
                    243 => {
                        rc = unsafe { sqlite3_corrupt_error(8675) };
                        __state = 244;
                    }
                    244 => { __state = 2; }
                    245 => { __state = 246; }
                    246 => {
                        page_flags =
                            unsafe {
                                    *unsafe { (*ap_old[0 as usize]).a_data.offset(0 as isize) }
                                } as i32;
                        __state = 247;
                    }
                    247 => { i = 0; __state = 249; }
                    248 => { i = 0; __state = 274; }
                    249 => {
                        if i < k { __state = 250; } else { __state = 248; }
                    }
                    250 => { __state = 252; }
                    251 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 249;
                    }
                    252 => {
                        if i < n_old { __state = 253; } else { __state = 254; }
                    }
                    253 => {
                        p_new =
                            {
                                ap_new[i as usize] = ap_old[i as usize];
                                ap_new[i as usize]
                            };
                        __state = 255;
                    }
                    254 => { { let _ = 0; }; __state = 262; }
                    255 => {
                        ap_old[i as usize] = core::ptr::null_mut();
                        __state = 256;
                    }
                    256 => {
                        rc =
                            unsafe {
                                sqlite3_pager_write(unsafe { (*p_new).p_db_page })
                            };
                        __state = 257;
                    }
                    257 => {
                        { let __p = &mut n_new; let __t = *__p; *__p += 1; __t };
                        __state = 258;
                    }
                    258 => {
                        if unsafe {
                                        sqlite3_pager_page_refcount(unsafe { (*p_new).p_db_page })
                                    } != 1 + (i == i_parent_idx_1 - nx_div) as i32 && rc == 0 {
                            __state = 260;
                        } else { __state = 259; }
                    }
                    259 => {
                        if rc != 0 { __state = 261; } else { __state = 251; }
                    }
                    260 => {
                        rc = unsafe { sqlite3_corrupt_error(8708) };
                        __state = 259;
                    }
                    261 => { __state = 2; }
                    262 => {
                        rc =
                            allocate_btree_page(p_bt, &mut p_new, &mut pgno,
                                if b_bulk_1 != 0 { 1 as Pgno } else { pgno }, 0 as u8);
                        __state = 263;
                    }
                    263 => {
                        if rc != 0 { __state = 265; } else { __state = 264; }
                    }
                    264 => { zero_page(p_new, page_flags); __state = 266; }
                    265 => { __state = 2; }
                    266 => { ap_new[i as usize] = p_new; __state = 267; }
                    267 => {
                        { let __p = &mut n_new; let __t = *__p; *__p += 1; __t };
                        __state = 268;
                    }
                    268 => { cnt_old[i as usize] = b.n_cell; __state = 269; }
                    269 => {
                        if unsafe { (*p_bt).auto_vacuum } != 0 {
                            __state = 270;
                        } else { __state = 251; }
                    }
                    270 => {
                        ptrmap_put(p_bt, unsafe { (*p_new).pgno }, 5 as u8,
                            unsafe { (*p_parent_1).pgno }, &mut rc);
                        __state = 271;
                    }
                    271 => {
                        if rc != 0 { __state = 272; } else { __state = 251; }
                    }
                    272 => { __state = 2; }
                    273 => { i = 0; __state = 280; }
                    274 => {
                        if i < n_new { __state = 275; } else { __state = 273; }
                    }
                    275 => {
                        a_pgno[i as usize] = unsafe { (*ap_new[i as usize]).pgno };
                        __state = 277;
                    }
                    276 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 274;
                    }
                    277 => { { let _ = 0; }; __state = 278; }
                    278 => { { let _ = 0; }; __state = 276; }
                    279 => { __state = 299; }
                    280 => {
                        if i < n_new - 1 { __state = 281; } else { __state = 279; }
                    }
                    281 => { i_b = i; __state = 283; }
                    282 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 280;
                    }
                    283 => { j = i + 1; __state = 285; }
                    284 => {
                        if i_b != i { __state = 289; } else { __state = 282; }
                    }
                    285 => {
                        if j < n_new { __state = 286; } else { __state = 284; }
                    }
                    286 => {
                        if unsafe { (*ap_new[j as usize]).pgno } <
                                unsafe { (*ap_new[i_b as usize]).pgno } {
                            __state = 288;
                        } else { __state = 287; }
                    }
                    287 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 285;
                    }
                    288 => { i_b = j; __state = 287; }
                    289 => {
                        pgno_a = unsafe { (*ap_new[i as usize]).pgno };
                        __state = 290;
                    }
                    290 => {
                        pgno_b = unsafe { (*ap_new[i_b as usize]).pgno };
                        __state = 291;
                    }
                    291 => {
                        pgno_temp =
                            sqlite3_pending_byte as u32 / unsafe { (*p_bt).page_size } +
                                1 as u32;
                        __state = 292;
                    }
                    292 => {
                        fg_a =
                            unsafe {
                                (*unsafe { (*ap_new[i as usize]).p_db_page }).flags
                            };
                        __state = 293;
                    }
                    293 => {
                        fg_b =
                            unsafe {
                                (*unsafe { (*ap_new[i_b as usize]).p_db_page }).flags
                            };
                        __state = 294;
                    }
                    294 => {
                        unsafe {
                            sqlite3_pager_rekey(unsafe {
                                    (*ap_new[i as usize]).p_db_page
                                }, pgno_temp, fg_b)
                        };
                        __state = 295;
                    }
                    295 => {
                        unsafe {
                            sqlite3_pager_rekey(unsafe {
                                    (*ap_new[i_b as usize]).p_db_page
                                }, pgno_a, fg_a)
                        };
                        __state = 296;
                    }
                    296 => {
                        unsafe {
                            sqlite3_pager_rekey(unsafe {
                                    (*ap_new[i as usize]).p_db_page
                                }, pgno_b, fg_b)
                        };
                        __state = 297;
                    }
                    297 => {
                        unsafe { (*ap_new[i as usize]).pgno = pgno_b };
                        __state = 298;
                    }
                    298 => {
                        unsafe { (*ap_new[i_b as usize]).pgno = pgno_a };
                        __state = 282;
                    }
                    299 => { { let _ = 0; }; __state = 300; }
                    300 => { { let _ = 0; }; __state = 301; }
                    301 => { { let _ = 0; }; __state = 302; }
                    302 => {
                        unsafe {
                            sqlite3_put4byte(p_right,
                                unsafe { (*ap_new[(n_new - 1) as usize]).pgno })
                        };
                        __state = 303;
                    }
                    303 => {
                        if page_flags & 8 == 0 && n_old != n_new {
                            __state = 305;
                        } else { __state = 304; }
                    }
                    304 => {
                        if unsafe { (*p_bt).auto_vacuum } != 0 {
                            __state = 311;
                        } else { __state = 310; }
                    }
                    305 => { __state = 306; }
                    306 => {
                        if n_new > n_old { __state = 308; } else { __state = 309; }
                    }
                    307 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe {
                                                    (*ap_new[(n_new - 1) as usize]).a_data.offset(8 as isize)
                                                }
                                    } as *mut (),
                                unsafe {
                                        &raw mut *unsafe { (*p_old_1).a_data.offset(8 as isize) }
                                    } as *const (), 4 as u64)
                        };
                        __state = 304;
                    }
                    308 => {
                        p_old_1 = ap_new[(n_old - 1) as usize];
                        __state = 307;
                    }
                    309 => {
                        p_old_1 = ap_old[(n_old - 1) as usize];
                        __state = 307;
                    }
                    310 => { i = 0; __state = 338; }
                    311 => { __state = 312; }
                    312 => {
                        p_new_1 = { p_old_2 = ap_new[0 as usize]; p_old_2 };
                        __state = 313;
                    }
                    313 => {
                        cnt_old_next =
                            unsafe { (*p_new_1).n_cell } as i32 +
                                unsafe { (*p_new_1).n_overflow } as i32;
                        __state = 314;
                    }
                    314 => { i_new = 0; __state = 315; }
                    315 => { i_old = 0; __state = 316; }
                    316 => { i = 0; __state = 317; }
                    317 => {
                        if i < b.n_cell { __state = 318; } else { __state = 310; }
                    }
                    318 => {
                        p_cell = unsafe { *b.ap_cell.offset(i as isize) };
                        __state = 320;
                    }
                    319 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 317;
                    }
                    320 => {
                        if i == cnt_old_next {
                            __state = 322;
                        } else { __state = 321; }
                    }
                    321 => {
                        if i == cnt_new[i_new as usize] {
                            __state = 328;
                        } else { __state = 327; }
                    }
                    322 => {
                        { let __p = &mut i_old; let __t = *__p; *__p += 1; __t };
                        __state = 323;
                    }
                    323 => { { let _ = 0; }; __state = 324; }
                    324 => { { let _ = 0; }; __state = 325; }
                    325 => {
                        p_old_2 =
                            if i_old < n_new {
                                ap_new[i_old as usize]
                            } else { ap_old[i_old as usize] };
                        __state = 326;
                    }
                    326 => {
                        cnt_old_next +=
                            unsafe { (*p_old_2).n_cell } as i32 +
                                    unsafe { (*p_old_2).n_overflow } as i32 +
                                (leaf_data == 0) as i32 as i32;
                        __state = 320;
                    }
                    327 => {
                        if i_old >= n_new ||
                                    unsafe { (*p_new_1).pgno } != a_pgno[i_old as usize] ||
                                !(p_cell as uptr >= unsafe { (*p_old_2).a_data } as uptr &&
                                                (p_cell as uptr) < unsafe { (*p_old_2).a_data_end } as uptr)
                                        as i32 != 0 {
                            __state = 331;
                        } else { __state = 319; }
                    }
                    328 => {
                        p_new_1 =
                            ap_new[{ let __p = &mut i_new; *__p += 1; *__p } as usize];
                        __state = 329;
                    }
                    329 => {
                        if (leaf_data == 0) as i32 != 0 {
                            __state = 330;
                        } else { __state = 327; }
                    }
                    330 => { __state = 319; }
                    331 => {
                        if (leaf_correction == 0) as i32 != 0 {
                            __state = 333;
                        } else { __state = 332; }
                    }
                    332 => {
                        if cached_cell_size(&mut b, i) as i32 >
                                unsafe { (*p_new_1).min_local } as i32 {
                            __state = 335;
                        } else { __state = 334; }
                    }
                    333 => {
                        ptrmap_put(p_bt,
                            unsafe { sqlite3_get4byte(p_cell as *const u8) }, 5 as u8,
                            unsafe { (*p_new_1).pgno }, &mut rc);
                        __state = 332;
                    }
                    334 => {
                        if rc != 0 { __state = 336; } else { __state = 319; }
                    }
                    335 => {
                        ptrmap_put_ovfl_ptr(p_new_1, unsafe { &*p_old_2 }, p_cell,
                            &mut rc);
                        __state = 334;
                    }
                    336 => { __state = 2; }
                    337 => { i = 1 - n_new; __state = 381; }
                    338 => {
                        if i < n_new - 1 { __state = 339; } else { __state = 337; }
                    }
                    339 => { __state = 341; }
                    340 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 338;
                    }
                    341 => { __state = 342; }
                    342 => { __state = 343; }
                    343 => { __state = 344; }
                    344 => { p_new_2 = ap_new[i as usize]; __state = 345; }
                    345 => { j = cnt_new[i as usize]; __state = 346; }
                    346 => { { let _ = 0; }; __state = 347; }
                    347 => { { let _ = 0; }; __state = 348; }
                    348 => {
                        p_cell_1 = unsafe { *b.ap_cell.offset(j as isize) };
                        __state = 349;
                    }
                    349 => {
                        sz__2 =
                            unsafe { *b.sz_cell.offset(j as isize) } as i32 +
                                leaf_correction as i32;
                        __state = 350;
                    }
                    350 => {
                        p_temp_1 =
                            unsafe { a_ovfl_space_1.offset(i_ovfl_space as isize) };
                        __state = 351;
                    }
                    351 => {
                        if (unsafe { (*p_new_2).leaf } == 0) as i32 != 0 {
                            __state = 353;
                        } else { __state = 354; }
                    }
                    352 => { i_ovfl_space += sz__2; __state = 365; }
                    353 => {
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe { (*p_new_2).a_data.offset(8 as isize) }
                                    } as *mut (), p_cell_1 as *const (), 4 as u64)
                        };
                        __state = 352;
                    }
                    354 => {
                        if leaf_data != 0 { __state = 355; } else { __state = 356; }
                    }
                    355 => { __state = 357; }
                    356 => {
                        {
                            let __n = 4;
                            let __p = &mut p_cell_1;
                            *__p = unsafe { (*__p).offset(-(__n as isize)) };
                        };
                        __state = 362;
                    }
                    357 => {
                        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                        __state = 358;
                    }
                    358 => {
                        unsafe {
                            (unsafe {
                                    (*p_new_2).x_parse_cell.unwrap()
                                })(p_new_2, unsafe { *b.ap_cell.offset(j as isize) },
                                &mut info)
                        };
                        __state = 359;
                    }
                    359 => { p_cell_1 = p_temp_1; __state = 360; }
                    360 => {
                        sz__2 =
                            4 +
                                unsafe {
                                    sqlite3_put_varint(unsafe {
                                            &mut *p_cell_1.offset(4 as isize)
                                        }, info.n_key as u64)
                                };
                        __state = 361;
                    }
                    361 => { p_temp_1 = core::ptr::null_mut(); __state = 352; }
                    362 => {
                        if unsafe { *b.sz_cell.offset(j as isize) } as i32 == 4 {
                            __state = 363;
                        } else { __state = 352; }
                    }
                    363 => { { let _ = 0; }; __state = 364; }
                    364 => {
                        sz__2 =
                            unsafe {
                                    (unsafe {
                                            (*p_parent_1).x_cell_size.unwrap()
                                        })(p_parent_1, p_cell_1)
                                } as i32;
                        __state = 352;
                    }
                    365 => { { let _ = 0; }; __state = 366; }
                    366 => { { let _ = 0; }; __state = 367; }
                    367 => { { let _ = 0; }; __state = 368; }
                    368 => { k = 0; __state = 370; }
                    369 => { p_src_end = b.ap_end[k as usize]; __state = 373; }
                    370 => {
                        if b.ix_nx[k as usize] <= j {
                            __state = 371;
                        } else { __state = 369; }
                    }
                    371 => { __state = 372; }
                    372 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 370;
                    }
                    373 => {
                        if (p_cell_1 as uptr) < p_src_end as uptr &&
                                unsafe { p_cell_1.offset(sz__2 as isize) } as uptr >
                                    p_src_end as uptr {
                            __state = 375;
                        } else { __state = 374; }
                    }
                    374 => {
                        rc =
                            insert_cell(p_parent_1, nx_div + i, p_cell_1, sz__2,
                                p_temp_1, unsafe { (*p_new_2).pgno });
                        __state = 377;
                    }
                    375 => {
                        rc = unsafe { sqlite3_corrupt_error(8914) };
                        __state = 376;
                    }
                    376 => { __state = 2; }
                    377 => {
                        if rc != 0 { __state = 379; } else { __state = 378; }
                    }
                    378 => { { let _ = 0; }; __state = 340; }
                    379 => { __state = 2; }
                    380 => { { let _ = 0; }; __state = 408; }
                    381 => {
                        if i < n_new { __state = 382; } else { __state = 380; }
                    }
                    382 => { i_pg = if i < 0 { -i } else { i }; __state = 384; }
                    383 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 381;
                    }
                    384 => { { let _ = 0; }; __state = 385; }
                    385 => { { let _ = 0; }; __state = 386; }
                    386 => { { let _ = 0; }; __state = 387; }
                    387 => {
                        if ab_done[i_pg as usize] != 0 {
                            __state = 389;
                        } else { __state = 388; }
                    }
                    388 => {
                        if i >= 0 ||
                                cnt_old[(i_pg - 1) as usize] >= cnt_new[(i_pg - 1) as usize]
                            {
                            __state = 390;
                        } else { __state = 383; }
                    }
                    389 => { __state = 383; }
                    390 => { __state = 391; }
                    391 => { __state = 392; }
                    392 => { __state = 393; }
                    393 => { { let _ = 0; }; __state = 394; }
                    394 => { { let _ = 0; }; __state = 395; }
                    395 => {
                        if i_pg == 0 { __state = 397; } else { __state = 398; }
                    }
                    396 => {
                        rc =
                            edit_page(ap_new[i_pg as usize], i_old_1, i_new_1,
                                n_new_cell, &mut b);
                        __state = 402;
                    }
                    397 => {
                        i_new_1 = { i_old_1 = 0; i_old_1 };
                        __state = 399;
                    }
                    398 => {
                        i_old_1 =
                            if i_pg < n_old {
                                cnt_old[(i_pg - 1) as usize] +
                                    (leaf_data == 0) as i32 as i32
                            } else { b.n_cell };
                        __state = 400;
                    }
                    399 => { n_new_cell = cnt_new[0 as usize]; __state = 396; }
                    400 => {
                        i_new_1 =
                            cnt_new[(i_pg - 1) as usize] +
                                (leaf_data == 0) as i32 as i32;
                        __state = 401;
                    }
                    401 => {
                        n_new_cell = cnt_new[i_pg as usize] - i_new_1;
                        __state = 396;
                    }
                    402 => {
                        if rc != 0 { __state = 404; } else { __state = 403; }
                    }
                    403 => {
                        {
                            let __p = &mut ab_done[i_pg as usize];
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 405;
                    }
                    404 => { __state = 2; }
                    405 => {
                        unsafe {
                            (*ap_new[i_pg as usize]).n_free =
                                usable_space - sz_new[i_pg as usize]
                        };
                        __state = 406;
                    }
                    406 => { { let _ = 0; }; __state = 407; }
                    407 => { { let _ = 0; }; __state = 383; }
                    408 => { { let _ = 0; }; __state = 409; }
                    409 => { { let _ = 0; }; __state = 410; }
                    410 => {
                        if is_root_1 != 0 &&
                                    unsafe { (*p_parent_1).n_cell } as i32 == 0 &&
                                unsafe { (*p_parent_1).hdr_offset } as i32 <=
                                    unsafe { (*ap_new[0 as usize]).n_free } {
                            __state = 412;
                        } else { __state = 413; }
                    }
                    411 => { { let _ = 0; }; __state = 424; }
                    412 => { { let _ = 0; }; __state = 414; }
                    413 => {
                        if unsafe { (*p_bt).auto_vacuum } != 0 &&
                                (leaf_correction == 0) as i32 != 0 {
                            __state = 419;
                        } else { __state = 411; }
                    }
                    414 => {
                        rc = defragment_page(ap_new[0 as usize], -1);
                        __state = 415;
                    }
                    415 => { __state = 416; }
                    416 => { { let _ = 0; }; __state = 417; }
                    417 => {
                        copy_node_content(unsafe { &*ap_new[0 as usize] },
                            p_parent_1, &mut rc);
                        __state = 418;
                    }
                    418 => {
                        free_page(ap_new[0 as usize], &mut rc);
                        __state = 411;
                    }
                    419 => { i = 0; __state = 420; }
                    420 => {
                        if i < n_new { __state = 421; } else { __state = 411; }
                    }
                    421 => {
                        key =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *unsafe {
                                                        (*ap_new[i as usize]).a_data.offset(8 as isize)
                                                    }
                                        } as *const u8)
                            };
                        __state = 423;
                    }
                    422 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 420;
                    }
                    423 => {
                        ptrmap_put(p_bt, key, 5 as u8,
                            unsafe { (*ap_new[i as usize]).pgno }, &mut rc);
                        __state = 422;
                    }
                    424 => { __state = 425; }
                    425 => { i = n_new; __state = 427; }
                    426 => { __state = 2; }
                    427 => {
                        if i < n_old { __state = 428; } else { __state = 426; }
                    }
                    428 => {
                        free_page(ap_old[i as usize], &mut rc);
                        __state = 429;
                    }
                    429 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 427;
                    }
                    430 => { i = 0; __state = 432; }
                    431 => { i = 0; __state = 436; }
                    432 => {
                        if i < n_old { __state = 433; } else { __state = 431; }
                    }
                    433 => { release_page(ap_old[i as usize]); __state = 434; }
                    434 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 432;
                    }
                    435 => { return rc; }
                    436 => {
                        if i < n_new { __state = 437; } else { __state = 435; }
                    }
                    437 => { release_page(ap_new[i as usize]); __state = 438; }
                    438 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 436;
                    }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn balance(p_cur_1: *mut BtCursor) -> i32 {
    let mut rc: i32 = 0;
    let mut a_balance_quick_space: [u8; 13] = [0; 13];
    let mut p_free: *mut u8 = core::ptr::null_mut();
    '__b75: loop {
        '__c75: loop {
            let mut i_page: i32 = 0;
            let p_page: *mut MemPage = unsafe { (*p_cur_1).p_page };
            if unsafe { (*p_page).n_free } < 0 &&
                    btree_compute_free_space(unsafe { &mut *p_page }) != 0 {
                break '__b75;
            }
            if unsafe { (*p_page).n_overflow } as i32 == 0 &&
                    unsafe { (*p_page).n_free } * 3 <=
                        unsafe { (*unsafe { (*p_cur_1).p_bt }).usable_size } as i32
                            * 2 {
                break '__b75;
            } else if { i_page = unsafe { (*p_cur_1).i_page } as i32; i_page }
                    == 0 {
                if unsafe { (*p_page).n_overflow } != 0 &&
                        { rc = another_valid_cursor(p_cur_1); rc } == 0 {
                    { let _ = 0; };
                    rc =
                        balance_deeper(p_page,
                            unsafe { &mut (*p_cur_1).ap_page[1 as usize] });
                    if rc == 0 {
                        unsafe { (*p_cur_1).i_page = 1 as i8 };
                        unsafe { (*p_cur_1).ix = 0 as u16 };
                        unsafe { (*p_cur_1).ai_idx[0 as usize] = 0 as u16 };
                        unsafe { (*p_cur_1).ap_page[0 as usize] = p_page };
                        unsafe {
                            (*p_cur_1).p_page =
                                unsafe { (*p_cur_1).ap_page[1 as usize] }
                        };
                        { let _ = 0; };
                    }
                } else { break '__b75; }
            } else if unsafe {
                        sqlite3_pager_page_refcount(unsafe { (*p_page).p_db_page })
                    } > 1 {
                rc = unsafe { sqlite3_corrupt_error(9206) };
            } else {
                let p_parent: *mut MemPage =
                    unsafe { (*p_cur_1).ap_page[(i_page - 1) as usize] };
                let i_idx: i32 =
                    unsafe { (*p_cur_1).ai_idx[(i_page - 1) as usize] } as i32;
                rc =
                    unsafe {
                        sqlite3_pager_write(unsafe { (*p_parent).p_db_page })
                    };
                if rc == 0 && unsafe { (*p_parent).n_free } < 0 {
                    rc = btree_compute_free_space(unsafe { &mut *p_parent });
                }
                if rc == 0 {
                    if unsafe { (*p_page).int_key_leaf } != 0 &&
                                        unsafe { (*p_page).n_overflow } as i32 == 1 &&
                                    unsafe { (*p_page).ai_ovfl[0 as usize] } as i32 ==
                                        unsafe { (*p_page).n_cell } as i32 &&
                                unsafe { (*p_parent).pgno } != 1 as u32 &&
                            unsafe { (*p_parent).n_cell } as i32 == i_idx {
                        { let _ = 0; };
                        rc =
                            balance_quick(p_parent, p_page,
                                &raw mut a_balance_quick_space[0 as usize] as *mut u8);
                    } else {
                        let p_space: *mut u8 =
                            unsafe {
                                    sqlite3_page_malloc(unsafe {
                                                (*unsafe { (*p_cur_1).p_bt }).page_size
                                            } as i32)
                                } as *mut u8;
                        rc =
                            balance_nonroot(p_parent, i_idx, p_space,
                                (i_page == 1) as i32,
                                unsafe { (*p_cur_1).hints } as i32 & 1);
                        if !(p_free).is_null() {
                            unsafe { sqlite3_page_free(p_free as *mut ()) };
                        }
                        p_free = p_space;
                    }
                }
                unsafe { (*p_page).n_overflow = 0 as u8 };
                release_page(p_page);
                {
                    let __p = unsafe { &mut (*p_cur_1).i_page };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
                { let _ = 0; };
                unsafe {
                    (*p_cur_1).p_page =
                        unsafe {
                            (*p_cur_1).ap_page[unsafe { (*p_cur_1).i_page } as usize]
                        }
                };
            }
            break '__c75;
        }
        if !(rc == 0) { break '__b75; }
    }
    if !(p_free).is_null() {
        unsafe { sqlite3_page_free(p_free as *mut ()) };
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_delete(p_cur_1: *mut BtCursor, flags: u8)
    -> i32 {
    unsafe {
        let p: *mut Btree = unsafe { (*p_cur_1).p_btree };
        let p_bt: *mut BtShared = unsafe { (*p).p_bt };
        let mut rc: i32 = 0;
        let mut p_page: *mut MemPage = core::ptr::null_mut();
        let mut p_cell: *mut u8 = core::ptr::null_mut();
        let mut i_cell_idx: i32 = 0;
        let mut i_cell_depth: i32 = 0;
        let mut info: CellInfo = unsafe { core::mem::zeroed() };
        let mut b_preserve: u8 = 0 as u8;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_cur_1).e_state } as i32 != 0 {
            if unsafe { (*p_cur_1).e_state } as i32 >= 3 {
                rc = btree_restore_cursor_position(p_cur_1);
                { let _ = 0; };
                if rc != 0 || unsafe { (*p_cur_1).e_state } as i32 != 0 {
                    return rc;
                }
            } else { return unsafe { sqlite3_corrupt_error(9897) }; }
        }
        { let _ = 0; };
        i_cell_depth = unsafe { (*p_cur_1).i_page } as i32;
        i_cell_idx = unsafe { (*p_cur_1).ix } as i32;
        p_page = unsafe { (*p_cur_1).p_page };
        if unsafe { (*p_page).n_cell } as i32 <= i_cell_idx {
            return unsafe { sqlite3_corrupt_error(9906) };
        }
        p_cell =
            unsafe {
                unsafe {
                    (*p_page).a_data.offset((unsafe { (*p_page).mask_page } as
                                    i32 &
                                ((unsafe {
                                                    *unsafe {
                                                            unsafe {
                                                                (*p_page).a_cell_idx.offset((2 * i_cell_idx) as
                                                                            isize).offset(0 as isize)
                                                            }
                                                        }
                                                } as i32) << 8 |
                                    unsafe {
                                            *unsafe {
                                                    unsafe {
                                                        (*p_page).a_cell_idx.offset((2 * i_cell_idx) as
                                                                    isize).offset(1 as isize)
                                                    }
                                                }
                                        } as i32)) as isize)
                }
            };
        if unsafe { (*p_page).n_free } < 0 &&
                btree_compute_free_space(unsafe { &mut *p_page }) != 0 {
            return unsafe { sqlite3_corrupt_error(9910) };
        }
        if p_cell <
                unsafe {
                    unsafe {
                        (*p_page).a_cell_idx.add(unsafe { (*p_page).n_cell } as
                                usize)
                    }
                } {
            return unsafe { sqlite3_corrupt_error(9913) };
        }
        b_preserve = (flags as i32 & 2 != 0) as u8;
        if b_preserve != 0 {
            if (unsafe { (*p_page).leaf } == 0) as i32 != 0 ||
                        unsafe { (*p_page).n_free } +
                                    unsafe {
                                            (unsafe { (*p_page).x_cell_size.unwrap() })(p_page, p_cell)
                                        } as i32 + 2 >
                            (unsafe { (*p_bt).usable_size } * 2 as u32 / 3 as u32) as
                                i32 || unsafe { (*p_page).n_cell } as i32 == 1 {
                rc = save_cursor_key(p_cur_1);
                if rc != 0 { return rc; }
            } else { b_preserve = 2 as u8; }
        }
        if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
            rc = sqlite3_btree_previous(p_cur_1, 0);
            { let _ = 0; };
            if rc != 0 { return rc; }
        }
        if unsafe { (*p_cur_1).cur_flags } as i32 & 32 != 0 {
            rc =
                save_all_cursors(unsafe { &*p_bt },
                    unsafe { (*p_cur_1).pgno_root }, p_cur_1);
            if rc != 0 { return rc; }
        }
        if unsafe { (*p_cur_1).p_key_info } == core::ptr::null_mut() &&
                unsafe { (*p).has_incrblob_cur } != 0 {
            invalidate_incrblob_cursors(unsafe { &mut *p },
                unsafe { (*p_cur_1).pgno_root },
                unsafe { (*p_cur_1).info.n_key }, 0);
        }
        rc = unsafe { sqlite3_pager_write(unsafe { (*p_page).p_db_page }) };
        if rc != 0 { return rc; }
        unsafe {
            (unsafe {
                    (*p_page).x_parse_cell.unwrap()
                })(p_page, p_cell, &mut info)
        };
        if info.n_local as u32 != info.n_payload {
            rc =
                clear_cell_overflow(unsafe { &*p_page }, p_cell as *const u8,
                    &info);
        } else { rc = 0; }
        drop_cell(p_page, i_cell_idx, info.n_size as i32, &mut rc);
        if rc != 0 { return rc; }
        if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
            let p_leaf: *mut MemPage = unsafe { (*p_cur_1).p_page };
            let mut n_cell: i32 = 0;
            let mut n: Pgno = 0 as Pgno;
            let mut p_tmp: *mut u8 = core::ptr::null_mut();
            if unsafe { (*p_leaf).n_free } < 0 {
                rc = btree_compute_free_space(unsafe { &mut *p_leaf });
                if rc != 0 { return rc; }
            }
            if i_cell_depth < unsafe { (*p_cur_1).i_page } as i32 - 1 {
                n =
                    unsafe {
                        (*unsafe {
                                        (*p_cur_1).ap_page[(i_cell_depth + 1) as usize]
                                    }).pgno
                    };
            } else { n = unsafe { (*unsafe { (*p_cur_1).p_page }).pgno }; }
            p_cell =
                unsafe {
                    unsafe {
                        (*p_leaf).a_data.offset((unsafe { (*p_leaf).mask_page } as
                                        i32 &
                                    ((unsafe {
                                                        *unsafe {
                                                                unsafe {
                                                                    (*p_leaf).a_cell_idx.offset((2 *
                                                                                    (unsafe { (*p_leaf).n_cell } as i32 - 1)) as
                                                                                isize).offset(0 as isize)
                                                                }
                                                            }
                                                    } as i32) << 8 |
                                        unsafe {
                                                *unsafe {
                                                        unsafe {
                                                            (*p_leaf).a_cell_idx.offset((2 *
                                                                            (unsafe { (*p_leaf).n_cell } as i32 - 1)) as
                                                                        isize).offset(1 as isize)
                                                        }
                                                    }
                                            } as i32)) as isize)
                    }
                };
            if p_cell <
                    unsafe { unsafe { (*p_leaf).a_data.offset(4 as isize) } } {
                return unsafe { sqlite3_corrupt_error(10004) };
            }
            n_cell =
                unsafe {
                        (unsafe { (*p_leaf).x_cell_size.unwrap() })(p_leaf, p_cell)
                    } as i32;
            { let _ = 0; };
            p_tmp = unsafe { (*p_bt).p_tmp_space };
            { let _ = 0; };
            rc =
                unsafe {
                    sqlite3_pager_write(unsafe { (*p_leaf).p_db_page })
                };
            if rc == 0 {
                rc =
                    insert_cell(p_page, i_cell_idx,
                        unsafe { p_cell.offset(-(4 as isize)) }, n_cell + 4, p_tmp,
                        n);
            }
            drop_cell(p_leaf, unsafe { (*p_leaf).n_cell } as i32 - 1, n_cell,
                &mut rc);
            if rc != 0 { return rc; }
        }
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*unsafe { (*p_cur_1).p_page }).n_free } * 3 <=
                unsafe { (*unsafe { (*p_cur_1).p_bt }).usable_size } as i32 *
                    2 {
            rc = 0;
        } else { rc = balance(p_cur_1); }
        if rc == 0 && unsafe { (*p_cur_1).i_page } as i32 > i_cell_depth {
            release_page_not_null(unsafe { &*unsafe { (*p_cur_1).p_page } });
            {
                let __p = unsafe { &mut (*p_cur_1).i_page };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            while unsafe { (*p_cur_1).i_page } as i32 > i_cell_depth {
                release_page(unsafe {
                        (*p_cur_1).ap_page[{
                                    let __p = unsafe { &mut (*p_cur_1).i_page };
                                    let __t = *__p;
                                    *__p -= 1;
                                    __t
                                } as usize]
                    });
            }
            unsafe {
                (*p_cur_1).p_page =
                    unsafe {
                        (*p_cur_1).ap_page[unsafe { (*p_cur_1).i_page } as usize]
                    }
            };
            rc = balance(p_cur_1);
        }
        if rc == 0 {
            if b_preserve as i32 > 1 {
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                unsafe { (*p_cur_1).e_state = 2 as u8 };
                if i_cell_idx >= unsafe { (*p_page).n_cell } as i32 {
                    unsafe { (*p_cur_1).skip_next = -1 };
                    unsafe {
                        (*p_cur_1).ix =
                            (unsafe { (*p_page).n_cell } as i32 - 1) as u16
                    };
                } else { unsafe { (*p_cur_1).skip_next = 1 }; }
            } else {
                rc = move_to_root(p_cur_1);
                if b_preserve != 0 {
                    btree_release_all_cursor_pages(unsafe { &mut *p_cur_1 });
                    unsafe { (*p_cur_1).e_state = 3 as u8 };
                }
                if rc == 16 { rc = 0; }
            }
        }
        return rc;
    }
}
extern "C" fn btree_overwrite_content(p_page_1: *mut MemPage,
    p_dest_1: *mut u8, p_x_1: *const BtreePayload, i_offset_1: i32,
    mut i_amt_1: i32) -> i32 {
    let n_data: i32 = unsafe { (*p_x_1).n_data } - i_offset_1;
    if n_data <= 0 {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b77: loop {
                if !(i < i_amt_1 &&
                                unsafe { *p_dest_1.offset(i as isize) } as i32 == 0) {
                    break '__b77;
                }
                '__c77: loop { break '__c77; }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if i < i_amt_1 {
            let rc: i32 =
                unsafe {
                    sqlite3_pager_write(unsafe { (*p_page_1).p_db_page })
                };
            if rc != 0 { return rc; }
            unsafe {
                memset(unsafe { p_dest_1.offset(i as isize) } as *mut (), 0,
                    (i_amt_1 - i) as u64)
            };
        }
    } else {
        if n_data < i_amt_1 {
            let rc: i32 =
                btree_overwrite_content(p_page_1,
                    unsafe { p_dest_1.offset(n_data as isize) }, p_x_1,
                    i_offset_1 + n_data, i_amt_1 - n_data);
            if rc != 0 { return rc; }
            i_amt_1 = n_data;
        }
        if unsafe {
                    memcmp(p_dest_1 as *const (),
                        unsafe {
                                (unsafe { (*p_x_1).p_data } as
                                        *mut u8).offset(i_offset_1 as isize)
                            } as *const (), i_amt_1 as u64)
                } != 0 {
            let rc: i32 =
                unsafe {
                    sqlite3_pager_write(unsafe { (*p_page_1).p_db_page })
                };
            if rc != 0 { return rc; }
            unsafe {
                memmove(p_dest_1 as *mut (),
                    unsafe {
                            (unsafe { (*p_x_1).p_data } as
                                    *mut u8).offset(i_offset_1 as isize)
                        } as *const (), i_amt_1 as u64)
            };
        }
    }
    return 0;
}
extern "C" fn btree_overwrite_overflow_cell(p_cur_1: &BtCursor,
    p_x_1: *const BtreePayload) -> i32 {
    let mut i_offset: i32 = 0;
    let n_total: i32 =
        unsafe { (*p_x_1).n_data } + unsafe { (*p_x_1).n_zero } as i32;
    let mut rc: i32 = 0;
    let mut p_page: *mut MemPage = (*p_cur_1).p_page;
    let mut p_bt: *mut BtShared = core::ptr::null_mut();
    let mut ovfl_pgno: Pgno = 0 as Pgno;
    let mut ovfl_page_size: u32 = 0 as u32;
    { let _ = 0; };
    rc =
        btree_overwrite_content(p_page, (*p_cur_1).info.p_payload, p_x_1, 0,
            (*p_cur_1).info.n_local as i32);
    if rc != 0 { return rc; }
    i_offset = (*p_cur_1).info.n_local as i32;
    { let _ = 0; };
    { let _ = 0; };
    ovfl_pgno =
        unsafe {
            sqlite3_get4byte(unsafe {
                        (*p_cur_1).info.p_payload.offset(i_offset as isize)
                    } as *const u8)
        };
    p_bt = unsafe { (*p_page).p_bt };
    ovfl_page_size = unsafe { (*p_bt).usable_size } - 4 as u32;
    '__b78: loop {
        '__c78: loop {
            rc = btree_get_page(p_bt, ovfl_pgno, &mut p_page, 0);
            if rc != 0 { return rc; }
            if unsafe {
                            sqlite3_pager_page_refcount(unsafe { (*p_page).p_db_page })
                        } != 1 || unsafe { (*p_page).is_init } != 0 {
                rc = unsafe { sqlite3_corrupt_error(9370) };
            } else {
                if i_offset as u32 + ovfl_page_size < n_total as u32 {
                    ovfl_pgno =
                        unsafe {
                            sqlite3_get4byte(unsafe { (*p_page).a_data } as *const u8)
                        };
                } else { ovfl_page_size = (n_total - i_offset) as u32; }
                rc =
                    btree_overwrite_content(p_page,
                        unsafe { unsafe { (*p_page).a_data.offset(4 as isize) } },
                        p_x_1, i_offset, ovfl_page_size as i32);
            }
            unsafe { sqlite3_pager_unref(unsafe { (*p_page).p_db_page }) };
            if rc != 0 { return rc; }
            i_offset += ovfl_page_size as i32;
            break '__c78;
        }
        if !(i_offset < n_total) { break '__b78; }
    }
    return 0;
}
extern "C" fn btree_overwrite_cell(p_cur_1: *mut BtCursor,
    p_x_1: *const BtreePayload) -> i32 {
    let n_total: i32 =
        unsafe { (*p_x_1).n_data } + unsafe { (*p_x_1).n_zero } as i32;
    let p_page: *mut MemPage = unsafe { (*p_cur_1).p_page };
    if unsafe {
                    unsafe {
                        (*p_cur_1).info.p_payload.add(unsafe {
                                    (*p_cur_1).info.n_local
                                } as usize)
                    }
                } > unsafe { (*p_page).a_data_end } ||
            unsafe { (*p_cur_1).info.p_payload } <
                unsafe {
                    unsafe {
                        (*p_page).a_data.add(unsafe { (*p_page).cell_offset } as
                                usize)
                    }
                } {
        return unsafe { sqlite3_corrupt_error(9398) };
    }
    if unsafe { (*p_cur_1).info.n_local } as i32 == n_total {
        return btree_overwrite_content(p_page,
                unsafe { (*p_cur_1).info.p_payload }, p_x_1, 0,
                unsafe { (*p_cur_1).info.n_local } as i32);
    } else {
        return btree_overwrite_overflow_cell(unsafe { &*p_cur_1 }, p_x_1);
    }
}
extern "C" fn fill_in_cell(p_page_1: &MemPage, p_cell_1: *mut u8,
    p_x_1: &BtreePayload, pn_size_1: &mut i32) -> i32 {
    unsafe {
        let mut n_payload: i32 = 0;
        let mut p_src: *const u8 = core::ptr::null();
        let mut n_src: i32 = 0;
        let mut n: i32 = 0;
        let mut rc: i32 = 0;
        let mut mn: i32 = 0;
        let mut space_left: i32 = 0;
        let mut p_to_release: *mut MemPage = core::ptr::null_mut();
        let mut p_prior: *mut u8 = core::ptr::null_mut();
        let mut p_payload: *mut u8 = core::ptr::null_mut();
        let mut p_bt: *mut BtShared = core::ptr::null_mut();
        let mut pgno_ovfl: Pgno = 0 as Pgno;
        let mut n_header: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        n_header = (*p_page_1).child_ptr_size as i32;
        if (*p_page_1).int_key != 0 {
            n_payload = (*p_x_1).n_data + (*p_x_1).n_zero as i32;
            p_src = (*p_x_1).p_data as *const u8;
            n_src = (*p_x_1).n_data as i32;
            { let _ = 0; };
            n_header +=
                if (n_payload as u32) < 128 as u32 {
                            {
                                ({
                                        let __v = n_payload as u8;
                                        unsafe {
                                            *unsafe { &mut *p_cell_1.offset(n_header as isize) } = __v
                                        };
                                        __v
                                    }) as i32;
                                1
                            }
                        } else {
                            unsafe {
                                sqlite3_put_varint(unsafe {
                                        &mut *p_cell_1.offset(n_header as isize)
                                    }, n_payload as u64)
                            }
                        } as u8 as i32;
            n_header +=
                unsafe {
                    sqlite3_put_varint(unsafe {
                            &mut *p_cell_1.offset(n_header as isize)
                        }, unsafe { *(&raw const (*p_x_1).n_key as *mut u64) })
                };
        } else {
            { let _ = 0; };
            n_src = { n_payload = (*p_x_1).n_key as i32; n_payload };
            p_src = (*p_x_1).p_key as *const u8;
            n_header +=
                if (n_payload as u32) < 128 as u32 {
                            {
                                ({
                                        let __v = n_payload as u8;
                                        unsafe {
                                            *unsafe { &mut *p_cell_1.offset(n_header as isize) } = __v
                                        };
                                        __v
                                    }) as i32;
                                1
                            }
                        } else {
                            unsafe {
                                sqlite3_put_varint(unsafe {
                                        &mut *p_cell_1.offset(n_header as isize)
                                    }, n_payload as u64)
                            }
                        } as u8 as i32;
        }
        p_payload = unsafe { p_cell_1.offset(n_header as isize) };
        if n_payload <= (*p_page_1).max_local as i32 {
            n = n_header + n_payload;
            if n < 4 {
                n = 4;
                unsafe { *p_payload.offset(n_payload as isize) = 0 as u8 };
            }
            *pn_size_1 = n;
            { let _ = 0; };
            unsafe {
                memcpy(p_payload as *mut (), p_src as *const (), n_src as u64)
            };
            unsafe {
                memset(unsafe { p_payload.offset(n_src as isize) } as *mut (),
                    0, (n_payload - n_src) as u64)
            };
            return 0;
        }
        mn = (*p_page_1).min_local as i32;
        n =
            (mn as u32 +
                    (n_payload - mn) as u32 %
                        (unsafe { (*(*p_page_1).p_bt).usable_size } - 4 as u32)) as
                i32;
        if n > (*p_page_1).max_local as i32 { n = mn; }
        space_left = n;
        *pn_size_1 = n + n_header + 4;
        p_prior = unsafe { p_cell_1.offset((n_header + n) as isize) };
        p_to_release = core::ptr::null_mut();
        pgno_ovfl = 0 as Pgno;
        p_bt = (*p_page_1).p_bt;
        loop {
            n = n_payload;
            if n > space_left { n = space_left; }
            { let _ = 0; };
            { let _ = 0; };
            if n_src >= n {
                unsafe {
                    memcpy(p_payload as *mut (), p_src as *const (), n as u64)
                };
            } else if n_src > 0 {
                n = n_src;
                unsafe {
                    memcpy(p_payload as *mut (), p_src as *const (), n as u64)
                };
            } else { unsafe { memset(p_payload as *mut (), 0, n as u64) }; }
            n_payload -= n;
            if n_payload <= 0 { break; }
            {
                let __n = n;
                let __p = &mut p_payload;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            {
                let __n = n;
                let __p = &mut p_src;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            n_src -= n;
            space_left -= n;
            if space_left == 0 {
                let mut p_ovfl: *mut MemPage = core::ptr::null_mut();
                let pgno_ptrmap: Pgno = pgno_ovfl;
                if unsafe { (*p_bt).auto_vacuum } != 0 {
                    '__b80: loop {
                        '__c80: loop {
                            {
                                let __p = &mut pgno_ovfl;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            break '__c80;
                        }
                        if !(ptrmap_pageno(unsafe { &*p_bt }, pgno_ovfl) ==
                                            pgno_ovfl ||
                                        pgno_ovfl ==
                                            (sqlite3_pending_byte as u32 / unsafe { (*p_bt).page_size }
                                                    + 1 as u32) as Pgno) {
                            break '__b80;
                        }
                    }
                }
                rc =
                    allocate_btree_page(p_bt, &mut p_ovfl, &mut pgno_ovfl,
                        pgno_ovfl, 0 as u8);
                if unsafe { (*p_bt).auto_vacuum } != 0 && rc == 0 {
                    let e_type: u8 = if pgno_ptrmap != 0 { 4 } else { 3 } as u8;
                    ptrmap_put(p_bt, pgno_ovfl, e_type, pgno_ptrmap, &mut rc);
                    if rc != 0 { release_page(p_ovfl); }
                }
                if rc != 0 { release_page(p_to_release); return rc; }
                { let _ = 0; };
                { let _ = 0; };
                unsafe { sqlite3_put4byte(p_prior, pgno_ovfl) };
                release_page(p_to_release);
                p_to_release = p_ovfl;
                p_prior = unsafe { (*p_ovfl).a_data };
                unsafe { sqlite3_put4byte(p_prior, 0 as u32) };
                p_payload =
                    unsafe { unsafe { (*p_ovfl).a_data.offset(4 as isize) } };
                space_left =
                    (unsafe { (*p_bt).usable_size } - 4 as u32) as i32;
            }
        }
        release_page(p_to_release);
        return 0;
    }
}
extern "C" fn insert_cell_fast(p_page_1: *mut MemPage, i: i32,
    p_cell_1: *mut u8, sz: i32) -> i32 {
    let mut idx: i32 = 0;
    let mut j: i32 = 0;
    let mut data: *mut u8 = core::ptr::null_mut();
    let mut p_ins: *mut u8 = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if sz + 2 > unsafe { (*p_page_1).n_free } {
        j =
            {
                    let __p = unsafe { &mut (*p_page_1).n_overflow };
                    let __t = *__p;
                    *__p += 1;
                    __t
                } as i32;
        { let _ = 0; };
        unsafe { (*p_page_1).ap_ovfl[j as usize] = p_cell_1 };
        unsafe { (*p_page_1).ai_ovfl[j as usize] = i as u16 };
        { let _ = 0; };
        { let _ = 0; };
    } else {
        let mut rc: i32 =
            unsafe { sqlite3_pager_write(unsafe { (*p_page_1).p_db_page }) };
        if rc != 0 { return rc; }
        { let _ = 0; };
        data = unsafe { (*p_page_1).a_data };
        { let _ = 0; };
        rc = allocate_space(p_page_1, sz, &mut idx);
        if rc != 0 { return rc; }
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_page_1).n_free -= (2 + sz) as u16 as i32 };
        unsafe {
            memcpy(unsafe { &raw mut *data.offset(idx as isize) } as *mut (),
                p_cell_1 as *const (), sz as u64)
        };
        p_ins =
            unsafe {
                unsafe { (*p_page_1).a_cell_idx.offset((i * 2) as isize) }
            };
        unsafe {
            memmove(unsafe { p_ins.offset(2 as isize) } as *mut (),
                p_ins as *const (),
                (2 * (unsafe { (*p_page_1).n_cell } as i32 - i)) as u64)
        };
        {
            unsafe { *p_ins.offset(0 as isize) = (idx >> 8) as u8 };
            unsafe { *p_ins.offset(1 as isize) = idx as u8 }
        };
        {
            let __p = unsafe { &mut (*p_page_1).n_cell };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if {
                        let __p =
                            unsafe {
                                &mut *data.offset((unsafe { (*p_page_1).hdr_offset } as i32
                                                    + 4) as isize)
                            };
                        *__p += 1;
                        *__p
                    } as i32 == 0 {
            {
                let __p =
                    unsafe {
                        &mut *data.offset((unsafe { (*p_page_1).hdr_offset } as i32
                                            + 3) as isize)
                    };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        { let _ = 0; };
        if unsafe { (*unsafe { (*p_page_1).p_bt }).auto_vacuum } != 0 {
            let mut rc2: i32 = 0;
            ptrmap_put_ovfl_ptr(p_page_1, unsafe { &*p_page_1 }, p_cell_1,
                &mut rc2);
            if rc2 != 0 { return rc2; }
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_insert(p_cur_1: *mut BtCursor,
    p_x_1: *const BtreePayload, flags: i32, seek_result_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        '__b81: loop {
            '__c81: loop {
                let mut loc: i32 = seek_result_1;
                let mut sz_new: i32 = 0;
                let mut idx: i32 = 0;
                let mut p_page: *mut MemPage = core::ptr::null_mut();
                let p: *mut Btree = unsafe { (*p_cur_1).p_btree };
                let mut old_cell: *mut u8 = core::ptr::null_mut();
                let mut new_cell: *mut u8 = core::ptr::null_mut();
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p_cur_1).cur_flags } as i32 & 32 != 0 {
                    rc =
                        save_all_cursors(unsafe { &*unsafe { (*p).p_bt } },
                            unsafe { (*p_cur_1).pgno_root }, p_cur_1);
                    if rc != 0 { return rc; }
                    if loc != 0 && (unsafe { (*p_cur_1).i_page } as i32) < 0 {
                        return unsafe { sqlite3_corrupt_error(9479) };
                    }
                }
                if unsafe { (*p_cur_1).e_state } as i32 >= 3 {
                    rc = move_to_root(p_cur_1);
                    if rc != 0 && rc != 16 { return rc; }
                }
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p_cur_1).p_key_info } == core::ptr::null_mut() {
                    { let _ = 0; };
                    if unsafe { (*p).has_incrblob_cur } != 0 {
                        invalidate_incrblob_cursors(unsafe { &mut *p },
                            unsafe { (*p_cur_1).pgno_root }, unsafe { (*p_x_1).n_key },
                            0);
                    }
                    if unsafe { (*p_cur_1).cur_flags } as i32 & 2 != 0 &&
                            unsafe { (*p_x_1).n_key } as sqlite3_int64 ==
                                unsafe { (*p_cur_1).info.n_key } {
                        { let _ = 0; };
                        if unsafe { (*p_cur_1).info.n_size } as i32 != 0 &&
                                unsafe { (*p_cur_1).info.n_payload } ==
                                    unsafe { (*p_x_1).n_data } as u32 +
                                        unsafe { (*p_x_1).n_zero } as u32 {
                            return btree_overwrite_cell(p_cur_1, p_x_1);
                        }
                        { let _ = 0; };
                    } else if loc == 0 {
                        rc =
                            sqlite3_btree_table_moveto(p_cur_1,
                                unsafe { (*p_x_1).n_key }, (flags & 8 != 0) as i32,
                                &mut loc);
                        if rc != 0 { return rc; }
                    }
                } else {
                    { let _ = 0; };
                    if loc == 0 && flags & 2 == 0 {
                        if unsafe { (*p_x_1).n_mem } != 0 {
                            let mut r: UnpackedRecord = unsafe { core::mem::zeroed() };
                            r.p_key_info = unsafe { (*p_cur_1).p_key_info };
                            r.a_mem = unsafe { (*p_x_1).a_mem } as *mut Mem;
                            r.n_field = unsafe { (*p_x_1).n_mem } as u16;
                            r.default_rc = 0 as i8;
                            r.eq_seen = 0 as u8;
                            rc = sqlite3_btree_index_moveto(p_cur_1, &mut r, &mut loc);
                        } else {
                            rc =
                                btree_moveto(p_cur_1, unsafe { (*p_x_1).p_key },
                                    unsafe { (*p_x_1).n_key }, (flags & 8 != 0) as i32,
                                    &mut loc);
                        }
                        if rc != 0 { return rc; }
                    }
                    if loc == 0 {
                        get_cell_info(unsafe { &mut *p_cur_1 });
                        if unsafe { (*p_cur_1).info.n_key } ==
                                unsafe { (*p_x_1).n_key } {
                            let mut x2: BtreePayload = unsafe { core::mem::zeroed() };
                            x2.p_data = unsafe { (*p_x_1).p_key };
                            x2.n_data = unsafe { (*p_x_1).n_key } as i32;
                            { let _ = 0; };
                            x2.n_zero = 0;
                            return btree_overwrite_cell(p_cur_1,
                                    &raw mut x2 as *const BtreePayload);
                        }
                    }
                }
                { let _ = 0; };
                p_page = unsafe { (*p_cur_1).p_page };
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p_page).n_free } < 0 {
                    if unsafe { (*p_cur_1).e_state } as i32 > 1 {
                        rc = unsafe { sqlite3_corrupt_error(9602) };
                    } else {
                        rc = btree_compute_free_space(unsafe { &mut *p_page });
                    }
                    if rc != 0 { return rc; }
                }
                { let _ = 0; };
                new_cell = unsafe { (*unsafe { (*p).p_bt }).p_tmp_space };
                { let _ = 0; };
                { let _ = 0; };
                if flags & 128 != 0 {
                    rc = 0;
                    sz_new =
                        unsafe { (*unsafe { (*p).p_bt }).n_preformat_size };
                    if sz_new < 4 {
                        sz_new = 4;
                        unsafe { *new_cell.offset(3 as isize) = 0 as u8 };
                    }
                    if unsafe { (*unsafe { (*p).p_bt }).auto_vacuum } != 0 &&
                            sz_new > unsafe { (*p_page).max_local } as i32 {
                        let mut info: CellInfo = unsafe { core::mem::zeroed() };
                        unsafe {
                            (unsafe {
                                    (*p_page).x_parse_cell.unwrap()
                                })(p_page, new_cell, &mut info)
                        };
                        if info.n_payload != info.n_local as u32 {
                            let ovfl: Pgno =
                                unsafe {
                                    sqlite3_get4byte(unsafe {
                                                &raw mut *new_cell.offset((sz_new - 4) as isize)
                                            } as *const u8)
                                };
                            ptrmap_put(unsafe { (*p).p_bt }, ovfl, 3 as u8,
                                unsafe { (*p_page).pgno }, &mut rc);
                            if rc != 0 { break '__b81; }
                        }
                    }
                } else {
                    rc =
                        fill_in_cell(unsafe { &*p_page }, new_cell,
                            unsafe { &*p_x_1 }, &mut sz_new);
                    if rc != 0 { break '__b81; }
                }
                { let _ = 0; };
                { let _ = 0; };
                idx = unsafe { (*p_cur_1).ix } as i32;
                unsafe { (*p_cur_1).info.n_size = 0 as u16 };
                if loc == 0 {
                    let mut info: CellInfo = unsafe { core::mem::zeroed() };
                    { let _ = 0; };
                    if idx >= unsafe { (*p_page).n_cell } as i32 {
                        return unsafe { sqlite3_corrupt_error(9644) };
                    }
                    rc =
                        unsafe {
                            sqlite3_pager_write(unsafe { (*p_page).p_db_page })
                        };
                    if rc != 0 { break '__b81; }
                    old_cell =
                        unsafe {
                            unsafe {
                                (*p_page).a_data.offset((unsafe { (*p_page).mask_page } as
                                                i32 &
                                            ((unsafe {
                                                                *unsafe {
                                                                        unsafe {
                                                                            (*p_page).a_cell_idx.offset((2 * idx) as
                                                                                        isize).offset(0 as isize)
                                                                        }
                                                                    }
                                                            } as i32) << 8 |
                                                unsafe {
                                                        *unsafe {
                                                                unsafe {
                                                                    (*p_page).a_cell_idx.offset((2 * idx) as
                                                                                isize).offset(1 as isize)
                                                                }
                                                            }
                                                    } as i32)) as isize)
                            }
                        };
                    if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
                        unsafe {
                            memcpy(new_cell as *mut (), old_cell as *const (), 4 as u64)
                        };
                    }
                    unsafe {
                        (unsafe {
                                (*p_page).x_parse_cell.unwrap()
                            })(p_page, old_cell, &mut info)
                    };
                    if info.n_local as u32 != info.n_payload {
                        rc =
                            clear_cell_overflow(unsafe { &*p_page },
                                old_cell as *const u8, &info);
                    } else { rc = 0; }
                    unsafe { (*p_cur_1).cur_flags &= !4 as u8 };
                    if info.n_size as i32 == sz_new &&
                                info.n_local as u32 == info.n_payload &&
                            ((unsafe { (*unsafe { (*p).p_bt }).auto_vacuum } == 0) as
                                        i32 != 0 || sz_new < unsafe { (*p_page).min_local } as i32)
                        {
                        { let _ = 0; };
                        if old_cell <
                                unsafe {
                                    unsafe {
                                        unsafe {
                                            (*p_page).a_data.add(unsafe { (*p_page).hdr_offset } as
                                                        usize).offset(10 as isize)
                                        }
                                    }
                                } {
                            return unsafe { sqlite3_corrupt_error(9671) };
                        }
                        if unsafe { old_cell.offset(sz_new as isize) } >
                                unsafe { (*p_page).a_data_end } {
                            return unsafe { sqlite3_corrupt_error(9674) };
                        }
                        unsafe {
                            memcpy(old_cell as *mut (), new_cell as *const (),
                                sz_new as u64)
                        };
                        return 0;
                    }
                    drop_cell(p_page, idx, info.n_size as i32, &mut rc);
                    if rc != 0 { break '__b81; }
                } else if loc < 0 && unsafe { (*p_page).n_cell } as i32 > 0 {
                    { let _ = 0; };
                    idx =
                        { let __p = unsafe { &mut (*p_cur_1).ix }; *__p += 1; *__p }
                            as i32;
                    unsafe { (*p_cur_1).cur_flags &= !(2 | 4) as u8 };
                } else { { let _ = 0; }; }
                rc = insert_cell_fast(p_page, idx, new_cell, sz_new);
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p_page).n_overflow } != 0 {
                    { let _ = 0; };
                    unsafe { (*p_cur_1).cur_flags &= !(2 | 4) as u8 };
                    rc = balance(p_cur_1);
                    unsafe {
                        (*unsafe { (*p_cur_1).p_page }).n_overflow = 0 as u8
                    };
                    unsafe { (*p_cur_1).e_state = 1 as u8 };
                    if flags & 2 != 0 && rc == 0 {
                        btree_release_all_cursor_pages(unsafe { &mut *p_cur_1 });
                        if !(unsafe { (*p_cur_1).p_key_info }).is_null() {
                            { let _ = 0; };
                            unsafe {
                                (*p_cur_1).p_key =
                                    unsafe { sqlite3Malloc(unsafe { (*p_x_1).n_key } as u64) }
                            };
                            if unsafe { (*p_cur_1).p_key } == core::ptr::null_mut() {
                                rc = 7;
                            } else {
                                unsafe {
                                    memcpy(unsafe { (*p_cur_1).p_key },
                                        unsafe { (*p_x_1).p_key }, unsafe { (*p_x_1).n_key } as u64)
                                };
                            }
                        }
                        unsafe { (*p_cur_1).e_state = 3 as u8 };
                        unsafe {
                            (*p_cur_1).n_key = unsafe { (*p_x_1).n_key } as i64
                        };
                    }
                }
                { let _ = 0; };
                break '__c81;
            }
            if !(false) { break '__b81; }
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_first(p_cur_1: *mut BtCursor,
    p_res_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    rc = move_to_root(p_cur_1);
    if rc == 0 {
        { let _ = 0; };
        *p_res_1 = 0;
        rc = move_to_leftmost(p_cur_1);
    } else if rc == 16 { { let _ = 0; }; *p_res_1 = 1; rc = 0; }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_is_empty(p_cur_1: *mut BtCursor,
    p_res_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_cur_1).e_state } as i32 == 0 { *p_res_1 = 0; return 0; }
    rc = move_to_root(p_cur_1);
    if rc == 16 { *p_res_1 = 1; rc = 0; } else { *p_res_1 = 0; }
    return rc;
}
extern "C" fn btree_last(p_cur_1: *mut BtCursor, p_res_1: &mut i32) -> i32 {
    let mut rc: i32 = move_to_root(p_cur_1);
    if rc == 0 {
        { let _ = 0; };
        *p_res_1 = 0;
        rc = move_to_rightmost(p_cur_1);
        if rc == 0 {
            unsafe { (*p_cur_1).cur_flags |= 8 as u8 };
        } else { unsafe { (*p_cur_1).cur_flags &= !8 as u8 }; }
    } else if rc == 16 { { let _ = 0; }; *p_res_1 = 1; rc = 0; }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_last(p_cur_1: *mut BtCursor,
    p_res_1: *mut i32) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    if 0 == unsafe { (*p_cur_1).e_state } as i32 &&
            unsafe { (*p_cur_1).cur_flags } as i32 & 8 != 0 {
        { let _ = 0; };
        unsafe { *p_res_1 = 0 };
        return 0;
    }
    return btree_last(p_cur_1, unsafe { &mut *p_res_1 });
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_eof(p_cur_1: &BtCursor) -> i32 {
    return (0 != (*p_cur_1).e_state as i32) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_pin(p_cur_1: &mut BtCursor) -> () {
    { let _ = 0; };
    (*p_cur_1).cur_flags |= 64 as u8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_unpin(p_cur_1: &mut BtCursor) -> () {
    { let _ = 0; };
    (*p_cur_1).cur_flags &= !64 as u8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_offset(p_cur_1: *mut BtCursor) -> i64 {
    { let _ = 0; };
    { let _ = 0; };
    get_cell_info(unsafe { &mut *p_cur_1 });
    return unsafe { (*unsafe { (*p_cur_1).p_bt }).page_size } as i64 *
                (unsafe { (*unsafe { (*p_cur_1).p_page }).pgno } as i64 -
                    1 as i64) +
            unsafe {
                        unsafe {
                            (*p_cur_1).info.p_payload.offset_from(unsafe {
                                    (*unsafe { (*p_cur_1).p_page }).a_data
                                })
                        }
                    } as i64 as i64;
}
extern "C" fn fetch_payload(p_cur_1: &mut BtCursor, p_amt_1: &mut u32)
    -> *const () {
    let mut amt: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    amt = (*p_cur_1).info.n_local as i32;
    if amt >
            unsafe {
                        unsafe {
                            (*(*p_cur_1).p_page).a_data_end.offset_from((*p_cur_1).info.p_payload)
                        }
                    } as i64 as i32 {
        { let _ = 0; };
        amt =
            if 0 >
                    unsafe {
                                unsafe {
                                    (*(*p_cur_1).p_page).a_data_end.offset_from((*p_cur_1).info.p_payload)
                                }
                            } as i64 as i32 {
                0
            } else {
                (unsafe {
                            unsafe {
                                (*(*p_cur_1).p_page).a_data_end.offset_from((*p_cur_1).info.p_payload)
                            }
                        }) as i64 as i32
            };
    }
    *p_amt_1 = amt as u32;
    return (*p_cur_1).info.p_payload as *mut () as *const ();
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_payload_fetch(p_cur_1: *mut BtCursor,
    p_amt_1: *mut u32) -> *const () {
    return fetch_payload(unsafe { &mut *p_cur_1 }, unsafe { &mut *p_amt_1 });
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_max_record_size(p_cur_1: &BtCursor)
    -> sqlite3_int64 {
    { let _ = 0; };
    { let _ = 0; };
    return unsafe { (*(*p_cur_1).p_bt).page_size } as sqlite3_int64 *
            unsafe { (*(*p_cur_1).p_bt).n_page } as sqlite3_int64;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IntegrityCk {
    p_bt: *mut BtShared,
    p_pager: *mut Pager,
    a_pg_ref: *mut u8,
    n_ck_page: Pgno,
    mx_err: i32,
    n_err: i32,
    rc: i32,
    n_step: u32,
    z_pfx: *const i8,
    v0: Pgno,
    v1: Pgno,
    v2: i32,
    err_msg: StrAccum,
    heap: *mut u32,
    db: *mut sqlite3,
    n_row: i64,
}
extern "C" fn check_oom(p_check_1: &mut IntegrityCk) -> () {
    (*p_check_1).rc = 7;
    (*p_check_1).mx_err = 0;
    if (*p_check_1).n_err == 0 {
        { let __p = &mut (*p_check_1).n_err; let __t = *__p; *__p += 1; __t };
    }
}
extern "C" fn set_page_referenced(p_check_1: &IntegrityCk, i_pg_1: Pgno)
    -> () {
    { let _ = 0; };
    { let _ = 0; };
    unsafe {
        *(*p_check_1).a_pg_ref.add((i_pg_1 / 8 as Pgno) as usize) |=
            (1 << (i_pg_1 & 7 as Pgno)) as u8
    };
}
extern "C" fn check_progress(p_check_1: &mut IntegrityCk) -> () {
    unsafe {
        let db: *mut sqlite3 = (*p_check_1).db;
        if unsafe {
                    std::sync::atomic::AtomicI32::from_ptr(unsafe {
                                    &raw mut (*db).u1.is_interrupted
                                } as *mut i32).load(std::sync::atomic::Ordering::Relaxed)
                } != 0 {
            (*p_check_1).rc = 9;
            {
                let __p = &mut (*p_check_1).n_err;
                let __t = *__p;
                *__p += 1;
                __t
            };
            (*p_check_1).mx_err = 0;
        }
        if unsafe { (*db).x_progress.is_some() } {
            { let _ = 0; };
            {
                let __p = &mut (*p_check_1).n_step;
                let __t = *__p;
                *__p += 1;
                __t
            };
            if (*p_check_1).n_step % unsafe { (*db).n_progress_ops } ==
                        0 as u32 &&
                    unsafe {
                            (unsafe {
                                    (*db).x_progress.unwrap()
                                })(unsafe { (*db).p_progress_arg })
                        } != 0 {
                (*p_check_1).rc = 9;
                {
                    let __p = &mut (*p_check_1).n_err;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                (*p_check_1).mx_err = 0;
            }
        }
    }
}
unsafe extern "C" fn check_append_msg(p_check_1: *mut IntegrityCk,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    check_progress(unsafe { &mut *p_check_1 });
    if (unsafe { (*p_check_1).mx_err } == 0) as i32 != 0 { return; }
    {
        let __p = unsafe { &mut (*p_check_1).mx_err };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    {
        let __p = unsafe { &mut (*p_check_1).n_err };
        let __t = *__p;
        *__p += 1;
        __t
    };
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    if unsafe { (*p_check_1).err_msg.n_char } != 0 {
        unsafe {
            sqlite3_str_append(unsafe { &raw mut (*p_check_1).err_msg } as
                    *mut sqlite3_str, c"\n".as_ptr() as *mut i8 as *const i8, 1)
        };
    }
    if !(unsafe { (*p_check_1).z_pfx }).is_null() {
        unsafe {
            sqlite3_str_appendf(unsafe { &raw mut (*p_check_1).err_msg } as
                    *mut sqlite3_str, unsafe { (*p_check_1).z_pfx },
                unsafe { (*p_check_1).v0 }, unsafe { (*p_check_1).v1 },
                unsafe { (*p_check_1).v2 })
        };
    }
    unsafe {
        sqlite3_str_vappendf(unsafe { &raw mut (*p_check_1).err_msg } as
                *mut sqlite3_str, z_format_1, ap)
    };
    ();
    if unsafe { (*p_check_1).err_msg.acc_error } as i32 == 7 {
        check_oom(unsafe { &mut *p_check_1 });
    }
}
extern "C" fn get_page_referenced(p_check_1: &IntegrityCk, i_pg_1: Pgno)
    -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    return unsafe {
                    *(*p_check_1).a_pg_ref.add((i_pg_1 / 8 as Pgno) as usize)
                } as i32 & 1 << (i_pg_1 & 7 as Pgno);
}
extern "C" fn check_ref(p_check_1: *mut IntegrityCk, i_page_1: Pgno) -> i32 {
    if i_page_1 > unsafe { (*p_check_1).n_ck_page } || i_page_1 == 0 as u32 {
        unsafe {
            check_append_msg(p_check_1,
                c"invalid page number %u".as_ptr() as *mut i8 as *const i8,
                i_page_1)
        };
        return 1;
    }
    if get_page_referenced(unsafe { &*p_check_1 }, i_page_1) != 0 {
        unsafe {
            check_append_msg(p_check_1,
                c"2nd reference to page %u".as_ptr() as *mut i8 as *const i8,
                i_page_1)
        };
        return 1;
    }
    set_page_referenced(unsafe { &*p_check_1 }, i_page_1);
    return 0;
}
extern "C" fn check_ptrmap(p_check_1: *mut IntegrityCk, i_child_1: Pgno,
    e_type_1: u8, i_parent_1: Pgno) -> () {
    let mut rc: i32 = 0;
    let mut e_ptrmap_type: u8 = 0 as u8;
    let mut i_ptrmap_parent: Pgno = 0 as Pgno;
    rc =
        ptrmap_get(unsafe { (*p_check_1).p_bt }, i_child_1,
            &mut e_ptrmap_type, &mut i_ptrmap_parent);
    if rc != 0 {
        if rc == 7 || rc == 10 | 12 << 8 {
            check_oom(unsafe { &mut *p_check_1 });
        }
        unsafe {
            check_append_msg(p_check_1,
                c"Failed to read ptrmap key=%u".as_ptr() as *mut i8 as
                    *const i8, i_child_1)
        };
        return;
    }
    if e_ptrmap_type as i32 != e_type_1 as i32 ||
            i_ptrmap_parent != i_parent_1 {
        unsafe {
            check_append_msg(p_check_1,
                c"Bad ptr map entry key=%u expected=(%u,%u) got=(%u,%u)".as_ptr()
                        as *mut i8 as *const i8, i_child_1, e_type_1 as i32,
                i_parent_1, e_ptrmap_type as i32, i_ptrmap_parent)
        };
    }
}
extern "C" fn check_list(p_check_1: *mut IntegrityCk, is_free_list_1: i32,
    mut i_page_1: Pgno, mut n_1: u32) -> () {
    let mut i: i32 = 0;
    let expected: u32 = n_1;
    let n_err_at_start: i32 = unsafe { (*p_check_1).n_err };
    while i_page_1 != 0 as u32 && unsafe { (*p_check_1).mx_err } != 0 {
        let mut p_ovfl_page: *mut DbPage = core::ptr::null_mut();
        let mut p_ovfl_data: *mut u8 = core::ptr::null_mut();
        if check_ref(p_check_1, i_page_1) != 0 { break; }
        { let __p = &mut n_1; let __t = *__p; *__p -= 1; __t };
        if unsafe {
                    sqlite3_pager_get(unsafe { (*p_check_1).p_pager },
                        i_page_1 as Pgno, &mut p_ovfl_page, 0)
                } != 0 {
            unsafe {
                check_append_msg(p_check_1,
                    c"failed to get page %u".as_ptr() as *mut i8 as *const i8,
                    i_page_1)
            };
            break;
        }
        p_ovfl_data =
            unsafe { sqlite3_pager_get_data(p_ovfl_page) } as *mut u8;
        if is_free_list_1 != 0 {
            let n: u32 =
                unsafe {
                        sqlite3_get4byte(unsafe {
                                    &raw mut *p_ovfl_data.offset(4 as isize)
                                } as *const u8)
                    } as u32;
            if unsafe { (*unsafe { (*p_check_1).p_bt }).auto_vacuum } != 0 {
                check_ptrmap(p_check_1, i_page_1, 2 as u8, 0 as Pgno);
            }
            if n >
                    unsafe { (*unsafe { (*p_check_1).p_bt }).usable_size } /
                            4 as u32 - 2 as u32 {
                unsafe {
                    check_append_msg(p_check_1,
                        c"freelist leaf count too big on page %u".as_ptr() as
                                *mut i8 as *const i8, i_page_1)
                };
                { let __p = &mut n_1; let __t = *__p; *__p -= 1; __t };
            } else {
                {
                    i = 0;
                    '__b83: loop {
                        if !(i < n as i32) { break '__b83; }
                        '__c83: loop {
                            let i_free_page: Pgno =
                                unsafe {
                                    sqlite3_get4byte(unsafe {
                                                &raw mut *p_ovfl_data.offset((8 + i * 4) as isize)
                                            } as *const u8)
                                };
                            if unsafe { (*unsafe { (*p_check_1).p_bt }).auto_vacuum } !=
                                    0 {
                                check_ptrmap(p_check_1, i_free_page, 2 as u8, 0 as Pgno);
                            }
                            check_ref(p_check_1, i_free_page);
                            break '__c83;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                n_1 -= n;
            }
        } else {
            if unsafe { (*unsafe { (*p_check_1).p_bt }).auto_vacuum } != 0 &&
                    n_1 > 0 as u32 {
                i =
                    unsafe { sqlite3_get4byte(p_ovfl_data as *const u8) } as
                        i32;
                check_ptrmap(p_check_1, i as Pgno, 4 as u8, i_page_1);
            }
        }
        i_page_1 = unsafe { sqlite3_get4byte(p_ovfl_data as *const u8) };
        unsafe { sqlite3_pager_unref(p_ovfl_page) };
    }
    if n_1 != 0 && n_err_at_start == unsafe { (*p_check_1).n_err } {
        unsafe {
            check_append_msg(p_check_1,
                c"%s is %u but should be %u".as_ptr() as *mut i8 as *const i8,
                if is_free_list_1 != 0 {
                    c"size".as_ptr() as *mut i8
                } else { c"overflow list length".as_ptr() as *mut i8 },
                expected - n_1, expected)
        };
    }
}
extern "C" fn btree_heap_insert(a_heap_1: *mut u32, mut x: u32) -> () {
    let mut j: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    { let _ = 0; };
    i =
        {
            let __p = unsafe { &mut *a_heap_1.offset(0 as isize) };
            *__p += 1;
            *__p
        };
    unsafe { *a_heap_1.add(i as usize) = x };
    while { j = i / 2 as u32; j } > 0 as u32 &&
            unsafe { *a_heap_1.add(j as usize) } >
                unsafe { *a_heap_1.add(i as usize) } {
        x = unsafe { *a_heap_1.add(j as usize) };
        unsafe {
            *a_heap_1.add(j as usize) = unsafe { *a_heap_1.add(i as usize) }
        };
        unsafe { *a_heap_1.add(i as usize) = x };
        i = j;
    }
}
extern "C" fn btree_heap_pull(a_heap_1: *mut u32, p_out_1: &mut u32) -> i32 {
    let mut j: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    let mut x: u32 = 0 as u32;
    if { x = unsafe { *a_heap_1.offset(0 as isize) }; x } == 0 as u32 {
        return 0;
    }
    *p_out_1 = unsafe { *a_heap_1.offset(1 as isize) };
    unsafe {
        *a_heap_1.offset(1 as isize) = unsafe { *a_heap_1.add(x as usize) }
    };
    unsafe { *a_heap_1.add(x as usize) = 4294967295u32 };
    {
        let __p = unsafe { &mut *a_heap_1.offset(0 as isize) };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    i = 1 as u32;
    while { j = i * 2 as u32; j } <= unsafe { *a_heap_1.offset(0 as isize) } {
        if unsafe { *a_heap_1.add(j as usize) } >
                unsafe { *a_heap_1.add((j + 1 as u32) as usize) } {
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
        if unsafe { *a_heap_1.add(i as usize) } <
                unsafe { *a_heap_1.add(j as usize) } {
            break;
        }
        x = unsafe { *a_heap_1.add(i as usize) };
        unsafe {
            *a_heap_1.add(i as usize) = unsafe { *a_heap_1.add(j as usize) }
        };
        unsafe { *a_heap_1.add(j as usize) = x };
        i = j;
    }
    return 1;
}
extern "C" fn check_tree_page(p_check_1: *mut IntegrityCk, i_page_1: Pgno,
    pi_min_key_1: *mut i64, mut max_key_1: i64) -> i32 {
    unsafe {
        let mut p_page: *mut MemPage = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        let mut depth: i32 = 0;
        let mut d2: i32 = 0;
        let mut pgno: i32 = 0;
        let mut n_frag: i32 = 0;
        let mut hdr: i32 = 0;
        let mut cell_start: i32 = 0;
        let mut n_cell: i32 = 0;
        let mut do_coverage_check: i32 = 0;
        let mut key_can_be_equal: i32 = 0;
        let mut data: *mut u8 = core::ptr::null_mut();
        let mut p_cell: *mut u8 = core::ptr::null_mut();
        let mut p_cell_idx: *const u8 = core::ptr::null();
        let mut p_bt: *mut BtShared = core::ptr::null_mut();
        let mut pc: u32 = 0 as u32;
        let mut usable_size: u32 = 0 as u32;
        let mut content_offset: u32 = 0 as u32;
        let mut heap: *mut u32 = core::ptr::null_mut();
        let mut x: u32 = 0 as u32;
        let mut prev: u32 = 0 as u32;
        let mut saved_z_pfx: *const i8 = core::ptr::null();
        let mut saved_v1: i32 = 0;
        let mut saved_v2: i32 = 0;
        let mut saved_is_init: u8 = 0 as u8;
        let mut info: CellInfo = unsafe { core::mem::zeroed() };
        let mut n_page: u32 = 0 as u32;
        let mut pgno_ovfl: Pgno = 0 as Pgno;
        let mut size: u32 = 0 as u32;
        let mut size__1: i32 = 0;
        let mut j: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s87:
                {
                match __state {
                    0 => { p_page = core::ptr::null_mut(); __state = 3; }
                    2 => {
                        if (do_coverage_check == 0) as i32 != 0 {
                            __state = 160;
                        } else { __state = 159; }
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { depth = -1; __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { do_coverage_check = 1; __state = 12; }
                    12 => { key_can_be_equal = 1; __state = 13; }
                    13 => { __state = 14; }
                    14 => { __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => { __state = 18; }
                    18 => { __state = 19; }
                    19 => { __state = 20; }
                    20 => { heap = core::ptr::null_mut(); __state = 21; }
                    21 => { prev = 0 as u32; __state = 22; }
                    22 => {
                        saved_z_pfx = unsafe { (*p_check_1).z_pfx };
                        __state = 23;
                    }
                    23 => {
                        saved_v1 = unsafe { (*p_check_1).v1 } as i32;
                        __state = 24;
                    }
                    24 => {
                        saved_v2 = unsafe { (*p_check_1).v2 };
                        __state = 25;
                    }
                    25 => { saved_is_init = 0 as u8; __state = 26; }
                    26 => {
                        check_progress(unsafe { &mut *p_check_1 });
                        __state = 27;
                    }
                    27 => {
                        if unsafe { (*p_check_1).mx_err } == 0 {
                            __state = 29;
                        } else { __state = 28; }
                    }
                    28 => { p_bt = unsafe { (*p_check_1).p_bt }; __state = 30; }
                    29 => { __state = 2; }
                    30 => {
                        usable_size = unsafe { (*p_bt).usable_size };
                        __state = 31;
                    }
                    31 => {
                        if i_page_1 == 0 as u32 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        if check_ref(p_check_1, i_page_1) != 0 {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    33 => { return 0; }
                    34 => {
                        unsafe {
                            (*p_check_1).z_pfx =
                                c"Tree %u page %u: ".as_ptr() as *mut i8 as *const i8
                        };
                        __state = 36;
                    }
                    35 => { return 0; }
                    36 => {
                        unsafe { (*p_check_1).v1 = i_page_1 };
                        __state = 37;
                    }
                    37 => {
                        if {
                                    rc = btree_get_page(p_bt, i_page_1, &mut p_page, 0);
                                    rc
                                } != 0 {
                            __state = 39;
                        } else { __state = 38; }
                    }
                    38 => {
                        saved_is_init = unsafe { (*p_page).is_init };
                        __state = 43;
                    }
                    39 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"unable to get the page. error code=%d".as_ptr() as *mut i8
                                    as *const i8, rc)
                        };
                        __state = 40;
                    }
                    40 => {
                        if rc == 10 | 12 << 8 {
                            __state = 42;
                        } else { __state = 41; }
                    }
                    41 => { __state = 2; }
                    42 => { unsafe { (*p_check_1).rc = 7 }; __state = 41; }
                    43 => {
                        unsafe { (*p_page).is_init = 0 as u8 };
                        __state = 44;
                    }
                    44 => {
                        if { rc = btree_init_page(p_page); rc } != 0 {
                            __state = 46;
                        } else { __state = 45; }
                    }
                    45 => {
                        if {
                                    rc = btree_compute_free_space(unsafe { &mut *p_page });
                                    rc
                                } != 0 {
                            __state = 50;
                        } else { __state = 49; }
                    }
                    46 => { { let _ = 0; }; __state = 47; }
                    47 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"btreeInitPage() returns error code %d".as_ptr() as *mut i8
                                    as *const i8, rc)
                        };
                        __state = 48;
                    }
                    48 => { __state = 2; }
                    49 => { data = unsafe { (*p_page).a_data }; __state = 53; }
                    50 => { { let _ = 0; }; __state = 51; }
                    51 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"free space corruption".as_ptr() as *mut i8 as *const i8,
                                rc)
                        };
                        __state = 52;
                    }
                    52 => { __state = 2; }
                    53 => {
                        hdr = unsafe { (*p_page).hdr_offset } as i32;
                        __state = 54;
                    }
                    54 => {
                        unsafe {
                            (*p_check_1).z_pfx =
                                c"Tree %u page %u cell %u: ".as_ptr() as *mut i8 as
                                    *const i8
                        };
                        __state = 55;
                    }
                    55 => {
                        content_offset =
                            ((((unsafe {
                                                                    *unsafe {
                                                                            data.offset((hdr + 5) as isize).offset(0 as isize)
                                                                        }
                                                                } as i32) << 8 |
                                                    unsafe {
                                                            *unsafe {
                                                                    data.offset((hdr + 5) as isize).offset(1 as isize)
                                                                }
                                                        } as i32) as i32 - 1 & 65535) + 1) as u32;
                        __state = 56;
                    }
                    56 => { { let _ = 0; }; __state = 57; }
                    57 => {
                        n_cell =
                            (unsafe {
                                                *unsafe {
                                                        data.offset((hdr + 3) as isize).offset(0 as isize)
                                                    }
                                            } as i32) << 8 |
                                unsafe {
                                        *unsafe {
                                                data.offset((hdr + 3) as isize).offset(1 as isize)
                                            }
                                    } as i32;
                        __state = 58;
                    }
                    58 => { { let _ = 0; }; __state = 59; }
                    59 => {
                        if unsafe { (*p_page).leaf } != 0 ||
                                unsafe { (*p_page).int_key } as i32 == 0 {
                            __state = 61;
                        } else { __state = 60; }
                    }
                    60 => {
                        cell_start =
                            hdr + 12 - 4 * unsafe { (*p_page).leaf } as i32;
                        __state = 62;
                    }
                    61 => {
                        unsafe { (*p_check_1).n_row += n_cell as i64 };
                        __state = 60;
                    }
                    62 => { { let _ = 0; }; __state = 63; }
                    63 => {
                        p_cell_idx =
                            unsafe {
                                data.offset((cell_start + 2 * (n_cell - 1)) as isize)
                            };
                        __state = 64;
                    }
                    64 => {
                        if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
                            __state = 66;
                        } else { __state = 67; }
                    }
                    65 => { i = n_cell - 1; __state = 75; }
                    66 => {
                        pgno =
                            unsafe {
                                    sqlite3_get4byte(unsafe {
                                                &raw mut *data.offset((hdr + 8) as isize)
                                            } as *const u8)
                                } as i32;
                        __state = 68;
                    }
                    67 => { heap = unsafe { (*p_check_1).heap }; __state = 73; }
                    68 => {
                        if unsafe { (*p_bt).auto_vacuum } != 0 {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    69 => {
                        depth =
                            check_tree_page(p_check_1, pgno as Pgno, &mut max_key_1,
                                max_key_1);
                        __state = 72;
                    }
                    70 => {
                        unsafe {
                            (*p_check_1).z_pfx =
                                c"Tree %u page %u right child: ".as_ptr() as *mut i8 as
                                    *const i8
                        };
                        __state = 71;
                    }
                    71 => {
                        check_ptrmap(p_check_1, pgno as Pgno, 5 as u8, i_page_1);
                        __state = 69;
                    }
                    72 => { key_can_be_equal = 0; __state = 65; }
                    73 => {
                        unsafe { *heap.offset(0 as isize) = 0 as u32 };
                        __state = 65;
                    }
                    74 => {
                        unsafe { *pi_min_key_1 = max_key_1 };
                        __state = 121;
                    }
                    75 => {
                        if i >= 0 && unsafe { (*p_check_1).mx_err } != 0 {
                            __state = 76;
                        } else { __state = 74; }
                    }
                    76 => { __state = 78; }
                    77 => {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        __state = 75;
                    }
                    78 => { unsafe { (*p_check_1).v2 = i }; __state = 79; }
                    79 => { { let _ = 0; }; __state = 80; }
                    80 => {
                        pc =
                            ((unsafe { *p_cell_idx.offset(0 as isize) } as i32) << 8 |
                                    unsafe { *p_cell_idx.offset(1 as isize) } as i32) as u32;
                        __state = 81;
                    }
                    81 => {
                        {
                            let __n = 2;
                            let __p = &mut p_cell_idx;
                            *__p = unsafe { (*__p).offset(-(__n as isize)) };
                        };
                        __state = 82;
                    }
                    82 => {
                        if pc < content_offset || pc > usable_size - 4 as u32 {
                            __state = 84;
                        } else { __state = 83; }
                    }
                    83 => {
                        p_cell = unsafe { data.add(pc as usize) };
                        __state = 87;
                    }
                    84 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"Offset %u out of range %u..%u".as_ptr() as *mut i8 as
                                    *const i8, pc, content_offset, usable_size - 4 as u32)
                        };
                        __state = 85;
                    }
                    85 => { do_coverage_check = 0; __state = 86; }
                    86 => { __state = 77; }
                    87 => {
                        unsafe {
                            (unsafe {
                                    (*p_page).x_parse_cell.unwrap()
                                })(p_page, p_cell, &mut info)
                        };
                        __state = 88;
                    }
                    88 => {
                        if pc + info.n_size as u32 > usable_size {
                            __state = 90;
                        } else { __state = 89; }
                    }
                    89 => {
                        if info.n_payload != 0 &&
                                (unsafe { *info.p_payload.offset(0 as isize) } as i32) < 2 {
                            __state = 94;
                        } else { __state = 93; }
                    }
                    90 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"Extends off end of page".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 91;
                    }
                    91 => { do_coverage_check = 0; __state = 92; }
                    92 => { __state = 77; }
                    93 => {
                        if unsafe { (*p_page).int_key } != 0 {
                            __state = 98;
                        } else { __state = 97; }
                    }
                    94 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"Bad cell header size".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 95;
                    }
                    95 => { do_coverage_check = 0; __state = 96; }
                    96 => { __state = 77; }
                    97 => {
                        if info.n_payload > info.n_local as u32 {
                            __state = 103;
                        } else { __state = 102; }
                    }
                    98 => {
                        if if key_can_be_equal != 0 {
                                    (info.n_key > max_key_1) as i32
                                } else { (info.n_key >= max_key_1) as i32 } != 0 {
                            __state = 100;
                        } else { __state = 99; }
                    }
                    99 => { max_key_1 = info.n_key; __state = 101; }
                    100 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"Rowid %lld out of order".as_ptr() as *mut i8 as *const i8,
                                info.n_key)
                        };
                        __state = 99;
                    }
                    101 => { key_can_be_equal = 0; __state = 97; }
                    102 => {
                        if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
                            __state = 111;
                        } else { __state = 112; }
                    }
                    103 => { __state = 104; }
                    104 => { __state = 105; }
                    105 => { { let _ = 0; }; __state = 106; }
                    106 => {
                        n_page =
                            (info.n_payload - info.n_local as u32 + usable_size -
                                    5 as u32) / (usable_size - 4 as u32);
                        __state = 107;
                    }
                    107 => {
                        pgno_ovfl =
                            unsafe {
                                sqlite3_get4byte(unsafe {
                                            &raw mut *p_cell.offset((info.n_size as i32 - 4) as isize)
                                        } as *const u8)
                            };
                        __state = 108;
                    }
                    108 => {
                        if unsafe { (*p_bt).auto_vacuum } != 0 {
                            __state = 110;
                        } else { __state = 109; }
                    }
                    109 => {
                        check_list(p_check_1, 0, pgno_ovfl, n_page);
                        __state = 102;
                    }
                    110 => {
                        check_ptrmap(p_check_1, pgno_ovfl, 3 as u8, i_page_1);
                        __state = 109;
                    }
                    111 => {
                        pgno =
                            unsafe { sqlite3_get4byte(p_cell as *const u8) } as i32;
                        __state = 113;
                    }
                    112 => { { let _ = 0; }; __state = 120; }
                    113 => {
                        if unsafe { (*p_bt).auto_vacuum } != 0 {
                            __state = 115;
                        } else { __state = 114; }
                    }
                    114 => {
                        d2 =
                            check_tree_page(p_check_1, pgno as Pgno, &mut max_key_1,
                                max_key_1);
                        __state = 116;
                    }
                    115 => {
                        check_ptrmap(p_check_1, pgno as Pgno, 5 as u8, i_page_1);
                        __state = 114;
                    }
                    116 => { key_can_be_equal = 0; __state = 117; }
                    117 => {
                        if d2 != depth { __state = 118; } else { __state = 77; }
                    }
                    118 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"Child page depth differs".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 119;
                    }
                    119 => { depth = d2; __state = 77; }
                    120 => {
                        btree_heap_insert(heap,
                            pc << 16 | pc + info.n_size as u32 - 1 as u32);
                        __state = 77;
                    }
                    121 => {
                        unsafe { (*p_check_1).z_pfx = core::ptr::null() };
                        __state = 122;
                    }
                    122 => {
                        if do_coverage_check != 0 &&
                                unsafe { (*p_check_1).mx_err } > 0 {
                            __state = 124;
                        } else { __state = 123; }
                    }
                    123 => { __state = 2; }
                    124 => {
                        if (unsafe { (*p_page).leaf } == 0) as i32 != 0 {
                            __state = 126;
                        } else { __state = 125; }
                    }
                    125 => { { let _ = 0; }; __state = 136; }
                    126 => {
                        heap = unsafe { (*p_check_1).heap };
                        __state = 127;
                    }
                    127 => {
                        unsafe { *heap.offset(0 as isize) = 0 as u32 };
                        __state = 128;
                    }
                    128 => { i = n_cell - 1; __state = 129; }
                    129 => {
                        if i >= 0 { __state = 130; } else { __state = 125; }
                    }
                    130 => { __state = 132; }
                    131 => {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        __state = 129;
                    }
                    132 => {
                        pc =
                            ((unsafe {
                                                    *unsafe {
                                                            data.offset((cell_start + i * 2) as
                                                                        isize).offset(0 as isize)
                                                        }
                                                } as i32) << 8 |
                                    unsafe {
                                            *unsafe {
                                                    data.offset((cell_start + i * 2) as
                                                                isize).offset(1 as isize)
                                                }
                                        } as i32) as u32;
                        __state = 133;
                    }
                    133 => {
                        size =
                            unsafe {
                                    (unsafe {
                                            (*p_page).x_cell_size.unwrap()
                                        })(p_page, unsafe { &mut *data.add(pc as usize) })
                                } as u32;
                        __state = 134;
                    }
                    134 => { { let _ = 0; }; __state = 135; }
                    135 => {
                        btree_heap_insert(heap, pc << 16 | pc + size - 1 as u32);
                        __state = 131;
                    }
                    136 => {
                        i =
                            (unsafe {
                                                *unsafe {
                                                        data.offset((hdr + 1) as isize).offset(0 as isize)
                                                    }
                                            } as i32) << 8 |
                                unsafe {
                                        *unsafe {
                                                data.offset((hdr + 1) as isize).offset(1 as isize)
                                            }
                                    } as i32;
                        __state = 137;
                    }
                    137 => {
                        if i > 0 { __state = 139; } else { __state = 138; }
                    }
                    138 => { n_frag = 0; __state = 149; }
                    139 => { __state = 140; }
                    140 => { { let _ = 0; }; __state = 141; }
                    141 => {
                        size__1 =
                            (unsafe {
                                                *unsafe { data.offset((i + 2) as isize).offset(0 as isize) }
                                            } as i32) << 8 |
                                unsafe {
                                        *unsafe { data.offset((i + 2) as isize).offset(1 as isize) }
                                    } as i32;
                        __state = 142;
                    }
                    142 => { { let _ = 0; }; __state = 143; }
                    143 => { { let _ = 0; }; __state = 144; }
                    144 => {
                        btree_heap_insert(heap,
                            (i as u32) << 16 | (i + size__1 - 1) as u32);
                        __state = 145;
                    }
                    145 => {
                        j =
                            (unsafe {
                                                *unsafe { data.offset(i as isize).offset(0 as isize) }
                                            } as i32) << 8 |
                                unsafe {
                                        *unsafe { data.offset(i as isize).offset(1 as isize) }
                                    } as i32;
                        __state = 146;
                    }
                    146 => { { let _ = 0; }; __state = 147; }
                    147 => { { let _ = 0; }; __state = 148; }
                    148 => { i = j; __state = 137; }
                    149 => { prev = content_offset - 1 as u32; __state = 150; }
                    150 => {
                        if btree_heap_pull(heap, &mut x) != 0 {
                            __state = 152;
                        } else { __state = 151; }
                    }
                    151 => {
                        n_frag +=
                            (usable_size - (prev & 65535 as u32) - 1 as u32) as i32;
                        __state = 157;
                    }
                    152 => {
                        if prev & 65535 as u32 >= x >> 16 {
                            __state = 153;
                        } else { __state = 154; }
                    }
                    153 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"Multiple uses for byte %u of page %u".as_ptr() as *mut i8
                                    as *const i8, x >> 16, i_page_1)
                        };
                        __state = 155;
                    }
                    154 => {
                        n_frag +=
                            ((x >> 16) - (prev & 65535 as u32) - 1 as u32) as i32;
                        __state = 156;
                    }
                    155 => { __state = 151; }
                    156 => { prev = x; __state = 150; }
                    157 => {
                        if unsafe { *heap.offset(0 as isize) } == 0 as u32 &&
                                n_frag != unsafe { *data.offset((hdr + 7) as isize) } as i32
                            {
                            __state = 158;
                        } else { __state = 123; }
                    }
                    158 => {
                        unsafe {
                            check_append_msg(p_check_1,
                                c"Fragmentation of %u bytes reported as %u on page %u".as_ptr()
                                        as *mut i8 as *const i8, n_frag,
                                unsafe { *data.offset((hdr + 7) as isize) } as i32,
                                i_page_1)
                        };
                        __state = 123;
                    }
                    159 => { release_page(p_page); __state = 161; }
                    160 => {
                        unsafe { (*p_page).is_init = saved_is_init };
                        __state = 159;
                    }
                    161 => {
                        unsafe { (*p_check_1).z_pfx = saved_z_pfx };
                        __state = 162;
                    }
                    162 => {
                        unsafe { (*p_check_1).v1 = saved_v1 as Pgno };
                        __state = 163;
                    }
                    163 => {
                        unsafe { (*p_check_1).v2 = saved_v2 };
                        __state = 164;
                    }
                    164 => { return depth + 1; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_integrity_check(db: *mut sqlite3,
    p: *mut Btree, a_root_1: *const Pgno, a_cnt_1: *mut Mem, n_root_1: i32,
    mx_err_1: i32, pn_err_1: &mut i32, pz_out_1: &mut *mut i8) -> i32 {
    unsafe {
        unsafe {
            let mut i: Pgno = 0 as Pgno;
            let mut s_check: IntegrityCk = unsafe { core::mem::zeroed() };
            let mut p_bt: *mut BtShared = core::ptr::null_mut();
            let mut saved_db_flags: u64 = 0 as u64;
            let mut z_err: [i8; 100] = [0; 100];
            let mut b_partial: i32 = 0;
            let mut b_ck_freelist: i32 = 0;
            let mut mx: Pgno = 0 as Pgno;
            let mut mx_in_hdr: Pgno = 0 as Pgno;
            let mut not_used: i64 = 0 as i64;
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s89:
                    {
                    match __state {
                        0 => { __state = 3; }
                        2 => {
                            unsafe { sqlite3_page_free(s_check.heap as *mut ()) };
                            __state = 83;
                        }
                        3 => { __state = 4; }
                        4 => { p_bt = unsafe { (*p).p_bt }; __state = 5; }
                        5 => {
                            saved_db_flags = unsafe { (*unsafe { (*p_bt).db }).flags };
                            __state = 6;
                        }
                        6 => { __state = 7; }
                        7 => { b_partial = 0; __state = 8; }
                        8 => { b_ck_freelist = 1; __state = 9; }
                        9 => { __state = 10; }
                        10 => { { let _ = 0; }; __state = 11; }
                        11 => { { let _ = 0; }; __state = 12; }
                        12 => {
                            if unsafe { *a_root_1.offset(0 as isize) } == 0 as u32 {
                                __state = 14;
                            } else { __state = 13; }
                        }
                        13 => { unsafe { sqlite3_btree_enter(p) }; __state = 18; }
                        14 => { { let _ = 0; }; __state = 15; }
                        15 => { b_partial = 1; __state = 16; }
                        16 => {
                            if unsafe { *a_root_1.offset(1 as isize) } != 1 as u32 {
                                __state = 17;
                            } else { __state = 13; }
                        }
                        17 => { b_ck_freelist = 0; __state = 13; }
                        18 => { { let _ = 0; }; __state = 19; }
                        19 => { __state = 20; }
                        20 => { { let _ = 0; }; __state = 21; }
                        21 => {
                            unsafe {
                                memset(&raw mut s_check as *mut (), 0,
                                    core::mem::size_of::<IntegrityCk>() as u64)
                            };
                            __state = 22;
                        }
                        22 => { s_check.db = db; __state = 23; }
                        23 => { s_check.p_bt = p_bt; __state = 24; }
                        24 => {
                            s_check.p_pager = unsafe { (*p_bt).p_pager };
                            __state = 25;
                        }
                        25 => {
                            s_check.n_ck_page =
                                btree_pagecount(unsafe { &*s_check.p_bt });
                            __state = 26;
                        }
                        26 => { s_check.mx_err = mx_err_1; __state = 27; }
                        27 => {
                            unsafe {
                                sqlite3_str_accum_init(&mut s_check.err_msg,
                                    core::ptr::null_mut(),
                                    &raw mut z_err[0 as usize] as *mut i8,
                                    core::mem::size_of::<[i8; 100]>() as i32, 1000000000)
                            };
                            __state = 28;
                        }
                        28 => {
                            s_check.err_msg.printf_flags = 1 as u8;
                            __state = 29;
                        }
                        29 => {
                            if s_check.n_ck_page == 0 as u32 {
                                __state = 31;
                            } else { __state = 30; }
                        }
                        30 => {
                            s_check.a_pg_ref =
                                unsafe {
                                        sqlite3_malloc_zero((s_check.n_ck_page / 8 as Pgno +
                                                    1 as Pgno) as u64)
                                    } as *mut u8;
                            __state = 32;
                        }
                        31 => { __state = 2; }
                        32 => {
                            if (s_check.a_pg_ref).is_null() as i32 != 0 {
                                __state = 34;
                            } else { __state = 33; }
                        }
                        33 => {
                            s_check.heap =
                                unsafe {
                                        sqlite3_page_malloc(unsafe { (*p_bt).page_size } as i32)
                                    } as *mut u32;
                            __state = 36;
                        }
                        34 => { check_oom(&mut s_check); __state = 35; }
                        35 => { __state = 2; }
                        36 => {
                            if s_check.heap == core::ptr::null_mut() {
                                __state = 38;
                            } else { __state = 37; }
                        }
                        37 => {
                            i =
                                (sqlite3_pending_byte as u32 / unsafe { (*p_bt).page_size }
                                        + 1 as u32) as Pgno;
                            __state = 40;
                        }
                        38 => { check_oom(&mut s_check); __state = 39; }
                        39 => { __state = 2; }
                        40 => {
                            if i <= s_check.n_ck_page {
                                __state = 42;
                            } else { __state = 41; }
                        }
                        41 => {
                            if b_ck_freelist != 0 {
                                __state = 44;
                            } else { __state = 43; }
                        }
                        42 => { set_page_referenced(&s_check, i); __state = 41; }
                        43 => {
                            if (b_partial == 0) as i32 != 0 {
                                __state = 48;
                            } else { __state = 47; }
                        }
                        44 => {
                            s_check.z_pfx =
                                c"Freelist: ".as_ptr() as *mut i8 as *const i8;
                            __state = 45;
                        }
                        45 => {
                            check_list(&mut s_check, 1,
                                unsafe {
                                    sqlite3_get4byte(unsafe {
                                                &raw mut *unsafe {
                                                            (*unsafe { (*p_bt).p_page1 }).a_data.offset(32 as isize)
                                                        }
                                            } as *const u8)
                                },
                                unsafe {
                                    sqlite3_get4byte(unsafe {
                                                &raw mut *unsafe {
                                                            (*unsafe { (*p_bt).p_page1 }).a_data.offset(36 as isize)
                                                        }
                                            } as *const u8)
                                });
                            __state = 46;
                        }
                        46 => { s_check.z_pfx = core::ptr::null(); __state = 43; }
                        47 => { __state = 61; }
                        48 => {
                            if unsafe { (*p_bt).auto_vacuum } != 0 {
                                __state = 49;
                            } else { __state = 50; }
                        }
                        49 => { mx = 0 as Pgno; __state = 51; }
                        50 => {
                            if unsafe {
                                        sqlite3_get4byte(unsafe {
                                                    &raw mut *unsafe {
                                                                (*unsafe { (*p_bt).p_page1 }).a_data.offset(64 as isize)
                                                            }
                                                } as *const u8)
                                    } != 0 as u32 {
                                __state = 60;
                            } else { __state = 47; }
                        }
                        51 => { __state = 52; }
                        52 => { i = 0 as Pgno; __state = 54; }
                        53 => {
                            mx_in_hdr =
                                unsafe {
                                    sqlite3_get4byte(unsafe {
                                                &raw mut *unsafe {
                                                            (*unsafe { (*p_bt).p_page1 }).a_data.offset(52 as isize)
                                                        }
                                            } as *const u8)
                                };
                            __state = 58;
                        }
                        54 => {
                            if (i as i32) < n_root_1 {
                                __state = 55;
                            } else { __state = 53; }
                        }
                        55 => {
                            if mx < unsafe { *a_root_1.add(i as usize) } {
                                __state = 57;
                            } else { __state = 56; }
                        }
                        56 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 54;
                        }
                        57 => {
                            mx = unsafe { *a_root_1.add(i as usize) };
                            __state = 56;
                        }
                        58 => {
                            if mx != mx_in_hdr { __state = 59; } else { __state = 47; }
                        }
                        59 => {
                            unsafe {
                                check_append_msg(&mut s_check,
                                    c"max rootpage (%u) disagrees with header (%u)".as_ptr() as
                                            *mut i8 as *const i8, mx, mx_in_hdr)
                            };
                            __state = 47;
                        }
                        60 => {
                            unsafe {
                                check_append_msg(&mut s_check,
                                    c"incremental_vacuum enabled with a max rootpage of zero".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                            __state = 47;
                        }
                        61 => {
                            unsafe {
                                (*unsafe { (*p_bt).db }).flags &= !(2097152 as u64)
                            };
                            __state = 62;
                        }
                        62 => { i = 0 as Pgno; __state = 64; }
                        63 => {
                            unsafe { (*unsafe { (*p_bt).db }).flags = saved_db_flags };
                            __state = 74;
                        }
                        64 => {
                            if (i as i32) < n_root_1 && s_check.mx_err != 0 {
                                __state = 65;
                            } else { __state = 63; }
                        }
                        65 => { s_check.n_row = 0 as i64; __state = 67; }
                        66 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 64;
                        }
                        67 => {
                            if unsafe { *a_root_1.add(i as usize) } != 0 {
                                __state = 69;
                            } else { __state = 68; }
                        }
                        68 => {
                            unsafe {
                                sqlite3_mem_set_array_int64(a_cnt_1 as *mut sqlite3_value,
                                    i as i32, s_check.n_row)
                            };
                            __state = 66;
                        }
                        69 => { __state = 70; }
                        70 => {
                            if unsafe { (*p_bt).auto_vacuum } != 0 &&
                                        unsafe { *a_root_1.add(i as usize) } > 1 as u32 &&
                                    (b_partial == 0) as i32 != 0 {
                                __state = 72;
                            } else { __state = 71; }
                        }
                        71 => {
                            s_check.v0 = unsafe { *a_root_1.add(i as usize) };
                            __state = 73;
                        }
                        72 => {
                            check_ptrmap(&mut s_check,
                                unsafe { *a_root_1.add(i as usize) }, 1 as u8, 0 as Pgno);
                            __state = 71;
                        }
                        73 => {
                            check_tree_page(&mut s_check,
                                unsafe { *a_root_1.add(i as usize) }, &mut not_used,
                                4294967295u32 as i64 | (2147483647 as i64) << 32);
                            __state = 68;
                        }
                        74 => {
                            if (b_partial == 0) as i32 != 0 {
                                __state = 76;
                            } else { __state = 75; }
                        }
                        75 => { __state = 2; }
                        76 => { i = 1 as Pgno; __state = 77; }
                        77 => {
                            if i <= s_check.n_ck_page && s_check.mx_err != 0 {
                                __state = 78;
                            } else { __state = 75; }
                        }
                        78 => {
                            if get_page_referenced(&s_check, i) == 0 &&
                                    (ptrmap_pageno(unsafe { &*p_bt }, i) != i ||
                                        (unsafe { (*p_bt).auto_vacuum } == 0) as i32 != 0) {
                                __state = 81;
                            } else { __state = 80; }
                        }
                        79 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 77;
                        }
                        80 => {
                            if get_page_referenced(&s_check, i) != 0 &&
                                    (ptrmap_pageno(unsafe { &*p_bt }, i) == i &&
                                        unsafe { (*p_bt).auto_vacuum } != 0) {
                                __state = 82;
                            } else { __state = 79; }
                        }
                        81 => {
                            unsafe {
                                check_append_msg(&mut s_check,
                                    c"Page %u: never used".as_ptr() as *mut i8 as *const i8, i)
                            };
                            __state = 80;
                        }
                        82 => {
                            unsafe {
                                check_append_msg(&mut s_check,
                                    c"Page %u: pointer map referenced".as_ptr() as *mut i8 as
                                        *const i8, i)
                            };
                            __state = 79;
                        }
                        83 => {
                            unsafe { sqlite3_free(s_check.a_pg_ref as *mut ()) };
                            __state = 84;
                        }
                        84 => { *pn_err_1 = s_check.n_err; __state = 85; }
                        85 => {
                            if s_check.n_err == 0 {
                                __state = 87;
                            } else { __state = 88; }
                        }
                        86 => { { let _ = 0; }; __state = 90; }
                        87 => {
                            unsafe {
                                sqlite3_str_reset(&raw mut s_check.err_msg as
                                        *mut sqlite3_str)
                            };
                            __state = 89;
                        }
                        88 => {
                            *pz_out_1 =
                                unsafe { sqlite3_str_accum_finish(&mut s_check.err_msg) };
                            __state = 86;
                        }
                        89 => { *pz_out_1 = core::ptr::null_mut(); __state = 86; }
                        90 => { unsafe { sqlite3_btree_leave(p) }; __state = 91; }
                        91 => { return s_check.rc; }
                        _ => {}
                    }
                }
            }
            unreachable!();
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_pager(p: &Btree) -> *mut Pager {
    return unsafe { (*(*p).p_bt).p_pager };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_row_count_est(p_cur_1: &BtCursor) -> i64 {
    let mut n: i64 = 0 as i64;
    let mut i: u8 = 0 as u8;
    { let _ = 0; };
    { let _ = 0; };
    if (*p_cur_1).e_state as i32 != 0 { return 0 as i64; }
    if unsafe { (*(*p_cur_1).p_page).leaf } as i32 == 0 { return -1 as i64; }
    n = unsafe { (*(*p_cur_1).p_page).n_cell } as i64;
    {
        i = 0 as u8;
        '__b90: loop {
            if !((i as i32) < (*p_cur_1).i_page as i32) { break '__b90; }
            '__c90: loop {
                n *=
                    (unsafe { (*(*p_cur_1).ap_page[i as usize]).n_cell } as i32
                            + 1) as i64;
                break '__c90;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return n;
}
extern "C" fn access_payload_checked(p_cur_1: *mut BtCursor, offset: u32,
    amt: u32, p_buf_1: *mut ()) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*p_cur_1).e_state } as i32 == 1 { return 4; }
    { let _ = 0; };
    rc = btree_restore_cursor_position(p_cur_1);
    return if rc != 0 {
            rc
        } else {
            access_payload(p_cur_1, offset, amt, p_buf_1 as *mut u8, 0)
        };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_payload_checked(p_cur_1: *mut BtCursor,
    offset: u32, amt: u32, p_buf_1: *mut ()) -> i32 {
    if unsafe { (*p_cur_1).e_state } as i32 == 0 {
        { let _ = 0; };
        return access_payload(p_cur_1, offset, amt, p_buf_1 as *mut u8, 0);
    } else { return access_payload_checked(p_cur_1, offset, amt, p_buf_1); }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_put_data(p_csr_1: *mut BtCursor, offset: u32,
    amt: u32, z: *mut ()) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    rc =
        if unsafe { (*p_csr_1).e_state } as i32 >= 3 {
            btree_restore_cursor_position(p_csr_1)
        } else { 0 };
    if rc != 0 { return rc; }
    { let _ = 0; };
    if unsafe { (*p_csr_1).e_state } as i32 != 0 { return 4; }
    save_all_cursors(unsafe { &*unsafe { (*p_csr_1).p_bt } },
        unsafe { (*p_csr_1).pgno_root }, p_csr_1);
    { let _ = 0; };
    if unsafe { (*p_csr_1).cur_flags } as i32 & 1 == 0 { return 8; }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    return access_payload(p_csr_1, offset, amt, z as *mut u8, 1);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_incrblob_cursor(p_cur_1: &mut BtCursor)
    -> () {
    (*p_cur_1).cur_flags |= 16 as u8;
    unsafe { (*(*p_cur_1).p_btree).has_incrblob_cur = 1 as u8 };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_set_version(p_btree_1: *mut Btree,
    i_version_1: i32) -> i32 {
    let p_bt: *mut BtShared = unsafe { (*p_btree_1).p_bt };
    let mut rc: i32 = 0;
    { let _ = 0; };
    unsafe { (*p_bt).bts_flags &= !32 as u16 };
    if i_version_1 == 1 { unsafe { (*p_bt).bts_flags |= 32 as u16 }; }
    rc = sqlite3_btree_begin_trans(p_btree_1, 0, core::ptr::null_mut());
    if rc == 0 {
        let a_data: *mut u8 = unsafe { (*unsafe { (*p_bt).p_page1 }).a_data };
        if unsafe { *a_data.offset(18 as isize) } as i32 !=
                    i_version_1 as u8 as i32 ||
                unsafe { *a_data.offset(19 as isize) } as i32 !=
                    i_version_1 as u8 as i32 {
            rc =
                sqlite3_btree_begin_trans(p_btree_1, 2,
                    core::ptr::null_mut());
            if rc == 0 {
                rc =
                    unsafe {
                        sqlite3_pager_write(unsafe {
                                (*unsafe { (*p_bt).p_page1 }).p_db_page
                            })
                    };
                if rc == 0 {
                    unsafe { *a_data.offset(18 as isize) = i_version_1 as u8 };
                    unsafe { *a_data.offset(19 as isize) = i_version_1 as u8 };
                }
            }
        }
    }
    unsafe { (*p_bt).bts_flags &= !32 as u16 };
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_has_hint(p_csr_1: &BtCursor, mask: u32)
    -> i32 {
    return ((*p_csr_1).hints as u32 & mask != 0 as u32) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_is_readonly(p: &Btree) -> i32 {
    return (unsafe { (*(*p).p_bt).bts_flags } as i32 & 1 != 0) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_header_size_btree() -> i32 {
    return (core::mem::size_of::<MemPage>() as u64 + 7 as u64 & !7 as u64) as
            i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_cursor_is_valid_nn(p_cur_1: &BtCursor)
    -> i32 {
    { let _ = 0; };
    return ((*p_cur_1).e_state as i32 == 0) as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_count(db: &mut sqlite3,
    p_cur_1: *mut BtCursor, pn_entry_1: &mut i64) -> i32 {
    unsafe {
        let mut n_entry: i64 = 0 as i64;
        let mut rc: i32 = 0;
        rc = move_to_root(p_cur_1);
        if rc == 16 { *pn_entry_1 = 0 as i64; return 0; }
        while rc == 0 &&
                (unsafe {
                                std::sync::atomic::AtomicI32::from_ptr(&raw mut (*db).u1.is_interrupted
                                            as *mut i32).load(std::sync::atomic::Ordering::Relaxed)
                            } == 0) as i32 != 0 {
            let mut i_idx: i32 = 0;
            let mut p_page: *const MemPage = core::ptr::null();
            p_page = unsafe { (*p_cur_1).p_page };
            if unsafe { (*p_page).leaf } != 0 ||
                    (unsafe { (*p_page).int_key } == 0) as i32 != 0 {
                n_entry += unsafe { (*p_page).n_cell } as i64;
            }
            if unsafe { (*p_page).leaf } != 0 {
                '__b92: loop {
                    '__c92: loop {
                        if unsafe { (*p_cur_1).i_page } as i32 == 0 {
                            *pn_entry_1 = n_entry;
                            return move_to_root(p_cur_1);
                        }
                        move_to_parent(unsafe { &mut *p_cur_1 });
                        break '__c92;
                    }
                    if !(unsafe { (*p_cur_1).ix } as i32 >=
                                    unsafe { (*unsafe { (*p_cur_1).p_page }).n_cell } as i32) {
                        break '__b92;
                    }
                }
                {
                    let __p = unsafe { &mut (*p_cur_1).ix };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                p_page = unsafe { (*p_cur_1).p_page };
            }
            i_idx = unsafe { (*p_cur_1).ix } as i32;
            if i_idx == unsafe { (*p_page).n_cell } as i32 {
                rc =
                    move_to_child(unsafe { &mut *p_cur_1 },
                        unsafe {
                            sqlite3_get4byte(unsafe {
                                        &raw mut *unsafe {
                                                    (*p_page).a_data.offset((unsafe { (*p_page).hdr_offset } as
                                                                    i32 + 8) as isize)
                                                }
                                    } as *const u8)
                        });
            } else {
                rc =
                    move_to_child(unsafe { &mut *p_cur_1 },
                        unsafe {
                            sqlite3_get4byte(unsafe {
                                        unsafe {
                                            (*p_page).a_data.offset((unsafe { (*p_page).mask_page } as
                                                            i32 &
                                                        ((unsafe {
                                                                            *unsafe {
                                                                                    unsafe {
                                                                                        (*p_page).a_cell_idx.offset((2 * i_idx) as
                                                                                                    isize).offset(0 as isize)
                                                                                    }
                                                                                }
                                                                        } as i32) << 8 |
                                                            unsafe {
                                                                    *unsafe {
                                                                            unsafe {
                                                                                (*p_page).a_cell_idx.offset((2 * i_idx) as
                                                                                            isize).offset(1 as isize)
                                                                            }
                                                                        }
                                                                } as i32)) as isize)
                                        }
                                    } as *const u8)
                        });
            }
        }
        return rc;
    }
}
extern "C" fn btree_payload_to_local(p_page_1: &MemPage, n_payload_1: i64)
    -> i32 {
    let mut max_local: i32 = 0;
    max_local = (*p_page_1).max_local as i32;
    { let _ = 0; };
    if n_payload_1 <= max_local as i64 {
        return n_payload_1 as i32;
    } else {
        let mut min_local: i32 = 0;
        let mut surplus: i32 = 0;
        min_local = (*p_page_1).min_local as i32;
        surplus =
            (min_local as i64 +
                    (n_payload_1 - min_local as i64) %
                        (unsafe { (*(*p_page_1).p_bt).usable_size } - 4 as u32) as
                            i64) as i32;
        return if surplus <= max_local { surplus } else { min_local };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_transfer_row(p_dest_1: &BtCursor,
    p_src_1: *mut BtCursor, i_key_1: i64) -> i32 {
    unsafe {
        let p_bt: *mut BtShared = (*p_dest_1).p_bt;
        let mut a_out: *mut u8 = unsafe { (*p_bt).p_tmp_space };
        let mut a_in: *const u8 = core::ptr::null();
        let mut n_in: u32 = 0 as u32;
        let mut n_rem: u32 = 0 as u32;
        get_cell_info(unsafe { &mut *p_src_1 });
        if unsafe { (*p_src_1).info.n_payload } < 128 as u32 {
            unsafe {
                *{
                            let __p = &mut a_out;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        } = unsafe { (*p_src_1).info.n_payload } as u8
            };
        } else {
            {
                let __n =
                    unsafe {
                        sqlite3_put_varint(a_out,
                            unsafe { (*p_src_1).info.n_payload } as u64)
                    };
                let __p = &mut a_out;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        if (*p_dest_1).p_key_info == core::ptr::null_mut() {
            {
                let __n =
                    unsafe { sqlite3_put_varint(a_out, i_key_1 as u64) };
                let __p = &mut a_out;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        n_in = unsafe { (*p_src_1).info.n_local } as u32;
        a_in = unsafe { (*p_src_1).info.p_payload } as *const u8;
        if unsafe { a_in.add(n_in as usize) } >
                unsafe { (*unsafe { (*p_src_1).p_page }).a_data_end } as
                    *const u8 {
            return unsafe { sqlite3_corrupt_error(9776) };
        }
        n_rem = unsafe { (*p_src_1).info.n_payload };
        if n_in == n_rem &&
                n_in < unsafe { (*(*p_dest_1).p_page).max_local } as u32 {
            unsafe {
                memcpy(a_out as *mut (), a_in as *const (), n_in as u64)
            };
            unsafe {
                (*p_bt).n_preformat_size =
                    (n_in +
                            unsafe { a_out.offset_from(unsafe { (*p_bt).p_tmp_space }) }
                                        as i64 as i32 as u32) as i32
            };
            return 0;
        } else {
            let mut rc: i32 = 0;
            let p_src_pager: *mut Pager =
                unsafe { (*unsafe { (*p_src_1).p_bt }).p_pager };
            let mut p_pgno_out: *mut u8 = core::ptr::null_mut();
            let mut ovfl_in: Pgno = 0 as Pgno;
            let mut p_page_in: *mut DbPage = core::ptr::null_mut();
            let mut p_page_out: *mut MemPage = core::ptr::null_mut();
            let mut n_out: u32 = 0 as u32;
            n_out =
                btree_payload_to_local(unsafe { &*(*p_dest_1).p_page },
                        unsafe { (*p_src_1).info.n_payload } as i64) as u32;
            unsafe {
                (*p_bt).n_preformat_size =
                    n_out as i32 +
                        unsafe { a_out.offset_from(unsafe { (*p_bt).p_tmp_space }) }
                                as i64 as i32
            };
            if n_out < unsafe { (*p_src_1).info.n_payload } {
                p_pgno_out = unsafe { a_out.add(n_out as usize) };
                unsafe { (*p_bt).n_preformat_size += 4 };
            }
            if n_rem > n_in {
                if unsafe {
                            unsafe { a_in.add(n_in as usize).offset(4 as isize) }
                        } >
                        unsafe { (*unsafe { (*p_src_1).p_page }).a_data_end } as
                            *const u8 {
                    return unsafe { sqlite3_corrupt_error(9801) };
                }
                ovfl_in =
                    unsafe {
                        sqlite3_get4byte(unsafe {
                                    &raw mut *unsafe {
                                                (*p_src_1).info.p_payload.add(n_in as usize)
                                            }
                                } as *const u8)
                    };
            }
            '__b93: loop {
                '__c93: loop {
                    n_rem -= n_out;
                    '__b94: loop {
                        '__c94: loop {
                            { let _ = 0; };
                            if n_in > 0 as u32 {
                                let n_copy: i32 =
                                    if n_out < n_in { n_out } else { n_in } as i32;
                                unsafe {
                                    memcpy(a_out as *mut (), a_in as *const (), n_copy as u64)
                                };
                                n_out -= n_copy as u32;
                                n_in -= n_copy as u32;
                                {
                                    let __n = n_copy;
                                    let __p = &mut a_out;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                                {
                                    let __n = n_copy;
                                    let __p = &mut a_in;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                            }
                            if n_out > 0 as u32 {
                                unsafe { sqlite3_pager_unref(p_page_in) };
                                p_page_in = core::ptr::null_mut();
                                rc =
                                    unsafe {
                                        sqlite3_pager_get(p_src_pager, ovfl_in, &mut p_page_in, 2)
                                    };
                                if rc == 0 {
                                    a_in =
                                        unsafe { sqlite3_pager_get_data(p_page_in) } as *const u8;
                                    ovfl_in = unsafe { sqlite3_get4byte(a_in) };
                                    {
                                        let __n = 4;
                                        let __p = &mut a_in;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    n_in =
                                        unsafe { (*unsafe { (*p_src_1).p_bt }).usable_size } -
                                            4 as u32;
                                }
                            }
                            break '__c94;
                        }
                        if !(rc == 0 && n_out > 0 as u32) { break '__b94; }
                    }
                    if rc == 0 && n_rem > 0 as u32 && !(p_pgno_out).is_null() {
                        let mut pgno_new: Pgno = 0 as Pgno;
                        let mut p_new: *mut MemPage = core::ptr::null_mut();
                        rc =
                            allocate_btree_page(p_bt, &mut p_new, &mut pgno_new,
                                0 as Pgno, 0 as u8);
                        unsafe { sqlite3_put4byte(p_pgno_out, pgno_new) };
                        if unsafe { (*p_bt).auto_vacuum } != 0 &&
                                !(p_page_out).is_null() {
                            ptrmap_put(p_bt, pgno_new, 4 as u8,
                                unsafe { (*p_page_out).pgno }, &mut rc);
                        }
                        release_page(p_page_out);
                        p_page_out = p_new;
                        if !(p_page_out).is_null() {
                            p_pgno_out = unsafe { (*p_page_out).a_data };
                            unsafe { sqlite3_put4byte(p_pgno_out, 0 as u32) };
                            a_out = unsafe { p_pgno_out.offset(4 as isize) };
                            n_out =
                                if (unsafe { (*p_bt).usable_size } - 4 as u32) < n_rem {
                                    (unsafe { (*p_bt).usable_size }) - 4 as u32
                                } else { n_rem };
                        }
                    }
                    break '__c93;
                }
                if !(n_rem > 0 as u32 && rc == 0) { break '__b93; }
            }
            release_page(p_page_out);
            unsafe { sqlite3_pager_unref(p_page_in) };
            return rc;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_clear_cache(p: &Btree) -> () {
    let p_bt: *const BtShared = (*p).p_bt as *const BtShared;
    if unsafe { (*p_bt).in_transaction } as i32 == 0 {
        unsafe { sqlite3_pager_clear_cache(unsafe { (*p_bt).p_pager }) };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_sharable(p: &Btree) -> i32 {
    return (*p).sharable as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_btree_connection_count(p: &Btree) -> i32 {
    return unsafe { (*(*p).p_bt).n_ref };
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VdbeOpList {
    opcode: u8,
    p1: i8,
    p2: i8,
    p3: i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct InitData {
    db: *mut sqlite3,
    pz_err_msg: *mut *mut i8,
    i_db: i32,
    rc: i32,
    m_init_flags: u32,
    n_init_row: u32,
    mx_page: Pgno,
}
static mut fake_cursor: u8 = 0 as u8;
extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    static sqlite3_version: [i8; 0];
    fn sqlite3_libversion()
    -> *const i8;
    fn sqlite3_sourceid()
    -> *const i8;
    fn sqlite3_libversion_number()
    -> i32;
    fn sqlite3_compileoption_used(z_opt_name_1: *const i8)
    -> i32;
    fn sqlite3_compileoption_get(n_1: i32)
    -> *const i8;
    fn sqlite3_threadsafe()
    -> i32;
    fn sqlite3_close(_: *mut sqlite3)
    -> i32;
    fn sqlite3_close_v2(_: *mut sqlite3)
    -> i32;
    fn sqlite3_exec(_: *mut sqlite3, sql: *const i8,
    callback:
        Option<unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8)
            -> i32>, _: *mut (), errmsg: *mut *mut i8)
    -> i32;
    fn sqlite3_initialize()
    -> i32;
    fn sqlite3_shutdown()
    -> i32;
    fn sqlite3_os_init()
    -> i32;
    fn sqlite3_os_end()
    -> i32;
    fn sqlite3_config(_: i32, ...)
    -> i32;
    fn sqlite3_db_config(_: *mut sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_extended_result_codes(_: *mut sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_last_insert_rowid(_: *mut sqlite3)
    -> sqlite3_int64;
    fn sqlite3_set_last_insert_rowid(_: *mut sqlite3, _: sqlite3_int64)
    -> ();
    fn sqlite3_changes(_: *mut sqlite3)
    -> i32;
    fn sqlite3_changes64(_: *mut sqlite3)
    -> sqlite3_int64;
    fn sqlite3_total_changes(_: *mut sqlite3)
    -> i32;
    fn sqlite3_total_changes64(_: *mut sqlite3)
    -> sqlite3_int64;
    fn sqlite3_interrupt(_: *mut sqlite3)
    -> ();
    fn sqlite3_is_interrupted(_: *mut sqlite3)
    -> i32;
    fn sqlite3_complete(sql: *const i8)
    -> i32;
    fn sqlite3_complete16(sql: *const ())
    -> i32;
    fn sqlite3_incomplete(sql: *const i8)
    -> sqlite3_int64;
    fn sqlite3_busy_handler(_: *mut sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), i32) -> i32>, _: *mut ())
    -> i32;
    fn sqlite3_busy_timeout(_: *mut sqlite3, ms: i32)
    -> i32;
    fn sqlite3_setlk_timeout(_: *mut sqlite3, ms: i32, flags: i32)
    -> i32;
    fn sqlite3_get_table(db: *mut sqlite3, z_sql_1: *const i8,
    paz_result_1: *mut *mut *mut i8, pn_row_1: *mut i32,
    pn_column_1: *mut i32, pz_errmsg_1: *mut *mut i8)
    -> i32;
    fn sqlite3_free_table(result: *mut *mut i8)
    -> ();
    fn sqlite3_mprintf(_: *const i8, ...)
    -> *mut i8;
    fn sqlite3_vmprintf(_: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_snprintf(_: i32, _: *mut i8, _: *const i8, ...)
    -> *mut i8;
    fn sqlite3_vsnprintf(_: i32, _: *mut i8, _: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_malloc(_: i32)
    -> *mut ();
    fn sqlite3_malloc64(_: sqlite3_uint64)
    -> *mut ();
    fn sqlite3_realloc(_: *mut (), _: i32)
    -> *mut ();
    fn sqlite3_realloc64(_: *mut (), _: sqlite3_uint64)
    -> *mut ();
    fn sqlite3_free(_: *mut ())
    -> ();
    fn sqlite3_msize(_: *mut ())
    -> sqlite3_uint64;
    fn sqlite3_memory_used()
    -> sqlite3_int64;
    fn sqlite3_memory_highwater(reset_flag_1: i32)
    -> sqlite3_int64;
    fn sqlite3_randomness(n_1: i32, p_1: *mut ())
    -> ();
    fn sqlite3_set_authorizer(_: *mut sqlite3,
    x_auth_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
            *const i8, *const i8) -> i32>, p_user_data_1: *mut ())
    -> i32;
    fn sqlite3_trace(_: *mut sqlite3,
    x_trace_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_profile(_: *mut sqlite3,
    x_profile_1: Option<unsafe extern "C" fn(*mut (), *const i8, u64) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_trace_v2(_: *mut sqlite3, u_mask_1: u32,
    x_callback_1:
        Option<unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
    p_ctx_1: *mut ())
    -> i32;
    fn sqlite3_progress_handler(_: *mut sqlite3, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> ();
    fn sqlite3_open(filename: *const i8, pp_db_1: *mut *mut sqlite3)
    -> i32;
    fn sqlite3_open16(filename: *const (), pp_db_1: *mut *mut sqlite3)
    -> i32;
    fn sqlite3_open_v2(filename: *const i8, pp_db_1: *mut *mut sqlite3,
    flags: i32, z_vfs_1: *const i8)
    -> i32;
    fn sqlite3_uri_parameter(z: sqlite3_filename, z_param_1: *const i8)
    -> *const i8;
    fn sqlite3_uri_boolean(z: sqlite3_filename, z_param_1: *const i8,
    b_default_1: i32)
    -> i32;
    fn sqlite3_uri_int64(_: sqlite3_filename, _: *const i8, _: sqlite3_int64)
    -> sqlite3_int64;
    fn sqlite3_uri_key(z: sqlite3_filename, n_1: i32)
    -> *const i8;
    fn sqlite3_filename_database(_: sqlite3_filename)
    -> *const i8;
    fn sqlite3_filename_journal(_: sqlite3_filename)
    -> *const i8;
    fn sqlite3_filename_wal(_: sqlite3_filename)
    -> *const i8;
    fn sqlite3_database_file_object(_: *const i8)
    -> *mut sqlite3_file;
    fn sqlite3_create_filename(z_database_1: *const i8,
    z_journal_1: *const i8, z_wal_1: *const i8, n_param_1: i32,
    az_param_1: *mut *const i8)
    -> sqlite3_filename;
    fn sqlite3_free_filename(_: sqlite3_filename)
    -> ();
    fn sqlite3_errcode(db: *mut sqlite3)
    -> i32;
    fn sqlite3_extended_errcode(db: *mut sqlite3)
    -> i32;
    fn sqlite3_errmsg(_: *mut sqlite3)
    -> *const i8;
    fn sqlite3_errmsg16(_: *mut sqlite3)
    -> *const ();
    fn sqlite3_errstr(_: i32)
    -> *const i8;
    fn sqlite3_error_offset(db: *mut sqlite3)
    -> i32;
    fn sqlite3_set_errmsg(db: *mut sqlite3, errcode: i32,
    z_err_msg_1: *const i8)
    -> i32;
    fn sqlite3_limit(_: *mut sqlite3, id: i32, new_val_1: i32)
    -> i32;
    fn sqlite3_prepare(db: *mut sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut sqlite3_stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v2(db: *mut sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut sqlite3_stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v3(db: *mut sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    prep_flags_1: u32, pp_stmt_1: *mut *mut sqlite3_stmt,
    pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare16(db: *mut sqlite3, z_sql_1: *const (), n_byte_1: i32,
    pp_stmt_1: *mut *mut sqlite3_stmt, pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v2(db: *mut sqlite3, z_sql_1: *const (),
    n_byte_1: i32, pp_stmt_1: *mut *mut sqlite3_stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v3(db: *mut sqlite3, z_sql_1: *const (),
    n_byte_1: i32, prep_flags_1: u32, pp_stmt_1: *mut *mut sqlite3_stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_sql(p_stmt_1: *mut sqlite3_stmt)
    -> *const i8;
    fn sqlite3_expanded_sql(p_stmt_1: *mut sqlite3_stmt)
    -> *mut i8;
    fn sqlite3_stmt_readonly(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_stmt_isexplain(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_stmt_explain(p_stmt_1: *mut sqlite3_stmt, e_mode_1: i32)
    -> i32;
    fn sqlite3_stmt_busy(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_bind_blob(_: *mut sqlite3_stmt, _: i32, _: *const (), n: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_blob64(_: *mut sqlite3_stmt, _: i32, _: *const (),
    _: sqlite3_uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_double(_: *mut sqlite3_stmt, _: i32, _: f64)
    -> i32;
    fn sqlite3_bind_int(_: *mut sqlite3_stmt, _: i32, _: i32)
    -> i32;
    fn sqlite3_bind_int64(_: *mut sqlite3_stmt, _: i32, _: sqlite3_int64)
    -> i32;
    fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: i32)
    -> i32;
    fn sqlite3_bind_text(_: *mut sqlite3_stmt, _: i32, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text16(_: *mut sqlite3_stmt, _: i32, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text64(_: *mut sqlite3_stmt, _: i32, _: *const i8,
    _: sqlite3_uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> i32;
    fn sqlite3_bind_value(_: *mut sqlite3_stmt, _: i32,
    _: *const sqlite3_value)
    -> i32;
    fn sqlite3_bind_pointer(_: *mut sqlite3_stmt, _: i32, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_zeroblob(_: *mut sqlite3_stmt, _: i32, n: i32)
    -> i32;
    fn sqlite3_bind_zeroblob64(_: *mut sqlite3_stmt, _: i32,
    _: sqlite3_uint64)
    -> i32;
    fn sqlite3_bind_parameter_count(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_bind_parameter_name(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_bind_parameter_index(_: *mut sqlite3_stmt, z_name_1: *const i8)
    -> i32;
    fn sqlite3_clear_bindings(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_column_count(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_column_name(_: *mut sqlite3_stmt, n_1: i32)
    -> *const i8;
    fn sqlite3_column_name16(_: *mut sqlite3_stmt, n_1: i32)
    -> *const ();
    fn sqlite3_column_database_name(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_database_name16(_: *mut sqlite3_stmt, _: i32)
    -> *const ();
    fn sqlite3_column_table_name(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_table_name16(_: *mut sqlite3_stmt, _: i32)
    -> *const ();
    fn sqlite3_column_origin_name(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_origin_name16(_: *mut sqlite3_stmt, _: i32)
    -> *const ();
    fn sqlite3_column_decltype(_: *mut sqlite3_stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_decltype16(_: *mut sqlite3_stmt, _: i32)
    -> *const ();
    fn sqlite3_step(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_data_count(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_column_blob(_: *mut sqlite3_stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_double(_: *mut sqlite3_stmt, i_col_1: i32)
    -> f64;
    fn sqlite3_column_int(_: *mut sqlite3_stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_int64(_: *mut sqlite3_stmt, i_col_1: i32)
    -> sqlite3_int64;
    fn sqlite3_column_text(_: *mut sqlite3_stmt, i_col_1: i32)
    -> *const u8;
    fn sqlite3_column_text16(_: *mut sqlite3_stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_value(_: *mut sqlite3_stmt, i_col_1: i32)
    -> *mut sqlite3_value;
    fn sqlite3_column_bytes(_: *mut sqlite3_stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_bytes16(_: *mut sqlite3_stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_type(_: *mut sqlite3_stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_finalize(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_reset(p_stmt_1: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_create_function(db: *mut sqlite3, z_function_name_1: *const i8,
    n_arg_1: i32, e_text_rep_1: i32, p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>)
    -> i32;
    fn sqlite3_create_function16(db: *mut sqlite3,
    z_function_name_1: *const (), n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>)
    -> i32;
    fn sqlite3_create_function_v2(db: *mut sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_window_function(db: *mut sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_step_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    x_value_1: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    x_inverse_1:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_aggregate_count(_: *mut sqlite3_context)
    -> i32;
    fn sqlite3_expired(_: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_transfer_bindings(_: *mut sqlite3_stmt, _: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_global_recover()
    -> i32;
    fn sqlite3_thread_cleanup()
    -> ();
    fn sqlite3_memory_alarm(_:
        Option<unsafe extern "C" fn(*mut (), i64, i32) -> ()>, _: *mut (),
    _: sqlite3_int64)
    -> i32;
    fn sqlite3_value_blob(_: *mut sqlite3_value)
    -> *const ();
    fn sqlite3_value_double(_: *mut sqlite3_value)
    -> f64;
    fn sqlite3_value_int(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_int64(_: *mut sqlite3_value)
    -> sqlite3_int64;
    fn sqlite3_value_pointer(_: *mut sqlite3_value, _: *const i8)
    -> *mut ();
    fn sqlite3_value_text(_: *mut sqlite3_value)
    -> *const u8;
    fn sqlite3_value_text16(_: *mut sqlite3_value)
    -> *const ();
    fn sqlite3_value_text16le(_: *mut sqlite3_value)
    -> *const ();
    fn sqlite3_value_text16be(_: *mut sqlite3_value)
    -> *const ();
    fn sqlite3_value_bytes(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_bytes16(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_type(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_numeric_type(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_nochange(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_frombind(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_encoding(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_subtype(_: *mut sqlite3_value)
    -> u32;
    fn sqlite3_value_dup(_: *const sqlite3_value)
    -> *mut sqlite3_value;
    fn sqlite3_value_free(_: *mut sqlite3_value)
    -> ();
    fn sqlite3_aggregate_context(_: *mut sqlite3_context, n_bytes_1: i32)
    -> *mut ();
    fn sqlite3_user_data(_: *mut sqlite3_context)
    -> *mut ();
    fn sqlite3_context_db_handle(_: *mut sqlite3_context)
    -> *mut sqlite3;
    fn sqlite3_get_auxdata(_: *mut sqlite3_context, n_1: i32)
    -> *mut ();
    fn sqlite3_set_auxdata(_: *mut sqlite3_context, n_1: i32, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_get_clientdata(_: *mut sqlite3, _: *const i8)
    -> *mut ();
    fn sqlite3_set_clientdata(_: *mut sqlite3, _: *const i8, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_result_blob(_: *mut sqlite3_context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_blob64(_: *mut sqlite3_context, _: *const (),
    _: sqlite3_uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_double(_: *mut sqlite3_context, _: f64)
    -> ();
    fn sqlite3_result_error(_: *mut sqlite3_context, _: *const i8, _: i32)
    -> ();
    fn sqlite3_result_error16(_: *mut sqlite3_context, _: *const (), _: i32)
    -> ();
    fn sqlite3_result_error_toobig(_: *mut sqlite3_context)
    -> ();
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context)
    -> ();
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: i32)
    -> ();
    fn sqlite3_result_int(_: *mut sqlite3_context, _: i32)
    -> ();
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64)
    -> ();
    fn sqlite3_result_null(_: *mut sqlite3_context)
    -> ();
    fn sqlite3_result_text(_: *mut sqlite3_context, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text64(_: *mut sqlite3_context, z: *const i8,
    n: sqlite3_uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> ();
    fn sqlite3_result_text16(_: *mut sqlite3_context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16le(_: *mut sqlite3_context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16be(_: *mut sqlite3_context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value)
    -> ();
    fn sqlite3_result_pointer(_: *mut sqlite3_context, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_zeroblob(_: *mut sqlite3_context, n: i32)
    -> ();
    fn sqlite3_result_zeroblob64(_: *mut sqlite3_context, n: sqlite3_uint64)
    -> i32;
    fn sqlite3_result_subtype(_: *mut sqlite3_context, _: u32)
    -> ();
    fn sqlite3_create_collation(_: *mut sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_create_collation_v2(_: *mut sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>, x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_collation16(_: *mut sqlite3, z_name_1: *const (),
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_collation_needed(_: *mut sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut sqlite3, i32, *const i8)
            -> ()>)
    -> i32;
    fn sqlite3_collation_needed16(_: *mut sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut sqlite3, i32, *const ())
            -> ()>)
    -> i32;
    fn sqlite3_sleep(_: i32)
    -> i32;
    static mut sqlite3_temp_directory: *mut i8;
    static mut sqlite3_data_directory: *mut i8;
    fn sqlite3_win32_set_directory(type__1: u64, z_value_1: *mut ())
    -> i32;
    fn sqlite3_win32_set_directory8(type__1: u64, z_value_1: *const i8)
    -> i32;
    fn sqlite3_win32_set_directory16(type__1: u64, z_value_1: *const ())
    -> i32;
    fn sqlite3_get_autocommit(_: *mut sqlite3)
    -> i32;
    fn sqlite3_db_handle(_: *mut sqlite3_stmt)
    -> *mut sqlite3;
    fn sqlite3_db_name(db: *mut sqlite3, n_1: i32)
    -> *const i8;
    fn sqlite3_db_filename(db: *mut sqlite3, z_db_name_1: *const i8)
    -> sqlite3_filename;
    fn sqlite3_db_readonly(db: *mut sqlite3, z_db_name_1: *const i8)
    -> i32;
    fn sqlite3_txn_state(_: *mut sqlite3, z_schema_1: *const i8)
    -> i32;
    fn sqlite3_next_stmt(p_db_1: *mut sqlite3, p_stmt_1: *mut sqlite3_stmt)
    -> *mut sqlite3_stmt;
    fn sqlite3_commit_hook(_: *mut sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_rollback_hook(_: *mut sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_autovacuum_pages(db: *mut sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
    _: *mut (), _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_update_hook(_: *mut sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, i64)
            -> ()>, _: *mut ())
    -> *mut ();
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_release_memory(_: i32)
    -> i32;
    fn sqlite3_db_release_memory(_: *mut sqlite3)
    -> i32;
    fn sqlite3_soft_heap_limit64(n_1: sqlite3_int64)
    -> sqlite3_int64;
    fn sqlite3_hard_heap_limit64(n_1: sqlite3_int64)
    -> sqlite3_int64;
    fn sqlite3_soft_heap_limit(n_1: i32)
    -> ();
    fn sqlite3_table_column_metadata(db: *mut sqlite3, z_db_name_1: *const i8,
    z_table_name_1: *const i8, z_column_name_1: *const i8,
    pz_data_type_1: *mut *const i8, pz_coll_seq_1: *mut *const i8,
    p_not_null_1: *mut i32, p_primary_key_1: *mut i32, p_autoinc_1: *mut i32)
    -> i32;
    fn sqlite3_load_extension(db: *mut sqlite3, z_file_1: *const i8,
    z_proc_1: *const i8, pz_err_msg_1: *mut *mut i8)
    -> i32;
    fn sqlite3_enable_load_extension(db: *mut sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_cancel_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_reset_auto_extension()
    -> ();
    fn sqlite3_create_module(db: *mut sqlite3, z_name_1: *const i8,
    p: *const sqlite3_module, p_client_data_1: *mut ())
    -> i32;
    fn sqlite3_create_module_v2(db: *mut sqlite3, z_name_1: *const i8,
    p: *const sqlite3_module, p_client_data_1: *mut (),
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_drop_modules(db: *mut sqlite3, az_keep_1: *mut *const i8)
    -> i32;
    fn sqlite3_declare_vtab(_: *mut sqlite3, z_sql_1: *const i8)
    -> i32;
    fn sqlite3_overload_function(_: *mut sqlite3, z_func_name_1: *const i8,
    n_arg_1: i32)
    -> i32;
    fn sqlite3_blob_open(_: *mut sqlite3, z_db_1: *const i8,
    z_table_1: *const i8, z_column_1: *const i8, i_row_1: sqlite3_int64,
    flags: i32, pp_blob_1: *mut *mut sqlite3_blob)
    -> i32;
    fn sqlite3_blob_reopen(_: *mut sqlite3_blob, _: sqlite3_int64)
    -> i32;
    fn sqlite3_blob_close(_: *mut sqlite3_blob)
    -> i32;
    fn sqlite3_blob_bytes(_: *mut sqlite3_blob)
    -> i32;
    fn sqlite3_blob_read(_: *mut sqlite3_blob, z_1: *mut (), n_1: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_blob_write(_: *mut sqlite3_blob, z: *const (), n: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_vfs_find(z_vfs_name_1: *const i8)
    -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(_: *mut sqlite3_vfs, make_dflt_1: i32)
    -> i32;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs)
    -> i32;
    fn sqlite3_mutex_alloc(_: i32)
    -> *mut sqlite3_mutex;
    fn sqlite3_mutex_free(_: *mut sqlite3_mutex)
    -> ();
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex)
    -> ();
    fn sqlite3_mutex_try(_: *mut sqlite3_mutex)
    -> i32;
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex)
    -> ();
    fn sqlite3_mutex_held(_: *mut sqlite3_mutex)
    -> i32;
    fn sqlite3_mutex_notheld(_: *mut sqlite3_mutex)
    -> i32;
    fn sqlite3_db_mutex(_: *mut sqlite3)
    -> *mut sqlite3_mutex;
    fn sqlite3_file_control(_: *mut sqlite3, z_db_name_1: *const i8, op: i32,
    _: *mut ())
    -> i32;
    fn sqlite3_test_control(op: i32, ...)
    -> i32;
    fn sqlite3_keyword_count()
    -> i32;
    fn sqlite3_keyword_name(_: i32, _: *mut *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_keyword_check(_: *const i8, _: i32)
    -> i32;
    fn sqlite3_str_new(_: *mut sqlite3)
    -> *mut sqlite3_str;
    fn sqlite3_str_finish(_: *mut sqlite3_str)
    -> *mut i8;
    fn sqlite3_str_free(_: *mut sqlite3_str)
    -> ();
    fn sqlite3_result_str(_: *mut sqlite3_context, _: *mut sqlite3_str,
    _: i32)
    -> ();
    fn sqlite3_str_appendf(_: *mut sqlite3_str, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_str_vappendf(_: *mut sqlite3_str, z_format_1: *const i8,
    _: *mut i8)
    -> ();
    fn sqlite3_str_append(_: *mut sqlite3_str, z_in_1: *const i8, n_1: i32)
    -> ();
    fn sqlite3_str_appendall(_: *mut sqlite3_str, z_in_1: *const i8)
    -> ();
    fn sqlite3_str_appendchar(_: *mut sqlite3_str, n_1: i32, c_1: i8)
    -> ();
    fn sqlite3_str_reset(_: *mut sqlite3_str)
    -> ();
    fn sqlite3_str_truncate(_: *mut sqlite3_str, n_1: i32)
    -> ();
    fn sqlite3_str_errcode(_: *mut sqlite3_str)
    -> i32;
    fn sqlite3_str_length(_: *mut sqlite3_str)
    -> i32;
    fn sqlite3_str_value(_: *mut sqlite3_str)
    -> *mut i8;
    fn sqlite3_status(op: i32, p_current_1: *mut i32, p_highwater_1: *mut i32,
    reset_flag_1: i32)
    -> i32;
    fn sqlite3_status64(op: i32, p_current_1: *mut sqlite3_int64,
    p_highwater_1: *mut sqlite3_int64, reset_flag_1: i32)
    -> i32;
    fn sqlite3_db_status(_: *mut sqlite3, op: i32, p_cur_1: *mut i32,
    p_hiwtr_1: *mut i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_db_status64(_: *mut sqlite3, _: i32, _: *mut sqlite3_int64,
    _: *mut sqlite3_int64, _: i32)
    -> i32;
    fn sqlite3_stmt_status(_: *mut sqlite3_stmt, op: i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_backup_init(p_dest_1: *mut sqlite3, z_dest_name_1: *const i8,
    p_source_1: *mut sqlite3, z_source_name_1: *const i8)
    -> *mut sqlite3_backup;
    fn sqlite3_backup_step(p: *mut sqlite3_backup, n_page_1: i32)
    -> i32;
    fn sqlite3_backup_finish(p: *mut sqlite3_backup)
    -> i32;
    fn sqlite3_backup_remaining(p: *mut sqlite3_backup)
    -> i32;
    fn sqlite3_backup_pagecount(p: *mut sqlite3_backup)
    -> i32;
    fn sqlite3_unlock_notify(p_blocked_1: *mut sqlite3,
    x_notify_1: Option<unsafe extern "C" fn(*mut *mut (), i32) -> ()>,
    p_notify_arg_1: *mut ())
    -> i32;
    fn sqlite3_stricmp(_: *const i8, _: *const i8)
    -> i32;
    fn sqlite3_strnicmp(_: *const i8, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_strglob(z_glob_1: *const i8, z_str_1: *const i8)
    -> i32;
    fn sqlite3_strlike(z_glob_1: *const i8, z_str_1: *const i8, c_esc_1: u32)
    -> i32;
    fn sqlite3_log(i_err_code_1: i32, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_wal_hook(_: *mut sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), *mut sqlite3, *const i8, i32)
            -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_wal_autocheckpoint(db: *mut sqlite3, n_1: i32)
    -> i32;
    fn sqlite3_wal_checkpoint(db: *mut sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_wal_checkpoint_v2(db: *mut sqlite3, z_db_1: *const i8,
    e_mode_1: i32, pn_log_1: *mut i32, pn_ckpt_1: *mut i32)
    -> i32;
    fn sqlite3_vtab_config(_: *mut sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_vtab_on_conflict(_: *mut sqlite3)
    -> i32;
    fn sqlite3_vtab_nochange(_: *mut sqlite3_context)
    -> i32;
    fn sqlite3_vtab_collation(_: *mut sqlite3_index_info, _: i32)
    -> *const i8;
    fn sqlite3_vtab_distinct(_: *mut sqlite3_index_info)
    -> i32;
    fn sqlite3_vtab_in(_: *mut sqlite3_index_info, i_cons_1: i32,
    b_handle_1: i32)
    -> i32;
    fn sqlite3_vtab_in_first(p_val_1: *mut sqlite3_value,
    pp_out_1: *mut *mut sqlite3_value)
    -> i32;
    fn sqlite3_vtab_in_next(p_val_1: *mut sqlite3_value,
    pp_out_1: *mut *mut sqlite3_value)
    -> i32;
    fn sqlite3_vtab_rhs_value(_: *mut sqlite3_index_info, _: i32,
    pp_val_1: *mut *mut sqlite3_value)
    -> i32;
    fn sqlite3_stmt_scanstatus(p_stmt_1: *mut sqlite3_stmt, idx: i32,
    i_scan_status_op_1: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_v2(p_stmt_1: *mut sqlite3_stmt, idx: i32,
    i_scan_status_op_1: i32, flags: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_reset(_: *mut sqlite3_stmt)
    -> ();
    fn sqlite3_db_cacheflush(_: *mut sqlite3)
    -> i32;
    fn sqlite3_system_errno(_: *mut sqlite3)
    -> i32;
    fn sqlite3_snapshot_get(db: *mut sqlite3, z_schema_1: *const i8,
    pp_snapshot_1: *mut *mut sqlite3_snapshot)
    -> i32;
    fn sqlite3_snapshot_open(db: *mut sqlite3, z_schema_1: *const i8,
    p_snapshot_1: *mut sqlite3_snapshot)
    -> i32;
    fn sqlite3_snapshot_free(_: *mut sqlite3_snapshot)
    -> ();
    fn sqlite3_snapshot_cmp(p1: *mut sqlite3_snapshot,
    p2: *mut sqlite3_snapshot)
    -> i32;
    fn sqlite3_snapshot_recover(db: *mut sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_serialize(db: *mut sqlite3, z_schema_1: *const i8,
    pi_size_1: *mut sqlite3_int64, m_flags_1: u32)
    -> *mut u8;
    fn sqlite3_deserialize(db: *mut sqlite3, z_schema_1: *const i8,
    p_data_1: *mut u8, sz_db_1: sqlite3_int64, sz_buf_1: sqlite3_int64,
    m_flags_1: u32)
    -> i32;
    fn sqlite3_carray_bind_v2(p_stmt_1: *mut sqlite3_stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>, p_del_1: *mut ())
    -> i32;
    fn sqlite3_carray_bind(p_stmt_1: *mut sqlite3_stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_rtree_geometry_callback(db: *mut sqlite3, z_geom_1: *const i8,
    x_geom_1:
        Option<unsafe extern "C" fn(*mut sqlite3_rtree_geometry, i32,
            *mut f64, *mut i32) -> i32>, p_context_1: *mut ())
    -> i32;
    fn sqlite3_rtree_query_callback(db: *mut sqlite3,
    z_query_func_1: *const i8,
    x_query_func_1:
        Option<unsafe extern "C" fn(*mut sqlite3_rtree_query_info) -> i32>,
    p_context_1: *mut (),
    x_destructor_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_hash_init(_: *mut Hash)
    -> ();
    fn sqlite3_hash_insert(_: *mut Hash, p_key_1: *const i8,
    p_data_1: *mut ())
    -> *mut ();
    fn sqlite3_hash_find(_: *const Hash, p_key_1: *const i8)
    -> *mut ();
    fn sqlite3_hash_clear(_: *mut Hash)
    -> ();
    static mut sqlite3_tree_trace: u32;
    static mut sqlite3_where_trace: u32;
    fn sqlite3OsInit()
    -> i32;
    fn sqlite3_os_close(_: *mut sqlite3_file)
    -> ();
    fn sqlite3_os_read(_: *mut sqlite3_file, _: *mut (), amt: i32,
    offset: i64)
    -> i32;
    fn sqlite3_os_write(_: *mut sqlite3_file, _: *const (), amt: i32,
    offset: i64)
    -> i32;
    fn sqlite3_os_truncate(_: *mut sqlite3_file, size: i64)
    -> i32;
    fn sqlite3_os_sync(_: *mut sqlite3_file, _: i32)
    -> i32;
    fn sqlite3_os_file_size(_: *mut sqlite3_file, p_size_1: *mut i64)
    -> i32;
    fn sqlite3_os_lock(_: *mut sqlite3_file, _: i32)
    -> i32;
    fn sqlite3_os_unlock(_: *mut sqlite3_file, _: i32)
    -> i32;
    fn sqlite3_os_check_reserved_lock(id: *mut sqlite3_file,
    p_res_out_1: *mut i32)
    -> i32;
    fn sqlite3_os_file_control(_: *mut sqlite3_file, _: i32, _: *mut ())
    -> i32;
    fn sqlite3_os_file_control_hint(_: *mut sqlite3_file, _: i32, _: *mut ())
    -> ();
    fn sqlite3_os_sector_size(id: *mut sqlite3_file)
    -> i32;
    fn sqlite3_os_device_characteristics(id: *mut sqlite3_file)
    -> i32;
    fn sqlite3_os_shm_map(_: *mut sqlite3_file, _: i32, _: i32, _: i32,
    _: *mut *mut ())
    -> i32;
    fn sqlite3_os_shm_lock(id: *mut sqlite3_file, _: i32, _: i32, _: i32)
    -> i32;
    fn sqlite3_os_shm_barrier(id: *mut sqlite3_file)
    -> ();
    fn sqlite3_os_shm_unmap(id: *mut sqlite3_file, _: i32)
    -> i32;
    fn sqlite3_os_fetch(id: *mut sqlite3_file, _: i64, _: i32,
    _: *mut *mut ())
    -> i32;
    fn sqlite3_os_unfetch(_: *mut sqlite3_file, _: i64, _: *mut ())
    -> i32;
    fn sqlite3_os_open(_: *mut sqlite3_vfs, _: *const i8,
    _: *mut sqlite3_file, _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_os_delete(_: *mut sqlite3_vfs, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_os_access(_: *mut sqlite3_vfs, _: *const i8, _: i32,
    p_res_out_1: *mut i32)
    -> i32;
    fn sqlite3_os_full_pathname(_: *mut sqlite3_vfs, _: *const i8, _: i32,
    _: *mut i8)
    -> i32;
    fn sqlite3_os_dl_open(_: *mut sqlite3_vfs, _: *const i8)
    -> *mut ();
    fn sqlite3_os_dl_error(_: *mut sqlite3_vfs, _: i32, _: *mut i8)
    -> ();
    fn sqlite3_os_dl_sym(_: *mut sqlite3_vfs, _: *mut (), _: *const i8)
    -> unsafe extern "C" fn() -> ();
    fn sqlite3_os_dl_close(_: *mut sqlite3_vfs, _: *mut ())
    -> ();
    fn sqlite3_os_randomness(_: *mut sqlite3_vfs, _: i32, _: *mut i8)
    -> i32;
    fn sqlite3_os_sleep(_: *mut sqlite3_vfs, _: i32)
    -> i32;
    fn sqlite3_os_get_last_error(_: *mut sqlite3_vfs)
    -> i32;
    fn sqlite3_os_current_time_int64(_: *mut sqlite3_vfs,
    _: *mut sqlite3_int64)
    -> i32;
    fn sqlite3_os_open_malloc(_: *mut sqlite3_vfs, _: *const i8,
    _: *mut *mut sqlite3_file, _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_os_close_free(_: *mut sqlite3_file)
    -> ();
    fn sqlite3_pager_open(_: *mut sqlite3_vfs, pp_pager_1: *mut *mut Pager,
    _: *const i8, _: i32, _: i32, _: i32,
    _: Option<unsafe extern "C" fn(*mut PgHdr) -> ()>)
    -> i32;
    fn sqlite3_pager_close(p_pager_1: *mut Pager, _: *mut sqlite3)
    -> i32;
    fn sqlite3_pager_read_fileheader(_: *mut Pager, _: i32, _: *mut u8)
    -> i32;
    fn sqlite3_pager_set_busy_handler(_: *mut Pager,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> ();
    fn sqlite3_pager_set_pagesize(_: *mut Pager, _: *mut u32, _: i32)
    -> i32;
    fn sqlite3_pager_max_page_count(_: *mut Pager, _: Pgno)
    -> Pgno;
    fn sqlite3_pager_set_cachesize(_: *mut Pager, _: i32)
    -> ();
    fn sqlite3_pager_set_spillsize(_: *mut Pager, _: i32)
    -> i32;
    fn sqlite3_pager_set_mmap_limit(_: *mut Pager, _: sqlite3_int64)
    -> ();
    fn sqlite3_pager_shrink(_: *mut Pager)
    -> ();
    fn sqlite3_pager_set_flags(_: *mut Pager, _: u32)
    -> ();
    fn sqlite3_pager_locking_mode(_: *mut Pager, _: i32)
    -> i32;
    fn sqlite3_pager_set_journal_mode(_: *mut Pager, _: i32)
    -> i32;
    fn sqlite3_pager_get_journal_mode(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_ok_to_change_journal_mode(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_journal_size_limit(_: *mut Pager, _: i64)
    -> i64;
    fn sqlite3_pager_backup_ptr(_: *mut Pager)
    -> *mut *mut sqlite3_backup;
    fn sqlite3_pager_flush(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_get(p_pager_1: *mut Pager, pgno: Pgno,
    pp_page_1: *mut *mut DbPage, clr_flag_1: i32)
    -> i32;
    fn sqlite3_pager_lookup(p_pager_1: *mut Pager, pgno: Pgno)
    -> *mut DbPage;
    fn sqlite3_pager_ref(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_unref(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_unref_not_null(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_unref_page_one(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_write(_: *mut DbPage)
    -> i32;
    fn sqlite3_pager_dont_write(_: *mut DbPage)
    -> ();
    fn sqlite3_pager_movepage(_: *mut Pager, _: *mut DbPage, _: Pgno, _: i32)
    -> i32;
    fn sqlite3_pager_page_refcount(_: *mut DbPage)
    -> i32;
    fn sqlite3_pager_get_data(_: *mut DbPage)
    -> *mut ();
    fn sqlite3_pager_get_extra(_: *mut DbPage)
    -> *mut ();
    fn sqlite3_pager_pagecount(_: *mut Pager, _: *mut i32)
    -> ();
    fn sqlite3_pager_begin(_: *mut Pager, ex_flag_1: i32, _: i32)
    -> i32;
    fn sqlite3_pager_commit_phase_one(_: *mut Pager, z_super_1: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_pager_exclusive_lock(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_sync(p_pager_1: *mut Pager, z_super_1: *const i8)
    -> i32;
    fn sqlite3_pager_commit_phase_two(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_rollback(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_open_savepoint(p_pager_1: *mut Pager, n: i32)
    -> i32;
    fn sqlite3_pager_savepoint(p_pager_1: *mut Pager, op: i32,
    i_savepoint_1: i32)
    -> i32;
    fn sqlite3_pager_shared_lock(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_pager_checkpoint(p_pager_1: *mut Pager, _: *mut sqlite3,
    _: i32, _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_pager_wal_supported(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_pager_wal_callback(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_pager_open_wal(p_pager_1: *mut Pager, pis_open_1: *mut i32)
    -> i32;
    fn sqlite3_pager_close_wal(p_pager_1: *mut Pager, _: *mut sqlite3)
    -> i32;
    fn sqlite3_pager_direct_read_ok(p_pager_1: *mut Pager, pgno: Pgno)
    -> i32;
    fn sqlite3_pager_isreadonly(_: *mut Pager)
    -> u8;
    fn sqlite3_pager_data_version(_: *mut Pager)
    -> u32;
    fn sqlite3_pager_mem_used(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_filename(_: *const Pager, _: i32)
    -> *const i8;
    fn sqlite3_pager_vfs(_: *mut Pager)
    -> *mut sqlite3_vfs;
    fn sqlite3_pager_file(_: *mut Pager)
    -> *mut sqlite3_file;
    fn sqlite3_pager_jrnl_file(_: *mut Pager)
    -> *mut sqlite3_file;
    fn sqlite3_pager_journalname(_: *mut Pager)
    -> *const i8;
    fn sqlite3_pager_temp_space(_: *mut Pager)
    -> *mut ();
    fn sqlite3_pager_is_memdb(_: *mut Pager)
    -> i32;
    fn sqlite3_pager_cache_stat(_: *mut Pager, _: i32, _: i32, _: *mut u64)
    -> ();
    fn sqlite3_pager_clear_cache(_: *mut Pager)
    -> ();
    fn sqlite3_sector_size(_: *mut sqlite3_file)
    -> i32;
    fn sqlite3_pager_truncate_image(_: *mut Pager, _: Pgno)
    -> ();
    fn sqlite3_pager_rekey(_: *mut DbPage, _: Pgno, _: u16)
    -> ();
    fn sqlite3_temp_in_memory(_: *const sqlite3)
    -> i32;
    fn sqlite3_malloc_zero(_: u64)
    -> *mut ();
    fn sqlite3_strlen30(_: *const i8)
    -> i32;
    fn sqlite3Malloc(_: u64)
    -> *mut ();
    fn sqlite3MutexAlloc(_: i32)
    -> *mut sqlite3_mutex;
    fn sqlite3_btree_enter(_: *mut Btree)
    -> ();
    fn sqlite3_db_malloc_zero(_: *mut sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_btree_leave(_: *mut Btree)
    -> ();
    fn sqlite3_corrupt_error(_: i32)
    -> i32;
    fn sqlite3_get_varint(_: *const u8, _: *mut u64)
    -> u8;
    fn sqlite3_invoke_busy_handler(_: *mut BusyHandler)
    -> i32;
    fn sqlite3_get4byte(_: *const u8)
    -> u32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_malloc_size(_: *const ())
    -> i32;
    fn sqlite3_fault_sim(_: i32)
    -> i32;
    fn sqlite3Realloc(_: *mut (), _: u64)
    -> *mut ();
    static mut sqlite3_pending_byte: i32;
    fn sqlite3_bitvec_destroy(_: *mut Bitvec)
    -> ();
    fn sqlite3_db_free(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_page_free(_: *mut ())
    -> ();
    fn sqlite3_writable_schema(_: *mut sqlite3)
    -> i32;
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn sqlite3_put4byte(_: *mut u8, _: u32)
    -> ();
    fn sqlite3_abs_int32(_: i32)
    -> i32;
    fn sqlite3_bitvec_size(_: *mut Bitvec)
    -> u32;
    fn sqlite3_bitvec_test_not_null(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_btree_copy_file(_: *mut Btree, _: *mut Btree)
    -> i32;
    fn sqlite3_bitvec_create(_: u32)
    -> *mut Bitvec;
    fn sqlite3_bitvec_set(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_page_malloc(_: i32)
    -> *mut ();
    fn sqlite3_vdbe_alloc_unpacked_record(_: *mut KeyInfo)
    -> *mut UnpackedRecord;
    fn sqlite3_vdbe_record_unpack(_: i32, _: *const (),
    _: *mut UnpackedRecord)
    -> ();
    fn sqlite3_vdbe_find_compare(_: *mut UnpackedRecord)
    -> unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord) -> i32;
    fn sqlite3_vdbe_record_compare(_: i32, _: *const (),
    _: *mut UnpackedRecord)
    -> i32;
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw(_: *mut sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_put_varint(_: *mut u8, _: u64)
    -> i32;
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut sqlite3, _: *mut i8,
    _: i32, _: i32)
    -> ();
    fn sqlite3_mem_set_array_int64(a_mem_1: *mut sqlite3_value, i_idx_1: i32,
    val: i64)
    -> ();
    fn sqlite3_str_accum_finish(_: *mut StrAccum)
    -> *mut i8;
    fn sqlite3_btree_enter_all(_: *mut sqlite3)
    -> ();
    fn sqlite3_btree_enter_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_leave_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_leave_all(_: *mut sqlite3)
    -> ();
    fn sqlite3_vdbe_create(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_vdbe_parser(_: *mut Vdbe)
    -> *mut Parse;
    fn sqlite3_vdbe_add_op0(_: *mut Vdbe, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op1(_: *mut Vdbe, _: i32, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op2(_: *mut Vdbe, _: i32, _: i32, _: i32)
    -> i32;
    fn sqlite3_vdbe_goto(_: *mut Vdbe, _: i32)
    -> i32;
    fn sqlite3_vdbe_load_string(_: *mut Vdbe, _: i32, _: *const i8)
    -> i32;
    fn sqlite3_vdbe_multi_load(_: *mut Vdbe, _: i32, _: *const i8, ...)
    -> ();
    fn sqlite3_vdbe_add_op3(_: *mut Vdbe, _: i32, _: i32, _: i32, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op4(_: *mut Vdbe, _: i32, _: i32, _: i32, _: i32,
    z_p4_1: *const i8, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op4_dup8(_: *mut Vdbe, _: i32, _: i32, _: i32, _: i32,
    _: *const u8, _: i32)
    -> i32;
    fn sqlite3_vdbe_add_op4_int(_: *mut Vdbe, _: i32, _: i32, _: i32, _: i32,
    _: i32)
    -> i32;
    fn sqlite3_vdbe_add_function_call(_: *mut Parse, _: i32, _: i32, _: i32,
    _: i32, _: *const FuncDef, _: i32)
    -> i32;
    fn sqlite3_vdbe_end_coroutine(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_add_op_list(_: *mut Vdbe, n_op_1: i32,
    a_op_1: *const VdbeOpList, i_lineno_1: i32)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_explain(_: *mut Parse, _: u8, _: *const i8, ...)
    -> i32;
    fn sqlite3_vdbe_explain_pop(_: *mut Parse)
    -> ();
    fn sqlite3_vdbe_explain_parent(_: *mut Parse)
    -> i32;
    fn sqlite3_vdbe_add_parse_schema_op(_: *mut Vdbe, _: i32, _: *mut i8,
    _: u16)
    -> ();
    fn sqlite3_vdbe_change_opcode(_: *mut Vdbe, addr: i32, _: u8)
    -> ();
    fn sqlite3_vdbe_change_p1(_: *mut Vdbe, addr: i32, p1_1: i32)
    -> ();
    fn sqlite3_vdbe_change_p2(_: *mut Vdbe, addr: i32, p2_1: i32)
    -> ();
    fn sqlite3_vdbe_change_p3(_: *mut Vdbe, addr: i32, p3_1: i32)
    -> ();
    fn sqlite3_vdbe_change_p5(_: *mut Vdbe, p5_1: u16)
    -> ();
    fn sqlite3_vdbe_typeof_column(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_jump_here(_: *mut Vdbe, addr: i32)
    -> ();
    fn sqlite3_vdbe_jump_here_or_pop_inst(_: *mut Vdbe, addr: i32)
    -> ();
    fn sqlite3_vdbe_change_to_noop(_: *mut Vdbe, addr: i32)
    -> i32;
    fn sqlite3_vdbe_delete_prior_opcode(_: *mut Vdbe, op: u8)
    -> i32;
    fn sqlite3_vdbe_change_p4(_: *mut Vdbe, addr: i32, z_p4_1: *const i8,
    n_1: i32)
    -> ();
    fn sqlite3_vdbe_append_p4(_: *mut Vdbe, p_p4_1: *mut (), p4type: i32)
    -> ();
    fn sqlite3_vdbe_set_p4_key_info(_: *mut Parse, _: *mut Index)
    -> ();
    fn sqlite3_vdbe_uses_btree(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_get_op(_: *mut Vdbe, _: i32)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_get_last_op(_: *mut Vdbe)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_make_label(_: *mut Parse)
    -> i32;
    fn sqlite3_vdbe_run_only_once(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_reusable(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_delete(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_make_ready(_: *mut Vdbe, _: *mut Parse)
    -> ();
    fn sqlite3_vdbe_finalize(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_resolve_label(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_current_addr(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_reset_step_result(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_rewind(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_reset(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_set_num_cols(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_set_col_name(_: *mut Vdbe, _: i32, _: i32, _: *const i8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_count_changes(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_db(_: *mut Vdbe)
    -> *mut sqlite3;
    fn sqlite3_vdbe_prepare_flags(_: *mut Vdbe)
    -> u8;
    fn sqlite3_vdbe_set_sql(_: *mut Vdbe, z: *const i8, n: i32, _: u8)
    -> ();
    fn sqlite3_vdbe_swap(_: *mut Vdbe, _: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_take_op_array(_: *mut Vdbe, _: *mut i32, _: *mut i32)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_get_bound_value(_: *mut Vdbe, _: i32, _: u8)
    -> *mut sqlite3_value;
    fn sqlite3_vdbe_set_varmask(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_expand_sql(_: *mut Vdbe, _: *const i8)
    -> *mut i8;
    fn sqlite3_mem_compare(_: *const Mem, _: *const Mem, _: *const CollSeq)
    -> i32;
    fn sqlite3_blob_compare(_: *const Mem, _: *const Mem)
    -> i32;
    fn sqlite3_vdbe_func_name(_: *const sqlite3_context)
    -> *const i8;
    fn sqlite3_vdbe_record_compare_with_skip(_: i32, _: *const (),
    _: *mut UnpackedRecord, _: i32)
    -> i32;
    fn sqlite3_vdbe_link_sub_program(_: *mut Vdbe, _: *mut SubProgram)
    -> ();
    fn sqlite3_vdbe_has_sub_program(_: *mut Vdbe)
    -> i32;
    fn sqlite3_not_pure_func(_: *mut sqlite3_context)
    -> i32;
    fn sqlite3_pcache_initialize()
    -> i32;
    fn sqlite3_pcache_shutdown()
    -> ();
    fn sqlite3_p_cache_buffer_setup(_: *mut (), sz: i32, n: i32)
    -> ();
    fn sqlite3_pcache_open(sz_page_1: i32, sz_extra_1: i32,
    b_purgeable_1: i32,
    x_stress_1: Option<unsafe extern "C" fn(*mut (), *mut PgHdr) -> i32>,
    p_stress_1: *mut (), p_to_init_1: *mut PCache)
    -> i32;
    fn sqlite3_pcache_set_page_size(_: *mut PCache, _: i32)
    -> i32;
    fn sqlite3_pcache_size()
    -> i32;
    fn sqlite3_pcache_fetch(_: *mut PCache, _: Pgno, create_flag_1: i32)
    -> *mut sqlite3_pcache_page;
    fn sqlite3_pcache_fetch_stress(_: *mut PCache, _: Pgno,
    _: *mut *mut sqlite3_pcache_page)
    -> i32;
    fn sqlite3_pcache_fetch_finish(_: *mut PCache, _: Pgno,
    p_page_1: *mut sqlite3_pcache_page)
    -> *mut PgHdr;
    fn sqlite3_pcache_release(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_drop(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_make_dirty(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_make_clean(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_clean_all(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_clear_writable(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_move(_: *mut PgHdr, _: Pgno)
    -> ();
    fn sqlite3_pcache_truncate(_: *mut PCache, x: Pgno)
    -> ();
    fn sqlite3_pcache_dirty_list(_: *mut PCache)
    -> *mut PgHdr;
    fn sqlite3_pcache_close(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_clear_sync_flags(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_clear(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_ref_count(_: *mut PCache)
    -> i64;
    fn sqlite3_pcache_ref(_: *mut PgHdr)
    -> ();
    fn sqlite3_pcache_page_refcount(_: *mut PgHdr)
    -> i64;
    fn sqlite3_pcache_pagecount(_: *mut PCache)
    -> i32;
    fn sqlite3_pcache_set_cachesize(_: *mut PCache, _: i32)
    -> ();
    fn sqlite3_pcache_set_spillsize(_: *mut PCache, _: i32)
    -> i32;
    fn sqlite3_pcache_shrink(_: *mut PCache)
    -> ();
    fn sqlite3_p_cache_set_default()
    -> ();
    fn sqlite3_header_size_pcache()
    -> i32;
    fn sqlite3_header_size_pcache1()
    -> i32;
    fn sqlite3_p_cache_percent_dirty(_: *mut PCache)
    -> i32;
    fn sqlite3_p_cache_is_dirty(p_cache_1: *mut PCache)
    -> i32;
    fn sqlite3_walk_expr(_: *mut Walker, _: *mut Expr)
    -> i32;
    fn sqlite3_walk_expr_nn(_: *mut Walker, _: *mut Expr)
    -> i32;
    fn sqlite3_walk_expr_list(_: *mut Walker, _: *mut ExprList)
    -> i32;
    fn sqlite3_walk_select(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_walk_select_expr(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_walk_select_from(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_expr_walk_noop(_: *mut Walker, _: *mut Expr)
    -> i32;
    fn sqlite3_select_walk_noop(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_select_walk_fail(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_walker_depth_increase(_: *mut Walker, _: *mut Select)
    -> i32;
    fn sqlite3_walker_depth_decrease(_: *mut Walker, _: *mut Select)
    -> ();
    fn sqlite3_walk_win_defn_dummy_callback(_: *mut Walker, _: *mut Select)
    -> ();
    fn sqlite3_select_pop_with(_: *mut Walker, _: *mut Select)
    -> ();
    fn sqlite3_multi_values(p_parse_1: *mut Parse, p_left_1: *mut Select,
    p_row_1: *mut ExprList)
    -> *mut Select;
    fn sqlite3_multi_values_end(p_parse_1: *mut Parse, p_val_1: *mut Select)
    -> ();
    fn sqlite3_window_delete(_: *mut sqlite3, _: *mut Window)
    -> ();
    fn sqlite3_window_unlink_from_select(_: *mut Window)
    -> ();
    fn sqlite3_window_list_delete(db: *mut sqlite3, p: *mut Window)
    -> ();
    fn sqlite3_window_alloc(_: *mut Parse, _: i32, _: i32, _: *mut Expr,
    _: i32, _: *mut Expr, _: u8)
    -> *mut Window;
    fn sqlite3_window_attach(_: *mut Parse, _: *mut Expr, _: *mut Window)
    -> ();
    fn sqlite3_window_link(p_sel_1: *mut Select, p_win_1: *mut Window)
    -> ();
    fn sqlite3_window_compare(_: *const Parse, _: *const Window,
    _: *const Window, _: i32)
    -> i32;
    fn sqlite3_window_code_init(_: *mut Parse, _: *mut Select)
    -> ();
    fn sqlite3_window_code_step(_: *mut Parse, _: *mut Select,
    _: *mut WhereInfo, _: i32, _: i32)
    -> ();
    fn sqlite3_window_rewrite(_: *mut Parse, _: *mut Select)
    -> i32;
    fn sqlite3_window_update(_: *mut Parse, _: *mut Window, _: *mut Window,
    _: *mut FuncDef)
    -> ();
    fn sqlite3_window_dup(db: *mut sqlite3, p_owner_1: *mut Expr,
    p: *mut Window)
    -> *mut Window;
    fn sqlite3_window_list_dup(db: *mut sqlite3, p: *mut Window)
    -> *mut Window;
    fn sqlite3_window_functions()
    -> ();
    fn sqlite3_window_chain(_: *mut Parse, _: *mut Window, _: *mut Window)
    -> ();
    fn sqlite3_window_assemble(_: *mut Parse, _: *mut Window,
    _: *mut ExprList, _: *mut ExprList, _: *mut Token)
    -> *mut Window;
    fn sqlite3_report_error(i_err_1: i32, lineno: i32, z_type_1: *const i8)
    -> i32;
    fn sqlite3_misuse_error(_: i32)
    -> i32;
    fn sqlite3_cantopen_error(_: i32)
    -> i32;
    fn sqlite3_is_id_char(_: u8)
    -> i32;
    fn sqlite3_str_i_cmp(_: *const i8, _: *const i8)
    -> i32;
    fn sqlite3ColumnType(_: *mut Column, _: *mut i8)
    -> *mut i8;
    fn sqlite3_malloc_init()
    -> i32;
    fn sqlite3_malloc_end()
    -> ();
    fn sqlite3_db_malloc_raw_nn(_: *mut sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_str_dup(_: *mut sqlite3, _: *const i8)
    -> *mut i8;
    fn sqlite3_db_str_n_dup(_: *mut sqlite3, _: *const i8, _: u64)
    -> *mut i8;
    fn sqlite3_db_span_dup(_: *mut sqlite3, _: *const i8, _: *const i8)
    -> *mut i8;
    fn sqlite3_db_realloc_or_free(_: *mut sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_realloc(_: *mut sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_free_nn(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_nn_free_nn(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_malloc_size(_: *mut sqlite3, _: *const ())
    -> i32;
    fn sqlite3_mem_set_default()
    -> ();
    fn sqlite3_benign_malloc_hooks(_: Option<unsafe extern "C" fn() -> ()>,
    _: Option<unsafe extern "C" fn() -> ()>)
    -> ();
    fn sqlite3_heap_nearly_full()
    -> i32;
    fn sqlite3_default_mutex()
    -> *const sqlite3_mutex_methods;
    fn sqlite3_noop_mutex()
    -> *const sqlite3_mutex_methods;
    fn sqlite3_mutex_init()
    -> i32;
    fn sqlite3_mutex_end()
    -> i32;
    fn sqlite3_memory_barrier()
    -> ();
    fn sqlite3_status_value(_: i32)
    -> sqlite3_int64;
    fn sqlite3_status_up(_: i32, _: i32)
    -> ();
    fn sqlite3_status_down(_: i32, _: i32)
    -> ();
    fn sqlite3_status_highwater(_: i32, _: i32)
    -> ();
    fn sqlite3_lookaside_used(_: *mut sqlite3, _: *mut i32)
    -> i32;
    fn sqlite3_pcache1_mutex()
    -> *mut sqlite3_mutex;
    fn sqlite3_malloc_mutex()
    -> *mut sqlite3_mutex;
    fn sqlite3_is_na_n(_: f64)
    -> i32;
    fn sqlite3_is_overflow(_: f64)
    -> i32;
    fn sqlite3_fp_decode(_: *mut FpDecode, _: f64, _: i32, _: i32)
    -> ();
    fn sqlite3_m_printf(_: *mut sqlite3, _: *const i8, ...)
    -> *mut i8;
    fn sqlite3_vm_printf(_: *mut sqlite3, _: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_set_string(_: *mut *mut i8, _: *mut sqlite3, _: *const i8)
    -> ();
    fn sqlite3_progress_check(_: *mut Parse)
    -> ();
    fn sqlite3_error_msg(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_error_to_parser(_: *mut sqlite3, _: i32)
    -> i32;
    fn sqlite3_dequote(_: *mut i8)
    -> ();
    fn sqlite3_dequote_expr(_: *mut Expr)
    -> ();
    fn sqlite3_dequote_token(_: *mut Token)
    -> ();
    fn sqlite3_dequote_number(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_token_init(_: *mut Token, _: *mut i8)
    -> ();
    fn sqlite3_keyword_code(_: *const u8, _: i32)
    -> i32;
    fn sqlite3_run_parser(_: *mut Parse, _: *const i8)
    -> i32;
    fn sqlite3_finish_coding(_: *mut Parse)
    -> ();
    fn sqlite3_get_temp_reg(_: *mut Parse)
    -> i32;
    fn sqlite3_release_temp_reg(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_get_temp_range(_: *mut Parse, _: i32)
    -> i32;
    fn sqlite3_release_temp_range(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_clear_temp_reg_cache(_: *mut Parse)
    -> ();
    fn sqlite3_touch_register(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_expr_alloc(_: *mut sqlite3, _: i32, _: *const Token, _: i32)
    -> *mut Expr;
    fn sqlite3_expr(_: *mut sqlite3, _: i32, _: *const i8)
    -> *mut Expr;
    fn sqlite3_expr_int32(_: *mut sqlite3, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_attach_subtrees(_: *mut sqlite3, _: *mut Expr,
    _: *mut Expr, _: *mut Expr)
    -> ();
    fn sqlite3_p_expr(_: *mut Parse, _: i32, _: *mut Expr, _: *mut Expr)
    -> *mut Expr;
    fn sqlite3_p_expr_add_select(_: *mut Parse, _: *mut Expr, _: *mut Select)
    -> ();
    fn sqlite3_expr_and(_: *mut Parse, _: *mut Expr, _: *mut Expr)
    -> *mut Expr;
    fn sqlite3_expr_simplified_and_or(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_expr_function(_: *mut Parse, _: *mut ExprList, _: *const Token,
    _: i32)
    -> *mut Expr;
    fn sqlite3_expr_add_function_order_by(_: *mut Parse, _: *mut Expr,
    _: *mut ExprList)
    -> ();
    fn sqlite3_expr_order_by_aggregate_error(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_expr_function_usable(_: *mut Parse, _: *const Expr,
    _: *const FuncDef)
    -> ();
    fn sqlite3_expr_assign_var_number(_: *mut Parse, _: *mut Expr, _: u32)
    -> ();
    fn sqlite3_expr_delete(_: *mut sqlite3, _: *mut Expr)
    -> ();
    fn sqlite3_expr_delete_generic(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_expr_deferred_delete(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_expr_unmap_and_delete(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_expr_list_append(_: *mut Parse, _: *mut ExprList, _: *mut Expr)
    -> *mut ExprList;
    fn sqlite3_expr_list_append_vector(_: *mut Parse, _: *mut ExprList,
    _: *mut IdList, _: *mut Expr)
    -> *mut ExprList;
    fn sqlite3_expr_list_to_values(_: *mut Parse, _: i32, _: *mut ExprList)
    -> *mut Select;
    fn sqlite3_expr_list_set_sort_order(_: *mut ExprList, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_list_set_name(_: *mut Parse, _: *mut ExprList,
    _: *const Token, _: i32)
    -> ();
    fn sqlite3_expr_list_set_span(_: *mut Parse, _: *mut ExprList,
    _: *const i8, _: *const i8)
    -> ();
    fn sqlite3_expr_list_delete(_: *mut sqlite3, _: *mut ExprList)
    -> ();
    fn sqlite3_expr_list_delete_generic(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_expr_list_flags(_: *const ExprList)
    -> u32;
    fn sqlite3_index_has_duplicate_root_page(_: *mut Index)
    -> i32;
    fn sqlite3_init(_: *mut sqlite3, _: *mut *mut i8)
    -> i32;
    fn sqlite3_init_callback(_: *mut (), _: i32, _: *mut *mut i8,
    _: *mut *mut i8)
    -> i32;
    fn sqlite3_init_one(_: *mut sqlite3, _: i32, _: *mut *mut i8, _: u32)
    -> i32;
    fn sqlite3_pragma(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: i32)
    -> ();
    fn sqlite3_pragma_vtab_register(_: *mut sqlite3, z_name_1: *const i8)
    -> *mut Module;
    fn sqlite3_reset_all_schemas_of_connection(_: *mut sqlite3)
    -> ();
    fn sqlite3_reset_one_schema(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_collapse_database_array(_: *mut sqlite3)
    -> ();
    fn sqlite3_commit_internal_changes(_: *mut sqlite3)
    -> ();
    fn sqlite3_column_set_expr(_: *mut Parse, _: *mut Table, _: *mut Column,
    _: *mut Expr)
    -> ();
    fn sqlite3_column_expr(_: *mut Table, _: *mut Column)
    -> *mut Expr;
    fn sqlite3_column_set_coll(_: *mut sqlite3, _: *mut Column,
    z_coll_1: *const i8)
    -> ();
    fn sqlite3_column_coll(_: *mut Column)
    -> *const i8;
    fn sqlite3_delete_column_names(_: *mut sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_generate_column_names(p_parse_1: *mut Parse,
    p_select_1: *mut Select)
    -> ();
    fn sqlite3_columns_from_expr_list(_: *mut Parse, _: *mut ExprList,
    _: *mut i16, _: *mut *mut Column)
    -> i32;
    fn sqlite3_subquery_column_types(_: *mut Parse, _: *mut Table,
    _: *mut Select, _: i8)
    -> ();
    fn sqlite3_result_set_of_select(_: *mut Parse, _: *mut Select, _: i8)
    -> *mut Table;
    fn sqlite3_open_schema_table(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_primary_key_index(_: *mut Table)
    -> *mut Index;
    fn sqlite3_table_column_to_index(_: *mut Index, _: i32)
    -> i32;
    fn sqlite3_table_column_to_storage(_: *mut Table, _: i16)
    -> i16;
    fn sqlite3_storage_column_to_table(_: *mut Table, _: i16)
    -> i16;
    fn sqlite3_start_table(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: i32, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_add_column(_: *mut Parse, _: Token, _: Token)
    -> ();
    fn sqlite3_add_not_null(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_add_primary_key(_: *mut Parse, _: *mut ExprList, _: i32,
    _: i32, _: i32)
    -> ();
    fn sqlite3_add_check_constraint(_: *mut Parse, _: *mut Expr, _: *const i8,
    _: *const i8)
    -> ();
    fn sqlite3_add_default_value(_: *mut Parse, _: *mut Expr, _: *const i8,
    _: *const i8)
    -> ();
    fn sqlite3_add_collate_type(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_add_generated(_: *mut Parse, _: *mut Expr, _: *mut Token)
    -> ();
    fn sqlite3_end_table(_: *mut Parse, _: *mut Token, _: *mut Token, _: u32,
    _: *mut Select)
    -> ();
    fn sqlite3_add_returning(_: *mut Parse, _: *mut ExprList)
    -> ();
    fn sqlite3_parse_uri(_: *const i8, _: *const i8, _: *mut u32,
    _: *mut *mut sqlite3_vfs, _: *mut *mut i8, _: *mut *mut i8)
    -> i32;
    fn sqlite3_db_name_to_btree(_: *mut sqlite3, _: *const i8)
    -> *mut Btree;
    fn sqlite3_bitvec_test(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_bitvec_clear(_: *mut Bitvec, _: u32, _: *mut ())
    -> ();
    fn sqlite3_bitvec_builtin_test(_: i32, _: *mut i32)
    -> i32;
    fn sqlite3_row_set_init(_: *mut sqlite3)
    -> *mut RowSet;
    fn sqlite3_row_set_delete(_: *mut ())
    -> ();
    fn sqlite3_row_set_clear(_: *mut ())
    -> ();
    fn sqlite3_row_set_insert(_: *mut RowSet, _: i64)
    -> ();
    fn sqlite3_row_set_test(_: *mut RowSet, i_batch_1: i32, _: i64)
    -> i32;
    fn sqlite3_row_set_next(_: *mut RowSet, _: *mut i64)
    -> i32;
    fn sqlite3_create_view(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: *mut ExprList, _: *mut Select, _: i32, _: i32)
    -> ();
    fn sqlite3_view_get_column_names(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_drop_table(_: *mut Parse, _: *mut SrcList, _: i32, _: i32)
    -> ();
    fn sqlite3_code_drop_table(_: *mut Parse, _: *mut Table, _: i32, _: i32)
    -> ();
    fn sqlite3_delete_table(_: *mut sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_delete_table_generic(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_free_index(_: *mut sqlite3, _: *mut Index)
    -> ();
    fn sqlite3_autoincrement_begin(p_parse_1: *mut Parse)
    -> ();
    fn sqlite3_autoincrement_end(p_parse_1: *mut Parse)
    -> ();
    fn sqlite3_insert(_: *mut Parse, _: *mut SrcList, _: *mut Select,
    _: *mut IdList, _: i32, _: *mut Upsert)
    -> ();
    fn sqlite3_compute_generated_columns(_: *mut Parse, _: i32, _: *mut Table)
    -> ();
    fn sqlite3_array_allocate(_: *mut sqlite3, _: *mut (), _: i32,
    _: *mut i32, _: *mut i32)
    -> *mut ();
    fn sqlite3_id_list_append(_: *mut Parse, _: *mut IdList, _: *mut Token)
    -> *mut IdList;
    fn sqlite3_id_list_index(_: *mut IdList, _: *const i8)
    -> i32;
    fn sqlite3_src_list_enlarge(_: *mut Parse, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut SrcList;
    fn sqlite3_src_list_append_list(p_parse_1: *mut Parse, p1: *mut SrcList,
    p2: *mut SrcList)
    -> *mut SrcList;
    fn sqlite3_src_list_append(_: *mut Parse, _: *mut SrcList, _: *mut Token,
    _: *mut Token)
    -> *mut SrcList;
    fn sqlite3_subquery_delete(_: *mut sqlite3, _: *mut Subquery)
    -> ();
    fn sqlite3_subquery_detach(_: *mut sqlite3, _: *mut SrcItem)
    -> *mut Select;
    fn sqlite3_src_item_attach_subquery(_: *mut Parse, _: *mut SrcItem,
    _: *mut Select, _: i32)
    -> i32;
    fn sqlite3_src_list_append_from_term(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token, _: *mut Token, _: *mut Select,
    _: *mut OnOrUsing)
    -> *mut SrcList;
    fn sqlite3_src_list_indexed_by(_: *mut Parse, _: *mut SrcList,
    _: *mut Token)
    -> ();
    fn sqlite3_src_list_func_args(_: *mut Parse, _: *mut SrcList,
    _: *mut ExprList)
    -> ();
    fn sqlite3_indexed_by_lookup(_: *mut Parse, _: *mut SrcItem)
    -> i32;
    fn sqlite3_src_list_shift_join_type(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_src_list_assign_cursors(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_id_list_delete(_: *mut sqlite3, _: *mut IdList)
    -> ();
    fn sqlite3_clear_on_or_using(_: *mut sqlite3, _: *mut OnOrUsing)
    -> ();
    fn sqlite3_src_list_delete(_: *mut sqlite3, _: *mut SrcList)
    -> ();
    fn sqlite3_allocate_index_object(_: *mut sqlite3, _: i32, _: i32,
    _: *mut *mut i8)
    -> *mut Index;
    fn sqlite3_create_index(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut SrcList, _: *mut ExprList, _: i32, _: *mut Token, _: *mut Expr,
    _: i32, _: i32, _: u8)
    -> ();
    fn sqlite3_drop_index(_: *mut Parse, _: *mut SrcList, _: i32)
    -> ();
    fn sqlite3_select(_: *mut Parse, _: *mut Select, _: *mut SelectDest)
    -> i32;
    fn sqlite3_select_new(_: *mut Parse, _: *mut ExprList, _: *mut SrcList,
    _: *mut Expr, _: *mut ExprList, _: *mut Expr, _: *mut ExprList, _: u32,
    _: *mut Expr)
    -> *mut Select;
    fn sqlite3_select_delete(_: *mut sqlite3, _: *mut Select)
    -> ();
    fn sqlite3_select_delete_generic(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_select_check_on_clauses(p_parse_1: *mut Parse,
    p_select_1: *mut Select)
    -> ();
    fn sqlite3_src_list_lookup(_: *mut Parse, _: *mut SrcList)
    -> *mut Table;
    fn sqlite3_is_read_only(_: *mut Parse, _: *mut Table, _: *mut Trigger)
    -> i32;
    fn sqlite3_open_table(_: *mut Parse, i_cur_1: i32, i_db_1: i32,
    _: *mut Table, _: i32)
    -> ();
    fn sqlite3_code_change_count(_: *mut Vdbe, _: i32, _: *const i8)
    -> ();
    fn sqlite3_delete_from(_: *mut Parse, _: *mut SrcList, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr)
    -> ();
    fn sqlite3_update(_: *mut Parse, _: *mut SrcList, _: *mut ExprList,
    _: *mut Expr, _: i32, _: *mut ExprList, _: *mut Expr, _: *mut Upsert)
    -> ();
    fn sqlite3_where_begin(_: *mut Parse, _: *mut SrcList, _: *mut Expr,
    _: *mut ExprList, _: *mut ExprList, _: *mut Select, _: u16, _: i32)
    -> *mut WhereInfo;
    fn sqlite3_where_end(_: *mut WhereInfo)
    -> ();
    fn sqlite3_where_output_row_count(_: *mut WhereInfo)
    -> LogEst;
    fn sqlite3_where_is_distinct(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_is_ordered(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_order_by_limit_opt_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_min_max_opt_early_out(_: *mut Vdbe, _: *mut WhereInfo)
    -> ();
    fn sqlite3_where_is_sorted(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_continue_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_break_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_ok_one_pass(_: *mut WhereInfo, _: *mut i32)
    -> i32;
    fn sqlite3_where_uses_deferred_seek(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_expr_code_load_index_column(_: *mut Parse, _: *mut Index,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_code_get_column(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_code_get_column_of_table(_: *mut Vdbe, _: *mut Table,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_code_move(_: *mut Parse, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_to_register(p_expr_1: *mut Expr, i_reg_1: i32)
    -> ();
    fn sqlite3_expr_code(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_generated_column(_: *mut Parse, _: *mut Table,
    _: *mut Column, _: i32)
    -> ();
    fn sqlite3_expr_code_copy(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_factorable(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_run_just_once(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_null_register_range(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_code_temp(_: *mut Parse, _: *mut Expr, _: *mut i32)
    -> i32;
    fn sqlite3_expr_code_target(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_code_expr_list(_: *mut Parse, _: *mut ExprList, _: i32,
    _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_if_true(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_if_false(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_if_false_dup(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_find_table(_: *mut sqlite3, _: *const i8, _: *const i8)
    -> *mut Table;
    fn sqlite3_locate_table(_: *mut Parse, flags: u32, _: *const i8,
    _: *const i8)
    -> *mut Table;
    fn sqlite3_preferred_table_name(_: *const i8)
    -> *const i8;
    fn sqlite3_locate_table_item(_: *mut Parse, flags: u32, _: *mut SrcItem)
    -> *mut Table;
    fn sqlite3_find_index(_: *mut sqlite3, _: *const i8, _: *const i8)
    -> *mut Index;
    fn sqlite3_unlink_and_delete_table(_: *mut sqlite3, _: i32, _: *const i8)
    -> ();
    fn sqlite3_unlink_and_delete_index(_: *mut sqlite3, _: i32, _: *const i8)
    -> ();
    fn sqlite3_vacuum(_: *mut Parse, _: *mut Token, _: *mut Expr)
    -> ();
    fn sqlite3_run_vacuum(_: *mut *mut i8, _: *mut sqlite3, _: i32,
    _: *mut sqlite3_value)
    -> i32;
    fn sqlite3_name_from_token(_: *mut sqlite3, _: *const Token)
    -> *mut i8;
    fn sqlite3_expr_compare(_: *const Parse, _: *const Expr, _: *const Expr,
    _: i32)
    -> i32;
    fn sqlite3_expr_compare_skip(_: *mut Expr, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_list_compare(_: *const ExprList, _: *const ExprList,
    _: i32)
    -> i32;
    fn sqlite3_expr_implies_expr(_: *const Parse, _: *const Expr,
    _: *const Expr, _: i32)
    -> i32;
    fn sqlite3_expr_implies_non_null_row(_: *mut Expr, _: i32, _: i32)
    -> i32;
    fn sqlite3_agg_info_persist_walker_init(_: *mut Walker, _: *mut Parse)
    -> ();
    fn sqlite3_expr_analyze_aggregates(_: *mut NameContext, _: *mut Expr)
    -> ();
    fn sqlite3_expr_analyze_agg_list(_: *mut NameContext, _: *mut ExprList)
    -> ();
    fn sqlite3_expr_covered_by_index(_: *mut Expr, i_cur_1: i32,
    p_idx_1: *mut Index)
    -> i32;
    fn sqlite3_references_src_list(_: *mut Parse, _: *mut Expr,
    _: *mut SrcList)
    -> i32;
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_prng_save_state()
    -> ();
    fn sqlite3_prng_restore_state()
    -> ();
    fn sqlite3_rollback_all(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_code_verify_schema(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_code_verify_named_schema(_: *mut Parse, z_db_1: *const i8)
    -> ();
    fn sqlite3_begin_transaction(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_end_transaction(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_savepoint(_: *mut Parse, _: i32, _: *mut Token)
    -> ();
    fn sqlite3_close_savepoints(_: *mut sqlite3)
    -> ();
    fn sqlite3_leave_mutex_and_close_zombie(_: *mut sqlite3)
    -> ();
    fn sqlite3_is_true_or_false(_: *const i8)
    -> u32;
    fn sqlite3_expr_id_to_true_false(_: *mut Expr)
    -> i32;
    fn sqlite3_expr_truth_value(_: *const Expr)
    -> i32;
    fn sqlite3_expr_is_constant(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_expr_is_constant_or_function(_: *mut Expr, _: u8)
    -> i32;
    fn sqlite3_expr_is_constant_or_group_by(_: *mut Parse, _: *mut Expr,
    _: *mut ExprList)
    -> i32;
    fn sqlite3_expr_is_single_table_constraint(_: *mut Expr,
    _: *const SrcList, _: i32, _: i32)
    -> i32;
    fn sqlite3_expr_is_integer(_: *const Expr, _: *mut i32, _: *mut Parse)
    -> i32;
    fn sqlite3_expr_can_be_null(_: *const Expr)
    -> i32;
    fn sqlite3_expr_needs_no_affinity_change(_: *const Expr, _: i8)
    -> i32;
    fn sqlite3_expr_is_like_operator(_: *const Expr)
    -> i32;
    fn sqlite3_is_rowid(_: *const i8)
    -> i32;
    fn sqlite3_rowid_alias(p_tab_1: *mut Table)
    -> *const i8;
    fn sqlite3_generate_row_delete(_: *mut Parse, _: *mut Table,
    _: *mut Trigger, _: i32, _: i32, _: i32, _: i16, _: u8, _: u8, _: u8,
    _: i32)
    -> ();
    fn sqlite3_generate_row_index_delete(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: *mut i32, _: i32)
    -> ();
    fn sqlite3_generate_index_key(_: *mut Parse, _: *mut Index, _: i32,
    _: i32, _: i32, _: *mut i32, _: *mut Index, _: i32)
    -> i32;
    fn sqlite3_resolve_part_idx_label(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_expr_references_updated_column(_: *mut Expr, _: *mut i32,
    _: i32)
    -> i32;
    fn sqlite3_generate_constraint_checks(_: *mut Parse, _: *mut Table,
    _: *mut i32, _: i32, _: i32, _: i32, _: i32, _: u8, _: u8, _: i32,
    _: *mut i32, _: *mut i32, _: *mut Upsert)
    -> ();
    fn sqlite3_complete_insertion(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: i32, _: *mut i32, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_open_table_and_indices(_: *mut Parse, _: *mut Table, _: i32,
    _: u8, _: i32, _: *mut u8, _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_begin_write_operation(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_multi_write(_: *mut Parse)
    -> ();
    fn sqlite3_may_abort(_: *mut Parse)
    -> ();
    fn sqlite3_halt_constraint(_: *mut Parse, _: i32, _: i32, _: *mut i8,
    _: i8, _: u8)
    -> ();
    fn sqlite3_unique_constraint(_: *mut Parse, _: i32, _: *mut Index)
    -> ();
    fn sqlite3_rowid_constraint(_: *mut Parse, _: i32, _: *mut Table)
    -> ();
    fn sqlite3_expr_dup(_: *mut sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_list_dup(_: *mut sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_src_list_dup(_: *mut sqlite3, _: *const SrcList, _: i32)
    -> *mut SrcList;
    fn sqlite3_id_list_dup(_: *mut sqlite3, _: *const IdList)
    -> *mut IdList;
    fn sqlite3_select_dup(_: *mut sqlite3, _: *const Select, _: i32)
    -> *mut Select;
    fn sqlite3_function_search(_: i32, _: *const i8)
    -> *mut FuncDef;
    fn sqlite3_insert_builtin_funcs(_: *mut FuncDef, _: i32)
    -> ();
    fn sqlite3_find_function(_: *mut sqlite3, _: *const i8, _: i32, _: u8,
    _: u8)
    -> *mut FuncDef;
    fn sqlite3_quote_value(_: *mut StrAccum, _: *mut sqlite3_value, _: i32)
    -> ();
    fn sqlite3_append_one_utf8_character(_: *mut i8, _: u32)
    -> i32;
    fn sqlite3_register_builtin_functions()
    -> ();
    fn sqlite3_register_date_time_functions()
    -> ();
    fn sqlite3_register_json_functions()
    -> ();
    fn sqlite3_register_per_connection_builtin_functions(_: *mut sqlite3)
    -> ();
    fn sqlite3_json_vtab_register(_: *mut sqlite3, _: *const i8)
    -> *mut Module;
    fn sqlite3_safety_check_ok(_: *mut sqlite3)
    -> i32;
    fn sqlite3_safety_check_sick_or_ok(_: *mut sqlite3)
    -> i32;
    fn sqlite3_change_cookie(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_with_dup(db: *mut sqlite3, p: *mut With)
    -> *mut With;
    fn sqlite3_materialize_view(_: *mut Parse, _: *mut Table, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_begin_trigger(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: i32, _: i32, _: *mut IdList, _: *mut SrcList, _: *mut Expr, _: i32,
    _: i32)
    -> ();
    fn sqlite3_finish_trigger(_: *mut Parse, _: *mut TriggerStep,
    _: *mut Token)
    -> ();
    fn sqlite3_drop_trigger(_: *mut Parse, _: *mut SrcList, _: i32)
    -> ();
    fn sqlite3_drop_trigger_ptr(_: *mut Parse, _: *mut Trigger)
    -> ();
    fn sqlite3_triggers_exist(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut ExprList, p_mask_1: *mut i32)
    -> *mut Trigger;
    fn sqlite3_trigger_list(_: *mut Parse, _: *mut Table)
    -> *mut Trigger;
    fn sqlite3_code_row_trigger(_: *mut Parse, _: *mut Trigger, _: i32,
    _: *mut ExprList, _: i32, _: *mut Table, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_code_row_trigger_direct(_: *mut Parse, _: *mut Trigger,
    _: *mut Table, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite_view_triggers(_: *mut Parse, _: *mut Table, _: *mut Expr,
    _: i32, _: *mut ExprList)
    -> ();
    fn sqlite3_delete_trigger_step(_: *mut sqlite3, _: *mut TriggerStep)
    -> ();
    fn sqlite3_trigger_select_step(_: *mut sqlite3, _: *mut Select,
    _: *const i8, _: *const i8)
    -> *mut TriggerStep;
    fn sqlite3_trigger_insert_step(_: *mut Parse, _: *mut SrcList,
    _: *mut IdList, _: *mut Select, _: u8, _: *mut Upsert, _: *const i8,
    _: *const i8)
    -> *mut TriggerStep;
    fn sqlite3_trigger_update_step(_: *mut Parse, _: *mut SrcList,
    _: *mut SrcList, _: *mut ExprList, _: *mut Expr, _: u8, _: *const i8,
    _: *const i8)
    -> *mut TriggerStep;
    fn sqlite3_trigger_delete_step(_: *mut Parse, _: *mut SrcList,
    _: *mut Expr, _: *const i8, _: *const i8)
    -> *mut TriggerStep;
    fn sqlite3_delete_trigger(_: *mut sqlite3, _: *mut Trigger)
    -> ();
    fn sqlite3_unlink_and_delete_trigger(_: *mut sqlite3, _: i32,
    _: *const i8)
    -> ();
    fn sqlite3_trigger_colmask(_: *mut Parse, _: *mut Trigger,
    _: *mut ExprList, _: i32, _: i32, _: *mut Table, _: i32)
    -> u32;
    fn sqlite3_join_type(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token)
    -> i32;
    fn sqlite3_column_index(p_tab_1: *mut Table, z_col_1: *const i8)
    -> i32;
    fn sqlite3_src_item_column_used(_: *mut SrcItem, _: i32)
    -> ();
    fn sqlite3_set_join_expr(_: *mut Expr, _: i32, _: u32)
    -> ();
    fn sqlite3_create_foreign_key(_: *mut Parse, _: *mut ExprList,
    _: *mut Token, _: *mut ExprList, _: i32)
    -> ();
    fn sqlite3_defer_foreign_key(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_auth_read(_: *mut Parse, _: *mut Expr, _: *mut Schema,
    _: *mut SrcList)
    -> ();
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_auth_context_push(_: *mut Parse, _: *mut AuthContext,
    _: *const i8)
    -> ();
    fn sqlite3_auth_context_pop(_: *mut AuthContext)
    -> ();
    fn sqlite3_auth_read_col(_: *mut Parse, _: *const i8, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_db_is_named(db: *mut sqlite3, i_db_1: i32, z_name_1: *const i8)
    -> i32;
    fn sqlite3_attach(_: *mut Parse, _: *mut Expr, _: *mut Expr, _: *mut Expr)
    -> ();
    fn sqlite3_detach(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_fix_init(_: *mut DbFixer, _: *mut Parse, _: i32, _: *const i8,
    _: *const Token)
    -> ();
    fn sqlite3_fix_src_list(_: *mut DbFixer, _: *mut SrcList)
    -> i32;
    fn sqlite3_fix_select(_: *mut DbFixer, _: *mut Select)
    -> i32;
    fn sqlite3_fix_expr(_: *mut DbFixer, _: *mut Expr)
    -> i32;
    fn sqlite3_fix_trigger_step(_: *mut DbFixer, _: *mut TriggerStep)
    -> i32;
    fn sqlite3_real_same_as_int(_: f64, _: sqlite3_int64)
    -> i32;
    fn sqlite3_real_to_i64(_: f64)
    -> i64;
    fn sqlite3_int64_to_text(_: i64, _: *mut i8)
    -> i32;
    fn sqlite3_ato_f(z: *const i8, _: *mut f64)
    -> i32;
    fn sqlite3_get_int32(_: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_get_u_int32(_: *const i8, _: *mut u32)
    -> i32;
    fn sqlite3_atoi(_: *const i8)
    -> i32;
    fn sqlite3_utf16_byte_len(p_data_1: *const (), n_byte_1: i32,
    n_char_1: i32)
    -> i32;
    fn sqlite3_utf8_char_len(p_data_1: *const i8, n_byte_1: i32)
    -> i32;
    fn sqlite3_utf8_read(_: *mut *const u8)
    -> u32;
    fn sqlite3_utf8_read_limited(_: *const u8, _: i32, _: *mut u32)
    -> i32;
    fn sqlite3_log_est(_: u64)
    -> LogEst;
    fn sqlite3_log_est_add(_: LogEst, _: LogEst)
    -> LogEst;
    fn sqlite3_log_est_from_double(_: f64)
    -> LogEst;
    fn sqlite3_log_est_to_int(_: LogEst)
    -> u64;
    fn sqlite3_v_list_add(_: *mut sqlite3, _: *mut VList, _: *const i8,
    _: i32, _: i32)
    -> *mut VList;
    fn sqlite3_v_list_num_to_name(_: *mut VList, _: i32)
    -> *const i8;
    fn sqlite3_v_list_name_to_num(_: *mut VList, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_get_varint32(_: *const u8, _: *mut u32)
    -> u8;
    fn sqlite3_varint_len(v: u64)
    -> i32;
    fn sqlite3_index_affinity_str(_: *mut sqlite3, _: *mut Index)
    -> *const i8;
    fn sqlite3_table_affinity_str(_: *mut sqlite3, _: *const Table)
    -> *mut i8;
    fn sqlite3_table_affinity(_: *mut Vdbe, _: *mut Table, _: i32)
    -> ();
    fn sqlite3_compare_affinity(p_expr_1: *const Expr, aff2: i8)
    -> i8;
    fn sqlite3_index_affinity_ok(p_expr_1: *const Expr, idx_affinity: i8)
    -> i32;
    fn sqlite3_table_column_affinity(_: *const Table, _: i32)
    -> i8;
    fn sqlite3_expr_affinity(p_expr_1: *const Expr)
    -> i8;
    fn sqlite3_expr_data_type(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_atoi64(_: *const i8, _: *mut i64, _: i32, _: u8)
    -> i32;
    fn sqlite3_dec_or_hex_to_i64(_: *const i8, _: *mut i64)
    -> i32;
    fn sqlite3_error_with_msg(_: *mut sqlite3, _: i32, _: *const i8, ...)
    -> ();
    fn sqlite3_error(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_error_clear(_: *mut sqlite3)
    -> ();
    fn sqlite3_system_error(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_hex_to_blob(_: *mut sqlite3, z: *const i8, n: i32)
    -> *mut ();
    fn sqlite3_hex_to_int(h: i32)
    -> u8;
    fn sqlite3_two_part_name(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut *mut Token)
    -> i32;
    fn sqlite3_memdb_init()
    -> i32;
    fn sqlite3_is_memdb(_: *const sqlite3_vfs)
    -> i32;
    fn sqlite3_err_str(_: i32)
    -> *const i8;
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_find_coll_seq(_: *mut sqlite3, enc: u8, _: *const i8, _: i32)
    -> *mut CollSeq;
    fn sqlite3_is_binary(_: *const CollSeq)
    -> i32;
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_set_text_encoding(db: *mut sqlite3, _: u8)
    -> ();
    fn sqlite3_expr_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_expr_nn_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_expr_coll_seq_match(_: *mut Parse, _: *const Expr,
    _: *const Expr)
    -> i32;
    fn sqlite3_expr_add_collate_token(p_parse_1: *const Parse, _: *mut Expr,
    _: *const Token, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_add_collate_string(_: *const Parse, _: *mut Expr,
    _: *const i8)
    -> *mut Expr;
    fn sqlite3_expr_skip_collate(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_expr_skip_collate_and_likely(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_check_coll_seq(_: *mut Parse, _: *mut CollSeq)
    -> i32;
    fn sqlite3_check_object_name(_: *mut Parse, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_vdbe_set_changes(_: *mut sqlite3, _: i64)
    -> ();
    fn sqlite3_add_int64(_: *mut i64, _: i64)
    -> i32;
    fn sqlite3_sub_int64(_: *mut i64, _: i64)
    -> i32;
    fn sqlite3_mul_int64(_: *mut i64, _: i64)
    -> i32;
    fn sqlite3_get_boolean(z: *const i8, _: u8)
    -> u8;
    fn sqlite3ValueText(_: *mut sqlite3_value, _: u8)
    -> *const ();
    fn sqlite3_value_is_of_class(_: *const sqlite3_value,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3ValueBytes(_: *mut sqlite3_value, _: u8)
    -> i32;
    fn sqlite3_value_set_str(_: *mut sqlite3_value, _: i32, _: *const (),
    _: u8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_value_set_null(_: *mut sqlite3_value)
    -> ();
    fn sqlite3ValueFree(_: *mut sqlite3_value)
    -> ();
    fn sqlite3_result_int_real(_: *mut sqlite3_context)
    -> ();
    fn sqlite3_value_new(_: *mut sqlite3)
    -> *mut sqlite3_value;
    fn sqlite3_utf16to8(_: *mut sqlite3, _: *const (), _: i32, _: u8)
    -> *mut i8;
    fn sqlite3_value_from_expr(_: *mut sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_apply_affinity(_: *mut sqlite3_value, _: u8, _: u8)
    -> ();
    static sqlite3_opcode_property: [u8; 0];
    static sqlite3_str_binary: [i8; 0];
    static sqlite3_std_type_len: [u8; 0];
    static sqlite3_std_type_affinity: [i8; 0];
    static mut sqlite3_std_type: [*const i8; 0];
    static sqlite3_upper_to_lower: [u8; 0];
    static mut sqlite3a_l_tb: *const u8;
    static mut sqlite3a_e_qb: *const u8;
    static mut sqlite3a_g_tb: *const u8;
    static sqlite3_ctype_map: [u8; 0];
    static mut sqlite3_builtin_functions: FuncDefHash;
    static sqlite3_oom_str: sqlite3_str;
    fn sqlite3_root_page_moved(_: *mut sqlite3, _: i32, _: Pgno, _: Pgno)
    -> ();
    fn sqlite3_reindex(_: *mut Parse, _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_alter_functions()
    -> ();
    fn sqlite3_alter_rename_table(_: *mut Parse, _: *mut SrcList,
    _: *mut Token)
    -> ();
    fn sqlite3_alter_rename_column(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_alter_drop_constraint(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_alter_add_constraint(p_parse_1: *mut Parse,
    p_src_1: *mut SrcList, p_first_1: *mut Token, p_name_1: *mut Token,
    z_expr_1: *const i8, n_expr_1: i32, p_expr_1: *mut Expr)
    -> ();
    fn sqlite3_alter_set_not_null(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_get_token(_: *const u8, _: *mut i32)
    -> i64;
    fn sqlite3_nested_parse(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_expire_prepared_statements(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_code_rhs_of_in(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_code_subselect(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_select_prep(_: *mut Parse, _: *mut Select, _: *mut NameContext)
    -> ();
    fn sqlite3_expand_subquery(_: *mut Parse, _: *mut SrcItem)
    -> i32;
    fn sqlite3_select_wrong_num_terms_error(p_parse_1: *mut Parse,
    p: *mut Select)
    -> ();
    fn sqlite3_match_e_name(_: *const ExprList_item, _: *const i8,
    _: *const i8, _: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_expr_col_used(_: *mut Expr)
    -> Bitmask;
    fn sqlite3_str_i_hash(_: *const i8)
    -> u8;
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
    -> i32;
    fn sqlite3_resolve_expr_list_names(_: *mut NameContext, _: *mut ExprList)
    -> i32;
    fn sqlite3_resolve_select_names(_: *mut Parse, _: *mut Select,
    _: *mut NameContext)
    -> ();
    fn sqlite3_resolve_self_reference(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut Expr, _: *mut ExprList)
    -> i32;
    fn sqlite3_resolve_order_group_by(_: *mut Parse, _: *mut Select,
    _: *mut ExprList, _: *const i8)
    -> i32;
    fn sqlite3_column_default(_: *mut Vdbe, _: *mut Table, _: i32, _: i32)
    -> ();
    fn sqlite3_alter_finish_add_column(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_alter_begin_add_column(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_alter_drop_column(_: *mut Parse, _: *mut SrcList,
    _: *const Token)
    -> ();
    fn sqlite3_rename_token_map(_: *mut Parse, _: *const (), _: *const Token)
    -> *const ();
    fn sqlite3_rename_token_remap(_: *mut Parse, p_to_1: *const (),
    p_from_1: *const ())
    -> ();
    fn sqlite3_rename_expr_unmap(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_rename_exprlist_unmap(_: *mut Parse, _: *mut ExprList)
    -> ();
    fn sqlite3_get_coll_seq(_: *mut Parse, _: u8, _: *mut CollSeq,
    _: *const i8)
    -> *mut CollSeq;
    fn sqlite3_affinity_type(_: *const i8, _: *mut Column)
    -> i8;
    fn sqlite3_analyze(_: *mut Parse, _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_find_db(_: *mut sqlite3, _: *mut Token)
    -> i32;
    fn sqlite3_find_db_name(_: *mut sqlite3, _: *const i8)
    -> i32;
    fn sqlite3_analysis_load(_: *mut sqlite3, i_db_1: i32)
    -> i32;
    fn sqlite3_delete_index_samples(_: *mut sqlite3, _: *mut Index)
    -> ();
    fn sqlite3_default_row_est(_: *mut Index)
    -> ();
    fn sqlite3_register_like_functions(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_is_like_function(_: *mut sqlite3, _: *mut Expr, _: *mut i32,
    _: *mut i8)
    -> i32;
    fn sqlite3_schema_clear(_: *mut ())
    -> ();
    fn sqlite3_schema_get(_: *mut sqlite3, _: *mut Btree)
    -> *mut Schema;
    fn sqlite3_schema_to_index(db: *mut sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_key_info_alloc(_: *mut sqlite3, _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_key_info_unref(_: *mut KeyInfo)
    -> ();
    fn sqlite3_key_info_ref(_: *mut KeyInfo)
    -> *mut KeyInfo;
    fn sqlite3_key_info_of_index(_: *mut Parse, _: *mut Index)
    -> *mut KeyInfo;
    fn sqlite3_key_info_from_expr_list(_: *mut Parse, _: *mut ExprList,
    _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_select_op_name(_: i32)
    -> *const i8;
    fn sqlite3_has_explicit_nulls(_: *mut Parse, _: *mut ExprList)
    -> i32;
    fn sqlite3_create_func(_: *mut sqlite3, _: *const i8, _: i32, _: i32,
    _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    _:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    _: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    _: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    _:
        Option<unsafe extern "C" fn(*mut sqlite3_context, i32,
            *mut *mut sqlite3_value) -> ()>,
    p_destructor_1: *mut FuncDestructor)
    -> i32;
    fn sqlite3_noop_destructor(_: *mut ())
    -> ();
    fn sqlite3_oom_fault(_: *mut sqlite3)
    -> *mut ();
    fn sqlite3_oom_clear(_: *mut sqlite3)
    -> ();
    fn sqlite3_api_exit(db: *mut sqlite3, _: i32)
    -> i32;
    fn sqlite3_open_temp_database(_: *mut Parse)
    -> i32;
    fn sqlite3_rc_str_ref(_: *mut i8)
    -> *mut i8;
    fn sqlite3_rc_str_unref(_: *mut ())
    -> ();
    fn sqlite3_rc_str_new(_: u64)
    -> *mut i8;
    fn sqlite3_rc_str_resize(_: *mut i8, _: u64)
    -> *mut i8;
    fn sqlite3_str_accum_enlarge(_: *mut StrAccum, _: i64)
    -> i32;
    fn sqlite3_str_accum_enlarge_if_needed(_: *mut StrAccum, _: i64)
    -> i32;
    fn sqlite3_str_accum_set_error(_: *mut StrAccum, _: u8)
    -> ();
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_create_column_expr(_: *mut sqlite3, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_record_error_byte_offset(_: *mut sqlite3, _: *const i8)
    -> ();
    fn sqlite3_record_error_offset_of_expr(_: *mut sqlite3, _: *const Expr)
    -> ();
    fn sqlite3_backup_restart(_: *mut sqlite3_backup)
    -> ();
    fn sqlite3_backup_update(_: *mut sqlite3_backup, _: Pgno, _: *const u8)
    -> ();
    fn sqlite3_expr_check_in(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_parser_alloc(_: Option<unsafe extern "C" fn(u64) -> *mut ()>,
    _: *mut Parse)
    -> *mut ();
    fn sqlite3_parser_free(_: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_parser(_: *mut (), _: i32, _: Token)
    -> ();
    fn sqlite3_parser_fallback(_: i32)
    -> i32;
    fn sqlite3_auto_load_extensions(_: *mut sqlite3)
    -> ();
    fn sqlite3_close_extensions(_: *mut sqlite3)
    -> ();
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_vtab_clear(db: *mut sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_disconnect(db: *mut sqlite3, p: *mut Table)
    -> ();
    fn sqlite3_vtab_sync(db: *mut sqlite3, _: *mut Vdbe)
    -> i32;
    fn sqlite3_vtab_rollback(db: *mut sqlite3)
    -> i32;
    fn sqlite3_vtab_commit(db: *mut sqlite3)
    -> i32;
    fn sqlite3_vtab_lock(_: *mut VTable)
    -> ();
    fn sqlite3_vtab_unlock(_: *mut VTable)
    -> ();
    fn sqlite3_vtab_module_unref(_: *mut sqlite3, _: *mut Module)
    -> ();
    fn sqlite3_vtab_unlock_list(_: *mut sqlite3)
    -> ();
    fn sqlite3_vtab_savepoint(_: *mut sqlite3, _: i32, _: i32)
    -> i32;
    fn sqlite3_vtab_import_errmsg(_: *mut Vdbe, _: *mut sqlite3_vtab)
    -> ();
    fn sqlite3_get_v_table(_: *mut sqlite3, _: *mut Table)
    -> *mut VTable;
    fn sqlite3_vtab_create_module(_: *mut sqlite3, _: *const i8,
    _: *const sqlite3_module, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut Module;
    fn sqlite3_read_only_shadow_tables(db: *mut sqlite3)
    -> i32;
    fn sqlite3_shadow_table_name(db: *mut sqlite3, z_name_1: *const i8)
    -> i32;
    fn sqlite3_is_shadow_table_of(_: *mut sqlite3, _: *mut Table,
    _: *const i8)
    -> i32;
    fn sqlite3_mark_all_shadow_tables_of(_: *mut sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_eponymous_table_init(_: *mut Parse, _: *mut Module)
    -> i32;
    fn sqlite3_vtab_eponymous_table_clear(_: *mut sqlite3, _: *mut Module)
    -> ();
    fn sqlite3_vtab_make_writable(_: *mut Parse, _: *mut Table)
    -> ();
    fn sqlite3_vtab_begin_parse(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: i32)
    -> ();
    fn sqlite3_vtab_finish_parse(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_vtab_arg_init(_: *mut Parse)
    -> ();
    fn sqlite3_vtab_arg_extend(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_vtab_call_create(_: *mut sqlite3, _: i32, _: *const i8,
    _: *mut *mut i8)
    -> i32;
    fn sqlite3_vtab_call_connect(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_vtab_call_destroy(_: *mut sqlite3, _: i32, _: *const i8)
    -> i32;
    fn sqlite3_vtab_begin(_: *mut sqlite3, _: *mut VTable)
    -> i32;
    fn sqlite3_vtab_overload_function(_: *mut sqlite3, _: *mut FuncDef,
    n_arg_1: i32, _: *mut Expr)
    -> *mut FuncDef;
    fn sqlite3_vtab_uses_all_schemas(_: *mut Parse)
    -> ();
    fn sqlite3_stmt_current_time(_: *mut sqlite3_context)
    -> sqlite3_int64;
    fn sqlite3_vdbe_parameter_index(_: *mut Vdbe, _: *const i8, _: i32)
    -> i32;
    fn sqlite3TransferBindings(_: *mut sqlite3_stmt, _: *mut sqlite3_stmt)
    -> i32;
    fn sqlite3_parse_object_init(_: *mut Parse, _: *mut sqlite3)
    -> ();
    fn sqlite3_parse_object_reset(_: *mut Parse)
    -> ();
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_reprepare(_: *mut Vdbe)
    -> i32;
    fn sqlite3_expr_list_check_length(_: *mut Parse, _: *mut ExprList,
    _: *const i8)
    -> ();
    fn sqlite3_expr_compare_coll_seq(_: *mut Parse, _: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_binary_compare_coll_seq(_: *mut Parse, _: *const Expr,
    _: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_journal_modename(_: i32)
    -> *const i8;
    fn sqlite3_checkpoint(_: *mut sqlite3, _: i32, _: i32, _: *mut i32,
    _: *mut i32)
    -> i32;
    fn sqlite3_wal_default_hook(_: *mut (), _: *mut sqlite3, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_cte_new(_: *mut Parse, _: *mut Token, _: *mut ExprList,
    _: *mut Select, _: u8)
    -> *mut Cte;
    fn sqlite3_cte_delete(_: *mut sqlite3, _: *mut Cte)
    -> ();
    fn sqlite3_with_add(_: *mut Parse, _: *mut With, _: *mut Cte)
    -> *mut With;
    fn sqlite3_with_delete(_: *mut sqlite3, _: *mut With)
    -> ();
    fn sqlite3_with_delete_generic(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_with_push(_: *mut Parse, _: *mut With, _: u8)
    -> *mut With;
    fn sqlite3_upsert_new(_: *mut sqlite3, _: *mut ExprList, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr, _: *mut Upsert)
    -> *mut Upsert;
    fn sqlite3_upsert_delete(_: *mut sqlite3, _: *mut Upsert)
    -> ();
    fn sqlite3_upsert_dup(_: *mut sqlite3, _: *mut Upsert)
    -> *mut Upsert;
    fn sqlite3_upsert_analyze_target(_: *mut Parse, _: *mut SrcList,
    _: *mut Upsert, _: *mut Upsert)
    -> i32;
    fn sqlite3_upsert_do_update(_: *mut Parse, _: *mut Upsert, _: *mut Table,
    _: *mut Index, _: i32)
    -> ();
    fn sqlite3_upsert_of_index(_: *mut Upsert, _: *mut Index)
    -> *mut Upsert;
    fn sqlite3_upsert_next_is_ipk(_: *mut Upsert)
    -> i32;
    fn sqlite3_fk_check(_: *mut Parse, _: *mut Table, _: i32, _: i32,
    _: *mut i32, _: i32)
    -> ();
    fn sqlite3_fk_drop_table(_: *mut Parse, _: *mut SrcList, _: *mut Table)
    -> ();
    fn sqlite3_fk_actions(_: *mut Parse, _: *mut Table, _: *mut ExprList,
    _: i32, _: *mut i32, _: i32)
    -> ();
    fn sqlite3_fk_required(_: *mut Parse, _: *mut Table, _: *mut i32, _: i32)
    -> i32;
    fn sqlite3_fk_oldmask(_: *mut Parse, _: *mut Table)
    -> u32;
    fn sqlite3_fk_references(_: *mut Table)
    -> *mut FKey;
    fn sqlite3_fk_clear_trigger_cache(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_fk_delete(_: *mut sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_fk_locate_index(_: *mut Parse, _: *mut Table, _: *mut FKey,
    _: *mut *mut Index, _: *mut *mut i32)
    -> i32;
    fn sqlite3_begin_benign_malloc()
    -> ();
    fn sqlite3_end_benign_malloc()
    -> ();
    fn sqlite3_find_in_index(_: *mut Parse, _: *mut Expr, _: u32, _: *mut i32,
    _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_journal_open(_: *mut sqlite3_vfs, _: *const i8,
    _: *mut sqlite3_file, _: i32, _: i32)
    -> i32;
    fn sqlite3_journal_size(_: *mut sqlite3_vfs)
    -> i32;
    fn sqlite3_journal_is_in_memory(p: *mut sqlite3_file)
    -> i32;
    fn sqlite3_mem_journal_open(_: *mut sqlite3_file)
    -> ();
    fn sqlite3_expr_set_height_and_flags(p_parse_1: *mut Parse, p: *mut Expr)
    -> ();
    fn sqlite3_select_expr_height(_: *const Select)
    -> i32;
    fn sqlite3_expr_check_height(_: *mut Parse, _: i32)
    -> i32;
    fn sqlite3_expr_set_error_offset(_: *mut Expr, _: i32)
    -> ();
    fn sqlite3_thread_create(_: *mut *mut SQLiteThread,
    _: Option<unsafe extern "C" fn(*mut ()) -> *mut ()>, _: *mut ())
    -> i32;
    fn sqlite3_thread_join(_: *mut SQLiteThread, _: *mut *mut ())
    -> i32;
    fn sqlite3_expr_vector_size(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_expr_is_vector(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_vector_field_subexpr(_: *mut Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_for_vector_field(_: *mut Parse, _: *mut Expr, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_vector_error_msg(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_compile_options(pn_opt_1: *mut i32)
    -> *mut *const i8;
    fn __builtin_unreachable()
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CCurHint {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CheckOnCtx {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CoveringIndexCheck {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IdxCover {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct RefSrcList {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct RenameCtx {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereConst {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WindowRewrite {
    _opaque: [u8; 0],
}