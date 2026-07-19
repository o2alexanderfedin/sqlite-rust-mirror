use super::*;
use crate::btree_int_h::Btree;
use crate::hash_h::Hash;
use crate::pager_h::Pgno;
use crate::sqlite3_h::{
    Sqlite3Context, Sqlite3Int64, Sqlite3MemMethods, Sqlite3Module,
    Sqlite3Mutex, Sqlite3MutexMethods, Sqlite3PcacheMethods2, Sqlite3Value,
    Sqlite3Vfs, Sqlite3Vtab,
};
use crate::vdbe_h::{Mem, SubProgram, Vdbe};

///* Each database connection is an instance of the following structure.
#[repr(C)]
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

///* A "Collating Sequence" is defined by an instance of the following
///* structure. Conceptually, a collating sequence consists of a name and
///* a comparison routine that defines the order of that sequence.
///*
///* If CollSeq.xCmp is NULL, it means that the
///* collating sequence is undefined.  Indices built on an undefined
///* collating sequence may not be read or written.
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

///* Each database file to be accessed by the system is an instance
///* of the following structure.  There are normally two of these structures
///* in the sqlite.aDb[] array.  aDb[0] is the main database file and
///* aDb[1] is the database file used to hold temporary tables.  Additional
///* databases may be attached.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Db {
    pub(crate) z_db_s_name: *mut i8,
    pub(crate) p_bt: *mut Btree,
    pub(crate) safety_level: u8,
    pub(crate) b_sync_set: u8,
    pub(crate) p_schema: *mut Schema,
}

///* An instance of the following structure is passed as the first
///* argument to sqlite3VdbeKeyCompare and is used to control the
///* comparison of the two index keys.
///*
///* The aSortOrder[] and aColl[] arrays have nAllField slots each. There
///* are nKeyField slots for the columns of an index then extra slots
///* for the rowid or key at the end.  The aSortOrder array is located after
///* the aColl[] array.
///*
///* If SQLITE_ENABLE_PREUPDATE_HOOK is defined, then aSortFlags might be NULL
///* to indicate that this object is for use by a preupdate hook.  When aSortFlags
///* is NULL, then nAllField is uninitialized and no space is allocated for
///* aColl[], so those fields may not be used.
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
pub(crate) struct Bitvec {
    pub(crate) _opaque: [u8; 0],
}

///* An instance of the following structure stores a database schema.
///*
///* Most Schema objects are associated with a Btree.  The exception is
///* the Schema for the TEMP database (sqlite3.aDb[1]) which is free-standing.
///* In shared cache mode, a single Schema object can be shared by multiple
///* Btrees that refer to the same underlying BtShared object.
///*
///* Schema objects are automatically deallocated when the last Btree that
///* references them is destroyed.   The TEMP Schema is manually freed by
///* sqlite3_close().
///
///* A thread must be holding a mutex on the corresponding Btree in order
///* to access Schema content.  This implies that the thread must also be
///* holding a mutex on the sqlite3 connection pointer that owns the Btree.
///* For a TEMP Schema, only the connection mutex is required.
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

///* The schema for each SQL table, virtual table, and view is represented
///* in memory by an instance of the following structure.
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

///* Information about each column of an SQL table is held in an instance
///* of the Column structure, in the Table.aCol[] array.
///*
///* Definitions:
///*
///*   "table column index"     This is the index of the column in the
///*                            Table.aCol[] array, and also the index of
///*                            the column in the original CREATE TABLE stmt.
///*
///*   "storage column index"   This is the index of the column in the
///*                            record BLOB generated by the OP_MakeRecord
///*                            opcode.  The storage column index is less than
///*                            or equal to the table column index.  It is
///*                            equal if and only if there are no VIRTUAL
///*                            columns to the left.
///*
///* Notes on zCnName:
///* The zCnName field stores the name of the column, the datatype of the
///* column, and the collating sequence for the column, in that order, all in
///* a single allocation.  Each string is 0x00 terminated.  The datatype
///* is only included if the COLFLAG_HASTYPE bit of colFlags is set and the
///* collating sequence name is only included if the COLFLAG_HASCOLL bit is
///* set.
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

///* Each SQL index is represented in memory by an
///* instance of the following structure.
///*
///* The columns of the table that are to be indexed are described
///* by the aiColumn[] field of this structure.  For example, suppose
///* we have the following table and index:
///*
///*     CREATE TABLE Ex1(c1 int, c2 int, c3 text);
///*     CREATE INDEX Ex2 ON Ex1(c3,c1);
///*
///* In the Table structure describing Ex1, nCol==3 because there are
///* three columns in the table.  In the Index structure describing
///* Ex2, nColumn==2 since 2 of the 3 columns of Ex1 are indexed.
///* The value of aiColumn is {2, 0}.  aiColumn[0]==2 because the
///* first column to be indexed (c3) has an index of 2 in Ex1.aCol[].
///* The second column to be indexed (c1) has an index of 0 in
///* Ex1.aCol[], hence Ex2.aiColumn[1]==0.
///*
///* The Index.onError field determines whether or not the indexed columns
///* must be unique and what to do if they are not.  When Index.onError=OE_None,
///* it means this is not a unique index.  Otherwise it is a unique index
///* and the value of Index.onError indicates which conflict resolution
///* algorithm to employ when an attempt is made to insert a non-unique
///* element.
///*
///* The colNotIdxed bitmask is used in combination with SrcItem.colUsed
///* for a fast test to see if an index can serve as a covering index.
///* colNotIdxed has a 1 bit for every column of the original table that
///* is *not* available in the index.  Thus the expression
///* "colUsed & colNotIdxed" will be non-zero if the index is not a
///* covering index.  The most significant bit of of colNotIdxed will always
///* be true (note-20221022-a).  If a column beyond the 63rd column of the
///* table is used, the "colUsed & colNotIdxed" test will always be non-zero
///* and we have to assume either that the index is not covering, or use
///* an alternative (slower) algorithm to determine whether or not
///* the index is covering.
///*
///* While parsing a CREATE TABLE or CREATE INDEX statement in order to
///* generate VDBE code (as opposed to parsing one read from an sqlite_schema
///* table as part of parsing an existing database schema), transient instances
///* of this structure may be created. In this case the Index.tnum variable is
///* used to store the address of a VDBE instruction, not a database page
///* number (it cannot - the database page is not allocated until the VDBE
///* program is executed). See convertToWithoutRowidTable() for details.
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

///* Estimated quantities used for query planning are stored as 16-bit
///* logarithms.  For quantity X, the value stored is 10*log2(X).  This
///* gives a possible range of values of approximately 1.0e986 to 1e-986.
///* But the allowed values are "grainy".  Not every value is representable.
///* For example, quantities 16 and 17 are both represented by a LogEst
///* of 40.  However, since LogEst quantities are suppose to be estimates,
///* not exact values, this imprecision is not a problem.
///*
///* "LogEst" is short for "Logarithmic Estimate".
///*
///* Examples:
///*      1 -> 0              20 -> 43          10000 -> 132
///*      2 -> 10             25 -> 46          25000 -> 146
///*      3 -> 16            100 -> 66        1000000 -> 199
///*      4 -> 20           1000 -> 99        1048576 -> 200
///*     10 -> 33           1024 -> 100    4294967296 -> 320
///*
///* The LogEst can be negative to indicate fractional values.
///* Examples:
///*
///*    0.5 -> -10           0.1 -> -33        0.0625 -> -40
pub(crate) type LogEst = i16;

///* Each node of an expression in the parse tree is an instance
///* of this structure.
///*
///* Expr.op is the opcode. The integer parser token codes are reused
///* as opcodes here. For example, the parser defines TK_GE to be an integer
///* code representing the ">=" operator. This same integer code is reused
///* to represent the greater-than-or-equal-to operator in the expression
///* tree.
///*
///* If the expression is an SQL literal (TK_INTEGER, TK_FLOAT, TK_BLOB,
///* or TK_STRING), then Expr.u.zToken contains the text of the SQL literal. If
///* the expression is a variable (TK_VARIABLE), then Expr.u.zToken contains the
///* variable name. Finally, if the expression is an SQL function (TK_FUNCTION),
///* then Expr.u.zToken contains the name of the function.
///*
///* Expr.pRight and Expr.pLeft are the left and right subexpressions of a
///* binary operator. Either or both may be NULL.
///*
///* Expr.x.pList is a list of arguments if the expression is an SQL function,
///* a CASE expression or an IN expression of the form "<lhs> IN (<y>, <z>...)".
///* Expr.x.pSelect is used if the expression is a sub-select or an expression of
///* the form "<lhs> IN (SELECT ...)". If the EP_xIsSelect bit is set in the
///* Expr.flags mask, then Expr.x.pSelect is valid. Otherwise, Expr.x.pList is
///* valid.
///*
///* An expression of the form ID or ID.ID refers to a column in a table.
///* For such expressions, Expr.op is set to TK_COLUMN and Expr.iTable is
///* the integer cursor number of a VDBE cursor pointing to that table and
///* Expr.iColumn is the column number for the specific column.  If the
///* expression is used as a result in an aggregate SELECT, then the
///* value is also stored in the Expr.iAgg column in the aggregate so that
///* it can be accessed after all aggregates are computed.
///*
///* If the expression is an unbound variable marker (a question mark
///* character '?' in the original SQL) then the Expr.iTable holds the index
///* number for that variable.
///*
///* If the expression is a subquery then Expr.iColumn holds an integer
///* register number containing the result of the subquery.  If the
///* subquery gives a constant result, then iTable is -1.  If the subquery
///* gives a different answer at different times during statement processing
///* then iTable is the address of a subroutine that computes the subquery.
///*
///* If the Expr is of type OP_Column, and the table it is selecting from
///* is a disk table or the "old.*" pseudo-table, then pTab points to the
///* corresponding table definition.
///*
///* ALLOCATION NOTES:
///*
///* Expr objects can use a lot of memory space in database schema.  To
///* help reduce memory requirements, sometimes an Expr object will be
///* truncated.  And to reduce the number of memory allocations, sometimes
///* two or more Expr objects will be stored in a single memory allocation,
///* together with Expr.u.zToken strings.
///*
///* If the EP_Reduced and EP_TokenOnly flags are set when
///* an Expr object is truncated.  When EP_Reduced is set, then all
///* the child Expr objects in the Expr.pLeft and Expr.pRight subtrees
///* are contained within the same memory allocation.  Note, however, that
///* the subtrees in Expr.x.pList or Expr.x.pSelect are always separately
///* allocated, regardless of whether or not EP_Reduced is set.
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

///* A list of expressions.  Each expression may optionally have a
///* name.  An expr/name combination can be used in several ways, such
///* as the list of "expr AS ID" fields following a "SELECT" or in the
///* list of "ID = expr" items in an UPDATE.  A list of expressions can
///* also be used as the argument to a function, in which case the a.zName
///* field is not used.
///*
///* In order to try to keep memory usage down, the Expr.a.zEName field
///* is used for multiple purposes:
///*
///*     eEName          Usage
///*    ----------       -------------------------
///*    ENAME_NAME       (1) the AS of result set column
///*                     (2) COLUMN= of an UPDATE
///*
///*    ENAME_TAB        DB.TABLE.NAME used to resolve names
///*                     of subqueries
///*
///*    ENAME_SPAN       Text of the original result set
///*                     expression.
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

///* An instance of the following structure contains all information
///* needed to generate code for a single SELECT statement.
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

///* This object represents one or more tables that are the source of
///* content for an SQL statement.  For example, a single SrcList object
///* is used to hold the FROM clause of a SELECT statement.  SrcList also
///* represents the target tables for DELETE, INSERT, and UPDATE statements.
///*
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SrcList {
    pub(crate) n_src: i32,
    pub(crate) n_alloc: u32,
    pub(crate) a: [SrcItem; 0],
}

///* The SrcItem object represents a single term in the FROM clause of a query.
///* The SrcList object is mostly an array of SrcItems.
///*
///* The jointype starts out showing the join type between the current table
///* and the next table on the list.  The parser builds the list this way.
///* But sqlite3SrcListShiftJoinType() later shifts the jointypes so that each
///* jointype expresses the join between the table and the previous table.
///*
///* In the colUsed field, the high-order bit (bit 63) is set if the table
///* contains more than 63 columns and the 64-th or later column is used.
///*
///* Aggressive use of "union" helps keep the size of the object small.  This
///* has been shown to boost performance, in addition to saving memory.
///* Access to union elements is gated by the following rules which should
///* always be checked, either by an if-statement or by an assert().
///*
///*    Field              Only access if this is true
///*    ---------------    -----------------------------------
///*    u1.zIndexedBy      fg.isIndexedBy
///*    u1.pFuncArg        fg.isTabFunc
///*    u1.nRow            !fg.isTabFunc  && !fg.isIndexedBy
///*
///*    u2.pIBIndex        fg.isIndexedBy
///*    u2.pCteUse         fg.isCte
///*
///*    u3.pOn             !fg.isUsing
///*    u3.pUsing          fg.isUsing
///*
///*    u4.zDatabase       !fg.fixedSchema && !fg.isSubquery
///*    u4.pSchema         fg.fixedSchema
///*    u4.pSubq           fg.isSubquery
///*
///* See also the sqlite3SrcListDelete() routine for assert() statements that
///* check invariants on the fields of this object, especially the flags
///* inside the fg struct.
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

///* The Cte object is not guaranteed to persist for the entire duration
///* of code generation.  (The query flattener or other parser tree
///* edits might delete it.)  The following object records information
///* about each Common Table Expression that must be preserved for the
///* duration of the parse.
///*
///* The CteUse objects are freed using sqlite3ParserAddCleanup() rather
///* than sqlite3SelectDelete(), which is what enables them to persist
///* until the end of code generation.
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

///* An instance of this structure can hold a simple list of identifiers,
///* such as the list "a,b,c" in the following statements:
///*
///*      INSERT INTO t(a,b,c) VALUES ...;
///*      CREATE INDEX idx ON t(a,b,c);
///*      CREATE TRIGGER trig BEFORE UPDATE ON t(a,b,c) ...;
///*
///* The IdList.a.idx field is used when the IdList represents the list of
///* column names after a table name in an INSERT statement.  In the statement
///*
///*     INSERT INTO t(a,b,c) ...
///*
///* If "a" is the k-th column of table "t", then IdList.a[0].idx==k.
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

///* Details of the implementation of a subquery.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Subquery {
    pub(crate) p_select: *mut Select,
    pub(crate) addr_fill_sub: i32,
    pub(crate) reg_return: i32,
    pub(crate) reg_result: i32,
}

///* An instance of the With object represents a WITH clause containing
///* one or more CTEs (common table expressions).
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct With {
    pub(crate) n_cte: i32,
    pub(crate) b_view: i32,
    pub(crate) p_outer: *mut With,
    pub(crate) a: [Cte; 0],
}

///* A single common table expression
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

///* This object is used in various ways, most (but not all) related to window
///* functions.
///*
///*   (1) A single instance of this structure is attached to the
///*       the Expr.y.pWin field for each window function in an expression tree.
///*       This object holds the information contained in the OVER clause,
///*       plus additional fields used during code generation.
///*
///*   (2) All window functions in a single SELECT form a linked-list
///*       attached to Select.pWin.  The Window.pFunc and Window.pExpr
///*       fields point back to the expression that is the window function.
///*
///*   (3) The terms of the WINDOW clause of a SELECT are instances of this
///*       object on a linked list attached to Select.pWinDefn.
///*
///*   (4) For an aggregate function with a FILTER clause, an instance
///*       of this object is stored in Expr.y.pWin with eFrmType set to
///*       TK_FILTER. In this case the only field used is Window.pFilter.
///*
///* The uses (1) and (2) are really the same Window object that just happens
///* to be accessible in two different ways.  Use case (3) are separate objects.
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

///* Each SQL function is defined by an instance of the following
///* structure.  For global built-in functions (ex: substr(), max(), count())
///* a pointer to this structure is held in the sqlite3BuiltinFunctions object.
///* For per-connection application-defined functions, a pointer to this
///* structure is held in the db->aHash hash table.
///*
///* The u.pHash field is used by the global built-ins.  The u.pDestructor
///* field is used by per-connection app-def functions.
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

///* This structure encapsulates a user-function destructor callback (as
///* configured using create_function_v2()) and a reference counter. When
///* create_function_v2() is called to create a function with a destructor,
///* a single object of this type is allocated. FuncDestructor.nRef is set to
///* the number of FuncDef objects created (either 1 or 3, depending on whether
///* or not the specified encoding is SQLITE_ANY). The FuncDef.pDestructor
///* member of each of the new FuncDef objects is set to point to the allocated
///* FuncDestructor.
///*
///* Thereafter, when one of the FuncDef objects is deleted, the reference
///* count on this object is decremented. When it reaches 0, the destructor
///* is invoked and the FuncDestructor structure freed.
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

///* An instance of this structure contains information needed to generate
///* code for a SELECT that contains aggregate functions.
///*
///* If Expr.op==TK_AGG_COLUMN or TK_AGG_FUNCTION then Expr.pAggInfo is a
///* pointer to this structure.  The Expr.iAgg field is the index in
///* AggInfo.aCol[] or AggInfo.aFunc[] of information needed to generate
///* code for that node.
///*
///* AggInfo.pGroupBy and AggInfo.aFunc.pExpr point to fields within the
///* original Select structure that describes the SELECT statement.  These
///* fields do not need to be freed when deallocating the AggInfo structure.
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

///* Each foreign key constraint is an instance of the following structure.
///*
///* A foreign key is associated with two tables.  The "from" table is
///* the table that contains the REFERENCES clause that creates the foreign
///* key.  The "to" table is the table that is named in the REFERENCES clause.
///* Consider this example:
///*
///*     CREATE TABLE ex1(
///*       a INTEGER PRIMARY KEY,
///*       b INTEGER CONSTRAINT fk1 REFERENCES ex2(x)
///*     );
///*
///* For foreign key "fk1", the from-table is "ex1" and the to-table is "ex2".
///* Equivalent names:
///*
///*     from-table == child-table
///*       to-table == parent-table
///*
///* Each REFERENCES clause generates an instance of the following structure
///* which is attached to the from-table.  The to-table need not exist when
///* the from-table is created.  The existence of the to-table is not checked.
///*
///* The list of all parents for child Table X is held at X.pFKey.
///*
///* A list of all children for a table named Z (which might not even exist)
///* is held in Schema.fkeyHash with a hash key of Z.
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

///* Each trigger present in the database schema is stored as an instance of
///* struct Trigger.
///*
///* Pointers to instances of struct Trigger are stored in two ways.
///* 1. In the "trigHash" hash table (part of the sqlite3* that represents the
///*    database). This allows Trigger structures to be retrieved by name.
///* 2. All triggers associated with a single table form a linked list, using the
///*    pNext member of struct Trigger. A pointer to the first element of the
///*    linked list is stored as the "pTrigger" member of the associated
///*    struct Table.
///*
///* The "step_list" member points to the first element of a linked list
///* containing the SQL statements specified as the trigger program.
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

///* An instance of struct TriggerStep is used to store a single SQL statement
///* that is a part of a trigger-program.
///*
///* Instances of struct TriggerStep are stored in a singly linked list (linked
///* using the "pNext" member) referenced by the "step_list" member of the
///* associated struct Trigger instance. The first element of the linked list is
///* the first step of the trigger-program.
///*
///* The "op" member indicates whether this is a "DELETE", "INSERT", "UPDATE" or
///* "SELECT" statement. The meanings of the other members is determined by the
///* value of "op" as follows:
///*
///* (op == TK_INSERT)
///* orconf    -> stores the ON CONFLICT algorithm
///* pSelect   -> The content to be inserted - either a SELECT statement or
///*              a VALUES clause.
///* pSrc      -> Table to insert into.
///* pIdList   -> If this is an INSERT INTO ... (<column-names>) VALUES ...
///*              statement, then this stores the column-names to be
///*              inserted into.
///* pUpsert   -> The ON CONFLICT clauses for an Upsert
///*
///* (op == TK_DELETE)
///* pSrc      -> Table to delete from
///* pWhere    -> The WHERE clause of the DELETE statement if one is specified.
///*              Otherwise NULL.
///*
///* (op == TK_UPDATE)
///* pSrc      -> Table to update, followed by any FROM clause tables.
///* pWhere    -> The WHERE clause of the UPDATE statement if one is specified.
///*              Otherwise NULL.
///* pExprList -> A list of the columns to update and the expressions to update
///*              them to. See sqlite3Update() documentation of "pChanges"
///*              argument.
///*
///* (op == TK_SELECT)
///* pSelect   -> The SELECT statement
///*
///* (op == TK_RETURNING)
///* pExprList -> The list of expressions that follow the RETURNING keyword.
///*
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

///* An instance of the following object describes a single ON CONFLICT
///* clause in an upsert.
///*
///* The pUpsertTarget field is only set if the ON CONFLICT clause includes
///* conflict-target clause.  (In "ON CONFLICT(a,b)" the "(a,b)" is the
///* conflict-target clause.)  The pUpsertTargetWhere is the optional
///* WHERE clause used to identify partial unique indexes.
///*
///* pUpsertSet is the list of column=expr terms of the UPDATE statement.
///* The pUpsertSet field is NULL for a ON CONFLICT DO NOTHING.  The
///* pUpsertWhere is the WHERE clause for the UPDATE and is NULL if the
///* WHERE clause is omitted.
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

///* An object of this type is created for each virtual table present in
///* the database schema.
///*
///* If the database schema is shared, then there is one instance of this
///* structure for each database connection (sqlite3*) that uses the shared
///* schema. This is because each database connection requires its own unique
///* instance of the sqlite3_vtab* handle used to access the virtual table
///* implementation. sqlite3_vtab* handles can not be shared between
///* database connections, even when the rest of the in-memory database
///* schema is shared, as the implementation often stores the database
///* connection handle passed to it via the xConnect() or xCreate() method
///* during initialization internally. This database connection handle may
///* then be used by the virtual table implementation to access real tables
///* within the database. So that they appear as part of the callers
///* transaction, these accesses need to be made via the same database
///* connection as that used to execute SQL operations on the virtual table.
///*
///* All VTable objects that correspond to a single table in a shared
///* database schema are initially stored in a linked-list pointed to by
///* the Table.pVTable member variable of the corresponding Table object.
///* When an sqlite3_prepare() operation is required to access the virtual
///* table, it searches the list for the VTable that corresponds to the
///* database connection doing the preparing so as to use the correct
///* sqlite3_vtab* handle in the compiled query.
///*
///* When an in-memory Table object is deleted (for example when the
///* schema is being reloaded for some reason), the VTable objects are not
///* deleted and the sqlite3_vtab* handles are not xDisconnect()ed
///* immediately. Instead, they are moved from the Table.pVTable list to
///* another linked list headed by the sqlite3.pDisconnect member of the
///* corresponding sqlite3 structure. They are then deleted/xDisconnected
///* next time a statement is prepared using said sqlite3*. This is done
///* to avoid deadlock issues involving multiple sqlite3.mutex mutexes.
///* Refer to comments above function sqlite3VtabUnlockList() for an
///* explanation as to why it is safe to add an entry to an sqlite3.pDisconnect
///* list without holding the corresponding sqlite3.mutex mutex.
///*
///* The memory for objects of this type is always allocated by
///* sqlite3DbMalloc(), using the connection handle stored in VTable.db as
///* the first argument.
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

///* Each SQLite module (virtual table definition) is defined by an
///* instance of the following structure, stored in the sqlite3.aModule
///* hash table.
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

///* An SQL parser context.  A copy of this structure is passed through
///* the parser and down into all the parser action routine in order to
///* carry around information that is global to the entire parse.
///*
///* The structure is divided into two parts.  When the parser and code
///* generate call themselves recursively, the first part of the structure
///* is constant but the second part is reset at the beginning and end of
///* each recursion.
///*
///* The nTableLock and aTableLock variables are only used if the shared-cache
///* feature is enabled (if sqlite3Tsd()->useSharedData is true). They are
///* used to store the set of table-locks required by the statement being
///* compiled. Function sqlite3TableLock() is used to add entries to the
///* list.
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

/// A bitfield type for use inside of structures.  Always follow with :N where
///* N is the number of bits.
pub(crate) type Bft = u32;

///* For each index X that has as one of its arguments either an expression
///* or the name of a virtual generated column, and if X is in scope such that
///* the value of the expression can simply be read from the index, then
///* there is an instance of this object on the Parse.pIdxExpr list.
///*
///* During code generation, while generating code to evaluate expressions,
///* this list is consulted and if a matching expression is found, the value
///* is read from the index rather than being recomputed.
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

///* At least one instance of the following structure is created for each
///* trigger that may be fired while parsing an INSERT, UPDATE or DELETE
///* statement. All such objects are stored in the linked list headed at
///* Parse.pTriggerPrg and deleted once statement compilation has been
///* completed.
///*
///* A Vdbe sub-program that implements the body and WHEN clause of trigger
///* TriggerPrg.pTrigger, assuming a default ON CONFLICT clause of
///* TriggerPrg.orconf, is stored in the TriggerPrg.pProgram variable.
///* The Parse.pTriggerPrg list never contains two entries with the same
///* values for both pTrigger and orconf.
///*
///* The TriggerPrg.aColmask[0] variable is set to a mask of old.* columns
///* accessed (or set to 0 for triggers fired as a result of INSERT
///* statements). Similarly, the TriggerPrg.aColmask[1] variable is set to
///* a mask of new.* columns used by the program.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TriggerPrg {
    pub(crate) p_trigger: *mut Trigger,
    pub(crate) p_next: *mut TriggerPrg,
    pub(crate) p_program: *mut SubProgram,
    pub(crate) orconf: i32,
    pub(crate) a_colmask: [u32; 2],
}

///* An instance of the ParseCleanup object specifies an operation that
///* should be performed after parsing to deallocation resources obtained
///* during the parse and which are no longer needed.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct ParseCleanup {
    pub(crate) p_next: *mut ParseCleanup,
    pub(crate) p_ptr: *mut (),
    pub(crate) x_cleanup: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ())
        -> ()>,
}

///* Each token coming out of the lexer is an instance of
///* this structure.  Tokens are also used as part of an expression.
///*
///* The memory that "z" points to is owned by other objects.  Take care
///* that the owner of the "z" string does not deallocate the string before
///* the Token goes out of scope!  Very often, the "z" points to some place
///* in the middle of the Parse.zSql text.  But it might also point to a
///* static string.
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

///* Information about a RETURNING clause
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

///* During code generation of statements that do inserts into AUTOINCREMENT
///* tables, the following information is attached to the Table.u.autoInc.p
///* pointer of each autoincrement table to record some side information that
///* the code generator needs.  We have to keep per-table autoincrement
///* information in case inserts are done within triggers.  Triggers do not
///* normally coordinate their activities, but we do need to coordinate the
///* loading and saving of autoincrement information.
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

/// A VList object records a mapping between parameters/variables/wildcards
///* in the SQL statement (such as $abc, @pqr, or :xyz) and the integer
///* variable number associated with that parameter.  See the format description
///* on the sqlite3VListAdd() routine for more information.  A VList is really
///* just an array of integers.
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

///* Lookaside malloc is a set of fixed-size buffers that can be used
///* to satisfy small transient memory allocation requests for objects
///* associated with a particular database connection.  The use of
///* lookaside malloc provides a significant performance enhancement
///* (approx 10%) by avoiding numerous malloc/free requests while parsing
///* SQL statements.
///*
///* The Lookaside structure holds configuration information about the
///* lookaside malloc subsystem.  Each available memory allocation in
///* the lookaside subsystem is stored on a linked list of LookasideSlot
///* objects.
///*
///* Lookaside allocations are only allowed for objects that are associated
///* with a particular database connection.  Hence, schema information cannot
///* be stored in lookaside because in shared cache mode the schema information
///* is shared by multiple database connections.  Therefore, while parsing
///* schema information, the Lookaside.bEnabled flag is cleared so that
///* lookaside allocations are not used to construct the schema objects.
///*
///* New lookaside allocations are only allowed if bDisable==0.  When
///* bDisable is greater than zero, sz is set to zero which effectively
///* disables lookaside without adding a new test for the bDisable flag
///* in a performance-critical path.  sz should be set by to szTrue whenever
///* bDisable changes back to zero.
///*
///* Lookaside buffers are initially held on the pInit list.  As they are
///* used and freed, they are added back to the pFree list.  New allocations
///* come off of pFree first, then pInit as a fallback.  This dual-list
///* allows use to compute a high-water mark - the maximum number of allocations
///* outstanding at any point in the past - by subtracting the number of
///* allocations on the pInit list from the total number of allocations.
///*
///* Enhancement on 2019-12-12:  Two-size-lookaside
///* The default lookaside configuration is 100 slots of 1200 bytes each.
///* The larger slot sizes are important for performance, but they waste
///* a lot of space, as most lookaside allocations are less than 128 bytes.
///* The two-size-lookaside enhancement breaks up the lookaside allocation
///* into two pools:  One of 128-byte slots and the other of the default size
///* (1200-byte) slots.   Allocations are filled from the small-pool first,
///* failing over to the full-size pool if that does not work.  Thus more
///* lookaside slots are available while also using less memory.
///* This enhancement can be omitted by compiling with
///* SQLITE_OMIT_TWOSIZE_LOOKASIDE.
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

///* typedef for the authorization callback function.
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

///* All current savepoints are stored in a linked list starting at
///* sqlite3.pSavepoint. The first element in the list is the most recently
///* opened savepoint. Savepoints are added to the list by the vdbe
///* OP_Savepoint instruction.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Savepoint {
    pub(crate) z_name: *mut i8,
    pub(crate) n_deferred_cons: i64,
    pub(crate) n_deferred_imm_cons: i64,
    pub(crate) p_next: *mut Savepoint,
}

/// Client data associated with sqlite3_set_clientdata() and
///* sqlite3_get_clientdata().
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct DbClientData {
    pub(crate) p_next: *mut DbClientData,
    pub(crate) p_data: *mut (),
    pub(crate) x_destructor: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    pub(crate) z_name: [i8; 0],
}

///* An object used to accumulate the text of a string where we
///* do not necessarily know how big the string will be in the end.
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

///* The datatype used to store estimates of the number of rows in a
///* table or index.
pub(crate) type TRowcnt = u64;

pub(crate) type Uptr = u64;

///* An instance of the following structure can be declared on a stack and used
///* to save the Parse.zAuthContext value so that it can be restored later.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct AuthContext {
    pub(crate) z_auth_context: *const i8,
    pub(crate) p_parse: *mut Parse,
}

///* The following structure contains information used by the sqliteFix...
///* routines as they walk the parse tree to make database references
///* explicit.
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

///* Context pointer passed down through the tree-walk.
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

///* A NameContext defines a context in which to resolve table and column
///* names.  The context consists of a list of tables (the pSrcList) field and
///* a list of named expression (pEList).  The named expression list may
///* be NULL.  The pSrc corresponds to the FROM clause of a SELECT or
///* to the table being operated on by INSERT, UPDATE, or DELETE.  The
///* pEList corresponds to the result set of a SELECT and is NULL for
///* other statements.
///*
///* NameContexts can be nested.  When resolving names, the inner-most
///* context is searched first.  If no match is found, the next outer
///* context is checked.  If there is still no match, the next context
///* is checked.  This process continues until either a match is found
///* or all contexts are check.  When a match is found, the nRef member of
///* the context containing the match is incremented.
///*
///* Each subquery gets a new NameContext.  The pNext field points to the
///* NameContext in the parent query.  Thus the process of scanning the
///* NameContext list corresponds to searching through successively outer
///* subqueries looking for a match.
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

///* An instance of this object receives the decoding of a floating point
///* value into an approximate decimal representation.
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

///* Each sample stored in the sqlite_stat4 table is represented in memory
///* using a structure of this type.  See documentation at the top of the
///* analyze.c source file for additional information.
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

///* The OnOrUsing object represents either an ON clause or a USING clause.
///* It can never be both at the same time, but it can be neither.
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

///* An instance of the following structure holds information about SQL
///* functions arguments that are the parameters to the printf() function.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct PrintfArguments {
    pub(crate) n_arg: i32,
    pub(crate) n_used: i32,
    pub(crate) ap_arg: *mut *mut Sqlite3Value,
}

///* The following object is the header for an "RCStr" or "reference-counted
///* string".  An RCStr is passed around and used like any other char*
///* that has been dynamically allocated.  The important interface
///* differences:
///*
///*   1.  RCStr strings are reference counted.  They are deallocated
///*       when the reference count reaches zero.
///*
///*   2.  Use sqlite3RCStrUnref() to free an RCStr string rather than
///*       sqlite3_free()
///*
///*   3.  Make a (read-only) copy of a read-only RCStr string using
///*       sqlite3RCStrRef().
///*
///* "String" is in the name, but an RCStr object can also be used to hold
///* binary data.
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

///* An instance of this object describes where to put of the results of
///* a SELECT statement.
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

///* This object holds a record which has been parsed out into individual
///* fields, for the purposes of doing a comparison.
///*
///* A record is an object that contains one or more fields of data.
///* Records are used to store the content of a table row and to store
///* the key of an index.  A blob encoding of a record is created by
///* the OP_MakeRecord opcode of the VDBE and is disassembled by the
///* OP_Column opcode.
///*
///* An instance of this object serves as a "key" for doing a search on
///* an index b+tree. The goal of the search is to find the entry that
///* is closest to the key described by this object.  This object might hold
///* just a prefix of the key.  The number of fields is given by nField.
///*
///* The r1 and r2 fields are the values to return if this key is less than
///* or greater than a key in the btree, respectively.  These are normally
///* -1 and +1 respectively, but might be inverted to +1 and -1 if the b-tree
///* is in DESC order.
///*
///* The key comparison functions actually return default_rc when they find
///* an equals comparison.  default_rc can be -1, 0, or +1.  If there are
///* multiple entries in the b-tree with the same key (when only looking
///* at the first nField elements) then default_rc can be set to -1 to
///* cause the search to find the last match, or +1 to cause the search to
///* find the first match.
///*
///* The key comparison functions will set eqSeen to true if they ever
///* get and equal results when comparing this structure to a b-tree record.
///* When default_rc!=0, the search might end up on the record immediately
///* before the first match or immediately after the last match.  The
///* eqSeen field will indicate whether or not an exact match exists in the
///* b-tree.
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

///* A pointer to this structure is used to communicate information
///* from sqlite3Init and OP_ParseSchema into the sqlite3InitCallback.
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

///* Structure containing global configuration data for the SQLite library.
///*
///* This structure also contains some state information.
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
