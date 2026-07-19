#![allow(unused_imports, dead_code)]

mod btree_h;
mod hash_h;
mod pager_h;
mod pcache_h;
mod sqlite3_h;
mod sqlite_int_h;
mod vdbe_h;
mod vdbe_int_h;
use crate::btree_h::{BtCursor, Btree, BtreePayload};
use crate::hash_h::Hash;
use crate::pager_h::{DbPage, Pager, Pgno};
use crate::pcache_h::{PCache, PgHdr};
use crate::sqlite3_h::{
    Sqlite3Backup, Sqlite3Blob, Sqlite3File, Sqlite3Filename,
    Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module, Sqlite3Mutex,
    Sqlite3MutexMethods, Sqlite3PcachePage, Sqlite3RtreeGeometry,
    Sqlite3RtreeQueryInfo, Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Uint64,
    Sqlite3Vfs, Sqlite3Vtab, SqliteInt64,
};
use crate::sqlite_int_h::{
    AuthContext, Bft, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte,
    DbFixer, Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode,
    FuncDef, FuncDefHash, FuncDestructor, IdList, Index, KeyInfo, LogEst,
    Module, NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema,
    Select, SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str,
    SrcItem, SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
    TriggerStep, UnpackedRecord, Upsert, VList, VTable, Walker, WhereInfo,
    Window, With,
};
use crate::vdbe_h::{Mem, SubProgram, VdbeOp, VdbeOpList};
use crate::vdbe_int_h::{
    AuxData, MemValue, Op, Sqlite3Context, Sqlite3Value, ValueList, Vdbe,
    VdbeCursor, VdbeFrame, VdbeSorter,
};

type DarwinSizeT = u64;

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

///* CAPI3REF: Retrieving Statement SQL
///* METHOD: sqlite3_stmt
///*
///* ^The sqlite3_sql(P) interface returns a pointer to a copy of the UTF-8
///* SQL text used to create [prepared statement] P if P was
///* created by [sqlite3_prepare_v2()], [sqlite3_prepare_v3()],
///* [sqlite3_prepare16_v2()], or [sqlite3_prepare16_v3()].
///* ^The sqlite3_expanded_sql(P) interface returns a pointer to a UTF-8
///* string containing the SQL text of prepared statement P with
///* [bound parameters] expanded.
///* ^The sqlite3_normalized_sql(P) interface returns a pointer to a UTF-8
///* string containing the normalized SQL text of prepared statement P.  The
///* semantics used to normalize a SQL statement are unspecified and subject
///* to change.  At a minimum, literal values will be replaced with suitable
///* placeholders.
///*
///* ^(For example, if a prepared statement is created using the SQL
///* text "SELECT $abc,:xyz" and if parameter $abc is bound to integer 2345
///* and parameter :xyz is unbound, then sqlite3_sql() will return
///* the original string, "SELECT $abc,:xyz" but sqlite3_expanded_sql()
///* will return "SELECT 2345,NULL".)^
///*
///* ^The sqlite3_expanded_sql() interface returns NULL if insufficient memory
///* is available to hold the result, or if the result would exceed the
///* maximum string length determined by the [SQLITE_LIMIT_LENGTH].
///*
///* ^The [SQLITE_TRACE_SIZE_LIMIT] compile-time option limits the size of
///* bound parameter expansions.  ^The [SQLITE_OMIT_TRACE] compile-time
///* option causes sqlite3_expanded_sql() to always return NULL.
///*
///* ^The strings returned by sqlite3_sql(P) and sqlite3_normalized_sql(P)
///* are managed by SQLite and are automatically freed when the prepared
///* statement is finalized.
///* ^The string returned by sqlite3_expanded_sql(P), on the other hand,
///* is obtained from [sqlite3_malloc()] and must be freed by the application
///* by passing it to [sqlite3_free()].
///*
///* ^The sqlite3_normalized_sql() interface is only available if
///* the [SQLITE_ENABLE_NORMALIZE] compile-time option is defined.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_sql(p_stmt: *mut Sqlite3Stmt) -> *const i8 {
    let p: *const Vdbe = p_stmt as *mut Vdbe as *const Vdbe;
    return if !(p).is_null() {
                unsafe { (*p).z_sql }
            } else { core::ptr::null_mut() } as *const i8;
}

///* Return the SQL associated with a prepared statement with
///* bound parameters expanded.  Space to hold the returned string is
///* obtained from sqlite3_malloc().  The caller is responsible for
///* freeing the returned string by passing it to sqlite3_free().
///*
///* The SQLITE_TRACE_SIZE_LIMIT puts an upper bound on the size of
///* expanded bound parameters.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expanded_sql(p_stmt: *mut Sqlite3Stmt) -> *mut i8 {
    let mut z: *mut i8 = core::ptr::null_mut();
    let z_sql: *const i8 = sqlite3_sql(p_stmt);
    if !(z_sql).is_null() {
        let p: *mut Vdbe = p_stmt as *mut Vdbe;
        unsafe {
            sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex })
        };
        z = unsafe { sqlite3_vdbe_expand_sql(p, z_sql) };
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    }
    return z;
}

///* CAPI3REF: Determine If An SQL Statement Writes The Database
///* METHOD: sqlite3_stmt
///*
///* ^The sqlite3_stmt_readonly(X) interface returns true (non-zero) if
///* and only if the [prepared statement] X makes no direct changes to
///* the content of the database file.
///*
///* Note that [application-defined SQL functions] or
///* [virtual tables] might change the database indirectly as a side effect.
///* ^(For example, if an application defines a function "eval()" that
///* calls [sqlite3_exec()], then the following SQL statement would
///* change the database file through side-effects:
///*
///* <blockquote><pre>
///*    SELECT eval('DELETE FROM t1') FROM t2;
///* </pre></blockquote>
///*
///* But because the [SELECT] statement does not change the database file
///* directly, sqlite3_stmt_readonly() would still return true.)^
///*
///* ^Transaction control statements such as [BEGIN], [COMMIT], [ROLLBACK],
///* [SAVEPOINT], and [RELEASE] cause sqlite3_stmt_readonly() to return true,
///* since the statements themselves do not actually modify the database but
///* rather they control the timing of when other statements modify the
///* database.  ^The [ATTACH] and [DETACH] statements also cause
///* sqlite3_stmt_readonly() to return true since, while those statements
///* change the configuration of a database connection, they do not make
///* changes to the content of the database files on disk.
///* ^The sqlite3_stmt_readonly() interface returns true for [BEGIN] since
///* [BEGIN] merely sets internal flags, but the [BEGIN|BEGIN IMMEDIATE] and
///* [BEGIN|BEGIN EXCLUSIVE] commands do touch the database and so
///* sqlite3_stmt_readonly() returns false for those commands.
///*
///* ^This routine returns false if there is any possibility that the
///* statement might change the database file.  ^A false return does
///* not guarantee that the statement will change the database file.
///* ^For example, an UPDATE statement might have a WHERE clause that
///* makes it a no-op, but the sqlite3_stmt_readonly() result would still
///* be false.  ^Similarly, a CREATE TABLE IF NOT EXISTS statement is a
///* read-only no-op if the table already exists, but
///* sqlite3_stmt_readonly() still returns false for such a statement.
///*
///* ^If prepared statement X is an [EXPLAIN] or [EXPLAIN QUERY PLAN]
///* statement, then sqlite3_stmt_readonly(X) returns the same value as
///* if the EXPLAIN or EXPLAIN QUERY PLAN prefix were omitted.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_stmt_readonly(p_stmt: *mut Sqlite3Stmt) -> i32 {
    return if !(p_stmt).is_null() {
            (unsafe { (*(p_stmt as *mut Vdbe)).read_only() }) as i32
        } else { 1 };
}

///* CAPI3REF: Query The EXPLAIN Setting For A Prepared Statement
///* METHOD: sqlite3_stmt
///*
///* ^The sqlite3_stmt_isexplain(S) interface returns 1 if the
///* prepared statement S is an EXPLAIN statement, or 2 if the
///* statement S is an EXPLAIN QUERY PLAN.
///* ^The sqlite3_stmt_isexplain(S) interface returns 0 if S is
///* an ordinary statement or a NULL pointer.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_stmt_isexplain(p_stmt: *mut Sqlite3Stmt) -> i32 {
    return if !(p_stmt).is_null() {
            (unsafe { (*(p_stmt as *mut Vdbe)).explain() }) as i32
        } else { 0 };
}

///* CAPI3REF: Change The EXPLAIN Setting For A Prepared Statement
///* METHOD: sqlite3_stmt
///*
///* The sqlite3_stmt_explain(S,E) interface changes the EXPLAIN
///* setting for [prepared statement] S.  If E is zero, then S becomes
///* a normal prepared statement.  If E is 1, then S behaves as if
///* its SQL text began with "[EXPLAIN]".  If E is 2, then S behaves as if
///* its SQL text began with "[EXPLAIN QUERY PLAN]".
///*
///* Calling sqlite3_stmt_explain(S,E) might cause S to be reprepared.
///* SQLite tries to avoid a reprepare, but a reprepare might be necessary
///* on the first transition into EXPLAIN or EXPLAIN QUERY PLAN mode.
///*
///* Because of the potential need to reprepare, a call to
///* sqlite3_stmt_explain(S,E) will fail with SQLITE_ERROR if S cannot be
///* reprepared because it was created using [sqlite3_prepare()] instead of
///* the newer [sqlite3_prepare_v2()] or [sqlite3_prepare_v3()] interfaces and
///* hence has no saved SQL text with which to reprepare.
///*
///* Changing the explain setting for a prepared statement does not change
///* the original SQL text for the statement.  Hence, if the SQL text originally
///* began with EXPLAIN or EXPLAIN QUERY PLAN, but sqlite3_stmt_explain(S,0)
///* is called to convert the statement into an ordinary statement, the EXPLAIN
///* or EXPLAIN QUERY PLAN keywords will still appear in the sqlite3_sql(S)
///* output, even though the statement now acts like a normal SQL statement.
///*
///* This routine returns SQLITE_OK if the explain mode is successfully
///* changed, or an error code if the explain mode could not be changed.
///* The explain mode cannot be changed while a statement is active.
///* Hence, it is good practice to call [sqlite3_reset(S)]
///* immediately prior to calling sqlite3_stmt_explain(S,E).
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_stmt_explain(p_stmt: *mut Sqlite3Stmt, e_mode: i32)
    -> i32 {
    let v: *mut Vdbe = p_stmt as *mut Vdbe;
    let mut rc: i32 = 0;
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*v).db }).mutex }) };
    if unsafe { (*v).explain() } as i32 == e_mode {
        rc = 0;
    } else if e_mode < 0 || e_mode > 2 {
        rc = 1;
    } else if unsafe { (*v).prep_flags } as i32 & 128 == 0 {
        rc = 1;
    } else if unsafe { (*v).e_vdbe_state } as i32 != 1 {
        rc = 5;
    } else if unsafe { (*v).n_mem } >= 10 &&
            (e_mode != 2 || unsafe { (*v).have_eqp_ops() } != 0) {

        /// No reprepare necessary
        unsafe { (*v).set_explain(e_mode as Bft as u32) };
        rc = 0;
    } else {
        unsafe { (*v).set_explain(e_mode as Bft as u32) };
        rc = unsafe { sqlite3_reprepare(v) };
        unsafe { (*v).set_have_eqp_ops((e_mode == 2) as Bft as u32) };
    }
    if unsafe { (*v).explain() } != 0 {
        unsafe {
            (*v).n_res_column =
                (12 - 4 * unsafe { (*v).explain() } as i32) as u16
        };
    } else { unsafe { (*v).n_res_column = unsafe { (*v).n_res_alloc } }; }
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*v).db }).mutex }) };
    return rc;
}

///* CAPI3REF: Determine If A Prepared Statement Has Been Reset
///* METHOD: sqlite3_stmt
///*
///* ^The sqlite3_stmt_busy(S) interface returns true (non-zero) if the
///* [prepared statement] S has been stepped at least once using
///* [sqlite3_step(S)] but has neither run to completion (returned
///* [SQLITE_DONE] from [sqlite3_step(S)]) nor
///* been reset using [sqlite3_reset(S)].  ^The sqlite3_stmt_busy(S)
///* interface returns false if S is a NULL pointer.  If S is not a
///* NULL pointer and is not a pointer to a valid [prepared statement]
///* object, then the behavior is undefined and probably undesirable.
///*
///* This interface can be used in combination [sqlite3_next_stmt()]
///* to locate all prepared statements associated with a database
///* connection that are in need of being reset.  This can be used,
///* for example, in diagnostic routines to search for prepared
///* statements that are holding a transaction open.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_stmt_busy(p_stmt: *mut Sqlite3Stmt) -> i32 {
    let v: *const Vdbe = p_stmt as *mut Vdbe as *const Vdbe;
    return (v != core::ptr::null_mut() &&
                unsafe { (*v).e_vdbe_state } as i32 == 2) as i32;
}

///* Check on a Vdbe to make sure it has not been finalized.  Log
///* an error and return true if it has been finalized (or is otherwise
///* invalid).  Return false if it is ok.
extern "C" fn vdbe_safety(p: &Vdbe) -> i32 {
    if (*p).db == core::ptr::null_mut() {
        unsafe {
            sqlite3_log(21,
                c"API called with finalized prepared statement".as_ptr() as
                        *mut i8 as *const i8)
        };
        return 1;
    } else { return 0; }
}

extern "C" fn vdbe_safety_not_null(p: *mut Vdbe) -> i32 {
    if p == core::ptr::null_mut() {
        unsafe {
            sqlite3_log(21,
                c"API called with NULL prepared statement".as_ptr() as *mut i8
                    as *const i8)
        };
        return 1;
    } else { return vdbe_safety(unsafe { &*p }); }
}

///**************************** sqlite3_bind_  ***************************
///*
///* Routines used to attach values to wildcards in a compiled SQL statement.
////
////*
///* Unbind the value bound to variable i in virtual machine p. This is the
///* the same as binding a NULL value to the column. If the "i" parameter is
///* out of range, then SQLITE_RANGE is returned. Otherwise SQLITE_OK.
///*
///* A successful evaluation of this routine acquires the mutex on p.
///* the mutex is released if any kind of error occurs.
///*
///* The error code stored in database p->db is overwritten with the return
///* value in any case.
///*
///* (tag-20240917-01) If  vdbeUnbind(p,(u32)(i-1))  returns SQLITE_OK,
///* that means all of the the following will be true:
///*
///*     p!=0
///*     p->pVar!=0
///*     i>0
///*     i<=p->nVar
///*
///* An assert() is normally added after vdbeUnbind() to help static analyzers
///* realize this.
#[allow(unused_doc_comments)]
extern "C" fn vdbe_unbind(p: *mut Vdbe, i: u32) -> i32 {
    let mut p_var: *mut Mem = core::ptr::null_mut();
    if vdbe_safety_not_null(p) != 0 {
        return unsafe { sqlite3_misuse_error(1728) };
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex }) };
    if unsafe { (*p).e_vdbe_state } as i32 != 1 {
        unsafe {
            sqlite3_error(unsafe { (*p).db },
                unsafe { sqlite3_misuse_error(1732) })
        };
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
        unsafe {
            sqlite3_log(21,
                c"bind on a busy prepared statement: [%s]".as_ptr() as *mut i8
                    as *const i8, unsafe { (*p).z_sql })
        };
        return unsafe { sqlite3_misuse_error(1736) };
    }
    if i >= unsafe { (*p).n_var } as u32 {
        unsafe { sqlite3_error(unsafe { (*p).db }, 25) };
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
        return 25;
    }
    p_var = unsafe { unsafe { (*p).a_var.add(i as usize) } };
    unsafe { sqlite3_vdbe_mem_release(p_var) };
    unsafe { (*p_var).flags = 1 as u16 };
    unsafe { (*unsafe { (*p).db }).err_code = 0 };

    /// If the bit corresponding to this variable in Vdbe.expmask is set, then
    ///* binding a new value to this variable invalidates the current query plan.
    ///*
    ///* IMPLEMENTATION-OF: R-57496-20354 If the specific value bound to a host
    ///* parameter in the WHERE clause might influence the choice of query plan
    ///* for a statement, then the statement will be automatically recompiled,
    ///* as if there had been a schema change, on the first sqlite3_step() call
    ///* following any change to the bindings of that parameter.
    { let _ = 0; };
    if unsafe { (*p).expmask } != 0 as u32 &&
            unsafe { (*p).expmask } &
                    if i >= 31 as u32 { 2147483648u32 } else { (1 as u32) << i }
                != 0 as u32 {
        unsafe { (*p).set_expired(1 as Bft as u32) };
    }
    return 0;
}

///* Bind a text or BLOB value.
#[allow(unused_doc_comments)]
extern "C" fn bind_text(p_stmt_1: *mut Sqlite3Stmt, i: i32,
    z_data_1: *const (), n_data_1: i64,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>, encoding: u8)
    -> i32 {
    let p: *mut Vdbe = p_stmt_1 as *mut Vdbe;
    let mut p_var: *mut Mem = core::ptr::null_mut();
    let mut rc: i32 = 0;
    rc = vdbe_unbind(p, (i - 1) as u32);
    if rc == 0 {
        { let _ = 0; };
        if z_data_1 != core::ptr::null() {
            p_var = unsafe { unsafe { (*p).a_var.offset((i - 1) as isize) } };
            if encoding as i32 == 1 {
                rc =
                    unsafe {
                        sqlite3_vdbe_mem_set_text(p_var, z_data_1 as *const i8,
                            n_data_1, x_del_1)
                    };
            } else if encoding as i32 == 16 {

                /// It is usually consider improper to assert() on an input.
                ///* However, the following assert() is checking for inputs
                ///* that are documented to result in undefined behavior.
                { let _ = 0; };
                rc =
                    unsafe {
                        sqlite3_vdbe_mem_set_text(p_var, z_data_1 as *const i8,
                            n_data_1, x_del_1)
                    };
                unsafe { (*p_var).flags |= 512 as u16 };
            } else {
                rc =
                    unsafe {
                        sqlite3_vdbe_mem_set_str(p_var, z_data_1 as *const i8,
                            n_data_1, encoding, x_del_1)
                    };
                if encoding as i32 == 0 {
                    unsafe {
                        (*p_var).enc = unsafe { (*unsafe { (*p).db }).enc }
                    };
                }
            }
            if rc == 0 && encoding as i32 != 0 {
                rc =
                    unsafe {
                        sqlite3_vdbe_change_encoding(p_var,
                            unsafe { (*unsafe { (*p).db }).enc } as i32)
                    };
            }
            if rc != 0 {
                unsafe { sqlite3_error(unsafe { (*p).db }, rc) };
                rc = unsafe { sqlite3_api_exit(unsafe { (*p).db }, rc) };
            }
        }
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    } else if x_del_1.is_some() &&
            x_del_1 !=
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }) {
        unsafe { x_del_1.unwrap()(z_data_1 as *mut ()) };
    }
    return rc;
}

///* CAPI3REF: Binding Values To Prepared Statements
///* KEYWORDS: {host parameter} {host parameters} {host parameter name}
///* KEYWORDS: {SQL parameter} {SQL parameters} {parameter binding}
///* METHOD: sqlite3_stmt
///*
///* ^(In the SQL statement text input to [sqlite3_prepare_v2()] and its variants,
///* literals may be replaced by a [parameter] that matches one of the following
///* templates:
///*
///* <ul>
///* <li>  ?
///* <li>  ?NNN
///* <li>  :VVV
///* <li>  @VVV
///* <li>  $VVV
///* </ul>
///*
///* In the templates above, NNN represents an integer literal,
///* and VVV represents an alphanumeric identifier.)^  ^The values of these
///* parameters (also called "host parameter names" or "SQL parameters")
///* can be set using the sqlite3_bind_*() routines defined here.
///*
///* ^The first argument to the sqlite3_bind_*() routines is always
///* a pointer to the [sqlite3_stmt] object returned from
///* [sqlite3_prepare_v2()] or its variants.
///*
///* ^The second argument is the index of the SQL parameter to be set.
///* ^The leftmost SQL parameter has an index of 1.  ^When the same named
///* SQL parameter is used more than once, second and subsequent
///* occurrences have the same index as the first occurrence.
///* ^The index for named parameters can be looked up using the
///* [sqlite3_bind_parameter_index()] API if desired.  ^The index
///* for "?NNN" parameters is the value of NNN.
///* ^The NNN value must be between 1 and the [sqlite3_limit()]
///* parameter [SQLITE_LIMIT_VARIABLE_NUMBER] (default value: 32766).
///*
///* ^The third argument is the value to bind to the parameter.
///* ^If the third parameter to sqlite3_bind_text() or sqlite3_bind_text16()
///* or sqlite3_bind_blob() is a NULL pointer then the fourth parameter
///* is ignored and the end result is the same as sqlite3_bind_null().
///* ^If the third parameter to sqlite3_bind_text() is not NULL, then
///* it should be a pointer to well-formed UTF8 text.
///* ^If the third parameter to sqlite3_bind_text16() is not NULL, then
///* it should be a pointer to well-formed UTF16 text.
///* ^If the third parameter to sqlite3_bind_text64() is not NULL, then
///* it should be a pointer to a well-formed unicode string that is
///* either UTF8 if the sixth parameter is SQLITE_UTF8 or SQLITE_UTF8_ZT,
///* or UTF16 otherwise.
///*
///* [[byte-order determination rules]] ^The byte-order of
///* UTF16 input text is determined by the byte-order mark (BOM, U+FEFF)
///* found in the first character, which is removed, or in the absence of a BOM
///* the byte order is the native byte order of the host
///* machine for sqlite3_bind_text16() or the byte order specified in
///* the 6th parameter for sqlite3_bind_text64().)^
///* ^If UTF16 input text contains invalid unicode
///* characters, then SQLite might change those invalid characters
///* into the unicode replacement character: U+FFFD.
///*
///* ^(In those routines that have a fourth argument, its value is the
///* number of bytes in the parameter.  To be clear: the value is the
///* number of <u>bytes</u> in the value, not the number of characters.)^
///* ^If the fourth parameter to sqlite3_bind_text() or sqlite3_bind_text16()
///* is negative, then the length of the string is
///* the number of bytes up to the first zero terminator.
///* If the fourth parameter to sqlite3_bind_blob() is negative, then
///* the behavior is undefined.
///* If a non-negative fourth parameter is provided to sqlite3_bind_text()
///* or sqlite3_bind_text16() or sqlite3_bind_text64() then
///* that parameter must be the byte offset
///* where the NUL terminator would occur assuming the string were NUL
///* terminated.  If any NUL characters occur at byte offsets less than
///* the value of the fourth parameter then the resulting string value will
///* contain embedded NULs.  The result of expressions involving strings
///* with embedded NULs is undefined.
///*
///* ^The fifth argument to the BLOB and string binding interfaces controls
///* or indicates the lifetime of the object referenced by the third parameter.
///* These three options exist:
///* ^ (1) A destructor to dispose of the BLOB or string after SQLite has finished
///* with it may be passed. ^It is called to dispose of the BLOB or string even
///* if the call to the bind API fails, except the destructor is not called if
///* the third parameter is a NULL pointer or the fourth parameter is negative.
///* ^ (2) The special constant, [SQLITE_STATIC], may be passed to indicate that
///* the application remains responsible for disposing of the object. ^In this
///* case, the object and the provided pointer to it must remain valid until
///* either the prepared statement is finalized or the same SQL parameter is
///* bound to something else, whichever occurs sooner.
///* ^ (3) The constant, [SQLITE_TRANSIENT], may be passed to indicate that the
///* object is to be copied prior to the return from sqlite3_bind_*(). ^The
///* object and pointer to it must remain valid until then. ^SQLite will then
///* manage the lifetime of its private copy.
///*
///* ^The sixth argument (the E argument)
///* to sqlite3_bind_text64(S,K,Z,N,D,E) must be one of
///* [SQLITE_UTF8], [SQLITE_UTF8_ZT], [SQLITE_UTF16], [SQLITE_UTF16BE],
///* or [SQLITE_UTF16LE] to specify the encoding of the text in the
///* third parameter, Z.  The special value [SQLITE_UTF8_ZT] means that the
///* string argument is both UTF-8 encoded and is zero-terminated.  In other
///* words, SQLITE_UTF8_ZT means that the Z array is allocated to hold at
///* least N+1 bytes and that the Z&#91;N&#93; byte is zero.  If
///* the E argument to sqlite3_bind_text64(S,K,Z,N,D,E) is not one of the
///* allowed values shown above, or if the text encoding is different
///* from the encoding specified by the sixth parameter, then the behavior
///* is undefined.
///*
///* ^The sqlite3_bind_zeroblob() routine binds a BLOB of length N that
///* is filled with zeroes.  ^A zeroblob uses a fixed amount of memory
///* (just an integer to hold its size) while it is being processed.
///* Zeroblobs are intended to serve as placeholders for BLOBs whose
///* content is later written using
///* [sqlite3_blob_open | incremental BLOB I/O] routines.
///* ^A negative value for the zeroblob results in a zero-length BLOB.
///*
///* ^The sqlite3_bind_pointer(S,I,P,T,D) routine causes the I-th parameter in
///* [prepared statement] S to have an SQL value of NULL, but to also be
///* associated with the pointer P of type T.  ^D is either a NULL pointer or
///* a pointer to a destructor function for P. ^SQLite will invoke the
///* destructor D with a single argument of P when it is finished using
///* P, even if the call to sqlite3_bind_pointer() fails.  Due to a
///* historical design quirk, results are undefined if D is
///* SQLITE_TRANSIENT. The T parameter should be a static string,
///* preferably a string literal. The sqlite3_bind_pointer() routine is
///* part of the [pointer passing interface] added for SQLite 3.20.0.
///*
///* ^If any of the sqlite3_bind_*() routines are called with a NULL pointer
///* for the [prepared statement] or with a prepared statement for which
///* [sqlite3_step()] has been called more recently than [sqlite3_reset()],
///* then the call will return [SQLITE_MISUSE].  If any sqlite3_bind_()
///* routine is passed a [prepared statement] that has been finalized, the
///* result is undefined and probably harmful.
///*
///* ^Bindings are not cleared by the [sqlite3_reset()] routine.
///* ^Unbound parameters are interpreted as NULL.
///*
///* ^The sqlite3_bind_* routines return [SQLITE_OK] on success or an
///* [error code] if anything goes wrong.
///* ^[SQLITE_TOOBIG] might be returned if the size of a string or BLOB
///* exceeds limits imposed by [sqlite3_limit]([SQLITE_LIMIT_LENGTH]) or
///* [SQLITE_MAX_LENGTH].
///* ^[SQLITE_RANGE] is returned if the parameter
///* index is out of range.  ^[SQLITE_NOMEM] is returned if malloc() fails.
///*
///* See also: [sqlite3_bind_parameter_count()],
///* [sqlite3_bind_parameter_name()], and [sqlite3_bind_parameter_index()].
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_blob(p_stmt: *mut Sqlite3Stmt, i: i32,
    z_data: *const (), n_data: i32,
    x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    return bind_text(p_stmt, i, z_data, n_data as i64, x_del, 0 as u8);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_blob64(p_stmt: *mut Sqlite3Stmt, i: i32,
    z_data: *const (), n_data: Sqlite3Uint64,
    x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    { let _ = 0; };
    return bind_text(p_stmt, i, z_data, n_data as i64, x_del, 0 as u8);
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_bind_double(p_stmt: *mut Sqlite3Stmt, i: i32,
    r_value: f64) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut Vdbe = p_stmt as *mut Vdbe;
    rc = vdbe_unbind(p, (i - 1) as u32);
    if rc == 0 {
        { let _ = 0; };

        /// tag-20240917-01
        unsafe {
            sqlite3_vdbe_mem_set_double(unsafe {
                    &mut *unsafe { (*p).a_var.offset((i - 1) as isize) }
                }, r_value)
        };
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    }
    return rc;
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_bind_int64(p_stmt: *mut Sqlite3Stmt, i: i32,
    i_value: Sqlite3Int64) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut Vdbe = p_stmt as *mut Vdbe;
    rc = vdbe_unbind(p, (i - 1) as u32);
    if rc == 0 {
        { let _ = 0; };

        /// tag-20240917-01
        unsafe {
            sqlite3_vdbe_mem_set_int64(unsafe {
                    &mut *unsafe { (*p).a_var.offset((i - 1) as isize) }
                }, i_value)
        };
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_int(p: *mut Sqlite3Stmt, i: i32, i_value: i32)
    -> i32 {
    return sqlite3_bind_int64(p, i, i_value as i64);
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_bind_null(p_stmt: *mut Sqlite3Stmt, i: i32) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut Vdbe = p_stmt as *mut Vdbe;
    rc = vdbe_unbind(p, (i - 1) as u32);
    if rc == 0 {
        { let _ = 0; };

        /// tag-20240917-01
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_text(p_stmt: *mut Sqlite3Stmt, i: i32,
    z_data: *const i8, n_data: i32,
    x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    return bind_text(p_stmt, i, z_data as *const (), n_data as i64, x_del,
            1 as u8);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_text16(p_stmt: *mut Sqlite3Stmt, i: i32,
    z_data: *const (), n: i32,
    x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    return bind_text(p_stmt, i, z_data, (n as u64 & !(1 as u64)) as i64,
            x_del, 2 as u8);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_text64(p_stmt: *mut Sqlite3Stmt, i: i32,
    z_data: *const i8, mut n_data: Sqlite3Uint64,
    x_del: Option<unsafe extern "C" fn(*mut ()) -> ()>, mut enc: u8) -> i32 {
    { let _ = 0; };
    if enc as i32 != 1 && enc as i32 != 16 {
        if enc as i32 == 4 { enc = 2 as u8; }
        n_data &= !(1 as u64);
    }
    return bind_text(p_stmt, i, z_data as *const (), n_data as i64, x_del,
            enc);
}

/// EVIDENCE-OF: R-12793-43283 Every value in SQLite has one of five
///* fundamental datatypes: 64-bit signed integer 64-bit IEEE floating
///* point number string BLOB NULL
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_value_type(p_val_1: &Sqlite3Value) -> i32 {

    /// 0x00 (not possible)
    /// 0x01 NULL
    /// 0x02 TEXT
    /// 0x03 (not possible)
    /// 0x04 INTEGER
    /// 0x05 (not possible)
    /// 0x06 INTEGER + TEXT
    /// 0x07 (not possible)
    /// 0x08 FLOAT
    /// 0x09 (not possible)
    /// 0x0a FLOAT + TEXT
    /// 0x0b (not possible)
    /// 0x0c (not possible)
    /// 0x0d (not possible)
    /// 0x0e (not possible)
    /// 0x0f (not possible)
    /// 0x10 BLOB
    /// 0x11 (not possible)
    /// 0x12 (not possible)
    /// 0x13 (not possible)
    /// 0x14 INTEGER + BLOB
    /// 0x15 (not possible)
    /// 0x16 (not possible)
    /// 0x17 (not possible)
    /// 0x18 FLOAT + BLOB
    /// 0x19 (not possible)
    /// 0x1a (not possible)
    /// 0x1b (not possible)
    /// 0x1c (not possible)
    /// 0x1d (not possible)
    /// 0x1e (not possible)
    /// 0x1f (not possible)
    /// 0x20 INTREAL
    /// 0x21 (not possible)
    /// 0x22 INTREAL + TEXT
    /// 0x23 (not possible)
    /// 0x24 (not possible)
    /// 0x25 (not possible)
    /// 0x26 (not possible)
    /// 0x27 (not possible)
    /// 0x28 (not possible)
    /// 0x29 (not possible)
    /// 0x2a (not possible)
    /// 0x2b (not possible)
    /// 0x2c (not possible)
    /// 0x2d (not possible)
    /// 0x2e (not possible)
    /// 0x2f (not possible)
    /// 0x30 (not possible)
    /// 0x31 (not possible)
    /// 0x32 (not possible)
    /// 0x33 (not possible)
    /// 0x34 (not possible)
    /// 0x35 (not possible)
    /// 0x36 (not possible)
    /// 0x37 (not possible)
    /// 0x38 (not possible)
    /// 0x39 (not possible)
    /// 0x3a (not possible)
    /// 0x3b (not possible)
    /// 0x3c (not possible)
    /// 0x3d (not possible)
    /// 0x3e (not possible)
    /// 0x3f (not possible)
    return a_type[((*p_val_1).flags as i32 & 63) as usize] as i32;
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_bind_zeroblob(p_stmt: *mut Sqlite3Stmt, i: i32,
    n: i32) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut Vdbe = p_stmt as *mut Vdbe;
    rc = vdbe_unbind(p, (i - 1) as u32);
    if rc == 0 {
        { let _ = 0; };

        /// tag-20240917-01
        unsafe {
            sqlite3_vdbe_mem_set_zero_blob(unsafe {
                    &mut *unsafe { (*p).a_var.offset((i - 1) as isize) }
                }, n)
        };
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_value(p_stmt_1: *mut Sqlite3Stmt, i: i32,
    p_value_1: *const Sqlite3Value) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        '__s0:
            {
            match sqlite3_value_type(unsafe {
                        &*(p_value_1 as *mut Sqlite3Value)
                    }) {
                1 => {
                    {
                        rc =
                            sqlite3_bind_int64(p_stmt_1, i,
                                unsafe { (*p_value_1).u.i });
                        break '__s0;
                    }
                    {
                        { let _ = 0; };
                        rc =
                            sqlite3_bind_double(p_stmt_1, i,
                                if unsafe { (*p_value_1).flags } as i32 & 8 != 0 {
                                    unsafe { (*p_value_1).u.r }
                                } else { (unsafe { (*p_value_1).u.i }) as f64 });
                        break '__s0;
                    }
                    {
                        if unsafe { (*p_value_1).flags } as i32 & 1024 != 0 {
                            rc =
                                sqlite3_bind_zeroblob(p_stmt_1, i,
                                    unsafe { (*p_value_1).u.n_zero });
                        } else {
                            rc =
                                sqlite3_bind_blob(p_stmt_1, i,
                                    unsafe { (*p_value_1).z } as *const (),
                                    unsafe { (*p_value_1).n },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }));
                        }
                        break '__s0;
                    }
                    {
                        rc =
                            bind_text(p_stmt_1, i,
                                unsafe { (*p_value_1).z } as *const (),
                                unsafe { (*p_value_1).n } as i64,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }), unsafe { (*p_value_1).enc });
                        break '__s0;
                    }
                    { rc = sqlite3_bind_null(p_stmt_1, i); break '__s0; }
                }
                2 => {
                    {
                        { let _ = 0; };
                        rc =
                            sqlite3_bind_double(p_stmt_1, i,
                                if unsafe { (*p_value_1).flags } as i32 & 8 != 0 {
                                    unsafe { (*p_value_1).u.r }
                                } else { (unsafe { (*p_value_1).u.i }) as f64 });
                        break '__s0;
                    }
                    {
                        if unsafe { (*p_value_1).flags } as i32 & 1024 != 0 {
                            rc =
                                sqlite3_bind_zeroblob(p_stmt_1, i,
                                    unsafe { (*p_value_1).u.n_zero });
                        } else {
                            rc =
                                sqlite3_bind_blob(p_stmt_1, i,
                                    unsafe { (*p_value_1).z } as *const (),
                                    unsafe { (*p_value_1).n },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }));
                        }
                        break '__s0;
                    }
                    {
                        rc =
                            bind_text(p_stmt_1, i,
                                unsafe { (*p_value_1).z } as *const (),
                                unsafe { (*p_value_1).n } as i64,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }), unsafe { (*p_value_1).enc });
                        break '__s0;
                    }
                    { rc = sqlite3_bind_null(p_stmt_1, i); break '__s0; }
                }
                4 => {
                    {
                        if unsafe { (*p_value_1).flags } as i32 & 1024 != 0 {
                            rc =
                                sqlite3_bind_zeroblob(p_stmt_1, i,
                                    unsafe { (*p_value_1).u.n_zero });
                        } else {
                            rc =
                                sqlite3_bind_blob(p_stmt_1, i,
                                    unsafe { (*p_value_1).z } as *const (),
                                    unsafe { (*p_value_1).n },
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }));
                        }
                        break '__s0;
                    }
                    {
                        rc =
                            bind_text(p_stmt_1, i,
                                unsafe { (*p_value_1).z } as *const (),
                                unsafe { (*p_value_1).n } as i64,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }), unsafe { (*p_value_1).enc });
                        break '__s0;
                    }
                    { rc = sqlite3_bind_null(p_stmt_1, i); break '__s0; }
                }
                3 => {
                    {
                        rc =
                            bind_text(p_stmt_1, i,
                                unsafe { (*p_value_1).z } as *const (),
                                unsafe { (*p_value_1).n } as i64,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }), unsafe { (*p_value_1).enc });
                        break '__s0;
                    }
                    { rc = sqlite3_bind_null(p_stmt_1, i); break '__s0; }
                }
                _ => { { rc = sqlite3_bind_null(p_stmt_1, i); break '__s0; } }
            }
        }
        return rc;
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_bind_pointer(p_stmt: *mut Sqlite3Stmt, i: i32,
    p_ptr: *mut (), z_p_ttype: *const i8,
    x_destructor: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> i32 {
    let mut rc: i32 = 0;
    let p: *mut Vdbe = p_stmt as *mut Vdbe;
    rc = vdbe_unbind(p, (i - 1) as u32);
    if rc == 0 {
        { let _ = 0; };

        /// tag-20240917-01
        unsafe {
            sqlite3_vdbe_mem_set_pointer(unsafe {
                    &mut *unsafe { (*p).a_var.offset((i - 1) as isize) }
                }, p_ptr, z_p_ttype, x_destructor)
        };
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    } else if x_destructor.is_some() {
        unsafe { x_destructor.unwrap()(p_ptr) };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_zeroblob64(p_stmt: *mut Sqlite3Stmt, i: i32,
    n: Sqlite3Uint64) -> i32 {
    let mut rc: i32 = 0;
    let p: *const Vdbe = p_stmt as *mut Vdbe as *const Vdbe;
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex }) };
    if n > unsafe { (*unsafe { (*p).db }).a_limit[0 as usize] } as u64 {
        rc = 18;
    } else {
        { let _ = 0; };
        rc = sqlite3_bind_zeroblob(p_stmt, i, n as i32);
    }
    rc = unsafe { sqlite3_api_exit(unsafe { (*p).db }, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    return rc;
}

///* CAPI3REF: Number Of SQL Parameters
///* METHOD: sqlite3_stmt
///*
///* ^This routine can be used to find the number of [SQL parameters]
///* in a [prepared statement].  SQL parameters are tokens of the
///* form "?", "?NNN", ":AAA", "$AAA", or "@AAA" that serve as
///* placeholders for values that are [sqlite3_bind_blob | bound]
///* to the parameters at a later time.
///*
///* ^(This routine actually returns the index of the largest (rightmost)
///* parameter. For all forms except ?NNN, this will correspond to the
///* number of unique parameters.  If parameters of the ?NNN form are used,
///* there may be gaps in the list.)^
///*
///* See also: [sqlite3_bind_blob|sqlite3_bind()],
///* [sqlite3_bind_parameter_name()], and
///* [sqlite3_bind_parameter_index()].
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_parameter_count(p_stmt: *mut Sqlite3Stmt)
    -> i32 {
    let p: *const Vdbe = p_stmt as *mut Vdbe as *const Vdbe;
    return if !(p).is_null() { (unsafe { (*p).n_var }) as i32 } else { 0 };
}

///* CAPI3REF: Name Of A Host Parameter
///* METHOD: sqlite3_stmt
///*
///* ^The sqlite3_bind_parameter_name(P,N) interface returns
///* the name of the N-th [SQL parameter] in the [prepared statement] P.
///* ^(SQL parameters of the form "?NNN" or ":AAA" or "@AAA" or "$AAA"
///* have a name which is the string "?NNN" or ":AAA" or "@AAA" or "$AAA"
///* respectively.
///* In other words, the initial ":" or "$" or "@" or "?"
///* is included as part of the name.)^
///* ^Parameters of the form "?" without a following integer have no name
///* and are referred to as "nameless" or "anonymous parameters".
///*
///* ^The first host parameter has an index of 1, not 0.
///*
///* ^If the value N is out of range or if the N-th parameter is
///* nameless, then NULL is returned.  ^The returned string is
///* always in UTF-8 encoding even if the named parameter was
///* originally specified as UTF-16 in [sqlite3_prepare16()],
///* [sqlite3_prepare16_v2()], or [sqlite3_prepare16_v3()].
///*
///* See also: [sqlite3_bind_blob|sqlite3_bind()],
///* [sqlite3_bind_parameter_count()], and
///* [sqlite3_bind_parameter_index()].
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_parameter_name(p_stmt: *mut Sqlite3Stmt,
    i: i32) -> *const i8 {
    let p: *const Vdbe = p_stmt as *mut Vdbe as *const Vdbe;
    if p == core::ptr::null_mut() { return core::ptr::null(); }
    return unsafe { sqlite3_v_list_num_to_name(unsafe { (*p).p_v_list }, i) };
}

///* Given a wildcard parameter name, return the index of the variable
///* with that name.  If there is no variable with the given name,
///* return 0.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_parameter_index(p: *const Vdbe,
    z_name_1: *const i8, n_name_1: i32) -> i32 {
    if p == core::ptr::null_mut() || z_name_1 == core::ptr::null() {
        return 0;
    }
    return unsafe {
            sqlite3_v_list_name_to_num(unsafe { (*p).p_v_list }, z_name_1,
                n_name_1)
        };
}

///* CAPI3REF: Index Of A Parameter With A Given Name
///* METHOD: sqlite3_stmt
///*
///* ^Return the index of an SQL parameter given its name.  ^The
///* index value returned is suitable for use as the second
///* parameter to [sqlite3_bind_blob|sqlite3_bind()].  ^A zero
///* is returned if no matching parameter is found.  ^The parameter
///* name must be given in UTF-8 even if the original statement
///* was prepared from UTF-16 text using [sqlite3_prepare16_v2()] or
///* [sqlite3_prepare16_v3()].
///*
///* See also: [sqlite3_bind_blob|sqlite3_bind()],
///* [sqlite3_bind_parameter_count()], and
///* [sqlite3_bind_parameter_name()].
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_bind_parameter_index(p_stmt: *mut Sqlite3Stmt,
    z_name: *const i8) -> i32 {
    return sqlite3_vdbe_parameter_index(p_stmt as *mut Vdbe as *const Vdbe,
            z_name, unsafe { sqlite3_strlen30(z_name) });
}

///* CAPI3REF: Reset All Bindings On A Prepared Statement
///* METHOD: sqlite3_stmt
///*
///* ^Contrary to the intuition of many, [sqlite3_reset()] does not reset
///* the [sqlite3_bind_blob | bindings] on a [prepared statement].
///* ^Use this routine to reset all host parameters to NULL.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_clear_bindings(p_stmt: *mut Sqlite3Stmt) -> i32 {
    let mut i: i32 = 0;
    let rc: i32 = 0;
    let p: *mut Vdbe = p_stmt as *mut Vdbe;
    let mut mutex: *mut Sqlite3Mutex = core::ptr::null_mut();
    mutex = unsafe { (*unsafe { (*p).db }).mutex };
    unsafe { sqlite3_mutex_enter(mutex) };
    {
        i = 0;
        '__b1: loop {
            if !(i < unsafe { (*p).n_var } as i32) { break '__b1; }
            '__c1: loop {
                unsafe {
                    sqlite3_vdbe_mem_release(unsafe {
                            &mut *unsafe { (*p).a_var.offset(i as isize) }
                        })
                };
                unsafe {
                    (*unsafe { (*p).a_var.offset(i as isize) }).flags = 1 as u16
                };
                break '__c1;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
    if unsafe { (*p).expmask } != 0 {
        unsafe { (*p).set_expired(1 as Bft as u32) };
    }
    unsafe { sqlite3_mutex_leave(mutex) };
    return rc;
}

///* CAPI3REF: Number Of Columns In A Result Set
///* METHOD: sqlite3_stmt
///*
///* ^Return the number of columns in the result set returned by the
///* [prepared statement]. ^If this routine returns 0, that means the
///* [prepared statement] returns no data (for example an [UPDATE]).
///* ^However, just because this routine returns a positive number does not
///* mean that one or more rows of data will be returned.  ^A SELECT statement
///* will always have a positive sqlite3_column_count() but depending on the
///* WHERE clause constraints and the table content, it might return no rows.
///*
///* See also: [sqlite3_data_count()]
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_count(p_stmt: *mut Sqlite3Stmt) -> i32 {
    let p_vm: *const Vdbe = p_stmt as *mut Vdbe as *const Vdbe;
    if p_vm == core::ptr::null_mut() { return 0; }
    return unsafe { (*p_vm).n_res_column } as i32;
}

static i_explain_col_names16: [u8; 12] =
    [0 as u8, 5 as u8, 12 as u8, 15 as u8, 18 as u8, 21 as u8, 24 as u8,
            27 as u8, 35 as u8, 38 as u8, 45 as u8, 53 as u8];

static az_explain_col_names16data: [u16; 60] =
    ['a' as i32 as u16, 'd' as i32 as u16, 'd' as i32 as u16,
            'r' as i32 as u16, 0 as u16, 'o' as i32 as u16, 'p' as i32 as u16,
            'c' as i32 as u16, 'o' as i32 as u16, 'd' as i32 as u16,
            'e' as i32 as u16, 0 as u16, 'p' as i32 as u16, '1' as i32 as u16,
            0 as u16, 'p' as i32 as u16, '2' as i32 as u16, 0 as u16,
            'p' as i32 as u16, '3' as i32 as u16, 0 as u16, 'p' as i32 as u16,
            '4' as i32 as u16, 0 as u16, 'p' as i32 as u16, '5' as i32 as u16,
            0 as u16, 'c' as i32 as u16, 'o' as i32 as u16, 'm' as i32 as u16,
            'm' as i32 as u16, 'e' as i32 as u16, 'n' as i32 as u16,
            't' as i32 as u16, 0 as u16, 'i' as i32 as u16, 'd' as i32 as u16,
            0 as u16, 'p' as i32 as u16, 'a' as i32 as u16, 'r' as i32 as u16,
            'e' as i32 as u16, 'n' as i32 as u16, 't' as i32 as u16, 0 as u16,
            'n' as i32 as u16, 'o' as i32 as u16, 't' as i32 as u16,
            'u' as i32 as u16, 's' as i32 as u16, 'e' as i32 as u16,
            'd' as i32 as u16, 0 as u16, 'd' as i32 as u16, 'e' as i32 as u16,
            't' as i32 as u16, 'a' as i32 as u16, 'i' as i32 as u16,
            'l' as i32 as u16, 0 as u16];

///* Column names appropriate for EXPLAIN or EXPLAIN QUERY PLAN.
static mut az_explain_col_names8: [*const i8; 12] =
    [c"addr".as_ptr() as *const i8, c"opcode".as_ptr() as *const i8,
            c"p1".as_ptr() as *const i8, c"p2".as_ptr() as *const i8,
            c"p3".as_ptr() as *const i8, c"p4".as_ptr() as *const i8,
            c"p5".as_ptr() as *const i8, c"comment".as_ptr() as *const i8,
            c"id".as_ptr() as *const i8, c"parent".as_ptr() as *const i8,
            c"notused".as_ptr() as *const i8,
            c"detail".as_ptr() as *const i8];

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_text16(p_val_1: *mut Sqlite3Value)
    -> *const () {
    return unsafe { sqlite3ValueText(p_val_1, 2 as u8) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_text(p_val_1: *mut Sqlite3Value)
    -> *const u8 {
    return unsafe { sqlite3ValueText(p_val_1, 1 as u8) } as *const u8;
}

///* Convert the N-th element of pStmt->pColName[] into a string using
///* xFunc() then return that string.  If N is out of range, return 0.
///*
///* There are up to 5 names for each column.  useType determines which
///* name is returned.  Here are the names:
///*
///*    0      The column name as it should be displayed for output
///*    1      The datatype name for the column
///*    2      The name of the database that the column derives from
///*    3      The name of the table that the column derives from
///*    4      The name of the table column that the result column derives from
///*
///* If the result is not a simple column reference (if it is an expression
///* or a constant) then useTypes 2, 3, and 4 return NULL.
#[allow(unused_doc_comments)]
extern "C" fn column_name(p_stmt_1: *mut Sqlite3Stmt, mut n_1: i32,
    use_utf16_1: i32, use_type_1: i32) -> *const () {
    unsafe {
        let mut ret: *const () = core::ptr::null();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        '__b2: loop {
            '__c2: loop {
                let mut p: *const Vdbe = core::ptr::null();
                let mut n: i32 = 0;
                if n_1 < 0 { return core::ptr::null(); }
                ret = core::ptr::null();
                p = p_stmt_1 as *mut Vdbe;
                db = unsafe { (*p).db };
                { let _ = 0; };
                unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                if unsafe { (*p).explain() } != 0 {
                    if use_type_1 > 0 { break '__b2; }
                    n =
                        if unsafe { (*p).explain() } as i32 == 1 { 8 } else { 4 };
                    if n_1 >= n { break '__b2; }
                    if use_utf16_1 != 0 {
                        let i: i32 =
                            i_explain_col_names16[(n_1 +
                                                8 * unsafe { (*p).explain() } as i32 - 8) as usize] as i32;
                        ret =
                            &raw const az_explain_col_names16data[i as usize] as *mut ()
                                as *const ();
                    } else {
                        ret =
                            az_explain_col_names8[(n_1 +
                                                    8 * unsafe { (*p).explain() } as i32 - 8) as usize] as
                                    *mut () as *const ();
                    }
                    break '__b2;
                }
                n = unsafe { (*p).n_res_column } as i32;
                if n_1 < n {
                    let prior_malloc_failed: u8 =
                        unsafe { (*db).malloc_failed };
                    n_1 += use_type_1 * n;
                    if use_utf16_1 != 0 {
                        ret =
                            sqlite3_value_text16(unsafe {
                                        &raw mut *unsafe { (*p).a_col_name.offset(n_1 as isize) }
                                    } as *mut Sqlite3Value);
                    } else {
                        ret =
                            sqlite3_value_text(unsafe {
                                            &raw mut *unsafe { (*p).a_col_name.offset(n_1 as isize) }
                                        } as *mut Sqlite3Value) as *const ();
                    }

                    /// A malloc may have failed inside of the _text() call. If this
                    ///* is the case, clear the mallocFailed flag and return NULL.
                    { let _ = 0; };
                    if unsafe { (*db).malloc_failed } as i32 >
                            prior_malloc_failed as i32 {
                        unsafe { sqlite3_oom_clear(db) };
                        ret = core::ptr::null();
                    }
                }
                break '__c2;
            }
            if !(false) { break '__b2; }
        }

        /// A malloc may have failed inside of the _text() call. If this
        ///* is the case, clear the mallocFailed flag and return NULL.
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return ret;
    }
}

///* CAPI3REF: Column Names In A Result Set
///* METHOD: sqlite3_stmt
///*
///* ^These routines return the name assigned to a particular column
///* in the result set of a [SELECT] statement.  ^The sqlite3_column_name()
///* interface returns a pointer to a zero-terminated UTF-8 string
///* and sqlite3_column_name16() returns a pointer to a zero-terminated
///* UTF-16 string.  ^The first parameter is the [prepared statement]
///* that implements the [SELECT] statement. ^The second parameter is the
///* column number.  ^The leftmost column is number 0.
///*
///* ^The returned string pointer is valid until either the [prepared statement]
///* is destroyed by [sqlite3_finalize()] or until the statement is automatically
///* reprepared by the first call to [sqlite3_step()] for a particular run
///* or until the next call to
///* sqlite3_column_name() or sqlite3_column_name16() on the same column.
///*
///* ^If sqlite3_malloc() fails during the processing of either routine
///* (for example during a conversion from UTF-8 to UTF-16) then a
///* NULL pointer is returned.
///*
///* ^The name of a result column is the value of the "AS" clause for
///* that column, if there is an AS clause.  If there is no AS clause
///* then the name of the column is unspecified and may change from
///* one release of SQLite to the next.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_name(p_stmt: *mut Sqlite3Stmt, n: i32)
    -> *const i8 {
    return column_name(p_stmt, n, 0, 0) as *const i8;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_name16(p_stmt: *mut Sqlite3Stmt, n: i32)
    -> *const () {
    return column_name(p_stmt, n, 1, 0);
}

///* CAPI3REF: Declared Datatype Of A Query Result
///* METHOD: sqlite3_stmt
///*
///* ^(The first parameter is a [prepared statement].
///* If this statement is a [SELECT] statement and the Nth column of the
///* returned result set of that [SELECT] is a table column (not an
///* expression or subquery) then the declared type of the table
///* column is returned.)^  ^If the Nth column of the result set is an
///* expression or subquery, then a NULL pointer is returned.
///* ^The returned string is always UTF-8 encoded.
///*
///* ^(For example, given the database schema:
///*
///* CREATE TABLE t1(c1 VARIANT);
///*
///* and the following statement to be compiled:
///*
///* SELECT c1 + 1, c1 FROM t1;
///*
///* this routine would return the string "VARIANT" for the second result
///* column (i==1), and a NULL pointer for the first result column (i==0).)^
///*
///* ^SQLite uses dynamic run-time typing.  ^So just because a column
///* is declared to contain a particular type does not mean that the
///* data stored in that column is of the declared type.  SQLite is
///* strongly typed, but the typing is dynamic not static.  ^Type
///* is associated with individual values, not with the containers
///* used to hold those values.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_decltype(p_stmt: *mut Sqlite3Stmt, n: i32)
    -> *const i8 {
    return column_name(p_stmt, n, 0, 1) as *const i8;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_decltype16(p_stmt: *mut Sqlite3Stmt, n: i32)
    -> *const () {
    return column_name(p_stmt, n, 1, 1);
}

///* Invoke the profile callback.  This routine is only called if we already
///* know that the profile callback is defined and needs to be invoked.
extern "C" fn invoke_profile_callback(db: &Sqlite3, p: *mut Vdbe) -> () {
    unsafe {
        let mut i_now: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut i_elapse: Sqlite3Int64 = 0 as Sqlite3Int64;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        unsafe { sqlite3_os_current_time_int64((*db).p_vfs, &mut i_now) };
        i_elapse =
            (i_now - unsafe { (*p).start_time }) * 1000000 as SqliteInt64;
        if (*db).x_profile.is_some() {
            unsafe {
                (*db).x_profile.unwrap()((*db).p_profile_arg,
                    unsafe { (*p).z_sql } as *const i8, i_elapse as u64)
            };
        }
        if (*db).m_trace as i32 & 2 != 0 {
            unsafe {
                (*db).trace.x_v2.unwrap()(2, (*db).p_trace_arg, p as *mut (),
                    &raw mut i_elapse as *mut ())
            };
        }
        unsafe { (*p).start_time = 0 as i64 };
    }
}

///* CAPI3REF: Reset A Prepared Statement Object
///* METHOD: sqlite3_stmt
///*
///* The sqlite3_reset() function is called to reset a [prepared statement]
///* object back to its initial state, ready to be re-executed.
///* ^Any SQL statement variables that had values bound to them using
///* the [sqlite3_bind_blob | sqlite3_bind_*() API] retain their values.
///* Use [sqlite3_clear_bindings()] to reset the bindings.
///*
///* ^The [sqlite3_reset(S)] interface resets the [prepared statement] S
///* back to the beginning of its program.
///*
///* ^The return code from [sqlite3_reset(S)] indicates whether or not
///* the previous evaluation of prepared statement S completed successfully.
///* ^If [sqlite3_step(S)] has never before been called on S or if
///* [sqlite3_step(S)] has not been called since the previous call
///* to [sqlite3_reset(S)], then [sqlite3_reset(S)] will return
///* [SQLITE_OK].
///*
///* ^If the most recent call to [sqlite3_step(S)] for the
///* [prepared statement] S indicated an error, then
///* [sqlite3_reset(S)] returns an appropriate [error code].
///* ^The [sqlite3_reset(S)] interface might also return an [error code]
///* if there were no prior errors but the process of resetting
///* the prepared statement caused a new error. ^For example, if an
///* [INSERT] statement with a [RETURNING] clause is only stepped one time,
///* that one call to [sqlite3_step(S)] might return SQLITE_ROW but
///* the overall statement might still fail and the [sqlite3_reset(S)] call
///* might return SQLITE_BUSY if locking constraints prevent the
///* database change from committing.  Therefore, it is important that
///* applications check the return code from [sqlite3_reset(S)] even if
///* no prior call to [sqlite3_step(S)] indicated a problem.
///*
///* ^The [sqlite3_reset(S)] interface does not change the values
///* of any [sqlite3_bind_blob|bindings] on the [prepared statement] S.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_reset(p_stmt: *mut Sqlite3Stmt) -> i32 {
    let mut rc: i32 = 0;
    if p_stmt == core::ptr::null_mut() {
        rc = 0;
    } else {
        let v: *mut Vdbe = p_stmt as *mut Vdbe;
        let db: *mut Sqlite3 = unsafe { (*v).db };
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        if unsafe { (*v).start_time } > 0 as i64 {
            invoke_profile_callback(unsafe { &*db }, v);
        }
        rc = unsafe { sqlite3_vdbe_reset(v) };
        unsafe { sqlite3_vdbe_rewind(v) };
        { let _ = 0; };
        rc = unsafe { sqlite3_api_exit(db, rc) };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    }
    return rc;
}

///* This function is called after a transaction has been committed. It
///* invokes callbacks registered with sqlite3_wal_hook() as required.
extern "C" fn do_wal_callbacks(db: *mut Sqlite3) -> i32 {
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b3: loop {
            if !(i < unsafe { (*db).n_db }) { break '__b3; }
            '__c3: loop {
                let p_bt: *mut Btree =
                    unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                if !(p_bt).is_null() {
                    let mut n_entry: i32 = 0;
                    unsafe { sqlite3_btree_enter(p_bt) };
                    n_entry =
                        unsafe {
                            sqlite3_pager_wal_callback(unsafe {
                                    sqlite3_btree_pager(p_bt)
                                })
                        };
                    unsafe { sqlite3_btree_leave(p_bt) };
                    if n_entry > 0 && unsafe { (*db).x_wal_callback.is_some() }
                            && rc == 0 {
                        rc =
                            unsafe {
                                (unsafe {
                                        (*db).x_wal_callback.unwrap()
                                    })(unsafe { (*db).p_wal_arg }, db,
                                    unsafe {
                                            (*unsafe { (*db).a_db.offset(i as isize) }).z_db_s_name
                                        } as *const i8, n_entry)
                            };
                    }
                }
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return rc;
}

///* Execute the statement pStmt, either until a row of data is ready, the
///* statement is completely executed or an error occurs.
///*
///* This routine implements the bulk of the logic behind the sqlite_step()
///* API.  The only thing omitted is the automatic recompile if a
///* schema change has occurred.  That detail is handled by the
///* outer sqlite3_step() wrapper procedure.
#[allow(unused_doc_comments)]
extern "C" fn sqlite3_step_2(p: *mut Vdbe) -> i32 {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s5:
                {
                match __state {
                    0 => { __state = 4; }
                    2 => {
                        if unsafe { (*p).e_vdbe_state } as i32 == 1 {
                            __state = 10;
                        } else { __state = 11; }
                    }
                    3 => { { let _ = 0; }; __state = 59; }
                    4 => { __state = 5; }
                    5 => { { let _ = 0; }; __state = 6; }
                    6 => { db = unsafe { (*p).db }; __state = 7; }
                    7 => {
                        if unsafe { (*p).e_vdbe_state } as i32 != 2 {
                            __state = 9;
                        } else { __state = 8; }
                    }
                    8 => {
                        if unsafe { (*p).explain() } != 0 {
                            __state = 34;
                        } else { __state = 35; }
                    }
                    9 => { __state = 2; }
                    10 => {
                        if unsafe { (*p).expired() } != 0 {
                            __state = 13;
                        } else { __state = 12; }
                    }
                    11 => {
                        if unsafe { (*p).e_vdbe_state } as i32 == 3 {
                            __state = 30;
                        } else { __state = 8; }
                    }
                    12 => {
                        if unsafe { (*db).n_vdbe_active } == 0 {
                            __state = 19;
                        } else { __state = 18; }
                    }
                    13 => { unsafe { (*p).rc = 17 }; __state = 14; }
                    14 => { rc = 1; __state = 15; }
                    15 => {
                        if unsafe { (*p).prep_flags } as i32 & 128 != 0 {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    16 => { __state = 3; }
                    17 => {
                        rc = unsafe { sqlite3_vdbe_transfer_error(p) };
                        __state = 16;
                    }
                    18 => { { let _ = 0; }; __state = 20; }
                    19 => {
                        unsafe {
                            std::sync::atomic::AtomicI32::from_ptr(unsafe {
                                            &raw mut (*db).u1.is_interrupted
                                        } as
                                        *mut i32).store(0 as i32,
                                std::sync::atomic::Ordering::Relaxed)
                        };
                        __state = 18;
                    }
                    20 => {
                        if unsafe { (*db).m_trace } as i32 & (2 | 128) != 0 &&
                                    (unsafe { (*db).init.busy } == 0) as i32 != 0 &&
                                !(unsafe { (*p).z_sql }).is_null() {
                            __state = 22;
                        } else { __state = 23; }
                    }
                    21 => {
                        {
                            let __p = unsafe { &mut (*db).n_vdbe_active };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 24;
                    }
                    22 => {
                        unsafe {
                            sqlite3_os_current_time_int64(unsafe { (*db).p_vfs },
                                unsafe { &mut (*p).start_time })
                        };
                        __state = 21;
                    }
                    23 => { { let _ = 0; }; __state = 21; }
                    24 => {
                        if unsafe { (*p).read_only() } as i32 == 0 {
                            __state = 26;
                        } else { __state = 25; }
                    }
                    25 => {
                        if unsafe { (*p).b_is_reader() } != 0 {
                            __state = 28;
                        } else { __state = 27; }
                    }
                    26 => {
                        {
                            let __p = unsafe { &mut (*db).n_vdbe_write };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 25;
                    }
                    27 => { unsafe { (*p).pc = 0 }; __state = 29; }
                    28 => {
                        {
                            let __p = unsafe { &mut (*db).n_vdbe_read };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 27;
                    }
                    29 => {
                        unsafe { (*p).e_vdbe_state = 2 as u8 };
                        __state = 8;
                    }
                    30 => {
                        sqlite3_reset(p as *mut Sqlite3Stmt);
                        __state = 31;
                    }
                    31 => { { let _ = 0; }; __state = 32; }
                    32 => { __state = 2; }
                    33 => {
                        if rc == 100 { __state = 39; } else { __state = 40; }
                    }
                    34 => {
                        rc = unsafe { sqlite3_vdbe_list(p) };
                        __state = 33;
                    }
                    35 => {
                        {
                            let __p = unsafe { &mut (*db).n_vdbe_exec };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 36;
                    }
                    36 => {
                        rc = unsafe { sqlite3_vdbe_exec(p) };
                        __state = 37;
                    }
                    37 => {
                        {
                            let __p = unsafe { &mut (*db).n_vdbe_exec };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 33;
                    }
                    38 => { unsafe { (*db).err_code = rc }; __state = 54; }
                    39 => { { let _ = 0; }; __state = 41; }
                    40 => {
                        if unsafe { (*p).start_time } > 0 as i64 {
                            __state = 45;
                        } else { __state = 44; }
                    }
                    41 => { { let _ = 0; }; __state = 42; }
                    42 => { unsafe { (*db).err_code = 100 }; __state = 43; }
                    43 => { return 100; }
                    44 => { __state = 46; }
                    45 => {
                        invoke_profile_callback(unsafe { &*db }, p);
                        __state = 44;
                    }
                    46 => {
                        unsafe { (*p).p_result_row = core::ptr::null_mut() };
                        __state = 47;
                    }
                    47 => {
                        if rc == 101 && unsafe { (*db).auto_commit } != 0 {
                            __state = 48;
                        } else { __state = 49; }
                    }
                    48 => { { let _ = 0; }; __state = 50; }
                    49 => {
                        if rc != 101 && unsafe { (*p).prep_flags } as i32 & 128 != 0
                            {
                            __state = 53;
                        } else { __state = 38; }
                    }
                    50 => {
                        unsafe { (*p).rc = do_wal_callbacks(db) };
                        __state = 51;
                    }
                    51 => {
                        if unsafe { (*p).rc } != 0 {
                            __state = 52;
                        } else { __state = 38; }
                    }
                    52 => { rc = 1; __state = 38; }
                    53 => {
                        rc = unsafe { sqlite3_vdbe_transfer_error(p) };
                        __state = 38;
                    }
                    54 => {
                        if 7 ==
                                unsafe {
                                    sqlite3_api_exit(unsafe { (*p).db }, unsafe { (*p).rc })
                                } {
                            __state = 56;
                        } else { __state = 55; }
                    }
                    55 => { __state = 3; }
                    56 => { unsafe { (*p).rc = 7 }; __state = 57; }
                    57 => {
                        if unsafe { (*p).prep_flags } as i32 & 128 != 0 {
                            __state = 58;
                        } else { __state = 55; }
                    }
                    58 => { rc = unsafe { (*p).rc }; __state = 55; }
                    59 => { return rc & unsafe { (*db).err_mask }; }
                    _ => {}
                }
            }
        }

        /// If this statement was prepared using saved SQL and an
        ///* error has occurred, then return the error code in p->rc to the
        ///* caller. Set the error code in the database handle to the same
        ///* value.
        /// If there are no other statements currently running, then
        ///* reset the interrupt flag.  This prevents a call to sqlite3_interrupt
        ///* from interrupting a statement that has not yet started.
        /// We used to require that sqlite3_reset() be called before retrying
        ///* sqlite3_step() after any error or after SQLITE_DONE.  But beginning
        ///* with version 3.7.0, we changed this so that sqlite3_reset() would
        ///* be called automatically instead of throwing the SQLITE_MISUSE error.
        ///* This "automatic-reset" change is not technically an incompatibility,
        ///* since any application that receives an SQLITE_MISUSE is broken by
        ///* definition.
        ///*
        ///* Nevertheless, some published applications that were originally written
        ///* for version 3.6.23 or earlier do in fact depend on SQLITE_MISUSE
        ///* returns, and those were broken by the automatic-reset change.  As a
        ///* a work-around, the SQLITE_OMIT_AUTORESET compile-time restores the
        ///* legacy behavior of returning SQLITE_MISUSE for cases where the
        ///* previous sqlite3_step() returned something other than a SQLITE_LOCKED
        ///* or SQLITE_BUSY error.
        /// SQLITE_OMIT_EXPLAIN
        /// If the statement completed successfully, invoke the profile callback
        /// If this statement was prepared using saved SQL and an
        ///* error has occurred, then return the error code in p->rc to the
        ///* caller. Set the error code in the database handle to the same value.
        /// There are only a limited number of result codes allowed from the
        ///* statements prepared using the legacy sqlite3_prepare() interface
        unreachable!();
    }
}

///* CAPI3REF: Evaluate An SQL Statement
///* METHOD: sqlite3_stmt
///*
///* After a [prepared statement] has been prepared using any of
///* [sqlite3_prepare_v2()], [sqlite3_prepare_v3()], [sqlite3_prepare16_v2()],
///* or [sqlite3_prepare16_v3()] or one of the legacy
///* interfaces [sqlite3_prepare()] or [sqlite3_prepare16()], this function
///* must be called one or more times to evaluate the statement.
///*
///* The details of the behavior of the sqlite3_step() interface depend
///* on whether the statement was prepared using the newer "vX" interfaces
///* [sqlite3_prepare_v3()], [sqlite3_prepare_v2()], [sqlite3_prepare16_v3()],
///* [sqlite3_prepare16_v2()] or the older legacy
///* interfaces [sqlite3_prepare()] and [sqlite3_prepare16()].  The use of the
///* new "vX" interface is recommended for new applications but the legacy
///* interface will continue to be supported.
///*
///* ^In the legacy interface, the return value will be either [SQLITE_BUSY],
///* [SQLITE_DONE], [SQLITE_ROW], [SQLITE_ERROR], or [SQLITE_MISUSE].
///* ^With the "v2" interface, any of the other [result codes] or
///* [extended result codes] might be returned as well.
///*
///* ^[SQLITE_BUSY] means that the database engine was unable to acquire the
///* database locks it needs to do its job.  ^If the statement is a [COMMIT]
///* or occurs outside of an explicit transaction, then you can retry the
///* statement.  If the statement is not a [COMMIT] and occurs within an
///* explicit transaction then you should rollback the transaction before
///* continuing.
///*
///* ^[SQLITE_DONE] means that the statement has finished executing
///* successfully.  sqlite3_step() should not be called again on this virtual
///* machine without first calling [sqlite3_reset()] to reset the virtual
///* machine back to its initial state.
///*
///* ^If the SQL statement being executed returns any data, then [SQLITE_ROW]
///* is returned each time a new row of data is ready for processing by the
///* caller. The values may be accessed using the [column access functions].
///* sqlite3_step() is called again to retrieve the next row of data.
///*
///* ^[SQLITE_ERROR] means that a run-time error (such as a constraint
///* violation) has occurred.  sqlite3_step() should not be called again on
///* the VM. More information may be found by calling [sqlite3_errmsg()].
///* ^With the legacy interface, a more specific error code (for example,
///* [SQLITE_INTERRUPT], [SQLITE_SCHEMA], [SQLITE_CORRUPT], and so forth)
///* can be obtained by calling [sqlite3_reset()] on the
///* [prepared statement].  ^In the "v2" interface,
///* the more specific error code is returned directly by sqlite3_step().
///*
///* [SQLITE_MISUSE] means that this routine was called inappropriately.
///* Perhaps it was called on a [prepared statement] that has
///* already been [sqlite3_finalize | finalized] or on one that had
///* previously returned [SQLITE_ERROR] or [SQLITE_DONE].  Or it could
///* be the case that the same database connection is being used by two or
///* more threads at the same moment in time.
///*
///* For all versions of SQLite up to and including 3.6.23.1, a call to
///* [sqlite3_reset()] was required after sqlite3_step() returned anything
///* other than [SQLITE_ROW] before any subsequent invocation of
///* sqlite3_step().  Failure to reset the prepared statement using
///* [sqlite3_reset()] would result in an [SQLITE_MISUSE] return from
///* sqlite3_step().  But after [version 3.6.23.1] ([dateof:3.6.23.1]),
///* sqlite3_step() began
///* calling [sqlite3_reset()] automatically in this circumstance rather
///* than returning [SQLITE_MISUSE].  This is not considered a compatibility
///* break because any application that ever receives an SQLITE_MISUSE error
///* is broken by definition.  The [SQLITE_OMIT_AUTORESET] compile-time option
///* can be used to restore the legacy behavior.
///*
///* <b>Goofy Interface Alert:</b> In the legacy interface, the sqlite3_step()
///* API always returns a generic error code, [SQLITE_ERROR], following any
///* error other than [SQLITE_BUSY] and [SQLITE_MISUSE].  You must call
///* [sqlite3_reset()] or [sqlite3_finalize()] in order to find one of the
///* specific [error codes] that better describes the error.
///* We admit that this is a goofy design.  The problem has been fixed
///* with the "v2" interface.  If you prepare all of your SQL statements
///* using [sqlite3_prepare_v3()] or [sqlite3_prepare_v2()]
///* or [sqlite3_prepare16_v2()] or [sqlite3_prepare16_v3()] instead
///* of the legacy [sqlite3_prepare()] and [sqlite3_prepare16()] interfaces,
///* then the more specific [error codes] are returned directly
///* by sqlite3_step().  The use of the "vX" interfaces is recommended.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_step(p_stmt: *mut Sqlite3Stmt) -> i32 {
    let mut rc: i32 = 0;
    /// Result from sqlite3Step()
    let v: *mut Vdbe = p_stmt as *mut Vdbe;
    /// the prepared statement
    let mut cnt: i32 = 0;
    /// Counter to prevent infinite loop of reprepares
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    if vdbe_safety_not_null(v) != 0 {
        return unsafe { sqlite3_misuse_error(991) };
    }
    db = unsafe { (*v).db };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    while { rc = sqlite3_step_2(v); rc } == 17 &&
            { let __p = &mut cnt; let __t = *__p; *__p += 1; __t } < 50 {
        let saved_pc: i32 = unsafe { (*v).pc };
        rc = unsafe { sqlite3_reprepare(v) };
        if rc != 0 {
            /// This case occurs after failing to recompile an sql statement.
            ///* The error message from the SQL compiler has already been loaded
            ///* into the database handle. This block copies the error message
            ///* from the database handle into the statement and sets the statement
            ///* program counter to 0 to ensure that when the statement is
            ///* finalized or reset the parser error message is available via
            ///* sqlite3_errmsg() and sqlite3_errcode().
            let z_err: *const i8 =
                sqlite3_value_text(unsafe { (*db).p_err }) as *const i8;
            unsafe {
                sqlite3_db_free(db, unsafe { (*v).z_err_msg } as *mut ())
            };
            if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                unsafe {
                    (*v).z_err_msg = unsafe { sqlite3_db_str_dup(db, z_err) }
                };
                unsafe {
                    (*v).rc = { rc = unsafe { sqlite3_api_exit(db, rc) }; rc }
                };
            } else {
                unsafe { (*v).z_err_msg = core::ptr::null_mut() };
                unsafe { (*v).rc = { rc = 7; rc } };
            }
            break;
        }
        sqlite3_reset(p_stmt);
        if saved_pc >= 0 {

            /// Setting minWriteFileFormat to 254 is a signal to the OP_Init and
            ///* OP_Trace opcodes to *not* perform SQLITE_TRACE_STMT because it has
            ///* already been done once on a prior invocation that failed due to
            ///* SQLITE_SCHEMA.   tag-20220401a
            unsafe { (*v).min_write_file_format = 254 as u8 };
        }
        { let _ = 0; };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

///* CAPI3REF: Number of columns in a result set
///* METHOD: sqlite3_stmt
///*
///* ^The sqlite3_data_count(P) interface returns the number of columns in the
///* current row of the result set of [prepared statement] P.
///* ^If prepared statement P does not have results ready to return
///* (via calls to the [sqlite3_column_int | sqlite3_column()] family of
///* interfaces) then sqlite3_data_count(P) returns 0.
///* ^The sqlite3_data_count(P) routine also returns 0 if P is a NULL pointer.
///* ^The sqlite3_data_count(P) routine returns 0 if the previous call to
///* [sqlite3_step](P) returned [SQLITE_DONE].  ^The sqlite3_data_count(P)
///* will return non-zero if previous call to [sqlite3_step](P) returned
///* [SQLITE_ROW], except in the case of the [PRAGMA incremental_vacuum]
///* where it always returns zero since each step of that multi-step
///* pragma returns 0 columns of data.
///*
///* See also: [sqlite3_column_count()]
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_data_count(p_stmt: *mut Sqlite3Stmt) -> i32 {
    let p_vm: *const Vdbe = p_stmt as *mut Vdbe as *const Vdbe;
    if p_vm == core::ptr::null_mut() ||
            unsafe { (*p_vm).p_result_row } == core::ptr::null_mut() {
        return 0;
    }
    return unsafe { (*p_vm).n_res_column } as i32;
}

///* CAPI3REF: Obtaining SQL Values
///* METHOD: sqlite3_value
///*
///* <b>Summary:</b>
///* <blockquote><table border=0 cellpadding=0 cellspacing=0>
///* <tr><td><b>sqlite3_value_blob</b><td>&rarr;<td>BLOB value
///* <tr><td><b>sqlite3_value_double</b><td>&rarr;<td>REAL value
///* <tr><td><b>sqlite3_value_int</b><td>&rarr;<td>32-bit INTEGER value
///* <tr><td><b>sqlite3_value_int64</b><td>&rarr;<td>64-bit INTEGER value
///* <tr><td><b>sqlite3_value_pointer</b><td>&rarr;<td>Pointer value
///* <tr><td><b>sqlite3_value_text</b><td>&rarr;<td>UTF-8 TEXT value
///* <tr><td><b>sqlite3_value_text16</b><td>&rarr;<td>UTF-16 TEXT value in
///* the native byteorder
///* <tr><td><b>sqlite3_value_text16be</b><td>&rarr;<td>UTF-16be TEXT value
///* <tr><td><b>sqlite3_value_text16le</b><td>&rarr;<td>UTF-16le TEXT value
///* <tr><td>&nbsp;<td>&nbsp;<td>&nbsp;
///* <tr><td><b>sqlite3_value_bytes</b><td>&rarr;<td>Size of a BLOB
///* or a UTF-8 TEXT in bytes
///* <tr><td><b>sqlite3_value_bytes16&nbsp;&nbsp;</b>
///* <td>&rarr;&nbsp;&nbsp;<td>Size of UTF-16
///* TEXT in bytes
///* <tr><td><b>sqlite3_value_type</b><td>&rarr;<td>Default
///* datatype of the value
///* <tr><td><b>sqlite3_value_numeric_type&nbsp;&nbsp;</b>
///* <td>&rarr;&nbsp;&nbsp;<td>Best numeric datatype of the value
///* <tr><td><b>sqlite3_value_nochange&nbsp;&nbsp;</b>
///* <td>&rarr;&nbsp;&nbsp;<td>True if the column is unchanged in an UPDATE
///* against a virtual table.
///* <tr><td><b>sqlite3_value_frombind&nbsp;&nbsp;</b>
///* <td>&rarr;&nbsp;&nbsp;<td>True if value originated from a [bound parameter]
///* </table></blockquote>
///*
///* <b>Details:</b>
///*
///* These routines extract type, size, and content information from
///* [protected sqlite3_value] objects.  Protected sqlite3_value objects
///* are used to pass parameter information into the functions that
///* implement [application-defined SQL functions] and [virtual tables].
///*
///* These routines work only with [protected sqlite3_value] objects.
///* Any attempt to use these routines on an [unprotected sqlite3_value]
///* is not threadsafe.
///*
///* ^These routines work just like the corresponding [column access functions]
///* except that these routines take a single [protected sqlite3_value] object
///* pointer instead of a [sqlite3_stmt*] pointer and an integer column number.
///*
///* ^The sqlite3_value_text16() interface extracts a UTF-16 string
///* in the native byte-order of the host machine.  ^The
///* sqlite3_value_text16be() and sqlite3_value_text16le() interfaces
///* extract UTF-16 strings as big-endian and little-endian respectively.
///*
///* ^If [sqlite3_value] object V was initialized
///* using [sqlite3_bind_pointer(S,I,P,X,D)] or [sqlite3_result_pointer(C,P,X,D)]
///* and if X and Y are strings that compare equal according to strcmp(X,Y),
///* then sqlite3_value_pointer(V,Y) will return the pointer P.  ^Otherwise,
///* sqlite3_value_pointer(V,Y) returns a NULL. The sqlite3_bind_pointer()
///* routine is part of the [pointer passing interface] added for SQLite 3.20.0.
///*
///* ^(The sqlite3_value_type(V) interface returns the
///* [SQLITE_INTEGER | datatype code] for the initial datatype of the
///* [sqlite3_value] object V. The returned value is one of [SQLITE_INTEGER],
///* [SQLITE_FLOAT], [SQLITE_TEXT], [SQLITE_BLOB], or [SQLITE_NULL].)^
///* Other interfaces might change the datatype for an sqlite3_value object.
///* For example, if the datatype is initially SQLITE_INTEGER and
///* sqlite3_value_text(V) is called to extract a text value for that
///* integer, then subsequent calls to sqlite3_value_type(V) might return
///* SQLITE_TEXT.  Whether or not a persistent internal datatype conversion
///* occurs is undefined and may change from one release of SQLite to the next.
///*
///* ^(The sqlite3_value_numeric_type() interface attempts to apply
///* numeric affinity to the value.  This means that an attempt is
///* made to convert the value to an integer or floating point.  If
///* such a conversion is possible without loss of information (in other
///* words, if the value is a string that looks like a number)
///* then the conversion is performed.  Otherwise no conversion occurs.
///* The [SQLITE_INTEGER | datatype] after conversion is returned.)^
///*
///* ^Within the [xUpdate] method of a [virtual table], the
///* sqlite3_value_nochange(X) interface returns true if and only if
///* the column corresponding to X is unchanged by the UPDATE operation
///* that the xUpdate method call was invoked to implement and if
///* the prior [xColumn] method call that was invoked to extract
///* the value for that column returned without setting a result (probably
///* because it queried [sqlite3_vtab_nochange()] and found that the column
///* was unchanging).  ^Within an [xUpdate] method, any value for which
///* sqlite3_value_nochange(X) is true will in all other respects appear
///* to be a NULL value.  If sqlite3_value_nochange(X) is invoked anywhere other
///* than within an [xUpdate] method call for an UPDATE statement, then
///* the return value is arbitrary and meaningless.
///*
///* ^The sqlite3_value_frombind(X) interface returns non-zero if the
///* value X originated from one of the [sqlite3_bind_int|sqlite3_bind()]
///* interfaces.  ^If X comes from an SQL literal value, or a table column,
///* or an expression, then sqlite3_value_frombind(X) returns zero.
///*
///* Please pay particular attention to the fact that the pointer returned
///* from [sqlite3_value_blob()], [sqlite3_value_text()], or
///* [sqlite3_value_text16()] can be invalidated by a subsequent call to
///* [sqlite3_value_bytes()], [sqlite3_value_bytes16()], [sqlite3_value_text()],
///* or [sqlite3_value_text16()].
///*
///* These routines must be called from the same thread as
///* the SQL function that supplied the [sqlite3_value*] parameters.
///*
///* As long as the input parameter is correct, these routines can only
///* fail if an out-of-memory error occurs while trying to do a
///* UTF8&rarr;UTF16 or UTF16&rarr;UTF8 conversion.
///* If an out-of-memory error occurs, then the return value from these
///* routines is the same as if the column had contained an SQL NULL value.
///* If the input sqlite3_value was not obtained from [sqlite3_value_dup()],
///* then valid SQL NULL returns can also be distinguished from
///* out-of-memory errors after extracting the value
///* by invoking the [sqlite3_errcode()] immediately after the suspicious
///* return value is obtained and before any
///* other SQLite interface is called on the same [database connection].
///* If the input sqlite3_value was obtained from sqlite3_value_dup() then
///* it is disconnected from the database connection and so sqlite3_errcode()
///* will not work.
///* In that case, the only way to distinguish an out-of-memory
///* condition from a true SQL NULL is to invoke sqlite3_value_type() on the
///* input to see if it is NULL prior to trying to extract the value.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_blob(p_val_1: *mut Sqlite3Value)
    -> *const () {
    unsafe {
        let p: *mut Mem = p_val_1 as *mut Mem;
        if unsafe { (*p).flags } as i32 & (16 | 2) != 0 {
            if if unsafe { (*p).flags } as i32 & 1024 != 0 {
                        unsafe { sqlite3_vdbe_mem_expand_blob(p) }
                    } else { 0 } != 0 {
                { let _ = 0; };
                return core::ptr::null();
            }
            unsafe { (*p).flags |= 16 as u16 };
            return if unsafe { (*p).n } != 0 {
                        unsafe { (*p).z }
                    } else { core::ptr::null_mut() } as *const ();
        } else { return unsafe { sqlite3_value_text(p_val_1) } as *const (); }
    }
}

///* Return a pointer to static memory containing an SQL NULL value.
#[allow(unused_doc_comments)]
extern "C" fn column_null_value() -> *const Mem {
    unsafe {

        /// .u          =
        /// .z          =
        /// .n          =
        /// .flags      =
        /// .enc        =
        /// .eSubtype   =
        /// .db         =
        /// .szMalloc   =
        /// .uTemp      =
        /// .zMalloc    =
        /// .xDel       =
        return &null_mem;
    }
}

///* Check to see if column iCol of the given statement is valid.  If
///* it is, return a pointer to the Mem for the value of that column.
///* If iCol is not valid, return a pointer to a Mem which has a value
///* of NULL.
extern "C" fn column_mem(p_stmt_1: *mut Sqlite3Stmt, i: i32) -> *mut Mem {
    let mut p_vm: *const Vdbe = core::ptr::null();
    let mut p_out: *mut Mem = core::ptr::null_mut();
    p_vm = p_stmt_1 as *mut Vdbe;
    if p_vm == core::ptr::null_mut() {
        return column_null_value() as *mut Mem;
    }
    { let _ = 0; };
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p_vm).db }).mutex }) };
    if unsafe { (*p_vm).p_result_row } != core::ptr::null_mut() &&
                i < unsafe { (*p_vm).n_res_column } as i32 && i >= 0 {
        p_out = unsafe { unsafe { (*p_vm).p_result_row.offset(i as isize) } };
    } else {
        unsafe { sqlite3_error(unsafe { (*p_vm).db }, 25) };
        p_out = column_null_value() as *mut Mem;
    }
    return p_out;
}

///* This function is called after invoking an sqlite3_value_XXX function on a
///* column value (i.e. a value returned by evaluating an SQL expression in the
///* select list of a SELECT statement) that may cause a malloc() failure. If
///* malloc() has failed, the threads mallocFailed flag is cleared and the result
///* code of statement pStmt set to SQLITE_NOMEM.
///*
///* Specifically, this is called from within:
///*
///*     sqlite3_column_int()
///*     sqlite3_column_int64()
///*     sqlite3_column_text()
///*     sqlite3_column_text16()
///*     sqlite3_column_double()
///*     sqlite3_column_bytes()
///*     sqlite3_column_bytes16()
///*     sqlite3_column_blob()
#[allow(unused_doc_comments)]
extern "C" fn column_malloc_failure(p_stmt_1: *mut Sqlite3Stmt) -> () {
    /// If malloc() failed during an encoding conversion within an
    ///* sqlite3_column_XXX API, then set the return code of the statement to
    ///* SQLITE_NOMEM. The next call to _step() (if any) will return SQLITE_ERROR
    ///* and _finalize() will return NOMEM.
    let p: *mut Vdbe = p_stmt_1 as *mut Vdbe;
    if !(p).is_null() {
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            (*p).rc =
                unsafe {
                    sqlite3_api_exit(unsafe { (*p).db }, unsafe { (*p).rc })
                }
        };
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    }
}

///* CAPI3REF: Result Values From A Query
///* KEYWORDS: {column access functions}
///* METHOD: sqlite3_stmt
///*
///* <b>Summary:</b>
///* <blockquote><table border=0 cellpadding=0 cellspacing=0>
///* <tr><td><b>sqlite3_column_blob</b><td>&rarr;<td>BLOB result
///* <tr><td><b>sqlite3_column_double</b><td>&rarr;<td>REAL result
///* <tr><td><b>sqlite3_column_int</b><td>&rarr;<td>32-bit INTEGER result
///* <tr><td><b>sqlite3_column_int64</b><td>&rarr;<td>64-bit INTEGER result
///* <tr><td><b>sqlite3_column_text</b><td>&rarr;<td>UTF-8 TEXT result
///* <tr><td><b>sqlite3_column_text16</b><td>&rarr;<td>UTF-16 TEXT result
///* <tr><td><b>sqlite3_column_value</b><td>&rarr;<td>The result as an
///* [sqlite3_value|unprotected sqlite3_value] object.
///* <tr><td>&nbsp;<td>&nbsp;<td>&nbsp;
///* <tr><td><b>sqlite3_column_bytes</b><td>&rarr;<td>Size of a BLOB
///* or a UTF-8 TEXT result in bytes
///* <tr><td><b>sqlite3_column_bytes16&nbsp;&nbsp;</b>
///* <td>&rarr;&nbsp;&nbsp;<td>Size of UTF-16
///* TEXT in bytes
///* <tr><td><b>sqlite3_column_type</b><td>&rarr;<td>Default
///* datatype of the result
///* </table></blockquote>
///*
///* <b>Details:</b>
///*
///* ^These routines return information about a single column of the current
///* result row of a query.  ^In every case the first argument is a pointer
///* to the [prepared statement] that is being evaluated (the [sqlite3_stmt*]
///* that was returned from [sqlite3_prepare_v2()] or one of its variants)
///* and the second argument is the index of the column for which information
///* should be returned. ^The leftmost column of the result set has the index 0.
///* ^The number of columns in the result can be determined using
///* [sqlite3_column_count()].
///*
///* If the SQL statement does not currently point to a valid row, or if the
///* column index is out of range, the result is undefined.
///* These routines may only be called when the most recent call to
///* [sqlite3_step()] has returned [SQLITE_ROW] and neither
///* [sqlite3_reset()] nor [sqlite3_finalize()] have been called subsequently.
///* If any of these routines are called after [sqlite3_reset()] or
///* [sqlite3_finalize()] or after [sqlite3_step()] has returned
///* something other than [SQLITE_ROW], the results are undefined.
///* If [sqlite3_step()] or [sqlite3_reset()] or [sqlite3_finalize()]
///* are called from a different thread while any of these routines
///* are pending, then the results are undefined.
///*
///* The first six interfaces (_blob, _double, _int, _int64, _text, and _text16)
///* each return the value of a result column in a specific data format.  If
///* the result column is not initially in the requested format (for example,
///* if the query returns an integer but the sqlite3_column_text() interface
///* is used to extract the value) then an automatic type conversion is performed.
///*
///* ^The sqlite3_column_type() routine returns the
///* [SQLITE_INTEGER | datatype code] for the initial data type
///* of the result column.  ^The returned value is one of [SQLITE_INTEGER],
///* [SQLITE_FLOAT], [SQLITE_TEXT], [SQLITE_BLOB], or [SQLITE_NULL].
///* The return value of sqlite3_column_type() can be used to decide which
///* of the first six interface should be used to extract the column value.
///* The value returned by sqlite3_column_type() is only meaningful if no
///* automatic type conversions have occurred for the value in question.
///* After a type conversion, the result of calling sqlite3_column_type()
///* is undefined, though harmless.  Future
///* versions of SQLite may change the behavior of sqlite3_column_type()
///* following a type conversion.
///*
///* If the result is a BLOB or a TEXT string, then the sqlite3_column_bytes()
///* or sqlite3_column_bytes16() interfaces can be used to determine the size
///* of that BLOB or string.
///*
///* ^If the result is a BLOB or UTF-8 string then the sqlite3_column_bytes()
///* routine returns the number of bytes in that BLOB or string.
///* ^If the result is a UTF-16 string, then sqlite3_column_bytes() converts
///* the string to UTF-8 and then returns the number of bytes.
///* ^If the result is a numeric value then sqlite3_column_bytes() uses
///* [sqlite3_snprintf()] to convert that value to a UTF-8 string and returns
///* the number of bytes in that string.
///* ^If the result is NULL, then sqlite3_column_bytes() returns zero.
///*
///* ^If the result is a BLOB or UTF-16 string then the sqlite3_column_bytes16()
///* routine returns the number of bytes in that BLOB or string.
///* ^If the result is a UTF-8 string, then sqlite3_column_bytes16() converts
///* the string to UTF-16 and then returns the number of bytes.
///* ^If the result is a numeric value then sqlite3_column_bytes16() uses
///* [sqlite3_snprintf()] to convert that value to a UTF-16 string and returns
///* the number of bytes in that string.
///* ^If the result is NULL, then sqlite3_column_bytes16() returns zero.
///*
///* ^The values returned by [sqlite3_column_bytes()] and
///* [sqlite3_column_bytes16()] do not include the zero terminators at the end
///* of the string.  ^For clarity: the values returned by
///* [sqlite3_column_bytes()] and [sqlite3_column_bytes16()] are the number of
///* bytes in the string, not the number of characters.
///*
///* ^Strings returned by sqlite3_column_text() and sqlite3_column_text16(),
///* even empty strings, are always zero-terminated.  ^The return
///* value from sqlite3_column_blob() for a zero-length BLOB is a NULL pointer.
///*
///* ^Strings returned by sqlite3_column_text16() always have the endianness
///* which is native to the platform, regardless of the text encoding set
///* for the database.
///*
///* <b>Warning:</b> ^The object returned by [sqlite3_column_value()] is an
///* [unprotected sqlite3_value] object.  In a multithreaded environment,
///* an unprotected sqlite3_value object may only be used safely with
///* [sqlite3_bind_value()] and [sqlite3_result_value()].
///* If the [unprotected sqlite3_value] object returned by
///* [sqlite3_column_value()] is used in any other way, including calls
///* to routines like [sqlite3_value_int()], [sqlite3_value_text()],
///* or [sqlite3_value_bytes()], the behavior is not threadsafe.
///* Hence, the sqlite3_column_value() interface
///* is normally only useful within the implementation of
///* [application-defined SQL functions] or [virtual tables], not within
///* top-level application code.
///*
///* These routines may attempt to convert the datatype of the result.
///* ^For example, if the internal representation is FLOAT and a text result
///* is requested, [sqlite3_snprintf()] is used internally to perform the
///* conversion automatically.  ^(The following table details the conversions
///* that are applied:
///*
///* <blockquote>
///* <table border="1">
///* <tr><th> Internal<br>Type <th> Requested<br>Type <th>  Conversion
///*
///* <tr><td>  NULL    <td> INTEGER   <td> Result is 0
///* <tr><td>  NULL    <td>  FLOAT    <td> Result is 0.0
///* <tr><td>  NULL    <td>   TEXT    <td> Result is a NULL pointer
///* <tr><td>  NULL    <td>   BLOB    <td> Result is a NULL pointer
///* <tr><td> INTEGER  <td>  FLOAT    <td> Convert from integer to float
///* <tr><td> INTEGER  <td>   TEXT    <td> ASCII rendering of the integer
///* <tr><td> INTEGER  <td>   BLOB    <td> Same as INTEGER->TEXT
///* <tr><td>  FLOAT   <td> INTEGER   <td> [CAST] to INTEGER
///* <tr><td>  FLOAT   <td>   TEXT    <td> ASCII rendering of the float
///* <tr><td>  FLOAT   <td>   BLOB    <td> [CAST] to BLOB
///* <tr><td>  TEXT    <td> INTEGER   <td> [CAST] to INTEGER
///* <tr><td>  TEXT    <td>  FLOAT    <td> [CAST] to REAL
///* <tr><td>  TEXT    <td>   BLOB    <td> No change
///* <tr><td>  BLOB    <td> INTEGER   <td> [CAST] to INTEGER
///* <tr><td>  BLOB    <td>  FLOAT    <td> [CAST] to REAL
///* <tr><td>  BLOB    <td>   TEXT    <td> [CAST] to TEXT, ensure zero terminator
///* </table>
///* </blockquote>)^
///*
///* Note that when type conversions occur, pointers returned by prior
///* calls to sqlite3_column_blob(), sqlite3_column_text(), and/or
///* sqlite3_column_text16() may be invalidated.
///* Type conversions and pointer invalidations might occur
///* in the following cases:
///*
///* <ul>
///* <li> The initial content is a BLOB and sqlite3_column_text() or
///*      sqlite3_column_text16() is called.  A zero-terminator might
///*      need to be added to the string.</li>
///* <li> The initial content is UTF-8 text and sqlite3_column_bytes16() or
///*      sqlite3_column_text16() is called.  The content must be converted
///*      to UTF-16.</li>
///* <li> The initial content is UTF-16 text and sqlite3_column_bytes() or
///*      sqlite3_column_text() is called.  The content must be converted
///*      to UTF-8.</li>
///* </ul>
///*
///* ^Conversions between UTF-16be and UTF-16le are always done in place and do
///* not invalidate a prior pointer, though of course the content of the buffer
///* that the prior pointer references will have been modified.  Other kinds
///* of conversion are done in place when it is possible, but sometimes they
///* are not possible and in those cases prior pointers are invalidated.
///*
///* The safest policy is to invoke these routines
///* in one of the following ways:
///*
///* <ul>
///*  <li>sqlite3_column_text() followed by sqlite3_column_bytes()</li>
///*  <li>sqlite3_column_blob() followed by sqlite3_column_bytes()</li>
///*  <li>sqlite3_column_text16() followed by sqlite3_column_bytes16()</li>
///* </ul>
///*
///* In other words, you should call sqlite3_column_text(),
///* sqlite3_column_blob(), or sqlite3_column_text16() first to force the result
///* into the desired format, then invoke sqlite3_column_bytes() or
///* sqlite3_column_bytes16() to find the size of the result.  Do not mix calls
///* to sqlite3_column_text() or sqlite3_column_blob() with calls to
///* sqlite3_column_bytes16(), and do not mix calls to sqlite3_column_text16()
///* with calls to sqlite3_column_bytes().
///*
///* ^The pointers returned are valid until a type conversion occurs as
///* described above, or until [sqlite3_step()] or [sqlite3_reset()] or
///* [sqlite3_finalize()] is called.  ^The memory space used to hold strings
///* and BLOBs is freed automatically.  Do not pass the pointers returned
///* from [sqlite3_column_blob()], [sqlite3_column_text()], etc. into
///* [sqlite3_free()].
///*
///* As long as the input parameters are correct, these routines will only
///* fail if an out-of-memory error occurs during a format conversion.
///* Only the following subset of interfaces are subject to out-of-memory
///* errors:
///*
///* <ul>
///* <li> sqlite3_column_blob()
///* <li> sqlite3_column_text()
///* <li> sqlite3_column_text16()
///* <li> sqlite3_column_bytes()
///* <li> sqlite3_column_bytes16()
///* </ul>
///*
///* If an out-of-memory error occurs, then the return value from these
///* routines is the same as if the column had contained an SQL NULL value.
///* Valid SQL NULL returns can be distinguished from out-of-memory errors
///* by invoking the [sqlite3_errcode()] immediately after the suspect
///* return value is obtained and before any
///* other SQLite interface is called on the same [database connection].
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_column_blob(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> *const () {
    let mut val: *const () = core::ptr::null();
    val = sqlite3_value_blob(column_mem(p_stmt, i) as *mut Sqlite3Value);

    /// Even though there is no encoding conversion, value_blob() might
    ///* need to call malloc() to expand the result of a zeroblob()
    ///* expression.
    column_malloc_failure(p_stmt);
    return val;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_double(p_val_1: *mut Sqlite3Value) -> f64 {
    return unsafe { sqlite3_vdbe_real_value(p_val_1 as *mut Mem) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_double(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> f64 {
    let val: f64 =
        sqlite3_value_double(column_mem(p_stmt, i) as *mut Sqlite3Value);
    column_malloc_failure(p_stmt);
    return val;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_int(p_val_1: *mut Sqlite3Value) -> i32 {
    return unsafe {
                sqlite3_vdbe_int_value(p_val_1 as *mut Mem as *const Mem)
            } as i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_int(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> i32 {
    let val: i32 =
        sqlite3_value_int(column_mem(p_stmt, i) as *mut Sqlite3Value);
    column_malloc_failure(p_stmt);
    return val;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_int64(p_val_1: *mut Sqlite3Value)
    -> Sqlite3Int64 {
    return unsafe {
            sqlite3_vdbe_int_value(p_val_1 as *mut Mem as *const Mem)
        };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_int64(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> Sqlite3Int64 {
    let val: SqliteInt64 =
        sqlite3_value_int64(column_mem(p_stmt, i) as *mut Sqlite3Value);
    column_malloc_failure(p_stmt);
    return val;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_text(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> *const u8 {
    let val: *const u8 =
        sqlite3_value_text(column_mem(p_stmt, i) as *mut Sqlite3Value);
    column_malloc_failure(p_stmt);
    return val;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_text16(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> *const () {
    let val: *const () =
        sqlite3_value_text16(column_mem(p_stmt, i) as *mut Sqlite3Value);
    column_malloc_failure(p_stmt);
    return val;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_value(p_stmt_1: *mut Sqlite3Stmt, i: i32)
    -> *mut Sqlite3Value {
    let p_out: *mut Mem = column_mem(p_stmt_1, i);
    if unsafe { (*p_out).flags } as i32 & 8192 != 0 {
        unsafe { (*p_out).flags &= !8192 as u16 };
        unsafe { (*p_out).flags |= 16384 as u16 };
    }
    column_malloc_failure(p_stmt_1);
    return p_out as *mut Sqlite3Value;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_bytes(p_val_1: *mut Sqlite3Value) -> i32 {
    return unsafe { sqlite3ValueBytes(p_val_1, 1 as u8) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_bytes(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> i32 {
    let val: i32 =
        sqlite3_value_bytes(column_mem(p_stmt, i) as *mut Sqlite3Value);
    column_malloc_failure(p_stmt);
    return val;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_bytes16(p_val_1: *mut Sqlite3Value) -> i32 {
    return unsafe { sqlite3ValueBytes(p_val_1, 2 as u8) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_bytes16(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> i32 {
    let val: i32 =
        sqlite3_value_bytes16(column_mem(p_stmt, i) as *mut Sqlite3Value);
    column_malloc_failure(p_stmt);
    return val;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_type(p_stmt: *mut Sqlite3Stmt, i: i32)
    -> i32 {
    let i_type: i32 = sqlite3_value_type(unsafe { &*column_mem(p_stmt, i) });
    column_malloc_failure(p_stmt);
    return i_type;
}

///* CAPI3REF: Destroy A Prepared Statement Object
///* DESTRUCTOR: sqlite3_stmt
///*
///* ^The sqlite3_finalize() function is called to delete a [prepared statement].
///* ^If the most recent evaluation of the statement encountered no errors
///* or if the statement has never been evaluated, then sqlite3_finalize() returns
///* SQLITE_OK.  ^If the most recent evaluation of statement S failed, then
///* sqlite3_finalize(S) returns the appropriate [error code] or
///* [extended error code].
///*
///* ^The sqlite3_finalize(S) routine can be called at any point during
///* the life cycle of [prepared statement] S:
///* before statement S is ever evaluated, after
///* one or more calls to [sqlite3_reset()], or after any call
///* to [sqlite3_step()] regardless of whether or not the statement has
///* completed execution.
///*
///* ^Invoking sqlite3_finalize() on a NULL pointer is a harmless no-op.
///*
///* The application must finalize every [prepared statement] in order to avoid
///* resource leaks.  It is a grievous error for the application to try to use
///* a prepared statement after it has been finalized.  Any use of a prepared
///* statement after it has been finalized can result in undefined and
///* undesirable behavior such as segfaults and heap corruption.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_finalize(p_stmt: *mut Sqlite3Stmt) -> i32 {
    let mut rc: i32 = 0;
    if p_stmt == core::ptr::null_mut() {

        /// IMPLEMENTATION-OF: R-57228-12904 Invoking sqlite3_finalize() on a NULL
        ///* pointer is a harmless no-op.
        (rc = 0);
    } else {
        let v: *mut Vdbe = p_stmt as *mut Vdbe;
        let db: *mut Sqlite3 = unsafe { (*v).db };
        if vdbe_safety(unsafe { &*v }) != 0 {
            return unsafe { sqlite3_misuse_error(114) };
        }
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        if unsafe { (*v).start_time } > 0 as i64 {
            invoke_profile_callback(unsafe { &*db }, v);
        }
        { let _ = 0; };
        rc = unsafe { sqlite3_vdbe_reset(v) };
        unsafe { sqlite3_vdbe_delete(v) };
        rc = unsafe { sqlite3_api_exit(db, rc) };
        unsafe { sqlite3_leave_mutex_and_close_zombie(db) };
    }
    return rc;
}

///* Return the number of times the Step function of an aggregate has been
///* called.
///*
///* This function is deprecated.  Do not use it for new code.  It is
///* provide only to avoid breaking legacy code.  New aggregate function
///* implementations should keep their own counts within their aggregate
///* context.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_aggregate_count(p: &Sqlite3Context) -> i32 {
    unsafe { { let _ = 0; }; return unsafe { (*(*p).p_mem).n }; }
}

///* Return TRUE (non-zero) of the statement supplied as an argument needs
///* to be recompiled.  A statement needs to be recompiled whenever the
///* execution environment changes in a way that would alter the program
///* that sqlite3_prepare() generates.  For example, if new functions or
///* collating sequences are registered or if an authorizer function is
///* added or changed.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expired(p_stmt: *mut Sqlite3Stmt) -> i32 {
    let mut i_ret: i32 = 1;
    if !(p_stmt).is_null() {
        let p: *const Vdbe = p_stmt as *mut Vdbe as *const Vdbe;
        unsafe {
            sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex })
        };
        i_ret = unsafe { (*p).expired() } as i32;
        unsafe {
            sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex })
        };
    }
    return i_ret;
}

///* Transfer all bindings from the first statement over to the second.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3TransferBindings(p_from_stmt: *mut Sqlite3Stmt,
    p_to_stmt: *mut Sqlite3Stmt) -> i32 {
    let p_from: *const Vdbe = p_from_stmt as *mut Vdbe as *const Vdbe;
    let p_to: *const Vdbe = p_to_stmt as *mut Vdbe as *const Vdbe;
    let mut i: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p_to).db }).mutex }) };
    {
        i = 0;
        '__b7: loop {
            if !(i < unsafe { (*p_from).n_var } as i32) { break '__b7; }
            '__c7: loop {
                unsafe {
                    sqlite3_vdbe_mem_move(unsafe {
                            &mut *unsafe { (*p_to).a_var.offset(i as isize) }
                        },
                        unsafe {
                            &mut *unsafe { (*p_from).a_var.offset(i as isize) }
                        })
                };
                break '__c7;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p_to).db }).mutex }) };
    return 0;
}

///* Deprecated external interface.  Internal/core SQLite code
///* should call sqlite3TransferBindings.
///*
///* It is misuse to call this routine with statements from different
///* database connections.  But as this is a deprecated interface, we
///* will not bother to check for that condition.
///*
///* If the two statements contain a different number of bindings, then
///* an SQLITE_ERROR is returned.  Nothing else can go wrong, so otherwise
///* SQLITE_OK is returned.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_transfer_bindings(p_from_stmt: *mut Sqlite3Stmt,
    p_to_stmt: *mut Sqlite3Stmt) -> i32 {
    let p_from: *mut Vdbe = p_from_stmt as *mut Vdbe;
    let p_to: *mut Vdbe = p_to_stmt as *mut Vdbe;
    if unsafe { (*p_from).n_var } as i32 != unsafe { (*p_to).n_var } as i32 {
        return 1;
    }
    { let _ = 0; };
    if unsafe { (*p_to).expmask } != 0 {
        unsafe { (*p_to).set_expired(1 as Bft as u32) };
    }
    { let _ = 0; };
    if unsafe { (*p_from).expmask } != 0 {
        unsafe { (*p_from).set_expired(1 as Bft as u32) };
    }
    return sqlite3TransferBindings(p_from_stmt, p_to_stmt);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_pointer(p_val_1: *mut Sqlite3Value,
    z_p_type_1: *const i8) -> *mut () {
    unsafe {
        let p: *mut Mem = p_val_1 as *mut Mem;
        if unsafe { (*p).flags } as i32 & (3519 | 512 | 2048) ==
                            1 | 512 | 2048 && z_p_type_1 != core::ptr::null() &&
                    unsafe { (*p).e_subtype } as i32 == 'p' as i32 &&
                unsafe { strcmp(unsafe { (*p).u.z_p_type }, z_p_type_1) } == 0
            {
            return unsafe { (*p).z } as *mut ();
        } else { return core::ptr::null_mut(); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_text16le(p_val_1: *mut Sqlite3Value)
    -> *const () {
    return unsafe { sqlite3ValueText(p_val_1, 2 as u8) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_text16be(p_val_1: *mut Sqlite3Value)
    -> *const () {
    return unsafe { sqlite3ValueText(p_val_1, 3 as u8) };
}

/// Return true if a parameter to xUpdate represents an unchanged column
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_nochange(p_val_1: &Sqlite3Value) -> i32 {
    return ((*p_val_1).flags as i32 & (1 | 1024) == 1 | 1024) as i32;
}

/// Return true if a parameter value originated from an sqlite3_bind()
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_frombind(p_val_1: &Sqlite3Value) -> i32 {
    return ((*p_val_1).flags as i32 & 64 != 0) as i32;
}

///* CAPI3REF: Report the internal text encoding state of an sqlite3_value object
///* METHOD: sqlite3_value
///*
///* ^(The sqlite3_value_encoding(X) interface returns one of [SQLITE_UTF8],
///* [SQLITE_UTF16BE], or [SQLITE_UTF16LE] according to the current text encoding
///* of the value X, assuming that X has type TEXT.)^  If sqlite3_value_type(X)
///* returns something other than SQLITE_TEXT, then the return value from
///* sqlite3_value_encoding(X) is meaningless.  ^Calls to
///* [sqlite3_value_text(X)], [sqlite3_value_text16(X)],
///* [sqlite3_value_text16be(X)],
///* [sqlite3_value_text16le(X)], [sqlite3_value_bytes(X)], or
///* [sqlite3_value_bytes16(X)] might change the encoding of the value X and
///* thus change the return from subsequent calls to sqlite3_value_encoding(X).
///*
///* This routine is intended for used by applications that test and validate
///* the SQLite implementation.  This routine is inquiring about the opaque
///* internal state of an [sqlite3_value] object.  Ordinary applications should
///* not need to know what the internal state of an sqlite3_value object is and
///* hence should not need to use this interface.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_encoding(p_val_1: &Sqlite3Value) -> i32 {
    return (*p_val_1).enc as i32;
}

///* CAPI3REF: Finding The Subtype Of SQL Values
///* METHOD: sqlite3_value
///*
///* The sqlite3_value_subtype(V) function returns the subtype for
///* an [application-defined SQL function] argument V.  The subtype
///* information can be used to pass a limited amount of context from
///* one SQL function to another.  Use the [sqlite3_result_subtype()]
///* routine to set the subtype for the return value of an SQL function.
///*
///* Every [application-defined SQL function] that invokes this interface
///* should include the [SQLITE_SUBTYPE] property in the text
///* encoding argument when the function is [sqlite3_create_function|registered].
///* If the [SQLITE_SUBTYPE] property is omitted, then sqlite3_value_subtype()
///* might return zero instead of the upstream subtype in some corner cases.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_subtype(p_val_1: *mut Sqlite3Value) -> u32 {
    let p_mem: *const Mem = p_val_1 as *mut Mem as *const Mem;
    return if unsafe { (*p_mem).flags } as i32 & 2048 != 0 {
                (unsafe { (*p_mem).e_subtype }) as i32
            } else { 0 } as u32;
}

///* CAPI3REF: Copy And Free SQL Values
///* METHOD: sqlite3_value
///*
///* ^The sqlite3_value_dup(V) interface makes a copy of the [sqlite3_value]
///* object V and returns a pointer to that copy.  ^The [sqlite3_value] returned
///* is a [protected sqlite3_value] object even if the input is not.
///* ^The sqlite3_value_dup(V) interface returns NULL if V is NULL or if a
///* memory allocation fails. ^If V is a [pointer value], then the result
///* of sqlite3_value_dup(V) is a NULL value.
///*
///* ^The sqlite3_value_free(V) interface frees an [sqlite3_value] object
///* previously obtained from [sqlite3_value_dup()].  ^If V is a NULL pointer
///* then sqlite3_value_free(V) is a harmless no-op.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_value_dup(p_orig_1: *const Sqlite3Value)
    -> *mut Sqlite3Value {
    let mut p_new: *mut Sqlite3Value = core::ptr::null_mut();
    if p_orig_1 == core::ptr::null() { return core::ptr::null_mut(); }
    p_new =
        unsafe { sqlite3_malloc(core::mem::size_of::<Sqlite3Value>() as i32) }
            as *mut Sqlite3Value;
    if p_new == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe {
        memset(p_new as *mut (), 0,
            core::mem::size_of::<Sqlite3Value>() as u64)
    };
    unsafe {
        memcpy(p_new as *mut (), p_orig_1 as *const (),
            core::mem::offset_of!(Sqlite3Value, db) as u64)
    };
    unsafe { (*p_new).flags &= !4096 as u16 };
    unsafe { (*p_new).db = core::ptr::null_mut() };
    if unsafe { (*p_new).flags } as i32 & (2 | 16) != 0 {
        unsafe { (*p_new).flags &= !(8192 | 4096) as u16 };
        unsafe { (*p_new).flags |= 16384 as u16 };
        if unsafe { sqlite3_vdbe_mem_make_writeable(p_new as *mut Mem) } != 0
            {
            unsafe { sqlite3ValueFree(p_new) };
            p_new = core::ptr::null_mut();
        }
    } else if unsafe { (*p_new).flags } as i32 & 1 != 0 {

        /// Do not duplicate pointer values
        unsafe { (*p_new).flags &= !(512 | 2048) as u16 };
    }
    return p_new;
}

/// Destroy an sqlite3_value object previously obtained from
///* sqlite3_value_dup().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_value_free(p_old_1: *mut Sqlite3Value) -> () {
    unsafe { sqlite3ValueFree(p_old_1) };
}

///* Create a new aggregate context for p and return a pointer to
///* its pMem->z element.
extern "C" fn create_agg_context(p: &Sqlite3Context, n_byte_1: i32)
    -> *mut () {
    unsafe {
        let p_mem: *mut Mem = (*p).p_mem;
        { let _ = 0; };
        if n_byte_1 <= 0 {
            unsafe { sqlite3_vdbe_mem_set_null(p_mem) };
            unsafe { (*p_mem).z = core::ptr::null_mut() };
        } else {
            unsafe { sqlite3_vdbe_mem_clear_and_resize(p_mem, n_byte_1) };
            unsafe { (*p_mem).flags = 32768 as u16 };
            unsafe { (*p_mem).u.p_def = (*p).p_func };
            if !(unsafe { (*p_mem).z }).is_null() {
                unsafe {
                    memset(unsafe { (*p_mem).z } as *mut (), 0, n_byte_1 as u64)
                };
            }
        }
        return unsafe { (*p_mem).z } as *mut ();
    }
}

///* CAPI3REF: Obtain Aggregate Function Context
///* METHOD: sqlite3_context
///*
///* Implementations of aggregate SQL functions use this
///* routine to allocate memory for storing their state.
///*
///* ^The first time the sqlite3_aggregate_context(C,N) routine is called
///* for a particular aggregate function, SQLite allocates
///* N bytes of memory, zeroes out that memory, and returns a pointer
///* to the new memory. ^On second and subsequent calls to
///* sqlite3_aggregate_context() for the same aggregate function instance,
///* the same buffer is returned.  Sqlite3_aggregate_context() is normally
///* called once for each invocation of the xStep callback and then one
///* last time when the xFinal callback is invoked.  ^(When no rows match
///* an aggregate query, the xStep() callback of the aggregate function
///* implementation is never called and xFinal() is called exactly once.
///* In those cases, sqlite3_aggregate_context() might be called for the
///* first time from within xFinal().)^
///*
///* ^The sqlite3_aggregate_context(C,N) routine returns a NULL pointer
///* when first called if N is less than or equal to zero or if a memory
///* allocation error occurs.
///*
///* ^(The amount of space allocated by sqlite3_aggregate_context(C,N) is
///* determined by the N parameter on the first successful call.  Changing the
///* value of N in any subsequent call to sqlite3_aggregate_context() within
///* the same aggregate function instance will not resize the memory
///* allocation.)^  Within the xFinal callback, it is customary to set
///* N=0 in calls to sqlite3_aggregate_context(C,N) so that no
///* pointless memory allocations occur.
///*
///* ^SQLite automatically frees the memory allocated by
///* sqlite3_aggregate_context() when the aggregate query concludes.
///*
///* The first parameter must be a copy of the
///* [sqlite3_context | SQL function context] that is the first parameter
///* to the xStep or xFinal callback routine that implements the aggregate
///* function.
///*
///* This routine must be called from the same thread in which
///* the aggregate SQL function is running.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_aggregate_context(p: *mut Sqlite3Context,
    n_byte_1: i32) -> *mut () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*unsafe { (*p).p_mem }).flags } as i32 & 32768 == 0 {
            return create_agg_context(unsafe { &*p }, n_byte_1);
        } else { return unsafe { (*unsafe { (*p).p_mem }).z } as *mut (); }
    }
}

///* CAPI3REF: User Data For Functions
///* METHOD: sqlite3_context
///*
///* ^The sqlite3_user_data() interface returns a copy of
///* the pointer that was the pUserData parameter (the 5th parameter)
///* of the [sqlite3_create_function()]
///* and [sqlite3_create_function16()] routines that originally
///* registered the application defined function.
///*
///* This routine must be called from the same thread in which
///* the application-defined function is running.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_user_data(p: &Sqlite3Context) -> *mut () {
    unsafe { { let _ = 0; }; return unsafe { (*(*p).p_func).p_user_data }; }
}

///* CAPI3REF: Database Connection For Functions
///* METHOD: sqlite3_context
///*
///* ^The sqlite3_context_db_handle() interface returns a copy of
///* the pointer to the [database connection] (the 1st parameter)
///* of the [sqlite3_create_function()]
///* and [sqlite3_create_function16()] routines that originally
///* registered the application defined function.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_context_db_handle(p: &Sqlite3Context)
    -> *mut Sqlite3 {
    { let _ = 0; };
    return unsafe { (*(*p).p_out).db };
}

///* CAPI3REF: Function Auxiliary Data
///* METHOD: sqlite3_context
///*
///* These functions may be used by (non-aggregate) SQL functions to
///* associate auxiliary data with argument values. If the same argument
///* value is passed to multiple invocations of the same SQL function during
///* query execution, under some circumstances the associated auxiliary data
///* might be preserved.  An example of where this might be useful is in a
///* regular-expression matching function. The compiled version of the regular
///* expression can be stored as auxiliary data associated with the pattern
///* string. Then as long as the pattern string remains the same,
///* the compiled regular expression can be reused on multiple
///* invocations of the same function.
///*
///* ^The sqlite3_get_auxdata(C,N) interface returns a pointer to the auxiliary
///* data associated by the sqlite3_set_auxdata(C,N,P,X) function with the
///* Nth argument value to the application-defined function.  ^N is zero
///* for the left-most function argument.  ^If there is no auxiliary data
///* associated with the function argument, the sqlite3_get_auxdata(C,N)
///* interface returns a NULL pointer.
///*
///* ^The sqlite3_set_auxdata(C,N,P,X) interface saves P as auxiliary data for the
///* N-th argument of the application-defined function.  ^Subsequent
///* calls to sqlite3_get_auxdata(C,N) return P from the most recent
///* sqlite3_set_auxdata(C,N,P,X) call if the auxiliary data is still valid or
///* NULL if the auxiliary data has been discarded.
///* ^After each call to sqlite3_set_auxdata(C,N,P,X) where X is not NULL,
///* SQLite will invoke the destructor function X with parameter P exactly
///* once, when the auxiliary data is discarded.
///* SQLite is free to discard the auxiliary data at any time, including: <ul>
///* <li> ^(when the corresponding function parameter changes)^, or
///* <li> ^(when [sqlite3_reset()] or [sqlite3_finalize()] is called for the
///*      SQL statement)^, or
///* <li> ^(when sqlite3_set_auxdata() is invoked again on the same
///*       parameter)^, or
///* <li> ^(during the original sqlite3_set_auxdata() call when a memory
///*      allocation error occurs.)^
///* <li> ^(during the original sqlite3_set_auxdata() call if the function
///*      is evaluated during query planning instead of during query execution,
///*      as sometimes happens with [SQLITE_ENABLE_STAT4].)^ </ul>
///*
///* Note the last two bullets in particular.  The destructor X in
///* sqlite3_set_auxdata(C,N,P,X) might be called immediately, before the
///* sqlite3_set_auxdata() interface even returns.  Hence sqlite3_set_auxdata()
///* should be called near the end of the function implementation and the
///* function implementation should not make any use of P after
///* sqlite3_set_auxdata() has been called.  Furthermore, a call to
///* sqlite3_get_auxdata() that occurs immediately after a corresponding call
///* to sqlite3_set_auxdata() might still return NULL if an out-of-memory
///* condition occurred during the sqlite3_set_auxdata() call or if the
///* function is being evaluated during query planning rather than during
///* query execution.
///*
///* ^(In practice, auxiliary data is preserved between function calls for
///* function parameters that are compile-time constants, including literal
///* values and [parameters] and expressions composed from the same.)^
///*
///* The value of the N parameter to these interfaces should be non-negative.
///* Future enhancements may make use of negative N values to define new
///* kinds of function caching behavior.
///*
///* These routines must be called from the same thread in which
///* the SQL function is running.
///*
///* See also: [sqlite3_get_clientdata()] and [sqlite3_set_clientdata()].
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_auxdata(p_ctx_1: &Sqlite3Context, i_arg_1: i32)
    -> *mut () {
    let mut p_aux_data: *const AuxData = core::ptr::null();
    { let _ = 0; };
    { let _ = 0; };
    {
        p_aux_data = unsafe { (*(*p_ctx_1).p_vdbe).p_aux_data };
        '__b8: loop {
            if !(!(p_aux_data).is_null()) { break '__b8; }
            '__c8: loop {
                if unsafe { (*p_aux_data).i_aux_arg } == i_arg_1 &&
                        (unsafe { (*p_aux_data).i_aux_op } == (*p_ctx_1).i_op ||
                            i_arg_1 < 0) {
                    return unsafe { (*p_aux_data).p_aux };
                }
                break '__c8;
            }
            p_aux_data = unsafe { (*p_aux_data).p_next_aux };
        }
    }
    return core::ptr::null_mut();
}

///* Set the auxiliary data pointer and delete function, for the iArg'th
///* argument to the user-function defined by pCtx. Any previous value is
///* deleted by calling the delete function specified when it was set.
///*
///* The left-most argument is 0.
///*
///* Undocumented behavior:  If iArg is negative then make the data available
///* to all functions within the current prepared statement using iArg as an
///* access code.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_set_auxdata(p_ctx_1: &mut Sqlite3Context,
    i_arg_1: i32, p_aux_1: *mut (),
    x_delete_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    let mut p_aux_data: *mut AuxData = core::ptr::null_mut();
    let mut p_vdbe: *mut Vdbe = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s10:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    if x_delete_1.is_some() {
                        __state = 28;
                    } else { __state = 1; }
                }
                3 => { __state = 4; }
                4 => { p_vdbe = (*p_ctx_1).p_vdbe; __state = 5; }
                5 => { { let _ = 0; }; __state = 6; }
                6 => { { let _ = 0; }; __state = 7; }
                7 => {
                    p_aux_data = unsafe { (*p_vdbe).p_aux_data };
                    __state = 9;
                }
                8 => {
                    if p_aux_data == core::ptr::null_mut() {
                        __state = 14;
                    } else { __state = 15; }
                }
                9 => {
                    if !(p_aux_data).is_null() {
                        __state = 10;
                    } else { __state = 8; }
                }
                10 => {
                    if unsafe { (*p_aux_data).i_aux_arg } == i_arg_1 &&
                            (unsafe { (*p_aux_data).i_aux_op } == (*p_ctx_1).i_op ||
                                i_arg_1 < 0) {
                        __state = 12;
                    } else { __state = 11; }
                }
                11 => {
                    p_aux_data = unsafe { (*p_aux_data).p_next_aux };
                    __state = 9;
                }
                12 => { __state = 8; }
                13 => {
                    unsafe { (*p_aux_data).p_aux = p_aux_1 };
                    __state = 25;
                }
                14 => {
                    p_aux_data =
                        unsafe {
                                sqlite3_db_malloc_zero(unsafe { (*p_vdbe).db },
                                    core::mem::size_of::<AuxData>() as u64)
                            } as *mut AuxData;
                    __state = 16;
                }
                15 => {
                    if unsafe { (*p_aux_data).x_delete_aux.is_some() } {
                        __state = 24;
                    } else { __state = 13; }
                }
                16 => {
                    if (p_aux_data).is_null() as i32 != 0 {
                        __state = 18;
                    } else { __state = 17; }
                }
                17 => {
                    unsafe { (*p_aux_data).i_aux_op = (*p_ctx_1).i_op };
                    __state = 19;
                }
                18 => { __state = 2; }
                19 => {
                    unsafe { (*p_aux_data).i_aux_arg = i_arg_1 };
                    __state = 20;
                }
                20 => {
                    unsafe {
                        (*p_aux_data).p_next_aux = unsafe { (*p_vdbe).p_aux_data }
                    };
                    __state = 21;
                }
                21 => {
                    unsafe { (*p_vdbe).p_aux_data = p_aux_data };
                    __state = 22;
                }
                22 => {
                    if (*p_ctx_1).is_error == 0 {
                        __state = 23;
                    } else { __state = 13; }
                }
                23 => { (*p_ctx_1).is_error = -1; __state = 13; }
                24 => {
                    unsafe {
                        (unsafe {
                                (*p_aux_data).x_delete_aux.unwrap()
                            })(unsafe { (*p_aux_data).p_aux })
                    };
                    __state = 13;
                }
                25 => {
                    unsafe { (*p_aux_data).x_delete_aux = x_delete_1 };
                    __state = 26;
                }
                26 => { return; }
                27 => { __state = 2; }
                28 => {
                    unsafe { x_delete_1.unwrap()(p_aux_1) };
                    __state = 1;
                }
                _ => {}
            }
        }
    }
}

/// Cause the SQL function to raise an SQLITE_TOOBIG error.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_error_toobig(p_ctx_1: &mut Sqlite3Context)
    -> () {
    { let _ = 0; };
    (*p_ctx_1).is_error = 18;
    unsafe {
        sqlite3_vdbe_mem_set_str((*p_ctx_1).p_out,
            c"string or blob too big".as_ptr() as *mut i8 as *const i8,
            -1 as i64, 1 as u8, None)
    };
}

/// Cause the SQL function to raise an SQLITE_NOMEM error.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_error_nomem(p_ctx_1: &mut Sqlite3Context)
    -> () {
    { let _ = 0; };
    unsafe { sqlite3_vdbe_mem_set_null((*p_ctx_1).p_out) };
    (*p_ctx_1).is_error = 7;
    unsafe { sqlite3_oom_fault(unsafe { (*(*p_ctx_1).p_out).db }) };
}

///************************* sqlite3_result_  *******************************
///* The following routines are used by application-defined SQL functions to
///* specify the function return value.  There are many variations on
///* sqlite3_result_xxxx() for different types of return values.
///*
///* The setStrOrError() function is a helper function that invokes
///* sqlite3VdbeMemSetStr() to store the result as a string or blob.
///* Appropriate errors are set if the string/blob is too big or if
///* an OOM occurs.
///*
///* The invokeValueDestructor(P,X) helper function invokes the destructor
///* function X() on value P if P is not going to be used and need to
///* be destroyed.
#[allow(unused_doc_comments)]
extern "C" fn set_result_str_or_error(p_ctx_1: *mut Sqlite3Context,
    z: *const i8, n: i32, enc: u8,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    let p_out: *mut Mem = unsafe { (*p_ctx_1).p_out };
    let mut rc: i32 = 0;
    if enc as i32 == 1 {
        rc =
            unsafe { sqlite3_vdbe_mem_set_text(p_out, z, n as i64, x_del_1) };
    } else if enc as i32 == 16 {

        /// It is usually considered improper to assert() on an input. However,
        ///* the following assert() is checking for inputs that are documented
        ///* to result in undefined behavior.
        { let _ = 0; };
        rc =
            unsafe { sqlite3_vdbe_mem_set_text(p_out, z, n as i64, x_del_1) };
        unsafe { (*p_out).flags |= 512 as u16 };
    } else {
        rc =
            unsafe {
                sqlite3_vdbe_mem_set_str(p_out, z, n as i64, enc, x_del_1)
            };
    }
    if rc != 0 {
        if rc == 18 {
            unsafe { sqlite3_result_error_toobig(unsafe { &mut *p_ctx_1 }) };
        } else {

            /// The only errors possible from sqlite3VdbeMemSetStr are
            ///* SQLITE_TOOBIG and SQLITE_NOMEM
            { let _ = 0; };
            unsafe { sqlite3_result_error_nomem(unsafe { &mut *p_ctx_1 }) };
        }
        return;
    }
    unsafe {
        sqlite3_vdbe_change_encoding(p_out, unsafe { (*p_ctx_1).enc } as i32)
    };
    if unsafe { sqlite3_vdbe_mem_too_big(p_out) } != 0 {
        unsafe { sqlite3_result_error_toobig(unsafe { &mut *p_ctx_1 }) };
    }
}

///* CAPI3REF: Setting The Result Of An SQL Function
///* METHOD: sqlite3_context
///*
///* These routines are used by the xFunc or xFinal callbacks that
///* implement SQL functions and aggregates.  See
///* [sqlite3_create_function()] and [sqlite3_create_function16()]
///* for additional information.
///*
///* These functions work very much like the [parameter binding] family of
///* functions used to bind values to host parameters in prepared statements.
///* Refer to the [SQL parameter] documentation for additional information.
///*
///* ^The sqlite3_result_blob() interface sets the result from
///* an application-defined function to be the BLOB whose content is pointed
///* to by the second parameter and which is N bytes long where N is the
///* third parameter.
///*
///* ^The sqlite3_result_zeroblob(C,N) and sqlite3_result_zeroblob64(C,N)
///* interfaces set the result of the application-defined function to be
///* a BLOB containing all zero bytes and N bytes in size.
///*
///* ^The sqlite3_result_double() interface sets the result from
///* an application-defined function to be a floating point value specified
///* by its 2nd argument.
///*
///* ^The sqlite3_result_error() and sqlite3_result_error16() functions
///* cause the implemented SQL function to throw an exception.
///* ^SQLite uses the string pointed to by the
///* 2nd parameter of sqlite3_result_error() or sqlite3_result_error16()
///* as the text of an error message.  ^SQLite interprets the error
///* message string from sqlite3_result_error() as UTF-8. ^SQLite
///* interprets the string from sqlite3_result_error16() as UTF-16 using
///* the same [byte-order determination rules] as [sqlite3_bind_text16()].
///* ^If the third parameter to sqlite3_result_error()
///* or sqlite3_result_error16() is negative then SQLite takes as the error
///* message all text up through the first zero character.
///* ^If the third parameter to sqlite3_result_error() or
///* sqlite3_result_error16() is non-negative then SQLite takes that many
///* bytes (not characters) from the 2nd parameter as the error message.
///* ^The sqlite3_result_error() and sqlite3_result_error16()
///* routines make a private copy of the error message text before
///* they return.  Hence, the calling function can deallocate or
///* modify the text after they return without harm.
///* ^The sqlite3_result_error_code() function changes the error code
///* returned by SQLite as a result of an error in a function.  ^By default,
///* the error code is SQLITE_ERROR.  ^A subsequent call to sqlite3_result_error()
///* or sqlite3_result_error16() resets the error code to SQLITE_ERROR.
///*
///* ^The sqlite3_result_error_toobig() interface causes SQLite to throw an
///* error indicating that a string or BLOB is too long to represent.
///*
///* ^The sqlite3_result_error_nomem() interface causes SQLite to throw an
///* error indicating that a memory allocation failed.
///*
///* ^The sqlite3_result_int() interface sets the return value
///* of the application-defined function to be the 32-bit signed integer
///* value given in the 2nd argument.
///* ^The sqlite3_result_int64() interface sets the return value
///* of the application-defined function to be the 64-bit signed integer
///* value given in the 2nd argument.
///*
///* ^The sqlite3_result_null() interface sets the return value
///* of the application-defined function to be NULL.
///*
///* ^The sqlite3_result_text(), sqlite3_result_text16(),
///* sqlite3_result_text16le(), and sqlite3_result_text16be() interfaces
///* set the return value of the application-defined function to be
///* a text string which is represented as UTF-8, UTF-16 native byte order,
///* UTF-16 little endian, or UTF-16 big endian, respectively.
///* ^The sqlite3_result_text64(C,Z,N,D,E) interface sets the return value of an
///* application-defined function to be a text string in an encoding
///* specified the E parameter, which must be one
///* of [SQLITE_UTF8], [SQLITE_UTF8_ZT], [SQLITE_UTF16], [SQLITE_UTF16BE],
///* or [SQLITE_UTF16LE].  ^The special value [SQLITE_UTF8_ZT] means that
///* the result text is both UTF-8 and zero-terminated.  In other words,
///* SQLITE_UTF8_ZT means that the Z array holds at least N+1 bytes and that
///* the Z&#91;N&#93; is zero.
///* ^SQLite takes the text result from the application from
///* the 2nd parameter of the sqlite3_result_text* interfaces.
///* ^If the 3rd parameter to any of the sqlite3_result_text* interfaces
///* other than sqlite3_result_text64() is negative, then SQLite computes
///* the string length itself by searching the 2nd parameter for the first
///* zero character.
///* ^If the 3rd parameter to the sqlite3_result_text* interfaces
///* is non-negative, then as many bytes (not characters) of the text
///* pointed to by the 2nd parameter are taken as the application-defined
///* function result.  If the 3rd parameter is non-negative, then it
///* must be the byte offset into the string where the NUL terminator would
///* appear if the string were NUL terminated.  If any NUL characters occur
///* in the string at a byte offset that is less than the value of the 3rd
///* parameter, then the resulting string will contain embedded NULs and the
///* result of expressions operating on strings with embedded NULs is undefined.
///* ^If the 4th parameter to the sqlite3_result_text* interfaces
///* or sqlite3_result_blob is a non-NULL pointer, then SQLite calls that
///* function as the destructor on the text or BLOB result when it has
///* finished using that result.
///* ^If the 4th parameter to the sqlite3_result_text* interfaces or to
///* sqlite3_result_blob is the special constant SQLITE_STATIC, then SQLite
///* assumes that the text or BLOB result is in constant space and does not
///* copy the content of the parameter nor call a destructor on the content
///* when it has finished using that result.
///* ^If the 4th parameter to the sqlite3_result_text* interfaces
///* or sqlite3_result_blob is the special constant SQLITE_TRANSIENT
///* then SQLite makes a copy of the result into space obtained
///* from [sqlite3_malloc()] before it returns.
///*
///* ^For the sqlite3_result_text16(), sqlite3_result_text16le(), and
///* sqlite3_result_text16be() routines, and for sqlite3_result_text64()
///* when the encoding is not UTF8, if the input UTF16 begins with a
///* byte-order mark (BOM, U+FEFF) then the BOM is removed from the
///* string and the rest of the string is interpreted according to the
///* byte-order specified by the BOM.  ^The byte-order specified by
///* the BOM at the beginning of the text overrides the byte-order
///* specified by the interface procedure.  ^So, for example, if
///* sqlite3_result_text16le() is invoked with text that begins
///* with bytes 0xfe, 0xff (a big-endian byte-order mark) then the
///* first two bytes of input are skipped and the remaining input
///* is interpreted as UTF16BE text.
///*
///* ^For UTF16 input text to the sqlite3_result_text16(),
///* sqlite3_result_text16be(), sqlite3_result_text16le(), and
///* sqlite3_result_text64() routines, if the text contains invalid
///* UTF16 characters, the invalid characters might be converted
///* into the unicode replacement character, U+FFFD.
///*
///* ^The sqlite3_result_value() interface sets the result of
///* the application-defined function to be a copy of the
///* [unprotected sqlite3_value] object specified by the 2nd parameter.  ^The
///* sqlite3_result_value() interface makes a copy of the [sqlite3_value]
///* so that the [sqlite3_value] specified in the parameter may change or
///* be deallocated after sqlite3_result_value() returns without harm.
///* ^A [protected sqlite3_value] object may always be used where an
///* [unprotected sqlite3_value] object is required, so either
///* kind of [sqlite3_value] object can be used with this interface.
///*
///* ^The sqlite3_result_pointer(C,P,T,D) interface sets the result to an
///* SQL NULL value, just like [sqlite3_result_null(C)], except that it
///* also associates the host-language pointer P or type T with that
///* NULL value such that the pointer can be retrieved within an
///* [application-defined SQL function] using [sqlite3_value_pointer()].
///* ^If the D parameter is not NULL, then it is a pointer to a destructor
///* for the P parameter.  ^SQLite invokes D with P as its only argument
///* when SQLite is finished with P.  The T parameter should be a static
///* string and preferably a string literal. The sqlite3_result_pointer()
///* routine is part of the [pointer passing interface] added for SQLite 3.20.0.
///*
///* If these routines are called from within a different thread
///* than the one containing the application-defined function that received
///* the [sqlite3_context] pointer, the results are undefined.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_blob(p_ctx_1: *mut Sqlite3Context,
    z: *const (), n: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    { let _ = 0; };
    { let _ = 0; };
    set_result_str_or_error(p_ctx_1, z as *const i8, n, 0 as u8, x_del_1);
}

extern "C" fn invoke_value_destructor(p: *const (),
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    p_ctx_1: *mut Sqlite3Context) -> i32 {
    { let _ = 0; };
    if !x_del_1.is_some() as i32 != 0
        {} else if x_del_1 ==
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }) {} else { unsafe { x_del_1.unwrap()(p as *mut ()) }; }
    { let _ = 0; };
    unsafe { sqlite3_result_error_toobig(unsafe { &mut *p_ctx_1 }) };
    return 18;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_blob64(p_ctx_1: *mut Sqlite3Context,
    z: *const (), n: Sqlite3Uint64,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    { let _ = 0; };
    { let _ = 0; };
    if n > 2147483647 as u64 {
        { let _ = invoke_value_destructor(z, x_del_1, p_ctx_1); };
    } else {
        set_result_str_or_error(p_ctx_1, z as *const i8, n as i32, 0 as u8,
            x_del_1);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_double(p_ctx_1: &Sqlite3Context,
    r_val_1: f64) -> () {
    { let _ = 0; };
    unsafe { sqlite3_vdbe_mem_set_double((*p_ctx_1).p_out, r_val_1) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_error(p_ctx_1: &mut Sqlite3Context,
    z: *const i8, n: i32) -> () {
    { let _ = 0; };
    (*p_ctx_1).is_error = 1;
    unsafe {
        sqlite3_vdbe_mem_set_str((*p_ctx_1).p_out, z, n as i64, 1 as u8,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_error16(p_ctx_1: &mut Sqlite3Context,
    z: *const (), n: i32) -> () {
    { let _ = 0; };
    (*p_ctx_1).is_error = 1;
    unsafe {
        sqlite3_vdbe_mem_set_str((*p_ctx_1).p_out, z as *const i8, n as i64,
            2 as u8,
            Some(unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut ())
                                -> ()>(-1 as isize as *const ())
                }))
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_error_code(p_ctx_1: *mut Sqlite3Context,
    err_code_1: i32) -> () {
    unsafe {
        (*p_ctx_1).is_error = if err_code_1 != 0 { err_code_1 } else { -1 }
    };
    if unsafe { (*unsafe { (*p_ctx_1).p_out }).flags } as i32 & 1 != 0 {
        set_result_str_or_error(p_ctx_1,
            unsafe { sqlite3_err_str(err_code_1) }, -1, 1 as u8, None);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_int(p_ctx_1: &Sqlite3Context, i_val_1: i32)
    -> () {
    { let _ = 0; };
    unsafe { sqlite3_vdbe_mem_set_int64((*p_ctx_1).p_out, i_val_1 as i64) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_int64(p_ctx_1: &Sqlite3Context, i_val_1: i64)
    -> () {
    { let _ = 0; };
    unsafe { sqlite3_vdbe_mem_set_int64((*p_ctx_1).p_out, i_val_1) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_null(p_ctx_1: &Sqlite3Context) -> () {
    { let _ = 0; };
    unsafe { sqlite3_vdbe_mem_set_null((*p_ctx_1).p_out) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_text(p_ctx_1: *mut Sqlite3Context,
    z: *const i8, n: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    { let _ = 0; };
    set_result_str_or_error(p_ctx_1, z, n, 1 as u8, x_del_1);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_text64(p_ctx_1: *mut Sqlite3Context,
    z: *const i8, mut n: Sqlite3Uint64,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>, mut enc: u8) -> () {
    { let _ = 0; };
    { let _ = 0; };
    if enc as i32 != 1 && enc as i32 != 16 {
        if enc as i32 == 4 { enc = 2 as u8; }
        n &= !(1 as u64);
    }
    if n > 2147483647 as u64 {
        {
            let _ = invoke_value_destructor(z as *const (), x_del_1, p_ctx_1);
        };
    } else {
        set_result_str_or_error(p_ctx_1, z, n as i32, enc, x_del_1);
        unsafe {
            sqlite3_vdbe_mem_zero_terminate_if_able(unsafe {
                    (*p_ctx_1).p_out
                })
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_text16(p_ctx_1: *mut Sqlite3Context,
    z: *const (), n: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    { let _ = 0; };
    set_result_str_or_error(p_ctx_1, z as *const i8,
        (n as u64 & !(1 as u64)) as i32, 2 as u8, x_del_1);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_text16le(p_ctx_1: *mut Sqlite3Context,
    z: *const (), n: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    { let _ = 0; };
    set_result_str_or_error(p_ctx_1, z as *const i8,
        (n as u64 & !(1 as u64)) as i32, 2 as u8, x_del_1);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_text16be(p_ctx_1: *mut Sqlite3Context,
    z: *const (), n: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    { let _ = 0; };
    set_result_str_or_error(p_ctx_1, z as *const i8,
        (n as u64 & !(1 as u64)) as i32, 3 as u8, x_del_1);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_value(p_ctx_1: *mut Sqlite3Context,
    p_value_1: *const Sqlite3Value) -> () {
    let mut p_out: *mut Mem = core::ptr::null_mut();
    p_out = unsafe { (*p_ctx_1).p_out };
    { let _ = 0; };
    unsafe { sqlite3_vdbe_mem_copy(p_out, p_value_1 as *const Mem) };
    unsafe {
        sqlite3_vdbe_change_encoding(p_out, unsafe { (*p_ctx_1).enc } as i32)
    };
    if unsafe { sqlite3_vdbe_mem_too_big(p_out) } != 0 {
        unsafe { sqlite3_result_error_toobig(unsafe { &mut *p_ctx_1 }) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_pointer(p_ctx_1: &Sqlite3Context,
    p_ptr_1: *mut (), z_p_type_1: *const i8,
    x_destructor_1: Option<unsafe extern "C" fn(*mut ()) -> ()>) -> () {
    let mut p_out: *mut Mem = core::ptr::null_mut();
    p_out = (*p_ctx_1).p_out;
    { let _ = 0; };
    unsafe { sqlite3_vdbe_mem_release(p_out) };
    unsafe { (*p_out).flags = 1 as u16 };
    unsafe {
        sqlite3_vdbe_mem_set_pointer(p_out, p_ptr_1, z_p_type_1,
            x_destructor_1)
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_zeroblob64(p_ctx_1: *mut Sqlite3Context,
    n: u64) -> i32 {
    let mut p_out: *const Mem = core::ptr::null();
    p_out = unsafe { (*p_ctx_1).p_out };
    { let _ = 0; };
    if n > unsafe { (*unsafe { (*p_out).db }).a_limit[0 as usize] } as u64 {
        unsafe { sqlite3_result_error_toobig(unsafe { &mut *p_ctx_1 }) };
        return 18;
    }
    unsafe {
        sqlite3_vdbe_mem_set_zero_blob(unsafe { (*p_ctx_1).p_out }, n as i32)
    };
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_zeroblob(p_ctx_1: *mut Sqlite3Context,
    n: i32) -> () {
    unsafe {
        sqlite3_result_zeroblob64(p_ctx_1,
            if n > 0 { n } else { 0 } as Sqlite3Uint64)
    };
}

///* CAPI3REF: Setting The Subtype Of An SQL Function
///* METHOD: sqlite3_context
///*
///* The sqlite3_result_subtype(C,T) function causes the subtype of
///* the result from the [application-defined SQL function] with
///* [sqlite3_context] C to be the value T.  Only the lower 8 bits
///* of the subtype T are preserved in current versions of SQLite;
///* higher order bits are discarded.
///* The number of subtype bytes preserved by SQLite might increase
///* in future releases of SQLite.
///*
///* Every [application-defined SQL function] that invokes this interface
///* should include the [SQLITE_RESULT_SUBTYPE] property in its
///* text encoding argument when the SQL function is
///* [sqlite3_create_function|registered].  If the [SQLITE_RESULT_SUBTYPE]
///* property is omitted from the function that invokes sqlite3_result_subtype(),
///* then in some cases the sqlite3_result_subtype() might fail to set
///* the result subtype.
///*
///* If SQLite is compiled with -DSQLITE_STRICT_SUBTYPE=1, then any
///* SQL function that invokes the sqlite3_result_subtype() interface
///* and that does not have the SQLITE_RESULT_SUBTYPE property will raise
///* an error.  Future versions of SQLite might enable -DSQLITE_STRICT_SUBTYPE=1
///* by default.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_result_subtype(p_ctx_1: &Sqlite3Context,
    e_subtype_1: u32) -> () {
    let mut p_out: *mut Mem = core::ptr::null_mut();

    /// SQLITE_STRICT_SUBTYPE
    (p_out = (*p_ctx_1).p_out);
    { let _ = 0; };
    unsafe { (*p_out).e_subtype = (e_subtype_1 & 255 as u32) as u8 };
    unsafe { (*p_out).flags |= 2048 as u16 };
}

///* CAPI3REF: Find The Database Handle Of A Prepared Statement
///* METHOD: sqlite3_stmt
///*
///* ^The sqlite3_db_handle interface returns the [database connection] handle
///* to which a [prepared statement] belongs.  ^The [database connection]
///* returned by sqlite3_db_handle is the same [database connection]
///* that was the first argument
///* to the [sqlite3_prepare_v2()] call (or its variants) that was used to
///* create the statement in the first place.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_db_handle(p_stmt_1: *mut Sqlite3Stmt)
    -> *mut Sqlite3 {
    return if !(p_stmt_1).is_null() {
            unsafe { (*(p_stmt_1 as *mut Vdbe)).db }
        } else { core::ptr::null_mut() };
}

///* CAPI3REF: Find the next prepared statement
///* METHOD: sqlite3
///*
///* ^This interface returns a pointer to the next [prepared statement] after
///* pStmt associated with the [database connection] pDb.  ^If pStmt is NULL
///* then this interface returns a pointer to the first prepared statement
///* associated with the database connection pDb.  ^If no prepared statement
///* satisfies the conditions of this routine, it returns NULL.
///*
///* The [database connection] pointer D in a call to
///* [sqlite3_next_stmt(D,S)] must refer to an open database
///* connection and in particular must not be a NULL pointer.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_next_stmt(p_db_1: &mut Sqlite3,
    p_stmt_1: *mut Sqlite3Stmt) -> *mut Sqlite3Stmt {
    let mut p_next: *mut Sqlite3Stmt = core::ptr::null_mut();
    unsafe { sqlite3_mutex_enter((*p_db_1).mutex) };
    if p_stmt_1 == core::ptr::null_mut() {
        p_next = (*p_db_1).p_vdbe as *mut Sqlite3Stmt;
    } else {
        p_next =
            unsafe { (*(p_stmt_1 as *mut Vdbe)).p_v_next } as
                *mut Sqlite3Stmt;
    }
    unsafe { sqlite3_mutex_leave((*p_db_1).mutex) };
    return p_next;
}

///* CAPI3REF: Return A Dynamic String From an SQL Function
///*
///* The [sqlite3_result_str(C,S,F)] interface causes the
///* [sqlite3_str|dynamic string] S to become the return value for the
///* application-defined function or virtual table that uses
///* [sqlite3_context] C.  The F flag can be one of [SQLITE_COPY]
///* or [SQLITE_XFER] or [SQLITE_FINISH].
///*
///* If the dynamic string is invalid or incomplete due to an out-of-memory
///* or string-too-large error, then this routine transfers that error
///* over to the SQL function.
///*
///* If the F argument is SQLITE_COPY, then a copy of the dynamic string
///* content is made and the dynamic string object is unchanged.
///* If the F argument is SQLITE_XFER, then ownership of the content
///* in the dynamic is transferred to the SQL function (via a pointer copy
///* rather than a string copy) and the dynamic string is reset to an
///* empty string.  The SQLITE_FINISH value for F works like SQLITE_RESET
///* except that it also invokes the [sqlite3_str_free(S)] destructor
///* on the dynamic string object.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_result_str(p_ctx_1: *mut Sqlite3Context,
    p_str_1: *mut Sqlite3Str, e_own_1: i32) -> () {
    if unsafe { (*p_str_1).acc_error } as i32 == 0 {
        if unsafe { (*p_str_1).n_char } == 0 as u32 {
            set_result_str_or_error(p_ctx_1,
                c"".as_ptr() as *mut i8 as *const i8, 0, 16 as u8, None);
            if e_own_1 != 0 { unsafe { sqlite3_str_reset(p_str_1) }; }
        } else {
            let z_text: *const i8 =
                unsafe { sqlite3_str_value(p_str_1) } as *const i8;

            /// Only internal code has the ability to capture a pointer to
            ///* an sqlite3_str object that uses static buffer.  And none of
            ///* those internal use cases every invoke the sqlite3_result_str()
            ///* interface on a static-buffer sqlite3_str.  Should this change
            ///* in the future, the following assert() will let us know.
            { let _ = 0; };
            if e_own_1 == 0 {
                set_result_str_or_error(p_ctx_1, z_text,
                    unsafe { (*p_str_1).n_char } as i32, 1 as u8,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }));
            } else {
                set_result_str_or_error(p_ctx_1, z_text,
                    unsafe { (*p_str_1).n_char } as i32, 16 as u8,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(sqlite3_row_set_clear as *const ())
                        }));
            }
        }
    } else if unsafe { (*p_str_1).acc_error } as i32 == 7 {
        sqlite3_result_error_nomem(unsafe { &mut *p_ctx_1 });
    } else {
        { let _ = 0; };
        sqlite3_result_error_toobig(unsafe { &mut *p_ctx_1 });
    }
    if e_own_1 != 0 {
        if unsafe { (*p_str_1).acc_error } as i32 == 0 {
            unsafe {
                sqlite3_str_accum_init(p_str_1 as *mut StrAccum,
                    unsafe { (*p_str_1).db }, core::ptr::null_mut(), 0,
                    unsafe { (*p_str_1).mx_alloc } as i32)
            };
        }
        if e_own_1 == 2 { unsafe { sqlite3_str_free(p_str_1) }; }
    }
}

///* CAPI3REF: Prepared Statement Status
///* METHOD: sqlite3_stmt
///*
///* ^(Each prepared statement maintains various
///* [SQLITE_STMTSTATUS counters] that measure the number
///* of times it has performed specific operations.)^  These counters can
///* be used to monitor the performance characteristics of the prepared
///* statements.  For example, if the number of table steps greatly exceeds
///* the number of table searches or result rows, that would tend to indicate
///* that the prepared statement is using a full table scan rather than
///* an index.
///*
///* ^(This interface is used to retrieve and reset counter values from
///* a [prepared statement].  The first argument is the prepared statement
///* object to be interrogated.  The second argument
///* is an integer code for a specific [SQLITE_STMTSTATUS counter]
///* to be interrogated.)^
///* ^The current value of the requested counter is returned.
///* ^If the resetFlg is true, then the counter is reset to zero after this
///* interface call returns.
///*
///* See also: [sqlite3_status()] and [sqlite3_db_status()].
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_stmt_status(p_stmt: *mut Sqlite3Stmt, op: i32,
    reset_flag: i32) -> i32 {
    let p_vdbe: *mut Vdbe = p_stmt as *mut Vdbe;
    let mut v: u32 = 0 as u32;
    if op == 99 {
        let db: *mut Sqlite3 = unsafe { (*p_vdbe).db };
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        v = 0 as u32;
        unsafe { (*db).pn_bytes_freed = &raw mut v as *mut i32 };
        { let _ = 0; };
        unsafe { (*db).lookaside.p_end = unsafe { (*db).lookaside.p_start } };
        unsafe { sqlite3_vdbe_delete(p_vdbe) };
        unsafe { (*db).pn_bytes_freed = core::ptr::null_mut() };
        unsafe {
            (*db).lookaside.p_end = unsafe { (*db).lookaside.p_true_end }
        };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    } else {
        v = unsafe { (*p_vdbe).a_counter[op as usize] };
        if reset_flag != 0 {
            unsafe { (*p_vdbe).a_counter[op as usize] = 0 as u32 };
        }
    }
    return v as i32;
}

///* CAPI3REF: Determine If Virtual Table Column Access Is For UPDATE
///*
///* If the sqlite3_vtab_nochange(X) routine is called within the [xColumn]
///* method of a [virtual table], then it might return true if the
///* column is being fetched as part of an UPDATE operation during which the
///* column value will not change.  The virtual table implementation can use
///* this hint as permission to substitute a return value that is less
///* expensive to compute and that the corresponding
///* [xUpdate] method understands as a "no-change" value.
///*
///* If the [xColumn] method calls sqlite3_vtab_nochange() and finds that
///* the column is not changed by the UPDATE statement, then the xColumn
///* method can optionally return without setting a result, without calling
///* any of the [sqlite3_result_int|sqlite3_result_xxxxx() interfaces].
///* In that case, [sqlite3_value_nochange(X)] will return true for the
///* same column in the [xUpdate] method.
///*
///* The sqlite3_vtab_nochange() routine is an optimization.  Virtual table
///* implementations should continue to give a correct answer even if the
///* sqlite3_vtab_nochange() interface were to always return false.  In the
///* current implementation, the sqlite3_vtab_nochange() interface does always
///* returns false for the enhanced [UPDATE FROM] statement.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtab_nochange(p: &Sqlite3Context) -> i32 {
    { let _ = 0; };
    return sqlite3_value_nochange(unsafe { &*(*p).p_out });
}

///* The destructor function for a ValueList object.  This needs to be
///* a separate function, unknowable to the application, to ensure that
///* calls to sqlite3_vtab_in_first()/sqlite3_vtab_in_next() that are not
///* preceded by activation of IN processing via sqlite3_vtab_int() do not
///* try to access a fake ValueList object inserted by a hostile extension.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_value_list_free(p_to_delete: *mut ()) -> () {
    unsafe { sqlite3_free(p_to_delete) };
}

///* Implementation of sqlite3_vtab_in_first() (if bNext==0) and
///* sqlite3_vtab_in_next() (if bNext!=0).
#[allow(unused_doc_comments)]
extern "C" fn value_from_value_list(p_val_1: *mut Sqlite3Value,
    pp_out_1: &mut *mut Sqlite3Value, b_next_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut p_rhs: *const ValueList = core::ptr::null();
        *pp_out_1 = core::ptr::null_mut();
        if p_val_1 == core::ptr::null_mut() {
            return unsafe { sqlite3_misuse_error(1112) };
        }
        if unsafe { (*p_val_1).flags } as i32 & 4096 == 0 ||
                unsafe { (*p_val_1).x_del } !=
                    Some(sqlite3_vdbe_value_list_free) {
            return 1;
        } else {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            p_rhs = unsafe { (*p_val_1).z } as *mut ValueList;
        }
        if b_next_1 != 0 {
            rc = unsafe { sqlite3_btree_next(unsafe { (*p_rhs).p_csr }, 0) };
        } else {
            let mut dummy: i32 = 0;
            rc =
                unsafe {
                    sqlite3_btree_first(unsafe { (*p_rhs).p_csr }, &mut dummy)
                };
            { let _ = 0; };
            if unsafe { sqlite3_btree_eof(unsafe { (*p_rhs).p_csr }) } != 0 {
                rc = 101;
            }
        }
        if rc == 0 {
            let mut sz: u32 = 0 as u32;
            /// Size of current row in bytes
            let mut s_mem: Mem = unsafe { core::mem::zeroed() };

            /// Raw content of current row
            unsafe {
                memset(&raw mut s_mem as *mut (), 0,
                    core::mem::size_of::<Mem>() as u64)
            };
            sz =
                unsafe {
                    sqlite3_btree_payload_size(unsafe { (*p_rhs).p_csr })
                };
            rc =
                unsafe {
                    sqlite3_vdbe_mem_from_btree_zero_offset(unsafe {
                            (*p_rhs).p_csr
                        }, sz, &mut s_mem)
                };
            if rc == 0 {
                let z_buf: *mut u8 = s_mem.z as *mut u8;
                let mut i_serial: u32 = 0 as u32;
                let p_out: *mut Sqlite3Value = unsafe { (*p_rhs).p_out };
                let i_off: i32 =
                    1 +
                        if (unsafe { *unsafe { &mut *z_buf.offset(1 as isize) } } as
                                                i32) < 128 as u8 as i32 {
                                    {
                                        ({
                                                i_serial =
                                                    unsafe { *unsafe { &mut *z_buf.offset(1 as isize) } } as
                                                        u32;
                                                i_serial
                                            }) as i32;
                                        1
                                    }
                                } else {
                                    (unsafe {
                                            sqlite3_get_varint32(unsafe {
                                                        &raw mut *z_buf.offset(1 as isize)
                                                    } as *const u8, &raw mut i_serial as *mut u32)
                                        }) as i32
                                } as u8 as i32;
                unsafe {
                    sqlite3_vdbe_serial_get(unsafe {
                                &raw mut *z_buf.offset(i_off as isize)
                            } as *const u8, i_serial, p_out as *mut Mem)
                };
                unsafe {
                    (*p_out).enc = unsafe { (*unsafe { (*p_out).db }).enc }
                };
                if unsafe { (*p_out).flags } as i32 & 16384 != 0 &&
                        unsafe {
                                sqlite3_vdbe_mem_make_writeable(p_out as *mut Mem)
                            } != 0 {
                    rc = 7;
                } else { *pp_out_1 = p_out; }
            }
            unsafe { sqlite3_vdbe_mem_release(&mut s_mem) };
        }
        return rc;
    }
}

///* CAPI3REF: Find all elements on the right-hand side of an IN constraint.
///*
///* These interfaces are only useful from within the
///* [xFilter|xFilter() method] of a [virtual table] implementation.
///* The result of invoking these interfaces from any other context
///* is undefined and probably harmful.
///*
///* The X parameter in a call to sqlite3_vtab_in_first(X,P) or
///* sqlite3_vtab_in_next(X,P) should be one of the parameters to the
///* xFilter method which invokes these routines, and specifically
///* a parameter that was previously selected for all-at-once IN constraint
///* processing using the [sqlite3_vtab_in()] interface in the
///* [xBestIndex|xBestIndex method].  ^(If the X parameter is not
///* an xFilter argument that was selected for all-at-once IN constraint
///* processing, then these routines return [SQLITE_ERROR].)^
///*
///* ^(Use these routines to access all values on the right-hand side
///* of the IN constraint using code like the following:
///*
///* <blockquote><pre>
///* &nbsp;  for(rc=sqlite3_vtab_in_first(pList, &pVal);
///* &nbsp;      rc==SQLITE_OK && pVal;
///* &nbsp;      rc=sqlite3_vtab_in_next(pList, &pVal)
///* &nbsp;  ){
///* &nbsp;    // do something with pVal
///* &nbsp;  }
///* &nbsp;  if( rc!=SQLITE_DONE ){
///* &nbsp;    // an error has occurred
///* &nbsp;  }
///* </pre></blockquote>)^
///*
///* ^On success, the sqlite3_vtab_in_first(X,P) and sqlite3_vtab_in_next(X,P)
///* routines return SQLITE_OK and set *P to point to the first or next value
///* on the RHS of the IN constraint.  ^If there are no more values on the
///* right hand side of the IN constraint, then *P is set to NULL and these
///* routines return [SQLITE_DONE].  ^The return value might be
///* some other value, such as SQLITE_NOMEM, in the event of a malfunction.
///*
///* The *ppOut values returned by these routines are only valid until the
///* next call to either of these routines or until the end of the xFilter
///* method from which these routines were called.  If the virtual table
///* implementation needs to retain the *ppOut values for longer, it must make
///* copies.  The *ppOut values are [protected sqlite3_value|protected].
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtab_in_first(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value) -> i32 {
    return value_from_value_list(p_val_1, unsafe { &mut *pp_out_1 }, 0);
}

///* Set the iterator value pVal to point to the next value in the set.
///* Set (*ppOut) to point to this value before returning.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtab_in_next(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value) -> i32 {
    return value_from_value_list(p_val_1, unsafe { &mut *pp_out_1 }, 1);
}

/// Force the INT64 value currently stored as the result to be
///* a MEM_IntReal value.  See the SQLITE_TESTCTRL_RESULT_INTREAL
///* test-control.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_int_real(p_ctx_1: &Sqlite3Context) -> () {
    { let _ = 0; };
    if unsafe { (*(*p_ctx_1).p_out).flags } as i32 & 4 != 0 {
        unsafe { (*(*p_ctx_1).p_out).flags &= !4 as u16 };
        unsafe { (*(*p_ctx_1).p_out).flags |= 32 as u16 };
    }
}

///* Return the current time for a statement.  If the current time
///* is requested more than once within the same run of a single prepared
///* statement, the exact same time is returned for each invocation regardless
///* of the amount of time that elapses between invocations.  In other words,
///* the time returned is always the time of the first call.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_stmt_current_time(p: &Sqlite3Context)
    -> Sqlite3Int64 {
    let mut rc: i32 = 0;
    let pi_time: *mut Sqlite3Int64 =
        unsafe { &mut (*(*p).p_vdbe).i_current_time };
    { let _ = 0; };
    if unsafe { *pi_time } == 0 as i64 {
        rc =
            unsafe {
                sqlite3_os_current_time_int64(unsafe {
                        (*unsafe { (*(*p).p_out).db }).p_vfs
                    }, pi_time)
            };
        if rc != 0 { unsafe { *pi_time = 0 as Sqlite3Int64 }; }
    }
    return unsafe { *pi_time };
}

static a_type: [u8; 64] =
    [4 as u8, 5 as u8, 3 as u8, 5 as u8, 1 as u8, 5 as u8, 1 as u8, 5 as u8,
            2 as u8, 5 as u8, 2 as u8, 5 as u8, 1 as u8, 5 as u8, 1 as u8,
            5 as u8, 4 as u8, 5 as u8, 3 as u8, 5 as u8, 1 as u8, 5 as u8,
            1 as u8, 5 as u8, 2 as u8, 5 as u8, 2 as u8, 5 as u8, 1 as u8,
            5 as u8, 1 as u8, 5 as u8, 2 as u8, 5 as u8, 2 as u8, 5 as u8,
            2 as u8, 5 as u8, 2 as u8, 5 as u8, 2 as u8, 5 as u8, 2 as u8,
            5 as u8, 2 as u8, 5 as u8, 2 as u8, 5 as u8, 4 as u8, 5 as u8,
            3 as u8, 5 as u8, 2 as u8, 5 as u8, 2 as u8, 5 as u8, 2 as u8,
            5 as u8, 2 as u8, 5 as u8, 2 as u8, 5 as u8, 2 as u8, 5 as u8];

static mut null_mem: Mem =
    Sqlite3Value {
        u: MemValue { r: 0 as f64 },
        z: 0 as *mut i8,
        n: 0 as i32,
        flags: 1 as u16,
        enc: 0 as u8,
        e_subtype: 0 as u8,
        db: 0 as *mut Sqlite3,
        sz_malloc: 0 as i32,
        u_temp: 0 as u32,
        z_malloc: 0 as *mut i8,
        x_del: None,
    };

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
    fn sqlite3_mutex_enter(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_vdbe_expand_sql(_: *mut Vdbe, _: *const i8)
    -> *mut i8;
    fn sqlite3_mutex_leave(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_reprepare(_: *mut Vdbe)
    -> i32;
    fn sqlite3_log(i_err_code_1: i32, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_misuse_error(_: i32)
    -> i32;
    fn sqlite3_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_vdbe_mem_release(p: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_set_text(_: *mut Mem, _: *const i8, _: i64,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_mem_set_str(_: *mut Mem, _: *const i8, _: i64, _: u8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_change_encoding(_: *mut Mem, _: i32)
    -> i32;
    fn sqlite3_api_exit(db: *mut Sqlite3, _: i32)
    -> i32;
    fn sqlite3_vdbe_mem_set_double(_: *mut Mem, _: f64)
    -> ();
    fn sqlite3_vdbe_mem_set_int64(_: *mut Mem, _: i64)
    -> ();
    fn sqlite3_vdbe_mem_set_zero_blob(_: *mut Mem, _: i32)
    -> ();
    fn sqlite3_vdbe_mem_set_pointer(_: *mut Mem, _: *mut (), _: *const i8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_v_list_num_to_name(_: *mut VList, _: i32)
    -> *const i8;
    fn sqlite3_v_list_name_to_num(_: *mut VList, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_strlen30(_: *const i8)
    -> i32;
    fn sqlite3ValueText(_: *mut Sqlite3Value, _: u8)
    -> *const ();
    fn sqlite3_oom_clear(_: *mut Sqlite3)
    -> ();
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
    fn sqlite3_vdbe_transfer_error(p: *mut Vdbe)
    -> i32;
    fn sqlite3_os_current_time_int64(_: *mut Sqlite3Vfs, _: *mut Sqlite3Int64)
    -> i32;
    fn sqlite3_vdbe_reset(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_rewind(_: *mut Vdbe)
    -> ();
    fn sqlite3_vdbe_list(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_exec(_: *mut Vdbe)
    -> i32;
    fn sqlite3_btree_enter(_: *mut Btree)
    -> ();
    fn sqlite3_pager_wal_callback(p_pager_1: *mut Pager)
    -> i32;
    fn sqlite3_btree_pager(_: *mut Btree)
    -> *mut Pager;
    fn sqlite3_btree_leave(_: *mut Btree)
    -> ();
    fn sqlite3_db_free(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_str_dup(_: *mut Sqlite3, _: *const i8)
    -> *mut i8;
    fn sqlite3_vdbe_mem_expand_blob(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_real_value(_: *mut Mem)
    -> f64;
    fn sqlite3_vdbe_int_value(_: *const Mem)
    -> i64;
    fn sqlite3ValueBytes(_: *mut Sqlite3Value, _: u8)
    -> i32;
    fn sqlite3_vdbe_delete(_: *mut Vdbe)
    -> ();
    fn sqlite3_leave_mutex_and_close_zombie(_: *mut Sqlite3)
    -> ();
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
    fn sqlite3_vdbe_mem_move(_: *mut Mem, _: *mut Mem)
    -> ();
    fn sqlite3_global_recover()
    -> i32;
    fn sqlite3_thread_cleanup()
    -> ();
    fn sqlite3_memory_alarm(_:
        Option<unsafe extern "C" fn(*mut (), i64, i32) -> ()>, _: *mut (),
    _: Sqlite3Int64)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn sqlite3_value_numeric_type(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_vdbe_mem_make_writeable(_: *mut Mem)
    -> i32;
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_vdbe_mem_set_null(_: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_clear_and_resize(p_mem_1: *mut Mem, n: i32)
    -> i32;
    fn sqlite3_db_malloc_zero(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_get_clientdata(_: *mut Sqlite3, _: *const i8)
    -> *mut ();
    fn sqlite3_set_clientdata(_: *mut Sqlite3, _: *const i8, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_vdbe_mem_too_big(_: *mut Mem)
    -> i32;
    fn sqlite3_err_str(_: i32)
    -> *const i8;
    fn sqlite3_vdbe_mem_zero_terminate_if_able(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_copy(_: *mut Mem, _: *const Mem)
    -> i32;
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
    fn sqlite3_db_name(db: *mut Sqlite3, n_1: i32)
    -> *const i8;
    fn sqlite3_db_filename(db: *mut Sqlite3, z_db_name_1: *const i8)
    -> Sqlite3Filename;
    fn sqlite3_db_readonly(db: *mut Sqlite3, z_db_name_1: *const i8)
    -> i32;
    fn sqlite3_txn_state(_: *mut Sqlite3, z_schema_1: *const i8)
    -> i32;
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
    fn sqlite3_mutex_try(_: *mut Sqlite3Mutex)
    -> i32;
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
    fn sqlite3_str_reset(_: *mut Sqlite3Str)
    -> ();
    fn sqlite3_str_value(_: *mut Sqlite3Str)
    -> *mut i8;
    fn sqlite3_row_set_clear(_: *mut ())
    -> ();
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut Sqlite3, _: *mut i8,
    _: i32, _: i32)
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
    fn sqlite3_str_truncate(_: *mut Sqlite3Str, n_1: i32)
    -> ();
    fn sqlite3_str_errcode(_: *mut Sqlite3Str)
    -> i32;
    fn sqlite3_str_length(_: *mut Sqlite3Str)
    -> i32;
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
    fn sqlite3_vtab_collation(_: *mut Sqlite3IndexInfo, _: i32)
    -> *const i8;
    fn sqlite3_vtab_distinct(_: *mut Sqlite3IndexInfo)
    -> i32;
    fn sqlite3_vtab_in(_: *mut Sqlite3IndexInfo, i_cons_1: i32,
    b_handle_1: i32)
    -> i32;
    fn sqlite3_btree_next(_: *mut BtCursor, flags: i32)
    -> i32;
    fn sqlite3_btree_first(_: *mut BtCursor, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_eof(_: *mut BtCursor)
    -> i32;
    fn sqlite3_btree_payload_size(_: *mut BtCursor)
    -> u32;
    fn sqlite3_vdbe_mem_from_btree_zero_offset(_: *mut BtCursor, _: u32,
    _: *mut Mem)
    -> i32;
    fn sqlite3_get_varint32(_: *const u8, _: *mut u32)
    -> u8;
    fn sqlite3_vdbe_serial_get(_: *const u8, _: u32, _: *mut Mem)
    -> ();
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
    fn sqlite3_btree_is_empty(p_cur_1: *mut BtCursor, p_res_1: *mut i32)
    -> i32;
    fn sqlite3_btree_last(_: *mut BtCursor, p_res_1: *mut i32)
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
    fn sqlite3_btree_max_record_size(_: *mut BtCursor)
    -> Sqlite3Int64;
    fn sqlite3_btree_integrity_check(db: *mut Sqlite3, p: *mut Btree,
    a_root_1: *mut Pgno, a_cnt_1: *mut Sqlite3Value, n_root_1: i32,
    mx_err_1: i32, pn_err_1: *mut i32, pz_out_1: *mut *mut i8)
    -> i32;
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
    fn sqlite3_btree_enter_all(_: *mut Sqlite3)
    -> ();
    fn sqlite3_btree_sharable(_: *mut Btree)
    -> i32;
    fn sqlite3_btree_enter_cursor(_: *mut BtCursor)
    -> ();
    fn sqlite3_btree_connection_count(_: *mut Btree)
    -> i32;
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
    fn sqlite3_db_malloc_raw(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw_nn(_: *mut Sqlite3, _: u64)
    -> *mut ();
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
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
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
    fn sqlite3_expr_dup(_: *mut Sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
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
    fn sqlite3_put_varint(_: *mut u8, _: u64)
    -> i32;
    fn sqlite3_get_varint(_: *const u8, _: *mut u64)
    -> u8;
    fn sqlite3_varint_len(v: u64)
    -> i32;
    fn sqlite3_index_affinity_str(_: *mut Sqlite3, _: *mut Index)
    -> *const i8;
    fn sqlite3_table_affinity_str(_: *mut Sqlite3, _: *const Table)
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
    fn sqlite3_error_with_msg(_: *mut Sqlite3, _: i32, _: *const i8, ...)
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
    fn sqlite3_value_is_of_class(_: *const Sqlite3Value,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_value_set_str(_: *mut Sqlite3Value, _: i32, _: *const (),
    _: u8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_value_set_null(_: *mut Sqlite3Value)
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
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_key_info_alloc(_: *mut Sqlite3, _: i32, _: i32)
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
    fn sqlite3_get_v_table(_: *mut Sqlite3, _: *mut Table)
    -> *mut VTable;
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
    fn sqlite3_vtab_uses_all_schemas(_: *mut Parse)
    -> ();
    fn sqlite3_parse_object_init(_: *mut Parse, _: *mut Sqlite3)
    -> ();
    fn sqlite3_parse_object_reset(_: *mut Parse)
    -> ();
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
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
    fn sqlite3_vdbe_delete_aux_data(_: *mut Sqlite3, _: *mut *mut AuxData,
    _: i32, _: i32)
    -> ();
    fn sqlite2_btree_key_compare(_: *mut BtCursor, _: *const (), _: i32,
    _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_idx_key_compare(_: *mut Sqlite3, _: *mut VdbeCursor,
    _: *mut UnpackedRecord, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_idx_rowid(_: *mut Sqlite3, _: *mut BtCursor, _: *mut i64)
    -> i32;
    fn sqlite3_vdbe_next_opcode(_: *mut Vdbe, _: *mut Mem, _: i32,
    _: *mut i32, _: *mut i32, _: *mut *mut Op)
    -> i32;
    fn sqlite3_vdbe_display_p4(_: *mut Sqlite3, _: *mut Op)
    -> *mut i8;
    fn sqlite3_vdbe_halt(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_mem_shallow_copy(_: *mut Mem, _: *const Mem, _: i32)
    -> ();
    fn sqlite3_vdbe_mem_nul_terminate(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_init(_: *mut Mem, _: *mut Sqlite3, _: u16)
    -> ();
    fn sqlite3_vdbe_mem_set_row_set(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_stringify(_: *mut Mem, _: u8, _: u8)
    -> i32;
    fn sqlite3_int_float_compare(_: i64, _: f64)
    -> i32;
    fn sqlite3_vdbe_mem_integerify(_: *mut Mem)
    -> i32;
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
    fn sqlite3_vdbe_close_statement(_: *mut Vdbe, _: i32)
    -> i32;
    fn sqlite3_vdbe_frame_mem_del(_: *mut ())
    -> ();
    fn sqlite3_vdbe_frame_delete(_: *mut VdbeFrame)
    -> ();
    fn sqlite3_vdbe_frame_restore(_: *mut VdbeFrame)
    -> i32;
    fn sqlite3_vdbe_find_index_key(_: *mut BtCursor, _: *mut Index,
    _: *mut UnpackedRecord, _: *mut i32, _: i32)
    -> i32;
    fn sqlite3_vdbe_sorter_init(_: *mut Sqlite3, _: i32, _: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_sorter_reset(_: *mut Sqlite3, _: *mut VdbeSorter)
    -> ();
    fn sqlite3_vdbe_sorter_close(_: *mut Sqlite3, _: *mut VdbeCursor)
    -> ();
    fn sqlite3_vdbe_sorter_rowkey(_: *const VdbeCursor, _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_sorter_next(_: *mut Sqlite3, _: *const VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_sorter_rewind(_: *const VdbeCursor, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_sorter_write(_: *const VdbeCursor, _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_sorter_compare(_: *const VdbeCursor, _: *mut Mem, _: i32,
    _: *mut i32)
    -> i32;
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
