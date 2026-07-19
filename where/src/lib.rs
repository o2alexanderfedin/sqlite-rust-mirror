#![allow(unused_imports, dead_code)]

mod btree_h;
mod hash_h;
mod pager_h;
mod pcache_h;
mod sqlite3_h;
mod sqlite_int_h;
mod vdbe_h;
mod where_int_h;
use crate::btree_h::{BtCursor, Btree, BtreePayload};
use crate::hash_h::Hash;
use crate::pager_h::{DbPage, Pager, Pgno};
use crate::pcache_h::{PCache, PgHdr};
use crate::sqlite3_h::{
    Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File, Sqlite3Filename,
    Sqlite3IndexConstraint, Sqlite3IndexConstraintUsage, Sqlite3IndexInfo,
    Sqlite3IndexOrderby, Sqlite3Int64, Sqlite3Module, Sqlite3Mutex,
    Sqlite3MutexMethods, Sqlite3PcachePage, Sqlite3RtreeGeometry,
    Sqlite3RtreeQueryInfo, Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Uint64,
    Sqlite3Value, Sqlite3Vfs, Sqlite3Vtab,
};
use crate::sqlite_int_h::{
    AuthContext, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte, DbFixer,
    Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode, FuncDef,
    FuncDefHash, FuncDestructor, IdList, Index, IndexedExpr, KeyInfo, LogEst,
    Module, NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema,
    Select, SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str,
    SrcItem, SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
    TriggerStep, UnpackedRecord, Upsert, VList, VTable, Walker, Window, With,
};
use crate::vdbe_h::{Mem, SubProgram, Vdbe, VdbeOp, VdbeOpList};
use crate::where_int_h::{
    InLoop, WhereClause, WhereInfo, WhereLevel, WhereLoop, WhereLoopBuilder,
    WhereLoopU0S1, WhereMaskSet, WhereMemBlock, WhereOrCost, WhereOrSet,
    WherePath, WhereRightJoin, WhereScan, WhereTerm,
};

type DarwinSizeT = u64;

type DarwinIntptrT = i64;

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

#[repr(C)]
#[derive(Copy, Clone)]
struct HiddenIndexInfo {
    p_wc: *mut WhereClause,
    p_parse: *mut Parse,
    e_distinct: i32,
    m_in: u32,
    m_handle_in: u32,
    a_rhs: [*mut Sqlite3Value; 0],
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

impl WhereLoopU0S1 {
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

///* Return term iTerm of the WhereClause passed as the first argument. Terms
///* are numbered from 0 upwards, starting with the terms in pWC->a[], then
///* those in pWC->pOuter->a[] (if any), and so on.
extern "C" fn term_from_where_clause(p_wc_1: *mut WhereClause,
    mut i_term_1: i32) -> *mut WhereTerm {
    let mut p: *const WhereClause = core::ptr::null();
    {
        p = p_wc_1;
        '__b0: loop {
            if !(!(p).is_null()) { break '__b0; }
            '__c0: loop {
                if i_term_1 < unsafe { (*p).n_term } {
                    return unsafe {
                            &mut *unsafe { (*p).a.offset(i_term_1 as isize) }
                        };
                }
                i_term_1 -= unsafe { (*p).n_term };
                break '__c0;
            }
            p = unsafe { (*p).p_outer };
        }
    }
    return core::ptr::null_mut();
}

///* CAPI3REF: Determine The Collation For a Virtual Table Constraint
///* METHOD: sqlite3_index_info
///*
///* This function may only be called from within a call to the [xBestIndex]
///* method of a [virtual table].  This function returns a pointer to a string
///* that is the name of the appropriate collation sequence to use for text
///* comparisons on the constraint identified by its arguments.
///*
///* The first argument must be the pointer to the [sqlite3_index_info] object
///* that is the first parameter to the xBestIndex() method. The second argument
///* must be an index into the aConstraint[] array belonging to the
///* sqlite3_index_info structure passed to xBestIndex.
///*
///* Important:
///* The first parameter must be the same pointer that is passed into the
///* xBestMethod() method.  The first parameter may not be a pointer to a
///* different [sqlite3_index_info] object, even an exact copy.
///*
///* The return value is computed as follows:
///*
///* <ol>
///* <li><p> If the constraint comes from a WHERE clause expression that contains
///*         a [COLLATE operator], then the name of the collation specified by
///*         that COLLATE operator is returned.
///* <li><p> If there is no COLLATE operator, but the column that is the subject
///*         of the constraint specifies an alternative collating sequence via
///*         a [COLLATE clause] on the column definition within the CREATE TABLE
///*         statement that was passed into [sqlite3_declare_vtab()], then the
///*         name of that alternative collating sequence is returned.
///* <li><p> Otherwise, "BINARY" is returned.
///* </ol>
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtab_collation(p_idx_info: *mut Sqlite3IndexInfo,
    i_cons: i32) -> *const i8 {
    unsafe {
        let p_hidden: *const HiddenIndexInfo =
            unsafe { &raw mut *p_idx_info.offset(1 as isize) } as
                    *mut HiddenIndexInfo as *const HiddenIndexInfo;
        let mut z_ret: *const i8 = core::ptr::null();
        if i_cons >= 0 && i_cons < unsafe { (*p_idx_info).n_constraint } {
            let mut p_c: *const CollSeq = core::ptr::null();
            let i_term: i32 =
                unsafe {
                    (*unsafe {
                                (*p_idx_info).a_constraint.offset(i_cons as isize)
                            }).i_term_offset
                };
            let p_x: *const Expr =
                unsafe {
                        (*term_from_where_clause(unsafe { (*p_hidden).p_wc },
                                        i_term)).p_expr
                    } as *const Expr;
            if !(unsafe { (*p_x).p_left }).is_null() {
                p_c =
                    unsafe {
                        sqlite3_expr_compare_coll_seq(unsafe {
                                (*p_hidden).p_parse
                            }, p_x as *const Expr)
                    };
            }
            z_ret =
                if !(p_c).is_null() {
                    (unsafe { (*p_c).z_name }) as *const i8
                } else { sqlite3_str_binary.as_ptr() as *const i8 };
        }
        return z_ret;
    }
}

///* CAPI3REF: Determine if a virtual table query is DISTINCT
///* METHOD: sqlite3_index_info
///*
///* This API may only be used from within an [xBestIndex|xBestIndex method]
///* of a [virtual table] implementation. The result of calling this
///* interface from outside of xBestIndex() is undefined and probably harmful.
///*
///* ^The sqlite3_vtab_distinct() interface returns an integer between 0 and
///* 3.  The integer returned by sqlite3_vtab_distinct()
///* gives the virtual table additional information about how the query
///* planner wants the output to be ordered. As long as the virtual table
///* can meet the ordering requirements of the query planner, it may set
///* the "orderByConsumed" flag.
///*
///* <ol><li value="0"><p>
///* ^If the sqlite3_vtab_distinct() interface returns 0, that means
///* that the query planner needs the virtual table to return all rows in the
///* sort order defined by the "nOrderBy" and "aOrderBy" fields of the
///* [sqlite3_index_info] object.  This is the default expectation.  If the
///* virtual table outputs all rows in sorted order, then it is always safe for
///* the xBestIndex method to set the "orderByConsumed" flag, regardless of
///* the return value from sqlite3_vtab_distinct().
///* <li value="1"><p>
///* ^(If the sqlite3_vtab_distinct() interface returns 1, that means
///* that the query planner does not need the rows to be returned in sorted order
///* as long as all rows with the same values in all columns identified by the
///* "aOrderBy" field are adjacent.)^  This mode is used when the query planner
///* is doing a GROUP BY.
///* <li value="2"><p>
///* ^(If the sqlite3_vtab_distinct() interface returns 2, that means
///* that the query planner does not need the rows returned in any particular
///* order, as long as rows with the same values in all columns identified
///* by "aOrderBy" are adjacent.)^  ^(Furthermore, when two or more rows
///* contain the same values for all columns identified by "colUsed", all but
///* one such row may optionally be omitted from the result.)^
///* The virtual table is not required to omit rows that are duplicates
///* over the "colUsed" columns, but if the virtual table can do that without
///* too much extra effort, it could potentially help the query to run faster.
///* This mode is used for a DISTINCT query.
///* <li value="3"><p>
///* ^(If the sqlite3_vtab_distinct() interface returns 3, that means the
///* virtual table must return rows in the order defined by "aOrderBy" as
///* if the sqlite3_vtab_distinct() interface had returned 0.  However if
///* two or more rows in the result have the same values for all columns
///* identified by "colUsed", then all but one such row may optionally be
///* omitted.)^  Like when the return value is 2, the virtual table
///* is not required to omit rows that are duplicates over the "colUsed"
///* columns, but if the virtual table can do that without
///* too much extra effort, it could potentially help the query to run faster.
///* This mode is used for queries
///* that have both DISTINCT and ORDER BY clauses.
///* </ol>
///*
///* <p>The following table summarizes the conditions under which the
///* virtual table is allowed to set the "orderByConsumed" flag based on
///* the value returned by sqlite3_vtab_distinct().  This table is a
///* restatement of the previous four paragraphs:
///*
///* <table border=1 cellspacing=0 cellpadding=10 width="90%">
///* <tr>
///* <td valign="top">sqlite3_vtab_distinct() return value
///* <td valign="top">Rows are returned in aOrderBy order
///* <td valign="top">Rows with the same value in all aOrderBy columns are
///*                  adjacent
///* <td valign="top">Duplicates over all colUsed columns may be omitted
///* <tr><td>0<td>yes<td>yes<td>no
///* <tr><td>1<td>no<td>yes<td>no
///* <tr><td>2<td>no<td>yes<td>yes
///* <tr><td>3<td>yes<td>yes<td>yes
///* </table>
///*
///* ^For the purposes of comparing virtual table output values to see if the
///* values are the same value for sorting purposes, two NULL values are
///* considered to be the same.  In other words, the comparison operator is "IS"
///* (or "IS NOT DISTINCT FROM") and not "==".
///*
///* If a virtual table implementation is unable to meet the requirements
///* specified above, then it must not set the "orderByConsumed" flag in the
///* [sqlite3_index_info] object or an incorrect answer may result.
///*
///* ^A virtual table implementation is always free to return rows in any order
///* it wants, as long as the "orderByConsumed" flag is not set.  ^When the
///* "orderByConsumed" flag is unset, the query planner will add extra
///* [bytecode] to ensure that the final results returned by the SQL query are
///* ordered correctly.  The use of the "orderByConsumed" flag and the
///* sqlite3_vtab_distinct() interface is merely an optimization.  ^Careful
///* use of the sqlite3_vtab_distinct() interface and the "orderByConsumed"
///* flag might help queries against a virtual table to run faster.  Being
///* overly aggressive and setting the "orderByConsumed" flag when it is not
///* valid to do so, on the other hand, might cause SQLite to return incorrect
///* results.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtab_distinct(p_idx_info: *mut Sqlite3IndexInfo)
    -> i32 {
    let p_hidden: *const HiddenIndexInfo =
        unsafe { &raw mut *p_idx_info.offset(1 as isize) } as
                *mut HiddenIndexInfo as *const HiddenIndexInfo;
    { let _ = 0; };
    return unsafe { (*p_hidden).e_distinct };
}

///* CAPI3REF: Identify and handle IN constraints in xBestIndex
///*
///* This interface may only be used from within an
///* [xBestIndex|xBestIndex() method] of a [virtual table] implementation.
///* The result of invoking this interface from any other context is
///* undefined and probably harmful.
///*
///* ^(A constraint on a virtual table of the form
///* "[IN operator|column IN (...)]" is
///* communicated to the xBestIndex method as a
///* [SQLITE_INDEX_CONSTRAINT_EQ] constraint.)^  If xBestIndex wants to use
///* this constraint, it must set the corresponding
///* aConstraintUsage[].argvIndex to a positive integer.  ^(Then, under
///* the usual mode of handling IN operators, SQLite generates [bytecode]
///* that invokes the [xFilter|xFilter() method] once for each value
///* on the right-hand side of the IN operator.)^  Thus the virtual table
///* only sees a single value from the right-hand side of the IN operator
///* at a time.
///*
///* In some cases, however, it would be advantageous for the virtual
///* table to see all values on the right-hand of the IN operator all at
///* once.  The sqlite3_vtab_in() interfaces facilitates this in two ways:
///*
///* <ol>
///* <li><p>
///*   ^A call to sqlite3_vtab_in(P,N,-1) will return true (non-zero)
///*   if and only if the [sqlite3_index_info|P->aConstraint][N] constraint
///*   is an [IN operator] that can be processed all at once.  ^In other words,
///*   sqlite3_vtab_in() with -1 in the third argument is a mechanism
///*   by which the virtual table can ask SQLite if all-at-once processing
///*   of the IN operator is even possible.
///*
///* <li><p>
///*   ^A call to sqlite3_vtab_in(P,N,F) with F==1 or F==0 indicates
///*   to SQLite that the virtual table does or does not want to process
///*   the IN operator all-at-once, respectively.  ^Thus when the third
///*   parameter (F) is non-negative, this interface is the mechanism by
///*   which the virtual table tells SQLite how it wants to process the
///*   IN operator.
///* </ol>
///*
///* ^The sqlite3_vtab_in(P,N,F) interface can be invoked multiple times
///* within the same xBestIndex method call.  ^For any given P,N pair,
///* the return value from sqlite3_vtab_in(P,N,F) will always be the same
///* within the same xBestIndex call.  ^If the interface returns true
///* (non-zero), that means that the constraint is an IN operator
///* that can be processed all-at-once.  ^If the constraint is not an IN
///* operator or cannot be processed all-at-once, then the interface returns
///* false.
///*
///* ^(All-at-once processing of the IN operator is selected if both of the
///* following conditions are met:
///*
///* <ol>
///* <li><p> The P->aConstraintUsage[N].argvIndex value is set to a positive
///* integer.  This is how the virtual table tells SQLite that it wants to
///* use the N-th constraint.
///*
///* <li><p> The last call to sqlite3_vtab_in(P,N,F) for which F was
///* non-negative had F>=1.
///* </ol>)^
///*
///* ^If either or both of the conditions above are false, then SQLite uses
///* the traditional one-at-a-time processing strategy for the IN constraint.
///* ^If both conditions are true, then the argvIndex-th parameter to the
///* xFilter method will be an [sqlite3_value] that appears to be NULL,
///* but which can be passed to [sqlite3_vtab_in_first()] and
///* [sqlite3_vtab_in_next()] to find all values on the right-hand side
///* of the IN constraint.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtab_in(p_idx_info: *mut Sqlite3IndexInfo,
    i_cons: i32, b_handle: i32) -> i32 {
    let p_hidden: *mut HiddenIndexInfo =
        unsafe { &raw mut *p_idx_info.offset(1 as isize) } as
            *mut HiddenIndexInfo;
    let m: u32 = if i_cons <= 31 { (1 as u32) << i_cons } else { 0 as u32 };
    if m & unsafe { (*p_hidden).m_in } != 0 {
        if b_handle == 0 {
            unsafe { (*p_hidden).m_handle_in &= !m };
        } else if b_handle > 0 { unsafe { (*p_hidden).m_handle_in |= m }; }
        return 1;
    }
    return 0;
}

///* CAPI3REF: Constraint values in xBestIndex()
///* METHOD: sqlite3_index_info
///*
///* This API may only be used from within the [xBestIndex|xBestIndex method]
///* of a [virtual table] implementation. The result of calling this interface
///* from outside of an xBestIndex method are undefined and probably harmful.
///*
///* ^When the sqlite3_vtab_rhs_value(P,J,V) interface is invoked from within
///* the [xBestIndex] method of a [virtual table] implementation, with P being
///* a copy of the [sqlite3_index_info] object pointer passed into xBestIndex and
///* J being a 0-based index into P->aConstraint[], then this routine
///* attempts to set *V to the value of the right-hand operand of
///* that constraint if the right-hand operand is known.  ^If the
///* right-hand operand is not known, then *V is set to a NULL pointer.
///* ^The sqlite3_vtab_rhs_value(P,J,V) interface returns SQLITE_OK if
///* and only if *V is set to a value.  ^The sqlite3_vtab_rhs_value(P,J,V)
///* inteface returns SQLITE_NOTFOUND if the right-hand side of the J-th
///* constraint is not available.  ^The sqlite3_vtab_rhs_value() interface
///* can return a result code other than SQLITE_OK or SQLITE_NOTFOUND if
///* something goes wrong.
///*
///* The sqlite3_vtab_rhs_value() interface is usually only successful if
///* the right-hand operand of a constraint is a literal value in the original
///* SQL statement.  If the right-hand operand is an expression or a reference
///* to some other column or a [host parameter], then sqlite3_vtab_rhs_value()
///* will probably return [SQLITE_NOTFOUND].
///*
///* ^(Some constraints, such as [SQLITE_INDEX_CONSTRAINT_ISNULL] and
///* [SQLITE_INDEX_CONSTRAINT_ISNOTNULL], have no right-hand operand.  For such
///* constraints, sqlite3_vtab_rhs_value() always returns SQLITE_NOTFOUND.)^
///*
///* ^The [sqlite3_value] object returned in *V is a protected sqlite3_value
///* and remains valid for the duration of the xBestIndex method call.
///* ^When xBestIndex returns, the sqlite3_value object returned by
///* sqlite3_vtab_rhs_value() is automatically deallocated.
///*
///* The "_rhs_" in the name of this routine is an abbreviation for
///* "Right-Hand Side".
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vtab_rhs_value(p_idx_info: *mut Sqlite3IndexInfo,
    i_cons: i32, pp_val: &mut *mut Sqlite3Value) -> i32 {
    let p_h: *mut HiddenIndexInfo =
        unsafe { &raw mut *p_idx_info.offset(1 as isize) } as
            *mut HiddenIndexInfo;
    let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if i_cons < 0 || i_cons >= unsafe { (*p_idx_info).n_constraint } {
        rc = unsafe { sqlite3_misuse_error(4604) };
    } else {
        if unsafe {
                    *(unsafe { (*p_h).a_rhs.as_ptr() } as
                                *mut *mut Sqlite3Value).offset(i_cons as isize)
                } == core::ptr::null_mut() {
            let p_term: *const WhereTerm =
                term_from_where_clause(unsafe { (*p_h).p_wc },
                        unsafe {
                            (*unsafe {
                                        (*p_idx_info).a_constraint.offset(i_cons as isize)
                                    }).i_term_offset
                        }) as *const WhereTerm;
            rc =
                unsafe {
                    sqlite3_value_from_expr(unsafe {
                            (*unsafe { (*p_h).p_parse }).db
                        },
                        unsafe { (*unsafe { (*p_term).p_expr }).p_right } as
                            *const Expr,
                        unsafe {
                            (*unsafe { (*unsafe { (*p_h).p_parse }).db }).enc
                        }, 65 as u8,
                        unsafe {
                            &mut *(unsafe { (*p_h).a_rhs.as_ptr() } as
                                            *mut *mut Sqlite3Value).offset(i_cons as isize)
                        })
                };
        }
        p_val =
            unsafe {
                *(unsafe { (*p_h).a_rhs.as_ptr() } as
                            *mut *mut Sqlite3Value).offset(i_cons as isize)
            };
    }
    *pp_val = p_val;
    if rc == 0 && p_val == core::ptr::null_mut() {

        /// IMP: R-19933-32160
        (rc = 12);
    }
    return rc;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CoveringIndexCheck {
    p_idx: *mut Index,
    i_tab_cur: i32,
    b_expr: u8,
    b_unidx: u8,
}

///* Clear the WhereLoop.u union.  Leave WhereLoop.pLTerm intact.
extern "C" fn where_loop_clear_union(db: *mut Sqlite3, p: &mut WhereLoop)
    -> () {
    unsafe {
        if (*p).ws_flags & (1024 | 16384) as u32 != 0 {
            if (*p).ws_flags & 1024 as u32 != 0 as u32 &&
                    (*p).u.vtab.need_free() != 0 {
                unsafe { sqlite3_free((*p).u.vtab.idx_str as *mut ()) };
                (*p).u.vtab.set_need_free(0 as u32 as u32);
                (*p).u.vtab.idx_str = core::ptr::null_mut();
            } else if (*p).ws_flags & 16384 as u32 != 0 as u32 &&
                    (*p).u.btree.p_index != core::ptr::null_mut() {
                unsafe {
                    sqlite3_db_free(db,
                        unsafe { (*(*p).u.btree.p_index).z_col_aff } as *mut ())
                };
                unsafe {
                    sqlite3_db_free_nn(db, (*p).u.btree.p_index as *mut ())
                };
                (*p).u.btree.p_index = core::ptr::null_mut();
            }
        }
    }
}

///* Deallocate internal memory used by a WhereLoop object.  Leave the
///* object in an initialized state, as if it had been newly allocated.
extern "C" fn where_loop_clear(db: *mut Sqlite3, p: *mut WhereLoop) -> () {
    if unsafe { (*p).a_l_term } !=
            unsafe { &raw mut (*p).a_l_term_space[0 as usize] } as
                *mut *mut WhereTerm {
        unsafe {
            sqlite3_db_free_nn(db, unsafe { (*p).a_l_term } as *mut ())
        };
        unsafe {
            (*p).a_l_term =
                unsafe { &raw mut (*p).a_l_term_space[0 as usize] } as
                    *mut *mut WhereTerm
        };
        unsafe {
            (*p).n_l_slot =
                (core::mem::size_of::<[*mut WhereTerm; 3]>() as u64 /
                            core::mem::size_of::<*mut WhereTerm>() as u64) as i32 as u16
        };
    }
    where_loop_clear_union(db, unsafe { &mut *p });
    unsafe { (*p).n_l_term = 0 as u16 };
    unsafe { (*p).ws_flags = 0 as u32 };
}

///* Delete a WhereLoop object
extern "C" fn where_loop_delete(db: *mut Sqlite3, p: *mut WhereLoop) -> () {
    { let _ = 0; };
    where_loop_clear(db, p);
    unsafe { sqlite3_db_nn_free_nn(db, p as *mut ()) };
}

///* Free a WhereInfo structure
extern "C" fn where_info_free(db: *mut Sqlite3, p_w_info_1: *mut WhereInfo)
    -> () {
    { let _ = 0; };
    { let _ = 0; };
    unsafe { sqlite3_where_clause_clear(unsafe { &mut (*p_w_info_1).s_wc }) };
    while !(unsafe { (*p_w_info_1).p_loops }).is_null() {
        let p: *mut WhereLoop = unsafe { (*p_w_info_1).p_loops };
        unsafe { (*p_w_info_1).p_loops = unsafe { (*p).p_next_loop } };
        where_loop_delete(db, p);
    }
    while !(unsafe { (*p_w_info_1).p_mem_to_free }).is_null() {
        let p_next: *mut WhereMemBlock =
            unsafe { (*unsafe { (*p_w_info_1).p_mem_to_free }).p_next };
        unsafe {
            sqlite3_db_nn_free_nn(db,
                unsafe { (*p_w_info_1).p_mem_to_free } as *mut ())
        };
        unsafe { (*p_w_info_1).p_mem_to_free = p_next };
    }
    unsafe { sqlite3_db_nn_free_nn(db, p_w_info_1 as *mut ()) };
}

///* Convert bulk memory into a valid WhereLoop that can be passed
///* to whereLoopClear harmlessly.
extern "C" fn where_loop_init(p: &mut WhereLoop) -> () {
    (*p).a_l_term =
        &raw mut (*p).a_l_term_space[0 as usize] as *mut *mut WhereTerm;
    (*p).n_l_term = 0 as u16;
    (*p).n_l_slot =
        (core::mem::size_of::<[*mut WhereTerm; 3]>() as u64 /
                    core::mem::size_of::<*mut WhereTerm>() as u64) as i32 as
            u16;
    (*p).ws_flags = 0 as u32;
}

///* Create a new mask for cursor iCursor.
///*
///* There is one cursor per table in the FROM clause.  The number of
///* tables in the FROM clause is limited by a test early in the
///* sqlite3WhereBegin() routine.  So we know that the pMaskSet->ix[]
///* array will never overflow.
extern "C" fn create_mask(p_mask_set_1: &mut WhereMaskSet, i_cursor_1: i32)
    -> () {
    unsafe {
        { let _ = 0; };
        (*p_mask_set_1).ix[{
                        let __p = &mut (*p_mask_set_1).n;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    } as usize] = i_cursor_1;
    }
}

///* Helper function for exprIsDeterministic().
extern "C" fn expr_node_is_deterministic(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    if unsafe { (*p_expr_1).op } as i32 == 172 &&
            (unsafe { (*p_expr_1).flags } & 1048576 as u32 != 0 as u32) as i32
                == 0 {
        unsafe { (*p_walker_1).e_code = 0 as u16 };
        return 2;
    }
    return 0;
}

///* Return true if the expression contains no non-deterministic SQL
///* functions. Do not consider non-deterministic SQL functions that are
///* part of sub-select statements.
extern "C" fn expr_is_deterministic(p: *mut Expr) -> i32 {
    let mut w: Walker = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut w as *mut (), 0,
            core::mem::size_of::<Walker>() as u64)
    };
    w.e_code = 1 as u16;
    w.x_expr_callback = Some(expr_node_is_deterministic);
    w.x_select_callback = Some(sqlite3_select_walk_fail);
    unsafe { sqlite3_walk_expr(&mut w, p) };
    return w.e_code as i32;
}

///* If the right-hand branch of the expression is a TK_COLUMN, then return
///* a pointer to the right-hand branch.  Otherwise, return NULL.
extern "C" fn where_right_subexpr_is_column(mut p: *mut Expr) -> *mut Expr {
    p =
        unsafe {
            sqlite3_expr_skip_collate_and_likely(unsafe { (*p).p_right })
        };
    if p != core::ptr::null_mut() && unsafe { (*p).op } as i32 == 168 &&
            !(unsafe { (*p).flags } & 32 as u32 != 0 as u32) as i32 != 0 {
        return p;
    }
    return core::ptr::null_mut();
}

///* Term pTerm is guaranteed to be a WO_IN term. It may be a component term
///* of a vector IN expression of the form "(x, y, ...) IN (SELECT ...)".
///* This function checks to see if the term is compatible with an index
///* column with affinity idxaff (one of the SQLITE_AFF_XYZ values). If so,
///* it returns a pointer to the name of the collation sequence (e.g. "BINARY"
///* or "NOCASE") used by the comparison in pTerm. If it is not compatible
///* with affinity idxaff, NULL is returned.
extern "C" fn index_in_affinity_ok(p_parse_1: *mut Parse,
    p_term_1: &WhereTerm, idxaff: u8) -> *const i8 {
    unsafe {
        unsafe {
            let mut p_x: *const Expr = (*p_term_1).p_expr as *const Expr;
            let mut inexpr: Expr = unsafe { core::mem::zeroed() };
            { let _ = 0; };
            if unsafe {
                        sqlite3_expr_is_vector(unsafe { (*p_x).p_left } as
                                *const Expr)
                    } != 0 {
                let i_field: i32 = (*p_term_1).u.x.i_field - 1;
                inexpr.flags = 0 as u32;
                inexpr.op = 54 as u8;
                inexpr.p_left =
                    unsafe {
                        (*(unsafe {
                                            (*unsafe {
                                                                (*unsafe { (*p_x).p_left }).x.p_list
                                                            }).a.as_ptr()
                                        } as *mut ExprListItem).offset(i_field as isize)).p_expr
                    };
                { let _ = 0; };
                inexpr.p_right =
                    unsafe {
                        (*(unsafe {
                                            (*unsafe {
                                                                (*unsafe { (*p_x).x.p_select }).p_e_list
                                                            }).a.as_ptr()
                                        } as *mut ExprListItem).offset(i_field as isize)).p_expr
                    };
                p_x = &mut inexpr;
            }
            if unsafe {
                        sqlite3_index_affinity_ok(p_x as *const Expr, idxaff as i8)
                    } != 0 {
                let p_ret: *const CollSeq =
                    unsafe {
                            sqlite3_expr_compare_coll_seq(p_parse_1, p_x as *const Expr)
                        } as *const CollSeq;
                return if !(p_ret).is_null() {
                        (unsafe { (*p_ret).z_name }) as *const i8
                    } else { sqlite3_str_binary.as_ptr() as *const i8 };
            }
            return core::ptr::null();
        }
    }
}

///* Advance to the next WhereTerm that matches according to the criteria
///* established when the pScan object was initialized by whereScanInit().
///* Return NULL if there are no more matching WhereTerms.
#[allow(unused_doc_comments)]
extern "C" fn where_scan_next(p_scan_1: &mut WhereScan) -> *mut WhereTerm {
    unsafe {
        unsafe {
            let mut i_cur: i32 = 0;
            /// The cursor on the LHS of the term
            let mut i_column: i16 = 0 as i16;
            /// The column on the LHS of the term.  -1 for IPK
            let mut p_x: *const Expr = core::ptr::null();
            /// An expression being tested
            let mut p_wc: *mut WhereClause = core::ptr::null_mut();
            /// Shorthand for pScan->pWC
            let mut p_term: *mut WhereTerm = core::ptr::null_mut();
            /// The term being tested
            let mut k: i32 = (*p_scan_1).k;

            /// Where to start scanning
            { let _ = 0; };
            p_wc = (*p_scan_1).p_wc;
            loop {
                i_column =
                    (*p_scan_1).ai_column[((*p_scan_1).i_equiv as i32 - 1) as
                            usize];
                i_cur =
                    (*p_scan_1).ai_cur[((*p_scan_1).i_equiv as i32 - 1) as
                            usize];
                { let _ = 0; };
                { let _ = 0; };
                '__b4: loop {
                    '__c4: loop {
                        {
                            p_term = unsafe { unsafe { (*p_wc).a.offset(k as isize) } };
                            '__b5: loop {
                                if !(k < unsafe { (*p_wc).n_term }) { break '__b5; }
                                '__c5: loop {
                                    { let _ = 0; };
                                    if unsafe { (*p_term).left_cursor } == i_cur &&
                                                    unsafe { (*p_term).u.x.left_column } == i_column as i32 &&
                                                (i_column as i32 != -2 ||
                                                    unsafe {
                                                            sqlite3_expr_compare_skip(unsafe {
                                                                    (*unsafe { (*p_term).p_expr }).p_left
                                                                }, (*p_scan_1).p_idx_expr, i_cur)
                                                        } == 0) &&
                                            ((*p_scan_1).i_equiv as i32 <= 1 ||
                                                !(unsafe { (*unsafe { (*p_term).p_expr }).flags } & 1 as u32
                                                                != 0 as u32) as i32 != 0) {
                                        if unsafe { (*p_term).e_operator } as i32 & 2048 != 0 &&
                                                    ((*p_scan_1).n_equiv as i32) <
                                                        (core::mem::size_of::<[i32; 11]>() as u64 /
                                                                core::mem::size_of::<i32>() as u64) as i32 &&
                                                {
                                                        p_x =
                                                            where_right_subexpr_is_column(unsafe { (*p_term).p_expr });
                                                        p_x
                                                    } != core::ptr::null_mut() {
                                            let mut j: i32 = 0;
                                            {
                                                j = 0;
                                                '__b6: loop {
                                                    if !(j < (*p_scan_1).n_equiv as i32) { break '__b6; }
                                                    '__c6: loop {
                                                        if (*p_scan_1).ai_cur[j as usize] ==
                                                                    unsafe { (*p_x).i_table } &&
                                                                (*p_scan_1).ai_column[j as usize] as i32 ==
                                                                    unsafe { (*p_x).i_column } as i32 {
                                                            break '__b6;
                                                        }
                                                        break '__c6;
                                                    }
                                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                            if j == (*p_scan_1).n_equiv as i32 {
                                                (*p_scan_1).ai_cur[j as usize] = unsafe { (*p_x).i_table };
                                                (*p_scan_1).ai_column[j as usize] =
                                                    unsafe { (*p_x).i_column };
                                                {
                                                    let __p = &mut (*p_scan_1).n_equiv;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                };
                                            }
                                        }
                                        if unsafe { (*p_term).e_operator } as u32 &
                                                    (*p_scan_1).op_mask != 0 as u32 {
                                            if !((*p_scan_1).z_coll_name).is_null() &&
                                                    unsafe { (*p_term).e_operator } as i32 & 256 == 0 {
                                                let mut z_coll_name: *const i8 = core::ptr::null();
                                                let p_parse: *mut Parse =
                                                    unsafe { (*unsafe { (*p_wc).p_w_info }).p_parse };
                                                p_x = unsafe { (*p_term).p_expr };
                                                if unsafe { (*p_term).e_operator } as i32 & 1 != 0 {
                                                    z_coll_name =
                                                        index_in_affinity_ok(p_parse, unsafe { &*p_term },
                                                            (*p_scan_1).idxaff as u8);
                                                    if (z_coll_name).is_null() as i32 != 0 { break '__c5; }
                                                } else {
                                                    let mut p_coll: *const CollSeq = core::ptr::null();
                                                    if (unsafe {
                                                                        sqlite3_index_affinity_ok(p_x as *const Expr,
                                                                            (*p_scan_1).idxaff)
                                                                    } == 0) as i32 != 0 {
                                                        break '__c5;
                                                    }
                                                    { let _ = 0; };
                                                    p_coll =
                                                        unsafe {
                                                            sqlite3_expr_compare_coll_seq(p_parse, p_x as *const Expr)
                                                        };
                                                    z_coll_name =
                                                        if !(p_coll).is_null() {
                                                            (unsafe { (*p_coll).z_name }) as *const i8
                                                        } else { sqlite3_str_binary.as_ptr() as *const i8 };
                                                }
                                                if unsafe {
                                                            sqlite3_str_i_cmp(z_coll_name, (*p_scan_1).z_coll_name)
                                                        } != 0 {
                                                    break '__c5;
                                                }
                                            }
                                            if unsafe { (*p_term).e_operator } as i32 & (2 | 128) != 0
                                                                &&
                                                                {
                                                                        p_x = unsafe { (*unsafe { (*p_term).p_expr }).p_right };
                                                                        (p_x != core::ptr::null_mut()) as i32
                                                                    } != 0 && unsafe { (*p_x).op } as i32 == 168 &&
                                                        unsafe { (*p_x).i_table } == (*p_scan_1).ai_cur[0 as usize]
                                                    &&
                                                    unsafe { (*p_x).i_column } as i32 ==
                                                        (*p_scan_1).ai_column[0 as usize] as i32 {
                                                break '__c5;
                                            }
                                            (*p_scan_1).p_wc = p_wc;
                                            (*p_scan_1).k = k + 1;
                                            return p_term;
                                        }
                                    }
                                    break '__c5;
                                }
                                {
                                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                    {
                                        let __p = &mut p_term;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                                };
                            }
                        }
                        p_wc = unsafe { (*p_wc).p_outer };
                        k = 0;
                        break '__c4;
                    }
                    if !(p_wc != core::ptr::null_mut()) { break '__b4; }
                }
                if (*p_scan_1).i_equiv as i32 >= (*p_scan_1).n_equiv as i32 {
                    break;
                }
                p_wc = (*p_scan_1).p_orig_wc;
                k = 0;
                {
                    let __p = &mut (*p_scan_1).i_equiv;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
            return core::ptr::null_mut();
        }
    }
}

///* This is whereScanInit() for the case of an index on an expression.
///* It is factored out into a separate tail-recursion subroutine so that
///* the normal whereScanInit() routine, which is a high-runner, does not
///* need to push registers onto the stack as part of its prologue.
extern "C" fn where_scan_init_index_expr(p_scan_1: *mut WhereScan)
    -> *mut WhereTerm {
    unsafe {
        (*p_scan_1).idxaff =
            unsafe {
                sqlite3_expr_affinity(unsafe { (*p_scan_1).p_idx_expr } as
                        *const Expr)
            }
    };
    return where_scan_next(unsafe { &mut *p_scan_1 });
}

///* Initialize a WHERE clause scanner object.  Return a pointer to the
///* first match.  Return NULL if there are no matches.
///*
///* The scanner will be searching the WHERE clause pWC.  It will look
///* for terms of the form "X <op> <expr>" where X is column iColumn of table
///* iCur.   Or if pIdx!=0 then X is column iColumn of index pIdx.  pIdx
///* must be one of the indexes of table iCur.
///*
///* The <op> must be one of the operators described by opMask.
///*
///* If the search is for X and the WHERE clause contains terms of the
///* form X=Y then this routine might also return terms of the form
///* "Y <op> <expr>".  The number of levels of transitivity is limited,
///* but is enough to handle most commonly occurring SQL statements.
///*
///* If X is not the INTEGER PRIMARY KEY then X must be compatible with
///* index pIdx.
extern "C" fn where_scan_init(p_scan_1: *mut WhereScan,
    p_wc_1: *mut WhereClause, i_cur_1: i32, mut i_column_1: i32,
    op_mask_1: u32, p_idx_1: *const Index) -> *mut WhereTerm {
    unsafe { (*p_scan_1).p_orig_wc = p_wc_1 };
    unsafe { (*p_scan_1).p_wc = p_wc_1 };
    unsafe { (*p_scan_1).p_idx_expr = core::ptr::null_mut() };
    unsafe { (*p_scan_1).idxaff = 0 as i8 };
    unsafe { (*p_scan_1).z_coll_name = core::ptr::null() };
    unsafe { (*p_scan_1).op_mask = op_mask_1 };
    unsafe { (*p_scan_1).k = 0 };
    unsafe { (*p_scan_1).ai_cur[0 as usize] = i_cur_1 };
    unsafe { (*p_scan_1).n_equiv = 1 as u8 };
    unsafe { (*p_scan_1).i_equiv = 1 as u8 };
    if !(p_idx_1).is_null() {
        let j: i32 = i_column_1;
        i_column_1 =
            unsafe { *unsafe { (*p_idx_1).ai_column.offset(j as isize) } } as
                i32;
        if i_column_1 ==
                unsafe { (*unsafe { (*p_idx_1).p_table }).i_p_key } as i32 {
            i_column_1 = -1;
        } else if i_column_1 >= 0 {
            unsafe {
                (*p_scan_1).idxaff =
                    unsafe {
                        (*unsafe {
                                    (*unsafe {
                                                        (*p_idx_1).p_table
                                                    }).a_col.offset(i_column_1 as isize)
                                }).affinity
                    }
            };
            unsafe {
                (*p_scan_1).z_coll_name =
                    unsafe { *unsafe { (*p_idx_1).az_coll.offset(j as isize) } }
            };
        } else if i_column_1 == -2 {
            unsafe {
                (*p_scan_1).p_idx_expr =
                    unsafe {
                        (*(unsafe { (*unsafe { (*p_idx_1).a_col_expr }).a.as_ptr() }
                                        as *mut ExprListItem).offset(j as isize)).p_expr
                    }
            };
            unsafe {
                (*p_scan_1).z_coll_name =
                    unsafe { *unsafe { (*p_idx_1).az_coll.offset(j as isize) } }
            };
            unsafe { (*p_scan_1).ai_column[0 as usize] = -2 as i16 };
            return where_scan_init_index_expr(p_scan_1);
        }
    } else if i_column_1 == -2 { return core::ptr::null_mut(); }
    unsafe { (*p_scan_1).ai_column[0 as usize] = i_column_1 as i16 };
    return where_scan_next(unsafe { &mut *p_scan_1 });
}

///* Search for a term in the WHERE clause that is of the form "X <op> <expr>"
///* where X is a reference to the iColumn of table iCur or of index pIdx
///* if pIdx!=0 and <op> is one of the WO_xx operator codes specified by
///* the op parameter.  Return a pointer to the term.  Return 0 if not found.
///*
///* If pIdx!=0 then it must be one of the indexes of table iCur. 
///* Search for terms matching the iColumn-th column of pIdx
///* rather than the iColumn-th column of table iCur.
///*
///* The term returned might by Y=<expr> if there is another constraint in
///* the WHERE clause that specifies that X=Y.  Any such constraints will be
///* identified by the WO_EQUIV bit in the pTerm->eOperator field.  The
///* aiCur[]/iaColumn[] arrays hold X and all its equivalents. There are 11
///* slots in aiCur[]/aiColumn[] so that means we can look for X plus up to 10
///* other equivalent values.  Hence a search for X will return <expr> if X=A1
///* and A1=A2 and A2=A3 and ... and A9=A10 and A10=<expr>.
///*
///* If there are multiple terms in the WHERE clause of the form "X <op> <expr>"
///* then try for the one with no dependencies on <expr> - in other words where
///* <expr> is a constant expression of some kind.  Only return entries of
///* the form "X <op> Y" where Y is a column in another table if no terms of
///* the form "X <op> <const-expr>" exist.   If no terms with a constant RHS
///* exist, try to return a term that does not use WO_EQUIV.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_find_term(p_wc_1: *mut WhereClause,
    i_cur_1: i32, i_column_1: i32, not_ready_1: Bitmask, mut op: u32,
    p_idx_1: *mut Index) -> *mut WhereTerm {
    let mut p_result: *mut WhereTerm = core::ptr::null_mut();
    let mut p: *mut WhereTerm = core::ptr::null_mut();
    let mut scan: WhereScan = unsafe { core::mem::zeroed() };
    p =
        where_scan_init(&mut scan, p_wc_1, i_cur_1, i_column_1, op,
            p_idx_1 as *const Index);
    op &= (2 | 128) as u32;
    while !(p).is_null() {
        if unsafe { (*p).prereq_right } & not_ready_1 == 0 as u64 {
            if unsafe { (*p).prereq_right } == 0 as u64 &&
                    unsafe { (*p).e_operator } as u32 & op != 0 as u32 {
                return p;
            }
            if p_result == core::ptr::null_mut() { p_result = p; }
        }
        p = where_scan_next(&mut scan);
    }
    return p_result;
}

///* This function searches pList for an entry that matches the iCol-th column
///* of index pIdx.
///*
///* If such an expression is found, its index in pList->a[] is returned. If
///* no expression is found, -1 is returned.
extern "C" fn find_index_col(p_parse_1: *mut Parse, p_list_1: &ExprList,
    i_base_1: i32, p_idx_1: &Index, i_col_1: i32) -> i32 {
    let mut i: i32 = 0;
    let z_coll: *const i8 =
        unsafe { *(*p_idx_1).az_coll.offset(i_col_1 as isize) };
    {
        i = 0;
        '__b8: loop {
            if !(i < (*p_list_1).n_expr) { break '__b8; }
            '__c8: loop {
                let p: *const Expr =
                    unsafe {
                            sqlite3_expr_skip_collate_and_likely(unsafe {
                                    (*((*p_list_1).a.as_ptr() as
                                                    *mut ExprListItem).offset(i as isize)).p_expr
                                })
                        } as *const Expr;
                if p != core::ptr::null_mut() &&
                                (unsafe { (*p).op } as i32 == 168 ||
                                    unsafe { (*p).op } as i32 == 170) &&
                            unsafe { (*p).i_column } as i32 ==
                                unsafe { *(*p_idx_1).ai_column.offset(i_col_1 as isize) } as
                                    i32 && unsafe { (*p).i_table } == i_base_1 {
                    let p_coll: *const CollSeq =
                        unsafe {
                                sqlite3_expr_nn_coll_seq(p_parse_1,
                                    unsafe {
                                            (*((*p_list_1).a.as_ptr() as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        } as *const Expr)
                            } as *const CollSeq;
                    if 0 ==
                            unsafe {
                                sqlite3_str_i_cmp(unsafe { (*p_coll).z_name } as *const i8,
                                    z_coll)
                            } {
                        return i;
                    }
                }
                break '__c8;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return -1;
}

///* Return TRUE if the iCol-th column of index pIdx is NOT NULL
extern "C" fn index_column_not_null(p_idx_1: &Index, i_col_1: i32) -> i32 {
    let mut j: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    j = unsafe { *(*p_idx_1).ai_column.offset(i_col_1 as isize) } as i32;
    if j >= 0 {
        return unsafe {
                    (*unsafe {
                                (*(*p_idx_1).p_table).a_col.offset(j as isize)
                            }).not_null()
                } as i32;
    } else if j == -1 { return 1; } else { { let _ = 0; }; return 0; }
}

///* Return true if the DISTINCT expression-list passed as the third argument
///* is redundant.
///*
///* A DISTINCT list is redundant if any subset of the columns in the
///* DISTINCT list are collectively unique and individually non-null.
#[allow(unused_doc_comments)]
extern "C" fn is_distinct_redundant(p_parse_1: *mut Parse,
    p_tab_list_1: &SrcList, p_wc_1: *mut WhereClause,
    p_distinct_1: *mut ExprList) -> i32 {
    let mut p_tab: *const Table = core::ptr::null();
    let mut p_idx: *mut Index = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut i_base: i32 = 0;
    if (*p_tab_list_1).n_src != 1 { return 0; }
    i_base =
        unsafe {
            (*((*p_tab_list_1).a.as_ptr() as
                            *mut SrcItem).offset(0 as isize)).i_cursor
        };
    p_tab =
        unsafe {
            (*((*p_tab_list_1).a.as_ptr() as
                            *mut SrcItem).offset(0 as isize)).p_s_tab
        };
    {
        i = 0;
        '__b9: loop {
            if !(i < unsafe { (*p_distinct_1).n_expr }) { break '__b9; }
            '__c9: loop {
                let p: *const Expr =
                    unsafe {
                            sqlite3_expr_skip_collate_and_likely(unsafe {
                                    (*(unsafe { (*p_distinct_1).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).p_expr
                                })
                        } as *const Expr;
                if p == core::ptr::null_mut() { break '__c9; }
                if unsafe { (*p).op } as i32 != 168 &&
                        unsafe { (*p).op } as i32 != 170 {
                    break '__c9;
                }
                if unsafe { (*p).i_table } == i_base &&
                        (unsafe { (*p).i_column } as i32) < 0 {
                    return 1;
                }
                break '__c9;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        p_idx = unsafe { (*p_tab).p_index };
        '__b10: loop {
            if !(!(p_idx).is_null()) { break '__b10; }
            '__c10: loop {
                if !(unsafe { (*p_idx).on_error } as i32 != 0) as i32 != 0 {
                    break '__c10;
                }
                if !(unsafe { (*p_idx).p_part_idx_where }).is_null() {
                    break '__c10;
                }
                {
                    i = 0;
                    '__b11: loop {
                        if !(i < unsafe { (*p_idx).n_key_col } as i32) {
                            break '__b11;
                        }
                        '__c11: loop {
                            if core::ptr::null_mut() ==
                                    sqlite3_where_find_term(p_wc_1, i_base, i, !(0 as Bitmask),
                                        2 as u32, p_idx) {
                                if find_index_col(p_parse_1, unsafe { &*p_distinct_1 },
                                            i_base, unsafe { &*p_idx }, i) < 0 {
                                    break '__b11;
                                }
                                if index_column_not_null(unsafe { &*p_idx }, i) == 0 {
                                    break '__b11;
                                }
                            }
                            break '__c11;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if i == unsafe { (*p_idx).n_key_col } as i32 {

                    /// This index implies that the DISTINCT qualifier is redundant.
                    return 1;
                }
                break '__c10;
            }
            p_idx = unsafe { (*p_idx).p_next };
        }
    }
    return 0;
}

///* Most queries use only a single table (they are not joins) and have
///* simple == constraints against indexed fields.  This routine attempts
///* to plan those simple cases using much less ceremony than the
///* general-purpose query planner, and thereby yield faster sqlite3_prepare()
///* times for the common case.
///*
///* Return non-zero on success, if this query can be handled by this
///* no-frills query planner.  Return zero if this query needs the
///* general-purpose query planner.
#[allow(unused_doc_comments)]
extern "C" fn where_short_cut(p_builder_1: &WhereLoopBuilder) -> i32 {
    unsafe {
        let mut p_w_info: *mut WhereInfo = core::ptr::null_mut();
        let mut p_item: *const SrcItem = core::ptr::null();
        let mut p_wc: *mut WhereClause = core::ptr::null_mut();
        let mut p_term: *mut WhereTerm = core::ptr::null_mut();
        let mut p_loop: *mut WhereLoop = core::ptr::null_mut();
        let mut i_cur: i32 = 0;
        let mut j: i32 = 0;
        let mut p_tab: *const Table = core::ptr::null();
        let mut p_idx: *mut Index = core::ptr::null_mut();
        let mut scan: WhereScan = unsafe { core::mem::zeroed() };
        p_w_info = (*p_builder_1).p_w_info;
        if unsafe { (*p_w_info).wctrl_flags } as i32 & 32 != 0 { return 0; }
        { let _ = 0; };
        p_item =
            unsafe { (*unsafe { (*p_w_info).p_tab_list }).a.as_ptr() } as
                *mut SrcItem;
        p_tab = unsafe { (*p_item).p_s_tab };
        if unsafe { (*p_tab).e_tab_type } as i32 == 1 { return 0; }
        if unsafe { (*p_item).fg.is_indexed_by() } != 0 ||
                unsafe { (*p_item).fg.not_indexed() } != 0 {
            return 0;
        }
        i_cur = unsafe { (*p_item).i_cursor };
        p_wc = unsafe { &mut (*p_w_info).s_wc };
        p_loop = (*p_builder_1).p_new;
        unsafe { (*p_loop).ws_flags = 0 as u32 };
        unsafe { (*p_loop).n_skip = 0 as u16 };
        p_term =
            where_scan_init(&mut scan, p_wc, i_cur, -1, (2 | 128) as u32,
                core::ptr::null());
        while !(p_term).is_null() && unsafe { (*p_term).prereq_right } != 0 {
            p_term = where_scan_next(&mut scan);
        }
        if !(p_term).is_null() {
            unsafe { (*p_loop).ws_flags = (1 | 256 | 4096) as u32 };
            unsafe {
                *unsafe { (*p_loop).a_l_term.offset(0 as isize) } = p_term
            };
            unsafe { (*p_loop).n_l_term = 1 as u16 };
            unsafe { (*p_loop).u.btree.n_eq = 1 as u16 };

            /// TUNING: Cost of a rowid lookup is 10
            unsafe { (*p_loop).r_run = 33 as LogEst };
        } else {
            {
                p_idx = unsafe { (*p_tab).p_index };
                '__b13: loop {
                    if !(!(p_idx).is_null()) { break '__b13; }
                    '__c13: loop {
                        let mut op_mask: i32 = 0;
                        { let _ = 0; };
                        if !(unsafe { (*p_idx).on_error } as i32 != 0) as i32 != 0
                                    ||
                                    unsafe { (*p_idx).p_part_idx_where } !=
                                        core::ptr::null_mut() ||
                                unsafe { (*p_idx).n_key_col } as i32 >
                                    (core::mem::size_of::<[*mut WhereTerm; 3]>() as u64 /
                                            core::mem::size_of::<*mut WhereTerm>() as u64) as i32 {
                            break '__c13;
                        }
                        op_mask =
                            if unsafe { (*p_idx).uniq_not_null() } != 0 {
                                2 | 128
                            } else { 2 };
                        {
                            j = 0;
                            '__b14: loop {
                                if !(j < unsafe { (*p_idx).n_key_col } as i32) {
                                    break '__b14;
                                }
                                '__c14: loop {
                                    p_term =
                                        where_scan_init(&mut scan, p_wc, i_cur, j, op_mask as u32,
                                            p_idx as *const Index);
                                    while !(p_term).is_null() &&
                                            unsafe { (*p_term).prereq_right } != 0 {
                                        p_term = where_scan_next(&mut scan);
                                    }
                                    if p_term == core::ptr::null_mut() { break '__b14; }
                                    unsafe {
                                        *unsafe { (*p_loop).a_l_term.offset(j as isize) } = p_term
                                    };
                                    break '__c14;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if j != unsafe { (*p_idx).n_key_col } as i32 {
                            break '__c13;
                        }
                        unsafe { (*p_loop).ws_flags = (1 | 4096 | 512) as u32 };
                        if unsafe { (*p_idx).is_covering() } != 0 ||
                                unsafe { (*p_item).col_used } &
                                        unsafe { (*p_idx).col_not_idxed } == 0 as u64 {
                            unsafe { (*p_loop).ws_flags |= 64 as u32 };
                        }
                        unsafe { (*p_loop).n_l_term = j as u16 };
                        unsafe { (*p_loop).u.btree.n_eq = j as u16 };
                        unsafe { (*p_loop).u.btree.p_index = p_idx };

                        /// TUNING: Cost of a unique index lookup is 15
                        unsafe { (*p_loop).r_run = 39 as LogEst };

                        /// 39==sqlite3LogEst(15)
                        break '__b13;
                        break '__c13;
                    }
                    p_idx = unsafe { (*p_idx).p_next };
                }
            }
        }
        if unsafe { (*p_loop).ws_flags } != 0 {
            unsafe { (*p_loop).n_out = 1 as LogEst };
            unsafe {
                (*(unsafe { (*p_w_info).a.as_ptr() } as
                                    *mut WhereLevel).offset(0 as isize)).p_w_loop = p_loop
            };
            { let _ = 0; };
            unsafe { (*p_loop).mask_self = 1 as Bitmask };
            unsafe {

                /// sqlite3WhereGetMask(&pWInfo->sMaskSet, iCur);
                ((*(unsafe { (*p_w_info).a.as_ptr() } as
                                    *mut WhereLevel).offset(0 as isize)).i_tab_cur = i_cur)
            };
            unsafe { (*p_w_info).n_row_out = 1 as LogEst };
            if !(unsafe { (*p_w_info).p_order_by }).is_null() {
                unsafe {
                    (*p_w_info).n_ob_sat =
                        unsafe { (*unsafe { (*p_w_info).p_order_by }).n_expr } as i8
                };
            }
            if unsafe { (*p_w_info).wctrl_flags } as i32 & 256 != 0 {
                unsafe { (*p_w_info).e_distinct = 1 as u8 };
            }
            if scan.i_equiv as i32 > 1 {
                unsafe { (*p_loop).ws_flags |= 2097152 as u32 };
            }
            return 1;
        }
        return 0;
    }
}

///* Private interfaces - callable only by other where.c routines.
///*
///* where.c:
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_get_mask(p_mask_set: &WhereMaskSet,
    i_cursor: i32) -> Bitmask {
    unsafe {
        let mut i: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if (*p_mask_set).ix[0 as usize] == i_cursor { return 1 as Bitmask; }
        {
            i = 1;
            '__b16: loop {
                if !(i < (*p_mask_set).n) { break '__b16; }
                '__c16: loop {
                    if (*p_mask_set).ix[i as usize] == i_cursor {
                        return (1 as Bitmask) << i;
                    }
                    break '__c16;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0 as Bitmask;
    }
}

///* We know that pSrc is an operand of an outer join.  Return true if
///* pTerm is a constraint that is compatible with that join.
///*
///* pTerm must be EP_OuterON if pSrc is the right operand of an
///* outer join.  pTerm can be either EP_OuterON or EP_InnerON if pSrc
///* is the left operand of a RIGHT join.
///*
///* See https://sqlite.org/forum/forumpost/206d99a16dd9212f
///* for an example of a WHERE clause constraints that may not be used on
///* the right table of a RIGHT JOIN because the constraint implies a
///* not-NULL condition on the left table of the RIGHT JOIN.
extern "C" fn constraint_compatible_with_outer_join(p_term_1: &WhereTerm,
    p_src_1: &SrcItem) -> i32 {
    unsafe {
        { let _ = 0; };
        if !(unsafe { (*(*p_term_1).p_expr).flags } & (1 | 2) as u32 !=
                                0 as u32) as i32 != 0 ||
                unsafe { (*(*p_term_1).p_expr).w.i_join } !=
                    (*p_src_1).i_cursor {
            return 0;
        }
        if (*p_src_1).fg.jointype as i32 & (8 | 16) != 0 &&
                unsafe { (*(*p_term_1).p_expr).flags } & 2 as u32 != 0 as u32
            {
            return 0;
        }
        return 1;
    }
}

///* Allocate and populate an sqlite3_index_info structure. It is the
///* responsibility of the caller to eventually release the structure
///* by passing the pointer returned by this function to freeIndexInfo().
#[allow(unused_doc_comments)]
extern "C" fn allocate_index_info(p_w_info_1: &WhereInfo,
    p_wc_1: *mut WhereClause, m_unusable_1: Bitmask, p_src_1: *const SrcItem,
    pm_no_omit_1: &mut u16) -> *mut Sqlite3IndexInfo {
    unsafe {
        unsafe {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut n_term: i32 = 0;
            let p_parse: *mut Parse = (*p_w_info_1).p_parse;
            let mut p_idx_cons: *mut Sqlite3IndexConstraint =
                core::ptr::null_mut();
            let mut p_idx_order_by: *mut Sqlite3IndexOrderby =
                core::ptr::null_mut();
            let mut p_usage: *mut Sqlite3IndexConstraintUsage =
                core::ptr::null_mut();
            let mut p_hidden: *mut HiddenIndexInfo = core::ptr::null_mut();
            let mut p_term: *mut WhereTerm = core::ptr::null_mut();
            let mut n_order_by: i32 = 0;
            let mut p_idx_info: *mut Sqlite3IndexInfo = core::ptr::null_mut();
            let mut m_no_omit: u16 = 0 as u16;
            let mut p_tab: *const Table = core::ptr::null();
            let mut e_distinct: i32 = 0;
            let p_order_by: *const ExprList =
                (*p_w_info_1).p_order_by as *const ExprList;
            let mut p: *const WhereClause = core::ptr::null();
            { let _ = 0; };
            p_tab = unsafe { (*p_src_1).p_s_tab } as *const Table;
            { let _ = 0; };
            { let _ = 0; };
            {
                { p = p_wc_1; n_term = 0 };
                '__b17: loop {
                    if !(!(p).is_null()) { break '__b17; }
                    '__c17: loop {
                        {
                            { i = 0; p_term = unsafe { (*p).a } };
                            '__b18: loop {
                                if !(i < unsafe { (*p).n_term }) { break '__b18; }
                                '__c18: loop {
                                    unsafe { (*p_term).wt_flags &= !64 as u16 };
                                    if unsafe { (*p_term).left_cursor } !=
                                            unsafe { (*p_src_1).i_cursor } {
                                        break '__c18;
                                    }
                                    if unsafe { (*p_term).prereq_right } & m_unusable_1 != 0 {
                                        break '__c18;
                                    }
                                    { let _ = 0; };
                                    if unsafe { (*p_term).e_operator } as i32 & !2048 == 0 {
                                        break '__c18;
                                    }
                                    if unsafe { (*p_term).wt_flags } as i32 & 128 != 0 {
                                        break '__c18;
                                    }
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    if unsafe { (*p_src_1).fg.jointype } as i32 & (8 | 64 | 16)
                                                != 0 &&
                                            (constraint_compatible_with_outer_join(unsafe { &*p_term },
                                                            unsafe { &*p_src_1 }) == 0) as i32 != 0 {
                                        break '__c18;
                                    }
                                    { let __p = &mut n_term; let __t = *__p; *__p += 1; __t };
                                    unsafe { (*p_term).wt_flags |= 64 as u16 };
                                    break '__c18;
                                }
                                {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    {
                                        let __p = &mut p_term;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                                };
                            }
                        }
                        break '__c17;
                    }
                    p = unsafe { (*p).p_outer };
                }
            }

            /// If the ORDER BY clause contains only columns in the current
            ///* virtual table then allocate space for the aOrderBy part of
            ///* the sqlite3_index_info structure.
            (n_order_by = 0);
            if !(p_order_by).is_null() {
                let n: i32 = unsafe { (*p_order_by).n_expr };
                {
                    i = 0;
                    '__b19: loop {
                        if !(i < n) { break '__b19; }
                        '__c19: loop {
                            let p_expr: *mut Expr =
                                unsafe {
                                    (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).p_expr
                                };
                            let mut p_e2: *const Expr = core::ptr::null();
                            if unsafe {
                                        sqlite3_expr_is_constant(core::ptr::null_mut(), p_expr)
                                    } != 0 {
                                break '__c19;
                            }
                            if unsafe {
                                                (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i as isize)).fg.sort_flags
                                            } as i32 & 2 != 0 {
                                break '__b19;
                            }
                            if unsafe { (*p_expr).op } as i32 == 168 &&
                                    unsafe { (*p_expr).i_table } ==
                                        unsafe { (*p_src_1).i_cursor } {
                                { let _ = 0; };
                                break '__c19;
                            }
                            if unsafe { (*p_expr).op } as i32 == 114 &&
                                        unsafe {
                                                    (*{ p_e2 = unsafe { (*p_expr).p_left }; p_e2 }).op
                                                } as i32 == 168 &&
                                    unsafe { (*p_e2).i_table } == unsafe { (*p_src_1).i_cursor }
                                {
                                let mut z_coll: *const i8 = core::ptr::null();

                                /// The collating sequence name
                                { let _ = 0; };
                                { let _ = 0; };
                                { let _ = 0; };
                                unsafe { (*p_expr).i_column = unsafe { (*p_e2).i_column } };
                                if (unsafe { (*p_e2).i_column } as i32) < 0 {
                                    break '__c19;
                                }

                                /// Collseq does not matter for rowid
                                (z_coll =
                                    unsafe {
                                        sqlite3_column_coll(unsafe {
                                                &mut *unsafe {
                                                            (*p_tab).a_col.offset(unsafe { (*p_e2).i_column } as isize)
                                                        }
                                            })
                                    });
                                if z_coll == core::ptr::null() {
                                    z_coll = sqlite3_str_binary.as_ptr() as *const i8;
                                }
                                if unsafe {
                                            sqlite3_stricmp(unsafe { (*p_expr).u.z_token } as *const i8,
                                                z_coll)
                                        } == 0 {
                                    break '__c19;
                                }
                            }

                            /// No matches cause a break out of the loop
                            break '__b19;
                            break '__c19;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if i == n {
                    let b_sort_by_group: i32 =
                        ((*p_w_info_1).wctrl_flags as i32 & 512 != 0) as i32;
                    n_order_by = n;
                    if (*p_w_info_1).wctrl_flags as i32 & 128 != 0 &&
                            (unsafe { (*p_src_1).fg.rowid_used() } == 0) as i32 != 0 {
                        e_distinct = 2 + b_sort_by_group;
                    } else if (*p_w_info_1).wctrl_flags as i32 & 64 != 0 {
                        e_distinct = 1 - b_sort_by_group;
                    } else if (*p_w_info_1).wctrl_flags as i32 & 256 != 0 {
                        e_distinct = 3;
                    }
                }
            }

            /// Allocate the sqlite3_index_info structure
            (p_idx_info =
                unsafe {
                        sqlite3_db_malloc_zero(unsafe { (*p_parse).db },
                            core::mem::size_of::<Sqlite3IndexInfo>() as u64 +
                                        (core::mem::size_of::<Sqlite3IndexConstraint>() as u64 +
                                                core::mem::size_of::<Sqlite3IndexConstraintUsage>() as u64)
                                            * n_term as u64 +
                                    core::mem::size_of::<Sqlite3IndexOrderby>() as u64 *
                                        n_order_by as u64 +
                                (core::mem::offset_of!(HiddenIndexInfo, a_rhs) as u64 +
                                    n_term as u64 *
                                        core::mem::size_of::<*mut Sqlite3Value>() as u64))
                    } as *mut Sqlite3IndexInfo);
            if p_idx_info == core::ptr::null_mut() {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"out of memory".as_ptr() as *mut i8 as *const i8)
                };
                return core::ptr::null_mut();
            }
            p_hidden =
                unsafe { &raw mut *p_idx_info.offset(1 as isize) } as
                    *mut HiddenIndexInfo;
            p_idx_cons =
                unsafe {
                        &raw mut *(unsafe { (*p_hidden).a_rhs.as_ptr() } as
                                        *mut *mut Sqlite3Value).offset(n_term as isize)
                    } as *mut Sqlite3IndexConstraint;
            p_idx_order_by =
                unsafe { &raw mut *p_idx_cons.offset(n_term as isize) } as
                    *mut Sqlite3IndexOrderby;
            p_usage =
                unsafe {
                        &raw mut *p_idx_order_by.offset(n_order_by as isize)
                    } as *mut Sqlite3IndexConstraintUsage;
            unsafe { (*p_idx_info).a_constraint = p_idx_cons };
            unsafe { (*p_idx_info).a_order_by = p_idx_order_by };
            unsafe { (*p_idx_info).a_constraint_usage = p_usage };
            unsafe {
                (*p_idx_info).col_used =
                    unsafe { (*p_src_1).col_used } as Sqlite3Int64 as
                        Sqlite3Uint64
            };
            if (unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32) as i32
                    == 0 {
                /// Ensure that all bits associated with PK columns are set. This is to
                ///* ensure they are available for cases like RIGHT joins or OR loops.
                let p_pk: *const Index =
                    unsafe { sqlite3_primary_key_index(p_tab as *mut Table) } as
                        *const Index;
                { let _ = 0; };
                {
                    i = 0;
                    '__b20: loop {
                        if !(i < unsafe { (*p_pk).n_key_col } as i32) {
                            break '__b20;
                        }
                        '__c20: loop {
                            let mut i_col: i32 =
                                unsafe { *unsafe { (*p_pk).ai_column.offset(i as isize) } }
                                    as i32;
                            { let _ = 0; };
                            if i_col >=
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                        1 {
                                i_col =
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                        1;
                            }
                            unsafe {
                                (*p_idx_info).col_used |= (1 as Bitmask) << i_col
                            };
                            break '__c20;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            unsafe { (*p_hidden).p_wc = p_wc_1 };
            unsafe { (*p_hidden).p_parse = p_parse };
            unsafe { (*p_hidden).e_distinct = e_distinct };
            unsafe { (*p_hidden).m_in = 0 as u32 };
            {
                { p = p_wc_1; i = { j = 0; j } };
                '__b21: loop {
                    if !(!(p).is_null()) { break '__b21; }
                    '__c21: loop {
                        let n_last: i32 = i + unsafe { (*p).n_term };
                        {
                            p_term = unsafe { (*p).a };
                            '__b22: loop {
                                if !(i < n_last) { break '__b22; }
                                '__c22: loop {
                                    let mut op: u16 = 0 as u16;
                                    if unsafe { (*p_term).wt_flags } as i32 & 64 == 0 {
                                        break '__c22;
                                    }
                                    unsafe {
                                        (*p_idx_cons.offset(j as isize)).i_column =
                                            unsafe { (*p_term).u.x.left_column }
                                    };
                                    unsafe {
                                        (*p_idx_cons.offset(j as isize)).i_term_offset = i
                                    };
                                    op =
                                        (unsafe { (*p_term).e_operator } as i32 & 16383) as u16;
                                    if op as i32 == 1 {
                                        if unsafe { (*p_term).wt_flags } as i32 & 32768 == 0 {
                                            unsafe {
                                                (*p_hidden).m_in |=
                                                    if j <= 31 { (1 as u32) << j } else { 0 as u32 }
                                            };
                                        }
                                        op = 2 as u16;
                                    }
                                    if op as i32 == 64 {
                                        unsafe {
                                            (*p_idx_cons.offset(j as isize)).op =
                                                unsafe { (*p_term).e_match_op }
                                        };
                                    } else if op as i32 & (256 | 128) != 0 {
                                        if op as i32 == 256 {
                                            unsafe { (*p_idx_cons.offset(j as isize)).op = 71 as u8 };
                                        } else {
                                            unsafe { (*p_idx_cons.offset(j as isize)).op = 72 as u8 };
                                        }
                                    } else {
                                        unsafe { (*p_idx_cons.offset(j as isize)).op = op as u8 };

                                        /// The direct assignment in the previous line is possible only because
                                        ///* the WO_ and SQLITE_INDEX_CONSTRAINT_ codes are identical.  The
                                        ///* following asserts verify this fact.
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        if op as i32 &
                                                        (2 << 57 - 54 | 2 << 56 - 54 | 2 << 55 - 54 | 2 << 58 - 54)
                                                    != 0 &&
                                                unsafe {
                                                        sqlite3_expr_is_vector(unsafe {
                                                                    (*unsafe { (*p_term).p_expr }).p_right
                                                                } as *const Expr)
                                                    } != 0 {
                                            if j < 16 { m_no_omit |= (1 << j) as u16; }
                                            if op as i32 == 2 << 57 - 54 {
                                                unsafe {
                                                    (*p_idx_cons.offset(j as isize)).op = (2 << 56 - 54) as u8
                                                };
                                            }
                                            if op as i32 == 2 << 55 - 54 {
                                                unsafe {
                                                    (*p_idx_cons.offset(j as isize)).op = (2 << 58 - 54) as u8
                                                };
                                            }
                                        }
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                    break '__c22;
                                }
                                {
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    {
                                        let __p = &mut p_term;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                                };
                            }
                        }
                        break '__c21;
                    }
                    p = unsafe { (*p).p_outer };
                }
            }
            { let _ = 0; };
            unsafe { (*p_idx_info).n_constraint = j };
            {
                i = { j = 0; j };
                '__b23: loop {
                    if !(i < n_order_by) { break '__b23; }
                    '__c23: loop {
                        let p_expr_1: *mut Expr =
                            unsafe {
                                (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)).p_expr
                            };
                        if unsafe {
                                    sqlite3_expr_is_constant(core::ptr::null_mut(), p_expr_1)
                                } != 0 {
                            break '__c23;
                        }
                        { let _ = 0; };
                        unsafe {
                            (*p_idx_order_by.offset(j as isize)).i_column =
                                unsafe { (*p_expr_1).i_column } as i32
                        };
                        unsafe {
                            (*p_idx_order_by.offset(j as isize)).desc =
                                (unsafe {
                                                (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i as isize)).fg.sort_flags
                                            } as i32 & 1) as u8
                        };
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        break '__c23;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*p_idx_info).n_order_by = j };
            *pm_no_omit_1 = m_no_omit;
            return p_idx_info;
        }
    }
}

/// Forward declaration of methods
extern "C" fn where_loop_resize(db: *mut Sqlite3, p: &mut WhereLoop,
    mut n: i32) -> i32 {
    let mut pa_new: *mut *mut WhereTerm = core::ptr::null_mut();
    if (*p).n_l_slot as i32 >= n { return 0; }
    n = n + 7 & !7;
    pa_new =
        unsafe {
                sqlite3_db_malloc_raw_nn(db,
                    core::mem::size_of::<*mut WhereTerm>() as u64 * n as u64)
            } as *mut *mut WhereTerm;
    if pa_new == core::ptr::null_mut() { return 7; }
    unsafe {
        memcpy(pa_new as *mut (), (*p).a_l_term as *const (),
            core::mem::size_of::<*mut WhereTerm>() as u64 *
                (*p).n_l_slot as u64)
    };
    if (*p).a_l_term !=
            &raw mut (*p).a_l_term_space[0 as usize] as *mut *mut WhereTerm {
        unsafe { sqlite3_db_free_nn(db, (*p).a_l_term as *mut ()) };
    }
    (*p).a_l_term = pa_new;
    (*p).n_l_slot = n as u16;
    return 0;
}

///* Free and zero the sqlite3_index_info.idxStr value if needed.
extern "C" fn free_idx_str(p_idx_info_1: &mut Sqlite3IndexInfo) -> () {
    if (*p_idx_info_1).need_to_free_idx_str != 0 {
        unsafe { sqlite3_free((*p_idx_info_1).idx_str as *mut ()) };
        (*p_idx_info_1).idx_str = core::ptr::null_mut();
        (*p_idx_info_1).need_to_free_idx_str = 0;
    }
}

///* Free an sqlite3_index_info structure allocated by allocateIndexInfo()
///* and possibly modified by xBestIndex methods.
#[allow(unused_doc_comments)]
extern "C" fn free_index_info(db: *mut Sqlite3,
    p_idx_info_1: *mut Sqlite3IndexInfo) -> () {
    let mut p_hidden: *mut HiddenIndexInfo = core::ptr::null_mut();
    let mut i: i32 = 0;
    { let _ = 0; };
    p_hidden =
        unsafe { &raw mut *p_idx_info_1.offset(1 as isize) } as
            *mut HiddenIndexInfo;
    { let _ = 0; };
    { let _ = 0; };
    {
        i = 0;
        '__b24: loop {
            if !(i < unsafe { (*p_idx_info_1).n_constraint }) {
                break '__b24;
            }
            '__c24: loop {
                unsafe {
                    sqlite3ValueFree(unsafe {
                            *(unsafe { (*p_hidden).a_rhs.as_ptr() } as
                                        *mut *mut Sqlite3Value).offset(i as isize)
                        })
                };

                /// IMP: R-14553-25174
                unsafe {
                    *(unsafe { (*p_hidden).a_rhs.as_ptr() } as
                                    *mut *mut Sqlite3Value).offset(i as isize) =
                        core::ptr::null_mut()
                };
                break '__c24;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    free_idx_str(unsafe { &mut *p_idx_info_1 });
    unsafe { sqlite3_db_free(db, p_idx_info_1 as *mut ()) };
}

///* Return true if pTerm is a virtual table LIMIT or OFFSET term.
extern "C" fn is_limit_term(p_term_1: &WhereTerm) -> i32 {
    { let _ = 0; };
    return ((*p_term_1).e_match_op as i32 >= 73 &&
                (*p_term_1).e_match_op as i32 <= 74) as i32;
}

///* Cause the prepared statement that is associated with a call to
///* xBestIndex to potentially use all schemas.  If the statement being
///* prepared is read-only, then just start read transactions on all
///* schemas.  But if this is a write operation, start writes on all
///* schemas.
///*
///* This is used by the (built-in) sqlite_dbpage virtual table.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtab_uses_all_schemas(p_parse: *mut Parse) -> () {
    let n_db: i32 = unsafe { (*unsafe { (*p_parse).db }).n_db };
    let mut i: i32 = 0;
    {
        i = 0;
        '__b25: loop {
            if !(i < n_db) { break '__b25; }
            '__c25: loop {
                unsafe { sqlite3_code_verify_schema(p_parse, i) };
                break '__c25;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if unsafe { (*p_parse).write_mask } != 0 as u32 {
        {
            i = 0;
            '__b26: loop {
                if !(i < n_db) { break '__b26; }
                '__c26: loop {
                    unsafe { sqlite3_begin_write_operation(p_parse, 0, i) };
                    break '__c26;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

///* The table object reference passed as the second argument to this function
///* must represent a virtual table. This function invokes the xBestIndex()
///* method of the virtual table with the sqlite3_index_info object that
///* comes in as the 3rd argument to this function.
///*
///* If an error occurs, pParse is populated with an error message and an
///* appropriate error code is returned.  A return of SQLITE_CONSTRAINT from
///* xBestIndex is not considered an error.  SQLITE_CONSTRAINT indicates that
///* the current configuration of "unusable" flags in sqlite3_index_info can
///* not result in a valid plan.
///*
///* Whether or not an error is returned, it is the responsibility of the
///* caller to eventually free p->idxStr if p->needToFreeIdxStr indicates
///* that this is required.
extern "C" fn vtab_best_index(p_parse_1: *mut Parse, p_tab_1: *mut Table,
    p: *mut Sqlite3IndexInfo) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_vtab: *mut Sqlite3Vtab = core::ptr::null_mut();
        { let _ = 0; };
        p_vtab =
            unsafe {
                (*unsafe {
                                sqlite3_get_v_table(unsafe { (*p_parse_1).db }, p_tab_1)
                            }).p_vtab
            };
        {
            let __p =
                unsafe { &mut (*unsafe { (*p_parse_1).db }).n_schema_lock };
            let __t = *__p;
            *__p += 1;
            __t
        };
        rc =
            unsafe {
                (unsafe {
                        (*unsafe { (*p_vtab).p_module }).x_best_index.unwrap()
                    })(p_vtab, p)
            };
        {
            let __p =
                unsafe { &mut (*unsafe { (*p_parse_1).db }).n_schema_lock };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if rc != 0 && rc != 19 {
            if rc == 7 {
                unsafe { sqlite3_oom_fault(unsafe { (*p_parse_1).db }) };
            } else if (unsafe { (*p_vtab).z_err_msg }).is_null() as i32 != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse_1,
                        c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { sqlite3_err_str(rc) })
                };
            } else {
                unsafe {
                    sqlite3_error_msg(p_parse_1,
                        c"%s".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p_vtab).z_err_msg })
                };
            }
        }
        if unsafe { (*unsafe { (*p_tab_1).u.vtab.p }).b_all_schemas } != 0 {
            sqlite3_vtab_uses_all_schemas(p_parse_1);
        }
        unsafe { sqlite3_free(unsafe { (*p_vtab).z_err_msg } as *mut ()) };
        unsafe { (*p_vtab).z_err_msg = core::ptr::null_mut() };
        return rc;
    }
}

///* Return true if the first nCons constraints in the pUsage array are
///* marked as in-use (have argvIndex>0). False otherwise.
extern "C" fn all_constraints_used(a_usage_1: &[Sqlite3IndexConstraintUsage])
    -> i32 {
    let mut ii: i32 = 0;
    {
        ii = 0;
        '__b27: loop {
            if !(ii < a_usage_1.len() as i32) { break '__b27; }
            '__c27: loop {
                if a_usage_1[ii as usize].argv_index <= 0 { return 0; }
                break '__c27;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}

///* Return TRUE if X is a proper subset of Y but is of equal or less cost.
///* In other words, return true if all constraints of X are also part of Y
///* and Y has additional constraints that might speed the search that X lacks
///* but the cost of running X is not more than the cost of running Y.
///*
///* In other words, return true if the cost relationship between X and Y
///* is inverted and needs to be adjusted.
///*
///* Case 1:
///*
///*   (1a)  X and Y use the same index.
///*   (1b)  X has fewer == terms than Y
///*   (1c)  Neither X nor Y use skip-scan
///*   (1d)  X does not have a a greater cost than Y
///*
///* Case 2:
///*
///*   (2a)  X has the same or lower cost, or returns the same or fewer rows,
///*         than Y.
///*   (2b)  X uses fewer WHERE clause terms than Y
///*   (2c)  Every WHERE clause term used by X is also used by Y
///*   (2d)  X skips at least as many columns as Y
///*   (2e)  If X is a covering index, than Y is too
#[allow(unused_doc_comments)]
extern "C" fn where_loop_cheaper_proper_subset(p_x_1: &WhereLoop,
    p_y_1: &WhereLoop) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        if (*p_x_1).r_run as i32 > (*p_y_1).r_run as i32 &&
                (*p_x_1).n_out as i32 > (*p_y_1).n_out as i32 {
            return 0;
        }

        /// (1d) and (2a)
        { let _ = 0; };
        { let _ = 0; };
        if ((*p_x_1).u.btree.n_eq as i32) < (*p_y_1).u.btree.n_eq as i32 &&
                        (*p_x_1).u.btree.p_index == (*p_y_1).u.btree.p_index &&
                    (*p_x_1).n_skip as i32 == 0 && (*p_y_1).n_skip as i32 == 0 {
            return 1;
        }
        if (*p_x_1).n_l_term as i32 - (*p_x_1).n_skip as i32 >=
                (*p_y_1).n_l_term as i32 - (*p_y_1).n_skip as i32 {
            return 0;
        }
        if (*p_y_1).n_skip as i32 > (*p_x_1).n_skip as i32 { return 0; }
        {
            i = (*p_x_1).n_l_term as i32 - 1;
            '__b28: loop {
                if !(i >= 0) { break '__b28; }
                '__c28: loop {
                    if unsafe { *(*p_x_1).a_l_term.offset(i as isize) } ==
                            core::ptr::null_mut() {
                        break '__c28;
                    }
                    {
                        j = (*p_y_1).n_l_term as i32 - 1;
                        '__b29: loop {
                            if !(j >= 0) { break '__b29; }
                            '__c29: loop {
                                if unsafe { *(*p_y_1).a_l_term.offset(j as isize) } ==
                                        unsafe { *(*p_x_1).a_l_term.offset(i as isize) } {
                                    break '__b29;
                                }
                                break '__c29;
                            }
                            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                        }
                    }
                    if j < 0 { return 0; }
                    break '__c28;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        if (*p_x_1).ws_flags & 64 as u32 != 0 as u32 &&
                (*p_y_1).ws_flags & 64 as u32 == 0 as u32 {
            return 0;
        }
        return 1;
    }
}

///* Try to adjust the cost and number of output rows of WhereLoop pTemplate
///* upwards or downwards so that:
///*
///*   (1) pTemplate costs less than any other WhereLoops that are a proper
///*       subset of pTemplate
///*
///*   (2) pTemplate costs more than any other WhereLoops for which pTemplate
///*       is a proper subset.
///*
///* To say "WhereLoop X is a proper subset of Y" means that X uses fewer
///* WHERE clause terms than Y and that every WHERE clause term used by X is
///* also used by Y.
extern "C" fn where_loop_adjust_cost(mut p: *const WhereLoop,
    p_template_1: *mut WhereLoop) -> () {
    if unsafe { (*p_template_1).ws_flags } & 512 as u32 == 0 as u32 {
        return;
    }
    {
        '__b30: loop {
            if !(!(p).is_null()) { break '__b30; }
            '__c30: loop {
                if unsafe { (*p).i_tab } as i32 !=
                        unsafe { (*p_template_1).i_tab } as i32 {
                    break '__c30;
                }
                if unsafe { (*p).ws_flags } & 512 as u32 == 0 as u32 {
                    break '__c30;
                }
                if where_loop_cheaper_proper_subset(unsafe { &*p },
                            unsafe { &*p_template_1 }) != 0 {
                    unsafe {
                        (*p_template_1).r_run =
                            if (unsafe { (*p).r_run } as i32) <
                                        unsafe { (*p_template_1).r_run } as i32 {
                                    (unsafe { (*p).r_run }) as i32
                                } else { (unsafe { (*p_template_1).r_run }) as i32 } as
                                LogEst
                    };
                    unsafe {
                        (*p_template_1).n_out =
                            if unsafe { (*p).n_out } as i32 - 1 <
                                        unsafe { (*p_template_1).n_out } as i32 {
                                    (unsafe { (*p).n_out }) as i32 - 1
                                } else { (unsafe { (*p_template_1).n_out }) as i32 } as
                                LogEst
                    };
                } else if where_loop_cheaper_proper_subset(unsafe {
                                &*p_template_1
                            }, unsafe { &*p }) != 0 {
                    unsafe {
                        (*p_template_1).r_run =
                            if unsafe { (*p).r_run } as i32 >
                                        unsafe { (*p_template_1).r_run } as i32 {
                                    (unsafe { (*p).r_run }) as i32
                                } else { (unsafe { (*p_template_1).r_run }) as i32 } as
                                LogEst
                    };
                    unsafe {
                        (*p_template_1).n_out =
                            if unsafe { (*p).n_out } as i32 + 1 >
                                        unsafe { (*p_template_1).n_out } as i32 {
                                    (unsafe { (*p).n_out }) as i32 + 1
                                } else { (unsafe { (*p_template_1).n_out }) as i32 } as
                                LogEst
                    };
                }
                break '__c30;
            }
            p = unsafe { (*p).p_next_loop } as *const WhereLoop;
        }
    }
}

///* Try to insert a new prerequisite/cost entry into the WhereOrSet pSet.
///*
///* The new entry might overwrite an existing entry, or it might be
///* appended, or it might be discarded.  Do whatever is the right thing
///* so that pSet keeps the N_OR_COST best entries seen so far.
extern "C" fn where_or_insert(p_set_1: &mut WhereOrSet, prereq: Bitmask,
    r_run_1: LogEst, n_out_1: LogEst) -> i32 {
    unsafe {
        let mut i: u16 = 0 as u16;
        let mut p: *mut WhereOrCost = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s32:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => { unsafe { (*p).prereq = prereq }; __state = 23; }
                    3 => { __state = 4; }
                    4 => {
                        {
                            i = (*p_set_1).n;
                            p = &raw mut (*p_set_1).a[0 as usize] as *mut WhereOrCost
                        };
                        __state = 6;
                    }
                    5 => {
                        if ((*p_set_1).n as i32) < 3 {
                            __state = 13;
                        } else { __state = 14; }
                    }
                    6 => {
                        if i as i32 > 0 { __state = 7; } else { __state = 5; }
                    }
                    7 => {
                        if r_run_1 as i32 <= unsafe { (*p).r_run } as i32 &&
                                prereq & unsafe { (*p).prereq } == prereq {
                            __state = 10;
                        } else { __state = 9; }
                    }
                    8 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 6;
                    }
                    9 => {
                        if unsafe { (*p).r_run } as i32 <= r_run_1 as i32 &&
                                unsafe { (*p).prereq } & prereq == unsafe { (*p).prereq } {
                            __state = 11;
                        } else { __state = 8; }
                    }
                    10 => { __state = 2; }
                    11 => { return 0; }
                    12 => { __state = 2; }
                    13 => {
                        p =
                            &mut (*p_set_1).a[{
                                            let __p = &mut (*p_set_1).n;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as usize];
                        __state = 15;
                    }
                    14 => {
                        p = &raw mut (*p_set_1).a[0 as usize] as *mut WhereOrCost;
                        __state = 16;
                    }
                    15 => { unsafe { (*p).n_out = n_out_1 }; __state = 12; }
                    16 => { i = 1 as u16; __state = 18; }
                    17 => {
                        if unsafe { (*p).r_run } as i32 <= r_run_1 as i32 {
                            __state = 22;
                        } else { __state = 12; }
                    }
                    18 => {
                        if (i as i32) < (*p_set_1).n as i32 {
                            __state = 19;
                        } else { __state = 17; }
                    }
                    19 => {
                        if unsafe { (*p).r_run } as i32 >
                                (*p_set_1).a[i as usize].r_run as i32 {
                            __state = 21;
                        } else { __state = 20; }
                    }
                    20 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 18;
                    }
                    21 => {
                        p =
                            unsafe {
                                (&raw mut (*p_set_1).a[0 as usize] as
                                        *mut WhereOrCost).add(i as usize)
                            };
                        __state = 20;
                    }
                    22 => { return 0; }
                    23 => { unsafe { (*p).r_run = r_run_1 }; __state = 24; }
                    24 => {
                        if unsafe { (*p).n_out } as i32 > n_out_1 as i32 {
                            __state = 26;
                        } else { __state = 25; }
                    }
                    25 => { return 1; }
                    26 => { unsafe { (*p).n_out = n_out_1 }; __state = 25; }
                    _ => {}
                }
            }
        }
        unreachable!();
    }
}

///* Search the list of WhereLoops in *ppPrev looking for one that can be
///* replaced by pTemplate.
///*
///* Return NULL if pTemplate does not belong on the WhereLoop list.
///* In other words if pTemplate ought to be dropped from further consideration.
///*
///* If pX is a WhereLoop that pTemplate can replace, then return the
///* link that points to pX.
///*
///* If pTemplate cannot replace any existing element of the list but needs
///* to be added to the list as a new entry, then return a pointer to the
///* tail of the list.
#[allow(unused_doc_comments)]
extern "C" fn where_loop_find_lesser(mut pp_prev_1: *mut *mut WhereLoop,
    p_template_1: &WhereLoop) -> *mut *mut WhereLoop {
    let mut p: *mut WhereLoop = core::ptr::null_mut();
    {
        p = unsafe { *pp_prev_1 };
        '__b33: loop {
            if !(!(p).is_null()) { break '__b33; }
            '__c33: loop {
                if unsafe { (*p).i_tab } as i32 !=
                            (*p_template_1).i_tab as i32 ||
                        unsafe { (*p).i_sort_idx } as i32 !=
                            (*p_template_1).i_sort_idx as i32 {

                    /// If either the iTab or iSortIdx values for two WhereLoop are different
                    ///* then those WhereLoops need to be considered separately.  Neither is
                    ///* a candidate to replace the other.
                    break '__c33;
                }

                /// In the current implementation, the rSetup value is either zero
                ///* or the cost of building an automatic index (NlogN) and the NlogN
                ///* is the same for compatible WhereLoops.
                { let _ = 0; };

                /// whereLoopAddBtree() always generates and inserts the automatic index
                ///* case first.  Hence compatible candidate WhereLoops never have a larger
                ///* rSetup. Call this SETUP-INVARIANT
                { let _ = 0; };
                if unsafe { (*p).ws_flags } & 16384 as u32 != 0 as u32 &&
                                    (*p_template_1).n_skip as i32 == 0 &&
                                (*p_template_1).ws_flags & 512 as u32 != 0 as u32 &&
                            (*p_template_1).ws_flags & 1 as u32 != 0 as u32 &&
                        unsafe { (*p).prereq } & (*p_template_1).prereq as Bitmask
                            == (*p_template_1).prereq {
                    break '__b33;
                }
                if unsafe { (*p).prereq } & (*p_template_1).prereq as Bitmask
                                    == unsafe { (*p).prereq } &&
                                unsafe { (*p).r_setup } as i32 <=
                                    (*p_template_1).r_setup as i32 &&
                            unsafe { (*p).r_run } as i32 <= (*p_template_1).r_run as i32
                        &&
                        unsafe { (*p).n_out } as i32 <= (*p_template_1).n_out as i32
                    {
                    return core::ptr::null_mut();
                }
                if unsafe { (*p).prereq } & (*p_template_1).prereq as Bitmask
                                == (*p_template_1).prereq &&
                            unsafe { (*p).r_run } as i32 >= (*p_template_1).r_run as i32
                        &&
                        unsafe { (*p).n_out } as i32 >= (*p_template_1).n_out as i32
                    {
                    { let _ = 0; };

                    /// SETUP-INVARIANT above
                    break '__b33;
                }
                break '__c33;
            }
            {
                pp_prev_1 = unsafe { &mut (*p).p_next_loop };
                p = unsafe { *pp_prev_1 }
            };
        }
    }
    return pp_prev_1;
}

///* Transfer content from the second pLoop into the first.
extern "C" fn where_loop_xfer(db: *mut Sqlite3, p_to_1: *mut WhereLoop,
    p_from_1: *mut WhereLoop) -> i32 {
    unsafe {
        where_loop_clear_union(db, unsafe { &mut *p_to_1 });
        if unsafe { (*p_from_1).n_l_term } as i32 >
                    unsafe { (*p_to_1).n_l_slot } as i32 &&
                where_loop_resize(db, unsafe { &mut *p_to_1 },
                        unsafe { (*p_from_1).n_l_term } as i32) != 0 {
            unsafe {
                memset(p_to_1 as *mut (), 0,
                    core::mem::offset_of!(WhereLoop, n_l_slot) as u64)
            };
            return 7;
        }
        unsafe {
            memcpy(p_to_1 as *mut (), p_from_1 as *const (),
                core::mem::offset_of!(WhereLoop, n_l_slot) as u64)
        };
        unsafe {
            memcpy(unsafe { (*p_to_1).a_l_term } as *mut (),
                unsafe { (*p_from_1).a_l_term } as *const (),
                unsafe { (*p_to_1).n_l_term } as u64 *
                    core::mem::size_of::<*mut WhereTerm>() as u64)
        };
        if unsafe { (*p_from_1).ws_flags } & 1024 as u32 != 0 {
            unsafe { (*p_from_1).u.vtab.set_need_free(0 as u32 as u32) };
        } else if unsafe { (*p_from_1).ws_flags } & 16384 as u32 != 0 as u32 {
            unsafe { (*p_from_1).u.btree.p_index = core::ptr::null_mut() };
        }
        return 0;
    }
}

///* Insert or replace a WhereLoop entry using the template supplied.
///*
///* An existing WhereLoop entry might be overwritten if the new template
///* is better and has fewer dependencies.  Or the template will be ignored
///* and no insert will occur if an existing WhereLoop is faster and has
///* fewer dependencies than the template.  Otherwise a new WhereLoop is
///* added based on the template.
///*
///* If pBuilder->pOrSet is not NULL then we care about only the
///* prerequisites and rRun and nOut costs of the N best loops.  That
///* information is gathered in the pBuilder->pOrSet object.  This special
///* processing mode is used only for OR clause processing.
///*
///* When accumulating multiple loops (when pBuilder->pOrSet is NULL) we
///* still might overwrite similar loops with the new template if the
///* new template is better.  Loops may be overwritten if the following
///* conditions are met:
///*
///*    (1)  They have the same iTab.
///*    (2)  They have the same iSortIdx.
///*    (3)  The template has same or fewer dependencies than the current loop
///*    (4)  The template has the same or lower cost than the current loop
#[allow(unused_doc_comments)]
extern "C" fn where_loop_insert(p_builder_1: &mut WhereLoopBuilder,
    p_template_1: *mut WhereLoop) -> i32 {
    unsafe {
        let mut pp_prev: *mut *mut WhereLoop = core::ptr::null_mut();
        let mut p: *mut WhereLoop = core::ptr::null_mut();
        let p_w_info: *mut WhereInfo = (*p_builder_1).p_w_info;
        let db: *mut Sqlite3 =
            unsafe { (*unsafe { (*p_w_info).p_parse }).db };
        let mut rc: i32 = 0;
        if (*p_builder_1).i_plan_limit == 0 as u32 {
            if !((*p_builder_1).p_or_set).is_null() {
                unsafe { (*(*p_builder_1).p_or_set).n = 0 as u16 };
            }
            return 101;
        }
        {
            let __p = &mut (*p_builder_1).i_plan_limit;
            let __t = *__p;
            *__p -= 1;
            __t
        };
        where_loop_adjust_cost(unsafe { (*p_w_info).p_loops } as
                *const WhereLoop, p_template_1);
        if (*p_builder_1).p_or_set != core::ptr::null_mut() {
            if unsafe { (*p_template_1).n_l_term } != 0 {
                where_or_insert(unsafe { &mut *(*p_builder_1).p_or_set },
                    unsafe { (*p_template_1).prereq },
                    unsafe { (*p_template_1).r_run },
                    unsafe { (*p_template_1).n_out });
            }
            return 0;
        }

        /// Look for an existing WhereLoop to replace with pTemplate
        (pp_prev =
            where_loop_find_lesser(unsafe { &mut (*p_w_info).p_loops },
                unsafe { &*p_template_1 }));
        if pp_prev == core::ptr::null_mut() {

            /// There already exists a WhereLoop on the list that is better
            ///* than pTemplate, so just ignore pTemplate
            /// 0x8
            return 0;
        } else { p = unsafe { *pp_prev }; }
        if p == core::ptr::null_mut() {

            /// Allocate a new WhereLoop to add to the end of the list
            unsafe {
                *pp_prev =
                    {
                        p =
                            unsafe {
                                    sqlite3_db_malloc_raw_nn(db,
                                        core::mem::size_of::<WhereLoop>() as u64)
                                } as *mut WhereLoop;
                        p
                    }
            };
            if p == core::ptr::null_mut() { return 7; }
            where_loop_init(unsafe { &mut *p });
            unsafe { (*p).p_next_loop = core::ptr::null_mut() };
        } else {
            /// We will be overwriting WhereLoop p[].  But before we do, first
            ///* go through the rest of the list and delete any other entries besides
            ///* p[] that are also supplanted by pTemplate
            let mut pp_tail: *mut *mut WhereLoop =
                unsafe { &mut (*p).p_next_loop };
            let mut p_to_del: *mut WhereLoop = core::ptr::null_mut();
            while !(unsafe { *pp_tail }).is_null() {
                pp_tail =
                    where_loop_find_lesser(pp_tail, unsafe { &*p_template_1 });
                if pp_tail == core::ptr::null_mut() { break; }
                p_to_del = unsafe { *pp_tail };
                if p_to_del == core::ptr::null_mut() { break; }
                unsafe { *pp_tail = unsafe { (*p_to_del).p_next_loop } };

                /// 0x8
                where_loop_delete(db, p_to_del);
            }
        }
        rc = where_loop_xfer(db, p, p_template_1);
        if unsafe { (*p).ws_flags } & 1024 as u32 == 0 as u32 {
            let p_index: *const Index =
                unsafe { (*p).u.btree.p_index } as *const Index;
            if !(p_index).is_null() &&
                    unsafe { (*p_index).idx_type() } as i32 == 3 {
                unsafe { (*p).u.btree.p_index = core::ptr::null_mut() };
            }
        }
        return rc;
    }
}

///* Argument pIdxInfo is already populated with all constraints that may
///* be used by the virtual table identified by pBuilder->pNew->iTab. This
///* function marks a subset of those constraints usable, invokes the
///* xBestIndex method and adds the returned plan to pBuilder.
///*
///* A constraint is marked usable if:
///*
///*   * Argument mUsable indicates that its prerequisites are available, and
///*
///*   * It is not one of the operators specified in the mExclude mask passed
///*     as the fourth argument (which in practice is either WO_IN or 0).
///*
///* Argument mPrereq is a mask of tables that must be scanned before the
///* virtual table in question. These are added to the plans prerequisites
///* before it is added to pBuilder.
///*
///* Output parameter *pbIn is set to true if the plan added to pBuilder
///* uses one or more WO_IN terms, or false otherwise.
#[allow(unused_doc_comments)]
extern "C" fn where_loop_add_virtual_one(p_builder_1: *mut WhereLoopBuilder,
    m_prereq_1: Bitmask, m_usable_1: Bitmask, m_exclude_1: u16,
    p_idx_info_1: *mut Sqlite3IndexInfo, m_no_omit_1: u16, pb_in_1: &mut i32,
    pb_retry_limit_1: *mut i32) -> i32 {
    unsafe {
        let p_wc: *mut WhereClause = unsafe { (*p_builder_1).p_wc };
        let p_hidden: *mut HiddenIndexInfo =
            unsafe { &raw mut *p_idx_info_1.offset(1 as isize) } as
                *mut HiddenIndexInfo;
        let mut p_idx_cons: *mut Sqlite3IndexConstraint =
            core::ptr::null_mut();
        let p_usage: *mut Sqlite3IndexConstraintUsage =
            unsafe { (*p_idx_info_1).a_constraint_usage };
        let mut i: i32 = 0;
        let mut mx_term: i32 = 0;
        let mut rc: i32 = 0;
        let p_new: *mut WhereLoop = unsafe { (*p_builder_1).p_new };
        let p_parse: *mut Parse =
            unsafe { (*unsafe { (*p_builder_1).p_w_info }).p_parse };
        let p_src: *const SrcItem =
            unsafe {
                    &raw mut *(unsafe {
                                        (*unsafe {
                                                            (*unsafe { (*p_builder_1).p_w_info }).p_tab_list
                                                        }).a.as_ptr()
                                    } as *mut SrcItem).add(unsafe { (*p_new).i_tab } as usize)
                } as *const SrcItem;
        let n_constraint: i32 = unsafe { (*p_idx_info_1).n_constraint };
        { let _ = 0; };
        *pb_in_1 = 0;
        unsafe { (*p_new).prereq = m_prereq_1 };

        /// Set the usable flag on the subset of constraints identified by
        ///* arguments mUsable and mExclude.
        (p_idx_cons =
            unsafe {
                *(unsafe { &raw mut (*p_idx_info_1).a_constraint } as
                        *mut *mut Sqlite3IndexConstraint)
            });
        {
            i = 0;
            '__b35: loop {
                if !(i < n_constraint) { break '__b35; }
                '__c35: loop {
                    let p_term: *mut WhereTerm =
                        term_from_where_clause(p_wc,
                            unsafe { (*p_idx_cons).i_term_offset });
                    unsafe { (*p_idx_cons).usable = 0 as u8 };
                    if unsafe { (*p_term).prereq_right } & m_usable_1 ==
                                    unsafe { (*p_term).prereq_right } &&
                                unsafe { (*p_term).e_operator } as i32 & m_exclude_1 as i32
                                    == 0 &&
                            (!(pb_retry_limit_1).is_null() ||
                                (is_limit_term(unsafe { &*p_term }) == 0) as i32 != 0) {
                        unsafe { (*p_idx_cons).usable = 1 as u8 };
                    }
                    break '__c35;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_idx_cons;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }

        /// Initialize the output fields of the sqlite3_index_info structure
        unsafe {
            memset(p_usage as *mut (), 0,
                core::mem::size_of::<Sqlite3IndexConstraintUsage>() as u64 *
                    n_constraint as u64)
        };
        { let _ = 0; };
        unsafe { (*p_idx_info_1).idx_str = core::ptr::null_mut() };
        unsafe { (*p_idx_info_1).idx_num = 0 };
        unsafe { (*p_idx_info_1).order_by_consumed = 0 };
        unsafe { (*p_idx_info_1).estimated_cost = 1e99 / 2 as f64 };
        unsafe { (*p_idx_info_1).estimated_rows = 25 as Sqlite3Int64 };
        unsafe { (*p_idx_info_1).idx_flags = 0 };
        unsafe { (*p_hidden).m_handle_in = 0 as u32 };

        /// Invoke the virtual table xBestIndex() method
        (rc =
            vtab_best_index(p_parse, unsafe { (*p_src).p_s_tab },
                p_idx_info_1));
        if rc != 0 {
            if rc == 19 {
                free_idx_str(unsafe { &mut *p_idx_info_1 });
                return 0;
            }
            return rc;
        }
        mx_term = -1;
        { let _ = 0; };
        unsafe {
            memset(unsafe { (*p_new).a_l_term } as *mut (), 0,
                core::mem::size_of::<*mut WhereTerm>() as u64 *
                    n_constraint as u64)
        };
        unsafe {
            memset(unsafe { &raw mut (*p_new).u.vtab } as *mut (), 0, 24)
        };
        p_idx_cons =
            unsafe {
                *(unsafe { &raw mut (*p_idx_info_1).a_constraint } as
                        *mut *mut Sqlite3IndexConstraint)
            };
        {
            i = 0;
            '__b36: loop {
                if !(i < n_constraint) { break '__b36; }
                '__c36: loop {
                    let mut i_term: i32 = 0;
                    if {
                                i_term =
                                    unsafe { (*p_usage.offset(i as isize)).argv_index } - 1;
                                i_term
                            } >= 0 {
                        let mut p_term_1: *mut WhereTerm = core::ptr::null_mut();
                        let j: i32 = unsafe { (*p_idx_cons).i_term_offset };
                        if i_term >= n_constraint || j < 0 ||
                                        { p_term_1 = term_from_where_clause(p_wc, j); p_term_1 } ==
                                            core::ptr::null_mut() ||
                                    unsafe {
                                            *unsafe { (*p_new).a_l_term.offset(i_term as isize) }
                                        } != core::ptr::null_mut() ||
                                unsafe { (*p_idx_cons).usable } as i32 == 0 {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"%s.xBestIndex malfunction".as_ptr() as *mut i8 as
                                        *const i8, unsafe { (*unsafe { (*p_src).p_s_tab }).z_name })
                            };
                            free_idx_str(unsafe { &mut *p_idx_info_1 });
                            return 1;
                        }
                        unsafe {
                            (*p_new).prereq |= unsafe { (*p_term_1).prereq_right }
                        };
                        { let _ = 0; };
                        unsafe {
                            *unsafe { (*p_new).a_l_term.offset(i_term as isize) } =
                                p_term_1
                        };
                        if i_term > mx_term { mx_term = i_term; }
                        if unsafe { (*p_usage.offset(i as isize)).omit } != 0 {
                            if i < 16 && 1 << i & m_no_omit_1 as i32 == 0 {
                                unsafe {
                                    (*p_new).u.vtab.omit_mask |= (1 << i_term) as u16
                                };
                            } else {}
                            if unsafe { (*p_term_1).e_match_op } as i32 == 74 {
                                unsafe {
                                    (*p_new).u.vtab.set_b_omit_offset(1 as u32 as u32)
                                };
                            }
                        }
                        if if i <= 31 { (1 as u32) << i } else { 0 as u32 } &
                                    unsafe { (*p_hidden).m_handle_in } != 0 {
                            unsafe {
                                (*p_new).u.vtab.m_handle_in |= (1 as u32) << i_term
                            };
                        } else if unsafe { (*p_term_1).e_operator } as i32 & 1 != 0
                            {

                            /// A virtual table that is constrained by an IN clause may not
                            ///* consume the ORDER BY clause because (1) the order of IN terms
                            ///* is not necessarily related to the order of output terms and
                            ///* (2) Multiple outputs from a single IN value will not merge
                            ///* together.
                            unsafe { (*p_idx_info_1).order_by_consumed = 0 };
                            unsafe { (*p_idx_info_1).idx_flags &= !1 };
                            *pb_in_1 = 1;
                            { let _ = 0; };
                        }

                        /// Unless pbRetryLimit is non-NULL, there should be no LIMIT/OFFSET
                        ///* terms. And if there are any, they should follow all other terms.
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        if is_limit_term(unsafe { &*p_term_1 }) != 0 &&
                                (*pb_in_1 != 0 ||
                                    (all_constraints_used(unsafe {
                                                        let __p = p_usage as *const Sqlite3IndexConstraintUsage;
                                                        if __p.is_null() {
                                                            &[]
                                                        } else { core::slice::from_raw_parts(__p, i as usize) }
                                                    }) == 0) as i32 != 0) {

                            /// If there is an IN(...) term handled as an == (separate call to
                            ///* xFilter for each value on the RHS of the IN) and a LIMIT or
                            ///* OFFSET term handled as well, the plan is unusable. Similarly,
                            ///* if there is a LIMIT/OFFSET and there are other unused terms,
                            ///* the plan cannot be used. In these cases set variable *pbRetryLimit
                            ///* to true to tell the caller to retry with LIMIT and OFFSET 
                            ///* disabled.
                            free_idx_str(unsafe { &mut *p_idx_info_1 });
                            unsafe { *pb_retry_limit_1 = 1 };
                            return 0;
                        }
                    }
                    break '__c36;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_idx_cons;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        unsafe { (*p_new).n_l_term = (mx_term + 1) as u16 };
        {
            i = 0;
            '__b37: loop {
                if !(i <= mx_term) { break '__b37; }
                '__c37: loop {
                    if unsafe {
                                *unsafe { (*p_new).a_l_term.offset(i as isize) }
                            } == core::ptr::null_mut() {

                        /// The non-zero argvIdx values must be contiguous.  Raise an
                        ///* error if they are not
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"%s.xBestIndex malfunction".as_ptr() as *mut i8 as
                                    *const i8, unsafe { (*unsafe { (*p_src).p_s_tab }).z_name })
                        };
                        free_idx_str(unsafe { &mut *p_idx_info_1 });
                        return 1;
                    }
                    break '__c37;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        { let _ = 0; };
        unsafe {
            (*p_new).u.vtab.idx_num = unsafe { (*p_idx_info_1).idx_num }
        };
        unsafe {
            (*p_new).u.vtab.set_need_free(unsafe {
                            (*p_idx_info_1).need_to_free_idx_str
                        } as u32 as u32)
        };
        unsafe { (*p_idx_info_1).need_to_free_idx_str = 0 };
        unsafe {
            (*p_new).u.vtab.idx_str = unsafe { (*p_idx_info_1).idx_str }
        };
        unsafe {
            (*p_new).u.vtab.is_ordered =
                if unsafe { (*p_idx_info_1).order_by_consumed } != 0 {
                        unsafe { (*p_idx_info_1).n_order_by }
                    } else { 0 } as i8
        };
        unsafe {
            (*p_new).u.vtab.set_b_idx_num_hex((unsafe {
                                    (*p_idx_info_1).idx_flags
                                } & 2 != 0) as u32 as u32)
        };
        unsafe { (*p_new).r_setup = 0 as LogEst };
        unsafe {
            (*p_new).r_run =
                unsafe {
                    sqlite3_log_est_from_double(unsafe {
                            (*p_idx_info_1).estimated_cost
                        })
                }
        };
        unsafe {
            (*p_new).n_out =
                unsafe {
                    sqlite3_log_est(unsafe { (*p_idx_info_1).estimated_rows } as
                            u64)
                }
        };
        if unsafe { (*p_idx_info_1).idx_flags } & 1 != 0 {
            unsafe { (*p_new).ws_flags |= 4096 as u32 };
        } else { unsafe { (*p_new).ws_flags &= !4096 as u32 }; }
        rc = where_loop_insert(unsafe { &mut *p_builder_1 }, p_new);
        if unsafe { (*p_new).u.vtab.need_free() } != 0 {
            unsafe {
                sqlite3_free(unsafe { (*p_new).u.vtab.idx_str } as *mut ())
            };
            unsafe { (*p_new).u.vtab.set_need_free(0 as u32 as u32) };
        }
        return rc;
    }
}

///* Add all WhereLoop objects for a table of the join identified by
///* pBuilder->pNew->iTab.  That table is guaranteed to be a virtual table.
///*
///* If there are no LEFT or CROSS JOIN joins in the query, both mPrereq and
///* mUnusable are set to 0. Otherwise, mPrereq is a mask of all FROM clause
///* entries that occur before the virtual table in the FROM clause and are
///* separated from it by at least one LEFT or CROSS JOIN. Similarly, the
///* mUnusable mask contains all FROM clause entries that occur after the
///* virtual table and are separated from it by at least one LEFT or
///* CROSS JOIN.
///*
///* For example, if the query were:
///*
///*   ... FROM t1, t2 LEFT JOIN t3, t4, vt CROSS JOIN t5, t6;
///*
///* then mPrereq corresponds to (t1, t2) and mUnusable to (t5, t6).
///*
///* All the tables in mPrereq must be scanned before the current virtual
///* table. So any terms for which all prerequisites are satisfied by
///* mPrereq may be specified as "usable" in all calls to xBestIndex.
///* Conversely, all tables in mUnusable must be scanned after the current
///* virtual table, so any terms for which the prerequisites overlap with
///* mUnusable should always be configured as "not-usable" for xBestIndex.
#[allow(unused_doc_comments)]
extern "C" fn where_loop_add_virtual(p_builder_1: *mut WhereLoopBuilder,
    m_prereq_1: Bitmask, m_unusable_1: Bitmask) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        /// Return code
        let mut p_w_info: *mut WhereInfo = core::ptr::null_mut();
        /// WHERE analysis context
        let mut p_parse: *const Parse = core::ptr::null();
        /// The parsing context
        let mut p_wc: *mut WhereClause = core::ptr::null_mut();
        /// The WHERE clause
        let mut p_src: *mut SrcItem = core::ptr::null_mut();
        /// The FROM clause term to search
        let mut p: *mut Sqlite3IndexInfo = core::ptr::null_mut();
        /// Object to pass to xBestIndex()
        let mut n_constraint: i32 = 0;
        /// Number of constraints in p
        let mut b_in: i32 = 0;
        /// True if plan uses IN(...) operator
        let mut p_new: *mut WhereLoop = core::ptr::null_mut();
        let mut m_best: Bitmask = 0 as Bitmask;
        /// Tables used by best possible plan
        let mut m_no_omit: u16 = 0 as u16;
        let mut b_retry: i32 = 0;

        /// True to retry with LIMIT/OFFSET disabled
        { let _ = 0; };
        p_w_info = unsafe { (*p_builder_1).p_w_info };
        p_parse = unsafe { (*p_w_info).p_parse };
        p_wc = unsafe { (*p_builder_1).p_wc };
        p_new = unsafe { (*p_builder_1).p_new };
        p_src =
            unsafe {
                &mut *(unsafe {
                                    (*unsafe { (*p_w_info).p_tab_list }).a.as_ptr()
                                } as *mut SrcItem).add(unsafe { (*p_new).i_tab } as usize)
            };
        { let _ = 0; };
        p =
            allocate_index_info(unsafe { &*p_w_info }, p_wc, m_unusable_1,
                p_src as *const SrcItem, &mut m_no_omit);
        if p == core::ptr::null_mut() { return 7; }
        unsafe { (*p_new).r_setup = 0 as LogEst };
        unsafe { (*p_new).ws_flags = 1024 as u32 };
        unsafe { (*p_new).n_l_term = 0 as u16 };
        unsafe { (*p_new).u.vtab.set_need_free(0 as u32 as u32) };
        n_constraint = unsafe { (*p).n_constraint };
        if where_loop_resize(unsafe { (*p_parse).db }, unsafe { &mut *p_new },
                    n_constraint) != 0 {
            free_index_info(unsafe { (*p_parse).db }, p);
            return 7;
        }
        rc =
            where_loop_add_virtual_one(p_builder_1, m_prereq_1,
                -1i32 as Bitmask, 0 as u16, p, m_no_omit, &mut b_in,
                &mut b_retry);
        if b_retry != 0 {
            { let _ = 0; };
            rc =
                where_loop_add_virtual_one(p_builder_1, m_prereq_1,
                    -1i32 as Bitmask, 0 as u16, p, m_no_omit, &mut b_in,
                    core::ptr::null_mut());
        }
        if rc == 0 &&
                ({ m_best = unsafe { (*p_new).prereq } & !m_prereq_1; m_best }
                        != 0 as u64 || b_in != 0) {
            let mut seen_zero: i32 = 0;
            /// True if a plan with no prereqs seen
            let mut seen_zero_no_in: i32 = 0;
            /// Plan with no prereqs and no IN(...) seen
            let mut m_prev: Bitmask = 0 as Bitmask;
            let mut m_best_no_in: Bitmask = 0 as Bitmask;
            if b_in != 0 {
                rc =
                    where_loop_add_virtual_one(p_builder_1, m_prereq_1,
                        -1i32 as Bitmask, 1 as u16, p, m_no_omit, &mut b_in,
                        core::ptr::null_mut());
                { let _ = 0; };
                m_best_no_in = unsafe { (*p_new).prereq } & !m_prereq_1;
                if m_best_no_in == 0 as u64 {
                    seen_zero = 1;
                    seen_zero_no_in = 1;
                }
            }
            while rc == 0 {
                let mut i: i32 = 0;
                let mut m_next: Bitmask = -1i32 as Bitmask;
                { let _ = 0; };
                {
                    i = 0;
                    '__b39: loop {
                        if !(i < n_constraint) { break '__b39; }
                        '__c39: loop {
                            let i_term: i32 =
                                unsafe {
                                    (*unsafe {
                                                (*p).a_constraint.offset(i as isize)
                                            }).i_term_offset
                                };
                            let m_this: Bitmask =
                                unsafe {
                                        (*term_from_where_clause(p_wc, i_term)).prereq_right
                                    } & !m_prereq_1;
                            if m_this > m_prev && m_this < m_next { m_next = m_this; }
                            break '__c39;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                m_prev = m_next;
                if m_next == -1i32 as Bitmask { break; }
                if m_next == m_best || m_next == m_best_no_in { continue; }
                rc =
                    where_loop_add_virtual_one(p_builder_1, m_prereq_1,
                        m_next | m_prereq_1, 0 as u16, p, m_no_omit, &mut b_in,
                        core::ptr::null_mut());
                if unsafe { (*p_new).prereq } == m_prereq_1 {
                    seen_zero = 1;
                    if b_in == 0 { seen_zero_no_in = 1; }
                }
            }
            if rc == 0 && seen_zero == 0 {
                rc =
                    where_loop_add_virtual_one(p_builder_1, m_prereq_1,
                        m_prereq_1, 0 as u16, p, m_no_omit, &mut b_in,
                        core::ptr::null_mut());
                if b_in == 0 { seen_zero_no_in = 1; }
            }
            if rc == 0 && seen_zero_no_in == 0 {
                rc =
                    where_loop_add_virtual_one(p_builder_1, m_prereq_1,
                        m_prereq_1, 1 as u16, p, m_no_omit, &mut b_in,
                        core::ptr::null_mut());
            }
        }
        free_index_info(unsafe { (*p_parse).db }, p);
        return rc;
    }
}

///* Estimate the logarithm of the input value to base 2.
extern "C" fn est_log(n_1: LogEst) -> LogEst {
    return if n_1 as i32 <= 10 {
                0
            } else { (unsafe { sqlite3_log_est(n_1 as u64) }) as i32 - 33 } as
            LogEst;
}

///* Return true if column iCol of table pTab seem like it might be a
///* good column to use as part of a query-time index.
///*
///* Current algorithm (subject to improvement!):
///*
///*   1.   If iCol is already the left-most column of some other index,
///*        then return false.
///*
///*   2.   If iCol is part of an existing index that has an aiRowLogEst of
///*        more than 20, then return false.
///*
///*   3.   If no disqualifying conditions above are found, return true.
///*
///* 2025-01-03: I experimented with a new rule that returns false if the
///* the datatype of the column is "BOOLEAN". This did not improve
///* performance on any queries at hand, but it did burn CPU cycles, so the
///* idea was not committed.
extern "C" fn column_is_good_index_candidate(p_tab_1: &Table, i_col_1: i32)
    -> i32 {
    let mut p_idx: *const Index = core::ptr::null();
    {
        p_idx = (*p_tab_1).p_index as *const Index;
        '__b40: loop {
            if !(p_idx != core::ptr::null()) { break '__b40; }
            '__c40: loop {
                let mut j: i32 = 0;
                {
                    j = 0;
                    '__b41: loop {
                        if !(j < unsafe { (*p_idx).n_key_col } as i32) {
                            break '__b41;
                        }
                        '__c41: loop {
                            if unsafe {
                                            *unsafe { (*p_idx).ai_column.offset(j as isize) }
                                        } as i32 == i_col_1 {
                                if j == 0 { return 0; }
                                if unsafe { (*p_idx).has_stat1() } != 0 &&
                                        unsafe {
                                                    *unsafe { (*p_idx).ai_row_log_est.offset((j + 1) as isize) }
                                                } as i32 > 20 {
                                    return 0;
                                }
                                break '__b41;
                            }
                            break '__c41;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                break '__c40;
            }
            p_idx = unsafe { (*p_idx).p_next } as *const Index;
        }
    }
    return 1;
}

///* Return TRUE if the WHERE clause term pTerm is of a form where it
///* could be used with an index to access pSrc, assuming an appropriate
///* index existed.
extern "C" fn term_can_drive_index(p_term_1: *const WhereTerm,
    p_src_1: *const SrcItem, not_ready_1: Bitmask) -> i32 {
    unsafe {
        let mut aff: i8 = 0 as i8;
        let mut left_col: i32 = 0;
        if unsafe { (*p_term_1).left_cursor } as i32 !=
                unsafe { (*p_src_1).i_cursor } {
            return 0;
        }
        if unsafe { (*p_term_1).e_operator } as i32 & (2 | 128) == 0 {
            return 0;
        }
        { let _ = 0; };
        if unsafe { (*p_src_1).fg.jointype } as i32 & (8 | 64 | 16) != 0 &&
                (constraint_compatible_with_outer_join(unsafe { &*p_term_1 },
                                unsafe { &*p_src_1 }) == 0) as i32 != 0 {
            return 0;
        }
        if unsafe { (*p_term_1).prereq_right } & not_ready_1 as Bitmask !=
                0 as u64 {
            return 0;
        }
        { let _ = 0; };
        left_col = unsafe { (*p_term_1).u.x.left_column } as i32;
        if left_col < 0 { return 0; }
        aff =
            unsafe {
                (*unsafe {
                            (*unsafe {
                                                (*p_src_1).p_s_tab
                                            }).a_col.offset(left_col as isize)
                        }).affinity
            };
        if (unsafe {
                            sqlite3_index_affinity_ok(unsafe { (*p_term_1).p_expr } as
                                    *const Expr, aff)
                        } == 0) as i32 != 0 {
            return 0;
        }
        return column_is_good_index_candidate(unsafe {
                    &*unsafe { (*p_src_1).p_s_tab }
                }, left_col);
    }
}

/// Check to see if a partial index with pPartIndexWhere can be used
///* in the current query.  Return true if it can be and false if not.
extern "C" fn where_usable_partial_index(i_tab_1: i32, jointype: u8,
    p_wc_1: *mut WhereClause, mut p_where_1: *const Expr) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p_term: *const WhereTerm = core::ptr::null();
        let mut p_parse: *const Parse = core::ptr::null();
        if jointype as i32 & 64 != 0 { return 0; }
        p_parse = unsafe { (*unsafe { (*p_wc_1).p_w_info }).p_parse };
        while unsafe { (*p_where_1).op } as i32 == 44 {
            if (where_usable_partial_index(i_tab_1, jointype, p_wc_1,
                                unsafe { (*p_where_1).p_left } as *const Expr) == 0) as i32
                    != 0 {
                return 0;
            }
            p_where_1 = unsafe { (*p_where_1).p_right };
        }
        {
            { i = 0; p_term = unsafe { (*p_wc_1).a } };
            '__b43: loop {
                if !(i < unsafe { (*p_wc_1).n_term }) { break '__b43; }
                '__c43: loop {
                    let mut p_expr: *const Expr = core::ptr::null();
                    p_expr = unsafe { (*p_term).p_expr };
                    if (!(unsafe { (*p_expr).flags } & 1 as u32 != 0 as u32) as
                                                    i32 != 0 || unsafe { (*p_expr).w.i_join } == i_tab_1) &&
                                        (jointype as i32 & 32 == 0 ||
                                            unsafe { (*p_expr).flags } & 1 as u32 != 0 as u32) &&
                                    unsafe {
                                            sqlite3_expr_implies_expr(p_parse as *const Parse,
                                                p_expr as *const Expr, p_where_1 as *const Expr, i_tab_1)
                                        } != 0 &&
                                (unsafe {
                                                sqlite3_expr_implies_expr(p_parse as *const Parse,
                                                    p_expr as *const Expr, p_where_1 as *const Expr, -1)
                                            } == 0) as i32 != 0 &&
                            unsafe { (*p_term).wt_flags } as i32 & 128 == 0 {
                        return 1;
                    }
                    break '__c43;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_term;
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

///* Return True if it is possible that pIndex might be useful in
///* implementing the ORDER BY clause in pBuilder.
///*
///* Return False if pBuilder does not contain an ORDER BY clause or
///* if there is no way for pIndex to be useful in implementing that
///* ORDER BY clause.
extern "C" fn index_might_help_with_order_by(p_builder_1: &WhereLoopBuilder,
    p_index_1: &Index, i_cursor_1: i32) -> i32 {
    let mut p_ob: *const ExprList = core::ptr::null();
    let mut a_col_expr: *const ExprList = core::ptr::null();
    let mut ii: i32 = 0;
    let mut jj: i32 = 0;
    if (*p_index_1).b_unordered() != 0 { return 0; }
    if { p_ob = unsafe { (*(*p_builder_1).p_w_info).p_order_by }; p_ob } ==
            core::ptr::null_mut() {
        return 0;
    }
    {
        ii = 0;
        '__b44: loop {
            if !(ii < unsafe { (*p_ob).n_expr }) { break '__b44; }
            '__c44: loop {
                let p_expr: *mut Expr =
                    unsafe {
                        sqlite3_expr_skip_collate_and_likely(unsafe {
                                (*(unsafe { (*p_ob).a.as_ptr() } as
                                                *mut ExprListItem).offset(ii as isize)).p_expr
                            })
                    };
                if p_expr == core::ptr::null_mut() { break '__c44; }
                if (unsafe { (*p_expr).op } as i32 == 168 ||
                            unsafe { (*p_expr).op } as i32 == 170) &&
                        unsafe { (*p_expr).i_table } == i_cursor_1 {
                    if (unsafe { (*p_expr).i_column } as i32) < 0 { return 1; }
                    {
                        jj = 0;
                        '__b45: loop {
                            if !(jj < (*p_index_1).n_key_col as i32) { break '__b45; }
                            '__c45: loop {
                                if unsafe { (*p_expr).i_column } as i32 ==
                                        unsafe { *(*p_index_1).ai_column.offset(jj as isize) } as
                                            i32 {
                                    return 1;
                                }
                                break '__c45;
                            }
                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                        }
                    }
                } else if { a_col_expr = (*p_index_1).a_col_expr; a_col_expr }
                        != core::ptr::null_mut() {
                    {
                        jj = 0;
                        '__b46: loop {
                            if !(jj < (*p_index_1).n_key_col as i32) { break '__b46; }
                            '__c46: loop {
                                if unsafe { *(*p_index_1).ai_column.offset(jj as isize) } as
                                            i32 != -2 {
                                    break '__c46;
                                }
                                if unsafe {
                                            sqlite3_expr_compare_skip(p_expr,
                                                unsafe {
                                                    (*(unsafe { (*a_col_expr).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(jj as isize)).p_expr
                                                }, i_cursor_1)
                                        } == 0 {
                                    return 1;
                                }
                                break '__c46;
                            }
                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
                break '__c44;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Callback for estLikePatternLength().
///*
///* If this node is a string literal that is longer pWalker->sz, then set
///* pWalker->sz to the byte length of that string literal.
///*
///* pWalker->eCode indicates how to count characters:
///*
///*    eCode==0     Count as a GLOB pattern
///*    eCode==1     Count as a LIKE pattern
#[allow(unused_doc_comments)]
extern "C" fn expr_node_pattern_length_est(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 118 {
            let mut sz: i32 = 0;
            /// Pattern size in bytes
            let mut z: *const u8 =
                unsafe { (*p_expr_1).u.z_token } as *mut u8 as *const u8;
            /// The pattern
            let mut c: u8 = 0 as u8;
            /// Next character of the pattern
            let mut c1: u8 = 0 as u8;
            let mut c2: u8 = 0 as u8;
            let mut c3: u8 = 0 as u8;
            if unsafe { (*p_walker_1).e_code } != 0 {
                c1 = '%' as i32 as u8;
                c2 = '_' as i32 as u8;
                c3 = 0 as u8;
            } else {
                c1 = '*' as i32 as u8;
                c2 = '?' as i32 as u8;
                c3 = '[' as i32 as u8;
            }
            while {
                            c =
                                unsafe {
                                    *{
                                            let __p = &mut z;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                };
                            c
                        } as i32 != 0 {
                if c as i32 == c3 as i32 {
                    if unsafe { *z } != 0 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                    while unsafe { *z } != 0 &&
                            unsafe { *z } as i32 != ']' as i32 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                } else if c as i32 != c1 as i32 && c as i32 != c2 as i32 {
                    { let __p = &mut sz; let __t = *__p; *__p += 1; __t };
                }
            }
            if sz > unsafe { (*p_walker_1).u.sz } {
                unsafe { (*p_walker_1).u.sz = sz };
            }
        }
        return 0;
    }
}

///* Return the length of the longest string literal in the given
///* expression.
///*
///* eCode indicates how to count characters:
///*
///*    eCode==0     Count as a GLOB pattern
///*    eCode==1     Count as a LIKE pattern
extern "C" fn est_like_pattern_length(p: *mut Expr, e_code_1: u16) -> i32 {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        w.u.sz = 0;
        w.e_code = e_code_1;
        w.x_expr_callback = Some(expr_node_pattern_length_est);
        w.x_select_callback = Some(sqlite3_select_walk_fail);
        unsafe { sqlite3_walk_expr(&mut w, p) };
        return w.u.sz;
    }
}

///* Adjust the WhereLoop.nOut value downward to account for terms of the
///* WHERE clause that reference the loop but which are not used by an
///* index.
///
///* For every WHERE clause term that is not used by the index
///* and which has a truth probability assigned by one of the likelihood(),
///* likely(), or unlikely() SQL functions, reduce the estimated number
///* of output rows by the probability specified.
///*
///* TUNING:  For every WHERE clause term that is not used by the index
///* and which does not have an assigned truth probability, heuristics
///* described below are used to try to estimate the truth probability.
///* TODO --> Perhaps this is something that could be improved by better
///* table statistics.
///*
///* Heuristic 1:  Estimate the truth probability as 93.75%.  The 93.75%
///* value corresponds to -1 in LogEst notation, so this means decrement
///* the WhereLoop.nOut field for every such WHERE clause term.
///*
///* Heuristic 2:  If there exists one or more WHERE clause terms of the
///* form "x==EXPR" and EXPR is not a constant 0 or 1, then make sure the
///* final output row estimate is no greater than 1/4 of the total number
///* of rows in the table.  In other words, assume that x==EXPR will filter
///* out at least 3 out of 4 rows.  If EXPR is -1 or 0 or 1, then maybe the
///* "x" column is boolean or else -1 or 0 or 1 is a common default value
///* on the "x" column and so in that case only cap the output row estimate
///* at 1/2 instead of 1/4.
///*
///* Heuristic 3:  If there is a LIKE or GLOB (or REGEXP or MATCH) operator
///* with a large constant pattern, then reduce the size of the search
///* space according to the length of the pattern, under the theory that
///* longer patterns are less likely to match.  This heuristic was added
///* to give better output-row count estimates when preparing queries for
///* the Join-Order Benchmarks.  See forum thread 2026-01-30T09:57:54z
#[allow(unused_doc_comments)]
extern "C" fn where_loop_output_adjust(p_wc_1: &WhereClause,
    p_loop_1: &mut WhereLoop, n_row_1: LogEst) -> () {
    unsafe {
        let mut p_term: *mut WhereTerm = core::ptr::null_mut();
        let mut p_x: *mut WhereTerm = core::ptr::null_mut();
        let not_allowed: Bitmask =
            !((*p_loop_1).prereq | (*p_loop_1).mask_self);
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut i_reduce: LogEst = 0 as LogEst;

        /// pLoop->nOut should not exceed nRow-iReduce
        { let _ = 0; };
        {
            { i = (*p_wc_1).n_base; p_term = (*p_wc_1).a };
            '__b49: loop {
                if !(i > 0) { break '__b49; }
                '__c49: loop {
                    { let _ = 0; };
                    if unsafe { (*p_term).prereq_all } & not_allowed != 0 as u64
                        {
                        break '__c49;
                    }
                    if unsafe { (*p_term).prereq_all } & (*p_loop_1).mask_self
                            == 0 as u64 {
                        break '__c49;
                    }
                    if unsafe { (*p_term).wt_flags } as i32 & 2 != 0 {
                        break '__c49;
                    }
                    {
                        j = (*p_loop_1).n_l_term as i32 - 1;
                        '__b50: loop {
                            if !(j >= 0) { break '__b50; }
                            '__c50: loop {
                                p_x = unsafe { *(*p_loop_1).a_l_term.offset(j as isize) };
                                if p_x == core::ptr::null_mut() { break '__c50; }
                                if p_x == p_term { break '__b50; }
                                if unsafe { (*p_x).i_parent } >= 0 &&
                                        unsafe {
                                                (*p_wc_1).a.offset(unsafe { (*p_x).i_parent } as isize)
                                            } == p_term {
                                    break '__b50;
                                }
                                break '__c50;
                            }
                            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                        }
                    }
                    if j < 0 {
                        unsafe {
                            sqlite3_progress_check(unsafe {
                                    (*(*p_wc_1).p_w_info).p_parse
                                })
                        };
                        if (*p_loop_1).mask_self == unsafe { (*p_term).prereq_all }
                            {
                            if unsafe { (*p_term).e_operator } as i32 & 63 != 0 ||
                                    unsafe {
                                                    (*(unsafe {
                                                                            (*unsafe { (*(*p_wc_1).p_w_info).p_tab_list }).a.as_ptr()
                                                                        } as
                                                                        *mut SrcItem).add((*p_loop_1).i_tab as usize)).fg.jointype
                                                } as i32 & (8 | 64) == 0 {
                                (*p_loop_1).ws_flags |= 8388608 as u32;
                            }
                        }
                        if unsafe { (*p_term).truth_prob } as i32 <= 0 {

                            /// If a truth probability is specified using the likelihood() hints,
                            ///* then use the probability provided by the application.
                            ((*p_loop_1).n_out +=
                                unsafe { (*p_term).truth_prob } as i32 as LogEst);
                        } else {
                            /// In the absence of explicit truth probabilities, use heuristics to
                            ///* guess a reasonable truth probability.
                            let p_op_expr: *const Expr =
                                unsafe { (*p_term).p_expr } as *const Expr;
                            {
                                let __p = &mut (*p_loop_1).n_out;
                                let __t = *__p;
                                *__p -= 1;
                                __t
                            };
                            if unsafe { (*p_term).e_operator } as i32 & (2 | 128) != 0
                                    && unsafe { (*p_term).wt_flags } as i32 & 0 == 0 {
                                let p_right: *const Expr =
                                    unsafe { (*p_op_expr).p_right } as *const Expr;
                                let p_parse: *mut Parse =
                                    unsafe { (*(*p_wc_1).p_w_info).p_parse };
                                let mut k: i32 = 0;
                                if unsafe {
                                                    sqlite3_expr_is_integer(p_right as *const Expr, &mut k,
                                                        p_parse)
                                                } != 0 && k >= -1 && k <= 1 {
                                    k = 10;
                                } else { k = 20; }
                                if (i_reduce as i32) < k {
                                    unsafe { (*p_term).wt_flags |= 8192 as u16 };
                                    i_reduce = k as LogEst;
                                }
                            } else if unsafe { (*p_op_expr).flags } & 256 as u32 !=
                                        0 as u32 && unsafe { (*p_op_expr).op } as i32 == 172 {
                                let mut e_op: i32 = 0;
                                { let _ = 0; };
                                { let _ = 0; };
                                e_op =
                                    unsafe {
                                        sqlite3_expr_is_like_operator(p_op_expr as *const Expr)
                                    };
                                if e_op > 0 {
                                    let mut sz_pattern: i32 = 0;
                                    let p_rhs: *mut Expr =
                                        unsafe {
                                            (*(unsafe { (*unsafe { (*p_op_expr).x.p_list }).a.as_ptr() }
                                                            as *mut ExprListItem).offset(0 as isize)).p_expr
                                        };
                                    e_op = (e_op == 65) as i32;
                                    sz_pattern = est_like_pattern_length(p_rhs, e_op as u16);
                                    if sz_pattern > 0 {
                                        (*p_loop_1).n_out -= (sz_pattern * 2) as LogEst;
                                    }
                                }
                            }
                        }
                    }
                    break '__c49;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                    {
                        let __p = &mut p_term;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        if (*p_loop_1).n_out as i32 > n_row_1 as i32 - i_reduce as i32 {
            (*p_loop_1).n_out = (n_row_1 as i32 - i_reduce as i32) as LogEst;
        }
    }
}

///* This is an sqlite3ParserAddCleanup() callback that is invoked to
///* free the Parse->pIdxEpr list when the Parse object is destroyed.
extern "C" fn where_indexed_expr_cleanup(db: *mut Sqlite3,
    p_object_1: *mut ()) -> () {
    let pp: *mut *mut IndexedExpr = p_object_1 as *mut *mut IndexedExpr;
    while unsafe { *pp } != core::ptr::null_mut() {
        let p: *mut IndexedExpr = unsafe { *pp };
        unsafe { *pp = unsafe { (*p).p_ie_next } };
        unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_expr }) };
        unsafe { sqlite3_db_free_nn(db, p as *mut ()) };
    }
}

///* This function is called for a partial index - one with a WHERE clause - in 
///* two scenarios. In both cases, it determines whether or not the WHERE 
///* clause on the index implies that a column of the table may be safely
///* replaced by a constant expression. For example, in the following 
///* SELECT:
///*
///*   CREATE INDEX i1 ON t1(b, c) WHERE a=<expr>;
///*   SELECT a, b, c FROM t1 WHERE a=<expr> AND b=?;
///*
///* The "a" in the select-list may be replaced by <expr>, iff:
///*
///*    (a) <expr> is a constant expression, and
///*    (b) The (a=<expr>) comparison uses the BINARY collation sequence, and
///*    (c) Column "a" has an affinity other than NONE or BLOB.
///*
///* If argument pItem is NULL, then pMask must not be NULL. In this case this 
///* function is being called as part of determining whether or not pIdx
///* is a covering index. This function clears any bits in (*pMask) 
///* corresponding to columns that may be replaced by constants as described
///* above.
///*
///* Otherwise, if pItem is not NULL, then this function is being called
///* as part of coding a loop that uses index pIdx. In this case, add entries
///* to the Parse.pIdxPartExpr list for each column that can be replaced
///* by a constant.
extern "C" fn where_part_idx_expr(p_parse_1: *mut Parse, p_idx_1: *mut Index,
    mut p_part_1: *const Expr, p_mask_1: *mut Bitmask, i_idx_cur_1: i32,
    p_item_1: *mut SrcItem) -> () {
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_part_1).op } as i32 == 44 {
        where_part_idx_expr(p_parse_1, p_idx_1,
            unsafe { (*p_part_1).p_right } as *const Expr, p_mask_1,
            i_idx_cur_1, p_item_1);
        p_part_1 = unsafe { (*p_part_1).p_left };
    }
    if unsafe { (*p_part_1).op } as i32 == 54 ||
            unsafe { (*p_part_1).op } as i32 == 45 {
        let p_left: *const Expr =
            unsafe { (*p_part_1).p_left } as *const Expr;
        let p_right: *mut Expr = unsafe { (*p_part_1).p_right };
        let mut aff: u8 = 0 as u8;
        if unsafe { (*p_left).op } as i32 != 168 { return; }
        if (unsafe {
                            sqlite3_expr_is_constant(core::ptr::null_mut(), p_right)
                        } == 0) as i32 != 0 {
            return;
        }
        if (unsafe {
                            sqlite3_is_binary(unsafe {
                                        sqlite3_expr_compare_coll_seq(p_parse_1,
                                            p_part_1 as *const Expr)
                                    } as *const CollSeq)
                        } == 0) as i32 != 0 {
            return;
        }
        if (unsafe { (*p_left).i_column } as i32) < 0 { return; }
        aff =
            unsafe {
                    (*unsafe {
                                (*unsafe {
                                                    (*p_idx_1).p_table
                                                }).a_col.offset(unsafe { (*p_left).i_column } as isize)
                            }).affinity
                } as u8;
        if aff as i32 >= 66 {
            if !(p_item_1).is_null() {
                let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
                let p: *mut IndexedExpr =
                    unsafe {
                            sqlite3_db_malloc_raw(db,
                                core::mem::size_of::<IndexedExpr>() as u64)
                        } as *mut IndexedExpr;
                if !(p).is_null() {
                    let b_null_row: i32 =
                        (unsafe { (*p_item_1).fg.jointype } as i32 & (8 | 64) != 0)
                            as i32;
                    unsafe {
                        (*p).p_expr =
                            unsafe { sqlite3_expr_dup(db, p_right as *const Expr, 0) }
                    };
                    unsafe {
                        (*p).i_data_cur = unsafe { (*p_item_1).i_cursor }
                    };
                    unsafe { (*p).i_idx_cur = i_idx_cur_1 };
                    unsafe {
                        (*p).i_idx_col = unsafe { (*p_left).i_column } as i32
                    };
                    unsafe { (*p).b_maybe_null_row = b_null_row as u8 };
                    unsafe {
                        (*p).p_ie_next = unsafe { (*p_parse_1).p_idx_part_expr }
                    };
                    unsafe { (*p).aff = aff };
                    unsafe { (*p_parse_1).p_idx_part_expr = p };
                    if unsafe { (*p).p_ie_next } == core::ptr::null_mut() {
                        let p_arg: *mut () =
                            unsafe { &raw mut (*p_parse_1).p_idx_part_expr } as *mut ();
                        unsafe {
                            sqlite3_parser_add_cleanup(p_parse_1,
                                Some(where_indexed_expr_cleanup), p_arg)
                        };
                    }
                }
            } else if (unsafe { (*p_left).i_column } as i32) <
                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                        1 {
                unsafe {
                    *p_mask_1 &=
                        !((1 as Bitmask) << unsafe { (*p_left).i_column })
                };
            }
        }
    }
}

///* pIdx is an index containing expressions.  Check it see if any of the
///* expressions in the index match the pExpr expression.
extern "C" fn expr_is_covered_by_index(p_expr_1: *const Expr, p_idx_1: &Index,
    i_tab_cur_1: i32) -> i32 {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b52: loop {
            if !(i < (*p_idx_1).n_column as i32) { break '__b52; }
            '__c52: loop {
                if unsafe { *(*p_idx_1).ai_column.offset(i as isize) } as i32
                            == -2 &&
                        unsafe {
                                sqlite3_expr_compare(core::ptr::null(), p_expr_1,
                                    unsafe {
                                            (*(unsafe { (*(*p_idx_1).a_col_expr).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        } as *const Expr, i_tab_cur_1)
                            } == 0 {
                    return 1;
                }
                break '__c52;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Information passed in is pWalk->u.pCovIdxCk.  Call it pCk.
///*
///* If the Expr node references the table with cursor pCk->iTabCur, then
///* make sure that column is covered by the index pCk->pIdx.  We know that
///* all columns less than 63 (really BMS-1) are covered, so we don't need
///* to check them.  But we do need to check any column at 63 or greater.
///*
///* If the index does not cover the column, then set pWalk->eCode to
///* non-zero and return WRC_Abort to stop the search.
///*
///* If this node does not disprove that the index can be a covering index,
///* then just return WRC_Continue, to continue the search.
///*
///* If pCk->pIdx contains indexed expressions and one of those expressions
///* matches pExpr, then prune the search.
#[allow(unused_doc_comments)]
extern "C" fn where_is_covering_index_walk_callback(p_walk_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        /// Loop counter
        let mut p_idx: *const Index = core::ptr::null();
        /// The index of interest
        let mut ai_column: *const i16 = core::ptr::null();
        /// Columns contained in the index
        let mut n_column: u16 = 0 as u16;
        /// Number of columns in the index
        let mut p_ck: *mut CoveringIndexCheck = core::ptr::null_mut();

        /// Info about this search
        (p_ck = unsafe { (*p_walk_1).u.p_cov_idx_ck });
        p_idx = unsafe { (*p_ck).p_idx } as *const Index;
        if unsafe { (*p_expr_1).op } as i32 == 168 ||
                unsafe { (*p_expr_1).op } as i32 == 170 {
            if unsafe { (*p_expr_1).i_table } != unsafe { (*p_ck).i_tab_cur }
                {
                return 0;
            }
            p_idx =
                unsafe { (*unsafe { (*p_walk_1).u.p_cov_idx_ck }).p_idx } as
                    *const Index;
            ai_column = unsafe { (*p_idx).ai_column } as *const i16;
            n_column = unsafe { (*p_idx).n_column } as u16;
            {
                i = 0;
                '__b53: loop {
                    if !(i < n_column as i32) { break '__b53; }
                    '__c53: loop {
                        if unsafe { *ai_column.offset(i as isize) } as i32 ==
                                unsafe { (*p_expr_1).i_column } as i32 {
                            return 0;
                        }
                        break '__c53;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*p_ck).b_unidx = 1 as u8 };
            return 2;
        } else if unsafe { (*p_idx).b_has_expr() } != 0 &&
                expr_is_covered_by_index(p_expr_1 as *const Expr,
                        unsafe { &*p_idx },
                        unsafe {
                            (*unsafe { (*p_walk_1).u.p_cov_idx_ck }).i_tab_cur
                        }) != 0 {
            unsafe { (*p_ck).b_expr = 1 as u8 };
            return 1;
        }
        return 0;
    }
}

///* pIdx is an index that covers all of the low-number columns used by
///* pWInfo->pSelect (columns from 0 through 62) or an index that has
///* expressions terms.  Hence, we cannot determine whether or not it is
///* a covering index by using the colUsed bitmasks.  We have to do a search
///* to see if the index is covering.  This routine does that search.
///*
///* The return value is one of these:
///*
///*      0                The index is definitely not a covering index
///*
///*      WHERE_IDX_ONLY   The index is definitely a covering index
///*
///*      WHERE_EXPRIDX    The index is likely a covering index, but it is
///*                       difficult to determine precisely because of the
///*                       expressions that are indexed.  Score it as a
///*                       covering index, but still keep the main table open
///*                       just in case we need it.
///*
///* This routine is an optimization.  It is always safe to return zero.
///* But returning one of the other two values when zero should have been
///* returned can lead to incorrect bytecode and assertion faults.
#[allow(unused_doc_comments)]
extern "C" fn where_is_covering_index(p_w_info_1: &WhereInfo,
    p_idx_1: *mut Index, i_tab_cur_1: i32) -> u32 {
    unsafe {
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        let mut ck: CoveringIndexCheck = unsafe { core::mem::zeroed() };
        let mut w: Walker = unsafe { core::mem::zeroed() };
        if (*p_w_info_1).p_select == core::ptr::null_mut() {

            /// We don't have access to the full query, so we cannot check to see
            ///* if pIdx is covering.  Assume it is not.
            return 0 as u32;
        }
        if unsafe { (*p_idx_1).b_has_expr() } as i32 == 0 {
            {
                i = 0;
                '__b54: loop {
                    if !(i < unsafe { (*p_idx_1).n_column } as i32) {
                        break '__b54;
                    }
                    '__c54: loop {
                        if unsafe {
                                        *unsafe { (*p_idx_1).ai_column.offset(i as isize) }
                                    } as i32 >=
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                    1 {
                            break '__b54;
                        }
                        break '__c54;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if i >= unsafe { (*p_idx_1).n_column } as i32 {

                /// pIdx does not index any columns greater than 62, but we know from
                ///* colMask that columns greater than 62 are used, so this is not a
                ///* covering index
                return 0 as u32;
            }
        }
        ck.p_idx = p_idx_1;
        ck.i_tab_cur = i_tab_cur_1;
        ck.b_expr = 0 as u8;
        ck.b_unidx = 0 as u8;
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.x_expr_callback = Some(where_is_covering_index_walk_callback);
        w.x_select_callback = Some(sqlite3_select_walk_noop);
        w.u.p_cov_idx_ck = &mut ck;
        unsafe { sqlite3_walk_select(&mut w, (*p_w_info_1).p_select) };
        if ck.b_unidx != 0 {
            rc = 0;
        } else if ck.b_expr != 0 { rc = 67108864; } else { rc = 64; }
        return rc as u32;
    }
}

///* Term pTerm is a vector range comparison operation. The first comparison
///* in the vector can be optimized using column nEq of the index. This
///* function returns the total number of vector elements that can be used
///* as part of the range comparison.
///*
///* For example, if the query is:
///*
///*   WHERE a = ? AND (b, c, d) > (?, ?, ?)
///*
///* and the index:
///*
///*   CREATE INDEX ... ON (a, b, c, d, e)
///*
///* then this function would be invoked with nEq=1. The value returned in
///* this case is 3.
#[allow(unused_doc_comments)]
extern "C" fn where_range_vector_len(p_parse_1: *mut Parse, i_cur_1: i32,
    p_idx_1: &Index, n_eq_1: i32, p_term_1: &WhereTerm) -> i32 {
    unsafe {
        let mut n_cmp: i32 =
            unsafe {
                sqlite3_expr_vector_size(unsafe {
                            (*(*p_term_1).p_expr).p_left
                        } as *const Expr)
            };
        let mut i: i32 = 0;
        n_cmp =
            if n_cmp < (*p_idx_1).n_column as i32 - n_eq_1 {
                n_cmp
            } else { (*p_idx_1).n_column as i32 - n_eq_1 };
        {
            i = 1;
            '__b55: loop {
                if !(i < n_cmp) { break '__b55; }
                '__c55: loop {
                    /// Test if comparison i of pTerm is compatible with column (i+nEq)
                    ///* of the index. If not, exit the loop.
                    let mut aff: i8 = 0 as i8;
                    /// Comparison affinity
                    let mut idxaff: i8 = 0 as i8;
                    /// Indexed columns affinity
                    let mut p_coll: *const CollSeq = core::ptr::null();
                    /// Comparison collation sequence
                    let mut p_lhs: *mut Expr = core::ptr::null_mut();
                    let mut p_rhs: *mut Expr = core::ptr::null_mut();
                    { let _ = 0; };
                    p_lhs =
                        unsafe {
                            (*(unsafe {
                                                (*unsafe {
                                                                    (*unsafe { (*(*p_term_1).p_expr).p_left }).x.p_list
                                                                }).a.as_ptr()
                                            } as *mut ExprListItem).offset(i as isize)).p_expr
                        };
                    p_rhs = unsafe { (*(*p_term_1).p_expr).p_right };
                    if unsafe { (*p_rhs).flags } & 4096 as u32 != 0 as u32 {
                        p_rhs =
                            unsafe {
                                (*(unsafe {
                                                    (*unsafe {
                                                                        (*unsafe { (*p_rhs).x.p_select }).p_e_list
                                                                    }).a.as_ptr()
                                                } as *mut ExprListItem).offset(i as isize)).p_expr
                            };
                    } else {
                        p_rhs =
                            unsafe {
                                (*(unsafe { (*unsafe { (*p_rhs).x.p_list }).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)).p_expr
                            };
                    }
                    if unsafe { (*p_lhs).op } as i32 != 168 ||
                                    unsafe { (*p_lhs).i_table } != i_cur_1 ||
                                unsafe { (*p_lhs).i_column } as i32 !=
                                    unsafe {
                                            *(*p_idx_1).ai_column.offset((i + n_eq_1) as isize)
                                        } as i32 ||
                            unsafe {
                                        *(*p_idx_1).a_sort_order.offset((i + n_eq_1) as isize)
                                    } as i32 !=
                                unsafe { *(*p_idx_1).a_sort_order.offset(n_eq_1 as isize) }
                                    as i32 {
                        break '__b55;
                    }
                    aff =
                        unsafe {
                            sqlite3_compare_affinity(p_rhs as *const Expr,
                                unsafe { sqlite3_expr_affinity(p_lhs as *const Expr) })
                        };
                    idxaff =
                        unsafe {
                            sqlite3_table_column_affinity((*p_idx_1).p_table as
                                    *const Table, unsafe { (*p_lhs).i_column } as i32)
                        };
                    if aff as i32 != idxaff as i32 { break '__b55; }
                    if unsafe { (*(*p_term_1).p_expr).flags } & 1024 as u32 !=
                            0 as u32 {
                        let t: *mut Expr = p_rhs;
                        p_rhs = p_lhs;
                        p_lhs = t;
                    }
                    p_coll =
                        unsafe {
                            sqlite3_binary_compare_coll_seq(p_parse_1,
                                p_lhs as *const Expr, p_rhs as *const Expr)
                        };
                    if p_coll == core::ptr::null_mut() { break '__b55; }
                    if unsafe {
                                sqlite3_str_i_cmp(unsafe { (*p_coll).z_name } as *const i8,
                                    unsafe {
                                        *(*p_idx_1).az_coll.offset((i + n_eq_1) as isize)
                                    })
                            } != 0 {
                        break '__b55;
                    }
                    break '__c55;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return i;
    }
}

///* If it is not NULL, pTerm is a term that provides an upper or lower
///* bound on a range scan. Without considering pTerm, it is estimated
///* that the scan will visit nNew rows. This function returns the number
///* estimated to be visited after taking pTerm into account.
///*
///* If the user explicitly specified a likelihood() value for this term,
///* then the return value is the likelihood multiplied by the number of
///* input rows. Otherwise, this function assumes that an "IS NOT NULL" term
///* has a likelihood of 0.50, and any other term a likelihood of 0.25.
extern "C" fn where_range_adjust(p_term_1: *const WhereTerm, n_new_1: LogEst)
    -> LogEst {
    let mut n_ret: LogEst = n_new_1;
    if !(p_term_1).is_null() {
        if unsafe { (*p_term_1).truth_prob } as i32 <= 0 {
            n_ret += unsafe { (*p_term_1).truth_prob } as i32 as LogEst;
        } else if unsafe { (*p_term_1).wt_flags } as i32 & 128 == 0 {
            n_ret -= 20 as LogEst;
            { let _ = 0; };
        }
    }
    return n_ret;
}

///* This function is used to estimate the number of rows that will be visited
///* by scanning an index for a range of values. The range may have an upper
///* bound, a lower bound, or both. The WHERE clause terms that set the upper
///* and lower bounds are represented by pLower and pUpper respectively. For
///* example, assuming that index p is on t1(a):
///*
///*   ... FROM t1 WHERE a > ? AND a < ? ...
///*                    |_____|   |_____|
///*                       |         |
///*                     pLower    pUpper
///*
///* If either of the upper or lower bound is not present, then NULL is passed in
///* place of the corresponding WhereTerm.
///*
///* The value in (pBuilder->pNew->u.btree.nEq) is the number of the index
///* column subject to the range constraint. Or, equivalently, the number of
///* equality constraints optimized by the proposed index scan. For example,
///* assuming index p is on t1(a, b), and the SQL query is:
///*
///*   ... FROM t1 WHERE a = ? AND b > ? AND b < ? ...
///*
///* then nEq is set to 1 (as the range restricted column, b, is the second
///* left-most column of the index). Or, if the query is:
///*
///*   ... FROM t1 WHERE a > ? AND a < ? ...
///*
///* then nEq is set to 0.
///*
///* When this function is called, *pnOut is set to the sqlite3LogEst() of the
///* number of rows that the index scan is expected to visit without
///* considering the range constraints. If nEq is 0, then *pnOut is the number of
///* rows in the index. Assuming no error occurs, *pnOut is adjusted (reduced)
///* to account for the range constraints pLower and pUpper.
///*
///* In the absence of sqlite_stat4 ANALYZE data, or if such data cannot be
///* used, a single range inequality reduces the search space by a factor of 4.
///* and a pair of constraints (x>? AND x<?) reduces the expected number of
///* rows visited by a factor of 64.
extern "C" fn where_range_scan_est(p_parse_1: *const Parse,
    p_builder_1: *const WhereLoopBuilder, p_lower_1: *mut WhereTerm,
    p_upper_1: *mut WhereTerm, p_loop_1: &mut WhereLoop) -> i32 {
    let rc: i32 = 0;
    let mut n_out: i32 = (*p_loop_1).n_out as i32;
    let mut n_new: LogEst = 0 as LogEst;
    { let _ = p_parse_1; };
    { let _ = p_builder_1; };
    { let _ = 0; };
    { let _ = 0; };
    n_new =
        where_range_adjust(p_lower_1 as *const WhereTerm, n_out as LogEst);
    n_new = where_range_adjust(p_upper_1 as *const WhereTerm, n_new);
    if !(p_lower_1).is_null() && unsafe { (*p_lower_1).truth_prob } as i32 > 0
                && !(p_upper_1).is_null() &&
            unsafe { (*p_upper_1).truth_prob } as i32 > 0 {
        n_new -= 20 as LogEst;
    }
    n_out -=
        (p_lower_1 != core::ptr::null_mut()) as i32 +
            (p_upper_1 != core::ptr::null_mut()) as i32;
    if (n_new as i32) < 10 { n_new = 10 as LogEst; }
    if (n_new as i32) < n_out { n_out = n_new as i32; }
    (*p_loop_1).n_out = n_out as LogEst;
    return rc;
}

///* We have so far matched pBuilder->pNew->u.btree.nEq terms of the
///* index pIndex. Try to match one more.
///*
///* When this function is called, pBuilder->pNew->nOut contains the
///* number of rows expected to be visited by filtering using the nEq
///* terms only. If it is modified, this value is restored before this
///* function returns.
///*
///* If pProbe->idxType==SQLITE_IDXTYPE_IPK, that means pIndex is
///* a fake index used for the INTEGER PRIMARY KEY.
#[allow(unused_doc_comments)]
extern "C" fn where_loop_add_btree_index(p_builder_1: *mut WhereLoopBuilder,
    p_src_1: *mut SrcItem, p_probe_1: *mut Index, n_in_mul_1: LogEst) -> i32 {
    unsafe {
        let p_w_info: *const WhereInfo =
            unsafe { (*p_builder_1).p_w_info } as *const WhereInfo;
        /// WHERE analyze context
        let p_parse: *mut Parse = unsafe { (*p_w_info).p_parse };
        /// Parsing context
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        /// Database connection malloc context
        let mut p_new: *mut WhereLoop = core::ptr::null_mut();
        /// Template WhereLoop under construction
        let mut p_term: *mut WhereTerm = core::ptr::null_mut();
        /// A WhereTerm under consideration
        let mut op_mask: i32 = 0;
        /// Valid operators for constraints
        let mut scan: WhereScan = unsafe { core::mem::zeroed() };
        /// Iterator for WHERE terms
        let mut saved_prereq: Bitmask = 0 as Bitmask;
        /// Original value of pNew->prereq
        let mut saved_n_l_term: u16 = 0 as u16;
        /// Original value of pNew->nLTerm
        let mut saved_n_eq: u16 = 0 as u16;
        /// Original value of pNew->u.btree.nEq
        let mut saved_n_btm: u16 = 0 as u16;
        /// Original value of pNew->u.btree.nBtm
        let mut saved_n_top: u16 = 0 as u16;
        /// Original value of pNew->u.btree.nTop
        let mut saved_n_skip: u16 = 0 as u16;
        /// Original value of pNew->nSkip
        let mut saved_ws_flags: u32 = 0 as u32;
        /// Original value of pNew->wsFlags
        let mut saved_n_out: LogEst = 0 as LogEst;
        /// Original value of pNew->nOut
        let mut rc: i32 = 0;
        /// Return code
        let mut r_size: LogEst = 0 as LogEst;
        /// Number of rows in the table
        let mut r_log_size: LogEst = 0 as LogEst;
        /// Logarithm of table size
        let mut p_top: *mut WhereTerm = core::ptr::null_mut();
        let mut p_btm: *mut WhereTerm = core::ptr::null_mut();

        /// Top and bottom range constraints
        (p_new = unsafe { (*p_builder_1).p_new });
        { let _ = 0; };
        if unsafe { (*p_parse).n_err } != 0 {
            return unsafe { (*p_parse).rc };
        }
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_new).ws_flags } & 32 as u32 != 0 {
            op_mask = 2 << 57 - 54 | 2 << 56 - 54;
        } else {
            { let _ = 0; };
            op_mask =
                2 | 1 | 2 << 55 - 54 | 2 << 58 - 54 | 2 << 57 - 54 |
                            2 << 56 - 54 | 256 | 128;
        }
        if unsafe { (*p_probe_1).b_unordered() } != 0 {
            op_mask &=
                !(2 << 55 - 54 | 2 << 58 - 54 | 2 << 57 - 54 | 2 << 56 - 54);
        }
        { let _ = 0; };
        { let _ = 0; };
        saved_n_eq = unsafe { (*p_new).u.btree.n_eq };
        saved_n_btm = unsafe { (*p_new).u.btree.n_btm };
        saved_n_top = unsafe { (*p_new).u.btree.n_top };
        saved_n_skip = unsafe { (*p_new).n_skip };
        saved_n_l_term = unsafe { (*p_new).n_l_term };
        saved_ws_flags = unsafe { (*p_new).ws_flags };
        saved_prereq = unsafe { (*p_new).prereq };
        saved_n_out = unsafe { (*p_new).n_out };
        p_term =
            where_scan_init(&mut scan, unsafe { (*p_builder_1).p_wc },
                unsafe { (*p_src_1).i_cursor }, saved_n_eq as i32,
                op_mask as u32, p_probe_1 as *const Index);
        unsafe { (*p_new).r_setup = 0 as LogEst };
        r_size =
            unsafe {
                *unsafe { (*p_probe_1).ai_row_log_est.offset(0 as isize) }
            };
        r_log_size = est_log(r_size);
        {
            '__b56: loop {
                if !(rc == 0 && p_term != core::ptr::null_mut()) {
                    break '__b56;
                }
                '__c56: loop {
                    let e_op: u16 = unsafe { (*p_term).e_operator };
                    /// Shorthand for pTerm->eOperator
                    let mut r_cost_idx: LogEst = 0 as LogEst;
                    let mut n_out_unadjusted: LogEst = 0 as LogEst;
                    /// nOut before IN() and WHERE adjustments
                    let mut n_in: i32 = 0;
                    if (e_op as i32 == 256 ||
                                unsafe { (*p_term).wt_flags } as i32 & 128 != 0) &&
                            index_column_not_null(unsafe { &*p_probe_1 },
                                    saved_n_eq as i32) != 0 {
                        break '__c56;
                    }
                    if unsafe { (*p_term).prereq_right } &
                                unsafe { (*p_new).mask_self } != 0 {
                        break '__c56;
                    }
                    if unsafe { (*p_term).wt_flags } as i32 & 256 != 0 &&
                            unsafe { (*p_term).e_operator } as i32 == 2 << 57 - 54 {
                        break '__c56;
                    }
                    if unsafe { (*p_src_1).fg.jointype } as i32 & (8 | 64 | 16)
                                != 0 &&
                            (constraint_compatible_with_outer_join(unsafe { &*p_term },
                                            unsafe { &*p_src_1 }) == 0) as i32 != 0 {
                        break '__c56;
                    }
                    if unsafe { (*p_probe_1).on_error } as i32 != 0 &&
                            saved_n_eq as i32 ==
                                unsafe { (*p_probe_1).n_key_col } as i32 - 1 {
                        unsafe { (*p_builder_1).bld_flags1 |= 2 as u8 };
                    } else { unsafe { (*p_builder_1).bld_flags1 |= 1 as u8 }; }
                    unsafe { (*p_new).ws_flags = saved_ws_flags };
                    unsafe { (*p_new).u.btree.n_eq = saved_n_eq };
                    unsafe { (*p_new).u.btree.n_btm = saved_n_btm };
                    unsafe { (*p_new).u.btree.n_top = saved_n_top };
                    unsafe { (*p_new).n_l_term = saved_n_l_term };
                    if unsafe { (*p_new).n_l_term } as i32 >=
                                unsafe { (*p_new).n_l_slot } as i32 &&
                            where_loop_resize(db, unsafe { &mut *p_new },
                                    unsafe { (*p_new).n_l_term } as i32 + 1) != 0 {
                        break '__b56;
                    }
                    unsafe {
                        *unsafe {
                                    (*p_new).a_l_term.add({
                                                let __p = unsafe { &mut (*p_new).n_l_term };
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as usize)
                                } = p_term
                    };
                    unsafe {
                        (*p_new).prereq =
                            (saved_prereq | unsafe { (*p_term).prereq_right }) &
                                !unsafe { (*p_new).mask_self }
                    };
                    { let _ = 0; };
                    if e_op as i32 & 1 != 0 {
                        let p_expr: *mut Expr = unsafe { (*p_term).p_expr };
                        if unsafe { (*p_expr).flags } & 4096 as u32 != 0 as u32 {
                            /// "x IN (SELECT ...)":  TUNING: the SELECT returns 25 rows
                            let mut i: i32 = 0;
                            let mut b_redundant: i32 = 0;
                            n_in = 46;
                            { let _ = 0; };
                            {
                                i = 0;
                                '__b57: loop {
                                    if !(i < unsafe { (*p_new).n_l_term } as i32 - 1) {
                                        break '__b57;
                                    }
                                    '__c57: loop {
                                        if !(unsafe {
                                                                *unsafe { (*p_new).a_l_term.offset(i as isize) }
                                                            }).is_null() &&
                                                unsafe {
                                                        (*unsafe {
                                                                        *unsafe { (*p_new).a_l_term.offset(i as isize) }
                                                                    }).p_expr
                                                    } == p_expr {
                                            n_in = 0;
                                            if unsafe {
                                                        (*unsafe {
                                                                                *unsafe { (*p_new).a_l_term.offset(i as isize) }
                                                                            }).u.x.i_field
                                                    } == unsafe { (*p_term).u.x.i_field } {

                                                /// Detect when two or more columns of an index match the same 
                                                ///* column of a vector IN operater, and avoid adding the column
                                                ///* to the WhereLoop more than once.  See tag-20250707-01
                                                ///* in test/rowvalue.test
                                                (b_redundant = 1);
                                            }
                                        }
                                        break '__c57;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if b_redundant != 0 {
                                {
                                    let __p = unsafe { &mut (*p_new).n_l_term };
                                    let __t = *__p;
                                    *__p -= 1;
                                    __t
                                };
                                break '__c56;
                            }
                        } else if !(unsafe { (*p_expr).x.p_list }).is_null() &&
                                unsafe { (*unsafe { (*p_expr).x.p_list }).n_expr } != 0 {

                            /// "x IN (value, value, ...)"
                            (n_in =
                                unsafe {
                                        sqlite3_log_est(unsafe {
                                                    (*unsafe { (*p_expr).x.p_list }).n_expr
                                                } as u64)
                                    } as i32);
                        }
                        if unsafe { (*p_probe_1).has_stat1() } != 0 &&
                                r_log_size as i32 >= 10 {
                            let mut m: LogEst = 0 as LogEst;
                            let mut log_k: LogEst = 0 as LogEst;
                            let mut x: LogEst = 0 as LogEst;

                            /// Let:
                            ///*   N = the total number of rows in the table
                            ///*   K = the number of entries on the RHS of the IN operator
                            ///*   M = the number of rows in the table that match terms to the
                            ///*       to the left in the same index.  If the IN operator is on
                            ///*       the left-most index column, M==N.
                            ///*
                            ///* Given the definitions above, it is better to omit the IN operator
                            ///* from the index lookup and instead do a scan of the M elements,
                            ///* testing each scanned row against the IN operator separately, if:
                            ///*
                            ///*        M*log(K) < K*log(N)
                            ///*
                            ///* Our estimates for M, K, and N might be inaccurate, so we build in
                            ///* a safety margin of 2 (LogEst: 10) that favors using the IN operator
                            ///* with the index, as using an index has better worst-case behavior.
                            ///* If we do not have real sqlite_stat1 data, always prefer to use
                            ///* the index.  Do not bother with this optimization on very small
                            ///* tables (less than 2 rows) as it is pointless in that case.
                            (m =
                                unsafe {
                                    *unsafe {
                                            (*p_probe_1).ai_row_log_est.add(saved_n_eq as usize)
                                        }
                                });
                            log_k = est_log(n_in as LogEst);

                            /// TUNING      v-----  10 to bias toward indexed IN
                            (x =
                                (m as i32 + log_k as i32 + 10 - (n_in + r_log_size as i32))
                                    as LogEst);
                            if x as i32 >= 0
                                {} else if (n_in_mul_1 as i32) < 2 &&
                                    unsafe { (*db).db_opt_flags } & 131072 as u32 == 0 as u32 {
                                unsafe { (*p_new).ws_flags |= 1048576 as u32 };
                            } else { break '__c56; }
                        }
                        unsafe { (*p_new).ws_flags |= 4 as u32 };
                    } else if e_op as i32 & (2 | 128) != 0 {
                        let i_col: i32 =
                            unsafe {
                                    *unsafe { (*p_probe_1).ai_column.add(saved_n_eq as usize) }
                                } as i32;
                        unsafe { (*p_new).ws_flags |= 1 as u32 };
                        { let _ = 0; };
                        if i_col == -1 ||
                                i_col >= 0 && n_in_mul_1 as i32 == 0 &&
                                    saved_n_eq as i32 ==
                                        unsafe { (*p_probe_1).n_key_col } as i32 - 1 {
                            if i_col == -1 ||
                                        unsafe { (*p_probe_1).uniq_not_null() } != 0 ||
                                    unsafe { (*p_probe_1).n_key_col } as i32 == 1 &&
                                            unsafe { (*p_probe_1).on_error } != 0 &&
                                        e_op as i32 & 2 != 0 {
                                unsafe { (*p_new).ws_flags |= 4096 as u32 };
                            } else { unsafe { (*p_new).ws_flags |= 65536 as u32 }; }
                        }
                        if scan.i_equiv as i32 > 1 {
                            unsafe { (*p_new).ws_flags |= 2097152 as u32 };
                        }
                    } else if e_op as i32 & 256 != 0 {
                        unsafe { (*p_new).ws_flags |= 8 as u32 };
                    } else {
                        let n_vec_len: i32 =
                            where_range_vector_len(p_parse,
                                unsafe { (*p_src_1).i_cursor }, unsafe { &*p_probe_1 },
                                saved_n_eq as i32, unsafe { &*p_term });
                        if e_op as i32 & (2 << 55 - 54 | 2 << 58 - 54) != 0 {
                            unsafe { (*p_new).ws_flags |= (2 | 32) as u32 };
                            unsafe { (*p_new).u.btree.n_btm = n_vec_len as u16 };
                            p_btm = p_term;
                            p_top = core::ptr::null_mut();
                            if unsafe { (*p_term).wt_flags } as i32 & 256 != 0 {

                                /// Range constraints that come from the LIKE optimization are
                                ///* always used in pairs.
                                (p_top = unsafe { p_term.offset(1 as isize) });
                                { let _ = 0; };
                                { let _ = 0; };
                                { let _ = 0; };
                                if where_loop_resize(db, unsafe { &mut *p_new },
                                            unsafe { (*p_new).n_l_term } as i32 + 1) != 0 {
                                    break '__b56;
                                }

                                /// OOM
                                unsafe {
                                    *unsafe {
                                                (*p_new).a_l_term.add({
                                                            let __p = unsafe { &mut (*p_new).n_l_term };
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as usize)
                                            } = p_top
                                };
                                unsafe { (*p_new).ws_flags |= 16 as u32 };
                                unsafe { (*p_new).u.btree.n_top = 1 as u16 };
                            }
                        } else {
                            { let _ = 0; };
                            unsafe { (*p_new).ws_flags |= (2 | 16) as u32 };
                            unsafe { (*p_new).u.btree.n_top = n_vec_len as u16 };
                            p_top = p_term;
                            p_btm =
                                if unsafe { (*p_new).ws_flags } & 32 as u32 != 0 as u32 {
                                    unsafe {
                                        *unsafe {
                                                (*p_new).a_l_term.offset((unsafe { (*p_new).n_l_term } as
                                                                i32 - 2) as isize)
                                            }
                                    }
                                } else { core::ptr::null_mut() };
                        }
                    }

                    /// At this point pNew->nOut is set to the number of rows expected to
                    ///* be visited by the index scan before considering term pTerm, or the
                    ///* values of nIn and nInMul. In other words, assuming that all
                    ///* "x IN(...)" terms are replaced with "x = ?". This block updates
                    ///* the value of pNew->nOut to account for pTerm (but not nIn/nInMul).
                    { let _ = 0; };
                    if unsafe { (*p_new).ws_flags } & 2 as u32 != 0 {

                        /// Adjust nOut using stat4 data. Or, if there is no stat4
                        ///* data, using some other estimate.
                        where_range_scan_est(p_parse as *const Parse,
                            p_builder_1 as *const WhereLoopBuilder, p_btm, p_top,
                            unsafe { &mut *p_new });
                    } else {
                        let n_eq: i32 =
                            {
                                    let __p = unsafe { &mut (*p_new).u.btree.n_eq };
                                    *__p += 1;
                                    *__p
                                } as i32;
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_term).truth_prob } as i32 <= 0 &&
                                unsafe {
                                            *unsafe { (*p_probe_1).ai_column.add(saved_n_eq as usize) }
                                        } as i32 >= 0 {
                            { let _ = 0; };
                            unsafe {
                                (*p_new).n_out +=
                                    unsafe { (*p_term).truth_prob } as i32 as LogEst
                            };
                            unsafe { (*p_new).n_out -= n_in as LogEst };
                        } else {
                            {
                                unsafe {
                                    (*p_new).n_out +=
                                        (unsafe {
                                                        *unsafe {
                                                                (*p_probe_1).ai_row_log_est.offset(n_eq as isize)
                                                            }
                                                    } as i32 -
                                                unsafe {
                                                        *unsafe {
                                                                (*p_probe_1).ai_row_log_est.offset((n_eq - 1) as isize)
                                                            }
                                                    } as i32) as LogEst
                                };
                                if e_op as i32 & 256 != 0 {

                                    /// TUNING: If there is no likelihood() value, assume that a
                                    ///* "col IS NULL" expression matches twice as many rows
                                    ///* as (col=?).
                                    unsafe { (*p_new).n_out += 10 as LogEst };
                                }
                            }
                        }
                    }

                    /// Set rCostIdx to the estimated cost of visiting selected rows in the
                    ///* index.  The estimate is the sum of two values:
                    ///*   1.  The cost of doing one search-by-key to find the first matching
                    ///*       entry
                    ///*   2.  Stepping forward in the index pNew->nOut times to find all
                    ///*       additional matching entries.
                    { let _ = 0; };
                    if unsafe { (*p_probe_1).idx_type() } as i32 == 3 {

                        /// The pProbe->szIdxRow is low for an IPK table since the interior
                        ///* pages are small.  Thus szIdxRow gives a good estimate of seek cost.
                        ///* But the leaf pages are full-size, so pProbe->szIdxRow would badly
                        ///* under-estimate the scanning cost.
                        (r_cost_idx =
                            (unsafe { (*p_new).n_out } as i32 + 16) as LogEst);
                    } else {
                        r_cost_idx =
                            (unsafe { (*p_new).n_out } as i32 + 1 +
                                    15 * unsafe { (*p_probe_1).sz_idx_row } as i32 /
                                        unsafe { (*unsafe { (*p_src_1).p_s_tab }).sz_tab_row } as
                                            i32) as LogEst;
                    }
                    r_cost_idx =
                        unsafe { sqlite3_log_est_add(r_log_size, r_cost_idx) };

                    /// Estimate the cost of running the loop.  If all data is coming
                    ///* from the index, then this is just the cost of doing the index
                    ///* lookup and scan.  But if some data is coming out of the main table,
                    ///* we also have to add in the cost of doing pNew->nOut searches to
                    ///* locate the row in the main table that corresponds to the index entry.
                    unsafe { (*p_new).r_run = r_cost_idx };
                    if unsafe { (*p_new).ws_flags } &
                                (64 | 256 | 67108864) as u32 == 0 as u32 {
                        unsafe {
                            (*p_new).r_run =
                                unsafe {
                                    sqlite3_log_est_add(unsafe { (*p_new).r_run },
                                        (unsafe { (*p_new).n_out } as i32 + 16) as LogEst)
                                }
                        };
                    }
                    n_out_unadjusted = unsafe { (*p_new).n_out };
                    unsafe {
                        (*p_new).r_run += (n_in_mul_1 as i32 + n_in) as LogEst
                    };
                    unsafe {
                        (*p_new).n_out += (n_in_mul_1 as i32 + n_in) as LogEst
                    };
                    where_loop_output_adjust(unsafe {
                            &*unsafe { (*p_builder_1).p_wc }
                        }, unsafe { &mut *p_new }, r_size);
                    if unsafe { (*p_src_1).fg.from_exists() } != 0 {
                        unsafe { (*p_new).n_out = 0 as LogEst };
                    }
                    rc = where_loop_insert(unsafe { &mut *p_builder_1 }, p_new);
                    if unsafe { (*p_new).ws_flags } & 2 as u32 != 0 {
                        unsafe { (*p_new).n_out = saved_n_out };
                    } else { unsafe { (*p_new).n_out = n_out_unadjusted }; }
                    if unsafe { (*p_new).ws_flags } & 16 as u32 == 0 as u32 &&
                                (unsafe { (*p_new).u.btree.n_eq } as i32) <
                                    unsafe { (*p_probe_1).n_column } as i32 &&
                            ((unsafe { (*p_new).u.btree.n_eq } as i32) <
                                    unsafe { (*p_probe_1).n_key_col } as i32 ||
                                unsafe { (*p_probe_1).idx_type() } as i32 != 2) {
                        if unsafe { (*p_new).u.btree.n_eq } as i32 > 3 {
                            unsafe { sqlite3_progress_check(p_parse) };
                        }
                        where_loop_add_btree_index(p_builder_1, p_src_1, p_probe_1,
                            (n_in_mul_1 as i32 + n_in) as LogEst);
                    }
                    unsafe { (*p_new).n_out = saved_n_out };
                    break '__c56;
                }
                p_term = where_scan_next(&mut scan);
            }
        }
        unsafe { (*p_new).prereq = saved_prereq };
        unsafe { (*p_new).u.btree.n_eq = saved_n_eq };
        unsafe { (*p_new).u.btree.n_btm = saved_n_btm };
        unsafe { (*p_new).u.btree.n_top = saved_n_top };
        unsafe { (*p_new).n_skip = saved_n_skip };
        unsafe { (*p_new).ws_flags = saved_ws_flags };
        unsafe { (*p_new).n_out = saved_n_out };
        unsafe { (*p_new).n_l_term = saved_n_l_term };

        /// Consider using a skip-scan if there are no WHERE clause constraints
        ///* available for the left-most terms of the index, and if the average
        ///* number of repeats in the left-most terms is at least 18.
        ///*
        ///* The magic number 18 is selected on the basis that scanning 17 rows
        ///* is almost always quicker than an index seek (even though if the index
        ///* contains fewer than 2^17 rows we assume otherwise in other parts of
        ///* the code). And, even if it is not, it should not be too much slower.
        ///* On the other hand, the extra seeks could end up being significantly
        ///* more expensive.
        { let _ = 0; };
        if saved_n_eq as i32 == saved_n_skip as i32 &&
                                            saved_n_eq as i32 + 1 <
                                                unsafe { (*p_probe_1).n_key_col } as i32 &&
                                        saved_n_eq as i32 == unsafe { (*p_new).n_l_term } as i32 &&
                                    unsafe { (*p_probe_1).no_skip_scan() } as i32 == 0 &&
                                unsafe { (*p_probe_1).has_stat1() } as i32 != 0 &&
                            unsafe { (*db).db_opt_flags } & 16384 as u32 == 0 as u32 &&
                        unsafe {
                                    *unsafe {
                                            (*p_probe_1).ai_row_log_est.offset((saved_n_eq as i32 + 1)
                                                    as isize)
                                        }
                                } as i32 >= 42 &&
                    unsafe { (*p_src_1).fg.from_exists() } as i32 == 0 &&
                {
                        rc =
                            where_loop_resize(db, unsafe { &mut *p_new },
                                unsafe { (*p_new).n_l_term } as i32 + 1);
                        rc
                    } == 0 {
            let mut n_iter: LogEst = 0 as LogEst;
            {
                let __p = unsafe { &mut (*p_new).u.btree.n_eq };
                let __t = *__p;
                *__p += 1;
                __t
            };
            {
                let __p = unsafe { &mut (*p_new).n_skip };
                let __t = *__p;
                *__p += 1;
                __t
            };
            unsafe {
                *unsafe {
                            (*p_new).a_l_term.add({
                                        let __p = unsafe { &mut (*p_new).n_l_term };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as usize)
                        } = core::ptr::null_mut()
            };
            unsafe { (*p_new).ws_flags |= 32768 as u32 };
            n_iter =
                (unsafe {
                                *unsafe {
                                        (*p_probe_1).ai_row_log_est.add(saved_n_eq as usize)
                                    }
                            } as i32 -
                        unsafe {
                                *unsafe {
                                        (*p_probe_1).ai_row_log_est.offset((saved_n_eq as i32 + 1)
                                                as isize)
                                    }
                            } as i32) as LogEst;
            unsafe { (*p_new).n_out -= n_iter as i32 as LogEst };

            /// TUNING:  Because uncertainties in the estimates for skip-scan queries,
            ///* add a 1.375 fudge factor to make skip-scan slightly less likely.
            (n_iter += 5 as LogEst);
            where_loop_add_btree_index(p_builder_1, p_src_1, p_probe_1,
                (n_iter as i32 + n_in_mul_1 as i32) as LogEst);
            unsafe { (*p_new).n_out = saved_n_out };
            unsafe { (*p_new).u.btree.n_eq = saved_n_eq };
            unsafe { (*p_new).n_skip = saved_n_skip };
            unsafe { (*p_new).ws_flags = saved_ws_flags };
        }
        return rc;
    }
}

///* Add all WhereLoop objects for a single table of the join where the table
///* is identified by pBuilder->pNew->iTab.  That table is guaranteed to be
///* a b-tree table, not a virtual table.
///*
///* The costs (WhereLoop.rRun) of the b-tree loops added by this function
///* are calculated as follows:
///*
///* For a full scan, assuming the table (or index) contains nRow rows:
///*
///*     cost = nRow * 3.0                    // full-table scan
///*     cost = nRow * K                      // scan of covering index
///*     cost = nRow * (K+3.0)                // scan of non-covering index
///*
///* where K is a value between 1.1 and 3.0 set based on the relative
///* estimated average size of the index and table records.
///*
///* For an index scan, where nVisit is the number of index rows visited
///* by the scan, and nSeek is the number of seek operations required on
///* the index b-tree:
///*
///*     cost = nSeek * (log(nRow) + K * nVisit)          // covering index
///*     cost = nSeek * (log(nRow) + (K+3.0) * nVisit)    // non-covering index
///*
///* Normally, nSeek is 1. nSeek values greater than 1 come about if the
///* WHERE clause includes "x IN (....)" terms used in place of "x=?". Or when
///* implicit "x IN (SELECT x FROM tbl)" terms are added for skip-scans.
///*
///* The estimated values (nRow, nVisit, nSeek) often contain a large amount
///* of uncertainty.  For this reason, scoring is designed to pick plans that
///* "do the least harm" if the estimates are inaccurate.  For example, a
///* log(nRow) factor is omitted from a non-covering index scan in order to
///* bias the scoring in favor of using an index, since the worst-case
///* performance of using an index is far better than the worst-case performance
///* of a full table scan.
#[allow(unused_doc_comments)]
extern "C" fn where_loop_add_btree(p_builder_1: *mut WhereLoopBuilder,
    m_prereq_1: Bitmask) -> i32 {
    unsafe {
        unsafe {
            let mut p_w_info: *mut WhereInfo = core::ptr::null_mut();
            /// WHERE analysis context
            let mut p_probe: *mut Index = core::ptr::null_mut();
            /// An index we are evaluating
            let mut s_pk: Index = unsafe { core::mem::zeroed() };
            /// A fake index object for the primary key
            let mut ai_row_est_pk: [i16; 2] = [0; 2];
            /// The aiRowLogEst[] value for the sPk index
            let mut ai_column_pk: i16 = -1 as i16;
            /// The aColumn[] value for the sPk index
            let mut p_tab_list: *const SrcList = core::ptr::null();
            /// The FROM clause
            let mut p_src: *mut SrcItem = core::ptr::null_mut();
            /// The FROM clause btree term to add
            let mut p_new: *mut WhereLoop = core::ptr::null_mut();
            /// Template WhereLoop object
            let mut rc: i32 = 0;
            /// Return code
            let mut i_sort_idx: i32 = 1;
            /// Index number
            let mut b: i32 = 0;
            /// A boolean value
            let mut r_size: LogEst = 0 as LogEst;
            /// number of rows in the table
            let mut p_wc: *mut WhereClause = core::ptr::null_mut();
            /// The parsed WHERE clause
            let mut p_tab: *mut Table = core::ptr::null_mut();

            /// Table being queried
            (p_new = unsafe { (*p_builder_1).p_new });
            p_w_info = unsafe { (*p_builder_1).p_w_info };
            p_tab_list = unsafe { (*p_w_info).p_tab_list };
            p_src =
                unsafe {
                    (unsafe { (*p_tab_list).a.as_ptr() } as
                            *mut SrcItem).add(unsafe { (*p_new).i_tab } as usize)
                };
            p_tab = unsafe { (*p_src).p_s_tab };
            p_wc = unsafe { (*p_builder_1).p_wc };
            { let _ = 0; };
            if unsafe { (*p_src).fg.is_indexed_by() } != 0 {
                { let _ = 0; };

                /// An INDEXED BY clause specifies a particular index to use
                (p_probe = unsafe { (*p_src).u2.p_ib_index });
            } else if !(unsafe { (*p_tab).tab_flags } & 128 as u32 ==
                                0 as u32) as i32 != 0 {
                p_probe = unsafe { (*p_tab).p_index };
            } else {
                /// There is no INDEXED BY clause.  Create a fake Index object in local
                ///* variable sPk to represent the rowid primary key index.  Make this
                ///* fake index the first in a chain of Index objects with all of the real
                ///* indices to follow
                let mut p_first: *mut Index = core::ptr::null_mut();

                /// First of real indices on the table
                unsafe {
                    memset(&raw mut s_pk as *mut (), 0,
                        core::mem::size_of::<Index>() as u64)
                };
                s_pk.n_key_col = 1 as u16;
                s_pk.n_column = 1 as u16;
                s_pk.ai_column = &mut ai_column_pk;
                s_pk.ai_row_log_est =
                    &raw mut ai_row_est_pk[0 as usize] as *mut LogEst;
                s_pk.on_error = 5 as u8;
                s_pk.p_table = p_tab;
                s_pk.sz_idx_row = 3 as LogEst;

                /// TUNING: Interior rows of IPK table are very small
                s_pk.set_idx_type(3 as u32 as u32);
                ai_row_est_pk[0 as usize] = unsafe { (*p_tab).n_row_log_est };
                ai_row_est_pk[1 as usize] = 0 as LogEst;
                p_first = unsafe { (*unsafe { (*p_src).p_s_tab }).p_index };
                if unsafe { (*p_src).fg.not_indexed() } as i32 == 0 {

                    /// The real indices of the table are only considered if the
                    ///* NOT INDEXED qualifier is omitted from the FROM clause
                    (s_pk.p_next = p_first);
                }
                p_probe = &mut s_pk;
            }
            r_size = unsafe { (*p_tab).n_row_log_est };
            if (unsafe { (*p_builder_1).p_or_set }).is_null() as i32 != 0 &&
                                            unsafe { (*p_w_info).wctrl_flags } as i32 & (4096 | 32) == 0
                                        &&
                                        unsafe {
                                                    (*unsafe { (*unsafe { (*p_w_info).p_parse }).db }).flags
                                                } & 32768 as u64 != 0 as u64 &&
                                    (unsafe { (*p_src).fg.is_indexed_by() } == 0) as i32 != 0 &&
                                (unsafe { (*p_src).fg.not_indexed() } == 0) as i32 != 0 &&
                            (unsafe { (*p_src).fg.is_correlated() } == 0) as i32 != 0 &&
                        (unsafe { (*p_src).fg.is_recursive() } == 0) as i32 != 0 &&
                    unsafe { (*p_src).fg.jointype } as i32 & 16 == 0 {
                /// Generate auto-index WhereLoops
                let mut r_log_size: LogEst = 0 as LogEst;
                /// Logarithm of the number of rows in the table
                let mut p_term: *mut WhereTerm = core::ptr::null_mut();
                let p_wc_end: *mut WhereTerm =
                    unsafe {
                        unsafe {
                            (*p_wc).a.offset(unsafe { (*p_wc).n_term } as isize)
                        }
                    };
                r_log_size = est_log(r_size);
                {
                    p_term = unsafe { (*p_wc).a };
                    '__b58: loop {
                        if !(rc == 0 && p_term < p_wc_end) { break '__b58; }
                        '__c58: loop {
                            if unsafe { (*p_term).prereq_right } &
                                        unsafe { (*p_new).mask_self } != 0 {
                                break '__c58;
                            }
                            if term_can_drive_index(p_term as *const WhereTerm,
                                        p_src as *const SrcItem, 0 as Bitmask) != 0 {
                                unsafe { (*p_new).u.btree.n_eq = 1 as u16 };
                                unsafe { (*p_new).n_skip = 0 as u16 };
                                unsafe { (*p_new).u.btree.p_index = core::ptr::null_mut() };
                                unsafe { (*p_new).n_l_term = 1 as u16 };
                                unsafe {
                                    *unsafe { (*p_new).a_l_term.offset(0 as isize) } = p_term
                                };

                                /// TUNING: One-time cost for computing the automatic index is
                                ///* estimated to be X*N*log2(N) where N is the number of rows in
                                ///* the table being indexed and where X is 7 (LogEst=28) for normal
                                ///* tables or 0.5 (LogEst=-10) for views and subqueries.  The value
                                ///* of X is smaller for views and subqueries so that the query planner
                                ///* will be more aggressive about generating automatic indexes for
                                ///* those objects, since there is no opportunity to add schema
                                ///* indexes on subqueries and views.
                                unsafe {
                                    (*p_new).r_setup =
                                        (r_log_size as i32 + r_size as i32) as LogEst
                                };
                                if !(unsafe { (*p_tab).e_tab_type } as i32 == 2) as i32 != 0
                                        && unsafe { (*p_tab).tab_flags } & 16384 as u32 == 0 as u32
                                    {
                                    unsafe { (*p_new).r_setup += 28 as LogEst };
                                } else { unsafe { (*p_new).r_setup -= 25 as LogEst }; }
                                if (unsafe { (*p_new).r_setup } as i32) < 0 {
                                    unsafe { (*p_new).r_setup = 0 as LogEst };
                                }

                                /// TUNING: Each index lookup yields 20 rows in the table.  This
                                ///* is more than the usual guess of 10 rows, since we have no way
                                ///* of knowing how selective the index will ultimately be.  It would
                                ///* not be unreasonable to make this value much larger.
                                unsafe { (*p_new).n_out = 43 as LogEst };
                                { let _ = 0; };
                                unsafe {
                                    (*p_new).r_run =
                                        unsafe {
                                            sqlite3_log_est_add(r_log_size, unsafe { (*p_new).n_out })
                                        }
                                };
                                unsafe { (*p_new).ws_flags = 16384 as u32 };
                                unsafe {
                                    (*p_new).prereq =
                                        m_prereq_1 | unsafe { (*p_term).prereq_right }
                                };
                                rc = where_loop_insert(unsafe { &mut *p_builder_1 }, p_new);
                            }
                            break '__c58;
                        }
                        {
                            let __p = &mut p_term;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                }
            }
            {
                '__b59: loop {
                    if !(rc == 0 && !(p_probe).is_null()) { break '__b59; }
                    '__c59: loop {
                        if unsafe { (*p_probe).p_part_idx_where } !=
                                    core::ptr::null_mut() &&
                                (where_usable_partial_index(unsafe { (*p_src).i_cursor },
                                                unsafe { (*p_src).fg.jointype }, p_wc,
                                                unsafe { (*p_probe).p_part_idx_where } as *const Expr) == 0)
                                        as i32 != 0 {

                            /// See ticket [98d973b8f5]
                            break '__c59;
                        }
                        if unsafe { (*p_probe).b_no_query() } != 0 { break '__c59; }
                        r_size =
                            unsafe {
                                *unsafe { (*p_probe).ai_row_log_est.offset(0 as isize) }
                            };
                        unsafe { (*p_new).u.btree.n_eq = 0 as u16 };
                        unsafe { (*p_new).u.btree.n_btm = 0 as u16 };
                        unsafe { (*p_new).u.btree.n_top = 0 as u16 };
                        unsafe { (*p_new).u.btree.n_distinct_col = 0 as u16 };
                        unsafe { (*p_new).n_skip = 0 as u16 };
                        unsafe { (*p_new).n_l_term = 0 as u16 };
                        unsafe { (*p_new).i_sort_idx = 0 as u8 };
                        unsafe { (*p_new).r_setup = 0 as LogEst };
                        unsafe { (*p_new).prereq = m_prereq_1 };
                        unsafe { (*p_new).n_out = r_size };
                        unsafe { (*p_new).u.btree.p_index = p_probe };
                        unsafe {
                            (*p_new).u.btree.p_order_by = core::ptr::null_mut()
                        };
                        b =
                            index_might_help_with_order_by(unsafe { &*p_builder_1 },
                                unsafe { &*p_probe }, unsafe { (*p_src).i_cursor });

                        /// The ONEPASS_DESIRED flags never occurs together with ORDER BY
                        { let _ = 0; };
                        if unsafe { (*p_probe).idx_type() } as i32 == 3 {

                            /// Integer primary key index
                            unsafe { (*p_new).ws_flags = 256 as u32 };

                            /// Full table scan
                            unsafe {
                                (*p_new).i_sort_idx =
                                    if b != 0 { i_sort_idx } else { 0 } as u8
                            };

                            /// TUNING: Cost of full table scan is 3.0*N.  The 3.0 factor is an
                            ///* extra cost designed to discourage the use of full table scans,
                            ///* since index lookups have better worst-case performance if our
                            ///* stat guesses are wrong.  Reduce the 3.0 penalty slightly
                            ///* (to 2.75) if we have valid STAT4 information for the table.
                            ///* At 2.75, a full table scan is preferred over using an index on
                            ///* a column with just two distinct values where each value has about
                            ///* an equal number of appearances.  Without STAT4 data, we still want
                            ///* to use an index in that case, since the constraint might be for
                            ///* the scarcer of the two values, and in that case an index lookup is
                            ///* better.
                            unsafe { (*p_new).r_run = (r_size as i32 + 16) as LogEst };
                            where_loop_output_adjust(unsafe { &*p_wc },
                                unsafe { &mut *p_new }, r_size);
                            if unsafe { (*p_src).fg.is_subquery() } != 0 {
                                if unsafe { (*p_src).fg.via_coroutine() } != 0 {
                                    unsafe { (*p_new).ws_flags |= 33554432 as u32 };
                                }
                                if unsafe {
                                                (*unsafe {
                                                                (*unsafe { (*p_src).u4.p_subq }).p_select
                                                            }).sel_flags
                                            } & 8192 as u32 == 0 as u32 {
                                    unsafe {
                                        (*p_new).u.btree.p_order_by =
                                            unsafe {
                                                (*unsafe {
                                                                (*unsafe { (*p_src).u4.p_subq }).p_select
                                                            }).p_order_by
                                            }
                                    };
                                }
                            } else if unsafe { (*p_src).fg.from_exists() } != 0 {
                                unsafe { (*p_new).n_out = 0 as LogEst };
                            }
                            rc = where_loop_insert(unsafe { &mut *p_builder_1 }, p_new);
                            unsafe { (*p_new).n_out = r_size };
                            if rc != 0 { break '__b59; }
                        } else {
                            let mut m: Bitmask = 0 as Bitmask;
                            if unsafe { (*p_probe).is_covering() } != 0 {
                                m = 0 as Bitmask;
                                unsafe { (*p_new).ws_flags = (64 | 512) as u32 };
                            } else {
                                m =
                                    unsafe { (*p_src).col_used } &
                                        unsafe { (*p_probe).col_not_idxed };
                                if !(unsafe { (*p_probe).p_part_idx_where }).is_null() {
                                    where_part_idx_expr(unsafe { (*p_w_info).p_parse }, p_probe,
                                        unsafe { (*p_probe).p_part_idx_where } as *const Expr,
                                        &mut m, 0, core::ptr::null_mut());
                                }
                                unsafe { (*p_new).ws_flags = 512 as u32 };
                                if m ==
                                            (1 as Bitmask) <<
                                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                                    1 ||
                                        unsafe { (*p_probe).b_has_expr() } != 0 &&
                                                (unsafe { (*p_probe).b_has_v_col() } == 0) as i32 != 0 &&
                                            m != 0 as u64 {
                                    let is_cov: u32 =
                                        where_is_covering_index(unsafe { &*p_w_info }, p_probe,
                                            unsafe { (*p_src).i_cursor });
                                    if is_cov == 0 as u32 {
                                        { let _ = 0; };
                                    } else {
                                        m = 0 as Bitmask;
                                        unsafe { (*p_new).ws_flags |= is_cov };
                                        if is_cov & 64 as u32 != 0 {} else { { let _ = 0; }; }
                                    }
                                } else if m == 0 as u64 &&
                                        (unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 ||
                                                unsafe { (*p_w_info).p_select } != core::ptr::null_mut() ||
                                            unsafe { sqlite3_fault_sim(700) } != 0) {
                                    unsafe { (*p_new).ws_flags = (64 | 512) as u32 };
                                }
                            }
                            if b != 0 ||
                                                !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32) as
                                                        i32 != 0 ||
                                            unsafe { (*p_probe).p_part_idx_where } !=
                                                core::ptr::null_mut() ||
                                        unsafe { (*p_src).fg.is_indexed_by() } != 0 ||
                                    m == 0 as u64 &&
                                                        unsafe { (*p_probe).b_unordered() } as i32 == 0 &&
                                                    (unsafe { (*p_probe).sz_idx_row } as i32) <
                                                        unsafe { (*p_tab).sz_tab_row } as i32 &&
                                                unsafe { (*p_w_info).wctrl_flags } as i32 & 4 == 0 &&
                                            sqlite3Config.b_use_cis != 0 &&
                                        unsafe {
                                                    (*unsafe {
                                                                    (*unsafe { (*p_w_info).p_parse }).db
                                                                }).db_opt_flags
                                                } & 32 as u32 == 0 as u32 {
                                unsafe {
                                    (*p_new).i_sort_idx =
                                        if b != 0 { i_sort_idx } else { 0 } as u8
                                };

                                /// The cost of visiting the index rows is N*K, where K is
                                ///* between 1.1 and 3.0, depending on the relative sizes of the
                                ///* index and table rows.
                                unsafe {
                                    (*p_new).r_run =
                                        (r_size as i32 + 1 +
                                                15 * unsafe { (*p_probe).sz_idx_row } as i32 /
                                                    unsafe { (*p_tab).sz_tab_row } as i32) as LogEst
                                };
                                if m != 0 as u64 {
                                    /// If this is a non-covering index scan, add in the cost of
                                    ///* doing table lookups.  The cost will be 3x the number of
                                    ///* lookups.  Take into account WHERE clause terms that can be
                                    ///* satisfied using just the index, and that do not require a
                                    ///* table lookup.
                                    let mut n_lookup: LogEst = (r_size as i32 + 16) as LogEst;
                                    /// Base cost:  N*3
                                    let mut ii: i32 = 0;
                                    let i_cur: i32 = unsafe { (*p_src).i_cursor };
                                    let p_wc2: *const WhereClause =
                                        unsafe { &raw mut (*p_w_info).s_wc } as *const WhereClause;
                                    {
                                        ii = 0;
                                        '__b60: loop {
                                            if !(ii < unsafe { (*p_wc2).n_term }) { break '__b60; }
                                            '__c60: loop {
                                                let p_term_1: *const WhereTerm =
                                                    unsafe {
                                                            &raw mut *unsafe { (*p_wc2).a.offset(ii as isize) }
                                                        } as *const WhereTerm;
                                                if (unsafe {
                                                                    sqlite3_expr_covered_by_index(unsafe { (*p_term_1).p_expr },
                                                                        i_cur, p_probe)
                                                                } == 0) as i32 != 0 {
                                                    break '__b60;
                                                }
                                                if unsafe { (*p_term_1).truth_prob } as i32 <= 0 {
                                                    n_lookup +=
                                                        unsafe { (*p_term_1).truth_prob } as i32 as LogEst;
                                                } else {
                                                    { let __p = &mut n_lookup; let __t = *__p; *__p -= 1; __t };
                                                    if unsafe { (*p_term_1).e_operator } as i32 & (2 | 128) != 0
                                                        {
                                                        n_lookup -= 19 as LogEst;
                                                    }
                                                }
                                                break '__c60;
                                            }
                                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    unsafe {
                                        (*p_new).r_run =
                                            unsafe {
                                                sqlite3_log_est_add(unsafe { (*p_new).r_run }, n_lookup)
                                            }
                                    };
                                }
                                where_loop_output_adjust(unsafe { &*p_wc },
                                    unsafe { &mut *p_new }, r_size);
                                if unsafe { (*p_src).fg.jointype } as i32 & 16 != 0 &&
                                        !(unsafe { (*p_probe).a_col_expr }).is_null()
                                    {} else {
                                    if unsafe { (*p_src).fg.from_exists() } != 0 {
                                        unsafe { (*p_new).n_out = 0 as LogEst };
                                    }
                                    rc = where_loop_insert(unsafe { &mut *p_builder_1 }, p_new);
                                }
                                unsafe { (*p_new).n_out = r_size };
                                if rc != 0 { break '__b59; }
                            }
                        }
                        unsafe { (*p_builder_1).bld_flags1 = 0 as u8 };
                        rc =
                            where_loop_add_btree_index(p_builder_1, p_src, p_probe,
                                0 as LogEst);
                        if unsafe { (*p_builder_1).bld_flags1 } as i32 == 1 {

                            /// If a non-unique index is used, or if a prefix of the key for
                            ///* unique index is used (making the index functionally non-unique)
                            ///* then the sqlite_stat1 data becomes important for scoring the
                            ///* plan
                            unsafe { (*p_tab).tab_flags |= 256 as u32 };
                        }
                        break '__c59;
                    }
                    {
                        p_probe =
                            if unsafe { (*p_src).fg.is_indexed_by() } != 0 {
                                core::ptr::null_mut()
                            } else { unsafe { (*p_probe).p_next } };
                        {
                            let __p = &mut i_sort_idx;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        }
                    };
                }
            }
            return rc;
        }
    }
}

///* Move the content of pSrc into pDest
extern "C" fn where_or_move(p_dest_1: &mut WhereOrSet, p_src_1: &WhereOrSet)
    -> () {
    unsafe {
        (*p_dest_1).n = (*p_src_1).n;
        unsafe {
            memcpy(&raw mut (*p_dest_1).a[0 as usize] as *mut WhereOrCost as
                    *mut (),
                &raw const (*p_src_1).a[0 as usize] as *mut WhereOrCost as
                    *const (),
                (*p_dest_1).n as u64 *
                    core::mem::size_of::<WhereOrCost>() as u64)
        };
    }
}

///* Add WhereLoop entries to handle OR terms.  This works for either
///* btrees or virtual tables.
#[allow(unused_doc_comments)]
extern "C" fn where_loop_add_or(p_builder_1: *mut WhereLoopBuilder,
    m_prereq_1: Bitmask, m_unusable_1: Bitmask) -> i32 {
    unsafe {
        let p_w_info: *const WhereInfo =
            unsafe { (*p_builder_1).p_w_info } as *const WhereInfo;
        let mut p_wc: *mut WhereClause = core::ptr::null_mut();
        let mut p_new: *mut WhereLoop = core::ptr::null_mut();
        let mut p_term: *mut WhereTerm = core::ptr::null_mut();
        let mut p_wc_end: *mut WhereTerm = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut i_cur: i32 = 0;
        let mut temp_wc: WhereClause = unsafe { core::mem::zeroed() };
        let mut s_sub_build: WhereLoopBuilder =
            unsafe { core::mem::zeroed() };
        let mut s_sum: WhereOrSet = unsafe { core::mem::zeroed() };
        let mut s_cur: WhereOrSet = unsafe { core::mem::zeroed() };
        let mut p_item: *const SrcItem = core::ptr::null();
        p_wc = unsafe { (*p_builder_1).p_wc };
        p_wc_end =
            unsafe {
                unsafe {
                    (*p_wc).a.offset(unsafe { (*p_wc).n_term } as isize)
                }
            };
        p_new = unsafe { (*p_builder_1).p_new };
        unsafe {
            memset(&raw mut s_sum as *mut (), 0,
                core::mem::size_of::<WhereOrSet>() as u64)
        };
        p_item =
            unsafe {
                (unsafe { (*unsafe { (*p_w_info).p_tab_list }).a.as_ptr() } as
                        *mut SrcItem).add(unsafe { (*p_new).i_tab } as usize)
            };
        i_cur = unsafe { (*p_item).i_cursor };
        if unsafe { (*p_item).fg.jointype } as i32 & 16 != 0 { return 0; }
        {
            p_term = unsafe { (*p_wc).a };
            '__b61: loop {
                if !(p_term < p_wc_end && rc == 0) { break '__b61; }
                '__c61: loop {
                    if unsafe { (*p_term).e_operator } as i32 & 512 != 0 &&
                            unsafe { (*unsafe { (*p_term).u.p_or_info }).indexable } &
                                    unsafe { (*p_new).mask_self } != 0 as u64 {
                        let p_or_wc: *const WhereClause =
                            unsafe { &raw mut (*unsafe { (*p_term).u.p_or_info }).wc }
                                as *const WhereClause;
                        let p_or_wc_end: *mut WhereTerm =
                            unsafe {
                                &mut *unsafe {
                                            (*p_or_wc).a.offset(unsafe { (*p_or_wc).n_term } as isize)
                                        }
                            };
                        let mut p_or_term: *mut WhereTerm = core::ptr::null_mut();
                        let mut once: i32 = 1;
                        let mut i: i32 = 0;
                        let mut j: i32 = 0;
                        s_sub_build = unsafe { core::ptr::read(p_builder_1) };
                        s_sub_build.p_or_set = &mut s_cur;
                        {
                            p_or_term = unsafe { (*p_or_wc).a };
                            '__b62: loop {
                                if !(p_or_term < p_or_wc_end) { break '__b62; }
                                '__c62: loop {
                                    if unsafe { (*p_or_term).e_operator } as i32 & 1024 != 0 {
                                        s_sub_build.p_wc =
                                            unsafe { &mut (*unsafe { (*p_or_term).u.p_and_info }).wc };
                                    } else if unsafe { (*p_or_term).left_cursor } == i_cur {
                                        temp_wc.p_w_info = unsafe { (*p_wc).p_w_info };
                                        temp_wc.p_outer = p_wc;
                                        temp_wc.op = 44 as u8;
                                        temp_wc.n_term = 1;
                                        temp_wc.n_base = 1;
                                        temp_wc.a = p_or_term;
                                        s_sub_build.p_wc = &mut temp_wc;
                                    } else { break '__c62; }
                                    s_cur.n = 0 as u16;
                                    if unsafe { (*unsafe { (*p_item).p_s_tab }).e_tab_type } as
                                                i32 == 1 {
                                        rc =
                                            where_loop_add_virtual(&mut s_sub_build, m_prereq_1,
                                                m_unusable_1);
                                    } else {
                                        rc = where_loop_add_btree(&mut s_sub_build, m_prereq_1);
                                    }
                                    if rc == 0 {
                                        rc =
                                            where_loop_add_or(&mut s_sub_build, m_prereq_1,
                                                m_unusable_1);
                                    }
                                    if s_cur.n as i32 == 0 {
                                        s_sum.n = 0 as u16;
                                        break '__b62;
                                    } else if once != 0 {
                                        where_or_move(&mut s_sum, &s_cur);
                                        once = 0;
                                    } else {
                                        let mut s_prev: WhereOrSet = unsafe { core::mem::zeroed() };
                                        where_or_move(&mut s_prev, &s_sum);
                                        s_sum.n = 0 as u16;
                                        {
                                            i = 0;
                                            '__b63: loop {
                                                if !(i < s_prev.n as i32) { break '__b63; }
                                                '__c63: loop {
                                                    {
                                                        j = 0;
                                                        '__b64: loop {
                                                            if !(j < s_cur.n as i32) { break '__b64; }
                                                            '__c64: loop {
                                                                where_or_insert(&mut s_sum,
                                                                    s_prev.a[i as usize].prereq | s_cur.a[j as usize].prereq,
                                                                    unsafe {
                                                                        sqlite3_log_est_add(s_prev.a[i as usize].r_run,
                                                                            s_cur.a[j as usize].r_run)
                                                                    },
                                                                    unsafe {
                                                                        sqlite3_log_est_add(s_prev.a[i as usize].n_out,
                                                                            s_cur.a[j as usize].n_out)
                                                                    });
                                                                break '__c64;
                                                            }
                                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                                        }
                                                    }
                                                    break '__c63;
                                                }
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                    }
                                    break '__c62;
                                }
                                {
                                    let __p = &mut p_or_term;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                            }
                        }
                        unsafe { (*p_new).n_l_term = 1 as u16 };
                        unsafe {
                            *unsafe { (*p_new).a_l_term.offset(0 as isize) } = p_term
                        };
                        unsafe { (*p_new).ws_flags = 8192 as u32 };
                        unsafe { (*p_new).r_setup = 0 as LogEst };
                        unsafe { (*p_new).i_sort_idx = 0 as u8 };
                        unsafe {
                            memset(unsafe { &raw mut (*p_new).u } as *mut (), 0, 24)
                        };
                        {
                            i = 0;
                            '__b65: loop {
                                if !(rc == 0 && i < s_sum.n as i32) { break '__b65; }
                                '__c65: loop {

                                    /// TUNING: Currently sSum.a[i].rRun is set to the sum of the costs
                                    ///* of all sub-scans required by the OR-scan. However, due to rounding
                                    ///* errors, it may be that the cost of the OR-scan is equal to its
                                    ///* most expensive sub-scan. Add the smallest possible penalty
                                    ///* (equivalent to multiplying the cost by 1.07) to ensure that
                                    ///* this does not happen. Otherwise, for WHERE clauses such as the
                                    ///* following where there is an index on "y":
                                    ///*
                                    ///*     WHERE likelihood(x=?, 0.99) OR y=?
                                    ///*
                                    ///* the planner may elect to "OR" together a full-table scan and an
                                    ///* index lookup. And other similarly odd results.
                                    unsafe {
                                        (*p_new).r_run =
                                            (s_sum.a[i as usize].r_run as i32 + 1) as LogEst
                                    };
                                    unsafe { (*p_new).n_out = s_sum.a[i as usize].n_out };
                                    unsafe { (*p_new).prereq = s_sum.a[i as usize].prereq };
                                    rc = where_loop_insert(unsafe { &mut *p_builder_1 }, p_new);
                                    break '__c65;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                    break '__c61;
                }
                {
                    let __p = &mut p_term;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
        return rc;
    }
}

///* Add all WhereLoop objects for all tables
#[allow(unused_doc_comments)]
extern "C" fn where_loop_add_all(p_builder_1: *mut WhereLoopBuilder) -> i32 {
    let p_w_info: *mut WhereInfo = unsafe { (*p_builder_1).p_w_info };
    let mut m_prereq: Bitmask = 0 as Bitmask;
    let mut m_prior: Bitmask = 0 as Bitmask;
    let mut i_tab: i32 = 0;
    let p_tab_list: *mut SrcList = unsafe { (*p_w_info).p_tab_list };
    let mut p_item: *mut SrcItem = core::ptr::null_mut();
    let p_end: *mut SrcItem =
        unsafe {
            &mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                            *mut SrcItem).add(unsafe { (*p_w_info).n_level } as usize)
        };
    let db: *mut Sqlite3 = unsafe { (*unsafe { (*p_w_info).p_parse }).db };
    let mut rc: i32 = 0;
    let mut b_first_past_rj: i32 = 0;
    let mut has_right_cross_join: i32 = 0;
    let mut p_new: *mut WhereLoop = core::ptr::null_mut();

    /// Loop over the tables in the join, from left to right
    (p_new = unsafe { (*p_builder_1).p_new });

    /// Verify that pNew has already been initialized
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { (*p_builder_1).i_plan_limit = 20000 as u32 };
    {
        {
            i_tab = 0;
            p_item = unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem
        };
        '__b66: loop {
            if !(p_item < p_end) { break '__b66; }
            '__c66: loop {
                let mut m_unusable: Bitmask = 0 as Bitmask;
                unsafe { (*p_new).i_tab = i_tab as u8 };
                unsafe { (*p_builder_1).i_plan_limit += 1000 as u32 };
                unsafe {
                    (*p_new).mask_self =
                        sqlite3_where_get_mask(unsafe { &(*p_w_info).s_mask_set },
                            unsafe { (*p_item).i_cursor })
                };
                if b_first_past_rj != 0 ||
                        unsafe { (*p_item).fg.jointype } as i32 & (32 | 2 | 64) != 0
                    {
                    if unsafe { (*p_item).fg.jointype } as i32 & (64 | 2) != 0 {
                        has_right_cross_join = 1;
                    }
                    m_prereq |= m_prior;
                    b_first_past_rj =
                        (unsafe { (*p_item).fg.jointype } as i32 & 16 != 0) as i32;
                } else if unsafe { (*p_item).fg.from_exists() } != 0 {
                    /// joins that result from the EXISTS-to-JOIN optimization should not
                    ///* be moved to the left of any of their dependencies
                    let p_wc: *const WhereClause =
                        unsafe { &raw mut (*p_w_info).s_wc } as *const WhereClause;
                    let mut p_term: *const WhereTerm = core::ptr::null();
                    let mut i: i32 = 0;
                    {
                        {
                            i = unsafe { (*p_wc).n_base };
                            p_term = unsafe { (*p_wc).a }
                        };
                        '__b67: loop {
                            if !(i > 0) { break '__b67; }
                            '__c67: loop {
                                if unsafe { (*p_new).mask_self } &
                                            unsafe { (*p_term).prereq_all } != 0 as u64 {
                                    m_prereq |=
                                        unsafe { (*p_term).prereq_all } &
                                            unsafe { (*p_new).mask_self } - 1 as Bitmask;
                                }
                                break '__c67;
                            }
                            {
                                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                                {
                                    let __p = &mut p_term;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                            };
                        }
                    }
                } else if (has_right_cross_join == 0) as i32 != 0 {
                    m_prereq = 0 as Bitmask;
                }
                if unsafe { (*unsafe { (*p_item).p_s_tab }).e_tab_type } as
                            i32 == 1 {
                    let mut p: *mut SrcItem = core::ptr::null_mut();
                    {
                        p = unsafe { p_item.offset(1 as isize) };
                        '__b68: loop {
                            if !(p < p_end) { break '__b68; }
                            '__c68: loop {
                                if m_unusable != 0 ||
                                        unsafe { (*p).fg.jointype } as i32 & (32 | 2) != 0 {
                                    m_unusable |=
                                        sqlite3_where_get_mask(unsafe { &(*p_w_info).s_mask_set },
                                            unsafe { (*p).i_cursor });
                                }
                                break '__c68;
                            }
                            {
                                let __p = &mut p;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                    }
                    rc =
                        where_loop_add_virtual(p_builder_1, m_prereq, m_unusable);
                } else { rc = where_loop_add_btree(p_builder_1, m_prereq); }
                if rc == 0 &&
                        unsafe { (*unsafe { (*p_builder_1).p_wc }).has_or } != 0 {
                    rc = where_loop_add_or(p_builder_1, m_prereq, m_unusable);
                }
                m_prior |= unsafe { (*p_new).mask_self };
                if rc != 0 || unsafe { (*db).malloc_failed } != 0 {
                    if rc == 101 {

                        /// We hit the query planner search limit set by iPlanLimit
                        unsafe {
                            sqlite3_log(28,
                                c"abbreviated query algorithm search".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        rc = 0;
                    } else { break '__b66; }
                }
                break '__c66;
            }
            {
                { let __p = &mut i_tab; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut p_item;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    where_loop_clear(db, p_new);
    return rc;
}

///* Compute the maximum number of paths in the solver algorithm, for
///* queries that have three or more terms in the FROM clause.  Queries with
///* two or fewer FROM clause terms are handled by the caller.
///*
///* Query planning is NP-hard.  We must limit the number of paths at
///* each step of the solver search algorithm to avoid exponential behavior.
///*
///* The value returned is a tuning parameter.  Currently the value is:
///*
///*     18    for star queries
///*     12    otherwise
///*
///* For the purposes of this heuristic, a star-query is defined as a query
///* with a central "fact" table that is joined against multiple
///* "dimension" tables, subject to the following constraints:
///*
///*   (aa)  Only a five-way or larger join is considered for this
///*         optimization.  If there are fewer than four terms in the FROM
///*         clause, this heuristic does not apply.
///*
///*   (bb)  The join between the fact table and the dimension tables must
///*         be an INNER join.  CROSS and OUTER JOINs do not qualify.
///*
///*   (cc)  A table must have 3 or more dimension tables in order to be
///*         considered a fact table. (Was 4 prior to 2026-02-10.)
///*
///*   (dd)  A table that is a self-join cannot be a dimension table.
///*         Dimension tables are joined against fact tables.
///*
///* SIDE EFFECT:  (and really the whole point of this subroutine)
///*
///* If pWInfo describes a star-query, then the cost for SCANs of dimension
///* WhereLoops is increased to be slightly larger than the cost of a SCAN
///* in the fact table.  Only SCAN costs are increased.  SEARCH costs are
///* unchanged. This heuristic helps keep fact tables in outer loops. Without
///* this heuristic, paths with fact tables in outer loops tend to get pruned
///* by the mxChoice limit on the number of paths, resulting in poor query
///* plans.  See the starschema1.test test module for examples of queries
///* that need this heuristic to find good query plans.
///*
///* This heuristic can be completely disabled, so that no query is
///* considered a star-query, using SQLITE_TESTCTRL_OPTIMIZATION to
///* disable the SQLITE_StarQuery optimization.  In the CLI, the command
///* to do that is:  ".testctrl opt -starquery".
///*
///* HISTORICAL NOTES:
///*
///* This optimization was first added on 2024-05-09 by check-in 38db9b5c83d.
///* The original optimization reduced the cost and output size estimate for
///* fact tables to help them move to outer loops.  But months later (as people
///* started upgrading) performance regression reports started caming in,
///* including:
///*
///*    forum post b18ef983e68d06d1 (2024-12-21)
///*    forum post 0025389d0860af82 (2025-01-14)
///*    forum post d87570a145599033 (2025-01-17)
///*
///* To address these, the criteria for a star-query was tightened to exclude
///* cases where the fact and dimensions are separated by an outer join, and
///* the affect of star-schema detection was changed to increase the rRun cost
///* on just full table scans of dimension tables, rather than reducing costs
///* in the all access methods of the fact table.
#[allow(unused_doc_comments)]
extern "C" fn compute_mx_choice(p_w_info_1: &mut WhereInfo) -> i32 {
    let n_loop: i32 = (*p_w_info_1).n_level as i32;
    /// Number of terms in the join
    let mut p_w_loop: *mut WhereLoop = core::ptr::null_mut();
    if n_loop >= 4 && ((*p_w_info_1).b_star_done() == 0) as i32 != 0 &&
            unsafe { (*unsafe { (*(*p_w_info_1).p_parse).db }).db_opt_flags }
                    & 536870912 as u32 == 0 as u32 {
        let mut a_from_tabs: *mut SrcItem = core::ptr::null_mut();
        /// All terms of the FROM clause
        let mut i_from_idx: i32 = 0;
        /// Term of FROM clause is the candidate fact-table
        let mut m: Bitmask = 0 as Bitmask;
        /// Bitmask for candidate fact-table
        let mut m_self_join: Bitmask = 0 as Bitmask;
        /// Tables that cannot be dimension tables
        let mut p_start: *mut WhereLoop = core::ptr::null_mut();

        /// Where to start searching for dimension-tables
        (*p_w_info_1).set_b_star_done(1 as u32 as u32);

        /// Only do this computation once
        /// Look for fact tables with three or more dimensions where the
        ///* dimension tables are not separately from the fact tables by an outer
        ///* or cross join.  Adjust cost weights if found.
        { let _ = 0; };
        a_from_tabs =
            unsafe { (*(*p_w_info_1).p_tab_list).a.as_ptr() } as *mut SrcItem;
        p_start = (*p_w_info_1).p_loops;
        {
            { ({ i_from_idx = 0; i_from_idx }) as Bitmask; m = 1 as Bitmask };
            '__b69: loop {
                if !(i_from_idx < n_loop) { break '__b69; }
                '__c69: loop {
                    let mut n_dep: i32 = 0;
                    /// Number of dimension tables
                    let mut mx_run: LogEst = 0 as LogEst;
                    /// Maximum SCAN cost of a fact table
                    let mut m_seen: Bitmask = 0 as Bitmask;
                    /// Mask of dimension tables
                    let mut p_fact_tab: *const SrcItem = core::ptr::null();

                    /// The candidate fact table
                    (p_fact_tab =
                        unsafe { a_from_tabs.offset(i_from_idx as isize) });
                    if unsafe { (*p_fact_tab).fg.jointype } as i32 & (32 | 2) !=
                            0 {
                        if i_from_idx + 3 > n_loop { break '__b69; }
                        while !(p_start).is_null() &&
                                unsafe { (*p_start).i_tab } as i32 <= i_from_idx {
                            p_start = unsafe { (*p_start).p_next_loop };
                        }
                    }
                    {
                        p_w_loop = p_start;
                        '__b71: loop {
                            if !(!(p_w_loop).is_null()) { break '__b71; }
                            '__c71: loop {
                                if unsafe {
                                                    (*a_from_tabs.add(unsafe { (*p_w_loop).i_tab } as
                                                                        usize)).fg.jointype
                                                } as i32 & (32 | 2) != 0 {
                                    break '__b71;
                                }
                                if unsafe { (*p_w_loop).prereq } & m != 0 as u64 &&
                                            unsafe { (*p_w_loop).mask_self } & m_seen == 0 as u64 &&
                                        unsafe { (*p_w_loop).mask_self } & m_self_join == 0 as u64 {
                                    if unsafe {
                                                (*a_from_tabs.add(unsafe { (*p_w_loop).i_tab } as
                                                                usize)).p_s_tab
                                            } == unsafe { (*p_fact_tab).p_s_tab } {
                                        m_self_join |= m;
                                    } else {
                                        { let __p = &mut n_dep; let __t = *__p; *__p += 1; __t };
                                        m_seen |= unsafe { (*p_w_loop).mask_self };
                                    }
                                }
                                break '__c71;
                            }
                            p_w_loop = unsafe { (*p_w_loop).p_next_loop };
                        }
                    }
                    if n_dep <= 2 { break '__c69; }

                    /// If we reach this point, it means that pFactTab is a fact table
                    ///* with four or more dimensions connected by inner joins.  Proceed
                    ///* to make cost adjustments.
                    /// 0x80000
                    (*p_w_info_1).set_b_star_used(1 as u32 as u32);

                    /// Compute the maximum cost of any WhereLoop for the
                    ///* fact table plus one epsilon
                    (mx_run = -32768 as LogEst);
                    {
                        p_w_loop = p_start;
                        '__b72: loop {
                            if !(!(p_w_loop).is_null()) { break '__b72; }
                            '__c72: loop {
                                if (unsafe { (*p_w_loop).i_tab } as i32) < i_from_idx {
                                    break '__c72;
                                }
                                if unsafe { (*p_w_loop).i_tab } as i32 > i_from_idx {
                                    break '__b72;
                                }
                                if unsafe { (*p_w_loop).r_run } as i32 > mx_run as i32 {
                                    mx_run = unsafe { (*p_w_loop).r_run };
                                }
                                break '__c72;
                            }
                            p_w_loop = unsafe { (*p_w_loop).p_next_loop };
                        }
                    }
                    if (mx_run as i32) < 32767 {
                        { let __p = &mut mx_run; let __t = *__p; *__p += 1; __t };
                    }
                    {
                        p_w_loop = p_start;
                        '__b73: loop {
                            if !(!(p_w_loop).is_null()) { break '__b73; }
                            '__c73: loop {
                                if unsafe { (*p_w_loop).mask_self } & m_seen == 0 as u64 {
                                    break '__c73;
                                }
                                if unsafe { (*p_w_loop).n_l_term } != 0 { break '__c73; }
                                if (unsafe { (*p_w_loop).r_run } as i32) < mx_run as i32 {

                                    /// 0x80000
                                    /// WHERETRACE_ENABLED
                                    unsafe { (*p_w_loop).r_run = mx_run };
                                }
                                break '__c73;
                            }
                            p_w_loop = unsafe { (*p_w_loop).p_next_loop };
                        }
                    }
                    break '__c69;
                }
                {
                    ({
                            let __p = &mut i_from_idx;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        }) as Bitmask;
                    m <<= 1 as Bitmask
                };
            }
        }
    }
    return if (*p_w_info_1).b_star_used() != 0 { 18 } else { 12 };
}

/// Implementation of the order-by-subquery optimization:
///*
///* WhereLoop pLoop, which the iLoop-th term of the nested loop, is really
///* a subquery or CTE that has an ORDER BY clause.  See if any of the terms
///* in the subquery ORDER BY clause will satisfy pOrderBy from the outer
///* query.  Mark off all satisfied terms (by setting bits in *pOBSat) and
///* return TRUE if they do.  If not, return false.
///*
///* Example:
///*
///*    CREATE TABLE t1(a,b,c, PRIMARY KEY(a,b));
///*    CREATE TABLE t2(x,y);
///*    WITH t3(p,q) AS MATERIALIZED (SELECT x+y, x-y FROM t2 ORDER BY x+y)
///*       SELECT * FROM t3 JOIN t1 ON a=q ORDER BY p, b;
///*
///* The CTE named "t3" comes out in the natural order of "p", so the first
///* first them of "ORDER BY p,b" is satisfied by a sequential scan of "t3"
///* and sorting only needs to occur on the second term "b".
///*
///* Limitations:
///*
///* (1)  The optimization is not applied if the outer ORDER BY contains
///*      a COLLATE clause.  The optimization might be applied if the
///*      outer ORDER BY uses NULLS FIRST, NULLS LAST, ASC, and/or DESC as
///*      long as the subquery ORDER BY does the same.  But if the
///*      outer ORDER BY uses COLLATE, even a redundant COLLATE, the
///*      optimization is bypassed.
///*
///* (2)  The subquery ORDER BY terms must exactly match subquery result
///*      columns, including any COLLATE annotations.  This routine relies
///*      on iOrderByCol to do matching between order by terms and result
///*      columns, and iOrderByCol will not be set if the result column
///*      and ORDER BY collations differ.
///*
///* (3)  The subquery and outer ORDER BY can be in opposite directions as
///*      long as  the subquery is materialized.  If the subquery is
///*      implemented as a co-routine, the sort orders must be in the same
///*      direction because there is no way to run a co-routine backwards.
#[allow(unused_doc_comments)]
extern "C" fn where_path_match_subquery_ob(p_w_info_1: &WhereInfo,
    p_loop_1: &WhereLoop, i_loop_1: i32, i_cur_1: i32,
    p_order_by_1: &ExprList, p_rev_mask_1: &mut Bitmask,
    p_ob_sat_1: &mut Bitmask) -> i32 {
    unsafe {
        let mut i_ob: i32 = 0;
        /// Index into pOrderBy->a[]
        let mut j_sub: i32 = 0;
        /// Index into pSubOB->a[]
        let mut rev: u8 = 0 as u8;
        /// True if iOB and jSub sort in opposite directions
        let mut rev_idx: u8 = 0 as u8;
        /// Sort direction for jSub
        let mut p_ob_expr: *const Expr = core::ptr::null();
        /// Current term of outer ORDER BY
        let mut p_sub_ob: *const ExprList = core::ptr::null();

        /// Complete ORDER BY on the subquery
        (p_sub_ob = (*p_loop_1).u.btree.p_order_by);
        { let _ = 0; };
        {
            i_ob = 0;
            '__b74: loop {
                if !((1 as Bitmask) << i_ob & *p_ob_sat_1 != 0 as u64) {
                    break '__b74;
                }
                '__c74: loop { break '__c74; }
                { let __p = &mut i_ob; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            j_sub = 0;
            '__b75: loop {
                if !(j_sub < unsafe { (*p_sub_ob).n_expr } &&
                                i_ob < (*p_order_by_1).n_expr) {
                    break '__b75;
                }
                '__c75: loop {
                    if unsafe {
                                    (*(unsafe { (*p_sub_ob).a.as_ptr() } as
                                                            *mut ExprListItem).offset(j_sub as
                                                            isize)).u.x.i_order_by_col
                                } as i32 == 0 {
                        break '__b75;
                    }
                    p_ob_expr =
                        unsafe {
                            (*((*p_order_by_1).a.as_ptr() as
                                            *mut ExprListItem).offset(i_ob as isize)).p_expr
                        };
                    if unsafe { (*p_ob_expr).op } as i32 != 168 &&
                            unsafe { (*p_ob_expr).op } as i32 != 170 {
                        break '__b75;
                    }
                    if unsafe { (*p_ob_expr).i_table } != i_cur_1 {
                        break '__b75;
                    }
                    if unsafe { (*p_ob_expr).i_column } as i32 !=
                            unsafe {
                                        (*(unsafe { (*p_sub_ob).a.as_ptr() } as
                                                                *mut ExprListItem).offset(j_sub as
                                                                isize)).u.x.i_order_by_col
                                    } as i32 - 1 {
                        break '__b75;
                    }
                    if (*p_w_info_1).wctrl_flags as i32 & 64 == 0 {
                        let sf_ob: u8 =
                            unsafe {
                                (*((*p_order_by_1).a.as_ptr() as
                                                    *mut ExprListItem).offset(i_ob as isize)).fg.sort_flags
                            };
                        /// sortFlags for iOB
                        let sf_sub: u8 =
                            unsafe {
                                (*(unsafe { (*p_sub_ob).a.as_ptr() } as
                                                    *mut ExprListItem).offset(j_sub as isize)).fg.sort_flags
                            };
                        if sf_sub as i32 & 2 != sf_ob as i32 & 2 { break '__b75; }
                        rev_idx = (sf_sub as i32 & 1) as u8;
                        if j_sub > 0 {
                            if rev as i32 ^ rev_idx as i32 != sf_ob as i32 & 1 {
                                break '__b75;
                            }
                        } else {
                            rev = (rev_idx as i32 ^ sf_ob as i32 & 1) as u8;
                            if rev != 0 {
                                if (*p_loop_1).ws_flags & 33554432 as u32 != 0 as u32 {

                                    /// Cannot run a co-routine in reverse order
                                    break '__b75;
                                }
                                *p_rev_mask_1 |= (1 as Bitmask) << i_loop_1;
                            }
                        }
                    }
                    *p_ob_sat_1 |= (1 as Bitmask) << i_ob;
                    break '__c75;
                }
                {
                    { let __p = &mut j_sub; let __t = *__p; *__p += 1; __t };
                    { let __p = &mut i_ob; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        return (j_sub > 0) as i32;
    }
}

///* Examine a WherePath (with the addition of the extra WhereLoop of the 6th
///* parameters) to see if it outputs rows in the requested ORDER BY
///* (or GROUP BY) without requiring a separate sort operation.  Return N:
///*
///*   N>0:   N terms of the ORDER BY clause are satisfied
///*   N==0:  No terms of the ORDER BY clause are satisfied
///*   N<0:   Unknown yet how many terms of ORDER BY might be satisfied.  
///*
///* Note that processing for WHERE_GROUPBY and WHERE_DISTINCTBY is not as
///* strict.  With GROUP BY and DISTINCT the only requirement is that
///* equivalent rows appear immediately adjacent to one another.  GROUP BY
///* and DISTINCT do not require rows to appear in any particular order as long
///* as equivalent rows are grouped together.  Thus for GROUP BY and DISTINCT
///* the pOrderBy terms can be matched in any order.  With ORDER BY, the
///* pOrderBy terms must be matched in strict left-to-right order.
#[allow(unused_doc_comments)]
extern "C" fn where_path_satisfies_order_by(p_w_info_1: *mut WhereInfo,
    p_order_by_1: *mut ExprList, p_path_1: &WherePath, wctrl_flags_1: u16,
    n_loop_1: u16, p_last_1: *mut WhereLoop, p_rev_mask_1: *mut Bitmask)
    -> i8 {
    unsafe {
        let mut rev_set: u8 = 0 as u8;
        /// True if rev is known
        let mut rev: u8 = 0 as u8;
        /// Composite sort order
        let mut rev_idx: u8 = 0 as u8;
        /// Index sort order
        let mut is_order_distinct: u8 = 0 as u8;
        /// All prior WhereLoops are order-distinct
        let mut distinct_columns: u8 = 0 as u8;
        /// True if the loop has UNIQUE NOT NULL columns
        let mut is_match: u8 = 0 as u8;
        /// iColumn matches a term of the ORDER BY clause
        let mut eq_op_mask: u16 = 0 as u16;
        /// Allowed equality operators
        let mut n_key_col: u16 = 0 as u16;
        /// Number of key columns in pIndex
        let mut n_column: u16 = 0 as u16;
        /// Total number of ordered columns in the index
        let mut n_order_by: u16 = 0 as u16;
        /// Number terms in the ORDER BY clause
        let mut i_loop: i32 = 0;
        /// Index of WhereLoop in pPath being processed
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        /// Loop counters
        let mut i_cur: i32 = 0;
        /// Cursor number for current WhereLoop
        let mut i_column: i32 = 0;
        /// A column number within table iCur
        let mut p_loop: *mut WhereLoop = core::ptr::null_mut();
        /// Current WhereLoop being processed.
        let mut p_term: *mut WhereTerm = core::ptr::null_mut();
        /// A single term of the WHERE clause
        let mut p_ob_expr: *mut Expr = core::ptr::null_mut();
        /// An expression from the ORDER BY clause
        let mut p_coll: *const CollSeq = core::ptr::null();
        /// COLLATE function from an ORDER BY clause term
        let mut p_index: *const Index = core::ptr::null();
        /// The index associated with pLoop
        let db: *const Sqlite3 =
            unsafe { (*unsafe { (*p_w_info_1).p_parse }).db } as
                *const Sqlite3;
        /// Database connection
        let mut ob_sat: Bitmask = 0 as Bitmask;
        /// Mask of ORDER BY terms satisfied so far
        let mut ob_done: Bitmask = 0 as Bitmask;
        /// Mask of all ORDER BY terms
        let mut order_distinct_mask: Bitmask = 0 as Bitmask;
        /// Mask of all well-ordered loops
        let mut ready: Bitmask = 0 as Bitmask;

        /// Mask of inner loops
        ///* We say the WhereLoop is "one-row" if it generates no more than one
        ///* row of output.  A WhereLoop is one-row if all of the following are true:
        ///*  (a) All index columns match with WHERE_COLUMN_EQ.
        ///*  (b) The index is unique
        ///* Any WhereLoop with an WHERE_COLUMN_EQ constraint on the rowid is one-row.
        ///* Every one-row WhereLoop will have the WHERE_ONEROW bit set in wsFlags.
        ///*
        ///* We say the WhereLoop is "order-distinct" if the set of columns from
        ///* that WhereLoop that are in the ORDER BY clause are different for every
        ///* row of the WhereLoop.  Every one-row WhereLoop is automatically
        ///* order-distinct.   A WhereLoop that has no columns in the ORDER BY clause
        ///* is not order-distinct. To be order-distinct is not quite the same as being
        ///* UNIQUE since a UNIQUE column or index can have multiple rows that
        ///* are NULL and NULL values are equivalent for the purpose of order-distinct.
        ///* To be order-distinct, the columns must be UNIQUE and NOT NULL.
        ///*
        ///* The rowid for a table is always UNIQUE and NOT NULL so whenever the
        ///* rowid appears in the ORDER BY clause, the corresponding WhereLoop is
        ///* automatically order-distinct.
        { let _ = 0; };
        if n_loop_1 != 0 &&
                unsafe { (*db).db_opt_flags } & 64 as u32 != 0 as u32 {
            return 0 as i8;
        }
        n_order_by = unsafe { (*p_order_by_1).n_expr } as u16;
        if n_order_by as i32 >
                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 - 1
            {
            return 0 as i8;
        }

        /// Cannot optimize overly large ORDER BYs
        (is_order_distinct = 1 as u8);
        ob_done = ((1 as Bitmask) << n_order_by) - 1 as Bitmask;
        order_distinct_mask = 0 as Bitmask;
        ready = 0 as Bitmask;
        eq_op_mask = (2 | 128 | 256) as u16;
        if wctrl_flags_1 as i32 & (2048 | 2 | 1) != 0 {
            eq_op_mask |= 1 as u16;
        }
        {
            i_loop = 0;
            '__b76: loop {
                if !(is_order_distinct != 0 && ob_sat < ob_done &&
                                i_loop <= n_loop_1 as i32) {
                    break '__b76;
                }
                '__c76: loop {
                    if i_loop > 0 { ready |= unsafe { (*p_loop).mask_self }; }
                    if i_loop < n_loop_1 as i32 {
                        p_loop =
                            unsafe { *(*p_path_1).a_loop.offset(i_loop as isize) };
                        if wctrl_flags_1 as i32 & 2048 != 0 { break '__c76; }
                    } else { p_loop = p_last_1; }
                    if unsafe { (*p_loop).ws_flags } & 1024 as u32 != 0 {
                        if unsafe { (*p_loop).u.vtab.is_ordered } != 0 &&
                                unsafe { (*p_w_info_1).p_order_by } == p_order_by_1 {
                            ob_sat = ob_done;
                        } else {

                            /// No further ORDER BY terms may be matched. So this call should
                            ///* return >=0, not -1. Clear isOrderDistinct to ensure it does so.
                            (is_order_distinct = 0 as u8);
                        }
                        break '__b76;
                    }
                    i_cur =
                        unsafe {
                            (*(unsafe {
                                                (*unsafe { (*p_w_info_1).p_tab_list }).a.as_ptr()
                                            } as
                                            *mut SrcItem).add(unsafe { (*p_loop).i_tab } as
                                            usize)).i_cursor
                        };
                    {
                        i = 0;
                        '__b77: loop {
                            if !(i < n_order_by as i32) { break '__b77; }
                            '__c77: loop {
                                if (1 as Bitmask) << i & ob_sat != 0 { break '__c77; }
                                p_ob_expr =
                                    unsafe {
                                        sqlite3_expr_skip_collate_and_likely(unsafe {
                                                (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                            })
                                    };
                                if p_ob_expr == core::ptr::null_mut() { break '__c77; }
                                if unsafe { (*p_ob_expr).op } as i32 != 168 &&
                                        unsafe { (*p_ob_expr).op } as i32 != 170 {
                                    break '__c77;
                                }
                                if unsafe { (*p_ob_expr).i_table } != i_cur {
                                    break '__c77;
                                }
                                p_term =
                                    sqlite3_where_find_term(unsafe { &mut (*p_w_info_1).s_wc },
                                        i_cur, unsafe { (*p_ob_expr).i_column } as i32, !ready,
                                        eq_op_mask as u32, core::ptr::null_mut());
                                if p_term == core::ptr::null_mut() { break '__c77; }
                                if unsafe { (*p_term).e_operator } as i32 == 1 {

                                    /// IN terms are only valid for sorting in the ORDER BY LIMIT
                                    ///* optimization, and then only if they are actually used
                                    ///* by the query plan
                                    { let _ = 0; };
                                    {
                                        j = 0;
                                        '__b78: loop {
                                            if !(j < unsafe { (*p_loop).n_l_term } as i32 &&
                                                            p_term !=
                                                                unsafe {
                                                                    *unsafe { (*p_loop).a_l_term.offset(j as isize) }
                                                                }) {
                                                break '__b78;
                                            }
                                            '__c78: loop { break '__c78; }
                                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    if j >= unsafe { (*p_loop).n_l_term } as i32 {
                                        break '__c77;
                                    }
                                }
                                if unsafe { (*p_term).e_operator } as i32 & (2 | 128) != 0
                                        && unsafe { (*p_ob_expr).i_column } as i32 >= 0 {
                                    let p_parse: *mut Parse = unsafe { (*p_w_info_1).p_parse };
                                    let p_coll1: *const CollSeq =
                                        unsafe {
                                                sqlite3_expr_nn_coll_seq(p_parse,
                                                    unsafe {
                                                            (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                                        } as *const Expr)
                                            } as *const CollSeq;
                                    let p_coll2: *const CollSeq =
                                        unsafe {
                                                sqlite3_expr_compare_coll_seq(p_parse,
                                                    unsafe { (*p_term).p_expr } as *const Expr)
                                            } as *const CollSeq;
                                    { let _ = 0; };
                                    if p_coll2 == core::ptr::null_mut() ||
                                            unsafe {
                                                    sqlite3_str_i_cmp(unsafe { (*p_coll1).z_name } as *const i8,
                                                        unsafe { (*p_coll2).z_name } as *const i8)
                                                } != 0 {
                                        break '__c77;
                                    }
                                }
                                ob_sat |= (1 as Bitmask) << i;
                                break '__c77;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if unsafe { (*p_loop).ws_flags } & 4096 as u32 == 0 as u32 {
                        if unsafe { (*p_loop).ws_flags } & 256 as u32 != 0 {
                            if !(unsafe { (*p_loop).u.btree.p_order_by }).is_null() &&
                                        unsafe { (*db).db_opt_flags } & 268435456 as u32 == 0 as u32
                                    &&
                                    where_path_match_subquery_ob(unsafe { &*p_w_info_1 },
                                            unsafe { &*p_loop }, i_loop, i_cur,
                                            unsafe { &*p_order_by_1 }, unsafe { &mut *p_rev_mask_1 },
                                            &mut ob_sat) != 0 {
                                n_column = 0 as u16;
                                is_order_distinct = 0 as u8;
                            } else { n_column = 1 as u16; }
                            p_index = core::ptr::null_mut();
                            n_key_col = 0 as u16;
                        } else if {
                                        p_index = unsafe { (*p_loop).u.btree.p_index };
                                        p_index
                                    } == core::ptr::null_mut() ||
                                unsafe { (*p_index).b_unordered() } != 0 {
                            return 0 as i8;
                        } else {
                            n_key_col = unsafe { (*p_index).n_key_col };
                            n_column = unsafe { (*p_index).n_column };
                            { let _ = 0; };
                            { let _ = 0; };

                            /// All relevant terms of the index must also be non-NULL in order
                            ///* for isOrderDistinct to be true.  So the isOrderDistinct value
                            ///* computed here might be a false positive.  Corrections will be
                            ///* made at tag-20210426-1 below
                            (is_order_distinct =
                                (unsafe { (*p_index).on_error } as i32 != 0 &&
                                        unsafe { (*p_loop).ws_flags } & 32768 as u32 == 0 as u32) as
                                    u8);
                        }

                        /// Loop through all columns of the index and deal with the ones
                        ///* that are not constrained by == or IN.
                        (rev = { rev_set = 0 as u8; rev_set });
                        distinct_columns = 0 as u8;
                        {
                            j = 0;
                            '__b79: loop {
                                if !(j < n_column as i32) { break '__b79; }
                                '__c79: loop {
                                    let mut b_once: u8 = 1 as u8;

                                    /// True to run the ORDER BY search loop
                                    { let _ = 0; };
                                    if j < unsafe { (*p_loop).u.btree.n_eq } as i32 &&
                                            j >= unsafe { (*p_loop).n_skip } as i32 {
                                        let e_op: u16 =
                                            unsafe {
                                                (*unsafe {
                                                                *unsafe { (*p_loop).a_l_term.offset(j as isize) }
                                                            }).e_operator
                                            };
                                        if e_op as i32 & eq_op_mask as i32 != 0 {
                                            if e_op as i32 & (256 | 128) != 0 {
                                                is_order_distinct = 0 as u8;
                                            }
                                            break '__c79;
                                        } else if e_op as i32 & 1 != 0 {
                                            /// ALWAYS() justification: eOp is an equality operator due to the
                                            ///* j<pLoop->u.btree.nEq constraint above.  Any equality other
                                            ///* than WO_IN is captured by the previous "if".  So this one
                                            ///* always has to be WO_IN.
                                            let p_x: *mut Expr =
                                                unsafe {
                                                    (*unsafe {
                                                                    *unsafe { (*p_loop).a_l_term.offset(j as isize) }
                                                                }).p_expr
                                                };
                                            {
                                                i = j + 1;
                                                '__b80: loop {
                                                    if !(i < unsafe { (*p_loop).u.btree.n_eq } as i32) {
                                                        break '__b80;
                                                    }
                                                    '__c80: loop {
                                                        if unsafe {
                                                                    (*unsafe {
                                                                                    *unsafe { (*p_loop).a_l_term.offset(i as isize) }
                                                                                }).p_expr
                                                                } == p_x {
                                                            { let _ = 0; };
                                                            b_once = 0 as u8;
                                                            break '__b80;
                                                        }
                                                        break '__c80;
                                                    }
                                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                        }
                                    }
                                    if !(p_index).is_null() {
                                        i_column =
                                            unsafe {
                                                    *unsafe { (*p_index).ai_column.offset(j as isize) }
                                                } as i32;
                                        rev_idx =
                                            (unsafe {
                                                            *unsafe { (*p_index).a_sort_order.offset(j as isize) }
                                                        } as i32 & 1) as u8;
                                        if i_column ==
                                                unsafe { (*unsafe { (*p_index).p_table }).i_p_key } as i32 {
                                            i_column = -1;
                                        }
                                    } else { i_column = -1; rev_idx = 0 as u8; }
                                    if is_order_distinct != 0 {
                                        if i_column >= 0 &&
                                                    j >= unsafe { (*p_loop).u.btree.n_eq } as i32 &&
                                                unsafe {
                                                            (*unsafe {
                                                                        (*unsafe {
                                                                                            (*p_index).p_table
                                                                                        }).a_col.offset(i_column as isize)
                                                                    }).not_null()
                                                        } as i32 == 0 {
                                            is_order_distinct = 0 as u8;
                                        }
                                        if i_column == -2 { is_order_distinct = 0 as u8; }
                                    }

                                    /// Find the ORDER BY term that corresponds to the j-th column
                                    ///* of the index and mark that ORDER BY term having been satisfied.
                                    (is_match = 0 as u8);
                                    {
                                        i = 0;
                                        '__b81: loop {
                                            if !(b_once != 0 && i < n_order_by as i32) { break '__b81; }
                                            '__c81: loop {
                                                if (1 as Bitmask) << i & ob_sat != 0 { break '__c81; }
                                                p_ob_expr =
                                                    unsafe {
                                                        sqlite3_expr_skip_collate_and_likely(unsafe {
                                                                (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                                            })
                                                    };
                                                if p_ob_expr == core::ptr::null_mut() { break '__c81; }
                                                if wctrl_flags_1 as i32 & (64 | 128) == 0 {
                                                    b_once = 0 as u8;
                                                }
                                                if i_column >= -1 {
                                                    if unsafe { (*p_ob_expr).op } as i32 != 168 &&
                                                            unsafe { (*p_ob_expr).op } as i32 != 170 {
                                                        break '__c81;
                                                    }
                                                    if unsafe { (*p_ob_expr).i_table } != i_cur {
                                                        break '__c81;
                                                    }
                                                    if unsafe { (*p_ob_expr).i_column } as i32 != i_column {
                                                        break '__c81;
                                                    }
                                                } else {
                                                    let p_ix_expr: *mut Expr =
                                                        unsafe {
                                                            (*(unsafe { (*unsafe { (*p_index).a_col_expr }).a.as_ptr() }
                                                                            as *mut ExprListItem).offset(j as isize)).p_expr
                                                        };
                                                    if unsafe {
                                                                sqlite3_expr_compare_skip(p_ob_expr, p_ix_expr, i_cur)
                                                            } != 0 {
                                                        break '__c81;
                                                    }
                                                }
                                                if i_column != -1 {
                                                    p_coll =
                                                        unsafe {
                                                            sqlite3_expr_nn_coll_seq(unsafe { (*p_w_info_1).p_parse },
                                                                unsafe {
                                                                        (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                                                    } as *const Expr)
                                                        };
                                                    if unsafe {
                                                                sqlite3_str_i_cmp(unsafe { (*p_coll).z_name } as *const i8,
                                                                    unsafe {
                                                                        *unsafe { (*p_index).az_coll.offset(j as isize) }
                                                                    })
                                                            } != 0 {
                                                        break '__c81;
                                                    }
                                                }
                                                if wctrl_flags_1 as i32 & 128 != 0 {
                                                    unsafe {
                                                        (*p_loop).u.btree.n_distinct_col = (j + 1) as u16
                                                    };
                                                }
                                                is_match = 1 as u8;
                                                break '__b81;
                                                break '__c81;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    if is_match != 0 && wctrl_flags_1 as i32 & 64 == 0 {
                                        if rev_set != 0 {
                                            if rev as i32 ^ rev_idx as i32 !=
                                                    unsafe {
                                                                (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).fg.sort_flags
                                                            } as i32 & 1 {
                                                is_match = 0 as u8;
                                            }
                                        } else {
                                            rev =
                                                (rev_idx as i32 ^
                                                        unsafe {
                                                                    (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                                        *mut ExprListItem).offset(i as isize)).fg.sort_flags
                                                                } as i32 & 1) as u8;
                                            if rev != 0 {
                                                unsafe { *p_rev_mask_1 |= (1 as Bitmask) << i_loop };
                                            }
                                            rev_set = 1 as u8;
                                        }
                                    }
                                    if is_match != 0 &&
                                            unsafe {
                                                            (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(i as isize)).fg.sort_flags
                                                        } as i32 & 2 != 0 {
                                        if j == unsafe { (*p_loop).u.btree.n_eq } as i32 {
                                            unsafe { (*p_loop).ws_flags |= 524288 as u32 };
                                        } else { is_match = 0 as u8; }
                                    }
                                    if is_match != 0 {
                                        if i_column == -1 { distinct_columns = 1 as u8; }
                                        ob_sat |= (1 as Bitmask) << i;
                                    } else {
                                        if j == 0 || j < n_key_col as i32 {
                                            is_order_distinct = 0 as u8;
                                        }
                                        break '__b79;
                                    }
                                    break '__c79;
                                }
                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if distinct_columns != 0 { is_order_distinct = 1 as u8; }
                    }
                    if is_order_distinct != 0 {
                        order_distinct_mask |= unsafe { (*p_loop).mask_self };
                        {
                            i = 0;
                            '__b82: loop {
                                if !(i < n_order_by as i32) { break '__b82; }
                                '__c82: loop {
                                    let mut p: *mut Expr = core::ptr::null_mut();
                                    let mut m_term: Bitmask = 0 as Bitmask;
                                    if (1 as Bitmask) << i & ob_sat != 0 { break '__c82; }
                                    p =
                                        unsafe {
                                            (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        };
                                    m_term =
                                        unsafe {
                                            sqlite3_where_expr_usage(unsafe {
                                                    &mut (*p_w_info_1).s_mask_set
                                                }, p)
                                        };
                                    if m_term == 0 as u64 &&
                                            (unsafe {
                                                            sqlite3_expr_is_constant(core::ptr::null_mut(), p)
                                                        } == 0) as i32 != 0 {
                                        break '__c82;
                                    }
                                    if m_term & !order_distinct_mask == 0 as u64 {
                                        ob_sat |= (1 as Bitmask) << i;
                                    }
                                    break '__c82;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                    break '__c76;
                }
                { let __p = &mut i_loop; let __t = *__p; *__p += 1; __t };
            }
        }
        if ob_sat == ob_done { return n_order_by as i8; }
        if (is_order_distinct == 0) as i32 != 0 {
            {
                i = n_order_by as i32 - 1;
                '__b83: loop {
                    if !(i > 0) { break '__b83; }
                    '__c83: loop {
                        let m: Bitmask =
                            if i <
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                                ((1 as Bitmask) << i) - 1 as Bitmask
                            } else { 0 as Bitmask };
                        if ob_sat & m == m { return i as i8; }
                        break '__c83;
                    }
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                }
            }
            return 0 as i8;
        }
        return -1 as i8;
    }
}

///* Return the cost of sorting nRow rows, assuming that the keys have
///* nOrderby columns and that the first nSorted columns are already in
///* order.
#[allow(unused_doc_comments)]
extern "C" fn where_sorting_cost(p_w_info_1: &WhereInfo, mut n_row_1: LogEst,
    n_order_by_1: i32, n_sorted_1: i32) -> LogEst {
    unsafe {
        /// Estimated cost of a full external sort, where N is
        ///* the number of rows to sort is:
        ///*
        ///*   cost = (K * N * log(N)).
        ///*
        ///* Or, if the order-by clause has X terms but only the last Y
        ///* terms are out of order, then block-sorting will reduce the
        ///* sorting cost to:
        ///*
        ///*   cost = (K * N * log(N)) * (Y/X)
        ///*
        ///* The constant K is at least 2.0 but will be larger if there are a
        ///* large number of columns to be sorted, as the sorting time is
        ///* proportional to the amount of content to be sorted.  The algorithm
        ///* does not currently distinguish between fat columns (BLOBs and TEXTs)
        ///* and skinny columns (INTs).  It just uses the number of columns as
        ///* an approximation for the row width.
        ///*
        ///* And extra factor of 2.0 or 3.0 is added to the sorting cost if the sort
        ///* is built using OP_IdxInsert and OP_Sort rather than with OP_SorterInsert.
        let mut r_sort_cost: LogEst = 0 as LogEst;
        let mut n_col: LogEst = 0 as LogEst;
        { let _ = 0; };
        { let _ = 0; };

        /// TUNING: sorting cost proportional to the number of output columns:
        (n_col =
            unsafe {
                sqlite3_log_est(((unsafe {
                                    (*unsafe { (*(*p_w_info_1).p_select).p_e_list }).n_expr
                                } + 59) / 30) as u64)
            });
        r_sort_cost = (n_row_1 as i32 + n_col as i32) as LogEst;
        if n_sorted_1 > 0 {

            /// Scale the result by (Y/X)
            (r_sort_cost +=
                (unsafe {
                                sqlite3_log_est(((n_order_by_1 - n_sorted_1) * 100 /
                                            n_order_by_1) as u64)
                            } as i32 - 66) as LogEst);
        }
        if (*p_w_info_1).wctrl_flags as i32 & 16384 != 0 {
            r_sort_cost += 10 as LogEst;
            if n_sorted_1 != 0 { r_sort_cost += 6 as LogEst; }
            if ((*p_w_info_1).i_limit as i32) < n_row_1 as i32 {
                n_row_1 = (*p_w_info_1).i_limit;
            }
        } else if (*p_w_info_1).wctrl_flags as i32 & 256 != 0 {
            if n_row_1 as i32 > 10 {
                n_row_1 -= 10 as LogEst;
                { let _ = 0; };
            }
        }
        r_sort_cost += est_log(n_row_1) as i32 as LogEst;
        return r_sort_cost;
    }
}

///* Two WhereLoop objects, pCandidate and pBaseline, are known to have the
///* same cost.  Look deep into each to see if pCandidate is even slightly
///* better than pBaseline.  Return false if it is, if pCandidate is is preferred.
///* Return true if pBaseline is preferred or if we cannot tell the difference.
///*
///*    Result       Meaning
///*    --------     ----------------------------------------------------------
///*    true         We cannot tell the difference in pCandidate and pBaseline
///*    false        pCandidate seems like a better choice than pBaseline
extern "C" fn where_loop_is_no_better(p_candidate_1: &WhereLoop,
    p_baseline_1: &WhereLoop) -> i32 {
    unsafe {
        if (*p_candidate_1).ws_flags & 512 as u32 == 0 as u32 { return 1; }
        if (*p_baseline_1).ws_flags & 512 as u32 == 0 as u32 { return 1; }
        if (unsafe { (*(*p_candidate_1).u.btree.p_index).sz_idx_row } as i32)
                <
                unsafe { (*(*p_baseline_1).u.btree.p_index).sz_idx_row } as
                    i32 {
            return 0;
        }
        return 1;
    }
}

///* Given the list of WhereLoop objects at pWInfo->pLoops, this routine
///* attempts to find the lowest cost path that visits each WhereLoop
///* once.  This path is then loaded into the pWInfo->a[].pWLoop fields.
///*
///* Assume that the total number of output rows that will need to be sorted
///* will be nRowEst (in the 10*log2 representation).  Or, ignore sorting
///* costs if nRowEst==0.
///*
///* Return SQLITE_OK on success or SQLITE_NOMEM of a memory allocation
///* error occurs.
#[allow(unused_doc_comments)]
extern "C" fn where_path_solver(p_w_info_1: *mut WhereInfo,
    n_row_est_1: LogEst) -> i32 {
    unsafe {
        let mut mx_choice: i32 = 0;
        /// Maximum number of simultaneous paths tracked
        let mut n_loop: i32 = 0;
        /// Number of terms in the join
        let mut p_parse: *mut Parse = core::ptr::null_mut();
        /// Parsing context
        let mut i_loop: i32 = 0;
        /// Loop counter over the terms of the join
        let mut ii: i32 = 0;
        let mut jj: i32 = 0;
        /// Loop counters
        let mut mx_i: i32 = 0;
        /// Index of next entry to replace
        let mut n_order_by: i32 = 0;
        /// Number of ORDER BY clause terms
        let mut mx_cost: LogEst = 0 as LogEst;
        /// Maximum cost of a set of paths
        let mut mx_unsort: LogEst = 0 as LogEst;
        /// Maximum unsorted cost of a set of path
        let mut n_to: i32 = 0;
        let mut n_from: i32 = 0;
        /// Number of valid entries in aTo[] and aFrom[]
        let mut a_from: *mut WherePath = core::ptr::null_mut();
        /// All nFrom paths at the previous level
        let mut a_to: *mut WherePath = core::ptr::null_mut();
        /// The nTo best paths at the current level
        let mut p_from: *mut WherePath = core::ptr::null_mut();
        /// An element of aFrom[] that we are working on
        let mut p_to: *mut WherePath = core::ptr::null_mut();
        /// An element of aTo[] that we are working on
        let mut p_w_loop: *mut WhereLoop = core::ptr::null_mut();
        /// One of the WhereLoop objects
        let mut p_x: *mut *mut WhereLoop = core::ptr::null_mut();
        /// Used to divy up the pSpace memory
        let mut a_sort_cost: *mut LogEst = core::ptr::null_mut();
        /// Sorting and partial sorting costs
        let mut p_space: *mut i8 = core::ptr::null_mut();
        /// Temporary memory used by this routine
        let mut n_space: i32 = 0;

        /// Bytes of space allocated at pSpace
        (p_parse = unsafe { (*p_w_info_1).p_parse });
        n_loop = unsafe { (*p_w_info_1).n_level } as i32;
        if n_loop <= 1 {
            mx_choice = 1;
        } else if n_loop == 2 {
            mx_choice = 5;
        } else if unsafe { (*p_parse).n_err } != 0 {
            mx_choice = 1;
        } else { mx_choice = compute_mx_choice(unsafe { &mut *p_w_info_1 }); }
        { let _ = 0; };
        if unsafe { (*p_w_info_1).p_order_by } == core::ptr::null_mut() ||
                n_row_est_1 as i32 == 0 {
            n_order_by = 0;
        } else {
            n_order_by =
                unsafe { (*unsafe { (*p_w_info_1).p_order_by }).n_expr };
        }

        /// Allocate and initialize space for aTo, aFrom and aSortCost[]
        (n_space =
            ((core::mem::size_of::<WherePath>() as u64 +
                            core::mem::size_of::<*mut WhereLoop>() as u64 *
                                n_loop as u64) * mx_choice as u64 * 2 as u64) as i32);
        n_space +=
            (core::mem::size_of::<LogEst>() as u64 * n_order_by as u64) as
                i32;
        p_space =
            unsafe {
                    sqlite3_db_malloc_raw_nn(unsafe { (*p_parse).db },
                        n_space as u64)
                } as *mut i8;
        if p_space == core::ptr::null_mut() { return 7; }
        a_to = p_space as *mut WherePath;
        a_from = unsafe { a_to.offset(mx_choice as isize) };
        unsafe {
            memset(a_from as *mut (), 0,
                core::mem::size_of::<WherePath>() as u64)
        };
        p_x =
            unsafe { a_from.offset(mx_choice as isize) } as
                *mut *mut WhereLoop;
        {
            { ii = mx_choice * 2; p_from = a_to };
            '__b84: loop {
                if !(ii > 0) { break '__b84; }
                '__c84: loop {
                    unsafe { (*p_from).a_loop = p_x };
                    break '__c84;
                }
                {
                    {
                        { let __p = &mut ii; let __t = *__p; *__p -= 1; __t };
                        {
                            let __p = &mut p_from;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                    {
                        let __n = n_loop;
                        let __p = &mut p_x;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    }
                };
            }
        }
        if n_order_by != 0 {

            /// If there is an ORDER BY clause and it is not being ignored, set up
            ///* space for the aSortCost[] array. Each element of the aSortCost array
            ///* is either zero - meaning it has not yet been initialized - or the
            ///* cost of sorting nRowEst rows of data where the first X terms of
            ///* the ORDER BY clause are already in order, where X is the array
            ///* index.
            (a_sort_cost = p_x as *mut LogEst);

            /// If there is an ORDER BY clause and it is not being ignored, set up
            ///* space for the aSortCost[] array. Each element of the aSortCost array
            ///* is either zero - meaning it has not yet been initialized - or the
            ///* cost of sorting nRowEst rows of data where the first X terms of
            ///* the ORDER BY clause are already in order, where X is the array
            ///* index.
            unsafe {
                memset(a_sort_cost as *mut (), 0,
                    core::mem::size_of::<LogEst>() as u64 * n_order_by as u64)
            };
        }
        { let _ = 0; };
        { let _ = 0; };
        unsafe {

            /// Seed the search with a single WherePath containing zero WhereLoops.
            ///*
            ///* TUNING: Do not let the number of iterations go above 28.  If the cost
            ///* of computing an automatic index is not paid back within the first 28
            ///* rows, then do not use the automatic index.
            ((*a_from.offset(0 as isize)).n_row =
                if (unsafe { (*p_parse).n_query_loop } as i32) < 48 {
                        (unsafe { (*p_parse).n_query_loop }) as i32
                    } else { 48 } as LogEst)
        };
        { let _ = 0; };
        n_from = 1;
        { let _ = 0; };
        if n_order_by != 0 {
            unsafe {

                /// If nLoop is zero, then there are no FROM terms in the query. Since
                ///* in this case the query may return a maximum of one row, the results
                ///* are already in the requested order. Set isOrdered to nOrderBy to
                ///* indicate this. Or, if nLoop is greater than zero, set isOrdered to
                ///* -1, indicating that the result set may or may not be ordered,
                ///* depending on the loops added to the current plan.
                ((*a_from.offset(0 as isize)).is_ordered =
                    if n_loop > 0 { -1 } else { n_order_by } as i8)
            };
        }
        {
            i_loop = 0;
            '__b85: loop {
                if !(i_loop < n_loop) { break '__b85; }
                '__c85: loop {
                    n_to = 0;
                    {
                        { ii = 0; p_from = a_from };
                        '__b86: loop {
                            if !(ii < n_from) { break '__b86; }
                            '__c86: loop {
                                {
                                    p_w_loop = unsafe { (*p_w_info_1).p_loops };
                                    '__b87: loop {
                                        if !(!(p_w_loop).is_null()) { break '__b87; }
                                        '__c87: loop {
                                            let mut n_out: LogEst = 0 as LogEst;
                                            /// Rows visited by (pFrom+pWLoop)
                                            let mut r_cost: LogEst = 0 as LogEst;
                                            /// Cost of path (pFrom+pWLoop)
                                            let mut r_unsort: LogEst = 0 as LogEst;
                                            /// Unsorted cost of (pFrom+pWLoop)
                                            let mut is_ordered: i8 = 0 as i8;
                                            /// isOrdered for (pFrom+pWLoop)
                                            let mut mask_new: Bitmask = 0 as Bitmask;
                                            /// Mask of src visited by (..)
                                            let mut rev_mask: Bitmask = 0 as Bitmask;
                                            if unsafe { (*p_w_loop).prereq } &
                                                        !unsafe { (*p_from).mask_loop } != 0 as u64 {
                                                break '__c87;
                                            }
                                            if unsafe { (*p_w_loop).mask_self } &
                                                        unsafe { (*p_from).mask_loop } != 0 as u64 {
                                                break '__c87;
                                            }
                                            if unsafe { (*p_w_loop).ws_flags } & 16384 as u32 !=
                                                        0 as u32 && (unsafe { (*p_from).n_row } as i32) < 3 {

                                                /// Do not use an automatic index if the this loop is expected
                                                ///* to run less than 1.25 times.  It is tempting to also exclude
                                                ///* automatic index usage on an outer loop, but sometimes an automatic
                                                ///* index is useful in the outer loop of a correlated subquery.
                                                { let _ = 0; };
                                                break '__c87;
                                            }

                                            /// At this point, pWLoop is a candidate to be the next loop.
                                            ///* Compute its cost
                                            (r_unsort =
                                                (unsafe { (*p_w_loop).r_run } as i32 +
                                                        unsafe { (*p_from).n_row } as i32) as LogEst);
                                            if unsafe { (*p_w_loop).r_setup } != 0 {
                                                r_unsort =
                                                    unsafe {
                                                        sqlite3_log_est_add(unsafe { (*p_w_loop).r_setup },
                                                            r_unsort)
                                                    };
                                            }
                                            r_unsort =
                                                unsafe {
                                                    sqlite3_log_est_add(r_unsort, unsafe { (*p_from).r_unsort })
                                                };
                                            n_out =
                                                (unsafe { (*p_from).n_row } as i32 +
                                                        unsafe { (*p_w_loop).n_out } as i32) as LogEst;
                                            mask_new =
                                                unsafe { (*p_from).mask_loop } |
                                                    unsafe { (*p_w_loop).mask_self };
                                            is_ordered = unsafe { (*p_from).is_ordered };
                                            if (is_ordered as i32) < 0 {
                                                rev_mask = 0 as Bitmask;
                                                is_ordered =
                                                    where_path_satisfies_order_by(p_w_info_1,
                                                        unsafe { (*p_w_info_1).p_order_by }, unsafe { &*p_from },
                                                        unsafe { (*p_w_info_1).wctrl_flags }, i_loop as u16,
                                                        p_w_loop, &mut rev_mask);
                                            } else { rev_mask = unsafe { (*p_from).rev_loop }; }
                                            if is_ordered as i32 >= 0 &&
                                                    (is_ordered as i32) < n_order_by {
                                                if unsafe { *a_sort_cost.offset(is_ordered as isize) } as
                                                            i32 == 0 {
                                                    unsafe {
                                                        *a_sort_cost.offset(is_ordered as isize) =
                                                            where_sorting_cost(unsafe { &*p_w_info_1 }, n_row_est_1,
                                                                n_order_by, is_ordered as i32)
                                                    };
                                                }

                                                /// TUNING:  Add a small extra penalty (3) to sorting as an
                                                ///* extra encouragement to the query planner to select a plan
                                                ///* where the rows emerge in the correct order without any sorting
                                                ///* required.
                                                (r_cost =
                                                    (unsafe {
                                                                    sqlite3_log_est_add(r_unsort,
                                                                        unsafe { *a_sort_cost.offset(is_ordered as isize) })
                                                                } as i32 + 3) as LogEst);
                                            } else { r_cost = r_unsort; r_unsort -= 2 as LogEst; }
                                            {
                                                { jj = 0; p_to = a_to };
                                                '__b88: loop {
                                                    if !(jj < n_to) { break '__b88; }
                                                    '__c88: loop {
                                                        if unsafe { (*p_to).mask_loop } == mask_new &&
                                                                ((unsafe { (*p_to).is_ordered } as i32 ^ is_ordered as i32)
                                                                            & 128 == 0 || i_loop == n_loop - 1) {
                                                            break '__b88;
                                                        }
                                                        break '__c88;
                                                    }
                                                    {
                                                        { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                                                        {
                                                            let __p = &mut p_to;
                                                            let __t = *__p;
                                                            *__p = unsafe { (*__p).offset(1) };
                                                            __t
                                                        }
                                                    };
                                                }
                                            }
                                            if jj >= n_to {
                                                if n_to >= mx_choice &&
                                                        (r_cost as i32 > mx_cost as i32 ||
                                                            r_cost as i32 == mx_cost as i32 &&
                                                                r_unsort as i32 >= mx_unsort as i32) {

                                                    /// The current candidate is no better than any of the mxChoice
                                                    ///* paths currently in the best-so-far buffer.  So discard
                                                    ///* this candidate as not viable.
                                                    /// 0x4
                                                    break '__c87;
                                                }
                                                if n_to < mx_choice {

                                                    /// Increase the size of the aTo set by one
                                                    (jj =
                                                        { let __p = &mut n_to; let __t = *__p; *__p += 1; __t });
                                                } else {

                                                    /// New path replaces the prior worst to keep count below mxChoice
                                                    (jj = mx_i);
                                                }
                                                p_to = unsafe { a_to.offset(jj as isize) };
                                            } else {
                                                if (unsafe { (*p_to).r_cost } as i32) < r_cost as i32 ||
                                                                unsafe { (*p_to).r_cost } as i32 == r_cost as i32 &&
                                                                    (unsafe { (*p_to).n_row } as i32) < n_out as i32 ||
                                                            unsafe { (*p_to).r_cost } as i32 == r_cost as i32 &&
                                                                    unsafe { (*p_to).n_row } as i32 == n_out as i32 &&
                                                                (unsafe { (*p_to).r_unsort } as i32) < r_unsort as i32 ||
                                                        unsafe { (*p_to).r_cost } as i32 == r_cost as i32 &&
                                                                    unsafe { (*p_to).n_row } as i32 == n_out as i32 &&
                                                                unsafe { (*p_to).r_unsort } as i32 == r_unsort as i32 &&
                                                            where_loop_is_no_better(unsafe { &*p_w_loop },
                                                                    unsafe {
                                                                        &*unsafe {
                                                                                    *unsafe { (*p_to).a_loop.offset(i_loop as isize) }
                                                                                }
                                                                    }) != 0 {
                                                    break '__c87;
                                                }
                                            }

                                            /// pWLoop is a winner.  Add it to the set of best so far
                                            unsafe {
                                                (*p_to).mask_loop =
                                                    unsafe { (*p_from).mask_loop } |
                                                        unsafe { (*p_w_loop).mask_self }
                                            };
                                            unsafe { (*p_to).rev_loop = rev_mask };
                                            unsafe { (*p_to).n_row = n_out };
                                            unsafe { (*p_to).r_cost = r_cost };
                                            unsafe { (*p_to).r_unsort = r_unsort };
                                            unsafe { (*p_to).is_ordered = is_ordered };
                                            unsafe {
                                                memcpy(unsafe { (*p_to).a_loop } as *mut (),
                                                    unsafe { (*p_from).a_loop } as *const (),
                                                    core::mem::size_of::<*mut WhereLoop>() as u64 *
                                                        i_loop as u64)
                                            };
                                            unsafe {
                                                *unsafe { (*p_to).a_loop.offset(i_loop as isize) } =
                                                    p_w_loop
                                            };
                                            if n_to >= mx_choice {
                                                mx_i = 0;
                                                mx_cost = unsafe { (*a_to.offset(0 as isize)).r_cost };
                                                mx_unsort = unsafe { (*a_to.offset(0 as isize)).n_row };
                                                {
                                                    { jj = 1; p_to = unsafe { a_to.offset(1 as isize) } };
                                                    '__b89: loop {
                                                        if !(jj < mx_choice) { break '__b89; }
                                                        '__c89: loop {
                                                            if unsafe { (*p_to).r_cost } as i32 > mx_cost as i32 ||
                                                                    unsafe { (*p_to).r_cost } as i32 == mx_cost as i32 &&
                                                                        unsafe { (*p_to).r_unsort } as i32 > mx_unsort as i32 {
                                                                mx_cost = unsafe { (*p_to).r_cost };
                                                                mx_unsort = unsafe { (*p_to).r_unsort };
                                                                mx_i = jj;
                                                            }
                                                            break '__c89;
                                                        }
                                                        {
                                                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                                                            {
                                                                let __p = &mut p_to;
                                                                let __t = *__p;
                                                                *__p = unsafe { (*__p).offset(1) };
                                                                __t
                                                            }
                                                        };
                                                    }
                                                }
                                            }
                                            break '__c87;
                                        }
                                        p_w_loop = unsafe { (*p_w_loop).p_next_loop };
                                    }
                                }
                                break '__c86;
                            }
                            {
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                                {
                                    let __p = &mut p_from;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                            };
                        }
                    }

                    /// >=2
                    /// Swap the roles of aFrom and aTo for the next generation
                    (p_from = a_to);
                    a_to = a_from;
                    a_from = p_from;
                    n_from = n_to;
                    break '__c85;
                }
                { let __p = &mut i_loop; let __t = *__p; *__p += 1; __t };
            }
        }
        if n_from == 0 {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"no query solution".as_ptr() as *mut i8 as *const i8)
            };
            unsafe {
                sqlite3_db_free_nn(unsafe { (*p_parse).db },
                    p_space as *mut ())
            };
            return 1;
        }

        /// Only one path is available, which is the best path
        { let _ = 0; };
        p_from = a_from;
        { let _ = 0; };
        {
            i_loop = 0;
            '__b90: loop {
                if !(i_loop < n_loop) { break '__b90; }
                '__c90: loop {
                    let p_level: *mut WhereLevel =
                        unsafe {
                            (unsafe { (*p_w_info_1).a.as_ptr() } as
                                    *mut WhereLevel).offset(i_loop as isize)
                        };
                    unsafe {
                        (*p_level).p_w_loop =
                            {
                                p_w_loop =
                                    unsafe {
                                        *unsafe { (*p_from).a_loop.offset(i_loop as isize) }
                                    };
                                p_w_loop
                            }
                    };
                    unsafe { (*p_level).i_from = unsafe { (*p_w_loop).i_tab } };
                    unsafe {
                        (*p_level).i_tab_cur =
                            unsafe {
                                (*(unsafe {
                                                    (*unsafe { (*p_w_info_1).p_tab_list }).a.as_ptr()
                                                } as
                                                *mut SrcItem).add(unsafe { (*p_level).i_from } as
                                                usize)).i_cursor
                            }
                    };
                    break '__c90;
                }
                { let __p = &mut i_loop; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_w_info_1).wctrl_flags } as i32 & 256 != 0 &&
                        unsafe { (*p_w_info_1).wctrl_flags } as i32 & 128 == 0 &&
                    unsafe { (*p_w_info_1).e_distinct } as i32 == 0 &&
                n_row_est_1 != 0 {
            let mut not_used: Bitmask = 0 as Bitmask;
            let rc: i32 =
                where_path_satisfies_order_by(p_w_info_1,
                        unsafe { (*p_w_info_1).p_result_set }, unsafe { &*p_from },
                        128 as u16, (n_loop - 1) as u16,
                        unsafe {
                            *unsafe { (*p_from).a_loop.offset((n_loop - 1) as isize) }
                        }, &mut not_used) as i32;
            if rc ==
                    unsafe { (*unsafe { (*p_w_info_1).p_result_set }).n_expr } {
                unsafe { (*p_w_info_1).e_distinct = 2 as u8 };
            }
        }
        unsafe { (*p_w_info_1).set_b_ordered_inner_loop(0 as u32 as u32) };
        if !(unsafe { (*p_w_info_1).p_order_by }).is_null() {
            unsafe {
                (*p_w_info_1).n_ob_sat = unsafe { (*p_from).is_ordered }
            };
            if unsafe { (*p_w_info_1).wctrl_flags } as i32 & 128 != 0 {
                if unsafe { (*p_from).is_ordered } as i32 ==
                        unsafe { (*unsafe { (*p_w_info_1).p_order_by }).n_expr } {
                    unsafe { (*p_w_info_1).e_distinct = 2 as u8 };
                }

                /// vvv--- See check-in [12ad822d9b827777] on 2023-03-16 ---vvv
                { let _ = 0; };
            } else {
                unsafe {
                    (*p_w_info_1).rev_mask = unsafe { (*p_from).rev_loop }
                };
                if unsafe { (*p_w_info_1).n_ob_sat } as i32 <= 0 {
                    unsafe { (*p_w_info_1).n_ob_sat = 0 as i8 };
                    if n_loop > 0 {
                        let ws_flags: u32 =
                            unsafe {
                                (*unsafe {
                                                *unsafe { (*p_from).a_loop.offset((n_loop - 1) as isize) }
                                            }).ws_flags
                            };
                        if ws_flags & 4096 as u32 == 0 as u32 &&
                                ws_flags & (256 | 4) as u32 != (256 | 4) as u32 {
                            let mut m: Bitmask = 0 as Bitmask;
                            let rc: i32 =
                                where_path_satisfies_order_by(p_w_info_1,
                                        unsafe { (*p_w_info_1).p_order_by }, unsafe { &*p_from },
                                        2048 as u16, (n_loop - 1) as u16,
                                        unsafe {
                                            *unsafe { (*p_from).a_loop.offset((n_loop - 1) as isize) }
                                        }, &mut m) as i32;
                            if rc ==
                                    unsafe { (*unsafe { (*p_w_info_1).p_order_by }).n_expr } {
                                unsafe {
                                    (*p_w_info_1).set_b_ordered_inner_loop(1 as u32 as u32)
                                };
                                unsafe { (*p_w_info_1).rev_mask = m };
                            }
                        }
                    }
                } else if n_loop != 0 &&
                            unsafe { (*p_w_info_1).n_ob_sat } as i32 == 1 &&
                        unsafe { (*p_w_info_1).wctrl_flags } as i32 & (1 | 2) != 0 {
                    unsafe {
                        (*p_w_info_1).set_b_ordered_inner_loop(1 as u32 as u32)
                    };
                }
            }
            if unsafe { (*p_w_info_1).wctrl_flags } as i32 & 512 != 0 &&
                        unsafe { (*p_w_info_1).n_ob_sat } as i32 ==
                            unsafe { (*unsafe { (*p_w_info_1).p_order_by }).n_expr } &&
                    n_loop > 0 {
                let mut rev_mask_1: Bitmask = 0 as Bitmask;
                let n_order: i32 =
                    where_path_satisfies_order_by(p_w_info_1,
                            unsafe { (*p_w_info_1).p_order_by }, unsafe { &*p_from },
                            0 as u16, (n_loop - 1) as u16,
                            unsafe {
                                *unsafe { (*p_from).a_loop.offset((n_loop - 1) as isize) }
                            }, &mut rev_mask_1) as i32;
                { let _ = 0; };
                if n_order ==
                        unsafe { (*unsafe { (*p_w_info_1).p_order_by }).n_expr } {
                    unsafe { (*p_w_info_1).set_sorted(1 as u32 as u32) };
                    unsafe { (*p_w_info_1).rev_mask = rev_mask_1 };
                }
            }
        }
        unsafe { (*p_w_info_1).n_row_out = unsafe { (*p_from).n_row } };

        /// Free temporary memory and return success
        unsafe {
            sqlite3_db_free_nn(unsafe { (*p_parse).db }, p_space as *mut ())
        };
        return 0;
    }
}

///* This routine implements a heuristic designed to improve query planning.
///* This routine is called in between the first and second call to
///* wherePathSolver().  Hence the name "Interstage" "Heuristic".
///*
///* The first call to wherePathSolver() (hereafter just "solver()") computes
///* the best path without regard to the order of the outputs.  The second call
///* to the solver() builds upon the first call to try to find an alternative
///* path that satisfies the ORDER BY clause.
///*
///* This routine looks at the results of the first solver() run, and for
///* every FROM clause term in the resulting query plan that uses an equality
///* constraint against an index, disable other WhereLoops for that same
///* FROM clause term that would try to do a full-table scan.  This prevents
///* an index search from being converted into a full-table scan in order to
///* satisfy an ORDER BY clause, since even though we might get slightly better
///* performance using the full-scan without sorting if the output size
///* estimates are very precise, we might also get severe performance
///* degradation using the full-scan if the output size estimate is too large.
///* It is better to err on the side of caution.
///*
///* Except, if the first solver() call generated a full-table scan in an outer
///* loop then stop this analysis at the first full-scan, since the second
///* solver() run might try to swap that full-scan for another in order to
///* get the output into the correct order.  In other words, we allow a
///* rewrite like this:
///*
///*     First Solver()                      Second Solver()
///*       |-- SCAN t1                         |-- SCAN t2
///*       |-- SEARCH t2                       `-- SEARCH t1
///*       `-- SORT USING B-TREE
///*
///* The purpose of this routine is to disallow rewrites such as:
///*
///*     First Solver()                      Second Solver()
///*       |-- SEARCH t1                       |-- SCAN t2     <--- bad!
///*       |-- SEARCH t2                       `-- SEARCH t1
///*       `-- SORT USING B-TREE
///*
///* See test cases in test/whereN.test for the real-world query that
///* originally provoked this heuristic.
#[allow(unused_doc_comments)]
extern "C" fn where_interstage_heuristic(p_w_info_1: &WhereInfo) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b91: loop {
            if !(i < (*p_w_info_1).n_level as i32) { break '__b91; }
            '__c91: loop {
                let p: *const WhereLoop =
                    unsafe {
                            (*((*p_w_info_1).a.as_ptr() as
                                            *mut WhereLevel).offset(i as isize)).p_w_loop
                        } as *const WhereLoop;
                if p == core::ptr::null_mut() { break '__b91; }
                if unsafe { (*p).ws_flags } & 1024 as u32 != 0 as u32 {

                    /// Treat a vtab scan as similar to a full-table scan
                    break '__b91;
                }
                if unsafe { (*p).ws_flags } & (1 | 8 | 4) as u32 != 0 as u32 {
                    let i_tab: u8 = unsafe { (*p).i_tab };
                    let mut p_loop: *mut WhereLoop = core::ptr::null_mut();
                    {
                        p_loop = (*p_w_info_1).p_loops;
                        '__b92: loop {
                            if !(!(p_loop).is_null()) { break '__b92; }
                            '__c92: loop {
                                if unsafe { (*p_loop).i_tab } as i32 != i_tab as i32 {
                                    break '__c92;
                                }
                                if unsafe { (*p_loop).ws_flags } & (15 | 16384) as u32 !=
                                        0 as u32 {

                                    /// Auto-index and index-constrained loops allowed to remain
                                    break '__c92;
                                }

                                /// WHERETRACE_ENABLED
                                unsafe { (*p_loop).prereq = -1i32 as Bitmask };
                                break '__c92;
                            }
                            p_loop = unsafe { (*p_loop).p_next_loop };
                        }
                    }
                } else { break '__b91; }
                break '__c91;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Set the reverse-scan order mask to one for all tables in the query
///* with the exception of MATERIALIZED common table expressions that have
///* their own internal ORDER BY clauses.
///*
///* This implements the PRAGMA reverse_unordered_selects=ON setting.
///* (Also SQLITE_DBCONFIG_REVERSE_SCANORDER).
extern "C" fn where_reverse_scan_order(p_w_info_1: &mut WhereInfo) -> () {
    unsafe {
        let mut ii: i32 = 0;
        {
            ii = 0;
            '__b93: loop {
                if !(ii < unsafe { (*(*p_w_info_1).p_tab_list).n_src }) {
                    break '__b93;
                }
                '__c93: loop {
                    let p_item: *const SrcItem =
                        unsafe {
                                &raw mut *(unsafe { (*(*p_w_info_1).p_tab_list).a.as_ptr() }
                                                as *mut SrcItem).offset(ii as isize)
                            } as *const SrcItem;
                    if (unsafe { (*p_item).fg.is_cte() } == 0) as i32 != 0 ||
                                    unsafe { (*unsafe { (*p_item).u2.p_cte_use }).e_m10d } as
                                            i32 != 0 ||
                                unsafe { (*p_item).fg.is_subquery() } as i32 == 0 ||
                            unsafe {
                                    (*unsafe {
                                                    (*unsafe { (*p_item).u4.p_subq }).p_select
                                                }).p_order_by
                                } == core::ptr::null_mut() {
                        (*p_w_info_1).rev_mask |= (1 as Bitmask) << ii;
                    }
                    break '__c93;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

/// Attempt to omit tables from a join that do not affect the result.
///* For a table to not affect the result, the following must be true:
///*
///*   1) The query must not be an aggregate.
///*   2) The table must be the RHS of a LEFT JOIN.
///*   3) Either the query must be DISTINCT, or else the ON or USING clause
///*      must contain a constraint that limits the scan of the table to
///*      at most a single row.
///*   4) The table must not be referenced by any part of the query apart
///*      from its own USING or ON clause.
///*   5) The table must not have an inner-join ON or USING clause if there is
///*      a RIGHT JOIN anywhere in the query.  Otherwise the ON/USING clause
///*      might move from the right side to the left side of the RIGHT JOIN.
///*      Note: Due to (2), this condition can only arise if the table is
///*      the right-most table of a subquery that was flattened into the
///*      main query and that subquery was the right-hand operand of an
///*      inner join that held an ON or USING clause.
///*   6) The ORDER BY clause has 63 or fewer terms
///*   7) The omit-noop-join optimization is enabled.
///*
///* Items (1), (6), and (7) are checked by the caller.
///*
///* For example, given:
///*
///*     CREATE TABLE t1(ipk INTEGER PRIMARY KEY, v1);
///*     CREATE TABLE t2(ipk INTEGER PRIMARY KEY, v2);
///*     CREATE TABLE t3(ipk INTEGER PRIMARY KEY, v3);
///*
///* then table t2 can be omitted from the following:
///*
///*     SELECT v1, v3 FROM t1
///*       LEFT JOIN t2 ON (t1.ipk=t2.ipk)
///*       LEFT JOIN t3 ON (t1.ipk=t3.ipk)
///*
///* or from:
///*
///*     SELECT DISTINCT v1, v3 FROM t1
///*       LEFT JOIN t2
///*       LEFT JOIN t3 ON (t1.ipk=t3.ipk)
#[allow(unused_doc_comments)]
extern "C" fn where_omit_noop_join(p_w_info_1: &mut WhereInfo,
    mut not_ready_1: Bitmask) -> Bitmask {
    unsafe {
        let mut i: i32 = 0;
        let mut tab_used: Bitmask = 0 as Bitmask;
        let mut has_right_join: i32 = 0;

        /// Preconditions checked by the caller
        { let _ = 0; };
        { let _ = 0; };

        /// These two preconditions checked by the caller combine to guarantee
        ///* condition (1) of the header comment
        { let _ = 0; };
        { let _ = 0; };
        tab_used =
            unsafe {
                sqlite3_where_expr_list_usage(&mut (*p_w_info_1).s_mask_set,
                    (*p_w_info_1).p_result_set)
            };
        if !((*p_w_info_1).p_order_by).is_null() {
            tab_used |=
                unsafe {
                    sqlite3_where_expr_list_usage(&mut (*p_w_info_1).s_mask_set,
                        (*p_w_info_1).p_order_by)
                };
        }
        has_right_join =
            (unsafe {
                                (*(unsafe { (*(*p_w_info_1).p_tab_list).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).fg.jointype
                            } as i32 & 64 != 0) as i32;
        {
            i = (*p_w_info_1).n_level as i32 - 1;
            '__b94: loop {
                if !(i >= 1) { break '__b94; }
                '__c94: loop {
                    let mut p_term: *mut WhereTerm = core::ptr::null_mut();
                    let mut p_end: *mut WhereTerm = core::ptr::null_mut();
                    let mut p_item: *const SrcItem = core::ptr::null();
                    let mut p_loop: *const WhereLoop = core::ptr::null();
                    let mut m1: Bitmask = 0 as Bitmask;
                    p_loop =
                        unsafe {
                            (*((*p_w_info_1).a.as_ptr() as
                                            *mut WhereLevel).offset(i as isize)).p_w_loop
                        };
                    p_item =
                        unsafe {
                            &mut *(unsafe { (*(*p_w_info_1).p_tab_list).a.as_ptr() } as
                                            *mut SrcItem).add(unsafe { (*p_loop).i_tab } as usize)
                        };
                    if unsafe { (*p_item).fg.jointype } as i32 & (8 | 16) != 8 {
                        break '__c94;
                    }
                    if (*p_w_info_1).wctrl_flags as i32 & 256 == 0 &&
                            unsafe { (*p_loop).ws_flags } & 4096 as u32 == 0 as u32 {
                        break '__c94;
                    }
                    if tab_used & unsafe { (*p_loop).mask_self } != 0 as u64 {
                        break '__c94;
                    }
                    p_end =
                        unsafe {
                            (*p_w_info_1).s_wc.a.offset((*p_w_info_1).s_wc.n_term as
                                    isize)
                        };
                    {
                        p_term = (*p_w_info_1).s_wc.a;
                        '__b95: loop {
                            if !(p_term < p_end) { break '__b95; }
                            '__c95: loop {
                                if unsafe { (*p_term).prereq_all } &
                                            unsafe { (*p_loop).mask_self } != 0 as u64 {
                                    if !(unsafe { (*unsafe { (*p_term).p_expr }).flags } &
                                                                1 as u32 != 0 as u32) as i32 != 0 ||
                                            unsafe { (*unsafe { (*p_term).p_expr }).w.i_join } !=
                                                unsafe { (*p_item).i_cursor } {
                                        break '__b95;
                                    }
                                }
                                if has_right_join != 0 &&
                                            unsafe { (*unsafe { (*p_term).p_expr }).flags } & 2 as u32
                                                != 0 as u32 &&
                                        unsafe { (*unsafe { (*p_term).p_expr }).w.i_join } ==
                                            unsafe { (*p_item).i_cursor } {
                                    break '__b95;
                                }
                                break '__c95;
                            }
                            {
                                let __p = &mut p_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                    }
                    if p_term < p_end { break '__c94; }
                    m1 = ((1 as Bitmask) << i) - 1 as Bitmask;
                    (*p_w_info_1).rev_mask =
                        m1 & (*p_w_info_1).rev_mask |
                            (*p_w_info_1).rev_mask >> 1 & !m1;
                    not_ready_1 &= !unsafe { (*p_loop).mask_self };
                    {
                        p_term = (*p_w_info_1).s_wc.a;
                        '__b96: loop {
                            if !(p_term < p_end) { break '__b96; }
                            '__c96: loop {
                                if unsafe { (*p_term).prereq_all } &
                                            unsafe { (*p_loop).mask_self } != 0 as u64 {
                                    unsafe { (*p_term).wt_flags |= 4 as u16 };
                                    unsafe { (*p_term).prereq_all = 0 as Bitmask };
                                }
                                break '__c96;
                            }
                            {
                                let __p = &mut p_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                    }
                    if i != (*p_w_info_1).n_level as i32 - 1 {
                        let n_byte: i32 =
                            (((*p_w_info_1).n_level as i32 - 1 - i) as u64 *
                                    core::mem::size_of::<WhereLevel>() as u64) as i32;
                        unsafe {
                            memmove(unsafe {
                                        &raw mut *((*p_w_info_1).a.as_ptr() as
                                                        *mut WhereLevel).offset(i as isize)
                                    } as *mut (),
                                unsafe {
                                        &raw mut *((*p_w_info_1).a.as_ptr() as
                                                        *mut WhereLevel).offset((i + 1) as isize)
                                    } as *const (), n_byte as u64)
                        };
                    }
                    {
                        let __p = &mut (*p_w_info_1).n_level;
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                    { let _ = 0; };
                    break '__c94;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        return not_ready_1;
    }
}

///* Check to see if there are any SEARCH loops that might benefit from
///* using a Bloom filter.  Consider a Bloom filter if:
///*
///*   (1)  The SEARCH happens more than N times where N is the number
///*        of rows in the table that is being considered for the Bloom
///*        filter.
///*   (2)  Some searches are expected to find zero rows.  (This is determined
///*        by the WHERE_SELFCULL flag on the term.)
///*   (3)  Bloom-filter processing is not disabled.  (Checked by the
///*        caller.)
///*   (4)  The size of the table being searched is known by ANALYZE.
///*
///* This block of code merely checks to see if a Bloom filter would be
///* appropriate, and if so sets the WHERE_BLOOMFILTER flag on the
///* WhereLoop.  The implementation of the Bloom filter comes further
///* down where the code for each WhereLoop is generated.
extern "C" fn where_check_if_bloom_filter_is_useful(p_w_info_1: &WhereInfo)
    -> () {
    let mut i: i32 = 0;
    let mut n_search: LogEst = 0 as LogEst;
    { let _ = 0; };
    { let _ = 0; };
    {
        i = 0;
        '__b97: loop {
            if !(i < (*p_w_info_1).n_level as i32) { break '__b97; }
            '__c97: loop {
                let p_loop: *mut WhereLoop =
                    unsafe {
                        (*((*p_w_info_1).a.as_ptr() as
                                        *const WhereLevel).offset(i as isize)).p_w_loop
                    };
                let req_flags: u32 = (8388608 | 1) as u32;
                let p_item: *const SrcItem =
                    unsafe {
                            &raw mut *(unsafe { (*(*p_w_info_1).p_tab_list).a.as_ptr() }
                                            as *mut SrcItem).add(unsafe { (*p_loop).i_tab } as usize)
                        } as *const SrcItem;
                let p_tab: *mut Table = unsafe { (*p_item).p_s_tab };
                if unsafe { (*p_tab).tab_flags } & 16 as u32 == 0 as u32 {
                    break '__b97;
                }
                unsafe { (*p_tab).tab_flags |= 256 as u32 };
                if i >= 1 &&
                            unsafe { (*p_loop).ws_flags } & req_flags as u32 ==
                                req_flags &&
                        unsafe { (*p_loop).ws_flags } & (256 | 512) as u32 !=
                            0 as u32 {
                    if n_search as i32 >
                            unsafe { (*p_tab).n_row_log_est } as i32 {
                        unsafe { (*p_loop).ws_flags |= 4194304 as u32 };
                        unsafe { (*p_loop).ws_flags &= !64 as u32 };
                    }
                }
                n_search += unsafe { (*p_loop).n_out } as i32 as LogEst;
                break '__c97;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* The index pIdx is used by a query and contains one or more expressions.
///* In other words pIdx is an index on an expression.  iIdxCur is the cursor
///* number for the index and iDataCur is the cursor number for the corresponding
///* table.
///*
///* This routine adds IndexedExpr entries to the Parse->pIdxEpr field for
///* each of the expressions in the index so that the expression code generator
///* will know to replace occurrences of the indexed expression with
///* references to the corresponding column of the index.
extern "C" fn where_add_indexed_expr(p_parse_1: *mut Parse,
    p_idx_1: *mut Index, i_idx_cur_1: i32, p_tab_item_1: &SrcItem) -> () {
    let mut i: i32 = 0;
    let mut p: *mut IndexedExpr = core::ptr::null_mut();
    let mut p_tab: *mut Table = core::ptr::null_mut();
    { let _ = 0; };
    p_tab = unsafe { (*p_idx_1).p_table };
    {
        i = 0;
        '__b98: loop {
            if !(i < unsafe { (*p_idx_1).n_column } as i32) { break '__b98; }
            '__c98: loop {
                let mut p_expr: *mut Expr = core::ptr::null_mut();
                let j: i32 =
                    unsafe {
                            *unsafe { (*p_idx_1).ai_column.offset(i as isize) }
                        } as i32;
                if j == -2 {
                    p_expr =
                        unsafe {
                            (*(unsafe { (*unsafe { (*p_idx_1).a_col_expr }).a.as_ptr() }
                                            as *mut ExprListItem).offset(i as isize)).p_expr
                        };
                } else if j >= 0 &&
                        unsafe {
                                        (*unsafe { (*p_tab).a_col.offset(j as isize) }).col_flags
                                    } as i32 & 32 != 0 {
                    p_expr =
                        unsafe {
                            sqlite3_column_expr(p_tab,
                                unsafe {
                                    &mut *unsafe { (*p_tab).a_col.offset(j as isize) }
                                })
                        };
                } else { break '__c98; }
                if unsafe {
                            sqlite3_expr_is_constant(core::ptr::null_mut(), p_expr)
                        } != 0 {
                    break '__c98;
                }
                p =
                    unsafe {
                            sqlite3_db_malloc_raw(unsafe { (*p_parse_1).db },
                                core::mem::size_of::<IndexedExpr>() as u64)
                        } as *mut IndexedExpr;
                if p == core::ptr::null_mut() { break '__b98; }
                unsafe { (*p).p_ie_next = unsafe { (*p_parse_1).p_idx_epr } };
                unsafe {
                    (*p).p_expr =
                        unsafe {
                            sqlite3_expr_dup(unsafe { (*p_parse_1).db },
                                p_expr as *const Expr, 0)
                        }
                };
                unsafe { (*p).i_data_cur = (*p_tab_item_1).i_cursor };
                unsafe { (*p).i_idx_cur = i_idx_cur_1 };
                unsafe { (*p).i_idx_col = i };
                unsafe {
                    (*p).b_maybe_null_row =
                        ((*p_tab_item_1).fg.jointype as i32 & (8 | 64 | 16) != 0) as
                            u8
                };
                if !(unsafe {
                                    sqlite3_index_affinity_str(unsafe { (*p_parse_1).db },
                                        p_idx_1)
                                }).is_null() {
                    unsafe {
                        (*p).aff =
                            unsafe {
                                    *unsafe { (*p_idx_1).z_col_aff.offset(i as isize) }
                                } as u8
                    };
                }
                unsafe { (*p_parse_1).p_idx_epr = p };
                if unsafe { (*p).p_ie_next } == core::ptr::null_mut() {
                    let p_arg: *mut () =
                        unsafe { &raw mut (*p_parse_1).p_idx_epr } as *mut ();
                    unsafe {
                        sqlite3_parser_add_cleanup(p_parse_1,
                            Some(where_indexed_expr_cleanup), p_arg)
                    };
                }
                break '__c98;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

/// Allocate memory that is automatically freed when pWInfo is freed.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_malloc(p_w_info_1: &mut WhereInfo,
    n_byte_1: u64) -> *mut () {
    unsafe {
        let mut p_block: *mut WhereMemBlock = core::ptr::null_mut();
        p_block =
            unsafe {
                    sqlite3_db_malloc_raw_nn(unsafe {
                            (*(*p_w_info_1).p_parse).db
                        }, n_byte_1 + core::mem::size_of::<WhereMemBlock>() as u64)
                } as *mut WhereMemBlock;
        if !(p_block).is_null() {
            unsafe { (*p_block).p_next = (*p_w_info_1).p_mem_to_free };
            unsafe { (*p_block).sz = n_byte_1 };
            (*p_w_info_1).p_mem_to_free = p_block;
            {
                let __p = &mut p_block;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        return p_block as *mut ();
    }
}

///* Convert OP_Column opcodes to OP_Copy in previously generated code.
///*
///* This routine runs over generated VDBE code and translates OP_Column
///* opcodes into OP_Copy when the table is being accessed via co-routine
///* instead of via table lookup.
///*
///* If the iAutoidxCur is not zero, then any OP_Rowid instructions on
///* cursor iTabCur are transformed into OP_Sequence opcode for the
///* iAutoidxCur cursor, in order to generate unique rowids for the
///* automatic index being generated.
extern "C" fn translate_column_to_copy(p_parse_1: &Parse, mut i_start_1: i32,
    i_tab_cur_1: i32, i_register_1: i32, i_autoidx_cur_1: i32) -> () {
    let v: *mut Vdbe = (*p_parse_1).p_vdbe;
    let mut p_op: *mut VdbeOp = unsafe { sqlite3_vdbe_get_op(v, i_start_1) };
    let i_end: i32 = unsafe { sqlite3_vdbe_current_addr(v) };
    if unsafe { (*(*p_parse_1).db).malloc_failed } != 0 { return; }
    {
        '__b99: loop {
            if !(i_start_1 < i_end) { break '__b99; }
            '__c99: loop {
                if unsafe { (*p_op).p1 } != i_tab_cur_1 { break '__c99; }
                if unsafe { (*p_op).opcode } as i32 == 96 {
                    unsafe { (*p_op).opcode = 82 as u8 };
                    unsafe {
                        (*p_op).p1 = unsafe { (*p_op).p2 } + i_register_1
                    };
                    unsafe { (*p_op).p2 = unsafe { (*p_op).p3 } };
                    unsafe { (*p_op).p3 = 0 };
                    unsafe { (*p_op).p5 = 2 as u16 };
                } else if unsafe { (*p_op).opcode } as i32 == 137 {
                    unsafe { (*p_op).opcode = 128 as u8 };
                    unsafe { (*p_op).p1 = i_autoidx_cur_1 };
                }
                break '__c99;
            }
            {
                { let __p = &mut i_start_1; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut p_op;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
}

///* Generate code to construct the Index object for an automatic index
///* and to set up the WhereLevel object pLevel so that the code generator
///* makes use of the automatic index.
#[allow(unused_doc_comments)]
extern "C" fn construct_automatic_index(p_parse_1: *mut Parse,
    p_wc_1: &WhereClause, not_ready_1: Bitmask, p_level_1: *mut WhereLevel)
    -> () {
    unsafe {
        unsafe {
            let mut n_key_col: i32 = 0;
            /// Number of columns in the constructed index
            let mut p_term: *mut WhereTerm = core::ptr::null_mut();
            /// A single term of the WHERE clause
            let mut p_wc_end: *mut WhereTerm = core::ptr::null_mut();
            /// End of pWC->a[]
            let mut p_idx: *mut Index = core::ptr::null_mut();
            /// Object describing the transient index
            let mut v: *mut Vdbe = core::ptr::null_mut();
            /// Prepared statement under construction
            let mut addr_init: i32 = 0;
            /// Address of the initialization bypass jump
            let mut p_table: *mut Table = core::ptr::null_mut();
            /// The table being indexed
            let mut addr_top: i32 = 0;
            /// Top of the index fill loop
            let mut reg_record: i32 = 0;
            /// Register holding an index record
            let mut n: i32 = 0;
            /// Column counter
            let mut i: i32 = 0;
            /// Loop counter
            let mut mx_bit_col: i32 = 0;
            /// Maximum column in pSrc->colUsed
            let mut p_coll: *const CollSeq = core::ptr::null();
            /// Collating sequence to on a column
            let mut p_loop: *mut WhereLoop = core::ptr::null_mut();
            /// The Loop object
            let mut z_not_used: *mut i8 = core::ptr::null_mut();
            /// Extra space on the end of pIdx
            let mut idx_cols: Bitmask = 0 as Bitmask;
            /// Bitmap of columns used for indexing
            let mut extra_cols: Bitmask = 0 as Bitmask;
            /// Bitmap of additional columns
            let mut sent_warning: u8 = 0 as u8;
            /// True if a warning has been issued
            let mut use_bloom_filter: u8 = 0 as u8;
            /// True to also add a Bloom filter
            let mut p_partial: *mut Expr = core::ptr::null_mut();
            /// Partial Index Expression
            let mut i_continue: i32 = 0;
            /// Jump here to skip excluded rows
            let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
            /// The complete FROM clause
            let mut p_src: *mut SrcItem = core::ptr::null_mut();
            /// The FROM clause term to get the next index
            let mut addr_counter: i32 = 0;
            /// Address where integer counter is initialized
            let mut reg_base: i32 = 0;
            /// Array of registers where record is assembled
            /// Generate code to skip over the creation and initialization of the
            ///* transient index on 2nd and subsequent iterations of the loop.
            /// Count the number of columns that will be added to the index
            ///* and used to match WHERE clause constraints
            let mut p_expr: *mut Expr = core::ptr::null_mut();
            /// Make the automatic index a partial index if there are terms in the
            ///* WHERE clause (or the ON clause of a LEFT join) that constrain which
            ///* rows of the target table (pSrc) that can be used.
            let mut i_col: i32 = 0;
            let mut c_mask: Bitmask = 0 as Bitmask;
            /// Count the number of additional columns needed to create a
            ///* covering index.  A "covering index" is an index that contains all
            ///* columns that are needed by the query.  With a covering index, the
            ///* original table never needs to be accessed.  Automatic indices must
            ///* be a covering index because the index will not be updated if the
            ///* original table changes and the index and table cannot both be used
            ///* if they go out of sync.
            /// For WITHOUT ROWID tables, ensure that all PRIMARY KEY columns are
            ///* either in the idxCols mask or in the extraCols mask
            /// Construct the Index object to describe this index
            /// ^-- This guarantees that the number of index columns will fit in the u16
            let mut i_col_1: i32 = 0;
            let mut c_mask_1: Bitmask = 0 as Bitmask;
            let mut p_x: *const Expr = core::ptr::null();
            /// TH3 collate01.800
            /// TUNING: only use a Bloom filter on an automatic index
            ///* if one or more key columns has the ability to hold numeric
            ///* values, since strings all have the same hash in the Bloom
            ///* filter implementation and hence a Bloom filter on a text column
            ///* is not usually helpful.
            /// Add additional columns needed to make the automatic index into
            ///* a covering index
            /// Create the automatic index
            /// Fill the automatic index with content
            let mut reg_yield: i32 = 0;
            let mut p_subq: *const Subquery = core::ptr::null();
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s101:
                    {
                    match __state {
                        0 => { __state = 3; }
                        2 => {
                            unsafe {
                                sqlite3_expr_delete(unsafe { (*p_parse_1).db }, p_partial)
                            };
                            __state = 1;
                        }
                        3 => { __state = 4; }
                        4 => { __state = 5; }
                        5 => { __state = 6; }
                        6 => { __state = 7; }
                        7 => { __state = 8; }
                        8 => { __state = 9; }
                        9 => { __state = 10; }
                        10 => { __state = 11; }
                        11 => { __state = 12; }
                        12 => { __state = 13; }
                        13 => { __state = 14; }
                        14 => { __state = 15; }
                        15 => { __state = 16; }
                        16 => { __state = 17; }
                        17 => { __state = 18; }
                        18 => { __state = 19; }
                        19 => { sent_warning = 0 as u8; __state = 20; }
                        20 => { use_bloom_filter = 0 as u8; __state = 21; }
                        21 => { p_partial = core::ptr::null_mut(); __state = 22; }
                        22 => { i_continue = 0; __state = 23; }
                        23 => { __state = 24; }
                        24 => { __state = 25; }
                        25 => { addr_counter = 0; __state = 26; }
                        26 => { __state = 27; }
                        27 => { v = unsafe { (*p_parse_1).p_vdbe }; __state = 28; }
                        28 => { { let _ = 0; }; __state = 29; }
                        29 => {
                            addr_init = unsafe { sqlite3_vdbe_add_op0(v, 15) };
                            __state = 30;
                        }
                        30 => { __state = 31; }
                        31 => { n_key_col = 0; __state = 32; }
                        32 => {
                            p_tab_list = unsafe { (*(*p_wc_1).p_w_info).p_tab_list };
                            __state = 33;
                        }
                        33 => {
                            p_src =
                                unsafe {
                                    &mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                    *mut SrcItem).add(unsafe { (*p_level_1).i_from } as usize)
                                };
                            __state = 34;
                        }
                        34 => {
                            p_table = unsafe { (*p_src).p_s_tab };
                            __state = 35;
                        }
                        35 => {
                            p_wc_end =
                                unsafe { (*p_wc_1).a.offset((*p_wc_1).n_term as isize) };
                            __state = 36;
                        }
                        36 => {
                            p_loop = unsafe { (*p_level_1).p_w_loop };
                            __state = 37;
                        }
                        37 => { idx_cols = 0 as Bitmask; __state = 38; }
                        38 => { p_term = (*p_wc_1).a; __state = 40; }
                        39 => { { let _ = 0; }; __state = 61; }
                        40 => {
                            if p_term < p_wc_end {
                                __state = 41;
                            } else { __state = 39; }
                        }
                        41 => {
                            p_expr = unsafe { (*p_term).p_expr };
                            __state = 43;
                        }
                        42 => {
                            {
                                let __p = &mut p_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            __state = 40;
                        }
                        43 => {
                            if unsafe { (*p_term).wt_flags } as i32 & 2 == 0 &&
                                    unsafe {
                                            sqlite3_expr_is_single_table_constraint(p_expr,
                                                p_tab_list as *const SrcList,
                                                unsafe { (*p_level_1).i_from } as i32, 0)
                                        } != 0 {
                                __state = 45;
                            } else { __state = 44; }
                        }
                        44 => {
                            if term_can_drive_index(p_term as *const WhereTerm,
                                        p_src as *const SrcItem, not_ready_1 as Bitmask) != 0 {
                                __state = 46;
                            } else { __state = 42; }
                        }
                        45 => {
                            p_partial =
                                unsafe {
                                    sqlite3_expr_and(p_parse_1, p_partial,
                                        unsafe {
                                            sqlite3_expr_dup(unsafe { (*p_parse_1).db },
                                                p_expr as *const Expr, 0)
                                        })
                                };
                            __state = 44;
                        }
                        46 => { __state = 47; }
                        47 => { __state = 48; }
                        48 => { { let _ = 0; }; __state = 49; }
                        49 => {
                            i_col = unsafe { (*p_term).u.x.left_column };
                            __state = 50;
                        }
                        50 => {
                            c_mask =
                                if i_col >=
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                                    (1 as Bitmask) <<
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                            1
                                } else { (1 as Bitmask) << i_col };
                            __state = 51;
                        }
                        51 => { __state = 52; }
                        52 => { __state = 53; }
                        53 => {
                            if (sent_warning == 0) as i32 != 0 {
                                __state = 55;
                            } else { __state = 54; }
                        }
                        54 => {
                            if idx_cols & c_mask == 0 as u64 {
                                __state = 57;
                            } else { __state = 42; }
                        }
                        55 => {
                            unsafe {
                                sqlite3_log(28 | 1 << 8,
                                    c"automatic index on %s(%s)".as_ptr() as *mut i8 as
                                        *const i8, unsafe { (*p_table).z_name },
                                    unsafe {
                                        (*unsafe {
                                                    (*p_table).a_col.offset(i_col as isize)
                                                }).z_cn_name
                                    })
                            };
                            __state = 56;
                        }
                        56 => { sent_warning = 1 as u8; __state = 54; }
                        57 => {
                            if unsafe {
                                        where_loop_resize(unsafe { (*p_parse_1).db },
                                            unsafe { &mut *p_loop }, n_key_col + 1)
                                    } != 0 {
                                __state = 59;
                            } else { __state = 58; }
                        }
                        58 => {
                            unsafe {
                                *unsafe {
                                            (*p_loop).a_l_term.offset({
                                                        let __p = &mut n_key_col;
                                                        let __t = *__p;
                                                        *__p += 1;
                                                        __t
                                                    } as isize)
                                        } = p_term
                            };
                            __state = 60;
                        }
                        59 => { __state = 2; }
                        60 => { idx_cols |= c_mask; __state = 42; }
                        61 => {
                            unsafe {
                                (*p_loop).u.btree.n_eq =
                                    {
                                        unsafe { (*p_loop).n_l_term = n_key_col as u16 };
                                        unsafe { (*p_loop).n_l_term }
                                    }
                            };
                            __state = 62;
                        }
                        62 => {
                            unsafe {
                                (*p_loop).ws_flags = (1 | 64 | 512 | 16384) as u32
                            };
                            __state = 63;
                        }
                        63 => {
                            if unsafe { (*p_table).e_tab_type } as i32 == 2 {
                                __state = 65;
                            } else { __state = 66; }
                        }
                        64 => {
                            if !(unsafe { (*p_table).tab_flags } & 128 as u32 ==
                                                0 as u32) as i32 != 0 {
                                __state = 68;
                            } else { __state = 67; }
                        }
                        65 => {
                            extra_cols = -1i32 as Bitmask & !idx_cols;
                            __state = 64;
                        }
                        66 => {
                            extra_cols =
                                unsafe { (*p_src).col_used } &
                                    (!idx_cols |
                                        (1 as Bitmask) <<
                                            (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                                1);
                            __state = 64;
                        }
                        67 => {
                            mx_bit_col =
                                if (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as
                                                i32 - 1 < unsafe { (*p_table).n_col } as i32 {
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                        1
                                } else { (unsafe { (*p_table).n_col }) as i32 };
                            __state = 79;
                        }
                        68 => { i = 0; __state = 69; }
                        69 => {
                            if i < unsafe { (*p_table).n_col } as i32 {
                                __state = 70;
                            } else { __state = 67; }
                        }
                        70 => {
                            if unsafe {
                                                (*unsafe { (*p_table).a_col.offset(i as isize) }).col_flags
                                            } as i32 & 1 == 0 {
                                __state = 73;
                            } else { __state = 72; }
                        }
                        71 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 69;
                        }
                        72 => {
                            if i >=
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                        1 {
                                __state = 75;
                            } else { __state = 74; }
                        }
                        73 => { __state = 71; }
                        74 => {
                            if idx_cols & (1 as Bitmask) << i != 0 {
                                __state = 78;
                            } else { __state = 77; }
                        }
                        75 => {
                            extra_cols |=
                                (1 as Bitmask) <<
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                        1;
                            __state = 76;
                        }
                        76 => { __state = 67; }
                        77 => { extra_cols |= (1 as Bitmask) << i; __state = 71; }
                        78 => { __state = 71; }
                        79 => { __state = 80; }
                        80 => { __state = 81; }
                        81 => { i = 0; __state = 83; }
                        82 => {
                            if unsafe { (*p_src).col_used } &
                                        (1 as Bitmask) <<
                                            (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                                1 != 0 {
                                __state = 88;
                            } else { __state = 87; }
                        }
                        83 => {
                            if i < mx_bit_col { __state = 84; } else { __state = 82; }
                        }
                        84 => {
                            if extra_cols & (1 as Bitmask) << i != 0 {
                                __state = 86;
                            } else { __state = 85; }
                        }
                        85 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 83;
                        }
                        86 => {
                            {
                                let __p = &mut n_key_col;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            __state = 85;
                        }
                        87 => { { let _ = 0; }; __state = 89; }
                        88 => {
                            n_key_col +=
                                unsafe { (*p_table).n_col } as i32 -
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 +
                                    1;
                            __state = 87;
                        }
                        89 => {
                            p_idx =
                                unsafe {
                                    sqlite3_allocate_index_object(unsafe { (*p_parse_1).db },
                                        n_key_col +
                                            (unsafe { (*p_table).tab_flags } & 128 as u32 == 0 as u32)
                                                as i32, 0, &mut z_not_used)
                                };
                            __state = 90;
                        }
                        90 => {
                            if p_idx == core::ptr::null_mut() {
                                __state = 92;
                            } else { __state = 91; }
                        }
                        91 => {
                            unsafe { (*p_loop).u.btree.p_index = p_idx };
                            __state = 93;
                        }
                        92 => { __state = 2; }
                        93 => {
                            unsafe {
                                (*p_idx).z_name = c"auto-index".as_ptr() as *mut i8
                            };
                            __state = 94;
                        }
                        94 => {
                            unsafe { (*p_idx).p_table = p_table };
                            __state = 95;
                        }
                        95 => { n = 0; __state = 96; }
                        96 => { idx_cols = 0 as Bitmask; __state = 97; }
                        97 => { p_term = (*p_wc_1).a; __state = 99; }
                        98 => { { let _ = 0; }; __state = 119; }
                        99 => {
                            if p_term < p_wc_end {
                                __state = 100;
                            } else { __state = 98; }
                        }
                        100 => {
                            if term_can_drive_index(p_term as *const WhereTerm,
                                        p_src as *const SrcItem, not_ready_1 as Bitmask) != 0 {
                                __state = 102;
                            } else { __state = 101; }
                        }
                        101 => {
                            {
                                let __p = &mut p_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                            __state = 99;
                        }
                        102 => { __state = 103; }
                        103 => { __state = 104; }
                        104 => { { let _ = 0; }; __state = 105; }
                        105 => {
                            i_col_1 = unsafe { (*p_term).u.x.left_column };
                            __state = 106;
                        }
                        106 => {
                            c_mask_1 =
                                if i_col_1 >=
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                                    (1 as Bitmask) <<
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                            1
                                } else { (1 as Bitmask) << i_col_1 };
                            __state = 107;
                        }
                        107 => { __state = 108; }
                        108 => { __state = 109; }
                        109 => {
                            if idx_cols & c_mask_1 == 0 as u64 {
                                __state = 110;
                            } else { __state = 101; }
                        }
                        110 => {
                            p_x = unsafe { (*p_term).p_expr } as *const Expr;
                            __state = 111;
                        }
                        111 => { idx_cols |= c_mask_1; __state = 112; }
                        112 => {
                            unsafe {
                                *unsafe { (*p_idx).ai_column.offset(n as isize) } =
                                    unsafe { (*p_term).u.x.left_column } as i16
                            };
                            __state = 113;
                        }
                        113 => {
                            p_coll =
                                unsafe {
                                    sqlite3_expr_compare_coll_seq(p_parse_1, p_x as *const Expr)
                                };
                            __state = 114;
                        }
                        114 => { { let _ = 0; }; __state = 115; }
                        115 => {
                            unsafe {
                                *unsafe { (*p_idx).az_coll.offset(n as isize) } =
                                    if !(p_coll).is_null() {
                                        (unsafe { (*p_coll).z_name }) as *const i8
                                    } else { sqlite3_str_binary.as_ptr() as *const i8 }
                            };
                            __state = 116;
                        }
                        116 => {
                            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                            __state = 117;
                        }
                        117 => {
                            if unsafe { (*p_x).p_left } != core::ptr::null_mut() &&
                                    unsafe {
                                                sqlite3_expr_affinity(unsafe { (*p_x).p_left } as
                                                        *const Expr)
                                            } as i32 != 66 {
                                __state = 118;
                            } else { __state = 101; }
                        }
                        118 => { use_bloom_filter = 1 as u8; __state = 101; }
                        119 => { i = 0; __state = 121; }
                        120 => {
                            if unsafe { (*p_src).col_used } &
                                        (1 as Bitmask) <<
                                            (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                                1 != 0 {
                                __state = 128;
                            } else { __state = 127; }
                        }
                        121 => {
                            if i < mx_bit_col { __state = 122; } else { __state = 120; }
                        }
                        122 => {
                            if extra_cols & (1 as Bitmask) << i != 0 {
                                __state = 124;
                            } else { __state = 123; }
                        }
                        123 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 121;
                        }
                        124 => {
                            unsafe {
                                *unsafe { (*p_idx).ai_column.offset(n as isize) } = i as i16
                            };
                            __state = 125;
                        }
                        125 => {
                            unsafe {
                                *unsafe { (*p_idx).az_coll.offset(n as isize) } =
                                    sqlite3_str_binary.as_ptr() as *const i8
                            };
                            __state = 126;
                        }
                        126 => {
                            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                            __state = 123;
                        }
                        127 => { { let _ = 0; }; __state = 134; }
                        128 => {
                            i =
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                    1;
                            __state = 129;
                        }
                        129 => {
                            if i < unsafe { (*p_table).n_col } as i32 {
                                __state = 130;
                            } else { __state = 127; }
                        }
                        130 => {
                            unsafe {
                                *unsafe { (*p_idx).ai_column.offset(n as isize) } = i as i16
                            };
                            __state = 132;
                        }
                        131 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 129;
                        }
                        132 => {
                            unsafe {
                                *unsafe { (*p_idx).az_coll.offset(n as isize) } =
                                    sqlite3_str_binary.as_ptr() as *const i8
                            };
                            __state = 133;
                        }
                        133 => {
                            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                            __state = 131;
                        }
                        134 => {
                            if unsafe { (*p_table).tab_flags } & 128 as u32 == 0 as u32
                                {
                                __state = 136;
                            } else { __state = 135; }
                        }
                        135 => { __state = 138; }
                        136 => {
                            unsafe {
                                *unsafe { (*p_idx).ai_column.offset(n as isize) } =
                                    -1 as i16
                            };
                            __state = 137;
                        }
                        137 => {
                            unsafe {
                                *unsafe { (*p_idx).az_coll.offset(n as isize) } =
                                    sqlite3_str_binary.as_ptr() as *const i8
                            };
                            __state = 135;
                        }
                        138 => { { let _ = 0; }; __state = 139; }
                        139 => {
                            unsafe {
                                (*p_level_1).i_idx_cur =
                                    {
                                        let __p = unsafe { &mut (*p_parse_1).n_tab };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    }
                            };
                            __state = 140;
                        }
                        140 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 119,
                                    unsafe { (*p_level_1).i_idx_cur }, n_key_col + 1)
                            };
                            __state = 141;
                        }
                        141 => {
                            unsafe { sqlite3_vdbe_set_p4_key_info(p_parse_1, p_idx) };
                            __state = 142;
                        }
                        142 => { __state = 143; }
                        143 => {
                            if unsafe { (*unsafe { (*p_parse_1).db }).db_opt_flags } &
                                            524288 as u32 == 0 as u32 && use_bloom_filter != 0 {
                                __state = 145;
                            } else { __state = 144; }
                        }
                        144 => { { let _ = 0; }; __state = 148; }
                        145 => {
                            unsafe {
                                sqlite3_where_explain_bloom_filter(p_parse_1 as
                                        *const Parse, (*p_wc_1).p_w_info as *const WhereInfo,
                                    p_level_1 as *const WhereLevel)
                            };
                            __state = 146;
                        }
                        146 => {
                            unsafe {
                                (*p_level_1).reg_filter =
                                    {
                                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                                        *__p += 1;
                                        *__p
                                    }
                            };
                            __state = 147;
                        }
                        147 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 79, 10000,
                                    unsafe { (*p_level_1).reg_filter })
                            };
                            __state = 144;
                        }
                        148 => {
                            if unsafe { (*p_src).fg.via_coroutine() } != 0 {
                                __state = 150;
                            } else { __state = 151; }
                        }
                        149 => {
                            if !(p_partial).is_null() {
                                __state = 165;
                            } else { __state = 164; }
                        }
                        150 => { __state = 152; }
                        151 => { { let _ = 0; }; __state = 162; }
                        152 => { __state = 153; }
                        153 => { { let _ = 0; }; __state = 154; }
                        154 => {
                            p_subq = unsafe { (*p_src).u4.p_subq };
                            __state = 155;
                        }
                        155 => { { let _ = 0; }; __state = 156; }
                        156 => {
                            reg_yield = unsafe { (*p_subq).reg_return };
                            __state = 157;
                        }
                        157 => {
                            addr_counter = unsafe { sqlite3_vdbe_add_op2(v, 73, 0, 0) };
                            __state = 158;
                        }
                        158 => {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 11, reg_yield, 0,
                                    unsafe { (*p_subq).addr_fill_sub })
                            };
                            __state = 159;
                        }
                        159 => {
                            addr_top =
                                unsafe { sqlite3_vdbe_add_op1(v, 12, reg_yield) };
                            __state = 160;
                        }
                        160 => { __state = 161; }
                        161 => { __state = 149; }
                        162 => {
                            addr_top =
                                unsafe {
                                    sqlite3_vdbe_add_op2(v, 36,
                                        unsafe { (*p_level_1).i_tab_cur },
                                        unsafe { (*p_level_1).addr_halt })
                                };
                            __state = 163;
                        }
                        163 => { __state = 149; }
                        164 => {
                            reg_record = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            __state = 168;
                        }
                        165 => {
                            i_continue = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                            __state = 166;
                        }
                        166 => {
                            unsafe {
                                sqlite3_expr_if_false(p_parse_1, p_partial, i_continue, 16)
                            };
                            __state = 167;
                        }
                        167 => {
                            unsafe { (*p_loop).ws_flags |= 131072 as u32 };
                            __state = 164;
                        }
                        168 => {
                            reg_base =
                                unsafe {
                                    sqlite3_generate_index_key(p_parse_1, p_idx,
                                        unsafe { (*p_level_1).i_tab_cur }, reg_record, 0,
                                        core::ptr::null_mut(), core::ptr::null_mut(), 0)
                                };
                            __state = 169;
                        }
                        169 => {
                            if unsafe { (*p_level_1).reg_filter } != 0 {
                                __state = 171;
                            } else { __state = 170; }
                        }
                        170 => { __state = 172; }
                        171 => {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185,
                                    unsafe { (*p_level_1).reg_filter }, 0, reg_base,
                                    unsafe { (*p_loop).u.btree.n_eq } as i32)
                            };
                            __state = 170;
                        }
                        172 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 140,
                                    unsafe { (*p_level_1).i_idx_cur }, reg_record)
                            };
                            __state = 173;
                        }
                        173 => {
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                            __state = 174;
                        }
                        174 => {
                            if !(p_partial).is_null() {
                                __state = 176;
                            } else { __state = 175; }
                        }
                        175 => {
                            if unsafe { (*p_src).fg.via_coroutine() } != 0 {
                                __state = 178;
                            } else { __state = 179; }
                        }
                        176 => {
                            unsafe { sqlite3_vdbe_resolve_label(v, i_continue) };
                            __state = 175;
                        }
                        177 => {
                            unsafe { sqlite3_release_temp_reg(p_parse_1, reg_record) };
                            __state = 191;
                        }
                        178 => { { let _ = 0; }; __state = 180; }
                        179 => {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 40,
                                    unsafe { (*p_level_1).i_tab_cur }, addr_top + 1)
                            };
                            __state = 187;
                        }
                        180 => {
                            unsafe {
                                sqlite3_vdbe_change_p2(v, addr_counter, reg_base + n)
                            };
                            __state = 181;
                        }
                        181 => { __state = 182; }
                        182 => { { let _ = 0; }; __state = 183; }
                        183 => {
                            translate_column_to_copy(unsafe { &*p_parse_1 }, addr_top,
                                unsafe { (*p_level_1).i_tab_cur },
                                unsafe { (*unsafe { (*p_src).u4.p_subq }).reg_result },
                                unsafe { (*p_level_1).i_idx_cur });
                            __state = 184;
                        }
                        184 => {
                            unsafe { sqlite3_vdbe_goto(v, addr_top) };
                            __state = 185;
                        }
                        185 => {
                            unsafe { (*p_src).fg.set_via_coroutine(0 as u32 as u32) };
                            __state = 186;
                        }
                        186 => {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_top) };
                            __state = 177;
                        }
                        187 => { __state = 188; }
                        188 => {
                            unsafe { sqlite3_vdbe_change_p5(v, 3 as u16) };
                            __state = 189;
                        }
                        189 => {
                            if unsafe { (*p_src).fg.jointype } as i32 & 8 != 0 {
                                __state = 190;
                            } else { __state = 177; }
                        }
                        190 => {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_top) };
                            __state = 177;
                        }
                        191 => {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_init) };
                            __state = 192;
                        }
                        192 => { __state = 193; }
                        193 => { __state = 2; }
                        _ => {}
                    }
                }
            }
        }
    }
}

///* Generate bytecode that will initialize a Bloom filter that is appropriate
///* for pLevel.
///*
///* If there are inner loops within pLevel that have the WHERE_BLOOMFILTER
///* flag set, initialize a Bloomfilter for them as well.  Except don't do
///* this recursive initialization if the SQLITE_BloomPulldown optimization has
///* been turned off.
///*
///* When the Bloom filter is initialized, the WHERE_BLOOMFILTER flag is cleared
///* from the loop, but the regFilter value is set to a register that implements
///* the Bloom filter.  When regFilter is positive, the
///* sqlite3WhereCodeOneLoopStart() will generate code to test the Bloom filter
///* and skip the subsequence B-Tree seek if the Bloom filter indicates that
///* no matching rows exist.
///*
///* This routine may only be called if it has previously been determined that
///* the loop would benefit from a Bloom filter, and the WHERE_BLOOMFILTER bit
///* is set.
#[allow(unused_doc_comments)]
extern "C" fn sqlite3_construct_bloom_filter(p_w_info_1: *mut WhereInfo,
    mut i_level_1: i32, mut p_level_1: *mut WhereLevel, not_ready_1: Bitmask)
    -> () {
    unsafe {
        let mut addr_once: i32 = 0;
        /// Address of opening OP_Once
        let mut addr_top: i32 = 0;
        /// Address of OP_Rewind
        let mut addr_cont: i32 = 0;
        /// Jump here to skip a row
        let mut p_term: *const WhereTerm = core::ptr::null();
        /// For looping over WHERE clause terms
        let mut p_wc_end: *const WhereTerm = core::ptr::null();
        /// Last WHERE clause term
        let p_parse: *mut Parse = unsafe { (*p_w_info_1).p_parse };
        /// Parsing context
        let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
        /// VDBE under construction
        let mut p_loop: *mut WhereLoop = unsafe { (*p_level_1).p_w_loop };
        /// The loop being coded
        let mut i_cur: i32 = 0;
        /// Cursor for table getting the filter
        let mut saved_p_idx_epr: *mut IndexedExpr = core::ptr::null_mut();
        /// saved copy of Parse.pIdxEpr
        let mut saved_p_idx_part_expr: *mut IndexedExpr =
            core::ptr::null_mut();

        /// saved copy of Parse.pIdxPartExpr
        (saved_p_idx_epr = unsafe { (*p_parse).p_idx_epr });
        saved_p_idx_part_expr = unsafe { (*p_parse).p_idx_part_expr };
        unsafe { (*p_parse).p_idx_epr = core::ptr::null_mut() };
        unsafe { (*p_parse).p_idx_part_expr = core::ptr::null_mut() };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        addr_once = unsafe { sqlite3_vdbe_add_op0(v, 15) };
        '__b102: loop {
            '__c102: loop {
                let mut p_tab_list: *const SrcList = core::ptr::null();
                let mut p_item: *const SrcItem = core::ptr::null();
                let mut p_tab: *const Table = core::ptr::null();
                let mut sz: u64 = 0 as u64;
                let mut i_src: i32 = 0;
                unsafe {
                    sqlite3_where_explain_bloom_filter(p_parse as *const Parse,
                        p_w_info_1 as *const WhereInfo,
                        p_level_1 as *const WhereLevel)
                };
                addr_cont = unsafe { sqlite3_vdbe_make_label(p_parse) };
                i_cur = unsafe { (*p_level_1).i_tab_cur };
                unsafe {
                    (*p_level_1).reg_filter =
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            *__p += 1;
                            *__p
                        }
                };

                /// The Bloom filter is a Blob held in a register.  Initialize it
                ///* to zero-filled blob of at least 80K bits, but maybe more if the
                ///* estimated size of the table is larger.  We could actually
                ///* measure the size of the table at run-time using OP_Count with
                ///* P3==1 and use that value to initialize the blob.  But that makes
                ///* testing complicated.  By basing the blob size on the value in the
                ///* sqlite_stat1 table, testing is much easier.
                (p_tab_list =
                    unsafe { (*p_w_info_1).p_tab_list } as *const SrcList);
                i_src = unsafe { (*p_level_1).i_from } as i32;
                p_item =
                    unsafe {
                        &*(unsafe { (*p_tab_list).a.as_ptr() } as
                                        *const SrcItem).offset(i_src as isize)
                    };
                { let _ = 0; };
                p_tab = unsafe { (*p_item).p_s_tab } as *const Table;
                { let _ = 0; };
                sz =
                    unsafe {
                        sqlite3_log_est_to_int(unsafe { (*p_tab).n_row_log_est })
                    };
                if sz < 10000 as u64 {
                    sz = 10000 as u64;
                } else if sz > 10000000 as u64 { sz = 10000000 as u64; }
                unsafe {
                    sqlite3_vdbe_add_op2(v, 79, sz as i32,
                        unsafe { (*p_level_1).reg_filter })
                };
                addr_top = unsafe { sqlite3_vdbe_add_op1(v, 36, i_cur) };
                p_wc_end =
                    unsafe {
                            unsafe {
                                (*p_w_info_1).s_wc.a.offset(unsafe {
                                            (*p_w_info_1).s_wc.n_term
                                        } as isize)
                            }
                        } as *const WhereTerm;
                {
                    p_term =
                        unsafe { (*p_w_info_1).s_wc.a } as *const WhereTerm;
                    '__b103: loop {
                        if !(p_term < p_wc_end) { break '__b103; }
                        '__c103: loop {
                            let p_expr: *mut Expr = unsafe { (*p_term).p_expr };
                            if unsafe { (*p_term).wt_flags } as i32 & 2 == 0 &&
                                    unsafe {
                                            sqlite3_expr_is_single_table_constraint(p_expr, p_tab_list,
                                                i_src, 0)
                                        } != 0 {
                                unsafe {
                                    sqlite3_expr_if_false(p_parse, unsafe { (*p_term).p_expr },
                                        addr_cont, 16)
                                };
                            }
                            break '__c103;
                        }
                        {
                            let __p = &mut p_term;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                }
                if unsafe { (*p_loop).ws_flags } & 256 as u32 != 0 {
                    let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse) };
                    unsafe { sqlite3_vdbe_add_op2(v, 137, i_cur, r1) };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 185,
                            unsafe { (*p_level_1).reg_filter }, 0, r1, 1)
                    };
                    unsafe { sqlite3_release_temp_reg(p_parse, r1) };
                } else {
                    let p_idx: *mut Index =
                        unsafe { (*p_loop).u.btree.p_index };
                    let n: i32 = unsafe { (*p_loop).u.btree.n_eq } as i32;
                    let r1: i32 = unsafe { sqlite3_get_temp_range(p_parse, n) };
                    let mut jj: i32 = 0;
                    {
                        jj = 0;
                        '__b104: loop {
                            if !(jj < n) { break '__b104; }
                            '__c104: loop {
                                { let _ = 0; };
                                unsafe {
                                    sqlite3_expr_code_load_index_column(p_parse, p_idx, i_cur,
                                        jj, r1 + jj)
                                };
                                break '__c104;
                            }
                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 185,
                            unsafe { (*p_level_1).reg_filter }, 0, r1, n)
                    };
                    unsafe { sqlite3_release_temp_range(p_parse, r1, n) };
                }
                unsafe { sqlite3_vdbe_resolve_label(v, addr_cont) };
                unsafe {
                    sqlite3_vdbe_add_op2(v, 40,
                        unsafe { (*p_level_1).i_tab_cur }, addr_top + 1)
                };
                unsafe { sqlite3_vdbe_jump_here(v, addr_top) };
                unsafe { (*p_loop).ws_flags &= !4194304 as u32 };
                if unsafe { (*unsafe { (*p_parse).db }).db_opt_flags } &
                            1048576 as u32 != 0 as u32 {
                    break '__b102;
                }
                while { let __p = &mut i_level_1; *__p += 1; *__p } <
                        unsafe { (*p_w_info_1).n_level } as i32 {
                    let mut p_tab_item: *const SrcItem = core::ptr::null();
                    p_level_1 =
                        unsafe {
                            &mut *(unsafe { (*p_w_info_1).a.as_ptr() } as
                                            *mut WhereLevel).offset(i_level_1 as isize)
                        };
                    p_tab_item =
                        unsafe {
                                &raw mut *(unsafe {
                                                    (*unsafe { (*p_w_info_1).p_tab_list }).a.as_ptr()
                                                } as
                                                *mut SrcItem).add(unsafe { (*p_level_1).i_from } as usize)
                            } as *const SrcItem;
                    if unsafe { (*p_tab_item).fg.jointype } as i32 & (8 | 64) !=
                            0 {
                        continue;
                    }
                    p_loop = unsafe { (*p_level_1).p_w_loop };
                    if p_loop == core::ptr::null_mut() { continue; }
                    if unsafe { (*p_loop).prereq } & not_ready_1 != 0 {
                        continue;
                    }
                    if unsafe { (*p_loop).ws_flags } & (4194304 | 4) as u32 ==
                            4194304 as u32 {

                        /// This is a candidate for bloom-filter pull-down (early evaluation).
                        ///* The test that WHERE_COLUMN_IN is omitted is important, as we are
                        ///* not able to do early evaluation of bloom filters that make use of
                        ///* the IN operator
                        break;
                    }
                }
                break '__c102;
            }
            if !(i_level_1 < unsafe { (*p_w_info_1).n_level } as i32) {
                break '__b102;
            }
        }
        unsafe { sqlite3_vdbe_jump_here(v, addr_once) };
        unsafe { (*p_parse).p_idx_epr = saved_p_idx_epr };
        unsafe { (*p_parse).p_idx_part_expr = saved_p_idx_part_expr };
    }
}

///* Generate the beginning of the loop used for WHERE clause processing.
///* The return value is a pointer to an opaque structure that contains
///* information needed to terminate the loop.  Later, the calling routine
///* should invoke sqlite3WhereEnd() with the return value of this function
///* in order to complete the WHERE clause processing.
///*
///* If an error occurs, this routine returns NULL.
///*
///* The basic idea is to do a nested loop, one loop for each table in
///* the FROM clause of a select.  (INSERT and UPDATE statements are the
///* same as a SELECT with only a single table in the FROM clause.)  For
///* example, if the SQL is this:
///*
///*       SELECT * FROM t1, t2, t3 WHERE ...;
///*
///* Then the code generated is conceptually like the following:
///*
///*      foreach row1 in t1 do       \    Code generated
///*        foreach row2 in t2 do      |-- by sqlite3WhereBegin()
///*          foreach row3 in t3 do   /
///*            ...
///*          end                     \    Code generated
///*        end                        |-- by sqlite3WhereEnd()
///*      end                         /
///*
///* Note that the loops might not be nested in the order in which they
///* appear in the FROM clause if a different order is better able to make
///* use of indices.  Note also that when the IN operator appears in
///* the WHERE clause, it might result in additional nested loops for
///* scanning through all values on the right-hand side of the IN.
///*
///* There are Btree cursors associated with each table.  t1 uses cursor
///* number pTabList->a[0].iCursor.  t2 uses the cursor pTabList->a[1].iCursor.
///* And so forth.  This routine generates code to open those VDBE cursors
///* and sqlite3WhereEnd() generates the code to close them.
///*
///* The code that sqlite3WhereBegin() generates leaves the cursors named
///* in pTabList pointing at their appropriate entries.  The [...] code
///* can use OP_Column and OP_Rowid opcodes on these cursors to extract
///* data from the various tables of the loop.
///*
///* If the WHERE clause is empty, the foreach loops must each scan their
///* entire tables.  Thus a three-way join is an O(N^3) operation.  But if
///* the tables have indices and there are terms in the WHERE clause that
///* refer to those indices, a complete table scan can be avoided and the
///* code will run much faster.  Most of the work of this routine is checking
///* to see if there are indices that can be used to speed up the loop.
///*
///* Terms of the WHERE clause are also used to limit which rows actually
///* make it to the "..." in the middle of the loop.  After each "foreach",
///* terms of the WHERE clause that use only terms in that loop and outer
///* loops are evaluated and if false a jump is made around all subsequent
///* inner loops (or around the "..." if the test occurs within the inner-
///* most loop)
///*
///* OUTER JOINS
///*
///* An outer join of tables t1 and t2 is conceptually coded as follows:
///*
///*    foreach row1 in t1 do
///*      flag = 0
///*      foreach row2 in t2 do
///*        start:
///*          ...
///*          flag = 1
///*      end
///*      if flag==0 then
///*        move the row2 cursor to a null row
///*        goto start
///*      fi
///*    end
///*
///* ORDER BY CLAUSE PROCESSING
///*
///* pOrderBy is a pointer to the ORDER BY clause (or the GROUP BY clause
///* if the WHERE_GROUPBY flag is set in wctrlFlags) of a SELECT statement
///* if there is one.  If there is no ORDER BY clause or if this routine
///* is called from an UPDATE or DELETE statement, then pOrderBy is NULL.
///*
///* The iIdxCur parameter is the cursor number of an index.  If
///* WHERE_OR_SUBCLAUSE is set, iIdxCur is the cursor number of an index
///* to use for OR clause processing.  The WHERE clause should use this
///* specific cursor.  If WHERE_ONEPASS_DESIRED is set, then iIdxCur is
///* the first cursor in an array of cursors for all indices.  iIdxCur should
///* be used to compute the appropriate cursor depending on which index is
///* used.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_where_begin(p_parse_1: *mut Parse,
    p_tab_list_1: *mut SrcList, p_where_1: *mut Expr,
    mut p_order_by_1: *mut ExprList, p_result_set_1: *mut ExprList,
    p_select_1: *mut Select, mut wctrl_flags_1: u16, i_aux_arg_1: i32)
    -> *mut WhereInfo {
    unsafe {
        let mut n_byte_w_info: i32 = 0;
        /// Num. bytes allocated for WhereInfo struct
        let mut n_tab_list: i32 = 0;
        /// Number of elements in pTabList
        let mut p_w_info: *mut WhereInfo = core::ptr::null_mut();
        /// Will become the return value of this function
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// The virtual database engine
        let mut not_ready: Bitmask = 0 as Bitmask;
        /// Cursors that are not yet positioned
        let mut s_wlb: WhereLoopBuilder = unsafe { core::mem::zeroed() };
        /// The WhereLoop builder
        let mut p_mask_set: *mut WhereMaskSet = core::ptr::null_mut();
        /// The expression mask set
        let mut p_level: *mut WhereLevel = core::ptr::null_mut();
        /// A single level in pWInfo->a[]
        let mut p_loop: *mut WhereLoop = core::ptr::null_mut();
        /// Pointer to a single WhereLoop object
        let mut ii: i32 = 0;
        /// Loop counter
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// Database connection
        let mut rc: i32 = 0;
        /// Return code
        let mut b_fordelete: u8 = 0 as u8;
        /// OPFLAG_FORDELETE or zero, as appropriate
        /// Only one of WHERE_OR_SUBCLAUSE or WHERE_USE_LIMIT
        /// Variable initialization
        /// An ORDER/GROUP BY clause of more than 63 terms cannot be optimized
        /// Disable omit-noop-join opt
        /// The number of tables in the FROM clause is limited by the number of
        ///* bits in a Bitmask
        /// This function normally generates a nested loop for all tables in
        ///* pTabList.  But if the WHERE_OR_SUBCLAUSE flag is set, then we should
        ///* only generate code for the first table in pTabList and assume that
        ///* any cursors associated with subsequent tables are uninitialized.
        /// Allocate and initialize the WhereInfo structure that will become the
        ///* return value. A single allocation is used to store the WhereInfo
        ///* struct, the contents of WhereInfo.a[], the WhereClause structure
        ///* and the WhereMaskSet structure. Since WhereClause contains an 8-byte
        ///* field (type Bitmask) it must be aligned on an 8-byte boundary on
        ///* some architectures. Hence the ROUND8() below.
        /// ONEPASS defaults to OFF
        /// Initialize ix[0] to a value that can never be
        ///* a valid cursor number, to avoid an initial
        ///* test for pMaskSet->n==0 in sqlite3WhereGetMask()
        /// Split the WHERE clause into separate subexpressions where each
        ///* subexpression is separated by an AND operator.
        /// Special case: No FROM clause
        /// Assign a bit from the bitmask to every term in the FROM clause.
        ///*
        ///* The N-th term of the FROM clause is assigned a bitmask of 1<<N.
        ///*
        ///* The rule of the previous sentence ensures that if X is the bitmask for
        ///* a table T, then X-1 is the bitmask for all other tables to the left of T.
        ///* Knowing the bitmask for all tables to the left of a left join is
        ///* important.  Ticket #3015.
        ///*
        ///* Note that bitmasks are created for all pTabList->nSrc tables in
        ///* pTabList, not just the first nTabList tables.  nTabList is normally
        ///* equal to pTabList->nSrc but might be shortened to 1 if the
        ///* WHERE_OR_SUBCLAUSE flag is set.
        /// Analyze all of the subexpressions.
        /// The False-WHERE-Term-Bypass optimization:
        ///*
        ///* If there are WHERE terms that are false, then no rows will be output,
        ///* so skip over all of the code generated here.
        ///*
        ///* Conditions:
        ///*
        ///*   (1)  The WHERE term must not refer to any tables in the join.
        ///*   (2)  The term must not come from an ON clause on the
        ///*        right-hand side of a LEFT or FULL JOIN.
        ///*   (3)  The term must not come from an ON clause, or there must be
        ///*        no RIGHT or FULL OUTER joins in pTabList.
        ///*   (4)  If the expression contains non-deterministic functions
        ///*        that are not within a sub-select. This is not required
        ///*        for correctness but rather to preserves SQLite's legacy
        ///*        behaviour in the following two cases:
        ///*
        ///*          WHERE random()>0;           -- eval random() once per row
        ///*          WHERE (SELECT random())>0;  -- eval random() just once overall
        ///*
        ///* Note that the Where term need not be a constant in order for this
        ///* optimization to apply, though it does need to be constant relative to
        ///* the current subquery (condition 1).  The term might include variables
        ///* from outer queries so that the value of the term changes from one
        ///* invocation of the current subquery to the next.
        let mut p_t: *mut WhereTerm = core::ptr::null_mut();
        /// A term of the WHERE clause
        let mut p_x: *mut Expr = core::ptr::null_mut();
        /// The expression of pT
        /// Conditions (1) and (2)
        /// Condition (4)
        /// Condition (3)
        /// Disable the DISTINCT optimization if SQLITE_DistinctOpt is set via
        ///* sqlite3_test_ctrl(SQLITE_TESTCTRL_OPTIMIZATIONS,...)
        /// The DISTINCT marking is pointless.  Ignore it.
        /// Try to ORDER BY the result set to make distinct processing easier
        /// Construct the WhereLoop objects
        /// TUNING:  Assume that a DISTINCT clause on a subquery reduces
        ///* the output size by a factor of 8 (LogEst -30).  Search for
        ///* tag-20250414a to see other cases.
        /// Attempt to omit tables from a join that do not affect the result.
        ///* See the comment on whereOmitNoopJoin() for further information.
        ///*
        ///* This query optimization is factored out into a separate "no-inline"
        ///* procedure to keep the sqlite3WhereBegin() procedure from becoming
        ///* too large.  If sqlite3WhereBegin() becomes too large, that prevents
        ///* some C-compiler optimizers from in-lining the
        ///* sqlite3WhereCodeOneLoopStart() procedure, and it is important to
        ///* in-line sqlite3WhereCodeOneLoopStart() for performance reasons.
        /// Must be a join, or this opt8n is pointless
        /// Condition (1)
        /// (1),(6)
        /// (7)
        /// Check to see if there are any SEARCH loops that might benefit from
        ///* using a Bloom filter.
        /// If the caller is an UPDATE or DELETE statement that is requesting
        ///* to use a one-pass algorithm, determine if this is appropriate.
        ///*
        ///* A one-pass approach can be used if the caller has requested one
        ///* and either (a) the scan visits at most one row or (b) each
        ///* of the following are true:
        ///*
        ///*   * the caller has indicated that a one-pass approach can be used
        ///*     with multiple rows (by setting WHERE_ONEPASS_MULTIROW), and
        ///*   * the table is not a virtual table, and
        ///*   * either the scan does not use the OR optimization or the caller
        ///*     is a DELETE operation (WHERE_DUPLICATES_OK is only specified
        ///*     for DELETE).
        ///*
        ///* The last qualification is because an UPDATE statement uses
        ///* WhereInfo.aiCurOnePass[1] to determine whether or not it really can
        ///* use a one-pass approach, and this is not set accurately for scans
        ///* that use the OR optimization.
        let mut ws_flags: i32 = 0;
        let mut b_onerow: i32 = 0;
        /// Open all tables in the pTabList and any indices selected for
        ///* searching those tables.
        let mut p_tab: *mut Table = core::ptr::null_mut();
        /// Table to open
        let mut i_db: i32 = 0;
        /// Index of database containing table/index
        let mut p_tab_item: *mut SrcItem = core::ptr::null_mut();
        /// Do nothing
        let mut p_v_tab: *const i8 = core::ptr::null();
        let mut i_cur: i32 = 0;
        /// noop
        let mut op: i32 = 0;
        /// If we know that only a prefix of the record will be used,
        ///* it is advantageous to reduce the "column count" field in
        ///* the P4 operand of the OP_OpenRead/Write opcode.
        let mut b: Bitmask = 0 as Bitmask;
        let mut n: i32 = 0;
        let mut p_ix: *mut Index = core::ptr::null_mut();
        let mut i_index_cur: i32 = 0;
        let mut op__1: i32 = 0;
        /// iAuxArg is always set to a positive value if ONEPASS is possible
        /// This is one term of an OR-optimization using the PRIMARY KEY of a
        ///* WITHOUT ROWID table.  No need for a separate index
        let mut p_j: *mut Index = core::ptr::null_mut();
        /// SQLITE_ENABLE_COLUMN_USED_MASK
        let mut p_rj: *mut WhereRightJoin = core::ptr::null_mut();
        let mut p_info: *mut KeyInfo = core::ptr::null_mut();
        let mut p_pk: *mut Index = core::ptr::null_mut();
        /// The nature of RIGHT JOIN processing is such that it messes up
        ///* the output order.  So omit any ORDER BY/GROUP BY elimination
        ///* optimizations.  We need to do an actual sort for RIGHT JOIN.
        /// Generate the code to do the search.  Each iteration of the for
        ///* loop below generates code for a single nested loop of the VM
        ///* program.
        let mut addr_explain: i32 = 0;
        let mut ws_flags_1: i32 = 0;
        let mut p_src: *const SrcItem = core::ptr::null();
        let mut p_subq: *const Subquery = core::ptr::null();
        let mut i_once: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s107:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        if !(p_w_info).is_null() {
                            __state = 290;
                        } else { __state = 289; }
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { v = unsafe { (*p_parse_1).p_vdbe }; __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { __state = 14; }
                    14 => { b_fordelete = 0 as u8; __state = 15; }
                    15 => { { let _ = 0; }; __state = 16; }
                    16 => { { let _ = 0; }; __state = 17; }
                    17 => { db = unsafe { (*p_parse_1).db }; __state = 18; }
                    18 => {
                        unsafe {
                            memset(&raw mut s_wlb as *mut (), 0,
                                core::mem::size_of::<WhereLoopBuilder>() as u64)
                        };
                        __state = 19;
                    }
                    19 => { __state = 20; }
                    20 => {
                        if !(p_order_by_1).is_null() &&
                                unsafe { (*p_order_by_1).n_expr } >=
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => { __state = 25; }
                    22 => {
                        p_order_by_1 = core::ptr::null_mut();
                        __state = 23;
                    }
                    23 => { wctrl_flags_1 &= !256 as u16; __state = 24; }
                    24 => { wctrl_flags_1 |= 8192 as u16; __state = 21; }
                    25 => {
                        if unsafe { (*p_tab_list_1).n_src } >
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    26 => {
                        n_tab_list =
                            if wctrl_flags_1 as i32 & 32 != 0 {
                                1
                            } else { unsafe { (*p_tab_list_1).n_src } };
                        __state = 29;
                    }
                    27 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"at most %d tables in a join".as_ptr() as *mut i8 as
                                    *const i8,
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32)
                        };
                        __state = 28;
                    }
                    28 => { return core::ptr::null_mut(); }
                    29 => {
                        n_byte_w_info =
                            (core::mem::offset_of!(WhereInfo, a) as u64 +
                                            n_tab_list as u64 *
                                                core::mem::size_of::<WhereLevel>() as u64 + 7 as u64 &
                                    !7 as u64) as i32;
                        __state = 30;
                    }
                    30 => {
                        p_w_info =
                            unsafe {
                                    sqlite3_db_malloc_raw_nn(db,
                                        n_byte_w_info as u64 +
                                            core::mem::size_of::<WhereLoop>() as u64)
                                } as *mut WhereInfo;
                        __state = 31;
                    }
                    31 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        unsafe { (*p_w_info).p_parse = p_parse_1 };
                        __state = 36;
                    }
                    33 => {
                        unsafe { sqlite3_db_free(db, p_w_info as *mut ()) };
                        __state = 34;
                    }
                    34 => { p_w_info = core::ptr::null_mut(); __state = 35; }
                    35 => { __state = 2; }
                    36 => {
                        unsafe { (*p_w_info).p_tab_list = p_tab_list_1 };
                        __state = 37;
                    }
                    37 => {
                        unsafe { (*p_w_info).p_order_by = p_order_by_1 };
                        __state = 38;
                    }
                    38 => {
                        unsafe { (*p_w_info).p_result_set = p_result_set_1 };
                        __state = 39;
                    }
                    39 => {
                        unsafe {
                            (*p_w_info).ai_cur_one_pass[0 as usize] =
                                {
                                    unsafe { (*p_w_info).ai_cur_one_pass[1 as usize] = -1 };
                                    unsafe { (*p_w_info).ai_cur_one_pass[1 as usize] }
                                }
                        };
                        __state = 40;
                    }
                    40 => {
                        unsafe { (*p_w_info).n_level = n_tab_list as u8 };
                        __state = 41;
                    }
                    41 => {
                        unsafe {
                            (*p_w_info).i_break =
                                {
                                    unsafe {
                                        (*p_w_info).i_continue =
                                            unsafe { sqlite3_vdbe_make_label(p_parse_1) }
                                    };
                                    unsafe { (*p_w_info).i_continue }
                                }
                        };
                        __state = 42;
                    }
                    42 => {
                        unsafe { (*p_w_info).wctrl_flags = wctrl_flags_1 };
                        __state = 43;
                    }
                    43 => {
                        unsafe { (*p_w_info).i_limit = i_aux_arg_1 as LogEst };
                        __state = 44;
                    }
                    44 => {
                        unsafe {
                            (*p_w_info).saved_n_query_loop =
                                unsafe { (*p_parse_1).n_query_loop } as i32
                        };
                        __state = 45;
                    }
                    45 => {
                        unsafe { (*p_w_info).p_select = p_select_1 };
                        __state = 46;
                    }
                    46 => {
                        unsafe {
                            memset(unsafe { &raw mut (*p_w_info).n_ob_sat } as *mut (),
                                0,
                                core::mem::offset_of!(WhereInfo, s_wc) as u64 -
                                    core::mem::offset_of!(WhereInfo, n_ob_sat) as u64)
                        };
                        __state = 47;
                    }
                    47 => {
                        unsafe {
                            memset(unsafe {
                                        &raw mut *(unsafe { (*p_w_info).a.as_ptr() } as
                                                        *mut WhereLevel).offset(0 as isize)
                                    } as *mut (), 0,
                                core::mem::size_of::<WhereLoop>() as u64 +
                                    n_tab_list as u64 *
                                        core::mem::size_of::<WhereLevel>() as u64)
                        };
                        __state = 48;
                    }
                    48 => { { let _ = 0; }; __state = 49; }
                    49 => {
                        p_mask_set = unsafe { &mut (*p_w_info).s_mask_set };
                        __state = 50;
                    }
                    50 => { unsafe { (*p_mask_set).n = 0 }; __state = 51; }
                    51 => {
                        unsafe { (*p_mask_set).ix[0 as usize] = -99 };
                        __state = 52;
                    }
                    52 => { s_wlb.p_w_info = p_w_info; __state = 53; }
                    53 => {
                        s_wlb.p_wc = unsafe { &mut (*p_w_info).s_wc };
                        __state = 54;
                    }
                    54 => {
                        s_wlb.p_new =
                            unsafe {
                                    (p_w_info as *mut i8).offset(n_byte_w_info as isize)
                                } as *mut WhereLoop;
                        __state = 55;
                    }
                    55 => { { let _ = 0; }; __state = 56; }
                    56 => {
                        where_loop_init(unsafe { &mut *s_wlb.p_new });
                        __state = 57;
                    }
                    57 => {
                        unsafe {
                            sqlite3_where_clause_init(unsafe { &mut (*p_w_info).s_wc },
                                p_w_info)
                        };
                        __state = 58;
                    }
                    58 => {
                        unsafe {
                            sqlite3_where_split(unsafe { &mut (*p_w_info).s_wc },
                                p_where_1, 44 as u8)
                        };
                        __state = 59;
                    }
                    59 => {
                        if n_tab_list == 0 { __state = 61; } else { __state = 62; }
                    }
                    60 => {
                        unsafe {
                            sqlite3_where_expr_analyze(p_tab_list_1,
                                unsafe { &mut (*p_w_info).s_wc })
                        };
                        __state = 71;
                    }
                    61 => {
                        if !(p_order_by_1).is_null() {
                            __state = 64;
                        } else { __state = 63; }
                    }
                    62 => { ii = 0; __state = 68; }
                    63 => {
                        if wctrl_flags_1 as i32 & 256 != 0 &&
                                unsafe { (*db).db_opt_flags } & 16 as u32 == 0 as u32 {
                            __state = 66;
                        } else { __state = 65; }
                    }
                    64 => {
                        unsafe {
                            (*p_w_info).n_ob_sat =
                                unsafe { (*p_order_by_1).n_expr } as i8
                        };
                        __state = 63;
                    }
                    65 => {
                        if !(unsafe { (*p_w_info).p_select }).is_null() &&
                                unsafe { (*unsafe { (*p_w_info).p_select }).sel_flags } &
                                        1024 as u32 == 0 as u32 {
                            __state = 67;
                        } else { __state = 60; }
                    }
                    66 => {
                        unsafe { (*p_w_info).e_distinct = 1 as u8 };
                        __state = 65;
                    }
                    67 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                c"SCAN CONSTANT ROW".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 60;
                    }
                    68 => {
                        create_mask(unsafe { &mut *p_mask_set },
                            unsafe {
                                (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                *mut SrcItem).offset(ii as isize)).i_cursor
                            });
                        __state = 70;
                    }
                    69 => {
                        if { let __p = &mut ii; *__p += 1; *__p } <
                                unsafe { (*p_tab_list_1).n_src } {
                            __state = 68;
                        } else { __state = 60; }
                    }
                    70 => {
                        unsafe {
                            sqlite3_where_tab_func_args(p_parse_1,
                                unsafe {
                                    &mut *(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                    *mut SrcItem).offset(ii as isize)
                                }, unsafe { &mut (*p_w_info).s_wc })
                        };
                        __state = 69;
                    }
                    71 => {
                        if !(p_select_1).is_null() &&
                                !(unsafe { (*p_select_1).p_limit }).is_null() {
                            __state = 73;
                        } else { __state = 72; }
                    }
                    72 => {
                        if unsafe { (*p_parse_1).n_err } != 0 {
                            __state = 75;
                        } else { __state = 74; }
                    }
                    73 => {
                        unsafe {
                            sqlite3_where_add_limit(unsafe { &mut (*p_w_info).s_wc },
                                p_select_1)
                        };
                        __state = 72;
                    }
                    74 => { ii = 0; __state = 77; }
                    75 => { __state = 2; }
                    76 => {
                        if wctrl_flags_1 as i32 & 256 != 0 {
                            __state = 90;
                        } else { __state = 89; }
                    }
                    77 => {
                        if ii < unsafe { (*s_wlb.p_wc).n_base } {
                            __state = 78;
                        } else { __state = 76; }
                    }
                    78 => {
                        p_t =
                            unsafe { unsafe { (*s_wlb.p_wc).a.offset(ii as isize) } };
                        __state = 80;
                    }
                    79 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 77;
                    }
                    80 => { __state = 81; }
                    81 => {
                        if unsafe { (*p_t).wt_flags } as i32 & 2 != 0 {
                            __state = 83;
                        } else { __state = 82; }
                    }
                    82 => { p_x = unsafe { (*p_t).p_expr }; __state = 84; }
                    83 => { __state = 79; }
                    84 => { { let _ = 0; }; __state = 85; }
                    85 => { { let _ = 0; }; __state = 86; }
                    86 => {
                        if unsafe { (*p_t).prereq_all } == 0 as u64 &&
                                    (n_tab_list == 0 || expr_is_deterministic(p_x) != 0) &&
                                !(unsafe { (*p_x).flags } & 2 as u32 != 0 as u32 &&
                                                unsafe {
                                                                (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                                                    *mut SrcItem).offset(0 as isize)).fg.jointype
                                                            } as i32 & 64 != 0) as i32 != 0 {
                            __state = 87;
                        } else { __state = 79; }
                    }
                    87 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse_1, p_x,
                                unsafe { (*p_w_info).i_break }, 16)
                        };
                        __state = 88;
                    }
                    88 => {
                        unsafe { (*p_t).wt_flags |= 4 as u16 };
                        __state = 79;
                    }
                    89 => {
                        if n_tab_list != 1 || where_short_cut(&s_wlb) == 0 {
                            __state = 99;
                        } else { __state = 98; }
                    }
                    90 => {
                        if unsafe { (*db).db_opt_flags } & 16 as u32 != 0 as u32 {
                            __state = 91;
                        } else { __state = 92; }
                    }
                    91 => { wctrl_flags_1 &= !256 as u16; __state = 93; }
                    92 => {
                        if is_distinct_redundant(p_parse_1,
                                    unsafe { &*p_tab_list_1 }, unsafe { &mut (*p_w_info).s_wc },
                                    p_result_set_1) != 0 {
                            __state = 94;
                        } else { __state = 95; }
                    }
                    93 => {
                        unsafe { (*p_w_info).wctrl_flags &= !256 as u16 };
                        __state = 89;
                    }
                    94 => {
                        unsafe { (*p_w_info).e_distinct = 1 as u8 };
                        __state = 89;
                    }
                    95 => {
                        if p_order_by_1 == core::ptr::null_mut() {
                            __state = 96;
                        } else { __state = 89; }
                    }
                    96 => {
                        unsafe { (*p_w_info).wctrl_flags |= 128 as u16 };
                        __state = 97;
                    }
                    97 => {
                        unsafe { (*p_w_info).p_order_by = p_result_set_1 };
                        __state = 89;
                    }
                    98 => { { let _ = 0; }; __state = 114; }
                    99 => {
                        rc = where_loop_add_all(&mut s_wlb);
                        __state = 100;
                    }
                    100 => {
                        if rc != 0 { __state = 102; } else { __state = 101; }
                    }
                    101 => { __state = 103; }
                    102 => { __state = 2; }
                    103 => {
                        where_path_solver(p_w_info, 0 as LogEst);
                        __state = 104;
                    }
                    104 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 106;
                        } else { __state = 105; }
                    }
                    105 => {
                        if !(unsafe { (*p_w_info).p_order_by }).is_null() {
                            __state = 108;
                        } else { __state = 107; }
                    }
                    106 => { __state = 2; }
                    107 => {
                        if unsafe { (*p_w_info).wctrl_flags } as i32 & 256 != 0 {
                            __state = 112;
                        } else { __state = 98; }
                    }
                    108 => {
                        where_interstage_heuristic(unsafe { &*p_w_info });
                        __state = 109;
                    }
                    109 => {
                        where_path_solver(p_w_info,
                            if (unsafe { (*p_w_info).n_row_out } as i32) < 0 {
                                    1
                                } else { (unsafe { (*p_w_info).n_row_out }) as i32 + 1 } as
                                LogEst);
                        __state = 110;
                    }
                    110 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 111;
                        } else { __state = 107; }
                    }
                    111 => { __state = 2; }
                    112 => { __state = 113; }
                    113 => {
                        unsafe { (*p_w_info).n_row_out -= 30 as LogEst };
                        __state = 98;
                    }
                    114 => {
                        if unsafe { (*p_w_info).p_order_by } ==
                                    core::ptr::null_mut() &&
                                unsafe { (*db).flags } & 4096 as u64 != 0 as u64 {
                            __state = 116;
                        } else { __state = 115; }
                    }
                    115 => {
                        if unsafe { (*p_parse_1).n_err } != 0 {
                            __state = 118;
                        } else { __state = 117; }
                    }
                    116 => {
                        where_reverse_scan_order(unsafe { &mut *p_w_info });
                        __state = 115;
                    }
                    117 => { { let _ = 0; }; __state = 119; }
                    118 => { __state = 2; }
                    119 => { not_ready = !(0 as Bitmask); __state = 120; }
                    120 => {
                        if unsafe { (*p_w_info).n_level } as i32 >= 2 &&
                                        p_result_set_1 != core::ptr::null_mut() &&
                                    0 == wctrl_flags_1 as i32 & (1024 | 8192) &&
                                unsafe { (*db).db_opt_flags } & 256 as u32 == 0 as u32 {
                            __state = 122;
                        } else { __state = 121; }
                    }
                    121 => {
                        if unsafe { (*p_w_info).n_level } as i32 >= 2 &&
                                unsafe { (*db).db_opt_flags } & 524288 as u32 == 0 as u32 {
                            __state = 126;
                        } else { __state = 125; }
                    }
                    122 => {
                        not_ready =
                            where_omit_noop_join(unsafe { &mut *p_w_info }, not_ready);
                        __state = 123;
                    }
                    123 => {
                        n_tab_list = unsafe { (*p_w_info).n_level } as i32;
                        __state = 124;
                    }
                    124 => { { let _ = 0; }; __state = 121; }
                    125 => {
                        unsafe {
                            (*unsafe { (*p_w_info).p_parse }).n_query_loop +=
                                unsafe { (*p_w_info).n_row_out } as i32 as LogEst
                        };
                        __state = 127;
                    }
                    126 => {
                        where_check_if_bloom_filter_is_useful(unsafe {
                                &*p_w_info
                            });
                        __state = 125;
                    }
                    127 => { { let _ = 0; }; __state = 128; }
                    128 => {
                        if wctrl_flags_1 as i32 & 4 != 0 {
                            __state = 130;
                        } else { __state = 129; }
                    }
                    129 => {
                        {
                            ii = 0;
                            p_level =
                                unsafe { (*p_w_info).a.as_ptr() } as *mut WhereLevel
                        };
                        __state = 140;
                    }
                    130 => {
                        ws_flags =
                            unsafe {
                                    (*unsafe {
                                                    (*(unsafe { (*p_w_info).a.as_ptr() } as
                                                                    *mut WhereLevel).offset(0 as isize)).p_w_loop
                                                }).ws_flags
                                } as i32;
                        __state = 131;
                    }
                    131 => {
                        b_onerow = (ws_flags & 4096 != 0) as i32;
                        __state = 132;
                    }
                    132 => { { let _ = 0; }; __state = 133; }
                    133 => {
                        if b_onerow != 0 ||
                                0 != wctrl_flags_1 as i32 & 8 &&
                                            !(unsafe {
                                                                    (*unsafe {
                                                                                    (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                                                                    *mut SrcItem).offset(0 as isize)).p_s_tab
                                                                                }).e_tab_type
                                                                } as i32 == 1) as i32 != 0 &&
                                        (0 == ws_flags & 8192 || wctrl_flags_1 as i32 & 16 != 0) &&
                                    unsafe { (*db).db_opt_flags } & 134217728 as u32 == 0 as u32
                            {
                            __state = 134;
                        } else { __state = 129; }
                    }
                    134 => {
                        unsafe {
                            (*p_w_info).e_one_pass =
                                if b_onerow != 0 { 1 } else { 2 } as u8
                        };
                        __state = 135;
                    }
                    135 => {
                        if unsafe {
                                            (*unsafe {
                                                            (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                                            *mut SrcItem).offset(0 as isize)).p_s_tab
                                                        }).tab_flags
                                        } & 128 as u32 == 0 as u32 && ws_flags & 64 != 0 {
                            __state = 136;
                        } else { __state = 129; }
                    }
                    136 => {
                        if wctrl_flags_1 as i32 & 8 != 0 {
                            __state = 138;
                        } else { __state = 137; }
                    }
                    137 => {
                        unsafe {
                            (*unsafe {
                                                (*(unsafe { (*p_w_info).a.as_ptr() } as
                                                                *mut WhereLevel).offset(0 as isize)).p_w_loop
                                            }).ws_flags = (ws_flags & !64) as u32
                        };
                        __state = 129;
                    }
                    138 => { b_fordelete = 8 as u8; __state = 137; }
                    139 => {
                        unsafe {
                            (*p_w_info).i_top = unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 246;
                    }
                    140 => {
                        if ii < n_tab_list {
                            __state = 141;
                        } else { __state = 139; }
                    }
                    141 => { __state = 143; }
                    142 => {
                        {
                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut p_level;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 140;
                    }
                    143 => { __state = 144; }
                    144 => { __state = 145; }
                    145 => {
                        p_tab_item =
                            unsafe {
                                &mut *(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                *mut SrcItem).add(unsafe { (*p_level).i_from } as usize)
                            };
                        __state = 146;
                    }
                    146 => {
                        p_tab = unsafe { (*p_tab_item).p_s_tab };
                        __state = 147;
                    }
                    147 => {
                        i_db =
                            unsafe {
                                sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                            };
                        __state = 148;
                    }
                    148 => {
                        p_loop = unsafe { (*p_level).p_w_loop };
                        __state = 149;
                    }
                    149 => {
                        unsafe {
                            (*p_level).addr_brk =
                                unsafe { sqlite3_vdbe_make_label(p_parse_1) }
                        };
                        __state = 150;
                    }
                    150 => {
                        if ii == 0 ||
                                unsafe { (*p_tab_item.offset(0 as isize)).fg.jointype } as
                                            i32 & 8 != 0 {
                            __state = 152;
                        } else { __state = 153; }
                    }
                    151 => {
                        if unsafe { (*p_tab).tab_flags } & 16384 as u32 != 0 as u32
                                || unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                            __state = 157;
                        } else { __state = 158; }
                    }
                    152 => {
                        unsafe {
                            (*p_level).addr_halt = unsafe { (*p_level).addr_brk }
                        };
                        __state = 151;
                    }
                    153 => {
                        if !(unsafe {
                                            (*(unsafe { (*p_w_info).a.as_ptr() } as
                                                            *mut WhereLevel).offset((ii - 1) as isize)).p_rj
                                        }).is_null() {
                            __state = 154;
                        } else { __state = 155; }
                    }
                    154 => {
                        unsafe {
                            (*p_level).addr_halt =
                                unsafe {
                                    (*(unsafe { (*p_w_info).a.as_ptr() } as
                                                    *mut WhereLevel).offset((ii - 1) as isize)).addr_brk
                                }
                        };
                        __state = 151;
                    }
                    155 => {
                        unsafe {
                            (*p_level).addr_halt =
                                unsafe {
                                    (*(unsafe { (*p_w_info).a.as_ptr() } as
                                                    *mut WhereLevel).offset((ii - 1) as isize)).addr_halt
                                }
                        };
                        __state = 151;
                    }
                    156 => {
                        if unsafe { (*p_loop).ws_flags } & 512 as u32 != 0 {
                            __state = 189;
                        } else { __state = 188; }
                    }
                    157 => { __state = 156; }
                    158 => {
                        if unsafe { (*p_loop).ws_flags } & 1024 as u32 != 0 as u32 {
                            __state = 159;
                        } else { __state = 160; }
                    }
                    159 => {
                        p_v_tab =
                            unsafe { sqlite3_get_v_table(db, p_tab) } as *const i8;
                        __state = 161;
                    }
                    160 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                            __state = 163;
                        } else { __state = 164; }
                    }
                    161 => {
                        i_cur = unsafe { (*p_tab_item).i_cursor };
                        __state = 162;
                    }
                    162 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 175, i_cur, 0, 0, p_v_tab, -12)
                        };
                        __state = 156;
                    }
                    163 => { __state = 156; }
                    164 => {
                        if unsafe { (*p_loop).ws_flags } & 64 as u32 == 0 as u32 &&
                                    wctrl_flags_1 as i32 & 32 == 0 ||
                                unsafe { (*p_tab_item).fg.jointype } as i32 & (64 | 16) != 0
                            {
                            __state = 165;
                        } else { __state = 166; }
                    }
                    165 => { op = 114; __state = 167; }
                    166 => {
                        unsafe {
                            sqlite3_table_lock(p_parse_1, i_db,
                                unsafe { (*p_tab).tnum }, 0 as u8,
                                unsafe { (*p_tab).z_name } as *const i8)
                        };
                        __state = 156;
                    }
                    167 => {
                        if unsafe { (*p_w_info).e_one_pass } as i32 != 0 {
                            __state = 169;
                        } else { __state = 168; }
                    }
                    168 => { __state = 171; }
                    169 => { op = 116; __state = 170; }
                    170 => {
                        unsafe {
                            (*p_w_info).ai_cur_one_pass[0 as usize] =
                                unsafe { (*p_tab_item).i_cursor }
                        };
                        __state = 168;
                    }
                    171 => {
                        unsafe {
                            sqlite3_open_table(p_parse_1,
                                unsafe { (*p_tab_item).i_cursor }, i_db, p_tab, op)
                        };
                        __state = 172;
                    }
                    172 => { { let _ = 0; }; __state = 173; }
                    173 => { __state = 174; }
                    174 => { __state = 175; }
                    175 => {
                        if unsafe { (*p_w_info).e_one_pass } as i32 == 0 &&
                                        (unsafe { (*p_tab).n_col } as i32) <
                                            (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32
                                    &&
                                    unsafe { (*p_tab).tab_flags } & (96 | 128) as u32 ==
                                        0 as u32 &&
                                unsafe { (*p_loop).ws_flags } & (16384 | 4194304) as u32 ==
                                    0 as u32 {
                            __state = 177;
                        } else { __state = 176; }
                    }
                    176 => {
                        unsafe { sqlite3_vdbe_change_p5(v, b_fordelete as u16) };
                        __state = 185;
                    }
                    177 => {
                        b = unsafe { (*p_tab_item).col_used };
                        __state = 178;
                    }
                    178 => { n = 0; __state = 179; }
                    179 => { __state = 181; }
                    180 => {
                        unsafe {
                            sqlite3_vdbe_change_p4(v, -1,
                                n as i64 as *mut () as *const i8, -3)
                        };
                        __state = 184;
                    }
                    181 => {
                        if b != 0 { __state = 182; } else { __state = 180; }
                    }
                    182 => { __state = 183; }
                    183 => {
                        {
                            ({ b = b >> 1; b }) as i32;
                            { let __p = &mut n; let __t = *__p; *__p += 1; __t }
                        };
                        __state = 181;
                    }
                    184 => { { let _ = 0; }; __state = 176; }
                    185 => {
                        if ii >= 2 &&
                                    unsafe { (*p_tab_item.offset(0 as isize)).fg.jointype } as
                                                i32 & (64 | 8) == 0 &&
                                unsafe { (*p_level).addr_halt } ==
                                    unsafe {
                                        (*(unsafe { (*p_w_info).a.as_ptr() } as
                                                        *mut WhereLevel).offset(0 as isize)).addr_halt
                                    } {
                            __state = 186;
                        } else { __state = 156; }
                    }
                    186 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 37,
                                unsafe { (*p_tab_item).i_cursor },
                                unsafe { (*p_w_info).i_break })
                        };
                        __state = 187;
                    }
                    187 => { __state = 156; }
                    188 => {
                        if i_db >= 0 { __state = 224; } else { __state = 223; }
                    }
                    189 => {
                        p_ix = unsafe { (*p_loop).u.btree.p_index };
                        __state = 190;
                    }
                    190 => { __state = 191; }
                    191 => { op__1 = 114; __state = 192; }
                    192 => { { let _ = 0; }; __state = 193; }
                    193 => {
                        if !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32)
                                            as i32 != 0 && unsafe { (*p_ix).idx_type() } as i32 == 2 &&
                                wctrl_flags_1 as i32 & 32 != 0 {
                            __state = 195;
                        } else { __state = 196; }
                    }
                    194 => {
                        unsafe { (*p_level).i_idx_cur = i_index_cur };
                        __state = 214;
                    }
                    195 => {
                        i_index_cur = unsafe { (*p_level).i_tab_cur };
                        __state = 197;
                    }
                    196 => {
                        if unsafe { (*p_w_info).e_one_pass } as i32 != 0 {
                            __state = 198;
                        } else { __state = 199; }
                    }
                    197 => { op__1 = 0; __state = 194; }
                    198 => {
                        p_j =
                            unsafe { (*unsafe { (*p_tab_item).p_s_tab }).p_index };
                        __state = 200;
                    }
                    199 => {
                        if i_aux_arg_1 != 0 && wctrl_flags_1 as i32 & 32 != 0 {
                            __state = 207;
                        } else { __state = 208; }
                    }
                    200 => { i_index_cur = i_aux_arg_1; __state = 201; }
                    201 => { { let _ = 0; }; __state = 202; }
                    202 => {
                        if !(p_j).is_null() && p_j != p_ix {
                            __state = 204;
                        } else { __state = 203; }
                    }
                    203 => { op__1 = 116; __state = 206; }
                    204 => {
                        {
                            let __p = &mut i_index_cur;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 205;
                    }
                    205 => { p_j = unsafe { (*p_j).p_next }; __state = 202; }
                    206 => {
                        unsafe {
                            (*p_w_info).ai_cur_one_pass[1 as usize] = i_index_cur
                        };
                        __state = 194;
                    }
                    207 => { i_index_cur = i_aux_arg_1; __state = 209; }
                    208 => {
                        i_index_cur =
                            {
                                let __p = unsafe { &mut (*p_parse_1).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 210;
                    }
                    209 => { op__1 = 113; __state = 194; }
                    210 => {
                        if unsafe { (*p_ix).b_has_expr() } != 0 &&
                                unsafe { (*db).db_opt_flags } & 16777216 as u32 == 0 as u32
                            {
                            __state = 212;
                        } else { __state = 211; }
                    }
                    211 => {
                        if !(unsafe { (*p_ix).p_part_idx_where }).is_null() &&
                                unsafe { (*p_tab_item).fg.jointype } as i32 & 16 == 0 {
                            __state = 213;
                        } else { __state = 194; }
                    }
                    212 => {
                        where_add_indexed_expr(p_parse_1, p_ix, i_index_cur,
                            unsafe { &*p_tab_item });
                        __state = 211;
                    }
                    213 => {
                        where_part_idx_expr(p_parse_1, p_ix,
                            unsafe { (*p_ix).p_part_idx_where } as *const Expr,
                            core::ptr::null_mut(), i_index_cur, p_tab_item);
                        __state = 194;
                    }
                    214 => { { let _ = 0; }; __state = 215; }
                    215 => { { let _ = 0; }; __state = 216; }
                    216 => { { let _ = 0; }; __state = 217; }
                    217 => {
                        if op__1 != 0 { __state = 218; } else { __state = 188; }
                    }
                    218 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, op__1, i_index_cur,
                                unsafe { (*p_ix).tnum } as i32, i_db)
                        };
                        __state = 219;
                    }
                    219 => {
                        unsafe { sqlite3_vdbe_set_p4_key_info(p_parse_1, p_ix) };
                        __state = 220;
                    }
                    220 => {
                        if unsafe { (*p_loop).ws_flags } & 15 as u32 != 0 as u32 &&
                                                unsafe { (*p_loop).ws_flags } & (2 | 32768) as u32 ==
                                                    0 as u32 &&
                                            unsafe { (*p_loop).ws_flags } & 524288 as u32 == 0 as u32 &&
                                        unsafe { (*p_loop).ws_flags } & 1048576 as u32 == 0 as u32
                                    && unsafe { (*p_w_info).wctrl_flags } as i32 & 1 == 0 &&
                                unsafe { (*p_w_info).e_distinct } as i32 != 2 {
                            __state = 222;
                        } else { __state = 221; }
                    }
                    221 => { __state = 188; }
                    222 => {
                        unsafe { sqlite3_vdbe_change_p5(v, 2 as u16) };
                        __state = 221;
                    }
                    223 => {
                        if unsafe { (*p_tab_item).fg.jointype } as i32 & 16 != 0 &&
                                {
                                        let __v =
                                            sqlite3_where_malloc(unsafe { &mut *p_w_info },
                                                    core::mem::size_of::<WhereRightJoin>() as u64) as
                                                *mut WhereRightJoin;
                                        unsafe { (*p_level).p_rj = __v };
                                        __v
                                    } != core::ptr::null_mut() {
                            __state = 225;
                        } else { __state = 142; }
                    }
                    224 => {
                        unsafe { sqlite3_code_verify_schema(p_parse_1, i_db) };
                        __state = 223;
                    }
                    225 => { p_rj = unsafe { (*p_level).p_rj }; __state = 226; }
                    226 => {
                        unsafe {
                            (*p_rj).i_match =
                                {
                                    let __p = unsafe { &mut (*p_parse_1).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                }
                        };
                        __state = 227;
                    }
                    227 => {
                        unsafe {
                            (*p_rj).reg_bloom =
                                {
                                    let __p = unsafe { &mut (*p_parse_1).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 228;
                    }
                    228 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 79, 65536,
                                unsafe { (*p_rj).reg_bloom })
                        };
                        __state = 229;
                    }
                    229 => {
                        unsafe {
                            (*p_rj).reg_return =
                                {
                                    let __p = unsafe { &mut (*p_parse_1).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 230;
                    }
                    230 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 77, 0,
                                unsafe { (*p_rj).reg_return })
                        };
                        __state = 231;
                    }
                    231 => { { let _ = 0; }; __state = 232; }
                    232 => {
                        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                            __state = 234;
                        } else { __state = 235; }
                    }
                    233 => {
                        unsafe { (*p_loop).ws_flags &= !64 as u32 };
                        __state = 244;
                    }
                    234 => { __state = 236; }
                    235 => {
                        p_pk = unsafe { sqlite3_primary_key_index(p_tab) };
                        __state = 242;
                    }
                    236 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 120, unsafe { (*p_rj).i_match }, 1)
                        };
                        __state = 237;
                    }
                    237 => {
                        p_info =
                            unsafe {
                                sqlite3_key_info_alloc(unsafe { (*p_parse_1).db }, 1, 0)
                            };
                        __state = 238;
                    }
                    238 => {
                        if !(p_info).is_null() {
                            __state = 239;
                        } else { __state = 233; }
                    }
                    239 => {
                        unsafe {
                            *(unsafe { (*p_info).a_coll.as_ptr() } as
                                            *mut *mut CollSeq).offset(0 as isize) =
                                core::ptr::null_mut()
                        };
                        __state = 240;
                    }
                    240 => {
                        unsafe {
                            *unsafe { (*p_info).a_sort_flags.offset(0 as isize) } =
                                0 as u8
                        };
                        __state = 241;
                    }
                    241 => {
                        unsafe { sqlite3_vdbe_append_p4(v, p_info as *mut (), -9) };
                        __state = 233;
                    }
                    242 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 120, unsafe { (*p_rj).i_match },
                                unsafe { (*p_pk).n_key_col } as i32)
                        };
                        __state = 243;
                    }
                    243 => {
                        unsafe { sqlite3_vdbe_set_p4_key_info(p_parse_1, p_pk) };
                        __state = 233;
                    }
                    244 => {
                        unsafe { (*p_w_info).n_ob_sat = 0 as i8 };
                        __state = 245;
                    }
                    245 => {
                        unsafe { (*p_w_info).e_distinct = 3 as u8 };
                        __state = 142;
                    }
                    246 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 248;
                        } else { __state = 247; }
                    }
                    247 => { ii = 0; __state = 250; }
                    248 => { __state = 2; }
                    249 => { __state = 286; }
                    250 => {
                        if ii < n_tab_list {
                            __state = 251;
                        } else { __state = 249; }
                    }
                    251 => { __state = 253; }
                    252 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 250;
                    }
                    253 => { __state = 254; }
                    254 => { __state = 255; }
                    255 => {
                        if unsafe { (*p_parse_1).n_err } != 0 {
                            __state = 257;
                        } else { __state = 256; }
                    }
                    256 => {
                        p_level =
                            unsafe {
                                &mut *(unsafe { (*p_w_info).a.as_ptr() } as
                                                *mut WhereLevel).offset(ii as isize)
                            };
                        __state = 258;
                    }
                    257 => { __state = 2; }
                    258 => {
                        ws_flags_1 =
                            unsafe { (*unsafe { (*p_level).p_w_loop }).ws_flags } as
                                i32;
                        __state = 259;
                    }
                    259 => {
                        p_src =
                            unsafe {
                                &mut *(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                *mut SrcItem).add(unsafe { (*p_level).i_from } as usize)
                            };
                        __state = 260;
                    }
                    260 => {
                        if unsafe { (*p_src).fg.is_materialized() } != 0 {
                            __state = 262;
                        } else { __state = 261; }
                    }
                    261 => { { let _ = 0; }; __state = 274; }
                    262 => { __state = 263; }
                    263 => { i_once = 0; __state = 264; }
                    264 => { { let _ = 0; }; __state = 265; }
                    265 => {
                        p_subq = unsafe { (*p_src).u4.p_subq };
                        __state = 266;
                    }
                    266 => {
                        if unsafe { (*p_src).fg.is_correlated() } as i32 == 0 {
                            __state = 268;
                        } else { __state = 269; }
                    }
                    267 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, unsafe { (*p_subq).reg_return },
                                unsafe { (*p_subq).addr_fill_sub })
                        };
                        __state = 271;
                    }
                    268 => {
                        i_once = unsafe { sqlite3_vdbe_add_op0(v, 15) };
                        __state = 270;
                    }
                    269 => { i_once = 0; __state = 267; }
                    270 => { __state = 267; }
                    271 => { __state = 272; }
                    272 => {
                        if i_once != 0 { __state = 273; } else { __state = 261; }
                    }
                    273 => {
                        unsafe { sqlite3_vdbe_jump_here(v, i_once) };
                        __state = 261;
                    }
                    274 => {
                        if ws_flags_1 & (16384 | 4194304) != 0 {
                            __state = 276;
                        } else { __state = 275; }
                    }
                    275 => {
                        addr_explain =
                            unsafe {
                                sqlite3_where_explain_one_scan(p_parse_1, p_tab_list_1,
                                    p_level, wctrl_flags_1)
                            };
                        __state = 281;
                    }
                    276 => {
                        if ws_flags_1 & 16384 != 0 {
                            __state = 278;
                        } else { __state = 279; }
                    }
                    277 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 280;
                        } else { __state = 275; }
                    }
                    278 => {
                        construct_automatic_index(p_parse_1,
                            unsafe { &(*p_w_info).s_wc }, not_ready as Bitmask,
                            p_level);
                        __state = 277;
                    }
                    279 => {
                        sqlite3_construct_bloom_filter(p_w_info, ii, p_level,
                            not_ready);
                        __state = 277;
                    }
                    280 => { __state = 2; }
                    281 => {
                        unsafe {
                            (*p_level).addr_body =
                                unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 282;
                    }
                    282 => {
                        not_ready =
                            unsafe {
                                sqlite3_where_code_one_loop_start(p_parse_1, v, p_w_info,
                                    ii, p_level, not_ready)
                            };
                        __state = 283;
                    }
                    283 => {
                        unsafe {
                            (*p_w_info).i_continue = unsafe { (*p_level).addr_cont }
                        };
                        __state = 284;
                    }
                    284 => {
                        if ws_flags_1 & 8192 == 0 && wctrl_flags_1 as i32 & 32 == 0
                            {
                            __state = 285;
                        } else { __state = 252; }
                    }
                    285 => { { let _ = addr_explain; }; __state = 252; }
                    286 => {
                        unsafe {
                            (*p_w_info).i_end_where =
                                unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        __state = 287;
                    }
                    287 => { return p_w_info; }
                    288 => { __state = 2; }
                    289 => { return core::ptr::null_mut(); }
                    290 => {
                        unsafe {
                            (*p_parse_1).n_query_loop =
                                unsafe { (*p_w_info).saved_n_query_loop } as LogEst
                        };
                        __state = 291;
                    }
                    291 => { where_info_free(db, p_w_info); __state = 289; }
                    _ => {}
                }
            }
        }

        /// Num. bytes allocated for WhereInfo struct
        /// Number of elements in pTabList
        /// Will become the return value of this function
        /// The virtual database engine
        /// Cursors that are not yet positioned
        /// The WhereLoop builder
        /// The expression mask set
        /// A single level in pWInfo->a[]
        /// Pointer to a single WhereLoop object
        /// Loop counter
        /// Database connection
        /// Return code
        /// OPFLAG_FORDELETE or zero, as appropriate
        /// Only one of WHERE_OR_SUBCLAUSE or WHERE_USE_LIMIT
        /// Variable initialization
        /// An ORDER/GROUP BY clause of more than 63 terms cannot be optimized
        /// Disable omit-noop-join opt
        /// The number of tables in the FROM clause is limited by the number of
        ///* bits in a Bitmask
        /// This function normally generates a nested loop for all tables in
        ///* pTabList.  But if the WHERE_OR_SUBCLAUSE flag is set, then we should
        ///* only generate code for the first table in pTabList and assume that
        ///* any cursors associated with subsequent tables are uninitialized.
        /// Allocate and initialize the WhereInfo structure that will become the
        ///* return value. A single allocation is used to store the WhereInfo
        ///* struct, the contents of WhereInfo.a[], the WhereClause structure
        ///* and the WhereMaskSet structure. Since WhereClause contains an 8-byte
        ///* field (type Bitmask) it must be aligned on an 8-byte boundary on
        ///* some architectures. Hence the ROUND8() below.
        /// ONEPASS defaults to OFF
        /// Initialize ix[0] to a value that can never be
        ///* a valid cursor number, to avoid an initial
        ///* test for pMaskSet->n==0 in sqlite3WhereGetMask()
        /// Split the WHERE clause into separate subexpressions where each
        ///* subexpression is separated by an AND operator.
        /// Special case: No FROM clause
        /// Assign a bit from the bitmask to every term in the FROM clause.
        ///*
        ///* The N-th term of the FROM clause is assigned a bitmask of 1<<N.
        ///*
        ///* The rule of the previous sentence ensures that if X is the bitmask for
        ///* a table T, then X-1 is the bitmask for all other tables to the left of T.
        ///* Knowing the bitmask for all tables to the left of a left join is
        ///* important.  Ticket #3015.
        ///*
        ///* Note that bitmasks are created for all pTabList->nSrc tables in
        ///* pTabList, not just the first nTabList tables.  nTabList is normally
        ///* equal to pTabList->nSrc but might be shortened to 1 if the
        ///* WHERE_OR_SUBCLAUSE flag is set.
        /// Analyze all of the subexpressions.
        /// The False-WHERE-Term-Bypass optimization:
        ///*
        ///* If there are WHERE terms that are false, then no rows will be output,
        ///* so skip over all of the code generated here.
        ///*
        ///* Conditions:
        ///*
        ///*   (1)  The WHERE term must not refer to any tables in the join.
        ///*   (2)  The term must not come from an ON clause on the
        ///*        right-hand side of a LEFT or FULL JOIN.
        ///*   (3)  The term must not come from an ON clause, or there must be
        ///*        no RIGHT or FULL OUTER joins in pTabList.
        ///*   (4)  If the expression contains non-deterministic functions
        ///*        that are not within a sub-select. This is not required
        ///*        for correctness but rather to preserves SQLite's legacy
        ///*        behaviour in the following two cases:
        ///*
        ///*          WHERE random()>0;           -- eval random() once per row
        ///*          WHERE (SELECT random())>0;  -- eval random() just once overall
        ///*
        ///* Note that the Where term need not be a constant in order for this
        ///* optimization to apply, though it does need to be constant relative to
        ///* the current subquery (condition 1).  The term might include variables
        ///* from outer queries so that the value of the term changes from one
        ///* invocation of the current subquery to the next.
        /// A term of the WHERE clause
        /// The expression of pT
        /// Conditions (1) and (2)
        /// Condition (4)
        /// Condition (3)
        /// Disable the DISTINCT optimization if SQLITE_DistinctOpt is set via
        ///* sqlite3_test_ctrl(SQLITE_TESTCTRL_OPTIMIZATIONS,...)
        /// The DISTINCT marking is pointless.  Ignore it.
        /// Try to ORDER BY the result set to make distinct processing easier
        /// Construct the WhereLoop objects
        /// TUNING:  Assume that a DISTINCT clause on a subquery reduces
        ///* the output size by a factor of 8 (LogEst -30).  Search for
        ///* tag-20250414a to see other cases.
        /// Attempt to omit tables from a join that do not affect the result.
        ///* See the comment on whereOmitNoopJoin() for further information.
        ///*
        ///* This query optimization is factored out into a separate "no-inline"
        ///* procedure to keep the sqlite3WhereBegin() procedure from becoming
        ///* too large.  If sqlite3WhereBegin() becomes too large, that prevents
        ///* some C-compiler optimizers from in-lining the
        ///* sqlite3WhereCodeOneLoopStart() procedure, and it is important to
        ///* in-line sqlite3WhereCodeOneLoopStart() for performance reasons.
        /// Must be a join, or this opt8n is pointless
        /// Condition (1)
        /// (1),(6)
        /// (7)
        /// Check to see if there are any SEARCH loops that might benefit from
        ///* using a Bloom filter.
        /// If the caller is an UPDATE or DELETE statement that is requesting
        ///* to use a one-pass algorithm, determine if this is appropriate.
        ///*
        ///* A one-pass approach can be used if the caller has requested one
        ///* and either (a) the scan visits at most one row or (b) each
        ///* of the following are true:
        ///*
        ///*   * the caller has indicated that a one-pass approach can be used
        ///*     with multiple rows (by setting WHERE_ONEPASS_MULTIROW), and
        ///*   * the table is not a virtual table, and
        ///*   * either the scan does not use the OR optimization or the caller
        ///*     is a DELETE operation (WHERE_DUPLICATES_OK is only specified
        ///*     for DELETE).
        ///*
        ///* The last qualification is because an UPDATE statement uses
        ///* WhereInfo.aiCurOnePass[1] to determine whether or not it really can
        ///* use a one-pass approach, and this is not set accurately for scans
        ///* that use the OR optimization.
        /// Open all tables in the pTabList and any indices selected for
        ///* searching those tables.
        /// Table to open
        /// Index of database containing table/index
        /// Do nothing
        /// noop
        /// If we know that only a prefix of the record will be used,
        ///* it is advantageous to reduce the "column count" field in
        ///* the P4 operand of the OP_OpenRead/Write opcode.
        /// iAuxArg is always set to a positive value if ONEPASS is possible
        /// This is one term of an OR-optimization using the PRIMARY KEY of a
        ///* WITHOUT ROWID table.  No need for a separate index
        /// SQLITE_ENABLE_COLUMN_USED_MASK
        /// The nature of RIGHT JOIN processing is such that it messes up
        ///* the output order.  So omit any ORDER BY/GROUP BY elimination
        ///* optimizations.  We need to do an actual sort for RIGHT JOIN.
        /// Generate the code to do the search.  Each iteration of the for
        ///* loop below generates code for a single nested loop of the VM
        ///* program.
        /// Done.
        /// Jump here if malloc fails
        /// WHERETRACE_ENABLED
        unreachable!();
    }
}

///* Generate the end of the WHERE loop.  See comments on
///* sqlite3WhereBegin() for additional information.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_where_end(p_w_info_1: *mut WhereInfo) -> () {
    unsafe {
        let p_parse: *mut Parse = unsafe { (*p_w_info_1).p_parse };
        let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
        let mut i: i32 = 0;
        let mut p_level: *mut WhereLevel = core::ptr::null_mut();
        let mut p_loop: *mut WhereLoop = core::ptr::null_mut();
        let p_tab_list: *mut SrcList = unsafe { (*p_w_info_1).p_tab_list };
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        let i_end: i32 = unsafe { sqlite3_vdbe_current_addr(v) };
        let mut n_rj: i32 = 0;
        let mut addr_seek: i32 = 0;
        {
            i = unsafe { (*p_w_info_1).n_level } as i32 - 1;
            '__b108: loop {
                if !(i >= 0) { break '__b108; }
                '__c108: loop {
                    let mut addr: i32 = 0;
                    p_level =
                        unsafe {
                            &mut *(unsafe { (*p_w_info_1).a.as_ptr() } as
                                            *mut WhereLevel).offset(i as isize)
                        };
                    if !(unsafe { (*p_level).p_rj }).is_null() {
                        /// Terminate the subroutine that forms the interior of the loop of
                        ///* the RIGHT JOIN table
                        let p_rj: *mut WhereRightJoin = unsafe { (*p_level).p_rj };
                        unsafe {
                            sqlite3_vdbe_resolve_label(v,
                                unsafe { (*p_level).addr_cont })
                        };

                        /// Replace addrCont with a new label that will never be used, just so
                        ///* the subsequent call to resolve pLevel->addrCont will have something
                        ///* to resolve.
                        unsafe {
                            (*p_level).addr_cont =
                                unsafe { sqlite3_vdbe_make_label(p_parse) }
                        };
                        unsafe {
                            (*p_rj).end_subrtn = unsafe { sqlite3_vdbe_current_addr(v) }
                        };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 69, unsafe { (*p_rj).reg_return },
                                unsafe { (*p_rj).addr_subrtn }, 1)
                        };
                        { let __p = &mut n_rj; let __t = *__p; *__p += 1; __t };
                    }
                    p_loop = unsafe { (*p_level).p_w_loop };
                    if unsafe { (*p_level).op } as i32 != 189 {
                        let mut p_idx: *const Index = core::ptr::null();
                        let mut n: i32 = 0;
                        if unsafe { (*p_w_info_1).e_distinct } as i32 == 2 &&
                                                i == unsafe { (*p_w_info_1).n_level } as i32 - 1 &&
                                            unsafe { (*p_loop).ws_flags } & 512 as u32 != 0 as u32 &&
                                        unsafe {
                                                (*{
                                                                p_idx = unsafe { (*p_loop).u.btree.p_index };
                                                                p_idx
                                                            }).has_stat1()
                                            } != 0 &&
                                    {
                                            n = unsafe { (*p_loop).u.btree.n_distinct_col } as i32;
                                            n
                                        } > 0 &&
                                unsafe {
                                            *unsafe { (*p_idx).ai_row_log_est.offset(n as isize) }
                                        } as i32 >= 36 {
                            let r1: i32 = unsafe { (*p_parse).n_mem } + 1;
                            let mut j: i32 = 0;
                            let mut op: i32 = 0;
                            let mut addr_if_null: i32 = 0;
                            if unsafe { (*p_level).i_left_join } != 0 {
                                addr_if_null =
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 20, unsafe { (*p_level).i_idx_cur },
                                            r1)
                                    };
                            }
                            {
                                j = 0;
                                '__b109: loop {
                                    if !(j < n) { break '__b109; }
                                    '__c109: loop {
                                        unsafe {
                                            sqlite3_vdbe_add_op3(v, 96, unsafe { (*p_level).i_idx_cur },
                                                j, r1 + j)
                                        };
                                        break '__c109;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe { (*p_parse).n_mem += n + 1 };
                            op =
                                if unsafe { (*p_level).op } as i32 == 39 { 21 } else { 24 };
                            addr_seek =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, op,
                                        unsafe { (*p_level).i_idx_cur }, 0, r1, n)
                                };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 9, 1, unsafe { (*p_level).p2 })
                            };
                            if unsafe { (*p_level).i_left_join } != 0 {
                                unsafe { sqlite3_vdbe_jump_here(v, addr_if_null) };
                            }
                        }
                    }
                    if unsafe {
                                (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                    *mut SrcItem).add(unsafe { (*p_level).i_from } as
                                                    usize)).fg.from_exists()
                            } != 0 {

                        /// This is an EXISTS-to-JOIN optimization loop. If this loop sees a 
                        ///* successful row, it should break out of itself.
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 9, 0,
                                unsafe { (*p_level).addr_brk })
                        };
                    }
                    unsafe {
                        sqlite3_vdbe_resolve_label(v,
                            unsafe { (*p_level).addr_cont })
                    };
                    if unsafe { (*p_level).op } as i32 != 189 {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, unsafe { (*p_level).op } as i32,
                                unsafe { (*p_level).p1 }, unsafe { (*p_level).p2 },
                                unsafe { (*p_level).p3 } as i32)
                        };
                        unsafe {
                            sqlite3_vdbe_change_p5(v, unsafe { (*p_level).p5 } as u16)
                        };
                        if unsafe { (*p_level).reg_bignull } != 0 {
                            unsafe {
                                sqlite3_vdbe_resolve_label(v,
                                    unsafe { (*p_level).addr_bignull })
                            };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 63,
                                    unsafe { (*p_level).reg_bignull },
                                    unsafe { (*p_level).p2 } - 1)
                            };
                        }
                        if addr_seek != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_seek) };
                            addr_seek = 0;
                        }
                    }
                    if unsafe { (*p_loop).ws_flags } & 2048 as u32 != 0 as u32
                            && unsafe { (*p_level).u.in_.n_in } > 0 {
                        let mut p_in: *const InLoop = core::ptr::null();
                        let mut j: i32 = 0;
                        unsafe {
                            sqlite3_vdbe_resolve_label(v,
                                unsafe { (*p_level).addr_nxt })
                        };
                        {
                            {
                                j = unsafe { (*p_level).u.in_.n_in };
                                p_in =
                                    unsafe {
                                        unsafe {
                                            (*p_level).u.in_.a_in_loop.offset((j - 1) as isize)
                                        }
                                    }
                            };
                            '__b110: loop {
                                if !(j > 0) { break '__b110; }
                                '__c110: loop {
                                    { let _ = 0; };
                                    unsafe {
                                        sqlite3_vdbe_jump_here(v,
                                            unsafe { (*p_in).addr_in_top } + 1)
                                    };
                                    if unsafe { (*p_in).e_end_loop_op } as i32 != 189 {
                                        if unsafe { (*p_in).n_prefix } != 0 {
                                            let b_early_out: i32 =
                                                (unsafe { (*p_loop).ws_flags } & 1024 as u32 == 0 as u32 &&
                                                        unsafe { (*p_loop).ws_flags } & 262144 as u32 != 0 as u32)
                                                    as i32;
                                            if unsafe { (*p_level).i_left_join } != 0 {

                                                /// For LEFT JOIN queries, cursor pIn->iCur may not have been
                                                ///* opened yet. This occurs for WHERE clauses such as
                                                ///* "a = ? AND b IN (...)", where the index is on (a, b). If
                                                ///* the RHS of the (a=?) is NULL, then the "b IN (...)" may
                                                ///* never have been coded, but the body of the loop run to
                                                ///* return the null-row. So, if the cursor is not open yet,
                                                ///* jump over the OP_Next or OP_Prev instruction about to
                                                ///* be coded.
                                                unsafe {
                                                    sqlite3_vdbe_add_op2(v, 25, unsafe { (*p_in).i_cur },
                                                        unsafe { sqlite3_vdbe_current_addr(v) } + 2 + b_early_out)
                                                };
                                            }
                                            if b_early_out != 0 {
                                                unsafe {
                                                    sqlite3_vdbe_add_op4_int(v, 26,
                                                        unsafe { (*p_level).i_idx_cur },
                                                        unsafe { sqlite3_vdbe_current_addr(v) } + 2,
                                                        unsafe { (*p_in).i_base }, unsafe { (*p_in).n_prefix })
                                                };

                                                /// Retarget the OP_IsNull against the left operand of IN so
                                                ///* it jumps past the OP_IfNoHope.  This is because the
                                                ///* OP_IsNull also bypasses the OP_Affinity opcode that is
                                                ///* required by OP_IfNoHope.
                                                unsafe {
                                                    sqlite3_vdbe_jump_here(v,
                                                        unsafe { (*p_in).addr_in_top } + 1)
                                                };
                                            }
                                        }
                                        unsafe {
                                            sqlite3_vdbe_add_op2(v,
                                                unsafe { (*p_in).e_end_loop_op } as i32,
                                                unsafe { (*p_in).i_cur }, unsafe { (*p_in).addr_in_top })
                                        };
                                    }
                                    unsafe {
                                        sqlite3_vdbe_jump_here(v,
                                            unsafe { (*p_in).addr_in_top } - 1)
                                    };
                                    break '__c110;
                                }
                                {
                                    { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                    {
                                        let __p = &mut p_in;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(-1) };
                                        __t
                                    }
                                };
                            }
                        }
                    }
                    unsafe {
                        sqlite3_vdbe_resolve_label(v,
                            unsafe { (*p_level).addr_brk })
                    };
                    if !(unsafe { (*p_level).p_rj }).is_null() {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 69,
                                unsafe { (*unsafe { (*p_level).p_rj }).reg_return }, 0, 1)
                        };
                    }
                    if unsafe { (*p_level).addr_skip } != 0 {
                        unsafe {
                            sqlite3_vdbe_goto(v, unsafe { (*p_level).addr_skip })
                        };
                        unsafe {
                            sqlite3_vdbe_jump_here(v, unsafe { (*p_level).addr_skip })
                        };
                        unsafe {
                            sqlite3_vdbe_jump_here(v,
                                unsafe { (*p_level).addr_skip } - 2)
                        };
                    }
                    if unsafe { (*p_level).addr_like_rep } != 0 {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 63,
                                (unsafe { (*p_level).i_like_rep_cntr } >> 1) as i32,
                                unsafe { (*p_level).addr_like_rep })
                        };
                    }
                    if unsafe { (*p_level).i_left_join } != 0 {
                        let ws: i32 = unsafe { (*p_loop).ws_flags } as i32;
                        addr =
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 61,
                                    unsafe { (*p_level).i_left_join })
                            };
                        { let _ = 0; };
                        if ws & 64 == 0 {
                            let p_src: *const SrcItem =
                                unsafe {
                                        &raw mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                        *mut SrcItem).add(unsafe { (*p_level).i_from } as usize)
                                    } as *const SrcItem;
                            { let _ = 0; };
                            if unsafe { (*p_src).fg.via_coroutine() } != 0 {
                                let mut m: i32 = 0;
                                let mut n: i32 = 0;
                                { let _ = 0; };
                                n = unsafe { (*unsafe { (*p_src).u4.p_subq }).reg_result };
                                { let _ = 0; };
                                m = unsafe { (*unsafe { (*p_src).p_s_tab }).n_col } as i32;
                                unsafe { sqlite3_vdbe_add_op3(v, 77, 0, n, n + m - 1) };
                            }
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 138,
                                    unsafe { (*p_level).i_tab_cur })
                            };
                        }
                        if ws & 512 != 0 ||
                                ws & 8192 != 0 &&
                                    !(unsafe { (*p_level).u.p_covering_idx }).is_null() {
                            if ws & 8192 != 0 {
                                let p_ix: *mut Index =
                                    unsafe { (*p_level).u.p_covering_idx };
                                let i_db: i32 =
                                    unsafe {
                                        sqlite3_schema_to_index(db, unsafe { (*p_ix).p_schema })
                                    };
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 113,
                                        unsafe { (*p_level).i_idx_cur },
                                        unsafe { (*p_ix).tnum } as i32, i_db)
                                };
                                unsafe { sqlite3_vdbe_set_p4_key_info(p_parse, p_ix) };
                            }
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 138,
                                    unsafe { (*p_level).i_idx_cur })
                            };
                        }
                        if unsafe { (*p_level).op } as i32 == 69 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 10, unsafe { (*p_level).p1 },
                                    unsafe { (*p_level).addr_first })
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_goto(v, unsafe { (*p_level).addr_first })
                            };
                        }
                        unsafe { sqlite3_vdbe_jump_here(v, addr) };
                    }
                    break '__c108;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        { let _ = 0; };
        {
            {
                i = 0;
                p_level =
                    unsafe { (*p_w_info_1).a.as_ptr() } as *mut WhereLevel
            };
            '__b111: loop {
                if !(i < unsafe { (*p_w_info_1).n_level } as i32) {
                    break '__b111;
                }
                '__c111: loop {
                    let mut k: i32 = 0;
                    let mut last: i32 = 0;
                    let mut p_op: *mut VdbeOp = core::ptr::null_mut();
                    let mut p_last_op: *mut VdbeOp = core::ptr::null_mut();
                    let mut p_idx_1: *mut Index = core::ptr::null_mut();
                    let p_tab_item: *const SrcItem =
                        unsafe {
                                &raw mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).add(unsafe { (*p_level).i_from } as usize)
                            } as *const SrcItem;
                    let p_tab: *mut Table = unsafe { (*p_tab_item).p_s_tab };
                    { let _ = 0; };
                    p_loop = unsafe { (*p_level).p_w_loop };
                    if !(unsafe { (*p_level).p_rj }).is_null() {
                        unsafe {
                            sqlite3_where_right_join_loop(p_w_info_1, i, p_level)
                        };
                        break '__c111;
                    }
                    if unsafe { (*p_tab_item).fg.via_coroutine() } != 0 {
                        { let _ = 0; };
                        { let _ = 0; };
                        translate_column_to_copy(unsafe { &*p_parse },
                            unsafe { (*p_level).addr_body },
                            unsafe { (*p_level).i_tab_cur },
                            unsafe { (*unsafe { (*p_tab_item).u4.p_subq }).reg_result },
                            0);
                        break '__c111;
                    }
                    if unsafe { (*p_loop).ws_flags } & (512 | 64) as u32 != 0 {
                        p_idx_1 = unsafe { (*p_loop).u.btree.p_index };
                    } else if unsafe { (*p_loop).ws_flags } & 8192 as u32 != 0 {
                        p_idx_1 = unsafe { (*p_level).u.p_covering_idx };
                    }
                    if !(p_idx_1).is_null() &&
                            (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                        if unsafe { (*p_w_info_1).e_one_pass } as i32 == 0 ||
                                !(unsafe { (*unsafe { (*p_idx_1).p_table }).tab_flags } &
                                                    128 as u32 == 0 as u32) as i32 != 0 {
                            last = i_end;
                        } else { last = unsafe { (*p_w_info_1).i_end_where }; }
                        if unsafe { (*p_idx_1).b_has_expr() } != 0 {
                            let mut p: *mut IndexedExpr =
                                unsafe { (*p_parse).p_idx_epr };
                            while !(p).is_null() {
                                if unsafe { (*p).i_idx_cur } ==
                                        unsafe { (*p_level).i_idx_cur } {
                                    unsafe { (*p).i_data_cur = -1 };
                                    unsafe { (*p).i_idx_cur = -1 };
                                }
                                p = unsafe { (*p).p_ie_next };
                            }
                        }
                        k = unsafe { (*p_level).addr_body } + 1;
                        p_op = unsafe { sqlite3_vdbe_get_op(v, k) };
                        p_last_op = unsafe { p_op.offset((last - k) as isize) };
                        { let _ = 0; };
                        '__b113: loop {
                            '__c113: loop {
                                if unsafe { (*p_op).p1 } != unsafe { (*p_level).i_tab_cur }
                                    {} else if unsafe { (*p_op).opcode } as i32 == 96 {
                                    let mut x: i32 = unsafe { (*p_op).p2 };
                                    { let _ = 0; };
                                    if !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32)
                                                as i32 != 0 {
                                        let p_pk: *const Index =
                                            unsafe { sqlite3_primary_key_index(p_tab) } as *const Index;
                                        x =
                                            unsafe { *unsafe { (*p_pk).ai_column.offset(x as isize) } }
                                                as i32;
                                        { let _ = 0; };
                                    } else {
                                        x =
                                            unsafe { sqlite3_storage_column_to_table(p_tab, x as i16) }
                                                as i32;
                                    }
                                    x = unsafe { sqlite3_table_column_to_index(p_idx_1, x) };
                                    if x >= 0 {
                                        unsafe { (*p_op).p2 = x };
                                        unsafe { (*p_op).p1 = unsafe { (*p_level).i_idx_cur } };
                                    } else if unsafe { (*p_loop).ws_flags } &
                                                (64 | 67108864) as u32 != 0 {
                                        if unsafe { (*p_loop).ws_flags } & 64 as u32 != 0 {

                                            /// An error. pLoop is supposed to be a covering index loop,
                                            ///* and yet the VM code refers to a column of the table that 
                                            ///* is not part of the index.
                                            unsafe {
                                                sqlite3_error_msg(p_parse,
                                                    c"internal query planner error".as_ptr() as *mut i8 as
                                                        *const i8)
                                            };
                                            unsafe { (*p_parse).rc = 2 };
                                        } else {

                                            /// The WHERE_EXPRIDX flag is set by the planner when it is likely
                                            ///* that pLoop is a covering index loop, but it is not possible
                                            ///* to be 100% sure. In this case, any OP_Explain opcode
                                            ///* corresponding to this loop describes the index as a "COVERING
                                            ///* INDEX". But, pOp proves that pLoop is not actually a covering 
                                            ///* index loop. So clear the WHERE_EXPRIDX flag and rewrite the
                                            ///* text that accompanies the OP_Explain opcode, if any.
                                            unsafe { (*p_loop).ws_flags &= !67108864 as u32 };
                                            unsafe {
                                                sqlite3_where_add_explain_text(p_parse,
                                                    unsafe { (*p_level).addr_body } - 1, p_tab_list, p_level,
                                                    unsafe { (*p_w_info_1).wctrl_flags })
                                            };
                                        }
                                    }
                                } else if unsafe { (*p_op).opcode } as i32 == 137 {
                                    unsafe { (*p_op).p1 = unsafe { (*p_level).i_idx_cur } };
                                    unsafe { (*p_op).opcode = 144 as u8 };
                                } else if unsafe { (*p_op).opcode } as i32 == 20 {
                                    unsafe { (*p_op).p1 = unsafe { (*p_level).i_idx_cur } };
                                }
                                break '__c113;
                            }
                            if !({
                                                let __p = &mut p_op;
                                                *__p = unsafe { (*__p).offset(1) };
                                                *__p
                                            } < p_last_op) {
                                break '__b113;
                            }
                        }
                    }
                    break '__c111;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_level;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }

        /// The "break" point is here, just past the end of the outer loop.
        ///* Set it.
        unsafe {
            sqlite3_vdbe_resolve_label(v, unsafe { (*p_w_info_1).i_break })
        };

        /// Final cleanup
        unsafe {
            (*p_parse).n_query_loop =
                unsafe { (*p_w_info_1).saved_n_query_loop } as LogEst
        };
        where_info_free(db, p_w_info_1);
        unsafe { (*p_parse).within_rj_subrtn -= n_rj as u8 };
        return;
    }
}

///* Return the estimated number of output rows from a WHERE clause
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_output_row_count(p_w_info_1: &WhereInfo)
    -> LogEst {
    return (*p_w_info_1).n_row_out;
}

///* Return one of the WHERE_DISTINCT_xxxxx values to indicate how this
///* WHERE clause returns outputs for DISTINCT processing.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_is_distinct(p_w_info_1: &WhereInfo) -> i32 {
    return (*p_w_info_1).e_distinct as i32;
}

///* Return the number of ORDER BY terms that are satisfied by the
///* WHERE clause.  A return of 0 means that the output must be
///* completely sorted.  A return equal to the number of ORDER BY
///* terms means that no sorting is needed at all.  A return that
///* is positive but less than the number of ORDER BY terms means that
///* block sorting is required.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_is_ordered(p_w_info_1: &WhereInfo) -> i32 {
    return if ((*p_w_info_1).n_ob_sat as i32) < 0 {
            0
        } else { (*p_w_info_1).n_ob_sat as i32 };
}

///* In the ORDER BY LIMIT optimization, if the inner-most loop is known
///* to emit rows in increasing order, and if the last row emitted by the
///* inner-most loop did not fit within the sorter, then we can skip all
///* subsequent rows for the current iteration of the inner loop (because they
///* will not fit in the sorter either) and continue with the second inner
///* loop - the loop immediately outside the inner-most.
///*
///* When a row does not fit in the sorter (because the sorter already
///* holds LIMIT+OFFSET rows that are smaller), then a jump is made to the
///* label returned by this function.
///*
///* If the ORDER BY LIMIT optimization applies, the jump destination should
///* be the continuation for the second-inner-most loop.  If the ORDER BY
///* LIMIT optimization does not apply, then the jump destination should
///* be the continuation for the inner-most loop.
///*
///* It is always safe for this routine to return the continuation of the
///* inner-most loop, in the sense that a correct answer will result. 
///* Returning the continuation the second inner loop is an optimization
///* that might make the code run a little faster, but should not change
///* the final answer.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_where_order_by_limit_opt_label(p_w_info_1:
        &mut WhereInfo) -> i32 {
    let mut p_inner: *const WhereLevel = core::ptr::null();
    if ((*p_w_info_1).b_ordered_inner_loop() == 0) as i32 != 0 {

        /// The ORDER BY LIMIT optimization does not apply.  Jump to the
        ///* continuation of the inner-most loop.
        return (*p_w_info_1).i_continue;
    }
    p_inner =
        unsafe {
            &mut *((*p_w_info_1).a.as_ptr() as
                            *mut WhereLevel).offset(((*p_w_info_1).n_level as i32 - 1)
                            as isize)
        };
    { let _ = 0; };
    return if !(unsafe { (*p_inner).p_rj }).is_null() {
            (*p_w_info_1).i_continue
        } else { unsafe { (*p_inner).addr_nxt } };
}

///* While generating code for the min/max optimization, after handling
///* the aggregate-step call to min() or max(), check to see if any
///* additional looping is required.  If the output order is such that
///* we are certain that the correct answer has already been found, then
///* code an OP_Goto to by pass subsequent processing.
///*
///* Any extra OP_Goto that is coded here is an optimization.  The
///* correct answer should be obtained regardless.  This OP_Goto just
///* makes the answer appear faster.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_min_max_opt_early_out(v: *mut Vdbe,
    p_w_info_1: &mut WhereInfo) -> () {
    let mut p_inner: *const WhereLevel = core::ptr::null();
    let mut i: i32 = 0;
    if ((*p_w_info_1).b_ordered_inner_loop() == 0) as i32 != 0 { return; }
    if (*p_w_info_1).n_ob_sat as i32 == 0 { return; }
    {
        i = (*p_w_info_1).n_level as i32 - 1;
        '__b114: loop {
            if !(i >= 0) { break '__b114; }
            '__c114: loop {
                p_inner =
                    unsafe {
                        &mut *((*p_w_info_1).a.as_ptr() as
                                        *mut WhereLevel).offset(i as isize)
                    };
                if unsafe { (*unsafe { (*p_inner).p_w_loop }).ws_flags } &
                            4 as u32 != 0 as u32 {
                    unsafe {
                        sqlite3_vdbe_goto(v, unsafe { (*p_inner).addr_nxt })
                    };
                    return;
                }
                break '__c114;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
    unsafe { sqlite3_vdbe_goto(v, (*p_w_info_1).i_break) };
}

///* If the WHERE_GROUPBY flag is set in the mask passed to sqlite3WhereBegin(),
///* the planner assumes that the specified pOrderBy list is actually a GROUP
///* BY clause - and so any order that groups rows as required satisfies the
///* request.
///*
///* Normally, in this case it is not possible for the caller to determine
///* whether or not the rows are really being delivered in sorted order, or
///* just in some other order that provides the required grouping. However,
///* if the WHERE_SORTBYGROUP flag is also passed to sqlite3WhereBegin(), then
///* this function may be called on the returned WhereInfo object. It returns
///* true if the rows really will be sorted in the specified order, or false
///* otherwise.
///*
///* For example, assuming:
///*
///*   CREATE INDEX i1 ON t1(x, Y);
///*
///* then
///*
///*   SELECT * FROM t1 GROUP BY x,y ORDER BY x,y;   -- IsSorted()==1
///*   SELECT * FROM t1 GROUP BY y,x ORDER BY y,x;   -- IsSorted()==0
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_is_sorted(p_w_info_1: &WhereInfo) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    return (*p_w_info_1).sorted() as i32;
}

///* Return the VDBE address or label to jump to in order to continue
///* immediately with the next row of a WHERE clause.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_continue_label(p_w_info_1: &WhereInfo)
    -> i32 {
    { let _ = 0; };
    return (*p_w_info_1).i_continue;
}

///* Return the VDBE address or label to jump to in order to break
///* out of a WHERE loop.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_break_label(p_w_info_1: &WhereInfo) -> i32 {
    return (*p_w_info_1).i_break;
}

///* Return ONEPASS_OFF (0) if an UPDATE or DELETE statement is unable to
///* operate directly on the rowids returned by a WHERE clause.  Return
///* ONEPASS_SINGLE (1) if the statement can operation directly because only
///* a single row is to be changed.  Return ONEPASS_MULTI (2) if the one-pass
///* optimization can be used on multiple
///*
///* If the ONEPASS optimization is used (if this routine returns true)
///* then also write the indices of open cursors used by ONEPASS
///* into aiCur[0] and aiCur[1].  iaCur[0] gets the cursor of the data
///* table and aiCur[1] gets the cursor used by an auxiliary index.
///* Either value may be -1, indicating that cursor is not used.
///* Any cursors returned will have been opened for writing.
///*
///* aiCur[0] and aiCur[1] both get -1 if the where-clause logic is
///* unable to use the ONEPASS optimization.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_ok_one_pass(p_w_info_1: &WhereInfo,
    ai_cur_1: *mut i32) -> i32 {
    unsafe {
        memcpy(ai_cur_1 as *mut (),
            &raw const (*p_w_info_1).ai_cur_one_pass[0 as usize] as *mut i32
                as *const (), core::mem::size_of::<i32>() as u64 * 2 as u64)
    };
    return (*p_w_info_1).e_one_pass as i32;
}

///* Return TRUE if the WHERE loop uses the OP_DeferredSeek opcode to move
///* the data cursor to the row selected by the index cursor.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_uses_deferred_seek(p_w_info_1: &WhereInfo)
    -> i32 {
    return (*p_w_info_1).b_deferred_seek() as i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_where_realloc(p_w_info_1: *mut WhereInfo,
    p_old_1: *mut (), n_byte_1: u64) -> *mut () {
    unsafe {
        let p_new: *mut () =
            sqlite3_where_malloc(unsafe { &mut *p_w_info_1 }, n_byte_1);
        if !(p_new).is_null() && !(p_old_1).is_null() {
            let mut p_old_blk: *const WhereMemBlock =
                p_old_1 as *mut WhereMemBlock as *const WhereMemBlock;
            {
                let __p = &mut p_old_blk;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(-1) };
                __t
            };
            { let _ = 0; };
            unsafe {
                memcpy(p_new, p_old_1 as *const (),
                    unsafe { (*p_old_blk).sz })
            };
        }
        return p_new;
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
    fn sqlite3_expr_compare_coll_seq(_: *mut Parse, _: *const Expr)
    -> *mut CollSeq;
    static sqlite3_str_binary: [i8; 0];
    fn sqlite3_vtab_in_first(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_vtab_in_next(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_misuse_error(_: i32)
    -> i32;
    fn sqlite3_value_from_expr(_: *mut Sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut Sqlite3Value)
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
    fn sqlite3_update(_: *mut Parse, _: *mut SrcList, _: *mut ExprList,
    _: *mut Expr, _: i32, _: *mut ExprList, _: *mut Expr, _: *mut Upsert)
    -> ();
    fn sqlite3_where_clause_clear(_: *mut WhereClause)
    -> ();
    fn sqlite3_where_clause_init(_: *mut WhereClause, _: *mut WhereInfo)
    -> ();
    fn sqlite3_where_split(_: *mut WhereClause, _: *mut Expr, _: u8)
    -> ();
    fn sqlite3_where_tab_func_args(_: *mut Parse, _: *mut SrcItem,
    _: *mut WhereClause)
    -> ();
    fn sqlite3_where_expr_analyze(_: *mut SrcList, _: *mut WhereClause)
    -> ();
    fn sqlite3_where_add_limit(_: *mut WhereClause, _: *mut Select)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_expr_if_false(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_skip_collate_and_likely(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_expr_affinity(p_expr_1: *const Expr)
    -> i8;
    fn sqlite3_expr_compare_skip(_: *mut Expr, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_is_vector(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_index_affinity_ok(p_expr_1: *const Expr, idx_affinity: i8)
    -> i32;
    fn sqlite3_expr_nn_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_expr_is_constant(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_get_v_table(_: *mut Sqlite3, _: *mut Table)
    -> *mut VTable;
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_err_str(_: i32)
    -> *const i8;
    fn sqlite3_code_verify_schema(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_begin_write_operation(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_log_est_from_double(_: f64)
    -> LogEst;
    fn sqlite3_log_est(_: u64)
    -> LogEst;
    fn sqlite3_log_est_add(_: LogEst, _: LogEst)
    -> LogEst;
    fn sqlite3_expr_implies_expr(_: *const Parse, _: *const Expr,
    _: *const Expr, _: i32)
    -> i32;
    fn sqlite3_expr_is_integer(_: *const Expr, _: *mut i32, _: *mut Parse)
    -> i32;
    fn sqlite3_expr_is_like_operator(_: *const Expr)
    -> i32;
    fn sqlite3_is_binary(_: *const CollSeq)
    -> i32;
    fn sqlite3_expr_dup(_: *mut Sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_expr_compare(_: *const Parse, _: *const Expr, _: *const Expr,
    _: i32)
    -> i32;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_expr_covered_by_index(_: *mut Expr, i_cur_1: i32,
    p_idx_1: *mut Index)
    -> i32;
    fn sqlite3_expr_vector_size(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_compare_affinity(p_expr_1: *const Expr, aff2: i8)
    -> i8;
    fn sqlite3_table_column_affinity(_: *const Table, _: i32)
    -> i8;
    fn sqlite3_binary_compare_coll_seq(_: *mut Parse, _: *const Expr,
    _: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_where_expr_usage(_: *mut WhereMaskSet, _: *mut Expr)
    -> Bitmask;
    fn sqlite3_where_expr_list_usage(_: *mut WhereMaskSet, _: *mut ExprList)
    -> Bitmask;
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_index_affinity_str(_: *mut Sqlite3, _: *mut Index)
    -> *const i8;
    fn sqlite3_key_info_alloc(_: *mut Sqlite3, _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_expr_is_single_table_constraint(_: *mut Expr,
    _: *const SrcList, _: i32, _: i32)
    -> i32;
    fn sqlite3_where_explain_bloom_filter(p_parse_1: *const Parse,
    p_w_info_1: *const WhereInfo, p_level_1: *const WhereLevel)
    -> i32;
    fn sqlite3_generate_index_key(_: *mut Parse, _: *mut Index, _: i32,
    _: i32, _: i32, _: *mut i32, _: *mut Index, _: i32)
    -> i32;
    fn sqlite3_log_est_to_int(_: LogEst)
    -> u64;
    fn sqlite3_expr_code_load_index_column(_: *mut Parse, _: *mut Index,
    _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_where_explain_one_scan(p_parse_1: *mut Parse,
    p_tab_list_1: *mut SrcList, p_level_1: *mut WhereLevel,
    wctrl_flags_1: u16)
    -> i32;
    fn sqlite3_where_code_one_loop_start(p_parse_1: *mut Parse, v: *mut Vdbe,
    p_w_info_1: *mut WhereInfo, i_level_1: i32, p_level_1: *mut WhereLevel,
    not_ready_1: Bitmask)
    -> Bitmask;
    fn sqlite3_where_right_join_loop(p_w_info_1: *mut WhereInfo,
    i_level_1: i32, p_level_1: *mut WhereLevel)
    -> ();
    fn sqlite3_where_add_explain_text(p_parse_1: *mut Parse, addr: i32,
    p_tab_list_1: *mut SrcList, p_level_1: *mut WhereLevel,
    wctrl_flags_1: u16)
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
    fn sqlite3_expr_list_compare(_: *const ExprList, _: *const ExprList,
    _: i32)
    -> i32;
    fn sqlite3_expr_implies_non_null_row(_: *mut Expr, _: i32, _: i32)
    -> i32;
    fn sqlite3_agg_info_persist_walker_init(_: *mut Walker, _: *mut Parse)
    -> ();
    fn sqlite3_expr_analyze_aggregates(_: *mut NameContext, _: *mut Expr)
    -> ();
    fn sqlite3_expr_analyze_agg_list(_: *mut NameContext, _: *mut ExprList)
    -> ();
    fn sqlite3_references_src_list(_: *mut Parse, _: *mut Expr,
    _: *mut SrcList)
    -> i32;
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_prng_save_state()
    -> ();
    fn sqlite3_prng_restore_state()
    -> ();
    fn sqlite3_rollback_all(_: *mut Sqlite3, _: i32)
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
    fn sqlite3_expr_is_constant_or_function(_: *mut Expr, _: u8)
    -> i32;
    fn sqlite3_expr_is_constant_or_group_by(_: *mut Parse, _: *mut Expr,
    _: *mut ExprList)
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
    fn sqlite3_expr_list_dup(_: *mut Sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_src_list_dup(_: *mut Sqlite3, _: *const SrcList, _: i32)
    -> *mut SrcList;
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
    fn sqlite3_table_affinity(_: *mut Vdbe, _: *mut Table, _: i32)
    -> ();
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
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_find_coll_seq(_: *mut Sqlite3, enc: u8, _: *const i8, _: i32)
    -> *mut CollSeq;
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_set_text_encoding(db: *mut Sqlite3, _: u8)
    -> ();
    fn sqlite3_expr_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
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
    fn sqlite3_result_int_real(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_value_new(_: *mut Sqlite3)
    -> *mut Sqlite3Value;
    fn sqlite3_utf16to8(_: *mut Sqlite3, _: *const (), _: i32, _: u8)
    -> *mut i8;
    fn sqlite3_value_apply_affinity(_: *mut Sqlite3Value, _: u8, _: u8)
    -> ();
    static sqlite3_opcode_property: [u8; 0];
    static sqlite3_std_type_len: [u8; 0];
    static sqlite3_std_type_affinity: [i8; 0];
    static mut sqlite3_std_type: [*const i8; 0];
    static sqlite3_upper_to_lower: [u8; 0];
    static mut sqlite3a_l_tb: *const u8;
    static mut sqlite3a_e_qb: *const u8;
    static mut sqlite3a_g_tb: *const u8;
    static sqlite3_ctype_map: [u8; 0];
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
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
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
    fn sqlite3_reprepare(_: *mut Vdbe)
    -> i32;
    fn sqlite3_expr_list_check_length(_: *mut Parse, _: *mut ExprList,
    _: *const i8)
    -> ();
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
    fn sqlite3_vector_field_subexpr(_: *mut Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_for_vector_field(_: *mut Parse, _: *mut Expr, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_vector_error_msg(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_compile_options(pn_opt_1: *mut i32)
    -> *mut *const i8;
    fn sqlite3_where_expr_usage_nn(_: *mut WhereMaskSet, _: *mut Expr)
    -> Bitmask;
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
struct CheckOnCtx {
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
