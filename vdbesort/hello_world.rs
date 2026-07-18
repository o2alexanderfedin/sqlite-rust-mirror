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
struct Vdbe {
    db: *mut sqlite3,
    pp_v_prev: *mut *mut Vdbe,
    p_v_next: *mut Vdbe,
    p_parse: *mut Parse,
    n_var: ynVar,
    n_mem: i32,
    n_cursor: i32,
    cache_ctr: u32,
    pc: i32,
    rc: i32,
    n_change: i64,
    i_statement: i32,
    i_current_time: i64,
    n_fk_constraint: i64,
    n_stmt_def_cons: i64,
    n_stmt_def_imm_cons: i64,
    a_mem: *mut Mem,
    ap_arg: *mut *mut Mem,
    ap_csr: *mut *mut VdbeCursor,
    a_var: *mut Mem,
    a_op: *mut Op,
    n_op: i32,
    n_op_alloc: i32,
    a_col_name: *mut Mem,
    p_result_row: *mut Mem,
    z_err_msg: *mut i8,
    p_v_list: *mut VList,
    start_time: i64,
    n_res_column: u16,
    n_res_alloc: u16,
    error_action: u8,
    min_write_file_format: u8,
    prep_flags: u8,
    e_vdbe_state: u8,
    _bitfield_1: u32,
    btree_mask: yDbMask,
    lock_mask: yDbMask,
    a_counter: [u32; 9],
    z_sql: *mut i8,
    p_free: *mut (),
    p_frame: *mut VdbeFrame,
    p_del_frame: *mut VdbeFrame,
    n_frame: i32,
    expmask: u32,
    p_program: *mut SubProgram,
    p_aux_data: *mut AuxData,
}
impl Vdbe {
    fn expired(&self) -> i32 { ((self._bitfield_1 >> 0u32) & 0x3u32) as i32 }
    fn set_expired(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x3u32) | ((val & 0x3u32) << 0u32);
    }
    fn explain(&self) -> i32 { ((self._bitfield_1 >> 2u32) & 0x3u32) as i32 }
    fn set_explain(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x3u32 << 2u32)) | ((val & 0x3u32) << 2u32);
    }
    fn change_cnt_on(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_change_cnt_on(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
    fn uses_stmt_journal(&self) -> i32 {
        ((self._bitfield_1 >> 5u32) & 0x1u32) as i32
    }
    fn set_uses_stmt_journal(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 5u32)) | ((val & 0x1u32) << 5u32);
    }
    fn read_only(&self) -> i32 {
        ((self._bitfield_1 >> 6u32) & 0x1u32) as i32
    }
    fn set_read_only(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 6u32)) | ((val & 0x1u32) << 6u32);
    }
    fn b_is_reader(&self) -> i32 {
        ((self._bitfield_1 >> 7u32) & 0x1u32) as i32
    }
    fn set_b_is_reader(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 7u32)) | ((val & 0x1u32) << 7u32);
    }
    fn have_eqp_ops(&self) -> i32 {
        ((self._bitfield_1 >> 8u32) & 0x1u32) as i32
    }
    fn set_have_eqp_ops(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 8u32)) | ((val & 0x1u32) << 8u32);
    }
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
type LogEst = i16;
type bft = u32;
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
type Pgno = u32;
type sqlite_uint64 = u64;
type Bitmask = u64;
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
struct sqlite3_value {
    u: MemValue,
    z: *mut i8,
    n: i32,
    flags: u16,
    enc: u8,
    e_subtype: u8,
    db: *mut sqlite3,
    sz_malloc: i32,
    u_temp: u32,
    z_malloc: *mut i8,
    x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
union MemValue {
    r: f64,
    i: i64,
    n_zero: i32,
    z_p_type: *const i8,
    p_def: *mut FuncDef,
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
    p_out: *mut Mem,
    p_func: *mut FuncDef,
    p_mem: *mut Mem,
    p_vdbe: *mut Vdbe,
    i_op: i32,
    is_error: i32,
    enc: u8,
    skip_flag: u8,
    argc: u16,
    argv: [*mut sqlite3_value; 0],
}
type Mem = sqlite3_value;
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
struct VdbeCursor {
    e_cur_type: u8,
    i_db: i8,
    null_row: u8,
    deferred_moveto: u8,
    is_table: u8,
    _bitfield_1: u32,
    seek_hit: u16,
    ub: VdbeCursor_u0,
    seq_count: i64,
    cache_status: u32,
    seek_result: i32,
    p_alt_cursor: *mut VdbeCursor,
    uc: VdbeCursor_u1,
    p_key_info: *mut KeyInfo,
    i_hdr_offset: u32,
    pgno_root: Pgno,
    n_field: i16,
    n_hdr_parsed: u16,
    moveto_target: i64,
    a_offset: *mut u32,
    a_row: *const u8,
    payload_size: u32,
    sz_row: u32,
    p_cache: *mut VdbeTxtBlbCache,
    a_type: [u32; 0],
}
impl VdbeCursor {
    fn is_ephemeral(&self) -> i32 {
        ((self._bitfield_1 >> 0u32) & 0x1u32) as i32
    }
    fn set_is_ephemeral(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !0x1u32) | ((val & 0x1u32) << 0u32);
    }
    fn use_random_rowid(&self) -> i32 {
        ((self._bitfield_1 >> 1u32) & 0x1u32) as i32
    }
    fn set_use_random_rowid(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 1u32)) | ((val & 0x1u32) << 1u32);
    }
    fn is_ordered(&self) -> i32 {
        ((self._bitfield_1 >> 2u32) & 0x1u32) as i32
    }
    fn set_is_ordered(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 2u32)) | ((val & 0x1u32) << 2u32);
    }
    fn no_reuse(&self) -> i32 { ((self._bitfield_1 >> 3u32) & 0x1u32) as i32 }
    fn set_no_reuse(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 3u32)) | ((val & 0x1u32) << 3u32);
    }
    fn col_cache(&self) -> i32 {
        ((self._bitfield_1 >> 4u32) & 0x1u32) as i32
    }
    fn set_col_cache(&mut self, val: u32) {
        self._bitfield_1 =
            (self._bitfield_1 & !(0x1u32 << 4u32)) | ((val & 0x1u32) << 4u32);
    }
}
type Bool = u32;
#[repr(C)]
#[derive(Copy, Clone)]
union VdbeCursor_u0 {
    p_btx: *mut Btree,
    a_alt_map: *mut u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Btree {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
union VdbeCursor_u1 {
    p_cursor: *mut BtCursor,
    p_v_cur: *mut sqlite3_vtab_cursor,
    p_sorter: *mut VdbeSorter,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct BtCursor {
    _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VdbeSorter {
    mn_pma_size: i32,
    mx_pma_size: i32,
    mx_keysize: i32,
    pgsz: i32,
    p_reader: *mut PmaReader,
    p_merger: *mut MergeEngine,
    db: *mut sqlite3,
    p_key_info: *mut KeyInfo,
    p_unpacked: *mut UnpackedRecord,
    list: SorterList,
    i_memory: i32,
    n_memory: i32,
    b_use_pma: u8,
    b_use_threads: u8,
    i_prev: u8,
    n_task: u8,
    type_mask: u8,
    a_task: [SortSubtask; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PmaReader {
    i_read_off: i64,
    i_eof: i64,
    n_alloc: i32,
    n_key: i32,
    p_fd: *mut sqlite3_file,
    a_alloc: *mut u8,
    a_key: *mut u8,
    a_buffer: *mut u8,
    n_buffer: i32,
    a_map: *mut u8,
    p_incr: *mut IncrMerger,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IncrMerger {
    p_task: *mut SortSubtask,
    p_merger: *mut MergeEngine,
    i_start_off: i64,
    mx_sz: i32,
    b_eof: i32,
    b_use_thread: i32,
    a_file: [SorterFile; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SortSubtask {
    p_thread: *mut SQLiteThread,
    b_done: i32,
    n_pma: i32,
    p_sorter: *mut VdbeSorter,
    p_unpacked: *mut UnpackedRecord,
    list: SorterList,
    x_compare: Option<unsafe extern "C" fn(*mut SortSubtask, *mut i32,
        *const (), i32, *const (), i32) -> i32>,
    file: SorterFile,
    file2: SorterFile,
    n_spill: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SQLiteThread {
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
struct SorterList {
    p_list: *mut SorterRecord,
    a_memory: *mut u8,
    sz_pma: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SorterRecord {
    n_val: i32,
    u: SorterRecord_u0,
}
#[repr(C)]
#[derive(Copy, Clone)]
union SorterRecord_u0 {
    p_next: *mut SorterRecord,
    i_next: i32,
}
type SorterCompare =
    unsafe extern "C" fn(*mut SortSubtask, *mut i32, *const (), i32,
        *const (), i32) -> i32;
#[repr(C)]
#[derive(Copy, Clone)]
struct SorterFile {
    p_fd: *mut sqlite3_file,
    i_eof: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct MergeEngine {
    n_tree: i32,
    p_task: *mut SortSubtask,
    a_tree: *mut i32,
    a_readr: *mut PmaReader,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct VdbeTxtBlbCache {
    p_c_value: *mut i8,
    i_offset: i64,
    i_col: i32,
    cache_status: u32,
    col_cache_ctr: u32,
}
type Op = VdbeOp;
#[repr(C)]
#[derive(Copy, Clone)]
struct VdbeFrame {
    v: *mut Vdbe,
    p_parent: *mut VdbeFrame,
    a_op: *mut Op,
    a_mem: *mut Mem,
    ap_csr: *mut *mut VdbeCursor,
    a_once: *mut u8,
    token: *mut (),
    last_rowid: i64,
    p_aux_data: *mut AuxData,
    n_cursor: i32,
    pc: i32,
    n_op: i32,
    n_mem: i32,
    n_child_mem: i32,
    n_child_csr: i32,
    n_change: i64,
    n_db_change: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct AuxData {
    i_aux_op: i32,
    i_aux_arg: i32,
    p_aux: *mut (),
    x_delete_aux: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_next_aux: *mut AuxData,
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
    v: *mut Vdbe,
    p_csr: *mut VdbeCursor,
    op: i32,
    a_record: *mut u8,
    p_keyinfo: *mut KeyInfo,
    p_unpacked: *mut UnpackedRecord,
    p_new_unpacked: *mut UnpackedRecord,
    i_new_reg: i32,
    i_blob_write: i32,
    i_key1: i64,
    i_key2: i64,
    oldipk: Mem,
    a_new: *mut Mem,
    p_tab: *mut Table,
    p_pk: *mut Index,
    ap_dflt: *mut *mut sqlite3_value,
    u_key: PreUpdate_s0,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PreUpdate_s0 {
    keyinfo_space: [u8; 32],
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
struct ScanStatus {
    addr_explain: i32,
    a_addr_range: [i32; 6],
    addr_loop: i32,
    addr_visit: i32,
    i_select_id: i32,
    n_est: LogEst,
    z_name: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct DblquoteStr {
    p_next_str: *mut DblquoteStr,
    z: [i8; 8],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct ValueList {
    p_csr: *mut BtCursor,
    p_out: *mut sqlite3_value,
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_sorter_init(db: *mut sqlite3, n_field_1: i32,
    p_csr_1: &mut VdbeCursor) -> i32 {
    unsafe {
        unsafe {
            let mut pgsz: i32 = 0;
            let mut i: i32 = 0;
            let mut p_sorter: *mut VdbeSorter = core::ptr::null_mut();
            let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
            let mut sz_key_info: i32 = 0;
            let mut sz: i64 = 0 as i64;
            let mut rc: i32 = 0;
            let mut n_worker: i32 = 0;
            if unsafe { sqlite3_temp_in_memory(db as *const sqlite3) } != 0 ||
                    sqlite3Config.b_core_mutex as i32 == 0 {
                n_worker = 0;
            } else { n_worker = unsafe { (*db).a_limit[11 as usize] }; }
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            sz_key_info =
                (core::mem::offset_of!(KeyInfo, a_coll) as u64 +
                        unsafe { (*(*p_csr_1).p_key_info).n_all_field } as u64 *
                            core::mem::size_of::<*mut CollSeq>() as u64) as i32;
            sz =
                (core::mem::offset_of!(VdbeSorter, a_task) as u64 +
                        (n_worker + 1) as u64 *
                            core::mem::size_of::<SortSubtask>() as u64) as i64;
            p_sorter =
                unsafe {
                        sqlite3_db_malloc_zero(db, (sz + sz_key_info as i64) as u64)
                    } as *mut VdbeSorter;
            (*p_csr_1).uc.p_sorter = p_sorter;
            if p_sorter == core::ptr::null_mut() {
                rc = 7;
            } else {
                let p_bt: *mut Btree =
                    unsafe { (*unsafe { (*db).a_db.offset(0 as isize) }).p_bt };
                unsafe {
                    (*p_sorter).p_key_info =
                        {
                            p_key_info =
                                unsafe { (p_sorter as *mut u8).offset(sz as isize) } as
                                    *mut KeyInfo;
                            p_key_info
                        }
                };
                unsafe {
                    memcpy(p_key_info as *mut (),
                        (*p_csr_1).p_key_info as *const (), sz_key_info as u64)
                };
                unsafe { (*p_key_info).db = core::ptr::null_mut() };
                if n_field_1 != 0 && n_worker == 0 {
                    unsafe { (*p_key_info).n_key_field = n_field_1 as u16 };
                    { let _ = 0; };
                }
                { let _ = 0; };
                unsafe { sqlite3_btree_enter(p_bt) };
                unsafe {
                    (*p_sorter).pgsz =
                        {
                            pgsz = unsafe { sqlite3_btree_get_page_size(p_bt) };
                            pgsz
                        }
                };
                unsafe { sqlite3_btree_leave(p_bt) };
                unsafe { (*p_sorter).n_task = (n_worker + 1) as u8 };
                unsafe { (*p_sorter).i_prev = (n_worker - 1) as u8 };
                unsafe {
                    (*p_sorter).b_use_threads =
                        (unsafe { (*p_sorter).n_task } as i32 > 1) as u8
                };
                unsafe { (*p_sorter).db = db };
                {
                    i = 0;
                    '__b0: loop {
                        if !(i < unsafe { (*p_sorter).n_task } as i32) {
                            break '__b0;
                        }
                        '__c0: loop {
                            let p_task: *mut SortSubtask =
                                unsafe {
                                    &mut *(unsafe { (*p_sorter).a_task.as_ptr() } as
                                                    *mut SortSubtask).offset(i as isize)
                                };
                            unsafe { (*p_task).p_sorter = p_sorter };
                            break '__c0;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if (unsafe { sqlite3_temp_in_memory(db as *const sqlite3) } ==
                                0) as i32 != 0 {
                    let mut mx_cache: i64 = 0 as i64;
                    let sz_pma: u32 = sqlite3Config.sz_pma;
                    unsafe {
                        (*p_sorter).mn_pma_size = (sz_pma * pgsz as u32) as i32
                    };
                    mx_cache =
                        unsafe {
                                (*unsafe {
                                                (*unsafe { (*db).a_db.offset(0 as isize) }).p_schema
                                            }).cache_size
                            } as i64;
                    if mx_cache < 0 as i64 {
                        mx_cache = mx_cache * -1024 as i64;
                    } else { mx_cache = mx_cache * pgsz as i64; }
                    mx_cache =
                        if mx_cache < (1 << 29) as i64 {
                            mx_cache
                        } else { (1 << 29) as i64 };
                    unsafe {
                        (*p_sorter).mx_pma_size =
                            if unsafe { (*p_sorter).mn_pma_size } > mx_cache as i32 {
                                unsafe { (*p_sorter).mn_pma_size }
                            } else { mx_cache as i32 }
                    };
                    if sqlite3Config.b_small_malloc as i32 == 0 {
                        { let _ = 0; };
                        unsafe { (*p_sorter).n_memory = pgsz };
                        unsafe {
                            (*p_sorter).list.a_memory =
                                unsafe { sqlite3Malloc(pgsz as u64) } as *mut u8
                        };
                        if (unsafe { (*p_sorter).list.a_memory }).is_null() as i32
                                != 0 {
                            rc = 7;
                        }
                    }
                }
                if (unsafe { (*p_key_info).n_all_field } as i32) < 13 &&
                            (unsafe {
                                        *(unsafe { (*p_key_info).a_coll.as_ptr() } as
                                                    *mut *mut CollSeq).offset(0 as isize)
                                    } == core::ptr::null_mut() ||
                                unsafe {
                                        *(unsafe { (*p_key_info).a_coll.as_ptr() } as
                                                    *mut *mut CollSeq).offset(0 as isize)
                                    } == unsafe { (*db).p_dflt_coll }) &&
                        unsafe {
                                        *unsafe { (*p_key_info).a_sort_flags.offset(0 as isize) }
                                    } as i32 & 2 == 0 {
                    unsafe { (*p_sorter).type_mask = (1 | 2) as u8 };
                }
            }
            return rc;
        }
    }
}
extern "C" fn vdbe_sorter_join_thread(p_task_1: &mut SortSubtask) -> i32 {
    let mut rc: i32 = 0;
    if !((*p_task_1).p_thread).is_null() {
        let mut p_ret: *mut () = 1 as i64 as *mut ();
        {
            let _ =
                unsafe {
                    sqlite3_thread_join((*p_task_1).p_thread, &mut p_ret)
                };
        };
        rc = p_ret as i64 as i32;
        { let _ = 0; };
        (*p_task_1).b_done = 0;
        (*p_task_1).p_thread = core::ptr::null_mut();
    }
    return rc;
}
extern "C" fn vdbe_sorter_join_all(p_sorter_1: &mut VdbeSorter, rcin: i32)
    -> i32 {
    let mut rc: i32 = rcin;
    let mut i: i32 = 0;
    {
        i = (*p_sorter_1).n_task as i32 - 1;
        '__b1: loop {
            if !(i >= 0) { break '__b1; }
            '__c1: loop {
                let p_task: *mut SortSubtask =
                    unsafe {
                        &mut *((*p_sorter_1).a_task.as_ptr() as
                                        *mut SortSubtask).offset(i as isize)
                    };
                let rc2: i32 =
                    vdbe_sorter_join_thread(unsafe { &mut *p_task });
                if rc == 0 { rc = rc2; }
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
    return rc;
}
extern "C" fn vdbe_merge_engine_free(p_merger_1: *mut MergeEngine) -> () {
    let mut i: i32 = 0;
    if !(p_merger_1).is_null() {
        {
            i = 0;
            '__b2: loop {
                if !(i < unsafe { (*p_merger_1).n_tree }) { break '__b2; }
                '__c2: loop {
                    vdbe_pma_reader_clear(unsafe {
                            &mut *unsafe { (*p_merger_1).a_readr.offset(i as isize) }
                        });
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    unsafe { sqlite3_free(p_merger_1 as *mut ()) };
}
extern "C" fn vdbe_incr_free(p_incr_1: *mut IncrMerger) -> () {
    if !(p_incr_1).is_null() {
        if unsafe { (*p_incr_1).b_use_thread } != 0 {
            vdbe_sorter_join_thread(unsafe {
                    &mut *unsafe { (*p_incr_1).p_task }
                });
            if !(unsafe { (*p_incr_1).a_file[0 as usize].p_fd }).is_null() {
                unsafe {
                    sqlite3_os_close_free(unsafe {
                            (*p_incr_1).a_file[0 as usize].p_fd
                        })
                };
            }
            if !(unsafe { (*p_incr_1).a_file[1 as usize].p_fd }).is_null() {
                unsafe {
                    sqlite3_os_close_free(unsafe {
                            (*p_incr_1).a_file[1 as usize].p_fd
                        })
                };
            }
        }
        vdbe_merge_engine_free(unsafe { (*p_incr_1).p_merger });
        unsafe { sqlite3_free(p_incr_1 as *mut ()) };
    }
}
extern "C" fn vdbe_pma_reader_clear(p_readr_1: *mut PmaReader) -> () {
    unsafe { sqlite3_free(unsafe { (*p_readr_1).a_alloc } as *mut ()) };
    unsafe { sqlite3_free(unsafe { (*p_readr_1).a_buffer } as *mut ()) };
    if !(unsafe { (*p_readr_1).a_map }).is_null() {
        unsafe {
            sqlite3_os_unfetch(unsafe { (*p_readr_1).p_fd }, 0 as i64,
                unsafe { (*p_readr_1).a_map } as *mut ())
        };
    }
    unsafe { vdbe_incr_free(unsafe { (*p_readr_1).p_incr }) };
    unsafe {
        memset(p_readr_1 as *mut (), 0,
            core::mem::size_of::<PmaReader>() as u64)
    };
}
extern "C" fn vdbe_sorter_record_free(db: *mut sqlite3,
    p_record_1: *mut SorterRecord) -> () {
    unsafe {
        let mut p: *mut SorterRecord = core::ptr::null_mut();
        let mut p_next: *mut SorterRecord = core::ptr::null_mut();
        {
            p = p_record_1;
            '__b3: loop {
                if !(!(p).is_null()) { break '__b3; }
                '__c3: loop {
                    p_next = unsafe { (*p).u.p_next };
                    unsafe { sqlite3_db_free(db, p as *mut ()) };
                    break '__c3;
                }
                p = p_next;
            }
        }
    }
}
extern "C" fn vdbe_sort_subtask_cleanup(db: *mut sqlite3,
    p_task_1: *mut SortSubtask) -> () {
    unsafe {
        unsafe {
            sqlite3_db_free(db, unsafe { (*p_task_1).p_unpacked } as *mut ())
        };
        if !(unsafe { (*p_task_1).list.a_memory }).is_null() {
            unsafe {
                sqlite3_free(unsafe { (*p_task_1).list.a_memory } as *mut ())
            };
        } else {
            { let _ = 0; };
            vdbe_sorter_record_free(core::ptr::null_mut(),
                unsafe { (*p_task_1).list.p_list });
        }
        if !(unsafe { (*p_task_1).file.p_fd }).is_null() {
            unsafe {
                sqlite3_os_close_free(unsafe { (*p_task_1).file.p_fd })
            };
        }
        if !(unsafe { (*p_task_1).file2.p_fd }).is_null() {
            unsafe {
                sqlite3_os_close_free(unsafe { (*p_task_1).file2.p_fd })
            };
        }
        unsafe {
            memset(p_task_1 as *mut (), 0,
                core::mem::size_of::<SortSubtask>() as u64)
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_sorter_reset(db: *mut sqlite3,
    p_sorter_1: *mut VdbeSorter) -> () {
    unsafe {
        let mut i: i32 = 0;
        { let _ = vdbe_sorter_join_all(unsafe { &mut *p_sorter_1 }, 0); };
        { let _ = 0; };
        if !(unsafe { (*p_sorter_1).p_reader }).is_null() {
            vdbe_pma_reader_clear(unsafe { (*p_sorter_1).p_reader });
            unsafe {
                sqlite3_db_free(db,
                    unsafe { (*p_sorter_1).p_reader } as *mut ())
            };
            unsafe { (*p_sorter_1).p_reader = core::ptr::null_mut() };
        }
        vdbe_merge_engine_free(unsafe { (*p_sorter_1).p_merger });
        unsafe { (*p_sorter_1).p_merger = core::ptr::null_mut() };
        {
            i = 0;
            '__b4: loop {
                if !(i < unsafe { (*p_sorter_1).n_task } as i32) {
                    break '__b4;
                }
                '__c4: loop {
                    let p_task: *mut SortSubtask =
                        unsafe {
                            &mut *(unsafe { (*p_sorter_1).a_task.as_ptr() } as
                                            *mut SortSubtask).offset(i as isize)
                        };
                    vdbe_sort_subtask_cleanup(db, p_task);
                    unsafe { (*p_task).p_sorter = p_sorter_1 };
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_sorter_1).list.a_memory } == core::ptr::null_mut() {
            vdbe_sorter_record_free(core::ptr::null_mut(),
                unsafe { (*p_sorter_1).list.p_list });
        }
        unsafe { (*p_sorter_1).list.p_list = core::ptr::null_mut() };
        unsafe { (*p_sorter_1).list.sz_pma = 0 as i64 };
        unsafe { (*p_sorter_1).b_use_pma = 0 as u8 };
        unsafe { (*p_sorter_1).i_memory = 0 };
        unsafe { (*p_sorter_1).mx_keysize = 0 };
        unsafe {
            sqlite3_db_free(db,
                unsafe { (*p_sorter_1).p_unpacked } as *mut ())
        };
        unsafe { (*p_sorter_1).p_unpacked = core::ptr::null_mut() };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_sorter_close(db: *mut sqlite3,
    p_csr_1: &mut VdbeCursor) -> () {
    unsafe {
        let mut p_sorter: *mut VdbeSorter = core::ptr::null_mut();
        { let _ = 0; };
        p_sorter = (*p_csr_1).uc.p_sorter;
        if !(p_sorter).is_null() {
            let mut ii: i32 = 0;
            {
                ii = 0;
                '__b5: loop {
                    if !(ii < unsafe { (*p_sorter).n_task } as i32) {
                        break '__b5;
                    }
                    '__c5: loop {
                        unsafe {
                            (*db).n_spill +=
                                unsafe {
                                    (*(unsafe { (*p_sorter).a_task.as_ptr() } as
                                                    *mut SortSubtask).offset(ii as isize)).n_spill
                                }
                        };
                        break '__c5;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            sqlite3_vdbe_sorter_reset(db, p_sorter);
            unsafe {
                sqlite3_free(unsafe { (*p_sorter).list.a_memory } as *mut ())
            };
            unsafe { sqlite3_db_free(db, p_sorter as *mut ()) };
            (*p_csr_1).uc.p_sorter = core::ptr::null_mut();
        }
    }
}
extern "C" fn vdbe_sorter_rowkey(p_sorter_1: &VdbeSorter, pn_key_1: &mut i32)
    -> *mut () {
    unsafe {
        let mut p_key: *mut () = core::ptr::null_mut();
        if (*p_sorter_1).b_use_pma != 0 {
            let mut p_reader: *const PmaReader = core::ptr::null();
            if (*p_sorter_1).b_use_threads != 0 {
                p_reader = (*p_sorter_1).p_reader;
            } else {
                p_reader =
                    unsafe {
                        unsafe {
                            (*(*p_sorter_1).p_merger).a_readr.offset(unsafe {
                                        *unsafe {
                                                (*(*p_sorter_1).p_merger).a_tree.offset(1 as isize)
                                            }
                                    } as isize)
                        }
                    };
            }
            *pn_key_1 = unsafe { (*p_reader).n_key };
            p_key = unsafe { (*p_reader).a_key } as *mut ();
        } else {
            *pn_key_1 = unsafe { (*(*p_sorter_1).list.p_list).n_val };
            p_key =
                unsafe {
                        ((*p_sorter_1).list.p_list as
                                *mut SorterRecord).offset(1 as isize)
                    } as *mut ();
        }
        return p_key;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_sorter_rowkey(p_csr_1: &VdbeCursor,
    p_out_1: *mut Mem) -> i32 {
    unsafe {
        let mut p_sorter: *const VdbeSorter = core::ptr::null();
        let mut p_key: *mut () = core::ptr::null_mut();
        let mut n_key: i32 = 0;
        { let _ = 0; };
        p_sorter = (*p_csr_1).uc.p_sorter;
        p_key = vdbe_sorter_rowkey(unsafe { &*p_sorter }, &mut n_key);
        if unsafe { sqlite3_vdbe_mem_clear_and_resize(p_out_1, n_key) } != 0 {
            return 7;
        }
        unsafe { (*p_out_1).n = n_key };
        unsafe {
            (*p_out_1).flags =
                (unsafe { (*p_out_1).flags } as i32 & !(3519 | 1024) | 16) as
                    u16
        };
        unsafe {
            memcpy(unsafe { (*p_out_1).z } as *mut (), p_key as *const (),
                n_key as u64)
        };
        return 0;
    }
}
extern "C" fn vdbe_sorter_create_thread(p_task_1: &mut SortSubtask,
    x_task_1: Option<unsafe extern "C" fn(*mut ()) -> *mut ()>,
    p_in_1: *mut ()) -> i32 {
    { let _ = 0; };
    return unsafe {
            sqlite3_thread_create(&mut (*p_task_1).p_thread, x_task_1, p_in_1)
        };
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PmaWriter {
    e_fw_err: i32,
    a_buffer: *mut u8,
    n_buffer: i32,
    i_buf_start: i32,
    i_buf_end: i32,
    i_write_off: i64,
    p_fd: *mut sqlite3_file,
    n_pma_spill: u64,
}
extern "C" fn vdbe_pma_writer_init(p_fd_1: *mut sqlite3_file,
    p: *mut PmaWriter, n_buf_1: i32, i_start_1: i64) -> () {
    unsafe {
        memset(p as *mut (), 0, core::mem::size_of::<PmaWriter>() as u64)
    };
    unsafe {
        (*p).a_buffer = unsafe { sqlite3Malloc(n_buf_1 as u64) } as *mut u8
    };
    if (unsafe { (*p).a_buffer }).is_null() as i32 != 0 {
        unsafe { (*p).e_fw_err = 7 };
    } else {
        unsafe {
            (*p).i_buf_end =
                {
                    unsafe {
                        (*p).i_buf_start = (i_start_1 % n_buf_1 as i64) as i32
                    };
                    unsafe { (*p).i_buf_start }
                }
        };
        unsafe {
            (*p).i_write_off = i_start_1 - unsafe { (*p).i_buf_start } as i64
        };
        unsafe { (*p).n_buffer = n_buf_1 };
        unsafe { (*p).p_fd = p_fd_1 };
    }
}
extern "C" fn vdbe_pma_write_blob(p: &mut PmaWriter, mut p_data_1: &mut [u8])
    -> () {
    let mut n_rem: i32 = p_data_1.len() as i32;
    while n_rem > 0 && (*p).e_fw_err == 0 {
        let mut n_copy: i32 = n_rem;
        if n_copy > (*p).n_buffer - (*p).i_buf_end {
            n_copy = (*p).n_buffer - (*p).i_buf_end;
        }
        unsafe {
            memcpy(unsafe {
                        &raw mut *(*p).a_buffer.offset((*p).i_buf_end as isize)
                    } as *mut (),
                &raw mut p_data_1[(p_data_1.len() as i32 - n_rem) as usize] as
                    *const (), n_copy as u64)
        };
        (*p).i_buf_end += n_copy;
        if (*p).i_buf_end == (*p).n_buffer {
            (*p).e_fw_err =
                unsafe {
                    sqlite3_os_write((*p).p_fd,
                        unsafe {
                                &raw mut *(*p).a_buffer.offset((*p).i_buf_start as isize)
                            } as *const (), (*p).i_buf_end - (*p).i_buf_start,
                        (*p).i_write_off + (*p).i_buf_start as i64)
                };
            (*p).n_pma_spill += ((*p).i_buf_end - (*p).i_buf_start) as u64;
            (*p).i_buf_start = { (*p).i_buf_end = 0; (*p).i_buf_end };
            (*p).i_write_off += (*p).n_buffer as i64;
        }
        { let _ = 0; };
        n_rem -= n_copy;
    }
}
extern "C" fn vdbe_pma_write_varint(p: *mut PmaWriter, i_val_1: u64) -> () {
    let mut n_byte: i32 = 0;
    let mut a_byte: [u8; 10] = [0; 10];
    n_byte =
        unsafe {
            sqlite3_put_varint(&raw mut a_byte[0 as usize] as *mut u8,
                i_val_1)
        };
    vdbe_pma_write_blob(unsafe { &mut *p }, &mut a_byte[..n_byte as usize]);
}
extern "C" fn vdbe_merge_engine_step(p_merger_1: &MergeEngine,
    pb_eof_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;
    let i_prev: i32 = unsafe { *(*p_merger_1).a_tree.offset(1 as isize) };
    let p_task: *mut SortSubtask = (*p_merger_1).p_task;
    rc =
        vdbe_pma_reader_next(unsafe {
                &mut *(*p_merger_1).a_readr.offset(i_prev as isize)
            });
    if rc == 0 {
        let mut i: i32 = 0;
        let mut p_readr1: *mut PmaReader = core::ptr::null_mut();
        let mut p_readr2: *mut PmaReader = core::ptr::null_mut();
        let mut b_cached: i32 = 0;
        p_readr1 =
            unsafe {
                (*p_merger_1).a_readr.offset((i_prev & 65534) as isize)
            };
        p_readr2 =
            unsafe { (*p_merger_1).a_readr.offset((i_prev | 1) as isize) };
        {
            i = ((*p_merger_1).n_tree + i_prev) / 2;
            '__b7: loop {
                if !(i > 0) { break '__b7; }
                '__c7: loop {
                    let mut i_res: i32 = 0;
                    if unsafe { (*p_readr1).p_fd } == core::ptr::null_mut() {
                        i_res = 1;
                    } else if unsafe { (*p_readr2).p_fd } ==
                            core::ptr::null_mut() {
                        i_res = -1;
                    } else {
                        i_res =
                            unsafe {
                                (unsafe {
                                        (*p_task).x_compare.unwrap()
                                    })(p_task, &mut b_cached,
                                    unsafe { (*p_readr1).a_key } as *const (),
                                    unsafe { (*p_readr1).n_key },
                                    unsafe { (*p_readr2).a_key } as *const (),
                                    unsafe { (*p_readr2).n_key })
                            };
                    }
                    if i_res < 0 || i_res == 0 && p_readr1 < p_readr2 {
                        unsafe {
                            *(*p_merger_1).a_tree.offset(i as isize) =
                                unsafe { p_readr1.offset_from((*p_merger_1).a_readr) } as
                                        i64 as i32
                        };
                        p_readr2 =
                            unsafe {
                                (*p_merger_1).a_readr.offset(unsafe {
                                            *(*p_merger_1).a_tree.offset((i ^ 1) as isize)
                                        } as isize)
                            };
                        b_cached = 0;
                    } else {
                        if !(unsafe { (*p_readr1).p_fd }).is_null() {
                            b_cached = 0;
                        }
                        unsafe {
                            *(*p_merger_1).a_tree.offset(i as isize) =
                                unsafe { p_readr2.offset_from((*p_merger_1).a_readr) } as
                                        i64 as i32
                        };
                        p_readr1 =
                            unsafe {
                                (*p_merger_1).a_readr.offset(unsafe {
                                            *(*p_merger_1).a_tree.offset((i ^ 1) as isize)
                                        } as isize)
                            };
                    }
                    break '__c7;
                }
                i = i / 2;
            }
        }
        *pb_eof_1 =
            (unsafe {
                        (*(*p_merger_1).a_readr.offset(unsafe {
                                            *(*p_merger_1).a_tree.offset(1 as isize)
                                        } as isize)).p_fd
                    } == core::ptr::null_mut()) as i32;
    }
    return if rc == 0 {
            (unsafe { (*unsafe { (*p_task).p_unpacked }).err_code }) as i32
        } else { rc };
}
extern "C" fn vdbe_pma_writer_finish(p: *mut PmaWriter, pi_eof_1: &mut i64,
    pn_spill_1: &mut u64) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*p).e_fw_err } == 0 && !(unsafe { (*p).a_buffer }).is_null()
            && unsafe { (*p).i_buf_end } > unsafe { (*p).i_buf_start } {
        unsafe {
            (*p).e_fw_err =
                unsafe {
                    sqlite3_os_write(unsafe { (*p).p_fd },
                        unsafe {
                                &raw mut *unsafe {
                                            (*p).a_buffer.offset(unsafe { (*p).i_buf_start } as isize)
                                        }
                            } as *const (),
                        unsafe { (*p).i_buf_end } - unsafe { (*p).i_buf_start },
                        unsafe { (*p).i_write_off } +
                            unsafe { (*p).i_buf_start } as i64)
                }
        };
        unsafe {
            (*p).n_pma_spill +=
                (unsafe { (*p).i_buf_end } - unsafe { (*p).i_buf_start }) as
                    u64
        };
    }
    *pi_eof_1 =
        unsafe { (*p).i_write_off } + unsafe { (*p).i_buf_end } as i64;
    *pn_spill_1 += unsafe { (*p).n_pma_spill };
    unsafe { sqlite3_free(unsafe { (*p).a_buffer } as *mut ()) };
    rc = unsafe { (*p).e_fw_err };
    unsafe {
        memset(p as *mut (), 0, core::mem::size_of::<PmaWriter>() as u64)
    };
    return rc;
}
extern "C" fn vdbe_incr_populate(p_incr_1: &mut IncrMerger) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut rc2: i32 = 0;
        let i_start: i64 = (*p_incr_1).i_start_off;
        let p_out: *mut SorterFile = &mut (*p_incr_1).a_file[1 as usize];
        let p_task: *mut SortSubtask = (*p_incr_1).p_task;
        let p_merger: *const MergeEngine =
            (*p_incr_1).p_merger as *const MergeEngine;
        let mut writer: PmaWriter = unsafe { core::mem::zeroed() };
        { let _ = 0; };
        vdbe_pma_writer_init(unsafe { (*p_out).p_fd }, &mut writer,
            unsafe { (*unsafe { (*p_task).p_sorter }).pgsz }, i_start);
        while rc == 0 {
            let mut dummy: i32 = 0;
            let p_reader: *const PmaReader =
                unsafe {
                        &raw mut *unsafe {
                                    (*p_merger).a_readr.offset(unsafe {
                                                *unsafe { (*p_merger).a_tree.offset(1 as isize) }
                                            } as isize)
                                }
                    } as *const PmaReader;
            let n_key: i32 = unsafe { (*p_reader).n_key };
            let i_eof: i64 = writer.i_write_off + writer.i_buf_end as i64;
            if unsafe { (*p_reader).p_fd } == core::ptr::null_mut() { break; }
            if i_eof + n_key as i64 +
                        unsafe { sqlite3_varint_len(n_key as u64) } as i64 >
                    i_start + (*p_incr_1).mx_sz as i64 {
                break;
            }
            vdbe_pma_write_varint(&mut writer, n_key as u64);
            vdbe_pma_write_blob(&mut writer,
                unsafe {
                    let __p = unsafe { (*p_reader).a_key } as *mut u8;
                    if __p.is_null() {
                        &mut []
                    } else {
                        core::slice::from_raw_parts_mut(__p, n_key as usize)
                    }
                });
            { let _ = 0; };
            rc =
                vdbe_merge_engine_step(unsafe { &*(*p_incr_1).p_merger },
                    &mut dummy);
        }
        rc2 =
            vdbe_pma_writer_finish(&mut writer,
                unsafe { &mut (*p_out).i_eof },
                unsafe { &mut (*p_task).n_spill });
        if rc == 0 { rc = rc2; }
        return rc;
    }
}
extern "C" fn vdbe_incr_populate_thread(p_ctx_1: *mut ()) -> *mut () {
    let p_incr: *mut IncrMerger = p_ctx_1 as *mut IncrMerger;
    let p_ret: *mut () =
        vdbe_incr_populate(unsafe { &mut *p_incr }) as i64 as *mut ();
    unsafe { (*unsafe { (*p_incr).p_task }).b_done = 1 };
    return p_ret;
}
extern "C" fn vdbe_incr_bg_populate(p_incr_1: *mut IncrMerger) -> i32 {
    let p: *mut () = p_incr_1 as *mut ();
    { let _ = 0; };
    return vdbe_sorter_create_thread(unsafe {
                &mut *unsafe { (*p_incr_1).p_task }
            }, Some(vdbe_incr_populate_thread), p);
}
extern "C" fn vdbe_incr_swap(p_incr_1: *mut IncrMerger) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*p_incr_1).b_use_thread } != 0 {
        rc =
            vdbe_sorter_join_thread(unsafe {
                    &mut *unsafe { (*p_incr_1).p_task }
                });
        if rc == 0 {
            let f0: SorterFile = unsafe { (*p_incr_1).a_file[0 as usize] };
            unsafe {
                (*p_incr_1).a_file[0 as usize] =
                    unsafe { (*p_incr_1).a_file[1 as usize] }
            };
            unsafe { (*p_incr_1).a_file[1 as usize] = f0 };
        }
        if rc == 0 {
            if unsafe { (*p_incr_1).a_file[0 as usize].i_eof } ==
                    unsafe { (*p_incr_1).i_start_off } {
                unsafe { (*p_incr_1).b_eof = 1 };
            } else { rc = vdbe_incr_bg_populate(p_incr_1); }
        }
    } else {
        rc = vdbe_incr_populate(unsafe { &mut *p_incr_1 });
        unsafe {
            (*p_incr_1).a_file[0 as usize] =
                unsafe { (*p_incr_1).a_file[1 as usize] }
        };
        if unsafe { (*p_incr_1).a_file[0 as usize].i_eof } ==
                unsafe { (*p_incr_1).i_start_off } {
            unsafe { (*p_incr_1).b_eof = 1 };
        }
    }
    return rc;
}
extern "C" fn vdbe_sorter_map_file(p_task_1: &SortSubtask,
    p_file_1: &SorterFile, pp: *mut *mut u8) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if (*p_file_1).i_eof <=
                unsafe {
                        (*unsafe { (*(*p_task_1).p_sorter).db }).n_max_sorter_mmap
                    } as i64 {
            let p_fd: *mut sqlite3_file = (*p_file_1).p_fd;
            if unsafe { (*unsafe { (*p_fd).p_methods }).i_version } as i32 >=
                    3 {
                rc =
                    unsafe {
                        sqlite3_os_fetch(p_fd, 0 as i64, (*p_file_1).i_eof as i32,
                            pp as *mut *mut ())
                    };
            }
        }
        return rc;
    }
}
extern "C" fn vdbe_pma_reader_seek(p_task_1: *mut SortSubtask,
    p_readr_1: &mut PmaReader, p_file_1: *mut SorterFile, i_off_1: i64)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        { let _ = 0; };
        if unsafe { sqlite3_fault_sim(201) } != 0 { return 10 | 1 << 8; }
        if !((*p_readr_1).a_map).is_null() {
            unsafe {
                sqlite3_os_unfetch((*p_readr_1).p_fd, 0 as i64,
                    (*p_readr_1).a_map as *mut ())
            };
            (*p_readr_1).a_map = core::ptr::null_mut();
        }
        (*p_readr_1).i_read_off = i_off_1;
        (*p_readr_1).i_eof = unsafe { (*p_file_1).i_eof };
        (*p_readr_1).p_fd = unsafe { (*p_file_1).p_fd };
        rc =
            vdbe_sorter_map_file(unsafe { &*p_task_1 }, unsafe { &*p_file_1 },
                &mut (*p_readr_1).a_map);
        if rc == 0 && (*p_readr_1).a_map == core::ptr::null_mut() {
            let pgsz: i32 =
                unsafe { (*unsafe { (*p_task_1).p_sorter }).pgsz };
            let i_buf: i32 = ((*p_readr_1).i_read_off % pgsz as i64) as i32;
            if (*p_readr_1).a_buffer == core::ptr::null_mut() {
                (*p_readr_1).a_buffer =
                    unsafe { sqlite3Malloc(pgsz as u64) } as *mut u8;
                if (*p_readr_1).a_buffer == core::ptr::null_mut() { rc = 7; }
                (*p_readr_1).n_buffer = pgsz;
            }
            if rc == 0 && i_buf != 0 {
                let mut n_read: i32 = pgsz - i_buf;
                if (*p_readr_1).i_read_off + n_read as i64 >
                        (*p_readr_1).i_eof {
                    n_read =
                        ((*p_readr_1).i_eof - (*p_readr_1).i_read_off) as i32;
                }
                rc =
                    unsafe {
                        sqlite3_os_read((*p_readr_1).p_fd,
                            unsafe {
                                    &raw mut *(*p_readr_1).a_buffer.offset(i_buf as isize)
                                } as *mut (), n_read, (*p_readr_1).i_read_off)
                    };
            }
        }
        return rc;
    }
}
extern "C" fn vdbe_pma_read_blob(p: *mut PmaReader, n_byte_1: i32,
    pp_out_1: &mut *mut u8) -> i32 {
    let mut i_buf: i32 = 0;
    let mut n_avail: i32 = 0;
    if !(unsafe { (*p).a_map }).is_null() {
        *pp_out_1 =
            unsafe {
                unsafe {
                    (*p).a_map.offset(unsafe { (*p).i_read_off } as isize)
                }
            };
        unsafe { (*p).i_read_off += n_byte_1 as i64 };
        return 0;
    }
    { let _ = 0; };
    i_buf =
        (unsafe { (*p).i_read_off } % unsafe { (*p).n_buffer } as i64) as i32;
    if i_buf == 0 {
        let mut n_read: i32 = 0;
        let mut rc: i32 = 0;
        if unsafe { (*p).i_eof } - unsafe { (*p).i_read_off } >
                unsafe { (*p).n_buffer } as i64 {
            n_read = unsafe { (*p).n_buffer };
        } else {
            n_read =
                (unsafe { (*p).i_eof } - unsafe { (*p).i_read_off }) as i32;
        }
        { let _ = 0; };
        rc =
            unsafe {
                sqlite3_os_read(unsafe { (*p).p_fd },
                    unsafe { (*p).a_buffer } as *mut (), n_read,
                    unsafe { (*p).i_read_off })
            };
        { let _ = 0; };
        if rc != 0 { return rc; }
    }
    n_avail = unsafe { (*p).n_buffer } - i_buf;
    if n_byte_1 <= n_avail {
        *pp_out_1 =
            unsafe { unsafe { (*p).a_buffer.offset(i_buf as isize) } };
        unsafe { (*p).i_read_off += n_byte_1 as i64 };
    } else {
        let mut n_rem: i32 = 0;
        if unsafe { (*p).n_alloc } < n_byte_1 {
            let mut a_new: *mut u8 = core::ptr::null_mut();
            let mut n_new: sqlite3_int64 =
                if 128 as sqlite3_int64 >
                        2 as sqlite3_int64 *
                            unsafe { (*p).n_alloc } as sqlite3_int64 {
                    128 as sqlite3_int64
                } else {
                    2 as sqlite3_int64 *
                        unsafe { (*p).n_alloc } as sqlite3_int64
                };
            while n_byte_1 as sqlite3_int64 > n_new {
                n_new = n_new * 2 as sqlite3_int64;
            }
            a_new =
                unsafe {
                        sqlite3Realloc(unsafe { (*p).a_alloc } as *mut (),
                            n_new as u64)
                    } as *mut u8;
            if (a_new).is_null() as i32 != 0 { return 7; }
            unsafe { (*p).n_alloc = n_new as i32 };
            unsafe { (*p).a_alloc = a_new };
        }
        unsafe {
            memcpy(unsafe { (*p).a_alloc } as *mut (),
                unsafe {
                        &raw mut *unsafe { (*p).a_buffer.offset(i_buf as isize) }
                    } as *const (), n_avail as u64)
        };
        unsafe { (*p).i_read_off += n_avail as i64 };
        n_rem = n_byte_1 - n_avail;
        while n_rem > 0 {
            let mut rc: i32 = 0;
            let mut n_copy: i32 = 0;
            let mut a_next: *mut u8 = core::ptr::null_mut();
            n_copy = n_rem;
            if n_rem > unsafe { (*p).n_buffer } {
                n_copy = unsafe { (*p).n_buffer };
            }
            rc = vdbe_pma_read_blob(p, n_copy, &mut a_next);
            if rc != 0 { return rc; }
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                memcpy(unsafe {
                            &raw mut *unsafe {
                                        (*p).a_alloc.offset((n_byte_1 - n_rem) as isize)
                                    }
                        } as *mut (), a_next as *const (), n_copy as u64)
            };
            n_rem -= n_copy;
        }
        *pp_out_1 = unsafe { (*p).a_alloc };
    }
    return 0;
}
extern "C" fn vdbe_pma_read_varint(p: *mut PmaReader, pn_out_1: *mut u64)
    -> i32 {
    let mut i_buf: i32 = 0;
    if !(unsafe { (*p).a_map }).is_null() {
        unsafe {
            (*p).i_read_off +=
                unsafe {
                        sqlite3_get_varint(unsafe {
                                    &raw mut *unsafe {
                                                (*p).a_map.offset(unsafe { (*p).i_read_off } as isize)
                                            }
                                } as *const u8, pn_out_1)
                    } as i64
        };
    } else {
        i_buf =
            (unsafe { (*p).i_read_off } % unsafe { (*p).n_buffer } as i64) as
                i32;
        if i_buf != 0 && unsafe { (*p).n_buffer } - i_buf >= 9 {
            unsafe {
                (*p).i_read_off +=
                    unsafe {
                            sqlite3_get_varint(unsafe {
                                        &raw mut *unsafe { (*p).a_buffer.offset(i_buf as isize) }
                                    } as *const u8, pn_out_1)
                        } as i64
            };
        } else {
            let mut a_varint: [u8; 16] = [0; 16];
            let mut a: *mut u8 = core::ptr::null_mut();
            let mut i: i32 = 0;
            let mut rc: i32 = 0;
            '__b11: loop {
                '__c11: loop {
                    rc = vdbe_pma_read_blob(p, 1, &mut a);
                    if rc != 0 { return rc; }
                    a_varint[({
                                        let __p = &mut i;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } & 15) as usize] = unsafe { *a.offset(0 as isize) };
                    break '__c11;
                }
                if !(unsafe { *a.offset(0 as isize) } as i32 & 128 != 0) {
                    break '__b11;
                }
            }
            unsafe {
                sqlite3_get_varint(&raw mut a_varint[0 as usize] as *mut u8 as
                        *const u8, pn_out_1)
            };
        }
    }
    return 0;
}
extern "C" fn vdbe_pma_reader_next(p_readr_1: *mut PmaReader) -> i32 {
    let mut rc: i32 = 0;
    let mut n_rec: u64 = 0 as u64;
    if unsafe { (*p_readr_1).i_read_off } >= unsafe { (*p_readr_1).i_eof } {
        let p_incr: *mut IncrMerger = unsafe { (*p_readr_1).p_incr };
        let mut b_eof: i32 = 1;
        if !(p_incr).is_null() {
            rc = unsafe { vdbe_incr_swap(p_incr) };
            if rc == 0 && unsafe { (*p_incr).b_eof } == 0 {
                rc =
                    vdbe_pma_reader_seek(unsafe { (*p_incr).p_task },
                        unsafe { &mut *p_readr_1 },
                        unsafe { &mut (*p_incr).a_file[0 as usize] },
                        unsafe { (*p_incr).i_start_off });
                b_eof = 0;
            }
        }
        if b_eof != 0 { vdbe_pma_reader_clear(p_readr_1); return rc; }
    }
    if rc == 0 { rc = vdbe_pma_read_varint(p_readr_1, &mut n_rec); }
    if rc == 0 {
        unsafe { (*p_readr_1).n_key = n_rec as i32 };
        rc =
            vdbe_pma_read_blob(p_readr_1, n_rec as i32,
                unsafe { &mut (*p_readr_1).a_key });
    }
    return rc;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_sorter_next(db: *mut sqlite3,
    p_csr_1: &VdbeCursor) -> i32 {
    unsafe {
        let mut p_sorter: *mut VdbeSorter = core::ptr::null_mut();
        let mut rc: i32 = 0;
        { let _ = 0; };
        p_sorter = (*p_csr_1).uc.p_sorter;
        { let _ = 0; };
        if unsafe { (*p_sorter).b_use_pma } != 0 {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*p_sorter).b_use_threads } != 0 {
                rc = vdbe_pma_reader_next(unsafe { (*p_sorter).p_reader });
                if rc == 0 &&
                        unsafe { (*unsafe { (*p_sorter).p_reader }).p_fd } ==
                            core::ptr::null_mut() {
                    rc = 101;
                }
            } else {
                let mut res: i32 = 0;
                { let _ = 0; };
                { let _ = 0; };
                rc =
                    vdbe_merge_engine_step(unsafe {
                            &*unsafe { (*p_sorter).p_merger }
                        }, &mut res);
                if rc == 0 && res != 0 { rc = 101; }
            }
        } else {
            let p_free: *mut SorterRecord =
                unsafe { (*p_sorter).list.p_list };
            unsafe {
                (*p_sorter).list.p_list = unsafe { (*p_free).u.p_next }
            };
            unsafe { (*p_free).u.p_next = core::ptr::null_mut() };
            if unsafe { (*p_sorter).list.a_memory } == core::ptr::null_mut() {
                vdbe_sorter_record_free(db, p_free);
            }
            rc =
                if !(unsafe { (*p_sorter).list.p_list }).is_null() {
                    0
                } else { 101 };
        }
        return rc;
    }
}
extern "C" fn vdbe_sort_alloc_unpacked(p_task_1: &mut SortSubtask) -> i32 {
    unsafe {
        if (*p_task_1).p_unpacked == core::ptr::null_mut() {
            (*p_task_1).p_unpacked =
                unsafe {
                    sqlite3_vdbe_alloc_unpacked_record(unsafe {
                            (*(*p_task_1).p_sorter).p_key_info
                        })
                };
            if (*p_task_1).p_unpacked == core::ptr::null_mut() { return 7; }
            unsafe {
                (*(*p_task_1).p_unpacked).n_field =
                    unsafe {
                        (*unsafe { (*(*p_task_1).p_sorter).p_key_info }).n_key_field
                    }
            };
            unsafe { (*(*p_task_1).p_unpacked).err_code = 0 as u8 };
        }
        return 0;
    }
}
extern "C" fn vdbe_sorter_compare_tail(p_task_1: &SortSubtask,
    pb_key2_cached_1: &mut i32, p_key1_1: &[u8], p_key2_1: &[u8]) -> i32 {
    let r2: *mut UnpackedRecord = (*p_task_1).p_unpacked;
    if *pb_key2_cached_1 == 0 {
        unsafe {
            sqlite3_vdbe_record_unpack(p_key2_1.len() as i32,
                p_key2_1.as_ptr() as *const (), r2)
        };
        *pb_key2_cached_1 = 1;
    }
    return unsafe {
            sqlite3_vdbe_record_compare_with_skip(p_key1_1.len() as i32,
                p_key1_1.as_ptr() as *const (), r2, 1)
        };
}
extern "C" fn vdbe_sorter_compare_int(p_task_1: *mut SortSubtask,
    pb_key2_cached_1: *mut i32, p_key1_1: *const (), n_key1_1: i32,
    p_key2_1: *const (), n_key2_1: i32) -> i32 {
    unsafe {
        let p1: *const u8 = p_key1_1 as *const u8;
        let p2: *const u8 = p_key2_1 as *const u8;
        let s1: i32 = unsafe { *p1.offset(1 as isize) } as i32;
        let s2: i32 = unsafe { *p2.offset(1 as isize) } as i32;
        let v1: *const u8 =
            unsafe { &*p1.add(unsafe { *p1.offset(0 as isize) } as usize) };
        let v2: *const u8 =
            unsafe { &*p2.add(unsafe { *p2.offset(0 as isize) } as usize) };
        let mut res: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        if s1 as i32 == s2 {
            let n: u8 = a_len[s1 as usize] as u8;
            let mut i: i32 = 0;
            res = 0;
            {
                i = 0;
                '__b12: loop {
                    if !(i < n as i32) { break '__b12; }
                    '__c12: loop {
                        if {
                                    res =
                                        unsafe { *v1.offset(i as isize) } as i32 -
                                            unsafe { *v2.offset(i as isize) } as i32;
                                    res
                                } != 0 {
                            if (unsafe { *v1.offset(0 as isize) } as i32 ^
                                            unsafe { *v2.offset(0 as isize) } as i32) & 128 != 0 {
                                res =
                                    if unsafe { *v1.offset(0 as isize) } as i32 & 128 != 0 {
                                        -1
                                    } else { 1 };
                            }
                            break '__b12;
                        }
                        break '__c12;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else if s1 as i32 > 7 && s2 as i32 > 7 {
            res = s1 - s2 as i32;
        } else {
            if s2 as i32 > 7 {
                res = 1;
            } else if s1 as i32 > 7 {
                res = -1;
            } else { res = s1 - s2 as i32; }
            { let _ = 0; };
            if res > 0 {
                if unsafe { *v1 } as i32 & 128 != 0 { res = -1; }
            } else { if unsafe { *v2 } as i32 & 128 != 0 { res = 1; } }
        }
        { let _ = 0; };
        if res == 0 {
            if unsafe {
                            (*unsafe {
                                            (*unsafe { (*p_task_1).p_sorter }).p_key_info
                                        }).n_key_field
                        } as i32 > 1 {
                res =
                    vdbe_sorter_compare_tail(unsafe { &*p_task_1 },
                        unsafe { &mut *pb_key2_cached_1 },
                        unsafe {
                            let __p = p_key1_1 as *const u8 as *const u8;
                            if __p.is_null() {
                                &[]
                            } else {
                                core::slice::from_raw_parts(__p, n_key1_1 as usize)
                            }
                        },
                        unsafe {
                            let __p = p_key2_1 as *const u8 as *const u8;
                            if __p.is_null() {
                                &[]
                            } else {
                                core::slice::from_raw_parts(__p, n_key2_1 as usize)
                            }
                        });
            }
        } else if unsafe {
                    *unsafe {
                            (*unsafe {
                                                (*unsafe { (*p_task_1).p_sorter }).p_key_info
                                            }).a_sort_flags.offset(0 as isize)
                        }
                } != 0 {
            { let _ = 0; };
            res = res * -1;
        }
        return res;
    }
}
extern "C" fn vdbe_sorter_compare_text(p_task_1: *mut SortSubtask,
    pb_key2_cached_1: *mut i32, p_key1_1: *const (), n_key1_1: i32,
    p_key2_1: *const (), n_key2_1: i32) -> i32 {
    unsafe {
        let p1: *const u8 = p_key1_1 as *const u8;
        let p2: *const u8 = p_key2_1 as *const u8;
        let v1: *const u8 =
            unsafe { &*p1.add(unsafe { *p1.offset(0 as isize) } as usize) };
        let v2: *const u8 =
            unsafe { &*p2.add(unsafe { *p2.offset(0 as isize) } as usize) };
        let mut n1: i32 = 0;
        let mut n2: i32 = 0;
        let mut res: i32 = 0;
        n1 = unsafe { *unsafe { &*p1.offset(1 as isize) } } as u32 as i32;
        if n1 >= 128 {
            unsafe {
                sqlite3_get_varint32(unsafe { &*p1.offset(1 as isize) },
                    &raw mut n1 as *mut u32)
            };
        }
        n2 = unsafe { *unsafe { &*p2.offset(1 as isize) } } as u32 as i32;
        if n2 >= 128 {
            unsafe {
                sqlite3_get_varint32(unsafe { &*p2.offset(1 as isize) },
                    &raw mut n2 as *mut u32)
            };
        }
        res =
            unsafe {
                memcmp(v1 as *const (), v2 as *const (),
                    ((if n1 < n2 { n1 } else { n2 } - 13) / 2) as u64)
            };
        if res == 0 { res = n1 - n2; }
        if res == 0 {
            if unsafe {
                            (*unsafe {
                                            (*unsafe { (*p_task_1).p_sorter }).p_key_info
                                        }).n_key_field
                        } as i32 > 1 {
                res =
                    vdbe_sorter_compare_tail(unsafe { &*p_task_1 },
                        unsafe { &mut *pb_key2_cached_1 },
                        unsafe {
                            let __p = p_key1_1 as *const u8 as *const u8;
                            if __p.is_null() {
                                &[]
                            } else {
                                core::slice::from_raw_parts(__p, n_key1_1 as usize)
                            }
                        },
                        unsafe {
                            let __p = p_key2_1 as *const u8 as *const u8;
                            if __p.is_null() {
                                &[]
                            } else {
                                core::slice::from_raw_parts(__p, n_key2_1 as usize)
                            }
                        });
            }
        } else {
            { let _ = 0; };
            { let _ = 0; };
            if unsafe {
                        *unsafe {
                                (*unsafe {
                                                    (*unsafe { (*p_task_1).p_sorter }).p_key_info
                                                }).a_sort_flags.offset(0 as isize)
                            }
                    } != 0 {
                res = res * -1;
            }
        }
        return res;
    }
}
extern "C" fn vdbe_sorter_compare(p_task_1: *mut SortSubtask,
    pb_key2_cached_1: *mut i32, p_key1_1: *const (), n_key1_1: i32,
    p_key2_1: *const (), n_key2_1: i32) -> i32 {
    let r2: *mut UnpackedRecord = unsafe { (*p_task_1).p_unpacked };
    if (unsafe { *pb_key2_cached_1 } == 0) as i32 != 0 {
        unsafe { sqlite3_vdbe_record_unpack(n_key2_1, p_key2_1, r2) };
        unsafe { *pb_key2_cached_1 = 1 };
    }
    return unsafe { sqlite3_vdbe_record_compare(n_key1_1, p_key1_1, r2) };
}
extern "C" fn vdbe_sorter_get_compare(p: &VdbeSorter)
    ->
        unsafe extern "C" fn(*mut SortSubtask, *mut i32, *const (), i32,
            *const (), i32) -> i32 {
    if (*p).type_mask as i32 == 1 {
        return vdbe_sorter_compare_int;
    } else if (*p).type_mask as i32 == 2 { return vdbe_sorter_compare_text; }
    return vdbe_sorter_compare;
}
extern "C" fn vdbe_sorter_merge(p_task_1: *mut SortSubtask,
    mut p1: *mut SorterRecord, mut p2: *mut SorterRecord)
    -> *mut SorterRecord {
    unsafe {
        let mut p_final: *mut SorterRecord = core::ptr::null_mut();
        let mut pp: *mut *mut SorterRecord = &mut p_final;
        let mut b_cached: i32 = 0;
        { let _ = 0; };
        {
            '__b13: loop {
                '__c13: loop {
                    let mut res: i32 = 0;
                    res =
                        unsafe {
                            (unsafe {
                                    (*p_task_1).x_compare.unwrap()
                                })(p_task_1, &mut b_cached,
                                unsafe { (p1 as *mut SorterRecord).offset(1 as isize) } as
                                        *mut () as *const (), unsafe { (*p1).n_val },
                                unsafe { (p2 as *mut SorterRecord).offset(1 as isize) } as
                                        *mut () as *const (), unsafe { (*p2).n_val })
                        };
                    if res <= 0 {
                        unsafe { *pp = p1 };
                        pp = unsafe { &mut (*p1).u.p_next };
                        p1 = unsafe { (*p1).u.p_next };
                        if p1 == core::ptr::null_mut() {
                            unsafe { *pp = p2 };
                            break '__b13;
                        }
                    } else {
                        unsafe { *pp = p2 };
                        pp = unsafe { &mut (*p2).u.p_next };
                        p2 = unsafe { (*p2).u.p_next };
                        b_cached = 0;
                        if p2 == core::ptr::null_mut() {
                            unsafe { *pp = p1 };
                            break '__b13;
                        }
                    }
                    break '__c13;
                }
            }
        }
        return p_final;
    }
}
extern "C" fn vdbe_sorter_sort(p_task_1: *mut SortSubtask,
    p_list_1: &mut SorterList) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p: *mut SorterRecord = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut a_slot: [*mut SorterRecord; 64] = [core::ptr::null_mut(); 64];
        rc = vdbe_sort_alloc_unpacked(unsafe { &mut *p_task_1 });
        if rc != 0 { return rc; }
        p = (*p_list_1).p_list;
        unsafe {
            (*p_task_1).x_compare =
                Some(vdbe_sorter_get_compare(unsafe {
                            &*unsafe { (*p_task_1).p_sorter }
                        }))
        };
        unsafe {
            memset(&raw mut a_slot[0 as usize] as *mut *mut SorterRecord as
                    *mut (), 0,
                core::mem::size_of::<[*mut SorterRecord; 64]>() as u64)
        };
        while !(p).is_null() {
            let mut p_next: *mut SorterRecord = core::ptr::null_mut();
            if !((*p_list_1).a_memory).is_null() {
                if p as *mut u8 == (*p_list_1).a_memory {
                    p_next = core::ptr::null_mut();
                } else {
                    { let _ = 0; };
                    p_next =
                        unsafe {
                                &raw mut *(*p_list_1).a_memory.offset(unsafe {
                                                    (*p).u.i_next
                                                } as isize)
                            } as *mut SorterRecord;
                }
            } else { p_next = unsafe { (*p).u.p_next }; }
            unsafe { (*p).u.p_next = core::ptr::null_mut() };
            {
                i = 0;
                '__b15: loop {
                    if !(!(a_slot[i as usize]).is_null()) { break '__b15; }
                    '__c15: loop {
                        p = vdbe_sorter_merge(p_task_1, p, a_slot[i as usize]);
                        { let _ = 0; };
                        a_slot[i as usize] = core::ptr::null_mut();
                        break '__c15;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            a_slot[i as usize] = p;
            p = p_next;
        }
        p = core::ptr::null_mut();
        {
            i = 0;
            '__b16: loop {
                if !(i <
                                (core::mem::size_of::<[*mut SorterRecord; 64]>() as u64 /
                                        core::mem::size_of::<*mut SorterRecord>() as u64) as i32) {
                    break '__b16;
                }
                '__c16: loop {
                    if a_slot[i as usize] == core::ptr::null_mut() {
                        break '__c16;
                    }
                    p =
                        if !(p).is_null() {
                            vdbe_sorter_merge(p_task_1, p, a_slot[i as usize])
                        } else { a_slot[i as usize] };
                    break '__c16;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        (*p_list_1).p_list = p;
        { let _ = 0; };
        return unsafe { (*unsafe { (*p_task_1).p_unpacked }).err_code } as
                i32;
    }
}
extern "C" fn vdbe_sorter_extend_file(db: &sqlite3, p_fd_1: *mut sqlite3_file,
    mut n_byte_1: i64) -> () {
    if n_byte_1 <= (*db).n_max_sorter_mmap as i64 &&
            unsafe { (*unsafe { (*p_fd_1).p_methods }).i_version } as i32 >= 3
        {
        let mut p: *mut () = core::ptr::null_mut();
        let mut chunksize: i32 = 4 * 1024;
        unsafe {
            sqlite3_os_file_control_hint(p_fd_1, 6,
                &raw mut chunksize as *mut ())
        };
        unsafe {
            sqlite3_os_file_control_hint(p_fd_1, 5,
                &raw mut n_byte_1 as *mut ())
        };
        unsafe {
            sqlite3_os_fetch(p_fd_1, 0 as i64, n_byte_1 as i32, &mut p)
        };
        if !(p).is_null() {
            unsafe { sqlite3_os_unfetch(p_fd_1, 0 as i64, p) };
        }
    }
}
extern "C" fn vdbe_sorter_open_temp_file(db: *mut sqlite3, n_extend_1: i64,
    pp_fd_1: *mut *mut sqlite3_file) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { sqlite3_fault_sim(202) } != 0 { return 10 | 13 << 8; }
    rc =
        unsafe {
            sqlite3_os_open_malloc(unsafe { (*db).p_vfs }, core::ptr::null(),
                pp_fd_1, 4096 | 2 | 4 | 16 | 8, &mut rc)
        };
    if rc == 0 {
        let mut max: i64 = 2147418112 as i64;
        unsafe {
            sqlite3_os_file_control_hint(unsafe { *pp_fd_1 }, 18,
                &raw mut max as *mut ())
        };
        if n_extend_1 > 0 as i64 {
            vdbe_sorter_extend_file(unsafe { &*db }, unsafe { *pp_fd_1 },
                n_extend_1);
        }
    }
    return rc;
}
extern "C" fn vdbe_sorter_list_to_pma(p_task_1: *mut SortSubtask,
    p_list_1: *mut SorterList) -> i32 {
    unsafe {
        let db: *mut sqlite3 =
            unsafe { (*unsafe { (*p_task_1).p_sorter }).db };
        let mut rc: i32 = 0;
        let mut writer: PmaWriter = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut writer as *mut (), 0,
                core::mem::size_of::<PmaWriter>() as u64)
        };
        { let _ = 0; };
        if unsafe { (*p_task_1).file.p_fd } == core::ptr::null_mut() {
            rc =
                vdbe_sorter_open_temp_file(db, 0 as i64,
                    unsafe { &mut (*p_task_1).file.p_fd });
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
        }
        if rc == 0 {
            vdbe_sorter_extend_file(unsafe { &*db },
                unsafe { (*p_task_1).file.p_fd },
                unsafe { (*p_task_1).file.i_eof } +
                        unsafe { (*p_list_1).sz_pma } + 9 as i64);
        }
        if rc == 0 {
            rc = vdbe_sorter_sort(p_task_1, unsafe { &mut *p_list_1 });
        }
        if rc == 0 {
            let mut p: *mut SorterRecord = core::ptr::null_mut();
            let mut p_next: *mut SorterRecord = core::ptr::null_mut();
            vdbe_pma_writer_init(unsafe { (*p_task_1).file.p_fd },
                &mut writer,
                unsafe { (*unsafe { (*p_task_1).p_sorter }).pgsz },
                unsafe { (*p_task_1).file.i_eof });
            {
                let __p = unsafe { &mut (*p_task_1).n_pma };
                let __t = *__p;
                *__p += 1;
                __t
            };
            vdbe_pma_write_varint(&mut writer,
                unsafe { (*p_list_1).sz_pma } as u64);
            {
                p = unsafe { (*p_list_1).p_list };
                '__b17: loop {
                    if !(!(p).is_null()) { break '__b17; }
                    '__c17: loop {
                        p_next = unsafe { (*p).u.p_next };
                        vdbe_pma_write_varint(&mut writer,
                            unsafe { (*p).n_val } as u64);
                        vdbe_pma_write_blob(&mut writer,
                            unsafe {
                                let __p =
                                    unsafe { (p as *mut SorterRecord).offset(1 as isize) } as
                                            *mut () as *mut u8;
                                if __p.is_null() {
                                    &mut []
                                } else {
                                    core::slice::from_raw_parts_mut(__p,
                                        unsafe { (*p).n_val } as usize)
                                }
                            });
                        if unsafe { (*p_list_1).a_memory } == core::ptr::null_mut()
                            {
                            unsafe { sqlite3_free(p as *mut ()) };
                        }
                        break '__c17;
                    }
                    p = p_next;
                }
            }
            unsafe { (*p_list_1).p_list = p };
            rc =
                vdbe_pma_writer_finish(&mut writer,
                    unsafe { &mut (*p_task_1).file.i_eof },
                    unsafe { &mut (*p_task_1).n_spill });
        }
        { let _ = 0; };
        { let _ = 0; };
        return rc;
    }
}
extern "C" fn vdbe_sorter_flush_thread(p_ctx_1: *mut ()) -> *mut () {
    let p_task: *mut SortSubtask = p_ctx_1 as *mut SortSubtask;
    let mut rc: i32 = 0;
    { let _ = 0; };
    rc = vdbe_sorter_list_to_pma(p_task, unsafe { &mut (*p_task).list });
    unsafe { (*p_task).b_done = 1 };
    return rc as i64 as *mut ();
}
extern "C" fn vdbe_sorter_flush_pma(p_sorter_1: &mut VdbeSorter) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut i: i32 = 0;
        let mut p_task: *mut SortSubtask = core::ptr::null_mut();
        let n_worker: i32 = (*p_sorter_1).n_task as i32 - 1;
        (*p_sorter_1).b_use_pma = 1 as u8;
        {
            i = 0;
            '__b18: loop {
                if !(i < n_worker) { break '__b18; }
                '__c18: loop {
                    let i_test: i32 =
                        ((*p_sorter_1).i_prev as i32 + i + 1) % n_worker;
                    p_task =
                        unsafe {
                            &mut *((*p_sorter_1).a_task.as_ptr() as
                                            *mut SortSubtask).offset(i_test as isize)
                        };
                    if unsafe { (*p_task).b_done } != 0 {
                        rc = vdbe_sorter_join_thread(unsafe { &mut *p_task });
                    }
                    if rc != 0 ||
                            unsafe { (*p_task).p_thread } == core::ptr::null_mut() {
                        break '__b18;
                    }
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc == 0 {
            if i == n_worker {
                rc =
                    vdbe_sorter_list_to_pma(unsafe {
                            &mut *((*p_sorter_1).a_task.as_ptr() as
                                            *mut SortSubtask).offset(n_worker as isize)
                        }, &mut (*p_sorter_1).list);
            } else {
                let mut a_mem: *mut u8 = core::ptr::null_mut();
                let mut p_ctx: *mut () = core::ptr::null_mut();
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                a_mem = unsafe { (*p_task).list.a_memory };
                p_ctx = p_task as *mut ();
                (*p_sorter_1).i_prev =
                    unsafe {
                                p_task.offset_from((*p_sorter_1).a_task.as_ptr() as
                                        *mut SortSubtask)
                            } as i64 as u8;
                unsafe { (*p_task).list = (*p_sorter_1).list };
                (*p_sorter_1).list.p_list = core::ptr::null_mut();
                (*p_sorter_1).list.sz_pma = 0 as i64;
                if !(a_mem).is_null() {
                    (*p_sorter_1).list.a_memory = a_mem;
                    (*p_sorter_1).n_memory =
                        unsafe { sqlite3_malloc_size(a_mem as *const ()) };
                } else if !((*p_sorter_1).list.a_memory).is_null() {
                    (*p_sorter_1).list.a_memory =
                        unsafe { sqlite3Malloc((*p_sorter_1).n_memory as u64) } as
                            *mut u8;
                    if ((*p_sorter_1).list.a_memory).is_null() as i32 != 0 {
                        return 7;
                    }
                }
                rc =
                    vdbe_sorter_create_thread(unsafe { &mut *p_task },
                        Some(vdbe_sorter_flush_thread), p_ctx);
            }
        }
        return rc;
    }
}
extern "C" fn vdbe_merge_engine_new(n_reader_1: i32) -> *mut MergeEngine {
    let mut n: i32 = 2;
    let mut n_byte: i64 = 0 as i64;
    let mut p_new: *mut MergeEngine = core::ptr::null_mut();
    { let _ = 0; };
    while n < n_reader_1 { n += n; }
    n_byte =
        (core::mem::size_of::<MergeEngine>() as u64 +
                n as u64 *
                    (core::mem::size_of::<i32>() as u64 +
                        core::mem::size_of::<PmaReader>() as u64)) as i64;
    p_new =
        if unsafe { sqlite3_fault_sim(100) } != 0 {
            core::ptr::null_mut()
        } else {
            (unsafe { sqlite3_malloc_zero(n_byte as u64) }) as
                *mut MergeEngine
        };
    if !(p_new).is_null() {
        unsafe { (*p_new).n_tree = n };
        unsafe { (*p_new).p_task = core::ptr::null_mut() };
        unsafe {
            (*p_new).a_readr =
                unsafe { &raw mut *p_new.offset(1 as isize) } as
                    *mut PmaReader
        };
        unsafe {
            (*p_new).a_tree =
                unsafe {
                        &raw mut *unsafe { (*p_new).a_readr.offset(n as isize) }
                    } as *mut i32
        };
    }
    return p_new;
}
extern "C" fn vdbe_sorter_tree_depth(n_pma_1: i32) -> i32 {
    let mut n_depth: i32 = 0;
    let mut n_div: i64 = 16 as i64;
    while n_div < n_pma_1 as i64 {
        n_div = n_div * 16 as i64;
        { let __p = &mut n_depth; let __t = *__p; *__p += 1; __t };
    }
    return n_depth;
}
extern "C" fn vdbe_pma_reader_init(p_task_1: *mut SortSubtask,
    p_file_1: *mut SorterFile, i_start_1: i64, p_readr_1: *mut PmaReader,
    pn_byte_1: &mut i64) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    rc =
        vdbe_pma_reader_seek(p_task_1, unsafe { &mut *p_readr_1 }, p_file_1,
            i_start_1);
    if rc == 0 {
        let mut n_byte: u64 = 0 as u64;
        rc = vdbe_pma_read_varint(p_readr_1, &mut n_byte);
        unsafe {
            (*p_readr_1).i_eof =
                (unsafe { (*p_readr_1).i_read_off } as u64 + n_byte) as i64
        };
        *pn_byte_1 += n_byte as i64;
    }
    if rc == 0 { rc = vdbe_pma_reader_next(p_readr_1); }
    return rc;
}
extern "C" fn vdbe_merge_engine_level0(p_task_1: *mut SortSubtask,
    n_pma_1: i32, pi_offset_1: &mut i64, pp_out_1: &mut *mut MergeEngine)
    -> i32 {
    let mut p_new: *mut MergeEngine = core::ptr::null_mut();
    let mut i_off: i64 = *pi_offset_1;
    let mut i: i32 = 0;
    let mut rc: i32 = 0;
    *pp_out_1 = { p_new = vdbe_merge_engine_new(n_pma_1); p_new };
    if p_new == core::ptr::null_mut() { rc = 7; }
    {
        i = 0;
        '__b21: loop {
            if !(i < n_pma_1 && rc == 0) { break '__b21; }
            '__c21: loop {
                let mut n_dummy: i64 = 0 as i64;
                let p_readr: *mut PmaReader =
                    unsafe {
                        &mut *unsafe { (*p_new).a_readr.offset(i as isize) }
                    };
                rc =
                    vdbe_pma_reader_init(p_task_1,
                        unsafe { &mut (*p_task_1).file }, i_off, p_readr,
                        &mut n_dummy);
                i_off = unsafe { (*p_readr).i_eof };
                break '__c21;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc != 0 {
        vdbe_merge_engine_free(p_new);
        *pp_out_1 = core::ptr::null_mut();
    }
    *pi_offset_1 = i_off;
    return rc;
}
extern "C" fn vdbe_incr_merger_new(p_task_1: *mut SortSubtask,
    p_merger_1: *mut MergeEngine, pp_out_1: &mut *mut IncrMerger) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_incr: *mut IncrMerger =
            {
                *pp_out_1 =
                    if unsafe { sqlite3_fault_sim(100) } != 0 {
                            core::ptr::null_mut()
                        } else {
                            unsafe {
                                sqlite3_malloc_zero(core::mem::size_of::<IncrMerger>() as
                                        u64)
                            }
                        } as *mut IncrMerger;
                *pp_out_1
            };
        if !(p_incr).is_null() {
            unsafe { (*p_incr).p_merger = p_merger_1 };
            unsafe { (*p_incr).p_task = p_task_1 };
            unsafe {
                (*p_incr).mx_sz =
                    if unsafe { (*unsafe { (*p_task_1).p_sorter }).mx_keysize }
                                + 9 >
                            unsafe { (*unsafe { (*p_task_1).p_sorter }).mx_pma_size } /
                                2 {
                        (unsafe { (*unsafe { (*p_task_1).p_sorter }).mx_keysize }) +
                            9
                    } else {
                        (unsafe { (*unsafe { (*p_task_1).p_sorter }).mx_pma_size })
                            / 2
                    }
            };
            unsafe {
                (*p_task_1).file2.i_eof += unsafe { (*p_incr).mx_sz } as i64
            };
        } else { vdbe_merge_engine_free(p_merger_1); rc = 7; }
        { let _ = 0; };
        return rc;
    }
}
extern "C" fn vdbe_sorter_add_to_tree(p_task_1: *mut SortSubtask,
    n_depth_1: i32, i_seq_1: i32, p_root_1: *mut MergeEngine,
    p_leaf_1: *mut MergeEngine) -> i32 {
    let mut rc: i32 = 0;
    let mut n_div: i32 = 1;
    let mut i: i32 = 0;
    let mut p: *const MergeEngine = p_root_1 as *const MergeEngine;
    let mut p_incr: *mut IncrMerger = core::ptr::null_mut();
    rc = vdbe_incr_merger_new(p_task_1, p_leaf_1, &mut p_incr);
    {
        i = 1;
        '__b22: loop {
            if !(i < n_depth_1) { break '__b22; }
            '__c22: loop { n_div = n_div * 16; break '__c22; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        i = 1;
        '__b23: loop {
            if !(i < n_depth_1 && rc == 0) { break '__b23; }
            '__c23: loop {
                let i_iter: i32 = i_seq_1 / n_div % 16;
                let p_readr: *mut PmaReader =
                    unsafe {
                        &mut *unsafe { (*p).a_readr.offset(i_iter as isize) }
                    };
                if unsafe { (*p_readr).p_incr } == core::ptr::null_mut() {
                    let p_new: *mut MergeEngine = vdbe_merge_engine_new(16);
                    if p_new == core::ptr::null_mut() {
                        rc = 7;
                    } else {
                        rc =
                            vdbe_incr_merger_new(p_task_1, p_new,
                                unsafe { &mut (*p_readr).p_incr });
                    }
                }
                if rc == 0 {
                    p = unsafe { (*unsafe { (*p_readr).p_incr }).p_merger };
                    n_div = n_div / 16;
                }
                break '__c23;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 {
        unsafe {
            (*unsafe { (*p).a_readr.offset((i_seq_1 % 16) as isize) }).p_incr
                = p_incr
        };
    } else { vdbe_incr_free(p_incr); }
    return rc;
}
extern "C" fn vdbe_sorter_merge_tree_build(p_sorter_1: &mut VdbeSorter,
    pp_out_1: &mut *mut MergeEngine) -> i32 {
    let mut p_main: *mut MergeEngine = core::ptr::null_mut();
    let mut rc: i32 = 0;
    let mut i_task: i32 = 0;
    { let _ = 0; };
    if (*p_sorter_1).n_task as i32 > 1 {
        p_main = vdbe_merge_engine_new((*p_sorter_1).n_task as i32);
        if p_main == core::ptr::null_mut() { rc = 7; }
    }
    {
        i_task = 0;
        '__b24: loop {
            if !(rc == 0 && i_task < (*p_sorter_1).n_task as i32) {
                break '__b24;
            }
            '__c24: loop {
                let p_task: *mut SortSubtask =
                    unsafe {
                        &mut *((*p_sorter_1).a_task.as_ptr() as
                                        *mut SortSubtask).offset(i_task as isize)
                    };
                { let _ = 0; };
                if 8 == 0 || unsafe { (*p_task).n_pma } != 0 {
                    let mut p_root: *mut MergeEngine = core::ptr::null_mut();
                    let n_depth: i32 =
                        vdbe_sorter_tree_depth(unsafe { (*p_task).n_pma });
                    let mut i_read_off: i64 = 0 as i64;
                    if unsafe { (*p_task).n_pma } <= 16 {
                        rc =
                            vdbe_merge_engine_level0(p_task, unsafe { (*p_task).n_pma },
                                &mut i_read_off, &mut p_root);
                    } else {
                        let mut i: i32 = 0;
                        let mut i_seq: i32 = 0;
                        p_root = vdbe_merge_engine_new(16);
                        if p_root == core::ptr::null_mut() { rc = 7; }
                        {
                            i = 0;
                            '__b25: loop {
                                if !(i < unsafe { (*p_task).n_pma } && rc == 0) {
                                    break '__b25;
                                }
                                '__c25: loop {
                                    let mut p_merger: *mut MergeEngine = core::ptr::null_mut();
                                    let mut n_reader: i32 = 0;
                                    n_reader =
                                        if unsafe { (*p_task).n_pma } - i < 16 {
                                            (unsafe { (*p_task).n_pma }) - i
                                        } else { 16 };
                                    rc =
                                        vdbe_merge_engine_level0(p_task, n_reader, &mut i_read_off,
                                            &mut p_merger);
                                    if rc == 0 {
                                        rc =
                                            vdbe_sorter_add_to_tree(p_task, n_depth,
                                                { let __p = &mut i_seq; let __t = *__p; *__p += 1; __t },
                                                p_root, p_merger);
                                    }
                                    break '__c25;
                                }
                                i += 16;
                            }
                        }
                    }
                    if rc == 0 {
                        if p_main != core::ptr::null_mut() {
                            rc =
                                vdbe_incr_merger_new(p_task, p_root,
                                    unsafe {
                                        &mut (*unsafe {
                                                        (*p_main).a_readr.offset(i_task as isize)
                                                    }).p_incr
                                    });
                        } else { { let _ = 0; }; p_main = p_root; }
                    } else { vdbe_merge_engine_free(p_root); }
                }
                break '__c24;
            }
            { let __p = &mut i_task; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc != 0 {
        vdbe_merge_engine_free(p_main);
        p_main = core::ptr::null_mut();
    }
    *pp_out_1 = p_main;
    return rc;
}
extern "C" fn vdbe_incr_merger_set_threads(p_incr_1: &mut IncrMerger) -> () {
    (*p_incr_1).b_use_thread = 1;
    unsafe { (*(*p_incr_1).p_task).file2.i_eof -= (*p_incr_1).mx_sz as i64 };
}
extern "C" fn vdbe_merge_engine_compare(p_merger_1: &MergeEngine,
    i_out_1: i32) -> () {
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i_res: i32 = 0;
    let mut p1: *const PmaReader = core::ptr::null();
    let mut p2: *const PmaReader = core::ptr::null();
    { let _ = 0; };
    if i_out_1 >= (*p_merger_1).n_tree / 2 {
        i1 = (i_out_1 - (*p_merger_1).n_tree / 2) * 2;
        i2 = i1 + 1;
    } else {
        i1 = unsafe { *(*p_merger_1).a_tree.offset((i_out_1 * 2) as isize) };
        i2 =
            unsafe {
                *(*p_merger_1).a_tree.offset((i_out_1 * 2 + 1) as isize)
            };
    }
    p1 = unsafe { (*p_merger_1).a_readr.offset(i1 as isize) };
    p2 = unsafe { (*p_merger_1).a_readr.offset(i2 as isize) };
    if unsafe { (*p1).p_fd } == core::ptr::null_mut() {
        i_res = i2;
    } else if unsafe { (*p2).p_fd } == core::ptr::null_mut() {
        i_res = i1;
    } else {
        let p_task: *mut SortSubtask = (*p_merger_1).p_task;
        let mut b_cached: i32 = 0;
        let mut res: i32 = 0;
        { let _ = 0; };
        res =
            unsafe {
                (unsafe {
                        (*p_task).x_compare.unwrap()
                    })(p_task, &mut b_cached,
                    unsafe { (*p1).a_key } as *const (), unsafe { (*p1).n_key },
                    unsafe { (*p2).a_key } as *const (), unsafe { (*p2).n_key })
            };
        if res <= 0 { i_res = i1; } else { i_res = i2; }
    }
    unsafe { *(*p_merger_1).a_tree.offset(i_out_1 as isize) = i_res };
}
extern "C" fn vdbe_merge_engine_init(p_task_1: *mut SortSubtask,
    p_merger_1: *mut MergeEngine, e_mode_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let mut n_tree: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { (*p_merger_1).p_task = p_task_1 };
    n_tree = unsafe { (*p_merger_1).n_tree };
    {
        i = 0;
        '__b26: loop {
            if !(i < n_tree) { break '__b26; }
            '__c26: loop {
                if 8 > 0 && e_mode_1 == 2 {
                    rc =
                        vdbe_pma_reader_next(unsafe {
                                &mut *unsafe {
                                            (*p_merger_1).a_readr.offset((n_tree - i - 1) as isize)
                                        }
                            });
                } else {
                    rc =
                        unsafe {
                            vdbe_pma_reader_incr_init(unsafe {
                                    &mut *unsafe { (*p_merger_1).a_readr.offset(i as isize) }
                                }, 0)
                        };
                }
                if rc != 0 { return rc; }
                break '__c26;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        i = unsafe { (*p_merger_1).n_tree } - 1;
        '__b27: loop {
            if !(i > 0) { break '__b27; }
            '__c27: loop {
                vdbe_merge_engine_compare(unsafe { &*p_merger_1 }, i);
                break '__c27;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
    return unsafe { (*unsafe { (*p_task_1).p_unpacked }).err_code } as i32;
}
extern "C" fn vdbe_pma_reader_incr_merge_init(p_readr_1: *mut PmaReader,
    e_mode_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_incr: *mut IncrMerger = unsafe { (*p_readr_1).p_incr };
        let p_task: *mut SortSubtask = unsafe { (*p_incr).p_task };
        let db: *mut sqlite3 = unsafe { (*unsafe { (*p_task).p_sorter }).db };
        { let _ = 0; };
        rc =
            vdbe_merge_engine_init(p_task, unsafe { (*p_incr).p_merger },
                e_mode_1);
        if rc == 0 {
            let mx_sz: i32 = unsafe { (*p_incr).mx_sz };
            if unsafe { (*p_incr).b_use_thread } != 0 {
                rc =
                    vdbe_sorter_open_temp_file(db, mx_sz as i64,
                        unsafe { &mut (*p_incr).a_file[0 as usize].p_fd });
                if rc == 0 {
                    rc =
                        vdbe_sorter_open_temp_file(db, mx_sz as i64,
                            unsafe { &mut (*p_incr).a_file[1 as usize].p_fd });
                }
            } else {
                if unsafe { (*p_task).file2.p_fd } == core::ptr::null_mut() {
                    { let _ = 0; };
                    rc =
                        vdbe_sorter_open_temp_file(db,
                            unsafe { (*p_task).file2.i_eof },
                            unsafe { &mut (*p_task).file2.p_fd });
                    unsafe { (*p_task).file2.i_eof = 0 as i64 };
                }
                if rc == 0 {
                    unsafe {
                        (*p_incr).a_file[1 as usize].p_fd =
                            unsafe { (*p_task).file2.p_fd }
                    };
                    unsafe {
                        (*p_incr).i_start_off = unsafe { (*p_task).file2.i_eof }
                    };
                    unsafe { (*p_task).file2.i_eof += mx_sz as i64 };
                }
            }
        }
        if rc == 0 && unsafe { (*p_incr).b_use_thread } != 0 {
            { let _ = 0; };
            rc = vdbe_incr_populate(unsafe { &mut *p_incr });
        }
        if rc == 0 && (8 == 0 || e_mode_1 != 1) {
            rc = vdbe_pma_reader_next(p_readr_1);
        }
        return rc;
    }
}
extern "C" fn vdbe_pma_reader_bg_incr_init(p_ctx_1: *mut ()) -> *mut () {
    let p_reader: *mut PmaReader = p_ctx_1 as *mut PmaReader;
    let p_ret: *mut () =
        vdbe_pma_reader_incr_merge_init(p_reader, 1) as i64 as *mut ();
    unsafe {
        (*unsafe { (*unsafe { (*p_reader).p_incr }).p_task }).b_done = 1
    };
    return p_ret;
}
extern "C" fn vdbe_pma_reader_incr_init(p_readr_1: *mut PmaReader,
    e_mode_1: i32) -> i32 {
    let p_incr: *const IncrMerger =
        unsafe { (*p_readr_1).p_incr } as *const IncrMerger;
    let mut rc: i32 = 0;
    if !(p_incr).is_null() {
        { let _ = 0; };
        if unsafe { (*p_incr).b_use_thread } != 0 {
            let p_ctx: *mut () = p_readr_1 as *mut ();
            rc =
                vdbe_sorter_create_thread(unsafe {
                        &mut *unsafe { (*p_incr).p_task }
                    }, Some(vdbe_pma_reader_bg_incr_init), p_ctx);
        } else { rc = vdbe_pma_reader_incr_merge_init(p_readr_1, e_mode_1); }
    }
    return rc;
}
extern "C" fn vdbe_sorter_setup_merge(p_sorter_1: *mut VdbeSorter) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let p_task0: *mut SortSubtask =
            unsafe {
                &mut *(unsafe { (*p_sorter_1).a_task.as_ptr() } as
                                *mut SortSubtask).offset(0 as isize)
            };
        let mut p_main: *mut MergeEngine = core::ptr::null_mut();
        let db: *mut sqlite3 =
            unsafe { (*unsafe { (*p_task0).p_sorter }).db };
        let mut i: i32 = 0;
        let x_compare:
                Option<unsafe extern "C" fn(*mut SortSubtask, *mut i32,
                    *const (), i32, *const (), i32) -> i32> =
            Some(vdbe_sorter_get_compare(unsafe { &*p_sorter_1 }));
        {
            i = 0;
            '__b28: loop {
                if !(i < unsafe { (*p_sorter_1).n_task } as i32) {
                    break '__b28;
                }
                '__c28: loop {
                    unsafe {
                        (*(unsafe { (*p_sorter_1).a_task.as_ptr() } as
                                            *mut SortSubtask).offset(i as isize)).x_compare = x_compare
                    };
                    break '__c28;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        rc =
            vdbe_sorter_merge_tree_build(unsafe { &mut *p_sorter_1 },
                &mut p_main);
        if rc == 0 {
            { let _ = 0; };
            if unsafe { (*p_sorter_1).b_use_threads } != 0 {
                let mut i_task: i32 = 0;
                let mut p_readr: *mut PmaReader = core::ptr::null_mut();
                let p_last: *mut SortSubtask =
                    unsafe {
                        &mut *(unsafe { (*p_sorter_1).a_task.as_ptr() } as
                                        *mut SortSubtask).offset((unsafe { (*p_sorter_1).n_task } as
                                                i32 - 1) as isize)
                    };
                rc = vdbe_sort_alloc_unpacked(unsafe { &mut *p_last });
                if rc == 0 {
                    p_readr =
                        unsafe {
                                sqlite3_db_malloc_zero(db,
                                    core::mem::size_of::<PmaReader>() as u64)
                            } as *mut PmaReader;
                    unsafe { (*p_sorter_1).p_reader = p_readr };
                    if p_readr == core::ptr::null_mut() { rc = 7; }
                }
                if rc == 0 {
                    rc =
                        vdbe_incr_merger_new(p_last, p_main,
                            unsafe { &mut (*p_readr).p_incr });
                    if rc == 0 {
                        vdbe_incr_merger_set_threads(unsafe {
                                &mut *unsafe { (*p_readr).p_incr }
                            });
                        {
                            i_task = 0;
                            '__b29: loop {
                                if !(i_task < unsafe { (*p_sorter_1).n_task } as i32 - 1) {
                                    break '__b29;
                                }
                                '__c29: loop {
                                    let mut p_incr: *mut IncrMerger = core::ptr::null_mut();
                                    if !({
                                                        p_incr =
                                                            unsafe {
                                                                (*unsafe {
                                                                            (*p_main).a_readr.offset(i_task as isize)
                                                                        }).p_incr
                                                            };
                                                        p_incr
                                                    }).is_null() {
                                        vdbe_incr_merger_set_threads(unsafe { &mut *p_incr });
                                        { let _ = 0; };
                                    }
                                    break '__c29;
                                }
                                { let __p = &mut i_task; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        {
                            i_task = 0;
                            '__b30: loop {
                                if !(rc == 0 &&
                                                i_task < unsafe { (*p_sorter_1).n_task } as i32) {
                                    break '__b30;
                                }
                                '__c30: loop {
                                    let p: *mut PmaReader =
                                        unsafe {
                                            &mut *unsafe { (*p_main).a_readr.offset(i_task as isize) }
                                        };
                                    { let _ = 0; };
                                    rc = vdbe_pma_reader_incr_init(p, 1);
                                    break '__c30;
                                }
                                { let __p = &mut i_task; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                    p_main = core::ptr::null_mut();
                }
                if rc == 0 {
                    rc = vdbe_pma_reader_incr_merge_init(p_readr, 2);
                }
            } else {
                rc = vdbe_merge_engine_init(p_task0, p_main, 0);
                unsafe { (*p_sorter_1).p_merger = p_main };
                p_main = core::ptr::null_mut();
            }
        }
        if rc != 0 { vdbe_merge_engine_free(p_main); }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_sorter_rewind(p_csr_1: &VdbeCursor,
    pb_eof_1: &mut i32) -> i32 {
    unsafe {
        let mut p_sorter: *mut VdbeSorter = core::ptr::null_mut();
        let mut rc: i32 = 0;
        { let _ = 0; };
        p_sorter = (*p_csr_1).uc.p_sorter;
        { let _ = 0; };
        if unsafe { (*p_sorter).b_use_pma } as i32 == 0 {
            if !(unsafe { (*p_sorter).list.p_list }).is_null() {
                *pb_eof_1 = 0;
                rc =
                    vdbe_sorter_sort(unsafe {
                            &mut *(unsafe { (*p_sorter).a_task.as_ptr() } as
                                            *mut SortSubtask).offset(0 as isize)
                        }, unsafe { &mut (*p_sorter).list });
            } else { *pb_eof_1 = 1; }
            return rc;
        }
        { let _ = 0; };
        rc = vdbe_sorter_flush_pma(unsafe { &mut *p_sorter });
        rc = vdbe_sorter_join_all(unsafe { &mut *p_sorter }, rc);
        { let _ = 0; };
        if rc == 0 { rc = vdbe_sorter_setup_merge(p_sorter); *pb_eof_1 = 0; }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_sorter_write(p_csr_1: &VdbeCursor,
    p_val_1: &Mem) -> i32 {
    unsafe {
        let mut p_sorter: *mut VdbeSorter = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut p_new: *mut SorterRecord = core::ptr::null_mut();
        let mut b_flush: i32 = 0;
        let mut n_req: i64 = 0 as i64;
        let mut n_pma: i64 = 0 as i64;
        let mut t: i32 = 0;
        { let _ = 0; };
        p_sorter = (*p_csr_1).uc.p_sorter;
        t =
            unsafe {
                        *(unsafe { &raw mut *(*p_val_1).z.offset(1 as isize) } as
                                *const u8)
                    } as u32 as i32;
        if t >= 128 {
            unsafe {
                sqlite3_get_varint32(unsafe {
                            &raw mut *(*p_val_1).z.offset(1 as isize)
                        } as *const u8, &raw mut t as *mut u32)
            };
        }
        if t > 0 && t < 10 && t != 7 {
            unsafe { (*p_sorter).type_mask &= 1 as u8 };
        } else if t > 10 && t & 1 != 0 {
            unsafe { (*p_sorter).type_mask &= 2 as u8 };
        } else { unsafe { (*p_sorter).type_mask = 0 as u8 }; }
        { let _ = 0; };
        n_req =
            ((*p_val_1).n as u64 +
                    core::mem::size_of::<SorterRecord>() as u64) as i64;
        n_pma =
            ((*p_val_1).n +
                    unsafe { sqlite3_varint_len((*p_val_1).n as u64) }) as i64;
        if unsafe { (*p_sorter).mx_pma_size } != 0 {
            if !(unsafe { (*p_sorter).list.a_memory }).is_null() {
                b_flush =
                    (unsafe { (*p_sorter).i_memory } != 0 &&
                            unsafe { (*p_sorter).i_memory } as i64 + n_req >
                                unsafe { (*p_sorter).mx_pma_size } as i64) as i32;
            } else {
                b_flush =
                    (unsafe { (*p_sorter).list.sz_pma } >
                                unsafe { (*p_sorter).mx_pma_size } as i64 ||
                            unsafe { (*p_sorter).list.sz_pma } >
                                    unsafe { (*p_sorter).mn_pma_size } as i64 &&
                                unsafe { sqlite3_heap_nearly_full() } != 0) as i32;
            }
            if b_flush != 0 {
                rc = vdbe_sorter_flush_pma(unsafe { &mut *p_sorter });
                unsafe { (*p_sorter).list.sz_pma = 0 as i64 };
                unsafe { (*p_sorter).i_memory = 0 };
                { let _ = 0; };
            }
        }
        unsafe { (*p_sorter).list.sz_pma += n_pma };
        if n_pma > unsafe { (*p_sorter).mx_keysize } as i64 {
            unsafe { (*p_sorter).mx_keysize = n_pma as i32 };
        }
        if !(unsafe { (*p_sorter).list.a_memory }).is_null() {
            let n_min: i32 =
                (unsafe { (*p_sorter).i_memory } as i64 + n_req) as i32;
            if n_min > unsafe { (*p_sorter).n_memory } {
                let mut a_new: *mut u8 = core::ptr::null_mut();
                let mut n_new: sqlite3_int64 =
                    2 as sqlite3_int64 *
                        unsafe { (*p_sorter).n_memory } as sqlite3_int64;
                let mut i_list_off: i32 = -1;
                if !(unsafe { (*p_sorter).list.p_list }).is_null() {
                    i_list_off =
                        unsafe {
                                    (unsafe { (*p_sorter).list.p_list } as
                                            *mut u8).offset_from(unsafe { (*p_sorter).list.a_memory })
                                } as i64 as i32;
                }
                while n_new < n_min as i64 {
                    n_new = n_new * 2 as sqlite3_int64;
                }
                if n_new > unsafe { (*p_sorter).mx_pma_size } as i64 {
                    n_new = unsafe { (*p_sorter).mx_pma_size } as sqlite3_int64;
                }
                if n_new < n_min as i64 { n_new = n_min as sqlite3_int64; }
                a_new =
                    unsafe {
                            sqlite3Realloc(unsafe { (*p_sorter).list.a_memory } as
                                    *mut (), n_new as u64)
                        } as *mut u8;
                if (a_new).is_null() as i32 != 0 { return 7; }
                if i_list_off >= 0 {
                    unsafe {
                        (*p_sorter).list.p_list =
                            unsafe { &raw mut *a_new.offset(i_list_off as isize) } as
                                *mut SorterRecord
                    };
                }
                unsafe { (*p_sorter).list.a_memory = a_new };
                unsafe { (*p_sorter).n_memory = n_new as i32 };
            }
            p_new =
                unsafe {
                        &raw mut *unsafe {
                                    (*p_sorter).list.a_memory.offset(unsafe {
                                                (*p_sorter).i_memory
                                            } as isize)
                                }
                    } as *mut SorterRecord;
            unsafe {
                (*p_sorter).i_memory += (n_req + 7 as i64 & !7 as i64) as i32
            };
            if !(unsafe { (*p_sorter).list.p_list }).is_null() {
                unsafe {
                    (*p_new).u.i_next =
                        unsafe {
                                    (unsafe { (*p_sorter).list.p_list } as
                                            *mut u8).offset_from(unsafe { (*p_sorter).list.a_memory })
                                } as i64 as i32
                };
            }
        } else {
            p_new =
                unsafe { sqlite3Malloc(n_req as u64) } as *mut SorterRecord;
            if p_new == core::ptr::null_mut() { return 7; }
            unsafe { (*p_new).u.p_next = unsafe { (*p_sorter).list.p_list } };
        }
        unsafe {
            memcpy(unsafe { (p_new as *mut SorterRecord).offset(1 as isize) }
                    as *mut (), (*p_val_1).z as *const (), (*p_val_1).n as u64)
        };
        unsafe { (*p_new).n_val = (*p_val_1).n };
        unsafe { (*p_sorter).list.p_list = p_new };
        return rc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_sorter_compare(p_csr_1: &VdbeCursor,
    p_val_1: &Mem, n_key_col_1: i32, p_res_1: &mut i32) -> i32 {
    unsafe {
        let mut p_sorter: *mut VdbeSorter = core::ptr::null_mut();
        let mut r2: *mut UnpackedRecord = core::ptr::null_mut();
        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut p_key: *mut () = core::ptr::null_mut();
        let mut n_key: i32 = 0;
        { let _ = 0; };
        p_sorter = (*p_csr_1).uc.p_sorter;
        r2 = unsafe { (*p_sorter).p_unpacked };
        p_key_info = (*p_csr_1).p_key_info;
        if r2 == core::ptr::null_mut() {
            r2 =
                {
                    unsafe {
                        (*p_sorter).p_unpacked =
                            unsafe { sqlite3_vdbe_alloc_unpacked_record(p_key_info) }
                    };
                    unsafe { (*p_sorter).p_unpacked }
                };
            if r2 == core::ptr::null_mut() { return 7; }
            unsafe { (*r2).n_field = n_key_col_1 as u16 };
        }
        { let _ = 0; };
        p_key = vdbe_sorter_rowkey(unsafe { &*p_sorter }, &mut n_key);
        unsafe { sqlite3_vdbe_record_unpack(n_key, p_key as *const (), r2) };
        {
            i = 0;
            '__b32: loop {
                if !(i < n_key_col_1) { break '__b32; }
                '__c32: loop {
                    if unsafe {
                                        (*unsafe { (*r2).a_mem.offset(i as isize) }).flags
                                    } as i32 & 1 != 0 {
                        *p_res_1 = -1;
                        return 0;
                    }
                    break '__c32;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        *p_res_1 =
            unsafe {
                sqlite3_vdbe_record_compare((*p_val_1).n,
                    (*p_val_1).z as *const (), r2)
            };
        return 0;
    }
}
static a_len: [u8; 10] =
    [0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 6 as u8, 8 as u8, 0 as u8,
            0 as u8, 0 as u8];
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
    static sqlite3_small_type_sizes: [u8; 0];
    fn sqlite3_vdbe_error(_: *mut Vdbe, _: *const i8, ...)
    -> ();
    fn sqlite3_vdbe_free_cursor(_: *mut Vdbe, _: *mut VdbeCursor)
    -> ();
    fn sqlite3_vdbe_free_cursor_nn(_: *mut Vdbe, _: *mut VdbeCursor)
    -> ();
    fn sqlite_vdbe_pop_stack(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_handle_moved_cursor(p: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_finish_moveto(_: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_cursor_restore(_: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_serial_type_len(_: u32)
    -> u32;
    fn sqlite3_vdbe_one_byte_serial_type_len(_: u8)
    -> u8;
    fn sqlite3_vdbe_serial_get(_: *const u8, _: u32, _: *mut Mem)
    -> ();
    fn sqlite3_vdbe_delete_aux_data(_: *mut sqlite3, _: *mut *mut AuxData,
    _: i32, _: i32)
    -> ();
    fn sqlite2_btree_key_compare(_: *mut BtCursor, _: *const (), _: i32,
    _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_idx_key_compare(_: *mut sqlite3, _: *mut VdbeCursor,
    _: *mut UnpackedRecord, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_idx_rowid(_: *mut sqlite3, _: *mut BtCursor, _: *mut i64)
    -> i32;
    fn sqlite3_vdbe_exec(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_next_opcode(_: *mut Vdbe, _: *mut Mem, _: i32,
    _: *mut i32, _: *mut i32, _: *mut *mut Op)
    -> i32;
    fn sqlite3_vdbe_display_p4(_: *mut sqlite3, _: *mut Op)
    -> *mut i8;
    fn sqlite3_vdbe_list(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_halt(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_change_encoding(_: *mut Mem, _: i32)
    -> i32;
    fn sqlite3_vdbe_mem_too_big(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_copy(_: *mut Mem, _: *const Mem)
    -> i32;
    fn sqlite3_vdbe_mem_shallow_copy(_: *mut Mem, _: *const Mem, _: i32)
    -> ();
    fn sqlite3_vdbe_mem_move(_: *mut Mem, _: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_nul_terminate(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_set_str(_: *mut Mem, _: *const i8, _: i64, _: u8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_mem_set_text(_: *mut Mem, _: *const i8, _: i64,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_mem_set_int64(_: *mut Mem, _: i64)
    -> ();
    fn sqlite3_vdbe_mem_set_double(_: *mut Mem, _: f64)
    -> ();
    fn sqlite3_vdbe_mem_set_pointer(_: *mut Mem, _: *mut (), _: *const i8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_vdbe_mem_init(_: *mut Mem, _: *mut sqlite3, _: u16)
    -> ();
    fn sqlite3_vdbe_mem_set_null(_: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_set_zero_blob(_: *mut Mem, _: i32)
    -> ();
    fn sqlite3_vdbe_mem_set_row_set(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_zero_terminate_if_able(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_make_writeable(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_stringify(_: *mut Mem, _: u8, _: u8)
    -> i32;
    fn sqlite3_int_float_compare(_: i64, _: f64)
    -> i32;
    fn sqlite3_vdbe_int_value(_: *const Mem)
    -> i64;
    fn sqlite3_vdbe_mem_integerify(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_real_value(_: *mut Mem)
    -> f64;
    fn sqlite3_mem_real_value_rc(_: *mut Mem, _: *mut f64)
    -> i32;
    fn sqlite3_vdbe_boolean_value(_: *mut Mem, if_null_1: i32)
    -> i32;
    fn sqlite3_vdbe_integer_affinity(_: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_realify(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_numerify(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_cast(_: *mut Mem, _: u8, _: u8)
    -> i32;
    fn sqlite3_vdbe_mem_from_btree(_: *mut BtCursor, _: u32, _: u32,
    _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_from_btree_zero_offset(_: *mut BtCursor, _: u32,
    _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_release(p: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_release_malloc(p: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_finalize(_: *mut Mem, _: *mut FuncDef)
    -> i32;
    fn sqlite3_vdbe_mem_agg_value(_: *mut Mem, _: *mut Mem, _: *mut FuncDef)
    -> i32;
    fn sqlite3_opcode_name(_: i32)
    -> *const i8;
    fn sqlite3_vdbe_mem_grow(p_mem_1: *mut Mem, n: i32, preserve: i32)
    -> i32;
    fn sqlite3_vdbe_mem_clear_and_resize(p_mem_1: *mut Mem, n: i32)
    -> i32;
    fn sqlite3_vdbe_close_statement(_: *mut Vdbe, _: i32)
    -> i32;
    fn sqlite3_vdbe_frame_mem_del(_: *mut ())
    -> ();
    fn sqlite3_vdbe_frame_delete(_: *mut VdbeFrame)
    -> ();
    fn sqlite3_vdbe_frame_restore(_: *mut VdbeFrame)
    -> i32;
    fn sqlite3_vdbe_transfer_error(p: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_find_index_key(_: *mut BtCursor, _: *mut Index,
    _: *mut UnpackedRecord, _: *mut i32, _: i32)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn sqlite3_vdbe_value_list_free(_: *mut ())
    -> ();
    fn sqlite3_vdbe_enter(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_leave(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_check_fk_immediate(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_check_fk_deferred(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_mem_translate(_: *mut Mem, _: u8)
    -> i32;
    fn sqlite3_vdbe_mem_handle_bom(p_mem_1: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_expand_blob(_: *mut Mem)
    -> i32;
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