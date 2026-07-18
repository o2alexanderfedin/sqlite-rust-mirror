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
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_match_e_name(p_item: &ExprList_item,
    z_col: *const i8, z_tab: *const i8, z_db: *const i8, pb_rowid: *mut i32)
    -> i32 {
    let mut n: i32 = 0;
    let mut z_span: *const i8 = core::ptr::null();
    let e_e_name: i32 = (*p_item).fg.e_e_name() as i32;
    if e_e_name != 2 && (e_e_name != 3 || pb_rowid == core::ptr::null_mut()) {
        return 0;
    }
    { let _ = 0; };
    z_span = (*p_item).z_e_name as *const i8;
    {
        n = 0;
        '__b0: loop {
            if !(unsafe { *z_span.offset(n as isize) } != 0 &&
                            unsafe { *z_span.offset(n as isize) } as i32 != '.' as i32)
                {
                break '__b0;
            }
            '__c0: loop { break '__c0; }
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
        }
    }
    if !(z_db).is_null() &&
            (unsafe { sqlite3_strnicmp(z_span, z_db, n) } != 0 ||
                unsafe { *z_db.offset(n as isize) } as i32 != 0) {
        return 0;
    }
    {
        let __n = n + 1;
        let __p = &mut z_span;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    {
        n = 0;
        '__b1: loop {
            if !(unsafe { *z_span.offset(n as isize) } != 0 &&
                            unsafe { *z_span.offset(n as isize) } as i32 != '.' as i32)
                {
                break '__b1;
            }
            '__c1: loop { break '__c1; }
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
        }
    }
    if !(z_tab).is_null() &&
            (unsafe { sqlite3_strnicmp(z_span, z_tab, n) } != 0 ||
                unsafe { *z_tab.offset(n as isize) } as i32 != 0) {
        return 0;
    }
    {
        let __n = n + 1;
        let __p = &mut z_span;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    if !(z_col).is_null() {
        if e_e_name == 2 && unsafe { sqlite3_str_i_cmp(z_span, z_col) } != 0 {
            return 0;
        }
        if e_e_name == 3 && unsafe { sqlite3_is_rowid(z_col) } == 0 {
            return 0;
        }
    }
    if e_e_name == 3 { unsafe { *pb_rowid = 1 }; }
    return 1;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_col_used(p_expr: &Expr) -> Bitmask {
    unsafe {
        let mut n: i32 = 0;
        let mut p_ex_tab: *const Table = core::ptr::null();
        n = (*p_expr).i_column as i32;
        { let _ = 0; };
        p_ex_tab = (*p_expr).y.p_tab;
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_ex_tab).tab_flags } & 96 as u32 != 0 as u32 &&
                unsafe {
                                (*unsafe { (*p_ex_tab).a_col.offset(n as isize) }).col_flags
                            } as i32 & 96 != 0 {
            return if unsafe { (*p_ex_tab).n_col } as i32 >=
                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                    -1i32 as Bitmask
                } else {
                    ((1 as Bitmask) << unsafe { (*p_ex_tab).n_col }) -
                        1 as Bitmask
                };
        } else {
            if n >= (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32
                {
                n =
                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                        1;
            }
            return (1 as Bitmask) << n;
        }
    }
}
extern "C" fn not_valid_impl(p_parse_1: *mut Parse, p_nc_1: &NameContext,
    z_msg_1: *const i8, p_expr_1: *mut Expr, p_error_1: *const Expr) -> () {
    let mut z_in: *const i8 =
        c"partial index WHERE clauses".as_ptr() as *mut i8 as *const i8;
    if (*p_nc_1).nc_flags & 32 != 0 {
        z_in = c"index expressions".as_ptr() as *mut i8 as *const i8;
    } else if (*p_nc_1).nc_flags & 4 != 0 {
        z_in = c"CHECK constraints".as_ptr() as *mut i8 as *const i8;
    } else if (*p_nc_1).nc_flags & 8 != 0 {
        z_in = c"generated columns".as_ptr() as *mut i8 as *const i8;
    }
    unsafe {
        sqlite3_error_msg(p_parse_1,
            c"%s prohibited in %s".as_ptr() as *mut i8 as *const i8, z_msg_1,
            z_in)
    };
    if !(p_expr_1).is_null() { unsafe { (*p_expr_1).op = 122 as u8 }; }
    unsafe {
        sqlite3_record_error_offset_of_expr(unsafe { (*p_parse_1).db },
            p_error_1 as *const Expr)
    };
}
extern "C" fn extend_fj_match(p_parse_1: *mut Parse,
    pp_list_1: &mut *mut ExprList, p_match_1: &SrcItem, i_column_1: i16)
    -> () {
    unsafe {
        let p_new: *mut Expr =
            unsafe {
                sqlite3_expr_alloc(unsafe { (*p_parse_1).db }, 168,
                    core::ptr::null(), 0)
            };
        if !(p_new).is_null() {
            unsafe { (*p_new).i_table = (*p_match_1).i_cursor };
            unsafe { (*p_new).i_column = i_column_1 };
            unsafe { (*p_new).y.p_tab = (*p_match_1).p_s_tab };
            { let _ = 0; };
            unsafe { (*p_new).flags |= 2097152 as u32 };
            *pp_list_1 =
                unsafe {
                    sqlite3_expr_list_append(p_parse_1, *pp_list_1, p_new)
                };
        }
    }
}
extern "C" fn is_valid_schema_table_name(z_tab_1: *const i8, p_tab_1: &Table,
    z_db_1: *const i8) -> i32 {
    let mut z_legacy: *const i8 = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    if unsafe {
                sqlite3_strnicmp(z_tab_1,
                    c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
            } != 0 {
        return 0;
    }
    z_legacy = (*p_tab_1).z_name as *const i8;
    if unsafe {
                strcmp(unsafe { z_legacy.offset(7 as isize) },
                    unsafe {
                            &raw mut *(c"sqlite_temp_master".as_ptr() as
                                            *mut i8).offset(7 as isize)
                        } as *const i8)
            } == 0 {
        if unsafe {
                    sqlite3_str_i_cmp(unsafe { z_tab_1.offset(7 as isize) },
                        unsafe {
                                &raw mut *(c"sqlite_temp_schema".as_ptr() as
                                                *mut i8).offset(7 as isize)
                            } as *const i8)
                } == 0 {
            return 1;
        }
        if z_db_1 == core::ptr::null() { return 0; }
        if unsafe {
                    sqlite3_str_i_cmp(unsafe { z_tab_1.offset(7 as isize) },
                        unsafe {
                                &raw mut *(c"sqlite_master".as_ptr() as
                                                *mut i8).offset(7 as isize)
                            } as *const i8)
                } == 0 {
            return 1;
        }
        if unsafe {
                    sqlite3_str_i_cmp(unsafe { z_tab_1.offset(7 as isize) },
                        unsafe {
                                &raw mut *(c"sqlite_schema".as_ptr() as
                                                *mut i8).offset(7 as isize)
                            } as *const i8)
                } == 0 {
            return 1;
        }
    } else {
        if unsafe {
                    sqlite3_str_i_cmp(unsafe { z_tab_1.offset(7 as isize) },
                        unsafe {
                                &raw mut *(c"sqlite_schema".as_ptr() as
                                                *mut i8).offset(7 as isize)
                            } as *const i8)
                } == 0 {
            return 1;
        }
    }
    return 0;
}
extern "C" fn incr_agg_depth(p_walker_1: *mut Walker, p_expr_1: *mut Expr)
    -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 169 {
            unsafe { (*p_expr_1).op2 += unsafe { (*p_walker_1).u.n } as u8 };
        }
        return 0;
    }
}
extern "C" fn incr_agg_function_depth(p_expr_1: *mut Expr, n_1: i32) -> () {
    unsafe {
        if n_1 > 0 {
            let mut w: Walker = unsafe { core::mem::zeroed() };
            unsafe {
                memset(&raw mut w as *mut (), 0,
                    core::mem::size_of::<Walker>() as u64)
            };
            w.x_expr_callback = Some(incr_agg_depth);
            w.u.n = n_1;
            unsafe { sqlite3_walk_expr(&mut w, p_expr_1) };
        }
    }
}
extern "C" fn resolve_alias(p_parse_1: *mut Parse, p_e_list_1: &ExprList,
    i_col_1: i32, p_expr_1: *mut Expr, n_subquery_1: i32) -> () {
    unsafe {
        let mut p_orig: *const Expr = core::ptr::null();
        let mut p_dup: *mut Expr = core::ptr::null_mut();
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        { let _ = 0; };
        p_orig =
            unsafe {
                (*((*p_e_list_1).a.as_ptr() as
                                *mut ExprList_item).offset(i_col_1 as isize)).p_expr
            };
        { let _ = 0; };
        { let _ = 0; };
        if !(unsafe { (*p_expr_1).p_agg_info }).is_null() { return; }
        db = unsafe { (*p_parse_1).db };
        p_dup = unsafe { sqlite3_expr_dup(db, p_orig as *const Expr, 0) };
        if unsafe { (*db).malloc_failed } != 0 {
            unsafe { sqlite3_expr_delete(db, p_dup) };
            p_dup = core::ptr::null_mut();
        } else {
            let mut temp: Expr = unsafe { core::mem::zeroed() };
            incr_agg_function_depth(p_dup, n_subquery_1);
            if unsafe { (*p_expr_1).op } as i32 == 114 {
                { let _ = 0; };
                p_dup =
                    unsafe {
                        sqlite3_expr_add_collate_string(p_parse_1 as *const Parse,
                            p_dup, unsafe { (*p_expr_1).u.z_token } as *const i8)
                    };
            }
            unsafe {
                memcpy(&raw mut temp as *mut (), p_dup as *const (),
                    core::mem::size_of::<Expr>() as u64)
            };
            unsafe {
                memcpy(p_dup as *mut (), p_expr_1 as *const (),
                    core::mem::size_of::<Expr>() as u64)
            };
            unsafe {
                memcpy(p_expr_1 as *mut (), &raw mut temp as *const (),
                    core::mem::size_of::<Expr>() as u64)
            };
            if unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                if unsafe { (*p_expr_1).y.p_win } != core::ptr::null_mut() {
                    unsafe {
                        (*unsafe { (*p_expr_1).y.p_win }).p_owner = p_expr_1
                    };
                }
            }
            unsafe { sqlite3_expr_deferred_delete(p_parse_1, p_dup) };
        }
    }
}
extern "C" fn are_double_quoted_strings_enabled(db: *mut sqlite3,
    p_top_nc_1: &NameContext) -> i32 {
    if unsafe { (*db).init.busy } != 0 { return 1; }
    if (*p_top_nc_1).nc_flags & 65536 != 0 {
        if unsafe { sqlite3_writable_schema(db) } != 0 &&
                unsafe { (*db).flags } & 1073741824 as u64 != 0 as u64 {
            return 1;
        }
        return (unsafe { (*db).flags } & 536870912 as u64 != 0 as u64) as i32;
    } else {
        return (unsafe { (*db).flags } & 1073741824 as u64 != 0 as u64) as
                i32;
    }
}
extern "C" fn lookup_name(p_parse_1: *mut Parse, mut z_db_1: *const i8,
    z_tab_1: *const i8, p_right_1: &Expr, mut p_nc_1: *mut NameContext,
    mut p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut cnt: i32 = 0;
        let mut cnt_tab: i32 = 0;
        let mut n_subquery: i32 = 0;
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        let mut p_match: *mut SrcItem = core::ptr::null_mut();
        let mut p_top_nc: *mut NameContext = core::ptr::null_mut();
        let mut p_schema: *mut Schema = core::ptr::null_mut();
        let mut e_new_expr_op: i32 = 0;
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut p_fj_match: *mut ExprList = core::ptr::null_mut();
        let mut z_col: *const i8 = core::ptr::null();
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let mut p_src_list: *mut SrcList = core::ptr::null_mut();
        let mut hit: i32 = 0;
        let mut p_sel: *const Select = core::ptr::null();
        let mut b_rowid: i32 = 0;
        let mut op: i32 = 0;
        let mut p_upsert: *const Upsert = core::ptr::null();
        let mut i_col: i32 = 0;
        let mut z_as: *mut i8 = core::ptr::null_mut();
        let mut p_orig: *const Expr = core::ptr::null();
        let mut z_err: *const i8 = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s3:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        if cnt == 1 { __state = 289; } else { __state = 290; }
                    }
                    3 => { cnt = 0; __state = 4; }
                    4 => { cnt_tab = 0; __state = 5; }
                    5 => { n_subquery = 0; __state = 6; }
                    6 => { db = unsafe { (*p_parse_1).db }; __state = 7; }
                    7 => { __state = 8; }
                    8 => { p_match = core::ptr::null_mut(); __state = 9; }
                    9 => { p_top_nc = p_nc_1; __state = 10; }
                    10 => { p_schema = core::ptr::null_mut(); __state = 11; }
                    11 => { e_new_expr_op = 168; __state = 12; }
                    12 => { p_tab = core::ptr::null_mut(); __state = 13; }
                    13 => { p_fj_match = core::ptr::null_mut(); __state = 14; }
                    14 => {
                        z_col = (*p_right_1).u.z_token as *const i8;
                        __state = 15;
                    }
                    15 => { { let _ = 0; }; __state = 16; }
                    16 => { { let _ = 0; }; __state = 17; }
                    17 => { { let _ = 0; }; __state = 18; }
                    18 => { { let _ = 0; }; __state = 19; }
                    19 => { unsafe { (*p_expr_1).i_table = -1 }; __state = 20; }
                    20 => { __state = 21; }
                    21 => {
                        if !(z_db_1).is_null() {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    22 => { { let _ = 0; }; __state = 37; }
                    23 => { __state = 24; }
                    24 => { __state = 25; }
                    25 => {
                        if unsafe { (*p_nc_1).nc_flags } & (2 | 4) != 0 {
                            __state = 26;
                        } else { __state = 27; }
                    }
                    26 => { z_db_1 = core::ptr::null(); __state = 22; }
                    27 => { i = 0; __state = 29; }
                    28 => {
                        if i == unsafe { (*db).n_db } &&
                                unsafe {
                                        sqlite3_str_i_cmp(c"main".as_ptr() as *mut i8 as *const i8,
                                            z_db_1)
                                    } == 0 {
                            __state = 35;
                        } else { __state = 22; }
                    }
                    29 => {
                        if i < unsafe { (*db).n_db } {
                            __state = 30;
                        } else { __state = 28; }
                    }
                    30 => { { let _ = 0; }; __state = 32; }
                    31 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 29;
                    }
                    32 => {
                        if unsafe {
                                    sqlite3_str_i_cmp(unsafe {
                                                (*unsafe { (*db).a_db.offset(i as isize) }).z_db_s_name
                                            } as *const i8, z_db_1)
                                } == 0 {
                            __state = 33;
                        } else { __state = 31; }
                    }
                    33 => {
                        p_schema =
                            unsafe {
                                (*unsafe { (*db).a_db.offset(i as isize) }).p_schema
                            };
                        __state = 34;
                    }
                    34 => { __state = 28; }
                    35 => {
                        p_schema =
                            unsafe {
                                (*unsafe { (*db).a_db.offset(0 as isize) }).p_schema
                            };
                        __state = 36;
                    }
                    36 => {
                        z_db_1 =
                            unsafe {
                                    (*unsafe { (*db).a_db.offset(0 as isize) }).z_db_s_name
                                } as *const i8;
                        __state = 22;
                    }
                    37 => { __state = 40; }
                    38 => {
                        if cnt == 0 && z_tab_1 == core::ptr::null() {
                            __state = 236;
                        } else { __state = 235; }
                    }
                    39 => {
                        if !(p_nc_1).is_null() {
                            __state = 37;
                        } else { __state = 38; }
                    }
                    40 => {
                        p_src_list = unsafe { (*p_nc_1).p_src_list };
                        __state = 41;
                    }
                    41 => {
                        if !(p_src_list).is_null() {
                            __state = 43;
                        } else { __state = 42; }
                    }
                    42 => {
                        if cnt == 0 && z_db_1 == core::ptr::null() {
                            __state = 138;
                        } else { __state = 137; }
                    }
                    43 => {
                        {
                            i = 0;
                            p_item = unsafe { (*p_src_list).a.as_ptr() } as *mut SrcItem
                        };
                        __state = 45;
                    }
                    44 => {
                        if !(p_match).is_null() {
                            __state = 131;
                        } else { __state = 42; }
                    }
                    45 => {
                        if i < unsafe { (*p_src_list).n_src } {
                            __state = 46;
                        } else { __state = 44; }
                    }
                    46 => {
                        p_tab = unsafe { (*p_item).p_s_tab };
                        __state = 48;
                    }
                    47 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut p_item;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 45;
                    }
                    48 => { { let _ = 0; }; __state = 49; }
                    49 => { { let _ = 0; }; __state = 50; }
                    50 => { { let _ = 0; }; __state = 51; }
                    51 => {
                        if unsafe { (*p_item).fg.is_nested_from() } != 0 {
                            __state = 53;
                        } else { __state = 52; }
                    }
                    52 => { { let _ = 0; }; __state = 93; }
                    53 => { hit = 0; __state = 54; }
                    54 => { __state = 55; }
                    55 => { { let _ = 0; }; __state = 56; }
                    56 => { { let _ = 0; }; __state = 57; }
                    57 => {
                        p_sel =
                            unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select };
                        __state = 58;
                    }
                    58 => { { let _ = 0; }; __state = 59; }
                    59 => {
                        p_e_list = unsafe { (*p_sel).p_e_list };
                        __state = 60;
                    }
                    60 => { { let _ = 0; }; __state = 61; }
                    61 => { { let _ = 0; }; __state = 62; }
                    62 => { j = 0; __state = 64; }
                    63 => {
                        if hit != 0 || z_tab_1 == core::ptr::null() {
                            __state = 92;
                        } else { __state = 52; }
                    }
                    64 => {
                        if j < unsafe { (*p_e_list).n_expr } {
                            __state = 65;
                        } else { __state = 63; }
                    }
                    65 => { b_rowid = 0; __state = 67; }
                    66 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 64;
                    }
                    67 => {
                        if (sqlite3_match_e_name(unsafe {
                                                &*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                *mut ExprList_item).offset(j as isize)
                                            }, z_col, z_tab_1, z_db_1, &mut b_rowid) == 0) as i32 != 0 {
                            __state = 69;
                        } else { __state = 68; }
                    }
                    68 => {
                        if b_rowid == 0 { __state = 71; } else { __state = 72; }
                    }
                    69 => { __state = 66; }
                    70 => {
                        { let __p = &mut cnt_tab; let __t = *__p; *__p += 1; __t };
                        __state = 86;
                    }
                    71 => {
                        if cnt > 0 { __state = 74; } else { __state = 73; }
                    }
                    72 => {
                        if cnt > 0 { __state = 85; } else { __state = 70; }
                    }
                    73 => {
                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                        __state = 84;
                    }
                    74 => {
                        if unsafe { (*p_item).fg.is_using() } as i32 == 0 ||
                                    unsafe {
                                            sqlite3_id_list_index(unsafe { (*p_item).u3.p_using },
                                                z_col)
                                        } < 0 || p_match == p_item {
                            __state = 75;
                        } else { __state = 76; }
                    }
                    75 => {
                        unsafe { sqlite3_expr_list_delete(db, p_fj_match) };
                        __state = 77;
                    }
                    76 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 16 == 0 {
                            __state = 78;
                        } else { __state = 79; }
                    }
                    77 => { p_fj_match = core::ptr::null_mut(); __state = 73; }
                    78 => { __state = 66; }
                    79 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 8 == 0 {
                            __state = 80;
                        } else { __state = 81; }
                    }
                    80 => { cnt = 0; __state = 82; }
                    81 => {
                        extend_fj_match(p_parse_1, &mut p_fj_match,
                            unsafe { &*p_match }, unsafe { (*p_expr_1).i_column });
                        __state = 73;
                    }
                    82 => {
                        unsafe { sqlite3_expr_list_delete(db, p_fj_match) };
                        __state = 83;
                    }
                    83 => { p_fj_match = core::ptr::null_mut(); __state = 73; }
                    84 => { hit = 1; __state = 70; }
                    85 => { __state = 66; }
                    86 => { p_match = p_item; __state = 87; }
                    87 => {
                        unsafe { (*p_expr_1).i_column = j as ynVar };
                        __state = 88;
                    }
                    88 => {
                        unsafe {
                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(j as
                                                isize)).fg.set_b_used(1 as u32 as u32)
                        };
                        __state = 89;
                    }
                    89 => { { let _ = 0; }; __state = 90; }
                    90 => {
                        if unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                        *mut ExprList_item).offset(j as isize)).fg.b_using_term()
                                } != 0 {
                            __state = 91;
                        } else { __state = 66; }
                    }
                    91 => { __state = 63; }
                    92 => { __state = 47; }
                    93 => {
                        if !(z_tab_1).is_null() {
                            __state = 95;
                        } else { __state = 94; }
                    }
                    94 => {
                        j = unsafe { sqlite3_column_index(p_tab, z_col) };
                        __state = 111;
                    }
                    95 => {
                        if !(z_db_1).is_null() {
                            __state = 97;
                        } else { __state = 96; }
                    }
                    96 => {
                        if unsafe { (*p_item).z_alias } != core::ptr::null_mut() {
                            __state = 102;
                        } else { __state = 103; }
                    }
                    97 => {
                        if unsafe { (*p_tab).p_schema } != p_schema {
                            __state = 99;
                        } else { __state = 98; }
                    }
                    98 => {
                        if p_schema == core::ptr::null_mut() &&
                                unsafe {
                                        strcmp(z_db_1, c"*".as_ptr() as *mut i8 as *const i8)
                                    } != 0 {
                            __state = 100;
                        } else { __state = 96; }
                    }
                    99 => { __state = 47; }
                    100 => { __state = 47; }
                    101 => { { let _ = 0; }; __state = 109; }
                    102 => {
                        if unsafe {
                                    sqlite3_str_i_cmp(z_tab_1,
                                        unsafe { (*p_item).z_alias } as *const i8)
                                } != 0 {
                            __state = 104;
                        } else { __state = 101; }
                    }
                    103 => {
                        if unsafe {
                                    sqlite3_str_i_cmp(z_tab_1,
                                        unsafe { (*p_tab).z_name } as *const i8)
                                } != 0 {
                            __state = 105;
                        } else { __state = 101; }
                    }
                    104 => { __state = 47; }
                    105 => {
                        if unsafe { (*p_tab).tnum } != 1 as u32 {
                            __state = 107;
                        } else { __state = 106; }
                    }
                    106 => {
                        if (is_valid_schema_table_name(z_tab_1, unsafe { &*p_tab },
                                            z_db_1) == 0) as i32 != 0 {
                            __state = 108;
                        } else { __state = 101; }
                    }
                    107 => { __state = 47; }
                    108 => { __state = 47; }
                    109 => {
                        if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 &&
                                !(unsafe { (*p_item).z_alias }).is_null() {
                            __state = 110;
                        } else { __state = 94; }
                    }
                    110 => {
                        unsafe {
                            sqlite3_rename_token_remap(p_parse_1, core::ptr::null(),
                                unsafe { &raw mut (*p_expr_1).y.p_tab } as *mut () as
                                    *const ())
                        };
                        __state = 94;
                    }
                    111 => {
                        if j >= 0 { __state = 113; } else { __state = 112; }
                    }
                    112 => {
                        if 0 == cnt &&
                                unsafe { (*p_tab).tab_flags } & 512 as u32 == 0 as u32 {
                            __state = 129;
                        } else { __state = 47; }
                    }
                    113 => {
                        if cnt > 0 { __state = 115; } else { __state = 114; }
                    }
                    114 => {
                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                        __state = 125;
                    }
                    115 => {
                        if unsafe { (*p_item).fg.is_using() } as i32 == 0 ||
                                unsafe {
                                        sqlite3_id_list_index(unsafe { (*p_item).u3.p_using },
                                            z_col)
                                    } < 0 {
                            __state = 116;
                        } else { __state = 117; }
                    }
                    116 => {
                        unsafe { sqlite3_expr_list_delete(db, p_fj_match) };
                        __state = 118;
                    }
                    117 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 16 == 0 {
                            __state = 119;
                        } else { __state = 120; }
                    }
                    118 => {
                        p_fj_match = core::ptr::null_mut();
                        __state = 114;
                    }
                    119 => { __state = 47; }
                    120 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 8 == 0 {
                            __state = 121;
                        } else { __state = 122; }
                    }
                    121 => { cnt = 0; __state = 123; }
                    122 => {
                        extend_fj_match(p_parse_1, &mut p_fj_match,
                            unsafe { &*p_match }, unsafe { (*p_expr_1).i_column });
                        __state = 114;
                    }
                    123 => {
                        unsafe { sqlite3_expr_list_delete(db, p_fj_match) };
                        __state = 124;
                    }
                    124 => {
                        p_fj_match = core::ptr::null_mut();
                        __state = 114;
                    }
                    125 => { p_match = p_item; __state = 126; }
                    126 => {
                        unsafe {
                            (*p_expr_1).i_column =
                                if j == unsafe { (*p_tab).i_p_key } as i32 {
                                        -1
                                    } else { j as i16 as i32 } as ynVar
                        };
                        __state = 127;
                    }
                    127 => {
                        if unsafe { (*p_item).fg.is_nested_from() } != 0 {
                            __state = 128;
                        } else { __state = 112; }
                    }
                    128 => {
                        unsafe { sqlite3_src_item_column_used(p_item, j) };
                        __state = 112;
                    }
                    129 => {
                        { let __p = &mut cnt_tab; let __t = *__p; *__p += 1; __t };
                        __state = 130;
                    }
                    130 => { p_match = p_item; __state = 47; }
                    131 => {
                        unsafe {
                            (*p_expr_1).i_table = unsafe { (*p_match).i_cursor }
                        };
                        __state = 132;
                    }
                    132 => { { let _ = 0; }; __state = 133; }
                    133 => {
                        unsafe {
                            (*p_expr_1).y.p_tab = unsafe { (*p_match).p_s_tab }
                        };
                        __state = 134;
                    }
                    134 => {
                        if unsafe { (*p_match).fg.jointype } as i32 & (8 | 64) != 0
                            {
                            __state = 136;
                        } else { __state = 135; }
                    }
                    135 => {
                        p_schema =
                            unsafe { (*unsafe { (*p_expr_1).y.p_tab }).p_schema };
                        __state = 42;
                    }
                    136 => {
                        unsafe { (*p_expr_1).flags |= 2097152 as u32 };
                        __state = 135;
                    }
                    137 => {
                        if cnt == 0 && cnt_tab >= 1 && !(p_match).is_null() &&
                                        unsafe { (*p_nc_1).nc_flags } & (32 | 8) == 0 &&
                                    unsafe { sqlite3_is_rowid(z_col) } != 0 &&
                                (unsafe { (*unsafe { (*p_match).p_s_tab }).tab_flags } &
                                            512 as u32 == 0 as u32 ||
                                    unsafe { (*p_match).fg.is_nested_from() } != 0) {
                            __state = 199;
                        } else { __state = 198; }
                    }
                    138 => { p_tab = core::ptr::null_mut(); __state = 139; }
                    139 => {
                        if unsafe { (*p_parse_1).p_trigger_tab } !=
                                core::ptr::null_mut() {
                            __state = 141;
                        } else { __state = 140; }
                    }
                    140 => {
                        if unsafe { (*p_nc_1).nc_flags } & 512 != 0 &&
                                z_tab_1 != core::ptr::null() {
                            __state = 154;
                        } else { __state = 153; }
                    }
                    141 => {
                        op = unsafe { (*p_parse_1).e_trigger_op } as i32;
                        __state = 142;
                    }
                    142 => { { let _ = 0; }; __state = 143; }
                    143 => {
                        if unsafe { (*p_parse_1).b_returning() } != 0 {
                            __state = 144;
                        } else { __state = 145; }
                    }
                    144 => {
                        if unsafe { (*p_nc_1).nc_flags } & 1024 != 0 &&
                                (z_tab_1 == core::ptr::null() ||
                                        unsafe {
                                                sqlite3_str_i_cmp(z_tab_1,
                                                    unsafe { (*unsafe { (*p_parse_1).p_trigger_tab }).z_name }
                                                        as *const i8)
                                            } == 0 ||
                                    is_valid_schema_table_name(z_tab_1,
                                            unsafe { &*unsafe { (*p_parse_1).p_trigger_tab } },
                                            core::ptr::null()) != 0) {
                            __state = 146;
                        } else { __state = 140; }
                    }
                    145 => {
                        if op != 129 && !(z_tab_1).is_null() &&
                                unsafe {
                                        sqlite3_str_i_cmp(c"new".as_ptr() as *mut i8 as *const i8,
                                            z_tab_1)
                                    } == 0 {
                            __state = 148;
                        } else { __state = 149; }
                    }
                    146 => {
                        unsafe { (*p_expr_1).i_table = (op != 129) as i32 };
                        __state = 147;
                    }
                    147 => {
                        p_tab = unsafe { (*p_parse_1).p_trigger_tab };
                        __state = 140;
                    }
                    148 => {
                        unsafe { (*p_expr_1).i_table = 1 };
                        __state = 150;
                    }
                    149 => {
                        if op != 128 && !(z_tab_1).is_null() &&
                                unsafe {
                                        sqlite3_str_i_cmp(c"old".as_ptr() as *mut i8 as *const i8,
                                            z_tab_1)
                                    } == 0 {
                            __state = 151;
                        } else { __state = 140; }
                    }
                    150 => {
                        p_tab = unsafe { (*p_parse_1).p_trigger_tab };
                        __state = 140;
                    }
                    151 => {
                        unsafe { (*p_expr_1).i_table = 0 };
                        __state = 152;
                    }
                    152 => {
                        p_tab = unsafe { (*p_parse_1).p_trigger_tab };
                        __state = 140;
                    }
                    153 => {
                        if !(p_tab).is_null() {
                            __state = 158;
                        } else { __state = 137; }
                    }
                    154 => {
                        p_upsert =
                            unsafe { (*p_nc_1).u_nc.p_upsert } as *const Upsert;
                        __state = 155;
                    }
                    155 => {
                        if !(p_upsert).is_null() &&
                                unsafe {
                                        sqlite3_str_i_cmp(c"excluded".as_ptr() as *mut i8 as
                                                *const i8, z_tab_1)
                                    } == 0 {
                            __state = 156;
                        } else { __state = 153; }
                    }
                    156 => {
                        p_tab =
                            unsafe {
                                (*(unsafe {
                                                    (*unsafe { (*p_upsert).p_upsert_src }).a.as_ptr()
                                                } as *mut SrcItem).offset(0 as isize)).p_s_tab
                            };
                        __state = 157;
                    }
                    157 => {
                        unsafe { (*p_expr_1).i_table = 2 };
                        __state = 153;
                    }
                    158 => { __state = 159; }
                    159 => {
                        p_schema = unsafe { (*p_tab).p_schema };
                        __state = 160;
                    }
                    160 => {
                        { let __p = &mut cnt_tab; let __t = *__p; *__p += 1; __t };
                        __state = 161;
                    }
                    161 => {
                        i_col = unsafe { sqlite3_column_index(p_tab, z_col) };
                        __state = 162;
                    }
                    162 => {
                        if i_col >= 0 { __state = 164; } else { __state = 165; }
                    }
                    163 => {
                        if i_col < unsafe { (*p_tab).n_col } as i32 {
                            __state = 169;
                        } else { __state = 137; }
                    }
                    164 => {
                        if unsafe { (*p_tab).i_p_key } as i32 == i_col {
                            __state = 166;
                        } else { __state = 163; }
                    }
                    165 => {
                        if unsafe { sqlite3_is_rowid(z_col) } != 0 &&
                                unsafe { (*p_tab).tab_flags } & 512 as u32 == 0 as u32 {
                            __state = 167;
                        } else { __state = 168; }
                    }
                    166 => { i_col = -1; __state = 163; }
                    167 => { i_col = -1; __state = 163; }
                    168 => {
                        i_col = unsafe { (*p_tab).n_col } as i32;
                        __state = 163;
                    }
                    169 => {
                        { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                        __state = 170;
                    }
                    170 => { p_match = core::ptr::null_mut(); __state = 171; }
                    171 => {
                        if unsafe { (*p_expr_1).i_table } == 2 {
                            __state = 172;
                        } else { __state = 173; }
                    }
                    172 => { __state = 174; }
                    173 => { { let _ = 0; }; __state = 181; }
                    174 => { { let _ = 0; }; __state = 175; }
                    175 => {
                        if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
                            __state = 176;
                        } else { __state = 177; }
                    }
                    176 => {
                        unsafe { (*p_expr_1).i_column = i_col as ynVar };
                        __state = 178;
                    }
                    177 => {
                        unsafe {
                            (*p_expr_1).i_table =
                                unsafe { (*unsafe { (*p_nc_1).u_nc.p_upsert }).reg_data } +
                                    unsafe {
                                            sqlite3_table_column_to_storage(p_tab, i_col as i16)
                                        } as i32
                        };
                        __state = 180;
                    }
                    178 => {
                        unsafe { (*p_expr_1).y.p_tab = p_tab };
                        __state = 179;
                    }
                    179 => { e_new_expr_op = 168; __state = 137; }
                    180 => { e_new_expr_op = 176; __state = 137; }
                    181 => {
                        unsafe { (*p_expr_1).y.p_tab = p_tab };
                        __state = 182;
                    }
                    182 => {
                        if unsafe { (*p_parse_1).b_returning() } != 0 {
                            __state = 183;
                        } else { __state = 184; }
                    }
                    183 => { e_new_expr_op = 176; __state = 185; }
                    184 => {
                        unsafe { (*p_expr_1).i_column = i_col as i16 };
                        __state = 188;
                    }
                    185 => {
                        unsafe { (*p_expr_1).op2 = 168 as u8 };
                        __state = 186;
                    }
                    186 => {
                        unsafe { (*p_expr_1).i_column = i_col as ynVar };
                        __state = 187;
                    }
                    187 => {
                        unsafe {
                            (*p_expr_1).i_table =
                                unsafe { (*p_nc_1).u_nc.i_base_reg } +
                                            (unsafe { (*p_tab).n_col } as i32 + 1) *
                                                unsafe { (*p_expr_1).i_table } +
                                        unsafe {
                                                sqlite3_table_column_to_storage(p_tab, i_col as i16)
                                            } as i32 + 1
                        };
                        __state = 137;
                    }
                    188 => { e_new_expr_op = 78; __state = 189; }
                    189 => {
                        if i_col < 0 { __state = 190; } else { __state = 191; }
                    }
                    190 => {
                        unsafe { (*p_expr_1).aff_expr = 68 as i8 };
                        __state = 137;
                    }
                    191 => {
                        if unsafe { (*p_expr_1).i_table } == 0 {
                            __state = 192;
                        } else { __state = 193; }
                    }
                    192 => { __state = 194; }
                    193 => { __state = 196; }
                    194 => { __state = 195; }
                    195 => {
                        unsafe {
                            (*p_parse_1).oldmask |=
                                if i_col >= 32 {
                                    4294967295u32
                                } else { (1 as u32) << i_col }
                        };
                        __state = 137;
                    }
                    196 => { __state = 197; }
                    197 => {
                        unsafe {
                            (*p_parse_1).newmask |=
                                if i_col >= 32 {
                                    4294967295u32
                                } else { (1 as u32) << i_col }
                        };
                        __state = 137;
                    }
                    198 => {
                        if cnt == 0 && unsafe { (*p_nc_1).nc_flags } & 128 != 0 &&
                                z_tab_1 == core::ptr::null() {
                            __state = 204;
                        } else { __state = 203; }
                    }
                    199 => { cnt = cnt_tab; __state = 200; }
                    200 => {
                        if unsafe { (*p_match).fg.is_nested_from() } as i32 == 0 {
                            __state = 202;
                        } else { __state = 201; }
                    }
                    201 => {
                        unsafe { (*p_expr_1).aff_expr = 68 as i8 };
                        __state = 198;
                    }
                    202 => {
                        unsafe { (*p_expr_1).i_column = -1 as ynVar };
                        __state = 201;
                    }
                    203 => {
                        if cnt != 0 { __state = 233; } else { __state = 232; }
                    }
                    204 => {
                        p_e_list = unsafe { (*p_nc_1).u_nc.p_e_list };
                        __state = 205;
                    }
                    205 => { { let _ = 0; }; __state = 206; }
                    206 => { j = 0; __state = 207; }
                    207 => {
                        if j < unsafe { (*p_e_list).n_expr } {
                            __state = 208;
                        } else { __state = 203; }
                    }
                    208 => {
                        z_as =
                            unsafe {
                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(j as isize)).z_e_name
                            };
                        __state = 210;
                    }
                    209 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 207;
                    }
                    210 => {
                        if unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                *mut ExprList_item).offset(j as isize)).fg.e_e_name()
                                        } as i32 == 0 &&
                                unsafe { sqlite3_stricmp(z_as as *const i8, z_col) } == 0 {
                            __state = 211;
                        } else { __state = 209; }
                    }
                    211 => { __state = 212; }
                    212 => { { let _ = 0; }; __state = 213; }
                    213 => { { let _ = 0; }; __state = 214; }
                    214 => { { let _ = 0; }; __state = 215; }
                    215 => {
                        p_orig =
                            unsafe {
                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(j as isize)).p_expr
                            };
                        __state = 216;
                    }
                    216 => {
                        if unsafe { (*p_nc_1).nc_flags } & 1 == 0 &&
                                unsafe { (*p_orig).flags } & 16 as u32 != 0 as u32 {
                            __state = 218;
                        } else { __state = 217; }
                    }
                    217 => {
                        if unsafe { (*p_orig).flags } & 32768 as u32 != 0 as u32 &&
                                (unsafe { (*p_nc_1).nc_flags } & 16384 == 0 ||
                                    p_nc_1 != p_top_nc) {
                            __state = 221;
                        } else { __state = 220; }
                    }
                    218 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"misuse of aliased aggregate %s".as_ptr() as *mut i8 as
                                    *const i8, z_as)
                        };
                        __state = 219;
                    }
                    219 => { return 2; }
                    220 => {
                        if unsafe {
                                    sqlite3_expr_vector_size(p_orig as *const Expr)
                                } != 1 {
                            __state = 224;
                        } else { __state = 223; }
                    }
                    221 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"misuse of aliased window function %s".as_ptr() as *mut i8
                                    as *const i8, z_as)
                        };
                        __state = 222;
                    }
                    222 => { return 2; }
                    223 => {
                        resolve_alias(p_parse_1, unsafe { &*p_e_list }, j, p_expr_1,
                            n_subquery);
                        __state = 226;
                    }
                    224 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"row value misused".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 225;
                    }
                    225 => { return 2; }
                    226 => { cnt = 1; __state = 227; }
                    227 => { p_match = core::ptr::null_mut(); __state = 228; }
                    228 => { { let _ = 0; }; __state = 229; }
                    229 => {
                        if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
                            __state = 231;
                        } else { __state = 230; }
                    }
                    230 => { __state = 2; }
                    231 => {
                        unsafe {
                            sqlite3_rename_token_remap(p_parse_1, core::ptr::null(),
                                p_expr_1 as *mut () as *const ())
                        };
                        __state = 230;
                    }
                    232 => {
                        p_nc_1 = unsafe { (*p_nc_1).p_next };
                        __state = 234;
                    }
                    233 => { __state = 38; }
                    234 => {
                        {
                            let __p = &mut n_subquery;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 39;
                    }
                    235 => { { let _ = 0; }; __state = 244; }
                    236 => { { let _ = 0; }; __state = 237; }
                    237 => {
                        if unsafe { (*p_expr_1).flags } & 128 as u32 != 0 as u32 &&
                                are_double_quoted_strings_enabled(db, unsafe { &*p_top_nc })
                                    != 0 {
                            __state = 239;
                        } else { __state = 238; }
                    }
                    238 => {
                        if unsafe { sqlite3_expr_id_to_true_false(p_expr_1) } != 0 {
                            __state = 243;
                        } else { __state = 235; }
                    }
                    239 => {
                        unsafe {
                            sqlite3_log(28,
                                c"double-quoted string literal: \"%w\"".as_ptr() as *mut i8
                                    as *const i8, z_col)
                        };
                        __state = 240;
                    }
                    240 => {
                        unsafe { (*p_expr_1).op = 118 as u8 };
                        __state = 241;
                    }
                    241 => {
                        unsafe {
                            memset(unsafe { &raw mut (*p_expr_1).y } as *mut (), 0, 8)
                        };
                        __state = 242;
                    }
                    242 => { return 1; }
                    243 => { return 1; }
                    244 => { { let _ = 0; }; __state = 245; }
                    245 => {
                        if cnt != 1 { __state = 247; } else { __state = 246; }
                    }
                    246 => { { let _ = 0; }; __state = 277; }
                    247 => { __state = 248; }
                    248 => {
                        if !(p_fj_match).is_null() {
                            __state = 250;
                        } else { __state = 249; }
                    }
                    249 => {
                        z_err =
                            if cnt == 0 {
                                    c"no such column".as_ptr() as *mut i8
                                } else { c"ambiguous column name".as_ptr() as *mut i8 } as
                                *const i8;
                        __state = 266;
                    }
                    250 => {
                        if unsafe { (*p_fj_match).n_expr } == cnt - 1 {
                            __state = 251;
                        } else { __state = 252; }
                    }
                    251 => {
                        if unsafe { (*p_expr_1).flags } & 8388608 as u32 != 0 as u32
                            {
                            __state = 254;
                        } else { __state = 255; }
                    }
                    252 => {
                        unsafe { sqlite3_expr_list_delete(db, p_fj_match) };
                        __state = 265;
                    }
                    253 => {
                        extend_fj_match(p_parse_1, &mut p_fj_match,
                            unsafe { &*p_match }, unsafe { (*p_expr_1).i_column });
                        __state = 259;
                    }
                    254 => {
                        unsafe { (*p_expr_1).flags &= !(8388608 as u32) };
                        __state = 253;
                    }
                    255 => {
                        unsafe {
                            sqlite3_expr_delete(db, unsafe { (*p_expr_1).p_left })
                        };
                        __state = 256;
                    }
                    256 => {
                        unsafe { (*p_expr_1).p_left = core::ptr::null_mut() };
                        __state = 257;
                    }
                    257 => {
                        unsafe {
                            sqlite3_expr_delete(db, unsafe { (*p_expr_1).p_right })
                        };
                        __state = 258;
                    }
                    258 => {
                        unsafe { (*p_expr_1).p_right = core::ptr::null_mut() };
                        __state = 253;
                    }
                    259 => {
                        unsafe { (*p_expr_1).op = 172 as u8 };
                        __state = 260;
                    }
                    260 => {
                        unsafe {
                            (*p_expr_1).u.z_token = c"coalesce".as_ptr() as *mut i8
                        };
                        __state = 261;
                    }
                    261 => {
                        unsafe { (*p_expr_1).x.p_list = p_fj_match };
                        __state = 262;
                    }
                    262 => {
                        unsafe { (*p_expr_1).aff_expr = 88 as i8 };
                        __state = 263;
                    }
                    263 => { cnt = 1; __state = 264; }
                    264 => { __state = 2; }
                    265 => {
                        p_fj_match = core::ptr::null_mut();
                        __state = 249;
                    }
                    266 => {
                        if !(z_db_1).is_null() {
                            __state = 268;
                        } else { __state = 269; }
                    }
                    267 => {
                        unsafe {
                            sqlite3_record_error_offset_of_expr(unsafe {
                                    (*p_parse_1).db
                                }, p_expr_1 as *const Expr)
                        };
                        __state = 274;
                    }
                    268 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"%s: %s.%s.%s".as_ptr() as *mut i8 as *const i8, z_err,
                                z_db_1, z_tab_1, z_col)
                        };
                        __state = 267;
                    }
                    269 => {
                        if !(z_tab_1).is_null() {
                            __state = 270;
                        } else { __state = 271; }
                    }
                    270 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"%s: %s.%s".as_ptr() as *mut i8 as *const i8, z_err,
                                z_tab_1, z_col)
                        };
                        __state = 267;
                    }
                    271 => {
                        if cnt == 0 && (*p_right_1).flags & 128 as u32 != 0 as u32 {
                            __state = 272;
                        } else { __state = 273; }
                    }
                    272 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"%s: \"%s\" - should this be a string literal in single-quotes?".as_ptr()
                                        as *mut i8 as *const i8, z_err, z_col)
                        };
                        __state = 267;
                    }
                    273 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"%s: %s".as_ptr() as *mut i8 as *const i8, z_err, z_col)
                        };
                        __state = 267;
                    }
                    274 => {
                        unsafe { (*p_parse_1).set_check_schema(1 as bft as u32) };
                        __state = 275;
                    }
                    275 => {
                        {
                            let __p = unsafe { &mut (*p_top_nc).n_nc_err };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 276;
                    }
                    276 => { e_new_expr_op = 122; __state = 246; }
                    277 => {
                        if !(unsafe { (*p_expr_1).flags } & (65536 | 8388608) as u32
                                            != 0 as u32) as i32 != 0 {
                            __state = 279;
                        } else { __state = 278; }
                    }
                    278 => {
                        if !(p_match).is_null() {
                            __state = 285;
                        } else { __state = 284; }
                    }
                    279 => {
                        unsafe {
                            sqlite3_expr_delete(db, unsafe { (*p_expr_1).p_left })
                        };
                        __state = 280;
                    }
                    280 => {
                        unsafe { (*p_expr_1).p_left = core::ptr::null_mut() };
                        __state = 281;
                    }
                    281 => {
                        unsafe {
                            sqlite3_expr_delete(db, unsafe { (*p_expr_1).p_right })
                        };
                        __state = 282;
                    }
                    282 => {
                        unsafe { (*p_expr_1).p_right = core::ptr::null_mut() };
                        __state = 283;
                    }
                    283 => {
                        unsafe { (*p_expr_1).flags |= 8388608 as u32 };
                        __state = 278;
                    }
                    284 => {
                        unsafe { (*p_expr_1).op = e_new_expr_op as u8 };
                        __state = 288;
                    }
                    285 => {
                        if unsafe { (*p_expr_1).i_column } as i32 >= 0 {
                            __state = 286;
                        } else { __state = 287; }
                    }
                    286 => {
                        unsafe {
                            (*p_match).col_used |=
                                sqlite3_expr_col_used(unsafe { &*p_expr_1 })
                        };
                        __state = 284;
                    }
                    287 => {
                        unsafe { (*p_match).fg.set_rowid_used(1 as u32 as u32) };
                        __state = 284;
                    }
                    288 => { __state = 2; }
                    289 => { { let _ = 0; }; __state = 291; }
                    290 => { return 2; }
                    291 => {
                        if unsafe { (*db).x_auth.is_some() } {
                            __state = 293;
                        } else { __state = 292; }
                    }
                    292 => { __state = 302; }
                    293 => {
                        if !(p_fj_match).is_null() {
                            __state = 295;
                        } else { __state = 294; }
                    }
                    294 => {
                        if unsafe { (*p_expr_1).op } as i32 == 168 ||
                                unsafe { (*p_expr_1).op } as i32 == 78 {
                            __state = 300;
                        } else { __state = 292; }
                    }
                    295 => { { let _ = 0; }; __state = 296; }
                    296 => { { let _ = 0; }; __state = 297; }
                    297 => { { let _ = 0; }; __state = 298; }
                    298 => { { let _ = 0; }; __state = 299; }
                    299 => {
                        p_expr_1 =
                            unsafe {
                                (*(unsafe { (*p_fj_match).a.as_ptr() } as
                                                *mut ExprList_item).offset(0 as isize)).p_expr
                            };
                        __state = 294;
                    }
                    300 => {
                        unsafe {
                            sqlite3_auth_read(p_parse_1, p_expr_1, p_schema,
                                unsafe { (*p_nc_1).p_src_list })
                        };
                        __state = 292;
                    }
                    301 => { return 1; }
                    302 => { __state = 303; }
                    303 => { { let _ = 0; }; __state = 305; }
                    304 => { __state = 302; }
                    305 => {
                        {
                            let __p = unsafe { &mut (*p_top_nc).n_ref };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 306;
                    }
                    306 => {
                        if p_top_nc == p_nc_1 {
                            __state = 308;
                        } else { __state = 307; }
                    }
                    307 => {
                        p_top_nc = unsafe { (*p_top_nc).p_next };
                        __state = 304;
                    }
                    308 => { __state = 301; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
extern "C" fn expr_probability(p: &Expr) -> i32 {
    unsafe {
        let mut r: f64 = -1.0;
        if (*p).op as i32 != 154 { return -1; }
        { let _ = 0; };
        unsafe { sqlite3_ato_f((*p).u.z_token as *const i8, &mut r) };
        { let _ = 0; };
        if r > 1.0 { return -1; }
        return (r * 134217728.0) as i32;
    }
}
extern "C" fn resolve_set_expr_subtype_arg(p_list_1: *const ExprList) -> () {
    unsafe {
        let mut nn: i32 = 0;
        let mut ii: i32 = 0;
        nn =
            if !(p_list_1).is_null() {
                unsafe { (*p_list_1).n_expr }
            } else { 0 };
        {
            ii = 0;
            '__b4: loop {
                if !(ii < nn) { break '__b4; }
                '__c4: loop {
                    let p_expr: *mut Expr =
                        unsafe {
                            (*(unsafe { (*p_list_1).a.as_ptr() } as
                                            *mut ExprList_item).offset(ii as isize)).p_expr
                        };
                    unsafe { (*p_expr).flags |= 2147483648u32 as u32 };
                    if unsafe { (*p_expr).op } as i32 == 139 {
                        { let _ = 0; };
                        { let _ = 0; };
                        resolve_set_expr_subtype_arg(unsafe {
                                    (*unsafe { (*p_expr).x.p_select }).p_e_list
                                } as *const ExprList);
                    }
                    break '__c4;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
extern "C" fn resolve_expr_step(p_walker_1: *mut Walker, p_expr_1: *mut Expr)
    -> i32 {
    unsafe {
        let mut p_nc: *mut NameContext = core::ptr::null_mut();
        let mut p_parse: *mut Parse = core::ptr::null_mut();
        p_nc = unsafe { (*p_walker_1).u.p_nc };
        { let _ = 0; };
        p_parse = unsafe { (*p_nc).p_parse };
        { let _ = 0; };
        '__s5:
            {
            match unsafe { (*p_expr_1).op } {
                76 => {
                    {
                        let p_src_list: *mut SrcList =
                            unsafe { (*p_nc).p_src_list };
                        let mut p_item: *const SrcItem = core::ptr::null();
                        { let _ = 0; };
                        p_item =
                            unsafe { (*p_src_list).a.as_ptr() } as *mut SrcItem;
                        unsafe { (*p_expr_1).op = 168 as u8 };
                        { let _ = 0; };
                        unsafe {
                            (*p_expr_1).y.p_tab = unsafe { (*p_item).p_s_tab }
                        };
                        unsafe {
                            (*p_expr_1).i_table = unsafe { (*p_item).i_cursor }
                        };
                        {
                            let __p = unsafe { &mut (*p_expr_1).i_column };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe { (*p_expr_1).aff_expr = 68 as i8 };
                        break '__s5;
                    }
                    {
                        let mut an_ref: [i32; 8] = [0; 8];
                        let mut p: *mut NameContext = core::ptr::null_mut();
                        let mut i: i32 = 0;
                        {
                            { i = 0; p = p_nc };
                            '__b6: loop {
                                if !(!(p).is_null() &&
                                                i <
                                                    (core::mem::size_of::<[i32; 8]>() as u64 /
                                                            core::mem::size_of::<i32>() as u64) as i32) {
                                    break '__b6;
                                }
                                '__c6: loop {
                                    an_ref[i as usize] = unsafe { (*p).n_ref };
                                    break '__c6;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        unsafe {
                            sqlite3_walk_expr(p_walker_1, unsafe { (*p_expr_1).p_left })
                        };
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                            return 1;
                        }
                        if unsafe {
                                    sqlite3_expr_can_be_null(unsafe { (*p_expr_1).p_left } as
                                            *const Expr)
                                } != 0 {
                            return 1;
                        }
                        {
                            { i = 0; p = p_nc };
                            '__b7: loop {
                                if !(!(p).is_null()) { break '__b7; }
                                '__c7: loop {
                                    if unsafe { (*p).nc_flags } & 1048576 == 0 { return 1; }
                                    break '__c7;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        { let _ = 0; };
                        unsafe {
                            (*p_expr_1).u.i_value =
                                (unsafe { (*p_expr_1).op } as i32 == 52) as i32
                        };
                        unsafe { (*p_expr_1).flags |= 2048 as u32 };
                        unsafe { (*p_expr_1).op = 156 as u8 };
                        {
                            { i = 0; p = p_nc };
                            '__b8: loop {
                                if !(!(p).is_null() &&
                                                i <
                                                    (core::mem::size_of::<[i32; 8]>() as u64 /
                                                            core::mem::size_of::<i32>() as u64) as i32) {
                                    break '__b8;
                                }
                                '__c8: loop {
                                    unsafe { (*p).n_ref = an_ref[i as usize] };
                                    break '__c8;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                unsafe { (*p_expr_1).p_left })
                        };
                        unsafe { (*p_expr_1).p_left = core::ptr::null_mut() };
                        return 1;
                    }
                    {
                        let mut z_table: *const i8 = core::ptr::null();
                        let mut z_db: *const i8 = core::ptr::null();
                        let mut p_right: *mut Expr = core::ptr::null_mut();
                        if unsafe { (*p_expr_1).op } as i32 == 60 {
                            z_db = core::ptr::null();
                            z_table = core::ptr::null();
                            { let _ = 0; };
                            p_right = p_expr_1;
                        } else {
                            let mut p_left: *mut Expr = unsafe { (*p_expr_1).p_left };
                            { let _ = 0; };
                            if unsafe { (*p_nc).nc_flags } & (32 | 8) != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"the \".\" operator".as_ptr() as *mut i8 as *const i8,
                                    core::ptr::null_mut(), p_expr_1 as *const Expr);
                            }
                            p_right = unsafe { (*p_expr_1).p_right };
                            if unsafe { (*p_right).op } as i32 == 60 {
                                z_db = core::ptr::null();
                            } else {
                                { let _ = 0; };
                                { let _ = 0; };
                                z_db = unsafe { (*p_left).u.z_token } as *const i8;
                                p_left = unsafe { (*p_right).p_left };
                                p_right = unsafe { (*p_right).p_right };
                            }
                            { let _ = 0; };
                            z_table = unsafe { (*p_left).u.z_token } as *const i8;
                            { let _ = 0; };
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        p_expr_1 as *mut () as *const (),
                                        p_right as *mut () as *const ())
                                };
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        unsafe { &raw mut (*p_expr_1).y.p_tab } as *mut () as
                                            *const (), p_left as *mut () as *const ())
                                };
                            }
                        }
                        return lookup_name(p_parse, z_db, z_table,
                                unsafe { &*p_right }, p_nc, p_expr_1);
                    }
                    {
                        let mut p_list: *mut ExprList = core::ptr::null_mut();
                        let mut n: i32 = 0;
                        let mut no_such_func: i32 = 0;
                        let mut wrong_num_args: i32 = 0;
                        let mut is_agg: i32 = 0;
                        let mut z_id: *const i8 = core::ptr::null();
                        let mut p_def: *mut FuncDef = core::ptr::null_mut();
                        let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                        let saved_allow_flags: i32 =
                            unsafe { (*p_nc).nc_flags } & (1 | 16384);
                        let p_win: *mut Window =
                            if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 &&
                                    unsafe { (*unsafe { (*p_expr_1).y.p_win }).e_frm_type } as
                                            i32 != 167 {
                                unsafe { (*p_expr_1).y.p_win }
                            } else { core::ptr::null_mut() };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_list = unsafe { (*p_expr_1).x.p_list };
                        n =
                            if !(p_list).is_null() {
                                unsafe { (*p_list).n_expr }
                            } else { 0 };
                        z_id = unsafe { (*p_expr_1).u.z_token } as *const i8;
                        p_def =
                            unsafe {
                                sqlite3_find_function(unsafe { (*p_parse).db }, z_id, n,
                                    enc, 0 as u8)
                            };
                        if p_def == core::ptr::null_mut() {
                            p_def =
                                unsafe {
                                    sqlite3_find_function(unsafe { (*p_parse).db }, z_id, -2,
                                        enc, 0 as u8)
                                };
                            if p_def == core::ptr::null_mut() {
                                no_such_func = 1;
                            } else { wrong_num_args = 1; }
                        } else {
                            is_agg = unsafe { (*p_def).x_finalize.is_some() } as i32;
                            if unsafe { (*p_def).func_flags } & 1024 as u32 != 0 {
                                unsafe { (*p_expr_1).flags |= 524288 as u32 };
                                if n == 2 {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            expr_probability(unsafe {
                                                    &*unsafe {
                                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                *mut ExprList_item).offset(1 as isize)).p_expr
                                                            }
                                                })
                                    };
                                    if unsafe { (*p_expr_1).i_table } < 0 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"second argument to %#T() must be a constant between 0.0 and 1.0".as_ptr()
                                                        as *mut i8 as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                } else {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            if unsafe { *unsafe { (*p_def).z_name.offset(0 as isize) } }
                                                        as i32 == 'u' as i32 {
                                                8388608
                                            } else { 125829120 }
                                    };
                                }
                            }
                            {
                                let auth: i32 =
                                    unsafe {
                                        sqlite3_auth_check(p_parse, 31, core::ptr::null(),
                                            unsafe { (*p_def).z_name }, core::ptr::null())
                                    };
                                if auth != 0 {
                                    if auth == 1 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"not authorized to use function: %#T".as_ptr() as *mut i8
                                                    as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                    unsafe { (*p_expr_1).op = 122 as u8 };
                                    return 1;
                                }
                            }
                            if unsafe { (*p_def).func_flags } & 1048576 as u32 != 0 ||
                                    unsafe { (*p_expr_1).flags } & 2147483648u32 as u32 !=
                                        0 as u32 {
                                resolve_set_expr_subtype_arg(p_list as *const ExprList);
                            }
                            if unsafe { (*p_def).func_flags } & (2048 | 8192) as u32 !=
                                    0 {
                                unsafe { (*p_expr_1).flags |= 1048576 as u32 };
                            }
                            if unsafe { (*p_def).func_flags } & 2048 as u32 == 0 as u32
                                {
                                { let _ = 0; };
                                if unsafe { (*p_nc).nc_flags } & (32 | 2 | 8) != 0 {
                                    not_valid_impl(p_parse, unsafe { &*p_nc },
                                        c"non-deterministic functions".as_ptr() as *mut i8 as
                                            *const i8, core::ptr::null_mut(), p_expr_1 as *const Expr);
                                }
                            } else {
                                { let _ = 0; };
                                unsafe {
                                    (*p_expr_1).op2 = (unsafe { (*p_nc).nc_flags } & 46) as u8
                                };
                            }
                            if unsafe { (*p_def).func_flags } & 262144 as u32 !=
                                            0 as u32 && unsafe { (*p_parse).nested } as i32 == 0 &&
                                    unsafe { (*unsafe { (*p_parse).db }).m_db_flags } &
                                            32 as u32 == 0 as u32 {
                                no_such_func = 2;
                                p_def = core::ptr::null_mut();
                            } else if unsafe { (*p_def).func_flags } &
                                            (524288 | 2097152) as u32 != 0 as u32 &&
                                    !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 !=
                                        0 {
                                if unsafe { (*p_nc).nc_flags } & 262144 != 0 {
                                    unsafe { (*p_expr_1).flags |= 1073741824 as u32 };
                                }
                                unsafe {
                                    sqlite3_expr_function_usable(p_parse,
                                        p_expr_1 as *const Expr, p_def as *const FuncDef)
                                };
                            }
                        }
                        if 0 ==
                                (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 {
                            { let _ = 0; };
                            if !(p_def).is_null() &&
                                        !unsafe { (*p_def).x_value.is_some() } as i32 != 0 &&
                                    !(p_win).is_null() {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"%#T() may not be used as a window function".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg != 0 &&
                                            unsafe { (*p_nc).nc_flags } & 1 == 0 ||
                                        is_agg != 0 &&
                                                unsafe { (*p_def).func_flags } & 65536 as u32 != 0 &&
                                            (p_win).is_null() as i32 != 0 ||
                                    is_agg != 0 && !(p_win).is_null() &&
                                        unsafe { (*p_nc).nc_flags } & 16384 == 0 {
                                let mut z_type: *const i8 = core::ptr::null();
                                if unsafe { (*p_def).func_flags } & 65536 as u32 != 0 ||
                                        !(p_win).is_null() {
                                    z_type = c"window".as_ptr() as *mut i8 as *const i8;
                                } else {
                                    z_type = c"aggregate".as_ptr() as *mut i8 as *const i8;
                                }
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"misuse of %s function %#T()".as_ptr() as *mut i8 as
                                            *const i8, z_type, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                is_agg = 0;
                            } else if no_such_func != 0 &&
                                    (unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                            0 ||
                                        no_such_func == 2 &&
                                            unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                                2) {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"no such function: %#T".as_ptr() as *mut i8 as *const i8,
                                        p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if wrong_num_args != 0 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"wrong number of arguments to function %#T()".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"FILTER may not be used with non-aggregate %#T()".as_ptr()
                                                as *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    !(unsafe { (*p_expr_1).p_left }).is_null() {
                                unsafe {
                                    sqlite3_expr_order_by_aggregate_error(p_parse, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                            if is_agg != 0 {
                                unsafe {
                                    (*p_nc).nc_flags &=
                                        !(16384 | if (p_win).is_null() as i32 != 0 { 1 } else { 0 })
                                };
                            }
                        } else if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                    0 as u32 || !(unsafe { (*p_expr_1).p_left }).is_null() {
                            is_agg = 1;
                        }
                        unsafe { sqlite3_walk_expr_list(p_walker_1, p_list) };
                        if is_agg != 0 {
                            if !(unsafe { (*p_expr_1).p_left }).is_null() {
                                { let _ = 0; };
                                { let _ = 0; };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list })
                                };
                            }
                            if !(p_win).is_null() && unsafe { (*p_parse).n_err } == 0 {
                                let p_sel: *mut Select = unsafe { (*p_nc).p_win_select };
                                { let _ = 0; };
                                if (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                        == 0 {
                                    unsafe {
                                        sqlite3_window_update(p_parse,
                                            if !(p_sel).is_null() {
                                                unsafe { (*p_sel).p_win_defn }
                                            } else { core::ptr::null_mut() }, p_win, p_def)
                                    };
                                    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                                        {
                                        break '__s5;
                                    }
                                }
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_partition })
                                };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_order_by })
                                };
                                unsafe {
                                    sqlite3_walk_expr(p_walker_1, unsafe { (*p_win).p_filter })
                                };
                                unsafe { sqlite3_window_link(p_sel, p_win) };
                                unsafe { (*p_nc).nc_flags |= 32768 };
                            } else {
                                let mut p_nc2: *mut NameContext = core::ptr::null_mut();
                                unsafe { (*p_expr_1).op = 169 as u8 };
                                unsafe { (*p_expr_1).op2 = 0 as u8 };
                                if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 {
                                    unsafe {
                                        sqlite3_walk_expr(p_walker_1,
                                            unsafe { (*unsafe { (*p_expr_1).y.p_win }).p_filter })
                                    };
                                }
                                p_nc2 = p_nc;
                                while !(p_nc2).is_null() &&
                                        unsafe {
                                                sqlite3_references_src_list(p_parse, p_expr_1,
                                                    unsafe { (*p_nc2).p_src_list })
                                            } == 0 {
                                    unsafe {
                                        (*p_expr_1).op2 +=
                                            (1 as u32 + unsafe { (*p_nc2).n_nested_select }) as u8
                                    };
                                    p_nc2 = unsafe { (*p_nc2).p_next };
                                }
                                { let _ = 0; };
                                if !(p_nc2).is_null() && !(p_def).is_null() {
                                    unsafe {
                                        (*p_expr_1).op2 += unsafe { (*p_nc2).n_nested_select } as u8
                                    };
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    unsafe {
                                        (*p_nc2).nc_flags |=
                                            (16 as u32 |
                                                    (unsafe { (*p_def).func_flags } ^ 134217728 as u32) &
                                                        (4096 | 134217728) as u32) as i32
                                    };
                                }
                            }
                            unsafe { (*p_nc).nc_flags |= saved_allow_flags };
                        }
                        return 1;
                    }
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                52 => {
                    {
                        let mut an_ref: [i32; 8] = [0; 8];
                        let mut p: *mut NameContext = core::ptr::null_mut();
                        let mut i: i32 = 0;
                        {
                            { i = 0; p = p_nc };
                            '__b6: loop {
                                if !(!(p).is_null() &&
                                                i <
                                                    (core::mem::size_of::<[i32; 8]>() as u64 /
                                                            core::mem::size_of::<i32>() as u64) as i32) {
                                    break '__b6;
                                }
                                '__c6: loop {
                                    an_ref[i as usize] = unsafe { (*p).n_ref };
                                    break '__c6;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        unsafe {
                            sqlite3_walk_expr(p_walker_1, unsafe { (*p_expr_1).p_left })
                        };
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                            return 1;
                        }
                        if unsafe {
                                    sqlite3_expr_can_be_null(unsafe { (*p_expr_1).p_left } as
                                            *const Expr)
                                } != 0 {
                            return 1;
                        }
                        {
                            { i = 0; p = p_nc };
                            '__b7: loop {
                                if !(!(p).is_null()) { break '__b7; }
                                '__c7: loop {
                                    if unsafe { (*p).nc_flags } & 1048576 == 0 { return 1; }
                                    break '__c7;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        { let _ = 0; };
                        unsafe {
                            (*p_expr_1).u.i_value =
                                (unsafe { (*p_expr_1).op } as i32 == 52) as i32
                        };
                        unsafe { (*p_expr_1).flags |= 2048 as u32 };
                        unsafe { (*p_expr_1).op = 156 as u8 };
                        {
                            { i = 0; p = p_nc };
                            '__b8: loop {
                                if !(!(p).is_null() &&
                                                i <
                                                    (core::mem::size_of::<[i32; 8]>() as u64 /
                                                            core::mem::size_of::<i32>() as u64) as i32) {
                                    break '__b8;
                                }
                                '__c8: loop {
                                    unsafe { (*p).n_ref = an_ref[i as usize] };
                                    break '__c8;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                unsafe { (*p_expr_1).p_left })
                        };
                        unsafe { (*p_expr_1).p_left = core::ptr::null_mut() };
                        return 1;
                    }
                    {
                        let mut z_table: *const i8 = core::ptr::null();
                        let mut z_db: *const i8 = core::ptr::null();
                        let mut p_right: *mut Expr = core::ptr::null_mut();
                        if unsafe { (*p_expr_1).op } as i32 == 60 {
                            z_db = core::ptr::null();
                            z_table = core::ptr::null();
                            { let _ = 0; };
                            p_right = p_expr_1;
                        } else {
                            let mut p_left: *mut Expr = unsafe { (*p_expr_1).p_left };
                            { let _ = 0; };
                            if unsafe { (*p_nc).nc_flags } & (32 | 8) != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"the \".\" operator".as_ptr() as *mut i8 as *const i8,
                                    core::ptr::null_mut(), p_expr_1 as *const Expr);
                            }
                            p_right = unsafe { (*p_expr_1).p_right };
                            if unsafe { (*p_right).op } as i32 == 60 {
                                z_db = core::ptr::null();
                            } else {
                                { let _ = 0; };
                                { let _ = 0; };
                                z_db = unsafe { (*p_left).u.z_token } as *const i8;
                                p_left = unsafe { (*p_right).p_left };
                                p_right = unsafe { (*p_right).p_right };
                            }
                            { let _ = 0; };
                            z_table = unsafe { (*p_left).u.z_token } as *const i8;
                            { let _ = 0; };
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        p_expr_1 as *mut () as *const (),
                                        p_right as *mut () as *const ())
                                };
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        unsafe { &raw mut (*p_expr_1).y.p_tab } as *mut () as
                                            *const (), p_left as *mut () as *const ())
                                };
                            }
                        }
                        return lookup_name(p_parse, z_db, z_table,
                                unsafe { &*p_right }, p_nc, p_expr_1);
                    }
                    {
                        let mut p_list: *mut ExprList = core::ptr::null_mut();
                        let mut n: i32 = 0;
                        let mut no_such_func: i32 = 0;
                        let mut wrong_num_args: i32 = 0;
                        let mut is_agg: i32 = 0;
                        let mut z_id: *const i8 = core::ptr::null();
                        let mut p_def: *mut FuncDef = core::ptr::null_mut();
                        let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                        let saved_allow_flags: i32 =
                            unsafe { (*p_nc).nc_flags } & (1 | 16384);
                        let p_win: *mut Window =
                            if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 &&
                                    unsafe { (*unsafe { (*p_expr_1).y.p_win }).e_frm_type } as
                                            i32 != 167 {
                                unsafe { (*p_expr_1).y.p_win }
                            } else { core::ptr::null_mut() };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_list = unsafe { (*p_expr_1).x.p_list };
                        n =
                            if !(p_list).is_null() {
                                unsafe { (*p_list).n_expr }
                            } else { 0 };
                        z_id = unsafe { (*p_expr_1).u.z_token } as *const i8;
                        p_def =
                            unsafe {
                                sqlite3_find_function(unsafe { (*p_parse).db }, z_id, n,
                                    enc, 0 as u8)
                            };
                        if p_def == core::ptr::null_mut() {
                            p_def =
                                unsafe {
                                    sqlite3_find_function(unsafe { (*p_parse).db }, z_id, -2,
                                        enc, 0 as u8)
                                };
                            if p_def == core::ptr::null_mut() {
                                no_such_func = 1;
                            } else { wrong_num_args = 1; }
                        } else {
                            is_agg = unsafe { (*p_def).x_finalize.is_some() } as i32;
                            if unsafe { (*p_def).func_flags } & 1024 as u32 != 0 {
                                unsafe { (*p_expr_1).flags |= 524288 as u32 };
                                if n == 2 {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            expr_probability(unsafe {
                                                    &*unsafe {
                                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                *mut ExprList_item).offset(1 as isize)).p_expr
                                                            }
                                                })
                                    };
                                    if unsafe { (*p_expr_1).i_table } < 0 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"second argument to %#T() must be a constant between 0.0 and 1.0".as_ptr()
                                                        as *mut i8 as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                } else {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            if unsafe { *unsafe { (*p_def).z_name.offset(0 as isize) } }
                                                        as i32 == 'u' as i32 {
                                                8388608
                                            } else { 125829120 }
                                    };
                                }
                            }
                            {
                                let auth: i32 =
                                    unsafe {
                                        sqlite3_auth_check(p_parse, 31, core::ptr::null(),
                                            unsafe { (*p_def).z_name }, core::ptr::null())
                                    };
                                if auth != 0 {
                                    if auth == 1 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"not authorized to use function: %#T".as_ptr() as *mut i8
                                                    as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                    unsafe { (*p_expr_1).op = 122 as u8 };
                                    return 1;
                                }
                            }
                            if unsafe { (*p_def).func_flags } & 1048576 as u32 != 0 ||
                                    unsafe { (*p_expr_1).flags } & 2147483648u32 as u32 !=
                                        0 as u32 {
                                resolve_set_expr_subtype_arg(p_list as *const ExprList);
                            }
                            if unsafe { (*p_def).func_flags } & (2048 | 8192) as u32 !=
                                    0 {
                                unsafe { (*p_expr_1).flags |= 1048576 as u32 };
                            }
                            if unsafe { (*p_def).func_flags } & 2048 as u32 == 0 as u32
                                {
                                { let _ = 0; };
                                if unsafe { (*p_nc).nc_flags } & (32 | 2 | 8) != 0 {
                                    not_valid_impl(p_parse, unsafe { &*p_nc },
                                        c"non-deterministic functions".as_ptr() as *mut i8 as
                                            *const i8, core::ptr::null_mut(), p_expr_1 as *const Expr);
                                }
                            } else {
                                { let _ = 0; };
                                unsafe {
                                    (*p_expr_1).op2 = (unsafe { (*p_nc).nc_flags } & 46) as u8
                                };
                            }
                            if unsafe { (*p_def).func_flags } & 262144 as u32 !=
                                            0 as u32 && unsafe { (*p_parse).nested } as i32 == 0 &&
                                    unsafe { (*unsafe { (*p_parse).db }).m_db_flags } &
                                            32 as u32 == 0 as u32 {
                                no_such_func = 2;
                                p_def = core::ptr::null_mut();
                            } else if unsafe { (*p_def).func_flags } &
                                            (524288 | 2097152) as u32 != 0 as u32 &&
                                    !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 !=
                                        0 {
                                if unsafe { (*p_nc).nc_flags } & 262144 != 0 {
                                    unsafe { (*p_expr_1).flags |= 1073741824 as u32 };
                                }
                                unsafe {
                                    sqlite3_expr_function_usable(p_parse,
                                        p_expr_1 as *const Expr, p_def as *const FuncDef)
                                };
                            }
                        }
                        if 0 ==
                                (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 {
                            { let _ = 0; };
                            if !(p_def).is_null() &&
                                        !unsafe { (*p_def).x_value.is_some() } as i32 != 0 &&
                                    !(p_win).is_null() {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"%#T() may not be used as a window function".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg != 0 &&
                                            unsafe { (*p_nc).nc_flags } & 1 == 0 ||
                                        is_agg != 0 &&
                                                unsafe { (*p_def).func_flags } & 65536 as u32 != 0 &&
                                            (p_win).is_null() as i32 != 0 ||
                                    is_agg != 0 && !(p_win).is_null() &&
                                        unsafe { (*p_nc).nc_flags } & 16384 == 0 {
                                let mut z_type: *const i8 = core::ptr::null();
                                if unsafe { (*p_def).func_flags } & 65536 as u32 != 0 ||
                                        !(p_win).is_null() {
                                    z_type = c"window".as_ptr() as *mut i8 as *const i8;
                                } else {
                                    z_type = c"aggregate".as_ptr() as *mut i8 as *const i8;
                                }
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"misuse of %s function %#T()".as_ptr() as *mut i8 as
                                            *const i8, z_type, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                is_agg = 0;
                            } else if no_such_func != 0 &&
                                    (unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                            0 ||
                                        no_such_func == 2 &&
                                            unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                                2) {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"no such function: %#T".as_ptr() as *mut i8 as *const i8,
                                        p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if wrong_num_args != 0 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"wrong number of arguments to function %#T()".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"FILTER may not be used with non-aggregate %#T()".as_ptr()
                                                as *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    !(unsafe { (*p_expr_1).p_left }).is_null() {
                                unsafe {
                                    sqlite3_expr_order_by_aggregate_error(p_parse, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                            if is_agg != 0 {
                                unsafe {
                                    (*p_nc).nc_flags &=
                                        !(16384 | if (p_win).is_null() as i32 != 0 { 1 } else { 0 })
                                };
                            }
                        } else if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                    0 as u32 || !(unsafe { (*p_expr_1).p_left }).is_null() {
                            is_agg = 1;
                        }
                        unsafe { sqlite3_walk_expr_list(p_walker_1, p_list) };
                        if is_agg != 0 {
                            if !(unsafe { (*p_expr_1).p_left }).is_null() {
                                { let _ = 0; };
                                { let _ = 0; };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list })
                                };
                            }
                            if !(p_win).is_null() && unsafe { (*p_parse).n_err } == 0 {
                                let p_sel: *mut Select = unsafe { (*p_nc).p_win_select };
                                { let _ = 0; };
                                if (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                        == 0 {
                                    unsafe {
                                        sqlite3_window_update(p_parse,
                                            if !(p_sel).is_null() {
                                                unsafe { (*p_sel).p_win_defn }
                                            } else { core::ptr::null_mut() }, p_win, p_def)
                                    };
                                    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                                        {
                                        break '__s5;
                                    }
                                }
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_partition })
                                };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_order_by })
                                };
                                unsafe {
                                    sqlite3_walk_expr(p_walker_1, unsafe { (*p_win).p_filter })
                                };
                                unsafe { sqlite3_window_link(p_sel, p_win) };
                                unsafe { (*p_nc).nc_flags |= 32768 };
                            } else {
                                let mut p_nc2: *mut NameContext = core::ptr::null_mut();
                                unsafe { (*p_expr_1).op = 169 as u8 };
                                unsafe { (*p_expr_1).op2 = 0 as u8 };
                                if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 {
                                    unsafe {
                                        sqlite3_walk_expr(p_walker_1,
                                            unsafe { (*unsafe { (*p_expr_1).y.p_win }).p_filter })
                                    };
                                }
                                p_nc2 = p_nc;
                                while !(p_nc2).is_null() &&
                                        unsafe {
                                                sqlite3_references_src_list(p_parse, p_expr_1,
                                                    unsafe { (*p_nc2).p_src_list })
                                            } == 0 {
                                    unsafe {
                                        (*p_expr_1).op2 +=
                                            (1 as u32 + unsafe { (*p_nc2).n_nested_select }) as u8
                                    };
                                    p_nc2 = unsafe { (*p_nc2).p_next };
                                }
                                { let _ = 0; };
                                if !(p_nc2).is_null() && !(p_def).is_null() {
                                    unsafe {
                                        (*p_expr_1).op2 += unsafe { (*p_nc2).n_nested_select } as u8
                                    };
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    unsafe {
                                        (*p_nc2).nc_flags |=
                                            (16 as u32 |
                                                    (unsafe { (*p_def).func_flags } ^ 134217728 as u32) &
                                                        (4096 | 134217728) as u32) as i32
                                    };
                                }
                            }
                            unsafe { (*p_nc).nc_flags |= saved_allow_flags };
                        }
                        return 1;
                    }
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                51 => {
                    {
                        let mut an_ref: [i32; 8] = [0; 8];
                        let mut p: *mut NameContext = core::ptr::null_mut();
                        let mut i: i32 = 0;
                        {
                            { i = 0; p = p_nc };
                            '__b6: loop {
                                if !(!(p).is_null() &&
                                                i <
                                                    (core::mem::size_of::<[i32; 8]>() as u64 /
                                                            core::mem::size_of::<i32>() as u64) as i32) {
                                    break '__b6;
                                }
                                '__c6: loop {
                                    an_ref[i as usize] = unsafe { (*p).n_ref };
                                    break '__c6;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        unsafe {
                            sqlite3_walk_expr(p_walker_1, unsafe { (*p_expr_1).p_left })
                        };
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                            return 1;
                        }
                        if unsafe {
                                    sqlite3_expr_can_be_null(unsafe { (*p_expr_1).p_left } as
                                            *const Expr)
                                } != 0 {
                            return 1;
                        }
                        {
                            { i = 0; p = p_nc };
                            '__b7: loop {
                                if !(!(p).is_null()) { break '__b7; }
                                '__c7: loop {
                                    if unsafe { (*p).nc_flags } & 1048576 == 0 { return 1; }
                                    break '__c7;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        { let _ = 0; };
                        unsafe {
                            (*p_expr_1).u.i_value =
                                (unsafe { (*p_expr_1).op } as i32 == 52) as i32
                        };
                        unsafe { (*p_expr_1).flags |= 2048 as u32 };
                        unsafe { (*p_expr_1).op = 156 as u8 };
                        {
                            { i = 0; p = p_nc };
                            '__b8: loop {
                                if !(!(p).is_null() &&
                                                i <
                                                    (core::mem::size_of::<[i32; 8]>() as u64 /
                                                            core::mem::size_of::<i32>() as u64) as i32) {
                                    break '__b8;
                                }
                                '__c8: loop {
                                    unsafe { (*p).n_ref = an_ref[i as usize] };
                                    break '__c8;
                                }
                                {
                                    p = unsafe { (*p).p_next };
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                };
                            }
                        }
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db },
                                unsafe { (*p_expr_1).p_left })
                        };
                        unsafe { (*p_expr_1).p_left = core::ptr::null_mut() };
                        return 1;
                    }
                    {
                        let mut z_table: *const i8 = core::ptr::null();
                        let mut z_db: *const i8 = core::ptr::null();
                        let mut p_right: *mut Expr = core::ptr::null_mut();
                        if unsafe { (*p_expr_1).op } as i32 == 60 {
                            z_db = core::ptr::null();
                            z_table = core::ptr::null();
                            { let _ = 0; };
                            p_right = p_expr_1;
                        } else {
                            let mut p_left: *mut Expr = unsafe { (*p_expr_1).p_left };
                            { let _ = 0; };
                            if unsafe { (*p_nc).nc_flags } & (32 | 8) != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"the \".\" operator".as_ptr() as *mut i8 as *const i8,
                                    core::ptr::null_mut(), p_expr_1 as *const Expr);
                            }
                            p_right = unsafe { (*p_expr_1).p_right };
                            if unsafe { (*p_right).op } as i32 == 60 {
                                z_db = core::ptr::null();
                            } else {
                                { let _ = 0; };
                                { let _ = 0; };
                                z_db = unsafe { (*p_left).u.z_token } as *const i8;
                                p_left = unsafe { (*p_right).p_left };
                                p_right = unsafe { (*p_right).p_right };
                            }
                            { let _ = 0; };
                            z_table = unsafe { (*p_left).u.z_token } as *const i8;
                            { let _ = 0; };
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        p_expr_1 as *mut () as *const (),
                                        p_right as *mut () as *const ())
                                };
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        unsafe { &raw mut (*p_expr_1).y.p_tab } as *mut () as
                                            *const (), p_left as *mut () as *const ())
                                };
                            }
                        }
                        return lookup_name(p_parse, z_db, z_table,
                                unsafe { &*p_right }, p_nc, p_expr_1);
                    }
                    {
                        let mut p_list: *mut ExprList = core::ptr::null_mut();
                        let mut n: i32 = 0;
                        let mut no_such_func: i32 = 0;
                        let mut wrong_num_args: i32 = 0;
                        let mut is_agg: i32 = 0;
                        let mut z_id: *const i8 = core::ptr::null();
                        let mut p_def: *mut FuncDef = core::ptr::null_mut();
                        let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                        let saved_allow_flags: i32 =
                            unsafe { (*p_nc).nc_flags } & (1 | 16384);
                        let p_win: *mut Window =
                            if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 &&
                                    unsafe { (*unsafe { (*p_expr_1).y.p_win }).e_frm_type } as
                                            i32 != 167 {
                                unsafe { (*p_expr_1).y.p_win }
                            } else { core::ptr::null_mut() };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_list = unsafe { (*p_expr_1).x.p_list };
                        n =
                            if !(p_list).is_null() {
                                unsafe { (*p_list).n_expr }
                            } else { 0 };
                        z_id = unsafe { (*p_expr_1).u.z_token } as *const i8;
                        p_def =
                            unsafe {
                                sqlite3_find_function(unsafe { (*p_parse).db }, z_id, n,
                                    enc, 0 as u8)
                            };
                        if p_def == core::ptr::null_mut() {
                            p_def =
                                unsafe {
                                    sqlite3_find_function(unsafe { (*p_parse).db }, z_id, -2,
                                        enc, 0 as u8)
                                };
                            if p_def == core::ptr::null_mut() {
                                no_such_func = 1;
                            } else { wrong_num_args = 1; }
                        } else {
                            is_agg = unsafe { (*p_def).x_finalize.is_some() } as i32;
                            if unsafe { (*p_def).func_flags } & 1024 as u32 != 0 {
                                unsafe { (*p_expr_1).flags |= 524288 as u32 };
                                if n == 2 {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            expr_probability(unsafe {
                                                    &*unsafe {
                                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                *mut ExprList_item).offset(1 as isize)).p_expr
                                                            }
                                                })
                                    };
                                    if unsafe { (*p_expr_1).i_table } < 0 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"second argument to %#T() must be a constant between 0.0 and 1.0".as_ptr()
                                                        as *mut i8 as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                } else {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            if unsafe { *unsafe { (*p_def).z_name.offset(0 as isize) } }
                                                        as i32 == 'u' as i32 {
                                                8388608
                                            } else { 125829120 }
                                    };
                                }
                            }
                            {
                                let auth: i32 =
                                    unsafe {
                                        sqlite3_auth_check(p_parse, 31, core::ptr::null(),
                                            unsafe { (*p_def).z_name }, core::ptr::null())
                                    };
                                if auth != 0 {
                                    if auth == 1 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"not authorized to use function: %#T".as_ptr() as *mut i8
                                                    as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                    unsafe { (*p_expr_1).op = 122 as u8 };
                                    return 1;
                                }
                            }
                            if unsafe { (*p_def).func_flags } & 1048576 as u32 != 0 ||
                                    unsafe { (*p_expr_1).flags } & 2147483648u32 as u32 !=
                                        0 as u32 {
                                resolve_set_expr_subtype_arg(p_list as *const ExprList);
                            }
                            if unsafe { (*p_def).func_flags } & (2048 | 8192) as u32 !=
                                    0 {
                                unsafe { (*p_expr_1).flags |= 1048576 as u32 };
                            }
                            if unsafe { (*p_def).func_flags } & 2048 as u32 == 0 as u32
                                {
                                { let _ = 0; };
                                if unsafe { (*p_nc).nc_flags } & (32 | 2 | 8) != 0 {
                                    not_valid_impl(p_parse, unsafe { &*p_nc },
                                        c"non-deterministic functions".as_ptr() as *mut i8 as
                                            *const i8, core::ptr::null_mut(), p_expr_1 as *const Expr);
                                }
                            } else {
                                { let _ = 0; };
                                unsafe {
                                    (*p_expr_1).op2 = (unsafe { (*p_nc).nc_flags } & 46) as u8
                                };
                            }
                            if unsafe { (*p_def).func_flags } & 262144 as u32 !=
                                            0 as u32 && unsafe { (*p_parse).nested } as i32 == 0 &&
                                    unsafe { (*unsafe { (*p_parse).db }).m_db_flags } &
                                            32 as u32 == 0 as u32 {
                                no_such_func = 2;
                                p_def = core::ptr::null_mut();
                            } else if unsafe { (*p_def).func_flags } &
                                            (524288 | 2097152) as u32 != 0 as u32 &&
                                    !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 !=
                                        0 {
                                if unsafe { (*p_nc).nc_flags } & 262144 != 0 {
                                    unsafe { (*p_expr_1).flags |= 1073741824 as u32 };
                                }
                                unsafe {
                                    sqlite3_expr_function_usable(p_parse,
                                        p_expr_1 as *const Expr, p_def as *const FuncDef)
                                };
                            }
                        }
                        if 0 ==
                                (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 {
                            { let _ = 0; };
                            if !(p_def).is_null() &&
                                        !unsafe { (*p_def).x_value.is_some() } as i32 != 0 &&
                                    !(p_win).is_null() {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"%#T() may not be used as a window function".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg != 0 &&
                                            unsafe { (*p_nc).nc_flags } & 1 == 0 ||
                                        is_agg != 0 &&
                                                unsafe { (*p_def).func_flags } & 65536 as u32 != 0 &&
                                            (p_win).is_null() as i32 != 0 ||
                                    is_agg != 0 && !(p_win).is_null() &&
                                        unsafe { (*p_nc).nc_flags } & 16384 == 0 {
                                let mut z_type: *const i8 = core::ptr::null();
                                if unsafe { (*p_def).func_flags } & 65536 as u32 != 0 ||
                                        !(p_win).is_null() {
                                    z_type = c"window".as_ptr() as *mut i8 as *const i8;
                                } else {
                                    z_type = c"aggregate".as_ptr() as *mut i8 as *const i8;
                                }
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"misuse of %s function %#T()".as_ptr() as *mut i8 as
                                            *const i8, z_type, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                is_agg = 0;
                            } else if no_such_func != 0 &&
                                    (unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                            0 ||
                                        no_such_func == 2 &&
                                            unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                                2) {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"no such function: %#T".as_ptr() as *mut i8 as *const i8,
                                        p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if wrong_num_args != 0 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"wrong number of arguments to function %#T()".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"FILTER may not be used with non-aggregate %#T()".as_ptr()
                                                as *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    !(unsafe { (*p_expr_1).p_left }).is_null() {
                                unsafe {
                                    sqlite3_expr_order_by_aggregate_error(p_parse, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                            if is_agg != 0 {
                                unsafe {
                                    (*p_nc).nc_flags &=
                                        !(16384 | if (p_win).is_null() as i32 != 0 { 1 } else { 0 })
                                };
                            }
                        } else if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                    0 as u32 || !(unsafe { (*p_expr_1).p_left }).is_null() {
                            is_agg = 1;
                        }
                        unsafe { sqlite3_walk_expr_list(p_walker_1, p_list) };
                        if is_agg != 0 {
                            if !(unsafe { (*p_expr_1).p_left }).is_null() {
                                { let _ = 0; };
                                { let _ = 0; };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list })
                                };
                            }
                            if !(p_win).is_null() && unsafe { (*p_parse).n_err } == 0 {
                                let p_sel: *mut Select = unsafe { (*p_nc).p_win_select };
                                { let _ = 0; };
                                if (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                        == 0 {
                                    unsafe {
                                        sqlite3_window_update(p_parse,
                                            if !(p_sel).is_null() {
                                                unsafe { (*p_sel).p_win_defn }
                                            } else { core::ptr::null_mut() }, p_win, p_def)
                                    };
                                    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                                        {
                                        break '__s5;
                                    }
                                }
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_partition })
                                };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_order_by })
                                };
                                unsafe {
                                    sqlite3_walk_expr(p_walker_1, unsafe { (*p_win).p_filter })
                                };
                                unsafe { sqlite3_window_link(p_sel, p_win) };
                                unsafe { (*p_nc).nc_flags |= 32768 };
                            } else {
                                let mut p_nc2: *mut NameContext = core::ptr::null_mut();
                                unsafe { (*p_expr_1).op = 169 as u8 };
                                unsafe { (*p_expr_1).op2 = 0 as u8 };
                                if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 {
                                    unsafe {
                                        sqlite3_walk_expr(p_walker_1,
                                            unsafe { (*unsafe { (*p_expr_1).y.p_win }).p_filter })
                                    };
                                }
                                p_nc2 = p_nc;
                                while !(p_nc2).is_null() &&
                                        unsafe {
                                                sqlite3_references_src_list(p_parse, p_expr_1,
                                                    unsafe { (*p_nc2).p_src_list })
                                            } == 0 {
                                    unsafe {
                                        (*p_expr_1).op2 +=
                                            (1 as u32 + unsafe { (*p_nc2).n_nested_select }) as u8
                                    };
                                    p_nc2 = unsafe { (*p_nc2).p_next };
                                }
                                { let _ = 0; };
                                if !(p_nc2).is_null() && !(p_def).is_null() {
                                    unsafe {
                                        (*p_expr_1).op2 += unsafe { (*p_nc2).n_nested_select } as u8
                                    };
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    unsafe {
                                        (*p_nc2).nc_flags |=
                                            (16 as u32 |
                                                    (unsafe { (*p_def).func_flags } ^ 134217728 as u32) &
                                                        (4096 | 134217728) as u32) as i32
                                    };
                                }
                            }
                            unsafe { (*p_nc).nc_flags |= saved_allow_flags };
                        }
                        return 1;
                    }
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                60 => {
                    {
                        let mut z_table: *const i8 = core::ptr::null();
                        let mut z_db: *const i8 = core::ptr::null();
                        let mut p_right: *mut Expr = core::ptr::null_mut();
                        if unsafe { (*p_expr_1).op } as i32 == 60 {
                            z_db = core::ptr::null();
                            z_table = core::ptr::null();
                            { let _ = 0; };
                            p_right = p_expr_1;
                        } else {
                            let mut p_left: *mut Expr = unsafe { (*p_expr_1).p_left };
                            { let _ = 0; };
                            if unsafe { (*p_nc).nc_flags } & (32 | 8) != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"the \".\" operator".as_ptr() as *mut i8 as *const i8,
                                    core::ptr::null_mut(), p_expr_1 as *const Expr);
                            }
                            p_right = unsafe { (*p_expr_1).p_right };
                            if unsafe { (*p_right).op } as i32 == 60 {
                                z_db = core::ptr::null();
                            } else {
                                { let _ = 0; };
                                { let _ = 0; };
                                z_db = unsafe { (*p_left).u.z_token } as *const i8;
                                p_left = unsafe { (*p_right).p_left };
                                p_right = unsafe { (*p_right).p_right };
                            }
                            { let _ = 0; };
                            z_table = unsafe { (*p_left).u.z_token } as *const i8;
                            { let _ = 0; };
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        p_expr_1 as *mut () as *const (),
                                        p_right as *mut () as *const ())
                                };
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        unsafe { &raw mut (*p_expr_1).y.p_tab } as *mut () as
                                            *const (), p_left as *mut () as *const ())
                                };
                            }
                        }
                        return lookup_name(p_parse, z_db, z_table,
                                unsafe { &*p_right }, p_nc, p_expr_1);
                    }
                    {
                        let mut p_list: *mut ExprList = core::ptr::null_mut();
                        let mut n: i32 = 0;
                        let mut no_such_func: i32 = 0;
                        let mut wrong_num_args: i32 = 0;
                        let mut is_agg: i32 = 0;
                        let mut z_id: *const i8 = core::ptr::null();
                        let mut p_def: *mut FuncDef = core::ptr::null_mut();
                        let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                        let saved_allow_flags: i32 =
                            unsafe { (*p_nc).nc_flags } & (1 | 16384);
                        let p_win: *mut Window =
                            if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 &&
                                    unsafe { (*unsafe { (*p_expr_1).y.p_win }).e_frm_type } as
                                            i32 != 167 {
                                unsafe { (*p_expr_1).y.p_win }
                            } else { core::ptr::null_mut() };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_list = unsafe { (*p_expr_1).x.p_list };
                        n =
                            if !(p_list).is_null() {
                                unsafe { (*p_list).n_expr }
                            } else { 0 };
                        z_id = unsafe { (*p_expr_1).u.z_token } as *const i8;
                        p_def =
                            unsafe {
                                sqlite3_find_function(unsafe { (*p_parse).db }, z_id, n,
                                    enc, 0 as u8)
                            };
                        if p_def == core::ptr::null_mut() {
                            p_def =
                                unsafe {
                                    sqlite3_find_function(unsafe { (*p_parse).db }, z_id, -2,
                                        enc, 0 as u8)
                                };
                            if p_def == core::ptr::null_mut() {
                                no_such_func = 1;
                            } else { wrong_num_args = 1; }
                        } else {
                            is_agg = unsafe { (*p_def).x_finalize.is_some() } as i32;
                            if unsafe { (*p_def).func_flags } & 1024 as u32 != 0 {
                                unsafe { (*p_expr_1).flags |= 524288 as u32 };
                                if n == 2 {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            expr_probability(unsafe {
                                                    &*unsafe {
                                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                *mut ExprList_item).offset(1 as isize)).p_expr
                                                            }
                                                })
                                    };
                                    if unsafe { (*p_expr_1).i_table } < 0 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"second argument to %#T() must be a constant between 0.0 and 1.0".as_ptr()
                                                        as *mut i8 as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                } else {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            if unsafe { *unsafe { (*p_def).z_name.offset(0 as isize) } }
                                                        as i32 == 'u' as i32 {
                                                8388608
                                            } else { 125829120 }
                                    };
                                }
                            }
                            {
                                let auth: i32 =
                                    unsafe {
                                        sqlite3_auth_check(p_parse, 31, core::ptr::null(),
                                            unsafe { (*p_def).z_name }, core::ptr::null())
                                    };
                                if auth != 0 {
                                    if auth == 1 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"not authorized to use function: %#T".as_ptr() as *mut i8
                                                    as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                    unsafe { (*p_expr_1).op = 122 as u8 };
                                    return 1;
                                }
                            }
                            if unsafe { (*p_def).func_flags } & 1048576 as u32 != 0 ||
                                    unsafe { (*p_expr_1).flags } & 2147483648u32 as u32 !=
                                        0 as u32 {
                                resolve_set_expr_subtype_arg(p_list as *const ExprList);
                            }
                            if unsafe { (*p_def).func_flags } & (2048 | 8192) as u32 !=
                                    0 {
                                unsafe { (*p_expr_1).flags |= 1048576 as u32 };
                            }
                            if unsafe { (*p_def).func_flags } & 2048 as u32 == 0 as u32
                                {
                                { let _ = 0; };
                                if unsafe { (*p_nc).nc_flags } & (32 | 2 | 8) != 0 {
                                    not_valid_impl(p_parse, unsafe { &*p_nc },
                                        c"non-deterministic functions".as_ptr() as *mut i8 as
                                            *const i8, core::ptr::null_mut(), p_expr_1 as *const Expr);
                                }
                            } else {
                                { let _ = 0; };
                                unsafe {
                                    (*p_expr_1).op2 = (unsafe { (*p_nc).nc_flags } & 46) as u8
                                };
                            }
                            if unsafe { (*p_def).func_flags } & 262144 as u32 !=
                                            0 as u32 && unsafe { (*p_parse).nested } as i32 == 0 &&
                                    unsafe { (*unsafe { (*p_parse).db }).m_db_flags } &
                                            32 as u32 == 0 as u32 {
                                no_such_func = 2;
                                p_def = core::ptr::null_mut();
                            } else if unsafe { (*p_def).func_flags } &
                                            (524288 | 2097152) as u32 != 0 as u32 &&
                                    !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 !=
                                        0 {
                                if unsafe { (*p_nc).nc_flags } & 262144 != 0 {
                                    unsafe { (*p_expr_1).flags |= 1073741824 as u32 };
                                }
                                unsafe {
                                    sqlite3_expr_function_usable(p_parse,
                                        p_expr_1 as *const Expr, p_def as *const FuncDef)
                                };
                            }
                        }
                        if 0 ==
                                (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 {
                            { let _ = 0; };
                            if !(p_def).is_null() &&
                                        !unsafe { (*p_def).x_value.is_some() } as i32 != 0 &&
                                    !(p_win).is_null() {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"%#T() may not be used as a window function".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg != 0 &&
                                            unsafe { (*p_nc).nc_flags } & 1 == 0 ||
                                        is_agg != 0 &&
                                                unsafe { (*p_def).func_flags } & 65536 as u32 != 0 &&
                                            (p_win).is_null() as i32 != 0 ||
                                    is_agg != 0 && !(p_win).is_null() &&
                                        unsafe { (*p_nc).nc_flags } & 16384 == 0 {
                                let mut z_type: *const i8 = core::ptr::null();
                                if unsafe { (*p_def).func_flags } & 65536 as u32 != 0 ||
                                        !(p_win).is_null() {
                                    z_type = c"window".as_ptr() as *mut i8 as *const i8;
                                } else {
                                    z_type = c"aggregate".as_ptr() as *mut i8 as *const i8;
                                }
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"misuse of %s function %#T()".as_ptr() as *mut i8 as
                                            *const i8, z_type, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                is_agg = 0;
                            } else if no_such_func != 0 &&
                                    (unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                            0 ||
                                        no_such_func == 2 &&
                                            unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                                2) {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"no such function: %#T".as_ptr() as *mut i8 as *const i8,
                                        p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if wrong_num_args != 0 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"wrong number of arguments to function %#T()".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"FILTER may not be used with non-aggregate %#T()".as_ptr()
                                                as *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    !(unsafe { (*p_expr_1).p_left }).is_null() {
                                unsafe {
                                    sqlite3_expr_order_by_aggregate_error(p_parse, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                            if is_agg != 0 {
                                unsafe {
                                    (*p_nc).nc_flags &=
                                        !(16384 | if (p_win).is_null() as i32 != 0 { 1 } else { 0 })
                                };
                            }
                        } else if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                    0 as u32 || !(unsafe { (*p_expr_1).p_left }).is_null() {
                            is_agg = 1;
                        }
                        unsafe { sqlite3_walk_expr_list(p_walker_1, p_list) };
                        if is_agg != 0 {
                            if !(unsafe { (*p_expr_1).p_left }).is_null() {
                                { let _ = 0; };
                                { let _ = 0; };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list })
                                };
                            }
                            if !(p_win).is_null() && unsafe { (*p_parse).n_err } == 0 {
                                let p_sel: *mut Select = unsafe { (*p_nc).p_win_select };
                                { let _ = 0; };
                                if (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                        == 0 {
                                    unsafe {
                                        sqlite3_window_update(p_parse,
                                            if !(p_sel).is_null() {
                                                unsafe { (*p_sel).p_win_defn }
                                            } else { core::ptr::null_mut() }, p_win, p_def)
                                    };
                                    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                                        {
                                        break '__s5;
                                    }
                                }
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_partition })
                                };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_order_by })
                                };
                                unsafe {
                                    sqlite3_walk_expr(p_walker_1, unsafe { (*p_win).p_filter })
                                };
                                unsafe { sqlite3_window_link(p_sel, p_win) };
                                unsafe { (*p_nc).nc_flags |= 32768 };
                            } else {
                                let mut p_nc2: *mut NameContext = core::ptr::null_mut();
                                unsafe { (*p_expr_1).op = 169 as u8 };
                                unsafe { (*p_expr_1).op2 = 0 as u8 };
                                if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 {
                                    unsafe {
                                        sqlite3_walk_expr(p_walker_1,
                                            unsafe { (*unsafe { (*p_expr_1).y.p_win }).p_filter })
                                    };
                                }
                                p_nc2 = p_nc;
                                while !(p_nc2).is_null() &&
                                        unsafe {
                                                sqlite3_references_src_list(p_parse, p_expr_1,
                                                    unsafe { (*p_nc2).p_src_list })
                                            } == 0 {
                                    unsafe {
                                        (*p_expr_1).op2 +=
                                            (1 as u32 + unsafe { (*p_nc2).n_nested_select }) as u8
                                    };
                                    p_nc2 = unsafe { (*p_nc2).p_next };
                                }
                                { let _ = 0; };
                                if !(p_nc2).is_null() && !(p_def).is_null() {
                                    unsafe {
                                        (*p_expr_1).op2 += unsafe { (*p_nc2).n_nested_select } as u8
                                    };
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    unsafe {
                                        (*p_nc2).nc_flags |=
                                            (16 as u32 |
                                                    (unsafe { (*p_def).func_flags } ^ 134217728 as u32) &
                                                        (4096 | 134217728) as u32) as i32
                                    };
                                }
                            }
                            unsafe { (*p_nc).nc_flags |= saved_allow_flags };
                        }
                        return 1;
                    }
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                142 => {
                    {
                        let mut z_table: *const i8 = core::ptr::null();
                        let mut z_db: *const i8 = core::ptr::null();
                        let mut p_right: *mut Expr = core::ptr::null_mut();
                        if unsafe { (*p_expr_1).op } as i32 == 60 {
                            z_db = core::ptr::null();
                            z_table = core::ptr::null();
                            { let _ = 0; };
                            p_right = p_expr_1;
                        } else {
                            let mut p_left: *mut Expr = unsafe { (*p_expr_1).p_left };
                            { let _ = 0; };
                            if unsafe { (*p_nc).nc_flags } & (32 | 8) != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"the \".\" operator".as_ptr() as *mut i8 as *const i8,
                                    core::ptr::null_mut(), p_expr_1 as *const Expr);
                            }
                            p_right = unsafe { (*p_expr_1).p_right };
                            if unsafe { (*p_right).op } as i32 == 60 {
                                z_db = core::ptr::null();
                            } else {
                                { let _ = 0; };
                                { let _ = 0; };
                                z_db = unsafe { (*p_left).u.z_token } as *const i8;
                                p_left = unsafe { (*p_right).p_left };
                                p_right = unsafe { (*p_right).p_right };
                            }
                            { let _ = 0; };
                            z_table = unsafe { (*p_left).u.z_token } as *const i8;
                            { let _ = 0; };
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        p_expr_1 as *mut () as *const (),
                                        p_right as *mut () as *const ())
                                };
                                unsafe {
                                    sqlite3_rename_token_remap(p_parse,
                                        unsafe { &raw mut (*p_expr_1).y.p_tab } as *mut () as
                                            *const (), p_left as *mut () as *const ())
                                };
                            }
                        }
                        return lookup_name(p_parse, z_db, z_table,
                                unsafe { &*p_right }, p_nc, p_expr_1);
                    }
                    {
                        let mut p_list: *mut ExprList = core::ptr::null_mut();
                        let mut n: i32 = 0;
                        let mut no_such_func: i32 = 0;
                        let mut wrong_num_args: i32 = 0;
                        let mut is_agg: i32 = 0;
                        let mut z_id: *const i8 = core::ptr::null();
                        let mut p_def: *mut FuncDef = core::ptr::null_mut();
                        let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                        let saved_allow_flags: i32 =
                            unsafe { (*p_nc).nc_flags } & (1 | 16384);
                        let p_win: *mut Window =
                            if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 &&
                                    unsafe { (*unsafe { (*p_expr_1).y.p_win }).e_frm_type } as
                                            i32 != 167 {
                                unsafe { (*p_expr_1).y.p_win }
                            } else { core::ptr::null_mut() };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_list = unsafe { (*p_expr_1).x.p_list };
                        n =
                            if !(p_list).is_null() {
                                unsafe { (*p_list).n_expr }
                            } else { 0 };
                        z_id = unsafe { (*p_expr_1).u.z_token } as *const i8;
                        p_def =
                            unsafe {
                                sqlite3_find_function(unsafe { (*p_parse).db }, z_id, n,
                                    enc, 0 as u8)
                            };
                        if p_def == core::ptr::null_mut() {
                            p_def =
                                unsafe {
                                    sqlite3_find_function(unsafe { (*p_parse).db }, z_id, -2,
                                        enc, 0 as u8)
                                };
                            if p_def == core::ptr::null_mut() {
                                no_such_func = 1;
                            } else { wrong_num_args = 1; }
                        } else {
                            is_agg = unsafe { (*p_def).x_finalize.is_some() } as i32;
                            if unsafe { (*p_def).func_flags } & 1024 as u32 != 0 {
                                unsafe { (*p_expr_1).flags |= 524288 as u32 };
                                if n == 2 {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            expr_probability(unsafe {
                                                    &*unsafe {
                                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                *mut ExprList_item).offset(1 as isize)).p_expr
                                                            }
                                                })
                                    };
                                    if unsafe { (*p_expr_1).i_table } < 0 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"second argument to %#T() must be a constant between 0.0 and 1.0".as_ptr()
                                                        as *mut i8 as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                } else {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            if unsafe { *unsafe { (*p_def).z_name.offset(0 as isize) } }
                                                        as i32 == 'u' as i32 {
                                                8388608
                                            } else { 125829120 }
                                    };
                                }
                            }
                            {
                                let auth: i32 =
                                    unsafe {
                                        sqlite3_auth_check(p_parse, 31, core::ptr::null(),
                                            unsafe { (*p_def).z_name }, core::ptr::null())
                                    };
                                if auth != 0 {
                                    if auth == 1 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"not authorized to use function: %#T".as_ptr() as *mut i8
                                                    as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                    unsafe { (*p_expr_1).op = 122 as u8 };
                                    return 1;
                                }
                            }
                            if unsafe { (*p_def).func_flags } & 1048576 as u32 != 0 ||
                                    unsafe { (*p_expr_1).flags } & 2147483648u32 as u32 !=
                                        0 as u32 {
                                resolve_set_expr_subtype_arg(p_list as *const ExprList);
                            }
                            if unsafe { (*p_def).func_flags } & (2048 | 8192) as u32 !=
                                    0 {
                                unsafe { (*p_expr_1).flags |= 1048576 as u32 };
                            }
                            if unsafe { (*p_def).func_flags } & 2048 as u32 == 0 as u32
                                {
                                { let _ = 0; };
                                if unsafe { (*p_nc).nc_flags } & (32 | 2 | 8) != 0 {
                                    not_valid_impl(p_parse, unsafe { &*p_nc },
                                        c"non-deterministic functions".as_ptr() as *mut i8 as
                                            *const i8, core::ptr::null_mut(), p_expr_1 as *const Expr);
                                }
                            } else {
                                { let _ = 0; };
                                unsafe {
                                    (*p_expr_1).op2 = (unsafe { (*p_nc).nc_flags } & 46) as u8
                                };
                            }
                            if unsafe { (*p_def).func_flags } & 262144 as u32 !=
                                            0 as u32 && unsafe { (*p_parse).nested } as i32 == 0 &&
                                    unsafe { (*unsafe { (*p_parse).db }).m_db_flags } &
                                            32 as u32 == 0 as u32 {
                                no_such_func = 2;
                                p_def = core::ptr::null_mut();
                            } else if unsafe { (*p_def).func_flags } &
                                            (524288 | 2097152) as u32 != 0 as u32 &&
                                    !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 !=
                                        0 {
                                if unsafe { (*p_nc).nc_flags } & 262144 != 0 {
                                    unsafe { (*p_expr_1).flags |= 1073741824 as u32 };
                                }
                                unsafe {
                                    sqlite3_expr_function_usable(p_parse,
                                        p_expr_1 as *const Expr, p_def as *const FuncDef)
                                };
                            }
                        }
                        if 0 ==
                                (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 {
                            { let _ = 0; };
                            if !(p_def).is_null() &&
                                        !unsafe { (*p_def).x_value.is_some() } as i32 != 0 &&
                                    !(p_win).is_null() {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"%#T() may not be used as a window function".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg != 0 &&
                                            unsafe { (*p_nc).nc_flags } & 1 == 0 ||
                                        is_agg != 0 &&
                                                unsafe { (*p_def).func_flags } & 65536 as u32 != 0 &&
                                            (p_win).is_null() as i32 != 0 ||
                                    is_agg != 0 && !(p_win).is_null() &&
                                        unsafe { (*p_nc).nc_flags } & 16384 == 0 {
                                let mut z_type: *const i8 = core::ptr::null();
                                if unsafe { (*p_def).func_flags } & 65536 as u32 != 0 ||
                                        !(p_win).is_null() {
                                    z_type = c"window".as_ptr() as *mut i8 as *const i8;
                                } else {
                                    z_type = c"aggregate".as_ptr() as *mut i8 as *const i8;
                                }
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"misuse of %s function %#T()".as_ptr() as *mut i8 as
                                            *const i8, z_type, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                is_agg = 0;
                            } else if no_such_func != 0 &&
                                    (unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                            0 ||
                                        no_such_func == 2 &&
                                            unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                                2) {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"no such function: %#T".as_ptr() as *mut i8 as *const i8,
                                        p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if wrong_num_args != 0 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"wrong number of arguments to function %#T()".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"FILTER may not be used with non-aggregate %#T()".as_ptr()
                                                as *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    !(unsafe { (*p_expr_1).p_left }).is_null() {
                                unsafe {
                                    sqlite3_expr_order_by_aggregate_error(p_parse, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                            if is_agg != 0 {
                                unsafe {
                                    (*p_nc).nc_flags &=
                                        !(16384 | if (p_win).is_null() as i32 != 0 { 1 } else { 0 })
                                };
                            }
                        } else if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                    0 as u32 || !(unsafe { (*p_expr_1).p_left }).is_null() {
                            is_agg = 1;
                        }
                        unsafe { sqlite3_walk_expr_list(p_walker_1, p_list) };
                        if is_agg != 0 {
                            if !(unsafe { (*p_expr_1).p_left }).is_null() {
                                { let _ = 0; };
                                { let _ = 0; };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list })
                                };
                            }
                            if !(p_win).is_null() && unsafe { (*p_parse).n_err } == 0 {
                                let p_sel: *mut Select = unsafe { (*p_nc).p_win_select };
                                { let _ = 0; };
                                if (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                        == 0 {
                                    unsafe {
                                        sqlite3_window_update(p_parse,
                                            if !(p_sel).is_null() {
                                                unsafe { (*p_sel).p_win_defn }
                                            } else { core::ptr::null_mut() }, p_win, p_def)
                                    };
                                    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                                        {
                                        break '__s5;
                                    }
                                }
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_partition })
                                };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_order_by })
                                };
                                unsafe {
                                    sqlite3_walk_expr(p_walker_1, unsafe { (*p_win).p_filter })
                                };
                                unsafe { sqlite3_window_link(p_sel, p_win) };
                                unsafe { (*p_nc).nc_flags |= 32768 };
                            } else {
                                let mut p_nc2: *mut NameContext = core::ptr::null_mut();
                                unsafe { (*p_expr_1).op = 169 as u8 };
                                unsafe { (*p_expr_1).op2 = 0 as u8 };
                                if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 {
                                    unsafe {
                                        sqlite3_walk_expr(p_walker_1,
                                            unsafe { (*unsafe { (*p_expr_1).y.p_win }).p_filter })
                                    };
                                }
                                p_nc2 = p_nc;
                                while !(p_nc2).is_null() &&
                                        unsafe {
                                                sqlite3_references_src_list(p_parse, p_expr_1,
                                                    unsafe { (*p_nc2).p_src_list })
                                            } == 0 {
                                    unsafe {
                                        (*p_expr_1).op2 +=
                                            (1 as u32 + unsafe { (*p_nc2).n_nested_select }) as u8
                                    };
                                    p_nc2 = unsafe { (*p_nc2).p_next };
                                }
                                { let _ = 0; };
                                if !(p_nc2).is_null() && !(p_def).is_null() {
                                    unsafe {
                                        (*p_expr_1).op2 += unsafe { (*p_nc2).n_nested_select } as u8
                                    };
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    unsafe {
                                        (*p_nc2).nc_flags |=
                                            (16 as u32 |
                                                    (unsafe { (*p_def).func_flags } ^ 134217728 as u32) &
                                                        (4096 | 134217728) as u32) as i32
                                    };
                                }
                            }
                            unsafe { (*p_nc).nc_flags |= saved_allow_flags };
                        }
                        return 1;
                    }
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                172 => {
                    {
                        let mut p_list: *mut ExprList = core::ptr::null_mut();
                        let mut n: i32 = 0;
                        let mut no_such_func: i32 = 0;
                        let mut wrong_num_args: i32 = 0;
                        let mut is_agg: i32 = 0;
                        let mut z_id: *const i8 = core::ptr::null();
                        let mut p_def: *mut FuncDef = core::ptr::null_mut();
                        let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                        let saved_allow_flags: i32 =
                            unsafe { (*p_nc).nc_flags } & (1 | 16384);
                        let p_win: *mut Window =
                            if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 &&
                                    unsafe { (*unsafe { (*p_expr_1).y.p_win }).e_frm_type } as
                                            i32 != 167 {
                                unsafe { (*p_expr_1).y.p_win }
                            } else { core::ptr::null_mut() };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_list = unsafe { (*p_expr_1).x.p_list };
                        n =
                            if !(p_list).is_null() {
                                unsafe { (*p_list).n_expr }
                            } else { 0 };
                        z_id = unsafe { (*p_expr_1).u.z_token } as *const i8;
                        p_def =
                            unsafe {
                                sqlite3_find_function(unsafe { (*p_parse).db }, z_id, n,
                                    enc, 0 as u8)
                            };
                        if p_def == core::ptr::null_mut() {
                            p_def =
                                unsafe {
                                    sqlite3_find_function(unsafe { (*p_parse).db }, z_id, -2,
                                        enc, 0 as u8)
                                };
                            if p_def == core::ptr::null_mut() {
                                no_such_func = 1;
                            } else { wrong_num_args = 1; }
                        } else {
                            is_agg = unsafe { (*p_def).x_finalize.is_some() } as i32;
                            if unsafe { (*p_def).func_flags } & 1024 as u32 != 0 {
                                unsafe { (*p_expr_1).flags |= 524288 as u32 };
                                if n == 2 {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            expr_probability(unsafe {
                                                    &*unsafe {
                                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                *mut ExprList_item).offset(1 as isize)).p_expr
                                                            }
                                                })
                                    };
                                    if unsafe { (*p_expr_1).i_table } < 0 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"second argument to %#T() must be a constant between 0.0 and 1.0".as_ptr()
                                                        as *mut i8 as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                } else {
                                    unsafe {
                                        (*p_expr_1).i_table =
                                            if unsafe { *unsafe { (*p_def).z_name.offset(0 as isize) } }
                                                        as i32 == 'u' as i32 {
                                                8388608
                                            } else { 125829120 }
                                    };
                                }
                            }
                            {
                                let auth: i32 =
                                    unsafe {
                                        sqlite3_auth_check(p_parse, 31, core::ptr::null(),
                                            unsafe { (*p_def).z_name }, core::ptr::null())
                                    };
                                if auth != 0 {
                                    if auth == 1 {
                                        unsafe {
                                            sqlite3_error_msg(p_parse,
                                                c"not authorized to use function: %#T".as_ptr() as *mut i8
                                                    as *const i8, p_expr_1)
                                        };
                                        {
                                            let __p = unsafe { &mut (*p_nc).n_nc_err };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    }
                                    unsafe { (*p_expr_1).op = 122 as u8 };
                                    return 1;
                                }
                            }
                            if unsafe { (*p_def).func_flags } & 1048576 as u32 != 0 ||
                                    unsafe { (*p_expr_1).flags } & 2147483648u32 as u32 !=
                                        0 as u32 {
                                resolve_set_expr_subtype_arg(p_list as *const ExprList);
                            }
                            if unsafe { (*p_def).func_flags } & (2048 | 8192) as u32 !=
                                    0 {
                                unsafe { (*p_expr_1).flags |= 1048576 as u32 };
                            }
                            if unsafe { (*p_def).func_flags } & 2048 as u32 == 0 as u32
                                {
                                { let _ = 0; };
                                if unsafe { (*p_nc).nc_flags } & (32 | 2 | 8) != 0 {
                                    not_valid_impl(p_parse, unsafe { &*p_nc },
                                        c"non-deterministic functions".as_ptr() as *mut i8 as
                                            *const i8, core::ptr::null_mut(), p_expr_1 as *const Expr);
                                }
                            } else {
                                { let _ = 0; };
                                unsafe {
                                    (*p_expr_1).op2 = (unsafe { (*p_nc).nc_flags } & 46) as u8
                                };
                            }
                            if unsafe { (*p_def).func_flags } & 262144 as u32 !=
                                            0 as u32 && unsafe { (*p_parse).nested } as i32 == 0 &&
                                    unsafe { (*unsafe { (*p_parse).db }).m_db_flags } &
                                            32 as u32 == 0 as u32 {
                                no_such_func = 2;
                                p_def = core::ptr::null_mut();
                            } else if unsafe { (*p_def).func_flags } &
                                            (524288 | 2097152) as u32 != 0 as u32 &&
                                    !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 !=
                                        0 {
                                if unsafe { (*p_nc).nc_flags } & 262144 != 0 {
                                    unsafe { (*p_expr_1).flags |= 1073741824 as u32 };
                                }
                                unsafe {
                                    sqlite3_expr_function_usable(p_parse,
                                        p_expr_1 as *const Expr, p_def as *const FuncDef)
                                };
                            }
                        }
                        if 0 ==
                                (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 {
                            { let _ = 0; };
                            if !(p_def).is_null() &&
                                        !unsafe { (*p_def).x_value.is_some() } as i32 != 0 &&
                                    !(p_win).is_null() {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"%#T() may not be used as a window function".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg != 0 &&
                                            unsafe { (*p_nc).nc_flags } & 1 == 0 ||
                                        is_agg != 0 &&
                                                unsafe { (*p_def).func_flags } & 65536 as u32 != 0 &&
                                            (p_win).is_null() as i32 != 0 ||
                                    is_agg != 0 && !(p_win).is_null() &&
                                        unsafe { (*p_nc).nc_flags } & 16384 == 0 {
                                let mut z_type: *const i8 = core::ptr::null();
                                if unsafe { (*p_def).func_flags } & 65536 as u32 != 0 ||
                                        !(p_win).is_null() {
                                    z_type = c"window".as_ptr() as *mut i8 as *const i8;
                                } else {
                                    z_type = c"aggregate".as_ptr() as *mut i8 as *const i8;
                                }
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"misuse of %s function %#T()".as_ptr() as *mut i8 as
                                            *const i8, z_type, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                is_agg = 0;
                            } else if no_such_func != 0 &&
                                    (unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                            0 ||
                                        no_such_func == 2 &&
                                            unsafe { (*unsafe { (*p_parse).db }).init.busy } as i32 ==
                                                2) {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"no such function: %#T".as_ptr() as *mut i8 as *const i8,
                                        p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if wrong_num_args != 0 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"wrong number of arguments to function %#T()".as_ptr() as
                                                *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"FILTER may not be used with non-aggregate %#T()".as_ptr()
                                                as *mut i8 as *const i8, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else if is_agg == 0 &&
                                    !(unsafe { (*p_expr_1).p_left }).is_null() {
                                unsafe {
                                    sqlite3_expr_order_by_aggregate_error(p_parse, p_expr_1)
                                };
                                {
                                    let __p = unsafe { &mut (*p_nc).n_nc_err };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            }
                            if is_agg != 0 {
                                unsafe {
                                    (*p_nc).nc_flags &=
                                        !(16384 | if (p_win).is_null() as i32 != 0 { 1 } else { 0 })
                                };
                            }
                        } else if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                    0 as u32 || !(unsafe { (*p_expr_1).p_left }).is_null() {
                            is_agg = 1;
                        }
                        unsafe { sqlite3_walk_expr_list(p_walker_1, p_list) };
                        if is_agg != 0 {
                            if !(unsafe { (*p_expr_1).p_left }).is_null() {
                                { let _ = 0; };
                                { let _ = 0; };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list })
                                };
                            }
                            if !(p_win).is_null() && unsafe { (*p_parse).n_err } == 0 {
                                let p_sel: *mut Select = unsafe { (*p_nc).p_win_select };
                                { let _ = 0; };
                                if (unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                        == 0 {
                                    unsafe {
                                        sqlite3_window_update(p_parse,
                                            if !(p_sel).is_null() {
                                                unsafe { (*p_sel).p_win_defn }
                                            } else { core::ptr::null_mut() }, p_win, p_def)
                                    };
                                    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                                        {
                                        break '__s5;
                                    }
                                }
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_partition })
                                };
                                unsafe {
                                    sqlite3_walk_expr_list(p_walker_1,
                                        unsafe { (*p_win).p_order_by })
                                };
                                unsafe {
                                    sqlite3_walk_expr(p_walker_1, unsafe { (*p_win).p_filter })
                                };
                                unsafe { sqlite3_window_link(p_sel, p_win) };
                                unsafe { (*p_nc).nc_flags |= 32768 };
                            } else {
                                let mut p_nc2: *mut NameContext = core::ptr::null_mut();
                                unsafe { (*p_expr_1).op = 169 as u8 };
                                unsafe { (*p_expr_1).op2 = 0 as u8 };
                                if unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32 {
                                    unsafe {
                                        sqlite3_walk_expr(p_walker_1,
                                            unsafe { (*unsafe { (*p_expr_1).y.p_win }).p_filter })
                                    };
                                }
                                p_nc2 = p_nc;
                                while !(p_nc2).is_null() &&
                                        unsafe {
                                                sqlite3_references_src_list(p_parse, p_expr_1,
                                                    unsafe { (*p_nc2).p_src_list })
                                            } == 0 {
                                    unsafe {
                                        (*p_expr_1).op2 +=
                                            (1 as u32 + unsafe { (*p_nc2).n_nested_select }) as u8
                                    };
                                    p_nc2 = unsafe { (*p_nc2).p_next };
                                }
                                { let _ = 0; };
                                if !(p_nc2).is_null() && !(p_def).is_null() {
                                    unsafe {
                                        (*p_expr_1).op2 += unsafe { (*p_nc2).n_nested_select } as u8
                                    };
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    unsafe {
                                        (*p_nc2).nc_flags |=
                                            (16 as u32 |
                                                    (unsafe { (*p_def).func_flags } ^ 134217728 as u32) &
                                                        (4096 | 134217728) as u32) as i32
                                    };
                                }
                            }
                            unsafe { (*p_nc).nc_flags |= saved_allow_flags };
                        }
                        return 1;
                    }
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                20 => {
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                139 => {
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                50 => {
                    {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            let n_ref: i32 = unsafe { (*p_nc).n_ref };
                            { let _ = 0; };
                            if unsafe { (*p_expr_1).op } as i32 == 20 {
                                unsafe { (*p_parse).set_b_has_exists(1 as bft as u32) };
                            }
                            if unsafe { (*p_nc).nc_flags } & 46 != 0 {
                                not_valid_impl(p_parse, unsafe { &*p_nc },
                                    c"subqueries".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                    p_expr_1 as *const Expr);
                            } else {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*p_expr_1).x.p_select })
                                };
                            }
                            { let _ = 0; };
                            if n_ref != unsafe { (*p_nc).n_ref } {
                                unsafe { (*p_expr_1).flags |= 64 as u32 };
                                unsafe {
                                    (*unsafe { (*p_expr_1).x.p_select }).sel_flags |=
                                        536870912 as u32
                                };
                            }
                            unsafe { (*p_nc).nc_flags |= 64 };
                        }
                        break '__s5;
                    }
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                157 => {
                    {
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & (4 | 2 | 32 | 8) != 0 {
                            not_valid_impl(p_parse, unsafe { &*p_nc },
                                c"parameters".as_ptr() as *mut i8 as *const i8, p_expr_1,
                                p_expr_1 as *const Expr);
                        }
                        break '__s5;
                    }
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                45 => {
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                46 => {
                    {
                        let p_right_1: *mut Expr =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_expr_1).p_right
                                    })
                            };
                        { let _ = 0; };
                        if !(p_right_1).is_null() &&
                                (unsafe { (*p_right_1).op } as i32 == 60 ||
                                    unsafe { (*p_right_1).op } as i32 == 171) {
                            let rc: i32 = resolve_expr_step(p_walker_1, p_right_1);
                            if rc == 2 { return 2; }
                            if unsafe { (*p_right_1).op } as i32 == 171 {
                                unsafe { (*p_expr_1).op2 = unsafe { (*p_expr_1).op } };
                                unsafe { (*p_expr_1).op = 175 as u8 };
                                return 0;
                            }
                        }
                    }
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                49 => {
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                54 => {
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                53 => {
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                57 => {
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                56 => {
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                55 => {
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                58 => {
                    {
                        let mut n_left: i32 = 0;
                        let mut n_right: i32 = 0;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            break '__s5;
                        }
                        { let _ = 0; };
                        n_left =
                            unsafe {
                                sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            };
                        if unsafe { (*p_expr_1).op } as i32 == 49 {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe {
                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                as *mut ExprList_item).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            if n_right == n_left {
                                n_right =
                                    unsafe {
                                        sqlite3_expr_vector_size(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                    as *mut ExprList_item).offset(1 as isize)).p_expr
                                                } as *const Expr)
                                    };
                            }
                        } else {
                            { let _ = 0; };
                            n_right =
                                unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr_1).p_right } as
                                            *const Expr)
                                };
                        }
                        if n_left != n_right {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"row value misused".as_ptr() as *mut i8 as *const i8)
                            };
                            unsafe {
                                sqlite3_record_error_offset_of_expr(unsafe {
                                        (*p_parse).db
                                    }, p_expr_1 as *const Expr)
                            };
                        }
                        break '__s5;
                    }
                }
                _ => {}
            }
        }
        { let _ = 0; };
        return if unsafe { (*p_parse).n_err } != 0 { 2 } else { 0 };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_resolve_select_names(p_parse: *mut Parse,
    p: *mut Select, p_outer_nc: *mut NameContext) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        { let _ = 0; };
        w.x_expr_callback = Some(resolve_expr_step);
        w.x_select_callback = Some(resolve_select_step);
        w.x_select_callback2 = None;
        w.p_parse = p_parse;
        w.u.p_nc = p_outer_nc;
        unsafe { sqlite3_walk_select(&mut w, p) };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_resolve_expr_list_names(p_nc: *mut NameContext,
    p_list: *mut ExprList) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut saved_has_agg: i32 = 0;
        let mut w: Walker = unsafe { core::mem::zeroed() };
        if p_list == core::ptr::null_mut() { return 0; }
        w.p_parse = unsafe { (*p_nc).p_parse };
        w.x_expr_callback = Some(resolve_expr_step);
        w.x_select_callback = Some(resolve_select_step);
        w.x_select_callback2 = None;
        w.u.p_nc = p_nc;
        saved_has_agg =
            unsafe { (*p_nc).nc_flags } & (16 | 4096 | 32768 | 134217728);
        unsafe { (*p_nc).nc_flags &= !(16 | 4096 | 32768 | 134217728) };
        {
            i = 0;
            '__b10: loop {
                if !(i < unsafe { (*p_list).n_expr }) { break '__b10; }
                '__c10: loop {
                    let p_expr: *mut Expr =
                        unsafe {
                            (*(unsafe { (*p_list).a.as_ptr() } as
                                            *mut ExprList_item).offset(i as isize)).p_expr
                        };
                    if p_expr == core::ptr::null_mut() { break '__c10; }
                    unsafe {
                        (*w.p_parse).n_height += unsafe { (*p_expr).n_height }
                    };
                    if unsafe {
                                sqlite3_expr_check_height(w.p_parse,
                                    unsafe { (*w.p_parse).n_height })
                            } != 0 {
                        return 1;
                    }
                    unsafe { sqlite3_walk_expr_nn(&mut w, p_expr) };
                    unsafe {
                        (*w.p_parse).n_height -= unsafe { (*p_expr).n_height }
                    };
                    { let _ = 0; };
                    { let _ = 0; };
                    if unsafe { (*p_nc).nc_flags } &
                                (16 | 4096 | 32768 | 134217728) != 0 {
                        unsafe {
                            (*p_expr).flags |=
                                (unsafe { (*p_nc).nc_flags } & (16 | 32768)) as u32
                        };
                        saved_has_agg |=
                            unsafe { (*p_nc).nc_flags } &
                                (16 | 4096 | 32768 | 134217728);
                        unsafe {
                            (*p_nc).nc_flags &= !(16 | 4096 | 32768 | 134217728)
                        };
                    }
                    if unsafe { (*w.p_parse).n_err } > 0 { return 1; }
                    break '__c10;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { (*p_nc).nc_flags |= saved_has_agg };
        return 0;
    }
}
extern "C" fn resolve_as_name(p_parse_1: *const Parse, p_e_list_1: &ExprList,
    p_e_1: &Expr) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        { let _ = p_parse_1; };
        if (*p_e_1).op as i32 == 60 {
            let mut z_col: *const i8 = core::ptr::null();
            { let _ = 0; };
            z_col = (*p_e_1).u.z_token as *const i8;
            {
                i = 0;
                '__b11: loop {
                    if !(i < (*p_e_list_1).n_expr) { break '__b11; }
                    '__c11: loop {
                        if unsafe {
                                            (*((*p_e_list_1).a.as_ptr() as
                                                                *mut ExprList_item).offset(i as isize)).fg.e_e_name()
                                        } as i32 == 0 &&
                                unsafe {
                                        sqlite3_stricmp(unsafe {
                                                    (*((*p_e_list_1).a.as_ptr() as
                                                                    *mut ExprList_item).offset(i as isize)).z_e_name
                                                } as *const i8, z_col)
                                    } == 0 {
                            return i + 1;
                        }
                        break '__c11;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return 0;
    }
}
extern "C" fn resolve_out_of_range_error(p_parse_1: *mut Parse,
    z_type_1: *const i8, i: i32, mx: i32, p_error_1: *const Expr) -> () {
    unsafe {
        sqlite3_error_msg(p_parse_1,
            c"%r %s BY term out of range - should be between 1 and %d".as_ptr()
                    as *mut i8 as *const i8, i, z_type_1, mx)
    };
    unsafe {
        sqlite3_record_error_offset_of_expr(unsafe { (*p_parse_1).db },
            p_error_1 as *const Expr)
    };
}
extern "C" fn resolve_remove_windows_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        { let _ = p_walker_1; };
        if unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
            let p_win: *mut Window = unsafe { (*p_expr_1).y.p_win };
            unsafe { sqlite3_window_unlink_from_select(p_win) };
        }
        return 0;
    }
}
extern "C" fn window_remove_expr_from_select(p_select_1: *mut Select,
    p_expr_1: *mut Expr) -> () {
    unsafe {
        if !(unsafe { (*p_select_1).p_win }).is_null() {
            let mut s_walker: Walker = unsafe { core::mem::zeroed() };
            unsafe {
                memset(&raw mut s_walker as *mut (), 0,
                    core::mem::size_of::<Walker>() as u64)
            };
            s_walker.x_expr_callback = Some(resolve_remove_windows_cb);
            s_walker.u.p_select = p_select_1;
            unsafe { sqlite3_walk_expr(&mut s_walker, p_expr_1) };
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_resolve_order_group_by(p_parse: *mut Parse,
    p_select: &Select, p_order_by: *mut ExprList, z_type: *const i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let db: *const sqlite3 = unsafe { (*p_parse).db } as *const sqlite3;
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let mut p_item: *const ExprList_item = core::ptr::null();
        if p_order_by == core::ptr::null_mut() ||
                    unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0 ||
                unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
            return 0;
        }
        if unsafe { (*p_order_by).n_expr } >
                unsafe { (*db).a_limit[2 as usize] } {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"too many terms in %s BY clause".as_ptr() as *mut i8 as
                        *const i8, z_type)
            };
            return 1;
        }
        p_e_list = (*p_select).p_e_list;
        { let _ = 0; };
        {
            {
                i = 0;
                p_item =
                    unsafe { (*p_order_by).a.as_ptr() } as *mut ExprList_item
            };
            '__b12: loop {
                if !(i < unsafe { (*p_order_by).n_expr }) { break '__b12; }
                '__c12: loop {
                    if unsafe { (*p_item).u.x.i_order_by_col } != 0 {
                        if unsafe { (*p_item).u.x.i_order_by_col } as i32 >
                                unsafe { (*p_e_list).n_expr } {
                            resolve_out_of_range_error(p_parse, z_type, i + 1,
                                unsafe { (*p_e_list).n_expr }, core::ptr::null());
                            return 1;
                        }
                        resolve_alias(p_parse, unsafe { &*p_e_list },
                            unsafe { (*p_item).u.x.i_order_by_col } as i32 - 1,
                            unsafe { (*p_item).p_expr }, 0);
                    }
                    break '__c12;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_item;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        return 0;
    }
}
extern "C" fn resolve_order_group_by(p_nc_1: *mut NameContext,
    p_select_1: *mut Select, p_order_by_1: *mut ExprList, z_type_1: *const i8)
    -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut i_col: i32 = 0;
        let mut p_item: *mut ExprList_item = core::ptr::null_mut();
        let mut p_parse: *mut Parse = core::ptr::null_mut();
        let mut n_result: i32 = 0;
        { let _ = 0; };
        n_result = unsafe { (*unsafe { (*p_select_1).p_e_list }).n_expr };
        p_parse = unsafe { (*p_nc_1).p_parse };
        {
            {
                i = 0;
                p_item =
                    unsafe { (*p_order_by_1).a.as_ptr() } as *mut ExprList_item
            };
            '__b13: loop {
                if !(i < unsafe { (*p_order_by_1).n_expr }) { break '__b13; }
                '__c13: loop {
                    let p_e: *mut Expr = unsafe { (*p_item).p_expr };
                    let p_e2: *mut Expr =
                        unsafe { sqlite3_expr_skip_collate_and_likely(p_e) };
                    if p_e2 == core::ptr::null_mut() { break '__c13; }
                    if unsafe { *z_type_1.offset(0 as isize) } as i32 !=
                            'G' as i32 {
                        i_col =
                            resolve_as_name(p_parse as *const Parse,
                                unsafe { &*unsafe { (*p_select_1).p_e_list } },
                                unsafe { &*p_e2 });
                        if i_col > 0 {
                            unsafe { (*p_item).u.x.i_order_by_col = i_col as u16 };
                            break '__c13;
                        }
                    }
                    if unsafe {
                                sqlite3_expr_is_integer(p_e2 as *const Expr, &mut i_col,
                                    core::ptr::null_mut())
                            } != 0 {
                        if i_col < 1 || i_col > 65535 {
                            resolve_out_of_range_error(p_parse, z_type_1, i + 1,
                                n_result, p_e2 as *const Expr);
                            return 1;
                        }
                        unsafe { (*p_item).u.x.i_order_by_col = i_col as u16 };
                        break '__c13;
                    }
                    unsafe { (*p_item).u.x.i_order_by_col = 0 as u16 };
                    if sqlite3_resolve_expr_names(p_nc_1, p_e) != 0 {
                        return 1;
                    }
                    {
                        j = 0;
                        '__b14: loop {
                            if !(j <
                                            unsafe { (*unsafe { (*p_select_1).p_e_list }).n_expr }) {
                                break '__b14;
                            }
                            '__c14: loop {
                                if unsafe {
                                            sqlite3_expr_compare(core::ptr::null(), p_e as *const Expr,
                                                unsafe {
                                                        (*(unsafe {
                                                                            (*unsafe { (*p_select_1).p_e_list }).a.as_ptr()
                                                                        } as *mut ExprList_item).offset(j as isize)).p_expr
                                                    } as *const Expr, -1)
                                        } == 0 {
                                    window_remove_expr_from_select(p_select_1, p_e);
                                    unsafe { (*p_item).u.x.i_order_by_col = (j + 1) as u16 };
                                }
                                break '__c14;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__c13;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_item;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        return sqlite3_resolve_order_group_by(p_parse,
                unsafe { &*p_select_1 }, p_order_by_1, z_type_1);
    }
}
extern "C" fn resolve_order_by_term_to_expr_list(p_parse_1: *mut Parse,
    p_select_1: &Select, p_e_1: *mut Expr) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let mut nc: NameContext = unsafe { core::mem::zeroed() };
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut saved_supp_err: u8 = 0 as u8;
        { let _ = 0; };
        p_e_list = (*p_select_1).p_e_list;
        unsafe {
            memset(&raw mut nc as *mut (), 0,
                core::mem::size_of::<NameContext>() as u64)
        };
        nc.p_parse = p_parse_1;
        nc.p_src_list = (*p_select_1).p_src;
        nc.u_nc.p_e_list = p_e_list;
        nc.nc_flags = 1 | 128 | 524288;
        nc.n_nc_err = 0;
        db = unsafe { (*p_parse_1).db };
        saved_supp_err = unsafe { (*db).suppress_err };
        unsafe { (*db).suppress_err = 1 as u8 };
        rc = sqlite3_resolve_expr_names(&mut nc, p_e_1);
        unsafe { (*db).suppress_err = saved_supp_err };
        if rc != 0 { return 0; }
        {
            i = 0;
            '__b15: loop {
                if !(i < unsafe { (*p_e_list).n_expr }) { break '__b15; }
                '__c15: loop {
                    if unsafe {
                                sqlite3_expr_compare(core::ptr::null(),
                                    unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprList_item).offset(i as isize)).p_expr
                                        } as *const Expr, p_e_1 as *const Expr, -1)
                            } < 2 {
                        return i + 1;
                    }
                    break '__c15;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}
extern "C" fn resolve_compound_order_by(p_parse_1: *mut Parse,
    mut p_select_1: *mut Select) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p_order_by: *mut ExprList = core::ptr::null_mut();
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut more_to_do: i32 = 1;
        p_order_by = unsafe { (*p_select_1).p_order_by };
        if p_order_by == core::ptr::null_mut() { return 0; }
        db = unsafe { (*p_parse_1).db };
        if unsafe { (*p_order_by).n_expr } >
                unsafe { (*db).a_limit[2 as usize] } {
            unsafe {
                sqlite3_error_msg(p_parse_1,
                    c"too many terms in ORDER BY clause".as_ptr() as *mut i8 as
                        *const i8)
            };
            return 1;
        }
        {
            i = 0;
            '__b16: loop {
                if !(i < unsafe { (*p_order_by).n_expr }) { break '__b16; }
                '__c16: loop {
                    unsafe {
                        (*(unsafe { (*p_order_by).a.as_ptr() } as
                                            *mut ExprList_item).offset(i as
                                            isize)).fg.set_done(0 as u32 as u32)
                    };
                    break '__c16;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { (*p_select_1).p_next = core::ptr::null_mut() };
        while !(unsafe { (*p_select_1).p_prior }).is_null() {
            unsafe {
                (*unsafe { (*p_select_1).p_prior }).p_next = p_select_1
            };
            p_select_1 = unsafe { (*p_select_1).p_prior };
        }
        while !(p_select_1).is_null() && more_to_do != 0 {
            let mut p_item: *mut ExprList_item = core::ptr::null_mut();
            more_to_do = 0;
            p_e_list = unsafe { (*p_select_1).p_e_list };
            { let _ = 0; };
            {
                {
                    i = 0;
                    p_item =
                        unsafe { (*p_order_by).a.as_ptr() } as *mut ExprList_item
                };
                '__b19: loop {
                    if !(i < unsafe { (*p_order_by).n_expr }) { break '__b19; }
                    '__c19: loop {
                        let mut i_col: i32 = -1;
                        let mut p_e: *mut Expr = core::ptr::null_mut();
                        let mut p_dup: *mut Expr = core::ptr::null_mut();
                        if unsafe { (*p_item).fg.done() } != 0 { break '__c19; }
                        p_e =
                            unsafe {
                                sqlite3_expr_skip_collate_and_likely(unsafe {
                                        (*p_item).p_expr
                                    })
                            };
                        if p_e == core::ptr::null_mut() { break '__c19; }
                        if unsafe {
                                    sqlite3_expr_is_integer(p_e as *const Expr, &mut i_col,
                                        core::ptr::null_mut())
                                } != 0 {
                            if i_col <= 0 || i_col > unsafe { (*p_e_list).n_expr } {
                                resolve_out_of_range_error(p_parse_1,
                                    c"ORDER".as_ptr() as *mut i8 as *const i8, i + 1,
                                    unsafe { (*p_e_list).n_expr }, p_e as *const Expr);
                                return 1;
                            }
                        } else {
                            i_col =
                                resolve_as_name(p_parse_1 as *const Parse,
                                    unsafe { &*p_e_list }, unsafe { &*p_e });
                            if i_col == 0 {
                                p_dup =
                                    unsafe { sqlite3_expr_dup(db, p_e as *const Expr, 0) };
                                if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                                    { let _ = 0; };
                                    i_col =
                                        resolve_order_by_term_to_expr_list(p_parse_1,
                                            unsafe { &*p_select_1 }, p_dup);
                                    if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 &&
                                            i_col > 0 {
                                        resolve_order_by_term_to_expr_list(p_parse_1,
                                            unsafe { &*p_select_1 }, p_e);
                                    }
                                }
                                unsafe { sqlite3_expr_delete(db, p_dup) };
                            }
                        }
                        if i_col > 0 {
                            if !(unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2) as
                                        i32 != 0 {
                                let p_new: *mut Expr =
                                    unsafe { sqlite3_expr_int32(db, i_col) };
                                if p_new == core::ptr::null_mut() { return 1; }
                                if unsafe { (*p_item).p_expr } == p_e {
                                    unsafe { (*p_item).p_expr = p_new };
                                } else {
                                    let mut p_parent: *mut Expr = unsafe { (*p_item).p_expr };
                                    { let _ = 0; };
                                    while unsafe { (*unsafe { (*p_parent).p_left }).op } as i32
                                            == 114 {
                                        p_parent = unsafe { (*p_parent).p_left };
                                    }
                                    { let _ = 0; };
                                    unsafe { (*p_parent).p_left = p_new };
                                }
                                unsafe { sqlite3_expr_delete(db, p_e) };
                                unsafe { (*p_item).u.x.i_order_by_col = i_col as u16 };
                            }
                            unsafe { (*p_item).fg.set_done(1 as u32 as u32) };
                        } else { more_to_do = 1; }
                        break '__c19;
                    }
                    {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        {
                            let __p = &mut p_item;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
            p_select_1 = unsafe { (*p_select_1).p_next };
        }
        {
            i = 0;
            '__b21: loop {
                if !(i < unsafe { (*p_order_by).n_expr }) { break '__b21; }
                '__c21: loop {
                    if unsafe {
                                    (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                        *mut ExprList_item).offset(i as isize)).fg.done()
                                } as i32 == 0 {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"%r ORDER BY term does not match any column in the result set".as_ptr()
                                        as *mut i8 as *const i8, i + 1)
                        };
                        return 1;
                    }
                    break '__c21;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}
extern "C" fn resolve_select_step(p_walker_1: *mut Walker, mut p: *mut Select)
    -> i32 {
    unsafe {
        let mut p_outer_nc: *mut NameContext = core::ptr::null_mut();
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        let mut is_compound: i32 = 0;
        let mut n_compound: i32 = 0;
        let mut p_parse: *mut Parse = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut p_group_by: *mut ExprList = core::ptr::null_mut();
        let mut p_leftmost: *mut Select = core::ptr::null_mut();
        let mut db: *const sqlite3 = core::ptr::null();
        { let _ = 0; };
        if unsafe { (*p).sel_flags } & 4 as u32 != 0 { return 1; }
        p_outer_nc = unsafe { (*p_walker_1).u.p_nc };
        p_parse = unsafe { (*p_walker_1).p_parse };
        db = unsafe { (*p_parse).db };
        if unsafe { (*p).sel_flags } & 64 as u32 == 0 as u32 {
            unsafe { sqlite3_select_prep(p_parse, p, p_outer_nc) };
            return if unsafe { (*p_parse).n_err } != 0 { 2 } else { 1 };
        }
        is_compound =
            (unsafe { (*p).p_prior } != core::ptr::null_mut()) as i32;
        n_compound = 0;
        p_leftmost = p;
        while !(p).is_null() {
            { let _ = 0; };
            { let _ = 0; };
            unsafe { (*p).sel_flags |= 4 as u32 };
            unsafe {
                memset(&raw mut s_nc as *mut (), 0,
                    core::mem::size_of::<NameContext>() as u64)
            };
            s_nc.p_parse = p_parse;
            s_nc.p_win_select = p;
            if sqlite3_resolve_expr_names(&mut s_nc, unsafe { (*p).p_limit })
                    != 0 {
                return 2;
            }
            if unsafe { (*p).sel_flags } & 65536 as u32 != 0 {
                let mut p_sub: *mut Select = core::ptr::null_mut();
                { let _ = 0; };
                { let _ = 0; };
                p_sub =
                    unsafe {
                        (*unsafe {
                                        (*(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                            *mut SrcItem).offset(0 as isize)).u4.p_subq
                                    }).p_select
                    };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                unsafe { (*p_sub).p_order_by = unsafe { (*p).p_order_by } };
                unsafe { (*p).p_order_by = core::ptr::null_mut() };
            }
            if !(p_outer_nc).is_null() {
                {
                    let __p = unsafe { &mut (*p_outer_nc).n_nested_select };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
            {
                i = 0;
                '__b23: loop {
                    if !(i < unsafe { (*unsafe { (*p).p_src }).n_src }) {
                        break '__b23;
                    }
                    '__c23: loop {
                        let p_item: *mut SrcItem =
                            unsafe {
                                &mut *(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)
                            };
                        { let _ = 0; };
                        if unsafe { (*p_item).fg.is_subquery() } != 0 &&
                                unsafe {
                                            (*unsafe {
                                                            (*unsafe { (*p_item).u4.p_subq }).p_select
                                                        }).sel_flags
                                        } & 4 as u32 == 0 as u32 {
                            let n_ref: i32 =
                                if !(p_outer_nc).is_null() {
                                    unsafe { (*p_outer_nc).n_ref }
                                } else { 0 };
                            let z_saved_context: *const i8 =
                                unsafe { (*p_parse).z_auth_context };
                            if !(unsafe { (*p_item).z_name }).is_null() {
                                unsafe {
                                    (*p_parse).z_auth_context =
                                        unsafe { (*p_item).z_name } as *const i8
                                };
                            }
                            sqlite3_resolve_select_names(p_parse,
                                unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select },
                                p_outer_nc);
                            unsafe { (*p_parse).z_auth_context = z_saved_context };
                            if unsafe { (*p_parse).n_err } != 0 { return 2; }
                            { let _ = 0; };
                            if !(p_outer_nc).is_null() {
                                { let _ = 0; };
                                unsafe {
                                    (*p_item).fg.set_is_correlated((unsafe {
                                                        (*p_outer_nc).n_ref
                                                    } > n_ref) as u32 as u32)
                                };
                            }
                        }
                        break '__c23;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if !(p_outer_nc).is_null() &&
                    unsafe { (*p_outer_nc).n_nested_select } > 0 as u32 {
                {
                    let __p = unsafe { &mut (*p_outer_nc).n_nested_select };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
            s_nc.nc_flags = 1 | 16384;
            s_nc.p_src_list = unsafe { (*p).p_src };
            s_nc.p_next = p_outer_nc;
            if sqlite3_resolve_expr_list_names(&mut s_nc,
                        unsafe { (*p).p_e_list }) != 0 {
                return 2;
            }
            s_nc.nc_flags &= !16384;
            { let _ = 0; };
            p_group_by = unsafe { (*p).p_group_by };
            if !(p_group_by).is_null() || s_nc.nc_flags & 16 != 0 {
                { let _ = 0; };
                { let _ = 0; };
                unsafe {
                    (*p).sel_flags |=
                        (8 | s_nc.nc_flags & (4096 | 134217728)) as u32
                };
            } else { s_nc.nc_flags &= !1; }
            { let _ = 0; };
            s_nc.u_nc.p_e_list = unsafe { (*p).p_e_list };
            s_nc.nc_flags |= 128;
            if !(unsafe { (*p).p_having }).is_null() {
                if unsafe { (*p).sel_flags } & 8 as u32 == 0 as u32 {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"HAVING clause on a non-aggregate query".as_ptr() as
                                    *mut i8 as *const i8)
                    };
                    return 2;
                }
                if sqlite3_resolve_expr_names(&mut s_nc,
                            unsafe { (*p).p_having }) != 0 {
                    return 2;
                }
            }
            s_nc.nc_flags |= 1048576;
            if sqlite3_resolve_expr_names(&mut s_nc, unsafe { (*p).p_where })
                    != 0 {
                return 2;
            }
            s_nc.nc_flags &= !1048576;
            {
                i = 0;
                '__b24: loop {
                    if !(i < unsafe { (*unsafe { (*p).p_src }).n_src }) {
                        break '__b24;
                    }
                    '__c24: loop {
                        let p_item_1: *const SrcItem =
                            unsafe {
                                    &raw mut *(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                    *mut SrcItem).offset(i as isize)
                                } as *const SrcItem;
                        if unsafe { (*p_item_1).fg.is_tab_func() } != 0 &&
                                sqlite3_resolve_expr_list_names(&mut s_nc,
                                        unsafe { (*p_item_1).u1.p_func_arg }) != 0 {
                            return 2;
                        }
                        break '__c24;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                let mut p_win: *const Window = core::ptr::null();
                {
                    p_win = unsafe { (*p).p_win_defn };
                    '__b25: loop {
                        if !(!(p_win).is_null()) { break '__b25; }
                        '__c25: loop {
                            if sqlite3_resolve_expr_list_names(&mut s_nc,
                                            unsafe { (*p_win).p_order_by }) != 0 ||
                                    sqlite3_resolve_expr_list_names(&mut s_nc,
                                            unsafe { (*p_win).p_partition }) != 0 {
                                return 2;
                            }
                            break '__c25;
                        }
                        p_win = unsafe { (*p_win).p_next_win };
                    }
                }
            }
            s_nc.nc_flags |= 1 | 16384;
            if unsafe { (*p).sel_flags } & 65536 as u32 != 0 {
                let mut p_sub_1: *mut Select = core::ptr::null_mut();
                { let _ = 0; };
                p_sub_1 =
                    unsafe {
                        (*unsafe {
                                        (*(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                            *mut SrcItem).offset(0 as isize)).u4.p_subq
                                    }).p_select
                    };
                { let _ = 0; };
                unsafe { (*p).p_order_by = unsafe { (*p_sub_1).p_order_by } };
                unsafe { (*p_sub_1).p_order_by = core::ptr::null_mut() };
            }
            if unsafe { (*p).p_order_by } != core::ptr::null_mut() &&
                        is_compound <= n_compound &&
                    resolve_order_group_by(&mut s_nc, p,
                            unsafe { (*p).p_order_by },
                            c"ORDER".as_ptr() as *mut i8 as *const i8) != 0 {
                return 2;
            }
            if unsafe { (*db).malloc_failed } != 0 { return 2; }
            s_nc.nc_flags &= !16384;
            if !(p_group_by).is_null() {
                let mut p_item_2: *const ExprList_item = core::ptr::null();
                if resolve_order_group_by(&mut s_nc, p, p_group_by,
                                c"GROUP".as_ptr() as *mut i8 as *const i8) != 0 ||
                        unsafe { (*db).malloc_failed } != 0 {
                    return 2;
                }
                {
                    {
                        i = 0;
                        p_item_2 =
                            unsafe { (*p_group_by).a.as_ptr() } as *mut ExprList_item
                    };
                    '__b26: loop {
                        if !(i < unsafe { (*p_group_by).n_expr }) { break '__b26; }
                        '__c26: loop {
                            if unsafe { (*unsafe { (*p_item_2).p_expr }).flags } &
                                        16 as u32 != 0 as u32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"aggregate functions are not allowed in the GROUP BY clause".as_ptr()
                                                as *mut i8 as *const i8)
                                };
                                return 2;
                            }
                            break '__c26;
                        }
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut p_item_2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                    }
                }
            }
            if !(unsafe { (*p).p_next }).is_null() &&
                    unsafe { (*unsafe { (*p).p_e_list }).n_expr } !=
                        unsafe {
                            (*unsafe { (*unsafe { (*p).p_next }).p_e_list }).n_expr
                        } {
                unsafe {
                    sqlite3_select_wrong_num_terms_error(p_parse,
                        unsafe { (*p).p_next })
                };
                return 2;
            }
            if unsafe { (*p).sel_flags } & 1073741824 as u32 != 0 {
                unsafe { sqlite3_select_check_on_clauses(p_parse, p) };
                if unsafe { (*p_parse).n_err } != 0 { return 2; }
            }
            p = unsafe { (*p).p_prior };
            { let __p = &mut n_compound; let __t = *__p; *__p += 1; __t };
        }
        if is_compound != 0 &&
                resolve_compound_order_by(p_parse, p_leftmost) != 0 {
            return 2;
        }
        return 1;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_resolve_expr_names(p_nc: *mut NameContext,
    p_expr: *mut Expr) -> i32 {
    unsafe {
        let mut saved_has_agg: i32 = 0;
        let mut w: Walker = unsafe { core::mem::zeroed() };
        if p_expr == core::ptr::null_mut() { return 0; }
        saved_has_agg =
            unsafe { (*p_nc).nc_flags } & (16 | 4096 | 32768 | 134217728);
        unsafe { (*p_nc).nc_flags &= !(16 | 4096 | 32768 | 134217728) };
        w.p_parse = unsafe { (*p_nc).p_parse };
        w.x_expr_callback = Some(resolve_expr_step);
        w.x_select_callback =
            if unsafe { (*p_nc).nc_flags } & 524288 != 0 {
                None
            } else { Some(resolve_select_step) };
        w.x_select_callback2 = None;
        w.u.p_nc = p_nc;
        unsafe { (*w.p_parse).n_height += unsafe { (*p_expr).n_height } };
        if unsafe {
                    sqlite3_expr_check_height(w.p_parse,
                        unsafe { (*w.p_parse).n_height })
                } != 0 {
            return 1;
        }
        { let _ = 0; };
        unsafe { sqlite3_walk_expr_nn(&mut w, p_expr) };
        unsafe { (*w.p_parse).n_height -= unsafe { (*p_expr).n_height } };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            (*p_expr).flags |=
                (unsafe { (*p_nc).nc_flags } & (16 | 32768)) as u32
        };
        unsafe { (*p_nc).nc_flags |= saved_has_agg };
        return (unsafe { (*p_nc).n_nc_err } > 0 ||
                    unsafe { (*w.p_parse).n_err } > 0) as i32;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_resolve_self_reference(p_parse: *mut Parse,
    p_tab: *mut Table, mut type_: i32, p_expr: *mut Expr,
    p_list: *mut ExprList) -> i32 {
    unsafe {
        let mut p_src: *mut SrcList = core::ptr::null_mut();
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        let mut rc: i32 = 0;
        let mut u_src:
                sqlite3_resolve_self_reference_u0_N33sqlite3_resolve_self_reference_u0 =
            unsafe { core::mem::zeroed() };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            memset(&raw mut s_nc as *mut (), 0,
                core::mem::size_of::<NameContext>() as u64)
        };
        unsafe { memset(&raw mut u_src as *mut (), 0, 80) };
        p_src = &mut u_src.s_src;
        if !(p_tab).is_null() {
            unsafe { (*p_src).n_src = 1 };
            unsafe {
                (*(unsafe { (*p_src).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)).z_name =
                    unsafe { (*p_tab).z_name }
            };
            unsafe {
                (*(unsafe { (*p_src).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)).p_s_tab = p_tab
            };
            unsafe {
                (*(unsafe { (*p_src).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)).i_cursor = -1
            };
            if unsafe { (*p_tab).p_schema } !=
                    unsafe {
                        (*unsafe {
                                    (*unsafe { (*p_parse).db }).a_db.offset(1 as isize)
                                }).p_schema
                    } {
                type_ |= 262144;
            }
        }
        s_nc.p_parse = p_parse;
        s_nc.p_src_list = p_src;
        s_nc.nc_flags = type_ | 65536;
        if { rc = sqlite3_resolve_expr_names(&mut s_nc, p_expr); rc } != 0 {
            return rc;
        }
        if !(p_list).is_null() {
            rc = sqlite3_resolve_expr_list_names(&mut s_nc, p_list);
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_column_expr(db: *mut sqlite3,
    p_src: &mut SrcList, i_src: i32, i_col: i32) -> *mut Expr {
    unsafe {
        let p: *mut Expr =
            unsafe { sqlite3_expr_alloc(db, 168, core::ptr::null(), 0) };
        if !(p).is_null() {
            let p_item: *mut SrcItem =
                unsafe {
                    &mut *((*p_src).a.as_ptr() as
                                    *mut SrcItem).offset(i_src as isize)
                };
            let mut p_tab: *const Table = core::ptr::null();
            { let _ = 0; };
            p_tab =
                {
                    unsafe { (*p).y.p_tab = unsafe { (*p_item).p_s_tab } };
                    unsafe { (*p).y.p_tab }
                };
            unsafe { (*p).i_table = unsafe { (*p_item).i_cursor } };
            if unsafe { (*unsafe { (*p).y.p_tab }).i_p_key } as i32 == i_col {
                unsafe { (*p).i_column = -1 as ynVar };
            } else {
                unsafe { (*p).i_column = i_col as ynVar };
                if unsafe { (*p_tab).tab_flags } & 96 as u32 != 0 as u32 &&
                        unsafe {
                                        (*unsafe {
                                                    (*p_tab).a_col.offset(i_col as isize)
                                                }).col_flags
                                    } as i32 & 96 != 0 {
                    unsafe {
                        (*p_item).col_used =
                            if unsafe { (*p_tab).n_col } as i32 >= 64 {
                                -1i32 as Bitmask
                            } else {
                                ((1 as Bitmask) << unsafe { (*p_tab).n_col }) - 1 as Bitmask
                            }
                    };
                } else {
                    unsafe {
                        (*p_item).col_used |=
                            (1 as Bitmask) <<
                                if i_col >=
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                        1
                                } else { i_col }
                    };
                }
            }
        }
        return p;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
union sqlite3_resolve_self_reference_u0_N33sqlite3_resolve_self_reference_u0 {
    s_src: SrcList,
    src_space: [u8; 80],
}
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
    fn sqlite3_str_i_hash(_: *const i8)
    -> u8;
    fn sqlite3_record_error_offset_of_expr(_: *mut sqlite3, _: *const Expr)
    -> ();
    fn sqlite3_rename_token_remap(_: *mut Parse, p_to_1: *const (),
    p_from_1: *const ())
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn sqlite3_expr_vector_size(p_expr_1: *const Expr)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_expr_check_height(_: *mut Parse, _: i32)
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
    fn sqlite3_rc_str_ref(_: *mut i8)
    -> *mut i8;
    fn sqlite3_rc_str_unref(_: *mut ())
    -> ();
    fn sqlite3_rc_str_new(_: u64)
    -> *mut i8;
    fn sqlite3_rc_str_resize(_: *mut i8, _: u64)
    -> *mut i8;
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
    fn sqlite3_record_error_byte_offset(_: *mut sqlite3, _: *const i8)
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