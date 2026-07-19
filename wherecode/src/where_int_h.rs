use super::*;
use crate::sqlite_int_h::{
    Bitmask, Expr, ExprList, Index, LogEst, Parse, Select, SrcList,
};

///* The WHERE clause processing routine has two halves.  The
///* first part does the start of the WHERE loop and the second
///* half does the tail of the WHERE loop.  An instance of
///* this structure is returned by the first half and passed
///* into the second half to give some continuity.
///*
///* An instance of this object holds the complete state of the query
///* planner.
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

///* Each instance of this object represents an algorithm for evaluating one
///* term of a join.  Every term of the FROM clause will have at least
///* one corresponding WhereLoop object (unless INDEXED BY constraints
///* prevent a query solution - which is an error) and many terms of the
///* FROM clause will have multiple WhereLoop objects, each describing a
///* potential way of implementing that FROM-clause term, together with
///* dependencies and cost estimates for using the chosen algorithm.
///*
///* Query planning consists of building up a collection of these WhereLoop
///* objects, then computing a particular sequence of WhereLoop objects, with
///* one WhereLoop object per FROM clause term, that satisfy all dependencies
///* and that minimize the overall cost.
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

///* The query generator uses an array of instances of this structure to
///* help it analyze the subexpressions of the WHERE clause.  Each WHERE
///* clause subexpression is separated from the others by AND operators,
///* usually, or sometimes subexpressions separated by OR.
///*
///* All WhereTerms are collected into a single WhereClause structure. 
///* The following identity holds:
///*
///*        WhereTerm.pWC->a[WhereTerm.idx] == WhereTerm
///*
///* When a term is of the form:
///*
///*              X <op> <expr>
///*
///* where X is a column name and <op> is one of certain operators,
///* then WhereTerm.leftCursor and WhereTerm.u.leftColumn record the
///* cursor number and column number for X.  WhereTerm.eOperator records
///* the <op> using a bitmask encoding defined by WO_xxx below.  The
///* use of a bitmask encoding for the operator allows us to search
///* quickly for terms that match any of several different operators.
///*
///* A WhereTerm might also be two or more subterms connected by OR:
///*
///*         (t1.X <op> <expr>) OR (t1.Y <op> <expr>) OR ....
///*
///* In this second case, wtFlag has the TERM_ORINFO bit set and eOperator==WO_OR
///* and the WhereTerm.u.pOrInfo field points to auxiliary information that
///* is collected about the OR clause.
///*
///* If a term in the WHERE clause does not match either of the two previous
///* categories, then eOperator==0.  The WhereTerm.pExpr field is still set
///* to the original subexpression content and wtFlags is set up appropriately
///* but no other fields in the WhereTerm object are meaningful.
///*
///* When eOperator!=0, prereqRight and prereqAll record sets of cursor numbers,
///* but they do so indirectly.  A single WhereMaskSet structure translates
///* cursor number into bits and the translated bit is stored in the prereq
///* fields.  The translation is used in order to maximize the number of
///* bits that will fit in a Bitmask.  The VDBE cursor numbers might be
///* spread out over the non-negative integers.  For example, the cursor
///* numbers might be 3, 8, 9, 10, 20, 23, 41, and 45.  The WhereMaskSet
///* translates these sparse cursor numbers into consecutive integers
///* beginning with 0 in order to make the best possible use of the available
///* bits in the Bitmask.  So, in the example above, the cursor numbers
///* would be mapped into integers 0 through 7.
///*
///* The number of terms in a join is limited by the number of bits
///* in prereqRight and prereqAll.  The default is 64 bits, hence SQLite
///* is only able to process joins with 64 or fewer tables.
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

///* An instance of the following structure holds all information about a
///* WHERE clause.  Mostly this is a container for one or more WhereTerms.
///*
///* Explanation of pOuter:  For a WHERE clause of the form
///*
///*           a AND ((b AND c) OR (d AND e)) AND f
///*
///* There are separate WhereClause objects for the whole clause and for
///* the subclauses "(b AND c)" and "(d AND e)".  The pOuter field of the
///* subclauses points to the WhereClause object for the whole clause.
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

///* A WhereTerm with eOperator==WO_OR has its u.pOrInfo pointer set to
///* a dynamically allocated instance of the following structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereOrInfo {
    pub(crate) wc: WhereClause,
    pub(crate) indexable: Bitmask,
}

///* A WhereTerm with eOperator==WO_AND has its u.pAndInfo pointer set to
///* a dynamically allocated instance of the following structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereAndInfo {
    pub(crate) wc: WhereClause,
}

///* This object is a header on a block of allocated memory that will be
///* automatically freed when its WInfo object is destructed.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereMemBlock {
    pub(crate) p_next: *mut WhereMemBlock,
    pub(crate) sz: u64,
}

///* An instance of the following structure keeps track of a mapping
///* between VDBE cursor numbers and bits of the bitmasks in WhereTerm.
///*
///* The VDBE cursor numbers are small integers contained in
///* SrcItem.iCursor and Expr.iTable fields.  For any given WHERE
///* clause, the cursor numbers might not begin with 0 and they might
///* contain gaps in the numbering sequence.  But we want to make maximum
///* use of the bits in our bitmasks.  This structure provides a mapping
///* from the sparse cursor numbers into consecutive integers beginning
///* with 0.
///*
///* If WhereMaskSet.ix[A]==B it means that The A-th bit of a Bitmask
///* corresponds VDBE cursor number B.  The A-th bit of a bitmask is 1<<A.
///*
///* For example, if the WHERE clause expression used these VDBE
///* cursors:  4, 5, 8, 29, 57, 73.  Then the  WhereMaskSet structure
///* would map those cursor numbers into bits 0 through 5.
///*
///* Note that the mapping is not necessarily ordered.  In the example
///* above, the mapping might go like this:  4->3, 5->1, 8->2, 29->0,
///* 57->5, 73->4.  Or one of 719 other combinations might be used. It
///* does not really matter.  What is important is that sparse cursor
///* numbers all get mapped into bit numbers that begin with 0 and contain
///* no gaps.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereMaskSet {
    pub(crate) b_var_select: i32,
    pub(crate) n: i32,
    pub(crate) ix: [i32; 64],
}

///* This object contains information needed to implement a single nested
///* loop in WHERE clause.
///*
///* Contrast this object with WhereLoop.  This object describes the
///* implementation of the loop.  WhereLoop describes the algorithm.
///* This object contains a pointer to the WhereLoop algorithm as one of
///* its elements.
///*
///* The WhereInfo object contains a single instance of this object for
///* each term in the FROM clause (which is to say, for each of the
///* nested loops as implemented).  The order of WhereLevel objects determines
///* the loop nested order, with WhereInfo.a[0] being the outer loop and
///* WhereInfo.a[WhereInfo.nLevel-1] being the inner loop.
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

///* Extra information attached to a WhereLevel that is a RIGHT JOIN.
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

///* Each instance of this object holds a sequence of WhereLoop objects
///* that implement some or all of a query plan.
///*
///* Think of each WhereLoop object as a node in a graph with arcs
///* showing dependencies and costs for travelling between nodes.  (That is
///* not a completely accurate description because WhereLoop costs are a
///* vector, not a scalar, and because dependencies are many-to-one, not
///* one-to-one as are graph nodes.  But it is a useful visualization aid.)
///* Then a WherePath object is a path through the graph that visits some
///* or all of the WhereLoop objects once.
///*
///* The "solver" works by creating the N best WherePath objects of length
///* 1.  Then using those as a basis to compute the N best WherePath objects
///* of length 2.  And so forth until the length of WherePaths equals the
///* number of nodes in the FROM clause.  The best (lowest cost) WherePath
///* at the end is the chosen query plan.
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

///* This object is a convenience wrapper holding all information needed
///* to construct WhereLoop objects for a particular query.
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

/// This object holds the prerequisites and the cost of running a
///* subquery on one operand of an OR operator in the WHERE clause.
///* See WhereOrSet for additional information
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct WhereOrCost {
    pub(crate) prereq: Bitmask,
    pub(crate) r_run: LogEst,
    pub(crate) n_out: LogEst,
}

///* An instance of the WhereScan object is used as an iterator for locating
///* terms in the WHERE clause that are useful to the query planner.
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
