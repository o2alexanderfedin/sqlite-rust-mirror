#![allow(unused_imports, dead_code)]

mod btree_h;
mod hash_h;
mod pager_h;
mod pcache_h;
mod sqlite3_h;
mod sqlite_int_h;
mod vdbe_h;
use crate::btree_h::{BtCursor, Btree, BtreePayload};
use crate::hash_h::Hash;
use crate::pager_h::{DbPage, Pager, Pgno};
use crate::pcache_h::{PCache, PgHdr};
use crate::sqlite3_h::{
    Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File, Sqlite3Filename,
    Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module, Sqlite3Mutex,
    Sqlite3MutexMethods, Sqlite3PcachePage, Sqlite3RtreeGeometry,
    Sqlite3RtreeQueryInfo, Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Uint64,
    Sqlite3Value, Sqlite3Vfs, Sqlite3Vtab,
};
use crate::sqlite_int_h::{
    AuthContext, Bft, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte,
    DbFixer, Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode,
    FuncDef, FuncDefHash, FuncDestructor, IdList, Index, KeyInfo, LogEst,
    Module, NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema,
    Select, SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str,
    SrcItem, SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
    TriggerStep, UnpackedRecord, Upsert, VList, VTable, Walker, WhereInfo,
    Window, With, YnVar,
};
use crate::vdbe_h::{Mem, SubProgram, Vdbe, VdbeOp, VdbeOpList};

type DarwinSizeT = u64;

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

impl ExprListItemS0 {
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

impl SrcItemS0 {
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

impl Sqlite3InitInfo {
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

///* Check to see if index pIdx is a partial index whose conditional
///* expression might change values due to an UPDATE.  Return true if
///* the index is subject to change and false if the index is guaranteed
///* to be unchanged.  This is an optimization.  False-positives are a
///* performance degradation, but false-negatives can result in a corrupt
///* index and incorrect answers.
///*
///* aXRef[j] will be non-negative if column j of the original table is
///* being updated.  chngRowid will be true if the rowid of the table is
///* being updated.
extern "C" fn index_where_clause_might_change(p_idx_1: &Index,
    a_x_ref_1: *mut i32, chng_rowid_1: i32) -> i32 {
    if (*p_idx_1).p_part_idx_where == core::ptr::null_mut() { return 0; }
    return unsafe {
            sqlite3_expr_references_updated_column((*p_idx_1).p_part_idx_where,
                a_x_ref_1, chng_rowid_1)
        };
}

///* Check to see if column iCol of index pIdx references any of the
///* columns defined by aXRef and chngRowid.  Return true if it does
///* and false if not.  This is an optimization.  False-positives are a
///* performance degradation, but false-negatives can result in a corrupt
///* index and incorrect answers.
///*
///* aXRef[j] will be non-negative if column j of the original table is
///* being updated.  chngRowid will be true if the rowid of the table is
///* being updated.
extern "C" fn index_column_is_being_updated(p_idx_1: &Index, i_col_1: i32,
    a_x_ref_1: *mut i32, chng_rowid_1: i32) -> i32 {
    let i_idx_col: i16 =
        unsafe { *(*p_idx_1).ai_column.offset(i_col_1 as isize) };
    { let _ = 0; };
    if i_idx_col as i32 >= 0 {
        return (unsafe { *a_x_ref_1.offset(i_idx_col as isize) } >= 0) as i32;
    }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    return unsafe {
            sqlite3_expr_references_updated_column(unsafe {
                    (*(unsafe { (*(*p_idx_1).a_col_expr).a.as_ptr() } as
                                    *mut ExprListItem).offset(i_col_1 as isize)).p_expr
                }, a_x_ref_1, chng_rowid_1)
        };
}

///* Allocate and return a pointer to an expression of type TK_ROW with
///* Expr.iColumn set to value (iCol+1). The resolver will modify the
///* expression to be a TK_COLUMN reading column iCol of the first
///* table in the source-list (pSrc->a[0]).
extern "C" fn expr_row_column(p_parse_1: *mut Parse, i_col_1: i32)
    -> *mut Expr {
    let p_ret: *mut Expr =
        unsafe {
            sqlite3_p_expr(p_parse_1, 76, core::ptr::null_mut(),
                core::ptr::null_mut())
        };
    if !(p_ret).is_null() {
        unsafe { (*p_ret).i_column = (i_col_1 + 1) as YnVar };
    }
    return p_ret;
}

///* Assuming both the pLimit and pOrderBy parameters are NULL, this function
///* generates VM code to run the query:
///*
///*   SELECT <other-columns>, pChanges FROM pTabList WHERE pWhere
///*
///* and write the results to the ephemeral table already opened as cursor
///* iEph. None of pChanges, pTabList or pWhere are modified or consumed by
///* this function, they must be deleted by the caller.
///*
///* Or, if pLimit and pOrderBy are not NULL, and pTab is not a view:
///*
///*   SELECT <other-columns>, pChanges FROM pTabList
///*   WHERE pWhere
///*   GROUP BY <other-columns>
///*   ORDER BY pOrderBy LIMIT pLimit
///*
///* If pTab is a view, the GROUP BY clause is omitted.
///*
///* Exactly how results are written to table iEph, and exactly what
///* the <other-columns> in the query above are is determined by the type
///* of table pTabList->a[0].pTab.
///*
///* If the table is a WITHOUT ROWID table, then argument pPk must be its
///* PRIMARY KEY. In this case <other-columns> are the primary key columns
///* of the table, in order. The results of the query are written to ephemeral
///* table iEph as index keys, using OP_IdxInsert.
///*
///* If the table is actually a view, then <other-columns> are all columns of
///* the view. The results are written to the ephemeral table iEph as records
///* with automatically assigned integer keys.
///*
///* If the table is a virtual or ordinary intkey table, then <other-columns>
///* is its rowid. For a virtual table, the results are written to iEph as
///* records with automatically assigned integer keys For intkey tables, the
///* rowid value in <other-columns> is used as the integer key, and the
///* remaining fields make up the table record.
extern "C" fn update_from_select(p_parse_1: *mut Parse, i_eph_1: i32,
    p_pk_1: *const Index, p_changes_1: *const ExprList,
    p_tab_list_1: *const SrcList, p_where_1: *const Expr,
    p_order_by_1: *const ExprList, p_limit_1: *const Expr) -> () {
    let mut i: i32 = 0;
    let mut dest: SelectDest = unsafe { core::mem::zeroed() };
    let mut p_select: *mut Select = core::ptr::null_mut();
    let mut p_list: *mut ExprList = core::ptr::null_mut();
    let p_grp: *mut ExprList = core::ptr::null_mut();
    let p_limit2: *mut Expr = core::ptr::null_mut();
    let p_order_by2: *mut ExprList = core::ptr::null_mut();
    let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
    let p_tab: *const Table =
        unsafe {
                (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).p_s_tab
            } as *const Table;
    let mut p_src: *mut SrcList = core::ptr::null_mut();
    let mut p_where2: *mut Expr = core::ptr::null_mut();
    let mut e_dest: i32 = 0;
    { let _ = p_order_by_1; };
    { let _ = p_limit_1; };
    p_src =
        unsafe {
            sqlite3_src_list_dup(db, p_tab_list_1 as *const SrcList, 0)
        };
    p_where2 = unsafe { sqlite3_expr_dup(db, p_where_1 as *const Expr, 0) };
    { let _ = 0; };
    if !(p_src).is_null() {
        { let _ = 0; };
        unsafe {
            (*(unsafe { (*p_src).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).i_cursor = -1
        };
        {
            let __p =
                unsafe {
                    &mut (*unsafe {
                                        (*(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).p_s_tab
                                    }).n_tab_ref
                };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        unsafe {
            (*(unsafe { (*p_src).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).p_s_tab =
                core::ptr::null_mut()
        };
    }
    if !(p_pk_1).is_null() {
        {
            i = 0;
            '__b0: loop {
                if !(i < unsafe { (*p_pk_1).n_key_col } as i32) {
                    break '__b0;
                }
                '__c0: loop {
                    let p_new: *mut Expr =
                        expr_row_column(p_parse_1,
                            unsafe {
                                    *unsafe { (*p_pk_1).ai_column.offset(i as isize) }
                                } as i32);
                    p_list =
                        unsafe {
                            sqlite3_expr_list_append(p_parse_1, p_list, p_new)
                        };
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        e_dest =
            if unsafe { (*p_tab).e_tab_type } as i32 == 1 { 12 } else { 13 };
    } else if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
        {
            i = 0;
            '__b1: loop {
                if !(i < unsafe { (*p_tab).n_col } as i32) { break '__b1; }
                '__c1: loop {
                    p_list =
                        unsafe {
                            sqlite3_expr_list_append(p_parse_1, p_list,
                                expr_row_column(p_parse_1, i))
                        };
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        e_dest = 12;
    } else {
        e_dest =
            if unsafe { (*p_tab).e_tab_type } as i32 == 1 { 12 } else { 13 };
        p_list =
            unsafe {
                sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                    unsafe {
                        sqlite3_p_expr(p_parse_1, 76, core::ptr::null_mut(),
                            core::ptr::null_mut())
                    })
            };
    }
    { let _ = 0; };
    if !(p_changes_1).is_null() {
        {
            i = 0;
            '__b2: loop {
                if !(i < unsafe { (*p_changes_1).n_expr }) { break '__b2; }
                '__c2: loop {
                    p_list =
                        unsafe {
                            sqlite3_expr_list_append(p_parse_1, p_list,
                                unsafe {
                                    sqlite3_expr_dup(db,
                                        unsafe {
                                                (*(unsafe { (*p_changes_1).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                            } as *const Expr, 0)
                                })
                        };
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    p_select =
        unsafe {
            sqlite3_select_new(p_parse_1, p_list, p_src, p_where2, p_grp,
                core::ptr::null_mut(), p_order_by2,
                (8388608 | 131072 | 268435456) as u32, p_limit2)
        };
    if !(p_select).is_null() {
        unsafe { (*p_select).sel_flags |= 134217728 as u32 };
    }
    unsafe { sqlite3_select_dest_init(&mut dest, e_dest, i_eph_1) };
    dest.i_sd_parm2 =
        if !(p_pk_1).is_null() {
            (unsafe { (*p_pk_1).n_key_col }) as i32
        } else { -1 };
    unsafe { sqlite3_select(p_parse_1, p_select, &mut dest) };
    unsafe { sqlite3_select_delete(db, p_select) };
}

/// Forward declaration
#[allow(unused_doc_comments)]
extern "C" fn update_virtual_table(p_parse: *mut Parse, p_src: *mut SrcList,
    p_tab: *mut Table, p_changes: &ExprList, p_rowid: *mut Expr,
    a_x_ref: *mut i32, p_where: *mut Expr, on_error: i32) -> () {
    let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
    /// Virtual machine under construction
    let mut ephem_tab: i32 = 0;
    /// Table holding the result of the SELECT
    let mut i: i32 = 0;
    /// Loop counter
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    /// Database connection
    let p_v_tab: *const i8 =
        unsafe { sqlite3_get_v_table(db, p_tab) } as *const i8;
    let mut p_w_info: *mut WhereInfo = core::ptr::null_mut();
    let n_arg: i32 = 2 + unsafe { (*p_tab).n_col } as i32;
    /// Number of arguments to VUpdate
    let mut reg_arg: i32 = 0;
    /// First register in VUpdate arg array
    let mut reg_rec: i32 = 0;
    /// Register in which to assemble record
    let mut reg_rowid: i32 = 0;
    /// Register for ephemeral table rowid
    let i_csr: i32 =
        unsafe {
            (*(unsafe { (*p_src).a.as_ptr() } as
                            *mut SrcItem).offset(0 as isize)).i_cursor
        };
    /// Cursor used for virtual table scan
    let mut a_dummy: [i32; 2] = [0; 2];
    /// Unused arg for sqlite3WhereOkOnePass()
    let mut e_one_pass: i32 = 0;
    /// True to use onepass strategy
    let mut addr: i32 = 0;

    /// Address of OP_OpenEphemeral
    /// Allocate nArg registers in which to gather the arguments for VUpdate. Then
    ///* create and open the ephemeral table in which the records created from
    ///* these arguments will be temporarily stored.
    { let _ = 0; };
    ephem_tab =
        {
            let __p = unsafe { &mut (*p_parse).n_tab };
            let __t = *__p;
            *__p += 1;
            __t
        };
    addr = unsafe { sqlite3_vdbe_add_op2(v, 120, ephem_tab, n_arg) };
    reg_arg = unsafe { (*p_parse).n_mem } + 1;
    unsafe { (*p_parse).n_mem += n_arg };
    if unsafe { (*p_src).n_src } > 1 {
        let mut p_pk: *mut Index = core::ptr::null_mut();
        let mut p_row: *mut Expr = core::ptr::null_mut();
        let mut p_list: *mut ExprList = core::ptr::null_mut();
        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
            if !(p_rowid).is_null() {
                p_row =
                    unsafe { sqlite3_expr_dup(db, p_rowid as *const Expr, 0) };
            } else {
                p_row =
                    unsafe {
                        sqlite3_p_expr(p_parse, 76, core::ptr::null_mut(),
                            core::ptr::null_mut())
                    };
            }
        } else {
            let mut i_pk: i16 = 0 as i16;

            /// PRIMARY KEY column
            (p_pk = unsafe { sqlite3_primary_key_index(p_tab) });
            { let _ = 0; };
            { let _ = 0; };
            i_pk =
                unsafe { *unsafe { (*p_pk).ai_column.offset(0 as isize) } };
            if unsafe { *a_x_ref.offset(i_pk as isize) } >= 0 {
                p_row =
                    unsafe {
                        sqlite3_expr_dup(db,
                            unsafe {
                                    (*((*p_changes).a.as_ptr() as
                                                    *mut ExprListItem).offset(unsafe {
                                                        *a_x_ref.offset(i_pk as isize)
                                                    } as isize)).p_expr
                                } as *const Expr, 0)
                    };
            } else { p_row = expr_row_column(p_parse, i_pk as i32); }
        }
        p_list =
            unsafe {
                sqlite3_expr_list_append(p_parse, core::ptr::null_mut(),
                    p_row)
            };
        {
            i = 0;
            '__b3: loop {
                if !(i < unsafe { (*p_tab).n_col } as i32) { break '__b3; }
                '__c3: loop {
                    if unsafe { *a_x_ref.offset(i as isize) } >= 0 {
                        p_list =
                            unsafe {
                                sqlite3_expr_list_append(p_parse, p_list,
                                    unsafe {
                                        sqlite3_expr_dup(db,
                                            unsafe {
                                                    (*((*p_changes).a.as_ptr() as
                                                                    *mut ExprListItem).offset(unsafe {
                                                                        *a_x_ref.offset(i as isize)
                                                                    } as isize)).p_expr
                                                } as *const Expr, 0)
                                    })
                            };
                    } else {
                        let p_row_expr: *mut Expr = expr_row_column(p_parse, i);
                        if !(p_row_expr).is_null() {
                            unsafe { (*p_row_expr).op2 = 1 as u8 };
                        }
                        p_list =
                            unsafe {
                                sqlite3_expr_list_append(p_parse, p_list, p_row_expr)
                            };
                    }
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        update_from_select(p_parse, ephem_tab, p_pk as *const Index,
            p_list as *const ExprList, p_src as *const SrcList,
            p_where as *const Expr, core::ptr::null(), core::ptr::null());
        unsafe { sqlite3_expr_list_delete(db, p_list) };
        e_one_pass = 0;
    } else {
        reg_rec =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        reg_rowid =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };

        /// Start scanning the virtual table
        (p_w_info =
            unsafe {
                sqlite3_where_begin(p_parse, p_src, p_where,
                    core::ptr::null_mut(), core::ptr::null_mut(),
                    core::ptr::null_mut(), 4 as u16, 0)
            });
        if p_w_info == core::ptr::null_mut() { return; }
        {
            i = 0;
            '__b4: loop {
                if !(i < unsafe { (*p_tab).n_col } as i32) { break '__b4; }
                '__c4: loop {
                    { let _ = 0; };
                    if unsafe { *a_x_ref.offset(i as isize) } >= 0 {
                        unsafe {
                            sqlite3_expr_code(p_parse,
                                unsafe {
                                    (*((*p_changes).a.as_ptr() as
                                                    *mut ExprListItem).offset(unsafe {
                                                        *a_x_ref.offset(i as isize)
                                                    } as isize)).p_expr
                                }, reg_arg + 2 + i)
                        };
                    } else {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 178, i_csr, i, reg_arg + 2 + i)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
                    }
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
            unsafe { sqlite3_vdbe_add_op2(v, 137, i_csr, reg_arg) };
            if !(p_rowid).is_null() {
                unsafe { sqlite3_expr_code(p_parse, p_rowid, reg_arg + 1) };
            } else {
                unsafe { sqlite3_vdbe_add_op2(v, 137, i_csr, reg_arg + 1) };
            }
        } else {
            let mut p_pk_1: *const Index = core::ptr::null();
            /// PRIMARY KEY index
            let mut i_pk_1: i16 = 0 as i16;

            /// PRIMARY KEY column
            (p_pk_1 = unsafe { sqlite3_primary_key_index(p_tab) });
            { let _ = 0; };
            { let _ = 0; };
            i_pk_1 =
                unsafe { *unsafe { (*p_pk_1).ai_column.offset(0 as isize) } };
            unsafe {
                sqlite3_vdbe_add_op3(v, 178, i_csr, i_pk_1 as i32, reg_arg)
            };
            unsafe {
                sqlite3_vdbe_add_op2(v, 83, reg_arg + 2 + i_pk_1 as i32,
                    reg_arg + 1)
            };
        }
        e_one_pass =
            unsafe {
                sqlite3_where_ok_one_pass(p_w_info,
                    &raw mut a_dummy[0 as usize] as *mut i32)
            };

        /// There is no ONEPASS_MULTI on virtual tables
        { let _ = 0; };
        if e_one_pass != 0 {

            /// If using the onepass strategy, no-op out the OP_OpenEphemeral coded
            ///* above.
            unsafe { sqlite3_vdbe_change_to_noop(v, addr) };
            unsafe { sqlite3_vdbe_add_op1(v, 124, i_csr) };
        } else {

            /// Create a record from the argument register contents and insert it into
            ///* the ephemeral table.
            unsafe { sqlite3_multi_write(p_parse) };
            unsafe { sqlite3_vdbe_add_op3(v, 99, reg_arg, n_arg, reg_rec) };
            unsafe { sqlite3_vdbe_add_op2(v, 129, ephem_tab, reg_rowid) };
            unsafe {
                sqlite3_vdbe_add_op3(v, 130, ephem_tab, reg_rec, reg_rowid)
            };
        }
    }
    if e_one_pass == 0 {
        if unsafe { (*p_src).n_src } == 1 {
            unsafe { sqlite3_where_end(p_w_info) };
        }

        /// Begin scanning through the ephemeral table.
        (addr = unsafe { sqlite3_vdbe_add_op1(v, 36, ephem_tab) });
        {
            i = 0;
            '__b5: loop {
                if !(i < n_arg) { break '__b5; }
                '__c5: loop {
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 96, ephem_tab, i, reg_arg + i)
                    };
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    unsafe { sqlite3_vtab_make_writable(p_parse, p_tab) };
    unsafe { sqlite3_vdbe_add_op4(v, 7, 0, n_arg, reg_arg, p_v_tab, -12) };
    unsafe {
        sqlite3_vdbe_change_p5(v,
            if on_error == 11 { 2 } else { on_error } as u16)
    };
    unsafe { sqlite3_may_abort(p_parse) };
    if e_one_pass == 0 {
        unsafe { sqlite3_vdbe_add_op2(v, 40, ephem_tab, addr + 1) };
        unsafe { sqlite3_vdbe_jump_here(v, addr) };
        unsafe { sqlite3_vdbe_add_op2(v, 124, ephem_tab, 0) };
    } else { unsafe { sqlite3_where_end(p_w_info) }; }
}

///* Process an UPDATE statement.
///*
///*   UPDATE OR IGNORE tbl SET a=b, c=d FROM tbl2... WHERE e<5 AND f NOT NULL;
///*          \_______/ \_/     \______/      \_____/       \________________/
///*           onError   |      pChanges         |                pWhere
///*                     \_______________________/
///*                               pTabList
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_update(p_parse: *mut Parse,
    p_tab_list: *mut SrcList, p_changes: *mut ExprList, p_where: *mut Expr,
    on_error: i32, mut p_order_by: *mut ExprList, mut p_limit: *mut Expr,
    p_upsert: *mut Upsert) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        /// Loop counters
        let mut p_tab: *mut Table = core::ptr::null_mut();
        /// The table to be updated
        let mut addr_top: i32 = 0;
        /// VDBE instruction address of the start of the loop
        let mut p_w_info: *mut WhereInfo = core::ptr::null_mut();
        /// Information about the WHERE clause
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// The virtual database engine
        let mut p_idx: *mut Index = core::ptr::null_mut();
        /// For looping over indices
        let mut p_pk: *mut Index = core::ptr::null_mut();
        /// The PRIMARY KEY index for WITHOUT ROWID tables
        let mut n_idx: i32 = 0;
        /// Number of indices that need updating
        let mut n_all_idx: i32 = 0;
        /// Total number of indexes
        let mut i_base_cur: i32 = 0;
        /// Base cursor number
        let mut i_data_cur: i32 = 0;
        /// Cursor for the canonical data btree
        let mut i_idx_cur: i32 = 0;
        /// Cursor for the first index
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// The database structure
        let mut a_reg_idx: *mut i32 = core::ptr::null_mut();
        /// Registers for to each index and the main table
        let mut a_x_ref: *mut i32 = core::ptr::null_mut();
        /// aXRef[i] is the index in pChanges->a[] of the
        ///* an expression for the i-th column of the table.
        ///* aXRef[i]==-1 if the i-th column is not changed.
        let mut a_to_open: *mut u8 = core::ptr::null_mut();
        /// 1 for tables and indices to be opened
        let mut chng_pk: u8 = 0 as u8;
        /// PRIMARY KEY changed in a WITHOUT ROWID table
        let mut chng_rowid: u8 = 0 as u8;
        /// Rowid changed in a normal table
        let mut chng_key: u8 = 0 as u8;
        /// Either chngPk or chngRowid
        let mut p_rowid_expr: *mut Expr = core::ptr::null_mut();
        /// Expression defining the new record number
        let mut i_rowid_expr: i32 = 0;
        /// Index of "rowid=" (or IPK) assignment in pChanges
        let mut s_context: AuthContext = unsafe { core::mem::zeroed() };
        /// The authorization context
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        /// The name-context to resolve expressions in
        let mut i_db: i32 = 0;
        /// Database containing the table being updated
        let mut e_one_pass: i32 = 0;
        /// ONEPASS_XXX value from where.c
        let mut has_fk: i32 = 0;
        /// True if foreign key processing is required
        let mut label_break: i32 = 0;
        /// Jump here to break out of UPDATE loop
        let mut label_continue: i32 = 0;
        /// Jump here to continue next step of UPDATE loop
        let mut flags: i32 = 0;
        /// Flags for sqlite3WhereBegin()
        let mut is_view: i32 = 0;
        /// True when updating a view (INSTEAD OF trigger)
        let mut p_trigger: *mut Trigger = core::ptr::null_mut();
        /// List of triggers on pTab, if required
        let mut tmask: i32 = 0;
        /// Mask of TRIGGER_BEFORE|TRIGGER_AFTER
        let mut newmask: i32 = 0;
        /// Mask of NEW.* columns accessed by BEFORE triggers
        let mut i_eph: i32 = 0;
        /// Ephemeral table holding all primary key values
        let mut n_key: i32 = 0;
        /// Number of elements in regKey for WITHOUT ROWID
        let mut ai_cur_one_pass: [i32; 2] = [0; 2];
        /// The write cursors opened by WHERE_ONEPASS
        let mut addr_open: i32 = 0;
        /// Address of OP_OpenEphemeral
        let mut i_pk: i32 = 0;
        /// First of nPk cells holding PRIMARY KEY value
        let mut n_pk: i16 = 0 as i16;
        /// Number of components of the PRIMARY KEY
        let mut b_replace: i32 = 0;
        /// True if REPLACE conflict resolution might happen
        let mut b_finish_seek: i32 = 0;
        /// The OP_FinishSeek opcode is needed
        let mut n_change_from: i32 = 0;
        /// If there is a FROM, pChanges->nExpr, else 0
        /// Register Allocations
        let mut reg_row_count: i32 = 0;
        /// A count of rows changed
        let mut reg_old_rowid: i32 = 0;
        /// The old rowid
        let mut reg_new_rowid: i32 = 0;
        /// The new rowid
        let mut reg_new: i32 = 0;
        /// Content of the NEW.* table in triggers
        let mut reg_old: i32 = 0;
        /// Content of OLD.* table in triggers
        let mut reg_row_set: i32 = 0;
        /// Rowset of rows to be updated
        let mut reg_key: i32 = 0;
        /// composite PRIMARY KEY value
        /// Locate the table which we want to update.
        /// Figure out if we have any triggers and if the table being
        ///* updated is a view.
        /// If there was a FROM clause, set nChangeFrom to the number of expressions
        ///* in the change-list. Otherwise, set it to 0. There cannot be a FROM
        ///* clause if this function is being called to generate code for part of
        ///* an UPSERT statement.
        /// Allocate a cursors for the main database table and for all indices.
        ///* The index cursors might not be used, but if they are used they
        ///* need to occur right after the database cursor.  So go ahead and
        ///* allocate enough space, just in case.
        /// On an UPSERT, reuse the same cursors already opened by INSERT
        /// Allocate space for aXRef[], aRegIdx[], and aToOpen[].
        ///* Initialize aXRef[] and aToOpen[] to their default values.
        /// Initialize the name-context
        /// Begin generating code.
        /// Resolve the column names in all the expressions of the
        ///* of the UPDATE statement.  Also find the column index
        ///* for each column to be updated in the pChanges array.  For each
        ///* column to be updated, make sure we have authorization to change
        ///* that column.
        /// If this is an UPDATE with a FROM clause, do not resolve expressions
        ///* here. The call to sqlite3Select() below will do that.
        let mut rc: i32 = 0;
        /// Mark generated columns as changing if their generator expressions
        ///* reference any changing column.  The actual aXRef[] value for
        ///* generated expressions is not used, other than to check to see that it
        ///* is non-negative, so the value of aXRef[] for generated columns can be
        ///* set to any non-negative number.  We use 99999 so that the value is
        ///* obvious when looking at aXRef[] in a symbolic debugger.
        let mut b_progress: i32 = 0;
        /// The SET expressions are not actually used inside the WHERE loop.
        ///* So reset the colUsed mask. Unless this is a virtual table. In that
        ///* case, set all bits of the colUsed mask (to ensure that the virtual
        ///* table implementation makes all columns available).
        /// There is one entry in the aRegIdx[] array for each index on the table
        ///* being updated.  Fill in aRegIdx[] with a register number that will hold
        ///* the key for accessing each index.
        let mut reg: i32 = 0;
        /// Register storing the table record
        /// If REPLACE conflict resolution might be invoked, open cursors on all
        ///* indexes in case they are needed to delete records.
        /// Allocate required registers.
        /// For now, regRowSet and aRegIdx[nAllIdx] share the same register.
        ///* If regRowSet turns out to be needed, then aRegIdx[nAllIdx] will be
        ///* reallocated.  aRegIdx[nAllIdx] is the register in which the main
        ///* table record is written.  regRowSet holds the RowSet for the
        ///* two-pass update algorithm.
        /// Start the view context.
        /// If we are trying to update a view, realize that view into
        ///* an ephemeral table.
        /// Resolve the column names in all the expressions in the
        ///* WHERE clause.
        /// Virtual tables must be handled separately
        /// Jump to labelBreak to abandon further processing of this UPDATE
        /// Not an UPSERT.  Normal processing.  Begin by
        ///* initialize the count of updated rows
        let mut n_eph_col: i32 = 0;
        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
        /// If this is an UPSERT, then all cursors have already been opened by
        ///* the outer INSERT and the data cursor should be pointing at the row
        ///* that is to be updated.  So bypass the code that searches for the
        ///* row(s) to be updated.
        /// Begin the database scan.
        ///*
        ///* Do not consider a single-pass strategy for a multi-row update if
        ///* there is anything that might disrupt the cursor being used to do
        ///* the UPDATE:
        ///*   (1) This is a nested UPDATE
        ///*   (2) There are triggers
        ///*   (3) There are FOREIGN KEY constraints
        ///*   (4) There are REPLACE conflict handlers
        ///*   (5) There are subqueries in the WHERE clause
        /// A one-pass strategy that might update more than one row may not
        ///* be used if any column of the index used for the scan is being
        ///* updated. Otherwise, if there is an index on "b", statements like
        ///* the following could create an infinite loop:
        ///*
        ///*   UPDATE t1 SET b=b+1 WHERE b>?
        ///*
        ///* Fall back to ONEPASS_OFF if where.c has selected a ONEPASS_MULTI
        ///* strategy that uses an index for which one or more columns are being
        ///* updated.
        let mut i_cur: i32 = 0;
        /// Read the rowid of the current row of the WHERE scan. In ONEPASS_OFF
        ///* mode, write the rowid into the FIFO. In either of the one-pass modes,
        ///* leave it in register regOldRowid.
        /// Read the PK of the current row into an array of registers. In
        ///* ONEPASS_OFF mode, serialize the array into a record and store it in
        ///* the ephemeral table. Or, in ONEPASS_SINGLE or MULTI mode, change
        ///* the OP_OpenEphemeral instruction to a Noop (the ephemeral table
        ///* is not required) and leave the PK fields in the array of registers.
        let mut addr_once: i32 = 0;
        let mut i_not_used1: i32 = 0;
        let mut i_not_used2: i32 = 0;
        /// Open every index that needs updating.
        /// Top of the update loop
        /// If the rowid value will change, set register regNewRowid to
        ///* contain the new value. If the rowid is not being modified,
        ///* then regNewRowid is the same register as regOldRowid, which is
        ///* already populated.
        /// Compute the old pre-UPDATE content of the row being changed, if that
        ///* information is needed
        let mut oldmask: u32 = 0 as u32;
        let mut col_flags: u32 = 0 as u32;
        /// Populate the array of registers beginning at regNew with the new
        ///* row data. This array is used to check constants, create the new
        ///* table and index records, and as the values for any new.* references
        ///* made by triggers.
        ///*
        ///* If there are one or more BEFORE triggers, then do not populate the
        ///* registers associated with columns that are (a) not modified by
        ///* this UPDATE statement and (b) not accessed by new.* references. The
        ///* values for registers not modified by the UPDATE must be reloaded from
        ///* the database after the BEFORE triggers are fired anyway (as the trigger
        ///* may have modified them). So not loading those that are not going to
        ///* be used eliminates some redundant opcodes.
        let mut n_off: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s7:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        unsafe { sqlite3_auth_context_pop(&mut s_context) };
                        __state = 446;
                    }
                    3 => { __state = 4; }
                    4 => { addr_top = 0; __state = 5; }
                    5 => { p_w_info = core::ptr::null_mut(); __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { __state = 14; }
                    14 => { __state = 15; }
                    15 => { a_reg_idx = core::ptr::null_mut(); __state = 16; }
                    16 => { a_x_ref = core::ptr::null_mut(); __state = 17; }
                    17 => { __state = 18; }
                    18 => { __state = 19; }
                    19 => { __state = 20; }
                    20 => { __state = 21; }
                    21 => {
                        p_rowid_expr = core::ptr::null_mut();
                        __state = 22;
                    }
                    22 => { i_rowid_expr = -1; __state = 23; }
                    23 => { __state = 24; }
                    24 => { __state = 25; }
                    25 => { __state = 26; }
                    26 => { __state = 27; }
                    27 => { __state = 28; }
                    28 => { __state = 29; }
                    29 => { __state = 30; }
                    30 => { __state = 31; }
                    31 => { __state = 32; }
                    32 => { __state = 33; }
                    33 => { __state = 34; }
                    34 => { __state = 35; }
                    35 => { i_eph = 0; __state = 36; }
                    36 => { n_key = 0; __state = 37; }
                    37 => { __state = 38; }
                    38 => { addr_open = 0; __state = 39; }
                    39 => { i_pk = 0; __state = 40; }
                    40 => { n_pk = 0 as i16; __state = 41; }
                    41 => { b_replace = 0; __state = 42; }
                    42 => { b_finish_seek = 1; __state = 43; }
                    43 => { n_change_from = 0; __state = 44; }
                    44 => { reg_row_count = 0; __state = 45; }
                    45 => { reg_old_rowid = 0; __state = 46; }
                    46 => { reg_new_rowid = 0; __state = 47; }
                    47 => { reg_new = 0; __state = 48; }
                    48 => { reg_old = 0; __state = 49; }
                    49 => { reg_row_set = 0; __state = 50; }
                    50 => { reg_key = 0; __state = 51; }
                    51 => {
                        unsafe {
                            memset(&raw mut s_context as *mut (), 0,
                                core::mem::size_of::<AuthContext>() as u64)
                        };
                        __state = 52;
                    }
                    52 => { db = unsafe { (*p_parse).db }; __state = 53; }
                    53 => { { let _ = 0; }; __state = 54; }
                    54 => {
                        if unsafe { (*p_parse).n_err } != 0 {
                            __state = 56;
                        } else { __state = 55; }
                    }
                    55 => { { let _ = 0; }; __state = 57; }
                    56 => { __state = 2; }
                    57 => {
                        p_tab =
                            unsafe { sqlite3_src_list_lookup(p_parse, p_tab_list) };
                        __state = 58;
                    }
                    58 => {
                        if p_tab == core::ptr::null_mut() {
                            __state = 60;
                        } else { __state = 59; }
                    }
                    59 => {
                        i_db =
                            unsafe {
                                sqlite3_schema_to_index(unsafe { (*p_parse).db },
                                    unsafe { (*p_tab).p_schema })
                            };
                        __state = 61;
                    }
                    60 => { __state = 2; }
                    61 => {
                        p_trigger =
                            unsafe {
                                sqlite3_triggers_exist(p_parse, p_tab, 130, p_changes,
                                    &mut tmask)
                            };
                        __state = 62;
                    }
                    62 => {
                        is_view =
                            (unsafe { (*p_tab).e_tab_type } as i32 == 2) as i32;
                        __state = 63;
                    }
                    63 => { { let _ = 0; }; __state = 64; }
                    64 => {
                        n_change_from =
                            if unsafe { (*p_tab_list).n_src } > 1 {
                                unsafe { (*p_changes).n_expr }
                            } else { 0 };
                        __state = 65;
                    }
                    65 => { { let _ = 0; }; __state = 66; }
                    66 => {
                        if unsafe { sqlite3_view_get_column_names(p_parse, p_tab) }
                                != 0 {
                            __state = 68;
                        } else { __state = 67; }
                    }
                    67 => {
                        if unsafe {
                                    sqlite3_is_read_only(p_parse, p_tab, p_trigger)
                                } != 0 {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    68 => { __state = 2; }
                    69 => {
                        i_base_cur =
                            {
                                i_data_cur =
                                    {
                                        let __p = unsafe { &mut (*p_parse).n_tab };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                i_data_cur
                            };
                        __state = 71;
                    }
                    70 => { __state = 2; }
                    71 => { i_idx_cur = i_data_cur + 1; __state = 72; }
                    72 => {
                        p_pk =
                            if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                                core::ptr::null_mut()
                            } else { unsafe { sqlite3_primary_key_index(p_tab) } };
                        __state = 73;
                    }
                    73 => { __state = 74; }
                    74 => {
                        { n_idx = 0; p_idx = unsafe { (*p_tab).p_index } };
                        __state = 76;
                    }
                    75 => {
                        if !(p_upsert).is_null() {
                            __state = 82;
                        } else { __state = 81; }
                    }
                    76 => {
                        if !(p_idx).is_null() {
                            __state = 77;
                        } else { __state = 75; }
                    }
                    77 => {
                        if p_pk == p_idx { __state = 80; } else { __state = 79; }
                    }
                    78 => {
                        {
                            p_idx = unsafe { (*p_idx).p_next };
                            { let __p = &mut n_idx; let __t = *__p; *__p += 1; __t }
                        };
                        __state = 76;
                    }
                    79 => {
                        {
                            let __p = unsafe { &mut (*p_parse).n_tab };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 78;
                    }
                    80 => {
                        i_data_cur = unsafe { (*p_parse).n_tab };
                        __state = 79;
                    }
                    81 => {
                        unsafe {
                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)).i_cursor = i_data_cur
                        };
                        __state = 85;
                    }
                    82 => {
                        i_data_cur = unsafe { (*p_upsert).i_data_cur };
                        __state = 83;
                    }
                    83 => {
                        i_idx_cur = unsafe { (*p_upsert).i_idx_cur };
                        __state = 84;
                    }
                    84 => {
                        unsafe { (*p_parse).n_tab = i_base_cur };
                        __state = 81;
                    }
                    85 => {
                        a_x_ref =
                            unsafe {
                                    sqlite3_db_malloc_raw_nn(db,
                                        core::mem::size_of::<i32>() as u64 *
                                                    (unsafe { (*p_tab).n_col } as i32 + n_idx + 1) as u64 +
                                                n_idx as u64 + 2 as u64)
                                } as *mut i32;
                        __state = 86;
                    }
                    86 => {
                        if a_x_ref == core::ptr::null_mut() {
                            __state = 88;
                        } else { __state = 87; }
                    }
                    87 => {
                        a_reg_idx =
                            unsafe {
                                a_x_ref.offset(unsafe { (*p_tab).n_col } as isize)
                            };
                        __state = 89;
                    }
                    88 => { __state = 2; }
                    89 => {
                        a_to_open =
                            unsafe {
                                    unsafe {
                                        a_reg_idx.offset(n_idx as isize).offset(1 as isize)
                                    }
                                } as *mut u8;
                        __state = 90;
                    }
                    90 => {
                        unsafe {
                            memset(a_to_open as *mut (), 1, (n_idx + 1) as u64)
                        };
                        __state = 91;
                    }
                    91 => {
                        unsafe {
                            *a_to_open.offset((n_idx + 1) as isize) = 0 as u8
                        };
                        __state = 92;
                    }
                    92 => { i = 0; __state = 94; }
                    93 => {
                        unsafe {
                            memset(&raw mut s_nc as *mut (), 0,
                                core::mem::size_of::<NameContext>() as u64)
                        };
                        __state = 97;
                    }
                    94 => {
                        if i < unsafe { (*p_tab).n_col } as i32 {
                            __state = 95;
                        } else { __state = 93; }
                    }
                    95 => {
                        unsafe { *a_x_ref.offset(i as isize) = -1 };
                        __state = 96;
                    }
                    96 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 94;
                    }
                    97 => { s_nc.p_parse = p_parse; __state = 98; }
                    98 => { s_nc.p_src_list = p_tab_list; __state = 99; }
                    99 => { s_nc.u_nc.p_upsert = p_upsert; __state = 100; }
                    100 => { s_nc.nc_flags = 512; __state = 101; }
                    101 => {
                        v = unsafe { sqlite3_get_vdbe(p_parse) };
                        __state = 102;
                    }
                    102 => {
                        if v == core::ptr::null_mut() {
                            __state = 104;
                        } else { __state = 103; }
                    }
                    103 => {
                        chng_rowid = { chng_pk = 0 as u8; chng_pk };
                        __state = 105;
                    }
                    104 => { __state = 2; }
                    105 => { i = 0; __state = 107; }
                    106 => { { let _ = 0; }; __state = 139; }
                    107 => {
                        if i < unsafe { (*p_changes).n_expr } {
                            __state = 108;
                        } else { __state = 106; }
                    }
                    108 => {
                        if n_change_from == 0 &&
                                unsafe {
                                        sqlite3_resolve_expr_names(&mut s_nc,
                                            unsafe {
                                                (*(unsafe { (*p_changes).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                            })
                                    } != 0 {
                            __state = 111;
                        } else { __state = 110; }
                    }
                    109 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 107;
                    }
                    110 => {
                        j =
                            unsafe {
                                sqlite3_column_index(p_tab,
                                    unsafe {
                                            (*(unsafe { (*p_changes).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).z_e_name
                                        } as *const i8)
                            };
                        __state = 112;
                    }
                    111 => { __state = 2; }
                    112 => {
                        if j >= 0 { __state = 114; } else { __state = 115; }
                    }
                    113 => { __state = 134; }
                    114 => {
                        if j == unsafe { (*p_tab).i_p_key } as i32 {
                            __state = 117;
                        } else { __state = 118; }
                    }
                    115 => {
                        if p_pk == core::ptr::null_mut() &&
                                unsafe {
                                        sqlite3_is_rowid(unsafe {
                                                    (*(unsafe { (*p_changes).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i as isize)).z_e_name
                                                } as *const i8)
                                    } != 0 {
                            __state = 127;
                        } else { __state = 128; }
                    }
                    116 => {
                        unsafe { *a_x_ref.offset(j as isize) = i };
                        __state = 113;
                    }
                    117 => { chng_rowid = 1 as u8; __state = 119; }
                    118 => {
                        if !(p_pk).is_null() &&
                                unsafe {
                                                (*unsafe { (*p_tab).a_col.offset(j as isize) }).col_flags
                                            } as i32 & 1 != 0 {
                            __state = 121;
                        } else { __state = 122; }
                    }
                    119 => {
                        p_rowid_expr =
                            unsafe {
                                (*(unsafe { (*p_changes).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)).p_expr
                            };
                        __state = 120;
                    }
                    120 => { i_rowid_expr = i; __state = 116; }
                    121 => { chng_pk = 1 as u8; __state = 116; }
                    122 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(j as isize) }).col_flags
                                        } as i32 & 96 != 0 {
                            __state = 123;
                        } else { __state = 116; }
                    }
                    123 => { __state = 124; }
                    124 => { __state = 125; }
                    125 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot UPDATE generated column \"%s\"".as_ptr() as *mut i8
                                    as *const i8,
                                unsafe {
                                    (*unsafe { (*p_tab).a_col.offset(j as isize) }).z_cn_name
                                })
                        };
                        __state = 126;
                    }
                    126 => { __state = 2; }
                    127 => { j = -1; __state = 129; }
                    128 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"no such column: %s".as_ptr() as *mut i8 as *const i8,
                                unsafe {
                                    (*(unsafe { (*p_changes).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).z_e_name
                                })
                        };
                        __state = 132;
                    }
                    129 => { chng_rowid = 1 as u8; __state = 130; }
                    130 => {
                        p_rowid_expr =
                            unsafe {
                                (*(unsafe { (*p_changes).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)).p_expr
                            };
                        __state = 131;
                    }
                    131 => { i_rowid_expr = i; __state = 113; }
                    132 => {
                        unsafe { (*p_parse).set_check_schema(1 as Bft as u32) };
                        __state = 133;
                    }
                    133 => { __state = 2; }
                    134 => {
                        rc =
                            unsafe {
                                sqlite3_auth_check(p_parse, 23,
                                    unsafe { (*p_tab).z_name } as *const i8,
                                    if j < 0 {
                                            c"ROWID".as_ptr() as *mut i8
                                        } else {
                                            unsafe {
                                                (*unsafe { (*p_tab).a_col.offset(j as isize) }).z_cn_name
                                            }
                                        } as *const i8,
                                    unsafe {
                                            (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                        } as *const i8)
                            };
                        __state = 135;
                    }
                    135 => {
                        if rc == 1 { __state = 136; } else { __state = 137; }
                    }
                    136 => { __state = 2; }
                    137 => {
                        if rc == 2 { __state = 138; } else { __state = 109; }
                    }
                    138 => {
                        unsafe { *a_x_ref.offset(j as isize) = -1 };
                        __state = 109;
                    }
                    139 => { { let _ = 0; }; __state = 140; }
                    140 => { { let _ = 0; }; __state = 141; }
                    141 => {
                        chng_key = (chng_rowid as i32 + chng_pk as i32) as u8;
                        __state = 142;
                    }
                    142 => {
                        if unsafe { (*p_tab).tab_flags } & 96 as u32 != 0 {
                            __state = 144;
                        } else { __state = 143; }
                    }
                    143 => {
                        unsafe {
                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)).col_used =
                                if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                                    -1i32 as Bitmask
                                } else { 0 as Bitmask }
                        };
                        __state = 159;
                    }
                    144 => { __state = 145; }
                    145 => { __state = 146; }
                    146 => { __state = 147; }
                    147 => { b_progress = 0; __state = 149; }
                    148 => {
                        if b_progress != 0 {
                            __state = 147;
                        } else { __state = 143; }
                    }
                    149 => { i = 0; __state = 150; }
                    150 => {
                        if i < unsafe { (*p_tab).n_col } as i32 {
                            __state = 151;
                        } else { __state = 148; }
                    }
                    151 => {
                        if unsafe { *a_x_ref.offset(i as isize) } >= 0 {
                            __state = 154;
                        } else { __state = 153; }
                    }
                    152 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 150;
                    }
                    153 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                        } as i32 & 96 == 0 {
                            __state = 156;
                        } else { __state = 155; }
                    }
                    154 => { __state = 152; }
                    155 => {
                        if unsafe {
                                    sqlite3_expr_references_updated_column(unsafe {
                                            sqlite3_column_expr(p_tab,
                                                unsafe {
                                                    &mut *unsafe { (*p_tab).a_col.offset(i as isize) }
                                                })
                                        }, a_x_ref, chng_rowid as i32)
                                } != 0 {
                            __state = 157;
                        } else { __state = 152; }
                    }
                    156 => { __state = 152; }
                    157 => {
                        unsafe { *a_x_ref.offset(i as isize) = 99999 };
                        __state = 158;
                    }
                    158 => { b_progress = 1; __state = 152; }
                    159 => {
                        has_fk =
                            unsafe {
                                sqlite3_fk_required(p_parse, p_tab, a_x_ref,
                                    chng_key as i32)
                            };
                        __state = 160;
                    }
                    160 => {
                        if on_error == 5 { __state = 162; } else { __state = 161; }
                    }
                    161 => {
                        { n_all_idx = 0; p_idx = unsafe { (*p_tab).p_index } };
                        __state = 164;
                    }
                    162 => { b_replace = 1; __state = 161; }
                    163 => {
                        unsafe {
                            *a_reg_idx.offset(n_all_idx as isize) =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 183;
                    }
                    164 => {
                        if !(p_idx).is_null() {
                            __state = 165;
                        } else { __state = 163; }
                    }
                    165 => { __state = 167; }
                    166 => {
                        {
                            p_idx = unsafe { (*p_idx).p_next };
                            { let __p = &mut n_all_idx; let __t = *__p; *__p += 1; __t }
                        };
                        __state = 164;
                    }
                    167 => {
                        if chng_key != 0 || has_fk > 1 || p_idx == p_pk ||
                                index_where_clause_might_change(unsafe { &*p_idx }, a_x_ref,
                                        chng_rowid as i32) != 0 {
                            __state = 169;
                        } else { __state = 170; }
                    }
                    168 => {
                        if reg == 0 { __state = 182; } else { __state = 181; }
                    }
                    169 => {
                        reg =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 171;
                    }
                    170 => { reg = 0; __state = 172; }
                    171 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_idx).n_column } as i32
                        };
                        __state = 168;
                    }
                    172 => { i = 0; __state = 173; }
                    173 => {
                        if i < unsafe { (*p_idx).n_key_col } as i32 {
                            __state = 174;
                        } else { __state = 168; }
                    }
                    174 => {
                        if index_column_is_being_updated(unsafe { &*p_idx }, i,
                                    a_x_ref, chng_rowid as i32) != 0 {
                            __state = 176;
                        } else { __state = 175; }
                    }
                    175 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 173;
                    }
                    176 => {
                        reg =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 177;
                    }
                    177 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_idx).n_column } as i32
                        };
                        __state = 178;
                    }
                    178 => {
                        if on_error == 11 &&
                                unsafe { (*p_idx).on_error } as i32 == 5 {
                            __state = 180;
                        } else { __state = 179; }
                    }
                    179 => { __state = 168; }
                    180 => { b_replace = 1; __state = 179; }
                    181 => {
                        unsafe { *a_reg_idx.offset(n_all_idx as isize) = reg };
                        __state = 166;
                    }
                    182 => {
                        unsafe {
                            *a_to_open.offset((n_all_idx + 1) as isize) = 0 as u8
                        };
                        __state = 181;
                    }
                    183 => {
                        if b_replace != 0 { __state = 185; } else { __state = 184; }
                    }
                    184 => {
                        if unsafe { (*p_parse).nested } as i32 == 0 {
                            __state = 187;
                        } else { __state = 186; }
                    }
                    185 => {
                        unsafe {
                            memset(a_to_open as *mut (), 1, (n_idx + 1) as u64)
                        };
                        __state = 184;
                    }
                    186 => {
                        unsafe {
                            sqlite3_begin_write_operation(p_parse,
                                (!(p_trigger).is_null() || has_fk != 0) as i32, i_db)
                        };
                        __state = 188;
                    }
                    187 => {
                        unsafe { sqlite3_vdbe_count_changes(v) };
                        __state = 186;
                    }
                    188 => {
                        if !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0
                            {
                            __state = 190;
                        } else { __state = 189; }
                    }
                    189 => {
                        if is_view != 0 { __state = 201; } else { __state = 200; }
                    }
                    190 => { { let _ = 0; }; __state = 191; }
                    191 => {
                        reg_row_set =
                            unsafe { *a_reg_idx.offset(n_all_idx as isize) };
                        __state = 192;
                    }
                    192 => {
                        reg_old_rowid =
                            {
                                reg_new_rowid =
                                    {
                                        let __p = unsafe { &mut (*p_parse).n_mem };
                                        *__p += 1;
                                        *__p
                                    };
                                reg_new_rowid
                            };
                        __state = 193;
                    }
                    193 => {
                        if chng_pk != 0 || !(p_trigger).is_null() || has_fk != 0 {
                            __state = 195;
                        } else { __state = 194; }
                    }
                    194 => {
                        if chng_key != 0 || !(p_trigger).is_null() || has_fk != 0 {
                            __state = 198;
                        } else { __state = 197; }
                    }
                    195 => {
                        reg_old = unsafe { (*p_parse).n_mem } + 1;
                        __state = 196;
                    }
                    196 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_tab).n_col } as i32
                        };
                        __state = 194;
                    }
                    197 => {
                        reg_new = unsafe { (*p_parse).n_mem } + 1;
                        __state = 199;
                    }
                    198 => {
                        reg_new_rowid =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 197;
                    }
                    199 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_tab).n_col } as i32
                        };
                        __state = 189;
                    }
                    200 => {
                        if n_change_from == 0 && is_view != 0 {
                            __state = 203;
                        } else { __state = 202; }
                    }
                    201 => {
                        unsafe {
                            sqlite3_auth_context_push(p_parse, &mut s_context,
                                unsafe { (*p_tab).z_name } as *const i8)
                        };
                        __state = 200;
                    }
                    202 => {
                        if n_change_from == 0 &&
                                unsafe { sqlite3_resolve_expr_names(&mut s_nc, p_where) } !=
                                    0 {
                            __state = 207;
                        } else { __state = 206; }
                    }
                    203 => {
                        unsafe {
                            sqlite3_materialize_view(p_parse, p_tab, p_where,
                                p_order_by, p_limit, i_data_cur)
                        };
                        __state = 204;
                    }
                    204 => {
                        p_order_by = core::ptr::null_mut();
                        __state = 205;
                    }
                    205 => { p_limit = core::ptr::null_mut(); __state = 202; }
                    206 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                            __state = 209;
                        } else { __state = 208; }
                    }
                    207 => { __state = 2; }
                    208 => {
                        label_continue =
                            {
                                label_break = unsafe { sqlite3_vdbe_make_label(p_parse) };
                                label_break
                            };
                        __state = 211;
                    }
                    209 => {
                        update_virtual_table(p_parse, p_tab_list, p_tab,
                            unsafe { &*p_changes }, p_rowid_expr, a_x_ref, p_where,
                            on_error);
                        __state = 210;
                    }
                    210 => { __state = 2; }
                    211 => {
                        if unsafe { (*db).flags } & (1 as u64) << 32 != 0 as u64 &&
                                            (unsafe { (*p_parse).p_trigger_tab }).is_null() as i32 != 0
                                        && (unsafe { (*p_parse).nested } == 0) as i32 != 0 &&
                                    (unsafe { (*p_parse).b_returning() } == 0) as i32 != 0 &&
                                p_upsert == core::ptr::null_mut() {
                            __state = 213;
                        } else { __state = 212; }
                    }
                    212 => {
                        if n_change_from == 0 &&
                                unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                            __state = 216;
                        } else { __state = 217; }
                    }
                    213 => {
                        reg_row_count =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 214;
                    }
                    214 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_row_count) };
                        __state = 212;
                    }
                    215 => {
                        if n_change_from != 0 {
                            __state = 241;
                        } else { __state = 242; }
                    }
                    216 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 77, 0, reg_row_set, reg_old_rowid)
                        };
                        __state = 218;
                    }
                    217 => { { let _ = 0; }; __state = 220; }
                    218 => {
                        i_eph =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 219;
                    }
                    219 => {
                        addr_open =
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 120, i_eph, 0, reg_row_set)
                            };
                        __state = 215;
                    }
                    220 => {
                        n_pk =
                            if !(p_pk).is_null() {
                                    (unsafe { (*p_pk).n_key_col }) as i32
                                } else { 0 } as i16;
                        __state = 221;
                    }
                    221 => {
                        i_pk = unsafe { (*p_parse).n_mem } + 1;
                        __state = 222;
                    }
                    222 => {
                        unsafe { (*p_parse).n_mem += n_pk as i32 };
                        __state = 223;
                    }
                    223 => {
                        unsafe { (*p_parse).n_mem += n_change_from };
                        __state = 224;
                    }
                    224 => {
                        reg_key =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 225;
                    }
                    225 => {
                        if p_upsert == core::ptr::null_mut() {
                            __state = 226;
                        } else { __state = 215; }
                    }
                    226 => {
                        n_eph_col =
                            n_pk as i32 + n_change_from +
                                if is_view != 0 {
                                    (unsafe { (*p_tab).n_col }) as i32
                                } else { 0 };
                        __state = 227;
                    }
                    227 => {
                        i_eph =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 228;
                    }
                    228 => {
                        if !(p_pk).is_null() {
                            __state = 230;
                        } else { __state = 229; }
                    }
                    229 => {
                        addr_open =
                            unsafe { sqlite3_vdbe_add_op2(v, 120, i_eph, n_eph_col) };
                        __state = 231;
                    }
                    230 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 77, 0, i_pk, i_pk + n_pk as i32 - 1)
                        };
                        __state = 229;
                    }
                    231 => {
                        if !(p_pk).is_null() {
                            __state = 233;
                        } else { __state = 232; }
                    }
                    232 => {
                        if n_change_from != 0 {
                            __state = 237;
                        } else { __state = 215; }
                    }
                    233 => {
                        p_key_info =
                            unsafe { sqlite3_key_info_of_index(p_parse, p_pk) };
                        __state = 234;
                    }
                    234 => {
                        if !(p_key_info).is_null() {
                            __state = 235;
                        } else { __state = 232; }
                    }
                    235 => {
                        unsafe { (*p_key_info).n_all_field = n_eph_col as u16 };
                        __state = 236;
                    }
                    236 => {
                        unsafe {
                            sqlite3_vdbe_append_p4(v, p_key_info as *mut (), -9)
                        };
                        __state = 232;
                    }
                    237 => {
                        update_from_select(p_parse, i_eph, p_pk as *const Index,
                            p_changes as *const ExprList, p_tab_list as *const SrcList,
                            p_where as *const Expr, p_order_by as *const ExprList,
                            p_limit as *const Expr);
                        __state = 238;
                    }
                    238 => {
                        if is_view != 0 { __state = 239; } else { __state = 215; }
                    }
                    239 => { i_data_cur = i_eph; __state = 215; }
                    240 => {
                        if p_upsert == core::ptr::null_mut() {
                            __state = 285;
                        } else { __state = 284; }
                    }
                    241 => {
                        unsafe { sqlite3_multi_write(p_parse) };
                        __state = 243;
                    }
                    242 => {
                        if !(p_upsert).is_null() {
                            __state = 247;
                        } else { __state = 248; }
                    }
                    243 => { e_one_pass = 0; __state = 244; }
                    244 => { n_key = n_pk as i32; __state = 245; }
                    245 => { reg_key = i_pk; __state = 240; }
                    246 => {
                        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                            __state = 266;
                        } else { __state = 267; }
                    }
                    247 => { p_w_info = core::ptr::null_mut(); __state = 249; }
                    248 => { flags = 4; __state = 252; }
                    249 => { e_one_pass = 1; __state = 250; }
                    250 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, p_where, label_break, 16)
                        };
                        __state = 251;
                    }
                    251 => { b_finish_seek = 0; __state = 246; }
                    252 => {
                        if (unsafe { (*p_parse).nested } == 0) as i32 != 0 &&
                                                (p_trigger).is_null() as i32 != 0 &&
                                            (has_fk == 0) as i32 != 0 && (chng_key == 0) as i32 != 0 &&
                                    (b_replace == 0) as i32 != 0 &&
                                (p_where == core::ptr::null_mut() ||
                                    !(unsafe { (*p_where).flags } & 4194304 as u32 != 0 as u32)
                                            as i32 != 0) {
                            __state = 254;
                        } else { __state = 253; }
                    }
                    253 => {
                        p_w_info =
                            unsafe {
                                sqlite3_where_begin(p_parse, p_tab_list, p_where,
                                    core::ptr::null_mut(), core::ptr::null_mut(),
                                    core::ptr::null_mut(), flags as u16, i_idx_cur)
                            };
                        __state = 255;
                    }
                    254 => { flags |= 8; __state = 253; }
                    255 => {
                        if p_w_info == core::ptr::null_mut() {
                            __state = 257;
                        } else { __state = 256; }
                    }
                    256 => {
                        e_one_pass =
                            unsafe {
                                sqlite3_where_ok_one_pass(p_w_info,
                                    &raw mut ai_cur_one_pass[0 as usize] as *mut i32)
                            };
                        __state = 258;
                    }
                    257 => { __state = 2; }
                    258 => {
                        b_finish_seek =
                            unsafe { sqlite3_where_uses_deferred_seek(p_w_info) };
                        __state = 259;
                    }
                    259 => {
                        if e_one_pass != 1 {
                            __state = 260;
                        } else { __state = 246; }
                    }
                    260 => {
                        unsafe { sqlite3_multi_write(p_parse) };
                        __state = 261;
                    }
                    261 => {
                        if e_one_pass == 2 {
                            __state = 262;
                        } else { __state = 246; }
                    }
                    262 => {
                        i_cur = ai_cur_one_pass[1 as usize];
                        __state = 263;
                    }
                    263 => {
                        if i_cur >= 0 && i_cur != i_data_cur &&
                                unsafe { *a_to_open.offset((i_cur - i_base_cur) as isize) }
                                    != 0 {
                            __state = 265;
                        } else { __state = 264; }
                    }
                    264 => { { let _ = 0; }; __state = 246; }
                    265 => { e_one_pass = 0; __state = 264; }
                    266 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 137, i_data_cur, reg_old_rowid)
                        };
                        __state = 268;
                    }
                    267 => { i = 0; __state = 274; }
                    268 => {
                        if e_one_pass == 0 {
                            __state = 269;
                        } else { __state = 270; }
                    }
                    269 => {
                        unsafe {
                            *a_reg_idx.offset(n_all_idx as isize) =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 271;
                    }
                    270 => {
                        if addr_open != 0 { __state = 272; } else { __state = 240; }
                    }
                    271 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 130, i_eph, reg_row_set,
                                reg_old_rowid)
                        };
                        __state = 240;
                    }
                    272 => {
                        unsafe { sqlite3_vdbe_change_to_noop(v, addr_open) };
                        __state = 240;
                    }
                    273 => {
                        if e_one_pass != 0 {
                            __state = 278;
                        } else { __state = 279; }
                    }
                    274 => {
                        if i < n_pk as i32 {
                            __state = 275;
                        } else { __state = 273; }
                    }
                    275 => { { let _ = 0; }; __state = 277; }
                    276 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 274;
                    }
                    277 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab, i_data_cur,
                                unsafe { *unsafe { (*p_pk).ai_column.offset(i as isize) } }
                                    as i32, i_pk + i)
                        };
                        __state = 276;
                    }
                    278 => {
                        if addr_open != 0 { __state = 281; } else { __state = 280; }
                    }
                    279 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, i_pk, n_pk as i32, reg_key,
                                unsafe { sqlite3_index_affinity_str(db, p_pk) },
                                n_pk as i32)
                        };
                        __state = 283;
                    }
                    280 => { n_key = n_pk as i32; __state = 282; }
                    281 => {
                        unsafe { sqlite3_vdbe_change_to_noop(v, addr_open) };
                        __state = 280;
                    }
                    282 => { reg_key = i_pk; __state = 240; }
                    283 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_eph, reg_key, i_pk,
                                n_pk as i32)
                        };
                        __state = 240;
                    }
                    284 => { { let _ = 0; }; __state = 339; }
                    285 => {
                        if n_change_from == 0 && e_one_pass != 2 {
                            __state = 287;
                        } else { __state = 286; }
                    }
                    286 => {
                        if (is_view == 0) as i32 != 0 {
                            __state = 289;
                        } else { __state = 288; }
                    }
                    287 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 286;
                    }
                    288 => {
                        if e_one_pass != 0 {
                            __state = 303;
                        } else { __state = 304; }
                    }
                    289 => { addr_once = 0; __state = 290; }
                    290 => { i_not_used1 = 0; __state = 291; }
                    291 => { i_not_used2 = 0; __state = 292; }
                    292 => {
                        if e_one_pass != 0 {
                            __state = 294;
                        } else { __state = 293; }
                    }
                    293 => {
                        if e_one_pass == 2 &&
                                n_idx - (ai_cur_one_pass[1 as usize] >= 0) as i32 > 0 {
                            __state = 299;
                        } else { __state = 298; }
                    }
                    294 => {
                        if ai_cur_one_pass[0 as usize] >= 0 {
                            __state = 296;
                        } else { __state = 295; }
                    }
                    295 => {
                        if ai_cur_one_pass[1 as usize] >= 0 {
                            __state = 297;
                        } else { __state = 293; }
                    }
                    296 => {
                        unsafe {
                            *a_to_open.offset((ai_cur_one_pass[0 as usize] - i_base_cur)
                                            as isize) = 0 as u8
                        };
                        __state = 295;
                    }
                    297 => {
                        unsafe {
                            *a_to_open.offset((ai_cur_one_pass[1 as usize] - i_base_cur)
                                            as isize) = 0 as u8
                        };
                        __state = 293;
                    }
                    298 => {
                        unsafe {
                            sqlite3_open_table_and_indices(p_parse, p_tab, 116, 0 as u8,
                                i_base_cur, a_to_open, &mut i_not_used1, &mut i_not_used2)
                        };
                        __state = 301;
                    }
                    299 => {
                        addr_once = unsafe { sqlite3_vdbe_add_op0(v, 15) };
                        __state = 300;
                    }
                    300 => { __state = 298; }
                    301 => {
                        if addr_once != 0 { __state = 302; } else { __state = 288; }
                    }
                    302 => {
                        unsafe { sqlite3_vdbe_jump_here_or_pop_inst(v, addr_once) };
                        __state = 288;
                    }
                    303 => {
                        if ai_cur_one_pass[0 as usize] != i_data_cur &&
                                ai_cur_one_pass[1 as usize] != i_data_cur {
                            __state = 306;
                        } else { __state = 305; }
                    }
                    304 => {
                        if !(p_pk).is_null() || n_change_from != 0 {
                            __state = 313;
                        } else { __state = 314; }
                    }
                    305 => {
                        if e_one_pass != 1 {
                            __state = 310;
                        } else { __state = 309; }
                    }
                    306 => { { let _ = 0; }; __state = 307; }
                    307 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 28, i_data_cur, label_break,
                                reg_key, n_key)
                        };
                        __state = 308;
                    }
                    308 => { __state = 305; }
                    309 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 51,
                                if !(p_pk).is_null() { reg_key } else { reg_old_rowid },
                                label_break)
                        };
                        __state = 311;
                    }
                    310 => {
                        label_continue =
                            unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 309;
                    }
                    311 => { __state = 312; }
                    312 => { __state = 284; }
                    313 => {
                        label_continue =
                            unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 315;
                    }
                    314 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 36, i_eph, label_break) };
                        __state = 333;
                    }
                    315 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 36, i_eph, label_break) };
                        __state = 316;
                    }
                    316 => { __state = 317; }
                    317 => {
                        addr_top = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 318;
                    }
                    318 => {
                        if n_change_from != 0 {
                            __state = 319;
                        } else { __state = 320; }
                    }
                    319 => {
                        if (is_view == 0) as i32 != 0 {
                            __state = 321;
                        } else { __state = 284; }
                    }
                    320 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 136, i_eph, reg_key) };
                        __state = 331;
                    }
                    321 => {
                        if !(p_pk).is_null() {
                            __state = 322;
                        } else { __state = 323; }
                    }
                    322 => { i = 0; __state = 325; }
                    323 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 137, i_eph, reg_old_rowid)
                        };
                        __state = 329;
                    }
                    324 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 28, i_data_cur, label_continue,
                                i_pk, n_pk as i32)
                        };
                        __state = 328;
                    }
                    325 => {
                        if i < n_pk as i32 {
                            __state = 326;
                        } else { __state = 324; }
                    }
                    326 => {
                        unsafe { sqlite3_vdbe_add_op3(v, 96, i_eph, i, i_pk + i) };
                        __state = 327;
                    }
                    327 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 325;
                    }
                    328 => { __state = 284; }
                    329 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 31, i_data_cur, label_continue,
                                reg_old_rowid)
                        };
                        __state = 330;
                    }
                    330 => { __state = 284; }
                    331 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 28, i_data_cur, label_continue,
                                reg_key, 0)
                        };
                        __state = 332;
                    }
                    332 => { __state = 284; }
                    333 => { __state = 334; }
                    334 => {
                        label_continue =
                            unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 335;
                    }
                    335 => {
                        addr_top =
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 137, i_eph, reg_old_rowid)
                            };
                        __state = 336;
                    }
                    336 => { __state = 337; }
                    337 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 31, i_data_cur, label_continue,
                                reg_old_rowid)
                        };
                        __state = 338;
                    }
                    338 => { __state = 284; }
                    339 => {
                        if chng_rowid != 0 {
                            __state = 341;
                        } else { __state = 340; }
                    }
                    340 => {
                        if chng_pk != 0 || has_fk != 0 || !(p_trigger).is_null() {
                            __state = 348;
                        } else { __state = 347; }
                    }
                    341 => { { let _ = 0; }; __state = 342; }
                    342 => {
                        if n_change_from == 0 {
                            __state = 344;
                        } else { __state = 345; }
                    }
                    343 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 13, reg_new_rowid) };
                        __state = 346;
                    }
                    344 => {
                        unsafe {
                            sqlite3_expr_code(p_parse, p_rowid_expr, reg_new_rowid)
                        };
                        __state = 343;
                    }
                    345 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_eph, i_rowid_expr,
                                reg_new_rowid)
                        };
                        __state = 343;
                    }
                    346 => { __state = 340; }
                    347 => {
                        newmask =
                            unsafe {
                                    sqlite3_trigger_colmask(p_parse, p_trigger, p_changes, 1, 1,
                                        p_tab, on_error)
                                } as i32;
                        __state = 361;
                    }
                    348 => {
                        oldmask =
                            if has_fk != 0 {
                                unsafe { sqlite3_fk_oldmask(p_parse, p_tab) }
                            } else { 0 as u32 };
                        __state = 349;
                    }
                    349 => {
                        oldmask |=
                            unsafe {
                                sqlite3_trigger_colmask(p_parse, p_trigger, p_changes, 0,
                                    1 | 2, p_tab, on_error)
                            };
                        __state = 350;
                    }
                    350 => { i = 0; __state = 352; }
                    351 => {
                        if chng_rowid as i32 == 0 && p_pk == core::ptr::null_mut() {
                            __state = 360;
                        } else { __state = 347; }
                    }
                    352 => {
                        if i < unsafe { (*p_tab).n_col } as i32 {
                            __state = 353;
                        } else { __state = 351; }
                    }
                    353 => {
                        col_flags =
                            unsafe {
                                    (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                } as u32;
                        __state = 355;
                    }
                    354 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 352;
                    }
                    355 => {
                        k =
                            unsafe { sqlite3_table_column_to_storage(p_tab, i as i16) }
                                    as i32 + reg_old;
                        __state = 356;
                    }
                    356 => {
                        if oldmask == 4294967295u32 ||
                                    i < 32 && oldmask & (1 as u32) << i != 0 as u32 ||
                                col_flags & 1 as u32 != 0 as u32 {
                            __state = 357;
                        } else { __state = 358; }
                    }
                    357 => { __state = 359; }
                    358 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, k) };
                        __state = 354;
                    }
                    359 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab, i_data_cur,
                                i, k)
                        };
                        __state = 354;
                    }
                    360 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 82, reg_old_rowid, reg_new_rowid)
                        };
                        __state = 347;
                    }
                    361 => { { i = 0; k = reg_new }; __state = 363; }
                    362 => {
                        if unsafe { (*p_tab).tab_flags } & 96 as u32 != 0 {
                            __state = 384;
                        } else { __state = 383; }
                    }
                    363 => {
                        if i < unsafe { (*p_tab).n_col } as i32 {
                            __state = 364;
                        } else { __state = 362; }
                    }
                    364 => {
                        if i == unsafe { (*p_tab).i_p_key } as i32 {
                            __state = 366;
                        } else { __state = 367; }
                    }
                    365 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t }
                        };
                        __state = 363;
                    }
                    366 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, k) };
                        __state = 365;
                    }
                    367 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                        } as i32 & 96 != 0 {
                            __state = 368;
                        } else { __state = 369; }
                    }
                    368 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                        } as i32 & 32 != 0 {
                            __state = 370;
                        } else { __state = 365; }
                    }
                    369 => {
                        j = unsafe { *a_x_ref.offset(i as isize) };
                        __state = 371;
                    }
                    370 => {
                        { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                        __state = 365;
                    }
                    371 => {
                        if j >= 0 { __state = 372; } else { __state = 373; }
                    }
                    372 => {
                        if n_change_from != 0 {
                            __state = 374;
                        } else { __state = 375; }
                    }
                    373 => {
                        if 0 == tmask & 1 || i > 31 ||
                                newmask as u32 & (1 as u32) << i != 0 {
                            __state = 378;
                        } else { __state = 379; }
                    }
                    374 => {
                        n_off =
                            if is_view != 0 {
                                (unsafe { (*p_tab).n_col }) as i32
                            } else { n_pk as i32 };
                        __state = 376;
                    }
                    375 => {
                        unsafe {
                            sqlite3_expr_code(p_parse,
                                unsafe {
                                    (*(unsafe { (*p_changes).a.as_ptr() } as
                                                    *mut ExprListItem).offset(j as isize)).p_expr
                                }, k)
                        };
                        __state = 365;
                    }
                    376 => { { let _ = 0; }; __state = 377; }
                    377 => {
                        unsafe { sqlite3_vdbe_add_op3(v, 96, i_eph, n_off + j, k) };
                        __state = 365;
                    }
                    378 => { __state = 380; }
                    379 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, k) };
                        __state = 365;
                    }
                    380 => { __state = 381; }
                    381 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab, i_data_cur,
                                i, k)
                        };
                        __state = 382;
                    }
                    382 => { b_finish_seek = 0; __state = 365; }
                    383 => {
                        if tmask & 1 != 0 { __state = 388; } else { __state = 387; }
                    }
                    384 => { __state = 385; }
                    385 => { __state = 386; }
                    386 => {
                        unsafe {
                            sqlite3_compute_generated_columns(p_parse, reg_new, p_tab)
                        };
                        __state = 383;
                    }
                    387 => {
                        if (is_view == 0) as i32 != 0 {
                            __state = 409;
                        } else { __state = 408; }
                    }
                    388 => {
                        unsafe { sqlite3_table_affinity(v, p_tab, reg_new) };
                        __state = 389;
                    }
                    389 => {
                        unsafe {
                            sqlite3_code_row_trigger(p_parse, p_trigger, 130, p_changes,
                                1, p_tab, reg_old_rowid, on_error, label_continue)
                        };
                        __state = 390;
                    }
                    390 => {
                        if (is_view == 0) as i32 != 0 {
                            __state = 391;
                        } else { __state = 387; }
                    }
                    391 => {
                        if !(p_pk).is_null() {
                            __state = 393;
                        } else { __state = 394; }
                    }
                    392 => { { i = 0; k = reg_new }; __state = 398; }
                    393 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 28, i_data_cur, label_continue,
                                reg_key, n_key)
                        };
                        __state = 395;
                    }
                    394 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 31, i_data_cur, label_continue,
                                reg_old_rowid)
                        };
                        __state = 396;
                    }
                    395 => { __state = 392; }
                    396 => { __state = 392; }
                    397 => {
                        if unsafe { (*p_tab).tab_flags } & 96 as u32 != 0 {
                            __state = 405;
                        } else { __state = 387; }
                    }
                    398 => {
                        if i < unsafe { (*p_tab).n_col } as i32 {
                            __state = 399;
                        } else { __state = 397; }
                    }
                    399 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                        } as i32 & 96 != 0 {
                            __state = 401;
                        } else { __state = 402; }
                    }
                    400 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t }
                        };
                        __state = 398;
                    }
                    401 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                        } as i32 & 32 != 0 {
                            __state = 403;
                        } else { __state = 400; }
                    }
                    402 => {
                        if unsafe { *a_x_ref.offset(i as isize) } < 0 &&
                                i != unsafe { (*p_tab).i_p_key } as i32 {
                            __state = 404;
                        } else { __state = 400; }
                    }
                    403 => {
                        { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                        __state = 400;
                    }
                    404 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab, i_data_cur,
                                i, k)
                        };
                        __state = 400;
                    }
                    405 => { __state = 406; }
                    406 => { __state = 407; }
                    407 => {
                        unsafe {
                            sqlite3_compute_generated_columns(p_parse, reg_new, p_tab)
                        };
                        __state = 387;
                    }
                    408 => {
                        if reg_row_count != 0 {
                            __state = 430;
                        } else { __state = 429; }
                    }
                    409 => { { let _ = 0; }; __state = 410; }
                    410 => {
                        unsafe {
                            sqlite3_generate_constraint_checks(p_parse, p_tab,
                                a_reg_idx, i_data_cur, i_idx_cur, reg_new_rowid,
                                reg_old_rowid, chng_key, on_error as u8, label_continue,
                                &mut b_replace, a_x_ref, core::ptr::null_mut())
                        };
                        __state = 411;
                    }
                    411 => {
                        if b_replace != 0 || chng_key != 0 {
                            __state = 413;
                        } else { __state = 412; }
                    }
                    412 => {
                        if has_fk != 0 { __state = 418; } else { __state = 417; }
                    }
                    413 => {
                        if !(p_pk).is_null() {
                            __state = 415;
                        } else { __state = 416; }
                    }
                    414 => { __state = 412; }
                    415 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 28, i_data_cur, label_continue,
                                reg_key, n_key)
                        };
                        __state = 414;
                    }
                    416 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 31, i_data_cur, label_continue,
                                reg_old_rowid)
                        };
                        __state = 414;
                    }
                    417 => {
                        unsafe {
                            sqlite3_generate_row_index_delete(p_parse, p_tab,
                                i_data_cur, i_idx_cur, a_reg_idx, -1)
                        };
                        __state = 419;
                    }
                    418 => {
                        unsafe {
                            sqlite3_fk_check(p_parse, p_tab, reg_old_rowid, 0, a_x_ref,
                                chng_key as i32)
                        };
                        __state = 417;
                    }
                    419 => {
                        if b_finish_seek != 0 {
                            __state = 421;
                        } else { __state = 420; }
                    }
                    420 => { { let _ = 0; }; __state = 422; }
                    421 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 145, i_data_cur) };
                        __state = 420;
                    }
                    422 => {
                        if has_fk > 1 || chng_key != 0 {
                            __state = 424;
                        } else { __state = 423; }
                    }
                    423 => {
                        if has_fk != 0 { __state = 426; } else { __state = 425; }
                    }
                    424 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 132, i_data_cur, 0) };
                        __state = 423;
                    }
                    425 => {
                        unsafe {
                            sqlite3_complete_insertion(p_parse, p_tab, i_data_cur,
                                i_idx_cur, reg_new_rowid, a_reg_idx,
                                4 | if e_one_pass == 2 { 2 } else { 0 }, 0, 0)
                        };
                        __state = 427;
                    }
                    426 => {
                        unsafe {
                            sqlite3_fk_check(p_parse, p_tab, 0, reg_new_rowid, a_x_ref,
                                chng_key as i32)
                        };
                        __state = 425;
                    }
                    427 => {
                        if has_fk != 0 { __state = 428; } else { __state = 408; }
                    }
                    428 => {
                        unsafe {
                            sqlite3_fk_actions(p_parse, p_tab, p_changes, reg_old_rowid,
                                a_x_ref, chng_key as i32)
                        };
                        __state = 408;
                    }
                    429 => {
                        if !(p_trigger).is_null() {
                            __state = 432;
                        } else { __state = 431; }
                    }
                    430 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 88, reg_row_count, 1) };
                        __state = 429;
                    }
                    431 => {
                        if e_one_pass == 1 {
                            __state = 434;
                        } else { __state = 435; }
                    }
                    432 => {
                        unsafe {
                            sqlite3_code_row_trigger(p_parse, p_trigger, 130, p_changes,
                                2, p_tab, reg_old_rowid, on_error, label_continue)
                        };
                        __state = 431;
                    }
                    433 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, label_break) };
                        __state = 441;
                    }
                    434 => { __state = 433; }
                    435 => {
                        if e_one_pass == 2 {
                            __state = 436;
                        } else { __state = 437; }
                    }
                    436 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, label_continue) };
                        __state = 438;
                    }
                    437 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, label_continue) };
                        __state = 439;
                    }
                    438 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 433;
                    }
                    439 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 40, i_eph, addr_top) };
                        __state = 440;
                    }
                    440 => { __state = 433; }
                    441 => {
                        if unsafe { (*p_parse).nested } as i32 == 0 &&
                                    unsafe { (*p_parse).p_trigger_tab } == core::ptr::null_mut()
                                && p_upsert == core::ptr::null_mut() {
                            __state = 443;
                        } else { __state = 442; }
                    }
                    442 => {
                        if reg_row_count != 0 {
                            __state = 445;
                        } else { __state = 444; }
                    }
                    443 => {
                        unsafe { sqlite3_autoincrement_end(p_parse) };
                        __state = 442;
                    }
                    444 => { __state = 2; }
                    445 => {
                        unsafe {
                            sqlite3_code_change_count(v, reg_row_count,
                                c"rows updated".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 444;
                    }
                    446 => {
                        unsafe { sqlite3_db_free(db, a_x_ref as *mut ()) };
                        __state = 447;
                    }
                    447 => {
                        unsafe { sqlite3_src_list_delete(db, p_tab_list) };
                        __state = 448;
                    }
                    448 => {
                        unsafe { sqlite3_expr_list_delete(db, p_changes) };
                        __state = 449;
                    }
                    449 => {
                        unsafe { sqlite3_expr_delete(db, p_where) };
                        __state = 450;
                    }
                    450 => { return; }
                    _ => {}
                }
            }
        }
    }
}

///* The most recently coded instruction was an OP_Column to retrieve the
///* i-th column of table pTab. This routine sets the P4 parameter of the
///* OP_Column to the default value, if any.
///*
///* The default value of a column is specified by a DEFAULT clause in the
///* column definition. This was either supplied by the user when the table
///* was created, or added later to the table definition by an ALTER TABLE
///* command. If the latter, then the row-records in the table btree on disk
///* may not contain a value for the column and the default value, taken
///* from the P4 parameter of the OP_Column instruction, is returned instead.
///* If the former, then all row-records are guaranteed to include a value
///* for the column and the P4 value is not required.
///*
///* Column definitions created by an ALTER TABLE command may only have
///* literal default values specified: a number, null or a string. (If a more
///* complicated default expression value was provided, it is evaluated
///* when the ALTER TABLE is executed and one of the literal values written
///* into the sqlite_schema table.)
///*
///* Therefore, the P4 parameter is only required if the default value for
///* the column is a literal number, string or null. The sqlite3ValueFromExpr()
///* function is capable of transforming these types of expressions into
///* sqlite3_value objects.
///*
///* If column as REAL affinity and the table is an ordinary b-tree table
///* (not a virtual table) then the value might have been stored as an
///* integer.  In that case, add an OP_RealAffinity opcode to make sure
///* it has been converted into REAL.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_default(v: *mut Vdbe, p_tab: *mut Table,
    i: i32, i_reg: i32) -> () {
    let mut p_col: *mut Column = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    p_col = unsafe { unsafe { (*p_tab).a_col.offset(i as isize) } };
    if unsafe { (*p_col).i_dflt } != 0 {
        let mut p_value: *mut Sqlite3Value = core::ptr::null_mut();
        let enc: u8 = unsafe { (*unsafe { sqlite3_vdbe_db(v) }).enc };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            sqlite3_value_from_expr(unsafe { sqlite3_vdbe_db(v) },
                unsafe { sqlite3_column_expr(p_tab, p_col) } as *const Expr,
                enc, unsafe { (*p_col).affinity } as u8, &mut p_value)
        };
        if !(p_value).is_null() {
            unsafe { sqlite3_vdbe_append_p4(v, p_value as *mut (), -11) };
        }
    }
    if unsafe { (*p_col).affinity } as i32 == 69 &&
            !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0 {
        unsafe { sqlite3_vdbe_add_op1(v, 89, i_reg) };
    }
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
    fn sqlite3_close(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_close_v2(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_exec(_: *mut Sqlite3, sql: *const i8,
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
    fn sqlite3_db_config(_: *mut Sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_extended_result_codes(_: *mut Sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_last_insert_rowid(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_set_last_insert_rowid(_: *mut Sqlite3, _: Sqlite3Int64)
    -> ();
    fn sqlite3_changes(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_changes64(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_total_changes(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_total_changes64(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_interrupt(_: *mut Sqlite3)
    -> ();
    fn sqlite3_is_interrupted(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_complete(sql: *const i8)
    -> i32;
    fn sqlite3_complete16(sql: *const ())
    -> i32;
    fn sqlite3_incomplete(sql: *const i8)
    -> Sqlite3Int64;
    fn sqlite3_busy_handler(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), i32) -> i32>, _: *mut ())
    -> i32;
    fn sqlite3_busy_timeout(_: *mut Sqlite3, ms: i32)
    -> i32;
    fn sqlite3_setlk_timeout(_: *mut Sqlite3, ms: i32, flags: i32)
    -> i32;
    fn sqlite3_get_table(db: *mut Sqlite3, z_sql_1: *const i8,
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
    fn sqlite3_malloc64(_: Sqlite3Uint64)
    -> *mut ();
    fn sqlite3_realloc(_: *mut (), _: i32)
    -> *mut ();
    fn sqlite3_realloc64(_: *mut (), _: Sqlite3Uint64)
    -> *mut ();
    fn sqlite3_free(_: *mut ())
    -> ();
    fn sqlite3_msize(_: *mut ())
    -> Sqlite3Uint64;
    fn sqlite3_memory_used()
    -> Sqlite3Int64;
    fn sqlite3_memory_highwater(reset_flag_1: i32)
    -> Sqlite3Int64;
    fn sqlite3_randomness(n_1: i32, p_1: *mut ())
    -> ();
    fn sqlite3_set_authorizer(_: *mut Sqlite3,
    x_auth_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
            *const i8, *const i8) -> i32>, p_user_data_1: *mut ())
    -> i32;
    fn sqlite3_trace(_: *mut Sqlite3,
    x_trace_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_profile(_: *mut Sqlite3,
    x_profile_1: Option<unsafe extern "C" fn(*mut (), *const i8, u64) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_trace_v2(_: *mut Sqlite3, u_mask_1: u32,
    x_callback_1:
        Option<unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
    p_ctx_1: *mut ())
    -> i32;
    fn sqlite3_progress_handler(_: *mut Sqlite3, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> ();
    fn sqlite3_open(filename: *const i8, pp_db_1: *mut *mut Sqlite3)
    -> i32;
    fn sqlite3_open16(filename: *const (), pp_db_1: *mut *mut Sqlite3)
    -> i32;
    fn sqlite3_open_v2(filename: *const i8, pp_db_1: *mut *mut Sqlite3,
    flags: i32, z_vfs_1: *const i8)
    -> i32;
    fn sqlite3_uri_parameter(z: Sqlite3Filename, z_param_1: *const i8)
    -> *const i8;
    fn sqlite3_uri_boolean(z: Sqlite3Filename, z_param_1: *const i8,
    b_default_1: i32)
    -> i32;
    fn sqlite3_uri_int64(_: Sqlite3Filename, _: *const i8, _: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_uri_key(z: Sqlite3Filename, n_1: i32)
    -> *const i8;
    fn sqlite3_filename_database(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_filename_journal(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_filename_wal(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_database_file_object(_: *const i8)
    -> *mut Sqlite3File;
    fn sqlite3_create_filename(z_database_1: *const i8,
    z_journal_1: *const i8, z_wal_1: *const i8, n_param_1: i32,
    az_param_1: *mut *const i8)
    -> Sqlite3Filename;
    fn sqlite3_free_filename(_: Sqlite3Filename)
    -> ();
    fn sqlite3_errcode(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_extended_errcode(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_errmsg(_: *mut Sqlite3)
    -> *const i8;
    fn sqlite3_errmsg16(_: *mut Sqlite3)
    -> *const ();
    fn sqlite3_errstr(_: i32)
    -> *const i8;
    fn sqlite3_error_offset(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_set_errmsg(db: *mut Sqlite3, errcode: i32,
    z_err_msg_1: *const i8)
    -> i32;
    fn sqlite3_limit(_: *mut Sqlite3, id: i32, new_val_1: i32)
    -> i32;
    fn sqlite3_prepare(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v2(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v3(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    prep_flags_1: u32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare16(db: *mut Sqlite3, z_sql_1: *const (), n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v2(db: *mut Sqlite3, z_sql_1: *const (),
    n_byte_1: i32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v3(db: *mut Sqlite3, z_sql_1: *const (),
    n_byte_1: i32, prep_flags_1: u32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_sql(p_stmt_1: *mut Sqlite3Stmt)
    -> *const i8;
    fn sqlite3_expanded_sql(p_stmt_1: *mut Sqlite3Stmt)
    -> *mut i8;
    fn sqlite3_stmt_readonly(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_stmt_isexplain(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_stmt_explain(p_stmt_1: *mut Sqlite3Stmt, e_mode_1: i32)
    -> i32;
    fn sqlite3_stmt_busy(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_bind_blob(_: *mut Sqlite3Stmt, _: i32, _: *const (), n: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_blob64(_: *mut Sqlite3Stmt, _: i32, _: *const (),
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_double(_: *mut Sqlite3Stmt, _: i32, _: f64)
    -> i32;
    fn sqlite3_bind_int(_: *mut Sqlite3Stmt, _: i32, _: i32)
    -> i32;
    fn sqlite3_bind_int64(_: *mut Sqlite3Stmt, _: i32, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_bind_null(_: *mut Sqlite3Stmt, _: i32)
    -> i32;
    fn sqlite3_bind_text(_: *mut Sqlite3Stmt, _: i32, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text16(_: *mut Sqlite3Stmt, _: i32, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text64(_: *mut Sqlite3Stmt, _: i32, _: *const i8,
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> i32;
    fn sqlite3_bind_value(_: *mut Sqlite3Stmt, _: i32, _: *const Sqlite3Value)
    -> i32;
    fn sqlite3_bind_pointer(_: *mut Sqlite3Stmt, _: i32, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_zeroblob(_: *mut Sqlite3Stmt, _: i32, n: i32)
    -> i32;
    fn sqlite3_bind_zeroblob64(_: *mut Sqlite3Stmt, _: i32, _: Sqlite3Uint64)
    -> i32;
    fn sqlite3_bind_parameter_count(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_bind_parameter_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_bind_parameter_index(_: *mut Sqlite3Stmt, z_name_1: *const i8)
    -> i32;
    fn sqlite3_clear_bindings(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_count(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_name(_: *mut Sqlite3Stmt, n_1: i32)
    -> *const i8;
    fn sqlite3_column_name16(_: *mut Sqlite3Stmt, n_1: i32)
    -> *const ();
    fn sqlite3_column_database_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_database_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_table_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_table_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_origin_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_origin_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_decltype(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_decltype16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_step(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_data_count(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_blob(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_double(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> f64;
    fn sqlite3_column_int(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_int64(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> Sqlite3Int64;
    fn sqlite3_column_text(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const u8;
    fn sqlite3_column_text16(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_value(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *mut Sqlite3Value;
    fn sqlite3_column_bytes(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_bytes16(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_type(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_finalize(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_reset(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_create_function(db: *mut Sqlite3, z_function_name_1: *const i8,
    n_arg_1: i32, e_text_rep_1: i32, p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>)
    -> i32;
    fn sqlite3_create_function16(db: *mut Sqlite3,
    z_function_name_1: *const (), n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>)
    -> i32;
    fn sqlite3_create_function_v2(db: *mut Sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_window_function(db: *mut Sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_value_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_inverse_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_aggregate_count(_: *mut Sqlite3Context)
    -> i32;
    fn sqlite3_expired(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_transfer_bindings(_: *mut Sqlite3Stmt, _: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_global_recover()
    -> i32;
    fn sqlite3_thread_cleanup()
    -> ();
    fn sqlite3_memory_alarm(_:
        Option<unsafe extern "C" fn(*mut (), i64, i32) -> ()>, _: *mut (),
    _: Sqlite3Int64)
    -> i32;
    fn sqlite3_value_blob(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_double(_: *mut Sqlite3Value)
    -> f64;
    fn sqlite3_value_int(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_int64(_: *mut Sqlite3Value)
    -> Sqlite3Int64;
    fn sqlite3_value_pointer(_: *mut Sqlite3Value, _: *const i8)
    -> *mut ();
    fn sqlite3_value_text(_: *mut Sqlite3Value)
    -> *const u8;
    fn sqlite3_value_text16(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_text16le(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_text16be(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_bytes(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_bytes16(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_type(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_numeric_type(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_nochange(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_frombind(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_encoding(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_subtype(_: *mut Sqlite3Value)
    -> u32;
    fn sqlite3_value_dup(_: *const Sqlite3Value)
    -> *mut Sqlite3Value;
    fn sqlite3_value_free(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_aggregate_context(_: *mut Sqlite3Context, n_bytes_1: i32)
    -> *mut ();
    fn sqlite3_user_data(_: *mut Sqlite3Context)
    -> *mut ();
    fn sqlite3_context_db_handle(_: *mut Sqlite3Context)
    -> *mut Sqlite3;
    fn sqlite3_get_auxdata(_: *mut Sqlite3Context, n_1: i32)
    -> *mut ();
    fn sqlite3_set_auxdata(_: *mut Sqlite3Context, n_1: i32, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_get_clientdata(_: *mut Sqlite3, _: *const i8)
    -> *mut ();
    fn sqlite3_set_clientdata(_: *mut Sqlite3, _: *const i8, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_result_blob(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_blob64(_: *mut Sqlite3Context, _: *const (),
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_double(_: *mut Sqlite3Context, _: f64)
    -> ();
    fn sqlite3_result_error(_: *mut Sqlite3Context, _: *const i8, _: i32)
    -> ();
    fn sqlite3_result_error16(_: *mut Sqlite3Context, _: *const (), _: i32)
    -> ();
    fn sqlite3_result_error_toobig(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_error_nomem(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_error_code(_: *mut Sqlite3Context, _: i32)
    -> ();
    fn sqlite3_result_int(_: *mut Sqlite3Context, _: i32)
    -> ();
    fn sqlite3_result_int64(_: *mut Sqlite3Context, _: Sqlite3Int64)
    -> ();
    fn sqlite3_result_null(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_text(_: *mut Sqlite3Context, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text64(_: *mut Sqlite3Context, z: *const i8,
    n: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> ();
    fn sqlite3_result_text16(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16le(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16be(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_value(_: *mut Sqlite3Context, _: *mut Sqlite3Value)
    -> ();
    fn sqlite3_result_pointer(_: *mut Sqlite3Context, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_zeroblob(_: *mut Sqlite3Context, n: i32)
    -> ();
    fn sqlite3_result_zeroblob64(_: *mut Sqlite3Context, n: Sqlite3Uint64)
    -> i32;
    fn sqlite3_result_subtype(_: *mut Sqlite3Context, _: u32)
    -> ();
    fn sqlite3_create_collation(_: *mut Sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_create_collation_v2(_: *mut Sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>, x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_collation16(_: *mut Sqlite3, z_name_1: *const (),
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_collation_needed(_: *mut Sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const i8)
            -> ()>)
    -> i32;
    fn sqlite3_collation_needed16(_: *mut Sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const ())
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
    fn sqlite3_get_autocommit(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_db_handle(_: *mut Sqlite3Stmt)
    -> *mut Sqlite3;
    fn sqlite3_db_name(db: *mut Sqlite3, n_1: i32)
    -> *const i8;
    fn sqlite3_db_filename(db: *mut Sqlite3, z_db_name_1: *const i8)
    -> Sqlite3Filename;
    fn sqlite3_db_readonly(db: *mut Sqlite3, z_db_name_1: *const i8)
    -> i32;
    fn sqlite3_txn_state(_: *mut Sqlite3, z_schema_1: *const i8)
    -> i32;
    fn sqlite3_next_stmt(p_db_1: *mut Sqlite3, p_stmt_1: *mut Sqlite3Stmt)
    -> *mut Sqlite3Stmt;
    fn sqlite3_commit_hook(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_rollback_hook(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_autovacuum_pages(db: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
    _: *mut (), _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_update_hook(_: *mut Sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, i64)
            -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_enable_shared_cache(_: i32)
    -> i32;
    fn sqlite3_release_memory(_: i32)
    -> i32;
    fn sqlite3_db_release_memory(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_soft_heap_limit64(n_1: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_hard_heap_limit64(n_1: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_soft_heap_limit(n_1: i32)
    -> ();
    fn sqlite3_table_column_metadata(db: *mut Sqlite3, z_db_name_1: *const i8,
    z_table_name_1: *const i8, z_column_name_1: *const i8,
    pz_data_type_1: *mut *const i8, pz_coll_seq_1: *mut *const i8,
    p_not_null_1: *mut i32, p_primary_key_1: *mut i32, p_autoinc_1: *mut i32)
    -> i32;
    fn sqlite3_load_extension(db: *mut Sqlite3, z_file_1: *const i8,
    z_proc_1: *const i8, pz_err_msg_1: *mut *mut i8)
    -> i32;
    fn sqlite3_enable_load_extension(db: *mut Sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_cancel_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_reset_auto_extension()
    -> ();
    fn sqlite3_create_module(db: *mut Sqlite3, z_name_1: *const i8,
    p: *const Sqlite3Module, p_client_data_1: *mut ())
    -> i32;
    fn sqlite3_create_module_v2(db: *mut Sqlite3, z_name_1: *const i8,
    p: *const Sqlite3Module, p_client_data_1: *mut (),
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_drop_modules(db: *mut Sqlite3, az_keep_1: *mut *const i8)
    -> i32;
    fn sqlite3_declare_vtab(_: *mut Sqlite3, z_sql_1: *const i8)
    -> i32;
    fn sqlite3_overload_function(_: *mut Sqlite3, z_func_name_1: *const i8,
    n_arg_1: i32)
    -> i32;
    fn sqlite3_blob_open(_: *mut Sqlite3, z_db_1: *const i8,
    z_table_1: *const i8, z_column_1: *const i8, i_row_1: Sqlite3Int64,
    flags: i32, pp_blob_1: *mut *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_reopen(_: *mut Sqlite3Blob, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_blob_close(_: *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_bytes(_: *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_read(_: *mut Sqlite3Blob, z_1: *mut (), n_1: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_blob_write(_: *mut Sqlite3Blob, z: *const (), n: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_vfs_find(z_vfs_name_1: *const i8)
    -> *mut Sqlite3Vfs;
    fn sqlite3_vfs_register(_: *mut Sqlite3Vfs, make_dflt_1: i32)
    -> i32;
    fn sqlite3_vfs_unregister(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_mutex_alloc(_: i32)
    -> *mut Sqlite3Mutex;
    fn sqlite3_mutex_free(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_enter(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_try(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_mutex_leave(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_held(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_mutex_notheld(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_db_mutex(_: *mut Sqlite3)
    -> *mut Sqlite3Mutex;
    fn sqlite3_file_control(_: *mut Sqlite3, z_db_name_1: *const i8, op: i32,
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
    fn sqlite3_str_new(_: *mut Sqlite3)
    -> *mut Sqlite3Str;
    fn sqlite3_str_finish(_: *mut Sqlite3Str)
    -> *mut i8;
    fn sqlite3_str_free(_: *mut Sqlite3Str)
    -> ();
    fn sqlite3_result_str(_: *mut Sqlite3Context, _: *mut Sqlite3Str, _: i32)
    -> ();
    fn sqlite3_str_appendf(_: *mut Sqlite3Str, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_str_vappendf(_: *mut Sqlite3Str, z_format_1: *const i8,
    _: *mut i8)
    -> ();
    fn sqlite3_str_append(_: *mut Sqlite3Str, z_in_1: *const i8, n_1: i32)
    -> ();
    fn sqlite3_str_appendall(_: *mut Sqlite3Str, z_in_1: *const i8)
    -> ();
    fn sqlite3_str_appendchar(_: *mut Sqlite3Str, n_1: i32, c_1: i8)
    -> ();
    fn sqlite3_str_reset(_: *mut Sqlite3Str)
    -> ();
    fn sqlite3_str_truncate(_: *mut Sqlite3Str, n_1: i32)
    -> ();
    fn sqlite3_str_errcode(_: *mut Sqlite3Str)
    -> i32;
    fn sqlite3_str_length(_: *mut Sqlite3Str)
    -> i32;
    fn sqlite3_str_value(_: *mut Sqlite3Str)
    -> *mut i8;
    fn sqlite3_status(op: i32, p_current_1: *mut i32, p_highwater_1: *mut i32,
    reset_flag_1: i32)
    -> i32;
    fn sqlite3_status64(op: i32, p_current_1: *mut Sqlite3Int64,
    p_highwater_1: *mut Sqlite3Int64, reset_flag_1: i32)
    -> i32;
    fn sqlite3_db_status(_: *mut Sqlite3, op: i32, p_cur_1: *mut i32,
    p_hiwtr_1: *mut i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_db_status64(_: *mut Sqlite3, _: i32, _: *mut Sqlite3Int64,
    _: *mut Sqlite3Int64, _: i32)
    -> i32;
    fn sqlite3_stmt_status(_: *mut Sqlite3Stmt, op: i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_backup_init(p_dest_1: *mut Sqlite3, z_dest_name_1: *const i8,
    p_source_1: *mut Sqlite3, z_source_name_1: *const i8)
    -> *mut Sqlite3Backup;
    fn sqlite3_backup_step(p: *mut Sqlite3Backup, n_page_1: i32)
    -> i32;
    fn sqlite3_backup_finish(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_backup_remaining(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_backup_pagecount(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_unlock_notify(p_blocked_1: *mut Sqlite3,
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
    fn sqlite3_wal_hook(_: *mut Sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, *const i8, i32)
            -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_wal_autocheckpoint(db: *mut Sqlite3, n_1: i32)
    -> i32;
    fn sqlite3_wal_checkpoint(db: *mut Sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_wal_checkpoint_v2(db: *mut Sqlite3, z_db_1: *const i8,
    e_mode_1: i32, pn_log_1: *mut i32, pn_ckpt_1: *mut i32)
    -> i32;
    fn sqlite3_vtab_config(_: *mut Sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_vtab_on_conflict(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_nochange(_: *mut Sqlite3Context)
    -> i32;
    fn sqlite3_vtab_collation(_: *mut Sqlite3IndexInfo, _: i32)
    -> *const i8;
    fn sqlite3_vtab_distinct(_: *mut Sqlite3IndexInfo)
    -> i32;
    fn sqlite3_vtab_in(_: *mut Sqlite3IndexInfo, i_cons_1: i32,
    b_handle_1: i32)
    -> i32;
    fn sqlite3_vtab_in_first(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_vtab_in_next(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_vtab_rhs_value(_: *mut Sqlite3IndexInfo, _: i32,
    pp_val_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_stmt_scanstatus(p_stmt_1: *mut Sqlite3Stmt, idx: i32,
    i_scan_status_op_1: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_v2(p_stmt_1: *mut Sqlite3Stmt, idx: i32,
    i_scan_status_op_1: i32, flags: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_reset(_: *mut Sqlite3Stmt)
    -> ();
    fn sqlite3_db_cacheflush(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_system_errno(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_snapshot_get(db: *mut Sqlite3, z_schema_1: *const i8,
    pp_snapshot_1: *mut *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_open(db: *mut Sqlite3, z_schema_1: *const i8,
    p_snapshot_1: *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_free(_: *mut Sqlite3Snapshot)
    -> ();
    fn sqlite3_snapshot_cmp(p1: *mut Sqlite3Snapshot,
    p2: *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_recover(db: *mut Sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_serialize(db: *mut Sqlite3, z_schema_1: *const i8,
    pi_size_1: *mut Sqlite3Int64, m_flags_1: u32)
    -> *mut u8;
    fn sqlite3_deserialize(db: *mut Sqlite3, z_schema_1: *const i8,
    p_data_1: *mut u8, sz_db_1: Sqlite3Int64, sz_buf_1: Sqlite3Int64,
    m_flags_1: u32)
    -> i32;
    fn sqlite3_carray_bind_v2(p_stmt_1: *mut Sqlite3Stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>, p_del_1: *mut ())
    -> i32;
    fn sqlite3_carray_bind(p_stmt_1: *mut Sqlite3Stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_rtree_geometry_callback(db: *mut Sqlite3, z_geom_1: *const i8,
    x_geom_1:
        Option<unsafe extern "C" fn(*mut Sqlite3RtreeGeometry, i32, *mut f64,
            *mut i32) -> i32>, p_context_1: *mut ())
    -> i32;
    fn sqlite3_rtree_query_callback(db: *mut Sqlite3,
    z_query_func_1: *const i8,
    x_query_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3RtreeQueryInfo) -> i32>,
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
    fn sqlite3_os_close(_: *mut Sqlite3File)
    -> ();
    fn sqlite3_os_read(_: *mut Sqlite3File, _: *mut (), amt: i32, offset: i64)
    -> i32;
    fn sqlite3_os_write(_: *mut Sqlite3File, _: *const (), amt: i32,
    offset: i64)
    -> i32;
    fn sqlite3_os_truncate(_: *mut Sqlite3File, size: i64)
    -> i32;
    fn sqlite3_os_sync(_: *mut Sqlite3File, _: i32)
    -> i32;
    fn sqlite3_os_file_size(_: *mut Sqlite3File, p_size_1: *mut i64)
    -> i32;
    fn sqlite3_os_lock(_: *mut Sqlite3File, _: i32)
    -> i32;
    fn sqlite3_os_unlock(_: *mut Sqlite3File, _: i32)
    -> i32;
    fn sqlite3_os_check_reserved_lock(id: *mut Sqlite3File,
    p_res_out_1: *mut i32)
    -> i32;
    fn sqlite3_os_file_control(_: *mut Sqlite3File, _: i32, _: *mut ())
    -> i32;
    fn sqlite3_os_file_control_hint(_: *mut Sqlite3File, _: i32, _: *mut ())
    -> ();
    fn sqlite3_os_sector_size(id: *mut Sqlite3File)
    -> i32;
    fn sqlite3_os_device_characteristics(id: *mut Sqlite3File)
    -> i32;
    fn sqlite3_os_shm_map(_: *mut Sqlite3File, _: i32, _: i32, _: i32,
    _: *mut *mut ())
    -> i32;
    fn sqlite3_os_shm_lock(id: *mut Sqlite3File, _: i32, _: i32, _: i32)
    -> i32;
    fn sqlite3_os_shm_barrier(id: *mut Sqlite3File)
    -> ();
    fn sqlite3_os_shm_unmap(id: *mut Sqlite3File, _: i32)
    -> i32;
    fn sqlite3_os_fetch(id: *mut Sqlite3File, _: i64, _: i32, _: *mut *mut ())
    -> i32;
    fn sqlite3_os_unfetch(_: *mut Sqlite3File, _: i64, _: *mut ())
    -> i32;
    fn sqlite3_os_open(_: *mut Sqlite3Vfs, _: *const i8, _: *mut Sqlite3File,
    _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_os_delete(_: *mut Sqlite3Vfs, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_os_access(_: *mut Sqlite3Vfs, _: *const i8, _: i32,
    p_res_out_1: *mut i32)
    -> i32;
    fn sqlite3_os_full_pathname(_: *mut Sqlite3Vfs, _: *const i8, _: i32,
    _: *mut i8)
    -> i32;
    fn sqlite3_os_dl_open(_: *mut Sqlite3Vfs, _: *const i8)
    -> *mut ();
    fn sqlite3_os_dl_error(_: *mut Sqlite3Vfs, _: i32, _: *mut i8)
    -> ();
    fn sqlite3_os_dl_sym(_: *mut Sqlite3Vfs, _: *mut (), _: *const i8)
    -> unsafe extern "C" fn() -> ();
    fn sqlite3_os_dl_close(_: *mut Sqlite3Vfs, _: *mut ())
    -> ();
    fn sqlite3_os_randomness(_: *mut Sqlite3Vfs, _: i32, _: *mut i8)
    -> i32;
    fn sqlite3_os_sleep(_: *mut Sqlite3Vfs, _: i32)
    -> i32;
    fn sqlite3_os_get_last_error(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_os_current_time_int64(_: *mut Sqlite3Vfs, _: *mut Sqlite3Int64)
    -> i32;
    fn sqlite3_os_open_malloc(_: *mut Sqlite3Vfs, _: *const i8,
    _: *mut *mut Sqlite3File, _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_os_close_free(_: *mut Sqlite3File)
    -> ();
    fn sqlite3_pager_open(_: *mut Sqlite3Vfs, pp_pager_1: *mut *mut Pager,
    _: *const i8, _: i32, _: i32, _: i32,
    _: Option<unsafe extern "C" fn(*mut PgHdr) -> ()>)
    -> i32;
    fn sqlite3_pager_close(p_pager_1: *mut Pager, _: *mut Sqlite3)
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
    fn sqlite3_pager_set_mmap_limit(_: *mut Pager, _: Sqlite3Int64)
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
    -> *mut *mut Sqlite3Backup;
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
    fn sqlite3_pager_checkpoint(p_pager_1: *mut Pager, _: *mut Sqlite3,
    _: i32, _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_pager_wal_supported(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_pager_wal_callback(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_pager_open_wal(p_pager_1: *mut Pager, pis_open_1: *mut i32)
    -> i32;
    fn sqlite3_pager_close_wal(p_pager_1: *mut Pager, _: *mut Sqlite3)
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
    -> *mut Sqlite3Vfs;
    fn sqlite3_pager_file(_: *mut Pager)
    -> *mut Sqlite3File;
    fn sqlite3_pager_jrnl_file(_: *mut Pager)
    -> *mut Sqlite3File;
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
    fn sqlite3_sector_size(_: *mut Sqlite3File)
    -> i32;
    fn sqlite3_pager_truncate_image(_: *mut Pager, _: Pgno)
    -> ();
    fn sqlite3_pager_rekey(_: *mut DbPage, _: Pgno, _: u16)
    -> ();
    fn sqlite3_btree_open(p_vfs_1: *mut Sqlite3Vfs, z_filename_1: *const i8,
    db: *mut Sqlite3, pp_btree_1: *mut *mut Btree, flags: i32,
    vfs_flags_1: i32)
    -> i32;
    fn sqlite3_btree_close(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_set_cache_size(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_set_spill_size(_: *mut Btree, _: i32)
    -> i32;
    fn sqlite3_btree_set_mmap_limit(_: *mut Btree, _: Sqlite3Int64)
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
    -> Sqlite3Int64;
    fn sqlite3_btree_integrity_check(db: *mut Sqlite3, p: *mut Btree,
    a_root_1: *mut Pgno, a_cnt_1: *mut Sqlite3Value, n_root_1: i32,
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
    fn sqlite3_btree_count(_: *mut Sqlite3, _: *mut BtCursor, _: *mut i64)
    -> i32;
    fn sqlite3_btree_transfer_row(_: *mut BtCursor, _: *mut BtCursor, _: i64)
    -> i32;
    fn sqlite3_btree_clear_cache(_: *mut Btree)
    -> ();
    fn sqlite3_btree_enter(_: *mut Btree)
    -> ();
    fn sqlite3_btree_enter_all(_: *mut Sqlite3)
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
    fn sqlite3_btree_leave_all(_: *mut Sqlite3)
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
    -> *mut Sqlite3;
    fn sqlite3_vdbe_prepare_flags(_: *mut Vdbe)
    -> u8;
    fn sqlite3_vdbe_set_sql(_: *mut Vdbe, z: *const i8, n: i32, _: u8)
    -> ();
    fn sqlite3_vdbe_swap(_: *mut Vdbe, _: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_take_op_array(_: *mut Vdbe, _: *mut i32, _: *mut i32)
    -> *mut VdbeOp;
    fn sqlite3_vdbe_get_bound_value(_: *mut Vdbe, _: i32, _: u8)
    -> *mut Sqlite3Value;
    fn sqlite3_vdbe_set_varmask(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite3_vdbe_expand_sql(_: *mut Vdbe, _: *const i8)
    -> *mut i8;
    fn sqlite3_mem_compare(_: *const Mem, _: *const Mem, _: *const CollSeq)
    -> i32;
    fn sqlite3_blob_compare(_: *const Mem, _: *const Mem)
    -> i32;
    fn sqlite3_vdbe_func_name(_: *const Sqlite3Context)
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
    fn sqlite3_mem_set_array_int64(a_mem_1: *mut Sqlite3Value, i_idx_1: i32,
    val: i64)
    -> ();
    fn sqlite3_not_pure_func(_: *mut Sqlite3Context)
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
    -> *mut Sqlite3PcachePage;
    fn sqlite3_pcache_fetch_stress(_: *mut PCache, _: Pgno,
    _: *mut *mut Sqlite3PcachePage)
    -> i32;
    fn sqlite3_pcache_fetch_finish(_: *mut PCache, _: Pgno,
    p_page_1: *mut Sqlite3PcachePage)
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
    fn sqlite3_window_delete(_: *mut Sqlite3, _: *mut Window)
    -> ();
    fn sqlite3_window_unlink_from_select(_: *mut Window)
    -> ();
    fn sqlite3_window_list_delete(db: *mut Sqlite3, p: *mut Window)
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
    fn sqlite3_window_dup(db: *mut Sqlite3, p_owner_1: *mut Expr,
    p: *mut Window)
    -> *mut Window;
    fn sqlite3_window_list_dup(db: *mut Sqlite3, p: *mut Window)
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
    fn sqlite3_db_malloc_zero(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw_nn(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_str_dup(_: *mut Sqlite3, _: *const i8)
    -> *mut i8;
    fn sqlite3_db_str_n_dup(_: *mut Sqlite3, _: *const i8, _: u64)
    -> *mut i8;
    fn sqlite3_db_span_dup(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut i8;
    fn sqlite3Realloc(_: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_realloc_or_free(_: *mut Sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_realloc(_: *mut Sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_free(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_nn_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_malloc_size(_: *const ())
    -> i32;
    fn sqlite3_db_malloc_size(_: *mut Sqlite3, _: *const ())
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
    -> *const Sqlite3MutexMethods;
    fn sqlite3_noop_mutex()
    -> *const Sqlite3MutexMethods;
    fn sqlite3MutexAlloc(_: i32)
    -> *mut Sqlite3Mutex;
    fn sqlite3_mutex_init()
    -> i32;
    fn sqlite3_mutex_end()
    -> i32;
    fn sqlite3_memory_barrier()
    -> ();
    fn sqlite3_status_value(_: i32)
    -> Sqlite3Int64;
    fn sqlite3_status_up(_: i32, _: i32)
    -> ();
    fn sqlite3_status_down(_: i32, _: i32)
    -> ();
    fn sqlite3_status_highwater(_: i32, _: i32)
    -> ();
    fn sqlite3_lookaside_used(_: *mut Sqlite3, _: *mut i32)
    -> i32;
    fn sqlite3_pcache1_mutex()
    -> *mut Sqlite3Mutex;
    fn sqlite3_malloc_mutex()
    -> *mut Sqlite3Mutex;
    fn sqlite3_is_na_n(_: f64)
    -> i32;
    fn sqlite3_is_overflow(_: f64)
    -> i32;
    fn sqlite3_fp_decode(_: *mut FpDecode, _: f64, _: i32, _: i32)
    -> ();
    fn sqlite3_m_printf(_: *mut Sqlite3, _: *const i8, ...)
    -> *mut i8;
    fn sqlite3_vm_printf(_: *mut Sqlite3, _: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_set_string(_: *mut *mut i8, _: *mut Sqlite3, _: *const i8)
    -> ();
    fn sqlite3_progress_check(_: *mut Parse)
    -> ();
    fn sqlite3_error_msg(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_error_to_parser(_: *mut Sqlite3, _: i32)
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
    fn sqlite3_expr_alloc(_: *mut Sqlite3, _: i32, _: *const Token, _: i32)
    -> *mut Expr;
    fn sqlite3_expr(_: *mut Sqlite3, _: i32, _: *const i8)
    -> *mut Expr;
    fn sqlite3_expr_int32(_: *mut Sqlite3, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_attach_subtrees(_: *mut Sqlite3, _: *mut Expr,
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
    fn sqlite3_expr_delete(_: *mut Sqlite3, _: *mut Expr)
    -> ();
    fn sqlite3_expr_delete_generic(_: *mut Sqlite3, _: *mut ())
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
    fn sqlite3_expr_list_delete(_: *mut Sqlite3, _: *mut ExprList)
    -> ();
    fn sqlite3_expr_list_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_expr_list_flags(_: *const ExprList)
    -> u32;
    fn sqlite3_index_has_duplicate_root_page(_: *mut Index)
    -> i32;
    fn sqlite3_init(_: *mut Sqlite3, _: *mut *mut i8)
    -> i32;
    fn sqlite3_init_callback(_: *mut (), _: i32, _: *mut *mut i8,
    _: *mut *mut i8)
    -> i32;
    fn sqlite3_init_one(_: *mut Sqlite3, _: i32, _: *mut *mut i8, _: u32)
    -> i32;
    fn sqlite3_pragma(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token, _: i32)
    -> ();
    fn sqlite3_pragma_vtab_register(_: *mut Sqlite3, z_name_1: *const i8)
    -> *mut Module;
    fn sqlite3_reset_all_schemas_of_connection(_: *mut Sqlite3)
    -> ();
    fn sqlite3_reset_one_schema(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_collapse_database_array(_: *mut Sqlite3)
    -> ();
    fn sqlite3_commit_internal_changes(_: *mut Sqlite3)
    -> ();
    fn sqlite3_column_set_expr(_: *mut Parse, _: *mut Table, _: *mut Column,
    _: *mut Expr)
    -> ();
    fn sqlite3_column_expr(_: *mut Table, _: *mut Column)
    -> *mut Expr;
    fn sqlite3_column_set_coll(_: *mut Sqlite3, _: *mut Column,
    z_coll_1: *const i8)
    -> ();
    fn sqlite3_column_coll(_: *mut Column)
    -> *const i8;
    fn sqlite3_delete_column_names(_: *mut Sqlite3, _: *mut Table)
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
    _: *mut *mut Sqlite3Vfs, _: *mut *mut i8, _: *mut *mut i8)
    -> i32;
    fn sqlite3_db_name_to_btree(_: *mut Sqlite3, _: *const i8)
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
    fn sqlite3_row_set_init(_: *mut Sqlite3)
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
    fn sqlite3_delete_table(_: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_delete_table_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_free_index(_: *mut Sqlite3, _: *mut Index)
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
    fn sqlite3_array_allocate(_: *mut Sqlite3, _: *mut (), _: i32,
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
    fn sqlite3_subquery_delete(_: *mut Sqlite3, _: *mut Subquery)
    -> ();
    fn sqlite3_subquery_detach(_: *mut Sqlite3, _: *mut SrcItem)
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
    fn sqlite3_id_list_delete(_: *mut Sqlite3, _: *mut IdList)
    -> ();
    fn sqlite3_clear_on_or_using(_: *mut Sqlite3, _: *mut OnOrUsing)
    -> ();
    fn sqlite3_src_list_delete(_: *mut Sqlite3, _: *mut SrcList)
    -> ();
    fn sqlite3_allocate_index_object(_: *mut Sqlite3, _: i32, _: i32,
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
    fn sqlite3_select_delete(_: *mut Sqlite3, _: *mut Select)
    -> ();
    fn sqlite3_select_delete_generic(_: *mut Sqlite3, _: *mut ())
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
    fn sqlite3_auth_context_pop(_: *mut AuthContext)
    -> ();
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_triggers_exist(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut ExprList, p_mask_1: *mut i32)
    -> *mut Trigger;
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
    -> i32;
    fn sqlite3_column_index(p_tab_1: *mut Table, z_col_1: *const i8)
    -> i32;
    fn sqlite3_is_rowid(_: *const i8)
    -> i32;
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_expr_references_updated_column(_: *mut Expr, _: *mut i32,
    _: i32)
    -> i32;
    fn sqlite3_fk_required(_: *mut Parse, _: *mut Table, _: *mut i32, _: i32)
    -> i32;
    fn sqlite3_begin_write_operation(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_auth_context_push(_: *mut Parse, _: *mut AuthContext,
    _: *const i8)
    -> ();
    fn sqlite3_materialize_view(_: *mut Parse, _: *mut Table, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_get_v_table(_: *mut Sqlite3, _: *mut Table)
    -> *mut VTable;
    fn sqlite3_expr_dup(_: *mut Sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_src_list_dup(_: *mut Sqlite3, _: *const SrcList, _: i32)
    -> *mut SrcList;
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_where_begin(_: *mut Parse, _: *mut SrcList, _: *mut Expr,
    _: *mut ExprList, _: *mut ExprList, _: *mut Select, _: u16, _: i32)
    -> *mut WhereInfo;
    fn sqlite3_expr_code(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_where_ok_one_pass(_: *mut WhereInfo, _: *mut i32)
    -> i32;
    fn sqlite3_multi_write(_: *mut Parse)
    -> ();
    fn sqlite3_where_end(_: *mut WhereInfo)
    -> ();
    fn sqlite3_vtab_make_writable(_: *mut Parse, _: *mut Table)
    -> ();
    fn sqlite3_may_abort(_: *mut Parse)
    -> ();
    fn sqlite3_key_info_of_index(_: *mut Parse, _: *mut Index)
    -> *mut KeyInfo;
    fn sqlite3_expr_if_false(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_where_uses_deferred_seek(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_expr_code_get_column_of_table(_: *mut Vdbe, _: *mut Table,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_index_affinity_str(_: *mut Sqlite3, _: *mut Index)
    -> *const i8;
    fn sqlite3_open_table_and_indices(_: *mut Parse, _: *mut Table, _: i32,
    _: u8, _: i32, _: *mut u8, _: *mut i32, _: *mut i32)
    -> i32;
    fn sqlite3_fk_oldmask(_: *mut Parse, _: *mut Table)
    -> u32;
    fn sqlite3_trigger_colmask(_: *mut Parse, _: *mut Trigger,
    _: *mut ExprList, _: i32, _: i32, _: *mut Table, _: i32)
    -> u32;
    fn sqlite3_table_affinity(_: *mut Vdbe, _: *mut Table, _: i32)
    -> ();
    fn sqlite3_code_row_trigger(_: *mut Parse, _: *mut Trigger, _: i32,
    _: *mut ExprList, _: i32, _: *mut Table, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_generate_constraint_checks(_: *mut Parse, _: *mut Table,
    _: *mut i32, _: i32, _: i32, _: i32, _: i32, _: u8, _: u8, _: i32,
    _: *mut i32, _: *mut i32, _: *mut Upsert)
    -> ();
    fn sqlite3_fk_check(_: *mut Parse, _: *mut Table, _: i32, _: i32,
    _: *mut i32, _: i32)
    -> ();
    fn sqlite3_generate_row_index_delete(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: *mut i32, _: i32)
    -> ();
    fn sqlite3_complete_insertion(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: i32, _: *mut i32, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_fk_actions(_: *mut Parse, _: *mut Table, _: *mut ExprList,
    _: i32, _: *mut i32, _: i32)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
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
    fn sqlite3_expr_code_load_index_column(_: *mut Parse, _: *mut Index,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_code_get_column(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_code_move(_: *mut Parse, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_to_register(p_expr_1: *mut Expr, i_reg_1: i32)
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
    fn sqlite3_expr_if_false_dup(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_find_table(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut Table;
    fn sqlite3_locate_table(_: *mut Parse, flags: u32, _: *const i8,
    _: *const i8)
    -> *mut Table;
    fn sqlite3_preferred_table_name(_: *const i8)
    -> *const i8;
    fn sqlite3_locate_table_item(_: *mut Parse, flags: u32, _: *mut SrcItem)
    -> *mut Table;
    fn sqlite3_find_index(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut Index;
    fn sqlite3_unlink_and_delete_table(_: *mut Sqlite3, _: i32, _: *const i8)
    -> ();
    fn sqlite3_unlink_and_delete_index(_: *mut Sqlite3, _: i32, _: *const i8)
    -> ();
    fn sqlite3_vacuum(_: *mut Parse, _: *mut Token, _: *mut Expr)
    -> ();
    fn sqlite3_run_vacuum(_: *mut *mut i8, _: *mut Sqlite3, _: i32,
    _: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_name_from_token(_: *mut Sqlite3, _: *const Token)
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
    fn sqlite3_prng_save_state()
    -> ();
    fn sqlite3_prng_restore_state()
    -> ();
    fn sqlite3_rollback_all(_: *mut Sqlite3, _: i32)
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
    fn sqlite3_close_savepoints(_: *mut Sqlite3)
    -> ();
    fn sqlite3_leave_mutex_and_close_zombie(_: *mut Sqlite3)
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
    fn sqlite3_rowid_alias(p_tab_1: *mut Table)
    -> *const i8;
    fn sqlite3_generate_row_delete(_: *mut Parse, _: *mut Table,
    _: *mut Trigger, _: i32, _: i32, _: i32, _: i16, _: u8, _: u8, _: u8,
    _: i32)
    -> ();
    fn sqlite3_generate_index_key(_: *mut Parse, _: *mut Index, _: i32,
    _: i32, _: i32, _: *mut i32, _: *mut Index, _: i32)
    -> i32;
    fn sqlite3_resolve_part_idx_label(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_halt_constraint(_: *mut Parse, _: i32, _: i32, _: *mut i8,
    _: i8, _: u8)
    -> ();
    fn sqlite3_unique_constraint(_: *mut Parse, _: i32, _: *mut Index)
    -> ();
    fn sqlite3_rowid_constraint(_: *mut Parse, _: i32, _: *mut Table)
    -> ();
    fn sqlite3_expr_list_dup(_: *mut Sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_id_list_dup(_: *mut Sqlite3, _: *const IdList)
    -> *mut IdList;
    fn sqlite3_select_dup(_: *mut Sqlite3, _: *const Select, _: i32)
    -> *mut Select;
    fn sqlite3_function_search(_: i32, _: *const i8)
    -> *mut FuncDef;
    fn sqlite3_insert_builtin_funcs(_: *mut FuncDef, _: i32)
    -> ();
    fn sqlite3_find_function(_: *mut Sqlite3, _: *const i8, _: i32, _: u8,
    _: u8)
    -> *mut FuncDef;
    fn sqlite3_quote_value(_: *mut StrAccum, _: *mut Sqlite3Value, _: i32)
    -> ();
    fn sqlite3_append_one_utf8_character(_: *mut i8, _: u32)
    -> i32;
    fn sqlite3_register_builtin_functions()
    -> ();
    fn sqlite3_register_date_time_functions()
    -> ();
    fn sqlite3_register_json_functions()
    -> ();
    fn sqlite3_register_per_connection_builtin_functions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_json_vtab_register(_: *mut Sqlite3, _: *const i8)
    -> *mut Module;
    fn sqlite3_safety_check_ok(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_safety_check_sick_or_ok(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_change_cookie(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_with_dup(db: *mut Sqlite3, p: *mut With)
    -> *mut With;
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
    fn sqlite3_trigger_list(_: *mut Parse, _: *mut Table)
    -> *mut Trigger;
    fn sqlite3_code_row_trigger_direct(_: *mut Parse, _: *mut Trigger,
    _: *mut Table, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite_view_triggers(_: *mut Parse, _: *mut Table, _: *mut Expr,
    _: i32, _: *mut ExprList)
    -> ();
    fn sqlite3_delete_trigger_step(_: *mut Sqlite3, _: *mut TriggerStep)
    -> ();
    fn sqlite3_trigger_select_step(_: *mut Sqlite3, _: *mut Select,
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
    fn sqlite3_delete_trigger(_: *mut Sqlite3, _: *mut Trigger)
    -> ();
    fn sqlite3_unlink_and_delete_trigger(_: *mut Sqlite3, _: i32,
    _: *const i8)
    -> ();
    fn sqlite3_join_type(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut Token)
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
    fn sqlite3_auth_read_col(_: *mut Parse, _: *const i8, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_db_is_named(db: *mut Sqlite3, i_db_1: i32, z_name_1: *const i8)
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
    fn sqlite3_real_same_as_int(_: f64, _: Sqlite3Int64)
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
    fn sqlite3_v_list_add(_: *mut Sqlite3, _: *mut VList, _: *const i8,
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
    fn sqlite3_table_affinity_str(_: *mut Sqlite3, _: *const Table)
    -> *mut i8;
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
    fn sqlite3_error_with_msg(_: *mut Sqlite3, _: i32, _: *const i8, ...)
    -> ();
    fn sqlite3_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_error_clear(_: *mut Sqlite3)
    -> ();
    fn sqlite3_system_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_hex_to_blob(_: *mut Sqlite3, z: *const i8, n: i32)
    -> *mut ();
    fn sqlite3_hex_to_int(h: i32)
    -> u8;
    fn sqlite3_two_part_name(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut *mut Token)
    -> i32;
    fn sqlite3_memdb_init()
    -> i32;
    fn sqlite3_is_memdb(_: *const Sqlite3Vfs)
    -> i32;
    fn sqlite3_err_str(_: i32)
    -> *const i8;
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_find_coll_seq(_: *mut Sqlite3, enc: u8, _: *const i8, _: i32)
    -> *mut CollSeq;
    fn sqlite3_is_binary(_: *const CollSeq)
    -> i32;
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_set_text_encoding(db: *mut Sqlite3, _: u8)
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
    fn sqlite3_writable_schema(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_check_object_name(_: *mut Parse, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_vdbe_set_changes(_: *mut Sqlite3, _: i64)
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
    fn sqlite3ValueText(_: *mut Sqlite3Value, _: u8)
    -> *const ();
    fn sqlite3_value_is_of_class(_: *const Sqlite3Value,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3ValueBytes(_: *mut Sqlite3Value, _: u8)
    -> i32;
    fn sqlite3_value_set_str(_: *mut Sqlite3Value, _: i32, _: *const (),
    _: u8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_value_set_null(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_result_int_real(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_value_new(_: *mut Sqlite3)
    -> *mut Sqlite3Value;
    fn sqlite3_utf16to8(_: *mut Sqlite3, _: *const (), _: i32, _: u8)
    -> *mut i8;
    fn sqlite3_value_from_expr(_: *mut Sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_apply_affinity(_: *mut Sqlite3Value, _: u8, _: u8)
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
    static sqlite3_oom_str: Sqlite3Str;
    static mut sqlite3_pending_byte: i32;
    fn sqlite3_root_page_moved(_: *mut Sqlite3, _: i32, _: Pgno, _: Pgno)
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
    fn sqlite3_expire_prepared_statements(_: *mut Sqlite3, _: i32)
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
    fn sqlite3_match_e_name(_: *const ExprListItem, _: *const i8,
    _: *const i8, _: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_expr_col_used(_: *mut Expr)
    -> Bitmask;
    fn sqlite3_str_i_hash(_: *const i8)
    -> u8;
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
    fn sqlite3_find_db(_: *mut Sqlite3, _: *mut Token)
    -> i32;
    fn sqlite3_find_db_name(_: *mut Sqlite3, _: *const i8)
    -> i32;
    fn sqlite3_analysis_load(_: *mut Sqlite3, i_db_1: i32)
    -> i32;
    fn sqlite3_delete_index_samples(_: *mut Sqlite3, _: *mut Index)
    -> ();
    fn sqlite3_default_row_est(_: *mut Index)
    -> ();
    fn sqlite3_register_like_functions(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_is_like_function(_: *mut Sqlite3, _: *mut Expr, _: *mut i32,
    _: *mut i8)
    -> i32;
    fn sqlite3_schema_clear(_: *mut ())
    -> ();
    fn sqlite3_schema_get(_: *mut Sqlite3, _: *mut Btree)
    -> *mut Schema;
    fn sqlite3_key_info_alloc(_: *mut Sqlite3, _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_key_info_unref(_: *mut KeyInfo)
    -> ();
    fn sqlite3_key_info_ref(_: *mut KeyInfo)
    -> *mut KeyInfo;
    fn sqlite3_key_info_from_expr_list(_: *mut Parse, _: *mut ExprList,
    _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_select_op_name(_: i32)
    -> *const i8;
    fn sqlite3_has_explicit_nulls(_: *mut Parse, _: *mut ExprList)
    -> i32;
    fn sqlite3_create_func(_: *mut Sqlite3, _: *const i8, _: i32, _: i32,
    _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    _:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    _: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    _: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    _:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    p_destructor_1: *mut FuncDestructor)
    -> i32;
    fn sqlite3_noop_destructor(_: *mut ())
    -> ();
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_oom_clear(_: *mut Sqlite3)
    -> ();
    fn sqlite3_api_exit(db: *mut Sqlite3, _: i32)
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
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut Sqlite3, _: *mut i8,
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
    fn sqlite3_create_column_expr(_: *mut Sqlite3, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_record_error_byte_offset(_: *mut Sqlite3, _: *const i8)
    -> ();
    fn sqlite3_record_error_offset_of_expr(_: *mut Sqlite3, _: *const Expr)
    -> ();
    fn sqlite3_backup_restart(_: *mut Sqlite3Backup)
    -> ();
    fn sqlite3_backup_update(_: *mut Sqlite3Backup, _: Pgno, _: *const u8)
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
    fn sqlite3_auto_load_extensions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_close_extensions(_: *mut Sqlite3)
    -> ();
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_vtab_clear(db: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_disconnect(db: *mut Sqlite3, p: *mut Table)
    -> ();
    fn sqlite3_vtab_sync(db: *mut Sqlite3, _: *mut Vdbe)
    -> i32;
    fn sqlite3_vtab_rollback(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_commit(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_lock(_: *mut VTable)
    -> ();
    fn sqlite3_vtab_unlock(_: *mut VTable)
    -> ();
    fn sqlite3_vtab_module_unref(_: *mut Sqlite3, _: *mut Module)
    -> ();
    fn sqlite3_vtab_unlock_list(_: *mut Sqlite3)
    -> ();
    fn sqlite3_vtab_savepoint(_: *mut Sqlite3, _: i32, _: i32)
    -> i32;
    fn sqlite3_vtab_import_errmsg(_: *mut Vdbe, _: *mut Sqlite3Vtab)
    -> ();
    fn sqlite3_vtab_create_module(_: *mut Sqlite3, _: *const i8,
    _: *const Sqlite3Module, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut Module;
    fn sqlite3_read_only_shadow_tables(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_shadow_table_name(db: *mut Sqlite3, z_name_1: *const i8)
    -> i32;
    fn sqlite3_is_shadow_table_of(_: *mut Sqlite3, _: *mut Table,
    _: *const i8)
    -> i32;
    fn sqlite3_mark_all_shadow_tables_of(_: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_eponymous_table_init(_: *mut Parse, _: *mut Module)
    -> i32;
    fn sqlite3_vtab_eponymous_table_clear(_: *mut Sqlite3, _: *mut Module)
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
    fn sqlite3_vtab_call_create(_: *mut Sqlite3, _: i32, _: *const i8,
    _: *mut *mut i8)
    -> i32;
    fn sqlite3_vtab_call_connect(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_vtab_call_destroy(_: *mut Sqlite3, _: i32, _: *const i8)
    -> i32;
    fn sqlite3_vtab_begin(_: *mut Sqlite3, _: *mut VTable)
    -> i32;
    fn sqlite3_vtab_overload_function(_: *mut Sqlite3, _: *mut FuncDef,
    n_arg_1: i32, _: *mut Expr)
    -> *mut FuncDef;
    fn sqlite3_vtab_uses_all_schemas(_: *mut Parse)
    -> ();
    fn sqlite3_stmt_current_time(_: *mut Sqlite3Context)
    -> Sqlite3Int64;
    fn sqlite3_vdbe_parameter_index(_: *mut Vdbe, _: *const i8, _: i32)
    -> i32;
    fn sqlite3TransferBindings(_: *mut Sqlite3Stmt, _: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_parse_object_init(_: *mut Parse, _: *mut Sqlite3)
    -> ();
    fn sqlite3_parse_object_reset(_: *mut Parse)
    -> ();
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
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
    fn sqlite3_temp_in_memory(_: *const Sqlite3)
    -> i32;
    fn sqlite3_journal_modename(_: i32)
    -> *const i8;
    fn sqlite3_checkpoint(_: *mut Sqlite3, _: i32, _: i32, _: *mut i32,
    _: *mut i32)
    -> i32;
    fn sqlite3_wal_default_hook(_: *mut (), _: *mut Sqlite3, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_cte_new(_: *mut Parse, _: *mut Token, _: *mut ExprList,
    _: *mut Select, _: u8)
    -> *mut Cte;
    fn sqlite3_cte_delete(_: *mut Sqlite3, _: *mut Cte)
    -> ();
    fn sqlite3_with_add(_: *mut Parse, _: *mut With, _: *mut Cte)
    -> *mut With;
    fn sqlite3_with_delete(_: *mut Sqlite3, _: *mut With)
    -> ();
    fn sqlite3_with_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_with_push(_: *mut Parse, _: *mut With, _: u8)
    -> *mut With;
    fn sqlite3_upsert_new(_: *mut Sqlite3, _: *mut ExprList, _: *mut Expr,
    _: *mut ExprList, _: *mut Expr, _: *mut Upsert)
    -> *mut Upsert;
    fn sqlite3_upsert_delete(_: *mut Sqlite3, _: *mut Upsert)
    -> ();
    fn sqlite3_upsert_dup(_: *mut Sqlite3, _: *mut Upsert)
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
    fn sqlite3_fk_drop_table(_: *mut Parse, _: *mut SrcList, _: *mut Table)
    -> ();
    fn sqlite3_fk_references(_: *mut Table)
    -> *mut FKey;
    fn sqlite3_fk_clear_trigger_cache(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_fk_delete(_: *mut Sqlite3, _: *mut Table)
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
    fn sqlite3_journal_open(_: *mut Sqlite3Vfs, _: *const i8,
    _: *mut Sqlite3File, _: i32, _: i32)
    -> i32;
    fn sqlite3_journal_size(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_journal_is_in_memory(p: *mut Sqlite3File)
    -> i32;
    fn sqlite3_mem_journal_open(_: *mut Sqlite3File)
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
