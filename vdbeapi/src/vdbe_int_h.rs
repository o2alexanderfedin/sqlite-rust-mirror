use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Vdbe {
    pub(crate) db: *mut Sqlite3,
    pub(crate) pp_v_prev: *mut *mut Vdbe,
    pub(crate) p_v_next: *mut Vdbe,
    pub(crate) p_parse: *mut Parse,
    pub(crate) n_var: YnVar,
    pub(crate) n_mem: i32,
    pub(crate) n_cursor: i32,
    pub(crate) cache_ctr: u32,
    pub(crate) pc: i32,
    pub(crate) rc: i32,
    pub(crate) n_change: i64,
    pub(crate) i_statement: i32,
    pub(crate) i_current_time: i64,
    pub(crate) n_fk_constraint: i64,
    pub(crate) n_stmt_def_cons: i64,
    pub(crate) n_stmt_def_imm_cons: i64,
    pub(crate) a_mem: *mut Mem,
    pub(crate) ap_arg: *mut *mut Mem,
    pub(crate) ap_csr: *mut *mut VdbeCursor,
    pub(crate) a_var: *mut Mem,
    pub(crate) a_op: *mut Op,
    pub(crate) n_op: i32,
    pub(crate) n_op_alloc: i32,
    pub(crate) a_col_name: *mut Mem,
    pub(crate) p_result_row: *mut Mem,
    pub(crate) z_err_msg: *mut i8,
    pub(crate) p_v_list: *mut VList,
    pub(crate) start_time: i64,
    pub(crate) n_res_column: u16,
    pub(crate) n_res_alloc: u16,
    pub(crate) error_action: u8,
    pub(crate) min_write_file_format: u8,
    pub(crate) prep_flags: u8,
    pub(crate) e_vdbe_state: u8,
    pub(crate) _bitfield_1: u32,
    pub(crate) btree_mask: YDbMask,
    pub(crate) lock_mask: YDbMask,
    pub(crate) a_counter: [u32; 9],
    pub(crate) z_sql: *mut i8,
    pub(crate) p_free: *mut (),
    pub(crate) p_frame: *mut VdbeFrame,
    pub(crate) p_del_frame: *mut VdbeFrame,
    pub(crate) n_frame: i32,
    pub(crate) expmask: u32,
    pub(crate) p_program: *mut SubProgram,
    pub(crate) p_aux_data: *mut AuxData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Value {
    pub(crate) u: MemValue,
    pub(crate) z: *mut i8,
    pub(crate) n: i32,
    pub(crate) flags: u16,
    pub(crate) enc: u8,
    pub(crate) e_subtype: u8,
    pub(crate) db: *mut Sqlite3,
    pub(crate) sz_malloc: i32,
    pub(crate) u_temp: u32,
    pub(crate) z_malloc: *mut i8,
    pub(crate) x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union MemValue {
    pub(crate) r: f64,
    pub(crate) i: i64,
    pub(crate) n_zero: i32,
    pub(crate) z_p_type: *const i8,
    pub(crate) p_def: *mut FuncDef,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Sqlite3Context {
    pub(crate) p_out: *mut Mem,
    pub(crate) p_func: *mut FuncDef,
    pub(crate) p_mem: *mut Mem,
    pub(crate) p_vdbe: *mut Vdbe,
    pub(crate) i_op: i32,
    pub(crate) is_error: i32,
    pub(crate) enc: u8,
    pub(crate) skip_flag: u8,
    pub(crate) argc: u16,
    pub(crate) argv: [*mut Sqlite3Value; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VdbeCursor {
    pub(crate) e_cur_type: u8,
    pub(crate) i_db: i8,
    pub(crate) null_row: u8,
    pub(crate) deferred_moveto: u8,
    pub(crate) is_table: u8,
    pub(crate) _bitfield_1: u32,
    pub(crate) seek_hit: u16,
    pub(crate) ub: VdbeCursorU0,
    pub(crate) seq_count: i64,
    pub(crate) cache_status: u32,
    pub(crate) seek_result: i32,
    pub(crate) p_alt_cursor: *mut VdbeCursor,
    pub(crate) uc: VdbeCursorU1,
    pub(crate) p_key_info: *mut KeyInfo,
    pub(crate) i_hdr_offset: u32,
    pub(crate) pgno_root: Pgno,
    pub(crate) n_field: i16,
    pub(crate) n_hdr_parsed: u16,
    pub(crate) moveto_target: i64,
    pub(crate) a_offset: *mut u32,
    pub(crate) a_row: *const u8,
    pub(crate) payload_size: u32,
    pub(crate) sz_row: u32,
    pub(crate) p_cache: *mut VdbeTxtBlbCache,
    pub(crate) a_type: [u32; 0],
}

pub(crate) type Bool = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union VdbeCursorU0 {
    pub(crate) p_btx: *mut Btree,
    pub(crate) a_alt_map: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union VdbeCursorU1 {
    pub(crate) p_cursor: *mut BtCursor,
    pub(crate) p_v_cur: *mut Sqlite3VtabCursor,
    pub(crate) p_sorter: *mut VdbeSorter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VdbeSorter {
    pub(crate) _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VdbeTxtBlbCache {
    pub(crate) p_c_value: *mut i8,
    pub(crate) i_offset: i64,
    pub(crate) i_col: i32,
    pub(crate) cache_status: u32,
    pub(crate) col_cache_ctr: u32,
}

pub(crate) type Op = VdbeOp;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VdbeFrame {
    pub(crate) v: *mut Vdbe,
    pub(crate) p_parent: *mut VdbeFrame,
    pub(crate) a_op: *mut Op,
    pub(crate) a_mem: *mut Mem,
    pub(crate) ap_csr: *mut *mut VdbeCursor,
    pub(crate) a_once: *mut u8,
    pub(crate) token: *mut (),
    pub(crate) last_rowid: i64,
    pub(crate) p_aux_data: *mut AuxData,
    pub(crate) n_cursor: i32,
    pub(crate) pc: i32,
    pub(crate) n_op: i32,
    pub(crate) n_mem: i32,
    pub(crate) n_child_mem: i32,
    pub(crate) n_child_csr: i32,
    pub(crate) n_change: i64,
    pub(crate) n_db_change: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct AuxData {
    pub(crate) i_aux_op: i32,
    pub(crate) i_aux_arg: i32,
    pub(crate) p_aux: *mut (),
    pub(crate) x_delete_aux: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) p_next_aux: *mut AuxData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ValueList {
    pub(crate) p_csr: *mut BtCursor,
    pub(crate) p_out: *mut Sqlite3Value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct PreUpdate {
    pub(crate) v: *mut Vdbe,
    pub(crate) p_csr: *mut VdbeCursor,
    pub(crate) op: i32,
    pub(crate) a_record: *mut u8,
    pub(crate) p_keyinfo: *mut KeyInfo,
    pub(crate) p_unpacked: *mut UnpackedRecord,
    pub(crate) p_new_unpacked: *mut UnpackedRecord,
    pub(crate) i_new_reg: i32,
    pub(crate) i_blob_write: i32,
    pub(crate) i_key1: i64,
    pub(crate) i_key2: i64,
    pub(crate) oldipk: Mem,
    pub(crate) a_new: *mut Mem,
    pub(crate) p_tab: *mut Table,
    pub(crate) p_pk: *mut Index,
    pub(crate) ap_dflt: *mut *mut Sqlite3Value,
    pub(crate) u_key: PreUpdateS0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct PreUpdateS0 {
    pub(crate) keyinfo_space: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ScanStatus {
    pub(crate) addr_explain: i32,
    pub(crate) a_addr_range: [i32; 6],
    pub(crate) addr_loop: i32,
    pub(crate) addr_visit: i32,
    pub(crate) i_select_id: i32,
    pub(crate) n_est: LogEst,
    pub(crate) z_name: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct DblquoteStr {
    pub(crate) p_next_str: *mut DblquoteStr,
    pub(crate) z: [i8; 8],
}
