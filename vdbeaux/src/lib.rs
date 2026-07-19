#![feature(c_variadic)]
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
    Sqlite3Vfs, Sqlite3Vtab, Sqlite3VtabCursor,
};
use crate::sqlite_int_h::{
    AuthContext, Bft, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte, Db,
    DbFixer, Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode,
    FuncDef, FuncDefHash, FuncDestructor, IdList, Index, KeyInfo, LogEst,
    Module, NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema,
    Select, SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str,
    SrcItem, SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
    TriggerStep, UnpackedRecord, Upsert, VList, VTable, Walker, WhereInfo,
    Window, With, YDbMask, YnVar,
};
use crate::vdbe_h::{Mem, SubProgram, SubrtnSig, VdbeOp, VdbeOpList};
use crate::vdbe_int_h::{
    AuxData, Bool, Op, Sqlite3Context, Sqlite3Value, Vdbe, VdbeCursor,
    VdbeFrame, VdbeSorter, VdbeTxtBlbCache,
};

type DarwinSizeT = u64;

type DarwinIntptrT = i64;

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

///* Resize the Vdbe.aOp array so that it is at least nOp elements larger
///* than its current size. nOp is guaranteed to be less than or equal
///* to 1024/sizeof(Op).
///*
///* If an out-of-memory error occurs while resizing the array, return
///* SQLITE_NOMEM. In this case Vdbe.aOp and Vdbe.nOpAlloc remain
///* unchanged (this is so that any opcodes already allocated can be
///* correctly deallocated along with the rest of the Vdbe).
#[allow(unused_doc_comments)]
extern "C" fn grow_op_array(v: &mut Vdbe, n_op_1: i32) -> i32 {
    let mut p_new: *mut VdbeOp = core::ptr::null_mut();
    let p: *const Parse = (*v).p_parse as *const Parse;
    /// The SQLITE_TEST_REALLOC_STRESS compile-time option is designed to force
    ///* more frequent reallocs and hence provide more opportunities for
    ///* simulated OOM faults.  SQLITE_TEST_REALLOC_STRESS is generally used
    ///* during testing only.  With SQLITE_TEST_REALLOC_STRESS grow the op array
    ///* by the minimum* amount required until the size reaches 512.  Normal
    ///* operation (without SQLITE_TEST_REALLOC_STRESS) is to double the current
    ///* size of the op array or add 1KB of space, whichever is smaller.
    let n_new: Sqlite3Int64 =
        if (*v).n_op_alloc != 0 {
            2 as Sqlite3Int64 * (*v).n_op_alloc as Sqlite3Int64
        } else {
            (1024 as u64 / core::mem::size_of::<Op>() as u64) as Sqlite3Int64
        };
    { let _ = n_op_1; };
    if n_new > unsafe { (*unsafe { (*p).db }).a_limit[5 as usize] } as i64 {
        unsafe { sqlite3_oom_fault(unsafe { (*p).db }) };
        return 7;
    }
    { let _ = 0; };
    { let _ = 0; };
    p_new =
        unsafe {
                sqlite3_db_realloc(unsafe { (*p).db }, (*v).a_op as *mut (),
                    n_new as u64 * core::mem::size_of::<Op>() as u64)
            } as *mut VdbeOp;
    if !(p_new).is_null() {
        (*v).n_op_alloc =
            (unsafe {
                            sqlite3_db_malloc_size(unsafe { (*p).db },
                                p_new as *const ())
                        } as u64 / core::mem::size_of::<Op>() as u64) as i32;
        (*v).a_op = p_new as *mut Op;
    }
    return if !(p_new).is_null() { 0 } else { 7 };
}

///* Slow paths for sqlite3VdbeAddOp3() and sqlite3VdbeAddOp4Int() for the
///* unusual case when we need to increase the size of the Vdbe.aOp[] array
///* before adding the new opcode.
extern "C" fn grow_op3(p: *mut Vdbe, op: i32, p1: i32, p2: i32, p3: i32)
    -> i32 {
    { let _ = 0; };
    if grow_op_array(unsafe { &mut *p }, 1) != 0 { return 1; }
    { let _ = 0; };
    return unsafe { sqlite3_vdbe_add_op3(p, op, p1, p2, p3) };
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_add_op3(p: *mut Vdbe, op: i32, p1: i32,
    p2: i32, p3: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p_op: *mut VdbeOp = core::ptr::null_mut();
        i = unsafe { (*p).n_op };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p).n_op_alloc } <= i {
            return grow_op3(p, op, p1, p2, p3);
        }
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p).n_op };
            let __t = *__p;
            *__p += 1;
            __t
        };
        p_op =
            unsafe { unsafe { (*p).a_op.offset(i as isize) } } as *mut VdbeOp;
        { let _ = 0; };
        unsafe { (*p_op).opcode = op as u8 };
        unsafe { (*p_op).p5 = 0 as u16 };
        unsafe { (*p_op).p1 = p1 };
        unsafe { (*p_op).p2 = p2 };
        unsafe { (*p_op).p3 = p3 };
        unsafe { (*p_op).p4.p = core::ptr::null_mut() };
        unsafe { (*p_op).p4type = 0 as i8 };

        /// Replicate this logic in sqlite3VdbeAddOp4Int()
        ///* vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
        /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        ///* Replicate in sqlite3VdbeAddOp4Int()
        return i;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_add_op2(p: *mut Vdbe, op: i32, p1: i32,
    p2: i32) -> i32 {
    return unsafe { sqlite3_vdbe_add_op3(p, op, p1, p2, 0) };
}

///* Prototypes for the VDBE interface.  See comments on the implementation
///* for a description of what each of these routines does.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_create(p_parse_1: *mut Parse) -> *mut Vdbe {
    let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
    let mut p: *mut Vdbe = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_db_malloc_raw_nn(db,
                    core::mem::size_of::<Vdbe>() as u64)
            } as *mut Vdbe;
    if p == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe {
        memset(unsafe { &raw mut (*p).a_op } as *mut (), 0,
            core::mem::size_of::<Vdbe>() as u64 -
                core::mem::offset_of!(Vdbe, a_op) as u64)
    };
    unsafe { (*p).db = db };
    if !(unsafe { (*db).p_vdbe }).is_null() {
        unsafe {
            (*unsafe { (*db).p_vdbe }).pp_v_prev =
                unsafe { &mut (*p).p_v_next }
        };
    }
    unsafe { (*p).p_v_next = unsafe { (*db).p_vdbe } };
    unsafe { (*p).pp_v_prev = unsafe { &mut (*db).p_vdbe } };
    unsafe { (*db).p_vdbe = p };
    { let _ = 0; };
    unsafe { (*p).p_parse = p_parse_1 };
    unsafe { (*p_parse_1).p_vdbe = p };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { sqlite3_vdbe_add_op2(p, 8, 0, 1) };
    return p;
}

///* Return the Parse object that owns a Vdbe object.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_parser(p: &Vdbe) -> *mut Parse {
    return (*p).p_parse;
}

///* Add a new instruction to the list of instructions current in the
///* VDBE.  Return the address of the new instruction.
///*
///* Parameters:
///*
///*    p               Pointer to the VDBE
///*
///*    op              The opcode for this instruction
///*
///*    p1, p2, p3, p4  Operands
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_add_op0(p: *mut Vdbe, op: i32) -> i32 {
    return unsafe { sqlite3_vdbe_add_op3(p, op, 0, 0, 0) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_add_op1(p: *mut Vdbe, op: i32, p1: i32)
    -> i32 {
    return unsafe { sqlite3_vdbe_add_op3(p, op, p1, 0, 0) };
}

/// Generate code for an unconditional jump to instruction iDest
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_goto(p: *mut Vdbe, i_dest_1: i32) -> i32 {
    return sqlite3_vdbe_add_op3(p, 9, 0, i_dest_1, 0);
}

/// Forward references
extern "C" fn free_ephemeral_function(db: *mut Sqlite3, p_def_1: *mut FuncDef)
    -> () {
    { let _ = 0; };
    if unsafe { (*p_def_1).func_flags } & 16 as u32 != 0 as u32 {
        unsafe { sqlite3_db_nn_free_nn(db, p_def_1 as *mut ()) };
    }
}

extern "C" fn free_p4_func_ctx(db: *mut Sqlite3, p: *mut Sqlite3Context)
    -> () {
    unsafe {
        { let _ = 0; };
        free_ephemeral_function(db, unsafe { (*p).p_func });
        unsafe { sqlite3_db_nn_free_nn(db, p as *mut ()) };
    }
}

///* Delete a P4 value if necessary.
extern "C" fn free_p4_mem(db: *mut Sqlite3, p: *mut Mem) -> () {
    if unsafe { (*p).sz_malloc } != 0 {
        unsafe { sqlite3_db_free(db, unsafe { (*p).z_malloc } as *mut ()) };
    }
    unsafe { sqlite3_db_nn_free_nn(db, p as *mut ()) };
}

extern "C" fn free_p4(db: *mut Sqlite3, p4type: i32, p4: *mut ()) -> () {
    { let _ = 0; };
    '__s0:
        {
        match p4type {
            -16 => {
                {
                    free_p4_func_ctx(db, p4 as *mut Sqlite3Context);
                    break '__s0;
                }
                {
                    if !(p4).is_null() {
                        unsafe { sqlite3_db_nn_free_nn(db, p4) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_key_info_unref(p4 as *mut KeyInfo) };
                    }
                    break '__s0;
                }
                {
                    free_ephemeral_function(db, p4 as *mut FuncDef);
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3ValueFree(p4 as *mut Sqlite3Value) };
                    } else { free_p4_mem(db, p4 as *mut Mem); }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -13 => {
                {
                    if !(p4).is_null() {
                        unsafe { sqlite3_db_nn_free_nn(db, p4) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_key_info_unref(p4 as *mut KeyInfo) };
                    }
                    break '__s0;
                }
                {
                    free_ephemeral_function(db, p4 as *mut FuncDef);
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3ValueFree(p4 as *mut Sqlite3Value) };
                    } else { free_p4_mem(db, p4 as *mut Mem); }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -14 => {
                {
                    if !(p4).is_null() {
                        unsafe { sqlite3_db_nn_free_nn(db, p4) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_key_info_unref(p4 as *mut KeyInfo) };
                    }
                    break '__s0;
                }
                {
                    free_ephemeral_function(db, p4 as *mut FuncDef);
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3ValueFree(p4 as *mut Sqlite3Value) };
                    } else { free_p4_mem(db, p4 as *mut Mem); }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -7 => {
                {
                    if !(p4).is_null() {
                        unsafe { sqlite3_db_nn_free_nn(db, p4) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_key_info_unref(p4 as *mut KeyInfo) };
                    }
                    break '__s0;
                }
                {
                    free_ephemeral_function(db, p4 as *mut FuncDef);
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3ValueFree(p4 as *mut Sqlite3Value) };
                    } else { free_p4_mem(db, p4 as *mut Mem); }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -15 => {
                {
                    if !(p4).is_null() {
                        unsafe { sqlite3_db_nn_free_nn(db, p4) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_key_info_unref(p4 as *mut KeyInfo) };
                    }
                    break '__s0;
                }
                {
                    free_ephemeral_function(db, p4 as *mut FuncDef);
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3ValueFree(p4 as *mut Sqlite3Value) };
                    } else { free_p4_mem(db, p4 as *mut Mem); }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -9 => {
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_key_info_unref(p4 as *mut KeyInfo) };
                    }
                    break '__s0;
                }
                {
                    free_ephemeral_function(db, p4 as *mut FuncDef);
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3ValueFree(p4 as *mut Sqlite3Value) };
                    } else { free_p4_mem(db, p4 as *mut Mem); }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -8 => {
                {
                    free_ephemeral_function(db, p4 as *mut FuncDef);
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3ValueFree(p4 as *mut Sqlite3Value) };
                    } else { free_p4_mem(db, p4 as *mut Mem); }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -11 => {
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3ValueFree(p4 as *mut Sqlite3Value) };
                    } else { free_p4_mem(db, p4 as *mut Mem); }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -12 => {
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_vtab_unlock(p4 as *mut VTable) };
                    }
                    break '__s0;
                }
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -17 => {
                {
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                        {
                        unsafe { sqlite3_delete_table(db, p4 as *mut Table) };
                    }
                    break '__s0;
                }
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            -18 => {
                {
                    let p_sig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe { sqlite3_db_free(db, p_sig as *mut ()) };
                    break '__s0;
                }
            }
            _ => {}
        }
    }
}

///* Change the value of the P4 operand for a specific instruction.
///* This routine is useful when a large program is loaded from a
///* static array using sqlite3VdbeAddOpList but we want to make a
///* few minor changes to the program.
///*
///* If n>=0 then the P4 operand is dynamic, meaning that a copy of
///* the string is made into memory obtained from sqlite3_malloc().
///* A value of n==0 means copy bytes of zP4 up to and including the
///* first null byte.  If n>0 then copy n+1 bytes of zP4.
///*
///* Other values of n (P4_STATIC, P4_COLLSEQ etc.) indicate that zP4 points
///* to a string or structure that is guaranteed to exist for the lifetime of
///* the Vdbe. In these cases we can just copy the pointer.
///*
///* If addr<0 then change P4 on the most recently inserted instruction.
extern "C" fn vdbe_change_p4_full(p: *mut Vdbe, p_op_1: *mut Op,
    z_p4_1: *const i8, mut n: i32) -> () {
    unsafe {
        if unsafe { (*p_op_1).p4type } != 0 {
            { let _ = 0; };
            unsafe { (*p_op_1).p4type = 0 as i8 };
            unsafe { (*p_op_1).p4.p = core::ptr::null_mut() };
        }
        if n < 0 {
            unsafe {
                sqlite3_vdbe_change_p4(p,
                    unsafe { p_op_1.offset_from(unsafe { (*p).a_op }) } as i64
                        as i32, z_p4_1, n)
            };
        } else {
            if n == 0 { n = unsafe { sqlite3_strlen30(z_p4_1) }; }
            unsafe {
                (*p_op_1).p4.z =
                    unsafe {
                        sqlite3_db_str_n_dup(unsafe { (*p).db }, z_p4_1, n as u64)
                    }
            };
            unsafe { (*p_op_1).p4type = -7 as i8 };
        }
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_change_p4(p: *mut Vdbe, mut addr: i32,
    mut z_p4_1: *const i8, n: i32) -> () {
    unsafe {
        let mut p_op: *mut Op = core::ptr::null_mut();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        { let _ = 0; };
        db = unsafe { (*p).db };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*db).malloc_failed } != 0 {
            if n != -12 {
                free_p4(db, n,
                    unsafe { *(&raw mut z_p4_1 as *mut *mut i8) } as *mut ());
            }
            return;
        }
        { let _ = 0; };
        { let _ = 0; };
        if addr < 0 { addr = unsafe { (*p).n_op } - 1; }
        p_op = unsafe { unsafe { (*p).a_op.offset(addr as isize) } };
        if n >= 0 || unsafe { (*p_op).p4type } != 0 {
            vdbe_change_p4_full(p, p_op, z_p4_1, n);
            return;
        }
        if n == -3 {
            unsafe {

                /// Note: this cast is safe, because the origin data point was an int
                ///* that was cast to a (const char *).
                ((*p_op).p4.i = z_p4_1 as i64 as i32)
            };
            unsafe { (*p_op).p4type = -3 as i8 };
        } else if z_p4_1 != core::ptr::null() {
            { let _ = 0; };
            unsafe { (*p_op).p4.p = z_p4_1 as *mut () };
            unsafe { (*p_op).p4type = n as i8 };
            if n == -12 {
                unsafe { sqlite3_vtab_lock(z_p4_1 as *mut VTable) };
            }
        }
    }
}

///* Add an opcode that includes the p4 value as a pointer.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_add_op4(p: *mut Vdbe, op: i32, p1: i32,
    p2: i32, p3: i32, z_p4_1: *const i8, p4type: i32) -> i32 {
    let addr: i32 = sqlite3_vdbe_add_op3(p, op, p1, p2, p3);
    unsafe { sqlite3_vdbe_change_p4(p, addr, z_p4_1, p4type) };
    return addr;
}

/// Generate code to cause the string zStr to be loaded into
///* register iDest
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_load_string(p: *mut Vdbe, i_dest_1: i32,
    z_str_1: *const i8) -> i32 {
    return unsafe {
            sqlite3_vdbe_add_op4(p, 118, 0, i_dest_1, 0, z_str_1, 0)
        };
}

///* Generate code that initializes multiple registers to string or integer
///* constants.  The registers begin with iDest and increase consecutively.
///* One register is initialized for each characgter in zTypes[].  For each
///* "s" character in zTypes[], the register is a string if the argument is
///* not NULL, or OP_Null if the value is a null pointer.  For each "i" character
///* in zTypes[], the register is initialized to an integer.
///*
///* If the input string does not end with "X" then an OP_ResultRow instruction
///* is generated for the values inserted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vdbe_multi_load(p: *mut Vdbe, i_dest_1: i32,
    z_types_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    let mut c: i8 = 0 as i8;
    let mut z: *const i8 = core::ptr::null();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s2:
            {
            match __state {
                0 => { __state = 3; }
                2 => { (); __state = 1; }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => {
                    unsafe { ap = core::mem::transmute_copy(&__va0) };
                    __state = 6;
                }
                6 => { i = 0; __state = 8; }
                7 => {
                    sqlite3_vdbe_add_op2(p, 86, i_dest_1, i);
                    __state = 16;
                }
                8 => {
                    if { c = unsafe { *z_types_1.offset(i as isize) } as i8; c }
                                as i32 != 0 {
                        __state = 9;
                    } else { __state = 7; }
                }
                9 => {
                    if c as i32 == 's' as i32 {
                        __state = 11;
                    } else { __state = 12; }
                }
                10 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 8;
                }
                11 => {
                    z =
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap =
                                (*__ap).add((core::mem::size_of::<*const i8>() + 7) & !7);
                            *(__va_p as *const *const i8)
                        };
                    __state = 13;
                }
                12 => {
                    if c as i32 == 'i' as i32 {
                        __state = 14;
                    } else { __state = 15; }
                }
                13 => {
                    unsafe {
                        sqlite3_vdbe_add_op4(p,
                            if z == core::ptr::null() { 77 } else { 118 }, 0,
                            i_dest_1 + i, 0, z, 0)
                    };
                    __state = 10;
                }
                14 => {
                    sqlite3_vdbe_add_op2(p, 73,
                        unsafe {
                            let __ap = &mut ap;
                            let __va_p = *__ap;
                            *__ap = (*__ap).add((core::mem::size_of::<i32>() + 7) & !7);
                            *(__va_p as *const i32)
                        }, i_dest_1 + i);
                    __state = 10;
                }
                15 => { __state = 2; }
                16 => { __state = 2; }
                _ => {}
            }
        }
    }
}

///* Return the database associated with the Vdbe.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_db(v: &Vdbe) -> *mut Sqlite3 {
    return (*v).db;
}

///* Add an opcode that includes the p4 value with a P4_INT64 or
///* P4_REAL type.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_add_op4_dup8(p: *mut Vdbe, op: i32, p1: i32,
    p2: i32, p3: i32, z_p4_1: *const u8, p4type: i32) -> i32 {
    let p4copy: *mut i8 =
        unsafe {
                sqlite3_db_malloc_raw_nn(unsafe {
                        sqlite3_vdbe_db(unsafe { &*p })
                    }, 8 as u64)
            } as *mut i8;
    if !(p4copy).is_null() {
        unsafe { memcpy(p4copy as *mut (), z_p4_1 as *const (), 8 as u64) };
    }
    return sqlite3_vdbe_add_op4(p, op, p1, p2, p3, p4copy as *const i8,
            p4type);
}

extern "C" fn add_op4_int_slow(p: *mut Vdbe, op: i32, p1: i32, p2: i32,
    p3: i32, p4: i32) -> i32 {
    unsafe {
        let addr: i32 = unsafe { sqlite3_vdbe_add_op3(p, op, p1, p2, p3) };
        if unsafe { (*unsafe { (*p).db }).malloc_failed } as i32 == 0 {
            let p_op: *mut VdbeOp =
                unsafe {
                        &raw mut *unsafe { (*p).a_op.offset(addr as isize) }
                    } as *mut VdbeOp;
            unsafe { (*p_op).p4type = -3 as i8 };
            unsafe { (*p_op).p4.i = p4 };
        }
        return addr;
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_add_op4_int(p: *mut Vdbe, op: i32, p1: i32,
    p2: i32, p3: i32, p4: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p_op: *mut VdbeOp = core::ptr::null_mut();
        i = unsafe { (*p).n_op };
        if unsafe { (*p).n_op_alloc } <= i {
            return add_op4_int_slow(p, op, p1, p2, p3, p4);
        }
        {
            let __p = unsafe { &mut (*p).n_op };
            let __t = *__p;
            *__p += 1;
            __t
        };
        p_op =
            unsafe { unsafe { (*p).a_op.offset(i as isize) } } as *mut VdbeOp;
        { let _ = 0; };
        unsafe { (*p_op).opcode = op as u8 };
        unsafe { (*p_op).p5 = 0 as u16 };
        unsafe { (*p_op).p1 = p1 };
        unsafe { (*p_op).p2 = p2 };
        unsafe { (*p_op).p3 = p3 };
        unsafe { (*p_op).p4.i = p4 };
        unsafe { (*p_op).p4type = -3 as i8 };

        /// Replicate this logic in sqlite3VdbeAddOp3()
        ///* vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
        /// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        ///* Replicate in sqlite3VdbeAddOp3()
        return i;
    }
}

///* Return the address of the next instruction to be inserted.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_current_addr(p: &Vdbe) -> i32 {
    { let _ = 0; };
    return (*p).n_op;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_change_p5(p: &Vdbe, p5: u16) -> () {
    { let _ = 0; };
    if (*p).n_op > 0 {
        unsafe { (*(*p).a_op.offset(((*p).n_op - 1) as isize)).p5 = p5 };
    }
}

///* Add an OP_Function or OP_PureFunc opcode.
///*
///* The eCallCtx argument is information (typically taken from Expr.op2)
///* that describes the calling context of the function.  0 means a general
///* function call.  NC_IsCheck means called by a check constraint,
///* NC_IdxExpr means called as part of an index expression.  NC_PartIdx
///* means in the WHERE clause of a partial index.  NC_GenCol means called
///* while computing a generated column value.  0 is the usual case.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_add_function_call(p_parse_1: *mut Parse,
    p1: i32, p2: i32, p3: i32, n_arg_1: i32, p_func_1: *const FuncDef,
    e_call_ctx_1: i32) -> i32 {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut addr: i32 = 0;
        let mut p_ctx: *mut Sqlite3Context = core::ptr::null_mut();
        { let _ = 0; };
        p_ctx =
            unsafe {
                    sqlite3_db_malloc_raw_nn(unsafe { (*p_parse_1).db },
                        core::mem::offset_of!(Sqlite3Context, argv) as u64 +
                            n_arg_1 as u64 *
                                core::mem::size_of::<*mut Sqlite3Value>() as u64)
                } as *mut Sqlite3Context;
        if p_ctx == core::ptr::null_mut() {
            { let _ = 0; };
            unsafe {
                free_ephemeral_function(unsafe { (*p_parse_1).db },
                    p_func_1 as *mut FuncDef)
            };
            return 0;
        }
        unsafe { (*p_ctx).p_out = core::ptr::null_mut() };
        unsafe { (*p_ctx).p_func = p_func_1 as *mut FuncDef };
        unsafe { (*p_ctx).p_vdbe = core::ptr::null_mut() };
        unsafe { (*p_ctx).is_error = 0 };
        unsafe { (*p_ctx).argc = n_arg_1 as u16 };
        unsafe {
            (*p_ctx).i_op =
                unsafe { sqlite3_vdbe_current_addr(unsafe { &*v }) }
        };
        addr =
            sqlite3_vdbe_add_op4(v, if e_call_ctx_1 != 0 { 67 } else { 68 },
                p1, p2, p3, p_ctx as *mut i8 as *const i8, -16);
        unsafe {
            sqlite3_vdbe_change_p5(unsafe { &*v }, (e_call_ctx_1 & 46) as u16)
        };
        unsafe { sqlite3_may_abort(p_parse_1) };
        return addr;
    }
}

/// Insert the end of a co-routine
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_end_coroutine(v: *mut Vdbe, reg_yield_1: i32)
    -> () {
    sqlite3_vdbe_add_op1(v, 70, reg_yield_1);

    /// Clear the temporary register cache, thereby ensuring that each
    ///* co-routine has its own independent set of registers, because co-routines
    ///* might expect their registers to be preserved across an OP_Yield, and
    ///* that could cause problems if two or more co-routines are using the same
    ///* temporary register.
    unsafe { (*unsafe { (*v).p_parse }).n_temp_reg = 0 as u8 };
    unsafe { (*unsafe { (*v).p_parse }).n_range_reg = 0 };
}

///* Add a whole list of operations to the operation stack.  Return a
///* pointer to the first operation inserted.
///*
///* Non-zero P2 arguments to jump instructions are automatically adjusted
///* so that the jump target is relative to the first operation inserted.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_add_op_list(p: *mut Vdbe, n_op_1: i32,
    mut a_op_1: *const VdbeOpList, i_lineno_1: i32) -> *mut VdbeOp {
    unsafe {
        unsafe {
            let mut i: i32 = 0;
            let mut p_out: *mut VdbeOp = core::ptr::null_mut();
            let mut p_first: *mut VdbeOp = core::ptr::null_mut();
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*p).n_op } + n_op_1 > unsafe { (*p).n_op_alloc } &&
                    grow_op_array(unsafe { &mut *p }, n_op_1) != 0 {
                return core::ptr::null_mut();
            }
            p_first =
                {
                    p_out =
                        unsafe {
                                unsafe { (*p).a_op.offset(unsafe { (*p).n_op } as isize) }
                            } as *mut VdbeOp;
                    p_out
                };
            {
                i = 0;
                '__b3: loop {
                    if !(i < n_op_1) { break '__b3; }
                    '__c3: loop {
                        unsafe {
                            (*p_out).opcode = unsafe { (*a_op_1).opcode } as u8
                        };
                        unsafe { (*p_out).p1 = unsafe { (*a_op_1).p1 } as i32 };
                        unsafe { (*p_out).p2 = unsafe { (*a_op_1).p2 } as i32 };
                        { let _ = 0; };
                        if unsafe {
                                                *(sqlite3_opcode_property.as_ptr() as
                                                            *const u8).add(unsafe { (*a_op_1).opcode } as usize)
                                            } as i32 & 1 != 0 && unsafe { (*a_op_1).p2 } as i32 > 0 {
                            unsafe { (*p_out).p2 += unsafe { (*p).n_op } };
                        }
                        unsafe { (*p_out).p3 = unsafe { (*a_op_1).p3 } as i32 };
                        unsafe { (*p_out).p4type = 0 as i8 };
                        unsafe { (*p_out).p4.p = core::ptr::null_mut() };
                        unsafe { (*p_out).p5 = 0 as u16 };
                        { let _ = i_lineno_1; };
                        break '__c3;
                    }
                    {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut a_op_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        {
                            let __p = &mut p_out;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
            unsafe { (*p).n_op += n_op_1 };
            return p_first;
        }
    }
}

///* Add a new OP_Explain opcode.
///*
///* If the bPush flag is true, then make this opcode the parent for
///* subsequent Explains until sqlite3VdbeExplainPop() is called.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vdbe_explain(p_parse_1: &mut Parse,
    b_push_1: u8, z_fmt_1: *const i8, mut __va0: ...) -> i32 {
    let mut addr: i32 = 0;
    if (*p_parse_1).explain as i32 == 2 || 0 != 0 {
        let mut z_msg: *const i8 = core::ptr::null();
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut i_this: i32 = 0;
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_msg = unsafe { sqlite3_vm_printf((*p_parse_1).db, z_fmt_1, ap) };
        ();
        v = (*p_parse_1).p_vdbe;
        i_this = unsafe { (*v).n_op };
        addr =
            sqlite3_vdbe_add_op4(v, 190, i_this, (*p_parse_1).addr_explain, 0,
                z_msg as *const i8, -7);
        if b_push_1 != 0 { (*p_parse_1).addr_explain = i_this; }
    }
    return addr;
}

///* Return the opcode for a given address.  The address must be non-negative.
///* See sqlite3VdbeGetLastOp() to get the most recently added opcode.
///*
///* If a memory allocation error has occurred prior to the calling of this
///* routine, then a pointer to a dummy VdbeOp will be returned.  That opcode
///* is readable but not writable, though it is cast to a writable value.
///* The return of a dummy opcode allows the call to continue functioning
///* after an OOM fault without having to check to see if the return from
///* this routine is a valid pointer.  But because the dummy.opcode is 0,
///* dummy will never be written to.  This is verified by code inspection and
///* by running with Valgrind.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_get_op(p: &Vdbe, addr: i32) -> *mut VdbeOp {
    unsafe {

        /// Ignore the MSVC warning about no initializer
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*(*p).db).malloc_failed } != 0 {
            return &raw mut dummy as *mut VdbeOp;
        } else {
            return unsafe { &raw mut *(*p).a_op.offset(addr as isize) } as
                    *mut VdbeOp;
        }
    }
}

///* Return the address of the current EXPLAIN QUERY PLAN baseline.
///* 0 means "none".
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_explain_parent(p_parse_1: &Parse) -> i32 {
    let mut p_op: *const VdbeOp = core::ptr::null();
    if (*p_parse_1).addr_explain == 0 { return 0; }
    p_op =
        unsafe {
            sqlite3_vdbe_get_op(unsafe { &*(*p_parse_1).p_vdbe },
                (*p_parse_1).addr_explain)
        };
    return unsafe { (*p_op).p2 };
}

///* Pop the EXPLAIN QUERY PLAN stack one level.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_explain_pop(p_parse_1: *mut Parse) -> () {
    unsafe {
        (*p_parse_1).addr_explain =
            sqlite3_vdbe_explain_parent(unsafe { &*p_parse_1 })
    };
}

///* Declare to the Vdbe that the BTree object at db->aDb[i] is used.
///*
///* The prepared statements need to know in advance the complete set of
///* attached databases that will be use.  A mask of these databases
///* is maintained in p->btreeMask.  The p->lockMask value is the subset of
///* p->btreeMask of databases that will require a lock.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_uses_btree(p: &mut Vdbe, i: i32) -> () {
    { let _ = 0; };
    { let _ = 0; };
    (*p).btree_mask |= (1 as YDbMask) << i;
    if i != 1 &&
            unsafe {
                    sqlite3_btree_sharable(unsafe {
                            (*unsafe { (*(*p).db).a_db.offset(i as isize) }).p_bt
                        })
                } != 0 {
        (*p).lock_mask |= (1 as YDbMask) << i;
    }
}

///* Add an OP_ParseSchema opcode.  This routine is broken out from
///* sqlite3VdbeAddOp4() since it needs to also needs to mark all btrees
///* as having been used.
///*
///* zWhere is a WHERE clause that defines which entries of the schema
///* to reparse.  If zWhere==0, that means all entries.  p5 is a mask
///* of INITFLAG_* values for the parse.
///*
///* In the current usage, the following are always true:
///*
///*     ALTER TABLE:     zWhere==0,  p5!=0
///*     Otherwise:       zWhere!=0,  p5==0
///*
///* The zWhere string must have been obtained from sqlite3DbMalloc().
///* This routine will take ownership of the allocated memory.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_add_parse_schema_op(p: *mut Vdbe, i_db_1: i32,
    z_where_1: *const i8, p5: u16) -> () {
    let mut j: i32 = 0;
    { let _ = 0; };
    sqlite3_vdbe_add_op4(p, 151, i_db_1, 0, 0, z_where_1 as *const i8, -7);
    unsafe { sqlite3_vdbe_change_p5(unsafe { &*p }, p5) };
    {
        j = 0;
        '__b4: loop {
            if !(j < unsafe { (*unsafe { (*p).db }).n_db }) { break '__b4; }
            '__c4: loop {
                unsafe { sqlite3_vdbe_uses_btree(unsafe { &mut *p }, j) };
                break '__c4;
            }
            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_may_abort(unsafe { (*p).p_parse }) };
}

///* Change the value of the opcode, or P1, P2, P3, or P5 operands
///* for a specific instruction.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_change_opcode(p: *mut Vdbe, addr: i32,
    i_new_opcode_1: u8) -> () {
    { let _ = 0; };
    unsafe {
        (*unsafe { sqlite3_vdbe_get_op(unsafe { &*p }, addr) }).opcode =
            i_new_opcode_1
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_change_p1(p: *mut Vdbe, addr: i32, val: i32)
    -> () {
    { let _ = 0; };
    unsafe {
        (*unsafe { sqlite3_vdbe_get_op(unsafe { &*p }, addr) }).p1 = val
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_change_p2(p: *mut Vdbe, addr: i32, val: i32)
    -> () {
    { let _ = 0; };
    unsafe {
        (*unsafe { sqlite3_vdbe_get_op(unsafe { &*p }, addr) }).p2 = val
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_change_p3(p: *mut Vdbe, addr: i32, val: i32)
    -> () {
    { let _ = 0; };
    unsafe {
        (*unsafe { sqlite3_vdbe_get_op(unsafe { &*p }, addr) }).p3 = val
    };
}

/// Return the most recently added opcode
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_get_last_op(p: *mut Vdbe) -> *mut VdbeOp {
    return sqlite3_vdbe_get_op(unsafe { &*p }, unsafe { (*p).n_op } - 1);
}

///* If the previous opcode is an OP_Column that delivers results
///* into register iDest, then add the OPFLAG_TYPEOFARG flag to that
///* opcode.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_typeof_column(p: *mut Vdbe, i_dest_1: i32)
    -> () {
    let p_op: *mut VdbeOp = unsafe { sqlite3_vdbe_get_last_op(p) };
    if unsafe { (*p_op).p3 } == i_dest_1 &&
            unsafe { (*p_op).opcode } as i32 == 96 {
        unsafe { (*p_op).p5 |= 128 as u16 };
    }
}

///* Change the P2 operand of instruction addr so that it points to
///* the address of the next instruction to be coded.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_jump_here(p: *mut Vdbe, addr: i32) -> () {
    sqlite3_vdbe_change_p2(p, addr, unsafe { (*p).n_op });
}

///* Change the P2 operand of the jump instruction at addr so that
///* the jump lands on the next opcode.  Or if the jump instruction was
///* the previous opcode (and is thus a no-op) then simply back up
///* the next instruction counter by one slot so that the jump is
///* overwritten by the next inserted opcode.
///*
///* This routine is an optimization of sqlite3VdbeJumpHere() that
///* strives to omit useless byte-code like this:
///*
///*        7   Once 0 8 0
///*        8   ...
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_jump_here_or_pop_inst(p: *mut Vdbe, addr: i32)
    -> () {
    if addr == unsafe { (*p).n_op } - 1 {
        { let _ = 0; };
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p).n_op };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    } else { sqlite3_vdbe_change_p2(p, addr, unsafe { (*p).n_op }); }
}

///* Change the opcode at addr into OP_Noop
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_change_to_noop(p: &Vdbe, addr: i32) -> i32 {
    unsafe {
        let mut p_op: *mut VdbeOp = core::ptr::null_mut();
        if unsafe { (*(*p).db).malloc_failed } != 0 { return 0; }
        { let _ = 0; };
        p_op = unsafe { (*p).a_op.offset(addr as isize) } as *mut VdbeOp;
        free_p4((*p).db, unsafe { (*p_op).p4type } as i32,
            unsafe { (*p_op).p4.p });
        unsafe { (*p_op).p4type = 0 as i8 };
        unsafe { (*p_op).p4.z = core::ptr::null_mut() };
        unsafe { (*p_op).opcode = 189 as u8 };
        return 1;
    }
}

///* If the last opcode is "op" and it is not a jump destination,
///* then remove it.  Return true if and only if an opcode was removed.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_delete_prior_opcode(p: *mut Vdbe, op: u8)
    -> i32 {
    if unsafe { (*p).n_op } > 0 &&
            unsafe {
                        (*unsafe {
                                    (*p).a_op.offset((unsafe { (*p).n_op } - 1) as isize)
                                }).opcode
                    } as i32 == op as i32 {
        return sqlite3_vdbe_change_to_noop(unsafe { &*p },
                unsafe { (*p).n_op } - 1);
    } else { return 0; }
}

///* Change the P4 operand of the most recently coded instruction
///* to the value defined by the arguments.  This is a high-speed
///* version of sqlite3VdbeChangeP4().
///*
///* The P4 operand must not have been previously defined.  And the new
///* P4 must not be P4_INT32.  Use sqlite3VdbeChangeP4() in either of
///* those cases.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_append_p4(p: &Vdbe, p_p4_1: *mut (), n: i32)
    -> () {
    unsafe {
        let mut p_op: *mut VdbeOp = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*(*p).db).malloc_failed } != 0 {
            free_p4((*p).db, n, p_p4_1);
        } else {
            { let _ = 0; };
            { let _ = 0; };
            p_op =
                unsafe { (*p).a_op.offset(((*p).n_op - 1) as isize) } as
                    *mut VdbeOp;
            { let _ = 0; };
            unsafe { (*p_op).p4type = n as i8 };
            unsafe { (*p_op).p4.p = p_p4_1 };
        }
    }
}

///* Set the P4 on the most recently added opcode to the KeyInfo for the
///* index given.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_set_p4_key_info(p_parse_1: *mut Parse,
    p_idx_1: *mut Index) -> () {
    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
    let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    p_key_info = unsafe { sqlite3_key_info_of_index(p_parse_1, p_idx_1) };
    if !(p_key_info).is_null() {
        sqlite3_vdbe_append_p4(unsafe { &*v }, p_key_info as *mut (), -9);
    }
}

///* Create a new symbolic label for an instruction that has yet to be
///* coded.  The symbolic label is really just a negative number.  The
///* label can be used as the P2 value of an operation.  Later, when
///* the label is resolved to a specific address, the VDBE will scan
///* through its operation list and change all values of P2 which match
///* the label into the resolved address.
///*
///* The VDBE knows that a P2 value is a label because labels are
///* always negative and P2 values are suppose to be non-negative.
///* Hence, a negative P2 value is a label that has yet to be resolved.
///* (Later:) This is only true for opcodes that have the OPFLG_JUMP
///* property.
///*
///* Variable usage notes:
///*
///*     Parse.aLabel[x]     Stores the address that the x-th label resolves
///*                         into.  For testing (SQLITE_DEBUG), unresolved
///*                         labels stores -1, but that is not required.
///*     Parse.nLabelAlloc   Number of slots allocated to Parse.aLabel[]
///*     Parse.nLabel        The *negative* of the number of labels that have
///*                         been issued.  The negative is stored because
///*                         that gives a performance improvement over storing
///*                         the equivalent positive value.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_make_label(p_parse_1: &mut Parse) -> i32 {
    return { let __p = &mut (*p_parse_1).n_label; *__p -= 1; *__p };
}

///* Mark the VDBE as one that can only be run one time.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_run_only_once(p: *mut Vdbe) -> () {
    sqlite3_vdbe_add_op2(p, 168, 1, 1);
}

///* Mark the VDBE as one that can be run multiple times.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_reusable(p: &Vdbe) -> () {
    let mut i: i32 = 0;
    {
        i = 1;
        '__b5: loop {
            if !(i < (*p).n_op) { break '__b5; }
            '__c5: loop {
                if unsafe { (*(*p).a_op.offset(i as isize)).opcode } as i32 ==
                        168 {
                    unsafe {
                        (*(*p).a_op.offset(1 as isize)).opcode = 189 as u8
                    };
                    break '__b5;
                }
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Release auxiliary memory held in an array of N Mem elements.
///*
///* After this routine returns, all Mem elements in the array will still
///* be valid.  Those Mem elements that were not holding auxiliary resources
///* will be unchanged.  Mem elements which had something freed will be
///* set to MEM_Undefined.
extern "C" fn release_mem_array(mut p: *mut Mem, n_1: i32) -> () {
    if !(p).is_null() && n_1 != 0 {
        let p_end: *mut Mem = unsafe { &mut *p.offset(n_1 as isize) };
        let db: *mut Sqlite3 = unsafe { (*p).db };
        { let _ = 0; };
        if !(unsafe { (*db).pn_bytes_freed }).is_null() {
            '__b6: loop {
                '__c6: loop {
                    if unsafe { (*p).sz_malloc } != 0 {
                        unsafe {
                            sqlite3_db_free(db, unsafe { (*p).z_malloc } as *mut ())
                        };
                    }
                    break '__c6;
                }
                if !({
                                    let __p = &mut p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    *__p
                                } < p_end) {
                    break '__b6;
                }
            }
            return;
        }
        '__b7: loop {
            '__c7: loop {
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p).flags } as i32 & (32768 | 4096) != 0 {
                    unsafe { sqlite3_vdbe_mem_release(p) };
                    unsafe { (*p).flags = 0 as u16 };
                } else if unsafe { (*p).sz_malloc } != 0 {
                    unsafe {
                        sqlite3_db_nn_free_nn(db,
                            unsafe { (*p).z_malloc } as *mut ())
                    };
                    unsafe { (*p).sz_malloc = 0 };
                    unsafe { (*p).flags = 0 as u16 };
                }
                break '__c7;
            }
            if !({
                                let __p = &mut p;
                                *__p = unsafe { (*__p).offset(1) };
                                *__p
                            } < p_end) {
                break '__b7;
            }
        }
    }
}

///* Free the space allocated for aOp and any p4 values allocated for the
///* opcodes contained within. If aOp is not NULL it is assumed to contain
///* nOp entries.
extern "C" fn vdbe_free_op_array(db: *mut Sqlite3, a_op_1: *mut Op,
    n_op_1: i32) -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if !(a_op_1).is_null() {
            let mut p_op: *mut Op =
                unsafe { &mut *a_op_1.offset((n_op_1 - 1) as isize) };
            loop {
                if unsafe { (*p_op).p4type } as i32 <= -7 {
                    free_p4(db, unsafe { (*p_op).p4type } as i32,
                        unsafe { (*p_op).p4.p });
                }
                if p_op == a_op_1 { break; }
                {
                    let __p = &mut p_op;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(-1) };
                    __t
                };
            }
            unsafe { sqlite3_db_nn_free_nn(db, a_op_1 as *mut ()) };
        }
    }
}

///* Free all memory associated with the Vdbe passed as the second argument,
///* except for object itself, which is preserved.
///*
///* The difference between this function and sqlite3VdbeDelete() is that
///* VdbeDelete() also unlinks the Vdbe from the list of VMs associated with
///* the database connection and frees the object itself.
extern "C" fn sqlite3_vdbe_clear_object(db: *mut Sqlite3, p: &Vdbe) -> () {
    unsafe {
        let mut p_sub: *mut SubProgram = core::ptr::null_mut();
        let mut p_next: *mut SubProgram = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        if !((*p).a_col_name).is_null() {
            release_mem_array((*p).a_col_name, (*p).n_res_alloc as i32 * 2);
            unsafe { sqlite3_db_nn_free_nn(db, (*p).a_col_name as *mut ()) };
        }
        {
            p_sub = (*p).p_program;
            '__b9: loop {
                if !(!(p_sub).is_null()) { break '__b9; }
                '__c9: loop {
                    p_next = unsafe { (*p_sub).p_next };
                    vdbe_free_op_array(db, unsafe { (*p_sub).a_op } as *mut Op,
                        unsafe { (*p_sub).n_op });
                    unsafe { sqlite3_db_free(db, p_sub as *mut ()) };
                    break '__c9;
                }
                p_sub = p_next;
            }
        }
        if (*p).e_vdbe_state as i32 != 0 {
            release_mem_array((*p).a_var, (*p).n_var as i32);
            if !((*p).p_v_list).is_null() {
                unsafe {
                    sqlite3_db_nn_free_nn(db, (*p).p_v_list as *mut ())
                };
            }
            if !((*p).p_free).is_null() {
                unsafe { sqlite3_db_nn_free_nn(db, (*p).p_free) };
            }
        }
        vdbe_free_op_array(db, (*p).a_op, (*p).n_op);
        if !((*p).z_sql).is_null() {
            unsafe { sqlite3_db_nn_free_nn(db, (*p).z_sql as *mut ()) };
        }
    }
}

///* Delete an entire VDBE.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_delete(p: *mut Vdbe) -> () {
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    { let _ = 0; };
    db = unsafe { (*p).db };
    { let _ = 0; };
    { let _ = 0; };
    sqlite3_vdbe_clear_object(db, unsafe { &*p });
    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut() {
        { let _ = 0; };
        unsafe { *unsafe { (*p).pp_v_prev } = unsafe { (*p).p_v_next } };
        if !(unsafe { (*p).p_v_next }).is_null() {
            unsafe {
                (*unsafe { (*p).p_v_next }).pp_v_prev =
                    unsafe { (*p).pp_v_prev }
            };
        }
    }
    unsafe { sqlite3_db_nn_free_nn(db, p as *mut ()) };
}

/// An instance of this object describes bulk memory available for use
///* by subcomponents of a prepared statement.  Space is allocated out
///* of a ReusableSpace object by the allocSpace() routine below.
#[repr(C)]
#[derive(Copy, Clone)]
struct ReusableSpace {
    p_space: *mut u8,
    n_free: Sqlite3Int64,
    n_needed: Sqlite3Int64,
}

///* This routine is called after all opcodes have been inserted.  It loops
///* through all the opcodes and fixes up some details.
///*
///* (1) For each jump instruction with a negative P2 value (a label)
///*     resolve the P2 value to an actual address.
///*
///* (2) Compute the maximum number of arguments used by the xUpdate/xFilter
///*     methods of any virtual table and store that value in *pMaxVtabArgs.
///*
///* (3) Update the Vdbe.readOnly and Vdbe.bIsReader flags to accurately
///*     indicate what the prepared statement actually does.
///*
///* (4) (discontinued)
///*
///* (5) Reclaim the memory allocated for storing labels.
///*
///* This routine will only function correctly if the mkopcodeh.tcl generator
///* script numbers the opcodes correctly.  Changes to this routine must be
///* coordinated with changes to mkopcodeh.tcl.
#[allow(unused_doc_comments)]
extern "C" fn resolve_p2_values(p: &mut Vdbe, p_max_vtab_args_1: &mut i32)
    -> () {
    let mut n_max_vtab_args: i32 = 0;
    let mut p_op: *mut Op = core::ptr::null_mut();
    let mut p_parse: *mut Parse = core::ptr::null_mut();
    let mut a_label: *const i32 = core::ptr::null();
    /// tag-20230419-1
    /// Loop terminates when it reaches the OP_Init opcode
    /// Only JUMP opcodes and the short list of special opcodes in the switch
    ///* below need to be considered.  The mkopcodeh.tcl generator script groups
    ///* all these opcodes together near the front of the opcode list.  Skip
    ///* any opcode that does not need processing by virtual of the fact that
    ///* it is larger than SQLITE_MX_JUMP_OPCODE, as a performance optimization.
    /// NOTE: Be sure to update mkopcodeh.tcl when adding or removing
    ///* cases from this switch!
    /// no break
    let mut n: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s11:
            {
            match __state {
                0 => { n_max_vtab_args = *p_max_vtab_args_1; __state = 3; }
                2 => {
                    if !(a_label).is_null() {
                        __state = 62;
                    } else { __state = 61; }
                }
                3 => { __state = 4; }
                4 => { p_parse = (*p).p_parse; __state = 5; }
                5 => {
                    a_label = unsafe { (*p_parse).a_label } as *const i32;
                    __state = 6;
                }
                6 => { { let _ = 0; }; __state = 7; }
                7 => { (*p).set_read_only(1 as Bft as u32); __state = 8; }
                8 => { (*p).set_b_is_reader(0 as Bft as u32); __state = 9; }
                9 => {
                    p_op =
                        unsafe { (*p).a_op.offset(((*p).n_op - 1) as isize) };
                    __state = 10;
                }
                10 => { { let _ = 0; }; __state = 11; }
                11 => { if 1 != 0 { __state = 13; } else { __state = 12; } }
                12 => { __state = 2; }
                13 => {
                    if unsafe { (*p_op).opcode } as i32 <= 66 {
                        __state = 15;
                    } else { __state = 14; }
                }
                14 => { { let _ = 0; }; __state = 60; }
                15 => {
                    '__s12:
                        {
                        match unsafe { (*p_op).opcode } {
                            2 => { __state = 17; }
                            1 => { __state = 18; }
                            0 => { __state = 19; }
                            3 => { __state = 20; }
                            5 => { __state = 21; }
                            4 => { __state = 22; }
                            8 => { __state = 23; }
                            7 => { __state = 24; }
                            6 => { __state = 25; }
                            _ => { __state = 26; }
                        }
                    }
                }
                16 => { { let _ = 0; }; __state = 14; }
                17 => {
                    if unsafe { (*p_op).p2 } != 0 {
                        __state = 30;
                    } else { __state = 29; }
                }
                18 => { __state = 19; }
                19 => { (*p).set_b_is_reader(1 as Bft as u32); __state = 33; }
                20 => { __state = 21; }
                21 => { __state = 22; }
                22 => { (*p).set_read_only(0 as Bft as u32); __state = 36; }
                23 => { { let _ = 0; }; __state = 40; }
                24 => {
                    if unsafe { (*p_op).p2 } > n_max_vtab_args {
                        __state = 43;
                    } else { __state = 42; }
                }
                25 => { __state = 45; }
                26 => {
                    if unsafe { (*p_op).p2 } < 0 {
                        __state = 54;
                    } else { __state = 53; }
                }
                27 => { __state = 17; }
                28 => { __state = 31; }
                29 => { __state = 28; }
                30 => { (*p).set_read_only(0 as Bft as u32); __state = 29; }
                31 => { __state = 18; }
                32 => { __state = 34; }
                33 => { __state = 16; }
                34 => { __state = 20; }
                35 => { __state = 38; }
                36 => { (*p).set_b_is_reader(1 as Bft as u32); __state = 37; }
                37 => { __state = 16; }
                38 => { __state = 23; }
                39 => { __state = 24; }
                40 => { __state = 2; }
                41 => { __state = 25; }
                42 => { __state = 16; }
                43 => {
                    n_max_vtab_args = unsafe { (*p_op).p2 };
                    __state = 42;
                }
                44 => { __state = 52; }
                45 => { { let _ = 0; }; __state = 46; }
                46 => { { let _ = 0; }; __state = 47; }
                47 => { { let _ = 0; }; __state = 48; }
                48 => {
                    n = unsafe { (*p_op.offset(-1 as isize)).p1 };
                    __state = 49;
                }
                49 => {
                    if n > n_max_vtab_args {
                        __state = 51;
                    } else { __state = 50; }
                }
                50 => { __state = 44; }
                51 => { n_max_vtab_args = n; __state = 50; }
                52 => { __state = 26; }
                53 => { { let _ = 0; }; __state = 58; }
                54 => { { let _ = 0; }; __state = 55; }
                55 => { { let _ = 0; }; __state = 56; }
                56 => { { let _ = 0; }; __state = 57; }
                57 => {
                    unsafe {
                        (*p_op).p2 =
                            unsafe { *a_label.offset(!unsafe { (*p_op).p2 } as isize) }
                    };
                    __state = 53;
                }
                58 => { { let _ = 0; }; __state = 59; }
                59 => { __state = 16; }
                60 => {
                    {
                        let __p = &mut p_op;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(-1) };
                        __t
                    };
                    __state = 11;
                }
                61 => { unsafe { (*p_parse).n_label = 0 }; __state = 64; }
                62 => {
                    unsafe {
                        sqlite3_db_nn_free_nn((*p).db,
                            unsafe { (*p_parse).a_label } as *mut ())
                    };
                    __state = 63;
                }
                63 => {
                    unsafe { (*p_parse).a_label = core::ptr::null_mut() };
                    __state = 61;
                }
                64 => { *p_max_vtab_args_1 = n_max_vtab_args; __state = 65; }
                65 => { { let _ = 0; }; __state = 1; }
                _ => {}
            }
        }
    }
}

/// Try to allocate nByte bytes of 8-byte aligned bulk memory for pBuf
///* from the ReusableSpace object.  Return a pointer to the allocated
///* memory on success.  If insufficient memory is available in the
///* ReusableSpace object, increase the ReusableSpace.nNeeded
///* value by the amount needed and return NULL.
///*
///* If pBuf is not initially NULL, that means that the memory has already
///* been allocated by a prior call to this routine, so just return a copy
///* of pBuf and leave ReusableSpace unchanged.
///*
///* This allocator is employed to repurpose unused slots at the end of the
///* opcode array of prepared state for other memory needs of the prepared
///* statement.
extern "C" fn alloc_space(p: &mut ReusableSpace, mut p_buf_1: *mut (),
    mut n_byte_1: Sqlite3Int64) -> *mut () {
    { let _ = 0; };
    if p_buf_1 == core::ptr::null_mut() {
        n_byte_1 = n_byte_1;
        if n_byte_1 <= (*p).n_free {
            (*p).n_free -= n_byte_1;
            p_buf_1 =
                unsafe { (*p).p_space.offset((*p).n_free as isize) } as
                    *mut ();
        } else { (*p).n_needed += n_byte_1; }
    }
    { let _ = 0; };
    return p_buf_1;
}

///* Initialize an array of N Mem element.
///*
///* This is a high-runner, so only those fields that really do need to
///* be initialized are set.  The Mem structure is organized so that
///* the fields that get initialized are nearby and hopefully on the same
///* cache line.
///*
///*    Mem.flags = flags
///*    Mem.db = db
///*    Mem.szMalloc = 0
///*
///* All other fields of Mem can safely remain uninitialized for now.  They
///* will be initialized before use.
extern "C" fn init_mem_array(mut p: *mut Mem, mut n_1: i32, db: *mut Sqlite3,
    flags: u16) -> () {
    { let _ = 0; };
    if n_1 > 0 {
        '__b13: loop {
            '__c13: loop {
                unsafe { (*p).flags = flags };
                unsafe { (*p).db = db };
                unsafe { (*p).sz_malloc = 0 };
                {
                    let __p = &mut p;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                break '__c13;
            }
            if !({ let __p = &mut n_1; *__p -= 1; *__p } > 0) {
                break '__b13;
            }
        }
    }
}

///* Rewind the VDBE back to the beginning in preparation for
///* running it.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_rewind(p: &mut Vdbe) -> () {
    { let _ = 0; };
    { let _ = 0; };

    /// There should be at least one opcode.
    { let _ = 0; };
    (*p).e_vdbe_state = 1 as u8;
    (*p).pc = -1;
    (*p).rc = 0;
    (*p).error_action = 2 as u8;
    (*p).n_change = 0 as i64;
    (*p).cache_ctr = 1 as u32;
    (*p).min_write_file_format = 255 as u8;
    (*p).i_statement = 0;
    (*p).n_fk_constraint = 0 as i64;
}

///* Prepare a virtual machine for execution for the first time after
///* creating the virtual machine.  This involves things such
///* as allocating registers and initializing the program counter.
///* After the VDBE has be prepped, it can be executed by one or more
///* calls to sqlite3VdbeExec(). 
///*
///* This function may be called exactly once on each virtual machine.
///* After this routine is called the VM has been "packaged" and is ready
///* to run.  After this routine is called, further calls to
///* sqlite3VdbeAddOp() functions are prohibited.  This routine disconnects
///* the Vdbe from the Parse object that helped generate it so that the
///* the Vdbe becomes an independent entity and the Parse object can be
///* destroyed.
///*
///* Use the sqlite3VdbeRewind() procedure to restore a virtual machine back
///* to its initial state after it has been run.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_make_ready(p: *mut Vdbe, p_parse_1: &mut Parse)
    -> () {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// The database connection
        let mut n_var: i32 = 0;
        /// Number of parameters
        let mut n_mem: i32 = 0;
        /// Number of VM memory registers
        let mut n_cursor: i32 = 0;
        /// Number of cursors required
        let mut n_arg: i32 = 0;
        /// Max number args to xFilter or xUpdate
        let mut n: i32 = 0;
        /// Loop counter
        let mut x: ReusableSpace = unsafe { core::mem::zeroed() };

        /// Reusable bulk memory
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p).p_v_list = (*p_parse_1).p_v_list };
        (*p_parse_1).p_v_list = core::ptr::null_mut();
        db = unsafe { (*p).db };
        { let _ = 0; };
        n_var = (*p_parse_1).n_var as i32;
        n_mem = (*p_parse_1).n_mem;
        n_cursor = (*p_parse_1).n_tab;
        n_arg = (*p_parse_1).n_max_arg;

        /// Each cursor uses a memory cell.  The first cursor (cursor 0) can
        ///* use aMem[0] which is not otherwise used by the VDBE program.  Allocate
        ///* space at the end of aMem[] for cursors 1 and greater.
        ///* See also: allocateCursor().
        (n_mem += n_cursor);
        if n_cursor == 0 && n_mem > 0 {
            { let __p = &mut n_mem; let __t = *__p; *__p += 1; __t };
        }

        /// Space for aMem[0] even if not used
        /// Figure out how much reusable memory is available at the end of the
        ///* opcode array.  This extra memory will be reallocated for other elements
        ///* of the prepared statement.
        (n =
            (core::mem::size_of::<Op>() as u64 * unsafe { (*p).n_op } as u64)
                as i32);

        /// Bytes of opcode memory used
        (x.p_space =
            unsafe { (unsafe { (*p).a_op } as *mut u8).offset(n as isize) });

        /// Unused opcode memory
        { let _ = 0; };
        x.n_free =
            ((unsafe { (*p).n_op_alloc } - unsafe { (*p).n_op }) as u64 *
                        core::mem::size_of::<Op>() as u64 & !7 as u64) as
                Sqlite3Int64;

        /// Bytes unused mem
        { let _ = 0; };
        { let _ = 0; };
        resolve_p2_values(unsafe { &mut *p }, &mut n_arg);
        unsafe {
            (*p).set_uses_stmt_journal(((*p_parse_1).is_multi_write != 0 &&
                                (*p_parse_1).may_abort() != 0) as u8 as Bft as u32)
        };
        if (*p_parse_1).explain != 0 {
            if n_mem < 10 { n_mem = 10; }
            unsafe { (*p).set_explain((*p_parse_1).explain as Bft as u32) };
            unsafe {
                (*p).n_res_column =
                    (12 - 4 * unsafe { (*p).explain() } as i32) as u16
            };
        }
        unsafe { (*p).set_expired(0 as Bft as u32) };

        /// Memory for registers, parameters, cursor, etc, is allocated in one or two
        ///* passes.  On the first pass, we try to reuse unused memory at the
        ///* end of the opcode array.  If we are unable to satisfy all memory
        ///* requirements by reusing the opcode array tail, then the second
        ///* pass will fill in the remainder using a fresh memory allocation. 
        ///*
        ///* This two-pass approach that reuses as much memory as possible from
        ///* the leftover memory at the end of the opcode array.  This can significantly
        ///* reduce the amount of memory held by a prepared statement.
        (x.n_needed = 0 as Sqlite3Int64);
        unsafe {
            (*p).a_mem =
                alloc_space(&mut x, core::ptr::null_mut(),
                        (n_mem as u64 * core::mem::size_of::<Mem>() as u64) as
                            Sqlite3Int64) as *mut Mem
        };
        unsafe {
            (*p).a_var =
                alloc_space(&mut x, core::ptr::null_mut(),
                        (n_var as u64 * core::mem::size_of::<Mem>() as u64) as
                            Sqlite3Int64) as *mut Mem
        };
        unsafe {
            (*p).ap_arg =
                alloc_space(&mut x, core::ptr::null_mut(),
                        (n_arg as u64 * core::mem::size_of::<*mut Mem>() as u64) as
                            Sqlite3Int64) as *mut *mut Mem
        };
        unsafe {
            (*p).ap_csr =
                alloc_space(&mut x, core::ptr::null_mut(),
                        (n_cursor as u64 *
                                core::mem::size_of::<*mut VdbeCursor>() as u64) as
                            Sqlite3Int64) as *mut *mut VdbeCursor
        };
        if x.n_needed != 0 {
            x.p_space =
                {
                        unsafe {
                            (*p).p_free =
                                unsafe { sqlite3_db_malloc_raw_nn(db, x.n_needed as u64) }
                        };
                        unsafe { (*p).p_free }
                    } as *mut u8;
            x.n_free = x.n_needed;
            if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                unsafe {
                    (*p).a_mem =
                        alloc_space(&mut x, unsafe { (*p).a_mem } as *mut (),
                                (n_mem as u64 * core::mem::size_of::<Mem>() as u64) as
                                    Sqlite3Int64) as *mut Mem
                };
                unsafe {
                    (*p).a_var =
                        alloc_space(&mut x, unsafe { (*p).a_var } as *mut (),
                                (n_var as u64 * core::mem::size_of::<Mem>() as u64) as
                                    Sqlite3Int64) as *mut Mem
                };
                unsafe {
                    (*p).ap_arg =
                        alloc_space(&mut x, unsafe { (*p).ap_arg } as *mut (),
                                (n_arg as u64 * core::mem::size_of::<*mut Mem>() as u64) as
                                    Sqlite3Int64) as *mut *mut Mem
                };
                unsafe {
                    (*p).ap_csr =
                        alloc_space(&mut x, unsafe { (*p).ap_csr } as *mut (),
                                (n_cursor as u64 *
                                        core::mem::size_of::<*mut VdbeCursor>() as u64) as
                                    Sqlite3Int64) as *mut *mut VdbeCursor
                };
            }
        }
        if unsafe { (*db).malloc_failed } != 0 {
            unsafe { (*p).n_var = 0 as YnVar };
            unsafe { (*p).n_cursor = 0 };
            unsafe { (*p).n_mem = 0 };
        } else {
            unsafe { (*p).n_cursor = n_cursor };
            unsafe { (*p).n_var = n_var as YnVar };
            init_mem_array(unsafe { (*p).a_var }, n_var, db, 1 as u16);
            unsafe { (*p).n_mem = n_mem };
            init_mem_array(unsafe { (*p).a_mem }, n_mem, db, 0 as u16);
            unsafe {
                memset(unsafe { (*p).ap_csr } as *mut (), 0,
                    n_cursor as u64 *
                        core::mem::size_of::<*mut VdbeCursor>() as u64)
            };
        }
        sqlite3_vdbe_rewind(unsafe { &mut *p });
    }
}

extern "C" fn free_cursor_with_cache(p: *mut Vdbe, p_cx_1: *mut VdbeCursor)
    -> () {
    let p_cache: *mut VdbeTxtBlbCache = unsafe { (*p_cx_1).p_cache };
    { let _ = 0; };
    unsafe { (*p_cx_1).set_col_cache(0 as Bool as u32) };
    unsafe { (*p_cx_1).p_cache = core::ptr::null_mut() };
    if !(unsafe { (*p_cache).p_c_value }).is_null() {
        unsafe {
            sqlite3_rc_str_unref(unsafe { (*p_cache).p_c_value } as *mut ())
        };
        unsafe { (*p_cache).p_c_value = core::ptr::null_mut() };
    }
    unsafe { sqlite3_db_free(unsafe { (*p).db }, p_cache as *mut ()) };
    unsafe { sqlite3_vdbe_free_cursor_nn(p, p_cx_1) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_free_cursor_nn(p: *mut Vdbe,
    p_cx_1: *mut VdbeCursor) -> () {
    unsafe {
        if unsafe { (*p_cx_1).col_cache() } != 0 {
            free_cursor_with_cache(p, p_cx_1);
            return;
        }
        '__s14:
            {
            match unsafe { (*p_cx_1).e_cur_type } {
                1 => {
                    {
                        unsafe {
                            sqlite3_vdbe_sorter_close(unsafe { (*p).db }, p_cx_1)
                        };
                        break '__s14;
                    }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_btree_close_cursor(unsafe { (*p_cx_1).uc.p_cursor })
                        };
                        break '__s14;
                    }
                    {
                        let p_v_cur: *mut Sqlite3VtabCursor =
                            unsafe { (*p_cx_1).uc.p_v_cur };
                        let p_module: *const Sqlite3Module =
                            unsafe { (*unsafe { (*p_v_cur).p_vtab }).p_module };
                        { let _ = 0; };
                        {
                            let __p =
                                unsafe { &mut (*unsafe { (*p_v_cur).p_vtab }).n_ref };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe {
                            (unsafe { (*p_module).x_close.unwrap() })(p_v_cur)
                        };
                        break '__s14;
                    }
                }
                0 => {
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_btree_close_cursor(unsafe { (*p_cx_1).uc.p_cursor })
                        };
                        break '__s14;
                    }
                    {
                        let p_v_cur: *mut Sqlite3VtabCursor =
                            unsafe { (*p_cx_1).uc.p_v_cur };
                        let p_module: *const Sqlite3Module =
                            unsafe { (*unsafe { (*p_v_cur).p_vtab }).p_module };
                        { let _ = 0; };
                        {
                            let __p =
                                unsafe { &mut (*unsafe { (*p_v_cur).p_vtab }).n_ref };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe {
                            (unsafe { (*p_module).x_close.unwrap() })(p_v_cur)
                        };
                        break '__s14;
                    }
                }
                2 => {
                    {
                        let p_v_cur: *mut Sqlite3VtabCursor =
                            unsafe { (*p_cx_1).uc.p_v_cur };
                        let p_module: *const Sqlite3Module =
                            unsafe { (*unsafe { (*p_v_cur).p_vtab }).p_module };
                        { let _ = 0; };
                        {
                            let __p =
                                unsafe { &mut (*unsafe { (*p_v_cur).p_vtab }).n_ref };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        unsafe {
                            (unsafe { (*p_module).x_close.unwrap() })(p_v_cur)
                        };
                        break '__s14;
                    }
                }
                _ => {}
            }
        }
    }
}

///* Close all cursors in the current frame.
extern "C" fn close_cursors_in_frame(p: *mut Vdbe) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b15: loop {
            if !(i < unsafe { (*p).n_cursor }) { break '__b15; }
            '__c15: loop {
                let p_c: *mut VdbeCursor =
                    unsafe { *unsafe { (*p).ap_csr.offset(i as isize) } };
                if !(p_c).is_null() {
                    sqlite3_vdbe_free_cursor_nn(p, p_c);
                    unsafe {
                        *unsafe { (*p).ap_csr.offset(i as isize) } =
                            core::ptr::null_mut()
                    };
                }
                break '__c15;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* If parameter iOp is less than zero, then invoke the destructor for
///* all auxiliary data pointers currently cached by the VM passed as
///* the first argument.
///*
///* Or, if iOp is greater than or equal to zero, then the destructor is
///* only invoked for those auxiliary data pointers created by the user
///* function invoked by the OP_Function opcode at instruction iOp of
///* VM pVdbe, and only then if:
///*
///*    * the associated function parameter is the 32nd or later (counting
///*      from left to right), or
///*
///*    * the corresponding bit in argument mask is clear (where the first
///*      function parameter corresponds to bit 0 etc.).
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_delete_aux_data(db: *mut Sqlite3,
    mut pp: *mut *mut AuxData, i_op_1: i32, mask: i32) -> () {
    while !(unsafe { *pp }).is_null() {
        let p_aux: *mut AuxData = unsafe { *pp };
        if i_op_1 < 0 ||
                unsafe { (*p_aux).i_aux_op } == i_op_1 &&
                        unsafe { (*p_aux).i_aux_arg } >= 0 &&
                    (unsafe { (*p_aux).i_aux_arg } > 31 ||
                        (mask as u32 & (1 as u32) << unsafe { (*p_aux).i_aux_arg }
                                    == 0) as i32 != 0) {
            if unsafe { (*p_aux).x_delete_aux.is_some() } {
                unsafe {
                    (unsafe {
                            (*p_aux).x_delete_aux.unwrap()
                        })(unsafe { (*p_aux).p_aux })
                };
            }
            unsafe { *pp = unsafe { (*p_aux).p_next_aux } };
            unsafe { sqlite3_db_free(db, p_aux as *mut ()) };
        } else { pp = unsafe { &mut (*p_aux).p_next_aux }; }
    }
}

///* Copy the values stored in the VdbeFrame structure to its Vdbe. This
///* is used, for example, when a trigger sub-program is halted to restore
///* control to the main program.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_frame_restore(p_frame_1: &mut VdbeFrame)
    -> i32 {
    unsafe {
        let v: *mut Vdbe = (*p_frame_1).v;
        close_cursors_in_frame(v);
        unsafe { (*v).a_op = (*p_frame_1).a_op };
        unsafe { (*v).n_op = (*p_frame_1).n_op };
        unsafe { (*v).a_mem = (*p_frame_1).a_mem };
        unsafe { (*v).n_mem = (*p_frame_1).n_mem };
        unsafe { (*v).ap_csr = (*p_frame_1).ap_csr };
        unsafe { (*v).n_cursor = (*p_frame_1).n_cursor };
        unsafe { (*unsafe { (*v).db }).last_rowid = (*p_frame_1).last_rowid };
        unsafe { (*v).n_change = (*p_frame_1).n_change };
        unsafe { (*unsafe { (*v).db }).n_change = (*p_frame_1).n_db_change };
        unsafe {
            sqlite3_vdbe_delete_aux_data(unsafe { (*v).db },
                unsafe { &mut (*v).p_aux_data }, -1, 0)
        };
        unsafe { (*v).p_aux_data = (*p_frame_1).p_aux_data };
        (*p_frame_1).p_aux_data = core::ptr::null_mut();
        return (*p_frame_1).pc;
    }
}

///* Delete a VdbeFrame object and its contents. VdbeFrame objects are
///* allocated by the OP_Program opcode in sqlite3VdbeExec().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_frame_delete(p: *mut VdbeFrame) -> () {
    let mut i: i32 = 0;
    let a_mem: *mut Mem =
        unsafe {
                &raw mut *(p as
                                *mut u8).add((core::mem::size_of::<VdbeFrame>() as u64 +
                                        7 as u64 & !7 as u64) as usize)
            } as *mut Mem;
    let ap_csr: *const *mut VdbeCursor =
        unsafe {
                    &raw mut *a_mem.offset(unsafe { (*p).n_child_mem } as isize)
                } as *mut *mut VdbeCursor as *const *mut VdbeCursor;
    { let _ = 0; };
    {
        i = 0;
        '__b17: loop {
            if !(i < unsafe { (*p).n_child_csr }) { break '__b17; }
            '__c17: loop {
                if !(unsafe { *ap_csr.offset(i as isize) }).is_null() {
                    unsafe {
                        sqlite3_vdbe_free_cursor_nn(unsafe { (*p).v },
                            unsafe { *ap_csr.offset(i as isize) })
                    };
                }
                break '__c17;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    release_mem_array(a_mem, unsafe { (*p).n_child_mem });
    unsafe {
        sqlite3_vdbe_delete_aux_data(unsafe { (*unsafe { (*p).v }).db },
            unsafe { &mut (*p).p_aux_data }, -1, 0)
    };
    unsafe {
        sqlite3_db_free(unsafe { (*unsafe { (*p).v }).db }, p as *mut ())
    };
}

///* Close all cursors.
///*
///* Also release any dynamic memory held by the VM in the Vdbe.aMem memory
///* cell array. This is necessary as the memory cell array may contain
///* pointers to VdbeFrame objects, which may in turn contain pointers to
///* open cursors.
extern "C" fn close_all_cursors(p: *mut Vdbe) -> () {
    unsafe {
        if !(unsafe { (*p).p_frame }).is_null() {
            let mut p_frame: *mut VdbeFrame = core::ptr::null_mut();
            {
                p_frame = unsafe { (*p).p_frame };
                '__b18: loop {
                    if !(!(unsafe { (*p_frame).p_parent }).is_null()) {
                        break '__b18;
                    }
                    '__c18: loop { break '__c18; }
                    p_frame = unsafe { (*p_frame).p_parent };
                }
            }
            sqlite3_vdbe_frame_restore(unsafe { &mut *p_frame });
            unsafe { (*p).p_frame = core::ptr::null_mut() };
            unsafe { (*p).n_frame = 0 };
        }
        { let _ = 0; };
        close_cursors_in_frame(p);
        release_mem_array(unsafe { (*p).a_mem }, unsafe { (*p).n_mem });
        while !(unsafe { (*p).p_del_frame }).is_null() {
            let p_del: *mut VdbeFrame = unsafe { (*p).p_del_frame };
            unsafe { (*p).p_del_frame = unsafe { (*p_del).p_parent } };
            sqlite3_vdbe_frame_delete(p_del);
        }
        if !(unsafe { (*p).p_aux_data }).is_null() {
            unsafe {
                sqlite3_vdbe_delete_aux_data(unsafe { (*p).db },
                    unsafe { &mut (*p).p_aux_data }, -1, 0)
            };
        }
        { let _ = 0; };
    }
}

///* If SQLite is compiled to support shared-cache mode and to be threadsafe,
///* this routine obtains the mutex associated with each BtShared structure
///* that may be accessed by the VM passed as an argument. In doing so it also
///* sets the BtShared.db member of each of the BtShared structures, ensuring
///* that the correct busy-handler callback is invoked if required.
///*
///* If SQLite is not threadsafe but does support shared-cache mode, then
///* sqlite3BtreeEnter() is invoked to set the BtShared.db variables
///* of all of BtShared structures accessible via the database handle
///* associated with the VM.
///*
///* If SQLite is not threadsafe and does not support shared-cache mode, this
///* function is a no-op.
///*
///* The p->btreeMask field is a bitmask of all btrees that the prepared
///* statement p will ever use.  Let N be the number of bits in p->btreeMask
///* corresponding to btrees that use shared cache.  Then the runtime of
///* this routine is N*N.  But as N is rarely more than 1, this should not
///* be a problem.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_enter(p: &Vdbe) -> () {
    let mut i: i32 = 0;
    let mut db: *const Sqlite3 = core::ptr::null();
    let mut a_db: *const Db = core::ptr::null();
    let mut n_db: i32 = 0;
    if (*p).lock_mask == 0 as u32 { return; }

    /// The common case
    (db = (*p).db);
    a_db = unsafe { (*db).a_db };
    n_db = unsafe { (*db).n_db };
    {
        i = 0;
        '__b20: loop {
            if !(i < n_db) { break '__b20; }
            '__c20: loop {
                if i != 1 && (*p).lock_mask & (1 as YDbMask) << i != 0 as u32
                        &&
                        unsafe { (*a_db.offset(i as isize)).p_bt } !=
                            core::ptr::null_mut() {
                    unsafe {
                        sqlite3_btree_enter(unsafe {
                                (*a_db.offset(i as isize)).p_bt
                            })
                    };
                }
                break '__c20;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Function prototypes
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vdbe_error(p: &mut Vdbe,
    z_format_1: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { sqlite3_db_free((*p).db, (*p).z_err_msg as *mut ()) };
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    (*p).z_err_msg = unsafe { sqlite3_vm_printf((*p).db, z_format_1, ap) };
    ();
}

extern "C" fn vdbe_fk_error(p: *mut Vdbe) -> i32 {
    unsafe { (*p).rc = 19 | 3 << 8 };
    unsafe { (*p).error_action = 2 as u8 };
    unsafe {
        sqlite3_vdbe_error(unsafe { &mut *p },
            c"FOREIGN KEY constraint failed".as_ptr() as *mut i8 as *const i8)
    };
    if unsafe { (*p).prep_flags } as i32 & 128 == 0 { return 1; }
    return 19 | 3 << 8;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_check_fk_immediate(p: *mut Vdbe) -> i32 {
    if unsafe { (*p).n_fk_constraint } == 0 as i64 { return 0; }
    return vdbe_fk_error(p);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_check_fk_deferred(p: *mut Vdbe) -> i32 {
    let db: *const Sqlite3 = unsafe { (*p).db } as *const Sqlite3;
    if unsafe { (*db).n_deferred_cons } + unsafe { (*db).n_deferred_imm_cons }
            == 0 as i64 {
        return 0;
    }
    return vdbe_fk_error(p);
}

///* Unlock all of the btrees previously locked by a call to sqlite3VdbeEnter().
extern "C" fn vdbe_leave(p: &Vdbe) -> () {
    let mut i: i32 = 0;
    let mut db: *const Sqlite3 = core::ptr::null();
    let mut a_db: *const Db = core::ptr::null();
    let mut n_db: i32 = 0;
    db = (*p).db;
    a_db = unsafe { (*db).a_db };
    n_db = unsafe { (*db).n_db };
    {
        i = 0;
        '__b21: loop {
            if !(i < n_db) { break '__b21; }
            '__c21: loop {
                if i != 1 && (*p).lock_mask & (1 as YDbMask) << i != 0 as u32
                        &&
                        unsafe { (*a_db.offset(i as isize)).p_bt } !=
                            core::ptr::null_mut() {
                    unsafe {
                        sqlite3_btree_leave(unsafe {
                                (*a_db.offset(i as isize)).p_bt
                            })
                    };
                }
                break '__c21;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_leave(p: *mut Vdbe) -> () {
    if unsafe { (*p).lock_mask } == 0 as u32 { return; }

    /// The common case
    vdbe_leave(unsafe { &*p });
}

///* A read or write transaction may or may not be active on database handle
///* db. If a transaction is active, commit it. If there is a
///* write-transaction spanning more than one database file, this routine
///* takes care of the super-journal trickery.
#[allow(unused_doc_comments)]
extern "C" fn vdbe_commit(db: *mut Sqlite3, p: *mut Vdbe) -> i32 {
    let mut i: i32 = 0;
    let mut n_trans: i32 = 0;
    /// Number of databases with an active write-transaction
    ///* that are candidates for a two-phase commit using a
    ///* super-journal
    let mut rc: i32 = 0;
    let mut need_xcommit: i32 = 0;

    /// Before doing anything else, call the xSync() callback for any
    ///* virtual module tables written in this transaction. This has to
    ///* be done before determining whether a super-journal file is
    ///* required, as an xSync() callback may add an attached database
    ///* to the transaction.
    (rc = unsafe { sqlite3_vtab_sync(db, p) });
    {
        i = 0;
        '__b22: loop {
            if !(rc == 0 && i < unsafe { (*db).n_db }) { break '__b22; }
            '__c22: loop {
                let p_bt: *mut Btree =
                    unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                if unsafe { sqlite3_btree_txn_state(p_bt) } == 2 {
                    /// DELETE
                    /// PERSIST
                    /// OFF
                    /// TRUNCATE
                    /// MEMORY
                    /// WAL
                    let mut p_pager: *mut Pager = core::ptr::null_mut();

                    /// Pager associated with pBt
                    (need_xcommit = 1);
                    unsafe { sqlite3_btree_enter(p_bt) };
                    p_pager = unsafe { sqlite3_btree_pager(p_bt) };
                    if unsafe {
                                            (*unsafe { (*db).a_db.offset(i as isize) }).safety_level
                                        } as i32 != 1 &&
                                a_mj_needed[unsafe {
                                                sqlite3_pager_get_journal_mode(p_pager)
                                            } as usize] != 0 &&
                            unsafe { sqlite3_pager_is_memdb(p_pager) } == 0 {
                        { let _ = 0; };
                        { let __p = &mut n_trans; let __t = *__p; *__p += 1; __t };
                    }
                    rc = unsafe { sqlite3_pager_exclusive_lock(p_pager) };
                    unsafe { sqlite3_btree_leave(p_bt) };
                }
                break '__c22;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc != 0 { return rc; }
    if need_xcommit != 0 && unsafe { (*db).x_commit_callback.is_some() } {
        rc =
            unsafe {
                (unsafe {
                        (*db).x_commit_callback.unwrap()
                    })(unsafe { (*db).p_commit_arg })
            };
        if rc != 0 { return 19 | 2 << 8; }
    }
    if 0 ==
                unsafe {
                    sqlite3_strlen30(unsafe {
                            sqlite3_btree_get_filename(unsafe {
                                    (*unsafe { (*db).a_db.offset(0 as isize) }).p_bt
                                })
                        })
                } || n_trans <= 1 {
        if need_xcommit != 0 {
            {
                i = 0;
                '__b23: loop {
                    if !(rc == 0 && i < unsafe { (*db).n_db }) { break '__b23; }
                    '__c23: loop {
                        let p_bt_1: *mut Btree =
                            unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                        if unsafe { sqlite3_btree_txn_state(p_bt_1) } >= 2 {
                            rc =
                                unsafe {
                                    sqlite3_btree_commit_phase_one(p_bt_1, core::ptr::null())
                                };
                        }
                        break '__c23;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        {
            i = 0;
            '__b24: loop {
                if !(rc == 0 && i < unsafe { (*db).n_db }) { break '__b24; }
                '__c24: loop {
                    let p_bt_2: *mut Btree =
                        unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                    let txn: i32 = unsafe { sqlite3_btree_txn_state(p_bt_2) };
                    if txn != 0 {
                        { let _ = 0; };
                        rc = unsafe { sqlite3_btree_commit_phase_two(p_bt_2, 0) };
                    }
                    break '__c24;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc == 0 { unsafe { sqlite3_vtab_commit(db) }; }
    } else {
        let p_vfs: *mut Sqlite3Vfs = unsafe { (*db).p_vfs };
        let mut z_super: *mut i8 = core::ptr::null_mut();
        /// File-name for the super-journal
        let z_main_file: *const i8 =
            unsafe {
                sqlite3_btree_get_filename(unsafe {
                        (*unsafe { (*db).a_db.offset(0 as isize) }).p_bt
                    })
            };
        let mut p_super_jrnl: *mut Sqlite3File = core::ptr::null_mut();
        let mut offset: i64 = 0 as i64;
        let mut res: i32 = 0;
        let mut retry_count: i32 = 0;
        let mut n_main_file: i32 = 0;

        /// Select a super-journal file name
        (n_main_file = unsafe { sqlite3_strlen30(z_main_file) });
        z_super =
            unsafe {
                sqlite3_m_printf(db,
                    c"%.4c%s%.16c".as_ptr() as *mut i8 as *const i8, 0,
                    z_main_file, 0)
            };
        if z_super == core::ptr::null_mut() { return 7; }
        {
            let __n = 4;
            let __p = &mut z_super;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        '__b25: loop {
            '__c25: loop {
                let mut i_random: u32 = 0 as u32;
                if retry_count != 0 {
                    if retry_count > 100 {
                        unsafe {
                            sqlite3_log(13,
                                c"MJ delete: %s".as_ptr() as *mut i8 as *const i8, z_super)
                        };
                        unsafe {
                            sqlite3_os_delete(p_vfs, z_super as *const i8, 0)
                        };
                        break '__b25;
                    } else if retry_count == 1 {
                        unsafe {
                            sqlite3_log(13,
                                c"MJ collide: %s".as_ptr() as *mut i8 as *const i8, z_super)
                        };
                    }
                }
                {
                    let __p = &mut retry_count;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                unsafe {
                    sqlite3_randomness(core::mem::size_of::<u32>() as i32,
                        &raw mut i_random as *mut ())
                };
                unsafe {
                    sqlite3_snprintf(13,
                        unsafe { &mut *z_super.offset(n_main_file as isize) },
                        c"-mj%06X9%02X".as_ptr() as *mut i8 as *const i8,
                        i_random >> 8 & 16777215 as u32, i_random & 255 as u32)
                };

                /// The antipenultimate character of the super-journal name must
                ///* be "9" to avoid name collisions when using 8+3 filenames.
                { let _ = 0; };
                rc =
                    unsafe {
                        sqlite3_os_access(p_vfs, z_super as *const i8, 0, &mut res)
                    };
                break '__c25;
            }
            if !(rc == 0 && res != 0) { break '__b25; }
        }
        if rc == 0 {

            /// Open the super-journal.
            (rc =
                unsafe {
                    sqlite3_os_open_malloc(p_vfs, z_super as *const i8,
                        &mut p_super_jrnl, 2 | 4 | 16 | 16384,
                        core::ptr::null_mut())
                });
        }
        if rc != 0 {
            unsafe {
                sqlite3_db_free(db,
                    unsafe { z_super.offset(-(4 as isize)) } as *mut ())
            };
            return rc;
        }
        {
            i = 0;
            '__b26: loop {
                if !(i < unsafe { (*db).n_db }) { break '__b26; }
                '__c26: loop {
                    let p_bt_3: *mut Btree =
                        unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                    if unsafe { sqlite3_btree_txn_state(p_bt_3) } == 2 {
                        let z_file: *const i8 =
                            unsafe { sqlite3_btree_get_journalname(p_bt_3) };
                        if z_file == core::ptr::null() { break '__c26; }
                        { let _ = 0; };
                        rc =
                            unsafe {
                                sqlite3_os_write(p_super_jrnl, z_file as *const (),
                                    unsafe { sqlite3_strlen30(z_file) } + 1, offset)
                            };
                        offset += (unsafe { sqlite3_strlen30(z_file) } + 1) as i64;
                        if rc != 0 {
                            unsafe { sqlite3_os_close_free(p_super_jrnl) };
                            unsafe {
                                sqlite3_os_delete(p_vfs, z_super as *const i8, 0)
                            };
                            unsafe {
                                sqlite3_db_free(db,
                                    unsafe { z_super.offset(-(4 as isize)) } as *mut ())
                            };
                            return rc;
                        }
                    }
                    break '__c26;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if 0 ==
                    unsafe { sqlite3_os_device_characteristics(p_super_jrnl) } &
                        1024 &&
                0 != { rc = unsafe { sqlite3_os_sync(p_super_jrnl, 2) }; rc }
            {
            unsafe { sqlite3_os_close_free(p_super_jrnl) };
            unsafe { sqlite3_os_delete(p_vfs, z_super as *const i8, 0) };
            unsafe {
                sqlite3_db_free(db,
                    unsafe { z_super.offset(-(4 as isize)) } as *mut ())
            };
            return rc;
        }
        {
            i = 0;
            '__b27: loop {
                if !(rc == 0 && i < unsafe { (*db).n_db }) { break '__b27; }
                '__c27: loop {
                    let p_bt_4: *mut Btree =
                        unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                    if !(p_bt_4).is_null() {
                        rc =
                            unsafe {
                                sqlite3_btree_commit_phase_one(p_bt_4, z_super as *const i8)
                            };
                    }
                    break '__c27;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_os_close_free(p_super_jrnl) };
        { let _ = 0; };
        if rc != 0 {
            unsafe {
                sqlite3_db_free(db,
                    unsafe { z_super.offset(-(4 as isize)) } as *mut ())
            };
            return rc;
        }

        /// Delete the super-journal file. This commits the transaction. After
        ///* doing this the directory is synced again before any individual
        ///* transaction files are deleted.
        (rc = unsafe { sqlite3_os_delete(p_vfs, z_super as *const i8, 1) });
        unsafe {
            sqlite3_db_free(db,
                unsafe { z_super.offset(-(4 as isize)) } as *mut ())
        };
        z_super = core::ptr::null_mut();
        if rc != 0 { return rc; }
        unsafe { sqlite3_begin_benign_malloc() };
        {
            i = 0;
            '__b28: loop {
                if !(i < unsafe { (*db).n_db }) { break '__b28; }
                '__c28: loop {
                    let p_bt_5: *mut Btree =
                        unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                    if !(p_bt_5).is_null() {
                        unsafe { sqlite3_btree_commit_phase_two(p_bt_5, 1) };
                    }
                    break '__c28;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_end_benign_malloc() };
        unsafe { sqlite3_vtab_commit(db) };
    }
    return rc;
}

///* If the Vdbe passed as the first argument opened a statement-transaction,
///* close it now. Argument eOp must be either SAVEPOINT_ROLLBACK or
///* SAVEPOINT_RELEASE. If it is SAVEPOINT_ROLLBACK, then the statement
///* transaction is rolled back. If eOp is SAVEPOINT_RELEASE, then the
///* statement transaction is committed.
///*
///* If an IO error occurs, an SQLITE_IOERR_XXX error code is returned.
///* Otherwise SQLITE_OK.
extern "C" fn vdbe_close_statement(p: &mut Vdbe, e_op_1: i32) -> i32 {
    let db: *mut Sqlite3 = (*p).db;
    let mut rc: i32 = 0;
    let mut i: i32 = 0;
    let i_savepoint: i32 = ((*p).i_statement - 1) as i32;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    {
        i = 0;
        '__b29: loop {
            if !(i < unsafe { (*db).n_db }) { break '__b29; }
            '__c29: loop {
                let mut rc2: i32 = 0;
                let p_bt: *mut Btree =
                    unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                if !(p_bt).is_null() {
                    if e_op_1 == 2 {
                        rc2 =
                            unsafe { sqlite3_btree_savepoint(p_bt, 2, i_savepoint) };
                    }
                    if rc2 == 0 {
                        rc2 =
                            unsafe { sqlite3_btree_savepoint(p_bt, 1, i_savepoint) };
                    }
                    if rc == 0 { rc = rc2; }
                }
                break '__c29;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    {
        let __p = unsafe { &mut (*db).n_statement };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    (*p).i_statement = 0;
    if rc == 0 {
        if e_op_1 == 2 {
            rc = unsafe { sqlite3_vtab_savepoint(db, 2, i_savepoint) };
        }
        if rc == 0 {
            rc = unsafe { sqlite3_vtab_savepoint(db, 1, i_savepoint) };
        }
    }
    if e_op_1 == 2 {
        unsafe { (*db).n_deferred_cons = (*p).n_stmt_def_cons };
        unsafe { (*db).n_deferred_imm_cons = (*p).n_stmt_def_imm_cons };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_close_statement(p: *mut Vdbe, e_op_1: i32)
    -> i32 {
    if unsafe { (*unsafe { (*p).db }).n_statement } != 0 &&
            unsafe { (*p).i_statement } != 0 {
        return vdbe_close_statement(unsafe { &mut *p }, e_op_1);
    }
    return 0;
}

///* This routine sets the value to be returned by subsequent calls to
///* sqlite3_changes() on the database handle 'db'.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_set_changes(db: &mut Sqlite3, n_change_1: i64)
    -> () {
    { let _ = 0; };
    (*db).n_change = n_change_1;
    (*db).n_total_change += n_change_1;
}

///* This routine is called the when a VDBE tries to halt.  If the VDBE
///* has made changes and is in autocommit mode, then commit those
///* changes.  If a rollback is needed, then do the rollback.
///*
///* This routine is the only way to move the sqlite3eOpenState of a VM from
///* SQLITE_STATE_RUN to SQLITE_STATE_HALT.  It is harmless to
///* call this on a VM that is in the SQLITE_STATE_HALT state.
///*
///* Return an error code.  If the commit could not complete because of
///* lock contention, return SQLITE_BUSY.  If SQLITE_BUSY is returned, it
///* means the close did not happen and needs to be repeated.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_halt(p: *mut Vdbe) -> i32 {
    let mut rc: i32 = 0;
    /// Used to store transient return codes
    let db: *mut Sqlite3 = unsafe { (*p).db };

    /// This function contains the logic that determines if a statement or
    ///* transaction will be committed or rolled back as a result of the
    ///* execution of this virtual machine.
    ///*
    ///* If any of the following errors occur:
    ///*
    ///*     SQLITE_NOMEM
    ///*     SQLITE_IOERR
    ///*     SQLITE_FULL
    ///*     SQLITE_INTERRUPT
    ///*
    ///* Then the internal cache might have been left in an inconsistent
    ///* state.  We need to rollback the statement transaction, if there is
    ///* one, or the complete transaction if there is no statement transaction.
    { let _ = 0; };
    if unsafe { (*db).malloc_failed } != 0 { unsafe { (*p).rc = 7 }; }
    close_all_cursors(p);
    if unsafe { (*p).b_is_reader() } != 0 {
        let mut mrc: i32 = 0;
        /// Primary error code from p->rc
        let mut e_statement_op: i32 = 0;
        let mut is_special_error: i32 = 0;

        /// Set to true if a 'special' error
        /// Lock all btrees used by the statement
        sqlite3_vdbe_enter(unsafe { &*p });
        if unsafe { (*p).rc } != 0 {
            mrc = unsafe { (*p).rc } & 255;
            is_special_error =
                (mrc == 7 || mrc == 10 || mrc == 9 || mrc == 13) as i32;
        } else { mrc = { is_special_error = 0; is_special_error }; }
        if is_special_error != 0 {
            if (unsafe { (*p).read_only() } == 0) as i32 != 0 || mrc != 9 {
                if (mrc == 7 || mrc == 13) &&
                        unsafe { (*p).uses_stmt_journal() } != 0 {
                    e_statement_op = 2;
                } else {

                    /// We are forced to roll back the active transaction. Before doing
                    ///* so, abort any other statements this handle currently has active.
                    unsafe { sqlite3_rollback_all(db, 4 | 2 << 8) };
                    unsafe { sqlite3_close_savepoints(db) };
                    unsafe { (*db).auto_commit = 1 as u8 };
                    unsafe { (*p).n_change = 0 as i64 };
                }
            }
        }
        if unsafe { (*p).rc } == 0 ||
                unsafe { (*p).error_action } as i32 == 3 &&
                    (is_special_error == 0) as i32 != 0 {
            { let _ = sqlite3_vdbe_check_fk_immediate(p); };
        }
        if !(unsafe { (*db).n_v_trans } > 0 &&
                                    unsafe { (*db).a_v_trans } == core::ptr::null_mut()) as i32
                        != 0 && unsafe { (*db).auto_commit } != 0 &&
                unsafe { (*db).n_vdbe_write } ==
                    (unsafe { (*p).read_only() } as i32 == 0) as i32 {
            if unsafe { (*p).rc } == 0 ||
                    unsafe { (*p).error_action } as i32 == 3 &&
                        (is_special_error == 0) as i32 != 0 {
                rc = sqlite3_vdbe_check_fk_deferred(p);
                if rc != 0 {
                    if unsafe { (*p).read_only() } != 0 {
                        sqlite3_vdbe_leave(p);
                        return 1;
                    }
                    rc = 19 | 3 << 8;
                } else if unsafe { (*db).flags } & (2 as u64) << 32 != 0 {
                    rc = 11;
                    unsafe { (*db).flags &= !((2 as u64) << 32) };
                } else {

                    /// The auto-commit flag is true, the vdbe program was successful
                    ///* or hit an 'OR FAIL' constraint and there are no deferred foreign
                    ///* key constraints to hold up the transaction. This means a commit
                    ///* is required.
                    (rc = vdbe_commit(db, p));
                }
                if rc == 5 && unsafe { (*p).read_only() } != 0 {
                    sqlite3_vdbe_leave(p);
                    return 5;
                } else if rc != 0 {
                    unsafe { sqlite3_system_error(db, rc) };
                    unsafe { (*p).rc = rc };
                    unsafe { sqlite3_rollback_all(db, 0) };
                    unsafe { (*p).n_change = 0 as i64 };
                } else {
                    unsafe { (*db).n_deferred_cons = 0 as i64 };
                    unsafe { (*db).n_deferred_imm_cons = 0 as i64 };
                    unsafe { (*db).flags &= !(524288 as u64) };
                    unsafe { sqlite3_commit_internal_changes(db) };
                }
            } else if unsafe { (*p).rc } == 17 &&
                    unsafe { (*db).n_vdbe_active } > 1 {
                unsafe { (*p).n_change = 0 as i64 };
            } else {
                unsafe { sqlite3_rollback_all(db, 0) };
                unsafe { (*p).n_change = 0 as i64 };
            }
            unsafe { (*db).n_statement = 0 };
        } else if e_statement_op == 0 {
            if unsafe { (*p).rc } == 0 ||
                    unsafe { (*p).error_action } as i32 == 3 {
                e_statement_op = 1;
            } else if unsafe { (*p).error_action } as i32 == 2 {
                e_statement_op = 2;
            } else {
                unsafe { sqlite3_rollback_all(db, 4 | 2 << 8) };
                unsafe { sqlite3_close_savepoints(db) };
                unsafe { (*db).auto_commit = 1 as u8 };
                unsafe { (*p).n_change = 0 as i64 };
            }
        }
        if e_statement_op != 0 {
            rc = sqlite3_vdbe_close_statement(p, e_statement_op);
            if rc != 0 {
                if unsafe { (*p).rc } == 0 || unsafe { (*p).rc } & 255 == 19 {
                    unsafe { (*p).rc = rc };
                    unsafe {
                        sqlite3_db_free(db, unsafe { (*p).z_err_msg } as *mut ())
                    };
                    unsafe { (*p).z_err_msg = core::ptr::null_mut() };
                }
                unsafe { sqlite3_rollback_all(db, 4 | 2 << 8) };
                unsafe { sqlite3_close_savepoints(db) };
                unsafe { (*db).auto_commit = 1 as u8 };
                unsafe { (*p).n_change = 0 as i64 };
            }
        }
        if unsafe { (*p).change_cnt_on() } != 0 {
            if e_statement_op != 2 {
                unsafe {
                    sqlite3_vdbe_set_changes(unsafe { &mut *db },
                        unsafe { (*p).n_change })
                };
            } else {
                unsafe {
                    sqlite3_vdbe_set_changes(unsafe { &mut *db }, 0 as i64)
                };
            }
            unsafe { (*p).n_change = 0 as i64 };
        }

        /// Release the locks
        sqlite3_vdbe_leave(p);
    }

    /// We have successfully halted and closed the VM.  Record this fact.
    {
        let __p = unsafe { &mut (*db).n_vdbe_active };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    if (unsafe { (*p).read_only() } == 0) as i32 != 0 {
        {
            let __p = unsafe { &mut (*db).n_vdbe_write };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
    if unsafe { (*p).b_is_reader() } != 0 {
        {
            let __p = unsafe { &mut (*db).n_vdbe_read };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { (*p).e_vdbe_state = 3 as u8 };
    if unsafe { (*db).malloc_failed } != 0 { unsafe { (*p).rc = 7 }; }
    if unsafe { (*db).auto_commit } != 0 {}
    { let _ = 0; };
    return if unsafe { (*p).rc } == 5 { 5 } else { 0 };
}

///* Copy the error code and error message belonging to the VDBE passed
///* as the first argument to its database handle (so that they will be
///* returned by calls to sqlite3_errcode() and sqlite3_errmsg()).
///*
///* This function does not clear the VDBE error code or message, just
///* copies them to the database handle.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_transfer_error(p: &Vdbe) -> i32 {
    let db: *mut Sqlite3 = (*p).db;
    let rc: i32 = (*p).rc;
    if !((*p).z_err_msg).is_null() {
        {
            let __p = unsafe { &mut (*db).b_benign_malloc };
            let __t = *__p;
            *__p += 1;
            __t
        };
        unsafe { sqlite3_begin_benign_malloc() };
        if unsafe { (*db).p_err } == core::ptr::null_mut() {
            unsafe { (*db).p_err = unsafe { sqlite3_value_new(db) } };
        }
        unsafe {
            sqlite3_value_set_str(unsafe { (*db).p_err }, -1,
                (*p).z_err_msg as *const (), 1 as u8,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(-1 as isize as *const ())
                    }))
        };
        unsafe { sqlite3_end_benign_malloc() };
        {
            let __p = unsafe { &mut (*db).b_benign_malloc };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    } else if !(unsafe { (*db).p_err }).is_null() {
        unsafe { sqlite3_value_set_null(unsafe { (*db).p_err }) };
    }
    unsafe { (*db).err_code = rc };
    unsafe { (*db).err_byte_offset = -1 };
    return rc;
}

///* Clean up a VDBE after execution but do not delete the VDBE just yet.
///* Write any error messages into *pzErrMsg.  Return the result code.
///*
///* After this routine is run, the VDBE should be ready to be executed
///* again.
///*
///* To look at it another way, this routine resets the state of the
///* virtual machine from VDBE_RUN_STATE or VDBE_HALT_STATE back to
///* VDBE_READY_STATE.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_reset(p: *mut Vdbe) -> i32 {
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    db = unsafe { (*p).db };
    if unsafe { (*p).e_vdbe_state } as i32 == 2 { sqlite3_vdbe_halt(p); }
    if unsafe { (*p).pc } >= 0 {
        if !(unsafe { (*db).p_err }).is_null() ||
                !(unsafe { (*p).z_err_msg }).is_null() {
            sqlite3_vdbe_transfer_error(unsafe { &*p });
        } else { unsafe { (*db).err_code = unsafe { (*p).rc } }; }
    }
    if !(unsafe { (*p).z_err_msg }).is_null() {
        unsafe { sqlite3_db_free(db, unsafe { (*p).z_err_msg } as *mut ()) };
        unsafe { (*p).z_err_msg = core::ptr::null_mut() };
    }
    unsafe { (*p).p_result_row = core::ptr::null_mut() };

    /// Save profiling information from this VDBE run.
    return unsafe { (*p).rc } & unsafe { (*db).err_mask };
}

///* Clean up and delete a VDBE after execution.  Return an integer which is
///* the result code.  Write any error message text into *pzErrMsg.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_finalize(p: *mut Vdbe) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p).e_vdbe_state } as i32 >= 1 {
        rc = sqlite3_vdbe_reset(p);
        { let _ = 0; };
    }
    unsafe { sqlite3_vdbe_delete(p) };
    return rc;
}

///* Resolve label "x" to be the address of the next instruction to
///* be inserted.  The parameter "x" must have been obtained from
///* a prior call to sqlite3VdbeMakeLabel().
extern "C" fn resize_resolve_label(p: *mut Parse, v: &Vdbe, j: i32) -> () {
    let n_new_size: i32 = 25 - unsafe { (*p).n_label };
    unsafe {
        (*p).a_label =
            unsafe {
                    sqlite3_db_realloc_or_free(unsafe { (*p).db },
                        unsafe { (*p).a_label } as *mut (),
                        n_new_size as u64 * core::mem::size_of::<i32>() as u64)
                } as *mut i32
    };
    if unsafe { (*p).a_label } == core::ptr::null_mut() {
        unsafe { (*p).n_label_alloc = 0 };
    } else {
        if n_new_size >= 100 &&
                n_new_size / 100 > unsafe { (*p).n_label_alloc } / 100 {
            unsafe { sqlite3_progress_check(p) };
        }
        unsafe { (*p).n_label_alloc = n_new_size };
        unsafe { *unsafe { (*p).a_label.offset(j as isize) } = (*v).n_op };
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_resolve_label(v: *mut Vdbe, x: i32) -> () {
    let p: *mut Parse = unsafe { (*v).p_parse };
    let j: i32 = !x;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p).n_label_alloc } + unsafe { (*p).n_label } < 0 {
        resize_resolve_label(p, unsafe { &*v }, j);
    } else {
        { let _ = 0; };

        /// Labels may only be resolved once
        unsafe {
            *unsafe { (*p).a_label.offset(j as isize) } = unsafe { (*v).n_op }
        };
    }
}

///* Each VDBE holds the result of the most recent sqlite3_step() call
///* in p->rc.  This routine sets that result back to SQLITE_OK.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_reset_step_result(p: &mut Vdbe) -> () {
    (*p).rc = 0;
}

///* Set the number of result columns that will be returned by this SQL
///* statement. This is now set at compile time, rather than during
///* execution of the vdbe program so that sqlite3_column_count() can
///* be called on an SQL statement before sqlite3_step().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_set_num_cols(p: &mut Vdbe, n_res_column_1: i32)
    -> () {
    let mut n: i32 = 0;
    let db: *mut Sqlite3 = (*p).db;
    if (*p).n_res_alloc != 0 {
        release_mem_array((*p).a_col_name, (*p).n_res_alloc as i32 * 2);
        unsafe { sqlite3_db_free(db, (*p).a_col_name as *mut ()) };
    }
    n = n_res_column_1 * 2;
    (*p).n_res_column =
        { (*p).n_res_alloc = n_res_column_1 as u16; (*p).n_res_alloc };
    (*p).a_col_name =
        unsafe {
                sqlite3_db_malloc_raw_nn(db,
                    core::mem::size_of::<Mem>() as u64 * n as u64)
            } as *mut Mem;
    if (*p).a_col_name == core::ptr::null_mut() { return; }
    init_mem_array((*p).a_col_name, n, db, 1 as u16);
}

///* Set the name of the idx'th column to be returned by the SQL statement.
///* zName must be a pointer to a nul terminated string.
///*
///* This call must be made after a call to sqlite3VdbeSetNumCols().
///*
///* The final parameter, xDel, must be one of SQLITE_DYNAMIC, SQLITE_STATIC
///* or SQLITE_TRANSIENT. If it is SQLITE_DYNAMIC, then the buffer pointed
///* to by zName will be freed by sqlite3DbFree() when the vdbe is destroyed.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_set_col_name(p: &Vdbe, idx: i32, var: i32,
    z_name_1: *const i8, x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32 {
    let mut rc: i32 = 0;
    let mut p_col_name: *mut Mem = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*(*p).db).malloc_failed } != 0 { { let _ = 0; }; return 7; }
    { let _ = 0; };
    p_col_name =
        unsafe {
            (*p).a_col_name.offset((idx + var * (*p).n_res_alloc as i32) as
                    isize)
        };
    rc =
        unsafe {
            sqlite3_vdbe_mem_set_text(p_col_name, z_name_1, -1 as i64,
                x_del_1)
        };
    { let _ = 0; };
    return rc;
}

///* Set a flag in the vdbe to update the change counter when it is finalised
///* or reset.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_count_changes(v: &mut Vdbe) -> () {
    (*v).set_change_cnt_on(1 as Bft as u32);
}

///* Return the SQLITE_PREPARE flags for a Vdbe.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_prepare_flags(v: &Vdbe) -> u8 {
    return (*v).prep_flags;
}

///* Remember the SQL string for a prepared statement.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_set_sql(p: *mut Vdbe, z: *const i8, n: i32,
    prep_flags_1: u8) -> () {
    if p == core::ptr::null_mut() { return; }
    unsafe { (*p).prep_flags = prep_flags_1 };
    if prep_flags_1 as i32 & 128 == 0 { unsafe { (*p).expmask = 0 as u32 }; }
    { let _ = 0; };
    unsafe {
        (*p).z_sql =
            unsafe { sqlite3_db_str_n_dup(unsafe { (*p).db }, z, n as u64) }
    };
}

///* Swap byte-code between two VDBE structures.
///*
///* This happens after pB was previously run and returned
///* SQLITE_SCHEMA.  The statement was then reprepared in pA.
///* This routine transfers the new bytecode in pA over to pB
///* so that pB can be run again.  The old pB byte code is
///* moved back to pA so that it will be cleaned up when pA is
///* finalized.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_swap(p_a_1: &mut Vdbe, p_b_1: &mut Vdbe)
    -> () {
    let mut tmp: Vdbe = unsafe { core::mem::zeroed() };
    let mut p_tmp: *mut Vdbe = core::ptr::null_mut();
    let mut pp_tmp: *mut *mut Vdbe = core::ptr::null_mut();
    let mut z_tmp: *mut i8 = core::ptr::null_mut();
    { let _ = 0; };
    tmp = *p_a_1;
    *p_a_1 = *p_b_1;
    *p_b_1 = tmp;
    p_tmp = (*p_a_1).p_v_next;
    (*p_a_1).p_v_next = (*p_b_1).p_v_next;
    (*p_b_1).p_v_next = p_tmp;
    pp_tmp = (*p_a_1).pp_v_prev;
    (*p_a_1).pp_v_prev = (*p_b_1).pp_v_prev;
    (*p_b_1).pp_v_prev = pp_tmp;
    z_tmp = (*p_a_1).z_sql;
    (*p_a_1).z_sql = (*p_b_1).z_sql;
    (*p_b_1).z_sql = z_tmp;
    (*p_b_1).expmask = (*p_a_1).expmask;
    (*p_b_1).prep_flags = (*p_a_1).prep_flags;
    unsafe {
        memcpy(&raw mut (*p_b_1).a_counter[0 as usize] as *mut u32 as *mut (),
            &raw mut (*p_a_1).a_counter[0 as usize] as *mut u32 as *const (),
            core::mem::size_of::<[u32; 9]>() as u64)
    };
    {
        let __p = &mut (*p_b_1).a_counter[5 as usize];
        let __t = *__p;
        *__p += 1;
        __t
    };
}

///* This function returns a pointer to the array of opcodes associated with
///* the Vdbe passed as the first argument. It is the callers responsibility
///* to arrange for the returned array to be eventually freed using the
///* vdbeFreeOpArray() function.
///*
///* Before returning, *pnOp is set to the number of entries in the returned
///* array. Also, *pnMaxArg is set to the larger of its current value and
///* the number of entries in the Vdbe.apArg[] array required to execute the
///* returned program.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_take_op_array(p: *mut Vdbe, pn_op_1: &mut i32,
    pn_max_arg_1: *mut i32) -> *mut VdbeOp {
    let a_op: *mut VdbeOp = unsafe { (*p).a_op } as *mut VdbeOp;
    { let _ = 0; };

    /// Check that sqlite3VdbeUsesBtree() was not called on this VM
    { let _ = 0; };
    resolve_p2_values(unsafe { &mut *p }, unsafe { &mut *pn_max_arg_1 });
    *pn_op_1 = unsafe { (*p).n_op };
    unsafe { (*p).a_op = core::ptr::null_mut() };
    return a_op;
}

///* Return a pointer to an sqlite3_value structure containing the value bound
///* parameter iVar of VM v. Except, if the value is an SQL NULL, return
///* 0 instead. Unless it is NULL, apply affinity aff (one of the SQLITE_AFF_*
///* constants) to the value before returning it.
///*
///* The returned value must be freed by the caller using sqlite3ValueFree().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_get_bound_value(v: *const Vdbe, i_var_1: i32,
    aff: u8) -> *mut Sqlite3Value {
    { let _ = 0; };
    if !(v).is_null() {
        let p_mem: *const Mem =
            unsafe {
                    &raw mut *unsafe {
                                (*v).a_var.offset((i_var_1 - 1) as isize)
                            }
                } as *const Mem;
        { let _ = 0; };
        if 0 == unsafe { (*p_mem).flags } as i32 & 1 {
            let p_ret: *mut Sqlite3Value =
                unsafe { sqlite3_value_new(unsafe { (*v).db }) };
            if !(p_ret).is_null() {
                unsafe {
                    sqlite3_vdbe_mem_copy(p_ret as *mut Mem,
                        p_mem as *const Mem)
                };
                unsafe { sqlite3_value_apply_affinity(p_ret, aff, 1 as u8) };
            }
            return p_ret;
        }
    }
    return core::ptr::null_mut();
}

///* Configure SQL variable iVar so that binding a new value to it signals
///* to sqlite3_reoptimize() that re-preparing the statement may result
///* in a better query plan.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_set_varmask(v: &mut Vdbe, i_var_1: i32) -> () {
    { let _ = 0; };
    { let _ = 0; };
    if i_var_1 >= 32 {
        (*v).expmask |= 2147483648u32;
    } else { (*v).expmask |= (1 as u32) << i_var_1 - 1; }
}

///* Do a comparison between a 64-bit signed integer and a 64-bit floating-point
///* number.  Return negative, zero, or positive if the first (i64) is less than,
///* equal to, or greater than the second (double).
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_int_float_compare(i: i64, r: f64) -> i32 {
    if unsafe { sqlite3_is_na_n(r) } != 0 {

        /// SQLite considers NaN to be a NULL. And all integer values are greater
        ///* than NULL
        return 1;
    } else {
        let mut y: i64 = 0 as i64;
        if r < -9.223372036854776e18 { return 1; }
        if r >= 9.223372036854776e18 { return -1; }
        y = r as i64;
        if i < y { return -1; }
        if i > y { return 1; }
        return if (i as f64) < r { -1 } else { (i as f64 > r) as i32 };
    }
}

///* Both *pMem1 and *pMem2 contain string values. Compare the two values
///* using the collation sequence pColl. As usual, return a negative , zero
///* or positive value if *pMem1 is less than, equal to or greater than
///* *pMem2, respectively. Similar in spirit to "rc = (*pMem1) - (*pMem2);".
extern "C" fn vdbe_compare_mem_string_with_encoding_change(p_mem1_1:
        *const Mem, p_mem2_1: *const Mem, p_coll_1: &CollSeq,
    prc_err_1: *mut u8) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut v1: *const () = core::ptr::null();
        let mut v2: *const () = core::ptr::null();
        let mut c1: Mem = unsafe { core::mem::zeroed() };
        let mut c2: Mem = unsafe { core::mem::zeroed() };
        unsafe {
            sqlite3_vdbe_mem_init(&mut c1, unsafe { (*p_mem1_1).db },
                1 as u16)
        };
        unsafe {
            sqlite3_vdbe_mem_init(&mut c2, unsafe { (*p_mem1_1).db },
                1 as u16)
        };
        unsafe { sqlite3_vdbe_mem_shallow_copy(&mut c1, p_mem1_1, 16384) };
        unsafe { sqlite3_vdbe_mem_shallow_copy(&mut c2, p_mem2_1, 16384) };
        v1 =
            unsafe {
                sqlite3ValueText(&raw mut c1 as *mut Sqlite3Value,
                    (*p_coll_1).enc)
            };
        v2 =
            unsafe {
                sqlite3ValueText(&raw mut c2 as *mut Sqlite3Value,
                    (*p_coll_1).enc)
            };
        if v1 == core::ptr::null() || v2 == core::ptr::null() {
            if !(prc_err_1).is_null() { unsafe { *prc_err_1 = 7 as u8 }; }
            rc = 0;
        } else {
            rc =
                unsafe {
                    (*p_coll_1).x_cmp.unwrap()((*p_coll_1).p_user, c1.n, v1,
                        c2.n, v2)
                };
        }
        unsafe { sqlite3_vdbe_mem_release_malloc(&mut c1) };
        unsafe { sqlite3_vdbe_mem_release_malloc(&mut c2) };
        return rc;
    }
}

#[allow(unused_doc_comments)]
extern "C" fn vdbe_compare_mem_string(p_mem1_1: *const Mem,
    p_mem2_1: *const Mem, p_coll_1: *const CollSeq, prc_err_1: *mut u8)
    -> i32 {
    unsafe {
        if unsafe { (*p_mem1_1).enc } as i32 ==
                unsafe { (*p_coll_1).enc } as i32 {

            /// The strings are already in the correct encoding.  Call the
            ///* comparison function directly
            return unsafe {
                    (unsafe {
                            (*p_coll_1).x_cmp.unwrap()
                        })(unsafe { (*p_coll_1).p_user }, unsafe { (*p_mem1_1).n },
                        unsafe { (*p_mem1_1).z } as *const (),
                        unsafe { (*p_mem2_1).n },
                        unsafe { (*p_mem2_1).z } as *const ())
                };
        } else {
            return vdbe_compare_mem_string_with_encoding_change(p_mem1_1,
                    p_mem2_1, unsafe { &*p_coll_1 }, prc_err_1);
        }
    }
}

///* The input pBlob is guaranteed to be a Blob that is not marked
///* with MEM_Zero.  Return true if it could be a zero-blob.
extern "C" fn is_all_zero(z: &[i8]) -> i32 {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b30: loop {
            if !(i < z.len() as i32) { break '__b30; }
            '__c30: loop { if z[i as usize] != 0 { return 0; } break '__c30; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}

///* Compare two blobs.  Return negative, zero, or positive if the first
///* is less than, equal to, or greater than the second, respectively.
///* If one blob is a prefix of the other, then the shorter is the lessor.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_blob_compare(p_b1_1: &Mem, p_b2_1: &Mem) -> i32 {
    unsafe {
        let mut c: i32 = 0;
        let n1: i32 = (*p_b1_1).n as i32;
        let n2: i32 = (*p_b2_1).n as i32;

        /// It is possible to have a Blob value that has some non-zero content
        ///* followed by zero content.  But that only comes up for Blobs formed
        ///* by the OP_MakeRecord opcode, and such Blobs never get passed into
        ///* sqlite3MemCompare().
        { let _ = 0; };
        { let _ = 0; };
        if ((*p_b1_1).flags as i32 | (*p_b2_1).flags as i32) & 1024 != 0 {
            if (*p_b1_1).flags as i32 & (*p_b2_1).flags as i32 & 1024 != 0 {
                return (*p_b1_1).u.n_zero - (*p_b2_1).u.n_zero as i32;
            } else if (*p_b1_1).flags as i32 & 1024 != 0 {
                if (is_all_zero(unsafe {
                                        let __p = (*p_b2_1).z as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else {
                                            core::slice::from_raw_parts(__p, (*p_b2_1).n as usize)
                                        }
                                    }) == 0) as i32 != 0 {
                    return -1;
                }
                return (*p_b1_1).u.n_zero - n2;
            } else {
                if (is_all_zero(unsafe {
                                        let __p = (*p_b1_1).z as *const i8;
                                        if __p.is_null() {
                                            &[]
                                        } else {
                                            core::slice::from_raw_parts(__p, (*p_b1_1).n as usize)
                                        }
                                    }) == 0) as i32 != 0 {
                    return 1;
                }
                return n1 - (*p_b2_1).u.n_zero as i32;
            }
        }
        c =
            unsafe {
                memcmp((*p_b1_1).z as *const (), (*p_b2_1).z as *const (),
                    if n1 > n2 { n2 } else { n1 } as u64)
            };
        if c != 0 { return c; }
        return n1 - n2;
    }
}

///* Compare the values contained by the two memory cells, returning
///* negative, zero or positive if pMem1 is less than, equal to, or greater
///* than pMem2. Sorting order is NULL's first, followed by numbers (integers
///* and reals) sorted numerically, followed by text ordered by the collating
///* sequence pColl and finally blob's ordered by memcmp().
///*
///* Two NULL values are considered equal by this function.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_mem_compare(p_mem1_1: *const Mem,
    p_mem2_1: *const Mem, p_coll_1: *const CollSeq) -> i32 {
    unsafe {
        let mut f1: i32 = 0;
        let mut f2: i32 = 0;
        let mut combined_flags: i32 = 0;
        f1 = unsafe { (*p_mem1_1).flags } as i32;
        f2 = unsafe { (*p_mem2_1).flags } as i32;
        combined_flags = f1 | f2;
        { let _ = 0; };
        if combined_flags & 1 != 0 { return (f2 & 1) - (f1 & 1); }
        if combined_flags & (4 | 8 | 32) != 0 {
            if f1 & f2 & (4 | 32) != 0 {
                if (unsafe { (*p_mem1_1).u.i } as i64) <
                        unsafe { (*p_mem2_1).u.i } {
                    return -1;
                }
                if unsafe { (*p_mem1_1).u.i } as i64 >
                        unsafe { (*p_mem2_1).u.i } {
                    return 1;
                }
                return 0;
            }
            if f1 & f2 & 8 != 0 {
                if (unsafe { (*p_mem1_1).u.r } as f64) <
                        unsafe { (*p_mem2_1).u.r } {
                    return -1;
                }
                if unsafe { (*p_mem1_1).u.r } as f64 >
                        unsafe { (*p_mem2_1).u.r } {
                    return 1;
                }
                return 0;
            }
            if f1 & (4 | 32) != 0 {
                if f2 & 8 != 0 {
                    return sqlite3_int_float_compare(unsafe { (*p_mem1_1).u.i },
                            unsafe { (*p_mem2_1).u.r });
                } else if f2 & (4 | 32) != 0 {
                    if (unsafe { (*p_mem1_1).u.i } as i64) <
                            unsafe { (*p_mem2_1).u.i } {
                        return -1;
                    }
                    if unsafe { (*p_mem1_1).u.i } as i64 >
                            unsafe { (*p_mem2_1).u.i } {
                        return 1;
                    }
                    return 0;
                } else { return -1; }
            }
            if f1 & 8 != 0 {
                if f2 & (4 | 32) != 0 {
                    return -sqlite3_int_float_compare(unsafe {
                                    (*p_mem2_1).u.i
                                }, unsafe { (*p_mem1_1).u.r });
                } else { return -1; }
            }
            return 1;
        }
        if combined_flags & 2 != 0 {
            if f1 & 2 == 0 { return 1; }
            if f2 & 2 == 0 { return -1; }
            { let _ = 0; };
            { let _ = 0; };

            /// The collation sequence must be defined at this point, even if
            ///* the user deletes the collation sequence after the vdbe program is
            ///* compiled (this was not always the case).
            { let _ = 0; };
            if !(p_coll_1).is_null() {
                return vdbe_compare_mem_string(p_mem1_1, p_mem2_1, p_coll_1,
                        core::ptr::null_mut());
            }
        }

        /// Both values must be blobs.  Compare using memcmp().
        return sqlite3_blob_compare(unsafe { &*p_mem1_1 },
                unsafe { &*p_mem2_1 });
    }
}

///* Return the name of an SQL function associated with the sqlite3_context.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_func_name(p_ctx_1: &Sqlite3Context)
    -> *const i8 {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        return unsafe { (*(*p_ctx_1).p_func).z_name };
    }
}

///* Deserialize the data blob pointed to by buf as serial type serial_type
///* and store the result in pMem.
///*
///* This function is implemented as two separate routines for performance.
///* The few cases that require local variables are broken out into a separate
///* routine so that in most cases the overhead of moving the stack pointer
///* is avoided.
#[allow(unused_doc_comments)]
extern "C" fn serial_get(buf: *const u8, serial_type: u32, p_mem_1: &mut Mem)
    -> () {
    unsafe {
        let mut x: u64 =
            ((unsafe { *buf.offset(0 as isize) } as u32) << 24 |
                            ((unsafe { *buf.offset(1 as isize) } as i32) << 16) as u32 |
                        ((unsafe { *buf.offset(2 as isize) } as i32) << 8) as u32 |
                    unsafe { *buf.offset(3 as isize) } as u32) as u64;
        let y: u32 =
            (unsafe { *unsafe { buf.offset(4 as isize).offset(0 as isize) } }
                                    as u32) << 24 |
                        ((unsafe {
                                            *unsafe { buf.offset(4 as isize).offset(1 as isize) }
                                        } as i32) << 16) as u32 |
                    ((unsafe {
                                        *unsafe { buf.offset(4 as isize).offset(2 as isize) }
                                    } as i32) << 8) as u32 |
                unsafe {
                        *unsafe { buf.offset(4 as isize).offset(3 as isize) }
                    } as u32;
        x = (x << 32) + y as u64;
        if serial_type == 6 as u32 {

            /// EVIDENCE-OF: R-29851-52272 Value is a big-endian 64-bit
            ///* twos-complement integer.
            ((*p_mem_1).u.i = unsafe { *(&raw mut x as *mut i64) });

            /// EVIDENCE-OF: R-29851-52272 Value is a big-endian 64-bit
            ///* twos-complement integer.
            ((*p_mem_1).flags = 4 as u16);
        } else {

            /// EVIDENCE-OF: R-57343-49114 Value is a big-endian IEEE 754-2008 64-bit
            ///* floating point number.
            { let _ = 0; };
            unsafe {
                memcpy(&raw mut (*p_mem_1).u.r as *mut (),
                    &raw mut x as *const (), core::mem::size_of::<u64>() as u64)
            };
            (*p_mem_1).flags =
                if x & (2047 as u64) << 52 == (2047 as u64) << 52 &&
                            x & ((1 as u64) << 52) - 1 as u64 != 0 as u64 {
                        1
                    } else { 8 } as u16;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_serial_get(buf: *const u8, serial_type: u32,
    p_mem_1: *mut Mem) -> () {
    unsafe {
        '__s31:
            {
            match serial_type {
                10 => {
                    {

                        /// Internal use only: NULL with virtual table
                        ///* UPDATE no-change flag set
                        unsafe { (*p_mem_1).flags = (1 | 1024) as u16 };
                        unsafe { (*p_mem_1).n = 0 };
                        unsafe { (*p_mem_1).u.n_zero = 0 };
                        return;
                    }
                    {

                        /// Null */
                        ///      /* EVIDENCE-OF: R-24078-09375 Value is a NULL.
                        unsafe { (*p_mem_1).flags = 1 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// EVIDENCE-OF: R-44885-25196 Value is an 8-bit twos-complement
                            ///* integer.
                            ((*p_mem_1).u.i =
                                unsafe { *buf.offset(0 as isize) } as i8 as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 2-byte signed integer */
                            ///      /* EVIDENCE-OF: R-49794-35026 Value is a big-endian 16-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 3-byte signed integer */
                            ///      /* EVIDENCE-OF: R-37839-54301 Value is a big-endian 24-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (65536 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                            (unsafe { *buf.offset(1 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(2 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 4-byte signed integer */
                            ///      /* EVIDENCE-OF: R-01849-26079 Value is a big-endian 32-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (16777216 * unsafe { *buf.offset(0 as isize) } as i8 as i32
                                                | (unsafe { *buf.offset(1 as isize) } as i32) << 16 |
                                            (unsafe { *buf.offset(2 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(3 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 6-byte signed integer */
                            ///      /* EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(0 as isize) }
                                                                } as u32) << 24 |
                                                    ((unsafe {
                                                                        *unsafe { buf.offset(2 as isize).offset(1 as isize) }
                                                                    } as i32) << 16) as u32 |
                                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(2 as isize) }
                                                                } as i32) << 8) as u32 |
                                            unsafe {
                                                    *unsafe { buf.offset(2 as isize).offset(3 as isize) }
                                                } as u32) as i64 +
                                    ((1 as i64) << 32) *
                                        (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                                unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                11 => {
                    {

                        /// Null */
                        ///      /* EVIDENCE-OF: R-24078-09375 Value is a NULL.
                        unsafe { (*p_mem_1).flags = 1 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// EVIDENCE-OF: R-44885-25196 Value is an 8-bit twos-complement
                            ///* integer.
                            ((*p_mem_1).u.i =
                                unsafe { *buf.offset(0 as isize) } as i8 as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 2-byte signed integer */
                            ///      /* EVIDENCE-OF: R-49794-35026 Value is a big-endian 16-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 3-byte signed integer */
                            ///      /* EVIDENCE-OF: R-37839-54301 Value is a big-endian 24-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (65536 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                            (unsafe { *buf.offset(1 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(2 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 4-byte signed integer */
                            ///      /* EVIDENCE-OF: R-01849-26079 Value is a big-endian 32-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (16777216 * unsafe { *buf.offset(0 as isize) } as i8 as i32
                                                | (unsafe { *buf.offset(1 as isize) } as i32) << 16 |
                                            (unsafe { *buf.offset(2 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(3 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 6-byte signed integer */
                            ///      /* EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(0 as isize) }
                                                                } as u32) << 24 |
                                                    ((unsafe {
                                                                        *unsafe { buf.offset(2 as isize).offset(1 as isize) }
                                                                    } as i32) << 16) as u32 |
                                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(2 as isize) }
                                                                } as i32) << 8) as u32 |
                                            unsafe {
                                                    *unsafe { buf.offset(2 as isize).offset(3 as isize) }
                                                } as u32) as i64 +
                                    ((1 as i64) << 32) *
                                        (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                                unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                0 => {
                    {

                        /// Null */
                        ///      /* EVIDENCE-OF: R-24078-09375 Value is a NULL.
                        unsafe { (*p_mem_1).flags = 1 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// EVIDENCE-OF: R-44885-25196 Value is an 8-bit twos-complement
                            ///* integer.
                            ((*p_mem_1).u.i =
                                unsafe { *buf.offset(0 as isize) } as i8 as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 2-byte signed integer */
                            ///      /* EVIDENCE-OF: R-49794-35026 Value is a big-endian 16-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 3-byte signed integer */
                            ///      /* EVIDENCE-OF: R-37839-54301 Value is a big-endian 24-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (65536 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                            (unsafe { *buf.offset(1 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(2 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 4-byte signed integer */
                            ///      /* EVIDENCE-OF: R-01849-26079 Value is a big-endian 32-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (16777216 * unsafe { *buf.offset(0 as isize) } as i8 as i32
                                                | (unsafe { *buf.offset(1 as isize) } as i32) << 16 |
                                            (unsafe { *buf.offset(2 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(3 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 6-byte signed integer */
                            ///      /* EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(0 as isize) }
                                                                } as u32) << 24 |
                                                    ((unsafe {
                                                                        *unsafe { buf.offset(2 as isize).offset(1 as isize) }
                                                                    } as i32) << 16) as u32 |
                                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(2 as isize) }
                                                                } as i32) << 8) as u32 |
                                            unsafe {
                                                    *unsafe { buf.offset(2 as isize).offset(3 as isize) }
                                                } as u32) as i64 +
                                    ((1 as i64) << 32) *
                                        (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                                unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                1 => {
                    {
                        unsafe {

                            /// EVIDENCE-OF: R-44885-25196 Value is an 8-bit twos-complement
                            ///* integer.
                            ((*p_mem_1).u.i =
                                unsafe { *buf.offset(0 as isize) } as i8 as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 2-byte signed integer */
                            ///      /* EVIDENCE-OF: R-49794-35026 Value is a big-endian 16-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 3-byte signed integer */
                            ///      /* EVIDENCE-OF: R-37839-54301 Value is a big-endian 24-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (65536 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                            (unsafe { *buf.offset(1 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(2 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 4-byte signed integer */
                            ///      /* EVIDENCE-OF: R-01849-26079 Value is a big-endian 32-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (16777216 * unsafe { *buf.offset(0 as isize) } as i8 as i32
                                                | (unsafe { *buf.offset(1 as isize) } as i32) << 16 |
                                            (unsafe { *buf.offset(2 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(3 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 6-byte signed integer */
                            ///      /* EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(0 as isize) }
                                                                } as u32) << 24 |
                                                    ((unsafe {
                                                                        *unsafe { buf.offset(2 as isize).offset(1 as isize) }
                                                                    } as i32) << 16) as u32 |
                                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(2 as isize) }
                                                                } as i32) << 8) as u32 |
                                            unsafe {
                                                    *unsafe { buf.offset(2 as isize).offset(3 as isize) }
                                                } as u32) as i64 +
                                    ((1 as i64) << 32) *
                                        (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                                unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                2 => {
                    {
                        unsafe {

                            /// 2-byte signed integer */
                            ///      /* EVIDENCE-OF: R-49794-35026 Value is a big-endian 16-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 3-byte signed integer */
                            ///      /* EVIDENCE-OF: R-37839-54301 Value is a big-endian 24-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (65536 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                            (unsafe { *buf.offset(1 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(2 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 4-byte signed integer */
                            ///      /* EVIDENCE-OF: R-01849-26079 Value is a big-endian 32-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (16777216 * unsafe { *buf.offset(0 as isize) } as i8 as i32
                                                | (unsafe { *buf.offset(1 as isize) } as i32) << 16 |
                                            (unsafe { *buf.offset(2 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(3 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 6-byte signed integer */
                            ///      /* EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(0 as isize) }
                                                                } as u32) << 24 |
                                                    ((unsafe {
                                                                        *unsafe { buf.offset(2 as isize).offset(1 as isize) }
                                                                    } as i32) << 16) as u32 |
                                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(2 as isize) }
                                                                } as i32) << 8) as u32 |
                                            unsafe {
                                                    *unsafe { buf.offset(2 as isize).offset(3 as isize) }
                                                } as u32) as i64 +
                                    ((1 as i64) << 32) *
                                        (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                                unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                3 => {
                    {
                        unsafe {

                            /// 3-byte signed integer */
                            ///      /* EVIDENCE-OF: R-37839-54301 Value is a big-endian 24-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (65536 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                            (unsafe { *buf.offset(1 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(2 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 4-byte signed integer */
                            ///      /* EVIDENCE-OF: R-01849-26079 Value is a big-endian 32-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (16777216 * unsafe { *buf.offset(0 as isize) } as i8 as i32
                                                | (unsafe { *buf.offset(1 as isize) } as i32) << 16 |
                                            (unsafe { *buf.offset(2 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(3 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 6-byte signed integer */
                            ///      /* EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(0 as isize) }
                                                                } as u32) << 24 |
                                                    ((unsafe {
                                                                        *unsafe { buf.offset(2 as isize).offset(1 as isize) }
                                                                    } as i32) << 16) as u32 |
                                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(2 as isize) }
                                                                } as i32) << 8) as u32 |
                                            unsafe {
                                                    *unsafe { buf.offset(2 as isize).offset(3 as isize) }
                                                } as u32) as i64 +
                                    ((1 as i64) << 32) *
                                        (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                                unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                4 => {
                    {
                        unsafe {

                            /// 4-byte signed integer */
                            ///      /* EVIDENCE-OF: R-01849-26079 Value is a big-endian 32-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                (16777216 * unsafe { *buf.offset(0 as isize) } as i8 as i32
                                                | (unsafe { *buf.offset(1 as isize) } as i32) << 16 |
                                            (unsafe { *buf.offset(2 as isize) } as i32) << 8 |
                                        unsafe { *buf.offset(3 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe {

                            /// 6-byte signed integer */
                            ///      /* EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(0 as isize) }
                                                                } as u32) << 24 |
                                                    ((unsafe {
                                                                        *unsafe { buf.offset(2 as isize).offset(1 as isize) }
                                                                    } as i32) << 16) as u32 |
                                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(2 as isize) }
                                                                } as i32) << 8) as u32 |
                                            unsafe {
                                                    *unsafe { buf.offset(2 as isize).offset(3 as isize) }
                                                } as u32) as i64 +
                                    ((1 as i64) << 32) *
                                        (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                                unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                5 => {
                    {
                        unsafe {

                            /// 6-byte signed integer */
                            ///      /* EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
                            ///* twos-complement integer.
                            ((*p_mem_1).u.i =
                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(0 as isize) }
                                                                } as u32) << 24 |
                                                    ((unsafe {
                                                                        *unsafe { buf.offset(2 as isize).offset(1 as isize) }
                                                                    } as i32) << 16) as u32 |
                                                ((unsafe {
                                                                    *unsafe { buf.offset(2 as isize).offset(2 as isize) }
                                                                } as i32) << 8) as u32 |
                                            unsafe {
                                                    *unsafe { buf.offset(2 as isize).offset(3 as isize) }
                                                } as u32) as i64 +
                                    ((1 as i64) << 32) *
                                        (256 * unsafe { *buf.offset(0 as isize) } as i8 as i32 |
                                                unsafe { *buf.offset(1 as isize) } as i32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                6 => {
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                7 => {
                    {

                        /// IEEE floating point */
                        ///      /* These use local variables, so do them in a separate routine
                        ///* to avoid having to move the frame pointer in the common case
                        serial_get(buf, serial_type, unsafe { &mut *p_mem_1 });
                        return;
                    }
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                8 => {
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                9 => {
                    {
                        unsafe {

                            /// Integer 1 */
                            ///      /* EVIDENCE-OF: R-12976-22893 Value is the integer 0. */
                            ///      /* EVIDENCE-OF: R-18143-12121 Value is the integer 1.
                            ((*p_mem_1).u.i = (serial_type - 8 as u32) as i64)
                        };
                        unsafe { (*p_mem_1).flags = 4 as u16 };
                        return;
                    }
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
                _ => {
                    {
                        unsafe { (*p_mem_1).z = buf as *mut i8 };
                        unsafe {
                            (*p_mem_1).n = ((serial_type - 12 as u32) / 2 as u32) as i32
                        };
                        unsafe {
                            (*p_mem_1).flags =
                                a_flag[(serial_type & 1 as u32) as usize] as u16
                        };
                        return;
                    }
                }
            }
        }
        return;
    }
}

///* The sizes for serial types less than 128
#[unsafe(no_mangle)]
pub static sqlite3_small_type_sizes: [u8; 128] =
    [0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 6 as u8, 8 as u8, 8 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 1 as u8,
            1 as u8, 2 as u8, 2 as u8, 3 as u8, 3 as u8, 4 as u8, 4 as u8,
            5 as u8, 5 as u8, 6 as u8, 6 as u8, 7 as u8, 7 as u8, 8 as u8,
            8 as u8, 9 as u8, 9 as u8, 10 as u8, 10 as u8, 11 as u8, 11 as u8,
            12 as u8, 12 as u8, 13 as u8, 13 as u8, 14 as u8, 14 as u8,
            15 as u8, 15 as u8, 16 as u8, 16 as u8, 17 as u8, 17 as u8,
            18 as u8, 18 as u8, 19 as u8, 19 as u8, 20 as u8, 20 as u8,
            21 as u8, 21 as u8, 22 as u8, 22 as u8, 23 as u8, 23 as u8,
            24 as u8, 24 as u8, 25 as u8, 25 as u8, 26 as u8, 26 as u8,
            27 as u8, 27 as u8, 28 as u8, 28 as u8, 29 as u8, 29 as u8,
            30 as u8, 30 as u8, 31 as u8, 31 as u8, 32 as u8, 32 as u8,
            33 as u8, 33 as u8, 34 as u8, 34 as u8, 35 as u8, 35 as u8,
            36 as u8, 36 as u8, 37 as u8, 37 as u8, 38 as u8, 38 as u8,
            39 as u8, 39 as u8, 40 as u8, 40 as u8, 41 as u8, 41 as u8,
            42 as u8, 42 as u8, 43 as u8, 43 as u8, 44 as u8, 44 as u8,
            45 as u8, 45 as u8, 46 as u8, 46 as u8, 47 as u8, 47 as u8,
            48 as u8, 48 as u8, 49 as u8, 49 as u8, 50 as u8, 50 as u8,
            51 as u8, 51 as u8, 52 as u8, 52 as u8, 53 as u8, 53 as u8,
            54 as u8, 54 as u8, 55 as u8, 55 as u8, 56 as u8, 56 as u8,
            57 as u8, 57 as u8];

///* Return the length of the data corresponding to the supplied serial-type.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_serial_type_len(serial_type: u32) -> u32 {
    if serial_type >= 128 as u32 {
        return (serial_type - 12 as u32) / 2 as u32;
    } else {
        { let _ = 0; };
        return sqlite3_small_type_sizes[serial_type as usize] as u32;
    }
}

///* Given the nKey-byte encoding of a record in pKey[], populate the
///* UnpackedRecord structure indicated by the fourth argument with the
///* contents of the decoded record.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_record_unpack(n_key_1: i32, p_key_1: *const (),
    p: &mut UnpackedRecord) -> () {
    unsafe {
        let a_key: *const u8 = p_key_1 as *const u8;
        let mut d: u32 = 0 as u32;
        let mut idx: u32 = 0 as u32;
        /// Offset in aKey[] to read from
        let mut u: u16 = 0 as u16;
        /// Unsigned loop counter
        let mut sz_hdr: u32 = 0 as u32;
        let mut p_mem: *mut Mem = (*p).a_mem;
        let p_key_info: *const KeyInfo = (*p).p_key_info as *const KeyInfo;
        (*p).default_rc = 0 as i8;
        { let _ = 0; };
        idx =
            if (unsafe { *a_key } as i32) < 128 as u8 as i32 {
                        {
                            ({ sz_hdr = unsafe { *a_key } as u32; sz_hdr }) as i32;
                            1
                        }
                    } else {
                        (unsafe {
                                sqlite3_get_varint32(a_key, &raw mut sz_hdr as *mut u32)
                            }) as i32
                    } as u8 as u32;
        d = sz_hdr;
        u = 0 as u16;
        while idx < sz_hdr && d <= n_key_1 as u32 {
            let mut serial_type: u32 = 0 as u32;
            idx +=
                if (unsafe { *unsafe { &*a_key.add(idx as usize) } } as i32) <
                                128 as u8 as i32 {
                            {
                                ({
                                        serial_type =
                                            unsafe { *unsafe { &*a_key.add(idx as usize) } } as u32;
                                        serial_type
                                    }) as i32;
                                1
                            }
                        } else {
                            (unsafe {
                                    sqlite3_get_varint32(unsafe { &*a_key.add(idx as usize) },
                                        &raw mut serial_type as *mut u32)
                                }) as i32
                        } as u8 as u32;
            unsafe { (*p_mem).enc = unsafe { (*p_key_info).enc } };
            unsafe { (*p_mem).db = unsafe { (*p_key_info).db } };

            /// pMem->flags = 0; // sqlite3VdbeSerialGet() will set this for us
            unsafe { (*p_mem).sz_malloc = 0 };
            unsafe { (*p_mem).z = core::ptr::null_mut() };
            sqlite3_vdbe_serial_get(unsafe { &*a_key.add(d as usize) },
                serial_type, p_mem);
            d += sqlite3_vdbe_serial_type_len(serial_type);
            if { let __p = &mut u; *__p += 1; *__p } as i32 >=
                    (*p).n_field as i32 {
                break;
            }
            {
                let __p = &mut p_mem;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if d > n_key_1 as u32 && u != 0 {
            { let _ = 0; };

            /// In a corrupt record entry, the last pMem might have been set up using
            ///* uninitialized memory. Overwrite its value with NULL, to prevent
            ///* warnings from MSAN.
            unsafe {
                sqlite3_vdbe_mem_set_null(unsafe {
                        p_mem.offset(-(((u as i32) < (*p).n_field as i32) as isize))
                    })
            };
        }
        { let _ = 0; };
        (*p).n_field = u;
    }
}

extern "C" fn serial_get7(buf: *const u8, p_mem_1: &mut Mem) -> i32 {
    unsafe {
        let mut x: u64 =
            ((unsafe { *buf.offset(0 as isize) } as u32) << 24 |
                            ((unsafe { *buf.offset(1 as isize) } as i32) << 16) as u32 |
                        ((unsafe { *buf.offset(2 as isize) } as i32) << 8) as u32 |
                    unsafe { *buf.offset(3 as isize) } as u32) as u64;
        let y: u32 =
            (unsafe { *unsafe { buf.offset(4 as isize).offset(0 as isize) } }
                                    as u32) << 24 |
                        ((unsafe {
                                            *unsafe { buf.offset(4 as isize).offset(1 as isize) }
                                        } as i32) << 16) as u32 |
                    ((unsafe {
                                        *unsafe { buf.offset(4 as isize).offset(2 as isize) }
                                    } as i32) << 8) as u32 |
                unsafe {
                        *unsafe { buf.offset(4 as isize).offset(3 as isize) }
                    } as u32;
        x = (x << 32) + y as u64;
        { let _ = 0; };
        unsafe {
            memcpy(&raw mut (*p_mem_1).u.r as *mut (),
                &raw mut x as *const (), core::mem::size_of::<u64>() as u64)
        };
        if x & (2047 as u64) << 52 == (2047 as u64) << 52 &&
                x & ((1 as u64) << 52) - 1 as u64 != 0 as u64 {
            (*p_mem_1).flags = 1 as u16;
            return 1;
        }
        (*p_mem_1).flags = 8 as u16;
        return 0;
    }
}

///* The first argument passed to this function is a serial-type that
///* corresponds to an integer - all values between 1 and 9 inclusive
///* except 7. The second points to a buffer containing an integer value
///* serialized according to serial_type. This function deserializes
///* and returns the value.
extern "C" fn vdbe_record_decode_int(serial_type: u32, a_key_1: *const u8)
    -> i64 {
    let mut y: u32 = 0 as u32;
    { let _ = 0; };
    '__s33:
        {
        match serial_type {
            0 => {
                return unsafe { *a_key_1.offset(0 as isize) } as i8 as i64;
                return (256 *
                                unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                            unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                return (65536 *
                                    unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                (unsafe { *a_key_1.offset(1 as isize) } as i32) << 8 |
                            unsafe { *a_key_1.offset(2 as isize) } as i32) as i64;
                {
                    y =
                        (unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                    ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                        u32 |
                                ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                    u32 | unsafe { *a_key_1.offset(3 as isize) } as u32;
                    return unsafe { *(&raw mut y as *mut i32) } as i64;
                }
                {
                    return ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(2 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(2 as isize).offset(3 as isize) }
                                        } as u32) as i64 +
                            ((1 as i64) << 32) *
                                (256 * unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                }
                {
                    let mut x: u64 =
                        ((unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                            u32 |
                                    ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                        u32 | unsafe { *a_key_1.offset(3 as isize) } as u32) as u64;
                    x =
                        x << 32 |
                            ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(4 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(4 as isize).offset(3 as isize) }
                                        } as u32) as u64;
                    return unsafe { *(&raw mut x as *mut i64) } as i64;
                }
            }
            1 => {
                return unsafe { *a_key_1.offset(0 as isize) } as i8 as i64;
                return (256 *
                                unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                            unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                return (65536 *
                                    unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                (unsafe { *a_key_1.offset(1 as isize) } as i32) << 8 |
                            unsafe { *a_key_1.offset(2 as isize) } as i32) as i64;
                {
                    y =
                        (unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                    ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                        u32 |
                                ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                    u32 | unsafe { *a_key_1.offset(3 as isize) } as u32;
                    return unsafe { *(&raw mut y as *mut i32) } as i64;
                }
                {
                    return ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(2 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(2 as isize).offset(3 as isize) }
                                        } as u32) as i64 +
                            ((1 as i64) << 32) *
                                (256 * unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                }
                {
                    let mut x: u64 =
                        ((unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                            u32 |
                                    ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                        u32 | unsafe { *a_key_1.offset(3 as isize) } as u32) as u64;
                    x =
                        x << 32 |
                            ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(4 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(4 as isize).offset(3 as isize) }
                                        } as u32) as u64;
                    return unsafe { *(&raw mut x as *mut i64) } as i64;
                }
            }
            2 => {
                return (256 *
                                unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                            unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                return (65536 *
                                    unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                (unsafe { *a_key_1.offset(1 as isize) } as i32) << 8 |
                            unsafe { *a_key_1.offset(2 as isize) } as i32) as i64;
                {
                    y =
                        (unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                    ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                        u32 |
                                ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                    u32 | unsafe { *a_key_1.offset(3 as isize) } as u32;
                    return unsafe { *(&raw mut y as *mut i32) } as i64;
                }
                {
                    return ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(2 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(2 as isize).offset(3 as isize) }
                                        } as u32) as i64 +
                            ((1 as i64) << 32) *
                                (256 * unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                }
                {
                    let mut x: u64 =
                        ((unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                            u32 |
                                    ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                        u32 | unsafe { *a_key_1.offset(3 as isize) } as u32) as u64;
                    x =
                        x << 32 |
                            ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(4 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(4 as isize).offset(3 as isize) }
                                        } as u32) as u64;
                    return unsafe { *(&raw mut x as *mut i64) } as i64;
                }
            }
            3 => {
                return (65536 *
                                    unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                (unsafe { *a_key_1.offset(1 as isize) } as i32) << 8 |
                            unsafe { *a_key_1.offset(2 as isize) } as i32) as i64;
                {
                    y =
                        (unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                    ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                        u32 |
                                ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                    u32 | unsafe { *a_key_1.offset(3 as isize) } as u32;
                    return unsafe { *(&raw mut y as *mut i32) } as i64;
                }
                {
                    return ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(2 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(2 as isize).offset(3 as isize) }
                                        } as u32) as i64 +
                            ((1 as i64) << 32) *
                                (256 * unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                }
                {
                    let mut x: u64 =
                        ((unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                            u32 |
                                    ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                        u32 | unsafe { *a_key_1.offset(3 as isize) } as u32) as u64;
                    x =
                        x << 32 |
                            ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(4 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(4 as isize).offset(3 as isize) }
                                        } as u32) as u64;
                    return unsafe { *(&raw mut x as *mut i64) } as i64;
                }
            }
            4 => {
                {
                    y =
                        (unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                    ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                        u32 |
                                ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                    u32 | unsafe { *a_key_1.offset(3 as isize) } as u32;
                    return unsafe { *(&raw mut y as *mut i32) } as i64;
                }
                {
                    return ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(2 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(2 as isize).offset(3 as isize) }
                                        } as u32) as i64 +
                            ((1 as i64) << 32) *
                                (256 * unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                }
                {
                    let mut x: u64 =
                        ((unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                            u32 |
                                    ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                        u32 | unsafe { *a_key_1.offset(3 as isize) } as u32) as u64;
                    x =
                        x << 32 |
                            ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(4 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(4 as isize).offset(3 as isize) }
                                        } as u32) as u64;
                    return unsafe { *(&raw mut x as *mut i64) } as i64;
                }
            }
            5 => {
                {
                    return ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(2 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(2 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(2 as isize).offset(3 as isize) }
                                        } as u32) as i64 +
                            ((1 as i64) << 32) *
                                (256 * unsafe { *a_key_1.offset(0 as isize) } as i8 as i32 |
                                        unsafe { *a_key_1.offset(1 as isize) } as i32) as i64;
                }
                {
                    let mut x: u64 =
                        ((unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                            u32 |
                                    ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                        u32 | unsafe { *a_key_1.offset(3 as isize) } as u32) as u64;
                    x =
                        x << 32 |
                            ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(4 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(4 as isize).offset(3 as isize) }
                                        } as u32) as u64;
                    return unsafe { *(&raw mut x as *mut i64) } as i64;
                }
            }
            6 => {
                {
                    let mut x: u64 =
                        ((unsafe { *a_key_1.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key_1.offset(1 as isize) } as i32) << 16) as
                                            u32 |
                                    ((unsafe { *a_key_1.offset(2 as isize) } as i32) << 8) as
                                        u32 | unsafe { *a_key_1.offset(3 as isize) } as u32) as u64;
                    x =
                        x << 32 |
                            ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(0 as isize) }
                                                        } as u32) << 24 |
                                            ((unsafe {
                                                                *unsafe { a_key_1.offset(4 as isize).offset(1 as isize) }
                                                            } as i32) << 16) as u32 |
                                        ((unsafe {
                                                            *unsafe { a_key_1.offset(4 as isize).offset(2 as isize) }
                                                        } as i32) << 8) as u32 |
                                    unsafe {
                                            *unsafe { a_key_1.offset(4 as isize).offset(3 as isize) }
                                        } as u32) as u64;
                    return unsafe { *(&raw mut x as *mut i64) } as i64;
                }
            }
            _ => {}
        }
    }
    return (serial_type - 8 as u32) as i64;
}

///* This function compares the two table rows or index records
///* specified by {nKey1, pKey1} and pPKey2.  It returns a negative, zero
///* or positive integer if key1 is less than, equal to or
///* greater than key2.  The {nKey1, pKey1} key must be a blob
///* created by the OP_MakeRecord opcode of the VDBE.  The pPKey2
///* key must be a parsed key such as obtained from
///* sqlite3VdbeParseRecord.
///*
///* If argument bSkip is non-zero, it is assumed that the caller has already
///* determined that the first fields of the keys are equal.
///*
///* Key1 and Key2 do not have to contain the same number of fields. If all
///* fields that appear in both keys are equal, then pPKey2->default_rc is
///* returned.
///*
///* If database corruption is discovered, set pPKey2->errCode to
///* SQLITE_CORRUPT and return 0. If an OOM error is encountered,
///* pPKey2->errCode is set to SQLITE_NOMEM and, if it is not NULL, the
///* malloc-failed flag set on database handle (pPKey2->pKeyInfo->db).
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_record_compare_with_skip(n_key1_1: i32,
    p_key1_1: *const (), p_p_key2_1: &mut UnpackedRecord, b_skip_1: i32)
    -> i32 {
    unsafe {
        let mut d1: u32 = 0 as u32;
        /// Offset into aKey[] of next data element
        let mut i: i32 = 0;
        /// Index of next field to compare
        let mut sz_hdr1: u32 = 0 as u32;
        /// Size of record header in bytes
        let mut idx1: u32 = 0 as u32;
        /// Offset of first type in header
        let mut rc: i32 = 0;
        /// Return value
        let mut p_rhs: *const Mem = (*p_p_key2_1).a_mem as *const Mem;
        /// Next field of pPKey2 to compare
        let mut p_key_info: *const KeyInfo = core::ptr::null();
        let a_key1: *const u8 = p_key1_1 as *const u8;
        let mut mem1: Mem = unsafe { core::mem::zeroed() };
        if b_skip_1 != 0 {
            let mut s1: u32 = unsafe { *a_key1.offset(1 as isize) } as u32;
            if s1 < 128 as u32 {
                idx1 = 2 as u32;
            } else {
                idx1 =
                    (1 +
                            unsafe {
                                    sqlite3_get_varint32(unsafe { &*a_key1.offset(1 as isize) },
                                        &mut s1)
                                } as i32) as u32;
            }
            sz_hdr1 = unsafe { *a_key1.offset(0 as isize) } as u32;
            d1 = sz_hdr1 + sqlite3_vdbe_serial_type_len(s1);
            i = 1;
            {
                let __p = &mut p_rhs;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else {
            if {
                        sz_hdr1 = unsafe { *a_key1.offset(0 as isize) } as u32;
                        sz_hdr1
                    } < 128 as u32 {
                idx1 = 1 as u32;
            } else {
                idx1 =
                    unsafe { sqlite3_get_varint32(a_key1, &mut sz_hdr1) } as
                        u32;
            }
            d1 = sz_hdr1;
            i = 0;
        }
        if d1 > n_key1_1 as u32 {
            (*p_p_key2_1).err_code =
                unsafe { sqlite3_corrupt_error(4779) } as u8;
            return 0;
        }

        /// Only needed by assert() statements
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        loop {
            let mut serial_type: u32 = 0 as u32;
            if unsafe { (*p_rhs).flags } as i32 & (4 | 32) != 0 {
                serial_type = unsafe { *a_key1.add(idx1 as usize) } as u32;
                if serial_type >= 10 as u32 {
                    rc = if serial_type == 10 as u32 { -1 } else { 1 };
                } else if serial_type == 0 as u32 {
                    rc = -1;
                } else if serial_type == 7 as u32 {
                    serial_get7(unsafe { &*a_key1.add(d1 as usize) },
                        &mut mem1);
                    rc =
                        -sqlite3_int_float_compare(unsafe { (*p_rhs).u.i },
                                mem1.u.r);
                } else {
                    let lhs: i64 =
                        vdbe_record_decode_int(serial_type,
                            unsafe { &*a_key1.add(d1 as usize) });
                    let rhs: i64 = unsafe { (*p_rhs).u.i };
                    if lhs < rhs { rc = -1; } else if lhs > rhs { rc = 1; }
                }
            } else if unsafe { (*p_rhs).flags } as i32 & 8 != 0 {
                serial_type = unsafe { *a_key1.add(idx1 as usize) } as u32;
                if serial_type >= 10 as u32 {

                    /// Serial types 12 or greater are strings and blobs (greater than
                    ///* numbers). Types 10 and 11 are currently "reserved for future
                    ///* use", so it doesn't really matter what the results of comparing
                    ///* them to numeric values are.
                    (rc = if serial_type == 10 as u32 { -1 } else { 1 });
                } else if serial_type == 0 as u32 {
                    rc = -1;
                } else {
                    if serial_type == 7 as u32 {
                        if serial_get7(unsafe { &*a_key1.add(d1 as usize) },
                                    &mut mem1) != 0 {
                            rc = -1;
                        } else if mem1.u.r < unsafe { (*p_rhs).u.r } {
                            rc = -1;
                        } else if mem1.u.r > unsafe { (*p_rhs).u.r } {
                            rc = 1;
                        } else { { let _ = 0; }; }
                    } else {
                        sqlite3_vdbe_serial_get(unsafe {
                                &*a_key1.add(d1 as usize)
                            }, serial_type, &mut mem1);
                        rc =
                            sqlite3_int_float_compare(mem1.u.i,
                                unsafe { (*p_rhs).u.r });
                    }
                }
            } else if unsafe { (*p_rhs).flags } as i32 & 2 != 0 {
                serial_type =
                    unsafe { *unsafe { &*a_key1.add(idx1 as usize) } } as u32;
                if serial_type >= 128 as u32 {
                    unsafe {
                        sqlite3_get_varint32(unsafe { &*a_key1.add(idx1 as usize) },
                            &raw mut serial_type as *mut u32)
                    };
                }
                if serial_type < 12 as u32 {
                    rc = -1;
                } else if (serial_type & 1 as u32 == 0) as i32 != 0 {
                    rc = 1;
                } else {
                    mem1.n = ((serial_type - 12 as u32) / 2 as u32) as i32;
                    if d1 + mem1.n as u32 > n_key1_1 as u32 ||
                            unsafe {
                                        (*{
                                                        p_key_info = (*p_p_key2_1).p_key_info;
                                                        p_key_info
                                                    }).n_all_field
                                    } as i32 <= i {
                        (*p_p_key2_1).err_code =
                            unsafe { sqlite3_corrupt_error(4860) } as u8;
                        return 0;
                    } else if !(unsafe {
                                        *(unsafe { (*p_key_info).a_coll.as_ptr() } as
                                                    *mut *mut CollSeq).offset(i as isize)
                                    }).is_null() {
                        mem1.enc = unsafe { (*p_key_info).enc };
                        mem1.db = unsafe { (*p_key_info).db };
                        mem1.flags = 2 as u16;
                        mem1.z =
                            unsafe { &raw const *a_key1.add(d1 as usize) } as *mut i8;
                        rc =
                            vdbe_compare_mem_string(&raw mut mem1 as *const Mem,
                                p_rhs as *const Mem,
                                unsafe {
                                        *(unsafe { (*p_key_info).a_coll.as_ptr() } as
                                                    *mut *mut CollSeq).offset(i as isize)
                                    } as *const CollSeq, &mut (*p_p_key2_1).err_code);
                    } else {
                        let n_cmp: i32 =
                            if mem1.n < unsafe { (*p_rhs).n } {
                                mem1.n
                            } else { unsafe { (*p_rhs).n } };
                        rc =
                            unsafe {
                                memcmp(unsafe { &raw const *a_key1.add(d1 as usize) } as
                                        *const (), unsafe { (*p_rhs).z } as *const (), n_cmp as u64)
                            };
                        if rc == 0 { rc = mem1.n - unsafe { (*p_rhs).n }; }
                    }
                }
            } else if unsafe { (*p_rhs).flags } as i32 & 16 != 0 {
                { let _ = 0; };
                serial_type =
                    unsafe { *unsafe { &*a_key1.add(idx1 as usize) } } as u32;
                if serial_type >= 128 as u32 {
                    unsafe {
                        sqlite3_get_varint32(unsafe { &*a_key1.add(idx1 as usize) },
                            &raw mut serial_type as *mut u32)
                    };
                }
                if serial_type < 12 as u32 || serial_type & 1 as u32 != 0 {
                    rc = -1;
                } else {
                    let n_str: i32 =
                        ((serial_type - 12 as u32) / 2 as u32) as i32;
                    if d1 + n_str as u32 > n_key1_1 as u32 {
                        (*p_p_key2_1).err_code =
                            unsafe { sqlite3_corrupt_error(4890) } as u8;
                        return 0;
                    } else if unsafe { (*p_rhs).flags } as i32 & 1024 != 0 {
                        if (is_all_zero(unsafe {
                                                let __p =
                                                    unsafe { &raw const *a_key1.add(d1 as usize) } as *const i8
                                                        as *const i8;
                                                if __p.is_null() {
                                                    &[]
                                                } else { core::slice::from_raw_parts(__p, n_str as usize) }
                                            }) == 0) as i32 != 0 {
                            rc = 1;
                        } else { rc = n_str - unsafe { (*p_rhs).u.n_zero }; }
                    } else {
                        let n_cmp_1: i32 =
                            if n_str < unsafe { (*p_rhs).n } {
                                n_str
                            } else { unsafe { (*p_rhs).n } };
                        rc =
                            unsafe {
                                memcmp(unsafe { &raw const *a_key1.add(d1 as usize) } as
                                        *const (), unsafe { (*p_rhs).z } as *const (),
                                    n_cmp_1 as u64)
                            };
                        if rc == 0 { rc = n_str - unsafe { (*p_rhs).n }; }
                    }
                }
            } else {
                serial_type = unsafe { *a_key1.add(idx1 as usize) } as u32;
                if serial_type == 0 as u32 || serial_type == 10 as u32 ||
                        serial_type == 7 as u32 &&
                            serial_get7(unsafe { &*a_key1.add(d1 as usize) }, &mut mem1)
                                != 0 {
                    { let _ = 0; };
                } else { rc = 1; }
            }
            if rc != 0 {
                let sort_flags: i32 =
                    unsafe {
                            *unsafe {
                                    (*(*p_p_key2_1).p_key_info).a_sort_flags.offset(i as isize)
                                }
                        } as i32;
                if sort_flags != 0 {
                    if sort_flags & 2 == 0 ||
                            sort_flags & 1 !=
                                (serial_type == 0 as u32 ||
                                        unsafe { (*p_rhs).flags } as i32 & 1 != 0) as i32 {
                        rc = -rc;
                    }
                }
                { let _ = 0; };
                { let _ = 0; };

                /// See comment below
                return rc;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            if i == (*p_p_key2_1).n_field as i32 { break; }
            {
                let __p = &mut p_rhs;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            d1 += sqlite3_vdbe_serial_type_len(serial_type);
            if d1 > n_key1_1 as u32 { break; }
            idx1 += unsafe { sqlite3_varint_len(serial_type as u64) } as u32;
            if idx1 >= sz_hdr1 as u32 {
                (*p_p_key2_1).err_code =
                    unsafe { sqlite3_corrupt_error(4941) } as u8;
                return 0;
            }
        }

        /// No memory allocation is ever used on mem1.  Prove this using
        ///* the following assert().  If the assert() fails, it indicates a
        ///* memory leak and a need to call sqlite3VdbeMemRelease(&mem1).
        { let _ = 0; };

        /// rc==0 here means that one or both of the keys ran out of fields and
        ///* all the fields up to that point were equal. Return the default_rc
        ///* value.
        { let _ = 0; };
        (*p_p_key2_1).eq_seen = 1 as u8;
        return (*p_p_key2_1).default_rc as i32;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_record_compare(n_key1_1: i32,
    p_key1_1: *const (), p_p_key2_1: *mut UnpackedRecord) -> i32 {
    return sqlite3_vdbe_record_compare_with_skip(n_key1_1, p_key1_1,
            unsafe { &mut *p_p_key2_1 }, 0);
}

///* Allocate sufficient space for an UnpackedRecord structure large enough
///* to hold a decoded index record for pKeyInfo.
///*
///* The space is allocated using sqlite3DbMallocRaw().  If an OOM error
///* occurs, NULL is returned.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_alloc_unpacked_record(p_key_info_1:
        *mut KeyInfo) -> *mut UnpackedRecord {
    unsafe {
        let mut p: *mut UnpackedRecord = core::ptr::null_mut();
        /// Unpacked record to return
        let mut n_byte: u64 = 0 as u64;

        /// Number of bytes required for *p
        { let _ = 0; };
        n_byte =
            (core::mem::size_of::<UnpackedRecord>() as u64 +
                    core::mem::size_of::<Mem>() as u64 *
                        (unsafe { (*p_key_info_1).n_key_field } as i32 + 1) as u64)
                as u64;
        p =
            unsafe {
                    sqlite3_db_malloc_raw(unsafe { (*p_key_info_1).db }, n_byte)
                } as *mut UnpackedRecord;
        if (p).is_null() as i32 != 0 { return core::ptr::null_mut(); }
        unsafe {
            (*p).a_mem =
                unsafe {
                        &raw mut *(p as
                                        *mut i8).add(core::mem::size_of::<UnpackedRecord>() as
                                        usize)
                    } as *mut Mem
        };
        unsafe { (*p).p_key_info = p_key_info_1 };
        unsafe {
            (*p).n_field =
                (unsafe { (*p_key_info_1).n_key_field } as i32 + 1) as u16
        };
        return p;
    }
}

///* This function is an optimized version of sqlite3VdbeRecordCompare()
///* that (a) the first field of pPKey2 is an integer, and (b) the
///* size-of-header varint at the start of (pKey1/nKey1) fits in a single
///* byte (i.e. is less than 128).
///*
///* To avoid concerns about buffer overreads, this routine is only used
///* on schemas where the maximum valid header size is 63 bytes or less.
#[allow(unused_doc_comments)]
extern "C" fn vdbe_record_compare_int(n_key1_1: i32, p_key1_1: *const (),
    p_p_key2_1: *mut UnpackedRecord) -> i32 {
    unsafe {
        let a_key: *const u8 =
            unsafe {
                &*(p_key1_1 as
                                *const u8).offset((unsafe { *(p_key1_1 as *const u8) } as
                                        i32 & 63) as isize)
            };
        let serial_type: i32 =
            unsafe { *(p_key1_1 as *const u8).offset(1 as isize) } as i32;
        let mut res: i32 = 0;
        let mut y: u32 = 0 as u32;
        let mut x: u64 = 0 as u64;
        let mut v: i64 = 0 as i64;
        let mut lhs: i64 = 0 as i64;
        { let _ = 0; };
        '__s35:
            {
            match serial_type {
                1 => {
                    {

                        /// 1-byte signed integer
                        (lhs = unsafe { *a_key.offset(0 as isize) } as i8 as i64);
                        break '__s35;
                    }
                    {

                        /// 2-byte signed integer
                        (lhs =
                            (256 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                    unsafe { *a_key.offset(1 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 3-byte signed integer
                        (lhs =
                            (65536 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                        (unsafe { *a_key.offset(1 as isize) } as i32) << 8 |
                                    unsafe { *a_key.offset(2 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 4-byte signed integer
                        (y =
                            (unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                    |
                                    ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                | unsafe { *a_key.offset(3 as isize) } as u32);
                        lhs = unsafe { *(&raw mut y as *mut i32) } as i64;
                        break '__s35;
                    }
                    {

                        /// 6-byte signed integer
                        (lhs =
                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(2 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(2 as isize).offset(3 as isize) }
                                            } as u32) as i64 +
                                ((1 as i64) << 32) *
                                    (256 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                            unsafe { *a_key.offset(1 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 8-byte signed integer
                        (x =
                            ((unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                            ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                        |
                                        ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                    | unsafe { *a_key.offset(3 as isize) } as u32) as u64);
                        x =
                            x << 32 |
                                ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(4 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(4 as isize).offset(3 as isize) }
                                            } as u32) as u64;
                        lhs = unsafe { *(&raw mut x as *mut i64) };
                        break '__s35;
                    }
                    lhs = 0 as i64;
                }
                2 => {
                    {

                        /// 2-byte signed integer
                        (lhs =
                            (256 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                    unsafe { *a_key.offset(1 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 3-byte signed integer
                        (lhs =
                            (65536 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                        (unsafe { *a_key.offset(1 as isize) } as i32) << 8 |
                                    unsafe { *a_key.offset(2 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 4-byte signed integer
                        (y =
                            (unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                    |
                                    ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                | unsafe { *a_key.offset(3 as isize) } as u32);
                        lhs = unsafe { *(&raw mut y as *mut i32) } as i64;
                        break '__s35;
                    }
                    {

                        /// 6-byte signed integer
                        (lhs =
                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(2 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(2 as isize).offset(3 as isize) }
                                            } as u32) as i64 +
                                ((1 as i64) << 32) *
                                    (256 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                            unsafe { *a_key.offset(1 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 8-byte signed integer
                        (x =
                            ((unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                            ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                        |
                                        ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                    | unsafe { *a_key.offset(3 as isize) } as u32) as u64);
                        x =
                            x << 32 |
                                ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(4 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(4 as isize).offset(3 as isize) }
                                            } as u32) as u64;
                        lhs = unsafe { *(&raw mut x as *mut i64) };
                        break '__s35;
                    }
                    lhs = 0 as i64;
                }
                3 => {
                    {

                        /// 3-byte signed integer
                        (lhs =
                            (65536 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                        (unsafe { *a_key.offset(1 as isize) } as i32) << 8 |
                                    unsafe { *a_key.offset(2 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 4-byte signed integer
                        (y =
                            (unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                    |
                                    ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                | unsafe { *a_key.offset(3 as isize) } as u32);
                        lhs = unsafe { *(&raw mut y as *mut i32) } as i64;
                        break '__s35;
                    }
                    {

                        /// 6-byte signed integer
                        (lhs =
                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(2 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(2 as isize).offset(3 as isize) }
                                            } as u32) as i64 +
                                ((1 as i64) << 32) *
                                    (256 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                            unsafe { *a_key.offset(1 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 8-byte signed integer
                        (x =
                            ((unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                            ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                        |
                                        ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                    | unsafe { *a_key.offset(3 as isize) } as u32) as u64);
                        x =
                            x << 32 |
                                ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(4 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(4 as isize).offset(3 as isize) }
                                            } as u32) as u64;
                        lhs = unsafe { *(&raw mut x as *mut i64) };
                        break '__s35;
                    }
                    lhs = 0 as i64;
                }
                4 => {
                    {

                        /// 4-byte signed integer
                        (y =
                            (unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                        ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                    |
                                    ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                | unsafe { *a_key.offset(3 as isize) } as u32);
                        lhs = unsafe { *(&raw mut y as *mut i32) } as i64;
                        break '__s35;
                    }
                    {

                        /// 6-byte signed integer
                        (lhs =
                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(2 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(2 as isize).offset(3 as isize) }
                                            } as u32) as i64 +
                                ((1 as i64) << 32) *
                                    (256 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                            unsafe { *a_key.offset(1 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 8-byte signed integer
                        (x =
                            ((unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                            ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                        |
                                        ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                    | unsafe { *a_key.offset(3 as isize) } as u32) as u64);
                        x =
                            x << 32 |
                                ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(4 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(4 as isize).offset(3 as isize) }
                                            } as u32) as u64;
                        lhs = unsafe { *(&raw mut x as *mut i64) };
                        break '__s35;
                    }
                    lhs = 0 as i64;
                }
                5 => {
                    {

                        /// 6-byte signed integer
                        (lhs =
                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(2 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(2 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(2 as isize).offset(3 as isize) }
                                            } as u32) as i64 +
                                ((1 as i64) << 32) *
                                    (256 * unsafe { *a_key.offset(0 as isize) } as i8 as i32 |
                                            unsafe { *a_key.offset(1 as isize) } as i32) as i64);
                        break '__s35;
                    }
                    {

                        /// 8-byte signed integer
                        (x =
                            ((unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                            ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                        |
                                        ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                    | unsafe { *a_key.offset(3 as isize) } as u32) as u64);
                        x =
                            x << 32 |
                                ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(4 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(4 as isize).offset(3 as isize) }
                                            } as u32) as u64;
                        lhs = unsafe { *(&raw mut x as *mut i64) };
                        break '__s35;
                    }
                    lhs = 0 as i64;
                }
                6 => {
                    {

                        /// 8-byte signed integer
                        (x =
                            ((unsafe { *a_key.offset(0 as isize) } as u32) << 24 |
                                            ((unsafe { *a_key.offset(1 as isize) } as i32) << 16) as u32
                                        |
                                        ((unsafe { *a_key.offset(2 as isize) } as i32) << 8) as u32
                                    | unsafe { *a_key.offset(3 as isize) } as u32) as u64);
                        x =
                            x << 32 |
                                ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(0 as isize) }
                                                            } as u32) << 24 |
                                                ((unsafe {
                                                                    *unsafe { a_key.offset(4 as isize).offset(1 as isize) }
                                                                } as i32) << 16) as u32 |
                                            ((unsafe {
                                                                *unsafe { a_key.offset(4 as isize).offset(2 as isize) }
                                                            } as i32) << 8) as u32 |
                                        unsafe {
                                                *unsafe { a_key.offset(4 as isize).offset(3 as isize) }
                                            } as u32) as u64;
                        lhs = unsafe { *(&raw mut x as *mut i64) };
                        break '__s35;
                    }
                    lhs = 0 as i64;
                }
                8 => { lhs = 0 as i64; }
                9 => { lhs = 1 as i64; }
                0 => {
                    return sqlite3_vdbe_record_compare(n_key1_1, p_key1_1,
                            p_p_key2_1);
                    return sqlite3_vdbe_record_compare(n_key1_1, p_key1_1,
                            p_p_key2_1);
                }
                7 => {
                    return sqlite3_vdbe_record_compare(n_key1_1, p_key1_1,
                            p_p_key2_1);
                    return sqlite3_vdbe_record_compare(n_key1_1, p_key1_1,
                            p_p_key2_1);
                }
                _ => {
                    return sqlite3_vdbe_record_compare(n_key1_1, p_key1_1,
                            p_p_key2_1);
                }
            }
        }
        { let _ = 0; };
        v = unsafe { (*p_p_key2_1).u.i };
        if v > lhs {
            res = unsafe { (*p_p_key2_1).r1 } as i32;
        } else if v < lhs {
            res = unsafe { (*p_p_key2_1).r2 } as i32;
        } else if unsafe { (*p_p_key2_1).n_field } as i32 > 1 {

            /// The first fields of the two keys are equal. Compare the trailing
            ///* fields.
            (res =
                sqlite3_vdbe_record_compare_with_skip(n_key1_1, p_key1_1,
                    unsafe { &mut *p_p_key2_1 }, 1));
        } else {

            /// The first fields of the two keys are equal and there are no trailing
            ///* fields. Return pPKey2->default_rc in this case.
            (res = unsafe { (*p_p_key2_1).default_rc } as i32);
            unsafe { (*p_p_key2_1).eq_seen = 1 as u8 };
        }
        { let _ = 0; };
        return res;
    }
}

///* This function is an optimized version of sqlite3VdbeRecordCompare()
///* that (a) the first field of pPKey2 is a string, that (b) the first field
///* uses the collation sequence BINARY and (c) that the size-of-header varint
///* at the start of (pKey1/nKey1) fits in a single byte.
#[allow(unused_doc_comments)]
extern "C" fn vdbe_record_compare_string(n_key1_1: i32, p_key1_1: *const (),
    p_p_key2_1: *mut UnpackedRecord) -> i32 {
    unsafe {
        let a_key1: *const u8 = p_key1_1 as *const u8;
        let mut serial_type: i32 = 0;
        let mut res: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        serial_type = unsafe { *a_key1.offset(1 as isize) } as i8 as i32;
        loop {
            if serial_type < 12 {
                if serial_type < 0 {
                    unsafe {
                        sqlite3_get_varint32(unsafe { &*a_key1.offset(1 as isize) },
                            &raw mut serial_type as *mut u32)
                    };
                    if serial_type >= 12 { continue; }
                    { let _ = 0; };
                }
                res = unsafe { (*p_p_key2_1).r1 } as i32;
            } else if (serial_type & 1 == 0) as i32 != 0 {
                res = unsafe { (*p_p_key2_1).r2 } as i32;
            } else {
                let mut n_cmp: i32 = 0;
                let mut n_str: i32 = 0;
                let sz_hdr: i32 =
                    unsafe { *a_key1.offset(0 as isize) } as i32;
                n_str = (serial_type - 12) / 2;
                if sz_hdr + n_str > n_key1_1 {
                    unsafe {
                        (*p_p_key2_1).err_code =
                            unsafe { sqlite3_corrupt_error(5104) } as u8
                    };
                    return 0;
                }
                n_cmp =
                    if unsafe { (*p_p_key2_1).n } < n_str {
                        unsafe { (*p_p_key2_1).n }
                    } else { n_str };
                res =
                    unsafe {
                        memcmp(unsafe { &raw const *a_key1.offset(sz_hdr as isize) }
                                as *const (), unsafe { (*p_p_key2_1).u.z } as *const (),
                            n_cmp as u64)
                    };
                if res > 0 {
                    res = unsafe { (*p_p_key2_1).r2 } as i32;
                } else if res < 0 {
                    res = unsafe { (*p_p_key2_1).r1 } as i32;
                } else {
                    res = n_str - unsafe { (*p_p_key2_1).n };
                    if res == 0 {
                        if unsafe { (*p_p_key2_1).n_field } as i32 > 1 {
                            res =
                                sqlite3_vdbe_record_compare_with_skip(n_key1_1, p_key1_1,
                                    unsafe { &mut *p_p_key2_1 }, 1);
                        } else {
                            res = unsafe { (*p_p_key2_1).default_rc } as i32;
                            unsafe { (*p_p_key2_1).eq_seen = 1 as u8 };
                        }
                    } else if res > 0 {
                        res = unsafe { (*p_p_key2_1).r2 } as i32;
                    } else { res = unsafe { (*p_p_key2_1).r1 } as i32; }
                }
            }
            break;
        }

        /// (pKey1/nKey1) is a number or a null
        /// (pKey1/nKey1) is a blob
        /// Corruption
        { let _ = 0; };
        return res;
    }
}

///* Return a pointer to an sqlite3VdbeRecordCompare() compatible function
///* suitable for comparing serialized records to the unpacked record passed
///* as the only argument.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_find_compare(p: &mut UnpackedRecord)
    -> unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord) -> i32 {
    unsafe {

        /// varintRecordCompareInt() and varintRecordCompareString() both assume
        ///* that the size-of-header varint that occurs at the start of each record
        ///* fits in a single byte (i.e. is 127 or less). varintRecordCompareInt()
        ///* also assumes that it is safe to overread a buffer by at least the
        ///* maximum possible legal header size plus 8 bytes. Because there is
        ///* guaranteed to be at least 74 (but not 136) bytes of padding following each
        ///* buffer passed to varintRecordCompareInt() this makes it convenient to
        ///* limit the size of the header to 64 bytes in cases where the first field
        ///* is an integer.
        ///*
        ///* The easiest way to enforce this limit is to consider only records with
        ///* 13 fields or less. If the first field is an integer, the maximum legal
        ///* header size is (12*5 + 1 + 1) bytes.
        { let _ = 0; };
        if unsafe { (*(*p).p_key_info).n_all_field } as i32 <= 13 {
            let flags: i32 =
                unsafe { (*(*p).a_mem.offset(0 as isize)).flags } as i32;
            if unsafe {
                        *unsafe {
                                (*(*p).p_key_info).a_sort_flags.offset(0 as isize)
                            }
                    } != 0 {
                if unsafe {
                                    *unsafe {
                                            (*(*p).p_key_info).a_sort_flags.offset(0 as isize)
                                        }
                                } as i32 & 2 != 0 {
                    return sqlite3_vdbe_record_compare;
                }
                (*p).r1 = 1 as i8;
                (*p).r2 = -1 as i8;
            } else { (*p).r1 = -1 as i8; (*p).r2 = 1 as i8; }
            if flags & 4 != 0 {
                (*p).u.i = unsafe { (*(*p).a_mem.offset(0 as isize)).u.i };
                return vdbe_record_compare_int;
            }
            if flags & (8 | 32 | 1 | 16) == 0 &&
                    unsafe {
                            *(unsafe { (*(*p).p_key_info).a_coll.as_ptr() } as
                                        *mut *mut CollSeq).offset(0 as isize)
                        } == core::ptr::null_mut() {
                { let _ = 0; };
                (*p).u.z = unsafe { (*(*p).a_mem.offset(0 as isize)).z };
                (*p).n = unsafe { (*(*p).a_mem.offset(0 as isize)).n };
                return vdbe_record_compare_string;
            }
        }
        return sqlite3_vdbe_record_compare;
    }
}

///* Link the SubProgram object passed as the second argument into the linked
///* list at Vdbe.pSubProgram. This list is used to delete all sub-program
///* objects when the VM is no longer required.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_link_sub_program(p_vdbe_1: &mut Vdbe,
    p: *mut SubProgram) -> () {
    unsafe {
        unsafe { (*p).p_next = (*p_vdbe_1).p_program };
        (*p_vdbe_1).p_program = p;
    }
}

///* Return true if the given Vdbe has any SubPrograms.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_has_sub_program(p_vdbe_1: &Vdbe) -> i32 {
    unsafe { return ((*p_vdbe_1).p_program != core::ptr::null_mut()) as i32; }
}

///* Cause a function to throw an error if it was call from OP_PureFunc
///* rather than OP_Function.
///*
///* OP_PureFunc means that the function must be deterministic, and should
///* throw an error if it is given inputs that would make it non-deterministic.
///* This routine is invoked by date/time functions that use non-deterministic
///* features such as 'now'.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_not_pure_func(p_ctx_1: *mut Sqlite3Context) -> i32 {
    unsafe {
        let mut p_op: *const VdbeOp = core::ptr::null();
        p_op =
            unsafe {
                    unsafe {
                        (*unsafe {
                                            (*p_ctx_1).p_vdbe
                                        }).a_op.offset(unsafe { (*p_ctx_1).i_op } as isize)
                    }
                } as *const VdbeOp;
        if unsafe { (*p_op).opcode } as i32 == 67 {
            let mut z_context: *const i8 = core::ptr::null();
            let mut z_msg: *mut i8 = core::ptr::null_mut();
            if unsafe { (*p_op).p5 } as i32 & 4 != 0 {
                z_context =
                    c"a CHECK constraint".as_ptr() as *mut i8 as *const i8;
            } else if unsafe { (*p_op).p5 } as i32 & 8 != 0 {
                z_context =
                    c"a generated column".as_ptr() as *mut i8 as *const i8;
            } else {
                z_context = c"an index".as_ptr() as *mut i8 as *const i8;
            }
            z_msg =
                unsafe {
                    sqlite3_mprintf(c"non-deterministic use of %s() in %s".as_ptr()
                                as *mut i8 as *const i8,
                        unsafe { (*unsafe { (*p_ctx_1).p_func }).z_name },
                        z_context)
                };
            unsafe { sqlite3_result_error(p_ctx_1, z_msg as *const i8, -1) };
            unsafe { sqlite3_free(z_msg as *mut ()) };
            return 0;
        }
        return 1;
    }
}

///* Mark every prepared statement associated with a database connection
///* as expired.
///*
///* An expired statement means that recompilation of the statement is
///* recommend.  Statements expire when things happen that make their
///* programs obsolete.  Removing user-defined functions or collating
///* sequences, or changing an authorization function are the types of
///* things that make prepared statements obsolete.
///*
///* If iCode is 1, then expiration is advisory.  The statement should
///* be reprepared before being restarted, but if it is already running
///* it is allowed to run to completion.
///*
///* Internally, this function just sets the Vdbe.expired flag on all
///* prepared statements.  The flag is set to 1 for an immediate expiration
///* and set to 2 for an advisory expiration.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expire_prepared_statements(db: &Sqlite3,
    i_code_1: i32) -> () {
    let mut p: *mut Vdbe = core::ptr::null_mut();
    {
        p = (*db).p_vdbe;
        '__b37: loop {
            if !(!(p).is_null()) { break '__b37; }
            '__c37: loop {
                unsafe { (*p).set_expired((i_code_1 + 1) as Bft as u32) };
                break '__c37;
            }
            p = unsafe { (*p).p_v_next };
        }
    }
}

///* Transfer error message text from an sqlite3_vtab.zErrMsg (text stored
///* in memory obtained from sqlite3_malloc) into a Vdbe.zErrMsg (text stored
///* in memory obtained from sqlite3DbMalloc).
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vtab_import_errmsg(p: &mut Vdbe,
    p_vtab_1: &mut Sqlite3Vtab) -> () {
    if !((*p_vtab_1).z_err_msg).is_null() {
        let db: *mut Sqlite3 = (*p).db;
        unsafe { sqlite3_db_free(db, (*p).z_err_msg as *mut ()) };
        (*p).z_err_msg =
            unsafe {
                sqlite3_db_str_dup(db, (*p_vtab_1).z_err_msg as *const i8)
            };
        unsafe { sqlite3_free((*p_vtab_1).z_err_msg as *mut ()) };
        (*p_vtab_1).z_err_msg = core::ptr::null_mut();
    }
}

///* Close a VDBE cursor and release all the resources that cursor
///* happens to hold.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_free_cursor(p: *mut Vdbe,
    p_cx_1: *mut VdbeCursor) -> () {
    if !(p_cx_1).is_null() {
        unsafe { sqlite3_vdbe_free_cursor_nn(p, p_cx_1) };
    }
}

///* Something has moved cursor "p" out of place.  Maybe the row it was
///* pointed to was deleted out from under it.  Or maybe the btree was
///* rebalanced.  Whatever the cause, try to restore "p" to the place it
///* is supposed to be pointing.  If the row was deleted out from under the
///* cursor, set the cursor to point to a NULL row.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_handle_moved_cursor(p: &mut VdbeCursor)
    -> i32 {
    unsafe {
        let mut is_different_row: i32 = 0;
        let mut rc: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        rc =
            unsafe {
                sqlite3_btree_cursor_restore((*p).uc.p_cursor,
                    &mut is_different_row)
            };
        (*p).cache_status = 0 as u32;
        if is_different_row != 0 { (*p).null_row = 1 as u8; }
        return rc;
    }
}

///* The cursor "p" has a pending seek operation that has not yet been
///* carried out.  Seek the cursor now.  If an error occurs, return
///* the appropriate error code.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_finish_moveto(p: &mut VdbeCursor) -> i32 {
    unsafe {
        let mut res: i32 = 0;
        let mut rc: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        rc =
            unsafe {
                sqlite3_btree_table_moveto((*p).uc.p_cursor,
                    (*p).moveto_target, 0, &mut res)
            };
        if rc != 0 { return rc; }
        if res != 0 { return unsafe { sqlite3_corrupt_error(3818) }; }
        (*p).deferred_moveto = 0 as u8;
        (*p).cache_status = 0 as u32;
        return 0;
    }
}

///* Check to ensure that the cursor is valid.  Restore the cursor
///* if need be.  Return any I/O error from the restore operation.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_cursor_restore(p: *mut VdbeCursor) -> i32 {
    unsafe {
        { let _ = 0; };
        if unsafe {
                    sqlite3_btree_cursor_has_moved(unsafe { (*p).uc.p_cursor })
                } != 0 {
            return sqlite3_vdbe_handle_moved_cursor(unsafe { &mut *p });
        }
        return 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_one_byte_serial_type_len(serial_type: u8)
    -> u8 {
    { let _ = 0; };
    return sqlite3_small_type_sizes[serial_type as usize] as u8;
}

///* Compare the key of the index entry that cursor pC is pointing to against
///* the key string in pUnpacked.  Write into *pRes a number
///* that is negative, zero, or positive if pC is less than, equal to,
///* or greater than pUnpacked.  Return SQLITE_OK on success.
///*
///* pUnpacked is either created without a rowid or is truncated so that it
///* omits the rowid at the end.  The rowid at the end of the index entry
///* is ignored as well.  Hence, this routine only compares the prefixes
///* of the keys prior to the final rowid, not the entire key.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_idx_key_compare(db: *mut Sqlite3,
    p_c_1: &VdbeCursor, p_unpacked_1: *mut UnpackedRecord, res: &mut i32)
    -> i32 {
    unsafe {
        let mut n_cell_key: i64 = 0 as i64;
        let mut rc: i32 = 0;
        let mut p_cur: *mut BtCursor = core::ptr::null_mut();
        let mut m: Mem = unsafe { core::mem::zeroed() };
        { let _ = 0; };
        p_cur = (*p_c_1).uc.p_cursor;
        { let _ = 0; };
        n_cell_key = unsafe { sqlite3_btree_payload_size(p_cur) } as i64;
        if n_cell_key <= 0 as i64 || n_cell_key > 2147483647 as i64 {
            *res = 0;
            return unsafe { sqlite3_corrupt_error(5296) };
        }
        unsafe { sqlite3_vdbe_mem_init(&mut m, db, 0 as u16) };
        rc =
            unsafe {
                sqlite3_vdbe_mem_from_btree_zero_offset(p_cur,
                    n_cell_key as u32, &mut m)
            };
        if rc != 0 { return rc; }
        *res =
            sqlite3_vdbe_record_compare_with_skip(m.n, m.z as *const (),
                unsafe { &mut *p_unpacked_1 }, 0);
        unsafe { sqlite3_vdbe_mem_release_malloc(&mut m) };
        return 0;
    }
}

///* pCur points at an index entry created using the OP_MakeRecord opcode.
///* Read the rowid (the last field in the record) and store it in *rowid.
///* Return SQLITE_OK if everything works, or an error code otherwise.
///*
///* pCur might be pointing to text obtained from a corrupt database file.
///* So the content cannot be trusted.  Do appropriate checks on the content.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_idx_rowid(db: *mut Sqlite3,
    p_cur_1: *mut BtCursor, rowid: &mut i64) -> i32 {
    unsafe {
        /// Size of the header
        /// Serial type of the rowid
        /// Size of the rowid
        let mut m: Mem = unsafe { core::mem::zeroed() };
        let mut v: Mem = unsafe { core::mem::zeroed() };
        '__b38: loop {
            '__c38: loop {
                let mut n_cell_key: i64 = 0 as i64;
                let mut rc: i32 = 0;
                let mut sz_hdr: u32 = 0 as u32;
                /// Size of the header
                let mut type_rowid: u32 = 0 as u32;
                /// Serial type of the rowid
                let mut len_rowid: u32 = 0 as u32;

                /// Size of the rowid
                /// Get the size of the index entry.  Only indices entries of less
                ///* than 2GiB are support - anything large must be database corruption.
                ///* Any corruption is detected in sqlite3BtreeParseCellPtr(), though, so
                ///* this code can safely assume that nCellKey is 32-bits
                { let _ = 0; };
                n_cell_key =
                    unsafe { sqlite3_btree_payload_size(p_cur_1) } as i64;
                { let _ = 0; };

                /// Read in the complete content of the index entry
                unsafe { sqlite3_vdbe_mem_init(&mut m, db, 0 as u16) };
                rc =
                    unsafe {
                        sqlite3_vdbe_mem_from_btree_zero_offset(p_cur_1,
                            n_cell_key as u32, &mut m)
                    };
                if rc != 0 { return rc; }

                /// The index entry must begin with a header size
                (sz_hdr = unsafe { *(m.z as *mut u8) } as u32);
                if sz_hdr >= 128 as u32 {
                    unsafe {
                        sqlite3_get_varint32(m.z as *mut u8 as *const u8,
                            &raw mut sz_hdr as *mut u32)
                    };
                }
                { let _ = 0; };
                if sz_hdr < 3 as u32 || sz_hdr > m.n as u32 { break '__b38; }

                /// The last field of the index should be an integer - the ROWID.
                ///* Verify that the last entry really is an integer.
                (type_rowid =
                    unsafe {
                            *(unsafe { &raw mut *m.z.add((sz_hdr - 1 as u32) as usize) }
                                    as *mut u8)
                        } as u32);
                if type_rowid >= 128 as u32 {
                    unsafe {
                        sqlite3_get_varint32(unsafe {
                                        &raw mut *m.z.add((sz_hdr - 1 as u32) as usize)
                                    } as *mut u8 as *const u8, &raw mut type_rowid as *mut u32)
                    };
                }
                if type_rowid < 1 as u32 || type_rowid > 9 as u32 ||
                        type_rowid == 7 as u32 {
                    break '__b38;
                }
                len_rowid =
                    sqlite3_small_type_sizes[type_rowid as usize] as u32;
                if (m.n as u32) < sz_hdr + len_rowid { break '__b38; }

                /// Fetch the integer off the end of the index record
                sqlite3_vdbe_serial_get(unsafe {
                                &raw mut *m.z.add((m.n as u32 - len_rowid) as usize)
                            } as *mut u8 as *const u8, type_rowid, &mut v);
                *rowid = v.u.i;
                unsafe { sqlite3_vdbe_mem_release_malloc(&mut m) };
                return 0;
                break '__c38;
            }
            if !(false) { break '__b38; }
        }
        unsafe { sqlite3_vdbe_mem_release_malloc(&mut m) };
        return unsafe { sqlite3_corrupt_error(5263) };
    }
}

///* Locate the next opcode to be displayed in EXPLAIN or EXPLAIN
///* QUERY PLAN output.
///*
///* Return SQLITE_ROW on success.  Return SQLITE_DONE if there are no
///* more opcodes to be displayed.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_next_opcode(p: &mut Vdbe, p_sub_1: *mut Mem,
    e_mode_1: i32, pi_pc_1: &mut i32, pi_addr_1: &mut i32,
    pa_op_1: &mut *mut Op) -> i32 {
    unsafe {
        let mut n_row: i32 = 0;
        /// Stop when row count reaches this
        let mut n_sub: i32 = 0;
        /// Number of sub-vdbes seen so far
        let mut ap_sub: *mut *mut SubProgram = core::ptr::null_mut();
        /// Array of sub-vdbes
        let mut i: i32 = 0;
        /// Next instruction address
        let mut rc: i32 = 0;
        /// Result code
        let mut a_op: *mut Op = core::ptr::null_mut();
        /// Opcode array
        let mut i_pc: i32 = 0;

        /// Rowid.  Copy of value in *piPc
        /// When the number of output rows reaches nRow, that means the
        ///* listing has finished and sqlite3_step() should return SQLITE_DONE.
        ///* nRow is the sum of the number of rows in the main program, plus
        ///* the sum of the number of rows in all trigger subprograms encountered
        ///* so far.  The nRow value will increase as new trigger subprograms are
        ///* encountered, but p->pc will eventually catch up to nRow.
        (n_row = (*p).n_op);
        if p_sub_1 != core::ptr::null_mut() {
            if unsafe { (*p_sub_1).flags } as i32 & 16 != 0 {

                /// pSub is initiallly NULL.  It is initialized to a BLOB by
                ///* the P4_SUBPROGRAM processing logic below
                (n_sub =
                    (unsafe { (*p_sub_1).n } as u64 /
                            core::mem::size_of::<*mut Vdbe>() as u64) as i32);
                ap_sub = unsafe { (*p_sub_1).z } as *mut *mut SubProgram;
            }
            {
                i = 0;
                '__b39: loop {
                    if !(i < n_sub) { break '__b39; }
                    '__c39: loop {
                        n_row +=
                            unsafe { (*unsafe { *ap_sub.offset(i as isize) }).n_op };
                        break '__c39;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        i_pc = *pi_pc_1;
        loop {

            /// Loop exits via break
            (i = { let __p = &mut i_pc; let __t = *__p; *__p += 1; __t });
            if i >= n_row { (*p).rc = 0; rc = 101; break; }
            if i < (*p).n_op {

                /// The rowid is small enough that we are still in the
                ///* main program.
                (a_op = (*p).a_op);
            } else {
                /// We are currently listing subprograms.  Figure out which one and
                ///* pick up the appropriate opcode.
                let mut j: i32 = 0;
                i -= (*p).n_op;
                { let _ = 0; };
                { let _ = 0; };
                {
                    j = 0;
                    '__b41: loop {
                        if !(i >=
                                        unsafe { (*unsafe { *ap_sub.offset(j as isize) }).n_op }) {
                            break '__b41;
                        }
                        '__c41: loop {
                            i -=
                                unsafe { (*unsafe { *ap_sub.offset(j as isize) }).n_op };
                            { let _ = 0; };
                            break '__c41;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                a_op =
                    unsafe { (*unsafe { *ap_sub.offset(j as isize) }).a_op } as
                        *mut Op;
            }
            if p_sub_1 != core::ptr::null_mut() &&
                    unsafe { (*a_op.offset(i as isize)).p4type } as i32 == -4 {
                let n_byte: i32 =
                    ((n_sub + 1) as u64 *
                            core::mem::size_of::<*mut SubProgram>() as u64) as i32;
                let mut j: i32 = 0;
                {
                    j = 0;
                    '__b42: loop {
                        if !(j < n_sub) { break '__b42; }
                        '__c42: loop {
                            if unsafe { *ap_sub.offset(j as isize) } ==
                                    unsafe { (*a_op.offset(i as isize)).p4.p_program } {
                                break '__b42;
                            }
                            break '__c42;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                if j == n_sub {
                    (*p).rc =
                        unsafe {
                            sqlite3_vdbe_mem_grow(p_sub_1, n_byte, (n_sub != 0) as i32)
                        };
                    if (*p).rc != 0 { rc = 1; break; }
                    ap_sub = unsafe { (*p_sub_1).z } as *mut *mut SubProgram;
                    unsafe {
                        *ap_sub.offset({
                                            let __p = &mut n_sub;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) =
                            unsafe { (*a_op.offset(i as isize)).p4.p_program }
                    };
                    unsafe {
                        (*p_sub_1).flags =
                            (unsafe { (*p_sub_1).flags } as i32 & !(3519 | 1024) | 16)
                                as u16
                    };
                    unsafe {
                        (*p_sub_1).n =
                            (n_sub as u64 *
                                    core::mem::size_of::<*mut SubProgram>() as u64) as i32
                    };
                    n_row +=
                        unsafe {
                            (*unsafe { (*a_op.offset(i as isize)).p4.p_program }).n_op
                        };
                }
            }
            if e_mode_1 == 0 { break; }
            {
                { let _ = 0; };
                if unsafe { (*a_op.offset(i as isize)).opcode } as i32 == 190
                    {
                    break;
                }
                if unsafe { (*a_op.offset(i as isize)).opcode } as i32 == 8 &&
                        i_pc > 1 {
                    break;
                }
            }
        }
        *pi_pc_1 = i_pc;
        *pi_addr_1 = i;
        *pa_op_1 = a_op;
        return rc;
    }
}

///* Compute a string that describes the P4 parameter for an opcode.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_display_p4(db: *mut Sqlite3, p_op_1: &Op)
    -> *mut i8 {
    unsafe {
        unsafe {
            let mut z_p4: *const i8 = core::ptr::null();
            let mut x: StrAccum = unsafe { core::mem::zeroed() };
            unsafe {
                sqlite3_str_accum_init(&mut x, core::ptr::null_mut(),
                    core::ptr::null_mut(), 0, 1000000000)
            };
            '__s43:
                {
                match (*p_op_1).p4type {
                    -9 => {
                        {
                            let mut j: i32 = 0;
                            let p_key_info: *const KeyInfo =
                                (*p_op_1).p4.p_key_info as *const KeyInfo;
                            { let _ = 0; };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"k(%d".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_key_info).n_key_field } as i32)
                            };
                            {
                                j = 0;
                                '__b44: loop {
                                    if !(j < unsafe { (*p_key_info).n_key_field } as i32) {
                                        break '__b44;
                                    }
                                    '__c44: loop {
                                        let p_coll: *const CollSeq =
                                            unsafe {
                                                    *(unsafe { (*p_key_info).a_coll.as_ptr() } as
                                                                *mut *mut CollSeq).offset(j as isize)
                                                } as *const CollSeq;
                                        let mut z_coll: *const i8 =
                                            if !(p_coll).is_null() {
                                                    unsafe { (*p_coll).z_name }
                                                } else { c"".as_ptr() as *mut i8 } as *const i8;
                                        if unsafe {
                                                    strcmp(z_coll, c"BINARY".as_ptr() as *mut i8 as *const i8)
                                                } == 0 {
                                            z_coll = c"B".as_ptr() as *mut i8 as *const i8;
                                        }
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c",%s%s%s".as_ptr() as *mut i8 as *const i8,
                                                if unsafe {
                                                                    *unsafe { (*p_key_info).a_sort_flags.offset(j as isize) }
                                                                } as i32 & 1 != 0 {
                                                    c"-".as_ptr() as *mut i8
                                                } else { c"".as_ptr() as *mut i8 },
                                                if unsafe {
                                                                    *unsafe { (*p_key_info).a_sort_flags.offset(j as isize) }
                                                                } as i32 & 2 != 0 {
                                                    c"N.".as_ptr() as *mut i8
                                                } else { c"".as_ptr() as *mut i8 }, z_coll)
                                        };
                                        break '__c44;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c")".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        {
                            let p_coll_1: *const CollSeq =
                                (*p_op_1).p4.p_coll as *const CollSeq;
                            { let _ = 0; };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.18s-%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_coll_1).z_name },
                                    encnames[unsafe { (*p_coll_1).enc } as usize])
                            };
                            break '__s43;
                        }
                        {
                            let p_def: *const FuncDef =
                                (*p_op_1).p4.p_func as *const FuncDef;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%s(%d)".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_def).z_name },
                                    unsafe { (*p_def).n_arg } as i32)
                            };
                            break '__s43;
                        }
                        {
                            let p_def_1: *const FuncDef =
                                unsafe { (*(*p_op_1).p4.p_ctx).p_func } as *const FuncDef;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%s(%d)".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_def_1).z_name },
                                    unsafe { (*p_def_1).n_arg } as i32)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%lld".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_i64 })
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%d".as_ptr() as *mut i8 as *const i8, (*p_op_1).p4.i)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.16g".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_real })
                            };
                            break '__s43;
                        }
                        {
                            let p_mem: *const Mem = (*p_op_1).p4.p_mem as *const Mem;
                            if unsafe { (*p_mem).flags } as i32 & 2 != 0 {
                                z_p4 = unsafe { (*p_mem).z };
                            } else if unsafe { (*p_mem).flags } as i32 & (4 | 32) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%lld".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.i })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 8 != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%.16g".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.r })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 1 != 0 {
                                z_p4 = c"NULL".as_ptr() as *mut i8;
                            } else {
                                { let _ = 0; };
                                z_p4 = c"(blob)".as_ptr() as *mut i8;
                            }
                            break '__s43;
                        }
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -2 => {
                        {
                            let p_coll_1: *const CollSeq =
                                (*p_op_1).p4.p_coll as *const CollSeq;
                            { let _ = 0; };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.18s-%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_coll_1).z_name },
                                    encnames[unsafe { (*p_coll_1).enc } as usize])
                            };
                            break '__s43;
                        }
                        {
                            let p_def: *const FuncDef =
                                (*p_op_1).p4.p_func as *const FuncDef;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%s(%d)".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_def).z_name },
                                    unsafe { (*p_def).n_arg } as i32)
                            };
                            break '__s43;
                        }
                        {
                            let p_def_1: *const FuncDef =
                                unsafe { (*(*p_op_1).p4.p_ctx).p_func } as *const FuncDef;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%s(%d)".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_def_1).z_name },
                                    unsafe { (*p_def_1).n_arg } as i32)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%lld".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_i64 })
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%d".as_ptr() as *mut i8 as *const i8, (*p_op_1).p4.i)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.16g".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_real })
                            };
                            break '__s43;
                        }
                        {
                            let p_mem: *const Mem = (*p_op_1).p4.p_mem as *const Mem;
                            if unsafe { (*p_mem).flags } as i32 & 2 != 0 {
                                z_p4 = unsafe { (*p_mem).z };
                            } else if unsafe { (*p_mem).flags } as i32 & (4 | 32) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%lld".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.i })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 8 != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%.16g".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.r })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 1 != 0 {
                                z_p4 = c"NULL".as_ptr() as *mut i8;
                            } else {
                                { let _ = 0; };
                                z_p4 = c"(blob)".as_ptr() as *mut i8;
                            }
                            break '__s43;
                        }
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -8 => {
                        {
                            let p_def: *const FuncDef =
                                (*p_op_1).p4.p_func as *const FuncDef;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%s(%d)".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_def).z_name },
                                    unsafe { (*p_def).n_arg } as i32)
                            };
                            break '__s43;
                        }
                        {
                            let p_def_1: *const FuncDef =
                                unsafe { (*(*p_op_1).p4.p_ctx).p_func } as *const FuncDef;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%s(%d)".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_def_1).z_name },
                                    unsafe { (*p_def_1).n_arg } as i32)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%lld".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_i64 })
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%d".as_ptr() as *mut i8 as *const i8, (*p_op_1).p4.i)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.16g".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_real })
                            };
                            break '__s43;
                        }
                        {
                            let p_mem: *const Mem = (*p_op_1).p4.p_mem as *const Mem;
                            if unsafe { (*p_mem).flags } as i32 & 2 != 0 {
                                z_p4 = unsafe { (*p_mem).z };
                            } else if unsafe { (*p_mem).flags } as i32 & (4 | 32) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%lld".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.i })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 8 != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%.16g".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.r })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 1 != 0 {
                                z_p4 = c"NULL".as_ptr() as *mut i8;
                            } else {
                                { let _ = 0; };
                                z_p4 = c"(blob)".as_ptr() as *mut i8;
                            }
                            break '__s43;
                        }
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -16 => {
                        {
                            let p_def_1: *const FuncDef =
                                unsafe { (*(*p_op_1).p4.p_ctx).p_func } as *const FuncDef;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%s(%d)".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_def_1).z_name },
                                    unsafe { (*p_def_1).n_arg } as i32)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%lld".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_i64 })
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%d".as_ptr() as *mut i8 as *const i8, (*p_op_1).p4.i)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.16g".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_real })
                            };
                            break '__s43;
                        }
                        {
                            let p_mem: *const Mem = (*p_op_1).p4.p_mem as *const Mem;
                            if unsafe { (*p_mem).flags } as i32 & 2 != 0 {
                                z_p4 = unsafe { (*p_mem).z };
                            } else if unsafe { (*p_mem).flags } as i32 & (4 | 32) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%lld".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.i })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 8 != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%.16g".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.r })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 1 != 0 {
                                z_p4 = c"NULL".as_ptr() as *mut i8;
                            } else {
                                { let _ = 0; };
                                z_p4 = c"(blob)".as_ptr() as *mut i8;
                            }
                            break '__s43;
                        }
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -14 => {
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%lld".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_i64 })
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%d".as_ptr() as *mut i8 as *const i8, (*p_op_1).p4.i)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.16g".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_real })
                            };
                            break '__s43;
                        }
                        {
                            let p_mem: *const Mem = (*p_op_1).p4.p_mem as *const Mem;
                            if unsafe { (*p_mem).flags } as i32 & 2 != 0 {
                                z_p4 = unsafe { (*p_mem).z };
                            } else if unsafe { (*p_mem).flags } as i32 & (4 | 32) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%lld".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.i })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 8 != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%.16g".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.r })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 1 != 0 {
                                z_p4 = c"NULL".as_ptr() as *mut i8;
                            } else {
                                { let _ = 0; };
                                z_p4 = c"(blob)".as_ptr() as *mut i8;
                            }
                            break '__s43;
                        }
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -3 => {
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%d".as_ptr() as *mut i8 as *const i8, (*p_op_1).p4.i)
                            };
                            break '__s43;
                        }
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.16g".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_real })
                            };
                            break '__s43;
                        }
                        {
                            let p_mem: *const Mem = (*p_op_1).p4.p_mem as *const Mem;
                            if unsafe { (*p_mem).flags } as i32 & 2 != 0 {
                                z_p4 = unsafe { (*p_mem).z };
                            } else if unsafe { (*p_mem).flags } as i32 & (4 | 32) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%lld".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.i })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 8 != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%.16g".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.r })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 1 != 0 {
                                z_p4 = c"NULL".as_ptr() as *mut i8;
                            } else {
                                { let _ = 0; };
                                z_p4 = c"(blob)".as_ptr() as *mut i8;
                            }
                            break '__s43;
                        }
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -13 => {
                        {
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"%.16g".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *(*p_op_1).p4.p_real })
                            };
                            break '__s43;
                        }
                        {
                            let p_mem: *const Mem = (*p_op_1).p4.p_mem as *const Mem;
                            if unsafe { (*p_mem).flags } as i32 & 2 != 0 {
                                z_p4 = unsafe { (*p_mem).z };
                            } else if unsafe { (*p_mem).flags } as i32 & (4 | 32) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%lld".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.i })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 8 != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%.16g".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.r })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 1 != 0 {
                                z_p4 = c"NULL".as_ptr() as *mut i8;
                            } else {
                                { let _ = 0; };
                                z_p4 = c"(blob)".as_ptr() as *mut i8;
                            }
                            break '__s43;
                        }
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -11 => {
                        {
                            let p_mem: *const Mem = (*p_op_1).p4.p_mem as *const Mem;
                            if unsafe { (*p_mem).flags } as i32 & 2 != 0 {
                                z_p4 = unsafe { (*p_mem).z };
                            } else if unsafe { (*p_mem).flags } as i32 & (4 | 32) != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%lld".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.i })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 8 != 0 {
                                unsafe {
                                    sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                        c"%.16g".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_mem).u.r })
                                };
                            } else if unsafe { (*p_mem).flags } as i32 & 1 != 0 {
                                z_p4 = c"NULL".as_ptr() as *mut i8;
                            } else {
                                { let _ = 0; };
                                z_p4 = c"(blob)".as_ptr() as *mut i8;
                            }
                            break '__s43;
                        }
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -12 => {
                        {
                            let p_vtab: *mut Sqlite3Vtab =
                                unsafe { (*(*p_op_1).p4.p_vtab).p_vtab };
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"vtab:%p".as_ptr() as *mut i8 as *const i8, p_vtab)
                            };
                            break '__s43;
                        }
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -15 => {
                        {
                            let mut i: u32 = 0 as u32;
                            let ai: *const u32 = (*p_op_1).p4.ai as *const u32;
                            let n: u32 = unsafe { *ai.offset(0 as isize) };
                            {
                                i = 1 as u32;
                                '__b45: loop {
                                    if !(i <= n) { break '__b45; }
                                    '__c45: loop {
                                        unsafe {
                                            sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                                c"%c%u".as_ptr() as *mut i8 as *const i8,
                                                if i == 1 as u32 { '[' as i32 } else { ',' as i32 },
                                                unsafe { *ai.add(i as usize) })
                                        };
                                        break '__c45;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            unsafe {
                                sqlite3_str_append(&raw mut x as *mut Sqlite3Str,
                                    c"]".as_ptr() as *mut i8 as *const i8, 1)
                            };
                            break '__s43;
                        }
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -4 => {
                        { z_p4 = c"program".as_ptr() as *mut i8; break '__s43; }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -5 => {
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_tab).z_name };
                            break '__s43;
                        }
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -6 => {
                        {
                            z_p4 = unsafe { (*(*p_op_1).p4.p_idx).z_name };
                            break '__s43;
                        }
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    -18 => {
                        {
                            let p_sig: *const SubrtnSig =
                                (*p_op_1).p4.p_subrtn_sig as *const SubrtnSig;
                            unsafe {
                                sqlite3_str_appendf(&raw mut x as *mut Sqlite3Str,
                                    c"subrtnsig:%d,%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_sig).sel_id }, unsafe { (*p_sig).z_aff })
                            };
                            break '__s43;
                        }
                        { z_p4 = (*p_op_1).p4.z; }
                    }
                    _ => { { z_p4 = (*p_op_1).p4.z; } }
                }
            }
            if !(z_p4).is_null() {
                unsafe {
                    sqlite3_str_appendall(&raw mut x as *mut Sqlite3Str,
                        z_p4 as *const i8)
                };
            }
            if x.acc_error as i32 & 7 != 0 {
                unsafe { sqlite3_oom_fault(db) };
            }
            return unsafe { sqlite3_str_accum_finish(&mut x) };
        }
    }
}

///* Give a listing of the program in the virtual machine.
///*
///* The interface is the same as sqlite3VdbeExec().  But instead of
///* running the code, it invokes the callback once for each instruction.
///* This feature is used to implement "EXPLAIN".
///*
///* When p->explain==1, each instruction is listed.  When
///* p->explain==2, only OP_Explain instructions are listed and these
///* are shown in a different format.  p->explain==2 is used to implement
///* EXPLAIN QUERY PLAN.
///* 2018-04-24:  In p->explain==2 mode, the OP_Init opcodes of triggers
///* are also shown, so that the boundaries between the main program and
///* each trigger are clear.
///*
///* When p->explain==1, first the main program is listed, then each of
///* the trigger subprograms are listed one by one.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_list(p: *mut Vdbe) -> i32 {
    unsafe {
        let mut p_sub: *mut Mem = core::ptr::null_mut();
        /// Memory cell hold array of subprogs
        let db: *mut Sqlite3 = unsafe { (*p).db };
        /// The database connection
        let mut i: i32 = 0;
        /// Loop counter
        let mut rc: i32 = 0;
        /// Return code
        let p_mem: *mut Mem =
            unsafe { &mut *unsafe { (*p).a_mem.offset(1 as isize) } };
        /// First Mem of result set
        let b_list_subprogs: i32 =
            (unsafe { (*p).explain() } as i32 == 1 ||
                    unsafe { (*db).flags } & 16777216 as u64 != 0 as u64) as
                i32;
        let mut a_op: *mut Op = core::ptr::null_mut();
        /// Array of opcodes
        let mut p_op: *mut Op = core::ptr::null_mut();

        /// Current opcode
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };

        /// Even though this opcode does not use dynamic strings for
        ///* the result, result columns may become dynamic if the user calls
        ///* sqlite3_column_text16(), causing a translation to UTF-16 encoding.
        release_mem_array(p_mem, 8);
        if unsafe { (*p).rc } == 7 {

            /// This happens if a malloc() inside a call to sqlite3_column_text() or
            ///* sqlite3_column_text16() failed.
            unsafe { sqlite3_oom_fault(db) };
            return 1;
        }
        if b_list_subprogs != 0 {

            /// The first 8 memory cells are used for the result set.  So we will
            ///* commandeer the 9th cell to use as storage for an array of pointers
            ///* to trigger subprograms.  The VDBE is guaranteed to have at least 9
            ///* cells.
            { let _ = 0; };
            p_sub = unsafe { unsafe { (*p).a_mem.offset(9 as isize) } };
        } else { p_sub = core::ptr::null_mut(); }

        /// Figure out which opcode is next to display
        (rc =
            sqlite3_vdbe_next_opcode(unsafe { &mut *p }, p_sub,
                (unsafe { (*p).explain() } as i32 == 2) as i32,
                unsafe { &mut (*p).pc }, &mut i, &mut a_op));
        if rc == 0 {
            p_op = unsafe { a_op.offset(i as isize) };
            if unsafe {
                        std::sync::atomic::AtomicI32::from_ptr(unsafe {
                                        &raw mut (*db).u1.is_interrupted
                                    } as *mut i32).load(std::sync::atomic::Ordering::Relaxed)
                    } != 0 {
                unsafe { (*p).rc = 9 };
                rc = 1;
                unsafe {
                    sqlite3_vdbe_error(unsafe { &mut *p },
                        unsafe { sqlite3_err_str(unsafe { (*p).rc }) })
                };
            } else {
                let z_p4: *const i8 =
                    sqlite3_vdbe_display_p4(db, unsafe { &*p_op }) as *const i8;
                if unsafe { (*p).explain() } as i32 == 2 {
                    unsafe {
                        sqlite3_vdbe_mem_set_int64(p_mem,
                            unsafe { (*p_op).p1 } as i64)
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_int64(unsafe {
                                p_mem.offset(1 as isize)
                            }, unsafe { (*p_op).p2 } as i64)
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_int64(unsafe {
                                p_mem.offset(2 as isize)
                            }, unsafe { (*p_op).p3 } as i64)
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_str(unsafe {
                                p_mem.offset(3 as isize)
                            }, z_p4 as *const i8, -1 as i64, 1 as u8,
                            Some(sqlite3_free))
                    };
                    { let _ = 0; };
                } else {
                    unsafe {
                        sqlite3_vdbe_mem_set_int64(unsafe {
                                p_mem.offset(0 as isize)
                            }, i as i64)
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_str(unsafe {
                                p_mem.offset(1 as isize)
                            },
                            unsafe {
                                        sqlite3_opcode_name(unsafe { (*p_op).opcode } as i32)
                                    } as *mut i8 as *const i8, -1 as i64, 1 as u8, None)
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_int64(unsafe {
                                p_mem.offset(2 as isize)
                            }, unsafe { (*p_op).p1 } as i64)
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_int64(unsafe {
                                p_mem.offset(3 as isize)
                            }, unsafe { (*p_op).p2 } as i64)
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_int64(unsafe {
                                p_mem.offset(4 as isize)
                            }, unsafe { (*p_op).p3 } as i64)
                    };

                    /// pMem+5 for p4 is done last
                    unsafe {
                        sqlite3_vdbe_mem_set_int64(unsafe {
                                p_mem.offset(6 as isize)
                            }, unsafe { (*p_op).p5 } as i64)
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_null(unsafe {
                                p_mem.offset(7 as isize)
                            })
                    };
                    unsafe {
                        sqlite3_vdbe_mem_set_str(unsafe {
                                p_mem.offset(5 as isize)
                            }, z_p4 as *const i8, -1 as i64, 1 as u8,
                            Some(sqlite3_free))
                    };
                    { let _ = 0; };
                }
                unsafe { (*p).p_result_row = p_mem };
                if unsafe { (*db).malloc_failed } != 0 {
                    unsafe { (*p).rc = 7 };
                    rc = 1;
                } else { unsafe { (*p).rc = 0 }; rc = 100; }
            }
        }
        return rc;
    }
}

///* This is a destructor on a Mem object (which is really an sqlite3_value)
///* that deletes the Frame object that is attached to it as a blob.
///*
///* This routine does not delete the Frame right away.  It merely adds the
///* frame to a list of frames to be deleted when the Vdbe halts.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vdbe_frame_mem_del(p_arg: *mut ()) -> () {
    let p_frame: *mut VdbeFrame = p_arg as *mut VdbeFrame;
    { let _ = 0; };
    unsafe {
        (*p_frame).p_parent =
            unsafe { (*unsafe { (*p_frame).v }).p_del_frame }
    };
    unsafe { (*unsafe { (*p_frame).v }).p_del_frame = p_frame };
}

///* Helper function for vdbeIsMatchingIndexKey(). Return true if column
///* iCol should be ignored when comparing a record with a record from 
///* an index on disk. The field should be ignored if:
///*
///*   * the corresponding bit in mask is set, and
///*   * either:
///*       - bIntegrity is false, or
///*       - the two Mem values are both real values that differ by 
///*         BTREE_ULPDISTORTION or fewer ULPs.
extern "C" fn vdbe_skip_field(mask: Bitmask, i_col_1: i32, p_mem1_1: &mut Mem,
    p_mem2_1: &mut Mem, b_integrity_1: i32) -> i32 {
    unsafe {
        if i_col_1 >=
                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32
                || mask & (1 as Bitmask) << i_col_1 == 0 as u64 {
            return 0;
        }
        if b_integrity_1 == 0 { return 1; }
        if (*p_mem1_1).flags as i32 & 8 != 0 &&
                (*p_mem2_1).flags as i32 & 8 != 0 {
            let mut m1: u64 = 0 as u64;
            let mut m2: u64 = 0 as u64;
            unsafe {
                memcpy(&raw mut m1 as *mut (),
                    &raw mut (*p_mem1_1).u.r as *const (), 8 as u64)
            };
            unsafe {
                memcpy(&raw mut m2 as *mut (),
                    &raw mut (*p_mem2_1).u.r as *const (), 8 as u64)
            };
            if if m1 < m2 { m2 - m1 } else { m1 - m2 } <= 2 as u64 {
                return 1;
            }
        }
        return 0;
    }
}

///* This function compares the unpacked record with the current key that
///* cursor pCur points to. If bInt is false, all fields for which the 
///* corresponding bit in parameter "mask" is set are ignored. Or, if
///* bInt is true, then a difference of BTREE_ULPDISTORTION or fewer ULPs
///* in real values is overlooked for fields with the corresponding bit
///* set in mask.  
///*
///* Return the usual less than zero, zero, or greater than zero if the 
///* remaining fields of the cursor cursor key are less than, equal to or 
///* greater than those in (*p).
#[allow(unused_doc_comments)]
extern "C" fn vdbe_is_matching_index_key(p_cur_1: *mut BtCursor, b_int_1: i32,
    mask: Bitmask, p: &UnpackedRecord, pi_res_1: &mut i32) -> i32 {
    unsafe {
        let mut a_rec: *mut u8 = core::ptr::null_mut();
        let mut n_rec: u32 = 0 as u32;
        let mut m: Mem = unsafe { core::mem::zeroed() };
        let mut rc: i32 = 0;
        unsafe {
            memset(&raw mut m as *mut (), 0,
                core::mem::size_of::<Mem>() as u64)
        };
        m.enc = unsafe { (*(*p).p_key_info).enc };
        m.db = unsafe { (*(*p).p_key_info).db };
        n_rec = unsafe { sqlite3_btree_payload_size(p_cur_1) };
        if n_rec > 2147483647 as u32 {
            return unsafe { sqlite3_corrupt_error(5468) };
        }

        /// Allocate 5 extra bytes at the end of the buffer. This allows the
        ///* getVarint32() call below to read slightly past the end of the buffer 
        ///* if the record is corrupt.
        (a_rec =
            unsafe { sqlite3_malloc_zero((n_rec + 5 as u32) as u64) } as
                *mut u8);
        if a_rec == core::ptr::null_mut() {
            rc = 7;
        } else {
            rc =
                unsafe {
                    sqlite3_btree_payload(p_cur_1, 0 as u32, n_rec,
                        a_rec as *mut ())
                };
        }
        if rc == 0 {
            let mut sz_hdr: u32 = 0 as u32;
            /// Size of record header in bytes
            let mut idx_hdr: u32 = 0 as u32;

            /// Current index in header
            (idx_hdr =
                if (unsafe { *a_rec } as i32) < 128 as u8 as i32 {
                            {
                                ({ sz_hdr = unsafe { *a_rec } as u32; sz_hdr }) as i32;
                                1
                            }
                        } else {
                            (unsafe {
                                    sqlite3_get_varint32(a_rec as *const u8,
                                        &raw mut sz_hdr as *mut u32)
                                }) as i32
                        } as u8 as u32);
            if sz_hdr > 98307 as u32 {
                rc = 11;
            } else {
                let mut res: i32 = 0;
                /// Result of this function call
                let mut idx_rec: u32 = sz_hdr;
                /// Index of next field in record body
                let mut ii: i32 = 0;
                /// Iterator variable
                let n_col: i32 =
                    unsafe { (*(*p).p_key_info).n_all_field } as i32;
                {
                    ii = 0;
                    '__b46: loop {
                        if !(ii < n_col && rc == 0) { break '__b46; }
                        '__c46: loop {
                            let mut i_serial: u32 = 0 as u32;
                            let mut n_serial: i32 = 0;
                            if idx_hdr >= sz_hdr {
                                rc = unsafe { sqlite3_corrupt_error(5499) };
                                break '__b46;
                            }
                            idx_hdr +=
                                if (unsafe { *unsafe { &mut *a_rec.add(idx_hdr as usize) } }
                                                        as i32) < 128 as u8 as i32 {
                                            {
                                                ({
                                                        i_serial =
                                                            unsafe { *unsafe { &mut *a_rec.add(idx_hdr as usize) } } as
                                                                u32;
                                                        i_serial
                                                    }) as i32;
                                                1
                                            }
                                        } else {
                                            (unsafe {
                                                    sqlite3_get_varint32(unsafe {
                                                                &raw mut *a_rec.add(idx_hdr as usize)
                                                            } as *const u8, &raw mut i_serial as *mut u32)
                                                }) as i32
                                        } as u8 as u32;
                            n_serial = sqlite3_vdbe_serial_type_len(i_serial) as i32;
                            if idx_rec + n_serial as u32 > n_rec {
                                rc = unsafe { sqlite3_corrupt_error(5505) };
                            } else {
                                sqlite3_vdbe_serial_get(unsafe {
                                            &raw mut *a_rec.add(idx_rec as usize)
                                        } as *const u8, i_serial, &mut m);
                                if vdbe_skip_field(mask, ii,
                                            unsafe { &mut *(*p).a_mem.offset(ii as isize) }, &mut m,
                                            b_int_1) == 0 {
                                    res =
                                        sqlite3_mem_compare(&raw mut m as *const Mem,
                                            unsafe { &raw mut *(*p).a_mem.offset(ii as isize) } as
                                                *const Mem,
                                            unsafe {
                                                    *(unsafe { (*(*p).p_key_info).a_coll.as_ptr() } as
                                                                *mut *mut CollSeq).offset(ii as isize)
                                                } as *const CollSeq);
                                    if res != 0 { break '__b46; }
                                }
                            }
                            idx_rec += sqlite3_vdbe_serial_type_len(i_serial);
                            break '__c46;
                        }
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                    }
                }
                *pi_res_1 = res;
            }
        }
        unsafe { sqlite3_free(a_rec as *mut ()) };
        return rc;
    }
}

///* This is called when the record in (*p) should be found in the index 
///* opened by cursor pCur, but was not. This may happen as part of a DELETE
///* operation or an integrity check.
///*
///* One reason that an exact match was not found may be the EIIB bug - that
///* a text-to-float conversion may have caused a real value in record (*p)
///* to be slightly different from its counterpart on disk. This function
///* attempts to find the right index record. If it does find the right
///* record, it leaves *pCur pointing to it and sets (*pRes) to 0 before
///* returning. Otherwise, (*pRes) is set to non-zero and an SQLite error
///* code returned.
///*
///* The algorithm used to find the correct record is:
///*
///*   * Scan up to BTREE_FDK_RANGE entries either side of the current entry.
///*     If parameter bIntegrity is false, then all fields that are indexed
///*     expressions or virtual table columns are omitted from the comparison.
///*     If bIntegrity is true, then small differences in real values in
///*     such fields are overlooked, but they are not omitted from the comparison
///*     altogether.
///*
///*   * If the above fails to find an entry and bIntegrity is false, search 
///*     the entire index.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_vdbe_find_index_key(p_cur_1: *mut BtCursor,
    p_idx_1: &Index, p: *mut UnpackedRecord, p_res_1: &mut i32,
    b_integrity_1: i32) -> i32 {
    let mut n_step: i32 = 0;
    let mut res: i32 = 1;
    let mut rc: i32 = 0;
    let mut ii: i32 = 0;
    /// Calculate a mask based on the first 64 columns of the index. The mask
    ///* bit is set if the corresponding index field is either an expression
    ///* or a virtual column of the table.
    let mut mask: Bitmask = 0 as Bitmask;
    {
        ii = 0;
        '__b47: loop {
            if !(ii <
                            if ((*p_idx_1).n_column as i32) <
                                    (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                                (*p_idx_1).n_column as i32
                            } else {
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32
                            }) {
                break '__b47;
            }
            '__c47: loop {
                let i_col: i32 =
                    unsafe { *(*p_idx_1).ai_column.offset(ii as isize) } as i32;
                if i_col == -2 ||
                        i_col >= 0 &&
                            unsafe {
                                            (*unsafe {
                                                        (*(*p_idx_1).p_table).a_col.offset(i_col as isize)
                                                    }).col_flags
                                        } as i32 & 32 != 0 {
                    mask |= (1 as Bitmask) << ii;
                }
                break '__c47;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    if mask != 0 as u64 {
        {
            ii = 0;
            '__b48: loop {
                if !(unsafe { sqlite3_btree_eof(p_cur_1) } == 0 && ii < 10) {
                    break '__b48;
                }
                '__c48: loop {
                    rc = unsafe { sqlite3_btree_previous(p_cur_1, 0) };
                    break '__c48;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc == 101 {
            rc = unsafe { sqlite3_btree_first(p_cur_1, &mut res) };
            n_step = -1;
        } else { n_step = 10 * 2; }
        while unsafe { sqlite3_btree_cursor_is_valid_nn(p_cur_1) } != 0 {
            {
                ii = 0;
                '__b50: loop {
                    if !(rc == 0 && (ii < n_step || n_step < 0)) {
                        break '__b50;
                    }
                    '__c50: loop {
                        rc =
                            vdbe_is_matching_index_key(p_cur_1, b_integrity_1, mask,
                                unsafe { &*p }, &mut res);
                        if res == 0 || rc != 0 { break '__b50; }
                        rc = unsafe { sqlite3_btree_next(p_cur_1, 0) };
                        break '__c50;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            if rc == 101 { rc = 0; { let _ = 0; }; }
            if n_step < 0 || rc != 0 || res == 0 || b_integrity_1 != 0 {
                break;
            }

            /// The first, non-exhaustive, search failed to find an entry with 
            ///* matching PK fields. So restart for an exhaustive search of the 
            ///* entire index.
            (n_step = -1);
            rc = unsafe { sqlite3_btree_first(p_cur_1, &mut res) };
        }
    }
    *p_res_1 = res;
    return rc;
}

static mut dummy: VdbeOp = unsafe { core::mem::zeroed() };

static a_mj_needed: [u8; 6] =
    [1 as u8, 1 as u8, 0 as u8, 1 as u8, 0 as u8, 0 as u8];

static a_flag: [u16; 2] = [(16 | 16384) as u16, (2 | 16384) as u16];

static mut encnames: [*const i8; 4] =
    [c"?".as_ptr() as *const i8, c"8".as_ptr() as *const i8,
            c"16LE".as_ptr() as *const i8, c"16BE".as_ptr() as *const i8];

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
    fn sqlite3_db_malloc_raw_nn(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_db_realloc(_: *mut Sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_size(_: *mut Sqlite3, _: *const ())
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_db_nn_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_key_info_unref(_: *mut KeyInfo)
    -> ();
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_db_free(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_vtab_unlock(_: *mut VTable)
    -> ();
    fn sqlite3_delete_table(_: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_strlen30(_: *const i8)
    -> i32;
    fn sqlite3_db_str_n_dup(_: *mut Sqlite3, _: *const i8, _: u64)
    -> *mut i8;
    fn sqlite3_vtab_lock(_: *mut VTable)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_may_abort(_: *mut Parse)
    -> ();
    static sqlite3_opcode_property: [u8; 0];
    fn sqlite3_vm_printf(_: *mut Sqlite3, _: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_key_info_of_index(_: *mut Parse, _: *mut Index)
    -> *mut KeyInfo;
    fn sqlite3_vdbe_mem_release(p: *mut Mem)
    -> ();
    fn sqlite3_rc_str_unref(_: *mut ())
    -> ();
    fn sqlite3_vdbe_sorter_close(_: *mut Sqlite3, _: *mut VdbeCursor)
    -> ();
    fn sqlite3_rollback_all(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_close_savepoints(_: *mut Sqlite3)
    -> ();
    fn sqlite3_vtab_sync(db: *mut Sqlite3, _: *mut Vdbe)
    -> i32;
    fn sqlite3_vtab_commit(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_m_printf(_: *mut Sqlite3, _: *const i8, ...)
    -> *mut i8;
    fn sqlite3_begin_benign_malloc()
    -> ();
    fn sqlite3_end_benign_malloc()
    -> ();
    fn sqlite3_system_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_commit_internal_changes(_: *mut Sqlite3)
    -> ();
    fn sqlite3_vtab_savepoint(_: *mut Sqlite3, _: i32, _: i32)
    -> i32;
    fn sqlite3_value_new(_: *mut Sqlite3)
    -> *mut Sqlite3Value;
    fn sqlite3_value_set_str(_: *mut Sqlite3Value, _: i32, _: *const (),
    _: u8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_value_set_null(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_db_realloc_or_free(_: *mut Sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_progress_check(_: *mut Parse)
    -> ();
    fn sqlite3_vdbe_mem_set_text(_: *mut Mem, _: *const i8, _: i64,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_vdbe_mem_copy(_: *mut Mem, _: *const Mem)
    -> i32;
    fn sqlite3_value_apply_affinity(_: *mut Sqlite3Value, _: u8, _: u8)
    -> ();
    fn sqlite3_vdbe_expand_sql(_: *mut Vdbe, _: *const i8)
    -> *mut i8;
    fn sqlite3_is_na_n(_: f64)
    -> i32;
    fn sqlite3_vdbe_mem_init(_: *mut Mem, _: *mut Sqlite3, _: u16)
    -> ();
    fn sqlite3_vdbe_mem_shallow_copy(_: *mut Mem, _: *const Mem, _: i32)
    -> ();
    fn sqlite3ValueText(_: *mut Sqlite3Value, _: u8)
    -> *const ();
    fn sqlite3_vdbe_mem_release_malloc(p: *mut Mem)
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn sqlite3_get_varint32(_: *const u8, _: *mut u32)
    -> u8;
    fn sqlite3_vdbe_mem_set_null(_: *mut Mem)
    -> ();
    fn sqlite3_corrupt_error(_: i32)
    -> i32;
    fn sqlite3_varint_len(v: u64)
    -> i32;
    fn sqlite3_db_malloc_raw(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_mem_set_array_int64(a_mem_1: *mut Sqlite3Value, i_idx_1: i32,
    val: i64)
    -> ();
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
    fn sqlite3_misuse_error(_: i32)
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
    fn sqlite3_db_malloc_zero(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_str_dup(_: *mut Sqlite3, _: *const i8)
    -> *mut i8;
    fn sqlite3_db_span_dup(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut i8;
    fn sqlite3Realloc(_: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_malloc_size(_: *const ())
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
    fn sqlite3_is_overflow(_: f64)
    -> i32;
    fn sqlite3_fp_decode(_: *mut FpDecode, _: f64, _: i32, _: i32)
    -> ();
    fn sqlite3_set_string(_: *mut *mut i8, _: *mut Sqlite3, _: *const i8)
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
    fn sqlite3_v_list_num_to_name(_: *mut VList, _: i32)
    -> *const i8;
    fn sqlite3_v_list_name_to_num(_: *mut VList, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_put_varint(_: *mut u8, _: u64)
    -> i32;
    fn sqlite3_get_varint(_: *const u8, _: *mut u64)
    -> u8;
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
    fn sqlite3_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_error_clear(_: *mut Sqlite3)
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
    fn sqlite3ValueBytes(_: *mut Sqlite3Value, _: u8)
    -> i32;
    fn sqlite3_result_int_real(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_utf16to8(_: *mut Sqlite3, _: *const (), _: i32, _: u8)
    -> *mut i8;
    fn sqlite3_value_from_expr(_: *mut Sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut Sqlite3Value)
    -> i32;
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
    fn sqlite3_oom_clear(_: *mut Sqlite3)
    -> ();
    fn sqlite3_api_exit(db: *mut Sqlite3, _: i32)
    -> i32;
    fn sqlite3_open_temp_database(_: *mut Parse)
    -> i32;
    fn sqlite3_rc_str_ref(_: *mut i8)
    -> *mut i8;
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
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_vtab_clear(db: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_disconnect(db: *mut Sqlite3, p: *mut Table)
    -> ();
    fn sqlite3_vtab_rollback(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_module_unref(_: *mut Sqlite3, _: *mut Module)
    -> ();
    fn sqlite3_vtab_unlock_list(_: *mut Sqlite3)
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
    fn sqlite_vdbe_pop_stack(_: *mut Vdbe, _: i32)
    -> ();
    fn sqlite2_btree_key_compare(_: *mut BtCursor, _: *const (), _: i32,
    _: i32, _: *mut i32)
    -> i32;
    fn sqlite3_vdbe_mem_from_btree_zero_offset(_: *mut BtCursor, _: u32,
    _: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_exec(_: *mut Vdbe)
    -> i32;
    fn sqlite3_vdbe_mem_grow(p_mem_1: *mut Mem, n: i32, preserve: i32)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn sqlite3_vdbe_mem_set_int64(_: *mut Mem, _: i64)
    -> ();
    fn sqlite3_vdbe_mem_set_str(_: *mut Mem, _: *const i8, _: i64, _: u8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_opcode_name(_: i32)
    -> *const i8;
    fn sqlite3_vdbe_change_encoding(_: *mut Mem, _: i32)
    -> i32;
    fn sqlite3_vdbe_mem_too_big(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_move(_: *mut Mem, _: *mut Mem)
    -> ();
    fn sqlite3_vdbe_mem_nul_terminate(_: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_set_double(_: *mut Mem, _: f64)
    -> ();
    fn sqlite3_vdbe_mem_set_pointer(_: *mut Mem, _: *mut (), _: *const i8,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
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
    fn sqlite3_vdbe_mem_finalize(_: *mut Mem, _: *mut FuncDef)
    -> i32;
    fn sqlite3_vdbe_mem_agg_value(_: *mut Mem, _: *mut Mem, _: *mut FuncDef)
    -> i32;
    fn sqlite3_vdbe_mem_clear_and_resize(p_mem_1: *mut Mem, n: i32)
    -> i32;
    fn sqlite3_vdbe_sorter_init(_: *mut Sqlite3, _: i32, _: *mut VdbeCursor)
    -> i32;
    fn sqlite3_vdbe_sorter_reset(_: *mut Sqlite3, _: *mut VdbeSorter)
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
    fn sqlite3_vdbe_value_list_free(_: *mut ())
    -> ();
    fn sqlite3_vdbe_mem_translate(_: *mut Mem, _: u8)
    -> i32;
    fn sqlite3_vdbe_mem_handle_bom(p_mem_1: *mut Mem)
    -> i32;
    fn sqlite3_vdbe_mem_expand_blob(_: *mut Mem)
    -> i32;
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_va_start(_: &mut *mut i8, ...)
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
