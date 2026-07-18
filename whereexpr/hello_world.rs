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
    p_parse: *mut Parse,
    p_tab_list: *mut SrcList,
    p_order_by: *mut ExprList,
    p_result_set: *mut ExprList,
    p_select: *mut Select,
    ai_cur_one_pass: [i32; 2],
    i_continue: i32,
    i_break: i32,
    saved_n_query_loop: i32,
    wctrl_flags: u16,
    i_limit: LogEst,
    n_level: u8,
    n_ob_sat: i8,
    e_one_pass: u8,
    e_distinct: u8,
    _bitfield_1: u32,
    n_row_out: LogEst,
    i_top: i32,
    i_end_where: i32,
    p_loops: *mut WhereLoop,
    p_mem_to_free: *mut WhereMemBlock,
    rev_mask: Bitmask,
    s_wc: WhereClause,
    s_mask_set: WhereMaskSet,
    a: [WhereLevel; 0],
}
impl WhereInfo {
    fn b_deferred_seek(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_b_deferred_seek(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn untested_terms(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_untested_terms(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn b_ordered_inner_loop(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_b_ordered_inner_loop(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn sorted(&self) -> i32 { ((self._bitfield_1 >> 3u32) & 0x1u32) as i32 }
    fn set_sorted(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn b_star_done(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_b_star_done(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn b_star_used(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_b_star_used(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereLoop {
    prereq: Bitmask,
    mask_self: Bitmask,
    i_tab: u8,
    i_sort_idx: u8,
    r_setup: LogEst,
    r_run: LogEst,
    n_out: LogEst,
    u: WhereLoop_u0,
    ws_flags: u32,
    n_l_term: u16,
    n_skip: u16,
    n_l_slot: u16,
    a_l_term: *mut *mut WhereTerm,
    p_next_loop: *mut WhereLoop,
    a_l_term_space: [*mut WhereTerm; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
union WhereLoop_u0 {
    btree: WhereLoop_u0_s0,
    vtab: WhereLoop_u0_s1,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereLoop_u0_s0 {
    n_eq: u16,
    n_btm: u16,
    n_top: u16,
    n_distinct_col: u16,
    p_index: *mut Index,
    p_order_by: *mut ExprList,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereLoop_u0_s1 {
    idx_num: i32,
    _bitfield_1: u32,
    is_ordered: i8,
    omit_mask: u16,
    idx_str: *mut i8,
    m_handle_in: u32,
}
impl WhereLoop_u0_s1 {
    fn need_free(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_need_free(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn b_omit_offset(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_b_omit_offset(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn b_idx_num_hex(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_b_idx_num_hex(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereTerm {
    p_expr: *mut Expr,
    p_wc: *mut WhereClause,
    truth_prob: LogEst,
    wt_flags: u16,
    e_operator: u16,
    n_child: u8,
    e_match_op: u8,
    i_parent: i32,
    left_cursor: i32,
    u: WhereTerm_u0,
    prereq_right: Bitmask,
    prereq_all: Bitmask,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereClause {
    p_w_info: *mut WhereInfo,
    p_outer: *mut WhereClause,
    op: u8,
    has_or: u8,
    n_term: i32,
    n_slot: i32,
    n_base: i32,
    a: *mut WhereTerm,
    a_static: [WhereTerm; 8],
}
#[repr(C)]
#[derive(Copy, Clone)]
union WhereTerm_u0 {
    x: WhereTerm_u0_s0,
    p_or_info: *mut WhereOrInfo,
    p_and_info: *mut WhereAndInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereTerm_u0_s0 {
    left_column: i32,
    i_field: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereOrInfo {
    wc: WhereClause,
    indexable: Bitmask,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereAndInfo {
    wc: WhereClause,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereMemBlock {
    p_next: *mut WhereMemBlock,
    sz: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereMaskSet {
    b_var_select: i32,
    n: i32,
    ix: [i32; 64],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereLevel {
    i_left_join: i32,
    i_tab_cur: i32,
    i_idx_cur: i32,
    addr_brk: i32,
    addr_halt: i32,
    addr_nxt: i32,
    addr_skip: i32,
    addr_cont: i32,
    addr_first: i32,
    addr_body: i32,
    reg_bignull: i32,
    addr_bignull: i32,
    i_like_rep_cntr: u32,
    addr_like_rep: i32,
    reg_filter: i32,
    p_rj: *mut WhereRightJoin,
    i_from: u8,
    op: u8,
    p3: u8,
    p5: u8,
    p1: i32,
    p2: i32,
    u: WhereLevel_u0,
    p_w_loop: *mut WhereLoop,
    not_ready: Bitmask,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereRightJoin {
    i_match: i32,
    reg_bloom: i32,
    reg_return: i32,
    addr_subrtn: i32,
    end_subrtn: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
union WhereLevel_u0 {
    in_: WhereLevel_u0_s0,
    p_covering_idx: *mut Index,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereLevel_u0_s0 {
    n_in: i32,
    a_in_loop: *mut InLoop,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct InLoop {
    i_cur: i32,
    addr_in_top: i32,
    i_base: i32,
    n_prefix: i32,
    e_end_loop_op: u8,
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
pub extern "C" fn sqlite3_expr_is_like_operator(p_expr: &Expr) -> i32 {
    unsafe {
        unsafe {
            let mut i: i32 = 0;
            { let _ = 0; };
            { let _ = 0; };
            {
                i = 0;
                '__b0: loop {
                    if !(i <
                                    (core::mem::size_of::<[sqlite3_expr_is_like_operator_s0_N32sqlite3_expr_is_like_operator_s0; 4]>()
                                                as u64 / 16) as i32) {
                        break '__b0;
                    }
                    '__c0: loop {
                        if unsafe {
                                    sqlite3_str_i_cmp((*p_expr).u.z_token as *const i8,
                                        a_op[i as usize].z_op)
                                } == 0 {
                            return a_op[i as usize].e_op as i32;
                        }
                        break '__c0;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            return 0;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WherePath {
    mask_loop: Bitmask,
    rev_loop: Bitmask,
    n_row: LogEst,
    r_cost: LogEst,
    r_unsort: LogEst,
    is_ordered: i8,
    a_loop: *mut *mut WhereLoop,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereLoopBuilder {
    p_w_info: *mut WhereInfo,
    p_wc: *mut WhereClause,
    p_new: *mut WhereLoop,
    p_or_set: *mut WhereOrSet,
    bld_flags1: u8,
    bld_flags2: u8,
    i_plan_limit: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereOrSet {
    n: u16,
    a: [WhereOrCost; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereOrCost {
    prereq: Bitmask,
    r_run: LogEst,
    n_out: LogEst,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct WhereScan {
    p_orig_wc: *mut WhereClause,
    p_wc: *mut WhereClause,
    z_coll_name: *const i8,
    p_idx_expr: *mut Expr,
    k: i32,
    op_mask: u32,
    idxaff: i8,
    i_equiv: u8,
    n_equiv: u8,
    ai_cur: [i32; 11],
    ai_column: [i16; 11],
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_clause_init(p_wc: &mut WhereClause,
    p_w_info: *mut WhereInfo) -> () {
    (*p_wc).p_w_info = p_w_info;
    (*p_wc).has_or = 0 as u8;
    (*p_wc).p_outer = core::ptr::null_mut();
    (*p_wc).n_term = 0;
    (*p_wc).n_base = 0;
    (*p_wc).n_slot =
        (core::mem::size_of::<[WhereTerm; 8]>() as u64 /
                core::mem::size_of::<WhereTerm>() as u64) as i32;
    (*p_wc).a = &raw mut (*p_wc).a_static[0 as usize] as *mut WhereTerm;
}
extern "C" fn where_or_info_delete(db: *mut sqlite3, p: *mut WhereOrInfo)
    -> () {
    sqlite3_where_clause_clear(unsafe { &(*p).wc });
    unsafe { sqlite3_db_free(db, p as *mut ()) };
}
extern "C" fn where_and_info_delete(db: *mut sqlite3, p: *mut WhereAndInfo)
    -> () {
    sqlite3_where_clause_clear(unsafe { &(*p).wc });
    unsafe { sqlite3_db_free(db, p as *mut ()) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_clause_clear(p_wc: &WhereClause) -> () {
    unsafe {
        let db: *mut sqlite3 =
            unsafe { (*unsafe { (*(*p_wc).p_w_info).p_parse }).db };
        { let _ = 0; };
        if (*p_wc).n_term > 0 {
            let mut a: *mut WhereTerm = (*p_wc).a;
            let a_last: *mut WhereTerm =
                unsafe {
                    &mut *(*p_wc).a.offset(((*p_wc).n_term - 1) as isize)
                };
            loop {
                { let _ = 0; };
                if unsafe { (*a).wt_flags } as i32 & 1 != 0 {
                    unsafe { sqlite3_expr_delete(db, unsafe { (*a).p_expr }) };
                }
                if unsafe { (*a).wt_flags } as i32 & (16 | 32) != 0 {
                    if unsafe { (*a).wt_flags } as i32 & 16 != 0 {
                        { let _ = 0; };
                        where_or_info_delete(db, unsafe { (*a).u.p_or_info });
                    } else {
                        { let _ = 0; };
                        where_and_info_delete(db, unsafe { (*a).u.p_and_info });
                    }
                }
                if a == a_last { break; }
                {
                    let __p = &mut a;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
    }
}
extern "C" fn where_clause_insert(p_wc_1: *mut WhereClause, p: *mut Expr,
    wt_flags_1: u16) -> i32 {
    let mut p_term: *mut WhereTerm = core::ptr::null_mut();
    let mut idx: i32 = 0;
    if unsafe { (*p_wc_1).n_term } >= unsafe { (*p_wc_1).n_slot } {
        let p_old: *mut WhereTerm = unsafe { (*p_wc_1).a };
        let db: *mut sqlite3 =
            unsafe {
                (*unsafe { (*unsafe { (*p_wc_1).p_w_info }).p_parse }).db
            };
        unsafe {
            (*p_wc_1).a =
                unsafe {
                        sqlite3_where_malloc(unsafe { (*p_wc_1).p_w_info },
                            core::mem::size_of::<WhereTerm>() as u64 *
                                    unsafe { (*p_wc_1).n_slot } as u64 * 2 as u64)
                    } as *mut WhereTerm
        };
        if unsafe { (*p_wc_1).a } == core::ptr::null_mut() {
            if wt_flags_1 as i32 & 1 != 0 {
                unsafe { sqlite3_expr_delete(db, p) };
            }
            unsafe { (*p_wc_1).a = p_old };
            return 0;
        }
        unsafe {
            memcpy(unsafe { (*p_wc_1).a } as *mut (), p_old as *const (),
                core::mem::size_of::<WhereTerm>() as u64 *
                    unsafe { (*p_wc_1).n_term } as u64)
        };
        unsafe { (*p_wc_1).n_slot = unsafe { (*p_wc_1).n_slot } * 2 };
    }
    p_term =
        unsafe {
            unsafe {
                (*p_wc_1).a.offset({
                            idx =
                                {
                                    let __p = unsafe { &mut (*p_wc_1).n_term };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            idx
                        } as isize)
            }
        };
    if wt_flags_1 as i32 & 2 == 0 {
        unsafe { (*p_wc_1).n_base = unsafe { (*p_wc_1).n_term } };
    }
    if !(p).is_null() && unsafe { (*p).flags } & 524288 as u32 != 0 as u32 {
        unsafe {
            (*p_term).truth_prob =
                (unsafe { sqlite3_log_est(unsafe { (*p).i_table } as u64) } as
                            i32 - 270) as LogEst
        };
    } else { unsafe { (*p_term).truth_prob = 1 as LogEst }; }
    unsafe {
        (*p_term).p_expr = unsafe { sqlite3_expr_skip_collate_and_likely(p) }
    };
    unsafe { (*p_term).wt_flags = wt_flags_1 };
    unsafe { (*p_term).p_wc = p_wc_1 };
    unsafe { (*p_term).i_parent = -1 };
    unsafe {
        memset(unsafe { &raw mut (*p_term).e_operator } as *mut (), 0,
            core::mem::size_of::<WhereTerm>() as u64 -
                core::mem::offset_of!(WhereTerm, e_operator) as u64)
    };
    return idx;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_split(p_wc: *mut WhereClause,
    p_expr: *mut Expr, op: u8) -> () {
    let p_e2: *const Expr =
        unsafe { sqlite3_expr_skip_collate_and_likely(p_expr) } as
            *const Expr;
    unsafe { (*p_wc).op = op };
    { let _ = 0; };
    if p_e2 == core::ptr::null_mut() { return; }
    if unsafe { (*p_e2).op } as i32 != op as i32 {
        where_clause_insert(p_wc, p_expr, 0 as u16);
    } else {
        sqlite3_where_split(p_wc, unsafe { (*p_e2).p_left }, op);
        sqlite3_where_split(p_wc, unsafe { (*p_e2).p_right }, op);
    }
}
extern "C" fn where_add_limit_expr(p_wc_1: *mut WhereClause, i_reg_1: i32,
    p_expr_1: *const Expr, i_csr_1: i32, e_match_op_1: i32) -> () {
    let p_parse: *mut Parse =
        unsafe { (*unsafe { (*p_wc_1).p_w_info }).p_parse };
    let db: *mut sqlite3 = unsafe { (*p_parse).db };
    let mut p_new: *mut Expr = core::ptr::null_mut();
    let mut i_val: i32 = 0;
    if unsafe {
                    sqlite3_expr_is_integer(p_expr_1 as *const Expr, &mut i_val,
                        p_parse)
                } != 0 && i_val >= 0 {
        let p_val: *mut Expr = unsafe { sqlite3_expr_int32(db, i_val) };
        if p_val == core::ptr::null_mut() { return; }
        p_new =
            unsafe {
                sqlite3_p_expr(p_parse, 47, core::ptr::null_mut(), p_val)
            };
    } else {
        let p_val_1: *mut Expr =
            unsafe { sqlite3_expr_alloc(db, 176, core::ptr::null(), 0) };
        if p_val_1 == core::ptr::null_mut() { return; }
        unsafe { (*p_val_1).i_table = i_reg_1 };
        p_new =
            unsafe {
                sqlite3_p_expr(p_parse, 47, core::ptr::null_mut(), p_val_1)
            };
    }
    if !(p_new).is_null() {
        let mut p_term: *mut WhereTerm = core::ptr::null_mut();
        let mut idx: i32 = 0;
        idx = where_clause_insert(p_wc_1, p_new, (1 | 2) as u16);
        p_term = unsafe { unsafe { (*p_wc_1).a.offset(idx as isize) } };
        unsafe { (*p_term).left_cursor = i_csr_1 };
        unsafe { (*p_term).e_operator = 64 as u16 };
        unsafe { (*p_term).e_match_op = e_match_op_1 as u8 };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_add_limit(p_wc: *mut WhereClause, p: &Select)
    -> () {
    unsafe {
        { let _ = 0; };
        if (*p).p_group_by == core::ptr::null_mut() &&
                    (*p).sel_flags & (1 | 8) as u32 == 0 as u32 &&
                (unsafe { (*(*p).p_src).n_src } == 1 &&
                    unsafe {
                                (*unsafe {
                                                (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).p_s_tab
                                            }).e_tab_type
                            } as i32 == 1) {
            let p_order_by: *const ExprList =
                (*p).p_order_by as *const ExprList;
            let i_csr: i32 =
                unsafe {
                    (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)).i_cursor
                };
            let mut ii: i32 = 0;
            {
                ii = 0;
                '__b2: loop {
                    if !(ii < unsafe { (*p_wc).n_term }) { break '__b2; }
                    '__c2: loop {
                        if unsafe {
                                            (*unsafe { (*p_wc).a.offset(ii as isize) }).wt_flags
                                        } as i32 & 4 != 0 {
                            { let _ = 0; };
                            { let _ = 0; };
                            break '__c2;
                        }
                        if unsafe {
                                    (*unsafe { (*p_wc).a.offset(ii as isize) }).n_child
                                } != 0 {
                            break '__c2;
                        }
                        if unsafe {
                                        (*unsafe { (*p_wc).a.offset(ii as isize) }).left_cursor
                                    } == i_csr &&
                                unsafe {
                                        (*unsafe { (*p_wc).a.offset(ii as isize) }).prereq_right
                                    } == 0 as u64 {
                            break '__c2;
                        }
                        if unsafe {
                                    (*unsafe { (*p_wc).a.offset(ii as isize) }).i_parent
                                } >= 0 {
                            let p_parent: *const WhereTerm =
                                unsafe {
                                        &raw mut *unsafe {
                                                    (*p_wc).a.offset(unsafe {
                                                                (*unsafe { (*p_wc).a.offset(ii as isize) }).i_parent
                                                            } as isize)
                                                }
                                    } as *const WhereTerm;
                            if unsafe { (*p_parent).left_cursor } == i_csr &&
                                        unsafe { (*p_parent).prereq_right } == 0 as u64 &&
                                    unsafe { (*p_parent).n_child } as i32 == 1 {
                                break '__c2;
                            }
                        }
                        return;
                        break '__c2;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            if !(p_order_by).is_null() {
                {
                    ii = 0;
                    '__b3: loop {
                        if !(ii < unsafe { (*p_order_by).n_expr }) { break '__b3; }
                        '__c3: loop {
                            let p_expr: *const Expr =
                                unsafe {
                                        (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                        *mut ExprList_item).offset(ii as isize)).p_expr
                                    } as *const Expr;
                            if unsafe { (*p_expr).op } as i32 != 168 { return; }
                            if unsafe { (*p_expr).i_table } != i_csr { return; }
                            if unsafe {
                                                (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                                    *mut ExprList_item).offset(ii as isize)).fg.sort_flags
                                            } as i32 & 2 != 0 {
                                return;
                            }
                            break '__c3;
                        }
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            { let _ = 0; };
            if (*p).i_offset != 0 && (*p).sel_flags & 256 as u32 == 0 as u32 {
                where_add_limit_expr(p_wc, (*p).i_offset,
                    unsafe { (*(*p).p_limit).p_right } as *const Expr, i_csr,
                    74);
            }
            if (*p).i_offset == 0 || (*p).sel_flags & 256 as u32 == 0 as u32 {
                where_add_limit_expr(p_wc, (*p).i_limit,
                    unsafe { (*(*p).p_limit).p_left } as *const Expr, i_csr,
                    73);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_expr_list_usage(p_mask_set: *mut WhereMaskSet,
    p_list: *mut ExprList) -> Bitmask {
    let mut i: i32 = 0;
    let mut mask: Bitmask = 0 as Bitmask;
    if !(p_list).is_null() {
        {
            i = 0;
            '__b4: loop {
                if !(i < unsafe { (*p_list).n_expr }) { break '__b4; }
                '__c4: loop {
                    mask |=
                        sqlite3_where_expr_usage(p_mask_set,
                            unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(i as isize)).p_expr
                            });
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return mask;
}
extern "C" fn expr_select_usage(p_mask_set_1: *mut WhereMaskSet,
    mut p_s_1: *const Select) -> Bitmask {
    unsafe {
        let mut mask: Bitmask = 0 as Bitmask;
        while !(p_s_1).is_null() {
            let p_src: *const SrcList =
                unsafe { (*p_s_1).p_src } as *const SrcList;
            mask |=
                sqlite3_where_expr_list_usage(p_mask_set_1,
                    unsafe { (*p_s_1).p_e_list });
            mask |=
                sqlite3_where_expr_list_usage(p_mask_set_1,
                    unsafe { (*p_s_1).p_group_by });
            mask |=
                sqlite3_where_expr_list_usage(p_mask_set_1,
                    unsafe { (*p_s_1).p_order_by });
            mask |=
                sqlite3_where_expr_usage(p_mask_set_1,
                    unsafe { (*p_s_1).p_where });
            mask |=
                sqlite3_where_expr_usage(p_mask_set_1,
                    unsafe { (*p_s_1).p_having });
            if p_src != core::ptr::null_mut() {
                let mut i: i32 = 0;
                {
                    i = 0;
                    '__b6: loop {
                        if !(i < unsafe { (*p_src).n_src }) { break '__b6; }
                        '__c6: loop {
                            if unsafe {
                                        (*(unsafe { (*p_src).a.as_ptr() } as
                                                            *mut SrcItem).offset(i as isize)).fg.is_subquery()
                                    } != 0 {
                                mask |=
                                    expr_select_usage(p_mask_set_1,
                                        unsafe {
                                                (*unsafe {
                                                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                    *mut SrcItem).offset(i as isize)).u4.p_subq
                                                            }).p_select
                                            } as *const Select);
                            }
                            if unsafe {
                                            (*(unsafe { (*p_src).a.as_ptr() } as
                                                                *mut SrcItem).offset(i as isize)).fg.is_using()
                                        } as i32 == 0 {
                                mask |=
                                    sqlite3_where_expr_usage(p_mask_set_1,
                                        unsafe {
                                            (*(unsafe { (*p_src).a.as_ptr() } as
                                                                *mut SrcItem).offset(i as isize)).u3.p_on
                                        });
                            }
                            if unsafe {
                                        (*(unsafe { (*p_src).a.as_ptr() } as
                                                            *mut SrcItem).offset(i as isize)).fg.is_tab_func()
                                    } != 0 {
                                mask |=
                                    sqlite3_where_expr_list_usage(p_mask_set_1,
                                        unsafe {
                                            (*(unsafe { (*p_src).a.as_ptr() } as
                                                                *mut SrcItem).offset(i as isize)).u1.p_func_arg
                                        });
                            }
                            break '__c6;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            p_s_1 = unsafe { (*p_s_1).p_prior };
        }
        return mask;
    }
}
extern "C" fn sqlite3_where_expr_usage_full(p_mask_set_1: *mut WhereMaskSet,
    p: &Expr) -> Bitmask {
    unsafe {
        let mut mask: Bitmask = 0 as Bitmask;
        mask =
            if (*p).op as i32 == 179 {
                unsafe { sqlite3_where_get_mask(p_mask_set_1, (*p).i_table) }
            } else { 0 as Bitmask };
        if !((*p).p_left).is_null() {
            mask |= sqlite3_where_expr_usage_nn(p_mask_set_1, (*p).p_left);
        }
        if !((*p).p_right).is_null() {
            mask |= sqlite3_where_expr_usage_nn(p_mask_set_1, (*p).p_right);
            { let _ = 0; };
        } else if (*p).flags & 4096 as u32 != 0 as u32 {
            if (*p).flags & 64 as u32 != 0 as u32 {
                unsafe { (*p_mask_set_1).b_var_select = 1 };
            }
            mask |=
                expr_select_usage(p_mask_set_1,
                    (*p).x.p_select as *const Select);
        } else if !((*p).x.p_list).is_null() {
            mask |=
                sqlite3_where_expr_list_usage(p_mask_set_1, (*p).x.p_list);
        }
        if ((*p).op as i32 == 172 || (*p).op as i32 == 169) &&
                (*p).flags & 16777216 as u32 != 0 as u32 {
            { let _ = 0; };
            mask |=
                sqlite3_where_expr_list_usage(p_mask_set_1,
                    unsafe { (*(*p).y.p_win).p_partition });
            mask |=
                sqlite3_where_expr_list_usage(p_mask_set_1,
                    unsafe { (*(*p).y.p_win).p_order_by });
            mask |=
                sqlite3_where_expr_usage(p_mask_set_1,
                    unsafe { (*(*p).y.p_win).p_filter });
        }
        return mask;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_expr_usage_nn(p_mask_set: *mut WhereMaskSet,
    p: *mut Expr) -> Bitmask {
    if unsafe { (*p).op } as i32 == 168 &&
            !(unsafe { (*p).flags } & 32 as u32 != 0 as u32) as i32 != 0 {
        return unsafe {
                sqlite3_where_get_mask(p_mask_set, unsafe { (*p).i_table })
            };
    } else if unsafe { (*p).flags } & (65536 | 8388608) as u32 != 0 as u32 {
        { let _ = 0; };
        return 0 as Bitmask;
    }
    return sqlite3_where_expr_usage_full(p_mask_set, unsafe { &*p });
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_expr_usage(p_mask_set: *mut WhereMaskSet,
    p: *mut Expr) -> Bitmask {
    return if !(p).is_null() {
            sqlite3_where_expr_usage_nn(p_mask_set, p)
        } else { 0 as Bitmask };
}
extern "C" fn allowed_op(op: i32) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if op > 58 { return 0; }
    if op >= 54 { return 1; }
    return (op == 50 || op == 51 || op == 45) as i32;
}
extern "C" fn expr_might_be_indexed2(p_from_1: &SrcList,
    ai_cur_col_1: *mut i32, p_expr_1: *mut Expr, mut j: i32) -> i32 {
    let mut p_idx: *const Index = core::ptr::null();
    let mut i: i32 = 0;
    let mut i_cur: i32 = 0;
    '__b7: loop {
        '__c7: loop {
            i_cur =
                unsafe {
                    (*((*p_from_1).a.as_ptr() as
                                    *mut SrcItem).offset(j as isize)).i_cursor
                };
            {
                p_idx =
                    unsafe {
                        (*unsafe {
                                        (*((*p_from_1).a.as_ptr() as
                                                        *mut SrcItem).offset(j as isize)).p_s_tab
                                    }).p_index
                    };
                '__b8: loop {
                    if !(!(p_idx).is_null()) { break '__b8; }
                    '__c8: loop {
                        if unsafe { (*p_idx).a_col_expr } == core::ptr::null_mut() {
                            break '__c8;
                        }
                        {
                            i = 0;
                            '__b9: loop {
                                if !(i < unsafe { (*p_idx).n_key_col } as i32) {
                                    break '__b9;
                                }
                                '__c9: loop {
                                    if unsafe {
                                                    *unsafe { (*p_idx).ai_column.offset(i as isize) }
                                                } as i32 != -2 {
                                        break '__c9;
                                    }
                                    { let _ = 0; };
                                    if unsafe {
                                                    sqlite3_expr_compare_skip(p_expr_1,
                                                        unsafe {
                                                            (*(unsafe { (*unsafe { (*p_idx).a_col_expr }).a.as_ptr() }
                                                                            as *mut ExprList_item).offset(i as isize)).p_expr
                                                        }, i_cur)
                                                } == 0 &&
                                            (unsafe {
                                                            sqlite3_expr_is_constant(core::ptr::null_mut(),
                                                                unsafe {
                                                                    (*(unsafe { (*unsafe { (*p_idx).a_col_expr }).a.as_ptr() }
                                                                                    as *mut ExprList_item).offset(i as isize)).p_expr
                                                                })
                                                        } == 0) as i32 != 0 {
                                        unsafe { *ai_cur_col_1.offset(0 as isize) = i_cur };
                                        unsafe { *ai_cur_col_1.offset(1 as isize) = -2 };
                                        return 1;
                                    }
                                    break '__c9;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        break '__c8;
                    }
                    p_idx = unsafe { (*p_idx).p_next };
                }
            }
            break '__c7;
        }
        if !({ let __p = &mut j; *__p += 1; *__p } < (*p_from_1).n_src) {
            break '__b7;
        }
    }
    return 0;
}
extern "C" fn expr_might_be_indexed(p_from_1: *mut SrcList,
    ai_cur_col_1: *mut i32, mut p_expr_1: *mut Expr, op: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_expr_1).op } as i32 == 177 && (op >= 55 && op <= 58) {
            { let _ = 0; };
            p_expr_1 =
                unsafe {
                    (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                    as *mut ExprList_item).offset(0 as isize)).p_expr
                };
        }
        if unsafe { (*p_expr_1).op } as i32 == 168 {
            unsafe {
                *ai_cur_col_1.offset(0 as isize) =
                    unsafe { (*p_expr_1).i_table }
            };
            unsafe {
                *ai_cur_col_1.offset(1 as isize) =
                    unsafe { (*p_expr_1).i_column } as i32
            };
            return 1;
        }
        {
            i = 0;
            '__b10: loop {
                if !(i < unsafe { (*p_from_1).n_src }) { break '__b10; }
                '__c10: loop {
                    let mut p_idx: *const Index = core::ptr::null();
                    {
                        p_idx =
                            unsafe {
                                (*unsafe {
                                                (*(unsafe { (*p_from_1).a.as_ptr() } as
                                                                *mut SrcItem).offset(i as isize)).p_s_tab
                                            }).p_index
                            };
                        '__b11: loop {
                            if !(!(p_idx).is_null()) { break '__b11; }
                            '__c11: loop {
                                if !(unsafe { (*p_idx).a_col_expr }).is_null() {
                                    return expr_might_be_indexed2(unsafe { &*p_from_1 },
                                            ai_cur_col_1, p_expr_1, i);
                                }
                                break '__c11;
                            }
                            p_idx = unsafe { (*p_idx).p_next };
                        }
                    }
                    break '__c10;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}
extern "C" fn operator_mask(op: i32) -> u16 {
    let mut c: u16 = 0 as u16;
    { let _ = 0; };
    if op >= 54 {
        { let _ = 0; };
        c = (2 << op - 54) as u16;
    } else if op == 50 {
        c = 1 as u16;
    } else if op == 51 {
        c = 256 as u16;
    } else { { let _ = 0; }; c = 128 as u16; }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    return c;
}
extern "C" fn mark_term_as_child(p_wc_1: &WhereClause, i_child_1: i32,
    i_parent_1: i32) -> () {
    unsafe {
        (*(*p_wc_1).a.offset(i_child_1 as isize)).i_parent = i_parent_1
    };
    unsafe {
        (*(*p_wc_1).a.offset(i_child_1 as isize)).truth_prob =
            unsafe { (*(*p_wc_1).a.offset(i_parent_1 as isize)).truth_prob }
    };
    { let _ = 0; };
    {
        let __p =
            unsafe {
                &mut (*(*p_wc_1).a.offset(i_parent_1 as isize)).n_child
            };
        let __t = *__p;
        *__p += 1;
        __t
    };
}
extern "C" fn term_is_equivalence(p_parse_1: *mut Parse, p_expr_1: &Expr,
    p_src_1: &SrcList) -> i32 {
    let mut aff1: i8 = 0 as i8;
    let mut aff2: i8 = 0 as i8;
    if !(unsafe { (*unsafe { (*p_parse_1).db }).db_opt_flags } & 128 as u32 ==
                        0 as u32) as i32 != 0 {
        return 0;
    }
    if (*p_expr_1).op as i32 != 54 && (*p_expr_1).op as i32 != 45 {
        return 0;
    }
    if (*p_expr_1).flags & (1 | 512) as u32 != 0 as u32 { return 0; }
    { let _ = 0; };
    if (*p_expr_1).op as i32 == 45 && (*p_src_1).n_src >= 2 &&
            unsafe {
                            (*((*p_src_1).a.as_ptr() as
                                                *mut SrcItem).offset(0 as isize)).fg.jointype
                        } as i32 & 64 != 0 {
        return 0;
    }
    aff1 =
        unsafe { sqlite3_expr_affinity((*p_expr_1).p_left as *const Expr) };
    aff2 =
        unsafe { sqlite3_expr_affinity((*p_expr_1).p_right as *const Expr) };
    if aff1 as i32 != aff2 as i32 &&
            (!(aff1 as i32 >= 67) as i32 != 0 ||
                !(aff2 as i32 >= 67) as i32 != 0) {
        return 0;
    }
    if (unsafe {
                        sqlite3_expr_coll_seq_match(p_parse_1,
                            (*p_expr_1).p_left as *const Expr,
                            (*p_expr_1).p_right as *const Expr)
                    } == 0) as i32 != 0 {
        return 0;
    }
    return 1;
}
extern "C" fn expr_commute(p_parse_1: *mut Parse, p_expr_1: &mut Expr)
    -> u16 {
    if unsafe { (*(*p_expr_1).p_left).op } as i32 == 177 ||
                unsafe { (*(*p_expr_1).p_right).op } as i32 == 177 ||
            unsafe {
                    sqlite3_binary_compare_coll_seq(p_parse_1,
                        (*p_expr_1).p_left as *const Expr,
                        (*p_expr_1).p_right as *const Expr)
                } !=
                unsafe {
                    sqlite3_binary_compare_coll_seq(p_parse_1,
                        (*p_expr_1).p_right as *const Expr,
                        (*p_expr_1).p_left as *const Expr)
                } {
        (*p_expr_1).flags ^= 1024 as u32;
    }
    {
        let t: *mut Expr = (*p_expr_1).p_right;
        (*p_expr_1).p_right = (*p_expr_1).p_left;
        (*p_expr_1).p_left = t;
    }
    if (*p_expr_1).op as i32 >= 55 {
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        (*p_expr_1).op = (((*p_expr_1).op as i32 - 55 ^ 2) + 55) as u8;
    }
    return 0 as u16;
}
extern "C" fn transfer_join_markings(p_derived_1: *mut Expr, p_base_1: &Expr)
    -> () {
    unsafe {
        if !(p_derived_1).is_null() &&
                (*p_base_1).flags & (1 | 2) as u32 != 0 as u32 {
            unsafe {
                (*p_derived_1).flags |= (*p_base_1).flags & (1 | 2) as u32
            };
            unsafe { (*p_derived_1).w.i_join = (*p_base_1).w.i_join };
        }
    }
}
extern "C" fn where_nth_subterm(p_term_1: *mut WhereTerm, n_1: i32)
    -> *mut WhereTerm {
    unsafe {
        if unsafe { (*p_term_1).e_operator } as i32 != 1024 {
            return if n_1 == 0 { p_term_1 } else { core::ptr::null_mut() };
        }
        if n_1 < unsafe { (*unsafe { (*p_term_1).u.p_and_info }).wc.n_term } {
            return unsafe {
                    &mut *unsafe {
                                (*unsafe {
                                                        (*p_term_1).u.p_and_info
                                                    }).wc.a.offset(n_1 as isize)
                            }
                };
        }
        return core::ptr::null_mut();
    }
}
extern "C" fn where_combine_disjuncts(p_src_1: *mut SrcList,
    p_wc_1: *mut WhereClause, p_one_1: &WhereTerm, p_two_1: &WhereTerm)
    -> () {
    let mut e_op: u16 =
        ((*p_one_1).e_operator as i32 | (*p_two_1).e_operator as i32) as u16;
    let mut db: *mut sqlite3 = core::ptr::null_mut();
    let mut p_new: *mut Expr = core::ptr::null_mut();
    let mut op: i32 = 0;
    let mut idx_new: i32 = 0;
    let mut p_a: *const Expr = core::ptr::null();
    let mut p_b: *const Expr = core::ptr::null();
    if ((*p_one_1).wt_flags as i32 | (*p_two_1).wt_flags as i32) & 128 != 0 {
        return;
    }
    if (*p_one_1).e_operator as i32 &
                (2 | 2 << 57 - 54 | 2 << 56 - 54 | 2 << 55 - 54 |
                    2 << 58 - 54) == 0 {
        return;
    }
    if (*p_two_1).e_operator as i32 &
                (2 | 2 << 57 - 54 | 2 << 56 - 54 | 2 << 55 - 54 |
                    2 << 58 - 54) == 0 {
        return;
    }
    if e_op as i32 & (2 | 2 << 57 - 54 | 2 << 56 - 54) != e_op as i32 &&
            e_op as i32 & (2 | 2 << 55 - 54 | 2 << 58 - 54) != e_op as i32 {
        return;
    }
    p_a = (*p_one_1).p_expr;
    p_b = (*p_two_1).p_expr;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe {
                sqlite3_expr_compare(core::ptr::null(),
                    unsafe { (*p_a).p_left } as *const Expr,
                    unsafe { (*p_b).p_left } as *const Expr, -1)
            } != 0 {
        return;
    }
    if unsafe {
                sqlite3_expr_compare(core::ptr::null(),
                    unsafe { (*p_a).p_right } as *const Expr,
                    unsafe { (*p_b).p_right } as *const Expr, -1)
            } != 0 {
        return;
    }
    if (unsafe { (*p_a).flags } & 1024 as u32 != 0 as u32) as i32 !=
            (unsafe { (*p_b).flags } & 1024 as u32 != 0 as u32) as i32 {
        return;
    }
    if e_op as i32 & e_op as i32 - 1 != 0 {
        if e_op as i32 & (2 << 57 - 54 | 2 << 56 - 54) != 0 {
            e_op = (2 << 56 - 54) as u16;
        } else { { let _ = 0; }; e_op = (2 << 58 - 54) as u16; }
    }
    db = unsafe { (*unsafe { (*unsafe { (*p_wc_1).p_w_info }).p_parse }).db };
    p_new = unsafe { sqlite3_expr_dup(db, p_a as *const Expr, 0) };
    if p_new == core::ptr::null_mut() { return; }
    {
        op = 54;
        '__b12: loop {
            if !(e_op as i32 != 2 << op - 54) { break '__b12; }
            '__c12: loop { { let _ = 0; }; break '__c12; }
            { let __p = &mut op; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { (*p_new).op = op as u8 };
    idx_new = where_clause_insert(p_wc_1, p_new, (2 | 1) as u16);
    expr_analyze(p_src_1, p_wc_1, idx_new);
}
extern "C" fn expr_analyze_or_term(p_src_1: *mut SrcList,
    p_wc_1: *mut WhereClause, idx_term_1: i32) -> () {
    unsafe {
        let p_w_info: *mut WhereInfo = unsafe { (*p_wc_1).p_w_info };
        let p_parse: *mut Parse = unsafe { (*p_w_info).p_parse };
        let db: *mut sqlite3 = unsafe { (*p_parse).db };
        let p_term: *mut WhereTerm =
            unsafe {
                &mut *unsafe { (*p_wc_1).a.offset(idx_term_1 as isize) }
            };
        let p_expr: *mut Expr = unsafe { (*p_term).p_expr };
        let mut i: i32 = 0;
        let mut p_or_wc: *mut WhereClause = core::ptr::null_mut();
        let mut p_or_term: *mut WhereTerm = core::ptr::null_mut();
        let mut p_or_info: *mut WhereOrInfo = core::ptr::null_mut();
        let mut chng_to_in: Bitmask = 0 as Bitmask;
        let mut indexable: Bitmask = 0 as Bitmask;
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            (*p_term).u.p_or_info =
                {
                    p_or_info =
                        unsafe {
                                sqlite3_db_malloc_zero(db,
                                    core::mem::size_of::<WhereOrInfo>() as u64)
                            } as *mut WhereOrInfo;
                    p_or_info
                }
        };
        if p_or_info == core::ptr::null_mut() { return; }
        unsafe { (*p_term).wt_flags |= 16 as u16 };
        p_or_wc = unsafe { &mut (*p_or_info).wc };
        unsafe {
            memset(unsafe { &raw mut (*p_or_wc).a_static[0 as usize] } as
                        *mut WhereTerm as *mut (), 0,
                core::mem::size_of::<[WhereTerm; 8]>() as u64)
        };
        sqlite3_where_clause_init(unsafe { &mut *p_or_wc }, p_w_info);
        sqlite3_where_split(p_or_wc, p_expr, 43 as u8);
        sqlite3_where_expr_analyze(p_src_1, p_or_wc);
        if unsafe { (*db).malloc_failed } != 0 { return; }
        { let _ = 0; };
        indexable = !(0 as Bitmask);
        chng_to_in = !(0 as Bitmask);
        {
            {
                i = unsafe { (*p_or_wc).n_term } - 1;
                p_or_term = unsafe { (*p_or_wc).a }
            };
            '__b13: loop {
                if !(i >= 0 && indexable != 0) { break '__b13; }
                '__c13: loop {
                    if unsafe { (*p_or_term).e_operator } as i32 & 511 == 0 {
                        let mut p_and_info: *mut WhereAndInfo =
                            core::ptr::null_mut();
                        { let _ = 0; };
                        chng_to_in = 0 as Bitmask;
                        p_and_info =
                            unsafe {
                                    sqlite3_db_malloc_raw_nn(db,
                                        core::mem::size_of::<WhereAndInfo>() as u64)
                                } as *mut WhereAndInfo;
                        if !(p_and_info).is_null() {
                            let mut p_and_wc: *mut WhereClause = core::ptr::null_mut();
                            let mut p_and_term: *const WhereTerm = core::ptr::null();
                            let mut j: i32 = 0;
                            let mut b: Bitmask = 0 as Bitmask;
                            unsafe { (*p_or_term).u.p_and_info = p_and_info };
                            unsafe { (*p_or_term).wt_flags |= 32 as u16 };
                            unsafe { (*p_or_term).e_operator = 1024 as u16 };
                            unsafe { (*p_or_term).left_cursor = -1 };
                            p_and_wc = unsafe { &mut (*p_and_info).wc };
                            unsafe {
                                memset(unsafe { &raw mut (*p_and_wc).a_static[0 as usize] }
                                            as *mut WhereTerm as *mut (), 0,
                                    core::mem::size_of::<[WhereTerm; 8]>() as u64)
                            };
                            sqlite3_where_clause_init(unsafe { &mut *p_and_wc },
                                unsafe { (*p_wc_1).p_w_info });
                            sqlite3_where_split(p_and_wc,
                                unsafe { (*p_or_term).p_expr }, 44 as u8);
                            sqlite3_where_expr_analyze(p_src_1, p_and_wc);
                            unsafe { (*p_and_wc).p_outer = p_wc_1 };
                            if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                                {
                                    { j = 0; p_and_term = unsafe { (*p_and_wc).a } };
                                    '__b14: loop {
                                        if !(j < unsafe { (*p_and_wc).n_term }) { break '__b14; }
                                        '__c14: loop {
                                            { let _ = 0; };
                                            if allowed_op(unsafe {
                                                                    (*unsafe { (*p_and_term).p_expr }).op
                                                                } as i32) != 0 ||
                                                    unsafe { (*p_and_term).e_operator } as i32 == 64 {
                                                b |=
                                                    unsafe {
                                                        sqlite3_where_get_mask(unsafe {
                                                                &mut (*p_w_info).s_mask_set
                                                            }, unsafe { (*p_and_term).left_cursor })
                                                    };
                                            }
                                            break '__c14;
                                        }
                                        {
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            {
                                                let __p = &mut p_and_term;
                                                let __t = *__p;
                                                *__p = unsafe { (*__p).offset(1) };
                                                __t
                                            }
                                        };
                                    }
                                }
                            }
                            indexable &= b;
                        }
                    } else if unsafe { (*p_or_term).wt_flags } as i32 & 8 != 0
                        {} else {
                        let mut b: Bitmask = 0 as Bitmask;
                        b =
                            unsafe {
                                sqlite3_where_get_mask(unsafe {
                                        &mut (*p_w_info).s_mask_set
                                    }, unsafe { (*p_or_term).left_cursor })
                            };
                        if unsafe { (*p_or_term).wt_flags } as i32 & 2 != 0 {
                            let p_other: *const WhereTerm =
                                unsafe {
                                        &raw mut *unsafe {
                                                    (*p_or_wc).a.offset(unsafe { (*p_or_term).i_parent } as
                                                            isize)
                                                }
                                    } as *const WhereTerm;
                            b |=
                                unsafe {
                                    sqlite3_where_get_mask(unsafe {
                                            &mut (*p_w_info).s_mask_set
                                        }, unsafe { (*p_other).left_cursor })
                                };
                        }
                        indexable &= b;
                        if unsafe { (*p_or_term).e_operator } as i32 & 2 == 0 {
                            chng_to_in = 0 as Bitmask;
                        } else { chng_to_in &= b; }
                    }
                    break '__c13;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                    {
                        let __p = &mut p_or_term;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        unsafe { (*p_or_info).indexable = indexable };
        unsafe { (*p_term).e_operator = 512 as u16 };
        unsafe { (*p_term).left_cursor = -1 };
        if indexable != 0 { unsafe { (*p_wc_1).has_or = 1 as u8 }; }
        if indexable != 0 && unsafe { (*p_or_wc).n_term } == 2 {
            let mut i_one: i32 = 0;
            let mut p_one: *mut WhereTerm = core::ptr::null_mut();
            while {
                        p_one =
                            where_nth_subterm(unsafe {
                                    &mut *unsafe { (*p_or_wc).a.offset(0 as isize) }
                                },
                                { let __p = &mut i_one; let __t = *__p; *__p += 1; __t });
                        p_one
                    } != core::ptr::null_mut() {
                let mut i_two: i32 = 0;
                let mut p_two: *mut WhereTerm = core::ptr::null_mut();
                while {
                            p_two =
                                where_nth_subterm(unsafe {
                                        &mut *unsafe { (*p_or_wc).a.offset(1 as isize) }
                                    },
                                    { let __p = &mut i_two; let __t = *__p; *__p += 1; __t });
                            p_two
                        } != core::ptr::null_mut() {
                    where_combine_disjuncts(p_src_1, p_wc_1, unsafe { &*p_one },
                        unsafe { &*p_two });
                }
            }
        }
        if chng_to_in != 0 {
            let mut ok_to_chng_to_in: i32 = 0;
            let mut i_column: i32 = -1;
            let mut i_cursor: i32 = -1;
            let mut j: i32 = 0;
            {
                j = 0;
                '__b17: loop {
                    if !(j < 2 && (ok_to_chng_to_in == 0) as i32 != 0) {
                        break '__b17;
                    }
                    '__c17: loop {
                        let mut p_left: *const Expr = core::ptr::null();
                        p_or_term = unsafe { (*p_or_wc).a };
                        {
                            i = unsafe { (*p_or_wc).n_term } - 1;
                            '__b18: loop {
                                if !(i >= 0) { break '__b18; }
                                '__c18: loop {
                                    { let _ = 0; };
                                    unsafe { (*p_or_term).wt_flags &= !64 as u16 };
                                    if unsafe { (*p_or_term).left_cursor } == i_cursor {
                                        { let _ = 0; };
                                        break '__c18;
                                    }
                                    if chng_to_in &
                                                unsafe {
                                                    sqlite3_where_get_mask(unsafe {
                                                            &mut (*p_w_info).s_mask_set
                                                        }, unsafe { (*p_or_term).left_cursor })
                                                } == 0 as u64 {
                                        { let _ = 0; };
                                        break '__c18;
                                    }
                                    { let _ = 0; };
                                    i_column = unsafe { (*p_or_term).u.x.left_column };
                                    i_cursor = unsafe { (*p_or_term).left_cursor };
                                    p_left =
                                        unsafe { (*unsafe { (*p_or_term).p_expr }).p_left };
                                    break '__b18;
                                    break '__c18;
                                }
                                {
                                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                    {
                                        let __p = &mut p_or_term;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                                };
                            }
                        }
                        if i < 0 {
                            { let _ = 0; };
                            { let _ = 0; };
                            { let _ = 0; };
                            break '__b17;
                        }
                        ok_to_chng_to_in = 1;
                        {
                            '__b19: loop {
                                if !(i >= 0 && ok_to_chng_to_in != 0) { break '__b19; }
                                '__c19: loop {
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    if unsafe { (*p_or_term).left_cursor } != i_cursor {
                                        unsafe { (*p_or_term).wt_flags &= !64 as u16 };
                                    } else if unsafe { (*p_or_term).u.x.left_column } !=
                                                i_column ||
                                            i_column == -2 &&
                                                unsafe {
                                                        sqlite3_expr_compare(p_parse as *const Parse,
                                                            unsafe { (*unsafe { (*p_or_term).p_expr }).p_left } as
                                                                *const Expr, p_left as *const Expr, -1)
                                                    } != 0 {
                                        ok_to_chng_to_in = 0;
                                    } else {
                                        let mut aff_left: i32 = 0;
                                        let mut aff_right: i32 = 0;
                                        aff_right =
                                            unsafe {
                                                    sqlite3_expr_affinity(unsafe {
                                                                (*unsafe { (*p_or_term).p_expr }).p_right
                                                            } as *const Expr)
                                                } as i32;
                                        aff_left =
                                            unsafe {
                                                    sqlite3_expr_affinity(unsafe {
                                                                (*unsafe { (*p_or_term).p_expr }).p_left
                                                            } as *const Expr)
                                                } as i32;
                                        if aff_right != 0 && aff_right != aff_left {
                                            ok_to_chng_to_in = 0;
                                        } else { unsafe { (*p_or_term).wt_flags |= 64 as u16 }; }
                                    }
                                    break '__c19;
                                }
                                {
                                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                    {
                                        let __p = &mut p_or_term;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                                };
                            }
                        }
                        break '__c17;
                    }
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                }
            }
            if ok_to_chng_to_in != 0 {
                let mut p_dup: *mut Expr = core::ptr::null_mut();
                let mut p_list: *mut ExprList = core::ptr::null_mut();
                let mut p_left_1: *const Expr = core::ptr::null();
                let mut p_coll_seq: *mut CollSeq = core::ptr::null_mut();
                let mut p_new: *mut Expr = core::ptr::null_mut();
                {
                    {
                        i = unsafe { (*p_or_wc).n_term } - 1;
                        p_or_term = unsafe { (*p_or_wc).a }
                    };
                    '__b20: loop {
                        if !(i >= 0) { break '__b20; }
                        '__c20: loop {
                            let mut p_this: *const Expr = core::ptr::null();
                            if unsafe { (*p_or_term).wt_flags } as i32 & 64 == 0 {
                                break '__c20;
                            }
                            { let _ = 0; };
                            { let _ = 0; };
                            { let _ = 0; };
                            { let _ = 0; };
                            p_this = unsafe { (*p_or_term).p_expr };
                            p_dup =
                                unsafe {
                                    sqlite3_expr_dup(db,
                                        unsafe { (*p_this).p_right } as *const Expr, 0)
                                };
                            p_list =
                                unsafe {
                                    sqlite3_expr_list_append(unsafe { (*p_w_info).p_parse },
                                        p_list, p_dup)
                                };
                            if p_left_1 == core::ptr::null_mut() {
                                p_left_1 = unsafe { (*p_this).p_left };
                                p_coll_seq =
                                    unsafe {
                                        sqlite3_expr_compare_coll_seq(p_parse,
                                            p_this as *const Expr)
                                    };
                            } else {
                                { let _ = 0; };
                                if p_coll_seq !=
                                        unsafe {
                                            sqlite3_expr_compare_coll_seq(p_parse,
                                                p_this as *const Expr)
                                        } {
                                    p_left_1 = core::ptr::null_mut();
                                    break '__b20;
                                }
                            }
                            break '__c20;
                        }
                        {
                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p_or_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                    }
                }
                if p_left_1 == core::ptr::null_mut() {
                    p_new = core::ptr::null_mut();
                } else {
                    p_dup =
                        unsafe { sqlite3_expr_dup(db, p_left_1 as *const Expr, 0) };
                    if unsafe {
                                    sqlite3_expr_coll_seq(p_parse, p_dup as *const Expr)
                                } != p_coll_seq && p_coll_seq != core::ptr::null_mut() {
                        { let _ = 0; };
                        p_dup =
                            unsafe {
                                sqlite3_expr_add_collate_string(p_parse as *const Parse,
                                    p_dup, unsafe { (*p_coll_seq).z_name } as *const i8)
                            };
                    }
                    p_new =
                        unsafe {
                            sqlite3_p_expr(p_parse, 50, p_dup, core::ptr::null_mut())
                        };
                }
                if !(p_new).is_null() {
                    let mut idx_new: i32 = 0;
                    transfer_join_markings(p_new, unsafe { &*p_expr });
                    { let _ = 0; };
                    unsafe { (*p_new).x.p_list = p_list };
                    idx_new =
                        where_clause_insert(p_wc_1, p_new, (2 | 1) as u16);
                    expr_analyze(p_src_1, p_wc_1, idx_new);
                    mark_term_as_child(unsafe { &*p_wc_1 }, idx_new,
                        idx_term_1);
                } else { unsafe { sqlite3_expr_list_delete(db, p_list) }; }
            }
        }
    }
}
extern "C" fn is_like_or_glob(p_parse_1: *mut Parse, p_expr_1: *mut Expr,
    pp_prefix_1: &mut *mut Expr, pis_complete_1: &mut i32,
    pno_case_1: *mut i32) -> i32 {
    unsafe {
        let mut z: *const u8 = core::ptr::null();
        let mut p_right: *mut Expr = core::ptr::null_mut();
        let mut p_left: *const Expr = core::ptr::null();
        let mut p_list: *const ExprList = core::ptr::null();
        let mut c: u8 = 0 as u8;
        let mut cnt: i32 = 0;
        let mut wc: [u8; 4] = [0; 4];
        let db: *mut sqlite3 = unsafe { (*p_parse_1).db };
        let mut p_val: *mut sqlite3_value = core::ptr::null_mut();
        let mut op: i32 = 0;
        let mut rc: i32 = 0;
        if (unsafe {
                            sqlite3_is_like_function(db, p_expr_1, pno_case_1,
                                &raw mut wc[0 as usize] as *mut i8)
                        } == 0) as i32 != 0 {
            return 0;
        }
        { let _ = 0; };
        p_list = unsafe { (*p_expr_1).x.p_list };
        p_left =
            unsafe {
                (*(unsafe { (*p_list).a.as_ptr() } as
                                *mut ExprList_item).offset(1 as isize)).p_expr
            };
        p_right =
            unsafe {
                sqlite3_expr_skip_collate(unsafe {
                        (*(unsafe { (*p_list).a.as_ptr() } as
                                        *mut ExprList_item).offset(0 as isize)).p_expr
                    })
            };
        op = unsafe { (*p_right).op } as i32;
        if op == 157 && unsafe { (*db).flags } & 8388608 as u64 == 0 as u64 {
            let p_reprepare: *mut Vdbe = unsafe { (*p_parse_1).p_reprepare };
            let i_col: i32 = unsafe { (*p_right).i_column } as i32;
            p_val =
                unsafe {
                    sqlite3_vdbe_get_bound_value(p_reprepare, i_col, 65 as u8)
                };
            if !(p_val).is_null() && unsafe { sqlite3_value_type(p_val) } == 3
                {
                z = unsafe { sqlite3_value_text(p_val) };
            }
            unsafe {
                sqlite3_vdbe_set_varmask(unsafe { (*p_parse_1).p_vdbe },
                    i_col)
            };
            { let _ = 0; };
        } else if op == 118 {
            { let _ = 0; };
            z = unsafe { (*p_right).u.z_token } as *mut u8 as *const u8;
        }
        if !(z).is_null() {
            cnt = 0;
            while { c = unsafe { *z.offset(cnt as isize) } as u8; c } as i32
                                != 0 && c as i32 != wc[0 as usize] as i32 &&
                        c as i32 != wc[1 as usize] as i32 &&
                    c as i32 != wc[2 as usize] as i32 {
                { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                if c as i32 == wc[3 as usize] as i32 &&
                            unsafe { *z.offset(cnt as isize) } as i32 > 0 &&
                        (unsafe { *z.offset(cnt as isize) } as i32) < 128 {
                    { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                } else if c as i32 >= 128 {
                    let mut z2: *const u8 =
                        unsafe {
                            unsafe { z.offset(cnt as isize).offset(-(1 as isize)) }
                        };
                    if c as i32 == 255 ||
                                unsafe { sqlite3_utf8_read(&mut z2) } == 65533 as u32 ||
                            unsafe { (*db).enc } as i32 == 2 {
                        { let __p = &mut cnt; let __t = *__p; *__p -= 1; __t };
                        break;
                    } else { cnt = unsafe { z2.offset_from(z) } as i64 as i32; }
                }
            }
            if (cnt > 1 ||
                        cnt > 0 &&
                            unsafe { *z.offset(0 as isize) } as i32 !=
                                wc[3 as usize] as i32) &&
                    255 != unsafe { *z.offset((cnt - 1) as isize) } as u8 as i32
                {
                let mut p_prefix: *mut Expr = core::ptr::null_mut();
                *pis_complete_1 =
                    (c as i32 == wc[0 as usize] as i32 &&
                                unsafe { *z.offset((cnt + 1) as isize) } as i32 == 0 &&
                            unsafe { (*db).enc } as i32 != 2) as i32;
                p_prefix =
                    unsafe { sqlite3_expr(db, 118, z as *mut i8 as *const i8) };
                if !(p_prefix).is_null() {
                    let mut i_from: i32 = 0;
                    let mut i_to: i32 = 0;
                    let mut z_new: *mut i8 = core::ptr::null_mut();
                    { let _ = 0; };
                    z_new = unsafe { (*p_prefix).u.z_token };
                    unsafe { *z_new.offset(cnt as isize) = 0 as i8 };
                    {
                        i_from = { i_to = 0; i_to };
                        '__b22: loop {
                            if !(i_from < cnt) { break '__b22; }
                            '__c22: loop {
                                if unsafe { *z_new.offset(i_from as isize) } as i32 ==
                                        wc[3 as usize] as i32 {
                                    { let __p = &mut i_from; let __t = *__p; *__p += 1; __t };
                                }
                                unsafe {
                                    *z_new.offset({
                                                        let __p = &mut i_to;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize) = unsafe { *z_new.offset(i_from as isize) }
                                };
                                break '__c22;
                            }
                            { let __p = &mut i_from; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { *z_new.offset(i_to as isize) = 0 as i8 };
                    { let _ = 0; };
                    if unsafe { (*p_left).op } as i32 != 168 ||
                                unsafe { sqlite3_expr_affinity(p_left as *const Expr) } as
                                        i32 != 66 ||
                            unsafe { (*p_left).flags } & (16777216 | 33554432) as u32 ==
                                        0 as u32 && !(unsafe { (*p_left).y.p_tab }).is_null() &&
                                unsafe { (*unsafe { (*p_left).y.p_tab }).e_tab_type } as i32
                                    == 1 {
                        let mut is_num: i32 = 0;
                        let mut r_dummy: f64 = 0.0;
                        { let _ = 0; };
                        is_num =
                            unsafe { sqlite3_ato_f(z_new as *const i8, &mut r_dummy) };
                        if is_num <= 0 {
                            if i_to == 1 &&
                                    unsafe { *z_new.offset(0 as isize) } as i32 == '-' as i32 {
                                is_num = 1;
                            } else {
                                {
                                    let __p =
                                        unsafe { &mut *z_new.offset((i_to - 1) as isize) };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                is_num =
                                    unsafe { sqlite3_ato_f(z_new as *const i8, &mut r_dummy) };
                                {
                                    let __p =
                                        unsafe { &mut *z_new.offset((i_to - 1) as isize) };
                                    let __t = *__p;
                                    *__p -= 1;
                                    __t
                                };
                            }
                        }
                        if is_num > 0 {
                            unsafe { sqlite3_expr_delete(db, p_prefix) };
                            unsafe { sqlite3ValueFree(p_val) };
                            return 0;
                        }
                    }
                }
                *pp_prefix_1 = p_prefix;
                if op == 157 {
                    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
                    unsafe {
                        sqlite3_vdbe_set_varmask(v,
                            unsafe { (*p_right).i_column } as i32)
                    };
                    { let _ = 0; };
                    if *pis_complete_1 != 0 &&
                            unsafe {
                                    *unsafe { (*p_right).u.z_token.offset(1 as isize) }
                                } != 0 {
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe { sqlite3_expr_code_target(p_parse_1, p_right, r1) };
                        unsafe {
                            sqlite3_vdbe_change_p3(v,
                                unsafe { sqlite3_vdbe_current_addr(v) } - 1, 0)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                    }
                }
            } else { z = core::ptr::null(); }
        }
        rc = (z != core::ptr::null()) as i32;
        unsafe { sqlite3ValueFree(p_val) };
        return rc;
    }
}
extern "C" fn is_auxiliary_vtab_operator(db: *mut sqlite3,
    p_expr_1: *const Expr, pe_op2_1: &mut u8, pp_left_1: &mut *mut Expr,
    pp_right_1: &mut *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 172 {
            let mut p_list: *const ExprList = core::ptr::null();
            let mut p_col: *mut Expr = core::ptr::null_mut();
            let mut i: i32 = 0;
            { let _ = 0; };
            p_list = unsafe { (*p_expr_1).x.p_list };
            if p_list == core::ptr::null_mut() ||
                    unsafe { (*p_list).n_expr } != 2 {
                return 0;
            }
            p_col =
                unsafe {
                    (*(unsafe { (*p_list).a.as_ptr() } as
                                    *mut ExprList_item).offset(1 as isize)).p_expr
                };
            { let _ = 0; };
            if unsafe { (*p_col).op } as i32 == 168 &&
                        unsafe { (*unsafe { (*p_col).y.p_tab }).e_tab_type } as i32
                            == 1 &&
                    {
                            i = sqlite3_expr_is_like_operator(unsafe { &*p_expr_1 });
                            i
                        } != 0 {
                *pe_op2_1 = i as u8;
                *pp_right_1 =
                    unsafe {
                        (*(unsafe { (*p_list).a.as_ptr() } as
                                        *mut ExprList_item).offset(0 as isize)).p_expr
                    };
                *pp_left_1 = p_col;
                return 1;
            }
            p_col =
                unsafe {
                    (*(unsafe { (*p_list).a.as_ptr() } as
                                    *mut ExprList_item).offset(0 as isize)).p_expr
                };
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*p_col).op } as i32 == 168 &&
                    unsafe { (*unsafe { (*p_col).y.p_tab }).e_tab_type } as i32
                        == 1 {
                let mut p_vtab: *mut sqlite3_vtab = core::ptr::null_mut();
                let mut p_mod: *const sqlite3_module = core::ptr::null();
                let mut x_not_used:
                        unsafe extern "C" fn(*mut sqlite3_context, i32,
                            *mut *mut sqlite3_value) -> () =
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut sqlite3_context, i32,
                                    *mut *mut sqlite3_value) -> ()>(0 as *const ())
                    };
                let mut p_not_used: *mut () = core::ptr::null_mut();
                p_vtab =
                    unsafe {
                        (*unsafe {
                                        sqlite3_get_v_table(db, unsafe { (*p_col).y.p_tab })
                                    }).p_vtab
                    };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                p_mod = unsafe { (*p_vtab).p_module } as *mut sqlite3_module;
                if unsafe { (*p_mod).x_find_function.is_some() } {
                    i =
                        unsafe {
                            (unsafe {
                                    (*p_mod).x_find_function.unwrap()
                                })(p_vtab, 2, unsafe { (*p_expr_1).u.z_token } as *const i8,
                                &mut x_not_used, &mut p_not_used)
                        };
                    if i >= 150 {
                        *pe_op2_1 = i as u8;
                        *pp_right_1 =
                            unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                *mut ExprList_item).offset(1 as isize)).p_expr
                            };
                        *pp_left_1 = p_col;
                        return 1;
                    }
                }
            }
        } else if unsafe { (*p_expr_1).op } as i32 >= 54 {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            return 0;
        } else if unsafe { (*p_expr_1).op } as i32 == 53 ||
                    unsafe { (*p_expr_1).op } as i32 == 46 ||
                unsafe { (*p_expr_1).op } as i32 == 52 {
            let mut res: i32 = 0;
            let mut p_left: *mut Expr = unsafe { (*p_expr_1).p_left };
            let mut p_right: *mut Expr = unsafe { (*p_expr_1).p_right };
            { let _ = 0; };
            if unsafe { (*p_left).op } as i32 == 168 &&
                    unsafe { (*unsafe { (*p_left).y.p_tab }).e_tab_type } as i32
                        == 1 {
                { let __p = &mut res; let __t = *__p; *__p += 1; __t };
            }
            { let _ = 0; };
            if !(p_right).is_null() &&
                    (unsafe { (*p_right).op } as i32 == 168 &&
                        unsafe { (*unsafe { (*p_right).y.p_tab }).e_tab_type } as
                                i32 == 1) {
                { let __p = &mut res; let __t = *__p; *__p += 1; __t };
                { let t: *mut Expr = p_left; p_left = p_right; p_right = t; }
            }
            *pp_left_1 = p_left;
            *pp_right_1 = p_right;
            if unsafe { (*p_expr_1).op } as i32 == 53 {
                *pe_op2_1 = 68 as u8;
            }
            if unsafe { (*p_expr_1).op } as i32 == 46 {
                *pe_op2_1 = 69 as u8;
            }
            if unsafe { (*p_expr_1).op } as i32 == 52 {
                *pe_op2_1 = 70 as u8;
            }
            return res;
        }
        return 0;
    }
}
extern "C" fn expr_analyze(p_src: *mut SrcList, p_wc: *mut WhereClause,
    idx_term: i32) -> () {
    unsafe {
        unsafe {
            let p_w_info: *mut WhereInfo = unsafe { (*p_wc).p_w_info };
            let mut p_term: *mut WhereTerm = core::ptr::null_mut();
            let mut p_mask_set: *mut WhereMaskSet = core::ptr::null_mut();
            let mut p_expr: *mut Expr = core::ptr::null_mut();
            let mut prereq_left: Bitmask = 0 as Bitmask;
            let mut prereq_all: Bitmask = 0 as Bitmask;
            let mut extra_right: Bitmask = 0 as Bitmask;
            let mut p_str1: *mut Expr = core::ptr::null_mut();
            let mut is_complete: i32 = 0;
            let mut no_case: i32 = 0;
            let mut op: i32 = 0;
            let p_parse: *mut Parse = unsafe { (*p_w_info).p_parse };
            let db: *mut sqlite3 = unsafe { (*p_parse).db };
            let mut e_op2: u8 = 0 as u8;
            let mut n_left: i32 = 0;
            if unsafe { (*db).malloc_failed } != 0 { return; }
            { let _ = 0; };
            p_term =
                unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
            p_mask_set = unsafe { &mut (*p_w_info).s_mask_set };
            p_expr = unsafe { (*p_term).p_expr };
            { let _ = 0; };
            { let _ = 0; };
            unsafe { (*p_mask_set).b_var_select = 0 };
            prereq_left =
                sqlite3_where_expr_usage(p_mask_set,
                    unsafe { (*p_expr).p_left });
            op = unsafe { (*p_expr).op } as i32;
            if op == 50 {
                { let _ = 0; };
                if unsafe { sqlite3_expr_check_in(p_parse, p_expr) } != 0 {
                    return;
                }
                if unsafe { (*p_expr).flags } & 4096 as u32 != 0 as u32 {
                    unsafe {
                        (*p_term).prereq_right =
                            expr_select_usage(p_mask_set,
                                unsafe { (*p_expr).x.p_select } as *const Select)
                    };
                } else {
                    unsafe {
                        (*p_term).prereq_right =
                            sqlite3_where_expr_list_usage(p_mask_set,
                                unsafe { (*p_expr).x.p_list })
                    };
                }
                prereq_all = prereq_left | unsafe { (*p_term).prereq_right };
            } else {
                unsafe {
                    (*p_term).prereq_right =
                        sqlite3_where_expr_usage(p_mask_set,
                            unsafe { (*p_expr).p_right })
                };
                if unsafe { (*p_expr).p_left } == core::ptr::null_mut() ||
                            unsafe { (*p_expr).flags } & (4096 | 262144) as u32 !=
                                0 as u32 ||
                        unsafe { (*p_expr).x.p_list } != core::ptr::null_mut() {
                    prereq_all =
                        sqlite3_where_expr_usage_nn(p_mask_set, p_expr);
                } else {
                    prereq_all =
                        prereq_left | unsafe { (*p_term).prereq_right };
                }
            }
            if unsafe { (*p_mask_set).b_var_select } != 0 {
                unsafe { (*p_term).wt_flags |= 4096 as u16 };
            }
            if unsafe { (*p_expr).flags } & (1 | 2) as u32 != 0 as u32 {
                let x: Bitmask =
                    unsafe {
                        sqlite3_where_get_mask(p_mask_set,
                            unsafe { (*p_expr).w.i_join })
                    };
                if unsafe { (*p_expr).flags } & 1 as u32 != 0 as u32 {
                    prereq_all |= x;
                    extra_right = x - 1 as Bitmask;
                } else if prereq_all >> 1 >= x {
                    unsafe { (*p_expr).flags &= !(2 as u32) };
                }
            }
            unsafe { (*p_term).prereq_all = prereq_all };
            unsafe { (*p_term).left_cursor = -1 };
            unsafe { (*p_term).i_parent = -1 };
            unsafe { (*p_term).e_operator = 0 as u16 };
            if allowed_op(op) != 0 {
                let mut ai_cur_col: [i32; 2] = [0; 2];
                let mut p_left: *mut Expr =
                    unsafe {
                        sqlite3_expr_skip_collate(unsafe { (*p_expr).p_left })
                    };
                let p_right: *mut Expr =
                    unsafe {
                        sqlite3_expr_skip_collate(unsafe { (*p_expr).p_right })
                    };
                let op_mask: u16 =
                    if unsafe { (*p_term).prereq_right } & prereq_left ==
                                0 as u64 {
                            16383
                        } else { 2048 } as u16;
                if unsafe { (*p_term).u.x.i_field } > 0 {
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    p_left =
                        unsafe {
                            (*(unsafe { (*unsafe { (*p_left).x.p_list }).a.as_ptr() } as
                                            *mut ExprList_item).offset((unsafe { (*p_term).u.x.i_field }
                                                - 1) as isize)).p_expr
                        };
                }
                if expr_might_be_indexed(p_src,
                            &raw mut ai_cur_col[0 as usize] as *mut i32, p_left, op) !=
                        0 {
                    unsafe { (*p_term).left_cursor = ai_cur_col[0 as usize] };
                    { let _ = 0; };
                    unsafe {
                        (*p_term).u.x.left_column = ai_cur_col[1 as usize]
                    };
                    unsafe {
                        (*p_term).e_operator =
                            (operator_mask(op) as i32 & op_mask as i32) as u16
                    };
                }
                if op == 45 { unsafe { (*p_term).wt_flags |= 2048 as u16 }; }
                if !(p_right).is_null() &&
                            expr_might_be_indexed(p_src,
                                    &raw mut ai_cur_col[0 as usize] as *mut i32, p_right, op) !=
                                0 &&
                        !(unsafe { (*p_right).flags } & 32 as u32 != 0 as u32) as
                                i32 != 0 {
                    let mut p_new: *mut WhereTerm = core::ptr::null_mut();
                    let mut p_dup: *mut Expr = core::ptr::null_mut();
                    let mut e_extra_op: u16 = 0 as u16;
                    { let _ = 0; };
                    if unsafe { (*p_term).left_cursor } >= 0 {
                        let mut idx_new: i32 = 0;
                        p_dup =
                            unsafe { sqlite3_expr_dup(db, p_expr as *const Expr, 0) };
                        if unsafe { (*db).malloc_failed } != 0 {
                            unsafe { sqlite3_expr_delete(db, p_dup) };
                            return;
                        }
                        idx_new = where_clause_insert(p_wc, p_dup, (2 | 1) as u16);
                        if idx_new == 0 { return; }
                        p_new =
                            unsafe { unsafe { (*p_wc).a.offset(idx_new as isize) } };
                        mark_term_as_child(unsafe { &*p_wc }, idx_new, idx_term);
                        if op == 45 { unsafe { (*p_new).wt_flags |= 2048 as u16 }; }
                        p_term =
                            unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
                        unsafe { (*p_term).wt_flags |= 8 as u16 };
                        { let _ = 0; };
                        if term_is_equivalence(p_parse, unsafe { &*p_dup },
                                    unsafe { &*unsafe { (*p_w_info).p_tab_list } }) != 0 {
                            unsafe { (*p_term).e_operator |= 2048 as u16 };
                            e_extra_op = 2048 as u16;
                        }
                    } else { p_dup = p_expr; p_new = p_term; }
                    unsafe {
                        (*p_new).wt_flags |=
                            expr_commute(p_parse, unsafe { &mut *p_dup }) as i32 as u16
                    };
                    unsafe { (*p_new).left_cursor = ai_cur_col[0 as usize] };
                    { let _ = 0; };
                    unsafe {
                        (*p_new).u.x.left_column = ai_cur_col[1 as usize]
                    };
                    unsafe {
                        (*p_new).prereq_right = prereq_left | extra_right
                    };
                    unsafe { (*p_new).prereq_all = prereq_all };
                    unsafe {
                        (*p_new).e_operator =
                            (operator_mask(unsafe { (*p_dup).op } as i32) as i32 +
                                        e_extra_op as i32 & op_mask as i32) as u16
                    };
                } else if op == 51 &&
                            !(unsafe { (*p_expr).flags } & 1 as u32 != 0 as u32) as i32
                                != 0 &&
                        0 ==
                            unsafe { sqlite3_expr_can_be_null(p_left as *const Expr) } {
                    { let _ = 0; };
                    unsafe { (*p_expr).op = 171 as u8 };
                    unsafe {
                        (*p_expr).u.z_token = c"false".as_ptr() as *mut i8
                    };
                    unsafe { (*p_expr).flags |= 536870912 as u32 };
                    unsafe { (*p_term).prereq_all = 0 as Bitmask };
                    unsafe { (*p_term).e_operator = 0 as u16 };
                }
            } else if unsafe { (*p_expr).op } as i32 == 49 &&
                    unsafe { (*p_wc).op } as i32 == 44 {
                let mut p_list: *const ExprList = core::ptr::null();
                let mut i: i32 = 0;
                { let _ = 0; };
                p_list = unsafe { (*p_expr).x.p_list };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                {
                    i = 0;
                    '__b23: loop {
                        if !(i < 2) { break '__b23; }
                        '__c23: loop {
                            let mut p_new_expr: *mut Expr = core::ptr::null_mut();
                            let mut idx_new_1: i32 = 0;
                            p_new_expr =
                                unsafe {
                                    sqlite3_p_expr(p_parse, ops[i as usize] as i32,
                                        unsafe {
                                            sqlite3_expr_dup(db,
                                                unsafe { (*p_expr).p_left } as *const Expr, 0)
                                        },
                                        unsafe {
                                            sqlite3_expr_dup(db,
                                                unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprList_item).offset(i as isize)).p_expr
                                                    } as *const Expr, 0)
                                        })
                                };
                            transfer_join_markings(p_new_expr, unsafe { &*p_expr });
                            idx_new_1 =
                                where_clause_insert(p_wc, p_new_expr, (2 | 1) as u16);
                            expr_analyze(p_src, p_wc, idx_new_1);
                            p_term =
                                unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
                            mark_term_as_child(unsafe { &*p_wc }, idx_new_1, idx_term);
                            break '__c23;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            } else if unsafe { (*p_expr).op } as i32 == 43 &&
                    !(unsafe { (*p_expr).flags } & 512 as u32 != 0 as u32) as
                            i32 != 0 {
                { let _ = 0; };
                expr_analyze_or_term(p_src, p_wc, idx_term);
                p_term =
                    unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
            } else if unsafe { (*p_expr).op } as i32 == 52 {
                if unsafe { (*unsafe { (*p_expr).p_left }).op } as i32 == 168
                            &&
                            unsafe { (*unsafe { (*p_expr).p_left }).i_column } as i32 >=
                                0 &&
                        !(unsafe { (*p_expr).flags } & 1 as u32 != 0 as u32) as i32
                            != 0 {
                    let mut p_new_expr_1: *mut Expr = core::ptr::null_mut();
                    let p_left_1: *const Expr =
                        unsafe { (*p_expr).p_left } as *const Expr;
                    let mut idx_new_2: i32 = 0;
                    let mut p_new_term: *mut WhereTerm = core::ptr::null_mut();
                    p_new_expr_1 =
                        unsafe {
                            sqlite3_p_expr(p_parse, 55,
                                unsafe { sqlite3_expr_dup(db, p_left_1 as *const Expr, 0) },
                                unsafe {
                                    sqlite3_expr_alloc(db, 122, core::ptr::null(), 0)
                                })
                        };
                    idx_new_2 =
                        where_clause_insert(p_wc, p_new_expr_1,
                            (2 | 1 | 128) as u16);
                    if idx_new_2 != 0 {
                        p_new_term =
                            unsafe { unsafe { (*p_wc).a.offset(idx_new_2 as isize) } };
                        unsafe { (*p_new_term).prereq_right = 0 as Bitmask };
                        unsafe {
                            (*p_new_term).left_cursor = unsafe { (*p_left_1).i_table }
                        };
                        unsafe {
                            (*p_new_term).u.x.left_column =
                                unsafe { (*p_left_1).i_column } as i32
                        };
                        unsafe { (*p_new_term).e_operator = (2 << 55 - 54) as u16 };
                        mark_term_as_child(unsafe { &*p_wc }, idx_new_2, idx_term);
                        p_term =
                            unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
                        unsafe { (*p_term).wt_flags |= 8 as u16 };
                        unsafe {
                            (*p_new_term).prereq_all = unsafe { (*p_term).prereq_all }
                        };
                    }
                }
            } else if unsafe { (*p_expr).op } as i32 == 172 &&
                        unsafe { (*p_wc).op } as i32 == 44 &&
                    is_like_or_glob(p_parse, p_expr, &mut p_str1,
                            &mut is_complete, &mut no_case) != 0 {
                let mut p_left_2: *const Expr = core::ptr::null();
                let mut p_str2: *mut Expr = core::ptr::null_mut();
                let mut p_new_expr1: *mut Expr = core::ptr::null_mut();
                let mut p_new_expr2: *mut Expr = core::ptr::null_mut();
                let mut idx_new1: i32 = 0;
                let mut idx_new2: i32 = 0;
                let mut z_coll_seq_name: *const i8 = core::ptr::null();
                let wt_flags: u16 = (256 | 2 | 1) as u16;
                { let _ = 0; };
                p_left_2 =
                    unsafe {
                        (*(unsafe { (*unsafe { (*p_expr).x.p_list }).a.as_ptr() } as
                                        *mut ExprList_item).offset(1 as isize)).p_expr
                    };
                p_str2 =
                    unsafe { sqlite3_expr_dup(db, p_str1 as *const Expr, 0) };
                { let _ = 0; };
                { let _ = 0; };
                if no_case != 0 &&
                        (unsafe { (*unsafe { (*p_parse).db }).malloc_failed } == 0)
                                as i32 != 0 {
                    let mut i: i32 = 0;
                    let mut c: i8 = 0 as i8;
                    unsafe { (*p_term).wt_flags |= 1024 as u16 };
                    {
                        i = 0;
                        '__b24: loop {
                            if !({
                                                    c =
                                                        unsafe {
                                                            *unsafe { (*p_str1).u.z_token.offset(i as isize) }
                                                        };
                                                    c
                                                } as i32 != 0) {
                                break '__b24;
                            }
                            '__c24: loop {
                                unsafe {
                                    *unsafe { (*p_str1).u.z_token.offset(i as isize) } =
                                        (c as i32 &
                                                !(unsafe {
                                                                *(sqlite3_ctype_map.as_ptr() as
                                                                            *const u8).add(c as u8 as usize)
                                                            } as i32 & 32)) as i8
                                };
                                unsafe {
                                    *unsafe { (*p_str2).u.z_token.offset(i as isize) } =
                                        unsafe {
                                                *(sqlite3_upper_to_lower.as_ptr() as
                                                            *const u8).add(c as u8 as usize)
                                            } as i8
                                };
                                break '__c24;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                    let mut p_c: *mut u8 = core::ptr::null_mut();
                    p_c =
                        unsafe {
                                &raw mut *unsafe {
                                            (*p_str2).u.z_token.offset((unsafe {
                                                            sqlite3_strlen30(unsafe { (*p_str2).u.z_token } as
                                                                    *const i8)
                                                        } - 1) as isize)
                                        }
                            } as *mut u8;
                    if no_case != 0 {
                        if unsafe { *p_c } as i32 == 'A' as i32 - 1 {
                            is_complete = 0;
                        }
                        unsafe {
                            *p_c =
                                unsafe {
                                        *(sqlite3_upper_to_lower.as_ptr() as
                                                    *const u8).add(unsafe { *p_c } as usize)
                                    } as u8
                        };
                    }
                    while unsafe { *p_c } as i32 == 191 &&
                            p_c > unsafe { (*p_str2).u.z_token } as *mut u8 {
                        unsafe { *p_c = 128 as u8 };
                        {
                            let __p = &mut p_c;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(-1) };
                            __t
                        };
                    }
                    { let _ = 0; };
                    {
                        let __p = unsafe { &mut *p_c };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
                z_coll_seq_name =
                    if no_case != 0 {
                        c"NOCASE".as_ptr() as *mut i8 as *const i8
                    } else { sqlite3_str_binary.as_ptr() as *const i8 };
                p_new_expr1 =
                    unsafe { sqlite3_expr_dup(db, p_left_2 as *const Expr, 0) };
                p_new_expr1 =
                    unsafe {
                        sqlite3_p_expr(p_parse, 58,
                            unsafe {
                                sqlite3_expr_add_collate_string(p_parse as *const Parse,
                                    p_new_expr1, z_coll_seq_name)
                            }, p_str1)
                    };
                transfer_join_markings(p_new_expr1, unsafe { &*p_expr });
                idx_new1 = where_clause_insert(p_wc, p_new_expr1, wt_flags);
                p_new_expr2 =
                    unsafe { sqlite3_expr_dup(db, p_left_2 as *const Expr, 0) };
                p_new_expr2 =
                    unsafe {
                        sqlite3_p_expr(p_parse, 57,
                            unsafe {
                                sqlite3_expr_add_collate_string(p_parse as *const Parse,
                                    p_new_expr2, z_coll_seq_name)
                            }, p_str2)
                    };
                transfer_join_markings(p_new_expr2, unsafe { &*p_expr });
                idx_new2 = where_clause_insert(p_wc, p_new_expr2, wt_flags);
                expr_analyze(p_src, p_wc, idx_new1);
                expr_analyze(p_src, p_wc, idx_new2);
                p_term =
                    unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
                if is_complete != 0 {
                    mark_term_as_child(unsafe { &*p_wc }, idx_new1, idx_term);
                    mark_term_as_child(unsafe { &*p_wc }, idx_new2, idx_term);
                }
            }
            if (unsafe { (*p_expr).op } as i32 == 54 ||
                                    unsafe { (*p_expr).op } as i32 == 45) &&
                                {
                                        n_left =
                                            unsafe {
                                                sqlite3_expr_vector_size(unsafe { (*p_expr).p_left } as
                                                        *const Expr)
                                            };
                                        n_left
                                    } > 1 &&
                            unsafe {
                                    sqlite3_expr_vector_size(unsafe { (*p_expr).p_right } as
                                            *const Expr)
                                } == n_left &&
                        (unsafe { (*unsafe { (*p_expr).p_left }).flags } &
                                    4096 as u32 == 0 as u32 ||
                            unsafe { (*unsafe { (*p_expr).p_right }).flags } &
                                    4096 as u32 == 0 as u32) &&
                    unsafe { (*p_wc).op } as i32 == 44 {
                let mut i: i32 = 0;
                {
                    i = 0;
                    '__b26: loop {
                        if !(i < n_left) { break '__b26; }
                        '__c26: loop {
                            let mut idx_new_3: i32 = 0;
                            let mut p_new_1: *mut Expr = core::ptr::null_mut();
                            let p_left_3: *mut Expr =
                                unsafe {
                                    sqlite3_expr_for_vector_field(p_parse,
                                        unsafe { (*p_expr).p_left }, i, n_left)
                                };
                            let p_right_1: *mut Expr =
                                unsafe {
                                    sqlite3_expr_for_vector_field(p_parse,
                                        unsafe { (*p_expr).p_right }, i, n_left)
                                };
                            p_new_1 =
                                unsafe {
                                    sqlite3_p_expr(p_parse, unsafe { (*p_expr).op } as i32,
                                        p_left_3, p_right_1)
                                };
                            transfer_join_markings(p_new_1, unsafe { &*p_expr });
                            idx_new_3 =
                                where_clause_insert(p_wc, p_new_1, (1 | 32768) as u16);
                            expr_analyze(p_src, p_wc, idx_new_3);
                            break '__c26;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                p_term =
                    unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
                unsafe { (*p_term).wt_flags |= (4 | 2) as u16 };
                unsafe { (*p_term).e_operator = 8192 as u16 };
            } else if unsafe { (*p_expr).op } as i32 == 50 &&
                                            unsafe { (*p_term).u.x.i_field } == 0 &&
                                        unsafe { (*unsafe { (*p_expr).p_left }).op } as i32 == 177
                                    && unsafe { (*p_expr).flags } & 4096 as u32 != 0 as u32 &&
                                (unsafe { (*unsafe { (*p_expr).x.p_select }).p_prior } ==
                                        core::ptr::null_mut() ||
                                    unsafe { (*unsafe { (*p_expr).x.p_select }).sel_flags } &
                                            512 as u32 != 0) &&
                            unsafe { (*unsafe { (*p_expr).x.p_select }).p_win } ==
                                core::ptr::null_mut() && unsafe { (*p_wc).op } as i32 == 44
                    &&
                    unsafe {
                                (*unsafe {
                                                (*unsafe { (*p_expr).x.p_select }).p_e_list
                                            }).n_expr
                            } as i64 <=
                        ((1 as i64) << core::mem::size_of::<u8>() as u64 * 8 as u64)
                            - 1 as i64 {
                let mut i: i32 = 0;
                { let _ = 0; };
                {
                    i = 0;
                    '__b27: loop {
                        if !(i <
                                        unsafe {
                                            sqlite3_expr_vector_size(unsafe { (*p_expr).p_left } as
                                                    *const Expr)
                                        }) {
                            break '__b27;
                        }
                        '__c27: loop {
                            let mut idx_new_4: i32 = 0;
                            idx_new_4 =
                                where_clause_insert(p_wc, p_expr, (2 | 32768) as u16);
                            unsafe {
                                (*unsafe {
                                                        (*p_wc).a.offset(idx_new_4 as isize)
                                                    }).u.x.i_field = i + 1
                            };
                            expr_analyze(p_src, p_wc, idx_new_4);
                            mark_term_as_child(unsafe { &*p_wc }, idx_new_4, idx_term);
                            break '__c27;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            } else if unsafe { (*p_wc).op } as i32 == 44 {
                let mut p_right_2: *mut Expr = core::ptr::null_mut();
                let mut p_left_4: *mut Expr = core::ptr::null_mut();
                let mut res: i32 =
                    is_auxiliary_vtab_operator(db, p_expr as *const Expr,
                        &mut e_op2, &mut p_left_4, &mut p_right_2);
                while { let __p = &mut res; let __t = *__p; *__p -= 1; __t } >
                        0 {
                    let mut idx_new_5: i32 = 0;
                    let mut p_new_term_1: *mut WhereTerm =
                        core::ptr::null_mut();
                    let mut prereq_column: Bitmask = 0 as Bitmask;
                    let mut prereq_expr: Bitmask = 0 as Bitmask;
                    prereq_expr =
                        sqlite3_where_expr_usage(p_mask_set, p_right_2);
                    prereq_column =
                        sqlite3_where_expr_usage(p_mask_set, p_left_4);
                    if prereq_expr & prereq_column == 0 as u64 {
                        let mut p_new_expr_2: *mut Expr = core::ptr::null_mut();
                        p_new_expr_2 =
                            unsafe {
                                sqlite3_p_expr(p_parse, 47, core::ptr::null_mut(),
                                    unsafe {
                                        sqlite3_expr_dup(db, p_right_2 as *const Expr, 0)
                                    })
                            };
                        if unsafe { (*p_expr).flags } & 1 as u32 != 0 as u32 &&
                                !(p_new_expr_2).is_null() {
                            unsafe { (*p_new_expr_2).flags |= 1 as u32 };
                            unsafe {
                                (*p_new_expr_2).w.i_join = unsafe { (*p_expr).w.i_join }
                            };
                        }
                        idx_new_5 =
                            where_clause_insert(p_wc, p_new_expr_2, (2 | 1) as u16);
                        p_new_term_1 =
                            unsafe { unsafe { (*p_wc).a.offset(idx_new_5 as isize) } };
                        unsafe {
                            (*p_new_term_1).prereq_right = prereq_expr | extra_right
                        };
                        unsafe {
                            (*p_new_term_1).left_cursor = unsafe { (*p_left_4).i_table }
                        };
                        unsafe {
                            (*p_new_term_1).u.x.left_column =
                                unsafe { (*p_left_4).i_column } as i32
                        };
                        unsafe { (*p_new_term_1).e_operator = 64 as u16 };
                        unsafe { (*p_new_term_1).e_match_op = e_op2 };
                        mark_term_as_child(unsafe { &*p_wc }, idx_new_5, idx_term);
                        p_term =
                            unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
                        unsafe { (*p_term).wt_flags |= 8 as u16 };
                        unsafe {
                            (*p_new_term_1).prereq_all = unsafe { (*p_term).prereq_all }
                        };
                    }
                    {
                        let t: *mut Expr = p_left_4;
                        p_left_4 = p_right_2;
                        p_right_2 = t;
                    }
                }
            }
            p_term =
                unsafe { unsafe { (*p_wc).a.offset(idx_term as isize) } };
            unsafe { (*p_term).prereq_right |= extra_right };
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_expr_analyze(p_tab_list: *mut SrcList,
    p_wc: *mut WhereClause) -> () {
    let mut i: i32 = 0;
    {
        i = unsafe { (*p_wc).n_term } - 1;
        '__b29: loop {
            if !(i >= 0) { break '__b29; }
            '__c29: loop { expr_analyze(p_tab_list, p_wc, i); break '__c29; }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_tab_func_args(p_parse: *mut Parse,
    p_item: &mut SrcItem, p_wc: *mut WhereClause) -> () {
    unsafe {
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut p_args: *const ExprList = core::ptr::null();
        let mut p_col_ref: *mut Expr = core::ptr::null_mut();
        let mut p_term: *mut Expr = core::ptr::null_mut();
        if (*p_item).fg.is_tab_func() as i32 == 0 { return; }
        p_tab = (*p_item).p_s_tab;
        { let _ = 0; };
        if !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0 {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"\'%s\' is not a function".as_ptr() as *mut i8 as
                        *const i8, (*p_item).z_name)
            };
            return;
        }
        p_args = (*p_item).u1.p_func_arg;
        if p_args == core::ptr::null_mut() { return; }
        {
            j = { k = 0; k };
            '__b30: loop {
                if !(j < unsafe { (*p_args).n_expr }) { break '__b30; }
                '__c30: loop {
                    let mut p_rhs: *mut Expr = core::ptr::null_mut();
                    let mut join_type: u32 = 0 as u32;
                    while k < unsafe { (*p_tab).n_col } as i32 &&
                            unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(k as isize) }).col_flags
                                        } as i32 & 2 == 0 {
                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                    }
                    if k >= unsafe { (*p_tab).n_col } as i32 {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"too many arguments on %s() - max %d".as_ptr() as *mut i8
                                    as *const i8, unsafe { (*p_tab).z_name }, j)
                        };
                        return;
                    }
                    p_col_ref =
                        unsafe {
                            sqlite3_expr_alloc(unsafe { (*p_parse).db }, 168,
                                core::ptr::null(), 0)
                        };
                    if p_col_ref == core::ptr::null_mut() { return; }
                    unsafe { (*p_col_ref).i_table = (*p_item).i_cursor };
                    unsafe {
                        (*p_col_ref).i_column =
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t } as
                                ynVar
                    };
                    { let _ = 0; };
                    unsafe { (*p_col_ref).y.p_tab = p_tab };
                    (*p_item).col_used |=
                        unsafe { sqlite3_expr_col_used(p_col_ref) };
                    p_rhs =
                        unsafe {
                            sqlite3_p_expr(p_parse, 173,
                                unsafe {
                                    sqlite3_expr_dup(unsafe { (*p_parse).db },
                                        unsafe {
                                                (*(unsafe { (*p_args).a.as_ptr() } as
                                                                *mut ExprList_item).offset(j as isize)).p_expr
                                            } as *const Expr, 0)
                                }, core::ptr::null_mut())
                        };
                    p_term =
                        unsafe { sqlite3_p_expr(p_parse, 54, p_col_ref, p_rhs) };
                    if (*p_item).fg.jointype as i32 & (8 | 16) != 0 {
                        join_type = 1 as u32;
                    } else { join_type = 2 as u32; }
                    unsafe {
                        sqlite3_set_join_expr(p_term, (*p_item).i_cursor, join_type)
                    };
                    where_clause_insert(p_wc, p_term, 1 as u16);
                    break '__c30;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sqlite3_expr_is_like_operator_s0_N32sqlite3_expr_is_like_operator_s0 {
    z_op: *const i8,
    e_op: u8,
}
static mut a_op:
    [sqlite3_expr_is_like_operator_s0_N32sqlite3_expr_is_like_operator_s0; 4]
    =
    [sqlite3_expr_is_like_operator_s0_N32sqlite3_expr_is_like_operator_s0 {
                z_op: c"match".as_ptr() as *const i8,
                e_op: 64 as u8,
            },
            sqlite3_expr_is_like_operator_s0_N32sqlite3_expr_is_like_operator_s0 {
                z_op: c"glob".as_ptr() as *const i8,
                e_op: 66 as u8,
            },
            sqlite3_expr_is_like_operator_s0_N32sqlite3_expr_is_like_operator_s0 {
                z_op: c"like".as_ptr() as *const i8,
                e_op: 65 as u8,
            },
            sqlite3_expr_is_like_operator_s0_N32sqlite3_expr_is_like_operator_s0 {
                z_op: c"regexp".as_ptr() as *const i8,
                e_op: 67 as u8,
            }];
static ops: [u8; 2] = [58 as u8, 56 as u8];
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
    fn sqlite3_where_get_mask(_: *mut WhereMaskSet, _: i32)
    -> Bitmask;
    fn sqlite3_where_find_term(p_wc_1: *mut WhereClause, i_cur_1: i32,
    i_column_1: i32, not_ready_1: Bitmask, op: u32, p_idx_1: *mut Index)
    -> *mut WhereTerm;
    fn sqlite3_where_malloc(p_w_info_1: *mut WhereInfo, n_byte_1: u64)
    -> *mut ();
    fn sqlite3_where_realloc(p_w_info_1: *mut WhereInfo, p_old_1: *mut (),
    n_byte_1: u64)
    -> *mut ();
    fn sqlite3_where_explain_one_scan(p_parse_1: *mut Parse,
    p_tab_list_1: *mut SrcList, p_level_1: *mut WhereLevel,
    wctrl_flags_1: u16)
    -> i32;
    fn sqlite3_where_explain_bloom_filter(p_parse_1: *const Parse,
    p_w_info_1: *const WhereInfo, p_level_1: *const WhereLevel)
    -> i32;
    fn sqlite3_where_add_explain_text(p_parse_1: *mut Parse, addr: i32,
    p_tab_list_1: *mut SrcList, p_level_1: *mut WhereLevel,
    wctrl_flags_1: u16)
    -> ();
    fn sqlite3_where_code_one_loop_start(p_parse_1: *mut Parse, v: *mut Vdbe,
    p_w_info_1: *mut WhereInfo, i_level_1: i32, p_level_1: *mut WhereLevel,
    not_ready_1: Bitmask)
    -> Bitmask;
    fn sqlite3_where_right_join_loop(p_w_info_1: *mut WhereInfo,
    i_level_1: i32, p_level_1: *mut WhereLevel)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
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