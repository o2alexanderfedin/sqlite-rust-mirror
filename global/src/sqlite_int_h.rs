use super::*;#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3 {
    pub(crate) p_vfs: *mut Sqlite3Vfs,
    pub(crate) p_vdbe: *mut Vdbe,
    pub(crate) p_dflt_coll: *mut CollSeq,
    pub(crate) mutex: *mut Sqlite3Mutex,
    pub(crate) a_db: *mut Db,
    pub(crate) n_db: i32,
    pub(crate) m_db_flags: u32,
    pub(crate) flags: u64,
    pub(crate) last_rowid: i64,
    pub(crate) sz_mmap: i64,
    pub(crate) n_schema_lock: u32,
    pub(crate) open_flags: u32,
    pub(crate) err_code: i32,
    pub(crate) err_byte_offset: i32,
    pub(crate) err_mask: i32,
    pub(crate) i_sys_errno: i32,
    pub(crate) db_opt_flags: u32,
    pub(crate) enc: u8,
    pub(crate) auto_commit: u8,
    pub(crate) temp_store: u8,
    pub(crate) malloc_failed: u8,
    pub(crate) b_benign_malloc: u8,
    pub(crate) dflt_lock_mode: u8,
    pub(crate) next_autovac: i8,
    pub(crate) suppress_err: u8,
    pub(crate) vtab_on_conflict: u8,
    pub(crate) is_transaction_savepoint: u8,
    pub(crate) m_trace: u8,
    pub(crate) no_shared_cache: u8,
    pub(crate) n_sql_exec: u8,
    pub(crate) e_open_state: u8,
    pub(crate) n_fp_digit: u8,
    pub(crate) next_pagesize: i32,
    pub(crate) n_change: i64,
    pub(crate) n_total_change: i64,
    pub(crate) a_limit: [i32; 13],
    pub(crate) n_max_sorter_mmap: i32,
    pub(crate) init: Sqlite3InitInfo,
    pub(crate) n_vdbe_active: i32,
    pub(crate) n_vdbe_read: i32,
    pub(crate) n_vdbe_write: i32,
    pub(crate) n_vdbe_exec: i32,
    pub(crate) n_v_destroy: i32,
    pub(crate) n_extension: i32,
    pub(crate) a_extension: *mut *mut (),
    pub(crate) trace: Sqlite3U0,
    pub(crate) p_trace_arg: *mut (),
    pub(crate) x_profile: Option<unsafe extern "C" fn(*mut (), *const i8, u64)
        -> ()>,
    pub(crate) p_profile_arg: *mut (),
    pub(crate) p_commit_arg: *mut (),
    pub(crate) x_commit_callback: Option<unsafe extern "C" fn(*mut ())
        -> i32>,
    pub(crate) p_rollback_arg: *mut (),
    pub(crate) x_rollback_callback: Option<unsafe extern "C" fn(*mut ())
        -> ()>,
    pub(crate) p_update_arg: *mut (),
    pub(crate) x_update_callback: Option<unsafe extern "C" fn(*mut (), i32,
        *const i8, *const i8, i64) -> ()>,
    pub(crate) p_autovac_pages_arg: *mut (),
    pub(crate) x_autovac_destr: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) x_autovac_pages: Option<unsafe extern "C" fn(*mut (),
        *const i8, u32, u32, u32) -> u32>,
    pub(crate) p_parse: *mut Parse,
    pub(crate) x_wal_callback: Option<unsafe extern "C" fn(*mut (),
        *mut Sqlite3, *const i8, i32) -> i32>,
    pub(crate) p_wal_arg: *mut (),
    pub(crate) x_coll_needed: Option<unsafe extern "C" fn(*mut (),
        *mut Sqlite3, i32, *const i8) -> ()>,
    pub(crate) x_coll_needed16: Option<unsafe extern "C" fn(*mut (),
        *mut Sqlite3, i32, *const ()) -> ()>,
    pub(crate) p_coll_needed_arg: *mut (),
    pub(crate) p_err: *mut Sqlite3Value,
    pub(crate) u1: Sqlite3U1,
    pub(crate) lookaside: Lookaside,
    pub(crate) x_auth: Option<unsafe extern "C" fn(*mut (), i32, *const i8,
        *const i8, *const i8, *const i8) -> i32>,
    pub(crate) p_auth_arg: *mut (),
    pub(crate) x_progress: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    pub(crate) p_progress_arg: *mut (),
    pub(crate) n_progress_ops: u32,
    pub(crate) n_v_trans: i32,
    pub(crate) a_module: Hash,
    pub(crate) p_vtab_ctx: *mut VtabCtx,
    pub(crate) a_v_trans: *mut *mut VTable,
    pub(crate) p_disconnect: *mut VTable,
    pub(crate) a_func: Hash,
    pub(crate) a_coll_seq: Hash,
    pub(crate) busy_handler: BusyHandler,
    pub(crate) a_db_static: [Db; 2],
    pub(crate) p_savepoint: *mut Savepoint,
    pub(crate) n_analysis_limit: i32,
    pub(crate) busy_timeout: i32,
    pub(crate) n_savepoint: i32,
    pub(crate) n_statement: i32,
    pub(crate) n_deferred_cons: i64,
    pub(crate) n_deferred_imm_cons: i64,
    pub(crate) pn_bytes_freed: *mut i32,
    pub(crate) p_db_data: *mut DbClientData,
    pub(crate) n_spill: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CollSeq {
    pub(crate) z_name: *mut i8,
    pub(crate) enc: u8,
    pub(crate) p_user: *mut (),
    pub(crate) x_cmp: Option<unsafe extern "C" fn(*mut (), i32, *const (),
        i32, *const ()) -> i32>,
    pub(crate) x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Db {
    pub(crate) z_db_s_name: *mut i8,
    pub(crate) p_bt: *mut Btree,
    pub(crate) safety_level: u8,
    pub(crate) b_sync_set: u8,
    pub(crate) p_schema: *mut Schema,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Schema {
    pub(crate) schema_cookie: i32,
    pub(crate) i_generation: i32,
    pub(crate) tbl_hash: Hash,
    pub(crate) idx_hash: Hash,
    pub(crate) trig_hash: Hash,
    pub(crate) fkey_hash: Hash,
    pub(crate) p_seq_tab: *mut Table,
    pub(crate) file_format: u8,
    pub(crate) enc: u8,
    pub(crate) schema_flags: u16,
    pub(crate) cache_size: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Table {
    pub(crate) z_name: *mut i8,
    pub(crate) a_col: *mut Column,
    pub(crate) p_index: *mut Index,
    pub(crate) z_col_aff: *mut i8,
    pub(crate) p_check: *mut ExprList,
    pub(crate) tnum: Pgno,
    pub(crate) n_tab_ref: u32,
    pub(crate) tab_flags: u32,
    pub(crate) i_p_key: i16,
    pub(crate) n_col: i16,
    pub(crate) n_nv_col: i16,
    pub(crate) n_row_log_est: LogEst,
    pub(crate) sz_tab_row: LogEst,
    pub(crate) key_conf: u8,
    pub(crate) e_tab_type: u8,
    pub(crate) u: TableU0,
    pub(crate) p_trigger: *mut Trigger,
    pub(crate) p_schema: *mut Schema,
    pub(crate) a_hx: [u8; 16],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Column {
    pub(crate) z_cn_name: *mut i8,
    pub(crate) _bitfield_1: u32,
    pub(crate) affinity: i8,
    pub(crate) sz_est: u8,
    pub(crate) h_name: u8,
    pub(crate) i_dflt: u16,
    pub(crate) col_flags: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Index {
    pub(crate) z_name: *mut i8,
    pub(crate) ai_column: *mut i16,
    pub(crate) ai_row_log_est: *mut LogEst,
    pub(crate) p_table: *mut Table,
    pub(crate) z_col_aff: *mut i8,
    pub(crate) p_next: *mut Index,
    pub(crate) p_schema: *mut Schema,
    pub(crate) a_sort_order: *mut u8,
    pub(crate) az_coll: *mut *const i8,
    pub(crate) p_part_idx_where: *mut Expr,
    pub(crate) a_col_expr: *mut ExprList,
    pub(crate) tnum: Pgno,
    pub(crate) sz_idx_row: LogEst,
    pub(crate) n_key_col: u16,
    pub(crate) n_column: u16,
    pub(crate) on_error: u8,
    pub(crate) _bitfield_1: u32,
    pub(crate) col_not_idxed: Bitmask,
}
pub(crate) type LogEst = i16;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Expr {
    pub(crate) op: u8,
    pub(crate) aff_expr: i8,
    pub(crate) op2: u8,
    pub(crate) flags: u32,
    pub(crate) u: ExprU0,
    pub(crate) p_left: *mut Expr,
    pub(crate) p_right: *mut Expr,
    pub(crate) x: ExprU1,
    pub(crate) n_height: i32,
    pub(crate) i_table: i32,
    pub(crate) i_column: YnVar,
    pub(crate) i_agg: i16,
    pub(crate) w: ExprU2,
    pub(crate) p_agg_info: *mut AggInfo,
    pub(crate) y: ExprU3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ExprU0 {
    pub(crate) z_token: *mut i8,
    pub(crate) i_value: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ExprU1 {
    pub(crate) p_list: *mut ExprList,
    pub(crate) p_select: *mut Select,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ExprList {
    pub(crate) n_expr: i32,
    pub(crate) n_alloc: i32,
    pub(crate) a: [ExprListItem; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ExprListItem {
    pub(crate) p_expr: *mut Expr,
    pub(crate) z_e_name: *mut i8,
    pub(crate) fg: ExprListItemS0,
    pub(crate) u: ExprListItemU1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ExprListItemS0 {
    pub(crate) sort_flags: u8,
    pub(crate) _bitfield_1: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ExprListItemU1 {
    pub(crate) x: ExprListItemU1S0,
    pub(crate) i_const_expr_reg: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ExprListItemU1S0 {
    pub(crate) i_order_by_col: u16,
    pub(crate) i_alias: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Select {
    pub(crate) op: u8,
    pub(crate) n_select_row: LogEst,
    pub(crate) sel_flags: u32,
    pub(crate) i_limit: i32,
    pub(crate) i_offset: i32,
    pub(crate) sel_id: u32,
    pub(crate) p_e_list: *mut ExprList,
    pub(crate) p_src: *mut SrcList,
    pub(crate) p_where: *mut Expr,
    pub(crate) p_group_by: *mut ExprList,
    pub(crate) p_having: *mut Expr,
    pub(crate) p_order_by: *mut ExprList,
    pub(crate) p_prior: *mut Select,
    pub(crate) p_next: *mut Select,
    pub(crate) p_limit: *mut Expr,
    pub(crate) p_with: *mut With,
    pub(crate) p_win: *mut Window,
    pub(crate) p_win_defn: *mut Window,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SrcList {
    pub(crate) n_src: i32,
    pub(crate) n_alloc: u32,
    pub(crate) a: [SrcItem; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SrcItem {
    pub(crate) z_name: *mut i8,
    pub(crate) z_alias: *mut i8,
    pub(crate) p_s_tab: *mut Table,
    pub(crate) fg: SrcItemS0,
    pub(crate) i_cursor: i32,
    pub(crate) col_used: Bitmask,
    pub(crate) u1: SrcItemU1,
    pub(crate) u2: SrcItemU2,
    pub(crate) u3: SrcItemU3,
    pub(crate) u4: SrcItemU4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SrcItemS0 {
    pub(crate) jointype: u8,
    pub(crate) _bitfield_1: u32,
}
pub(crate) type Bitmask = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union SrcItemU1 {
    pub(crate) z_indexed_by: *mut i8,
    pub(crate) p_func_arg: *mut ExprList,
    pub(crate) n_row: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union SrcItemU2 {
    pub(crate) p_ib_index: *mut Index,
    pub(crate) p_cte_use: *mut CteUse,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CteUse {
    pub(crate) n_use: i32,
    pub(crate) addr_m9e: i32,
    pub(crate) reg_rtn: i32,
    pub(crate) i_cur: i32,
    pub(crate) n_row_est: LogEst,
    pub(crate) e_m10d: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union SrcItemU3 {
    pub(crate) p_on: *mut Expr,
    pub(crate) p_using: *mut IdList,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IdList {
    pub(crate) n_id: i32,
    pub(crate) a: [IdListItem; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IdListItem {
    pub(crate) z_name: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union SrcItemU4 {
    pub(crate) p_schema: *mut Schema,
    pub(crate) z_database: *mut i8,
    pub(crate) p_subq: *mut Subquery,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Subquery {
    pub(crate) p_select: *mut Select,
    pub(crate) addr_fill_sub: i32,
    pub(crate) reg_return: i32,
    pub(crate) reg_result: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct With {
    pub(crate) n_cte: i32,
    pub(crate) b_view: i32,
    pub(crate) p_outer: *mut With,
    pub(crate) a: [Cte; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Cte {
    pub(crate) z_name: *mut i8,
    pub(crate) p_cols: *mut ExprList,
    pub(crate) p_select: *mut Select,
    pub(crate) z_cte_err: *const i8,
    pub(crate) p_use: *mut CteUse,
    pub(crate) e_m10d: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Window {
    pub(crate) z_name: *mut i8,
    pub(crate) z_base: *mut i8,
    pub(crate) p_partition: *mut ExprList,
    pub(crate) p_order_by: *mut ExprList,
    pub(crate) e_frm_type: u8,
    pub(crate) e_start: u8,
    pub(crate) e_end: u8,
    pub(crate) b_implicit_frame: u8,
    pub(crate) e_exclude: u8,
    pub(crate) p_start: *mut Expr,
    pub(crate) p_end: *mut Expr,
    pub(crate) pp_this: *mut *mut Window,
    pub(crate) p_next_win: *mut Window,
    pub(crate) p_filter: *mut Expr,
    pub(crate) p_w_func: *mut FuncDef,
    pub(crate) i_eph_csr: i32,
    pub(crate) reg_accum: i32,
    pub(crate) reg_result: i32,
    pub(crate) csr_app: i32,
    pub(crate) reg_app: i32,
    pub(crate) reg_part: i32,
    pub(crate) p_owner: *mut Expr,
    pub(crate) n_buffer_col: i32,
    pub(crate) i_arg_col: i32,
    pub(crate) reg_one: i32,
    pub(crate) reg_start_rowid: i32,
    pub(crate) reg_end_rowid: i32,
    pub(crate) b_expr_args: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FuncDef {
    pub(crate) n_arg: i16,
    pub(crate) func_flags: u32,
    pub(crate) p_user_data: *mut (),
    pub(crate) p_next: *mut FuncDef,
    pub(crate) x_s_func: Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
        *mut *mut Sqlite3Value) -> ()>,
    pub(crate) x_finalize: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> ()>,
    pub(crate) x_value: Option<unsafe extern "C" fn(*mut Sqlite3Context)
        -> ()>,
    pub(crate) x_inverse: Option<unsafe extern "C" fn(*mut Sqlite3Context,
        i32, *mut *mut Sqlite3Value) -> ()>,
    pub(crate) z_name: *const i8,
    pub(crate) u: FuncDefU0,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union FuncDefU0 {
    pub(crate) p_hash: *mut FuncDef,
    pub(crate) p_destructor: *mut FuncDestructor,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FuncDestructor {
    pub(crate) n_ref: i32,
    pub(crate) x_destroy: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) p_user_data: *mut (),
}
pub(crate) type YnVar = i16;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ExprU2 {
    pub(crate) i_join: i32,
    pub(crate) i_ofst: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct AggInfo {
    pub(crate) direct_mode: u8,
    pub(crate) use_sorting_idx: u8,
    pub(crate) n_sorting_column: u32,
    pub(crate) sorting_idx: i32,
    pub(crate) sorting_idx_p_tab: i32,
    pub(crate) i_first_reg: i32,
    pub(crate) p_group_by: *mut ExprList,
    pub(crate) a_col: *mut AggInfoCol,
    pub(crate) n_column: i32,
    pub(crate) n_accumulator: i32,
    pub(crate) a_func: *mut AggInfoFunc,
    pub(crate) n_func: i32,
    pub(crate) sel_id: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct AggInfoCol {
    pub(crate) p_tab: *mut Table,
    pub(crate) p_c_expr: *mut Expr,
    pub(crate) i_table: i32,
    pub(crate) i_column: i32,
    pub(crate) i_sorter_column: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct AggInfoFunc {
    pub(crate) p_f_expr: *mut Expr,
    pub(crate) p_func: *mut FuncDef,
    pub(crate) i_distinct: i32,
    pub(crate) i_dist_addr: i32,
    pub(crate) i_ob_tab: i32,
    pub(crate) b_ob_payload: u8,
    pub(crate) b_ob_unique: u8,
    pub(crate) b_use_subtype: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ExprU3 {
    pub(crate) p_tab: *mut Table,
    pub(crate) p_win: *mut Window,
    pub(crate) n_reg: i32,
    pub(crate) sub: ExprU3S0,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ExprU3S0 {
    pub(crate) i_addr: i32,
    pub(crate) reg_return: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union TableU0 {
    pub(crate) tab: TableU0S0,
    pub(crate) view: TableU0S1,
    pub(crate) vtab: TableU0S2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TableU0S0 {
    pub(crate) add_col_offset: i32,
    pub(crate) p_f_key: *mut FKey,
    pub(crate) p_dflt_list: *mut ExprList,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FKey {
    pub(crate) p_from: *mut Table,
    pub(crate) p_next_from: *mut FKey,
    pub(crate) z_to: *mut i8,
    pub(crate) p_next_to: *mut FKey,
    pub(crate) p_prev_to: *mut FKey,
    pub(crate) n_col: i32,
    pub(crate) is_deferred: u8,
    pub(crate) a_action: [u8; 2],
    pub(crate) ap_trigger: [*mut Trigger; 2],
    pub(crate) a_col: [SColMap; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Trigger {
    pub(crate) z_name: *mut i8,
    pub(crate) table: *mut i8,
    pub(crate) op: u8,
    pub(crate) tr_tm: u8,
    pub(crate) b_returning: u8,
    pub(crate) p_when: *mut Expr,
    pub(crate) p_columns: *mut IdList,
    pub(crate) p_schema: *mut Schema,
    pub(crate) p_tab_schema: *mut Schema,
    pub(crate) step_list: *mut TriggerStep,
    pub(crate) p_next: *mut Trigger,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TriggerStep {
    pub(crate) op: u8,
    pub(crate) orconf: u8,
    pub(crate) p_trig: *mut Trigger,
    pub(crate) p_select: *mut Select,
    pub(crate) p_src: *mut SrcList,
    pub(crate) p_where: *mut Expr,
    pub(crate) p_expr_list: *mut ExprList,
    pub(crate) p_id_list: *mut IdList,
    pub(crate) p_upsert: *mut Upsert,
    pub(crate) z_span: *mut i8,
    pub(crate) p_next: *mut TriggerStep,
    pub(crate) p_last: *mut TriggerStep,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Upsert {
    pub(crate) p_upsert_target: *mut ExprList,
    pub(crate) p_upsert_target_where: *mut Expr,
    pub(crate) p_upsert_set: *mut ExprList,
    pub(crate) p_upsert_where: *mut Expr,
    pub(crate) p_next_upsert: *mut Upsert,
    pub(crate) is_do_update: u8,
    pub(crate) is_dup: u8,
    pub(crate) p_to_free: *mut (),
    pub(crate) p_upsert_idx: *mut Index,
    pub(crate) p_upsert_src: *mut SrcList,
    pub(crate) reg_data: i32,
    pub(crate) i_data_cur: i32,
    pub(crate) i_idx_cur: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SColMap {
    pub(crate) i_from: i32,
    pub(crate) z_col: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TableU0S1 {
    pub(crate) p_select: *mut Select,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TableU0S2 {
    pub(crate) n_arg: i32,
    pub(crate) az_arg: *mut *mut i8,
    pub(crate) p: *mut VTable,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VTable {
    pub(crate) db: *mut Sqlite3,
    pub(crate) p_mod: *mut Module,
    pub(crate) p_vtab: *mut Sqlite3Vtab,
    pub(crate) n_ref: i32,
    pub(crate) b_constraint: u8,
    pub(crate) b_all_schemas: u8,
    pub(crate) e_vtab_risk: u8,
    pub(crate) i_savepoint: i32,
    pub(crate) p_next: *mut VTable,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Module {
    pub(crate) p_module: *const Sqlite3Module,
    pub(crate) z_name: *const i8,
    pub(crate) n_ref_module: i32,
    pub(crate) p_aux: *mut (),
    pub(crate) x_destroy: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) p_epo_tab: *mut Table,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3InitInfo {
    pub(crate) new_tnum: Pgno,
    pub(crate) i_db: u8,
    pub(crate) busy: u8,
    pub(crate) _bitfield_1: u32,
    pub(crate) az_init: *mut *const i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union Sqlite3U0 {
    pub(crate) x_legacy: Option<unsafe extern "C" fn(*mut (), *const i8)
        -> ()>,
    pub(crate) x_v2: Option<unsafe extern "C" fn(u32, *mut (), *mut (),
        *mut ()) -> i32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Parse {
    pub(crate) db: *mut Sqlite3,
    pub(crate) z_err_msg: *mut i8,
    pub(crate) p_vdbe: *mut Vdbe,
    pub(crate) rc: i32,
    pub(crate) n_query_loop: LogEst,
    pub(crate) nested: u8,
    pub(crate) n_temp_reg: u8,
    pub(crate) is_multi_write: u8,
    pub(crate) disable_lookaside: u8,
    pub(crate) prep_flags: u8,
    pub(crate) within_rj_subrtn: u8,
    pub(crate) m_subrtn_sig: u8,
    pub(crate) e_trigger_op: u8,
    pub(crate) e_orconf: u8,
    pub(crate) _bitfield_1: u32,
    pub(crate) n_range_reg: i32,
    pub(crate) i_range_reg: i32,
    pub(crate) n_err: i32,
    pub(crate) n_tab: i32,
    pub(crate) n_mem: i32,
    pub(crate) i_self_tab: i32,
    pub(crate) n_nest_sel: i32,
    pub(crate) n_label: i32,
    pub(crate) n_label_alloc: i32,
    pub(crate) a_label: *mut i32,
    pub(crate) p_const_expr: *mut ExprList,
    pub(crate) p_idx_epr: *mut IndexedExpr,
    pub(crate) p_idx_part_expr: *mut IndexedExpr,
    pub(crate) write_mask: YDbMask,
    pub(crate) cookie_mask: YDbMask,
    pub(crate) n_max_arg: i32,
    pub(crate) n_select: i32,
    pub(crate) n_progress_steps: u32,
    pub(crate) n_table_lock: i32,
    pub(crate) p_toplevel: *mut Parse,
    pub(crate) p_trigger_tab: *mut Table,
    pub(crate) p_trigger_prg: *mut TriggerPrg,
    pub(crate) p_cleanup: *mut ParseCleanup,
    pub(crate) a_temp_reg: [i32; 8],
    pub(crate) p_outer_parse: *mut Parse,
    pub(crate) s_name_token: Token,
    pub(crate) oldmask: u32,
    pub(crate) newmask: u32,
    pub(crate) u1: ParseU0,
    pub(crate) p_ainc: *mut AutoincInfo,
    pub(crate) a_table_lock: *mut TableLock,
    pub(crate) s_last_token: Token,
    pub(crate) n_var: YnVar,
    pub(crate) i_pk_sort_order: u8,
    pub(crate) explain: u8,
    pub(crate) e_parse_mode: u8,
    pub(crate) n_vtab_lock: i32,
    pub(crate) n_height: i32,
    pub(crate) addr_explain: i32,
    pub(crate) p_v_list: *mut VList,
    pub(crate) p_reprepare: *mut Vdbe,
    pub(crate) z_tail: *const i8,
    pub(crate) p_new_table: *mut Table,
    pub(crate) p_new_index: *mut Index,
    pub(crate) p_new_trigger: *mut Trigger,
    pub(crate) z_auth_context: *const i8,
    pub(crate) s_arg: Token,
    pub(crate) ap_vtab_lock: *mut *mut Table,
    pub(crate) p_with: *mut With,
    pub(crate) p_rename: *mut RenameToken,
}
pub(crate) type Bft = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IndexedExpr {
    pub(crate) p_expr: *mut Expr,
    pub(crate) i_data_cur: i32,
    pub(crate) i_idx_cur: i32,
    pub(crate) i_idx_col: i32,
    pub(crate) b_maybe_null_row: u8,
    pub(crate) aff: u8,
    pub(crate) p_ie_next: *mut IndexedExpr,
}
pub(crate) type YDbMask = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TriggerPrg {
    pub(crate) p_trigger: *mut Trigger,
    pub(crate) p_next: *mut TriggerPrg,
    pub(crate) p_program: *mut SubProgram,
    pub(crate) orconf: i32,
    pub(crate) a_colmask: [u32; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct KeyInfo {
    pub(crate) n_ref: u32,
    pub(crate) enc: u8,
    pub(crate) n_key_field: u16,
    pub(crate) n_all_field: u16,
    pub(crate) db: *mut Sqlite3,
    pub(crate) a_sort_flags: *mut u8,
    pub(crate) a_coll: [*mut CollSeq; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ParseCleanup {
    pub(crate) p_next: *mut ParseCleanup,
    pub(crate) p_ptr: *mut (),
    pub(crate) x_cleanup: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ())
        -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Token {
    pub(crate) z: *const i8,
    pub(crate) n: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ParseU0 {
    pub(crate) cr: ParseU0S0,
    pub(crate) d: ParseU0S1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ParseU0S0 {
    pub(crate) addr_cr_tab: i32,
    pub(crate) reg_rowid: i32,
    pub(crate) reg_root: i32,
    pub(crate) constraint_name: Token,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ParseU0S1 {
    pub(crate) p_returning: *mut Returning,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Returning {
    pub(crate) p_parse: *mut Parse,
    pub(crate) p_return_el: *mut ExprList,
    pub(crate) ret_trig: Trigger,
    pub(crate) ret_t_step: TriggerStep,
    pub(crate) i_ret_cur: i32,
    pub(crate) n_ret_col: i32,
    pub(crate) i_ret_reg: i32,
    pub(crate) z_name: [i8; 40],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct AutoincInfo {
    pub(crate) p_next: *mut AutoincInfo,
    pub(crate) p_tab: *mut Table,
    pub(crate) i_db: i32,
    pub(crate) reg_ctr: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TableLock {
    pub(crate) _opaque: [u8; 0],
}
pub(crate) type VList = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct RenameToken {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union Sqlite3U1 {
    pub(crate) is_interrupted: i32,
    pub(crate) not_used1: f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Lookaside {
    pub(crate) b_disable: u32,
    pub(crate) sz: u16,
    pub(crate) sz_true: u16,
    pub(crate) b_malloced: u8,
    pub(crate) n_slot: u32,
    pub(crate) an_stat: [u32; 3],
    pub(crate) p_init: *mut LookasideSlot,
    pub(crate) p_free: *mut LookasideSlot,
    pub(crate) p_small_init: *mut LookasideSlot,
    pub(crate) p_small_free: *mut LookasideSlot,
    pub(crate) p_middle: *mut (),
    pub(crate) p_start: *mut (),
    pub(crate) p_end: *mut (),
    pub(crate) p_true_end: *mut (),
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct LookasideSlot {
    pub(crate) p_next: *mut LookasideSlot,
}
pub(crate) type Sqlite3Xauth =
    unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, *const i8,
        *const i8) -> i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VtabCtx {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct BusyHandler {
    pub(crate) x_busy_handler: Option<unsafe extern "C" fn(*mut (), i32)
        -> i32>,
    pub(crate) p_busy_arg: *mut (),
    pub(crate) n_busy: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Savepoint {
    pub(crate) z_name: *mut i8,
    pub(crate) n_deferred_cons: i64,
    pub(crate) n_deferred_imm_cons: i64,
    pub(crate) p_next: *mut Savepoint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct DbClientData {
    pub(crate) p_next: *mut DbClientData,
    pub(crate) p_data: *mut (),
    pub(crate) x_destructor: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) z_name: [i8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Str {
    pub(crate) db: *mut Sqlite3,
    pub(crate) z_text: *mut i8,
    pub(crate) n_alloc: u32,
    pub(crate) mx_alloc: u32,
    pub(crate) n_char: u32,
    pub(crate) acc_error: u8,
    pub(crate) printf_flags: u8,
}
pub(crate) type TRowcnt = u64;
pub(crate) type Uptr = u64;
#[unsafe(no_mangle)]
pub static mut sqlite3_tree_trace: u32 = 0 as u32;
#[unsafe(no_mangle)]
pub static mut sqlite3_where_trace: u32 = 0 as u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct AuthContext {
    pub(crate) z_auth_context: *const i8,
    pub(crate) p_parse: *mut Parse,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Bitvec {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct DbFixer {
    pub(crate) p_parse: *mut Parse,
    pub(crate) w: Walker,
    pub(crate) p_schema: *mut Schema,
    pub(crate) b_temp: u8,
    pub(crate) z_db: *const i8,
    pub(crate) z_type: *const i8,
    pub(crate) p_name: *const Token,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Walker {
    pub(crate) p_parse: *mut Parse,
    pub(crate) x_expr_callback: Option<unsafe extern "C" fn(*mut Walker,
        *mut Expr) -> i32>,
    pub(crate) x_select_callback: Option<unsafe extern "C" fn(*mut Walker,
        *mut Select) -> i32>,
    pub(crate) x_select_callback2: Option<unsafe extern "C" fn(*mut Walker,
        *mut Select) -> ()>,
    pub(crate) walker_depth: i32,
    pub(crate) e_code: u16,
    pub(crate) m_w_flags: u16,
    pub(crate) u: WalkerU0,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union WalkerU0 {
    pub(crate) p_nc: *mut NameContext,
    pub(crate) n: i32,
    pub(crate) i_cur: i32,
    pub(crate) sz: i32,
    pub(crate) p_src_list: *mut SrcList,
    pub(crate) p_c_cur_hint: *mut CCurHint,
    pub(crate) p_ref_src_list: *mut RefSrcList,
    pub(crate) ai_col: *mut i32,
    pub(crate) p_idx_cover: *mut IdxCover,
    pub(crate) p_group_by: *mut ExprList,
    pub(crate) p_select: *mut Select,
    pub(crate) p_rewrite: *mut WindowRewrite,
    pub(crate) p_const: *mut WhereConst,
    pub(crate) p_rename: *mut RenameCtx,
    pub(crate) p_tab: *mut Table,
    pub(crate) p_cov_idx_ck: *mut CoveringIndexCheck,
    pub(crate) p_src_item: *mut SrcItem,
    pub(crate) p_fix: *mut DbFixer,
    pub(crate) a_mem: *mut Mem,
    pub(crate) p_check_on_ctx: *mut CheckOnCtx,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct NameContext {
    pub(crate) p_parse: *mut Parse,
    pub(crate) p_src_list: *mut SrcList,
    pub(crate) u_nc: NameContextU0,
    pub(crate) p_next: *mut NameContext,
    pub(crate) n_ref: i32,
    pub(crate) n_nc_err: i32,
    pub(crate) nc_flags: i32,
    pub(crate) n_nested_select: u32,
    pub(crate) p_win_select: *mut Select,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union NameContextU0 {
    pub(crate) p_e_list: *mut ExprList,
    pub(crate) p_agg_info: *mut AggInfo,
    pub(crate) p_upsert: *mut Upsert,
    pub(crate) i_base_reg: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FpDecode {
    pub(crate) n: i32,
    pub(crate) i_dp: i32,
    pub(crate) z: *mut i8,
    pub(crate) z_buf: [i8; 21],
    pub(crate) sign: i8,
    pub(crate) is_special: i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct FuncDefHash {
    pub(crate) a: [*mut FuncDef; 23],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct IndexSample {
    pub(crate) p: *mut (),
    pub(crate) n: i32,
    pub(crate) an_eq: *mut TRowcnt,
    pub(crate) an_lt: *mut TRowcnt,
    pub(crate) an_d_lt: *mut TRowcnt,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct KeyClass {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct OnOrUsing {
    pub(crate) p_on: *mut Expr,
    pub(crate) p_using: *mut IdList,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct PreUpdate {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct PrintfArguments {
    pub(crate) n_arg: i32,
    pub(crate) n_used: i32,
    pub(crate) ap_arg: *mut *mut Sqlite3Value,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct RCStr {
    pub(crate) n_rc_ref: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct RowSet {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SQLiteThread {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SelectDest {
    pub(crate) e_dest: u8,
    pub(crate) i_sd_parm: i32,
    pub(crate) i_sd_parm2: i32,
    pub(crate) i_sdst: i32,
    pub(crate) n_sdst: i32,
    pub(crate) z_aff_sdst: *mut i8,
    pub(crate) p_order_by: *mut ExprList,
}
pub(crate) type StrAccum = Sqlite3Str;
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TreeView {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct UnpackedRecord {
    pub(crate) p_key_info: *mut KeyInfo,
    pub(crate) a_mem: *mut Mem,
    pub(crate) u: UnpackedRecordU0,
    pub(crate) n: i32,
    pub(crate) n_field: u16,
    pub(crate) default_rc: i8,
    pub(crate) err_code: u8,
    pub(crate) r1: i8,
    pub(crate) r2: i8,
    pub(crate) eq_seen: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union UnpackedRecordU0 {
    pub(crate) z: *mut i8,
    pub(crate) i: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereInfo {
    pub(crate) _opaque: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct InitData {
    pub(crate) db: *mut Sqlite3,
    pub(crate) pz_err_msg: *mut *mut i8,
    pub(crate) i_db: i32,
    pub(crate) rc: i32,
    pub(crate) m_init_flags: u32,
    pub(crate) n_init_row: u32,
    pub(crate) mx_page: Pgno,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Config {
    pub(crate) b_memstat: i32,
    pub(crate) b_core_mutex: u8,
    pub(crate) b_full_mutex: u8,
    pub(crate) b_open_uri: u8,
    pub(crate) b_use_cis: u8,
    pub(crate) b_small_malloc: u8,
    pub(crate) b_extra_schema_checks: u8,
    pub(crate) mx_strlen: i32,
    pub(crate) never_corrupt: i32,
    pub(crate) sz_lookaside: i32,
    pub(crate) n_lookaside: i32,
    pub(crate) n_stmt_spill: i32,
    pub(crate) m: Sqlite3MemMethods,
    pub(crate) mutex: Sqlite3MutexMethods,
    pub(crate) pcache2: Sqlite3PcacheMethods2,
    pub(crate) p_heap: *mut (),
    pub(crate) n_heap: i32,
    pub(crate) mn_req: i32,
    pub(crate) mx_req: i32,
    pub(crate) sz_mmap: Sqlite3Int64,
    pub(crate) mx_mmap: Sqlite3Int64,
    pub(crate) p_page: *mut (),
    pub(crate) sz_page: i32,
    pub(crate) n_page: i32,
    pub(crate) mx_parser_stack: i32,
    pub(crate) shared_cache_enabled: i32,
    pub(crate) sz_pma: u32,
    pub(crate) is_init: i32,
    pub(crate) in_progress: i32,
    pub(crate) is_mutex_init: i32,
    pub(crate) is_malloc_init: i32,
    pub(crate) is_p_cache_init: i32,
    pub(crate) n_ref_init_mutex: i32,
    pub(crate) p_init_mutex: *mut Sqlite3Mutex,
    pub(crate) x_log: Option<unsafe extern "C" fn(*mut (), i32, *const i8)
        -> ()>,
    pub(crate) p_log_arg: *mut (),
    pub(crate) mx_memdb_size: Sqlite3Int64,
    pub(crate) x_test_callback: Option<unsafe extern "C" fn(i32) -> i32>,
    pub(crate) b_localtime_fault: i32,
    pub(crate) x_alt_localtime: Option<unsafe extern "C" fn(*const (),
        *mut ()) -> i32>,
    pub(crate) i_once_reset_threshold: i32,
    pub(crate) sz_sorter_ref: u32,
    pub(crate) i_prng_seed: u32,
}
#[unsafe(no_mangle)]
pub static sqlite3_opcode_property: [u8; 192] =
    [0 as u8, 0 as u8, 0 as u8, 0 as u8, 16 as u8, 0 as u8, 65 as u8, 0 as u8,
            129 as u8, 1 as u8, 1 as u8, 129 as u8, 131 as u8, 131 as u8,
            1 as u8, 1 as u8, 3 as u8, 3 as u8, 1 as u8, 18 as u8, 1 as u8,
            201 as u8, 201 as u8, 201 as u8, 201 as u8, 1 as u8, 73 as u8,
            73 as u8, 73 as u8, 73 as u8, 201 as u8, 73 as u8, 193 as u8,
            1 as u8, 65 as u8, 65 as u8, 193 as u8, 1 as u8, 1 as u8,
            65 as u8, 65 as u8, 65 as u8, 65 as u8, 38 as u8, 38 as u8,
            65 as u8, 65 as u8, 9 as u8, 35 as u8, 11 as u8, 129 as u8,
            3 as u8, 3 as u8, 11 as u8, 11 as u8, 11 as u8, 11 as u8,
            11 as u8, 11 as u8, 1 as u8, 1 as u8, 3 as u8, 3 as u8, 3 as u8,
            1 as u8, 65 as u8, 1 as u8, 0 as u8, 0 as u8, 2 as u8, 2 as u8,
            8 as u8, 0 as u8, 16 as u8, 16 as u8, 16 as u8, 0 as u8, 16 as u8,
            0 as u8, 16 as u8, 16 as u8, 0 as u8, 0 as u8, 16 as u8, 16 as u8,
            0 as u8, 0 as u8, 0 as u8, 2 as u8, 2 as u8, 2 as u8, 0 as u8,
            0 as u8, 18 as u8, 30 as u8, 32 as u8, 64 as u8, 0 as u8, 0 as u8,
            0 as u8, 16 as u8, 16 as u8, 0 as u8, 38 as u8, 38 as u8,
            38 as u8, 38 as u8, 38 as u8, 38 as u8, 38 as u8, 38 as u8,
            38 as u8, 38 as u8, 64 as u8, 64 as u8, 18 as u8, 0 as u8,
            64 as u8, 16 as u8, 64 as u8, 64 as u8, 0 as u8, 0 as u8, 0 as u8,
            64 as u8, 0 as u8, 64 as u8, 64 as u8, 16 as u8, 16 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 64 as u8, 0 as u8,
            80 as u8, 0 as u8, 64 as u8, 4 as u8, 4 as u8, 0 as u8, 64 as u8,
            80 as u8, 64 as u8, 16 as u8, 0 as u8, 0 as u8, 16 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 16 as u8, 0 as u8, 0 as u8, 0 as u8,
            6 as u8, 16 as u8, 0 as u8, 4 as u8, 26 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 64 as u8, 16 as u8, 80 as u8, 64 as u8,
            0 as u8, 16 as u8, 16 as u8, 2 as u8, 18 as u8, 18 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8];
#[unsafe(no_mangle)]
pub static sqlite3_str_binary: [i8; 7] =
    [66 as i8, 73 as i8, 78 as i8, 65 as i8, 82 as i8, 89 as i8, 0 as i8];
#[unsafe(no_mangle)]
pub static sqlite3_std_type_len: [u8; 6] =
    [3 as u8, 4 as u8, 3 as u8, 7 as u8, 4 as u8, 4 as u8];
#[unsafe(no_mangle)]
pub static sqlite3_std_type_affinity: [i8; 6] =
    [67 as i8, 65 as i8, 68 as i8, 68 as i8, 69 as i8, 66 as i8];
#[unsafe(no_mangle)]
pub static mut sqlite3_std_type: [*const i8; 6] =
    [c"ANY".as_ptr() as *const i8, c"BLOB".as_ptr() as *const i8,
            c"INT".as_ptr() as *const i8, c"INTEGER".as_ptr() as *const i8,
            c"REAL".as_ptr() as *const i8, c"TEXT".as_ptr() as *const i8];
#[unsafe(no_mangle)]
pub static sqlite3_upper_to_lower: [u8; 274] =
    [0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 7 as u8,
            8 as u8, 9 as u8, 10 as u8, 11 as u8, 12 as u8, 13 as u8,
            14 as u8, 15 as u8, 16 as u8, 17 as u8, 18 as u8, 19 as u8,
            20 as u8, 21 as u8, 22 as u8, 23 as u8, 24 as u8, 25 as u8,
            26 as u8, 27 as u8, 28 as u8, 29 as u8, 30 as u8, 31 as u8,
            32 as u8, 33 as u8, 34 as u8, 35 as u8, 36 as u8, 37 as u8,
            38 as u8, 39 as u8, 40 as u8, 41 as u8, 42 as u8, 43 as u8,
            44 as u8, 45 as u8, 46 as u8, 47 as u8, 48 as u8, 49 as u8,
            50 as u8, 51 as u8, 52 as u8, 53 as u8, 54 as u8, 55 as u8,
            56 as u8, 57 as u8, 58 as u8, 59 as u8, 60 as u8, 61 as u8,
            62 as u8, 63 as u8, 64 as u8, 97 as u8, 98 as u8, 99 as u8,
            100 as u8, 101 as u8, 102 as u8, 103 as u8, 104 as u8, 105 as u8,
            106 as u8, 107 as u8, 108 as u8, 109 as u8, 110 as u8, 111 as u8,
            112 as u8, 113 as u8, 114 as u8, 115 as u8, 116 as u8, 117 as u8,
            118 as u8, 119 as u8, 120 as u8, 121 as u8, 122 as u8, 91 as u8,
            92 as u8, 93 as u8, 94 as u8, 95 as u8, 96 as u8, 97 as u8,
            98 as u8, 99 as u8, 100 as u8, 101 as u8, 102 as u8, 103 as u8,
            104 as u8, 105 as u8, 106 as u8, 107 as u8, 108 as u8, 109 as u8,
            110 as u8, 111 as u8, 112 as u8, 113 as u8, 114 as u8, 115 as u8,
            116 as u8, 117 as u8, 118 as u8, 119 as u8, 120 as u8, 121 as u8,
            122 as u8, 123 as u8, 124 as u8, 125 as u8, 126 as u8, 127 as u8,
            128 as u8, 129 as u8, 130 as u8, 131 as u8, 132 as u8, 133 as u8,
            134 as u8, 135 as u8, 136 as u8, 137 as u8, 138 as u8, 139 as u8,
            140 as u8, 141 as u8, 142 as u8, 143 as u8, 144 as u8, 145 as u8,
            146 as u8, 147 as u8, 148 as u8, 149 as u8, 150 as u8, 151 as u8,
            152 as u8, 153 as u8, 154 as u8, 155 as u8, 156 as u8, 157 as u8,
            158 as u8, 159 as u8, 160 as u8, 161 as u8, 162 as u8, 163 as u8,
            164 as u8, 165 as u8, 166 as u8, 167 as u8, 168 as u8, 169 as u8,
            170 as u8, 171 as u8, 172 as u8, 173 as u8, 174 as u8, 175 as u8,
            176 as u8, 177 as u8, 178 as u8, 179 as u8, 180 as u8, 181 as u8,
            182 as u8, 183 as u8, 184 as u8, 185 as u8, 186 as u8, 187 as u8,
            188 as u8, 189 as u8, 190 as u8, 191 as u8, 192 as u8, 193 as u8,
            194 as u8, 195 as u8, 196 as u8, 197 as u8, 198 as u8, 199 as u8,
            200 as u8, 201 as u8, 202 as u8, 203 as u8, 204 as u8, 205 as u8,
            206 as u8, 207 as u8, 208 as u8, 209 as u8, 210 as u8, 211 as u8,
            212 as u8, 213 as u8, 214 as u8, 215 as u8, 216 as u8, 217 as u8,
            218 as u8, 219 as u8, 220 as u8, 221 as u8, 222 as u8, 223 as u8,
            224 as u8, 225 as u8, 226 as u8, 227 as u8, 228 as u8, 229 as u8,
            230 as u8, 231 as u8, 232 as u8, 233 as u8, 234 as u8, 235 as u8,
            236 as u8, 237 as u8, 238 as u8, 239 as u8, 240 as u8, 241 as u8,
            242 as u8, 243 as u8, 244 as u8, 245 as u8, 246 as u8, 247 as u8,
            248 as u8, 249 as u8, 250 as u8, 251 as u8, 252 as u8, 253 as u8,
            254 as u8, 255 as u8, 1 as u8, 0 as u8, 0 as u8, 1 as u8, 1 as u8,
            0 as u8, 0 as u8, 1 as u8, 0 as u8, 1 as u8, 0 as u8, 1 as u8,
            1 as u8, 0 as u8, 1 as u8, 0 as u8, 0 as u8, 1 as u8];
#[unsafe(no_mangle)]
pub static mut sqlite3a_l_tb: *const u8 =
    &sqlite3_upper_to_lower[(256 - 53) as usize];
#[unsafe(no_mangle)]
pub static mut sqlite3a_e_qb: *const u8 =
    &sqlite3_upper_to_lower[(256 + 6 - 53) as usize];
#[unsafe(no_mangle)]
pub static mut sqlite3a_g_tb: *const u8 =
    &sqlite3_upper_to_lower[(256 + 12 - 53) as usize];
#[unsafe(no_mangle)]
pub static sqlite3_ctype_map: [u8; 256] =
    [0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 1 as u8, 0 as u8, 128 as u8, 0 as u8,
            64 as u8, 0 as u8, 0 as u8, 128 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8, 12 as u8,
            12 as u8, 12 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 10 as u8, 10 as u8, 10 as u8, 10 as u8,
            10 as u8, 10 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 128 as u8, 0 as u8, 0 as u8, 0 as u8, 64 as u8,
            128 as u8, 42 as u8, 42 as u8, 42 as u8, 42 as u8, 42 as u8,
            42 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8,
            34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8,
            34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8, 34 as u8,
            34 as u8, 34 as u8, 34 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8,
            64 as u8, 64 as u8, 64 as u8];
#[unsafe(no_mangle)]
pub static mut sqlite3Config: Sqlite3Config =
    Sqlite3Config {
        b_memstat: 1,
        b_core_mutex: 1 as u8,
        b_full_mutex: (1 == 1) as u8,
        b_open_uri: 0 as u8,
        b_use_cis: 1 as u8,
        b_small_malloc: 0 as u8,
        b_extra_schema_checks: 1 as u8,
        mx_strlen: 2147483646,
        never_corrupt: 0,
        sz_lookaside: 1200,
        n_lookaside: 40,
        n_stmt_spill: 64 * 1024,
        m: Sqlite3MemMethods {
            x_malloc: None,
            x_free: None,
            x_realloc: None,
            x_size: None,
            x_roundup: None,
            x_init: None,
            x_shutdown: None,
            p_app_data: core::ptr::null_mut(),
        },
        mutex: Sqlite3MutexMethods {
            x_mutex_init: None,
            x_mutex_end: None,
            x_mutex_alloc: None,
            x_mutex_free: None,
            x_mutex_enter: None,
            x_mutex_try: None,
            x_mutex_leave: None,
            x_mutex_held: None,
            x_mutex_notheld: None,
        },
        pcache2: Sqlite3PcacheMethods2 {
            i_version: 0,
            p_arg: core::ptr::null_mut(),
            x_init: None,
            x_shutdown: None,
            x_create: None,
            x_cachesize: None,
            x_pagecount: None,
            x_fetch: None,
            x_unpin: None,
            x_rekey: None,
            x_truncate: None,
            x_destroy: None,
            x_shrink: None,
        },
        p_heap: 0 as *mut (),
        n_heap: 0,
        mn_req: 0,
        mx_req: 0,
        sz_mmap: 0 as Sqlite3Int64,
        mx_mmap: 2147418112 as Sqlite3Int64,
        p_page: 0 as *mut (),
        sz_page: 0,
        n_page: 20,
        mx_parser_stack: 0,
        shared_cache_enabled: 0,
        sz_pma: 250 as u32,
        is_init: 0,
        in_progress: 0,
        is_mutex_init: 0,
        is_malloc_init: 0,
        is_p_cache_init: 0,
        n_ref_init_mutex: 0,
        p_init_mutex: core::ptr::null_mut(),
        x_log: None,
        p_log_arg: core::ptr::null_mut(),
        mx_memdb_size: 1073741824 as Sqlite3Int64,
        x_test_callback: None,
        b_localtime_fault: 0,
        x_alt_localtime: None,
        i_once_reset_threshold: 2147483646,
        sz_sorter_ref: 2147483647 as u32,
        i_prng_seed: 0 as u32,
    };
#[unsafe(no_mangle)]
pub static mut sqlite3_builtin_functions: FuncDefHash =
    unsafe { core::mem::zeroed() };
#[unsafe(no_mangle)]
pub static mut sqlite3_oom_str: Sqlite3Str =
    Sqlite3Str {
        db: core::ptr::null_mut(),
        z_text: core::ptr::null_mut(),
        n_alloc: 0 as u32,
        mx_alloc: 0 as u32,
        n_char: 0 as u32,
        acc_error: 7 as u8,
        printf_flags: 0 as u8,
    };
#[unsafe(no_mangle)]
pub static mut sqlite3_pending_byte: i32 = 1073741824;