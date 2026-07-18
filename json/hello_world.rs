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
struct JsonParse {
    a_blob: *mut u8,
    n_blob: u32,
    n_blob_alloc: u32,
    z_json: *mut i8,
    db: *mut sqlite3,
    n_json: i32,
    n_jp_ref: u32,
    i_err: u32,
    i_depth: u16,
    n_err: u8,
    oom: u8,
    b_json_is_rc_str: u8,
    has_nonstd: u8,
    b_read_only: u8,
    e_edit: u8,
    delta: i32,
    n_ins: u32,
    i_label: u32,
    a_ins: *mut u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct JsonCache {
    db: *mut sqlite3,
    n_used: i32,
    a: [*mut JsonParse; 4],
}
extern "C" fn json_cache_search(ctx: *mut sqlite3_context,
    p_arg_1: *mut sqlite3_value) -> *mut JsonParse {
    let mut p: *mut JsonCache = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut z_json: *const i8 = core::ptr::null();
    let mut n_json: i32 = 0;
    if unsafe { sqlite3_value_type(p_arg_1) } != 3 {
        return core::ptr::null_mut();
    }
    z_json = unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
    if z_json == core::ptr::null() { return core::ptr::null_mut(); }
    n_json = unsafe { sqlite3_value_bytes(p_arg_1) };
    p = unsafe { sqlite3_get_auxdata(ctx, -429938) } as *mut JsonCache;
    if p == core::ptr::null_mut() { return core::ptr::null_mut(); }
    {
        i = 0;
        '__b0: loop {
            if !(i < unsafe { (*p).n_used }) { break '__b0; }
            '__c0: loop {
                if unsafe { (*unsafe { (*p).a[i as usize] }).z_json } as
                            *const i8 == z_json {
                    break '__b0;
                }
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if i >= unsafe { (*p).n_used } {
        {
            i = 0;
            '__b1: loop {
                if !(i < unsafe { (*p).n_used }) { break '__b1; }
                '__c1: loop {
                    if unsafe { (*unsafe { (*p).a[i as usize] }).n_json } !=
                            n_json {
                        break '__c1;
                    }
                    if unsafe {
                                memcmp(unsafe { (*unsafe { (*p).a[i as usize] }).z_json } as
                                        *const (), z_json as *const (), n_json as u64)
                            } == 0 {
                        break '__b1;
                    }
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if i < unsafe { (*p).n_used } {
        if i < unsafe { (*p).n_used } - 1 {
            let tmp: *mut JsonParse = unsafe { (*p).a[i as usize] };
            unsafe {
                memmove(unsafe { &raw mut (*p).a[i as usize] } as *mut (),
                    unsafe { &raw mut (*p).a[(i + 1) as usize] } as *const (),
                    (unsafe { (*p).n_used } - i - 1) as u64 *
                        core::mem::size_of::<*mut JsonParse>() as u64)
            };
            unsafe { (*p).a[(unsafe { (*p).n_used } - 1) as usize] = tmp };
            i = unsafe { (*p).n_used } - 1;
        }
        { let _ = 0; };
        return unsafe { (*p).a[i as usize] };
    } else { return core::ptr::null_mut(); }
}
extern "C" fn json_parse_reset(p_parse_1: &mut JsonParse) -> () {
    { let _ = 0; };
    if (*p_parse_1).b_json_is_rc_str != 0 {
        unsafe { sqlite3_rc_str_unref((*p_parse_1).z_json as *mut ()) };
        (*p_parse_1).z_json = core::ptr::null_mut();
        (*p_parse_1).n_json = 0;
        (*p_parse_1).b_json_is_rc_str = 0 as u8;
    }
    if (*p_parse_1).n_blob_alloc != 0 {
        unsafe {
            sqlite3_db_free((*p_parse_1).db, (*p_parse_1).a_blob as *mut ())
        };
        (*p_parse_1).a_blob = core::ptr::null_mut();
        (*p_parse_1).n_blob = 0 as u32;
        (*p_parse_1).n_blob_alloc = 0 as u32;
    }
}
extern "C" fn json_parse_free(p_parse_1: *mut JsonParse) -> () {
    if !(p_parse_1).is_null() {
        if unsafe { (*p_parse_1).n_jp_ref } > 1 as u32 {
            {
                let __p = unsafe { &mut (*p_parse_1).n_jp_ref };
                let __t = *__p;
                *__p -= 1;
                __t
            };
        } else {
            json_parse_reset(unsafe { &mut *p_parse_1 });
            unsafe {
                sqlite3_db_free(unsafe { (*p_parse_1).db },
                    p_parse_1 as *mut ())
            };
        }
    }
}
extern "C" fn jsonb_payload_size(p_parse_1: &JsonParse, i: u32,
    p_sz_1: &mut u32) -> u32 {
    let mut x: u8 = 0 as u8;
    let mut sz: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    if i >= (*p_parse_1).n_blob {
        *p_sz_1 = 0 as u32;
        return 0 as u32;
    } else if {
                    x =
                        (unsafe { *(*p_parse_1).a_blob.add(i as usize) } as i32 >>
                                4) as u8;
                    x
                } as i32 <= 11 {
        sz = x as u32;
        n = 1 as u32;
    } else if x as i32 == 12 {
        if i + 1 as u32 >= (*p_parse_1).n_blob {
            *p_sz_1 = 0 as u32;
            return 0 as u32;
        }
        sz =
            unsafe { *(*p_parse_1).a_blob.add((i + 1 as u32) as usize) } as
                u32;
        n = 2 as u32;
    } else if x as i32 == 13 {
        if i + 2 as u32 >= (*p_parse_1).n_blob {
            *p_sz_1 = 0 as u32;
            return 0 as u32;
        }
        sz =
            (((unsafe { *(*p_parse_1).a_blob.add((i + 1 as u32) as usize) } as
                                i32) << 8) +
                    unsafe { *(*p_parse_1).a_blob.add((i + 2 as u32) as usize) }
                        as i32) as u32;
        n = 3 as u32;
    } else if x as i32 == 14 {
        if i + 4 as u32 >= (*p_parse_1).n_blob {
            *p_sz_1 = 0 as u32;
            return 0 as u32;
        }
        sz =
            ((unsafe { *(*p_parse_1).a_blob.add((i + 1 as u32) as usize) } as
                                    u32) << 24) +
                        ((unsafe {
                                            *(*p_parse_1).a_blob.add((i + 2 as u32) as usize)
                                        } as i32) << 16) as u32 +
                    ((unsafe {
                                        *(*p_parse_1).a_blob.add((i + 3 as u32) as usize)
                                    } as i32) << 8) as u32 +
                unsafe { *(*p_parse_1).a_blob.add((i + 4 as u32) as usize) }
                    as u32;
        n = 5 as u32;
    } else {
        if i + 8 as u32 >= (*p_parse_1).n_blob ||
                            unsafe { *(*p_parse_1).a_blob.add((i + 1 as u32) as usize) }
                                    as i32 != 0 ||
                        unsafe { *(*p_parse_1).a_blob.add((i + 2 as u32) as usize) }
                                as i32 != 0 ||
                    unsafe { *(*p_parse_1).a_blob.add((i + 3 as u32) as usize) }
                            as i32 != 0 ||
                unsafe { *(*p_parse_1).a_blob.add((i + 4 as u32) as usize) }
                        as i32 != 0 {
            *p_sz_1 = 0 as u32;
            return 0 as u32;
        }
        sz =
            ((unsafe { *(*p_parse_1).a_blob.add((i + 5 as u32) as usize) } as
                                    u32) << 24) +
                        ((unsafe {
                                            *(*p_parse_1).a_blob.add((i + 6 as u32) as usize)
                                        } as i32) << 16) as u32 +
                    ((unsafe {
                                        *(*p_parse_1).a_blob.add((i + 7 as u32) as usize)
                                    } as i32) << 8) as u32 +
                unsafe { *(*p_parse_1).a_blob.add((i + 8 as u32) as usize) }
                    as u32;
        n = 9 as u32;
    }
    if i as i64 + sz as i64 + n as i64 > (*p_parse_1).n_blob as i64 &&
            i as i64 + sz as i64 + n as i64 >
                ((*p_parse_1).n_blob - (*p_parse_1).delta as u32) as i64 {
        *p_sz_1 = 0 as u32;
        return 0 as u32;
    }
    *p_sz_1 = sz;
    return n;
}
static json_is_ok: [i8; 256] =
    [0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 1 as i8, 1 as i8, 0 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 0 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            0 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8, 1 as i8,
            1 as i8, 1 as i8, 1 as i8];
extern "C" fn json_is2_hex(z: *const i8) -> i32 {
    unsafe {
        return (unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                                usize)
                                } as i32 & 8 != 0 &&
                    unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe { *z.offset(1 as isize) } as u8 as
                                                usize)
                                } as i32 & 8 != 0) as i32;
    }
}
extern "C" fn json_is4_hex(z: *const i8) -> i32 {
    return (json_is2_hex(z) != 0 &&
                json_is2_hex(unsafe { &*z.offset(2 as isize) }) != 0) as i32;
}
extern "C" fn json_hex_to_int(mut h: i32) -> u8 {
    h += 9 * (1 & h >> 6);
    return (h & 15) as u8;
}
extern "C" fn json_hex_to_int4(z: *const i8) -> u32 {
    let mut v: u32 = 0 as u32;
    v =
        (((json_hex_to_int(unsafe { *z.offset(0 as isize) } as i32) as i32) <<
                            12) +
                        ((json_hex_to_int(unsafe { *z.offset(1 as isize) } as i32)
                                    as i32) << 8) +
                    ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                as i32) << 4) +
                json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                    i32) as u32;
    return v;
}
extern "C" fn json_bytes_to_bypass(z: *const i8, n: u32) -> u32 {
    let mut i: u32 = 0 as u32;
    while (i + 1 as u32) < n {
        if unsafe { *z.add(i as usize) } as i32 != '\\' as i32 { return i; }
        if unsafe { *z.add((i + 1 as u32) as usize) } as i32 == '\n' as i32 {
            i += 2 as u32;
            continue;
        }
        if unsafe { *z.add((i + 1 as u32) as usize) } as i32 == '\r' as i32 {
            if (i + 2 as u32) < n &&
                    unsafe { *z.add((i + 2 as u32) as usize) } as i32 ==
                        '\n' as i32 {
                i += 3 as u32;
            } else { i += 2 as u32; }
            continue;
        }
        if 226 == unsafe { *z.add((i + 1 as u32) as usize) } as u8 as i32 &&
                        (i + 3 as u32) < n &&
                    128 ==
                        unsafe { *z.add((i + 2 as u32) as usize) } as u8 as i32 &&
                (168 ==
                        unsafe { *z.add((i + 3 as u32) as usize) } as u8 as i32 ||
                    169 ==
                        unsafe { *z.add((i + 3 as u32) as usize) } as u8 as i32) {
            i += 4 as u32;
            continue;
        }
        break;
    }
    return i;
}
extern "C" fn json_unescape_one_char(z: *const i8, n: u32, pi_out: *mut u32)
    -> u32 {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if n < 2 as u32 { unsafe { *pi_out = 629145 as u32 }; return n; }
        '__s3:
            {
            match unsafe { *z.offset(1 as isize) } as u8 {
                117 => {
                    {
                        let mut v: u32 = 0 as u32;
                        let mut vlo: u32 = 0 as u32;
                        if n < 6 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        v = json_hex_to_int4(unsafe { &*z.offset(2 as isize) });
                        if v & 64512 as u32 == 55296 as u32 && n >= 12 as u32 &&
                                        unsafe { *z.offset(6 as isize) } as i32 == '\\' as i32 &&
                                    unsafe { *z.offset(7 as isize) } as i32 == 'u' as i32 &&
                                {
                                            vlo = json_hex_to_int4(unsafe { &*z.offset(8 as isize) });
                                            vlo
                                        } & 64512 as u32 == 56320 as u32 {
                            unsafe {
                                *pi_out =
                                    ((v & 1023 as u32) << 10) + (vlo & 1023 as u32) +
                                        65536 as u32
                            };
                            return 12 as u32;
                        } else { unsafe { *pi_out = v }; return 6 as u32; }
                    }
                    {
                        unsafe { *pi_out = '\u{8}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\u{c}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\n' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\r' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\t' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\u{b}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out =
                                if n > 2 as u32 &&
                                            unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(2 as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 4 != 0 {
                                        629145
                                    } else { 0 } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                98 => {
                    {
                        unsafe { *pi_out = '\u{8}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\u{c}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\n' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\r' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\t' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\u{b}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out =
                                if n > 2 as u32 &&
                                            unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(2 as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 4 != 0 {
                                        629145
                                    } else { 0 } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                102 => {
                    {
                        unsafe { *pi_out = '\u{c}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\n' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\r' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\t' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\u{b}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out =
                                if n > 2 as u32 &&
                                            unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(2 as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 4 != 0 {
                                        629145
                                    } else { 0 } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                110 => {
                    {
                        unsafe { *pi_out = '\n' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\r' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\t' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\u{b}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out =
                                if n > 2 as u32 &&
                                            unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(2 as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 4 != 0 {
                                        629145
                                    } else { 0 } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                114 => {
                    {
                        unsafe { *pi_out = '\r' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\t' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\u{b}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out =
                                if n > 2 as u32 &&
                                            unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(2 as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 4 != 0 {
                                        629145
                                    } else { 0 } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                116 => {
                    {
                        unsafe { *pi_out = '\t' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe { *pi_out = '\u{b}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out =
                                if n > 2 as u32 &&
                                            unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(2 as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 4 != 0 {
                                        629145
                                    } else { 0 } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                118 => {
                    {
                        unsafe { *pi_out = '\u{b}' as i32 as u32 };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out =
                                if n > 2 as u32 &&
                                            unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(2 as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 4 != 0 {
                                        629145
                                    } else { 0 } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                48 => {
                    {
                        unsafe {
                            *pi_out =
                                if n > 2 as u32 &&
                                            unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.offset(2 as isize) } as u8 as
                                                                        usize)
                                                        } as i32 & 4 != 0 {
                                        629145
                                    } else { 0 } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                39 => {
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                34 => {
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                47 => {
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                92 => {
                    {
                        unsafe {
                            *pi_out = unsafe { *z.offset(1 as isize) } as u32
                        };
                        return 2 as u32;
                    }
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                120 => {
                    {
                        if n < 4 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        }
                        unsafe {
                            *pi_out =
                                ((json_hex_to_int(unsafe { *z.offset(2 as isize) } as i32)
                                                    as i32) << 4 |
                                        json_hex_to_int(unsafe { *z.offset(3 as isize) } as i32) as
                                            i32) as u32
                        };
                        return 4 as u32;
                    }
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                226 => {
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                13 => {
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                10 => {
                    {
                        let n_skip: u32 = json_bytes_to_bypass(z, n);
                        if n_skip == 0 as u32 {
                            unsafe { *pi_out = 629145 as u32 };
                            return n;
                        } else if n_skip == n {
                            unsafe { *pi_out = 0 as u32 };
                            return n;
                        } else if unsafe { *z.add(n_skip as usize) } as i32 ==
                                '\\' as i32 {
                            return n_skip +
                                    json_unescape_one_char(unsafe { &*z.add(n_skip as usize) },
                                        n - n_skip, pi_out);
                        } else {
                            let sz: i32 =
                                unsafe {
                                    sqlite3_utf8_read_limited(unsafe {
                                                    &raw const *z.add(n_skip as usize)
                                                } as *mut u8 as *const u8, (n - n_skip) as i32, pi_out)
                                };
                            return n_skip + sz as u32;
                        }
                    }
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
                _ => {
                    { unsafe { *pi_out = 629145 as u32 }; return 2 as u32; }
                }
            }
        }
    }
}
extern "C" fn jsonb_validity_check(p_parse_1: *const JsonParse, i: u32,
    i_end_1: u32, i_depth_1: u32) -> u32 {
    unsafe {
        let mut n: u32 = 0 as u32;
        let mut sz: u32 = 0 as u32;
        let mut j: u32 = 0 as u32;
        let mut k: u32 = 0 as u32;
        let mut z: *const u8 = core::ptr::null();
        let mut x: u8 = 0 as u8;
        if i_depth_1 > 1000 as u32 { return i + 1 as u32; }
        sz = 0 as u32;
        n = unsafe { jsonb_payload_size(unsafe { &*p_parse_1 }, i, &mut sz) };
        if n == 0 as u32 { return i + 1 as u32; }
        if i + n + sz != i_end_1 { return i + 1 as u32; }
        z = unsafe { (*p_parse_1).a_blob } as *const u8;
        x = (unsafe { *z.add(i as usize) } as i32 & 15) as u8;
        '__s4:
            {
            match x {
                0 => {
                    {
                        return if n + sz == 1 as u32 {
                                0 as u32
                            } else { i + 1 as u32 };
                    }
                    {
                        if sz < 1 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 2 as u32 { return i + 1 as u32; }
                        }
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 4 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        if sz < 3 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            if sz < 4 as u32 { return i + 1 as u32; }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { *z.add(j as usize) } as i32 != '0' as i32 {
                            return i + 1 as u32;
                        }
                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'x' as i32 &&
                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'X' as i32 {
                            return j + 2 as u32;
                        }
                        j += 2 as u32;
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 8 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        let mut seen: u8 = 0 as u8;
                        if sz < 2 as u32 { return i + 1 as u32; }
                        j = i + n;
                        k = j + sz;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 3 as u32 { return i + 1 as u32; }
                        }
                        if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                            if x as i32 == 5 { return j + 1 as u32; }
                            if (unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                        u8 as usize)
                                                    } as i32 & 4 == 0) as i32 != 0 {
                                return j + 1 as u32;
                            }
                            j += 2 as u32;
                            seen = 1 as u8;
                        } else if unsafe { *z.add(j as usize) } as i32 == '0' as i32
                                && x as i32 == 5 {
                            if j + 3 as u32 > k { return j + 1 as u32; }
                            if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            '.' as i32 &&
                                        unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            'e' as i32 &&
                                    unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                        'E' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        {
                            '__b7: loop {
                                if !(j < k) { break '__b7; }
                                '__c7: loop {
                                    if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                                    } as i32 & 4 != 0 {
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                                        if seen as i32 > 0 { return j + 1 as u32; }
                                        if x as i32 == 5 &&
                                                (j == k - 1 as u32 ||
                                                    (unsafe {
                                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                                        *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                                            u8 as usize)
                                                                        } as i32 & 4 == 0) as i32 != 0) {
                                            return j + 1 as u32;
                                        }
                                        seen = 1 as u8;
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == 'e' as i32 ||
                                            unsafe { *z.add(j as usize) } as i32 == 'E' as i32 {
                                        if seen as i32 == 2 { return j + 1 as u32; }
                                        if j == k - 1 as u32 { return j + 1 as u32; }
                                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '+' as i32 ||
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '-' as i32 {
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            if j == k - 1 as u32 { return j + 1 as u32; }
                                        }
                                        seen = 2 as u8;
                                        break '__c7;
                                    }
                                    return j + 1 as u32;
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if seen as i32 == 0 { return i + 1 as u32; }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                1 => {
                    {
                        return if n + sz == 1 as u32 {
                                0 as u32
                            } else { i + 1 as u32 };
                    }
                    {
                        if sz < 1 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 2 as u32 { return i + 1 as u32; }
                        }
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 4 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        if sz < 3 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            if sz < 4 as u32 { return i + 1 as u32; }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { *z.add(j as usize) } as i32 != '0' as i32 {
                            return i + 1 as u32;
                        }
                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'x' as i32 &&
                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'X' as i32 {
                            return j + 2 as u32;
                        }
                        j += 2 as u32;
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 8 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        let mut seen: u8 = 0 as u8;
                        if sz < 2 as u32 { return i + 1 as u32; }
                        j = i + n;
                        k = j + sz;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 3 as u32 { return i + 1 as u32; }
                        }
                        if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                            if x as i32 == 5 { return j + 1 as u32; }
                            if (unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                        u8 as usize)
                                                    } as i32 & 4 == 0) as i32 != 0 {
                                return j + 1 as u32;
                            }
                            j += 2 as u32;
                            seen = 1 as u8;
                        } else if unsafe { *z.add(j as usize) } as i32 == '0' as i32
                                && x as i32 == 5 {
                            if j + 3 as u32 > k { return j + 1 as u32; }
                            if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            '.' as i32 &&
                                        unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            'e' as i32 &&
                                    unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                        'E' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        {
                            '__b7: loop {
                                if !(j < k) { break '__b7; }
                                '__c7: loop {
                                    if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                                    } as i32 & 4 != 0 {
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                                        if seen as i32 > 0 { return j + 1 as u32; }
                                        if x as i32 == 5 &&
                                                (j == k - 1 as u32 ||
                                                    (unsafe {
                                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                                        *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                                            u8 as usize)
                                                                        } as i32 & 4 == 0) as i32 != 0) {
                                            return j + 1 as u32;
                                        }
                                        seen = 1 as u8;
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == 'e' as i32 ||
                                            unsafe { *z.add(j as usize) } as i32 == 'E' as i32 {
                                        if seen as i32 == 2 { return j + 1 as u32; }
                                        if j == k - 1 as u32 { return j + 1 as u32; }
                                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '+' as i32 ||
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '-' as i32 {
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            if j == k - 1 as u32 { return j + 1 as u32; }
                                        }
                                        seen = 2 as u8;
                                        break '__c7;
                                    }
                                    return j + 1 as u32;
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if seen as i32 == 0 { return i + 1 as u32; }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                2 => {
                    {
                        return if n + sz == 1 as u32 {
                                0 as u32
                            } else { i + 1 as u32 };
                    }
                    {
                        if sz < 1 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 2 as u32 { return i + 1 as u32; }
                        }
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 4 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        if sz < 3 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            if sz < 4 as u32 { return i + 1 as u32; }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { *z.add(j as usize) } as i32 != '0' as i32 {
                            return i + 1 as u32;
                        }
                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'x' as i32 &&
                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'X' as i32 {
                            return j + 2 as u32;
                        }
                        j += 2 as u32;
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 8 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        let mut seen: u8 = 0 as u8;
                        if sz < 2 as u32 { return i + 1 as u32; }
                        j = i + n;
                        k = j + sz;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 3 as u32 { return i + 1 as u32; }
                        }
                        if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                            if x as i32 == 5 { return j + 1 as u32; }
                            if (unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                        u8 as usize)
                                                    } as i32 & 4 == 0) as i32 != 0 {
                                return j + 1 as u32;
                            }
                            j += 2 as u32;
                            seen = 1 as u8;
                        } else if unsafe { *z.add(j as usize) } as i32 == '0' as i32
                                && x as i32 == 5 {
                            if j + 3 as u32 > k { return j + 1 as u32; }
                            if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            '.' as i32 &&
                                        unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            'e' as i32 &&
                                    unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                        'E' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        {
                            '__b7: loop {
                                if !(j < k) { break '__b7; }
                                '__c7: loop {
                                    if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                                    } as i32 & 4 != 0 {
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                                        if seen as i32 > 0 { return j + 1 as u32; }
                                        if x as i32 == 5 &&
                                                (j == k - 1 as u32 ||
                                                    (unsafe {
                                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                                        *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                                            u8 as usize)
                                                                        } as i32 & 4 == 0) as i32 != 0) {
                                            return j + 1 as u32;
                                        }
                                        seen = 1 as u8;
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == 'e' as i32 ||
                                            unsafe { *z.add(j as usize) } as i32 == 'E' as i32 {
                                        if seen as i32 == 2 { return j + 1 as u32; }
                                        if j == k - 1 as u32 { return j + 1 as u32; }
                                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '+' as i32 ||
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '-' as i32 {
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            if j == k - 1 as u32 { return j + 1 as u32; }
                                        }
                                        seen = 2 as u8;
                                        break '__c7;
                                    }
                                    return j + 1 as u32;
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if seen as i32 == 0 { return i + 1 as u32; }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                3 => {
                    {
                        if sz < 1 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 2 as u32 { return i + 1 as u32; }
                        }
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 4 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        if sz < 3 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            if sz < 4 as u32 { return i + 1 as u32; }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { *z.add(j as usize) } as i32 != '0' as i32 {
                            return i + 1 as u32;
                        }
                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'x' as i32 &&
                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'X' as i32 {
                            return j + 2 as u32;
                        }
                        j += 2 as u32;
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 8 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        let mut seen: u8 = 0 as u8;
                        if sz < 2 as u32 { return i + 1 as u32; }
                        j = i + n;
                        k = j + sz;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 3 as u32 { return i + 1 as u32; }
                        }
                        if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                            if x as i32 == 5 { return j + 1 as u32; }
                            if (unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                        u8 as usize)
                                                    } as i32 & 4 == 0) as i32 != 0 {
                                return j + 1 as u32;
                            }
                            j += 2 as u32;
                            seen = 1 as u8;
                        } else if unsafe { *z.add(j as usize) } as i32 == '0' as i32
                                && x as i32 == 5 {
                            if j + 3 as u32 > k { return j + 1 as u32; }
                            if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            '.' as i32 &&
                                        unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            'e' as i32 &&
                                    unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                        'E' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        {
                            '__b7: loop {
                                if !(j < k) { break '__b7; }
                                '__c7: loop {
                                    if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                                    } as i32 & 4 != 0 {
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                                        if seen as i32 > 0 { return j + 1 as u32; }
                                        if x as i32 == 5 &&
                                                (j == k - 1 as u32 ||
                                                    (unsafe {
                                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                                        *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                                            u8 as usize)
                                                                        } as i32 & 4 == 0) as i32 != 0) {
                                            return j + 1 as u32;
                                        }
                                        seen = 1 as u8;
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == 'e' as i32 ||
                                            unsafe { *z.add(j as usize) } as i32 == 'E' as i32 {
                                        if seen as i32 == 2 { return j + 1 as u32; }
                                        if j == k - 1 as u32 { return j + 1 as u32; }
                                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '+' as i32 ||
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '-' as i32 {
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            if j == k - 1 as u32 { return j + 1 as u32; }
                                        }
                                        seen = 2 as u8;
                                        break '__c7;
                                    }
                                    return j + 1 as u32;
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if seen as i32 == 0 { return i + 1 as u32; }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                4 => {
                    {
                        if sz < 3 as u32 { return i + 1 as u32; }
                        j = i + n;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            if sz < 4 as u32 { return i + 1 as u32; }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { *z.add(j as usize) } as i32 != '0' as i32 {
                            return i + 1 as u32;
                        }
                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'x' as i32 &&
                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                    'X' as i32 {
                            return j + 2 as u32;
                        }
                        j += 2 as u32;
                        k = i + n + sz;
                        while j < k {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 8 != 0 {
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            } else { return j + 1 as u32; }
                        }
                        return 0 as u32;
                    }
                    {
                        let mut seen: u8 = 0 as u8;
                        if sz < 2 as u32 { return i + 1 as u32; }
                        j = i + n;
                        k = j + sz;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 3 as u32 { return i + 1 as u32; }
                        }
                        if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                            if x as i32 == 5 { return j + 1 as u32; }
                            if (unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                        u8 as usize)
                                                    } as i32 & 4 == 0) as i32 != 0 {
                                return j + 1 as u32;
                            }
                            j += 2 as u32;
                            seen = 1 as u8;
                        } else if unsafe { *z.add(j as usize) } as i32 == '0' as i32
                                && x as i32 == 5 {
                            if j + 3 as u32 > k { return j + 1 as u32; }
                            if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            '.' as i32 &&
                                        unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            'e' as i32 &&
                                    unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                        'E' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        {
                            '__b7: loop {
                                if !(j < k) { break '__b7; }
                                '__c7: loop {
                                    if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                                    } as i32 & 4 != 0 {
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                                        if seen as i32 > 0 { return j + 1 as u32; }
                                        if x as i32 == 5 &&
                                                (j == k - 1 as u32 ||
                                                    (unsafe {
                                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                                        *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                                            u8 as usize)
                                                                        } as i32 & 4 == 0) as i32 != 0) {
                                            return j + 1 as u32;
                                        }
                                        seen = 1 as u8;
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == 'e' as i32 ||
                                            unsafe { *z.add(j as usize) } as i32 == 'E' as i32 {
                                        if seen as i32 == 2 { return j + 1 as u32; }
                                        if j == k - 1 as u32 { return j + 1 as u32; }
                                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '+' as i32 ||
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '-' as i32 {
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            if j == k - 1 as u32 { return j + 1 as u32; }
                                        }
                                        seen = 2 as u8;
                                        break '__c7;
                                    }
                                    return j + 1 as u32;
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if seen as i32 == 0 { return i + 1 as u32; }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                5 => {
                    {
                        let mut seen: u8 = 0 as u8;
                        if sz < 2 as u32 { return i + 1 as u32; }
                        j = i + n;
                        k = j + sz;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 3 as u32 { return i + 1 as u32; }
                        }
                        if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                            if x as i32 == 5 { return j + 1 as u32; }
                            if (unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                        u8 as usize)
                                                    } as i32 & 4 == 0) as i32 != 0 {
                                return j + 1 as u32;
                            }
                            j += 2 as u32;
                            seen = 1 as u8;
                        } else if unsafe { *z.add(j as usize) } as i32 == '0' as i32
                                && x as i32 == 5 {
                            if j + 3 as u32 > k { return j + 1 as u32; }
                            if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            '.' as i32 &&
                                        unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            'e' as i32 &&
                                    unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                        'E' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        {
                            '__b7: loop {
                                if !(j < k) { break '__b7; }
                                '__c7: loop {
                                    if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                                    } as i32 & 4 != 0 {
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                                        if seen as i32 > 0 { return j + 1 as u32; }
                                        if x as i32 == 5 &&
                                                (j == k - 1 as u32 ||
                                                    (unsafe {
                                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                                        *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                                            u8 as usize)
                                                                        } as i32 & 4 == 0) as i32 != 0) {
                                            return j + 1 as u32;
                                        }
                                        seen = 1 as u8;
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == 'e' as i32 ||
                                            unsafe { *z.add(j as usize) } as i32 == 'E' as i32 {
                                        if seen as i32 == 2 { return j + 1 as u32; }
                                        if j == k - 1 as u32 { return j + 1 as u32; }
                                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '+' as i32 ||
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '-' as i32 {
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            if j == k - 1 as u32 { return j + 1 as u32; }
                                        }
                                        seen = 2 as u8;
                                        break '__c7;
                                    }
                                    return j + 1 as u32;
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if seen as i32 == 0 { return i + 1 as u32; }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                6 => {
                    {
                        let mut seen: u8 = 0 as u8;
                        if sz < 2 as u32 { return i + 1 as u32; }
                        j = i + n;
                        k = j + sz;
                        if unsafe { *z.add(j as usize) } as i32 == '-' as i32 {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            if sz < 3 as u32 { return i + 1 as u32; }
                        }
                        if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                            if x as i32 == 5 { return j + 1 as u32; }
                            if (unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                        u8 as usize)
                                                    } as i32 & 4 == 0) as i32 != 0 {
                                return j + 1 as u32;
                            }
                            j += 2 as u32;
                            seen = 1 as u8;
                        } else if unsafe { *z.add(j as usize) } as i32 == '0' as i32
                                && x as i32 == 5 {
                            if j + 3 as u32 > k { return j + 1 as u32; }
                            if unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            '.' as i32 &&
                                        unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                            'e' as i32 &&
                                    unsafe { *z.add((j + 1 as u32) as usize) } as i32 !=
                                        'E' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        {
                            '__b7: loop {
                                if !(j < k) { break '__b7; }
                                '__c7: loop {
                                    if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                                    } as i32 & 4 != 0 {
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == '.' as i32 {
                                        if seen as i32 > 0 { return j + 1 as u32; }
                                        if x as i32 == 5 &&
                                                (j == k - 1 as u32 ||
                                                    (unsafe {
                                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                                        *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                                            u8 as usize)
                                                                        } as i32 & 4 == 0) as i32 != 0) {
                                            return j + 1 as u32;
                                        }
                                        seen = 1 as u8;
                                        break '__c7;
                                    }
                                    if unsafe { *z.add(j as usize) } as i32 == 'e' as i32 ||
                                            unsafe { *z.add(j as usize) } as i32 == 'E' as i32 {
                                        if seen as i32 == 2 { return j + 1 as u32; }
                                        if j == k - 1 as u32 { return j + 1 as u32; }
                                        if unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '+' as i32 ||
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                                    '-' as i32 {
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            if j == k - 1 as u32 { return j + 1 as u32; }
                                        }
                                        seen = 2 as u8;
                                        break '__c7;
                                    }
                                    return j + 1 as u32;
                                    break '__c7;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if seen as i32 == 0 { return i + 1 as u32; }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                7 => {
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                return j + 1 as u32;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                8 => {
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                9 => {
                    {
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            if (json_is_ok[unsafe { *z.add(j as usize) } as usize] == 0)
                                            as i32 != 0 &&
                                    unsafe { *z.add(j as usize) } as i32 != '\'' as i32 {
                                if unsafe { *z.add(j as usize) } as i32 == '\"' as i32 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 <= 31 {
                                    if x as i32 == 8 { return j + 1 as u32; }
                                } else if unsafe { *z.add(j as usize) } as i32 !=
                                            '\\' as i32 || j + 1 as u32 >= k {
                                    return j + 1 as u32;
                                } else if unsafe {
                                            strchr(c"\"\\/bfnrt".as_ptr() as *mut i8 as *const i8,
                                                unsafe { *z.add((j + 1 as u32) as usize) } as i32)
                                        } != core::ptr::null_mut() {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if unsafe { *z.add((j + 1 as u32) as usize) } as i32
                                        == 'u' as i32 {
                                    if j + 5 as u32 >= k { return j + 1 as u32; }
                                    if (json_is4_hex(unsafe {
                                                                &raw const *z.add((j + 2 as u32) as usize)
                                                            } as *const i8) == 0) as i32 != 0 {
                                        return j + 1 as u32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                } else if x as i32 != 9 {
                                    return j + 1 as u32;
                                } else {
                                    let mut c: u32 = 0 as u32;
                                    let sz_c: u32 =
                                        json_unescape_one_char(unsafe {
                                                    &raw const *z.add(j as usize)
                                                } as *const i8, k - j, &mut c);
                                    if c == 629145 as u32 { return j + 1 as u32; }
                                    j += sz_c - 1 as u32;
                                }
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        return 0 as u32;
                    }
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                10 => {
                    { return 0 as u32; }
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                11 => {
                    {
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            j += n + sz;
                        }
                        { let _ = 0; };
                        return 0 as u32;
                    }
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                12 => {
                    {
                        let mut cnt: u32 = 0 as u32;
                        let mut sub: u32 = 0 as u32;
                        j = i + n;
                        k = j + sz;
                        while j < k {
                            sz = 0 as u32;
                            n =
                                unsafe {
                                    jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz)
                                };
                            if n == 0 as u32 { return j + 1 as u32; }
                            if j + n + sz > k { return j + 1 as u32; }
                            if cnt & 1 as u32 == 0 as u32 {
                                x = (unsafe { *z.add(j as usize) } as i32 & 15) as u8;
                                if (x as i32) < 7 || x as i32 > 10 { return j + 1 as u32; }
                            }
                            sub =
                                jsonb_validity_check(p_parse_1, j, j + n + sz,
                                    i_depth_1 + 1 as u32);
                            if sub != 0 { return sub; }
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            j += n + sz;
                        }
                        { let _ = 0; };
                        if cnt & 1 as u32 != 0 as u32 { return j + 1 as u32; }
                        return 0 as u32;
                    }
                    { return i + 1 as u32; }
                }
                _ => { { return i + 1 as u32; } }
            }
        }
    }
}
extern "C" fn json_arg_is_jsonb(p_arg_1: *mut sqlite3_value,
    p: *mut JsonParse) -> i32 {
    unsafe {
        let mut n: u32 = 0 as u32;
        let mut sz: u32 = 0 as u32;
        let mut c: u8 = 0 as u8;
        if unsafe { sqlite3_value_type(p_arg_1) } != 4 { return 0; }
        unsafe {
            (*p).a_blob = unsafe { sqlite3_value_blob(p_arg_1) } as *mut u8
        };
        unsafe {
            (*p).n_blob = unsafe { sqlite3_value_bytes(p_arg_1) } as u32
        };
        if unsafe { (*p).n_blob } > 0 as u32 &&
                                    unsafe { (*p).a_blob } != core::ptr::null_mut() &&
                                {
                                                c = unsafe { *unsafe { (*p).a_blob.offset(0 as isize) } };
                                                c
                                            } as i32 & 15 <= 12 &&
                            {
                                    n = jsonb_payload_size(unsafe { &*p }, 0 as u32, &mut sz);
                                    n
                                } > 0 as u32 && sz + n == unsafe { (*p).n_blob } &&
                    (c as i32 & 15 > 2 || sz == 0 as u32) &&
                (sz > 7 as u32 ||
                        c as i32 != 123 && c as i32 != 91 &&
                            (unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(c as u8 as usize)
                                                } as i32 & 4 == 0) as i32 != 0 ||
                    jsonb_validity_check(p as *const JsonParse, 0 as u32,
                            unsafe { (*p).n_blob }, 1 as u32) == 0 as u32) {
            return 1;
        }
        unsafe { (*p).a_blob = core::ptr::null_mut() };
        unsafe { (*p).n_blob = 0 as u32 };
        return 0;
    }
}
extern "C" fn json_blob_expand(p_parse_1: &mut JsonParse, n_1: u32) -> i32 {
    let mut a_new: *mut u8 = core::ptr::null_mut();
    let mut t: u64 = 0 as u64;
    { let _ = 0; };
    if (*p_parse_1).n_blob_alloc == 0 as u32 {
        t = 100 as u64;
    } else { t = ((*p_parse_1).n_blob_alloc * 2 as u32) as u64; }
    if t < n_1 as u64 { t = (n_1 + 100 as u32) as u64; }
    a_new =
        unsafe {
                sqlite3_db_realloc((*p_parse_1).db,
                    (*p_parse_1).a_blob as *mut (), t)
            } as *mut u8;
    if a_new == core::ptr::null_mut() {
        (*p_parse_1).oom = 1 as u8;
        return 1;
    }
    { let _ = 0; };
    (*p_parse_1).a_blob = a_new;
    (*p_parse_1).n_blob_alloc = t as u32;
    return 0;
}
extern "C" fn json_blob_make_editable(p_parse_1: *mut JsonParse,
    n_extra_1: u32) -> i32 {
    let mut a_old: *const u8 = core::ptr::null();
    let mut n_size: u32 = 0 as u32;
    { let _ = 0; };
    if unsafe { (*p_parse_1).oom } != 0 { return 0; }
    if unsafe { (*p_parse_1).n_blob_alloc } > 0 as u32 { return 1; }
    a_old = unsafe { (*p_parse_1).a_blob };
    n_size = unsafe { (*p_parse_1).n_blob } + n_extra_1;
    unsafe { (*p_parse_1).a_blob = core::ptr::null_mut() };
    if json_blob_expand(unsafe { &mut *p_parse_1 }, n_size) != 0 { return 0; }
    { let _ = 0; };
    unsafe {
        memcpy(unsafe { (*p_parse_1).a_blob } as *mut (), a_old as *const (),
            unsafe { (*p_parse_1).n_blob } as u64)
    };
    return 1;
}
extern "C" fn json_blob_expand_and_append_node(p_parse_1: *mut JsonParse,
    e_type_1: u8, sz_payload_1: u64, a_payload_1: *const ()) -> () {
    if json_blob_expand(unsafe { &mut *p_parse_1 },
                (unsafe { (*p_parse_1).n_blob } as u64 + sz_payload_1 +
                        9 as u64) as u32) != 0 {
        return;
    }
    unsafe {
        json_blob_append_node(p_parse_1, e_type_1, sz_payload_1, a_payload_1)
    };
}
extern "C" fn json_blob_append_node(p_parse_1: *mut JsonParse, e_type_1: u8,
    sz_payload_1: u64, a_payload_1: *const ()) -> () {
    let mut a: *mut u8 = core::ptr::null_mut();
    if unsafe { (*p_parse_1).n_blob } as u64 + sz_payload_1 + 9 as u64 >
            unsafe { (*p_parse_1).n_blob_alloc } as u64 {
        json_blob_expand_and_append_node(p_parse_1, e_type_1, sz_payload_1,
            a_payload_1);
        return;
    }
    { let _ = 0; };
    a =
        unsafe {
            unsafe {
                (*p_parse_1).a_blob.add(unsafe { (*p_parse_1).n_blob } as
                        usize)
            }
        };
    if sz_payload_1 <= 11 as u64 {
        unsafe {
            *a.offset(0 as isize) =
                (e_type_1 as u64 | sz_payload_1 << 4) as u8
        };
        unsafe { (*p_parse_1).n_blob += 1 as u32 };
    } else if sz_payload_1 <= 255 as u64 {
        unsafe { *a.offset(0 as isize) = (e_type_1 as i32 | 192) as u8 };
        unsafe { *a.offset(1 as isize) = (sz_payload_1 & 255 as u64) as u8 };
        unsafe { (*p_parse_1).n_blob += 2 as u32 };
    } else if sz_payload_1 <= 65535 as u64 {
        unsafe { *a.offset(0 as isize) = (e_type_1 as i32 | 208) as u8 };
        unsafe {
            *a.offset(1 as isize) = (sz_payload_1 >> 8 & 255 as u64) as u8
        };
        unsafe { *a.offset(2 as isize) = (sz_payload_1 & 255 as u64) as u8 };
        unsafe { (*p_parse_1).n_blob += 3 as u32 };
    } else {
        unsafe { *a.offset(0 as isize) = (e_type_1 as i32 | 224) as u8 };
        unsafe {
            *a.offset(1 as isize) = (sz_payload_1 >> 24 & 255 as u64) as u8
        };
        unsafe {
            *a.offset(2 as isize) = (sz_payload_1 >> 16 & 255 as u64) as u8
        };
        unsafe {
            *a.offset(3 as isize) = (sz_payload_1 >> 8 & 255 as u64) as u8
        };
        unsafe { *a.offset(4 as isize) = (sz_payload_1 & 255 as u64) as u8 };
        unsafe { (*p_parse_1).n_blob += 5 as u32 };
    }
    if !(a_payload_1).is_null() {
        unsafe { (*p_parse_1).n_blob += sz_payload_1 as u32 };
        unsafe {
            memcpy(unsafe {
                        &raw mut *unsafe {
                                    (*p_parse_1).a_blob.add((unsafe { (*p_parse_1).n_blob } as
                                                    u64 - sz_payload_1) as usize)
                                }
                    } as *mut (), a_payload_1, sz_payload_1)
        };
    }
}
extern "C" fn json5_whitespace(z_in_1: *const i8) -> i32 {
    let mut n: i32 = 0;
    let mut z: *const u8 = core::ptr::null();
    let mut j: i32 = 0;
    let mut j__1: i32 = 0;
    let mut c: i8 = 0 as i8;
    let mut c__1: u8 = 0 as u8;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s13:
            {
            match __state {
                0 => { n = 0; __state = 3; }
                2 => { return n; }
                3 => { z = z_in_1 as *mut u8 as *const u8; __state = 4; }
                4 => { if 1 != 0 { __state = 6; } else { __state = 5; } }
                5 => { __state = 2; }
                6 => {
                    '__s14:
                        {
                        match unsafe { *z.offset(n as isize) } {
                            9 => { __state = 7; }
                            10 => { __state = 8; }
                            11 => { __state = 9; }
                            12 => { __state = 10; }
                            13 => { __state = 11; }
                            32 => { __state = 12; }
                            47 => { __state = 13; }
                            194 => { __state = 14; }
                            225 => { __state = 15; }
                            226 => { __state = 16; }
                            227 => { __state = 17; }
                            239 => { __state = 18; }
                            _ => { __state = 19; }
                        }
                    }
                }
                7 => { __state = 8; }
                8 => { __state = 9; }
                9 => { __state = 10; }
                10 => { __state = 11; }
                11 => { __state = 12; }
                12 => {
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                    __state = 22;
                }
                13 => {
                    if unsafe { *z.offset((n + 1) as isize) } as i32 ==
                                '*' as i32 &&
                            unsafe { *z.offset((n + 2) as isize) } as i32 != 0 {
                        __state = 26;
                    } else { __state = 27; }
                }
                14 => {
                    if unsafe { *z.offset((n + 1) as isize) } as i32 == 160 {
                        __state = 51;
                    } else { __state = 50; }
                }
                15 => {
                    if unsafe { *z.offset((n + 1) as isize) } as i32 == 154 &&
                            unsafe { *z.offset((n + 2) as isize) } as i32 == 128 {
                        __state = 55;
                    } else { __state = 54; }
                }
                16 => {
                    if unsafe { *z.offset((n + 1) as isize) } as i32 == 128 {
                        __state = 59;
                    } else { __state = 60; }
                }
                17 => {
                    if unsafe { *z.offset((n + 1) as isize) } as i32 == 128 &&
                            unsafe { *z.offset((n + 2) as isize) } as i32 == 128 {
                        __state = 70;
                    } else { __state = 69; }
                }
                18 => {
                    if unsafe { *z.offset((n + 1) as isize) } as i32 == 187 &&
                            unsafe { *z.offset((n + 2) as isize) } as i32 == 191 {
                        __state = 74;
                    } else { __state = 73; }
                }
                19 => { __state = 2; }
                20 => { __state = 7; }
                21 => { __state = 23; }
                22 => { __state = 4; }
                23 => { __state = 13; }
                24 => { __state = 14; }
                25 => { __state = 2; }
                26 => { __state = 28; }
                27 => {
                    if unsafe { *z.offset((n + 1) as isize) } as i32 ==
                            '/' as i32 {
                        __state = 35;
                    } else { __state = 25; }
                }
                28 => { j = n + 3; __state = 30; }
                29 => { n = j + 1; __state = 34; }
                30 => {
                    if unsafe { *z.offset(j as isize) } as i32 != '/' as i32 ||
                            unsafe { *z.offset((j - 1) as isize) } as i32 != '*' as i32
                        {
                        __state = 31;
                    } else { __state = 29; }
                }
                31 => {
                    if unsafe { *z.offset(j as isize) } as i32 == 0 {
                        __state = 33;
                    } else { __state = 32; }
                }
                32 => {
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    __state = 30;
                }
                33 => { __state = 2; }
                34 => { __state = 4; }
                35 => { __state = 36; }
                36 => { __state = 37; }
                37 => { j__1 = n + 2; __state = 39; }
                38 => { n = j__1; __state = 46; }
                39 => {
                    if { c = unsafe { *z.offset(j__1 as isize) } as i8; c } as
                                i32 != 0 {
                        __state = 40;
                    } else { __state = 38; }
                }
                40 => {
                    if c as i32 == '\n' as i32 || c as i32 == '\r' as i32 {
                        __state = 43;
                    } else { __state = 42; }
                }
                41 => {
                    { let __p = &mut j__1; let __t = *__p; *__p += 1; __t };
                    __state = 39;
                }
                42 => {
                    if 226 == c as u8 as i32 &&
                                128 ==
                                    unsafe { *z.offset((j__1 + 1) as isize) } as u8 as i32 &&
                            (168 ==
                                    unsafe { *z.offset((j__1 + 2) as isize) } as u8 as i32 ||
                                169 ==
                                    unsafe { *z.offset((j__1 + 2) as isize) } as u8 as i32) {
                        __state = 44;
                    } else { __state = 41; }
                }
                43 => { __state = 38; }
                44 => { j__1 += 2; __state = 45; }
                45 => { __state = 38; }
                46 => {
                    if unsafe { *z.offset(n as isize) } != 0 {
                        __state = 48;
                    } else { __state = 47; }
                }
                47 => { __state = 4; }
                48 => {
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                    __state = 47;
                }
                49 => { __state = 15; }
                50 => { __state = 2; }
                51 => { n += 2; __state = 52; }
                52 => { __state = 4; }
                53 => { __state = 16; }
                54 => { __state = 2; }
                55 => { n += 3; __state = 56; }
                56 => { __state = 4; }
                57 => { __state = 17; }
                58 => { __state = 2; }
                59 => {
                    c__1 = unsafe { *z.offset((n + 2) as isize) } as u8;
                    __state = 61;
                }
                60 => {
                    if unsafe { *z.offset((n + 1) as isize) } as i32 == 129 &&
                            unsafe { *z.offset((n + 2) as isize) } as i32 == 159 {
                        __state = 66;
                    } else { __state = 58; }
                }
                61 => {
                    if (c__1 as i32) < 128 {
                        __state = 63;
                    } else { __state = 62; }
                }
                62 => {
                    if c__1 as i32 <= 138 || c__1 as i32 == 168 ||
                                c__1 as i32 == 169 || c__1 as i32 == 175 {
                        __state = 64;
                    } else { __state = 58; }
                }
                63 => { __state = 2; }
                64 => { n += 3; __state = 65; }
                65 => { __state = 4; }
                66 => { n += 3; __state = 67; }
                67 => { __state = 4; }
                68 => { __state = 18; }
                69 => { __state = 2; }
                70 => { n += 3; __state = 71; }
                71 => { __state = 4; }
                72 => { __state = 19; }
                73 => { __state = 2; }
                74 => { n += 3; __state = 75; }
                75 => { __state = 4; }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn json_is4_hex_b(z: *const i8, p_op_1: &mut i32) -> i32 {
    if unsafe { *z.offset(0 as isize) } as i32 != 'u' as i32 { return 0; }
    if (json_is4_hex(unsafe { &*z.offset(1 as isize) }) == 0) as i32 != 0 {
        return 0;
    }
    *p_op_1 = 8;
    return 1;
}
static json_is_space: [i8; 256] =
    [0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 1 as i8, 1 as i8, 0 as i8, 0 as i8, 1 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 1 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8];
static json_spaces: [i8; 5] =
    [9 as i8, 10 as i8, 13 as i8, 32 as i8, 0 as i8];
extern "C" fn json_blob_change_payload_size(p_parse_1: *mut JsonParse, i: u32,
    sz_payload_1: u32) -> i32 {
    let mut a: *mut u8 = core::ptr::null_mut();
    let mut sz_type: u8 = 0 as u8;
    let mut n_extra: u8 = 0 as u8;
    let mut n_needed: u8 = 0 as u8;
    let mut delta: i32 = 0;
    if unsafe { (*p_parse_1).oom } != 0 { return 0; }
    a = unsafe { unsafe { (*p_parse_1).a_blob.add(i as usize) } };
    sz_type = (unsafe { *a.offset(0 as isize) } as i32 >> 4) as u8;
    if sz_type as i32 <= 11 {
        n_extra = 0 as u8;
    } else if sz_type as i32 == 12 {
        n_extra = 1 as u8;
    } else if sz_type as i32 == 13 {
        n_extra = 2 as u8;
    } else if sz_type as i32 == 14 {
        n_extra = 4 as u8;
    } else { n_extra = 8 as u8; }
    if sz_payload_1 <= 11 as u32 {
        n_needed = 0 as u8;
    } else if sz_payload_1 <= 255 as u32 {
        n_needed = 1 as u8;
    } else if sz_payload_1 <= 65535 as u32 {
        n_needed = 2 as u8;
    } else { n_needed = 4 as u8; }
    delta = n_needed as i32 - n_extra as i32;
    if delta != 0 {
        let new_size: u32 = unsafe { (*p_parse_1).n_blob } + delta as u32;
        if delta > 0 {
            if new_size > unsafe { (*p_parse_1).n_blob_alloc } &&
                    json_blob_expand(unsafe { &mut *p_parse_1 }, new_size) != 0
                {
                return 0;
            }
            a = unsafe { unsafe { (*p_parse_1).a_blob.add(i as usize) } };
            unsafe {
                memmove(unsafe { &raw mut *a.offset((1 + delta) as isize) } as
                        *mut (),
                    unsafe { &raw mut *a.offset(1 as isize) } as *const (),
                    (unsafe { (*p_parse_1).n_blob } - (i + 1 as u32)) as u64)
            };
        } else {
            unsafe {
                memmove(unsafe { &raw mut *a.offset(1 as isize) } as *mut (),
                    unsafe { &raw mut *a.offset((1 - delta) as isize) } as
                        *const (),
                    (unsafe { (*p_parse_1).n_blob } -
                            (i + 1 as u32 - delta as u32)) as u64)
            };
        }
        unsafe { (*p_parse_1).n_blob = new_size };
    }
    if n_needed as i32 == 0 {
        unsafe {
            *a.offset(0 as isize) =
                ((unsafe { *a.offset(0 as isize) } as i32 & 15) as u32 |
                        sz_payload_1 << 4) as u8
        };
    } else if n_needed as i32 == 1 {
        unsafe {
            *a.offset(0 as isize) =
                (unsafe { *a.offset(0 as isize) } as i32 & 15 | 192) as u8
        };
        unsafe { *a.offset(1 as isize) = (sz_payload_1 & 255 as u32) as u8 };
    } else if n_needed as i32 == 2 {
        unsafe {
            *a.offset(0 as isize) =
                (unsafe { *a.offset(0 as isize) } as i32 & 15 | 208) as u8
        };
        unsafe {
            *a.offset(1 as isize) = (sz_payload_1 >> 8 & 255 as u32) as u8
        };
        unsafe { *a.offset(2 as isize) = (sz_payload_1 & 255 as u32) as u8 };
    } else {
        unsafe {
            *a.offset(0 as isize) =
                (unsafe { *a.offset(0 as isize) } as i32 & 15 | 224) as u8
        };
        unsafe {
            *a.offset(1 as isize) = (sz_payload_1 >> 24 & 255 as u32) as u8
        };
        unsafe {
            *a.offset(2 as isize) = (sz_payload_1 >> 16 & 255 as u32) as u8
        };
        unsafe {
            *a.offset(3 as isize) = (sz_payload_1 >> 8 & 255 as u32) as u8
        };
        unsafe { *a.offset(4 as isize) = (sz_payload_1 & 255 as u32) as u8 };
    }
    return delta;
}
extern "C" fn json_blob_expand_and_append_one_byte(p_parse_1: *mut JsonParse,
    c: u8) -> () {
    json_blob_expand(unsafe { &mut *p_parse_1 },
        unsafe { (*p_parse_1).n_blob } + 1 as u32);
    if unsafe { (*p_parse_1).oom } as i32 == 0 {
        { let _ = 0; };
        unsafe {
            *unsafe {
                        (*p_parse_1).a_blob.add({
                                    let __p = unsafe { &mut (*p_parse_1).n_blob };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize)
                    } = c
        };
    }
}
extern "C" fn json_blob_append_one_byte(p_parse_1: *mut JsonParse, c: u8)
    -> () {
    if unsafe { (*p_parse_1).n_blob } >= unsafe { (*p_parse_1).n_blob_alloc }
        {
        json_blob_expand_and_append_one_byte(p_parse_1, c);
    } else {
        unsafe {
            *unsafe {
                        (*p_parse_1).a_blob.add({
                                    let __p = unsafe { &mut (*p_parse_1).n_blob };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize)
                    } = c
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct NanInfName {
    c1: i8,
    c2: i8,
    n: i8,
    e_type: i8,
    n_repl: i8,
    z_match: *mut i8,
    z_repl: *mut i8,
}
static mut a_nan_inf_name: [NanInfName; 5] =
    [NanInfName {
                c1: 'i' as i32 as i8,
                c2: 'I' as i32 as i8,
                n: 3 as i8,
                e_type: 5 as i8,
                n_repl: 7 as i8,
                z_match: c"inf".as_ptr() as *mut i8,
                z_repl: c"9.0e999".as_ptr() as *mut i8,
            },
            NanInfName {
                c1: 'i' as i32 as i8,
                c2: 'I' as i32 as i8,
                n: 8 as i8,
                e_type: 5 as i8,
                n_repl: 7 as i8,
                z_match: c"infinity".as_ptr() as *mut i8,
                z_repl: c"9.0e999".as_ptr() as *mut i8,
            },
            NanInfName {
                c1: 'n' as i32 as i8,
                c2: 'N' as i32 as i8,
                n: 3 as i8,
                e_type: 0 as i8,
                n_repl: 4 as i8,
                z_match: c"NaN".as_ptr() as *mut i8,
                z_repl: c"null".as_ptr() as *mut i8,
            },
            NanInfName {
                c1: 'q' as i32 as i8,
                c2: 'Q' as i32 as i8,
                n: 4 as i8,
                e_type: 0 as i8,
                n_repl: 4 as i8,
                z_match: c"QNaN".as_ptr() as *mut i8,
                z_repl: c"null".as_ptr() as *mut i8,
            },
            NanInfName {
                c1: 's' as i32 as i8,
                c2: 'S' as i32 as i8,
                n: 4 as i8,
                e_type: 0 as i8,
                n_repl: 4 as i8,
                z_match: c"SNaN".as_ptr() as *mut i8,
                z_repl: c"null".as_ptr() as *mut i8,
            }];
extern "C" fn json_translate_text_to_blob(p_parse_1: *mut JsonParse,
    mut i: u32) -> i32 {
    unsafe {
        unsafe {
            let mut c: i8 = 0 as i8;
            let mut j: u32 = 0 as u32;
            let mut i_this: u32 = 0 as u32;
            let mut i_start: u32 = 0 as u32;
            let mut x: i32 = 0;
            let mut t: u8 = 0 as u8;
            let mut z: *const i8 = core::ptr::null();
            let mut i_blob: u32 = 0 as u32;
            let mut op: i32 = 0;
            let mut k: i32 = 0;
            let mut __state__1: i32 = 0;
            let mut opcode: u8 = 0 as u8;
            let mut c_delim: i8 = 0 as i8;
            let mut seen_e: u8 = 0 as u8;
            let mut k__1: u32 = 0 as u32;
            let mut nn: i32 = 0;
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s16:
                    {
                    match __state {
                        0 => { __state = 6; }
                        2 => {
                            '__s17:
                                {
                                match unsafe { *z.add(i as usize) } as u8 {
                                    123 => { __state = 12; }
                                    91 => { __state = 13; }
                                    39 => { __state = 14; }
                                    34 => { __state = 15; }
                                    116 => { __state = 16; }
                                    102 => { __state = 17; }
                                    43 => { __state = 18; }
                                    46 => { __state = 19; }
                                    45 => { __state = 20; }
                                    48 => { __state = 21; }
                                    49 => { __state = 22; }
                                    50 => { __state = 23; }
                                    51 => { __state = 24; }
                                    52 => { __state = 25; }
                                    53 => { __state = 26; }
                                    54 => { __state = 27; }
                                    55 => { __state = 28; }
                                    56 => { __state = 29; }
                                    57 => { __state = 30; }
                                    125 => { __state = 31; }
                                    93 => { __state = 32; }
                                    44 => { __state = 33; }
                                    58 => { __state = 34; }
                                    0 => { __state = 35; }
                                    9 => { __state = 36; }
                                    10 => { __state = 37; }
                                    13 => { __state = 38; }
                                    32 => { __state = 39; }
                                    11 => { __state = 40; }
                                    12 => { __state = 41; }
                                    47 => { __state = 42; }
                                    194 => { __state = 43; }
                                    225 => { __state = 44; }
                                    226 => { __state = 45; }
                                    227 => { __state = 46; }
                                    239 => { __state = 47; }
                                    110 => { __state = 48; }
                                    _ => { __state = 49; }
                                }
                            }
                        }
                        3 => { seen_e = 0 as u8; __state = 494; }
                        4 => { j = i + 1 as u32; __state = 539; }
                        5 => { { let _ = 0; }; __state = 577; }
                        6 => { __state = 7; }
                        7 => { __state = 8; }
                        8 => { __state = 9; }
                        9 => { __state = 10; }
                        10 => {
                            z = unsafe { (*p_parse_1).z_json } as *const i8;
                            __state = 11;
                        }
                        11 => { __state = 2; }
                        12 => { __state = 52; }
                        13 => {
                            i_this = unsafe { (*p_parse_1).n_blob };
                            __state = 381;
                        }
                        14 => { __state = 426; }
                        15 => { opcode = 7 as u8; __state = 429; }
                        16 => {
                            if unsafe {
                                            strncmp(unsafe { z.add(i as usize) },
                                                c"true".as_ptr() as *mut i8 as *const i8, 4 as u64)
                                        } == 0 &&
                                    (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.add((i + 4 as u32) as usize) } as
                                                                            u8 as usize)
                                                        } as i32 & 6 == 0) as i32 != 0 {
                                __state = 473;
                            } else { __state = 472; }
                        }
                        17 => {
                            if unsafe {
                                            strncmp(unsafe { z.add(i as usize) },
                                                c"false".as_ptr() as *mut i8 as *const i8, 5 as u64)
                                        } == 0 &&
                                    (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.add((i + 5 as u32) as usize) } as
                                                                            u8 as usize)
                                                        } as i32 & 6 == 0) as i32 != 0 {
                                __state = 478;
                            } else { __state = 477; }
                        }
                        18 => { __state = 482; }
                        19 => {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add((i + 1 as u32) as usize) } as
                                                                u8 as usize)
                                            } as i32 & 4 != 0 {
                                __state = 487;
                            } else { __state = 486; }
                        }
                        20 => { __state = 21; }
                        21 => { __state = 22; }
                        22 => { __state = 23; }
                        23 => { __state = 24; }
                        24 => { __state = 25; }
                        25 => { __state = 26; }
                        26 => { __state = 27; }
                        27 => { __state = 28; }
                        28 => { __state = 29; }
                        29 => { __state = 30; }
                        30 => { t = 0 as u8; __state = 493; }
                        31 => { unsafe { (*p_parse_1).i_err = i }; __state = 584; }
                        32 => { unsafe { (*p_parse_1).i_err = i }; __state = 586; }
                        33 => { unsafe { (*p_parse_1).i_err = i }; __state = 588; }
                        34 => { unsafe { (*p_parse_1).i_err = i }; __state = 590; }
                        35 => { return 0; }
                        36 => { __state = 37; }
                        37 => { __state = 38; }
                        38 => { __state = 39; }
                        39 => {
                            i +=
                                1 as u32 +
                                    unsafe {
                                            strspn(unsafe { &*z.add((i + 1 as u32) as usize) },
                                                &raw const json_spaces[0 as usize] as *const i8)
                                        } as u32;
                            __state = 593;
                        }
                        40 => { __state = 41; }
                        41 => { __state = 42; }
                        42 => { __state = 43; }
                        43 => { __state = 44; }
                        44 => { __state = 45; }
                        45 => { __state = 46; }
                        46 => { __state = 47; }
                        47 => {
                            j = json5_whitespace(unsafe { &*z.add(i as usize) }) as u32;
                            __state = 596;
                        }
                        48 => {
                            if unsafe {
                                            strncmp(unsafe { z.add(i as usize) },
                                                c"null".as_ptr() as *mut i8 as *const i8, 4 as u64)
                                        } == 0 &&
                                    (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.add((i + 4 as u32) as usize) } as
                                                                            u8 as usize)
                                                        } as i32 & 6 == 0) as i32 != 0 {
                                __state = 605;
                            } else { __state = 604; }
                        }
                        49 => { __state = 608; }
                        50 => { __state = 12; }
                        51 => { __state = 13; }
                        52 => { __state = 53; }
                        53 => { __state = 54; }
                        54 => { __state__1 = 0; __state = 55; }
                        55 => { if true { __state = 56; } else { __state = 51; } }
                        56 => {
                            if __state__1 == 1 { __state = 58; } else { __state = 57; }
                        }
                        57 => {
                            '__s18:
                                {
                                match __state__1 {
                                    0 => { __state = 59; }
                                    2 => { __state = 60; }
                                    3 => { __state = 61; }
                                    4 => { __state = 62; }
                                    5 => { __state = 63; }
                                    6 => { __state = 64; }
                                    7 => { __state = 65; }
                                    8 => { __state = 66; }
                                    9 => { __state = 67; }
                                    10 => { __state = 68; }
                                    11 => { __state = 69; }
                                    12 => { __state = 70; }
                                    13 => { __state = 71; }
                                    14 => { __state = 72; }
                                    15 => { __state = 73; }
                                    16 => { __state = 74; }
                                    17 => { __state = 75; }
                                    18 => { __state = 76; }
                                    19 => { __state = 77; }
                                    20 => { __state = 78; }
                                    21 => { __state = 79; }
                                    22 => { __state = 80; }
                                    23 => { __state = 81; }
                                    24 => { __state = 82; }
                                    25 => { __state = 83; }
                                    26 => { __state = 84; }
                                    27 => { __state = 85; }
                                    28 => { __state = 86; }
                                    29 => { __state = 87; }
                                    30 => { __state = 88; }
                                    31 => { __state = 89; }
                                    32 => { __state = 90; }
                                    33 => { __state = 91; }
                                    34 => { __state = 92; }
                                    35 => { __state = 93; }
                                    36 => { __state = 94; }
                                    37 => { __state = 95; }
                                    38 => { __state = 96; }
                                    39 => { __state = 97; }
                                    40 => { __state = 98; }
                                    41 => { __state = 99; }
                                    42 => { __state = 100; }
                                    43 => { __state = 101; }
                                    44 => { __state = 102; }
                                    45 => { __state = 103; }
                                    46 => { __state = 104; }
                                    47 => { __state = 105; }
                                    48 => { __state = 106; }
                                    49 => { __state = 107; }
                                    50 => { __state = 108; }
                                    51 => { __state = 109; }
                                    52 => { __state = 110; }
                                    53 => { __state = 111; }
                                    54 => { __state = 112; }
                                    55 => { __state = 113; }
                                    56 => { __state = 114; }
                                    57 => { __state = 115; }
                                    58 => { __state = 116; }
                                    59 => { __state = 117; }
                                    60 => { __state = 118; }
                                    61 => { __state = 119; }
                                    62 => { __state = 120; }
                                    63 => { __state = 121; }
                                    64 => { __state = 122; }
                                    65 => { __state = 123; }
                                    66 => { __state = 124; }
                                    67 => { __state = 125; }
                                    68 => { __state = 126; }
                                    69 => { __state = 127; }
                                    70 => { __state = 128; }
                                    71 => { __state = 129; }
                                    72 => { __state = 130; }
                                    73 => { __state = 131; }
                                    74 => { __state = 132; }
                                    75 => { __state = 133; }
                                    76 => { __state = 134; }
                                    77 => { __state = 135; }
                                    78 => { __state = 136; }
                                    79 => { __state = 137; }
                                    80 => { __state = 138; }
                                    81 => { __state = 139; }
                                    _ => { __state = 55; }
                                }
                            }
                        }
                        58 => { __state = 51; }
                        59 => {
                            i_this = unsafe { (*p_parse_1).n_blob };
                            __state = 141;
                        }
                        60 => {
                            x = json_translate_text_to_blob(p_parse_1, j);
                            __state = 144;
                        }
                        61 => {
                            json_blob_append_node(p_parse_1, 12 as u8,
                                (unsafe { (*p_parse_1).n_json } as u32 - i) as u64,
                                core::ptr::null());
                            __state = 147;
                        }
                        62 => {
                            if {
                                            let __p = unsafe { &mut (*p_parse_1).i_depth };
                                            *__p += 1;
                                            *__p
                                        } as i32 > 1000 {
                                __state = 151;
                            } else { __state = 152; }
                        }
                        63 => {
                            i_start = unsafe { (*p_parse_1).n_blob };
                            __state = 154;
                        }
                        64 => { unsafe { (*p_parse_1).i_err = i }; __state = 157; }
                        65 => { return -1; }
                        66 => { j = i + 1 as u32; __state = 161; }
                        67 => {
                            json_blob_change_payload_size(p_parse_1, i_this,
                                unsafe { (*p_parse_1).n_blob } - i_start);
                            __state = 164;
                        }
                        68 => { __state__1 = 11; __state = 167; }
                        69 => {
                            i_blob = unsafe { (*p_parse_1).n_blob };
                            __state = 169;
                        }
                        70 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 172;
                        }
                        71 => {
                            x = json_translate_text_to_blob(p_parse_1, j);
                            __state = 175;
                        }
                        72 => {
                            if x <= 0 { __state = 179; } else { __state = 180; }
                        }
                        73 => {
                            if unsafe { (*p_parse_1).oom } != 0 {
                                __state = 183;
                            } else { __state = 184; }
                        }
                        74 => { __state__1 = 17; __state = 186; }
                        75 => {
                            if x == -2 { __state = 189; } else { __state = 190; }
                        }
                        76 => {
                            j +=
                                json5_whitespace(unsafe { &*z.add(j as usize) }) as u32;
                            __state = 192;
                        }
                        77 => { j = unsafe { (*p_parse_1).i_err }; __state = 195; }
                        78 => {
                            if unsafe { (*p_parse_1).n_blob } != i_start as u32 {
                                __state = 199;
                            } else { __state = 200; }
                        }
                        79 => { __state__1 = 9; __state = 202; }
                        80 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 204;
                        }
                        81 => { op = 7; __state = 207; }
                        82 => {
                            if unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                                } as i32 & 66 != 0 ||
                                    unsafe { *z.add(j as usize) } as i32 == '\\' as i32 &&
                                        json_is4_hex_b(unsafe { &*z.add((j + 1 as u32) as usize) },
                                                &mut op) != 0 {
                                __state = 211;
                            } else { __state = 212; }
                        }
                        83 => { k = (j + 1 as u32) as i32; __state = 214; }
                        84 => {
                            if x != -1 { __state = 218; } else { __state = 219; }
                        }
                        85 => {
                            if unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.offset(k as isize) } as u8 as
                                                                    usize)
                                                    } as i32 & 70 != 0 &&
                                        json5_whitespace(unsafe { &*z.offset(k as isize) }) == 0 ||
                                    unsafe { *z.offset(k as isize) } as i32 == '\\' as i32 &&
                                        json_is4_hex_b(unsafe { &*z.offset((k + 1) as isize) },
                                                &mut op) != 0 {
                                __state = 222;
                            } else { __state = 223; }
                        }
                        86 => { { let _ = 0; }; __state = 225; }
                        87 => {
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                            __state = 228;
                        }
                        88 => {
                            json_blob_append_node(p_parse_1, op as u8,
                                (k as u32 - j) as u64,
                                unsafe { &raw const *z.add(j as usize) } as *const ());
                            __state = 231;
                        }
                        89 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 234;
                        }
                        90 => { x = k; __state = 237; }
                        91 => { return -1; }
                        92 => { unsafe { (*p_parse_1).i_err = j }; __state = 241; }
                        93 => {
                            t =
                                (unsafe {
                                                *unsafe { (*p_parse_1).a_blob.add(i_blob as usize) }
                                            } as i32 & 15) as u8;
                            __state = 244;
                        }
                        94 => { return -1; }
                        95 => {
                            if (t as i32) < 7 || t as i32 > 10 {
                                __state = 249;
                            } else { __state = 250; }
                        }
                        96 => { j = x as u32; __state = 252; }
                        97 => { unsafe { (*p_parse_1).i_err = j }; __state = 255; }
                        98 => { return -1; }
                        99 => {
                            if unsafe { *z.add(j as usize) } as i32 == ':' as i32 {
                                __state = 260;
                            } else { __state = 261; }
                        }
                        100 => { __state__1 = 2; __state = 263; }
                        101 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 265;
                        }
                        102 => {
                            if json_is_space[unsafe { *z.add(j as usize) } as u8 as
                                            usize] != 0 {
                                __state = 269;
                            } else { __state = 270; }
                        }
                        103 => {
                            x = json_translate_text_to_blob(p_parse_1, j);
                            __state = 272;
                        }
                        104 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 275;
                        }
                        105 => {
                            if unsafe { *z.add(j as usize) } as i32 == ':' as i32 {
                                __state = 279;
                            } else { __state = 280; }
                        }
                        106 => {
                            if json_is_space[unsafe { *z.add(j as usize) } as u8 as
                                            usize] != 0 {
                                __state = 283;
                            } else { __state = 284; }
                        }
                        107 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 286;
                        }
                        108 => { __state__1 = 2; __state = 289; }
                        109 => {
                            if x != -5 { __state = 292; } else { __state = 293; }
                        }
                        110 => {
                            j = unsafe { (*p_parse_1).i_err } + 1 as u32;
                            __state = 295;
                        }
                        111 => {
                            if x != -1 { __state = 299; } else { __state = 300; }
                        }
                        112 => { return -1; }
                        113 => { unsafe { (*p_parse_1).i_err = j }; __state = 303; }
                        114 => {
                            if x <= 0 { __state = 307; } else { __state = 308; }
                        }
                        115 => { j = x as u32; __state = 310; }
                        116 => {
                            if x != -1 { __state = 314; } else { __state = 315; }
                        }
                        117 => { return -1; }
                        118 => { unsafe { (*p_parse_1).i_err = j }; __state = 318; }
                        119 => {
                            if unsafe { *z.add(j as usize) } as i32 == ',' as i32 {
                                __state = 322;
                            } else { __state = 323; }
                        }
                        120 => { unsafe { (*p_parse_1).i_err = j }; __state = 325; }
                        121 => { __state__1 = 12; __state = 328; }
                        122 => {
                            if unsafe { *z.add(j as usize) } as i32 == '}' as i32 {
                                __state = 331;
                            } else { __state = 332; }
                        }
                        123 => { __state__1 = 9; __state = 334; }
                        124 => {
                            if json_is_space[unsafe { *z.add(j as usize) } as u8 as
                                            usize] != 0 {
                                __state = 337;
                            } else { __state = 338; }
                        }
                        125 => {
                            x = json_translate_text_to_blob(p_parse_1, j);
                            __state = 340;
                        }
                        126 => {
                            j +=
                                1 as u32 +
                                    unsafe {
                                            strspn(unsafe { &*z.add((j + 1 as u32) as usize) },
                                                &raw const json_spaces[0 as usize] as *const i8)
                                        } as u32;
                            __state = 343;
                        }
                        127 => {
                            if unsafe { *z.add(j as usize) } as i32 == ',' as i32 {
                                __state = 347;
                            } else { __state = 348; }
                        }
                        128 => { __state__1 = 12; __state = 350; }
                        129 => {
                            if unsafe { *z.add(j as usize) } as i32 == '}' as i32 {
                                __state = 353;
                            } else { __state = 354; }
                        }
                        130 => { __state__1 = 9; __state = 356; }
                        131 => {
                            if x == -4 { __state = 359; } else { __state = 360; }
                        }
                        132 => {
                            if x == -2 { __state = 363; } else { __state = 364; }
                        }
                        133 => { j = unsafe { (*p_parse_1).i_err }; __state = 366; }
                        134 => { __state__1 = 12; __state = 369; }
                        135 => { j = unsafe { (*p_parse_1).i_err }; __state = 371; }
                        136 => { __state__1 = 9; __state = 374; }
                        137 => { return -1; }
                        138 => {
                            {
                                let __p = unsafe { &mut (*p_parse_1).i_depth };
                                let __t = *__p;
                                *__p -= 1;
                                __t
                            };
                            __state = 377;
                        }
                        139 => { return (j + 1 as u32) as i32; }
                        140 => { __state = 59; }
                        141 => { __state__1 = 3; __state = 142; }
                        142 => { __state = 55; }
                        143 => { __state = 60; }
                        144 => { __state__1 = 56; __state = 145; }
                        145 => { __state = 55; }
                        146 => { __state = 61; }
                        147 => { __state__1 = 4; __state = 148; }
                        148 => { __state = 55; }
                        149 => { __state = 62; }
                        150 => { __state = 55; }
                        151 => { __state__1 = 6; __state = 150; }
                        152 => { __state__1 = 5; __state = 150; }
                        153 => { __state = 63; }
                        154 => { __state__1 = 8; __state = 155; }
                        155 => { __state = 55; }
                        156 => { __state = 64; }
                        157 => { __state__1 = 7; __state = 158; }
                        158 => { __state = 55; }
                        159 => { __state = 65; }
                        160 => { __state = 66; }
                        161 => { __state__1 = 10; __state = 162; }
                        162 => { __state = 55; }
                        163 => { __state = 67; }
                        164 => { __state__1 = 80; __state = 165; }
                        165 => { __state = 55; }
                        166 => { __state = 68; }
                        167 => { __state = 55; }
                        168 => { __state = 69; }
                        169 => { __state__1 = 13; __state = 170; }
                        170 => { __state = 55; }
                        171 => { __state = 70; }
                        172 => { __state__1 = 10; __state = 173; }
                        173 => { __state = 55; }
                        174 => { __state = 71; }
                        175 => { __state__1 = 14; __state = 176; }
                        176 => { __state = 55; }
                        177 => { __state = 72; }
                        178 => { __state = 55; }
                        179 => { __state__1 = 16; __state = 178; }
                        180 => { __state__1 = 15; __state = 178; }
                        181 => { __state = 73; }
                        182 => { __state = 55; }
                        183 => { __state__1 = 36; __state = 182; }
                        184 => { __state__1 = 35; __state = 182; }
                        185 => { __state = 74; }
                        186 => { __state = 55; }
                        187 => { __state = 75; }
                        188 => { __state = 55; }
                        189 => { __state__1 = 19; __state = 188; }
                        190 => { __state__1 = 18; __state = 188; }
                        191 => { __state = 76; }
                        192 => { __state__1 = 23; __state = 193; }
                        193 => { __state = 55; }
                        194 => { __state = 77; }
                        195 => { __state__1 = 20; __state = 196; }
                        196 => { __state = 55; }
                        197 => { __state = 78; }
                        198 => { __state = 55; }
                        199 => { __state__1 = 22; __state = 198; }
                        200 => { __state__1 = 21; __state = 198; }
                        201 => { __state = 79; }
                        202 => { __state = 55; }
                        203 => { __state = 80; }
                        204 => { __state__1 = 21; __state = 205; }
                        205 => { __state = 55; }
                        206 => { __state = 81; }
                        207 => { __state__1 = 24; __state = 208; }
                        208 => { __state = 55; }
                        209 => { __state = 82; }
                        210 => { __state = 55; }
                        211 => { __state__1 = 25; __state = 210; }
                        212 => { __state__1 = 26; __state = 210; }
                        213 => { __state = 83; }
                        214 => { __state__1 = 27; __state = 215; }
                        215 => { __state = 55; }
                        216 => { __state = 84; }
                        217 => { __state = 55; }
                        218 => { __state__1 = 34; __state = 217; }
                        219 => { __state__1 = 33; __state = 217; }
                        220 => { __state = 85; }
                        221 => { __state = 55; }
                        222 => { __state__1 = 29; __state = 221; }
                        223 => { __state__1 = 28; __state = 221; }
                        224 => { __state = 86; }
                        225 => { __state__1 = 30; __state = 226; }
                        226 => { __state = 55; }
                        227 => { __state = 87; }
                        228 => { __state__1 = 27; __state = 229; }
                        229 => { __state = 55; }
                        230 => { __state = 88; }
                        231 => { __state__1 = 31; __state = 232; }
                        232 => { __state = 55; }
                        233 => { __state = 89; }
                        234 => { __state__1 = 32; __state = 235; }
                        235 => { __state = 55; }
                        236 => { __state = 90; }
                        237 => { __state__1 = 15; __state = 238; }
                        238 => { __state = 55; }
                        239 => { __state = 91; }
                        240 => { __state = 92; }
                        241 => { __state__1 = 33; __state = 242; }
                        242 => { __state = 55; }
                        243 => { __state = 93; }
                        244 => { __state__1 = 37; __state = 245; }
                        245 => { __state = 55; }
                        246 => { __state = 94; }
                        247 => { __state = 95; }
                        248 => { __state = 55; }
                        249 => { __state__1 = 39; __state = 248; }
                        250 => { __state__1 = 38; __state = 248; }
                        251 => { __state = 96; }
                        252 => { __state__1 = 41; __state = 253; }
                        253 => { __state = 55; }
                        254 => { __state = 97; }
                        255 => { __state__1 = 40; __state = 256; }
                        256 => { __state = 55; }
                        257 => { __state = 98; }
                        258 => { __state = 99; }
                        259 => { __state = 55; }
                        260 => { __state__1 = 43; __state = 259; }
                        261 => { __state__1 = 44; __state = 259; }
                        262 => { __state = 100; }
                        263 => { __state = 55; }
                        264 => { __state = 101; }
                        265 => { __state__1 = 42; __state = 266; }
                        266 => { __state = 55; }
                        267 => { __state = 102; }
                        268 => { __state = 55; }
                        269 => { __state__1 = 46; __state = 268; }
                        270 => { __state__1 = 45; __state = 268; }
                        271 => { __state = 103; }
                        272 => { __state__1 = 51; __state = 273; }
                        273 => { __state = 55; }
                        274 => { __state = 104; }
                        275 => { __state__1 = 48; __state = 276; }
                        276 => { __state = 55; }
                        277 => { __state = 105; }
                        278 => { __state = 55; }
                        279 => { __state__1 = 49; __state = 278; }
                        280 => { __state__1 = 45; __state = 278; }
                        281 => { __state = 106; }
                        282 => { __state = 55; }
                        283 => { __state__1 = 46; __state = 282; }
                        284 => { __state__1 = 47; __state = 282; }
                        285 => { __state = 107; }
                        286 => { __state__1 = 50; __state = 287; }
                        287 => { __state = 55; }
                        288 => { __state = 108; }
                        289 => { __state = 55; }
                        290 => { __state = 109; }
                        291 => { __state = 55; }
                        292 => { __state__1 = 53; __state = 291; }
                        293 => { __state__1 = 52; __state = 291; }
                        294 => { __state = 110; }
                        295 => { __state__1 = 42; __state = 296; }
                        296 => { __state = 55; }
                        297 => { __state = 111; }
                        298 => { __state = 55; }
                        299 => { __state__1 = 55; __state = 298; }
                        300 => { __state__1 = 54; __state = 298; }
                        301 => { __state = 112; }
                        302 => { __state = 113; }
                        303 => { __state__1 = 54; __state = 304; }
                        304 => { __state = 55; }
                        305 => { __state = 114; }
                        306 => { __state = 55; }
                        307 => { __state__1 = 58; __state = 306; }
                        308 => { __state__1 = 57; __state = 306; }
                        309 => { __state = 115; }
                        310 => { __state__1 = 61; __state = 311; }
                        311 => { __state = 55; }
                        312 => { __state = 116; }
                        313 => { __state = 55; }
                        314 => { __state__1 = 60; __state = 313; }
                        315 => { __state__1 = 59; __state = 313; }
                        316 => { __state = 117; }
                        317 => { __state = 118; }
                        318 => { __state__1 = 59; __state = 319; }
                        319 => { __state = 55; }
                        320 => { __state = 119; }
                        321 => { __state = 55; }
                        322 => { __state__1 = 63; __state = 321; }
                        323 => { __state__1 = 64; __state = 321; }
                        324 => { __state = 120; }
                        325 => { __state__1 = 79; __state = 326; }
                        326 => { __state = 55; }
                        327 => { __state = 121; }
                        328 => { __state = 55; }
                        329 => { __state = 122; }
                        330 => { __state = 55; }
                        331 => { __state__1 = 65; __state = 330; }
                        332 => { __state__1 = 66; __state = 330; }
                        333 => { __state = 123; }
                        334 => { __state = 55; }
                        335 => { __state = 124; }
                        336 => { __state = 55; }
                        337 => { __state__1 = 68; __state = 336; }
                        338 => { __state__1 = 67; __state = 336; }
                        339 => { __state = 125; }
                        340 => { __state__1 = 73; __state = 341; }
                        341 => { __state = 55; }
                        342 => { __state = 126; }
                        343 => { __state__1 = 69; __state = 344; }
                        344 => { __state = 55; }
                        345 => { __state = 127; }
                        346 => { __state = 55; }
                        347 => { __state__1 = 70; __state = 346; }
                        348 => { __state__1 = 71; __state = 346; }
                        349 => { __state = 128; }
                        350 => { __state = 55; }
                        351 => { __state = 129; }
                        352 => { __state = 55; }
                        353 => { __state__1 = 72; __state = 352; }
                        354 => { __state__1 = 67; __state = 352; }
                        355 => { __state = 130; }
                        356 => { __state = 55; }
                        357 => { __state = 131; }
                        358 => { __state = 55; }
                        359 => { __state__1 = 75; __state = 358; }
                        360 => { __state__1 = 74; __state = 358; }
                        361 => { __state = 132; }
                        362 => { __state = 55; }
                        363 => { __state__1 = 77; __state = 362; }
                        364 => { __state__1 = 62; __state = 362; }
                        365 => { __state = 133; }
                        366 => { __state__1 = 76; __state = 367; }
                        367 => { __state = 55; }
                        368 => { __state = 134; }
                        369 => { __state = 55; }
                        370 => { __state = 135; }
                        371 => { __state__1 = 78; __state = 372; }
                        372 => { __state = 55; }
                        373 => { __state = 136; }
                        374 => { __state = 55; }
                        375 => { __state = 137; }
                        376 => { __state = 138; }
                        377 => { __state__1 = 81; __state = 378; }
                        378 => { __state = 55; }
                        379 => { __state = 139; }
                        380 => { __state = 14; }
                        381 => { { let _ = 0; }; __state = 382; }
                        382 => {
                            json_blob_append_node(p_parse_1, 11 as u8,
                                (unsafe { (*p_parse_1).n_json } as u32 - i) as u64,
                                core::ptr::null());
                            __state = 383;
                        }
                        383 => {
                            i_start = unsafe { (*p_parse_1).n_blob };
                            __state = 384;
                        }
                        384 => {
                            if unsafe { (*p_parse_1).oom } != 0 {
                                __state = 386;
                            } else { __state = 385; }
                        }
                        385 => {
                            if {
                                            let __p = unsafe { &mut (*p_parse_1).i_depth };
                                            *__p += 1;
                                            *__p
                                        } as i32 > 1000 {
                                __state = 388;
                            } else { __state = 387; }
                        }
                        386 => { return -1; }
                        387 => { j = i + 1 as u32; __state = 391; }
                        388 => { unsafe { (*p_parse_1).i_err = i }; __state = 389; }
                        389 => { return -1; }
                        390 => {
                            json_blob_change_payload_size(p_parse_1, i_this,
                                unsafe { (*p_parse_1).n_blob } - i_start);
                            __state = 423;
                        }
                        391 => { __state = 392; }
                        392 => {
                            x = json_translate_text_to_blob(p_parse_1, j);
                            __state = 394;
                        }
                        393 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 391;
                        }
                        394 => {
                            if x <= 0 { __state = 396; } else { __state = 395; }
                        }
                        395 => { j = x as u32; __state = 404; }
                        396 => {
                            if x == -3 { __state = 398; } else { __state = 397; }
                        }
                        397 => {
                            if x != -1 { __state = 403; } else { __state = 402; }
                        }
                        398 => { j = unsafe { (*p_parse_1).i_err }; __state = 399; }
                        399 => {
                            if unsafe { (*p_parse_1).n_blob } != i_start {
                                __state = 401;
                            } else { __state = 400; }
                        }
                        400 => { __state = 390; }
                        401 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 400;
                        }
                        402 => { return -1; }
                        403 => { unsafe { (*p_parse_1).i_err = j }; __state = 402; }
                        404 => {
                            if unsafe { *z.add(j as usize) } as i32 == ',' as i32 {
                                __state = 406;
                            } else { __state = 407; }
                        }
                        405 => { unsafe { (*p_parse_1).i_err = j }; __state = 422; }
                        406 => { __state = 393; }
                        407 => {
                            if unsafe { *z.add(j as usize) } as i32 == ']' as i32 {
                                __state = 408;
                            } else { __state = 409; }
                        }
                        408 => { __state = 390; }
                        409 => {
                            if json_is_space[unsafe { *z.add(j as usize) } as u8 as
                                            usize] != 0 {
                                __state = 411;
                            } else { __state = 410; }
                        }
                        410 => {
                            x = json_translate_text_to_blob(p_parse_1, j);
                            __state = 416;
                        }
                        411 => {
                            j +=
                                1 as u32 +
                                    unsafe {
                                            strspn(unsafe { &*z.add((j + 1 as u32) as usize) },
                                                &raw const json_spaces[0 as usize] as *const i8)
                                        } as u32;
                            __state = 412;
                        }
                        412 => {
                            if unsafe { *z.add(j as usize) } as i32 == ',' as i32 {
                                __state = 413;
                            } else { __state = 414; }
                        }
                        413 => { __state = 393; }
                        414 => {
                            if unsafe { *z.add(j as usize) } as i32 == ']' as i32 {
                                __state = 415;
                            } else { __state = 410; }
                        }
                        415 => { __state = 390; }
                        416 => {
                            if x == -4 { __state = 418; } else { __state = 417; }
                        }
                        417 => {
                            if x == -3 { __state = 420; } else { __state = 405; }
                        }
                        418 => { j = unsafe { (*p_parse_1).i_err }; __state = 419; }
                        419 => { __state = 393; }
                        420 => { j = unsafe { (*p_parse_1).i_err }; __state = 421; }
                        421 => { __state = 390; }
                        422 => { return -1; }
                        423 => {
                            {
                                let __p = unsafe { &mut (*p_parse_1).i_depth };
                                let __t = *__p;
                                *__p -= 1;
                                __t
                            };
                            __state = 424;
                        }
                        424 => { return (j + 1 as u32) as i32; }
                        425 => { __state = 16; }
                        426 => { __state = 427; }
                        427 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 430;
                        }
                        428 => {
                            c_delim = unsafe { *z.add(i as usize) } as i8;
                            __state = 433;
                        }
                        429 => {
                            if false { __state = 427; } else { __state = 428; }
                        }
                        430 => { opcode = 7 as u8; __state = 431; }
                        431 => { __state = 428; }
                        432 => { __state = 15; }
                        433 => { j = i + 1 as u32; __state = 434; }
                        434 => {
                            if 1 != 0 { __state = 436; } else { __state = 435; }
                        }
                        435 => {
                            json_blob_append_node(p_parse_1, opcode,
                                (j - 1 as u32 - i) as u64,
                                unsafe { &raw const *z.add((i + 1 as u32) as usize) } as
                                    *const ());
                            __state = 470;
                        }
                        436 => {
                            if json_is_ok[unsafe { *z.add(j as usize) } as u8 as usize]
                                    != 0 {
                                __state = 438;
                            } else { __state = 437; }
                        }
                        437 => {
                            c = unsafe { *z.add(j as usize) } as i8;
                            __state = 444;
                        }
                        438 => {
                            if (json_is_ok[unsafe { *z.add((j + 1 as u32) as usize) } as
                                                        u8 as usize] == 0) as i32 != 0 {
                                __state = 439;
                            } else { __state = 440; }
                        }
                        439 => { j += 1 as u32; __state = 437; }
                        440 => {
                            if (json_is_ok[unsafe { *z.add((j + 2 as u32) as usize) } as
                                                        u8 as usize] == 0) as i32 != 0 {
                                __state = 441;
                            } else { __state = 442; }
                        }
                        441 => { j += 2 as u32; __state = 437; }
                        442 => { j += 3 as u32; __state = 443; }
                        443 => { __state = 434; }
                        444 => {
                            if c as i32 == c_delim as i32 {
                                __state = 446;
                            } else { __state = 447; }
                        }
                        445 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 434;
                        }
                        446 => { __state = 435; }
                        447 => {
                            if c as i32 == '\\' as i32 {
                                __state = 448;
                            } else { __state = 449; }
                        }
                        448 => {
                            c =
                                unsafe {
                                        *z.add({ let __p = &mut j; *__p += 1; *__p } as usize)
                                    } as i8;
                            __state = 450;
                        }
                        449 => {
                            if c as i32 <= 31 { __state = 463; } else { __state = 464; }
                        }
                        450 => {
                            if c as i32 == '\"' as i32 || c as i32 == '\\' as i32 ||
                                                            c as i32 == '/' as i32 || c as i32 == 'b' as i32 ||
                                                    c as i32 == 'f' as i32 || c as i32 == 'n' as i32 ||
                                            c as i32 == 'r' as i32 || c as i32 == 't' as i32 ||
                                    c as i32 == 'u' as i32 &&
                                        json_is4_hex(unsafe { &*z.add((j + 1 as u32) as usize) }) !=
                                            0 {
                                __state = 451;
                            } else { __state = 452; }
                        }
                        451 => {
                            if opcode as i32 == 7 {
                                __state = 453;
                            } else { __state = 445; }
                        }
                        452 => {
                            if c as i32 == '\'' as i32 || c as i32 == 'v' as i32 ||
                                                c as i32 == '\n' as i32 ||
                                            c as i32 == '0' as i32 &&
                                                (unsafe {
                                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                                    *const u8).add(unsafe { *z.add((j + 1 as u32) as usize) } as
                                                                                        u8 as usize)
                                                                    } as i32 & 4 == 0) as i32 != 0 ||
                                        226 == c as u8 as i32 &&
                                                128 ==
                                                    unsafe { *z.add((j + 1 as u32) as usize) } as u8 as i32 &&
                                            (168 ==
                                                    unsafe { *z.add((j + 2 as u32) as usize) } as u8 as i32 ||
                                                169 ==
                                                    unsafe { *z.add((j + 2 as u32) as usize) } as u8 as i32) ||
                                    c as i32 == 'x' as i32 &&
                                        json_is2_hex(unsafe { &*z.add((j + 1 as u32) as usize) }) !=
                                            0 {
                                __state = 454;
                            } else { __state = 455; }
                        }
                        453 => { opcode = 8 as u8; __state = 445; }
                        454 => { opcode = 9 as u8; __state = 456; }
                        455 => {
                            if c as i32 == '\r' as i32 {
                                __state = 457;
                            } else { __state = 458; }
                        }
                        456 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 445;
                        }
                        457 => {
                            if unsafe { *z.add((j + 1 as u32) as usize) } as i32 ==
                                    '\n' as i32 {
                                __state = 460;
                            } else { __state = 459; }
                        }
                        458 => { unsafe { (*p_parse_1).i_err = j }; __state = 462; }
                        459 => { opcode = 9 as u8; __state = 461; }
                        460 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 459;
                        }
                        461 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 445;
                        }
                        462 => { return -1; }
                        463 => {
                            if c as i32 == 0 { __state = 466; } else { __state = 465; }
                        }
                        464 => {
                            if c as i32 == '\"' as i32 {
                                __state = 469;
                            } else { __state = 445; }
                        }
                        465 => { opcode = 9 as u8; __state = 468; }
                        466 => { unsafe { (*p_parse_1).i_err = j }; __state = 467; }
                        467 => { return -1; }
                        468 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 445;
                        }
                        469 => { opcode = 9 as u8; __state = 445; }
                        470 => { return (j + 1 as u32) as i32; }
                        471 => { __state = 17; }
                        472 => { unsafe { (*p_parse_1).i_err = i }; __state = 475; }
                        473 => {
                            json_blob_append_one_byte(p_parse_1, 1 as u8);
                            __state = 474;
                        }
                        474 => { return (i + 4 as u32) as i32; }
                        475 => { return -1; }
                        476 => { __state = 18; }
                        477 => { unsafe { (*p_parse_1).i_err = i }; __state = 480; }
                        478 => {
                            json_blob_append_one_byte(p_parse_1, 2 as u8);
                            __state = 479;
                        }
                        479 => { return (i + 5 as u32) as i32; }
                        480 => { return -1; }
                        481 => { __state = 31; }
                        482 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 483;
                        }
                        483 => { t = 0 as u8; __state = 484; }
                        484 => { __state = 3; }
                        485 => { __state = 19; }
                        486 => { unsafe { (*p_parse_1).i_err = i }; __state = 491; }
                        487 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 488;
                        }
                        488 => { t = 3 as u8; __state = 489; }
                        489 => { seen_e = 0 as u8; __state = 490; }
                        490 => { __state = 4; }
                        491 => { return -1; }
                        492 => { __state = 20; }
                        493 => { __state = 3; }
                        494 => { { let _ = 0; }; __state = 495; }
                        495 => { { let _ = 0; }; __state = 496; }
                        496 => { { let _ = 0; }; __state = 497; }
                        497 => {
                            c = unsafe { *z.add(i as usize) } as i8;
                            __state = 498;
                        }
                        498 => {
                            if c as i32 <= '0' as i32 {
                                __state = 500;
                            } else { __state = 499; }
                        }
                        499 => { __state = 4; }
                        500 => {
                            if c as i32 == '0' as i32 {
                                __state = 501;
                            } else { __state = 502; }
                        }
                        501 => {
                            if (unsafe { *z.add((i + 1 as u32) as usize) } as i32 ==
                                            'x' as i32 ||
                                        unsafe { *z.add((i + 1 as u32) as usize) } as i32 ==
                                            'X' as i32) &&
                                    unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z.add((i + 2 as u32) as usize) } as
                                                                    u8 as usize)
                                                } as i32 & 8 != 0 {
                                __state = 503;
                            } else { __state = 504; }
                        }
                        502 => {
                            if (unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z.add((i + 1 as u32) as usize) } as
                                                                        u8 as usize)
                                                    } as i32 & 4 == 0) as i32 != 0 {
                                __state = 515;
                            } else { __state = 514; }
                        }
                        503 => { { let _ = 0; }; __state = 505; }
                        504 => {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add((i + 1 as u32) as usize) } as
                                                                u8 as usize)
                                            } as i32 & 4 != 0 {
                                __state = 512;
                            } else { __state = 499; }
                        }
                        505 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 506;
                        }
                        506 => { t = 1 as u8; __state = 507; }
                        507 => { j = i + 3 as u32; __state = 509; }
                        508 => { __state = 5; }
                        509 => {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 8 != 0 {
                                __state = 510;
                            } else { __state = 508; }
                        }
                        510 => { __state = 511; }
                        511 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 509;
                        }
                        512 => {
                            unsafe { (*p_parse_1).i_err = i + 1 as u32 };
                            __state = 513;
                        }
                        513 => { return -1; }
                        514 => {
                            if unsafe { *z.add((i + 1 as u32) as usize) } as i32 ==
                                    '0' as i32 {
                                __state = 527;
                            } else { __state = 499; }
                        }
                        515 => {
                            if (unsafe { *z.add((i + 1 as u32) as usize) } as i32 ==
                                            'I' as i32 ||
                                        unsafe { *z.add((i + 1 as u32) as usize) } as i32 ==
                                            'i' as i32) &&
                                    unsafe {
                                            sqlite3_strnicmp(unsafe {
                                                    &*z.add((i + 1 as u32) as usize)
                                                }, c"inf".as_ptr() as *mut i8 as *const i8, 3)
                                        } == 0 {
                                __state = 517;
                            } else { __state = 516; }
                        }
                        516 => {
                            if unsafe { *z.add((i + 1 as u32) as usize) } as i32 ==
                                    '.' as i32 {
                                __state = 523;
                            } else { __state = 522; }
                        }
                        517 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 518;
                        }
                        518 => {
                            if unsafe { *z.add(i as usize) } as i32 == '-' as i32 {
                                __state = 520;
                            } else { __state = 521; }
                        }
                        519 => {
                            return (i +
                                        if unsafe {
                                                        sqlite3_strnicmp(unsafe {
                                                                &*z.add((i + 4 as u32) as usize)
                                                            }, c"inity".as_ptr() as *mut i8 as *const i8, 5)
                                                    } == 0 {
                                                9
                                            } else { 4 } as u32) as i32;
                        }
                        520 => {
                            json_blob_append_node(p_parse_1, 5 as u8, 6 as u64,
                                c"-9e999".as_ptr() as *mut i8 as *const ());
                            __state = 519;
                        }
                        521 => {
                            json_blob_append_node(p_parse_1, 5 as u8, 5 as u64,
                                c"9e999".as_ptr() as *mut i8 as *const ());
                            __state = 519;
                        }
                        522 => { unsafe { (*p_parse_1).i_err = i }; __state = 526; }
                        523 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 524;
                        }
                        524 => { t |= 1 as u8; __state = 525; }
                        525 => { __state = 4; }
                        526 => { return -1; }
                        527 => {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add((i + 2 as u32) as usize) } as
                                                                u8 as usize)
                                            } as i32 & 4 != 0 {
                                __state = 528;
                            } else { __state = 529; }
                        }
                        528 => {
                            unsafe { (*p_parse_1).i_err = i + 1 as u32 };
                            __state = 530;
                        }
                        529 => {
                            if (unsafe { *z.add((i + 2 as u32) as usize) } as i32 ==
                                            'x' as i32 ||
                                        unsafe { *z.add((i + 2 as u32) as usize) } as i32 ==
                                            'X' as i32) &&
                                    unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z.add((i + 3 as u32) as usize) } as
                                                                    u8 as usize)
                                                } as i32 & 8 != 0 {
                                __state = 531;
                            } else { __state = 499; }
                        }
                        530 => { return -1; }
                        531 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 532;
                        }
                        532 => { t |= 1 as u8; __state = 533; }
                        533 => { j = i + 4 as u32; __state = 535; }
                        534 => { __state = 5; }
                        535 => {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add(j as usize) } as u8 as usize)
                                            } as i32 & 8 != 0 {
                                __state = 536;
                            } else { __state = 534; }
                        }
                        536 => { __state = 537; }
                        537 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 535;
                        }
                        538 => {
                            if (unsafe { *z.add((j - 1 as u32) as usize) } as i32) <
                                    '0' as i32 {
                                __state = 572;
                            } else { __state = 571; }
                        }
                        539 => { __state = 540; }
                        540 => {
                            c = unsafe { *z.add(j as usize) } as i8;
                            __state = 542;
                        }
                        541 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 539;
                        }
                        542 => {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(c as u8 as usize)
                                            } as i32 & 4 != 0 {
                                __state = 544;
                            } else { __state = 543; }
                        }
                        543 => {
                            if c as i32 == '.' as i32 {
                                __state = 546;
                            } else { __state = 545; }
                        }
                        544 => { __state = 541; }
                        545 => {
                            if c as i32 == 'e' as i32 || c as i32 == 'E' as i32 {
                                __state = 552;
                            } else { __state = 551; }
                        }
                        546 => {
                            if t as i32 & 2 != 0 {
                                __state = 548;
                            } else { __state = 547; }
                        }
                        547 => { t |= 2 as u8; __state = 550; }
                        548 => { unsafe { (*p_parse_1).i_err = j }; __state = 549; }
                        549 => { return -1; }
                        550 => { __state = 541; }
                        551 => { __state = 538; }
                        552 => {
                            if (unsafe { *z.add((j - 1 as u32) as usize) } as i32) <
                                    '0' as i32 {
                                __state = 554;
                            } else { __state = 553; }
                        }
                        553 => {
                            if seen_e != 0 { __state = 560; } else { __state = 559; }
                        }
                        554 => {
                            if unsafe { *z.add((j - 1 as u32) as usize) } as i32 ==
                                            '.' as i32 && j - 2 as u32 >= i &&
                                    unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z.add((j - 2 as u32) as usize) } as
                                                                    u8 as usize)
                                                } as i32 & 4 != 0 {
                                __state = 555;
                            } else { __state = 556; }
                        }
                        555 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 557;
                        }
                        556 => { unsafe { (*p_parse_1).i_err = j }; __state = 558; }
                        557 => { t |= 1 as u8; __state = 553; }
                        558 => { return -1; }
                        559 => { t |= 2 as u8; __state = 562; }
                        560 => { unsafe { (*p_parse_1).i_err = j }; __state = 561; }
                        561 => { return -1; }
                        562 => { seen_e = 1 as u8; __state = 563; }
                        563 => {
                            c = unsafe { *z.add((j + 1 as u32) as usize) } as i8;
                            __state = 564;
                        }
                        564 => {
                            if c as i32 == '+' as i32 || c as i32 == '-' as i32 {
                                __state = 566;
                            } else { __state = 565; }
                        }
                        565 => {
                            if (c as i32) < '0' as i32 || c as i32 > '9' as i32 {
                                __state = 569;
                            } else { __state = 568; }
                        }
                        566 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 567;
                        }
                        567 => {
                            c = unsafe { *z.add((j + 1 as u32) as usize) } as i8;
                            __state = 565;
                        }
                        568 => { __state = 541; }
                        569 => { unsafe { (*p_parse_1).i_err = j }; __state = 570; }
                        570 => { return -1; }
                        571 => { __state = 5; }
                        572 => {
                            if unsafe { *z.add((j - 1 as u32) as usize) } as i32 ==
                                            '.' as i32 && j - 2 as u32 >= i &&
                                    unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z.add((j - 2 as u32) as usize) } as
                                                                    u8 as usize)
                                                } as i32 & 4 != 0 {
                                __state = 573;
                            } else { __state = 574; }
                        }
                        573 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 575;
                        }
                        574 => { unsafe { (*p_parse_1).i_err = j }; __state = 576; }
                        575 => { t |= 1 as u8; __state = 571; }
                        576 => { return -1; }
                        577 => { { let _ = 0; }; __state = 578; }
                        578 => { { let _ = 0; }; __state = 579; }
                        579 => {
                            if unsafe { *z.add(i as usize) } as i32 == '+' as i32 {
                                __state = 581;
                            } else { __state = 580; }
                        }
                        580 => {
                            json_blob_append_node(p_parse_1, (3 + t as i32) as u8,
                                (j - i) as u64,
                                unsafe { &raw const *z.add(i as usize) } as *const ());
                            __state = 582;
                        }
                        581 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 580;
                        }
                        582 => { return j as i32; }
                        583 => { __state = 32; }
                        584 => { return -2; }
                        585 => { __state = 33; }
                        586 => { return -3; }
                        587 => { __state = 34; }
                        588 => { return -4; }
                        589 => { __state = 35; }
                        590 => { return -5; }
                        591 => { __state = 36; }
                        592 => { __state = 594; }
                        593 => { __state = 2; }
                        594 => { __state = 40; }
                        595 => { __state = 602; }
                        596 => {
                            if j > 0 as u32 { __state = 598; } else { __state = 597; }
                        }
                        597 => { unsafe { (*p_parse_1).i_err = i }; __state = 601; }
                        598 => { i += j; __state = 599; }
                        599 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 600;
                        }
                        600 => { __state = 2; }
                        601 => { return -1; }
                        602 => { __state = 48; }
                        603 => { __state = 607; }
                        604 => { __state = 603; }
                        605 => {
                            json_blob_append_one_byte(p_parse_1, 0 as u8);
                            __state = 606;
                        }
                        606 => { return (i + 4 as u32) as i32; }
                        607 => { __state = 49; }
                        608 => { __state = 609; }
                        609 => {
                            c = unsafe { *z.add(i as usize) } as i8;
                            __state = 610;
                        }
                        610 => { k__1 = 0 as u32; __state = 612; }
                        611 => { unsafe { (*p_parse_1).i_err = i }; __state = 626; }
                        612 => {
                            if (k__1 as u64) <
                                    core::mem::size_of::<[NanInfName; 5]>() as u64 /
                                        core::mem::size_of::<NanInfName>() as u64 {
                                __state = 613;
                            } else { __state = 611; }
                        }
                        613 => {
                            if c as i32 != a_nan_inf_name[k__1 as usize].c1 as i32 &&
                                    c as i32 != a_nan_inf_name[k__1 as usize].c2 as i32 {
                                __state = 616;
                            } else { __state = 615; }
                        }
                        614 => {
                            { let __p = &mut k__1; let __t = *__p; *__p += 1; __t };
                            __state = 612;
                        }
                        615 => {
                            nn = a_nan_inf_name[k__1 as usize].n as i32;
                            __state = 617;
                        }
                        616 => { __state = 614; }
                        617 => {
                            if unsafe {
                                        sqlite3_strnicmp(unsafe { &*z.add(i as usize) },
                                            a_nan_inf_name[k__1 as usize].z_match as *const i8, nn)
                                    } != 0 {
                                __state = 619;
                            } else { __state = 618; }
                        }
                        618 => {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.add((i + nn as u32) as usize) }
                                                                as u8 as usize)
                                            } as i32 & 6 != 0 {
                                __state = 621;
                            } else { __state = 620; }
                        }
                        619 => { __state = 614; }
                        620 => {
                            if a_nan_inf_name[k__1 as usize].e_type as i32 == 5 {
                                __state = 623;
                            } else { __state = 624; }
                        }
                        621 => { __state = 614; }
                        622 => {
                            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
                            __state = 625;
                        }
                        623 => {
                            json_blob_append_node(p_parse_1, 5 as u8, 5 as u64,
                                c"9e999".as_ptr() as *mut i8 as *const ());
                            __state = 622;
                        }
                        624 => {
                            json_blob_append_one_byte(p_parse_1, 0 as u8);
                            __state = 622;
                        }
                        625 => { return (i + nn as u32) as i32; }
                        626 => { return -1; }
                        _ => {}
                    }
                }
            }
            unreachable!();
        }
    }
}
extern "C" fn json_convert_text_to_blob(p_parse_1: *mut JsonParse,
    p_ctx_1: *mut sqlite3_context) -> i32 {
    let mut i: i32 = 0;
    let z_json: *const i8 = unsafe { (*p_parse_1).z_json } as *const i8;
    i = json_translate_text_to_blob(p_parse_1, 0 as u32);
    if unsafe { (*p_parse_1).oom } != 0 { i = -1; }
    if i > 0 {
        while json_is_space[unsafe { *z_json.offset(i as isize) } as u8 as
                        usize] != 0 {
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
        if unsafe { *z_json.offset(i as isize) } != 0 {
            i += json5_whitespace(unsafe { &*z_json.offset(i as isize) });
            if unsafe { *z_json.offset(i as isize) } != 0 {
                if !(p_ctx_1).is_null() {
                    unsafe {
                        sqlite3_result_error(p_ctx_1,
                            c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
                    };
                }
                json_parse_reset(unsafe { &mut *p_parse_1 });
                return 1;
            }
            unsafe { (*p_parse_1).has_nonstd = 1 as u8 };
        }
    }
    if i <= 0 {
        if p_ctx_1 != core::ptr::null_mut() {
            if unsafe { (*p_parse_1).oom } != 0 {
                unsafe { sqlite3_result_error_nomem(p_ctx_1) };
            } else {
                unsafe {
                    sqlite3_result_error(p_ctx_1,
                        c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
                };
            }
        }
        json_parse_reset(unsafe { &mut *p_parse_1 });
        return 1;
    }
    return 0;
}
extern "C" fn json_cache_delete(p: *mut JsonCache) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b20: loop {
            if !(i < unsafe { (*p).n_used }) { break '__b20; }
            '__c20: loop {
                unsafe { json_parse_free(unsafe { (*p).a[i as usize] }) };
                break '__c20;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_db_free(unsafe { (*p).db }, p as *mut ()) };
}
extern "C" fn json_cache_delete_generic(p: *mut ()) -> () {
    json_cache_delete(p as *mut JsonCache);
}
extern "C" fn json_cache_insert(ctx: *mut sqlite3_context,
    p_parse_1: *mut JsonParse) -> i32 {
    let mut p: *mut JsonCache = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    p = unsafe { sqlite3_get_auxdata(ctx, -429938) } as *mut JsonCache;
    if p == core::ptr::null_mut() {
        let db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(ctx) };
        p =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<JsonCache>() as u64)
                } as *mut JsonCache;
        if p == core::ptr::null_mut() { return 7; }
        unsafe { (*p).db = db };
        unsafe {
            sqlite3_set_auxdata(ctx, -429938, p as *mut (),
                Some(json_cache_delete_generic))
        };
        p = unsafe { sqlite3_get_auxdata(ctx, -429938) } as *mut JsonCache;
        if p == core::ptr::null_mut() { return 7; }
    }
    if unsafe { (*p).n_used } >= 4 {
        unsafe { json_parse_free(unsafe { (*p).a[0 as usize] }) };
        unsafe {
            memmove(unsafe { &raw mut (*p).a[0 as usize] } as
                        *mut *mut JsonParse as *mut (),
                unsafe { &raw mut (*p).a[1 as usize] } as *const (),
                (4 - 1) as u64 *
                    core::mem::size_of::<*mut JsonParse>() as u64)
        };
        unsafe { (*p).n_used = 4 - 1 };
    }
    { let _ = 0; };
    unsafe { (*p_parse_1).e_edit = 0 as u8 };
    {
        let __p = unsafe { &mut (*p_parse_1).n_jp_ref };
        let __t = *__p;
        *__p += 1;
        __t
    };
    unsafe { (*p_parse_1).b_read_only = 1 as u8 };
    unsafe { (*p).a[unsafe { (*p).n_used } as usize] = p_parse_1 };
    { let __p = unsafe { &mut (*p).n_used }; let __t = *__p; *__p += 1; __t };
    return 0;
}
extern "C" fn json_parse_func_arg(ctx: *mut sqlite3_context,
    p_arg_1: *mut sqlite3_value, flgs: u32) -> *mut JsonParse {
    let mut e_type: i32 = 0;
    let mut p: *mut JsonParse = core::ptr::null_mut();
    let mut p_from_cache: *mut JsonParse = core::ptr::null_mut();
    let mut db: *mut sqlite3 = core::ptr::null_mut();
    let mut n_blob: u32 = 0 as u32;
    let mut is_rc_str: i32 = 0;
    let mut rc: i32 = 0;
    let mut z_new: *mut i8 = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s22:
            {
            match __state {
                0 => { __state = 5; }
                2 => {
                    p =
                        unsafe {
                                sqlite3_db_malloc_zero(db,
                                    core::mem::size_of::<JsonParse>() as u64)
                            } as *mut JsonParse;
                    __state = 19;
                }
                3 => {
                    if flgs & 2 as u32 != 0 {
                        __state = 73;
                    } else { __state = 74; }
                }
                4 => { json_parse_free(p_from_cache); __state = 78; }
                5 => { p = core::ptr::null_mut(); __state = 6; }
                6 => { p_from_cache = core::ptr::null_mut(); __state = 7; }
                7 => { __state = 8; }
                8 => { { let _ = 0; }; __state = 9; }
                9 => {
                    e_type = unsafe { sqlite3_value_type(p_arg_1) };
                    __state = 10;
                }
                10 => {
                    if e_type == 5 { __state = 12; } else { __state = 11; }
                }
                11 => {
                    p_from_cache = json_cache_search(ctx, p_arg_1);
                    __state = 13;
                }
                12 => { return core::ptr::null_mut(); }
                13 => {
                    if !(p_from_cache).is_null() {
                        __state = 15;
                    } else { __state = 14; }
                }
                14 => {
                    db = unsafe { sqlite3_context_db_handle(ctx) };
                    __state = 18;
                }
                15 => {
                    {
                        let __p = unsafe { &mut (*p_from_cache).n_jp_ref };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 16;
                }
                16 => {
                    if flgs & 1 as u32 == 0 as u32 {
                        __state = 17;
                    } else { __state = 14; }
                }
                17 => { return p_from_cache; }
                18 => { __state = 2; }
                19 => {
                    if p == core::ptr::null_mut() {
                        __state = 21;
                    } else { __state = 20; }
                }
                20 => {
                    unsafe {
                        memset(p as *mut (), 0,
                            core::mem::size_of::<JsonParse>() as u64)
                    };
                    __state = 22;
                }
                21 => { __state = 4; }
                22 => { unsafe { (*p).db = db }; __state = 23; }
                23 => { unsafe { (*p).n_jp_ref = 1 as u32 }; __state = 24; }
                24 => {
                    if p_from_cache != core::ptr::null_mut() {
                        __state = 26;
                    } else { __state = 25; }
                }
                25 => {
                    if e_type == 4 { __state = 36; } else { __state = 35; }
                }
                26 => {
                    n_blob = unsafe { (*p_from_cache).n_blob };
                    __state = 27;
                }
                27 => {
                    unsafe {
                        (*p).a_blob =
                            unsafe { sqlite3_db_malloc_raw(db, n_blob as u64) } as
                                *mut u8
                    };
                    __state = 28;
                }
                28 => {
                    if unsafe { (*p).a_blob } == core::ptr::null_mut() {
                        __state = 30;
                    } else { __state = 29; }
                }
                29 => {
                    unsafe {
                        memcpy(unsafe { (*p).a_blob } as *mut (),
                            unsafe { (*p_from_cache).a_blob } as *const (),
                            n_blob as u64)
                    };
                    __state = 31;
                }
                30 => { __state = 4; }
                31 => {
                    unsafe {
                        (*p).n_blob_alloc =
                            { unsafe { (*p).n_blob = n_blob }; unsafe { (*p).n_blob } }
                    };
                    __state = 32;
                }
                32 => {
                    unsafe {
                        (*p).has_nonstd = unsafe { (*p_from_cache).has_nonstd }
                    };
                    __state = 33;
                }
                33 => { json_parse_free(p_from_cache); __state = 34; }
                34 => { return p; }
                35 => {
                    unsafe {
                        (*p).z_json =
                            unsafe { sqlite3_value_text(p_arg_1) } as *mut i8
                    };
                    __state = 40;
                }
                36 => {
                    if json_arg_is_jsonb(p_arg_1, p) != 0 {
                        __state = 37;
                    } else { __state = 35; }
                }
                37 => {
                    if flgs & 1 as u32 != 0 as u32 &&
                            json_blob_make_editable(p, 0 as u32) == 0 {
                        __state = 39;
                    } else { __state = 38; }
                }
                38 => { return p; }
                39 => { __state = 4; }
                40 => {
                    unsafe {
                        (*p).n_json = unsafe { sqlite3_value_bytes(p_arg_1) }
                    };
                    __state = 41;
                }
                41 => {
                    if unsafe { (*db).malloc_failed } != 0 {
                        __state = 43;
                    } else { __state = 42; }
                }
                42 => {
                    if unsafe { (*p).n_json } == 0 {
                        __state = 45;
                    } else { __state = 44; }
                }
                43 => { __state = 4; }
                44 => { { let _ = 0; }; __state = 46; }
                45 => { __state = 3; }
                46 => {
                    if json_convert_text_to_blob(p,
                                if flgs & 2 as u32 != 0 {
                                    core::ptr::null_mut()
                                } else { ctx }) != 0 {
                        __state = 48;
                    } else { __state = 49; }
                }
                47 => { return p; }
                48 => {
                    if flgs & 2 as u32 != 0 {
                        __state = 50;
                    } else { __state = 51; }
                }
                49 => {
                    is_rc_str =
                        unsafe {
                            sqlite3_value_is_of_class(p_arg_1 as *const sqlite3_value,
                                Some(sqlite3_rc_str_unref))
                        };
                    __state = 54;
                }
                50 => { unsafe { (*p).n_err = 1 as u8 }; __state = 52; }
                51 => { json_parse_free(p); __state = 53; }
                52 => { return p; }
                53 => { return core::ptr::null_mut(); }
                54 => { __state = 55; }
                55 => {
                    if (is_rc_str == 0) as i32 != 0 {
                        __state = 57;
                    } else { __state = 58; }
                }
                56 => {
                    unsafe { (*p).b_json_is_rc_str = 1 as u8 };
                    __state = 64;
                }
                57 => {
                    z_new =
                        unsafe {
                            sqlite3_rc_str_new(unsafe { (*p).n_json } as u64)
                        };
                    __state = 59;
                }
                58 => {
                    unsafe { sqlite3_rc_str_ref(unsafe { (*p).z_json }) };
                    __state = 56;
                }
                59 => {
                    if z_new == core::ptr::null_mut() {
                        __state = 61;
                    } else { __state = 60; }
                }
                60 => {
                    unsafe {
                        memcpy(z_new as *mut (),
                            unsafe { (*p).z_json } as *const (),
                            unsafe { (*p).n_json } as u64)
                    };
                    __state = 62;
                }
                61 => { __state = 4; }
                62 => { unsafe { (*p).z_json = z_new }; __state = 63; }
                63 => {
                    unsafe {
                        *unsafe {
                                    (*p).z_json.offset(unsafe { (*p).n_json } as isize)
                                } = 0 as i8
                    };
                    __state = 56;
                }
                64 => { rc = json_cache_insert(ctx, p); __state = 65; }
                65 => { if rc == 7 { __state = 67; } else { __state = 66; } }
                66 => {
                    if flgs & 1 as u32 != 0 {
                        __state = 68;
                    } else { __state = 47; }
                }
                67 => { __state = 4; }
                68 => { p_from_cache = p; __state = 69; }
                69 => { p = core::ptr::null_mut(); __state = 70; }
                70 => { __state = 2; }
                71 => { __state = 3; }
                72 => { __state = 4; }
                73 => { unsafe { (*p).n_err = 1 as u8 }; __state = 75; }
                74 => { json_parse_free(p); __state = 76; }
                75 => { return p; }
                76 => {
                    unsafe {
                        sqlite3_result_error(ctx,
                            c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
                    };
                    __state = 77;
                }
                77 => { return core::ptr::null_mut(); }
                78 => { json_parse_free(p); __state = 79; }
                79 => {
                    unsafe { sqlite3_result_error_nomem(ctx) };
                    __state = 80;
                }
                80 => { return core::ptr::null_mut(); }
                _ => {}
            }
        }
    }
    unreachable!();
}
extern "C" fn json_bad_path_error(ctx: *mut sqlite3_context,
    z_path_1: *const i8, rc: i32) -> *mut i8 {
    let mut z_msg: *mut i8 = core::ptr::null_mut();
    if rc == 4294967293u32 as i32 {
        z_msg =
            unsafe {
                sqlite3_mprintf(c"not an array element: %Q".as_ptr() as
                            *mut i8 as *const i8, z_path_1)
            };
    } else if rc == 4294967295u32 as i32 {
        z_msg =
            unsafe {
                sqlite3_mprintf(c"malformed JSON".as_ptr() as *mut i8 as
                        *const i8)
            };
    } else if rc == 4294967292u32 as i32 {
        z_msg =
            unsafe {
                sqlite3_mprintf(c"JSON path too deep".as_ptr() as *mut i8 as
                        *const i8)
            };
    } else {
        z_msg =
            unsafe {
                sqlite3_mprintf(c"bad JSON path: %Q".as_ptr() as *mut i8 as
                        *const i8, z_path_1)
            };
    }
    if ctx == core::ptr::null_mut() { return z_msg; }
    if !(z_msg).is_null() {
        unsafe { sqlite3_result_error(ctx, z_msg as *const i8, -1) };
        unsafe { sqlite3_free(z_msg as *mut ()) };
    } else { unsafe { sqlite3_result_error_nomem(ctx) }; }
    return core::ptr::null_mut();
}
extern "C" fn json_blob_overwrite(a_out_1: *mut u8, a_ins_1: *const u8,
    n_ins_1: u32, d: u32) -> i32 {
    let mut sz_payload: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    let mut sz_hdr: u8 = 0 as u8;
    if unsafe { *a_ins_1.offset(0 as isize) } as i32 & 15 <= 2 { return 0; }
    '__s23:
        {
        match unsafe { *a_ins_1.offset(0 as isize) } as i32 >> 4 {
            12 => {
                {
                    if 1 << d & 138 == 0 { return 0; }
                    i = d + 2 as u32;
                    sz_hdr = 2 as u8;
                    break '__s23;
                }
                {
                    if d != 2 as u32 && d != 6 as u32 { return 0; }
                    i = d + 3 as u32;
                    sz_hdr = 3 as u8;
                    break '__s23;
                }
                {
                    if d != 4 as u32 { return 0; }
                    i = 9 as u32;
                    sz_hdr = 5 as u8;
                    break '__s23;
                }
                { return 0; }
            }
            13 => {
                {
                    if d != 2 as u32 && d != 6 as u32 { return 0; }
                    i = d + 3 as u32;
                    sz_hdr = 3 as u8;
                    break '__s23;
                }
                {
                    if d != 4 as u32 { return 0; }
                    i = 9 as u32;
                    sz_hdr = 5 as u8;
                    break '__s23;
                }
                { return 0; }
            }
            14 => {
                {
                    if d != 4 as u32 { return 0; }
                    i = 9 as u32;
                    sz_hdr = 5 as u8;
                    break '__s23;
                }
                { return 0; }
            }
            15 => { { return 0; } }
            _ => {
                {
                    if 1 << d & 278 == 0 { return 0; }
                    i = d + 1 as u32;
                    sz_hdr = 1 as u8;
                    break '__s23;
                }
                {
                    if 1 << d & 138 == 0 { return 0; }
                    i = d + 2 as u32;
                    sz_hdr = 2 as u8;
                    break '__s23;
                }
                {
                    if d != 2 as u32 && d != 6 as u32 { return 0; }
                    i = d + 3 as u32;
                    sz_hdr = 3 as u8;
                    break '__s23;
                }
                {
                    if d != 4 as u32 { return 0; }
                    i = 9 as u32;
                    sz_hdr = 5 as u8;
                    break '__s23;
                }
                { return 0; }
            }
        }
    }
    { let _ = 0; };
    unsafe {
        *a_out_1.offset(0 as isize) =
            (unsafe { *a_ins_1.offset(0 as isize) } as i32 & 15 |
                    a_type[(i - 2 as u32) as usize] as i32) as u8
    };
    unsafe {
        memcpy(unsafe { &raw mut *a_out_1.add(i as usize) } as *mut (),
            unsafe { &raw const *a_ins_1.add(sz_hdr as usize) } as *const (),
            (n_ins_1 - sz_hdr as u32) as u64)
    };
    sz_payload = n_ins_1 - sz_hdr as u32;
    loop {
        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        unsafe { *a_out_1.add(i as usize) = (sz_payload & 255 as u32) as u8 };
        if i == 1 as u32 { break; }
        sz_payload >>= 8 as u32;
    }
    { let _ = 0; };
    return 1;
}
extern "C" fn json_blob_edit(p_parse_1: *mut JsonParse, i_del_1: u32,
    n_del_1: u32, a_ins_1: *const u8, n_ins_1: u32) -> () {
    let d: i64 = n_ins_1 as i64 - n_del_1 as i64;
    { let _ = 0; };
    if d < 0 as i64 && d >= -8 as i64 && a_ins_1 != core::ptr::null() &&
            json_blob_overwrite(unsafe {
                        &mut *unsafe { (*p_parse_1).a_blob.add(i_del_1 as usize) }
                    }, a_ins_1, n_ins_1, -d as i32 as u32) != 0 {
        return;
    }
    if d != 0 as i64 {
        if unsafe { (*p_parse_1).n_blob } as i64 + d >
                unsafe { (*p_parse_1).n_blob_alloc } as i64 {
            json_blob_expand(unsafe { &mut *p_parse_1 },
                (unsafe { (*p_parse_1).n_blob } as i64 + d) as u32);
            if unsafe { (*p_parse_1).oom } != 0 { return; }
        }
        unsafe {
            memmove(unsafe {
                        &raw mut *unsafe {
                                    (*p_parse_1).a_blob.add((i_del_1 + n_ins_1) as usize)
                                }
                    } as *mut (),
                unsafe {
                        &raw mut *unsafe {
                                    (*p_parse_1).a_blob.add((i_del_1 + n_del_1) as usize)
                                }
                    } as *const (),
                (unsafe { (*p_parse_1).n_blob } - (i_del_1 + n_del_1)) as u64)
        };
        unsafe { (*p_parse_1).n_blob += d as u32 };
        unsafe { (*p_parse_1).delta += d as i32 };
    }
    if n_ins_1 != 0 && !(a_ins_1).is_null() {
        unsafe {
            memcpy(unsafe {
                        &raw mut *unsafe {
                                    (*p_parse_1).a_blob.add(i_del_1 as usize)
                                }
                    } as *mut (), a_ins_1 as *const (), n_ins_1 as u64)
        };
    }
}
extern "C" fn json_label_compare_escaped(mut z_left_1: *const i8,
    mut n_left_1: u32, raw_left_1: i32, mut z_right_1: *const i8,
    mut n_right_1: u32, raw_right_1: i32) -> i32 {
    let mut c_left: u32 = 0 as u32;
    let mut c_right: u32 = 0 as u32;
    { let _ = 0; };
    loop {
        if n_left_1 == 0 as u32 {
            c_left = 0 as u32;
        } else if raw_left_1 != 0 ||
                unsafe { *z_left_1.offset(0 as isize) } as i32 != '\\' as i32
            {
            c_left =
                unsafe { *(z_left_1 as *mut u8).offset(0 as isize) } as u32;
            if c_left >= 192 as u32 {
                let sz: i32 =
                    unsafe {
                        sqlite3_utf8_read_limited(z_left_1 as *mut u8 as *const u8,
                            n_left_1 as i32, &mut c_left)
                    };
                {
                    let __n = sz;
                    let __p = &mut z_left_1;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                n_left_1 -= sz as u32;
            } else {
                {
                    let __p = &mut z_left_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n_left_1; let __t = *__p; *__p -= 1; __t };
            }
        } else {
            let n: u32 =
                json_unescape_one_char(z_left_1, n_left_1, &mut c_left);
            {
                let __n = n;
                let __p = &mut z_left_1;
                *__p = unsafe { (*__p).add(__n as usize) };
            };
            { let _ = 0; };
            n_left_1 -= n;
        }
        if n_right_1 == 0 as u32 {
            c_right = 0 as u32;
        } else if raw_right_1 != 0 ||
                unsafe { *z_right_1.offset(0 as isize) } as i32 != '\\' as i32
            {
            c_right =
                unsafe { *(z_right_1 as *mut u8).offset(0 as isize) } as u32;
            if c_right >= 192 as u32 {
                let sz: i32 =
                    unsafe {
                        sqlite3_utf8_read_limited(z_right_1 as *mut u8 as *const u8,
                            n_right_1 as i32, &mut c_right)
                    };
                {
                    let __n = sz;
                    let __p = &mut z_right_1;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                n_right_1 -= sz as u32;
            } else {
                {
                    let __p = &mut z_right_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                { let __p = &mut n_right_1; let __t = *__p; *__p -= 1; __t };
            }
        } else {
            let n: u32 =
                json_unescape_one_char(z_right_1, n_right_1, &mut c_right);
            {
                let __n = n;
                let __p = &mut z_right_1;
                *__p = unsafe { (*__p).add(__n as usize) };
            };
            { let _ = 0; };
            n_right_1 -= n;
        }
        if c_left != c_right { return 0; }
        if c_left == 0 as u32 { return 1; }
    }
}
extern "C" fn json_label_compare(z_left_1: *const i8, n_left_1: u32,
    raw_left_1: i32, z_right_1: *const i8, n_right_1: u32, raw_right_1: i32)
    -> i32 {
    if raw_left_1 != 0 && raw_right_1 != 0 {
        if n_left_1 != n_right_1 { return 0; }
        return (unsafe {
                        memcmp(z_left_1 as *const (), z_right_1 as *const (),
                            n_left_1 as u64)
                    } == 0) as i32;
    } else {
        return json_label_compare_escaped(z_left_1, n_left_1, raw_left_1,
                z_right_1, n_right_1, raw_right_1);
    }
}
extern "C" fn json_after_edit_size_adjust(p_parse_1: *mut JsonParse,
    i_root_1: u32) -> () {
    let mut sz: u32 = 0 as u32;
    let mut n_blob: u32 = 0 as u32;
    { let _ = 0; };
    { let _ = 0; };
    n_blob = unsafe { (*p_parse_1).n_blob };
    unsafe { (*p_parse_1).n_blob = unsafe { (*p_parse_1).n_blob_alloc } };
    {
        let _ = jsonb_payload_size(unsafe { &*p_parse_1 }, i_root_1, &mut sz);
    };
    unsafe { (*p_parse_1).n_blob = n_blob };
    sz += unsafe { (*p_parse_1).delta } as u32;
    unsafe {
        (*p_parse_1).delta +=
            json_blob_change_payload_size(p_parse_1, i_root_1, sz)
    };
}
extern "C" fn json_create_edit_substructure(p_parse_1: &mut JsonParse,
    p_ins_1: *mut JsonParse, z_tail_1: *const i8) -> u32 {
    let mut rc: i32 = 0;
    unsafe {
        memset(p_ins_1 as *mut (), 0,
            core::mem::size_of::<JsonParse>() as u64)
    };
    unsafe { (*p_ins_1).db = (*p_parse_1).db };
    if unsafe { *z_tail_1.offset(0 as isize) } as i32 == 0 {
        unsafe { (*p_ins_1).a_blob = (*p_parse_1).a_ins };
        unsafe { (*p_ins_1).n_blob = (*p_parse_1).n_ins };
        rc = 0;
    } else {
        unsafe { (*p_ins_1).n_blob = 1 as u32 };
        unsafe {
            (*p_ins_1).a_blob =
                &raw const empty_object_1[(unsafe {
                                            *z_tail_1.offset(0 as isize)
                                        } as i32 == '.' as i32) as usize] as *mut u8
        };
        unsafe { (*p_ins_1).e_edit = (*p_parse_1).e_edit };
        unsafe { (*p_ins_1).n_ins = (*p_parse_1).n_ins };
        unsafe { (*p_ins_1).a_ins = (*p_parse_1).a_ins };
        unsafe {
            (*p_ins_1).i_depth = ((*p_parse_1).i_depth as i32 + 1) as u16
        };
        if unsafe { (*p_ins_1).i_depth } as i32 >= 1000 {
            return 4294967292u32;
        }
        rc =
            unsafe { json_lookup_step(p_ins_1, 0 as u32, z_tail_1, 0 as u32) }
                as i32;
        {
            let __p = &mut (*p_parse_1).i_depth;
            let __t = *__p;
            *__p -= 1;
            __t
        };
        (*p_parse_1).oom |= unsafe { (*p_ins_1).oom } as i32 as u8;
    }
    return rc as u32;
}
extern "C" fn jsonb_array_count(p_parse_1: *const JsonParse, i_root_1: u32)
    -> u32 {
    let mut n: u32 = 0 as u32;
    let mut sz: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    let mut i_end: u32 = 0 as u32;
    let mut k: u32 = 0 as u32;
    n = jsonb_payload_size(unsafe { &*p_parse_1 }, i_root_1, &mut sz);
    i_end = i_root_1 + n + sz;
    {
        i = i_root_1 + n;
        '__b26: loop {
            if !(n > 0 as u32 && i < i_end) { break '__b26; }
            '__c26: loop {
                n = jsonb_payload_size(unsafe { &*p_parse_1 }, i, &mut sz);
                break '__c26;
            }
            {
                i += sz + n;
                { let __p = &mut k; let __t = *__p; *__p += 1; __t }
            };
        }
    }
    return k;
}
extern "C" fn json_lookup_step(p_parse_1: *mut JsonParse, mut i_root_1: u32,
    mut z_path_1: *const i8, i_label_1: u32) -> u32 {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut j: u32 = 0 as u32;
        let mut k: u32 = 0 as u32;
        let mut n_key: u32 = 0 as u32;
        let mut sz: u32 = 0 as u32;
        let mut n: u32 = 0 as u32;
        let mut i_end: u32 = 0 as u32;
        let mut rc: u32 = 0 as u32;
        let mut z_key: *const i8 = core::ptr::null();
        let mut x: u8 = 0 as u8;
        if unsafe { *z_path_1.offset(0 as isize) } as i32 == 0 {
            if unsafe { (*p_parse_1).e_edit } != 0 &&
                    json_blob_make_editable(p_parse_1,
                            unsafe { (*p_parse_1).n_ins }) != 0 {
                n =
                    jsonb_payload_size(unsafe { &*p_parse_1 }, i_root_1,
                        &mut sz);
                sz += n;
                if unsafe { (*p_parse_1).e_edit } as i32 == 1 {
                    if i_label_1 > 0 as u32 {
                        sz += i_root_1 - i_label_1;
                        i_root_1 = i_label_1;
                    }
                    json_blob_edit(p_parse_1, i_root_1, sz, core::ptr::null(),
                        0 as u32);
                } else if unsafe { (*p_parse_1).e_edit } as i32 == 3
                    {} else if unsafe { (*p_parse_1).e_edit } as i32 == 5 {
                    if unsafe { *z_path_1.offset(-1 as isize) } as i32 !=
                            ']' as i32 {
                        return 4294967293u32;
                    } else {
                        json_blob_edit(p_parse_1, i_root_1, 0 as u32,
                            unsafe { (*p_parse_1).a_ins } as *const u8,
                            unsafe { (*p_parse_1).n_ins });
                    }
                } else {
                    json_blob_edit(p_parse_1, i_root_1, sz,
                        unsafe { (*p_parse_1).a_ins } as *const u8,
                        unsafe { (*p_parse_1).n_ins });
                }
            }
            unsafe { (*p_parse_1).i_label = i_label_1 };
            return i_root_1;
        }
        if unsafe { *z_path_1.offset(0 as isize) } as i32 == '.' as i32 {
            let mut raw_key: i32 = 1;
            x =
                unsafe {
                    *unsafe { (*p_parse_1).a_blob.add(i_root_1 as usize) }
                };
            {
                let __p = &mut z_path_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            if unsafe { *z_path_1.offset(0 as isize) } as i32 == '\"' as i32 {
                z_key = unsafe { z_path_1.offset(1 as isize) };
                {
                    i = 1 as u32;
                    '__b27: loop {
                        if !(unsafe { *z_path_1.add(i as usize) } != 0 &&
                                        unsafe { *z_path_1.add(i as usize) } as i32 != '\"' as i32)
                            {
                            break '__b27;
                        }
                        '__c27: loop {
                            if unsafe { *z_path_1.add(i as usize) } as i32 ==
                                        '\\' as i32 &&
                                    unsafe { *z_path_1.add((i + 1 as u32) as usize) } as i32 !=
                                        0 {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                            break '__c27;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                n_key = i - 1 as u32;
                if unsafe { *z_path_1.add(i as usize) } != 0 {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                } else { return 4294967291u32; }
                raw_key =
                    (unsafe {
                                memchr(z_key as *const (), '\\' as i32, n_key as u64)
                            } == core::ptr::null_mut()) as i32;
            } else {
                z_key = z_path_1;
                {
                    i = 0 as u32;
                    '__b28: loop {
                        if !(unsafe { *z_path_1.add(i as usize) } != 0 &&
                                            unsafe { *z_path_1.add(i as usize) } as i32 != '.' as i32 &&
                                        unsafe { *z_path_1.add(i as usize) } as i32 != '[' as i32) {
                            break '__b28;
                        }
                        '__c28: loop { break '__c28; }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                n_key = i;
                if n_key == 0 as u32 { return 4294967291u32; }
            }
            if x as i32 & 15 != 12 { return 4294967294u32; }
            n = jsonb_payload_size(unsafe { &*p_parse_1 }, i_root_1, &mut sz);
            j = i_root_1 + n;
            i_end = j + sz;
            while j < i_end {
                let mut raw_label: i32 = 0;
                let mut z_label: *const i8 = core::ptr::null();
                x =
                    (unsafe { *unsafe { (*p_parse_1).a_blob.add(j as usize) } }
                                as i32 & 15) as u8;
                if (x as i32) < 7 || x as i32 > 10 { return 4294967295u32; }
                n = jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz);
                if n == 0 as u32 { return 4294967295u32; }
                k = j + n;
                if k + sz >= i_end { return 4294967295u32; }
                z_label =
                    unsafe {
                            &raw mut *unsafe { (*p_parse_1).a_blob.add(k as usize) }
                        } as *const i8;
                raw_label = (x as i32 == 7 || x as i32 == 10) as i32;
                if json_label_compare(z_key, n_key, raw_key, z_label, sz,
                            raw_label) != 0 {
                    let mut v: u32 = k + sz;
                    if unsafe {
                                        *unsafe { (*p_parse_1).a_blob.add(v as usize) }
                                    } as i32 & 15 > 12 {
                        return 4294967295u32;
                    }
                    n = jsonb_payload_size(unsafe { &*p_parse_1 }, v, &mut sz);
                    if n == 0 as u32 || v + n + sz > i_end {
                        return 4294967295u32;
                    }
                    { let _ = 0; };
                    if {
                                    let __p = unsafe { &mut (*p_parse_1).i_depth };
                                    *__p += 1;
                                    *__p
                                } as i32 >= 1000 {
                        return 4294967292u32;
                    }
                    rc =
                        json_lookup_step(p_parse_1, v,
                            unsafe { &*z_path_1.add(i as usize) }, j);
                    {
                        let __p = unsafe { &mut (*p_parse_1).i_depth };
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    if unsafe { (*p_parse_1).delta } != 0 {
                        json_after_edit_size_adjust(p_parse_1, i_root_1);
                    }
                    return rc;
                }
                j = k + sz;
                if unsafe { *unsafe { (*p_parse_1).a_blob.add(j as usize) } }
                                as i32 & 15 > 12 {
                    return 4294967295u32;
                }
                n = jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz);
                if n == 0 as u32 { return 4294967295u32; }
                j += n + sz;
            }
            if j > i_end { return 4294967295u32; }
            if unsafe { (*p_parse_1).e_edit } as i32 >= 3 {
                let mut n_ins: u32 = 0 as u32;
                let mut v: JsonParse = unsafe { core::mem::zeroed() };
                let mut ix: JsonParse = unsafe { core::mem::zeroed() };
                if unsafe { (*p_parse_1).e_edit } as i32 == 5 &&
                        unsafe {
                                sqlite3_strglob(c"*]".as_ptr() as *mut i8 as *const i8,
                                    unsafe { &*z_path_1.add(i as usize) })
                            } != 0 {
                    return 4294967293u32;
                }
                unsafe {
                    memset(&raw mut ix as *mut (), 0,
                        core::mem::size_of::<JsonParse>() as u64)
                };
                ix.db = unsafe { (*p_parse_1).db };
                json_blob_append_node(&mut ix,
                    if raw_key != 0 { 10 } else { 9 } as u8, n_key as u64,
                    core::ptr::null());
                unsafe { (*p_parse_1).oom |= ix.oom as i32 as u8 };
                rc =
                    json_create_edit_substructure(unsafe { &mut *p_parse_1 },
                        &mut v, unsafe { &*z_path_1.add(i as usize) });
                if !(rc >= 4294967291u32) as i32 != 0 &&
                        json_blob_make_editable(p_parse_1,
                                ix.n_blob + n_key + v.n_blob) != 0 {
                    { let _ = 0; };
                    n_ins = ix.n_blob + n_key + v.n_blob;
                    json_blob_edit(p_parse_1, j, 0 as u32, core::ptr::null(),
                        n_ins);
                    if (unsafe { (*p_parse_1).oom } == 0) as i32 != 0 {
                        { let _ = 0; };
                        { let _ = 0; };
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe { (*p_parse_1).a_blob.add(j as usize) }
                                    } as *mut (), ix.a_blob as *const (), ix.n_blob as u64)
                        };
                        k = j + ix.n_blob;
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe { (*p_parse_1).a_blob.add(k as usize) }
                                    } as *mut (), z_key as *const (), n_key as u64)
                        };
                        k += n_key;
                        unsafe {
                            memcpy(unsafe {
                                        &raw mut *unsafe { (*p_parse_1).a_blob.add(k as usize) }
                                    } as *mut (), v.a_blob as *const (), v.n_blob as u64)
                        };
                        if unsafe { (*p_parse_1).delta } != 0 {
                            json_after_edit_size_adjust(p_parse_1, i_root_1);
                        }
                    }
                }
                json_parse_reset(&mut v);
                json_parse_reset(&mut ix);
                return rc;
            }
        } else if unsafe { *z_path_1.offset(0 as isize) } as i32 == '[' as i32
            {
            let mut kk: u64 = 0 as u64;
            x =
                (unsafe {
                                *unsafe { (*p_parse_1).a_blob.add(i_root_1 as usize) }
                            } as i32 & 15) as u8;
            if x as i32 != 11 { return 4294967294u32; }
            n = jsonb_payload_size(unsafe { &*p_parse_1 }, i_root_1, &mut sz);
            i = 1 as u32;
            while unsafe {
                                *(sqlite3_ctype_map.as_ptr() as
                                            *const u8).add(unsafe { *z_path_1.add(i as usize) } as u8 as
                                            usize)
                            } as i32 & 4 != 0 {
                if kk < 4294967295u32 as u64 {
                    kk =
                        kk * 10 as u64 + unsafe { *z_path_1.add(i as usize) } as u64
                            - '0' as i32 as u64;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
            if i < 2 as u32 ||
                    unsafe { *z_path_1.add(i as usize) } as i32 != ']' as i32 {
                if unsafe { *z_path_1.offset(1 as isize) } as i32 ==
                        '#' as i32 {
                    kk =
                        jsonb_array_count(p_parse_1 as *const JsonParse, i_root_1)
                            as u64;
                    i = 2 as u32;
                    if unsafe { *z_path_1.offset(2 as isize) } as i32 ==
                                '-' as i32 &&
                            unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z_path_1.offset(3 as isize) } as u8
                                                        as usize)
                                        } as i32 & 4 != 0 {
                        let mut nn: u64 = 0 as u64;
                        i = 3 as u32;
                        '__b31: loop {
                            '__c31: loop {
                                if nn < 4294967295u32 as u64 {
                                    nn =
                                        nn * 10 as u64 + unsafe { *z_path_1.add(i as usize) } as u64
                                            - '0' as i32 as u64;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                break '__c31;
                            }
                            if !(unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z_path_1.add(i as usize) } as u8 as
                                                                    usize)
                                                    } as i32 & 4 != 0) {
                                break '__b31;
                            }
                        }
                        if nn > kk { return 4294967294u32; }
                        kk -= nn;
                    }
                    if unsafe { *z_path_1.add(i as usize) } as i32 != ']' as i32
                        {
                        return 4294967291u32;
                    }
                } else { return 4294967291u32; }
            }
            j = i_root_1 + n;
            i_end = j + sz;
            while j < i_end {
                if kk == 0 as u64 {
                    if {
                                    let __p = unsafe { &mut (*p_parse_1).i_depth };
                                    *__p += 1;
                                    *__p
                                } as i32 >= 1000 {
                        return 4294967292u32;
                    }
                    rc =
                        json_lookup_step(p_parse_1, j,
                            unsafe { &*z_path_1.add((i + 1 as u32) as usize) },
                            0 as u32);
                    {
                        let __p = unsafe { &mut (*p_parse_1).i_depth };
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    if unsafe { (*p_parse_1).delta } != 0 {
                        json_after_edit_size_adjust(p_parse_1, i_root_1);
                    }
                    return rc;
                }
                { let __p = &mut kk; let __t = *__p; *__p -= 1; __t };
                n = jsonb_payload_size(unsafe { &*p_parse_1 }, j, &mut sz);
                if n == 0 as u32 { return 4294967295u32; }
                j += n + sz;
            }
            if j > i_end { return 4294967295u32; }
            if kk > 0 as u64 { return 4294967294u32; }
            if unsafe { (*p_parse_1).e_edit } as i32 >= 3 {
                let mut v: JsonParse = unsafe { core::mem::zeroed() };
                rc =
                    json_create_edit_substructure(unsafe { &mut *p_parse_1 },
                        &mut v, unsafe { &*z_path_1.add((i + 1 as u32) as usize) });
                if !(rc >= 4294967291u32) as i32 != 0 &&
                        json_blob_make_editable(p_parse_1, v.n_blob) != 0 {
                    { let _ = 0; };
                    json_blob_edit(p_parse_1, j, 0 as u32,
                        v.a_blob as *const u8, v.n_blob);
                }
                json_parse_reset(&mut v);
                if unsafe { (*p_parse_1).delta } != 0 {
                    json_after_edit_size_adjust(p_parse_1, i_root_1);
                }
                return rc;
            }
        } else { return 4294967291u32; }
        return 4294967294u32;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct JsonString {
    p_ctx: *mut sqlite3_context,
    z_buf: *mut i8,
    n_alloc: u64,
    n_used: u64,
    b_static: u8,
    e_err: u8,
    z_space: [i8; 100],
}
extern "C" fn json_string_zero(p: &mut JsonString) -> () {
    (*p).z_buf = &raw mut (*p).z_space[0 as usize] as *mut i8;
    (*p).n_alloc = core::mem::size_of::<[i8; 100]>() as u64;
    (*p).n_used = 0 as u64;
    (*p).b_static = 1 as u8;
}
extern "C" fn json_string_init(p: *mut JsonString,
    p_ctx_1: *mut sqlite3_context) -> () {
    unsafe {
        unsafe { (*p).p_ctx = p_ctx_1 };
        unsafe { (*p).e_err = 0 as u8 };
        json_string_zero(unsafe { &mut *p });
    }
}
extern "C" fn json_string_reset(p: *mut JsonString) -> () {
    if (unsafe { (*p).b_static } == 0) as i32 != 0 {
        unsafe { sqlite3_rc_str_unref(unsafe { (*p).z_buf } as *mut ()) };
    }
    json_string_zero(unsafe { &mut *p });
}
extern "C" fn json_string_oom(p: *mut JsonString) -> () {
    unsafe {
        unsafe { (*p).e_err |= 1 as u8 };
        if !(unsafe { (*p).p_ctx }).is_null() {
            unsafe { sqlite3_result_error_nomem(unsafe { (*p).p_ctx }) };
        }
        json_string_reset(p);
    }
}
extern "C" fn json_string_grow(p: *mut JsonString, n_1: u32) -> i32 {
    let n_total: u64 =
        if (n_1 as u64) < unsafe { (*p).n_alloc } {
            (unsafe { (*p).n_alloc }) * 2 as u64
        } else { (unsafe { (*p).n_alloc }) + n_1 as u64 + 10 as u64 };
    let mut z_new: *mut i8 = core::ptr::null_mut();
    if unsafe { (*p).b_static } != 0 {
        if unsafe { (*p).e_err } != 0 { return 1; }
        z_new = unsafe { sqlite3_rc_str_new(n_total) };
        if z_new == core::ptr::null_mut() { json_string_oom(p); return 7; }
        unsafe {
            memcpy(z_new as *mut (), unsafe { (*p).z_buf } as *const (),
                unsafe { (*p).n_used } as u64)
        };
        unsafe { (*p).z_buf = z_new };
        unsafe { (*p).b_static = 0 as u8 };
    } else {
        unsafe {
            (*p).z_buf =
                unsafe {
                    sqlite3_rc_str_resize(unsafe { (*p).z_buf }, n_total)
                }
        };
        if unsafe { (*p).z_buf } == core::ptr::null_mut() {
            unsafe { (*p).e_err |= 1 as u8 };
            json_string_zero(unsafe { &mut *p });
            return 7;
        }
    }
    unsafe { (*p).n_alloc = n_total };
    return 0;
}
extern "C" fn json_string_expand_and_append(p: *mut JsonString,
    z_in_1: *const i8, n_1: u32) -> () {
    { let _ = 0; };
    if json_string_grow(p, n_1) != 0 { return; }
    unsafe {
        memcpy(unsafe {
                    unsafe { (*p).z_buf.add(unsafe { (*p).n_used } as usize) }
                } as *mut (), z_in_1 as *const (), n_1 as u64)
    };
    unsafe { (*p).n_used += n_1 as u64 };
}
extern "C" fn json_append_raw_nz(p: *mut JsonString, z_in_1: *const i8,
    n_1: u32) -> () {
    { let _ = 0; };
    if n_1 as u64 + unsafe { (*p).n_used } >= unsafe { (*p).n_alloc } {
        json_string_expand_and_append(p, z_in_1, n_1);
    } else {
        unsafe {
            memcpy(unsafe {
                        unsafe { (*p).z_buf.add(unsafe { (*p).n_used } as usize) }
                    } as *mut (), z_in_1 as *const (), n_1 as u64)
        };
        unsafe { (*p).n_used += n_1 as u64 };
    }
}
extern "C" fn json_append_raw(p: *mut JsonString, z_in_1: *const i8, n_1: u32)
    -> () {
    if n_1 == 0 as u32 { return; }
    if n_1 as u64 + unsafe { (*p).n_used } >= unsafe { (*p).n_alloc } {
        json_string_expand_and_append(p, z_in_1, n_1);
    } else {
        unsafe {
            memcpy(unsafe {
                        unsafe { (*p).z_buf.add(unsafe { (*p).n_used } as usize) }
                    } as *mut (), z_in_1 as *const (), n_1 as u64)
        };
        unsafe { (*p).n_used += n_1 as u64 };
    }
}
extern "C" fn json_append_char_expand(p: *mut JsonString, c: i8) -> () {
    if json_string_grow(p, 1 as u32) != 0 { return; }
    unsafe {
        *unsafe {
                    (*p).z_buf.add({
                                let __p = unsafe { &mut (*p).n_used };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as usize)
                } = c
    };
}
extern "C" fn json_append_char(p: *mut JsonString, c: i8) -> () {
    if unsafe { (*p).n_used } >= unsafe { (*p).n_alloc } {
        json_append_char_expand(p, c);
    } else {
        unsafe {
            *unsafe {
                        (*p).z_buf.add({
                                    let __p = unsafe { &mut (*p).n_used };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as usize)
                    } = c
        };
    }
}
unsafe extern "C" fn json_printf(n_1: i32, p: *mut JsonString,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    if unsafe { (*p).n_used } + n_1 as u64 >= unsafe { (*p).n_alloc } &&
            json_string_grow(p, n_1 as u32) != 0 {
        return;
    }
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    unsafe {
        sqlite3_vsnprintf(n_1,
            unsafe {
                unsafe { (*p).z_buf.add(unsafe { (*p).n_used } as usize) }
            }, z_format_1, ap)
    };
    ();
    unsafe {
        (*p).n_used +=
            unsafe {
                        strlen(unsafe {
                                    unsafe { (*p).z_buf.add(unsafe { (*p).n_used } as usize) }
                                } as *const i8)
                    } as i32 as u64
    };
}
extern "C" fn json_append_control_char(p: &mut JsonString, c: u8) -> () {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if a_special[c as usize] != 0 {
        unsafe { *(*p).z_buf.add((*p).n_used as usize) = '\\' as i32 as i8 };
        unsafe {
            *(*p).z_buf.add(((*p).n_used + 1 as u64) as usize) =
                a_special[c as usize] as i8
        };
        (*p).n_used += 2 as u64;
    } else {
        unsafe { *(*p).z_buf.add((*p).n_used as usize) = '\\' as i32 as i8 };
        unsafe {
            *(*p).z_buf.add(((*p).n_used + 1 as u64) as usize) =
                'u' as i32 as i8
        };
        unsafe {
            *(*p).z_buf.add(((*p).n_used + 2 as u64) as usize) =
                '0' as i32 as i8
        };
        unsafe {
            *(*p).z_buf.add(((*p).n_used + 3 as u64) as usize) =
                '0' as i32 as i8
        };
        unsafe {
            *(*p).z_buf.add(((*p).n_used + 4 as u64) as usize) =
                unsafe {
                    *(c"0123456789abcdef".as_ptr() as
                                *mut i8).offset((c as i32 >> 4) as isize)
                }
        };
        unsafe {
            *(*p).z_buf.add(((*p).n_used + 5 as u64) as usize) =
                unsafe {
                    *(c"0123456789abcdef".as_ptr() as
                                *mut i8).offset((c as i32 & 15) as isize)
                }
        };
        (*p).n_used += 6 as u64;
    }
}
extern "C" fn json_append_string(p: *mut JsonString, z_in_1: *const i8,
    mut n_1: u32) -> () {
    let mut k: u32 = 0 as u32;
    let mut c: u8 = 0 as u8;
    let mut z: *const u8 = z_in_1 as *const u8;
    if z == core::ptr::null() { return; }
    if n_1 as u64 + unsafe { (*p).n_used } + 2 as u64 >=
                unsafe { (*p).n_alloc } &&
            json_string_grow(p, n_1 + 2 as u32) != 0 {
        return;
    }
    unsafe {
        *unsafe {
                    (*p).z_buf.add({
                                let __p = unsafe { &mut (*p).n_used };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as usize)
                } = '\"' as i32 as i8
    };
    loop {
        k = 0 as u32;
        loop {
            if k + 3 as u32 >= n_1 {
                while k < n_1 &&
                        json_is_ok[unsafe { *z.add(k as usize) } as usize] != 0 {
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                }
                break;
            }
            if (json_is_ok[unsafe { *z.add(k as usize) } as usize] == 0) as
                        i32 != 0 {
                break;
            }
            if (json_is_ok[unsafe { *z.add((k + 1 as u32) as usize) } as
                                    usize] == 0) as i32 != 0 {
                k += 1 as u32;
                break;
            }
            if (json_is_ok[unsafe { *z.add((k + 2 as u32) as usize) } as
                                    usize] == 0) as i32 != 0 {
                k += 2 as u32;
                break;
            }
            if (json_is_ok[unsafe { *z.add((k + 3 as u32) as usize) } as
                                    usize] == 0) as i32 != 0 {
                k += 3 as u32;
                break;
            } else { k += 4 as u32; }
        }
        if k >= n_1 {
            if k > 0 as u32 {
                unsafe {
                    memcpy(unsafe {
                                &raw mut *unsafe {
                                            (*p).z_buf.add(unsafe { (*p).n_used } as usize)
                                        }
                            } as *mut (), z as *const (), k as u64)
                };
                unsafe { (*p).n_used += k as u64 };
            }
            break;
        }
        if k > 0 as u32 {
            unsafe {
                memcpy(unsafe {
                            &raw mut *unsafe {
                                        (*p).z_buf.add(unsafe { (*p).n_used } as usize)
                                    }
                        } as *mut (), z as *const (), k as u64)
            };
            unsafe { (*p).n_used += k as u64 };
            {
                let __n = k;
                let __p = &mut z;
                *__p = unsafe { (*__p).add(__n as usize) };
            };
            n_1 -= k;
        }
        c = unsafe { *z.offset(0 as isize) } as u8;
        if c as i32 == '\"' as i32 || c as i32 == '\\' as i32 {
            if unsafe { (*p).n_used } + n_1 as u64 + 3 as u64 >
                        unsafe { (*p).n_alloc } &&
                    json_string_grow(p, n_1 + 3 as u32) != 0 {
                return;
            }
            unsafe {
                *unsafe {
                            (*p).z_buf.add({
                                        let __p = unsafe { &mut (*p).n_used };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as usize)
                        } = '\\' as i32 as i8
            };
            unsafe {
                *unsafe {
                            (*p).z_buf.add({
                                        let __p = unsafe { &mut (*p).n_used };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as usize)
                        } = c as i8
            };
        } else if c as i32 == '\'' as i32 {
            unsafe {
                *unsafe {
                            (*p).z_buf.add({
                                        let __p = unsafe { &mut (*p).n_used };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as usize)
                        } = c as i8
            };
        } else {
            if unsafe { (*p).n_used } + n_1 as u64 + 7 as u64 >
                        unsafe { (*p).n_alloc } &&
                    json_string_grow(p, n_1 + 7 as u32) != 0 {
                return;
            }
            json_append_control_char(unsafe { &mut *p }, c);
        }
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
        { let __p = &mut n_1; let __t = *__p; *__p -= 1; __t };
    }
    unsafe {
        *unsafe {
                    (*p).z_buf.add({
                                let __p = unsafe { &mut (*p).n_used };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as usize)
                } = '\"' as i32 as i8
    };
    { let _ = 0; };
}
extern "C" fn json_string_too_deep(p: *mut JsonString) -> () {
    unsafe {
        unsafe { (*p).e_err |= 4 as u8 };
        { let _ = 0; };
        unsafe {
            sqlite3_result_error(unsafe { (*p).p_ctx },
                c"JSON nested too deep".as_ptr() as *mut i8 as *const i8, -1)
        };
        json_string_reset(p);
    }
}
extern "C" fn json_string_trim_one_char(p: &mut JsonString) -> () {
    if (*p).e_err as i32 == 0 {
        { let _ = 0; };
        { let __p = &mut (*p).n_used; let __t = *__p; *__p -= 1; __t };
    }
}
extern "C" fn json_translate_blob_to_text(p_parse_1: *mut JsonParse, i: u32,
    p_out_1: *mut JsonString) -> u32 {
    unsafe {
        let mut sz: u32 = 0 as u32;
        let mut n: u32 = 0 as u32;
        let mut j: u32 = 0 as u32;
        let mut i_end: u32 = 0 as u32;
        let mut k: u32 = 0 as u32;
        let mut u: sqlite3_uint64 = 0 as sqlite3_uint64;
        let mut z_in: *const i8 = core::ptr::null();
        let mut b_overflow: i32 = 0;
        let mut k__1: u32 = 0 as u32;
        let mut z_in_1: *const i8 = core::ptr::null();
        let mut z_in_2: *const i8 = core::ptr::null();
        let mut k__2: u32 = 0 as u32;
        let mut sz2: u32 = 0 as u32;
        let mut x: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s37:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        unsafe { (*p_out_1).e_err |= 2 as u8 };
                        __state = 189;
                    }
                    3 => {
                        n = jsonb_payload_size(unsafe { &*p_parse_1 }, i, &mut sz);
                        __state = 4;
                    }
                    4 => {
                        if n == 0 as u32 { __state = 6; } else { __state = 5; }
                    }
                    5 => {
                        '__s38:
                            {
                            match unsafe {
                                            *unsafe { (*p_parse_1).a_blob.add(i as usize) }
                                        } as i32 & 15 {
                                0 => { __state = 9; }
                                1 => { __state = 10; }
                                2 => { __state = 11; }
                                3 => { __state = 12; }
                                5 => { __state = 13; }
                                4 => { __state = 14; }
                                6 => { __state = 15; }
                                7 => { __state = 16; }
                                8 => { __state = 17; }
                                9 => { __state = 18; }
                                10 => { __state = 19; }
                                11 => { __state = 20; }
                                12 => { __state = 21; }
                                _ => { __state = 22; }
                            }
                        }
                    }
                    6 => {
                        unsafe { (*p_out_1).e_err |= 2 as u8 };
                        __state = 7;
                    }
                    7 => { return unsafe { (*p_parse_1).n_blob } + 1 as u32; }
                    8 => { return i + n + sz; }
                    9 => {
                        json_append_raw_nz(p_out_1,
                            c"null".as_ptr() as *mut i8 as *const i8, 4 as u32);
                        __state = 25;
                    }
                    10 => {
                        json_append_raw_nz(p_out_1,
                            c"true".as_ptr() as *mut i8 as *const i8, 4 as u32);
                        __state = 27;
                    }
                    11 => {
                        json_append_raw_nz(p_out_1,
                            c"false".as_ptr() as *mut i8 as *const i8, 5 as u32);
                        __state = 29;
                    }
                    12 => { __state = 13; }
                    13 => {
                        if sz == 0 as u32 { __state = 32; } else { __state = 31; }
                    }
                    14 => { k = 2 as u32; __state = 36; }
                    15 => { k__1 = 0 as u32; __state = 58; }
                    16 => { __state = 17; }
                    17 => {
                        if unsafe { (*p_out_1).n_used } + sz as u64 + 2 as u64 <=
                                    unsafe { (*p_out_1).n_alloc } ||
                                json_string_grow(p_out_1, sz + 2 as u32) == 0 {
                            __state = 75;
                        } else { __state = 74; }
                    }
                    18 => { __state = 81; }
                    19 => {
                        json_append_string(p_out_1,
                            unsafe {
                                    &raw mut *unsafe {
                                                (*p_parse_1).a_blob.add((i + n) as usize)
                                            }
                                } as *const i8, sz);
                        __state = 157;
                    }
                    20 => {
                        json_append_char(p_out_1, '[' as i32 as i8);
                        __state = 159;
                    }
                    21 => { x = 0; __state = 174; }
                    22 => { __state = 2; }
                    23 => { __state = 9; }
                    24 => { __state = 10; }
                    25 => { return i + 1 as u32; }
                    26 => { __state = 11; }
                    27 => { return i + 1 as u32; }
                    28 => { __state = 12; }
                    29 => { return i + 1 as u32; }
                    30 => { __state = 34; }
                    31 => {
                        json_append_raw(p_out_1,
                            unsafe {
                                    &raw mut *unsafe {
                                                (*p_parse_1).a_blob.add((i + n) as usize)
                                            }
                                } as *const i8, sz);
                        __state = 33;
                    }
                    32 => { __state = 2; }
                    33 => { __state = 8; }
                    34 => { __state = 14; }
                    35 => { __state = 15; }
                    36 => { u = 0 as sqlite3_uint64; __state = 37; }
                    37 => {
                        z_in =
                            unsafe {
                                    &raw mut *unsafe {
                                                (*p_parse_1).a_blob.add((i + n) as usize)
                                            }
                                } as *const i8;
                        __state = 38;
                    }
                    38 => { b_overflow = 0; __state = 39; }
                    39 => {
                        if sz == 0 as u32 { __state = 41; } else { __state = 40; }
                    }
                    40 => {
                        if unsafe { *z_in.offset(0 as isize) } as i32 == '-' as i32
                            {
                            __state = 43;
                        } else { __state = 44; }
                    }
                    41 => { __state = 2; }
                    42 => { __state = 48; }
                    43 => {
                        json_append_char(p_out_1, '-' as i32 as i8);
                        __state = 45;
                    }
                    44 => {
                        if unsafe { *z_in.offset(0 as isize) } as i32 == '+' as i32
                            {
                            __state = 46;
                        } else { __state = 42; }
                    }
                    45 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 42;
                    }
                    46 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 42;
                    }
                    47 => {
                        unsafe {
                            json_printf(100, p_out_1,
                                if b_overflow != 0 {
                                        c"9.0e999".as_ptr() as *mut i8
                                    } else { c"%llu".as_ptr() as *mut i8 } as *const i8, u)
                        };
                        __state = 56;
                    }
                    48 => { if k < sz { __state = 49; } else { __state = 47; } }
                    49 => {
                        if (unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z_in.add(k as usize) } as u8 as
                                                                usize)
                                                } as i32 & 8 == 0) as i32 != 0 {
                            __state = 51;
                        } else { __state = 52; }
                    }
                    50 => {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                        __state = 48;
                    }
                    51 => {
                        unsafe { (*p_out_1).e_err |= 2 as u8 };
                        __state = 53;
                    }
                    52 => {
                        if u >> 60 != 0 as u64 {
                            __state = 54;
                        } else { __state = 55; }
                    }
                    53 => { __state = 47; }
                    54 => { b_overflow = 1; __state = 50; }
                    55 => {
                        u =
                            u * 16 as sqlite3_uint64 +
                                unsafe {
                                        sqlite3_hex_to_int(unsafe { *z_in.add(k as usize) } as i32)
                                    } as sqlite3_uint64;
                        __state = 50;
                    }
                    56 => { __state = 8; }
                    57 => { __state = 16; }
                    58 => {
                        z_in_1 =
                            unsafe {
                                    &raw mut *unsafe {
                                                (*p_parse_1).a_blob.add((i + n) as usize)
                                            }
                                } as *const i8;
                        __state = 59;
                    }
                    59 => {
                        if sz == 0 as u32 { __state = 61; } else { __state = 60; }
                    }
                    60 => {
                        if unsafe { *z_in_1.offset(0 as isize) } as i32 ==
                                '-' as i32 {
                            __state = 63;
                        } else { __state = 62; }
                    }
                    61 => { __state = 2; }
                    62 => {
                        if unsafe { *z_in_1.add(k__1 as usize) } as i32 ==
                                '.' as i32 {
                            __state = 66;
                        } else { __state = 65; }
                    }
                    63 => {
                        json_append_char(p_out_1, '-' as i32 as i8);
                        __state = 64;
                    }
                    64 => {
                        { let __p = &mut k__1; let __t = *__p; *__p += 1; __t };
                        __state = 62;
                    }
                    65 => { __state = 68; }
                    66 => {
                        json_append_char(p_out_1, '0' as i32 as i8);
                        __state = 65;
                    }
                    67 => { __state = 8; }
                    68 => {
                        if k__1 < sz { __state = 69; } else { __state = 67; }
                    }
                    69 => {
                        json_append_char(p_out_1,
                            unsafe { *z_in_1.add(k__1 as usize) });
                        __state = 71;
                    }
                    70 => {
                        { let __p = &mut k__1; let __t = *__p; *__p += 1; __t };
                        __state = 68;
                    }
                    71 => {
                        if unsafe { *z_in_1.add(k__1 as usize) } as i32 ==
                                    '.' as i32 &&
                                (k__1 + 1 as u32 == sz ||
                                    (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe {
                                                                                *z_in_1.add((k__1 + 1 as u32) as usize)
                                                                            } as u8 as usize)
                                                        } as i32 & 4 == 0) as i32 != 0) {
                            __state = 72;
                        } else { __state = 70; }
                    }
                    72 => {
                        json_append_char(p_out_1, '0' as i32 as i8);
                        __state = 70;
                    }
                    73 => { __state = 79; }
                    74 => { __state = 8; }
                    75 => {
                        unsafe {
                            *unsafe {
                                        (*p_out_1).z_buf.add(unsafe { (*p_out_1).n_used } as usize)
                                    } = '\"' as i32 as i8
                        };
                        __state = 76;
                    }
                    76 => {
                        unsafe {
                            memcpy(unsafe {
                                        unsafe {
                                            unsafe {
                                                (*p_out_1).z_buf.add(unsafe { (*p_out_1).n_used } as
                                                            usize).offset(1 as isize)
                                            }
                                        }
                                    } as *mut (),
                                unsafe {
                                            &raw mut *unsafe {
                                                        (*p_parse_1).a_blob.add((i + n) as usize)
                                                    }
                                        } as *const i8 as *const (), sz as u64)
                        };
                        __state = 77;
                    }
                    77 => {
                        unsafe {
                            *unsafe {
                                        (*p_out_1).z_buf.add((unsafe { (*p_out_1).n_used } +
                                                        sz as u64 + 1 as u64) as usize)
                                    } = '\"' as i32 as i8
                        };
                        __state = 78;
                    }
                    78 => {
                        unsafe { (*p_out_1).n_used += (sz + 2 as u32) as u64 };
                        __state = 74;
                    }
                    79 => { __state = 18; }
                    80 => { __state = 19; }
                    81 => { __state = 82; }
                    82 => { sz2 = sz; __state = 83; }
                    83 => {
                        z_in_2 =
                            unsafe {
                                    &raw mut *unsafe {
                                                (*p_parse_1).a_blob.add((i + n) as usize)
                                            }
                                } as *const i8;
                        __state = 84;
                    }
                    84 => {
                        json_append_char(p_out_1, '\"' as i32 as i8);
                        __state = 85;
                    }
                    85 => {
                        if sz2 > 0 as u32 { __state = 87; } else { __state = 86; }
                    }
                    86 => {
                        json_append_char(p_out_1, '\"' as i32 as i8);
                        __state = 155;
                    }
                    87 => { k__2 = 0 as u32; __state = 89; }
                    88 => {
                        if k__2 > 0 as u32 { __state = 93; } else { __state = 92; }
                    }
                    89 => {
                        if k__2 < sz2 &&
                                (json_is_ok[unsafe { *z_in_2.add(k__2 as usize) } as u8 as
                                                usize] != 0 ||
                                    unsafe { *z_in_2.add(k__2 as usize) } as i32 == '\'' as i32)
                            {
                            __state = 90;
                        } else { __state = 88; }
                    }
                    90 => { __state = 91; }
                    91 => {
                        { let __p = &mut k__2; let __t = *__p; *__p += 1; __t };
                        __state = 89;
                    }
                    92 => {
                        if unsafe { *z_in_2.offset(0 as isize) } as i32 ==
                                '\"' as i32 {
                            __state = 99;
                        } else { __state = 98; }
                    }
                    93 => {
                        json_append_raw_nz(p_out_1, z_in_2, k__2);
                        __state = 94;
                    }
                    94 => {
                        if k__2 >= sz2 { __state = 96; } else { __state = 95; }
                    }
                    95 => {
                        {
                            let __n = k__2;
                            let __p = &mut z_in_2;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        __state = 97;
                    }
                    96 => { __state = 86; }
                    97 => { sz2 -= k__2; __state = 92; }
                    98 => {
                        if unsafe { *z_in_2.offset(0 as isize) } as i32 <= 31 {
                            __state = 104;
                        } else { __state = 103; }
                    }
                    99 => {
                        json_append_raw_nz(p_out_1,
                            c"\\\"".as_ptr() as *mut i8 as *const i8, 2 as u32);
                        __state = 100;
                    }
                    100 => {
                        {
                            let __p = &mut z_in_2;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 101;
                    }
                    101 => {
                        { let __p = &mut sz2; let __t = *__p; *__p -= 1; __t };
                        __state = 102;
                    }
                    102 => { __state = 85; }
                    103 => { { let _ = 0; }; __state = 110; }
                    104 => {
                        if unsafe { (*p_out_1).n_used } + 7 as u64 >
                                    unsafe { (*p_out_1).n_alloc } &&
                                json_string_grow(p_out_1, 7 as u32) != 0 {
                            __state = 106;
                        } else { __state = 105; }
                    }
                    105 => {
                        json_append_control_char(unsafe { &mut *p_out_1 },
                            unsafe { *z_in_2.offset(0 as isize) } as u8);
                        __state = 107;
                    }
                    106 => { __state = 86; }
                    107 => {
                        {
                            let __p = &mut z_in_2;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 108;
                    }
                    108 => {
                        { let __p = &mut sz2; let __t = *__p; *__p -= 1; __t };
                        __state = 109;
                    }
                    109 => { __state = 85; }
                    110 => { { let _ = 0; }; __state = 111; }
                    111 => {
                        if sz2 < 2 as u32 { __state = 113; } else { __state = 112; }
                    }
                    112 => {
                        '__s39:
                            {
                            match unsafe { *z_in_2.offset(1 as isize) } as u8 {
                                39 => { __state = 116; }
                                118 => { __state = 117; }
                                120 => { __state = 118; }
                                48 => { __state = 119; }
                                13 => { __state = 120; }
                                10 => { __state = 121; }
                                226 => { __state = 122; }
                                _ => { __state = 123; }
                            }
                        }
                    }
                    113 => {
                        unsafe { (*p_out_1).e_err |= 2 as u8 };
                        __state = 114;
                    }
                    114 => { __state = 86; }
                    115 => { { let _ = 0; }; __state = 153; }
                    116 => {
                        json_append_char(p_out_1, '\'' as i32 as i8);
                        __state = 125;
                    }
                    117 => {
                        json_append_raw_nz(p_out_1,
                            c"\\u000b".as_ptr() as *mut i8 as *const i8, 6 as u32);
                        __state = 127;
                    }
                    118 => {
                        if sz2 < 4 as u32 { __state = 130; } else { __state = 129; }
                    }
                    119 => {
                        json_append_raw_nz(p_out_1,
                            c"\\u0000".as_ptr() as *mut i8 as *const i8, 6 as u32);
                        __state = 138;
                    }
                    120 => {
                        if sz2 > 2 as u32 &&
                                unsafe { *z_in_2.offset(2 as isize) } as i32 == '\n' as i32
                            {
                            __state = 141;
                        } else { __state = 140; }
                    }
                    121 => { __state = 115; }
                    122 => {
                        if sz2 < 4 as u32 ||
                                    128 != unsafe { *z_in_2.offset(2 as isize) } as u8 as i32 ||
                                168 != unsafe { *z_in_2.offset(3 as isize) } as u8 as i32 &&
                                    169 != unsafe { *z_in_2.offset(3 as isize) } as u8 as i32 {
                            __state = 146;
                        } else { __state = 145; }
                    }
                    123 => {
                        json_append_raw_nz(p_out_1, z_in_2, 2 as u32);
                        __state = 152;
                    }
                    124 => { __state = 116; }
                    125 => { __state = 115; }
                    126 => { __state = 117; }
                    127 => { __state = 115; }
                    128 => { __state = 118; }
                    129 => {
                        json_append_raw_nz(p_out_1,
                            c"\\u00".as_ptr() as *mut i8 as *const i8, 4 as u32);
                        __state = 133;
                    }
                    130 => {
                        unsafe { (*p_out_1).e_err |= 2 as u8 };
                        __state = 131;
                    }
                    131 => { sz2 = 2 as u32; __state = 132; }
                    132 => { __state = 115; }
                    133 => {
                        json_append_raw_nz(p_out_1,
                            unsafe { &*z_in_2.offset(2 as isize) }, 2 as u32);
                        __state = 134;
                    }
                    134 => {
                        {
                            let __n = 2;
                            let __p = &mut z_in_2;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 135;
                    }
                    135 => { sz2 -= 2 as u32; __state = 136; }
                    136 => { __state = 115; }
                    137 => { __state = 119; }
                    138 => { __state = 115; }
                    139 => { __state = 120; }
                    140 => { __state = 115; }
                    141 => {
                        {
                            let __p = &mut z_in_2;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 142;
                    }
                    142 => {
                        { let __p = &mut sz2; let __t = *__p; *__p -= 1; __t };
                        __state = 140;
                    }
                    143 => { __state = 121; }
                    144 => { __state = 122; }
                    145 => {
                        {
                            let __n = 2;
                            let __p = &mut z_in_2;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 149;
                    }
                    146 => {
                        unsafe { (*p_out_1).e_err |= 2 as u8 };
                        __state = 147;
                    }
                    147 => { sz2 = 2 as u32; __state = 148; }
                    148 => { __state = 115; }
                    149 => { sz2 -= 2 as u32; __state = 150; }
                    150 => { __state = 115; }
                    151 => { __state = 123; }
                    152 => { __state = 115; }
                    153 => {
                        {
                            let __n = 2;
                            let __p = &mut z_in_2;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 154;
                    }
                    154 => { sz2 -= 2 as u32; __state = 85; }
                    155 => { __state = 8; }
                    156 => { __state = 20; }
                    157 => { __state = 8; }
                    158 => { __state = 21; }
                    159 => { j = i + n; __state = 160; }
                    160 => { i_end = j + sz; __state = 161; }
                    161 => {
                        if {
                                        let __p = unsafe { &mut (*p_parse_1).i_depth };
                                        *__p += 1;
                                        *__p
                                    } as i32 > 1000 {
                            __state = 163;
                        } else { __state = 162; }
                    }
                    162 => {
                        if j < i_end && unsafe { (*p_out_1).e_err } as i32 == 0 {
                            __state = 165;
                        } else { __state = 164; }
                    }
                    163 => { json_string_too_deep(p_out_1); __state = 162; }
                    164 => {
                        {
                            let __p = unsafe { &mut (*p_parse_1).i_depth };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 167;
                    }
                    165 => {
                        j = json_translate_blob_to_text(p_parse_1, j, p_out_1);
                        __state = 166;
                    }
                    166 => {
                        json_append_char(p_out_1, ',' as i32 as i8);
                        __state = 162;
                    }
                    167 => {
                        if j > i_end { __state = 169; } else { __state = 168; }
                    }
                    168 => {
                        if sz > 0 as u32 { __state = 171; } else { __state = 170; }
                    }
                    169 => {
                        unsafe { (*p_out_1).e_err |= 2 as u8 };
                        __state = 168;
                    }
                    170 => {
                        json_append_char(p_out_1, ']' as i32 as i8);
                        __state = 172;
                    }
                    171 => {
                        json_string_trim_one_char(unsafe { &mut *p_out_1 });
                        __state = 170;
                    }
                    172 => { __state = 8; }
                    173 => { __state = 22; }
                    174 => {
                        json_append_char(p_out_1, '{' as i32 as i8);
                        __state = 175;
                    }
                    175 => { j = i + n; __state = 176; }
                    176 => { i_end = j + sz; __state = 177; }
                    177 => {
                        if {
                                        let __p = unsafe { &mut (*p_parse_1).i_depth };
                                        *__p += 1;
                                        *__p
                                    } as i32 > 1000 {
                            __state = 179;
                        } else { __state = 178; }
                    }
                    178 => {
                        if j < i_end && unsafe { (*p_out_1).e_err } as i32 == 0 {
                            __state = 181;
                        } else { __state = 180; }
                    }
                    179 => { json_string_too_deep(p_out_1); __state = 178; }
                    180 => {
                        {
                            let __p = unsafe { &mut (*p_parse_1).i_depth };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 183;
                    }
                    181 => {
                        j = json_translate_blob_to_text(p_parse_1, j, p_out_1);
                        __state = 182;
                    }
                    182 => {
                        json_append_char(p_out_1,
                            if { let __p = &mut x; let __t = *__p; *__p += 1; __t } & 1
                                        != 0 {
                                    ',' as i32
                                } else { ':' as i32 } as i8);
                        __state = 178;
                    }
                    183 => {
                        if x & 1 != 0 || j > i_end {
                            __state = 185;
                        } else { __state = 184; }
                    }
                    184 => {
                        if sz > 0 as u32 { __state = 187; } else { __state = 186; }
                    }
                    185 => {
                        unsafe { (*p_out_1).e_err |= 2 as u8 };
                        __state = 184;
                    }
                    186 => {
                        json_append_char(p_out_1, '}' as i32 as i8);
                        __state = 188;
                    }
                    187 => {
                        json_string_trim_one_char(unsafe { &mut *p_out_1 });
                        __state = 186;
                    }
                    188 => { __state = 8; }
                    189 => { __state = 8; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn json_string_terminate(p: *mut JsonString) -> i32 {
    json_append_char(p, 0 as i8);
    json_string_trim_one_char(unsafe { &mut *p });
    return (unsafe { (*p).e_err } as i32 == 0) as i32;
}
extern "C" fn json_return_string_as_blob(p_str_1: &JsonString) -> () {
    unsafe {
        let mut px: JsonParse = unsafe { core::mem::zeroed() };
        { let _ = 0; };
        unsafe {
            memset(&raw mut px as *mut (), 0,
                core::mem::size_of::<JsonParse>() as u64)
        };
        px.z_json = (*p_str_1).z_buf;
        px.n_json = (*p_str_1).n_used as i32;
        px.db = unsafe { sqlite3_context_db_handle((*p_str_1).p_ctx) };
        { let _ = json_translate_text_to_blob(&mut px, 0 as u32); };
        if px.oom != 0 {
            unsafe { sqlite3_db_free(px.db, px.a_blob as *mut ()) };
            unsafe { sqlite3_result_error_nomem((*p_str_1).p_ctx) };
        } else {
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                sqlite3_result_blob((*p_str_1).p_ctx, px.a_blob as *const (),
                    px.n_blob as i32,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(sqlite3_row_set_clear as *const ())
                        }))
            };
        }
    }
}
extern "C" fn json_return_string(p: *mut JsonString,
    p_parse_1: *mut JsonParse, ctx: *mut sqlite3_context) -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        json_string_terminate(p);
        if unsafe { (*p).e_err } as i32 == 0 {
            let flags: i32 =
                unsafe { sqlite3_user_data(unsafe { (*p).p_ctx }) } as i64 as
                    i32;
            if flags & 16 != 0 {
                unsafe { json_return_string_as_blob(unsafe { &*p }) };
            } else if unsafe { (*p).b_static } != 0 {
                unsafe {
                    sqlite3_result_text64(unsafe { (*p).p_ctx },
                        unsafe { (*p).z_buf } as *const i8, unsafe { (*p).n_used },
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }), 1 as u8)
                };
            } else {
                if !(p_parse_1).is_null() &&
                            unsafe { (*p_parse_1).b_json_is_rc_str } as i32 == 0 &&
                        unsafe { (*p_parse_1).n_blob_alloc } > 0 as u32 {
                    let mut rc: i32 = 0;
                    unsafe {
                        (*p_parse_1).z_json =
                            unsafe { sqlite3_rc_str_ref(unsafe { (*p).z_buf }) }
                    };
                    unsafe {
                        (*p_parse_1).n_json = unsafe { (*p).n_used } as i32
                    };
                    unsafe { (*p_parse_1).b_json_is_rc_str = 1 as u8 };
                    rc = json_cache_insert(ctx, p_parse_1);
                    if rc == 7 {
                        unsafe { sqlite3_result_error_nomem(ctx) };
                        json_string_reset(p);
                        return;
                    }
                }
                unsafe {
                    sqlite3_result_text64(unsafe { (*p).p_ctx },
                        unsafe { sqlite3_rc_str_ref(unsafe { (*p).z_buf }) } as
                            *const i8, unsafe { (*p).n_used },
                        Some(sqlite3_rc_str_unref), 1 as u8)
                };
            }
        } else if unsafe { (*p).e_err } as i32 & 1 != 0 {
            unsafe { sqlite3_result_error_nomem(unsafe { (*p).p_ctx }) };
        } else if unsafe { (*p).e_err } as i32 & 4 != 0
            {} else if unsafe { (*p).e_err } as i32 & 2 != 0 {
            unsafe {
                sqlite3_result_error(unsafe { (*p).p_ctx },
                    c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
            };
        }
        json_string_reset(p);
    }
}
extern "C" fn json_return_parse(ctx: *mut sqlite3_context, p: *mut JsonParse)
    -> () {
    let mut flgs: i32 = 0;
    if unsafe { (*p).oom } != 0 {
        unsafe { sqlite3_result_error_nomem(ctx) };
        return;
    }
    flgs = unsafe { sqlite3_user_data(ctx) } as i64 as i32;
    if flgs & 16 != 0 {
        if unsafe { (*p).n_blob_alloc } > 0 as u32 &&
                (unsafe { (*p).b_read_only } == 0) as i32 != 0 {
            unsafe {
                sqlite3_result_blob(ctx, unsafe { (*p).a_blob } as *const (),
                    unsafe { (*p).n_blob } as i32,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(sqlite3_row_set_clear as *const ())
                        }))
            };
            unsafe { (*p).n_blob_alloc = 0 as u32 };
        } else {
            unsafe {
                sqlite3_result_blob(ctx, unsafe { (*p).a_blob } as *const (),
                    unsafe { (*p).n_blob } as i32,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }))
            };
        }
    } else {
        let mut s: JsonString = unsafe { core::mem::zeroed() };
        json_string_init(&mut s, ctx);
        unsafe { (*p).delta = 0 };
        json_translate_blob_to_text(p, 0 as u32, &mut s);
        json_return_string(&mut s, p, ctx);
        unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
    }
}
extern "C" fn json_remove_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p: *mut JsonParse = core::ptr::null_mut();
    let mut z_path: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    let mut rc: u32 = 0 as u32;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s41:
            {
            match __state {
                0 => { __state = 4; }
                2 => { json_bad_path_error(ctx, z_path, 0); __state = 34; }
                3 => { json_parse_free(p); __state = 35; }
                4 => { z_path = core::ptr::null(); __state = 5; }
                5 => { __state = 6; }
                6 => { __state = 7; }
                7 => { if argc < 1 { __state = 9; } else { __state = 8; } }
                8 => {
                    p =
                        json_parse_func_arg(ctx,
                            unsafe { *argv.offset(0 as isize) },
                            if argc > 1 { 1 } else { 0 } as u32);
                    __state = 10;
                }
                9 => { return; }
                10 => {
                    if p == core::ptr::null_mut() {
                        __state = 12;
                    } else { __state = 11; }
                }
                11 => { i = 1; __state = 14; }
                12 => { return; }
                13 => { json_return_parse(ctx, p); __state = 31; }
                14 => { if i < argc { __state = 15; } else { __state = 13; } }
                15 => {
                    z_path =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *const i8;
                    __state = 17;
                }
                16 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 14;
                }
                17 => {
                    if z_path == core::ptr::null() {
                        __state = 19;
                    } else { __state = 18; }
                }
                18 => {
                    if unsafe { *z_path.offset(0 as isize) } as i32 !=
                            '$' as i32 {
                        __state = 21;
                    } else { __state = 20; }
                }
                19 => { __state = 3; }
                20 => {
                    if unsafe { *z_path.offset(1 as isize) } as i32 == 0 {
                        __state = 23;
                    } else { __state = 22; }
                }
                21 => { __state = 2; }
                22 => { unsafe { (*p).e_edit = 1 as u8 }; __state = 24; }
                23 => { __state = 3; }
                24 => { unsafe { (*p).delta = 0 }; __state = 25; }
                25 => {
                    rc =
                        json_lookup_step(p, 0 as u32,
                            unsafe { z_path.offset(1 as isize) }, 0 as u32);
                    __state = 26;
                }
                26 => {
                    if rc >= 4294967291u32 {
                        __state = 27;
                    } else { __state = 16; }
                }
                27 => {
                    if rc == 4294967294u32 {
                        __state = 29;
                    } else { __state = 30; }
                }
                28 => { __state = 3; }
                29 => { __state = 16; }
                30 => {
                    json_bad_path_error(ctx, z_path, rc as i32);
                    __state = 28;
                }
                31 => { json_parse_free(p); __state = 32; }
                32 => { return; }
                33 => { __state = 2; }
                34 => { __state = 3; }
                35 => { return; }
                _ => {}
            }
        }
    }
}
extern "C" fn json_append_separator(p: *mut JsonString) -> () {
    let mut c: i8 = 0 as i8;
    if unsafe { (*p).n_used } == 0 as u64 { return; }
    c =
        unsafe {
            *unsafe {
                    (*p).z_buf.add((unsafe { (*p).n_used } - 1 as u64) as usize)
                }
        };
    if c as i32 == '[' as i32 || c as i32 == '{' as i32 { return; }
    json_append_char(p, ',' as i32 as i8);
}
extern "C" fn json_append_sql_value(p: *mut JsonString,
    p_value_1: *mut sqlite3_value) -> () {
    unsafe {
        '__s42:
            {
            match unsafe { sqlite3_value_type(p_value_1) } {
                5 => {
                    {
                        json_append_raw_nz(p,
                            c"null".as_ptr() as *mut i8 as *const i8, 4 as u32);
                        break '__s42;
                    }
                    {
                        unsafe {
                            json_printf(100, p,
                                c"%!0.17g".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_value_double(p_value_1) })
                        };
                        break '__s42;
                    }
                    {
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_value_1) } as *const i8;
                        let n: u32 =
                            unsafe { sqlite3_value_bytes(p_value_1) } as u32;
                        json_append_raw(p, z, n);
                        break '__s42;
                    }
                    {
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_value_1) } as *const i8;
                        let n: u32 =
                            unsafe { sqlite3_value_bytes(p_value_1) } as u32;
                        if unsafe { sqlite3_value_subtype(p_value_1) } == 74 as u32
                            {
                            json_append_raw(p, z, n);
                        } else { json_append_string(p, z, n); }
                        break '__s42;
                    }
                    {
                        let mut px: JsonParse = unsafe { core::mem::zeroed() };
                        unsafe {
                            memset(&raw mut px as *mut (), 0,
                                core::mem::size_of::<JsonParse>() as u64)
                        };
                        if unsafe { json_arg_is_jsonb(p_value_1, &mut px) } != 0 {
                            unsafe {
                                json_translate_blob_to_text(&mut px, 0 as u32, p)
                            };
                        } else if unsafe { (*p).e_err } as i32 == 0 {
                            unsafe {
                                sqlite3_result_error(unsafe { (*p).p_ctx },
                                    c"JSON cannot hold BLOB values".as_ptr() as *mut i8 as
                                        *const i8, -1)
                            };
                            unsafe { (*p).e_err = 8 as u8 };
                            json_string_reset(p);
                        }
                        break '__s42;
                    }
                }
                2 => {
                    {
                        unsafe {
                            json_printf(100, p,
                                c"%!0.17g".as_ptr() as *mut i8 as *const i8,
                                unsafe { sqlite3_value_double(p_value_1) })
                        };
                        break '__s42;
                    }
                    {
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_value_1) } as *const i8;
                        let n: u32 =
                            unsafe { sqlite3_value_bytes(p_value_1) } as u32;
                        json_append_raw(p, z, n);
                        break '__s42;
                    }
                    {
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_value_1) } as *const i8;
                        let n: u32 =
                            unsafe { sqlite3_value_bytes(p_value_1) } as u32;
                        if unsafe { sqlite3_value_subtype(p_value_1) } == 74 as u32
                            {
                            json_append_raw(p, z, n);
                        } else { json_append_string(p, z, n); }
                        break '__s42;
                    }
                    {
                        let mut px: JsonParse = unsafe { core::mem::zeroed() };
                        unsafe {
                            memset(&raw mut px as *mut (), 0,
                                core::mem::size_of::<JsonParse>() as u64)
                        };
                        if unsafe { json_arg_is_jsonb(p_value_1, &mut px) } != 0 {
                            unsafe {
                                json_translate_blob_to_text(&mut px, 0 as u32, p)
                            };
                        } else if unsafe { (*p).e_err } as i32 == 0 {
                            unsafe {
                                sqlite3_result_error(unsafe { (*p).p_ctx },
                                    c"JSON cannot hold BLOB values".as_ptr() as *mut i8 as
                                        *const i8, -1)
                            };
                            unsafe { (*p).e_err = 8 as u8 };
                            json_string_reset(p);
                        }
                        break '__s42;
                    }
                }
                1 => {
                    {
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_value_1) } as *const i8;
                        let n: u32 =
                            unsafe { sqlite3_value_bytes(p_value_1) } as u32;
                        json_append_raw(p, z, n);
                        break '__s42;
                    }
                    {
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_value_1) } as *const i8;
                        let n: u32 =
                            unsafe { sqlite3_value_bytes(p_value_1) } as u32;
                        if unsafe { sqlite3_value_subtype(p_value_1) } == 74 as u32
                            {
                            json_append_raw(p, z, n);
                        } else { json_append_string(p, z, n); }
                        break '__s42;
                    }
                    {
                        let mut px: JsonParse = unsafe { core::mem::zeroed() };
                        unsafe {
                            memset(&raw mut px as *mut (), 0,
                                core::mem::size_of::<JsonParse>() as u64)
                        };
                        if unsafe { json_arg_is_jsonb(p_value_1, &mut px) } != 0 {
                            unsafe {
                                json_translate_blob_to_text(&mut px, 0 as u32, p)
                            };
                        } else if unsafe { (*p).e_err } as i32 == 0 {
                            unsafe {
                                sqlite3_result_error(unsafe { (*p).p_ctx },
                                    c"JSON cannot hold BLOB values".as_ptr() as *mut i8 as
                                        *const i8, -1)
                            };
                            unsafe { (*p).e_err = 8 as u8 };
                            json_string_reset(p);
                        }
                        break '__s42;
                    }
                }
                3 => {
                    {
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_value_1) } as *const i8;
                        let n: u32 =
                            unsafe { sqlite3_value_bytes(p_value_1) } as u32;
                        if unsafe { sqlite3_value_subtype(p_value_1) } == 74 as u32
                            {
                            json_append_raw(p, z, n);
                        } else { json_append_string(p, z, n); }
                        break '__s42;
                    }
                    {
                        let mut px: JsonParse = unsafe { core::mem::zeroed() };
                        unsafe {
                            memset(&raw mut px as *mut (), 0,
                                core::mem::size_of::<JsonParse>() as u64)
                        };
                        if unsafe { json_arg_is_jsonb(p_value_1, &mut px) } != 0 {
                            unsafe {
                                json_translate_blob_to_text(&mut px, 0 as u32, p)
                            };
                        } else if unsafe { (*p).e_err } as i32 == 0 {
                            unsafe {
                                sqlite3_result_error(unsafe { (*p).p_ctx },
                                    c"JSON cannot hold BLOB values".as_ptr() as *mut i8 as
                                        *const i8, -1)
                            };
                            unsafe { (*p).e_err = 8 as u8 };
                            json_string_reset(p);
                        }
                        break '__s42;
                    }
                }
                _ => {
                    {
                        let mut px: JsonParse = unsafe { core::mem::zeroed() };
                        unsafe {
                            memset(&raw mut px as *mut (), 0,
                                core::mem::size_of::<JsonParse>() as u64)
                        };
                        if unsafe { json_arg_is_jsonb(p_value_1, &mut px) } != 0 {
                            unsafe {
                                json_translate_blob_to_text(&mut px, 0 as u32, p)
                            };
                        } else if unsafe { (*p).e_err } as i32 == 0 {
                            unsafe {
                                sqlite3_result_error(unsafe { (*p).p_ctx },
                                    c"JSON cannot hold BLOB values".as_ptr() as *mut i8 as
                                        *const i8, -1)
                            };
                            unsafe { (*p).e_err = 8 as u8 };
                            json_string_reset(p);
                        }
                        break '__s42;
                    }
                }
            }
        }
    }
}
extern "C" fn json_array_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut i: i32 = 0;
    let mut jx: JsonString = unsafe { core::mem::zeroed() };
    json_string_init(&mut jx, ctx);
    json_append_char(&mut jx, '[' as i32 as i8);
    {
        i = 0;
        '__b43: loop {
            if !(i < argc) { break '__b43; }
            '__c43: loop {
                json_append_separator(&mut jx);
                json_append_sql_value(&mut jx,
                    unsafe { *argv.offset(i as isize) });
                break '__c43;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    json_append_char(&mut jx, ']' as i32 as i8);
    json_return_string(&mut jx, core::ptr::null_mut(), core::ptr::null_mut());
    unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
}
extern "C" fn json_wrong_num_args(p_ctx_1: *mut sqlite3_context,
    z_func_name_1: *const i8) -> () {
    let z_msg: *mut i8 =
        unsafe {
            sqlite3_mprintf(c"json_%s() needs an odd number of arguments".as_ptr()
                        as *mut i8 as *const i8, z_func_name_1)
        };
    unsafe { sqlite3_result_error(p_ctx_1, z_msg as *const i8, -1) };
    unsafe { sqlite3_free(z_msg as *mut ()) };
}
extern "C" fn json_function_arg_to_blob(ctx: *mut sqlite3_context,
    p_arg_1: *mut sqlite3_value, p_parse_1: *mut JsonParse) -> i32 {
    unsafe {
        let e_type: i32 = unsafe { sqlite3_value_type(p_arg_1) };
        unsafe {
            memset(p_parse_1 as *mut (), 0,
                core::mem::size_of::<JsonParse>() as u64)
        };
        unsafe {
            (*p_parse_1).db = unsafe { sqlite3_context_db_handle(ctx) }
        };
        '__s44:
            {
            match e_type {
                4 => {
                    {
                        if (unsafe { json_arg_is_jsonb(p_arg_1, p_parse_1) } == 0)
                                    as i32 != 0 {
                            unsafe {
                                sqlite3_result_error(ctx,
                                    c"JSON cannot hold BLOB values".as_ptr() as *mut i8 as
                                        *const i8, -1)
                            };
                            return 1;
                        }
                        break '__s44;
                    }
                    {
                        let z_json: *const i8 =
                            unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                        let n_json: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                        if z_json == core::ptr::null() { return 1; }
                        if unsafe { sqlite3_value_subtype(p_arg_1) } == 74 as u32 {
                            unsafe { (*p_parse_1).z_json = z_json as *mut i8 };
                            unsafe { (*p_parse_1).n_json = n_json };
                            if json_convert_text_to_blob(p_parse_1, ctx) != 0 {
                                unsafe {
                                    sqlite3_result_error(ctx,
                                        c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
                                };
                                unsafe {
                                    sqlite3_db_free(unsafe { (*p_parse_1).db },
                                        unsafe { (*p_parse_1).a_blob } as *mut ())
                                };
                                unsafe {
                                    memset(p_parse_1 as *mut (), 0,
                                        core::mem::size_of::<JsonParse>() as u64)
                                };
                                return 1;
                            }
                        } else {
                            json_blob_append_node(p_parse_1, 10 as u8, n_json as u64,
                                z_json as *const ());
                        }
                        break '__s44;
                    }
                    {
                        if unsafe {
                                    sqlite3_is_na_n(unsafe { sqlite3_value_double(p_arg_1) })
                                } != 0 {
                            json_blob_append_node(p_parse_1, 0 as u8, 0 as u64,
                                core::ptr::null());
                        } else {
                            let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                            let z: *const i8 =
                                unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                            if z == core::ptr::null() { return 1; }
                            if unsafe { *z.offset(0 as isize) } as i32 == 'I' as i32 {
                                json_blob_append_node(p_parse_1, 5 as u8, 5 as u64,
                                    c"9e999".as_ptr() as *mut i8 as *const ());
                            } else if unsafe { *z.offset(0 as isize) } as i32 ==
                                        '-' as i32 &&
                                    unsafe { *z.offset(1 as isize) } as i32 == 'I' as i32 {
                                json_blob_append_node(p_parse_1, 5 as u8, 6 as u64,
                                    c"-9e999".as_ptr() as *mut i8 as *const ());
                            } else {
                                json_blob_append_node(p_parse_1, 5 as u8, n as u64,
                                    z as *const ());
                            }
                        }
                        break '__s44;
                    }
                    {
                        let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                        if z == core::ptr::null() { return 1; }
                        json_blob_append_node(p_parse_1, 3 as u8, n as u64,
                            z as *const ());
                        break '__s44;
                    }
                }
                3 => {
                    {
                        let z_json: *const i8 =
                            unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                        let n_json: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                        if z_json == core::ptr::null() { return 1; }
                        if unsafe { sqlite3_value_subtype(p_arg_1) } == 74 as u32 {
                            unsafe { (*p_parse_1).z_json = z_json as *mut i8 };
                            unsafe { (*p_parse_1).n_json = n_json };
                            if json_convert_text_to_blob(p_parse_1, ctx) != 0 {
                                unsafe {
                                    sqlite3_result_error(ctx,
                                        c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
                                };
                                unsafe {
                                    sqlite3_db_free(unsafe { (*p_parse_1).db },
                                        unsafe { (*p_parse_1).a_blob } as *mut ())
                                };
                                unsafe {
                                    memset(p_parse_1 as *mut (), 0,
                                        core::mem::size_of::<JsonParse>() as u64)
                                };
                                return 1;
                            }
                        } else {
                            json_blob_append_node(p_parse_1, 10 as u8, n_json as u64,
                                z_json as *const ());
                        }
                        break '__s44;
                    }
                    {
                        if unsafe {
                                    sqlite3_is_na_n(unsafe { sqlite3_value_double(p_arg_1) })
                                } != 0 {
                            json_blob_append_node(p_parse_1, 0 as u8, 0 as u64,
                                core::ptr::null());
                        } else {
                            let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                            let z: *const i8 =
                                unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                            if z == core::ptr::null() { return 1; }
                            if unsafe { *z.offset(0 as isize) } as i32 == 'I' as i32 {
                                json_blob_append_node(p_parse_1, 5 as u8, 5 as u64,
                                    c"9e999".as_ptr() as *mut i8 as *const ());
                            } else if unsafe { *z.offset(0 as isize) } as i32 ==
                                        '-' as i32 &&
                                    unsafe { *z.offset(1 as isize) } as i32 == 'I' as i32 {
                                json_blob_append_node(p_parse_1, 5 as u8, 6 as u64,
                                    c"-9e999".as_ptr() as *mut i8 as *const ());
                            } else {
                                json_blob_append_node(p_parse_1, 5 as u8, n as u64,
                                    z as *const ());
                            }
                        }
                        break '__s44;
                    }
                    {
                        let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                        if z == core::ptr::null() { return 1; }
                        json_blob_append_node(p_parse_1, 3 as u8, n as u64,
                            z as *const ());
                        break '__s44;
                    }
                }
                2 => {
                    {
                        if unsafe {
                                    sqlite3_is_na_n(unsafe { sqlite3_value_double(p_arg_1) })
                                } != 0 {
                            json_blob_append_node(p_parse_1, 0 as u8, 0 as u64,
                                core::ptr::null());
                        } else {
                            let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                            let z: *const i8 =
                                unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                            if z == core::ptr::null() { return 1; }
                            if unsafe { *z.offset(0 as isize) } as i32 == 'I' as i32 {
                                json_blob_append_node(p_parse_1, 5 as u8, 5 as u64,
                                    c"9e999".as_ptr() as *mut i8 as *const ());
                            } else if unsafe { *z.offset(0 as isize) } as i32 ==
                                        '-' as i32 &&
                                    unsafe { *z.offset(1 as isize) } as i32 == 'I' as i32 {
                                json_blob_append_node(p_parse_1, 5 as u8, 6 as u64,
                                    c"-9e999".as_ptr() as *mut i8 as *const ());
                            } else {
                                json_blob_append_node(p_parse_1, 5 as u8, n as u64,
                                    z as *const ());
                            }
                        }
                        break '__s44;
                    }
                    {
                        let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                        if z == core::ptr::null() { return 1; }
                        json_blob_append_node(p_parse_1, 3 as u8, n as u64,
                            z as *const ());
                        break '__s44;
                    }
                }
                1 => {
                    {
                        let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                        if z == core::ptr::null() { return 1; }
                        json_blob_append_node(p_parse_1, 3 as u8, n as u64,
                            z as *const ());
                        break '__s44;
                    }
                }
                _ => {
                    {
                        unsafe {
                            (*p_parse_1).a_blob = &raw mut a_null[0 as usize] as *mut u8
                        };
                        unsafe { (*p_parse_1).n_blob = 1 as u32 };
                        return 0;
                    }
                    {
                        if (unsafe { json_arg_is_jsonb(p_arg_1, p_parse_1) } == 0)
                                    as i32 != 0 {
                            unsafe {
                                sqlite3_result_error(ctx,
                                    c"JSON cannot hold BLOB values".as_ptr() as *mut i8 as
                                        *const i8, -1)
                            };
                            return 1;
                        }
                        break '__s44;
                    }
                    {
                        let z_json: *const i8 =
                            unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                        let n_json: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                        if z_json == core::ptr::null() { return 1; }
                        if unsafe { sqlite3_value_subtype(p_arg_1) } == 74 as u32 {
                            unsafe { (*p_parse_1).z_json = z_json as *mut i8 };
                            unsafe { (*p_parse_1).n_json = n_json };
                            if json_convert_text_to_blob(p_parse_1, ctx) != 0 {
                                unsafe {
                                    sqlite3_result_error(ctx,
                                        c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
                                };
                                unsafe {
                                    sqlite3_db_free(unsafe { (*p_parse_1).db },
                                        unsafe { (*p_parse_1).a_blob } as *mut ())
                                };
                                unsafe {
                                    memset(p_parse_1 as *mut (), 0,
                                        core::mem::size_of::<JsonParse>() as u64)
                                };
                                return 1;
                            }
                        } else {
                            json_blob_append_node(p_parse_1, 10 as u8, n_json as u64,
                                z_json as *const ());
                        }
                        break '__s44;
                    }
                    {
                        if unsafe {
                                    sqlite3_is_na_n(unsafe { sqlite3_value_double(p_arg_1) })
                                } != 0 {
                            json_blob_append_node(p_parse_1, 0 as u8, 0 as u64,
                                core::ptr::null());
                        } else {
                            let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                            let z: *const i8 =
                                unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                            if z == core::ptr::null() { return 1; }
                            if unsafe { *z.offset(0 as isize) } as i32 == 'I' as i32 {
                                json_blob_append_node(p_parse_1, 5 as u8, 5 as u64,
                                    c"9e999".as_ptr() as *mut i8 as *const ());
                            } else if unsafe { *z.offset(0 as isize) } as i32 ==
                                        '-' as i32 &&
                                    unsafe { *z.offset(1 as isize) } as i32 == 'I' as i32 {
                                json_blob_append_node(p_parse_1, 5 as u8, 6 as u64,
                                    c"-9e999".as_ptr() as *mut i8 as *const ());
                            } else {
                                json_blob_append_node(p_parse_1, 5 as u8, n as u64,
                                    z as *const ());
                            }
                        }
                        break '__s44;
                    }
                    {
                        let n: i32 = unsafe { sqlite3_value_bytes(p_arg_1) };
                        let z: *const i8 =
                            unsafe { sqlite3_value_text(p_arg_1) } as *const i8;
                        if z == core::ptr::null() { return 1; }
                        json_blob_append_node(p_parse_1, 3 as u8, n as u64,
                            z as *const ());
                        break '__s44;
                    }
                }
            }
        }
        if unsafe { (*p_parse_1).oom } != 0 {
            unsafe { sqlite3_result_error_nomem(ctx) };
            return 1;
        } else { return 0; }
    }
}
extern "C" fn json_insert_into_blob(ctx: *mut sqlite3_context, argc: i32,
    argv: *const *mut sqlite3_value, e_edit_1: i32) -> () {
    let mut i: i32 = 0;
    let mut rc: u32 = 0 as u32;
    let mut z_path: *const i8 = core::ptr::null();
    let mut flgs: i32 = 0;
    let mut p: *mut JsonParse = core::ptr::null_mut();
    let mut ax: JsonParse = unsafe { core::mem::zeroed() };
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s46:
            {
            match __state {
                0 => { __state = 3; }
                2 => { json_parse_free(p); __state = 48; }
                3 => { rc = 0 as u32; __state = 4; }
                4 => { z_path = core::ptr::null(); __state = 5; }
                5 => { __state = 6; }
                6 => { __state = 7; }
                7 => { __state = 8; }
                8 => { { let _ = 0; }; __state = 9; }
                9 => { flgs = if argc == 1 { 0 } else { 1 }; __state = 10; }
                10 => {
                    p =
                        unsafe {
                            json_parse_func_arg(ctx,
                                unsafe { *argv.offset(0 as isize) }, flgs as u32)
                        };
                    __state = 11;
                }
                11 => {
                    if p == core::ptr::null_mut() {
                        __state = 13;
                    } else { __state = 12; }
                }
                12 => { i = 1; __state = 15; }
                13 => { return; }
                14 => { unsafe { json_return_parse(ctx, p) }; __state = 45; }
                15 => {
                    if i < argc - 1 { __state = 16; } else { __state = 14; }
                }
                16 => {
                    if unsafe {
                                sqlite3_value_type(unsafe { *argv.offset(i as isize) })
                            } == 5 {
                        __state = 19;
                    } else { __state = 18; }
                }
                17 => { i += 2; __state = 15; }
                18 => {
                    z_path =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *const i8;
                    __state = 20;
                }
                19 => { __state = 17; }
                20 => {
                    if z_path == core::ptr::null() {
                        __state = 22;
                    } else { __state = 21; }
                }
                21 => {
                    if unsafe { *z_path.offset(0 as isize) } as i32 !=
                            '$' as i32 {
                        __state = 26;
                    } else { __state = 25; }
                }
                22 => {
                    unsafe { sqlite3_result_error_nomem(ctx) };
                    __state = 23;
                }
                23 => { json_parse_free(p); __state = 24; }
                24 => { return; }
                25 => {
                    if json_function_arg_to_blob(ctx,
                                unsafe { *argv.offset((i + 1) as isize) }, &mut ax) != 0 {
                        __state = 28;
                    } else { __state = 27; }
                }
                26 => { __state = 2; }
                27 => {
                    if unsafe { *z_path.offset(1 as isize) } as i32 == 0 {
                        __state = 32;
                    } else { __state = 33; }
                }
                28 => { json_parse_reset(&mut ax); __state = 29; }
                29 => { json_parse_free(p); __state = 30; }
                30 => { return; }
                31 => { json_parse_reset(&mut ax); __state = 41; }
                32 => {
                    if e_edit_1 == 2 || e_edit_1 == 4 {
                        __state = 35;
                    } else { __state = 34; }
                }
                33 => {
                    unsafe { (*p).e_edit = e_edit_1 as u8 };
                    __state = 36;
                }
                34 => { rc = 0 as u32; __state = 31; }
                35 => {
                    json_blob_edit(p, 0 as u32, unsafe { (*p).n_blob },
                        ax.a_blob as *const u8, ax.n_blob);
                    __state = 34;
                }
                36 => { unsafe { (*p).n_ins = ax.n_blob }; __state = 37; }
                37 => { unsafe { (*p).a_ins = ax.a_blob }; __state = 38; }
                38 => { unsafe { (*p).delta = 0 }; __state = 39; }
                39 => { unsafe { (*p).i_depth = 0 as u16 }; __state = 40; }
                40 => {
                    rc =
                        json_lookup_step(p, 0 as u32,
                            unsafe { z_path.offset(1 as isize) }, 0 as u32);
                    __state = 31;
                }
                41 => {
                    if rc == 4294967294u32 {
                        __state = 43;
                    } else { __state = 42; }
                }
                42 => {
                    if rc >= 4294967291u32 {
                        __state = 44;
                    } else { __state = 17; }
                }
                43 => { __state = 17; }
                44 => { __state = 2; }
                45 => { json_parse_free(p); __state = 46; }
                46 => { return; }
                47 => { __state = 2; }
                48 => {
                    json_bad_path_error(ctx, z_path, rc as i32);
                    __state = 49;
                }
                49 => { return; }
                _ => {}
            }
        }
    }
}
extern "C" fn json_set_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    unsafe {
        let flags: i32 = unsafe { sqlite3_user_data(ctx) } as i64 as i32;
        let e_ins_type: i32 = (flags & 12) >> 2;
        if argc < 1 { return; }
        { let _ = 0; };
        if argc & 1 == 0 {
            json_wrong_num_args(ctx, az_ins_type[e_ins_type as usize]);
            return;
        }
        json_insert_into_blob(ctx, argc, argv as *const *mut sqlite3_value,
            a_edit_type[e_ins_type as usize] as i32);
    }
}
extern "C" fn json_array_length_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p: *mut JsonParse = core::ptr::null_mut();
    let mut cnt: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i: u32 = 0 as u32;
    let mut e_err: u8 = 0 as u8;
    p =
        json_parse_func_arg(ctx, unsafe { *argv.offset(0 as isize) },
            0 as u32);
    if p == core::ptr::null_mut() { return; }
    if argc == 2 {
        let z_path: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) }
                as *const i8;
        if z_path == core::ptr::null() { json_parse_free(p); return; }
        i =
            json_lookup_step(p, 0 as u32,
                if unsafe { *z_path.offset(0 as isize) } as i32 == '$' as i32
                    {
                    unsafe { z_path.offset(1 as isize) }
                } else { c"@".as_ptr() as *mut i8 as *const i8 }, 0 as u32);
        if i >= 4294967291u32 {
            if i == 4294967294u32
                {} else { json_bad_path_error(ctx, z_path, i as i32); }
            e_err = 1 as u8;
            i = 0 as u32;
        }
    } else { i = 0 as u32; }
    if unsafe { *unsafe { (*p).a_blob.add(i as usize) } } as i32 & 15 == 11 {
        cnt = jsonb_array_count(p as *const JsonParse, i) as sqlite3_int64;
    }
    if (e_err == 0) as i32 != 0 { unsafe { sqlite3_result_int64(ctx, cnt) }; }
    json_parse_free(p);
}
extern "C" fn json_error_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut i_err_pos: i64 = 0 as i64;
    let mut s: JsonParse = unsafe { core::mem::zeroed() };
    { let _ = 0; };
    { let _ = argc; };
    unsafe {
        memset(&raw mut s as *mut (), 0,
            core::mem::size_of::<JsonParse>() as u64)
    };
    s.db = unsafe { sqlite3_context_db_handle(ctx) };
    if json_arg_is_jsonb(unsafe { *argv.offset(0 as isize) }, &mut s) != 0 {
        i_err_pos =
            jsonb_validity_check(&raw mut s as *const JsonParse, 0 as u32,
                    s.n_blob, 1 as u32) as i64;
    } else {
        s.z_json =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *mut i8;
        if s.z_json == core::ptr::null_mut() { return; }
        s.n_json =
            unsafe {
                sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
            };
        if json_convert_text_to_blob(&mut s, core::ptr::null_mut()) != 0 {
            if s.oom != 0 {
                i_err_pos = -1 as i64;
            } else {
                let mut k: u32 = 0 as u32;
                { let _ = 0; };
                {
                    k = 0 as u32;
                    '__b47: loop {
                        if !(k < s.i_err &&
                                        unsafe { *s.z_json.add(k as usize) } != 0) {
                            break '__b47;
                        }
                        '__c47: loop {
                            if unsafe { *s.z_json.add(k as usize) } as i32 & 192 != 128
                                {
                                {
                                    let __p = &mut i_err_pos;
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                            break '__c47;
                        }
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    }
                }
                { let __p = &mut i_err_pos; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    json_parse_reset(&mut s);
    if i_err_pos < 0 as i64 {
        unsafe { sqlite3_result_error_nomem(ctx) };
    } else { unsafe { sqlite3_result_int64(ctx, i_err_pos) }; }
}
extern "C" fn json_all_alphanum(z: *const i8, n: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b48: loop {
                if !(i < n &&
                                (unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z.offset(i as isize) } as u8 as
                                                                usize)
                                                } as i32 & 6 != 0 ||
                                    unsafe { *z.offset(i as isize) } as i32 == '_' as i32)) {
                    break '__b48;
                }
                '__c48: loop { break '__c48; }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return (i == n) as i32;
    }
}
extern "C" fn json_return_text_json_from_blob(ctx: *mut sqlite3_context,
    a_blob_1: *const u8, n_blob_1: u32) -> () {
    let mut x: JsonParse = unsafe { core::mem::zeroed() };
    let mut s: JsonString = unsafe { core::mem::zeroed() };
    if a_blob_1 == core::ptr::null() { return; }
    unsafe {
        memset(&raw mut x as *mut (), 0,
            core::mem::size_of::<JsonParse>() as u64)
    };
    x.a_blob = a_blob_1 as *mut u8;
    x.n_blob = n_blob_1;
    json_string_init(&mut s, ctx);
    json_translate_blob_to_text(&mut x, 0 as u32, &mut s);
    json_return_string(&mut s, core::ptr::null_mut(), core::ptr::null_mut());
}
extern "C" fn json_return_from_blob(p_parse_1: *const JsonParse, i: u32,
    p_ctx_1: *mut sqlite3_context, mut e_mode_1: i32) -> () {
    let mut n: u32 = 0 as u32;
    let mut sz: u32 = 0 as u32;
    let mut rc: i32 = 0;
    let mut db: *mut sqlite3 = core::ptr::null_mut();
    let mut i_res: sqlite3_int64 = 0 as sqlite3_int64;
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut b_neg: i32 = 0;
    let mut x: i8 = 0 as i8;
    let mut r: f64 = 0.0;
    let mut r__1: f64 = 0.0;
    let mut z__1: *mut i8 = core::ptr::null_mut();
    let mut i_in: u32 = 0 as u32;
    let mut i_out: u32 = 0 as u32;
    let mut z__2: *const i8 = core::ptr::null();
    let mut z_out: *mut i8 = core::ptr::null_mut();
    let mut n_out: u32 = 0 as u32;
    let mut c: i8 = 0 as i8;
    let mut v: u32 = 0 as u32;
    let mut sz_escape: u32 = 0 as u32;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s50:
            {
            match __state {
                0 => { __state = 5; }
                2 => {
                    z__1 =
                        unsafe {
                            sqlite3_db_str_n_dup(db,
                                unsafe {
                                        &raw mut *unsafe {
                                                    (*p_parse_1).a_blob.add((i + n) as usize)
                                                }
                                    } as *const i8, sz as i32 as u64)
                        };
                    __state = 80;
                }
                3 => {
                    unsafe { sqlite3_result_error_nomem(p_ctx_1) };
                    __state = 142;
                }
                4 => {
                    unsafe {
                        sqlite3_result_error(p_ctx_1,
                            c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
                    };
                    __state = 144;
                }
                5 => { __state = 6; }
                6 => {
                    db = unsafe { sqlite3_context_db_handle(p_ctx_1) };
                    __state = 7;
                }
                7 => { { let _ = 0; }; __state = 8; }
                8 => {
                    n = jsonb_payload_size(unsafe { &*p_parse_1 }, i, &mut sz);
                    __state = 9;
                }
                9 => {
                    if n == 0 as u32 { __state = 11; } else { __state = 10; }
                }
                10 => {
                    '__s51:
                        {
                        match unsafe {
                                        *unsafe { (*p_parse_1).a_blob.add(i as usize) }
                                    } as i32 & 15 {
                            0 => { __state = 14; }
                            1 => { __state = 15; }
                            2 => { __state = 16; }
                            4 => { __state = 17; }
                            3 => { __state = 18; }
                            6 => { __state = 19; }
                            5 => { __state = 20; }
                            10 => { __state = 21; }
                            7 => { __state = 22; }
                            9 => { __state = 23; }
                            8 => { __state = 24; }
                            11 => { __state = 25; }
                            12 => { __state = 26; }
                            _ => { __state = 27; }
                        }
                    }
                }
                11 => {
                    unsafe {
                        sqlite3_result_error(p_ctx_1,
                            c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
                    };
                    __state = 12;
                }
                12 => { return; }
                13 => { return; }
                14 => { if sz != 0 { __state = 31; } else { __state = 30; } }
                15 => { if sz != 0 { __state = 35; } else { __state = 34; } }
                16 => { if sz != 0 { __state = 39; } else { __state = 38; } }
                17 => { __state = 18; }
                18 => { i_res = 0 as sqlite3_int64; __state = 42; }
                19 => { __state = 20; }
                20 => { __state = 76; }
                21 => { __state = 22; }
                22 => {
                    unsafe {
                        sqlite3_result_text(p_ctx_1,
                            unsafe {
                                        &raw mut *unsafe {
                                                    (*p_parse_1).a_blob.add((i + n) as usize)
                                                }
                                    } as *mut i8 as *const i8, sz as i32,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    __state = 90;
                }
                23 => { __state = 24; }
                24 => { __state = 93; }
                25 => { __state = 26; }
                26 => {
                    if e_mode_1 == 0 { __state = 134; } else { __state = 133; }
                }
                27 => { __state = 4; }
                28 => { __state = 14; }
                29 => { __state = 15; }
                30 => {
                    unsafe { sqlite3_result_null(p_ctx_1) };
                    __state = 32;
                }
                31 => { __state = 4; }
                32 => { __state = 13; }
                33 => { __state = 16; }
                34 => {
                    unsafe { sqlite3_result_int(p_ctx_1, 1) };
                    __state = 36;
                }
                35 => { __state = 4; }
                36 => { __state = 13; }
                37 => { __state = 17; }
                38 => {
                    unsafe { sqlite3_result_int(p_ctx_1, 0) };
                    __state = 40;
                }
                39 => { __state = 4; }
                40 => { __state = 13; }
                41 => { __state = 74; }
                42 => { __state = 43; }
                43 => { b_neg = 0; __state = 44; }
                44 => { __state = 45; }
                45 => {
                    if sz == 0 as u32 { __state = 47; } else { __state = 46; }
                }
                46 => {
                    x =
                        unsafe {
                                *unsafe { (*p_parse_1).a_blob.add((i + n) as usize) }
                            } as i8;
                    __state = 48;
                }
                47 => { __state = 4; }
                48 => {
                    if x as i32 == '-' as i32 {
                        __state = 50;
                    } else { __state = 49; }
                }
                49 => {
                    z =
                        unsafe {
                            sqlite3_db_str_n_dup(db,
                                unsafe {
                                        &raw mut *unsafe {
                                                    (*p_parse_1).a_blob.add((i + n) as usize)
                                                }
                                    } as *const i8, sz as i32 as u64)
                        };
                    __state = 55;
                }
                50 => {
                    if sz < 2 as u32 { __state = 52; } else { __state = 51; }
                }
                51 => {
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                    __state = 53;
                }
                52 => { __state = 4; }
                53 => {
                    { let __p = &mut sz; let __t = *__p; *__p -= 1; __t };
                    __state = 54;
                }
                54 => { b_neg = 1; __state = 49; }
                55 => {
                    if z == core::ptr::null_mut() {
                        __state = 57;
                    } else { __state = 56; }
                }
                56 => {
                    rc =
                        unsafe {
                            sqlite3_dec_or_hex_to_i64(z as *const i8, &mut i_res)
                        };
                    __state = 58;
                }
                57 => { __state = 3; }
                58 => {
                    unsafe { sqlite3_db_free(db, z as *mut ()) };
                    __state = 59;
                }
                59 => { if rc == 0 { __state = 61; } else { __state = 62; } }
                60 => { __state = 13; }
                61 => {
                    if i_res < 0 as i64 { __state = 63; } else { __state = 64; }
                }
                62 => {
                    if rc == 3 && b_neg != 0 {
                        __state = 67;
                    } else { __state = 68; }
                }
                63 => { __state = 65; }
                64 => {
                    unsafe {
                        sqlite3_result_int64(p_ctx_1,
                            if b_neg != 0 { -i_res } else { i_res })
                    };
                    __state = 60;
                }
                65 => {
                    r =
                        unsafe { *(&raw mut i_res as *mut sqlite3_uint64) } as f64;
                    __state = 66;
                }
                66 => {
                    unsafe {
                        sqlite3_result_double(p_ctx_1,
                            if b_neg != 0 { -r } else { r })
                    };
                    __state = 60;
                }
                67 => {
                    unsafe {
                        sqlite3_result_int64(p_ctx_1,
                            -1 as i64 -
                                (4294967295u32 as i64 | (2147483647 as i64) << 32))
                    };
                    __state = 60;
                }
                68 => { if rc == 1 { __state = 69; } else { __state = 70; } }
                69 => { __state = 4; }
                70 => {
                    if b_neg != 0 { __state = 72; } else { __state = 71; }
                }
                71 => { __state = 2; }
                72 => {
                    { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                    __state = 73;
                }
                73 => {
                    { let __p = &mut sz; let __t = *__p; *__p += 1; __t };
                    __state = 71;
                }
                74 => { __state = 19; }
                75 => { __state = 88; }
                76 => { __state = 77; }
                77 => {
                    if sz == 0 as u32 { __state = 79; } else { __state = 78; }
                }
                78 => { __state = 2; }
                79 => { __state = 4; }
                80 => {
                    if z__1 == core::ptr::null_mut() {
                        __state = 82;
                    } else { __state = 81; }
                }
                81 => {
                    rc = unsafe { sqlite3_ato_f(z__1 as *const i8, &mut r__1) };
                    __state = 83;
                }
                82 => { __state = 3; }
                83 => {
                    unsafe { sqlite3_db_free(db, z__1 as *mut ()) };
                    __state = 84;
                }
                84 => { if rc <= 0 { __state = 86; } else { __state = 85; } }
                85 => {
                    unsafe { sqlite3_result_double(p_ctx_1, r__1) };
                    __state = 87;
                }
                86 => { __state = 4; }
                87 => { __state = 13; }
                88 => { __state = 21; }
                89 => { __state = 91; }
                90 => { __state = 13; }
                91 => { __state = 23; }
                92 => { __state = 131; }
                93 => { __state = 94; }
                94 => { __state = 95; }
                95 => { n_out = sz; __state = 96; }
                96 => {
                    z__2 =
                        unsafe {
                                &raw mut *unsafe {
                                            (*p_parse_1).a_blob.add((i + n) as usize)
                                        }
                            } as *const i8;
                    __state = 97;
                }
                97 => {
                    z_out =
                        unsafe {
                                sqlite3_db_malloc_raw(db, n_out as u64 + 1 as u64)
                            } as *mut i8;
                    __state = 98;
                }
                98 => {
                    if z_out == core::ptr::null_mut() {
                        __state = 100;
                    } else { __state = 99; }
                }
                99 => { i_in = { i_out = 0 as u32; i_out }; __state = 102; }
                100 => { __state = 3; }
                101 => { { let _ = 0; }; __state = 128; }
                102 => {
                    if i_in < sz { __state = 103; } else { __state = 101; }
                }
                103 => {
                    c = unsafe { *z__2.add(i_in as usize) } as i8;
                    __state = 105;
                }
                104 => {
                    { let __p = &mut i_in; let __t = *__p; *__p += 1; __t };
                    __state = 102;
                }
                105 => {
                    if c as i32 == '\\' as i32 {
                        __state = 106;
                    } else { __state = 107; }
                }
                106 => { __state = 108; }
                107 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = c
                    };
                    __state = 104;
                }
                108 => {
                    sz_escape =
                        json_unescape_one_char(unsafe { &*z__2.add(i_in as usize) },
                            sz - i_in, &mut v);
                    __state = 109;
                }
                109 => {
                    if v <= 127 as u32 {
                        __state = 111;
                    } else { __state = 112; }
                }
                110 => { i_in += sz_escape - 1 as u32; __state = 104; }
                111 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = v as i8
                    };
                    __state = 110;
                }
                112 => {
                    if v <= 2047 as u32 {
                        __state = 113;
                    } else { __state = 114; }
                }
                113 => { { let _ = 0; }; __state = 115; }
                114 => {
                    if v < 65536 as u32 {
                        __state = 117;
                    } else { __state = 118; }
                }
                115 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (192 as u32 | v >> 6) as i8
                    };
                    __state = 116;
                }
                116 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (128 as u32 | v & 63 as u32) as i8
                    };
                    __state = 110;
                }
                117 => { { let _ = 0; }; __state = 119; }
                118 => {
                    if v == 629145 as u32 {
                        __state = 122;
                    } else { __state = 123; }
                }
                119 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (224 as u32 | v >> 12) as i8
                    };
                    __state = 120;
                }
                120 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (128 as u32 | v >> 6 & 63 as u32) as i8
                    };
                    __state = 121;
                }
                121 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (128 as u32 | v & 63 as u32) as i8
                    };
                    __state = 110;
                }
                122 => { __state = 110; }
                123 => { { let _ = 0; }; __state = 124; }
                124 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (240 as u32 | v >> 18) as i8
                    };
                    __state = 125;
                }
                125 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (128 as u32 | v >> 12 & 63 as u32) as i8
                    };
                    __state = 126;
                }
                126 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (128 as u32 | v >> 6 & 63 as u32) as i8
                    };
                    __state = 127;
                }
                127 => {
                    unsafe {
                        *z_out.add({
                                            let __p = &mut i_out;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize) = (128 as u32 | v & 63 as u32) as i8
                    };
                    __state = 110;
                }
                128 => {
                    unsafe { *z_out.add(i_out as usize) = 0 as i8 };
                    __state = 129;
                }
                129 => {
                    unsafe {
                        sqlite3_result_text(p_ctx_1, z_out as *const i8,
                            i_out as i32,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(sqlite3_row_set_clear as *const ())
                                }))
                    };
                    __state = 130;
                }
                130 => { __state = 13; }
                131 => { __state = 25; }
                132 => { __state = 140; }
                133 => {
                    if e_mode_1 == 2 { __state = 138; } else { __state = 139; }
                }
                134 => {
                    if unsafe { sqlite3_user_data(p_ctx_1) } as i64 as i32 & 16
                            != 0 {
                        __state = 135;
                    } else { __state = 136; }
                }
                135 => { e_mode_1 = 2; __state = 133; }
                136 => { e_mode_1 = 1; __state = 133; }
                137 => { __state = 13; }
                138 => {
                    unsafe {
                        sqlite3_result_blob(p_ctx_1,
                            unsafe {
                                    &raw mut *unsafe { (*p_parse_1).a_blob.add(i as usize) }
                                } as *const (), (sz + n) as i32,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    __state = 137;
                }
                139 => {
                    json_return_text_json_from_blob(p_ctx_1,
                        unsafe {
                                &raw mut *unsafe { (*p_parse_1).a_blob.add(i as usize) }
                            } as *const u8, sz + n);
                    __state = 137;
                }
                140 => { __state = 27; }
                141 => { __state = 3; }
                142 => { return; }
                143 => { __state = 4; }
                144 => { return; }
                _ => {}
            }
        }
    }
}
extern "C" fn json_extract_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p: *mut JsonParse = core::ptr::null_mut();
    let mut flags: i32 = 0;
    let mut i: i32 = 0;
    let mut jx: JsonString = unsafe { core::mem::zeroed() };
    let mut z_path: *const i8 = core::ptr::null();
    let mut n_path: i32 = 0;
    let mut j: u32 = 0 as u32;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s53:
            {
            match __state {
                0 => { p = core::ptr::null_mut(); __state = 3; }
                2 => { json_string_reset(&mut jx); __state = 74; }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { if argc < 2 { __state = 8; } else { __state = 7; } }
                7 => {
                    p =
                        json_parse_func_arg(ctx,
                            unsafe { *argv.offset(0 as isize) }, 0 as u32);
                    __state = 9;
                }
                8 => { return; }
                9 => {
                    if p == core::ptr::null_mut() {
                        __state = 11;
                    } else { __state = 10; }
                }
                10 => {
                    flags = unsafe { sqlite3_user_data(ctx) } as i64 as i32;
                    __state = 12;
                }
                11 => { return; }
                12 => { json_string_init(&mut jx, ctx); __state = 13; }
                13 => { if argc > 2 { __state = 15; } else { __state = 14; } }
                14 => { i = 1; __state = 17; }
                15 => {
                    json_append_char(&mut jx, '[' as i32 as i8);
                    __state = 14;
                }
                16 => { if argc > 2 { __state = 70; } else { __state = 69; } }
                17 => { if i < argc { __state = 18; } else { __state = 16; } }
                18 => {
                    z_path =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *const i8;
                    __state = 20;
                }
                19 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 17;
                }
                20 => { __state = 21; }
                21 => { __state = 22; }
                22 => {
                    if z_path == core::ptr::null() {
                        __state = 24;
                    } else { __state = 23; }
                }
                23 => {
                    n_path = unsafe { sqlite3_strlen30(z_path) };
                    __state = 25;
                }
                24 => { __state = 2; }
                25 => {
                    if unsafe { *z_path.offset(0 as isize) } as i32 ==
                            '$' as i32 {
                        __state = 27;
                    } else { __state = 28; }
                }
                26 => {
                    if j < unsafe { (*p).n_blob } {
                        __state = 49;
                    } else { __state = 50; }
                }
                27 => {
                    j =
                        json_lookup_step(p, 0 as u32,
                            unsafe { z_path.offset(1 as isize) }, 0 as u32);
                    __state = 26;
                }
                28 => {
                    if flags & 3 != 0 { __state = 29; } else { __state = 30; }
                }
                29 => { json_string_init(&mut jx, ctx); __state = 31; }
                30 => { json_bad_path_error(ctx, z_path, 0); __state = 48; }
                31 => {
                    if unsafe {
                                sqlite3_value_type(unsafe { *argv.offset(i as isize) })
                            } == 1 {
                        __state = 33;
                    } else { __state = 34; }
                }
                32 => { json_string_terminate(&mut jx); __state = 46; }
                33 => {
                    json_append_raw_nz(&mut jx,
                        c"[".as_ptr() as *mut i8 as *const i8, 1 as u32);
                    __state = 35;
                }
                34 => {
                    if json_all_alphanum(z_path, n_path) != 0 {
                        __state = 39;
                    } else { __state = 40; }
                }
                35 => {
                    if unsafe { *z_path.offset(0 as isize) } as i32 ==
                            '-' as i32 {
                        __state = 37;
                    } else { __state = 36; }
                }
                36 => {
                    json_append_raw(&mut jx, z_path, n_path as u32);
                    __state = 38;
                }
                37 => {
                    json_append_raw_nz(&mut jx,
                        c"#".as_ptr() as *mut i8 as *const i8, 1 as u32);
                    __state = 36;
                }
                38 => {
                    json_append_raw_nz(&mut jx,
                        c"]".as_ptr() as *mut i8 as *const i8, 2 as u32);
                    __state = 32;
                }
                39 => {
                    json_append_raw_nz(&mut jx,
                        c".".as_ptr() as *mut i8 as *const i8, 1 as u32);
                    __state = 41;
                }
                40 => {
                    if unsafe { *z_path.offset(0 as isize) } as i32 ==
                                    '[' as i32 && n_path >= 3 &&
                            unsafe { *z_path.offset((n_path - 1) as isize) } as i32 ==
                                ']' as i32 {
                        __state = 42;
                    } else { __state = 43; }
                }
                41 => {
                    json_append_raw(&mut jx, z_path, n_path as u32);
                    __state = 32;
                }
                42 => {
                    json_append_raw(&mut jx, z_path, n_path as u32);
                    __state = 32;
                }
                43 => {
                    json_append_raw_nz(&mut jx,
                        c".\"".as_ptr() as *mut i8 as *const i8, 2 as u32);
                    __state = 44;
                }
                44 => {
                    json_append_raw(&mut jx, z_path, n_path as u32);
                    __state = 45;
                }
                45 => {
                    json_append_raw_nz(&mut jx,
                        c"\"".as_ptr() as *mut i8 as *const i8, 1 as u32);
                    __state = 32;
                }
                46 => {
                    j =
                        json_lookup_step(p, 0 as u32, jx.z_buf as *const i8,
                            0 as u32);
                    __state = 47;
                }
                47 => { json_string_reset(&mut jx); __state = 26; }
                48 => { __state = 2; }
                49 => {
                    if argc == 2 { __state = 51; } else { __state = 52; }
                }
                50 => {
                    if j == 4294967294u32 {
                        __state = 63;
                    } else { __state = 64; }
                }
                51 => {
                    if flags & 1 != 0 { __state = 53; } else { __state = 54; }
                }
                52 => { json_append_separator(&mut jx); __state = 62; }
                53 => { json_string_init(&mut jx, ctx); __state = 55; }
                54 => {
                    json_return_from_blob(p as *const JsonParse, j, ctx, 0);
                    __state = 60;
                }
                55 => {
                    json_translate_blob_to_text(p, j, &mut jx);
                    __state = 56;
                }
                56 => {
                    json_return_string(&mut jx, core::ptr::null_mut(),
                        core::ptr::null_mut());
                    __state = 57;
                }
                57 => { json_string_reset(&mut jx); __state = 58; }
                58 => { { let _ = 0; }; __state = 59; }
                59 => {
                    unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
                    __state = 19;
                }
                60 => {
                    if flags & (2 | 16) == 0 &&
                            unsafe { *unsafe { (*p).a_blob.add(j as usize) } } as i32 &
                                    15 >= 11 {
                        __state = 61;
                    } else { __state = 19; }
                }
                61 => {
                    unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
                    __state = 19;
                }
                62 => {
                    json_translate_blob_to_text(p, j, &mut jx);
                    __state = 19;
                }
                63 => {
                    if argc == 2 { __state = 65; } else { __state = 66; }
                }
                64 => {
                    json_bad_path_error(ctx, z_path, j as i32);
                    __state = 68;
                }
                65 => { __state = 2; }
                66 => { json_append_separator(&mut jx); __state = 67; }
                67 => {
                    json_append_raw_nz(&mut jx,
                        c"null".as_ptr() as *mut i8 as *const i8, 4 as u32);
                    __state = 19;
                }
                68 => { __state = 2; }
                69 => { __state = 2; }
                70 => {
                    json_append_char(&mut jx, ']' as i32 as i8);
                    __state = 71;
                }
                71 => {
                    json_return_string(&mut jx, core::ptr::null_mut(),
                        core::ptr::null_mut());
                    __state = 72;
                }
                72 => {
                    if flags & 16 == 0 { __state = 73; } else { __state = 69; }
                }
                73 => {
                    unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
                    __state = 69;
                }
                74 => { json_parse_free(p); __state = 75; }
                75 => { return; }
                _ => {}
            }
        }
    }
}
extern "C" fn json_object_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut i: i32 = 0;
    let mut jx: JsonString = unsafe { core::mem::zeroed() };
    let mut z: *const i8 = core::ptr::null();
    let mut n: u32 = 0 as u32;
    if argc & 1 != 0 {
        unsafe {
            sqlite3_result_error(ctx,
                c"json_object() requires an even number of arguments".as_ptr()
                        as *mut i8 as *const i8, -1)
        };
        return;
    }
    json_string_init(&mut jx, ctx);
    json_append_char(&mut jx, '{' as i32 as i8);
    {
        i = 0;
        '__b54: loop {
            if !(i < argc) { break '__b54; }
            '__c54: loop {
                if unsafe {
                            sqlite3_value_type(unsafe { *argv.offset(i as isize) })
                        } != 3 {
                    unsafe {
                        sqlite3_result_error(ctx,
                            c"json_object() labels must be TEXT".as_ptr() as *mut i8 as
                                *const i8, -1)
                    };
                    json_string_reset(&mut jx);
                    return;
                }
                json_append_separator(&mut jx);
                z =
                    unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                        } as *const i8;
                n =
                    unsafe {
                            sqlite3_value_bytes(unsafe { *argv.offset(i as isize) })
                        } as u32;
                json_append_string(&mut jx, z, n);
                json_append_char(&mut jx, ':' as i32 as i8);
                json_append_sql_value(&mut jx,
                    unsafe { *argv.offset((i + 1) as isize) });
                break '__c54;
            }
            i += 2;
        }
    }
    json_append_char(&mut jx, '}' as i32 as i8);
    json_return_string(&mut jx, core::ptr::null_mut(), core::ptr::null_mut());
    unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
}
extern "C" fn json_merge_patch(p_target_1: *mut JsonParse, i_target_1: u32,
    p_patch_1: *const JsonParse, i_patch_1: u32, i_depth_1: u32) -> i32 {
    let mut x: u8 = 0 as u8;
    let mut n: u32 = 0 as u32;
    let mut sz: u32 = 0 as u32;
    let mut i_t_cursor: u32 = 0 as u32;
    let mut i_t_start: u32 = 0 as u32;
    let mut i_t_end_be: u32 = 0 as u32;
    let mut i_t_end: u32 = 0 as u32;
    let mut e_t_label: u8 = 0 as u8;
    let mut i_t_label: u32 = 0 as u32;
    let mut n_t_label: u32 = 0 as u32;
    let mut sz_t_label: u32 = 0 as u32;
    let mut i_t_value: u32 = 0 as u32;
    let mut n_t_value: u32 = 0 as u32;
    let mut sz_t_value: u32 = 0 as u32;
    let mut i_p_cursor: u32 = 0 as u32;
    let mut i_p_end: u32 = 0 as u32;
    let mut e_p_label: u8 = 0 as u8;
    let mut i_p_label: u32 = 0 as u32;
    let mut n_p_label: u32 = 0 as u32;
    let mut sz_p_label: u32 = 0 as u32;
    let mut i_p_value: u32 = 0 as u32;
    let mut n_p_value: u32 = 0 as u32;
    let mut sz_p_value: u32 = 0 as u32;
    { let _ = 0; };
    { let _ = 0; };
    x =
        (unsafe { *unsafe { (*p_patch_1).a_blob.add(i_patch_1 as usize) } } as
                    i32 & 15) as u8;
    if x as i32 != 12 {
        let mut sz_patch: u32 = 0 as u32;
        let mut sz_target: u32 = 0 as u32;
        n = jsonb_payload_size(unsafe { &*p_patch_1 }, i_patch_1, &mut sz);
        sz_patch = n + sz;
        sz = 0 as u32;
        n = jsonb_payload_size(unsafe { &*p_target_1 }, i_target_1, &mut sz);
        sz_target = n + sz;
        json_blob_edit(p_target_1, i_target_1, sz_target,
            unsafe { unsafe { (*p_patch_1).a_blob.add(i_patch_1 as usize) } }
                as *const u8, sz_patch);
        return if unsafe { (*p_target_1).oom } != 0 { 3 } else { 0 };
    }
    x =
        (unsafe { *unsafe { (*p_target_1).a_blob.add(i_target_1 as usize) } }
                    as i32 & 15) as u8;
    if x as i32 != 12 {
        n = jsonb_payload_size(unsafe { &*p_target_1 }, i_target_1, &mut sz);
        json_blob_edit(p_target_1, i_target_1 + n, sz, core::ptr::null(),
            0 as u32);
        x =
            unsafe {
                *unsafe { (*p_target_1).a_blob.add(i_target_1 as usize) }
            };
        unsafe {
            *unsafe { (*p_target_1).a_blob.add(i_target_1 as usize) } =
                (x as i32 & 240 | 12) as u8
        };
    }
    n = jsonb_payload_size(unsafe { &*p_patch_1 }, i_patch_1, &mut sz);
    if n == 0 as u32 { return 2; }
    i_p_cursor = i_patch_1 + n;
    i_p_end = i_p_cursor + sz;
    n = jsonb_payload_size(unsafe { &*p_target_1 }, i_target_1, &mut sz);
    if n == 0 as u32 { return 1; }
    i_t_start = i_target_1 + n;
    i_t_end_be = i_t_start + sz;
    while i_p_cursor < i_p_end {
        i_p_label = i_p_cursor;
        e_p_label =
            (unsafe {
                            *unsafe { (*p_patch_1).a_blob.add(i_p_cursor as usize) }
                        } as i32 & 15) as u8;
        if (e_p_label as i32) < 7 || e_p_label as i32 > 10 { return 2; }
        n_p_label =
            jsonb_payload_size(unsafe { &*p_patch_1 }, i_p_cursor,
                &mut sz_p_label);
        if n_p_label == 0 as u32 { return 2; }
        i_p_value = i_p_cursor + n_p_label + sz_p_label;
        if i_p_value >= i_p_end { return 2; }
        n_p_value =
            jsonb_payload_size(unsafe { &*p_patch_1 }, i_p_value,
                &mut sz_p_value);
        if n_p_value == 0 as u32 { return 2; }
        i_p_cursor = i_p_value + n_p_value + sz_p_value;
        if i_p_cursor > i_p_end { return 2; }
        i_t_cursor = i_t_start;
        i_t_end = i_t_end_be + unsafe { (*p_target_1).delta } as u32;
        while i_t_cursor < i_t_end {
            let mut is_equal: i32 = 0;
            i_t_label = i_t_cursor;
            e_t_label =
                (unsafe {
                                *unsafe { (*p_target_1).a_blob.add(i_t_cursor as usize) }
                            } as i32 & 15) as u8;
            if (e_t_label as i32) < 7 || e_t_label as i32 > 10 { return 1; }
            n_t_label =
                jsonb_payload_size(unsafe { &*p_target_1 }, i_t_cursor,
                    &mut sz_t_label);
            if n_t_label == 0 as u32 { return 1; }
            i_t_value = i_t_label + n_t_label + sz_t_label;
            if i_t_value >= i_t_end { return 1; }
            n_t_value =
                jsonb_payload_size(unsafe { &*p_target_1 }, i_t_value,
                    &mut sz_t_value);
            if n_t_value == 0 as u32 { return 1; }
            if i_t_value + n_t_value + sz_t_value > i_t_end { return 1; }
            is_equal =
                json_label_compare(unsafe {
                            &raw mut *unsafe {
                                        (*p_patch_1).a_blob.add((i_p_label + n_p_label) as usize)
                                    }
                        } as *const i8, sz_p_label,
                    (e_p_label as i32 == 7 || e_p_label as i32 == 10) as i32,
                    unsafe {
                            &raw mut *unsafe {
                                        (*p_target_1).a_blob.add((i_t_label + n_t_label) as usize)
                                    }
                        } as *const i8, sz_t_label,
                    (e_t_label as i32 == 7 || e_t_label as i32 == 10) as i32);
            if is_equal != 0 { break; }
            i_t_cursor = i_t_value + n_t_value + sz_t_value;
        }
        x =
            (unsafe {
                            *unsafe { (*p_patch_1).a_blob.add(i_p_value as usize) }
                        } as i32 & 15) as u8;
        if i_t_cursor < i_t_end {
            if x as i32 == 0 {
                json_blob_edit(p_target_1, i_t_label,
                    n_t_label + sz_t_label + n_t_value + sz_t_value,
                    core::ptr::null(), 0 as u32);
                if unsafe { (*p_target_1).oom } != 0 { return 3; }
            } else {
                let mut rc: i32 = 0;
                let saved_delta: i32 = unsafe { (*p_target_1).delta };
                unsafe { (*p_target_1).delta = 0 };
                if i_depth_1 >= 1000 as u32 { return 4; }
                rc =
                    json_merge_patch(p_target_1, i_t_value, p_patch_1,
                        i_p_value, i_depth_1 + 1 as u32);
                if rc != 0 { return rc; }
                unsafe { (*p_target_1).delta += saved_delta };
            }
        } else if x as i32 > 0 {
            let sz_new: u32 = sz_p_label + n_p_label;
            if unsafe {
                                *unsafe { (*p_patch_1).a_blob.add(i_p_value as usize) }
                            } as i32 & 15 != 12 {
                json_blob_edit(p_target_1, i_t_end, 0 as u32,
                    core::ptr::null(), sz_p_value + n_p_value + sz_new);
                if unsafe { (*p_target_1).oom } != 0 { return 3; }
                unsafe {
                    memcpy(unsafe {
                                &raw mut *unsafe {
                                            (*p_target_1).a_blob.add(i_t_end as usize)
                                        }
                            } as *mut (),
                        unsafe {
                                &raw mut *unsafe {
                                            (*p_patch_1).a_blob.add(i_p_label as usize)
                                        }
                            } as *const (), sz_new as u64)
                };
                unsafe {
                    memcpy(unsafe {
                                &raw mut *unsafe {
                                            (*p_target_1).a_blob.add((i_t_end + sz_new) as usize)
                                        }
                            } as *mut (),
                        unsafe {
                                &raw mut *unsafe {
                                            (*p_patch_1).a_blob.add(i_p_value as usize)
                                        }
                            } as *const (), (sz_p_value + n_p_value) as u64)
                };
            } else {
                let mut rc: i32 = 0;
                let mut saved_delta_1: i32 = 0;
                json_blob_edit(p_target_1, i_t_end, 0 as u32,
                    core::ptr::null(), sz_new + 1 as u32);
                if unsafe { (*p_target_1).oom } != 0 { return 3; }
                unsafe {
                    memcpy(unsafe {
                                &raw mut *unsafe {
                                            (*p_target_1).a_blob.add(i_t_end as usize)
                                        }
                            } as *mut (),
                        unsafe {
                                &raw mut *unsafe {
                                            (*p_patch_1).a_blob.add(i_p_label as usize)
                                        }
                            } as *const (), sz_new as u64)
                };
                unsafe {
                    *unsafe {
                                (*p_target_1).a_blob.add((i_t_end + sz_new) as usize)
                            } = 0 as u8
                };
                saved_delta_1 = unsafe { (*p_target_1).delta };
                unsafe { (*p_target_1).delta = 0 };
                if i_depth_1 >= 1000 as u32 { return 4; }
                rc =
                    json_merge_patch(p_target_1, i_t_end + sz_new, p_patch_1,
                        i_p_value, i_depth_1 + 1 as u32);
                if rc != 0 { return rc; }
                unsafe { (*p_target_1).delta += saved_delta_1 };
            }
        }
    }
    if unsafe { (*p_target_1).delta } != 0 {
        json_after_edit_size_adjust(p_target_1, i_target_1);
    }
    return if unsafe { (*p_target_1).oom } != 0 { 3 } else { 0 };
}
extern "C" fn json_patch_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p_target: *mut JsonParse = core::ptr::null_mut();
    let mut p_patch: *mut JsonParse = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = argc; };
    { let _ = 0; };
    p_target =
        json_parse_func_arg(ctx, unsafe { *argv.offset(0 as isize) },
            1 as u32);
    if p_target == core::ptr::null_mut() { return; }
    p_patch =
        json_parse_func_arg(ctx, unsafe { *argv.offset(1 as isize) },
            0 as u32);
    if !(p_patch).is_null() {
        rc =
            json_merge_patch(p_target, 0 as u32, p_patch as *const JsonParse,
                0 as u32, 0 as u32);
        if rc == 0 {
            json_return_parse(ctx, p_target);
        } else if rc == 3 {
            unsafe { sqlite3_result_error_nomem(ctx) };
        } else if rc == 4 {
            unsafe {
                sqlite3_result_error(ctx,
                    c"JSON nested too deep".as_ptr() as *mut i8 as *const i8,
                    -1)
            };
        } else {
            unsafe {
                sqlite3_result_error(ctx,
                    c"malformed JSON".as_ptr() as *mut i8 as *const i8, -1)
            };
        }
        json_parse_free(p_patch);
    }
    json_parse_free(p_target);
}
#[repr(C)]
#[derive(Copy, Clone)]
struct JsonPretty {
    p_parse: *mut JsonParse,
    p_out: *mut JsonString,
    z_indent: *const i8,
    sz_indent: u32,
    n_indent: u32,
}
extern "C" fn json_pretty_indent(p_pretty_1: &JsonPretty) -> () {
    let mut jj: u32 = 0 as u32;
    {
        jj = 0 as u32;
        '__b57: loop {
            if !(jj < (*p_pretty_1).n_indent) { break '__b57; }
            '__c57: loop {
                json_append_raw((*p_pretty_1).p_out, (*p_pretty_1).z_indent,
                    (*p_pretty_1).sz_indent);
                break '__c57;
            }
            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn json_translate_blob_to_pretty_text(p_pretty_1: *mut JsonPretty,
    mut i: u32) -> u32 {
    let mut sz: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    let mut j: u32 = 0 as u32;
    let mut i_end: u32 = 0 as u32;
    let p_parse: *mut JsonParse = unsafe { (*p_pretty_1).p_parse };
    let p_out: *mut JsonString = unsafe { (*p_pretty_1).p_out };
    n = jsonb_payload_size(unsafe { &*p_parse }, i, &mut sz);
    if n == 0 as u32 {
        unsafe { (*p_out).e_err |= 2 as u8 };
        return unsafe { (*p_parse).n_blob } + 1 as u32;
    }
    '__s58:
        {
        match unsafe { *unsafe { (*p_parse).a_blob.add(i as usize) } } as i32
                & 15 {
            11 => {
                {
                    j = i + n;
                    i_end = j + sz;
                    json_append_char(p_out, '[' as i32 as i8);
                    if j < i_end {
                        json_append_char(p_out, '\n' as i32 as i8);
                        {
                            let __p = unsafe { &mut (*p_pretty_1).n_indent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        if unsafe { (*p_pretty_1).n_indent } >= 1000 as u32 {
                            json_string_too_deep(p_out);
                        }
                        while unsafe { (*p_out).e_err } as i32 == 0 {
                            json_pretty_indent(unsafe { &*p_pretty_1 });
                            j = json_translate_blob_to_pretty_text(p_pretty_1, j);
                            if j >= i_end { break; }
                            json_append_raw_nz(p_out,
                                c",\n".as_ptr() as *mut i8 as *const i8, 2 as u32);
                        }
                        json_append_char(p_out, '\n' as i32 as i8);
                        {
                            let __p = unsafe { &mut (*p_pretty_1).n_indent };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        json_pretty_indent(unsafe { &*p_pretty_1 });
                    }
                    json_append_char(p_out, ']' as i32 as i8);
                    i = i_end;
                    break '__s58;
                }
                {
                    j = i + n;
                    i_end = j + sz;
                    json_append_char(p_out, '{' as i32 as i8);
                    if j < i_end {
                        json_append_char(p_out, '\n' as i32 as i8);
                        {
                            let __p = unsafe { &mut (*p_pretty_1).n_indent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        if unsafe { (*p_pretty_1).n_indent } >= 1000 as u32 {
                            json_string_too_deep(p_out);
                        }
                        unsafe {
                            (*p_parse).i_depth =
                                unsafe { (*p_pretty_1).n_indent } as u16
                        };
                        while unsafe { (*p_out).e_err } as i32 == 0 {
                            json_pretty_indent(unsafe { &*p_pretty_1 });
                            j = json_translate_blob_to_text(p_parse, j, p_out);
                            if j > i_end {
                                unsafe { (*p_out).e_err |= 2 as u8 };
                                break;
                            }
                            json_append_raw_nz(p_out,
                                c": ".as_ptr() as *mut i8 as *const i8, 2 as u32);
                            j = json_translate_blob_to_pretty_text(p_pretty_1, j);
                            if j >= i_end { break; }
                            json_append_raw_nz(p_out,
                                c",\n".as_ptr() as *mut i8 as *const i8, 2 as u32);
                        }
                        json_append_char(p_out, '\n' as i32 as i8);
                        {
                            let __p = unsafe { &mut (*p_pretty_1).n_indent };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        json_pretty_indent(unsafe { &*p_pretty_1 });
                    }
                    json_append_char(p_out, '}' as i32 as i8);
                    i = i_end;
                    break '__s58;
                }
                {
                    i = json_translate_blob_to_text(p_parse, i, p_out);
                    break '__s58;
                }
            }
            12 => {
                {
                    j = i + n;
                    i_end = j + sz;
                    json_append_char(p_out, '{' as i32 as i8);
                    if j < i_end {
                        json_append_char(p_out, '\n' as i32 as i8);
                        {
                            let __p = unsafe { &mut (*p_pretty_1).n_indent };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        if unsafe { (*p_pretty_1).n_indent } >= 1000 as u32 {
                            json_string_too_deep(p_out);
                        }
                        unsafe {
                            (*p_parse).i_depth =
                                unsafe { (*p_pretty_1).n_indent } as u16
                        };
                        while unsafe { (*p_out).e_err } as i32 == 0 {
                            json_pretty_indent(unsafe { &*p_pretty_1 });
                            j = json_translate_blob_to_text(p_parse, j, p_out);
                            if j > i_end {
                                unsafe { (*p_out).e_err |= 2 as u8 };
                                break;
                            }
                            json_append_raw_nz(p_out,
                                c": ".as_ptr() as *mut i8 as *const i8, 2 as u32);
                            j = json_translate_blob_to_pretty_text(p_pretty_1, j);
                            if j >= i_end { break; }
                            json_append_raw_nz(p_out,
                                c",\n".as_ptr() as *mut i8 as *const i8, 2 as u32);
                        }
                        json_append_char(p_out, '\n' as i32 as i8);
                        {
                            let __p = unsafe { &mut (*p_pretty_1).n_indent };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        json_pretty_indent(unsafe { &*p_pretty_1 });
                    }
                    json_append_char(p_out, '}' as i32 as i8);
                    i = i_end;
                    break '__s58;
                }
                {
                    i = json_translate_blob_to_text(p_parse, i, p_out);
                    break '__s58;
                }
            }
            _ => {
                {
                    i = json_translate_blob_to_text(p_parse, i, p_out);
                    break '__s58;
                }
            }
        }
    }
    return i;
}
extern "C" fn json_pretty_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut s: JsonString = unsafe { core::mem::zeroed() };
    let mut x: JsonPretty = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut x as *mut (), 0,
            core::mem::size_of::<JsonPretty>() as u64)
    };
    x.p_parse =
        json_parse_func_arg(ctx, unsafe { *argv.offset(0 as isize) },
            0 as u32);
    if x.p_parse == core::ptr::null_mut() { return; }
    x.p_out = &mut s;
    json_string_init(&mut s, ctx);
    if argc == 1 ||
            {
                    x.z_indent =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                            } as *const i8;
                    x.z_indent
                } == core::ptr::null() {
        x.z_indent = c"    ".as_ptr() as *mut i8 as *const i8;
        x.sz_indent = 4 as u32;
    } else { x.sz_indent = unsafe { strlen(x.z_indent) } as u32; }
    json_translate_blob_to_pretty_text(&mut x, 0 as u32);
    json_return_string(&mut s, core::ptr::null_mut(), core::ptr::null_mut());
    json_parse_free(x.p_parse);
}
extern "C" fn json_quote_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut jx: JsonString = unsafe { core::mem::zeroed() };
    { let _ = argc; };
    json_string_init(&mut jx, ctx);
    json_append_sql_value(&mut jx, unsafe { *argv.offset(0 as isize) });
    json_return_string(&mut jx, core::ptr::null_mut(), core::ptr::null_mut());
    unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
}
extern "C" fn json_replace_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    if argc < 1 { return; }
    if argc & 1 == 0 {
        json_wrong_num_args(ctx, c"replace".as_ptr() as *mut i8 as *const i8);
        return;
    }
    json_insert_into_blob(ctx, argc, argv as *const *mut sqlite3_value, 2);
}
static mut jsonb_type: [*const i8; 17] =
    [c"null".as_ptr() as *const i8, c"true".as_ptr() as *const i8,
            c"false".as_ptr() as *const i8, c"integer".as_ptr() as *const i8,
            c"integer".as_ptr() as *const i8, c"real".as_ptr() as *const i8,
            c"real".as_ptr() as *const i8, c"text".as_ptr() as *const i8,
            c"text".as_ptr() as *const i8, c"text".as_ptr() as *const i8,
            c"text".as_ptr() as *const i8, c"array".as_ptr() as *const i8,
            c"object".as_ptr() as *const i8, c"".as_ptr() as *const i8,
            c"".as_ptr() as *const i8, c"".as_ptr() as *const i8,
            c"".as_ptr() as *const i8];
extern "C" fn json_type_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    unsafe {
        let mut p: *mut JsonParse = core::ptr::null_mut();
        '__b61: loop {
            '__c61: loop {
                let mut z_path: *const i8 = core::ptr::null();
                let mut i: u32 = 0 as u32;
                p =
                    json_parse_func_arg(ctx,
                        unsafe { *argv.offset(0 as isize) }, 0 as u32);
                if p == core::ptr::null_mut() { return; }
                if argc == 2 {
                    z_path =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                            } as *const i8;
                    if z_path == core::ptr::null() { break '__b61; }
                    if unsafe { *z_path.offset(0 as isize) } as i32 !=
                            '$' as i32 {
                        json_bad_path_error(ctx, z_path, 0);
                        break '__b61;
                    }
                    i =
                        json_lookup_step(p, 0 as u32,
                            unsafe { z_path.offset(1 as isize) }, 0 as u32);
                    if i >= 4294967291u32 {
                        if i == 4294967294u32
                            {} else { json_bad_path_error(ctx, z_path, i as i32); }
                        break '__b61;
                    }
                } else { i = 0 as u32; }
                unsafe {
                    sqlite3_result_text(ctx,
                        jsonb_type[(unsafe {
                                            *unsafe { (*p).a_blob.add(i as usize) }
                                        } as i32 & 15) as usize], -1, None)
                };
                break '__c61;
            }
            if !(false) { break '__b61; }
        }
        json_parse_free(p);
    }
}
extern "C" fn json_valid_func(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut p: *mut JsonParse = core::ptr::null_mut();
    let mut flags: u8 = 1 as u8;
    let mut res: u8 = 0 as u8;
    if argc == 2 {
        let f: i64 =
            unsafe {
                sqlite3_value_int64(unsafe { *argv.offset(1 as isize) })
            };
        if f < 1 as i64 || f > 15 as i64 {
            unsafe {
                sqlite3_result_error(ctx,
                    c"FLAGS parameter to json_valid() must be between 1 and 15".as_ptr()
                            as *mut i8 as *const i8, -1)
            };
            return;
        }
        flags = (f & 15 as i64) as u8;
    }
    '__s62:
        {
        match unsafe {
                sqlite3_value_type(unsafe { *argv.offset(0 as isize) })
            } {
            5 => {
                { return; }
                {
                    let mut py: JsonParse = unsafe { core::mem::zeroed() };
                    unsafe {
                        memset(&raw mut py as *mut (), 0,
                            core::mem::size_of::<JsonParse>() as u64)
                    };
                    if json_arg_is_jsonb(unsafe { *argv.offset(0 as isize) },
                                &mut py) != 0 {
                        if flags as i32 & 4 != 0 {
                            res = 1 as u8;
                        } else if flags as i32 & 8 != 0 {
                            res =
                                (0 as u32 ==
                                        jsonb_validity_check(&raw mut py as *const JsonParse,
                                            0 as u32, py.n_blob, 1 as u32)) as u8;
                        }
                        break '__s62;
                    }
                }
                {
                    let mut px: JsonParse = unsafe { core::mem::zeroed() };
                    if flags as i32 & 3 == 0 { break '__s62; }
                    unsafe {
                        memset(&raw mut px as *mut (), 0,
                            core::mem::size_of::<JsonParse>() as u64)
                    };
                    p =
                        json_parse_func_arg(ctx,
                            unsafe { *argv.offset(0 as isize) }, 2 as u32);
                    if !(p).is_null() {
                        if unsafe { (*p).oom } != 0 {
                            unsafe { sqlite3_result_error_nomem(ctx) };
                        } else if unsafe { (*p).n_err } != 0
                            {} else if flags as i32 & 2 != 0 ||
                                unsafe { (*p).has_nonstd } as i32 == 0 {
                            res = 1 as u8;
                        }
                        json_parse_free(p);
                    } else { unsafe { sqlite3_result_error_nomem(ctx) }; }
                    break '__s62;
                }
            }
            4 => {
                {
                    let mut py: JsonParse = unsafe { core::mem::zeroed() };
                    unsafe {
                        memset(&raw mut py as *mut (), 0,
                            core::mem::size_of::<JsonParse>() as u64)
                    };
                    if json_arg_is_jsonb(unsafe { *argv.offset(0 as isize) },
                                &mut py) != 0 {
                        if flags as i32 & 4 != 0 {
                            res = 1 as u8;
                        } else if flags as i32 & 8 != 0 {
                            res =
                                (0 as u32 ==
                                        jsonb_validity_check(&raw mut py as *const JsonParse,
                                            0 as u32, py.n_blob, 1 as u32)) as u8;
                        }
                        break '__s62;
                    }
                }
                {
                    let mut px: JsonParse = unsafe { core::mem::zeroed() };
                    if flags as i32 & 3 == 0 { break '__s62; }
                    unsafe {
                        memset(&raw mut px as *mut (), 0,
                            core::mem::size_of::<JsonParse>() as u64)
                    };
                    p =
                        json_parse_func_arg(ctx,
                            unsafe { *argv.offset(0 as isize) }, 2 as u32);
                    if !(p).is_null() {
                        if unsafe { (*p).oom } != 0 {
                            unsafe { sqlite3_result_error_nomem(ctx) };
                        } else if unsafe { (*p).n_err } != 0
                            {} else if flags as i32 & 2 != 0 ||
                                unsafe { (*p).has_nonstd } as i32 == 0 {
                            res = 1 as u8;
                        }
                        json_parse_free(p);
                    } else { unsafe { sqlite3_result_error_nomem(ctx) }; }
                    break '__s62;
                }
            }
            _ => {
                {
                    let mut px: JsonParse = unsafe { core::mem::zeroed() };
                    if flags as i32 & 3 == 0 { break '__s62; }
                    unsafe {
                        memset(&raw mut px as *mut (), 0,
                            core::mem::size_of::<JsonParse>() as u64)
                    };
                    p =
                        json_parse_func_arg(ctx,
                            unsafe { *argv.offset(0 as isize) }, 2 as u32);
                    if !(p).is_null() {
                        if unsafe { (*p).oom } != 0 {
                            unsafe { sqlite3_result_error_nomem(ctx) };
                        } else if unsafe { (*p).n_err } != 0
                            {} else if flags as i32 & 2 != 0 ||
                                unsafe { (*p).has_nonstd } as i32 == 0 {
                            res = 1 as u8;
                        }
                        json_parse_free(p);
                    } else { unsafe { sqlite3_result_error_nomem(ctx) }; }
                    break '__s62;
                }
            }
        }
    }
    unsafe { sqlite3_result_int(ctx, res as i32) };
}
extern "C" fn json_array_step(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    unsafe {
        let mut p_str: *mut JsonString = core::ptr::null_mut();
        { let _ = argc; };
        p_str =
            unsafe {
                    sqlite3_aggregate_context(ctx,
                        core::mem::size_of::<JsonString>() as i32)
                } as *mut JsonString;
        if !(p_str).is_null() {
            if unsafe { (*p_str).z_buf } == core::ptr::null_mut() {
                json_string_init(p_str, ctx);
                json_append_char(p_str, '[' as i32 as i8);
            } else if unsafe { (*p_str).n_used } > 1 as u64 {
                json_append_char(p_str, ',' as i32 as i8);
            }
            unsafe { (*p_str).p_ctx = ctx };
            json_append_sql_value(p_str, unsafe { *argv.offset(0 as isize) });
        }
    }
}
extern "C" fn json_array_compute(ctx: *mut sqlite3_context, is_final_1: i32)
    -> () {
    unsafe {
        let mut p_str: *mut JsonString = core::ptr::null_mut();
        let flags: i32 = unsafe { sqlite3_user_data(ctx) } as i64 as i32;
        p_str =
            unsafe { sqlite3_aggregate_context(ctx, 0) } as *mut JsonString;
        if !(p_str).is_null() {
            unsafe { (*p_str).p_ctx = ctx };
            json_append_raw_nz(p_str, c"]".as_ptr() as *mut i8 as *const i8,
                2 as u32);
            json_string_trim_one_char(unsafe { &mut *p_str });
            if unsafe { (*p_str).e_err } != 0 {
                json_return_string(p_str, core::ptr::null_mut(),
                    core::ptr::null_mut());
                return;
            } else if flags & 16 != 0 {
                json_return_string_as_blob(unsafe { &*p_str });
                if is_final_1 != 0 {
                    if (unsafe { (*p_str).b_static } == 0) as i32 != 0 {
                        unsafe {
                            sqlite3_rc_str_unref(unsafe { (*p_str).z_buf } as *mut ())
                        };
                    }
                } else { json_string_trim_one_char(unsafe { &mut *p_str }); }
                return;
            } else if is_final_1 != 0 {
                unsafe {
                    sqlite3_result_text(ctx,
                        unsafe { (*p_str).z_buf } as *const i8,
                        unsafe { (*p_str).n_used } as i32,
                        Some(if unsafe { (*p_str).b_static } != 0 {
                                unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }
                            } else { sqlite3_rc_str_unref }))
                };
                unsafe { (*p_str).b_static = 1 as u8 };
            } else {
                unsafe {
                    sqlite3_result_text(ctx,
                        unsafe { (*p_str).z_buf } as *const i8,
                        unsafe { (*p_str).n_used } as i32,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
                json_string_trim_one_char(unsafe { &mut *p_str });
            }
        } else if flags & 16 != 0 {
            unsafe {
                sqlite3_result_blob(ctx, &raw const empty_array as *const (),
                    1, None)
            };
        } else {
            unsafe {
                sqlite3_result_text(ctx,
                    c"[]".as_ptr() as *mut i8 as *const i8, 2, None)
            };
        }
        unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
    }
}
extern "C" fn json_array_final(ctx: *mut sqlite3_context) -> () {
    json_array_compute(ctx, 1);
}
extern "C" fn json_array_value(ctx: *mut sqlite3_context) -> () {
    json_array_compute(ctx, 0);
}
extern "C" fn json_group_inverse(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    let mut i: u32 = 0 as u32;
    let mut in_str: i32 = 0;
    let mut n_nest: i32 = 0;
    let mut z: *mut i8 = core::ptr::null_mut();
    let mut c: i8 = 0 as i8;
    let mut p_str: *mut JsonString = core::ptr::null_mut();
    { let _ = argc; };
    { let _ = argv; };
    p_str = unsafe { sqlite3_aggregate_context(ctx, 0) } as *mut JsonString;
    if (p_str).is_null() as i32 != 0 { return; }
    z = unsafe { (*p_str).z_buf };
    {
        i = 1 as u32;
        '__b63: loop {
            if !((i as u64) < unsafe { (*p_str).n_used } &&
                            ({ c = unsafe { *z.add(i as usize) }; c } as i32 !=
                                        ',' as i32 || in_str != 0 || n_nest != 0)) {
                break '__b63;
            }
            '__c63: loop {
                if c as i32 == '\"' as i32 {
                    in_str = (in_str == 0) as i32 as i32;
                } else if c as i32 == '\\' as i32 {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                } else if (in_str == 0) as i32 != 0 {
                    if c as i32 == '{' as i32 || c as i32 == '[' as i32 {
                        { let __p = &mut n_nest; let __t = *__p; *__p += 1; __t };
                    }
                    if c as i32 == '}' as i32 || c as i32 == ']' as i32 {
                        { let __p = &mut n_nest; let __t = *__p; *__p -= 1; __t };
                    }
                }
                break '__c63;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if (i as u64) < unsafe { (*p_str).n_used } {
        unsafe { (*p_str).n_used -= i as u64 };
        unsafe {
            memmove(unsafe { &raw mut *z.offset(1 as isize) } as *mut (),
                unsafe { &raw mut *z.add((i + 1 as u32) as usize) } as
                    *const (), unsafe { (*p_str).n_used } as u64 - 1 as u64)
        };
        unsafe { *z.add(unsafe { (*p_str).n_used } as usize) = 0 as i8 };
    } else { unsafe { (*p_str).n_used = 1 as u64 }; }
}
extern "C" fn json_object_step(ctx: *mut sqlite3_context, argc: i32,
    argv: *mut *mut sqlite3_value) -> () {
    unsafe {
        let mut p_str: *mut JsonString = core::ptr::null_mut();
        let mut z: *const i8 = core::ptr::null();
        let mut n: u32 = 0 as u32;
        { let _ = argc; };
        p_str =
            unsafe {
                    sqlite3_aggregate_context(ctx,
                        core::mem::size_of::<JsonString>() as i32)
                } as *mut JsonString;
        if !(p_str).is_null() {
            z =
                unsafe {
                        sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                    } as *const i8;
            n = unsafe { sqlite3_strlen30(z) } as u32;
            if unsafe { (*p_str).z_buf } == core::ptr::null_mut() {
                json_string_init(p_str, ctx);
                json_append_char(p_str, '{' as i32 as i8);
            } else if unsafe { (*p_str).n_used } > 1 as u64 {
                json_append_char(p_str, ',' as i32 as i8);
            }
            unsafe { (*p_str).p_ctx = ctx };
            if z != core::ptr::null() {
                json_append_string(p_str, z, n);
                json_append_char(p_str, ':' as i32 as i8);
                json_append_sql_value(p_str,
                    unsafe { *argv.offset(1 as isize) });
            } else {
                unsafe {
                    *unsafe { (*p_str).z_buf.offset(0 as isize) } =
                        '@' as i32 as i8
                };
                json_append_raw_nz(p_str,
                    c"@".as_ptr() as *mut i8 as *const i8, 1 as u32);
            }
        }
    }
}
extern "C" fn json_object_compute(ctx: *mut sqlite3_context, is_final_1: i32)
    -> () {
    unsafe {
        let mut p_str: *mut JsonString = core::ptr::null_mut();
        let flags: i32 = unsafe { sqlite3_user_data(ctx) } as i64 as i32;
        p_str =
            unsafe { sqlite3_aggregate_context(ctx, 0) } as *mut JsonString;
        if !(p_str).is_null() {
            let p_og_str: *mut JsonString = p_str;
            let mut tmp_str: JsonString = unsafe { core::mem::zeroed() };
            json_append_raw_nz(p_og_str,
                c"}".as_ptr() as *mut i8 as *const i8, 2 as u32);
            json_string_trim_one_char(unsafe { &mut *p_og_str });
            unsafe { (*p_str).p_ctx = ctx };
            if unsafe { (*p_str).e_err } != 0 {
                json_return_string(p_str, core::ptr::null_mut(),
                    core::ptr::null_mut());
                return;
            }
            if unsafe { *unsafe { (*p_str).z_buf.offset(0 as isize) } } as i32
                    != '{' as i32 {
                let mut i: u64 = 0 as u64;
                let mut j: u64 = 0 as u64;
                let mut in_str: i32 = 0;
                if (is_final_1 == 0) as i32 != 0 {
                    json_string_init(&mut tmp_str, ctx);
                    json_append_raw_nz(&mut tmp_str,
                        unsafe { (*p_str).z_buf } as *const i8,
                        (unsafe { (*p_str).n_used } + 1 as u64) as u32);
                    p_str = &mut tmp_str;
                    if unsafe { (*p_str).e_err } != 0 {
                        json_return_string(p_str, core::ptr::null_mut(),
                            core::ptr::null_mut());
                        return;
                    }
                    json_string_trim_one_char(unsafe { &mut *p_str });
                }
                unsafe {
                    *unsafe { (*p_str).z_buf.offset(0 as isize) } =
                        '{' as i32 as i8
                };
                {
                    i = { j = 1 as u64; j };
                    '__b64: loop {
                        if !(i < unsafe { (*p_str).n_used }) { break '__b64; }
                        '__c64: loop {
                            let c: i8 =
                                unsafe { *unsafe { (*p_str).z_buf.add(i as usize) } };
                            if c as i32 == '\"' as i32 {
                                in_str = (in_str == 0) as i32 as i32;
                                unsafe {
                                    *unsafe {
                                                (*p_str).z_buf.add({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as usize)
                                            } = '\"' as i32 as i8
                                };
                            } else if c as i32 == '\\' as i32 {
                                unsafe {
                                    *unsafe {
                                                (*p_str).z_buf.add({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as usize)
                                            } = '\\' as i32 as i8
                                };
                                unsafe {
                                    *unsafe {
                                                (*p_str).z_buf.add({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as usize)
                                            } =
                                        unsafe {
                                            *unsafe {
                                                    (*p_str).z_buf.add({ let __p = &mut i; *__p += 1; *__p } as
                                                            usize)
                                                }
                                        }
                                };
                            } else if c as i32 == '@' as i32 &&
                                    (in_str == 0) as i32 != 0 {
                                { let _ = 0; };
                                if unsafe {
                                                *unsafe { (*p_str).z_buf.add((i + 1 as u64) as usize) }
                                            } as i32 == ',' as i32 {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                } else if unsafe {
                                                *unsafe { (*p_str).z_buf.add((j - 1 as u64) as usize) }
                                            } as i32 == ',' as i32 {
                                    { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                }
                            } else {
                                unsafe {
                                    *unsafe {
                                                (*p_str).z_buf.add({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as usize)
                                            } = c
                                };
                            }
                            break '__c64;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe {
                    *unsafe { (*p_str).z_buf.add(j as usize) } = 0 as i8
                };
                unsafe { (*p_str).n_used = j };
            }
            if flags & 16 != 0 {
                json_return_string_as_blob(unsafe { &*p_str });
                if is_final_1 != 0 {
                    if (unsafe { (*p_str).b_static } == 0) as i32 != 0 {
                        unsafe {
                            sqlite3_rc_str_unref(unsafe { (*p_str).z_buf } as *mut ())
                        };
                    }
                } else {
                    json_string_trim_one_char(unsafe { &mut *p_og_str });
                }
            } else if is_final_1 != 0 {
                unsafe {
                    sqlite3_result_text(ctx,
                        unsafe { (*p_str).z_buf } as *const i8,
                        unsafe { (*p_str).n_used } as i32,
                        Some(if unsafe { (*p_str).b_static } != 0 {
                                unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }
                            } else { sqlite3_rc_str_unref }))
                };
                unsafe { (*p_str).b_static = 1 as u8 };
            } else {
                unsafe {
                    sqlite3_result_text(ctx,
                        unsafe { (*p_str).z_buf } as *const i8,
                        unsafe { (*p_str).n_used } as i32,
                        Some(unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ())
                                            -> ()>(-1 as isize as *const ())
                            }))
                };
                json_string_trim_one_char(unsafe { &mut *p_og_str });
            }
            if p_str != p_og_str { json_string_reset(p_str); }
        } else if flags & 16 != 0 {
            unsafe {
                sqlite3_result_blob(ctx,
                    &raw const empty_object_2 as *const (), 1, None)
            };
        } else {
            unsafe {
                sqlite3_result_text(ctx,
                    c"{}".as_ptr() as *mut i8 as *const i8, 2, None)
            };
        }
        unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
    }
}
extern "C" fn json_object_final(ctx: *mut sqlite3_context) -> () {
    json_object_compute(ctx, 1);
}
extern "C" fn json_object_value(ctx: *mut sqlite3_context) -> () {
    json_object_compute(ctx, 0);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_register_json_functions() -> () {
    unsafe {
        unsafe {
            sqlite3_insert_builtin_funcs(&raw mut a_json_func[0 as usize] as
                    *mut FuncDef,
                (core::mem::size_of::<[FuncDef; 36]>() as u64 /
                        core::mem::size_of::<FuncDef>() as u64) as i32)
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct JsonEachConnection {
    base: sqlite3_vtab,
    db: *mut sqlite3,
    e_mode: u8,
    b_recursive: u8,
}
extern "C" fn json_each_connect(db: *mut sqlite3, p_aux_1: *mut (), argc: i32,
    argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    let mut p_new: *mut JsonEachConnection = core::ptr::null_mut();
    let mut rc: i32 = 0;
    { let _ = pz_err_1; };
    { let _ = argv; };
    { let _ = argc; };
    { let _ = p_aux_1; };
    rc =
        unsafe {
            sqlite3_declare_vtab(db,
                c"CREATE TABLE x(key,value,type,atom,id,parent,fullkey,path,json HIDDEN,root HIDDEN)".as_ptr()
                        as *mut i8 as *const i8)
        };
    if rc == 0 {
        p_new =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<JsonEachConnection>() as u64)
                } as *mut JsonEachConnection;
        unsafe { *pp_vtab_1 = p_new as *mut sqlite3_vtab };
        if p_new == core::ptr::null_mut() { return 7; }
        unsafe { sqlite3_vtab_config(db, 2) };
        unsafe { (*p_new).db = db };
        unsafe {
            (*p_new).e_mode =
                if unsafe {
                                    *unsafe { (*argv.offset(0 as isize)).offset(4 as isize) }
                                } as i32 == 'b' as i32 {
                        2
                    } else { 1 } as u8
        };
        unsafe {
            (*p_new).b_recursive =
                (unsafe {
                                *unsafe {
                                        (*argv.offset(0 as
                                                        isize)).offset((4 + unsafe { (*p_new).e_mode } as i32) as
                                                isize)
                                    }
                            } as i32 == 't' as i32) as u8
        };
    }
    return rc;
}
extern "C" fn json_each_best_index(tab: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let mut i: i32 = 0;
    let mut a_idx: [i32; 2] = [0; 2];
    let mut unusable_mask: i32 = 0;
    let mut idx_mask: i32 = 0;
    let mut p_constraint: *const sqlite3_index_constraint = core::ptr::null();
    { let _ = 0; };
    { let _ = tab; };
    a_idx[0 as usize] = { a_idx[1 as usize] = -1; a_idx[1 as usize] };
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const sqlite3_index_constraint;
    {
        i = 0;
        '__b65: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b65;
            }
            '__c65: loop {
                let mut i_col: i32 = 0;
                let mut i_mask: i32 = 0;
                if (unsafe { (*p_constraint).i_column } as i32) < 8 {
                    break '__c65;
                }
                i_col = unsafe { (*p_constraint).i_column } - 8;
                { let _ = 0; };
                i_mask = 1 << i_col;
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    unusable_mask |= i_mask;
                } else if unsafe { (*p_constraint).op } as i32 == 2 {
                    a_idx[i_col as usize] = i;
                    idx_mask |= i_mask;
                }
                break '__c65;
            }
            {
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut p_constraint;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    if unsafe { (*p_idx_info_1).n_order_by } > 0 &&
                unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).i_column
                    } < 0 &&
            unsafe {
                        (*unsafe {
                                    (*p_idx_info_1).a_order_by.offset(0 as isize)
                                }).desc
                    } as i32 == 0 {
        unsafe { (*p_idx_info_1).order_by_consumed = 1 };
    }
    if unusable_mask & !idx_mask != 0 { return 19; }
    if a_idx[0 as usize] < 0 {
        unsafe { (*p_idx_info_1).idx_num = 0 };
    } else {
        unsafe { (*p_idx_info_1).estimated_cost = 1.0 };
        i = a_idx[0 as usize];
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                        }).argv_index = 1
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                        }).omit = 1 as u8
        };
        if a_idx[1 as usize] < 0 {
            unsafe { (*p_idx_info_1).idx_num = 1 };
        } else {
            i = a_idx[1 as usize];
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                            }).argv_index = 2
            };
            unsafe {
                (*unsafe {
                                (*p_idx_info_1).a_constraint_usage.offset(i as isize)
                            }).omit = 1 as u8
            };
            unsafe { (*p_idx_info_1).idx_num = 3 };
        }
    }
    return 0;
}
extern "C" fn json_each_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    let p: *const JsonEachConnection =
        p_vtab_1 as *mut JsonEachConnection as *const JsonEachConnection;
    unsafe { sqlite3_db_free(unsafe { (*p).db }, p_vtab_1 as *mut ()) };
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct JsonEachCursor {
    base: sqlite3_vtab_cursor,
    i_rowid: u32,
    i: u32,
    i_end: u32,
    n_root: u32,
    e_type: u8,
    b_recursive: u8,
    e_mode: u8,
    n_parent: u32,
    n_parent_alloc: u32,
    a_parent: *mut JsonParent,
    db: *mut sqlite3,
    path: JsonString,
    s_parse: JsonParse,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct JsonParent {
    i_head: u32,
    i_value: u32,
    i_end: u32,
    n_path: u32,
    i_key: i64,
}
extern "C" fn json_each_open(p: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    let p_vtab: *const JsonEachConnection =
        p as *mut JsonEachConnection as *const JsonEachConnection;
    let mut p_cur: *mut JsonEachCursor = core::ptr::null_mut();
    { let _ = p; };
    p_cur =
        unsafe {
                sqlite3_db_malloc_zero(unsafe { (*p_vtab).db },
                    core::mem::size_of::<JsonEachCursor>() as u64)
            } as *mut JsonEachCursor;
    if p_cur == core::ptr::null_mut() { return 7; }
    unsafe { (*p_cur).db = unsafe { (*p_vtab).db } };
    unsafe { (*p_cur).e_mode = unsafe { (*p_vtab).e_mode } };
    unsafe { (*p_cur).b_recursive = unsafe { (*p_vtab).b_recursive } };
    json_string_zero(unsafe { &mut (*p_cur).path });
    unsafe { *pp_cursor_1 = unsafe { &mut (*p_cur).base } };
    return 0;
}
extern "C" fn json_each_cursor_reset(p: &mut JsonEachCursor) -> () {
    unsafe {
        json_parse_reset(&mut (*p).s_parse);
        json_string_reset(&mut (*p).path);
        unsafe { sqlite3_db_free((*p).db, (*p).a_parent as *mut ()) };
        (*p).i_rowid = 0 as u32;
        (*p).i = 0 as u32;
        (*p).a_parent = core::ptr::null_mut();
        (*p).n_parent = 0 as u32;
        (*p).n_parent_alloc = 0 as u32;
        (*p).i_end = 0 as u32;
        (*p).e_type = 0 as u8;
    }
}
extern "C" fn json_each_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
    json_each_cursor_reset(unsafe { &mut *p });
    unsafe { sqlite3_db_free(unsafe { (*p).db }, cur as *mut ()) };
    return 0;
}
extern "C" fn json_each_malformed_input(cur: *mut sqlite3_vtab_cursor)
    -> i32 {
    unsafe {
        unsafe {
            sqlite3_free(unsafe { (*unsafe { (*cur).p_vtab }).z_err_msg } as
                    *mut ())
        };
        unsafe {
            (*unsafe { (*cur).p_vtab }).z_err_msg =
                unsafe {
                    sqlite3_mprintf(c"malformed JSON".as_ptr() as *mut i8 as
                            *const i8)
                }
        };
        json_each_cursor_reset(unsafe { &mut *(cur as *mut JsonEachCursor) });
        return if !(unsafe {
                                (*unsafe { (*cur).p_vtab }).z_err_msg
                            }).is_null() {
                1
            } else { 7 };
    }
}
extern "C" fn json_each_filter(cur: *mut sqlite3_vtab_cursor, idx_num_1: i32,
    idx_str_1: *const i8, argc: i32, argv: *mut *mut sqlite3_value) -> i32 {
    unsafe {
        let p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
        let mut z_root: *const i8 = core::ptr::null();
        let mut i: u32 = 0 as u32;
        let mut n: u32 = 0 as u32;
        let mut sz: u32 = 0 as u32;
        { let _ = idx_str_1; };
        { let _ = argc; };
        json_each_cursor_reset(unsafe { &mut *p });
        if idx_num_1 == 0 { return 0; }
        unsafe {
            memset(unsafe { &raw mut (*p).s_parse } as *mut (), 0,
                core::mem::size_of::<JsonParse>() as u64)
        };
        unsafe { (*p).s_parse.n_jp_ref = 1 as u32 };
        unsafe { (*p).s_parse.db = unsafe { (*p).db } };
        if json_arg_is_jsonb(unsafe { *argv.offset(0 as isize) },
                    unsafe { &mut (*p).s_parse }) != 0
            {} else {
            unsafe {
                (*p).s_parse.z_json =
                    unsafe {
                            sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                        } as *mut i8
            };
            unsafe {
                (*p).s_parse.n_json =
                    unsafe {
                        sqlite3_value_bytes(unsafe { *argv.offset(0 as isize) })
                    }
            };
            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                unsafe {
                    (*p).i =
                        { unsafe { (*p).i_end = 0 as u32 }; unsafe { (*p).i_end } }
                };
                return 0;
            }
            if json_convert_text_to_blob(unsafe { &mut (*p).s_parse },
                        core::ptr::null_mut()) != 0 {
                if unsafe { (*p).s_parse.oom } != 0 { return 7; }
                return json_each_malformed_input(cur);
            }
        }
        if idx_num_1 == 3 {
            z_root =
                unsafe {
                        sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                    } as *const i8;
            if z_root == core::ptr::null() { return 0; }
            if unsafe { *z_root.offset(0 as isize) } as i32 != '$' as i32 {
                unsafe {
                    sqlite3_free(unsafe {
                                (*unsafe { (*cur).p_vtab }).z_err_msg
                            } as *mut ())
                };
                unsafe {
                    (*unsafe { (*cur).p_vtab }).z_err_msg =
                        json_bad_path_error(core::ptr::null_mut(), z_root, 0)
                };
                json_each_cursor_reset(unsafe { &mut *p });
                return if !(unsafe {
                                        (*unsafe { (*cur).p_vtab }).z_err_msg
                                    }).is_null() {
                        1
                    } else { 7 };
            }
            unsafe {
                (*p).n_root = unsafe { sqlite3_strlen30(z_root) } as u32
            };
            if unsafe { *z_root.offset(1 as isize) } as i32 == 0 {
                i = { unsafe { (*p).i = 0 as u32 }; unsafe { (*p).i } };
                unsafe { (*p).e_type = 0 as u8 };
            } else {
                i =
                    json_lookup_step(unsafe { &mut (*p).s_parse }, 0 as u32,
                        unsafe { z_root.offset(1 as isize) }, 0 as u32);
                if i >= 4294967291u32 {
                    if i == 4294967294u32 {
                        unsafe { (*p).i = 0 as u32 };
                        unsafe { (*p).e_type = 0 as u8 };
                        unsafe { (*p).i_end = 0 as u32 };
                        return 0;
                    }
                    unsafe {
                        sqlite3_free(unsafe {
                                    (*unsafe { (*cur).p_vtab }).z_err_msg
                                } as *mut ())
                    };
                    unsafe {
                        (*unsafe { (*cur).p_vtab }).z_err_msg =
                            json_bad_path_error(core::ptr::null_mut(), z_root, 0)
                    };
                    json_each_cursor_reset(unsafe { &mut *p });
                    return if !(unsafe {
                                            (*unsafe { (*cur).p_vtab }).z_err_msg
                                        }).is_null() {
                            1
                        } else { 7 };
                }
                if unsafe { (*p).s_parse.i_label } != 0 {
                    unsafe { (*p).i = unsafe { (*p).s_parse.i_label } };
                    unsafe { (*p).e_type = 12 as u8 };
                } else {
                    unsafe { (*p).i = i };
                    unsafe { (*p).e_type = 11 as u8 };
                }
            }
            json_append_raw(unsafe { &mut (*p).path }, z_root,
                unsafe { (*p).n_root });
        } else {
            i = { unsafe { (*p).i = 0 as u32 }; unsafe { (*p).i } };
            unsafe { (*p).e_type = 0 as u8 };
            unsafe { (*p).n_root = 1 as u32 };
            json_append_raw(unsafe { &mut (*p).path },
                c"$".as_ptr() as *mut i8 as *const i8, 1 as u32);
        }
        unsafe { (*p).n_parent = 0 as u32 };
        n = jsonb_payload_size(unsafe { &(*p).s_parse }, i, &mut sz);
        unsafe { (*p).i_end = i + n + sz };
        if unsafe { *unsafe { (*p).s_parse.a_blob.add(i as usize) } } as i32 &
                        15 >= 11 && (unsafe { (*p).b_recursive } == 0) as i32 != 0 {
            unsafe { (*p).i = i + n };
            unsafe {
                (*p).e_type =
                    (unsafe { *unsafe { (*p).s_parse.a_blob.add(i as usize) } }
                                as i32 & 15) as u8
            };
            unsafe {
                (*p).a_parent =
                    unsafe {
                            sqlite3_db_malloc_zero(unsafe { (*p).db },
                                core::mem::size_of::<JsonParent>() as u64)
                        } as *mut JsonParent
            };
            if unsafe { (*p).a_parent } == core::ptr::null_mut() { return 7; }
            unsafe { (*p).n_parent = 1 as u32 };
            unsafe { (*p).n_parent_alloc = 1 as u32 };
            unsafe {
                (*unsafe { (*p).a_parent.offset(0 as isize) }).i_key =
                    0 as i64
            };
            unsafe {
                (*unsafe { (*p).a_parent.offset(0 as isize) }).i_end =
                    unsafe { (*p).i_end }
            };
            unsafe {
                (*unsafe { (*p).a_parent.offset(0 as isize) }).i_head =
                    unsafe { (*p).i }
            };
            unsafe {
                (*unsafe { (*p).a_parent.offset(0 as isize) }).i_value = i
            };
        }
        return 0;
    }
}
extern "C" fn json_skip_label(p: &mut JsonEachCursor) -> i32 {
    unsafe {
        if (*p).e_type as i32 == 12 {
            let mut sz: u32 = 0 as u32;
            let n: u32 = jsonb_payload_size(&(*p).s_parse, (*p).i, &mut sz);
            return ((*p).i + n + sz) as i32;
        } else { return (*p).i as i32; }
    }
}
extern "C" fn json_append_path_name(p: &mut JsonEachCursor) -> () {
    unsafe {
        unsafe {
            { let _ = 0; };
            { let _ = 0; };
            if (*p).e_type as i32 == 11 {
                unsafe {
                    json_printf(30, &mut (*p).path,
                        c"[%lld]".as_ptr() as *mut i8 as *const i8,
                        unsafe {
                            (*(*p).a_parent.add(((*p).n_parent - 1 as u32) as
                                            usize)).i_key
                        })
                };
            } else {
                let mut n: u32 = 0 as u32;
                let mut sz: u32 = 0 as u32;
                let mut k: u32 = 0 as u32;
                let mut i: u32 = 0 as u32;
                let mut z: *const i8 = core::ptr::null();
                let mut need_quote: i32 = 0;
                n = jsonb_payload_size(&(*p).s_parse, (*p).i, &mut sz);
                k = (*p).i + n;
                z =
                    unsafe { &raw mut *(*p).s_parse.a_blob.add(k as usize) } as
                        *const i8;
                if sz == 0 as u32 ||
                        (unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                                            usize)
                                            } as i32 & 2 == 0) as i32 != 0 {
                    need_quote = 1;
                } else {
                    {
                        i = 0 as u32;
                        '__b66: loop {
                            if !(i < sz) { break '__b66; }
                            '__c66: loop {
                                if (unsafe {
                                                            *(sqlite3_ctype_map.as_ptr() as
                                                                        *const u8).add(unsafe { *z.add(i as usize) } as u8 as usize)
                                                        } as i32 & 6 == 0) as i32 != 0 {
                                    need_quote = 1;
                                    break '__b66;
                                }
                                break '__c66;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                if need_quote != 0 {
                    unsafe {
                        json_printf((sz + 4 as u32) as i32, &mut (*p).path,
                            c".\"%.*s\"".as_ptr() as *mut i8 as *const i8, sz, z)
                    };
                } else {
                    unsafe {
                        json_printf((sz + 2 as u32) as i32, &mut (*p).path,
                            c".%.*s".as_ptr() as *mut i8 as *const i8, sz, z)
                    };
                }
            }
        }
    }
}
extern "C" fn json_each_next(cur: *mut sqlite3_vtab_cursor) -> i32 {
    unsafe {
        let p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
        let mut rc: i32 = 0;
        if unsafe { (*p).b_recursive } != 0 {
            let mut x: u8 = 0 as u8;
            let mut level_change: u8 = 0 as u8;
            let mut n: u32 = 0 as u32;
            let mut sz: u32 = 0 as u32;
            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
            x =
                (unsafe { *unsafe { (*p).s_parse.a_blob.add(i as usize) } } as
                            i32 & 15) as u8;
            n = jsonb_payload_size(unsafe { &(*p).s_parse }, i, &mut sz);
            if n == 0 as u32 { return json_each_malformed_input(cur); }
            if x as i32 == 12 || x as i32 == 11 {
                let mut p_parent: *mut JsonParent = core::ptr::null_mut();
                if unsafe { (*p).n_parent } >= unsafe { (*p).n_parent_alloc }
                    {
                    let mut p_new: *mut JsonParent = core::ptr::null_mut();
                    let mut n_new: u64 = 0 as u64;
                    n_new =
                        (unsafe { (*p).n_parent_alloc } * 2 as u32 + 3 as u32) as
                            u64;
                    p_new =
                        unsafe {
                                sqlite3_db_realloc(unsafe { (*p).db },
                                    unsafe { (*p).a_parent } as *mut (),
                                    core::mem::size_of::<JsonParent>() as u64 * n_new)
                            } as *mut JsonParent;
                    if p_new == core::ptr::null_mut() { return 7; }
                    unsafe { (*p).n_parent_alloc = n_new as u32 };
                    unsafe { (*p).a_parent = p_new };
                }
                level_change = 1 as u8;
                p_parent =
                    unsafe {
                        unsafe {
                            (*p).a_parent.add(unsafe { (*p).n_parent } as usize)
                        }
                    };
                unsafe { (*p_parent).i_head = unsafe { (*p).i } };
                unsafe { (*p_parent).i_value = i };
                unsafe { (*p_parent).i_end = i + n + sz };
                unsafe { (*p_parent).i_key = -1 as i64 };
                unsafe {
                    (*p_parent).n_path = unsafe { (*p).path.n_used } as u32
                };
                if unsafe { (*p).e_type } != 0 &&
                        unsafe { (*p).n_parent } != 0 {
                    json_append_path_name(unsafe { &mut *p });
                    if unsafe { (*p).path.e_err } != 0 { rc = 7; }
                }
                {
                    let __p = unsafe { &mut (*p).n_parent };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                unsafe { (*p).i = i + n };
            } else { unsafe { (*p).i = i + n + sz }; }
            while unsafe { (*p).n_parent } > 0 as u32 &&
                    unsafe { (*p).i } >=
                        unsafe {
                            (*unsafe {
                                        (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                usize)
                                    }).i_end
                        } {
                {
                    let __p = unsafe { &mut (*p).n_parent };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
                unsafe {
                    (*p).path.n_used =
                        unsafe {
                                (*unsafe {
                                            (*p).a_parent.add(unsafe { (*p).n_parent } as usize)
                                        }).n_path
                            } as u64
                };
                level_change = 1 as u8;
            }
            if level_change != 0 {
                if unsafe { (*p).n_parent } > 0 as u32 {
                    let p_parent_1: *const JsonParent =
                        unsafe {
                                &raw mut *unsafe {
                                            (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                    usize)
                                        }
                            } as *const JsonParent;
                    let i_val: u32 = unsafe { (*p_parent_1).i_value };
                    unsafe {
                        (*p).e_type =
                            (unsafe {
                                            *unsafe { (*p).s_parse.a_blob.add(i_val as usize) }
                                        } as i32 & 15) as u8
                    };
                } else { unsafe { (*p).e_type = 0 as u8 }; }
            }
        } else {
            let mut n: u32 = 0 as u32;
            let mut sz: u32 = 0 as u32;
            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
            n = jsonb_payload_size(unsafe { &(*p).s_parse }, i, &mut sz);
            if n == 0 as u32 { return json_each_malformed_input(cur); }
            unsafe { (*p).i = i + n + sz };
        }
        if unsafe { (*p).e_type } as i32 == 11 &&
                unsafe { (*p).n_parent } != 0 {
            {
                let __p =
                    unsafe {
                        &mut (*unsafe {
                                        (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                usize)
                                    }).i_key
                    };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
        {
            let __p = unsafe { &mut (*p).i_rowid };
            let __t = *__p;
            *__p += 1;
            __t
        };
        return rc;
    }
}
extern "C" fn json_each_eof(cur: *mut sqlite3_vtab_cursor) -> i32 {
    unsafe {
        let p: *const JsonEachCursor =
            cur as *mut JsonEachCursor as *const JsonEachCursor;
        return (unsafe { (*p).i } >= unsafe { (*p).i_end }) as i32;
    }
}
extern "C" fn json_each_path_length(p: &mut JsonEachCursor) -> i32 {
    unsafe {
        let mut n: u32 = (*p).path.n_used as u32;
        let z: *mut i8 = (*p).path.z_buf;
        if (*p).i_rowid == 0 as u32 && (*p).b_recursive != 0 && n >= 2 as u32
            {
            while n > 1 as u32 {
                { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                if unsafe { *z.add(n as usize) } as i32 == '[' as i32 ||
                        unsafe { *z.add(n as usize) } as i32 == '.' as i32 {
                    let mut x: u32 = 0 as u32;
                    let mut sz: u32 = 0 as u32;
                    let c_saved: i8 = unsafe { *z.add(n as usize) };
                    unsafe { *z.add(n as usize) = 0 as i8 };
                    { let _ = 0; };
                    x =
                        json_lookup_step(&mut (*p).s_parse, 0 as u32,
                            unsafe { z.offset(1 as isize) } as *const i8, 0 as u32);
                    unsafe { *z.add(n as usize) = c_saved };
                    if x >= 4294967291u32 { continue; }
                    if x + jsonb_payload_size(&(*p).s_parse, x, &mut sz) ==
                            (*p).i {
                        break;
                    }
                }
            }
        }
        return n as i32;
    }
}
extern "C" fn json_each_column(cur: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i_column_1: i32) -> i32 {
    unsafe {
        unsafe {
            let p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
            '__s69:
                {
                match i_column_1 {
                    0 => {
                        {
                            if unsafe { (*p).n_parent } == 0 as u32 {
                                let mut n: u32 = 0 as u32;
                                let mut j: u32 = 0 as u32;
                                if unsafe { (*p).n_root } == 1 as u32 { break '__s69; }
                                j = json_each_path_length(unsafe { &mut *p }) as u32;
                                n = unsafe { (*p).n_root } - j;
                                if n == 0 as u32 {
                                    break '__s69;
                                } else if unsafe {
                                                *unsafe { (*p).path.z_buf.add(j as usize) }
                                            } as i32 == '[' as i32 {
                                    let mut x: i64 = 0 as i64;
                                    unsafe {
                                        sqlite3_atoi64(unsafe {
                                                    &raw mut *unsafe {
                                                                (*p).path.z_buf.add((j + 1 as u32) as usize)
                                                            }
                                                } as *const i8, &mut x, (n - 1 as u32) as i32, 1 as u8)
                                    };
                                    unsafe { sqlite3_result_int64(ctx, x) };
                                } else if unsafe {
                                                *unsafe { (*p).path.z_buf.add((j + 1 as u32) as usize) }
                                            } as i32 == '\"' as i32 {
                                    unsafe {
                                        sqlite3_result_text(ctx,
                                            unsafe {
                                                    &raw mut *unsafe {
                                                                (*p).path.z_buf.add((j + 2 as u32) as usize)
                                                            }
                                                } as *const i8, (n - 3 as u32) as i32,
                                            Some(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn(*mut ())
                                                                -> ()>(-1 as isize as *const ())
                                                }))
                                    };
                                } else {
                                    unsafe {
                                        sqlite3_result_text(ctx,
                                            unsafe {
                                                    &raw mut *unsafe {
                                                                (*p).path.z_buf.add((j + 1 as u32) as usize)
                                                            }
                                                } as *const i8, (n - 1 as u32) as i32,
                                            Some(unsafe {
                                                    core::mem::transmute::<*const (),
                                                            unsafe extern "C" fn(*mut ())
                                                                -> ()>(-1 as isize as *const ())
                                                }))
                                    };
                                }
                                break '__s69;
                            }
                            if unsafe { (*p).e_type } as i32 == 12 {
                                json_return_from_blob(unsafe { &raw mut (*p).s_parse } as
                                        *const JsonParse, unsafe { (*p).i }, ctx, 1);
                            } else {
                                { let _ = 0; };
                                unsafe {
                                    sqlite3_result_int64(ctx,
                                        unsafe {
                                            (*unsafe {
                                                        (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                                usize)
                                                    }).i_key
                                        })
                                };
                            }
                            break '__s69;
                        }
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            json_return_from_blob(unsafe { &raw mut (*p).s_parse } as
                                    *const JsonParse, i, ctx, unsafe { (*p).e_mode } as i32);
                            if unsafe {
                                                *unsafe { (*p).s_parse.a_blob.add(i as usize) }
                                            } as i32 & 15 >= 11 {
                                unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
                            }
                            break '__s69;
                        }
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            let e_type: u8 =
                                (unsafe { *unsafe { (*p).s_parse.a_blob.add(i as usize) } }
                                            as i32 & 15) as u8;
                            unsafe {
                                sqlite3_result_text(ctx, jsonb_type[e_type as usize], -1,
                                    None)
                            };
                            break '__s69;
                        }
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            if unsafe {
                                                *unsafe { (*p).s_parse.a_blob.add(i as usize) }
                                            } as i32 & 15 < 11 {
                                json_return_from_blob(unsafe { &raw mut (*p).s_parse } as
                                        *const JsonParse, i, ctx, 1);
                            }
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_int64(ctx,
                                    unsafe { (*p).i } as sqlite3_int64)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).n_parent } > 0 as u32 &&
                                    unsafe { (*p).b_recursive } != 0 {
                                unsafe {
                                    sqlite3_result_int64(ctx,
                                        unsafe {
                                                (*unsafe {
                                                            (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                                    usize)
                                                        }).i_head
                                            } as sqlite3_int64)
                                };
                            }
                            break '__s69;
                        }
                        {
                            let n_base: u64 = unsafe { (*p).path.n_used };
                            if unsafe { (*p).n_parent } != 0 {
                                json_append_path_name(unsafe { &mut *p });
                            }
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).path.n_used },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            unsafe { (*p).path.n_used = n_base };
                            break '__s69;
                        }
                        {
                            let mut n: u32 =
                                json_each_path_length(unsafe { &mut *p }) as u32;
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    n as sqlite3_uint64,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    1 => {
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            json_return_from_blob(unsafe { &raw mut (*p).s_parse } as
                                    *const JsonParse, i, ctx, unsafe { (*p).e_mode } as i32);
                            if unsafe {
                                                *unsafe { (*p).s_parse.a_blob.add(i as usize) }
                                            } as i32 & 15 >= 11 {
                                unsafe { sqlite3_result_subtype(ctx, 74 as u32) };
                            }
                            break '__s69;
                        }
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            let e_type: u8 =
                                (unsafe { *unsafe { (*p).s_parse.a_blob.add(i as usize) } }
                                            as i32 & 15) as u8;
                            unsafe {
                                sqlite3_result_text(ctx, jsonb_type[e_type as usize], -1,
                                    None)
                            };
                            break '__s69;
                        }
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            if unsafe {
                                                *unsafe { (*p).s_parse.a_blob.add(i as usize) }
                                            } as i32 & 15 < 11 {
                                json_return_from_blob(unsafe { &raw mut (*p).s_parse } as
                                        *const JsonParse, i, ctx, 1);
                            }
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_int64(ctx,
                                    unsafe { (*p).i } as sqlite3_int64)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).n_parent } > 0 as u32 &&
                                    unsafe { (*p).b_recursive } != 0 {
                                unsafe {
                                    sqlite3_result_int64(ctx,
                                        unsafe {
                                                (*unsafe {
                                                            (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                                    usize)
                                                        }).i_head
                                            } as sqlite3_int64)
                                };
                            }
                            break '__s69;
                        }
                        {
                            let n_base: u64 = unsafe { (*p).path.n_used };
                            if unsafe { (*p).n_parent } != 0 {
                                json_append_path_name(unsafe { &mut *p });
                            }
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).path.n_used },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            unsafe { (*p).path.n_used = n_base };
                            break '__s69;
                        }
                        {
                            let mut n: u32 =
                                json_each_path_length(unsafe { &mut *p }) as u32;
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    n as sqlite3_uint64,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    2 => {
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            let e_type: u8 =
                                (unsafe { *unsafe { (*p).s_parse.a_blob.add(i as usize) } }
                                            as i32 & 15) as u8;
                            unsafe {
                                sqlite3_result_text(ctx, jsonb_type[e_type as usize], -1,
                                    None)
                            };
                            break '__s69;
                        }
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            if unsafe {
                                                *unsafe { (*p).s_parse.a_blob.add(i as usize) }
                                            } as i32 & 15 < 11 {
                                json_return_from_blob(unsafe { &raw mut (*p).s_parse } as
                                        *const JsonParse, i, ctx, 1);
                            }
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_int64(ctx,
                                    unsafe { (*p).i } as sqlite3_int64)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).n_parent } > 0 as u32 &&
                                    unsafe { (*p).b_recursive } != 0 {
                                unsafe {
                                    sqlite3_result_int64(ctx,
                                        unsafe {
                                                (*unsafe {
                                                            (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                                    usize)
                                                        }).i_head
                                            } as sqlite3_int64)
                                };
                            }
                            break '__s69;
                        }
                        {
                            let n_base: u64 = unsafe { (*p).path.n_used };
                            if unsafe { (*p).n_parent } != 0 {
                                json_append_path_name(unsafe { &mut *p });
                            }
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).path.n_used },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            unsafe { (*p).path.n_used = n_base };
                            break '__s69;
                        }
                        {
                            let mut n: u32 =
                                json_each_path_length(unsafe { &mut *p }) as u32;
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    n as sqlite3_uint64,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    3 => {
                        {
                            let i: u32 = json_skip_label(unsafe { &mut *p }) as u32;
                            if unsafe {
                                                *unsafe { (*p).s_parse.a_blob.add(i as usize) }
                                            } as i32 & 15 < 11 {
                                json_return_from_blob(unsafe { &raw mut (*p).s_parse } as
                                        *const JsonParse, i, ctx, 1);
                            }
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_int64(ctx,
                                    unsafe { (*p).i } as sqlite3_int64)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).n_parent } > 0 as u32 &&
                                    unsafe { (*p).b_recursive } != 0 {
                                unsafe {
                                    sqlite3_result_int64(ctx,
                                        unsafe {
                                                (*unsafe {
                                                            (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                                    usize)
                                                        }).i_head
                                            } as sqlite3_int64)
                                };
                            }
                            break '__s69;
                        }
                        {
                            let n_base: u64 = unsafe { (*p).path.n_used };
                            if unsafe { (*p).n_parent } != 0 {
                                json_append_path_name(unsafe { &mut *p });
                            }
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).path.n_used },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            unsafe { (*p).path.n_used = n_base };
                            break '__s69;
                        }
                        {
                            let mut n: u32 =
                                json_each_path_length(unsafe { &mut *p }) as u32;
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    n as sqlite3_uint64,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    4 => {
                        {
                            unsafe {
                                sqlite3_result_int64(ctx,
                                    unsafe { (*p).i } as sqlite3_int64)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).n_parent } > 0 as u32 &&
                                    unsafe { (*p).b_recursive } != 0 {
                                unsafe {
                                    sqlite3_result_int64(ctx,
                                        unsafe {
                                                (*unsafe {
                                                            (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                                    usize)
                                                        }).i_head
                                            } as sqlite3_int64)
                                };
                            }
                            break '__s69;
                        }
                        {
                            let n_base: u64 = unsafe { (*p).path.n_used };
                            if unsafe { (*p).n_parent } != 0 {
                                json_append_path_name(unsafe { &mut *p });
                            }
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).path.n_used },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            unsafe { (*p).path.n_used = n_base };
                            break '__s69;
                        }
                        {
                            let mut n: u32 =
                                json_each_path_length(unsafe { &mut *p }) as u32;
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    n as sqlite3_uint64,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    5 => {
                        {
                            if unsafe { (*p).n_parent } > 0 as u32 &&
                                    unsafe { (*p).b_recursive } != 0 {
                                unsafe {
                                    sqlite3_result_int64(ctx,
                                        unsafe {
                                                (*unsafe {
                                                            (*p).a_parent.add((unsafe { (*p).n_parent } - 1 as u32) as
                                                                    usize)
                                                        }).i_head
                                            } as sqlite3_int64)
                                };
                            }
                            break '__s69;
                        }
                        {
                            let n_base: u64 = unsafe { (*p).path.n_used };
                            if unsafe { (*p).n_parent } != 0 {
                                json_append_path_name(unsafe { &mut *p });
                            }
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).path.n_used },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            unsafe { (*p).path.n_used = n_base };
                            break '__s69;
                        }
                        {
                            let mut n: u32 =
                                json_each_path_length(unsafe { &mut *p }) as u32;
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    n as sqlite3_uint64,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    6 => {
                        {
                            let n_base: u64 = unsafe { (*p).path.n_used };
                            if unsafe { (*p).n_parent } != 0 {
                                json_append_path_name(unsafe { &mut *p });
                            }
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).path.n_used },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            unsafe { (*p).path.n_used = n_base };
                            break '__s69;
                        }
                        {
                            let mut n: u32 =
                                json_each_path_length(unsafe { &mut *p }) as u32;
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    n as sqlite3_uint64,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    7 => {
                        {
                            let mut n: u32 =
                                json_each_path_length(unsafe { &mut *p }) as u32;
                            unsafe {
                                sqlite3_result_text64(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    n as sqlite3_uint64,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }), 1 as u8)
                            };
                            break '__s69;
                        }
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    8 => {
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                    _ => {
                        {
                            unsafe {
                                sqlite3_result_text(ctx,
                                    unsafe { (*p).path.z_buf } as *const i8,
                                    unsafe { (*p).n_root } as i32, None)
                            };
                            break '__s69;
                        }
                        {
                            if unsafe { (*p).s_parse.z_json } == core::ptr::null_mut() {
                                unsafe {
                                    sqlite3_result_blob(ctx,
                                        unsafe { (*p).s_parse.a_blob } as *const (),
                                        unsafe { (*p).s_parse.n_blob } as i32,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            } else {
                                unsafe {
                                    sqlite3_result_text(ctx,
                                        unsafe { (*p).s_parse.z_json } as *const i8, -1,
                                        Some(unsafe {
                                                core::mem::transmute::<*const (),
                                                        unsafe extern "C" fn(*mut ())
                                                            -> ()>(-1 as isize as *const ())
                                            }))
                                };
                            }
                            break '__s69;
                        }
                    }
                }
            }
            return 0;
        }
    }
}
extern "C" fn json_each_rowid(cur: *mut sqlite3_vtab_cursor,
    p_rowid_1: *mut sqlite_int64) -> i32 {
    let p: *const JsonEachCursor =
        cur as *mut JsonEachCursor as *const JsonEachCursor;
    unsafe { *p_rowid_1 = unsafe { (*p).i_rowid } as sqlite_int64 };
    return 0;
}
static mut json_each_module: sqlite3_module =
    sqlite3_module {
        i_version: 0,
        x_create: None,
        x_connect: Some(json_each_connect),
        x_best_index: Some(json_each_best_index),
        x_disconnect: Some(json_each_disconnect),
        x_destroy: None,
        x_open: Some(json_each_open),
        x_close: Some(json_each_close),
        x_filter: Some(json_each_filter),
        x_next: Some(json_each_next),
        x_eof: Some(json_each_eof),
        x_column: Some(json_each_column),
        x_rowid: Some(json_each_rowid),
        x_update: None,
        x_begin: None,
        x_sync: None,
        x_commit: None,
        x_rollback: None,
        x_find_function: None,
        x_rename: None,
        x_savepoint: None,
        x_release: None,
        x_rollback_to: None,
        x_shadow_name: None,
        x_integrity: None,
    };
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_json_vtab_register(db: *mut sqlite3,
    z_name: *const i8) -> *mut Module {
    unsafe {
        let mut i: u32 = 0 as u32;
        { let _ = 0; };
        {
            i = 0 as u32;
            '__b70: loop {
                if !((i as u64) <
                                core::mem::size_of::<[*const i8; 4]>() as u64 /
                                    core::mem::size_of::<*const i8>() as u64) {
                    break '__b70;
                }
                '__c70: loop {
                    if unsafe {
                                sqlite3_str_i_cmp(az_module[i as usize], z_name)
                            } == 0 {
                        return unsafe {
                                sqlite3_vtab_create_module(db, az_module[i as usize],
                                    &raw mut json_each_module as *const sqlite3_module,
                                    core::ptr::null_mut(), None)
                            };
                    }
                    break '__c70;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return core::ptr::null_mut();
    }
}
static a_type: [u8; 8] =
    [192 as u8, 208 as u8, 0 as u8, 224 as u8, 0 as u8, 0 as u8, 0 as u8,
            240 as u8];
static empty_object_1: [u8; 2] = [11 as u8, 12 as u8];
static a_special: [i8; 32] =
    [0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            'b' as i32 as i8, 't' as i32 as i8, 'n' as i32 as i8, 0 as i8,
            'f' as i32 as i8, 'r' as i32 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
            0 as i8];
static mut a_null: [u8; 1] = [0 as u8];
static mut az_ins_type: [*const i8; 3] =
    [c"insert".as_ptr() as *const i8, c"set".as_ptr() as *const i8,
            c"array_insert".as_ptr() as *const i8];
static a_edit_type: [u8; 3] = [3 as u8, 4 as u8, 5 as u8];
static empty_array: u8 = 11 as u8;
static empty_object_2: u8 = 12 as u8;
static mut a_json_func: [FuncDef; 36] =
    [FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_remove_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_remove_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 0 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_array_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_array".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 0 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_array_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_array".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (8 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_set_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_array_insert".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            1 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (8 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_set_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_array_insert".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_array_length_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_array_length".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_array_length_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_array_length".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_error_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_error_position".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_extract_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_extract".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_extract_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_extract".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (1 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_extract_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"->".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (2 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_extract_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"->>".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_set_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_insert".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            1 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_set_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_insert".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 0 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_object_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_object".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 0 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_object_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_object".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_patch_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_patch".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_patch_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_patch".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_pretty_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_pretty".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_pretty_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_pretty".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 0 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_quote_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_quote".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_remove_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_remove".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_remove_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_remove".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_replace_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_replace".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            1 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_replace_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_replace".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            1 * 1048576 | 1 * 16777216) as u32,
                p_user_data: (4 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_set_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_set".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: -1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            1 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (4 | 1 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_set_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"jsonb_set".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_type_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_type".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_type_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_type".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_valid_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_valid".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 2048 | 2048 | 1 | 1 * 32768 |
                            0 * 1048576 | 0 * 16777216) as u32,
                p_user_data: (0 | 0 * 16) as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_valid_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"json_valid".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 1048576 | 16777216 | 1 |
                        2048) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_array_step),
                x_finalize: Some(json_array_final),
                x_value: Some(json_array_value),
                x_inverse: Some(json_group_inverse),
                z_name: c"json_group_array".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 1048576 | 16777216 | 1 |
                        2048) as u32,
                p_user_data: 16 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_array_step),
                x_finalize: Some(json_array_final),
                x_value: Some(json_array_value),
                x_inverse: Some(json_group_inverse),
                z_name: c"jsonb_group_array".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 1048576 | 16777216 | 1 |
                        2048) as u32,
                p_user_data: 0 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_object_step),
                x_finalize: Some(json_object_final),
                x_value: Some(json_object_value),
                x_inverse: Some(json_group_inverse),
                z_name: c"json_group_object".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 0 * 32 | 1048576 | 16777216 | 1 |
                        2048) as u32,
                p_user_data: 16 as i64 as *mut (),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(json_object_step),
                x_finalize: Some(json_object_final),
                x_value: Some(json_object_value),
                x_inverse: Some(json_group_inverse),
                z_name: c"jsonb_group_object".as_ptr() as *const i8,
                u: FuncDef_u0 { p_hash: core::ptr::null_mut() },
            }];
static mut az_module: [*const i8; 4] =
    [c"json_each".as_ptr() as *const i8, c"json_tree".as_ptr() as *const i8,
            c"jsonb_each".as_ptr() as *const i8,
            c"jsonb_tree".as_ptr() as *const i8];
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
    fn sqlite3_strlen30(_: *const i8)
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
    fn sqlite3_db_malloc_raw(_: *mut sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw_nn(_: *mut sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_str_dup(_: *mut sqlite3, _: *const i8)
    -> *mut i8;
    fn sqlite3_db_str_n_dup(_: *mut sqlite3, _: *const i8, _: u64)
    -> *mut i8;
    fn sqlite3_db_span_dup(_: *mut sqlite3, _: *const i8, _: *const i8)
    -> *mut i8;
    fn sqlite3Realloc(_: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_realloc_or_free(_: *mut sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_realloc(_: *mut sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_free(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_free_nn(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_nn_free_nn(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_malloc_size(_: *const ())
    -> i32;
    fn sqlite3_db_malloc_size(_: *mut sqlite3, _: *const ())
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
    fn sqlite3_append_one_utf8_character(_: *mut i8, _: u32)
    -> i32;
    fn sqlite3_register_builtin_functions()
    -> ();
    fn sqlite3_register_date_time_functions()
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn sqlite3_rc_str_unref(_: *mut ())
    -> ();
    static sqlite3_ctype_map: [u8; 0];
    fn sqlite3_utf8_read_limited(_: *const u8, _: i32, _: *mut u32)
    -> i32;
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strspn(__s: *const i8, __charset: *const i8)
    -> u64;
    fn sqlite3_value_is_of_class(_: *const sqlite3_value,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_rc_str_new(_: u64)
    -> *mut i8;
    fn sqlite3_rc_str_ref(_: *mut i8)
    -> *mut i8;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memchr(__s: *const (), __c: i32, __n: u64)
    -> *mut ();
    fn sqlite3_rc_str_resize(_: *mut i8, _: u64)
    -> *mut i8;
    fn sqlite3_hex_to_int(h: i32)
    -> u8;
    fn strlen(__s: *const i8)
    -> u64;
    fn sqlite3_dec_or_hex_to_i64(_: *const i8, _: *mut i64)
    -> i32;
    fn sqlite3_ato_f(z: *const i8, _: *mut f64)
    -> i32;
    fn sqlite3_register_per_connection_builtin_functions(_: *mut sqlite3)
    -> ();
    fn sqlite3_vtab_create_module(_: *mut sqlite3, _: *const i8,
    _: *const sqlite3_module, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut Module;
    fn sqlite3_atoi64(_: *const i8, _: *mut i64, _: i32, _: u8)
    -> i32;
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
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3_builtin_functions: FuncDefHash;
    static sqlite3_oom_str: sqlite3_str;
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
    fn sqlite3_oom_fault(_: *mut sqlite3)
    -> *mut ();
    fn sqlite3_oom_clear(_: *mut sqlite3)
    -> ();
    fn sqlite3_api_exit(db: *mut sqlite3, _: i32)
    -> i32;
    fn sqlite3_open_temp_database(_: *mut Parse)
    -> i32;
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut sqlite3, _: *mut i8,
    _: i32, _: i32)
    -> ();
    fn sqlite3_str_accum_enlarge(_: *mut StrAccum, _: i64)
    -> i32;
    fn sqlite3_str_accum_enlarge_if_needed(_: *mut StrAccum, _: i64)
    -> i32;
    fn sqlite3_str_accum_finish(_: *mut StrAccum)
    -> *mut i8;
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