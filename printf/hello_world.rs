#![feature(c_variadic)]
type __darwin_va_list = *mut i8;
type __darwin_size_t = u64;
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
    _opaque: [u8; 0],
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
type Pgno = u32;
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
struct sqlite3_str {
    db: *mut sqlite3,
    z_text: *mut i8,
    n_alloc: u32,
    mx_alloc: u32,
    n_char: u32,
    acc_error: u8,
    printf_flags: u8,
}
type StrAccum = sqlite3_str;
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_accum_init(p: &mut StrAccum, db: *mut sqlite3,
    z_base_1: *mut i8, n: i32, mx: i32) -> () {
    (*p).z_text = z_base_1;
    (*p).db = db;
    (*p).n_alloc = n as u32;
    (*p).mx_alloc = mx as u32;
    (*p).n_char = 0 as u32;
    (*p).acc_error = 0 as u8;
    (*p).printf_flags = 0 as u8;
}
type etByte = u8;
#[repr(C)]
#[derive(Copy, Clone)]
struct et_info {
    fmttype: i8,
    base: etByte,
    flags: etByte,
    type_: etByte,
    charset: etByte,
    prefix: etByte,
    i_nxt: i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PrintfArguments {
    n_arg: i32,
    n_used: i32,
    ap_arg: *mut *mut sqlite3_value,
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_reset(p: *mut StrAccum) -> () {
    unsafe {
        if unsafe { (*p).printf_flags } as i32 & 4 != 0 {
            unsafe {
                sqlite3_db_free(unsafe { (*p).db },
                    unsafe { (*p).z_text } as *mut ())
            };
            unsafe { (*p).printf_flags &= !4 as u8 };
        } else if p == &raw const sqlite3_oom_str as *mut sqlite3_str {
            return;
        }
        unsafe { (*p).n_alloc = 0 as u32 };
        unsafe { (*p).n_char = 0 as u32 };
        unsafe { (*p).z_text = core::ptr::null_mut() };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_accum_set_error(p: *mut StrAccum, e_error_1: u8)
    -> () {
    { let _ = 0; };
    unsafe { (*p).acc_error = e_error_1 };
    if unsafe { (*p).mx_alloc } != 0 {
        unsafe { sqlite3_str_reset(p as *mut sqlite3_str) };
    }
    if e_error_1 as i32 == 18 {
        unsafe {
            sqlite3_error_to_parser(unsafe { (*p).db }, e_error_1 as i32)
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_accum_enlarge(p: *mut StrAccum, n_1: i64)
    -> i32 {
    let mut z_new: *mut i8 = core::ptr::null_mut();
    { let _ = 0; };
    if unsafe { (*p).acc_error } != 0 { return 0; }
    if unsafe { (*p).mx_alloc } == 0 as u32 {
        sqlite3_str_accum_set_error(p, 18 as u8);
        return (unsafe { (*p).n_alloc } - unsafe { (*p).n_char } - 1 as u32)
                as i32;
    } else {
        let z_old: *mut i8 =
            if unsafe { (*p).printf_flags } as i32 & 4 != 0 {
                unsafe { (*p).z_text }
            } else { core::ptr::null_mut() };
        let mut sz_new: i64 = unsafe { (*p).n_char } as i64 + n_1 + 1 as i64;
        if sz_new + unsafe { (*p).n_char } as i64 <=
                unsafe { (*p).mx_alloc } as i64 {
            sz_new += unsafe { (*p).n_char } as i64;
        }
        if sz_new > unsafe { (*p).mx_alloc } as i64 {
            unsafe { sqlite3_str_reset(p as *mut sqlite3_str) };
            sqlite3_str_accum_set_error(p, 18 as u8);
            return 0;
        } else { unsafe { (*p).n_alloc = sz_new as i32 as u32 }; }
        if !(unsafe { (*p).db }).is_null() {
            z_new =
                unsafe {
                        sqlite3_db_realloc(unsafe { (*p).db }, z_old as *mut (),
                            unsafe { (*p).n_alloc } as u64)
                    } as *mut i8;
        } else {
            z_new =
                unsafe {
                        sqlite3Realloc(z_old as *mut (),
                            unsafe { (*p).n_alloc } as u64)
                    } as *mut i8;
        }
        if !(z_new).is_null() {
            { let _ = 0; };
            if !(unsafe { (*p).printf_flags } as i32 & 4 != 0) as i32 != 0 &&
                    unsafe { (*p).n_char } > 0 as u32 {
                unsafe {
                    memcpy(z_new as *mut (),
                        unsafe { (*p).z_text } as *const (),
                        unsafe { (*p).n_char } as u64)
                };
            }
            unsafe { (*p).z_text = z_new };
            unsafe {
                (*p).n_alloc =
                    unsafe {
                            sqlite3_db_malloc_size(unsafe { (*p).db },
                                z_new as *const ())
                        } as u32
            };
            unsafe { (*p).printf_flags |= 4 as u8 };
        } else {
            unsafe { sqlite3_str_reset(p as *mut sqlite3_str) };
            sqlite3_str_accum_set_error(p, 7 as u8);
            return 0;
        }
    }
    { let _ = 0; };
    return n_1 as i32;
}
extern "C" fn enlarge_and_append(p: *mut StrAccum, z: *const i8, mut n_1: i32)
    -> () {
    n_1 = sqlite3_str_accum_enlarge(p, n_1 as i64);
    { let _ = 0; };
    if n_1 > 0 {
        unsafe {
            memcpy(unsafe {
                        &raw mut *unsafe {
                                    (*p).z_text.add(unsafe { (*p).n_char } as usize)
                                }
                    } as *mut (), z as *const (), n_1 as u64)
        };
        unsafe { (*p).n_char += n_1 as u32 };
    }
}
extern "C" fn sqlite3_str_append64(p: *mut sqlite3_str, z: *const i8,
    n_1: i64) -> () {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p).n_char } as i64 + n_1 >= unsafe { (*p).n_alloc } as i64 {
        enlarge_and_append(p as *mut StrAccum, z, n_1 as i32);
    } else if n_1 != 0 {
        { let _ = 0; };
        unsafe { (*p).n_char += n_1 as u32 };
        unsafe {
            memcpy(unsafe {
                        &raw mut *unsafe {
                                    (*p).z_text.offset((unsafe { (*p).n_char } as i64 - n_1) as
                                            isize)
                                }
                    } as *mut (), z as *const (), n_1 as u64)
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_append(p: *mut sqlite3_str, z: *const i8,
    n_1: i32) -> () {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p).n_char } + n_1 as u32 >= unsafe { (*p).n_alloc } {
        enlarge_and_append(p as *mut StrAccum, z, n_1);
    } else if n_1 != 0 {
        { let _ = 0; };
        unsafe { (*p).n_char += n_1 as u32 };
        unsafe {
            memcpy(unsafe {
                        &raw mut *unsafe {
                                    (*p).z_text.add((unsafe { (*p).n_char } - n_1 as u32) as
                                            usize)
                                }
                    } as *mut (), z as *const (), n_1 as u64)
        };
    }
}
extern "C" fn get_int_arg(p: &mut PrintfArguments) -> sqlite3_int64 {
    if (*p).n_arg <= (*p).n_used { return 0 as sqlite3_int64; }
    return unsafe {
            sqlite3_value_int64(unsafe {
                    *(*p).ap_arg.offset({
                                    let __p = &mut (*p).n_used;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                })
        };
}
static fmtinfo: [et_info; 25] =
    [et_info {
                fmttype: 'd' as i32 as i8,
                base: 10 as etByte,
                flags: 1 as etByte,
                type_: 16 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'e' as i32 as i8,
                base: 0 as etByte,
                flags: 1 as etByte,
                type_: 2 as etByte,
                charset: 30 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'f' as i32 as i8,
                base: 0 as etByte,
                flags: 1 as etByte,
                type_: 1 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'g' as i32 as i8,
                base: 0 as etByte,
                flags: 1 as etByte,
                type_: 3 as etByte,
                charset: 30 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'j' as i32 as i8,
                base: 0 as etByte,
                flags: 0 as etByte,
                type_: 17 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'i' as i32 as i8,
                base: 10 as etByte,
                flags: 1 as etByte,
                type_: 16 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'Q' as i32 as i8,
                base: 0 as etByte,
                flags: 4 as etByte,
                type_: 10 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 4 as i8,
            },
            et_info {
                fmttype: 'p' as i32 as i8,
                base: 16 as etByte,
                flags: 0 as etByte,
                type_: 13 as etByte,
                charset: 0 as etByte,
                prefix: 1 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'S' as i32 as i8,
                base: 0 as etByte,
                flags: 0 as etByte,
                type_: 12 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'T' as i32 as i8,
                base: 0 as etByte,
                flags: 0 as etByte,
                type_: 11 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'n' as i32 as i8,
                base: 0 as etByte,
                flags: 0 as etByte,
                type_: 4 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'o' as i32 as i8,
                base: 8 as etByte,
                flags: 0 as etByte,
                type_: 0 as etByte,
                charset: 0 as etByte,
                prefix: 2 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: '%' as i32 as i8,
                base: 0 as etByte,
                flags: 0 as etByte,
                type_: 7 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 7 as i8,
            },
            et_info {
                fmttype: 'q' as i32 as i8,
                base: 0 as etByte,
                flags: 4 as etByte,
                type_: 9 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 16 as i8,
            },
            et_info {
                fmttype: 'r' as i32 as i8,
                base: 10 as etByte,
                flags: 1 as etByte,
                type_: 15 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 's' as i32 as i8,
                base: 0 as etByte,
                flags: 4 as etByte,
                type_: 5 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'X' as i32 as i8,
                base: 16 as etByte,
                flags: 0 as etByte,
                type_: 0 as etByte,
                charset: 0 as etByte,
                prefix: 4 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'u' as i32 as i8,
                base: 10 as etByte,
                flags: 0 as etByte,
                type_: 16 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'w' as i32 as i8,
                base: 0 as etByte,
                flags: 4 as etByte,
                type_: 14 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'E' as i32 as i8,
                base: 0 as etByte,
                flags: 1 as etByte,
                type_: 2 as etByte,
                charset: 14 as etByte,
                prefix: 0 as etByte,
                i_nxt: 18 as i8,
            },
            et_info {
                fmttype: 'x' as i32 as i8,
                base: 16 as etByte,
                flags: 0 as etByte,
                type_: 0 as etByte,
                charset: 16 as etByte,
                prefix: 1 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'G' as i32 as i8,
                base: 0 as etByte,
                flags: 1 as etByte,
                type_: 3 as etByte,
                charset: 14 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'z' as i32 as i8,
                base: 0 as etByte,
                flags: 4 as etByte,
                type_: 6 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'J' as i32 as i8,
                base: 0 as etByte,
                flags: 0 as etByte,
                type_: 18 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 0 as i8,
            },
            et_info {
                fmttype: 'c' as i32 as i8,
                base: 0 as etByte,
                flags: 0 as etByte,
                type_: 8 as etByte,
                charset: 0 as etByte,
                prefix: 0 as etByte,
                i_nxt: 23 as i8,
            }];
extern "C" fn printf_temp_buf(p_accum_1: *mut sqlite3_str, n: sqlite3_int64)
    -> *mut i8 {
    let mut z: *mut i8 = core::ptr::null_mut();
    if unsafe { (*p_accum_1).acc_error } != 0 {
        return core::ptr::null_mut();
    }
    if n > unsafe { (*p_accum_1).n_alloc } as i64 &&
            n > unsafe { (*p_accum_1).mx_alloc } as i64 {
        sqlite3_str_accum_set_error(p_accum_1 as *mut StrAccum, 18 as u8);
        return core::ptr::null_mut();
    }
    z = unsafe { sqlite3_malloc(n as i32) } as *mut i8;
    if z == core::ptr::null_mut() {
        sqlite3_str_accum_set_error(p_accum_1 as *mut StrAccum, 7 as u8);
    }
    return z;
}
static a_digits: [i8; 33] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 65 as i8, 66 as i8, 67 as i8,
            68 as i8, 69 as i8, 70 as i8, 48 as i8, 49 as i8, 50 as i8,
            51 as i8, 52 as i8, 53 as i8, 54 as i8, 55 as i8, 56 as i8,
            57 as i8, 97 as i8, 98 as i8, 99 as i8, 100 as i8, 101 as i8,
            102 as i8, 0 as i8];
static a_prefix: [i8; 7] =
    [45 as i8, 120 as i8, 48 as i8, 0 as i8, 88 as i8, 48 as i8, 0 as i8];
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
extern "C" fn get_double_arg(p: &mut PrintfArguments) -> f64 {
    if (*p).n_arg <= (*p).n_used { return 0.0; }
    return unsafe {
            sqlite3_value_double(unsafe {
                    *(*p).ap_arg.offset({
                                    let __p = &mut (*p).n_used;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
                })
        };
}
extern "C" fn get_text_arg(p: &mut PrintfArguments) -> *mut i8 {
    if (*p).n_arg <= (*p).n_used { return core::ptr::null_mut(); }
    return unsafe {
                sqlite3_value_text(unsafe {
                        *(*p).ap_arg.offset({
                                        let __p = &mut (*p).n_used;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize)
                    })
            } as *mut i8;
}
extern "C" fn sqlite3_str_appendchar64(p: *mut sqlite3_str, mut n_1: i64,
    c: i8) -> () {
    if unsafe { (*p).n_char } as i64 + n_1 >= unsafe { (*p).n_alloc } as i64
            &&
            {
                    n_1 =
                        sqlite3_str_accum_enlarge(p as *mut StrAccum, n_1) as i64;
                    n_1
                } <= 0 as i64 {
        return;
    }
    while { let __p = &mut n_1; let __t = *__p; *__p -= 1; __t } > 0 as i64 {
        unsafe {
            *unsafe {
                        (*p).z_text.add({
                                    let __p = unsafe { &mut (*p).n_char };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize)
                    } = c
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_accum_enlarge_if_needed(p: *mut StrAccum,
    n_1: i64) -> i32 {
    if n_1 + unsafe { (*p).n_char } as i64 >= unsafe { (*p).n_alloc } as i64 {
        sqlite3_str_accum_enlarge(p, n_1);
    }
    return unsafe { (*p).acc_error } as i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_length(p: *const sqlite3_str) -> i32 {
    return if !(p).is_null() { unsafe { (*p).n_char } } else { 0 as u32 } as
            i32;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_appendchar(p: *mut sqlite3_str, mut n_1: i32,
    c: i8) -> () {
    if unsafe { (*p).n_char } as i64 + n_1 as i64 >=
                unsafe { (*p).n_alloc } as i64 &&
            {
                    n_1 =
                        sqlite3_str_accum_enlarge(p as *mut StrAccum, n_1 as i64);
                    n_1
                } <= 0 {
        return;
    }
    while { let __p = &mut n_1; let __t = *__p; *__p -= 1; __t } > 0 {
        unsafe {
            *unsafe {
                        (*p).z_text.add({
                                    let __p = unsafe { &mut (*p).n_char };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize)
                    } = c
        };
    }
}
static a_hex: [i8; 17] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 97 as i8, 98 as i8, 99 as i8,
            100 as i8, 101 as i8, 102 as i8, 0 as i8];
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_errcode(p: *const sqlite3_str) -> i32 {
    return if !(p).is_null() {
            (unsafe { (*p).acc_error }) as i32
        } else { 7 };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_value(p: *const sqlite3_str) -> *mut i8 {
    if p == core::ptr::null_mut() || unsafe { (*p).n_char } == 0 as u32 {
        return core::ptr::null_mut();
    }
    unsafe {
        *unsafe { (*p).z_text.add(unsafe { (*p).n_char } as usize) } = 0 as i8
    };
    return unsafe { (*p).z_text };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_appendall(p: *mut sqlite3_str, z: *const i8)
    -> () {
    sqlite3_str_append(p, z, unsafe { sqlite3_strlen30(z) });
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_record_error_offset_of_expr(db: &mut sqlite3,
    mut p_expr_1: *const Expr) -> () {
    unsafe {
        while !(p_expr_1).is_null() &&
                (unsafe { (*p_expr_1).flags } & (1 | 2) as u32 != 0 as u32 ||
                    unsafe { (*p_expr_1).w.i_ofst } as i32 <= 0) {
            p_expr_1 = unsafe { (*p_expr_1).p_left } as *const Expr;
        }
        if p_expr_1 == core::ptr::null() { return; }
        if unsafe { (*p_expr_1).flags } & 1073741824 as u32 != 0 as u32 {
            return;
        }
        (*db).err_byte_offset = unsafe { (*p_expr_1).w.i_ofst } as i32;
    }
}
type uptr = u64;
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_record_error_byte_offset(db: *mut sqlite3,
    z: *const i8) -> () {
    let mut p_parse: *const Parse = core::ptr::null();
    let mut z_text: *const i8 = core::ptr::null();
    let mut z_end: *const i8 = core::ptr::null();
    { let _ = 0; };
    if db == core::ptr::null_mut() { return; }
    if unsafe { (*db).err_byte_offset } != -2 { return; }
    p_parse = unsafe { (*db).p_parse } as *const Parse;
    if p_parse == core::ptr::null() { return; }
    z_text = unsafe { (*p_parse).z_tail };
    if z_text == core::ptr::null() { return; }
    z_end = unsafe { z_text.add(unsafe { strlen(z_text) } as usize) };
    if z as uptr >= z_text as uptr && (z as uptr) < z_end as uptr {
        unsafe {
            (*db).err_byte_offset =
                unsafe { z.offset_from(z_text) } as i64 as i32
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_appendf(p: *mut StrAccum,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    sqlite3_str_vappendf(p as *mut sqlite3_str, z_format_1, ap as *const i8);
    ();
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_vappendf(p_accum_1: *mut sqlite3_str,
    mut fmt: *const i8, mut ap: *const i8) -> () {
    unsafe {
        let mut c: i32 = 0;
        let mut bufpt: *mut i8 = core::ptr::null_mut();
        let mut precision: i64 = 0 as i64;
        let mut length: i64 = 0 as i64;
        let mut idx: i32 = 0;
        let mut width: i64 = 0 as i64;
        let mut flag_leftjustify: etByte = 0 as etByte;
        let mut flag_prefix: etByte = 0 as etByte;
        let mut flag_alternateform: etByte = 0 as etByte;
        let mut flag_altform2: etByte = 0 as etByte;
        let mut flag_zeropad: etByte = 0 as etByte;
        let mut flag_long: etByte = 0 as etByte;
        let mut done: etByte = 0 as etByte;
        let mut c_thousand: etByte = 0 as etByte;
        let mut xtype: etByte = 0 as etByte;
        let mut b_arg_list: u8 = 0 as u8;
        let mut prefix: i8 = 0 as i8;
        let mut longvalue: sqlite_uint64 = 0 as sqlite_uint64;
        let mut realvalue: f64 = 0.0;
        let mut infop: *const et_info = core::ptr::null();
        let mut z_out: *mut i8 = core::ptr::null_mut();
        let mut n_out: i32 = 0;
        let mut z_extra: *mut i8 = core::ptr::null_mut();
        let mut exp: i32 = 0;
        let mut e2: i32 = 0;
        let mut flag_dp: etByte = 0 as etByte;
        let mut flag_rtz: etByte = 0 as etByte;
        let mut p_arg_list: *mut PrintfArguments = core::ptr::null_mut();
        let mut buf: [i8; 70] = [0; 70];
        let mut wx: u32 = 0 as u32;
        let mut px: u32 = 0 as u32;
        let mut v: i64 = 0 as i64;
        let mut n: u64 = 0 as u64;
        let mut x: i32 = 0;
        let mut cset: *const i8 = core::ptr::null();
        let mut base: u8 = 0 as u8;
        let mut nn: i64 = 0 as i64;
        let mut nn__1: i64 = 0 as i64;
        let mut ix: i64 = 0 as i64;
        let mut ii: i32 = 0;
        let mut pre: *const i8 = core::ptr::null();
        let mut x__1: i8 = 0 as i8;
        let mut s: FpDecode = unsafe { core::mem::zeroed() };
        let mut i_round: i32 = 0;
        let mut j: i32 = 0;
        let mut sz_buf_needed: i64 = 0 as i64;
        let mut nn__2: i32 = 0;
        let mut nn__3: i32 = 0;
        let mut n_pad: i64 = 0 as i64;
        let mut adj: i32 = 0;
        let mut ch: u32 = 0 as u32;
        let mut n_prior: i64 = 0 as i64;
        let mut n_copy_bytes: i64 = 0 as i64;
        let mut z: *const u8 = core::ptr::null();
        let mut ii__1: i64 = 0 as i64;
        let mut escarg: *mut i8 = core::ptr::null_mut();
        let mut i: i64 = 0 as i64;
        let mut j__1: i64 = 0 as i64;
        let mut px__1: i64 = 0 as i64;
        let mut i_start: i64 = 0 as i64;
        let mut ch__1: u8 = 0 as u8;
        let mut n__1: sqlite3_int64 = 0 as sqlite3_int64;
        let mut len: sqlite3_int64 = 0 as sqlite3_int64;
        let mut zz: *mut i8 = core::ptr::null_mut();
        let mut sp: sqlite3_int64 = 0 as sqlite3_int64;
        let mut i__1: i64 = 0 as i64;
        let mut j__2: i64 = 0 as i64;
        let mut k: i64 = 0 as i64;
        let mut n__2: i64 = 0 as i64;
        let mut need_quote: i32 = 0;
        let mut ch__2: i8 = 0 as i8;
        let mut escarg__1: *mut i8 = core::ptr::null_mut();
        let mut q: i8 = 0 as i8;
        let mut n_back: i64 = 0 as i64;
        let mut n_ctrl: i64 = 0 as i64;
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        let mut p_token: *const Token = core::ptr::null();
        let mut p_item: *const SrcItem = core::ptr::null();
        let mut p_sel: *const Select = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s4:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        if flag_altform2 != 0 && width > 0 as i64 {
                            __state = 490;
                        } else { __state = 489; }
                    }
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
                    16 => { xtype = 19 as etByte; __state = 17; }
                    17 => { __state = 18; }
                    18 => { __state = 19; }
                    19 => { __state = 20; }
                    20 => { __state = 21; }
                    21 => { __state = 22; }
                    22 => { __state = 23; }
                    23 => { __state = 24; }
                    24 => { z_extra = core::ptr::null_mut(); __state = 25; }
                    25 => { __state = 26; }
                    26 => { __state = 27; }
                    27 => { __state = 28; }
                    28 => { p_arg_list = core::ptr::null_mut(); __state = 29; }
                    29 => { __state = 30; }
                    30 => { { let _ = 0; }; __state = 31; }
                    31 => { bufpt = core::ptr::null_mut(); __state = 32; }
                    32 => {
                        if unsafe { (*p_accum_1).printf_flags } as i32 & 2 != 0 {
                            __state = 34;
                        } else { __state = 35; }
                    }
                    33 => { __state = 37; }
                    34 => {
                        p_arg_list =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut PrintfArguments>() +
                                                    7) & !7);
                                *(__va_p as *const *mut PrintfArguments)
                            };
                        __state = 36;
                    }
                    35 => { b_arg_list = 0 as u8; __state = 33; }
                    36 => { b_arg_list = 1 as u8; __state = 33; }
                    37 => {
                        if { c = unsafe { *fmt } as i32; c } != 0 {
                            __state = 38;
                        } else { __state = 1; }
                    }
                    38 => {
                        if c != '%' as i32 { __state = 41; } else { __state = 40; }
                    }
                    39 => {
                        {
                            let __p = &mut fmt;
                            *__p = unsafe { (*__p).offset(1) };
                            *__p
                        };
                        __state = 37;
                    }
                    40 => {
                        if {
                                    c =
                                        unsafe {
                                                *{
                                                        let __p = &mut fmt;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        *__p
                                                    }
                                            } as i32;
                                    c
                                } == 0 {
                            __state = 49;
                        } else { __state = 48; }
                    }
                    41 => { bufpt = fmt as *mut i8; __state = 42; }
                    42 => {
                        fmt = unsafe { strchr(fmt, '%' as i32) } as *const i8;
                        __state = 43;
                    }
                    43 => {
                        if fmt == core::ptr::null() {
                            __state = 45;
                        } else { __state = 44; }
                    }
                    44 => {
                        unsafe {
                            sqlite3_str_append64(p_accum_1, bufpt as *const i8,
                                unsafe { fmt.offset_from(bufpt) } as i64)
                        };
                        __state = 46;
                    }
                    45 => {
                        fmt =
                            unsafe {
                                    bufpt.add(unsafe { strlen(bufpt as *const i8) } as usize)
                                } as *const i8;
                        __state = 44;
                    }
                    46 => {
                        if unsafe { *fmt } as i32 == 0 {
                            __state = 47;
                        } else { __state = 40; }
                    }
                    47 => { __state = 1; }
                    48 => {
                        flag_leftjustify =
                            {
                                flag_prefix =
                                    {
                                        c_thousand =
                                            {
                                                flag_alternateform =
                                                    {
                                                        flag_altform2 =
                                                            { flag_zeropad = 0 as etByte; flag_zeropad };
                                                        flag_altform2
                                                    };
                                                flag_alternateform
                                            };
                                        c_thousand
                                    };
                                flag_prefix
                            };
                        __state = 51;
                    }
                    49 => {
                        unsafe {
                            sqlite3_str_append(p_accum_1,
                                c"%".as_ptr() as *mut i8 as *const i8, 1)
                        };
                        __state = 50;
                    }
                    50 => { __state = 1; }
                    51 => { done = 0 as etByte; __state = 52; }
                    52 => { width = 0 as i64; __state = 53; }
                    53 => { flag_long = 0 as etByte; __state = 54; }
                    54 => { precision = -1 as i64; __state = 55; }
                    55 => {
                        '__s5:
                            {
                            match c {
                                45 => { __state = 58; }
                                43 => { __state = 59; }
                                32 => { __state = 60; }
                                35 => { __state = 61; }
                                33 => { __state = 62; }
                                48 => { __state = 63; }
                                44 => { __state = 64; }
                                108 => { __state = 66; }
                                49 => { __state = 67; }
                                50 => { __state = 68; }
                                51 => { __state = 69; }
                                52 => { __state = 70; }
                                53 => { __state = 71; }
                                54 => { __state = 72; }
                                55 => { __state = 73; }
                                56 => { __state = 74; }
                                57 => { __state = 75; }
                                42 => { __state = 76; }
                                46 => { __state = 77; }
                                _ => { __state = 65; }
                            }
                        }
                    }
                    56 => { { let _ = 0; }; __state = 139; }
                    57 => {
                        if (done == 0) as i32 != 0 &&
                                {
                                        c =
                                            unsafe {
                                                    *{
                                                            let __p = &mut fmt;
                                                            *__p = unsafe { (*__p).offset(1) };
                                                            *__p
                                                        }
                                                } as i32;
                                        c
                                    } != 0 {
                            __state = 55;
                        } else { __state = 56; }
                    }
                    58 => { flag_leftjustify = 1 as etByte; __state = 79; }
                    59 => { flag_prefix = '+' as i32 as etByte; __state = 81; }
                    60 => { flag_prefix = ' ' as i32 as etByte; __state = 83; }
                    61 => { flag_alternateform = 1 as etByte; __state = 85; }
                    62 => { flag_altform2 = 1 as etByte; __state = 87; }
                    63 => { flag_zeropad = 1 as etByte; __state = 89; }
                    64 => { c_thousand = ',' as i32 as etByte; __state = 91; }
                    65 => { done = 1 as etByte; __state = 93; }
                    66 => { flag_long = 1 as etByte; __state = 96; }
                    67 => { __state = 68; }
                    68 => { __state = 69; }
                    69 => { __state = 70; }
                    70 => { __state = 71; }
                    71 => { __state = 72; }
                    72 => { __state = 73; }
                    73 => { __state = 74; }
                    74 => { __state = 75; }
                    75 => { wx = (c - '0' as i32) as u32; __state = 103; }
                    76 => {
                        if b_arg_list != 0 {
                            __state = 114;
                        } else { __state = 115; }
                    }
                    77 => {
                        c =
                            unsafe {
                                    *{
                                            let __p = &mut fmt;
                                            *__p = unsafe { (*__p).offset(1) };
                                            *__p
                                        }
                                } as i32;
                        __state = 122;
                    }
                    78 => { __state = 58; }
                    79 => { __state = 57; }
                    80 => { __state = 59; }
                    81 => { __state = 57; }
                    82 => { __state = 60; }
                    83 => { __state = 57; }
                    84 => { __state = 61; }
                    85 => { __state = 57; }
                    86 => { __state = 62; }
                    87 => { __state = 57; }
                    88 => { __state = 63; }
                    89 => { __state = 57; }
                    90 => { __state = 64; }
                    91 => { __state = 57; }
                    92 => { __state = 65; }
                    93 => { __state = 57; }
                    94 => { __state = 66; }
                    95 => { __state = 67; }
                    96 => {
                        c =
                            unsafe {
                                    *{
                                            let __p = &mut fmt;
                                            *__p = unsafe { (*__p).offset(1) };
                                            *__p
                                        }
                                } as i32;
                        __state = 97;
                    }
                    97 => {
                        if c == 'l' as i32 { __state = 99; } else { __state = 98; }
                    }
                    98 => { done = 1 as etByte; __state = 101; }
                    99 => {
                        c =
                            unsafe {
                                    *{
                                            let __p = &mut fmt;
                                            *__p = unsafe { (*__p).offset(1) };
                                            *__p
                                        }
                                } as i32;
                        __state = 100;
                    }
                    100 => { flag_long = 2 as etByte; __state = 98; }
                    101 => { __state = 57; }
                    102 => { __state = 111; }
                    103 => {
                        if {
                                        c =
                                            unsafe {
                                                    *{
                                                            let __p = &mut fmt;
                                                            *__p = unsafe { (*__p).offset(1) };
                                                            *__p
                                                        }
                                                } as i32;
                                        c
                                    } >= '0' as i32 && c <= '9' as i32 {
                            __state = 105;
                        } else { __state = 104; }
                    }
                    104 => { __state = 106; }
                    105 => {
                        wx = wx * 10 as u32 + c as u32 - '0' as i32 as u32;
                        __state = 103;
                    }
                    106 => {
                        width = (wx & 2147483647 as u32) as i64;
                        __state = 107;
                    }
                    107 => {
                        if c != '.' as i32 && c != 'l' as i32 {
                            __state = 109;
                        } else { __state = 110; }
                    }
                    108 => { __state = 57; }
                    109 => { done = 1 as etByte; __state = 108; }
                    110 => {
                        {
                            let __p = &mut fmt;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(-1) };
                            __t
                        };
                        __state = 108;
                    }
                    111 => { __state = 76; }
                    112 => { __state = 77; }
                    113 => {
                        if width < 0 as i64 {
                            __state = 117;
                        } else { __state = 116; }
                    }
                    114 => {
                        width =
                            get_int_arg(unsafe { &mut *p_arg_list }) as i32 as i64;
                        __state = 113;
                    }
                    115 => {
                        width =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as i64;
                        __state = 113;
                    }
                    116 => {
                        if { c = unsafe { *fmt.offset(1 as isize) } as i32; c } !=
                                    '.' as i32 && c != 'l' as i32 {
                            __state = 120;
                        } else { __state = 119; }
                    }
                    117 => { flag_leftjustify = 1 as etByte; __state = 118; }
                    118 => {
                        width =
                            if width >= -2147483647 as i64 { -width } else { 0 as i64 };
                        __state = 116;
                    }
                    119 => { __state = 57; }
                    120 => {
                        c =
                            unsafe {
                                    *{
                                            let __p = &mut fmt;
                                            *__p = unsafe { (*__p).offset(1) };
                                            *__p
                                        }
                                } as i32;
                        __state = 121;
                    }
                    121 => { done = 1 as etByte; __state = 119; }
                    122 => {
                        if c == '*' as i32 {
                            __state = 124;
                        } else { __state = 125; }
                    }
                    123 => {
                        if c == 'l' as i32 {
                            __state = 137;
                        } else { __state = 138; }
                    }
                    124 => {
                        if b_arg_list != 0 {
                            __state = 127;
                        } else { __state = 128; }
                    }
                    125 => { px = 0 as u32; __state = 131; }
                    126 => {
                        if precision < 0 as i64 {
                            __state = 130;
                        } else { __state = 129; }
                    }
                    127 => {
                        precision =
                            get_int_arg(unsafe { &mut *p_arg_list }) as i32 as i64;
                        __state = 126;
                    }
                    128 => {
                        precision =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as i64;
                        __state = 126;
                    }
                    129 => {
                        c =
                            unsafe {
                                    *{
                                            let __p = &mut fmt;
                                            *__p = unsafe { (*__p).offset(1) };
                                            *__p
                                        }
                                } as i32;
                        __state = 123;
                    }
                    130 => {
                        precision =
                            if precision >= -2147483647 as i64 {
                                -precision
                            } else { -1 as i64 };
                        __state = 129;
                    }
                    131 => {
                        if c >= '0' as i32 && c <= '9' as i32 {
                            __state = 133;
                        } else { __state = 132; }
                    }
                    132 => { __state = 135; }
                    133 => {
                        px = px * 10 as u32 + c as u32 - '0' as i32 as u32;
                        __state = 134;
                    }
                    134 => {
                        c =
                            unsafe {
                                    *{
                                            let __p = &mut fmt;
                                            *__p = unsafe { (*__p).offset(1) };
                                            *__p
                                        }
                                } as i32;
                        __state = 131;
                    }
                    135 => {
                        precision = (px & 2147483647 as u32) as i64;
                        __state = 123;
                    }
                    136 => { __state = 57; }
                    137 => {
                        {
                            let __p = &mut fmt;
                            *__p = unsafe { (*__p).offset(-1) };
                            *__p
                        };
                        __state = 136;
                    }
                    138 => { done = 1 as etByte; __state = 136; }
                    139 => {
                        idx = (c as u32 % 25 as u32) as i32;
                        __state = 140;
                    }
                    140 => {
                        if fmtinfo[idx as usize].fmttype as i32 == c ||
                                fmtinfo[{ idx = fmtinfo[idx as usize].i_nxt as i32; idx } as
                                                    usize].fmttype as i32 == c {
                            __state = 142;
                        } else { __state = 143; }
                    }
                    141 => { { let _ = 0; }; __state = 146; }
                    142 => { infop = &fmtinfo[idx as usize]; __state = 144; }
                    143 => { infop = &fmtinfo[0 as usize]; __state = 145; }
                    144 => {
                        xtype = unsafe { (*infop).type_ } as etByte;
                        __state = 141;
                    }
                    145 => { xtype = 19 as etByte; __state = 141; }
                    146 => { { let _ = 0; }; __state = 147; }
                    147 => {
                        '__s6:
                            {
                            match xtype {
                                13 => { __state = 149; }
                                15 => { __state = 150; }
                                0 => { __state = 151; }
                                16 => { __state = 152; }
                                1 => { __state = 153; }
                                2 => { __state = 154; }
                                3 => { __state = 155; }
                                4 => { __state = 156; }
                                7 => { __state = 157; }
                                8 => { __state = 158; }
                                5 => { __state = 159; }
                                6 => { __state = 160; }
                                17 => { __state = 161; }
                                18 => { __state = 162; }
                                9 => { __state = 163; }
                                10 => { __state = 164; }
                                14 => { __state = 165; }
                                11 => { __state = 166; }
                                12 => { __state = 167; }
                                _ => { __state = 168; }
                            }
                        }
                    }
                    148 => { width -= length; __state = 688; }
                    149 => {
                        flag_long =
                            if core::mem::size_of::<*mut i8>() as u64 ==
                                        core::mem::size_of::<i64>() as u64 {
                                    2
                                } else {
                                    if core::mem::size_of::<*mut i8>() as u64 ==
                                            core::mem::size_of::<i64>() as u64 {
                                        1
                                    } else { 0 }
                                } as etByte;
                        __state = 170;
                    }
                    150 => { __state = 151; }
                    151 => { c_thousand = 0 as etByte; __state = 172; }
                    152 => {
                        if unsafe { (*infop).flags } as i32 & 1 != 0 {
                            __state = 175;
                        } else { __state = 176; }
                    }
                    153 => { __state = 154; }
                    154 => { __state = 155; }
                    155 => { __state = 260; }
                    156 => {
                        if (b_arg_list == 0) as i32 != 0 {
                            __state = 420;
                        } else { __state = 419; }
                    }
                    157 => {
                        buf[0 as usize] = '%' as i32 as i8;
                        __state = 423;
                    }
                    158 => {
                        if b_arg_list != 0 {
                            __state = 428;
                        } else { __state = 429; }
                    }
                    159 => { __state = 160; }
                    160 => {
                        if b_arg_list != 0 {
                            __state = 460;
                        } else { __state = 461; }
                    }
                    161 => { __state = 162; }
                    162 => { __state = 496; }
                    163 => { __state = 164; }
                    164 => { __state = 165; }
                    165 => { __state = 565; }
                    166 => {
                        if unsafe { (*p_accum_1).printf_flags } as i32 & 1 == 0 {
                            __state = 650;
                        } else { __state = 649; }
                    }
                    167 => { __state = 663; }
                    168 => { { let _ = 0; }; __state = 687; }
                    169 => { __state = 149; }
                    170 => { __state = 171; }
                    171 => { __state = 150; }
                    172 => { __state = 173; }
                    173 => { __state = 152; }
                    174 => {
                        if longvalue == 0 as u64 {
                            __state = 200;
                        } else { __state = 199; }
                    }
                    175 => { __state = 177; }
                    176 => {
                        if b_arg_list != 0 {
                            __state = 193;
                        } else { __state = 194; }
                    }
                    177 => {
                        if b_arg_list != 0 {
                            __state = 179;
                        } else { __state = 180; }
                    }
                    178 => {
                        if v < 0 as i64 { __state = 185; } else { __state = 186; }
                    }
                    179 => {
                        v = get_int_arg(unsafe { &mut *p_arg_list });
                        __state = 178;
                    }
                    180 => {
                        if flag_long != 0 { __state = 181; } else { __state = 182; }
                    }
                    181 => {
                        if flag_long as i32 == 2 {
                            __state = 183;
                        } else { __state = 184; }
                    }
                    182 => {
                        v =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                                    *(__va_p as *const i32)
                                } as i64;
                        __state = 178;
                    }
                    183 => {
                        v =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<i64>() + 7) & !7);
                                *(__va_p as *const i64)
                            };
                        __state = 178;
                    }
                    184 => {
                        v =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<i64>() + 7) & !7);
                                    *(__va_p as *const i64)
                                } as i64;
                        __state = 178;
                    }
                    185 => { __state = 187; }
                    186 => { longvalue = v as sqlite_uint64; __state = 191; }
                    187 => { __state = 188; }
                    188 => { longvalue = !v as sqlite_uint64; __state = 189; }
                    189 => {
                        {
                            let __p = &mut longvalue;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 190;
                    }
                    190 => { prefix = '-' as i32 as i8; __state = 174; }
                    191 => { prefix = flag_prefix as i8; __state = 174; }
                    192 => { prefix = 0 as i8; __state = 174; }
                    193 => {
                        longvalue = get_int_arg(unsafe { &mut *p_arg_list }) as u64;
                        __state = 192;
                    }
                    194 => {
                        if flag_long != 0 { __state = 195; } else { __state = 196; }
                    }
                    195 => {
                        if flag_long as i32 == 2 {
                            __state = 197;
                        } else { __state = 198; }
                    }
                    196 => {
                        longvalue =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                    *(__va_p as *const u32)
                                } as sqlite_uint64;
                        __state = 192;
                    }
                    197 => {
                        longvalue =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u64>() + 7) & !7);
                                *(__va_p as *const u64)
                            };
                        __state = 192;
                    }
                    198 => {
                        longvalue =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap = (*__ap).add((core::mem::size_of::<u64>() + 7) & !7);
                                    *(__va_p as *const u64)
                                } as sqlite_uint64;
                        __state = 192;
                    }
                    199 => {
                        if flag_zeropad != 0 &&
                                precision < width - (prefix as i32 != 0) as i64 {
                            __state = 202;
                        } else { __state = 201; }
                    }
                    200 => { flag_alternateform = 0 as etByte; __state = 199; }
                    201 => {
                        if precision < (70 - 10 - 70 / 3) as i64 {
                            __state = 204;
                        } else { __state = 205; }
                    }
                    202 => {
                        precision = width - (prefix as i32 != 0) as i64;
                        __state = 201;
                    }
                    203 => {
                        bufpt = unsafe { z_out.offset((n_out - 1) as isize) };
                        __state = 214;
                    }
                    204 => { n_out = 70; __state = 206; }
                    205 => { __state = 207; }
                    206 => {
                        z_out = &raw mut buf[0 as usize] as *mut i8;
                        __state = 203;
                    }
                    207 => { n = precision as u64 + 10 as u64; __state = 208; }
                    208 => {
                        if c_thousand != 0 {
                            __state = 210;
                        } else { __state = 209; }
                    }
                    209 => {
                        z_out =
                            {
                                z_extra = printf_temp_buf(p_accum_1, n as sqlite3_int64);
                                z_extra
                            };
                        __state = 211;
                    }
                    210 => {
                        n += (precision / 3 as i64) as u64;
                        __state = 209;
                    }
                    211 => {
                        if z_out == core::ptr::null_mut() {
                            __state = 213;
                        } else { __state = 212; }
                    }
                    212 => { n_out = n as i32; __state = 203; }
                    213 => { return; }
                    214 => {
                        if xtype as i32 == 15 {
                            __state = 216;
                        } else { __state = 215; }
                    }
                    215 => {
                        cset = &a_digits[unsafe { (*infop).charset } as usize];
                        __state = 223;
                    }
                    216 => { __state = 217; }
                    217 => {
                        x = (longvalue % 10 as sqlite_uint64) as i32;
                        __state = 218;
                    }
                    218 => {
                        if x >= 4 ||
                                longvalue / 10 as sqlite_uint64 % 10 as sqlite_uint64 ==
                                    1 as u64 {
                            __state = 220;
                        } else { __state = 219; }
                    }
                    219 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        *__p = unsafe { (*__p).offset(-1) };
                                        *__p
                                    } = z_ord[(x * 2 + 1) as usize] as i8
                        };
                        __state = 221;
                    }
                    220 => { x = 0; __state = 219; }
                    221 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        *__p = unsafe { (*__p).offset(-1) };
                                        *__p
                                    } = z_ord[(x * 2) as usize] as i8
                        };
                        __state = 215;
                    }
                    222 => {
                        length =
                            unsafe {
                                        unsafe {
                                            z_out.offset((n_out - 1) as isize).offset_from(bufpt)
                                        }
                                    } as i64 as i64;
                        __state = 227;
                    }
                    223 => {
                        base = unsafe { (*infop).base } as etByte;
                        __state = 224;
                    }
                    224 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        *__p = unsafe { (*__p).offset(-1) };
                                        *__p
                                    } =
                                unsafe {
                                        *cset.add((longvalue % base as sqlite_uint64) as usize)
                                    } as i8
                        };
                        __state = 226;
                    }
                    225 => {
                        if longvalue > 0 as u64 {
                            __state = 224;
                        } else { __state = 222; }
                    }
                    226 => {
                        longvalue = longvalue / base as sqlite_uint64;
                        __state = 225;
                    }
                    227 => {
                        if precision > length {
                            __state = 229;
                        } else { __state = 228; }
                    }
                    228 => {
                        if c_thousand != 0 {
                            __state = 234;
                        } else { __state = 233; }
                    }
                    229 => { nn = precision - length; __state = 230; }
                    230 => {
                        {
                            let __n = nn;
                            let __p = &mut bufpt;
                            *__p = unsafe { (*__p).offset(-(__n as isize)) };
                        };
                        __state = 231;
                    }
                    231 => {
                        unsafe { memset(bufpt as *mut (), '0' as i32, nn as u64) };
                        __state = 232;
                    }
                    232 => { length = precision; __state = 228; }
                    233 => {
                        if prefix != 0 { __state = 248; } else { __state = 247; }
                    }
                    234 => {
                        nn__1 = (length - 1 as i64) / 3 as i64;
                        __state = 235;
                    }
                    235 => {
                        ix = (length - 1 as i64) % 3 as i64 + 1 as i64;
                        __state = 236;
                    }
                    236 => { __state = 237; }
                    237 => {
                        {
                            let __n = nn__1;
                            let __p = &mut bufpt;
                            *__p = unsafe { (*__p).offset(-(__n as isize)) };
                        };
                        __state = 238;
                    }
                    238 => { ii = 0; __state = 239; }
                    239 => {
                        if nn__1 > 0 as i64 {
                            __state = 240;
                        } else { __state = 233; }
                    }
                    240 => {
                        unsafe {
                            *bufpt.offset(ii as isize) =
                                unsafe { *bufpt.offset((ii as i64 + nn__1) as isize) }
                        };
                        __state = 242;
                    }
                    241 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 239;
                    }
                    242 => {
                        { let __p = &mut ix; let __t = *__p; *__p -= 1; __t };
                        __state = 243;
                    }
                    243 => {
                        if ix == 0 as i64 { __state = 244; } else { __state = 241; }
                    }
                    244 => {
                        unsafe {
                            *bufpt.offset({ let __p = &mut ii; *__p += 1; *__p } as
                                            isize) = c_thousand as i8
                        };
                        __state = 245;
                    }
                    245 => {
                        { let __p = &mut nn__1; let __t = *__p; *__p -= 1; __t };
                        __state = 246;
                    }
                    246 => { ix = 3 as i64; __state = 241; }
                    247 => {
                        if flag_alternateform != 0 &&
                                unsafe { (*infop).prefix } != 0 {
                            __state = 250;
                        } else { __state = 249; }
                    }
                    248 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        *__p = unsafe { (*__p).offset(-1) };
                                        *__p
                                    } = prefix
                        };
                        __state = 247;
                    }
                    249 => {
                        length =
                            unsafe {
                                        unsafe {
                                            z_out.offset((n_out - 1) as isize).offset_from(bufpt)
                                        }
                                    } as i64 as i64;
                        __state = 257;
                    }
                    250 => { __state = 251; }
                    251 => { __state = 252; }
                    252 => {
                        pre = &a_prefix[unsafe { (*infop).prefix } as usize];
                        __state = 253;
                    }
                    253 => { __state = 254; }
                    254 => {
                        if { x__1 = unsafe { *pre } as i8; x__1 } as i32 != 0 {
                            __state = 255;
                        } else { __state = 249; }
                    }
                    255 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        *__p = unsafe { (*__p).offset(-1) };
                                        *__p
                                    } = x__1
                        };
                        __state = 256;
                    }
                    256 => {
                        {
                            let __p = &mut pre;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 254;
                    }
                    257 => { __state = 148; }
                    258 => { __state = 153; }
                    259 => { __state = 418; }
                    260 => { __state = 261; }
                    261 => { __state = 262; }
                    262 => { __state = 263; }
                    263 => {
                        if b_arg_list != 0 {
                            __state = 265;
                        } else { __state = 266; }
                    }
                    264 => {
                        if precision < 0 as i64 {
                            __state = 268;
                        } else { __state = 267; }
                    }
                    265 => {
                        realvalue = get_double_arg(unsafe { &mut *p_arg_list });
                        __state = 264;
                    }
                    266 => {
                        realvalue =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<f64>() + 7) & !7);
                                *(__va_p as *const f64)
                            };
                        __state = 264;
                    }
                    267 => {
                        if precision > 100000000 as i64 {
                            __state = 270;
                        } else { __state = 269; }
                    }
                    268 => { precision = 6 as i64; __state = 267; }
                    269 => {
                        if xtype as i32 == 1 {
                            __state = 272;
                        } else { __state = 273; }
                    }
                    270 => { precision = 100000000 as i64; __state = 269; }
                    271 => {
                        unsafe {
                            sqlite3_fp_decode(&mut s, realvalue, i_round,
                                if flag_altform2 != 0 { 20 } else { 16 })
                        };
                        __state = 278;
                    }
                    272 => { i_round = -precision as i32; __state = 271; }
                    273 => {
                        if xtype as i32 == 3 {
                            __state = 274;
                        } else { __state = 275; }
                    }
                    274 => {
                        if precision == 0 as i64 {
                            __state = 277;
                        } else { __state = 276; }
                    }
                    275 => {
                        i_round = (precision + 1 as i64) as i32;
                        __state = 271;
                    }
                    276 => { i_round = precision as i32; __state = 271; }
                    277 => { precision = 1 as i64; __state = 276; }
                    278 => {
                        if s.is_special != 0 {
                            __state = 280;
                        } else { __state = 279; }
                    }
                    279 => {
                        if s.sign as i32 == '-' as i32 {
                            __state = 301;
                        } else { __state = 302; }
                    }
                    280 => {
                        if s.is_special as i32 == 2 {
                            __state = 281;
                        } else { __state = 282; }
                    }
                    281 => {
                        if flag_zeropad != 0 {
                            __state = 284;
                        } else { __state = 285; }
                    }
                    282 => {
                        if flag_zeropad != 0 {
                            __state = 288;
                        } else { __state = 289; }
                    }
                    283 => { __state = 148; }
                    284 => {
                        bufpt = c"null".as_ptr() as *mut i8;
                        __state = 286;
                    }
                    285 => {
                        bufpt = c"NaN".as_ptr() as *mut i8;
                        __state = 287;
                    }
                    286 => { length = 4 as i64; __state = 283; }
                    287 => { length = 3 as i64; __state = 283; }
                    288 => {
                        unsafe { *s.z.offset(0 as isize) = '9' as i32 as i8 };
                        __state = 290;
                    }
                    289 => {
                        unsafe {
                            memcpy(&raw mut buf[0 as usize] as *mut i8 as *mut (),
                                c"-Inf".as_ptr() as *mut i8 as *const (), 5 as u64)
                        };
                        __state = 292;
                    }
                    290 => { s.i_dp = 1000; __state = 291; }
                    291 => { s.n = 1; __state = 279; }
                    292 => {
                        bufpt = &raw mut buf[0 as usize] as *mut i8;
                        __state = 293;
                    }
                    293 => {
                        if s.sign as i32 == '-' as i32 {
                            __state = 295;
                        } else { __state = 296; }
                    }
                    294 => {
                        length = unsafe { strlen(bufpt as *const i8) } as i64;
                        __state = 299;
                    }
                    295 => { __state = 294; }
                    296 => {
                        if flag_prefix != 0 {
                            __state = 297;
                        } else { __state = 298; }
                    }
                    297 => {
                        buf[0 as usize] = flag_prefix as i8;
                        __state = 294;
                    }
                    298 => {
                        {
                            let __p = &mut bufpt;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 294;
                    }
                    299 => { __state = 148; }
                    300 => { exp = s.i_dp - 1; __state = 305; }
                    301 => {
                        if flag_alternateform != 0 && (flag_prefix == 0) as i32 != 0
                                    && xtype as i32 == 1 && s.i_dp <= i_round {
                            __state = 303;
                        } else { __state = 304; }
                    }
                    302 => { prefix = flag_prefix as i8; __state = 300; }
                    303 => { prefix = 0 as i8; __state = 300; }
                    304 => { prefix = '-' as i32 as i8; __state = 300; }
                    305 => {
                        if xtype as i32 == 3 {
                            __state = 307;
                        } else { __state = 308; }
                    }
                    306 => {
                        if xtype as i32 == 2 {
                            __state = 316;
                        } else { __state = 317; }
                    }
                    307 => { { let _ = 0; }; __state = 309; }
                    308 => { flag_rtz = flag_altform2; __state = 306; }
                    309 => {
                        {
                            let __p = &mut precision;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 310;
                    }
                    310 => {
                        flag_rtz = (flag_alternateform == 0) as i32 as etByte;
                        __state = 311;
                    }
                    311 => {
                        if exp < -4 || exp as i64 > precision {
                            __state = 312;
                        } else { __state = 313; }
                    }
                    312 => { xtype = 2 as etByte; __state = 306; }
                    313 => {
                        precision = precision - exp as i64;
                        __state = 314;
                    }
                    314 => { xtype = 1 as etByte; __state = 306; }
                    315 => {
                        sz_buf_needed =
                            if e2 > 0 { e2 } else { 0 } as i64 + precision as i64 +
                                    width as i64 + 10 as i64;
                        __state = 318;
                    }
                    316 => { e2 = 0; __state = 315; }
                    317 => { e2 = s.i_dp - 1; __state = 315; }
                    318 => {
                        if c_thousand != 0 && e2 > 0 {
                            __state = 320;
                        } else { __state = 319; }
                    }
                    319 => {
                        if sz_buf_needed + unsafe { (*p_accum_1).n_char } as i64 >=
                                unsafe { (*p_accum_1).n_alloc } as i64 {
                            __state = 322;
                        } else { __state = 323; }
                    }
                    320 => {
                        sz_buf_needed += ((e2 + 2) / 3) as i64;
                        __state = 319;
                    }
                    321 => { z_out = bufpt; __state = 333; }
                    322 => {
                        if unsafe { (*p_accum_1).mx_alloc } == 0 as u32 &&
                                unsafe { (*p_accum_1).acc_error } as i32 == 0 {
                            __state = 324;
                        } else { __state = 325; }
                    }
                    323 => {
                        bufpt =
                            unsafe {
                                unsafe {
                                    (*p_accum_1).z_text.add(unsafe { (*p_accum_1).n_char } as
                                            usize)
                                }
                            };
                        __state = 321;
                    }
                    324 => {
                        bufpt =
                            unsafe { sqlite3_malloc(sz_buf_needed as i32) } as *mut i8;
                        __state = 326;
                    }
                    325 => {
                        if (unsafe {
                                            sqlite3_str_accum_enlarge(p_accum_1 as *mut StrAccum,
                                                sz_buf_needed)
                                        } as i64) < sz_buf_needed {
                            __state = 330;
                        } else { __state = 331; }
                    }
                    326 => {
                        if bufpt == core::ptr::null_mut() {
                            __state = 328;
                        } else { __state = 327; }
                    }
                    327 => { z_extra = bufpt; __state = 321; }
                    328 => {
                        sqlite3_str_accum_set_error(p_accum_1 as *mut StrAccum,
                            7 as u8);
                        __state = 329;
                    }
                    329 => { return; }
                    330 => {
                        width = { length = 0 as i64; length };
                        __state = 332;
                    }
                    331 => {
                        bufpt =
                            unsafe {
                                unsafe {
                                    (*p_accum_1).z_text.add(unsafe { (*p_accum_1).n_char } as
                                            usize)
                                }
                            };
                        __state = 321;
                    }
                    332 => { __state = 148; }
                    333 => {
                        flag_dp =
                            (if precision > 0 as i64 { 1 } else { 0 } |
                                        flag_alternateform as i32 | flag_altform2 as i32) as etByte;
                        __state = 334;
                    }
                    334 => {
                        if prefix != 0 { __state = 336; } else { __state = 335; }
                    }
                    335 => { j = 0; __state = 337; }
                    336 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = prefix
                        };
                        __state = 335;
                    }
                    337 => { { let _ = 0; }; __state = 338; }
                    338 => {
                        if e2 < 0 { __state = 340; } else { __state = 341; }
                    }
                    339 => {
                        if flag_dp != 0 { __state = 359; } else { __state = 358; }
                    }
                    340 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = '0' as i32 as i8
                        };
                        __state = 339;
                    }
                    341 => {
                        if c_thousand != 0 {
                            __state = 342;
                        } else { __state = 343; }
                    }
                    342 => { __state = 344; }
                    343 => { j = e2 + 1; __state = 349; }
                    344 => {
                        if e2 >= 0 { __state = 345; } else { __state = 339; }
                    }
                    345 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } =
                                if j < s.n {
                                        (unsafe {
                                                *s.z.offset({
                                                                let __p = &mut j;
                                                                let __t = *__p;
                                                                *__p += 1;
                                                                __t
                                                            } as isize)
                                            }) as i32
                                    } else { '0' as i32 } as i8
                        };
                        __state = 347;
                    }
                    346 => {
                        { let __p = &mut e2; let __t = *__p; *__p -= 1; __t };
                        __state = 344;
                    }
                    347 => {
                        if e2 % 3 == 0 && e2 > 1 {
                            __state = 348;
                        } else { __state = 346; }
                    }
                    348 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = ',' as i32 as i8
                        };
                        __state = 346;
                    }
                    349 => {
                        if j > s.n { __state = 351; } else { __state = 350; }
                    }
                    350 => {
                        unsafe {
                            memcpy(bufpt as *mut (), s.z as *const (), j as u64)
                        };
                        __state = 352;
                    }
                    351 => { j = s.n; __state = 350; }
                    352 => {
                        {
                            let __n = j;
                            let __p = &mut bufpt;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 353;
                    }
                    353 => { e2 -= j; __state = 354; }
                    354 => {
                        if e2 >= 0 { __state = 355; } else { __state = 339; }
                    }
                    355 => {
                        unsafe {
                            memset(bufpt as *mut (), '0' as i32, (e2 + 1) as u64)
                        };
                        __state = 356;
                    }
                    356 => {
                        {
                            let __n = e2 + 1;
                            let __p = &mut bufpt;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 357;
                    }
                    357 => { e2 = -1; __state = 339; }
                    358 => {
                        if e2 < -1 && precision > 0 as i64 {
                            __state = 361;
                        } else { __state = 360; }
                    }
                    359 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = '.' as i32 as i8
                        };
                        __state = 358;
                    }
                    360 => {
                        if precision > 0 as i64 {
                            __state = 368;
                        } else { __state = 367; }
                    }
                    361 => { nn__2 = -1 - e2; __state = 362; }
                    362 => {
                        if nn__2 as i64 > precision {
                            __state = 364;
                        } else { __state = 363; }
                    }
                    363 => {
                        unsafe {
                            memset(bufpt as *mut (), '0' as i32, nn__2 as u64)
                        };
                        __state = 365;
                    }
                    364 => { nn__2 = precision as i32; __state = 363; }
                    365 => {
                        {
                            let __n = nn__2;
                            let __p = &mut bufpt;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 366;
                    }
                    366 => { precision -= nn__2 as i64; __state = 360; }
                    367 => {
                        if flag_rtz != 0 && flag_dp != 0 {
                            __state = 379;
                        } else { __state = 378; }
                    }
                    368 => { nn__3 = s.n - j; __state = 369; }
                    369 => {
                        if nn__3 as i64 > precision {
                            __state = 371;
                        } else { __state = 370; }
                    }
                    370 => {
                        if nn__3 > 0 { __state = 373; } else { __state = 372; }
                    }
                    371 => { nn__3 = precision as i32; __state = 370; }
                    372 => {
                        if precision > 0 as i64 && (flag_rtz == 0) as i32 != 0 {
                            __state = 376;
                        } else { __state = 367; }
                    }
                    373 => {
                        unsafe {
                            memcpy(bufpt as *mut (),
                                unsafe { s.z.offset(j as isize) } as *const (),
                                nn__3 as u64)
                        };
                        __state = 374;
                    }
                    374 => {
                        {
                            let __n = nn__3;
                            let __p = &mut bufpt;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 375;
                    }
                    375 => { precision -= nn__3 as i64; __state = 372; }
                    376 => {
                        unsafe {
                            memset(bufpt as *mut (), '0' as i32, precision as u64)
                        };
                        __state = 377;
                    }
                    377 => {
                        {
                            let __n = precision;
                            let __p = &mut bufpt;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 367;
                    }
                    378 => {
                        if xtype as i32 == 2 {
                            __state = 387;
                        } else { __state = 386; }
                    }
                    379 => {
                        if unsafe { *bufpt.offset(-1 as isize) } as i32 ==
                                '0' as i32 {
                            __state = 381;
                        } else { __state = 380; }
                    }
                    380 => { { let _ = 0; }; __state = 382; }
                    381 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        *__p = unsafe { (*__p).offset(-1) };
                                        *__p
                                    } = 0 as i8
                        };
                        __state = 379;
                    }
                    382 => {
                        if unsafe { *bufpt.offset(-1 as isize) } as i32 ==
                                '.' as i32 {
                            __state = 383;
                        } else { __state = 378; }
                    }
                    383 => {
                        if flag_altform2 != 0 {
                            __state = 384;
                        } else { __state = 385; }
                    }
                    384 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = '0' as i32 as i8
                        };
                        __state = 378;
                    }
                    385 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        *__p = unsafe { (*__p).offset(-1) };
                                        *__p
                                    } = 0 as i8
                        };
                        __state = 378;
                    }
                    386 => {
                        length = unsafe { bufpt.offset_from(z_out) } as i64 as i64;
                        __state = 398;
                    }
                    387 => { exp = s.i_dp - 1; __state = 388; }
                    388 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = a_digits[unsafe { (*infop).charset } as usize] as i8
                        };
                        __state = 389;
                    }
                    389 => {
                        if exp < 0 { __state = 391; } else { __state = 392; }
                    }
                    390 => {
                        if exp >= 100 { __state = 395; } else { __state = 394; }
                    }
                    391 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = '-' as i32 as i8
                        };
                        __state = 393;
                    }
                    392 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = '+' as i32 as i8
                        };
                        __state = 390;
                    }
                    393 => { exp = -exp; __state = 390; }
                    394 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (exp / 10 + '0' as i32) as i8
                        };
                        __state = 397;
                    }
                    395 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (exp / 100 + '0' as i32) as i8
                        };
                        __state = 396;
                    }
                    396 => { exp %= 100; __state = 394; }
                    397 => {
                        unsafe {
                            *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    } = (exp % 10 + '0' as i32) as i8
                        };
                        __state = 386;
                    }
                    398 => { { let _ = 0; }; __state = 399; }
                    399 => {
                        if length < width { __state = 401; } else { __state = 400; }
                    }
                    400 => {
                        if z_extra == core::ptr::null_mut() {
                            __state = 411;
                        } else { __state = 412; }
                    }
                    401 => { n_pad = width - length; __state = 402; }
                    402 => {
                        if flag_leftjustify != 0 {
                            __state = 404;
                        } else { __state = 405; }
                    }
                    403 => { length = width; __state = 400; }
                    404 => {
                        unsafe {
                            memset(bufpt as *mut (), ' ' as i32, n_pad as u64)
                        };
                        __state = 403;
                    }
                    405 => {
                        if (flag_zeropad == 0) as i32 != 0 {
                            __state = 406;
                        } else { __state = 407; }
                    }
                    406 => {
                        unsafe {
                            memmove(unsafe { z_out.offset(n_pad as isize) } as *mut (),
                                z_out as *const (), length as u64)
                        };
                        __state = 408;
                    }
                    407 => { adj = (prefix as i32 != 0) as i32; __state = 409; }
                    408 => {
                        unsafe {
                            memset(z_out as *mut (), ' ' as i32, n_pad as u64)
                        };
                        __state = 403;
                    }
                    409 => {
                        unsafe {
                            memmove(unsafe {
                                        unsafe { z_out.offset(n_pad as isize).offset(adj as isize) }
                                    } as *mut (),
                                unsafe { z_out.offset(adj as isize) } as *const (),
                                (length - adj as i64) as u64)
                        };
                        __state = 410;
                    }
                    410 => {
                        unsafe {
                            memset(unsafe { z_out.offset(adj as isize) } as *mut (),
                                '0' as i32, n_pad as u64)
                        };
                        __state = 403;
                    }
                    411 => { { let _ = 0; }; __state = 413; }
                    412 => {
                        unsafe { *bufpt.offset(0 as isize) = 0 as i8 };
                        __state = 416;
                    }
                    413 => {
                        unsafe { (*p_accum_1).n_char += length as u32 };
                        __state = 414;
                    }
                    414 => {
                        unsafe { *z_out.offset(length as isize) = 0 as i8 };
                        __state = 415;
                    }
                    415 => { __state = 39; }
                    416 => { bufpt = z_extra; __state = 417; }
                    417 => { __state = 148; }
                    418 => { __state = 156; }
                    419 => {
                        length = { width = 0 as i64; width };
                        __state = 421;
                    }
                    420 => {
                        unsafe {
                            *unsafe {
                                        let __ap = &mut ap;
                                        let __va_p = *__ap;
                                        *__ap =
                                            (*__ap).add((core::mem::size_of::<*mut i32>() + 7) & !7);
                                        *(__va_p as *const *mut i32)
                                    } = unsafe { (*p_accum_1).n_char } as i32
                        };
                        __state = 419;
                    }
                    421 => { __state = 148; }
                    422 => { __state = 157; }
                    423 => {
                        bufpt = &raw mut buf[0 as usize] as *mut i8;
                        __state = 424;
                    }
                    424 => { length = 1 as i64; __state = 425; }
                    425 => { __state = 148; }
                    426 => { __state = 158; }
                    427 => {
                        if precision > 1 as i64 {
                            __state = 439;
                        } else { __state = 438; }
                    }
                    428 => {
                        bufpt = get_text_arg(unsafe { &mut *p_arg_list });
                        __state = 430;
                    }
                    429 => {
                        ch =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap = (*__ap).add((core::mem::size_of::<u32>() + 7) & !7);
                                *(__va_p as *const u32)
                            };
                        __state = 437;
                    }
                    430 => { length = 1 as i64; __state = 431; }
                    431 => {
                        if !(bufpt).is_null() {
                            __state = 432;
                        } else { __state = 433; }
                    }
                    432 => {
                        buf[0 as usize] =
                            {
                                    c =
                                        unsafe {
                                                *{
                                                        let __p = &mut bufpt;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            } as i32;
                                    c
                                } as i8;
                        __state = 434;
                    }
                    433 => { buf[0 as usize] = 0 as i8; __state = 427; }
                    434 => {
                        if c & 192 == 192 { __state = 435; } else { __state = 427; }
                    }
                    435 => {
                        if length < 4 as i64 &&
                                unsafe { *bufpt.offset(0 as isize) } as i32 & 192 == 128 {
                            __state = 436;
                        } else { __state = 427; }
                    }
                    436 => {
                        buf[{
                                        let __p = &mut length;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as usize] =
                            unsafe {
                                *{
                                        let __p = &mut bufpt;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            };
                        __state = 435;
                    }
                    437 => {
                        length =
                            unsafe {
                                    sqlite3_append_one_utf8_character(&raw mut buf[0 as usize]
                                            as *mut i8, ch)
                                } as i64;
                        __state = 427;
                    }
                    438 => {
                        bufpt = &raw mut buf[0 as usize] as *mut i8;
                        __state = 456;
                    }
                    439 => { n_prior = 1 as i64; __state = 440; }
                    440 => { width -= precision - 1 as i64; __state = 441; }
                    441 => {
                        if width > 1 as i64 && (flag_leftjustify == 0) as i32 != 0 {
                            __state = 443;
                        } else { __state = 442; }
                    }
                    442 => {
                        unsafe {
                            sqlite3_str_append64(p_accum_1,
                                &raw mut buf[0 as usize] as *mut i8 as *const i8, length)
                        };
                        __state = 445;
                    }
                    443 => {
                        unsafe {
                            sqlite3_str_appendchar64(p_accum_1, width - 1 as i64,
                                ' ' as i32 as i8)
                        };
                        __state = 444;
                    }
                    444 => { width = 0 as i64; __state = 442; }
                    445 => {
                        {
                            let __p = &mut precision;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 446;
                    }
                    446 => {
                        if precision > 1 as i64 {
                            __state = 447;
                        } else { __state = 438; }
                    }
                    447 => { __state = 448; }
                    448 => {
                        if n_prior > precision - 1 as i64 {
                            __state = 450;
                        } else { __state = 449; }
                    }
                    449 => { n_copy_bytes = length * n_prior; __state = 451; }
                    450 => { n_prior = precision - 1 as i64; __state = 449; }
                    451 => {
                        if unsafe {
                                    sqlite3_str_accum_enlarge_if_needed(p_accum_1 as
                                            *mut StrAccum, n_copy_bytes)
                                } != 0 {
                            __state = 453;
                        } else { __state = 452; }
                    }
                    452 => {
                        unsafe {
                            sqlite3_str_append(p_accum_1,
                                unsafe {
                                        &raw mut *unsafe {
                                                    (*p_accum_1).z_text.offset((unsafe { (*p_accum_1).n_char }
                                                                    as i64 - n_copy_bytes) as isize)
                                                }
                                    } as *const i8, n_copy_bytes as i32)
                        };
                        __state = 454;
                    }
                    453 => { __state = 438; }
                    454 => { precision -= n_prior; __state = 455; }
                    455 => { n_prior *= 2 as i64; __state = 446; }
                    456 => { flag_altform2 = 1 as etByte; __state = 457; }
                    457 => { __state = 2; }
                    458 => { __state = 159; }
                    459 => {
                        if bufpt == core::ptr::null_mut() {
                            __state = 464;
                        } else { __state = 465; }
                    }
                    460 => {
                        bufpt = get_text_arg(unsafe { &mut *p_arg_list });
                        __state = 462;
                    }
                    461 => {
                        bufpt =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i8>() + 7) & !7);
                                *(__va_p as *const *mut i8)
                            };
                        __state = 459;
                    }
                    462 => { xtype = 5 as etByte; __state = 459; }
                    463 => {
                        if precision >= 0 as i64 {
                            __state = 476;
                        } else { __state = 477; }
                    }
                    464 => { bufpt = c"".as_ptr() as *mut i8; __state = 463; }
                    465 => {
                        if xtype as i32 == 6 {
                            __state = 466;
                        } else { __state = 463; }
                    }
                    466 => {
                        if unsafe { (*p_accum_1).n_char } == 0 as u32 &&
                                            unsafe { (*p_accum_1).mx_alloc } != 0 && width == 0 as i64
                                    && precision < 0 as i64 &&
                                unsafe { (*p_accum_1).acc_error } as i32 == 0 {
                            __state = 468;
                        } else { __state = 467; }
                    }
                    467 => { z_extra = bufpt; __state = 463; }
                    468 => { { let _ = 0; }; __state = 469; }
                    469 => {
                        unsafe { (*p_accum_1).z_text = bufpt };
                        __state = 470;
                    }
                    470 => {
                        unsafe {
                            (*p_accum_1).n_alloc =
                                unsafe {
                                        sqlite3_db_malloc_size(unsafe { (*p_accum_1).db },
                                            bufpt as *const ())
                                    } as u32
                        };
                        __state = 471;
                    }
                    471 => {
                        unsafe {
                            (*p_accum_1).n_char =
                                (2147483647 & unsafe { strlen(bufpt as *const i8) } as i32)
                                    as u32
                        };
                        __state = 472;
                    }
                    472 => {
                        unsafe { (*p_accum_1).printf_flags |= 4 as u8 };
                        __state = 473;
                    }
                    473 => { length = 0 as i64; __state = 474; }
                    474 => { __state = 148; }
                    475 => { __state = 2; }
                    476 => {
                        if flag_altform2 != 0 {
                            __state = 478;
                        } else { __state = 479; }
                    }
                    477 => {
                        length = unsafe { strlen(bufpt as *const i8) } as i64;
                        __state = 475;
                    }
                    478 => { z = bufpt as *mut u8 as *const u8; __state = 480; }
                    479 => { length = 0 as i64; __state = 486; }
                    480 => {
                        if {
                                        let __p = &mut precision;
                                        let __t = *__p;
                                        *__p -= 1;
                                        __t
                                    } > 0 as i64 && unsafe { *z.offset(0 as isize) } != 0 {
                            __state = 482;
                        } else { __state = 481; }
                    }
                    481 => {
                        length =
                            unsafe { z.offset_from(bufpt as *mut u8) } as i64 as i64;
                        __state = 475;
                    }
                    482 => {
                        if unsafe {
                                        *{
                                                let __p = &mut z;
                                                let __t = *__p;
                                                *__p = unsafe { (*__p).offset(1) };
                                                __t
                                            }
                                    } as i32 >= 192 {
                            __state = 484;
                        } else { __state = 483; }
                    }
                    483 => { __state = 480; }
                    484 => {
                        if unsafe { *z } as i32 & 192 == 128 {
                            __state = 485;
                        } else { __state = 483; }
                    }
                    485 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 484;
                    }
                    486 => {
                        if length < precision &&
                                unsafe { *bufpt.offset(length as isize) } != 0 {
                            __state = 487;
                        } else { __state = 475; }
                    }
                    487 => { __state = 488; }
                    488 => {
                        { let __p = &mut length; let __t = *__p; *__p += 1; __t };
                        __state = 486;
                    }
                    489 => { __state = 148; }
                    490 => { ii__1 = length - 1 as i64; __state = 491; }
                    491 => {
                        if ii__1 >= 0 as i64 {
                            __state = 492;
                        } else { __state = 489; }
                    }
                    492 => {
                        if unsafe {
                                            *bufpt.offset({
                                                            let __p = &mut ii__1;
                                                            let __t = *__p;
                                                            *__p -= 1;
                                                            __t
                                                        } as isize)
                                        } as i32 & 192 == 128 {
                            __state = 493;
                        } else { __state = 491; }
                    }
                    493 => {
                        { let __p = &mut width; let __t = *__p; *__p += 1; __t };
                        __state = 491;
                    }
                    494 => { __state = 161; }
                    495 => { __state = 563; }
                    496 => { __state = 497; }
                    497 => { __state = 498; }
                    498 => {
                        if b_arg_list != 0 {
                            __state = 500;
                        } else { __state = 501; }
                    }
                    499 => {
                        i_start = unsafe { sqlite3_str_length(p_accum_1) } as i64;
                        __state = 502;
                    }
                    500 => {
                        escarg = get_text_arg(unsafe { &mut *p_arg_list });
                        __state = 499;
                    }
                    501 => {
                        escarg =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i8>() + 7) & !7);
                                *(__va_p as *const *mut i8)
                            };
                        __state = 499;
                    }
                    502 => {
                        if escarg == core::ptr::null_mut() {
                            __state = 504;
                        } else { __state = 505; }
                    }
                    503 => {
                        if width > 0 as i64 &&
                                unsafe { sqlite3_str_errcode(p_accum_1) } == 0 {
                            __state = 544;
                        } else { __state = 543; }
                    }
                    504 => {
                        if xtype as i32 == 18 {
                            __state = 506;
                        } else { __state = 503; }
                    }
                    505 => {
                        if xtype as i32 == 18 {
                            __state = 508;
                        } else { __state = 507; }
                    }
                    506 => {
                        unsafe {
                            sqlite3_str_append(p_accum_1,
                                c"null".as_ptr() as *mut i8 as *const i8, 4)
                        };
                        __state = 503;
                    }
                    507 => { px__1 = precision; __state = 509; }
                    508 => {
                        unsafe {
                            sqlite3_str_append(p_accum_1,
                                c"\"".as_ptr() as *mut i8 as *const i8, 1)
                        };
                        __state = 507;
                    }
                    509 => { __state = 510; }
                    510 => {
                        if px__1 < 0 as i64 {
                            __state = 512;
                        } else { __state = 513; }
                    }
                    511 => { i = { j__1 = 0 as i64; j__1 }; __state = 523; }
                    512 => { px__1 = 2147483647 as i64; __state = 511; }
                    513 => {
                        if flag_altform2 != 0 {
                            __state = 514;
                        } else { __state = 511; }
                    }
                    514 => { i = 0 as i64; __state = 516; }
                    515 => {
                        if i == px__1 { __state = 520; } else { __state = 511; }
                    }
                    516 => {
                        if i < px__1 && unsafe { *escarg.offset(i as isize) } != 0 {
                            __state = 517;
                        } else { __state = 515; }
                    }
                    517 => {
                        if unsafe { *escarg.offset(i as isize) } as i32 & 192 == 128
                            {
                            __state = 519;
                        } else { __state = 518; }
                    }
                    518 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 516;
                    }
                    519 => {
                        { let __p = &mut px__1; let __t = *__p; *__p += 1; __t };
                        __state = 518;
                    }
                    520 => {
                        if unsafe { *escarg.offset(px__1 as isize) } as i32 & 192 ==
                                128 {
                            __state = 521;
                        } else { __state = 511; }
                    }
                    521 => {
                        { let __p = &mut px__1; let __t = *__p; *__p += 1; __t };
                        __state = 520;
                    }
                    522 => {
                        if j__1 < i { __state = 541; } else { __state = 540; }
                    }
                    523 => {
                        if i < px__1 { __state = 524; } else { __state = 522; }
                    }
                    524 => {
                        if {
                                                ch__1 = unsafe { *(escarg as *mut u8).offset(i as isize) };
                                                ch__1
                                            } as i32 <= 31 || ch__1 as i32 == '\"' as i32 ||
                                ch__1 as i32 == '\\' as i32 {
                            __state = 526;
                        } else { __state = 525; }
                    }
                    525 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 523;
                    }
                    526 => {
                        if j__1 < i { __state = 528; } else { __state = 527; }
                    }
                    527 => { j__1 = i + 1 as i64; __state = 529; }
                    528 => {
                        unsafe {
                            sqlite3_str_append64(p_accum_1,
                                unsafe { &raw mut *escarg.offset(j__1 as isize) } as
                                    *const i8, i - j__1)
                        };
                        __state = 527;
                    }
                    529 => {
                        if ch__1 as i32 == 0 {
                            __state = 531;
                        } else { __state = 530; }
                    }
                    530 => {
                        unsafe {
                            sqlite3_str_appendchar(p_accum_1, 1, '\\' as i32 as i8)
                        };
                        __state = 532;
                    }
                    531 => { __state = 522; }
                    532 => {
                        if ch__1 as i32 > 31 {
                            __state = 533;
                        } else { __state = 534; }
                    }
                    533 => {
                        unsafe {
                            sqlite3_str_appendchar(p_accum_1, 1, ch__1 as i8)
                        };
                        __state = 525;
                    }
                    534 => {
                        if 1 << ch__1 & 14080 as u32 != 0 as u32 {
                            __state = 535;
                        } else { __state = 536; }
                    }
                    535 => {
                        ch__1 =
                            unsafe {
                                    *(c"btn?fr".as_ptr() as
                                                *mut i8).offset((ch__1 as i32 - 8) as isize)
                                } as u8;
                        __state = 537;
                    }
                    536 => {
                        unsafe {
                            sqlite3_str_append(p_accum_1,
                                c"u00".as_ptr() as *mut i8 as *const i8, 3)
                        };
                        __state = 538;
                    }
                    537 => {
                        unsafe {
                            sqlite3_str_appendchar(p_accum_1, 1, ch__1 as i8)
                        };
                        __state = 525;
                    }
                    538 => {
                        unsafe {
                            sqlite3_str_appendchar(p_accum_1, 1,
                                a_hex[(ch__1 as i32 >> 4) as usize])
                        };
                        __state = 539;
                    }
                    539 => {
                        unsafe {
                            sqlite3_str_appendchar(p_accum_1, 1,
                                a_hex[(ch__1 as i32 & 15) as usize])
                        };
                        __state = 525;
                    }
                    540 => {
                        if xtype as i32 == 18 {
                            __state = 542;
                        } else { __state = 503; }
                    }
                    541 => {
                        unsafe {
                            sqlite3_str_append64(p_accum_1,
                                unsafe { &raw mut *escarg.offset(j__1 as isize) } as
                                    *const i8, i - j__1)
                        };
                        __state = 540;
                    }
                    542 => {
                        unsafe {
                            sqlite3_str_append(p_accum_1,
                                c"\"".as_ptr() as *mut i8 as *const i8, 1)
                        };
                        __state = 503;
                    }
                    543 => { __state = 39; }
                    544 => {
                        n__1 =
                            unsafe { sqlite3_str_length(p_accum_1) } as i64 - i_start;
                        __state = 545;
                    }
                    545 => { len = n__1; __state = 546; }
                    546 => { __state = 547; }
                    547 => {
                        if flag_altform2 != 0 && n__1 > 0 as i64 {
                            __state = 549;
                        } else { __state = 548; }
                    }
                    548 => {
                        if width > len { __state = 555; } else { __state = 543; }
                    }
                    549 => {
                        zz = unsafe { sqlite3_str_value(p_accum_1) };
                        __state = 550;
                    }
                    550 => { i = i_start; __state = 551; }
                    551 => {
                        if unsafe { *zz.offset(i as isize) } != 0 {
                            __state = 552;
                        } else { __state = 548; }
                    }
                    552 => {
                        if unsafe { *zz.offset(i as isize) } as i32 & 192 == 128 {
                            __state = 554;
                        } else { __state = 553; }
                    }
                    553 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 551;
                    }
                    554 => {
                        { let __p = &mut len; let __t = *__p; *__p -= 1; __t };
                        __state = 553;
                    }
                    555 => { sp = width - len; __state = 556; }
                    556 => { { let _ = 0; }; __state = 557; }
                    557 => {
                        unsafe {
                            sqlite3_str_appendchar64(p_accum_1, sp as i32 as i64,
                                ' ' as i32 as i8)
                        };
                        __state = 558;
                    }
                    558 => {
                        if (flag_leftjustify == 0) as i32 != 0 && n__1 > 0 as i64 &&
                                unsafe { sqlite3_str_errcode(p_accum_1) } == 0 {
                            __state = 559;
                        } else { __state = 543; }
                    }
                    559 => {
                        zz = unsafe { sqlite3_str_value(p_accum_1) };
                        __state = 560;
                    }
                    560 => {
                        {
                            let __n = i_start;
                            let __p = &mut zz;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 561;
                    }
                    561 => {
                        unsafe {
                            memmove(unsafe { zz.offset(sp as isize) } as *mut (),
                                zz as *const (), n__1 as u64)
                        };
                        __state = 562;
                    }
                    562 => {
                        unsafe { memset(zz as *mut (), ' ' as i32, sp as u64) };
                        __state = 543;
                    }
                    563 => { __state = 163; }
                    564 => { __state = 647; }
                    565 => { need_quote = 0; __state = 566; }
                    566 => { __state = 567; }
                    567 => { __state = 568; }
                    568 => { __state = 569; }
                    569 => {
                        if b_arg_list != 0 {
                            __state = 571;
                        } else { __state = 572; }
                    }
                    570 => {
                        if escarg__1 == core::ptr::null_mut() {
                            __state = 574;
                        } else { __state = 575; }
                    }
                    571 => {
                        escarg__1 = get_text_arg(unsafe { &mut *p_arg_list });
                        __state = 570;
                    }
                    572 => {
                        escarg__1 =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut i8>() + 7) & !7);
                                *(__va_p as *const *mut i8)
                            };
                        __state = 570;
                    }
                    573 => {
                        if xtype as i32 == 14 {
                            __state = 578;
                        } else { __state = 579; }
                    }
                    574 => {
                        escarg__1 =
                            if xtype as i32 == 10 {
                                c"NULL".as_ptr() as *mut i8
                            } else { c"(NULL)".as_ptr() as *mut i8 };
                        __state = 573;
                    }
                    575 => {
                        if xtype as i32 == 10 {
                            __state = 576;
                        } else { __state = 573; }
                    }
                    576 => { need_quote = 1; __state = 573; }
                    577 => { k = precision; __state = 581; }
                    578 => { q = '\"' as i32 as i8; __state = 580; }
                    579 => { q = '\'' as i32 as i8; __state = 577; }
                    580 => { flag_alternateform = 0 as etByte; __state = 577; }
                    581 => { i__1 = { n__2 = 0 as i64; n__2 }; __state = 583; }
                    582 => {
                        if flag_alternateform != 0 {
                            __state = 591;
                        } else { __state = 590; }
                    }
                    583 => {
                        if k != 0 as i64 &&
                                {
                                            ch__2 = unsafe { *escarg__1.offset(i__1 as isize) };
                                            ch__2
                                        } as i32 != 0 {
                            __state = 584;
                        } else { __state = 582; }
                    }
                    584 => {
                        if ch__2 as i32 == q as i32 {
                            __state = 587;
                        } else { __state = 586; }
                    }
                    585 => {
                        {
                            { let __p = &mut i__1; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut k; let __t = *__p; *__p -= 1; __t }
                        };
                        __state = 583;
                    }
                    586 => {
                        if flag_altform2 != 0 && ch__2 as i32 & 192 == 192 {
                            __state = 588;
                        } else { __state = 585; }
                    }
                    587 => {
                        { let __p = &mut n__2; let __t = *__p; *__p += 1; __t };
                        __state = 586;
                    }
                    588 => {
                        if unsafe { *escarg__1.offset((i__1 + 1 as i64) as isize) }
                                        as i32 & 192 == 128 {
                            __state = 589;
                        } else { __state = 585; }
                    }
                    589 => {
                        { let __p = &mut i__1; let __t = *__p; *__p += 1; __t };
                        __state = 588;
                    }
                    590 => { n__2 += i__1 + 3 as i64; __state = 606; }
                    591 => { n_back = 0 as i64; __state = 592; }
                    592 => { n_ctrl = 0 as i64; __state = 593; }
                    593 => { k = 0 as i64; __state = 595; }
                    594 => {
                        if n_ctrl != 0 || xtype as i32 == 9 {
                            __state = 601;
                        } else { __state = 602; }
                    }
                    595 => {
                        if k < i__1 { __state = 596; } else { __state = 594; }
                    }
                    596 => {
                        if unsafe { *escarg__1.offset(k as isize) } as i32 ==
                                '\\' as i32 {
                            __state = 598;
                        } else { __state = 599; }
                    }
                    597 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 595;
                    }
                    598 => {
                        { let __p = &mut n_back; let __t = *__p; *__p += 1; __t };
                        __state = 597;
                    }
                    599 => {
                        if unsafe { *(escarg__1 as *mut u8).offset(k as isize) } as
                                    i32 <= 31 {
                            __state = 600;
                        } else { __state = 597; }
                    }
                    600 => {
                        { let __p = &mut n_ctrl; let __t = *__p; *__p += 1; __t };
                        __state = 597;
                    }
                    601 => {
                        n__2 += n_back + 5 as i64 * n_ctrl;
                        __state = 603;
                    }
                    602 => { flag_alternateform = 0 as etByte; __state = 590; }
                    603 => {
                        if xtype as i32 == 10 {
                            __state = 604;
                        } else { __state = 590; }
                    }
                    604 => { n__2 += 10 as i64; __state = 605; }
                    605 => { need_quote = 2; __state = 590; }
                    606 => {
                        if n__2 > 70 as i64 {
                            __state = 608;
                        } else { __state = 609; }
                    }
                    607 => { j__2 = 0 as i64; __state = 612; }
                    608 => {
                        bufpt =
                            { z_extra = printf_temp_buf(p_accum_1, n__2); z_extra };
                        __state = 610;
                    }
                    609 => {
                        bufpt = &raw mut buf[0 as usize] as *mut i8;
                        __state = 607;
                    }
                    610 => {
                        if bufpt == core::ptr::null_mut() {
                            __state = 611;
                        } else { __state = 607; }
                    }
                    611 => { return; }
                    612 => {
                        if need_quote != 0 {
                            __state = 614;
                        } else { __state = 613; }
                    }
                    613 => { k = i__1; __state = 618; }
                    614 => {
                        if need_quote == 2 {
                            __state = 615;
                        } else { __state = 616; }
                    }
                    615 => {
                        unsafe {
                            memcpy(unsafe { &raw mut *bufpt.offset(j__2 as isize) } as
                                    *mut (), c"unistr(\'".as_ptr() as *mut i8 as *const (),
                                8 as u64)
                        };
                        __state = 617;
                    }
                    616 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = '\'' as i32 as i8
                        };
                        __state = 613;
                    }
                    617 => { j__2 += 8 as i64; __state = 613; }
                    618 => {
                        if flag_alternateform != 0 {
                            __state = 620;
                        } else { __state = 621; }
                    }
                    619 => {
                        if need_quote != 0 {
                            __state = 642;
                        } else { __state = 641; }
                    }
                    620 => { i__1 = 0 as i64; __state = 622; }
                    621 => { i__1 = 0 as i64; __state = 636; }
                    622 => {
                        if i__1 < k { __state = 623; } else { __state = 619; }
                    }
                    623 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) =
                                {
                                    ch__2 = unsafe { *escarg__1.offset(i__1 as isize) };
                                    ch__2
                                }
                        };
                        __state = 625;
                    }
                    624 => {
                        { let __p = &mut i__1; let __t = *__p; *__p += 1; __t };
                        __state = 622;
                    }
                    625 => {
                        if ch__2 as i32 == q as i32 {
                            __state = 626;
                        } else { __state = 627; }
                    }
                    626 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = ch__2
                        };
                        __state = 624;
                    }
                    627 => {
                        if ch__2 as i32 == '\\' as i32 {
                            __state = 628;
                        } else { __state = 629; }
                    }
                    628 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = '\\' as i32 as i8
                        };
                        __state = 624;
                    }
                    629 => {
                        if ch__2 as u8 as i32 <= 31 {
                            __state = 630;
                        } else { __state = 624; }
                    }
                    630 => {
                        unsafe {
                            *bufpt.offset((j__2 - 1 as i64) as isize) =
                                '\\' as i32 as i8
                        };
                        __state = 631;
                    }
                    631 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = 'u' as i32 as i8
                        };
                        __state = 632;
                    }
                    632 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = '0' as i32 as i8
                        };
                        __state = 633;
                    }
                    633 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = '0' as i32 as i8
                        };
                        __state = 634;
                    }
                    634 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) =
                                if ch__2 as i32 >= 16 { '1' as i32 } else { '0' as i32 } as
                                    i8
                        };
                        __state = 635;
                    }
                    635 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = a_hex[(ch__2 as i32 & 15) as usize] as i8
                        };
                        __state = 624;
                    }
                    636 => {
                        if i__1 < k { __state = 637; } else { __state = 619; }
                    }
                    637 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) =
                                {
                                    ch__2 = unsafe { *escarg__1.offset(i__1 as isize) };
                                    ch__2
                                }
                        };
                        __state = 639;
                    }
                    638 => {
                        { let __p = &mut i__1; let __t = *__p; *__p += 1; __t };
                        __state = 636;
                    }
                    639 => {
                        if ch__2 as i32 == q as i32 {
                            __state = 640;
                        } else { __state = 638; }
                    }
                    640 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = ch__2
                        };
                        __state = 638;
                    }
                    641 => {
                        unsafe { *bufpt.offset(j__2 as isize) = 0 as i8 };
                        __state = 645;
                    }
                    642 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = '\'' as i32 as i8
                        };
                        __state = 643;
                    }
                    643 => {
                        if need_quote == 2 {
                            __state = 644;
                        } else { __state = 641; }
                    }
                    644 => {
                        unsafe {
                            *bufpt.offset({
                                                let __p = &mut j__2;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = ')' as i32 as i8
                        };
                        __state = 641;
                    }
                    645 => { length = j__2; __state = 646; }
                    646 => { __state = 2; }
                    647 => { __state = 166; }
                    648 => { __state = 167; }
                    649 => {
                        if flag_alternateform != 0 {
                            __state = 652;
                        } else { __state = 653; }
                    }
                    650 => { return; }
                    651 => {
                        length = { width = 0 as i64; width };
                        __state = 661;
                    }
                    652 => {
                        p_expr =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut Expr>() + 7) & !7);
                                *(__va_p as *const *mut Expr)
                            };
                        __state = 654;
                    }
                    653 => {
                        p_token =
                            unsafe {
                                    let __ap = &mut ap;
                                    let __va_p = *__ap;
                                    *__ap =
                                        (*__ap).add((core::mem::size_of::<*mut Token>() + 7) & !7);
                                    *(__va_p as *const *mut Token)
                                } as *const Token;
                        __state = 657;
                    }
                    654 => {
                        if !(p_expr).is_null() &&
                                !(unsafe { (*p_expr).flags } & 2048 as u32 != 0 as u32) as
                                        i32 != 0 {
                            __state = 655;
                        } else { __state = 651; }
                    }
                    655 => {
                        unsafe {
                            sqlite3_str_appendall(p_accum_1,
                                unsafe { (*p_expr).u.z_token } as *const i8)
                        };
                        __state = 656;
                    }
                    656 => {
                        unsafe {
                            sqlite3_record_error_offset_of_expr(unsafe {
                                    &mut *unsafe { (*p_accum_1).db }
                                }, p_expr as *const Expr)
                        };
                        __state = 651;
                    }
                    657 => { { let _ = 0; }; __state = 658; }
                    658 => {
                        if !(p_token).is_null() && unsafe { (*p_token).n } != 0 {
                            __state = 659;
                        } else { __state = 651; }
                    }
                    659 => {
                        unsafe {
                            sqlite3_str_append(p_accum_1,
                                unsafe { (*p_token).z } as *const i8,
                                unsafe { (*p_token).n } as i32)
                        };
                        __state = 660;
                    }
                    660 => {
                        unsafe {
                            sqlite3_record_error_byte_offset(unsafe { (*p_accum_1).db },
                                unsafe { (*p_token).z })
                        };
                        __state = 651;
                    }
                    661 => { __state = 148; }
                    662 => { __state = 168; }
                    663 => {
                        if unsafe { (*p_accum_1).printf_flags } as i32 & 1 == 0 {
                            __state = 665;
                        } else { __state = 664; }
                    }
                    664 => {
                        p_item =
                            unsafe {
                                let __ap = &mut ap;
                                let __va_p = *__ap;
                                *__ap =
                                    (*__ap).add((core::mem::size_of::<*mut SrcItem>() + 7) &
                                            !7);
                                *(__va_p as *const *mut SrcItem)
                            };
                        __state = 666;
                    }
                    665 => { return; }
                    666 => { { let _ = 0; }; __state = 667; }
                    667 => {
                        if !(unsafe { (*p_item).z_alias }).is_null() &&
                                (flag_altform2 == 0) as i32 != 0 {
                            __state = 669;
                        } else { __state = 670; }
                    }
                    668 => {
                        length = { width = 0 as i64; width };
                        __state = 686;
                    }
                    669 => {
                        unsafe {
                            sqlite3_str_appendall(p_accum_1,
                                unsafe { (*p_item).z_alias } as *const i8)
                        };
                        __state = 668;
                    }
                    670 => {
                        if !(unsafe { (*p_item).z_name }).is_null() {
                            __state = 671;
                        } else { __state = 672; }
                    }
                    671 => {
                        if unsafe { (*p_item).fg.fixed_schema() } as i32 == 0 &&
                                    unsafe { (*p_item).fg.is_subquery() } as i32 == 0 &&
                                unsafe { (*p_item).u4.z_database } != core::ptr::null_mut()
                            {
                            __state = 674;
                        } else { __state = 673; }
                    }
                    672 => {
                        if !(unsafe { (*p_item).z_alias }).is_null() {
                            __state = 676;
                        } else { __state = 677; }
                    }
                    673 => {
                        unsafe {
                            sqlite3_str_appendall(p_accum_1,
                                unsafe { (*p_item).z_name } as *const i8)
                        };
                        __state = 668;
                    }
                    674 => {
                        unsafe {
                            sqlite3_str_appendall(p_accum_1,
                                unsafe { (*p_item).u4.z_database } as *const i8)
                        };
                        __state = 675;
                    }
                    675 => {
                        unsafe {
                            sqlite3_str_append(p_accum_1,
                                c".".as_ptr() as *mut i8 as *const i8, 1)
                        };
                        __state = 673;
                    }
                    676 => {
                        unsafe {
                            sqlite3_str_appendall(p_accum_1,
                                unsafe { (*p_item).z_alias } as *const i8)
                        };
                        __state = 668;
                    }
                    677 => {
                        if unsafe { (*p_item).fg.is_subquery() } != 0 {
                            __state = 678;
                        } else { __state = 668; }
                    }
                    678 => {
                        p_sel =
                            unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select } as
                                *const Select;
                        __state = 679;
                    }
                    679 => { { let _ = 0; }; __state = 680; }
                    680 => {
                        if unsafe { (*p_sel).sel_flags } & 2048 as u32 != 0 {
                            __state = 681;
                        } else { __state = 682; }
                    }
                    681 => {
                        unsafe {
                            sqlite3_str_appendf(p_accum_1,
                                c"(join-%u)".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_sel).sel_id })
                        };
                        __state = 668;
                    }
                    682 => {
                        if unsafe { (*p_sel).sel_flags } & 1024 as u32 != 0 {
                            __state = 683;
                        } else { __state = 684; }
                    }
                    683 => { { let _ = 0; }; __state = 685; }
                    684 => {
                        unsafe {
                            sqlite3_str_appendf(p_accum_1,
                                c"(subquery-%u)".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_sel).sel_id })
                        };
                        __state = 668;
                    }
                    685 => {
                        unsafe {
                            sqlite3_str_appendf(p_accum_1,
                                c"%u-ROW VALUES CLAUSE".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_item).u1.n_row })
                        };
                        __state = 668;
                    }
                    686 => { __state = 148; }
                    687 => { return; }
                    688 => {
                        if width > 0 as i64 {
                            __state = 690;
                        } else { __state = 691; }
                    }
                    689 => {
                        if !(z_extra).is_null() {
                            __state = 696;
                        } else { __state = 39; }
                    }
                    690 => {
                        if (flag_leftjustify == 0) as i32 != 0 {
                            __state = 693;
                        } else { __state = 692; }
                    }
                    691 => {
                        unsafe {
                            sqlite3_str_append64(p_accum_1, bufpt as *const i8, length)
                        };
                        __state = 689;
                    }
                    692 => {
                        unsafe {
                            sqlite3_str_append64(p_accum_1, bufpt as *const i8, length)
                        };
                        __state = 694;
                    }
                    693 => {
                        unsafe {
                            sqlite3_str_appendchar64(p_accum_1, width, ' ' as i32 as i8)
                        };
                        __state = 692;
                    }
                    694 => {
                        if flag_leftjustify != 0 {
                            __state = 695;
                        } else { __state = 689; }
                    }
                    695 => {
                        unsafe {
                            sqlite3_str_appendchar64(p_accum_1, width, ' ' as i32 as i8)
                        };
                        __state = 689;
                    }
                    696 => {
                        unsafe {
                            sqlite3_db_free(unsafe { (*p_accum_1).db },
                                z_extra as *mut ())
                        };
                        __state = 697;
                    }
                    697 => { z_extra = core::ptr::null_mut(); __state = 39; }
                    _ => {}
                }
            }
        }
    }
}
extern "C" fn str_accum_finish_realloc(p: *mut StrAccum) -> *mut i8 {
    let mut z_text: *mut i8 = core::ptr::null_mut();
    { let _ = 0; };
    z_text =
        unsafe {
                sqlite3_db_malloc_raw(unsafe { (*p).db },
                    1 as u64 + unsafe { (*p).n_char } as u64)
            } as *mut i8;
    if !(z_text).is_null() {
        unsafe {
            memcpy(z_text as *mut (), unsafe { (*p).z_text } as *const (),
                (unsafe { (*p).n_char } + 1 as u32) as u64)
        };
        unsafe { (*p).printf_flags |= 4 as u8 };
    } else { sqlite3_str_accum_set_error(p, 7 as u8); }
    unsafe { (*p).z_text = z_text };
    return z_text;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_accum_finish(p: *mut StrAccum) -> *mut i8 {
    if !(unsafe { (*p).z_text }).is_null() {
        unsafe {
            *unsafe { (*p).z_text.add(unsafe { (*p).n_char } as usize) } =
                0 as i8
        };
        if unsafe { (*p).mx_alloc } > 0 as u32 &&
                !(unsafe { (*p).printf_flags } as i32 & 4 != 0) as i32 != 0 {
            return str_accum_finish_realloc(p);
        }
    }
    return unsafe { (*p).z_text };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vmprintf(z_format: *const i8, ap: *mut i8)
    -> *mut i8 {
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut z_base: [i8; 70] = [0; 70];
    let mut acc: StrAccum = unsafe { core::mem::zeroed() };
    if unsafe { sqlite3_initialize() } != 0 { return core::ptr::null_mut(); }
    sqlite3_str_accum_init(&mut acc, core::ptr::null_mut(),
        &raw mut z_base[0 as usize] as *mut i8,
        core::mem::size_of::<[i8; 70]>() as i32, 1000000000);
    sqlite3_str_vappendf(&raw mut acc as *mut sqlite3_str, z_format,
        ap as *const i8);
    z = sqlite3_str_accum_finish(&mut acc);
    return z;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_mprintf(z_format: *const i8, mut __va0: ...)
    -> *mut i8 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z: *mut i8 = core::ptr::null_mut();
    if unsafe { sqlite3_initialize() } != 0 { return core::ptr::null_mut(); }
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z = sqlite3_vmprintf(z_format, ap);
    ();
    return z;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_snprintf(n: i32, z_buf: *mut i8,
    z_format: *const i8, mut __va0: ...) -> *mut i8 {
    let mut acc: StrAccum = unsafe { core::mem::zeroed() };
    let mut ap: *mut i8 = core::ptr::null_mut();
    if n <= 0 { return z_buf; }
    sqlite3_str_accum_init(&mut acc, core::ptr::null_mut(), z_buf, n, 0);
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    sqlite3_str_vappendf(&raw mut acc as *mut sqlite3_str, z_format,
        ap as *const i8);
    ();
    unsafe { *z_buf.add(acc.n_char as usize) = 0 as i8 };
    return z_buf;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vsnprintf(n: i32, z_buf: *mut i8,
    z_format: *const i8, ap: *mut i8) -> *mut i8 {
    let mut acc: StrAccum = unsafe { core::mem::zeroed() };
    if n <= 0 { return z_buf; }
    sqlite3_str_accum_init(&mut acc, core::ptr::null_mut(), z_buf, n, 0);
    sqlite3_str_vappendf(&raw mut acc as *mut sqlite3_str, z_format,
        ap as *const i8);
    unsafe { *z_buf.add(acc.n_char as usize) = 0 as i8 };
    return z_buf;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_stmt {
    _opaque: [u8; 0],
}
type sqlite3_destructor_type = unsafe extern "C" fn(*mut ()) -> ();
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_blob {
    _opaque: [u8; 0],
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
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_new(db: *const sqlite3) -> *mut sqlite3_str {
    unsafe {
        let mut p: *mut sqlite3_str =
            unsafe {
                    sqlite3_malloc64(core::mem::size_of::<sqlite3_str>() as
                            sqlite3_uint64)
                } as *mut sqlite3_str;
        if !(p).is_null() {
            sqlite3_str_accum_init(unsafe { &mut *p }, core::ptr::null_mut(),
                core::ptr::null_mut(), 0,
                if !(db).is_null() {
                    unsafe { (*db).a_limit[0 as usize] }
                } else { 1000000000 });
        } else { p = &raw const sqlite3_oom_str as *mut sqlite3_str; }
        return p;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_finish(p: *mut sqlite3_str) -> *mut i8 {
    unsafe {
        let mut z: *mut i8 = core::ptr::null_mut();
        if p != core::ptr::null_mut() &&
                p != &raw const sqlite3_oom_str as *mut sqlite3_str {
            z = sqlite3_str_accum_finish(p as *mut StrAccum);
            unsafe { sqlite3_free(p as *mut ()) };
        } else { z = core::ptr::null_mut(); }
        return z;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_free(p: *mut sqlite3_str) -> () {
    unsafe {
        if p != core::ptr::null_mut() &&
                p != &raw const sqlite3_oom_str as *mut sqlite3_str {
            sqlite3_str_reset(p as *mut StrAccum);
            unsafe { sqlite3_free(p as *mut ()) };
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_truncate(p: *mut sqlite3_str, n_1: i32) -> () {
    if p != core::ptr::null_mut() && n_1 >= 0 &&
            (n_1 as u32) < unsafe { (*p).n_char } {
        unsafe { (*p).n_char = n_1 as u32 };
        unsafe {
            *unsafe { (*p).z_text.add(unsafe { (*p).n_char } as usize) } =
                0 as i8
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_pcache_page {
    p_buf: *mut (),
    p_extra: *mut (),
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
extern "C" fn render_log_msg(i_err_code_1: i32, z_format_1: *const i8,
    ap: *mut i8) -> () {
    unsafe {
        let mut acc: StrAccum = unsafe { core::mem::zeroed() };
        let mut z_msg: [i8; 700] = [0; 700];
        sqlite3_str_accum_init(&mut acc, core::ptr::null_mut(),
            &raw mut z_msg[0 as usize] as *mut i8,
            core::mem::size_of::<[i8; 700]>() as i32, 0);
        sqlite3_str_vappendf(&raw mut acc as *mut sqlite3_str, z_format_1,
            ap as *const i8);
        unsafe {
            sqlite3Config.x_log.unwrap()(sqlite3Config.p_log_arg,
                i_err_code_1, sqlite3_str_accum_finish(&mut acc) as *const i8)
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_log(i_err_code: i32, z_format: *const i8,
    mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        if sqlite3Config.x_log.is_some() {
            unsafe { ap = core::mem::transmute_copy(&__va0) };
            render_log_msg(i_err_code, z_format, ap);
            ();
        }
    }
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
#[repr(C)]
#[derive(Copy, Clone)]
struct AuthContext {
    z_auth_context: *const i8,
    p_parse: *mut Parse,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Bitvec {
    _opaque: [u8; 0],
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
struct Pager {
    _opaque: [u8; 0],
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
struct PCache {
    _opaque: [u8; 0],
}
type DbPage = PgHdr;
#[repr(C)]
#[derive(Copy, Clone)]
struct BtCursor {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct BtShared {
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
#[repr(C)]
#[derive(Copy, Clone)]
struct VdbeOpList {
    opcode: u8,
    p1: i8,
    p2: i8,
    p3: i8,
}
type RecordCompare =
    unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord) -> i32;
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
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vm_printf(db: *mut sqlite3, z_format_1: *const i8,
    ap: *mut i8) -> *mut i8 {
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut z_base: [i8; 70] = [0; 70];
    let mut acc: StrAccum = unsafe { core::mem::zeroed() };
    { let _ = 0; };
    sqlite3_str_accum_init(&mut acc, db,
        &raw mut z_base[0 as usize] as *mut i8,
        core::mem::size_of::<[i8; 70]>() as i32,
        unsafe { (*db).a_limit[0 as usize] });
    acc.printf_flags = 1 as u8;
    sqlite3_str_vappendf(&raw mut acc as *mut sqlite3_str, z_format_1,
        ap as *const i8);
    z = sqlite3_str_accum_finish(&mut acc);
    if acc.acc_error as i32 == 7 { unsafe { sqlite3_oom_fault(db) }; }
    return z;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_m_printf(db: *mut sqlite3,
    z_format_1: *const i8, mut __va0: ...) -> *mut i8 {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z = sqlite3_vm_printf(db, z_format_1, ap);
    ();
    return z;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rc_str_ref(z: *mut i8) -> *mut i8 {
    let mut p: *mut RCStr = z as *mut RCStr;
    { let _ = 0; };
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(-1) };
        __t
    };
    {
        let __p = unsafe { &mut (*p).n_rc_ref };
        let __t = *__p;
        *__p += 1;
        __t
    };
    return z;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rc_str_unref(z: *mut ()) -> () {
    let mut p: *mut RCStr = z as *mut RCStr;
    { let _ = 0; };
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(-1) };
        __t
    };
    { let _ = 0; };
    if unsafe { (*p).n_rc_ref } >= 2 as u64 {
        {
            let __p = unsafe { &mut (*p).n_rc_ref };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    } else { unsafe { sqlite3_free(p as *mut ()) }; }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rc_str_new(n: u64) -> *mut i8 {
    let p: *mut RCStr =
        unsafe {
                sqlite3_malloc64(n + core::mem::size_of::<RCStr>() as u64 +
                        1 as u64)
            } as *mut RCStr;
    if p == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe { (*p).n_rc_ref = 1 as u64 };
    return unsafe { &raw mut *p.offset(1 as isize) } as *mut i8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rc_str_resize(z: *mut i8, n: u64) -> *mut i8 {
    let mut p: *mut RCStr = z as *mut RCStr;
    let mut p_new: *mut RCStr = core::ptr::null_mut();
    { let _ = 0; };
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(-1) };
        __t
    };
    { let _ = 0; };
    p_new =
        unsafe {
                sqlite3_realloc64(p as *mut (),
                    n + core::mem::size_of::<RCStr>() as u64 + 1 as u64)
            } as *mut RCStr;
    if p_new == core::ptr::null_mut() {
        unsafe { sqlite3_free(p as *mut ()) };
        return core::ptr::null_mut();
    } else {
        return unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8;
    }
}
static z_ord: [i8; 9] =
    [116 as i8, 104 as i8, 115 as i8, 116 as i8, 110 as i8, 100 as i8,
            114 as i8, 100 as i8, 0 as i8];
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
    fn sqlite3_db_free(_: *mut sqlite3, _: *mut ())
    -> ();
    static sqlite3_oom_str: sqlite3_str;
    fn sqlite3_error_to_parser(_: *mut sqlite3, _: i32)
    -> i32;
    fn sqlite3_db_realloc(_: *mut sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3Realloc(_: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_size(_: *mut sqlite3, _: *const ())
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_value_int64(_: *mut sqlite3_value)
    -> sqlite3_int64;
    fn sqlite3_malloc(_: i32)
    -> *mut ();
    fn sqlite3_value_double(_: *mut sqlite3_value)
    -> f64;
    fn sqlite3_fp_decode(_: *mut FpDecode, _: f64, _: i32, _: i32)
    -> ();
    fn sqlite3_value_text(_: *mut sqlite3_value)
    -> *const u8;
    fn sqlite3_append_one_utf8_character(_: *mut i8, _: u32)
    -> i32;
    fn sqlite3_strlen30(_: *const i8)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw(_: *mut sqlite3, _: u64)
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
    fn sqlite3_value_int(_: *mut sqlite3_value)
    -> i32;
    fn sqlite3_value_pointer(_: *mut sqlite3_value, _: *const i8)
    -> *mut ();
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
    fn sqlite3_enable_shared_cache(_: i32)
    -> i32;
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
    fn sqlite3_result_str(_: *mut sqlite3_context, _: *mut sqlite3_str,
    _: i32)
    -> ();
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
    static mut sqlite3Config: Sqlite3Config;
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
    fn sqlite3_btree_open(p_vfs_1: *mut sqlite3_vfs, z_filename_1: *const i8,
    db: *mut sqlite3, pp_btree_1: *mut *mut Btree, flags: i32,
    vfs_flags_1: i32)
    -> i32;
    fn sqlite3_btree_close(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_set_cache_size(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_set_spill_size(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_set_mmap_limit(_: *mut Btree, _: sqlite3_int64)
    -> i32;
    fn sqlite3_btree_set_pager_flags(_: *mut Btree, _: u32)
    -> i32;
    fn sqlite3_btree_set_page_size(p: *mut Btree, n_pagesize_1: i32,
    n_reserve_1: i32, e_fix_1: i32)
    -> i32;
    fn sqlite3_btree_get_page_size(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_max_page_count(_: *mut Btree, _: Pgno)
    -> Pgno;
    fn sqlite3_btree_last_page(_: *mut Btree)
    -> Pgno;
    fn sqlite3_btree_secure_delete(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_get_requested_reserve(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_get_reserve_no_mutex(p: *mut Btree)
    -> i32;
    fn sqlite3_btree_set_auto_vacuum(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_get_auto_vacuum(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_begin_trans(_: *mut Btree, _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_btree_commit_phase_one(_: *mut Btree, _: *const i8)
    -> i32;
    fn sqlite3_btree_commit_phase_two(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_commit(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_rollback(_: *mut Btree, _: i32, _: i32)
    -> i32;
    fn sqlite3_btree_begin_stmt(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_create_table(_: *mut Btree, _: *mut Pgno, flags: i32)
    -> i32;
    fn sqlite3_btree_txn_state(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_is_in_backup(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_schema(_: *mut Btree, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut ();
    fn sqlite3_btree_schema_locked(p_btree_1: *mut Btree)
    -> i32;
    fn sqlite3_btree_lock_table(p_btree_1: *mut Btree, i_tab_1: i32,
    is_write_lock_1: u8)
    -> i32;
    fn sqlite3_btree_savepoint(_: *mut Btree, _: i32, _: i32)
    -> i32;
    fn sqlite3_btree_checkpoint(_: *mut Btree, _: i32, _: *mut i32,
    _: *mut i32)
    -> i32;
    fn sqlite3_btree_get_filename(_: *mut Btree)
    -> *const i8;
    fn sqlite3_btree_get_journalname(_: *mut Btree)
    -> *const i8;
    fn sqlite3_btree_copy_file(_: *mut Btree, _: *mut Btree)
    -> i32;
    fn sqlite3_btree_incr_vacuum(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_drop_table(_: *mut Btree, _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_btree_clear_table(_: *mut Btree, _: i32, _: *mut i64)
    -> i32;
    fn sqlite3_btree_clear_table_of_cursor(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_trip_all_cursors(_: *mut Btree, _: i32, _: i32)
    -> i32;
    fn sqlite3_btree_get_meta(p_btree_1: *mut Btree, idx: i32,
    p_value_1: *mut u32)
    -> ();
    fn sqlite3_btree_update_meta(_: *mut Btree, idx: i32, value: u32)
    -> i32;
    fn sqlite3_btree_new_db(p: *mut Btree)
    -> i32;
    fn sqlite3_btree_cursor(_: *mut Btree, i_table_1: Pgno, wr_flag_1: i32,
    _: *mut KeyInfo, p_cursor_1: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_fake_valid_cursor()
    -> *mut BtCursor;
    fn sqlite3_btree_cursor_size()
    -> i32;
    fn sqlite3_btree_cursor_zero(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_cursor_hint_flags(_: *mut BtCursor, _: u32)
    -> ();
    fn sqlite3_btree_close_cursor(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_table_moveto(_: *mut BtCursor, int_key_1: i64, bias: i32,
    p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_index_moveto(_: *mut BtCursor,
    p_un_key_1: *mut UnpackedRecord, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_cursor_has_moved(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_cursor_restore(_: *mut BtCursor, _: *mut i32)
    -> i32;
    fn sqlite3_btree_delete(_: *mut BtCursor, flags: u8)
    -> i32;
    fn sqlite3_btree_insert(_: *mut BtCursor,
    p_payload_1: *const BtreePayload, flags: i32, seek_result_1: i32)
    -> i32;
    fn sqlite3_btree_first(_: *mut BtCursor, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_is_empty(p_cur_1: *mut BtCursor, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_last(_: *mut BtCursor, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_next(_: *mut BtCursor, flags: i32)
    -> i32;
    fn sqlite3_btree_eof(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_previous(_: *mut BtCursor, flags: i32)
    -> i32;
    fn sqlite3_btree_integer_key(_: *mut BtCursor)
    -> i64;
    fn sqlite3_btree_cursor_pin(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_cursor_unpin(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_offset(_: *mut BtCursor)
    -> i64;
    fn sqlite3_btree_payload(_: *mut BtCursor, offset: u32, amt: u32,
    _: *mut ())
    -> i32;
    fn sqlite3_btree_payload_fetch(_: *mut BtCursor, p_amt_1: *mut u32)
    -> *const ();
    fn sqlite3_btree_payload_size(_: *mut BtCursor)
    -> u32;
    fn sqlite3_btree_max_record_size(_: *mut BtCursor)
    -> sqlite3_int64;
    fn sqlite3_btree_integrity_check(db: *mut sqlite3, p: *mut Btree,
    a_root_1: *mut Pgno, a_cnt_1: *mut sqlite3_value, n_root_1: i32,
    mx_err_1: i32, pn_err_1: *mut i32, pz_out_1: *mut *mut i8)
    -> i32;
    fn sqlite3_btree_pager(_: *mut Btree)
    -> *mut Pager;
    fn sqlite3_btree_row_count_est(_: *mut BtCursor)
    -> i64;
    fn sqlite3_btree_payload_checked(_: *mut BtCursor, offset: u32, amt: u32,
    _: *mut ())
    -> i32;
    fn sqlite3_btree_put_data(_: *mut BtCursor, offset: u32, amt: u32,
    _: *mut ())
    -> i32;
    fn sqlite3_btree_incrblob_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_clear_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_set_version(p_bt_1: *mut Btree, i_version_1: i32)
    -> i32;
    fn sqlite3_btree_cursor_has_hint(_: *mut BtCursor, mask: u32)
    -> i32;
    fn sqlite3_btree_is_readonly(p_bt_1: *mut Btree)
    -> i32;
    fn sqlite3_header_size_btree()
    -> i32;
    fn sqlite3_btree_cursor_is_valid_nn(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_count(_: *mut sqlite3, _: *mut BtCursor, _: *mut i64)
    -> i32;
    fn sqlite3_btree_transfer_row(_: *mut BtCursor, _: *mut BtCursor, _: i64)
    -> i32;
    fn sqlite3_btree_clear_cache(_: *mut Btree)
    -> ();
    fn sqlite3_btree_enter(_: *mut Btree)
    -> ();
    fn sqlite3_btree_enter_all(_: *mut sqlite3)
    -> ();
    fn sqlite3_btree_sharable(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_enter_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_connection_count(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_leave(_: *mut Btree)
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
    fn sqlite3_vdbe_record_unpack(_: i32, _: *const (),
    _: *mut UnpackedRecord)
    -> ();
    fn sqlite3_vdbe_record_compare(_: i32, _: *const (),
    _: *mut UnpackedRecord)
    -> i32;
    fn sqlite3_vdbe_record_compare_with_skip(_: i32, _: *const (),
    _: *mut UnpackedRecord, _: i32)
    -> i32;
    fn sqlite3_vdbe_alloc_unpacked_record(_: *mut KeyInfo)
    -> *mut UnpackedRecord;
    fn sqlite3_vdbe_find_compare(_: *mut UnpackedRecord)
    -> unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord) -> i32;
    fn sqlite3_vdbe_link_sub_program(_: *mut Vdbe, _: *mut SubProgram)
    -> ();
    fn sqlite3_vdbe_has_sub_program(_: *mut Vdbe)
    -> i32;
    fn sqlite3_mem_set_array_int64(a_mem_1: *mut sqlite3_value, i_idx_1: i32,
    val: i64)
    -> ();
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
    fn sqlite3_corrupt_error(_: i32)
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
    fn sqlite3Malloc(_: u64)
    -> *mut ();
    fn sqlite3_malloc_zero(_: u64)
    -> *mut ();
    fn sqlite3_db_malloc_zero(_: *mut sqlite3, _: u64)
    -> *mut ();
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
    fn sqlite3_db_free_nn(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_nn_free_nn(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_malloc_size(_: *const ())
    -> i32;
    fn sqlite3_page_malloc(_: i32)
    -> *mut ();
    fn sqlite3_page_free(_: *mut ())
    -> ();
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
    fn sqlite3MutexAlloc(_: i32)
    -> *mut sqlite3_mutex;
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
    fn sqlite3_oom_fault(_: *mut sqlite3)
    -> *mut ();
    fn sqlite3_set_string(_: *mut *mut i8, _: *mut sqlite3, _: *const i8)
    -> ();
    fn sqlite3_progress_check(_: *mut Parse)
    -> ();
    fn sqlite3_error_msg(_: *mut Parse, _: *const i8, ...)
    -> ();
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
    fn sqlite3_fault_sim(_: i32)
    -> i32;
    fn sqlite3_bitvec_create(_: u32)
    -> *mut Bitvec;
    fn sqlite3_bitvec_test(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_bitvec_test_not_null(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_bitvec_set(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_bitvec_clear(_: *mut Bitvec, _: u32, _: *mut ())
    -> ();
    fn sqlite3_bitvec_destroy(_: *mut Bitvec)
    -> ();
    fn sqlite3_bitvec_size(_: *mut Bitvec)
    -> u32;
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
    fn sqlite3_put_varint(_: *mut u8, _: u64)
    -> i32;
    fn sqlite3_get_varint(_: *const u8, _: *mut u64)
    -> u8;
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
    fn sqlite3_writable_schema(_: *mut sqlite3)
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
    fn sqlite3_abs_int32(_: i32)
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
    static mut sqlite3_pending_byte: i32;
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
    fn sqlite3_invoke_busy_handler(_: *mut BusyHandler)
    -> i32;
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
    fn sqlite3_oom_clear(_: *mut sqlite3)
    -> ();
    fn sqlite3_api_exit(db: *mut sqlite3, _: i32)
    -> i32;
    fn sqlite3_open_temp_database(_: *mut Parse)
    -> i32;
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_create_column_expr(_: *mut sqlite3, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut Expr;
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
    fn sqlite3_temp_in_memory(_: *const sqlite3)
    -> i32;
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
    fn sqlite3_get4byte(_: *const u8)
    -> u32;
    fn sqlite3_put4byte(_: *mut u8, _: u32)
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