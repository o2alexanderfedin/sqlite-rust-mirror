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
struct WhereConst {
    p_parse: *mut Parse,
    p_oom_fault: *mut u8,
    n_const: i32,
    n_chng: i32,
    b_has_aff_blob: i32,
    m_exclude_on: u32,
    ap_expr: *mut *mut Expr,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct CheckOnCtx {
    p_src: *mut SrcList,
    i_join: i32,
    b_func_arg: i32,
    p_parent: *mut CheckOnCtx,
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
extern "C" fn find_rightmost(mut p: *mut Select) -> *mut Select {
    while !(unsafe { (*p).p_next }).is_null() { p = unsafe { (*p).p_next }; }
    return p;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_pop_with(p_walker: *mut Walker,
    p: *mut Select) -> () {
    let p_parse: *mut Parse = unsafe { (*p_walker).p_parse };
    if !(unsafe { (*p_parse).p_with }).is_null() &&
            unsafe { (*p).p_prior } == core::ptr::null_mut() {
        let p_with: *const With =
            unsafe { (*find_rightmost(p)).p_with } as *const With;
        if p_with != core::ptr::null_mut() {
            { let _ = 0; };
            unsafe { (*p_parse).p_with = unsafe { (*p_with).p_outer } };
        }
    }
}
extern "C" fn column_type_impl(mut p_nc_1: *mut NameContext, p_expr_1: &Expr)
    -> *const i8 {
    unsafe {
        let mut z_type: *const i8 = core::ptr::null();
        let mut j: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        '__s1:
            {
            match (*p_expr_1).op {
                168 => {
                    {
                        let mut p_tab: *const Table = core::ptr::null();
                        let mut p_s: *const Select = core::ptr::null();
                        let i_col: i32 = (*p_expr_1).i_column as i32;
                        while !(p_nc_1).is_null() && (p_tab).is_null() as i32 != 0 {
                            let p_tab_list: *const SrcList =
                                unsafe { (*p_nc_1).p_src_list } as *const SrcList;
                            {
                                j = 0;
                                '__b3: loop {
                                    if !(j < unsafe { (*p_tab_list).n_src } &&
                                                    unsafe {
                                                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                            *mut SrcItem).offset(j as isize)).i_cursor
                                                        } != (*p_expr_1).i_table) {
                                        break '__b3;
                                    }
                                    '__c3: loop { break '__c3; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if j < unsafe { (*p_tab_list).n_src } {
                                p_tab =
                                    unsafe {
                                        (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                        *mut SrcItem).offset(j as isize)).p_s_tab
                                    };
                                if unsafe {
                                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                *mut SrcItem).offset(j as isize)).fg.is_subquery()
                                        } != 0 {
                                    p_s =
                                        unsafe {
                                            (*unsafe {
                                                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                                *mut SrcItem).offset(j as isize)).u4.p_subq
                                                        }).p_select
                                        };
                                } else { p_s = core::ptr::null_mut(); }
                            } else { p_nc_1 = unsafe { (*p_nc_1).p_next }; }
                        }
                        if p_tab == core::ptr::null_mut() { break '__s1; }
                        { let _ = 0; };
                        if !(p_s).is_null() {
                            if i_col < unsafe { (*unsafe { (*p_s).p_e_list }).n_expr }
                                    && ((0 == 0) as i32 != 0 || i_col >= 0) {
                                let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
                                let mut p: *mut Expr =
                                    unsafe {
                                        (*(unsafe { (*unsafe { (*p_s).p_e_list }).a.as_ptr() } as
                                                        *mut ExprList_item).offset(i_col as isize)).p_expr
                                    };
                                s_nc.p_src_list = unsafe { (*p_s).p_src };
                                s_nc.p_next = p_nc_1;
                                s_nc.p_parse = unsafe { (*p_nc_1).p_parse };
                                z_type = column_type_impl(&mut s_nc, unsafe { &*p });
                            }
                        } else {
                            { let _ = 0; };
                            { let _ = 0; };
                            if i_col < 0 {
                                z_type = c"INTEGER".as_ptr() as *mut i8 as *const i8;
                            } else {
                                z_type =
                                    unsafe {
                                            sqlite3ColumnType(unsafe {
                                                    &mut *unsafe { (*p_tab).a_col.offset(i_col as isize) }
                                                }, core::ptr::null_mut())
                                        } as *const i8;
                            }
                        }
                        break '__s1;
                    }
                    {
                        let mut s_nc_1: NameContext =
                            unsafe { core::mem::zeroed() };
                        let mut p_s_1: *const Select = core::ptr::null();
                        let mut p: *mut Expr = core::ptr::null_mut();
                        { let _ = 0; };
                        p_s_1 = (*p_expr_1).x.p_select;
                        p =
                            unsafe {
                                (*(unsafe { (*unsafe { (*p_s_1).p_e_list }).a.as_ptr() } as
                                                *mut ExprList_item).offset(0 as isize)).p_expr
                            };
                        s_nc_1.p_src_list = unsafe { (*p_s_1).p_src };
                        s_nc_1.p_next = p_nc_1;
                        s_nc_1.p_parse = unsafe { (*p_nc_1).p_parse };
                        z_type = column_type_impl(&mut s_nc_1, unsafe { &*p });
                        break '__s1;
                    }
                }
                139 => {
                    {
                        let mut s_nc_1: NameContext =
                            unsafe { core::mem::zeroed() };
                        let mut p_s_1: *const Select = core::ptr::null();
                        let mut p: *mut Expr = core::ptr::null_mut();
                        { let _ = 0; };
                        p_s_1 = (*p_expr_1).x.p_select;
                        p =
                            unsafe {
                                (*(unsafe { (*unsafe { (*p_s_1).p_e_list }).a.as_ptr() } as
                                                *mut ExprList_item).offset(0 as isize)).p_expr
                            };
                        s_nc_1.p_src_list = unsafe { (*p_s_1).p_src };
                        s_nc_1.p_next = p_nc_1;
                        s_nc_1.p_parse = unsafe { (*p_nc_1).p_parse };
                        z_type = column_type_impl(&mut s_nc_1, unsafe { &*p });
                        break '__s1;
                    }
                }
                _ => {}
            }
        }
        return z_type;
    }
}
extern "C" fn generate_column_types(p_parse_1: *mut Parse,
    p_tab_list_1: *mut SrcList, p_e_list_1: &ExprList) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        s_nc.p_src_list = p_tab_list_1;
        s_nc.p_parse = p_parse_1;
        s_nc.p_next = core::ptr::null_mut();
        {
            i = 0;
            '__b4: loop {
                if !(i < (*p_e_list_1).n_expr) { break '__b4; }
                '__c4: loop {
                    let p: *mut Expr =
                        unsafe {
                            (*((*p_e_list_1).a.as_ptr() as
                                            *mut ExprList_item).offset(i as isize)).p_expr
                        };
                    let mut z_type: *const i8 = core::ptr::null();
                    z_type = column_type_impl(&mut s_nc, unsafe { &*p });
                    unsafe {
                        sqlite3_vdbe_set_col_name(v, i, 1, z_type,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_generate_column_names(p_parse: *mut Parse,
    mut p_select: *mut Select) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
        let mut i: i32 = 0;
        let mut p_tab: *const Table = core::ptr::null();
        let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let db: *mut sqlite3 = unsafe { (*p_parse).db };
        let mut full_name: i32 = 0;
        let mut src_name: i32 = 0;
        if unsafe { (*p_parse).col_names_set() } != 0 { return; }
        while !(unsafe { (*p_select).p_prior }).is_null() {
            p_select = unsafe { (*p_select).p_prior };
        }
        p_tab_list = unsafe { (*p_select).p_src };
        p_e_list = unsafe { (*p_select).p_e_list };
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_parse).set_col_names_set(1 as bft as u32) };
        full_name = (unsafe { (*db).flags } & 4 as u64 != 0 as u64) as i32;
        src_name =
            (unsafe { (*db).flags } & 64 as u64 != 0 as u64 || full_name != 0)
                as i32;
        unsafe {
            sqlite3_vdbe_set_num_cols(v, unsafe { (*p_e_list).n_expr })
        };
        {
            i = 0;
            '__b6: loop {
                if !(i < unsafe { (*p_e_list).n_expr }) { break '__b6; }
                '__c6: loop {
                    let p: *const Expr =
                        unsafe {
                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(i as isize)).p_expr
                            } as *const Expr;
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    if !(unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprList_item).offset(i as isize)).z_e_name
                                        }).is_null() &&
                            unsafe {
                                        (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprList_item).offset(i as isize)).fg.e_e_name()
                                    } as i32 == 0 {
                        let z_name: *const i8 =
                            unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                    *mut ExprList_item).offset(i as isize)).z_e_name
                                } as *const i8;
                        unsafe {
                            sqlite3_vdbe_set_col_name(v, i, 0, z_name as *const i8,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                    } else if src_name != 0 && unsafe { (*p).op } as i32 == 168
                        {
                        let mut z_col: *mut i8 = core::ptr::null_mut();
                        let mut i_col: i32 = unsafe { (*p).i_column } as i32;
                        p_tab = unsafe { (*p).y.p_tab };
                        { let _ = 0; };
                        if i_col < 0 { i_col = unsafe { (*p_tab).i_p_key } as i32; }
                        { let _ = 0; };
                        if i_col < 0 {
                            z_col = c"rowid".as_ptr() as *mut i8;
                        } else {
                            z_col =
                                unsafe {
                                    (*unsafe {
                                                (*p_tab).a_col.offset(i_col as isize)
                                            }).z_cn_name
                                };
                        }
                        if full_name != 0 {
                            let mut z_name_1: *const i8 = core::ptr::null();
                            z_name_1 =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"%s.%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab).z_name }, z_col)
                                };
                            unsafe {
                                sqlite3_vdbe_set_col_name(v, i, 0, z_name_1 as *const i8,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(sqlite3_row_set_clear as *const ())
                                        }))
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_set_col_name(v, i, 0, z_col as *const i8,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }))
                            };
                        }
                    } else {
                        let mut z: *const i8 =
                            unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                    *mut ExprList_item).offset(i as isize)).z_e_name
                                } as *const i8;
                        z =
                            if z == core::ptr::null() {
                                    unsafe {
                                        sqlite3_m_printf(db,
                                            c"column%d".as_ptr() as *mut i8 as *const i8, i + 1)
                                    }
                                } else { unsafe { sqlite3_db_str_dup(db, z) } } as
                                *const i8;
                        unsafe {
                            sqlite3_vdbe_set_col_name(v, i, 0, z,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(sqlite3_row_set_clear as *const ())
                                    }))
                        };
                    }
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        generate_column_types(p_parse, p_tab_list, unsafe { &*p_e_list });
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_columns_from_expr_list(p_parse: *mut Parse,
    p_e_list: *mut ExprList, pn_col: &mut i16, pa_col: &mut *mut Column)
    -> i32 {
    unsafe {
        unsafe {
            let db: *mut sqlite3 = unsafe { (*p_parse).db };
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut cnt: u32 = 0 as u32;
            let mut a_col: *mut Column = core::ptr::null_mut();
            let mut p_col: *mut Column = core::ptr::null_mut();
            let mut n_col: i32 = 0;
            let mut z_name: *mut i8 = core::ptr::null_mut();
            let mut n_name: i32 = 0;
            let mut ht: Hash = unsafe { core::mem::zeroed() };
            let mut p_tab: *const Table = core::ptr::null();
            unsafe { sqlite3_hash_init(&mut ht) };
            if !(p_e_list).is_null() {
                n_col = unsafe { (*p_e_list).n_expr };
                a_col =
                    unsafe {
                            sqlite3_db_malloc_zero(db,
                                core::mem::size_of::<Column>() as u64 * n_col as u64)
                        } as *mut Column;
                if n_col > 32767 { n_col = 32767; }
            } else { n_col = 0; a_col = core::ptr::null_mut(); }
            { let _ = 0; };
            *pn_col = n_col as i16;
            *pa_col = a_col;
            {
                { i = 0; p_col = a_col };
                '__b7: loop {
                    if !(i < n_col &&
                                    (unsafe { (*p_parse).n_err } == 0) as i32 != 0) {
                        break '__b7;
                    }
                    '__c7: loop {
                        let p_x: *mut ExprList_item =
                            unsafe {
                                &mut *(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(i as isize)
                            };
                        let mut p_collide: *const ExprList_item = core::ptr::null();
                        if { z_name = unsafe { (*p_x).z_e_name }; z_name } !=
                                    core::ptr::null_mut() &&
                                unsafe { (*p_x).fg.e_e_name() } as i32 == 0
                            {} else {
                            let mut p_col_expr: *const Expr =
                                unsafe {
                                        sqlite3_expr_skip_collate_and_likely(unsafe {
                                                (*p_x).p_expr
                                            })
                                    } as *const Expr;
                            while p_col_expr != core::ptr::null_mut() &&
                                    unsafe { (*p_col_expr).op } as i32 == 142 {
                                p_col_expr = unsafe { (*p_col_expr).p_right };
                                { let _ = 0; };
                            }
                            if unsafe { (*p_col_expr).op } as i32 == 168 &&
                                        unsafe { (*p_col_expr).flags } &
                                                (16777216 | 33554432) as u32 == 0 as u32 &&
                                    unsafe { (*p_col_expr).y.p_tab } != core::ptr::null_mut() {
                                let mut i_col: i32 =
                                    unsafe { (*p_col_expr).i_column } as i32;
                                p_tab = unsafe { (*p_col_expr).y.p_tab };
                                if i_col < 0 { i_col = unsafe { (*p_tab).i_p_key } as i32; }
                                z_name =
                                    if i_col >= 0 {
                                        unsafe {
                                            (*unsafe {
                                                        (*p_tab).a_col.offset(i_col as isize)
                                                    }).z_cn_name
                                        }
                                    } else { c"rowid".as_ptr() as *mut i8 };
                            } else if unsafe { (*p_col_expr).op } as i32 == 60 {
                                { let _ = 0; };
                                z_name = unsafe { (*p_col_expr).u.z_token };
                            } else { { let _ = 0; }; }
                        }
                        if !(z_name).is_null() &&
                                (unsafe { sqlite3_is_true_or_false(z_name as *const i8) } ==
                                            0) as i32 != 0 {
                            z_name =
                                unsafe { sqlite3_db_str_dup(db, z_name as *const i8) };
                        } else {
                            z_name =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"column%d".as_ptr() as *mut i8 as *const i8, i + 1)
                                };
                        }
                        cnt = 0 as u32;
                        while !(z_name).is_null() &&
                                {
                                        p_collide =
                                            unsafe {
                                                    sqlite3_hash_find(&raw mut ht as *const Hash,
                                                        z_name as *const i8)
                                                } as *mut ExprList_item;
                                        p_collide
                                    } != core::ptr::null_mut() {
                            if unsafe { (*p_collide).fg.b_using_term() } != 0 {
                                unsafe { (*p_col).col_flags |= 1024 as u16 };
                            }
                            n_name = unsafe { sqlite3_strlen30(z_name as *const i8) };
                            if n_name > 0 {
                                {
                                    j = n_name - 1;
                                    '__b10: loop {
                                        if !(j > 0 &&
                                                        unsafe {
                                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                                    *const u8).add(unsafe { *z_name.offset(j as isize) } as u8
                                                                                    as usize)
                                                                    } as i32 & 4 != 0) {
                                            break '__b10;
                                        }
                                        '__c10: loop { break '__c10; }
                                        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                    }
                                }
                                if unsafe { *z_name.offset(j as isize) } as i32 ==
                                        ':' as i32 {
                                    n_name = j;
                                }
                            }
                            z_name =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"%.*z:%u".as_ptr() as *mut i8 as *const i8, n_name, z_name,
                                        { let __p = &mut cnt; *__p += 1; *__p })
                                };
                            unsafe { sqlite3_progress_check(p_parse) };
                            if cnt > 3 as u32 {
                                unsafe {
                                    sqlite3_randomness(core::mem::size_of::<u32>() as i32,
                                        &raw mut cnt as *mut ())
                                };
                            }
                        }
                        unsafe { (*p_col).z_cn_name = z_name };
                        unsafe {
                            (*p_col).h_name =
                                unsafe { sqlite3_str_i_hash(z_name as *const i8) }
                        };
                        if unsafe { (*p_x).fg.b_no_expand() } != 0 {
                            unsafe { (*p_col).col_flags |= 1024 as u16 };
                        }
                        if !(z_name).is_null() &&
                                unsafe {
                                        sqlite3_hash_insert(&mut ht, z_name as *const i8,
                                            p_x as *mut ())
                                    } == p_x as *mut () {
                            unsafe { sqlite3_oom_fault(db) };
                        }
                        break '__c7;
                    }
                    {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        {
                            let __p = &mut p_col;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
            unsafe { sqlite3_hash_clear(&mut ht) };
            if unsafe { (*p_parse).n_err } != 0 {
                {
                    j = 0;
                    '__b11: loop {
                        if !(j < i) { break '__b11; }
                        '__c11: loop {
                            unsafe {
                                sqlite3_db_free(db,
                                    unsafe { (*a_col.offset(j as isize)).z_cn_name } as *mut ())
                            };
                            break '__c11;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { sqlite3_db_free(db, a_col as *mut ()) };
                *pa_col = core::ptr::null_mut();
                *pn_col = 0 as i16;
                return unsafe { (*p_parse).rc };
            }
            return 0;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_subquery_column_types(p_parse: *mut Parse,
    p_tab: &mut Table, mut p_select: *mut Select, aff: i8) -> () {
    unsafe {
        unsafe {
            let db: *mut sqlite3 = unsafe { (*p_parse).db };
            let mut p_col: *mut Column = core::ptr::null_mut();
            let mut p_coll: *const CollSeq = core::ptr::null();
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut p: *mut Expr = core::ptr::null_mut();
            let mut a: *const ExprList_item = core::ptr::null();
            let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*db).malloc_failed } != 0 ||
                    unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                return;
            }
            while !(unsafe { (*p_select).p_prior }).is_null() {
                p_select = unsafe { (*p_select).p_prior };
            }
            a =
                unsafe { (*unsafe { (*p_select).p_e_list }).a.as_ptr() } as
                    *mut ExprList_item;
            unsafe {
                memset(&raw mut s_nc as *mut (), 0,
                    core::mem::size_of::<NameContext>() as u64)
            };
            s_nc.p_src_list = unsafe { (*p_select).p_src };
            {
                { i = 0; p_col = (*p_tab).a_col };
                '__b13: loop {
                    if !(i < (*p_tab).n_col as i32) { break '__b13; }
                    '__c13: loop {
                        let mut z_type: *const i8 = core::ptr::null();
                        let mut n: i64 = 0 as i64;
                        let mut m: i32 = 0;
                        let mut p_s2: *mut Select = p_select;
                        (*p_tab).tab_flags |=
                            (unsafe { (*p_col).col_flags } as i32 & 98) as u32;
                        p = unsafe { (*a.offset(i as isize)).p_expr };
                        unsafe {
                            (*p_col).affinity =
                                unsafe { sqlite3_expr_affinity(p as *const Expr) }
                        };
                        while unsafe { (*p_col).affinity } as i32 <= 64 &&
                                unsafe { (*p_s2).p_next } != core::ptr::null_mut() {
                            m |=
                                unsafe {
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*unsafe { (*p_s2).p_e_list }).a.as_ptr() } as
                                                                *mut ExprList_item).offset(i as isize)).p_expr
                                            } as *const Expr)
                                };
                            p_s2 = unsafe { (*p_s2).p_next };
                            unsafe {
                                (*p_col).affinity =
                                    unsafe {
                                        sqlite3_expr_affinity(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_s2).p_e_list }).a.as_ptr() } as
                                                                    *mut ExprList_item).offset(i as isize)).p_expr
                                                } as *const Expr)
                                    }
                            };
                        }
                        if unsafe { (*p_col).affinity } as i32 <= 64 {
                            unsafe { (*p_col).affinity = aff };
                        }
                        if unsafe { (*p_col).affinity } as i32 >= 66 &&
                                (!(unsafe { (*p_s2).p_next }).is_null() || p_s2 != p_select)
                            {
                            {
                                p_s2 = unsafe { (*p_s2).p_next };
                                '__b15: loop {
                                    if !(!(p_s2).is_null()) { break '__b15; }
                                    '__c15: loop {
                                        m |=
                                            unsafe {
                                                sqlite3_expr_data_type(unsafe {
                                                            (*(unsafe { (*unsafe { (*p_s2).p_e_list }).a.as_ptr() } as
                                                                            *mut ExprList_item).offset(i as isize)).p_expr
                                                        } as *const Expr)
                                            };
                                        break '__c15;
                                    }
                                    p_s2 = unsafe { (*p_s2).p_next };
                                }
                            }
                            if unsafe { (*p_col).affinity } as i32 == 66 && m & 1 != 0 {
                                unsafe { (*p_col).affinity = 65 as i8 };
                            } else if unsafe { (*p_col).affinity } as i32 >= 67 &&
                                    m & 2 != 0 {
                                unsafe { (*p_col).affinity = 65 as i8 };
                            }
                            if unsafe { (*p_col).affinity } as i32 >= 67 &&
                                    unsafe { (*p).op } as i32 == 36 {
                                unsafe { (*p_col).affinity = 70 as i8 };
                            }
                        }
                        z_type = column_type_impl(&mut s_nc, unsafe { &*p });
                        if z_type == core::ptr::null() ||
                                unsafe { (*p_col).affinity } as i32 !=
                                    unsafe {
                                            sqlite3_affinity_type(z_type, core::ptr::null_mut())
                                        } as i32 {
                            if unsafe { (*p_col).affinity } as i32 == 67 ||
                                    unsafe { (*p_col).affinity } as i32 == 70 {
                                z_type = c"NUM".as_ptr() as *mut i8 as *const i8;
                            } else {
                                z_type = core::ptr::null();
                                {
                                    j = 1;
                                    '__b16: loop {
                                        if !(j < 6) { break '__b16; }
                                        '__c16: loop {
                                            if unsafe {
                                                            *(sqlite3_std_type_affinity.as_ptr() as
                                                                        *const i8).offset(j as isize)
                                                        } as i32 == unsafe { (*p_col).affinity } as i32 {
                                                z_type =
                                                    unsafe {
                                                        *(sqlite3_std_type.as_ptr() as
                                                                    *mut *const i8).offset(j as isize)
                                                    };
                                                break '__b16;
                                            }
                                            break '__c16;
                                        }
                                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            }
                        }
                        if !(z_type).is_null() {
                            let k: i64 = unsafe { strlen(z_type) } as i64;
                            n =
                                unsafe {
                                        strlen(unsafe { (*p_col).z_cn_name } as *const i8)
                                    } as i64;
                            unsafe {
                                (*p_col).z_cn_name =
                                    unsafe {
                                            sqlite3_db_realloc_or_free(db,
                                                unsafe { (*p_col).z_cn_name } as *mut (),
                                                (n + k as i64 + 2 as i64) as u64)
                                        } as *mut i8
                            };
                            unsafe { (*p_col).col_flags &= !(4 | 512) as u16 };
                            if !(unsafe { (*p_col).z_cn_name }).is_null() {
                                unsafe {
                                    memcpy(unsafe {
                                                &raw mut *unsafe {
                                                            (*p_col).z_cn_name.offset((n + 1 as i64) as isize)
                                                        }
                                            } as *mut (), z_type as *const (), (k + 1 as i64) as u64)
                                };
                                unsafe { (*p_col).col_flags |= 4 as u16 };
                            }
                        }
                        p_coll =
                            unsafe { sqlite3_expr_coll_seq(p_parse, p as *const Expr) };
                        if !(p_coll).is_null() {
                            { let _ = 0; };
                            unsafe {
                                sqlite3_column_set_coll(db, p_col,
                                    unsafe { (*p_coll).z_name } as *const i8)
                            };
                        }
                        break '__c13;
                    }
                    {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        {
                            let __p = &mut p_col;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
            (*p_tab).sz_tab_row = 1 as LogEst;
        }
    }
}
extern "C" fn convert_compound_select_to_subquery(p_walker_1: *mut Walker,
    p: *mut Select) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p_new: *mut Select = core::ptr::null_mut();
        let mut p_x: *const Select = core::ptr::null();
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut a: *const ExprList_item = core::ptr::null();
        let mut p_new_src: *mut SrcList = core::ptr::null_mut();
        let mut p_parse: *mut Parse = core::ptr::null_mut();
        let mut dummy: Token = unsafe { core::mem::zeroed() };
        if unsafe { (*p).p_prior } == core::ptr::null_mut() { return 0; }
        if unsafe { (*p).p_order_by } == core::ptr::null_mut() { return 0; }
        {
            p_x = p;
            '__b17: loop {
                if !(!(p_x).is_null() &&
                                (unsafe { (*p_x).op } as i32 == 136 ||
                                    unsafe { (*p_x).op } as i32 == 139)) {
                    break '__b17;
                }
                '__c17: loop { break '__c17; }
                p_x = unsafe { (*p_x).p_prior };
            }
        }
        if p_x == core::ptr::null_mut() { return 0; }
        a =
            unsafe { (*unsafe { (*p).p_order_by }).a.as_ptr() } as
                *mut ExprList_item;
        if unsafe { (*a.offset(0 as isize)).u.x.i_order_by_col } != 0 {
            return 0;
        }
        {
            i = unsafe { (*unsafe { (*p).p_order_by }).n_expr } - 1;
            '__b18: loop {
                if !(i >= 0) { break '__b18; }
                '__c18: loop {
                    if unsafe {
                                    (*unsafe { (*a.offset(i as isize)).p_expr }).flags
                                } & 512 as u32 != 0 {
                        break '__b18;
                    }
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        if i < 0 { return 0; }
        p_parse = unsafe { (*p_walker_1).p_parse };
        db = unsafe { (*p_parse).db };
        p_new =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<Select>() as u64)
                } as *mut Select;
        if p_new == core::ptr::null_mut() { return 2; }
        unsafe {
            memset(&raw mut dummy as *mut (), 0,
                core::mem::size_of::<Token>() as u64)
        };
        p_new_src =
            unsafe {
                sqlite3_src_list_append_from_term(p_parse,
                    core::ptr::null_mut(), core::ptr::null_mut(),
                    core::ptr::null_mut(), &mut dummy, p_new,
                    core::ptr::null_mut())
            };
        { let _ = 0; };
        if unsafe { (*p_parse).n_err } != 0 {
            unsafe { sqlite3_src_list_delete(db, p_new_src) };
            return 2;
        }
        unsafe { *p_new = unsafe { core::ptr::read(p) } };
        unsafe { (*p).p_src = p_new_src };
        unsafe {
            (*p).p_e_list =
                unsafe {
                    sqlite3_expr_list_append(p_parse, core::ptr::null_mut(),
                        unsafe { sqlite3_expr(db, 180, core::ptr::null()) })
                }
        };
        unsafe { (*p).op = 139 as u8 };
        unsafe { (*p).p_where = core::ptr::null_mut() };
        unsafe { (*p_new).p_group_by = core::ptr::null_mut() };
        unsafe { (*p_new).p_having = core::ptr::null_mut() };
        unsafe { (*p_new).p_order_by = core::ptr::null_mut() };
        unsafe { (*p).p_prior = core::ptr::null_mut() };
        unsafe { (*p).p_next = core::ptr::null_mut() };
        unsafe { (*p).p_with = core::ptr::null_mut() };
        unsafe { (*p).p_win_defn = core::ptr::null_mut() };
        unsafe { (*p).sel_flags &= !(256 as u32) };
        { let _ = 0; };
        unsafe { (*p).sel_flags |= 65536 as u32 };
        { let _ = 0; };
        unsafe { (*unsafe { (*p_new).p_prior }).p_next = p_new };
        unsafe { (*p_new).p_limit = core::ptr::null_mut() };
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_with_push(p_parse: *mut Parse,
    mut p_with: *mut With, b_free: u8) -> *mut With {
    if !(p_with).is_null() {
        if b_free != 0 {
            p_with =
                unsafe {
                        sqlite3_parser_add_cleanup(p_parse,
                            Some(sqlite3_with_delete_generic), p_with as *mut ())
                    } as *mut With;
            if p_with == core::ptr::null_mut() {
                return core::ptr::null_mut();
            }
        }
        if unsafe { (*p_parse).n_err } == 0 {
            { let _ = 0; };
            unsafe { (*p_with).p_outer = unsafe { (*p_parse).p_with } };
            unsafe { (*p_parse).p_with = p_with };
        }
    }
    return p_with;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expand_subquery(p_parse: *mut Parse,
    p_from: *mut SrcItem) -> i32 {
    unsafe {
        let mut p_sel: *const Select = core::ptr::null();
        let mut p_tab: *mut Table = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        p_sel = unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select };
        { let _ = 0; };
        unsafe {
            (*p_from).p_s_tab =
                {
                    p_tab =
                        unsafe {
                                sqlite3_db_malloc_zero(unsafe { (*p_parse).db },
                                    core::mem::size_of::<Table>() as u64)
                            } as *mut Table;
                    p_tab
                }
        };
        if p_tab == core::ptr::null_mut() { return 7; }
        unsafe { (*p_tab).n_tab_ref = 1 as u32 };
        if !(unsafe { (*p_from).z_alias }).is_null() {
            unsafe {
                (*p_tab).z_name =
                    unsafe {
                        sqlite3_db_str_dup(unsafe { (*p_parse).db },
                            unsafe { (*p_from).z_alias } as *const i8)
                    }
            };
        } else {
            unsafe {
                (*p_tab).z_name =
                    unsafe {
                        sqlite3_m_printf(unsafe { (*p_parse).db },
                            c"%!S".as_ptr() as *mut i8 as *const i8, p_from)
                    }
            };
        }
        while !(unsafe { (*p_sel).p_prior }).is_null() {
            p_sel = unsafe { (*p_sel).p_prior };
        }
        sqlite3_columns_from_expr_list(p_parse, unsafe { (*p_sel).p_e_list },
            unsafe { &mut (*p_tab).n_col }, unsafe { &mut (*p_tab).a_col });
        unsafe { (*p_tab).i_p_key = -1 as i16 };
        unsafe { (*p_tab).e_tab_type = 2 as u8 };
        unsafe { (*p_tab).n_row_log_est = 200 as LogEst };
        { let _ = 0; };
        unsafe { (*p_tab).tab_flags |= (16384 | 512) as u32 };
        return if unsafe { (*p_parse).n_err } != 0 { 1 } else { 0 };
    }
}
extern "C" fn search_with(p_with_1: *mut With, p_item_1: &SrcItem,
    pp_context_1: &mut *mut With) -> *mut Cte {
    let z_name: *const i8 = (*p_item_1).z_name as *const i8;
    let mut p: *mut With = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    {
        p = p_with_1;
        '__b20: loop {
            if !(!(p).is_null()) { break '__b20; }
            '__c20: loop {
                let mut i: i32 = 0;
                {
                    i = 0;
                    '__b21: loop {
                        if !(i < unsafe { (*p).n_cte }) { break '__b21; }
                        '__c21: loop {
                            if unsafe {
                                        sqlite3_str_i_cmp(z_name,
                                            unsafe {
                                                    (*(unsafe { (*p).a.as_ptr() } as
                                                                    *mut Cte).offset(i as isize)).z_name
                                                } as *const i8)
                                    } == 0 {
                                *pp_context_1 = p;
                                return unsafe {
                                        &mut *(unsafe { (*p).a.as_ptr() } as
                                                        *mut Cte).offset(i as isize)
                                    };
                            }
                            break '__c21;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if unsafe { (*p).b_view } != 0 { break '__b20; }
                break '__c20;
            }
            p = unsafe { (*p).p_outer };
        }
    }
    return core::ptr::null_mut();
}
extern "C" fn cannot_be_function(p_parse_1: *mut Parse, p_from_1: &SrcItem)
    -> i32 {
    if (*p_from_1).fg.is_tab_func() != 0 {
        unsafe {
            sqlite3_error_msg(p_parse_1,
                c"\'%s\' is not a function".as_ptr() as *mut i8 as *const i8,
                (*p_from_1).z_name)
        };
        return 1;
    }
    return 0;
}
extern "C" fn resolve_from_term_to_cte(p_parse_1: *mut Parse,
    p_walker_1: *mut Walker, p_from_1: *mut SrcItem) -> i32 {
    unsafe {
        let mut p_cte: *mut Cte = core::ptr::null_mut();
        let mut p_with: *mut With = core::ptr::null_mut();
        { let _ = 0; };
        if unsafe { (*p_parse_1).p_with } == core::ptr::null_mut() {
            return 0;
        }
        if unsafe { (*p_parse_1).n_err } != 0 { return 0; }
        { let _ = 0; };
        if unsafe { (*p_from_1).fg.fixed_schema() } as i32 == 0 &&
                unsafe { (*p_from_1).u4.z_database } != core::ptr::null_mut()
            {
            return 0;
        }
        if unsafe { (*p_from_1).fg.not_cte() } != 0 { return 0; }
        p_cte =
            search_with(unsafe { (*p_parse_1).p_with }, unsafe { &*p_from_1 },
                &mut p_with);
        if !(p_cte).is_null() {
            let db: *mut sqlite3 = unsafe { (*p_parse_1).db };
            let mut p_tab: *mut Table = core::ptr::null_mut();
            let mut p_e_list: *mut ExprList = core::ptr::null_mut();
            let mut p_sel: *mut Select = core::ptr::null_mut();
            let mut p_left: *const Select = core::ptr::null();
            let mut p_rec_term: *mut Select = core::ptr::null_mut();
            let mut b_may_recursive: i32 = 0;
            let mut p_saved_with: *mut With = core::ptr::null_mut();
            let mut i_rec_tab: i32 = -1;
            let mut p_cte_use: *mut CteUse = core::ptr::null_mut();
            if !(unsafe { (*p_cte).z_cte_err }).is_null() {
                unsafe {
                    sqlite3_error_msg(p_parse_1, unsafe { (*p_cte).z_cte_err },
                        unsafe { (*p_cte).z_name })
                };
                return 2;
            }
            if cannot_be_function(p_parse_1, unsafe { &*p_from_1 }) != 0 {
                return 2;
            }
            { let _ = 0; };
            p_tab =
                unsafe {
                        sqlite3_db_malloc_zero(db,
                            core::mem::size_of::<Table>() as u64)
                    } as *mut Table;
            if p_tab == core::ptr::null_mut() { return 2; }
            p_cte_use = unsafe { (*p_cte).p_use };
            if p_cte_use == core::ptr::null_mut() {
                unsafe {
                    (*p_cte).p_use =
                        {
                            p_cte_use =
                                unsafe {
                                        sqlite3_db_malloc_zero(db,
                                            core::mem::size_of::<CteUse>() as u64)
                                    } as *mut CteUse;
                            p_cte_use
                        }
                };
                if p_cte_use == core::ptr::null_mut() ||
                        unsafe {
                                sqlite3_parser_add_cleanup(p_parse_1, Some(sqlite3_db_free),
                                    p_cte_use as *mut ())
                            } == core::ptr::null_mut() {
                    unsafe { sqlite3_db_free(db, p_tab as *mut ()) };
                    return 2;
                }
                unsafe { (*p_cte_use).e_m10d = unsafe { (*p_cte).e_m10d } };
            }
            unsafe { (*p_from_1).p_s_tab = p_tab };
            unsafe { (*p_tab).n_tab_ref = 1 as u32 };
            unsafe {
                (*p_tab).z_name =
                    unsafe {
                        sqlite3_db_str_dup(db,
                            unsafe { (*p_cte).z_name } as *const i8)
                    }
            };
            unsafe { (*p_tab).i_p_key = -1 as i16 };
            unsafe { (*p_tab).n_row_log_est = 200 as LogEst };
            { let _ = 0; };
            unsafe { (*p_tab).tab_flags |= (16384 | 512) as u32 };
            unsafe {
                sqlite3_src_item_attach_subquery(p_parse_1, p_from_1,
                    unsafe { (*p_cte).p_select }, 1)
            };
            if unsafe { (*db).malloc_failed } != 0 { return 2; }
            { let _ = 0; };
            p_sel = unsafe { (*unsafe { (*p_from_1).u4.p_subq }).p_select };
            { let _ = 0; };
            unsafe { (*p_sel).sel_flags |= 67108864 as u32 };
            if unsafe { (*p_from_1).fg.is_indexed_by() } != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse_1,
                        c"no such index: \"%s\"".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p_from_1).u1.z_indexed_by })
                };
                return 2;
            }
            { let _ = 0; };
            unsafe { (*p_from_1).fg.set_is_cte(1 as u32 as u32) };
            unsafe { (*p_from_1).u2.p_cte_use = p_cte_use };
            {
                let __p = unsafe { &mut (*p_cte_use).n_use };
                let __t = *__p;
                *__p += 1;
                __t
            };
            p_rec_term = p_sel;
            b_may_recursive =
                (unsafe { (*p_sel).op } as i32 == 136 ||
                        unsafe { (*p_sel).op } as i32 == 135) as i32;
            while b_may_recursive != 0 &&
                    unsafe { (*p_rec_term).op } as i32 ==
                        unsafe { (*p_sel).op } as i32 {
                let mut i: i32 = 0;
                let p_src: *mut SrcList = unsafe { (*p_rec_term).p_src };
                { let _ = 0; };
                {
                    i = 0;
                    '__b23: loop {
                        if !(i < unsafe { (*p_src).n_src }) { break '__b23; }
                        '__c23: loop {
                            let p_item: *mut SrcItem =
                                unsafe {
                                    &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(i as isize)
                                };
                            if unsafe { (*p_item).z_name } != core::ptr::null_mut() &&
                                                (unsafe { (*p_item).fg.had_schema() } == 0) as i32 != 0 &&
                                            (unsafe { (*p_item).fg.is_subquery() } == 0) as i32 != 0 &&
                                        (unsafe { (*p_item).fg.fixed_schema() } != 0 ||
                                            unsafe { (*p_item).u4.z_database } == core::ptr::null_mut())
                                    &&
                                    0 ==
                                        unsafe {
                                            sqlite3_str_i_cmp(unsafe { (*p_item).z_name } as *const i8,
                                                unsafe { (*p_cte).z_name } as *const i8)
                                        } {
                                unsafe { (*p_item).p_s_tab = p_tab };
                                {
                                    let __p = unsafe { &mut (*p_tab).n_tab_ref };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                unsafe { (*p_item).fg.set_is_recursive(1 as u32 as u32) };
                                if unsafe { (*p_rec_term).sel_flags } & 8192 as u32 != 0 {
                                    unsafe {
                                        sqlite3_error_msg(p_parse_1,
                                            c"multiple references to recursive table: %s".as_ptr() as
                                                    *mut i8 as *const i8, unsafe { (*p_cte).z_name })
                                    };
                                    return 2;
                                }
                                unsafe { (*p_rec_term).sel_flags |= 8192 as u32 };
                                if i_rec_tab < 0 {
                                    i_rec_tab =
                                        {
                                            let __p = unsafe { &mut (*p_parse_1).n_tab };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                }
                                unsafe { (*p_item).i_cursor = i_rec_tab };
                            }
                            break '__c23;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if unsafe { (*p_rec_term).sel_flags } & 8192 as u32 ==
                        0 as u32 {
                    break;
                }
                p_rec_term = unsafe { (*p_rec_term).p_prior };
            }
            unsafe {
                (*p_cte).z_cte_err =
                    c"circular reference: %s".as_ptr() as *mut i8 as *const i8
            };
            p_saved_with = unsafe { (*p_parse_1).p_with };
            unsafe { (*p_parse_1).p_with = p_with };
            if unsafe { (*p_sel).sel_flags } & 8192 as u32 != 0 {
                let mut rc: i32 = 0;
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                unsafe { (*p_rec_term).p_with = unsafe { (*p_sel).p_with } };
                rc = unsafe { sqlite3_walk_select(p_walker_1, p_rec_term) };
                unsafe { (*p_rec_term).p_with = core::ptr::null_mut() };
                if rc != 0 {
                    unsafe { (*p_parse_1).p_with = p_saved_with };
                    return 2;
                }
            } else {
                if unsafe { sqlite3_walk_select(p_walker_1, p_sel) } != 0 {
                    unsafe { (*p_parse_1).p_with = p_saved_with };
                    return 2;
                }
            }
            unsafe { (*p_parse_1).p_with = p_with };
            {
                p_left = p_sel;
                '__b24: loop {
                    if !(!(unsafe { (*p_left).p_prior }).is_null()) {
                        break '__b24;
                    }
                    '__c24: loop { break '__c24; }
                    p_left = unsafe { (*p_left).p_prior };
                }
            }
            p_e_list = unsafe { (*p_left).p_e_list };
            if !(unsafe { (*p_cte).p_cols }).is_null() {
                if !(p_e_list).is_null() &&
                        unsafe { (*p_e_list).n_expr } !=
                            unsafe { (*unsafe { (*p_cte).p_cols }).n_expr } {
                    unsafe {
                        sqlite3_error_msg(p_parse_1,
                            c"table %s has %d values for %d columns".as_ptr() as *mut i8
                                as *const i8, unsafe { (*p_cte).z_name },
                            unsafe { (*p_e_list).n_expr },
                            unsafe { (*unsafe { (*p_cte).p_cols }).n_expr })
                    };
                    unsafe { (*p_parse_1).p_with = p_saved_with };
                    return 2;
                }
                p_e_list = unsafe { (*p_cte).p_cols };
            }
            sqlite3_columns_from_expr_list(p_parse_1, p_e_list,
                unsafe { &mut (*p_tab).n_col },
                unsafe { &mut (*p_tab).a_col });
            if b_may_recursive != 0 {
                if unsafe { (*p_sel).sel_flags } & 8192 as u32 != 0 {
                    unsafe {
                        (*p_cte).z_cte_err =
                            c"multiple recursive references: %s".as_ptr() as *mut i8 as
                                *const i8
                    };
                } else {
                    unsafe {
                        (*p_cte).z_cte_err =
                            c"recursive reference in a subquery: %s".as_ptr() as *mut i8
                                as *const i8
                    };
                }
                unsafe { sqlite3_walk_select(p_walker_1, p_sel) };
            }
            unsafe { (*p_cte).z_cte_err = core::ptr::null() };
            unsafe { (*p_parse_1).p_with = p_saved_with };
            return 1;
        }
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_indexed_by_lookup(p_parse: *mut Parse,
    p_from: &mut SrcItem) -> i32 {
    unsafe {
        let p_tab: *const Table = (*p_from).p_s_tab as *const Table;
        let z_indexed_by: *mut i8 = (*p_from).u1.z_indexed_by;
        let mut p_idx: *mut Index = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        {
            p_idx = unsafe { (*p_tab).p_index };
            '__b25: loop {
                if !(!(p_idx).is_null() &&
                                unsafe {
                                        sqlite3_str_i_cmp(unsafe { (*p_idx).z_name } as *const i8,
                                            z_indexed_by as *const i8)
                                    } != 0) {
                    break '__b25;
                }
                '__c25: loop { break '__c25; }
                p_idx = unsafe { (*p_idx).p_next };
            }
        }
        if (p_idx).is_null() as i32 != 0 {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"no such index: %s".as_ptr() as *mut i8 as *const i8,
                    z_indexed_by, 0)
            };
            unsafe { (*p_parse).set_check_schema(1 as bft as u32) };
            return 1;
        }
        { let _ = 0; };
        (*p_from).u2.p_ib_index = p_idx;
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_index(p_tab: &Table, z_col: *const i8)
    -> i32 {
    let mut i: i32 = 0;
    let mut h: u8 = 0 as u8;
    let mut a_col: *const Column = core::ptr::null();
    let mut n_col: i32 = 0;
    h = unsafe { sqlite3_str_i_hash(z_col) };
    a_col = (*p_tab).a_col as *const Column;
    n_col = (*p_tab).n_col as i32;
    i =
        (*p_tab).a_hx[(h as u64 % core::mem::size_of::<[u8; 16]>() as u64) as
                    usize] as i32;
    { let _ = 0; };
    if unsafe { (*a_col.offset(i as isize)).h_name } as i32 == h as i32 &&
            unsafe {
                    sqlite3_str_i_cmp(unsafe {
                                (*a_col.offset(i as isize)).z_cn_name
                            } as *const i8, z_col)
                } == 0 {
        return i;
    }
    i = 0;
    loop {
        if unsafe { (*a_col.offset(i as isize)).h_name } as i32 == h as i32 &&
                unsafe {
                        sqlite3_str_i_cmp(unsafe {
                                    (*a_col.offset(i as isize)).z_cn_name
                                } as *const i8, z_col)
                    } == 0 {
            return i;
        }
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        if i >= n_col { break; }
    }
    return -1;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_item_column_used(p_item: &SrcItem, i_col: i32)
    -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if (*p_item).fg.is_nested_from() != 0 {
            let mut p_results: *mut ExprList = core::ptr::null_mut();
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            p_results =
                unsafe {
                    (*unsafe { (*(*p_item).u4.p_subq).p_select }).p_e_list
                };
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                (*(unsafe { (*p_results).a.as_ptr() } as
                                    *mut ExprList_item).offset(i_col as
                                    isize)).fg.set_b_used(1 as u32 as u32)
            };
        }
    }
}
extern "C" fn table_and_column_index(p_src_1: &mut SrcList, i_start_1: i32,
    i_end_1: i32, z_col_1: *const i8, pi_tab_1: *mut i32, pi_col_1: &mut i32,
    b_ignore_hidden_1: i32) -> i32 {
    let mut i: i32 = 0;
    let mut i_col: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    {
        i = i_start_1;
        '__b27: loop {
            if !(i <= i_end_1) { break '__b27; }
            '__c27: loop {
                i_col =
                    sqlite3_column_index(unsafe {
                            &*unsafe {
                                        (*((*p_src_1).a.as_ptr() as
                                                        *mut SrcItem).offset(i as isize)).p_s_tab
                                    }
                        }, z_col_1);
                if i_col >= 0 &&
                        (b_ignore_hidden_1 == 0 ||
                            (unsafe {
                                                    (*unsafe {
                                                                    &mut *unsafe {
                                                                                (*unsafe {
                                                                                                    (*((*p_src_1).a.as_ptr() as
                                                                                                                    *mut SrcItem).offset(i as isize)).p_s_tab
                                                                                                }).a_col.offset(i_col as isize)
                                                                            }
                                                                }).col_flags
                                                } as i32 & 2 != 0) as i32 == 0) {
                    if !(pi_tab_1).is_null() {
                        sqlite3_src_item_column_used(unsafe {
                                &*((*p_src_1).a.as_ptr() as *mut SrcItem).offset(i as isize)
                            }, i_col);
                        unsafe { *pi_tab_1 = i };
                        *pi_col_1 = i_col;
                    }
                    return 1;
                }
                break '__c27;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_set_join_expr(mut p: *mut Expr, i_table: i32,
    join_flag: u32) -> () {
    unsafe {
        { let _ = 0; };
        while !(p).is_null() {
            unsafe { (*p).flags |= join_flag as u32 };
            { let _ = 0; };
            unsafe { (*p).w.i_join = i_table };
            if unsafe { (*p).flags } & 4096 as u32 == 0 as u32 {
                if !(unsafe { (*p).x.p_list }).is_null() {
                    let mut i: i32 = 0;
                    {
                        i = 0;
                        '__b29: loop {
                            if !(i < unsafe { (*unsafe { (*p).x.p_list }).n_expr }) {
                                break '__b29;
                            }
                            '__c29: loop {
                                sqlite3_set_join_expr(unsafe {
                                        (*(unsafe { (*unsafe { (*p).x.p_list }).a.as_ptr() } as
                                                        *mut ExprList_item).offset(i as isize)).p_expr
                                    }, i_table, join_flag);
                                break '__c29;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
            }
            sqlite3_set_join_expr(unsafe { (*p).p_left }, i_table, join_flag);
            p = unsafe { (*p).p_right };
        }
    }
}
extern "C" fn sqlite3_process_join(p_parse_1: *mut Parse, p: &mut Select)
    -> i32 {
    unsafe {
        unsafe {
            let mut p_src: *mut SrcList = core::ptr::null_mut();
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut p_left: *mut SrcItem = core::ptr::null_mut();
            let mut p_right: *mut SrcItem = core::ptr::null_mut();
            p_src = (*p).p_src;
            p_left =
                unsafe {
                    &mut *(unsafe { (*p_src).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)
                };
            p_right = unsafe { p_left.offset(1 as isize) };
            {
                i = 0;
                '__b30: loop {
                    if !(i < unsafe { (*p_src).n_src } - 1) { break '__b30; }
                    '__c30: loop {
                        let p_right_tab: *mut Table = unsafe { (*p_right).p_s_tab };
                        let mut join_type: u32 = 0 as u32;
                        if unsafe { (*p_left).p_s_tab } == core::ptr::null_mut() ||
                                p_right_tab == core::ptr::null_mut() {
                            break '__c30;
                        }
                        join_type =
                            if unsafe { (*p_right).fg.jointype } as i32 & 32 != 0 {
                                    1
                                } else { 2 } as u32;
                        if unsafe { (*p_right).fg.jointype } as i32 & 4 != 0 {
                            let mut p_using: *mut IdList = core::ptr::null_mut();
                            if unsafe { (*p_right).fg.is_using() } != 0 ||
                                    !(unsafe { (*p_right).u3.p_on }).is_null() {
                                unsafe {
                                    sqlite3_error_msg(p_parse_1,
                                        c"a NATURAL join may not have an ON or USING clause".as_ptr()
                                                as *mut i8 as *const i8, 0)
                                };
                                return 1;
                            }
                            {
                                j = 0;
                                '__b31: loop {
                                    if !(j < unsafe { (*p_right_tab).n_col } as i32) {
                                        break '__b31;
                                    }
                                    '__c31: loop {
                                        let mut z_name: *const i8 = core::ptr::null();
                                        if unsafe {
                                                            (*unsafe {
                                                                            &mut *unsafe { (*p_right_tab).a_col.offset(j as isize) }
                                                                        }).col_flags
                                                        } as i32 & 2 != 0 {
                                            break '__c31;
                                        }
                                        z_name =
                                            unsafe {
                                                (*unsafe {
                                                            (*p_right_tab).a_col.offset(j as isize)
                                                        }).z_cn_name
                                            };
                                        if table_and_column_index(unsafe { &mut *p_src }, 0, i,
                                                    z_name as *const i8, core::ptr::null_mut(), &mut 0, 1) != 0
                                            {
                                            p_using =
                                                unsafe {
                                                    sqlite3_id_list_append(p_parse_1, p_using,
                                                        core::ptr::null_mut())
                                                };
                                            if !(p_using).is_null() {
                                                { let _ = 0; };
                                                { let _ = 0; };
                                                unsafe {
                                                    (*(unsafe { (*p_using).a.as_ptr() } as
                                                                        *mut IdList_item).offset((unsafe { (*p_using).n_id } - 1) as
                                                                        isize)).z_name =
                                                        unsafe {
                                                            sqlite3_db_str_dup(unsafe { (*p_parse_1).db },
                                                                z_name as *const i8)
                                                        }
                                                };
                                            }
                                        }
                                        break '__c31;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if !(p_using).is_null() {
                                unsafe { (*p_right).fg.set_is_using(1 as u32 as u32) };
                                unsafe {
                                    (*p_right).fg.set_is_synth_using(1 as u32 as u32)
                                };
                                unsafe { (*p_right).u3.p_using = p_using };
                            }
                            if unsafe { (*p_parse_1).n_err } != 0 { return 1; }
                        }
                        if unsafe { (*p_right).fg.is_using() } != 0 {
                            let p_list: *const IdList =
                                unsafe { (*p_right).u3.p_using } as *const IdList;
                            let db: *mut sqlite3 = unsafe { (*p_parse_1).db };
                            { let _ = 0; };
                            {
                                j = 0;
                                '__b32: loop {
                                    if !(j < unsafe { (*p_list).n_id }) { break '__b32; }
                                    '__c32: loop {
                                        let mut z_name_1: *mut i8 = core::ptr::null_mut();
                                        let mut i_left: i32 = 0;
                                        let mut i_left_col: i32 = 0;
                                        let mut i_right_col: i32 = 0;
                                        let mut p_e1: *mut Expr = core::ptr::null_mut();
                                        let mut p_e2: *mut Expr = core::ptr::null_mut();
                                        let mut p_eq: *mut Expr = core::ptr::null_mut();
                                        z_name_1 =
                                            unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut IdList_item).offset(j as isize)).z_name
                                            };
                                        i_right_col =
                                            sqlite3_column_index(unsafe { &*p_right_tab },
                                                z_name_1 as *const i8);
                                        if i_right_col < 0 ||
                                                table_and_column_index(unsafe { &mut *p_src }, 0, i,
                                                        z_name_1 as *const i8, &mut i_left, &mut i_left_col,
                                                        unsafe { (*p_right).fg.is_synth_using() } as i32) == 0 {
                                            unsafe {
                                                sqlite3_error_msg(p_parse_1,
                                                    c"cannot join using column %s - column not present in both tables".as_ptr()
                                                            as *mut i8 as *const i8, z_name_1)
                                            };
                                            return 1;
                                        }
                                        p_e1 =
                                            unsafe {
                                                sqlite3_create_column_expr(db, p_src, i_left, i_left_col)
                                            };
                                        sqlite3_src_item_column_used(unsafe {
                                                &*(unsafe { (*p_src).a.as_ptr() } as
                                                                *mut SrcItem).offset(i_left as isize)
                                            }, i_left_col);
                                        if unsafe {
                                                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                    *mut SrcItem).offset(0 as isize)).fg.jointype
                                                            } as i32 & 64 != 0 && unsafe { (*p_parse_1).n_err } == 0 {
                                            let mut p_func_args: *mut ExprList = core::ptr::null_mut();
                                            { let _ = 0; };
                                            unsafe { (*p_e1).flags |= 2097152 as u32 };
                                            while table_and_column_index(unsafe { &mut *p_src },
                                                        i_left + 1, i, z_name_1 as *const i8, &mut i_left,
                                                        &mut i_left_col,
                                                        unsafe { (*p_right).fg.is_synth_using() } as i32) != 0 {
                                                if unsafe {
                                                                    (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                        *mut SrcItem).offset(i_left as isize)).fg.is_using()
                                                                } as i32 == 0 ||
                                                        unsafe {
                                                                sqlite3_id_list_index(unsafe {
                                                                        (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                            *mut SrcItem).offset(i_left as isize)).u3.p_using
                                                                    }, z_name_1 as *const i8)
                                                            } < 0 {
                                                    unsafe {
                                                        sqlite3_error_msg(p_parse_1,
                                                            c"ambiguous reference to %s in USING()".as_ptr() as *mut i8
                                                                as *const i8, z_name_1)
                                                    };
                                                    break;
                                                }
                                                p_func_args =
                                                    unsafe {
                                                        sqlite3_expr_list_append(p_parse_1, p_func_args, p_e1)
                                                    };
                                                p_e1 =
                                                    unsafe {
                                                        sqlite3_create_column_expr(db, p_src, i_left, i_left_col)
                                                    };
                                                sqlite3_src_item_column_used(unsafe {
                                                        &*(unsafe { (*p_src).a.as_ptr() } as
                                                                        *mut SrcItem).offset(i_left as isize)
                                                    }, i_left_col);
                                            }
                                            if !(p_func_args).is_null() {
                                                p_func_args =
                                                    unsafe {
                                                        sqlite3_expr_list_append(p_parse_1, p_func_args, p_e1)
                                                    };
                                                p_e1 =
                                                    unsafe {
                                                        sqlite3_expr_function(p_parse_1, p_func_args, &tk_coalesce,
                                                            0)
                                                    };
                                                if !(p_e1).is_null() {
                                                    unsafe { (*p_e1).aff_expr = 88 as i8 };
                                                }
                                            }
                                        } else if unsafe {
                                                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                    *mut SrcItem).offset((i + 1) as isize)).fg.jointype
                                                            } as i32 & 8 != 0 && unsafe { (*p_parse_1).n_err } == 0 {
                                            { let _ = 0; };
                                            unsafe { (*p_e1).flags |= 2097152 as u32 };
                                        }
                                        p_e2 =
                                            unsafe {
                                                sqlite3_create_column_expr(db, p_src, i + 1, i_right_col)
                                            };
                                        sqlite3_src_item_column_used(unsafe { &*p_right },
                                            i_right_col);
                                        p_eq = unsafe { sqlite3_p_expr(p_parse_1, 54, p_e1, p_e2) };
                                        { let _ = 0; };
                                        if !(p_eq).is_null() {
                                            unsafe { (*p_eq).flags |= join_type as u32 };
                                            { let _ = 0; };
                                            unsafe { (*p_eq).w.i_join = unsafe { (*p_e2).i_table } };
                                        }
                                        (*p).p_where =
                                            unsafe { sqlite3_expr_and(p_parse_1, (*p).p_where, p_eq) };
                                        break '__c32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        } else if !(unsafe { (*p_right).u3.p_on }).is_null() {
                            sqlite3_set_join_expr(unsafe { (*p_right).u3.p_on },
                                unsafe { (*p_right).i_cursor }, join_type);
                            (*p).p_where =
                                unsafe {
                                    sqlite3_expr_and(p_parse_1, (*p).p_where,
                                        unsafe { (*p_right).u3.p_on })
                                };
                            unsafe { (*p_right).u3.p_on = core::ptr::null_mut() };
                            unsafe { (*p_right).fg.set_is_on(1 as u32 as u32) };
                            (*p).sel_flags |= 1073741824 as u32;
                        }
                        if unsafe { (*p_right).fg.is_tab_func() } != 0 &&
                                    join_type == 1 as u32 &&
                                !(unsafe { (*p_right).u1.p_func_arg }).is_null() {
                            (*p).sel_flags |= 1073741824 as u32;
                        }
                        break '__c30;
                    }
                    {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut p_right;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        {
                            let __p = &mut p_left;
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
}
extern "C" fn in_any_using_clause(z_name_1: *const i8,
    mut p_base_1: *const SrcItem, mut n_1: i32) -> i32 {
    unsafe {
        while n_1 > 0 {
            { let __p = &mut n_1; let __t = *__p; *__p -= 1; __t };
            {
                let __p = &mut p_base_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            if unsafe { (*p_base_1).fg.is_using() } as i32 == 0 { continue; }
            if unsafe { (*p_base_1).u3.p_using } == core::ptr::null_mut() {
                continue;
            }
            if unsafe {
                        sqlite3_id_list_index(unsafe { (*p_base_1).u3.p_using },
                            z_name_1)
                    } >= 0 {
                return 1;
            }
        }
        return 0;
    }
}
extern "C" fn select_expander(p_walker_1: *mut Walker, p: *mut Select)
    -> i32 {
    unsafe {
        let p_parse: *mut Parse = unsafe { (*p_walker_1).p_parse };
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut rc: i32 = 0;
        let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let mut p_from: *mut SrcItem = core::ptr::null_mut();
        let db: *mut sqlite3 = unsafe { (*p_parse).db };
        let mut p_e: *const Expr = core::ptr::null();
        let mut p_right: *mut Expr = core::ptr::null_mut();
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        let sel_flags: u16 = unsafe { (*p).sel_flags } as u16;
        let mut elist_flags: u32 = 0 as u32;
        unsafe { (*p).sel_flags |= 64 as u32 };
        if unsafe { (*db).malloc_failed } != 0 { return 2; }
        { let _ = 0; };
        if sel_flags as i32 & 64 != 0 { return 1; }
        if unsafe { (*p_walker_1).e_code } != 0 {
            unsafe {
                (*p).sel_id =
                    {
                            let __p = unsafe { &mut (*p_parse).n_select };
                            *__p += 1;
                            *__p
                        } as u32
            };
        }
        p_tab_list = unsafe { (*p).p_src };
        p_e_list = unsafe { (*p).p_e_list };
        if !(unsafe { (*p_parse).p_with }).is_null() &&
                unsafe { (*p).sel_flags } & 2097152 as u32 != 0 {
            if unsafe { (*p).p_with } == core::ptr::null_mut() {
                unsafe {
                    (*p).p_with =
                        unsafe {
                                sqlite3_db_malloc_zero(db,
                                    core::mem::offset_of!(With, a) as u64 +
                                        1 as u64 * core::mem::size_of::<Cte>() as u64)
                            } as *mut With
                };
                if unsafe { (*p).p_with } == core::ptr::null_mut() {
                    return 2;
                }
            }
            unsafe { (*unsafe { (*p).p_with }).b_view = 1 };
        }
        sqlite3_with_push(p_parse, unsafe { (*p).p_with }, 0 as u8);
        unsafe { sqlite3_src_list_assign_cursors(p_parse, p_tab_list) };
        {
            {
                i = 0;
                p_from = unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem
            };
            '__b35: loop {
                if !(i < unsafe { (*p_tab_list).n_src }) { break '__b35; }
                '__c35: loop {
                    let mut p_tab: *mut Table = core::ptr::null_mut();
                    { let _ = 0; };
                    if !(unsafe { (*p_from).p_s_tab }).is_null() {
                        break '__c35;
                    }
                    { let _ = 0; };
                    if unsafe { (*p_from).z_name } == core::ptr::null_mut() {
                        let mut p_sel: *mut Select = core::ptr::null_mut();
                        { let _ = 0; };
                        p_sel =
                            unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select };
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { sqlite3_walk_select(p_walker_1, p_sel) } != 0 {
                            return 2;
                        }
                        if sqlite3_expand_subquery(p_parse, p_from) != 0 {
                            return 2;
                        }
                    } else if {
                                rc = resolve_from_term_to_cte(p_parse, p_walker_1, p_from);
                                rc
                            } != 0 {
                        if rc > 1 { return 2; }
                        p_tab = unsafe { (*p_from).p_s_tab };
                        { let _ = 0; };
                    } else {
                        { let _ = 0; };
                        unsafe {
                            (*p_from).p_s_tab =
                                {
                                    p_tab =
                                        unsafe {
                                            sqlite3_locate_table_item(p_parse, 0 as u32, p_from)
                                        };
                                    p_tab
                                }
                        };
                        if p_tab == core::ptr::null_mut() { return 2; }
                        if unsafe { (*p_tab).n_tab_ref } >= 65535 as u32 {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"too many references to \"%s\": max 65535".as_ptr() as
                                            *mut i8 as *const i8, unsafe { (*p_tab).z_name })
                            };
                            unsafe { (*p_from).p_s_tab = core::ptr::null_mut() };
                            return 2;
                        }
                        {
                            let __p = unsafe { &mut (*p_tab).n_tab_ref };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        if !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0
                                && cannot_be_function(p_parse, unsafe { &*p_from }) != 0 {
                            return 2;
                        }
                        if !(unsafe { (*p_tab).e_tab_type } as i32 == 0) as i32 != 0
                            {
                            let mut n_col: i16 = 0 as i16;
                            let e_code_orig: u8 = unsafe { (*p_walker_1).e_code } as u8;
                            if unsafe { sqlite3_view_get_column_names(p_parse, p_tab) }
                                    != 0 {
                                return 2;
                            }
                            { let _ = 0; };
                            if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                                if unsafe { (*db).flags } & 2147483648u32 as u64 == 0 as u64
                                        &&
                                        unsafe { (*p_tab).p_schema } !=
                                            unsafe {
                                                (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                                            } {
                                    unsafe {
                                        sqlite3_error_msg(p_parse,
                                            c"access to view \"%s\" prohibited".as_ptr() as *mut i8 as
                                                *const i8, unsafe { (*p_tab).z_name })
                                    };
                                }
                                unsafe {
                                    sqlite3_src_item_attach_subquery(p_parse, p_from,
                                        unsafe { (*p_tab).u.view.p_select }, 1)
                                };
                            } else if unsafe { (*p_tab).e_tab_type } as i32 == 1 &&
                                            (unsafe { (*p_from).fg.from_ddl() } != 0 ||
                                                unsafe { (*p_parse).prep_flags } as i32 & 32 != 0) &&
                                        unsafe { (*p_tab).u.vtab.p } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_tab).u.vtab.p }).e_vtab_risk } as
                                            i32 >
                                        (unsafe { (*db).flags } & 128 as u64 != 0 as u64) as i32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"unsafe use of virtual table \"%s\"".as_ptr() as *mut i8 as
                                            *const i8, unsafe { (*p_tab).z_name })
                                };
                            }
                            { let _ = 0; };
                            n_col = unsafe { (*p_tab).n_col };
                            unsafe { (*p_tab).n_col = -1 as i16 };
                            unsafe { (*p_walker_1).e_code = 1 as u16 };
                            if unsafe { (*p_from).fg.is_subquery() } != 0 {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select })
                                };
                            }
                            unsafe { (*p_walker_1).e_code = e_code_orig as u16 };
                            unsafe { (*p_tab).n_col = n_col };
                        }
                    }
                    if unsafe { (*p_from).fg.is_indexed_by() } != 0 &&
                            sqlite3_indexed_by_lookup(p_parse, unsafe { &mut *p_from })
                                != 0 {
                        return 2;
                    }
                    break '__c35;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_from;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        { let _ = 0; };
        if unsafe { (*p_parse).n_err } != 0 ||
                sqlite3_process_join(p_parse, unsafe { &mut *p }) != 0 {
            return 2;
        }
        {
            k = 0;
            '__b36: loop {
                if !(k < unsafe { (*p_e_list).n_expr }) { break '__b36; }
                '__c36: loop {
                    p_e =
                        unsafe {
                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                            *mut ExprList_item).offset(k as isize)).p_expr
                        };
                    if unsafe { (*p_e).op } as i32 == 180 { break '__b36; }
                    { let _ = 0; };
                    { let _ = 0; };
                    if unsafe { (*p_e).op } as i32 == 142 &&
                            unsafe { (*unsafe { (*p_e).p_right }).op } as i32 == 180 {
                        break '__b36;
                    }
                    elist_flags |= unsafe { (*p_e).flags };
                    break '__c36;
                }
                { let __p = &mut k; let __t = *__p; *__p += 1; __t };
            }
        }
        if k < unsafe { (*p_e_list).n_expr } {
            let a: *mut ExprList_item =
                unsafe { (*p_e_list).a.as_ptr() } as *mut ExprList_item;
            let mut p_new: *mut ExprList = core::ptr::null_mut();
            let flags: i32 =
                unsafe { (*unsafe { (*p_parse).db }).flags } as i32;
            let long_names: i32 = (flags & 4 != 0 && flags & 64 == 0) as i32;
            {
                k = 0;
                '__b37: loop {
                    if !(k < unsafe { (*p_e_list).n_expr }) { break '__b37; }
                    '__c37: loop {
                        p_e = unsafe { (*a.offset(k as isize)).p_expr };
                        elist_flags |= unsafe { (*p_e).flags };
                        p_right = unsafe { (*p_e).p_right };
                        { let _ = 0; };
                        if unsafe { (*p_e).op } as i32 != 180 &&
                                (unsafe { (*p_e).op } as i32 != 142 ||
                                    unsafe { (*p_right).op } as i32 != 180) {
                            p_new =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse, p_new,
                                        unsafe { (*a.offset(k as isize)).p_expr })
                                };
                            if !(p_new).is_null() {
                                unsafe {
                                    (*(unsafe { (*p_new).a.as_ptr() } as
                                                        *mut ExprList_item).offset((unsafe { (*p_new).n_expr } - 1)
                                                        as isize)).z_e_name =
                                        unsafe { (*a.offset(k as isize)).z_e_name }
                                };
                                unsafe {
                                    (*(unsafe { (*p_new).a.as_ptr() } as
                                                        *mut ExprList_item).offset((unsafe { (*p_new).n_expr } - 1)
                                                        as
                                                        isize)).fg.set_e_e_name(unsafe {
                                                (*a.offset(k as isize)).fg.e_e_name()
                                            } as u32)
                                };
                                unsafe {
                                    (*a.offset(k as isize)).z_e_name = core::ptr::null_mut()
                                };
                            }
                            unsafe {
                                (*a.offset(k as isize)).p_expr = core::ptr::null_mut()
                            };
                        } else {
                            let mut table_seen: i32 = 0;
                            let mut z_t_name: *mut i8 = core::ptr::null_mut();
                            let mut i_err_ofst: i32 = 0;
                            if unsafe { (*p_e).op } as i32 == 142 {
                                { let _ = 0; };
                                { let _ = 0; };
                                { let _ = 0; };
                                z_t_name = unsafe { (*unsafe { (*p_e).p_left }).u.z_token };
                                { let _ = 0; };
                                i_err_ofst =
                                    unsafe { (*unsafe { (*p_e).p_right }).w.i_ofst };
                            } else {
                                { let _ = 0; };
                                i_err_ofst = unsafe { (*p_e).w.i_ofst };
                            }
                            {
                                {
                                    i = 0;
                                    p_from = unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem
                                };
                                '__b38: loop {
                                    if !(i < unsafe { (*p_tab_list).n_src }) { break '__b38; }
                                    '__c38: loop {
                                        let mut n_add: i32 = 0;
                                        let p_tab_1: *mut Table = unsafe { (*p_from).p_s_tab };
                                        let mut p_nested_from: *mut ExprList =
                                            core::ptr::null_mut();
                                        let mut z_tab_name: *mut i8 = core::ptr::null_mut();
                                        let mut z_schema_name: *const i8 = core::ptr::null();
                                        let mut i_db: i32 = 0;
                                        let mut p_using: *mut IdList = core::ptr::null_mut();
                                        if { z_tab_name = unsafe { (*p_from).z_alias }; z_tab_name }
                                                == core::ptr::null_mut() {
                                            z_tab_name = unsafe { (*p_tab_1).z_name };
                                        }
                                        if unsafe { (*db).malloc_failed } != 0 { break '__b38; }
                                        { let _ = 0; };
                                        if unsafe { (*p_from).fg.is_nested_from() } != 0 {
                                            { let _ = 0; };
                                            { let _ = 0; };
                                            p_nested_from =
                                                unsafe {
                                                    (*unsafe {
                                                                    (*unsafe { (*p_from).u4.p_subq }).p_select
                                                                }).p_e_list
                                                };
                                            { let _ = 0; };
                                            { let _ = 0; };
                                            { let _ = 0; };
                                        } else {
                                            if !(z_t_name).is_null() &&
                                                    unsafe {
                                                            sqlite3_str_i_cmp(z_t_name as *const i8,
                                                                z_tab_name as *const i8)
                                                        } != 0 {
                                                break '__c38;
                                            }
                                            p_nested_from = core::ptr::null_mut();
                                            i_db =
                                                unsafe {
                                                    sqlite3_schema_to_index(db, unsafe { (*p_tab_1).p_schema })
                                                };
                                            z_schema_name =
                                                if i_db >= 0 {
                                                        unsafe {
                                                            (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                                        }
                                                    } else { c"*".as_ptr() as *mut i8 } as *const i8;
                                        }
                                        if i + 1 < unsafe { (*p_tab_list).n_src } &&
                                                    unsafe { (*p_from.offset(1 as isize)).fg.is_using() } != 0
                                                && sel_flags as i32 & 2048 != 0 {
                                            let mut ii: i32 = 0;
                                            p_using =
                                                unsafe { (*p_from.offset(1 as isize)).u3.p_using };
                                            {
                                                ii = 0;
                                                '__b39: loop {
                                                    if !(ii < unsafe { (*p_using).n_id }) { break '__b39; }
                                                    '__c39: loop {
                                                        let z_u_name: *const i8 =
                                                            unsafe {
                                                                    (*(unsafe { (*p_using).a.as_ptr() } as
                                                                                    *mut IdList_item).offset(ii as isize)).z_name
                                                                } as *const i8;
                                                        p_right = unsafe { sqlite3_expr(db, 60, z_u_name) };
                                                        unsafe {
                                                            sqlite3_expr_set_error_offset(p_right, i_err_ofst)
                                                        };
                                                        p_new =
                                                            unsafe {
                                                                sqlite3_expr_list_append(p_parse, p_new, p_right)
                                                            };
                                                        if !(p_new).is_null() {
                                                            let p_x: *mut ExprList_item =
                                                                unsafe {
                                                                    &mut *(unsafe { (*p_new).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset((unsafe { (*p_new).n_expr } - 1)
                                                                                    as isize)
                                                                };
                                                            { let _ = 0; };
                                                            unsafe {
                                                                (*p_x).z_e_name =
                                                                    unsafe {
                                                                        sqlite3_m_printf(db,
                                                                            c"..%s".as_ptr() as *mut i8 as *const i8, z_u_name)
                                                                    }
                                                            };
                                                            unsafe { (*p_x).fg.set_e_e_name(2 as u32 as u32) };
                                                            unsafe { (*p_x).fg.set_b_using_term(1 as u32 as u32) };
                                                        }
                                                        break '__c39;
                                                    }
                                                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                        } else { p_using = core::ptr::null_mut(); }
                                        n_add = unsafe { (*p_tab_1).n_col } as i32;
                                        if unsafe { (*p_tab_1).tab_flags } & 512 as u32 == 0 as u32
                                                && sel_flags as i32 & 2048 != 0 {
                                            { let __p = &mut n_add; let __t = *__p; *__p += 1; __t };
                                        }
                                        {
                                            j = 0;
                                            '__b40: loop {
                                                if !(j < n_add) { break '__b40; }
                                                '__c40: loop {
                                                    let mut z_name: *const i8 = core::ptr::null();
                                                    let mut p_x_1: *mut ExprList_item = core::ptr::null_mut();
                                                    if j == unsafe { (*p_tab_1).n_col } as i32 {
                                                        z_name = unsafe { sqlite3_rowid_alias(p_tab_1) };
                                                        if z_name == core::ptr::null() { break '__c40; }
                                                    } else {
                                                        z_name =
                                                            unsafe {
                                                                    (*unsafe { (*p_tab_1).a_col.offset(j as isize) }).z_cn_name
                                                                } as *const i8;
                                                        if !(p_nested_from).is_null() &&
                                                                unsafe {
                                                                            (*(unsafe { (*p_nested_from).a.as_ptr() } as
                                                                                                *mut ExprList_item).offset(j as isize)).fg.e_e_name()
                                                                        } as i32 == 3 {
                                                            break '__c40;
                                                        }
                                                        if !(z_t_name).is_null() && !(p_nested_from).is_null() &&
                                                                unsafe {
                                                                        sqlite3_match_e_name(unsafe {
                                                                                    &raw mut *(unsafe { (*p_nested_from).a.as_ptr() } as
                                                                                                    *mut ExprList_item).offset(j as isize)
                                                                                } as *const ExprList_item, core::ptr::null(),
                                                                            z_t_name as *const i8, core::ptr::null(),
                                                                            core::ptr::null_mut())
                                                                    } == 0 {
                                                            break '__c40;
                                                        }
                                                        if unsafe { (*p).sel_flags } & 131072 as u32 == 0 as u32 &&
                                                                unsafe {
                                                                                (*unsafe {
                                                                                                &mut *unsafe { (*p_tab_1).a_col.offset(j as isize) }
                                                                                            }).col_flags
                                                                            } as i32 & 2 != 0 {
                                                            break '__c40;
                                                        }
                                                        if unsafe {
                                                                                    (*unsafe { (*p_tab_1).a_col.offset(j as isize) }).col_flags
                                                                                } as i32 & 1024 != 0 && z_t_name == core::ptr::null_mut() &&
                                                                sel_flags as i32 & 2048 == 0 {
                                                            break '__c40;
                                                        }
                                                    }
                                                    { let _ = 0; };
                                                    table_seen = 1;
                                                    if i > 0 && z_t_name == core::ptr::null_mut() &&
                                                            sel_flags as i32 & 2048 == 0 {
                                                        if unsafe { (*p_from).fg.is_using() } != 0 &&
                                                                unsafe {
                                                                        sqlite3_id_list_index(unsafe { (*p_from).u3.p_using },
                                                                            z_name)
                                                                    } >= 0 {
                                                            break '__c40;
                                                        }
                                                    }
                                                    p_right = unsafe { sqlite3_expr(db, 60, z_name) };
                                                    if unsafe { (*p_tab_list).n_src } > 1 &&
                                                                (unsafe { (*p_from).fg.jointype } as i32 & 64 == 0 ||
                                                                        sel_flags as i32 & 2048 != 0 ||
                                                                    (in_any_using_clause(z_name, p_from as *const SrcItem,
                                                                                    unsafe { (*p_tab_list).n_src } - i - 1) == 0) as i32 != 0)
                                                            || unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                                        let mut p_left: *mut Expr = core::ptr::null_mut();
                                                        p_left =
                                                            unsafe { sqlite3_expr(db, 60, z_tab_name as *const i8) };
                                                        p_expr =
                                                            unsafe { sqlite3_p_expr(p_parse, 142, p_left, p_right) };
                                                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 &&
                                                                !(unsafe { (*p_e).p_left }).is_null() {
                                                            unsafe {
                                                                sqlite3_rename_token_remap(p_parse, p_left as *const (),
                                                                    unsafe { (*p_e).p_left } as *const ())
                                                            };
                                                        }
                                                        if !(z_schema_name).is_null() {
                                                            p_left = unsafe { sqlite3_expr(db, 60, z_schema_name) };
                                                            p_expr =
                                                                unsafe { sqlite3_p_expr(p_parse, 142, p_left, p_expr) };
                                                        }
                                                    } else { p_expr = p_right; }
                                                    unsafe {
                                                        sqlite3_expr_set_error_offset(p_expr, i_err_ofst)
                                                    };
                                                    p_new =
                                                        unsafe { sqlite3_expr_list_append(p_parse, p_new, p_expr) };
                                                    if p_new == core::ptr::null_mut() { break '__b40; }
                                                    p_x_1 =
                                                        unsafe {
                                                            &mut *(unsafe { (*p_new).a.as_ptr() } as
                                                                            *mut ExprList_item).offset((unsafe { (*p_new).n_expr } - 1)
                                                                            as isize)
                                                        };
                                                    { let _ = 0; };
                                                    if sel_flags as i32 & 2048 != 0 &&
                                                            !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 !=
                                                                0 {
                                                        if !(p_nested_from).is_null() &&
                                                                ((0 == 0) as i32 != 0 ||
                                                                    j < unsafe { (*p_nested_from).n_expr }) {
                                                            { let _ = 0; };
                                                            unsafe {
                                                                (*p_x_1).z_e_name =
                                                                    unsafe {
                                                                        sqlite3_db_str_dup(db,
                                                                            unsafe {
                                                                                    (*(unsafe { (*p_nested_from).a.as_ptr() } as
                                                                                                    *mut ExprList_item).offset(j as isize)).z_e_name
                                                                                } as *const i8)
                                                                    }
                                                            };
                                                        } else {
                                                            unsafe {
                                                                (*p_x_1).z_e_name =
                                                                    unsafe {
                                                                        sqlite3_m_printf(db,
                                                                            c"%s.%s.%s".as_ptr() as *mut i8 as *const i8, z_schema_name,
                                                                            z_tab_name, z_name)
                                                                    }
                                                            };
                                                        }
                                                        unsafe {
                                                            (*p_x_1).fg.set_e_e_name(if j ==
                                                                                unsafe { (*p_tab_1).n_col } as i32 {
                                                                            3
                                                                        } else { 2 } as u32 as u32)
                                                        };
                                                        if unsafe { (*p_from).fg.is_using() } != 0 &&
                                                                        unsafe {
                                                                                sqlite3_id_list_index(unsafe { (*p_from).u3.p_using },
                                                                                    z_name)
                                                                            } >= 0 ||
                                                                    !(p_using).is_null() &&
                                                                        unsafe { sqlite3_id_list_index(p_using, z_name) } >= 0 ||
                                                                j < unsafe { (*p_tab_1).n_col } as i32 &&
                                                                    unsafe {
                                                                                    (*unsafe { (*p_tab_1).a_col.offset(j as isize) }).col_flags
                                                                                } as i32 & 1024 != 0 {
                                                            unsafe { (*p_x_1).fg.set_b_no_expand(1 as u32 as u32) };
                                                        }
                                                    } else if long_names != 0 {
                                                        unsafe {
                                                            (*p_x_1).z_e_name =
                                                                unsafe {
                                                                    sqlite3_m_printf(db,
                                                                        c"%s.%s".as_ptr() as *mut i8 as *const i8, z_tab_name,
                                                                        z_name)
                                                                }
                                                        };
                                                        unsafe { (*p_x_1).fg.set_e_e_name(0 as u32 as u32) };
                                                    } else {
                                                        unsafe {
                                                            (*p_x_1).z_e_name =
                                                                unsafe { sqlite3_db_str_dup(db, z_name) }
                                                        };
                                                        unsafe { (*p_x_1).fg.set_e_e_name(0 as u32 as u32) };
                                                    }
                                                    break '__c40;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                        break '__c38;
                                    }
                                    {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_from;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if (table_seen == 0) as i32 != 0 {
                                if !(z_t_name).is_null() {
                                    unsafe {
                                        sqlite3_error_msg(p_parse,
                                            c"no such table: %s".as_ptr() as *mut i8 as *const i8,
                                            z_t_name)
                                    };
                                } else {
                                    unsafe {
                                        sqlite3_error_msg(p_parse,
                                            c"no tables specified".as_ptr() as *mut i8 as *const i8)
                                    };
                                }
                            }
                        }
                        break '__c37;
                    }
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { sqlite3_expr_list_delete(db, p_e_list) };
            unsafe { (*p).p_e_list = p_new };
        }
        if !(unsafe { (*p).p_e_list }).is_null() {
            if unsafe { (*unsafe { (*p).p_e_list }).n_expr } >
                    unsafe { (*db).a_limit[2 as usize] } {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"too many columns in result set".as_ptr() as *mut i8 as
                            *const i8)
                };
                return 2;
            }
            if elist_flags & (8 | 4194304) as u32 != 0 as u32 {
                unsafe { (*p).sel_flags |= 262144 as u32 };
            }
        }
        return 0;
    }
}
extern "C" fn sqlite3_select_expand(p_parse_1: *mut Parse,
    p_select_1: *mut Select) -> () {
    let mut w: Walker = unsafe { core::mem::zeroed() };
    w.x_expr_callback = Some(sqlite3_expr_walk_noop);
    w.p_parse = p_parse_1;
    if unsafe { (*p_parse_1).has_compound() } != 0 {
        w.x_select_callback = Some(convert_compound_select_to_subquery);
        w.x_select_callback2 = None;
        unsafe { sqlite3_walk_select(&mut w, p_select_1) };
    }
    w.x_select_callback = Some(select_expander);
    w.x_select_callback2 = Some(sqlite3_select_pop_with);
    w.e_code = 0 as u16;
    unsafe { sqlite3_walk_select(&mut w, p_select_1) };
}
extern "C" fn select_add_subquery_type_info(p_walker_1: *mut Walker,
    p: *mut Select) -> () {
    unsafe {
        let mut p_parse: *mut Parse = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
        let mut p_from: *const SrcItem = core::ptr::null();
        if unsafe { (*p).sel_flags } & 128 as u32 != 0 { return; }
        unsafe { (*p).sel_flags |= 128 as u32 };
        p_parse = unsafe { (*p_walker_1).p_parse };
        { let _ = 0; };
        p_tab_list = unsafe { (*p).p_src };
        {
            {
                i = 0;
                p_from = unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem
            };
            '__b41: loop {
                if !(i < unsafe { (*p_tab_list).n_src }) { break '__b41; }
                '__c41: loop {
                    let p_tab: *mut Table = unsafe { (*p_from).p_s_tab };
                    { let _ = 0; };
                    if unsafe { (*p_tab).tab_flags } & 16384 as u32 != 0 as u32
                            && unsafe { (*p_from).fg.is_subquery() } != 0 {
                        let p_sel: *mut Select =
                            unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select };
                        sqlite3_subquery_column_types(p_parse,
                            unsafe { &mut *p_tab }, p_sel, 64 as i8);
                    }
                    break '__c41;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_from;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
    }
}
extern "C" fn sqlite3_select_add_type_info(p_parse_1: *mut Parse,
    p_select_1: *mut Select) -> () {
    let mut w: Walker = unsafe { core::mem::zeroed() };
    w.x_select_callback = Some(sqlite3_select_walk_noop);
    w.x_select_callback2 = Some(select_add_subquery_type_info);
    w.x_expr_callback = Some(sqlite3_expr_walk_noop);
    w.p_parse = p_parse_1;
    unsafe { sqlite3_walk_select(&mut w, p_select_1) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_prep(p_parse: *mut Parse, p: *mut Select,
    p_outer_nc: *mut NameContext) -> () {
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0 { return; }
    if unsafe { (*p).sel_flags } & 128 as u32 != 0 { return; }
    sqlite3_select_expand(p_parse, p);
    if unsafe { (*p_parse).n_err } != 0 { return; }
    unsafe { sqlite3_resolve_select_names(p_parse, p, p_outer_nc) };
    if unsafe { (*p_parse).n_err } != 0 { return; }
    sqlite3_select_add_type_info(p_parse, p);
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_set_of_select(p_parse: *mut Parse,
    mut p_select: *mut Select, aff: i8) -> *mut Table {
    unsafe {
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let db: *mut sqlite3 = unsafe { (*p_parse).db };
        let mut saved_flags: u64 = 0 as u64;
        {
            let __p = unsafe { &mut (*p_parse).n_nest_sel };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if unsafe { (*p_parse).n_nest_sel } >=
                unsafe { (*db).a_limit[3 as usize] } {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"VIEWs and/or subqueries nested too deep".as_ptr() as
                            *mut i8 as *const i8)
            };
            return core::ptr::null_mut();
        }
        saved_flags = unsafe { (*db).flags };
        unsafe { (*db).flags &= !(4 as u64) };
        unsafe { (*db).flags |= 64 as u64 };
        sqlite3_select_prep(p_parse, p_select, core::ptr::null_mut());
        unsafe { (*db).flags = saved_flags };
        if unsafe { (*p_parse).n_err } != 0 { return core::ptr::null_mut(); }
        while !(unsafe { (*p_select).p_prior }).is_null() {
            p_select = unsafe { (*p_select).p_prior };
        }
        p_tab =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<Table>() as u64)
                } as *mut Table;
        if p_tab == core::ptr::null_mut() { return core::ptr::null_mut(); }
        unsafe { (*p_tab).n_tab_ref = 1 as u32 };
        unsafe { (*p_tab).z_name = core::ptr::null_mut() };
        unsafe { (*p_tab).n_row_log_est = 200 as LogEst };
        { let _ = 0; };
        sqlite3_columns_from_expr_list(p_parse,
            unsafe { (*p_select).p_e_list }, unsafe { &mut (*p_tab).n_col },
            unsafe { &mut (*p_tab).a_col });
        sqlite3_subquery_column_types(p_parse, unsafe { &mut *p_tab },
            p_select, aff);
        unsafe { (*p_tab).i_p_key = -1 as i16 };
        if unsafe { (*db).malloc_failed } != 0 {
            unsafe { sqlite3_delete_table(db, p_tab) };
            return core::ptr::null_mut();
        }
        {
            let __p = unsafe { &mut (*p_parse).n_nest_sel };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        { let _ = 0; };
        return p_tab;
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct DistinctCtx {
    is_tnct: u8,
    e_tnct_type: u8,
    tab_tnct: i32,
    addr_tnct: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SortCtx {
    p_order_by: *mut ExprList,
    n_ob_sat: i32,
    i_e_cursor: i32,
    reg_return: i32,
    label_bk_out: i32,
    addr_sort_index: i32,
    label_done: i32,
    label_ob_lopt: i32,
    sort_flags: u8,
    p_deferred_row_load: *mut RowLoadInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct RowLoadInfo {
    reg_result: i32,
    ecel_flags: u8,
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_vdbe(p_parse: *mut Parse) -> *mut Vdbe {
    if !(unsafe { (*p_parse).p_vdbe }).is_null() {
        return unsafe { (*p_parse).p_vdbe };
    }
    if unsafe { (*p_parse).p_toplevel } == core::ptr::null_mut() &&
            unsafe { (*unsafe { (*p_parse).db }).db_opt_flags } & 8 as u32 ==
                0 as u32 {
        unsafe { (*p_parse).set_ok_const_factor(1 as bft as u32) };
    }
    return unsafe { sqlite3_vdbe_create(p_parse) };
}
extern "C" fn same_src_alias(p0: *mut SrcItem, p_src_1: &mut SrcList) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b43: loop {
                if !(i < (*p_src_1).n_src) { break '__b43; }
                '__c43: loop {
                    let p1: *mut SrcItem =
                        unsafe {
                            &mut *((*p_src_1).a.as_ptr() as
                                            *mut SrcItem).offset(i as isize)
                        };
                    if p1 == p0 { break '__c43; }
                    if unsafe { (*p0).p_s_tab } == unsafe { (*p1).p_s_tab } &&
                            0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { (*p0).z_alias } as *const i8,
                                        unsafe { (*p1).z_alias } as *const i8)
                                } {
                        return 1;
                    }
                    if unsafe { (*p1).fg.is_subquery() } != 0 &&
                                unsafe {
                                            (*unsafe {
                                                            (*unsafe { (*p1).u4.p_subq }).p_select
                                                        }).sel_flags
                                        } & 2048 as u32 != 0 as u32 &&
                            same_src_alias(p0,
                                    unsafe {
                                        &mut *unsafe {
                                                    (*unsafe { (*unsafe { (*p1).u4.p_subq }).p_select }).p_src
                                                }
                                    }) != 0 {
                        return 1;
                    }
                    break '__c43;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}
extern "C" fn unset_join_expr(mut p: *mut Expr, i_table_1: i32, nullable: i32)
    -> () {
    unsafe {
        while !(p).is_null() {
            if i_table_1 < 0 ||
                    unsafe { (*p).flags } & 1 as u32 != 0 as u32 &&
                        unsafe { (*p).w.i_join } == i_table_1 {
                unsafe { (*p).flags &= !((1 | 2) as u32) };
                if i_table_1 >= 0 { unsafe { (*p).flags |= 2 as u32 }; }
            }
            if unsafe { (*p).op } as i32 == 168 &&
                        unsafe { (*p).i_table } == i_table_1 &&
                    (nullable == 0) as i32 != 0 {
                unsafe { (*p).flags &= !(2097152 as u32) };
            }
            if unsafe { (*p).op } as i32 == 172 {
                { let _ = 0; };
                { let _ = 0; };
                if !(unsafe { (*p).x.p_list }).is_null() {
                    let mut i: i32 = 0;
                    {
                        i = 0;
                        '__b45: loop {
                            if !(i < unsafe { (*unsafe { (*p).x.p_list }).n_expr }) {
                                break '__b45;
                            }
                            '__c45: loop {
                                unset_join_expr(unsafe {
                                        (*(unsafe { (*unsafe { (*p).x.p_list }).a.as_ptr() } as
                                                        *mut ExprList_item).offset(i as isize)).p_expr
                                    }, i_table_1, nullable);
                                break '__c45;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
            }
            unset_join_expr(unsafe { (*p).p_left }, i_table_1, nullable);
            p = unsafe { (*p).p_right };
        }
    }
}
extern "C" fn compound_has_different_affinities(p: &Select) -> i32 {
    unsafe {
        let mut ii: i32 = 0;
        let mut p_list: *const ExprList = core::ptr::null();
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        p_list = (*p).p_e_list;
        {
            ii = 0;
            '__b46: loop {
                if !(ii < unsafe { (*p_list).n_expr }) { break '__b46; }
                '__c46: loop {
                    let mut aff: i8 = 0 as i8;
                    let mut p_sub1: *const Select = core::ptr::null();
                    { let _ = 0; };
                    aff =
                        unsafe {
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                        *mut ExprList_item).offset(ii as isize)).p_expr
                                    } as *const Expr)
                        };
                    {
                        p_sub1 = (*p).p_prior;
                        '__b47: loop {
                            if !(!(p_sub1).is_null()) { break '__b47; }
                            '__c47: loop {
                                { let _ = 0; };
                                { let _ = 0; };
                                { let _ = 0; };
                                if unsafe {
                                                sqlite3_expr_affinity(unsafe {
                                                            (*(unsafe { (*unsafe { (*p_sub1).p_e_list }).a.as_ptr() } as
                                                                            *mut ExprList_item).offset(ii as isize)).p_expr
                                                        } as *const Expr)
                                            } as i32 != aff as i32 {
                                    return 1;
                                }
                                break '__c47;
                            }
                            p_sub1 = unsafe { (*p_sub1).p_prior };
                        }
                    }
                    break '__c46;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}
extern "C" fn srclist_renumber_cursors(p_parse_1: *mut Parse,
    a_csr_map_1: *mut i32, p_src_1: &mut SrcList, i_except_1: i32) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        {
            { i = 0; p_item = (*p_src_1).a.as_ptr() as *mut SrcItem };
            '__b48: loop {
                if !(i < (*p_src_1).n_src) { break '__b48; }
                '__c48: loop {
                    if i != i_except_1 {
                        let mut p: *const Select = core::ptr::null();
                        { let _ = 0; };
                        if (unsafe { (*p_item).fg.is_recursive() } == 0) as i32 != 0
                                ||
                                unsafe {
                                        *a_csr_map_1.offset((unsafe { (*p_item).i_cursor } + 1) as
                                                    isize)
                                    } == 0 {
                            unsafe {
                                *a_csr_map_1.offset((unsafe { (*p_item).i_cursor } + 1) as
                                                isize) =
                                    {
                                        let __p = unsafe { &mut (*p_parse_1).n_tab };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    }
                            };
                        }
                        unsafe {
                            (*p_item).i_cursor =
                                unsafe {
                                    *a_csr_map_1.offset((unsafe { (*p_item).i_cursor } + 1) as
                                                isize)
                                }
                        };
                        if unsafe { (*p_item).fg.is_subquery() } != 0 {
                            {
                                p = unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select };
                                '__b49: loop {
                                    if !(!(p).is_null()) { break '__b49; }
                                    '__c49: loop {
                                        srclist_renumber_cursors(p_parse_1, a_csr_map_1,
                                            unsafe { &mut *unsafe { (*p).p_src } }, -1);
                                        break '__c49;
                                    }
                                    p = unsafe { (*p).p_prior };
                                }
                            }
                        }
                    }
                    break '__c48;
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
    }
}
extern "C" fn renumber_cursor_do_mapping(p_walker_1: &Walker,
    pi_cursor_1: &mut i32) -> () {
    unsafe {
        let a_csr_map: *const i32 = (*p_walker_1).u.ai_col as *const i32;
        let i_csr: i32 = *pi_cursor_1;
        if i_csr < unsafe { *a_csr_map.offset(0 as isize) } &&
                unsafe { *a_csr_map.offset((i_csr + 1) as isize) } > 0 {
            *pi_cursor_1 = unsafe { *a_csr_map.offset((i_csr + 1) as isize) };
        }
    }
}
extern "C" fn renumber_cursors_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let op: i32 = unsafe { (*p_expr_1).op } as i32;
        if op == 168 || op == 179 {
            renumber_cursor_do_mapping(unsafe { &*p_walker_1 },
                unsafe { &mut (*p_expr_1).i_table });
        }
        if unsafe { (*p_expr_1).flags } & 1 as u32 != 0 as u32 {
            renumber_cursor_do_mapping(unsafe { &*p_walker_1 },
                unsafe { &mut (*p_expr_1).w.i_join });
        }
        return 0;
    }
}
extern "C" fn renumber_cursors(p_parse_1: *mut Parse, p: *mut Select,
    i_except_1: i32, a_csr_map_1: *mut i32) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        srclist_renumber_cursors(p_parse_1, a_csr_map_1,
            unsafe { &mut *unsafe { (*p).p_src } }, i_except_1);
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.u.ai_col = a_csr_map_1;
        w.x_expr_callback = Some(renumber_cursors_cb);
        w.x_select_callback = Some(sqlite3_select_walk_noop);
        unsafe { sqlite3_walk_select(&mut w, p) };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SubstContext {
    p_parse: *mut Parse,
    i_table: i32,
    i_new_table: i32,
    is_outer_join: i32,
    n_sel_depth: i32,
    p_e_list: *mut ExprList,
    p_c_list: *mut ExprList,
}
extern "C" fn find_leftmost_exprlist(mut p_sel_1: *const Select)
    -> *mut ExprList {
    unsafe {
        while !(unsafe { (*p_sel_1).p_prior }).is_null() {
            p_sel_1 = unsafe { (*p_sel_1).p_prior };
        }
        return unsafe { (*p_sel_1).p_e_list };
    }
}
extern "C" fn subst_expr(p_subst_1: *mut SubstContext,
    mut p_expr_1: *mut Expr) -> *mut Expr {
    unsafe {
        if p_expr_1 == core::ptr::null_mut() { return core::ptr::null_mut(); }
        if unsafe { (*p_expr_1).flags } & (1 | 2) as u32 != 0 as u32 &&
                unsafe { (*p_expr_1).w.i_join } ==
                    unsafe { (*p_subst_1).i_table } {
            unsafe {
                (*p_expr_1).w.i_join = unsafe { (*p_subst_1).i_new_table }
            };
        }
        if unsafe { (*p_expr_1).op } as i32 == 168 &&
                    unsafe { (*p_expr_1).i_table } ==
                        unsafe { (*p_subst_1).i_table } &&
                !(unsafe { (*p_expr_1).flags } & 32 as u32 != 0 as u32) as i32
                    != 0 {
            {
                let mut p_new: *mut Expr = core::ptr::null_mut();
                let mut i_column: i32 = 0;
                let mut p_copy: *mut Expr = core::ptr::null_mut();
                let mut if_null_row: Expr = unsafe { core::mem::zeroed() };
                i_column = unsafe { (*p_expr_1).i_column } as i32;
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                p_copy =
                    unsafe {
                        (*(unsafe { (*unsafe { (*p_subst_1).p_e_list }).a.as_ptr() }
                                        as *mut ExprList_item).offset(i_column as isize)).p_expr
                    };
                if unsafe { sqlite3_expr_is_vector(p_copy as *const Expr) } !=
                        0 {
                    unsafe {
                        sqlite3_vector_error_msg(unsafe { (*p_subst_1).p_parse },
                            p_copy)
                    };
                } else {
                    let db: *mut sqlite3 =
                        unsafe { (*unsafe { (*p_subst_1).p_parse }).db };
                    if unsafe { (*p_subst_1).is_outer_join } != 0 &&
                            (unsafe { (*p_copy).op } as i32 != 168 ||
                                unsafe { (*p_copy).i_table } !=
                                    unsafe { (*p_subst_1).i_new_table }) {
                        unsafe {
                            memset(&raw mut if_null_row as *mut (), 0,
                                core::mem::size_of::<Expr>() as u64)
                        };
                        if_null_row.op = 179 as u8;
                        if_null_row.p_left = p_copy;
                        if_null_row.i_table = unsafe { (*p_subst_1).i_new_table };
                        if_null_row.i_column = -99 as ynVar;
                        if_null_row.flags = 262144 as u32;
                        p_copy = &mut if_null_row;
                    }
                    p_new =
                        unsafe { sqlite3_expr_dup(db, p_copy as *const Expr, 0) };
                    if unsafe { (*db).malloc_failed } != 0 {
                        unsafe { sqlite3_expr_delete(db, p_new) };
                        return p_expr_1;
                    }
                    if unsafe { (*p_subst_1).is_outer_join } != 0 {
                        unsafe { (*p_new).flags |= 2097152 as u32 };
                    }
                    if unsafe { (*p_new).op } as i32 == 171 {
                        unsafe {
                            (*p_new).u.i_value =
                                unsafe { sqlite3_expr_truth_value(p_new as *const Expr) }
                        };
                        unsafe { (*p_new).op = 156 as u8 };
                        unsafe { (*p_new).flags |= 2048 as u32 };
                    }
                    {
                        let p_nat: *mut CollSeq =
                            unsafe {
                                sqlite3_expr_coll_seq(unsafe { (*p_subst_1).p_parse },
                                    p_new as *const Expr)
                            };
                        let p_coll: *mut CollSeq =
                            unsafe {
                                sqlite3_expr_coll_seq(unsafe { (*p_subst_1).p_parse },
                                    unsafe {
                                            (*(unsafe { (*unsafe { (*p_subst_1).p_c_list }).a.as_ptr() }
                                                            as *mut ExprList_item).offset(i_column as isize)).p_expr
                                        } as *const Expr)
                            };
                        if p_nat != p_coll ||
                                unsafe { (*p_new).op } as i32 != 168 &&
                                    unsafe { (*p_new).op } as i32 != 114 {
                            p_new =
                                unsafe {
                                    sqlite3_expr_add_collate_string(unsafe {
                                                (*p_subst_1).p_parse
                                            } as *const Parse, p_new,
                                        if !(p_coll).is_null() {
                                                unsafe { (*p_coll).z_name }
                                            } else { c"BINARY".as_ptr() as *mut i8 } as *const i8)
                                };
                        }
                    }
                    unsafe { (*p_new).flags &= !(512 as u32) };
                    if unsafe { (*p_expr_1).flags } & (1 | 2) as u32 != 0 as u32
                        {
                        sqlite3_set_join_expr(p_new,
                            unsafe { (*p_expr_1).w.i_join },
                            unsafe { (*p_expr_1).flags } & (1 | 2) as u32);
                    }
                    unsafe { sqlite3_expr_delete(db, p_expr_1) };
                    p_expr_1 = p_new;
                }
            }
        } else {
            if unsafe { (*p_expr_1).op } as i32 == 179 &&
                    unsafe { (*p_expr_1).i_table } ==
                        unsafe { (*p_subst_1).i_table } {
                unsafe {
                    (*p_expr_1).i_table = unsafe { (*p_subst_1).i_new_table }
                };
            }
            if unsafe { (*p_expr_1).op } as i32 == 169 &&
                    unsafe { (*p_expr_1).op2 } as i32 >=
                        unsafe { (*p_subst_1).n_sel_depth } {
                {
                    let __p = unsafe { &mut (*p_expr_1).op2 };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
            unsafe {
                (*p_expr_1).p_left =
                    subst_expr(p_subst_1, unsafe { (*p_expr_1).p_left })
            };
            unsafe {
                (*p_expr_1).p_right =
                    subst_expr(p_subst_1, unsafe { (*p_expr_1).p_right })
            };
            if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                subst_select(p_subst_1, unsafe { (*p_expr_1).x.p_select }, 1);
            } else {
                subst_expr_list(p_subst_1, unsafe { (*p_expr_1).x.p_list });
            }
            if unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                let p_win: *mut Window = unsafe { (*p_expr_1).y.p_win };
                unsafe {
                    (*p_win).p_filter =
                        subst_expr(p_subst_1, unsafe { (*p_win).p_filter })
                };
                subst_expr_list(p_subst_1, unsafe { (*p_win).p_partition });
                subst_expr_list(p_subst_1, unsafe { (*p_win).p_order_by });
            }
        }
        return p_expr_1;
    }
}
extern "C" fn subst_expr_list(p_subst: *mut SubstContext,
    p_list: *mut ExprList) -> () {
    let mut i: i32 = 0;
    if p_list == core::ptr::null_mut() { return; }
    {
        i = 0;
        '__b51: loop {
            if !(i < unsafe { (*p_list).n_expr }) { break '__b51; }
            '__c51: loop {
                unsafe {
                    (*(unsafe { (*p_list).a.as_ptr() } as
                                        *mut ExprList_item).offset(i as isize)).p_expr =
                        subst_expr(p_subst,
                            unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(i as isize)).p_expr
                            })
                };
                break '__c51;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn subst_select(p_subst: *mut SubstContext, mut p: *mut Select,
    do_prior: i32) -> () {
    unsafe {
        let mut p_src: *mut SrcList = core::ptr::null_mut();
        let mut p_item: *const SrcItem = core::ptr::null();
        let mut i: i32 = 0;
        if (p).is_null() as i32 != 0 { return; }
        {
            let __p = unsafe { &mut (*p_subst).n_sel_depth };
            let __t = *__p;
            *__p += 1;
            __t
        };
        '__b52: loop {
            '__c52: loop {
                subst_expr_list(p_subst, unsafe { (*p).p_e_list });
                subst_expr_list(p_subst, unsafe { (*p).p_group_by });
                subst_expr_list(p_subst, unsafe { (*p).p_order_by });
                unsafe {
                    (*p).p_having =
                        subst_expr(p_subst, unsafe { (*p).p_having })
                };
                unsafe {
                    (*p).p_where = subst_expr(p_subst, unsafe { (*p).p_where })
                };
                p_src = unsafe { (*p).p_src };
                { let _ = 0; };
                {
                    {
                        i = unsafe { (*p_src).n_src };
                        p_item = unsafe { (*p_src).a.as_ptr() } as *mut SrcItem
                    };
                    '__b53: loop {
                        if !(i > 0) { break '__b53; }
                        '__c53: loop {
                            if unsafe { (*p_item).fg.is_subquery() } != 0 {
                                subst_select(p_subst,
                                    unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select }, 1);
                            }
                            if unsafe { (*p_item).fg.is_tab_func() } != 0 {
                                subst_expr_list(p_subst,
                                    unsafe { (*p_item).u1.p_func_arg });
                            }
                            break '__c53;
                        }
                        {
                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p_item;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                    }
                }
                break '__c52;
            }
            if !(do_prior != 0 &&
                            { p = unsafe { (*p).p_prior }; p } != core::ptr::null_mut())
                {
                break '__b52;
            }
        }
        {
            let __p = unsafe { &mut (*p_subst).n_sel_depth };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
}
extern "C" fn recompute_columns_used_expr(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        if unsafe { (*p_expr_1).op } as i32 != 168 { return 0; }
        p_item = unsafe { (*p_walker_1).u.p_src_item };
        if unsafe { (*p_item).i_cursor } != unsafe { (*p_expr_1).i_table } {
            return 0;
        }
        if (unsafe { (*p_expr_1).i_column } as i32) < 0 { return 0; }
        unsafe {
            (*p_item).col_used |= unsafe { sqlite3_expr_col_used(p_expr_1) }
        };
        return 0;
    }
}
extern "C" fn recompute_columns_used(p_select_1: *mut Select,
    p_src_item_1: *mut SrcItem) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        if unsafe { (*p_src_item_1).p_s_tab } == core::ptr::null_mut() {
            return;
        }
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.x_expr_callback = Some(recompute_columns_used_expr);
        w.x_select_callback = Some(sqlite3_select_walk_noop);
        w.u.p_src_item = p_src_item_1;
        unsafe { (*p_src_item_1).col_used = 0 as Bitmask };
        unsafe { sqlite3_walk_select(&mut w, p_select_1) };
    }
}
extern "C" fn clear_select(db: *mut sqlite3, mut p: *mut Select,
    mut b_free_1: i32) -> () {
    unsafe {
        { let _ = 0; };
        while !(p).is_null() {
            let p_prior: *mut Select = unsafe { (*p).p_prior };
            unsafe { sqlite3_expr_list_delete(db, unsafe { (*p).p_e_list }) };
            unsafe { sqlite3_src_list_delete(db, unsafe { (*p).p_src }) };
            unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_where }) };
            unsafe {
                sqlite3_expr_list_delete(db, unsafe { (*p).p_group_by })
            };
            unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_having }) };
            unsafe {
                sqlite3_expr_list_delete(db, unsafe { (*p).p_order_by })
            };
            unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_limit }) };
            if !(unsafe { (*p).p_with }).is_null() {
                unsafe { sqlite3_with_delete(db, unsafe { (*p).p_with }) };
            }
            if !(unsafe { (*p).p_win_defn }).is_null() {
                unsafe {
                    sqlite3_window_list_delete(db, unsafe { (*p).p_win_defn })
                };
            }
            while !(unsafe { (*p).p_win }).is_null() {
                { let _ = 0; };
                unsafe {
                    sqlite3_window_unlink_from_select(unsafe { (*p).p_win })
                };
            }
            if b_free_1 != 0 {
                unsafe { sqlite3_db_nn_free_nn(db, p as *mut ()) };
            }
            p = p_prior;
            b_free_1 = 1;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_delete(db: *mut sqlite3, p: *mut Select)
    -> () {
    if !(p).is_null() { clear_select(db, p, 1); }
}
extern "C" fn flatten_subquery(p_parse_1: *mut Parse, p: *mut Select,
    i_from_1: i32, is_agg_1: i32) -> i32 {
    unsafe {
        let z_saved_auth_context: *const i8 =
            unsafe { (*p_parse_1).z_auth_context };
        let mut p_parent: *mut Select = core::ptr::null_mut();
        let mut p_sub: *mut Select = core::ptr::null_mut();
        let mut p_sub1: *mut Select = core::ptr::null_mut();
        let mut p_src: *mut SrcList = core::ptr::null_mut();
        let mut p_sub_src: *mut SrcList = core::ptr::null_mut();
        let mut i_parent: i32 = 0;
        let mut i_new_parent: i32 = -1;
        let mut is_outer_join: i32 = 0;
        let mut i: i32 = 0;
        let mut p_where: *mut Expr = core::ptr::null_mut();
        let mut p_subitem: *mut SrcItem = core::ptr::null_mut();
        let db: *mut sqlite3 = unsafe { (*p_parse_1).db };
        let mut w: Walker = unsafe { core::mem::zeroed() };
        let mut a_csr_map: *mut i32 = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*db).db_opt_flags } & 1 as u32 != 0 as u32 { return 0; }
        p_src = unsafe { (*p).p_src };
        { let _ = 0; };
        p_subitem =
            unsafe {
                &mut *(unsafe { (*p_src).a.as_ptr() } as
                                *mut SrcItem).offset(i_from_1 as isize)
            };
        i_parent = unsafe { (*p_subitem).i_cursor };
        { let _ = 0; };
        p_sub = unsafe { (*unsafe { (*p_subitem).u4.p_subq }).p_select };
        { let _ = 0; };
        if !(unsafe { (*p).p_win }).is_null() ||
                !(unsafe { (*p_sub).p_win }).is_null() {
            return 0;
        }
        p_sub_src = unsafe { (*p_sub).p_src };
        { let _ = 0; };
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                !(unsafe { (*p).p_limit }).is_null() {
            return 0;
        }
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                !(unsafe { (*unsafe { (*p_sub).p_limit }).p_right }).is_null()
            {
            return 0;
        }
        if unsafe { (*p).sel_flags } & 256 as u32 != 0 as u32 &&
                !(unsafe { (*p_sub).p_limit }).is_null() {
            return 0;
        }
        if unsafe { (*p_sub_src).n_src } == 0 { return 0; }
        if unsafe { (*p_sub).sel_flags } & 1 as u32 != 0 { return 0; }
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                (unsafe { (*p_src).n_src } > 1 || is_agg_1 != 0) {
            return 0;
        }
        if !(unsafe { (*p).p_order_by }).is_null() &&
                !(unsafe { (*p_sub).p_order_by }).is_null() {
            return 0;
        }
        if is_agg_1 != 0 && !(unsafe { (*p_sub).p_order_by }).is_null() {
            return 0;
        }
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                !(unsafe { (*p).p_where }).is_null() {
            return 0;
        }
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                unsafe { (*p).sel_flags } & 1 as u32 != 0 as u32 {
            return 0;
        }
        if unsafe { (*p_sub).sel_flags } & 8192 as u32 != 0 { return 0; }
        if unsafe { (*p_subitem).fg.jointype } as i32 & (32 | 64) != 0 {
            if unsafe { (*p_sub_src).n_src } > 1 ||
                        unsafe { (*p).sel_flags } & 1 as u32 != 0 as u32 ||
                    unsafe { (*p_subitem).fg.jointype } as i32 & 16 != 0 {
                return 0;
            }
            is_outer_join = 1;
        }
        { let _ = 0; };
        if i_from_1 > 0 &&
                unsafe {
                                (*(unsafe { (*p_sub_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).fg.jointype
                            } as i32 & 64 != 0 {
            return 0;
        }
        { let _ = 0; };
        if !(unsafe { (*p_sub).p_prior }).is_null() {
            let mut ii: i32 = 0;
            if !(unsafe { (*p_sub).p_order_by }).is_null() { return 0; }
            if is_agg_1 != 0 ||
                        unsafe { (*p).sel_flags } & 1 as u32 != 0 as u32 ||
                    is_outer_join > 0 {
                return 0;
            }
            {
                p_sub1 = p_sub;
                '__b56: loop {
                    if !(!(p_sub1).is_null()) { break '__b56; }
                    '__c56: loop {
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_sub1).sel_flags } & (1 | 8) as u32 !=
                                            0 as u32 ||
                                        !(unsafe { (*p_sub1).p_prior }).is_null() &&
                                            unsafe { (*p_sub1).op } as i32 != 136 ||
                                    unsafe { (*unsafe { (*p_sub1).p_src }).n_src } < 1 ||
                                !(unsafe { (*p_sub1).p_win }).is_null() {
                            return 0;
                        }
                        if i_from_1 > 0 &&
                                unsafe {
                                                (*(unsafe { (*unsafe { (*p_sub1).p_src }).a.as_ptr() } as
                                                                    *mut SrcItem).offset(0 as isize)).fg.jointype
                                            } as i32 & 64 != 0 {
                            return 0;
                        }
                        break '__c56;
                    }
                    p_sub1 = unsafe { (*p_sub1).p_prior };
                }
            }
            if !(unsafe { (*p).p_order_by }).is_null() {
                {
                    ii = 0;
                    '__b57: loop {
                        if !(ii < unsafe { (*unsafe { (*p).p_order_by }).n_expr }) {
                            break '__b57;
                        }
                        '__c57: loop {
                            if unsafe {
                                            (*(unsafe { (*unsafe { (*p).p_order_by }).a.as_ptr() } as
                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                        } as i32 == 0 {
                                return 0;
                            }
                            break '__c57;
                        }
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            if unsafe { (*p).sel_flags } & 8192 as u32 != 0 { return 0; }
            if compound_has_different_affinities(unsafe { &*p_sub }) != 0 {
                return 0;
            }
            if unsafe { (*p_src).n_src } > 1 {
                if unsafe { (*p_parse_1).n_select } > 500 { return 0; }
                if unsafe { (*db).db_opt_flags } & 8388608 as u32 != 0 as u32
                    {
                    return 0;
                }
                a_csr_map =
                    unsafe {
                            sqlite3_db_malloc_zero(db,
                                (unsafe { (*p_parse_1).n_tab } as i64 + 1 as i64) as u64 *
                                    core::mem::size_of::<i32>() as u64)
                        } as *mut i32;
                if !(a_csr_map).is_null() {
                    unsafe {
                        *a_csr_map.offset(0 as isize) =
                            unsafe { (*p_parse_1).n_tab }
                    };
                }
            }
        }
        unsafe {
            (*p_parse_1).z_auth_context =
                unsafe { (*p_subitem).z_name } as *const i8
        };
        unsafe {
            sqlite3_auth_check(p_parse_1, 21, core::ptr::null(),
                core::ptr::null(), core::ptr::null())
        };
        unsafe { (*p_parse_1).z_auth_context = z_saved_auth_context };
        if unsafe { (*p_subitem).fg.is_subquery() } != 0 {
            p_sub1 = unsafe { sqlite3_subquery_detach(db, p_subitem) };
        } else { p_sub1 = core::ptr::null_mut(); }
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            sqlite3_db_free(db, unsafe { (*p_subitem).z_name } as *mut ())
        };
        unsafe {
            sqlite3_db_free(db, unsafe { (*p_subitem).z_alias } as *mut ())
        };
        unsafe { (*p_subitem).z_name = core::ptr::null_mut() };
        unsafe { (*p_subitem).z_alias = core::ptr::null_mut() };
        { let _ = 0; };
        {
            p_sub = unsafe { (*p_sub).p_prior };
            '__b58: loop {
                if !(!(p_sub).is_null()) { break '__b58; }
                '__c58: loop {
                    let mut p_new: *mut Select = core::ptr::null_mut();
                    let p_order_by: *mut ExprList = unsafe { (*p).p_order_by };
                    let p_limit: *mut Expr = unsafe { (*p).p_limit };
                    let p_prior: *mut Select = unsafe { (*p).p_prior };
                    let p_item_tab: *mut Table =
                        unsafe { (*p_subitem).p_s_tab };
                    unsafe { (*p_subitem).p_s_tab = core::ptr::null_mut() };
                    unsafe { (*p).p_order_by = core::ptr::null_mut() };
                    unsafe { (*p).p_prior = core::ptr::null_mut() };
                    unsafe { (*p).p_limit = core::ptr::null_mut() };
                    p_new =
                        unsafe { sqlite3_select_dup(db, p as *const Select, 0) };
                    unsafe { (*p).p_limit = p_limit };
                    unsafe { (*p).p_order_by = p_order_by };
                    unsafe { (*p).op = 136 as u8 };
                    unsafe { (*p_subitem).p_s_tab = p_item_tab };
                    if p_new == core::ptr::null_mut() {
                        unsafe { (*p).p_prior = p_prior };
                    } else {
                        unsafe {
                            (*p_new).sel_id =
                                {
                                        let __p = unsafe { &mut (*p_parse_1).n_select };
                                        *__p += 1;
                                        *__p
                                    } as u32
                        };
                        if !(a_csr_map).is_null() &&
                                unsafe { (*db).malloc_failed } as i32 == 0 {
                            renumber_cursors(p_parse_1, p_new, i_from_1, a_csr_map);
                        }
                        unsafe { (*p_new).p_prior = p_prior };
                        if !(p_prior).is_null() {
                            unsafe { (*p_prior).p_next = p_new };
                        }
                        unsafe { (*p_new).p_next = p };
                        unsafe { (*p).p_prior = p_new };
                    }
                    { let _ = 0; };
                    break '__c58;
                }
                p_sub = unsafe { (*p_sub).p_prior };
            }
        }
        unsafe { sqlite3_db_free(db, a_csr_map as *mut ()) };
        if unsafe { (*db).malloc_failed } != 0 {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                sqlite3_src_item_attach_subquery(p_parse_1, p_subitem, p_sub1,
                    0)
            };
            return 1;
        }
        if unsafe { (*p_subitem).p_s_tab } != core::ptr::null_mut() {
            let p_tab_to_del: *mut Table = unsafe { (*p_subitem).p_s_tab };
            if unsafe { (*p_tab_to_del).n_tab_ref } == 1 as u32 {
                let p_toplevel: *mut Parse =
                    if !(unsafe { (*p_parse_1).p_toplevel }).is_null() {
                        unsafe { (*p_parse_1).p_toplevel }
                    } else { p_parse_1 };
                unsafe {
                    sqlite3_parser_add_cleanup(p_toplevel,
                        Some(sqlite3_delete_table_generic), p_tab_to_del as *mut ())
                };
            } else {
                {
                    let __p = unsafe { &mut (*p_tab_to_del).n_tab_ref };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
            unsafe { (*p_subitem).p_s_tab = core::ptr::null_mut() };
        }
        p_sub = p_sub1;
        {
            p_parent = p;
            '__b59: loop {
                if !(!(p_parent).is_null()) { break '__b59; }
                '__c59: loop {
                    let mut n_sub_src: i32 = 0;
                    let jointype: u8 = unsafe { (*p_subitem).fg.jointype };
                    { let _ = 0; };
                    p_sub_src = unsafe { (*p_sub).p_src };
                    n_sub_src = unsafe { (*p_sub_src).n_src };
                    p_src = unsafe { (*p_parent).p_src };
                    if n_sub_src > 1 {
                        p_src =
                            unsafe {
                                sqlite3_src_list_enlarge(p_parse_1, p_src, n_sub_src - 1,
                                    i_from_1 + 1)
                            };
                        if p_src == core::ptr::null_mut() { break '__b59; }
                        unsafe { (*p_parent).p_src = p_src };
                        p_subitem =
                            unsafe {
                                &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(i_from_1 as isize)
                            };
                    }
                    i_new_parent =
                        unsafe {
                            (*(unsafe { (*p_sub_src).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as isize)).i_cursor
                        };
                    {
                        i = 0;
                        '__b60: loop {
                            if !(i < n_sub_src) { break '__b60; }
                            '__c60: loop {
                                let p_item: *mut SrcItem =
                                    unsafe {
                                        &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset((i + i_from_1) as isize)
                                    };
                                { let _ = 0; };
                                { let _ = 0; };
                                if unsafe { (*p_item).fg.is_using() } != 0 {
                                    unsafe {
                                        sqlite3_id_list_delete(db, unsafe { (*p_item).u3.p_using })
                                    };
                                }
                                unsafe {
                                    *p_item =
                                        unsafe {
                                            *(unsafe { (*p_sub_src).a.as_ptr() } as
                                                        *mut SrcItem).offset(i as isize)
                                        }
                                };
                                unsafe {
                                    (*p_item).fg.jointype |= (jointype as i32 & 64) as u8
                                };
                                unsafe {
                                    memset(unsafe {
                                                &raw mut *(unsafe { (*p_sub_src).a.as_ptr() } as
                                                                *mut SrcItem).offset(i as isize)
                                            } as *mut (), 0, core::mem::size_of::<SrcItem>() as u64)
                                };
                                break '__c60;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe {
                        (*p_subitem).fg.jointype |= jointype as i32 as u8
                    };
                    if !(unsafe { (*p_sub).p_order_by }).is_null() {
                        let p_order_by_1: *mut ExprList =
                            unsafe { (*p_sub).p_order_by };
                        {
                            i = 0;
                            '__b61: loop {
                                if !(i < unsafe { (*p_order_by_1).n_expr }) {
                                    break '__b61;
                                }
                                '__c61: loop {
                                    unsafe {
                                        (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col =
                                            0 as u16
                                    };
                                    break '__c61;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        { let _ = 0; };
                        unsafe { (*p_parent).p_order_by = p_order_by_1 };
                        unsafe { (*p_sub).p_order_by = core::ptr::null_mut() };
                    }
                    p_where = unsafe { (*p_sub).p_where };
                    unsafe { (*p_sub).p_where = core::ptr::null_mut() };
                    if is_outer_join > 0 {
                        { let _ = 0; };
                        sqlite3_set_join_expr(p_where, i_new_parent, 1 as u32);
                    }
                    if !(p_where).is_null() {
                        if !(unsafe { (*p_parent).p_where }).is_null() {
                            unsafe {
                                (*p_parent).p_where =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 44, p_where,
                                            unsafe { (*p_parent).p_where })
                                    }
                            };
                        } else { unsafe { (*p_parent).p_where = p_where }; }
                    }
                    if unsafe { (*db).malloc_failed } as i32 == 0 {
                        let mut x: SubstContext = unsafe { core::mem::zeroed() };
                        x.p_parse = p_parse_1;
                        x.i_table = i_parent;
                        x.i_new_table = i_new_parent;
                        x.is_outer_join = is_outer_join;
                        x.n_sel_depth = 0;
                        x.p_e_list = unsafe { (*p_sub).p_e_list };
                        x.p_c_list = find_leftmost_exprlist(p_sub as *const Select);
                        subst_select(&mut x, p_parent, 0);
                    }
                    unsafe {
                        (*p_parent).sel_flags |=
                            unsafe { (*p_sub).sel_flags } & 256 as u32
                    };
                    { let _ = 0; };
                    if !(unsafe { (*p_sub).p_limit }).is_null() {
                        unsafe {
                            (*p_parent).p_limit = unsafe { (*p_sub).p_limit }
                        };
                        unsafe { (*p_sub).p_limit = core::ptr::null_mut() };
                    }
                    {
                        i = 0;
                        '__b62: loop {
                            if !(i < n_sub_src) { break '__b62; }
                            '__c62: loop {
                                recompute_columns_used(p_parent,
                                    unsafe {
                                        &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset((i + i_from_1) as isize)
                                    });
                                break '__c62;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__c59;
                }
                {
                    p_parent = unsafe { (*p_parent).p_prior };
                    p_sub = unsafe { (*p_sub).p_prior }
                };
            }
        }
        unsafe { sqlite3_agg_info_persist_walker_init(&mut w, p_parse_1) };
        unsafe { sqlite3_walk_select(&mut w, p_sub1) };
        sqlite3_select_delete(db, p_sub1);
        return 1;
    }
}
extern "C" fn code_offset(v: *mut Vdbe, i_offset_1: i32, i_continue_1: i32)
    -> () {
    if i_offset_1 > 0 {
        unsafe { sqlite3_vdbe_add_op3(v, 61, i_offset_1, i_continue_1, 1) };
    }
}
extern "C" fn inner_loop_load_row(p_parse_1: *mut Parse, p_select_1: &Select,
    p_info_1: &RowLoadInfo) -> () {
    unsafe {
        unsafe {
            sqlite3_expr_code_expr_list(p_parse_1, (*p_select_1).p_e_list,
                (*p_info_1).reg_result, 0, (*p_info_1).ecel_flags)
        };
    }
}
extern "C" fn code_distinct(p_parse_1: *mut Parse, e_tnct_type_1: i32,
    i_tab_1: i32, addr_repeat_1: i32, p_e_list_1: &ExprList, reg_elem_1: i32)
    -> i32 {
    let mut i_ret: i32 = 0;
    let n_result_col: i32 = (*p_e_list_1).n_expr;
    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
    '__s63:
        {
        match e_tnct_type_1 {
            2 => {
                {
                    let mut i: i32 = 0;
                    let mut i_jump: i32 = 0;
                    let mut reg_prev: i32 = 0;
                    i_ret =
                        { reg_prev = unsafe { (*p_parse_1).n_mem } + 1; reg_prev };
                    unsafe { (*p_parse_1).n_mem += n_result_col };
                    i_jump =
                        unsafe { sqlite3_vdbe_current_addr(v) } + n_result_col;
                    {
                        i = 0;
                        '__b64: loop {
                            if !(i < n_result_col) { break '__b64; }
                            '__c64: loop {
                                let p_coll: *const CollSeq =
                                    unsafe {
                                            sqlite3_expr_coll_seq(p_parse_1,
                                                unsafe {
                                                        (*((*p_e_list_1).a.as_ptr() as
                                                                        *mut ExprList_item).offset(i as isize)).p_expr
                                                    } as *const Expr)
                                        } as *const CollSeq;
                                if i < n_result_col - 1 {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 53, reg_elem_1 + i, i_jump,
                                            reg_prev + i)
                                    };
                                } else {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 54, reg_elem_1 + i, addr_repeat_1,
                                            reg_prev + i)
                                    };
                                }
                                unsafe {
                                    sqlite3_vdbe_change_p4(v, -1, p_coll as *const i8, -2)
                                };
                                unsafe { sqlite3_vdbe_change_p5(v, 128 as u16) };
                                break '__c64;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    { let _ = 0; };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 82, reg_elem_1, reg_prev,
                            n_result_col - 1)
                    };
                    break '__s63;
                }
                { break '__s63; }
                {
                    let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 29, i_tab_1, addr_repeat_1,
                            reg_elem_1, n_result_col)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 99, reg_elem_1, n_result_col, r1)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 140, i_tab_1, r1, reg_elem_1,
                            n_result_col)
                    };
                    unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                    unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                    i_ret = i_tab_1;
                    break '__s63;
                }
            }
            1 => {
                { break '__s63; }
                {
                    let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 29, i_tab_1, addr_repeat_1,
                            reg_elem_1, n_result_col)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 99, reg_elem_1, n_result_col, r1)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 140, i_tab_1, r1, reg_elem_1,
                            n_result_col)
                    };
                    unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                    unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                    i_ret = i_tab_1;
                    break '__s63;
                }
            }
            _ => {
                {
                    let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 29, i_tab_1, addr_repeat_1,
                            reg_elem_1, n_result_col)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 99, reg_elem_1, n_result_col, r1)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 140, i_tab_1, r1, reg_elem_1,
                            n_result_col)
                    };
                    unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                    unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                    i_ret = i_tab_1;
                    break '__s63;
                }
            }
        }
    }
    return i_ret;
}
extern "C" fn fix_distinct_open_eph(p_parse_1: &Parse, e_tnct_type_1: i32,
    i_val_1: i32, i_open_eph_addr_1: i32) -> () {
    if (*p_parse_1).n_err == 0 && (e_tnct_type_1 == 1 || e_tnct_type_1 == 2) {
        let v: *mut Vdbe = (*p_parse_1).p_vdbe;
        unsafe { sqlite3_vdbe_change_to_noop(v, i_open_eph_addr_1) };
        if unsafe {
                        (*unsafe {
                                        sqlite3_vdbe_get_op(v, i_open_eph_addr_1 + 1)
                                    }).opcode
                    } as i32 == 190 {
            unsafe { sqlite3_vdbe_change_to_noop(v, i_open_eph_addr_1 + 1) };
        }
        if e_tnct_type_1 == 2 {
            let p_op: *mut VdbeOp =
                unsafe { sqlite3_vdbe_get_op(v, i_open_eph_addr_1) };
            unsafe { (*p_op).opcode = 77 as u8 };
            unsafe { (*p_op).p1 = 1 };
            unsafe { (*p_op).p2 = i_val_1 };
        }
    }
}
extern "C" fn make_sorter_record(p_parse_1: *mut Parse, p_sort_1: &SortCtx,
    p_select_1: *mut Select, reg_base_1: i32, n_base_1: i32) -> i32 {
    let n_ob_sat: i32 = (*p_sort_1).n_ob_sat;
    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
    let reg_out: i32 =
        { let __p = unsafe { &mut (*p_parse_1).n_mem }; *__p += 1; *__p };
    if !((*p_sort_1).p_deferred_row_load).is_null() {
        inner_loop_load_row(p_parse_1, unsafe { &*p_select_1 },
            unsafe { &*(*p_sort_1).p_deferred_row_load });
    }
    unsafe {
        sqlite3_vdbe_add_op3(v, 99, reg_base_1 + n_ob_sat,
            n_base_1 - n_ob_sat, reg_out)
    };
    return reg_out;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_key_info_alloc(db: *mut sqlite3, n: i32, x: i32)
    -> *mut KeyInfo {
    let n_extra: i32 =
        ((n + x) as u64 *
                (core::mem::size_of::<*mut CollSeq>() as u64 + 1 as u64)) as
            i32;
    let mut p: *mut KeyInfo = core::ptr::null_mut();
    { let _ = 0; };
    if n + x > 65535 {
        return unsafe { sqlite3_oom_fault(db) } as *mut KeyInfo;
    }
    p =
        unsafe {
                sqlite3_db_malloc_raw_nn(db,
                    core::mem::offset_of!(KeyInfo, a_coll) as u64 +
                            0 as u64 * core::mem::size_of::<*mut CollSeq>() as u64 +
                        n_extra as u64)
            } as *mut KeyInfo;
    if !(p).is_null() {
        unsafe {
            (*p).a_sort_flags =
                unsafe {
                        &raw mut *(unsafe { (*p).a_coll.as_ptr() } as
                                        *mut *mut CollSeq).offset((n + x) as isize)
                    } as *mut u8
        };
        unsafe { (*p).n_key_field = n as u16 };
        unsafe { (*p).n_all_field = (n + x) as u16 };
        unsafe { (*p).enc = unsafe { (*db).enc } };
        unsafe { (*p).db = db };
        unsafe { (*p).n_ref = 1 as u32 };
        unsafe {
            memset(unsafe { (*p).a_coll.as_ptr() } as *mut *mut CollSeq as
                    *mut (), 0, n_extra as u64)
        };
    } else { return unsafe { sqlite3_oom_fault(db) } as *mut KeyInfo; }
    return p;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_key_info_from_expr_list(p_parse: *mut Parse,
    p_list: &ExprList, i_start: i32, n_extra: i32) -> *mut KeyInfo {
    let mut n_expr: i32 = 0;
    let mut p_info: *mut KeyInfo = core::ptr::null_mut();
    let mut p_item: *const ExprList_item = core::ptr::null();
    let db: *mut sqlite3 = unsafe { (*p_parse).db };
    let mut i: i32 = 0;
    n_expr = (*p_list).n_expr;
    p_info = sqlite3_key_info_alloc(db, n_expr - i_start, n_extra + 1);
    if !(p_info).is_null() {
        { let _ = 0; };
        {
            {
                i = i_start;
                p_item =
                    unsafe {
                        ((*p_list).a.as_ptr() as
                                *mut ExprList_item).offset(i_start as isize)
                    }
            };
            '__b65: loop {
                if !(i < n_expr) { break '__b65; }
                '__c65: loop {
                    unsafe {
                        *(unsafe { (*p_info).a_coll.as_ptr() } as
                                        *mut *mut CollSeq).offset((i - i_start) as isize) =
                            unsafe {
                                sqlite3_expr_nn_coll_seq(p_parse,
                                    unsafe { (*p_item).p_expr } as *const Expr)
                            }
                    };
                    unsafe {
                        *unsafe {
                                    (*p_info).a_sort_flags.offset((i - i_start) as isize)
                                } = unsafe { (*p_item).fg.sort_flags }
                    };
                    break '__c65;
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
    }
    return p_info;
}
extern "C" fn push_onto_sorter(p_parse_1: *mut Parse, p_sort_1: *mut SortCtx,
    p_select_1: *mut Select, reg_data_1: i32, reg_orig_data_1: i32,
    n_data_1: i32, n_prefix_reg_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let b_seq: i32 =
            (unsafe { (*p_sort_1).sort_flags } as i32 & 1 == 0) as i32;
        let n_expr: i32 =
            unsafe { (*unsafe { (*p_sort_1).p_order_by }).n_expr };
        let n_base: i32 = n_expr + b_seq + n_data_1;
        let mut reg_base: i32 = 0;
        let mut reg_record: i32 = 0;
        let n_ob_sat: i32 = unsafe { (*p_sort_1).n_ob_sat };
        let mut op: i32 = 0;
        let mut i_limit: i32 = 0;
        let mut i_skip: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        if n_prefix_reg_1 != 0 {
            { let _ = 0; };
            reg_base = reg_data_1 - n_prefix_reg_1;
        } else {
            reg_base = unsafe { (*p_parse_1).n_mem } + 1;
            unsafe { (*p_parse_1).n_mem += n_base };
        }
        { let _ = 0; };
        i_limit =
            if unsafe { (*p_select_1).i_offset } != 0 {
                (unsafe { (*p_select_1).i_offset }) + 1
            } else { unsafe { (*p_select_1).i_limit } };
        unsafe {
            (*p_sort_1).label_done =
                unsafe { sqlite3_vdbe_make_label(p_parse_1) }
        };
        unsafe {
            sqlite3_expr_code_expr_list(p_parse_1,
                unsafe { (*p_sort_1).p_order_by }, reg_base, reg_orig_data_1,
                (1 | if reg_orig_data_1 != 0 { 4 } else { 0 }) as u8)
        };
        if b_seq != 0 {
            unsafe {
                sqlite3_vdbe_add_op2(v, 128,
                    unsafe { (*p_sort_1).i_e_cursor }, reg_base + n_expr)
            };
        }
        if n_prefix_reg_1 == 0 && n_data_1 > 0 {
            unsafe {
                sqlite3_expr_code_move(p_parse_1, reg_data_1,
                    reg_base + n_expr + b_seq, n_data_1)
            };
        }
        if n_ob_sat > 0 {
            let mut reg_prev_key: i32 = 0;
            let mut addr_first: i32 = 0;
            let mut addr_jmp: i32 = 0;
            let mut p_op: *mut VdbeOp = core::ptr::null_mut();
            let mut n_key: i32 = 0;
            let mut p_ki: *mut KeyInfo = core::ptr::null_mut();
            reg_record =
                make_sorter_record(p_parse_1, unsafe { &*p_sort_1 },
                    p_select_1, reg_base, n_base);
            reg_prev_key = unsafe { (*p_parse_1).n_mem } + 1;
            unsafe { (*p_parse_1).n_mem += unsafe { (*p_sort_1).n_ob_sat } };
            n_key = n_expr - unsafe { (*p_sort_1).n_ob_sat } + b_seq;
            if b_seq != 0 {
                addr_first =
                    unsafe { sqlite3_vdbe_add_op1(v, 17, reg_base + n_expr) };
            } else {
                addr_first =
                    unsafe {
                        sqlite3_vdbe_add_op1(v, 122,
                            unsafe { (*p_sort_1).i_e_cursor })
                    };
            }
            unsafe {
                sqlite3_vdbe_add_op3(v, 92, reg_prev_key, reg_base,
                    unsafe { (*p_sort_1).n_ob_sat })
            };
            p_op =
                unsafe {
                    sqlite3_vdbe_get_op(v,
                        unsafe { (*p_sort_1).addr_sort_index })
                };
            if unsafe { (*unsafe { (*p_parse_1).db }).malloc_failed } != 0 {
                return;
            }
            unsafe { (*p_op).p2 = n_key + n_data_1 };
            p_ki = unsafe { (*p_op).p4.p_key_info };
            unsafe {
                memset(unsafe { (*p_ki).a_sort_flags } as *mut (), 0,
                    unsafe { (*p_ki).n_key_field } as u64)
            };
            unsafe {
                sqlite3_vdbe_change_p4(v, -1, p_ki as *mut i8 as *const i8,
                    -9)
            };
            unsafe {
                (*p_op).p4.p_key_info =
                    sqlite3_key_info_from_expr_list(p_parse_1,
                        unsafe { &*unsafe { (*p_sort_1).p_order_by } }, n_ob_sat,
                        unsafe { (*p_ki).n_all_field } as i32 -
                                unsafe { (*p_ki).n_key_field } as i32 - 1)
            };
            p_op = core::ptr::null_mut();
            addr_jmp = unsafe { sqlite3_vdbe_current_addr(v) };
            unsafe {
                sqlite3_vdbe_add_op3(v, 14, addr_jmp + 1, 0, addr_jmp + 1)
            };
            unsafe {
                (*p_sort_1).label_bk_out =
                    unsafe { sqlite3_vdbe_make_label(p_parse_1) }
            };
            unsafe {
                (*p_sort_1).reg_return =
                    {
                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                        *__p += 1;
                        *__p
                    }
            };
            unsafe {
                sqlite3_vdbe_add_op2(v, 10, unsafe { (*p_sort_1).reg_return },
                    unsafe { (*p_sort_1).label_bk_out })
            };
            unsafe {
                sqlite3_vdbe_add_op1(v, 148,
                    unsafe { (*p_sort_1).i_e_cursor })
            };
            if i_limit != 0 {
                unsafe {
                    sqlite3_vdbe_add_op2(v, 17, i_limit,
                        unsafe { (*p_sort_1).label_done })
                };
            }
            unsafe { sqlite3_vdbe_jump_here(v, addr_first) };
            unsafe {
                sqlite3_expr_code_move(p_parse_1, reg_base, reg_prev_key,
                    unsafe { (*p_sort_1).n_ob_sat })
            };
            unsafe { sqlite3_vdbe_jump_here(v, addr_jmp) };
        }
        if i_limit != 0 {
            let i_csr: i32 = unsafe { (*p_sort_1).i_e_cursor };
            unsafe {
                sqlite3_vdbe_add_op2(v, 62, i_limit,
                    unsafe { sqlite3_vdbe_current_addr(v) } + 4)
            };
            unsafe { sqlite3_vdbe_add_op2(v, 32, i_csr, 0) };
            i_skip =
                unsafe {
                    sqlite3_vdbe_add_op4_int(v, 41, i_csr, 0,
                        reg_base + n_ob_sat, n_expr - n_ob_sat)
                };
            unsafe { sqlite3_vdbe_add_op1(v, 132, i_csr) };
        }
        if reg_record == 0 {
            reg_record =
                make_sorter_record(p_parse_1, unsafe { &*p_sort_1 },
                    p_select_1, reg_base, n_base);
        }
        if unsafe { (*p_sort_1).sort_flags } as i32 & 1 != 0 {
            op = 141;
        } else { op = 140; }
        unsafe {
            sqlite3_vdbe_add_op4_int(v, op, unsafe { (*p_sort_1).i_e_cursor },
                reg_record, reg_base + n_ob_sat, n_base - n_ob_sat)
        };
        if i_skip != 0 {
            unsafe {
                sqlite3_vdbe_change_p2(v, i_skip,
                    if unsafe { (*p_sort_1).label_ob_lopt } != 0 {
                        unsafe { (*p_sort_1).label_ob_lopt }
                    } else { unsafe { sqlite3_vdbe_current_addr(v) } })
            };
        }
    }
}
extern "C" fn select_inner_loop(p_parse_1: *mut Parse, p: *mut Select,
    src_tab_1: i32, mut p_sort_1: *mut SortCtx,
    p_distinct_1: *const DistinctCtx, p_dest_1: &mut SelectDest,
    i_continue_1: i32, i_break_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut has_distinct: i32 = 0;
        let e_dest: i32 = (*p_dest_1).e_dest as i32;
        let i_parm: i32 = (*p_dest_1).i_sd_parm;
        let mut n_result_col: i32 = 0;
        let mut n_prefix_reg: i32 = 0;
        let mut s_row_load_info: RowLoadInfo = unsafe { core::mem::zeroed() };
        let mut reg_result: i32 = 0;
        let mut reg_orig: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        has_distinct =
            if !(p_distinct_1).is_null() {
                (unsafe { (*p_distinct_1).e_tnct_type }) as i32
            } else { 0 };
        if !(p_sort_1).is_null() &&
                unsafe { (*p_sort_1).p_order_by } == core::ptr::null_mut() {
            p_sort_1 = core::ptr::null_mut();
        }
        if p_sort_1 == core::ptr::null_mut() &&
                (has_distinct == 0) as i32 != 0 {
            { let _ = 0; };
            code_offset(v, unsafe { (*p).i_offset }, i_continue_1);
        }
        n_result_col = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
        if (*p_dest_1).i_sdst == 0 {
            if !(p_sort_1).is_null() {
                n_prefix_reg =
                    unsafe { (*unsafe { (*p_sort_1).p_order_by }).n_expr };
                if (unsafe { (*p_sort_1).sort_flags } as i32 & 1 == 0) as i32
                        != 0 {
                    {
                        let __p = &mut n_prefix_reg;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
                unsafe { (*p_parse_1).n_mem += n_prefix_reg };
            }
            (*p_dest_1).i_sdst = unsafe { (*p_parse_1).n_mem } + 1;
            unsafe { (*p_parse_1).n_mem += n_result_col };
        } else if (*p_dest_1).i_sdst + n_result_col >
                unsafe { (*p_parse_1).n_mem } {
            unsafe { (*p_parse_1).n_mem += n_result_col };
        }
        (*p_dest_1).n_sdst = n_result_col;
        reg_orig = { reg_result = (*p_dest_1).i_sdst; reg_result };
        if src_tab_1 >= 0 {
            {
                i = 0;
                '__b66: loop {
                    if !(i < n_result_col) { break '__b66; }
                    '__c66: loop {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, src_tab_1, i, reg_result + i)
                        };
                        break '__c66;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else if e_dest != 1 {
            let mut ecel_flags: u8 = 0 as u8;
            let mut p_e_list: *const ExprList = core::ptr::null();
            if e_dest == 8 || e_dest == 7 || e_dest == 11 {
                ecel_flags = 1 as u8;
            } else { ecel_flags = 0 as u8; }
            if !(p_sort_1).is_null() && has_distinct == 0 && e_dest != 10 &&
                    e_dest != 12 {
                ecel_flags |= (8 | 4) as u8;
                {
                    i = unsafe { (*p_sort_1).n_ob_sat };
                    '__b67: loop {
                        if !(i <
                                        unsafe { (*unsafe { (*p_sort_1).p_order_by }).n_expr }) {
                            break '__b67;
                        }
                        '__c67: loop {
                            let mut j: i32 = 0;
                            if {
                                        j =
                                            unsafe {
                                                    (*(unsafe {
                                                                                (*unsafe { (*p_sort_1).p_order_by }).a.as_ptr()
                                                                            } as
                                                                            *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                } as i32;
                                        j
                                    } > 0 {
                                unsafe {
                                    (*(unsafe { (*unsafe { (*p).p_e_list }).a.as_ptr() } as
                                                                *mut ExprList_item).offset((j - 1) as
                                                                isize)).u.x.i_order_by_col =
                                        (i + 1 - unsafe { (*p_sort_1).n_ob_sat }) as u16
                                };
                            }
                            break '__c67;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                p_e_list = unsafe { (*p).p_e_list };
                {
                    i = 0;
                    '__b68: loop {
                        if !(i < unsafe { (*p_e_list).n_expr }) { break '__b68; }
                        '__c68: loop {
                            if unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                        } as i32 > 0 {
                                {
                                    let __p = &mut n_result_col;
                                    let __t = *__p;
                                    *__p -= 1;
                                    __t
                                };
                                reg_orig = 0;
                            }
                            break '__c68;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                { let _ = 0; };
            }
            s_row_load_info.reg_result = reg_result;
            s_row_load_info.ecel_flags = ecel_flags;
            if unsafe { (*p).i_limit } != 0 && ecel_flags as i32 & 8 != 0 &&
                    n_prefix_reg > 0 {
                { let _ = 0; };
                { let _ = 0; };
                unsafe {
                    (*p_sort_1).p_deferred_row_load = &mut s_row_load_info
                };
                reg_orig = 0;
            } else {
                inner_loop_load_row(p_parse_1, unsafe { &*p },
                    &s_row_load_info);
            }
        }
        if has_distinct != 0 {
            let e_type: i32 = unsafe { (*p_distinct_1).e_tnct_type } as i32;
            let mut i_tab: i32 = unsafe { (*p_distinct_1).tab_tnct };
            { let _ = 0; };
            i_tab =
                code_distinct(p_parse_1, e_type, i_tab, i_continue_1,
                    unsafe { &*unsafe { (*p).p_e_list } }, reg_result);
            fix_distinct_open_eph(unsafe { &*p_parse_1 }, e_type, i_tab,
                unsafe { (*p_distinct_1).addr_tnct });
            if p_sort_1 == core::ptr::null_mut() {
                code_offset(v, unsafe { (*p).i_offset }, i_continue_1);
            }
        }
        '__s69:
            {
            match e_dest {
                6 => {
                    {
                        let mut r1: i32 =
                            unsafe {
                                sqlite3_get_temp_range(p_parse_1, n_prefix_reg + 1)
                            };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col,
                                r1 + n_prefix_reg)
                        };
                        if e_dest == 3 {
                            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) } + 4;
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, addr, r1, 0)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1, reg_result,
                                    n_result_col)
                            };
                            { let _ = 0; };
                        }
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, r1 + n_prefix_reg,
                                reg_orig, 1, n_prefix_reg);
                        } else {
                            let mut r2: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                            unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                            unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        }
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r1, n_prefix_reg + 1)
                        };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                3 => {
                    {
                        let mut r1: i32 =
                            unsafe {
                                sqlite3_get_temp_range(p_parse_1, n_prefix_reg + 1)
                            };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col,
                                r1 + n_prefix_reg)
                        };
                        if e_dest == 3 {
                            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) } + 4;
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, addr, r1, 0)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1, reg_result,
                                    n_result_col)
                            };
                            { let _ = 0; };
                        }
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, r1 + n_prefix_reg,
                                reg_orig, 1, n_prefix_reg);
                        } else {
                            let mut r2: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                            unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                            unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        }
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r1, n_prefix_reg + 1)
                        };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                12 => {
                    {
                        let mut r1: i32 =
                            unsafe {
                                sqlite3_get_temp_range(p_parse_1, n_prefix_reg + 1)
                            };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col,
                                r1 + n_prefix_reg)
                        };
                        if e_dest == 3 {
                            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) } + 4;
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, addr, r1, 0)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1, reg_result,
                                    n_result_col)
                            };
                            { let _ = 0; };
                        }
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, r1 + n_prefix_reg,
                                reg_orig, 1, n_prefix_reg);
                        } else {
                            let mut r2: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                            unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                            unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        }
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r1, n_prefix_reg + 1)
                        };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                10 => {
                    {
                        let mut r1: i32 =
                            unsafe {
                                sqlite3_get_temp_range(p_parse_1, n_prefix_reg + 1)
                            };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col,
                                r1 + n_prefix_reg)
                        };
                        if e_dest == 3 {
                            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) } + 4;
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, addr, r1, 0)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1, reg_result,
                                    n_result_col)
                            };
                            { let _ = 0; };
                        }
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, r1 + n_prefix_reg,
                                reg_orig, 1, n_prefix_reg);
                        } else {
                            let mut r2: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                            unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                            unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        }
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r1, n_prefix_reg + 1)
                        };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                13 => {
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                9 => {
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                1 => {
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                8 => {
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                11 => {
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                7 => {
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                4 => {
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                5 => {
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {
                            addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                _ => { { { let _ = 0; }; break '__s69; } }
            }
        }
        if p_sort_1 == core::ptr::null_mut() && unsafe { (*p).i_limit } != 0 {
            unsafe {
                sqlite3_vdbe_add_op2(v, 63, unsafe { (*p).i_limit },
                    i_break_1)
            };
        }
    }
}
extern "C" fn multi_select_values(p_parse_1: *mut Parse, mut p: *mut Select,
    p_dest_1: *mut SelectDest) -> i32 {
    unsafe {
        let mut n_row: i32 = 1;
        let rc: i32 = 0;
        let b_show_all: i32 =
            (unsafe { (*p).p_limit } == core::ptr::null_mut()) as i32;
        { let _ = 0; };
        '__b71: loop {
            '__c71: loop {
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                if !(unsafe { (*p).p_win }).is_null() { return -1; }
                if unsafe { (*p).p_prior } == core::ptr::null_mut() {
                    break '__b71;
                }
                { let _ = 0; };
                p = unsafe { (*p).p_prior };
                n_row += b_show_all;
                break '__c71;
            }
        }
        unsafe {
            sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                c"SCAN %d CONSTANT ROW%s".as_ptr() as *mut i8 as *const i8,
                n_row,
                if n_row == 1 {
                    c"".as_ptr() as *mut i8
                } else { c"S".as_ptr() as *mut i8 })
        };
        while !(p).is_null() {
            select_inner_loop(p_parse_1, p, -1, core::ptr::null_mut(),
                core::ptr::null(), unsafe { &mut *p_dest_1 }, 1, 1);
            if (b_show_all == 0) as i32 != 0 { break; }
            unsafe { (*p).n_select_row = n_row as LogEst };
            p = unsafe { (*p).p_next };
        }
        return rc;
    }
}
extern "C" fn has_anchor(mut p: *const Select) -> i32 {
    while !(p).is_null() &&
            unsafe { (*p).sel_flags } & 8192 as u32 != 0 as u32 {
        p = unsafe { (*p).p_prior };
    }
    return (p != core::ptr::null_mut()) as i32;
}
extern "C" fn compute_limit_registers(p_parse_1: *mut Parse, p: &mut Select,
    i_break_1: i32) -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    let mut i_limit: i32 = 0;
    let mut i_offset: i32 = 0;
    let mut n: i32 = 0;
    let p_limit: *const Expr = (*p).p_limit as *const Expr;
    if (*p).i_limit != 0 { return; }
    if !(p_limit).is_null() {
        { let _ = 0; };
        { let _ = 0; };
        (*p).i_limit =
            {
                i_limit =
                    {
                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                        *__p += 1;
                        *__p
                    };
                i_limit
            };
        v = sqlite3_get_vdbe(p_parse_1);
        { let _ = 0; };
        if unsafe {
                    sqlite3_expr_is_integer(unsafe { (*p_limit).p_left } as
                            *const Expr, &mut n, p_parse_1)
                } != 0 {
            unsafe { sqlite3_vdbe_add_op2(v, 73, n, i_limit) };
            if n == 0 {
                unsafe { sqlite3_vdbe_goto(v, i_break_1) };
            } else if n >= 0 &&
                    (*p).n_select_row as i32 >
                        unsafe { sqlite3_log_est(n as u64) } as i32 {
                (*p).n_select_row = unsafe { sqlite3_log_est(n as u64) };
                (*p).sel_flags |= 16384 as u32;
            }
        } else {
            unsafe {
                sqlite3_expr_code(p_parse_1, unsafe { (*p_limit).p_left },
                    i_limit)
            };
            unsafe { sqlite3_vdbe_add_op1(v, 13, i_limit) };
            unsafe { sqlite3_vdbe_add_op2(v, 17, i_limit, i_break_1) };
        }
        if !(unsafe { (*p_limit).p_right }).is_null() {
            (*p).i_offset =
                {
                    i_offset =
                        {
                            let __p = unsafe { &mut (*p_parse_1).n_mem };
                            *__p += 1;
                            *__p
                        };
                    i_offset
                };
            {
                let __p = unsafe { &mut (*p_parse_1).n_mem };
                let __t = *__p;
                *__p += 1;
                __t
            };
            unsafe {
                sqlite3_expr_code(p_parse_1, unsafe { (*p_limit).p_right },
                    i_offset)
            };
            unsafe { sqlite3_vdbe_add_op1(v, 13, i_offset) };
            unsafe {
                sqlite3_vdbe_add_op3(v, 162, i_limit, i_offset + 1, i_offset)
            };
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_dest_init(p_dest: &mut SelectDest,
    e_dest: i32, i_parm: i32) -> () {
    (*p_dest).e_dest = e_dest as u8;
    (*p_dest).i_sd_parm = i_parm;
    (*p_dest).i_sd_parm2 = 0;
    (*p_dest).z_aff_sdst = core::ptr::null_mut();
    (*p_dest).i_sdst = 0;
    (*p_dest).n_sdst = 0;
}
extern "C" fn multi_select_coll_seq(p_parse_1: *mut Parse, p: &Select,
    i_col_1: i32) -> *mut CollSeq {
    unsafe {
        let mut p_ret: *mut CollSeq = core::ptr::null_mut();
        if !((*p).p_prior).is_null() {
            p_ret =
                multi_select_coll_seq(p_parse_1, unsafe { &*(*p).p_prior },
                    i_col_1);
        } else { p_ret = core::ptr::null_mut(); }
        { let _ = 0; };
        if p_ret == core::ptr::null_mut() &&
                i_col_1 < unsafe { (*(*p).p_e_list).n_expr } {
            p_ret =
                unsafe {
                    sqlite3_expr_coll_seq(p_parse_1,
                        unsafe {
                                (*(unsafe { (*(*p).p_e_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(i_col_1 as isize)).p_expr
                            } as *const Expr)
                };
        }
        return p_ret;
    }
}
extern "C" fn multi_select_by_merge_key_info(p_parse_1: *mut Parse,
    p: *mut Select, n_extra_1: i32) -> *mut KeyInfo {
    unsafe {
        let p_order_by: *mut ExprList = unsafe { (*p).p_order_by };
        let n_order_by: i32 =
            if p_order_by != core::ptr::null_mut() {
                unsafe { (*p_order_by).n_expr }
            } else { 0 };
        let db: *mut sqlite3 = unsafe { (*p_parse_1).db };
        let p_ret: *mut KeyInfo =
            sqlite3_key_info_alloc(db, n_order_by + n_extra_1, 1);
        if !(p_ret).is_null() {
            let mut i: i32 = 0;
            {
                i = 0;
                '__b74: loop {
                    if !(i < n_order_by) { break '__b74; }
                    '__c74: loop {
                        let p_item: *const ExprList_item =
                            unsafe {
                                    &raw mut *(unsafe { (*p_order_by).a.as_ptr() } as
                                                    *mut ExprList_item).offset(i as isize)
                                } as *const ExprList_item;
                        let p_term: *mut Expr = unsafe { (*p_item).p_expr };
                        let mut p_coll: *mut CollSeq = core::ptr::null_mut();
                        if unsafe { (*p_term).flags } & 512 as u32 != 0 {
                            p_coll =
                                unsafe {
                                    sqlite3_expr_coll_seq(p_parse_1, p_term as *const Expr)
                                };
                        } else {
                            p_coll =
                                multi_select_coll_seq(p_parse_1, unsafe { &*p },
                                    unsafe { (*p_item).u.x.i_order_by_col } as i32 - 1);
                            if p_coll == core::ptr::null_mut() {
                                p_coll = unsafe { (*db).p_dflt_coll };
                            }
                            unsafe {
                                (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                    *mut ExprList_item).offset(i as isize)).p_expr =
                                    unsafe {
                                        sqlite3_expr_add_collate_string(p_parse_1 as *const Parse,
                                            p_term, unsafe { (*p_coll).z_name } as *const i8)
                                    }
                            };
                        }
                        { let _ = 0; };
                        unsafe {
                            *(unsafe { (*p_ret).a_coll.as_ptr() } as
                                            *mut *mut CollSeq).offset(i as isize) = p_coll
                        };
                        unsafe {
                            *unsafe { (*p_ret).a_sort_flags.offset(i as isize) } =
                                unsafe {
                                    (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                        *mut ExprList_item).offset(i as isize)).fg.sort_flags
                                }
                        };
                        break '__c74;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return p_ret;
    }
}
extern "C" fn generate_with_recursive_query(p_parse_1: *mut Parse,
    p: *mut Select, p_dest_1: *mut SelectDest) -> () {
    unsafe {
        let mut p_src: *const SrcList = core::ptr::null();
        let mut n_col: i32 = 0;
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let mut p_setup: *mut Select = core::ptr::null_mut();
        let mut p_first_rec: *mut Select = core::ptr::null_mut();
        let mut addr_top: i32 = 0;
        let mut addr_cont: i32 = 0;
        let mut addr_break: i32 = 0;
        let mut i_current: i32 = 0;
        let mut reg_current: i32 = 0;
        let mut i_queue: i32 = 0;
        let mut i_distinct: i32 = 0;
        let mut e_dest: i32 = 0;
        let mut dest_queue: SelectDest = unsafe { core::mem::zeroed() };
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        let mut p_order_by: *mut ExprList = core::ptr::null_mut();
        let mut p_limit: *mut Expr = core::ptr::null_mut();
        let mut reg_limit: i32 = 0;
        let mut reg_offset: i32 = 0;
        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
        let mut p_key_info_1: *mut KeyInfo = core::ptr::null_mut();
        let mut ap_coll: *mut *mut CollSeq = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s76:
                {
                match __state {
                    0 => {
                        p_src = unsafe { (*p).p_src } as *const SrcList;
                        __state = 3;
                    }
                    2 => {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse_1).db },
                                unsafe { (*p).p_order_by })
                        };
                        __state = 109;
                    }
                    3 => {
                        n_col = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
                        __state = 4;
                    }
                    4 => { v = unsafe { (*p_parse_1).p_vdbe }; __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { i_current = 0; __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { i_distinct = 0; __state = 13; }
                    13 => { e_dest = 6; __state = 14; }
                    14 => { __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => { __state = 18; }
                    18 => { __state = 19; }
                    19 => { __state = 20; }
                    20 => {
                        if !(unsafe { (*p).p_win }).is_null() {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => {
                        if unsafe {
                                    sqlite3_auth_check(p_parse_1, 33, core::ptr::null(),
                                        core::ptr::null(), core::ptr::null())
                                } != 0 {
                            __state = 25;
                        } else { __state = 24; }
                    }
                    22 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"cannot use window functions in recursive queries".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 23;
                    }
                    23 => { return; }
                    24 => {
                        addr_break = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        __state = 26;
                    }
                    25 => { return; }
                    26 => {
                        unsafe { (*p).n_select_row = 320 as LogEst };
                        __state = 27;
                    }
                    27 => {
                        compute_limit_registers(p_parse_1, unsafe { &mut *p },
                            addr_break);
                        __state = 28;
                    }
                    28 => { p_limit = unsafe { (*p).p_limit }; __state = 29; }
                    29 => { reg_limit = unsafe { (*p).i_limit }; __state = 30; }
                    30 => {
                        reg_offset = unsafe { (*p).i_offset };
                        __state = 31;
                    }
                    31 => {
                        unsafe { (*p).p_limit = core::ptr::null_mut() };
                        __state = 32;
                    }
                    32 => {
                        unsafe {
                            (*p).i_limit =
                                { unsafe { (*p).i_offset = 0 }; unsafe { (*p).i_offset } }
                        };
                        __state = 33;
                    }
                    33 => {
                        p_order_by = unsafe { (*p).p_order_by };
                        __state = 34;
                    }
                    34 => { i = 0; __state = 36; }
                    35 => {
                        i_queue =
                            {
                                let __p = unsafe { &mut (*p_parse_1).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 41;
                    }
                    36 => {
                        if i < unsafe { (*p_src).n_src } {
                            __state = 37;
                        } else { __state = 35; }
                    }
                    37 => {
                        if unsafe {
                                    (*(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset(i as isize)).fg.is_recursive()
                                } != 0 {
                            __state = 39;
                        } else { __state = 38; }
                    }
                    38 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 36;
                    }
                    39 => {
                        i_current =
                            unsafe {
                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)).i_cursor
                            };
                        __state = 40;
                    }
                    40 => { __state = 35; }
                    41 => {
                        if unsafe { (*p).op } as i32 == 135 {
                            __state = 43;
                        } else { __state = 44; }
                    }
                    42 => {
                        sqlite3_select_dest_init(&mut dest_queue, e_dest, i_queue);
                        __state = 46;
                    }
                    43 => {
                        e_dest = if !(p_order_by).is_null() { 4 } else { 3 };
                        __state = 45;
                    }
                    44 => {
                        e_dest = if !(p_order_by).is_null() { 5 } else { 6 };
                        __state = 42;
                    }
                    45 => {
                        i_distinct =
                            {
                                let __p = unsafe { &mut (*p_parse_1).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 42;
                    }
                    46 => {
                        reg_current =
                            {
                                let __p = unsafe { &mut (*p_parse_1).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 47;
                    }
                    47 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 123, i_current, reg_current, n_col)
                        };
                        __state = 48;
                    }
                    48 => {
                        if !(p_order_by).is_null() {
                            __state = 50;
                        } else { __state = 51; }
                    }
                    49 => { __state = 54; }
                    50 => {
                        p_key_info =
                            multi_select_by_merge_key_info(p_parse_1, p, 1);
                        __state = 52;
                    }
                    51 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 120, i_queue, n_col) };
                        __state = 49;
                    }
                    52 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 120, i_queue,
                                unsafe { (*p_order_by).n_expr } + 2, 0,
                                p_key_info as *mut i8 as *const i8, -9)
                        };
                        __state = 53;
                    }
                    53 => { dest_queue.p_order_by = p_order_by; __state = 49; }
                    54 => {
                        if i_distinct != 0 { __state = 56; } else { __state = 55; }
                    }
                    55 => {
                        unsafe { (*p).p_order_by = core::ptr::null_mut() };
                        __state = 71;
                    }
                    56 => { __state = 57; }
                    57 => { __state = 58; }
                    58 => { { let _ = 0; }; __state = 59; }
                    59 => { { let _ = 0; }; __state = 60; }
                    60 => {
                        n_col = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
                        __state = 61;
                    }
                    61 => {
                        p_key_info_1 =
                            sqlite3_key_info_alloc(unsafe { (*p_parse_1).db }, n_col,
                                1);
                        __state = 62;
                    }
                    62 => {
                        if !(p_key_info_1).is_null() {
                            __state = 63;
                        } else { __state = 64; }
                    }
                    63 => {
                        {
                            i = 0;
                            ap_coll =
                                unsafe { (*p_key_info_1).a_coll.as_ptr() } as
                                    *mut *mut CollSeq
                        };
                        __state = 66;
                    }
                    64 => { { let _ = 0; }; __state = 55; }
                    65 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 120, i_distinct, n_col, 0,
                                p_key_info_1 as *mut () as *const i8, -9)
                        };
                        __state = 55;
                    }
                    66 => {
                        if i < n_col { __state = 67; } else { __state = 65; }
                    }
                    67 => {
                        unsafe {
                            *ap_coll =
                                multi_select_coll_seq(p_parse_1, unsafe { &*p }, i)
                        };
                        __state = 69;
                    }
                    68 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut ap_coll;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 66;
                    }
                    69 => {
                        if core::ptr::null_mut() == unsafe { *ap_coll } {
                            __state = 70;
                        } else { __state = 68; }
                    }
                    70 => {
                        unsafe {
                            *ap_coll =
                                unsafe { (*unsafe { (*p_parse_1).db }).p_dflt_coll }
                        };
                        __state = 68;
                    }
                    71 => { p_first_rec = p; __state = 73; }
                    72 => {
                        p_setup = unsafe { (*p_first_rec).p_prior };
                        __state = 81;
                    }
                    73 => {
                        if p_first_rec != core::ptr::null_mut() {
                            __state = 74;
                        } else { __state = 72; }
                    }
                    74 => {
                        if unsafe { (*p_first_rec).sel_flags } & 8 as u32 != 0 {
                            __state = 77;
                        } else { __state = 76; }
                    }
                    75 => {
                        p_first_rec = unsafe { (*p_first_rec).p_prior };
                        __state = 73;
                    }
                    76 => {
                        unsafe { (*p_first_rec).op = 136 as u8 };
                        __state = 79;
                    }
                    77 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"recursive aggregate queries not supported".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 78;
                    }
                    78 => { __state = 2; }
                    79 => {
                        if unsafe { (*unsafe { (*p_first_rec).p_prior }).sel_flags }
                                    & 8192 as u32 == 0 as u32 {
                            __state = 80;
                        } else { __state = 75; }
                    }
                    80 => { __state = 72; }
                    81 => {
                        unsafe { (*p_setup).p_next = core::ptr::null_mut() };
                        __state = 82;
                    }
                    82 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                                c"SETUP".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 83;
                    }
                    83 => {
                        rc = sqlite3_select(p_parse_1, p_setup, &mut dest_queue);
                        __state = 84;
                    }
                    84 => { unsafe { (*p_setup).p_next = p }; __state = 85; }
                    85 => {
                        if rc != 0 { __state = 87; } else { __state = 86; }
                    }
                    86 => {
                        addr_top =
                            unsafe { sqlite3_vdbe_add_op2(v, 36, i_queue, addr_break) };
                        __state = 88;
                    }
                    87 => { __state = 2; }
                    88 => { __state = 89; }
                    89 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 138, i_current) };
                        __state = 90;
                    }
                    90 => {
                        if !(p_order_by).is_null() {
                            __state = 92;
                        } else { __state = 93; }
                    }
                    91 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 132, i_queue) };
                        __state = 94;
                    }
                    92 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_queue,
                                unsafe { (*p_order_by).n_expr } + 1, reg_current)
                        };
                        __state = 91;
                    }
                    93 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 136, i_queue, reg_current)
                        };
                        __state = 91;
                    }
                    94 => {
                        addr_cont = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        __state = 95;
                    }
                    95 => {
                        code_offset(v, reg_offset, addr_cont);
                        __state = 96;
                    }
                    96 => {
                        select_inner_loop(p_parse_1, p, i_current,
                            core::ptr::null_mut(), core::ptr::null(),
                            unsafe { &mut *p_dest_1 }, addr_cont, addr_break);
                        __state = 97;
                    }
                    97 => {
                        if reg_limit != 0 { __state = 99; } else { __state = 98; }
                    }
                    98 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_cont) };
                        __state = 101;
                    }
                    99 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 63, reg_limit, addr_break)
                        };
                        __state = 100;
                    }
                    100 => { __state = 98; }
                    101 => {
                        unsafe { (*p_first_rec).p_prior = core::ptr::null_mut() };
                        __state = 102;
                    }
                    102 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                                c"RECURSIVE STEP".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 103;
                    }
                    103 => {
                        sqlite3_select(p_parse_1, p, &mut dest_queue);
                        __state = 104;
                    }
                    104 => { { let _ = 0; }; __state = 105; }
                    105 => {
                        unsafe { (*p_first_rec).p_prior = p_setup };
                        __state = 106;
                    }
                    106 => {
                        unsafe { sqlite3_vdbe_goto(v, addr_top) };
                        __state = 107;
                    }
                    107 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_break) };
                        __state = 108;
                    }
                    108 => { __state = 2; }
                    109 => {
                        unsafe { (*p).p_order_by = p_order_by };
                        __state = 110;
                    }
                    110 => { unsafe { (*p).p_limit = p_limit }; __state = 111; }
                    111 => { return; }
                    _ => {}
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_op_name(id: i32) -> *const i8 {
    let mut z: *mut i8 = core::ptr::null_mut();
    '__s77:
        {
        match id {
            136 => { z = c"UNION ALL".as_ptr() as *mut i8; }
            138 => { z = c"INTERSECT".as_ptr() as *mut i8; }
            137 => { z = c"EXCEPT".as_ptr() as *mut i8; }
            _ => { z = c"UNION".as_ptr() as *mut i8; }
        }
    }
    return z as *const i8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_key_info_ref(p: *mut KeyInfo) -> *mut KeyInfo {
    if !(p).is_null() {
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p).n_ref };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    return p;
}
extern "C" fn generate_output_subroutine(p_parse_1: *mut Parse, p: &Select,
    p_in_1: &SelectDest, p_dest_1: &mut SelectDest, reg_return_1: i32,
    reg_prev_1: i32, p_key_info_1: *mut KeyInfo, i_break_1: i32) -> i32 {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i_continue: i32 = 0;
        let mut addr: i32 = 0;
        { let _ = 0; };
        addr = unsafe { sqlite3_vdbe_current_addr(v) };
        i_continue = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
        if reg_prev_1 != 0 {
            let mut addr1: i32 = 0;
            let mut addr2: i32 = 0;
            addr1 = unsafe { sqlite3_vdbe_add_op1(v, 17, reg_prev_1) };
            addr2 =
                unsafe {
                    sqlite3_vdbe_add_op4(v, 92, (*p_in_1).i_sdst,
                        reg_prev_1 + 1, (*p_in_1).n_sdst,
                        sqlite3_key_info_ref(p_key_info_1) as *mut i8 as *const i8,
                        -9)
                };
            unsafe {
                sqlite3_vdbe_add_op3(v, 14, addr2 + 2, i_continue, addr2 + 2)
            };
            unsafe { sqlite3_vdbe_jump_here(v, addr1) };
            unsafe {
                sqlite3_vdbe_add_op3(v, 82, (*p_in_1).i_sdst, reg_prev_1 + 1,
                    (*p_in_1).n_sdst - 1)
            };
            unsafe { sqlite3_vdbe_add_op2(v, 73, 1, reg_prev_1) };
        }
        if unsafe { (*unsafe { (*p_parse_1).db }).malloc_failed } != 0 {
            return 0;
        }
        code_offset(v, (*p).i_offset, i_continue);
        '__s78:
            {
            match (*p_dest_1).e_dest {
                6 => {
                    {
                        let mut r1: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let mut r2: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let i_parm: i32 = (*p_dest_1).i_sd_parm;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1)
                        };
                        if (*p_dest_1).e_dest as i32 == 3 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                        unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                3 => {
                    {
                        let mut r1: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let mut r2: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let i_parm: i32 = (*p_dest_1).i_sd_parm;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1)
                        };
                        if (*p_dest_1).e_dest as i32 == 3 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                        unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                12 => {
                    {
                        let mut r1: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let mut r2: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let i_parm: i32 = (*p_dest_1).i_sd_parm;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1)
                        };
                        if (*p_dest_1).e_dest as i32 == 3 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                        unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                10 => {
                    {
                        let mut r1: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let mut r2: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let i_parm: i32 = (*p_dest_1).i_sd_parm;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1)
                        };
                        if (*p_dest_1).e_dest as i32 == 3 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                        unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                1 => {
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                9 => {
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                8 => {
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                11 => {
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                4 => {
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                5 => {
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprList_item).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                2 => {
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                _ => {
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
            }
        }
        if (*p).i_limit != 0 {
            unsafe { sqlite3_vdbe_add_op2(v, 63, (*p).i_limit, i_break_1) };
        }
        unsafe { sqlite3_vdbe_resolve_label(v, i_continue) };
        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_return_1) };
        return addr;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_key_info_unref(p: *mut KeyInfo) -> () {
    if !(p).is_null() {
        { let _ = 0; };
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p).n_ref };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p).n_ref } == 0 as u32 {
            unsafe {
                sqlite3_db_nn_free_nn(unsafe { (*p).db }, p as *mut ())
            };
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_delete_generic(db: *mut sqlite3, p: *mut ())
    -> () {
    if !(p).is_null() { clear_select(db, p as *mut Select, 1); }
}
extern "C" fn multi_select_by_merge(p_parse: *mut Parse, p: *mut Select,
    p_dest: *mut SelectDest) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut p_prior: *mut Select = core::ptr::null_mut();
        let mut p_split: *mut Select = core::ptr::null_mut();
        let mut n_select: i32 = 0;
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let mut dest_a: SelectDest = unsafe { core::mem::zeroed() };
        let mut dest_b: SelectDest = unsafe { core::mem::zeroed() };
        let mut reg_addr_a: i32 = 0;
        let mut reg_addr_b: i32 = 0;
        let mut addr_select_a: i32 = 0;
        let mut addr_select_b: i32 = 0;
        let mut reg_out_a: i32 = 0;
        let mut reg_out_b: i32 = 0;
        let mut addr_out_a: i32 = 0;
        let mut addr_out_b: i32 = 0;
        let mut addr_eof_a: i32 = 0;
        let mut addr_eof_a_no_b: i32 = 0;
        let mut addr_eof_b: i32 = 0;
        let mut addr_alt_b: i32 = 0;
        let mut addr_aeq_b: i32 = 0;
        let mut addr_agt_b: i32 = 0;
        let mut reg_limit_a: i32 = 0;
        let mut reg_limit_b: i32 = 0;
        let mut reg_prev: i32 = 0;
        let mut saved_limit: i32 = 0;
        let mut saved_offset: i32 = 0;
        let mut label_cmpr: i32 = 0;
        let mut label_end: i32 = 0;
        let mut addr1: i32 = 0;
        let mut op: i32 = 0;
        let mut p_key_dup: *mut KeyInfo = core::ptr::null_mut();
        let mut p_key_merge: *mut KeyInfo = core::ptr::null_mut();
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut p_order_by: *mut ExprList = core::ptr::null_mut();
        let mut n_order_by: i32 = 0;
        let mut a_permute: *mut u32 = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        db = unsafe { (*p_parse).db };
        v = unsafe { (*p_parse).p_vdbe };
        { let _ = 0; };
        label_end = unsafe { sqlite3_vdbe_make_label(p_parse) };
        label_cmpr = unsafe { sqlite3_vdbe_make_label(p_parse) };
        op = unsafe { (*p).op } as i32;
        { let _ = 0; };
        p_order_by = unsafe { (*p).p_order_by };
        { let _ = 0; };
        n_order_by = unsafe { (*p_order_by).n_expr };
        if op != 136 {
            {
                i = 1;
                '__b80: loop {
                    if !(unsafe { (*db).malloc_failed } as i32 == 0 &&
                                    i <= unsafe { (*unsafe { (*p).p_e_list }).n_expr }) {
                        break '__b80;
                    }
                    '__c80: loop {
                        let mut p_item: *const ExprList_item = core::ptr::null();
                        {
                            {
                                j = 0;
                                p_item =
                                    unsafe { (*p_order_by).a.as_ptr() } as *mut ExprList_item
                            };
                            '__b81: loop {
                                if !(j < n_order_by) { break '__b81; }
                                '__c81: loop {
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    if unsafe { (*p_item).u.x.i_order_by_col } as i32 == i {
                                        break '__b81;
                                    }
                                    break '__c81;
                                }
                                {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                    {
                                        let __p = &mut p_item;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                                };
                            }
                        }
                        if j == n_order_by {
                            let p_new: *mut Expr = unsafe { sqlite3_expr_int32(db, i) };
                            if p_new == core::ptr::null_mut() { return 7; }
                            unsafe {
                                (*p).p_order_by =
                                    {
                                        p_order_by =
                                            unsafe {
                                                sqlite3_expr_list_append(p_parse, p_order_by, p_new)
                                            };
                                        p_order_by
                                    }
                            };
                            if !(p_order_by).is_null() {
                                unsafe {
                                    (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                                *mut ExprList_item).offset({
                                                                    let __p = &mut n_order_by;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize)).u.x.i_order_by_col = i as u16
                                };
                            }
                        }
                        break '__c80;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        a_permute =
            unsafe {
                    sqlite3_db_malloc_raw_nn(db,
                        core::mem::size_of::<u32>() as u64 *
                            (n_order_by + 1) as u64)
                } as *mut u32;
        if !(a_permute).is_null() {
            let mut p_item_1: *const ExprList_item = core::ptr::null();
            let mut b_keep: i32 = 0;
            unsafe { *a_permute.offset(0 as isize) = n_order_by as u32 };
            {
                {
                    i = 1;
                    p_item_1 =
                        unsafe { (*p_order_by).a.as_ptr() } as *mut ExprList_item
                };
                '__b82: loop {
                    if !(i <= n_order_by) { break '__b82; }
                    '__c82: loop {
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        unsafe {
                            *a_permute.offset(i as isize) =
                                (unsafe { (*p_item_1).u.x.i_order_by_col } as i32 - 1) as
                                    u32
                        };
                        if unsafe { *a_permute.offset(i as isize) } !=
                                i as u32 - 1 as u32 {
                            b_keep = 1;
                        }
                        break '__c82;
                    }
                    {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        {
                            let __p = &mut p_item_1;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
            if b_keep == 0 {
                unsafe { sqlite3_db_free_nn(db, a_permute as *mut ()) };
                a_permute = core::ptr::null_mut();
            }
        }
        p_key_merge = multi_select_by_merge_key_info(p_parse, p, 1);
        if op == 136 {
            reg_prev = 0;
        } else {
            let n_expr: i32 = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
            { let _ = 0; };
            reg_prev = unsafe { (*p_parse).n_mem } + 1;
            unsafe { (*p_parse).n_mem += n_expr + 1 };
            unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_prev) };
            p_key_dup = sqlite3_key_info_alloc(db, n_expr, 1);
            if !(p_key_dup).is_null() {
                { let _ = 0; };
                {
                    i = 0;
                    '__b83: loop {
                        if !(i < n_expr) { break '__b83; }
                        '__c83: loop {
                            unsafe {
                                *(unsafe { (*p_key_dup).a_coll.as_ptr() } as
                                                *mut *mut CollSeq).offset(i as isize) =
                                    multi_select_coll_seq(p_parse, unsafe { &*p }, i)
                            };
                            unsafe {
                                *unsafe { (*p_key_dup).a_sort_flags.offset(i as isize) } =
                                    0 as u8
                            };
                            break '__c83;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        }
        n_select = 1;
        if (op == 136 || op == 135) &&
                unsafe { (*db).db_opt_flags } & 2097152 as u32 == 0 as u32 {
            {
                p_split = p;
                '__b84: loop {
                    if !(unsafe { (*p_split).p_prior } != core::ptr::null_mut()
                                    && unsafe { (*p_split).op } as i32 == op) {
                        break '__b84;
                    }
                    '__c84: loop {
                        { let __p = &mut n_select; let __t = *__p; *__p += 1; __t };
                        { let _ = 0; };
                        break '__c84;
                    }
                    p_split = unsafe { (*p_split).p_prior };
                }
            }
        }
        if n_select <= 3 {
            p_split = p;
        } else {
            p_split = p;
            {
                i = 2;
                '__b85: loop {
                    if !(i < n_select) { break '__b85; }
                    '__c85: loop {
                        p_split = unsafe { (*p_split).p_prior };
                        break '__c85;
                    }
                    i += 2;
                }
            }
        }
        p_prior = unsafe { (*p_split).p_prior };
        { let _ = 0; };
        unsafe { (*p_split).p_prior = core::ptr::null_mut() };
        unsafe { (*p_prior).p_next = core::ptr::null_mut() };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            (*p_prior).p_order_by =
                unsafe {
                    sqlite3_expr_list_dup(unsafe { (*p_parse).db },
                        p_order_by as *const ExprList, 0)
                }
        };
        unsafe {
            sqlite3_resolve_order_group_by(p_parse, p,
                unsafe { (*p).p_order_by },
                c"ORDER".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            sqlite3_resolve_order_group_by(p_parse, p_prior,
                unsafe { (*p_prior).p_order_by },
                c"ORDER".as_ptr() as *mut i8 as *const i8)
        };
        compute_limit_registers(p_parse, unsafe { &mut *p }, label_end);
        if unsafe { (*p).i_limit } != 0 && op == 136 {
            reg_limit_a =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                };
            reg_limit_b =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                };
            unsafe {
                sqlite3_vdbe_add_op2(v, 82,
                    if unsafe { (*p).i_offset } != 0 {
                        (unsafe { (*p).i_offset }) + 1
                    } else { unsafe { (*p).i_limit } }, reg_limit_a)
            };
            unsafe { sqlite3_vdbe_add_op2(v, 82, reg_limit_a, reg_limit_b) };
        } else { reg_limit_a = { reg_limit_b = 0; reg_limit_b }; }
        unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_limit }) };
        unsafe { (*p).p_limit = core::ptr::null_mut() };
        reg_addr_a =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        reg_addr_b =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        reg_out_a =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        reg_out_b =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        sqlite3_select_dest_init(&mut dest_a, 11, reg_addr_a);
        sqlite3_select_dest_init(&mut dest_b, 11, reg_addr_b);
        unsafe {
            sqlite3_vdbe_explain(p_parse, 1 as u8,
                c"MERGE (%s)".as_ptr() as *mut i8 as *const i8,
                sqlite3_select_op_name(unsafe { (*p).op } as i32))
        };
        addr_select_a = unsafe { sqlite3_vdbe_current_addr(v) } + 1;
        addr1 =
            unsafe {
                sqlite3_vdbe_add_op3(v, 11, reg_addr_a, 0, addr_select_a)
            };
        unsafe { (*p_prior).i_limit = reg_limit_a };
        unsafe {
            sqlite3_vdbe_explain(p_parse, 1 as u8,
                c"LEFT".as_ptr() as *mut i8 as *const i8)
        };
        sqlite3_select(p_parse, p_prior, &mut dest_a);
        unsafe { sqlite3_vdbe_end_coroutine(v, reg_addr_a) };
        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
        addr_select_b = unsafe { sqlite3_vdbe_current_addr(v) } + 1;
        addr1 =
            unsafe {
                sqlite3_vdbe_add_op3(v, 11, reg_addr_b, 0, addr_select_b)
            };
        saved_limit = unsafe { (*p).i_limit };
        saved_offset = unsafe { (*p).i_offset };
        unsafe { (*p).i_limit = reg_limit_b };
        unsafe { (*p).i_offset = 0 };
        unsafe {
            sqlite3_vdbe_explain(p_parse, 1 as u8,
                c"RIGHT".as_ptr() as *mut i8 as *const i8)
        };
        sqlite3_select(p_parse, p, &mut dest_b);
        unsafe { (*p).i_limit = saved_limit };
        unsafe { (*p).i_offset = saved_offset };
        unsafe { sqlite3_vdbe_end_coroutine(v, reg_addr_b) };
        addr_out_a =
            generate_output_subroutine(p_parse, unsafe { &*p }, &dest_a,
                unsafe { &mut *p_dest }, reg_out_a, reg_prev, p_key_dup,
                label_end);
        if op == 136 || op == 135 {
            addr_out_b =
                generate_output_subroutine(p_parse, unsafe { &*p }, &dest_b,
                    unsafe { &mut *p_dest }, reg_out_b, reg_prev, p_key_dup,
                    label_end);
        }
        sqlite3_key_info_unref(p_key_dup);
        if op == 137 || op == 138 {
            addr_eof_a_no_b = { addr_eof_a = label_end; addr_eof_a };
        } else {
            addr_eof_a =
                unsafe { sqlite3_vdbe_add_op2(v, 10, reg_out_b, addr_out_b) };
            addr_eof_a_no_b =
                unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_b, label_end) };
            unsafe { sqlite3_vdbe_goto(v, addr_eof_a) };
            unsafe {
                (*p).n_select_row =
                    unsafe {
                        sqlite3_log_est_add(unsafe { (*p).n_select_row },
                            unsafe { (*p_prior).n_select_row })
                    }
            };
        }
        if op == 138 {
            addr_eof_b = addr_eof_a;
            if unsafe { (*p).n_select_row } as i32 >
                    unsafe { (*p_prior).n_select_row } as i32 {
                unsafe {
                    (*p).n_select_row = unsafe { (*p_prior).n_select_row }
                };
            }
        } else {
            addr_eof_b =
                unsafe { sqlite3_vdbe_add_op2(v, 10, reg_out_a, addr_out_a) };
            unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_a, label_end) };
            unsafe { sqlite3_vdbe_goto(v, addr_eof_b) };
        }
        addr_alt_b =
            unsafe { sqlite3_vdbe_add_op2(v, 10, reg_out_a, addr_out_a) };
        unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_a, addr_eof_a) };
        unsafe { sqlite3_vdbe_goto(v, label_cmpr) };
        if op == 136 {
            addr_aeq_b = addr_alt_b;
        } else if op == 138 {
            addr_aeq_b = addr_alt_b;
            { let __p = &mut addr_alt_b; let __t = *__p; *__p += 1; __t };
        } else { addr_aeq_b = addr_alt_b + 1; }
        addr_agt_b = unsafe { sqlite3_vdbe_current_addr(v) };
        if op == 136 || op == 135 {
            unsafe { sqlite3_vdbe_add_op2(v, 10, reg_out_b, addr_out_b) };
            unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_b, addr_eof_b) };
            unsafe { sqlite3_vdbe_goto(v, label_cmpr) };
        } else {
            { let __p = &mut addr_agt_b; let __t = *__p; *__p += 1; __t };
        }
        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
        unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_a, addr_eof_a_no_b) };
        unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_b, addr_eof_b) };
        if a_permute != core::ptr::null_mut() {
            unsafe {
                sqlite3_vdbe_add_op4(v, 91, 0, 0, 0,
                    a_permute as *mut i8 as *const i8, -15)
            };
        }
        unsafe { sqlite3_vdbe_resolve_label(v, label_cmpr) };
        unsafe {
            sqlite3_vdbe_add_op4(v, 92, dest_a.i_sdst, dest_b.i_sdst,
                n_order_by, p_key_merge as *mut i8 as *const i8, -9)
        };
        if a_permute != core::ptr::null_mut() {
            unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
        }
        unsafe {
            sqlite3_vdbe_add_op3(v, 14, addr_alt_b, addr_aeq_b, addr_agt_b)
        };
        unsafe { sqlite3_vdbe_resolve_label(v, label_end) };
        if !(unsafe { (*p_split).p_prior }).is_null() {
            unsafe {
                sqlite3_parser_add_cleanup(p_parse,
                    Some(sqlite3_select_delete_generic),
                    unsafe { (*p_split).p_prior } as *mut ())
            };
        }
        unsafe { (*p_split).p_prior = p_prior };
        unsafe { (*p_prior).p_next = p_split };
        unsafe {
            sqlite3_expr_list_delete(db, unsafe { (*p_prior).p_order_by })
        };
        unsafe { (*p_prior).p_order_by = core::ptr::null_mut() };
        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
        return (unsafe { (*p_parse).n_err } != 0) as i32;
    }
}
extern "C" fn multi_select(p_parse_1: *mut Parse, p: *mut Select,
    p_dest_1: *mut SelectDest) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut dest: SelectDest = unsafe { core::mem::zeroed() };
        let mut p_delete: *mut Select = core::ptr::null_mut();
        '__b86: loop {
            '__c86: loop {
                let mut p_prior: *mut Select = core::ptr::null_mut();
                let mut v: *mut Vdbe = core::ptr::null_mut();
                let mut db: *mut sqlite3 = core::ptr::null_mut();
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                db = unsafe { (*p_parse_1).db };
                p_prior = unsafe { (*p).p_prior };
                dest = unsafe { core::ptr::read(p_dest_1) };
                { let _ = 0; };
                { let _ = 0; };
                v = sqlite3_get_vdbe(p_parse_1);
                { let _ = 0; };
                if dest.e_dest as i32 == 10 {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 120, dest.i_sd_parm,
                            unsafe { (*unsafe { (*p).p_e_list }).n_expr })
                    };
                    dest.e_dest = 12 as u8;
                }
                if unsafe { (*p).sel_flags } & 1024 as u32 != 0 {
                    rc = multi_select_values(p_parse_1, p, &mut dest);
                    if rc >= 0 { break '__b86; }
                    rc = 0;
                }
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p).sel_flags } & 8192 as u32 != 0 as u32 &&
                        has_anchor(p as *const Select) != 0 {
                    generate_with_recursive_query(p_parse_1, p, &mut dest);
                } else if !(unsafe { (*p).p_order_by }).is_null() {
                    return multi_select_by_merge(p_parse_1, p, p_dest_1);
                } else if unsafe { (*p).op } as i32 != 136 {
                    let p_one: *mut Expr = unsafe { sqlite3_expr_int32(db, 1) };
                    unsafe {
                        (*p).p_order_by =
                            unsafe {
                                sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                    p_one)
                            }
                    };
                    if unsafe { (*p_parse_1).n_err } != 0 { break '__b86; }
                    { let _ = 0; };
                    unsafe {
                        (*(unsafe { (*unsafe { (*p).p_order_by }).a.as_ptr() } as
                                                    *mut ExprList_item).offset(0 as isize)).u.x.i_order_by_col =
                            1 as u16
                    };
                    return multi_select_by_merge(p_parse_1, p, p_dest_1);
                } else {
                    let mut addr: i32 = 0;
                    let mut n_limit: i32 = 0;
                    if unsafe { (*p_prior).p_prior } == core::ptr::null_mut() {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                                c"COMPOUND QUERY".as_ptr() as *mut i8 as *const i8)
                        };
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                                c"LEFT-MOST SUBQUERY".as_ptr() as *mut i8 as *const i8)
                        };
                    }
                    { let _ = 0; };
                    unsafe { (*p_prior).i_limit = unsafe { (*p).i_limit } };
                    unsafe { (*p_prior).i_offset = unsafe { (*p).i_offset } };
                    unsafe {
                        (*p_prior).p_limit =
                            unsafe {
                                sqlite3_expr_dup(db, unsafe { (*p).p_limit } as *const Expr,
                                    0)
                            }
                    };
                    rc = sqlite3_select(p_parse_1, p_prior, &mut dest);
                    unsafe {
                        sqlite3_expr_delete(db, unsafe { (*p_prior).p_limit })
                    };
                    unsafe { (*p_prior).p_limit = core::ptr::null_mut() };
                    if rc != 0 { break '__b86; }
                    unsafe { (*p).p_prior = core::ptr::null_mut() };
                    unsafe { (*p).i_limit = unsafe { (*p_prior).i_limit } };
                    unsafe { (*p).i_offset = unsafe { (*p_prior).i_offset } };
                    if unsafe { (*p).i_limit } != 0 {
                        addr =
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 17, unsafe { (*p).i_limit })
                            };
                        if unsafe { (*p).i_offset } != 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 162, unsafe { (*p).i_limit },
                                    unsafe { (*p).i_offset } + 1, unsafe { (*p).i_offset })
                            };
                        }
                    }
                    unsafe {
                        sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                            c"UNION ALL".as_ptr() as *mut i8 as *const i8)
                    };
                    rc = sqlite3_select(p_parse_1, p, &mut dest);
                    p_delete = unsafe { (*p).p_prior };
                    unsafe { (*p).p_prior = p_prior };
                    unsafe {
                        (*p).n_select_row =
                            unsafe {
                                sqlite3_log_est_add(unsafe { (*p).n_select_row },
                                    unsafe { (*p_prior).n_select_row })
                            }
                    };
                    if !(unsafe { (*p).p_limit }).is_null() &&
                                    unsafe {
                                            sqlite3_expr_is_integer(unsafe {
                                                        (*unsafe { (*p).p_limit }).p_left
                                                    } as *const Expr, &mut n_limit, p_parse_1)
                                        } != 0 && n_limit > 0 &&
                            unsafe { (*p).n_select_row } as i32 >
                                unsafe { sqlite3_log_est(n_limit as u64) } as i32 {
                        unsafe {
                            (*p).n_select_row =
                                unsafe { sqlite3_log_est(n_limit as u64) }
                        };
                    }
                    if addr != 0 { unsafe { sqlite3_vdbe_jump_here(v, addr) }; }
                    if unsafe { (*p).p_next } == core::ptr::null_mut() {
                        unsafe { sqlite3_vdbe_explain_pop(p_parse_1) };
                    }
                }
                break '__c86;
            }
            if !(false) { break '__b86; }
        }
        unsafe { (*p_dest_1).i_sdst = dest.i_sdst };
        unsafe { (*p_dest_1).n_sdst = dest.n_sdst };
        unsafe { (*p_dest_1).i_sd_parm2 = dest.i_sd_parm2 };
        if !(p_delete).is_null() {
            unsafe {
                sqlite3_parser_add_cleanup(p_parse_1,
                    Some(sqlite3_select_delete_generic), p_delete as *mut ())
            };
        }
        return rc;
    }
}
extern "C" fn exists_to_join(p_parse_1: *mut Parse, p: *mut Select,
    p_where_1: *mut Expr) -> () {
    unsafe {
        if unsafe { (*p_parse_1).n_err } == 0 &&
                                p_where_1 != core::ptr::null_mut() &&
                            !(unsafe { (*p_where_1).flags } & (1 | 2) as u32 !=
                                            0 as u32) as i32 != 0 &&
                        unsafe { (*p).p_src } != core::ptr::null_mut() &&
                    unsafe { (*unsafe { (*p).p_src }).n_src } <
                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32
                &&
                (unsafe { (*p).p_limit } == core::ptr::null_mut() ||
                    unsafe { (*unsafe { (*p).p_limit }).p_right } ==
                        core::ptr::null_mut()) {
            if unsafe { (*p_where_1).op } as i32 == 44 {
                let p_right: *mut Expr = unsafe { (*p_where_1).p_right };
                exists_to_join(p_parse_1, p, unsafe { (*p_where_1).p_left });
                exists_to_join(p_parse_1, p, p_right);
            } else if unsafe { (*p_where_1).op } as i32 == 20 {
                let p_sub: *mut Select = unsafe { (*p_where_1).x.p_select };
                let p_sub_where: *mut Expr = unsafe { (*p_sub).p_where };
                if unsafe { (*unsafe { (*p_sub).p_src }).n_src } == 1 &&
                                    unsafe { (*p_sub).sel_flags } & 8 as u32 == 0 as u32 &&
                                (unsafe {
                                                (*(unsafe { (*unsafe { (*p_sub).p_src }).a.as_ptr() } as
                                                                    *mut SrcItem).offset(0 as isize)).fg.is_subquery()
                                            } == 0) as i32 != 0 &&
                            unsafe { (*p_sub).p_limit } == core::ptr::null_mut() &&
                        unsafe { (*p_sub).p_prior } == core::ptr::null_mut() {
                    let db: *mut sqlite3 = unsafe { (*p_parse_1).db };
                    let a_csr_map: *mut i32 =
                        unsafe {
                                sqlite3_db_malloc_zero(db,
                                    (unsafe { (*p_parse_1).n_tab } + 2) as u64 *
                                        core::mem::size_of::<i32>() as u64)
                            } as *mut i32;
                    if a_csr_map == core::ptr::null_mut() { return; }
                    unsafe {
                        *a_csr_map.offset(0 as isize) =
                            unsafe { (*p_parse_1).n_tab } + 1
                    };
                    renumber_cursors(p_parse_1, p_sub, -1, a_csr_map);
                    unsafe { sqlite3_db_free(db, a_csr_map as *mut ()) };
                    unsafe {
                        memset(p_where_1 as *mut (), 0,
                            core::mem::size_of::<Expr>() as u64)
                    };
                    unsafe { (*p_where_1).op = 156 as u8 };
                    unsafe { (*p_where_1).u.i_value = 1 };
                    unsafe { (*p_where_1).flags |= 2048 as u32 };
                    { let _ = 0; };
                    unsafe {
                        (*(unsafe { (*unsafe { (*p_sub).p_src }).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as
                                            isize)).fg.set_from_exists(1 as u32 as u32)
                    };
                    unsafe {
                        (*p).p_src =
                            unsafe {
                                sqlite3_src_list_append_list(p_parse_1,
                                    unsafe { (*p).p_src }, unsafe { (*p_sub).p_src })
                            }
                    };
                    if !(p_sub_where).is_null() {
                        unsafe {
                            (*p).p_where =
                                unsafe {
                                    sqlite3_p_expr(p_parse_1, 44, unsafe { (*p).p_where },
                                        p_sub_where)
                                }
                        };
                        unsafe { (*p_sub).p_where = core::ptr::null_mut() };
                    }
                    unsafe { (*p_sub).p_src = core::ptr::null_mut() };
                    unsafe {
                        sqlite3_parser_add_cleanup(p_parse_1,
                            Some(sqlite3_select_delete_generic), p_sub as *mut ())
                    };
                }
            }
        }
    }
}
extern "C" fn const_insert(p_const_1: &mut WhereConst, p_column_1: *mut Expr,
    p_value_1: *mut Expr, p_expr_1: *const Expr) -> () {
    let mut i: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_column_1).flags } & 32 as u32 != 0 as u32 { return; }
    if unsafe { sqlite3_expr_affinity(p_value_1 as *const Expr) } as i32 != 0
        {
        return;
    }
    if (unsafe {
                        sqlite3_is_binary(unsafe {
                                    sqlite3_expr_compare_coll_seq((*p_const_1).p_parse,
                                        p_expr_1 as *const Expr)
                                } as *const CollSeq)
                    } == 0) as i32 != 0 {
        return;
    }
    {
        i = 0;
        '__b87: loop {
            if !(i < (*p_const_1).n_const) { break '__b87; }
            '__c87: loop {
                let p_e2: *const Expr =
                    unsafe { *(*p_const_1).ap_expr.offset((i * 2) as isize) } as
                        *const Expr;
                { let _ = 0; };
                if unsafe { (*p_e2).i_table } as i32 ==
                            unsafe { (*p_column_1).i_table } &&
                        unsafe { (*p_e2).i_column } as i32 ==
                            unsafe { (*p_column_1).i_column } as i32 {
                    return;
                }
                break '__c87;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
    if unsafe { sqlite3_expr_affinity(p_column_1 as *const Expr) } as i32 <=
            65 {
        (*p_const_1).b_has_aff_blob = 1;
    }
    { let __p = &mut (*p_const_1).n_const; let __t = *__p; *__p += 1; __t };
    (*p_const_1).ap_expr =
        unsafe {
                sqlite3_db_realloc_or_free(unsafe {
                        (*(*p_const_1).p_parse).db
                    }, (*p_const_1).ap_expr as *mut (),
                    ((*p_const_1).n_const * 2) as u64 *
                        core::mem::size_of::<*mut Expr>() as u64)
            } as *mut *mut Expr;
    if (*p_const_1).ap_expr == core::ptr::null_mut() {
        (*p_const_1).n_const = 0;
    } else {
        unsafe {
            *(*p_const_1).ap_expr.offset(((*p_const_1).n_const * 2 - 2) as
                            isize) = p_column_1
        };
        unsafe {
            *(*p_const_1).ap_expr.offset(((*p_const_1).n_const * 2 - 1) as
                            isize) = p_value_1
        };
    }
}
extern "C" fn find_const_in_where(p_const_1: *mut WhereConst,
    p_expr_1: *mut Expr) -> () {
    let mut p_right: *mut Expr = core::ptr::null_mut();
    let mut p_left: *mut Expr = core::ptr::null_mut();
    if p_expr_1 == core::ptr::null_mut() { return; }
    if unsafe { (*p_expr_1).flags } &
                unsafe { (*p_const_1).m_exclude_on } as u32 != 0 as u32 {
        return;
    }
    if unsafe { (*p_expr_1).op } as i32 == 44 {
        find_const_in_where(p_const_1, unsafe { (*p_expr_1).p_right });
        find_const_in_where(p_const_1, unsafe { (*p_expr_1).p_left });
        return;
    }
    if unsafe { (*p_expr_1).op } as i32 != 54 { return; }
    p_right = unsafe { (*p_expr_1).p_right };
    p_left = unsafe { (*p_expr_1).p_left };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_right).op } as i32 == 168 &&
            unsafe {
                    sqlite3_expr_is_constant(unsafe { (*p_const_1).p_parse },
                        p_left)
                } != 0 {
        const_insert(unsafe { &mut *p_const_1 }, p_right, p_left,
            p_expr_1 as *const Expr);
    }
    if unsafe { (*p_left).op } as i32 == 168 &&
            unsafe {
                    sqlite3_expr_is_constant(unsafe { (*p_const_1).p_parse },
                        p_right)
                } != 0 {
        const_insert(unsafe { &mut *p_const_1 }, p_left, p_right,
            p_expr_1 as *const Expr);
    }
}
extern "C" fn propagate_constant_expr_rewrite_one(p_const_1: &mut WhereConst,
    p_expr_1: *mut Expr, b_ignore_aff_blob_1: i32) -> i32 {
    let mut i: i32 = 0;
    if unsafe { *(*p_const_1).p_oom_fault.offset(0 as isize) } != 0 {
        return 1;
    }
    if unsafe { (*p_expr_1).op } as i32 != 168 { return 0; }
    if unsafe { (*p_expr_1).flags } &
                (32 as u32 | (*p_const_1).m_exclude_on) as u32 != 0 as u32 {
        return 0;
    }
    {
        i = 0;
        '__b88: loop {
            if !(i < (*p_const_1).n_const) { break '__b88; }
            '__c88: loop {
                let p_column: *mut Expr =
                    unsafe { *(*p_const_1).ap_expr.offset((i * 2) as isize) };
                if p_column == p_expr_1 { break '__c88; }
                if unsafe { (*p_column).i_table } !=
                        unsafe { (*p_expr_1).i_table } {
                    break '__c88;
                }
                if unsafe { (*p_column).i_column } as i32 !=
                        unsafe { (*p_expr_1).i_column } as i32 {
                    break '__c88;
                }
                { let _ = 0; };
                if b_ignore_aff_blob_1 != 0 &&
                        unsafe { sqlite3_expr_affinity(p_column as *const Expr) } as
                                i32 <= 65 {
                    break '__b88;
                }
                {
                    let __p = &mut (*p_const_1).n_chng;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                unsafe { (*p_expr_1).flags &= !(8388608 as u32) };
                unsafe { (*p_expr_1).flags |= 32 as u32 };
                { let _ = 0; };
                unsafe {
                    (*p_expr_1).p_left =
                        unsafe {
                            sqlite3_expr_dup(unsafe { (*(*p_const_1).p_parse).db },
                                unsafe {
                                        *(*p_const_1).ap_expr.offset((i * 2 + 1) as isize)
                                    } as *const Expr, 0)
                        }
                };
                if unsafe {
                            (*unsafe { (*(*p_const_1).p_parse).db }).malloc_failed
                        } != 0 {
                    return 1;
                }
                break '__b88;
                break '__c88;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}
extern "C" fn propagate_constant_expr_rewrite(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let p_const: *mut WhereConst = unsafe { (*p_walker_1).u.p_const };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_const).b_has_aff_blob } != 0 {
            if unsafe { (*p_expr_1).op } as i32 >= 54 &&
                        unsafe { (*p_expr_1).op } as i32 <= 58 ||
                    unsafe { (*p_expr_1).op } as i32 == 45 {
                propagate_constant_expr_rewrite_one(unsafe { &mut *p_const },
                    unsafe { (*p_expr_1).p_left }, 0);
                if unsafe {
                            *unsafe { (*p_const).p_oom_fault.offset(0 as isize) }
                        } != 0 {
                    return 1;
                }
                if unsafe {
                                sqlite3_expr_affinity(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            } as i32 != 66 {
                    propagate_constant_expr_rewrite_one(unsafe {
                            &mut *p_const
                        }, unsafe { (*p_expr_1).p_right }, 0);
                }
            }
        }
        return propagate_constant_expr_rewrite_one(unsafe { &mut *p_const },
                p_expr_1, unsafe { (*p_const).b_has_aff_blob });
    }
}
extern "C" fn propagate_constants(p_parse_1: *mut Parse, p: &Select) -> i32 {
    unsafe {
        let mut x: WhereConst = unsafe { core::mem::zeroed() };
        let mut w: Walker = unsafe { core::mem::zeroed() };
        let mut n_chng: i32 = 0;
        x.p_parse = p_parse_1;
        x.p_oom_fault =
            unsafe { &mut (*unsafe { (*p_parse_1).db }).malloc_failed };
        '__b89: loop {
            '__c89: loop {
                x.n_const = 0;
                x.n_chng = 0;
                x.ap_expr = core::ptr::null_mut();
                x.b_has_aff_blob = 0;
                if (*p).p_src != core::ptr::null_mut() &&
                            unsafe { (*(*p).p_src).n_src } > 0 &&
                        unsafe {
                                        (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                                            *mut SrcItem).offset(0 as isize)).fg.jointype
                                    } as i32 & 64 != 0 {
                    x.m_exclude_on = (2 | 1) as u32;
                } else { x.m_exclude_on = 1 as u32; }
                find_const_in_where(&mut x, (*p).p_where);
                if x.n_const != 0 {
                    unsafe {
                        memset(&raw mut w as *mut (), 0,
                            core::mem::size_of::<Walker>() as u64)
                    };
                    w.p_parse = p_parse_1;
                    w.x_expr_callback = Some(propagate_constant_expr_rewrite);
                    w.x_select_callback = Some(sqlite3_select_walk_noop);
                    w.x_select_callback2 = None;
                    w.walker_depth = 0;
                    w.u.p_const = &mut x;
                    unsafe { sqlite3_walk_expr(&mut w, (*p).p_where) };
                    unsafe {
                        sqlite3_db_free(unsafe { (*x.p_parse).db },
                            x.ap_expr as *mut ())
                    };
                    n_chng += x.n_chng;
                }
                break '__c89;
            }
            if !(x.n_chng != 0) { break '__b89; }
        }
        return n_chng;
    }
}
extern "C" fn count_of_view_optimization(p_parse_1: *mut Parse,
    p: &mut Select) -> i32 {
    unsafe {
        let mut p_sub: *mut Select = core::ptr::null_mut();
        let mut p_prior: *mut Select = core::ptr::null_mut();
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        let mut p_count: *mut Expr = core::ptr::null_mut();
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut p_from: *mut SrcItem = core::ptr::null_mut();
        if (*p).sel_flags & 8 as u32 == 0 as u32 { return 0; }
        if unsafe { (*(*p).p_e_list).n_expr } != 1 { return 0; }
        if !((*p).p_where).is_null() { return 0; }
        if !((*p).p_having).is_null() { return 0; }
        if !((*p).p_group_by).is_null() { return 0; }
        if !((*p).p_order_by).is_null() { return 0; }
        p_expr =
            unsafe {
                (*(unsafe { (*(*p).p_e_list).a.as_ptr() } as
                                *mut ExprList_item).offset(0 as isize)).p_expr
            };
        if unsafe { (*p_expr).op } as i32 != 169 { return 0; }
        { let _ = 0; };
        if unsafe {
                    sqlite3_stricmp(unsafe { (*p_expr).u.z_token } as *const i8,
                        c"count".as_ptr() as *mut i8 as *const i8)
                } != 0 {
            return 0;
        }
        { let _ = 0; };
        if unsafe { (*p_expr).x.p_list } != core::ptr::null_mut() {
            return 0;
        }
        if unsafe { (*(*p).p_src).n_src } != 1 { return 0; }
        if unsafe { (*p_expr).flags } & 16777216 as u32 != 0 as u32 {
            return 0;
        }
        p_from = unsafe { (*(*p).p_src).a.as_ptr() } as *mut SrcItem;
        if unsafe { (*p_from).fg.is_subquery() } as i32 == 0 { return 0; }
        p_sub = unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select };
        if unsafe { (*p_sub).p_prior } == core::ptr::null_mut() { return 0; }
        if unsafe { (*p_sub).sel_flags } & 67108864 as u32 != 0 { return 0; }
        '__b90: loop {
            '__c90: loop {
                if unsafe { (*p_sub).op } as i32 != 136 &&
                        !(unsafe { (*p_sub).p_prior }).is_null() {
                    return 0;
                }
                if !(unsafe { (*p_sub).p_where }).is_null() { return 0; }
                if !(unsafe { (*p_sub).p_limit }).is_null() { return 0; }
                if unsafe { (*p_sub).sel_flags } & (8 | 1) as u32 != 0 {
                    return 0;
                }
                { let _ = 0; };
                p_sub = unsafe { (*p_sub).p_prior };
                break '__c90;
            }
            if !(!(p_sub).is_null()) { break '__b90; }
        }
        db = unsafe { (*p_parse_1).db };
        p_count = p_expr;
        p_expr = core::ptr::null_mut();
        p_sub = unsafe { sqlite3_subquery_detach(db, p_from) };
        unsafe { sqlite3_src_list_delete(db, (*p).p_src) };
        (*p).p_src =
            unsafe {
                    sqlite3_db_malloc_zero(unsafe { (*p_parse_1).db },
                        core::mem::offset_of!(SrcList, a) as u64 +
                            core::mem::size_of::<SrcItem>() as u64)
                } as *mut SrcList;
        while !(p_sub).is_null() {
            let mut p_term: *mut Expr = core::ptr::null_mut();
            p_prior = unsafe { (*p_sub).p_prior };
            unsafe { (*p_sub).p_prior = core::ptr::null_mut() };
            unsafe { (*p_sub).p_next = core::ptr::null_mut() };
            unsafe { (*p_sub).sel_flags |= 8 as u32 };
            unsafe { (*p_sub).sel_flags &= !(256 as u32) };
            unsafe { (*p_sub).n_select_row = 0 as LogEst };
            unsafe {
                sqlite3_parser_add_cleanup(p_parse_1,
                    Some(sqlite3_expr_list_delete_generic),
                    unsafe { (*p_sub).p_e_list } as *mut ())
            };
            p_term =
                if !(p_prior).is_null() {
                    unsafe { sqlite3_expr_dup(db, p_count as *const Expr, 0) }
                } else { p_count };
            unsafe {
                (*p_sub).p_e_list =
                    unsafe {
                        sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                            p_term)
                    }
            };
            p_term =
                unsafe {
                    sqlite3_p_expr(p_parse_1, 139, core::ptr::null_mut(),
                        core::ptr::null_mut())
                };
            unsafe { sqlite3_p_expr_add_select(p_parse_1, p_term, p_sub) };
            if p_expr == core::ptr::null_mut() {
                p_expr = p_term;
            } else {
                p_expr =
                    unsafe { sqlite3_p_expr(p_parse_1, 107, p_term, p_expr) };
            }
            p_sub = p_prior;
        }
        unsafe {
            (*(unsafe { (*(*p).p_e_list).a.as_ptr() } as
                                *mut ExprList_item).offset(0 as isize)).p_expr = p_expr
        };
        (*p).sel_flags &= !(8 as u32);
        return 1;
    }
}
extern "C" fn push_down_window_check(p_parse_1: *mut Parse, p_subq_1: &Select,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        return unsafe {
                sqlite3_expr_is_constant_or_group_by(p_parse_1, p_expr_1,
                    unsafe { (*(*p_subq_1).p_win).p_partition })
            };
    }
}
extern "C" fn push_down_where_terms(p_parse_1: *mut Parse,
    mut p_subq_1: *mut Select, mut p_where_1: *mut Expr,
    p_src_list_1: *mut SrcList, i_src_1: i32) -> i32 {
    unsafe {
        let mut p_new: *mut Expr = core::ptr::null_mut();
        let mut p_src: *const SrcItem = core::ptr::null();
        let mut n_chng: i32 = 0;
        p_src =
            unsafe {
                &mut *(unsafe { (*p_src_list_1).a.as_ptr() } as
                                *mut SrcItem).offset(i_src_1 as isize)
            };
        if p_where_1 == core::ptr::null_mut() { return 0; }
        if unsafe { (*p_subq_1).sel_flags } & (8192 | 33554432) as u32 != 0 {
            return 0;
        }
        if unsafe { (*p_src).fg.jointype } as i32 & (64 | 16) != 0 {
            return 0;
        }
        if !(unsafe { (*p_subq_1).p_prior }).is_null() {
            let mut p_sel: *const Select = core::ptr::null();
            let mut not_union_all: i32 = 0;
            {
                p_sel = p_subq_1;
                '__b92: loop {
                    if !(!(p_sel).is_null()) { break '__b92; }
                    '__c92: loop {
                        let op: u8 = unsafe { (*p_sel).op };
                        { let _ = 0; };
                        if op as i32 != 136 && op as i32 != 139 {
                            not_union_all = 1;
                        }
                        if !(unsafe { (*p_sel).p_win }).is_null() { return 0; }
                        break '__c92;
                    }
                    p_sel = unsafe { (*p_sel).p_prior };
                }
            }
            if not_union_all != 0 {
                {
                    p_sel = p_subq_1;
                    '__b93: loop {
                        if !(!(p_sel).is_null()) { break '__b93; }
                        '__c93: loop {
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_sel).p_e_list } as *const ExprList;
                            { let _ = 0; };
                            {
                                ii = 0;
                                '__b94: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b94; }
                                    '__c94: loop {
                                        let p_coll: *const CollSeq =
                                            unsafe {
                                                    sqlite3_expr_coll_seq(p_parse_1,
                                                        unsafe {
                                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                *const ExprList_item).offset(ii as isize)).p_expr
                                                            } as *const Expr)
                                                } as *const CollSeq;
                                        if (unsafe { sqlite3_is_binary(p_coll as *const CollSeq) }
                                                        == 0) as i32 != 0 {
                                            return 0;
                                        }
                                        break '__c94;
                                    }
                                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            break '__c93;
                        }
                        p_sel = unsafe { (*p_sel).p_prior };
                    }
                }
            }
        } else {
            if !(unsafe { (*p_subq_1).p_win }).is_null() &&
                    unsafe { (*unsafe { (*p_subq_1).p_win }).p_partition } ==
                        core::ptr::null_mut() {
                return 0;
            }
        }
        if unsafe { (*p_subq_1).p_limit } != core::ptr::null_mut() {
            return 0;
        }
        while unsafe { (*p_where_1).op } as i32 == 44 {
            n_chng +=
                push_down_where_terms(p_parse_1, p_subq_1,
                    unsafe { (*p_where_1).p_right }, p_src_list_1, i_src_1);
            p_where_1 = unsafe { (*p_where_1).p_left };
        }
        if unsafe {
                    sqlite3_expr_is_single_table_constraint(p_where_1,
                        p_src_list_1 as *const SrcList, i_src_1, 1)
                } != 0 {
            { let __p = &mut n_chng; let __t = *__p; *__p += 1; __t };
            unsafe { (*p_subq_1).sel_flags |= 16777216 as u32 };
            while !(p_subq_1).is_null() {
                let mut x: SubstContext = unsafe { core::mem::zeroed() };
                p_new =
                    unsafe {
                        sqlite3_expr_dup(unsafe { (*p_parse_1).db },
                            p_where_1 as *const Expr, 0)
                    };
                unset_join_expr(p_new, -1, 1);
                x.p_parse = p_parse_1;
                x.i_table = unsafe { (*p_src).i_cursor };
                x.i_new_table = unsafe { (*p_src).i_cursor };
                x.is_outer_join = 0;
                x.n_sel_depth = 0;
                x.p_e_list = unsafe { (*p_subq_1).p_e_list };
                x.p_c_list =
                    find_leftmost_exprlist(p_subq_1 as *const Select);
                p_new = subst_expr(&mut x, p_new);
                { let _ = 0; };
                if unsafe { (*p_parse_1).n_err } == 0 &&
                            unsafe { (*p_new).op } as i32 == 50 &&
                        unsafe { (*p_new).flags } & 4096 as u32 != 0 as u32 {
                    { let _ = 0; };
                    unsafe {
                        (*unsafe { (*p_new).x.p_select }).sel_flags |= 32 as u32
                    };
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    unsafe {
                        (*unsafe { (*p_where_1).x.p_select }).sel_flags |= 32 as u32
                    };
                }
                if !(unsafe { (*p_subq_1).p_win }).is_null() &&
                        0 ==
                            push_down_window_check(p_parse_1, unsafe { &*p_subq_1 },
                                p_new) {
                    unsafe {
                        sqlite3_expr_delete(unsafe { (*p_parse_1).db }, p_new)
                    };
                    { let __p = &mut n_chng; let __t = *__p; *__p -= 1; __t };
                    break;
                }
                if unsafe { (*p_subq_1).sel_flags } & 8 as u32 != 0 {
                    unsafe {
                        (*p_subq_1).p_having =
                            unsafe {
                                sqlite3_expr_and(p_parse_1, unsafe { (*p_subq_1).p_having },
                                    p_new)
                            }
                    };
                } else {
                    unsafe {
                        (*p_subq_1).p_where =
                            unsafe {
                                sqlite3_expr_and(p_parse_1, unsafe { (*p_subq_1).p_where },
                                    p_new)
                            }
                    };
                }
                p_subq_1 = unsafe { (*p_subq_1).p_prior };
            }
        }
        return n_chng;
    }
}
extern "C" fn disable_unused_subquery_result_columns(p_item_1: &SrcItem)
    -> i32 {
    unsafe {
        let mut n_col: i32 = 0;
        let mut p_sub: *mut Select = core::ptr::null_mut();
        let mut p_x: *mut Select = core::ptr::null_mut();
        let mut p_tab: *const Table = core::ptr::null();
        let mut j: i32 = 0;
        let mut n_chng: i32 = 0;
        let mut col_used: Bitmask = 0 as Bitmask;
        { let _ = 0; };
        if (*p_item_1).fg.is_correlated() != 0 || (*p_item_1).fg.is_cte() != 0
            {
            return 0;
        }
        { let _ = 0; };
        p_tab = (*p_item_1).p_s_tab;
        { let _ = 0; };
        p_sub = unsafe { (*(*p_item_1).u4.p_subq).p_select };
        { let _ = 0; };
        {
            p_x = p_sub;
            '__b97: loop {
                if !(!(p_x).is_null()) { break '__b97; }
                '__c97: loop {
                    if unsafe { (*p_x).sel_flags } & (1 | 8) as u32 != 0 as u32
                        {
                        return 0;
                    }
                    if !(unsafe { (*p_x).p_prior }).is_null() &&
                            unsafe { (*p_x).op } as i32 != 136 {
                        return 0;
                    }
                    if !(unsafe { (*p_x).p_win }).is_null() { return 0; }
                    break '__c97;
                }
                p_x = unsafe { (*p_x).p_prior };
            }
        }
        col_used = (*p_item_1).col_used;
        if !(unsafe { (*p_sub).p_order_by }).is_null() {
            let p_list: *const ExprList =
                unsafe { (*p_sub).p_order_by } as *const ExprList;
            {
                j = 0;
                '__b98: loop {
                    if !(j < unsafe { (*p_list).n_expr }) { break '__b98; }
                    '__c98: loop {
                        let mut i_col: u16 =
                            unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                        *mut ExprList_item).offset(j as isize)).u.x.i_order_by_col
                            };
                        if i_col as i32 > 0 {
                            { let __p = &mut i_col; let __t = *__p; *__p -= 1; __t };
                            col_used |=
                                (1 as Bitmask) <<
                                    if i_col as i32 >=
                                            (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                            1
                                    } else { i_col as i32 };
                        }
                        break '__c98;
                    }
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        n_col = unsafe { (*p_tab).n_col } as i32;
        {
            j = 0;
            '__b99: loop {
                if !(j < n_col) { break '__b99; }
                '__c99: loop {
                    let m: Bitmask =
                        if j <
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                    1 {
                            (1 as Bitmask) << j
                        } else {
                            (1 as Bitmask) <<
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                    1
                        };
                    if m & col_used != 0 as u64 { break '__c99; }
                    {
                        p_x = p_sub;
                        '__b100: loop {
                            if !(!(p_x).is_null()) { break '__b100; }
                            '__c100: loop {
                                let p_y: *mut Expr =
                                    unsafe {
                                        (*(unsafe { (*unsafe { (*p_x).p_e_list }).a.as_ptr() } as
                                                        *mut ExprList_item).offset(j as isize)).p_expr
                                    };
                                if unsafe { (*p_y).op } as i32 == 122 { break '__c100; }
                                unsafe { (*p_y).op = 122 as u8 };
                                unsafe { (*p_y).flags &= !((8192 | 524288) as u32) };
                                unsafe { (*p_x).sel_flags |= 16777216 as u32 };
                                { let __p = &mut n_chng; let __t = *__p; *__p += 1; __t };
                                break '__c100;
                            }
                            p_x = unsafe { (*p_x).p_prior };
                        }
                    }
                    break '__c99;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        return n_chng;
    }
}
extern "C" fn is_self_join_view(p_tab_list_1: &mut SrcList,
    p_this_1: &SrcItem, mut i_first_1: i32, i_end_1: i32) -> *mut SrcItem {
    unsafe {
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        let mut p_sel: *const Select = core::ptr::null();
        { let _ = 0; };
        p_sel = unsafe { (*(*p_this_1).u4.p_subq).p_select };
        { let _ = 0; };
        if unsafe { (*p_sel).sel_flags } & 16777216 as u32 != 0 {
            return core::ptr::null_mut();
        }
        while i_first_1 < i_end_1 {
            let mut p_s1: *const Select = core::ptr::null();
            p_item =
                unsafe {
                    &mut *((*p_tab_list_1).a.as_ptr() as
                                    *mut SrcItem).offset({
                                        let __p = &mut i_first_1;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize)
                };
            if (unsafe { (*p_item).fg.is_subquery() } == 0) as i32 != 0 {
                continue;
            }
            if unsafe { (*p_item).fg.via_coroutine() } != 0 { continue; }
            if unsafe { (*p_item).z_name } == core::ptr::null_mut() {
                continue;
            }
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*unsafe { (*p_item).p_s_tab }).p_schema } !=
                    unsafe { (*(*p_this_1).p_s_tab).p_schema } {
                continue;
            }
            if unsafe {
                        sqlite3_stricmp(unsafe { (*p_item).z_name } as *const i8,
                            (*p_this_1).z_name as *const i8)
                    } != 0 {
                continue;
            }
            p_s1 = unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select };
            if unsafe { (*unsafe { (*p_item).p_s_tab }).p_schema } ==
                        core::ptr::null_mut() &&
                    unsafe { (*p_sel).sel_id } != unsafe { (*p_s1).sel_id } {
                continue;
            }
            if unsafe { (*p_s1).sel_flags } & 16777216 as u32 != 0 {
                continue;
            }
            return p_item;
        }
        return core::ptr::null_mut();
    }
}
extern "C" fn from_clause_term_can_be_coroutine(p_parse_1: &Parse,
    p_tab_list_1: *mut SrcList, mut i: i32, sel_flags_1: i32) -> i32 {
    unsafe {
        let mut p_item: *mut SrcItem =
            unsafe {
                &mut *(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                *mut SrcItem).offset(i as isize)
            };
        if unsafe { (*p_item).fg.is_cte() } != 0 {
            let p_cte_use: *const CteUse =
                unsafe { (*p_item).u2.p_cte_use } as *const CteUse;
            if unsafe { (*p_cte_use).e_m10d } as i32 == 0 { return 0; }
            if unsafe { (*p_cte_use).n_use } as i32 >= 2 &&
                    unsafe { (*p_cte_use).e_m10d } as i32 != 2 {
                return 0;
            }
        }
        if unsafe {
                            (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)).fg.jointype
                        } as i32 & 64 != 0 {
            return 0;
        }
        if unsafe { (*(*p_parse_1).db).db_opt_flags } & 33554432 as u32 !=
                0 as u32 {
            return 0;
        }
        if is_self_join_view(unsafe { &mut *p_tab_list_1 },
                    unsafe { &*p_item }, i + 1,
                    unsafe { (*p_tab_list_1).n_src }) != core::ptr::null_mut() {
            return 0;
        }
        if i == 0 {
            if unsafe { (*p_tab_list_1).n_src } == 1 { return 1; }
            if unsafe {
                                (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                    *mut SrcItem).offset(1 as isize)).fg.jointype
                            } as i32 & 2 != 0 {
                return 1;
            }
            if sel_flags_1 & 268435456 != 0 { return 0; }
            return 1;
        }
        if sel_flags_1 & 268435456 != 0 { return 0; }
        loop {
            if unsafe { (*p_item).fg.jointype } as i32 & (32 | 2) != 0 {
                return 0;
            }
            if i == 0 { break; }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            {
                let __p = &mut p_item;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(-1) };
                __t
            };
            if unsafe { (*p_item).fg.is_subquery() } != 0 { return 0; }
        }
        return 1;
    }
}
extern "C" fn sqlite3_copy_sort_order(p1: &mut ExprList, p2: *const ExprList)
    -> i32 {
    { let _ = 0; };
    if !(p2).is_null() && (*p1).n_expr == unsafe { (*p2).n_expr } {
        let mut ii: i32 = 0;
        {
            ii = 0;
            '__b103: loop {
                if !(ii < (*p1).n_expr) { break '__b103; }
                '__c103: loop {
                    let mut sort_flags: u8 = 0 as u8;
                    sort_flags =
                        (unsafe {
                                        (*(unsafe { (*p2).a.as_ptr() } as
                                                            *mut ExprList_item).offset(ii as isize)).fg.sort_flags
                                    } as i32 & 1) as u8;
                    unsafe {
                        (*((*p1).a.as_ptr() as
                                                *mut ExprList_item).offset(ii as isize)).fg.sort_flags =
                            sort_flags
                    };
                    break '__c103;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        return 1;
    } else { return 0; }
}
extern "C" fn agginfo_free(db: *mut sqlite3, p_arg_1: *mut ()) -> () {
    let p: *mut AggInfo = p_arg_1 as *mut AggInfo;
    unsafe { sqlite3_db_free(db, unsafe { (*p).a_col } as *mut ()) };
    unsafe { sqlite3_db_free(db, unsafe { (*p).a_func } as *mut ()) };
    unsafe { sqlite3_db_free_nn(db, p as *mut ()) };
}
extern "C" fn having_to_where_expr_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 != 44 {
            let p_s: *mut Select = unsafe { (*p_walker_1).u.p_select };
            if unsafe {
                                sqlite3_expr_is_constant_or_group_by(unsafe {
                                        (*p_walker_1).p_parse
                                    }, p_expr_1, unsafe { (*p_s).p_group_by })
                            } != 0 &&
                        (unsafe { (*p_expr_1).flags } & (1 | 536870912) as u32 ==
                                    536870912 as u32) as i32 == 0 &&
                    unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
                let db: *mut sqlite3 =
                    unsafe { (*unsafe { (*p_walker_1).p_parse }).db };
                let mut p_new: *mut Expr =
                    unsafe { sqlite3_expr_int32(db, 1) };
                if !(p_new).is_null() {
                    let p_where: *mut Expr = unsafe { (*p_s).p_where };
                    {
                        let t: Expr = unsafe { core::ptr::read(p_new) };
                        unsafe { *p_new = unsafe { core::ptr::read(p_expr_1) } };
                        unsafe { *p_expr_1 = t };
                    }
                    p_new =
                        unsafe {
                            sqlite3_expr_and(unsafe { (*p_walker_1).p_parse }, p_where,
                                p_new)
                        };
                    unsafe { (*p_s).p_where = p_new };
                    unsafe { (*p_walker_1).e_code = 1 as u16 };
                }
            }
            return 1;
        }
        return 0;
    }
}
extern "C" fn having_to_where(p_parse_1: *mut Parse, p: *mut Select) -> () {
    unsafe {
        let mut s_walker: Walker = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut s_walker as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        s_walker.p_parse = p_parse_1;
        s_walker.x_expr_callback = Some(having_to_where_expr_cb);
        s_walker.u.p_select = p;
        unsafe { sqlite3_walk_expr(&mut s_walker, unsafe { (*p).p_having }) };
    }
}
extern "C" fn min_max_query(db: *mut sqlite3, p_func_1: &Expr,
    pp_min_max_1: &mut *mut ExprList) -> u8 {
    unsafe {
        let mut e_ret: i32 = 0;
        let mut p_e_list: *const ExprList = core::ptr::null();
        let mut z_func: *const i8 = core::ptr::null();
        let mut p_order_by: *mut ExprList = core::ptr::null_mut();
        let mut sort_flags: u8 = 0 as u8;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        p_e_list = (*p_func_1).x.p_list;
        if p_e_list == core::ptr::null_mut() ||
                        unsafe { (*p_e_list).n_expr } != 1 ||
                    (*p_func_1).flags & 16777216 as u32 != 0 as u32 ||
                unsafe { (*db).db_opt_flags } & 65536 as u32 != 0 as u32 {
            return e_ret as u8;
        }
        { let _ = 0; };
        z_func = (*p_func_1).u.z_token as *const i8;
        if unsafe {
                    sqlite3_str_i_cmp(z_func,
                        c"min".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            e_ret = 1;
            if unsafe {
                        sqlite3_expr_can_be_null(unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                    *mut ExprList_item).offset(0 as isize)).p_expr
                                } as *const Expr)
                    } != 0 {
                sort_flags = 2 as u8;
            }
        } else if unsafe {
                    sqlite3_str_i_cmp(z_func,
                        c"max".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            e_ret = 2;
            sort_flags = 1 as u8;
        } else { return e_ret as u8; }
        *pp_min_max_1 =
            {
                p_order_by =
                    unsafe {
                        sqlite3_expr_list_dup(db, p_e_list as *const ExprList, 0)
                    };
                p_order_by
            };
        { let _ = 0; };
        if !(p_order_by).is_null() {
            unsafe {
                (*(unsafe { (*p_order_by).a.as_ptr() } as
                                        *mut ExprList_item).offset(0 as isize)).fg.sort_flags =
                    sort_flags
            };
        }
        return e_ret as u8;
    }
}
extern "C" fn analyze_agg_func_args(p_agg_info_1: &AggInfo,
    p_nc_1: *mut NameContext) -> () {
    unsafe {
        let mut i: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_nc_1).nc_flags |= 131072 };
        {
            i = 0;
            '__b104: loop {
                if !(i < (*p_agg_info_1).n_func) { break '__b104; }
                '__c104: loop {
                    let p_expr: *const Expr =
                        unsafe {
                                (*(*p_agg_info_1).a_func.offset(i as isize)).p_f_expr
                            } as *const Expr;
                    { let _ = 0; };
                    { let _ = 0; };
                    unsafe {
                        sqlite3_expr_analyze_agg_list(p_nc_1,
                            unsafe { (*p_expr).x.p_list })
                    };
                    if !(unsafe { (*p_expr).p_left }).is_null() {
                        { let _ = 0; };
                        { let _ = 0; };
                        unsafe {
                            sqlite3_expr_analyze_agg_list(p_nc_1,
                                unsafe { (*unsafe { (*p_expr).p_left }).x.p_list })
                        };
                    }
                    { let _ = 0; };
                    if unsafe { (*p_expr).flags } & 16777216 as u32 != 0 as u32
                        {
                        unsafe {
                            sqlite3_expr_analyze_aggregates(p_nc_1,
                                unsafe { (*unsafe { (*p_expr).y.p_win }).p_filter })
                        };
                    }
                    break '__c104;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { (*p_nc_1).nc_flags &= !131072 };
    }
}
extern "C" fn optimize_aggregate_use_of_indexed_expr(p_parse_1: *const Parse,
    p_select_1: *const Select, p_agg_info_1: *mut AggInfo,
    p_nc_1: *mut NameContext) -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            (*p_agg_info_1).n_column =
                unsafe { (*p_agg_info_1).n_accumulator }
        };
        if unsafe { (*p_agg_info_1).n_sorting_column } > 0 as u32 {
            let mut mx: i32 =
                unsafe { (*unsafe { (*p_select_1).p_group_by }).n_expr } - 1;
            let mut j: i32 = 0;
            let mut k: i32 = 0;
            {
                j = 0;
                '__b105: loop {
                    if !(j < unsafe { (*p_agg_info_1).n_column }) {
                        break '__b105;
                    }
                    '__c105: loop {
                        k =
                            unsafe {
                                (*unsafe {
                                            (*p_agg_info_1).a_col.offset(j as isize)
                                        }).i_sorter_column
                            };
                        if k > mx { mx = k; }
                        break '__c105;
                    }
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*p_agg_info_1).n_sorting_column = (mx + 1) as u32 };
        }
        analyze_agg_func_args(unsafe { &*p_agg_info_1 }, p_nc_1);
        { let _ = p_select_1; };
        { let _ = p_parse_1; };
    }
}
extern "C" fn assign_aggregate_registers(p_parse_1: &mut Parse,
    p_agg_info_1: &mut AggInfo) -> () {
    { let _ = 0; };
    { let _ = 0; };
    (*p_agg_info_1).i_first_reg = (*p_parse_1).n_mem + 1;
    (*p_parse_1).n_mem += (*p_agg_info_1).n_column + (*p_agg_info_1).n_func;
}
extern "C" fn aggregate_idx_epr_ref_to_col_callback(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut p_agg_info: *const AggInfo = core::ptr::null();
        let mut p_col: *const AggInfo_col = core::ptr::null();
        { let _ = p_walker_1; };
        if unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
            return 0;
        }
        if unsafe { (*p_expr_1).op } as i32 == 170 { return 0; }
        if unsafe { (*p_expr_1).op } as i32 == 169 { return 0; }
        if unsafe { (*p_expr_1).op } as i32 == 179 { return 0; }
        p_agg_info = unsafe { (*p_expr_1).p_agg_info };
        if unsafe { (*p_expr_1).i_agg } as i32 >=
                unsafe { (*p_agg_info).n_column } {
            return 0;
        }
        { let _ = 0; };
        p_col =
            unsafe {
                unsafe {
                    (*p_agg_info).a_col.offset(unsafe { (*p_expr_1).i_agg } as
                            isize)
                }
            };
        unsafe { (*p_expr_1).op = 170 as u8 };
        unsafe { (*p_expr_1).i_table = unsafe { (*p_col).i_table } };
        unsafe {
            (*p_expr_1).i_column = unsafe { (*p_col).i_column } as ynVar
        };
        unsafe { (*p_expr_1).flags &= !((8192 | 512 | 524288) as u32) };
        return 1;
    }
}
extern "C" fn aggregate_convert_indexed_expr_ref_to_column(p_agg_info_1:
        &AggInfo) -> () {
    let mut i: i32 = 0;
    let mut w: Walker = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut w as *mut (), 0,
            core::mem::size_of::<Walker>() as u64)
    };
    w.x_expr_callback = Some(aggregate_idx_epr_ref_to_col_callback);
    {
        i = 0;
        '__b106: loop {
            if !(i < (*p_agg_info_1).n_func) { break '__b106; }
            '__c106: loop {
                unsafe {
                    sqlite3_walk_expr(&mut w,
                        unsafe {
                            (*(*p_agg_info_1).a_func.offset(i as isize)).p_f_expr
                        })
                };
                break '__c106;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn update_accumulator(p_parse_1: *mut Parse, reg_acc_1: i32,
    p_agg_info_1: &mut AggInfo, e_distinct_type_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut reg_hit: i32 = 0;
        let mut addr_hit_test: i32 = 0;
        let mut p_f: *mut AggInfo_func = core::ptr::null_mut();
        let mut p_c: *const AggInfo_col = core::ptr::null();
        { let _ = 0; };
        if unsafe { (*p_parse_1).n_err } != 0 { return; }
        (*p_agg_info_1).direct_mode = 1 as u8;
        {
            { i = 0; p_f = (*p_agg_info_1).a_func };
            '__b107: loop {
                if !(i < (*p_agg_info_1).n_func) { break '__b107; }
                '__c107: loop {
                    let mut n_arg: i32 = 0;
                    let mut addr_next: i32 = 0;
                    let mut reg_agg: i32 = 0;
                    let mut reg_agg_sz: i32 = 0;
                    let mut reg_distinct: i32 = 0;
                    let mut p_list: *mut ExprList = core::ptr::null_mut();
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    p_list = unsafe { (*unsafe { (*p_f).p_f_expr }).x.p_list };
                    if unsafe { (*unsafe { (*p_f).p_f_expr }).flags } &
                                16777216 as u32 != 0 as u32 {
                        let p_filter: *mut Expr =
                            unsafe {
                                (*unsafe { (*unsafe { (*p_f).p_f_expr }).y.p_win }).p_filter
                            };
                        if (*p_agg_info_1).n_accumulator != 0 &&
                                    unsafe { (*unsafe { (*p_f).p_func }).func_flags } &
                                            32 as u32 != 0 && reg_acc_1 != 0 {
                            if reg_hit == 0 {
                                reg_hit =
                                    {
                                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                                        *__p += 1;
                                        *__p
                                    };
                            }
                            unsafe { sqlite3_vdbe_add_op2(v, 82, reg_acc_1, reg_hit) };
                        }
                        addr_next = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        unsafe {
                            sqlite3_expr_if_false(p_parse_1, p_filter, addr_next, 16)
                        };
                    }
                    if unsafe { (*p_f).i_ob_tab } >= 0 {
                        let mut jj: i32 = 0;
                        let mut p_ob_list: *mut ExprList = core::ptr::null_mut();
                        { let _ = 0; };
                        n_arg = unsafe { (*p_list).n_expr };
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_ob_list =
                            unsafe {
                                (*unsafe { (*unsafe { (*p_f).p_f_expr }).p_left }).x.p_list
                            };
                        { let _ = 0; };
                        { let _ = 0; };
                        reg_agg_sz = unsafe { (*p_ob_list).n_expr };
                        if (unsafe { (*p_f).b_ob_unique } == 0) as i32 != 0 {
                            {
                                let __p = &mut reg_agg_sz;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        if unsafe { (*p_f).b_ob_payload } != 0 {
                            reg_agg_sz += n_arg;
                        }
                        if unsafe { (*p_f).b_use_subtype } != 0 {
                            reg_agg_sz += n_arg;
                        }
                        {
                            let __p = &mut reg_agg_sz;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        reg_agg =
                            unsafe { sqlite3_get_temp_range(p_parse_1, reg_agg_sz) };
                        reg_distinct = reg_agg;
                        unsafe {
                            sqlite3_expr_code_expr_list(p_parse_1, p_ob_list, reg_agg,
                                0, 1 as u8)
                        };
                        jj = unsafe { (*p_ob_list).n_expr };
                        if (unsafe { (*p_f).b_ob_unique } == 0) as i32 != 0 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 128, unsafe { (*p_f).i_ob_tab },
                                    reg_agg + jj)
                            };
                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { (*p_f).b_ob_payload } != 0 {
                            reg_distinct = reg_agg + jj;
                            unsafe {
                                sqlite3_expr_code_expr_list(p_parse_1, p_list, reg_distinct,
                                    0, 1 as u8)
                            };
                            jj += n_arg;
                        }
                        if unsafe { (*p_f).b_use_subtype } != 0 {
                            let mut kk: i32 = 0;
                            let reg_base: i32 =
                                if unsafe { (*p_f).b_ob_payload } != 0 {
                                    reg_distinct
                                } else { reg_agg };
                            {
                                kk = 0;
                                '__b108: loop {
                                    if !(kk < n_arg) { break '__b108; }
                                    '__c108: loop {
                                        unsafe {
                                            sqlite3_vdbe_add_op2(v, 183, reg_base + kk, reg_agg + jj)
                                        };
                                        break '__c108;
                                    }
                                    {
                                        { let __p = &mut kk; let __t = *__p; *__p += 1; __t };
                                        { let __p = &mut jj; let __t = *__p; *__p += 1; __t }
                                    };
                                }
                            }
                        }
                    } else if !(p_list).is_null() {
                        n_arg = unsafe { (*p_list).n_expr };
                        reg_agg =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_arg) };
                        reg_distinct = reg_agg;
                        unsafe {
                            sqlite3_expr_code_expr_list(p_parse_1, p_list, reg_agg, 0,
                                1 as u8)
                        };
                    } else { n_arg = 0; reg_agg = 0; }
                    if unsafe { (*p_f).i_distinct } >= 0 && !(p_list).is_null()
                        {
                        if addr_next == 0 {
                            addr_next = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        }
                        unsafe {
                            (*p_f).i_distinct =
                                code_distinct(p_parse_1, e_distinct_type_1,
                                    unsafe { (*p_f).i_distinct }, addr_next,
                                    unsafe { &*p_list }, reg_distinct)
                        };
                    }
                    if unsafe { (*p_f).i_ob_tab } >= 0 {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_agg, reg_agg_sz - 1,
                                reg_agg + reg_agg_sz - 1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, unsafe { (*p_f).i_ob_tab },
                                reg_agg + reg_agg_sz - 1, reg_agg, reg_agg_sz - 1)
                        };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, reg_agg, reg_agg_sz)
                        };
                    } else {
                        if unsafe { (*unsafe { (*p_f).p_func }).func_flags } &
                                    32 as u32 != 0 {
                            let mut p_coll: *mut CollSeq = core::ptr::null_mut();
                            let mut p_item: *const ExprList_item = core::ptr::null();
                            let mut j: i32 = 0;
                            { let _ = 0; };
                            {
                                {
                                    j = 0;
                                    p_item =
                                        unsafe { (*p_list).a.as_ptr() } as *mut ExprList_item
                                };
                                '__b109: loop {
                                    if !((p_coll).is_null() as i32 != 0 && j < n_arg) {
                                        break '__b109;
                                    }
                                    '__c109: loop {
                                        p_coll =
                                            unsafe {
                                                sqlite3_expr_coll_seq(p_parse_1,
                                                    unsafe { (*p_item).p_expr } as *const Expr)
                                            };
                                        break '__c109;
                                    }
                                    {
                                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_item;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if (p_coll).is_null() as i32 != 0 {
                                p_coll =
                                    unsafe { (*unsafe { (*p_parse_1).db }).p_dflt_coll };
                            }
                            if reg_hit == 0 && (*p_agg_info_1).n_accumulator != 0 {
                                reg_hit =
                                    {
                                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                                        *__p += 1;
                                        *__p
                                    };
                            }
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 87, reg_hit, 0, 0,
                                    p_coll as *mut i8 as *const i8, -2)
                            };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 164, 0, reg_agg,
                                (*p_agg_info_1).i_first_reg + (*p_agg_info_1).n_column + i)
                        };
                        unsafe {
                            sqlite3_vdbe_append_p4(v,
                                unsafe { (*p_f).p_func } as *mut (), -8)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, n_arg as u16) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, reg_agg, n_arg)
                        };
                    }
                    if addr_next != 0 {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_next) };
                    }
                    if unsafe { (*p_parse_1).n_err } != 0 { return; }
                    break '__c107;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_f;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        if reg_hit == 0 && (*p_agg_info_1).n_accumulator != 0 {
            reg_hit = reg_acc_1;
        }
        if reg_hit != 0 {
            addr_hit_test = unsafe { sqlite3_vdbe_add_op1(v, 16, reg_hit) };
        }
        {
            { i = 0; p_c = (*p_agg_info_1).a_col };
            '__b110: loop {
                if !(i < (*p_agg_info_1).n_accumulator) { break '__b110; }
                '__c110: loop {
                    unsafe {
                        sqlite3_expr_code(p_parse_1, unsafe { (*p_c).p_c_expr },
                            (*p_agg_info_1).i_first_reg + i)
                    };
                    if unsafe { (*p_parse_1).n_err } != 0 { return; }
                    break '__c110;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_c;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        (*p_agg_info_1).direct_mode = 0 as u8;
        if addr_hit_test != 0 {
            unsafe { sqlite3_vdbe_jump_here_or_pop_inst(v, addr_hit_test) };
        }
    }
}
extern "C" fn finalize_agg_functions(p_parse_1: *mut Parse,
    p_agg_info_1: &AggInfo) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut p_f: *const AggInfo_func = core::ptr::null();
        {
            { i = 0; p_f = (*p_agg_info_1).a_func };
            '__b111: loop {
                if !(i < (*p_agg_info_1).n_func) { break '__b111; }
                '__c111: loop {
                    let mut p_list: *const ExprList = core::ptr::null();
                    { let _ = 0; };
                    if unsafe { (*p_parse_1).n_err } != 0 { return; }
                    p_list = unsafe { (*unsafe { (*p_f).p_f_expr }).x.p_list };
                    if unsafe { (*p_f).i_ob_tab } >= 0 {
                        let mut i_top: i32 = 0;
                        let mut n_arg: i32 = 0;
                        let mut n_key: i32 = 0;
                        let mut reg_agg: i32 = 0;
                        let mut j: i32 = 0;
                        { let _ = 0; };
                        n_arg = unsafe { (*p_list).n_expr };
                        reg_agg =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_arg) };
                        if unsafe { (*p_f).b_ob_payload } as i32 == 0 {
                            n_key = 0;
                        } else {
                            { let _ = 0; };
                            { let _ = 0; };
                            { let _ = 0; };
                            n_key =
                                unsafe {
                                    (*unsafe {
                                                    (*unsafe { (*unsafe { (*p_f).p_f_expr }).p_left }).x.p_list
                                                }).n_expr
                                };
                            if (unsafe { (*p_f).b_ob_unique } == 0) as i32 != 0 {
                                { let __p = &mut n_key; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        i_top =
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 36, unsafe { (*p_f).i_ob_tab })
                            };
                        {
                            j = n_arg - 1;
                            '__b112: loop {
                                if !(j >= 0) { break '__b112; }
                                '__c112: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 96, unsafe { (*p_f).i_ob_tab },
                                            n_key + j, reg_agg + j)
                                    };
                                    break '__c112;
                                }
                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                            }
                        }
                        if unsafe { (*p_f).b_use_subtype } != 0 {
                            let reg_subtype: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            let i_base_col: i32 =
                                n_key + n_arg +
                                    (unsafe { (*p_f).b_ob_payload } as i32 == 0 &&
                                            unsafe { (*p_f).b_ob_unique } as i32 == 0) as i32;
                            {
                                j = n_arg - 1;
                                '__b113: loop {
                                    if !(j >= 0) { break '__b113; }
                                    '__c113: loop {
                                        unsafe {
                                            sqlite3_vdbe_add_op3(v, 96, unsafe { (*p_f).i_ob_tab },
                                                i_base_col + j, reg_subtype)
                                        };
                                        unsafe {
                                            sqlite3_vdbe_add_op2(v, 184, reg_subtype, reg_agg + j)
                                        };
                                        break '__c113;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                }
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, reg_subtype) };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 164, 0, reg_agg,
                                (*p_agg_info_1).i_first_reg + (*p_agg_info_1).n_column + i)
                        };
                        unsafe {
                            sqlite3_vdbe_append_p4(v,
                                unsafe { (*p_f).p_func } as *mut (), -8)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, n_arg as u16) };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 40, unsafe { (*p_f).i_ob_tab },
                                i_top + 1)
                        };
                        unsafe { sqlite3_vdbe_jump_here(v, i_top) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, reg_agg, n_arg)
                        };
                    }
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 167,
                            (*p_agg_info_1).i_first_reg + (*p_agg_info_1).n_column + i,
                            if !(p_list).is_null() {
                                unsafe { (*p_list).n_expr }
                            } else { 0 })
                    };
                    unsafe {
                        sqlite3_vdbe_append_p4(v,
                            unsafe { (*p_f).p_func } as *mut (), -8)
                    };
                    break '__c111;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_f;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
    }
}
extern "C" fn reset_accumulator(p_parse_1: *mut Parse, p_agg_info_1: &AggInfo)
    -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut p_func: *mut AggInfo_func = core::ptr::null_mut();
        let n_reg: i32 = (*p_agg_info_1).n_func + (*p_agg_info_1).n_column;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if n_reg == 0 { return; }
        if unsafe { (*p_parse_1).n_err } != 0 { return; }
        unsafe {
            sqlite3_vdbe_add_op3(v, 77, 0, (*p_agg_info_1).i_first_reg,
                (*p_agg_info_1).i_first_reg + n_reg - 1)
        };
        {
            { p_func = (*p_agg_info_1).a_func; i = 0 };
            '__b114: loop {
                if !(i < (*p_agg_info_1).n_func) { break '__b114; }
                '__c114: loop {
                    if unsafe { (*p_func).i_distinct } >= 0 {
                        let p_e: *const Expr =
                            unsafe { (*p_func).p_f_expr } as *const Expr;
                        { let _ = 0; };
                        if unsafe { (*p_e).x.p_list } == core::ptr::null_mut() ||
                                unsafe { (*unsafe { (*p_e).x.p_list }).n_expr } != 1 {
                            unsafe {
                                sqlite3_error_msg(p_parse_1,
                                    c"DISTINCT aggregates must have exactly one argument".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                            unsafe { (*p_func).i_distinct = -1 };
                        } else {
                            let p_key_info: *mut KeyInfo =
                                sqlite3_key_info_from_expr_list(p_parse_1,
                                    unsafe { &*unsafe { (*p_e).x.p_list } }, 0, 0);
                            unsafe {
                                (*p_func).i_dist_addr =
                                    unsafe {
                                        sqlite3_vdbe_add_op4(v, 120,
                                            unsafe { (*p_func).i_distinct }, 0, 0,
                                            p_key_info as *mut i8 as *const i8, -9)
                                    }
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"USE TEMP B-TREE FOR %s(DISTINCT)".as_ptr() as *mut i8 as
                                        *const i8, unsafe { (*unsafe { (*p_func).p_func }).z_name })
                            };
                        }
                    }
                    if unsafe { (*p_func).i_ob_tab } >= 0 {
                        let mut p_ob_list: *mut ExprList = core::ptr::null_mut();
                        let mut p_key_info_1: *mut KeyInfo = core::ptr::null_mut();
                        let mut n_extra: i32 = 0;
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_ob_list =
                            unsafe {
                                (*unsafe {
                                                    (*unsafe { (*p_func).p_f_expr }).p_left
                                                }).x.p_list
                            };
                        if (unsafe { (*p_func).b_ob_unique } == 0) as i32 != 0 {
                            { let __p = &mut n_extra; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { (*p_func).b_ob_payload } != 0 {
                            { let _ = 0; };
                            { let _ = 0; };
                            n_extra +=
                                unsafe {
                                    (*unsafe {
                                                    (*unsafe { (*p_func).p_f_expr }).x.p_list
                                                }).n_expr
                                };
                        }
                        if unsafe { (*p_func).b_use_subtype } != 0 {
                            n_extra +=
                                unsafe {
                                    (*unsafe {
                                                    (*unsafe { (*p_func).p_f_expr }).x.p_list
                                                }).n_expr
                                };
                        }
                        p_key_info_1 =
                            sqlite3_key_info_from_expr_list(p_parse_1,
                                unsafe { &*p_ob_list }, 0, n_extra);
                        if (unsafe { (*p_func).b_ob_unique } == 0) as i32 != 0 &&
                                unsafe { (*p_parse_1).n_err } == 0 {
                            {
                                let __p = unsafe { &mut (*p_key_info_1).n_key_field };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 120, unsafe { (*p_func).i_ob_tab },
                                unsafe { (*p_ob_list).n_expr } + n_extra, 0,
                                p_key_info_1 as *mut i8 as *const i8, -9)
                        };
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                c"USE TEMP B-TREE FOR %s(ORDER BY)".as_ptr() as *mut i8 as
                                    *const i8, unsafe { (*unsafe { (*p_func).p_func }).z_name })
                        };
                    }
                    break '__c114;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_func;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
    }
}
extern "C" fn is_simple_count(p: &Select, p_agg_info_1: *mut AggInfo)
    -> *mut Table {
    unsafe {
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut p_expr: *const Expr = core::ptr::null();
        { let _ = 0; };
        if !((*p).p_where).is_null() ||
                                unsafe { (*(*p).p_e_list).n_expr } != 1 ||
                            unsafe { (*(*p).p_src).n_src } != 1 ||
                        unsafe {
                                (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).fg.is_subquery()
                            } != 0 || unsafe { (*p_agg_info_1).n_func } != 1 ||
                !((*p).p_having).is_null() {
            return core::ptr::null_mut();
        }
        p_tab =
            unsafe {
                (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).p_s_tab
            };
        { let _ = 0; };
        { let _ = 0; };
        if !(unsafe { (*p_tab).e_tab_type } as i32 == 0) as i32 != 0 {
            return core::ptr::null_mut();
        }
        p_expr =
            unsafe {
                (*(unsafe { (*(*p).p_e_list).a.as_ptr() } as
                                *mut ExprList_item).offset(0 as isize)).p_expr
            };
        { let _ = 0; };
        if unsafe { (*p_expr).op } as i32 != 169 {
            return core::ptr::null_mut();
        }
        if unsafe { (*p_expr).p_agg_info } != p_agg_info_1 {
            return core::ptr::null_mut();
        }
        if unsafe {
                        (*unsafe {
                                        (*unsafe {
                                                    (*p_agg_info_1).a_func.offset(0 as isize)
                                                }).p_func
                                    }).func_flags
                    } & 256 as u32 == 0 as u32 {
            return core::ptr::null_mut();
        }
        { let _ = 0; };
        if unsafe { (*p_expr).flags } & (4 | 16777216) as u32 != 0 as u32 {
            return core::ptr::null_mut();
        }
        return p_tab;
    }
}
extern "C" fn explain_simple_count(p_parse_1: *mut Parse, p_tab_1: &Table,
    p_idx_1: *const Index) -> () {
    if unsafe { (*p_parse_1).explain } as i32 == 2 {
        let b_cover: i32 =
            (p_idx_1 != core::ptr::null_mut() &&
                    ((*p_tab_1).tab_flags & 128 as u32 == 0 as u32 ||
                        !(unsafe { (*p_idx_1).idx_type() } as i32 == 2) as i32 !=
                            0)) as i32;
        unsafe {
            sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                c"SCAN %s%s%s".as_ptr() as *mut i8 as *const i8,
                (*p_tab_1).z_name,
                if b_cover != 0 {
                    c" USING COVERING INDEX ".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 },
                if b_cover != 0 {
                    unsafe { (*p_idx_1).z_name }
                } else { c"".as_ptr() as *mut i8 })
        };
    }
}
extern "C" fn explain_temp_table(p_parse_1: *mut Parse, z_usage_1: *const i8)
    -> () {
    unsafe {
        sqlite3_vdbe_explain(p_parse_1, 0 as u8,
            c"USE TEMP B-TREE FOR %s".as_ptr() as *mut i8 as *const i8,
            z_usage_1)
    };
}
extern "C" fn generate_sort_tail(p_parse_1: *mut Parse, p: &Select,
    p_sort_1: &SortCtx, mut n_column_1: i32, p_dest_1: &SelectDest) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let addr_break: i32 = (*p_sort_1).label_done;
        let addr_continue: i32 =
            unsafe { sqlite3_vdbe_make_label(p_parse_1) };
        let mut addr: i32 = 0;
        let mut addr_once: i32 = 0;
        let mut i_tab: i32 = 0;
        let p_order_by: *const ExprList =
            (*p_sort_1).p_order_by as *const ExprList;
        let e_dest: i32 = (*p_dest_1).e_dest as i32;
        let i_parm: i32 = (*p_dest_1).i_sd_parm;
        let mut reg_row: i32 = 0;
        let mut reg_rowid: i32 = 0;
        let mut i_col: i32 = 0;
        let mut n_key: i32 = 0;
        let mut i_sort_tab: i32 = 0;
        let mut i: i32 = 0;
        let mut b_seq: i32 = 0;
        let n_ref_key: i32 = 0;
        let a_out_ex: *const ExprList_item =
            unsafe { (*(*p).p_e_list).a.as_ptr() } as *const ExprList_item;
        n_key = unsafe { (*p_order_by).n_expr } - (*p_sort_1).n_ob_sat;
        if (*p_sort_1).n_ob_sat == 0 || n_key == 1 {
            unsafe {
                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                    c"USE TEMP B-TREE FOR %sORDER BY".as_ptr() as *mut i8 as
                        *const i8,
                    if (*p_sort_1).n_ob_sat != 0 {
                        c"LAST TERM OF ".as_ptr() as *mut i8
                    } else { c"".as_ptr() as *mut i8 })
            };
        } else {
            unsafe {
                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                    c"USE TEMP B-TREE FOR LAST %d TERMS OF ORDER BY".as_ptr() as
                            *mut i8 as *const i8, n_key)
            };
        }
        { let _ = 0; };
        if (*p_sort_1).label_bk_out != 0 {
            unsafe {
                sqlite3_vdbe_add_op2(v, 10, (*p_sort_1).reg_return,
                    (*p_sort_1).label_bk_out)
            };
            unsafe { sqlite3_vdbe_goto(v, addr_break) };
            unsafe {
                sqlite3_vdbe_resolve_label(v, (*p_sort_1).label_bk_out)
            };
        }
        i_tab = (*p_sort_1).i_e_cursor;
        if e_dest == 7 || e_dest == 11 || e_dest == 8 {
            if e_dest == 8 && (*p).i_offset != 0 {
                unsafe { sqlite3_vdbe_add_op2(v, 77, 0, (*p_dest_1).i_sdst) };
            }
            reg_rowid = 0;
            reg_row = (*p_dest_1).i_sdst;
        } else {
            reg_rowid = unsafe { sqlite3_get_temp_reg(p_parse_1) };
            if e_dest == 10 || e_dest == 12 {
                reg_row = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                n_column_1 = 0;
            } else {
                reg_row =
                    unsafe { sqlite3_get_temp_range(p_parse_1, n_column_1) };
            }
        }
        if (*p_sort_1).sort_flags as i32 & 1 != 0 {
            let reg_sort_out: i32 =
                {
                    let __p = unsafe { &mut (*p_parse_1).n_mem };
                    *__p += 1;
                    *__p
                };
            i_sort_tab =
                {
                    let __p = unsafe { &mut (*p_parse_1).n_tab };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            if (*p_sort_1).label_bk_out != 0 {
                addr_once = unsafe { sqlite3_vdbe_add_op0(v, 15) };
            }
            unsafe {
                sqlite3_vdbe_add_op3(v, 123, i_sort_tab, reg_sort_out,
                    n_key + 1 + n_column_1 + n_ref_key)
            };
            if addr_once != 0 {
                unsafe { sqlite3_vdbe_jump_here(v, addr_once) };
            }
            addr =
                1 + unsafe { sqlite3_vdbe_add_op2(v, 34, i_tab, addr_break) };
            { let _ = 0; };
            unsafe {
                sqlite3_vdbe_add_op3(v, 135, i_tab, reg_sort_out, i_sort_tab)
            };
            b_seq = 0;
        } else {
            addr =
                1 + unsafe { sqlite3_vdbe_add_op2(v, 35, i_tab, addr_break) };
            code_offset(v, (*p).i_offset, addr_continue);
            i_sort_tab = i_tab;
            b_seq = 1;
            if (*p).i_offset > 0 {
                unsafe { sqlite3_vdbe_add_op2(v, 88, (*p).i_limit, -1) };
            }
        }
        {
            { i = 0; i_col = n_key + b_seq - 1 };
            '__b115: loop {
                if !(i < n_column_1) { break '__b115; }
                '__c115: loop {
                    if unsafe {
                                    (*a_out_ex.offset(i as isize)).u.x.i_order_by_col
                                } as i32 == 0 {
                        { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c115;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = n_column_1 - 1;
            '__b116: loop {
                if !(i >= 0) { break '__b116; }
                '__c116: loop {
                    {
                        let mut i_read: i32 = 0;
                        if unsafe {
                                    (*a_out_ex.offset(i as isize)).u.x.i_order_by_col
                                } != 0 {
                            i_read =
                                unsafe { (*a_out_ex.offset(i as isize)).u.x.i_order_by_col }
                                        as i32 - 1;
                        } else {
                            i_read =
                                { let __p = &mut i_col; let __t = *__p; *__p -= 1; __t };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_sort_tab, i_read, reg_row + i)
                        };
                    }
                    break '__c116;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        '__s117:
            {
            match e_dest {
                12 => {
                    {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_sort_tab, n_key + b_seq,
                                reg_row)
                        };
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, reg_rowid) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 130, i_parm, reg_row, reg_rowid)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, reg_row, n_column_1, reg_rowid,
                                (*p_dest_1).z_aff_sdst as *const i8, n_column_1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, reg_rowid, reg_row,
                                n_column_1)
                        };
                        break '__s117;
                    }
                    { break '__s117; }
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                10 => {
                    {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_sort_tab, n_key + b_seq,
                                reg_row)
                        };
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, reg_rowid) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 130, i_parm, reg_row, reg_rowid)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, reg_row, n_column_1, reg_rowid,
                                (*p_dest_1).z_aff_sdst as *const i8, n_column_1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, reg_rowid, reg_row,
                                n_column_1)
                        };
                        break '__s117;
                    }
                    { break '__s117; }
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                9 => {
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, reg_row, n_column_1, reg_rowid,
                                (*p_dest_1).z_aff_sdst as *const i8, n_column_1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, reg_rowid, reg_row,
                                n_column_1)
                        };
                        break '__s117;
                    }
                    { break '__s117; }
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                8 => {
                    { break '__s117; }
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                13 => {
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                _ => {
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
            }
        }
        if reg_rowid != 0 {
            if e_dest == 9 {
                unsafe {
                    sqlite3_release_temp_range(p_parse_1, reg_row, n_column_1)
                };
            } else {
                unsafe { sqlite3_release_temp_reg(p_parse_1, reg_row) };
            }
            unsafe { sqlite3_release_temp_reg(p_parse_1, reg_rowid) };
        }
        unsafe { sqlite3_vdbe_resolve_label(v, addr_continue) };
        if (*p_sort_1).sort_flags as i32 & 1 != 0 {
            unsafe { sqlite3_vdbe_add_op2(v, 38, i_tab, addr) };
        } else { unsafe { sqlite3_vdbe_add_op2(v, 40, i_tab, addr) }; }
        if (*p_sort_1).reg_return != 0 {
            unsafe { sqlite3_vdbe_add_op1(v, 69, (*p_sort_1).reg_return) };
        }
        unsafe { sqlite3_vdbe_resolve_label(v, addr_break) };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select(p_parse: *mut Parse, p: *mut Select,
    p_dest: *mut SelectDest) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut p_w_info: *mut WhereInfo = core::ptr::null_mut();
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let mut is_agg: i32 = 0;
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
        let mut p_where: *mut Expr = core::ptr::null_mut();
        let mut p_group_by: *mut ExprList = core::ptr::null_mut();
        let mut p_having: *mut Expr = core::ptr::null_mut();
        let mut p_agg_info: *mut AggInfo = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut s_distinct: DistinctCtx = unsafe { core::mem::zeroed() };
        let mut s_sort: SortCtx = unsafe { core::mem::zeroed() };
        let mut i_end: i32 = 0;
        let mut db: *mut sqlite3 = core::ptr::null_mut();
        let mut p_min_max_order_by: *mut ExprList = core::ptr::null_mut();
        let mut min_max_flag: u8 = 0 as u8;
        let mut p0: *mut SrcItem = core::ptr::null_mut();
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        let mut p_sub: *mut Select = core::ptr::null_mut();
        let mut p_tab: *const Table = core::ptr::null();
        let mut p_i2: *mut SrcItem = core::ptr::null_mut();
        let mut p_item_1: *mut SrcItem = core::ptr::null_mut();
        let mut p_prior: *const SrcItem = core::ptr::null();
        let mut dest: SelectDest = unsafe { core::mem::zeroed() };
        let mut p_subq: *mut Subquery = core::ptr::null_mut();
        let mut p_sub_1: *mut Select = core::ptr::null_mut();
        let mut z_saved_auth_context: *const i8 = core::ptr::null();
        let mut z_db: *const i8 = core::ptr::null();
        let mut i_db: i32 = 0;
        let mut addr_top: i32 = 0;
        let mut p_cte_use: *const CteUse = core::ptr::null();
        let mut p_prior_subq: *const Subquery = core::ptr::null();
        let mut top_addr: i32 = 0;
        let mut once_addr: i32 = 0;
        let mut p_cte_use_1: *mut CteUse = core::ptr::null_mut();
        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
        let mut ii: i32 = 0;
        let mut wctrl_flags: u16 = 0 as u16;
        let mut p_win: *const Window = core::ptr::null();
        let mut addr_gosub: i32 = 0;
        let mut i_cont: i32 = 0;
        let mut i_break: i32 = 0;
        let mut reg_gosub: i32 = 0;
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        let mut i_a_mem: i32 = 0;
        let mut i_b_mem: i32 = 0;
        let mut i_use_flag: i32 = 0;
        let mut i_abort_flag: i32 = 0;
        let mut group_by_sort: i32 = 0;
        let mut addr_end: i32 = 0;
        let mut sort_p_tab: i32 = 0;
        let mut sort_out: i32 = 0;
        let mut order_by_grp: i32 = 0;
        let mut k: i32 = 0;
        let mut p_item_2: *mut ExprList_item = core::ptr::null_mut();
        let mut p_key_info_1: *mut KeyInfo = core::ptr::null_mut();
        let mut addr1: i32 = 0;
        let mut addr_output_row: i32 = 0;
        let mut reg_output_row: i32 = 0;
        let mut addr_set_abort: i32 = 0;
        let mut addr_top_of_loop: i32 = 0;
        let mut addr_sorting_idx: i32 = 0;
        let mut addr_reset: i32 = 0;
        let mut reg_reset: i32 = 0;
        let mut p_distinct: *mut ExprList = core::ptr::null_mut();
        let mut dist_flag: u16 = 0 as u16;
        let mut e_dist: i32 = 0;
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        let mut reg_base: i32 = 0;
        let mut reg_record: i32 = 0;
        let mut n_col: i32 = 0;
        let mut n_group_by: i32 = 0;
        let mut p_col: *const AggInfo_col = core::ptr::null();
        let mut i_order_by_col: i32 = 0;
        let mut p_x: *mut Expr = core::ptr::null_mut();
        let mut p_base: *const Expr = core::ptr::null();
        let mut p_f: *const AggInfo_func = core::ptr::null();
        let mut p_tab_1: *mut Table = core::ptr::null_mut();
        let mut i_db_1: i32 = 0;
        let mut i_csr: i32 = 0;
        let mut p_idx: *mut Index = core::ptr::null_mut();
        let mut p_key_info_2: *mut KeyInfo = core::ptr::null_mut();
        let mut p_best: *mut Index = core::ptr::null_mut();
        let mut i_root: Pgno = 0 as Pgno;
        let mut reg_acc: i32 = 0;
        let mut p_distinct_1: *mut ExprList = core::ptr::null_mut();
        let mut dist_flag_1: u16 = 0 as u16;
        let mut e_dist_1: i32 = 0;
        let mut p_f_1: *const AggInfo_func = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s119:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => { { let _ = 0; }; __state = 631; }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { p_e_list = core::ptr::null_mut(); __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { p_agg_info = core::ptr::null_mut(); __state = 12; }
                    12 => { rc = 1; __state = 13; }
                    13 => { __state = 14; }
                    14 => { __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => {
                        p_min_max_order_by = core::ptr::null_mut();
                        __state = 18;
                    }
                    18 => { __state = 19; }
                    19 => { db = unsafe { (*p_parse).db }; __state = 20; }
                    20 => { { let _ = 0; }; __state = 21; }
                    21 => { v = sqlite3_get_vdbe(p_parse); __state = 22; }
                    22 => {
                        if p == core::ptr::null_mut() ||
                                unsafe { (*p_parse).n_err } != 0 {
                            __state = 24;
                        } else { __state = 23; }
                    }
                    23 => { { let _ = 0; }; __state = 25; }
                    24 => { return 1; }
                    25 => {
                        if unsafe {
                                    sqlite3_auth_check(p_parse, 21, core::ptr::null(),
                                        core::ptr::null(), core::ptr::null())
                                } != 0 {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    26 => { { let _ = 0; }; __state = 28; }
                    27 => { return 1; }
                    28 => { { let _ = 0; }; __state = 29; }
                    29 => { { let _ = 0; }; __state = 30; }
                    30 => { { let _ = 0; }; __state = 31; }
                    31 => {
                        if unsafe { (*p_dest).e_dest } as i32 <= 4 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        sqlite3_select_prep(p_parse, p, core::ptr::null_mut());
                        __state = 39;
                    }
                    33 => { { let _ = 0; }; __state = 34; }
                    34 => {
                        if !(unsafe { (*p).p_order_by }).is_null() {
                            __state = 36;
                        } else { __state = 35; }
                    }
                    35 => {
                        unsafe { (*p).sel_flags &= !(1 as u32) };
                        __state = 32;
                    }
                    36 => {
                        unsafe {
                            sqlite3_parser_add_cleanup(p_parse,
                                Some(sqlite3_expr_list_delete_generic),
                                unsafe { (*p).p_order_by } as *mut ())
                        };
                        __state = 37;
                    }
                    37 => { __state = 38; }
                    38 => {
                        unsafe { (*p).p_order_by = core::ptr::null_mut() };
                        __state = 35;
                    }
                    39 => {
                        if unsafe { (*p_parse).n_err } != 0 {
                            __state = 41;
                        } else { __state = 40; }
                    }
                    40 => { { let _ = 0; }; __state = 42; }
                    41 => { __state = 2; }
                    42 => { { let _ = 0; }; __state = 43; }
                    43 => {
                        if unsafe { (*p).sel_flags } & 8388608 as u32 != 0 {
                            __state = 45;
                        } else { __state = 44; }
                    }
                    44 => {
                        if unsafe { (*p_dest).e_dest } as i32 == 7 {
                            __state = 51;
                        } else { __state = 50; }
                    }
                    45 => {
                        p0 =
                            unsafe {
                                &mut *(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)
                            };
                        __state = 46;
                    }
                    46 => {
                        if same_src_alias(p0,
                                    unsafe { &mut *unsafe { (*p).p_src } }) != 0 {
                            __state = 48;
                        } else { __state = 47; }
                    }
                    47 => {
                        unsafe { (*p).sel_flags &= !(8388608 as u32) };
                        __state = 44;
                    }
                    48 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"target object/alias may not appear in FROM clause: %s".as_ptr()
                                        as *mut i8 as *const i8,
                                if !(unsafe { (*p0).z_alias }).is_null() {
                                    unsafe { (*p0).z_alias }
                                } else { unsafe { (*unsafe { (*p0).p_s_tab }).z_name } })
                        };
                        __state = 49;
                    }
                    49 => { __state = 2; }
                    50 => {
                        if unsafe { sqlite3_window_rewrite(p_parse, p) } != 0 {
                            __state = 53;
                        } else { __state = 52; }
                    }
                    51 => {
                        sqlite3_generate_column_names(p_parse, p);
                        __state = 50;
                    }
                    52 => { p_tab_list = unsafe { (*p).p_src }; __state = 55; }
                    53 => { { let _ = 0; }; __state = 54; }
                    54 => { __state = 2; }
                    55 => {
                        is_agg =
                            (unsafe { (*p).sel_flags } & 8 as u32 != 0 as u32) as i32;
                        __state = 56;
                    }
                    56 => {
                        unsafe {
                            memset(&raw mut s_sort as *mut (), 0,
                                core::mem::size_of::<SortCtx>() as u64)
                        };
                        __state = 57;
                    }
                    57 => {
                        s_sort.p_order_by = unsafe { (*p).p_order_by };
                        __state = 58;
                    }
                    58 => { i = 0; __state = 60; }
                    59 => {
                        if !(unsafe { (*p).p_prior }).is_null() {
                            __state = 118;
                        } else { __state = 117; }
                    }
                    60 => {
                        if (unsafe { (*p).p_prior }).is_null() as i32 != 0 &&
                                i < unsafe { (*p_tab_list).n_src } {
                            __state = 61;
                        } else { __state = 59; }
                    }
                    61 => {
                        p_item =
                            unsafe {
                                &mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)
                            };
                        __state = 63;
                    }
                    62 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 60;
                    }
                    63 => {
                        p_sub =
                            if unsafe { (*p_item).fg.is_subquery() } != 0 {
                                unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select }
                            } else { core::ptr::null_mut() };
                        __state = 64;
                    }
                    64 => {
                        p_tab = unsafe { (*p_item).p_s_tab } as *const Table;
                        __state = 65;
                    }
                    65 => { { let _ = 0; }; __state = 66; }
                    66 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & (8 | 64) != 0
                                    &&
                                    unsafe {
                                            sqlite3_expr_implies_non_null_row(unsafe { (*p).p_where },
                                                unsafe { (*p_item).i_cursor },
                                                unsafe { (*p_item).fg.jointype } as i32 & 64)
                                        } != 0 &&
                                unsafe { (*db).db_opt_flags } & 8192 as u32 == 0 as u32 {
                            __state = 68;
                        } else { __state = 67; }
                    }
                    67 => {
                        if p_sub == core::ptr::null_mut() {
                            __state = 94;
                        } else { __state = 93; }
                    }
                    68 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 8 != 0 {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    69 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 64 != 0 {
                            __state = 76;
                        } else { __state = 67; }
                    }
                    70 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 16 != 0 {
                            __state = 71;
                        } else { __state = 72; }
                    }
                    71 => { __state = 73; }
                    72 => { __state = 74; }
                    73 => {
                        unsafe { (*p_item).fg.jointype &= !8 as u8 };
                        __state = 69;
                    }
                    74 => {
                        unsafe { (*p_item).fg.jointype &= !(8 | 32) as u8 };
                        __state = 75;
                    }
                    75 => {
                        unset_join_expr(unsafe { (*p).p_where },
                            unsafe { (*p_item).i_cursor }, 0);
                        __state = 69;
                    }
                    76 => { j = i + 1; __state = 78; }
                    77 => {
                        j = unsafe { (*p_tab_list).n_src } - 1;
                        __state = 88;
                    }
                    78 => {
                        if j < unsafe { (*p_tab_list).n_src } {
                            __state = 79;
                        } else { __state = 77; }
                    }
                    79 => {
                        p_i2 =
                            unsafe {
                                &mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(j as isize)
                            };
                        __state = 81;
                    }
                    80 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 78;
                    }
                    81 => {
                        if unsafe { (*p_i2).fg.jointype } as i32 & 16 != 0 {
                            __state = 82;
                        } else { __state = 80; }
                    }
                    82 => {
                        if unsafe { (*p_i2).fg.jointype } as i32 & 8 != 0 {
                            __state = 83;
                        } else { __state = 84; }
                    }
                    83 => { __state = 85; }
                    84 => { __state = 86; }
                    85 => {
                        unsafe { (*p_i2).fg.jointype &= !16 as u8 };
                        __state = 80;
                    }
                    86 => {
                        unsafe { (*p_i2).fg.jointype &= !(16 | 32) as u8 };
                        __state = 87;
                    }
                    87 => {
                        unset_join_expr(unsafe { (*p).p_where },
                            unsafe { (*p_i2).i_cursor }, 1);
                        __state = 80;
                    }
                    88 => { if j >= 0 { __state = 89; } else { __state = 67; } }
                    89 => {
                        unsafe {
                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                    *mut SrcItem).offset(j as isize)).fg.jointype &= !64 as u8
                        };
                        __state = 91;
                    }
                    90 => {
                        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                        __state = 88;
                    }
                    91 => {
                        if unsafe {
                                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                *mut SrcItem).offset(j as isize)).fg.jointype
                                        } as i32 & 16 != 0 {
                            __state = 92;
                        } else { __state = 90; }
                    }
                    92 => { __state = 67; }
                    93 => {
                        if unsafe { (*p_tab).n_col } as i32 !=
                                unsafe { (*unsafe { (*p_sub).p_e_list }).n_expr } {
                            __state = 96;
                        } else { __state = 95; }
                    }
                    94 => { __state = 62; }
                    95 => {
                        if unsafe { (*p_item).fg.is_cte() } != 0 &&
                                unsafe { (*unsafe { (*p_item).u2.p_cte_use }).e_m10d } as
                                        i32 == 0 {
                            __state = 99;
                        } else { __state = 98; }
                    }
                    96 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"expected %d columns for \'%s\' but got %d".as_ptr() as
                                        *mut i8 as *const i8, unsafe { (*p_tab).n_col } as i32,
                                unsafe { (*p_tab).z_name },
                                unsafe { (*unsafe { (*p_sub).p_e_list }).n_expr })
                        };
                        __state = 97;
                    }
                    97 => { __state = 2; }
                    98 => {
                        if unsafe { (*p_sub).sel_flags } & 8 as u32 != 0 as u32 {
                            __state = 101;
                        } else { __state = 100; }
                    }
                    99 => { __state = 62; }
                    100 => { { let _ = 0; }; __state = 102; }
                    101 => { __state = 62; }
                    102 => {
                        if unsafe { (*p_sub).p_order_by } != core::ptr::null_mut()
                                                &&
                                                (unsafe { (*p).p_order_by } != core::ptr::null_mut() ||
                                                    unsafe { (*p_tab_list).n_src } > 1) &&
                                            unsafe { (*p_sub).p_limit } == core::ptr::null_mut() &&
                                        unsafe { (*p_sub).sel_flags } & (134217728 | 8192) as u32 ==
                                            0 as u32 &&
                                    unsafe { (*p).sel_flags } & 134217728 as u32 == 0 as u32 &&
                                unsafe { (*db).db_opt_flags } & 262144 as u32 == 0 as u32 {
                            __state = 104;
                        } else { __state = 103; }
                    }
                    103 => {
                        if unsafe { (*p_sub).p_order_by } != core::ptr::null_mut()
                                        && i == 0 &&
                                    unsafe { (*p).sel_flags } & 262144 as u32 != 0 as u32 &&
                                (unsafe { (*p_tab_list).n_src } == 1 ||
                                    unsafe {
                                                    (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                        *mut SrcItem).offset(1 as isize)).fg.jointype
                                                } as i32 & (32 | 2) != 0) {
                            __state = 108;
                        } else { __state = 107; }
                    }
                    104 => { __state = 105; }
                    105 => {
                        unsafe {
                            sqlite3_parser_add_cleanup(p_parse,
                                Some(sqlite3_expr_list_delete_generic),
                                unsafe { (*p_sub).p_order_by } as *mut ())
                        };
                        __state = 106;
                    }
                    106 => {
                        unsafe { (*p_sub).p_order_by = core::ptr::null_mut() };
                        __state = 103;
                    }
                    107 => {
                        if flatten_subquery(p_parse, p, i, is_agg) != 0 {
                            __state = 110;
                        } else { __state = 109; }
                    }
                    108 => { __state = 62; }
                    109 => {
                        p_tab_list = unsafe { (*p).p_src };
                        __state = 113;
                    }
                    110 => {
                        if unsafe { (*p_parse).n_err } != 0 {
                            __state = 112;
                        } else { __state = 111; }
                    }
                    111 => { i = -1; __state = 109; }
                    112 => { __state = 2; }
                    113 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 115;
                        } else { __state = 114; }
                    }
                    114 => {
                        if !(unsafe { (*p_dest).e_dest } as i32 <= 6) as i32 != 0 {
                            __state = 116;
                        } else { __state = 62; }
                    }
                    115 => { __state = 2; }
                    116 => {
                        s_sort.p_order_by = unsafe { (*p).p_order_by };
                        __state = 62;
                    }
                    117 => {
                        if unsafe { (*p_parse).b_has_exists() } != 0 &&
                                unsafe { (*db).db_opt_flags } & 1073741824 as u32 ==
                                    0 as u32 {
                            __state = 123;
                        } else { __state = 122; }
                    }
                    118 => {
                        rc = multi_select(p_parse, p, p_dest);
                        __state = 119;
                    }
                    119 => {
                        if unsafe { (*p).p_next } == core::ptr::null_mut() {
                            __state = 121;
                        } else { __state = 120; }
                    }
                    120 => { return rc; }
                    121 => {
                        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
                        __state = 120;
                    }
                    122 => {
                        if unsafe { (*p).p_where } != core::ptr::null_mut() &&
                                        unsafe { (*unsafe { (*p).p_where }).op } as i32 == 44 &&
                                    unsafe { (*db).db_opt_flags } & 32768 as u32 == 0 as u32 &&
                                propagate_constants(p_parse, unsafe { &*p }) != 0 {
                            __state = 126;
                        } else { __state = 127; }
                    }
                    123 => {
                        exists_to_join(p_parse, p, unsafe { (*p).p_where });
                        __state = 124;
                    }
                    124 => {
                        p_tab_list = unsafe { (*p).p_src };
                        __state = 122;
                    }
                    125 => {
                        if unsafe { (*db).db_opt_flags } & (1 | 512) as u32 ==
                                    0 as u32 &&
                                count_of_view_optimization(p_parse, unsafe { &mut *p }) != 0
                            {
                            __state = 129;
                        } else { __state = 128; }
                    }
                    126 => { __state = 125; }
                    127 => { __state = 125; }
                    128 => { i = 0; __state = 133; }
                    129 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 131;
                        } else { __state = 130; }
                    }
                    130 => {
                        p_tab_list = unsafe { (*p).p_src };
                        __state = 128;
                    }
                    131 => { __state = 2; }
                    132 => {
                        p_e_list = unsafe { (*p).p_e_list };
                        __state = 229;
                    }
                    133 => {
                        if i < unsafe { (*p_tab_list).n_src } {
                            __state = 134;
                        } else { __state = 132; }
                    }
                    134 => {
                        p_item_1 =
                            unsafe {
                                &mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)
                            };
                        __state = 136;
                    }
                    135 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 133;
                    }
                    136 => { __state = 137; }
                    137 => { __state = 138; }
                    138 => { __state = 139; }
                    139 => { __state = 140; }
                    140 => { __state = 141; }
                    141 => {
                        if unsafe { (*p_item_1).col_used } == 0 as u64 &&
                                unsafe { (*p_item_1).z_name } != core::ptr::null_mut() {
                            __state = 143;
                        } else { __state = 142; }
                    }
                    142 => {
                        if unsafe { (*p_item_1).fg.is_subquery() } as i32 == 0 {
                            __state = 152;
                        } else { __state = 151; }
                    }
                    143 => { __state = 144; }
                    144 => {
                        if unsafe { (*p_item_1).fg.fixed_schema() } != 0 {
                            __state = 146;
                        } else { __state = 147; }
                    }
                    145 => {
                        unsafe {
                            sqlite3_auth_check(p_parse, 20,
                                unsafe { (*p_item_1).z_name } as *const i8,
                                c"".as_ptr() as *mut i8 as *const i8, z_db)
                        };
                        __state = 142;
                    }
                    146 => {
                        i_db =
                            unsafe {
                                sqlite3_schema_to_index(unsafe { (*p_parse).db },
                                    unsafe { (*p_item_1).u4.p_schema })
                            };
                        __state = 148;
                    }
                    147 => {
                        if unsafe { (*p_item_1).fg.is_subquery() } != 0 {
                            __state = 149;
                        } else { __state = 150; }
                    }
                    148 => {
                        z_db =
                            unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                } as *const i8;
                        __state = 145;
                    }
                    149 => { z_db = core::ptr::null(); __state = 145; }
                    150 => {
                        z_db = unsafe { (*p_item_1).u4.z_database } as *const i8;
                        __state = 145;
                    }
                    151 => {
                        p_subq = unsafe { (*p_item_1).u4.p_subq };
                        __state = 153;
                    }
                    152 => { __state = 135; }
                    153 => { { let _ = 0; }; __state = 154; }
                    154 => {
                        p_sub_1 = unsafe { (*p_subq).p_select };
                        __state = 155;
                    }
                    155 => {
                        if unsafe { (*p_subq).addr_fill_sub } != 0 {
                            __state = 157;
                        } else { __state = 156; }
                    }
                    156 => {
                        unsafe {
                            (*p_parse).n_height +=
                                unsafe { sqlite3_select_expr_height(p as *const Select) }
                        };
                        __state = 158;
                    }
                    157 => { __state = 135; }
                    158 => {
                        if unsafe { (*db).db_opt_flags } & 4096 as u32 == 0 as u32
                                    &&
                                    (unsafe { (*p_item_1).fg.is_cte() } as i32 == 0 ||
                                        unsafe { (*unsafe { (*p_item_1).u2.p_cte_use }).e_m10d } as
                                                    i32 != 0 &&
                                            unsafe { (*unsafe { (*p_item_1).u2.p_cte_use }).n_use } < 2)
                                &&
                                push_down_where_terms(p_parse, p_sub_1,
                                        unsafe { (*p).p_where }, p_tab_list, i) != 0 {
                            __state = 160;
                        } else { __state = 161; }
                    }
                    159 => {
                        if unsafe { (*db).db_opt_flags } & 67108864 as u32 ==
                                    0 as u32 &&
                                disable_unused_subquery_result_columns(unsafe {
                                            &*p_item_1
                                        }) != 0 {
                            __state = 163;
                        } else { __state = 162; }
                    }
                    160 => { { let _ = 0; }; __state = 159; }
                    161 => { __state = 159; }
                    162 => {
                        z_saved_auth_context = unsafe { (*p_parse).z_auth_context };
                        __state = 164;
                    }
                    163 => { __state = 162; }
                    164 => {
                        unsafe {
                            (*p_parse).z_auth_context =
                                unsafe { (*p_item_1).z_name } as *const i8
                        };
                        __state = 165;
                    }
                    165 => {
                        if from_clause_term_can_be_coroutine(unsafe { &*p_parse },
                                    p_tab_list, i, unsafe { (*p).sel_flags } as i32) != 0 {
                            __state = 167;
                        } else { __state = 168; }
                    }
                    166 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 227;
                        } else { __state = 226; }
                    }
                    167 => {
                        addr_top = unsafe { sqlite3_vdbe_current_addr(v) } + 1;
                        __state = 169;
                    }
                    168 => {
                        if unsafe { (*p_item_1).fg.is_cte() } != 0 &&
                                unsafe { (*unsafe { (*p_item_1).u2.p_cte_use }).addr_m9e } >
                                    0 {
                            __state = 183;
                        } else { __state = 184; }
                    }
                    169 => {
                        unsafe {
                            (*p_subq).reg_return =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 170;
                    }
                    170 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 11, unsafe { (*p_subq).reg_return },
                                0, addr_top)
                        };
                        __state = 171;
                    }
                    171 => { __state = 172; }
                    172 => {
                        unsafe { (*p_subq).addr_fill_sub = addr_top };
                        __state = 173;
                    }
                    173 => {
                        sqlite3_select_dest_init(&mut dest, 11,
                            unsafe { (*p_subq).reg_return });
                        __state = 174;
                    }
                    174 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 1 as u8,
                                c"CO-ROUTINE %!S".as_ptr() as *mut i8 as *const i8,
                                p_item_1)
                        };
                        __state = 175;
                    }
                    175 => {
                        sqlite3_select(p_parse, p_sub_1, &mut dest);
                        __state = 176;
                    }
                    176 => {
                        unsafe {
                            (*unsafe { (*p_item_1).p_s_tab }).n_row_log_est =
                                unsafe { (*p_sub_1).n_select_row }
                        };
                        __state = 177;
                    }
                    177 => {
                        unsafe {
                            (*p_item_1).fg.set_via_coroutine(1 as u32 as u32)
                        };
                        __state = 178;
                    }
                    178 => {
                        unsafe { (*p_subq).reg_result = dest.i_sdst };
                        __state = 179;
                    }
                    179 => {
                        unsafe {
                            sqlite3_vdbe_end_coroutine(v,
                                unsafe { (*p_subq).reg_return })
                        };
                        __state = 180;
                    }
                    180 => { __state = 181; }
                    181 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_top - 1) };
                        __state = 182;
                    }
                    182 => {
                        unsafe { sqlite3_clear_temp_reg_cache(p_parse) };
                        __state = 166;
                    }
                    183 => {
                        p_cte_use =
                            unsafe { (*p_item_1).u2.p_cte_use } as *const CteUse;
                        __state = 185;
                    }
                    184 => {
                        if {
                                    p_prior =
                                        is_self_join_view(unsafe { &mut *p_tab_list },
                                            unsafe { &*p_item_1 }, 0, i);
                                    p_prior
                                } != core::ptr::null_mut() {
                            __state = 190;
                        } else { __state = 191; }
                    }
                    185 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, unsafe { (*p_cte_use).reg_rtn },
                                unsafe { (*p_cte_use).addr_m9e })
                        };
                        __state = 186;
                    }
                    186 => {
                        if unsafe { (*p_item_1).i_cursor } !=
                                unsafe { (*p_cte_use).i_cur } {
                            __state = 188;
                        } else { __state = 187; }
                    }
                    187 => {
                        unsafe {
                            (*p_sub_1).n_select_row = unsafe { (*p_cte_use).n_row_est }
                        };
                        __state = 166;
                    }
                    188 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 117,
                                unsafe { (*p_item_1).i_cursor },
                                unsafe { (*p_cte_use).i_cur })
                        };
                        __state = 189;
                    }
                    189 => { __state = 187; }
                    190 => { __state = 192; }
                    191 => { __state = 199; }
                    192 => { { let _ = 0; }; __state = 193; }
                    193 => {
                        p_prior_subq = unsafe { (*p_prior).u4.p_subq };
                        __state = 194;
                    }
                    194 => { { let _ = 0; }; __state = 195; }
                    195 => {
                        if unsafe { (*p_prior_subq).addr_fill_sub } != 0 {
                            __state = 197;
                        } else { __state = 196; }
                    }
                    196 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 117,
                                unsafe { (*p_item_1).i_cursor },
                                unsafe { (*p_prior).i_cursor })
                        };
                        __state = 198;
                    }
                    197 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10,
                                unsafe { (*p_prior_subq).reg_return },
                                unsafe { (*p_prior_subq).addr_fill_sub })
                        };
                        __state = 196;
                    }
                    198 => {
                        unsafe {
                            (*p_sub_1).n_select_row =
                                unsafe {
                                    (*unsafe { (*p_prior_subq).p_select }).n_select_row
                                }
                        };
                        __state = 166;
                    }
                    199 => { once_addr = 0; __state = 200; }
                    200 => {
                        unsafe {
                            (*p_subq).reg_return =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 201;
                    }
                    201 => {
                        top_addr = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                        __state = 202;
                    }
                    202 => {
                        unsafe { (*p_subq).addr_fill_sub = top_addr + 1 };
                        __state = 203;
                    }
                    203 => {
                        unsafe {
                            (*p_item_1).fg.set_is_materialized(1 as u32 as u32)
                        };
                        __state = 204;
                    }
                    204 => {
                        if unsafe { (*p_item_1).fg.is_correlated() } as i32 == 0 {
                            __state = 206;
                        } else { __state = 207; }
                    }
                    205 => {
                        sqlite3_select_dest_init(&mut dest, 10,
                            unsafe { (*p_item_1).i_cursor });
                        __state = 210;
                    }
                    206 => {
                        once_addr = unsafe { sqlite3_vdbe_add_op0(v, 15) };
                        __state = 208;
                    }
                    207 => { __state = 205; }
                    208 => { __state = 209; }
                    209 => { __state = 205; }
                    210 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 1 as u8,
                                c"MATERIALIZE %!S".as_ptr() as *mut i8 as *const i8,
                                p_item_1)
                        };
                        __state = 211;
                    }
                    211 => {
                        sqlite3_select(p_parse, p_sub_1, &mut dest);
                        __state = 212;
                    }
                    212 => {
                        unsafe {
                            (*unsafe { (*p_item_1).p_s_tab }).n_row_log_est =
                                unsafe { (*p_sub_1).n_select_row }
                        };
                        __state = 213;
                    }
                    213 => {
                        if once_addr != 0 { __state = 215; } else { __state = 214; }
                    }
                    214 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 69, unsafe { (*p_subq).reg_return },
                                top_addr + 1)
                        };
                        __state = 216;
                    }
                    215 => {
                        unsafe { sqlite3_vdbe_jump_here(v, once_addr) };
                        __state = 214;
                    }
                    216 => { __state = 217; }
                    217 => { __state = 218; }
                    218 => {
                        unsafe { sqlite3_vdbe_jump_here(v, top_addr) };
                        __state = 219;
                    }
                    219 => {
                        unsafe { sqlite3_clear_temp_reg_cache(p_parse) };
                        __state = 220;
                    }
                    220 => {
                        if unsafe { (*p_item_1).fg.is_cte() } != 0 &&
                                unsafe { (*p_item_1).fg.is_correlated() } as i32 == 0 {
                            __state = 221;
                        } else { __state = 166; }
                    }
                    221 => {
                        p_cte_use_1 = unsafe { (*p_item_1).u2.p_cte_use };
                        __state = 222;
                    }
                    222 => {
                        unsafe {
                            (*p_cte_use_1).addr_m9e = unsafe { (*p_subq).addr_fill_sub }
                        };
                        __state = 223;
                    }
                    223 => {
                        unsafe {
                            (*p_cte_use_1).reg_rtn = unsafe { (*p_subq).reg_return }
                        };
                        __state = 224;
                    }
                    224 => {
                        unsafe {
                            (*p_cte_use_1).i_cur = unsafe { (*p_item_1).i_cursor }
                        };
                        __state = 225;
                    }
                    225 => {
                        unsafe {
                            (*p_cte_use_1).n_row_est =
                                unsafe { (*p_sub_1).n_select_row }
                        };
                        __state = 166;
                    }
                    226 => {
                        unsafe {
                            (*p_parse).n_height -=
                                unsafe { sqlite3_select_expr_height(p as *const Select) }
                        };
                        __state = 228;
                    }
                    227 => { __state = 2; }
                    228 => {
                        unsafe { (*p_parse).z_auth_context = z_saved_auth_context };
                        __state = 135;
                    }
                    229 => { p_where = unsafe { (*p).p_where }; __state = 230; }
                    230 => {
                        p_group_by = unsafe { (*p).p_group_by };
                        __state = 231;
                    }
                    231 => {
                        p_having = unsafe { (*p).p_having };
                        __state = 232;
                    }
                    232 => {
                        s_distinct.is_tnct =
                            (unsafe { (*p).sel_flags } & 1 as u32 != 0 as u32) as u8;
                        __state = 233;
                    }
                    233 => {
                        if unsafe { (*p).sel_flags } & (1 | 8) as u32 == 1 as u32 &&
                                            sqlite3_copy_sort_order(unsafe { &mut *p_e_list },
                                                    s_sort.p_order_by as *const ExprList) != 0 &&
                                        unsafe {
                                                sqlite3_expr_list_compare(p_e_list as *const ExprList,
                                                    s_sort.p_order_by as *const ExprList, -1)
                                            } == 0 &&
                                    unsafe { (*db).db_opt_flags } & 4 as u32 == 0 as u32 &&
                                unsafe { (*p).p_win } == core::ptr::null_mut() {
                            __state = 235;
                        } else { __state = 234; }
                    }
                    234 => {
                        if !(s_sort.p_order_by).is_null() {
                            __state = 246;
                        } else { __state = 247; }
                    }
                    235 => {
                        unsafe { (*p).sel_flags &= !(1 as u32) };
                        __state = 236;
                    }
                    236 => {
                        p_group_by =
                            {
                                unsafe {
                                    (*p).p_group_by =
                                        unsafe {
                                            sqlite3_expr_list_dup(db, p_e_list as *const ExprList, 0)
                                        }
                                };
                                unsafe { (*p).p_group_by }
                            };
                        __state = 237;
                    }
                    237 => {
                        if !(p_group_by).is_null() {
                            __state = 239;
                        } else { __state = 238; }
                    }
                    238 => {
                        unsafe { (*p).sel_flags |= 8 as u32 };
                        __state = 243;
                    }
                    239 => { i = 0; __state = 240; }
                    240 => {
                        if i < unsafe { (*p_group_by).n_expr } {
                            __state = 241;
                        } else { __state = 238; }
                    }
                    241 => {
                        unsafe {
                            (*(unsafe { (*p_group_by).a.as_ptr() } as
                                                        *mut ExprList_item).offset(i as isize)).u.x.i_order_by_col =
                                (i + 1) as u16
                        };
                        __state = 242;
                    }
                    242 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 240;
                    }
                    243 => { { let _ = 0; }; __state = 244; }
                    244 => { s_distinct.is_tnct = 2 as u8; __state = 234; }
                    245 => {
                        if unsafe { (*p_dest).e_dest } as i32 == 10 {
                            __state = 252;
                        } else { __state = 251; }
                    }
                    246 => { __state = 248; }
                    247 => { s_sort.addr_sort_index = -1; __state = 245; }
                    248 => {
                        p_key_info =
                            sqlite3_key_info_from_expr_list(p_parse,
                                unsafe { &*s_sort.p_order_by }, 0,
                                unsafe { (*p_e_list).n_expr });
                        __state = 249;
                    }
                    249 => {
                        s_sort.i_e_cursor =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 250;
                    }
                    250 => {
                        s_sort.addr_sort_index =
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 120, s_sort.i_e_cursor,
                                    unsafe { (*s_sort.p_order_by).n_expr } + 1 +
                                        unsafe { (*p_e_list).n_expr }, 0,
                                    p_key_info as *mut i8 as *const i8, -9)
                            };
                        __state = 245;
                    }
                    251 => {
                        i_end = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 266;
                    }
                    252 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 120, unsafe { (*p_dest).i_sd_parm },
                                unsafe { (*p_e_list).n_expr })
                        };
                        __state = 253;
                    }
                    253 => {
                        if unsafe { (*p).sel_flags } & 2048 as u32 != 0 {
                            __state = 254;
                        } else { __state = 251; }
                    }
                    254 => { __state = 255; }
                    255 => {
                        ii = unsafe { (*p_e_list).n_expr } - 1;
                        __state = 257;
                    }
                    256 => { ii = 0; __state = 262; }
                    257 => {
                        if ii > 0 &&
                                unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                *mut ExprList_item).offset(ii as isize)).fg.b_used()
                                        } as i32 == 0 {
                            __state = 258;
                        } else { __state = 256; }
                    }
                    258 => {
                        unsafe {
                            sqlite3_expr_delete(db,
                                unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                    *mut ExprList_item).offset(ii as isize)).p_expr
                                })
                        };
                        __state = 260;
                    }
                    259 => {
                        { let __p = &mut ii; let __t = *__p; *__p -= 1; __t };
                        __state = 257;
                    }
                    260 => {
                        unsafe {
                            sqlite3_db_free(db,
                                unsafe {
                                        (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                        *mut ExprList_item).offset(ii as isize)).z_e_name
                                    } as *mut ())
                        };
                        __state = 261;
                    }
                    261 => {
                        {
                            let __p = unsafe { &mut (*p_e_list).n_expr };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 259;
                    }
                    262 => {
                        if ii < unsafe { (*p_e_list).n_expr } {
                            __state = 263;
                        } else { __state = 251; }
                    }
                    263 => {
                        if unsafe {
                                        (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprList_item).offset(ii as isize)).fg.b_used()
                                    } as i32 == 0 {
                            __state = 265;
                        } else { __state = 264; }
                    }
                    264 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 262;
                    }
                    265 => {
                        unsafe {
                            (*unsafe {
                                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                *mut ExprList_item).offset(ii as isize)).p_expr
                                            }).op = 122 as u8
                        };
                        __state = 264;
                    }
                    266 => {
                        if unsafe { (*p).sel_flags } & 16384 as u32 == 0 as u32 {
                            __state = 268;
                        } else { __state = 267; }
                    }
                    267 => {
                        if !(unsafe { (*p).p_limit }).is_null() {
                            __state = 270;
                        } else { __state = 269; }
                    }
                    268 => {
                        unsafe { (*p).n_select_row = 320 as LogEst };
                        __state = 267;
                    }
                    269 => {
                        if unsafe { (*p).i_limit } == 0 &&
                                s_sort.addr_sort_index >= 0 {
                            __state = 272;
                        } else { __state = 271; }
                    }
                    270 => {
                        compute_limit_registers(p_parse, unsafe { &mut *p }, i_end);
                        __state = 269;
                    }
                    271 => {
                        if unsafe { (*p).sel_flags } & 1 as u32 != 0 {
                            __state = 275;
                        } else { __state = 276; }
                    }
                    272 => {
                        unsafe {
                            sqlite3_vdbe_change_opcode(v, s_sort.addr_sort_index,
                                121 as u8)
                        };
                        __state = 273;
                    }
                    273 => { s_sort.sort_flags |= 1 as u8; __state = 271; }
                    274 => {
                        if (is_agg == 0) as i32 != 0 &&
                                p_group_by == core::ptr::null_mut() {
                            __state = 281;
                        } else { __state = 282; }
                    }
                    275 => {
                        s_distinct.tab_tnct =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 277;
                    }
                    276 => { s_distinct.e_tnct_type = 0 as u8; __state = 274; }
                    277 => {
                        s_distinct.addr_tnct =
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 120, s_distinct.tab_tnct, 0, 0,
                                    sqlite3_key_info_from_expr_list(p_parse,
                                                unsafe { &*unsafe { (*p).p_e_list } }, 0, 0) as *mut i8 as
                                        *const i8, -9)
                            };
                        __state = 278;
                    }
                    278 => {
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        __state = 279;
                    }
                    279 => { s_distinct.e_tnct_type = 3 as u8; __state = 274; }
                    280 => {
                        if s_distinct.e_tnct_type as i32 == 3 {
                            __state = 625;
                        } else { __state = 624; }
                    }
                    281 => {
                        wctrl_flags =
                            (if s_distinct.is_tnct != 0 { 256 } else { 0 } as u32 |
                                    unsafe { (*p).sel_flags } & 16384 as u32) as u16;
                        __state = 283;
                    }
                    282 => { __state = 324; }
                    283 => {
                        p_win = unsafe { (*p).p_win } as *const Window;
                        __state = 284;
                    }
                    284 => {
                        if !(p_win).is_null() {
                            __state = 286;
                        } else { __state = 285; }
                    }
                    285 => { { let _ = 0; }; __state = 287; }
                    286 => {
                        unsafe { sqlite3_window_code_init(p_parse, p) };
                        __state = 285;
                    }
                    287 => { __state = 288; }
                    288 => {
                        p_w_info =
                            unsafe {
                                sqlite3_where_begin(p_parse, p_tab_list, p_where,
                                    s_sort.p_order_by, unsafe { (*p).p_e_list }, p, wctrl_flags,
                                    unsafe { (*p).n_select_row } as i32)
                            };
                        __state = 289;
                    }
                    289 => {
                        if p_w_info == core::ptr::null_mut() {
                            __state = 291;
                        } else { __state = 290; }
                    }
                    290 => {
                        if (unsafe { sqlite3_where_output_row_count(p_w_info) } as
                                        i32) < unsafe { (*p).n_select_row } as i32 {
                            __state = 293;
                        } else { __state = 292; }
                    }
                    291 => { __state = 2; }
                    292 => {
                        if s_distinct.is_tnct != 0 &&
                                unsafe { sqlite3_where_is_distinct(p_w_info) } != 0 {
                            __state = 297;
                        } else { __state = 296; }
                    }
                    293 => {
                        unsafe {
                            (*p).n_select_row =
                                unsafe { sqlite3_where_output_row_count(p_w_info) }
                        };
                        __state = 294;
                    }
                    294 => {
                        if unsafe { (*p_dest).e_dest } as i32 <= 4 &&
                                unsafe { (*p_dest).e_dest } as i32 >= 3 {
                            __state = 295;
                        } else { __state = 292; }
                    }
                    295 => {
                        unsafe { (*p).n_select_row -= 30 as LogEst };
                        __state = 292;
                    }
                    296 => {
                        if !(s_sort.p_order_by).is_null() {
                            __state = 299;
                        } else { __state = 298; }
                    }
                    297 => {
                        s_distinct.e_tnct_type =
                            unsafe { sqlite3_where_is_distinct(p_w_info) } as u8;
                        __state = 296;
                    }
                    298 => { __state = 303; }
                    299 => {
                        s_sort.n_ob_sat =
                            unsafe { sqlite3_where_is_ordered(p_w_info) };
                        __state = 300;
                    }
                    300 => {
                        s_sort.label_ob_lopt =
                            unsafe { sqlite3_where_order_by_limit_opt_label(p_w_info) };
                        __state = 301;
                    }
                    301 => {
                        if s_sort.n_ob_sat == unsafe { (*s_sort.p_order_by).n_expr }
                            {
                            __state = 302;
                        } else { __state = 298; }
                    }
                    302 => {
                        s_sort.p_order_by = core::ptr::null_mut();
                        __state = 298;
                    }
                    303 => {
                        if s_sort.addr_sort_index >= 0 &&
                                s_sort.p_order_by == core::ptr::null_mut() {
                            __state = 305;
                        } else { __state = 304; }
                    }
                    304 => { { let _ = 0; }; __state = 306; }
                    305 => {
                        unsafe {
                            sqlite3_vdbe_change_to_noop(v, s_sort.addr_sort_index)
                        };
                        __state = 304;
                    }
                    306 => {
                        if !(p_win).is_null() {
                            __state = 307;
                        } else { __state = 308; }
                    }
                    307 => {
                        addr_gosub = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 309;
                    }
                    308 => {
                        select_inner_loop(p_parse, p, -1, &mut s_sort,
                            &raw mut s_distinct as *const DistinctCtx,
                            unsafe { &mut *p_dest },
                            unsafe { sqlite3_where_continue_label(p_w_info) },
                            unsafe { sqlite3_where_break_label(p_w_info) });
                        __state = 322;
                    }
                    309 => {
                        i_cont = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 310;
                    }
                    310 => {
                        i_break = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 311;
                    }
                    311 => {
                        reg_gosub =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 312;
                    }
                    312 => {
                        unsafe {
                            sqlite3_window_code_step(p_parse, p, p_w_info, reg_gosub,
                                addr_gosub)
                        };
                        __state = 313;
                    }
                    313 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 9, 0, i_break) };
                        __state = 314;
                    }
                    314 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_gosub) };
                        __state = 315;
                    }
                    315 => { __state = 316; }
                    316 => { s_sort.label_ob_lopt = 0; __state = 317; }
                    317 => {
                        select_inner_loop(p_parse, p, -1, &mut s_sort,
                            &raw mut s_distinct as *const DistinctCtx,
                            unsafe { &mut *p_dest }, i_cont, i_break);
                        __state = 318;
                    }
                    318 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, i_cont) };
                        __state = 319;
                    }
                    319 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_gosub) };
                        __state = 320;
                    }
                    320 => { __state = 321; }
                    321 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, i_break) };
                        __state = 280;
                    }
                    322 => { __state = 323; }
                    323 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 280;
                    }
                    324 => { __state = 325; }
                    325 => { __state = 326; }
                    326 => { __state = 327; }
                    327 => { __state = 328; }
                    328 => { __state = 329; }
                    329 => { __state = 330; }
                    330 => { sort_p_tab = 0; __state = 331; }
                    331 => { sort_out = 0; __state = 332; }
                    332 => { order_by_grp = 0; __state = 333; }
                    333 => {
                        if !(p_group_by).is_null() {
                            __state = 335;
                        } else { __state = 336; }
                    }
                    334 => {
                        addr_end = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 352;
                    }
                    335 => { __state = 337; }
                    336 => { { let _ = 0; }; __state = 351; }
                    337 => { __state = 338; }
                    338 => {
                        {
                            k = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
                            p_item_2 =
                                unsafe { (*unsafe { (*p).p_e_list }).a.as_ptr() } as
                                    *mut ExprList_item
                        };
                        __state = 340;
                    }
                    339 => {
                        {
                            k = unsafe { (*p_group_by).n_expr };
                            p_item_2 =
                                unsafe { (*p_group_by).a.as_ptr() } as *mut ExprList_item
                        };
                        __state = 344;
                    }
                    340 => {
                        if k > 0 { __state = 341; } else { __state = 339; }
                    }
                    341 => {
                        unsafe { (*p_item_2).u.x.i_alias = 0 as u16 };
                        __state = 342;
                    }
                    342 => {
                        {
                            { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p_item_2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 340;
                    }
                    343 => { { let _ = 0; }; __state = 347; }
                    344 => {
                        if k > 0 { __state = 345; } else { __state = 343; }
                    }
                    345 => {
                        unsafe { (*p_item_2).u.x.i_alias = 0 as u16 };
                        __state = 346;
                    }
                    346 => {
                        {
                            { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p_item_2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 344;
                    }
                    347 => {
                        if unsafe { (*p).n_select_row } as i32 > 66 {
                            __state = 349;
                        } else { __state = 348; }
                    }
                    348 => {
                        if sqlite3_copy_sort_order(unsafe { &mut *p_group_by },
                                        s_sort.p_order_by as *const ExprList) != 0 &&
                                unsafe {
                                        sqlite3_expr_list_compare(p_group_by as *const ExprList,
                                            s_sort.p_order_by as *const ExprList, -1)
                                    } == 0 {
                            __state = 350;
                        } else { __state = 334; }
                    }
                    349 => {
                        unsafe { (*p).n_select_row = 66 as LogEst };
                        __state = 348;
                    }
                    350 => { order_by_grp = 1; __state = 334; }
                    351 => {
                        unsafe { (*p).n_select_row = 0 as LogEst };
                        __state = 334;
                    }
                    352 => {
                        p_agg_info =
                            unsafe {
                                    sqlite3_db_malloc_zero(db,
                                        core::mem::size_of::<AggInfo>() as u64)
                                } as *mut AggInfo;
                        __state = 353;
                    }
                    353 => {
                        if !(p_agg_info).is_null() {
                            __state = 355;
                        } else { __state = 354; }
                    }
                    354 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 358;
                        } else { __state = 357; }
                    }
                    355 => {
                        unsafe {
                            sqlite3_parser_add_cleanup(p_parse, Some(agginfo_free),
                                p_agg_info as *mut ())
                        };
                        __state = 356;
                    }
                    356 => { __state = 354; }
                    357 => {
                        unsafe { (*p_agg_info).sel_id = unsafe { (*p).sel_id } };
                        __state = 359;
                    }
                    358 => { __state = 2; }
                    359 => {
                        unsafe {
                            memset(&raw mut s_nc as *mut (), 0,
                                core::mem::size_of::<NameContext>() as u64)
                        };
                        __state = 360;
                    }
                    360 => { s_nc.p_parse = p_parse; __state = 361; }
                    361 => { s_nc.p_src_list = p_tab_list; __state = 362; }
                    362 => { s_nc.u_nc.p_agg_info = p_agg_info; __state = 363; }
                    363 => {
                        unsafe {
                            (*p_agg_info).n_sorting_column =
                                if !(p_group_by).is_null() {
                                        unsafe { (*p_group_by).n_expr }
                                    } else { 0 } as u32
                        };
                        __state = 364;
                    }
                    364 => {
                        unsafe { (*p_agg_info).p_group_by = p_group_by };
                        __state = 365;
                    }
                    365 => {
                        unsafe {
                            sqlite3_expr_analyze_agg_list(&mut s_nc, p_e_list)
                        };
                        __state = 366;
                    }
                    366 => {
                        unsafe {
                            sqlite3_expr_analyze_agg_list(&mut s_nc, s_sort.p_order_by)
                        };
                        __state = 367;
                    }
                    367 => {
                        if !(p_having).is_null() {
                            __state = 369;
                        } else { __state = 368; }
                    }
                    368 => {
                        unsafe {
                            (*p_agg_info).n_accumulator =
                                unsafe { (*p_agg_info).n_column }
                        };
                        __state = 376;
                    }
                    369 => {
                        if !(p_group_by).is_null() {
                            __state = 371;
                        } else { __state = 370; }
                    }
                    370 => {
                        unsafe {
                            sqlite3_expr_analyze_aggregates(&mut s_nc, p_having)
                        };
                        __state = 368;
                    }
                    371 => { { let _ = 0; }; __state = 372; }
                    372 => { { let _ = 0; }; __state = 373; }
                    373 => { { let _ = 0; }; __state = 374; }
                    374 => { having_to_where(p_parse, p); __state = 375; }
                    375 => { p_where = unsafe { (*p).p_where }; __state = 370; }
                    376 => {
                        if unsafe { (*p).p_group_by } == core::ptr::null_mut() &&
                                    unsafe { (*p).p_having } == core::ptr::null_mut() &&
                                unsafe { (*p_agg_info).n_func } == 1 {
                            __state = 378;
                        } else { __state = 379; }
                    }
                    377 => {
                        analyze_agg_func_args(unsafe { &*p_agg_info }, &mut s_nc);
                        __state = 380;
                    }
                    378 => {
                        min_max_flag =
                            min_max_query(db,
                                unsafe {
                                    &*unsafe {
                                                (*unsafe {
                                                            (*p_agg_info).a_func.offset(0 as isize)
                                                        }).p_f_expr
                                            }
                                }, &mut p_min_max_order_by);
                        __state = 377;
                    }
                    379 => { min_max_flag = 0 as u8; __state = 377; }
                    380 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 382;
                        } else { __state = 381; }
                    }
                    381 => {
                        if !(p_group_by).is_null() {
                            __state = 384;
                        } else { __state = 385; }
                    }
                    382 => { __state = 2; }
                    383 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_end) };
                        __state = 280;
                    }
                    384 => { __state = 386; }
                    385 => { __state = 552; }
                    386 => { __state = 387; }
                    387 => { __state = 388; }
                    388 => { __state = 389; }
                    389 => { __state = 390; }
                    390 => { __state = 391; }
                    391 => { __state = 392; }
                    392 => { __state = 393; }
                    393 => { __state = 394; }
                    394 => {
                        p_distinct = core::ptr::null_mut();
                        __state = 395;
                    }
                    395 => { dist_flag = 0 as u16; __state = 396; }
                    396 => { e_dist = 0; __state = 397; }
                    397 => {
                        if unsafe { (*p_agg_info).n_func } == 1 &&
                                            unsafe {
                                                    (*unsafe {
                                                                (*p_agg_info).a_func.offset(0 as isize)
                                                            }).i_distinct
                                                } >= 0 &&
                                        unsafe {
                                                (*unsafe {
                                                            (*p_agg_info).a_func.offset(0 as isize)
                                                        }).p_f_expr
                                            } != core::ptr::null_mut() &&
                                    unsafe {
                                                (*unsafe {
                                                                (*unsafe {
                                                                            (*p_agg_info).a_func.offset(0 as isize)
                                                                        }).p_f_expr
                                                            }).flags
                                            } & 4096 as u32 == 0 as u32 &&
                                unsafe {
                                        (*unsafe {
                                                            (*unsafe {
                                                                        (*p_agg_info).a_func.offset(0 as isize)
                                                                    }).p_f_expr
                                                        }).x.p_list
                                    } != core::ptr::null_mut() {
                            __state = 399;
                        } else { __state = 398; }
                    }
                    398 => {
                        unsafe {
                            (*p_agg_info).sorting_idx =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                }
                        };
                        __state = 404;
                    }
                    399 => {
                        p_expr =
                            unsafe {
                                (*(unsafe {
                                                    (*unsafe {
                                                                        (*unsafe {
                                                                                            (*unsafe {
                                                                                                        (*p_agg_info).a_func.offset(0 as isize)
                                                                                                    }).p_f_expr
                                                                                        }).x.p_list
                                                                    }).a.as_ptr()
                                                } as *mut ExprList_item).offset(0 as isize)).p_expr
                            };
                        __state = 400;
                    }
                    400 => {
                        p_expr =
                            unsafe { sqlite3_expr_dup(db, p_expr as *const Expr, 0) };
                        __state = 401;
                    }
                    401 => {
                        p_distinct =
                            unsafe {
                                sqlite3_expr_list_dup(db, p_group_by as *const ExprList, 0)
                            };
                        __state = 402;
                    }
                    402 => {
                        p_distinct =
                            unsafe {
                                sqlite3_expr_list_append(p_parse, p_distinct, p_expr)
                            };
                        __state = 403;
                    }
                    403 => {
                        dist_flag =
                            if !(p_distinct).is_null() { 256 | 1024 } else { 0 } as u16;
                        __state = 398;
                    }
                    404 => {
                        p_key_info_1 =
                            sqlite3_key_info_from_expr_list(p_parse,
                                unsafe { &*p_group_by }, 0,
                                unsafe { (*p_agg_info).n_column });
                        __state = 405;
                    }
                    405 => {
                        addr_sorting_idx =
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 121,
                                    unsafe { (*p_agg_info).sorting_idx },
                                    unsafe { (*p_agg_info).n_sorting_column } as i32, 0,
                                    p_key_info_1 as *mut i8 as *const i8, -9)
                            };
                        __state = 406;
                    }
                    406 => {
                        i_use_flag =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 407;
                    }
                    407 => {
                        i_abort_flag =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 408;
                    }
                    408 => {
                        reg_output_row =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 409;
                    }
                    409 => {
                        addr_output_row =
                            unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 410;
                    }
                    410 => {
                        reg_reset =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 411;
                    }
                    411 => {
                        addr_reset = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 412;
                    }
                    412 => {
                        i_a_mem = unsafe { (*p_parse).n_mem } + 1;
                        __state = 413;
                    }
                    413 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_group_by).n_expr }
                        };
                        __state = 414;
                    }
                    414 => {
                        i_b_mem = unsafe { (*p_parse).n_mem } + 1;
                        __state = 415;
                    }
                    415 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_group_by).n_expr }
                        };
                        __state = 416;
                    }
                    416 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, i_abort_flag) };
                        __state = 417;
                    }
                    417 => { __state = 418; }
                    418 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 77, 0, i_a_mem,
                                i_a_mem + unsafe { (*p_group_by).n_expr } - 1)
                        };
                        __state = 419;
                    }
                    419 => {
                        unsafe {
                            sqlite3_expr_null_register_range(p_parse, i_a_mem,
                                unsafe { (*p_group_by).n_expr })
                        };
                        __state = 420;
                    }
                    420 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_reset, addr_reset)
                        };
                        __state = 421;
                    }
                    421 => { __state = 422; }
                    422 => {
                        p_w_info =
                            unsafe {
                                sqlite3_where_begin(p_parse, p_tab_list, p_where,
                                    p_group_by, p_distinct, p,
                                    (if s_distinct.is_tnct as i32 == 2 { 128 } else { 64 } |
                                                if order_by_grp != 0 { 512 } else { 0 } | dist_flag as i32)
                                        as u16, 0)
                            };
                        __state = 423;
                    }
                    423 => {
                        if p_w_info == core::ptr::null_mut() {
                            __state = 425;
                        } else { __state = 424; }
                    }
                    424 => {
                        if !(unsafe { (*p_parse).p_idx_epr }).is_null() {
                            __state = 428;
                        } else { __state = 427; }
                    }
                    425 => {
                        unsafe { sqlite3_expr_list_delete(db, p_distinct) };
                        __state = 426;
                    }
                    426 => { __state = 2; }
                    427 => {
                        assign_aggregate_registers(unsafe { &mut *p_parse },
                            unsafe { &mut *p_agg_info });
                        __state = 429;
                    }
                    428 => {
                        optimize_aggregate_use_of_indexed_expr(p_parse as
                                *const Parse, p as *const Select, p_agg_info, &mut s_nc);
                        __state = 427;
                    }
                    429 => {
                        e_dist = unsafe { sqlite3_where_is_distinct(p_w_info) };
                        __state = 430;
                    }
                    430 => { __state = 431; }
                    431 => {
                        if unsafe { sqlite3_where_is_ordered(p_w_info) } ==
                                unsafe { (*p_group_by).n_expr } {
                            __state = 433;
                        } else { __state = 434; }
                    }
                    432 => {
                        if !(unsafe { (*p_parse).p_idx_epr }).is_null() {
                            __state = 481;
                        } else { __state = 480; }
                    }
                    433 => { group_by_sort = 0; __state = 432; }
                    434 => { __state = 435; }
                    435 => { __state = 436; }
                    436 => { __state = 437; }
                    437 => { __state = 438; }
                    438 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 0 as u8,
                                c"USE TEMP B-TREE FOR %s".as_ptr() as *mut i8 as *const i8,
                                if s_distinct.is_tnct != 0 &&
                                        unsafe { (*p).sel_flags } & 1 as u32 == 0 as u32 {
                                    c"DISTINCT".as_ptr() as *mut i8
                                } else { c"GROUP BY".as_ptr() as *mut i8 })
                        };
                        __state = 439;
                    }
                    439 => { group_by_sort = 1; __state = 440; }
                    440 => {
                        n_group_by = unsafe { (*p_group_by).n_expr };
                        __state = 441;
                    }
                    441 => { n_col = n_group_by; __state = 442; }
                    442 => { j = n_group_by; __state = 443; }
                    443 => { i = 0; __state = 445; }
                    444 => {
                        reg_base =
                            unsafe { sqlite3_get_temp_range(p_parse, n_col) };
                        __state = 450;
                    }
                    445 => {
                        if i < unsafe { (*p_agg_info).n_column } {
                            __state = 446;
                        } else { __state = 444; }
                    }
                    446 => {
                        if unsafe {
                                    (*unsafe {
                                                (*p_agg_info).a_col.offset(i as isize)
                                            }).i_sorter_column
                                } >= j {
                            __state = 448;
                        } else { __state = 447; }
                    }
                    447 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 445;
                    }
                    448 => {
                        { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
                        __state = 449;
                    }
                    449 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 447;
                    }
                    450 => {
                        unsafe {
                            sqlite3_expr_code_expr_list(p_parse, p_group_by, reg_base,
                                0, 0 as u8)
                        };
                        __state = 451;
                    }
                    451 => { j = n_group_by; __state = 452; }
                    452 => {
                        unsafe { (*p_agg_info).direct_mode = 1 as u8 };
                        __state = 453;
                    }
                    453 => { i = 0; __state = 455; }
                    454 => {
                        unsafe { (*p_agg_info).direct_mode = 0 as u8 };
                        __state = 461;
                    }
                    455 => {
                        if i < unsafe { (*p_agg_info).n_column } {
                            __state = 456;
                        } else { __state = 454; }
                    }
                    456 => {
                        p_col =
                            unsafe { unsafe { (*p_agg_info).a_col.offset(i as isize) } }
                                as *const AggInfo_col;
                        __state = 458;
                    }
                    457 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 455;
                    }
                    458 => {
                        if unsafe { (*p_col).i_sorter_column } >= j {
                            __state = 459;
                        } else { __state = 457; }
                    }
                    459 => {
                        unsafe {
                            sqlite3_expr_code(p_parse, unsafe { (*p_col).p_c_expr },
                                j + reg_base)
                        };
                        __state = 460;
                    }
                    460 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 457;
                    }
                    461 => {
                        reg_record = unsafe { sqlite3_get_temp_reg(p_parse) };
                        __state = 462;
                    }
                    462 => { __state = 463; }
                    463 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_base, n_col, reg_record)
                        };
                        __state = 464;
                    }
                    464 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 141,
                                unsafe { (*p_agg_info).sorting_idx }, reg_record)
                        };
                        __state = 465;
                    }
                    465 => { __state = 466; }
                    466 => {
                        unsafe { sqlite3_release_temp_reg(p_parse, reg_record) };
                        __state = 467;
                    }
                    467 => {
                        unsafe {
                            sqlite3_release_temp_range(p_parse, reg_base, n_col)
                        };
                        __state = 468;
                    }
                    468 => { __state = 469; }
                    469 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 470;
                    }
                    470 => {
                        unsafe {
                            (*p_agg_info).sorting_idx_p_tab =
                                {
                                    sort_p_tab =
                                        {
                                            let __p = unsafe { &mut (*p_parse).n_tab };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    sort_p_tab
                                }
                        };
                        __state = 471;
                    }
                    471 => {
                        sort_out = unsafe { sqlite3_get_temp_reg(p_parse) };
                        __state = 472;
                    }
                    472 => { __state = 473; }
                    473 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 123, sort_p_tab, sort_out, n_col)
                        };
                        __state = 474;
                    }
                    474 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 34,
                                unsafe { (*p_agg_info).sorting_idx }, addr_end)
                        };
                        __state = 475;
                    }
                    475 => { __state = 476; }
                    476 => { __state = 477; }
                    477 => {
                        unsafe { (*p_agg_info).use_sorting_idx = 1 as u8 };
                        __state = 478;
                    }
                    478 => { __state = 479; }
                    479 => { __state = 432; }
                    480 => {
                        if order_by_grp != 0 &&
                                    unsafe { (*db).db_opt_flags } & 4 as u32 == 0 as u32 &&
                                (group_by_sort != 0 ||
                                    unsafe { sqlite3_where_is_sorted(p_w_info) } != 0) {
                            __state = 483;
                        } else { __state = 482; }
                    }
                    481 => {
                        aggregate_convert_indexed_expr_ref_to_column(unsafe {
                                &*p_agg_info
                            });
                        __state = 480;
                    }
                    482 => {
                        addr_top_of_loop = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 485;
                    }
                    483 => {
                        s_sort.p_order_by = core::ptr::null_mut();
                        __state = 484;
                    }
                    484 => {
                        unsafe {
                            sqlite3_vdbe_change_to_noop(v, s_sort.addr_sort_index)
                        };
                        __state = 482;
                    }
                    485 => {
                        if group_by_sort != 0 {
                            __state = 487;
                        } else { __state = 486; }
                    }
                    486 => { j = 0; __state = 489; }
                    487 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 135,
                                unsafe { (*p_agg_info).sorting_idx }, sort_out, sort_p_tab)
                        };
                        __state = 486;
                    }
                    488 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 92, i_a_mem, i_b_mem,
                                unsafe { (*p_group_by).n_expr },
                                sqlite3_key_info_ref(p_key_info_1) as *mut i8 as *const i8,
                                -9)
                        };
                        __state = 504;
                    }
                    489 => {
                        if j < unsafe { (*p_group_by).n_expr } {
                            __state = 490;
                        } else { __state = 488; }
                    }
                    490 => {
                        i_order_by_col =
                            unsafe {
                                    (*(unsafe { (*p_group_by).a.as_ptr() } as
                                                            *mut ExprList_item).offset(j as isize)).u.x.i_order_by_col
                                } as i32;
                        __state = 492;
                    }
                    491 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 489;
                    }
                    492 => {
                        if group_by_sort != 0 {
                            __state = 494;
                        } else { __state = 495; }
                    }
                    493 => {
                        if i_order_by_col != 0 {
                            __state = 497;
                        } else { __state = 491; }
                    }
                    494 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, sort_p_tab, j, i_b_mem + j)
                        };
                        __state = 493;
                    }
                    495 => {
                        unsafe { (*p_agg_info).direct_mode = 1 as u8 };
                        __state = 496;
                    }
                    496 => {
                        unsafe {
                            sqlite3_expr_code(p_parse,
                                unsafe {
                                    (*(unsafe { (*p_group_by).a.as_ptr() } as
                                                    *mut ExprList_item).offset(j as isize)).p_expr
                                }, i_b_mem + j)
                        };
                        __state = 493;
                    }
                    497 => {
                        p_x =
                            unsafe {
                                (*(unsafe { (*unsafe { (*p).p_e_list }).a.as_ptr() } as
                                                *mut ExprList_item).offset((i_order_by_col - 1) as
                                                isize)).p_expr
                            };
                        __state = 498;
                    }
                    498 => {
                        p_base =
                            unsafe { sqlite3_expr_skip_collate_and_likely(p_x) } as
                                *const Expr;
                        __state = 499;
                    }
                    499 => {
                        if p_base != core::ptr::null_mut() &&
                                unsafe { (*p_base).op } as i32 == 179 {
                            __state = 501;
                        } else { __state = 500; }
                    }
                    500 => {
                        if p_base != core::ptr::null_mut() &&
                                    unsafe { (*p_base).op } as i32 != 170 &&
                                unsafe { (*p_base).op } as i32 != 176 {
                            __state = 503;
                        } else { __state = 491; }
                    }
                    501 => { p_x = unsafe { (*p_base).p_left }; __state = 502; }
                    502 => {
                        p_base =
                            unsafe { sqlite3_expr_skip_collate_and_likely(p_x) };
                        __state = 499;
                    }
                    503 => {
                        unsafe { sqlite3_expr_to_register(p_x, i_a_mem + j) };
                        __state = 491;
                    }
                    504 => {
                        addr1 = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 505;
                    }
                    505 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 14, addr1 + 1, 0, addr1 + 1)
                        };
                        __state = 506;
                    }
                    506 => { __state = 507; }
                    507 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_output_row, addr_output_row)
                        };
                        __state = 508;
                    }
                    508 => { __state = 509; }
                    509 => {
                        unsafe {
                            sqlite3_expr_code_move(p_parse, i_b_mem, i_a_mem,
                                unsafe { (*p_group_by).n_expr })
                        };
                        __state = 510;
                    }
                    510 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 61, i_abort_flag, addr_end)
                        };
                        __state = 511;
                    }
                    511 => { __state = 512; }
                    512 => { __state = 513; }
                    513 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_reset, addr_reset)
                        };
                        __state = 514;
                    }
                    514 => { __state = 515; }
                    515 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
                        __state = 516;
                    }
                    516 => {
                        update_accumulator(p_parse, i_use_flag,
                            unsafe { &mut *p_agg_info }, e_dist);
                        __state = 517;
                    }
                    517 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_use_flag) };
                        __state = 518;
                    }
                    518 => { __state = 519; }
                    519 => {
                        if group_by_sort != 0 {
                            __state = 521;
                        } else { __state = 522; }
                    }
                    520 => {
                        unsafe { sqlite3_expr_list_delete(db, p_distinct) };
                        __state = 526;
                    }
                    521 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 38,
                                unsafe { (*p_agg_info).sorting_idx }, addr_top_of_loop)
                        };
                        __state = 523;
                    }
                    522 => { __state = 524; }
                    523 => { __state = 520; }
                    524 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 525;
                    }
                    525 => {
                        unsafe { sqlite3_vdbe_change_to_noop(v, addr_sorting_idx) };
                        __state = 520;
                    }
                    526 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_output_row, addr_output_row)
                        };
                        __state = 527;
                    }
                    527 => { __state = 528; }
                    528 => {
                        unsafe { sqlite3_vdbe_goto(v, addr_end) };
                        __state = 529;
                    }
                    529 => {
                        addr_set_abort = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 530;
                    }
                    530 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_abort_flag) };
                        __state = 531;
                    }
                    531 => { __state = 532; }
                    532 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_output_row) };
                        __state = 533;
                    }
                    533 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_output_row) };
                        __state = 534;
                    }
                    534 => {
                        addr_output_row = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 535;
                    }
                    535 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 61, i_use_flag, addr_output_row + 2)
                        };
                        __state = 536;
                    }
                    536 => { __state = 537; }
                    537 => { __state = 538; }
                    538 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_output_row) };
                        __state = 539;
                    }
                    539 => {
                        finalize_agg_functions(p_parse, unsafe { &*p_agg_info });
                        __state = 540;
                    }
                    540 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, p_having,
                                addr_output_row + 1, 16)
                        };
                        __state = 541;
                    }
                    541 => {
                        select_inner_loop(p_parse, p, -1, &mut s_sort,
                            &raw mut s_distinct as *const DistinctCtx,
                            unsafe { &mut *p_dest }, addr_output_row + 1,
                            addr_set_abort);
                        __state = 542;
                    }
                    542 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_output_row) };
                        __state = 543;
                    }
                    543 => { __state = 544; }
                    544 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_reset) };
                        __state = 545;
                    }
                    545 => {
                        reset_accumulator(p_parse, unsafe { &*p_agg_info });
                        __state = 546;
                    }
                    546 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, i_use_flag) };
                        __state = 547;
                    }
                    547 => { __state = 548; }
                    548 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_reset) };
                        __state = 549;
                    }
                    549 => {
                        if dist_flag as i32 != 0 && e_dist != 0 {
                            __state = 550;
                        } else { __state = 383; }
                    }
                    550 => {
                        p_f =
                            unsafe {
                                    unsafe { (*p_agg_info).a_func.offset(0 as isize) }
                                } as *const AggInfo_func;
                        __state = 551;
                    }
                    551 => {
                        fix_distinct_open_eph(unsafe { &*p_parse }, e_dist,
                            unsafe { (*p_f).i_distinct },
                            unsafe { (*p_f).i_dist_addr });
                        __state = 383;
                    }
                    552 => {
                        if {
                                    p_tab_1 = is_simple_count(unsafe { &*p }, p_agg_info);
                                    p_tab_1
                                } != core::ptr::null_mut() {
                            __state = 554;
                        } else { __state = 555; }
                    }
                    553 => {
                        s_sort.p_order_by = core::ptr::null_mut();
                        __state = 622;
                    }
                    554 => {
                        i_db_1 =
                            unsafe {
                                    sqlite3_schema_to_index(unsafe { (*p_parse).db },
                                        unsafe { (*p_tab_1).p_schema })
                                } as i32;
                        __state = 556;
                    }
                    555 => { reg_acc = 0; __state = 581; }
                    556 => {
                        i_csr =
                            {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as i32;
                        __state = 557;
                    }
                    557 => { __state = 558; }
                    558 => {
                        p_key_info_2 = core::ptr::null_mut();
                        __state = 559;
                    }
                    559 => { p_best = core::ptr::null_mut(); __state = 560; }
                    560 => {
                        i_root = unsafe { (*p_tab_1).tnum };
                        __state = 561;
                    }
                    561 => {
                        unsafe { sqlite3_code_verify_schema(p_parse, i_db_1) };
                        __state = 562;
                    }
                    562 => {
                        unsafe {
                            sqlite3_table_lock(p_parse, i_db_1,
                                unsafe { (*p_tab_1).tnum }, 0 as u8,
                                unsafe { (*p_tab_1).z_name } as *const i8)
                        };
                        __state = 563;
                    }
                    563 => {
                        if !(unsafe { (*p_tab_1).tab_flags } & 128 as u32 ==
                                            0 as u32) as i32 != 0 {
                            __state = 565;
                        } else { __state = 564; }
                    }
                    564 => {
                        if (unsafe {
                                            (*(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).fg.not_indexed()
                                        } == 0) as i32 != 0 {
                            __state = 567;
                        } else { __state = 566; }
                    }
                    565 => {
                        p_best = unsafe { sqlite3_primary_key_index(p_tab_1) };
                        __state = 564;
                    }
                    566 => {
                        if !(p_best).is_null() {
                            __state = 573;
                        } else { __state = 572; }
                    }
                    567 => {
                        p_idx = unsafe { (*p_tab_1).p_index };
                        __state = 568;
                    }
                    568 => {
                        if !(p_idx).is_null() {
                            __state = 569;
                        } else { __state = 566; }
                    }
                    569 => {
                        if unsafe { (*p_idx).b_unordered() } as i32 == 0 &&
                                        (unsafe { (*p_idx).sz_idx_row } as i32) <
                                            unsafe { (*p_tab_1).sz_tab_row } as i32 &&
                                    unsafe { (*p_idx).p_part_idx_where } ==
                                        core::ptr::null_mut() &&
                                ((p_best).is_null() as i32 != 0 ||
                                    (unsafe { (*p_idx).sz_idx_row } as i32) <
                                        unsafe { (*p_best).sz_idx_row } as i32) {
                            __state = 571;
                        } else { __state = 570; }
                    }
                    570 => {
                        p_idx = unsafe { (*p_idx).p_next };
                        __state = 568;
                    }
                    571 => { p_best = p_idx; __state = 570; }
                    572 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 114, i_csr, i_root as i32,
                                i_db_1, 1)
                        };
                        __state = 575;
                    }
                    573 => {
                        i_root = unsafe { (*p_best).tnum };
                        __state = 574;
                    }
                    574 => {
                        p_key_info_2 =
                            unsafe { sqlite3_key_info_of_index(p_parse, p_best) };
                        __state = 572;
                    }
                    575 => {
                        if !(p_key_info_2).is_null() {
                            __state = 577;
                        } else { __state = 576; }
                    }
                    576 => {
                        assign_aggregate_registers(unsafe { &mut *p_parse },
                            unsafe { &mut *p_agg_info });
                        __state = 578;
                    }
                    577 => {
                        unsafe {
                            sqlite3_vdbe_change_p4(v, -1,
                                p_key_info_2 as *mut i8 as *const i8, -9)
                        };
                        __state = 576;
                    }
                    578 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 100, i_csr,
                                unsafe { (*p_agg_info).i_first_reg } +
                                        unsafe { (*p_agg_info).n_column } + 0)
                        };
                        __state = 579;
                    }
                    579 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 124, i_csr) };
                        __state = 580;
                    }
                    580 => {
                        explain_simple_count(p_parse, unsafe { &*p_tab_1 },
                            p_best as *const Index);
                        __state = 553;
                    }
                    581 => {
                        p_distinct_1 = core::ptr::null_mut();
                        __state = 582;
                    }
                    582 => { dist_flag_1 = 0 as u16; __state = 583; }
                    583 => { __state = 584; }
                    584 => {
                        if unsafe { (*p_agg_info).n_accumulator } != 0 {
                            __state = 586;
                        } else { __state = 587; }
                    }
                    585 => {
                        assign_aggregate_registers(unsafe { &mut *p_parse },
                            unsafe { &mut *p_agg_info });
                        __state = 600;
                    }
                    586 => { i = 0; __state = 589; }
                    587 => {
                        if unsafe { (*p_agg_info).n_func } == 1 &&
                                unsafe {
                                        (*unsafe {
                                                    (*p_agg_info).a_func.offset(0 as isize)
                                                }).i_distinct
                                    } >= 0 {
                            __state = 597;
                        } else { __state = 585; }
                    }
                    588 => {
                        if i == unsafe { (*p_agg_info).n_func } {
                            __state = 595;
                        } else { __state = 585; }
                    }
                    589 => {
                        if i < unsafe { (*p_agg_info).n_func } {
                            __state = 590;
                        } else { __state = 588; }
                    }
                    590 => {
                        if unsafe {
                                        (*unsafe {
                                                        (*unsafe {
                                                                    (*p_agg_info).a_func.offset(i as isize)
                                                                }).p_f_expr
                                                    }).flags
                                    } & 16777216 as u32 != 0 as u32 {
                            __state = 593;
                        } else { __state = 592; }
                    }
                    591 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 589;
                    }
                    592 => {
                        if unsafe {
                                        (*unsafe {
                                                        (*unsafe { (*p_agg_info).a_func.offset(i as isize) }).p_func
                                                    }).func_flags
                                    } & 32 as u32 != 0 {
                            __state = 594;
                        } else { __state = 591; }
                    }
                    593 => { __state = 591; }
                    594 => { __state = 588; }
                    595 => {
                        reg_acc =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 596;
                    }
                    596 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_acc) };
                        __state = 585;
                    }
                    597 => { { let _ = 0; }; __state = 598; }
                    598 => {
                        p_distinct_1 =
                            unsafe {
                                (*unsafe {
                                                    (*unsafe {
                                                                (*p_agg_info).a_func.offset(0 as isize)
                                                            }).p_f_expr
                                                }).x.p_list
                            };
                        __state = 599;
                    }
                    599 => {
                        dist_flag_1 =
                            if !(p_distinct_1).is_null() { 256 | 1024 } else { 0 } as
                                u16;
                        __state = 585;
                    }
                    600 => { { let _ = 0; }; __state = 601; }
                    601 => {
                        reset_accumulator(p_parse, unsafe { &*p_agg_info });
                        __state = 602;
                    }
                    602 => { { let _ = 0; }; __state = 603; }
                    603 => { { let _ = 0; }; __state = 604; }
                    604 => { __state = 605; }
                    605 => {
                        p_w_info =
                            unsafe {
                                sqlite3_where_begin(p_parse, p_tab_list, p_where,
                                    p_min_max_order_by, p_distinct_1, p,
                                    (min_max_flag as i32 | dist_flag_1 as i32) as u16, 0)
                            };
                        __state = 606;
                    }
                    606 => {
                        if p_w_info == core::ptr::null_mut() {
                            __state = 608;
                        } else { __state = 607; }
                    }
                    607 => { __state = 609; }
                    608 => { __state = 2; }
                    609 => {
                        e_dist_1 = unsafe { sqlite3_where_is_distinct(p_w_info) };
                        __state = 610;
                    }
                    610 => {
                        update_accumulator(p_parse, reg_acc,
                            unsafe { &mut *p_agg_info }, e_dist_1);
                        __state = 611;
                    }
                    611 => {
                        if e_dist_1 != 0 { __state = 613; } else { __state = 612; }
                    }
                    612 => {
                        if reg_acc != 0 { __state = 617; } else { __state = 616; }
                    }
                    613 => {
                        p_f_1 =
                            unsafe { (*p_agg_info).a_func } as *const AggInfo_func;
                        __state = 614;
                    }
                    614 => {
                        if !(p_f_1).is_null() {
                            __state = 615;
                        } else { __state = 612; }
                    }
                    615 => {
                        fix_distinct_open_eph(unsafe { &*p_parse }, e_dist_1,
                            unsafe { (*p_f_1).i_distinct },
                            unsafe { (*p_f_1).i_dist_addr });
                        __state = 612;
                    }
                    616 => {
                        if min_max_flag != 0 {
                            __state = 619;
                        } else { __state = 618; }
                    }
                    617 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, reg_acc) };
                        __state = 616;
                    }
                    618 => { __state = 620; }
                    619 => {
                        unsafe { sqlite3_where_min_max_opt_early_out(v, p_w_info) };
                        __state = 618;
                    }
                    620 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 621;
                    }
                    621 => {
                        finalize_agg_functions(p_parse, unsafe { &*p_agg_info });
                        __state = 553;
                    }
                    622 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, p_having, addr_end, 16)
                        };
                        __state = 623;
                    }
                    623 => {
                        select_inner_loop(p_parse, p, -1, core::ptr::null_mut(),
                            core::ptr::null(), unsafe { &mut *p_dest }, addr_end,
                            addr_end);
                        __state = 383;
                    }
                    624 => {
                        if !(s_sort.p_order_by).is_null() {
                            __state = 627;
                        } else { __state = 626; }
                    }
                    625 => {
                        explain_temp_table(p_parse,
                            c"DISTINCT".as_ptr() as *mut i8 as *const i8);
                        __state = 624;
                    }
                    626 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, i_end) };
                        __state = 629;
                    }
                    627 => { { let _ = 0; }; __state = 628; }
                    628 => {
                        generate_sort_tail(p_parse, unsafe { &*p }, &s_sort,
                            unsafe { (*p_e_list).n_expr }, unsafe { &*p_dest });
                        __state = 626;
                    }
                    629 => {
                        rc = (unsafe { (*p_parse).n_err } > 0) as i32;
                        __state = 630;
                    }
                    630 => { __state = 2; }
                    631 => { { let _ = 0; }; __state = 632; }
                    632 => {
                        unsafe { sqlite3_expr_list_delete(db, p_min_max_order_by) };
                        __state = 633;
                    }
                    633 => {
                        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
                        __state = 634;
                    }
                    634 => { return rc; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_new(p_parse: *mut Parse,
    mut p_e_list: *mut ExprList, mut p_src: *mut SrcList, p_where: *mut Expr,
    p_group_by: *mut ExprList, p_having: *mut Expr, p_order_by: *mut ExprList,
    sel_flags: u32, p_limit: *mut Expr) -> *mut Select {
    unsafe {
        let mut p_new: *mut Select = core::ptr::null_mut();
        let mut p_allocated: *mut Select = core::ptr::null_mut();
        let mut standin: Select = unsafe { core::mem::zeroed() };
        p_allocated =
            {
                p_new =
                    unsafe {
                            sqlite3_db_malloc_raw_nn(unsafe { (*p_parse).db },
                                core::mem::size_of::<Select>() as u64)
                        } as *mut Select;
                p_new
            };
        if p_new == core::ptr::null_mut() {
            { let _ = 0; };
            p_new = &mut standin;
        }
        if p_e_list == core::ptr::null_mut() {
            p_e_list =
                unsafe {
                    sqlite3_expr_list_append(p_parse, core::ptr::null_mut(),
                        unsafe {
                            sqlite3_expr(unsafe { (*p_parse).db }, 180,
                                core::ptr::null())
                        })
                };
        }
        unsafe { (*p_new).p_e_list = p_e_list };
        unsafe { (*p_new).op = 139 as u8 };
        unsafe { (*p_new).sel_flags = sel_flags };
        unsafe { (*p_new).i_limit = 0 };
        unsafe { (*p_new).i_offset = 0 };
        unsafe {
            (*p_new).sel_id =
                {
                        let __p = unsafe { &mut (*p_parse).n_select };
                        *__p += 1;
                        *__p
                    } as u32
        };
        unsafe { (*p_new).n_select_row = 0 as LogEst };
        if p_src == core::ptr::null_mut() {
            p_src =
                unsafe {
                        sqlite3_db_malloc_zero(unsafe { (*p_parse).db },
                            core::mem::offset_of!(SrcList, a) as u64 +
                                core::mem::size_of::<SrcItem>() as u64)
                    } as *mut SrcList;
        }
        unsafe { (*p_new).p_src = p_src };
        unsafe { (*p_new).p_where = p_where };
        unsafe { (*p_new).p_group_by = p_group_by };
        unsafe { (*p_new).p_having = p_having };
        unsafe { (*p_new).p_order_by = p_order_by };
        unsafe { (*p_new).p_prior = core::ptr::null_mut() };
        unsafe { (*p_new).p_next = core::ptr::null_mut() };
        unsafe { (*p_new).p_limit = p_limit };
        unsafe { (*p_new).p_with = core::ptr::null_mut() };
        unsafe { (*p_new).p_win = core::ptr::null_mut() };
        unsafe { (*p_new).p_win_defn = core::ptr::null_mut() };
        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0 {
            clear_select(unsafe { (*p_parse).db }, p_new,
                (p_new != &raw mut standin as *mut Select) as i32);
            p_allocated = core::ptr::null_mut();
        } else { { let _ = 0; }; }
        return p_allocated;
    }
}
extern "C" fn select_check_on_clauses_expr(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut p_ctx: *mut CheckOnCtx =
            unsafe { (*p_walker_1).u.p_check_on_ctx };
        if unsafe { (*p_expr_1).flags } & 1 as u32 != 0 as u32 ||
                unsafe { (*p_expr_1).flags } & 2 as u32 != 0 as u32 &&
                    unsafe {
                                    (*(unsafe { (*unsafe { (*p_ctx).p_src }).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).fg.jointype
                                } as i32 & 64 != 0 {
            { let _ = 0; };
            if unsafe { (*p_ctx).i_join } == 0 {
                unsafe { (*p_ctx).i_join = unsafe { (*p_expr_1).w.i_join } };
                unsafe { sqlite3_walk_expr_nn(p_walker_1, p_expr_1) };
                unsafe { (*p_ctx).i_join = 0 };
                return 1;
            }
        }
        if unsafe { (*p_expr_1).op } as i32 == 168 {
            '__b120: loop {
                '__c120: loop {
                    let p_src: *const SrcList =
                        unsafe { (*p_ctx).p_src } as *const SrcList;
                    let n_src: i32 = unsafe { (*p_src).n_src };
                    let i_tab: i32 = unsafe { (*p_expr_1).i_table };
                    let mut ii: i32 = 0;
                    {
                        ii = 0;
                        '__b121: loop {
                            if !(ii < n_src &&
                                            unsafe {
                                                    (*(unsafe { (*p_src).a.as_ptr() } as
                                                                    *mut SrcItem).offset(ii as isize)).i_cursor
                                                } != i_tab) {
                                break '__b121;
                            }
                            '__c121: loop { break '__c121; }
                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if ii < n_src {
                        if unsafe { (*p_ctx).i_join } != 0 &&
                                i_tab > unsafe { (*p_ctx).i_join } {
                            unsafe {
                                sqlite3_error_msg(unsafe { (*p_walker_1).p_parse },
                                    c"%s references tables to its right".as_ptr() as *mut i8 as
                                        *const i8,
                                    if unsafe { (*p_ctx).b_func_arg } != 0 {
                                        c"table-function argument".as_ptr() as *mut i8
                                    } else { c"ON clause".as_ptr() as *mut i8 })
                            };
                            return 2;
                        }
                        break '__b120;
                    }
                    p_ctx = unsafe { (*p_ctx).p_parent };
                    break '__c120;
                }
                if !(!(p_ctx).is_null()) { break '__b120; }
            }
        }
        return 0;
    }
}
extern "C" fn select_check_on_clauses_select(p_walker_1: *mut Walker,
    p_select_1: *mut Select) -> i32 {
    unsafe {
        let p_ctx: *mut CheckOnCtx =
            unsafe { (*p_walker_1).u.p_check_on_ctx };
        if unsafe { (*p_select_1).p_src } == unsafe { (*p_ctx).p_src } ||
                unsafe { (*unsafe { (*p_select_1).p_src }).n_src } == 0 {
            return 0;
        } else {
            let mut s_ctx: CheckOnCtx = unsafe { core::mem::zeroed() };
            unsafe {
                memset(&raw mut s_ctx as *mut (), 0,
                    core::mem::size_of::<CheckOnCtx>() as u64)
            };
            s_ctx.p_src = unsafe { (*p_select_1).p_src };
            s_ctx.p_parent = p_ctx;
            unsafe { (*p_walker_1).u.p_check_on_ctx = &mut s_ctx };
            unsafe { sqlite3_walk_select(p_walker_1, p_select_1) };
            unsafe { (*p_walker_1).u.p_check_on_ctx = p_ctx };
            unsafe { (*p_select_1).sel_flags &= !1073741824 as u32 };
            return 1;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_check_on_clauses(p_parse: *mut Parse,
    p_select: &mut Select) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        let mut s_ctx: CheckOnCtx = unsafe { core::mem::zeroed() };
        let mut ii: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.p_parse = p_parse;
        w.x_expr_callback = Some(select_check_on_clauses_expr);
        w.x_select_callback = Some(select_check_on_clauses_select);
        w.u.p_check_on_ctx = &mut s_ctx;
        unsafe {
            memset(&raw mut s_ctx as *mut (), 0,
                core::mem::size_of::<CheckOnCtx>() as u64)
        };
        s_ctx.p_src = (*p_select).p_src;
        unsafe { sqlite3_walk_expr(&mut w, (*p_select).p_where) };
        (*p_select).sel_flags &= !1073741824 as u32;
        s_ctx.b_func_arg = 1;
        {
            ii = 0;
            '__b122: loop {
                if !(ii < unsafe { (*(*p_select).p_src).n_src }) {
                    break '__b122;
                }
                '__c122: loop {
                    let p_item: *const SrcItem =
                        unsafe {
                                &raw mut *(unsafe { (*(*p_select).p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(ii as isize)
                            } as *const SrcItem;
                    if unsafe { (*p_item).fg.is_tab_func() } != 0 &&
                            unsafe { (*p_item).fg.jointype } as i32 & 32 != 0 {
                        s_ctx.i_join = unsafe { (*p_item).i_cursor };
                        unsafe {
                            sqlite3_walk_expr_list(&mut w,
                                unsafe { (*p_item).u1.p_func_arg })
                        };
                    }
                    break '__c122;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_join_type(p_parse: *mut Parse, p_a: *mut Token,
    p_b: *mut Token, p_c: *mut Token) -> i32 {
    unsafe {
        let mut jointype: i32 = 0;
        let mut ap_all: [*mut Token; 3] = [core::ptr::null_mut(); 3];
        let mut p: *mut Token = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        ap_all[0 as usize] = p_a;
        ap_all[1 as usize] = p_b;
        ap_all[2 as usize] = p_c;
        {
            i = 0;
            '__b123: loop {
                if !(i < 3 && !(ap_all[i as usize]).is_null()) {
                    break '__b123;
                }
                '__c123: loop {
                    p = ap_all[i as usize];
                    {
                        j = 0;
                        '__b124: loop {
                            if !(j <
                                            (core::mem::size_of::<[sqlite3_join_type_s0_N20sqlite3_join_type_s0; 7]>()
                                                        as u64 / 3) as i32) {
                                break '__b124;
                            }
                            '__c124: loop {
                                if unsafe { (*p).n } == a_keyword[j as usize].n_char as u32
                                        &&
                                        unsafe {
                                                sqlite3_strnicmp(unsafe { (*p).z } as *mut i8 as *const i8,
                                                    &z_key_text[a_keyword[j as usize].i as usize],
                                                    unsafe { (*p).n } as i32)
                                            } == 0 {
                                    jointype |= a_keyword[j as usize].code as i32;
                                    break '__b124;
                                }
                                break '__c124;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if j >=
                            (core::mem::size_of::<[sqlite3_join_type_s0_N20sqlite3_join_type_s0; 7]>()
                                        as u64 / 3) as i32 {
                        jointype |= 128;
                        break '__b123;
                    }
                    break '__c123;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if jointype & (1 | 32) == 1 | 32 || jointype & 128 != 0 ||
                jointype & (32 | 8 | 16) == 32 {
            let mut z_sp1: *const i8 = c" ".as_ptr() as *mut i8 as *const i8;
            let mut z_sp2: *const i8 = c" ".as_ptr() as *mut i8 as *const i8;
            if p_b == core::ptr::null_mut() {
                {
                    let __p = &mut z_sp1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            if p_c == core::ptr::null_mut() {
                {
                    let __p = &mut z_sp2;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"unknown join type: %T%s%T%s%T".as_ptr() as *mut i8 as
                        *const i8, p_a, z_sp1, p_b, z_sp2, p_c)
            };
            jointype = 1;
        }
        return jointype;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_wrong_num_terms_error(p_parse: *mut Parse,
    p: &Select) -> () {
    if (*p).sel_flags & 512 as u32 != 0 {
        unsafe {
            sqlite3_error_msg(p_parse,
                c"all VALUES must have the same number of terms".as_ptr() as
                        *mut i8 as *const i8)
        };
    } else {
        unsafe {
            sqlite3_error_msg(p_parse,
                c"SELECTs to the left and right of %s do not have the same number of result columns".as_ptr()
                        as *mut i8 as *const i8,
                sqlite3_select_op_name((*p).op as i32))
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_join_type_s0_N20sqlite3_join_type_s0 {
    i: u8,
    n_char: u8,
    code: u8,
}
static mut tk_coalesce: Token =
    Token { z: c"coalesce".as_ptr() as *const i8, n: 8 as u32 };
static z_key_text: [i8; 34] =
    [110 as i8, 97 as i8, 116 as i8, 117 as i8, 114 as i8, 97 as i8,
            108 as i8, 101 as i8, 102 as i8, 116 as i8, 111 as i8, 117 as i8,
            116 as i8, 101 as i8, 114 as i8, 105 as i8, 103 as i8, 104 as i8,
            116 as i8, 102 as i8, 117 as i8, 108 as i8, 108 as i8, 105 as i8,
            110 as i8, 110 as i8, 101 as i8, 114 as i8, 99 as i8, 114 as i8,
            111 as i8, 115 as i8, 115 as i8, 0 as i8];
static a_keyword: [sqlite3_join_type_s0_N20sqlite3_join_type_s0; 7] =
    [sqlite3_join_type_s0_N20sqlite3_join_type_s0 {
                i: 0 as u8,
                n_char: 7 as u8,
                code: 4 as u8,
            },
            sqlite3_join_type_s0_N20sqlite3_join_type_s0 {
                i: 6 as u8,
                n_char: 4 as u8,
                code: (8 | 32) as u8,
            },
            sqlite3_join_type_s0_N20sqlite3_join_type_s0 {
                i: 10 as u8,
                n_char: 5 as u8,
                code: 32 as u8,
            },
            sqlite3_join_type_s0_N20sqlite3_join_type_s0 {
                i: 14 as u8,
                n_char: 5 as u8,
                code: (16 | 32) as u8,
            },
            sqlite3_join_type_s0_N20sqlite3_join_type_s0 {
                i: 19 as u8,
                n_char: 4 as u8,
                code: (8 | 16 | 32) as u8,
            },
            sqlite3_join_type_s0_N20sqlite3_join_type_s0 {
                i: 23 as u8,
                n_char: 5 as u8,
                code: 1 as u8,
            },
            sqlite3_join_type_s0_N20sqlite3_join_type_s0 {
                i: 28 as u8,
                n_char: 5 as u8,
                code: (1 | 2) as u8,
            }];
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
    fn sqlite3_row_set_clear(_: *mut ())
    -> ();
    fn sqlite3_expr_skip_collate_and_likely(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_is_true_or_false(_: *const i8)
    -> u32;
    static sqlite3_ctype_map: [u8; 0];
    fn sqlite3_str_i_hash(_: *const i8)
    -> u8;
    fn sqlite3_oom_fault(_: *mut sqlite3)
    -> *mut ();
    fn sqlite3_expr_affinity(p_expr_1: *const Expr)
    -> i8;
    fn sqlite3_expr_data_type(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_affinity_type(_: *const i8, _: *mut Column)
    -> i8;
    static sqlite3_std_type_affinity: [i8; 0];
    static mut sqlite3_std_type: [*const i8; 0];
    fn sqlite3_expr_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_src_list_append_from_term(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token, _: *mut Token, _: *mut Select,
    _: *mut OnOrUsing)
    -> *mut SrcList;
    fn sqlite3_src_list_delete(_: *mut sqlite3, _: *mut SrcList)
    -> ();
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_with_delete_generic(_: *mut sqlite3, _: *mut ())
    -> ();
    fn sqlite3_src_list_assign_cursors(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_src_item_attach_subquery(_: *mut Parse, _: *mut SrcItem,
    _: *mut Select, _: i32)
    -> i32;
    fn sqlite3_locate_table_item(_: *mut Parse, flags: u32, _: *mut SrcItem)
    -> *mut Table;
    fn sqlite3_view_get_column_names(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_id_list_append(_: *mut Parse, _: *mut IdList, _: *mut Token)
    -> *mut IdList;
    fn sqlite3_create_column_expr(_: *mut sqlite3, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_id_list_index(_: *mut IdList, _: *const i8)
    -> i32;
    fn sqlite3_schema_to_index(db: *mut sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_expr_set_error_offset(_: *mut Expr, _: i32)
    -> ();
    fn sqlite3_rowid_alias(p_tab_1: *mut Table)
    -> *const i8;
    fn sqlite3_match_e_name(_: *const ExprList_item, _: *const i8,
    _: *const i8, _: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_rename_token_remap(_: *mut Parse, p_to_1: *const (),
    p_from_1: *const ())
    -> ();
    fn sqlite3_resolve_select_names(_: *mut Parse, _: *mut Select,
    _: *mut NameContext)
    -> ();
    fn sqlite3_delete_table(_: *mut sqlite3, _: *mut Table)
    -> ();
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
    fn sqlite3_row_set_insert(_: *mut RowSet, _: i64)
    -> ();
    fn sqlite3_row_set_test(_: *mut RowSet, i_batch_1: i32, _: i64)
    -> i32;
    fn sqlite3_row_set_next(_: *mut RowSet, _: *mut i64)
    -> i32;
    fn sqlite3_create_view(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: *mut ExprList, _: *mut Select, _: i32, _: i32)
    -> ();
    fn sqlite3_drop_table(_: *mut Parse, _: *mut SrcList, _: i32, _: i32)
    -> ();
    fn sqlite3_code_drop_table(_: *mut Parse, _: *mut Table, _: i32, _: i32)
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
    fn sqlite3_src_list_indexed_by(_: *mut Parse, _: *mut SrcList,
    _: *mut Token)
    -> ();
    fn sqlite3_src_list_func_args(_: *mut Parse, _: *mut SrcList,
    _: *mut ExprList)
    -> ();
    fn sqlite3_src_list_shift_join_type(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_id_list_delete(_: *mut sqlite3, _: *mut IdList)
    -> ();
    fn sqlite3_clear_on_or_using(_: *mut sqlite3, _: *mut OnOrUsing)
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
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_expr_implies_non_null_row(_: *mut Expr, _: i32, _: i32)
    -> i32;
    fn sqlite3_select_dup(_: *mut sqlite3, _: *const Select, _: i32)
    -> *mut Select;
    fn sqlite3_expr_is_vector(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_vector_error_msg(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_expr_dup(_: *mut sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_truth_value(_: *const Expr)
    -> i32;
    fn sqlite3_expr_add_collate_string(_: *const Parse, _: *mut Expr,
    _: *const i8)
    -> *mut Expr;
    fn sqlite3_expr_col_used(_: *mut Expr)
    -> Bitmask;
    fn sqlite3_agg_info_persist_walker_init(_: *mut Walker, _: *mut Parse)
    -> ();
    fn sqlite3_with_delete(_: *mut sqlite3, _: *mut With)
    -> ();
    fn sqlite3_expr_code_expr_list(_: *mut Parse, _: *mut ExprList, _: i32,
    _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_code_move(_: *mut Parse, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_nn_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_expr_is_integer(_: *const Expr, _: *mut i32, _: *mut Parse)
    -> i32;
    fn sqlite3_log_est(_: u64)
    -> LogEst;
    fn sqlite3_expr_code(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_list_dup(_: *mut sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_resolve_order_group_by(_: *mut Parse, _: *mut Select,
    _: *mut ExprList, _: *const i8)
    -> i32;
    fn sqlite3_log_est_add(_: LogEst, _: LogEst)
    -> LogEst;
    fn sqlite3_expr_is_constant(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_is_binary(_: *const CollSeq)
    -> i32;
    fn sqlite3_expr_compare_coll_seq(_: *mut Parse, _: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_select_expr_height(_: *const Select)
    -> i32;
    fn sqlite3_expr_is_single_table_constraint(_: *mut Expr,
    _: *const SrcList, _: i32, _: i32)
    -> i32;
    fn sqlite3_expr_is_constant_or_group_by(_: *mut Parse, _: *mut Expr,
    _: *mut ExprList)
    -> i32;
    fn sqlite3_expr_list_compare(_: *const ExprList, _: *const ExprList,
    _: i32)
    -> i32;
    fn sqlite3_where_begin(_: *mut Parse, _: *mut SrcList, _: *mut Expr,
    _: *mut ExprList, _: *mut ExprList, _: *mut Select, _: u16, _: i32)
    -> *mut WhereInfo;
    fn sqlite3_where_output_row_count(_: *mut WhereInfo)
    -> LogEst;
    fn sqlite3_where_is_distinct(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_is_ordered(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_order_by_limit_opt_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_continue_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_break_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_end(_: *mut WhereInfo)
    -> ();
    fn sqlite3_expr_analyze_agg_list(_: *mut NameContext, _: *mut ExprList)
    -> ();
    fn sqlite3_expr_analyze_aggregates(_: *mut NameContext, _: *mut Expr)
    -> ();
    fn sqlite3_expr_can_be_null(_: *const Expr)
    -> i32;
    fn sqlite3_expr_null_register_range(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_where_is_sorted(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_expr_to_register(p_expr_1: *mut Expr, i_reg_1: i32)
    -> ();
    fn sqlite3_expr_if_false(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_code_verify_schema(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_key_info_of_index(_: *mut Parse, _: *mut Index)
    -> *mut KeyInfo;
    fn sqlite3_where_min_max_opt_early_out(_: *mut Vdbe, _: *mut WhereInfo)
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
    fn sqlite3_expr_code_generated_column(_: *mut Parse, _: *mut Table,
    _: *mut Column, _: i32)
    -> ();
    fn sqlite3_expr_code_copy(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_factorable(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_run_just_once(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_code_temp(_: *mut Parse, _: *mut Expr, _: *mut i32)
    -> i32;
    fn sqlite3_expr_code_target(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_if_true(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
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
    fn sqlite3_expr_implies_expr(_: *const Parse, _: *const Expr,
    _: *const Expr, _: i32)
    -> i32;
    fn sqlite3_expr_covered_by_index(_: *mut Expr, i_cur_1: i32,
    p_idx_1: *mut Index)
    -> i32;
    fn sqlite3_references_src_list(_: *mut Parse, _: *mut Expr,
    _: *mut SrcList)
    -> i32;
    fn sqlite3_prng_save_state()
    -> ();
    fn sqlite3_prng_restore_state()
    -> ();
    fn sqlite3_rollback_all(_: *mut sqlite3, _: i32)
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
    fn sqlite3_expr_id_to_true_false(_: *mut Expr)
    -> i32;
    fn sqlite3_expr_is_constant_or_function(_: *mut Expr, _: u8)
    -> i32;
    fn sqlite3_expr_needs_no_affinity_change(_: *const Expr, _: i8)
    -> i32;
    fn sqlite3_expr_is_like_operator(_: *const Expr)
    -> i32;
    fn sqlite3_is_rowid(_: *const i8)
    -> i32;
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
    fn sqlite3_src_list_dup(_: *mut sqlite3, _: *const SrcList, _: i32)
    -> *mut SrcList;
    fn sqlite3_id_list_dup(_: *mut sqlite3, _: *const IdList)
    -> *mut IdList;
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
    fn sqlite3_create_foreign_key(_: *mut Parse, _: *mut ExprList,
    _: *mut Token, _: *mut ExprList, _: i32)
    -> ();
    fn sqlite3_defer_foreign_key(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_auth_read(_: *mut Parse, _: *mut Expr, _: *mut Schema,
    _: *mut SrcList)
    -> ();
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
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_set_text_encoding(db: *mut sqlite3, _: u8)
    -> ();
    fn sqlite3_expr_coll_seq_match(_: *mut Parse, _: *const Expr,
    _: *const Expr)
    -> i32;
    fn sqlite3_expr_add_collate_token(p_parse_1: *const Parse, _: *mut Expr,
    _: *const Token, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_skip_collate(_: *mut Expr)
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
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
    -> i32;
    fn sqlite3_resolve_expr_list_names(_: *mut NameContext, _: *mut ExprList)
    -> i32;
    fn sqlite3_resolve_self_reference(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut Expr, _: *mut ExprList)
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
    fn sqlite3_reprepare(_: *mut Vdbe)
    -> i32;
    fn sqlite3_expr_list_check_length(_: *mut Parse, _: *mut ExprList,
    _: *const i8)
    -> ();
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
    fn sqlite3_expr_check_height(_: *mut Parse, _: i32)
    -> i32;
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
    fn sqlite3_vector_field_subexpr(_: *mut Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_for_vector_field(_: *mut Parse, _: *mut Expr, _: i32,
    _: i32)
    -> *mut Expr;
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
struct WindowRewrite {
    _opaque: [u8; 0],
}