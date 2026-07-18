use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereClause {
    pub(crate) p_w_info: *mut WhereInfo,
    pub(crate) p_outer: *mut WhereClause,
    pub(crate) op: u8,
    pub(crate) has_or: u8,
    pub(crate) n_term: i32,
    pub(crate) n_slot: i32,
    pub(crate) n_base: i32,
    pub(crate) a: *mut WhereTerm,
    pub(crate) a_static: [WhereTerm; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereInfo {
    pub(crate) p_parse: *mut Parse,
    pub(crate) p_tab_list: *mut SrcList,
    pub(crate) p_order_by: *mut ExprList,
    pub(crate) p_result_set: *mut ExprList,
    pub(crate) p_select: *mut Select,
    pub(crate) ai_cur_one_pass: [i32; 2],
    pub(crate) i_continue: i32,
    pub(crate) i_break: i32,
    pub(crate) saved_n_query_loop: i32,
    pub(crate) wctrl_flags: u16,
    pub(crate) i_limit: LogEst,
    pub(crate) n_level: u8,
    pub(crate) n_ob_sat: i8,
    pub(crate) e_one_pass: u8,
    pub(crate) e_distinct: u8,
    pub(crate) _bitfield_1: u32,
    pub(crate) n_row_out: LogEst,
    pub(crate) i_top: i32,
    pub(crate) i_end_where: i32,
    pub(crate) p_loops: *mut WhereLoop,
    pub(crate) p_mem_to_free: *mut WhereMemBlock,
    pub(crate) rev_mask: Bitmask,
    pub(crate) s_wc: WhereClause,
    pub(crate) s_mask_set: WhereMaskSet,
    pub(crate) a: [WhereLevel; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereLoop {
    pub(crate) prereq: Bitmask,
    pub(crate) mask_self: Bitmask,
    pub(crate) i_tab: u8,
    pub(crate) i_sort_idx: u8,
    pub(crate) r_setup: LogEst,
    pub(crate) r_run: LogEst,
    pub(crate) n_out: LogEst,
    pub(crate) u: WhereLoopU0,
    pub(crate) ws_flags: u32,
    pub(crate) n_l_term: u16,
    pub(crate) n_skip: u16,
    pub(crate) n_l_slot: u16,
    pub(crate) a_l_term: *mut *mut WhereTerm,
    pub(crate) p_next_loop: *mut WhereLoop,
    pub(crate) a_l_term_space: [*mut WhereTerm; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union WhereLoopU0 {
    pub(crate) btree: WhereLoopU0S0,
    pub(crate) vtab: WhereLoopU0S1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereLoopU0S0 {
    pub(crate) n_eq: u16,
    pub(crate) n_btm: u16,
    pub(crate) n_top: u16,
    pub(crate) n_distinct_col: u16,
    pub(crate) p_index: *mut Index,
    pub(crate) p_order_by: *mut ExprList,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereLoopU0S1 {
    pub(crate) idx_num: i32,
    pub(crate) _bitfield_1: u32,
    pub(crate) is_ordered: i8,
    pub(crate) omit_mask: u16,
    pub(crate) idx_str: *mut i8,
    pub(crate) m_handle_in: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereTerm {
    pub(crate) p_expr: *mut Expr,
    pub(crate) p_wc: *mut WhereClause,
    pub(crate) truth_prob: LogEst,
    pub(crate) wt_flags: u16,
    pub(crate) e_operator: u16,
    pub(crate) n_child: u8,
    pub(crate) e_match_op: u8,
    pub(crate) i_parent: i32,
    pub(crate) left_cursor: i32,
    pub(crate) u: WhereTermU0,
    pub(crate) prereq_right: Bitmask,
    pub(crate) prereq_all: Bitmask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union WhereTermU0 {
    pub(crate) x: WhereTermU0S0,
    pub(crate) p_or_info: *mut WhereOrInfo,
    pub(crate) p_and_info: *mut WhereAndInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereTermU0S0 {
    pub(crate) left_column: i32,
    pub(crate) i_field: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereOrInfo {
    pub(crate) wc: WhereClause,
    pub(crate) indexable: Bitmask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereAndInfo {
    pub(crate) wc: WhereClause,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereMemBlock {
    pub(crate) p_next: *mut WhereMemBlock,
    pub(crate) sz: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereMaskSet {
    pub(crate) b_var_select: i32,
    pub(crate) n: i32,
    pub(crate) ix: [i32; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereLevel {
    pub(crate) i_left_join: i32,
    pub(crate) i_tab_cur: i32,
    pub(crate) i_idx_cur: i32,
    pub(crate) addr_brk: i32,
    pub(crate) addr_halt: i32,
    pub(crate) addr_nxt: i32,
    pub(crate) addr_skip: i32,
    pub(crate) addr_cont: i32,
    pub(crate) addr_first: i32,
    pub(crate) addr_body: i32,
    pub(crate) reg_bignull: i32,
    pub(crate) addr_bignull: i32,
    pub(crate) i_like_rep_cntr: u32,
    pub(crate) addr_like_rep: i32,
    pub(crate) reg_filter: i32,
    pub(crate) p_rj: *mut WhereRightJoin,
    pub(crate) i_from: u8,
    pub(crate) op: u8,
    pub(crate) p3: u8,
    pub(crate) p5: u8,
    pub(crate) p1: i32,
    pub(crate) p2: i32,
    pub(crate) u: WhereLevelU0,
    pub(crate) p_w_loop: *mut WhereLoop,
    pub(crate) not_ready: Bitmask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereRightJoin {
    pub(crate) i_match: i32,
    pub(crate) reg_bloom: i32,
    pub(crate) reg_return: i32,
    pub(crate) addr_subrtn: i32,
    pub(crate) end_subrtn: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union WhereLevelU0 {
    pub(crate) in_: WhereLevelU0S0,
    pub(crate) p_covering_idx: *mut Index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereLevelU0S0 {
    pub(crate) n_in: i32,
    pub(crate) a_in_loop: *mut InLoop,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct InLoop {
    pub(crate) i_cur: i32,
    pub(crate) addr_in_top: i32,
    pub(crate) i_base: i32,
    pub(crate) n_prefix: i32,
    pub(crate) e_end_loop_op: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereLoopBuilder {
    pub(crate) p_w_info: *mut WhereInfo,
    pub(crate) p_wc: *mut WhereClause,
    pub(crate) p_new: *mut WhereLoop,
    pub(crate) p_or_set: *mut WhereOrSet,
    pub(crate) bld_flags1: u8,
    pub(crate) bld_flags2: u8,
    pub(crate) i_plan_limit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereOrSet {
    pub(crate) n: u16,
    pub(crate) a: [WhereOrCost; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereOrCost {
    pub(crate) prereq: Bitmask,
    pub(crate) r_run: LogEst,
    pub(crate) n_out: LogEst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereScan {
    pub(crate) p_orig_wc: *mut WhereClause,
    pub(crate) p_wc: *mut WhereClause,
    pub(crate) z_coll_name: *const i8,
    pub(crate) p_idx_expr: *mut Expr,
    pub(crate) k: i32,
    pub(crate) op_mask: u32,
    pub(crate) idxaff: i8,
    pub(crate) i_equiv: u8,
    pub(crate) n_equiv: u8,
    pub(crate) ai_cur: [i32; 11],
    pub(crate) ai_column: [i16; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WherePath {
    pub(crate) mask_loop: Bitmask,
    pub(crate) rev_loop: Bitmask,
    pub(crate) n_row: LogEst,
    pub(crate) r_cost: LogEst,
    pub(crate) r_unsort: LogEst,
    pub(crate) is_ordered: i8,
    pub(crate) a_loop: *mut *mut WhereLoop,
}
