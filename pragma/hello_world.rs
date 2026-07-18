type __darwin_intptr_t = i64;
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
#[repr(C)]
#[derive(Copy, Clone)]
struct PragmaName {
    z_name: *const i8,
    e_prag_typ: u8,
    m_prag_flg: u8,
    i_prag_c_name: u8,
    n_prag_c_name: u8,
    i_arg: u64,
}
extern "C" fn return_single_text(v: *mut Vdbe, z_value_1: *const i8) -> () {
    if !(z_value_1).is_null() {
        unsafe { sqlite3_vdbe_load_string(v, 1, z_value_1 as *const i8) };
        unsafe { sqlite3_vdbe_add_op2(v, 86, 1, 1) };
    }
}
static mut a_pragma_name: [PragmaName; 67] =
    [PragmaName {
                z_name: c"analysis_limit".as_ptr() as *const i8,
                e_prag_typ: 1 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"application_id".as_ptr() as *const i8,
                e_prag_typ: 2 as u8,
                m_prag_flg: (4 | 16) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 8 as u64,
            },
            PragmaName {
                z_name: c"auto_vacuum".as_ptr() as *const i8,
                e_prag_typ: 3 as u8,
                m_prag_flg: (1 | 16 | 128 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"automatic_index".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 32768 as u64,
            },
            PragmaName {
                z_name: c"busy_timeout".as_ptr() as *const i8,
                e_prag_typ: 5 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 56 as u8,
                n_prag_c_name: 1 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"cache_size".as_ptr() as *const i8,
                e_prag_typ: 6 as u8,
                m_prag_flg: (1 | 16 | 128 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"cache_spill".as_ptr() as *const i8,
                e_prag_typ: 7 as u8,
                m_prag_flg: (16 | 128 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"case_sensitive_like".as_ptr() as *const i8,
                e_prag_typ: 8 as u8,
                m_prag_flg: 2 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"cell_size_check".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 2097152 as u64,
            },
            PragmaName {
                z_name: c"checkpoint_fullfsync".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 16 as u64,
            },
            PragmaName {
                z_name: c"collation_list".as_ptr() as *const i8,
                e_prag_typ: 9 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 33 as u8,
                n_prag_c_name: 2 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"compile_options".as_ptr() as *const i8,
                e_prag_typ: 10 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"count_changes".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: (1 as u64) << 32,
            },
            PragmaName {
                z_name: c"data_version".as_ptr() as *const i8,
                e_prag_typ: 2 as u8,
                m_prag_flg: (8 | 16) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 15 as u64,
            },
            PragmaName {
                z_name: c"database_list".as_ptr() as *const i8,
                e_prag_typ: 12 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 50 as u8,
                n_prag_c_name: 3 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"default_cache_size".as_ptr() as *const i8,
                e_prag_typ: 13 as u8,
                m_prag_flg: (1 | 16 | 128 | 4) as u8,
                i_prag_c_name: 55 as u8,
                n_prag_c_name: 1 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"defer_foreign_keys".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 524288 as u64,
            },
            PragmaName {
                z_name: c"empty_result_callbacks".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 256 as u64,
            },
            PragmaName {
                z_name: c"encoding".as_ptr() as *const i8,
                e_prag_typ: 14 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"foreign_key_check".as_ptr() as *const i8,
                e_prag_typ: 15 as u8,
                m_prag_flg: (1 | 16 | 32 | 64) as u8,
                i_prag_c_name: 43 as u8,
                n_prag_c_name: 4 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"foreign_key_list".as_ptr() as *const i8,
                e_prag_typ: 16 as u8,
                m_prag_flg: (1 | 32 | 64) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 8 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"foreign_keys".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 16384 as u64,
            },
            PragmaName {
                z_name: c"freelist_count".as_ptr() as *const i8,
                e_prag_typ: 2 as u8,
                m_prag_flg: (8 | 16) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"full_column_names".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 4 as u64,
            },
            PragmaName {
                z_name: c"fullfsync".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 8 as u64,
            },
            PragmaName {
                z_name: c"function_list".as_ptr() as *const i8,
                e_prag_typ: 17 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 15 as u8,
                n_prag_c_name: 6 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"hard_heap_limit".as_ptr() as *const i8,
                e_prag_typ: 18 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"ignore_check_constraints".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 512 as u64,
            },
            PragmaName {
                z_name: c"incremental_vacuum".as_ptr() as *const i8,
                e_prag_typ: 19 as u8,
                m_prag_flg: (1 | 2) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"index_info".as_ptr() as *const i8,
                e_prag_typ: 20 as u8,
                m_prag_flg: (1 | 32 | 64) as u8,
                i_prag_c_name: 27 as u8,
                n_prag_c_name: 3 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"index_list".as_ptr() as *const i8,
                e_prag_typ: 21 as u8,
                m_prag_flg: (1 | 32 | 64) as u8,
                i_prag_c_name: 33 as u8,
                n_prag_c_name: 5 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"index_xinfo".as_ptr() as *const i8,
                e_prag_typ: 20 as u8,
                m_prag_flg: (1 | 32 | 64) as u8,
                i_prag_c_name: 27 as u8,
                n_prag_c_name: 6 as u8,
                i_arg: 1 as u64,
            },
            PragmaName {
                z_name: c"integrity_check".as_ptr() as *const i8,
                e_prag_typ: 22 as u8,
                m_prag_flg: (1 | 16 | 32 | 64) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"journal_mode".as_ptr() as *const i8,
                e_prag_typ: 23 as u8,
                m_prag_flg: (1 | 16 | 128) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"journal_size_limit".as_ptr() as *const i8,
                e_prag_typ: 24 as u8,
                m_prag_flg: (16 | 128) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"legacy_alter_table".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 67108864 as u64,
            },
            PragmaName {
                z_name: c"lock_proxy_file".as_ptr() as *const i8,
                e_prag_typ: 25 as u8,
                m_prag_flg: 4 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"locking_mode".as_ptr() as *const i8,
                e_prag_typ: 26 as u8,
                m_prag_flg: (16 | 128) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"max_page_count".as_ptr() as *const i8,
                e_prag_typ: 27 as u8,
                m_prag_flg: (1 | 16 | 128) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"mmap_size".as_ptr() as *const i8,
                e_prag_typ: 28 as u8,
                m_prag_flg: 0 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"module_list".as_ptr() as *const i8,
                e_prag_typ: 29 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 9 as u8,
                n_prag_c_name: 1 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"optimize".as_ptr() as *const i8,
                e_prag_typ: 30 as u8,
                m_prag_flg: (32 | 1) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"page_count".as_ptr() as *const i8,
                e_prag_typ: 27 as u8,
                m_prag_flg: (1 | 16 | 128) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"page_size".as_ptr() as *const i8,
                e_prag_typ: 31 as u8,
                m_prag_flg: (16 | 128 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"pragma_list".as_ptr() as *const i8,
                e_prag_typ: 32 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 9 as u8,
                n_prag_c_name: 1 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"query_only".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 1048576 as u64,
            },
            PragmaName {
                z_name: c"quick_check".as_ptr() as *const i8,
                e_prag_typ: 22 as u8,
                m_prag_flg: (1 | 16 | 32 | 64) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"read_uncommitted".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: (4 as u64) << 32,
            },
            PragmaName {
                z_name: c"recursive_triggers".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 8192 as u64,
            },
            PragmaName {
                z_name: c"reverse_unordered_selects".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 4096 as u64,
            },
            PragmaName {
                z_name: c"schema_version".as_ptr() as *const i8,
                e_prag_typ: 2 as u8,
                m_prag_flg: (4 | 16) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 1 as u64,
            },
            PragmaName {
                z_name: c"secure_delete".as_ptr() as *const i8,
                e_prag_typ: 33 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"short_column_names".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 64 as u64,
            },
            PragmaName {
                z_name: c"shrink_memory".as_ptr() as *const i8,
                e_prag_typ: 34 as u8,
                m_prag_flg: 2 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"soft_heap_limit".as_ptr() as *const i8,
                e_prag_typ: 35 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"synchronous".as_ptr() as *const i8,
                e_prag_typ: 36 as u8,
                m_prag_flg: (1 | 16 | 128 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"table_info".as_ptr() as *const i8,
                e_prag_typ: 37 as u8,
                m_prag_flg: (1 | 32 | 64) as u8,
                i_prag_c_name: 8 as u8,
                n_prag_c_name: 6 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"table_list".as_ptr() as *const i8,
                e_prag_typ: 38 as u8,
                m_prag_flg: (1 | 32) as u8,
                i_prag_c_name: 21 as u8,
                n_prag_c_name: 6 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"table_xinfo".as_ptr() as *const i8,
                e_prag_typ: 37 as u8,
                m_prag_flg: (1 | 32 | 64) as u8,
                i_prag_c_name: 8 as u8,
                n_prag_c_name: 7 as u8,
                i_arg: 1 as u64,
            },
            PragmaName {
                z_name: c"temp_store".as_ptr() as *const i8,
                e_prag_typ: 39 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"temp_store_directory".as_ptr() as *const i8,
                e_prag_typ: 40 as u8,
                m_prag_flg: 4 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"threads".as_ptr() as *const i8,
                e_prag_typ: 41 as u8,
                m_prag_flg: 16 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"trusted_schema".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 128 as u64,
            },
            PragmaName {
                z_name: c"user_version".as_ptr() as *const i8,
                e_prag_typ: 2 as u8,
                m_prag_flg: (4 | 16) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 6 as u64,
            },
            PragmaName {
                z_name: c"wal_autocheckpoint".as_ptr() as *const i8,
                e_prag_typ: 42 as u8,
                m_prag_flg: 0 as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"wal_checkpoint".as_ptr() as *const i8,
                e_prag_typ: 43 as u8,
                m_prag_flg: 1 as u8,
                i_prag_c_name: 47 as u8,
                n_prag_c_name: 3 as u8,
                i_arg: 0 as u64,
            },
            PragmaName {
                z_name: c"writable_schema".as_ptr() as *const i8,
                e_prag_typ: 4 as u8,
                m_prag_flg: (16 | 4) as u8,
                i_prag_c_name: 0 as u8,
                n_prag_c_name: 0 as u8,
                i_arg: (1 | 134217728) as u64,
            }];
extern "C" fn pragma_locate(z_name_1: *const i8) -> *const PragmaName {
    unsafe {
        let mut upr: i32 = 0;
        let mut lwr: i32 = 0;
        let mut mid: i32 = 0;
        let mut rc: i32 = 0;
        lwr = 0;
        upr =
            (core::mem::size_of::<[PragmaName; 67]>() as u64 /
                        core::mem::size_of::<PragmaName>() as u64) as i32 - 1;
        while lwr <= upr {
            mid = (lwr + upr) / 2;
            rc =
                unsafe {
                    sqlite3_stricmp(z_name_1,
                        a_pragma_name[mid as usize].z_name)
                };
            if rc == 0 { break; }
            if rc < 0 { upr = mid - 1; } else { lwr = mid + 1; }
        }
        return if lwr > upr {
                core::ptr::null()
            } else { &a_pragma_name[mid as usize] };
    }
}
static mut prag_c_name: [*const i8; 57] =
    [c"id".as_ptr() as *const i8, c"seq".as_ptr() as *const i8,
            c"table".as_ptr() as *const i8, c"from".as_ptr() as *const i8,
            c"to".as_ptr() as *const i8, c"on_update".as_ptr() as *const i8,
            c"on_delete".as_ptr() as *const i8,
            c"match".as_ptr() as *const i8, c"cid".as_ptr() as *const i8,
            c"name".as_ptr() as *const i8, c"type".as_ptr() as *const i8,
            c"notnull".as_ptr() as *const i8,
            c"dflt_value".as_ptr() as *const i8, c"pk".as_ptr() as *const i8,
            c"hidden".as_ptr() as *const i8, c"name".as_ptr() as *const i8,
            c"builtin".as_ptr() as *const i8, c"type".as_ptr() as *const i8,
            c"enc".as_ptr() as *const i8, c"narg".as_ptr() as *const i8,
            c"flags".as_ptr() as *const i8, c"schema".as_ptr() as *const i8,
            c"name".as_ptr() as *const i8, c"type".as_ptr() as *const i8,
            c"ncol".as_ptr() as *const i8, c"wr".as_ptr() as *const i8,
            c"strict".as_ptr() as *const i8, c"seqno".as_ptr() as *const i8,
            c"cid".as_ptr() as *const i8, c"name".as_ptr() as *const i8,
            c"desc".as_ptr() as *const i8, c"coll".as_ptr() as *const i8,
            c"key".as_ptr() as *const i8, c"seq".as_ptr() as *const i8,
            c"name".as_ptr() as *const i8, c"unique".as_ptr() as *const i8,
            c"origin".as_ptr() as *const i8, c"partial".as_ptr() as *const i8,
            c"tbl".as_ptr() as *const i8, c"idx".as_ptr() as *const i8,
            c"wdth".as_ptr() as *const i8, c"hght".as_ptr() as *const i8,
            c"flgs".as_ptr() as *const i8, c"table".as_ptr() as *const i8,
            c"rowid".as_ptr() as *const i8, c"parent".as_ptr() as *const i8,
            c"fkid".as_ptr() as *const i8, c"busy".as_ptr() as *const i8,
            c"log".as_ptr() as *const i8,
            c"checkpointed".as_ptr() as *const i8,
            c"seq".as_ptr() as *const i8, c"name".as_ptr() as *const i8,
            c"file".as_ptr() as *const i8, c"database".as_ptr() as *const i8,
            c"status".as_ptr() as *const i8,
            c"cache_size".as_ptr() as *const i8,
            c"timeout".as_ptr() as *const i8];
extern "C" fn set_pragma_result_column_names(v: *mut Vdbe,
    p_pragma_1: &PragmaName) -> () {
    unsafe {
        let n: u8 = (*p_pragma_1).n_prag_c_name as u8;
        unsafe {
            sqlite3_vdbe_set_num_cols(v,
                if n as i32 == 0 { 1 } else { n as i32 })
        };
        if n as i32 == 0 {
            unsafe {
                sqlite3_vdbe_set_col_name(v, 0, 0, (*p_pragma_1).z_name, None)
            };
        } else {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            {
                { i = 0; j = (*p_pragma_1).i_prag_c_name as i32 };
                '__b1: loop {
                    if !(i < n as i32) { break '__b1; }
                    '__c1: loop {
                        unsafe {
                            sqlite3_vdbe_set_col_name(v, i, 0, prag_c_name[j as usize],
                                None)
                        };
                        break '__c1;
                    }
                    {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t }
                    };
                }
            }
        }
    }
}
extern "C" fn return_single_int(v: *mut Vdbe, mut value: i64) -> () {
    unsafe {
        sqlite3_vdbe_add_op4_dup8(v, 74, 0, 1, 0, &raw mut value as *const u8,
            -14)
    };
    unsafe { sqlite3_vdbe_add_op2(v, 86, 1, 1) };
}
extern "C" fn get_safety_level(z: *const i8, omit_full_1: i32, dflt: u8)
    -> u8 {
    unsafe {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        if unsafe {
                            *(sqlite3_ctype_map.as_ptr() as
                                        *const u8).add(unsafe { *z } as u8 as usize)
                        } as i32 & 4 != 0 {
            return unsafe { sqlite3_atoi(z) } as u8;
        }
        n = unsafe { sqlite3_strlen30(z) };
        {
            i = 0;
            '__b2: loop {
                if !(i <
                                (core::mem::size_of::<[u8; 8]>() as u64 /
                                        core::mem::size_of::<u8>() as u64) as i32) {
                    break '__b2;
                }
                '__c2: loop {
                    if i_length[i as usize] as i32 == n &&
                                unsafe {
                                        sqlite3_strnicmp(&z_text_1[i_offset[i as usize] as usize],
                                            z, n)
                                    } == 0 &&
                            ((omit_full_1 == 0) as i32 != 0 ||
                                i_value[i as usize] as i32 <= 1) {
                        return i_value[i as usize] as u8;
                    }
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return dflt;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_boolean(z: *const i8, dflt: u8) -> u8 {
    return (get_safety_level(z, 1, dflt) as i32 != 0) as u8;
}
extern "C" fn get_locking_mode(z: *const i8) -> i32 {
    if !(z).is_null() {
        if 0 ==
                unsafe {
                    sqlite3_str_i_cmp(z,
                        c"exclusive".as_ptr() as *mut i8 as *const i8)
                } {
            return 1;
        }
        if 0 ==
                unsafe {
                    sqlite3_str_i_cmp(z,
                        c"normal".as_ptr() as *mut i8 as *const i8)
                } {
            return 0;
        }
    }
    return -1;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_journal_modename(e_mode: i32) -> *const i8 {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if e_mode ==
                (core::mem::size_of::<[*mut i8; 6]>() as u64 /
                        core::mem::size_of::<*mut i8>() as u64) as i32 {
            return core::ptr::null();
        }
        return az_mode_name[e_mode as usize] as *const i8;
    }
}
extern "C" fn get_auto_vacuum(z: *const i8) -> i32 {
    let mut i: i32 = 0;
    if 0 ==
            unsafe {
                sqlite3_str_i_cmp(z, c"none".as_ptr() as *mut i8 as *const i8)
            } {
        return 0;
    }
    if 0 ==
            unsafe {
                sqlite3_str_i_cmp(z, c"full".as_ptr() as *mut i8 as *const i8)
            } {
        return 1;
    }
    if 0 ==
            unsafe {
                sqlite3_str_i_cmp(z,
                    c"incremental".as_ptr() as *mut i8 as *const i8)
            } {
        return 2;
    }
    i = unsafe { sqlite3_atoi(z) };
    return if i >= 0 && i <= 2 { i } else { 0 } as u8 as i32;
}
extern "C" fn set_all_pager_flags(db: &sqlite3) -> () {
    if (*db).auto_commit != 0 {
        let mut p_db: *const Db = (*db).a_db as *const Db;
        let mut n: i32 = (*db).n_db;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        while { let __p = &mut n; let __t = *__p; *__p -= 1; __t } > 0 {
            if !(unsafe { (*p_db).p_bt }).is_null() {
                unsafe {
                    sqlite3_btree_set_pager_flags(unsafe { (*p_db).p_bt },
                        (unsafe { (*p_db).safety_level } as u64 |
                                (*db).flags & 56 as u64) as u32)
                };
            }
            {
                let __p = &mut p_db;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
    }
}
extern "C" fn get_temp_store(z: *const i8) -> i32 {
    if unsafe { *z.offset(0 as isize) } as i32 >= '0' as i32 &&
            unsafe { *z.offset(0 as isize) } as i32 <= '2' as i32 {
        return unsafe { *z.offset(0 as isize) } as i32 - '0' as i32;
    } else if unsafe {
                sqlite3_str_i_cmp(z, c"file".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return 1;
    } else if unsafe {
                sqlite3_str_i_cmp(z,
                    c"memory".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return 2;
    } else { return 0; }
}
extern "C" fn invalidate_temp_storage(p_parse_1: *mut Parse) -> i32 {
    let db: *mut sqlite3 = unsafe { (*p_parse_1).db };
    if unsafe { (*unsafe { (*db).a_db.offset(1 as isize) }).p_bt } !=
            core::ptr::null_mut() {
        if (unsafe { (*db).auto_commit } == 0) as i32 != 0 ||
                unsafe {
                        sqlite3_btree_txn_state(unsafe {
                                (*unsafe { (*db).a_db.offset(1 as isize) }).p_bt
                            })
                    } != 0 {
            unsafe {
                sqlite3_error_msg(p_parse_1,
                    c"temporary storage cannot be changed from within a transaction".as_ptr()
                            as *mut i8 as *const i8)
            };
            return 1;
        }
        unsafe {
            sqlite3_btree_close(unsafe {
                    (*unsafe { (*db).a_db.offset(1 as isize) }).p_bt
                })
        };
        unsafe {
            (*unsafe { (*db).a_db.offset(1 as isize) }).p_bt =
                core::ptr::null_mut()
        };
        unsafe { sqlite3_reset_all_schemas_of_connection(db) };
    }
    return 0;
}
extern "C" fn change_temp_storage(p_parse_1: *mut Parse,
    z_storage_type_1: *const i8) -> i32 {
    let ts: i32 = get_temp_store(z_storage_type_1);
    let db: *mut sqlite3 = unsafe { (*p_parse_1).db };
    if unsafe { (*db).temp_store } as i32 == ts { return 0; }
    if invalidate_temp_storage(p_parse_1) != 0 { return 1; }
    unsafe { (*db).temp_store = ts as u8 };
    return 0;
}
extern "C" fn pragma_funclist_line(v: *mut Vdbe, mut p: *const FuncDef,
    is_builtin_1: i32, show_intern_funcs_1: i32) -> () {
    unsafe {
        let mut mask: u32 =
            (2048 | 524288 | 1048576 | 2097152 | 262144) as u32;
        if show_intern_funcs_1 != 0 { mask = 4294967295u32; }
        {
            '__b4: loop {
                if !(!(p).is_null()) { break '__b4; }
                '__c4: loop {
                    let mut z_type: *const i8 = core::ptr::null();
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    if !unsafe { (*p).x_s_func.is_some() } as i32 != 0 {
                        break '__c4;
                    }
                    if unsafe { (*p).func_flags } & 262144 as u32 != 0 as u32 &&
                            show_intern_funcs_1 == 0 {
                        break '__c4;
                    }
                    if unsafe { (*p).x_value.is_some() } {
                        z_type = c"w".as_ptr() as *mut i8 as *const i8;
                    } else if unsafe { (*p).x_finalize.is_some() } {
                        z_type = c"a".as_ptr() as *mut i8 as *const i8;
                    } else { z_type = c"s".as_ptr() as *mut i8 as *const i8; }
                    unsafe {
                        sqlite3_vdbe_multi_load(v, 1,
                            c"sissii".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p).z_name }, is_builtin_1, z_type,
                            az_enc[(unsafe { (*p).func_flags } & 3 as u32) as usize],
                            unsafe { (*p).n_arg } as i32,
                            unsafe { (*p).func_flags } & mask ^ 2097152 as u32)
                    };
                    break '__c4;
                }
                p = unsafe { (*p).p_next };
            }
        }
    }
}
extern "C" fn action_name(action: u8) -> *const i8 {
    let mut z_name: *const i8 = core::ptr::null();
    '__s5:
        {
        match action {
            8 => { z_name = c"SET NULL".as_ptr() as *mut i8 as *const i8; }
            9 => { z_name = c"SET DEFAULT".as_ptr() as *mut i8 as *const i8; }
            10 => { z_name = c"CASCADE".as_ptr() as *mut i8 as *const i8; }
            7 => { z_name = c"RESTRICT".as_ptr() as *mut i8 as *const i8; }
            _ => {
                z_name = c"NO ACTION".as_ptr() as *mut i8 as *const i8;
                { let _ = 0; };
            }
        }
    }
    return z_name;
}
extern "C" fn table_skip_integrity_check(p_tab_1: *const Table,
    p_obj_tab_1: *const Table) -> i32 {
    if !(p_obj_tab_1).is_null() {
        return (p_tab_1 != p_obj_tab_1) as i32;
    } else {
        return (unsafe { (*p_tab_1).tab_flags } & 131072 as u32 != 0 as u32)
                as i32;
    }
}
extern "C" fn integrity_check_result_row(v: *mut Vdbe) -> i32 {
    let mut addr: i32 = 0;
    unsafe { sqlite3_vdbe_add_op2(v, 86, 3, 1) };
    addr =
        unsafe {
            sqlite3_vdbe_add_op3(v, 61, 1,
                unsafe { sqlite3_vdbe_current_addr(v) } + 2, 1)
        };
    unsafe { sqlite3_vdbe_add_op0(v, 72) };
    return addr;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pragma(p_parse: *mut Parse, p_id1: *mut Token,
    p_id2: *mut Token, p_value: *mut Token, minus_flag: i32) -> () {
    unsafe {
        unsafe {
            let mut z_left: *mut i8 = core::ptr::null_mut();
            let mut z_right: *mut i8 = core::ptr::null_mut();
            let mut z_db: *const i8 = core::ptr::null();
            let mut p_id: *mut Token = core::ptr::null_mut();
            let mut a_fcntl: [*mut i8; 4] = [core::ptr::null_mut(); 4];
            let mut i_db: i32 = 0;
            let mut rc: i32 = 0;
            let mut db: *mut sqlite3 = core::ptr::null_mut();
            let mut p_db: *mut Db = core::ptr::null_mut();
            let mut v: *mut Vdbe = core::ptr::null_mut();
            let mut p_pragma: *const PragmaName = core::ptr::null();
            let mut a_op: *mut VdbeOp = core::ptr::null_mut();
            let mut size: i32 = 0;
            let mut p_bt: *mut Btree = core::ptr::null_mut();
            let mut size__1: i32 = 0;
            let mut p_bt_1: *mut Btree = core::ptr::null_mut();
            let mut b: i32 = 0;
            let mut ii: i32 = 0;
            let mut i_reg: i32 = 0;
            let mut x: i64 = 0 as i64;
            let mut z_ret: *const i8 = core::ptr::null();
            let mut e_mode: i32 = 0;
            let mut p_pager: *mut Pager = core::ptr::null_mut();
            let mut ii__1: i32 = 0;
            let mut e_mode_1: i32 = 0;
            let mut ii__2: i32 = 0;
            let mut z_mode: *const i8 = core::ptr::null();
            let mut n: i32 = 0;
            let mut p_pager_1: *mut Pager = core::ptr::null_mut();
            let mut i_limit: i64 = 0 as i64;
            let mut p_bt_2: *mut Btree = core::ptr::null_mut();
            let mut e_auto: i32 = 0;
            let mut a_op_1: *mut VdbeOp = core::ptr::null_mut();
            let mut i_addr: i32 = 0;
            let mut i_limit_1: i32 = 0;
            let mut addr: i32 = 0;
            let mut size__2: i32 = 0;
            let mut size__3: i32 = 0;
            let mut sz: sqlite3_int64 = 0 as sqlite3_int64;
            let mut ii__3: i32 = 0;
            let mut res: i32 = 0;
            let mut p_pager_2: *mut Pager = core::ptr::null_mut();
            let mut proxy_file_path: *const i8 = core::ptr::null();
            let mut p_file: *mut sqlite3_file = core::ptr::null_mut();
            let mut p_pager_3: *mut Pager = core::ptr::null_mut();
            let mut p_file_1: *mut sqlite3_file = core::ptr::null_mut();
            let mut res__1: i32 = 0;
            let mut i_level: i32 = 0;
            let mut mask: u64 = 0 as u64;
            let mut p_tab: *mut Table = core::ptr::null_mut();
            let mut i: i32 = 0;
            let mut k: i32 = 0;
            let mut n_hidden: i32 = 0;
            let mut p_col: *mut Column = core::ptr::null_mut();
            let mut p_pk: *const Index = core::ptr::null();
            let mut is_hidden: i32 = 0;
            let mut p_col_expr: *const Expr = core::ptr::null();
            let mut ii__4: i32 = 0;
            let mut k__1: *const HashElem = core::ptr::null();
            let mut p_hash: *const Hash = core::ptr::null();
            let mut init_n_col: i32 = 0;
            let mut p_tab_1: *const Table = core::ptr::null();
            let mut z_sql: *mut i8 = core::ptr::null_mut();
            let mut p_dummy: *mut sqlite3_stmt = core::ptr::null_mut();
            let mut p_tab_2: *const Table = core::ptr::null();
            let mut z_type: *const i8 = core::ptr::null();
            let mut p_idx: *const Index = core::ptr::null();
            let mut p_tab_3: *mut Table = core::ptr::null_mut();
            let mut i_idx_db: i32 = 0;
            let mut i__1: i32 = 0;
            let mut mx: i32 = 0;
            let mut cnum: i16 = 0 as i16;
            let mut p_idx_1: *const Index = core::ptr::null();
            let mut p_tab_4: *const Table = core::ptr::null();
            let mut i__2: i32 = 0;
            let mut i_tab_db: i32 = 0;
            let az_origin: [*const i8; 3] =
                [c"c".as_ptr() as *const i8, c"u".as_ptr() as *const i8,
                        c"pk".as_ptr() as *const i8];
            let mut i__3: i32 = 0;
            let mut i__4: i32 = 0;
            let mut p: *mut HashElem = core::ptr::null_mut();
            let mut p_coll: *const CollSeq = core::ptr::null();
            let mut i__5: i32 = 0;
            let mut j: *mut HashElem = core::ptr::null_mut();
            let mut p__1: *mut FuncDef = core::ptr::null_mut();
            let mut show_intern_func: i32 = 0;
            let mut j__1: *mut HashElem = core::ptr::null_mut();
            let mut p_mod: *const Module = core::ptr::null();
            let mut i__6: i32 = 0;
            let mut p_fk: *const FKey = core::ptr::null();
            let mut p_tab_5: *const Table = core::ptr::null();
            let mut i_tab_db_1: i32 = 0;
            let mut i__7: i32 = 0;
            let mut j__2: i32 = 0;
            let mut p_fk_1: *mut FKey = core::ptr::null_mut();
            let mut p_tab_6: *mut Table = core::ptr::null_mut();
            let mut p_parent: *mut Table = core::ptr::null_mut();
            let mut p_idx_2: *mut Index = core::ptr::null_mut();
            let mut i__8: i32 = 0;
            let mut j__3: i32 = 0;
            let mut k__2: *mut HashElem = core::ptr::null_mut();
            let mut x__1: i32 = 0;
            let mut reg_result: i32 = 0;
            let mut reg_row: i32 = 0;
            let mut addr_top: i32 = 0;
            let mut addr_ok: i32 = 0;
            let mut ai_cols: *mut i32 = core::ptr::null_mut();
            let mut i_col: i32 = 0;
            let mut jmp: i32 = 0;
            let mut i__9: i32 = 0;
            let mut j__4: i32 = 0;
            let mut addr__1: i32 = 0;
            let mut mx_err: i32 = 0;
            let mut p_obj_tab: *const Table = core::ptr::null();
            let mut is_quick: i32 = 0;
            let mut x__2: *const HashElem = core::ptr::null();
            let mut p_tbls: *const Hash = core::ptr::null();
            let mut a_root: *mut i32 = core::ptr::null_mut();
            let mut cnt: i32 = 0;
            let mut p_tab_7: *const Table = core::ptr::null();
            let mut p_idx_3: *const Index = core::ptr::null();
            let mut n_idx: i32 = 0;
            let mut p_tab_8: *const Table = core::ptr::null();
            let mut p_idx_4: *const Index = core::ptr::null();
            let mut i_tab: i32 = 0;
            let mut p_tab_9: *const Table = core::ptr::null();
            let mut p_idx_5: *const Index = core::ptr::null();
            let mut p_tab_10: *mut Table = core::ptr::null_mut();
            let mut p_idx_6: *mut Index = core::ptr::null_mut();
            let mut p_pk_1: *mut Index = core::ptr::null_mut();
            let mut p_prior: *mut Index = core::ptr::null_mut();
            let mut loop_top: i32 = 0;
            let mut i_data_cur: i32 = 0;
            let mut i_idx_cur: i32 = 0;
            let mut r1: i32 = 0;
            let mut b_strict: i32 = 0;
            let mut r2: i32 = 0;
            let mut mx_col: i32 = 0;
            let mut a1: i32 = 0;
            let mut z_err: *const i8 = core::ptr::null();
            let mut z_err_1: *const i8 = core::ptr::null();
            let mut p_col_1: *mut Column = core::ptr::null_mut();
            let mut label_error: i32 = 0;
            let mut label_ok: i32 = 0;
            let mut p1: i32 = 0;
            let mut p3: i32 = 0;
            let mut p4: i32 = 0;
            let mut do_type_check: i32 = 0;
            let mut p_dflt_value: *mut sqlite3_value = core::ptr::null_mut();
            let mut jmp3: i32 = 0;
            let mut jmp2: i32 = 0;
            let mut p_check: *mut ExprList = core::ptr::null_mut();
            let mut addr_ck_fault: i32 = 0;
            let mut addr_ck_ok: i32 = 0;
            let mut z_err_2: *const i8 = core::ptr::null();
            let mut k__3: i32 = 0;
            let mut jmp2__1: i32 = 0;
            let mut jmp3__1: i32 = 0;
            let mut jmp4: i32 = 0;
            let mut jmp5: i32 = 0;
            let mut label6: i32 = 0;
            let mut kk: i32 = 0;
            let mut ck_uniq: i32 = 0;
            let mut jmp7: i32 = 0;
            let mut jmp6: i32 = 0;
            let mut uniq_ok: i32 = 0;
            let mut jmp6__1: i32 = 0;
            let mut i_col_1: i32 = 0;
            let mut p_tab_11: *mut Table = core::ptr::null_mut();
            let mut p_v_tab: *const sqlite3_vtab = core::ptr::null();
            let mut a1__1: i32 = 0;
            let mut z_mod: *const i8 = core::ptr::null();
            let mut a_op_2: *mut VdbeOp = core::ptr::null_mut();
            let mut p_enc: *const EncName_N7EncName = core::ptr::null();
            let mut enc: u8 = 0 as u8;
            let mut i_cookie: i32 = 0;
            let mut a_op_3: *mut VdbeOp = core::ptr::null_mut();
            let mut a_op_4: *mut VdbeOp = core::ptr::null_mut();
            let mut i__10: i32 = 0;
            let mut z_opt: *const i8 = core::ptr::null();
            let mut i_bt: i32 = 0;
            let mut e_mode_2: i32 = 0;
            let mut i_db_last: i32 = 0;
            let mut i_tab_cur: i32 = 0;
            let mut k__4: *mut HashElem = core::ptr::null_mut();
            let mut p_schema: *mut Schema = core::ptr::null_mut();
            let mut p_tab_12: *mut Table = core::ptr::null_mut();
            let mut p_idx_7: *const Index = core::ptr::null();
            let mut sz_threshold: LogEst = 0 as LogEst;
            let mut z_sub_sql: *const i8 = core::ptr::null();
            let mut op_mask: u32 = 0 as u32;
            let mut n_limit: i32 = 0;
            let mut n_check: i32 = 0;
            let mut n_btree: i32 = 0;
            let mut n_index: i32 = 0;
            let mut i_range: LogEst = 0 as LogEst;
            let mut r1__1: i32 = 0;
            let mut i_addr_1: i32 = 0;
            let mut i_end: i32 = 0;
            let mut a_op_5: *mut VdbeOp = core::ptr::null_mut();
            let mut n__1: sqlite3_int64 = 0 as sqlite3_int64;
            let mut n_1: sqlite3_int64 = 0 as sqlite3_int64;
            let mut i_prior: sqlite3_int64 = 0 as sqlite3_int64;
            let mut n_2: sqlite3_int64 = 0 as sqlite3_int64;
            let mut n_3: sqlite3_int64 = 0 as sqlite3_int64;
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s7:
                    {
                    match __state {
                        0 => { z_left = core::ptr::null_mut(); __state = 3; }
                        2 => {
                            unsafe { sqlite3_db_free(db, z_left as *mut ()) };
                            __state = 1275;
                        }
                        3 => { z_right = core::ptr::null_mut(); __state = 4; }
                        4 => { z_db = core::ptr::null(); __state = 5; }
                        5 => { __state = 6; }
                        6 => { __state = 7; }
                        7 => { __state = 8; }
                        8 => { __state = 9; }
                        9 => { db = unsafe { (*p_parse).db }; __state = 10; }
                        10 => { __state = 11; }
                        11 => {
                            v = unsafe { sqlite3_get_vdbe(p_parse) };
                            __state = 12;
                        }
                        12 => { __state = 13; }
                        13 => {
                            if v == core::ptr::null_mut() {
                                __state = 15;
                            } else { __state = 14; }
                        }
                        14 => {
                            unsafe { sqlite3_vdbe_run_only_once(v) };
                            __state = 16;
                        }
                        15 => { return; }
                        16 => { unsafe { (*p_parse).n_mem = 2 }; __state = 17; }
                        17 => {
                            i_db =
                                unsafe {
                                    sqlite3_two_part_name(p_parse, p_id1, p_id2, &mut p_id)
                                };
                            __state = 18;
                        }
                        18 => {
                            if i_db < 0 { __state = 20; } else { __state = 19; }
                        }
                        19 => {
                            p_db =
                                unsafe { unsafe { (*db).a_db.offset(i_db as isize) } };
                            __state = 21;
                        }
                        20 => { return; }
                        21 => {
                            if i_db == 1 &&
                                    unsafe { sqlite3_open_temp_database(p_parse) } != 0 {
                                __state = 23;
                            } else { __state = 22; }
                        }
                        22 => {
                            z_left =
                                unsafe {
                                    sqlite3_name_from_token(db, p_id as *const Token)
                                };
                            __state = 24;
                        }
                        23 => { return; }
                        24 => {
                            if (z_left).is_null() as i32 != 0 {
                                __state = 26;
                            } else { __state = 25; }
                        }
                        25 => {
                            if minus_flag != 0 { __state = 28; } else { __state = 29; }
                        }
                        26 => { return; }
                        27 => { { let _ = 0; }; __state = 30; }
                        28 => {
                            z_right =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"-%T".as_ptr() as *mut i8 as *const i8, p_value)
                                };
                            __state = 27;
                        }
                        29 => {
                            z_right =
                                unsafe {
                                    sqlite3_name_from_token(db, p_value as *const Token)
                                };
                            __state = 27;
                        }
                        30 => {
                            z_db =
                                if unsafe { (*p_id2).n } > 0 as u32 {
                                        unsafe { (*p_db).z_db_s_name }
                                    } else { core::ptr::null_mut() } as *const i8;
                            __state = 31;
                        }
                        31 => {
                            if unsafe {
                                        sqlite3_auth_check(p_parse, 19, z_left as *const i8,
                                            z_right as *const i8, z_db)
                                    } != 0 {
                                __state = 33;
                            } else { __state = 32; }
                        }
                        32 => {
                            a_fcntl[0 as usize] = core::ptr::null_mut();
                            __state = 34;
                        }
                        33 => { __state = 2; }
                        34 => { a_fcntl[1 as usize] = z_left; __state = 35; }
                        35 => { a_fcntl[2 as usize] = z_right; __state = 36; }
                        36 => {
                            a_fcntl[3 as usize] = core::ptr::null_mut();
                            __state = 37;
                        }
                        37 => {
                            unsafe { (*db).busy_handler.n_busy = 0 };
                            __state = 38;
                        }
                        38 => {
                            rc =
                                unsafe {
                                    sqlite3_file_control(db, z_db, 14,
                                        &raw mut a_fcntl[0 as usize] as *mut ())
                                };
                            __state = 39;
                        }
                        39 => {
                            if rc == 0 { __state = 41; } else { __state = 40; }
                        }
                        40 => {
                            if rc != 12 { __state = 47; } else { __state = 46; }
                        }
                        41 => {
                            unsafe { sqlite3_vdbe_set_num_cols(v, 1) };
                            __state = 42;
                        }
                        42 => {
                            unsafe {
                                sqlite3_vdbe_set_col_name(v, 0, 0,
                                    a_fcntl[0 as usize] as *const i8,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }))
                            };
                            __state = 43;
                        }
                        43 => {
                            return_single_text(v, a_fcntl[0 as usize] as *const i8);
                            __state = 44;
                        }
                        44 => {
                            unsafe { sqlite3_free(a_fcntl[0 as usize] as *mut ()) };
                            __state = 45;
                        }
                        45 => { __state = 2; }
                        46 => {
                            p_pragma = pragma_locate(z_left as *const i8);
                            __state = 53;
                        }
                        47 => {
                            if !(a_fcntl[0 as usize]).is_null() {
                                __state = 49;
                            } else { __state = 48; }
                        }
                        48 => {
                            {
                                let __p = unsafe { &mut (*p_parse).n_err };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            __state = 51;
                        }
                        49 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"%s".as_ptr() as *mut i8 as *const i8, a_fcntl[0 as usize])
                            };
                            __state = 50;
                        }
                        50 => {
                            unsafe { sqlite3_free(a_fcntl[0 as usize] as *mut ()) };
                            __state = 48;
                        }
                        51 => { unsafe { (*p_parse).rc = rc }; __state = 52; }
                        52 => { __state = 2; }
                        53 => {
                            if p_pragma == core::ptr::null() {
                                __state = 55;
                            } else { __state = 54; }
                        }
                        54 => {
                            if unsafe { (*p_pragma).m_prag_flg } as i32 & 1 != 0 {
                                __state = 57;
                            } else { __state = 56; }
                        }
                        55 => { __state = 2; }
                        56 => {
                            if unsafe { (*p_pragma).m_prag_flg } as i32 & 2 == 0 &&
                                    (unsafe { (*p_pragma).m_prag_flg } as i32 & 4 == 0 ||
                                        z_right == core::ptr::null_mut()) {
                                __state = 60;
                            } else { __state = 59; }
                        }
                        57 => {
                            if unsafe { sqlite3_read_schema(p_parse) } != 0 {
                                __state = 58;
                            } else { __state = 56; }
                        }
                        58 => { __state = 2; }
                        59 => {
                            '__s8:
                                {
                                match unsafe { (*p_pragma).e_prag_typ } {
                                    13 => { __state = 62; }
                                    31 => { __state = 63; }
                                    33 => { __state = 64; }
                                    27 => { __state = 65; }
                                    26 => { __state = 66; }
                                    23 => { __state = 67; }
                                    24 => { __state = 68; }
                                    3 => { __state = 69; }
                                    19 => { __state = 70; }
                                    6 => { __state = 71; }
                                    7 => { __state = 72; }
                                    28 => { __state = 73; }
                                    39 => { __state = 74; }
                                    40 => { __state = 75; }
                                    25 => { __state = 76; }
                                    36 => { __state = 77; }
                                    4 => { __state = 78; }
                                    37 => { __state = 79; }
                                    38 => { __state = 80; }
                                    20 => { __state = 81; }
                                    21 => { __state = 82; }
                                    12 => { __state = 83; }
                                    9 => { __state = 84; }
                                    17 => { __state = 85; }
                                    29 => { __state = 86; }
                                    32 => { __state = 87; }
                                    16 => { __state = 88; }
                                    15 => { __state = 89; }
                                    8 => { __state = 90; }
                                    22 => { __state = 91; }
                                    14 => { __state = 92; }
                                    2 => { __state = 93; }
                                    10 => { __state = 94; }
                                    43 => { __state = 95; }
                                    42 => { __state = 96; }
                                    34 => { __state = 97; }
                                    30 => { __state = 98; }
                                    35 => { __state = 100; }
                                    18 => { __state = 101; }
                                    41 => { __state = 102; }
                                    1 => { __state = 103; }
                                    _ => { __state = 99; }
                                }
                            }
                        }
                        60 => {
                            set_pragma_result_column_names(v, unsafe { &*p_pragma });
                            __state = 59;
                        }
                        61 => {
                            if unsafe { (*p_pragma).m_prag_flg } as i32 & 4 != 0 &&
                                    !(z_right).is_null() {
                                __state = 1274;
                            } else { __state = 1273; }
                        }
                        62 => { __state = 106; }
                        63 => { p_bt = unsafe { (*p_db).p_bt }; __state = 126; }
                        64 => { p_bt_1 = unsafe { (*p_db).p_bt }; __state = 135; }
                        65 => { __state = 151; }
                        66 => {
                            z_ret = c"normal".as_ptr() as *mut i8 as *const i8;
                            __state = 166;
                        }
                        67 => { __state = 187; }
                        68 => {
                            p_pager_1 =
                                unsafe { sqlite3_btree_pager(unsafe { (*p_db).p_bt }) };
                            __state = 213;
                        }
                        69 => { p_bt_2 = unsafe { (*p_db).p_bt }; __state = 222; }
                        70 => { i_limit_1 = 0; __state = 246; }
                        71 => { { let _ = 0; }; __state = 259; }
                        72 => { { let _ = 0; }; __state = 266; }
                        73 => { __state = 277; }
                        74 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 300;
                            } else { __state = 301; }
                        }
                        75 => {
                            unsafe {
                                sqlite3_mutex_enter(unsafe { sqlite3MutexAlloc(11) })
                            };
                            __state = 303;
                        }
                        76 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 322;
                            } else { __state = 323; }
                        }
                        77 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 338;
                            } else { __state = 339; }
                        }
                        78 => {
                            if z_right == core::ptr::null_mut() {
                                __state = 350;
                            } else { __state = 351; }
                        }
                        79 => {
                            if !(z_right).is_null() {
                                __state = 367;
                            } else { __state = 366; }
                        }
                        80 => { __state = 406; }
                        81 => {
                            if !(z_right).is_null() {
                                __state = 456;
                            } else { __state = 455; }
                        }
                        82 => {
                            if !(z_right).is_null() {
                                __state = 485;
                            } else { __state = 484; }
                        }
                        83 => { __state = 500; }
                        84 => { i__4 = 0; __state = 510; }
                        85 => { __state = 519; }
                        86 => { __state = 539; }
                        87 => { __state = 547; }
                        88 => {
                            if !(z_right).is_null() {
                                __state = 553;
                            } else { __state = 552; }
                        }
                        89 => { __state = 573; }
                        90 => {
                            if !(z_right).is_null() {
                                __state = 669;
                            } else { __state = 668; }
                        }
                        91 => { __state = 672; }
                        92 => { __state = 1085; }
                        93 => {
                            i_cookie = unsafe { (*p_pragma).i_arg } as i32;
                            __state = 1106;
                        }
                        94 => { i__10 = 0; __state = 1133; }
                        95 => {
                            i_bt =
                                if !(unsafe { (*p_id2).z }).is_null() {
                                    i_db
                                } else { 10 + 2 };
                            __state = 1141;
                        }
                        96 => {
                            if !(z_right).is_null() {
                                __state = 1157;
                            } else { __state = 1156; }
                        }
                        97 => {
                            unsafe { sqlite3_db_release_memory(db) };
                            __state = 1160;
                        }
                        98 => { __state = 1162; }
                        99 => { { let _ = 0; }; __state = 1248; }
                        100 => { __state = 1253; }
                        101 => { __state = 1258; }
                        102 => { __state = 1265; }
                        103 => { __state = 1269; }
                        104 => { __state = 62; }
                        105 => { __state = 63; }
                        106 => { __state = 107; }
                        107 => { __state = 108; }
                        108 => {
                            unsafe { sqlite3_vdbe_uses_btree(v, i_db) };
                            __state = 109;
                        }
                        109 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 111;
                            } else { __state = 112; }
                        }
                        110 => { __state = 61; }
                        111 => { unsafe { (*p_parse).n_mem += 2 }; __state = 113; }
                        112 => {
                            size =
                                unsafe {
                                    sqlite3_abs_int32(unsafe {
                                            sqlite3_atoi(z_right as *const i8)
                                        })
                                };
                            __state = 120;
                        }
                        113 => { __state = 114; }
                        114 => {
                            a_op =
                                unsafe {
                                    sqlite3_vdbe_add_op_list(v,
                                        (core::mem::size_of::<[VdbeOpList; 9]>() as u64 /
                                                core::mem::size_of::<VdbeOpList>() as u64) as i32,
                                        &raw const get_cache_size[0 as usize] as *const VdbeOpList,
                                        i_ln)
                                };
                            __state = 115;
                        }
                        115 => {
                            if 0 != 0 { __state = 117; } else { __state = 116; }
                        }
                        116 => {
                            unsafe { (*a_op.offset(0 as isize)).p1 = i_db };
                            __state = 118;
                        }
                        117 => { __state = 61; }
                        118 => {
                            unsafe { (*a_op.offset(1 as isize)).p1 = i_db };
                            __state = 119;
                        }
                        119 => {
                            unsafe { (*a_op.offset(6 as isize)).p1 = -2000 };
                            __state = 110;
                        }
                        120 => {
                            unsafe { sqlite3_begin_write_operation(p_parse, 0, i_db) };
                            __state = 121;
                        }
                        121 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 102, i_db, 3, size) };
                            __state = 122;
                        }
                        122 => { { let _ = 0; }; __state = 123; }
                        123 => {
                            unsafe { (*unsafe { (*p_db).p_schema }).cache_size = size };
                            __state = 124;
                        }
                        124 => {
                            unsafe {
                                sqlite3_btree_set_cache_size(unsafe { (*p_db).p_bt },
                                    unsafe { (*unsafe { (*p_db).p_schema }).cache_size })
                            };
                            __state = 110;
                        }
                        125 => { __state = 64; }
                        126 => { { let _ = 0; }; __state = 127; }
                        127 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 129;
                            } else { __state = 130; }
                        }
                        128 => { __state = 61; }
                        129 => {
                            size__1 =
                                if !(p_bt).is_null() {
                                    unsafe { sqlite3_btree_get_page_size(p_bt) }
                                } else { 0 };
                            __state = 131;
                        }
                        130 => {
                            unsafe {
                                (*db).next_pagesize =
                                    unsafe { sqlite3_atoi(z_right as *const i8) }
                            };
                            __state = 132;
                        }
                        131 => {
                            return_single_int(v, size__1 as i64);
                            __state = 128;
                        }
                        132 => {
                            if 7 ==
                                    unsafe {
                                        sqlite3_btree_set_page_size(p_bt,
                                            unsafe { (*db).next_pagesize }, 0, 0)
                                    } {
                                __state = 133;
                            } else { __state = 128; }
                        }
                        133 => { unsafe { sqlite3_oom_fault(db) }; __state = 128; }
                        134 => { __state = 65; }
                        135 => { b = -1; __state = 136; }
                        136 => { { let _ = 0; }; __state = 137; }
                        137 => {
                            if !(z_right).is_null() {
                                __state = 139;
                            } else { __state = 138; }
                        }
                        138 => {
                            if unsafe { (*p_id2).n } == 0 as u32 && b >= 0 {
                                __state = 143;
                            } else { __state = 142; }
                        }
                        139 => {
                            if unsafe {
                                        sqlite3_stricmp(z_right as *const i8,
                                            c"fast".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                __state = 140;
                            } else { __state = 141; }
                        }
                        140 => { b = 2; __state = 138; }
                        141 => {
                            b =
                                sqlite3_get_boolean(z_right as *const i8, 0 as u8) as i32;
                            __state = 138;
                        }
                        142 => {
                            b = unsafe { sqlite3_btree_secure_delete(p_bt_1, b) };
                            __state = 148;
                        }
                        143 => { __state = 144; }
                        144 => { ii = 0; __state = 145; }
                        145 => {
                            if ii < unsafe { (*db).n_db } {
                                __state = 146;
                            } else { __state = 142; }
                        }
                        146 => {
                            unsafe {
                                sqlite3_btree_secure_delete(unsafe {
                                        (*unsafe { (*db).a_db.offset(ii as isize) }).p_bt
                                    }, b)
                            };
                            __state = 147;
                        }
                        147 => {
                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            __state = 145;
                        }
                        148 => { return_single_int(v, b as i64); __state = 149; }
                        149 => { __state = 61; }
                        150 => { __state = 66; }
                        151 => { x = 0 as i64; __state = 152; }
                        152 => {
                            unsafe { sqlite3_code_verify_schema(p_parse, i_db) };
                            __state = 153;
                        }
                        153 => {
                            i_reg =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                };
                            __state = 154;
                        }
                        154 => {
                            if unsafe {
                                            *(sqlite3_upper_to_lower.as_ptr() as
                                                        *const u8).add(unsafe { *z_left.offset(0 as isize) } as u8
                                                        as usize)
                                        } as i32 == 'p' as i32 {
                                __state = 156;
                            } else { __state = 157; }
                        }
                        155 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 86, i_reg, 1) };
                            __state = 164;
                        }
                        156 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 180, i_db, i_reg) };
                            __state = 155;
                        }
                        157 => {
                            if !(z_right).is_null() &&
                                    unsafe {
                                            sqlite3_dec_or_hex_to_i64(z_right as *const i8, &mut x)
                                        } == 0 {
                                __state = 159;
                            } else { __state = 160; }
                        }
                        158 => {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 181, i_db, i_reg, x as i32)
                            };
                            __state = 155;
                        }
                        159 => {
                            if x < 0 as i64 { __state = 161; } else { __state = 162; }
                        }
                        160 => { x = 0 as i64; __state = 158; }
                        161 => { x = 0 as i64; __state = 158; }
                        162 => {
                            if x > 4294967294u32 as i64 {
                                __state = 163;
                            } else { __state = 158; }
                        }
                        163 => { x = 4294967294u32 as i64; __state = 158; }
                        164 => { __state = 61; }
                        165 => { __state = 67; }
                        166 => {
                            e_mode = get_locking_mode(z_right as *const i8);
                            __state = 167;
                        }
                        167 => {
                            if unsafe { (*p_id2).n } == 0 as u32 && e_mode == -1 {
                                __state = 169;
                            } else { __state = 170; }
                        }
                        168 => { { let _ = 0; }; __state = 182; }
                        169 => {
                            e_mode = unsafe { (*db).dflt_lock_mode } as i32;
                            __state = 168;
                        }
                        170 => { __state = 171; }
                        171 => {
                            if unsafe { (*p_id2).n } == 0 as u32 {
                                __state = 173;
                            } else { __state = 172; }
                        }
                        172 => {
                            p_pager =
                                unsafe { sqlite3_btree_pager(unsafe { (*p_db).p_bt }) };
                            __state = 181;
                        }
                        173 => { __state = 174; }
                        174 => { { let _ = 0; }; __state = 175; }
                        175 => { ii__1 = 2; __state = 177; }
                        176 => {
                            unsafe { (*db).dflt_lock_mode = e_mode as u8 };
                            __state = 172;
                        }
                        177 => {
                            if ii__1 < unsafe { (*db).n_db } {
                                __state = 178;
                            } else { __state = 176; }
                        }
                        178 => {
                            p_pager =
                                unsafe {
                                    sqlite3_btree_pager(unsafe {
                                            (*unsafe { (*db).a_db.offset(ii__1 as isize) }).p_bt
                                        })
                                };
                            __state = 180;
                        }
                        179 => {
                            { let __p = &mut ii__1; let __t = *__p; *__p += 1; __t };
                            __state = 177;
                        }
                        180 => {
                            unsafe { sqlite3_pager_locking_mode(p_pager, e_mode) };
                            __state = 179;
                        }
                        181 => {
                            e_mode =
                                unsafe { sqlite3_pager_locking_mode(p_pager, e_mode) };
                            __state = 168;
                        }
                        182 => {
                            if e_mode == 1 { __state = 184; } else { __state = 183; }
                        }
                        183 => { return_single_text(v, z_ret); __state = 185; }
                        184 => {
                            z_ret = c"exclusive".as_ptr() as *mut i8 as *const i8;
                            __state = 183;
                        }
                        185 => { __state = 61; }
                        186 => { __state = 68; }
                        187 => { __state = 188; }
                        188 => {
                            if z_right == core::ptr::null_mut() {
                                __state = 190;
                            } else { __state = 191; }
                        }
                        189 => {
                            if e_mode_1 == -1 && unsafe { (*p_id2).n } == 0 as u32 {
                                __state = 203;
                            } else { __state = 202; }
                        }
                        190 => { e_mode_1 = -1; __state = 189; }
                        191 => { __state = 192; }
                        192 => {
                            n = unsafe { sqlite3_strlen30(z_right as *const i8) };
                            __state = 193;
                        }
                        193 => { e_mode_1 = 0; __state = 195; }
                        194 => {
                            if (z_mode).is_null() as i32 != 0 {
                                __state = 200;
                            } else { __state = 199; }
                        }
                        195 => {
                            if { z_mode = sqlite3_journal_modename(e_mode_1); z_mode }
                                    != core::ptr::null() {
                                __state = 196;
                            } else { __state = 194; }
                        }
                        196 => {
                            if unsafe {
                                        sqlite3_strnicmp(z_right as *const i8, z_mode, n)
                                    } == 0 {
                                __state = 198;
                            } else { __state = 197; }
                        }
                        197 => {
                            { let __p = &mut e_mode_1; let __t = *__p; *__p += 1; __t };
                            __state = 195;
                        }
                        198 => { __state = 194; }
                        199 => {
                            if e_mode_1 == 2 &&
                                    unsafe { (*db).flags } & 268435456 as u64 != 0 as u64 {
                                __state = 201;
                            } else { __state = 189; }
                        }
                        200 => { e_mode_1 = -1; __state = 199; }
                        201 => { e_mode_1 = -1; __state = 189; }
                        202 => { ii__2 = unsafe { (*db).n_db } - 1; __state = 206; }
                        203 => { i_db = 0; __state = 204; }
                        204 => { unsafe { (*p_id2).n = 1 as u32 }; __state = 202; }
                        205 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 86, 1, 1) };
                            __state = 211;
                        }
                        206 => {
                            if ii__2 >= 0 { __state = 207; } else { __state = 205; }
                        }
                        207 => {
                            if !(unsafe {
                                                    (*unsafe { (*db).a_db.offset(ii__2 as isize) }).p_bt
                                                }).is_null() &&
                                    (ii__2 == i_db || unsafe { (*p_id2).n } == 0 as u32) {
                                __state = 209;
                            } else { __state = 208; }
                        }
                        208 => {
                            { let __p = &mut ii__2; let __t = *__p; *__p -= 1; __t };
                            __state = 206;
                        }
                        209 => {
                            unsafe { sqlite3_vdbe_uses_btree(v, ii__2) };
                            __state = 210;
                        }
                        210 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 4, ii__2, 1, e_mode_1) };
                            __state = 208;
                        }
                        211 => { __state = 61; }
                        212 => { __state = 69; }
                        213 => { i_limit = -2 as i64; __state = 214; }
                        214 => {
                            if !(z_right).is_null() {
                                __state = 216;
                            } else { __state = 215; }
                        }
                        215 => {
                            i_limit =
                                unsafe {
                                    sqlite3_pager_journal_size_limit(p_pager_1, i_limit)
                                };
                            __state = 219;
                        }
                        216 => {
                            unsafe {
                                sqlite3_dec_or_hex_to_i64(z_right as *const i8,
                                    &mut i_limit)
                            };
                            __state = 217;
                        }
                        217 => {
                            if i_limit < -1 as i64 {
                                __state = 218;
                            } else { __state = 215; }
                        }
                        218 => { i_limit = -1 as i64; __state = 215; }
                        219 => { return_single_int(v, i_limit); __state = 220; }
                        220 => { __state = 61; }
                        221 => { __state = 70; }
                        222 => { { let _ = 0; }; __state = 223; }
                        223 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 225;
                            } else { __state = 226; }
                        }
                        224 => { __state = 61; }
                        225 => {
                            return_single_int(v,
                                unsafe { sqlite3_btree_get_auto_vacuum(p_bt_2) } as i64);
                            __state = 224;
                        }
                        226 => {
                            e_auto = get_auto_vacuum(z_right as *const i8);
                            __state = 227;
                        }
                        227 => { { let _ = 0; }; __state = 228; }
                        228 => {
                            unsafe { (*db).next_autovac = e_auto as u8 as i8 };
                            __state = 229;
                        }
                        229 => {
                            rc =
                                unsafe { sqlite3_btree_set_auto_vacuum(p_bt_2, e_auto) };
                            __state = 230;
                        }
                        230 => {
                            if rc == 0 && (e_auto == 1 || e_auto == 2) {
                                __state = 231;
                            } else { __state = 224; }
                        }
                        231 => { __state = 232; }
                        232 => { __state = 233; }
                        233 => { __state = 234; }
                        234 => {
                            i_addr = unsafe { sqlite3_vdbe_current_addr(v) };
                            __state = 235;
                        }
                        235 => { __state = 236; }
                        236 => {
                            a_op_1 =
                                unsafe {
                                    sqlite3_vdbe_add_op_list(v,
                                        (core::mem::size_of::<[VdbeOpList; 5]>() as u64 /
                                                core::mem::size_of::<VdbeOpList>() as u64) as i32,
                                        &raw const set_meta6[0 as usize] as *const VdbeOpList,
                                        i_ln_1)
                                };
                            __state = 237;
                        }
                        237 => {
                            if 0 != 0 { __state = 239; } else { __state = 238; }
                        }
                        238 => {
                            unsafe { (*a_op_1.offset(0 as isize)).p1 = i_db };
                            __state = 240;
                        }
                        239 => { __state = 61; }
                        240 => {
                            unsafe { (*a_op_1.offset(1 as isize)).p1 = i_db };
                            __state = 241;
                        }
                        241 => {
                            unsafe { (*a_op_1.offset(2 as isize)).p2 = i_addr + 4 };
                            __state = 242;
                        }
                        242 => {
                            unsafe { (*a_op_1.offset(4 as isize)).p1 = i_db };
                            __state = 243;
                        }
                        243 => {
                            unsafe { (*a_op_1.offset(4 as isize)).p3 = e_auto - 1 };
                            __state = 244;
                        }
                        244 => {
                            unsafe { sqlite3_vdbe_uses_btree(v, i_db) };
                            __state = 224;
                        }
                        245 => { __state = 71; }
                        246 => {
                            if z_right == core::ptr::null_mut() ||
                                        (unsafe {
                                                        sqlite3_get_int32(z_right as *const i8, &mut i_limit_1)
                                                    } == 0) as i32 != 0 || i_limit_1 <= 0 {
                                __state = 248;
                            } else { __state = 247; }
                        }
                        247 => {
                            unsafe { sqlite3_begin_write_operation(p_parse, 0, i_db) };
                            __state = 249;
                        }
                        248 => { i_limit_1 = 2147483647; __state = 247; }
                        249 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 73, i_limit_1, 1) };
                            __state = 250;
                        }
                        250 => {
                            addr = unsafe { sqlite3_vdbe_add_op1(v, 64, i_db) };
                            __state = 251;
                        }
                        251 => { __state = 252; }
                        252 => {
                            unsafe { sqlite3_vdbe_add_op1(v, 86, 1) };
                            __state = 253;
                        }
                        253 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 88, 1, -1) };
                            __state = 254;
                        }
                        254 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 61, 1, addr) };
                            __state = 255;
                        }
                        255 => { __state = 256; }
                        256 => {
                            unsafe { sqlite3_vdbe_jump_here(v, addr) };
                            __state = 257;
                        }
                        257 => { __state = 61; }
                        258 => { __state = 72; }
                        259 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 261;
                            } else { __state = 262; }
                        }
                        260 => { __state = 61; }
                        261 => {
                            return_single_int(v,
                                unsafe { (*unsafe { (*p_db).p_schema }).cache_size } as
                                    i64);
                            __state = 260;
                        }
                        262 => {
                            size__2 = unsafe { sqlite3_atoi(z_right as *const i8) };
                            __state = 263;
                        }
                        263 => {
                            unsafe {
                                (*unsafe { (*p_db).p_schema }).cache_size = size__2
                            };
                            __state = 264;
                        }
                        264 => {
                            unsafe {
                                sqlite3_btree_set_cache_size(unsafe { (*p_db).p_bt },
                                    unsafe { (*unsafe { (*p_db).p_schema }).cache_size })
                            };
                            __state = 260;
                        }
                        265 => { __state = 73; }
                        266 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 268;
                            } else { __state = 269; }
                        }
                        267 => { __state = 61; }
                        268 => {
                            return_single_int(v,
                                if unsafe { (*db).flags } & 32 as u64 == 0 as u64 {
                                        0
                                    } else {
                                        unsafe {
                                            sqlite3_btree_set_spill_size(unsafe { (*p_db).p_bt }, 0)
                                        }
                                    } as i64);
                            __state = 267;
                        }
                        269 => { size__3 = 1; __state = 270; }
                        270 => {
                            if unsafe {
                                        sqlite3_get_int32(z_right as *const i8, &mut size__3)
                                    } != 0 {
                                __state = 272;
                            } else { __state = 271; }
                        }
                        271 => {
                            if sqlite3_get_boolean(z_right as *const i8,
                                        (size__3 != 0) as u8) != 0 {
                                __state = 274;
                            } else { __state = 275; }
                        }
                        272 => {
                            unsafe {
                                sqlite3_btree_set_spill_size(unsafe { (*p_db).p_bt },
                                    size__3)
                            };
                            __state = 271;
                        }
                        273 => {
                            set_all_pager_flags(unsafe { &*db });
                            __state = 267;
                        }
                        274 => {
                            unsafe { (*db).flags |= 32 as u64 };
                            __state = 273;
                        }
                        275 => {
                            unsafe { (*db).flags &= !(32 as u64) };
                            __state = 273;
                        }
                        276 => { __state = 74; }
                        277 => { { let _ = 0; }; __state = 278; }
                        278 => {
                            if !(z_right).is_null() {
                                __state = 280;
                            } else { __state = 279; }
                        }
                        279 => { sz = -1 as sqlite3_int64; __state = 291; }
                        280 => { __state = 281; }
                        281 => {
                            unsafe {
                                sqlite3_dec_or_hex_to_i64(z_right as *const i8, &mut sz)
                            };
                            __state = 282;
                        }
                        282 => {
                            if sz < 0 as i64 { __state = 284; } else { __state = 283; }
                        }
                        283 => {
                            if unsafe { (*p_id2).n } == 0 as u32 {
                                __state = 286;
                            } else { __state = 285; }
                        }
                        284 => { sz = sqlite3Config.sz_mmap; __state = 283; }
                        285 => { ii__3 = unsafe { (*db).n_db } - 1; __state = 287; }
                        286 => { unsafe { (*db).sz_mmap = sz }; __state = 285; }
                        287 => {
                            if ii__3 >= 0 { __state = 288; } else { __state = 279; }
                        }
                        288 => {
                            if !(unsafe {
                                                    (*unsafe { (*db).a_db.offset(ii__3 as isize) }).p_bt
                                                }).is_null() &&
                                    (ii__3 == i_db || unsafe { (*p_id2).n } == 0 as u32) {
                                __state = 290;
                            } else { __state = 289; }
                        }
                        289 => {
                            { let __p = &mut ii__3; let __t = *__p; *__p -= 1; __t };
                            __state = 287;
                        }
                        290 => {
                            unsafe {
                                sqlite3_btree_set_mmap_limit(unsafe {
                                        (*unsafe { (*db).a_db.offset(ii__3 as isize) }).p_bt
                                    }, sz)
                            };
                            __state = 289;
                        }
                        291 => {
                            rc =
                                unsafe {
                                    sqlite3_file_control(db, z_db, 18, &raw mut sz as *mut ())
                                };
                            __state = 292;
                        }
                        292 => {
                            if rc == 0 { __state = 294; } else { __state = 295; }
                        }
                        293 => { __state = 61; }
                        294 => { return_single_int(v, sz); __state = 293; }
                        295 => {
                            if rc != 12 { __state = 296; } else { __state = 293; }
                        }
                        296 => {
                            {
                                let __p = unsafe { &mut (*p_parse).n_err };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            __state = 297;
                        }
                        297 => { unsafe { (*p_parse).rc = rc }; __state = 293; }
                        298 => { __state = 75; }
                        299 => { __state = 61; }
                        300 => {
                            return_single_int(v, unsafe { (*db).temp_store } as i64);
                            __state = 299;
                        }
                        301 => {
                            change_temp_storage(p_parse, z_right as *const i8);
                            __state = 299;
                        }
                        302 => { __state = 76; }
                        303 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 305;
                            } else { __state = 306; }
                        }
                        304 => {
                            unsafe {
                                sqlite3_mutex_leave(unsafe { sqlite3MutexAlloc(11) })
                            };
                            __state = 319;
                        }
                        305 => {
                            return_single_text(v, sqlite3_temp_directory as *const i8);
                            __state = 304;
                        }
                        306 => {
                            if unsafe { *z_right.offset(0 as isize) } != 0 {
                                __state = 308;
                            } else { __state = 307; }
                        }
                        307 => {
                            if 1 == 0 ||
                                        1 == 1 && unsafe { (*db).temp_store } as i32 <= 1 ||
                                    1 == 2 && unsafe { (*db).temp_store } as i32 == 1 {
                                __state = 315;
                            } else { __state = 314; }
                        }
                        308 => { __state = 309; }
                        309 => {
                            rc =
                                unsafe {
                                    sqlite3_os_access(unsafe { (*db).p_vfs },
                                        z_right as *const i8, 1, &mut res)
                                };
                            __state = 310;
                        }
                        310 => {
                            if rc != 0 || res == 0 {
                                __state = 311;
                            } else { __state = 307; }
                        }
                        311 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"not a writable directory".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                            __state = 312;
                        }
                        312 => {
                            unsafe {
                                sqlite3_mutex_leave(unsafe { sqlite3MutexAlloc(11) })
                            };
                            __state = 313;
                        }
                        313 => { __state = 2; }
                        314 => {
                            unsafe { sqlite3_free(sqlite3_temp_directory as *mut ()) };
                            __state = 316;
                        }
                        315 => { invalidate_temp_storage(p_parse); __state = 314; }
                        316 => {
                            if unsafe { *z_right.offset(0 as isize) } != 0 {
                                __state = 317;
                            } else { __state = 318; }
                        }
                        317 => {
                            sqlite3_temp_directory =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        z_right)
                                };
                            __state = 304;
                        }
                        318 => {
                            sqlite3_temp_directory = core::ptr::null_mut();
                            __state = 304;
                        }
                        319 => { __state = 61; }
                        320 => { __state = 77; }
                        321 => { __state = 61; }
                        322 => {
                            p_pager_2 =
                                unsafe { sqlite3_btree_pager(unsafe { (*p_db).p_bt }) };
                            __state = 324;
                        }
                        323 => {
                            p_pager_3 =
                                unsafe { sqlite3_btree_pager(unsafe { (*p_db).p_bt }) };
                            __state = 328;
                        }
                        324 => {
                            proxy_file_path = 0 as *mut () as *const i8;
                            __state = 325;
                        }
                        325 => {
                            p_file = unsafe { sqlite3_pager_file(p_pager_2) };
                            __state = 326;
                        }
                        326 => {
                            unsafe {
                                sqlite3_os_file_control_hint(p_file, 2,
                                    &raw mut proxy_file_path as *mut ())
                            };
                            __state = 327;
                        }
                        327 => {
                            return_single_text(v, proxy_file_path as *const i8);
                            __state = 321;
                        }
                        328 => {
                            p_file_1 = unsafe { sqlite3_pager_file(p_pager_3) };
                            __state = 329;
                        }
                        329 => { __state = 330; }
                        330 => {
                            if unsafe { *z_right.offset(0 as isize) } != 0 {
                                __state = 332;
                            } else { __state = 333; }
                        }
                        331 => {
                            if res__1 != 0 { __state = 334; } else { __state = 321; }
                        }
                        332 => {
                            res__1 =
                                unsafe {
                                    sqlite3_os_file_control(p_file_1, 3, z_right as *mut ())
                                };
                            __state = 331;
                        }
                        333 => {
                            res__1 =
                                unsafe {
                                    sqlite3_os_file_control(p_file_1, 3, 0 as *mut ())
                                };
                            __state = 331;
                        }
                        334 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"failed to set lock proxy file".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                            __state = 335;
                        }
                        335 => { __state = 2; }
                        336 => { __state = 78; }
                        337 => { __state = 61; }
                        338 => {
                            return_single_int(v,
                                (unsafe { (*p_db).safety_level } as i32 - 1) as i64);
                            __state = 337;
                        }
                        339 => {
                            if (unsafe { (*db).auto_commit } == 0) as i32 != 0 {
                                __state = 340;
                            } else { __state = 341; }
                        }
                        340 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"Safety level may not be changed inside a transaction".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                            __state = 337;
                        }
                        341 => {
                            if i_db != 1 { __state = 342; } else { __state = 337; }
                        }
                        342 => {
                            i_level =
                                get_safety_level(z_right as *const i8, 0, 1 as u8) as i32 +
                                        1 & 7;
                            __state = 343;
                        }
                        343 => {
                            if i_level == 0 { __state = 345; } else { __state = 344; }
                        }
                        344 => {
                            unsafe { (*p_db).safety_level = i_level as u8 };
                            __state = 346;
                        }
                        345 => { i_level = 1; __state = 344; }
                        346 => {
                            unsafe { (*p_db).b_sync_set = 1 as u8 };
                            __state = 347;
                        }
                        347 => {
                            set_all_pager_flags(unsafe { &*db });
                            __state = 337;
                        }
                        348 => { __state = 79; }
                        349 => { __state = 61; }
                        350 => {
                            set_pragma_result_column_names(v, unsafe { &*p_pragma });
                            __state = 352;
                        }
                        351 => {
                            mask = unsafe { (*p_pragma).i_arg } as u64;
                            __state = 353;
                        }
                        352 => {
                            return_single_int(v,
                                (unsafe { (*db).flags } &
                                            unsafe { (*p_pragma).i_arg } as u64 != 0 as u64) as i64);
                            __state = 349;
                        }
                        353 => {
                            if unsafe { (*db).auto_commit } as i32 == 0 {
                                __state = 355;
                            } else { __state = 354; }
                        }
                        354 => {
                            if sqlite3_get_boolean(z_right as *const i8, 0 as u8) != 0 {
                                __state = 357;
                            } else { __state = 358; }
                        }
                        355 => { mask &= !16384 as u64; __state = 354; }
                        356 => {
                            unsafe { sqlite3_vdbe_add_op0(v, 168) };
                            __state = 365;
                        }
                        357 => {
                            if mask & 1 as u64 == 0 as u64 ||
                                    unsafe { (*db).flags } & 268435456 as u64 == 0 as u64 {
                                __state = 359;
                            } else { __state = 356; }
                        }
                        358 => { unsafe { (*db).flags &= !mask }; __state = 360; }
                        359 => { unsafe { (*db).flags |= mask }; __state = 356; }
                        360 => {
                            if mask == 524288 as u64 {
                                __state = 362;
                            } else { __state = 361; }
                        }
                        361 => {
                            if mask & 1 as u64 != 0 as u64 &&
                                    unsafe {
                                            sqlite3_stricmp(z_right as *const i8,
                                                c"reset".as_ptr() as *mut i8 as *const i8)
                                        } == 0 {
                                __state = 364;
                            } else { __state = 356; }
                        }
                        362 => {
                            unsafe { (*db).n_deferred_imm_cons = 0 as i64 };
                            __state = 363;
                        }
                        363 => {
                            unsafe { (*db).n_deferred_cons = 0 as i64 };
                            __state = 361;
                        }
                        364 => {
                            unsafe { sqlite3_reset_all_schemas_of_connection(db) };
                            __state = 356;
                        }
                        365 => {
                            set_all_pager_flags(unsafe { &*db });
                            __state = 349;
                        }
                        366 => { __state = 61; }
                        367 => { __state = 368; }
                        368 => {
                            unsafe { sqlite3_code_verify_named_schema(p_parse, z_db) };
                            __state = 369;
                        }
                        369 => {
                            p_tab =
                                unsafe {
                                    sqlite3_locate_table(p_parse, 2 as u32,
                                        z_right as *const i8, z_db)
                                };
                            __state = 370;
                        }
                        370 => {
                            if !(p_tab).is_null() {
                                __state = 371;
                            } else { __state = 366; }
                        }
                        371 => { __state = 372; }
                        372 => { n_hidden = 0; __state = 373; }
                        373 => { __state = 374; }
                        374 => {
                            p_pk =
                                unsafe { sqlite3_primary_key_index(p_tab) } as *const Index;
                            __state = 375;
                        }
                        375 => { unsafe { (*p_parse).n_mem = 7 }; __state = 376; }
                        376 => {
                            unsafe { sqlite3_view_get_column_names(p_parse, p_tab) };
                            __state = 377;
                        }
                        377 => {
                            { i = 0; p_col = unsafe { (*p_tab).a_col } };
                            __state = 378;
                        }
                        378 => {
                            if i < unsafe { (*p_tab).n_col } as i32 {
                                __state = 379;
                            } else { __state = 366; }
                        }
                        379 => { is_hidden = 0; __state = 381; }
                        380 => {
                            {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                {
                                    let __p = &mut p_col;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                            };
                            __state = 378;
                        }
                        381 => { __state = 382; }
                        382 => {
                            if unsafe { (*p_col).col_flags } as i32 & 98 != 0 {
                                __state = 384;
                            } else { __state = 383; }
                        }
                        383 => {
                            if unsafe { (*p_col).col_flags } as i32 & 1 == 0 {
                                __state = 394;
                            } else { __state = 395; }
                        }
                        384 => {
                            if unsafe { (*p_pragma).i_arg } as u64 == 0 as u64 {
                                __state = 386;
                            } else { __state = 385; }
                        }
                        385 => {
                            if unsafe { (*p_col).col_flags } as i32 & 32 != 0 {
                                __state = 388;
                            } else { __state = 389; }
                        }
                        386 => {
                            { let __p = &mut n_hidden; let __t = *__p; *__p += 1; __t };
                            __state = 387;
                        }
                        387 => { __state = 380; }
                        388 => { is_hidden = 2; __state = 383; }
                        389 => {
                            if unsafe { (*p_col).col_flags } as i32 & 64 != 0 {
                                __state = 390;
                            } else { __state = 391; }
                        }
                        390 => { is_hidden = 3; __state = 383; }
                        391 => { { let _ = 0; }; __state = 392; }
                        392 => { is_hidden = 1; __state = 383; }
                        393 => {
                            p_col_expr =
                                unsafe { sqlite3_column_expr(p_tab, p_col) } as *const Expr;
                            __state = 401;
                        }
                        394 => { k = 0; __state = 393; }
                        395 => {
                            if p_pk == core::ptr::null_mut() {
                                __state = 396;
                            } else { __state = 397; }
                        }
                        396 => { k = 1; __state = 393; }
                        397 => { k = 1; __state = 398; }
                        398 => {
                            if k <= unsafe { (*p_tab).n_col } as i32 &&
                                    unsafe {
                                                *unsafe { (*p_pk).ai_column.offset((k - 1) as isize) }
                                            } as i32 != i {
                                __state = 399;
                            } else { __state = 393; }
                        }
                        399 => { __state = 400; }
                        400 => {
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                            __state = 398;
                        }
                        401 => { { let _ = 0; }; __state = 402; }
                        402 => { { let _ = 0; }; __state = 403; }
                        403 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    if unsafe { (*p_pragma).i_arg } != 0 {
                                            c"issisii".as_ptr() as *mut i8
                                        } else { c"issisi".as_ptr() as *mut i8 } as *const i8,
                                    i - n_hidden, unsafe { (*p_col).z_cn_name },
                                    unsafe {
                                        sqlite3ColumnType(p_col, c"".as_ptr() as *mut i8)
                                    }, if unsafe { (*p_col).not_null() } != 0 { 1 } else { 0 },
                                    if is_hidden >= 2 || p_col_expr == core::ptr::null() {
                                        core::ptr::null_mut()
                                    } else { unsafe { (*p_col_expr).u.z_token } }, k, is_hidden)
                            };
                            __state = 380;
                        }
                        404 => { __state = 80; }
                        405 => { __state = 61; }
                        406 => { unsafe { (*p_parse).n_mem = 6 }; __state = 407; }
                        407 => {
                            unsafe { sqlite3_code_verify_named_schema(p_parse, z_db) };
                            __state = 408;
                        }
                        408 => { ii__4 = 0; __state = 409; }
                        409 => {
                            if ii__4 < unsafe { (*db).n_db } {
                                __state = 410;
                            } else { __state = 405; }
                        }
                        410 => { __state = 412; }
                        411 => {
                            { let __p = &mut ii__4; let __t = *__p; *__p += 1; __t };
                            __state = 409;
                        }
                        412 => { __state = 413; }
                        413 => { __state = 414; }
                        414 => {
                            if !(z_db).is_null() &&
                                    unsafe {
                                            sqlite3_stricmp(z_db,
                                                unsafe {
                                                        (*unsafe { (*db).a_db.offset(ii__4 as isize) }).z_db_s_name
                                                    } as *const i8)
                                        } != 0 {
                                __state = 416;
                            } else { __state = 415; }
                        }
                        415 => {
                            p_hash =
                                unsafe {
                                    &mut (*unsafe {
                                                        (*unsafe { (*db).a_db.offset(ii__4 as isize) }).p_schema
                                                    }).tbl_hash
                                };
                            __state = 417;
                        }
                        416 => { __state = 411; }
                        417 => {
                            init_n_col = unsafe { (*p_hash).count } as i32;
                            __state = 418;
                        }
                        418 => {
                            if {
                                        let __p = &mut init_n_col;
                                        let __t = *__p;
                                        *__p -= 1;
                                        __t
                                    } != 0 {
                                __state = 420;
                            } else { __state = 419; }
                        }
                        419 => { k__1 = unsafe { (*p_hash).first }; __state = 440; }
                        420 => { k__1 = unsafe { (*p_hash).first }; __state = 421; }
                        421 => {
                            if 1 != 0 { __state = 422; } else { __state = 418; }
                        }
                        422 => { __state = 424; }
                        423 => { k__1 = unsafe { (*k__1).next }; __state = 421; }
                        424 => {
                            if k__1 == core::ptr::null_mut() {
                                __state = 426;
                            } else { __state = 425; }
                        }
                        425 => {
                            p_tab_1 = unsafe { (*k__1).data } as *mut Table;
                            __state = 428;
                        }
                        426 => { init_n_col = 0; __state = 427; }
                        427 => { __state = 418; }
                        428 => {
                            if unsafe { (*p_tab_1).n_col } as i32 == 0 {
                                __state = 429;
                            } else { __state = 423; }
                        }
                        429 => {
                            z_sql =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"SELECT*FROM\"%w\"".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab_1).z_name })
                                };
                            __state = 430;
                        }
                        430 => {
                            if !(z_sql).is_null() {
                                __state = 432;
                            } else { __state = 431; }
                        }
                        431 => {
                            if unsafe { (*db).malloc_failed } != 0 {
                                __state = 437;
                            } else { __state = 436; }
                        }
                        432 => { p_dummy = core::ptr::null_mut(); __state = 433; }
                        433 => {
                            {
                                let _ =
                                    unsafe {
                                        sqlite3_prepare_v3(db, z_sql as *const i8, -1, 16 as u32,
                                            &mut p_dummy, core::ptr::null_mut())
                                    };
                            };
                            __state = 434;
                        }
                        434 => {
                            { let _ = unsafe { sqlite3_finalize(p_dummy) }; };
                            __state = 435;
                        }
                        435 => {
                            unsafe { sqlite3_db_free(db, z_sql as *mut ()) };
                            __state = 431;
                        }
                        436 => {
                            p_hash =
                                unsafe {
                                    &mut (*unsafe {
                                                        (*unsafe { (*db).a_db.offset(ii__4 as isize) }).p_schema
                                                    }).tbl_hash
                                };
                            __state = 439;
                        }
                        437 => {
                            unsafe {
                                sqlite3_error_msg(unsafe { (*db).p_parse },
                                    c"out of memory".as_ptr() as *mut i8 as *const i8)
                            };
                            __state = 438;
                        }
                        438 => {
                            unsafe { (*unsafe { (*db).p_parse }).rc = 7 };
                            __state = 436;
                        }
                        439 => { __state = 418; }
                        440 => {
                            if !(k__1).is_null() {
                                __state = 441;
                            } else { __state = 411; }
                        }
                        441 => {
                            p_tab_2 = unsafe { (*k__1).data } as *const Table;
                            __state = 443;
                        }
                        442 => { k__1 = unsafe { (*k__1).next }; __state = 440; }
                        443 => { __state = 444; }
                        444 => {
                            if !(z_right).is_null() &&
                                    unsafe {
                                            sqlite3_stricmp(z_right as *const i8,
                                                unsafe { (*p_tab_2).z_name } as *const i8)
                                        } != 0 {
                                __state = 446;
                            } else { __state = 445; }
                        }
                        445 => {
                            if unsafe { (*p_tab_2).e_tab_type } as i32 == 2 {
                                __state = 448;
                            } else { __state = 449; }
                        }
                        446 => { __state = 442; }
                        447 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    c"sssiii".as_ptr() as *mut i8 as *const i8,
                                    unsafe {
                                        (*unsafe { (*db).a_db.offset(ii__4 as isize) }).z_db_s_name
                                    },
                                    unsafe {
                                        sqlite3_preferred_table_name(unsafe { (*p_tab_2).z_name } as
                                                *const i8)
                                    }, z_type, unsafe { (*p_tab_2).n_col } as i32,
                                    (unsafe { (*p_tab_2).tab_flags } & 128 as u32 != 0 as u32)
                                        as i32,
                                    (unsafe { (*p_tab_2).tab_flags } & 65536 as u32 != 0 as u32)
                                        as i32)
                            };
                            __state = 442;
                        }
                        448 => {
                            z_type = c"view".as_ptr() as *mut i8 as *const i8;
                            __state = 447;
                        }
                        449 => {
                            if unsafe { (*p_tab_2).e_tab_type } as i32 == 1 {
                                __state = 450;
                            } else { __state = 451; }
                        }
                        450 => {
                            z_type = c"virtual".as_ptr() as *mut i8 as *const i8;
                            __state = 447;
                        }
                        451 => {
                            if unsafe { (*p_tab_2).tab_flags } & 4096 as u32 != 0 {
                                __state = 452;
                            } else { __state = 453; }
                        }
                        452 => {
                            z_type = c"shadow".as_ptr() as *mut i8 as *const i8;
                            __state = 447;
                        }
                        453 => {
                            z_type = c"table".as_ptr() as *mut i8 as *const i8;
                            __state = 447;
                        }
                        454 => { __state = 81; }
                        455 => { __state = 61; }
                        456 => { __state = 457; }
                        457 => { __state = 458; }
                        458 => {
                            p_idx =
                                unsafe {
                                    sqlite3_find_index(db, z_right as *const i8, z_db)
                                };
                            __state = 459;
                        }
                        459 => {
                            if p_idx == core::ptr::null_mut() {
                                __state = 461;
                            } else { __state = 460; }
                        }
                        460 => {
                            if !(p_idx).is_null() {
                                __state = 464;
                            } else { __state = 455; }
                        }
                        461 => {
                            p_tab_3 =
                                unsafe {
                                    sqlite3_locate_table(p_parse, 2 as u32,
                                        z_right as *const i8, z_db)
                                };
                            __state = 462;
                        }
                        462 => {
                            if !(p_tab_3).is_null() &&
                                    !(unsafe { (*p_tab_3).tab_flags } & 128 as u32 == 0 as u32)
                                            as i32 != 0 {
                                __state = 463;
                            } else { __state = 460; }
                        }
                        463 => {
                            p_idx = unsafe { sqlite3_primary_key_index(p_tab_3) };
                            __state = 460;
                        }
                        464 => {
                            i_idx_db =
                                unsafe {
                                    sqlite3_schema_to_index(db, unsafe { (*p_idx).p_schema })
                                };
                            __state = 465;
                        }
                        465 => { __state = 466; }
                        466 => { __state = 467; }
                        467 => {
                            if unsafe { (*p_pragma).i_arg } != 0 {
                                __state = 469;
                            } else { __state = 470; }
                        }
                        468 => {
                            p_tab_3 = unsafe { (*p_idx).p_table };
                            __state = 473;
                        }
                        469 => {
                            mx = unsafe { (*p_idx).n_column } as i32;
                            __state = 471;
                        }
                        470 => {
                            mx = unsafe { (*p_idx).n_key_col } as i32;
                            __state = 472;
                        }
                        471 => { unsafe { (*p_parse).n_mem = 6 }; __state = 468; }
                        472 => { unsafe { (*p_parse).n_mem = 3 }; __state = 468; }
                        473 => {
                            unsafe { sqlite3_code_verify_schema(p_parse, i_idx_db) };
                            __state = 474;
                        }
                        474 => { { let _ = 0; }; __state = 475; }
                        475 => { i__1 = 0; __state = 476; }
                        476 => {
                            if i__1 < mx { __state = 477; } else { __state = 455; }
                        }
                        477 => {
                            cnum =
                                unsafe {
                                    *unsafe { (*p_idx).ai_column.offset(i__1 as isize) }
                                };
                            __state = 479;
                        }
                        478 => {
                            { let __p = &mut i__1; let __t = *__p; *__p += 1; __t };
                            __state = 476;
                        }
                        479 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    c"iisX".as_ptr() as *mut i8 as *const i8, i__1, cnum as i32,
                                    if (cnum as i32) < 0 {
                                        core::ptr::null_mut()
                                    } else {
                                        unsafe {
                                            (*unsafe {
                                                        (*p_tab_3).a_col.offset(cnum as isize)
                                                    }).z_cn_name
                                        }
                                    })
                            };
                            __state = 480;
                        }
                        480 => {
                            if unsafe { (*p_pragma).i_arg } != 0 {
                                __state = 482;
                            } else { __state = 481; }
                        }
                        481 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, 1, unsafe { (*p_parse).n_mem })
                            };
                            __state = 478;
                        }
                        482 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 4,
                                    c"isiX".as_ptr() as *mut i8 as *const i8,
                                    unsafe {
                                            *unsafe { (*p_idx).a_sort_order.offset(i__1 as isize) }
                                        } as i32,
                                    unsafe {
                                        *unsafe { (*p_idx).az_coll.offset(i__1 as isize) }
                                    }, (i__1 < unsafe { (*p_idx).n_key_col } as i32) as i32)
                            };
                            __state = 481;
                        }
                        483 => { __state = 82; }
                        484 => { __state = 61; }
                        485 => { __state = 486; }
                        486 => { __state = 487; }
                        487 => { __state = 488; }
                        488 => {
                            p_tab_4 =
                                unsafe {
                                    sqlite3_find_table(db, z_right as *const i8, z_db)
                                };
                            __state = 489;
                        }
                        489 => {
                            if !(p_tab_4).is_null() {
                                __state = 490;
                            } else { __state = 484; }
                        }
                        490 => {
                            i_tab_db =
                                unsafe {
                                    sqlite3_schema_to_index(db, unsafe { (*p_tab_4).p_schema })
                                };
                            __state = 491;
                        }
                        491 => { unsafe { (*p_parse).n_mem = 5 }; __state = 492; }
                        492 => {
                            unsafe { sqlite3_code_verify_schema(p_parse, i_tab_db) };
                            __state = 493;
                        }
                        493 => {
                            { p_idx_1 = unsafe { (*p_tab_4).p_index }; i__2 = 0 };
                            __state = 494;
                        }
                        494 => {
                            if !(p_idx_1).is_null() {
                                __state = 495;
                            } else { __state = 484; }
                        }
                        495 => { __state = 497; }
                        496 => {
                            {
                                p_idx_1 = unsafe { (*p_idx_1).p_next };
                                { let __p = &mut i__2; let __t = *__p; *__p += 1; __t }
                            };
                            __state = 494;
                        }
                        497 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    c"isisi".as_ptr() as *mut i8 as *const i8, i__2,
                                    unsafe { (*p_idx_1).z_name },
                                    (unsafe { (*p_idx_1).on_error } as i32 != 0) as i32,
                                    az_origin[unsafe { (*p_idx_1).idx_type() } as usize],
                                    (unsafe { (*p_idx_1).p_part_idx_where } !=
                                            core::ptr::null_mut()) as i32)
                            };
                            __state = 496;
                        }
                        498 => { __state = 83; }
                        499 => { __state = 61; }
                        500 => { unsafe { (*p_parse).n_mem = 3 }; __state = 501; }
                        501 => { i__3 = 0; __state = 502; }
                        502 => {
                            if i__3 < unsafe { (*db).n_db } {
                                __state = 503;
                            } else { __state = 499; }
                        }
                        503 => {
                            if unsafe {
                                        (*unsafe { (*db).a_db.offset(i__3 as isize) }).p_bt
                                    } == core::ptr::null_mut() {
                                __state = 506;
                            } else { __state = 505; }
                        }
                        504 => {
                            { let __p = &mut i__3; let __t = *__p; *__p += 1; __t };
                            __state = 502;
                        }
                        505 => { { let _ = 0; }; __state = 507; }
                        506 => { __state = 504; }
                        507 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    c"iss".as_ptr() as *mut i8 as *const i8, i__3,
                                    unsafe {
                                        (*unsafe { (*db).a_db.offset(i__3 as isize) }).z_db_s_name
                                    },
                                    unsafe {
                                        sqlite3_btree_get_filename(unsafe {
                                                (*unsafe { (*db).a_db.offset(i__3 as isize) }).p_bt
                                            })
                                    })
                            };
                            __state = 504;
                        }
                        508 => { __state = 84; }
                        509 => { __state = 61; }
                        510 => { __state = 511; }
                        511 => { unsafe { (*p_parse).n_mem = 2 }; __state = 512; }
                        512 => {
                            p = unsafe { (*unsafe { &mut (*db).a_coll_seq }).first };
                            __state = 513;
                        }
                        513 => {
                            if !(p).is_null() { __state = 514; } else { __state = 509; }
                        }
                        514 => {
                            p_coll =
                                unsafe { (*p).data } as *mut CollSeq as *const CollSeq;
                            __state = 516;
                        }
                        515 => { p = unsafe { (*p).next }; __state = 513; }
                        516 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    c"is".as_ptr() as *mut i8 as *const i8,
                                    { let __p = &mut i__4; let __t = *__p; *__p += 1; __t },
                                    unsafe { (*p_coll).z_name })
                            };
                            __state = 515;
                        }
                        517 => { __state = 85; }
                        518 => { __state = 61; }
                        519 => { __state = 520; }
                        520 => { __state = 521; }
                        521 => {
                            show_intern_func =
                                (unsafe { (*db).m_db_flags } & 32 as u32 != 0 as u32) as
                                    i32;
                            __state = 522;
                        }
                        522 => { unsafe { (*p_parse).n_mem = 6 }; __state = 523; }
                        523 => { i__5 = 0; __state = 525; }
                        524 => {
                            j = unsafe { (*unsafe { &mut (*db).a_func }).first };
                            __state = 532;
                        }
                        525 => {
                            if i__5 < 23 { __state = 526; } else { __state = 524; }
                        }
                        526 => {
                            p__1 = sqlite3_builtin_functions.a[i__5 as usize];
                            __state = 528;
                        }
                        527 => {
                            { let __p = &mut i__5; let __t = *__p; *__p += 1; __t };
                            __state = 525;
                        }
                        528 => {
                            if !(p__1).is_null() {
                                __state = 529;
                            } else { __state = 527; }
                        }
                        529 => { { let _ = 0; }; __state = 531; }
                        530 => {
                            p__1 = unsafe { (*p__1).u.p_hash };
                            __state = 528;
                        }
                        531 => {
                            pragma_funclist_line(v, p__1 as *const FuncDef, 1,
                                show_intern_func);
                            __state = 530;
                        }
                        532 => {
                            if !(j).is_null() { __state = 533; } else { __state = 518; }
                        }
                        533 => {
                            p__1 = unsafe { (*j).data } as *mut FuncDef;
                            __state = 535;
                        }
                        534 => { j = unsafe { (*j).next }; __state = 532; }
                        535 => { { let _ = 0; }; __state = 536; }
                        536 => {
                            pragma_funclist_line(v, p__1 as *const FuncDef, 0,
                                show_intern_func);
                            __state = 534;
                        }
                        537 => { __state = 86; }
                        538 => { __state = 61; }
                        539 => { unsafe { (*p_parse).n_mem = 1 }; __state = 540; }
                        540 => {
                            j__1 = unsafe { (*unsafe { &mut (*db).a_module }).first };
                            __state = 541;
                        }
                        541 => {
                            if !(j__1).is_null() {
                                __state = 542;
                            } else { __state = 538; }
                        }
                        542 => {
                            p_mod =
                                unsafe { (*j__1).data } as *mut Module as *const Module;
                            __state = 544;
                        }
                        543 => { j__1 = unsafe { (*j__1).next }; __state = 541; }
                        544 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    c"s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_mod).z_name })
                            };
                            __state = 543;
                        }
                        545 => { __state = 87; }
                        546 => { __state = 61; }
                        547 => { i__6 = 0; __state = 548; }
                        548 => {
                            if i__6 <
                                    (core::mem::size_of::<[PragmaName; 67]>() as u64 /
                                            core::mem::size_of::<PragmaName>() as u64) as i32 {
                                __state = 549;
                            } else { __state = 546; }
                        }
                        549 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    c"s".as_ptr() as *mut i8 as *const i8,
                                    a_pragma_name[i__6 as usize].z_name)
                            };
                            __state = 550;
                        }
                        550 => {
                            { let __p = &mut i__6; let __t = *__p; *__p += 1; __t };
                            __state = 548;
                        }
                        551 => { __state = 88; }
                        552 => { __state = 61; }
                        553 => { __state = 554; }
                        554 => { __state = 555; }
                        555 => {
                            p_tab_5 =
                                unsafe {
                                    sqlite3_find_table(db, z_right as *const i8, z_db)
                                };
                            __state = 556;
                        }
                        556 => {
                            if !(p_tab_5).is_null() &&
                                    unsafe { (*p_tab_5).e_tab_type } as i32 == 0 {
                                __state = 557;
                            } else { __state = 552; }
                        }
                        557 => {
                            p_fk = unsafe { (*p_tab_5).u.tab.p_f_key };
                            __state = 558;
                        }
                        558 => {
                            if !(p_fk).is_null() {
                                __state = 559;
                            } else { __state = 552; }
                        }
                        559 => {
                            i_tab_db_1 =
                                unsafe {
                                    sqlite3_schema_to_index(db, unsafe { (*p_tab_5).p_schema })
                                };
                            __state = 560;
                        }
                        560 => { i__7 = 0; __state = 561; }
                        561 => { unsafe { (*p_parse).n_mem = 8 }; __state = 562; }
                        562 => {
                            unsafe { sqlite3_code_verify_schema(p_parse, i_tab_db_1) };
                            __state = 563;
                        }
                        563 => {
                            if !(p_fk).is_null() {
                                __state = 564;
                            } else { __state = 552; }
                        }
                        564 => { __state = 565; }
                        565 => { j__2 = 0; __state = 567; }
                        566 => {
                            { let __p = &mut i__7; *__p += 1; *__p };
                            __state = 570;
                        }
                        567 => {
                            if j__2 < unsafe { (*p_fk).n_col } {
                                __state = 568;
                            } else { __state = 566; }
                        }
                        568 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, 1,
                                    c"iissssss".as_ptr() as *mut i8 as *const i8, i__7, j__2,
                                    unsafe { (*p_fk).z_to },
                                    unsafe {
                                        (*unsafe {
                                                    (*p_tab_5).a_col.offset(unsafe {
                                                                (*(unsafe { (*p_fk).a_col.as_ptr() } as
                                                                                *mut sColMap).offset(j__2 as isize)).i_from
                                                            } as isize)
                                                }).z_cn_name
                                    },
                                    unsafe {
                                        (*(unsafe { (*p_fk).a_col.as_ptr() } as
                                                        *mut sColMap).offset(j__2 as isize)).z_col
                                    }, action_name(unsafe { (*p_fk).a_action[1 as usize] }),
                                    action_name(unsafe { (*p_fk).a_action[0 as usize] }),
                                    c"NONE".as_ptr() as *mut i8)
                            };
                            __state = 569;
                        }
                        569 => {
                            { let __p = &mut j__2; let __t = *__p; *__p += 1; __t };
                            __state = 567;
                        }
                        570 => {
                            p_fk = unsafe { (*p_fk).p_next_from };
                            __state = 563;
                        }
                        571 => { __state = 89; }
                        572 => { __state = 61; }
                        573 => { __state = 574; }
                        574 => { __state = 575; }
                        575 => { __state = 576; }
                        576 => { __state = 577; }
                        577 => { __state = 578; }
                        578 => { __state = 579; }
                        579 => { __state = 580; }
                        580 => { __state = 581; }
                        581 => { __state = 582; }
                        582 => { __state = 583; }
                        583 => { __state = 584; }
                        584 => { __state = 585; }
                        585 => {
                            reg_result = unsafe { (*p_parse).n_mem } + 1;
                            __state = 586;
                        }
                        586 => { unsafe { (*p_parse).n_mem += 4 }; __state = 587; }
                        587 => {
                            reg_row =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                };
                            __state = 588;
                        }
                        588 => {
                            k__2 =
                                unsafe {
                                    (*unsafe {
                                                    &mut (*unsafe {
                                                                        (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                                                    }).tbl_hash
                                                }).first
                                };
                            __state = 589;
                        }
                        589 => {
                            if !(k__2).is_null() {
                                __state = 590;
                            } else { __state = 572; }
                        }
                        590 => {
                            if !(z_right).is_null() {
                                __state = 592;
                            } else { __state = 593; }
                        }
                        591 => {
                            if p_tab_6 == core::ptr::null_mut() ||
                                        !(unsafe { (*p_tab_6).e_tab_type } as i32 == 0) as i32 != 0
                                    ||
                                    unsafe { (*p_tab_6).u.tab.p_f_key } == core::ptr::null_mut()
                                {
                                __state = 597;
                            } else { __state = 596; }
                        }
                        592 => {
                            p_tab_6 =
                                unsafe {
                                    sqlite3_locate_table(p_parse, 0 as u32,
                                        z_right as *const i8, z_db)
                                };
                            __state = 594;
                        }
                        593 => {
                            p_tab_6 = unsafe { (*k__2).data } as *mut Table;
                            __state = 595;
                        }
                        594 => { k__2 = core::ptr::null_mut(); __state = 591; }
                        595 => { k__2 = unsafe { (*k__2).next }; __state = 591; }
                        596 => {
                            i_db =
                                unsafe {
                                    sqlite3_schema_to_index(db, unsafe { (*p_tab_6).p_schema })
                                };
                            __state = 598;
                        }
                        597 => { __state = 589; }
                        598 => {
                            z_db =
                                unsafe {
                                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                    } as *const i8;
                            __state = 599;
                        }
                        599 => {
                            unsafe { sqlite3_code_verify_schema(p_parse, i_db) };
                            __state = 600;
                        }
                        600 => {
                            unsafe {
                                sqlite3_table_lock(p_parse, i_db,
                                    unsafe { (*p_tab_6).tnum }, 0 as u8,
                                    unsafe { (*p_tab_6).z_name } as *const i8)
                            };
                            __state = 601;
                        }
                        601 => {
                            unsafe {
                                sqlite3_touch_register(p_parse,
                                    unsafe { (*p_tab_6).n_col } as i32 + reg_row)
                            };
                            __state = 602;
                        }
                        602 => {
                            unsafe {
                                sqlite3_open_table(p_parse, 0, i_db, p_tab_6, 114)
                            };
                            __state = 603;
                        }
                        603 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, reg_result,
                                    unsafe { (*p_tab_6).z_name } as *const i8)
                            };
                            __state = 604;
                        }
                        604 => { { let _ = 0; }; __state = 605; }
                        605 => {
                            { i__8 = 1; p_fk_1 = unsafe { (*p_tab_6).u.tab.p_f_key } };
                            __state = 607;
                        }
                        606 => { { let _ = 0; }; __state = 622; }
                        607 => {
                            if !(p_fk_1).is_null() {
                                __state = 608;
                            } else { __state = 606; }
                        }
                        608 => {
                            p_parent =
                                unsafe {
                                    sqlite3_find_table(db,
                                        unsafe { (*p_fk_1).z_to } as *const i8, z_db)
                                };
                            __state = 610;
                        }
                        609 => {
                            {
                                { let __p = &mut i__8; let __t = *__p; *__p += 1; __t };
                                p_fk_1 = unsafe { (*p_fk_1).p_next_from }
                            };
                            __state = 607;
                        }
                        610 => {
                            if p_parent == core::ptr::null_mut() {
                                __state = 612;
                            } else { __state = 611; }
                        }
                        611 => { p_idx_2 = core::ptr::null_mut(); __state = 613; }
                        612 => { __state = 609; }
                        613 => {
                            unsafe {
                                sqlite3_table_lock(p_parse, i_db,
                                    unsafe { (*p_parent).tnum }, 0 as u8,
                                    unsafe { (*p_parent).z_name } as *const i8)
                            };
                            __state = 614;
                        }
                        614 => {
                            x__1 =
                                unsafe {
                                    sqlite3_fk_locate_index(p_parse, p_parent, p_fk_1,
                                        &mut p_idx_2, core::ptr::null_mut())
                                };
                            __state = 615;
                        }
                        615 => {
                            if x__1 == 0 { __state = 616; } else { __state = 617; }
                        }
                        616 => {
                            if p_idx_2 == core::ptr::null_mut() {
                                __state = 618;
                            } else { __state = 619; }
                        }
                        617 => { k__2 = core::ptr::null_mut(); __state = 621; }
                        618 => {
                            unsafe {
                                sqlite3_open_table(p_parse, i__8, i_db, p_parent, 114)
                            };
                            __state = 609;
                        }
                        619 => {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 114, i__8,
                                    unsafe { (*p_idx_2).tnum } as i32, i_db)
                            };
                            __state = 620;
                        }
                        620 => {
                            unsafe { sqlite3_vdbe_set_p4_key_info(p_parse, p_idx_2) };
                            __state = 609;
                        }
                        621 => { __state = 606; }
                        622 => {
                            if !(p_fk_1).is_null() {
                                __state = 624;
                            } else { __state = 623; }
                        }
                        623 => {
                            if unsafe { (*p_parse).n_tab } < i__8 {
                                __state = 626;
                            } else { __state = 625; }
                        }
                        624 => { __state = 572; }
                        625 => {
                            addr_top = unsafe { sqlite3_vdbe_add_op1(v, 36, 0) };
                            __state = 627;
                        }
                        626 => {
                            unsafe { (*p_parse).n_tab = i__8 };
                            __state = 625;
                        }
                        627 => { __state = 628; }
                        628 => { { let _ = 0; }; __state = 629; }
                        629 => {
                            { i__8 = 1; p_fk_1 = unsafe { (*p_tab_6).u.tab.p_f_key } };
                            __state = 631;
                        }
                        630 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 40, 0, addr_top + 1) };
                            __state = 665;
                        }
                        631 => {
                            if !(p_fk_1).is_null() {
                                __state = 632;
                            } else { __state = 630; }
                        }
                        632 => {
                            p_parent =
                                unsafe {
                                    sqlite3_find_table(db,
                                        unsafe { (*p_fk_1).z_to } as *const i8, z_db)
                                };
                            __state = 634;
                        }
                        633 => {
                            {
                                { let __p = &mut i__8; let __t = *__p; *__p += 1; __t };
                                p_fk_1 = unsafe { (*p_fk_1).p_next_from }
                            };
                            __state = 631;
                        }
                        634 => { p_idx_2 = core::ptr::null_mut(); __state = 635; }
                        635 => { ai_cols = core::ptr::null_mut(); __state = 636; }
                        636 => {
                            if !(p_parent).is_null() {
                                __state = 638;
                            } else { __state = 637; }
                        }
                        637 => {
                            addr_ok = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            __state = 640;
                        }
                        638 => {
                            x__1 =
                                unsafe {
                                    sqlite3_fk_locate_index(p_parse, p_parent, p_fk_1,
                                        &mut p_idx_2, &mut ai_cols)
                                };
                            __state = 639;
                        }
                        639 => { { let _ = 0; }; __state = 637; }
                        640 => {
                            unsafe {
                                sqlite3_touch_register(p_parse,
                                    reg_row + unsafe { (*p_fk_1).n_col })
                            };
                            __state = 641;
                        }
                        641 => { j__3 = 0; __state = 643; }
                        642 => {
                            if !(p_idx_2).is_null() {
                                __state = 650;
                            } else { __state = 651; }
                        }
                        643 => {
                            if j__3 < unsafe { (*p_fk_1).n_col } {
                                __state = 644;
                            } else { __state = 642; }
                        }
                        644 => {
                            i_col =
                                if !(ai_cols).is_null() {
                                    unsafe { *ai_cols.offset(j__3 as isize) }
                                } else {
                                    unsafe {
                                        (*(unsafe { (*p_fk_1).a_col.as_ptr() } as
                                                        *mut sColMap).offset(j__3 as isize)).i_from
                                    }
                                };
                            __state = 646;
                        }
                        645 => {
                            { let __p = &mut j__3; let __t = *__p; *__p += 1; __t };
                            __state = 643;
                        }
                        646 => {
                            unsafe {
                                sqlite3_expr_code_get_column_of_table(v, p_tab_6, 0, i_col,
                                    reg_row + j__3)
                            };
                            __state = 647;
                        }
                        647 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_row + j__3, addr_ok)
                            };
                            __state = 648;
                        }
                        648 => { __state = 645; }
                        649 => {
                            if unsafe { (*p_tab_6).tab_flags } & 128 as u32 == 0 as u32
                                {
                                __state = 660;
                            } else { __state = 661; }
                        }
                        650 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 98, reg_row,
                                    unsafe { (*p_fk_1).n_col }, 0,
                                    unsafe { sqlite3_index_affinity_str(db, p_idx_2) },
                                    unsafe { (*p_fk_1).n_col })
                            };
                            __state = 652;
                        }
                        651 => {
                            if !(p_parent).is_null() {
                                __state = 654;
                            } else { __state = 649; }
                        }
                        652 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i__8, addr_ok, reg_row,
                                    unsafe { (*p_fk_1).n_col })
                            };
                            __state = 653;
                        }
                        653 => { __state = 649; }
                        654 => {
                            jmp = unsafe { sqlite3_vdbe_current_addr(v) } + 2;
                            __state = 655;
                        }
                        655 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 30, i__8, jmp, reg_row) };
                            __state = 656;
                        }
                        656 => { __state = 657; }
                        657 => {
                            unsafe { sqlite3_vdbe_goto(v, addr_ok) };
                            __state = 658;
                        }
                        658 => { { let _ = 0; }; __state = 649; }
                        659 => {
                            unsafe {
                                sqlite3_vdbe_multi_load(v, reg_result + 2,
                                    c"siX".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_fk_1).z_to }, i__8 - 1)
                            };
                            __state = 662;
                        }
                        660 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 137, 0, reg_result + 1) };
                            __state = 659;
                        }
                        661 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 77, 0, reg_result + 1) };
                            __state = 659;
                        }
                        662 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 86, reg_result, 4) };
                            __state = 663;
                        }
                        663 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, addr_ok) };
                            __state = 664;
                        }
                        664 => {
                            unsafe { sqlite3_db_free(db, ai_cols as *mut ()) };
                            __state = 633;
                        }
                        665 => { __state = 666; }
                        666 => {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_top) };
                            __state = 589;
                        }
                        667 => { __state = 90; }
                        668 => { __state = 61; }
                        669 => {
                            unsafe {
                                sqlite3_register_like_functions(db,
                                    sqlite3_get_boolean(z_right as *const i8, 0 as u8) as i32)
                            };
                            __state = 668;
                        }
                        670 => { __state = 91; }
                        671 => { __state = 61; }
                        672 => { p_obj_tab = core::ptr::null(); __state = 673; }
                        673 => {
                            is_quick =
                                (unsafe {
                                                *(sqlite3_upper_to_lower.as_ptr() as
                                                            *const u8).add(unsafe { *z_left.offset(0 as isize) } as u8
                                                            as usize)
                                            } as i32 == 'q' as i32) as i32;
                            __state = 674;
                        }
                        674 => { { let _ = 0; }; __state = 675; }
                        675 => { { let _ = 0; }; __state = 676; }
                        676 => {
                            if unsafe { (*p_id2).z } == core::ptr::null() {
                                __state = 678;
                            } else { __state = 677; }
                        }
                        677 => { unsafe { (*p_parse).n_mem = 6 }; __state = 679; }
                        678 => { i_db = -1; __state = 677; }
                        679 => { mx_err = 100; __state = 680; }
                        680 => {
                            if !(z_right).is_null() {
                                __state = 682;
                            } else { __state = 681; }
                        }
                        681 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 73, mx_err - 1, 1) };
                            __state = 686;
                        }
                        682 => {
                            if unsafe {
                                        sqlite3_get_int32(unsafe { (*p_value).z }, &mut mx_err)
                                    } != 0 {
                                __state = 683;
                            } else { __state = 684; }
                        }
                        683 => {
                            if mx_err <= 0 { __state = 685; } else { __state = 681; }
                        }
                        684 => {
                            p_obj_tab =
                                unsafe {
                                    sqlite3_locate_table(p_parse, 0 as u32,
                                        z_right as *const i8,
                                        if i_db >= 0 {
                                                unsafe {
                                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                                }
                                            } else { core::ptr::null_mut() } as *const i8)
                                };
                            __state = 681;
                        }
                        685 => { mx_err = 100; __state = 681; }
                        686 => { i__9 = 0; __state = 688; }
                        687 => { __state = 1073; }
                        688 => {
                            if i__9 < unsafe { (*db).n_db } {
                                __state = 689;
                            } else { __state = 687; }
                        }
                        689 => { __state = 691; }
                        690 => {
                            { let __p = &mut i__9; let __t = *__p; *__p += 1; __t };
                            __state = 688;
                        }
                        691 => { __state = 692; }
                        692 => { __state = 693; }
                        693 => { cnt = 0; __state = 694; }
                        694 => {
                            if 0 != 0 && i__9 == 1 {
                                __state = 696;
                            } else { __state = 695; }
                        }
                        695 => {
                            if i_db >= 0 && i__9 != i_db {
                                __state = 698;
                            } else { __state = 697; }
                        }
                        696 => { __state = 690; }
                        697 => {
                            unsafe { sqlite3_code_verify_schema(p_parse, i__9) };
                            __state = 699;
                        }
                        698 => { __state = 690; }
                        699 => {
                            unsafe { (*p_parse).set_ok_const_factor(0 as bft as u32) };
                            __state = 700;
                        }
                        700 => { { let _ = 0; }; __state = 701; }
                        701 => {
                            p_tbls =
                                unsafe {
                                    &mut (*unsafe {
                                                        (*unsafe { (*db).a_db.offset(i__9 as isize) }).p_schema
                                                    }).tbl_hash
                                };
                            __state = 702;
                        }
                        702 => {
                            { cnt = 0; x__2 = unsafe { (*p_tbls).first } };
                            __state = 704;
                        }
                        703 => {
                            if cnt == 0 { __state = 718; } else { __state = 717; }
                        }
                        704 => {
                            if !(x__2).is_null() {
                                __state = 705;
                            } else { __state = 703; }
                        }
                        705 => {
                            p_tab_7 = unsafe { (*x__2).data } as *const Table;
                            __state = 707;
                        }
                        706 => { x__2 = unsafe { (*x__2).next }; __state = 704; }
                        707 => { __state = 708; }
                        708 => { __state = 709; }
                        709 => {
                            if table_skip_integrity_check(p_tab_7 as *const Table,
                                        p_obj_tab as *const Table) != 0 {
                                __state = 711;
                            } else { __state = 710; }
                        }
                        710 => {
                            if unsafe { (*p_tab_7).tab_flags } & 128 as u32 == 0 as u32
                                {
                                __state = 713;
                            } else { __state = 712; }
                        }
                        711 => { __state = 706; }
                        712 => {
                            { n_idx = 0; p_idx_3 = unsafe { (*p_tab_7).p_index } };
                            __state = 714;
                        }
                        713 => {
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            __state = 712;
                        }
                        714 => {
                            if !(p_idx_3).is_null() {
                                __state = 715;
                            } else { __state = 706; }
                        }
                        715 => {
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            __state = 716;
                        }
                        716 => {
                            {
                                p_idx_3 = unsafe { (*p_idx_3).p_next };
                                { let __p = &mut n_idx; let __t = *__p; *__p += 1; __t }
                            };
                            __state = 714;
                        }
                        717 => {
                            if !(p_obj_tab).is_null() {
                                __state = 720;
                            } else { __state = 719; }
                        }
                        718 => { __state = 690; }
                        719 => {
                            a_root =
                                unsafe {
                                        sqlite3_db_malloc_raw_nn(db,
                                            core::mem::size_of::<i32>() as u64 * (cnt + 1) as u64)
                                    } as *mut i32;
                            __state = 721;
                        }
                        720 => {
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            __state = 719;
                        }
                        721 => {
                            if a_root == core::ptr::null_mut() {
                                __state = 723;
                            } else { __state = 722; }
                        }
                        722 => { cnt = 0; __state = 724; }
                        723 => { __state = 687; }
                        724 => {
                            if !(p_obj_tab).is_null() {
                                __state = 726;
                            } else { __state = 725; }
                        }
                        725 => { x__2 = unsafe { (*p_tbls).first }; __state = 728; }
                        726 => {
                            unsafe {
                                *a_root.offset({ let __p = &mut cnt; *__p += 1; *__p } as
                                                isize) = 0
                            };
                            __state = 725;
                        }
                        727 => {
                            unsafe { *a_root.offset(0 as isize) = cnt };
                            __state = 740;
                        }
                        728 => {
                            if !(x__2).is_null() {
                                __state = 729;
                            } else { __state = 727; }
                        }
                        729 => {
                            p_tab_8 = unsafe { (*x__2).data } as *const Table;
                            __state = 731;
                        }
                        730 => { x__2 = unsafe { (*x__2).next }; __state = 728; }
                        731 => { __state = 732; }
                        732 => {
                            if table_skip_integrity_check(p_tab_8 as *const Table,
                                        p_obj_tab as *const Table) != 0 {
                                __state = 734;
                            } else { __state = 733; }
                        }
                        733 => {
                            if unsafe { (*p_tab_8).tab_flags } & 128 as u32 == 0 as u32
                                {
                                __state = 736;
                            } else { __state = 735; }
                        }
                        734 => { __state = 730; }
                        735 => {
                            p_idx_4 = unsafe { (*p_tab_8).p_index };
                            __state = 737;
                        }
                        736 => {
                            unsafe {
                                *a_root.offset({ let __p = &mut cnt; *__p += 1; *__p } as
                                                isize) = unsafe { (*p_tab_8).tnum } as i32
                            };
                            __state = 735;
                        }
                        737 => {
                            if !(p_idx_4).is_null() {
                                __state = 738;
                            } else { __state = 730; }
                        }
                        738 => {
                            unsafe {
                                *a_root.offset({ let __p = &mut cnt; *__p += 1; *__p } as
                                                isize) = unsafe { (*p_idx_4).tnum } as i32
                            };
                            __state = 739;
                        }
                        739 => {
                            p_idx_4 = unsafe { (*p_idx_4).p_next };
                            __state = 737;
                        }
                        740 => {
                            unsafe { sqlite3_touch_register(p_parse, 8 + cnt) };
                            __state = 741;
                        }
                        741 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 77, 0, 8, 8 + cnt) };
                            __state = 742;
                        }
                        742 => {
                            unsafe { sqlite3_clear_temp_reg_cache(p_parse) };
                            __state = 743;
                        }
                        743 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 157, 1, cnt, 8,
                                    a_root as *mut i8 as *const i8, -15)
                            };
                            __state = 744;
                        }
                        744 => {
                            unsafe { sqlite3_vdbe_change_p5(v, i__9 as u16) };
                            __state = 745;
                        }
                        745 => {
                            addr__1 = unsafe { sqlite3_vdbe_add_op1(v, 51, 2) };
                            __state = 746;
                        }
                        746 => { __state = 747; }
                        747 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, 3, 0,
                                    unsafe {
                                            sqlite3_m_printf(db,
                                                c"*** in database %s ***\n".as_ptr() as *mut i8 as
                                                    *const i8,
                                                unsafe {
                                                    (*unsafe { (*db).a_db.offset(i__9 as isize) }).z_db_s_name
                                                })
                                        } as *const i8, -7)
                            };
                            __state = 748;
                        }
                        748 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 112, 2, 3, 3) };
                            __state = 749;
                        }
                        749 => { integrity_check_result_row(v); __state = 750; }
                        750 => {
                            unsafe { sqlite3_vdbe_jump_here(v, addr__1) };
                            __state = 751;
                        }
                        751 => {
                            cnt = if !(p_obj_tab).is_null() { 1 } else { 0 };
                            __state = 752;
                        }
                        752 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 2,
                                    c"wrong # of entries in index ".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                            __state = 753;
                        }
                        753 => { x__2 = unsafe { (*p_tbls).first }; __state = 755; }
                        754 => { x__2 = unsafe { (*p_tbls).first }; __state = 783; }
                        755 => {
                            if !(x__2).is_null() {
                                __state = 756;
                            } else { __state = 754; }
                        }
                        756 => { i_tab = 0; __state = 758; }
                        757 => { x__2 = unsafe { (*x__2).next }; __state = 755; }
                        758 => {
                            p_tab_9 = unsafe { (*x__2).data } as *const Table;
                            __state = 759;
                        }
                        759 => { __state = 760; }
                        760 => {
                            if table_skip_integrity_check(p_tab_9 as *const Table,
                                        p_obj_tab as *const Table) != 0 {
                                __state = 762;
                            } else { __state = 761; }
                        }
                        761 => {
                            if unsafe { (*p_tab_9).tab_flags } & 128 as u32 == 0 as u32
                                {
                                __state = 764;
                            } else { __state = 765; }
                        }
                        762 => { __state = 757; }
                        763 => {
                            p_idx_5 = unsafe { (*p_tab_9).p_index };
                            __state = 772;
                        }
                        764 => {
                            i_tab =
                                { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            __state = 763;
                        }
                        765 => { i_tab = cnt; __state = 766; }
                        766 => {
                            p_idx_5 = unsafe { (*p_tab_9).p_index };
                            __state = 767;
                        }
                        767 => {
                            if !(p_idx_5).is_null() {
                                __state = 768;
                            } else { __state = 763; }
                        }
                        768 => {
                            if unsafe { (*p_idx_5).idx_type() } as i32 == 2 {
                                __state = 771;
                            } else { __state = 770; }
                        }
                        769 => {
                            p_idx_5 = unsafe { (*p_idx_5).p_next };
                            __state = 767;
                        }
                        770 => {
                            { let __p = &mut i_tab; let __t = *__p; *__p += 1; __t };
                            __state = 769;
                        }
                        771 => { __state = 763; }
                        772 => {
                            if !(p_idx_5).is_null() {
                                __state = 773;
                            } else { __state = 757; }
                        }
                        773 => {
                            if unsafe { (*p_idx_5).p_part_idx_where } ==
                                    core::ptr::null_mut() {
                                __state = 776;
                            } else { __state = 775; }
                        }
                        774 => {
                            p_idx_5 = unsafe { (*p_idx_5).p_next };
                            __state = 772;
                        }
                        775 => {
                            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t };
                            __state = 774;
                        }
                        776 => {
                            addr__1 =
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 54, 8 + cnt, 0, 8 + i_tab)
                                };
                            __state = 777;
                        }
                        777 => { __state = 778; }
                        778 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 4,
                                    unsafe { (*p_idx_5).z_name } as *const i8)
                            };
                            __state = 779;
                        }
                        779 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 112, 4, 2, 3) };
                            __state = 780;
                        }
                        780 => { integrity_check_result_row(v); __state = 781; }
                        781 => {
                            unsafe { sqlite3_vdbe_jump_here(v, addr__1) };
                            __state = 775;
                        }
                        782 => {
                            x__2 = unsafe { (*p_tbls).first };
                            __state = 1038;
                        }
                        783 => {
                            if !(x__2).is_null() {
                                __state = 784;
                            } else { __state = 782; }
                        }
                        784 => {
                            p_tab_10 = unsafe { (*x__2).data } as *mut Table;
                            __state = 786;
                        }
                        785 => { x__2 = unsafe { (*x__2).next }; __state = 783; }
                        786 => { __state = 787; }
                        787 => { p_prior = core::ptr::null_mut(); __state = 788; }
                        788 => { __state = 789; }
                        789 => { __state = 790; }
                        790 => { r1 = -1; __state = 791; }
                        791 => { __state = 792; }
                        792 => { __state = 793; }
                        793 => { __state = 794; }
                        794 => {
                            if table_skip_integrity_check(p_tab_10 as *const Table,
                                        p_obj_tab as *const Table) != 0 {
                                __state = 796;
                            } else { __state = 795; }
                        }
                        795 => {
                            if !(unsafe { (*p_tab_10).e_tab_type } as i32 == 0) as i32
                                    != 0 {
                                __state = 798;
                            } else { __state = 797; }
                        }
                        796 => { __state = 785; }
                        797 => {
                            if is_quick != 0 ||
                                    unsafe { (*p_tab_10).tab_flags } & 128 as u32 == 0 as u32 {
                                __state = 800;
                            } else { __state = 801; }
                        }
                        798 => { __state = 785; }
                        799 => {
                            unsafe {
                                sqlite3_open_table_and_indices(p_parse, p_tab_10, 114,
                                    0 as u8, 1, core::ptr::null_mut(), &mut i_data_cur,
                                    &mut i_idx_cur)
                            };
                            __state = 805;
                        }
                        800 => { p_pk_1 = core::ptr::null_mut(); __state = 802; }
                        801 => {
                            p_pk_1 = unsafe { sqlite3_primary_key_index(p_tab_10) };
                            __state = 803;
                        }
                        802 => { r2 = 0; __state = 799; }
                        803 => {
                            r2 =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse,
                                        unsafe { (*p_pk_1).n_key_col } as i32)
                                };
                            __state = 804;
                        }
                        804 => {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 77, 1, r2,
                                    r2 + unsafe { (*p_pk_1).n_key_col } as i32 - 1)
                            };
                            __state = 799;
                        }
                        805 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 73, 0, 7) };
                            __state = 806;
                        }
                        806 => {
                            { j__4 = 0; p_idx_6 = unsafe { (*p_tab_10).p_index } };
                            __state = 808;
                        }
                        807 => { { let _ = 0; }; __state = 811; }
                        808 => {
                            if !(p_idx_6).is_null() {
                                __state = 809;
                            } else { __state = 807; }
                        }
                        809 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 73, 0, 8 + j__4) };
                            __state = 810;
                        }
                        810 => {
                            {
                                p_idx_6 = unsafe { (*p_idx_6).p_next };
                                { let __p = &mut j__4; let __t = *__p; *__p += 1; __t }
                            };
                            __state = 808;
                        }
                        811 => { { let _ = 0; }; __state = 812; }
                        812 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 36, i_data_cur, 0) };
                            __state = 813;
                        }
                        813 => { __state = 814; }
                        814 => {
                            loop_top = unsafe { sqlite3_vdbe_add_op2(v, 88, 7, 1) };
                            __state = 815;
                        }
                        815 => { { let _ = 0; }; __state = 816; }
                        816 => {
                            if unsafe { (*p_tab_10).tab_flags } & 128 as u32 == 0 as u32
                                {
                                __state = 818;
                            } else { __state = 819; }
                        }
                        817 => {
                            if mx_col >= 0 { __state = 828; } else { __state = 827; }
                        }
                        818 => { mx_col = -1; __state = 820; }
                        819 => {
                            mx_col =
                                unsafe {
                                            (*unsafe { sqlite3_primary_key_index(p_tab_10) }).n_column
                                        } as i32 - 1;
                            __state = 817;
                        }
                        820 => { j__4 = 0; __state = 822; }
                        821 => {
                            if mx_col == unsafe { (*p_tab_10).i_p_key } as i32 {
                                __state = 826;
                            } else { __state = 817; }
                        }
                        822 => {
                            if j__4 < unsafe { (*p_tab_10).n_col } as i32 {
                                __state = 823;
                            } else { __state = 821; }
                        }
                        823 => {
                            if unsafe {
                                                (*unsafe {
                                                            (*p_tab_10).a_col.offset(j__4 as isize)
                                                        }).col_flags
                                            } as i32 & 32 == 0 {
                                __state = 825;
                            } else { __state = 824; }
                        }
                        824 => {
                            { let __p = &mut j__4; let __t = *__p; *__p += 1; __t };
                            __state = 822;
                        }
                        825 => {
                            { let __p = &mut mx_col; let __t = *__p; *__p += 1; __t };
                            __state = 824;
                        }
                        826 => {
                            { let __p = &mut mx_col; let __t = *__p; *__p -= 1; __t };
                            __state = 817;
                        }
                        827 => {
                            if (is_quick == 0) as i32 != 0 {
                                __state = 831;
                            } else { __state = 830; }
                        }
                        828 => {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 96, i_data_cur, mx_col, 3)
                            };
                            __state = 829;
                        }
                        829 => {
                            unsafe { sqlite3_vdbe_typeof_column(v, 3) };
                            __state = 827;
                        }
                        830 => {
                            b_strict =
                                (unsafe { (*p_tab_10).tab_flags } & 65536 as u32 !=
                                        0 as u32) as i32;
                            __state = 847;
                        }
                        831 => {
                            if !(p_pk_1).is_null() {
                                __state = 832;
                            } else { __state = 830; }
                        }
                        832 => { __state = 833; }
                        833 => { __state = 834; }
                        834 => {
                            a1 =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 42, i_data_cur, 0, r2,
                                        unsafe { (*p_pk_1).n_key_col } as i32)
                                };
                            __state = 835;
                        }
                        835 => { __state = 836; }
                        836 => {
                            unsafe { sqlite3_vdbe_add_op1(v, 51, r2) };
                            __state = 837;
                        }
                        837 => { __state = 838; }
                        838 => {
                            z_err =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"row not in PRIMARY KEY order for %s".as_ptr() as *mut i8
                                            as *const i8, unsafe { (*p_tab_10).z_name })
                                };
                            __state = 839;
                        }
                        839 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, 3, 0, z_err as *const i8,
                                    -7)
                            };
                            __state = 840;
                        }
                        840 => { integrity_check_result_row(v); __state = 841; }
                        841 => {
                            unsafe { sqlite3_vdbe_jump_here(v, a1) };
                            __state = 842;
                        }
                        842 => {
                            unsafe { sqlite3_vdbe_jump_here(v, a1 + 1) };
                            __state = 843;
                        }
                        843 => { j__4 = 0; __state = 844; }
                        844 => {
                            if j__4 < unsafe { (*p_pk_1).n_key_col } as i32 {
                                __state = 845;
                            } else { __state = 830; }
                        }
                        845 => {
                            unsafe {
                                sqlite3_expr_code_load_index_column(p_parse, p_pk_1,
                                    i_data_cur, j__4, r2 + j__4)
                            };
                            __state = 846;
                        }
                        846 => {
                            { let __p = &mut j__4; let __t = *__p; *__p += 1; __t };
                            __state = 844;
                        }
                        847 => { j__4 = 0; __state = 849; }
                        848 => {
                            if !(unsafe { (*p_tab_10).p_check }).is_null() &&
                                    unsafe { (*db).flags } & 512 as u64 == 0 as u64 {
                                __state = 932;
                            } else { __state = 931; }
                        }
                        849 => {
                            if j__4 < unsafe { (*p_tab_10).n_col } as i32 {
                                __state = 850;
                            } else { __state = 848; }
                        }
                        850 => { __state = 852; }
                        851 => {
                            { let __p = &mut j__4; let __t = *__p; *__p += 1; __t };
                            __state = 849;
                        }
                        852 => {
                            p_col_1 =
                                unsafe {
                                    unsafe { (*p_tab_10).a_col.offset(j__4 as isize) }
                                };
                            __state = 853;
                        }
                        853 => { __state = 854; }
                        854 => { __state = 855; }
                        855 => { __state = 856; }
                        856 => { __state = 857; }
                        857 => {
                            if j__4 == unsafe { (*p_tab_10).i_p_key } as i32 {
                                __state = 859;
                            } else { __state = 858; }
                        }
                        858 => {
                            if b_strict != 0 { __state = 861; } else { __state = 862; }
                        }
                        859 => { __state = 851; }
                        860 => {
                            if unsafe { (*p_col_1).not_null() } as i32 == 0 &&
                                    (do_type_check == 0) as i32 != 0 {
                                __state = 864;
                            } else { __state = 863; }
                        }
                        861 => {
                            do_type_check =
                                (unsafe { (*p_col_1).e_c_type() } as i32 > 1) as i32;
                            __state = 860;
                        }
                        862 => {
                            do_type_check =
                                (unsafe { (*p_col_1).affinity } as i32 > 65) as i32;
                            __state = 860;
                        }
                        863 => { p4 = 5; __state = 865; }
                        864 => { __state = 851; }
                        865 => {
                            if unsafe { (*p_col_1).col_flags } as i32 & 32 != 0 {
                                __state = 867;
                            } else { __state = 868; }
                        }
                        866 => {
                            label_error = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            __state = 882;
                        }
                        867 => {
                            unsafe {
                                sqlite3_expr_code_get_column_of_table(v, p_tab_10,
                                    i_data_cur, j__4, 3)
                            };
                            __state = 869;
                        }
                        868 => {
                            if unsafe { (*p_col_1).i_dflt } != 0 {
                                __state = 872;
                            } else { __state = 871; }
                        }
                        869 => { p1 = -1; __state = 870; }
                        870 => { p3 = 3; __state = 866; }
                        871 => { p1 = i_data_cur; __state = 877; }
                        872 => {
                            p_dflt_value = core::ptr::null_mut();
                            __state = 873;
                        }
                        873 => {
                            unsafe {
                                sqlite3_value_from_expr(db,
                                    unsafe { sqlite3_column_expr(p_tab_10, p_col_1) } as
                                        *const Expr, unsafe { (*db).enc },
                                    unsafe { (*p_col_1).affinity } as u8, &mut p_dflt_value)
                            };
                            __state = 874;
                        }
                        874 => {
                            if !(p_dflt_value).is_null() {
                                __state = 875;
                            } else { __state = 871; }
                        }
                        875 => {
                            p4 = unsafe { sqlite3_value_type(p_dflt_value) };
                            __state = 876;
                        }
                        876 => {
                            unsafe { sqlite3ValueFree(p_dflt_value) };
                            __state = 871;
                        }
                        877 => {
                            if !(unsafe { (*p_tab_10).tab_flags } & 128 as u32 ==
                                                0 as u32) as i32 != 0 {
                                __state = 878;
                            } else { __state = 879; }
                        }
                        878 => { __state = 880; }
                        879 => {
                            p3 =
                                unsafe {
                                        sqlite3_table_column_to_storage(p_tab_10, j__4 as i16)
                                    } as i32;
                            __state = 881;
                        }
                        880 => {
                            p3 =
                                unsafe {
                                    sqlite3_table_column_to_index(unsafe {
                                            sqlite3_primary_key_index(p_tab_10)
                                        }, j__4)
                                };
                            __state = 866;
                        }
                        881 => { __state = 866; }
                        882 => {
                            label_ok = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            __state = 883;
                        }
                        883 => {
                            if unsafe { (*p_col_1).not_null() } != 0 {
                                __state = 885;
                            } else { __state = 884; }
                        }
                        884 => {
                            if b_strict != 0 && do_type_check != 0 {
                                __state = 904;
                            } else { __state = 905; }
                        }
                        885 => { __state = 886; }
                        886 => {
                            jmp2 =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 18, p1, label_ok, p3, p4)
                                };
                            __state = 887;
                        }
                        887 => { __state = 888; }
                        888 => {
                            if p1 < 0 { __state = 890; } else { __state = 891; }
                        }
                        889 => {
                            z_err_1 =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"NULL value in %s.%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab_10).z_name },
                                        unsafe { (*p_col_1).z_cn_name })
                                };
                            __state = 897;
                        }
                        890 => {
                            unsafe { sqlite3_vdbe_change_p5(v, 15 as u16) };
                            __state = 892;
                        }
                        891 => {
                            unsafe { sqlite3_vdbe_change_p5(v, 13 as u16) };
                            __state = 893;
                        }
                        892 => { jmp3 = jmp2; __state = 889; }
                        893 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 96, p1, p3, 3) };
                            __state = 894;
                        }
                        894 => {
                            unsafe { sqlite3_column_default(v, p_tab_10, j__4, 3) };
                            __state = 895;
                        }
                        895 => {
                            jmp3 = unsafe { sqlite3_vdbe_add_op2(v, 52, 3, label_ok) };
                            __state = 896;
                        }
                        896 => { __state = 889; }
                        897 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, 3, 0, z_err_1 as *const i8,
                                    -7)
                            };
                            __state = 898;
                        }
                        898 => {
                            if do_type_check != 0 {
                                __state = 899;
                            } else { __state = 900; }
                        }
                        899 => {
                            unsafe { sqlite3_vdbe_goto(v, label_error) };
                            __state = 901;
                        }
                        900 => { __state = 884; }
                        901 => {
                            unsafe { sqlite3_vdbe_jump_here(v, jmp2) };
                            __state = 902;
                        }
                        902 => {
                            unsafe { sqlite3_vdbe_jump_here(v, jmp3) };
                            __state = 884;
                        }
                        903 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, label_error) };
                            __state = 929;
                        }
                        904 => { __state = 906; }
                        905 => {
                            if (b_strict == 0) as i32 != 0 &&
                                    unsafe { (*p_col_1).affinity } as i32 == 66 {
                                __state = 912;
                            } else { __state = 913; }
                        }
                        906 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 18, p1, label_ok, p3, p4)
                            };
                            __state = 907;
                        }
                        907 => { { let _ = 0; }; __state = 908; }
                        908 => {
                            unsafe {
                                sqlite3_vdbe_change_p5(v,
                                    a_std_type_mask[(unsafe { (*p_col_1).e_c_type() } as i32 -
                                                    1) as usize] as u16)
                            };
                            __state = 909;
                        }
                        909 => { __state = 910; }
                        910 => {
                            z_err_1 =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"non-%s value in %s.%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe {
                                            *(sqlite3_std_type.as_ptr() as
                                                        *mut *const i8).offset((unsafe { (*p_col_1).e_c_type() } as
                                                                i32 - 1) as isize)
                                        }, unsafe { (*p_tab_10).z_name },
                                        unsafe {
                                            (*unsafe {
                                                        (*p_tab_10).a_col.offset(j__4 as isize)
                                                    }).z_cn_name
                                        })
                                };
                            __state = 911;
                        }
                        911 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, 3, 0, z_err_1 as *const i8,
                                    -7)
                            };
                            __state = 903;
                        }
                        912 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 18, p1, label_ok, p3, p4)
                            };
                            __state = 914;
                        }
                        913 => {
                            if (b_strict == 0) as i32 != 0 &&
                                    unsafe { (*p_col_1).affinity } as i32 >= 67 {
                                __state = 918;
                            } else { __state = 903; }
                        }
                        914 => {
                            unsafe { sqlite3_vdbe_change_p5(v, 28 as u16) };
                            __state = 915;
                        }
                        915 => { __state = 916; }
                        916 => {
                            z_err_1 =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"NUMERIC value in %s.%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab_10).z_name },
                                        unsafe {
                                            (*unsafe {
                                                        (*p_tab_10).a_col.offset(j__4 as isize)
                                                    }).z_cn_name
                                        })
                                };
                            __state = 917;
                        }
                        917 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, 3, 0, z_err_1 as *const i8,
                                    -7)
                            };
                            __state = 903;
                        }
                        918 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 18, p1, label_ok, p3, p4)
                            };
                            __state = 919;
                        }
                        919 => {
                            unsafe { sqlite3_vdbe_change_p5(v, 27 as u16) };
                            __state = 920;
                        }
                        920 => { __state = 921; }
                        921 => {
                            if p1 >= 0 { __state = 923; } else { __state = 922; }
                        }
                        922 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 98, 3, 1, 0,
                                    c"C".as_ptr() as *mut i8 as *const i8, -1)
                            };
                            __state = 924;
                        }
                        923 => {
                            unsafe {
                                sqlite3_expr_code_get_column_of_table(v, p_tab_10,
                                    i_data_cur, j__4, 3)
                            };
                            __state = 922;
                        }
                        924 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 18, -1, label_ok, 3, p4)
                            };
                            __state = 925;
                        }
                        925 => {
                            unsafe { sqlite3_vdbe_change_p5(v, 28 as u16) };
                            __state = 926;
                        }
                        926 => { __state = 927; }
                        927 => {
                            z_err_1 =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"TEXT value in %s.%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab_10).z_name },
                                        unsafe {
                                            (*unsafe {
                                                        (*p_tab_10).a_col.offset(j__4 as isize)
                                                    }).z_cn_name
                                        })
                                };
                            __state = 928;
                        }
                        928 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, 3, 0, z_err_1 as *const i8,
                                    -7)
                            };
                            __state = 903;
                        }
                        929 => { integrity_check_result_row(v); __state = 930; }
                        930 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, label_ok) };
                            __state = 851;
                        }
                        931 => {
                            if (is_quick == 0) as i32 != 0 {
                                __state = 952;
                            } else { __state = 951; }
                        }
                        932 => {
                            p_check =
                                unsafe {
                                    sqlite3_expr_list_dup(db,
                                        unsafe { (*p_tab_10).p_check } as *const ExprList, 0)
                                };
                            __state = 933;
                        }
                        933 => {
                            if unsafe { (*db).malloc_failed } as i32 == 0 {
                                __state = 935;
                            } else { __state = 934; }
                        }
                        934 => {
                            unsafe { sqlite3_expr_list_delete(db, p_check) };
                            __state = 931;
                        }
                        935 => {
                            addr_ck_fault = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            __state = 936;
                        }
                        936 => {
                            addr_ck_ok = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            __state = 937;
                        }
                        937 => { __state = 938; }
                        938 => { __state = 939; }
                        939 => {
                            unsafe { (*p_parse).i_self_tab = i_data_cur + 1 };
                            __state = 940;
                        }
                        940 => {
                            k__3 = unsafe { (*p_check).n_expr } - 1;
                            __state = 942;
                        }
                        941 => {
                            unsafe {
                                sqlite3_expr_if_true(p_parse,
                                    unsafe {
                                        (*(unsafe { (*p_check).a.as_ptr() } as
                                                        *mut ExprList_item).offset(0 as isize)).p_expr
                                    }, addr_ck_ok, 16)
                            };
                            __state = 945;
                        }
                        942 => {
                            if k__3 > 0 { __state = 943; } else { __state = 941; }
                        }
                        943 => {
                            unsafe {
                                sqlite3_expr_if_false(p_parse,
                                    unsafe {
                                        (*(unsafe { (*p_check).a.as_ptr() } as
                                                        *mut ExprList_item).offset(k__3 as isize)).p_expr
                                    }, addr_ck_fault, 0)
                            };
                            __state = 944;
                        }
                        944 => {
                            { let __p = &mut k__3; let __t = *__p; *__p -= 1; __t };
                            __state = 942;
                        }
                        945 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, addr_ck_fault) };
                            __state = 946;
                        }
                        946 => {
                            unsafe { (*p_parse).i_self_tab = 0 };
                            __state = 947;
                        }
                        947 => {
                            z_err_2 =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"CHECK constraint failed in %s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { (*p_tab_10).z_name })
                                };
                            __state = 948;
                        }
                        948 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, 3, 0, z_err_2 as *const i8,
                                    -7)
                            };
                            __state = 949;
                        }
                        949 => { integrity_check_result_row(v); __state = 950; }
                        950 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, addr_ck_ok) };
                            __state = 934;
                        }
                        951 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 40, i_data_cur, loop_top)
                            };
                            __state = 1033;
                        }
                        952 => {
                            { j__4 = 0; p_idx_6 = unsafe { (*p_tab_10).p_index } };
                            __state = 953;
                        }
                        953 => {
                            if !(p_idx_6).is_null() {
                                __state = 954;
                            } else { __state = 951; }
                        }
                        954 => { __state = 956; }
                        955 => {
                            {
                                p_idx_6 = unsafe { (*p_idx_6).p_next };
                                { let __p = &mut j__4; let __t = *__p; *__p += 1; __t }
                            };
                            __state = 953;
                        }
                        956 => { __state = 957; }
                        957 => {
                            ck_uniq = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            __state = 958;
                        }
                        958 => {
                            if p_pk_1 == p_idx_6 {
                                __state = 960;
                            } else { __state = 959; }
                        }
                        959 => {
                            r1 =
                                unsafe {
                                    sqlite3_generate_index_key(p_parse, p_idx_6, i_data_cur, 0,
                                        0, &mut jmp3__1, p_prior, r1)
                                };
                            __state = 961;
                        }
                        960 => { __state = 955; }
                        961 => { p_prior = p_idx_6; __state = 962; }
                        962 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 88, 8 + j__4, 1) };
                            __state = 963;
                        }
                        963 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_idx_cur + j__4, ck_uniq,
                                    r1, unsafe { (*p_idx_6).n_column } as i32)
                            };
                            __state = 964;
                        }
                        964 => { __state = 965; }
                        965 => {
                            jmp2__1 =
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 47, i_idx_cur + j__4, ck_uniq, r1)
                                };
                            __state = 966;
                        }
                        966 => { __state = 967; }
                        967 => {
                            unsafe {
                                sqlite3_vdbe_change_p4(v, -1, p_idx_6 as *const i8, -6)
                            };
                            __state = 968;
                        }
                        968 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, 3, 0,
                                    unsafe {
                                            sqlite3_m_printf(db,
                                                c"index %s stores an imprecise floating-point value for row ".as_ptr()
                                                        as *mut i8 as *const i8, unsafe { (*p_idx_6).z_name })
                                        } as *const i8, -7)
                            };
                            __state = 969;
                        }
                        969 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 112, 7, 3, 3) };
                            __state = 970;
                        }
                        970 => { integrity_check_result_row(v); __state = 971; }
                        971 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 9, 0, ck_uniq) };
                            __state = 972;
                        }
                        972 => {
                            unsafe { sqlite3_vdbe_jump_here(v, jmp2__1) };
                            __state = 973;
                        }
                        973 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 3,
                                    c"row ".as_ptr() as *mut i8 as *const i8)
                            };
                            __state = 974;
                        }
                        974 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 112, 7, 3, 3) };
                            __state = 975;
                        }
                        975 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 4,
                                    c" missing from index ".as_ptr() as *mut i8 as *const i8)
                            };
                            __state = 976;
                        }
                        976 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 112, 4, 3, 3) };
                            __state = 977;
                        }
                        977 => {
                            jmp5 =
                                unsafe {
                                    sqlite3_vdbe_load_string(v, 4,
                                        unsafe { (*p_idx_6).z_name } as *const i8)
                                };
                            __state = 978;
                        }
                        978 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 112, 4, 3, 3) };
                            __state = 979;
                        }
                        979 => {
                            jmp4 = integrity_check_result_row(v);
                            __state = 980;
                        }
                        980 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, ck_uniq) };
                            __state = 981;
                        }
                        981 => {
                            if unsafe { (*p_tab_10).tab_flags } & 128 as u32 == 0 as u32
                                {
                                __state = 983;
                            } else { __state = 982; }
                        }
                        982 => { label6 = 0; __state = 992; }
                        983 => { __state = 984; }
                        984 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 144, i_idx_cur + j__4, 3)
                            };
                            __state = 985;
                        }
                        985 => {
                            jmp7 =
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 54, 3, 0,
                                        r1 + unsafe { (*p_idx_6).n_column } as i32 - 1)
                                };
                            __state = 986;
                        }
                        986 => { __state = 987; }
                        987 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 3,
                                    c"rowid not at end-of-record for row ".as_ptr() as *mut i8
                                        as *const i8)
                            };
                            __state = 988;
                        }
                        988 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 112, 7, 3, 3) };
                            __state = 989;
                        }
                        989 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 4,
                                    c" of index ".as_ptr() as *mut i8 as *const i8)
                            };
                            __state = 990;
                        }
                        990 => {
                            unsafe { sqlite3_vdbe_goto(v, jmp5 - 1) };
                            __state = 991;
                        }
                        991 => {
                            unsafe { sqlite3_vdbe_jump_here(v, jmp7) };
                            __state = 982;
                        }
                        992 => { kk = 0; __state = 994; }
                        993 => {
                            if label6 != 0 { __state = 1004; } else { __state = 1003; }
                        }
                        994 => {
                            if kk < unsafe { (*p_idx_6).n_key_col } as i32 {
                                __state = 995;
                            } else { __state = 993; }
                        }
                        995 => {
                            if unsafe {
                                        *unsafe { (*p_idx_6).az_coll.offset(kk as isize) }
                                    } == sqlite3_str_binary.as_ptr() as *const i8 {
                                __state = 998;
                            } else { __state = 997; }
                        }
                        996 => {
                            { let __p = &mut kk; let __t = *__p; *__p += 1; __t };
                            __state = 994;
                        }
                        997 => {
                            if label6 == 0 { __state = 1000; } else { __state = 999; }
                        }
                        998 => { __state = 996; }
                        999 => {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 96, i_idx_cur + j__4, kk, 3)
                            };
                            __state = 1001;
                        }
                        1000 => {
                            label6 = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            __state = 999;
                        }
                        1001 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 53, 3, label6, r1 + kk) };
                            __state = 1002;
                        }
                        1002 => { __state = 996; }
                        1003 => {
                            if unsafe { (*p_idx_6).on_error } as i32 != 0 {
                                __state = 1012;
                            } else { __state = 1011; }
                        }
                        1004 => {
                            jmp6 = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                            __state = 1005;
                        }
                        1005 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, label6) };
                            __state = 1006;
                        }
                        1006 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 3,
                                    c"row ".as_ptr() as *mut i8 as *const i8)
                            };
                            __state = 1007;
                        }
                        1007 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 112, 7, 3, 3) };
                            __state = 1008;
                        }
                        1008 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 4,
                                    c" values differ from index ".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                            __state = 1009;
                        }
                        1009 => {
                            unsafe { sqlite3_vdbe_goto(v, jmp5 - 1) };
                            __state = 1010;
                        }
                        1010 => {
                            unsafe { sqlite3_vdbe_jump_here(v, jmp6) };
                            __state = 1003;
                        }
                        1011 => {
                            unsafe { sqlite3_vdbe_jump_here(v, jmp4) };
                            __state = 1032;
                        }
                        1012 => {
                            uniq_ok = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            __state = 1013;
                        }
                        1013 => { __state = 1014; }
                        1014 => { kk = 0; __state = 1016; }
                        1015 => {
                            jmp6__1 =
                                unsafe { sqlite3_vdbe_add_op1(v, 40, i_idx_cur + j__4) };
                            __state = 1024;
                        }
                        1016 => {
                            if kk < unsafe { (*p_idx_6).n_key_col } as i32 {
                                __state = 1017;
                            } else { __state = 1015; }
                        }
                        1017 => {
                            i_col_1 =
                                unsafe {
                                        *unsafe { (*p_idx_6).ai_column.offset(kk as isize) }
                                    } as i32;
                            __state = 1019;
                        }
                        1018 => {
                            { let __p = &mut kk; let __t = *__p; *__p += 1; __t };
                            __state = 1016;
                        }
                        1019 => { { let _ = 0; }; __state = 1020; }
                        1020 => {
                            if i_col_1 >= 0 &&
                                    unsafe {
                                            (*unsafe {
                                                        (*p_tab_10).a_col.offset(i_col_1 as isize)
                                                    }).not_null()
                                        } != 0 {
                                __state = 1022;
                            } else { __state = 1021; }
                        }
                        1021 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 51, r1 + kk, uniq_ok) };
                            __state = 1023;
                        }
                        1022 => { __state = 1018; }
                        1023 => { __state = 1018; }
                        1024 => { __state = 1025; }
                        1025 => {
                            unsafe { sqlite3_vdbe_goto(v, uniq_ok) };
                            __state = 1026;
                        }
                        1026 => {
                            unsafe { sqlite3_vdbe_jump_here(v, jmp6__1) };
                            __state = 1027;
                        }
                        1027 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 42, i_idx_cur + j__4, uniq_ok,
                                    r1, unsafe { (*p_idx_6).n_key_col } as i32)
                            };
                            __state = 1028;
                        }
                        1028 => { __state = 1029; }
                        1029 => {
                            unsafe {
                                sqlite3_vdbe_load_string(v, 3,
                                    c"non-unique entry in index ".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                            __state = 1030;
                        }
                        1030 => {
                            unsafe { sqlite3_vdbe_goto(v, jmp5) };
                            __state = 1031;
                        }
                        1031 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, uniq_ok) };
                            __state = 1011;
                        }
                        1032 => {
                            unsafe { sqlite3_resolve_part_idx_label(p_parse, jmp3__1) };
                            __state = 955;
                        }
                        1033 => { __state = 1034; }
                        1034 => {
                            unsafe { sqlite3_vdbe_jump_here(v, loop_top - 1) };
                            __state = 1035;
                        }
                        1035 => {
                            if !(p_pk_1).is_null() {
                                __state = 1036;
                            } else { __state = 785; }
                        }
                        1036 => { { let _ = 0; }; __state = 1037; }
                        1037 => {
                            unsafe {
                                sqlite3_release_temp_range(p_parse, r2,
                                    unsafe { (*p_pk_1).n_key_col } as i32)
                            };
                            __state = 785;
                        }
                        1038 => {
                            if !(x__2).is_null() {
                                __state = 1039;
                            } else { __state = 690; }
                        }
                        1039 => {
                            p_tab_11 = unsafe { (*x__2).data } as *mut Table;
                            __state = 1041;
                        }
                        1040 => { x__2 = unsafe { (*x__2).next }; __state = 1038; }
                        1041 => { __state = 1042; }
                        1042 => { __state = 1043; }
                        1043 => {
                            if table_skip_integrity_check(p_tab_11 as *const Table,
                                        p_obj_tab as *const Table) != 0 {
                                __state = 1045;
                            } else { __state = 1044; }
                        }
                        1044 => {
                            if unsafe { (*p_tab_11).e_tab_type } as i32 == 0 {
                                __state = 1047;
                            } else { __state = 1046; }
                        }
                        1045 => { __state = 1040; }
                        1046 => {
                            if !(unsafe { (*p_tab_11).e_tab_type } as i32 == 1) as i32
                                    != 0 {
                                __state = 1049;
                            } else { __state = 1048; }
                        }
                        1047 => { __state = 1040; }
                        1048 => {
                            if unsafe { (*p_tab_11).n_col } as i32 <= 0 {
                                __state = 1051;
                            } else { __state = 1050; }
                        }
                        1049 => { __state = 1040; }
                        1050 => {
                            unsafe { sqlite3_view_get_column_names(p_parse, p_tab_11) };
                            __state = 1054;
                        }
                        1051 => {
                            z_mod =
                                unsafe {
                                        *unsafe { (*p_tab_11).u.vtab.az_arg.offset(0 as isize) }
                                    } as *const i8;
                            __state = 1052;
                        }
                        1052 => {
                            if unsafe {
                                        sqlite3_hash_find(unsafe { &raw mut (*db).a_module } as
                                                *const Hash, z_mod)
                                    } == core::ptr::null_mut() {
                                __state = 1053;
                            } else { __state = 1050; }
                        }
                        1053 => { __state = 1040; }
                        1054 => {
                            if unsafe { (*p_tab_11).u.vtab.p } == core::ptr::null_mut()
                                {
                                __state = 1056;
                            } else { __state = 1055; }
                        }
                        1055 => {
                            p_v_tab =
                                unsafe { (*unsafe { (*p_tab_11).u.vtab.p }).p_vtab };
                            __state = 1057;
                        }
                        1056 => { __state = 1040; }
                        1057 => {
                            if p_v_tab == core::ptr::null_mut() {
                                __state = 1059;
                            } else { __state = 1058; }
                        }
                        1058 => {
                            if unsafe { (*p_v_tab).p_module } == core::ptr::null() {
                                __state = 1061;
                            } else { __state = 1060; }
                        }
                        1059 => { __state = 1040; }
                        1060 => {
                            if (unsafe { (*unsafe { (*p_v_tab).p_module }).i_version }
                                            as i32) < 4 {
                                __state = 1063;
                            } else { __state = 1062; }
                        }
                        1061 => { __state = 1040; }
                        1062 => {
                            if !unsafe {
                                                (*unsafe { (*p_v_tab).p_module }).x_integrity.is_some()
                                            } as i32 != 0 {
                                __state = 1065;
                            } else { __state = 1064; }
                        }
                        1063 => { __state = 1040; }
                        1064 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 176, i__9, 3, is_quick) };
                            __state = 1066;
                        }
                        1065 => { __state = 1040; }
                        1066 => {
                            {
                                let __p = unsafe { &mut (*p_tab_11).n_tab_ref };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            __state = 1067;
                        }
                        1067 => {
                            unsafe {
                                sqlite3_vdbe_append_p4(v, p_tab_11 as *mut (), -17)
                            };
                            __state = 1068;
                        }
                        1068 => {
                            a1__1 = unsafe { sqlite3_vdbe_add_op1(v, 51, 3) };
                            __state = 1069;
                        }
                        1069 => { __state = 1070; }
                        1070 => { integrity_check_result_row(v); __state = 1071; }
                        1071 => {
                            unsafe { sqlite3_vdbe_jump_here(v, a1__1) };
                            __state = 1072;
                        }
                        1072 => { __state = 1040; }
                        1073 => { __state = 1074; }
                        1074 => { __state = 1075; }
                        1075 => {
                            a_op_2 =
                                unsafe {
                                    sqlite3_vdbe_add_op_list(v,
                                        (core::mem::size_of::<[VdbeOpList; 7]>() as u64 /
                                                core::mem::size_of::<VdbeOpList>() as u64) as i32,
                                        &raw const end_code[0 as usize] as *const VdbeOpList,
                                        i_ln_2)
                                };
                            __state = 1076;
                        }
                        1076 => {
                            if !(a_op_2).is_null() {
                                __state = 1078;
                            } else { __state = 1077; }
                        }
                        1077 => {
                            unsafe {
                                sqlite3_vdbe_change_p3(v, 0,
                                    unsafe { sqlite3_vdbe_current_addr(v) } - 2)
                            };
                            __state = 671;
                        }
                        1078 => {
                            unsafe { (*a_op_2.offset(0 as isize)).p2 = 1 - mx_err };
                            __state = 1079;
                        }
                        1079 => {
                            unsafe { (*a_op_2.offset(2 as isize)).p4type = -1 as i8 };
                            __state = 1080;
                        }
                        1080 => {
                            unsafe {
                                (*a_op_2.offset(2 as isize)).p4.z =
                                    c"ok".as_ptr() as *mut i8
                            };
                            __state = 1081;
                        }
                        1081 => {
                            unsafe { (*a_op_2.offset(5 as isize)).p4type = -1 as i8 };
                            __state = 1082;
                        }
                        1082 => {
                            unsafe {
                                (*a_op_2.offset(5 as isize)).p4.z =
                                    unsafe { sqlite3_err_str(11) } as *mut i8
                            };
                            __state = 1077;
                        }
                        1083 => { __state = 92; }
                        1084 => { __state = 61; }
                        1085 => { __state = 1086; }
                        1086 => {
                            if (z_right).is_null() as i32 != 0 {
                                __state = 1087;
                            } else { __state = 1088; }
                        }
                        1087 => {
                            if unsafe { sqlite3_read_schema(p_parse) } != 0 {
                                __state = 1090;
                            } else { __state = 1089; }
                        }
                        1088 => {
                            if unsafe { (*db).m_db_flags } & 64 as u32 == 0 as u32 {
                                __state = 1094;
                            } else { __state = 1084; }
                        }
                        1089 => { { let _ = 0; }; __state = 1091; }
                        1090 => { __state = 2; }
                        1091 => { { let _ = 0; }; __state = 1092; }
                        1092 => { { let _ = 0; }; __state = 1093; }
                        1093 => {
                            return_single_text(v,
                                encnames[unsafe { (*unsafe { (*p_parse).db }).enc } as
                                                usize].z_name as *const i8);
                            __state = 1084;
                        }
                        1094 => { p_enc = &encnames[0 as usize]; __state = 1096; }
                        1095 => {
                            if (unsafe { (*p_enc).z_name }).is_null() as i32 != 0 {
                                __state = 1103;
                            } else { __state = 1084; }
                        }
                        1096 => {
                            if !(unsafe { (*p_enc).z_name }).is_null() {
                                __state = 1097;
                            } else { __state = 1095; }
                        }
                        1097 => {
                            if 0 ==
                                    unsafe {
                                        sqlite3_str_i_cmp(z_right as *const i8,
                                            unsafe { (*p_enc).z_name } as *const i8)
                                    } {
                                __state = 1099;
                            } else { __state = 1098; }
                        }
                        1098 => {
                            {
                                let __p = &mut p_enc;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            __state = 1096;
                        }
                        1099 => {
                            enc =
                                if unsafe { (*p_enc).enc } != 0 {
                                        (unsafe { (*p_enc).enc }) as i32
                                    } else { 2 } as u8;
                            __state = 1100;
                        }
                        1100 => {
                            unsafe {
                                (*unsafe {
                                                    (*unsafe { (*db).a_db.offset(0 as isize) }).p_schema
                                                }).enc = enc
                            };
                            __state = 1101;
                        }
                        1101 => {
                            unsafe { sqlite3_set_text_encoding(db, enc) };
                            __state = 1102;
                        }
                        1102 => { __state = 1095; }
                        1103 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"unsupported encoding: %s".as_ptr() as *mut i8 as
                                        *const i8, z_right)
                            };
                            __state = 1084;
                        }
                        1104 => { __state = 93; }
                        1105 => { __state = 61; }
                        1106 => {
                            unsafe { sqlite3_vdbe_uses_btree(v, i_db) };
                            __state = 1107;
                        }
                        1107 => {
                            if !(z_right).is_null() &&
                                    unsafe { (*p_pragma).m_prag_flg } as i32 & 8 == 0 {
                                __state = 1108;
                            } else { __state = 1109; }
                        }
                        1108 => { __state = 1110; }
                        1109 => { __state = 1122; }
                        1110 => { __state = 1111; }
                        1111 => { __state = 1112; }
                        1112 => {
                            a_op_3 =
                                unsafe {
                                    sqlite3_vdbe_add_op_list(v,
                                        (core::mem::size_of::<[VdbeOpList; 2]>() as u64 /
                                                core::mem::size_of::<VdbeOpList>() as u64) as i32,
                                        &raw const set_cookie[0 as usize] as *const VdbeOpList, 0)
                                };
                            __state = 1113;
                        }
                        1113 => {
                            if 0 != 0 { __state = 1115; } else { __state = 1114; }
                        }
                        1114 => {
                            unsafe { (*a_op_3.offset(0 as isize)).p1 = i_db };
                            __state = 1116;
                        }
                        1115 => { __state = 61; }
                        1116 => {
                            unsafe { (*a_op_3.offset(1 as isize)).p1 = i_db };
                            __state = 1117;
                        }
                        1117 => {
                            unsafe { (*a_op_3.offset(1 as isize)).p2 = i_cookie };
                            __state = 1118;
                        }
                        1118 => {
                            unsafe {
                                (*a_op_3.offset(1 as isize)).p3 =
                                    unsafe { sqlite3_atoi(z_right as *const i8) }
                            };
                            __state = 1119;
                        }
                        1119 => {
                            unsafe { (*a_op_3.offset(1 as isize)).p5 = 1 as u16 };
                            __state = 1120;
                        }
                        1120 => {
                            if i_cookie == 1 &&
                                    unsafe { (*db).flags } & 268435456 as u64 != 0 as u64 {
                                __state = 1121;
                            } else { __state = 1105; }
                        }
                        1121 => {
                            unsafe { (*a_op_3.offset(1 as isize)).opcode = 189 as u8 };
                            __state = 1105;
                        }
                        1122 => { __state = 1123; }
                        1123 => { __state = 1124; }
                        1124 => {
                            a_op_4 =
                                unsafe {
                                    sqlite3_vdbe_add_op_list(v,
                                        (core::mem::size_of::<[VdbeOpList; 3]>() as u64 /
                                                core::mem::size_of::<VdbeOpList>() as u64) as i32,
                                        &raw const read_cookie[0 as usize] as *const VdbeOpList, 0)
                                };
                            __state = 1125;
                        }
                        1125 => {
                            if 0 != 0 { __state = 1127; } else { __state = 1126; }
                        }
                        1126 => {
                            unsafe { (*a_op_4.offset(0 as isize)).p1 = i_db };
                            __state = 1128;
                        }
                        1127 => { __state = 61; }
                        1128 => {
                            unsafe { (*a_op_4.offset(1 as isize)).p1 = i_db };
                            __state = 1129;
                        }
                        1129 => {
                            unsafe { (*a_op_4.offset(1 as isize)).p3 = i_cookie };
                            __state = 1130;
                        }
                        1130 => {
                            unsafe { sqlite3_vdbe_reusable(v) };
                            __state = 1105;
                        }
                        1131 => { __state = 94; }
                        1132 => { __state = 61; }
                        1133 => { __state = 1134; }
                        1134 => { unsafe { (*p_parse).n_mem = 1 }; __state = 1135; }
                        1135 => {
                            if {
                                        z_opt =
                                            unsafe {
                                                sqlite3_compileoption_get({
                                                        let __p = &mut i__10;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    })
                                            };
                                        z_opt
                                    } != core::ptr::null() {
                                __state = 1137;
                            } else { __state = 1136; }
                        }
                        1136 => {
                            unsafe { sqlite3_vdbe_reusable(v) };
                            __state = 1132;
                        }
                        1137 => {
                            unsafe { sqlite3_vdbe_load_string(v, 1, z_opt) };
                            __state = 1138;
                        }
                        1138 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 86, 1, 1) };
                            __state = 1135;
                        }
                        1139 => { __state = 95; }
                        1140 => { __state = 61; }
                        1141 => { e_mode_2 = 0; __state = 1142; }
                        1142 => {
                            if !(z_right).is_null() {
                                __state = 1144;
                            } else { __state = 1143; }
                        }
                        1143 => { unsafe { (*p_parse).n_mem = 3 }; __state = 1152; }
                        1144 => {
                            if unsafe {
                                        sqlite3_str_i_cmp(z_right as *const i8,
                                            c"full".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                __state = 1145;
                            } else { __state = 1146; }
                        }
                        1145 => { e_mode_2 = 1; __state = 1143; }
                        1146 => {
                            if unsafe {
                                        sqlite3_str_i_cmp(z_right as *const i8,
                                            c"restart".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                __state = 1147;
                            } else { __state = 1148; }
                        }
                        1147 => { e_mode_2 = 2; __state = 1143; }
                        1148 => {
                            if unsafe {
                                        sqlite3_str_i_cmp(z_right as *const i8,
                                            c"truncate".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                __state = 1149;
                            } else { __state = 1150; }
                        }
                        1149 => { e_mode_2 = 3; __state = 1143; }
                        1150 => {
                            if unsafe {
                                        sqlite3_str_i_cmp(z_right as *const i8,
                                            c"noop".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                __state = 1151;
                            } else { __state = 1143; }
                        }
                        1151 => { e_mode_2 = -1; __state = 1143; }
                        1152 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 3, i_bt, e_mode_2, 1) };
                            __state = 1153;
                        }
                        1153 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 86, 1, 3) };
                            __state = 1140;
                        }
                        1154 => { __state = 96; }
                        1155 => { __state = 61; }
                        1156 => {
                            return_single_int(v,
                                if unsafe { (*db).x_wal_callback } ==
                                            Some(sqlite3_wal_default_hook) {
                                        (unsafe { (*db).p_wal_arg }) as i64 as i32
                                    } else { 0 } as i64);
                            __state = 1155;
                        }
                        1157 => {
                            unsafe {
                                sqlite3_wal_autocheckpoint(db,
                                    unsafe { sqlite3_atoi(z_right as *const i8) })
                            };
                            __state = 1156;
                        }
                        1158 => { __state = 97; }
                        1159 => { __state = 98; }
                        1160 => { __state = 61; }
                        1161 => { __state = 99; }
                        1162 => { __state = 1163; }
                        1163 => { __state = 1164; }
                        1164 => { __state = 1165; }
                        1165 => { __state = 1166; }
                        1166 => { __state = 1167; }
                        1167 => { __state = 1168; }
                        1168 => { __state = 1169; }
                        1169 => { __state = 1170; }
                        1170 => { __state = 1171; }
                        1171 => { n_check = 0; __state = 1172; }
                        1172 => { n_btree = 0; __state = 1173; }
                        1173 => { __state = 1174; }
                        1174 => {
                            if !(z_right).is_null() {
                                __state = 1176;
                            } else { __state = 1177; }
                        }
                        1175 => {
                            if op_mask & 16 as u32 == 0 as u32 {
                                __state = 1181;
                            } else { __state = 1182; }
                        }
                        1176 => {
                            op_mask =
                                unsafe { sqlite3_atoi(z_right as *const i8) } as u32;
                            __state = 1178;
                        }
                        1177 => { op_mask = 65534 as u32; __state = 1175; }
                        1178 => {
                            if op_mask & 2 as u32 == 0 as u32 {
                                __state = 1179;
                            } else { __state = 1175; }
                        }
                        1179 => { __state = 61; }
                        1180 => {
                            i_tab_cur =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            __state = 1185;
                        }
                        1181 => { n_limit = 0; __state = 1180; }
                        1182 => {
                            if unsafe { (*db).n_analysis_limit } > 0 &&
                                    unsafe { (*db).n_analysis_limit } < 2000 {
                                __state = 1183;
                            } else { __state = 1184; }
                        }
                        1183 => { n_limit = 0; __state = 1180; }
                        1184 => { n_limit = 2000; __state = 1180; }
                        1185 => {
                            i_db_last =
                                if !(z_db).is_null() {
                                    i_db
                                } else { (unsafe { (*db).n_db }) - 1 };
                            __state = 1187;
                        }
                        1186 => {
                            unsafe { sqlite3_vdbe_add_op0(v, 168) };
                            __state = 1233;
                        }
                        1187 => {
                            if i_db <= i_db_last {
                                __state = 1188;
                            } else { __state = 1186; }
                        }
                        1188 => {
                            if i_db == 1 { __state = 1191; } else { __state = 1190; }
                        }
                        1189 => {
                            { let __p = &mut i_db; let __t = *__p; *__p += 1; __t };
                            __state = 1187;
                        }
                        1190 => {
                            unsafe { sqlite3_code_verify_schema(p_parse, i_db) };
                            __state = 1192;
                        }
                        1191 => { __state = 1189; }
                        1192 => {
                            p_schema =
                                unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                };
                            __state = 1193;
                        }
                        1193 => {
                            k__4 =
                                unsafe { (*unsafe { &mut (*p_schema).tbl_hash }).first };
                            __state = 1194;
                        }
                        1194 => {
                            if !(k__4).is_null() {
                                __state = 1195;
                            } else { __state = 1189; }
                        }
                        1195 => {
                            p_tab_12 = unsafe { (*k__4).data } as *mut Table;
                            __state = 1197;
                        }
                        1196 => { k__4 = unsafe { (*k__4).next }; __state = 1194; }
                        1197 => {
                            if !(unsafe { (*p_tab_12).e_tab_type } as i32 == 0) as i32
                                    != 0 {
                                __state = 1199;
                            } else { __state = 1198; }
                        }
                        1198 => {
                            if 0 ==
                                    unsafe {
                                        sqlite3_strnicmp(unsafe { (*p_tab_12).z_name } as *const i8,
                                            c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
                                    } {
                                __state = 1201;
                            } else { __state = 1200; }
                        }
                        1199 => { __state = 1196; }
                        1200 => {
                            sz_threshold = unsafe { (*p_tab_12).n_row_log_est };
                            __state = 1202;
                        }
                        1201 => { __state = 1196; }
                        1202 => { n_index = 0; __state = 1203; }
                        1203 => {
                            p_idx_7 = unsafe { (*p_tab_12).p_index };
                            __state = 1205;
                        }
                        1204 => {
                            if unsafe { (*p_tab_12).tab_flags } & 256 as u32 != 0 as u32
                                {
                                __state = 1211;
                            } else { __state = 1212; }
                        }
                        1205 => {
                            if !(p_idx_7).is_null() {
                                __state = 1206;
                            } else { __state = 1204; }
                        }
                        1206 => {
                            { let __p = &mut n_index; let __t = *__p; *__p += 1; __t };
                            __state = 1208;
                        }
                        1207 => {
                            p_idx_7 = unsafe { (*p_idx_7).p_next };
                            __state = 1205;
                        }
                        1208 => {
                            if (unsafe { (*p_idx_7).has_stat1() } == 0) as i32 != 0 {
                                __state = 1209;
                            } else { __state = 1207; }
                        }
                        1209 => { sz_threshold = -1 as LogEst; __state = 1207; }
                        1210 => {
                            { let __p = &mut n_check; let __t = *__p; *__p += 1; __t };
                            __state = 1217;
                        }
                        1211 => { __state = 1210; }
                        1212 => {
                            if op_mask & 65536 as u32 != 0 {
                                __state = 1213;
                            } else { __state = 1214; }
                        }
                        1213 => { __state = 1210; }
                        1214 => {
                            if unsafe { (*p_tab_12).p_index } != core::ptr::null_mut()
                                    && (sz_threshold as i32) < 0 {
                                __state = 1215;
                            } else { __state = 1216; }
                        }
                        1215 => { __state = 1210; }
                        1216 => { __state = 1196; }
                        1217 => {
                            if n_check == 2 { __state = 1219; } else { __state = 1218; }
                        }
                        1218 => { n_btree += n_index + 1; __state = 1220; }
                        1219 => {
                            unsafe { sqlite3_begin_write_operation(p_parse, 0, i_db) };
                            __state = 1218;
                        }
                        1220 => {
                            unsafe {
                                sqlite3_open_table(p_parse, i_tab_cur, i_db, p_tab_12, 114)
                            };
                            __state = 1221;
                        }
                        1221 => {
                            if sz_threshold as i32 >= 0 {
                                __state = 1223;
                            } else { __state = 1224; }
                        }
                        1222 => {
                            z_sub_sql =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"ANALYZE \"%w\".\"%w\"".as_ptr() as *mut i8 as *const i8,
                                        unsafe {
                                            (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                        }, unsafe { (*p_tab_12).z_name })
                                };
                            __state = 1228;
                        }
                        1223 => { i_range = 33 as LogEst; __state = 1225; }
                        1224 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 36, i_tab_cur,
                                    ((unsafe { sqlite3_vdbe_current_addr(v) } + 2) as u32 +
                                            (op_mask & 1 as u32)) as i32)
                            };
                            __state = 1227;
                        }
                        1225 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 33, i_tab_cur,
                                    ((unsafe { sqlite3_vdbe_current_addr(v) } + 2) as u32 +
                                            (op_mask & 1 as u32)) as i32,
                                    if sz_threshold as i32 >= i_range as i32 {
                                        sz_threshold as i32 - i_range as i32
                                    } else { -1 }, sz_threshold as i32 + i_range as i32)
                            };
                            __state = 1226;
                        }
                        1226 => { __state = 1222; }
                        1227 => { __state = 1222; }
                        1228 => {
                            if op_mask & 1 as u32 != 0 {
                                __state = 1229;
                            } else { __state = 1230; }
                        }
                        1229 => {
                            r1__1 = unsafe { sqlite3_get_temp_reg(p_parse) };
                            __state = 1231;
                        }
                        1230 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 150,
                                    if n_limit != 0 { 2 } else { 0 }, n_limit, 0,
                                    z_sub_sql as *const i8, -7)
                            };
                            __state = 1196;
                        }
                        1231 => {
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 118, 0, r1__1, 0,
                                    z_sub_sql as *const i8, -7)
                            };
                            __state = 1232;
                        }
                        1232 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 86, r1__1, 1) };
                            __state = 1196;
                        }
                        1233 => {
                            if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 &&
                                        n_limit > 0 && n_btree > 100 {
                                __state = 1235;
                            } else { __state = 1234; }
                        }
                        1234 => { __state = 61; }
                        1235 => { __state = 1236; }
                        1236 => { __state = 1237; }
                        1237 => {
                            n_limit = 100 * n_limit / n_btree;
                            __state = 1238;
                        }
                        1238 => {
                            if n_limit < 100 {
                                __state = 1240;
                            } else { __state = 1239; }
                        }
                        1239 => {
                            a_op_5 = unsafe { sqlite3_vdbe_get_op(v, 0) };
                            __state = 1241;
                        }
                        1240 => { n_limit = 100; __state = 1239; }
                        1241 => {
                            i_end = unsafe { sqlite3_vdbe_current_addr(v) };
                            __state = 1242;
                        }
                        1242 => { i_addr_1 = 0; __state = 1243; }
                        1243 => {
                            if i_addr_1 < i_end {
                                __state = 1244;
                            } else { __state = 1234; }
                        }
                        1244 => {
                            if unsafe { (*a_op_5.offset(i_addr_1 as isize)).opcode } as
                                        i32 == 150 {
                                __state = 1246;
                            } else { __state = 1245; }
                        }
                        1245 => {
                            { let __p = &mut i_addr_1; let __t = *__p; *__p += 1; __t };
                            __state = 1243;
                        }
                        1246 => {
                            unsafe { (*a_op_5.offset(i_addr_1 as isize)).p2 = n_limit };
                            __state = 1245;
                        }
                        1247 => { __state = 100; }
                        1248 => {
                            if !(z_right).is_null() {
                                __state = 1250;
                            } else { __state = 1249; }
                        }
                        1249 => {
                            return_single_int(v, unsafe { (*db).busy_timeout } as i64);
                            __state = 1251;
                        }
                        1250 => {
                            unsafe {
                                sqlite3_busy_timeout(db,
                                    unsafe { sqlite3_atoi(z_right as *const i8) })
                            };
                            __state = 1249;
                        }
                        1251 => { __state = 61; }
                        1252 => { __state = 101; }
                        1253 => {
                            if !(z_right).is_null() &&
                                    unsafe {
                                            sqlite3_dec_or_hex_to_i64(z_right as *const i8, &mut n__1)
                                        } == 0 {
                                __state = 1255;
                            } else { __state = 1254; }
                        }
                        1254 => {
                            return_single_int(v,
                                unsafe { sqlite3_soft_heap_limit64(-1 as sqlite3_int64) });
                            __state = 1256;
                        }
                        1255 => {
                            unsafe { sqlite3_soft_heap_limit64(n__1) };
                            __state = 1254;
                        }
                        1256 => { __state = 61; }
                        1257 => { __state = 102; }
                        1258 => {
                            if !(z_right).is_null() &&
                                    unsafe {
                                            sqlite3_dec_or_hex_to_i64(z_right as *const i8, &mut n_1)
                                        } == 0 {
                                __state = 1260;
                            } else { __state = 1259; }
                        }
                        1259 => {
                            return_single_int(v,
                                unsafe { sqlite3_hard_heap_limit64(-1 as sqlite3_int64) });
                            __state = 1263;
                        }
                        1260 => {
                            i_prior =
                                unsafe { sqlite3_hard_heap_limit64(-1 as sqlite3_int64) };
                            __state = 1261;
                        }
                        1261 => {
                            if n_1 > 0 as i64 && (i_prior == 0 as i64 || i_prior > n_1)
                                {
                                __state = 1262;
                            } else { __state = 1259; }
                        }
                        1262 => {
                            unsafe { sqlite3_hard_heap_limit64(n_1) };
                            __state = 1259;
                        }
                        1263 => { __state = 61; }
                        1264 => { __state = 103; }
                        1265 => {
                            if !(z_right).is_null() &&
                                        unsafe {
                                                sqlite3_dec_or_hex_to_i64(z_right as *const i8, &mut n_2)
                                            } == 0 && n_2 >= 0 as i64 {
                                __state = 1267;
                            } else { __state = 1266; }
                        }
                        1266 => {
                            return_single_int(v,
                                unsafe { sqlite3_limit(db, 11, -1) } as i64);
                            __state = 1268;
                        }
                        1267 => {
                            unsafe {
                                sqlite3_limit(db, 11,
                                    (n_2 & 2147483647 as sqlite3_int64) as i32)
                            };
                            __state = 1266;
                        }
                        1268 => { __state = 61; }
                        1269 => {
                            if !(z_right).is_null() &&
                                        unsafe {
                                                sqlite3_dec_or_hex_to_i64(z_right as *const i8, &mut n_3)
                                            } == 0 && n_3 >= 0 as i64 {
                                __state = 1271;
                            } else { __state = 1270; }
                        }
                        1270 => {
                            return_single_int(v,
                                unsafe { (*db).n_analysis_limit } as i64);
                            __state = 1272;
                        }
                        1271 => {
                            unsafe {
                                (*db).n_analysis_limit =
                                    (n_3 & 2147483647 as sqlite3_int64) as i32
                            };
                            __state = 1270;
                        }
                        1272 => { __state = 61; }
                        1273 => { __state = 2; }
                        1274 => { __state = 1273; }
                        1275 => {
                            unsafe { sqlite3_db_free(db, z_right as *mut ()) };
                            __state = 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PragmaVtab {
    base: sqlite3_vtab,
    db: *mut sqlite3,
    p_name: *const PragmaName,
    n_hidden: u8,
    i_hidden: u8,
}
extern "C" fn pragma_vtab_connect(db: *mut sqlite3, p_aux_1: *mut (),
    argc: i32, argv: *const *const i8, pp_vtab_1: *mut *mut sqlite3_vtab,
    pz_err_1: *mut *mut i8) -> i32 {
    unsafe {
        let p_pragma: *const PragmaName = p_aux_1 as *const PragmaName;
        let mut p_tab: *mut PragmaVtab = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut c_sep: i8 = '(' as i32 as i8;
        let mut acc: StrAccum = unsafe { core::mem::zeroed() };
        let mut z_buf: [i8; 200] = [0; 200];
        { let _ = argc; };
        { let _ = argv; };
        unsafe {
            sqlite3_str_accum_init(&mut acc, core::ptr::null_mut(),
                &raw mut z_buf[0 as usize] as *mut i8,
                core::mem::size_of::<[i8; 200]>() as i32, 0)
        };
        unsafe {
            sqlite3_str_appendall(&raw mut acc as *mut sqlite3_str,
                c"CREATE TABLE x".as_ptr() as *mut i8 as *const i8)
        };
        {
            { i = 0; j = unsafe { (*p_pragma).i_prag_c_name } as i32 };
            '__b9: loop {
                if !(i < unsafe { (*p_pragma).n_prag_c_name } as i32) {
                    break '__b9;
                }
                '__c9: loop {
                    unsafe {
                        sqlite3_str_appendf(&raw mut acc as *mut sqlite3_str,
                            c"%c\"%s\"".as_ptr() as *mut i8 as *const i8, c_sep as i32,
                            prag_c_name[j as usize])
                    };
                    c_sep = ',' as i32 as i8;
                    break '__c9;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        if i == 0 {
            unsafe {
                sqlite3_str_appendf(&raw mut acc as *mut sqlite3_str,
                    c"(\"%s\"".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p_pragma).z_name })
            };
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
        j = 0;
        if unsafe { (*p_pragma).m_prag_flg } as i32 & 32 != 0 {
            unsafe {
                sqlite3_str_appendall(&raw mut acc as *mut sqlite3_str,
                    c",arg HIDDEN".as_ptr() as *mut i8 as *const i8)
            };
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
        if unsafe { (*p_pragma).m_prag_flg } as i32 & (64 | 128) != 0 {
            unsafe {
                sqlite3_str_appendall(&raw mut acc as *mut sqlite3_str,
                    c",schema HIDDEN".as_ptr() as *mut i8 as *const i8)
            };
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
        unsafe {
            sqlite3_str_append(&raw mut acc as *mut sqlite3_str,
                c")".as_ptr() as *mut i8 as *const i8, 1)
        };
        unsafe { sqlite3_str_accum_finish(&mut acc) };
        { let _ = 0; };
        rc =
            unsafe {
                sqlite3_declare_vtab(db,
                    &raw mut z_buf[0 as usize] as *mut i8 as *const i8)
            };
        if rc == 0 {
            p_tab =
                unsafe {
                        sqlite3_malloc(core::mem::size_of::<PragmaVtab>() as i32)
                    } as *mut PragmaVtab;
            if p_tab == core::ptr::null_mut() {
                rc = 7;
            } else {
                unsafe {
                    memset(p_tab as *mut (), 0,
                        core::mem::size_of::<PragmaVtab>() as u64)
                };
                unsafe { (*p_tab).p_name = p_pragma };
                unsafe { (*p_tab).db = db };
                unsafe { (*p_tab).i_hidden = i as u8 };
                unsafe { (*p_tab).n_hidden = j as u8 };
            }
        } else {
            unsafe {
                *pz_err_1 =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errmsg(db) })
                    }
            };
        }
        unsafe { *pp_vtab_1 = p_tab as *mut sqlite3_vtab };
        return rc;
    }
}
extern "C" fn pragma_vtab_best_index(tab: *mut sqlite3_vtab,
    p_idx_info_1: *mut sqlite3_index_info) -> i32 {
    let p_tab: *const PragmaVtab =
        tab as *mut PragmaVtab as *const PragmaVtab;
    let mut p_constraint: *const sqlite3_index_constraint = core::ptr::null();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut seen: [i32; 2] = [0; 2];
    unsafe { (*p_idx_info_1).estimated_cost = 1 as f64 };
    if unsafe { (*p_tab).n_hidden } as i32 == 0 { return 0; }
    p_constraint =
        unsafe { (*p_idx_info_1).a_constraint } as
            *const sqlite3_index_constraint;
    seen[0 as usize] = 0;
    seen[1 as usize] = 0;
    {
        i = 0;
        '__b10: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b10;
            }
            '__c10: loop {
                if (unsafe { (*p_constraint).i_column } as i32) <
                        unsafe { (*p_tab).i_hidden } as i32 {
                    break '__c10;
                }
                if unsafe { (*p_constraint).op } as i32 != 2 { break '__c10; }
                if unsafe { (*p_constraint).usable } as i32 == 0 {
                    return 19;
                }
                j =
                    unsafe { (*p_constraint).i_column } -
                        unsafe { (*p_tab).i_hidden } as i32;
                { let _ = 0; };
                seen[j as usize] = i + 1;
                break '__c10;
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
    if seen[0 as usize] == 0 {
        unsafe { (*p_idx_info_1).estimated_cost = 2147483647 as f64 };
        unsafe {
            (*p_idx_info_1).estimated_rows = 2147483647 as sqlite3_int64
        };
        return 0;
    }
    j = seen[0 as usize] - 1;
    unsafe {
        (*unsafe {
                        (*p_idx_info_1).a_constraint_usage.offset(j as isize)
                    }).argv_index = 1
    };
    unsafe {
        (*unsafe {
                        (*p_idx_info_1).a_constraint_usage.offset(j as isize)
                    }).omit = 1 as u8
    };
    unsafe { (*p_idx_info_1).estimated_cost = 20 as f64 };
    unsafe { (*p_idx_info_1).estimated_rows = 20 as sqlite3_int64 };
    if seen[1 as usize] != 0 {
        j = seen[1 as usize] - 1;
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(j as isize)
                        }).argv_index = 2
        };
        unsafe {
            (*unsafe {
                            (*p_idx_info_1).a_constraint_usage.offset(j as isize)
                        }).omit = 1 as u8
        };
    }
    return 0;
}
extern "C" fn pragma_vtab_disconnect(p_vtab_1: *mut sqlite3_vtab) -> i32 {
    let p_tab: *mut PragmaVtab = p_vtab_1 as *mut PragmaVtab;
    unsafe { sqlite3_free(p_tab as *mut ()) };
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PragmaVtabCursor {
    base: sqlite3_vtab_cursor,
    p_pragma: *mut sqlite3_stmt,
    i_rowid: sqlite_int64,
    az_arg: [*mut i8; 2],
}
extern "C" fn pragma_vtab_open(p_vtab_1: *mut sqlite3_vtab,
    pp_cursor_1: *mut *mut sqlite3_vtab_cursor) -> i32 {
    unsafe {
        let mut p_csr: *mut PragmaVtabCursor = core::ptr::null_mut();
        p_csr =
            unsafe {
                    sqlite3_malloc(core::mem::size_of::<PragmaVtabCursor>() as
                            i32)
                } as *mut PragmaVtabCursor;
        if p_csr == core::ptr::null_mut() { return 7; }
        unsafe {
            memset(p_csr as *mut (), 0,
                core::mem::size_of::<PragmaVtabCursor>() as u64)
        };
        unsafe { (*p_csr).base.p_vtab = p_vtab_1 };
        unsafe { *pp_cursor_1 = unsafe { &mut (*p_csr).base } };
        return 0;
    }
}
extern "C" fn pragma_vtab_cursor_clear(p_csr_1: &mut PragmaVtabCursor) -> () {
    let mut i: i32 = 0;
    unsafe { sqlite3_finalize((*p_csr_1).p_pragma) };
    (*p_csr_1).p_pragma = core::ptr::null_mut();
    (*p_csr_1).i_rowid = 0 as sqlite_int64;
    {
        i = 0;
        '__b11: loop {
            if !(i <
                            (core::mem::size_of::<[*mut i8; 2]>() as u64 /
                                    core::mem::size_of::<*mut i8>() as u64) as i32) {
                break '__b11;
            }
            '__c11: loop {
                unsafe {
                    sqlite3_free((*p_csr_1).az_arg[i as usize] as *mut ())
                };
                (*p_csr_1).az_arg[i as usize] = core::ptr::null_mut();
                break '__c11;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}
extern "C" fn pragma_vtab_close(cur: *mut sqlite3_vtab_cursor) -> i32 {
    let p_csr: *mut PragmaVtabCursor = cur as *mut PragmaVtabCursor;
    pragma_vtab_cursor_clear(unsafe { &mut *p_csr });
    unsafe { sqlite3_free(p_csr as *mut ()) };
    return 0;
}
extern "C" fn pragma_vtab_next(p_vtab_cursor_1: *mut sqlite3_vtab_cursor)
    -> i32 {
    let p_csr: *mut PragmaVtabCursor =
        p_vtab_cursor_1 as *mut PragmaVtabCursor;
    let mut rc: i32 = 0;
    {
        let __p = unsafe { &mut (*p_csr).i_rowid };
        let __t = *__p;
        *__p += 1;
        __t
    };
    { let _ = 0; };
    if 100 != unsafe { sqlite3_step(unsafe { (*p_csr).p_pragma }) } {
        rc = unsafe { sqlite3_finalize(unsafe { (*p_csr).p_pragma }) };
        unsafe { (*p_csr).p_pragma = core::ptr::null_mut() };
        pragma_vtab_cursor_clear(unsafe { &mut *p_csr });
    }
    return rc;
}
extern "C" fn pragma_vtab_filter(p_vtab_cursor_1: *mut sqlite3_vtab_cursor,
    idx_num_1: i32, idx_str_1: *const i8, argc: i32,
    argv: *mut *mut sqlite3_value) -> i32 {
    unsafe {
        let p_csr: *mut PragmaVtabCursor =
            p_vtab_cursor_1 as *mut PragmaVtabCursor;
        let p_tab: *mut PragmaVtab =
            unsafe { (*p_vtab_cursor_1).p_vtab } as *mut PragmaVtab;
        let mut rc: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut acc: StrAccum = unsafe { core::mem::zeroed() };
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        { let _ = idx_num_1; };
        { let _ = idx_str_1; };
        pragma_vtab_cursor_clear(unsafe { &mut *p_csr });
        j =
            if unsafe { (*unsafe { (*p_tab).p_name }).m_prag_flg } as i32 & 32
                    != 0 {
                0
            } else { 1 };
        {
            i = 0;
            '__b12: loop {
                if !(i < argc) { break '__b12; }
                '__c12: loop {
                    let z_text: *const i8 =
                        unsafe {
                                sqlite3_value_text(unsafe { *argv.offset(i as isize) })
                            } as *const i8;
                    { let _ = 0; };
                    { let _ = 0; };
                    if !(z_text).is_null() {
                        unsafe {
                            (*p_csr).az_arg[j as usize] =
                                unsafe {
                                    sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                                        z_text)
                                }
                        };
                        if unsafe { (*p_csr).az_arg[j as usize] } ==
                                core::ptr::null_mut() {
                            return 7;
                        }
                    }
                    break '__c12;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        unsafe {
            sqlite3_str_accum_init(&mut acc, core::ptr::null_mut(),
                core::ptr::null_mut(), 0,
                unsafe { (*unsafe { (*p_tab).db }).a_limit[1 as usize] })
        };
        unsafe {
            sqlite3_str_appendall(&raw mut acc as *mut sqlite3_str,
                c"PRAGMA ".as_ptr() as *mut i8 as *const i8)
        };
        if !(unsafe { (*p_csr).az_arg[1 as usize] }).is_null() {
            unsafe {
                sqlite3_str_appendf(&raw mut acc as *mut sqlite3_str,
                    c"%Q.".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p_csr).az_arg[1 as usize] })
            };
        }
        unsafe {
            sqlite3_str_appendall(&raw mut acc as *mut sqlite3_str,
                unsafe { (*unsafe { (*p_tab).p_name }).z_name })
        };
        if !(unsafe { (*p_csr).az_arg[0 as usize] }).is_null() {
            unsafe {
                sqlite3_str_appendf(&raw mut acc as *mut sqlite3_str,
                    c"=%Q".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p_csr).az_arg[0 as usize] })
            };
        }
        z_sql = unsafe { sqlite3_str_accum_finish(&mut acc) };
        if z_sql == core::ptr::null_mut() { return 7; }
        rc =
            unsafe {
                sqlite3_prepare_v2(unsafe { (*p_tab).db }, z_sql as *const i8,
                    -1, unsafe { &mut (*p_csr).p_pragma },
                    core::ptr::null_mut())
            };
        unsafe { sqlite3_free(z_sql as *mut ()) };
        if rc != 0 {
            unsafe {
                (*p_tab).base.z_err_msg =
                    unsafe {
                        sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { sqlite3_errmsg(unsafe { (*p_tab).db }) })
                    }
            };
            return rc;
        }
        return pragma_vtab_next(p_vtab_cursor_1);
    }
}
extern "C" fn pragma_vtab_eof(p_vtab_cursor_1: *mut sqlite3_vtab_cursor)
    -> i32 {
    let p_csr: *const PragmaVtabCursor =
        p_vtab_cursor_1 as *mut PragmaVtabCursor as *const PragmaVtabCursor;
    return (unsafe { (*p_csr).p_pragma } == core::ptr::null_mut()) as i32;
}
extern "C" fn pragma_vtab_column(p_vtab_cursor_1: *mut sqlite3_vtab_cursor,
    ctx: *mut sqlite3_context, i: i32) -> i32 {
    unsafe {
        let p_csr: *const PragmaVtabCursor =
            p_vtab_cursor_1 as *mut PragmaVtabCursor as
                *const PragmaVtabCursor;
        let p_tab: *const PragmaVtab =
            unsafe { (*p_vtab_cursor_1).p_vtab } as *mut PragmaVtab as
                *const PragmaVtab;
        if i < unsafe { (*p_tab).i_hidden } as i32 {
            unsafe {
                sqlite3_result_value(ctx,
                    unsafe {
                        sqlite3_column_value(unsafe { (*p_csr).p_pragma }, i)
                    })
            };
        } else {
            unsafe {
                sqlite3_result_text(ctx,
                    unsafe {
                            (*p_csr).az_arg[(i - unsafe { (*p_tab).i_hidden } as i32) as
                                    usize]
                        } as *const i8, -1,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }))
            };
        }
        return 0;
    }
}
extern "C" fn pragma_vtab_rowid(p_vtab_cursor_1: *mut sqlite3_vtab_cursor,
    p: *mut sqlite_int64) -> i32 {
    let p_csr: *const PragmaVtabCursor =
        p_vtab_cursor_1 as *mut PragmaVtabCursor as *const PragmaVtabCursor;
    unsafe { *p = unsafe { (*p_csr).i_rowid } };
    return 0;
}
static pragma_vtab_module: sqlite3_module =
    sqlite3_module {
        i_version: 0,
        x_create: None,
        x_connect: Some(pragma_vtab_connect),
        x_best_index: Some(pragma_vtab_best_index),
        x_disconnect: Some(pragma_vtab_disconnect),
        x_destroy: None,
        x_open: Some(pragma_vtab_open),
        x_close: Some(pragma_vtab_close),
        x_filter: Some(pragma_vtab_filter),
        x_next: Some(pragma_vtab_next),
        x_eof: Some(pragma_vtab_eof),
        x_column: Some(pragma_vtab_column),
        x_rowid: Some(pragma_vtab_rowid),
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
pub extern "C" fn sqlite3_pragma_vtab_register(db: *mut sqlite3,
    z_name: *const i8) -> *mut Module {
    let mut p_name: *const PragmaName = core::ptr::null();
    { let _ = 0; };
    p_name = pragma_locate(unsafe { z_name.offset(7 as isize) });
    if p_name == core::ptr::null() { return core::ptr::null_mut(); }
    if unsafe { (*p_name).m_prag_flg } as i32 & (16 | 32) == 0 {
        return core::ptr::null_mut();
    }
    { let _ = 0; };
    return unsafe {
            sqlite3_vtab_create_module(db, z_name, &pragma_vtab_module,
                p_name as *mut (), None)
        };
}
#[repr(C)]
#[derive(Copy, Clone)]
struct EncName_N7EncName {
    z_name: *mut i8,
    enc: u8,
}
static z_text_1: [i8; 25] =
    [111 as i8, 110 as i8, 111 as i8, 102 as i8, 102 as i8, 97 as i8,
            108 as i8, 115 as i8, 101 as i8, 121 as i8, 101 as i8, 115 as i8,
            116 as i8, 114 as i8, 117 as i8, 101 as i8, 120 as i8, 116 as i8,
            114 as i8, 97 as i8, 102 as i8, 117 as i8, 108 as i8, 108 as i8,
            0 as i8];
static i_offset: [u8; 8] =
    [0 as u8, 1 as u8, 2 as u8, 4 as u8, 9 as u8, 12 as u8, 15 as u8,
            20 as u8];
static i_length: [u8; 8] =
    [2 as u8, 2 as u8, 3 as u8, 5 as u8, 3 as u8, 4 as u8, 5 as u8, 4 as u8];
static i_value: [u8; 8] =
    [1 as u8, 0 as u8, 0 as u8, 0 as u8, 1 as u8, 1 as u8, 3 as u8, 2 as u8];
static mut az_mode_name: [*mut i8; 6] =
    [c"delete".as_ptr() as *mut i8, c"persist".as_ptr() as *mut i8,
            c"off".as_ptr() as *mut i8, c"truncate".as_ptr() as *mut i8,
            c"memory".as_ptr() as *mut i8, c"wal".as_ptr() as *mut i8];
static mut az_enc: [*const i8; 4] =
    [core::ptr::null(), c"utf8".as_ptr() as *const i8,
            c"utf16le".as_ptr() as *const i8,
            c"utf16be".as_ptr() as *const i8];
static i_ln: i32 = 0 as i32;
static get_cache_size: [VdbeOpList; 9] =
    [VdbeOpList { opcode: 2 as u8, p1: 0 as i8, p2: 0 as i8, p3: 0 as i8 },
            VdbeOpList {
                opcode: 101 as u8,
                p1: 0 as i8,
                p2: 1 as i8,
                p3: 3 as i8,
            },
            VdbeOpList {
                opcode: 61 as u8,
                p1: 1 as i8,
                p2: 8 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 73 as u8,
                p1: 0 as i8,
                p2: 2 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 108 as u8,
                p1: 1 as i8,
                p2: 2 as i8,
                p3: 1 as i8,
            },
            VdbeOpList {
                opcode: 61 as u8,
                p1: 1 as i8,
                p2: 8 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 73 as u8,
                p1: 0 as i8,
                p2: 1 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 189 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 86 as u8,
                p1: 1 as i8,
                p2: 1 as i8,
                p3: 0 as i8,
            }];
static i_ln_1: i32 = 0 as i32;
static set_meta6: [VdbeOpList; 5] =
    [VdbeOpList { opcode: 2 as u8, p1: 0 as i8, p2: 1 as i8, p3: 0 as i8 },
            VdbeOpList {
                opcode: 101 as u8,
                p1: 0 as i8,
                p2: 1 as i8,
                p3: 4 as i8,
            },
            VdbeOpList {
                opcode: 16 as u8,
                p1: 1 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 72 as u8,
                p1: 0 as i8,
                p2: 2 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 102 as u8,
                p1: 0 as i8,
                p2: 7 as i8,
                p3: 0 as i8,
            }];
static mut a_std_type_mask: [u8; 6] =
    [31 as u8, 24 as u8, 17 as u8, 17 as u8, 19 as u8, 20 as u8];
static i_ln_2: i32 = 0 as i32;
static end_code: [VdbeOpList; 7] =
    [VdbeOpList { opcode: 88 as u8, p1: 1 as i8, p2: 0 as i8, p3: 0 as i8 },
            VdbeOpList {
                opcode: 62 as u8,
                p1: 1 as i8,
                p2: 4 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 118 as u8,
                p1: 0 as i8,
                p2: 3 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 86 as u8,
                p1: 3 as i8,
                p2: 1 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 72 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 118 as u8,
                p1: 0 as i8,
                p2: 3 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 9 as u8,
                p1: 0 as i8,
                p2: 3 as i8,
                p3: 0 as i8,
            }];
static mut encnames: [EncName_N7EncName; 9] =
    [EncName_N7EncName { z_name: c"UTF8".as_ptr() as *mut i8, enc: 1 as u8 },
            EncName_N7EncName {
                z_name: c"UTF-8".as_ptr() as *mut i8,
                enc: 1 as u8,
            },
            EncName_N7EncName {
                z_name: c"UTF-16le".as_ptr() as *mut i8,
                enc: 2 as u8,
            },
            EncName_N7EncName {
                z_name: c"UTF-16be".as_ptr() as *mut i8,
                enc: 3 as u8,
            },
            EncName_N7EncName {
                z_name: c"UTF16le".as_ptr() as *mut i8,
                enc: 2 as u8,
            },
            EncName_N7EncName {
                z_name: c"UTF16be".as_ptr() as *mut i8,
                enc: 3 as u8,
            },
            EncName_N7EncName {
                z_name: c"UTF-16".as_ptr() as *mut i8,
                enc: 0 as u8,
            },
            EncName_N7EncName {
                z_name: c"UTF16".as_ptr() as *mut i8,
                enc: 0 as u8,
            },
            EncName_N7EncName {
                z_name: core::ptr::null_mut(),
                enc: 0 as u8,
            }];
static set_cookie: [VdbeOpList; 2] =
    [VdbeOpList { opcode: 2 as u8, p1: 0 as i8, p2: 1 as i8, p3: 0 as i8 },
            VdbeOpList {
                opcode: 102 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            }];
static read_cookie: [VdbeOpList; 3] =
    [VdbeOpList { opcode: 2 as u8, p1: 0 as i8, p2: 0 as i8, p3: 0 as i8 },
            VdbeOpList {
                opcode: 101 as u8,
                p1: 0 as i8,
                p2: 1 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 86 as u8,
                p1: 1 as i8,
                p2: 1 as i8,
                p3: 0 as i8,
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
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_two_part_name(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut *mut Token)
    -> i32;
    fn sqlite3_open_temp_database(_: *mut Parse)
    -> i32;
    fn sqlite3_name_from_token(_: *mut sqlite3, _: *const Token)
    -> *mut i8;
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_abs_int32(_: i32)
    -> i32;
    fn sqlite3_atoi(_: *const i8)
    -> i32;
    fn sqlite3_begin_write_operation(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_oom_fault(_: *mut sqlite3)
    -> *mut ();
    static sqlite3_ctype_map: [u8; 0];
    fn sqlite3_code_verify_schema(_: *mut Parse, _: i32)
    -> ();
    static sqlite3_upper_to_lower: [u8; 0];
    fn sqlite3_dec_or_hex_to_i64(_: *const i8, _: *mut i64)
    -> i32;
    fn sqlite3_get_int32(_: *const i8, _: *mut i32)
    -> i32;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_reset_all_schemas_of_connection(_: *mut sqlite3)
    -> ();
    fn sqlite3_code_verify_named_schema(_: *mut Parse, z_db_1: *const i8)
    -> ();
    fn sqlite3_locate_table(_: *mut Parse, flags: u32, _: *const i8,
    _: *const i8)
    -> *mut Table;
    fn sqlite3_primary_key_index(_: *mut Table)
    -> *mut Index;
    fn sqlite3_view_get_column_names(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_column_expr(_: *mut Table, _: *mut Column)
    -> *mut Expr;
    fn sqlite3_preferred_table_name(_: *const i8)
    -> *const i8;
    fn sqlite3_find_index(_: *mut sqlite3, _: *const i8, _: *const i8)
    -> *mut Index;
    fn sqlite3_schema_to_index(db: *mut sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_find_table(_: *mut sqlite3, _: *const i8, _: *const i8)
    -> *mut Table;
    static mut sqlite3_builtin_functions: FuncDefHash;
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_open_table(_: *mut Parse, i_cur_1: i32, i_db_1: i32,
    _: *mut Table, _: i32)
    -> ();
    fn sqlite3_fk_locate_index(_: *mut Parse, _: *mut Table, _: *mut FKey,
    _: *mut *mut Index, _: *mut *mut i32)
    -> i32;
    fn sqlite3_expr_code_get_column_of_table(_: *mut Vdbe, _: *mut Table,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_index_affinity_str(_: *mut sqlite3, _: *mut Index)
    -> *const i8;
    fn sqlite3_register_like_functions(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_open_table_and_indices(_: *mut Parse, _: *mut Table, _: i32,
    _: u8, _: i32, _: *mut u8, _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_expr_code_load_index_column(_: *mut Parse, _: *mut Index,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_value_from_expr(_: *mut sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut sqlite3_value)
    -> i32;
    fn sqlite3ValueFree(_: *mut sqlite3_value)
    -> ();
    fn sqlite3_table_column_to_index(_: *mut Index, _: i32)
    -> i32;
    fn sqlite3_table_column_to_storage(_: *mut Table, _: i16)
    -> i16;
    fn sqlite3_column_default(_: *mut Vdbe, _: *mut Table, _: i32, _: i32)
    -> ();
    static mut sqlite3_std_type: [*const i8; 0];
    fn sqlite3_expr_list_dup(_: *mut sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_expr_if_false(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_if_true(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_generate_index_key(_: *mut Parse, _: *mut Index, _: i32,
    _: i32, _: i32, _: *mut i32, _: *mut Index, _: i32)
    -> i32;
    static sqlite3_str_binary: [i8; 0];
    fn sqlite3_resolve_part_idx_label(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_err_str(_: i32)
    -> *const i8;
    fn sqlite3_set_text_encoding(db: *mut sqlite3, _: u8)
    -> ();
    fn sqlite3_wal_default_hook(_: *mut (), _: *mut sqlite3, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_vtab_create_module(_: *mut sqlite3, _: *const i8,
    _: *const sqlite3_module, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut Module;
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut sqlite3, _: *mut i8,
    _: i32, _: i32)
    -> ();
    fn sqlite3_str_accum_finish(_: *mut StrAccum)
    -> *mut i8;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_reset_one_schema(_: *mut sqlite3, _: i32)
    -> ();
    fn sqlite3_collapse_database_array(_: *mut sqlite3)
    -> ();
    fn sqlite3_commit_internal_changes(_: *mut sqlite3)
    -> ();
    fn sqlite3_column_set_expr(_: *mut Parse, _: *mut Table, _: *mut Column,
    _: *mut Expr)
    -> ();
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
    fn sqlite3_expr_code_get_column(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: i32, _: u8)
    -> i32;
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
    fn sqlite3_expr_if_false_dup(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_locate_table_item(_: *mut Parse, flags: u32, _: *mut SrcItem)
    -> *mut Table;
    fn sqlite3_unlink_and_delete_table(_: *mut sqlite3, _: i32, _: *const i8)
    -> ();
    fn sqlite3_unlink_and_delete_index(_: *mut sqlite3, _: i32, _: *const i8)
    -> ();
    fn sqlite3_vacuum(_: *mut Parse, _: *mut Token, _: *mut Expr)
    -> ();
    fn sqlite3_run_vacuum(_: *mut *mut i8, _: *mut sqlite3, _: i32,
    _: *mut sqlite3_value)
    -> i32;
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
    fn sqlite3_prng_save_state()
    -> ();
    fn sqlite3_prng_restore_state()
    -> ();
    fn sqlite3_rollback_all(_: *mut sqlite3, _: i32)
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
    fn sqlite3_get_u_int32(_: *const i8, _: *mut u32)
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
    fn sqlite3_memdb_init()
    -> i32;
    fn sqlite3_is_memdb(_: *const sqlite3_vfs)
    -> i32;
    fn sqlite3_find_coll_seq(_: *mut sqlite3, enc: u8, _: *const i8, _: i32)
    -> *mut CollSeq;
    fn sqlite3_is_binary(_: *const CollSeq)
    -> i32;
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
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
    fn sqlite3_result_int_real(_: *mut sqlite3_context)
    -> ();
    fn sqlite3_value_new(_: *mut sqlite3)
    -> *mut sqlite3_value;
    fn sqlite3_utf16to8(_: *mut sqlite3, _: *const (), _: i32, _: u8)
    -> *mut i8;
    fn sqlite3_value_apply_affinity(_: *mut sqlite3_value, _: u8, _: u8)
    -> ();
    static sqlite3_opcode_property: [u8; 0];
    static sqlite3_std_type_len: [u8; 0];
    static sqlite3_std_type_affinity: [i8; 0];
    static mut sqlite3a_l_tb: *const u8;
    static mut sqlite3a_e_qb: *const u8;
    static mut sqlite3a_g_tb: *const u8;
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
    fn sqlite3_is_like_function(_: *mut sqlite3, _: *mut Expr, _: *mut i32,
    _: *mut i8)
    -> i32;
    fn sqlite3_schema_clear(_: *mut ())
    -> ();
    fn sqlite3_schema_get(_: *mut sqlite3, _: *mut Btree)
    -> *mut Schema;
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
    fn sqlite3_checkpoint(_: *mut sqlite3, _: i32, _: i32, _: *mut i32,
    _: *mut i32)
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