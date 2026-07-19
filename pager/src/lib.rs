#![allow(unused_imports, dead_code)]

mod btree_h;
mod hash_h;
mod pager_h;
mod pcache_h;
mod sqlite3_h;
mod sqlite_int_h;
mod vdbe_h;
mod wal_h;
use crate::btree_h::{BtCursor, Btree, BtreePayload};
use crate::hash_h::Hash;
use crate::pager_h::{DbPage, Pgno};
use crate::pcache_h::{PCache, PgHdr};
use crate::sqlite3_h::{
    Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File, Sqlite3Filename,
    Sqlite3IndexInfo, Sqlite3Int64, Sqlite3IoMethods, Sqlite3Module,
    Sqlite3Mutex, Sqlite3MutexMethods, Sqlite3PcachePage,
    Sqlite3RtreeGeometry, Sqlite3RtreeQueryInfo, Sqlite3Snapshot, Sqlite3Stmt,
    Sqlite3Uint64, Sqlite3Value, Sqlite3Vfs, Sqlite3Vtab,
};
use crate::sqlite_int_h::{
    AuthContext, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte, DbFixer,
    Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode, FuncDef,
    FuncDefHash, FuncDestructor, IdList, Index, KeyInfo, LogEst, Module,
    NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema, Select,
    SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str, SrcItem,
    SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
    TriggerStep, UnpackedRecord, Upsert, VList, VTable, Walker, WhereInfo,
    Window, With,
};
use crate::vdbe_h::{Mem, SubProgram, Vdbe, VdbeOp, VdbeOpList};
use crate::wal_h::Wal;

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

///* An open page cache is an instance of struct Pager. A description of
///* some of the more important member variables follows:
///*
///* eState
///*
///*   The current 'state' of the pager object. See the comment and state
///*   diagram above for a description of the pager state.
///*
///* eLock
///*
///*   For a real on-disk database, the current lock held on the database file -
///*   NO_LOCK, SHARED_LOCK, RESERVED_LOCK or EXCLUSIVE_LOCK.
///*
///*   For a temporary or in-memory database (neither of which require any
///*   locks), this variable is always set to EXCLUSIVE_LOCK. Since such
///*   databases always have Pager.exclusiveMode==1, this tricks the pager
///*   logic into thinking that it already has all the locks it will ever
///*   need (and no reason to release them).
///*
///*   In some (obscure) circumstances, this variable may also be set to
///*   UNKNOWN_LOCK. See the comment above the #define of UNKNOWN_LOCK for
///*   details.
///*
///* changeCountDone
///*
///*   This boolean variable is used to make sure that the change-counter
///*   (the 4-byte header field at byte offset 24 of the database file) is
///*   not updated more often than necessary.
///*
///*   It is set to true when the change-counter field is updated, which
///*   can only happen if an exclusive lock is held on the database file.
///*   It is cleared (set to false) whenever an exclusive lock is
///*   relinquished on the database file. Each time a transaction is committed,
///*   The changeCountDone flag is inspected. If it is true, the work of
///*   updating the change-counter is omitted for the current transaction.
///*
///*   This mechanism means that when running in exclusive mode, a connection
///*   need only update the change-counter once, for the first transaction
///*   committed.
///*
///* setSuper
///*
///*   When PagerCommitPhaseOne() is called to commit a transaction, it may
///*   (or may not) specify a super-journal name to be written into the
///*   journal file before it is synced to disk.
///*
///*   Whether or not a journal file contains a super-journal pointer affects
///*   the way in which the journal file is finalized after the transaction is
///*   committed or rolled back when running in "journal_mode=PERSIST" mode.
///*   If a journal file does not contain a super-journal pointer, it is
///*   finalized by overwriting the first journal header with zeroes. If
///*   it does contain a super-journal pointer the journal file is finalized
///*   by truncating it to zero bytes, just as if the connection were
///*   running in "journal_mode=truncate" mode.
///*
///*   Journal files that contain super-journal pointers cannot be finalized
///*   simply by overwriting the first journal-header with zeroes, as the
///*   super-journal pointer could interfere with hot-journal rollback of any
///*   subsequently interrupted transaction that reuses the journal file.
///*
///*   The flag is cleared as soon as the journal file is finalized (either
///*   by PagerCommitPhaseTwo or PagerRollback). If an IO error prevents the
///*   journal file from being successfully finalized, the setSuper flag
///*   is cleared anyway (and the pager will move to ERROR state).
///*
///* doNotSpill
///*
///*   This variables control the behavior of cache-spills  (calls made by
///*   the pcache module to the pagerStress() routine to write cached data
///*   to the file-system in order to free up memory).
///*
///*   When bits SPILLFLAG_OFF or SPILLFLAG_ROLLBACK of doNotSpill are set,
///*   writing to the database from pagerStress() is disabled altogether.
///*   The SPILLFLAG_ROLLBACK case is done in a very obscure case that
///*   comes up during savepoint rollback that requires the pcache module
///*   to allocate a new page to prevent the journal file from being written
///*   while it is being traversed by code in pager_playback().  The SPILLFLAG_OFF
///*   case is a user preference.
///*
///*   If the SPILLFLAG_NOSYNC bit is set, writing to the database from
///*   pagerStress() is permitted, but syncing the journal file is not.
///*   This flag is set by sqlite3PagerWrite() when the file-system sector-size
///*   is larger than the database page-size in order to prevent a journal sync
///*   from happening in between the journalling of two pages on the same sector.
///*
///* subjInMemory
///*
///*   This is a boolean variable. If true, then any required sub-journal
///*   is opened as an in-memory journal file. If false, then in-memory
///*   sub-journals are only used for in-memory pager files.
///*
///*   This variable is updated by the upper layer each time a new
///*   write-transaction is opened.
///*
///* dbSize, dbOrigSize, dbFileSize
///*
///*   Variable dbSize is set to the number of pages in the database file.
///*   It is valid in PAGER_READER and higher states (all states except for
///*   OPEN and ERROR).
///*
///*   dbSize is set based on the size of the database file, which may be
///*   larger than the size of the database (the value stored at offset
///*   28 of the database header by the btree). If the size of the file
///*   is not an integer multiple of the page-size, the value stored in
///*   dbSize is rounded down (i.e. a 5KB file with 2K page-size has dbSize==2).
///*   Except, any file that is greater than 0 bytes in size is considered
///*   to have at least one page. (i.e. a 1KB file with 2K page-size leads
///*   to dbSize==1).
///*
///*   During a write-transaction, if pages with page-numbers greater than
///*   dbSize are modified in the cache, dbSize is updated accordingly.
///*   Similarly, if the database is truncated using PagerTruncateImage(),
///*   dbSize is updated.
///*
///*   Variables dbOrigSize and dbFileSize are valid in states
///*   PAGER_WRITER_LOCKED and higher. dbOrigSize is a copy of the dbSize
///*   variable at the start of the transaction. It is used during rollback,
///*   and to determine whether or not pages need to be journalled before
///*   being modified.
///*
///*   Throughout a write-transaction, dbFileSize contains the size of
///*   the file on disk in pages. It is set to a copy of dbSize when the
///*   write-transaction is first opened, and updated when VFS calls are made
///*   to write or truncate the database file on disk.
///*
///*   The only reason the dbFileSize variable is required is to suppress
///*   unnecessary calls to xTruncate() after committing a transaction. If,
///*   when a transaction is committed, the dbFileSize variable indicates
///*   that the database file is larger than the database image (Pager.dbSize),
///*   pager_truncate() is called. The pager_truncate() call uses xFilesize()
///*   to measure the database file on disk, and then truncates it if required.
///*   dbFileSize is not used when rolling back a transaction. In this case
///*   pager_truncate() is called unconditionally (which means there may be
///*   a call to xFilesize() that is not strictly required). In either case,
///*   pager_truncate() may cause the file to become smaller or larger.
///*
///* dbHintSize
///*
///*   The dbHintSize variable is used to limit the number of calls made to
///*   the VFS xFileControl(FCNTL_SIZE_HINT) method.
///*
///*   dbHintSize is set to a copy of the dbSize variable when a
///*   write-transaction is opened (at the same time as dbFileSize and
///*   dbOrigSize). If the xFileControl(FCNTL_SIZE_HINT) method is called,
///*   dbHintSize is increased to the number of pages that correspond to the
///*   size-hint passed to the method call. See pager_write_pagelist() for
///*   details.
///*
///* errCode
///*
///*   The Pager.errCode variable is only ever used in PAGER_ERROR state. It
///*   is set to zero in all other states. In PAGER_ERROR state, Pager.errCode
///*   is always set to SQLITE_FULL, SQLITE_IOERR or one of the SQLITE_IOERR_XXX
///*   sub-codes.
///*
///* syncFlags, walSyncFlags
///*
///*   syncFlags is either SQLITE_SYNC_NORMAL (0x02) or SQLITE_SYNC_FULL (0x03).
///*   syncFlags is used for rollback mode.  walSyncFlags is used for WAL mode
///*   and contains the flags used to sync the checkpoint operations in the
///*   lower two bits, and sync flags used for transaction commits in the WAL
///*   file in bits 0x04 and 0x08.  In other words, to get the correct sync flags
///*   for checkpoint operations, use (walSyncFlags&0x03) and to get the correct
///*   sync flags for transaction commit, use ((walSyncFlags>>2)&0x03).  Note
///*   that with synchronous=NORMAL in WAL mode, transaction commit is not synced
///*   meaning that the 0x04 and 0x08 bits are both zero.
#[repr(C)]
#[derive(Copy, Clone)]
struct Pager {
    p_vfs: *mut Sqlite3Vfs,
    exclusive_mode: u8,
    journal_mode: u8,
    use_journal: u8,
    no_sync: u8,
    full_sync: u8,
    extra_sync: u8,
    sync_flags: u8,
    wal_sync_flags: u8,
    temp_file: u8,
    no_lock: u8,
    read_only: u8,
    mem_db: u8,
    mem_vfs: u8,
    e_state: u8,
    e_lock: u8,
    change_count_done: u8,
    set_super: u8,
    do_not_spill: u8,
    subj_in_memory: u8,
    b_use_fetch: u8,
    has_held_shared_lock: u8,
    db_size: Pgno,
    db_orig_size: Pgno,
    db_file_size: Pgno,
    db_hint_size: Pgno,
    err_code: i32,
    n_rec: i32,
    cksum_init: u32,
    n_sub_rec: u32,
    p_in_journal: *mut Bitvec,
    fd: *mut Sqlite3File,
    jfd: *mut Sqlite3File,
    sjfd: *mut Sqlite3File,
    journal_off: i64,
    journal_hdr: i64,
    p_backup: *mut Sqlite3Backup,
    a_savepoint: *mut PagerSavepoint,
    n_savepoint: i32,
    i_data_version: u32,
    db_file_vers: [i8; 16],
    n_mmap_out: i32,
    sz_mmap: Sqlite3Int64,
    p_mmap_freelist: *mut PgHdr,
    n_extra: u16,
    n_reserve: i16,
    vfs_flags: u32,
    sector_size: u32,
    mx_pgno: Pgno,
    lck_pgno: Pgno,
    page_size: i64,
    journal_size_limit: i64,
    z_filename: *mut i8,
    z_journal: *mut i8,
    x_busy_handler: unsafe extern "C" fn(*mut ()) -> i32,
    p_busy_handler_arg: *mut (),
    a_stat: [u32; 4],
    x_reiniter: Option<unsafe extern "C" fn(*mut PgHdr) -> ()>,
    x_get: Option<unsafe extern "C" fn(*mut Pager, u32, *mut *mut PgHdr, i32)
        -> i32>,
    p_tmp_space: *mut i8,
    p_p_cache: *mut PCache,
    p_wal: *mut Wal,
    z_wal: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct PagerSavepoint {
    i_offset: i64,
    i_hdr_offset: i64,
    p_in_savepoint: *mut Bitvec,
    n_orig: Pgno,
    i_sub_rec: Pgno,
    b_truncate_on_release: i32,
    a_wal_data: [u32; 4],
}

///* CAPI3REF:  Database File Corresponding To A Journal
///*
///* ^If X is the name of a rollback or WAL-mode journal file that is
///* passed into the xOpen method of [sqlite3_vfs], then
///* sqlite3_database_file_object(X) returns a pointer to the [sqlite3_file]
///* object that represents the main database file.
///*
///* This routine is intended for use in custom [VFS] implementations
///* only.  It is not a general-purpose interface.
///* The argument sqlite3_file_object(X) must be a filename pointer that
///* has been passed into [sqlite3_vfs].xOpen method where the
///* flags parameter to xOpen contains one of the bits
///* [SQLITE_OPEN_MAIN_JOURNAL] or [SQLITE_OPEN_WAL].  Any other use
///* of this routine results in undefined and probably undesirable
///* behavior.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_database_file_object(mut z_name: *const i8)
    -> *mut Sqlite3File {
    let mut p_pager: *const Pager = core::ptr::null();
    let mut p: *const i8 = core::ptr::null();
    while unsafe { *z_name.offset(-1 as isize) } as i32 != 0 ||
                    unsafe { *z_name.offset(-2 as isize) } as i32 != 0 ||
                unsafe { *z_name.offset(-3 as isize) } as i32 != 0 ||
            unsafe { *z_name.offset(-4 as isize) } as i32 != 0 {
        {
            let __p = &mut z_name;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(-1) };
            __t
        };
    }
    p =
        unsafe {
            unsafe {
                z_name.offset(-(4 as
                                isize)).sub(core::mem::size_of::<*mut Pager>() as usize)
            }
        };
    { let _ = 0; };
    p_pager = unsafe { *(p as *mut *mut Pager) };
    return unsafe { (*p_pager).fd };
}

///* Return a sanitized version of the sector-size of OS file pFile. The
///* return value is guaranteed to lie between 32 and MAX_SECTOR_SIZE.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_sector_size(p_file: *mut Sqlite3File) -> i32 {
    let mut i_ret: i32 = unsafe { sqlite3_os_sector_size(p_file) };
    if i_ret < 32 {
        i_ret = 512;
    } else if i_ret > 65536 { { let _ = 0; }; i_ret = 65536; }
    return i_ret;
}

///* Set the value of the Pager.sectorSize variable for the given
///* pager based on the value returned by the xSectorSize method
///* of the open database file. The sector size will be used
///* to determine the size and alignment of journal header and
///* super-journal pointers within created journal files.
///*
///* For temporary files the effective sector size is always 512 bytes.
///*
///* Otherwise, for non-temporary files, the effective sector size is
///* the value returned by the xSectorSize() method rounded up to 32 if
///* it is less than 32, or rounded down to MAX_SECTOR_SIZE if it
///* is greater than MAX_SECTOR_SIZE.
///*
///* If the file has the SQLITE_IOCAP_POWERSAFE_OVERWRITE property, then set
///* the effective sector size to its minimum value (512).  The purpose of
///* pPager->sectorSize is to define the "blast radius" of bytes that
///* might change if a crash occurs while writing to a single byte in
///* that range.  But with POWERSAFE_OVERWRITE, the blast radius is zero
///* (that is what POWERSAFE_OVERWRITE means), so we minimize the sector
///* size.  For backwards compatibility of the rollback journal file format,
///* we cannot reduce the effective sector size below 512.
#[allow(unused_doc_comments)]
extern "C" fn set_sector_size(p_pager_1: &mut Pager) -> () {
    { let _ = 0; };
    if (*p_pager_1).temp_file != 0 ||
            unsafe { sqlite3_os_device_characteristics((*p_pager_1).fd) } &
                    4096 != 0 {

        /// Sector size doesn't matter for temporary files. Also, the file
        ///* may not have been opened yet, in which case the OsSectorSize()
        ///* call will segfault.
        ((*p_pager_1).sector_size = 512 as u32);
    } else {
        (*p_pager_1).sector_size =
            sqlite3_sector_size((*p_pager_1).fd) as u32;
    }
}

///* Discard the entire contents of the in-memory page-cache.
extern "C" fn pager_reset(p_pager_1: &mut Pager) -> () {
    {
        let __p = &mut (*p_pager_1).i_data_version;
        let __t = *__p;
        *__p += 1;
        __t
    };
    unsafe { sqlite3_backup_restart((*p_pager_1).p_backup) };
    unsafe { sqlite3_pcache_clear((*p_pager_1).p_p_cache) };
}

/// The page getter method for when the pager is an error state
extern "C" fn get_page_error(p_pager_1: *mut Pager, pgno: Pgno,
    pp_page_1: *mut *mut DbPage, flags: i32) -> i32 {
    { let _ = pgno; };
    { let _ = flags; };
    { let _ = 0; };
    unsafe { *pp_page_1 = core::ptr::null_mut() };
    return unsafe { (*p_pager_1).err_code };
}

///* Acquire a page if it is already in the in-memory cache.  Do
///* not read the page from disk.  Return a pointer to the page,
///* or 0 if the page is not in cache.
///*
///* See also sqlite3PagerGet().  The difference between this routine
///* and sqlite3PagerGet() is that _get() will go to the disk and read
///* in the page if the page is not already in cache.  This routine
///* returns NULL if the page is not in cache or if a disk I/O error
///* has ever happened.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_lookup(p_pager_1: &Pager, pgno: Pgno)
    -> *mut DbPage {
    let mut p_page: *mut Sqlite3PcachePage = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    p_page = unsafe { sqlite3_pcache_fetch((*p_pager_1).p_p_cache, pgno, 0) };
    { let _ = 0; };
    if p_page == core::ptr::null_mut() { return core::ptr::null_mut(); }
    return unsafe {
                sqlite3_pcache_fetch_finish((*p_pager_1).p_p_cache, pgno,
                    p_page)
            } as *mut DbPage;
}

///* Obtain a reference to a memory mapped page object for page number pgno.
///* The new object will use the pointer pData, obtained from xFetch().
///* If successful, set *ppPage to point to the new page reference
///* and return SQLITE_OK. Otherwise, return an SQLite error code and set
///* *ppPage to zero.
///*
///* Page references obtained by calling this function should be released
///* by calling pagerReleaseMapPage().
extern "C" fn pager_acquire_map_page(p_pager_1: *mut Pager, pgno: Pgno,
    p_data_1: *mut (), pp_page_1: &mut *mut PgHdr) -> i32 {
    let mut p: *mut PgHdr = core::ptr::null_mut();
    if !(unsafe { (*p_pager_1).p_mmap_freelist }).is_null() {
        *pp_page_1 = { p = unsafe { (*p_pager_1).p_mmap_freelist }; p };
        unsafe { (*p_pager_1).p_mmap_freelist = unsafe { (*p).p_dirty } };
        unsafe { (*p).p_dirty = core::ptr::null_mut() };
        { let _ = 0; };
        unsafe { memset(unsafe { (*p).p_extra }, 0, 8 as u64) };
    } else {
        *pp_page_1 =
            {
                p =
                    unsafe {
                            sqlite3_malloc_zero(core::mem::size_of::<PgHdr>() as u64 +
                                    unsafe { (*p_pager_1).n_extra } as u64)
                        } as *mut PgHdr;
                p
            };
        if p == core::ptr::null_mut() {
            unsafe {
                sqlite3_os_unfetch(unsafe { (*p_pager_1).fd },
                    (pgno - 1 as Pgno) as i64 *
                        unsafe { (*p_pager_1).page_size }, p_data_1)
            };
            return 7;
        }
        unsafe {
            (*p).p_extra =
                unsafe { &raw mut *p.offset(1 as isize) } as *mut ()
        };
        { let _ = 0; };
        unsafe { (*p).flags = 32 as u16 };
        unsafe { (*p).n_ref = 1 as i64 };
        unsafe { (*p).p_pager = p_pager_1 };
    }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { (*p).pgno = pgno };
    unsafe { (*p).p_data = p_data_1 };
    {
        let __p = unsafe { &mut (*p_pager_1).n_mmap_out };
        let __t = *__p;
        *__p += 1;
        __t
    };
    return 0;
}

///* Set the bit number pgno in the PagerSavepoint.pInSavepoint
///* bitvecs of all open savepoints. Return SQLITE_OK if successful
///* or SQLITE_NOMEM if a malloc failure occurs.
#[allow(unused_doc_comments)]
extern "C" fn add_to_savepoint_bitvecs(p_pager_1: &Pager, pgno: Pgno) -> i32 {
    let mut ii: i32 = 0;
    /// Loop counter
    let mut rc: i32 = 0;
    {
        ii = 0;
        '__b1: loop {
            if !(ii < (*p_pager_1).n_savepoint) { break '__b1; }
            '__c1: loop {
                let p: *const PagerSavepoint =
                    unsafe {
                            &raw mut *(*p_pager_1).a_savepoint.offset(ii as isize)
                        } as *const PagerSavepoint;
                if pgno <= unsafe { (*p).n_orig } {
                    rc |=
                        unsafe {
                            sqlite3_bitvec_set(unsafe { (*p).p_in_savepoint }, pgno)
                        };
                    { let _ = 0; };
                }
                break '__c1;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return rc;
}

///* Read the content for page pPg out of the database file (or out of
///* the WAL if that is where the most recent copy if found) into
///* pPg->pData. A shared lock or greater must be held on the database
///* file before this function is called.
///*
///* If page 1 is read, then the value of Pager.dbFileVers[] is set to
///* the value read from the database file.
///*
///* If an IO error occurs, then the IO error is returned to the caller.
///* Otherwise, SQLITE_OK is returned.
#[allow(unused_doc_comments)]
extern "C" fn read_db_page(p_pg_1: &mut PgHdr) -> i32 {
    let p_pager: *mut Pager = (*p_pg_1).p_pager;
    /// Pager object associated with page pPg
    let mut rc: i32 = 0;
    /// Return code
    let mut i_frame: u32 = 0 as u32;

    /// Frame of WAL containing pgno
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_pager).p_wal } != core::ptr::null_mut() {
        rc =
            unsafe {
                sqlite3_wal_find_frame(unsafe { (*p_pager).p_wal },
                    (*p_pg_1).pgno, &mut i_frame)
            };
        if rc != 0 { return rc; }
    }
    if i_frame != 0 {
        rc =
            unsafe {
                sqlite3_wal_read_frame(unsafe { (*p_pager).p_wal }, i_frame,
                    unsafe { (*p_pager).page_size } as i32,
                    (*p_pg_1).p_data as *mut u8)
            };
    } else {
        let i_offset: i64 =
            ((*p_pg_1).pgno - 1 as Pgno) as i64 *
                unsafe { (*p_pager).page_size } as i64;
        rc =
            unsafe {
                sqlite3_os_read(unsafe { (*p_pager).fd }, (*p_pg_1).p_data,
                    unsafe { (*p_pager).page_size } as i32, i_offset)
            };
        if rc == 10 | 2 << 8 { rc = 0; }
    }
    if (*p_pg_1).pgno == 1 as u32 {
        if rc != 0 {

            /// If the read is unsuccessful, set the dbFileVers[] to something
            ///* that will never be a valid file version.  dbFileVers[] is a copy
            ///* of bytes 24..39 of the database.  Bytes 28..31 should always be
            ///* zero or the size of the database in page. Bytes 32..35 and 35..39
            ///* should be page numbers which are never 0xffffffff.  So filling
            ///* pPager->dbFileVers[] with all 0xff bytes should suffice.
            ///*
            ///* For an encrypted database, the situation is more complex:  bytes
            ///* 24..39 of the database are white noise.  But the probability of
            ///* white noise equaling 16 bytes of 0xff is vanishingly small so
            ///* we should still be ok.
            unsafe {
                memset(unsafe { &raw mut (*p_pager).db_file_vers[0 as usize] }
                            as *mut i8 as *mut (), 255,
                    core::mem::size_of::<[i8; 16]>() as u64)
            };
        } else {
            let db_file_vers: *const u8 =
                unsafe {
                        &raw mut *((*p_pg_1).p_data as *mut u8).offset(24 as isize)
                    } as *const u8;
            unsafe {
                memcpy(unsafe { &raw mut (*p_pager).db_file_vers } as *mut (),
                    db_file_vers as *const (),
                    core::mem::size_of::<[i8; 16]>() as u64)
            };
        }
    }
    return rc;
}

///* Release a reference to page pPg. pPg must have been returned by an
///* earlier call to pagerAcquireMapPage().
extern "C" fn pager_release_map_page(p_pg_1: *mut PgHdr) -> () {
    let p_pager: *mut Pager = unsafe { (*p_pg_1).p_pager };
    {
        let __p = unsafe { &mut (*p_pager).n_mmap_out };
        let __t = *__p;
        *__p -= 1;
        __t
    };
    unsafe { (*p_pg_1).p_dirty = unsafe { (*p_pager).p_mmap_freelist } };
    unsafe { (*p_pager).p_mmap_freelist = p_pg_1 };
    { let _ = 0; };
    unsafe {
        sqlite3_os_unfetch(unsafe { (*p_pager).fd },
            (unsafe { (*p_pg_1).pgno } - 1 as Pgno) as i64 *
                unsafe { (*p_pager).page_size }, unsafe { (*p_pg_1).p_data })
    };
}

///* Release a page reference.
///*
///* The sqlite3PagerUnref() and sqlite3PagerUnrefNotNull() may only be used
///* if we know that the page being released is not the last reference to page1.
///* The btree layer always holds page1 open until the end, so these first
///* two routines can be used to release any page other than BtShared.pPage1.
///* The assert() at tag-20230419-2 proves that this constraint is always
///* honored.
///*
///* Use sqlite3PagerUnrefPageOne() to release page1.  This latter routine
///* checks the total number of outstanding pages and if the number of
///* pages reaches zero it drops the database lock.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_unref_not_null(p_pg_1: *mut DbPage) -> () {
    { let _ = 0; };
    if unsafe { (*p_pg_1).flags } as i32 & 32 != 0 {
        { let _ = 0; };

        /// Page1 is never memory mapped
        pager_release_map_page(p_pg_1 as *mut PgHdr);
    } else { unsafe { sqlite3_pcache_release(p_pg_1 as *mut PgHdr) }; }

    /// Do not use this routine to release the last reference to page1
    { let _ = 0; };
}

///* This function is invoked once for each page that has already been
///* written into the log file when a WAL transaction is rolled back.
///* Parameter iPg is the page number of said page. The pCtx argument
///* is actually a pointer to the Pager structure.
///*
///* If page iPg is present in the cache, and has no outstanding references,
///* it is discarded. Otherwise, if there are one or more outstanding
///* references, the page content is reloaded from the database. If the
///* attempt to reload content from the database is required and fails,
///* return an SQLite error code. Otherwise, SQLITE_OK.
#[allow(unused_doc_comments)]
extern "C" fn pager_undo_callback(p_ctx_1: *mut (), i_pg_1: Pgno) -> i32 {
    let mut rc: i32 = 0;
    let p_pager: *mut Pager = p_ctx_1 as *mut Pager;
    let mut p_pg: *mut PgHdr = core::ptr::null_mut();
    { let _ = 0; };
    p_pg =
        unsafe { sqlite3_pager_lookup(unsafe { &*p_pager }, i_pg_1) } as
            *mut PgHdr;
    if !(p_pg).is_null() {
        if unsafe { sqlite3_pcache_page_refcount(p_pg) } == 1 as i64 {
            unsafe { sqlite3_pcache_drop(p_pg) };
        } else {
            rc = read_db_page(unsafe { &mut *p_pg });
            if rc == 0 {
                unsafe { (unsafe { (*p_pager).x_reiniter.unwrap() })(p_pg) };
            }
            unsafe { sqlite3_pager_unref_not_null(p_pg as *mut DbPage) };
        }
    }

    /// Normally, if a transaction is rolled back, any backup processes are
    ///* updated as data is copied out of the rollback journal and into the
    ///* database. This is not generally possible with a WAL database, as
    ///* rollback involves simply truncating the log file. Therefore, if one
    ///* or more frames have already been written to the log (and therefore
    ///* also copied into the backup databases) as part of this transaction,
    ///* the backups must be restarted.
    unsafe { sqlite3_backup_restart(unsafe { (*p_pager).p_backup }) };
    return rc;
}

///* This function is called to rollback a transaction on a WAL database.
#[allow(unused_doc_comments)]
extern "C" fn pager_rollback_wal(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = 0;
    /// Return Code
    let mut p_list: *const PgHdr = core::ptr::null();

    /// List of dirty pages to revert
    /// For all pages in the cache that are currently dirty or have already
    ///* been written (but not committed) to the log file, do one of the
    ///* following:
    ///*
    ///*   + Discard the cached page (if refcount==0), or
    ///*   + Reload page content from the database (if refcount>0).
    unsafe { (*p_pager_1).db_size = unsafe { (*p_pager_1).db_orig_size } };
    rc =
        unsafe {
            sqlite3_wal_undo(unsafe { (*p_pager_1).p_wal },
                Some(pager_undo_callback), p_pager_1 as *mut ())
        };
    p_list =
        unsafe {
            sqlite3_pcache_dirty_list(unsafe { (*p_pager_1).p_p_cache })
        };
    while !(p_list).is_null() && rc == 0 {
        let p_next: *mut PgHdr = unsafe { (*p_list).p_dirty };
        rc =
            pager_undo_callback(p_pager_1 as *mut (),
                unsafe { (*p_list).pgno });
        p_list = p_next;
    }
    return rc;
}

///* Read a 32-bit integer from the given file descriptor.  Store the integer
///* that is read in *pRes.  Return SQLITE_OK if everything worked, or an
///* error code is something goes wrong.
///*
///* All values are stored on disk as big-endian.
extern "C" fn read32bits(fd: *mut Sqlite3File, offset: i64, p_res_1: &mut u32)
    -> i32 {
    let mut ac: [u8; 4] = [0; 4];
    let rc: i32 =
        unsafe {
            sqlite3_os_read(fd, &raw mut ac[0 as usize] as *mut u8 as *mut (),
                core::mem::size_of::<[u8; 4]>() as i32, offset)
        };
    if rc == 0 {
        *p_res_1 =
            unsafe {
                sqlite3_get4byte(&raw mut ac[0 as usize] as *mut u8 as
                        *const u8)
            };
    }
    return rc;
}

///* Parameter aData must point to a buffer of pPager->pageSize bytes
///* of data. Compute and return a checksum based on the contents of the
///* page of data and the current value of pPager->cksumInit.
///*
///* This is not a real checksum. It is really just the sum of the
///* random initial value (pPager->cksumInit) and every 200th byte
///* of the page data, starting with byte offset (pPager->pageSize%200).
///* Each byte is interpreted as an 8-bit unsigned integer.
///*
///* Changing the formula used to compute this checksum results in an
///* incompatible journal file format.
///*
///* If journal corruption occurs due to a power failure, the most likely
///* scenario is that one end or the other of the record will be changed.
///* It is much less likely that the two ends of the journal record will be
///* correct and the middle be corrupt.  Thus, this "checksum" scheme,
///* though fast and simple, catches the mostly likely kind of corruption.
#[allow(unused_doc_comments)]
extern "C" fn pager_cksum(p_pager_1: &Pager, a_data_1: *const u8) -> u32 {
    let mut cksum: u32 = (*p_pager_1).cksum_init;
    /// Checksum value to return
    let mut i: i32 = ((*p_pager_1).page_size - 200 as i64) as i32;
    while i > 0 {
        cksum += unsafe { *a_data_1.offset(i as isize) } as u32;
        i -= 200;
    }
    return cksum;
}

/// Functions used to obtain and release page references.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_get(p_pager_1: *mut Pager, pgno: Pgno,
    pp_page_1: *mut *mut DbPage, flags: i32) -> i32 {

    /// Trace page fetch by setting to 1
    /// Normal, high-speed version of sqlite3PagerGet()
    return unsafe {
            (unsafe {
                    (*p_pager_1).x_get.unwrap()
                })(p_pager_1, pgno, pp_page_1 as *mut *mut PgHdr, flags)
        };
}

///* Read a single page from either the journal file (if isMainJrnl==1) or
///* from the sub-journal (if isMainJrnl==0) and playback that page.
///* The page begins at offset *pOffset into the file. The *pOffset
///* value is increased to the start of the next page in the journal.
///*
///* The main rollback journal uses checksums - the statement journal does
///* not.
///*
///* If the page number of the page record read from the (sub-)journal file
///* is greater than the current value of Pager.dbSize, then playback is
///* skipped and SQLITE_OK is returned.
///*
///* If pDone is not NULL, then it is a record of pages that have already
///* been played back.  If the page at *pOffset has already been played back
///* (if the corresponding pDone bit is set) then skip the playback.
///* Make sure the pDone bit corresponding to the *pOffset page is set
///* prior to returning.
///*
///* If the page record is successfully read from the (sub-)journal file
///* and played back, then SQLITE_OK is returned. If an IO error occurs
///* while reading the record from the (sub-)journal file or while writing
///* to the database file, then the IO error code is returned. If data
///* is successfully read from the (sub-)journal file but appears to be
///* corrupted, SQLITE_DONE is returned. Data is considered corrupted in
///* two circumstances:
///*
///*   * If the record page-number is illegal (0 or PAGER_SJ_PGNO), or
///*   * If the record is being rolled back from the main journal file
///*     and the checksum field does not match the record content.
///*
///* Neither of these two scenarios are possible during a savepoint rollback.
///*
///* If this is a savepoint rollback, then memory may have to be dynamically
///* allocated by this function. If this is the case and an allocation fails,
///* SQLITE_NOMEM is returned.
#[allow(unused_doc_comments)]
extern "C" fn pager_playback_one_page(p_pager_1: *mut Pager,
    p_offset_1: &mut i64, p_done_1: *mut Bitvec, is_main_jrnl_1: i32,
    is_savepnt_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut p_pg: *mut PgHdr = core::ptr::null_mut();
    /// An existing page in the cache
    let mut pgno: Pgno = 0 as Pgno;
    /// The page number of a page in journal
    let mut cksum: u32 = 0 as u32;
    /// Checksum used for sanity checking
    let mut a_data: *mut i8 = core::ptr::null_mut();
    /// Temporary storage for the page
    let mut jfd: *mut Sqlite3File = core::ptr::null_mut();
    /// The file descriptor for the journal file
    let mut is_synced: i32 = 0;

    /// True if journal page is synced
    { let _ = 0; };

    /// isMainJrnl is 0 or 1
    { let _ = 0; };

    /// isSavepnt is 0 or 1
    { let _ = 0; };

    /// pDone always used on sub-journals
    { let _ = 0; };

    /// pDone never used on non-savepoint
    (a_data = unsafe { (*p_pager_1).p_tmp_space });
    { let _ = 0; };

    /// Temp storage must have already been allocated
    { let _ = 0; };

    /// Either the state is greater than PAGER_WRITER_CACHEMOD (a transaction
    ///* or savepoint rollback done at the request of the caller) or this is
    ///* a hot-journal rollback. If it is a hot-journal rollback, the pager
    ///* is in state OPEN and holds an EXCLUSIVE lock. Hot-journal rollback
    ///* only reads from the main journal, not the sub-journal.
    { let _ = 0; };
    { let _ = 0; };

    /// Read the page number and page data from the journal or sub-journal
    ///* file. Return an error code to the caller if an IO error occurs.
    (jfd =
        if is_main_jrnl_1 != 0 {
            unsafe { (*p_pager_1).jfd }
        } else { unsafe { (*p_pager_1).sjfd } });
    rc = read32bits(jfd, *p_offset_1, &mut pgno);
    if rc != 0 { return rc; }
    rc =
        unsafe {
            sqlite3_os_read(jfd, a_data as *mut u8 as *mut (),
                unsafe { (*p_pager_1).page_size } as i32,
                *p_offset_1 + 4 as i64)
        };
    if rc != 0 { return rc; }
    *p_offset_1 +=
        unsafe { (*p_pager_1).page_size } + 4 as i64 +
            (is_main_jrnl_1 * 4) as i64;
    if pgno == 0 as u32 || pgno == unsafe { (*p_pager_1).lck_pgno } {
        { let _ = 0; };
        return 101;
    }
    if pgno > unsafe { (*p_pager_1).db_size } as Pgno ||
            unsafe { sqlite3_bitvec_test(p_done_1, pgno) } != 0 {
        return 0;
    }
    if is_main_jrnl_1 != 0 {
        rc = read32bits(jfd, *p_offset_1 - 4 as i64, &mut cksum);
        if rc != 0 { return rc; }
        if (is_savepnt_1 == 0) as i32 != 0 &&
                pager_cksum(unsafe { &*p_pager_1 },
                        a_data as *mut u8 as *const u8) != cksum {
            return 101;
        }
    }
    if !(p_done_1).is_null() &&
            { rc = unsafe { sqlite3_bitvec_set(p_done_1, pgno) }; rc } != 0 {
        return rc;
    }
    if pgno == 1 as u32 &&
            unsafe { (*p_pager_1).n_reserve } as i32 !=
                unsafe { *(a_data as *mut u8).offset(20 as isize) } as i32 {
        unsafe {
            (*p_pager_1).n_reserve =
                unsafe { *(a_data as *mut u8).offset(20 as isize) } as i16
        };
    }
    if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
        p_pg = core::ptr::null_mut();
    } else {
        p_pg =
            unsafe { sqlite3_pager_lookup(unsafe { &*p_pager_1 }, pgno) } as
                *mut PgHdr;
    }
    { let _ = 0; };
    { let _ = 0; };
    if is_main_jrnl_1 != 0 {
        is_synced =
            (unsafe { (*p_pager_1).no_sync } != 0 ||
                    *p_offset_1 <= unsafe { (*p_pager_1).journal_hdr }) as i32;
    } else {
        is_synced =
            (p_pg == core::ptr::null_mut() ||
                    0 == unsafe { (*p_pg).flags } as i32 & 8) as i32;
    }
    if unsafe { (*unsafe { (*p_pager_1).fd }).p_methods } != core::ptr::null()
                &&
                (unsafe { (*p_pager_1).e_state } as i32 >= 4 ||
                    unsafe { (*p_pager_1).e_state } as i32 == 0) &&
            is_synced != 0 {
        let ofst: i64 =
            (pgno - 1 as Pgno) as i64 *
                unsafe { (*p_pager_1).page_size } as i64;
        { let _ = 0; };

        /// Write the data read from the journal back into the database file.
        ///* This is usually safe even for an encrypted database - as the data
        ///* was encrypted before it was written to the journal file. The exception
        ///* is if the data was just read from an in-memory sub-journal. In that
        ///* case it must be encrypted here before it is copied into the database
        ///* file.
        (rc =
            unsafe {
                sqlite3_os_write(unsafe { (*p_pager_1).fd },
                    a_data as *mut u8 as *const (),
                    unsafe { (*p_pager_1).page_size } as i32, ofst)
            });
        if pgno > unsafe { (*p_pager_1).db_file_size } {
            unsafe { (*p_pager_1).db_file_size = pgno };
        }
        if !(unsafe { (*p_pager_1).p_backup }).is_null() {
            unsafe {
                sqlite3_backup_update(unsafe { (*p_pager_1).p_backup }, pgno,
                    a_data as *mut u8 as *const u8)
            };
        }
    } else if (is_main_jrnl_1 == 0) as i32 != 0 &&
            p_pg == core::ptr::null_mut() {

        /// If this is a rollback of a savepoint and data was not written to
        ///* the database and the page is not in-memory, there is a potential
        ///* problem. When the page is next fetched by the b-tree layer, it
        ///* will be read from the database file, which may or may not be
        ///* current.
        ///*
        ///* There are a couple of different ways this can happen. All are quite
        ///* obscure. When running in synchronous mode, this can only happen
        ///* if the page is on the free-list at the start of the transaction, then
        ///* populated, then moved using sqlite3PagerMovepage().
        ///*
        ///* The solution is to add an in-memory page to the cache containing
        ///* the data just read from the sub-journal. Mark the page as dirty
        ///* and if the pager requires a journal-sync, then mark the page as
        ///* requiring a journal-sync before it is written.
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_pager_1).do_not_spill |= 2 as u8 };
        rc =
            unsafe {
                sqlite3_pager_get(p_pager_1, pgno,
                    &raw mut p_pg as *mut *mut DbPage, 1)
            };
        { let _ = 0; };
        unsafe { (*p_pager_1).do_not_spill &= !2 as u8 };
        if rc != 0 { return rc; }
        unsafe { sqlite3_pcache_make_dirty(p_pg) };
    }
    if !(p_pg).is_null() {
        /// No page should ever be explicitly rolled back that is in use, except
        ///* for page 1 which is held in use in order to keep the lock on the
        ///* database active. However such a page may be rolled back as a result
        ///* of an internal error resulting in an automatic call to
        ///* sqlite3PagerRollback().
        let mut p_data: *mut () = core::ptr::null_mut();
        p_data = unsafe { (*p_pg).p_data };
        unsafe {
            memcpy(p_data, a_data as *mut u8 as *const (),
                unsafe { (*p_pager_1).page_size } as u64)
        };
        unsafe { (unsafe { (*p_pager_1).x_reiniter.unwrap() })(p_pg) };
        if pgno == 1 as u32 {
            unsafe {
                memcpy(unsafe { &raw mut (*p_pager_1).db_file_vers } as
                        *mut (),
                    unsafe { &raw mut *(p_data as *mut u8).offset(24 as isize) }
                        as *const (), core::mem::size_of::<[i8; 16]>() as u64)
            };
        }
        unsafe { sqlite3_pcache_release(p_pg) };
    }
    return rc;
}

///* Return the offset of the sector boundary at or immediately
///* following the value in pPager->journalOff, assuming a sector
///* size of pPager->sectorSize bytes.
///*
///* i.e for a sector size of 512:
///*
///*   Pager.journalOff          Return value
///*   ---------------------------------------
///*   0                         0
///*   512                       512
///*   100                       512
///*   2000                      2048
///*
extern "C" fn journal_hdr_offset(p_pager_1: &Pager) -> i64 {
    let mut offset: i64 = 0 as i64;
    let c: i64 = (*p_pager_1).journal_off;
    if c != 0 {
        offset =
            ((c - 1 as i64) / (*p_pager_1).sector_size as i64 + 1 as i64) *
                (*p_pager_1).sector_size as i64;
    }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    return offset;
}

///* Journal files begin with the following magic string.  The data
///* was obtained from /dev/random.  It is used only as a sanity check.
///*
///* Since version 2.8.0, the journal format contains additional sanity
///* checking information.  If the power fails while the journal is being
///* written, semi-random garbage data might appear in the journal
///* file after power is restored.  If an attempt is then made
///* to roll the journal back, the database could be corrupted.  The additional
///* sanity checking data is an attempt to discover the garbage in the
///* journal and ignore it.
///*
///* The sanity checking information for the new journal format consists
///* of a 32-bit checksum on each page of data.  The checksum covers both
///* the page number and the pPager->pageSize bytes of data for the page.
///* This cksum is initialized to a 32-bit random value that appears in the
///* journal file right after the header.  The random initializer is important,
///* because garbage data that appears at the end of a journal is likely
///* data that was once in other files that have now been deleted.  If the
///* garbage data came from an obsolete journal file, the checksums might
///* be correct.  But by initializing the checksum to random value which
///* is different for every journal, we minimize that risk.
static a_journal_magic: [u8; 8] =
    [217 as u8, 213 as u8, 5 as u8, 249 as u8, 32 as u8, 161 as u8, 99 as u8,
            215 as u8];

///* The journal file must be open when this is called. A journal header file
///* (JOURNAL_HDR_SZ bytes) is read from the current location in the journal
///* file. The current location in the journal file is given by
///* pPager->journalOff. See comments above function writeJournalHdr() for
///* a description of the journal header format.
///*
///* If the header is read successfully, *pNRec is set to the number of
///* page records following this header and *pDbSize is set to the size of the
///* database before the transaction began, in pages. Also, pPager->cksumInit
///* is set to the value read from the journal header. SQLITE_OK is returned
///* in this case.
///*
///* If the journal header file appears to be corrupted, SQLITE_DONE is
///* returned and *pNRec and *PDbSize are undefined.  If JOURNAL_HDR_SZ bytes
///* cannot be read from the journal file an error code is returned.
#[allow(unused_doc_comments)]
extern "C" fn read_journal_hdr(p_pager_1: *mut Pager, is_hot_1: i32,
    journal_size_1: i64, p_n_rec_1: *mut u32, p_db_size_1: *mut u32) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut a_magic: [u8; 8] = [0; 8];
    /// A buffer to hold the magic header
    let mut i_hdr_off: i64 = 0 as i64;

    /// Offset of journal header being read
    { let _ = 0; };

    /// Journal file must be open.
    /// Advance Pager.journalOff to the start of the next sector. If the
    ///* journal file is too small for there to be a header stored at this
    ///* point, return SQLITE_DONE.
    unsafe {
        (*p_pager_1).journal_off = journal_hdr_offset(unsafe { &*p_pager_1 })
    };
    if unsafe { (*p_pager_1).journal_off } +
                unsafe { (*p_pager_1).sector_size } as i64 > journal_size_1 {
        return 101;
    }
    i_hdr_off = unsafe { (*p_pager_1).journal_off };
    if is_hot_1 != 0 || i_hdr_off != unsafe { (*p_pager_1).journal_hdr } {
        rc =
            unsafe {
                sqlite3_os_read(unsafe { (*p_pager_1).jfd },
                    &raw mut a_magic[0 as usize] as *mut u8 as *mut (),
                    core::mem::size_of::<[u8; 8]>() as i32, i_hdr_off)
            };
        if rc != 0 { return rc; }
        if unsafe {
                    memcmp(&raw mut a_magic[0 as usize] as *mut u8 as *const (),
                        &raw const a_journal_magic[0 as usize] as *const u8 as
                            *const (), core::mem::size_of::<[u8; 8]>() as u64)
                } != 0 {
            return 101;
        }
    }
    if 0 !=
                    {
                        rc =
                            read32bits(unsafe { (*p_pager_1).jfd },
                                i_hdr_off + 8 as i64, unsafe { &mut *p_n_rec_1 });
                        rc
                    } ||
                0 !=
                    {
                        rc =
                            read32bits(unsafe { (*p_pager_1).jfd },
                                i_hdr_off + 12 as i64,
                                unsafe { &mut (*p_pager_1).cksum_init });
                        rc
                    } ||
            0 !=
                {
                    rc =
                        read32bits(unsafe { (*p_pager_1).jfd },
                            i_hdr_off + 16 as i64, unsafe { &mut *p_db_size_1 });
                    rc
                } {
        return rc;
    }
    if unsafe { (*p_pager_1).journal_off } == 0 as i64 {
        let mut i_page_size: u32 = 0 as u32;
        /// Page-size field of journal header
        let mut i_sector_size: u32 = 0 as u32;
        if 0 !=
                    {
                        rc =
                            read32bits(unsafe { (*p_pager_1).jfd },
                                i_hdr_off + 20 as i64, &mut i_sector_size);
                        rc
                    } ||
                0 !=
                    {
                        rc =
                            read32bits(unsafe { (*p_pager_1).jfd },
                                i_hdr_off + 24 as i64, &mut i_page_size);
                        rc
                    } {
            return rc;
        }
        if i_page_size == 0 as u32 {
            i_page_size = unsafe { (*p_pager_1).page_size } as u32;
        }
        if i_page_size < 512 as u32 || i_sector_size < 32 as u32 ||
                            i_page_size > 65536 as u32 || i_sector_size > 65536 as u32
                    || i_page_size - 1 as u32 & i_page_size != 0 as u32 ||
                i_sector_size - 1 as u32 & i_sector_size != 0 as u32 {

            /// If the either the page-size or sector-size in the journal-header is
            ///* invalid, then the process that wrote the journal-header must have
            ///* crashed before the header was synced. In this case stop reading
            ///* the journal file here.
            return 101;
        }

        /// Update the page-size to match the value read from the journal.
        ///* Use a testcase() macro to make sure that malloc failure within
        ///* PagerSetPagesize() is tested.
        (rc =
            unsafe {
                sqlite3_pager_set_pagesize(p_pager_1, &mut i_page_size, -1)
            });

        /// Update the assumed sector-size to match the value used by
        ///* the process that created this journal. If this journal was
        ///* created by a process other than this one, then this routine
        ///* is being called from within pager_playback(). The local value
        ///* of Pager.sectorSize is restored at the end of that routine.
        unsafe { (*p_pager_1).sector_size = i_sector_size };
    }
    unsafe {
        (*p_pager_1).journal_off += unsafe { (*p_pager_1).sector_size } as i64
    };
    return rc;
}

///* Playback savepoint pSavepoint. Or, if pSavepoint==NULL, then playback
///* the entire super-journal file. The case pSavepoint==NULL occurs when
///* a ROLLBACK TO command is invoked on a SAVEPOINT that is a transaction
///* savepoint.
///*
///* When pSavepoint is not NULL (meaning a non-transaction savepoint is
///* being rolled back), then the rollback consists of up to three stages,
///* performed in the order specified:
///*
///*   * Pages are played back from the main journal starting at byte
///*     offset PagerSavepoint.iOffset and continuing to
///*     PagerSavepoint.iHdrOffset, or to the end of the main journal
///*     file if PagerSavepoint.iHdrOffset is zero.
///*
///*   * If PagerSavepoint.iHdrOffset is not zero, then pages are played
///*     back starting from the journal header immediately following
///*     PagerSavepoint.iHdrOffset to the end of the main journal file.
///*
///*   * Pages are then played back from the sub-journal file, starting
///*     with the PagerSavepoint.iSubRec and continuing to the end of
///*     the journal file.
///*
///* Throughout the rollback process, each time a page is rolled back, the
///* corresponding bit is set in a bitvec structure (variable pDone in the
///* implementation below). This is used to ensure that a page is only
///* rolled back the first time it is encountered in either journal.
///*
///* If pSavepoint is NULL, then pages are only played back from the main
///* journal file. There is no need for a bitvec in this case.
///*
///* In either case, before playback commences the Pager.dbSize variable
///* is reset to the value that it held at the start of the savepoint
///* (or transaction). No page with a page-number greater than this value
///* is played back. If one is encountered it is simply skipped.
#[allow(unused_doc_comments)]
extern "C" fn pager_playback_savepoint(p_pager_1: *mut Pager,
    p_savepoint_1: *mut PagerSavepoint) -> i32 {
    let mut sz_j: i64 = 0 as i64;
    /// Effective size of the main journal
    let mut i_hdr_off: i64 = 0 as i64;
    /// End of first segment of main-journal records
    let mut rc: i32 = 0;
    /// Return code
    let mut p_done: *mut Bitvec = core::ptr::null_mut();

    /// Bitvec to ensure pages played back only once
    { let _ = 0; };
    { let _ = 0; };
    if !(p_savepoint_1).is_null() {
        p_done =
            unsafe {
                sqlite3_bitvec_create(unsafe { (*p_savepoint_1).n_orig })
            };
        if (p_done).is_null() as i32 != 0 { return 7; }
    }

    /// Set the database size back to the value it was before the savepoint
    ///* being reverted was opened.
    unsafe {
        (*p_pager_1).db_size =
            if !(p_savepoint_1).is_null() {
                unsafe { (*p_savepoint_1).n_orig }
            } else { unsafe { (*p_pager_1).db_orig_size } }
    };
    unsafe {
        (*p_pager_1).change_count_done = unsafe { (*p_pager_1).temp_file }
    };
    if (p_savepoint_1).is_null() as i32 != 0 &&
            unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
        return pager_rollback_wal(p_pager_1);
    }

    /// Use pPager->journalOff as the effective size of the main rollback
    ///* journal.  The actual file might be larger than this in
    ///* PAGER_JOURNALMODE_TRUNCATE or PAGER_JOURNALMODE_PERSIST.  But anything
    ///* past pPager->journalOff is off-limits to us.
    (sz_j = unsafe { (*p_pager_1).journal_off });
    { let _ = 0; };
    if !(p_savepoint_1).is_null() &&
            !(unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut()) as i32
                != 0 {
        i_hdr_off =
            if unsafe { (*p_savepoint_1).i_hdr_offset } != 0 {
                unsafe { (*p_savepoint_1).i_hdr_offset }
            } else { sz_j };
        unsafe {
            (*p_pager_1).journal_off = unsafe { (*p_savepoint_1).i_offset }
        };
        while rc == 0 && unsafe { (*p_pager_1).journal_off } < i_hdr_off {
            rc =
                pager_playback_one_page(p_pager_1,
                    unsafe { &mut (*p_pager_1).journal_off }, p_done, 1, 1);
        }
        { let _ = 0; };
    } else { unsafe { (*p_pager_1).journal_off = 0 as i64 }; }
    while rc == 0 && unsafe { (*p_pager_1).journal_off } < sz_j {
        let mut ii: u32 = 0 as u32;
        /// Loop counter
        let mut n_j_rec: u32 = 0 as u32;
        /// Number of Journal Records
        let mut dummy: u32 = 0 as u32;
        rc = read_journal_hdr(p_pager_1, 0, sz_j, &mut n_j_rec, &mut dummy);
        { let _ = 0; };
        if n_j_rec == 0 as u32 &&
                unsafe { (*p_pager_1).journal_hdr } +
                        unsafe { (*p_pager_1).sector_size } as i64 ==
                    unsafe { (*p_pager_1).journal_off } {
            n_j_rec =
                ((sz_j - unsafe { (*p_pager_1).journal_off }) /
                        (unsafe { (*p_pager_1).page_size } + 8 as i64)) as u32;
        }
        {
            ii = 0 as u32;
            '__b6: loop {
                if !(rc == 0 && ii < n_j_rec &&
                                unsafe { (*p_pager_1).journal_off } < sz_j) {
                    break '__b6;
                }
                '__c6: loop {
                    rc =
                        pager_playback_one_page(p_pager_1,
                            unsafe { &mut (*p_pager_1).journal_off }, p_done, 1, 1);
                    break '__c6;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        { let _ = 0; };
    }
    { let _ = 0; };
    if !(p_savepoint_1).is_null() {
        let mut ii: u32 = 0 as u32;
        /// Loop counter
        let mut offset: i64 =
            unsafe { (*p_savepoint_1).i_sub_rec } as i64 *
                (4 as i64 + unsafe { (*p_pager_1).page_size });
        if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
            rc =
                unsafe {
                    sqlite3_wal_savepoint_undo(unsafe { (*p_pager_1).p_wal },
                        unsafe { &raw mut (*p_savepoint_1).a_wal_data[0 as usize] }
                            as *mut u32)
                };
        }
        {
            ii = unsafe { (*p_savepoint_1).i_sub_rec };
            '__b7: loop {
                if !(rc == 0 && ii < unsafe { (*p_pager_1).n_sub_rec }) {
                    break '__b7;
                }
                '__c7: loop {
                    { let _ = 0; };
                    rc =
                        pager_playback_one_page(p_pager_1, &mut offset, p_done, 0,
                            1);
                    break '__c7;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        { let _ = 0; };
    }
    unsafe { sqlite3_bitvec_destroy(p_done) };
    if rc == 0 { unsafe { (*p_pager_1).journal_off = sz_j }; }
    return rc;
}

///* This function is called to rollback or release (commit) a savepoint.
///* The savepoint to release or rollback need not be the most recently
///* created savepoint.
///*
///* Parameter op is always either SAVEPOINT_ROLLBACK or SAVEPOINT_RELEASE.
///* If it is SAVEPOINT_RELEASE, then release and destroy the savepoint with
///* index iSavepoint. If it is SAVEPOINT_ROLLBACK, then rollback all changes
///* that have occurred since the specified savepoint was created.
///*
///* The savepoint to rollback or release is identified by parameter
///* iSavepoint. A value of 0 means to operate on the outermost savepoint
///* (the first created). A value of (Pager.nSavepoint-1) means operate
///* on the most recently created savepoint. If iSavepoint is greater than
///* (Pager.nSavepoint-1), then this function is a no-op.
///*
///* If a negative value is passed to this function, then the current
///* transaction is rolled back. This is different to calling
///* sqlite3PagerRollback() because this function does not terminate
///* the transaction or unlock the database, it just restores the
///* contents of the database to its original state.
///*
///* In any case, all savepoints with an index greater than iSavepoint
///* are destroyed. If this is a release operation (op==SAVEPOINT_RELEASE),
///* then savepoint iSavepoint is also destroyed.
///*
///* This function may return SQLITE_NOMEM if a memory allocation fails,
///* or an IO error code if an IO error occurs while rolling back a
///* savepoint. If no errors occur, SQLITE_OK is returned.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_savepoint(p_pager_1: *mut Pager, op: i32,
    i_savepoint_1: i32) -> i32 {
    let mut rc: i32 = unsafe { (*p_pager_1).err_code };
    { let _ = 0; };
    { let _ = 0; };
    if rc == 0 && i_savepoint_1 < unsafe { (*p_pager_1).n_savepoint } {
        let mut ii: i32 = 0;
        /// Iterator variable
        let mut n_new: i32 = 0;

        /// Number of remaining savepoints after this op.
        /// Figure out how many savepoints will still be active after this
        ///* operation. Store this value in nNew. Then free resources associated
        ///* with any savepoints that are destroyed by this operation.
        (n_new = i_savepoint_1 + if op == 1 { 0 } else { 1 });
        {
            ii = n_new;
            '__b8: loop {
                if !(ii < unsafe { (*p_pager_1).n_savepoint }) {
                    break '__b8;
                }
                '__c8: loop {
                    unsafe {
                        sqlite3_bitvec_destroy(unsafe {
                                (*unsafe {
                                            (*p_pager_1).a_savepoint.offset(ii as isize)
                                        }).p_in_savepoint
                            })
                    };
                    break '__c8;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { (*p_pager_1).n_savepoint = n_new };
        if op == 1 {
            let p_rel: *const PagerSavepoint =
                unsafe {
                        &raw mut *unsafe {
                                    (*p_pager_1).a_savepoint.offset(n_new as isize)
                                }
                    } as *const PagerSavepoint;
            if unsafe { (*p_rel).b_truncate_on_release } != 0 &&
                    unsafe { (*unsafe { (*p_pager_1).sjfd }).p_methods } !=
                        core::ptr::null() {
                if unsafe {
                            sqlite3_journal_is_in_memory(unsafe { (*p_pager_1).sjfd })
                        } != 0 {
                    let sz: i64 =
                        (unsafe { (*p_pager_1).page_size } + 4 as i64) *
                            unsafe { (*p_rel).i_sub_rec } as i64;
                    rc =
                        unsafe {
                            sqlite3_os_truncate(unsafe { (*p_pager_1).sjfd }, sz)
                        };
                    { let _ = 0; };
                }
                unsafe {
                    (*p_pager_1).n_sub_rec = unsafe { (*p_rel).i_sub_rec }
                };
            }
        } else if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() ||
                unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                    core::ptr::null() {
            let p_savepoint: *mut PagerSavepoint =
                if n_new == 0 {
                    core::ptr::null_mut()
                } else {
                    unsafe {
                        &mut *unsafe {
                                    (*p_pager_1).a_savepoint.offset((n_new - 1) as isize)
                                }
                    }
                };
            rc = pager_playback_savepoint(p_pager_1, p_savepoint);
            { let _ = 0; };
        }
    }
    return rc;
}

///* Free all structures in the Pager.aSavepoint[] array and set both
///* Pager.aSavepoint and Pager.nSavepoint to zero. Close the sub-journal
///* if it is open and the pager is not in exclusive mode.
extern "C" fn release_all_savepoints(p_pager_1: &mut Pager) -> () {
    let mut ii: i32 = 0;
    {
        ii = 0;
        '__b9: loop {
            if !(ii < (*p_pager_1).n_savepoint) { break '__b9; }
            '__c9: loop {
                unsafe {
                    sqlite3_bitvec_destroy(unsafe {
                            (*(*p_pager_1).a_savepoint.offset(ii as
                                            isize)).p_in_savepoint
                        })
                };
                break '__c9;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    if ((*p_pager_1).exclusive_mode == 0) as i32 != 0 ||
            unsafe { sqlite3_journal_is_in_memory((*p_pager_1).sjfd) } != 0 {
        unsafe { sqlite3_os_close((*p_pager_1).sjfd) };
    }
    unsafe { sqlite3_free((*p_pager_1).a_savepoint as *mut ()) };
    (*p_pager_1).a_savepoint = core::ptr::null_mut();
    (*p_pager_1).n_savepoint = 0;
    (*p_pager_1).n_sub_rec = 0 as u32;
}

///* The journal file must be open when this function is called.
///*
///* This function is a no-op if the journal file has not been written to
///* within the current transaction (i.e. if Pager.journalOff==0).
///*
///* If doTruncate is non-zero or the Pager.journalSizeLimit variable is
///* set to 0, then truncate the journal file to zero bytes in size. Otherwise,
///* zero the 28-byte header at the start of the journal file. In either case,
///* if the pager is not in no-sync mode, sync the journal file immediately
///* after writing or truncating it.
///*
///* If Pager.journalSizeLimit is set to a positive, non-zero value, and
///* following the truncation or zeroing described above the size of the
///* journal file in bytes is larger than this value, then truncate the
///* journal file to Pager.journalSizeLimit bytes. The journal file does
///* not need to be synced following this operation.
///*
///* If an IO error occurs, abandon processing and return the IO error code.
///* Otherwise, return SQLITE_OK.
#[allow(unused_doc_comments)]
extern "C" fn zero_journal_hdr(p_pager_1: &Pager, do_truncate_1: i32) -> i32 {
    let mut rc: i32 = 0;

    /// Return code
    { let _ = 0; };
    { let _ = 0; };
    if (*p_pager_1).journal_off != 0 {
        let i_limit: i64 = (*p_pager_1).journal_size_limit as i64;
        if do_truncate_1 != 0 || i_limit as i64 == 0 as i64 {
            rc = unsafe { sqlite3_os_truncate((*p_pager_1).jfd, 0 as i64) };
        } else {
            rc =
                unsafe {
                    sqlite3_os_write((*p_pager_1).jfd,
                        &raw const zero_hdr[0 as usize] as *const i8 as *const (),
                        core::mem::size_of::<[i8; 28]>() as i32, 0 as i64)
                };
        }
        if rc == 0 && ((*p_pager_1).no_sync == 0) as i32 != 0 {
            rc =
                unsafe {
                    sqlite3_os_sync((*p_pager_1).jfd,
                        16 | (*p_pager_1).sync_flags as i32)
                };
        }
        if rc == 0 && i_limit as i64 > 0 as i64 {
            let mut sz: i64 = 0 as i64;
            rc = unsafe { sqlite3_os_file_size((*p_pager_1).jfd, &mut sz) };
            if rc == 0 && sz > i_limit {
                rc =
                    unsafe { sqlite3_os_truncate((*p_pager_1).jfd, i_limit) };
            }
        }
    }
    return rc;
}

///* The write transaction open on pPager is being committed (bCommit==1)
///* or rolled back (bCommit==0).
///*
///* Return TRUE if and only if all dirty pages should be flushed to disk.
///*
///* Rules:
///*
///*   *  For non-TEMP databases, always sync to disk.  This is necessary
///*      for transactions to be durable.
///*
///*   *  Sync TEMP database only on a COMMIT (not a ROLLBACK) when the backing
///*      file has been created already (via a spill on pagerStress()) and
///*      when the number of dirty pages in memory exceeds 25% of the total
///*      cache size.
extern "C" fn pager_flush_on_commit(p_pager_1: &Pager, b_commit_1: i32)
    -> i32 {
    if (*p_pager_1).temp_file as i32 == 0 { return 1; }
    if (b_commit_1 == 0) as i32 != 0 { return 0; }
    if !(unsafe { (*(*p_pager_1).fd).p_methods } != core::ptr::null()) as i32
            != 0 {
        return 0;
    }
    return (unsafe { sqlite3_p_cache_percent_dirty((*p_pager_1).p_p_cache) }
                >= 25) as i32;
}

///* This function is used to change the actual size of the database
///* file in the file-system. This only happens when committing a transaction,
///* or rolling back a transaction (including rolling back a hot-journal).
///*
///* If the main database file is not open, or the pager is not in either
///* DBMOD or OPEN state, this function is a no-op. Otherwise, the size
///* of the file is changed to nPage pages (nPage*pPager->pageSize bytes).
///* If the file on disk is currently larger than nPage pages, then use the VFS
///* xTruncate() method to truncate it.
///*
///* Or, it might be the case that the file on disk is smaller than
///* nPage pages. Some operating system implementations can get confused if
///* you try to truncate a file to some size that is larger than it
///* currently is, so detect this case and write a single zero byte to
///* the end of the new file instead.
///*
///* If successful, return SQLITE_OK. If an IO error occurs while modifying
///* the database file, return the error code to the caller.
#[allow(unused_doc_comments)]
extern "C" fn pager_truncate(p_pager_1: &mut Pager, n_page_1: Pgno) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*(*p_pager_1).fd).p_methods } != core::ptr::null() &&
            ((*p_pager_1).e_state as i32 >= 4 ||
                (*p_pager_1).e_state as i32 == 0) {
        let mut current_size: i64 = 0 as i64;
        let mut new_size: i64 = 0 as i64;
        let sz_page: i32 = (*p_pager_1).page_size as i32;
        { let _ = 0; };

        /// TODO: Is it safe to use Pager.dbFileSize here?
        (rc =
            unsafe {
                sqlite3_os_file_size((*p_pager_1).fd, &mut current_size)
            });
        new_size = sz_page as i64 * n_page_1 as i64;
        if rc == 0 && current_size != new_size {
            if current_size > new_size {
                rc =
                    unsafe { sqlite3_os_truncate((*p_pager_1).fd, new_size) };
            } else if current_size + sz_page as i64 <= new_size {
                let p_tmp: *mut i8 = (*p_pager_1).p_tmp_space;
                unsafe { memset(p_tmp as *mut (), 0, sz_page as u64) };
                unsafe {
                    sqlite3_os_file_control_hint((*p_pager_1).fd, 5,
                        &raw mut new_size as *mut ())
                };
                rc =
                    unsafe {
                        sqlite3_os_write((*p_pager_1).fd, p_tmp as *const (),
                            sz_page, new_size - sz_page as i64)
                    };
            }
            if rc == 0 { (*p_pager_1).db_file_size = n_page_1; }
        }
    }
    return rc;
}

///* Unlock the database file to level eLock, which must be either NO_LOCK
///* or SHARED_LOCK. Regardless of whether or not the call to xUnlock()
///* succeeds, set the Pager.eLock variable to match the (attempted) new lock.
///*
///* Except, if Pager.eLock is set to UNKNOWN_LOCK when this function is
///* called, do not modify it. See the comment above the #define of
///* UNKNOWN_LOCK for an explanation of this.
#[allow(unused_doc_comments)]
extern "C" fn pager_unlock_db(p_pager_1: &mut Pager, e_lock_1: i32) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*(*p_pager_1).fd).p_methods } != core::ptr::null() {
        { let _ = 0; };
        rc =
            if (*p_pager_1).no_lock != 0 {
                0
            } else {
                unsafe { sqlite3_os_unlock((*p_pager_1).fd, e_lock_1) }
            };
        if (*p_pager_1).e_lock as i32 != 4 + 1 {
            (*p_pager_1).e_lock = e_lock_1 as u8;
        }
    }
    (*p_pager_1).change_count_done = (*p_pager_1).temp_file;

    /// ticket fb3b3024ea238d5c
    return rc;
}

///* This routine ends a transaction. A transaction is usually ended by
///* either a COMMIT or a ROLLBACK operation. This routine may be called
///* after rollback of a hot-journal, or if an error occurs while opening
///* the journal file or writing the very first journal-header of a
///* database transaction.
///*
///* This routine is never called in PAGER_ERROR state. If it is called
///* in PAGER_OPEN or PAGER_READER state and the lock held is less
///* exclusive than a RESERVED lock, it is a no-op.
///*
///* Otherwise, any active savepoints are released.
///*
///* If the journal file is open, then it is "finalized". Once a journal
///* file has been finalized it is not possible to use it to roll back a
///* transaction. Nor will it be considered to be a hot-journal by this
///* or any other database connection. Exactly how a journal is finalized
///* depends on whether or not the pager is running in exclusive mode and
///* the current journal-mode (Pager.journalMode value), as follows:
///*
///*   journalMode==MEMORY
///*     Journal file descriptor is simply closed. This destroys an
///*     in-memory journal.
///*
///*   journalMode==TRUNCATE
///*     Journal file is truncated to zero bytes in size.
///*
///*   journalMode==PERSIST
///*     The first 28 bytes of the journal file are zeroed. This invalidates
///*     the first journal header in the file, and hence the entire journal
///*     file. An invalid journal file cannot be rolled back.
///*
///*   journalMode==DELETE
///*     The journal file is closed and deleted using sqlite3OsDelete().
///*
///*     If the pager is running in exclusive mode, this method of finalizing
///*     the journal file is never used. Instead, if the journalMode is
///*     DELETE and the pager is in exclusive mode, the method described under
///*     journalMode==PERSIST is used instead.
///*
///* After the journal is finalized, the pager moves to PAGER_READER state.
///* If running in non-exclusive rollback mode, the lock on the file is
///* downgraded to a SHARED_LOCK.
///*
///* SQLITE_OK is returned if no error occurs. If an error occurs during
///* any of the IO operations to finalize the journal file or unlock the
///* database then the IO error code is returned to the user. If the
///* operation to finalize the journal file fails, then the code still
///* tries to unlock the database file if not in exclusive mode. If the
///* unlock operation fails as well, then the first error code related
///* to the first error encountered (the journal finalization one) is
///* returned.
#[allow(unused_doc_comments)]
extern "C" fn pager_end_transaction(p_pager_1: *mut Pager, has_super_1: i32,
    b_commit_1: i32) -> i32 {
    let mut rc: i32 = 0;
    /// Error code from journal finalization operation
    let mut rc2: i32 = 0;

    /// Error code from db file unlock operation
    /// Do nothing if the pager does not have an open write transaction
    ///* or at least a RESERVED lock. This function may be called when there
    ///* is no write-transaction active but a RESERVED or greater lock is
    ///* held under two circumstances:
    ///*
    ///*   1. After a successful hot-journal rollback, it is called with
    ///*      eState==PAGER_NONE and eLock==EXCLUSIVE_LOCK.
    ///*
    ///*   2. If a connection with locking_mode=exclusive holding an EXCLUSIVE
    ///*      lock switches back to locking_mode=normal and then executes a
    ///*      read-transaction, this function is called with eState==PAGER_READER
    ///*      and eLock==EXCLUSIVE_LOCK when the read-transaction is closed.
    { let _ = 0; };
    { let _ = 0; };
    if (unsafe { (*p_pager_1).e_state } as i32) < 2 &&
            (unsafe { (*p_pager_1).e_lock } as i32) < 2 {
        return 0;
    }
    release_all_savepoints(unsafe { &mut *p_pager_1 });
    { let _ = 0; };
    if unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
            core::ptr::null() {
        { let _ = 0; };
        if unsafe {
                    sqlite3_journal_is_in_memory(unsafe { (*p_pager_1).jfd })
                } != 0 {

            /// assert( pPager->journalMode==PAGER_JOURNALMODE_MEMORY );
            unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
        } else if unsafe { (*p_pager_1).journal_mode } as i32 == 3 {
            if unsafe { (*p_pager_1).journal_off } == 0 as i64 {
                rc = 0;
            } else {
                rc =
                    unsafe {
                        sqlite3_os_truncate(unsafe { (*p_pager_1).jfd }, 0 as i64)
                    };
                if rc == 0 && unsafe { (*p_pager_1).full_sync } != 0 {

                    /// Make sure the new file size is written into the inode right away.
                    ///* Otherwise the journal might resurrect following a power loss and
                    ///* cause the last transaction to roll back.  See
                    ///* https://bugzilla.mozilla.org/show_bug.cgi?id=1072773
                    (rc =
                        unsafe {
                            sqlite3_os_sync(unsafe { (*p_pager_1).jfd },
                                unsafe { (*p_pager_1).sync_flags } as i32)
                        });
                }
            }
            unsafe { (*p_pager_1).journal_off = 0 as i64 };
        } else if unsafe { (*p_pager_1).journal_mode } as i32 == 1 ||
                unsafe { (*p_pager_1).exclusive_mode } != 0 &&
                    (unsafe { (*p_pager_1).journal_mode } as i32) < 5 {
            rc =
                zero_journal_hdr(unsafe { &*p_pager_1 },
                    (has_super_1 != 0 || unsafe { (*p_pager_1).temp_file } != 0)
                        as i32);
            unsafe { (*p_pager_1).journal_off = 0 as i64 };
        } else {
            /// This branch may be executed with Pager.journalMode==MEMORY if
            ///* a hot-journal was just rolled back. In this case the journal
            ///* file should be closed and deleted. If this connection writes to
            ///* the database file, it will do so using an in-memory journal.
            let b_delete: i32 =
                (unsafe { (*p_pager_1).temp_file } == 0) as i32 as i32;
            { let _ = 0; };
            { let _ = 0; };
            unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
            if b_delete != 0 {
                rc =
                    unsafe {
                        sqlite3_os_delete(unsafe { (*p_pager_1).p_vfs },
                            unsafe { (*p_pager_1).z_journal } as *const i8,
                            unsafe { (*p_pager_1).extra_sync } as i32)
                    };
            }
        }
    }
    unsafe { sqlite3_bitvec_destroy(unsafe { (*p_pager_1).p_in_journal }) };
    unsafe { (*p_pager_1).p_in_journal = core::ptr::null_mut() };
    unsafe { (*p_pager_1).n_rec = 0 };
    if rc == 0 {
        if unsafe { (*p_pager_1).mem_db } != 0 ||
                pager_flush_on_commit(unsafe { &*p_pager_1 }, b_commit_1) != 0
            {
            unsafe {
                sqlite3_pcache_clean_all(unsafe { (*p_pager_1).p_p_cache })
            };
        } else {
            unsafe {
                sqlite3_pcache_clear_writable(unsafe {
                        (*p_pager_1).p_p_cache
                    })
            };
        }
        unsafe {
            sqlite3_pcache_truncate(unsafe { (*p_pager_1).p_p_cache },
                unsafe { (*p_pager_1).db_size })
        };
    }
    if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {

        /// Drop the WAL write-lock, if any. Also, if the connection was in
        ///* locking_mode=exclusive mode but is no longer, drop the EXCLUSIVE
        ///* lock held on the database file.
        (rc2 =
            unsafe {
                sqlite3_wal_end_write_transaction(unsafe {
                        (*p_pager_1).p_wal
                    })
            });
        { let _ = 0; };
    } else if rc == 0 && b_commit_1 != 0 &&
            unsafe { (*p_pager_1).db_file_size } >
                unsafe { (*p_pager_1).db_size } {

        /// This branch is taken when committing a transaction in rollback-journal
        ///* mode if the database file on disk is larger than the database image.
        ///* At this point the journal has been finalized and the transaction
        ///* successfully committed, but the EXCLUSIVE lock is still held on the
        ///* file. So it is safe to truncate the database file to its minimum
        ///* required size.
        { let _ = 0; };
        rc =
            unsafe {
                pager_truncate(unsafe { &mut *p_pager_1 },
                    unsafe { (*p_pager_1).db_size })
            };
    }
    if rc == 0 && b_commit_1 != 0 {
        rc =
            unsafe {
                sqlite3_os_file_control(unsafe { (*p_pager_1).fd }, 22,
                    core::ptr::null_mut())
            };
        if rc == 12 { rc = 0; }
    }
    if (unsafe { (*p_pager_1).exclusive_mode } == 0) as i32 != 0 &&
            (!(unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut()) as i32
                    != 0 ||
                unsafe {
                        sqlite3_wal_exclusive_mode(unsafe { (*p_pager_1).p_wal }, 0)
                    } != 0) {
        rc2 = pager_unlock_db(unsafe { &mut *p_pager_1 }, 1);
    }
    unsafe { (*p_pager_1).e_state = 1 as u8 };
    unsafe { (*p_pager_1).set_super = 0 as u8 };
    return if rc == 0 { rc2 } else { rc };
}

/// 
///* Check if zSuper is a valid super-journal name. There are two valid
///* formats:
///*
///*   + The 3rd and 4th last bytes of the filename are ".9", and the 
///*     following 2 bytes are hex digits. This is a file created in 8.3 
///*     filenames mode.
///*
///*   + The 3rd last byte of the filename is "9" and the filename
///*     contains the string "-mj" starting at the 12th last byte.
///*     All bytes following the "-mj" are hex digits.
///*
///* If the filename matches either of these patterns, return non-zero. 
///* Otherwise, return zero.
extern "C" fn pager_is_super_jrnl_name(z_super_1: *const i8) -> i32 {
    unsafe {
        let n_super: i32 = unsafe { sqlite3_strlen30(z_super_1) } as i32;
        let mut ii: i32 = 0;
        if (n_super as i32) < 12 { return 0; }
        if unsafe {
                    memcmp(unsafe {
                                &raw const *z_super_1.offset((n_super - 12) as isize)
                            } as *const (), c"-mj".as_ptr() as *mut i8 as *const (),
                        3 as u64)
                } != 0 {
            return 0;
        }
        if unsafe { *z_super_1.offset((n_super - 3) as isize) } as i32 !=
                '9' as i32 {
            return 0;
        }
        {
            ii = n_super - 9;
            '__b10: loop {
                if !(ii < n_super) { break '__b10; }
                '__c10: loop {
                    if unsafe {
                                        *(sqlite3_ctype_map.as_ptr() as
                                                    *const u8).add(unsafe { *z_super_1.offset(ii as isize) } as
                                                        u8 as usize)
                                    } as i32 & 8 == 0 {
                        return 0;
                    }
                    break '__c10;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        return 1;
    }
}

///* Free a buffer allocated by the readSuperJournal() function.
extern "C" fn free_super_journal(z_super_1: *mut i8) -> () {
    if !(z_super_1).is_null() {
        unsafe {
            sqlite3_free(unsafe { &raw mut *z_super_1.offset(-4 as isize) } as
                    *mut ())
        };
    }
}

///* Parameter pJrnl is a file-handle open on a journal file. This function
///* attempts to read a super-journal file name from the end of the journal 
///* file. If successful, it sets output parameter (*pzSuper) to point to a
///* buffer containing the super-journal name as a nul-terminated string.
///* The caller is responsible for freeing the buffer using freeSuperJournal().
///*
///* Refer to comments above writeSuperJournal() for the format used to store 
///* a super-journal file name at the end of a journal file.
///*
///* Parameter nSuper is passed the maximum allowable size of the super journal
///* name in bytes. If the super-journal name in the journal is longer than
///* nSuper bytes (including a nul-terminator), then this is handled as if no
///* super-journal name were present in the journal.
///*
///* If there is no super-journal name at the end of pJrnl, (*pzSuper) is
///* set to 0 and SQLITE_OK is returned. Or, if an error occurs while reading
///* the super-journal name, an SQLite error code is returned and (*pzSuper)
///* is set to 0.
#[allow(unused_doc_comments)]
extern "C" fn read_super_journal(p_jrnl_1: *mut Sqlite3File, n_super_1: u64,
    pz_super_1: &mut *mut i8) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut len: u32 = 0 as u32;
    /// Length in bytes of super-journal name
    let mut sz_j: i64 = 0 as i64;
    /// Total size in bytes of journal file pJrnl
    let mut cksum: u32 = 0 as u32;
    /// MJ checksum value read from journal
    let mut a_magic: [u8; 8] = [0; 8];
    /// A buffer to hold the magic header
    let mut z_out: *mut i8 = core::ptr::null_mut();
    *pz_super_1 = core::ptr::null_mut();
    if 0 != { rc = unsafe { sqlite3_os_file_size(p_jrnl_1, &mut sz_j) }; rc }
                                    || sz_j < 16 as i64 ||
                                0 !=
                                    {
                                        rc = read32bits(p_jrnl_1, sz_j - 16 as i64, &mut len);
                                        rc
                                    } || len as u64 >= n_super_1 ||
                        len as i64 > sz_j - 16 as i64 || len == 0 as u32 ||
                0 !=
                    {
                        rc = read32bits(p_jrnl_1, sz_j - 12 as i64, &mut cksum);
                        rc
                    } ||
            0 !=
                {
                    rc =
                        unsafe {
                            sqlite3_os_read(p_jrnl_1,
                                &raw mut a_magic[0 as usize] as *mut u8 as *mut (), 8,
                                sz_j - 8 as i64)
                        };
                    rc
                } {
        return rc;
    }
    z_out =
        unsafe { sqlite3_malloc_zero((4 as u32 + len + 2 as u32) as u64) } as
            *mut i8;
    if (z_out).is_null() as i32 != 0 {
        rc =
            if unsafe {
                        memcmp(&raw mut a_magic[0 as usize] as *mut u8 as *const (),
                            &raw const a_journal_magic[0 as usize] as *const u8 as
                                *const (), 8 as u64)
                    } != 0 {
                0
            } else { 7 };
    } else {
        z_out = unsafe { z_out.offset(4 as isize) };
        if 0 ==
                {
                    rc =
                        unsafe {
                            sqlite3_os_read(p_jrnl_1, z_out as *mut (), len as i32,
                                sz_j - 16 as i64 - len as i64)
                        };
                    rc
                } {
            let mut u: u32 = 0 as u32;
            {
                u = 0 as u32;
                '__b11: loop {
                    if !(u < len) { break '__b11; }
                    '__c11: loop {
                        cksum -= unsafe { *z_out.add(u as usize) } as u32;
                        break '__c11;
                    }
                    { let __p = &mut u; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if rc != 0 ||
                        (pager_is_super_jrnl_name(z_out as *const i8) == 0) as i32
                            != 0 || cksum != 0 ||
                unsafe {
                        memcmp(&raw mut a_magic[0 as usize] as *mut u8 as *const (),
                            &raw const a_journal_magic[0 as usize] as *const u8 as
                                *const (), 8 as u64)
                    } != 0 {

            /// If any validity checks fail, that means the super-journal filename
            ///* is corrupted, so rollback.  Return SQLITE_K and a NULL super-journal
            ///* name
            free_super_journal(z_out);
            z_out = core::ptr::null_mut();
        }
    }
    *pz_super_1 = z_out;
    return rc;
}

///* Sync the database file to disk. This is a no-op for in-memory databases
///* or pages with the Pager.noSync flag set.
///*
///* If successful, or if called on a pager for which it is a no-op, this
///* function returns SQLITE_OK. Otherwise, an IO error code is returned.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_sync(p_pager_1: &Pager, z_super_1: *const i8)
    -> i32 {
    let mut rc: i32 = 0;
    let p_arg: *mut () = z_super_1 as *mut ();
    rc = unsafe { sqlite3_os_file_control((*p_pager_1).fd, 21, p_arg) };
    if rc == 12 { rc = 0; }
    if rc == 0 && ((*p_pager_1).no_sync == 0) as i32 != 0 {
        { let _ = 0; };
        rc =
            unsafe {
                sqlite3_os_sync((*p_pager_1).fd,
                    (*p_pager_1).sync_flags as i32)
            };
    }
    return rc;
}

///* Parameter zSuper is the name of a super-journal file. A single journal
///* file that referred to the super-journal file has just been rolled back.
///* This routine checks if it is possible to delete the super-journal file,
///* and does so if it is.
///*
///* Argument zSuper may point to Pager.pTmpSpace. So that buffer is not
///* available for use within this function.
///*
///* When a super-journal file is created, it is populated with the names
///* of all of its child journals, one after another, formatted as utf-8
///* encoded text. The end of each child journal file is marked with a
///* nul-terminator byte (0x00). i.e. the entire contents of a super-journal
///* file for a transaction involving two databases might be:
///*
///*   "/home/bill/a.db-journal\x00/home/bill/b.db-journal\x00"
///*
///* A super-journal file may only be deleted once all of its child
///* journals have been rolled back.
///*
///* This function reads the contents of the super-journal file into
///* memory and loops through each of the child journal names. For
///* each child journal, it checks if:
///*
///*   * if the child journal exists, and if so
///*   * if the child journal contains a reference to super-journal
///*     file zSuper
///*
///* If a child journal can be found that matches both of the criteria
///* above, this function returns without doing anything. Otherwise, if
///* no such child journal can be found, file zSuper is deleted from
///* the file-system using sqlite3OsDelete().
///*
///* If an IO error within this function, an error code is returned. This
///* function allocates memory by calling sqlite3Malloc(). If an allocation
///* fails, SQLITE_NOMEM is returned. Otherwise, if no IO or malloc errors
///* occur, SQLITE_OK is returned.
///*
///* TODO: This function allocates a single block of memory to load
///* the entire contents of the super-journal file. This could be
///* a couple of kilobytes or so - potentially larger than the page
///* size.
#[allow(unused_doc_comments)]
extern "C" fn pager_delsuper(p_pager_1: &Pager, z_super_1: *const i8) -> i32 {
    let mut p_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
    let mut rc: i32 = 0;
    /// Return code
    let mut p_super: *mut Sqlite3File = core::ptr::null_mut();
    /// Malloc'd super-journal file descriptor
    let mut p_journal: *mut Sqlite3File = core::ptr::null_mut();
    /// Malloc'd child-journal file descriptor
    let mut z_super_journal: *mut i8 = core::ptr::null_mut();
    /// Contents of super-journal file
    let mut n_super_journal: i64 = 0 as i64;
    /// Size of super-journal file
    let mut z_journal: *const i8 = core::ptr::null();
    /// Pointer to one journal within MJ file
    let mut z_free: *mut i8 = core::ptr::null_mut();
    /// Free this buffer
    let mut b_seen: i32 = 0;
    /// If super-journal contains pPager->zJournal
    /// Check if this looks like a real super-journal name. If it does not,
    ///* return SQLITE_OK without attempting to delete it. This is to limit
    ///* the degree to which a crafted journal file can be used to cause
    ///* SQLite to delete arbitrary files.
    ///*
    ///* This test never fails, becaue the super journal name is checked
    ///* by readSuperJournal().
    /// Allocate space for both the pJournal and pSuper file descriptors.
    ///* If successful, open the super-journal file for reading.
    let mut flags: i32 = 0;
    /// Load the entire super-journal file into space obtained from
    ///* sqlite3_malloc() and pointed to by zSuperJournal.   Also obtain
    ///* sufficient space (in zSuperPtr) to hold the names of super-journal
    ///* files extracted from regular rollback-journals.
    let mut exists: i32 = 0;
    let mut z_super_ptr: *mut i8 = core::ptr::null_mut();
    /// One of the journals pointed to by the super-journal exists.
    ///* Open it and check if it points at the super-journal. If
    ///* so, return without deleting the super-journal file.
    ///* NB:  zJournal is really a MAIN_JOURNAL.  But call it a
    ///* SUPER_JOURNAL here so that the VFS will not send the zJournal
    ///* name into sqlite3_database_file_object().
    let mut c: i32 = 0;
    let mut flags__1: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s13:
            {
            match __state {
                0 => { p_vfs = (*p_pager_1).p_vfs; __state = 3; }
                2 => {
                    unsafe { sqlite3_free(z_free as *mut ()) };
                    __state = 67;
                }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { z_super_journal = core::ptr::null_mut(); __state = 7; }
                7 => { __state = 8; }
                8 => { __state = 9; }
                9 => { z_free = core::ptr::null_mut(); __state = 10; }
                10 => { b_seen = 0; __state = 11; }
                11 => {
                    if pager_is_super_jrnl_name(z_super_1) == 0 {
                        __state = 13;
                    } else { __state = 12; }
                }
                12 => {
                    p_super =
                        unsafe {
                                sqlite3_malloc_zero((2 as i64 *
                                            unsafe { (*p_vfs).sz_os_file } as i64) as u64)
                            } as *mut Sqlite3File;
                    __state = 14;
                }
                13 => { return 0; }
                14 => {
                    if (p_super).is_null() as i32 != 0 {
                        __state = 16;
                    } else { __state = 17; }
                }
                15 => { if rc != 0 { __state = 22; } else { __state = 21; } }
                16 => { rc = 7; __state = 18; }
                17 => { flags = (1 | 16384) as i32; __state = 19; }
                18 => { p_journal = core::ptr::null_mut(); __state = 15; }
                19 => {
                    rc =
                        unsafe {
                            sqlite3_os_open(p_vfs, z_super_1, p_super, flags,
                                core::ptr::null_mut())
                        };
                    __state = 20;
                }
                20 => {
                    p_journal =
                        unsafe {
                                (p_super as
                                        *mut u8).offset(unsafe { (*p_vfs).sz_os_file } as isize)
                            } as *mut Sqlite3File;
                    __state = 15;
                }
                21 => {
                    rc =
                        unsafe {
                            sqlite3_os_file_size(p_super, &mut n_super_journal)
                        };
                    __state = 23;
                }
                22 => { __state = 2; }
                23 => { if rc != 0 { __state = 25; } else { __state = 24; } }
                24 => { { let _ = 0; }; __state = 26; }
                25 => { __state = 2; }
                26 => {
                    z_free =
                        unsafe {
                                sqlite3Malloc((4 as i64 + n_super_journal + 2 as i64) as
                                        u64)
                            } as *mut i8;
                    __state = 27;
                }
                27 => {
                    if (z_free).is_null() as i32 != 0 {
                        __state = 29;
                    } else { __state = 30; }
                }
                28 => {
                    unsafe {
                        *z_free.offset(0 as isize) =
                            {
                                unsafe {
                                    *z_free.offset(1 as isize) =
                                        {
                                            unsafe {
                                                *z_free.offset(2 as isize) =
                                                    {
                                                        unsafe { *z_free.offset(3 as isize) = 0 as i8 };
                                                        unsafe { *z_free.offset(3 as isize) }
                                                    }
                                            };
                                            unsafe { *z_free.offset(2 as isize) }
                                        }
                                };
                                unsafe { *z_free.offset(1 as isize) }
                            }
                    };
                    __state = 32;
                }
                29 => { rc = 7; __state = 31; }
                30 => { { let _ = 0; }; __state = 28; }
                31 => { __state = 2; }
                32 => {
                    z_super_journal = unsafe { z_free.offset(4 as isize) };
                    __state = 33;
                }
                33 => {
                    rc =
                        unsafe {
                            sqlite3_os_read(p_super, z_super_journal as *mut (),
                                n_super_journal as i32, 0 as i64)
                        };
                    __state = 34;
                }
                34 => { if rc != 0 { __state = 36; } else { __state = 35; } }
                35 => {
                    unsafe {
                        *z_super_journal.offset(n_super_journal as isize) = 0 as i8
                    };
                    __state = 37;
                }
                36 => { __state = 2; }
                37 => {
                    unsafe {
                        *z_super_journal.offset((n_super_journal + 1 as i64) as
                                        isize) = 0 as i8
                    };
                    __state = 38;
                }
                38 => { z_journal = z_super_journal; __state = 39; }
                39 => {
                    if (unsafe { z_journal.offset_from(z_super_journal) } as i64
                                    as i64) < n_super_journal {
                        __state = 41;
                    } else { __state = 40; }
                }
                40 => { unsafe { sqlite3_os_close(p_super) }; __state = 64; }
                41 => {
                    if unsafe {
                                strcmp(z_journal as *const i8,
                                    (*p_pager_1).z_journal as *const i8)
                            } == 0 {
                        __state = 43;
                    } else { __state = 44; }
                }
                42 => {
                    {
                        let __n =
                            unsafe { sqlite3_strlen30(z_journal as *const i8) } + 1;
                        let __p = &mut z_journal;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 39;
                }
                43 => { b_seen = 1; __state = 42; }
                44 => { __state = 45; }
                45 => {
                    rc =
                        unsafe {
                            sqlite3_os_access(p_vfs, z_journal as *const i8, 0,
                                &mut exists)
                        };
                    __state = 46;
                }
                46 => { if rc != 0 { __state = 48; } else { __state = 47; } }
                47 => {
                    if exists != 0 { __state = 49; } else { __state = 42; }
                }
                48 => { __state = 2; }
                49 => { z_super_ptr = core::ptr::null_mut(); __state = 50; }
                50 => { __state = 51; }
                51 => { flags__1 = 1 | 16384; __state = 52; }
                52 => {
                    rc =
                        unsafe {
                            sqlite3_os_open(p_vfs, z_journal as *const i8, p_journal,
                                flags__1, core::ptr::null_mut())
                        };
                    __state = 53;
                }
                53 => { if rc != 0 { __state = 55; } else { __state = 54; } }
                54 => {
                    rc =
                        read_super_journal(p_journal,
                            1 as u64 + unsafe { (*p_vfs).mx_pathname } as u64,
                            &mut z_super_ptr);
                    __state = 56;
                }
                55 => { __state = 2; }
                56 => {
                    unsafe { sqlite3_os_close(p_journal) };
                    __state = 57;
                }
                57 => { if rc != 0 { __state = 59; } else { __state = 58; } }
                58 => {
                    c =
                        (z_super_ptr != core::ptr::null_mut() &&
                                unsafe { strcmp(z_super_ptr as *const i8, z_super_1) } == 0)
                            as i32;
                    __state = 61;
                }
                59 => { { let _ = 0; }; __state = 60; }
                60 => { __state = 2; }
                61 => { free_super_journal(z_super_ptr); __state = 62; }
                62 => { if c != 0 { __state = 63; } else { __state = 42; } }
                63 => { __state = 2; }
                64 => {
                    if b_seen != 0 { __state = 66; } else { __state = 65; }
                }
                65 => { __state = 2; }
                66 => {
                    rc = unsafe { sqlite3_os_delete(p_vfs, z_super_1, 0) };
                    __state = 65;
                }
                67 => {
                    if !(p_super).is_null() {
                        __state = 69;
                    } else { __state = 68; }
                }
                68 => { return rc; }
                69 => { unsafe { sqlite3_os_close(p_super) }; __state = 70; }
                70 => { { let _ = 0; }; __state = 71; }
                71 => {
                    unsafe { sqlite3_free(p_super as *mut ()) };
                    __state = 68;
                }
                _ => {}
            }
        }
    }

    /// Return code
    /// Malloc'd super-journal file descriptor
    /// Malloc'd child-journal file descriptor
    /// Contents of super-journal file
    /// Size of super-journal file
    /// Pointer to one journal within MJ file
    /// Free this buffer
    /// If super-journal contains pPager->zJournal
    /// Check if this looks like a real super-journal name. If it does not,
    ///* return SQLITE_OK without attempting to delete it. This is to limit
    ///* the degree to which a crafted journal file can be used to cause
    ///* SQLite to delete arbitrary files.
    ///*
    ///* This test never fails, becaue the super journal name is checked
    ///* by readSuperJournal().
    /// Allocate space for both the pJournal and pSuper file descriptors.
    ///* If successful, open the super-journal file for reading.
    /// Load the entire super-journal file into space obtained from
    ///* sqlite3_malloc() and pointed to by zSuperJournal.   Also obtain
    ///* sufficient space (in zSuperPtr) to hold the names of super-journal
    ///* files extracted from regular rollback-journals.
    /// One of the journals pointed to by the super-journal exists.
    ///* Open it and check if it points at the super-journal. If
    ///* so, return without deleting the super-journal file.
    ///* NB:  zJournal is really a MAIN_JOURNAL.  But call it a
    ///* SUPER_JOURNAL here so that the VFS will not send the zJournal
    ///* name into sqlite3_database_file_object().
    /// We have a match. Do not delete the super-journal file.
    /// Only delete the super-journal if bSeen is true - indicating that
    ///* the super-journal contained a pointer to this database's journal 
    ///* file.
    unreachable!();
}

/// Forward reference
#[allow(unused_doc_comments)]
extern "C" fn pager_playback(p_pager_1: *mut Pager, is_hot_1: i32) -> i32 {
    let mut p_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
    let mut sz_j: i64 = 0 as i64;
    /// Size of the journal file in bytes
    let mut n_rec: u32 = 0 as u32;
    /// Number of Records in the journal
    let mut u: u32 = 0 as u32;
    /// Unsigned loop counter
    let mut mx_pg: Pgno = 0 as Pgno;
    /// Size of the original file in pages
    let mut rc: i32 = 0;
    /// Result code of a subroutine
    let mut res: i32 = 0;
    /// Value returned by sqlite3OsAccess()
    let mut z_super: *mut i8 = core::ptr::null_mut();
    /// Name of super-journal file if any
    let mut need_pager_reset: i32 = 0;
    /// True to reset page prior to first page rollback
    let mut n_playback: i32 = 0;
    /// Total number of pages restored from journal
    let mut saved_page_size: u32 = 0 as u32;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s15:
            {
            match __state {
                0 => { p_vfs = unsafe { (*p_pager_1).p_vfs }; __state = 3; }
                2 => { if rc == 0 { __state = 61; } else { __state = 60; } }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => { __state = 6; }
                6 => { mx_pg = 0 as Pgno; __state = 7; }
                7 => { __state = 8; }
                8 => { res = 1; __state = 9; }
                9 => { z_super = core::ptr::null_mut(); __state = 10; }
                10 => { __state = 11; }
                11 => { n_playback = 0; __state = 12; }
                12 => {
                    saved_page_size = unsafe { (*p_pager_1).page_size } as u32;
                    __state = 13;
                }
                13 => { { let _ = 0; }; __state = 14; }
                14 => {
                    rc =
                        unsafe {
                            sqlite3_os_file_size(unsafe { (*p_pager_1).jfd }, &mut sz_j)
                        };
                    __state = 15;
                }
                15 => { if rc != 0 { __state = 17; } else { __state = 16; } }
                16 => {
                    rc =
                        read_super_journal(unsafe { (*p_pager_1).jfd },
                            (1 as i64 +
                                    unsafe { (*unsafe { (*p_pager_1).p_vfs }).mx_pathname } as
                                        i64) as u64, &mut z_super);
                    __state = 18;
                }
                17 => { __state = 2; }
                18 => {
                    if rc == 0 && !(z_super).is_null() {
                        __state = 20;
                    } else { __state = 19; }
                }
                19 => {
                    if rc != 0 || (res == 0) as i32 != 0 {
                        __state = 22;
                    } else { __state = 21; }
                }
                20 => {
                    rc =
                        unsafe {
                            sqlite3_os_access(p_vfs, z_super as *const i8, 0, &mut res)
                        };
                    __state = 19;
                }
                21 => {
                    unsafe { (*p_pager_1).journal_off = 0 as i64 };
                    __state = 23;
                }
                22 => { __state = 2; }
                23 => { need_pager_reset = is_hot_1; __state = 24; }
                24 => { if 1 != 0 { __state = 26; } else { __state = 25; } }
                25 => { { let _ = 0; }; __state = 59; }
                26 => {
                    rc =
                        read_journal_hdr(p_pager_1, is_hot_1, sz_j, &mut n_rec,
                            &mut mx_pg);
                    __state = 27;
                }
                27 => { if rc != 0 { __state = 29; } else { __state = 28; } }
                28 => {
                    if n_rec == 4294967295u32 {
                        __state = 33;
                    } else { __state = 32; }
                }
                29 => {
                    if rc == 101 { __state = 31; } else { __state = 30; }
                }
                30 => { __state = 2; }
                31 => { rc = 0; __state = 30; }
                32 => {
                    if n_rec == 0 as u32 && (is_hot_1 == 0) as i32 != 0 &&
                            unsafe { (*p_pager_1).journal_hdr } +
                                    unsafe { (*p_pager_1).sector_size } as i64 ==
                                unsafe { (*p_pager_1).journal_off } {
                        __state = 36;
                    } else { __state = 35; }
                }
                33 => { { let _ = 0; }; __state = 34; }
                34 => {
                    n_rec =
                        ((sz_j - unsafe { (*p_pager_1).sector_size } as i64) /
                                    (unsafe { (*p_pager_1).page_size } + 8 as i64)) as i32 as
                            u32;
                    __state = 32;
                }
                35 => {
                    if unsafe { (*p_pager_1).journal_off } ==
                            unsafe { (*p_pager_1).sector_size } as i64 {
                        __state = 38;
                    } else { __state = 37; }
                }
                36 => {
                    n_rec =
                        ((sz_j - unsafe { (*p_pager_1).journal_off }) /
                                    (unsafe { (*p_pager_1).page_size } + 8 as i64)) as i32 as
                            u32;
                    __state = 35;
                }
                37 => { u = 0 as u32; __state = 44; }
                38 => {
                    rc = pager_truncate(unsafe { &mut *p_pager_1 }, mx_pg);
                    __state = 39;
                }
                39 => { if rc != 0 { __state = 41; } else { __state = 40; } }
                40 => {
                    unsafe { (*p_pager_1).db_size = mx_pg };
                    __state = 42;
                }
                41 => { __state = 2; }
                42 => {
                    if unsafe { (*p_pager_1).mx_pgno } < mx_pg {
                        __state = 43;
                    } else { __state = 37; }
                }
                43 => {
                    unsafe { (*p_pager_1).mx_pgno = mx_pg };
                    __state = 37;
                }
                44 => {
                    if u < n_rec { __state = 45; } else { __state = 24; }
                }
                45 => {
                    if need_pager_reset != 0 {
                        __state = 48;
                    } else { __state = 47; }
                }
                46 => {
                    { let __p = &mut u; let __t = *__p; *__p += 1; __t };
                    __state = 44;
                }
                47 => {
                    rc =
                        pager_playback_one_page(p_pager_1,
                            unsafe { &mut (*p_pager_1).journal_off },
                            core::ptr::null_mut(), 1, 0);
                    __state = 50;
                }
                48 => {
                    pager_reset(unsafe { &mut *p_pager_1 });
                    __state = 49;
                }
                49 => { need_pager_reset = 0; __state = 47; }
                50 => { if rc == 0 { __state = 51; } else { __state = 52; } }
                51 => {
                    {
                        let __p = &mut n_playback;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 46;
                }
                52 => {
                    if rc == 101 { __state = 53; } else { __state = 54; }
                }
                53 => {
                    unsafe { (*p_pager_1).journal_off = sz_j };
                    __state = 55;
                }
                54 => {
                    if rc == 10 | 2 << 8 {
                        __state = 56;
                    } else { __state = 57; }
                }
                55 => { __state = 24; }
                56 => { rc = 0; __state = 58; }
                57 => { __state = 2; }
                58 => { __state = 2; }
                59 => { __state = 2; }
                60 => {
                    unsafe {
                        (*p_pager_1).change_count_done =
                            unsafe { (*p_pager_1).temp_file }
                    };
                    __state = 62;
                }
                61 => {
                    rc =
                        unsafe {
                            sqlite3_pager_set_pagesize(p_pager_1, &mut saved_page_size,
                                -1)
                        };
                    __state = 60;
                }
                62 => {
                    if rc == 0 &&
                            (unsafe { (*p_pager_1).e_state } as i32 >= 4 ||
                                unsafe { (*p_pager_1).e_state } as i32 == 0) {
                        __state = 64;
                    } else { __state = 63; }
                }
                63 => { if rc == 0 { __state = 66; } else { __state = 65; } }
                64 => {
                    rc =
                        unsafe {
                            sqlite3_pager_sync(unsafe { &*p_pager_1 },
                                core::ptr::null())
                        };
                    __state = 63;
                }
                65 => {
                    if rc == 0 && !(z_super).is_null() && res != 0 {
                        __state = 69;
                    } else { __state = 68; }
                }
                66 => {
                    rc =
                        pager_end_transaction(p_pager_1,
                            (z_super != core::ptr::null_mut()) as i32, 0);
                    __state = 67;
                }
                67 => { __state = 65; }
                68 => {
                    if is_hot_1 != 0 && n_playback != 0 {
                        __state = 73;
                    } else { __state = 72; }
                }
                69 => { { let _ = 0; }; __state = 70; }
                70 => {
                    rc =
                        pager_delsuper(unsafe { &*p_pager_1 },
                            z_super as *const i8);
                    __state = 71;
                }
                71 => { __state = 68; }
                72 => { free_super_journal(z_super); __state = 74; }
                73 => {
                    unsafe {
                        sqlite3_log(27 | 2 << 8,
                            c"recovered %d pages from %s".as_ptr() as *mut i8 as
                                *const i8, n_playback, unsafe { (*p_pager_1).z_journal })
                    };
                    __state = 72;
                }
                74 => {
                    set_sector_size(unsafe { &mut *p_pager_1 });
                    __state = 75;
                }
                75 => { return rc; }
                _ => {}
            }
        }
    }

    /// Size of the journal file in bytes
    /// Number of Records in the journal
    /// Unsigned loop counter
    /// Size of the original file in pages
    /// Result code of a subroutine
    /// Value returned by sqlite3OsAccess()
    /// Name of super-journal file if any
    /// True to reset page prior to first page rollback
    /// Total number of pages restored from journal
    /// Figure out how many records are in the journal.  Abort early if
    ///* the journal is empty.
    /// Read the super-journal name from the journal, if it is present.
    ///* If a super-journal file name is specified, but the file is not
    ///* present on disk, then the journal is not hot and does not need to be
    ///* played back.
    /// This loop terminates either when a readJournalHdr() or
    ///* pager_playback_one_page() call returns SQLITE_DONE or an IO error
    ///* occurs.
    /// Read the next journal header from the journal file.  If there are
    ///* not enough bytes left in the journal file for a complete header, or
    ///* it is corrupted, then a process must have failed while writing it.
    ///* This indicates nothing more needs to be rolled back.
    /// If nRec is 0xffffffff, then this journal was created by a process
    ///* working in no-sync mode. This means that the rest of the journal
    ///* file consists of pages, there are no more journal headers. Compute
    ///* the value of nRec based on this assumption.
    /// If nRec is 0 and this rollback is of a transaction created by this
    ///* process and if this is the final header in the journal, then it means
    ///* that this part of the journal was being filled but has not yet been
    ///* synced to disk.  Compute the number of pages based on the remaining
    ///* size of the file.
    ///*
    ///* The third term of the test was added to fix ticket #2565.
    ///* When rolling back a hot journal, nRec==0 always means that the next
    ///* chunk of the journal contains zero pages to be rolled back.  But
    ///* when doing a ROLLBACK and the nRec==0 chunk is the last chunk in
    ///* the journal, it means that the journal might contain additional
    ///* pages that need to be rolled back and that the number of pages
    ///* should be computed based on the journal file size.
    /// If this is the first header read from the journal, truncate the
    ///* database file back to its original size.
    /// Copy original pages out of the journal and back into the
    ///* database file and/or page cache.
    /// If the journal has been truncated, simply stop reading and
    ///* processing the journal. This might happen if the journal was
    ///* not completely written and synced prior to a crash.  In that
    ///* case, the database should have never been written in the
    ///* first place so it is OK to simply abandon the rollback.
    /// If we are unable to rollback, quit and return the error
    ///* code.  This will cause the pager to enter the error state
    ///* so that no further harm will be done.  Perhaps the next
    ///* process to come along will be able to rollback the database.
    ///NOTREACHED
    /// Following a rollback, the database file should be back in its original
    ///* state prior to the start of the transaction, so invoke the
    ///* SQLITE_FCNTL_DB_UNCHANGED file-control method to disable the
    ///* assertion that the transaction counter was modified.
    /// If this playback is happening automatically as a result of an IO or
    ///* malloc error that occurred after the change-counter was updated but
    ///* before the transaction was committed, then the change-counter
    ///* modification may just have been reverted. If this happens in exclusive
    ///* mode, then subsequent transactions performed by the connection will not
    ///* update the change-counter at all. This may lead to cache inconsistency
    ///* problems for other processes at some point in the future. So, just
    ///* in case this has happened, clear the changeCountDone flag now.
    /// If there was a super-journal and this routine will return success,
    ///* see if it is possible to delete the super-journal.
    /// The Pager.sectorSize variable may have been updated while rolling
    ///* back a journal created by a process with a different sector size
    ///* value. Reset it to the correct value for this process.
    unreachable!();
}

///* This function is called whenever an IOERR or FULL error that requires
///* the pager to transition into the ERROR state may have occurred.
///* The first argument is a pointer to the pager structure, the second
///* the error-code about to be returned by a pager API function. The
///* value returned is a copy of the second argument to this function.
///*
///* If the second argument is SQLITE_FULL, SQLITE_IOERR or one of the
///* IOERR sub-codes, the pager enters the ERROR state and the error code
///* is stored in Pager.errCode. While the pager remains in the ERROR state,
///* all major API calls on the Pager will immediately return Pager.errCode.
///*
///* The ERROR state indicates that the contents of the pager-cache
///* cannot be trusted. This state can be cleared by completely discarding
///* the contents of the pager-cache. If a transaction was active when
///* the persistent error occurred, then the rollback journal may need
///* to be replayed to restore the contents of the database file (as if
///* it were a hot-journal).
extern "C" fn pager_error(p_pager_1: *mut Pager, rc: i32) -> i32 {
    let rc2: i32 = rc & 255;
    { let _ = 0; };
    { let _ = 0; };
    if rc2 == 13 || rc2 == 10 {
        unsafe { (*p_pager_1).err_code = rc };
        unsafe { (*p_pager_1).e_state = 6 as u8 };
        set_getter_method(unsafe { &mut *p_pager_1 });
    }
    return rc;
}

///* If a write transaction is open, then all changes made within the
///* transaction are reverted and the current write-transaction is closed.
///* The pager falls back to PAGER_READER state if successful, or PAGER_ERROR
///* state if an error occurs.
///*
///* If the pager is already in PAGER_ERROR state when this function is called,
///* it returns Pager.errCode immediately. No work is performed in this case.
///*
///* Otherwise, in rollback mode, this function performs two functions:
///*
///*   1) It rolls back the journal file, restoring all database file and
///*      in-memory cache pages to the state they were in when the transaction
///*      was opened, and
///*
///*   2) It finalizes the journal file, so that it is not used for hot
///*      rollback at any point in the future.
///*
///* Finalization of the journal file (task 2) is only performed if the
///* rollback is successful.
///*
///* In WAL mode, all cache-entries containing data modified within the
///* current transaction are either expelled from the cache or reverted to
///* their pre-transaction state by re-reading data from the database or
///* WAL files. The WAL transaction is then closed.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_rollback(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = 0;

    /// PagerRollback() is a no-op if called in READER or OPEN state. If
    ///* the pager is already in the ERROR state, the rollback is not
    ///* attempted here. Instead, the error code is returned to the caller.
    { let _ = 0; };
    if unsafe { (*p_pager_1).e_state } as i32 == 6 {
        return unsafe { (*p_pager_1).err_code };
    }
    if unsafe { (*p_pager_1).e_state } as i32 <= 1 { return 0; }
    if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
        let mut rc2: i32 = 0;
        rc = unsafe { sqlite3_pager_savepoint(p_pager_1, 2, -1) };
        rc2 =
            pager_end_transaction(p_pager_1,
                unsafe { (*p_pager_1).set_super } as i32, 0);
        if rc == 0 { rc = rc2; }
    } else if !(unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                            core::ptr::null()) as i32 != 0 ||
            unsafe { (*p_pager_1).e_state } as i32 == 2 {
        let e_state: i32 = unsafe { (*p_pager_1).e_state } as i32;
        rc = pager_end_transaction(p_pager_1, 0, 0);
        if (unsafe { (*p_pager_1).mem_db } == 0) as i32 != 0 && e_state > 2 {

            /// This can happen using journal_mode=off. Move the pager to the error
            ///* state to indicate that the contents of the cache may not be trusted.
            ///* Any active readers will get SQLITE_ABORT.
            unsafe { (*p_pager_1).err_code = 4 };
            unsafe { (*p_pager_1).e_state = 6 as u8 };
            set_getter_method(unsafe { &mut *p_pager_1 });
            return rc;
        }
    } else { rc = pager_playback(p_pager_1, 0); }
    { let _ = 0; };
    { let _ = 0; };

    /// If an error occurs during a ROLLBACK, we can no longer trust the pager
    ///* cache. So call pager_error() on the way out to make any error persistent.
    return pager_error(p_pager_1, rc);
}

///* This function is a no-op if the pager is in exclusive mode and not
///* in the ERROR state. Otherwise, it switches the pager to PAGER_OPEN
///* state.
///*
///* If the pager is not in exclusive-access mode, the database file is
///* completely unlocked. If the file is unlocked and the file-system does
///* not exhibit the UNDELETABLE_WHEN_OPEN property, the journal file is
///* closed (if it is open).
///*
///* If the pager is in ERROR state when this function is called, the
///* contents of the pager cache are discarded before switching back to
///* the OPEN state. Regardless of whether the pager is in exclusive-mode
///* or not, any journal file left in the file-system will be treated
///* as a hot-journal and rolled back the next time a read-transaction
///* is opened (by this or by any other connection).
#[allow(unused_doc_comments)]
extern "C" fn pager_unlock(p_pager_1: *mut Pager) -> () {
    { let _ = 0; };
    unsafe { sqlite3_bitvec_destroy(unsafe { (*p_pager_1).p_in_journal }) };
    unsafe { (*p_pager_1).p_in_journal = core::ptr::null_mut() };
    release_all_savepoints(unsafe { &mut *p_pager_1 });
    if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
        { let _ = 0; };
        if unsafe { (*p_pager_1).e_state } as i32 == 6 {

            /// If an IO error occurs in wal.c while attempting to wrap the wal file,
            ///* then the Wal object may be holding a write-lock but no read-lock.
            ///* This call ensures that the write-lock is dropped as well. We cannot
            ///* have sqlite3WalEndReadTransaction() drop the write-lock, as it once
            ///* did, because this would break "BEGIN EXCLUSIVE" handling for
            ///* SQLITE_ENABLE_SETLK_TIMEOUT builds.
            {
                let _ =
                    unsafe {
                        sqlite3_wal_end_write_transaction(unsafe {
                                (*p_pager_1).p_wal
                            })
                    };
            };
        }
        unsafe {
            sqlite3_wal_end_read_transaction(unsafe { (*p_pager_1).p_wal })
        };
        unsafe { (*p_pager_1).e_state = 0 as u8 };
    } else if (unsafe { (*p_pager_1).exclusive_mode } == 0) as i32 != 0 {
        let mut rc: i32 = 0;
        /// Error code returned by pagerUnlockDb()
        let i_dc: i32 =
            if unsafe { (*unsafe { (*p_pager_1).fd }).p_methods } !=
                    core::ptr::null() {
                unsafe {
                    sqlite3_os_device_characteristics(unsafe {
                            (*p_pager_1).fd
                        })
                }
            } else { 0 };

        /// If the operating system support deletion of open files, then
        ///* close the journal file when dropping the database lock.  Otherwise
        ///* another connection with journal_mode=delete might delete the file
        ///* out from under us.
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if 0 == i_dc & 2048 ||
                1 != unsafe { (*p_pager_1).journal_mode } as i32 & 5 {
            unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
        }

        /// If the pager is in the ERROR state and the call to unlock the database
        ///* file fails, set the current lock to UNKNOWN_LOCK. See the comment
        ///* above the #define for UNKNOWN_LOCK for an explanation of why this
        ///* is necessary.
        (rc = pager_unlock_db(unsafe { &mut *p_pager_1 }, 0));
        if rc != 0 && unsafe { (*p_pager_1).e_state } as i32 == 6 {
            unsafe { (*p_pager_1).e_lock = (4 + 1) as u8 };
        }

        /// The pager state may be changed from PAGER_ERROR to PAGER_OPEN here
        ///* without clearing the error code. This is intentional - the error
        ///* code is cleared and the cache reset in the block below.
        { let _ = 0; };
        unsafe { (*p_pager_1).e_state = 0 as u8 };
    }

    /// If Pager.errCode is set, the contents of the pager cache cannot be
    ///* trusted. Now that there are no outstanding references to the pager,
    ///* it can safely move back to PAGER_OPEN state. This happens in both
    ///* normal and exclusive-locking mode.
    { let _ = 0; };
    if unsafe { (*p_pager_1).err_code } != 0 {
        if unsafe { (*p_pager_1).temp_file } as i32 == 0 {
            pager_reset(unsafe { &mut *p_pager_1 });
            unsafe { (*p_pager_1).change_count_done = 0 as u8 };
            unsafe { (*p_pager_1).e_state = 0 as u8 };
        } else {
            unsafe {
                (*p_pager_1).e_state =
                    if unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                                core::ptr::null() {
                            0
                        } else { 1 } as u8
            };
        }
        if unsafe { (*p_pager_1).b_use_fetch } != 0 {
            unsafe {
                sqlite3_os_unfetch(unsafe { (*p_pager_1).fd }, 0 as i64,
                    core::ptr::null_mut())
            };
        }
        unsafe { (*p_pager_1).err_code = 0 };
        set_getter_method(unsafe { &mut *p_pager_1 });
    }
    unsafe { (*p_pager_1).journal_off = 0 as i64 };
    unsafe { (*p_pager_1).journal_hdr = 0 as i64 };
    unsafe { (*p_pager_1).set_super = 0 as u8 };
}

///* Execute a rollback if a transaction is active and unlock the
///* database file.
///*
///* If the pager has already entered the ERROR state, do not attempt
///* the rollback at this time. Instead, pager_unlock() is called. The
///* call to pager_unlock() will discard all in-memory pages, unlock
///* the database file and move the pager back to OPEN state. If this
///* means that there is a hot-journal left in the file-system, the next
///* connection to obtain a shared lock on the pager (which may be this one)
///* will roll it back.
///*
///* If the pager has not already entered the ERROR state, but an IO or
///* malloc error occurs during a rollback, then this will itself cause
///* the pager to enter the ERROR state. Which will be cleared by the
///* call to pager_unlock(), as described above.
#[allow(unused_doc_comments)]
extern "C" fn pager_unlock_and_rollback(p_pager_1: *mut Pager) -> () {
    if unsafe { (*p_pager_1).e_state } as i32 != 6 &&
            unsafe { (*p_pager_1).e_state } as i32 != 0 {
        { let _ = 0; };
        if unsafe { (*p_pager_1).e_state } as i32 >= 2 {
            unsafe { sqlite3_begin_benign_malloc() };
            unsafe { sqlite3_pager_rollback(p_pager_1) };
            unsafe { sqlite3_end_benign_malloc() };
        } else if (unsafe { (*p_pager_1).exclusive_mode } == 0) as i32 != 0 {
            { let _ = 0; };
            pager_end_transaction(p_pager_1, 0, 0);
        }
    } else if unsafe { (*p_pager_1).e_state } as i32 == 6 &&
                unsafe { (*p_pager_1).journal_mode } as i32 == 4 &&
            unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                core::ptr::null() {
        /// Special case for a ROLLBACK due to I/O error with an in-memory
        ///* journal:  We have to rollback immediately, before the journal is
        ///* closed, because once it is closed, all content is forgotten.
        let err_code: i32 = unsafe { (*p_pager_1).err_code };
        let e_lock: u8 = unsafe { (*p_pager_1).e_lock };
        unsafe { (*p_pager_1).e_state = 0 as u8 };
        unsafe { (*p_pager_1).err_code = 0 };
        unsafe { (*p_pager_1).e_lock = 4 as u8 };
        unsafe { pager_playback(p_pager_1, 1) };
        unsafe { (*p_pager_1).err_code = err_code };
        unsafe { (*p_pager_1).e_lock = e_lock };
    }
    pager_unlock(p_pager_1);
}

///* If the reference count has reached zero, rollback any active
///* transaction and unlock the pager.
///*
///* Except, in locking_mode=EXCLUSIVE when there is nothing to in
///* the rollback journal, the unlock is not performed and there is
///* nothing to rollback, so this routine is a no-op.
#[allow(unused_doc_comments)]
extern "C" fn pager_unlock_if_unused(p_pager_1: *mut Pager) -> () {
    if unsafe { sqlite3_pcache_ref_count(unsafe { (*p_pager_1).p_p_cache }) }
            == 0 as i64 {
        { let _ = 0; };

        /// because page1 is never memory mapped
        pager_unlock_and_rollback(p_pager_1);
    }
}

/// Forward references to the various page getters
#[allow(unused_doc_comments)]
extern "C" fn get_page_normal(p_pager_1: *mut Pager, pgno: Pgno,
    pp_page_1: *mut *mut DbPage, flags: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut p_pg: *mut PgHdr = core::ptr::null_mut();
    '__b16: loop {
        '__c16: loop {
            let mut no_content: u8 = 0 as u8;
            /// True if PAGER_GET_NOCONTENT is set
            let mut p_base: *mut Sqlite3PcachePage = core::ptr::null_mut();
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if pgno == 0 as u32 {
                return unsafe { sqlite3_corrupt_error(5605) };
            }
            p_base =
                unsafe {
                    sqlite3_pcache_fetch(unsafe { (*p_pager_1).p_p_cache },
                        pgno, 3)
                };
            if p_base == core::ptr::null_mut() {
                p_pg = core::ptr::null_mut();
                rc =
                    unsafe {
                        sqlite3_pcache_fetch_stress(unsafe {
                                (*p_pager_1).p_p_cache
                            }, pgno, &mut p_base)
                    };
                if rc != 0 { break '__b16; }
                if p_base == core::ptr::null_mut() { rc = 7; break '__b16; }
            }
            p_pg =
                {
                        unsafe {
                            *pp_page_1 =
                                unsafe {
                                        sqlite3_pcache_fetch_finish(unsafe {
                                                (*p_pager_1).p_p_cache
                                            }, pgno, p_base)
                                    } as *mut DbPage
                        };
                        unsafe { *pp_page_1 }
                    } as *mut PgHdr;
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            no_content = (flags & 1 != 0) as u8;
            if !(unsafe { (*p_pg).p_pager }).is_null() &&
                    (no_content == 0) as i32 != 0 {

                /// In this case the pcache already contains an initialized copy of
                ///* the page. Return without further ado.
                { let _ = 0; };
                {
                    let __p = unsafe { &mut (*p_pager_1).a_stat[0 as usize] };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                return 0;
            } else {
                if pgno == unsafe { (*p_pager_1).lck_pgno } {
                    rc = unsafe { sqlite3_corrupt_error(5637) };
                    break '__b16;
                }
                unsafe { (*p_pg).p_pager = p_pager_1 };
                { let _ = 0; };
                if !(unsafe { (*unsafe { (*p_pager_1).fd }).p_methods } !=
                                            core::ptr::null()) as i32 != 0 ||
                            unsafe { (*p_pager_1).db_size } < pgno || no_content != 0 {
                    if pgno > unsafe { (*p_pager_1).mx_pgno } {
                        rc = 13;
                        if pgno <= unsafe { (*p_pager_1).db_size } {
                            unsafe { sqlite3_pcache_release(p_pg) };
                            p_pg = core::ptr::null_mut();
                        }
                        break '__b16;
                    }
                    if no_content != 0 {

                        /// Failure to set the bits in the InJournal bit-vectors is benign.
                        ///* It merely means that we might do some extra work to journal a
                        ///* page that does not need to be journaled.  Nevertheless, be sure
                        ///* to test the case where a malloc error occurs while trying to set
                        ///* a bit in a bit vector.
                        unsafe { sqlite3_begin_benign_malloc() };
                        if pgno <= unsafe { (*p_pager_1).db_orig_size } {
                            unsafe {
                                sqlite3_bitvec_set(unsafe { (*p_pager_1).p_in_journal },
                                    pgno)
                            };
                        }
                        add_to_savepoint_bitvecs(unsafe { &*p_pager_1 }, pgno);
                        unsafe { sqlite3_end_benign_malloc() };
                    }
                    unsafe {
                        memset(unsafe { (*p_pg).p_data }, 0,
                            unsafe { (*p_pager_1).page_size } as u64)
                    };
                } else {
                    { let _ = 0; };
                    {
                        let __p = unsafe { &mut (*p_pager_1).a_stat[1 as usize] };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    rc = read_db_page(unsafe { &mut *p_pg });
                    if rc != 0 { break '__b16; }
                }
            }
            return 0;
            break '__c16;
        }
        if !(false) { break '__b16; }
    }

    /// True if PAGER_GET_NOCONTENT is set
    /// In this case the pcache already contains an initialized copy of
    ///* the page. Return without further ado.
    /// The pager cache has created a new page. Its content needs to
    ///* be initialized. But first some error checks:
    ///*
    ///* (*) obsolete.  Was: maximum page number is 2^31
    ///* (2) Never try to fetch the locking page
    /// Failure to set the bits in the InJournal bit-vectors is benign.
    ///* It merely means that we might do some extra work to journal a
    ///* page that does not need to be journaled.  Nevertheless, be sure
    ///* to test the case where a malloc error occurs while trying to set
    ///* a bit in a bit vector.
    { let _ = 0; };
    if !(p_pg).is_null() { unsafe { sqlite3_pcache_drop(p_pg) }; }
    pager_unlock_if_unused(p_pager_1);
    unsafe { *pp_page_1 = core::ptr::null_mut() };
    return rc;
}

/// The page getter for when memory-mapped I/O is enabled
#[allow(unused_doc_comments)]
extern "C" fn get_page_m_map(p_pager_1: *mut Pager, pgno: Pgno,
    pp_page_1: *mut *mut DbPage, flags: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut p_pg: *mut PgHdr = core::ptr::null_mut();
    let mut i_frame: u32 = 0 as u32;
    /// Frame to read from WAL file
    /// It is acceptable to use a read-only (mmap) page for any page except
    ///* page 1 if there is no write-transaction open or the ACQUIRE_READONLY
    ///* flag was specified by the caller. And so long as the db is not a
    ///* temporary or in-memory database.
    let b_mmap_ok: i32 =
        (pgno > 1 as u32 &&
                (unsafe { (*p_pager_1).e_state } as i32 == 1 ||
                    flags & 2 != 0)) as i32;
    { let _ = 0; };
    if pgno <= 1 as u32 && pgno == 0 as u32 {
        return unsafe { sqlite3_corrupt_error(5720) };
    }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if b_mmap_ok != 0 &&
            unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
        rc =
            unsafe {
                sqlite3_wal_find_frame(unsafe { (*p_pager_1).p_wal }, pgno,
                    &mut i_frame)
            };
        if rc != 0 {
            unsafe { *pp_page_1 = core::ptr::null_mut() };
            return rc;
        }
    }
    if b_mmap_ok != 0 && i_frame == 0 as u32 {
        let mut p_data: *mut () = core::ptr::null_mut();
        rc =
            unsafe {
                sqlite3_os_fetch(unsafe { (*p_pager_1).fd },
                    (pgno - 1 as Pgno) as i64 *
                        unsafe { (*p_pager_1).page_size },
                    unsafe { (*p_pager_1).page_size } as i32, &mut p_data)
            };
        if rc == 0 && !(p_data).is_null() {
            if unsafe { (*p_pager_1).e_state } as i32 > 1 ||
                    unsafe { (*p_pager_1).temp_file } != 0 {
                p_pg =
                    unsafe {
                            sqlite3_pager_lookup(unsafe { &*p_pager_1 }, pgno)
                        } as *mut PgHdr;
            }
            if p_pg == core::ptr::null_mut() {
                rc =
                    pager_acquire_map_page(p_pager_1, pgno, p_data, &mut p_pg);
            } else {
                unsafe {
                    sqlite3_os_unfetch(unsafe { (*p_pager_1).fd },
                        (pgno - 1 as Pgno) as i64 *
                            unsafe { (*p_pager_1).page_size }, p_data)
                };
            }
            if !(p_pg).is_null() {
                { let _ = 0; };
                unsafe { *pp_page_1 = p_pg as *mut DbPage };
                return 0;
            }
        }
        if rc != 0 {
            unsafe { *pp_page_1 = core::ptr::null_mut() };
            return rc;
        }
    }
    return get_page_normal(p_pager_1, pgno, pp_page_1, flags);
}

///* Set the Pager.xGet method for the appropriate routine used to fetch
///* content from the pager.
extern "C" fn set_getter_method(p_pager_1: &mut Pager) -> () {
    if (*p_pager_1).err_code != 0 {
        (*p_pager_1).x_get = Some(get_page_error);
    } else if (*p_pager_1).b_use_fetch != 0 {
        (*p_pager_1).x_get = Some(get_page_m_map);
    } else { (*p_pager_1).x_get = Some(get_page_normal); }
}

///* Invoke SQLITE_FCNTL_MMAP_SIZE based on the current value of szMmap.
extern "C" fn pager_fix_maplimit(p_pager_1: *mut Pager) -> () {
    let fd: *const Sqlite3File =
        unsafe { (*p_pager_1).fd } as *const Sqlite3File;
    if unsafe { (*fd).p_methods } != core::ptr::null() &&
            unsafe { (*unsafe { (*fd).p_methods }).i_version } as i32 >= 3 {
        let mut sz: Sqlite3Int64 = 0 as Sqlite3Int64;
        sz = unsafe { (*p_pager_1).sz_mmap };
        unsafe { (*p_pager_1).b_use_fetch = (sz > 0 as i64) as u8 };
        set_getter_method(unsafe { &mut *p_pager_1 });
        unsafe {
            sqlite3_os_file_control_hint(unsafe { (*p_pager_1).fd }, 18,
                &raw mut sz as *mut ())
        };
    }
}

///* Change the page size used by the Pager object. The new page size
///* is passed in *pPageSize.
///*
///* If the pager is in the error state when this function is called, it
///* is a no-op. The value returned is the error state error code (i.e.
///* one of SQLITE_IOERR, an SQLITE_IOERR_xxx sub-code or SQLITE_FULL).
///*
///* Otherwise, if all of the following are true:
///*
///*   * the new page size (value of *pPageSize) is valid (a power
///*     of two between 512 and SQLITE_MAX_PAGE_SIZE, inclusive), and
///*
///*   * there are no outstanding page references, and
///*
///*   * the database is either not an in-memory database or it is
///*     an in-memory database that currently consists of zero pages.
///*
///* then the pager object page size is set to *pPageSize.
///*
///* If the page size is changed, then this function uses sqlite3PageMalloc()
///* to obtain a new Pager.pTmpSpace buffer. If this allocation attempt
///* fails, SQLITE_NOMEM is returned and the page size remains unchanged.
///* In all other cases, SQLITE_OK is returned.
///*
///* If the page size is not changed, either because one of the enumerated
///* conditions above is not true, the pager was in error state when this
///* function was called, or because the memory allocation attempt failed,
///* then *pPageSize is set to the old, retained page size before returning.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_set_pagesize(p_pager_1: *mut Pager,
    p_page_size_1: &mut u32, mut n_reserve_1: i32) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        /// It is not possible to do a full assert_pager_state() here, as this
        ///* function may be called from within PagerOpen(), before the state
        ///* of the Pager object is internally consistent.
        ///*
        ///* At one point this function returned an error if the pager was in
        ///* PAGER_ERROR state. But since PAGER_ERROR state guarantees that
        ///* there is at least one outstanding page reference, this function
        ///* is a no-op for that case anyhow.
        let page_size: u32 = *p_page_size_1;
        { let _ = 0; };
        if (unsafe { (*p_pager_1).mem_db } as i32 == 0 ||
                            unsafe { (*p_pager_1).db_size } == 0 as u32) &&
                        unsafe {
                                sqlite3_pcache_ref_count(unsafe { (*p_pager_1).p_p_cache })
                            } == 0 as i64 && page_size != 0 &&
                page_size != unsafe { (*p_pager_1).page_size } as u32 {
            let mut p_new: *mut i8 = 0 as *mut () as *mut i8;
            /// New temp space
            let mut n_byte: i64 = 0 as i64;
            if unsafe { (*p_pager_1).e_state } as i32 > 0 &&
                    unsafe { (*unsafe { (*p_pager_1).fd }).p_methods } !=
                        core::ptr::null() {
                rc =
                    unsafe {
                        sqlite3_os_file_size(unsafe { (*p_pager_1).fd },
                            &mut n_byte)
                    };
            }
            if rc == 0 {

                /// 8 bytes of zeroed overrun space is sufficient so that the b-tree
                ///cell header parser will never run off the end of the allocation
                (p_new =
                    unsafe {
                            sqlite3_page_malloc((page_size + 8 as u32) as i32)
                        } as *mut i8);
                if (p_new).is_null() as i32 != 0 {
                    rc = 7;
                } else {
                    unsafe {
                        memset(unsafe { p_new.add(page_size as usize) } as *mut (),
                            0, 8 as u64)
                    };
                }
            }
            if rc == 0 {
                pager_reset(unsafe { &mut *p_pager_1 });
                rc =
                    unsafe {
                        sqlite3_pcache_set_page_size(unsafe {
                                (*p_pager_1).p_p_cache
                            }, page_size as i32)
                    };
            }
            if rc == 0 {
                unsafe {
                    sqlite3_page_free(unsafe { (*p_pager_1).p_tmp_space } as
                            *mut ())
                };
                unsafe { (*p_pager_1).p_tmp_space = p_new };
                unsafe {
                    (*p_pager_1).db_size =
                        ((n_byte + page_size as i64 - 1 as i64) / page_size as i64)
                            as Pgno
                };
                unsafe { (*p_pager_1).page_size = page_size as i64 };
                unsafe {
                    (*p_pager_1).lck_pgno =
                        (sqlite3_pending_byte as u32 / page_size) as Pgno +
                            1 as Pgno
                };
            } else { unsafe { sqlite3_page_free(p_new as *mut ()) }; }
        }
        *p_page_size_1 = unsafe { (*p_pager_1).page_size } as u32;
        if rc == 0 {
            if n_reserve_1 < 0 {
                n_reserve_1 = unsafe { (*p_pager_1).n_reserve } as i32;
            }
            { let _ = 0; };
            unsafe { (*p_pager_1).n_reserve = n_reserve_1 as i16 };
            pager_fix_maplimit(p_pager_1);
        }
        return rc;
    }
}

///* Return true if it is necessary to write page *pPg into the sub-journal.
///* A page needs to be written into the sub-journal if there exists one
///* or more open savepoints for which:
///*
///*   * The page-number is less than or equal to PagerSavepoint.nOrig, and
///*   * The bit corresponding to the page-number is not set in
///*     PagerSavepoint.pInSavepoint.
extern "C" fn subj_requires_page(p_pg_1: &PgHdr) -> i32 {
    let p_pager: *const Pager = (*p_pg_1).p_pager as *const Pager;
    let mut p: *const PagerSavepoint = core::ptr::null();
    let pgno: Pgno = (*p_pg_1).pgno;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b17: loop {
            if !(i < unsafe { (*p_pager).n_savepoint }) { break '__b17; }
            '__c17: loop {
                p =
                    unsafe {
                        unsafe { (*p_pager).a_savepoint.offset(i as isize) }
                    };
                if unsafe { (*p).n_orig } >= pgno &&
                        0 ==
                            unsafe {
                                sqlite3_bitvec_test_not_null(unsafe { (*p).p_in_savepoint },
                                    pgno)
                            } {
                    {
                        i = i + 1;
                        '__b18: loop {
                            if !(i < unsafe { (*p_pager).n_savepoint }) {
                                break '__b18;
                            }
                            '__c18: loop {
                                unsafe {
                                    (*unsafe {
                                                    (*p_pager).a_savepoint.offset(i as isize)
                                                }).b_truncate_on_release = 0
                                };
                                break '__c18;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    return 1;
                }
                break '__c17;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Ensure that the sub-journal file is open. If it is already open, this
///* function is a no-op.
///*
///* SQLITE_OK is returned if everything goes according to plan. An
///* SQLITE_IOERR_XXX error code is returned if a call to sqlite3OsOpen()
///* fails.
extern "C" fn open_sub_journal(p_pager_1: &Pager) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if !(unsafe { (*(*p_pager_1).sjfd).p_methods } != core::ptr::null())
                    as i32 != 0 {
            let flags: i32 = (8192 | 2 | 4 | 16 | 8) as i32;
            let mut n_stmt_spill: i32 = sqlite3Config.n_stmt_spill;
            if (*p_pager_1).journal_mode as i32 == 4 ||
                    (*p_pager_1).subj_in_memory != 0 {
                n_stmt_spill = -1;
            }
            rc =
                unsafe {
                    sqlite3_journal_open((*p_pager_1).p_vfs, core::ptr::null(),
                        (*p_pager_1).sjfd, flags, n_stmt_spill)
                };
        }
        return rc;
    }
}

///* Write a 32-bit integer into the given file descriptor.  Return SQLITE_OK
///* on success or an error code is something goes wrong.
extern "C" fn write32bits(fd: *mut Sqlite3File, offset: i64, val: u32)
    -> i32 {
    let mut ac: [i8; 4] = [0; 4];
    unsafe { sqlite3_put4byte(&raw mut ac[0 as usize] as *mut u8, val) };
    return unsafe {
            sqlite3_os_write(fd,
                &raw mut ac[0 as usize] as *mut i8 as *const (), 4, offset)
        };
}

///* Append a record of the current state of page pPg to the sub-journal.
///*
///* If successful, set the bit corresponding to pPg->pgno in the bitvecs
///* for all open savepoints before returning.
///*
///* This function returns SQLITE_OK if everything is successful, an IO
///* error code if the attempt to write to the sub-journal fails, or
///* SQLITE_NOMEM if a malloc fails while setting a bit in a savepoint
///* bitvec.
#[allow(unused_doc_comments)]
extern "C" fn subjournal_page(p_pg_1: &PgHdr) -> i32 {
    let mut rc: i32 = 0;
    let p_pager: *mut Pager = (*p_pg_1).p_pager;
    if unsafe { (*p_pager).journal_mode } as i32 != 2 {

        /// Open the sub-journal, if it has not already been opened
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        rc = open_sub_journal(unsafe { &*p_pager });
        if rc == 0 {
            let p_data: *mut () = (*p_pg_1).p_data;
            let offset: i64 =
                unsafe { (*p_pager).n_sub_rec } as i64 *
                    (4 as i64 + unsafe { (*p_pager).page_size });
            let mut p_data2: *const i8 = core::ptr::null();
            p_data2 = p_data as *mut i8;
            rc =
                write32bits(unsafe { (*p_pager).sjfd }, offset,
                    (*p_pg_1).pgno);
            if rc == 0 {
                rc =
                    unsafe {
                        sqlite3_os_write(unsafe { (*p_pager).sjfd },
                            p_data2 as *const (),
                            unsafe { (*p_pager).page_size } as i32, offset + 4 as i64)
                    };
            }
        }
    }
    if rc == 0 {
        {
            let __p = unsafe { &mut (*p_pager).n_sub_rec };
            let __t = *__p;
            *__p += 1;
            __t
        };
        { let _ = 0; };
        rc = add_to_savepoint_bitvecs(unsafe { &*p_pager }, (*p_pg_1).pgno);
    }
    return rc;
}

extern "C" fn subjournal_page_if_required(p_pg_1: *mut PgHdr) -> i32 {
    if subj_requires_page(unsafe { &*p_pg_1 }) != 0 {
        return subjournal_page(unsafe { &*p_pg_1 });
    } else { return 0; }
}

///* Update the value of the change-counter at offsets 24 and 92 in
///* the header and the sqlite version number at offset 96.
///*
///* This is an unconditional update.  See also the pager_incr_changecounter()
///* routine which only updates the change-counter if the update is actually
///* needed, as determined by the pPager->changeCountDone state variable.
#[allow(unused_doc_comments)]
extern "C" fn pager_write_changecounter(p_pg_1: *mut PgHdr) -> () {
    let mut change_counter: u32 = 0 as u32;
    if p_pg_1 == core::ptr::null_mut() { return; }

    /// Increment the value just read and write it back to byte 24.
    (change_counter =
        unsafe {
                sqlite3_get4byte(unsafe {
                                &raw mut (*unsafe {
                                                        (*p_pg_1).p_pager
                                                    }).db_file_vers[0 as usize]
                            } as *mut u8 as *const u8)
            } + 1 as u32);
    unsafe {
        sqlite3_put4byte(unsafe {
                (unsafe { (*p_pg_1).p_data } as *mut i8 as
                        *mut u8).offset(24 as isize)
            }, change_counter)
    };

    /// Also store the SQLite version number in bytes 96..99 and in
    ///* bytes 92..95 store the change counter for which the version number
    ///* is valid.
    unsafe {
        sqlite3_put4byte(unsafe {
                (unsafe { (*p_pg_1).p_data } as *mut i8 as
                        *mut u8).offset(92 as isize)
            }, change_counter)
    };
    unsafe {
        sqlite3_put4byte(unsafe {
                (unsafe { (*p_pg_1).p_data } as *mut i8 as
                        *mut u8).offset(96 as isize)
            }, 3054000 as u32)
    };
}

///* This function is a wrapper around sqlite3WalFrames(). As well as logging
///* the contents of the list of pages headed by pList (connected by pDirty),
///* this function notifies any active backup processes that the pages have
///* changed.
///*
///* The list of pages passed into this routine is always sorted by page number.
///* Hence, if page 1 appears anywhere on the list, it will be the first page.
#[allow(unused_doc_comments)]
extern "C" fn pager_wal_frames(p_pager_1: &mut Pager,
    mut p_list_1: *mut PgHdr, n_truncate_1: Pgno, is_commit_1: i32) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut n_list: i32 = 0;
    /// Number of pages in pList
    let mut p: *mut PgHdr = core::ptr::null_mut();

    /// For looping over pages
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if is_commit_1 != 0 {
        /// If a WAL transaction is being committed, there is no point in writing
        ///* any pages with page numbers greater than nTruncate into the WAL file.
        ///* They will never be read by any client. So remove them from the pDirty
        ///* list here.
        let mut pp_next: *mut *mut PgHdr = &mut p_list_1;
        n_list = 0;
        {
            p = p_list_1;
            '__b19: loop {
                if !({ let __v = p; unsafe { *pp_next = __v }; __v } !=
                                core::ptr::null_mut()) {
                    break '__b19;
                }
                '__c19: loop {
                    if unsafe { (*p).pgno } <= n_truncate_1 {
                        pp_next = unsafe { &mut (*p).p_dirty };
                        { let __p = &mut n_list; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c19;
                }
                p = unsafe { (*p).p_dirty };
            }
        }
        { let _ = 0; };
    } else { n_list = 1; }
    (*p_pager_1).a_stat[2 as usize] += n_list as u32;
    if unsafe { (*p_list_1).pgno } == 1 as u32 {
        pager_write_changecounter(p_list_1);
    }
    rc =
        unsafe {
            sqlite3_wal_frames((*p_pager_1).p_wal,
                (*p_pager_1).page_size as i32, p_list_1, n_truncate_1,
                is_commit_1, (*p_pager_1).wal_sync_flags as i32)
        };
    if rc == 0 && !((*p_pager_1).p_backup).is_null() {
        {
            p = p_list_1;
            '__b20: loop {
                if !(!(p).is_null()) { break '__b20; }
                '__c20: loop {
                    unsafe {
                        sqlite3_backup_update((*p_pager_1).p_backup,
                            unsafe { (*p).pgno },
                            unsafe { (*p).p_data } as *mut u8 as *const u8)
                    };
                    break '__c20;
                }
                p = unsafe { (*p).p_dirty };
            }
        }
    }
    return rc;
}

///* Lock the database file to level eLock, which must be either SHARED_LOCK,
///* RESERVED_LOCK or EXCLUSIVE_LOCK. If the caller is successful, set the
///* Pager.eLock variable to the new locking state.
///*
///* Except, if Pager.eLock is set to UNKNOWN_LOCK when this function is
///* called, do not modify it unless the new locking state is EXCLUSIVE_LOCK.
///* See the comment above the #define of UNKNOWN_LOCK for an explanation
///* of this.
extern "C" fn pager_lock_db(p_pager_1: &mut Pager, e_lock_1: i32) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    if ((*p_pager_1).e_lock as i32) < e_lock_1 ||
            (*p_pager_1).e_lock as i32 == 4 + 1 {
        rc =
            if (*p_pager_1).no_lock != 0 {
                0
            } else { unsafe { sqlite3_os_lock((*p_pager_1).fd, e_lock_1) } };
        if rc == 0 && ((*p_pager_1).e_lock as i32 != 4 + 1 || e_lock_1 == 4) {
            (*p_pager_1).e_lock = e_lock_1 as u8;
        }
    }
    return rc;
}

///* Try to obtain a lock of type locktype on the database file. If
///* a similar or greater lock is already held, this function is a no-op
///* (returning SQLITE_OK immediately).
///*
///* Otherwise, attempt to obtain the lock using sqlite3OsLock(). Invoke
///* the busy callback if the lock is currently not available. Repeat
///* until the busy callback returns false or until the attempt to
///* obtain the lock succeeds.
///*
///* Return SQLITE_OK on success and an error code if we cannot obtain
///* the lock. If the lock is obtained successfully, set the Pager.state
///* variable to locktype before returning.
#[allow(unused_doc_comments)]
extern "C" fn pager_wait_on_lock(p_pager_1: *mut Pager, locktype: i32)
    -> i32 {
    let mut rc: i32 = 0;

    /// Return code
    /// Check that this is either a no-op (because the requested lock is
    ///* already held), or one of the transitions that the busy-handler
    ///* may be invoked during, according to the comment above
    ///* sqlite3PagerSetBusyhandler().
    { let _ = 0; };
    '__b21: loop {
        '__c21: loop {
            rc = pager_lock_db(unsafe { &mut *p_pager_1 }, locktype);
            break '__c21;
        }
        if !(rc == 5 &&
                        unsafe {
                                (unsafe {
                                        (*p_pager_1).x_busy_handler
                                    })(unsafe { (*p_pager_1).p_busy_handler_arg })
                            } != 0) {
            break '__b21;
        }
    }
    return rc;
}

///* This function may only be called while a write-transaction is active in
///* rollback. If the connection is in WAL mode, this call is a no-op.
///* Otherwise, if the connection does not already have an EXCLUSIVE lock on
///* the database file, an attempt is made to obtain one.
///*
///* If the EXCLUSIVE lock is already held or the attempt to obtain it is
///* successful, or the connection is in WAL mode, SQLITE_OK is returned.
///* Otherwise, either SQLITE_BUSY or an SQLITE_IOERR_XXX error code is
///* returned.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_exclusive_lock(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = unsafe { (*p_pager_1).err_code };
    { let _ = 0; };
    if rc == 0 {
        { let _ = 0; };
        { let _ = 0; };
        if 0 ==
                (unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut()) as
                    i32 {
            rc = pager_wait_on_lock(p_pager_1, 4);
        }
    }
    return rc;
}

///* The journal file must be open when this routine is called. A journal
///* header (JOURNAL_HDR_SZ bytes) is written into the journal file at the
///* current location.
///*
///* The format for the journal header is as follows:
///* - 8 bytes: Magic identifying journal format.
///* - 4 bytes: Number of records in journal, or -1 no-sync mode is on.
///* - 4 bytes: Random number used for page hash.
///* - 4 bytes: Initial database page count.
///* - 4 bytes: Sector size used by the process that wrote this journal.
///* - 4 bytes: Database page size.
///*
///* Followed by (JOURNAL_HDR_SZ - 28) bytes of unused space.
#[allow(unused_doc_comments)]
extern "C" fn write_journal_hdr(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let z_header: *mut i8 = unsafe { (*p_pager_1).p_tmp_space };
    /// Temporary space used to build header
    let mut n_header: u32 = unsafe { (*p_pager_1).page_size } as u32;
    /// Size of buffer pointed to by zHeader
    let mut n_write: u32 = 0 as u32;
    /// Bytes of header sector written
    let mut ii: i32 = 0;

    /// Loop counter
    { let _ = 0; };
    if n_header > unsafe { (*p_pager_1).sector_size } {
        n_header = unsafe { (*p_pager_1).sector_size };
    }
    {
        ii = 0;
        '__b22: loop {
            if !(ii < unsafe { (*p_pager_1).n_savepoint }) { break '__b22; }
            '__c22: loop {
                if unsafe {
                            (*unsafe {
                                        (*p_pager_1).a_savepoint.offset(ii as isize)
                                    }).i_hdr_offset
                        } == 0 as i64 {
                    unsafe {
                        (*unsafe {
                                        (*p_pager_1).a_savepoint.offset(ii as isize)
                                    }).i_hdr_offset = unsafe { (*p_pager_1).journal_off }
                    };
                }
                break '__c22;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe {
        (*p_pager_1).journal_hdr =
            {
                unsafe {
                    (*p_pager_1).journal_off =
                        journal_hdr_offset(unsafe { &*p_pager_1 })
                };
                unsafe { (*p_pager_1).journal_off }
            }
    };

    ///* Write the nRec Field - the number of page records that follow this
    ///* journal header. Normally, zero is written to this value at this time.
    ///* After the records are added to the journal (and the journal synced,
    ///* if in full-sync mode), the zero is overwritten with the true number
    ///* of records (see syncJournal()).
    ///*
    ///* A faster alternative is to write 0xFFFFFFFF to the nRec field. When
    ///* reading the journal this value tells SQLite to assume that the
    ///* rest of the journal file contains valid page records. This assumption
    ///* is dangerous, as if a failure occurred whilst writing to the journal
    ///* file it may contain some garbage data. There are two scenarios
    ///* where this risk can be ignored:
    ///*
    ///*   * When the pager is in no-sync mode. Corruption can follow a
    ///*     power failure in this case anyway.
    ///*
    ///*   * When the SQLITE_IOCAP_SAFE_APPEND flag is set. This guarantees
    ///*     that garbage data is never appended to the journal file.
    { let _ = 0; };
    if unsafe { (*p_pager_1).no_sync } != 0 ||
                unsafe { (*p_pager_1).journal_mode } as i32 == 4 ||
            unsafe {
                        sqlite3_os_device_characteristics(unsafe {
                                (*p_pager_1).fd
                            })
                    } & 512 != 0 {
        unsafe {
            memcpy(z_header as *mut (),
                &raw const a_journal_magic[0 as usize] as *const u8 as
                    *const (), core::mem::size_of::<[u8; 8]>() as u64)
        };
        unsafe {
            sqlite3_put4byte(unsafe {
                        &raw mut *z_header.add(core::mem::size_of::<[u8; 8]>() as
                                        usize)
                    } as *mut u8, 4294967295u32)
        };
    } else {
        unsafe {
            memset(z_header as *mut (), 0,
                core::mem::size_of::<[u8; 8]>() as u64 + 4 as u64)
        };
    }
    if unsafe { (*p_pager_1).journal_mode } as i32 != 4 {
        unsafe {
            sqlite3_randomness(core::mem::size_of::<u32>() as i32,
                unsafe { &raw mut (*p_pager_1).cksum_init } as *mut ())
        };
    }
    unsafe {
        sqlite3_put4byte(unsafe {
                    &raw mut *z_header.add((core::mem::size_of::<[u8; 8]>() as
                                            u64 + 4 as u64) as usize)
                } as *mut u8, unsafe { (*p_pager_1).cksum_init })
    };

    /// The initial database size
    unsafe {
        sqlite3_put4byte(unsafe {
                    &raw mut *z_header.add((core::mem::size_of::<[u8; 8]>() as
                                            u64 + 8 as u64) as usize)
                } as *mut u8, unsafe { (*p_pager_1).db_orig_size })
    };

    /// The assumed sector size for this process
    unsafe {
        sqlite3_put4byte(unsafe {
                    &raw mut *z_header.add((core::mem::size_of::<[u8; 8]>() as
                                            u64 + 12 as u64) as usize)
                } as *mut u8, unsafe { (*p_pager_1).sector_size })
    };

    /// The page size
    unsafe {
        sqlite3_put4byte(unsafe {
                    &raw mut *z_header.add((core::mem::size_of::<[u8; 8]>() as
                                            u64 + 16 as u64) as usize)
                } as *mut u8, unsafe { (*p_pager_1).page_size } as u32)
    };

    /// Initializing the tail of the buffer is not necessary.  Everything
    ///* works find if the following memset() is omitted.  But initializing
    ///* the memory prevents valgrind from complaining, so we are willing to
    ///* take the performance hit.
    unsafe {
        memset(unsafe {
                    &raw mut *z_header.add((core::mem::size_of::<[u8; 8]>() as
                                            u64 + 20 as u64) as usize)
                } as *mut (), 0,
            n_header as u64 -
                (core::mem::size_of::<[u8; 8]>() as u64 + 20 as u64))
    };
    {
        n_write = 0 as u32;
        '__b23: loop {
            if !(rc == 0 && n_write < unsafe { (*p_pager_1).sector_size }) {
                break '__b23;
            }
            '__c23: loop {
                rc =
                    unsafe {
                        sqlite3_os_write(unsafe { (*p_pager_1).jfd },
                            z_header as *const (), n_header as i32,
                            unsafe { (*p_pager_1).journal_off })
                    };
                { let _ = 0; };
                unsafe { (*p_pager_1).journal_off += n_header as i64 };
                break '__c23;
            }
            n_write += n_header;
        }
    }
    return rc;
}

///* Sync the journal. In other words, make sure all the pages that have
///* been written to the journal have actually reached the surface of the
///* disk and can be restored in the event of a hot-journal rollback.
///*
///* If the Pager.noSync flag is set, then this function is a no-op.
///* Otherwise, the actions required depend on the journal-mode and the
///* device characteristics of the file-system, as follows:
///*
///*   * If the journal file is an in-memory journal file, no action need
///*     be taken.
///*
///*   * Otherwise, if the device does not support the SAFE_APPEND property,
///*     then the nRec field of the most recently written journal header
///*     is updated to contain the number of journal records that have
///*     been written following it. If the pager is operating in full-sync
///*     mode, then the journal file is synced before this field is updated.
///*
///*   * If the device does not support the SEQUENTIAL property, then
///*     journal file is synced.
///*
///* Or, in pseudo-code:
///*
///*   if( NOT <in-memory journal> ){
///*     if( NOT SAFE_APPEND ){
///*       if( <full-sync mode> ) xSync(<journal file>);
///*       <update nRec field>
///*     }
///*     if( NOT SEQUENTIAL ) xSync(<journal file>);
///*   }
///*
///* If successful, this routine clears the PGHDR_NEED_SYNC flag of every
///* page currently held in memory before returning SQLITE_OK. If an IO
///* error is encountered, then the IO error code is returned to the caller.
#[allow(unused_doc_comments)]
extern "C" fn sync_journal(p_pager_1: *mut Pager, new_hdr_1: i32) -> i32 {
    let mut rc: i32 = 0;

    /// Return code
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    rc = unsafe { sqlite3_pager_exclusive_lock(p_pager_1) };
    if rc != 0 { return rc; }
    if (unsafe { (*p_pager_1).no_sync } == 0) as i32 != 0 {
        { let _ = 0; };
        if unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                    core::ptr::null() &&
                unsafe { (*p_pager_1).journal_mode } as i32 != 4 {
            let i_dc: i32 =
                unsafe {
                        sqlite3_os_device_characteristics(unsafe {
                                (*p_pager_1).fd
                            })
                    } as i32;
            { let _ = 0; };
            if 0 == i_dc & 512 {
                /// This block deals with an obscure problem. If the last connection
                ///* that wrote to this database was operating in persistent-journal
                ///* mode, then the journal file may at this point actually be larger
                ///* than Pager.journalOff bytes. If the next thing in the journal
                ///* file happens to be a journal-header (written as part of the
                ///* previous connection's transaction), and a crash or power-failure
                ///* occurs after nRec is updated but before this connection writes
                ///* anything else to the journal file (or commits/rolls back its
                ///* transaction), then SQLite may become confused when doing the
                ///* hot-journal rollback following recovery. It may roll back all
                ///* of this connections data, then proceed to rolling back the old,
                ///* out-of-date data that follows it. Database corruption.
                ///*
                ///* To work around this, if the journal file does appear to contain
                ///* a valid header following Pager.journalOff, then write a 0x00
                ///* byte to the start of it to prevent it from being recognized.
                ///*
                ///* Variable iNextHdrOffset is set to the offset at which this
                ///* problematic header will occur, if it exists. aMagic is used
                ///* as a temporary buffer to inspect the first couple of bytes of
                ///* the potential journal header.
                let mut i_next_hdr_offset: i64 = 0 as i64;
                let mut a_magic: [u8; 8] = [0; 8];
                let mut z_header: [u8; 12] = [0; 12];
                unsafe {
                    memcpy(&raw mut z_header[0 as usize] as *mut u8 as *mut (),
                        &raw const a_journal_magic[0 as usize] as *const u8 as
                            *const (), core::mem::size_of::<[u8; 8]>() as u64)
                };
                unsafe {
                    sqlite3_put4byte(&raw mut z_header[core::mem::size_of::<[u8; 8]>()
                                        as usize] as *mut u8, unsafe { (*p_pager_1).n_rec } as u32)
                };
                i_next_hdr_offset =
                    journal_hdr_offset(unsafe { &*p_pager_1 });
                rc =
                    unsafe {
                        sqlite3_os_read(unsafe { (*p_pager_1).jfd },
                            &raw mut a_magic[0 as usize] as *mut u8 as *mut (), 8,
                            i_next_hdr_offset)
                    };
                if rc == 0 &&
                        0 ==
                            unsafe {
                                memcmp(&raw mut a_magic[0 as usize] as *mut u8 as *const (),
                                    &raw const a_journal_magic[0 as usize] as *const u8 as
                                        *const (), 8 as u64)
                            } {
                    rc =
                        unsafe {
                            sqlite3_os_write(unsafe { (*p_pager_1).jfd },
                                &raw const zerobyte as *const (), 1, i_next_hdr_offset)
                        };
                }
                if rc != 0 && rc != 10 | 2 << 8 { return rc; }
                if unsafe { (*p_pager_1).full_sync } != 0 && 0 == i_dc & 1024
                    {
                    rc =
                        unsafe {
                            sqlite3_os_sync(unsafe { (*p_pager_1).jfd },
                                unsafe { (*p_pager_1).sync_flags } as i32)
                        };
                    if rc != 0 { return rc; }
                }
                rc =
                    unsafe {
                        sqlite3_os_write(unsafe { (*p_pager_1).jfd },
                            &raw mut z_header[0 as usize] as *mut u8 as *const (),
                            core::mem::size_of::<[u8; 12]>() as i32,
                            unsafe { (*p_pager_1).journal_hdr })
                    };
                if rc != 0 { return rc; }
            }
            if 0 == i_dc & 1024 {
                rc =
                    unsafe {
                        sqlite3_os_sync(unsafe { (*p_pager_1).jfd },
                            unsafe { (*p_pager_1).sync_flags } as i32 |
                                if unsafe { (*p_pager_1).sync_flags } as i32 == 3 {
                                    16
                                } else { 0 })
                    };
                if rc != 0 { return rc; }
            }
            unsafe {
                (*p_pager_1).journal_hdr = unsafe { (*p_pager_1).journal_off }
            };
            if new_hdr_1 != 0 && 0 == i_dc & 512 {
                unsafe { (*p_pager_1).n_rec = 0 };
                rc = write_journal_hdr(p_pager_1);
                if rc != 0 { return rc; }
            }
        } else {
            unsafe {
                (*p_pager_1).journal_hdr = unsafe { (*p_pager_1).journal_off }
            };
        }
    }

    /// Unless the pager is in noSync mode, the journal file was just
    ///* successfully synced. Either way, clear the PGHDR_NEED_SYNC flag on
    ///* all pages.
    unsafe {
        sqlite3_pcache_clear_sync_flags(unsafe { (*p_pager_1).p_p_cache })
    };
    unsafe { (*p_pager_1).e_state = 4 as u8 };
    { let _ = 0; };
    return 0;
}

///* Open a temporary file.
///*
///* Write the file descriptor into *pFile. Return SQLITE_OK on success
///* or some other error code if we fail. The OS will automatically
///* delete the temporary file when it is closed.
///*
///* The flags passed to the VFS layer xOpen() call are those specified
///* by parameter vfsFlags ORed with the following:
///*
///*     SQLITE_OPEN_READWRITE
///*     SQLITE_OPEN_CREATE
///*     SQLITE_OPEN_EXCLUSIVE
///*     SQLITE_OPEN_DELETEONCLOSE
#[allow(unused_doc_comments)]
extern "C" fn pager_opentemp(p_pager_1: &Pager, p_file_1: *mut Sqlite3File,
    mut vfs_flags_1: i32) -> i32 {
    let mut rc: i32 = 0;

    /// Return code
    (vfs_flags_1 |= 2 | 4 | 16 | 8);
    rc =
        unsafe {
            sqlite3_os_open((*p_pager_1).p_vfs, core::ptr::null(), p_file_1,
                vfs_flags_1, core::ptr::null_mut())
        };
    { let _ = 0; };
    return rc;
}

///* The argument is the first in a linked list of dirty pages connected
///* by the PgHdr.pDirty pointer. This function writes each one of the
///* in-memory pages in the list to the database file. The argument may
///* be NULL, representing an empty list. In this case this function is
///* a no-op.
///*
///* The pager must hold at least a RESERVED lock when this function
///* is called. Before writing anything to the database file, this lock
///* is upgraded to an EXCLUSIVE lock. If the lock cannot be obtained,
///* SQLITE_BUSY is returned and no data is written to the database file.
///*
///* If the pager is a temp-file pager and the actual file-system file
///* is not yet open, it is created and opened before any data is
///* written out.
///*
///* Once the lock has been upgraded and, if necessary, the file opened,
///* the pages are written out to the database file in list order. Writing
///* a page is skipped if it meets either of the following criteria:
///*
///*   * The page number is greater than Pager.dbSize, or
///*   * The PGHDR_DONT_WRITE flag is set on the page.
///*
///* If writing out a page causes the database file to grow, Pager.dbFileSize
///* is updated accordingly. If page 1 is written out, then the value cached
///* in Pager.dbFileVers[] is updated to match the new value stored in
///* the database file.
///*
///* If everything is successful, SQLITE_OK is returned. If an IO error
///* occurs, an IO error code is returned. Or, if the EXCLUSIVE lock cannot
///* be obtained, SQLITE_BUSY is returned.
#[allow(unused_doc_comments)]
extern "C" fn pager_write_pagelist(p_pager_1: *mut Pager,
    mut p_list_1: *mut PgHdr) -> i32 {
    let mut rc: i32 = 0;

    /// Return code
    /// This function is only called for rollback pagers in WRITER_DBMOD state.
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if !(unsafe { (*unsafe { (*p_pager_1).fd }).p_methods } !=
                        core::ptr::null()) as i32 != 0 {
        { let _ = 0; };
        rc =
            pager_opentemp(unsafe { &*p_pager_1 }, unsafe { (*p_pager_1).fd },
                unsafe { (*p_pager_1).vfs_flags } as i32);
    }

    /// Before the first write, give the VFS a hint of what the final
    ///* file size will be.
    { let _ = 0; };
    if rc == 0 &&
                unsafe { (*p_pager_1).db_hint_size } <
                    unsafe { (*p_pager_1).db_size } &&
            (!(unsafe { (*p_list_1).p_dirty }).is_null() ||
                unsafe { (*p_list_1).pgno } >
                    unsafe { (*p_pager_1).db_hint_size }) {
        let mut sz_file: Sqlite3Int64 =
            unsafe { (*p_pager_1).page_size } *
                unsafe { (*p_pager_1).db_size } as Sqlite3Int64;
        unsafe {
            sqlite3_os_file_control_hint(unsafe { (*p_pager_1).fd }, 5,
                &raw mut sz_file as *mut ())
        };
        unsafe {
            (*p_pager_1).db_hint_size = unsafe { (*p_pager_1).db_size }
        };
    }
    while rc == 0 && !(p_list_1).is_null() {
        let pgno: Pgno = unsafe { (*p_list_1).pgno };
        if pgno <= unsafe { (*p_pager_1).db_size } &&
                0 == unsafe { (*p_list_1).flags } as i32 & 16 {
            let offset: i64 =
                (pgno - 1 as Pgno) as i64 *
                    unsafe { (*p_pager_1).page_size } as i64;
            /// Offset to write
            let mut p_data: *mut i8 = core::ptr::null_mut();

            /// Data to write
            { let _ = 0; };
            if unsafe { (*p_list_1).pgno } == 1 as u32 {
                pager_write_changecounter(p_list_1);
            }
            p_data = unsafe { (*p_list_1).p_data } as *mut i8;

            /// Write out the page data.
            (rc =
                unsafe {
                    sqlite3_os_write(unsafe { (*p_pager_1).fd },
                        p_data as *const (),
                        unsafe { (*p_pager_1).page_size } as i32, offset)
                });
            if pgno == 1 as u32 {
                unsafe {
                    memcpy(unsafe { &raw mut (*p_pager_1).db_file_vers } as
                            *mut (),
                        unsafe { &raw mut *p_data.offset(24 as isize) } as
                            *const (), core::mem::size_of::<[i8; 16]>() as u64)
                };
            }
            if pgno > unsafe { (*p_pager_1).db_file_size } {
                unsafe { (*p_pager_1).db_file_size = pgno };
            }
            {
                let __p = unsafe { &mut (*p_pager_1).a_stat[2 as usize] };
                let __t = *__p;
                *__p += 1;
                __t
            };

            /// Update any backup objects copying the contents of this pager.
            unsafe {
                sqlite3_backup_update(unsafe { (*p_pager_1).p_backup }, pgno,
                    unsafe { (*p_list_1).p_data } as *mut u8 as *const u8)
            };
        } else {}
        p_list_1 = unsafe { (*p_list_1).p_dirty };
    }
    return rc;
}

///* This function is called by the pcache layer when it has reached some
///* soft memory limit. The first argument is a pointer to a Pager object
///* (cast as a void*). The pager is always 'purgeable' (not an in-memory
///* database). The second argument is a reference to a page that is
///* currently dirty but has no outstanding references. The page
///* is always associated with the Pager object passed as the first
///* argument.
///*
///* The job of this function is to make pPg clean by writing its contents
///* out to the database file, if possible. This may involve syncing the
///* journal file.
///*
///* If successful, sqlite3PcacheMakeClean() is called on the page and
///* SQLITE_OK returned. If an IO error occurs while trying to make the
///* page clean, the IO error code is returned. If the page cannot be
///* made clean for some other reason, but no error occurs, then SQLITE_OK
///* is returned by sqlite3PcacheMakeClean() is not called.
#[allow(unused_doc_comments)]
extern "C" fn pager_stress(p: *mut (), p_pg_1: *mut PgHdr) -> i32 {
    let p_pager: *mut Pager = p as *mut Pager;
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_pager).err_code } != 0 { return 0; }
    if unsafe { (*p_pager).do_not_spill } != 0 &&
            (unsafe { (*p_pager).do_not_spill } as i32 & (2 | 1) != 0 ||
                unsafe { (*p_pg_1).flags } as i32 & 8 != 0) {
        return 0;
    }
    {
        let __p = unsafe { &mut (*p_pager).a_stat[3 as usize] };
        let __t = *__p;
        *__p += 1;
        __t
    };
    unsafe { (*p_pg_1).p_dirty = core::ptr::null_mut() };
    if unsafe { (*p_pager).p_wal } != core::ptr::null_mut() {

        /// Write a single frame for this page to the log.
        (rc = subjournal_page_if_required(p_pg_1));
        if rc == 0 {
            rc =
                pager_wal_frames(unsafe { &mut *p_pager }, p_pg_1, 0 as Pgno,
                    0);
        }
    } else {
        if unsafe { (*p_pg_1).flags } as i32 & 8 != 0 ||
                unsafe { (*p_pager).e_state } as i32 == 3 {
            rc = sync_journal(p_pager, 1);
        }
        if rc == 0 {
            { let _ = 0; };
            rc = pager_write_pagelist(p_pager, p_pg_1);
        }
    }
    if rc == 0 { unsafe { sqlite3_pcache_make_clean(p_pg_1) }; }
    return pager_error(p_pager, rc);
}

///* Adjust settings of the pager to those specified in the pgFlags parameter.
///*
///* The "level" in pgFlags & PAGER_SYNCHRONOUS_MASK sets the robustness
///* of the database to damage due to OS crashes or power failures by
///* changing the number of syncs()s when writing the journals.
///* There are four levels:
///*
///*    OFF       sqlite3OsSync() is never called.  This is the default
///*              for temporary and transient files.
///*
///*    NORMAL    The journal is synced once before writes begin on the
///*              database.  This is normally adequate protection, but
///*              it is theoretically possible, though very unlikely,
///*              that an inopertune power failure could leave the journal
///*              in a state which would cause damage to the database
///*              when it is rolled back.
///*
///*    FULL      The journal is synced twice before writes begin on the
///*              database (with some additional information - the nRec field
///*              of the journal header - being written in between the two
///*              syncs).  If we assume that writing a
///*              single disk sector is atomic, then this mode provides
///*              assurance that the journal will not be corrupted to the
///*              point of causing damage to the database during rollback.
///*
///*    EXTRA     This is like FULL except that is also syncs the directory
///*              that contains the rollback journal after the rollback
///*              journal is unlinked.
///*
///* The above is for a rollback-journal mode.  For WAL mode, OFF continues
///* to mean that no syncs ever occur.  NORMAL means that the WAL is synced
///* prior to the start of checkpoint and that the database file is synced
///* at the conclusion of the checkpoint if the entire content of the WAL
///* was written back into the database.  But no sync operations occur for
///* an ordinary commit in NORMAL mode with WAL.  FULL means that the WAL
///* file is synced following each commit operation, in addition to the
///* syncs associated with NORMAL.  There is no difference between FULL
///* and EXTRA for WAL mode.
///*
///* Do not confuse synchronous=FULL with SQLITE_SYNC_FULL.  The
///* SQLITE_SYNC_FULL macro means to use the MacOSX-style full-fsync
///* using fcntl(F_FULLFSYNC).  SQLITE_SYNC_NORMAL means to do an
///* ordinary fsync() call.  There is no difference between SQLITE_SYNC_FULL
///* and SQLITE_SYNC_NORMAL on platforms other than MacOSX.  But the
///* synchronous=FULL versus synchronous=NORMAL setting determines when
///* the xSync primitive is called and is relevant to all platforms.
///*
///* Numeric values associated with these states are OFF==1, NORMAL=2,
///* and FULL=3.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_set_flags(p_pager_1: &mut Pager,
    pg_flags_1: u32) -> () {
    let level: u32 = pg_flags_1 & 7 as u32;
    if (*p_pager_1).temp_file != 0 || level == 1 as u32 {
        (*p_pager_1).no_sync = 1 as u8;
        (*p_pager_1).full_sync = 0 as u8;
        (*p_pager_1).extra_sync = 0 as u8;
    } else {
        (*p_pager_1).no_sync = 0 as u8;
        (*p_pager_1).full_sync = if level >= 3 as u32 { 1 } else { 0 } as u8;
        if level == 4 as u32 {
            (*p_pager_1).extra_sync = 1 as u8;
        } else { (*p_pager_1).extra_sync = 0 as u8; }
    }
    if (*p_pager_1).no_sync != 0 {
        (*p_pager_1).sync_flags = 0 as u8;
    } else if pg_flags_1 & 8 as u32 != 0 {
        (*p_pager_1).sync_flags = 3 as u8;
    } else { (*p_pager_1).sync_flags = 2 as u8; }
    (*p_pager_1).wal_sync_flags =
        (((*p_pager_1).sync_flags as i32) << 2) as u8;
    if (*p_pager_1).full_sync != 0 {
        (*p_pager_1).wal_sync_flags |= (*p_pager_1).sync_flags as i32 as u8;
    }
    if pg_flags_1 & 16 as u32 != 0 && ((*p_pager_1).no_sync == 0) as i32 != 0
        {
        (*p_pager_1).wal_sync_flags |= (3 << 2) as u8;
    }
    if pg_flags_1 & 32 as u32 != 0 {
        (*p_pager_1).do_not_spill &= !1 as u8;
    } else { (*p_pager_1).do_not_spill |= 1 as u8; }
}

/// Open and close a Pager connection.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_open(p_vfs_1: *mut Sqlite3Vfs,
    pp_pager_1: &mut *mut Pager, mut z_filename_1: *const i8,
    mut n_extra_1: i32, flags: i32, mut vfs_flags_1: i32,
    x_reinit_1: Option<unsafe extern "C" fn(*mut PgHdr) -> ()>) -> i32 {
    let mut p_ptr: *mut u8 = core::ptr::null_mut();
    let mut p_pager: *mut Pager = core::ptr::null_mut();
    /// Pager object to allocate and return
    let mut rc: i32 = 0;
    /// Return code
    let mut temp_file: i32 = 0;
    /// True for temp files (incl. in-memory files)
    let mut mem_db: i32 = 0;
    /// True if this is an in-memory file
    let mut mem_jm: i32 = 0;
    /// Memory journal mode
    let mut read_only: i32 = 0;
    /// True if this is a read-only file
    let mut journal_file_size: i32 = 0;
    /// Bytes to allocate for each journal fd
    let mut z_pathname: *mut i8 = core::ptr::null_mut();
    /// Full path to database file
    let mut n_pathname: i32 = 0;
    /// Number of bytes in zPathname
    let mut use_journal: i32 = 0;
    /// False to omit journal
    let mut pcache_size: i32 = 0;
    /// Bytes to allocate for PCache
    let mut sz_page_dflt: u32 = 0 as u32;
    /// Default page size
    let mut z_uri: *const i8 = core::ptr::null();
    /// URI args to copy
    let mut n_uri_byte: i32 = 0;
    /// Number of bytes of URI args at *zUri
    /// Figure out how much space is required for each journal file-handle
    ///* (there are two of them, the main journal and the sub-journal).
    /// Set the output variable to NULL in case an error occurs.
    /// Compute and store the full pathname in an allocated buffer pointed
    ///* to by zPathname, length nPathname. Or, if this is a temporary file,
    ///* leave both nPathname and zPathname set to 0.
    let mut z: *const i8 = core::ptr::null();
    /// Make sure initialized even if FullPathname() fails
    /// This branch is taken when the journal path required by
    ///* the database being opened will be more than pVfs->mxPathname
    ///* bytes in length. This means the database cannot be opened,
    ///* as it will not be possible to open the journal file or even
    ///* check for a hot-journal before reading.
    /// Allocate memory for the Pager structure, PCache object, the
    ///* three file descriptors, the database file name and the journal
    ///* file name. The layout in memory is as follows:
    ///*
    ///*     Pager object                    (sizeof(Pager) bytes)
    ///*     PCache object                   (sqlite3PcacheSize() bytes)
    ///*     Database file handle            (pVfs->szOsFile bytes)
    ///*     Sub-journal file handle         (journalFileSize bytes)
    ///*     Main journal file handle        (journalFileSize bytes)
    ///*     Ptr back to the Pager           (sizeof(Pager*) bytes)
    ///*     \0\0\0\0 database prefix        (4 bytes)
    ///*     Database file name              (nPathname+1 bytes)
    ///*     URI query parameters            (nUriByte bytes)
    ///*     Journal filename                (nPathname+8+1 bytes)
    ///*     WAL filename                    (nPathname+4+1 bytes)
    ///*     \0\0\0 terminator               (3 bytes)
    ///*
    ///* Some 3rd-party software, over which we have no control, depends on
    ///* the specific order of the filenames and the \0 separators between them
    ///* so that it can (for example) find the database filename given the WAL
    ///* filename without using the sqlite3_filename_database() API.  This is a
    ///* misuse of SQLite and a bug in the 3rd-party software, but the 3rd-party
    ///* software is in widespread use, so we try to avoid changing the filename
    ///* order and formatting if possible.  In particular, the details of the
    ///* filename format expected by 3rd-party software should be as follows:
    ///*
    ///*   - Main Database Path
    ///*   - \0
    ///*   - Multiple URI components consisting of:
    ///*     - Key
    ///*     - \0
    ///*     - Value
    ///*     - \0
    ///*   - \0
    ///*   - Journal Path
    ///*   - \0
    ///*   - WAL Path (zWALName)
    ///*   - \0
    ///*
    ///* The sqlite3_create_filename() interface and the databaseFilename() utility
    ///* that is used by sqlite3_filename_database() and kin also depend on the
    ///* specific formatting and order of the various filenames, so if the format
    ///* changes here, be sure to change it there as well.
    /// Pager structure
    /// PCache object
    /// The main db file
    /// The two journal files
    /// Space to hold a pointer
    /// Database prefix
    /// database filename
    /// query parameters
    /// Journal filename
    /// WAL filename
    /// Terminator
    /// Fill in the Pager.zFilename and pPager.zQueryParam fields
    /// Skip zero prefix
    /// Fill in Pager.zJournal
    /// Fill in Pager.zWal
    /// Suppress warning about unused pPtr value
    /// Open the pager file.
    let mut fout: i32 = 0;
    /// VFS flags returned by xOpen()
    /// If the file was successfully opened for read/write access,
    ///* choose a default page size in case we have to create the
    ///* database file. The default page size is the maximum of:
    ///*
    ///*    + SQLITE_DEFAULT_PAGE_SIZE,
    ///*    + The value returned by sqlite3OsSectorSize()
    ///*    + The largest page size that can be written atomically.
    let mut i_dc: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s26:
            {
            match __state {
                0 => { __state = 3; }
                2 => { temp_file = 1; __state = 120; }
                3 => { p_pager = core::ptr::null_mut(); __state = 4; }
                4 => { rc = 0; __state = 5; }
                5 => { temp_file = 0; __state = 6; }
                6 => { mem_db = 0; __state = 7; }
                7 => { mem_jm = 0; __state = 8; }
                8 => { read_only = 0; __state = 9; }
                9 => { __state = 10; }
                10 => { z_pathname = core::ptr::null_mut(); __state = 11; }
                11 => { n_pathname = 0; __state = 12; }
                12 => { use_journal = (flags & 1 == 0) as i32; __state = 13; }
                13 => {
                    pcache_size = unsafe { sqlite3_pcache_size() };
                    __state = 14;
                }
                14 => { sz_page_dflt = 4096 as u32; __state = 15; }
                15 => { z_uri = core::ptr::null(); __state = 16; }
                16 => { n_uri_byte = 1; __state = 17; }
                17 => {
                    journal_file_size =
                        unsafe { sqlite3_journal_size(p_vfs_1) } + 7 & !7;
                    __state = 18;
                }
                18 => { *pp_pager_1 = core::ptr::null_mut(); __state = 19; }
                19 => {
                    if flags & 2 != 0 { __state = 21; } else { __state = 20; }
                }
                20 => {
                    if !(z_filename_1).is_null() &&
                            unsafe { *z_filename_1.offset(0 as isize) } != 0 {
                        __state = 29;
                    } else { __state = 28; }
                }
                21 => { mem_db = 1; __state = 22; }
                22 => {
                    if !(z_filename_1).is_null() &&
                            unsafe { *z_filename_1.offset(0 as isize) } != 0 {
                        __state = 23;
                    } else { __state = 20; }
                }
                23 => {
                    z_pathname =
                        unsafe {
                            sqlite3_db_str_dup(core::ptr::null_mut(), z_filename_1)
                        };
                    __state = 24;
                }
                24 => {
                    if z_pathname == core::ptr::null_mut() {
                        __state = 26;
                    } else { __state = 25; }
                }
                25 => {
                    n_pathname =
                        unsafe { sqlite3_strlen30(z_pathname as *const i8) };
                    __state = 27;
                }
                26 => { return 7; }
                27 => { z_filename_1 = core::ptr::null(); __state = 20; }
                28 => { { let _ = 0; }; __state = 53; }
                29 => { __state = 30; }
                30 => {
                    n_pathname = unsafe { (*p_vfs_1).mx_pathname } + 1;
                    __state = 31;
                }
                31 => {
                    z_pathname =
                        unsafe {
                                sqlite3_db_malloc_raw(core::ptr::null_mut(),
                                    (2 as i64 * n_pathname as i64) as u64)
                            } as *mut i8;
                    __state = 32;
                }
                32 => {
                    if z_pathname == core::ptr::null_mut() {
                        __state = 34;
                    } else { __state = 33; }
                }
                33 => {
                    unsafe { *z_pathname.offset(0 as isize) = 0 as i8 };
                    __state = 35;
                }
                34 => { return 7; }
                35 => {
                    rc =
                        unsafe {
                            sqlite3_os_full_pathname(p_vfs_1, z_filename_1, n_pathname,
                                z_pathname)
                        };
                    __state = 36;
                }
                36 => { if rc != 0 { __state = 38; } else { __state = 37; } }
                37 => {
                    n_pathname =
                        unsafe { sqlite3_strlen30(z_pathname as *const i8) };
                    __state = 42;
                }
                38 => {
                    if rc == 0 | 2 << 8 { __state = 39; } else { __state = 37; }
                }
                39 => {
                    if vfs_flags_1 & 16777216 != 0 {
                        __state = 40;
                    } else { __state = 41; }
                }
                40 => { rc = 14 | 6 << 8; __state = 37; }
                41 => { rc = 0; __state = 37; }
                42 => {
                    z =
                        {
                            z_uri =
                                unsafe {
                                    z_filename_1.offset((unsafe {
                                                    sqlite3_strlen30(z_filename_1)
                                                } + 1) as isize)
                                };
                            z_uri
                        };
                    __state = 43;
                }
                43 => {
                    if unsafe { *z } != 0 {
                        __state = 45;
                    } else { __state = 44; }
                }
                44 => {
                    n_uri_byte =
                        unsafe {
                                    unsafe { z.offset(1 as isize).offset_from(z_uri) }
                                } as i64 as i32;
                    __state = 47;
                }
                45 => {
                    {
                        let __n = unsafe { strlen(z) } + 1 as u64;
                        let __p = &mut z;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    __state = 46;
                }
                46 => {
                    {
                        let __n = unsafe { strlen(z) } + 1 as u64;
                        let __p = &mut z;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    __state = 43;
                }
                47 => { { let _ = 0; }; __state = 48; }
                48 => {
                    if rc == 0 &&
                            n_pathname + 8 > unsafe { (*p_vfs_1).mx_pathname } {
                        __state = 50;
                    } else { __state = 49; }
                }
                49 => { if rc != 0 { __state = 51; } else { __state = 28; } }
                50 => {
                    rc = unsafe { sqlite3_cantopen_error(4871) };
                    __state = 49;
                }
                51 => {
                    unsafe {
                        sqlite3_db_free(core::ptr::null_mut(),
                            z_pathname as *mut ())
                    };
                    __state = 52;
                }
                52 => { return rc; }
                53 => {
                    p_ptr =
                        unsafe {
                                sqlite3_malloc_zero((core::mem::size_of::<Pager>() as u64 +
                                                                                                        7 as u64 & !7 as u64) + (pcache_size + 7 & !7) as u64 +
                                                                                            (unsafe { (*p_vfs_1).sz_os_file } + 7 & !7) as u64 +
                                                                                        journal_file_size as u64 * 2 as u64 + 8 as u64 + 4 as u64 +
                                                                            n_pathname as u64 + 1 as u64 + n_uri_byte as u64 +
                                                                n_pathname as u64 + 8 as u64 + 1 as u64 + n_pathname as u64
                                                + 4 as u64 + 1 as u64 + 3 as u64)
                            } as *mut u8;
                    __state = 54;
                }
                54 => { { let _ = 0; }; __state = 55; }
                55 => {
                    if (p_ptr).is_null() as i32 != 0 {
                        __state = 57;
                    } else { __state = 56; }
                }
                56 => { p_pager = p_ptr as *mut Pager; __state = 59; }
                57 => {
                    unsafe {
                        sqlite3_db_free(core::ptr::null_mut(),
                            z_pathname as *mut ())
                    };
                    __state = 58;
                }
                58 => { return 7; }
                59 => {
                    {
                        let __n =
                            core::mem::size_of::<Pager>() as u64 + 7 as u64 & !7 as u64;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).add(__n as usize) };
                    };
                    __state = 60;
                }
                60 => {
                    unsafe { (*p_pager).p_p_cache = p_ptr as *mut PCache };
                    __state = 61;
                }
                61 => {
                    {
                        let __n = pcache_size + 7 & !7;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 62;
                }
                62 => {
                    unsafe { (*p_pager).fd = p_ptr as *mut Sqlite3File };
                    __state = 63;
                }
                63 => {
                    {
                        let __n = unsafe { (*p_vfs_1).sz_os_file } + 7 & !7;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 64;
                }
                64 => {
                    unsafe { (*p_pager).sjfd = p_ptr as *mut Sqlite3File };
                    __state = 65;
                }
                65 => {
                    {
                        let __n = journal_file_size;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 66;
                }
                66 => {
                    unsafe { (*p_pager).jfd = p_ptr as *mut Sqlite3File };
                    __state = 67;
                }
                67 => {
                    {
                        let __n = journal_file_size;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 68;
                }
                68 => { { let _ = 0; }; __state = 69; }
                69 => {
                    unsafe {
                        memcpy(p_ptr as *mut (), &raw mut p_pager as *const (),
                            8 as u64)
                    };
                    __state = 70;
                }
                70 => {
                    {
                        let __n = 8;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 71;
                }
                71 => {
                    {
                        let __n = 4;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 72;
                }
                72 => {
                    unsafe { (*p_pager).z_filename = p_ptr as *mut i8 };
                    __state = 73;
                }
                73 => {
                    if n_pathname > 0 { __state = 75; } else { __state = 74; }
                }
                74 => {
                    if n_pathname > 0 { __state = 82; } else { __state = 83; }
                }
                75 => {
                    unsafe {
                        memcpy(p_ptr as *mut (), z_pathname as *const (),
                            n_pathname as u64)
                    };
                    __state = 76;
                }
                76 => {
                    {
                        let __n = n_pathname + 1;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 77;
                }
                77 => {
                    if !(z_uri).is_null() {
                        __state = 78;
                    } else { __state = 79; }
                }
                78 => {
                    unsafe {
                        memcpy(p_ptr as *mut (), z_uri as *const (),
                            n_uri_byte as u64)
                    };
                    __state = 80;
                }
                79 => {
                    {
                        let __p = &mut p_ptr;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    __state = 74;
                }
                80 => {
                    {
                        let __n = n_uri_byte;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 74;
                }
                81 => {
                    if n_pathname > 0 { __state = 89; } else { __state = 90; }
                }
                82 => {
                    unsafe { (*p_pager).z_journal = p_ptr as *mut i8 };
                    __state = 84;
                }
                83 => {
                    unsafe { (*p_pager).z_journal = core::ptr::null_mut() };
                    __state = 81;
                }
                84 => {
                    unsafe {
                        memcpy(p_ptr as *mut (), z_pathname as *const (),
                            n_pathname as u64)
                    };
                    __state = 85;
                }
                85 => {
                    {
                        let __n = n_pathname;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 86;
                }
                86 => {
                    unsafe {
                        memcpy(p_ptr as *mut (),
                            c"-journal".as_ptr() as *mut i8 as *const (), 8 as u64)
                    };
                    __state = 87;
                }
                87 => {
                    {
                        let __n = 8 + 1;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 81;
                }
                88 => { { let _ = p_ptr; }; __state = 95; }
                89 => {
                    unsafe { (*p_pager).z_wal = p_ptr as *mut i8 };
                    __state = 91;
                }
                90 => {
                    unsafe { (*p_pager).z_wal = core::ptr::null_mut() };
                    __state = 88;
                }
                91 => {
                    unsafe {
                        memcpy(p_ptr as *mut (), z_pathname as *const (),
                            n_pathname as u64)
                    };
                    __state = 92;
                }
                92 => {
                    {
                        let __n = n_pathname;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 93;
                }
                93 => {
                    unsafe {
                        memcpy(p_ptr as *mut (),
                            c"-wal".as_ptr() as *mut i8 as *const (), 4 as u64)
                    };
                    __state = 94;
                }
                94 => {
                    {
                        let __n = 4 + 1;
                        let __p = &mut p_ptr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    __state = 88;
                }
                95 => {
                    if n_pathname != 0 { __state = 97; } else { __state = 96; }
                }
                96 => { unsafe { (*p_pager).p_vfs = p_vfs_1 }; __state = 98; }
                97 => {
                    unsafe {
                        sqlite3_db_free(core::ptr::null_mut(),
                            z_pathname as *mut ())
                    };
                    __state = 96;
                }
                98 => {
                    unsafe { (*p_pager).vfs_flags = vfs_flags_1 as u32 };
                    __state = 99;
                }
                99 => {
                    if !(z_filename_1).is_null() &&
                            unsafe { *z_filename_1.offset(0 as isize) } != 0 {
                        __state = 101;
                    } else { __state = 102; }
                }
                100 => {
                    if rc == 0 { __state = 125; } else { __state = 124; }
                }
                101 => { fout = 0; __state = 103; }
                102 => { __state = 2; }
                103 => {
                    rc =
                        unsafe {
                            sqlite3_os_open(p_vfs_1,
                                unsafe { (*p_pager).z_filename } as *const i8,
                                unsafe { (*p_pager).fd }, vfs_flags_1, &mut fout)
                        };
                    __state = 104;
                }
                104 => { { let _ = 0; }; __state = 105; }
                105 => {
                    unsafe {
                        (*p_pager).mem_vfs =
                            { mem_jm = (fout & 128 != 0) as i32; mem_jm } as u8
                    };
                    __state = 106;
                }
                106 => { read_only = (fout & 1 != 0) as i32; __state = 107; }
                107 => {
                    if rc == 0 { __state = 108; } else { __state = 100; }
                }
                108 => {
                    i_dc =
                        unsafe {
                            sqlite3_os_device_characteristics(unsafe { (*p_pager).fd })
                        };
                    __state = 109;
                }
                109 => {
                    if (read_only == 0) as i32 != 0 {
                        __state = 111;
                    } else { __state = 110; }
                }
                110 => {
                    unsafe {
                        (*p_pager).no_lock =
                            unsafe {
                                    sqlite3_uri_boolean(unsafe { (*p_pager).z_filename } as
                                            Sqlite3Filename, c"nolock".as_ptr() as *mut i8 as *const i8,
                                        0)
                                } as u8
                    };
                    __state = 117;
                }
                111 => {
                    set_sector_size(unsafe { &mut *p_pager });
                    __state = 112;
                }
                112 => { { let _ = 0; }; __state = 113; }
                113 => {
                    if sz_page_dflt < unsafe { (*p_pager).sector_size } {
                        __state = 114;
                    } else { __state = 110; }
                }
                114 => {
                    if unsafe { (*p_pager).sector_size } > 8192 as u32 {
                        __state = 115;
                    } else { __state = 116; }
                }
                115 => { sz_page_dflt = 8192 as u32; __state = 110; }
                116 => {
                    sz_page_dflt = unsafe { (*p_pager).sector_size } as u32;
                    __state = 110;
                }
                117 => {
                    if i_dc & 8192 != 0 ||
                            unsafe {
                                    sqlite3_uri_boolean(unsafe { (*p_pager).z_filename } as
                                            Sqlite3Filename,
                                        c"immutable".as_ptr() as *mut i8 as *const i8, 0)
                                } != 0 {
                        __state = 118;
                    } else { __state = 100; }
                }
                118 => { vfs_flags_1 |= 1; __state = 119; }
                119 => { __state = 2; }
                120 => {
                    unsafe { (*p_pager).e_state = 1 as u8 };
                    __state = 121;
                }
                121 => {
                    unsafe { (*p_pager).e_lock = 4 as u8 };
                    __state = 122;
                }
                122 => {
                    unsafe { (*p_pager).no_lock = 1 as u8 };
                    __state = 123;
                }
                123 => { read_only = vfs_flags_1 & 1; __state = 100; }
                124 => {
                    if rc == 0 { __state = 129; } else { __state = 128; }
                }
                125 => { { let _ = 0; }; __state = 126; }
                126 => {
                    rc =
                        sqlite3_pager_set_pagesize(p_pager, &mut sz_page_dflt, -1);
                    __state = 127;
                }
                127 => { __state = 124; }
                128 => {
                    if rc != 0 { __state = 133; } else { __state = 132; }
                }
                129 => { n_extra_1 = n_extra_1 + 7 & !7; __state = 130; }
                130 => { { let _ = 0; }; __state = 131; }
                131 => {
                    rc =
                        unsafe {
                            sqlite3_pcache_open(sz_page_dflt as i32, n_extra_1,
                                (mem_db == 0) as i32 as i32,
                                Some(if (mem_db == 0) as i32 != 0 {
                                        pager_stress
                                    } else {
                                        unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut (), *mut PgHdr)
                                                        -> i32>(0 as *const ())
                                        }
                                    }), p_pager as *mut (), unsafe { (*p_pager).p_p_cache })
                        };
                    __state = 128;
                }
                132 => { __state = 137; }
                133 => {
                    unsafe { sqlite3_os_close(unsafe { (*p_pager).fd }) };
                    __state = 134;
                }
                134 => {
                    unsafe {
                        sqlite3_page_free(unsafe { (*p_pager).p_tmp_space } as
                                *mut ())
                    };
                    __state = 135;
                }
                135 => {
                    unsafe { sqlite3_free(p_pager as *mut ()) };
                    __state = 136;
                }
                136 => { return rc; }
                137 => {
                    unsafe { (*p_pager).use_journal = use_journal as u8 };
                    __state = 138;
                }
                138 => {
                    unsafe { (*p_pager).mx_pgno = 4294967294u32 };
                    __state = 139;
                }
                139 => {
                    unsafe { (*p_pager).temp_file = temp_file as u8 };
                    __state = 140;
                }
                140 => { { let _ = 0; }; __state = 141; }
                141 => { { let _ = 0; }; __state = 142; }
                142 => {
                    unsafe { (*p_pager).exclusive_mode = temp_file as u8 };
                    __state = 143;
                }
                143 => {
                    unsafe {
                        (*p_pager).change_count_done =
                            unsafe { (*p_pager).temp_file }
                    };
                    __state = 144;
                }
                144 => {
                    unsafe { (*p_pager).mem_db = mem_db as u8 };
                    __state = 145;
                }
                145 => {
                    unsafe { (*p_pager).read_only = read_only as u8 };
                    __state = 146;
                }
                146 => { { let _ = 0; }; __state = 147; }
                147 => {
                    sqlite3_pager_set_flags(unsafe { &mut *p_pager },
                        (2 + 1 | 32) as u32);
                    __state = 148;
                }
                148 => {
                    unsafe { (*p_pager).n_extra = n_extra_1 as u16 };
                    __state = 149;
                }
                149 => {
                    unsafe { (*p_pager).journal_size_limit = -1 as i64 };
                    __state = 150;
                }
                150 => { { let _ = 0; }; __state = 151; }
                151 => {
                    set_sector_size(unsafe { &mut *p_pager });
                    __state = 152;
                }
                152 => {
                    if (use_journal == 0) as i32 != 0 {
                        __state = 154;
                    } else { __state = 155; }
                }
                153 => {
                    unsafe { (*p_pager).x_reiniter = x_reinit_1 };
                    __state = 157;
                }
                154 => {
                    unsafe { (*p_pager).journal_mode = 2 as u8 };
                    __state = 153;
                }
                155 => {
                    if mem_db != 0 || mem_jm != 0 {
                        __state = 156;
                    } else { __state = 153; }
                }
                156 => {
                    unsafe { (*p_pager).journal_mode = 4 as u8 };
                    __state = 153;
                }
                157 => {
                    set_getter_method(unsafe { &mut *p_pager });
                    __state = 158;
                }
                158 => { *pp_pager_1 = p_pager; __state = 159; }
                159 => { return 0; }
                _ => {}
            }
        }
    }

    /// Pager object to allocate and return
    /// Return code
    /// True for temp files (incl. in-memory files)
    /// True if this is an in-memory file
    /// Memory journal mode
    /// True if this is a read-only file
    /// Bytes to allocate for each journal fd
    /// Full path to database file
    /// Number of bytes in zPathname
    /// False to omit journal
    /// Bytes to allocate for PCache
    /// Default page size
    /// URI args to copy
    /// Number of bytes of URI args at *zUri
    /// Figure out how much space is required for each journal file-handle
    ///* (there are two of them, the main journal and the sub-journal).
    /// Set the output variable to NULL in case an error occurs.
    /// Compute and store the full pathname in an allocated buffer pointed
    ///* to by zPathname, length nPathname. Or, if this is a temporary file,
    ///* leave both nPathname and zPathname set to 0.
    /// Make sure initialized even if FullPathname() fails
    /// This branch is taken when the journal path required by
    ///* the database being opened will be more than pVfs->mxPathname
    ///* bytes in length. This means the database cannot be opened,
    ///* as it will not be possible to open the journal file or even
    ///* check for a hot-journal before reading.
    /// Allocate memory for the Pager structure, PCache object, the
    ///* three file descriptors, the database file name and the journal
    ///* file name. The layout in memory is as follows:
    ///*
    ///*     Pager object                    (sizeof(Pager) bytes)
    ///*     PCache object                   (sqlite3PcacheSize() bytes)
    ///*     Database file handle            (pVfs->szOsFile bytes)
    ///*     Sub-journal file handle         (journalFileSize bytes)
    ///*     Main journal file handle        (journalFileSize bytes)
    ///*     Ptr back to the Pager           (sizeof(Pager*) bytes)
    ///*     \0\0\0\0 database prefix        (4 bytes)
    ///*     Database file name              (nPathname+1 bytes)
    ///*     URI query parameters            (nUriByte bytes)
    ///*     Journal filename                (nPathname+8+1 bytes)
    ///*     WAL filename                    (nPathname+4+1 bytes)
    ///*     \0\0\0 terminator               (3 bytes)
    ///*
    ///* Some 3rd-party software, over which we have no control, depends on
    ///* the specific order of the filenames and the \0 separators between them
    ///* so that it can (for example) find the database filename given the WAL
    ///* filename without using the sqlite3_filename_database() API.  This is a
    ///* misuse of SQLite and a bug in the 3rd-party software, but the 3rd-party
    ///* software is in widespread use, so we try to avoid changing the filename
    ///* order and formatting if possible.  In particular, the details of the
    ///* filename format expected by 3rd-party software should be as follows:
    ///*
    ///*   - Main Database Path
    ///*   - \0
    ///*   - Multiple URI components consisting of:
    ///*     - Key
    ///*     - \0
    ///*     - Value
    ///*     - \0
    ///*   - \0
    ///*   - Journal Path
    ///*   - \0
    ///*   - WAL Path (zWALName)
    ///*   - \0
    ///*
    ///* The sqlite3_create_filename() interface and the databaseFilename() utility
    ///* that is used by sqlite3_filename_database() and kin also depend on the
    ///* specific formatting and order of the various filenames, so if the format
    ///* changes here, be sure to change it there as well.
    /// Pager structure
    /// PCache object
    /// The main db file
    /// The two journal files
    /// Space to hold a pointer
    /// Database prefix
    /// database filename
    /// query parameters
    /// Journal filename
    /// WAL filename
    /// Terminator
    /// Fill in the Pager.zFilename and pPager.zQueryParam fields
    /// Skip zero prefix
    /// Fill in Pager.zJournal
    /// Fill in Pager.zWal
    /// Suppress warning about unused pPtr value
    /// Open the pager file.
    /// VFS flags returned by xOpen()
    /// If the file was successfully opened for read/write access,
    ///* choose a default page size in case we have to create the
    ///* database file. The default page size is the maximum of:
    ///*
    ///*    + SQLITE_DEFAULT_PAGE_SIZE,
    ///*    + The value returned by sqlite3OsSectorSize()
    ///*    + The largest page size that can be written atomically.
    /// If a temporary file is requested, it is not opened immediately.
    ///* In this case we accept the default page size and delay actually
    ///* opening the file until the first call to OsWrite().
    ///*
    ///* This branch is also run for an in-memory database. An in-memory
    ///* database is the same as a temp-file that is never written out to
    ///* disk and uses an in-memory rollback journal.
    ///*
    ///* This branch also runs for files marked as immutable.
    /// Pretend we already have a lock
    /// Pretend we are in EXCLUSIVE mode
    /// Do no locking
    /// The following call to PagerSetPagesize() serves to set the value of
    ///* Pager.pageSize and to allocate the Pager.pTmpSpace buffer.
    /// Initialize the PCache object.
    /// If an error occurred above, free the  Pager structure and close the file.
    /// pPager->stmtOpen = 0; */
    ///  /* pPager->stmtInUse = 0; */
    ///  /* pPager->nRef = 0; */
    ///  /* pPager->stmtSize = 0; */
    ///  /* pPager->stmtJSize = 0; */
    ///  /* pPager->nPage = 0;
    /// pPager->state = PAGER_UNLOCK; */
    ///  /* pPager->errMask = 0;
    /// pPager->pFirst = 0; */
    ///  /* pPager->pFirstSynced = 0; */
    ///  /* pPager->pLast = 0;
    /// pPager->xBusyHandler = 0; */
    ///  /* pPager->pBusyHandlerArg = 0;
    /// memset(pPager->aHash, 0, sizeof(pPager->aHash)); */
    ///  /* pPager->szMmap = SQLITE_DEFAULT_MMAP_SIZE // will be set by btree.c
    unreachable!();
}

///* Free all PgHdr objects stored in the Pager.pMmapFreelist list.
extern "C" fn pager_free_map_hdrs(p_pager_1: &Pager) -> () {
    let mut p: *mut PgHdr = core::ptr::null_mut();
    let mut p_next: *mut PgHdr = core::ptr::null_mut();
    {
        p = (*p_pager_1).p_mmap_freelist;
        '__b27: loop {
            if !(!(p).is_null()) { break '__b27; }
            '__c27: loop {
                p_next = unsafe { (*p).p_dirty };
                unsafe { sqlite3_free(p as *mut ()) };
                break '__c27;
            }
            p = p_next;
        }
    }
}

/// Verify that the database file has not be deleted or renamed out from
///* under the pager.  Return SQLITE_OK if the database is still where it ought
///* to be on disk.  Return non-zero (SQLITE_READONLY_DBMOVED or some other error
///* code from sqlite3OsAccess()) if the database has gone missing.
#[allow(unused_doc_comments)]
extern "C" fn database_is_unmoved(p_pager_1: &Pager) -> i32 {
    let mut b_has_moved: i32 = 0;
    let mut rc: i32 = 0;
    if (*p_pager_1).temp_file != 0 { return 0; }
    if (*p_pager_1).db_size == 0 as u32 { return 0; }
    { let _ = 0; };
    rc =
        unsafe {
            sqlite3_os_file_control((*p_pager_1).fd, 20,
                &raw mut b_has_moved as *mut ())
        };
    if rc == 12 {

        /// If the HAS_MOVED file-control is unimplemented, assume that the file
        ///* has not been moved.  That is the historical behavior of SQLite: prior to
        ///* version 3.8.3, it never checked
        (rc = 0);
    } else if rc == 0 && b_has_moved != 0 { rc = 8 | 4 << 8; }
    return rc;
}

///* This function is called before attempting a hot-journal rollback. It
///* syncs the journal file to disk, then sets pPager->journalHdr to the
///* size of the journal file so that the pager_playback() routine knows
///* that the entire journal file has been synced.
///*
///* Syncing a hot-journal to disk before attempting to roll it back ensures
///* that if a power-failure occurs during the rollback, the process that
///* attempts rollback following system recovery sees the same journal
///* content as this process.
///*
///* If everything goes as planned, SQLITE_OK is returned. Otherwise,
///* an SQLite error code.
extern "C" fn pager_sync_hot_journal(p_pager_1: &mut Pager) -> i32 {
    let mut rc: i32 = 0;
    if ((*p_pager_1).no_sync == 0) as i32 != 0 {
        rc = unsafe { sqlite3_os_sync((*p_pager_1).jfd, 2) };
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_os_file_size((*p_pager_1).jfd,
                    &mut (*p_pager_1).journal_hdr)
            };
    }
    return rc;
}

///* Shutdown the page cache.  Free all memory and close all files.
///*
///* If a transaction was in progress when this routine is called, that
///* transaction is rolled back.  All outstanding pages are invalidated
///* and their memory is freed.  Any attempt to use a page associated
///* with this page cache after this function returns will likely
///* result in a coredump.
///*
///* This function always succeeds. If a transaction is active an attempt
///* is made to roll it back. If an error occurs during the rollback
///* a hot journal may be left in the filesystem but no error is returned
///* to the caller.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_close(p_pager_1: *mut Pager, db: *mut Sqlite3)
    -> i32 {
    let p_tmp: *mut u8 = unsafe { (*p_pager_1).p_tmp_space } as *mut u8;
    { let _ = 0; };
    { let _ = 0; };
    unsafe { sqlite3_begin_benign_malloc() };
    pager_free_map_hdrs(unsafe { &*p_pager_1 });

    /// pPager->errCode = 0;
    unsafe { (*p_pager_1).exclusive_mode = 0 as u8 };
    {
        let mut a: *mut u8 = core::ptr::null_mut();
        { let _ = 0; };
        if !(db).is_null() && 0 as u64 == unsafe { (*db).flags } & 2048 as u64
                && 0 == database_is_unmoved(unsafe { &*p_pager_1 }) {
            a = p_tmp;
        }
        unsafe {
            sqlite3_wal_close(unsafe { (*p_pager_1).p_wal }, db,
                unsafe { (*p_pager_1).wal_sync_flags } as i32,
                unsafe { (*p_pager_1).page_size } as i32, a)
        };
        unsafe { (*p_pager_1).p_wal = core::ptr::null_mut() };
    }
    pager_reset(unsafe { &mut *p_pager_1 });
    if unsafe { (*p_pager_1).mem_db } != 0 {
        pager_unlock(p_pager_1);
    } else {
        if unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                core::ptr::null() {
            pager_error(p_pager_1,
                pager_sync_hot_journal(unsafe { &mut *p_pager_1 }));
        }
        pager_unlock_and_rollback(p_pager_1);
    }
    unsafe { sqlite3_end_benign_malloc() };
    unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
    unsafe { sqlite3_os_close(unsafe { (*p_pager_1).fd }) };
    unsafe { sqlite3_page_free(p_tmp as *mut ()) };
    unsafe { sqlite3_pcache_close(unsafe { (*p_pager_1).p_p_cache }) };
    { let _ = 0; };
    { let _ = 0; };
    unsafe { sqlite3_free(p_pager_1 as *mut ()) };
    return 0;
}

///* Read the first N bytes from the beginning of the file into memory
///* that pDest points to.
///*
///* If the pager was opened on a transient file (zFilename==""), or
///* opened on a file less than N bytes in size, the output buffer is
///* zeroed and SQLITE_OK returned. The rationale for this is that this
///* function is used to read database headers, and a new transient or
///* zero sized database has a header than consists entirely of zeroes.
///*
///* If any IO error apart from SQLITE_IOERR_SHORT_READ is encountered,
///* the error code is returned to the caller and the contents of the
///* output buffer undefined.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_read_fileheader(p_pager_1: &Pager, n_1: i32,
    p_dest_1: *mut u8) -> i32 {
    let mut rc: i32 = 0;
    unsafe { memset(p_dest_1 as *mut (), 0, n_1 as u64) };
    { let _ = 0; };

    /// This routine is only called by btree immediately after creating
    ///* the Pager object.  There has not been an opportunity to transition
    ///* to WAL mode yet.
    { let _ = 0; };
    if unsafe { (*(*p_pager_1).fd).p_methods } != core::ptr::null() {
        rc =
            unsafe {
                sqlite3_os_read((*p_pager_1).fd, p_dest_1 as *mut (), n_1,
                    0 as i64)
            };
        if rc == 10 | 2 << 8 { rc = 0; }
    }
    return rc;
}

/// Functions used to configure a Pager object.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_set_busy_handler(p_pager_1: &mut Pager,
    x_busy_handler_1: unsafe extern "C" fn(*mut ()) -> i32,
    p_busy_handler_arg_1: *mut ()) -> () {
    let mut ap: *mut *mut () = core::ptr::null_mut();
    (*p_pager_1).x_busy_handler = x_busy_handler_1;
    (*p_pager_1).p_busy_handler_arg = p_busy_handler_arg_1;
    ap = &raw mut (*p_pager_1).x_busy_handler as *mut *mut ();
    { let _ = 0; };
    { let _ = 0; };
    unsafe {
        sqlite3_os_file_control_hint((*p_pager_1).fd, 15, ap as *mut ())
    };
}

///* Attempt to set the maximum database page count if mxPage is positive.
///* Make no changes if mxPage is zero or negative.  And never reduce the
///* maximum page count below the current size of the database.
///*
///* Regardless of mxPage, return the current maximum page count.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_max_page_count(p_pager_1: &mut Pager,
    mx_page_1: Pgno) -> Pgno {
    if mx_page_1 > 0 as u32 { (*p_pager_1).mx_pgno = mx_page_1; }
    { let _ = 0; };

    /// Called only by OP_MaxPgcnt */
    ///  /* assert( pPager->mxPgno>=pPager->dbSize ); */
    ///  /* OP_MaxPgcnt ensures that the parameter passed to this function is not
    ///* less than the total number of valid pages in the database. But this
    ///* may be less than Pager.dbSize, and so the assert() above is not valid
    return (*p_pager_1).mx_pgno;
}

///* Change the maximum number of in-memory pages that are allowed
///* before attempting to recycle clean and unused pages.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_set_cachesize(p_pager_1: &Pager,
    mx_page_1: i32) -> () {
    unsafe {
        sqlite3_pcache_set_cachesize((*p_pager_1).p_p_cache, mx_page_1)
    };
}

///* Change the maximum number of in-memory pages that are allowed
///* before attempting to spill pages to journal.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_set_spillsize(p_pager_1: &Pager,
    mx_page_1: i32) -> i32 {
    return unsafe {
            sqlite3_pcache_set_spillsize((*p_pager_1).p_p_cache, mx_page_1)
        };
}

///* Change the maximum size of any memory mapping made of the database file.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_set_mmap_limit(p_pager_1: *mut Pager,
    sz_mmap_1: Sqlite3Int64) -> () {
    unsafe { (*p_pager_1).sz_mmap = sz_mmap_1 };
    pager_fix_maplimit(p_pager_1);
}

///* Free as much memory as possible from the pager.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_shrink(p_pager_1: &Pager) -> () {
    unsafe { sqlite3_pcache_shrink((*p_pager_1).p_p_cache) };
}

///* Get/set the locking-mode for this pager. Parameter eMode must be one
///* of PAGER_LOCKINGMODE_QUERY, PAGER_LOCKINGMODE_NORMAL or
///* PAGER_LOCKINGMODE_EXCLUSIVE. If the parameter is not _QUERY, then
///* the locking-mode is set to the value specified.
///*
///* The returned value is either PAGER_LOCKINGMODE_NORMAL or
///* PAGER_LOCKINGMODE_EXCLUSIVE, indicating the current (possibly updated)
///* locking-mode.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_locking_mode(p_pager_1: &mut Pager,
    e_mode_1: i32) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if e_mode_1 >= 0 && ((*p_pager_1).temp_file == 0) as i32 != 0 &&
            (unsafe { sqlite3_wal_heap_memory((*p_pager_1).p_wal) } == 0) as
                    i32 != 0 {
        (*p_pager_1).exclusive_mode = e_mode_1 as u8;
    }
    return (*p_pager_1).exclusive_mode as i32;
}

///* This function is called as part of the transition from PAGER_OPEN
///* to PAGER_READER state to determine the size of the database file
///* in pages (assuming the page size currently stored in Pager.pageSize).
///*
///* If no error occurs, SQLITE_OK is returned and the size of the database
///* in pages is stored in *pnPage. Otherwise, an error code (perhaps
///* SQLITE_IOERR_FSTAT) is returned and *pnPage is left unmodified.
#[allow(unused_doc_comments)]
extern "C" fn pager_pagecount(p_pager_1: &mut Pager, pn_page_1: &mut Pgno)
    -> i32 {
    let mut n_page: Pgno = 0 as Pgno;

    /// Value to return via *pnPage
    /// Query the WAL sub-system for the database size. The WalDbsize()
    ///* function returns zero if the WAL is not open (i.e. Pager.pWal==0), or
    ///* if the database size is not available. The database size is not
    ///* available from the WAL sub-system if the log file is empty or
    ///* contains no valid committed transactions.
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    n_page = unsafe { sqlite3_wal_dbsize((*p_pager_1).p_wal) };
    if n_page == 0 as u32 &&
            unsafe { (*(*p_pager_1).fd).p_methods } != core::ptr::null() {
        let mut n: i64 = 0 as i64;
        /// Size of db file in bytes
        let rc: i32 =
            unsafe { sqlite3_os_file_size((*p_pager_1).fd, &mut n) };
        if rc != 0 { return rc; }
        n_page =
            ((n + (*p_pager_1).page_size - 1 as i64) / (*p_pager_1).page_size)
                as Pgno;
    }
    if n_page > (*p_pager_1).mx_pgno {
        (*p_pager_1).mx_pgno = n_page as Pgno;
    }
    *pn_page_1 = n_page;
    return 0;
}

///* This function is called while transitioning from PAGER_OPEN to a
///* higher state. It tests if there is a hot journal present in
///* the file-system for the given pager. A hot journal is one that
///* needs to be played back. According to this function, a hot-journal
///* file exists if the following criteria are met:
///*
///*   * The journal file exists in the file system, and
///*   * No process holds a RESERVED or greater lock on the database file, and
///*   * The database file itself is greater than 0 bytes in size, and
///*   * The first byte of the journal file exists and is not 0x00.
///*
///* If the current size of the database file is 0 but a journal file
///* exists, that is probably an old journal left over from a prior
///* database with the same name. In this case the journal file is
///* just deleted using OsDelete, *pExists is set to 0 and SQLITE_OK
///* is returned.
///*
///* This routine does not check if there is a super-journal filename
///* at the end of the file. If there is, and that super-journal file
///* does not exist, then the journal file is not really hot. In this
///* case this routine will return a false-positive. The pager_playback()
///* routine will discover that the journal file is not really hot and
///* will not roll it back.
///*
///* If a hot-journal file is found to exist, *pExists is set to 1 and
///* SQLITE_OK returned. If no hot-journal file is present, *pExists is
///* set to 0 and SQLITE_OK returned. If an IO error occurs while trying
///* to determine whether or not a hot-journal file exists, the IO error
///* code is returned and the value of *pExists is undefined.
#[allow(unused_doc_comments)]
extern "C" fn has_hot_journal(p_pager_1: *mut Pager, p_exists_1: &mut i32)
    -> i32 {
    let p_vfs: *mut Sqlite3Vfs = unsafe { (*p_pager_1).p_vfs };
    let mut rc: i32 = 0;
    /// Return code
    let mut exists: i32 = 1;
    /// True if a journal file is present
    let jrnl_open: i32 =
        (!(unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                                core::ptr::null()) as i32 == 0) as i32 as i32;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    *p_exists_1 = 0;
    if (jrnl_open == 0) as i32 != 0 {
        rc =
            unsafe {
                sqlite3_os_access(p_vfs,
                    unsafe { (*p_pager_1).z_journal } as *const i8, 0,
                    &mut exists)
            };
    }
    if rc == 0 && exists != 0 {
        let mut locked: i32 = 0;

        /// True if some process holds a RESERVED lock
        /// Race condition here:  Another process might have been holding the
        ///* the RESERVED lock and have a journal open at the sqlite3OsAccess()
        ///* call above, but then delete the journal and drop the lock before
        ///* we get to the following sqlite3OsCheckReservedLock() call.  If that
        ///* is the case, this routine might think there is a hot journal when
        ///* in fact there is none.  This results in a false-positive which will
        ///* be dealt with by the playback routine.  Ticket #3883.
        (rc =
            unsafe {
                sqlite3_os_check_reserved_lock(unsafe { (*p_pager_1).fd },
                    &mut locked)
            });
        if rc == 0 && (locked == 0) as i32 != 0 {
            let mut n_page: Pgno = 0 as Pgno;

            /// Number of pages in database file
            { let _ = 0; };
            rc = pager_pagecount(unsafe { &mut *p_pager_1 }, &mut n_page);
            if rc == 0 {
                if n_page == 0 as u32 && (jrnl_open == 0) as i32 != 0 {
                    unsafe { sqlite3_begin_benign_malloc() };
                    if pager_lock_db(unsafe { &mut *p_pager_1 }, 2) == 0 {
                        unsafe {
                            sqlite3_os_delete(p_vfs,
                                unsafe { (*p_pager_1).z_journal } as *const i8, 0)
                        };
                        if (unsafe { (*p_pager_1).exclusive_mode } == 0) as i32 != 0
                            {
                            pager_unlock_db(unsafe { &mut *p_pager_1 }, 1);
                        }
                    }
                    unsafe { sqlite3_end_benign_malloc() };
                } else {
                    if (jrnl_open == 0) as i32 != 0 {
                        let mut f: i32 = 1 | 2048;
                        rc =
                            unsafe {
                                sqlite3_os_open(p_vfs,
                                    unsafe { (*p_pager_1).z_journal } as *const i8,
                                    unsafe { (*p_pager_1).jfd }, f, &mut f)
                            };
                    }
                    if rc == 0 {
                        let mut first: u8 = 0 as u8;
                        rc =
                            unsafe {
                                sqlite3_os_read(unsafe { (*p_pager_1).jfd },
                                    &raw mut first as *mut (), 1, 0 as i64)
                            };
                        if rc == 10 | 2 << 8 { rc = 0; }
                        if (jrnl_open == 0) as i32 != 0 {
                            unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
                        }
                        *p_exists_1 = (first as i32 != 0) as i32;
                    } else if rc == 14 {

                        /// If we cannot open the rollback journal file in order to see if
                        ///* it has a zero header, that might be due to an I/O error, or
                        ///* it might be due to the race condition described above and in
                        ///* ticket #3883.  Either way, assume that the journal is hot.
                        ///* This might be a false positive.  But if it is, then the
                        ///* automatic journal playback and recovery mechanism will deal
                        ///* with it under an EXCLUSIVE lock where we do not need to
                        ///* worry so much with race conditions.
                        (*p_exists_1 = 1);
                        rc = 0;
                    }
                }
            }
        }
    }
    return rc;
}

///* Return true if the underlying VFS for the given pager supports the
///* primitives necessary for write-ahead logging.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_wal_supported(p_pager_1: &Pager) -> i32 {
    let p_methods: *const Sqlite3IoMethods =
        unsafe { (*(*p_pager_1).fd).p_methods };
    if (*p_pager_1).no_lock != 0 { return 0; }
    return ((*p_pager_1).exclusive_mode != 0 ||
                unsafe { (*p_methods).i_version } as i32 >= 2 &&
                    unsafe { (*p_methods).x_shm_map.is_some() }) as i32;
}

///* Attempt to take an exclusive lock on the database file. If a PENDING lock
///* is obtained instead, immediately release it.
#[allow(unused_doc_comments)]
extern "C" fn pager_exclusive_lock(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut e_orig_lock: u8 = 0 as u8;

    /// Original lock
    { let _ = 0; };
    e_orig_lock = unsafe { (*p_pager_1).e_lock };
    rc = pager_lock_db(unsafe { &mut *p_pager_1 }, 4);
    if rc != 0 {

        /// If the attempt to grab the exclusive lock failed, release the
        ///* pending lock that may have been obtained instead.
        pager_unlock_db(unsafe { &mut *p_pager_1 }, e_orig_lock as i32);
    }
    return rc;
}

///* Call sqlite3WalOpen() to open the WAL handle. If the pager is in
///* exclusive-locking mode when this function is called, take an EXCLUSIVE
///* lock on the database file and use heap-memory to store the wal-index
///* in. Otherwise, use the normal shared-memory.
extern "C" fn pager_open_wal(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_pager_1).exclusive_mode } != 0 {
        rc = pager_exclusive_lock(p_pager_1);
    }
    if rc == 0 {
        rc =
            unsafe {
                sqlite3_wal_open(unsafe { (*p_pager_1).p_vfs },
                    unsafe { (*p_pager_1).fd },
                    unsafe { (*p_pager_1).z_wal } as *const i8,
                    unsafe { (*p_pager_1).exclusive_mode } as i32,
                    unsafe { (*p_pager_1).journal_size_limit },
                    unsafe { &mut (*p_pager_1).p_wal })
            };
    }
    pager_fix_maplimit(p_pager_1);
    return rc;
}

///* The caller must be holding a SHARED lock on the database file to call
///* this function.
///*
///* If the pager passed as the first argument is open on a real database
///* file (not a temp file or an in-memory database), and the WAL file
///* is not already open, make an attempt to open it now. If successful,
///* return SQLITE_OK. If an error occurs or the VFS used by the pager does
///* not support the xShmXXX() methods, return an error code. *pbOpen is
///* not modified in either case.
///*
///* If the pager is open on a temp-file (or in-memory database), or if
///* the WAL file is already open, set *pbOpen to 1 and return SQLITE_OK
///* without doing anything.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_open_wal(p_pager_1: *mut Pager,
    pb_open_1: &mut i32) -> i32 {
    let mut rc: i32 = 0;

    /// Return code
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if (unsafe { (*p_pager_1).temp_file } == 0) as i32 != 0 &&
            (unsafe { (*p_pager_1).p_wal }).is_null() as i32 != 0 {
        if (sqlite3_pager_wal_supported(unsafe { &*p_pager_1 }) == 0) as i32
                != 0 {
            return 14;
        }

        /// Close any rollback journal previously open
        unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
        rc = pager_open_wal(p_pager_1);
        if rc == 0 {
            unsafe { (*p_pager_1).journal_mode = 5 as u8 };
            unsafe { (*p_pager_1).e_state = 0 as u8 };
        }
    } else { *pb_open_1 = 1; }
    return rc;
}

///* Check if the *-wal file that corresponds to the database opened by pPager
///* exists if the database is not empty, or verify that the *-wal file does
///* not exist (by deleting it) if the database file is empty.
///*
///* If the database is not empty and the *-wal file exists, open the pager
///* in WAL mode.  If the database is empty or if no *-wal file exists and
///* if no error occurs, make sure Pager.journalMode is not set to
///* PAGER_JOURNALMODE_WAL.
///*
///* Return SQLITE_OK or an error code.
///*
///* The caller must hold a SHARED lock on the database file to call this
///* function. Because an EXCLUSIVE lock on the db file is required to delete
///* a WAL on a none-empty database, this ensures there is no race condition
///* between the xAccess() below and an xDelete() being executed by some
///* other connection.
#[allow(unused_doc_comments)]
extern "C" fn pager_open_wal_if_present(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if (unsafe { (*p_pager_1).temp_file } == 0) as i32 != 0 {
        let mut is_wal: i32 = 0;

        /// True if WAL file exists
        (rc =
            unsafe {
                sqlite3_os_access(unsafe { (*p_pager_1).p_vfs },
                    unsafe { (*p_pager_1).z_wal } as *const i8, 0, &mut is_wal)
            });
        if rc == 0 {
            if is_wal != 0 {
                let mut n_page: Pgno = 0 as Pgno;

                /// Size of the database file
                (rc =
                    pager_pagecount(unsafe { &mut *p_pager_1 }, &mut n_page));
                if rc != 0 { return rc; }
                if n_page == 0 as u32 {
                    rc =
                        unsafe {
                            sqlite3_os_delete(unsafe { (*p_pager_1).p_vfs },
                                unsafe { (*p_pager_1).z_wal } as *const i8, 0)
                        };
                } else {
                    rc = unsafe { sqlite3_pager_open_wal(p_pager_1, &mut 0) };
                }
            } else if unsafe { (*p_pager_1).journal_mode } as i32 == 5 {
                unsafe { (*p_pager_1).journal_mode = 0 as u8 };
            }
        }
    }
    return rc;
}

///* Begin a read transaction on the WAL.
///*
///* This routine used to be called "pagerOpenSnapshot()" because it essentially
///* makes a snapshot of the database at the current point in time and preserves
///* that snapshot for use by the reader in spite of concurrently changes by
///* other writers or checkpointers.
#[allow(unused_doc_comments)]
extern "C" fn pager_begin_read_transaction(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut changed: i32 = 0;

    /// True if cache must be reset
    { let _ = 0; };
    { let _ = 0; };

    /// sqlite3WalEndReadTransaction() was not called for the previous
    ///* transaction in locking_mode=EXCLUSIVE.  So call it now.  If we
    ///* are in locking_mode=NORMAL and EndRead() was previously called,
    ///* the duplicate call is harmless.
    unsafe {
        sqlite3_wal_end_read_transaction(unsafe { (*p_pager_1).p_wal })
    };
    rc =
        unsafe {
            sqlite3_wal_begin_read_transaction(unsafe { (*p_pager_1).p_wal },
                &mut changed)
        };
    if rc != 0 || changed != 0 {
        pager_reset(unsafe { &mut *p_pager_1 });
        if unsafe { (*p_pager_1).b_use_fetch } != 0 {
            unsafe {
                sqlite3_os_unfetch(unsafe { (*p_pager_1).fd }, 0 as i64,
                    core::ptr::null_mut())
            };
        }
    }
    return rc;
}

///* This function is called to obtain a shared lock on the database file.
///* It is illegal to call sqlite3PagerGet() until after this function
///* has been successfully called. If a shared-lock is already held when
///* this function is called, it is a no-op.
///*
///* The following operations are also performed by this function.
///*
///*   1) If the pager is currently in PAGER_OPEN state (no lock held
///*      on the database file), then an attempt is made to obtain a
///*      SHARED lock on the database file. Immediately after obtaining
///*      the SHARED lock, the file-system is checked for a hot-journal,
///*      which is played back if present. Following any hot-journal
///*      rollback, the contents of the cache are validated by checking
///*      the 'change-counter' field of the database file header and
///*      discarded if they are found to be invalid.
///*
///*   2) If the pager is running in exclusive-mode, and there are currently
///*      no outstanding references to any pages, and is in the error state,
///*      then an attempt is made to clear the error state by discarding
///*      the contents of the page cache and rolling back any open journal
///*      file.
///*
///* If everything is successful, SQLITE_OK is returned. If an IO error
///* occurs while locking the database, checking for a hot-journal file or
///* rolling back a journal file, the IO error code is returned.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_shared_lock(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = 0;
    '__b28: loop {
        '__c28: loop {

            /// Return code
            /// This routine is only called from b-tree and only when there are no
            ///* outstanding pages. This implies that the pager state should either
            ///* be OPEN or READER. READER is only possible if the pager is or was in
            ///* exclusive access mode.
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if !(unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut()) as
                            i32 != 0 && unsafe { (*p_pager_1).e_state } as i32 == 0 {
                let mut b_hot_journal: i32 = 1;

                /// True if there exists a hot journal-file
                { let _ = 0; };
                { let _ = 0; };
                rc = pager_wait_on_lock(p_pager_1, 1);
                if rc != 0 { { let _ = 0; }; break '__b28; }
                if unsafe { (*p_pager_1).e_lock } as i32 <= 1 {
                    rc = has_hot_journal(p_pager_1, &mut b_hot_journal);
                }
                if rc != 0 { break '__b28; }
                if b_hot_journal != 0 {
                    if unsafe { (*p_pager_1).read_only } != 0 {
                        rc = 8 | 3 << 8;
                        break '__b28;
                    }

                    /// Get an EXCLUSIVE lock on the database file. At this point it is
                    ///* important that a RESERVED lock is not obtained on the way to the
                    ///* EXCLUSIVE lock. If it were, another process might open the
                    ///* database file, detect the RESERVED lock, and conclude that the
                    ///* database is safe to read while this process is still rolling the
                    ///* hot-journal back.
                    ///*
                    ///* Because the intermediate RESERVED lock is not requested, any
                    ///* other process attempting to access the database file will get to
                    ///* this point in the code and fail to obtain its own EXCLUSIVE lock
                    ///* on the database file.
                    ///*
                    ///* Unless the pager is in locking_mode=exclusive mode, the lock is
                    ///* downgraded to SHARED_LOCK before this function returns.
                    (rc = pager_lock_db(unsafe { &mut *p_pager_1 }, 4));
                    if rc != 0 { break '__b28; }
                    if !(unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                                            core::ptr::null()) as i32 != 0 &&
                            unsafe { (*p_pager_1).journal_mode } as i32 != 2 {
                        let p_vfs: *mut Sqlite3Vfs = unsafe { (*p_pager_1).p_vfs };
                        let mut b_exists: i32 = 0;

                        /// True if journal file exists
                        (rc =
                            unsafe {
                                sqlite3_os_access(p_vfs,
                                    unsafe { (*p_pager_1).z_journal } as *const i8, 0,
                                    &mut b_exists)
                            });
                        if rc == 0 && b_exists != 0 {
                            let mut fout: i32 = 0;
                            let f: i32 = 2 | 2048;
                            { let _ = 0; };
                            rc =
                                unsafe {
                                    sqlite3_os_open(p_vfs,
                                        unsafe { (*p_pager_1).z_journal } as *const i8,
                                        unsafe { (*p_pager_1).jfd }, f, &mut fout)
                                };
                            { let _ = 0; };
                            if rc == 0 && fout & 1 != 0 {
                                rc = unsafe { sqlite3_cantopen_error(5392) };
                                unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
                            }
                        }
                    }
                    if unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                            core::ptr::null() {
                        { let _ = 0; };
                        rc = pager_sync_hot_journal(unsafe { &mut *p_pager_1 });
                        if rc == 0 {
                            rc =
                                pager_playback(p_pager_1,
                                    (unsafe { (*p_pager_1).temp_file } == 0) as i32 as i32);
                            unsafe { (*p_pager_1).e_state = 0 as u8 };
                        }
                    } else if (unsafe { (*p_pager_1).exclusive_mode } == 0) as
                                i32 != 0 {
                        pager_unlock_db(unsafe { &mut *p_pager_1 }, 1);
                    }
                    if rc != 0 {

                        /// This branch is taken if an error occurs while trying to open
                        ///* or roll back a hot-journal while holding an EXCLUSIVE lock. The
                        ///* pager_unlock() routine will be called before returning to unlock
                        ///* the file. If the unlock attempt fails, then Pager.eLock must be
                        ///* set to UNKNOWN_LOCK (see the comment above the #define for
                        ///* UNKNOWN_LOCK above for an explanation).
                        ///*
                        ///* In order to get pager_unlock() to do this, set Pager.eState to
                        ///* PAGER_ERROR now. This is not actually counted as a transition
                        ///* to ERROR state in the state diagram at the top of this file,
                        ///* since we know that the same call to pager_unlock() will very
                        ///* shortly transition the pager object to the OPEN state. Calling
                        ///* assert_pager_state() would fail now, as it should not be possible
                        ///* to be in ERROR state when there are zero outstanding page
                        ///* references.
                        pager_error(p_pager_1, rc);
                        break '__b28;
                    }
                    { let _ = 0; };
                    { let _ = 0; };
                }
                if (unsafe { (*p_pager_1).temp_file } == 0) as i32 != 0 &&
                        unsafe { (*p_pager_1).has_held_shared_lock } != 0 {
                    /// The shared-lock has just been acquired then check to
                    ///* see if the database has been modified.  If the database has changed,
                    ///* flush the cache.  The hasHeldSharedLock flag prevents this from
                    ///* occurring on the very first access to a file, in order to save a
                    ///* single unnecessary sqlite3OsRead() call at the start-up.
                    ///*
                    ///* Database changes are detected by looking at 15 bytes beginning
                    ///* at offset 24 into the file.  The first 4 of these 16 bytes are
                    ///* a 32-bit counter that is incremented with each change.  The
                    ///* other bytes change randomly with each file change when
                    ///* a codec is in use.
                    ///*
                    ///* There is a vanishingly small chance that a change will not be
                    ///* detected.  The chance of an undetected change is so small that
                    ///* it can be neglected.
                    let mut db_file_vers: [i8; 16] = [0; 16];
                    rc =
                        unsafe {
                            sqlite3_os_read(unsafe { (*p_pager_1).fd },
                                &raw mut db_file_vers as *mut (),
                                core::mem::size_of::<[i8; 16]>() as i32, 24 as i64)
                        };
                    if rc != 0 {
                        if rc != 10 | 2 << 8 { break '__b28; }
                        unsafe {
                            memset(&raw mut db_file_vers[0 as usize] as *mut i8 as
                                    *mut (), 0, core::mem::size_of::<[i8; 16]>() as u64)
                        };
                    }
                    if unsafe {
                                memcmp(unsafe {
                                                &raw mut (*p_pager_1).db_file_vers[0 as usize]
                                            } as *mut i8 as *const (),
                                    &raw mut db_file_vers[0 as usize] as *mut i8 as *const (),
                                    core::mem::size_of::<[i8; 16]>() as u64)
                            } != 0 {
                        pager_reset(unsafe { &mut *p_pager_1 });
                        if unsafe { (*p_pager_1).b_use_fetch } != 0 {
                            unsafe {
                                sqlite3_os_unfetch(unsafe { (*p_pager_1).fd }, 0 as i64,
                                    core::ptr::null_mut())
                            };
                        }
                    }
                }

                /// If there is a WAL file in the file-system, open this database in WAL
                ///* mode. Otherwise, the following function call is a no-op.
                (rc = pager_open_wal_if_present(p_pager_1));
                { let _ = 0; };
            }
            if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
                { let _ = 0; };
                rc = pager_begin_read_transaction(p_pager_1);
            }
            if unsafe { (*p_pager_1).temp_file } as i32 == 0 &&
                        unsafe { (*p_pager_1).e_state } as i32 == 0 && rc == 0 {
                rc =
                    pager_pagecount(unsafe { &mut *p_pager_1 },
                        unsafe { &mut (*p_pager_1).db_size });
            }
            break '__c28;
        }
        if !(false) { break '__b28; }
    }
    if rc != 0 {
        { let _ = 0; };
        pager_unlock(p_pager_1);
        { let _ = 0; };
    } else {
        unsafe { (*p_pager_1).e_state = 1 as u8 };
        unsafe { (*p_pager_1).has_held_shared_lock = 1 as u8 };
    }
    return rc;
}

///* Set the journal-mode for this pager. Parameter eMode must be one of:
///*
///*    PAGER_JOURNALMODE_DELETE
///*    PAGER_JOURNALMODE_TRUNCATE
///*    PAGER_JOURNALMODE_PERSIST
///*    PAGER_JOURNALMODE_OFF
///*    PAGER_JOURNALMODE_MEMORY
///*    PAGER_JOURNALMODE_WAL
///*
///* The journalmode is set to the value specified if the change is allowed.
///* The change may be disallowed for the following reasons:
///*
///*   *  An in-memory database can only have its journal_mode set to _OFF
///*      or _MEMORY.
///*
///*   *  Temporary databases cannot have _WAL journalmode.
///*
///* The returned indicate the current (possibly updated) journal-mode.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_set_journal_mode(p_pager_1: *mut Pager,
    mut e_mode_1: i32) -> i32 {
    let e_old: u8 = unsafe { (*p_pager_1).journal_mode };

    /// Prior journalmode
    /// The eMode parameter is always valid
    { let _ = 0; };

    /// 0
    /// 1
    /// 2
    /// 3
    /// 4
    /// 5
    /// This routine is only called from the OP_JournalMode opcode, and
    ///* the logic there will never allow a temporary file to be changed
    ///* to WAL mode.
    { let _ = 0; };
    if unsafe { (*p_pager_1).mem_db } != 0 {
        { let _ = 0; };
        if e_mode_1 != 4 && e_mode_1 != 2 { e_mode_1 = e_old as i32; }
    }
    if e_mode_1 != e_old as i32 {

        /// Change the journal mode.
        { let _ = 0; };
        unsafe { (*p_pager_1).journal_mode = e_mode_1 as u8 };

        /// When transitioning from TRUNCATE or PERSIST to any other journal
        ///* mode except WAL, unless the pager is in locking_mode=exclusive mode,
        ///* delete the journal file.
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if (unsafe { (*p_pager_1).exclusive_mode } == 0) as i32 != 0 &&
                    e_old as i32 & 5 == 1 && e_mode_1 & 1 == 0 {

            /// In this case we would like to delete the journal file. If it is
            ///* not possible, then that is not a problem. Deleting the journal file
            ///* here is an optimization only.
            ///*
            ///* Before deleting the journal file, obtain a RESERVED lock on the
            ///* database file. This ensures that the journal file is not deleted
            ///* while it is in use by some other client.
            unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
            if unsafe { (*p_pager_1).e_lock } as i32 >= 2 {
                unsafe {
                    sqlite3_os_delete(unsafe { (*p_pager_1).p_vfs },
                        unsafe { (*p_pager_1).z_journal } as *const i8, 0)
                };
            } else {
                let mut rc: i32 = 0;
                let state: i32 = unsafe { (*p_pager_1).e_state } as i32;
                { let _ = 0; };
                if state == 0 { rc = sqlite3_pager_shared_lock(p_pager_1); }
                if unsafe { (*p_pager_1).e_state } as i32 == 1 {
                    { let _ = 0; };
                    rc = pager_lock_db(unsafe { &mut *p_pager_1 }, 2);
                }
                if rc == 0 {
                    unsafe {
                        sqlite3_os_delete(unsafe { (*p_pager_1).p_vfs },
                            unsafe { (*p_pager_1).z_journal } as *const i8, 0)
                    };
                }
                if rc == 0 && state == 1 {
                    pager_unlock_db(unsafe { &mut *p_pager_1 }, 1);
                } else if state == 0 { pager_unlock(p_pager_1); }
                { let _ = 0; };
            }
        } else if e_mode_1 == 2 || e_mode_1 == 4 {
            unsafe { sqlite3_os_close(unsafe { (*p_pager_1).jfd }) };
        }
    }

    /// Return the new journal mode
    return unsafe { (*p_pager_1).journal_mode } as i32;
}

///* Return the current journal mode.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_get_journal_mode(p_pager_1: &Pager) -> i32 {
    return (*p_pager_1).journal_mode as i32;
}

///* Return TRUE if the pager is in a state where it is OK to change the
///* journalmode.  Journalmode changes can only happen when the database
///* is unmodified.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_ok_to_change_journal_mode(p_pager_1: &Pager)
    -> i32 {
    { let _ = 0; };
    if (*p_pager_1).e_state as i32 >= 3 { return 0; }
    if unsafe { (*(*p_pager_1).jfd).p_methods } != core::ptr::null() &&
            (*p_pager_1).journal_off > 0 as i64 {
        return 0;
    }
    return 1;
}

///* Get/set the size-limit used for persistent journal files.
///*
///* Setting the size limit to -1 means no limit is enforced.
///* An attempt to set a limit smaller than -1 is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_journal_size_limit(p_pager_1: &mut Pager,
    i_limit_1: i64) -> i64 {
    if i_limit_1 >= -1 as i64 {
        (*p_pager_1).journal_size_limit = i_limit_1;
        unsafe { sqlite3_wal_limit((*p_pager_1).p_wal, i_limit_1) };
    }
    return (*p_pager_1).journal_size_limit;
}

///* Return a pointer to the pPager->pBackup variable. The backup module
///* in backup.c maintains the content of this variable. This module
///* uses it opaquely as an argument to sqlite3BackupRestart() and
///* sqlite3BackupUpdate() only.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_backup_ptr(p_pager_1: &mut Pager)
    -> *mut *mut Sqlite3Backup {
    return &mut (*p_pager_1).p_backup;
}

///* Flush all unreferenced dirty pages to disk.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_flush(p_pager_1: *mut Pager) -> i32 {
    let mut rc: i32 = unsafe { (*p_pager_1).err_code };
    if (unsafe { (*p_pager_1).mem_db } == 0) as i32 != 0 {
        let mut p_list: *mut PgHdr =
            unsafe {
                sqlite3_pcache_dirty_list(unsafe { (*p_pager_1).p_p_cache })
            };
        { let _ = 0; };
        while rc == 0 && !(p_list).is_null() {
            let p_next: *mut PgHdr = unsafe { (*p_list).p_dirty };
            if unsafe { (*p_list).n_ref } == 0 as i64 {
                rc = pager_stress(p_pager_1 as *mut (), p_list);
            }
            p_list = p_next;
        }
    }
    return rc;
}

///* Increment the reference count for page pPg.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_ref(p_pg_1: *mut DbPage) -> () {
    unsafe { sqlite3_pcache_ref(p_pg_1 as *mut PgHdr) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_unref(p_pg_1: *mut DbPage) -> () {
    if !(p_pg_1).is_null() { sqlite3_pager_unref_not_null(p_pg_1); }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_unref_page_one(p_pg_1: *mut DbPage) -> () {
    let mut p_pager: *mut Pager = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };

    /// Page1 is never memory mapped
    (p_pager = unsafe { (*p_pg_1).p_pager });
    unsafe { sqlite3_pcache_release(p_pg_1 as *mut PgHdr) };
    pager_unlock_if_unused(p_pager);
}

///* This function determines whether or not the atomic-write or
///* atomic-batch-write optimizations can be used with this pager. The
///* atomic-write optimization can be used if:
///*
///*  (a) the value returned by OsDeviceCharacteristics() indicates that
///*      a database page may be written atomically, and
///*  (b) the value returned by OsSectorSize() is less than or equal
///*      to the page size.
///*
///* If it can be used, then the value returned is the size of the journal
///* file when it contains rollback data for exactly one page.
///*
///* The atomic-batch-write optimization can be used if OsDeviceCharacteristics()
///* returns a value with the SQLITE_IOCAP_BATCH_ATOMIC bit set. -1 is
///* returned in this case.
///*
///* If neither optimization can be used, 0 is returned.
extern "C" fn jrnl_buffer_size(p_pager_1: *const Pager) -> i32 {
    { let _ = 0; };
    { let _ = p_pager_1; };
    return 0;
}

///* This function is called at the start of every write transaction.
///* There must already be a RESERVED or EXCLUSIVE lock on the database
///* file when this routine is called.
///*
///* Open the journal file for pager pPager and write a journal header
///* to the start of it. If there are active savepoints, open the sub-journal
///* as well. This function is only used when the journal file is being
///* opened to write a rollback log for a transaction. It is not used
///* when opening a hot journal file to roll it back.
///*
///* If the journal file is already open (as it may be in exclusive mode),
///* then this function just writes a journal header to the start of the
///* already open file.
///*
///* Whether or not the journal file is opened by this function, the
///* Pager.pInJournal bitvec structure is allocated.
///*
///* Return SQLITE_OK if everything is successful. Otherwise, return
///* SQLITE_NOMEM if the attempt to allocate Pager.pInJournal fails, or
///* an IO error code if opening or writing the journal file fails.
#[allow(unused_doc_comments)]
extern "C" fn pager_open_journal(p_pager_1: *mut Pager) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        /// Return code
        let p_vfs: *mut Sqlite3Vfs = unsafe { (*p_pager_1).p_vfs };

        /// Local cache of vfs pointer
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_pager_1).err_code } != 0 {
            return unsafe { (*p_pager_1).err_code };
        }
        if !(unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut()) as i32 !=
                    0 && unsafe { (*p_pager_1).journal_mode } as i32 != 2 {
            unsafe {
                (*p_pager_1).p_in_journal =
                    unsafe {
                        sqlite3_bitvec_create(unsafe { (*p_pager_1).db_size })
                    }
            };
            if unsafe { (*p_pager_1).p_in_journal } == core::ptr::null_mut() {
                return 7;
            }
            if !(unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                                core::ptr::null()) as i32 != 0 {
                if unsafe { (*p_pager_1).journal_mode } as i32 == 4 {
                    unsafe {
                        sqlite3_mem_journal_open(unsafe { (*p_pager_1).jfd })
                    };
                } else {
                    let mut flags: i32 = 2 | 4;
                    let mut n_spill: i32 = 0;
                    if unsafe { (*p_pager_1).temp_file } != 0 {
                        flags |= 8 | 4096;
                        flags |= 16;
                        n_spill = sqlite3Config.n_stmt_spill;
                    } else {
                        flags |= 2048;
                        n_spill = jrnl_buffer_size(p_pager_1 as *const Pager);
                    }

                    /// Verify that the database still has the same name as it did when
                    ///* it was originally opened.
                    (rc = database_is_unmoved(unsafe { &*p_pager_1 }));
                    if rc == 0 {
                        rc =
                            unsafe {
                                sqlite3_journal_open(p_vfs,
                                    unsafe { (*p_pager_1).z_journal } as *const i8,
                                    unsafe { (*p_pager_1).jfd }, flags, n_spill)
                            };
                    }
                }
                { let _ = 0; };
            }
            if rc == 0 {

                /// TODO: Check if all of these are really required.
                unsafe { (*p_pager_1).n_rec = 0 };
                unsafe { (*p_pager_1).journal_off = 0 as i64 };
                unsafe { (*p_pager_1).set_super = 0 as u8 };
                unsafe { (*p_pager_1).journal_hdr = 0 as i64 };
                rc = write_journal_hdr(p_pager_1);
            }
        }
        if rc != 0 {
            unsafe {
                sqlite3_bitvec_destroy(unsafe { (*p_pager_1).p_in_journal })
            };
            unsafe { (*p_pager_1).p_in_journal = core::ptr::null_mut() };
            unsafe { (*p_pager_1).journal_off = 0 as i64 };
        } else { { let _ = 0; }; unsafe { (*p_pager_1).e_state = 3 as u8 }; }
        return rc;
    }
}

///* Write page pPg onto the end of the rollback journal.
#[allow(unused_doc_comments)]
extern "C" fn pager_add_page_to_rollback_journal(p_pg_1: &mut PgHdr) -> i32 {
    let p_pager: *mut Pager = (*p_pg_1).p_pager;
    let mut rc: i32 = 0;
    let mut cksum: u32 = 0 as u32;
    let mut p_data2: *mut i8 = core::ptr::null_mut();
    let i_off: i64 = unsafe { (*p_pager).journal_off };

    /// We should never write to the journal file the page that
    ///* contains the database locks.  The following assert verifies
    ///* that we do not.
    { let _ = 0; };
    { let _ = 0; };
    p_data2 = (*p_pg_1).p_data as *mut i8;
    cksum =
        pager_cksum(unsafe { &*p_pager }, p_data2 as *mut u8 as *const u8);

    /// Even if an IO or diskfull error occurs while journalling the
    ///* page in the block above, set the need-sync flag for the page.
    ///* Otherwise, when the transaction is rolled back, the logic in
    ///* playback_one_page() will think that the page needs to be restored
    ///* in the database file. And if an IO error occurs while doing so,
    ///* then corruption may follow.
    ((*p_pg_1).flags |= 8 as u16);
    rc = write32bits(unsafe { (*p_pager).jfd }, i_off, (*p_pg_1).pgno);
    if rc != 0 { return rc; }
    rc =
        unsafe {
            sqlite3_os_write(unsafe { (*p_pager).jfd }, p_data2 as *const (),
                unsafe { (*p_pager).page_size } as i32, i_off + 4 as i64)
        };
    if rc != 0 { return rc; }
    rc =
        write32bits(unsafe { (*p_pager).jfd },
            i_off + unsafe { (*p_pager).page_size } + 4 as i64, cksum);
    if rc != 0 { return rc; }
    unsafe {
        (*p_pager).journal_off += 8 as i64 + unsafe { (*p_pager).page_size }
    };
    {
        let __p = unsafe { &mut (*p_pager).n_rec };
        let __t = *__p;
        *__p += 1;
        __t
    };
    { let _ = 0; };
    rc =
        unsafe {
            sqlite3_bitvec_set(unsafe { (*p_pager).p_in_journal },
                (*p_pg_1).pgno)
        };
    { let _ = 0; };
    rc |= add_to_savepoint_bitvecs(unsafe { &*p_pager }, (*p_pg_1).pgno);
    { let _ = 0; };
    return rc;
}

///* Mark a single data page as writeable. The page is written into the
///* main journal or sub-journal as required. If the page is written into
///* one of the journals, the corresponding bit is set in the
///* Pager.pInJournal bitvec and the PagerSavepoint.pInSavepoint bitvecs
///* of any open savepoints as appropriate.
#[allow(unused_doc_comments)]
extern "C" fn pager_write(p_pg_1: *mut PgHdr) -> i32 {
    let p_pager: *mut Pager = unsafe { (*p_pg_1).p_pager };
    let mut rc: i32 = 0;

    /// This routine is not called unless a write-transaction has already
    ///* been started. The journal file may or may not be open at this point.
    ///* It is never called in the ERROR state.
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_pager).e_state } as i32 == 2 {
        rc = pager_open_journal(p_pager);
        if rc != 0 { return rc; }
    }
    { let _ = 0; };
    { let _ = 0; };

    /// Mark the page that is about to be modified as dirty.
    unsafe { sqlite3_pcache_make_dirty(p_pg_1) };

    /// If a rollback journal is in use, them make sure the page that is about
    ///* to change is in the rollback journal, or if the page is a new page off
    ///* then end of the file, make sure it is marked as PGHDR_NEED_SYNC.
    { let _ = 0; };
    if unsafe { (*p_pager).p_in_journal } != core::ptr::null_mut() &&
            unsafe {
                    sqlite3_bitvec_test_not_null(unsafe {
                            (*p_pager).p_in_journal
                        }, unsafe { (*p_pg_1).pgno })
                } == 0 {
        { let _ = 0; };
        if unsafe { (*p_pg_1).pgno } <= unsafe { (*p_pager).db_orig_size } {
            rc = pager_add_page_to_rollback_journal(unsafe { &mut *p_pg_1 });
            if rc != 0 { return rc; }
        } else {
            if unsafe { (*p_pager).e_state } as i32 != 4 {
                unsafe { (*p_pg_1).flags |= 8 as u16 };
            }
        }
    }

    /// The PGHDR_DIRTY bit is set above when the page was added to the dirty-list
    ///* and before writing the page into the rollback journal.  Wait until now,
    ///* after the page has been successfully journalled, before setting the
    ///* PGHDR_WRITEABLE bit that indicates that the page can be safely modified.
    unsafe { (*p_pg_1).flags |= 4 as u16 };
    if unsafe { (*p_pager).n_savepoint } > 0 {
        rc = subjournal_page_if_required(p_pg_1);
    }
    if unsafe { (*p_pager).db_size } < unsafe { (*p_pg_1).pgno } {
        unsafe { (*p_pager).db_size = unsafe { (*p_pg_1).pgno } };
    }
    return rc;
}

///* This is a variant of sqlite3PagerWrite() that runs when the sector size
///* is larger than the page size.  SQLite makes the (reasonable) assumption that
///* all bytes of a sector are written together by hardware.  Hence, all bytes of
///* a sector need to be journalled in case of a power loss in the middle of
///* a write.
///*
///* Usually, the sector size is less than or equal to the page size, in which
///* case pages can be individually written.  This routine only runs in the
///* exceptional case where the page size is smaller than the sector size.
#[allow(unused_doc_comments)]
extern "C" fn pager_write_large_sector(p_pg_1: &PgHdr) -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut n_page_count: Pgno = 0 as Pgno;
    /// Total number of pages in database file
    let mut pg1: Pgno = 0 as Pgno;
    /// First page of the sector pPg is located on.
    let mut n_page: i32 = 0;
    /// Number of pages starting at pg1 to journal
    let mut ii: i32 = 0;
    /// Loop counter
    let mut need_sync: i32 = 0;
    /// True if any page has PGHDR_NEED_SYNC
    let p_pager: *mut Pager = (*p_pg_1).p_pager;
    /// The pager that owns pPg
    let n_page_per_sector: Pgno =
        (unsafe { (*p_pager).sector_size } as i64 /
                unsafe { (*p_pager).page_size }) as Pgno;

    /// Set the doNotSpill NOSYNC bit to 1. This is because we cannot allow
    ///* a journal header to be written between the pages journaled by
    ///* this function.
    { let _ = 0; };
    { let _ = 0; };
    unsafe { (*p_pager).do_not_spill |= 4 as u8 };

    /// This trick assumes that both the page-size and sector-size are
    ///* an integer power of 2. It sets variable pg1 to the identifier
    ///* of the first page of the sector pPg is located on.
    (pg1 =
        ((*p_pg_1).pgno - 1 as Pgno & !(n_page_per_sector - 1 as Pgno)) +
            1 as Pgno);
    n_page_count = unsafe { (*p_pager).db_size };
    if (*p_pg_1).pgno > n_page_count {
        n_page = ((*p_pg_1).pgno - pg1 + 1 as Pgno) as i32;
    } else if pg1 + n_page_per_sector - 1 as Pgno > n_page_count {
        n_page = (n_page_count + 1 as Pgno - pg1) as i32;
    } else { n_page = n_page_per_sector as i32; }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    {
        ii = 0;
        '__b30: loop {
            if !(ii < n_page && rc == 0) { break '__b30; }
            '__c30: loop {
                let pg: Pgno = pg1 + ii as Pgno;
                let mut p_page: *mut PgHdr = core::ptr::null_mut();
                if pg == (*p_pg_1).pgno ||
                        (unsafe {
                                        sqlite3_bitvec_test(unsafe { (*p_pager).p_in_journal }, pg)
                                    } == 0) as i32 != 0 {
                    if pg != unsafe { (*p_pager).lck_pgno } {
                        rc =
                            sqlite3_pager_get(p_pager, pg,
                                &raw mut p_page as *mut *mut DbPage, 0);
                        if rc == 0 {
                            rc = pager_write(p_page);
                            if unsafe { (*p_page).flags } as i32 & 8 != 0 {
                                need_sync = 1;
                            }
                            sqlite3_pager_unref_not_null(p_page as *mut DbPage);
                        }
                    }
                } else if {
                            p_page =
                                sqlite3_pager_lookup(unsafe { &*p_pager }, pg) as
                                    *mut PgHdr;
                            p_page
                        } != core::ptr::null_mut() {
                    if unsafe { (*p_page).flags } as i32 & 8 != 0 {
                        need_sync = 1;
                    }
                    sqlite3_pager_unref_not_null(p_page as *mut DbPage);
                }
                break '__c30;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 && need_sync != 0 {
        { let _ = 0; };
        {
            ii = 0;
            '__b31: loop {
                if !(ii < n_page) { break '__b31; }
                '__c31: loop {
                    let p_page_1: *mut PgHdr =
                        sqlite3_pager_lookup(unsafe { &*p_pager }, pg1 + ii as Pgno)
                            as *mut PgHdr;
                    if !(p_page_1).is_null() {
                        unsafe { (*p_page_1).flags |= 8 as u16 };
                        sqlite3_pager_unref_not_null(p_page_1 as *mut DbPage);
                    }
                    break '__c31;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    { let _ = 0; };
    unsafe { (*p_pager).do_not_spill &= !4 as u8 };
    return rc;
}

/// Operations on page references.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_write(p_pg_1: *mut PgHdr) -> i32 {
    let p_pager: *const Pager = unsafe { (*p_pg_1).p_pager } as *const Pager;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_pg_1).flags } as i32 & 4 != 0 &&
            unsafe { (*p_pager).db_size } >= unsafe { (*p_pg_1).pgno } {
        if unsafe { (*p_pager).n_savepoint } != 0 {
            return subjournal_page_if_required(p_pg_1);
        }
        return 0;
    } else if unsafe { (*p_pager).err_code } != 0 {
        return unsafe { (*p_pager).err_code };
    } else if unsafe { (*p_pager).sector_size } >
            unsafe { (*p_pager).page_size } as u32 {
        { let _ = 0; };
        return pager_write_large_sector(unsafe { &*p_pg_1 });
    } else { return pager_write(p_pg_1); }
}

///* A call to this routine tells the pager that it is not necessary to
///* write the information on page pPg back to the disk, even though
///* that page might be marked as dirty.  This happens, for example, when
///* the page has been added as a leaf of the freelist and so its
///* content no longer matters.
///*
///* The overlying software layer calls this routine when all of the data
///* on the given page is unused. The pager marks the page as clean so
///* that it does not get written to disk.
///*
///* Tests show that this optimization can quadruple the speed of large
///* DELETE operations.
///*
///* This optimization cannot be used with a temp-file, as the page may
///* have been dirty at the start of the transaction. In that case, if
///* memory pressure forces page pPg out of the cache, the data does need
///* to be written out to disk so that it may be read back in if the
///* current transaction is rolled back.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_dont_write(p_pg_1: &mut PgHdr) -> () {
    let p_pager: *const Pager = (*p_pg_1).p_pager as *const Pager;
    if (unsafe { (*p_pager).temp_file } == 0) as i32 != 0 &&
                (*p_pg_1).flags as i32 & 2 != 0 &&
            unsafe { (*p_pager).n_savepoint } == 0 {
        (*p_pg_1).flags |= 16 as u16;
        (*p_pg_1).flags &= !4 as u16;
    }
}

///* Move the page pPg to location pgno in the file.
///*
///* There must be no references to the page previously located at
///* pgno (which we call pPgOld) though that page is allowed to be
///* in cache.  If the page previously located at pgno is not already
///* in the rollback journal, it is not put there by by this routine.
///*
///* References to the page pPg remain valid. Updating any
///* meta-data associated with pPg (i.e. data stored in the nExtra bytes
///* allocated along with the page) is the responsibility of the caller.
///*
///* A transaction must be active when this routine is called. It used to be
///* required that a statement transaction was not active, but this restriction
///* has been removed (CREATE INDEX needs to move a page when a statement
///* transaction is active).
///*
///* If the fourth argument, isCommit, is non-zero, then this page is being
///* moved as part of a database reorganization just before the transaction
///* is being committed. In this case, it is guaranteed that the database page
///* pPg refers to will not be written to again within this transaction.
///*
///* This function may return SQLITE_NOMEM or an IO error code if an error
///* occurs. Otherwise, it returns SQLITE_OK.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_movepage(p_pager_1: *mut Pager,
    p_pg_1: *mut DbPage, pgno: Pgno, is_commit_1: i32) -> i32 {
    let mut p_pg_old: *mut PgHdr = core::ptr::null_mut();
    /// The page being overwritten.
    let mut need_sync_pgno: Pgno = 0 as Pgno;
    /// Old value of pPg->pgno, if sync is required
    let mut rc: i32 = 0;
    /// Return code
    let mut orig_pgno: Pgno = 0 as Pgno;

    /// The original page number
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };

    /// In order to be able to rollback, an in-memory database must journal
    ///* the page we are moving from.
    { let _ = 0; };
    if unsafe { (*p_pager_1).temp_file } != 0 {
        rc = sqlite3_pager_write(p_pg_1 as *mut PgHdr);
        if rc != 0 { return rc; }
    }
    if unsafe { (*p_pg_1).flags } as i32 & 2 != 0 &&
            0 !=
                { rc = subjournal_page_if_required(p_pg_1 as *mut PgHdr); rc }
        {
        return rc;
    }
    if unsafe { (*p_pg_1).flags } as i32 & 8 != 0 &&
            (is_commit_1 == 0) as i32 != 0 {
        need_sync_pgno = unsafe { (*p_pg_1).pgno };
        { let _ = 0; };
        { let _ = 0; };
    }

    /// If the cache contains a page with page-number pgno, remove it
    ///* from its hash chain. Also, if the PGHDR_NEED_SYNC flag was set for
    ///* page pgno before the 'move' operation, it needs to be retained
    ///* for the page moved there.
    unsafe { (*p_pg_1).flags &= !8 as u16 };
    p_pg_old =
        sqlite3_pager_lookup(unsafe { &*p_pager_1 }, pgno) as *mut PgHdr;
    { let _ = 0; };
    if !(p_pg_old).is_null() {
        if unsafe { (*p_pg_old).n_ref } > 1 as i64 {
            sqlite3_pager_unref_not_null(p_pg_old as *mut DbPage);
            return unsafe { sqlite3_corrupt_error(7286) };
        }
        unsafe {
            (*p_pg_1).flags |=
                (unsafe { (*p_pg_old).flags } as i32 & 8) as u16
        };
        if unsafe { (*p_pager_1).temp_file } != 0 {

            /// Do not discard pages from an in-memory database since we might
            ///* need to rollback later.  Just move the page out of the way.
            unsafe {
                sqlite3_pcache_move(p_pg_old,
                    unsafe { (*p_pager_1).db_size } + 1 as Pgno)
            };
        } else { unsafe { sqlite3_pcache_drop(p_pg_old) }; }
    }
    orig_pgno = unsafe { (*p_pg_1).pgno };
    unsafe { sqlite3_pcache_move(p_pg_1 as *mut PgHdr, pgno) };
    unsafe { sqlite3_pcache_make_dirty(p_pg_1 as *mut PgHdr) };
    if unsafe { (*p_pager_1).temp_file } != 0 && !(p_pg_old).is_null() {
        unsafe { sqlite3_pcache_move(p_pg_old, orig_pgno) };
        sqlite3_pager_unref_not_null(p_pg_old as *mut DbPage);
    }
    if need_sync_pgno != 0 {
        /// If needSyncPgno is non-zero, then the journal file needs to be
        ///* sync()ed before any data is written to database file page needSyncPgno.
        ///* Currently, no such page exists in the page-cache and the
        ///* "is journaled" bitvec flag has been set. This needs to be remedied by
        ///* loading the page into the pager-cache and setting the PGHDR_NEED_SYNC
        ///* flag.
        ///*
        ///* If the attempt to load the page into the page-cache fails, (due
        ///* to a malloc() or IO failure), clear the bit in the pInJournal[]
        ///* array. Otherwise, if the page is loaded and written again in
        ///* this transaction, it may be written to the database file before
        ///* it is synced into the journal file. This way, it may end up in
        ///* the journal file twice, but that is not a problem.
        let mut p_pg_hdr: *mut PgHdr = core::ptr::null_mut();
        rc =
            sqlite3_pager_get(p_pager_1, need_sync_pgno,
                &raw mut p_pg_hdr as *mut *mut DbPage, 0);
        if rc != 0 {
            if need_sync_pgno <= unsafe { (*p_pager_1).db_orig_size } {
                { let _ = 0; };
                unsafe {
                    sqlite3_bitvec_clear(unsafe { (*p_pager_1).p_in_journal },
                        need_sync_pgno,
                        unsafe { (*p_pager_1).p_tmp_space } as *mut ())
                };
            }
            return rc;
        }
        unsafe { (*p_pg_hdr).flags |= 8 as u16 };
        unsafe { sqlite3_pcache_make_dirty(p_pg_hdr) };
        sqlite3_pager_unref_not_null(p_pg_hdr as *mut DbPage);
    }
    return 0;
}

///* Return the number of references to the specified page.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_page_refcount(p_page_1: *mut DbPage) -> i32 {
    return unsafe { sqlite3_pcache_page_refcount(p_page_1 as *mut PgHdr) } as
            i32;
}

///* Return a pointer to the data for the specified page.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_get_data(p_pg_1: &DbPage) -> *mut () {
    { let _ = 0; };
    return (*p_pg_1).p_data;
}

///* Return a pointer to the Pager.nExtra bytes of "extra" space
///* allocated along with the specified page.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_get_extra(p_pg_1: &DbPage) -> *mut () {
    return (*p_pg_1).p_extra;
}

/// Functions used to manage pager transactions and savepoints.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_pagecount(p_pager_1: &Pager,
    pn_page_1: &mut i32) -> () {
    { let _ = 0; };
    { let _ = 0; };
    *pn_page_1 = (*p_pager_1).db_size as i32;
}

///* Begin a write-transaction on the specified pager object. If a
///* write-transaction has already been opened, this function is a no-op.
///*
///* If the exFlag argument is false, then acquire at least a RESERVED
///* lock on the database file. If exFlag is true, then acquire at least
///* an EXCLUSIVE lock. If such a lock is already held, no locking
///* functions need be called.
///*
///* If the subjInMemory argument is non-zero, then any sub-journal opened
///* within this transaction will be opened as an in-memory file. This
///* has no effect if the sub-journal is already opened (as it may be when
///* running in exclusive mode) or if the transaction does not require a
///* sub-journal. If the subjInMemory argument is zero, then any required
///* sub-journal is implemented in-memory if pPager is an in-memory database,
///* or using a temporary file otherwise.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_begin(p_pager_1: *mut Pager, ex_flag_1: i32,
    subj_in_memory_1: i32) -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*p_pager_1).err_code } != 0 {
        return unsafe { (*p_pager_1).err_code };
    }
    { let _ = 0; };
    unsafe { (*p_pager_1).subj_in_memory = subj_in_memory_1 as u8 };
    if unsafe { (*p_pager_1).e_state } as i32 == 1 {
        { let _ = 0; };
        if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
            if unsafe { (*p_pager_1).exclusive_mode } != 0 &&
                    unsafe {
                            sqlite3_wal_exclusive_mode(unsafe { (*p_pager_1).p_wal },
                                -1)
                        } != 0 {
                rc = pager_lock_db(unsafe { &mut *p_pager_1 }, 4);
                if rc != 0 { return rc; }
                {
                    let _ =
                        unsafe {
                            sqlite3_wal_exclusive_mode(unsafe { (*p_pager_1).p_wal }, 1)
                        };
                };
            }

            /// Grab the write lock on the log file. If successful, upgrade to
            ///* PAGER_RESERVED state. Otherwise, return an error code to the caller.
            ///* The busy-handler is not invoked if another connection already
            ///* holds the write-lock. If possible, the upper layer will call it.
            (rc =
                unsafe {
                    sqlite3_wal_begin_write_transaction(unsafe {
                            (*p_pager_1).p_wal
                        })
                });
        } else {

            /// Obtain a RESERVED lock on the database file. If the exFlag parameter
            ///* is true, then immediately upgrade this to an EXCLUSIVE lock. The
            ///* busy-handler callback can be used when upgrading to the EXCLUSIVE
            ///* lock, but not when obtaining the RESERVED lock.
            (rc = pager_lock_db(unsafe { &mut *p_pager_1 }, 2));
            if rc == 0 && ex_flag_1 != 0 {
                rc = pager_wait_on_lock(p_pager_1, 4);
            }
        }
        if rc == 0 {

            /// Change to WRITER_LOCKED state.
            ///*
            ///* WAL mode sets Pager.eState to PAGER_WRITER_LOCKED or CACHEMOD
            ///* when it has an open transaction, but never to DBMOD or FINISHED.
            ///* This is because in those states the code to roll back savepoint
            ///* transactions may copy data from the sub-journal into the database
            ///* file as well as into the page cache. Which would be incorrect in
            ///* WAL mode.
            unsafe { (*p_pager_1).e_state = 2 as u8 };
            unsafe {
                (*p_pager_1).db_hint_size = unsafe { (*p_pager_1).db_size }
            };
            unsafe {
                (*p_pager_1).db_file_size = unsafe { (*p_pager_1).db_size }
            };
            unsafe {
                (*p_pager_1).db_orig_size = unsafe { (*p_pager_1).db_size }
            };
            unsafe { (*p_pager_1).journal_off = 0 as i64 };
        }
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
    }
    return rc;
}

///* This routine is called to increment the value of the database file
///* change-counter, stored as a 4-byte big-endian integer starting at
///* byte offset 24 of the pager file.  The secondary change counter at
///* 92 is also updated, as is the SQLite version number at offset 96.
///*
///* But this only happens if the pPager->changeCountDone flag is false.
///* To avoid excess churning of page 1, the update only happens once.
///* See also the pager_write_changecounter() routine that does an
///* unconditional update of the change counters.
///*
///* If the isDirectMode flag is zero, then this is done by calling
///* sqlite3PagerWrite() on page 1, then modifying the contents of the
///* page data. In this case the file will be updated when the current
///* transaction is committed.
///*
///* The isDirectMode flag may only be non-zero if the library was compiled
///* with the SQLITE_ENABLE_ATOMIC_WRITE macro defined. In this case,
///* if isDirect is non-zero, then the database file is updated directly
///* by writing an updated version of page 1 using a call to the
///* sqlite3OsWrite() function.
#[allow(unused_doc_comments)]
extern "C" fn pager_incr_changecounter(p_pager_1: *mut Pager,
    is_direct_mode_1: i32) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };

    /// Declare and initialize constant integer 'isDirect'. If the
    ///* atomic-write optimization is enabled in this build, then isDirect
    ///* is initialized to the value passed as the isDirectMode parameter
    ///* to this function. Otherwise, it is always set to zero.
    ///*
    ///* The idea is that if the atomic-write optimization is not
    ///* enabled at compile time, the compiler can omit the tests of
    ///* 'isDirect' below, as well as the block enclosed in the
    ///* "if( isDirect )" condition.
    { let _ = 0; };
    { let _ = is_direct_mode_1; };
    if (unsafe { (*p_pager_1).change_count_done } == 0) as i32 != 0 &&
            unsafe { (*p_pager_1).db_size } > 0 as u32 {
        let mut p_pg_hdr: *mut PgHdr = core::ptr::null_mut();

        /// Reference to page 1
        { let _ = 0; };

        /// Open page 1 of the file for writing.
        (rc =
            sqlite3_pager_get(p_pager_1, 1 as Pgno,
                &raw mut p_pg_hdr as *mut *mut DbPage, 0));
        { let _ = 0; };
        if (0 == 0) as i32 != 0 && rc == 0 {
            rc = sqlite3_pager_write(p_pg_hdr);
        }
        if rc == 0 {

            /// Actually do the update of the change counter
            pager_write_changecounter(p_pg_hdr);
            if 0 != 0 {
                let mut z_buf: *const () = core::ptr::null();
                { let _ = 0; };
                z_buf = unsafe { (*p_pg_hdr).p_data } as *const ();
                if rc == 0 {
                    rc =
                        unsafe {
                            sqlite3_os_write(unsafe { (*p_pager_1).fd }, z_buf,
                                unsafe { (*p_pager_1).page_size } as i32, 0 as i64)
                        };
                    {
                        let __p = unsafe { &mut (*p_pager_1).a_stat[2 as usize] };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
                if rc == 0 {
                    /// Update the pager's copy of the change-counter. Otherwise, the
                    ///* next time a read transaction is opened the cache will be
                    ///* flushed (as the change-counter values will not match).
                    let p_copy: *const () =
                        unsafe {
                                &raw const *(z_buf as *const i8).offset(24 as isize)
                            } as *const ();
                    unsafe {
                        memcpy(unsafe { &raw mut (*p_pager_1).db_file_vers } as
                                *mut (), p_copy, core::mem::size_of::<[i8; 16]>() as u64)
                    };
                    unsafe { (*p_pager_1).change_count_done = 1 as u8 };
                }
            } else { unsafe { (*p_pager_1).change_count_done = 1 as u8 }; }
        }

        /// Release the page reference.
        sqlite3_pager_unref(p_pg_hdr as *mut DbPage);
    }
    return rc;
}

///* Write the supplied super-journal name into the journal file for pager
///* pPager at the current location. The super-journal name must be the last
///* thing written to a journal file. If the pager is in full-sync mode, the
///* journal file descriptor is advanced to the next sector boundary before
///* anything is written. The format is:
///*
///*   + 4 bytes: PAGER_SJ_PGNO.
///*   + N bytes: super-journal filename in utf-8.
///*   + 4 bytes: N (length of super-journal name in bytes, no nul-terminator).
///*   + 4 bytes: super-journal name checksum.
///*   + 8 bytes: aJournalMagic[].
///*
///* The super-journal page checksum is the sum of the bytes in the super-journal
///* name, where each byte is interpreted as a signed 8-bit integer.
///*
///* If zSuper is a NULL pointer (occurs for a single database transaction),
///* this call is a no-op.
#[allow(unused_doc_comments)]
extern "C" fn write_super_journal(p_pager_1: *mut Pager, z_super_1: *const i8)
    -> i32 {
    let mut rc: i32 = 0;
    /// Return code
    let mut n_super: i32 = 0;
    /// Length of string zSuper
    let mut i_hdr_off: i64 = 0 as i64;
    /// Offset of header in journal file
    let mut jrnl_size: i64 = 0 as i64;
    /// Size of journal file on disk
    let mut cksum: u32 = 0 as u32;

    /// Checksum of string zSuper
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if (z_super_1).is_null() as i32 != 0 ||
                unsafe { (*p_pager_1).journal_mode } as i32 == 4 ||
            !(unsafe { (*unsafe { (*p_pager_1).jfd }).p_methods } !=
                            core::ptr::null()) as i32 != 0 {
        return 0;
    }
    unsafe { (*p_pager_1).set_super = 1 as u8 };
    { let _ = 0; };
    {
        n_super = 0;
        '__b32: loop {
            if !(unsafe { *z_super_1.offset(n_super as isize) } != 0) {
                break '__b32;
            }
            '__c32: loop {
                cksum +=
                    unsafe { *z_super_1.offset(n_super as isize) } as u32;
                break '__c32;
            }
            { let __p = &mut n_super; let __t = *__p; *__p += 1; __t };
        }
    }
    if unsafe { (*p_pager_1).full_sync } != 0 {
        unsafe {
            (*p_pager_1).journal_off =
                journal_hdr_offset(unsafe { &*p_pager_1 })
        };
    }
    i_hdr_off = unsafe { (*p_pager_1).journal_off };
    if 0 !=
                            {
                                rc =
                                    write32bits(unsafe { (*p_pager_1).jfd }, i_hdr_off,
                                        unsafe { (*p_pager_1).lck_pgno });
                                rc
                            } ||
                        0 !=
                            {
                                rc =
                                    unsafe {
                                        sqlite3_os_write(unsafe { (*p_pager_1).jfd },
                                            z_super_1 as *const (), n_super, i_hdr_off + 4 as i64)
                                    };
                                rc
                            } ||
                    0 !=
                        {
                            rc =
                                write32bits(unsafe { (*p_pager_1).jfd },
                                    i_hdr_off + 4 as i64 + n_super as i64, n_super as u32);
                            rc
                        } ||
                0 !=
                    {
                        rc =
                            write32bits(unsafe { (*p_pager_1).jfd },
                                i_hdr_off + 4 as i64 + n_super as i64 + 4 as i64, cksum);
                        rc
                    } ||
            0 !=
                {
                    rc =
                        unsafe {
                            sqlite3_os_write(unsafe { (*p_pager_1).jfd },
                                &raw const a_journal_magic[0 as usize] as *const u8 as
                                    *const (), 8,
                                i_hdr_off + 4 as i64 + n_super as i64 + 8 as i64)
                        };
                    rc
                } {
        return rc;
    }
    unsafe { (*p_pager_1).journal_off += (n_super + 20) as i64 };
    if 0 ==
                {
                    rc =
                        unsafe {
                            sqlite3_os_file_size(unsafe { (*p_pager_1).jfd },
                                &mut jrnl_size)
                        };
                    rc
                } && jrnl_size > unsafe { (*p_pager_1).journal_off } {
        rc =
            unsafe {
                sqlite3_os_truncate(unsafe { (*p_pager_1).jfd },
                    unsafe { (*p_pager_1).journal_off })
            };
    }
    return rc;
}

///* Sync the database file for the pager pPager. zSuper points to the name
///* of a super-journal file that should be written into the individual
///* journal file. zSuper may be NULL, which is interpreted as no
///* super-journal (a single database transaction).
///*
///* This routine ensures that:
///*
///*   * The database file change-counter is updated,
///*   * the journal is synced (unless the atomic-write optimization is used),
///*   * all dirty pages are written to the database file,
///*   * the database file is truncated (if required), and
///*   * the database file synced.
///*
///* The only thing that remains to commit the transaction is to finalize
///* (delete, truncate or zero the first part of) the journal file (or
///* delete the super-journal file if specified).
///*
///* Note that if zSuper==NULL, this does not overwrite a previous value
///* passed to an sqlite3PagerCommitPhaseOne() call.
///*
///* If the final parameter - noSync - is true, then the database file itself
///* is not synced. The caller must call sqlite3PagerSync() directly to
///* sync the database file before calling CommitPhaseTwo() to delete the
///* journal file in this case.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_commit_phase_one(p_pager_1: *mut Pager,
    z_super_1: *const i8, no_sync_1: i32) -> i32 {
    let mut rc: i32 = 0;
    '__b33: loop {
        '__c33: loop {

            /// Return code
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*p_pager_1).err_code } != 0 {
                return unsafe { (*p_pager_1).err_code };
            }
            if unsafe { sqlite3_fault_sim(400) } != 0 { return 10; }
            if (unsafe { (*p_pager_1).e_state } as i32) < 3 { return 0; }
            { let _ = 0; };
            { let _ = 0; };
            if 0 == pager_flush_on_commit(unsafe { &*p_pager_1 }, 1) {

                /// If this is an in-memory db, or no pages have been written to, or this
                ///* function has already been called, it is mostly a no-op.  However, any
                ///* backup in progress needs to be restarted.
                unsafe {
                    sqlite3_backup_restart(unsafe { (*p_pager_1).p_backup })
                };
            } else {
                let mut p_list: *mut PgHdr = core::ptr::null_mut();
                if unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut() {
                    let mut p_page_one: *mut PgHdr = core::ptr::null_mut();
                    p_list =
                        unsafe {
                            sqlite3_pcache_dirty_list(unsafe { (*p_pager_1).p_p_cache })
                        };
                    if p_list == core::ptr::null_mut() {

                        /// Must have at least one page for the WAL commit flag.
                        ///* Ticket [2d1a5c67dfc2363e44f29d9bbd57f] 2011-05-18
                        (rc =
                            sqlite3_pager_get(p_pager_1, 1 as Pgno,
                                &raw mut p_page_one as *mut *mut DbPage, 0));
                        p_list = p_page_one;
                        unsafe { (*p_list).p_dirty = core::ptr::null_mut() };
                    }
                    { let _ = 0; };
                    if !(p_list).is_null() {
                        rc =
                            pager_wal_frames(unsafe { &mut *p_pager_1 }, p_list,
                                unsafe { (*p_pager_1).db_size }, 1);
                    }
                    sqlite3_pager_unref(p_page_one as *mut DbPage);
                    if rc == 0 {
                        unsafe {
                            sqlite3_pcache_clean_all(unsafe { (*p_pager_1).p_p_cache })
                        };
                    }
                } else {

                    /// The bBatch boolean is true if the batch-atomic-write commit method
                    ///* should be used.  No rollback journal is created if batch-atomic-write
                    ///* is enabled.
                    /// SQLITE_ENABLE_ATOMIC_WRITE
                    (rc = pager_incr_changecounter(p_pager_1, 0));
                    if rc != 0 { break '__b33; }

                    /// Write the super-journal name into the journal file. If a
                    ///* super-journal file name has already been written to the journal file,
                    ///* or if zSuper is NULL (no super-journal), then this call is a no-op.
                    (rc = write_super_journal(p_pager_1, z_super_1));
                    if rc != 0 { break '__b33; }

                    /// Sync the journal file and write all dirty pages to the database.
                    ///* If the atomic-update optimization is being used, this sync will not
                    ///* create the journal file or perform any real IO.
                    ///*
                    ///* Because the change-counter page was just modified, unless the
                    ///* atomic-update optimization is used it is almost certain that the
                    ///* journal requires a sync here. However, in locking_mode=exclusive
                    ///* on a system under memory pressure it is just possible that this is
                    ///* not the case. In this case it is likely enough that the redundant
                    ///* xSync() call will be changed to a no-op by the OS anyhow.
                    (rc = sync_journal(p_pager_1, 0));
                    if rc != 0 { break '__b33; }
                    p_list =
                        unsafe {
                            sqlite3_pcache_dirty_list(unsafe { (*p_pager_1).p_p_cache })
                        };
                    if 0 == 0 { rc = pager_write_pagelist(p_pager_1, p_list); }
                    if rc != 0 { { let _ = 0; }; break '__b33; }
                    unsafe {
                        sqlite3_pcache_clean_all(unsafe { (*p_pager_1).p_p_cache })
                    };
                    if unsafe { (*p_pager_1).db_size } >
                            unsafe { (*p_pager_1).db_file_size } {
                        let n_new: Pgno =
                            unsafe { (*p_pager_1).db_size } -
                                (unsafe { (*p_pager_1).db_size } ==
                                        unsafe { (*p_pager_1).lck_pgno }) as Pgno;
                        { let _ = 0; };
                        rc = pager_truncate(unsafe { &mut *p_pager_1 }, n_new);
                        if rc != 0 { break '__b33; }
                    }
                    if (no_sync_1 == 0) as i32 != 0 {
                        rc = sqlite3_pager_sync(unsafe { &*p_pager_1 }, z_super_1);
                    }
                }
            }
            break '__c33;
        }
        if !(false) { break '__b33; }
    }
    if rc == 0 &&
            !(unsafe { (*p_pager_1).p_wal } != core::ptr::null_mut()) as i32
                != 0 {
        unsafe { (*p_pager_1).e_state = 5 as u8 };
    }
    return rc;
}

///* When this function is called, the database file has been completely
///* updated to reflect the changes made by the current transaction and
///* synced to disk. The journal file still exists in the file-system
///* though, and if a failure occurs at this point it will eventually
///* be used as a hot-journal and the current transaction rolled back.
///*
///* This function finalizes the journal file, either by deleting,
///* truncating or partially zeroing it, so that it cannot be used
///* for hot-journal rollback. Once this is done the transaction is
///* irrevocably committed.
///*
///* If an error occurs, an IO error code is returned and the pager
///* moves into the error state. Otherwise, SQLITE_OK is returned.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_commit_phase_two(p_pager_1: *mut Pager)
    -> i32 {
    let mut rc: i32 = 0;
    if unsafe { (*p_pager_1).err_code } != 0 {
        return unsafe { (*p_pager_1).err_code };
    }
    {
        let __p = unsafe { &mut (*p_pager_1).i_data_version };
        let __t = *__p;
        *__p += 1;
        __t
    };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_pager_1).e_state } as i32 == 2 &&
                unsafe { (*p_pager_1).exclusive_mode } != 0 &&
            unsafe { (*p_pager_1).journal_mode } as i32 == 1 {
        { let _ = 0; };
        unsafe { (*p_pager_1).e_state = 1 as u8 };
        return 0;
    }
    rc =
        pager_end_transaction(p_pager_1,
            unsafe { (*p_pager_1).set_super } as i32, 1);
    return pager_error(p_pager_1, rc);
}

///* Check that there are at least nSavepoint savepoints open. If there are
///* currently less than nSavepoints open, then open one or more savepoints
///* to make up the difference. If the number of savepoints is already
///* equal to nSavepoint, then this function is a no-op.
///*
///* If a memory allocation fails, SQLITE_NOMEM is returned. If an error
///* occurs while opening the sub-journal file, then an IO error code is
///* returned. Otherwise, SQLITE_OK.
#[allow(unused_doc_comments)]
extern "C" fn pager_open_savepoint(p_pager_1: &mut Pager, n_savepoint_1: i32)
    -> i32 {
    let rc: i32 = 0;
    /// Return code
    let n_current: i32 = (*p_pager_1).n_savepoint;
    /// Current number of savepoints
    let mut ii: i32 = 0;
    /// Iterator variable
    let mut a_new: *mut PagerSavepoint = core::ptr::null_mut();

    /// New Pager.aSavepoint array
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };

    /// Grow the Pager.aSavepoint array using realloc(). Return SQLITE_NOMEM
    ///* if the allocation fails. Otherwise, zero the new portion in case a
    ///* malloc failure occurs while populating it in the for(...) loop below.
    (a_new =
        unsafe {
                sqlite3Realloc((*p_pager_1).a_savepoint as *mut (),
                    core::mem::size_of::<PagerSavepoint>() as u64 *
                        n_savepoint_1 as u64)
            } as *mut PagerSavepoint);
    if (a_new).is_null() as i32 != 0 { return 7; }
    unsafe {
        memset(unsafe { &raw mut *a_new.offset(n_current as isize) } as
                *mut (), 0,
            (n_savepoint_1 - n_current) as u64 *
                core::mem::size_of::<PagerSavepoint>() as u64)
    };
    (*p_pager_1).a_savepoint = a_new;
    {
        ii = n_current;
        '__b34: loop {
            if !(ii < n_savepoint_1) { break '__b34; }
            '__c34: loop {
                unsafe {
                    (*a_new.offset(ii as isize)).n_orig = (*p_pager_1).db_size
                };
                if unsafe { (*(*p_pager_1).jfd).p_methods } !=
                            core::ptr::null() && (*p_pager_1).journal_off > 0 as i64 {
                    unsafe {
                        (*a_new.offset(ii as isize)).i_offset =
                            (*p_pager_1).journal_off
                    };
                } else {
                    unsafe {
                        (*a_new.offset(ii as isize)).i_offset =
                            (*p_pager_1).sector_size as i64
                    };
                }
                unsafe {
                    (*a_new.offset(ii as isize)).i_sub_rec =
                        (*p_pager_1).n_sub_rec
                };
                unsafe {
                    (*a_new.offset(ii as isize)).p_in_savepoint =
                        unsafe { sqlite3_bitvec_create((*p_pager_1).db_size) }
                };
                unsafe {
                    (*a_new.offset(ii as isize)).b_truncate_on_release = 1
                };
                if (unsafe {
                                        (*a_new.offset(ii as isize)).p_in_savepoint
                                    }).is_null() as i32 != 0 {
                    return 7;
                }
                if (*p_pager_1).p_wal != core::ptr::null_mut() {
                    unsafe {
                        sqlite3_wal_savepoint((*p_pager_1).p_wal,
                            unsafe {
                                    &raw mut (*a_new.offset(ii as isize)).a_wal_data[0 as usize]
                                } as *mut u32)
                    };
                }
                (*p_pager_1).n_savepoint = ii + 1;
                break '__c34;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_open_savepoint(p_pager_1: *mut Pager,
    n_savepoint_1: i32) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    if n_savepoint_1 > unsafe { (*p_pager_1).n_savepoint } &&
            unsafe { (*p_pager_1).use_journal } != 0 {
        return pager_open_savepoint(unsafe { &mut *p_pager_1 },
                n_savepoint_1);
    } else { return 0; }
}

///* This function is called when the user invokes "PRAGMA wal_checkpoint",
///* "PRAGMA wal_blocking_checkpoint" or calls the sqlite3_wal_checkpoint()
///* or wal_blocking_checkpoint() API functions.
///*
///* Parameter eMode is one of SQLITE_CHECKPOINT_PASSIVE, FULL or RESTART.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_pager_checkpoint(p_pager_1: &mut Pager,
    db: *mut Sqlite3, e_mode_1: i32, pn_log_1: *mut i32, pn_ckpt_1: *mut i32)
    -> i32 {
    let mut rc: i32 = 0;
    if (*p_pager_1).p_wal == core::ptr::null_mut() &&
            (*p_pager_1).journal_mode as i32 == 5 {

        /// This only happens when a database file is zero bytes in size opened and
        ///* then "PRAGMA journal_mode=WAL" is run and then sqlite3_wal_checkpoint()
        ///* is invoked without any intervening transactions.  We need to start
        ///* a transaction to initialize pWal.  The PRAGMA table_list statement is
        ///* used for this since it starts transactions on every database file,
        ///* including all ATTACHed databases.  This seems expensive for a single
        ///* sqlite3_wal_checkpoint() call, but it happens very rarely.
        ///* https://sqlite.org/forum/forumpost/fd0f19d229156939
        unsafe {
            sqlite3_exec(db,
                c"PRAGMA table_list".as_ptr() as *mut i8 as *const i8, None,
                core::ptr::null_mut(), core::ptr::null_mut())
        };
    }
    if !((*p_pager_1).p_wal).is_null() {
        rc =
            unsafe {
                sqlite3WalCheckpoint((*p_pager_1).p_wal, db, e_mode_1,
                    Some(if e_mode_1 <= 0 {
                            unsafe {
                                core::mem::transmute::<*const (),
                                        unsafe extern "C" fn(*mut ()) -> i32>(0 as *const ())
                            }
                        } else { (*p_pager_1).x_busy_handler }),
                    (*p_pager_1).p_busy_handler_arg,
                    (*p_pager_1).wal_sync_flags as i32,
                    (*p_pager_1).page_size as i32,
                    (*p_pager_1).p_tmp_space as *mut u8, pn_log_1, pn_ckpt_1)
            };
    }
    return rc;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_wal_callback(p_pager_1: &Pager) -> i32 {
    return unsafe { sqlite3_wal_callback((*p_pager_1).p_wal) };
}

///* This function is called to close the connection to the log file prior
///* to switching from WAL to rollback mode.
///*
///* Before closing the log file, this function attempts to take an
///* EXCLUSIVE lock on the database file. If this cannot be obtained, an
///* error (SQLITE_BUSY) is returned and the log connection is not closed.
///* If successful, the EXCLUSIVE lock is not released before returning.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_close_wal(p_pager_1: *mut Pager,
    db: *mut Sqlite3) -> i32 {
    let mut rc: i32 = 0;
    { let _ = 0; };
    if (unsafe { (*p_pager_1).p_wal }).is_null() as i32 != 0 {
        let mut logexists: i32 = 0;
        rc = pager_lock_db(unsafe { &mut *p_pager_1 }, 1);
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_os_access(unsafe { (*p_pager_1).p_vfs },
                        unsafe { (*p_pager_1).z_wal } as *const i8, 0,
                        &mut logexists)
                };
        }
        if rc == 0 && logexists != 0 { rc = pager_open_wal(p_pager_1); }
    }
    if rc == 0 && !(unsafe { (*p_pager_1).p_wal }).is_null() {
        rc = pager_exclusive_lock(p_pager_1);
        if rc == 0 {
            rc =
                unsafe {
                    sqlite3_wal_close(unsafe { (*p_pager_1).p_wal }, db,
                        unsafe { (*p_pager_1).wal_sync_flags } as i32,
                        unsafe { (*p_pager_1).page_size } as i32,
                        unsafe { (*p_pager_1).p_tmp_space } as *mut u8)
                };
            unsafe { (*p_pager_1).p_wal = core::ptr::null_mut() };
            pager_fix_maplimit(p_pager_1);
            if rc != 0 &&
                    (unsafe { (*p_pager_1).exclusive_mode } == 0) as i32 != 0 {
                pager_unlock_db(unsafe { &mut *p_pager_1 }, 1);
            }
        }
    }
    return rc;
}

///* Return true if page pgno can be read directly from the database file
///* by the b-tree layer. This is the case if:
///*
///*   (1)  the database file is open
///*   (2)  the VFS for the database is able to do unaligned sub-page reads
///*   (3)  there are no dirty pages in the cache, and
///*   (4)  the desired page is not currently in the wal file.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_direct_read_ok(p_pager_1: &Pager, pgno: Pgno)
    -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*(*p_pager_1).fd).p_methods } == core::ptr::null() {
        return 0;
    }
    if unsafe { sqlite3_p_cache_is_dirty((*p_pager_1).p_p_cache) } != 0 {
        return 0;
    }
    if !((*p_pager_1).p_wal).is_null() {
        let mut i_read: u32 = 0 as u32;
        {
            let _ =
                unsafe {
                    sqlite3_wal_find_frame((*p_pager_1).p_wal, pgno,
                        &mut i_read)
                };
        };
        if i_read != 0 { return 0; }
    }
    { let _ = 0; };
    if unsafe {
                    (unsafe {
                            (*unsafe {
                                                (*(*p_pager_1).fd).p_methods
                                            }).x_device_characteristics.unwrap()
                        })((*p_pager_1).fd)
                } & 32768 == 0 {
        return 0;
    }
    return 1;
}

/// Functions used to query pager state and configuration.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_isreadonly(p_pager_1: &Pager) -> u8 {
    return (*p_pager_1).read_only;
}

///* Return the pPager->iDataVersion value
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_data_version(p_pager_1: &Pager) -> u32 {
    return (*p_pager_1).i_data_version;
}

///* Return the approximate number of bytes of memory currently
///* used by the pager and its associated cache.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_mem_used(p_pager_1: *const Pager) -> i32 {
    let per_page_size: i32 =
        (unsafe { (*p_pager_1).page_size } +
                    unsafe { (*p_pager_1).n_extra } as i64 +
                (core::mem::size_of::<PgHdr>() as u64 +
                            5 as u64 * core::mem::size_of::<*mut ()>() as u64) as i32 as
                    i64) as i32;
    return ((per_page_size *
                            unsafe {
                                sqlite3_pcache_pagecount(unsafe { (*p_pager_1).p_p_cache })
                            } + unsafe { sqlite3_malloc_size(p_pager_1 as *const ()) })
                    as i64 + unsafe { (*p_pager_1).page_size }) as i32;
}

///* Return the full pathname of the database file.
///*
///* Except, if the pager is in-memory only, then return an empty string if
///* nullIfMemDb is true.  This routine is called with nullIfMemDb==1 when
///* used to report the filename to the user, for compatibility with legacy
///* behavior.  But when the Btree needs to know the filename for matching to
///* shared cache, it uses nullIfMemDb==0 so that in-memory databases can
///* participate in shared-cache.
///*
///* The return value to this routine is always safe to use with
///* sqlite3_uri_parameter() and sqlite3_filename_database() and friends.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_filename(p_pager_1: &Pager,
    null_if_mem_db_1: i32) -> *const i8 {
    if null_if_mem_db_1 != 0 &&
            ((*p_pager_1).mem_db != 0 ||
                unsafe {
                        sqlite3_is_memdb((*p_pager_1).p_vfs as *const Sqlite3Vfs)
                    } != 0) {
        return &z_fake[4 as usize];
    } else { return (*p_pager_1).z_filename as *const i8; }
}

///* Return the VFS structure for the pager.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_vfs(p_pager_1: &Pager) -> *mut Sqlite3Vfs {
    return (*p_pager_1).p_vfs;
}

///* Return the file handle for the database file associated
///* with the pager.  This might return NULL if the file has
///* not yet been opened.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_file(p_pager_1: &Pager) -> *mut Sqlite3File {
    return (*p_pager_1).fd;
}

///* Return the file handle for the journal file (if it exists).
///* This will be either the rollback journal or the WAL file.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_jrnl_file(p_pager_1: &Pager)
    -> *mut Sqlite3File {
    return if !((*p_pager_1).p_wal).is_null() {
            unsafe { sqlite3_wal_file((*p_pager_1).p_wal) }
        } else { (*p_pager_1).jfd };
}

///* Return the full pathname of the journal file.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_journalname(p_pager_1: &Pager) -> *const i8 {
    return (*p_pager_1).z_journal as *const i8;
}

///* Return a pointer to the "temporary page" buffer held internally
///* by the pager.  This is a buffer that is big enough to hold the
///* entire content of a database page.  This buffer is used internally
///* during rollback and will be overwritten whenever a rollback
///* occurs.  But other modules are free to use it too, as long as
///* no rollbacks are happening.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_temp_space(p_pager_1: &Pager) -> *mut () {
    return (*p_pager_1).p_tmp_space as *mut ();
}

///* Return true if this is an in-memory or temp-file backed pager.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_is_memdb(p_pager_1: &Pager) -> i32 {
    return ((*p_pager_1).temp_file != 0 || (*p_pager_1).mem_vfs != 0) as i32;
}

///* Parameter eStat must be one of SQLITE_DBSTATUS_CACHE_HIT, _MISS, _WRITE,
///* or _WRITE+1.  The SQLITE_DBSTATUS_CACHE_WRITE+1 case is a translation
///* of SQLITE_DBSTATUS_CACHE_SPILL.  The _SPILL case is not contiguous because
///* it was added later.
///*
///* Before returning, *pnVal is incremented by the
///* current cache hit or miss count, according to the value of eStat. If the
///* reset parameter is non-zero, the cache hit or miss count is zeroed before
///* returning.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_cache_stat(p_pager_1: &mut Pager,
    mut e_stat_1: i32, reset: i32, pn_val_1: &mut u64) -> () {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    e_stat_1 -= 7;
    *pn_val_1 += (*p_pager_1).a_stat[e_stat_1 as usize] as u64;
    if reset != 0 { (*p_pager_1).a_stat[e_stat_1 as usize] = 0 as u32; }
}

///* Unless this is an in-memory or temporary database, clear the pager cache.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_clear_cache(p_pager_1: *mut Pager) -> () {
    { let _ = 0; };
    if unsafe { (*p_pager_1).temp_file } as i32 == 0 {
        pager_reset(unsafe { &mut *p_pager_1 });
    }
}

/// Functions used to truncate the database file.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_truncate_image(p_pager_1: &mut Pager,
    n_page_1: Pgno) -> () {
    { let _ = 0; };
    { let _ = 0; };
    (*p_pager_1).db_size = n_page_1;
}

///* The page handle passed as the first argument refers to a dirty page
///* with a page number other than iNew. This function changes the page's
///* page number to iNew and sets the value of the PgHdr.flags field to
///* the value passed as the third parameter.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_pager_rekey(p_pg_1: *mut DbPage, i_new_1: Pgno,
    flags: u16) -> () {
    { let _ = 0; };
    unsafe { (*p_pg_1).flags = flags };
    unsafe { sqlite3_pcache_move(p_pg_1 as *mut PgHdr, i_new_1) };
}

static zero_hdr: [i8; 28] =
    [0 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0];

static zerobyte: u8 = 0 as u8;

static z_fake: [i8; 8] =
    [0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8];

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
    fn sqlite3_pcache_size()
    -> i32;
    fn sqlite3_journal_size(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_db_str_dup(_: *mut Sqlite3, _: *const i8)
    -> *mut i8;
    fn sqlite3_strlen30(_: *const i8)
    -> i32;
    fn sqlite3_db_malloc_raw(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_cantopen_error(_: i32)
    -> i32;
    fn sqlite3_db_free(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_malloc_zero(_: u64)
    -> *mut ();
    fn sqlite3_pcache_ref_count(_: *mut PCache)
    -> i64;
    fn sqlite3_page_malloc(_: i32)
    -> *mut ();
    fn sqlite3_backup_restart(_: *mut Sqlite3Backup)
    -> ();
    fn sqlite3_pcache_clear(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_set_page_size(_: *mut PCache, _: i32)
    -> i32;
    fn sqlite3_page_free(_: *mut ())
    -> ();
    static mut sqlite3_pending_byte: i32;
    fn sqlite3_corrupt_error(_: i32)
    -> i32;
    fn sqlite3_wal_find_frame(_: *mut Wal, _: Pgno, _: *mut u32)
    -> i32;
    fn sqlite3_pcache_fetch(_: *mut PCache, _: Pgno, create_flag_1: i32)
    -> *mut Sqlite3PcachePage;
    fn sqlite3_pcache_fetch_finish(_: *mut PCache, _: Pgno,
    p_page_1: *mut Sqlite3PcachePage)
    -> *mut PgHdr;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_pcache_fetch_stress(_: *mut PCache, _: Pgno,
    _: *mut *mut Sqlite3PcachePage)
    -> i32;
    fn sqlite3_pcache_release(_: *mut PgHdr)
    -> ();
    fn sqlite3_begin_benign_malloc()
    -> ();
    fn sqlite3_bitvec_set(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_end_benign_malloc()
    -> ();
    fn sqlite3_wal_read_frame(_: *mut Wal, _: u32, _: i32, _: *mut u8)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_pcache_drop(_: *mut PgHdr)
    -> ();
    fn sqlite3_bitvec_destroy(_: *mut Bitvec)
    -> ();
    fn sqlite3_journal_is_in_memory(p: *mut Sqlite3File)
    -> i32;
    fn sqlite3_bitvec_create(_: u32)
    -> *mut Bitvec;
    fn sqlite3_wal_undo(p_wal_1: *mut Wal,
    x_undo_1: Option<unsafe extern "C" fn(*mut (), u32) -> i32>,
    p_undo_ctx_1: *mut ())
    -> i32;
    fn sqlite3_pcache_page_refcount(_: *mut PgHdr)
    -> i64;
    fn sqlite3_pcache_dirty_list(_: *mut PCache)
    -> *mut PgHdr;
    fn sqlite3_get4byte(_: *const u8)
    -> u32;
    fn sqlite3_bitvec_test(_: *mut Bitvec, _: u32)
    -> i32;
    fn sqlite3_backup_update(_: *mut Sqlite3Backup, _: Pgno, _: *const u8)
    -> ();
    fn sqlite3_pcache_make_dirty(_: *mut PgHdr)
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn sqlite3_wal_savepoint_undo(p_wal_1: *mut Wal, a_wal_data_1: *mut u32)
    -> i32;
    fn sqlite3_p_cache_percent_dirty(_: *mut PCache)
    -> i32;
    fn sqlite3_pcache_clean_all(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_clear_writable(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_truncate(_: *mut PCache, x: Pgno)
    -> ();
    fn sqlite3_wal_end_write_transaction(p_wal_1: *mut Wal)
    -> i32;
    fn sqlite3_wal_exclusive_mode(p_wal_1: *mut Wal, op: i32)
    -> i32;
    static sqlite3_ctype_map: [u8; 0];
    fn sqlite3Malloc(_: u64)
    -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn sqlite3_wal_end_read_transaction(p_wal_1: *mut Wal)
    -> ();
    fn sqlite3_pcache_open(sz_page_1: i32, sz_extra_1: i32,
    b_purgeable_1: i32,
    x_stress_1: Option<unsafe extern "C" fn(*mut (), *mut PgHdr) -> i32>,
    p_stress_1: *mut (), p_to_init_1: *mut PCache)
    -> i32;
    fn sqlite3_bitvec_test_not_null(_: *mut Bitvec, _: u32)
    -> i32;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_journal_open(_: *mut Sqlite3Vfs, _: *const i8,
    _: *mut Sqlite3File, _: i32, _: i32)
    -> i32;
    fn sqlite3_put4byte(_: *mut u8, _: u32)
    -> ();
    fn sqlite3_wal_frames(p_wal_1: *mut Wal, _: i32, _: *mut PgHdr, _: Pgno,
    _: i32, _: i32)
    -> i32;
    fn sqlite3_pcache_clear_sync_flags(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_make_clean(_: *mut PgHdr)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn sqlite3_wal_close(p_wal_1: *mut Wal, _: *mut Sqlite3, sync_flags: i32,
    _: i32, _: *mut u8)
    -> i32;
    fn sqlite3_pcache_close(_: *mut PCache)
    -> ();
    fn sqlite3_pcache_set_cachesize(_: *mut PCache, _: i32)
    -> ();
    fn sqlite3_pcache_set_spillsize(_: *mut PCache, _: i32)
    -> i32;
    fn sqlite3_pcache_shrink(_: *mut PCache)
    -> ();
    fn sqlite3_wal_heap_memory(p_wal_1: *mut Wal)
    -> i32;
    fn sqlite3_wal_dbsize(p_wal_1: *mut Wal)
    -> Pgno;
    fn sqlite3_wal_open(_: *mut Sqlite3Vfs, _: *mut Sqlite3File, _: *const i8,
    _: i32, _: i64, _: *mut *mut Wal)
    -> i32;
    fn sqlite3_wal_begin_read_transaction(p_wal_1: *mut Wal, _: *mut i32)
    -> i32;
    fn sqlite3_wal_limit(_: *mut Wal, _: i64)
    -> ();
    fn sqlite3_pcache_ref(_: *mut PgHdr)
    -> ();
    fn sqlite3_mem_journal_open(_: *mut Sqlite3File)
    -> ();
    fn sqlite3_pcache_move(_: *mut PgHdr, _: Pgno)
    -> ();
    fn sqlite3_bitvec_clear(_: *mut Bitvec, _: u32, _: *mut ())
    -> ();
    fn sqlite3_wal_begin_write_transaction(p_wal_1: *mut Wal)
    -> i32;
    fn sqlite3_fault_sim(_: i32)
    -> i32;
    fn sqlite3Realloc(_: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_wal_savepoint(p_wal_1: *mut Wal, a_wal_data_1: *mut u32)
    -> ();
    fn sqlite3WalCheckpoint(p_wal_1: *mut Wal, db: *mut Sqlite3,
    e_mode_1: i32, x_busy_1: Option<unsafe extern "C" fn(*mut ()) -> i32>,
    p_busy_arg_1: *mut (), sync_flags: i32, n_buf_1: i32, z_buf_1: *mut u8,
    pn_log_1: *mut i32, pn_ckpt_1: *mut i32)
    -> i32;
    fn sqlite3_wal_callback(p_wal_1: *mut Wal)
    -> i32;
    fn sqlite3_p_cache_is_dirty(p_cache_1: *mut PCache)
    -> i32;
    fn sqlite3_pcache_pagecount(_: *mut PCache)
    -> i32;
    fn sqlite3_malloc_size(_: *const ())
    -> i32;
    fn sqlite3_is_memdb(_: *const Sqlite3Vfs)
    -> i32;
    fn sqlite3_wal_file(p_wal_1: *mut Wal)
    -> *mut Sqlite3File;
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
    fn sqlite3_p_cache_set_default()
    -> ();
    fn sqlite3_header_size_pcache()
    -> i32;
    fn sqlite3_header_size_pcache1()
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
    fn sqlite3_db_malloc_zero(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw_nn(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_str_n_dup(_: *mut Sqlite3, _: *const i8, _: u64)
    -> *mut i8;
    fn sqlite3_db_span_dup(_: *mut Sqlite3, _: *const i8, _: *const i8)
    -> *mut i8;
    fn sqlite3_db_realloc_or_free(_: *mut Sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_realloc(_: *mut Sqlite3, _: *mut (), _: u64)
    -> *mut ();
    fn sqlite3_db_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_nn_free_nn(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_db_malloc_size(_: *mut Sqlite3, _: *const ())
    -> i32;
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
    static mut sqlite3_builtin_functions: FuncDefHash;
    static sqlite3_oom_str: Sqlite3Str;
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
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_create_column_expr(_: *mut Sqlite3, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_record_error_byte_offset(_: *mut Sqlite3, _: *const i8)
    -> ();
    fn sqlite3_record_error_offset_of_expr(_: *mut Sqlite3, _: *const Expr)
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
    fn sqlite3_expr_set_height_and_flags(p_parse_1: *mut Parse, p: *mut Expr)
    -> ();
    fn sqlite3_select_expr_height(_: *const Select)
    -> i32;
    fn sqlite3_expr_check_height(_: *mut Parse, _: i32)
    -> i32;
    fn sqlite3_expr_set_error_offset(_: *mut Expr, _: i32)
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
