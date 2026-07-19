#![allow(unused_imports, dead_code)]

mod btree_h;
mod hash_h;
mod pager_h;
mod pcache_h;
mod sqlite3_h;
mod sqlite_int_h;
mod vdbe_h;
use crate::btree_h::{BtCursor, Btree, BtreePayload};
use crate::hash_h::{Hash, HashElem};
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
    Module, NameContext, OnOrUsing, Parse, Returning, RowSet, SQLiteThread,
    Schema, Select, SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo,
    Sqlite3Str, SrcItem, SrcItemS0, SrcList, StrAccum, Subquery, Table, Token,
    Trigger, TriggerPrg, TriggerStep, UnpackedRecord, Upsert, VList, VTable,
    Walker, WhereInfo, Window, With,
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

///* Delete a linked list of TriggerStep structures.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_delete_trigger_step(db: *mut Sqlite3,
    mut p_trigger_step: *mut TriggerStep) -> () {
    unsafe {
        while !(p_trigger_step).is_null() {
            let p_tmp: *mut TriggerStep = p_trigger_step;
            p_trigger_step = unsafe { (*p_trigger_step).p_next };
            unsafe { sqlite3_expr_delete(db, unsafe { (*p_tmp).p_where }) };
            unsafe {
                sqlite3_expr_list_delete(db, unsafe { (*p_tmp).p_expr_list })
            };
            unsafe {
                sqlite3_select_delete(db, unsafe { (*p_tmp).p_select })
            };
            unsafe {
                sqlite3_id_list_delete(db, unsafe { (*p_tmp).p_id_list })
            };
            unsafe {
                sqlite3_upsert_delete(db, unsafe { (*p_tmp).p_upsert })
            };
            unsafe { sqlite3_src_list_delete(db, unsafe { (*p_tmp).p_src }) };
            unsafe {
                sqlite3_db_free(db, unsafe { (*p_tmp).z_span } as *mut ())
            };
            unsafe { sqlite3_db_free(db, p_tmp as *mut ()) };
        }
    }
}

/// 
///* Recursively delete a Trigger structure
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_delete_trigger(db: *mut Sqlite3,
    p_trigger: *mut Trigger) -> () {
    if p_trigger == core::ptr::null_mut() ||
            unsafe { (*p_trigger).b_returning } != 0 {
        return;
    }
    sqlite3_delete_trigger_step(db, unsafe { (*p_trigger).step_list });
    unsafe { sqlite3_db_free(db, unsafe { (*p_trigger).z_name } as *mut ()) };
    unsafe { sqlite3_db_free(db, unsafe { (*p_trigger).table } as *mut ()) };
    unsafe { sqlite3_expr_delete(db, unsafe { (*p_trigger).p_when }) };
    unsafe { sqlite3_id_list_delete(db, unsafe { (*p_trigger).p_columns }) };
    unsafe { sqlite3_db_free(db, p_trigger as *mut ()) };
}

///* This is called by the parser when it sees a CREATE TRIGGER statement
///* up to the point of the BEGIN before the trigger actions.  A Trigger
///* structure is generated based on the information available and stored
///* in pParse->pNewTrigger.  After the trigger actions have been parsed, the
///* sqlite3FinishTrigger() function is called to complete the trigger
///* construction process.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_begin_trigger(p_parse: *mut Parse,
    p_name1: *mut Token, p_name2: *mut Token, mut tr_tm: i32, op: i32,
    mut p_columns: *mut IdList, p_table_name: *mut SrcList,
    mut p_when: *mut Expr, is_temp: i32, no_err: i32) -> () {
    unsafe {
        let mut p_trigger: *mut Trigger = core::ptr::null_mut();
        /// The new trigger
        let mut p_tab: *const Table = core::ptr::null();
        /// Table that the trigger fires off of
        let mut z_name: *mut i8 = core::ptr::null_mut();
        /// Name of the trigger
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// The database connection
        let mut i_db: i32 = 0;
        /// The database to store the trigger in
        let mut p_name: *mut Token = core::ptr::null_mut();
        /// The unqualified db name
        let mut s_fix: DbFixer = unsafe { core::mem::zeroed() };
        /// State vector for the DB fixer
        /// pName1->z might be NULL, but not pName1 itself
        /// If TEMP was specified, then the trigger name may not be qualified.
        /// Figure out the db that the trigger will be created in
        /// A long-standing parser bug is that this syntax was allowed:
        ///*
        ///*    CREATE TRIGGER attached.demo AFTER INSERT ON attached.tab ....
        ///*                                                 ^^^^^^^^
        ///*
        ///* To maintain backwards compatibility, ignore the database
        ///* name on pTableName if we are reparsing out of the schema table
        /// If the trigger name was unqualified, and the table is a temp table,
        ///* then set iDb to 1 to create the trigger in the temporary database.
        ///* If sqlite3SrcListLookup() returns 0, indicating the table does not
        ///* exist, the error is caught by the block below.
        /// Ensure the table name matches database name and that the table exists
        /// The table does not exist.
        /// Check that the trigger name is not reserved and that no trigger of the
        ///* specified name exists
        /// NB: The SQLITE_ALLOW_TRIGGERS_ON_SYSTEM_TABLES compile-time option is
        ///* experimental and unsupported. Do not use it unless understand the
        ///* implications and you cannot get by without this capability.
        /// Experimental */
        ///  /* Do not create a trigger on a system table
        /// INSTEAD of triggers are only for views and views only support INSTEAD
        ///* of triggers.
        let mut i_tab_db: i32 = 0;
        let mut code: i32 = 0;
        let mut z_db: *const i8 = core::ptr::null();
        let mut z_db_trig: *const i8 = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s2:
                {
                match __state {
                    0 => { p_trigger = core::ptr::null_mut(); __state = 4; }
                    2 => {
                        unsafe { sqlite3_db_free(db, z_name as *mut ()) };
                        __state = 104;
                    }
                    3 => {
                        if unsafe { (*db).init.i_db } as i32 == 1 {
                            __state = 113;
                        } else { __state = 112; }
                    }
                    4 => { __state = 5; }
                    5 => { z_name = core::ptr::null_mut(); __state = 6; }
                    6 => { db = unsafe { (*p_parse).db }; __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { { let _ = 0; }; __state = 11; }
                    11 => { { let _ = 0; }; __state = 12; }
                    12 => { { let _ = 0; }; __state = 13; }
                    13 => { { let _ = 0; }; __state = 14; }
                    14 => {
                        if is_temp != 0 { __state = 16; } else { __state = 17; }
                    }
                    15 => {
                        if (p_table_name).is_null() as i32 != 0 ||
                                unsafe { (*db).malloc_failed } != 0 {
                            __state = 25;
                        } else { __state = 24; }
                    }
                    16 => {
                        if unsafe { (*p_name2).n } > 0 as u32 {
                            __state = 19;
                        } else { __state = 18; }
                    }
                    17 => {
                        i_db =
                            unsafe {
                                sqlite3_two_part_name(p_parse, p_name1, p_name2,
                                    &mut p_name)
                            };
                        __state = 22;
                    }
                    18 => { i_db = 1; __state = 21; }
                    19 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"temporary trigger may not have qualified name".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 20;
                    }
                    20 => { __state = 2; }
                    21 => { p_name = p_name1; __state = 15; }
                    22 => {
                        if i_db < 0 { __state = 23; } else { __state = 15; }
                    }
                    23 => { __state = 2; }
                    24 => {
                        if unsafe { (*db).init.busy } != 0 && i_db != 1 {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    25 => { __state = 2; }
                    26 => {
                        p_tab =
                            unsafe { sqlite3_src_list_lookup(p_parse, p_table_name) };
                        __state = 31;
                    }
                    27 => { { let _ = 0; }; __state = 28; }
                    28 => { { let _ = 0; }; __state = 29; }
                    29 => {
                        unsafe {
                            sqlite3_db_free(db,
                                unsafe {
                                        (*(unsafe { (*p_table_name).a.as_ptr() } as
                                                            *mut SrcItem).offset(0 as isize)).u4.z_database
                                    } as *mut ())
                        };
                        __state = 30;
                    }
                    30 => {
                        unsafe {
                            (*(unsafe { (*p_table_name).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).u4.z_database =
                                core::ptr::null_mut()
                        };
                        __state = 26;
                    }
                    31 => {
                        if unsafe { (*db).init.busy } as i32 == 0 &&
                                        unsafe { (*p_name2).n } == 0 as u32 && !(p_tab).is_null() &&
                                unsafe { (*p_tab).p_schema } ==
                                    unsafe {
                                        (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                                    } {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    33 => { i_db = 1; __state = 32; }
                    34 => { { let _ = 0; }; __state = 36; }
                    35 => { __state = 2; }
                    36 => {
                        unsafe {
                            sqlite3_fix_init(&mut s_fix, p_parse, i_db,
                                c"trigger".as_ptr() as *mut i8 as *const i8,
                                p_name as *const Token)
                        };
                        __state = 37;
                    }
                    37 => {
                        if unsafe { sqlite3_fix_src_list(&mut s_fix, p_table_name) }
                                != 0 {
                            __state = 39;
                        } else { __state = 38; }
                    }
                    38 => {
                        p_tab =
                            unsafe { sqlite3_src_list_lookup(p_parse, p_table_name) };
                        __state = 40;
                    }
                    39 => { __state = 2; }
                    40 => {
                        if (p_tab).is_null() as i32 != 0 {
                            __state = 42;
                        } else { __state = 41; }
                    }
                    41 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                            __state = 44;
                        } else { __state = 43; }
                    }
                    42 => { __state = 3; }
                    43 => {
                        if unsafe { (*p_tab).tab_flags } & 4096 as u32 != 0 as u32
                                && unsafe { sqlite3_read_only_shadow_tables(db) } != 0 {
                            __state = 47;
                        } else { __state = 46; }
                    }
                    44 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot create triggers on virtual tables".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 45;
                    }
                    45 => { __state = 3; }
                    46 => {
                        z_name =
                            unsafe {
                                sqlite3_name_from_token(db, p_name as *const Token)
                            };
                        __state = 49;
                    }
                    47 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot create triggers on shadow tables".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 48;
                    }
                    48 => { __state = 3; }
                    49 => {
                        if z_name == core::ptr::null_mut() {
                            __state = 51;
                        } else { __state = 50; }
                    }
                    50 => {
                        if unsafe {
                                    sqlite3_check_object_name(p_parse, z_name as *const i8,
                                        c"trigger".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab).z_name } as *const i8)
                                } != 0 {
                            __state = 54;
                        } else { __state = 53; }
                    }
                    51 => { { let _ = 0; }; __state = 52; }
                    52 => { __state = 2; }
                    53 => { { let _ = 0; }; __state = 55; }
                    54 => { __state = 2; }
                    55 => {
                        if !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                != 0 {
                            __state = 57;
                        } else { __state = 56; }
                    }
                    56 => {
                        if unsafe {
                                    sqlite3_strnicmp(unsafe { (*p_tab).z_name } as *const i8,
                                        c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
                                } == 0 {
                            __state = 64;
                        } else { __state = 63; }
                    }
                    57 => {
                        if !(unsafe {
                                            sqlite3_hash_find(unsafe {
                                                        &raw mut (*unsafe {
                                                                            (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                                                        }).trig_hash
                                                    } as *const Hash, z_name as *const i8)
                                        }).is_null() {
                            __state = 58;
                        } else { __state = 56; }
                    }
                    58 => {
                        if (no_err == 0) as i32 != 0 {
                            __state = 60;
                        } else { __state = 61; }
                    }
                    59 => { __state = 2; }
                    60 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"trigger %T already exists".as_ptr() as *mut i8 as
                                    *const i8, p_name)
                        };
                        __state = 59;
                    }
                    61 => { { let _ = 0; }; __state = 62; }
                    62 => {
                        unsafe { sqlite3_code_verify_schema(p_parse, i_db) };
                        __state = 59;
                    }
                    63 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 2 && tr_tm != 66
                            {
                            __state = 67;
                        } else { __state = 66; }
                    }
                    64 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot create trigger on system table".as_ptr() as *mut i8
                                    as *const i8)
                        };
                        __state = 65;
                    }
                    65 => { __state = 2; }
                    66 => {
                        if !(unsafe { (*p_tab).e_tab_type } as i32 == 2) as i32 != 0
                                && tr_tm == 66 {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    67 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot create %s trigger on view: %S".as_ptr() as *mut i8
                                    as *const i8,
                                if tr_tm == 33 {
                                    c"BEFORE".as_ptr() as *mut i8
                                } else { c"AFTER".as_ptr() as *mut i8 },
                                unsafe { (*p_table_name).a.as_ptr() } as *mut SrcItem)
                        };
                        __state = 68;
                    }
                    68 => { __state = 3; }
                    69 => {
                        if !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                != 0 {
                            __state = 73;
                        } else { __state = 72; }
                    }
                    70 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot create INSTEAD OF trigger on table: %S".as_ptr() as
                                        *mut i8 as *const i8,
                                unsafe { (*p_table_name).a.as_ptr() } as *mut SrcItem)
                        };
                        __state = 71;
                    }
                    71 => { __state = 3; }
                    72 => {
                        if tr_tm == 66 { __state = 84; } else { __state = 83; }
                    }
                    73 => {
                        i_tab_db =
                            unsafe {
                                sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                            };
                        __state = 74;
                    }
                    74 => { code = 7; __state = 75; }
                    75 => {
                        z_db =
                            unsafe {
                                    (*unsafe {
                                                (*db).a_db.offset(i_tab_db as isize)
                                            }).z_db_s_name
                                } as *const i8;
                        __state = 76;
                    }
                    76 => {
                        z_db_trig =
                            if is_temp != 0 {
                                (unsafe {
                                        (*unsafe { (*db).a_db.offset(1 as isize) }).z_db_s_name
                                    }) as *const i8
                            } else { z_db };
                        __state = 77;
                    }
                    77 => {
                        if i_tab_db == 1 || is_temp != 0 {
                            __state = 79;
                        } else { __state = 78; }
                    }
                    78 => {
                        if unsafe {
                                    sqlite3_auth_check(p_parse, code, z_name as *const i8,
                                        unsafe { (*p_tab).z_name } as *const i8, z_db_trig)
                                } != 0 {
                            __state = 81;
                        } else { __state = 80; }
                    }
                    79 => { code = 5; __state = 78; }
                    80 => {
                        if unsafe {
                                    sqlite3_auth_check(p_parse, 18,
                                        if (0 == 0) as i32 != 0 && i_tab_db == 1 {
                                                c"sqlite_temp_master".as_ptr() as *mut i8
                                            } else { c"sqlite_master".as_ptr() as *mut i8 } as
                                            *const i8, core::ptr::null(), z_db)
                                } != 0 {
                            __state = 82;
                        } else { __state = 72; }
                    }
                    81 => { __state = 2; }
                    82 => { __state = 2; }
                    83 => {
                        p_trigger =
                            unsafe {
                                    sqlite3_db_malloc_zero(db,
                                        core::mem::size_of::<Trigger>() as u64)
                                } as *mut Trigger;
                        __state = 85;
                    }
                    84 => { tr_tm = 33; __state = 83; }
                    85 => {
                        if p_trigger == core::ptr::null_mut() {
                            __state = 87;
                        } else { __state = 86; }
                    }
                    86 => {
                        unsafe { (*p_trigger).z_name = z_name };
                        __state = 88;
                    }
                    87 => { __state = 2; }
                    88 => { z_name = core::ptr::null_mut(); __state = 89; }
                    89 => {
                        unsafe {
                            (*p_trigger).table =
                                unsafe {
                                    sqlite3_db_str_dup(db,
                                        unsafe {
                                                (*(unsafe { (*p_table_name).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).z_name
                                            } as *const i8)
                                }
                        };
                        __state = 90;
                    }
                    90 => {
                        unsafe {
                            (*p_trigger).p_schema =
                                unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                }
                        };
                        __state = 91;
                    }
                    91 => {
                        unsafe {
                            (*p_trigger).p_tab_schema = unsafe { (*p_tab).p_schema }
                        };
                        __state = 92;
                    }
                    92 => {
                        unsafe { (*p_trigger).op = op as u8 };
                        __state = 93;
                    }
                    93 => {
                        unsafe {
                            (*p_trigger).tr_tm = if tr_tm == 33 { 1 } else { 2 } as u8
                        };
                        __state = 94;
                    }
                    94 => {
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                            __state = 96;
                        } else { __state = 97; }
                    }
                    95 => {
                        unsafe { (*p_trigger).p_columns = p_columns };
                        __state = 100;
                    }
                    96 => {
                        unsafe {
                            sqlite3_rename_token_remap(p_parse,
                                unsafe { (*p_trigger).table } as *const (),
                                unsafe {
                                        (*(unsafe { (*p_table_name).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).z_name
                                    } as *const ())
                        };
                        __state = 98;
                    }
                    97 => {
                        unsafe {
                            (*p_trigger).p_when =
                                unsafe { sqlite3_expr_dup(db, p_when as *const Expr, 1) }
                        };
                        __state = 95;
                    }
                    98 => {
                        unsafe { (*p_trigger).p_when = p_when };
                        __state = 99;
                    }
                    99 => { p_when = core::ptr::null_mut(); __state = 95; }
                    100 => { p_columns = core::ptr::null_mut(); __state = 101; }
                    101 => { { let _ = 0; }; __state = 102; }
                    102 => {
                        unsafe { (*p_parse).p_new_trigger = p_trigger };
                        __state = 103;
                    }
                    103 => { __state = 2; }
                    104 => {
                        unsafe { sqlite3_src_list_delete(db, p_table_name) };
                        __state = 105;
                    }
                    105 => {
                        unsafe { sqlite3_id_list_delete(db, p_columns) };
                        __state = 106;
                    }
                    106 => {
                        unsafe { sqlite3_expr_delete(db, p_when) };
                        __state = 107;
                    }
                    107 => {
                        if (unsafe { (*p_parse).p_new_trigger }).is_null() as i32 !=
                                0 {
                            __state = 109;
                        } else { __state = 110; }
                    }
                    108 => { return; }
                    109 => {
                        sqlite3_delete_trigger(db, p_trigger);
                        __state = 108;
                    }
                    110 => { { let _ = 0; }; __state = 108; }
                    111 => { __state = 3; }
                    112 => { __state = 2; }
                    113 => {
                        unsafe { (*db).init.set_orphan_trigger(1 as u32 as u32) };
                        __state = 112;
                    }
                    _ => {}
                }
            }
        }
    }
}

///* This routine is called after all of the trigger actions have been parsed
///* in order to complete the process of building the trigger.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_finish_trigger(p_parse: *mut Parse,
    mut p_step_list: *mut TriggerStep, p_all: &mut Token) -> () {
    unsafe {
        let mut p_trig: *mut Trigger = core::ptr::null_mut();
        /// Trigger being finished
        let mut z_name: *mut i8 = core::ptr::null_mut();
        /// Name of trigger
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// The database
        let mut s_fix: DbFixer = unsafe { core::mem::zeroed() };
        /// Fixer object
        let mut i_db: i32 = 0;
        /// Database containing the trigger
        let mut name_token: Token = unsafe { core::mem::zeroed() };
        /// Trigger name for error reporting
        /// if we are not initializing,
        ///* build the sqlite_schema entry
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let mut z: *mut i8 = core::ptr::null_mut();
        /// If this is a new CREATE TABLE statement, and if shadow tables
        ///* are read-only, and the trigger makes a change to a shadow table,
        ///* then raise an error - do not allow the trigger to be created.
        let mut p_step: *const TriggerStep = core::ptr::null();
        /// Make an entry in the sqlite_schema table
        let mut p_link: *mut Trigger = core::ptr::null_mut();
        let mut p_hash: *mut Hash = core::ptr::null_mut();
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s4:
                {
                match __state {
                    0 => {
                        p_trig = unsafe { (*p_parse).p_new_trigger };
                        __state = 3;
                    }
                    2 => { sqlite3_delete_trigger(db, p_trig); __state = 62; }
                    3 => { __state = 4; }
                    4 => { db = unsafe { (*p_parse).db }; __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => {
                        unsafe { (*p_parse).p_new_trigger = core::ptr::null_mut() };
                        __state = 9;
                    }
                    9 => {
                        if unsafe { (*p_parse).n_err } != 0 ||
                                (p_trig).is_null() as i32 != 0 {
                            __state = 11;
                        } else { __state = 10; }
                    }
                    10 => {
                        z_name = unsafe { (*p_trig).z_name };
                        __state = 12;
                    }
                    11 => { __state = 2; }
                    12 => {
                        i_db =
                            unsafe {
                                sqlite3_schema_to_index(unsafe { (*p_parse).db },
                                    unsafe { (*p_trig).p_schema })
                            };
                        __state = 13;
                    }
                    13 => { { let _ = 0; }; __state = 14; }
                    14 => {
                        unsafe { (*p_trig).step_list = p_step_list };
                        __state = 15;
                    }
                    15 => {
                        if !(p_step_list).is_null() {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    16 => {
                        unsafe {
                            sqlite3_token_init(&mut name_token,
                                unsafe { (*p_trig).z_name })
                        };
                        __state = 19;
                    }
                    17 => {
                        unsafe { (*p_step_list).p_trig = p_trig };
                        __state = 18;
                    }
                    18 => {
                        p_step_list = unsafe { (*p_step_list).p_next };
                        __state = 15;
                    }
                    19 => {
                        unsafe {
                            sqlite3_fix_init(&mut s_fix, p_parse, i_db,
                                c"trigger".as_ptr() as *mut i8 as *const i8,
                                &raw mut name_token as *const Token)
                        };
                        __state = 20;
                    }
                    20 => {
                        if unsafe {
                                        sqlite3_fix_trigger_step(&mut s_fix,
                                            unsafe { (*p_trig).step_list })
                                    } != 0 ||
                                unsafe {
                                        sqlite3_fix_expr(&mut s_fix, unsafe { (*p_trig).p_when })
                                    } != 0 {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => {
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                            __state = 24;
                        } else { __state = 25; }
                    }
                    22 => { __state = 2; }
                    23 => {
                        if unsafe { (*db).init.busy } != 0 {
                            __state = 49;
                        } else { __state = 48; }
                    }
                    24 => { { let _ = 0; }; __state = 26; }
                    25 => {
                        if (unsafe { (*db).init.busy } == 0) as i32 != 0 {
                            __state = 28;
                        } else { __state = 23; }
                    }
                    26 => {
                        unsafe { (*p_parse).p_new_trigger = p_trig };
                        __state = 27;
                    }
                    27 => { p_trig = core::ptr::null_mut(); __state = 23; }
                    28 => { __state = 29; }
                    29 => { __state = 30; }
                    30 => {
                        if unsafe { sqlite3_read_only_shadow_tables(db) } != 0 {
                            __state = 32;
                        } else { __state = 31; }
                    }
                    31 => {
                        v = unsafe { sqlite3_get_vdbe(p_parse) };
                        __state = 39;
                    }
                    32 => { __state = 33; }
                    33 => {
                        p_step = unsafe { (*p_trig).step_list };
                        __state = 34;
                    }
                    34 => {
                        if !(p_step).is_null() {
                            __state = 35;
                        } else { __state = 31; }
                    }
                    35 => {
                        if unsafe { (*p_step).p_src } != core::ptr::null_mut() &&
                                unsafe {
                                        sqlite3_shadow_table_name(db,
                                            unsafe {
                                                    (*(unsafe { (*unsafe { (*p_step).p_src }).a.as_ptr() } as
                                                                    *mut SrcItem).offset(0 as isize)).z_name
                                                } as *const i8)
                                    } != 0 {
                            __state = 37;
                        } else { __state = 36; }
                    }
                    36 => {
                        p_step = unsafe { (*p_step).p_next };
                        __state = 34;
                    }
                    37 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"trigger \"%s\" may not write to shadow table \"%s\"".as_ptr()
                                        as *mut i8 as *const i8, unsafe { (*p_trig).z_name },
                                unsafe {
                                    (*(unsafe { (*unsafe { (*p_step).p_src }).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).z_name
                                })
                        };
                        __state = 38;
                    }
                    38 => { __state = 2; }
                    39 => {
                        if v == core::ptr::null_mut() {
                            __state = 41;
                        } else { __state = 40; }
                    }
                    40 => {
                        unsafe { sqlite3_begin_write_operation(p_parse, 0, i_db) };
                        __state = 42;
                    }
                    41 => { __state = 2; }
                    42 => {
                        z =
                            unsafe {
                                sqlite3_db_str_n_dup(db, (*p_all).z as *mut i8 as *const i8,
                                    (*p_all).n as u64)
                            };
                        __state = 43;
                    }
                    43 => { __state = 44; }
                    44 => {
                        unsafe {
                            sqlite3_nested_parse(p_parse,
                                c"INSERT INTO %Q.sqlite_master VALUES(\'trigger\',%Q,%Q,0,\'CREATE TRIGGER %q\')".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                }, z_name, unsafe { (*p_trig).table }, z)
                        };
                        __state = 45;
                    }
                    45 => {
                        unsafe { sqlite3_db_free(db, z as *mut ()) };
                        __state = 46;
                    }
                    46 => {
                        unsafe { sqlite3_change_cookie(p_parse, i_db) };
                        __state = 47;
                    }
                    47 => {
                        unsafe {
                            sqlite3_vdbe_add_parse_schema_op(v, i_db,
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"type=\'trigger\' AND name=\'%q\'".as_ptr() as *mut i8 as
                                            *const i8, z_name)
                                }, 0 as u16)
                        };
                        __state = 23;
                    }
                    48 => { __state = 2; }
                    49 => { p_link = p_trig; __state = 50; }
                    50 => {
                        p_hash =
                            unsafe {
                                &mut (*unsafe {
                                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                                }).trig_hash
                            };
                        __state = 51;
                    }
                    51 => { { let _ = 0; }; __state = 52; }
                    52 => { { let _ = 0; }; __state = 53; }
                    53 => {
                        p_trig =
                            unsafe {
                                    sqlite3_hash_insert(p_hash, z_name as *const i8,
                                        p_trig as *mut ())
                                } as *mut Trigger;
                        __state = 54;
                    }
                    54 => {
                        if !(p_trig).is_null() {
                            __state = 55;
                        } else { __state = 56; }
                    }
                    55 => { unsafe { sqlite3_oom_fault(db) }; __state = 48; }
                    56 => {
                        if unsafe { (*p_link).p_schema } ==
                                unsafe { (*p_link).p_tab_schema } {
                            __state = 57;
                        } else { __state = 48; }
                    }
                    57 => { __state = 58; }
                    58 => {
                        p_tab =
                            unsafe {
                                    sqlite3_hash_find(unsafe {
                                                &raw mut (*unsafe { (*p_link).p_tab_schema }).tbl_hash
                                            } as *const Hash, unsafe { (*p_link).table } as *const i8)
                                } as *mut Table;
                        __state = 59;
                    }
                    59 => { { let _ = 0; }; __state = 60; }
                    60 => {
                        unsafe { (*p_link).p_next = unsafe { (*p_tab).p_trigger } };
                        __state = 61;
                    }
                    61 => {
                        unsafe { (*p_tab).p_trigger = p_link };
                        __state = 48;
                    }
                    62 => { { let _ = 0; }; __state = 63; }
                    63 => {
                        sqlite3_delete_trigger_step(db, p_step_list);
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

///* Return a pointer to the Table structure for the table that a trigger
///* is set on.
extern "C" fn table_of_trigger(p_trigger_1: &Trigger) -> *mut Table {
    return unsafe {
                sqlite3_hash_find(unsafe {
                            &raw mut (*(*p_trigger_1).p_tab_schema).tbl_hash
                        } as *const Hash, (*p_trigger_1).table as *const i8)
            } as *mut Table;
}

///* Drop a trigger given a pointer to that trigger.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_drop_trigger_ptr(p_parse: *mut Parse,
    p_trigger: *mut Trigger) -> () {
    unsafe {
        let mut p_table: *const Table = core::ptr::null();
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let db: *const Sqlite3 = unsafe { (*p_parse).db } as *const Sqlite3;
        let mut i_db: i32 = 0;
        i_db =
            unsafe {
                sqlite3_schema_to_index(unsafe { (*p_parse).db },
                    unsafe { (*p_trigger).p_schema })
            };
        { let _ = 0; };
        p_table = table_of_trigger(unsafe { &*p_trigger });
        { let _ = 0; };
        if !(p_table).is_null() {
            let mut code: i32 = 16;
            let z_db: *const i8 =
                unsafe {
                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                    } as *const i8;
            let z_tab: *const i8 =
                if (0 == 0) as i32 != 0 && i_db == 1 {
                        c"sqlite_temp_master".as_ptr() as *mut i8
                    } else { c"sqlite_master".as_ptr() as *mut i8 } as
                    *const i8;
            if i_db == 1 { code = 14; }
            if unsafe {
                            sqlite3_auth_check(p_parse, code,
                                unsafe { (*p_trigger).z_name } as *const i8,
                                unsafe { (*p_table).z_name } as *const i8, z_db)
                        } != 0 ||
                    unsafe {
                            sqlite3_auth_check(p_parse, 9, z_tab, core::ptr::null(),
                                z_db)
                        } != 0 {
                return;
            }
        }
        if { v = unsafe { sqlite3_get_vdbe(p_parse) }; v } !=
                core::ptr::null_mut() {
            unsafe {
                sqlite3_nested_parse(p_parse,
                    c"DELETE FROM %Q.sqlite_master WHERE name=%Q AND type=\'trigger\'".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe {
                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                    }, unsafe { (*p_trigger).z_name })
            };
            unsafe { sqlite3_change_cookie(p_parse, i_db) };
            unsafe {
                sqlite3_vdbe_add_op4(v, 156, i_db, 0, 0,
                    unsafe { (*p_trigger).z_name } as *const i8, 0)
            };
        }
    }
}

///* This function is called to drop a trigger from the database schema. 
///*
///* This may be called directly from the parser and therefore identifies
///* the trigger by name.  The sqlite3DropTriggerPtr() routine does the
///* same job as this routine except it takes a pointer to the trigger
///* instead of the trigger name.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_drop_trigger(p_parse: *mut Parse,
    p_name: *mut SrcList, no_err: i32) -> () {
    unsafe {
        let mut p_trigger: *mut Trigger = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut z_db: *const i8 = core::ptr::null();
        let mut z_name: *const i8 = core::ptr::null();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut j: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s6:
                {
                match __state {
                    0 => { p_trigger = core::ptr::null_mut(); __state = 3; }
                    2 => {
                        unsafe { sqlite3_src_list_delete(db, p_name) };
                        __state = 1;
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { db = unsafe { (*p_parse).db }; __state = 7; }
                    7 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 9;
                        } else { __state = 8; }
                    }
                    8 => {
                        if 0 != unsafe { sqlite3_read_schema(p_parse) } {
                            __state = 11;
                        } else { __state = 10; }
                    }
                    9 => { __state = 2; }
                    10 => { { let _ = 0; }; __state = 12; }
                    11 => { __state = 2; }
                    12 => { { let _ = 0; }; __state = 13; }
                    13 => {
                        z_db =
                            unsafe {
                                    (*(unsafe { (*p_name).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).u4.z_database
                                } as *const i8;
                        __state = 14;
                    }
                    14 => {
                        z_name =
                            unsafe {
                                    (*(unsafe { (*p_name).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).z_name
                                } as *const i8;
                        __state = 15;
                    }
                    15 => { { let _ = 0; }; __state = 16; }
                    16 => { i = 0; __state = 18; }
                    17 => {
                        if (p_trigger).is_null() as i32 != 0 {
                            __state = 28;
                        } else { __state = 27; }
                    }
                    18 => {
                        if i < unsafe { (*db).n_db } {
                            __state = 19;
                        } else { __state = 17; }
                    }
                    19 => { j = if i < 2 { i ^ 1 } else { i }; __state = 21; }
                    20 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 18;
                    }
                    21 => {
                        if !(z_db).is_null() &&
                                unsafe { sqlite3_db_is_named(db, j, z_db) } == 0 {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    22 => { { let _ = 0; }; __state = 24; }
                    23 => { __state = 20; }
                    24 => {
                        p_trigger =
                            unsafe {
                                    sqlite3_hash_find(unsafe {
                                                &raw mut (*unsafe {
                                                                    (*unsafe { (*db).a_db.offset(j as isize) }).p_schema
                                                                }).trig_hash
                                            } as *const Hash, z_name)
                                } as *mut Trigger;
                        __state = 25;
                    }
                    25 => {
                        if !(p_trigger).is_null() {
                            __state = 26;
                        } else { __state = 20; }
                    }
                    26 => { __state = 17; }
                    27 => {
                        sqlite3_drop_trigger_ptr(p_parse, p_trigger);
                        __state = 33;
                    }
                    28 => {
                        if (no_err == 0) as i32 != 0 {
                            __state = 30;
                        } else { __state = 31; }
                    }
                    29 => {
                        unsafe { (*p_parse).set_check_schema(1 as Bft as u32) };
                        __state = 32;
                    }
                    30 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"no such trigger: %S".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_name).a.as_ptr() } as *mut SrcItem)
                        };
                        __state = 29;
                    }
                    31 => {
                        unsafe { sqlite3_code_verify_named_schema(p_parse, z_db) };
                        __state = 29;
                    }
                    32 => { __state = 2; }
                    33 => { __state = 2; }
                    _ => {}
                }
            }
        }
    }
}

///* Return true if any TEMP triggers exist
extern "C" fn temp_triggers_exist(db: &Sqlite3) -> i32 {
    unsafe {
        if unsafe { (*(*db).a_db.offset(1 as isize)).p_schema } ==
                core::ptr::null_mut() {
            return 0;
        }
        if unsafe {
                    (*unsafe {
                                    &mut (*unsafe {
                                                        (*(*db).a_db.offset(1 as isize)).p_schema
                                                    }).trig_hash
                                }).first
                } == core::ptr::null_mut() {
            return 0;
        }
        return 1;
    }
}

///* Given table pTab, return a list of all the triggers attached to 
///* the table. The list is connected by Trigger.pNext pointers.
///*
///* All of the triggers on pTab that are in the same database as pTab
///* are already attached to pTab->pTrigger.  But there might be additional
///* triggers on pTab in the TEMP schema.  This routine prepends all
///* TEMP triggers on pTab to the beginning of the pTab->pTrigger list
///* and returns the combined list.
///*
///* To state it another way:  This routine returns a list of all triggers
///* that fire off of pTab.  The list will include any TEMP triggers on
///* pTab as well as the triggers lised in pTab->pTrigger.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_trigger_list(p_parse: &Parse, p_tab: &Table)
    -> *mut Trigger {
    unsafe {
        let mut p_tmp_schema: *mut Schema = core::ptr::null_mut();
        /// Schema of the pTab table
        let mut p_list: *mut Trigger = core::ptr::null_mut();
        /// List of triggers to return
        let mut p: *mut HashElem = core::ptr::null_mut();

        /// Loop variable for TEMP triggers
        { let _ = 0; };
        p_tmp_schema =
            unsafe {
                (*unsafe {
                            (*(*p_parse).db).a_db.offset(1 as isize)
                        }).p_schema
            };
        p = unsafe { (*unsafe { &mut (*p_tmp_schema).trig_hash }).first };
        p_list = (*p_tab).p_trigger;
        while !(p).is_null() {
            let p_trig: *mut Trigger = unsafe { (*p).data } as *mut Trigger;
            if unsafe { (*p_trig).p_tab_schema } == (*p_tab).p_schema &&
                            !(unsafe { (*p_trig).table }).is_null() &&
                        0 ==
                            unsafe {
                                sqlite3_str_i_cmp(unsafe { (*p_trig).table } as *const i8,
                                    (*p_tab).z_name as *const i8)
                            } &&
                    (unsafe { (*p_trig).p_tab_schema } != p_tmp_schema ||
                        unsafe { (*p_trig).b_returning } != 0) {
                unsafe { (*p_trig).p_next = p_list };
                p_list = p_trig;
            } else if unsafe { (*p_trig).op } as i32 == 151 {
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                unsafe { (*p_trig).table = (*p_tab).z_name };
                unsafe { (*p_trig).p_tab_schema = (*p_tab).p_schema };
                unsafe { (*p_trig).p_next = p_list };
                p_list = p_trig;
            }
            p = unsafe { (*p).next };
        }
        return p_list;
    }
}

///* pEList is the SET clause of an UPDATE statement.  Each entry
///* in pEList is of the format <id>=<expr>.  If any of the entries
///* in pEList have an <id> which matches an identifier in pIdList,
///* then return TRUE.  If pIdList==NULL, then it is considered a
///* wildcard that matches anything.  Likewise if pEList==NULL then
///* it matches anything so always return true.  Return false only
///* if there is no match.
extern "C" fn check_column_overlap(p_id_list_1: *mut IdList,
    p_e_list_1: *const ExprList) -> i32 {
    let mut e: i32 = 0;
    if p_id_list_1 == core::ptr::null_mut() ||
            p_e_list_1 == core::ptr::null_mut() {
        return 1;
    }
    {
        e = 0;
        '__b8: loop {
            if !(e < unsafe { (*p_e_list_1).n_expr }) { break '__b8; }
            '__c8: loop {
                if unsafe {
                            sqlite3_id_list_index(p_id_list_1,
                                unsafe {
                                        (*(unsafe { (*p_e_list_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(e as isize)).z_e_name
                                    } as *const i8)
                        } >= 0 {
                    return 1;
                }
                break '__c8;
            }
            { let __p = &mut e; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Return a list of all triggers on table pTab if there exists at least
///* one trigger that must be fired when an operation of type 'op' is 
///* performed on the table, and, if that operation is an UPDATE, if at
///* least one of the columns in pChanges is being modified.
#[allow(unused_doc_comments)]
extern "C" fn triggers_really_exist(p_parse_1: *mut Parse,
    p_tab_1: *mut Table, op: i32, p_changes_1: *mut ExprList,
    p_mask_1: *mut i32) -> *mut Trigger {
    unsafe {
        let mut mask: i32 = 0;
        let mut p_list: *mut Trigger = core::ptr::null_mut();
        let mut p: *mut Trigger = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s10:
                {
                match __state {
                    0 => { mask = 0; __state = 3; }
                    2 => {
                        if !(p_mask_1).is_null() {
                            __state = 34;
                        } else { __state = 33; }
                    }
                    3 => { p_list = core::ptr::null_mut(); __state = 4; }
                    4 => { __state = 5; }
                    5 => {
                        p_list =
                            sqlite3_trigger_list(unsafe { &*p_parse_1 },
                                unsafe { &*p_tab_1 });
                        __state = 6;
                    }
                    6 => { { let _ = 0; }; __state = 7; }
                    7 => {
                        if p_list != core::ptr::null_mut() {
                            __state = 9;
                        } else { __state = 8; }
                    }
                    8 => { __state = 2; }
                    9 => { p = p_list; __state = 10; }
                    10 => {
                        if unsafe { (*unsafe { (*p_parse_1).db }).flags } &
                                            262144 as u64 == 0 as u64 &&
                                    unsafe { (*p_tab_1).p_trigger } != core::ptr::null_mut() &&
                                unsafe {
                                        sqlite3_schema_to_index(unsafe { (*p_parse_1).db },
                                            unsafe { (*unsafe { (*p_tab_1).p_trigger }).p_schema })
                                    } != 1 {
                            __state = 12;
                        } else { __state = 11; }
                    }
                    11 => {
                        if unsafe { (*p).op } as i32 == op &&
                                check_column_overlap(unsafe { (*p).p_columns },
                                        p_changes_1 as *const ExprList) != 0 {
                            __state = 21;
                        } else { __state = 22; }
                    }
                    12 => {
                        if p_list == unsafe { (*p_tab_1).p_trigger } {
                            __state = 14;
                        } else { __state = 13; }
                    }
                    13 => {
                        if !(unsafe { (*p).p_next }).is_null() &&
                                unsafe { (*p).p_next } != unsafe { (*p_tab_1).p_trigger } {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    14 => { p_list = core::ptr::null_mut(); __state = 15; }
                    15 => { __state = 2; }
                    16 => {
                        unsafe { (*p).p_next = core::ptr::null_mut() };
                        __state = 18;
                    }
                    17 => { p = unsafe { (*p).p_next }; __state = 13; }
                    18 => { p = p_list; __state = 11; }
                    19 => {
                        if !(p).is_null() { __state = 11; } else { __state = 8; }
                    }
                    20 => { p = unsafe { (*p).p_next }; __state = 19; }
                    21 => {
                        mask |= unsafe { (*p).tr_tm } as i32;
                        __state = 20;
                    }
                    22 => {
                        if unsafe { (*p).op } as i32 == 151 {
                            __state = 23;
                        } else { __state = 24; }
                    }
                    23 => { { let _ = 0; }; __state = 25; }
                    24 => {
                        if unsafe { (*p).b_returning } != 0 &&
                                        unsafe { (*p).op } as i32 == 128 && op == 130 &&
                                unsafe { (*p_parse_1).p_toplevel } == core::ptr::null_mut()
                            {
                            __state = 32;
                        } else { __state = 20; }
                    }
                    25 => { unsafe { (*p).op = op as u8 }; __state = 26; }
                    26 => {
                        if unsafe { (*p_tab_1).e_tab_type } as i32 == 1 {
                            __state = 28;
                        } else { __state = 29; }
                    }
                    27 => {
                        mask |= unsafe { (*p).tr_tm } as i32;
                        __state = 20;
                    }
                    28 => {
                        if op != 128 { __state = 31; } else { __state = 30; }
                    }
                    29 => { unsafe { (*p).tr_tm = 2 as u8 }; __state = 27; }
                    30 => { unsafe { (*p).tr_tm = 1 as u8 }; __state = 27; }
                    31 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"%s RETURNING is not available on virtual tables".as_ptr()
                                        as *mut i8 as *const i8,
                                if op == 129 {
                                    c"DELETE".as_ptr() as *mut i8
                                } else { c"UPDATE".as_ptr() as *mut i8 })
                        };
                        __state = 30;
                    }
                    32 => {
                        mask |= unsafe { (*p).tr_tm } as i32;
                        __state = 20;
                    }
                    33 => {
                        return if mask != 0 {
                                p_list
                            } else { core::ptr::null_mut() };
                    }
                    34 => { unsafe { *p_mask_1 = mask }; __state = 33; }
                    _ => {}
                }
            }
        }

        /// The SQLITE_DBCONFIG_ENABLE_TRIGGER setting is off.  That means that
        ///* only TEMP triggers are allowed.  Truncate the pList so that it
        ///* includes only TEMP triggers
        /// The first time a RETURNING trigger is seen, the "op" value tells
        ///* us what time of trigger it should be.
        /// Also fire a RETURNING trigger for an UPSERT
        unreachable!();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_triggers_exist(p_parse: *mut Parse,
    p_tab: *mut Table, op: i32, p_changes: *mut ExprList, p_mask: *mut i32)
    -> *mut Trigger {
    { let _ = 0; };
    if unsafe { (*p_tab).p_trigger } == core::ptr::null_mut() &&
                (temp_triggers_exist(unsafe { &*unsafe { (*p_parse).db } }) ==
                            0) as i32 != 0 ||
            unsafe { (*p_parse).disable_triggers() } != 0 {
        if !(p_mask).is_null() { unsafe { *p_mask = 0 }; }
        return core::ptr::null_mut();
    }
    return triggers_really_exist(p_parse, p_tab, op, p_changes, p_mask);
}

///* Generate VDBE code for the statements inside the body of a single 
///* trigger.
#[allow(unused_doc_comments)]
extern "C" fn code_trigger_program(p_parse_1: *mut Parse,
    p_step_list_1: *mut TriggerStep, orconf: i32) -> i32 {
    unsafe {
        let mut p_step: *const TriggerStep = core::ptr::null();
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        {
            p_step = p_step_list_1;
            '__b11: loop {
                if !(!(p_step).is_null()) { break '__b11; }
                '__c11: loop {

                    /// Figure out the ON CONFLICT policy that will be used for this step
                    ///* of the trigger program. If the statement that caused this trigger
                    ///* to fire had an explicit ON CONFLICT, then use it. Otherwise, use
                    ///* the ON CONFLICT policy that was specified as part of the trigger
                    ///* step statement. Example:
                    ///*
                    ///*   CREATE TRIGGER AFTER INSERT ON t1 BEGIN;
                    ///*     INSERT OR REPLACE INTO t2 VALUES(new.a, new.b);
                    ///*   END;
                    ///*
                    ///*   INSERT INTO t1 ... ;            -- insert into t2 uses REPLACE policy
                    ///*   INSERT OR IGNORE INTO t1 ... ;  -- insert into t2 uses IGNORE policy
                    unsafe {
                        (*p_parse_1).e_orconf =
                            if orconf == 11 {
                                    (unsafe { (*p_step).orconf }) as i32
                                } else { orconf as u8 as i32 } as u8
                    };
                    { let _ = 0; };
                    if !(unsafe { (*p_step).z_span }).is_null() {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 186, 2147483647, 1, 0,
                                unsafe {
                                        sqlite3_m_printf(db,
                                            c"-- %s".as_ptr() as *mut i8 as *const i8,
                                            unsafe { (*p_step).z_span })
                                    } as *const i8, -7)
                        };
                    }
                    '__s12:
                        {
                        match unsafe { (*p_step).op } {
                            130 => {
                                {
                                    unsafe {
                                        sqlite3_update(p_parse_1,
                                            unsafe {
                                                sqlite3_src_list_dup(db,
                                                    unsafe { (*p_step).p_src } as *const SrcList, 0)
                                            },
                                            unsafe {
                                                sqlite3_expr_list_dup(db,
                                                    unsafe { (*p_step).p_expr_list } as *const ExprList, 0)
                                            },
                                            unsafe {
                                                sqlite3_expr_dup(db,
                                                    unsafe { (*p_step).p_where } as *const Expr, 0)
                                            }, unsafe { (*p_parse_1).e_orconf } as i32,
                                            core::ptr::null_mut(), core::ptr::null_mut(),
                                            core::ptr::null_mut())
                                    };
                                    unsafe { sqlite3_vdbe_add_op0(v, 133) };
                                    break '__s12;
                                }
                                {
                                    unsafe {
                                        sqlite3_insert(p_parse_1,
                                            unsafe {
                                                sqlite3_src_list_dup(db,
                                                    unsafe { (*p_step).p_src } as *const SrcList, 0)
                                            },
                                            unsafe {
                                                sqlite3_select_dup(db,
                                                    unsafe { (*p_step).p_select } as *const Select, 0)
                                            },
                                            unsafe {
                                                sqlite3_id_list_dup(db,
                                                    unsafe { (*p_step).p_id_list } as *const IdList)
                                            }, unsafe { (*p_parse_1).e_orconf } as i32,
                                            unsafe {
                                                sqlite3_upsert_dup(db, unsafe { (*p_step).p_upsert })
                                            })
                                    };
                                    unsafe { sqlite3_vdbe_add_op0(v, 133) };
                                    break '__s12;
                                }
                                {
                                    unsafe {
                                        sqlite3_delete_from(p_parse_1,
                                            unsafe {
                                                sqlite3_src_list_dup(db,
                                                    unsafe { (*p_step).p_src } as *const SrcList, 0)
                                            },
                                            unsafe {
                                                sqlite3_expr_dup(db,
                                                    unsafe { (*p_step).p_where } as *const Expr, 0)
                                            }, core::ptr::null_mut(), core::ptr::null_mut())
                                    };
                                    unsafe { sqlite3_vdbe_add_op0(v, 133) };
                                    break '__s12;
                                }
                                { let _ = 0; };
                                {
                                    let mut s_dest: SelectDest = unsafe { core::mem::zeroed() };
                                    let p_select: *mut Select =
                                        unsafe {
                                            sqlite3_select_dup(db,
                                                unsafe { (*p_step).p_select } as *const Select, 0)
                                        };
                                    unsafe { sqlite3_select_dest_init(&mut s_dest, 2, 0) };
                                    unsafe { sqlite3_select(p_parse_1, p_select, &mut s_dest) };
                                    unsafe { sqlite3_select_delete(db, p_select) };
                                    break '__s12;
                                }
                            }
                            128 => {
                                {
                                    unsafe {
                                        sqlite3_insert(p_parse_1,
                                            unsafe {
                                                sqlite3_src_list_dup(db,
                                                    unsafe { (*p_step).p_src } as *const SrcList, 0)
                                            },
                                            unsafe {
                                                sqlite3_select_dup(db,
                                                    unsafe { (*p_step).p_select } as *const Select, 0)
                                            },
                                            unsafe {
                                                sqlite3_id_list_dup(db,
                                                    unsafe { (*p_step).p_id_list } as *const IdList)
                                            }, unsafe { (*p_parse_1).e_orconf } as i32,
                                            unsafe {
                                                sqlite3_upsert_dup(db, unsafe { (*p_step).p_upsert })
                                            })
                                    };
                                    unsafe { sqlite3_vdbe_add_op0(v, 133) };
                                    break '__s12;
                                }
                                {
                                    unsafe {
                                        sqlite3_delete_from(p_parse_1,
                                            unsafe {
                                                sqlite3_src_list_dup(db,
                                                    unsafe { (*p_step).p_src } as *const SrcList, 0)
                                            },
                                            unsafe {
                                                sqlite3_expr_dup(db,
                                                    unsafe { (*p_step).p_where } as *const Expr, 0)
                                            }, core::ptr::null_mut(), core::ptr::null_mut())
                                    };
                                    unsafe { sqlite3_vdbe_add_op0(v, 133) };
                                    break '__s12;
                                }
                                { let _ = 0; };
                                {
                                    let mut s_dest: SelectDest = unsafe { core::mem::zeroed() };
                                    let p_select: *mut Select =
                                        unsafe {
                                            sqlite3_select_dup(db,
                                                unsafe { (*p_step).p_select } as *const Select, 0)
                                        };
                                    unsafe { sqlite3_select_dest_init(&mut s_dest, 2, 0) };
                                    unsafe { sqlite3_select(p_parse_1, p_select, &mut s_dest) };
                                    unsafe { sqlite3_select_delete(db, p_select) };
                                    break '__s12;
                                }
                            }
                            129 => {
                                {
                                    unsafe {
                                        sqlite3_delete_from(p_parse_1,
                                            unsafe {
                                                sqlite3_src_list_dup(db,
                                                    unsafe { (*p_step).p_src } as *const SrcList, 0)
                                            },
                                            unsafe {
                                                sqlite3_expr_dup(db,
                                                    unsafe { (*p_step).p_where } as *const Expr, 0)
                                            }, core::ptr::null_mut(), core::ptr::null_mut())
                                    };
                                    unsafe { sqlite3_vdbe_add_op0(v, 133) };
                                    break '__s12;
                                }
                                { let _ = 0; };
                                {
                                    let mut s_dest: SelectDest = unsafe { core::mem::zeroed() };
                                    let p_select: *mut Select =
                                        unsafe {
                                            sqlite3_select_dup(db,
                                                unsafe { (*p_step).p_select } as *const Select, 0)
                                        };
                                    unsafe { sqlite3_select_dest_init(&mut s_dest, 2, 0) };
                                    unsafe { sqlite3_select(p_parse_1, p_select, &mut s_dest) };
                                    unsafe { sqlite3_select_delete(db, p_select) };
                                    break '__s12;
                                }
                            }
                            _ => {
                                { let _ = 0; };
                                {
                                    let mut s_dest: SelectDest = unsafe { core::mem::zeroed() };
                                    let p_select: *mut Select =
                                        unsafe {
                                            sqlite3_select_dup(db,
                                                unsafe { (*p_step).p_select } as *const Select, 0)
                                        };
                                    unsafe { sqlite3_select_dest_init(&mut s_dest, 2, 0) };
                                    unsafe { sqlite3_select(p_parse_1, p_select, &mut s_dest) };
                                    unsafe { sqlite3_select_delete(db, p_select) };
                                    break '__s12;
                                }
                            }
                        }
                    }
                    break '__c11;
                }
                p_step = unsafe { (*p_step).p_next };
            }
        }
        return 0;
    }
}

///* Parse context structure pFrom has just been used to create a sub-vdbe
///* (trigger program). If an error has occurred, transfer error information
///* from pFrom to pTo.
extern "C" fn transfer_parse_error(p_to_1: &mut Parse, p_from_1: &Parse)
    -> () {
    { let _ = 0; };
    { let _ = 0; };
    if (*p_to_1).n_err == 0 {
        (*p_to_1).z_err_msg = (*p_from_1).z_err_msg;
        (*p_to_1).n_err = (*p_from_1).n_err;
        (*p_to_1).rc = (*p_from_1).rc;
    } else {
        unsafe {
            sqlite3_db_free((*p_from_1).db, (*p_from_1).z_err_msg as *mut ())
        };
    }
}

///* Create and populate a new TriggerPrg object with a sub-program 
///* implementing trigger pTrigger with ON CONFLICT policy orconf.
#[allow(unused_doc_comments)]
extern "C" fn code_row_trigger(p_parse_1: *mut Parse,
    p_trigger_1: *mut Trigger, p_tab_1: *mut Table, orconf: i32)
    -> *mut TriggerPrg {
    unsafe {
        let mut p_top: *mut Parse = core::ptr::null_mut();
        /// Top level Parse object
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        /// Database handle
        let mut p_prg: *mut TriggerPrg = core::ptr::null_mut();
        /// Value to return
        let mut p_when: *mut Expr = core::ptr::null_mut();
        /// Duplicate of trigger WHEN expression
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// Temporary VM
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        /// Name context for sub-vdbe
        let mut p_program: *mut SubProgram = core::ptr::null_mut();
        /// Sub-vdbe for trigger program
        let mut i_end_trigger: i32 = 0;
        /// Label to jump to if WHEN is false
        let mut s_sub_parse: Parse = unsafe { core::mem::zeroed() };
        /// Parse context for sub-vdbe
        let mut n_depth: i32 = 0;

        /// Trigger depth
        /// Ensure that triggers are not chained too deep.  This test is linear
        ///* in the chaining depth, but sensible code ought not be chaining
        ///* triggers excessively, so that shouldn't be a problem.
        (p_top = p_parse_1);
        {
            n_depth = 0;
            '__b13: loop {
                if !(!(unsafe { (*p_top).p_outer_parse }).is_null()) {
                    break '__b13;
                }
                '__c13: loop { break '__c13; }
                {
                    p_top = unsafe { (*p_top).p_outer_parse };
                    { let __p = &mut n_depth; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        if n_depth >= unsafe { (*db).a_limit[10 as usize] } {
            unsafe {
                sqlite3_error_msg(p_parse_1,
                    c"triggers nested too deep".as_ptr() as *mut i8 as
                        *const i8)
            };
            return core::ptr::null_mut();
        }
        p_top =
            if !(unsafe { (*p_parse_1).p_toplevel }).is_null() {
                unsafe { (*p_parse_1).p_toplevel }
            } else { p_parse_1 };
        { let _ = 0; };
        { let _ = 0; };

        /// Allocate the TriggerPrg and SubProgram objects. To ensure that they
        ///* are freed if an error occurs, link them into the Parse.pTriggerPrg 
        ///* list of the top-level Parse object sooner rather than later.
        (p_prg =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<TriggerPrg>() as u64)
                } as *mut TriggerPrg);
        if (p_prg).is_null() as i32 != 0 { return core::ptr::null_mut(); }
        unsafe { (*p_prg).p_next = unsafe { (*p_top).p_trigger_prg } };
        unsafe { (*p_top).p_trigger_prg = p_prg };
        unsafe {
            (*p_prg).p_program =
                {
                    p_program =
                        unsafe {
                                sqlite3_db_malloc_zero(db,
                                    core::mem::size_of::<SubProgram>() as u64)
                            } as *mut SubProgram;
                    p_program
                }
        };
        if (p_program).is_null() as i32 != 0 { return core::ptr::null_mut(); }
        unsafe {
            sqlite3_vdbe_link_sub_program(unsafe { (*p_top).p_vdbe },
                p_program)
        };
        unsafe { (*p_prg).p_trigger = p_trigger_1 };
        unsafe { (*p_prg).orconf = orconf };
        unsafe { (*p_prg).a_colmask[0 as usize] = 4294967295u32 };
        unsafe { (*p_prg).a_colmask[1 as usize] = 4294967295u32 };

        /// Allocate and populate a new Parse context to use for coding the 
        ///* trigger sub-program.
        unsafe { sqlite3_parse_object_init(&mut s_sub_parse, db) };
        unsafe {
            memset(&raw mut s_nc as *mut (), 0,
                core::mem::size_of::<NameContext>() as u64)
        };
        s_nc.p_parse = &mut s_sub_parse;
        s_sub_parse.p_trigger_tab = p_tab_1;
        s_sub_parse.p_toplevel = p_top;
        s_sub_parse.z_auth_context =
            unsafe { (*p_trigger_1).z_name } as *const i8;
        s_sub_parse.e_trigger_op = unsafe { (*p_trigger_1).op };
        s_sub_parse.n_query_loop = unsafe { (*p_parse_1).n_query_loop };
        s_sub_parse.prep_flags = unsafe { (*p_parse_1).prep_flags };
        s_sub_parse.oldmask = 0 as u32;
        s_sub_parse.newmask = 0 as u32;
        v = unsafe { sqlite3_get_vdbe(&mut s_sub_parse) };
        if !(v).is_null() {
            if !(unsafe { (*p_trigger_1).z_name }).is_null() {
                unsafe {
                    sqlite3_vdbe_change_p4(v, -1,
                        unsafe {
                                sqlite3_m_printf(db,
                                    c"-- TRIGGER %s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { (*p_trigger_1).z_name })
                            } as *const i8, -7)
                };
            }
            if !(unsafe { (*p_trigger_1).p_when }).is_null() {
                p_when =
                    unsafe {
                        sqlite3_expr_dup(db,
                            unsafe { (*p_trigger_1).p_when } as *const Expr, 0)
                    };
                if unsafe { (*db).malloc_failed } as i32 == 0 &&
                        0 ==
                            unsafe { sqlite3_resolve_expr_names(&mut s_nc, p_when) } {
                    i_end_trigger =
                        unsafe { sqlite3_vdbe_make_label(&mut s_sub_parse) };
                    unsafe {
                        sqlite3_expr_if_false(&mut s_sub_parse, p_when,
                            i_end_trigger, 16)
                    };
                }
                unsafe { sqlite3_expr_delete(db, p_when) };
            }

            /// Code the trigger program into the sub-vdbe.
            code_trigger_program(&mut s_sub_parse,
                unsafe { (*p_trigger_1).step_list }, orconf);
            if i_end_trigger != 0 {
                unsafe { sqlite3_vdbe_resolve_label(v, i_end_trigger) };
            }
            unsafe { sqlite3_vdbe_add_op0(v, 72) };
            transfer_parse_error(unsafe { &mut *p_parse_1 }, &s_sub_parse);
            if unsafe { (*p_parse_1).n_err } == 0 {
                { let _ = 0; };
                unsafe {
                    (*p_program).a_op =
                        unsafe {
                            sqlite3_vdbe_take_op_array(v,
                                unsafe { &mut (*p_program).n_op },
                                unsafe { &mut (*p_top).n_max_arg })
                        }
                };
            }
            unsafe { (*p_program).n_mem = s_sub_parse.n_mem };
            unsafe { (*p_program).n_csr = s_sub_parse.n_tab };
            unsafe { (*p_program).token = p_trigger_1 as *mut () };
            unsafe { (*p_prg).a_colmask[0 as usize] = s_sub_parse.oldmask };
            unsafe { (*p_prg).a_colmask[1 as usize] = s_sub_parse.newmask };
            unsafe { sqlite3_vdbe_delete(v) };
        } else {
            transfer_parse_error(unsafe { &mut *p_parse_1 }, &s_sub_parse);
        }
        { let _ = 0; };
        unsafe { sqlite3_parse_object_reset(&mut s_sub_parse) };
        return p_prg;
    }
}

///* Return a pointer to a TriggerPrg object containing the sub-program for
///* trigger pTrigger with default ON CONFLICT algorithm orconf. If no such
///* TriggerPrg object exists, a new object is allocated and populated before
///* being returned.
extern "C" fn get_row_trigger(p_parse_1: *mut Parse,
    p_trigger_1: *mut Trigger, p_tab_1: *mut Table, orconf: i32)
    -> *mut TriggerPrg {
    let p_root: *const Parse =
        if !(unsafe { (*p_parse_1).p_toplevel }).is_null() {
                unsafe { (*p_parse_1).p_toplevel }
            } else { p_parse_1 } as *const Parse;
    let mut p_prg: *mut TriggerPrg = core::ptr::null_mut();
    { let _ = 0; };
    {
        p_prg = unsafe { (*p_root).p_trigger_prg };
        '__b14: loop {
            if !(!(p_prg).is_null() &&
                            (unsafe { (*p_prg).p_trigger } != p_trigger_1 ||
                                unsafe { (*p_prg).orconf } != orconf)) {
                break '__b14;
            }
            '__c14: loop { break '__c14; }
            p_prg = unsafe { (*p_prg).p_next };
        }
    }
    if (p_prg).is_null() as i32 != 0 {
        p_prg = code_row_trigger(p_parse_1, p_trigger_1, p_tab_1, orconf);
        unsafe { (*unsafe { (*p_parse_1).db }).err_byte_offset = -1 };
    }
    return p_prg;
}

///* Generate code for the trigger program associated with trigger p on 
///* table pTab. The reg, orconf and ignoreJump parameters passed to this
///* function are the same as those described in the header function for
///* sqlite3CodeRowTrigger()
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_code_row_trigger_direct(p_parse: *mut Parse,
    p: *mut Trigger, p_tab: *mut Table, reg: i32, orconf: i32,
    ignore_jump: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };
        /// Main VM
        let mut p_prg: *const TriggerPrg = core::ptr::null();
        p_prg = get_row_trigger(p_parse, p, p_tab, orconf);
        { let _ = 0; };
        if !(p_prg).is_null() {
            let b_recursive: i32 =
                (!(unsafe { (*p).z_name }).is_null() &&
                        0 as u64 ==
                            unsafe { (*unsafe { (*p_parse).db }).flags } & 8192 as u64)
                    as i32;
            unsafe {
                sqlite3_vdbe_add_op4(v, 50, reg, ignore_jump,
                    {
                        let __p = unsafe { &mut (*p_parse).n_mem };
                        *__p += 1;
                        *__p
                    }, unsafe { (*p_prg).p_program } as *const i8, -4)
            };

            /// Set the P5 operand of the OP_Program instruction to non-zero if
            ///* recursive invocation of this trigger program is disallowed. Recursive
            ///* invocation is disallowed if (a) the sub-program is really a trigger,
            ///* not a foreign key action, and (b) the flag to enable recursive triggers
            ///* is clear.
            unsafe { sqlite3_vdbe_change_p5(v, b_recursive as u16) };
        }
    }
}

///* Return true if the pExpr term from the RETURNING clause argument
///* list is of the form "*".  Raise an error if the terms if of the
///* form "table.*".
extern "C" fn is_asterisk_term(p_parse_1: *mut Parse, p_term_1: &Expr)
    -> i32 {
    { let _ = 0; };
    if (*p_term_1).op as i32 == 180 { return 1; }
    if (*p_term_1).op as i32 != 142 { return 0; }
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*(*p_term_1).p_right).op } as i32 != 180 { return 0; }
    unsafe {
        sqlite3_error_msg(p_parse_1,
            c"RETURNING may not use \"TABLE.*\" wildcards".as_ptr() as *mut i8
                as *const i8)
    };
    return 1;
}

/// The input list pList is the list of result set terms from a RETURNING
///* clause.  The table that we are returning from is pTab.
///*
///* This routine makes a copy of the pList, and at the same time expands
///* any "*" wildcards to be the complete set of columns from pTab.
extern "C" fn sqlite3_expand_returning(p_parse_1: *mut Parse,
    p_list_1: &ExprList, p_tab_1: &Table) -> *mut ExprList {
    let mut p_new: *mut ExprList = core::ptr::null_mut();
    let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
    let mut i: i32 = 0;
    {
        i = 0;
        '__b15: loop {
            if !(i < (*p_list_1).n_expr) { break '__b15; }
            '__c15: loop {
                let p_old_expr: *mut Expr =
                    unsafe {
                        (*((*p_list_1).a.as_ptr() as
                                        *mut ExprListItem).offset(i as isize)).p_expr
                    };
                if p_old_expr == core::ptr::null_mut() { break '__c15; }
                if is_asterisk_term(p_parse_1, unsafe { &*p_old_expr }) != 0 {
                    let mut jj: i32 = 0;
                    {
                        jj = 0;
                        '__b16: loop {
                            if !(jj < (*p_tab_1).n_col as i32) { break '__b16; }
                            '__c16: loop {
                                let mut p_new_expr: *mut Expr = core::ptr::null_mut();
                                if unsafe {
                                                    (*unsafe { (*p_tab_1).a_col.offset(jj as isize) }).col_flags
                                                } as i32 & 2 != 0 {
                                    break '__c16;
                                }
                                p_new_expr =
                                    unsafe {
                                        sqlite3_expr(db, 60,
                                            unsafe { (*(*p_tab_1).a_col.offset(jj as isize)).z_cn_name }
                                                as *const i8)
                                    };
                                p_new =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse_1, p_new, p_new_expr)
                                    };
                                if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                                    let p_item: *mut ExprListItem =
                                        unsafe {
                                            &mut *(unsafe { (*p_new).a.as_ptr() } as
                                                            *mut ExprListItem).offset((unsafe { (*p_new).n_expr } - 1)
                                                            as isize)
                                        };
                                    unsafe {
                                        (*p_item).z_e_name =
                                            unsafe {
                                                sqlite3_db_str_dup(db,
                                                    unsafe { (*(*p_tab_1).a_col.offset(jj as isize)).z_cn_name }
                                                        as *const i8)
                                            }
                                    };
                                    unsafe { (*p_item).fg.set_e_e_name(0 as u32 as u32) };
                                }
                                break '__c16;
                            }
                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                        }
                    }
                } else {
                    let p_new_expr_1: *mut Expr =
                        unsafe {
                            sqlite3_expr_dup(db, p_old_expr as *const Expr, 0)
                        };
                    p_new =
                        unsafe {
                            sqlite3_expr_list_append(p_parse_1, p_new, p_new_expr_1)
                        };
                    if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 &&
                            unsafe {
                                    (*((*p_list_1).a.as_ptr() as
                                                    *mut ExprListItem).offset(i as isize)).z_e_name
                                } != core::ptr::null_mut() {
                        let p_item_1: *mut ExprListItem =
                            unsafe {
                                &mut *(unsafe { (*p_new).a.as_ptr() } as
                                                *mut ExprListItem).offset((unsafe { (*p_new).n_expr } - 1)
                                                as isize)
                            };
                        unsafe {
                            (*p_item_1).z_e_name =
                                unsafe {
                                    sqlite3_db_str_dup(db,
                                        unsafe {
                                                (*((*p_list_1).a.as_ptr() as
                                                                *mut ExprListItem).offset(i as isize)).z_e_name
                                            } as *const i8)
                                }
                        };
                        unsafe {
                            (*p_item_1).fg.set_e_e_name(unsafe {
                                        (*((*p_list_1).a.as_ptr() as
                                                            *mut ExprListItem).offset(i as isize)).fg.e_e_name()
                                    } as u32)
                        };
                    }
                }
                break '__c15;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return p_new;
}

///* If the SELECT references the table pWalker->u.pTab, then do two things:
///*
///*    (1) Mark the SELECT as as SF_Correlated.
///*    (2) Set pWalker->eCode to non-zero so that the caller will know
///*        that (1) has happened.
extern "C" fn sqlite3_returning_subquery_correlated(p_walker_1: *mut Walker,
    p_select_1: *mut Select) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p_src: *const SrcList = core::ptr::null();
        { let _ = 0; };
        p_src = unsafe { (*p_select_1).p_src };
        { let _ = 0; };
        {
            i = 0;
            '__b17: loop {
                if !(i < unsafe { (*p_src).n_src }) { break '__b17; }
                '__c17: loop {
                    if unsafe {
                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)).p_s_tab
                            } == unsafe { (*p_walker_1).u.p_tab } {
                        unsafe { (*p_select_1).sel_flags |= 536870912 as u32 };
                        unsafe { (*p_walker_1).e_code = 1 as u16 };
                        break '__b17;
                    }
                    break '__c17;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}

/// If the Expr node is a subquery or an EXISTS operator or an IN operator that
///* uses a subquery, and if the subquery is SF_Correlated, then mark the
///* expression as EP_VarSelect.
extern "C" fn sqlite3_returning_subquery_var_select(not_used_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        { let _ = not_used_1; };
        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 &&
                unsafe { (*unsafe { (*p_expr_1).x.p_select }).sel_flags } &
                        536870912 as u32 != 0 as u32 {
            unsafe { (*p_expr_1).flags |= 64 as u32 };
        }
        return 0;
    }
}

///* Scan the expression list that is the argument to RETURNING looking
///* for subqueries that depend on the table which is being modified in the
///* statement that is hosting the RETURNING clause (pTab).  Mark all such
///* subqueries as SF_Correlated.  If the subqueries are part of an
///* expression, mark the expression as EP_VarSelect.
///*
///* https://sqlite.org/forum/forumpost/2c83569ce8945d39
extern "C" fn sqlite3_process_returning_subqueries(p_e_list_1: *mut ExprList,
    p_tab_1: *mut Table) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.x_expr_callback = Some(sqlite3_expr_walk_noop);
        w.x_select_callback = Some(sqlite3_returning_subquery_correlated);
        w.u.p_tab = p_tab_1;
        unsafe { sqlite3_walk_expr_list(&mut w, p_e_list_1) };
        if w.e_code != 0 {
            w.x_expr_callback = Some(sqlite3_returning_subquery_var_select);
            w.x_select_callback = Some(sqlite3_select_walk_noop);
            unsafe { sqlite3_walk_expr_list(&mut w, p_e_list_1) };
        }
    }
}

///* Generate code for the RETURNING trigger.  Unlike other triggers
///* that invoke a subprogram in the bytecode, the code for RETURNING
///* is generated in-line.
#[allow(unused_doc_comments)]
extern "C" fn code_returning_trigger(p_parse_1: *mut Parse,
    p_trigger_1: *mut Trigger, p_tab_1: *mut Table, reg_in_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let mut p_new: *mut ExprList = core::ptr::null_mut();
        let mut p_returning: *mut Returning = core::ptr::null_mut();
        let mut s_select: Select = unsafe { core::mem::zeroed() };
        let mut p_from: *mut SrcList = core::ptr::null_mut();
        let mut u_src: CodeReturningTriggerU0N25codeReturningTriggerU0 =
            unsafe { core::mem::zeroed() };
        { let _ = 0; };
        if (unsafe { (*p_parse_1).b_returning() } == 0) as i32 != 0 {

            /// This RETURNING trigger must be for a different statement as
            ///* this statement lacks a RETURNING clause.
            return;
        }
        { let _ = 0; };
        { let _ = 0; };
        p_returning = unsafe { (*p_parse_1).u1.d.p_returning };
        if p_trigger_1 !=
                unsafe { &raw mut (*p_returning).ret_trig } as *mut Trigger {

            /// This RETURNING trigger is for a different statement
            return;
        }
        unsafe {
            memset(&raw mut s_select as *mut (), 0,
                core::mem::size_of::<Select>() as u64)
        };
        unsafe { memset(&raw mut u_src as *mut (), 0, 80) };
        p_from = &mut u_src.s_src;
        s_select.p_e_list =
            unsafe {
                sqlite3_expr_list_dup(db,
                    unsafe { (*p_returning).p_return_el } as *const ExprList, 0)
            };
        s_select.p_src = p_from;
        unsafe { (*p_from).n_src = 1 };
        unsafe {
            (*(unsafe { (*p_from).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).p_s_tab = p_tab_1
        };
        unsafe {
            (*(unsafe { (*p_from).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).z_name =
                unsafe { (*p_tab_1).z_name }
        };
        unsafe {

            /// tag-20240424-1
            ((*(unsafe { (*p_from).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).i_cursor = -1)
        };
        unsafe {
            sqlite3_select_prep(p_parse_1, &mut s_select,
                core::ptr::null_mut())
        };
        if unsafe { (*p_parse_1).n_err } == 0 {
            { let _ = 0; };
            unsafe {
                sqlite3_generate_column_names(p_parse_1, &mut s_select)
            };
        }
        unsafe { sqlite3_expr_list_delete(db, s_select.p_e_list) };
        p_new =
            sqlite3_expand_returning(p_parse_1,
                unsafe { &*unsafe { (*p_returning).p_return_el } },
                unsafe { &*p_tab_1 });
        if unsafe { (*p_parse_1).n_err } == 0 {
            let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
            unsafe {
                memset(&raw mut s_nc as *mut (), 0,
                    core::mem::size_of::<NameContext>() as u64)
            };
            if unsafe { (*p_returning).n_ret_col } == 0 {
                unsafe {
                    (*p_returning).n_ret_col = unsafe { (*p_new).n_expr }
                };
                unsafe {
                    (*p_returning).i_ret_cur =
                        {
                            let __p = unsafe { &mut (*p_parse_1).n_tab };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        }
                };
            }
            s_nc.p_parse = p_parse_1;
            s_nc.u_nc.i_base_reg = reg_in_1;
            s_nc.nc_flags = 1024;
            unsafe {
                (*p_parse_1).e_trigger_op = unsafe { (*p_trigger_1).op }
            };
            unsafe { (*p_parse_1).p_trigger_tab = p_tab_1 };
            if unsafe { sqlite3_resolve_expr_list_names(&mut s_nc, p_new) } ==
                        0 && (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                let mut i: i32 = 0;
                let n_col: i32 = unsafe { (*p_new).n_expr };
                let reg: i32 = unsafe { (*p_parse_1).n_mem } + 1;
                sqlite3_process_returning_subqueries(p_new, p_tab_1);
                unsafe { (*p_parse_1).n_mem += n_col + 2 };
                unsafe { (*p_returning).i_ret_reg = reg };
                {
                    i = 0;
                    '__b18: loop {
                        if !(i < n_col) { break '__b18; }
                        '__c18: loop {
                            let p_col: *mut Expr =
                                unsafe {
                                    (*(unsafe { (*p_new).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).p_expr
                                };
                            { let _ = 0; };

                            /// Due to !db->mallocFailed ~9 lines above
                            unsafe {
                                sqlite3_expr_code_factorable(p_parse_1, p_col, reg + i)
                            };
                            if unsafe { sqlite3_expr_affinity(p_col as *const Expr) } as
                                        i32 == 69 {
                                unsafe { sqlite3_vdbe_add_op1(v, 89, reg + i) };
                            }
                            break '__c18;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { sqlite3_vdbe_add_op3(v, 99, reg, i, reg + i) };
                unsafe {
                    sqlite3_vdbe_add_op2(v, 129,
                        unsafe { (*p_returning).i_ret_cur }, reg + i + 1)
                };
                unsafe {
                    sqlite3_vdbe_add_op3(v, 130,
                        unsafe { (*p_returning).i_ret_cur }, reg + i, reg + i + 1)
                };
            }
        }
        unsafe { sqlite3_expr_list_delete(db, p_new) };
        unsafe { (*p_parse_1).e_trigger_op = 0 as u8 };
        unsafe { (*p_parse_1).p_trigger_tab = core::ptr::null_mut() };
    }
}

///* This is called to code the required FOR EACH ROW triggers for an operation
///* on table pTab. The operation to code triggers for (INSERT, UPDATE or DELETE)
///* is given by the op parameter. The tr_tm parameter determines whether the
///* BEFORE or AFTER triggers are coded. If the operation is an UPDATE, then
///* parameter pChanges is passed the list of columns being modified.
///*
///* If there are no triggers that fire at the specified time for the specified
///* operation on pTab, this function is a no-op.
///*
///* The reg argument is the address of the first in an array of registers 
///* that contain the values substituted for the new.* and old.* references
///* in the trigger program. If N is the number of columns in table pTab
///* (a copy of pTab->nCol), then registers are populated as follows:
///*
///*   Register       Contains
///*   ------------------------------------------------------
///*   reg+0          OLD.rowid
///*   reg+1          OLD.* value of left-most column of pTab
///*   ...            ...
///*   reg+N          OLD.* value of right-most column of pTab
///*   reg+N+1        NEW.rowid
///*   reg+N+2        NEW.* value of left-most column of pTab
///*   ...            ...
///*   reg+N+N+1      NEW.* value of right-most column of pTab
///*
///* For ON DELETE triggers, the registers containing the NEW.* values will
///* never be accessed by the trigger program, so they are not allocated or 
///* populated by the caller (there is no data to populate them with anyway). 
///* Similarly, for ON INSERT triggers the values stored in the OLD.* registers
///* are never accessed, and so are not allocated by the caller. So, for an
///* ON INSERT trigger, the value passed to this function as parameter reg
///* is not a readable register, although registers (reg+N) through 
///* (reg+N+N+1) are.
///*
///* Parameter orconf is the default conflict resolution algorithm for the
///* trigger program to use (REPLACE, IGNORE etc.). Parameter ignoreJump
///* is the instruction that control should jump to if a trigger program
///* raises an IGNORE exception.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_code_row_trigger(p_parse: *mut Parse,
    p_trigger: *mut Trigger, op: i32, p_changes: *mut ExprList, tr_tm: i32,
    p_tab: *mut Table, reg: i32, orconf: i32, ignore_jump: i32) -> () {
    let mut p: *mut Trigger = core::ptr::null_mut();

    /// Used to iterate through pTrigger list
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    {
        p = p_trigger;
        '__b19: loop {
            if !(!(p).is_null()) { break '__b19; }
            '__c19: loop {

                /// Sanity checking:  The schema for the trigger and for the table are
                ///* always defined.  The trigger must be in the same schema as the table
                ///* or else it must be a TEMP trigger.
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                if (unsafe { (*p).op } as i32 == op ||
                                unsafe { (*p).b_returning } != 0 &&
                                        unsafe { (*p).op } as i32 == 128 && op == 130) &&
                            unsafe { (*p).tr_tm } as i32 == tr_tm &&
                        check_column_overlap(unsafe { (*p).p_columns },
                                p_changes as *const ExprList) != 0 {
                    if (unsafe { (*p).b_returning } == 0) as i32 != 0 {
                        sqlite3_code_row_trigger_direct(p_parse, p, p_tab, reg,
                            orconf, ignore_jump);
                    } else if unsafe { (*p_parse).p_toplevel } ==
                            core::ptr::null_mut() {
                        code_returning_trigger(p_parse, p, p_tab, reg);
                    }
                }
                break '__c19;
            }
            p = unsafe { (*p).p_next };
        }
    }
}

///* Duplicate a range of text from an SQL statement, then convert all
///* whitespace characters into ordinary space characters.
extern "C" fn trigger_span_dup(db: *mut Sqlite3, z_start_1: *const i8,
    z_end_1: *const i8) -> *mut i8 {
    unsafe {
        let z: *mut i8 =
            unsafe { sqlite3_db_span_dup(db, z_start_1, z_end_1) };
        let mut i: i32 = 0;
        if !(z).is_null() {
            {
                i = 0;
                '__b20: loop {
                    if !(unsafe { *z.offset(i as isize) } != 0) {
                        break '__b20;
                    }
                    '__c20: loop {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset(i as isize) } as u8 as
                                                        usize)
                                        } as i32 & 1 != 0 {
                            unsafe { *z.offset(i as isize) = ' ' as i32 as i8 };
                        }
                        break '__c20;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return z;
    }
}

///* Turn a SELECT statement (that the pSelect parameter points to) into
///* a trigger step.  Return a pointer to a TriggerStep structure.
///*
///* The parser calls this routine when it finds a SELECT statement in
///* body of a TRIGGER.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_trigger_select_step(db: *mut Sqlite3,
    p_select: *mut Select, z_start: *const i8, z_end: *const i8)
    -> *mut TriggerStep {
    unsafe {
        let p_trigger_step: *mut TriggerStep =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<TriggerStep>() as u64)
                } as *mut TriggerStep;
        if p_trigger_step == core::ptr::null_mut() {
            unsafe { sqlite3_select_delete(db, p_select) };
            return core::ptr::null_mut();
        }
        unsafe { (*p_trigger_step).op = 139 as u8 };
        unsafe { (*p_trigger_step).p_select = p_select };
        unsafe { (*p_trigger_step).orconf = 11 as u8 };
        unsafe {
            (*p_trigger_step).z_span = trigger_span_dup(db, z_start, z_end)
        };
        return p_trigger_step;
    }
}

///* Allocate space to hold a new trigger step.  The allocated space
///* holds both the TriggerStep object and the TriggerStep.target.z string.
///*
///* If an OOM error occurs, NULL is returned and db->mallocFailed is set.
extern "C" fn trigger_step_allocate(p_parse_1: *mut Parse, op: u8,
    p_tab_list_1: *mut SrcList, z_start_1: *const i8, z_end_1: *const i8)
    -> *mut TriggerStep {
    unsafe {
        let p_new: *const Trigger =
            unsafe { (*p_parse_1).p_new_trigger } as *const Trigger;
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let mut p_trigger_step: *mut TriggerStep = core::ptr::null_mut();
        if unsafe { (*p_parse_1).n_err } == 0 {
            if !(p_new).is_null() &&
                        unsafe { (*p_new).p_schema } !=
                            unsafe {
                                (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                            } &&
                    !(unsafe {
                                    (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).u4.z_database
                                }).is_null() {
                unsafe {
                    sqlite3_error_msg(p_parse_1,
                        c"qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers".as_ptr()
                                as *mut i8 as *const i8)
                };
            } else {
                p_trigger_step =
                    unsafe {
                            sqlite3_db_malloc_zero(db,
                                core::mem::size_of::<TriggerStep>() as u64)
                        } as *mut TriggerStep;
                if !(p_trigger_step).is_null() {
                    unsafe {
                        (*p_trigger_step).p_src =
                            unsafe {
                                sqlite3_src_list_dup(db, p_tab_list_1 as *const SrcList, 1)
                            }
                    };
                    unsafe { (*p_trigger_step).op = op };
                    unsafe {
                        (*p_trigger_step).z_span =
                            trigger_span_dup(db, z_start_1, z_end_1)
                    };
                    if !(unsafe { (*p_trigger_step).p_src }).is_null() &&
                            unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
                        unsafe {
                            sqlite3_rename_token_remap(p_parse_1,
                                unsafe {
                                        (*(unsafe {
                                                            (*unsafe { (*p_trigger_step).p_src }).a.as_ptr()
                                                        } as *mut SrcItem).offset(0 as isize)).z_name
                                    } as *const (),
                                unsafe {
                                        (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).z_name
                                    } as *const ())
                        };
                    }
                }
            }
        }
        unsafe { sqlite3_src_list_delete(db, p_tab_list_1) };
        return p_trigger_step;
    }
}

///* Build a trigger step out of an INSERT statement.  Return a pointer
///* to the new trigger step.
///*
///* The parser calls this routine when it sees an INSERT inside the
///* body of a trigger.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_trigger_insert_step(p_parse: *mut Parse,
    p_tab_list: *mut SrcList, p_column: *mut IdList,
    mut p_select: *mut Select, orconf: u8, p_upsert: *mut Upsert,
    z_start: *const i8, z_end: *const i8) -> *mut TriggerStep {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        let mut p_trigger_step: *mut TriggerStep = core::ptr::null_mut();
        { let _ = 0; };
        p_trigger_step =
            trigger_step_allocate(p_parse, 128 as u8, p_tab_list, z_start,
                z_end);
        if !(p_trigger_step).is_null() {
            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                unsafe { (*p_trigger_step).p_select = p_select };
                p_select = core::ptr::null_mut();
            } else {
                unsafe {
                    (*p_trigger_step).p_select =
                        unsafe {
                            sqlite3_select_dup(db, p_select as *const Select, 1)
                        }
                };
            }
            unsafe { (*p_trigger_step).p_id_list = p_column };
            unsafe { (*p_trigger_step).p_upsert = p_upsert };
            unsafe { (*p_trigger_step).orconf = orconf };
            if !(p_upsert).is_null() {
                unsafe {
                    sqlite3_has_explicit_nulls(p_parse,
                        unsafe { (*p_upsert).p_upsert_target })
                };
            }
        } else {
            unsafe { sqlite3_id_list_delete(db, p_column) };
            unsafe { sqlite3_upsert_delete(db, p_upsert) };
        }
        unsafe { sqlite3_select_delete(db, p_select) };
        return p_trigger_step;
    }
}

///* Construct a trigger step that implements an UPDATE statement and return
///* a pointer to that trigger step.  The parser calls this routine when it
///* sees an UPDATE statement inside the body of a CREATE TRIGGER.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_trigger_update_step(p_parse: *mut Parse,
    p_tab_list: *mut SrcList, mut p_from: *mut SrcList,
    mut p_e_list: *mut ExprList, mut p_where: *mut Expr, orconf: u8,
    z_start: *const i8, z_end: *const i8) -> *mut TriggerStep {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let mut p_trigger_step: *mut TriggerStep = core::ptr::null_mut();
    p_trigger_step =
        trigger_step_allocate(p_parse, 130 as u8, p_tab_list, z_start, z_end);
    if !(p_trigger_step).is_null() {
        let mut p_from_dup: *mut SrcList = core::ptr::null_mut();
        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
            unsafe { (*p_trigger_step).p_expr_list = p_e_list };
            unsafe { (*p_trigger_step).p_where = p_where };
            p_from_dup = p_from;
            p_e_list = core::ptr::null_mut();
            p_where = core::ptr::null_mut();
            p_from = core::ptr::null_mut();
        } else {
            unsafe {
                (*p_trigger_step).p_expr_list =
                    unsafe {
                        sqlite3_expr_list_dup(db, p_e_list as *const ExprList, 1)
                    }
            };
            unsafe {
                (*p_trigger_step).p_where =
                    unsafe { sqlite3_expr_dup(db, p_where as *const Expr, 1) }
            };
            p_from_dup =
                unsafe {
                    sqlite3_src_list_dup(db, p_from as *const SrcList, 1)
                };
        }
        unsafe { (*p_trigger_step).orconf = orconf };
        if !(p_from_dup).is_null() &&
                !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 != 0
            {
            let mut p_sub: *mut Select = core::ptr::null_mut();
            let mut as_: Token = Token { z: core::ptr::null(), n: 0 as u32 };
            p_sub =
                unsafe {
                    sqlite3_select_new(p_parse, core::ptr::null_mut(),
                        p_from_dup, core::ptr::null_mut(), core::ptr::null_mut(),
                        core::ptr::null_mut(), core::ptr::null_mut(), 2048 as u32,
                        core::ptr::null_mut())
                };
            p_from_dup =
                unsafe {
                    sqlite3_src_list_append_from_term(p_parse,
                        core::ptr::null_mut(), core::ptr::null_mut(),
                        core::ptr::null_mut(), &mut as_, p_sub,
                        core::ptr::null_mut())
                };
        }
        if !(p_from_dup).is_null() &&
                !(unsafe { (*p_trigger_step).p_src }).is_null() {
            unsafe {
                (*p_trigger_step).p_src =
                    unsafe {
                        sqlite3_src_list_append_list(p_parse,
                            unsafe { (*p_trigger_step).p_src }, p_from_dup)
                    }
            };
        } else { unsafe { sqlite3_src_list_delete(db, p_from_dup) }; }
    }
    unsafe { sqlite3_expr_list_delete(db, p_e_list) };
    unsafe { sqlite3_expr_delete(db, p_where) };
    unsafe { sqlite3_src_list_delete(db, p_from) };
    return p_trigger_step;
}

///* Construct a trigger step that implements a DELETE statement and return
///* a pointer to that trigger step.  The parser calls this routine when it
///* sees a DELETE statement inside the body of a CREATE TRIGGER.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_trigger_delete_step(p_parse: *mut Parse,
    p_tab_list: *mut SrcList, mut p_where: *mut Expr, z_start: *const i8,
    z_end: *const i8) -> *mut TriggerStep {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let mut p_trigger_step: *mut TriggerStep = core::ptr::null_mut();
    p_trigger_step =
        trigger_step_allocate(p_parse, 129 as u8, p_tab_list, z_start, z_end);
    if !(p_trigger_step).is_null() {
        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
            unsafe { (*p_trigger_step).p_where = p_where };
            p_where = core::ptr::null_mut();
        } else {
            unsafe {
                (*p_trigger_step).p_where =
                    unsafe { sqlite3_expr_dup(db, p_where as *const Expr, 1) }
            };
        }
        unsafe { (*p_trigger_step).orconf = 11 as u8 };
    }
    unsafe { sqlite3_expr_delete(db, p_where) };
    return p_trigger_step;
}

///* Remove a trigger from the hash tables of the sqlite* pointer.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_unlink_and_delete_trigger(db: *mut Sqlite3,
    i_db: i32, z_name: *const i8) -> () {
    unsafe {
        let mut p_trigger: *mut Trigger = core::ptr::null_mut();
        let mut p_hash: *mut Hash = core::ptr::null_mut();
        { let _ = 0; };
        p_hash =
            unsafe {
                &mut (*unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                }).trig_hash
            };
        p_trigger =
            unsafe {
                    sqlite3_hash_insert(p_hash, z_name, core::ptr::null_mut())
                } as *mut Trigger;
        if !(p_trigger).is_null() {
            if unsafe { (*p_trigger).p_schema } ==
                    unsafe { (*p_trigger).p_tab_schema } {
                let p_tab: *mut Table =
                    table_of_trigger(unsafe { &*p_trigger });
                if !(p_tab).is_null() {
                    let mut pp: *mut *mut Trigger = core::ptr::null_mut();
                    {
                        pp = unsafe { &mut (*p_tab).p_trigger };
                        '__b21: loop {
                            if !(!(unsafe { *pp }).is_null()) { break '__b21; }
                            '__c21: loop {
                                if unsafe { *pp } == p_trigger {
                                    unsafe { *pp = unsafe { (*unsafe { *pp }).p_next } };
                                    break '__b21;
                                }
                                break '__c21;
                            }
                            pp = unsafe { &mut (*unsafe { *pp }).p_next };
                        }
                    }
                }
            }
            sqlite3_delete_trigger(db, p_trigger);
            unsafe { (*db).m_db_flags |= 1 as u32 };
        }
    }
}

///* Triggers may access values stored in the old.* or new.* pseudo-table. 
///* This function returns a 32-bit bitmask indicating which columns of the 
///* old.* or new.* tables actually are used by triggers. This information 
///* may be used by the caller, for example, to avoid having to load the entire
///* old.* record into memory when executing an UPDATE or DELETE command.
///*
///* Bit 0 of the returned mask is set if the left-most column of the
///* table may be accessed using an [old|new].<col> reference. Bit 1 is set if
///* the second leftmost column value is required, and so on. If there
///* are more than 32 columns in the table, and at least one of the columns
///* with an index greater than 32 may be accessed, 0xffffffff is returned.
///*
///* It is not possible to determine if the old.rowid or new.rowid column is 
///* accessed by triggers. The caller must always assume that it is.
///*
///* Parameter isNew must be either 1 or 0. If it is 0, then the mask returned
///* applies to the old.* table. If 1, the new.* table.
///*
///* Parameter tr_tm must be a mask with one or both of the TRIGGER_BEFORE
///* and TRIGGER_AFTER bits set. Values accessed by BEFORE triggers are only
///* included in the returned mask if the TRIGGER_BEFORE bit is set in the
///* tr_tm parameter. Similarly, values accessed by AFTER triggers are only
///* included in the returned mask if the TRIGGER_AFTER bit is set in tr_tm.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_trigger_colmask(p_parse: *mut Parse,
    p_trigger: *mut Trigger, p_changes: *mut ExprList, is_new: i32,
    tr_tm: i32, p_tab: *mut Table, orconf: i32) -> u32 {
    let op: i32 = if !(p_changes).is_null() { 130 } else { 129 } as i32;
    let mut mask: u32 = 0 as u32;
    let mut p: *mut Trigger = core::ptr::null_mut();
    { let _ = 0; };
    if unsafe { (*p_tab).e_tab_type } as i32 == 2 { return 4294967295u32; }
    {
        p = p_trigger;
        '__b22: loop {
            if !(!(p).is_null()) { break '__b22; }
            '__c22: loop {
                if unsafe { (*p).op } as i32 == op &&
                            tr_tm & unsafe { (*p).tr_tm } as i32 != 0 &&
                        check_column_overlap(unsafe { (*p).p_columns },
                                p_changes as *const ExprList) != 0 {
                    if unsafe { (*p).b_returning } != 0 {
                        mask = 4294967295u32;
                    } else {
                        let mut p_prg: *const TriggerPrg = core::ptr::null();
                        p_prg = get_row_trigger(p_parse, p, p_tab, orconf);
                        if !(p_prg).is_null() {
                            mask |= unsafe { (*p_prg).a_colmask[is_new as usize] };
                        }
                    }
                }
                break '__c22;
            }
            p = unsafe { (*p).p_next };
        }
    }
    return mask;
}

#[repr(C)]
#[derive(Copy, Clone)]
union CodeReturningTriggerU0N25codeReturningTriggerU0 {
    s_src: SrcList,
    from_space: [u8; 80],
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
    fn sqlite3_two_part_name(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut *mut Token)
    -> i32;
    fn sqlite3_fix_init(_: *mut DbFixer, _: *mut Parse, _: i32, _: *const i8,
    _: *const Token)
    -> ();
    fn sqlite3_fix_src_list(_: *mut DbFixer, _: *mut SrcList)
    -> i32;
    fn sqlite3_read_only_shadow_tables(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_check_object_name(_: *mut Parse, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_rename_token_remap(_: *mut Parse, p_to_1: *const (),
    p_from_1: *const ())
    -> ();
    fn sqlite3_upsert_delete(_: *mut Sqlite3, _: *mut Upsert)
    -> ();
    fn sqlite3_fix_trigger_step(_: *mut DbFixer, _: *mut TriggerStep)
    -> i32;
    fn sqlite3_fix_expr(_: *mut DbFixer, _: *mut Expr)
    -> i32;
    fn sqlite3_shadow_table_name(db: *mut Sqlite3, z_name_1: *const i8)
    -> i32;
    fn sqlite3_nested_parse(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_db_is_named(db: *mut Sqlite3, i_db_1: i32, z_name_1: *const i8)
    -> i32;
    fn sqlite3_parse_object_init(_: *mut Parse, _: *mut Sqlite3)
    -> ();
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
    -> i32;
    fn sqlite3_upsert_dup(_: *mut Sqlite3, _: *mut Upsert)
    -> *mut Upsert;
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_parse_object_reset(_: *mut Parse)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_select_prep(_: *mut Parse, _: *mut Select, _: *mut NameContext)
    -> ();
    fn sqlite3_resolve_expr_list_names(_: *mut NameContext, _: *mut ExprList)
    -> i32;
    fn sqlite3_expr_affinity(p_expr_1: *const Expr)
    -> i8;
    fn sqlite_view_triggers(_: *mut Parse, _: *mut Table, _: *mut Expr,
    _: i32, _: *mut ExprList)
    -> ();
    static sqlite3_ctype_map: [u8; 0];
    fn sqlite3_has_explicit_nulls(_: *mut Parse, _: *mut ExprList)
    -> i32;
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
    fn sqlite3_auth_context_push(_: *mut Parse, _: *mut AuthContext,
    _: *const i8)
    -> ();
    fn sqlite3_auth_context_pop(_: *mut AuthContext)
    -> ();
    fn sqlite3_auth_read_col(_: *mut Parse, _: *const i8, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_attach(_: *mut Parse, _: *mut Expr, _: *mut Expr, _: *mut Expr)
    -> ();
    fn sqlite3_detach(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_fix_select(_: *mut DbFixer, _: *mut Select)
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
    fn sqlite3_memdb_init()
    -> i32;
    fn sqlite3_is_memdb(_: *const Sqlite3Vfs)
    -> i32;
    fn sqlite3_err_str(_: i32)
    -> *const i8;
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
    fn sqlite3_expire_prepared_statements(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_code_rhs_of_in(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_code_subselect(_: *mut Parse, _: *mut Expr)
    -> i32;
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
    fn sqlite3_key_info_of_index(_: *mut Parse, _: *mut Index)
    -> *mut KeyInfo;
    fn sqlite3_key_info_from_expr_list(_: *mut Parse, _: *mut ExprList,
    _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_select_op_name(_: i32)
    -> *const i8;
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
