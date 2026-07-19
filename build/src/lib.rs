#![feature(c_variadic)]
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
    AuthContext, Bft, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte, Db,
    DbFixer, Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode,
    FuncDef, FuncDefHash, FuncDestructor, IdList, IdListItem, Index, KeyInfo,
    LogEst, Module, NameContext, OnOrUsing, Parse, Returning, RowSet, SColMap,
    SQLiteThread, Schema, Select, SelectDest, Sqlite3, Sqlite3Config,
    Sqlite3InitInfo, Sqlite3Str, SrcItem, SrcItemS0, SrcList, StrAccum,
    Subquery, Table, Token, Trigger, TriggerStep, UnpackedRecord, Upsert,
    VList, VTable, Walker, WhereInfo, Window, With, YDbMask,
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

///* The TableLock structure is only used by the sqlite3TableLock() and
///* codeTableLocks() functions.
#[repr(C)]
#[derive(Copy, Clone)]
struct TableLock {
    i_db: i32,
    i_tab: Pgno,
    is_write_lock: u8,
    z_lock_name: *const i8,
}

///* Code an OP_TableLock instruction for each table locked by the
///* statement (configured by calls to sqlite3TableLock()).
extern "C" fn code_table_locks(p_parse_1: &Parse) -> () {
    let mut i: i32 = 0;
    let p_vdbe: *mut Vdbe = (*p_parse_1).p_vdbe;
    { let _ = 0; };
    {
        i = 0;
        '__b0: loop {
            if !(i < (*p_parse_1).n_table_lock) { break '__b0; }
            '__c0: loop {
                let p: *const TableLock =
                    unsafe {
                            &raw mut *(*p_parse_1).a_table_lock.offset(i as isize)
                        } as *const TableLock;
                let p1: i32 = unsafe { (*p).i_db };
                unsafe {
                    sqlite3_vdbe_add_op4(p_vdbe, 171, p1,
                        unsafe { (*p).i_tab } as i32,
                        unsafe { (*p).is_write_lock } as i32,
                        unsafe { (*p).z_lock_name }, -1)
                };
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* This routine is called after a single SQL statement has been
///* parsed and a VDBE program to execute that statement has been
///* prepared.  This routine puts the finishing touches on the
///* VDBE program and resets the pParse structure for the next
///* parse.
///*
///* Note that if an error occurred, it might be the case that
///* no VDBE code was generated.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_finish_coding(p_parse: *mut Parse) -> () {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let mut i_db: i32 = 0;
        let mut i: i32 = 0;
        { let _ = 0; };
        db = unsafe { (*p_parse).db };
        { let _ = 0; };
        if unsafe { (*p_parse).nested } != 0 { return; }
        if unsafe { (*p_parse).n_err } != 0 {
            if unsafe { (*db).malloc_failed } != 0 {
                unsafe { (*p_parse).rc = 7 };
            }
            return;
        }
        { let _ = 0; };

        /// Begin by generating some termination code at the end of the
        ///* vdbe program
        (v = unsafe { (*p_parse).p_vdbe });
        if v == core::ptr::null_mut() {
            if unsafe { (*db).init.busy } != 0 {
                unsafe { (*p_parse).rc = 101 };
                return;
            }
            v = unsafe { sqlite3_get_vdbe(p_parse) };
            if v == core::ptr::null_mut() { unsafe { (*p_parse).rc = 1 }; }
        }
        { let _ = 0; };
        if !(v).is_null() {
            if unsafe { (*p_parse).b_returning() } != 0 {
                let mut p_returning: *const Returning = core::ptr::null();
                let mut addr_rewind: i32 = 0;
                let mut reg: i32 = 0;
                { let _ = 0; };
                p_returning = unsafe { (*p_parse).u1.d.p_returning };
                if unsafe { (*p_returning).n_ret_col } != 0 {
                    unsafe { sqlite3_vdbe_add_op0(v, 85) };
                    addr_rewind =
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 36,
                                unsafe { (*p_returning).i_ret_cur })
                        };
                    reg = unsafe { (*p_returning).i_ret_reg };
                    {
                        i = 0;
                        '__b1: loop {
                            if !(i < unsafe { (*p_returning).n_ret_col }) {
                                break '__b1;
                            }
                            '__c1: loop {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 96,
                                        unsafe { (*p_returning).i_ret_cur }, i, reg + i)
                                };
                                break '__c1;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe { sqlite3_vdbe_add_op2(v, 86, reg, i) };
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 40,
                            unsafe { (*p_returning).i_ret_cur }, addr_rewind + 1)
                    };
                    unsafe { sqlite3_vdbe_jump_here(v, addr_rewind) };
                }
            }
            unsafe { sqlite3_vdbe_add_op0(v, 72) };

            /// The cookie mask contains one bit for each database file open.
            ///* (Bit 0 is for main, bit 1 is for temp, and so forth.)  Bits are
            ///* set for each database that is used.  Generate code to start a
            ///* transaction on each used database and to verify the schema cookie
            ///* on each used database.
            { let _ = 0; };
            unsafe { sqlite3_vdbe_jump_here(v, 0) };
            { let _ = 0; };
            i_db = 0;
            '__b2: loop {
                '__c2: loop {
                    let mut p_schema: *const Schema = core::ptr::null();
                    if (unsafe { (*p_parse).cookie_mask } &
                                        (1 as YDbMask) << i_db != 0 as u32) as i32 == 0 {
                        break '__c2;
                    }
                    unsafe { sqlite3_vdbe_uses_btree(v, i_db) };
                    p_schema =
                        unsafe {
                            (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                        };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 2, i_db,
                            (unsafe { (*p_parse).write_mask } & (1 as YDbMask) << i_db
                                    != 0 as u32) as i32, unsafe { (*p_schema).schema_cookie },
                            unsafe { (*p_schema).i_generation })
                    };
                    if unsafe { (*db).init.busy } as i32 == 0 {
                        unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
                    }
                    break '__c2;
                }
                if !({ let __p = &mut i_db; *__p += 1; *__p } <
                                unsafe { (*db).n_db }) {
                    break '__b2;
                }
            }
            {
                i = 0;
                '__b3: loop {
                    if !(i < unsafe { (*p_parse).n_vtab_lock }) { break '__b3; }
                    '__c3: loop {
                        let vtab: *const i8 =
                            unsafe {
                                        sqlite3_get_v_table(db,
                                            unsafe {
                                                *unsafe { (*p_parse).ap_vtab_lock.offset(i as isize) }
                                            })
                                    } as *mut i8 as *const i8;
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 172, 0, 0, 0, vtab as *const i8,
                                -12)
                        };
                        break '__c3;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*p_parse).n_vtab_lock = 0 };
            if unsafe { (*p_parse).n_table_lock } != 0 {
                code_table_locks(unsafe { &*p_parse });
            }
            if unsafe { (*p_parse).uses_ainc() } != 0 {
                unsafe { sqlite3_autoincrement_begin(p_parse) };
            }
            if !(unsafe { (*p_parse).p_const_expr }).is_null() {
                let p_el: *const ExprList =
                    unsafe { (*p_parse).p_const_expr } as *const ExprList;
                unsafe { (*p_parse).set_ok_const_factor(0 as Bft as u32) };
                {
                    i = 0;
                    '__b4: loop {
                        if !(i < unsafe { (*p_el).n_expr }) { break '__b4; }
                        '__c4: loop {
                            { let _ = 0; };
                            unsafe {
                                sqlite3_expr_code(p_parse,
                                    unsafe {
                                        (*(unsafe { (*p_el).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                    },
                                    unsafe {
                                        (*(unsafe { (*p_el).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).u.i_const_expr_reg
                                    })
                            };
                            break '__c4;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            if unsafe { (*p_parse).b_returning() } != 0 {
                let mut p_ret: *const Returning = core::ptr::null();
                { let _ = 0; };
                p_ret = unsafe { (*p_parse).u1.d.p_returning };
                if unsafe { (*p_ret).n_ret_col } != 0 {
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 120, unsafe { (*p_ret).i_ret_cur },
                            unsafe { (*p_ret).n_ret_col })
                    };
                }
            }

            /// Finally, jump back to the beginning of the executable code.
            unsafe { sqlite3_vdbe_goto(v, 1) };
        }

        /// Get the VDBE program ready for execution
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_parse).n_err } == 0 {

            /// A minimum of one cursor is required if autoincrement is used
            /// See ticket [a696379c1f08866]
            { let _ = 0; };
            unsafe { sqlite3_vdbe_make_ready(v, p_parse) };
            unsafe { (*p_parse).rc = 101 };
        } else { unsafe { (*p_parse).rc = 1 }; }
    }
}

///* Look through the list of open database files in db->aDb[] and if
///* any have been closed, remove them from the list.  Reallocate the
///* db->aDb[] structure to a smaller size, if possible.
///*
///* Entry 0 (the "main" database) and entry 1 (the "temp" database)
///* are never candidates for being collapsed.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_collapse_database_array(db: *mut Sqlite3) -> () {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        i = { j = 2; j };
        '__b5: loop {
            if !(i < unsafe { (*db).n_db }) { break '__b5; }
            '__c5: loop {
                let p_db: *mut Db =
                    unsafe { &mut *unsafe { (*db).a_db.offset(i as isize) } };
                if unsafe { (*p_db).p_bt } == core::ptr::null_mut() {
                    unsafe {
                        sqlite3_db_free(db,
                            unsafe { (*p_db).z_db_s_name } as *mut ())
                    };
                    unsafe { (*p_db).z_db_s_name = core::ptr::null_mut() };
                    break '__c5;
                }
                if j < i {
                    unsafe {
                        *unsafe { (*db).a_db.offset(j as isize) } =
                            unsafe { *unsafe { (*db).a_db.offset(i as isize) } }
                    };
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                break '__c5;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { (*db).n_db = j };
    if unsafe { (*db).n_db } <= 2 &&
            unsafe { (*db).a_db } !=
                unsafe { &raw mut (*db).a_db_static[0 as usize] } as *mut Db {
        unsafe {
            memcpy(unsafe { &raw mut (*db).a_db_static[0 as usize] } as
                        *mut Db as *mut (), unsafe { (*db).a_db } as *const (),
                2 as u64 * core::mem::size_of::<Db>() as u64)
        };
        unsafe { sqlite3_db_free(db, unsafe { (*db).a_db } as *mut ()) };
        unsafe {
            (*db).a_db =
                unsafe { &raw mut (*db).a_db_static[0 as usize] } as *mut Db
        };
    }
}

///* Erase all schema information from all attached databases (including
///* "main" and "temp") for a single database connection.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_reset_all_schemas_of_connection(db: *mut Sqlite3)
    -> () {
    unsafe {
        let mut i: i32 = 0;
        unsafe { sqlite3_btree_enter_all(db) };
        {
            i = 0;
            '__b6: loop {
                if !(i < unsafe { (*db).n_db }) { break '__b6; }
                '__c6: loop {
                    let p_db: *const Db =
                        unsafe {
                                &raw mut *unsafe { (*db).a_db.offset(i as isize) }
                            } as *const Db;
                    if !(unsafe { (*p_db).p_schema }).is_null() {
                        if unsafe { (*db).n_schema_lock } == 0 as u32 {
                            unsafe {
                                sqlite3_schema_clear(unsafe { (*p_db).p_schema } as *mut ())
                            };
                        } else {
                            unsafe {
                                (*unsafe {
                                                    (*unsafe { (*db).a_db.offset(i as isize) }).p_schema
                                                }).schema_flags |= 8 as u16
                            };
                        }
                    }
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { (*db).m_db_flags &= !(1 | 16) as u32 };
        unsafe { sqlite3_vtab_unlock_list(db) };
        unsafe { sqlite3_btree_leave_all(db) };
        if unsafe { (*db).n_schema_lock } == 0 as u32 {
            sqlite3_collapse_database_array(db);
        }
    }
}

///* Reset the schema for the database at index iDb.  Also reset the
///* TEMP schema.  The reset is deferred if db->nSchemaLock is not zero.
///* Deferred resets may be run by calling with iDb<0.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_reset_one_schema(db: &mut Sqlite3, i_db: i32)
    -> () {
    unsafe {
        let mut i: i32 = 0;
        { let _ = 0; };
        if i_db >= 0 {
            { let _ = 0; };
            unsafe {
                (*unsafe {
                                    (*(*db).a_db.offset(i_db as isize)).p_schema
                                }).schema_flags |= 8 as u16
            };
            unsafe {
                (*unsafe {
                                    (*(*db).a_db.offset(1 as isize)).p_schema
                                }).schema_flags |= 8 as u16
            };
            (*db).m_db_flags &= !16 as u32;
        }
        if (*db).n_schema_lock == 0 as u32 {
            {
                i = 0;
                '__b7: loop {
                    if !(i < (*db).n_db) { break '__b7; }
                    '__c7: loop {
                        if unsafe {
                                            (*unsafe {
                                                            (*(*db).a_db.offset(i as isize)).p_schema
                                                        }).schema_flags
                                        } as i32 & 8 == 8 {
                            unsafe {
                                sqlite3_schema_clear(unsafe {
                                            (*(*db).a_db.offset(i as isize)).p_schema
                                        } as *mut ())
                            };
                        }
                        break '__c7;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
    }
}

///* This routine is called when a commit occurs.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_commit_internal_changes(db: &mut Sqlite3) -> () {
    (*db).m_db_flags &= !1 as u32;
}

///* Set the expression associated with a column.  This is usually
///* the DEFAULT value, but might also be the expression that computes
///* the value for a generated column.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_set_expr(p_parse: *mut Parse,
    p_tab: &mut Table, p_col: &mut Column, p_expr: *mut Expr) -> () {
    unsafe {
        let mut p_list: *mut ExprList = core::ptr::null_mut();
        { let _ = 0; };
        p_list = (*p_tab).u.tab.p_dflt_list;
        if (*p_col).i_dflt as i32 == 0 || p_list == core::ptr::null_mut() ||
                unsafe { (*p_list).n_expr } < (*p_col).i_dflt as i32 {
            (*p_col).i_dflt =
                if p_list == core::ptr::null_mut() {
                        1
                    } else { (unsafe { (*p_list).n_expr }) + 1 } as u16;
            (*p_tab).u.tab.p_dflt_list =
                unsafe { sqlite3_expr_list_append(p_parse, p_list, p_expr) };
        } else {
            unsafe {
                sqlite3_expr_delete(unsafe { (*p_parse).db },
                    unsafe {
                        (*(unsafe { (*p_list).a.as_ptr() } as
                                        *mut ExprListItem).offset(((*p_col).i_dflt as i32 - 1) as
                                        isize)).p_expr
                    })
            };
            unsafe {
                (*(unsafe { (*p_list).a.as_ptr() } as
                                    *mut ExprListItem).offset(((*p_col).i_dflt as i32 - 1) as
                                    isize)).p_expr = p_expr
            };
        }
    }
}

///* Return the expression associated with a column.  The expression might be
///* the DEFAULT clause or the AS clause of a generated column.
///* Return NULL if the column has no associated expression.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_expr(p_tab: &Table, p_col: &Column)
    -> *mut Expr {
    unsafe {
        if (*p_col).i_dflt as i32 == 0 { return core::ptr::null_mut(); }
        if !((*p_tab).e_tab_type as i32 == 0) as i32 != 0 {
            return core::ptr::null_mut();
        }
        if (*p_tab).u.tab.p_dflt_list == core::ptr::null_mut() {
            return core::ptr::null_mut();
        }
        if unsafe { (*(*p_tab).u.tab.p_dflt_list).n_expr } <
                (*p_col).i_dflt as i32 {
            return core::ptr::null_mut();
        }
        return unsafe {
                (*(unsafe { (*(*p_tab).u.tab.p_dflt_list).a.as_ptr() } as
                                *mut ExprListItem).offset(((*p_col).i_dflt as i32 - 1) as
                                isize)).p_expr
            };
    }
}

///* Set the collating sequence name for a column.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_set_coll(db: *mut Sqlite3,
    p_col: &mut Column, z_coll: *const i8) -> () {
    let mut n_coll: i64 = 0 as i64;
    let mut n: i64 = 0 as i64;
    let mut z_new: *mut i8 = core::ptr::null_mut();
    { let _ = 0; };
    n =
        (unsafe { sqlite3_strlen30((*p_col).z_cn_name as *const i8) } + 1) as
            i64;
    if (*p_col).col_flags as i32 & 4 != 0 {
        n +=
            (unsafe {
                        sqlite3_strlen30(unsafe {
                                    (*p_col).z_cn_name.offset(n as isize)
                                } as *const i8)
                    } + 1) as i64;
    }
    n_coll = (unsafe { sqlite3_strlen30(z_coll) } + 1) as i64;
    z_new =
        unsafe {
                sqlite3_db_realloc(db, (*p_col).z_cn_name as *mut (),
                    (n_coll + n) as u64)
            } as *mut i8;
    if !(z_new).is_null() {
        (*p_col).z_cn_name = z_new;
        unsafe {
            memcpy(unsafe { (*p_col).z_cn_name.offset(n as isize) } as
                    *mut (), z_coll as *const (), n_coll as u64)
        };
        (*p_col).col_flags |= 512 as u16;
    }
}

///* Return the collating sequence name for a column
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_column_coll(p_col: &Column) -> *const i8 {
    let mut z: *const i8 = core::ptr::null();
    if (*p_col).col_flags as i32 & 512 == 0 { return core::ptr::null(); }
    z = (*p_col).z_cn_name as *const i8;
    while unsafe { *z } != 0 {
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if (*p_col).col_flags as i32 & 4 != 0 {
        '__b9: loop {
            '__c9: loop {
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
                break '__c9;
            }
            if !(unsafe { *z } != 0) { break '__b9; }
        }
    }
    return unsafe { z.offset(1 as isize) };
}

///* Delete memory allocated for the column names of a table or view (the
///* Table.aCol[] array).
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_delete_column_names(db: *mut Sqlite3,
    p_table: &mut Table) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut p_col: *const Column = core::ptr::null();
        { let _ = 0; };
        { let _ = 0; };
        if { p_col = (*p_table).a_col; p_col } != core::ptr::null_mut() {
            {
                i = 0;
                '__b10: loop {
                    if !(i < (*p_table).n_col as i32) { break '__b10; }
                    '__c10: loop {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_db_free(db,
                                unsafe { (*p_col).z_cn_name } as *mut ())
                        };
                        break '__c10;
                    }
                    {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        {
                            let __p = &mut p_col;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
            unsafe { sqlite3_db_nn_free_nn(db, (*p_table).a_col as *mut ()) };
            if (*p_table).e_tab_type as i32 == 0 {
                unsafe {
                    sqlite3_expr_list_delete(db, (*p_table).u.tab.p_dflt_list)
                };
            }
            if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut() {
                (*p_table).a_col = core::ptr::null_mut();
                (*p_table).n_col = 0 as i16;
                if (*p_table).e_tab_type as i32 == 0 {
                    (*p_table).u.tab.p_dflt_list = core::ptr::null_mut();
                }
            }
        }
    }
}

///* Record the fact that we want to lock a table at run-time. 
///*
///* The table to be locked has root page iTab and is found in database iDb.
///* A read or a write lock can be taken depending on isWritelock.
///*
///* This routine just records the fact that the lock is desired.  The
///* code to make the lock occur is generated by a later call to
///* codeTableLocks() which occurs during sqlite3FinishCoding().
extern "C" fn lock_table(p_parse_1: *mut Parse, i_db_1: i32, i_tab_1: Pgno,
    is_write_lock_1: u8, z_name_1: *const i8) -> () {
    let mut p_toplevel: *mut Parse = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut n_bytes: i32 = 0;
    let mut p: *mut TableLock = core::ptr::null_mut();
    { let _ = 0; };
    p_toplevel =
        if !(unsafe { (*p_parse_1).p_toplevel }).is_null() {
            unsafe { (*p_parse_1).p_toplevel }
        } else { p_parse_1 };
    {
        i = 0;
        '__b11: loop {
            if !(i < unsafe { (*p_toplevel).n_table_lock }) { break '__b11; }
            '__c11: loop {
                p =
                    unsafe {
                        unsafe { (*p_toplevel).a_table_lock.offset(i as isize) }
                    };
                if unsafe { (*p).i_db } == i_db_1 &&
                        unsafe { (*p).i_tab } == i_tab_1 {
                    unsafe {
                        (*p).is_write_lock =
                            (unsafe { (*p).is_write_lock } != 0 || is_write_lock_1 != 0)
                                as u8
                    };
                    return;
                }
                break '__c11;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
    n_bytes =
        (core::mem::size_of::<TableLock>() as u64 *
                (unsafe { (*p_toplevel).n_table_lock } + 1) as u64) as i32;
    if unsafe { (*p_toplevel).n_table_lock } == 0 {
        unsafe { (*p_toplevel).a_table_lock = core::ptr::null_mut() };
    }
    unsafe {
        (*p_toplevel).a_table_lock =
            unsafe {
                    sqlite3_db_realloc_or_free(unsafe { (*p_toplevel).db },
                        unsafe { (*p_toplevel).a_table_lock } as *mut (),
                        n_bytes as u64)
                } as *mut TableLock
    };
    if !(unsafe { (*p_toplevel).a_table_lock }).is_null() {
        p =
            unsafe {
                unsafe {
                    (*p_toplevel).a_table_lock.offset({
                                let __p = unsafe { &mut (*p_toplevel).n_table_lock };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize)
                }
            };
        unsafe { (*p).i_db = i_db_1 };
        unsafe { (*p).i_tab = i_tab_1 };
        unsafe { (*p).is_write_lock = is_write_lock_1 };
        unsafe { (*p).z_lock_name = z_name_1 };
    } else {
        unsafe { (*p_toplevel).n_table_lock = 0 };
        unsafe { sqlite3_oom_fault(unsafe { (*p_toplevel).db }) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_table_lock(p_parse: *mut Parse, i_db: i32,
    i_tab: Pgno, is_write_lock: u8, z_name: *const i8) -> () {
    if i_db == 1 { return; }
    if (unsafe {
                        sqlite3_btree_sharable(unsafe {
                                (*unsafe {
                                            (*unsafe { (*p_parse).db }).a_db.offset(i_db as isize)
                                        }).p_bt
                            })
                    } == 0) as i32 != 0 {
        return;
    }
    lock_table(p_parse, i_db, i_tab, is_write_lock, z_name);
}

///* Open the sqlite_schema table stored in database number iDb for
///* writing. The table is opened using cursor 0.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_open_schema_table(p: *mut Parse, i_db: i32) -> () {
    let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p) };
    sqlite3_table_lock(p, i_db, 1 as Pgno, 1 as u8,
        c"sqlite_master".as_ptr() as *mut i8 as *const i8);
    unsafe { sqlite3_vdbe_add_op4_int(v, 116, 0, 1, i_db, 5) };
    if unsafe { (*p).n_tab } == 0 { unsafe { (*p).n_tab = 1 }; }
}

///* Return the PRIMARY KEY index of a table
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_primary_key_index(p_tab: &Table) -> *mut Index {
    let mut p: *mut Index = core::ptr::null_mut();
    {
        p = (*p_tab).p_index;
        '__b12: loop {
            if !(!(p).is_null() &&
                            !(unsafe { (*p).idx_type() } as i32 == 2) as i32 != 0) {
                break '__b12;
            }
            '__c12: loop { break '__c12; }
            p = unsafe { (*p).p_next };
        }
    }
    return p;
}

///* Convert an table column number into a index column number.  That is,
///* for the column iCol in the table (as defined by the CREATE TABLE statement)
///* find the (first) offset of that column in index pIdx.  Or return -1
///* if column iCol is not used in index pIdx.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_table_column_to_index(p_idx: &Index, i_col: i32)
    -> i32 {
    let mut i: i32 = 0;
    let mut i_col16: i16 = 0 as i16;
    { let _ = 0; };
    { let _ = 0; };
    i_col16 = i_col as i16;
    {
        i = 0;
        '__b13: loop {
            if !(i < (*p_idx).n_column as i32) { break '__b13; }
            '__c13: loop {
                if i_col16 as i32 ==
                        unsafe { *(*p_idx).ai_column.offset(i as isize) } as i32 {
                    return i;
                }
                break '__c13;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return -1;
}

/// Convert a table column number into a storage column number.
///*
///* The storage column number (0,1,2,....) is the index of the value
///* as it appears in the record on disk.  Or, if the input column is
///* the N-th virtual column (zero-based) then the storage number is
///* the number of non-virtual columns in the table plus N. 
///*
///* The true column number is the index (0,1,2,...) of the column in
///* the CREATE TABLE statement.
///*
///* If the input column is a VIRTUAL column, then it should not appear
///* in storage.  But the value sometimes is cached in registers that
///* follow the range of registers used to construct storage.  This
///* avoids computing the same VIRTUAL column multiple times, and provides
///* values for use by OP_Param opcodes in triggers.  Hence, if the
///* input column is a VIRTUAL table, put it after all the other columns.
///*
///* In the following, N means "normal column", S means STORED, and
///* V means VIRTUAL.  Suppose the CREATE TABLE has columns like this:
///*
///*        CREATE TABLE ex(N,S,V,N,S,V,N,S,V);
///*                     -- 0 1 2 3 4 5 6 7 8
///*
///* Then the mapping from this function is as follows:
///*
///*    INPUTS:     0 1 2 3 4 5 6 7 8
///*    OUTPUTS:    0 1 6 2 3 7 4 5 8
///*
///* So, in other words, this routine shifts all the virtual columns to
///* the end.
///*
///* If SQLITE_OMIT_GENERATED_COLUMNS then there are no virtual columns and
///* this routine is a no-op macro.  If the pTab does not have any virtual
///* columns, then this routine is no-op that always return iCol.  If iCol
///* is negative (indicating the ROWID column) then this routine return iCol.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_table_column_to_storage(p_tab: &Table, i_col: i16)
    -> i16 {
    let mut i: i32 = 0;
    let mut n: i16 = 0 as i16;
    { let _ = 0; };
    if (*p_tab).tab_flags & 32 as u32 == 0 as u32 || (i_col as i32) < 0 {
        return i_col;
    }
    {
        { ({ i = 0; i }) as i16; n = 0 as i16 };
        '__b14: loop {
            if !(i < i_col as i32) { break '__b14; }
            '__c14: loop {
                if unsafe { (*(*p_tab).a_col.offset(i as isize)).col_flags }
                                as i32 & 32 == 0 {
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                }
                break '__c14;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if unsafe { (*(*p_tab).a_col.offset(i as isize)).col_flags } as i32 & 32
            != 0 {

        /// iCol is a virtual column itself
        return ((*p_tab).n_nv_col as i32 + i - n as i32) as i16;
    } else {

        /// iCol is a normal or stored column
        return n;
    }
}

/// Convert a storage column number into a table column number.
///*
///* The storage column number (0,1,2,....) is the index of the value
///* as it appears in the record on disk.  The true column number
///* is the index (0,1,2,...) of the column in the CREATE TABLE statement.
///*
///* The storage column number is less than the table column number if
///* and only there are VIRTUAL columns to the left.
///*
///* If SQLITE_OMIT_GENERATED_COLUMNS, this routine is a no-op macro.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_storage_column_to_table(p_tab: &Table,
    mut i_col: i16) -> i16 {
    if (*p_tab).tab_flags & 32 as u32 != 0 {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b15: loop {
                if !(i <= i_col as i32) { break '__b15; }
                '__c15: loop {
                    if unsafe { (*(*p_tab).a_col.offset(i as isize)).col_flags }
                                    as i32 & 32 != 0 {
                        { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c15;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return i_col;
}

///* Given a token, return a string that consists of the text of that
///* token.  Space to hold the returned string
///* is obtained from sqliteMalloc() and must be freed by the calling
///* function.
///*
///* Any quotation marks (ex:  "name", 'name', [name], or `name`) that
///* surround the body of the token are removed.
///*
///* Tokens are often just pointers into the original SQL text and so
///* are not \000 terminated and are not persistent.  The returned string
///* is \000 terminated and is persistent.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_name_from_token(db: *mut Sqlite3,
    p_name: *const Token) -> *mut i8 {
    unsafe {
        let mut z_name: *mut i8 = core::ptr::null_mut();
        if !(p_name).is_null() {
            z_name =
                unsafe {
                    sqlite3_db_str_n_dup(db,
                        unsafe { (*p_name).z } as *const i8,
                        unsafe { (*p_name).n } as u64)
                };
            unsafe { sqlite3_dequote(z_name) };
        } else { z_name = core::ptr::null_mut(); }
        return z_name;
    }
}

///* Parameter zName points to a nul-terminated buffer containing the name
///* of a database ("main", "temp" or the name of an attached db). This
///* function returns the index of the named database in db->aDb[], or
///* -1 if the named db cannot be found.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_find_db_name(db: &Sqlite3, z_name: *const i8)
    -> i32 {
    let mut i: i32 = -1;
    if !(z_name).is_null() {
        let mut p_db: *const Db = core::ptr::null();
        {
            {
                i = (*db).n_db - 1;
                p_db = unsafe { (*db).a_db.offset(i as isize) }
            };
            '__b16: loop {
                if !(i >= 0) { break '__b16; }
                '__c16: loop {
                    if 0 ==
                            unsafe {
                                sqlite3_stricmp(unsafe { (*p_db).z_db_s_name } as *const i8,
                                    z_name)
                            } {
                        break '__b16;
                    }
                    if i == 0 &&
                            0 ==
                                unsafe {
                                    sqlite3_stricmp(c"main".as_ptr() as *mut i8 as *const i8,
                                        z_name)
                                } {
                        break '__b16;
                    }
                    break '__c16;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                    {
                        let __p = &mut p_db;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(-1) };
                        __t
                    }
                };
            }
        }
    }
    return i;
}

///* The token *pName contains the name of a database (either "main" or
///* "temp" or the name of an attached db). This routine returns the
///* index of the named database in db->aDb[], or -1 if the named db
///* does not exist.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_find_db(db: *mut Sqlite3, p_name: *mut Token)
    -> i32 {
    let mut i: i32 = 0;
    /// Database number
    let mut z_name: *mut i8 = core::ptr::null_mut();

    /// Name we are searching for
    (z_name = sqlite3_name_from_token(db, p_name as *const Token));
    i = sqlite3_find_db_name(unsafe { &*db }, z_name as *const i8);
    unsafe { sqlite3_db_free(db, z_name as *mut ()) };
    return i;
}

/// The table or view or trigger name is passed to this routine via tokens
///* pName1 and pName2. If the table name was fully qualified, for example:
///*
///* CREATE TABLE xxx.yyy (...);
///*
///* Then pName1 is set to "xxx" and pName2 "yyy". On the other hand if
///* the table name is not fully qualified, i.e.:
///*
///* CREATE TABLE yyy(...);
///*
///* Then pName1 is set to "yyy" and pName2 is "".
///*
///* This routine sets the *ppUnqual pointer to point at the token (pName1 or
///* pName2) that stores the unqualified table name.  The index of the
///* database "xxx" is returned.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_two_part_name(p_parse: *mut Parse,
    p_name1: *mut Token, p_name2: *mut Token, p_unqual: &mut *mut Token)
    -> i32 {
    unsafe {
        let mut i_db: i32 = 0;
        /// Database holding the object
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        { let _ = 0; };
        if unsafe { (*p_name2).n } > 0 as u32 {
            if unsafe { (*db).init.busy } != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"corrupt database".as_ptr() as *mut i8 as *const i8)
                };
                return -1;
            }
            *p_unqual = p_name2;
            i_db = sqlite3_find_db(db, p_name1);
            if i_db < 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"unknown database %T".as_ptr() as *mut i8 as *const i8,
                        p_name1)
                };
                return -1;
            }
        } else {
            { let _ = 0; };
            i_db = unsafe { (*db).init.i_db } as i32;
            *p_unqual = p_name1;
        }
        return i_db;
    }
}

///* True if PRAGMA writable_schema is ON
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_writable_schema(db: &Sqlite3) -> i32 {
    return ((*db).flags & (1 | 268435456) as u64 == 1 as u64) as i32;
}

///* Return TRUE if shadow tables should be read-only in the current
///* context.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_read_only_shadow_tables(db: &Sqlite3) -> i32 {
    if (*db).flags & 268435456 as u64 != 0 as u64 &&
                    (*db).p_vtab_ctx == core::ptr::null_mut() &&
                (*db).n_vdbe_exec == 0 &&
            !((*db).n_v_trans > 0 && (*db).a_v_trans == core::ptr::null_mut())
                    as i32 != 0 {
        return 1;
    }
    return 0;
}

///* Locate the in-memory structure that describes a particular database
///* table given the name of that table and (optionally) the name of the
///* database containing the table.  Return NULL if not found.
///*
///* If zDatabase is 0, all databases are searched for the table and the
///* first matching table is returned.  (No checking for duplicate table
///* names is done.)  The search order is TEMP first, then MAIN, then any
///* auxiliary databases added using the ATTACH command.
///*
///* See also sqlite3LocateTable().
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_find_table(db: &Sqlite3, z_name: *const i8,
    z_database: *const i8) -> *mut Table {
    unsafe {
        let mut p: *mut Table = core::ptr::null_mut();
        let mut i: i32 = 0;

        /// All mutexes are required for schema access.  Make sure we hold them.
        { let _ = 0; };
        if !(z_database).is_null() {
            {
                i = 0;
                '__b17: loop {
                    if !(i < (*db).n_db) { break '__b17; }
                    '__c17: loop {
                        if unsafe {
                                    sqlite3_str_i_cmp(z_database,
                                        unsafe { (*(*db).a_db.offset(i as isize)).z_db_s_name } as
                                            *const i8)
                                } == 0 {
                            break '__b17;
                        }
                        break '__c17;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if i >= (*db).n_db {
                if unsafe {
                            sqlite3_str_i_cmp(z_database,
                                c"main".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    i = 0;
                } else { return core::ptr::null_mut(); }
            }
            p =
                unsafe {
                        sqlite3_hash_find(unsafe {
                                    &raw mut (*unsafe {
                                                        (*(*db).a_db.offset(i as isize)).p_schema
                                                    }).tbl_hash
                                } as *const Hash, z_name)
                    } as *mut Table;
            if p == core::ptr::null_mut() &&
                    unsafe {
                            sqlite3_strnicmp(z_name,
                                c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
                        } == 0 {
                if i == 1 {
                    if unsafe {
                                        sqlite3_str_i_cmp(unsafe { z_name.offset(7 as isize) },
                                            unsafe {
                                                    &raw mut *(c"sqlite_temp_schema".as_ptr() as
                                                                    *mut i8).offset(7 as isize)
                                                } as *const i8)
                                    } == 0 ||
                                unsafe {
                                        sqlite3_str_i_cmp(unsafe { z_name.offset(7 as isize) },
                                            unsafe {
                                                    &raw mut *(c"sqlite_schema".as_ptr() as
                                                                    *mut i8).offset(7 as isize)
                                                } as *const i8)
                                    } == 0 ||
                            unsafe {
                                    sqlite3_str_i_cmp(unsafe { z_name.offset(7 as isize) },
                                        unsafe {
                                                &raw mut *(c"sqlite_master".as_ptr() as
                                                                *mut i8).offset(7 as isize)
                                            } as *const i8)
                                } == 0 {
                        p =
                            unsafe {
                                    sqlite3_hash_find(unsafe {
                                                &raw mut (*unsafe {
                                                                    (*(*db).a_db.offset(1 as isize)).p_schema
                                                                }).tbl_hash
                                            } as *const Hash,
                                        c"sqlite_temp_master".as_ptr() as *mut i8 as *const i8)
                                } as *mut Table;
                    }
                } else {
                    if unsafe {
                                sqlite3_str_i_cmp(unsafe { z_name.offset(7 as isize) },
                                    unsafe {
                                            &raw mut *(c"sqlite_schema".as_ptr() as
                                                            *mut i8).offset(7 as isize)
                                        } as *const i8)
                            } == 0 {
                        p =
                            unsafe {
                                    sqlite3_hash_find(unsafe {
                                                &raw mut (*unsafe {
                                                                    (*(*db).a_db.offset(i as isize)).p_schema
                                                                }).tbl_hash
                                            } as *const Hash,
                                        c"sqlite_master".as_ptr() as *mut i8 as *const i8)
                                } as *mut Table;
                    }
                }
            }
        } else {

            /// Match against TEMP first
            (p =
                unsafe {
                        sqlite3_hash_find(unsafe {
                                    &raw mut (*unsafe {
                                                        (*(*db).a_db.offset(1 as isize)).p_schema
                                                    }).tbl_hash
                                } as *const Hash, z_name)
                    } as *mut Table);
            if !(p).is_null() { return p; }

            /// The main database is second
            (p =
                unsafe {
                        sqlite3_hash_find(unsafe {
                                    &raw mut (*unsafe {
                                                        (*(*db).a_db.offset(0 as isize)).p_schema
                                                    }).tbl_hash
                                } as *const Hash, z_name)
                    } as *mut Table);
            if !(p).is_null() { return p; }
            {
                i = 2;
                '__b18: loop {
                    if !(i < (*db).n_db) { break '__b18; }
                    '__c18: loop {
                        { let _ = 0; };
                        p =
                            unsafe {
                                    sqlite3_hash_find(unsafe {
                                                &raw mut (*unsafe {
                                                                    (*(*db).a_db.offset(i as isize)).p_schema
                                                                }).tbl_hash
                                            } as *const Hash, z_name)
                                } as *mut Table;
                        if !(p).is_null() { break '__b18; }
                        break '__c18;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if p == core::ptr::null_mut() &&
                    unsafe {
                            sqlite3_strnicmp(z_name,
                                c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
                        } == 0 {
                if unsafe {
                            sqlite3_str_i_cmp(unsafe { z_name.offset(7 as isize) },
                                unsafe {
                                        &raw mut *(c"sqlite_schema".as_ptr() as
                                                        *mut i8).offset(7 as isize)
                                    } as *const i8)
                        } == 0 {
                    p =
                        unsafe {
                                sqlite3_hash_find(unsafe {
                                            &raw mut (*unsafe {
                                                                (*(*db).a_db.offset(0 as isize)).p_schema
                                                            }).tbl_hash
                                        } as *const Hash,
                                    c"sqlite_master".as_ptr() as *mut i8 as *const i8)
                            } as *mut Table;
                } else if unsafe {
                            sqlite3_str_i_cmp(unsafe { z_name.offset(7 as isize) },
                                unsafe {
                                        &raw mut *(c"sqlite_temp_schema".as_ptr() as
                                                        *mut i8).offset(7 as isize)
                                    } as *const i8)
                        } == 0 {
                    p =
                        unsafe {
                                sqlite3_hash_find(unsafe {
                                            &raw mut (*unsafe {
                                                                (*(*db).a_db.offset(1 as isize)).p_schema
                                                            }).tbl_hash
                                        } as *const Hash,
                                    c"sqlite_temp_master".as_ptr() as *mut i8 as *const i8)
                            } as *mut Table;
                }
            }
        }
        return p;
    }
}

///* Return true if pTab is a virtual table and zName is a shadow table name
///* for that virtual table.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_is_shadow_table_of(db: &mut Sqlite3, p_tab: &Table,
    z_name: *const i8) -> i32 {
    unsafe {
        let mut n_name: i32 = 0;
        /// Length of zName
        let mut p_mod: *const Module = core::ptr::null();
        if !((*p_tab).e_tab_type as i32 == 1) as i32 != 0 { return 0; }
        n_name = unsafe { sqlite3_strlen30((*p_tab).z_name as *const i8) };
        if unsafe {
                    sqlite3_strnicmp(z_name, (*p_tab).z_name as *const i8,
                        n_name)
                } != 0 {
            return 0;
        }
        if unsafe { *z_name.offset(n_name as isize) } as i32 != '_' as i32 {
            return 0;
        }
        p_mod =
            unsafe {
                    sqlite3_hash_find(&raw mut (*db).a_module as *const Hash,
                        unsafe { *(*p_tab).u.vtab.az_arg.offset(0 as isize) } as
                            *const i8)
                } as *mut Module;
        if p_mod == core::ptr::null_mut() { return 0; }
        if (unsafe { (*unsafe { (*p_mod).p_module }).i_version } as i32) < 3 {
            return 0;
        }
        if !unsafe { (*unsafe { (*p_mod).p_module }).x_shadow_name.is_some() }
                    as i32 != 0 {
            return 0;
        }
        return unsafe {
                (unsafe {
                        (*unsafe { (*p_mod).p_module }).x_shadow_name.unwrap()
                    })(unsafe {
                        unsafe { z_name.offset(n_name as isize).offset(1 as isize) }
                    })
            };
    }
}

///* Return true if zName is a shadow table name in the current database
///* connection.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_shadow_table_name(db: *mut Sqlite3,
    z_name: *const i8) -> i32 {
    let mut z_tail: *const i8 = core::ptr::null();
    /// Pointer to the last "_" in zName
    let mut p_tab: *mut Table = core::ptr::null_mut();
    /// Table that zName is a shadow of
    let mut z_copy: *mut i8 = core::ptr::null_mut();

    /// Transient copy of zName after last "_"
    (z_tail = unsafe { strrchr(z_name, '_' as i32) } as *const i8);
    if z_tail == core::ptr::null() { return 0; }
    z_copy =
        unsafe {
            sqlite3_db_str_n_dup(db, z_name,
                unsafe { z_tail.offset_from(z_name) } as i64 as i32 as u64)
        };
    p_tab =
        if !(z_copy).is_null() {
            sqlite3_find_table(unsafe { &*db }, z_copy as *const i8,
                core::ptr::null())
        } else { core::ptr::null_mut() };
    unsafe { sqlite3_db_free(db, z_copy as *mut ()) };
    if p_tab == core::ptr::null_mut() { return 0; }
    if !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0 { return 0; }
    return sqlite3_is_shadow_table_of(unsafe { &mut *db }, unsafe { &*p_tab },
            z_name);
}

///* This routine is used to check if the UTF-8 string zName is a legal
///* unqualified name for a new schema object (table, index, view or
///* trigger). All names are legal except those that begin with the string
///* "sqlite_" (in upper, lower or mixed case). This portion of the namespace
///* is reserved for internal use.
///*
///* When parsing the sqlite_schema table, this routine also checks to
///* make sure the "type", "name", and "tbl_name" columns are consistent
///* with the SQL.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_check_object_name(p_parse: *mut Parse,
    z_name: *const i8, z_type: *const i8, z_tbl_name: *const i8) -> i32 {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        if sqlite3_writable_schema(unsafe { &*db }) != 0 ||
                    unsafe { (*db).init.imposter_table() } != 0 ||
                (sqlite3Config.b_extra_schema_checks == 0) as i32 != 0 {

            /// Skip these error checks for writable_schema=ON
            return 0;
        }
        if unsafe { (*db).init.busy } != 0 {
            if unsafe {
                                sqlite3_stricmp(z_type,
                                    unsafe {
                                        *unsafe { (*db).init.az_init.offset(0 as isize) }
                                    })
                            } != 0 ||
                        unsafe {
                                sqlite3_stricmp(z_name,
                                    unsafe {
                                        *unsafe { (*db).init.az_init.offset(1 as isize) }
                                    })
                            } != 0 ||
                    unsafe {
                            sqlite3_stricmp(z_tbl_name,
                                unsafe {
                                    *unsafe { (*db).init.az_init.offset(2 as isize) }
                                })
                        } != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"".as_ptr() as *mut i8 as *const i8)
                };

                /// corruptSchema() will supply the error
                return 1;
            }
        } else {
            if unsafe { (*p_parse).nested } as i32 == 0 &&
                        0 ==
                            unsafe {
                                sqlite3_strnicmp(z_name,
                                    c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
                            } ||
                    sqlite3_read_only_shadow_tables(unsafe { &*db }) != 0 &&
                        sqlite3_shadow_table_name(db, z_name) != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"object name reserved for internal use: %s".as_ptr() as
                                *mut i8 as *const i8, z_name)
                };
                return 1;
            }
        }
        return 0;
    }
}

///* Make sure the TEMP database is open and available for use.  Return
///* the number of errors.  Leave any error messages in the pParse structure.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_open_temp_database(p_parse: *mut Parse) -> i32 {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    if unsafe { (*unsafe { (*db).a_db.offset(1 as isize) }).p_bt } ==
                core::ptr::null_mut() &&
            (unsafe { (*p_parse).explain } == 0) as i32 != 0 {
        let mut rc: i32 = 0;
        let mut p_bt: *mut Btree = core::ptr::null_mut();
        rc =
            unsafe {
                sqlite3_btree_open(unsafe { (*db).p_vfs }, core::ptr::null(),
                    db, &mut p_bt, 0, flags_1)
            };
        if rc != 0 {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"unable to open a temporary database file for storing temporary tables".as_ptr()
                            as *mut i8 as *const i8)
            };
            unsafe { (*p_parse).rc = rc };
            return 1;
        }
        unsafe { (*unsafe { (*db).a_db.offset(1 as isize) }).p_bt = p_bt };
        { let _ = 0; };
        if 7 ==
                unsafe {
                    sqlite3_btree_set_page_size(p_bt,
                        unsafe { (*db).next_pagesize }, 0, 0)
                } {
            unsafe { sqlite3_oom_fault(db) };
            return 1;
        }
    }
    return 0;
}

///* Record the fact that the schema cookie will need to be verified
///* for database iDb.  The code to actually verify the schema cookie
///* will occur at the end of the top-level VDBE and will be generated
///* later, by sqlite3FinishCoding().
extern "C" fn sqlite3_code_verify_schema_at_toplevel(p_toplevel_1: *mut Parse,
    i_db_1: i32) -> () {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if (unsafe { (*p_toplevel_1).cookie_mask } & (1 as YDbMask) << i_db_1 !=
                    0 as u32) as i32 == 0 {
        unsafe { (*p_toplevel_1).cookie_mask |= (1 as YDbMask) << i_db_1 };
        if (0 == 0) as i32 != 0 && i_db_1 == 1 {
            sqlite3_open_temp_database(p_toplevel_1);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_code_verify_schema(p_parse: *mut Parse, i_db: i32)
    -> () {
    sqlite3_code_verify_schema_at_toplevel(if !(unsafe {
                            (*p_parse).p_toplevel
                        }).is_null() {
            unsafe { (*p_parse).p_toplevel }
        } else { p_parse }, i_db);
}

///* Insert a single OP_JournalMode query opcode in order to force the
///* prepared statement to return false for sqlite3_stmt_readonly().  This
///* is used by CREATE TABLE IF NOT EXISTS and similar if the table already
///* exists, so that the prepared statement for CREATE TABLE IF NOT EXISTS
///* will return false for sqlite3_stmt_readonly() even if that statement
///* is a read-only no-op.
extern "C" fn sqlite3_force_not_read_only(p_parse_1: *mut Parse) -> () {
    let i_reg: i32 =
        { let __p = unsafe { &mut (*p_parse_1).n_mem }; *__p += 1; *__p };
    let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse_1) };
    if !(v).is_null() {
        unsafe { sqlite3_vdbe_add_op3(v, 4, 0, i_reg, -1) };
        unsafe { sqlite3_vdbe_uses_btree(v, 0) };
    }
}

///* Locate the in-memory structure that describes
///* a particular index given the name of that index
///* and the name of the database that contains the index.
///* Return NULL if not found.
///*
///* If zDatabase is 0, all databases are searched for the
///* table and the first matching index is returned.  (No checking
///* for duplicate index names is done.)  The search order is
///* TEMP first, then MAIN, then any auxiliary databases added
///* using the ATTACH command.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_find_index(db: *mut Sqlite3, z_name: *const i8,
    z_db: *const i8) -> *mut Index {
    unsafe {
        let mut p: *mut Index = core::ptr::null_mut();
        let mut i: i32 = 0;

        /// All mutexes are required for schema access.  Make sure we hold them.
        { let _ = 0; };
        {
            i = 0;
            '__b19: loop {
                if !(i < unsafe { (*db).n_db }) { break '__b19; }
                '__c19: loop {
                    let j: i32 = if i < 2 { i ^ 1 } else { i };
                    /// Search TEMP before MAIN
                    let p_schema: *mut Schema =
                        unsafe {
                            (*unsafe { (*db).a_db.offset(j as isize) }).p_schema
                        };
                    { let _ = 0; };
                    if !(z_db).is_null() &&
                            unsafe { sqlite3_db_is_named(db, j, z_db) } == 0 {
                        break '__c19;
                    }
                    { let _ = 0; };
                    p =
                        unsafe {
                                sqlite3_hash_find(unsafe { &raw mut (*p_schema).idx_hash }
                                        as *const Hash, z_name)
                            } as *mut Index;
                    if !(p).is_null() { break '__b19; }
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return p;
    }
}

///* Generate VDBE code that prepares for doing an operation that
///* might change the database.
///*
///* This routine starts a new transaction if we are not already within
///* a transaction.  If we are already within a transaction, then a checkpoint
///* is set if the setStatement parameter is true.  A checkpoint should
///* be set for operations that might fail (due to a constraint) part of
///* the way through and which will need to undo some writes without having to
///* rollback the whole transaction.  For operations where all constraints
///* can be checked before any changes are made to the database, it is never
///* necessary to undo a write and the checkpoint should not be set.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_begin_write_operation(p_parse: *mut Parse,
    set_statement: i32, i_db: i32) -> () {
    let p_toplevel: *mut Parse =
        if !(unsafe { (*p_parse).p_toplevel }).is_null() {
            unsafe { (*p_parse).p_toplevel }
        } else { p_parse };
    sqlite3_code_verify_schema_at_toplevel(p_toplevel, i_db);
    unsafe { (*p_toplevel).write_mask |= (1 as YDbMask) << i_db };
    unsafe { (*p_toplevel).is_multi_write |= set_statement as u8 };
}

///* Begin constructing a new table representation in memory.  This is
///* the first of several action routines that get called in response
///* to a CREATE TABLE statement.  In particular, this routine is called
///* after seeing tokens "CREATE" and "TABLE" and the table name. The isTemp
///* flag is true if the table should be stored in the auxiliary database
///* file instead of in the main database file.  This is normally the case
///* when the "TEMP" or "TEMPORARY" keyword occurs in between
///* CREATE and TABLE.
///*
///* The new table record is initialized and put in pParse->pNewTable.
///* As more of the CREATE TABLE statement is parsed, additional action
///* routines will be called to add more information to this record.
///* At the end of the CREATE TABLE statement, the sqlite3EndTable() routine
///* is called to complete the construction of the new table record.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_start_table(p_parse: *mut Parse,
    p_name1: *mut Token, p_name2: *mut Token, mut is_temp: i32, is_view: i32,
    is_virtual: i32, no_err: i32) -> () {
    unsafe {
        let mut z_name: *mut i8 = core::ptr::null_mut();
        /// The name of the new table
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        '__b20: loop {
            '__c20: loop {
                let mut p_table: *mut Table = core::ptr::null_mut();
                /// The name of the new table
                let mut v: *mut Vdbe = core::ptr::null_mut();
                let mut i_db: i32 = 0;
                /// Database number to create the table in
                let mut p_name: *mut Token = core::ptr::null_mut();
                if unsafe { (*db).init.busy } != 0 &&
                        unsafe { (*db).init.new_tnum } == 1 as u32 {

                    /// Special case:  Parsing the sqlite_schema or sqlite_temp_schema schema
                    (i_db = unsafe { (*db).init.i_db } as i32);
                    z_name =
                        unsafe {
                            sqlite3_db_str_dup(db,
                                if (0 == 0) as i32 != 0 && i_db == 1 {
                                        c"sqlite_temp_master".as_ptr() as *mut i8
                                    } else { c"sqlite_master".as_ptr() as *mut i8 } as
                                    *const i8)
                        };
                    p_name = p_name1;
                } else {

                    /// The common case
                    (i_db =
                        sqlite3_two_part_name(p_parse, p_name1, p_name2,
                            &mut p_name));
                    if i_db < 0 { return; }
                    if (0 == 0) as i32 != 0 && is_temp != 0 &&
                                unsafe { (*p_name2).n } > 0 as u32 && i_db != 1 {

                        /// If creating a temp table, the name may not be qualified. Unless
                        ///* the database name is "temp" anyway.
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"temporary table name must be unqualified".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        return;
                    }
                    if (0 == 0) as i32 != 0 && is_temp != 0 { i_db = 1; }
                    z_name =
                        sqlite3_name_from_token(db, p_name as *const Token);
                    if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                        unsafe {
                            sqlite3_rename_token_map(p_parse,
                                z_name as *mut () as *const (), p_name as *const Token)
                        };
                    }
                }
                unsafe {
                    (*p_parse).s_name_token = unsafe { core::ptr::read(p_name) }
                };
                if z_name == core::ptr::null_mut() { return; }
                if sqlite3_check_object_name(p_parse, z_name as *const i8,
                            if is_view != 0 {
                                    c"view".as_ptr() as *mut i8
                                } else { c"table".as_ptr() as *mut i8 } as *const i8,
                            z_name as *const i8) != 0 {
                    break '__b20;
                }
                if unsafe { (*db).init.i_db } as i32 == 1 { is_temp = 1; }
                { let _ = 0; };
                { let _ = 0; };
                {
                    let z_db: *const i8 =
                        unsafe {
                                (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                            } as *const i8;
                    if unsafe {
                                sqlite3_auth_check(p_parse, 18,
                                    if (0 == 0) as i32 != 0 && is_temp == 1 {
                                            c"sqlite_temp_master".as_ptr() as *mut i8
                                        } else { c"sqlite_master".as_ptr() as *mut i8 } as
                                        *const i8, core::ptr::null(), z_db as *const i8)
                            } != 0 {
                        break '__b20;
                    }
                    if (is_virtual == 0) as i32 != 0 &&
                            unsafe {
                                    sqlite3_auth_check(p_parse,
                                        a_code[(is_temp + 2 * is_view) as usize] as i32,
                                        z_name as *const i8, core::ptr::null(), z_db as *const i8)
                                } != 0 {
                        break '__b20;
                    }
                }
                if !(unsafe { (*p_parse).e_parse_mode } as i32 != 0) as i32 !=
                        0 {
                    let z_db_1: *const i8 =
                        unsafe {
                                (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                            } as *const i8;
                    if 0 != unsafe { sqlite3_read_schema(p_parse) } {
                        break '__b20;
                    }
                    p_table =
                        sqlite3_find_table(unsafe { &*db }, z_name as *const i8,
                            z_db_1 as *const i8);
                    if !(p_table).is_null() {
                        if (no_err == 0) as i32 != 0 {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"%s %T already exists".as_ptr() as *mut i8 as *const i8,
                                    if unsafe { (*p_table).e_tab_type } as i32 == 2 {
                                        c"view".as_ptr() as *mut i8
                                    } else { c"table".as_ptr() as *mut i8 }, p_name)
                            };
                        } else {
                            { let _ = 0; };
                            sqlite3_code_verify_schema(p_parse, i_db);
                            sqlite3_force_not_read_only(p_parse);
                        }
                        break '__b20;
                    }
                    if sqlite3_find_index(db, z_name as *const i8,
                                z_db_1 as *const i8) != core::ptr::null_mut() {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"there is already an index named %s".as_ptr() as *mut i8 as
                                    *const i8, z_name)
                        };
                        break '__b20;
                    }
                }
                p_table =
                    unsafe {
                            sqlite3_db_malloc_zero(db,
                                core::mem::size_of::<Table>() as u64)
                        } as *mut Table;
                if p_table == core::ptr::null_mut() {
                    { let _ = 0; };
                    unsafe { (*p_parse).rc = 7 };
                    {
                        let __p = unsafe { &mut (*p_parse).n_err };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    break '__b20;
                }
                unsafe { (*p_table).z_name = z_name };
                unsafe { (*p_table).i_p_key = -1 as i16 };
                unsafe {
                    (*p_table).p_schema =
                        unsafe {
                            (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                        }
                };
                unsafe { (*p_table).n_tab_ref = 1 as u32 };
                unsafe { (*p_table).n_row_log_est = 200 as LogEst };
                { let _ = 0; };
                { let _ = 0; };
                unsafe { (*p_parse).p_new_table = p_table };
                if (unsafe { (*db).init.busy } == 0) as i32 != 0 &&
                        { v = unsafe { sqlite3_get_vdbe(p_parse) }; v } !=
                            core::ptr::null_mut() {
                    let mut addr1: i32 = 0;
                    let mut file_format: i32 = 0;
                    let mut reg1: i32 = 0;
                    let mut reg2: i32 = 0;
                    let mut reg3: i32 = 0;
                    sqlite3_begin_write_operation(p_parse, 1, i_db);
                    if is_virtual != 0 {
                        unsafe { sqlite3_vdbe_add_op0(v, 172) };
                    }

                    /// If the file format and encoding in the database have not been set,
                    ///* set them now.
                    { let _ = 0; };
                    reg1 =
                        {
                            unsafe {
                                (*p_parse).u1.cr.reg_rowid =
                                    {
                                        let __p = unsafe { &mut (*p_parse).n_mem };
                                        *__p += 1;
                                        *__p
                                    }
                            };
                            unsafe { (*p_parse).u1.cr.reg_rowid }
                        };
                    reg2 =
                        {
                            unsafe {
                                (*p_parse).u1.cr.reg_root =
                                    {
                                        let __p = unsafe { &mut (*p_parse).n_mem };
                                        *__p += 1;
                                        *__p
                                    }
                            };
                            unsafe { (*p_parse).u1.cr.reg_root }
                        };
                    reg3 =
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            *__p += 1;
                            *__p
                        };
                    unsafe { sqlite3_vdbe_add_op3(v, 101, i_db, reg3, 2) };
                    unsafe { sqlite3_vdbe_uses_btree(v, i_db) };
                    addr1 = unsafe { sqlite3_vdbe_add_op1(v, 16, reg3) };
                    file_format =
                        if unsafe { (*db).flags } & 2 as u64 != 0 as u64 {
                            1
                        } else { 4 };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 102, i_db, 2, file_format)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 102, i_db, 5,
                            unsafe { (*db).enc } as i32)
                    };
                    unsafe { sqlite3_vdbe_jump_here(v, addr1) };
                    if is_view != 0 || is_virtual != 0 {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg2) };
                    } else {
                        { let _ = 0; };
                        unsafe {
                            (*p_parse).u1.cr.addr_cr_tab =
                                unsafe { sqlite3_vdbe_add_op3(v, 149, i_db, reg2, 1) }
                        };
                    }
                    sqlite3_open_schema_table(p_parse, i_db);
                    unsafe { sqlite3_vdbe_add_op2(v, 129, 0, reg1) };
                    unsafe {
                        sqlite3_vdbe_add_op4(v, 79, 6, reg3, 0,
                            &raw const null_row[0 as usize] as *const i8, -1)
                    };
                    unsafe { sqlite3_vdbe_add_op3(v, 130, 0, reg3, reg1) };
                    unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                    unsafe { sqlite3_vdbe_add_op0(v, 124) };
                } else if unsafe { (*db).init.imposter_table() } != 0 {
                    unsafe { (*p_table).tab_flags |= 131072 as u32 };
                    if unsafe { (*db).init.imposter_table() } as i32 >= 2 {
                        unsafe { (*p_table).tab_flags |= 1 as u32 };
                    }
                }

                /// Normal (non-error) return.
                return;
                break '__c20;
            }
            if !(false) { break '__b20; }
        }

        /// The name of the new table
        /// Database number to create the table in
        /// Unqualified name of the table to create
        /// Special case:  Parsing the sqlite_schema or sqlite_temp_schema schema
        /// The common case
        /// If creating a temp table, the name may not be qualified. Unless
        ///* the database name is "temp" anyway.
        /// Make sure the new table name does not collide with an existing
        ///* index or table name in the same database.  Issue an error message if
        ///* it does. The exception is if the statement being parsed was passed
        ///* to an sqlite3_declare_vtab() call. In that case only the column names
        ///* and types will be used, so there is no need to test for namespace
        ///* collisions.
        /// Begin generating the code that will insert the table record into
        ///* the schema table.  Note in particular that we must go ahead
        ///* and allocate the record number for the table entry now.  Before any
        ///* PRIMARY KEY or UNIQUE keywords are parsed.  Those keywords will cause
        ///* indices to be created and the table record must come before the
        ///* indices.  Hence, the record number for the table must be allocated
        ///* now.
        /// nullRow[] is an OP_Record encoding of a row containing 5 NULLs
        /// If the file format and encoding in the database have not been set,
        ///* set them now.
        /// This just creates a place-holder record in the sqlite_schema table.
        ///* The record created does not contain anything yet.  It will be replaced
        ///* by the real entry in code generated at sqlite3EndTable().
        ///*
        ///* The rowid for the new entry is left in register pParse->u1.cr.regRowid.
        ///* The root page of the new table is left in reg pParse->u1.cr.regRoot.
        ///* The rowid and root page number values are needed by the code that
        ///* sqlite3EndTable will generate.
        /// Normal (non-error) return.
        /// If an error occurs, we jump here
        unsafe { (*p_parse).set_check_schema(1 as Bft as u32) };
        unsafe { sqlite3_db_free(db, z_name as *mut ()) };
        return;
    }
}

///* Scan the column type name zType (length nType) and return the
///* associated affinity type.
///*
///* This routine does a case-independent search of zType for the
///* substrings in the following table. If one of the substrings is
///* found, the corresponding affinity is returned. If zType contains
///* more than one of the substrings, entries toward the top of
///* the table take priority. For example, if zType is 'BLOBINT',
///* SQLITE_AFF_INTEGER is returned.
///*
///* Substring     | Affinity
///* --------------------------------
///* 'INT'         | SQLITE_AFF_INTEGER
///* 'CHAR'        | SQLITE_AFF_TEXT
///* 'CLOB'        | SQLITE_AFF_TEXT
///* 'TEXT'        | SQLITE_AFF_TEXT
///* 'BLOB'        | SQLITE_AFF_BLOB
///* 'REAL'        | SQLITE_AFF_REAL
///* 'FLOA'        | SQLITE_AFF_REAL
///* 'DOUB'        | SQLITE_AFF_REAL
///*
///* If none of the substrings in the above table are found,
///* SQLITE_AFF_NUMERIC is returned.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_affinity_type(mut z_in: *const i8,
    p_col: *mut Column) -> i8 {
    unsafe {
        let mut h: u32 = 0 as u32;
        let mut aff: i8 = 67 as i8;
        let mut z_char: *const i8 = core::ptr::null();
        { let _ = 0; };
        while unsafe { *z_in.offset(0 as isize) } != 0 {
            let x: u8 = unsafe { *(z_in as *mut u8) };
            h =
                (h << 8) +
                    unsafe {
                            *(sqlite3_upper_to_lower.as_ptr() as
                                        *const u8).add(x as usize)
                        } as u32;
            {
                let __p = &mut z_in;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            if h ==
                    ((('c' as i32) << 24) + (('h' as i32) << 16) +
                                (('a' as i32) << 8) + 'r' as i32) as u32 {

                /// CHAR
                (aff = 66 as i8);
                z_char = z_in;
            } else if h ==
                    ((('c' as i32) << 24) + (('l' as i32) << 16) +
                                (('o' as i32) << 8) + 'b' as i32) as u32 {

                /// CLOB
                (aff = 66 as i8);
            } else if h ==
                    ((('t' as i32) << 24) + (('e' as i32) << 16) +
                                (('x' as i32) << 8) + 't' as i32) as u32 {

                /// TEXT
                (aff = 66 as i8);
            } else if h ==
                        ((('b' as i32) << 24) + (('l' as i32) << 16) +
                                    (('o' as i32) << 8) + 'b' as i32) as u32 &&
                    (aff as i32 == 67 || aff as i32 == 69) {
                aff = 65 as i8;
                if unsafe { *z_in.offset(0 as isize) } as i32 == '(' as i32 {
                    z_char = z_in;
                }
            } else if h ==
                        ((('r' as i32) << 24) + (('e' as i32) << 16) +
                                    (('a' as i32) << 8) + 'l' as i32) as u32 && aff as i32 == 67
                {
                aff = 69 as i8;
            } else if h ==
                        ((('f' as i32) << 24) + (('l' as i32) << 16) +
                                    (('o' as i32) << 8) + 'a' as i32) as u32 && aff as i32 == 67
                {
                aff = 69 as i8;
            } else if h ==
                        ((('d' as i32) << 24) + (('o' as i32) << 16) +
                                    (('u' as i32) << 8) + 'b' as i32) as u32 && aff as i32 == 67
                {
                aff = 69 as i8;
            } else if h & 16777215 as u32 ==
                    ((('i' as i32) << 16) + (('n' as i32) << 8) + 't' as i32) as
                        u32 {

                /// INT
                (aff = 68 as i8);
                break;
            }
        }
        if !(p_col).is_null() {
            let mut v: i32 = 0;
            if (aff as i32) < 67 {
                if !(z_char).is_null() {
                    while unsafe { *z_char.offset(0 as isize) } != 0 {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z_char.offset(0 as isize) } as u8
                                                        as usize)
                                        } as i32 & 4 != 0 {

                            /// BLOB(k), VARCHAR(k), CHAR(k) -> r=(k/4+1)
                            unsafe { sqlite3_get_int32(z_char, &mut v) };
                            break;
                        }
                        {
                            let __p = &mut z_char;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                    }
                } else { v = 16; }
            }
            v = v / 4 + 1;
            if v > 255 { v = 255; }
            unsafe { (*p_col).sz_est = v as u8 };
        }
        return aff;
    }
}

///* Add a new column to the table currently being constructed.
///*
///* The parser calls this routine once for each column declaration
///* in a CREATE TABLE statement.  sqlite3StartTable() gets called
///* first to get things going.  Then this routine is called for each
///* column.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_add_column(p_parse: *mut Parse, mut s_name: Token,
    mut s_type: Token) -> () {
    unsafe {
        unsafe {
            let mut p: *mut Table = core::ptr::null_mut();
            let mut i: i32 = 0;
            let mut z: *mut i8 = core::ptr::null_mut();
            let mut z_type: *mut i8 = core::ptr::null_mut();
            let mut p_col: *mut Column = core::ptr::null_mut();
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            let mut a_new: *mut Column = core::ptr::null_mut();
            let mut e_type: u8 = 0 as u8;
            let mut sz_est: u8 = 1 as u8;
            let mut affinity: i8 = 65 as i8;
            if { p = unsafe { (*p_parse).p_new_table }; p } ==
                    core::ptr::null_mut() {
                return;
            }
            if unsafe { (*p).n_col } as i32 + 1 >
                    unsafe { (*db).a_limit[2 as usize] } {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"too many columns on %s".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p).z_name })
                };
                return;
            }
            if !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 != 0 {
                unsafe { sqlite3_dequote_token(&mut s_name) };
            }
            if s_type.n >= 16 as u32 &&
                    unsafe {
                            sqlite3_strnicmp(unsafe {
                                    s_type.z.add((s_type.n - 6 as u32) as usize)
                                }, c"always".as_ptr() as *mut i8 as *const i8, 6)
                        } == 0 {
                s_type.n -= 6 as u32;
                while s_type.n > 0 as u32 &&
                        unsafe {
                                        *(sqlite3_ctype_map.as_ptr() as
                                                    *const u8).add(unsafe {
                                                            *s_type.z.add((s_type.n - 1 as u32) as usize)
                                                        } as u8 as usize)
                                    } as i32 & 1 != 0 {
                    { let __p = &mut s_type.n; let __t = *__p; *__p -= 1; __t };
                }
                if s_type.n >= 9 as u32 &&
                        unsafe {
                                sqlite3_strnicmp(unsafe {
                                        s_type.z.add((s_type.n - 9 as u32) as usize)
                                    }, c"generated".as_ptr() as *mut i8 as *const i8, 9)
                            } == 0 {
                    s_type.n -= 9 as u32;
                    while s_type.n > 0 as u32 &&
                            unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe {
                                                                *s_type.z.add((s_type.n - 1 as u32) as usize)
                                                            } as u8 as usize)
                                        } as i32 & 1 != 0 {
                        { let __p = &mut s_type.n; let __t = *__p; *__p -= 1; __t };
                    }
                }
            }
            if s_type.n >= 3 as u32 {
                unsafe { sqlite3_dequote_token(&mut s_type) };
                {
                    i = 0;
                    '__b25: loop {
                        if !(i < 6) { break '__b25; }
                        '__c25: loop {
                            if s_type.n ==
                                        unsafe {
                                                *(sqlite3_std_type_len.as_ptr() as
                                                            *const u8).offset(i as isize)
                                            } as u32 &&
                                    unsafe {
                                            sqlite3_strnicmp(s_type.z,
                                                unsafe {
                                                    *(sqlite3_std_type.as_ptr() as
                                                                *mut *const i8).offset(i as isize)
                                                }, s_type.n as i32)
                                        } == 0 {
                                s_type.n = 0 as u32;
                                e_type = (i + 1) as u8;
                                affinity =
                                    unsafe {
                                            *(sqlite3_std_type_affinity.as_ptr() as
                                                        *const i8).offset(i as isize)
                                        } as i8;
                                if affinity as i32 <= 66 { sz_est = 5 as u8; }
                                break '__b25;
                            }
                            break '__c25;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            z =
                unsafe {
                        sqlite3_db_malloc_raw(db,
                            (s_name.n as i64 + 1 as i64 + s_type.n as i64 +
                                    (s_type.n > 0 as u32) as i64) as u64)
                    } as *mut i8;
            if z == core::ptr::null_mut() { return; }
            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                unsafe {
                    sqlite3_rename_token_map(p_parse, z as *mut () as *const (),
                        &raw mut s_name as *const Token)
                };
            }
            unsafe {
                memcpy(z as *mut (), s_name.z as *const (), s_name.n as u64)
            };
            unsafe { *z.add(s_name.n as usize) = 0 as i8 };
            unsafe { sqlite3_dequote(z) };
            if unsafe { (*p).n_col } != 0 &&
                    unsafe { sqlite3_column_index(p, z as *const i8) } >= 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"duplicate column name: %s".as_ptr() as *mut i8 as
                            *const i8, z)
                };
                unsafe { sqlite3_db_free(db, z as *mut ()) };
                return;
            }
            a_new =
                unsafe {
                        sqlite3_db_realloc(db, unsafe { (*p).a_col } as *mut (),
                            (unsafe { (*p).n_col } as i64 + 1 as i64) as u64 *
                                core::mem::size_of::<Column>() as u64)
                    } as *mut Column;
            if a_new == core::ptr::null_mut() {
                unsafe { sqlite3_db_free(db, z as *mut ()) };
                return;
            }
            unsafe { (*p).a_col = a_new };
            p_col =
                unsafe {
                    unsafe { (*p).a_col.offset(unsafe { (*p).n_col } as isize) }
                };
            unsafe {
                memset(p_col as *mut (), 0,
                    core::mem::size_of::<Column>() as u64)
            };
            unsafe { (*p_col).z_cn_name = z };
            unsafe {
                (*p_col).h_name =
                    unsafe { sqlite3_str_i_hash(z as *const i8) }
            };
            if s_type.n == 0 as u32 {

                /// If there is no type specified, columns have the default affinity
                ///* 'BLOB' with a default size of 4 bytes.
                unsafe { (*p_col).affinity = affinity };
                unsafe { (*p_col).set_e_c_type(e_type as u32 as u32) };
                unsafe { (*p_col).sz_est = sz_est };
            } else {
                z_type =
                    unsafe {
                        unsafe {
                            z.offset(unsafe { sqlite3_strlen30(z as *const i8) } as
                                        isize).offset(1 as isize)
                        }
                    };
                unsafe {
                    memcpy(z_type as *mut (), s_type.z as *const (),
                        s_type.n as u64)
                };
                unsafe { *z_type.add(s_type.n as usize) = 0 as i8 };
                unsafe { sqlite3_dequote(z_type) };
                unsafe {
                    (*p_col).affinity =
                        sqlite3_affinity_type(z_type as *const i8, p_col)
                };
                unsafe { (*p_col).col_flags |= 4 as u16 };
            }
            if unsafe { (*p).n_col } as i32 <= 255 {
                let h: u8 =
                    (unsafe { (*p_col).h_name } as u64 %
                            core::mem::size_of::<[u8; 16]>() as u64) as u8;
                unsafe {
                    (*p).a_hx[h as usize] = unsafe { (*p).n_col } as u8
                };
            }
            {
                let __p = unsafe { &mut (*p).n_col };
                let __t = *__p;
                *__p += 1;
                __t
            };
            {
                let __p = unsafe { &mut (*p).n_nv_col };
                let __t = *__p;
                *__p += 1;
                __t
            };
            { let _ = 0; };
            unsafe { (*p_parse).u1.cr.constraint_name.n = 0 as u32 };
        }
    }
}

///* This routine is called by the parser while in the middle of
///* parsing a CREATE TABLE statement.  A "NOT NULL" constraint has
///* been seen on a column.  This routine sets the notNull flag on
///* the column currently under construction.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_add_not_null(p_parse: &Parse, on_error: i32) -> () {
    let mut p: *mut Table = core::ptr::null_mut();
    let mut p_col: *mut Column = core::ptr::null_mut();
    p = (*p_parse).p_new_table;
    if p == core::ptr::null_mut() || (unsafe { (*p).n_col } as i32) < 1 {
        return;
    }
    p_col =
        unsafe {
            unsafe {
                (*p).a_col.offset((unsafe { (*p).n_col } as i32 - 1) as isize)
            }
        };
    unsafe { (*p_col).set_not_null(on_error as u8 as u32 as u32) };
    unsafe { (*p).tab_flags |= 2048 as u32 };
    if unsafe { (*p_col).col_flags } as i32 & 8 != 0 {
        let mut p_idx: *mut Index = core::ptr::null_mut();
        {
            p_idx = unsafe { (*p).p_index };
            '__b26: loop {
                if !(!(p_idx).is_null()) { break '__b26; }
                '__c26: loop {
                    { let _ = 0; };
                    if unsafe {
                                    *unsafe { (*p_idx).ai_column.offset(0 as isize) }
                                } as i32 == unsafe { (*p).n_col } as i32 - 1 {
                        unsafe { (*p_idx).set_uniq_not_null(1 as u32 as u32) };
                    }
                    break '__c26;
                }
                p_idx = unsafe { (*p_idx).p_next };
            }
        }
    }
}

///* Tag the given column as being part of the PRIMARY KEY
extern "C" fn make_column_part_of_primary_key(p_parse_1: *mut Parse,
    p_col_1: &mut Column) -> () {
    (*p_col_1).col_flags |= 1 as u16;
    if (*p_col_1).col_flags as i32 & 96 != 0 {
        unsafe {
            sqlite3_error_msg(p_parse_1,
                c"generated columns cannot be part of the PRIMARY KEY".as_ptr()
                        as *mut i8 as *const i8)
        };
    }
}

///* Backwards Compatibility Hack:
///*
///* Historical versions of SQLite accepted strings as column names in
///* indexes and PRIMARY KEY constraints and in UNIQUE constraints.  Example:
///*
///*     CREATE TABLE xyz(a,b,c,d,e,PRIMARY KEY('a'),UNIQUE('b','c' COLLATE trim)
///*     CREATE INDEX abc ON xyz('c','d' DESC,'e' COLLATE nocase DESC);
///*
///* This is goofy.  But to preserve backwards compatibility we continue to
///* accept it.  This routine does the necessary conversion.  It converts
///* the expression given in its argument from a TK_STRING into a TK_ID
///* if the expression is just a TK_STRING with an optional COLLATE clause.
///* If the expression is anything other than TK_STRING, the expression is
///* unchanged.
extern "C" fn sqlite3_string_to_id(p: &mut Expr) -> () {
    if (*p).op as i32 == 118 {
        (*p).op = 60 as u8;
    } else if (*p).op as i32 == 114 &&
            unsafe { (*(*p).p_left).op } as i32 == 118 {
        unsafe { (*(*p).p_left).op = 60 as u8 };
    }
}

///* If expression list pList contains an expression that was parsed with
///* an explicit "NULLS FIRST" or "NULLS LAST" clause, leave an error in
///* pParse and return non-zero. Otherwise, return zero.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_has_explicit_nulls(p_parse: *mut Parse,
    p_list: *mut ExprList) -> i32 {
    if !(p_list).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b27: loop {
                if !(i < unsafe { (*p_list).n_expr }) { break '__b27; }
                '__c27: loop {
                    if unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).fg.b_nulls()
                            } != 0 {
                        let sf: u8 =
                            unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).fg.sort_flags
                            };
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"unsupported use of NULLS %s".as_ptr() as *mut i8 as
                                    *const i8,
                                if sf as i32 == 0 || sf as i32 == 3 {
                                    c"FIRST".as_ptr() as *mut i8
                                } else { c"LAST".as_ptr() as *mut i8 })
                        };
                        return 1;
                    }
                    break '__c27;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    return 0;
}

///* Reclaim the memory used by an index
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_free_index(db: *mut Sqlite3, p: *mut Index) -> () {
    unsafe { sqlite3_delete_index_samples(db, p) };
    unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_part_idx_where }) };
    unsafe { sqlite3_expr_list_delete(db, unsafe { (*p).a_col_expr }) };
    unsafe { sqlite3_db_free(db, unsafe { (*p).z_col_aff } as *mut ()) };
    if unsafe { (*p).is_resized() } != 0 {
        unsafe { sqlite3_db_free(db, unsafe { (*p).az_coll } as *mut ()) };
    }
    unsafe { sqlite3_db_free(db, p as *mut ()) };
}

///* Locate the in-memory structure that describes a particular database
///* table given the name of that table and (optionally) the name of the
///* database containing the table.  Return NULL if not found.  Also leave an
///* error message in pParse->zErrMsg.
///*
///* The difference between this routine and sqlite3FindTable() is that this
///* routine leaves an error message in pParse->zErrMsg where
///* sqlite3FindTable() does not.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_locate_table(p_parse: *mut Parse, flags: u32,
    z_name: *const i8, z_dbase: *const i8) -> *mut Table {
    let mut p: *mut Table = core::ptr::null_mut();
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    if unsafe { (*db).m_db_flags } & 16 as u32 == 0 as u32 &&
            0 != unsafe { sqlite3_read_schema(p_parse) } {
        return core::ptr::null_mut();
    }
    p = sqlite3_find_table(unsafe { &*db }, z_name, z_dbase);
    if p == core::ptr::null_mut() {
        if unsafe { (*p_parse).prep_flags } as i32 & 4 == 0 &&
                unsafe { (*db).init.busy } as i32 == 0 {
            let mut p_mod: *mut Module =
                unsafe {
                        sqlite3_hash_find(unsafe { &raw mut (*db).a_module } as
                                *const Hash, z_name)
                    } as *mut Module;
            if p_mod == core::ptr::null_mut() &&
                    unsafe {
                            sqlite3_strnicmp(z_name,
                                c"pragma_".as_ptr() as *mut i8 as *const i8, 7)
                        } == 0 {
                p_mod = unsafe { sqlite3_pragma_vtab_register(db, z_name) };
            }
            if p_mod == core::ptr::null_mut() &&
                    unsafe {
                            sqlite3_strnicmp(z_name,
                                c"json".as_ptr() as *mut i8 as *const i8, 4)
                        } == 0 {
                p_mod = unsafe { sqlite3_json_vtab_register(db, z_name) };
            }
            if !(p_mod).is_null() &&
                    unsafe { sqlite3_vtab_eponymous_table_init(p_parse, p_mod) }
                        != 0 {
                return unsafe { (*p_mod).p_epo_tab };
            }
        }
        if flags & 2 as u32 != 0 { return core::ptr::null_mut(); }
        unsafe { (*p_parse).set_check_schema(1 as Bft as u32) };
    } else if unsafe { (*p).e_tab_type } as i32 == 1 &&
            unsafe { (*p_parse).prep_flags } as i32 & 4 != 0 {
        p = core::ptr::null_mut();
    }
    if p == core::ptr::null_mut() {
        let z_msg: *const i8 =
            if flags & 1 as u32 != 0 {
                    c"no such view".as_ptr() as *mut i8
                } else { c"no such table".as_ptr() as *mut i8 } as *const i8;
        if !(z_dbase).is_null() {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"%s: %s.%s".as_ptr() as *mut i8 as *const i8, z_msg,
                    z_dbase, z_name)
            };
        } else {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"%s: %s".as_ptr() as *mut i8 as *const i8, z_msg, z_name)
            };
        }
    } else { { let _ = 0; }; }
    return p;
}

///* Locate the table identified by *p.
///*
///* This is a wrapper around sqlite3LocateTable(). The difference between
///* sqlite3LocateTable() and this function is that this function restricts
///* the search to schema (p->pSchema) if it is not NULL. p->pSchema may be
///* non-NULL if it is part of a view or trigger program definition. See
///* sqlite3FixSrcList() for details.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_locate_table_item(p_parse: *mut Parse, flags: u32,
    p: &SrcItem) -> *mut Table {
    unsafe {
        let mut z_db: *const i8 = core::ptr::null();
        if (*p).fg.fixed_schema() != 0 {
            let i_db: i32 =
                unsafe {
                    sqlite3_schema_to_index(unsafe { (*p_parse).db },
                        (*p).u4.p_schema)
                };
            { let _ = 0; };
            z_db =
                unsafe {
                        (*unsafe {
                                    (*unsafe { (*p_parse).db }).a_db.offset(i_db as isize)
                                }).z_db_s_name
                    } as *const i8;
        } else { { let _ = 0; }; z_db = (*p).u4.z_database as *const i8; }
        return sqlite3_locate_table(p_parse, flags, (*p).z_name as *const i8,
                z_db);
    }
}

///* Allocate heap space to hold an Index object with nCol columns.
///*
///* Increase the allocation size to provide an extra nExtra bytes
///* of 8-byte aligned space after the Index object and return a
///* pointer to this extra space in *ppExtra.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_allocate_index_object(db: *mut Sqlite3, n_col: i32,
    n_extra: i32, pp_extra: &mut *mut i8) -> *mut Index {
    let mut p: *mut Index = core::ptr::null_mut();
    /// Allocated index object
    let mut n_byte: i64 = 0 as i64;

    /// Bytes of space for Index object + arrays
    { let _ = 0; };
    n_byte =
        ((core::mem::size_of::<Index>() as u64 + 7 as u64 & !7 as u64) +
                    (core::mem::size_of::<*mut i8>() as u64 * n_col as u64 +
                            7 as u64 & !7 as u64) +
                (core::mem::size_of::<LogEst>() as u64 * (n_col + 1) as u64 +
                                core::mem::size_of::<i16>() as u64 * n_col as u64 +
                            core::mem::size_of::<u8>() as u64 * n_col as u64 + 7 as u64
                    & !7 as u64)) as i64;

    /// Index.azColl
    /// Index.aiRowLogEst
    /// Index.aiColumn
    /// Index.aSortOrder
    (p =
        unsafe {
                sqlite3_db_malloc_zero(db, (n_byte + n_extra as i64) as u64)
            } as *mut Index);
    if !(p).is_null() {
        let mut p_extra: *mut i8 =
            unsafe {
                (p as
                        *mut i8).add((core::mem::size_of::<Index>() as u64 +
                                7 as u64 & !7 as u64) as usize)
            };
        unsafe { (*p).az_coll = p_extra as *mut *const i8 };
        {
            let __n =
                core::mem::size_of::<*mut i8>() as u64 * n_col as u64 +
                        7 as u64 & !7 as u64;
            let __p = &mut p_extra;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        unsafe { (*p).ai_row_log_est = p_extra as *mut LogEst };
        {
            let __n =
                core::mem::size_of::<LogEst>() as u64 * (n_col + 1) as u64;
            let __p = &mut p_extra;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        unsafe { (*p).ai_column = p_extra as *mut i16 };
        {
            let __n = core::mem::size_of::<i16>() as u64 * n_col as u64;
            let __p = &mut p_extra;
            *__p = unsafe { (*__p).add(__n as usize) };
        };
        unsafe { (*p).a_sort_order = p_extra as *mut u8 };
        { let _ = 0; };
        unsafe { (*p).n_column = n_col as u16 };
        unsafe { (*p).n_key_col = (n_col - 1) as u16 };
        *pp_extra = unsafe { (p as *mut i8).offset(n_byte as isize) };
    }
    return p;
}

///* Return true if any of the first nKey entries of index pIdx exactly
///* match the iCol-th entry of pPk.  pPk is always a WITHOUT ROWID
///* PRIMARY KEY index.  pIdx is an index on the same table.  pIdx may
///* or may not be the same index as pPk.
///*
///* The first nKey entries of pIdx are guaranteed to be ordinary columns,
///* not a rowid or expression.
///*
///* This routine differs from hasColumn() in that both the column and the
///* collating sequence must match for this routine, but for hasColumn() only
///* the column name must match.
extern "C" fn is_dup_column(p_idx_1: &Index, n_key_1: i32, p_pk_1: &Index,
    i_col_1: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    j = unsafe { *(*p_pk_1).ai_column.offset(i_col_1 as isize) } as i32;
    { let _ = 0; };
    {
        i = 0;
        '__b28: loop {
            if !(i < n_key_1) { break '__b28; }
            '__c28: loop {
                { let _ = 0; };
                if unsafe { *(*p_idx_1).ai_column.offset(i as isize) } as i32
                            == j &&
                        unsafe {
                                sqlite3_str_i_cmp(unsafe {
                                        *(*p_idx_1).az_coll.offset(i as isize)
                                    }, unsafe { *(*p_pk_1).az_coll.offset(i_col_1 as isize) })
                            } == 0 {
                    return 1;
                }
                break '__c28;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Fill the Index.aiRowEst[] array with default information - information
///* to be used when we have not run the ANALYZE command.
///*
///* aiRowEst[0] is supposed to contain the number of elements in the index.
///* Since we do not know, guess 1 million.  aiRowEst[1] is an estimate of the
///* number of rows in the table that match any particular value of the
///* first column of the index.  aiRowEst[2] is an estimate of the number
///* of rows that match any particular combination of the first 2 columns
///* of the index.  And so forth.  It must always be the case that
///
///*           aiRowEst[N]<=aiRowEst[N-1]
///*           aiRowEst[N]>=1
///*
///* Apart from that, we have little to go on besides intuition as to
///* how aiRowEst[] should be initialized.  The numbers generated here
///* are based on typical values found in actual indices.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_default_row_est(p_idx: &Index) -> () {
    let a: *mut LogEst = (*p_idx).ai_row_log_est;
    let mut x: LogEst = 0 as LogEst;
    let n_copy: i32 =
        if ((core::mem::size_of::<[i16; 5]>() as u64 /
                            core::mem::size_of::<LogEst>() as u64) as i32) <
                (*p_idx).n_key_col as i32 {
            (core::mem::size_of::<[i16; 5]>() as u64 /
                    core::mem::size_of::<LogEst>() as u64) as i32
        } else { (*p_idx).n_key_col as i32 };
    let mut i: i32 = 0;

    /// Indexes with default row estimates should not have stat1 data
    { let _ = 0; };

    /// Set the first entry (number of rows in the index) to the estimated
    ///* number of rows in the table, or half the number of rows in the table
    ///* for a partial index.
    ///*
    ///* 2020-05-27:  If some of the stat data is coming from the sqlite_stat1
    ///* table but other parts we are having to guess at, then do not let the
    ///* estimated number of rows in the table be less than 1000 (LogEst 99).
    ///* Failure to do this can cause the indexes for which we do not have
    ///* stat1 data to be ignored by the query planner.
    (x = unsafe { (*(*p_idx).p_table).n_row_log_est });
    { let _ = 0; };
    if (x as i32) < 99 {
        unsafe {
            (*(*p_idx).p_table).n_row_log_est = { x = 99 as LogEst; x }
        };
    }
    if (*p_idx).p_part_idx_where != core::ptr::null_mut() {
        x -= 10 as LogEst;
        { let _ = 0; };
    }
    unsafe { *a.offset(0 as isize) = x };

    /// Estimate that a[1] is 10, a[2] is 9, a[3] is 8, a[4] is 7, a[5] is
    ///* 6 and each subsequent value (if any) is 5.
    unsafe {
        memcpy(unsafe { &raw mut *a.offset(1 as isize) } as *mut (),
            &raw const a_val[0 as usize] as *const LogEst as *const (),
            n_copy as u64 * core::mem::size_of::<LogEst>() as u64)
    };
    {
        i = n_copy + 1;
        '__b29: loop {
            if !(i <= (*p_idx).n_key_col as i32) { break '__b29; }
            '__c29: loop {
                unsafe { *a.offset(i as isize) = 23 as LogEst };
                { let _ = 0; };
                break '__c29;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
    if (*p_idx).on_error as i32 != 0 {
        unsafe { *a.add((*p_idx).n_key_col as usize) = 0 as LogEst };
    }
}

///* Estimate the average size of a row for an index.
extern "C" fn estimate_index_width(p_idx_1: &mut Index) -> () {
    let mut w_index: u32 = 0 as u32;
    let mut i: i32 = 0;
    let a_col: *const Column =
        unsafe { (*(*p_idx_1).p_table).a_col } as *const Column;
    {
        i = 0;
        '__b30: loop {
            if !(i < (*p_idx_1).n_column as i32) { break '__b30; }
            '__c30: loop {
                let x: i16 =
                    unsafe { *(*p_idx_1).ai_column.offset(i as isize) };
                { let _ = 0; };
                w_index +=
                    if (x as i32) < 0 {
                            1
                        } else {
                            (unsafe { (*a_col.offset(x as isize)).sz_est }) as i32
                        } as u32;
                break '__c30;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    (*p_idx_1).sz_idx_row =
        unsafe { sqlite3_log_est((w_index * 4 as u32) as u64) };
}

/// Recompute the colNotIdxed field of the Index.
///*
///* colNotIdxed is a bitmask that has a 0 bit representing each indexed
///* columns that are within the first 63 columns of the table and a 1 for
///* all other bits (all columns that are not in the index).  The
///* high-order bit of colNotIdxed is always 1.  All unindexed columns
///* of the table have a 1.
///*
///* 2019-10-24:  For the purpose of this computation, virtual columns are
///* not considered to be covered by the index, even if they are in the
///* index, because we do not trust the logic in whereIndexExprTrans() to be
///* able to find all instances of a reference to the indexed table column
///* and convert them into references to the index.  Hence we always want
///* the actual table at hand in order to recompute the virtual column, if
///* necessary.
///*
///* The colNotIdxed mask is AND-ed with the SrcList.a[].colUsed mask
///* to determine if the index is covering index.
extern "C" fn recompute_columns_not_indexed(p_idx_1: &mut Index) -> () {
    let mut m: Bitmask = 0 as Bitmask;
    let mut j: i32 = 0;
    let p_tab: *const Table = (*p_idx_1).p_table as *const Table;
    {
        j = (*p_idx_1).n_column as i32 - 1;
        '__b31: loop {
            if !(j >= 0) { break '__b31; }
            '__c31: loop {
                let x: i32 =
                    unsafe { *(*p_idx_1).ai_column.offset(j as isize) } as i32;
                if x >= 0 &&
                        unsafe {
                                        (*unsafe { (*p_tab).a_col.offset(x as isize) }).col_flags
                                    } as i32 & 32 == 0 {
                    if x <
                            (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                1 {
                        m |= (1 as Bitmask) << x;
                    }
                }
                break '__c31;
            }
            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
        }
    }
    (*p_idx_1).col_not_idxed = !m;
    { let _ = 0; };
}

///* Run the parser and code generator recursively in order to generate
///* code for the SQL statement given onto the end of the pParse context
///* currently under construction.  Notes:
///*
///*   *  The final OP_Halt is not appended and other initialization
///*      and finalization steps are omitted because those are handling by the
///*      outermost parser.
///*
///*   *  Built-in SQL functions always take precedence over application-defined
///*      SQL functions.  In other words, it is not possible to override a
///*      built-in function.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub unsafe extern "C" fn sqlite3_nested_parse(p_parse: *mut Parse,
    z_format: *const i8, mut __va0: ...) -> () {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut z_sql: *mut i8 = core::ptr::null_mut();
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let saved_db_flags: u32 = unsafe { (*db).m_db_flags };
    let mut save_buf: [i8; 136] = [0; 136];
    if unsafe { (*p_parse).n_err } != 0 { return; }
    if unsafe { (*p_parse).e_parse_mode } != 0 { return; }
    { let _ = 0; };

    /// Nesting should only be of limited depth
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_sql = unsafe { sqlite3_vm_printf(db, z_format, ap) };
    ();
    if z_sql == core::ptr::null_mut() {
        if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
            unsafe { (*p_parse).rc = 18 };
        }
        {
            let __p = unsafe { &mut (*p_parse).n_err };
            let __t = *__p;
            *__p += 1;
            __t
        };
        return;
    }
    {
        let __p = unsafe { &mut (*p_parse).nested };
        let __t = *__p;
        *__p += 1;
        __t
    };
    unsafe {
        memcpy(&raw mut save_buf[0 as usize] as *mut i8 as *mut (),
            unsafe {
                    (p_parse as
                            *mut i8).add(core::mem::offset_of!(Parse, s_last_token) as
                            usize)
                } as *const (),
            core::mem::size_of::<Parse>() as u64 -
                core::mem::offset_of!(Parse, s_last_token) as u64)
    };
    unsafe {
        memset(unsafe {
                    (p_parse as
                            *mut i8).add(core::mem::offset_of!(Parse, s_last_token) as
                            usize)
                } as *mut (), 0,
            core::mem::size_of::<Parse>() as u64 -
                core::mem::offset_of!(Parse, s_last_token) as u64)
    };
    unsafe { (*db).m_db_flags |= 2 as u32 };
    unsafe { sqlite3_run_parser(p_parse, z_sql as *const i8) };
    unsafe { (*db).m_db_flags = saved_db_flags };
    unsafe { sqlite3_db_free(db, z_sql as *mut ()) };
    unsafe {
        memcpy(unsafe {
                    (p_parse as
                            *mut i8).add(core::mem::offset_of!(Parse, s_last_token) as
                            usize)
                } as *mut (),
            &raw mut save_buf[0 as usize] as *mut i8 as *const (),
            core::mem::size_of::<Parse>() as u64 -
                core::mem::offset_of!(Parse, s_last_token) as u64)
    };
    {
        let __p = unsafe { &mut (*p_parse).nested };
        let __t = *__p;
        *__p -= 1;
        __t
    };
}

///* Return a KeyInfo structure that is appropriate for the given Index.
///*
///* The caller should invoke sqlite3KeyInfoUnref() on the returned object
///* when it has finished using it.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_key_info_of_index(p_parse: *mut Parse,
    p_idx: &mut Index) -> *mut KeyInfo {
    unsafe {
        unsafe {
            let mut i: i32 = 0;
            let n_col: i32 = (*p_idx).n_column as i32;
            let n_key: i32 = (*p_idx).n_key_col as i32;
            let mut p_key: *mut KeyInfo = core::ptr::null_mut();
            if unsafe { (*p_parse).n_err } != 0 {
                return core::ptr::null_mut();
            }
            if (*p_idx).uniq_not_null() != 0 {
                p_key =
                    unsafe {
                        sqlite3_key_info_alloc(unsafe { (*p_parse).db }, n_key,
                            n_col - n_key)
                    };
            } else {
                p_key =
                    unsafe {
                        sqlite3_key_info_alloc(unsafe { (*p_parse).db }, n_col, 0)
                    };
            }
            if !(p_key).is_null() {
                { let _ = 0; };
                {
                    i = 0;
                    '__b32: loop {
                        if !(i < n_col) { break '__b32; }
                        '__c32: loop {
                            let z_coll: *const i8 =
                                unsafe { *(*p_idx).az_coll.offset(i as isize) };
                            unsafe {
                                *(unsafe { (*p_key).a_coll.as_ptr() } as
                                                *mut *mut CollSeq).offset(i as isize) =
                                    if z_coll == sqlite3_str_binary.as_ptr() as *const i8 {
                                        core::ptr::null_mut()
                                    } else {
                                        unsafe { sqlite3_locate_coll_seq(p_parse, z_coll) }
                                    }
                            };
                            unsafe {
                                *unsafe { (*p_key).a_sort_flags.offset(i as isize) } =
                                    unsafe { *(*p_idx).a_sort_order.offset(i as isize) }
                            };
                            { let _ = 0; };
                            break '__c32;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if unsafe { (*p_parse).n_err } != 0 {
                    { let _ = 0; };
                    if (*p_idx).b_no_query() as i32 == 0 &&
                            !(unsafe {
                                            sqlite3_hash_find(unsafe {
                                                        &raw mut (*(*p_idx).p_schema).idx_hash
                                                    } as *const Hash, (*p_idx).z_name as *const i8)
                                        }).is_null() {

                        /// Deactivate the index because it contains an unknown collating
                        ///* sequence.  The only way to reactive the index is to reload the
                        ///* schema.  Adding the missing collating sequence later does not
                        ///* reactive the index.  The application had the chance to register
                        ///* the missing index using the collation-needed callback.  For
                        ///* simplicity, SQLite will not give the application a second chance.
                        ///*
                        ///* Except, do not do this if the index is not in the schema hash
                        ///* table. In this case the index is currently being constructed
                        ///* by a CREATE INDEX statement, and retrying will not help.
                        (*p_idx).set_b_no_query(1 as u32 as u32);
                        unsafe { (*p_parse).rc = 1 | 2 << 8 };
                    }
                    unsafe { sqlite3_key_info_unref(p_key) };
                    p_key = core::ptr::null_mut();
                }
            }
            return p_key;
        }
    }
}

///* Indicate that the statement currently under construction might write
///* more than one entry (example: deleting one row then inserting another,
///* inserting multiple rows in a table, or inserting a row and index entries.)
///* If an abort occurs after some of these writes have completed, then it will
///* be necessary to undo the completed writes.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_multi_write(p_parse: *mut Parse) -> () {
    let p_toplevel: *mut Parse =
        if !(unsafe { (*p_parse).p_toplevel }).is_null() {
            unsafe { (*p_parse).p_toplevel }
        } else { p_parse };
    unsafe { (*p_toplevel).is_multi_write = 1 as u8 };
}

///* The code generator calls this routine if is discovers that it is
///* possible to abort a statement prior to completion.  In order to
///* perform this abort without corrupting the database, we need to make
///* sure that the statement is protected by a statement transaction.
///*
///* Technically, we only need to set the mayAbort flag if the
///* isMultiWrite flag was previously set.  There is a time dependency
///* such that the abort must occur after the multiwrite.  This makes
///* some statements involving the REPLACE conflict resolution algorithm
///* go a little faster.  But taking advantage of this time dependency
///* makes it more difficult to prove that the code is correct (in
///* particular, it prevents us from writing an effective
///* implementation of sqlite3AssertMayAbort()) and so we have chosen
///* to take the safe route and skip the optimization.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_may_abort(p_parse: *mut Parse) -> () {
    let p_toplevel: *mut Parse =
        if !(unsafe { (*p_parse).p_toplevel }).is_null() {
            unsafe { (*p_parse).p_toplevel }
        } else { p_parse };
    unsafe { (*p_toplevel).set_may_abort(1 as Bft as u32) };
}

///* Code an OP_Halt that causes the vdbe to return an SQLITE_CONSTRAINT
///* error. The onError parameter determines which (if any) of the statement
///* and/or current transaction is rolled back.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_halt_constraint(p_parse: *mut Parse, err_code: i32,
    on_error: i32, p4: *mut i8, p4type: i8, p5_errmsg: u8) -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    { let _ = 0; };
    v = unsafe { sqlite3_get_vdbe(p_parse) };
    { let _ = 0; };
    if on_error == 2 { sqlite3_may_abort(p_parse); }
    unsafe {
        sqlite3_vdbe_add_op4(v, 72, err_code, on_error, 0, p4 as *const i8,
            p4type as i32)
    };
    unsafe { sqlite3_vdbe_change_p5(v, p5_errmsg as u16) };
}

///* Code an OP_Halt due to UNIQUE or PRIMARY KEY constraint violation.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_unique_constraint(p_parse: *mut Parse,
    on_error: i32, p_idx: &Index) -> () {
    let mut z_err: *mut i8 = core::ptr::null_mut();
    let mut j: i32 = 0;
    let mut err_msg: StrAccum = unsafe { core::mem::zeroed() };
    let p_tab: *const Table = (*p_idx).p_table as *const Table;
    unsafe {
        sqlite3_str_accum_init(&mut err_msg, unsafe { (*p_parse).db },
            core::ptr::null_mut(), 0,
            unsafe { (*unsafe { (*p_parse).db }).a_limit[0 as usize] })
    };
    if !((*p_idx).a_col_expr).is_null() {
        unsafe {
            sqlite3_str_appendf(&raw mut err_msg as *mut Sqlite3Str,
                c"index \'%q\'".as_ptr() as *mut i8 as *const i8,
                (*p_idx).z_name)
        };
    } else {
        {
            j = 0;
            '__b33: loop {
                if !(j < (*p_idx).n_key_col as i32) { break '__b33; }
                '__c33: loop {
                    let mut z_col: *const i8 = core::ptr::null();
                    { let _ = 0; };
                    z_col =
                        unsafe {
                            (*unsafe {
                                        (*p_tab).a_col.offset(unsafe {
                                                    *(*p_idx).ai_column.offset(j as isize)
                                                } as isize)
                                    }).z_cn_name
                        };
                    if j != 0 {
                        unsafe {
                            sqlite3_str_append(&raw mut err_msg as *mut Sqlite3Str,
                                c", ".as_ptr() as *mut i8 as *const i8, 2)
                        };
                    }
                    unsafe {
                        sqlite3_str_appendall(&raw mut err_msg as *mut Sqlite3Str,
                            unsafe { (*p_tab).z_name } as *const i8)
                    };
                    unsafe {
                        sqlite3_str_append(&raw mut err_msg as *mut Sqlite3Str,
                            c".".as_ptr() as *mut i8 as *const i8, 1)
                    };
                    unsafe {
                        sqlite3_str_appendall(&raw mut err_msg as *mut Sqlite3Str,
                            z_col as *const i8)
                    };
                    break '__c33;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    z_err = unsafe { sqlite3_str_accum_finish(&mut err_msg) };
    sqlite3_halt_constraint(p_parse,
        if (*p_idx).idx_type() as i32 == 2 {
            19 | 6 << 8
        } else { 19 | 8 << 8 }, on_error, z_err, -7 as i8, 2 as u8);
}

///* Generate code that will erase and refill index *pIdx.  This is
///* used to initialize a newly created index or to recompute the
///* content of an index in response to a REINDEX command.
///*
///* if memRootPage is not negative, it means that the index is newly
///* created.  The register specified by memRootPage contains the
///* root page number of the index.  If memRootPage is negative, then
///* the index already exists and must be cleared before being refilled and
///* the root page number of the index is taken from pIndex->tnum.
#[allow(unused_doc_comments)]
extern "C" fn sqlite3_refill_index(p_parse_1: *mut Parse,
    p_index_1: *mut Index, mem_root_page_1: i32) -> () {
    unsafe {
        let p_tab: *mut Table = unsafe { (*p_index_1).p_table };
        /// The table that is indexed
        let i_tab: i32 =
            {
                let __p = unsafe { &mut (*p_parse_1).n_tab };
                let __t = *__p;
                *__p += 1;
                __t
            };
        /// Btree cursor used for pTab
        let i_idx: i32 =
            {
                let __p = unsafe { &mut (*p_parse_1).n_tab };
                let __t = *__p;
                *__p += 1;
                __t
            };
        /// Btree cursor used for pIndex
        let mut i_sorter: i32 = 0;
        /// Cursor opened by OpenSorter (if in use)
        let mut addr1: i32 = 0;
        /// Address of top of loop
        let mut addr2: i32 = 0;
        /// Address to jump to for next iteration
        let mut tnum: Pgno = 0 as Pgno;
        /// Root page of index
        let mut i_part_idx_label: i32 = 0;
        /// Jump to this label to skip a row
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// Generate code into this virtual machine
        let mut p_key: *mut KeyInfo = core::ptr::null_mut();
        /// KeyInfo for index
        let mut reg_record: i32 = 0;
        /// Register holding assembled index record
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        /// The database connection
        let i_db: i32 =
            unsafe {
                sqlite3_schema_to_index(db, unsafe { (*p_index_1).p_schema })
            };
        if unsafe {
                    sqlite3_auth_check(p_parse_1, 27,
                        unsafe { (*p_index_1).z_name } as *const i8,
                        core::ptr::null(),
                        unsafe {
                                (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                            } as *const i8)
                } != 0 {
            return;
        }

        /// Require a write-lock on the table to perform this operation
        sqlite3_table_lock(p_parse_1, i_db, unsafe { (*p_tab).tnum }, 1 as u8,
            unsafe { (*p_tab).z_name } as *const i8);
        v = unsafe { sqlite3_get_vdbe(p_parse_1) };
        if v == core::ptr::null_mut() { return; }
        if mem_root_page_1 >= 0 {
            tnum = mem_root_page_1 as Pgno;
        } else { tnum = unsafe { (*p_index_1).tnum }; }
        p_key =
            sqlite3_key_info_of_index(p_parse_1, unsafe { &mut *p_index_1 });
        { let _ = 0; };

        /// Open the sorter cursor if we are to use one.
        (i_sorter =
            {
                let __p = unsafe { &mut (*p_parse_1).n_tab };
                let __t = *__p;
                *__p += 1;
                __t
            });
        unsafe {
            sqlite3_vdbe_add_op4(v, 121, i_sorter, 0,
                unsafe { (*p_index_1).n_key_col } as i32,
                unsafe { sqlite3_key_info_ref(p_key) } as *mut i8 as
                    *const i8, -9)
        };

        /// Open the table. Loop through all rows of the table, inserting index
        ///* records into the sorter.
        unsafe { sqlite3_open_table(p_parse_1, i_tab, i_db, p_tab, 114) };
        addr1 = unsafe { sqlite3_vdbe_add_op2(v, 36, i_tab, 0) };
        reg_record = unsafe { sqlite3_get_temp_reg(p_parse_1) };
        sqlite3_multi_write(p_parse_1);
        unsafe {
            sqlite3_generate_index_key(p_parse_1, p_index_1, i_tab,
                reg_record, 0, &mut i_part_idx_label, core::ptr::null_mut(),
                0)
        };
        unsafe { sqlite3_vdbe_add_op2(v, 141, i_sorter, reg_record) };
        unsafe {
            sqlite3_resolve_part_idx_label(p_parse_1, i_part_idx_label)
        };
        unsafe { sqlite3_vdbe_add_op2(v, 40, i_tab, addr1 + 1) };
        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
        if mem_root_page_1 < 0 {
            unsafe { sqlite3_vdbe_add_op2(v, 147, tnum as i32, i_db) };
        }
        unsafe {
            sqlite3_vdbe_add_op4(v, 116, i_idx, tnum as i32, i_db,
                p_key as *mut i8 as *const i8, -9)
        };
        unsafe {
            sqlite3_vdbe_change_p5(v,
                (1 | if mem_root_page_1 >= 0 { 16 } else { 0 }) as u16)
        };
        addr1 = unsafe { sqlite3_vdbe_add_op2(v, 34, i_sorter, 0) };
        if unsafe { (*p_index_1).on_error } as i32 != 0 {
            let j2: i32 = unsafe { sqlite3_vdbe_goto(v, 1) };
            addr2 = unsafe { sqlite3_vdbe_current_addr(v) };
            unsafe {
                sqlite3_vdbe_add_op4_int(v, 134, i_sorter, j2, reg_record,
                    unsafe { (*p_index_1).n_key_col } as i32)
            };
            sqlite3_unique_constraint(p_parse_1, 2, unsafe { &*p_index_1 });
            unsafe { sqlite3_vdbe_jump_here(v, j2) };
        } else {

            /// Most CREATE INDEX and REINDEX statements that are not UNIQUE can not
            ///* abort. The exception is if one of the indexed expressions contains a
            ///* user function that throws an exception when it is evaluated. But the
            ///* overhead of adding a statement journal to a CREATE INDEX statement is
            ///* very small (since most of the pages written do not contain content that
            ///* needs to be restored if the statement aborts), so we call
            ///* sqlite3MayAbort() for all CREATE INDEX statements.
            sqlite3_may_abort(p_parse_1);
            addr2 = unsafe { sqlite3_vdbe_current_addr(v) };
        }
        unsafe { sqlite3_vdbe_add_op3(v, 135, i_sorter, reg_record, i_idx) };
        if (unsafe { (*p_index_1).b_asc_key_bug() } == 0) as i32 != 0 {

            /// This OP_SeekEnd opcode makes index insert for a REINDEX go much
            ///* faster by avoiding unnecessary seeks.  But the optimization does
            ///* not work for UNIQUE constraint indexes on WITHOUT ROWID tables
            ///* with DESC primary keys, since those indexes have there keys in
            ///* a different order from the main table.
            ///* See ticket: https://sqlite.org/src/info/bba7b69f9849b5bf
            unsafe { sqlite3_vdbe_add_op1(v, 139, i_idx) };
        }
        unsafe { sqlite3_vdbe_add_op2(v, 140, i_idx, reg_record) };
        unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
        unsafe { sqlite3_release_temp_reg(p_parse_1, reg_record) };
        unsafe { sqlite3_vdbe_add_op2(v, 38, i_sorter, addr2) };
        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
        unsafe { sqlite3_vdbe_add_op1(v, 124, i_tab) };
        unsafe { sqlite3_vdbe_add_op1(v, 124, i_idx) };
        unsafe { sqlite3_vdbe_add_op1(v, 124, i_sorter) };
    }
}

///* Generate code that will increment the schema cookie.
///*
///* The schema cookie is used to determine when the schema for the
///* database changes.  After each schema change, the cookie value
///* changes.  When a process first reads the schema it records the
///* cookie.  Thereafter, whenever it goes to access the database,
///* it checks the cookie to make sure the schema has not changed
///* since it was last read.
///*
///* This plan is not completely bullet-proof.  It is possible for
///* the schema to change multiple times and for the cookie to be
///* set back to prior value.  But schema changes are infrequent
///* and the probability of hitting the same cookie value is only
///* 1 chance in 2^32.  So we're safe enough.
///*
///* IMPLEMENTATION-OF: R-34230-56049 SQLite automatically increments
///* the schema-version whenever the schema changes.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_change_cookie(p_parse: &Parse, i_db: i32) -> () {
    unsafe {
        let db: *const Sqlite3 = (*p_parse).db as *const Sqlite3;
        let v: *mut Vdbe = (*p_parse).p_vdbe;
        { let _ = 0; };
        unsafe {
            sqlite3_vdbe_add_op3(v, 102, i_db, 1,
                (1 as u32 +
                        unsafe {
                                (*unsafe {
                                                (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                            }).schema_cookie
                            } as u32) as i32)
        };
    }
}

///* Delete a Subquery object and its substructure.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_subquery_delete(db: *mut Sqlite3,
    p_subq: *mut Subquery) -> () {
    unsafe {
        { let _ = 0; };
        unsafe { sqlite3_select_delete(db, unsafe { (*p_subq).p_select }) };
        unsafe { sqlite3_db_free(db, p_subq as *mut ()) };
    }
}

///* Remove the memory data structures associated with the given
///* Table.  No changes are made to disk by this routine.
///*
///* This routine just deletes the data structure.  It does not unlink
///* the table data structure from the hash table.  But it does destroy
///* memory structures of the indices and foreign keys associated with
///* the table.
///*
///* The db parameter is optional.  It is needed if the Table object
///* contains lookaside memory.  (Table objects in the schema do not use
///* lookaside memory, but some ephemeral Table objects do.)  Or the
///* db parameter can be used with db->pnBytesFreed to measure the memory
///* used by the Table object.
#[allow(unused_doc_comments)]
extern "C" fn delete_table(db: *mut Sqlite3, p_table_1: *mut Table) -> () {
    unsafe {
        let mut p_index: *mut Index = core::ptr::null_mut();
        let mut p_next: *mut Index = core::ptr::null_mut();
        {
            p_index = unsafe { (*p_table_1).p_index };
            '__b34: loop {
                if !(!(p_index).is_null()) { break '__b34; }
                '__c34: loop {
                    p_next = unsafe { (*p_index).p_next };
                    { let _ = 0; };
                    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut()
                            &&
                            !(unsafe { (*p_table_1).e_tab_type } as i32 == 1) as i32 !=
                                0 {
                        let z_name: *const i8 =
                            unsafe { (*p_index).z_name } as *const i8;
                        unsafe {
                            sqlite3_hash_insert(unsafe {
                                    &mut (*unsafe { (*p_index).p_schema }).idx_hash
                                }, z_name as *const i8, core::ptr::null_mut())
                        };
                        { let _ = 0; };
                        { let _ = 0; };
                    }
                    sqlite3_free_index(db, p_index);
                    break '__c34;
                }
                p_index = p_next;
            }
        }
        if unsafe { (*p_table_1).e_tab_type } as i32 == 0 {
            unsafe { sqlite3_fk_delete(db, p_table_1) };
        } else if unsafe { (*p_table_1).e_tab_type } as i32 == 1 {
            unsafe { sqlite3_vtab_clear(db, p_table_1) };
        } else {
            { let _ = 0; };
            unsafe {
                sqlite3_select_delete(db,
                    unsafe { (*p_table_1).u.view.p_select })
            };
        }

        /// Delete the Table structure itself.
        sqlite3_delete_column_names(db, unsafe { &mut *p_table_1 });
        unsafe {
            sqlite3_db_free(db, unsafe { (*p_table_1).z_name } as *mut ())
        };
        unsafe {
            sqlite3_db_free(db, unsafe { (*p_table_1).z_col_aff } as *mut ())
        };
        unsafe {
            sqlite3_expr_list_delete(db, unsafe { (*p_table_1).p_check })
        };
        unsafe { sqlite3_db_free(db, p_table_1 as *mut ()) };

        /// Verify that no lookaside memory was used by schema tables
        { let _ = 0; };
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_delete_table(db: *mut Sqlite3, p_table: *mut Table)
    -> () {

    /// Do not delete the table until the reference count reaches zero.
    { let _ = 0; };
    if (p_table).is_null() as i32 != 0 { return; }
    if unsafe { (*db).pn_bytes_freed } == core::ptr::null_mut() &&
            {
                    let __p = unsafe { &mut (*p_table).n_tab_ref };
                    *__p -= 1;
                    *__p
                } > 0 as u32 {
        return;
    }
    delete_table(db, p_table);
}

///* Delete an IdList.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_id_list_delete(db: *mut Sqlite3,
    p_list: *mut IdList) -> () {
    let mut i: i32 = 0;
    { let _ = 0; };
    if p_list == core::ptr::null_mut() { return; }
    {
        i = 0;
        '__b35: loop {
            if !(i < unsafe { (*p_list).n_id }) { break '__b35; }
            '__c35: loop {
                unsafe {
                    sqlite3_db_free(db,
                        unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                *mut IdListItem).offset(i as isize)).z_name
                            } as *mut ())
                };
                break '__c35;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_db_nn_free_nn(db, p_list as *mut ()) };
}

///* Delete an entire SrcList including all its substructure.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_src_list_delete(db: *mut Sqlite3,
    p_list: *mut SrcList) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut p_item: *const SrcItem = core::ptr::null();
        { let _ = 0; };
        if p_list == core::ptr::null_mut() { return; }
        {
            {
                p_item = unsafe { (*p_list).a.as_ptr() } as *mut SrcItem;
                i = 0
            };
            '__b36: loop {
                if !(i < unsafe { (*p_list).n_src }) { break '__b36; }
                '__c36: loop {

                    /// Check invariants on SrcItem
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    if !(unsafe { (*p_item).z_name }).is_null() {
                        unsafe {
                            sqlite3_db_nn_free_nn(db,
                                unsafe { (*p_item).z_name } as *mut ())
                        };
                    }
                    if !(unsafe { (*p_item).z_alias }).is_null() {
                        unsafe {
                            sqlite3_db_nn_free_nn(db,
                                unsafe { (*p_item).z_alias } as *mut ())
                        };
                    }
                    if unsafe { (*p_item).fg.is_subquery() } != 0 {
                        sqlite3_subquery_delete(db, unsafe { (*p_item).u4.p_subq });
                    } else if unsafe { (*p_item).fg.fixed_schema() } as i32 == 0
                            &&
                            unsafe { (*p_item).u4.z_database } != core::ptr::null_mut()
                        {
                        unsafe {
                            sqlite3_db_nn_free_nn(db,
                                unsafe { (*p_item).u4.z_database } as *mut ())
                        };
                    }
                    if unsafe { (*p_item).fg.is_indexed_by() } != 0 {
                        unsafe {
                            sqlite3_db_free(db,
                                unsafe { (*p_item).u1.z_indexed_by } as *mut ())
                        };
                    }
                    if unsafe { (*p_item).fg.is_tab_func() } != 0 {
                        unsafe {
                            sqlite3_expr_list_delete(db,
                                unsafe { (*p_item).u1.p_func_arg })
                        };
                    }
                    sqlite3_delete_table(db, unsafe { (*p_item).p_s_tab });
                    if unsafe { (*p_item).fg.is_using() } != 0 {
                        sqlite3_id_list_delete(db, unsafe { (*p_item).u3.p_using });
                    } else if !(unsafe { (*p_item).u3.p_on }).is_null() {
                        unsafe {
                            sqlite3_expr_delete(db, unsafe { (*p_item).u3.p_on })
                        };
                    }
                    break '__c36;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_item;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        unsafe { sqlite3_db_nn_free_nn(db, p_list as *mut ()) };
    }
}

///* Create a new index for an SQL table.  pName1.pName2 is the name of the index
///* and pTblList is the name of the table that is to be indexed.  Both will
///* be NULL for a primary key or an index that is created to satisfy a
///* UNIQUE constraint.  If pTable and pIndex are NULL, use pParse->pNewTable
///* as the table to be indexed.  pParse->pNewTable is a table that is
///* currently being constructed by a CREATE TABLE statement.
///*
///* pList is a list of columns to be indexed.  pList will be NULL if this
///* is a primary key or unique-constraint on the most recent column added
///* to the table currently under construction.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_create_index(p_parse: *mut Parse,
    p_name1: *mut Token, p_name2: *mut Token, p_tbl_name: *mut SrcList,
    mut p_list: *mut ExprList, on_error: i32, p_start: Option<&Token>,
    mut p_pi_where: *mut Expr, sort_order: i32, if_not_exist: i32,
    idx_type: u8) -> () {
    unsafe {
        unsafe {
            let mut p_tab: *mut Table = core::ptr::null_mut();
            /// Table to be indexed
            let mut p_index: *mut Index = core::ptr::null_mut();
            /// The index to be created
            let mut z_name: *mut i8 = core::ptr::null_mut();
            /// Name of the index
            let mut n_name: i32 = 0;
            /// Number of characters in zName
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut s_fix: DbFixer = unsafe { core::mem::zeroed() };
            /// For assigning database names to pTable
            let mut sort_order_mask: i32 = 0;
            /// 1 to honor DESC in index.  0 to ignore.
            let mut db: *mut Sqlite3 = core::ptr::null_mut();
            let mut p_db: *const Db = core::ptr::null();
            /// The specific table containing the indexed database
            let mut i_db: i32 = 0;
            /// Index of the database that is being written
            let mut p_name: *mut Token = core::ptr::null_mut();
            /// Unqualified name of the index to create
            let mut p_list_item: *const ExprListItem = core::ptr::null();
            /// For looping over pList
            let mut n_extra: i32 = 0;
            /// Space allocated for zExtra[]
            let mut n_extra_col: i32 = 0;
            /// Number of extra columns needed
            let mut z_extra: *mut i8 = core::ptr::null_mut();
            /// Extra space after the Index object
            let mut p_pk: *mut Index = core::ptr::null_mut();
            /// PRIMARY KEY index for WITHOUT ROWID tables
            ///* Find the table that is to be indexed.  Return early if not found.
            /// Use the two-part index name to determine the database
            ///* to search for the table. 'Fix' the table name to this db
            ///* before looking up the table.
            /// If the index name was unqualified, check if the table
            ///* is a temp table. If so, set the database to 1. Do not do this
            ///* if initializing a database schema.
            /// Because the parser constructs pTblName from a single identifier,
            ///* sqlite3FixSrcList can never fail.
            ///* Find the name of the index.  Make sure there is not already another
            ///* index or table with the same name. 
            ///*
            ///* Exception:  If we are reading the names of permanent indices from the
            ///* sqlite_schema table (because some other process changed the schema) and
            ///* one of the index names collides with the name of a temporary table or
            ///* index, then we will continue to process this index.
            ///*
            ///* If pName==0 it means that we are
            ///* dealing with a primary key or UNIQUE constraint.  We have to invent our
            ///* own name.
            let mut n: i32 = 0;
            let mut p_loop: *const Index = core::ptr::null();
            /// Automatic index names generated from within sqlite3_declare_vtab()
            ///* must have names that are distinct from normal automatic index names.
            ///* The following statement converts "sqlite3_autoindex..." into
            ///* "sqlite3_butoindex..." in order to make the names distinct.
            ///* The "vtab_err.test" test demonstrates the need of this statement.
            /// Check for authorization to create an index.
            let mut z_db: *const i8 = core::ptr::null();
            /// If pList==0, it means this routine was called to make a primary
            ///* key out of the last column added to the table under construction.
            ///* So create a fake list to simulate this.
            let mut prev_col: Token = unsafe { core::mem::zeroed() };
            let mut p_col: *mut Column = core::ptr::null_mut();
            /// Figure out how many bytes of space are required to store explicitly
            ///* specified collation sequence names.
            let mut p_expr: *const Expr = core::ptr::null();
            ///* Allocate the index structure.
            /// Fits in i16
            /// Check to see if we should honor DESC requests on index columns
            /// Honor DESC
            /// Ignore DESC
            /// Analyze the list of expressions that form the terms of the index and
            ///* report any errors.  In the common case where the expression is exactly
            ///* a table column, store that column in aiColumn[].  For general expressions,
            ///* populate pIndex->aColExpr and store XN_EXPR (-2) in aiColumn[].
            ///*
            ///* TODO: Issue a warning if two or more columns of the index are identical.
            ///* TODO: Issue a warning if the table primary key is used as part of the
            ///* index key.
            let mut p_c_expr: *const Expr = core::ptr::null();
            /// The i-th index expression
            let mut requested_sort_order: i32 = 0;
            /// ASC or DESC on the i-th expression
            let mut z_coll: *const i8 = core::ptr::null();
            /// Collation sequence name
            let mut n_coll: i32 = 0;
            /// Append the table key to the end of the index.  For WITHOUT ROWID
            ///* tables (when pPk!=0) this will be the declared PRIMARY KEY.  For
            ///* normal tables (when pPk==0) this will be the rowid.
            let mut x: i32 = 0;
            /// If this index contains every column of its table, then mark
            ///* it as a covering index
            /// This routine has been called to create an automatic index as a
            ///* result of a PRIMARY KEY or UNIQUE clause on a column definition, or
            ///* a PRIMARY KEY or UNIQUE clause following the column definitions.
            ///* i.e. one of:
            ///*
            ///* CREATE TABLE t(x PRIMARY KEY, y);
            ///* CREATE TABLE t(x, y, UNIQUE(x, y));
            ///*
            ///* Either way, check to see if the table already has such an index. If
            ///* so, don't bother creating this one. This only applies to
            ///* automatically created indices. Users can do as they wish with
            ///* explicit indices.
            ///*
            ///* Two UNIQUE or PRIMARY KEY constraints are considered equivalent
            ///* (and thus suppressing the second one) even if they have different
            ///* sort orders.
            ///*
            ///* If there are different collating sequences or if the columns of
            ///* the constraint occur in different orders, then the constraints are
            ///* considered distinct and both result in separate indices.
            let mut p_idx: *mut Index = core::ptr::null_mut();
            let mut k: i32 = 0;
            let mut z1: *const i8 = core::ptr::null();
            let mut z2: *const i8 = core::ptr::null();
            /// This constraint creates the same index as a previous
            ///* constraint specified somewhere in the CREATE TABLE statement.
            ///* However the ON CONFLICT clauses are different. If both this
            ///* constraint and the previous equivalent constraint have explicit
            ///* ON CONFLICT clauses this is an error. Otherwise, use the
            ///* explicitly specified behavior for the index.
            /// Link the new Index structure to its table and to the other
            ///* in-memory database structures.
            let mut p: *const Index = core::ptr::null();
            /// Malloc must have failed
            /// If this is the initial CREATE INDEX statement (or CREATE TABLE if the
            ///* index is an implied index for a UNIQUE or PRIMARY KEY constraint) then
            ///* emit code to allocate the index rootpage on disk and make an entry for
            ///* the index in the sqlite_schema table and populate the index with
            ///* content.  But, do not do this if we are simply reading the sqlite_schema
            ///* table to parse the schema, or if this index is the PRIMARY KEY index
            ///* of a WITHOUT ROWID table.
            ///*
            ///* If pTblName==0 it means this index is generated as an implied PRIMARY KEY
            ///* or UNIQUE index in a CREATE TABLE statement.  Since the table
            ///* has just been created, it contains no data and the index initialization
            ///* step can be skipped.
            let mut v: *mut Vdbe = core::ptr::null_mut();
            let mut z_stmt: *mut i8 = core::ptr::null_mut();
            let mut i_mem: i32 = 0;
            /// Create the rootpage for the index using CreateIndex. But before
            ///* doing so, code a Noop instruction and store its address in
            ///* Index.tnum. This is required in case this index is actually a
            ///* PRIMARY KEY and the table is actually a WITHOUT ROWID table. In
            ///* that case the convertToWithoutRowidTable() routine will replace
            ///* the Noop with a Goto to jump over the VDBE code generated below.
            /// Gather the complete text of the CREATE INDEX statement into
            ///* the zStmt variable
            let mut n__1: i32 = 0;
            /// A named index with an explicit CREATE INDEX statement
            /// An automatic index created by a PRIMARY KEY or UNIQUE constraint */
            ///        /* zStmt = sqlite3MPrintf("");
            /// Add an entry in sqlite_schema for this index
            /// Fill the index with data and reparse the schema. Code an OP_Expire
            ///* to invalidate all pre-compiled statements.
            /// Clean up before exiting
            /// Ensure all REPLACE indexes on pTab are at the end of the pIndex list.
            ///* The list was already ordered when this routine was entered, so at this
            ///* point at most a single index (the newly added index) will be out of
            ///* order.  So we have to reorder at most one index.
            let mut pp_from: *mut *mut Index = core::ptr::null_mut();
            let mut p_this: *mut Index = core::ptr::null_mut();
            let mut p_next: *mut Index = core::ptr::null_mut();
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s38:
                    {
                    match __state {
                        0 => { p_tab = core::ptr::null_mut(); __state = 3; }
                        2 => {
                            if !(p_index).is_null() {
                                __state = 329;
                            } else { __state = 328; }
                        }
                        3 => { p_index = core::ptr::null_mut(); __state = 4; }
                        4 => { z_name = core::ptr::null_mut(); __state = 5; }
                        5 => { __state = 6; }
                        6 => { __state = 7; }
                        7 => { __state = 8; }
                        8 => { __state = 9; }
                        9 => { db = unsafe { (*p_parse).db }; __state = 10; }
                        10 => { __state = 11; }
                        11 => { __state = 12; }
                        12 => { p_name = core::ptr::null_mut(); __state = 13; }
                        13 => { __state = 14; }
                        14 => { n_extra = 0; __state = 15; }
                        15 => { __state = 16; }
                        16 => { z_extra = core::ptr::null_mut(); __state = 17; }
                        17 => { p_pk = core::ptr::null_mut(); __state = 18; }
                        18 => { { let _ = 0; }; __state = 19; }
                        19 => {
                            if unsafe { (*p_parse).n_err } != 0 {
                                __state = 21;
                            } else { __state = 20; }
                        }
                        20 => { { let _ = 0; }; __state = 22; }
                        21 => { __state = 2; }
                        22 => {
                            if unsafe { (*p_parse).e_parse_mode } as i32 == 1 &&
                                    idx_type as i32 != 2 {
                                __state = 24;
                            } else { __state = 23; }
                        }
                        23 => {
                            if 0 != unsafe { sqlite3_read_schema(p_parse) } {
                                __state = 26;
                            } else { __state = 25; }
                        }
                        24 => { __state = 2; }
                        25 => {
                            if sqlite3_has_explicit_nulls(p_parse, p_list) != 0 {
                                __state = 28;
                            } else { __state = 27; }
                        }
                        26 => { __state = 2; }
                        27 => {
                            if p_tbl_name != core::ptr::null_mut() {
                                __state = 30;
                            } else { __state = 31; }
                        }
                        28 => { __state = 2; }
                        29 => {
                            p_db =
                                unsafe { unsafe { (*db).a_db.offset(i_db as isize) } };
                            __state = 57;
                        }
                        30 => { { let _ = 0; }; __state = 32; }
                        31 => { { let _ = 0; }; __state = 52; }
                        32 => {
                            i_db =
                                sqlite3_two_part_name(p_parse, p_name1, p_name2,
                                    &mut p_name);
                            __state = 33;
                        }
                        33 => {
                            if i_db < 0 { __state = 35; } else { __state = 34; }
                        }
                        34 => { { let _ = 0; }; __state = 36; }
                        35 => { __state = 2; }
                        36 => {
                            if (unsafe { (*db).init.busy } == 0) as i32 != 0 {
                                __state = 38;
                            } else { __state = 37; }
                        }
                        37 => {
                            unsafe {
                                sqlite3_fix_init(&mut s_fix, p_parse, i_db,
                                    c"index".as_ptr() as *mut i8 as *const i8,
                                    p_name as *const Token)
                            };
                            __state = 41;
                        }
                        38 => {
                            p_tab =
                                unsafe { sqlite3_src_list_lookup(p_parse, p_tbl_name) };
                            __state = 39;
                        }
                        39 => {
                            if unsafe { (*p_name2).n } == 0 as u32 && !(p_tab).is_null()
                                    &&
                                    unsafe { (*p_tab).p_schema } ==
                                        unsafe {
                                            (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                                        } {
                                __state = 40;
                            } else { __state = 37; }
                        }
                        40 => { i_db = 1; __state = 37; }
                        41 => {
                            if unsafe { sqlite3_fix_src_list(&mut s_fix, p_tbl_name) }
                                    != 0 {
                                __state = 43;
                            } else { __state = 42; }
                        }
                        42 => {
                            p_tab =
                                sqlite3_locate_table_item(p_parse, 0 as u32,
                                    unsafe {
                                        &*(unsafe { (*p_tbl_name).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)
                                    });
                            __state = 44;
                        }
                        43 => { { let _ = 0; }; __state = 42; }
                        44 => { { let _ = 0; }; __state = 45; }
                        45 => {
                            if p_tab == core::ptr::null_mut() {
                                __state = 47;
                            } else { __state = 46; }
                        }
                        46 => {
                            if i_db == 1 &&
                                    unsafe {
                                            (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                        } != unsafe { (*p_tab).p_schema } {
                                __state = 49;
                            } else { __state = 48; }
                        }
                        47 => { __state = 2; }
                        48 => {
                            if !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32)
                                        as i32 != 0 {
                                __state = 51;
                            } else { __state = 29; }
                        }
                        49 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"cannot create a TEMP index on non-TEMP table \"%s\"".as_ptr()
                                            as *mut i8 as *const i8, unsafe { (*p_tab).z_name })
                            };
                            __state = 50;
                        }
                        50 => { __state = 2; }
                        51 => {
                            p_pk = sqlite3_primary_key_index(unsafe { &*p_tab });
                            __state = 29;
                        }
                        52 => { { let _ = 0; }; __state = 53; }
                        53 => {
                            p_tab = unsafe { (*p_parse).p_new_table };
                            __state = 54;
                        }
                        54 => {
                            if (p_tab).is_null() as i32 != 0 {
                                __state = 56;
                            } else { __state = 55; }
                        }
                        55 => {
                            i_db =
                                unsafe {
                                    sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                                };
                            __state = 29;
                        }
                        56 => { __state = 2; }
                        57 => { { let _ = 0; }; __state = 58; }
                        58 => {
                            if unsafe {
                                                sqlite3_strnicmp(unsafe { (*p_tab).z_name } as *const i8,
                                                    c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
                                            } == 0 && unsafe { (*db).init.busy } as i32 == 0 &&
                                    p_tbl_name != core::ptr::null_mut() {
                                __state = 60;
                            } else { __state = 59; }
                        }
                        59 => {
                            if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                                __state = 63;
                            } else { __state = 62; }
                        }
                        60 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"table %s may not be indexed".as_ptr() as *mut i8 as
                                        *const i8, unsafe { (*p_tab).z_name })
                            };
                            __state = 61;
                        }
                        61 => { __state = 2; }
                        62 => {
                            if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                                __state = 66;
                            } else { __state = 65; }
                        }
                        63 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"views may not be indexed".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                            __state = 64;
                        }
                        64 => { __state = 2; }
                        65 => {
                            if !(p_name).is_null() {
                                __state = 69;
                            } else { __state = 70; }
                        }
                        66 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"virtual tables may not be indexed".as_ptr() as *mut i8 as
                                        *const i8)
                            };
                            __state = 67;
                        }
                        67 => { __state = 2; }
                        68 => {
                            if !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                    != 0 {
                                __state = 99;
                            } else { __state = 98; }
                        }
                        69 => {
                            z_name =
                                sqlite3_name_from_token(db, p_name as *const Token);
                            __state = 71;
                        }
                        70 => { __state = 88; }
                        71 => {
                            if z_name == core::ptr::null_mut() {
                                __state = 73;
                            } else { __state = 72; }
                        }
                        72 => { { let _ = 0; }; __state = 74; }
                        73 => { __state = 2; }
                        74 => {
                            if 0 !=
                                    sqlite3_check_object_name(p_parse, z_name as *const i8,
                                        c"index".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab).z_name } as *const i8) {
                                __state = 76;
                            } else { __state = 75; }
                        }
                        75 => {
                            if !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                    != 0 {
                                __state = 77;
                            } else { __state = 68; }
                        }
                        76 => { __state = 2; }
                        77 => {
                            if (unsafe { (*db).init.busy } == 0) as i32 != 0 {
                                __state = 79;
                            } else { __state = 78; }
                        }
                        78 => {
                            if sqlite3_find_index(db, z_name as *const i8,
                                        unsafe { (*p_db).z_db_s_name } as *const i8) !=
                                    core::ptr::null_mut() {
                                __state = 82;
                            } else { __state = 68; }
                        }
                        79 => {
                            if sqlite3_find_table(unsafe { &*db }, z_name as *const i8,
                                        unsafe { (*p_db).z_db_s_name } as *const i8) !=
                                    core::ptr::null_mut() {
                                __state = 80;
                            } else { __state = 78; }
                        }
                        80 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"there is already a table named %s".as_ptr() as *mut i8 as
                                        *const i8, z_name)
                            };
                            __state = 81;
                        }
                        81 => { __state = 2; }
                        82 => {
                            if (if_not_exist == 0) as i32 != 0 {
                                __state = 84;
                            } else { __state = 85; }
                        }
                        83 => { __state = 2; }
                        84 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"index %s already exists".as_ptr() as *mut i8 as *const i8,
                                    z_name)
                            };
                            __state = 83;
                        }
                        85 => { { let _ = 0; }; __state = 86; }
                        86 => {
                            sqlite3_code_verify_schema(p_parse, i_db);
                            __state = 87;
                        }
                        87 => {
                            sqlite3_force_not_read_only(p_parse);
                            __state = 83;
                        }
                        88 => { __state = 89; }
                        89 => {
                            { p_loop = unsafe { (*p_tab).p_index }; n = 1 };
                            __state = 91;
                        }
                        90 => {
                            z_name =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"sqlite_autoindex_%s_%d".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab).z_name }, n)
                                };
                            __state = 94;
                        }
                        91 => {
                            if !(p_loop).is_null() {
                                __state = 92;
                            } else { __state = 90; }
                        }
                        92 => { __state = 93; }
                        93 => {
                            {
                                p_loop = unsafe { (*p_loop).p_next };
                                { let __p = &mut n; let __t = *__p; *__p += 1; __t }
                            };
                            __state = 91;
                        }
                        94 => {
                            if z_name == core::ptr::null_mut() {
                                __state = 96;
                            } else { __state = 95; }
                        }
                        95 => {
                            if unsafe { (*p_parse).e_parse_mode } as i32 != 0 {
                                __state = 97;
                            } else { __state = 68; }
                        }
                        96 => { __state = 2; }
                        97 => {
                            {
                                let __p = unsafe { &mut *z_name.offset(7 as isize) };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                            __state = 68;
                        }
                        98 => {
                            if p_list == core::ptr::null_mut() {
                                __state = 108;
                            } else { __state = 109; }
                        }
                        99 => {
                            z_db = unsafe { (*p_db).z_db_s_name } as *const i8;
                            __state = 100;
                        }
                        100 => {
                            if unsafe {
                                        sqlite3_auth_check(p_parse, 18,
                                            if (0 == 0) as i32 != 0 && i_db == 1 {
                                                    c"sqlite_temp_master".as_ptr() as *mut i8
                                                } else { c"sqlite_master".as_ptr() as *mut i8 } as
                                                *const i8, core::ptr::null(), z_db)
                                    } != 0 {
                                __state = 102;
                            } else { __state = 101; }
                        }
                        101 => { i = 1; __state = 103; }
                        102 => { __state = 2; }
                        103 => {
                            if (0 == 0) as i32 != 0 && i_db == 1 {
                                __state = 105;
                            } else { __state = 104; }
                        }
                        104 => {
                            if unsafe {
                                        sqlite3_auth_check(p_parse, i, z_name as *const i8,
                                            unsafe { (*p_tab).z_name } as *const i8, z_db)
                                    } != 0 {
                                __state = 106;
                            } else { __state = 98; }
                        }
                        105 => { i = 3; __state = 104; }
                        106 => { __state = 2; }
                        107 => { i = 0; __state = 121; }
                        108 => { __state = 110; }
                        109 => {
                            unsafe {
                                sqlite3_expr_list_check_length(p_parse, p_list,
                                    c"index".as_ptr() as *mut i8 as *const i8)
                            };
                            __state = 118;
                        }
                        110 => {
                            p_col =
                                unsafe {
                                    unsafe {
                                        (*p_tab).a_col.offset((unsafe { (*p_tab).n_col } as i32 - 1)
                                                as isize)
                                    }
                                };
                            __state = 111;
                        }
                        111 => {
                            unsafe { (*p_col).col_flags |= 8 as u16 };
                            __state = 112;
                        }
                        112 => {
                            unsafe {
                                sqlite3_token_init(&mut prev_col,
                                    unsafe { (*p_col).z_cn_name })
                            };
                            __state = 113;
                        }
                        113 => {
                            p_list =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse, core::ptr::null_mut(),
                                        unsafe {
                                            sqlite3_expr_alloc(db, 60,
                                                &raw mut prev_col as *const Token, 0)
                                        })
                                };
                            __state = 114;
                        }
                        114 => {
                            if p_list == core::ptr::null_mut() {
                                __state = 116;
                            } else { __state = 115; }
                        }
                        115 => { { let _ = 0; }; __state = 117; }
                        116 => { __state = 2; }
                        117 => {
                            unsafe {
                                sqlite3_expr_list_set_sort_order(p_list, sort_order, -1)
                            };
                            __state = 107;
                        }
                        118 => {
                            if unsafe { (*p_parse).n_err } != 0 {
                                __state = 119;
                            } else { __state = 107; }
                        }
                        119 => { __state = 2; }
                        120 => {
                            n_name = unsafe { sqlite3_strlen30(z_name as *const i8) };
                            __state = 128;
                        }
                        121 => {
                            if i < unsafe { (*p_list).n_expr } {
                                __state = 122;
                            } else { __state = 120; }
                        }
                        122 => {
                            p_expr =
                                unsafe {
                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                    } as *const Expr;
                            __state = 124;
                        }
                        123 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 121;
                        }
                        124 => { { let _ = 0; }; __state = 125; }
                        125 => {
                            if unsafe { (*p_expr).op } as i32 == 114 {
                                __state = 126;
                            } else { __state = 123; }
                        }
                        126 => { { let _ = 0; }; __state = 127; }
                        127 => {
                            n_extra +=
                                1 +
                                    unsafe {
                                        sqlite3_strlen30(unsafe { (*p_expr).u.z_token } as
                                                *const i8)
                                    };
                            __state = 123;
                        }
                        128 => {
                            n_extra_col =
                                if !(p_pk).is_null() {
                                    (unsafe { (*p_pk).n_key_col }) as i32
                                } else { 1 };
                            __state = 129;
                        }
                        129 => { { let _ = 0; }; __state = 130; }
                        130 => {
                            p_index =
                                sqlite3_allocate_index_object(db,
                                    unsafe { (*p_list).n_expr } + n_extra_col,
                                    n_name + n_extra + 1, &mut z_extra);
                            __state = 131;
                        }
                        131 => {
                            if unsafe { (*db).malloc_failed } != 0 {
                                __state = 133;
                            } else { __state = 132; }
                        }
                        132 => { { let _ = 0; }; __state = 134; }
                        133 => { __state = 2; }
                        134 => { { let _ = 0; }; __state = 135; }
                        135 => {
                            unsafe { (*p_index).z_name = z_extra };
                            __state = 136;
                        }
                        136 => {
                            {
                                let __n = n_name + 1;
                                let __p = &mut z_extra;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            __state = 137;
                        }
                        137 => {
                            unsafe {
                                memcpy(unsafe { (*p_index).z_name } as *mut (),
                                    z_name as *const (), (n_name + 1) as u64)
                            };
                            __state = 138;
                        }
                        138 => {
                            unsafe { (*p_index).p_table = p_tab };
                            __state = 139;
                        }
                        139 => {
                            unsafe { (*p_index).on_error = on_error as u8 };
                            __state = 140;
                        }
                        140 => {
                            unsafe {
                                (*p_index).set_uniq_not_null((on_error != 0) as u32 as u32)
                            };
                            __state = 141;
                        }
                        141 => {
                            unsafe { (*p_index).set_idx_type(idx_type as u32 as u32) };
                            __state = 142;
                        }
                        142 => {
                            unsafe {
                                (*p_index).p_schema =
                                    unsafe {
                                        (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                    }
                            };
                            __state = 143;
                        }
                        143 => {
                            unsafe {
                                (*p_index).n_key_col = unsafe { (*p_list).n_expr } as u16
                            };
                            __state = 144;
                        }
                        144 => {
                            if !(p_pi_where).is_null() {
                                __state = 146;
                            } else { __state = 145; }
                        }
                        145 => { { let _ = 0; }; __state = 149; }
                        146 => {
                            unsafe {
                                sqlite3_resolve_self_reference(p_parse, p_tab, 2,
                                    p_pi_where, core::ptr::null_mut())
                            };
                            __state = 147;
                        }
                        147 => {
                            unsafe { (*p_index).p_part_idx_where = p_pi_where };
                            __state = 148;
                        }
                        148 => {
                            p_pi_where = core::ptr::null_mut();
                            __state = 145;
                        }
                        149 => {
                            if unsafe { (*unsafe { (*p_db).p_schema }).file_format } as
                                        i32 >= 4 {
                                __state = 151;
                            } else { __state = 152; }
                        }
                        150 => {
                            p_list_item =
                                unsafe { (*p_list).a.as_ptr() } as *mut ExprListItem;
                            __state = 153;
                        }
                        151 => { sort_order_mask = -1; __state = 150; }
                        152 => { sort_order_mask = 0; __state = 150; }
                        153 => {
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                __state = 155;
                            } else { __state = 154; }
                        }
                        154 => { i = 0; __state = 158; }
                        155 => {
                            unsafe { (*p_index).a_col_expr = p_list };
                            __state = 156;
                        }
                        156 => { p_list = core::ptr::null_mut(); __state = 154; }
                        157 => {
                            if !(p_pk).is_null() {
                                __state = 210;
                            } else { __state = 211; }
                        }
                        158 => {
                            if i < unsafe { (*p_index).n_key_col } as i32 {
                                __state = 159;
                            } else { __state = 157; }
                        }
                        159 => { __state = 161; }
                        160 => {
                            {
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                {
                                    let __p = &mut p_list_item;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                }
                            };
                            __state = 158;
                        }
                        161 => { __state = 162; }
                        162 => { __state = 163; }
                        163 => {
                            sqlite3_string_to_id(unsafe {
                                    &mut *unsafe { (*p_list_item).p_expr }
                                });
                            __state = 164;
                        }
                        164 => {
                            unsafe {
                                sqlite3_resolve_self_reference(p_parse, p_tab, 32,
                                    unsafe { (*p_list_item).p_expr }, core::ptr::null_mut())
                            };
                            __state = 165;
                        }
                        165 => {
                            if unsafe { (*p_parse).n_err } != 0 {
                                __state = 167;
                            } else { __state = 166; }
                        }
                        166 => {
                            p_c_expr =
                                unsafe {
                                    sqlite3_expr_skip_collate(unsafe { (*p_list_item).p_expr })
                                };
                            __state = 168;
                        }
                        167 => { __state = 2; }
                        168 => {
                            if unsafe { (*p_c_expr).op } as i32 != 168 {
                                __state = 170;
                            } else { __state = 171; }
                        }
                        169 => { z_coll = core::ptr::null(); __state = 190; }
                        170 => {
                            if p_tab == unsafe { (*p_parse).p_new_table } {
                                __state = 173;
                            } else { __state = 172; }
                        }
                        171 => {
                            j = unsafe { (*p_c_expr).i_column } as i32;
                            __state = 181;
                        }
                        172 => {
                            if unsafe { (*p_index).a_col_expr } == core::ptr::null_mut()
                                {
                                __state = 176;
                            } else { __state = 175; }
                        }
                        173 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"expressions prohibited in PRIMARY KEY and UNIQUE constraints".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                            __state = 174;
                        }
                        174 => { __state = 2; }
                        175 => { j = -2; __state = 178; }
                        176 => {
                            unsafe { (*p_index).a_col_expr = p_list };
                            __state = 177;
                        }
                        177 => { p_list = core::ptr::null_mut(); __state = 175; }
                        178 => {
                            unsafe {
                                *unsafe { (*p_index).ai_column.offset(i as isize) } =
                                    -2 as i16
                            };
                            __state = 179;
                        }
                        179 => {
                            unsafe { (*p_index).set_uniq_not_null(0 as u32 as u32) };
                            __state = 180;
                        }
                        180 => {
                            unsafe { (*p_index).set_b_has_expr(1 as u32 as u32) };
                            __state = 169;
                        }
                        181 => { { let _ = 0; }; __state = 182; }
                        182 => {
                            if j < 0 { __state = 184; } else { __state = 185; }
                        }
                        183 => {
                            unsafe {
                                *unsafe { (*p_index).ai_column.offset(i as isize) } =
                                    j as i16
                            };
                            __state = 169;
                        }
                        184 => {
                            j = unsafe { (*p_tab).i_p_key } as i32;
                            __state = 183;
                        }
                        185 => {
                            if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(j as isize) }).not_null()
                                        } as i32 == 0 {
                                __state = 187;
                            } else { __state = 186; }
                        }
                        186 => {
                            if unsafe {
                                                (*unsafe { (*p_tab).a_col.offset(j as isize) }).col_flags
                                            } as i32 & 32 != 0 {
                                __state = 188;
                            } else { __state = 183; }
                        }
                        187 => {
                            unsafe { (*p_index).set_uniq_not_null(0 as u32 as u32) };
                            __state = 186;
                        }
                        188 => {
                            unsafe { (*p_index).set_b_has_v_col(1 as u32 as u32) };
                            __state = 189;
                        }
                        189 => {
                            unsafe { (*p_index).set_b_has_expr(1 as u32 as u32) };
                            __state = 183;
                        }
                        190 => {
                            if unsafe { (*unsafe { (*p_list_item).p_expr }).op } as i32
                                    == 114 {
                                __state = 192;
                            } else { __state = 193; }
                        }
                        191 => {
                            if (z_coll).is_null() as i32 != 0 {
                                __state = 204;
                            } else { __state = 203; }
                        }
                        192 => { __state = 194; }
                        193 => {
                            if j >= 0 { __state = 202; } else { __state = 191; }
                        }
                        194 => { { let _ = 0; }; __state = 195; }
                        195 => {
                            z_coll =
                                unsafe { (*unsafe { (*p_list_item).p_expr }).u.z_token } as
                                    *const i8;
                            __state = 196;
                        }
                        196 => {
                            n_coll = unsafe { sqlite3_strlen30(z_coll) } + 1;
                            __state = 197;
                        }
                        197 => { { let _ = 0; }; __state = 198; }
                        198 => {
                            unsafe {
                                memcpy(z_extra as *mut (), z_coll as *const (),
                                    n_coll as u64)
                            };
                            __state = 199;
                        }
                        199 => { z_coll = z_extra as *const i8; __state = 200; }
                        200 => {
                            {
                                let __n = n_coll;
                                let __p = &mut z_extra;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            __state = 201;
                        }
                        201 => { n_extra -= n_coll; __state = 191; }
                        202 => {
                            z_coll =
                                sqlite3_column_coll(unsafe {
                                        &*unsafe { (*p_tab).a_col.offset(j as isize) }
                                    });
                            __state = 191;
                        }
                        203 => {
                            if (unsafe { (*db).init.busy } == 0) as i32 != 0 &&
                                    (unsafe {
                                                        sqlite3_locate_coll_seq(p_parse, z_coll)
                                                    }).is_null() as i32 != 0 {
                                __state = 206;
                            } else { __state = 205; }
                        }
                        204 => {
                            z_coll = sqlite3_str_binary.as_ptr() as *const i8;
                            __state = 203;
                        }
                        205 => {
                            unsafe {
                                *unsafe { (*p_index).az_coll.offset(i as isize) } = z_coll
                            };
                            __state = 207;
                        }
                        206 => { __state = 2; }
                        207 => {
                            requested_sort_order =
                                unsafe { (*p_list_item).fg.sort_flags } as i32 &
                                    sort_order_mask;
                            __state = 208;
                        }
                        208 => {
                            unsafe {
                                *unsafe { (*p_index).a_sort_order.offset(i as isize) } =
                                    requested_sort_order as u8
                            };
                            __state = 160;
                        }
                        209 => {
                            sqlite3_default_row_est(unsafe { &*p_index });
                            __state = 225;
                        }
                        210 => { j = 0; __state = 213; }
                        211 => {
                            unsafe {
                                *unsafe { (*p_index).ai_column.offset(i as isize) } =
                                    -1 as i16
                            };
                            __state = 224;
                        }
                        212 => { { let _ = 0; }; __state = 209; }
                        213 => {
                            if j < unsafe { (*p_pk).n_key_col } as i32 {
                                __state = 214;
                            } else { __state = 212; }
                        }
                        214 => {
                            x =
                                unsafe { *unsafe { (*p_pk).ai_column.offset(j as isize) } }
                                    as i32;
                            __state = 216;
                        }
                        215 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 213;
                        }
                        216 => { { let _ = 0; }; __state = 217; }
                        217 => {
                            if is_dup_column(unsafe { &*p_index },
                                        unsafe { (*p_index).n_key_col } as i32, unsafe { &*p_pk },
                                        j) != 0 {
                                __state = 218;
                            } else { __state = 219; }
                        }
                        218 => {
                            {
                                let __p = unsafe { &mut (*p_index).n_column };
                                let __t = *__p;
                                *__p -= 1;
                                __t
                            };
                            __state = 215;
                        }
                        219 => { __state = 220; }
                        220 => {
                            unsafe {
                                *unsafe { (*p_index).ai_column.offset(i as isize) } =
                                    x as i16
                            };
                            __state = 221;
                        }
                        221 => {
                            unsafe {
                                *unsafe { (*p_index).az_coll.offset(i as isize) } =
                                    unsafe { *unsafe { (*p_pk).az_coll.offset(j as isize) } }
                            };
                            __state = 222;
                        }
                        222 => {
                            unsafe {
                                *unsafe { (*p_index).a_sort_order.offset(i as isize) } =
                                    unsafe {
                                        *unsafe { (*p_pk).a_sort_order.offset(j as isize) }
                                    }
                            };
                            __state = 223;
                        }
                        223 => {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            __state = 215;
                        }
                        224 => {
                            unsafe {
                                *unsafe { (*p_index).az_coll.offset(i as isize) } =
                                    sqlite3_str_binary.as_ptr() as *const i8
                            };
                            __state = 209;
                        }
                        225 => {
                            if unsafe { (*p_parse).p_new_table } ==
                                    core::ptr::null_mut() {
                                __state = 227;
                            } else { __state = 226; }
                        }
                        226 => { { let _ = 0; }; __state = 228; }
                        227 => {
                            estimate_index_width(unsafe { &mut *p_index });
                            __state = 226;
                        }
                        228 => {
                            recompute_columns_not_indexed(unsafe { &mut *p_index });
                            __state = 229;
                        }
                        229 => {
                            if p_tbl_name != core::ptr::null_mut() &&
                                    unsafe { (*p_index).n_column } as i32 >=
                                        unsafe { (*p_tab).n_col } as i32 {
                                __state = 231;
                            } else { __state = 230; }
                        }
                        230 => {
                            if p_tab == unsafe { (*p_parse).p_new_table } {
                                __state = 242;
                            } else { __state = 241; }
                        }
                        231 => {
                            unsafe { (*p_index).set_is_covering(1 as u32 as u32) };
                            __state = 232;
                        }
                        232 => { j = 0; __state = 233; }
                        233 => {
                            if j < unsafe { (*p_tab).n_col } as i32 {
                                __state = 234;
                            } else { __state = 230; }
                        }
                        234 => {
                            if j == unsafe { (*p_tab).i_p_key } as i32 {
                                __state = 237;
                            } else { __state = 236; }
                        }
                        235 => {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            __state = 233;
                        }
                        236 => {
                            if sqlite3_table_column_to_index(unsafe { &*p_index }, j) >=
                                    0 {
                                __state = 239;
                            } else { __state = 238; }
                        }
                        237 => { __state = 235; }
                        238 => {
                            unsafe { (*p_index).set_is_covering(0 as u32 as u32) };
                            __state = 240;
                        }
                        239 => { __state = 235; }
                        240 => { __state = 230; }
                        241 => {
                            if !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32
                                    != 0 {
                                __state = 278;
                            } else { __state = 277; }
                        }
                        242 => { __state = 243; }
                        243 => {
                            p_idx = unsafe { (*p_tab).p_index };
                            __state = 244;
                        }
                        244 => {
                            if !(p_idx).is_null() {
                                __state = 245;
                            } else { __state = 241; }
                        }
                        245 => { __state = 247; }
                        246 => {
                            p_idx = unsafe { (*p_idx).p_next };
                            __state = 244;
                        }
                        247 => { { let _ = 0; }; __state = 248; }
                        248 => { { let _ = 0; }; __state = 249; }
                        249 => { { let _ = 0; }; __state = 250; }
                        250 => {
                            if unsafe { (*p_idx).n_key_col } as i32 !=
                                    unsafe { (*p_index).n_key_col } as i32 {
                                __state = 252;
                            } else { __state = 251; }
                        }
                        251 => { k = 0; __state = 254; }
                        252 => { __state = 246; }
                        253 => {
                            if k == unsafe { (*p_idx).n_key_col } as i32 {
                                __state = 265;
                            } else { __state = 246; }
                        }
                        254 => {
                            if k < unsafe { (*p_idx).n_key_col } as i32 {
                                __state = 255;
                            } else { __state = 253; }
                        }
                        255 => { __state = 257; }
                        256 => {
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                            __state = 254;
                        }
                        257 => { __state = 258; }
                        258 => { { let _ = 0; }; __state = 259; }
                        259 => {
                            if unsafe {
                                            *unsafe { (*p_idx).ai_column.offset(k as isize) }
                                        } as i32 !=
                                    unsafe {
                                            *unsafe { (*p_index).ai_column.offset(k as isize) }
                                        } as i32 {
                                __state = 261;
                            } else { __state = 260; }
                        }
                        260 => {
                            z1 =
                                unsafe { *unsafe { (*p_idx).az_coll.offset(k as isize) } };
                            __state = 262;
                        }
                        261 => { __state = 253; }
                        262 => {
                            z2 =
                                unsafe {
                                    *unsafe { (*p_index).az_coll.offset(k as isize) }
                                };
                            __state = 263;
                        }
                        263 => {
                            if unsafe { sqlite3_str_i_cmp(z1, z2) } != 0 {
                                __state = 264;
                            } else { __state = 256; }
                        }
                        264 => { __state = 253; }
                        265 => {
                            if unsafe { (*p_idx).on_error } as i32 !=
                                    unsafe { (*p_index).on_error } as i32 {
                                __state = 267;
                            } else { __state = 266; }
                        }
                        266 => {
                            if idx_type as i32 == 2 {
                                __state = 272;
                            } else { __state = 271; }
                        }
                        267 => {
                            if !(unsafe { (*p_idx).on_error } as i32 == 11 ||
                                                unsafe { (*p_index).on_error } as i32 == 11) as i32 != 0 {
                                __state = 269;
                            } else { __state = 268; }
                        }
                        268 => {
                            if unsafe { (*p_idx).on_error } as i32 == 11 {
                                __state = 270;
                            } else { __state = 266; }
                        }
                        269 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"conflicting ON CONFLICT clauses specified".as_ptr() as
                                            *mut i8 as *const i8, 0)
                            };
                            __state = 268;
                        }
                        270 => {
                            unsafe {
                                (*p_idx).on_error = unsafe { (*p_index).on_error }
                            };
                            __state = 266;
                        }
                        271 => {
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                __state = 274;
                            } else { __state = 273; }
                        }
                        272 => {
                            unsafe { (*p_idx).set_idx_type(idx_type as u32 as u32) };
                            __state = 271;
                        }
                        273 => { __state = 2; }
                        274 => {
                            unsafe {
                                (*p_index).p_next = unsafe { (*p_parse).p_new_index }
                            };
                            __state = 275;
                        }
                        275 => {
                            unsafe { (*p_parse).p_new_index = p_index };
                            __state = 276;
                        }
                        276 => { p_index = core::ptr::null_mut(); __state = 273; }
                        277 => {
                            if unsafe { (*db).init.busy } != 0 ||
                                    p_tbl_name == core::ptr::null_mut() {
                                __state = 321;
                            } else { __state = 322; }
                        }
                        278 => { { let _ = 0; }; __state = 279; }
                        279 => {
                            if unsafe { (*db).init.busy } != 0 {
                                __state = 280;
                            } else { __state = 281; }
                        }
                        280 => { __state = 282; }
                        281 => {
                            if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 ||
                                    p_tbl_name != core::ptr::null_mut() {
                                __state = 296;
                            } else { __state = 277; }
                        }
                        282 => { { let _ = 0; }; __state = 283; }
                        283 => { { let _ = 0; }; __state = 284; }
                        284 => {
                            if p_tbl_name != core::ptr::null_mut() {
                                __state = 286;
                            } else { __state = 285; }
                        }
                        285 => {
                            p =
                                unsafe {
                                        sqlite3_hash_insert(unsafe {
                                                &mut (*unsafe { (*p_index).p_schema }).idx_hash
                                            }, unsafe { (*p_index).z_name } as *const i8,
                                            p_index as *mut ())
                                    } as *mut Index;
                            __state = 291;
                        }
                        286 => {
                            unsafe { (*p_index).tnum = unsafe { (*db).init.new_tnum } };
                            __state = 287;
                        }
                        287 => {
                            if unsafe { sqlite3_index_has_duplicate_root_page(p_index) }
                                    != 0 {
                                __state = 288;
                            } else { __state = 285; }
                        }
                        288 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"invalid rootpage".as_ptr() as *mut i8 as *const i8)
                            };
                            __state = 289;
                        }
                        289 => {
                            unsafe {
                                (*p_parse).rc = unsafe { sqlite3_corrupt_error(4412) }
                            };
                            __state = 290;
                        }
                        290 => { __state = 2; }
                        291 => {
                            if !(p).is_null() { __state = 293; } else { __state = 292; }
                        }
                        292 => {
                            unsafe { (*db).m_db_flags |= 1 as u32 };
                            __state = 277;
                        }
                        293 => { { let _ = 0; }; __state = 294; }
                        294 => { unsafe { sqlite3_oom_fault(db) }; __state = 295; }
                        295 => { __state = 2; }
                        296 => { __state = 297; }
                        297 => { __state = 298; }
                        298 => {
                            i_mem =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                };
                            __state = 299;
                        }
                        299 => {
                            v = unsafe { sqlite3_get_vdbe(p_parse) };
                            __state = 300;
                        }
                        300 => {
                            if v == core::ptr::null_mut() {
                                __state = 302;
                            } else { __state = 301; }
                        }
                        301 => {
                            sqlite3_begin_write_operation(p_parse, 1, i_db);
                            __state = 303;
                        }
                        302 => { __state = 2; }
                        303 => {
                            unsafe {
                                (*p_index).tnum =
                                    unsafe { sqlite3_vdbe_add_op0(v, 189) } as Pgno
                            };
                            __state = 304;
                        }
                        304 => {
                            unsafe { sqlite3_vdbe_add_op3(v, 149, i_db, i_mem, 2) };
                            __state = 305;
                        }
                        305 => { { let _ = 0; }; __state = 306; }
                        306 => {
                            if p_start.is_some() {
                                __state = 308;
                            } else { __state = 309; }
                        }
                        307 => {
                            unsafe {
                                sqlite3_nested_parse(p_parse,
                                    c"INSERT INTO %Q.sqlite_master VALUES(\'index\',%Q,%Q,#%d,%Q);".as_ptr()
                                            as *mut i8 as *const i8,
                                    unsafe {
                                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                    }, unsafe { (*p_index).z_name }, unsafe { (*p_tab).z_name },
                                    i_mem, z_stmt)
                            };
                            __state = 313;
                        }
                        308 => {
                            n__1 =
                                (unsafe {
                                                        unsafe {
                                                            (*p_parse).s_last_token.z.offset_from(unsafe {
                                                                    (*p_name).z
                                                                })
                                                        }
                                                    } as i64 as i32 as u32 +
                                        unsafe { (*p_parse).s_last_token.n }) as i32;
                            __state = 310;
                        }
                        309 => { z_stmt = core::ptr::null_mut(); __state = 307; }
                        310 => {
                            if unsafe {
                                            *unsafe { (*p_name).z.offset((n__1 - 1) as isize) }
                                        } as i32 == ';' as i32 {
                                __state = 312;
                            } else { __state = 311; }
                        }
                        311 => {
                            z_stmt =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"CREATE%s INDEX %.*s".as_ptr() as *mut i8 as *const i8,
                                        if on_error == 0 {
                                            c"".as_ptr() as *mut i8
                                        } else { c" UNIQUE".as_ptr() as *mut i8 }, n__1,
                                        unsafe { (*p_name).z })
                                };
                            __state = 307;
                        }
                        312 => {
                            { let __p = &mut n__1; let __t = *__p; *__p -= 1; __t };
                            __state = 311;
                        }
                        313 => {
                            unsafe { sqlite3_db_free(db, z_stmt as *mut ()) };
                            __state = 314;
                        }
                        314 => {
                            if !(p_tbl_name).is_null() {
                                __state = 316;
                            } else { __state = 315; }
                        }
                        315 => {
                            unsafe {
                                sqlite3_vdbe_jump_here(v, unsafe { (*p_index).tnum } as i32)
                            };
                            __state = 277;
                        }
                        316 => {
                            sqlite3_refill_index(p_parse, p_index, i_mem);
                            __state = 317;
                        }
                        317 => {
                            sqlite3_change_cookie(unsafe { &*p_parse }, i_db);
                            __state = 318;
                        }
                        318 => {
                            unsafe {
                                sqlite3_vdbe_add_parse_schema_op(v, i_db,
                                    unsafe {
                                        sqlite3_m_printf(db,
                                            c"name=\'%q\' AND type=\'index\'".as_ptr() as *mut i8 as
                                                *const i8, unsafe { (*p_index).z_name })
                                    }, 0 as u16)
                            };
                            __state = 319;
                        }
                        319 => {
                            unsafe { sqlite3_vdbe_add_op2(v, 168, 0, 1) };
                            __state = 315;
                        }
                        320 => { __state = 2; }
                        321 => {
                            unsafe { (*p_index).p_next = unsafe { (*p_tab).p_index } };
                            __state = 323;
                        }
                        322 => {
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                __state = 325;
                            } else { __state = 320; }
                        }
                        323 => {
                            unsafe { (*p_tab).p_index = p_index };
                            __state = 324;
                        }
                        324 => { p_index = core::ptr::null_mut(); __state = 320; }
                        325 => { { let _ = 0; }; __state = 326; }
                        326 => {
                            unsafe { (*p_parse).p_new_index = p_index };
                            __state = 327;
                        }
                        327 => { p_index = core::ptr::null_mut(); __state = 320; }
                        328 => {
                            if !(p_tab).is_null() {
                                __state = 331;
                            } else { __state = 330; }
                        }
                        329 => { sqlite3_free_index(db, p_index); __state = 328; }
                        330 => {
                            unsafe { sqlite3_expr_delete(db, p_pi_where) };
                            __state = 345;
                        }
                        331 => { __state = 332; }
                        332 => { __state = 333; }
                        333 => {
                            pp_from = unsafe { &mut (*p_tab).p_index };
                            __state = 334;
                        }
                        334 => {
                            if { p_this = unsafe { *pp_from }; p_this } !=
                                    core::ptr::null_mut() {
                                __state = 335;
                            } else { __state = 330; }
                        }
                        335 => { __state = 337; }
                        336 => {
                            pp_from = unsafe { &mut (*p_this).p_next };
                            __state = 334;
                        }
                        337 => {
                            if unsafe { (*p_this).on_error } as i32 != 5 {
                                __state = 339;
                            } else { __state = 338; }
                        }
                        338 => {
                            if { p_next = unsafe { (*p_this).p_next }; p_next } !=
                                        core::ptr::null_mut() &&
                                    unsafe { (*p_next).on_error } as i32 != 5 {
                                __state = 341;
                            } else { __state = 340; }
                        }
                        339 => { __state = 336; }
                        340 => { __state = 330; }
                        341 => { unsafe { *pp_from = p_next }; __state = 342; }
                        342 => {
                            unsafe { (*p_this).p_next = unsafe { (*p_next).p_next } };
                            __state = 343;
                        }
                        343 => {
                            unsafe { (*p_next).p_next = p_this };
                            __state = 344;
                        }
                        344 => {
                            pp_from = unsafe { &mut (*p_next).p_next };
                            __state = 338;
                        }
                        345 => {
                            unsafe { sqlite3_expr_list_delete(db, p_list) };
                            __state = 346;
                        }
                        346 => {
                            sqlite3_src_list_delete(db, p_tbl_name);
                            __state = 347;
                        }
                        347 => {
                            unsafe { sqlite3_db_free(db, z_name as *mut ()) };
                            __state = 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

///* Designate the PRIMARY KEY for the table.  pList is a list of names
///* of columns that form the primary key.  If pList is NULL, then the
///* most recently added column of the table is the primary key.
///*
///* A table can have at most one primary key.  If the table already has
///* a primary key (and this is the second primary key) then create an
///* error.
///*
///* If the PRIMARY KEY is on a single column whose datatype is INTEGER,
///* then we will try to use that column as the rowid.  Set the Table.iPKey
///* field of the table under construction to be the index of the
///* INTEGER PRIMARY KEY column.  Table.iPKey is set to -1 if there is
///* no INTEGER PRIMARY KEY.
///*
///* If the key is not an INTEGER PRIMARY KEY, then create a unique
///* index for the key.  No index is created for INTEGER PRIMARY KEYs.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_add_primary_key(p_parse: *mut Parse,
    mut p_list: *mut ExprList, on_error: i32, auto_inc: i32, sort_order: i32)
    -> () {
    unsafe {
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut p_col: *mut Column = core::ptr::null_mut();
        let mut i_col: i32 = 0;
        let mut i: i32 = 0;
        let mut n_term: i32 = 0;
        let mut p_c_expr: *mut Expr = core::ptr::null_mut();
        let mut p_c_expr_1: *const Expr = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s40:
                {
                match __state {
                    0 => {
                        p_tab = unsafe { (*p_parse).p_new_table };
                        __state = 3;
                    }
                    2 => {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse).db }, p_list)
                        };
                        __state = 46;
                    }
                    3 => { p_col = core::ptr::null_mut(); __state = 4; }
                    4 => { i_col = -1; __state = 5; }
                    5 => { __state = 6; }
                    6 => {
                        if p_tab == core::ptr::null_mut() {
                            __state = 8;
                        } else { __state = 7; }
                    }
                    7 => {
                        if unsafe { (*p_tab).tab_flags } & 4 as u32 != 0 {
                            __state = 10;
                        } else { __state = 9; }
                    }
                    8 => { __state = 2; }
                    9 => {
                        unsafe { (*p_tab).tab_flags |= 4 as u32 };
                        __state = 12;
                    }
                    10 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"table \"%s\" has more than one primary key".as_ptr() as
                                        *mut i8 as *const i8, unsafe { (*p_tab).z_name })
                        };
                        __state = 11;
                    }
                    11 => { __state = 2; }
                    12 => {
                        if p_list == core::ptr::null_mut() {
                            __state = 14;
                        } else { __state = 15; }
                    }
                    13 => {
                        if n_term == 1 && !(p_col).is_null() &&
                                    unsafe { (*p_col).e_c_type() } as i32 == 4 &&
                                sort_order != 1 {
                            __state = 32;
                        } else { __state = 33; }
                    }
                    14 => {
                        i_col = unsafe { (*p_tab).n_col } as i32 - 1;
                        __state = 16;
                    }
                    15 => {
                        n_term = unsafe { (*p_list).n_expr };
                        __state = 19;
                    }
                    16 => {
                        p_col =
                            unsafe { unsafe { (*p_tab).a_col.offset(i_col as isize) } };
                        __state = 17;
                    }
                    17 => {
                        make_column_part_of_primary_key(p_parse,
                            unsafe { &mut *p_col });
                        __state = 18;
                    }
                    18 => { n_term = 1; __state = 13; }
                    19 => { i = 0; __state = 20; }
                    20 => {
                        if i < n_term { __state = 21; } else { __state = 13; }
                    }
                    21 => {
                        p_c_expr =
                            unsafe {
                                sqlite3_expr_skip_collate(unsafe {
                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                    })
                            };
                        __state = 23;
                    }
                    22 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 20;
                    }
                    23 => { { let _ = 0; }; __state = 24; }
                    24 => {
                        sqlite3_string_to_id(unsafe { &mut *p_c_expr });
                        __state = 25;
                    }
                    25 => {
                        if unsafe { (*p_c_expr).op } as i32 == 60 {
                            __state = 26;
                        } else { __state = 22; }
                    }
                    26 => { { let _ = 0; }; __state = 27; }
                    27 => {
                        i_col =
                            unsafe {
                                sqlite3_column_index(p_tab,
                                    unsafe { (*p_c_expr).u.z_token } as *const i8)
                            };
                        __state = 28;
                    }
                    28 => {
                        if i_col >= 0 { __state = 29; } else { __state = 22; }
                    }
                    29 => {
                        p_col =
                            unsafe { unsafe { (*p_tab).a_col.offset(i_col as isize) } };
                        __state = 30;
                    }
                    30 => {
                        make_column_part_of_primary_key(p_parse,
                            unsafe { &mut *p_col });
                        __state = 22;
                    }
                    31 => { __state = 2; }
                    32 => {
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 &&
                                !(p_list).is_null() {
                            __state = 35;
                        } else { __state = 34; }
                    }
                    33 => {
                        if auto_inc != 0 { __state = 43; } else { __state = 44; }
                    }
                    34 => {
                        unsafe { (*p_tab).i_p_key = i_col as i16 };
                        __state = 37;
                    }
                    35 => {
                        p_c_expr_1 =
                            unsafe {
                                    sqlite3_expr_skip_collate(unsafe {
                                            (*(unsafe { (*p_list).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        })
                                } as *const Expr;
                        __state = 36;
                    }
                    36 => {
                        unsafe {
                            sqlite3_rename_token_remap(p_parse,
                                unsafe { &raw mut (*p_tab).i_p_key } as *const (),
                                p_c_expr_1 as *const ())
                        };
                        __state = 34;
                    }
                    37 => {
                        unsafe { (*p_tab).key_conf = on_error as u8 };
                        __state = 38;
                    }
                    38 => { { let _ = 0; }; __state = 39; }
                    39 => {
                        unsafe { (*p_tab).tab_flags |= (auto_inc * 8) as u32 };
                        __state = 40;
                    }
                    40 => {
                        if !(p_list).is_null() {
                            __state = 42;
                        } else { __state = 41; }
                    }
                    41 => {
                        { let _ = sqlite3_has_explicit_nulls(p_parse, p_list); };
                        __state = 31;
                    }
                    42 => {
                        unsafe {
                            (*p_parse).i_pk_sort_order =
                                unsafe {
                                    (*(unsafe { (*p_list).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).fg.sort_flags
                                }
                        };
                        __state = 41;
                    }
                    43 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"AUTOINCREMENT is only allowed on an INTEGER PRIMARY KEY".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 31;
                    }
                    44 => {
                        sqlite3_create_index(p_parse, core::ptr::null_mut(),
                            core::ptr::null_mut(), core::ptr::null_mut(), p_list,
                            on_error, None, core::ptr::null_mut(), sort_order, 0,
                            2 as u8);
                        __state = 45;
                    }
                    45 => { p_list = core::ptr::null_mut(); __state = 31; }
                    46 => { return; }
                    _ => {}
                }
            }
        }
    }
}

///* Add a new CHECK constraint to the table currently under construction.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_add_check_constraint(p_parse: *mut Parse,
    p_check_expr: *mut Expr, mut z_start: *const i8, mut z_end: *const i8)
    -> () {
    unsafe {
        unsafe {
            let p_tab: *mut Table = unsafe { (*p_parse).p_new_table };
            let db: *const Sqlite3 =
                unsafe { (*p_parse).db } as *const Sqlite3;
            if !(p_tab).is_null() &&
                        !(unsafe { (*p_parse).e_parse_mode } as i32 == 1) as i32 !=
                            0 &&
                    (unsafe {
                                    sqlite3_btree_is_readonly(unsafe {
                                            (*unsafe {
                                                        (*db).a_db.add(unsafe { (*db).init.i_db } as usize)
                                                    }).p_bt
                                        })
                                } == 0) as i32 != 0 {
                unsafe {
                    (*p_tab).p_check =
                        unsafe {
                            sqlite3_expr_list_append(p_parse,
                                unsafe { (*p_tab).p_check }, p_check_expr)
                        }
                };
                { let _ = 0; };
                if unsafe { (*p_parse).u1.cr.constraint_name.n } != 0 {
                    unsafe {
                        sqlite3_expr_list_set_name(p_parse,
                            unsafe { (*p_tab).p_check },
                            unsafe { &raw mut (*p_parse).u1.cr.constraint_name } as
                                *const Token, 1)
                    };
                } else {
                    let mut t: Token = unsafe { core::mem::zeroed() };
                    {
                        {
                            let __p = &mut z_start;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        '__b41: loop {
                            if !(unsafe {
                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                    *const u8).add(unsafe { *z_start.offset(0 as isize) } as u8
                                                                    as usize)
                                                    } as i32 & 1 != 0) {
                                break '__b41;
                            }
                            '__c41: loop { break '__c41; }
                            {
                                let __p = &mut z_start;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                    }
                    while unsafe {
                                        *(sqlite3_ctype_map.as_ptr() as
                                                    *const u8).add(unsafe { *z_end.offset(-1 as isize) } as u8
                                                    as usize)
                                    } as i32 & 1 != 0 {
                        {
                            let __p = &mut z_end;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(-1) };
                            __t
                        };
                    }
                    t.z = z_start;
                    t.n =
                        unsafe { z_end.offset_from(t.z) } as i64 as i32 as u32;
                    unsafe {
                        sqlite3_expr_list_set_name(p_parse,
                            unsafe { (*p_tab).p_check }, &raw mut t as *const Token, 1)
                    };
                }
            } else {
                unsafe {
                    sqlite3_expr_delete(unsafe { (*p_parse).db }, p_check_expr)
                };
            }
        }
    }
}

///* The expression is the default value for the most recently added column
///* of the table currently under construction.
///*
///* Default value expressions must be constant.  Raise an exception if this
///* is not the case.
///*
///* This routine is called by the parser while in the middle of
///* parsing a CREATE TABLE statement.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_add_default_value(p_parse: *mut Parse,
    p_expr: *mut Expr, z_start: *const i8, z_end: *const i8) -> () {
    unsafe {
        let mut p: *mut Table = core::ptr::null_mut();
        let mut p_col: *mut Column = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        p = unsafe { (*p_parse).p_new_table };
        if p != core::ptr::null_mut() {
            let is_init: i32 =
                (unsafe { (*db).init.busy } != 0 &&
                        unsafe { (*db).init.i_db } as i32 != 1) as i32;
            p_col =
                unsafe {
                    unsafe {
                        (*p).a_col.offset((unsafe { (*p).n_col } as i32 - 1) as
                                isize)
                    }
                };
            if (unsafe {
                                sqlite3_expr_is_constant_or_function(p_expr, is_init as u8)
                            } == 0) as i32 != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"default value of column [%s] is not constant".as_ptr() as
                                *mut i8 as *const i8, unsafe { (*p_col).z_cn_name })
                };
            } else if unsafe { (*p_col).col_flags } as i32 & 96 != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"cannot use DEFAULT on a generated column".as_ptr() as
                                *mut i8 as *const i8)
                };
            } else {
                /// A copy of pExpr is used instead of the original, as pExpr contains
                ///* tokens that point to volatile memory.
                let mut x: Expr = unsafe { core::mem::zeroed() };
                let mut p_dflt_expr: *mut Expr = core::ptr::null_mut();
                unsafe {
                    memset(&raw mut x as *mut (), 0,
                        core::mem::size_of::<Expr>() as u64)
                };
                x.op = 181 as u8;
                x.u.z_token =
                    unsafe { sqlite3_db_span_dup(db, z_start, z_end) };
                x.p_left = p_expr;
                x.flags = 8192 as u32;
                p_dflt_expr =
                    unsafe {
                        sqlite3_expr_dup(db, &raw mut x as *const Expr, 1)
                    };
                unsafe { sqlite3_db_free(db, x.u.z_token as *mut ()) };
                sqlite3_column_set_expr(p_parse, unsafe { &mut *p },
                    unsafe { &mut *p_col }, p_dflt_expr);
            }
        }
        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
            unsafe { sqlite3_rename_expr_unmap(p_parse, p_expr) };
        }
        unsafe { sqlite3_expr_delete(db, p_expr) };
    }
}

///* Set the collation function of the most recently parsed table column
///* to the CollSeq given.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_add_collate_type(p_parse: *mut Parse,
    p_token: *mut Token) -> () {
    let mut p: *const Table = core::ptr::null();
    let mut i: i32 = 0;
    let mut z_coll: *mut i8 = core::ptr::null_mut();
    /// Dequoted name of collation sequence
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    if { p = unsafe { (*p_parse).p_new_table }; p } == core::ptr::null_mut()
            || unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
        return;
    }
    i = unsafe { (*p).n_col } as i32 - 1;
    db = unsafe { (*p_parse).db };
    z_coll = sqlite3_name_from_token(db, p_token as *const Token);
    if (z_coll).is_null() as i32 != 0 { return; }
    if !(unsafe {
                        sqlite3_locate_coll_seq(p_parse, z_coll as *const i8)
                    }).is_null() {
        let mut p_idx: *const Index = core::ptr::null();
        sqlite3_column_set_coll(db,
            unsafe { &mut *unsafe { (*p).a_col.offset(i as isize) } },
            z_coll as *const i8);
        {
            p_idx = unsafe { (*p).p_index };
            '__b43: loop {
                if !(!(p_idx).is_null()) { break '__b43; }
                '__c43: loop {
                    { let _ = 0; };
                    if unsafe {
                                    *unsafe { (*p_idx).ai_column.offset(0 as isize) }
                                } as i32 == i {
                        unsafe {
                            *unsafe { (*p_idx).az_coll.offset(0 as isize) } =
                                sqlite3_column_coll(unsafe {
                                        &*unsafe { (*p).a_col.offset(i as isize) }
                                    })
                        };
                    }
                    break '__c43;
                }
                p_idx = unsafe { (*p_idx).p_next };
            }
        }
    }
    unsafe { sqlite3_db_free(db, z_coll as *mut ()) };
}

/// Change the most recently parsed column to be a GENERATED ALWAYS AS
///* column.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_add_generated(p_parse: *mut Parse,
    mut p_expr: *mut Expr, p_type: *mut Token) -> () {
    unsafe {
        let mut e_type: u8 = 0 as u8;
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut p_col: *mut Column = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s45:
                {
                match __state {
                    0 => { e_type = 32 as u8; __state = 4; }
                    2 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"error in generated column \"%s\"".as_ptr() as *mut i8 as
                                    *const i8, unsafe { (*p_col).z_cn_name })
                        };
                        __state = 36;
                    }
                    3 => {
                        unsafe {
                            sqlite3_expr_delete(unsafe { (*p_parse).db }, p_expr)
                        };
                        __state = 1;
                    }
                    4 => {
                        p_tab = unsafe { (*p_parse).p_new_table };
                        __state = 5;
                    }
                    5 => { __state = 6; }
                    6 => {
                        if p_tab == core::ptr::null_mut() {
                            __state = 8;
                        } else { __state = 7; }
                    }
                    7 => {
                        p_col =
                            unsafe {
                                unsafe {
                                    (*p_tab).a_col.offset((unsafe { (*p_tab).n_col } as i32 - 1)
                                            as isize)
                                }
                            };
                        __state = 9;
                    }
                    8 => { __state = 3; }
                    9 => {
                        if unsafe { (*p_parse).e_parse_mode } as i32 == 1 {
                            __state = 11;
                        } else { __state = 10; }
                    }
                    10 => {
                        if unsafe { (*p_col).i_dflt } as i32 > 0 {
                            __state = 14;
                        } else { __state = 13; }
                    }
                    11 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"virtual tables cannot use computed columns".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 12;
                    }
                    12 => { __state = 3; }
                    13 => {
                        if !(p_type).is_null() {
                            __state = 16;
                        } else { __state = 15; }
                    }
                    14 => { __state = 2; }
                    15 => {
                        if e_type as i32 == 32 {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    16 => {
                        if unsafe { (*p_type).n } == 7 as u32 &&
                                unsafe {
                                        sqlite3_strnicmp(c"virtual".as_ptr() as *mut i8 as
                                                *const i8, unsafe { (*p_type).z }, 7)
                                    } == 0 {
                            __state = 17;
                        } else { __state = 18; }
                    }
                    17 => { __state = 15; }
                    18 => {
                        if unsafe { (*p_type).n } == 6 as u32 &&
                                unsafe {
                                        sqlite3_strnicmp(c"stored".as_ptr() as *mut i8 as *const i8,
                                            unsafe { (*p_type).z }, 6)
                                    } == 0 {
                            __state = 19;
                        } else { __state = 20; }
                    }
                    19 => { e_type = 64 as u8; __state = 15; }
                    20 => { __state = 2; }
                    21 => {
                        unsafe { (*p_col).col_flags |= e_type as i32 as u16 };
                        __state = 23;
                    }
                    22 => {
                        {
                            let __p = unsafe { &mut (*p_tab).n_nv_col };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 21;
                    }
                    23 => { { let _ = 0; }; __state = 24; }
                    24 => { { let _ = 0; }; __state = 25; }
                    25 => {
                        unsafe { (*p_tab).tab_flags |= e_type as u32 };
                        __state = 26;
                    }
                    26 => {
                        if unsafe { (*p_col).col_flags } as i32 & 1 != 0 {
                            __state = 28;
                        } else { __state = 27; }
                    }
                    27 => {
                        if !(p_expr).is_null() &&
                                unsafe { (*p_expr).op } as i32 == 60 {
                            __state = 30;
                        } else { __state = 29; }
                    }
                    28 => {
                        make_column_part_of_primary_key(p_parse,
                            unsafe { &mut *p_col });
                        __state = 27;
                    }
                    29 => {
                        if !(p_expr).is_null() &&
                                unsafe { (*p_expr).op } as i32 != 72 {
                            __state = 32;
                        } else { __state = 31; }
                    }
                    30 => {
                        p_expr =
                            unsafe {
                                sqlite3_p_expr(p_parse, 173, p_expr, core::ptr::null_mut())
                            };
                        __state = 29;
                    }
                    31 => {
                        sqlite3_column_set_expr(p_parse, unsafe { &mut *p_tab },
                            unsafe { &mut *p_col }, p_expr);
                        __state = 33;
                    }
                    32 => {
                        unsafe {
                            (*p_expr).aff_expr = unsafe { (*p_col).affinity }
                        };
                        __state = 31;
                    }
                    33 => { p_expr = core::ptr::null_mut(); __state = 34; }
                    34 => { __state = 3; }
                    35 => { __state = 2; }
                    36 => { __state = 3; }
                    _ => {}
                }
            }
        }
    }
}

///* Resize an Index object to hold N columns total.  Return SQLITE_OK
///* on success and SQLITE_NOMEM on an OOM error.
#[allow(unused_doc_comments)]
extern "C" fn resize_index_object(p_parse_1: &Parse, p_idx_1: &mut Index,
    n_1: i32) -> i32 {
    let mut z_extra: *mut i8 = core::ptr::null_mut();
    let mut n_byte: u64 = 0 as u64;
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    if (*p_idx_1).n_column as i32 >= n_1 { return 0; }
    db = (*p_parse_1).db;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    n_byte =
        (core::mem::size_of::<*mut i8>() as u64 +
                        core::mem::size_of::<LogEst>() as u64 +
                    core::mem::size_of::<i16>() as u64 + 1 as u64) * n_1 as u64;
    z_extra = unsafe { sqlite3_db_malloc_zero(db, n_byte) } as *mut i8;
    if z_extra == core::ptr::null_mut() { return 7; }
    unsafe {
        memcpy(z_extra as *mut (), (*p_idx_1).az_coll as *const (),
            core::mem::size_of::<*mut i8>() as u64 *
                (*p_idx_1).n_column as u64)
    };
    (*p_idx_1).az_coll = z_extra as *mut *const i8;
    {
        let __n = core::mem::size_of::<*mut i8>() as u64 * n_1 as u64;
        let __p = &mut z_extra;
        *__p = unsafe { (*__p).add(__n as usize) };
    };
    unsafe {
        memcpy(z_extra as *mut (), (*p_idx_1).ai_row_log_est as *const (),
            core::mem::size_of::<LogEst>() as u64 *
                ((*p_idx_1).n_key_col as i32 + 1) as u64)
    };
    (*p_idx_1).ai_row_log_est = z_extra as *mut LogEst;
    {
        let __n = core::mem::size_of::<LogEst>() as u64 * n_1 as u64;
        let __p = &mut z_extra;
        *__p = unsafe { (*__p).add(__n as usize) };
    };
    unsafe {
        memcpy(z_extra as *mut (), (*p_idx_1).ai_column as *const (),
            core::mem::size_of::<i16>() as u64 * (*p_idx_1).n_column as u64)
    };
    (*p_idx_1).ai_column = z_extra as *mut i16;
    {
        let __n = core::mem::size_of::<i16>() as u64 * n_1 as u64;
        let __p = &mut z_extra;
        *__p = unsafe { (*__p).add(__n as usize) };
    };
    unsafe {
        memcpy(z_extra as *mut (), (*p_idx_1).a_sort_order as *const (),
            (*p_idx_1).n_column as u64)
    };
    (*p_idx_1).a_sort_order = z_extra as *mut u8;
    (*p_idx_1).n_column = n_1 as u16;

    /// See tag-20250221-1 above for proof of safety
    (*p_idx_1).set_is_resized(1 as u32 as u32);
    return 0;
}

/// Return true if column number x is any of the first nCol entries of aiCol[].
///* This is used to determine if the column number x appears in any of the
///* first nCol entries of an index.
extern "C" fn has_column(mut ai_col_1: *const i16, mut n_col_1: i32, x: i32)
    -> i32 {
    while { let __p = &mut n_col_1; let __t = *__p; *__p -= 1; __t } > 0 {
        if x ==
                unsafe {
                        *{
                                let __p = &mut ai_col_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                    } as i32 {
            return 1;
        }
    }
    return 0;
}

///* This routine runs at the end of parsing a CREATE TABLE statement that
///* has a WITHOUT ROWID clause.  The job of this routine is to convert both
///* internal schema data structures and the generated VDBE code so that they
///* are appropriate for a WITHOUT ROWID table instead of a rowid table.
///* Changes include:
///*
///*     (1)  Set all columns of the PRIMARY KEY schema object to be NOT NULL.
///*     (2)  Convert P3 parameter of the OP_CreateBtree from BTREE_INTKEY
///*          into BTREE_BLOBKEY.
///*     (3)  Bypass the creation of the sqlite_schema table entry
///*          for the PRIMARY KEY as the primary key index is now
///*          identified by the sqlite_schema table entry of the table itself.
///*     (4)  Set the Index.tnum of the PRIMARY KEY Index object in the
///*          schema to the rootpage from the main table.
///*     (5)  Add all table columns to the PRIMARY KEY Index object
///*          so that the PRIMARY KEY is a covering index.  The surplus
///*          columns are part of KeyInfo.nAllField and are not used for
///*          sorting or lookup or uniqueness checks.
///*     (6)  Replace the rowid tail on all automatically generated UNIQUE
///*          indices with the PRIMARY KEY columns.
///*
///* For virtual tables, only (1) is performed.
#[allow(unused_doc_comments)]
extern "C" fn convert_to_without_rowid_table(p_parse_1: *mut Parse,
    p_tab_1: *mut Table) -> () {
    unsafe {
        unsafe {
            let mut p_idx: *mut Index = core::ptr::null_mut();
            let mut p_pk: *mut Index = core::ptr::null_mut();
            let mut n_pk: i32 = 0;
            let mut n_extra: i32 = 0;
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
            let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
            if (unsafe { (*db).init.imposter_table() } == 0) as i32 != 0 {
                {
                    i = 0;
                    '__b47: loop {
                        if !(i < unsafe { (*p_tab_1).n_col } as i32) {
                            break '__b47;
                        }
                        '__c47: loop {
                            if unsafe {
                                                    (*unsafe { (*p_tab_1).a_col.offset(i as isize) }).col_flags
                                                } as i32 & 1 != 0 &&
                                    unsafe {
                                                (*unsafe { (*p_tab_1).a_col.offset(i as isize) }).not_null()
                                            } as i32 == 0 {
                                unsafe {
                                    (*unsafe {
                                                (*p_tab_1).a_col.offset(i as isize)
                                            }).set_not_null(2 as u32 as u32)
                                };
                            }
                            break '__c47;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { (*p_tab_1).tab_flags |= 2048 as u32 };
            }

            /// Convert the P3 operand of the OP_CreateBtree opcode from BTREE_INTKEY
            ///* into BTREE_BLOBKEY.
            { let _ = 0; };
            if unsafe { (*p_parse_1).u1.cr.addr_cr_tab } != 0 {
                { let _ = 0; };
                unsafe {
                    sqlite3_vdbe_change_p3(v,
                        unsafe { (*p_parse_1).u1.cr.addr_cr_tab }, 2)
                };
            }
            if unsafe { (*p_tab_1).i_p_key } as i32 >= 0 {
                let mut p_list: *mut ExprList = core::ptr::null_mut();
                let mut ipk_token: Token = unsafe { core::mem::zeroed() };
                unsafe {
                    sqlite3_token_init(&mut ipk_token,
                        unsafe {
                            (*unsafe {
                                        (*p_tab_1).a_col.offset(unsafe { (*p_tab_1).i_p_key } as
                                                isize)
                                    }).z_cn_name
                        })
                };
                p_list =
                    unsafe {
                        sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                            unsafe {
                                sqlite3_expr_alloc(db, 60,
                                    &raw mut ipk_token as *const Token, 0)
                            })
                    };
                if p_list == core::ptr::null_mut() {
                    unsafe { (*p_tab_1).tab_flags &= !128 as u32 };
                    return;
                }
                if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
                    unsafe {
                        sqlite3_rename_token_remap(p_parse_1,
                            unsafe {
                                    (*(unsafe { (*p_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(0 as isize)).p_expr
                                } as *const (),
                            unsafe { &raw mut (*p_tab_1).i_p_key } as *const ())
                    };
                }
                unsafe {
                    (*(unsafe { (*p_list).a.as_ptr() } as
                                            *mut ExprListItem).offset(0 as isize)).fg.sort_flags =
                        unsafe { (*p_parse_1).i_pk_sort_order }
                };
                { let _ = 0; };
                unsafe { (*p_tab_1).i_p_key = -1 as i16 };
                sqlite3_create_index(p_parse_1, core::ptr::null_mut(),
                    core::ptr::null_mut(), core::ptr::null_mut(), p_list,
                    unsafe { (*p_tab_1).key_conf } as i32, None,
                    core::ptr::null_mut(), 0, 0, 2 as u8);
                if unsafe { (*p_parse_1).n_err } != 0 {
                    unsafe { (*p_tab_1).tab_flags &= !128 as u32 };
                    return;
                }
                { let _ = 0; };
                p_pk = sqlite3_primary_key_index(unsafe { &*p_tab_1 });
                { let _ = 0; };
            } else {
                p_pk = sqlite3_primary_key_index(unsafe { &*p_tab_1 });
                { let _ = 0; };
                {
                    i = { j = 1; j };
                    '__b48: loop {
                        if !(i < unsafe { (*p_pk).n_key_col } as i32) {
                            break '__b48;
                        }
                        '__c48: loop {
                            if is_dup_column(unsafe { &*p_pk }, j, unsafe { &*p_pk }, i)
                                    != 0 {
                                {
                                    let __p = unsafe { &mut (*p_pk).n_column };
                                    let __t = *__p;
                                    *__p -= 1;
                                    __t
                                };
                            } else {
                                unsafe {
                                    *unsafe { (*p_pk).az_coll.offset(j as isize) } =
                                        unsafe { *unsafe { (*p_pk).az_coll.offset(i as isize) } }
                                };
                                unsafe {
                                    *unsafe { (*p_pk).a_sort_order.offset(j as isize) } =
                                        unsafe {
                                            *unsafe { (*p_pk).a_sort_order.offset(i as isize) }
                                        }
                                };
                                unsafe {
                                    *unsafe {
                                                (*p_pk).ai_column.offset({
                                                            let __p = &mut j;
                                                            let __t = *__p;
                                                            *__p += 1;
                                                            __t
                                                        } as isize)
                                            } =
                                        unsafe { *unsafe { (*p_pk).ai_column.offset(i as isize) } }
                                };
                            }
                            break '__c48;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { (*p_pk).n_key_col = j as u16 };
            }
            { let _ = 0; };
            unsafe { (*p_pk).set_is_covering(1 as u32 as u32) };
            if (unsafe { (*db).init.imposter_table() } == 0) as i32 != 0 {
                unsafe { (*p_pk).set_uniq_not_null(1 as u32 as u32) };
            }
            n_pk =
                {
                        let __v = unsafe { (*p_pk).n_key_col };
                        unsafe { (*p_pk).n_column = __v };
                        __v
                    } as i32;
            if !(v).is_null() && unsafe { (*p_pk).tnum } > 0 as u32 {
                { let _ = 0; };
                unsafe {
                    sqlite3_vdbe_change_opcode(v,
                        unsafe { (*p_pk).tnum } as i32, 9 as u8)
                };
            }

            /// The root page of the PRIMARY KEY is the table root page
            unsafe { (*p_pk).tnum = unsafe { (*p_tab_1).tnum } };
            {
                p_idx = unsafe { (*p_tab_1).p_index };
                '__b49: loop {
                    if !(!(p_idx).is_null()) { break '__b49; }
                    '__c49: loop {
                        let mut n: i32 = 0;
                        if unsafe { (*p_idx).idx_type() } as i32 == 2 {
                            break '__c49;
                        }
                        {
                            i = { n = 0; n };
                            '__b50: loop {
                                if !(i < n_pk) { break '__b50; }
                                '__c50: loop {
                                    if (is_dup_column(unsafe { &*p_idx },
                                                        unsafe { (*p_idx).n_key_col } as i32, unsafe { &*p_pk }, i)
                                                    == 0) as i32 != 0 {
                                        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                                    }
                                    break '__c50;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if n == 0 {

                            /// This index is a superset of the primary key
                            unsafe {
                                (*p_idx).n_column = unsafe { (*p_idx).n_key_col }
                            };
                            break '__c49;
                        }
                        if resize_index_object(unsafe { &*p_parse_1 },
                                    unsafe { &mut *p_idx },
                                    unsafe { (*p_idx).n_key_col } as i32 + n) != 0 {
                            return;
                        }
                        {
                            { i = 0; j = unsafe { (*p_idx).n_key_col } as i32 };
                            '__b51: loop {
                                if !(i < n_pk) { break '__b51; }
                                '__c51: loop {
                                    if (is_dup_column(unsafe { &*p_idx },
                                                        unsafe { (*p_idx).n_key_col } as i32, unsafe { &*p_pk }, i)
                                                    == 0) as i32 != 0 {
                                        unsafe {
                                            *unsafe { (*p_idx).ai_column.offset(j as isize) } =
                                                unsafe { *unsafe { (*p_pk).ai_column.offset(i as isize) } }
                                        };
                                        unsafe {
                                            *unsafe { (*p_idx).az_coll.offset(j as isize) } =
                                                unsafe { *unsafe { (*p_pk).az_coll.offset(i as isize) } }
                                        };
                                        if unsafe {
                                                    *unsafe { (*p_pk).a_sort_order.offset(i as isize) }
                                                } != 0 {

                                            /// See ticket https://sqlite.org/src/info/bba7b69f9849b5bf
                                            unsafe { (*p_idx).set_b_asc_key_bug(1 as u32 as u32) };
                                        }
                                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                    }
                                    break '__c51;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        { let _ = 0; };
                        { let _ = 0; };
                        break '__c49;
                    }
                    p_idx = unsafe { (*p_idx).p_next };
                }
            }

            /// Add all table columns to the PRIMARY KEY index
            (n_extra = 0);
            {
                i = 0;
                '__b52: loop {
                    if !(i < unsafe { (*p_tab_1).n_col } as i32) {
                        break '__b52;
                    }
                    '__c52: loop {
                        if (has_column(unsafe { (*p_pk).ai_column } as *const i16,
                                                n_pk, i) == 0) as i32 != 0 &&
                                unsafe {
                                                (*unsafe { (*p_tab_1).a_col.offset(i as isize) }).col_flags
                                            } as i32 & 32 == 0 {
                            { let __p = &mut n_extra; let __t = *__p; *__p += 1; __t };
                        }
                        break '__c52;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if resize_index_object(unsafe { &*p_parse_1 },
                        unsafe { &mut *p_pk }, n_pk + n_extra) != 0 {
                return;
            }
            {
                { i = 0; j = n_pk };
                '__b53: loop {
                    if !(i < unsafe { (*p_tab_1).n_col } as i32) {
                        break '__b53;
                    }
                    '__c53: loop {
                        if (has_column(unsafe { (*p_pk).ai_column } as *const i16,
                                                j, i) == 0) as i32 != 0 &&
                                unsafe {
                                                (*unsafe { (*p_tab_1).a_col.offset(i as isize) }).col_flags
                                            } as i32 & 32 == 0 {
                            let z_coll: *const i8 =
                                sqlite3_column_coll(unsafe {
                                        &*unsafe { (*p_tab_1).a_col.offset(i as isize) }
                                    });
                            { let _ = 0; };
                            unsafe {
                                *unsafe { (*p_pk).ai_column.offset(j as isize) } = i as i16
                            };
                            unsafe {
                                *unsafe { (*p_pk).az_coll.offset(j as isize) } =
                                    if !(z_coll).is_null() {
                                        z_coll
                                    } else { sqlite3_str_binary.as_ptr() as *const i8 }
                            };
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                        break '__c53;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            { let _ = 0; };
            { let _ = 0; };
            recompute_columns_not_indexed(unsafe { &mut *p_pk });
        }
    }
}

///* Estimate the total row width for a table.
extern "C" fn estimate_table_width(p_tab_1: &mut Table) -> () {
    let mut w_table: u32 = 0 as u32;
    let mut p_tab_col: *const Column = core::ptr::null();
    let mut i: i32 = 0;
    {
        {
            i = (*p_tab_1).n_col as i32;
            p_tab_col = (*p_tab_1).a_col as *const Column
        };
        '__b54: loop {
            if !(i > 0) { break '__b54; }
            '__c54: loop {
                w_table += unsafe { (*p_tab_col).sz_est } as u32;
                break '__c54;
            }
            {
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                {
                    let __p = &mut p_tab_col;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    if ((*p_tab_1).i_p_key as i32) < 0 {
        { let __p = &mut w_table; let __t = *__p; *__p += 1; __t };
    }
    (*p_tab_1).sz_tab_row =
        unsafe { sqlite3_log_est((w_table * 4 as u32) as u64) };
}

///* Measure the number of characters needed to output the given
///* identifier.  The number returned includes any quotes used
///* but does not include the null terminator.
///*
///* The estimate is conservative.  It might be larger that what is
///* really needed.
extern "C" fn ident_length(mut z: *const i8) -> i64 {
    let mut n: i64 = 0 as i64;
    {
        n = 0 as i64;
        '__b55: loop {
            if !(unsafe { *z } != 0) { break '__b55; }
            '__c55: loop {
                if unsafe { *z } as i32 == '\"' as i32 {
                    { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                }
                break '__c55;
            }
            {
                { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                {
                    let __p = &mut z;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    return n + 2 as i64;
}

///* The first parameter is a pointer to an output buffer. The second
///* parameter is a pointer to an integer that contains the offset at
///* which to write into the output buffer. This function copies the
///* nul-terminated string pointed to by the third parameter, zSignedIdent,
///* to the specified offset in the buffer and updates *pIdx to refer
///* to the first byte after the last byte written before returning.
///*
///* If the string zSignedIdent consists entirely of alphanumeric
///* characters, does not begin with a digit and is not an SQL keyword,
///* then it is copied to the output buffer exactly as it is. Otherwise,
///* it is quoted using double-quotes.
extern "C" fn ident_put(z: *mut i8, p_idx_1: &mut i32,
    z_signed_ident_1: *mut i8) -> () {
    unsafe {
        let z_ident: *const u8 = z_signed_ident_1 as *mut u8 as *const u8;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut need_quote: i32 = 0;
        i = *p_idx_1;
        {
            j = 0;
            '__b56: loop {
                if !(unsafe { *z_ident.offset(j as isize) } != 0) {
                    break '__b56;
                }
                '__c56: loop {
                    if (unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z_ident.offset(j as isize) } as u8
                                                                as usize)
                                                } as i32 & 6 == 0) as i32 != 0 &&
                            unsafe { *z_ident.offset(j as isize) } as i32 != '_' as i32
                        {
                        break '__b56;
                    }
                    break '__c56;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        need_quote =
            (unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z_ident.offset(0 as isize) } as u8
                                                        as usize)
                                        } as i32 & 4 != 0 ||
                            unsafe { sqlite3_keyword_code(z_ident as *const u8, j) } !=
                                60 || unsafe { *z_ident.offset(j as isize) } as i32 != 0 ||
                    j == 0) as i32;
        if need_quote != 0 {
            unsafe {
                *z.offset({ let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                as isize) = '\"' as i32 as i8
            };
        }
        {
            j = 0;
            '__b57: loop {
                if !(unsafe { *z_ident.offset(j as isize) } != 0) {
                    break '__b57;
                }
                '__c57: loop {
                    unsafe {
                        *z.offset({
                                            let __p = &mut i;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = unsafe { *z_ident.offset(j as isize) } as i8
                    };
                    if unsafe { *z_ident.offset(j as isize) } as i32 ==
                            '\"' as i32 {
                        unsafe {
                            *z.offset({
                                                let __p = &mut i;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = '\"' as i32 as i8
                        };
                    }
                    break '__c57;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        if need_quote != 0 {
            unsafe {
                *z.offset({ let __p = &mut i; let __t = *__p; *__p += 1; __t }
                                as isize) = '\"' as i32 as i8
            };
        }
        unsafe { *z.offset(i as isize) = 0 as i8 };
        *p_idx_1 = i;
    }
}

///* Generate a CREATE TABLE statement appropriate for the given
///* table.  Memory to hold the text of the statement is obtained
///* from sqliteMalloc() and must be freed by the calling function.
#[allow(unused_doc_comments)]
extern "C" fn create_table_stmt(db: *mut Sqlite3, p: &Table) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        let mut k: i32 = 0;
        let mut len: i32 = 0;
        let mut n: i64 = 0 as i64;
        let mut z_stmt: *mut i8 = core::ptr::null_mut();
        let mut z_sep: *const i8 = core::ptr::null();
        let mut z_sep2: *mut i8 = core::ptr::null_mut();
        let mut z_end: *const i8 = core::ptr::null();
        let mut p_col: *const Column = core::ptr::null();
        n = 0 as i64;
        {
            { p_col = (*p).a_col; i = 0 };
            '__b58: loop {
                if !(i < (*p).n_col as i32) { break '__b58; }
                '__c58: loop {
                    n +=
                        ident_length(unsafe { (*p_col).z_cn_name } as *const i8) +
                            5 as i64;
                    break '__c58;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_col;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        n += ident_length((*p).z_name as *const i8);
        if n < 50 as i64 {
            z_sep = c"".as_ptr() as *mut i8;
            z_sep2 = c",".as_ptr() as *mut i8;
            z_end = c")".as_ptr() as *mut i8;
        } else {
            z_sep = c"\n  ".as_ptr() as *mut i8;
            z_sep2 = c",\n  ".as_ptr() as *mut i8;
            z_end = c"\n)".as_ptr() as *mut i8;
        }
        n += (35 + 6 * (*p).n_col as i32) as i64;
        z_stmt =
            unsafe { sqlite3_db_malloc_raw(core::ptr::null_mut(), n as u64) }
                as *mut i8;
        if z_stmt == core::ptr::null_mut() {
            unsafe { sqlite3_oom_fault(db) };
            return core::ptr::null_mut();
        }
        { let _ = 0; };
        unsafe {
            memcpy(z_stmt as *mut (),
                c"CREATE TABLE ".as_ptr() as *mut i8 as *const (), 13 as u64)
        };
        k = 13;
        ident_put(z_stmt, &mut k, (*p).z_name);
        unsafe {
            *z_stmt.offset({
                                let __p = &mut k;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize) = '(' as i32 as i8
        };
        {
            { p_col = (*p).a_col; i = 0 };
            '__b59: loop {
                if !(i < (*p).n_col as i32) { break '__b59; }
                '__c59: loop {
                    /// SQLITE_AFF_BLOB
                    /// SQLITE_AFF_TEXT
                    /// SQLITE_AFF_NUMERIC
                    /// SQLITE_AFF_INTEGER
                    /// SQLITE_AFF_REAL
                    /// SQLITE_AFF_FLEXNUM
                    let mut z_type: *const i8 = core::ptr::null();
                    len = unsafe { sqlite3_strlen30(z_sep as *const i8) };
                    { let _ = 0; };
                    unsafe {
                        memcpy(unsafe { &raw mut *z_stmt.offset(k as isize) } as
                                *mut (), z_sep as *const (), len as u64)
                    };
                    k += len;
                    z_sep = z_sep2;
                    ident_put(z_stmt, &mut k, unsafe { (*p_col).z_cn_name });
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    z_type =
                        az_type[(unsafe { (*p_col).affinity } as i32 - 65) as
                                usize];
                    len = unsafe { sqlite3_strlen30(z_type) };
                    { let _ = 0; };
                    { let _ = 0; };
                    unsafe {
                        memcpy(unsafe { &raw mut *z_stmt.offset(k as isize) } as
                                *mut (), z_type as *const (), len as u64)
                    };
                    k += len;
                    { let _ = 0; };
                    break '__c59;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_col;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        len = unsafe { sqlite3_strlen30(z_end as *const i8) };
        { let _ = 0; };
        unsafe {
            memcpy(unsafe { &raw mut *z_stmt.offset(k as isize) } as *mut (),
                z_end as *const (), (len + 1) as u64)
        };
        return z_stmt;
    }
}

///* This routine is called to report the final ")" that terminates
///* a CREATE TABLE statement.
///*
///* The table structure that other action routines have been building
///* is added to the internal hash tables, assuming no errors have
///* occurred.
///*
///* An entry for the table is made in the schema table on disk, unless
///* this is a temporary table or db->init.busy==1.  When db->init.busy==1
///* it means we are reading the sqlite_schema table because we just
///* connected to the database or because the sqlite_schema table has
///* recently changed, so the entry for this table already exists in
///* the sqlite_schema table.  We do not want to create it again.
///*
///* If the pSelect argument is not NULL, it means that this routine
///* was called to create a table generated from a
///* "CREATE TABLE ... AS SELECT ..." statement.  The column names of
///* the new table will match the result set of the SELECT.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_end_table(p_parse: *mut Parse,
    mut p_cons: *mut Token, p_end: *mut Token, tab_opts: u32,
    p_select: *mut Select) -> () {
    unsafe {
        let mut p: *mut Table = core::ptr::null_mut();
        /// The new table
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        /// The database connection
        let mut i_db: i32 = 0;
        /// Database in which the table lives
        let mut p_idx: *mut Index = core::ptr::null_mut();
        if p_end == core::ptr::null_mut() && p_select == core::ptr::null_mut()
            {
            return;
        }
        p = unsafe { (*p_parse).p_new_table };
        if p == core::ptr::null_mut() { return; }
        if p_select == core::ptr::null_mut() &&
                sqlite3_shadow_table_name(db,
                        unsafe { (*p).z_name } as *const i8) != 0 {
            unsafe { (*p).tab_flags |= 4096 as u32 };
        }
        if unsafe { (*db).init.busy } != 0 {
            if !(p_select).is_null() ||
                    !(unsafe { (*p).e_tab_type } as i32 == 0) as i32 != 0 &&
                        unsafe { (*db).init.new_tnum } != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"".as_ptr() as *mut i8 as *const i8)
                };
                return;
            }
            unsafe { (*p).tnum = unsafe { (*db).init.new_tnum } };
            if unsafe { (*p).tnum } == 1 as u32 {
                unsafe { (*p).tab_flags |= 1 as u32 };
            }
        }
        if tab_opts & 65536 as u32 != 0 {
            let mut ii: i32 = 0;
            unsafe { (*p).tab_flags |= 65536 as u32 };
            {
                ii = 0;
                '__b60: loop {
                    if !(ii < unsafe { (*p).n_col } as i32) { break '__b60; }
                    '__c60: loop {
                        let p_col: *mut Column =
                            unsafe { &mut *unsafe { (*p).a_col.offset(ii as isize) } };
                        if unsafe { (*p_col).e_c_type() } as i32 == 0 {
                            if unsafe { (*p_col).col_flags } as i32 & 4 != 0 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"unknown datatype for %s.%s: \"%s\"".as_ptr() as *mut i8 as
                                            *const i8, unsafe { (*p).z_name },
                                        unsafe { (*p_col).z_cn_name },
                                        unsafe {
                                            sqlite3ColumnType(p_col, c"".as_ptr() as *mut i8)
                                        })
                                };
                            } else {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"missing datatype for %s.%s".as_ptr() as *mut i8 as
                                            *const i8, unsafe { (*p).z_name },
                                        unsafe { (*p_col).z_cn_name })
                                };
                            }
                            return;
                        } else if unsafe { (*p_col).e_c_type() } as i32 == 1 {
                            unsafe { (*p_col).affinity = 65 as i8 };
                        }
                        if unsafe { (*p_col).col_flags } as i32 & 1 != 0 &&
                                    unsafe { (*p).i_p_key } as i32 != ii &&
                                unsafe { (*p_col).not_null() } as i32 == 0 {
                            unsafe { (*p_col).set_not_null(2 as u32 as u32) };
                            unsafe { (*p).tab_flags |= 2048 as u32 };
                        }
                        break '__c60;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        { let _ = 0; };
        { let _ = 0; };
        if tab_opts & 128 as u32 != 0 {
            if unsafe { (*p).tab_flags } & 8 as u32 != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"AUTOINCREMENT not allowed on WITHOUT ROWID tables".as_ptr()
                                as *mut i8 as *const i8)
                };
                return;
            }
            if unsafe { (*p).tab_flags } & 4 as u32 == 0 as u32 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"PRIMARY KEY missing on table %s".as_ptr() as *mut i8 as
                            *const i8, unsafe { (*p).z_name })
                };
                return;
            }
            unsafe { (*p).tab_flags |= (128 | 512) as u32 };
            convert_to_without_rowid_table(p_parse, p);
        }
        i_db =
            unsafe { sqlite3_schema_to_index(db, unsafe { (*p).p_schema }) };
        { let _ = 0; };
        if !(unsafe { (*p).p_check }).is_null() {
            unsafe {
                sqlite3_resolve_self_reference(p_parse, p, 4,
                    core::ptr::null_mut(), unsafe { (*p).p_check })
            };
            if unsafe { (*p_parse).n_err } != 0 {

                /// If errors are seen, delete the CHECK constraints now, else they might
                ///* actually be used if PRAGMA writable_schema=ON is set.
                unsafe {
                    sqlite3_expr_list_delete(db, unsafe { (*p).p_check })
                };
                unsafe { (*p).p_check = core::ptr::null_mut() };
            } else {}
        }
        if unsafe { (*p).tab_flags } & 96 as u32 != 0 {
            let mut ii: i32 = 0;
            let mut n_ng: i32 = 0;
            {
                ii = 0;
                '__b61: loop {
                    if !(ii < unsafe { (*p).n_col } as i32) { break '__b61; }
                    '__c61: loop {
                        let col_flags: u32 =
                            unsafe {
                                    (*unsafe { (*p).a_col.offset(ii as isize) }).col_flags
                                } as u32;
                        if col_flags & 96 as u32 != 0 as u32 {
                            let p_x: *mut Expr =
                                sqlite3_column_expr(unsafe { &*p },
                                    unsafe { &*unsafe { (*p).a_col.offset(ii as isize) } });
                            if unsafe {
                                        sqlite3_resolve_self_reference(p_parse, p, 8, p_x,
                                            core::ptr::null_mut())
                                    } != 0 {

                                /// If there are errors in resolving the expression, change the
                                ///* expression to a NULL.  This prevents code generators that operate
                                ///* on the expression from inserting extra parts into the expression
                                ///* tree that have been allocated from lookaside memory, which is
                                ///* illegal in a schema and will lead to errors or heap corruption
                                ///* when the database connection closes.
                                sqlite3_column_set_expr(p_parse, unsafe { &mut *p },
                                    unsafe { &mut *unsafe { (*p).a_col.offset(ii as isize) } },
                                    unsafe {
                                        sqlite3_expr_alloc(db, 122, core::ptr::null(), 0)
                                    });
                            }
                        } else {
                            { let __p = &mut n_ng; let __t = *__p; *__p += 1; __t };
                        }
                        break '__c61;
                    }
                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                }
            }
            if n_ng == 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"must have at least one non-generated column".as_ptr() as
                                *mut i8 as *const i8)
                };
                return;
            }
        }

        /// Estimate the average row size for the table and for all implied indices
        estimate_table_width(unsafe { &mut *p });
        {
            p_idx = unsafe { (*p).p_index };
            '__b62: loop {
                if !(!(p_idx).is_null()) { break '__b62; }
                '__c62: loop {
                    estimate_index_width(unsafe { &mut *p_idx });
                    break '__c62;
                }
                p_idx = unsafe { (*p_idx).p_next };
            }
        }
        if (unsafe { (*db).init.busy } == 0) as i32 != 0 {
            let mut n: i32 = 0;
            let mut v: *mut Vdbe = core::ptr::null_mut();
            let mut z_type: *mut i8 = core::ptr::null_mut();
            /// "view" or "table"
            let mut z_type2: *mut i8 = core::ptr::null_mut();
            /// "VIEW" or "TABLE"
            let mut z_stmt: *mut i8 = core::ptr::null_mut();

            /// Text of the CREATE TABLE or CREATE VIEW statement
            (v = unsafe { sqlite3_get_vdbe(p_parse) });
            if v == core::ptr::null_mut() { return; }
            unsafe { sqlite3_vdbe_add_op1(v, 124, 0) };
            if unsafe { (*p).e_tab_type } as i32 == 0 {

                /// A regular table
                (z_type = c"table".as_ptr() as *mut i8);
                z_type2 = c"TABLE".as_ptr() as *mut i8;
            } else {

                /// A view
                (z_type = c"view".as_ptr() as *mut i8);
                z_type2 = c"VIEW".as_ptr() as *mut i8;
            }
            if !(p_select).is_null() {
                let mut dest: SelectDest = unsafe { core::mem::zeroed() };
                /// Where the SELECT should store results
                let mut reg_yield: i32 = 0;
                /// Register holding co-routine entry-point
                let mut addr_top: i32 = 0;
                /// Top of the co-routine
                let mut reg_rec: i32 = 0;
                /// A record to be insert into the new table
                let mut reg_rowid: i32 = 0;
                /// Rowid of the next row to insert
                let mut addr_ins_loop: i32 = 0;
                /// Top of the loop for inserting rows
                let mut p_sel_tab: *mut Table = core::ptr::null_mut();
                /// A table that describes the SELECT results
                let mut i_csr: i32 = 0;
                if unsafe { (*p_parse).e_parse_mode } as i32 != 0 {
                    unsafe { (*p_parse).rc = 1 };
                    {
                        let __p = unsafe { &mut (*p_parse).n_err };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    return;
                }
                i_csr =
                    {
                        let __p = unsafe { &mut (*p_parse).n_tab };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                reg_yield =
                    {
                        let __p = unsafe { &mut (*p_parse).n_mem };
                        *__p += 1;
                        *__p
                    };
                reg_rec =
                    {
                        let __p = unsafe { &mut (*p_parse).n_mem };
                        *__p += 1;
                        *__p
                    };
                reg_rowid =
                    {
                        let __p = unsafe { &mut (*p_parse).n_mem };
                        *__p += 1;
                        *__p
                    };
                sqlite3_may_abort(p_parse);
                { let _ = 0; };
                unsafe {
                    sqlite3_vdbe_add_op3(v, 116, i_csr,
                        unsafe { (*p_parse).u1.cr.reg_root }, i_db)
                };
                unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                addr_top = unsafe { sqlite3_vdbe_current_addr(v) } + 1;
                unsafe {
                    sqlite3_vdbe_add_op3(v, 11, reg_yield, 0, addr_top)
                };
                if unsafe { (*p_parse).n_err } != 0 { return; }
                p_sel_tab =
                    unsafe {
                        sqlite3_result_set_of_select(p_parse, p_select, 65 as i8)
                    };
                if p_sel_tab == core::ptr::null_mut() { return; }
                { let _ = 0; };
                unsafe {
                    (*p).n_col =
                        {
                            unsafe { (*p).n_nv_col = unsafe { (*p_sel_tab).n_col } };
                            unsafe { (*p).n_nv_col }
                        }
                };
                unsafe { (*p).a_col = unsafe { (*p_sel_tab).a_col } };
                unsafe { (*p_sel_tab).n_col = 0 as i16 };
                unsafe { (*p_sel_tab).a_col = core::ptr::null_mut() };
                sqlite3_delete_table(db, p_sel_tab);
                unsafe { sqlite3_select_dest_init(&mut dest, 11, reg_yield) };
                unsafe { sqlite3_select(p_parse, p_select, &mut dest) };
                if unsafe { (*p_parse).n_err } != 0 { return; }
                unsafe { sqlite3_vdbe_end_coroutine(v, reg_yield) };
                unsafe { sqlite3_vdbe_jump_here(v, addr_top - 1) };
                addr_ins_loop =
                    unsafe { sqlite3_vdbe_add_op1(v, 12, dest.i_sd_parm) };
                unsafe {
                    sqlite3_vdbe_add_op3(v, 99, dest.i_sdst, dest.n_sdst,
                        reg_rec)
                };
                unsafe { sqlite3_table_affinity(v, p, 0) };
                unsafe { sqlite3_vdbe_add_op2(v, 129, i_csr, reg_rowid) };
                unsafe {
                    sqlite3_vdbe_add_op3(v, 130, i_csr, reg_rec, reg_rowid)
                };
                unsafe { sqlite3_vdbe_goto(v, addr_ins_loop) };
                unsafe { sqlite3_vdbe_jump_here(v, addr_ins_loop) };
                unsafe { sqlite3_vdbe_add_op1(v, 124, i_csr) };
            }
            if !(p_select).is_null() {
                z_stmt = create_table_stmt(db, unsafe { &*p });
            } else {
                let p_end2: *const Token =
                    if tab_opts != 0 {
                            unsafe { &mut (*p_parse).s_last_token }
                        } else { p_end } as *const Token;
                n =
                    unsafe {
                                unsafe {
                                    (*p_end2).z.offset_from(unsafe {
                                            (*p_parse).s_name_token.z
                                        })
                                }
                            } as i64 as i32;
                if unsafe { *unsafe { (*p_end2).z.offset(0 as isize) } } as
                            i32 != ';' as i32 {
                    n += unsafe { (*p_end2).n } as i32;
                }
                z_stmt =
                    unsafe {
                        sqlite3_m_printf(db,
                            c"CREATE %s %.*s".as_ptr() as *mut i8 as *const i8, z_type2,
                            n, unsafe { (*p_parse).s_name_token.z })
                    };
            }

            /// A slot for the record has already been allocated in the
            ///* schema table.  We just need to update that slot with all
            ///* the information we've collected.
            { let _ = 0; };
            unsafe {
                sqlite3_nested_parse(p_parse,
                    c"UPDATE %Q.sqlite_master SET type=\'%s\', name=%Q, tbl_name=%Q, rootpage=#%d, sql=%Q WHERE rowid=#%d".as_ptr()
                            as *mut i8 as *const i8,
                    unsafe {
                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                    }, z_type, unsafe { (*p).z_name }, unsafe { (*p).z_name },
                    unsafe { (*p_parse).u1.cr.reg_root }, z_stmt,
                    unsafe { (*p_parse).u1.cr.reg_rowid })
            };
            unsafe { sqlite3_db_free(db, z_stmt as *mut ()) };
            sqlite3_change_cookie(unsafe { &*p_parse }, i_db);
            if unsafe { (*p).tab_flags } & 8 as u32 != 0 as u32 &&
                    !(unsafe { (*p_parse).e_parse_mode } as i32 != 0) as i32 !=
                        0 {
                let p_db: *const Db =
                    unsafe {
                            &raw mut *unsafe { (*db).a_db.offset(i_db as isize) }
                        } as *const Db;
                { let _ = 0; };
                if unsafe { (*unsafe { (*p_db).p_schema }).p_seq_tab } ==
                        core::ptr::null_mut() {
                    unsafe {
                        sqlite3_nested_parse(p_parse,
                            c"CREATE TABLE %Q.sqlite_sequence(name,seq)".as_ptr() as
                                    *mut i8 as *const i8, unsafe { (*p_db).z_db_s_name })
                    };
                }
            }

            /// Reparse everything to update our internal data structures
            unsafe {
                sqlite3_vdbe_add_parse_schema_op(v, i_db,
                    unsafe {
                        sqlite3_m_printf(db,
                            c"tbl_name=\'%q\' AND type!=\'trigger\'".as_ptr() as *mut i8
                                as *const i8, unsafe { (*p).z_name })
                    }, 0 as u16)
            };
            if unsafe { (*p).tab_flags } & 96 as u32 != 0 {
                unsafe {
                    sqlite3_vdbe_add_op4(v, 150, 1, 0, 0,
                        unsafe {
                                sqlite3_m_printf(db,
                                    c"SELECT*FROM\"%w\".\"%w\"".as_ptr() as *mut i8 as
                                        *const i8,
                                    unsafe {
                                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                    }, unsafe { (*p).z_name })
                            } as *const i8, -7)
                };
            }
        }
        if unsafe { (*db).init.busy } != 0 {
            let mut p_old: *const Table = core::ptr::null();
            let p_schema: *mut Schema = unsafe { (*p).p_schema };
            { let _ = 0; };
            { let _ = 0; };
            p_old =
                unsafe {
                        sqlite3_hash_insert(unsafe { &mut (*p_schema).tbl_hash },
                            unsafe { (*p).z_name } as *const i8, p as *mut ())
                    } as *mut Table;
            if !(p_old).is_null() {
                { let _ = 0; };

                /// Malloc must have failed inside HashInsert()
                unsafe { sqlite3_oom_fault(db) };
                return;
            }
            unsafe { (*p_parse).p_new_table = core::ptr::null_mut() };
            unsafe { (*db).m_db_flags |= 1 as u32 };

            /// If this is the magic sqlite_sequence table used by autoincrement,
            ///* then record a pointer to this table in the main database structure
            ///* so that INSERT can find the table easily.
            { let _ = 0; };
            if unsafe {
                        strcmp(unsafe { (*p).z_name } as *const i8,
                            c"sqlite_sequence".as_ptr() as *mut i8 as *const i8)
                    } == 0 {
                { let _ = 0; };
                unsafe { (*unsafe { (*p).p_schema }).p_seq_tab = p };
            }
        }
        if (p_select).is_null() as i32 != 0 &&
                unsafe { (*p).e_tab_type } as i32 == 0 {
            { let _ = 0; };
            if unsafe { (*p_cons).z } == core::ptr::null() { p_cons = p_end; }
            unsafe {
                (*p).u.tab.add_col_offset =
                    13 +
                        unsafe {
                                    unsafe {
                                        (*p_cons).z.offset_from(unsafe {
                                                (*p_parse).s_name_token.z
                                            })
                                    }
                                } as i64 as i32
            };
        }
    }
}

///* Clean up the data structures associated with the RETURNING clause.
extern "C" fn sqlite3_delete_returning(db: *mut Sqlite3, p_arg_1: *mut ())
    -> () {
    unsafe {
        let p_ret: *mut Returning = p_arg_1 as *mut Returning;
        let mut p_hash: *mut Hash = core::ptr::null_mut();
        p_hash =
            unsafe {
                &mut (*unsafe {
                                    (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                                }).trig_hash
            };
        unsafe {
            sqlite3_hash_insert(p_hash,
                unsafe { &raw mut (*p_ret).z_name[0 as usize] } as *mut i8 as
                    *const i8, core::ptr::null_mut())
        };
        unsafe {
            sqlite3_expr_list_delete(db, unsafe { (*p_ret).p_return_el })
        };
        unsafe { sqlite3_db_free(db, p_ret as *mut ()) };
    }
}

///* Add the RETURNING clause to the parse currently underway.
///*
///* This routine creates a special TEMP trigger that will fire for each row
///* of the DML statement.  That TEMP trigger contains a single SELECT
///* statement with a result set that is the argument of the RETURNING clause.
///* The trigger has the Trigger.bReturning flag and an opcode of
///* TK_RETURNING instead of TK_SELECT, so that the trigger code generator
///* knows to handle it specially.  The TEMP trigger is automatically
///* removed at the end of the parse.
///*
///* When this routine is called, we do not yet know if the RETURNING clause
///* is attached to a DELETE, INSERT, or UPDATE, so construct it as a
///* RETURNING trigger instead.  It will then be converted into the appropriate
///* type on the first call to sqlite3TriggersExist().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_add_returning(p_parse: *mut Parse,
    p_list: *mut ExprList) -> () {
    unsafe {
        let mut p_ret: *mut Returning = core::ptr::null_mut();
        let mut p_hash: *mut Hash = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        if !(unsafe { (*p_parse).p_new_trigger }).is_null() {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"cannot use RETURNING in a trigger".as_ptr() as *mut i8 as
                        *const i8)
            };
        } else { { let _ = 0; }; }
        unsafe { (*p_parse).set_b_returning(1 as Bft as u32) };
        p_ret =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<Returning>() as u64)
                } as *mut Returning;
        if p_ret == core::ptr::null_mut() {
            unsafe { sqlite3_expr_list_delete(db, p_list) };
            return;
        }
        { let _ = 0; };
        unsafe { (*p_parse).u1.d.p_returning = p_ret };
        unsafe { (*p_ret).p_parse = p_parse };
        unsafe { (*p_ret).p_return_el = p_list };
        unsafe {
            sqlite3_parser_add_cleanup(p_parse,
                Some(sqlite3_delete_returning), p_ret as *mut ())
        };
        if unsafe { (*db).malloc_failed } != 0 { return; }
        unsafe {
            sqlite3_snprintf(core::mem::size_of::<[i8; 40]>() as i32,
                unsafe { &raw mut (*p_ret).z_name[0 as usize] } as *mut i8,
                c"sqlite_returning_%p".as_ptr() as *mut i8 as *const i8,
                p_parse)
        };
        unsafe {
            (*p_ret).ret_trig.z_name =
                unsafe { &raw mut (*p_ret).z_name[0 as usize] } as *mut i8
        };
        unsafe { (*p_ret).ret_trig.op = 151 as u8 };
        unsafe { (*p_ret).ret_trig.tr_tm = 2 as u8 };
        unsafe { (*p_ret).ret_trig.b_returning = 1 as u8 };
        unsafe {
            (*p_ret).ret_trig.p_schema =
                unsafe {
                    (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                }
        };
        unsafe {
            (*p_ret).ret_trig.p_tab_schema =
                unsafe {
                    (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                }
        };
        unsafe {
            (*p_ret).ret_trig.step_list = unsafe { &mut (*p_ret).ret_t_step }
        };
        unsafe { (*p_ret).ret_t_step.op = 151 as u8 };
        unsafe {
            (*p_ret).ret_t_step.p_trig = unsafe { &mut (*p_ret).ret_trig }
        };
        unsafe { (*p_ret).ret_t_step.p_expr_list = p_list };
        p_hash =
            unsafe {
                &mut (*unsafe {
                                    (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                                }).trig_hash
            };
        { let _ = 0; };
        if unsafe {
                    sqlite3_hash_insert(p_hash,
                        unsafe { &raw mut (*p_ret).z_name[0 as usize] } as *mut i8
                            as *const i8,
                        unsafe { &raw mut (*p_ret).ret_trig } as *mut ())
                } == unsafe { &raw mut (*p_ret).ret_trig } as *mut () {
            unsafe { sqlite3_oom_fault(db) };
        }
    }
}

///* The parser calls this routine in order to create a new VIEW
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_view(p_parse: *mut Parse, p_begin: &Token,
    p_name1: *mut Token, p_name2: *mut Token, p_c_names: *mut ExprList,
    mut p_select: *mut Select, is_temp: i32, no_err: i32) -> () {
    unsafe {
        unsafe {
            let mut p: *mut Table = core::ptr::null_mut();
            let mut n: i32 = 0;
            let mut z: *const i8 = core::ptr::null();
            let mut s_end: Token = unsafe { core::mem::zeroed() };
            let mut s_fix: DbFixer = unsafe { core::mem::zeroed() };
            let mut p_name: *mut Token = core::ptr::null_mut();
            let mut i_db: i32 = 0;
            let mut db: *mut Sqlite3 = core::ptr::null_mut();
            let mut __state: i32 = 0;
            loop {
                if __state == 1 { break; }
                '__s64:
                    {
                    match __state {
                        0 => { __state = 3; }
                        2 => {
                            unsafe { sqlite3_select_delete(db, p_select) };
                            __state = 47;
                        }
                        3 => { __state = 4; }
                        4 => { __state = 5; }
                        5 => { __state = 6; }
                        6 => { __state = 7; }
                        7 => { p_name = core::ptr::null_mut(); __state = 8; }
                        8 => { __state = 9; }
                        9 => { db = unsafe { (*p_parse).db }; __state = 10; }
                        10 => {
                            if unsafe { (*p_parse).n_var } as i32 > 0 {
                                __state = 12;
                            } else { __state = 11; }
                        }
                        11 => {
                            sqlite3_start_table(p_parse, p_name1, p_name2, is_temp, 1,
                                0, no_err);
                            __state = 14;
                        }
                        12 => {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"parameters are not allowed in views".as_ptr() as *mut i8
                                        as *const i8)
                            };
                            __state = 13;
                        }
                        13 => { __state = 2; }
                        14 => {
                            p = unsafe { (*p_parse).p_new_table };
                            __state = 15;
                        }
                        15 => {
                            if p == core::ptr::null_mut() ||
                                    unsafe { (*p_parse).n_err } != 0 {
                                __state = 17;
                            } else { __state = 16; }
                        }
                        16 => {
                            unsafe { (*p).tab_flags |= 512 as u32 };
                            __state = 18;
                        }
                        17 => { __state = 2; }
                        18 => {
                            sqlite3_two_part_name(p_parse, p_name1, p_name2,
                                &mut p_name);
                            __state = 19;
                        }
                        19 => {
                            i_db =
                                unsafe {
                                    sqlite3_schema_to_index(db, unsafe { (*p).p_schema })
                                };
                            __state = 20;
                        }
                        20 => { { let _ = 0; }; __state = 21; }
                        21 => {
                            unsafe {
                                sqlite3_fix_init(&mut s_fix, p_parse, i_db,
                                    c"view".as_ptr() as *mut i8 as *const i8,
                                    p_name as *const Token)
                            };
                            __state = 22;
                        }
                        22 => {
                            if unsafe { sqlite3_fix_select(&mut s_fix, p_select) } != 0
                                {
                                __state = 24;
                            } else { __state = 23; }
                        }
                        23 => {
                            unsafe { (*p_select).sel_flags |= 2097152 as u32 };
                            __state = 25;
                        }
                        24 => { __state = 2; }
                        25 => {
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                __state = 27;
                            } else { __state = 28; }
                        }
                        26 => {
                            unsafe {
                                (*p).p_check =
                                    unsafe {
                                        sqlite3_expr_list_dup(db, p_c_names as *const ExprList, 1)
                                    }
                            };
                            __state = 30;
                        }
                        27 => {
                            unsafe { (*p).u.view.p_select = p_select };
                            __state = 29;
                        }
                        28 => {
                            unsafe {
                                (*p).u.view.p_select =
                                    unsafe {
                                        sqlite3_select_dup(db, p_select as *const Select, 1)
                                    }
                            };
                            __state = 26;
                        }
                        29 => { p_select = core::ptr::null_mut(); __state = 26; }
                        30 => {
                            unsafe { (*p).e_tab_type = 2 as u8 };
                            __state = 31;
                        }
                        31 => {
                            if unsafe { (*db).malloc_failed } != 0 {
                                __state = 33;
                            } else { __state = 32; }
                        }
                        32 => {
                            s_end = unsafe { (*p_parse).s_last_token };
                            __state = 34;
                        }
                        33 => { __state = 2; }
                        34 => { { let _ = 0; }; __state = 35; }
                        35 => {
                            if unsafe { *s_end.z.offset(0 as isize) } as i32 !=
                                    ';' as i32 {
                                __state = 37;
                            } else { __state = 36; }
                        }
                        36 => { s_end.n = 0 as u32; __state = 38; }
                        37 => {
                            {
                                let __n = s_end.n;
                                let __p = &mut s_end.z;
                                *__p = unsafe { (*__p).add(__n as usize) };
                            };
                            __state = 36;
                        }
                        38 => {
                            n =
                                unsafe { s_end.z.offset_from((*p_begin).z) } as i64 as i32;
                            __state = 39;
                        }
                        39 => { { let _ = 0; }; __state = 40; }
                        40 => { z = (*p_begin).z; __state = 41; }
                        41 => {
                            if unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.offset((n - 1) as isize) } as u8
                                                            as usize)
                                            } as i32 & 1 != 0 {
                                __state = 43;
                            } else { __state = 42; }
                        }
                        42 => {
                            s_end.z = unsafe { z.offset((n - 1) as isize) };
                            __state = 44;
                        }
                        43 => {
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                            __state = 41;
                        }
                        44 => { s_end.n = 1 as u32; __state = 45; }
                        45 => {
                            sqlite3_end_table(p_parse, core::ptr::null_mut(),
                                &mut s_end, 0 as u32, core::ptr::null_mut());
                            __state = 46;
                        }
                        46 => { __state = 2; }
                        47 => {
                            if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                __state = 49;
                            } else { __state = 48; }
                        }
                        48 => {
                            unsafe { sqlite3_expr_list_delete(db, p_c_names) };
                            __state = 50;
                        }
                        49 => {
                            unsafe {
                                sqlite3_rename_exprlist_unmap(p_parse, p_c_names)
                            };
                            __state = 48;
                        }
                        50 => { return; }
                        _ => {}
                    }
                }
            }
        }
    }
}

///* Assign VdbeCursor index numbers to all tables in a SrcList
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_list_assign_cursors(p_parse: *mut Parse,
    p_list: *mut SrcList) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        { let _ = 0; };
        if !(p_list).is_null() {
            {
                {
                    i = 0;
                    p_item = unsafe { (*p_list).a.as_ptr() } as *mut SrcItem
                };
                '__b65: loop {
                    if !(i < unsafe { (*p_list).n_src }) { break '__b65; }
                    '__c65: loop {
                        if unsafe { (*p_item).i_cursor } >= 0 { break '__c65; }
                        unsafe {
                            (*p_item).i_cursor =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                }
                        };
                        if unsafe { (*p_item).fg.is_subquery() } != 0 {
                            { let _ = 0; };
                            { let _ = 0; };
                            { let _ = 0; };
                            sqlite3_src_list_assign_cursors(p_parse,
                                unsafe {
                                    (*unsafe {
                                                    (*unsafe { (*p_item).u4.p_subq }).p_select
                                                }).p_src
                                });
                        }
                        break '__c65;
                    }
                    {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        {
                            let __p = &mut p_item;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
        }
    }
}

///* The Table structure pTable is really a VIEW.  Fill in the names of
///* the columns of the view in the pTable structure.  Return non-zero if
///* there are errors.  If an error is seen an error message is left
///* in pParse->zErrMsg.
#[allow(unused_doc_comments)]
extern "C" fn view_get_column_names(p_parse_1: *mut Parse,
    p_table_1: *mut Table) -> i32 {
    unsafe {
        let mut p_sel_tab: *mut Table = core::ptr::null_mut();
        /// A fake table from which we get the result set
        let mut p_sel: *mut Select = core::ptr::null_mut();
        /// Copy of the SELECT that implements the view
        let mut n_err: i32 = 0;
        /// Number of errors encountered
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        /// Database connection for malloc errors
        let mut rc: i32 = 0;
        let mut x_auth:
                Option<unsafe extern "C" fn(*mut (), i32, *const i8,
                    *const i8, *const i8, *const i8) -> i32> = None;

        /// Saved xAuth pointer
        { let _ = 0; };
        if unsafe { (*p_table_1).e_tab_type } as i32 == 1 {
            {
                let __p = unsafe { &mut (*db).n_schema_lock };
                let __t = *__p;
                *__p += 1;
                __t
            };
            rc = unsafe { sqlite3_vtab_call_connect(p_parse_1, p_table_1) };
            {
                let __p = unsafe { &mut (*db).n_schema_lock };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            return rc;
        }

        /// A positive nCol means the columns names for this view are
        ///* already known.  This routine is not called unless either the
        ///* table is virtual or nCol is zero.
        { let _ = 0; };
        if (unsafe { (*p_table_1).n_col } as i32) < 0 {
            unsafe {
                sqlite3_error_msg(p_parse_1,
                    c"view %s is circularly defined".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_table_1).z_name })
            };
            return 1;
        }
        { let _ = 0; };

        /// If we get this far, it means we need to compute the table names.
        ///* Note that the call to sqlite3ResultSetOfSelect() will expand any
        ///* "*" elements in the results set of the view and will assign cursors
        ///* to the elements of the FROM clause.  But we do not want these changes
        ///* to be permanent.  So the computation is done on a copy of the SELECT
        ///* statement that defines the view.
        { let _ = 0; };
        p_sel =
            unsafe {
                sqlite3_select_dup(db,
                    unsafe { (*p_table_1).u.view.p_select } as *const Select, 0)
            };
        if !(p_sel).is_null() {
            let e_parse_mode: u8 = unsafe { (*p_parse_1).e_parse_mode };
            let n_tab: i32 = unsafe { (*p_parse_1).n_tab };
            let n_select: i32 = unsafe { (*p_parse_1).n_select };
            unsafe { (*p_parse_1).e_parse_mode = 0 as u8 };
            sqlite3_src_list_assign_cursors(p_parse_1,
                unsafe { (*p_sel).p_src });
            unsafe { (*p_table_1).n_col = -1 as i16 };
            {
                let __p = unsafe { &mut (*db).lookaside.b_disable };
                let __t = *__p;
                *__p += 1;
                __t
            };
            unsafe { (*db).lookaside.sz = 0 as u16 };
            x_auth = unsafe { (*db).x_auth };
            unsafe { (*db).x_auth = None };
            p_sel_tab =
                unsafe {
                    sqlite3_result_set_of_select(p_parse_1, p_sel, 64 as i8)
                };
            unsafe { (*db).x_auth = x_auth };
            unsafe { (*p_parse_1).n_tab = n_tab };
            unsafe { (*p_parse_1).n_select = n_select };
            if p_sel_tab == core::ptr::null_mut() {
                unsafe { (*p_table_1).n_col = 0 as i16 };
                { let __p = &mut n_err; let __t = *__p; *__p += 1; __t };
            } else if !(unsafe { (*p_table_1).p_check }).is_null() {

                /// CREATE VIEW name(arglist) AS ...
                ///* The names of the columns in the table are taken from
                ///* arglist which is stored in pTable->pCheck.  The pCheck field
                ///* normally holds CHECK constraints on an ordinary table, but for
                ///* a VIEW it holds the list of column names.
                unsafe {
                    sqlite3_columns_from_expr_list(p_parse_1,
                        unsafe { (*p_table_1).p_check },
                        unsafe { &mut (*p_table_1).n_col },
                        unsafe { &mut (*p_table_1).a_col })
                };
                if unsafe { (*p_parse_1).n_err } == 0 &&
                        unsafe { (*p_table_1).n_col } as i32 ==
                            unsafe { (*unsafe { (*p_sel).p_e_list }).n_expr } {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_subquery_column_types(p_parse_1, p_table_1, p_sel,
                            64 as i8)
                    };
                }
            } else {

                /// CREATE VIEW name AS...  without an argument list.  Construct
                ///* the column names from the SELECT statement that defines the view.
                { let _ = 0; };
                unsafe { (*p_table_1).n_col = unsafe { (*p_sel_tab).n_col } };
                unsafe { (*p_table_1).a_col = unsafe { (*p_sel_tab).a_col } };
                unsafe {
                    (*p_table_1).tab_flags |=
                        unsafe { (*p_sel_tab).tab_flags } & 98 as u32
                };
                unsafe { (*p_sel_tab).n_col = 0 as i16 };
                unsafe { (*p_sel_tab).a_col = core::ptr::null_mut() };
                { let _ = 0; };
            }
            unsafe { (*p_table_1).n_nv_col = unsafe { (*p_table_1).n_col } };
            sqlite3_delete_table(db, p_sel_tab);
            unsafe { sqlite3_select_delete(db, p_sel) };
            {
                let __p = unsafe { &mut (*db).lookaside.b_disable };
                let __t = *__p;
                *__p -= 1;
                __t
            };
            unsafe {
                (*db).lookaside.sz =
                    if unsafe { (*db).lookaside.b_disable } != 0 {
                            0
                        } else { (unsafe { (*db).lookaside.sz_true }) as i32 } as
                        u16
            };
            unsafe { (*p_parse_1).e_parse_mode = e_parse_mode };
        } else { { let __p = &mut n_err; let __t = *__p; *__p += 1; __t }; }
        unsafe {
            (*unsafe { (*p_table_1).p_schema }).schema_flags |= 2 as u16
        };
        if unsafe { (*db).malloc_failed } != 0 {
            sqlite3_delete_column_names(db, unsafe { &mut *p_table_1 });
        }

        /// SQLITE_OMIT_VIEW
        return n_err + unsafe { (*p_parse_1).n_err };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_view_get_column_names(p_parse: *mut Parse,
    p_table: *mut Table) -> i32 {
    { let _ = 0; };
    if !(unsafe { (*p_table).e_tab_type } as i32 == 1) as i32 != 0 &&
            unsafe { (*p_table).n_col } as i32 > 0 {
        return 0;
    }
    return view_get_column_names(p_parse, p_table);
}

///* If argument zDb is NULL, then call sqlite3CodeVerifySchema() for each
///* attached database. Otherwise, invoke it for the database named zDb only.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_code_verify_named_schema(p_parse: *mut Parse,
    z_db: *const i8) -> () {
    let db: *const Sqlite3 = unsafe { (*p_parse).db } as *const Sqlite3;
    let mut i: i32 = 0;
    {
        i = 0;
        '__b66: loop {
            if !(i < unsafe { (*db).n_db }) { break '__b66; }
            '__c66: loop {
                let p_db: *const Db =
                    unsafe {
                            &raw mut *unsafe { (*db).a_db.offset(i as isize) }
                        } as *const Db;
                if !(unsafe { (*p_db).p_bt }).is_null() &&
                        ((z_db).is_null() as i32 != 0 ||
                            0 ==
                                unsafe {
                                    sqlite3_str_i_cmp(z_db,
                                        unsafe { (*p_db).z_db_s_name } as *const i8)
                                }) {
                    sqlite3_code_verify_schema(p_parse, i);
                }
                break '__c66;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Return true if it is not allowed to drop the given table
extern "C" fn table_may_not_be_dropped(db: *mut Sqlite3, p_tab_1: &Table)
    -> i32 {
    if unsafe {
                sqlite3_strnicmp((*p_tab_1).z_name as *const i8,
                    c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
            } == 0 {
        if unsafe {
                    sqlite3_strnicmp(unsafe {
                                (*p_tab_1).z_name.offset(7 as isize)
                            } as *const i8, c"stat".as_ptr() as *mut i8 as *const i8, 4)
                } == 0 {
            return 0;
        }
        if unsafe {
                    sqlite3_strnicmp(unsafe {
                                (*p_tab_1).z_name.offset(7 as isize)
                            } as *const i8,
                        c"parameters".as_ptr() as *mut i8 as *const i8, 10)
                } == 0 {
            return 0;
        }
        return 1;
    }
    if (*p_tab_1).tab_flags & 4096 as u32 != 0 as u32 &&
            sqlite3_read_only_shadow_tables(unsafe { &*db }) != 0 {
        return 1;
    }
    if (*p_tab_1).tab_flags & 32768 as u32 != 0 { return 1; }
    return 0;
}

///* Remove entries from the sqlite_statN tables (for N in (1,2,3))
///* after a DROP INDEX or DROP TABLE command.
extern "C" fn sqlite3_clear_stat_tables(p_parse_1: *mut Parse, i_db_1: i32,
    z_type_1: *const i8, z_name_1: *const i8) -> () {
    let mut i: i32 = 0;
    let z_db_name: *const i8 =
        unsafe {
                (*unsafe {
                            (*unsafe { (*p_parse_1).db }).a_db.offset(i_db_1 as isize)
                        }).z_db_s_name
            } as *const i8;
    {
        i = 1;
        '__b67: loop {
            if !(i <= 4) { break '__b67; }
            '__c67: loop {
                let mut z_tab: [i8; 24] = [0; 24];
                unsafe {
                    sqlite3_snprintf(core::mem::size_of::<[i8; 24]>() as i32,
                        &raw mut z_tab[0 as usize] as *mut i8,
                        c"sqlite_stat%d".as_ptr() as *mut i8 as *const i8, i)
                };
                if !(sqlite3_find_table(unsafe {
                                        &*unsafe { (*p_parse_1).db }
                                    }, &raw mut z_tab[0 as usize] as *mut i8 as *const i8,
                                    z_db_name)).is_null() {
                    unsafe {
                        sqlite3_nested_parse(p_parse_1,
                            c"DELETE FROM %Q.%s WHERE %s=%Q".as_ptr() as *mut i8 as
                                *const i8, z_db_name, &raw mut z_tab[0 as usize] as *mut i8,
                            z_type_1, z_name_1)
                    };
                }
                break '__c67;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Write code to erase the table with root-page iTable from database iDb.
///* Also write code to modify the sqlite_schema table and internal schema
///* if a root-page of another table is moved by the btree-layer whilst
///* erasing iTable (this can happen with an auto-vacuum database).
#[allow(unused_doc_comments)]
extern "C" fn destroy_root_page(p_parse_1: *mut Parse, i_table_1: i32,
    i_db_1: i32) -> () {
    let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse_1) };
    let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
    if i_table_1 < 2 {
        unsafe {
            sqlite3_error_msg(p_parse_1,
                c"corrupt schema".as_ptr() as *mut i8 as *const i8)
        };
    }
    unsafe { sqlite3_vdbe_add_op3(v, 146, i_table_1, r1, i_db_1) };
    sqlite3_may_abort(p_parse_1);

    /// OP_Destroy stores an in integer r1. If this integer
    ///* is non-zero, then it is the root page number of a table moved to
    ///* location iTable. The following code modifies the sqlite_schema table to
    ///* reflect this.
    ///*
    ///* The "#NNN" in the SQL is a special constant that means whatever value
    ///* is in register NNN.  See grammar rules associated with the TK_REGISTER
    ///* token for additional information.
    unsafe {
        sqlite3_nested_parse(p_parse_1,
            c"UPDATE %Q.sqlite_master SET rootpage=%d WHERE #%d AND rootpage=#%d".as_ptr()
                    as *mut i8 as *const i8,
            unsafe {
                (*unsafe {
                            (*unsafe { (*p_parse_1).db }).a_db.offset(i_db_1 as isize)
                        }).z_db_s_name
            }, i_table_1, r1, r1)
    };
    unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
}

///* Write VDBE code to erase table pTab and all associated indices on disk.
///* Code to update the sqlite_schema tables and internal schema definitions
///* in case a root-page belonging to another table is moved by the btree layer
///* is also added (this can happen with an auto-vacuum database).
#[allow(unused_doc_comments)]
extern "C" fn destroy_table(p_parse_1: *mut Parse, p_tab_1: &Table) -> () {
    unsafe {
        /// If the database may be auto-vacuum capable (if SQLITE_OMIT_AUTOVACUUM
        ///* is not defined), then it is important to call OP_Destroy on the
        ///* table and index root-pages in order, starting with the numerically
        ///* largest root-page number. This guarantees that none of the root-pages
        ///* to be destroyed is relocated by an earlier OP_Destroy. i.e. if the
        ///* following were coded:
        ///*
        ///* OP_Destroy 4 0
        ///* ...
        ///* OP_Destroy 5 0
        ///*
        ///* and root page 5 happened to be the largest root-page number in the
        ///* database, then root page 5 would be moved to page 4 by the
        ///* "OP_Destroy 4 0" opcode. The subsequent "OP_Destroy 5 0" would hit
        ///* a free-list page.
        let i_tab: Pgno = (*p_tab_1).tnum;
        let mut i_destroyed: Pgno = 0 as Pgno;
        loop {
            let mut p_idx: *const Index = core::ptr::null();
            let mut i_largest: Pgno = 0 as Pgno;
            if i_destroyed == 0 as u32 || i_tab < i_destroyed {
                i_largest = i_tab;
            }
            {
                p_idx = (*p_tab_1).p_index;
                '__b69: loop {
                    if !(!(p_idx).is_null()) { break '__b69; }
                    '__c69: loop {
                        let i_idx: Pgno = unsafe { (*p_idx).tnum };
                        { let _ = 0; };
                        if (i_destroyed == 0 as u32 || i_idx < i_destroyed) &&
                                i_idx > i_largest {
                            i_largest = i_idx;
                        }
                        break '__c69;
                    }
                    p_idx = unsafe { (*p_idx).p_next };
                }
            }
            if i_largest == 0 as u32 {
                return;
            } else {
                let i_db: i32 =
                    unsafe {
                        sqlite3_schema_to_index(unsafe { (*p_parse_1).db },
                            (*p_tab_1).p_schema)
                    };
                { let _ = 0; };
                destroy_root_page(p_parse_1, i_largest as i32, i_db);
                i_destroyed = i_largest;
            }
        }
    }
}

///* Clear the column names from every VIEW in database idx.
extern "C" fn sqlite_view_reset_all(db: *mut Sqlite3, idx: i32) -> () {
    unsafe {
        let mut i: *const HashElem = core::ptr::null();
        { let _ = 0; };
        if !(unsafe {
                                        (*unsafe {
                                                        (*unsafe { (*db).a_db.offset(idx as isize) }).p_schema
                                                    }).schema_flags
                                    } as i32 & 2 == 2) as i32 != 0 {
            return;
        }
        {
            i =
                unsafe {
                    (*unsafe {
                                    &mut (*unsafe {
                                                        (*unsafe { (*db).a_db.offset(idx as isize) }).p_schema
                                                    }).tbl_hash
                                }).first
                };
            '__b70: loop {
                if !(!(i).is_null()) { break '__b70; }
                '__c70: loop {
                    let p_tab: *mut Table = unsafe { (*i).data } as *mut Table;
                    if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                        sqlite3_delete_column_names(db, unsafe { &mut *p_tab });
                    }
                    break '__c70;
                }
                i = unsafe { (*i).next };
            }
        }
        unsafe {
            (*unsafe {
                                (*unsafe { (*db).a_db.offset(idx as isize) }).p_schema
                            }).schema_flags &= !2 as u16
        };
    }
}

///* Generate code to drop a table.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_code_drop_table(p_parse: *mut Parse,
    p_tab: *mut Table, i_db: i32, is_view: i32) -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let mut p_trigger: *mut Trigger = core::ptr::null_mut();
    let p_db: *const Db =
        unsafe { &raw mut *unsafe { (*db).a_db.offset(i_db as isize) } } as
            *const Db;
    v = unsafe { sqlite3_get_vdbe(p_parse) };
    { let _ = 0; };
    sqlite3_begin_write_operation(p_parse, 1, i_db);
    if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
        unsafe { sqlite3_vdbe_add_op0(v, 172) };
    }

    /// Drop all triggers associated with the table being dropped. Code
    ///* is generated to remove entries from sqlite_schema and/or
    ///* sqlite_temp_schema if required.
    (p_trigger = unsafe { sqlite3_trigger_list(p_parse, p_tab) });
    while !(p_trigger).is_null() {
        { let _ = 0; };
        unsafe { sqlite3_drop_trigger_ptr(p_parse, p_trigger) };
        p_trigger = unsafe { (*p_trigger).p_next };
    }
    if unsafe { (*p_tab).tab_flags } & 8 as u32 != 0 {
        unsafe {
            sqlite3_nested_parse(p_parse,
                c"DELETE FROM %Q.sqlite_sequence WHERE name=%Q".as_ptr() as
                        *mut i8 as *const i8, unsafe { (*p_db).z_db_s_name },
                unsafe { (*p_tab).z_name })
        };
    }

    /// Drop all entries in the schema table that refer to the
    ///* table. The program name loops through the schema table and deletes
    ///* every row that refers to a table of the same name as the one being
    ///* dropped. Triggers are handled separately because a trigger can be
    ///* created in the temp database that refers to a table in another
    ///* database.
    unsafe {
        sqlite3_nested_parse(p_parse,
            c"DELETE FROM %Q.sqlite_master WHERE tbl_name=%Q and type!=\'trigger\'".as_ptr()
                    as *mut i8 as *const i8, unsafe { (*p_db).z_db_s_name },
            unsafe { (*p_tab).z_name })
    };
    if (is_view == 0) as i32 != 0 &&
            !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0 {
        destroy_table(p_parse, unsafe { &*p_tab });
    }
    if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
        unsafe {
            sqlite3_vdbe_add_op4(v, 174, i_db, 0, 0,
                unsafe { (*p_tab).z_name } as *const i8, 0)
        };
        sqlite3_may_abort(p_parse);
    }
    unsafe {
        sqlite3_vdbe_add_op4(v, 153, i_db, 0, 0,
            unsafe { (*p_tab).z_name } as *const i8, 0)
    };
    sqlite3_change_cookie(unsafe { &*p_parse }, i_db);
    sqlite_view_reset_all(db, i_db);
}

///* This routine is called to do the work of a DROP TABLE statement.
///* pName is the name of the table to be dropped.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_drop_table(p_parse: *mut Parse,
    p_name: *mut SrcList, is_view: i32, no_err: i32) -> () {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        '__b72: loop {
            '__c72: loop {
                let mut p_tab: *mut Table = core::ptr::null_mut();
                let mut v: *const Vdbe = core::ptr::null();
                let mut i_db: i32 = 0;
                if unsafe { (*db).malloc_failed } != 0 { break '__b72; }
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { sqlite3_read_schema(p_parse) } != 0 {
                    break '__b72;
                }
                if no_err != 0 {
                    {
                        let __p = unsafe { &mut (*db).suppress_err };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
                { let _ = 0; };
                p_tab =
                    sqlite3_locate_table_item(p_parse, is_view as u32,
                        unsafe {
                            &*(unsafe { (*p_name).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as isize)
                        });
                if no_err != 0 {
                    {
                        let __p = unsafe { &mut (*db).suppress_err };
                        let __t = *__p;
                        *__p -= 1;
                        __t
                    };
                }
                if p_tab == core::ptr::null_mut() {
                    if no_err != 0 {
                        sqlite3_code_verify_named_schema(p_parse,
                            unsafe {
                                    (*(unsafe { (*p_name).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).u4.z_database
                                } as *const i8);
                        sqlite3_force_not_read_only(p_parse);
                    }
                    break '__b72;
                }
                i_db =
                    unsafe {
                        sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                    };
                { let _ = 0; };
                if unsafe { (*p_tab).e_tab_type } as i32 == 1 &&
                        sqlite3_view_get_column_names(p_parse, p_tab) != 0 {
                    break '__b72;
                }
                {
                    let mut code: i32 = 0;
                    let z_tab: *const i8 =
                        if (0 == 0) as i32 != 0 && i_db == 1 {
                                c"sqlite_temp_master".as_ptr() as *mut i8
                            } else { c"sqlite_master".as_ptr() as *mut i8 } as
                            *const i8;
                    let z_db: *const i8 =
                        unsafe {
                                (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                            } as *const i8;
                    let mut z_arg2: *const i8 = core::ptr::null();
                    if unsafe {
                                sqlite3_auth_check(p_parse, 9, z_tab, core::ptr::null(),
                                    z_db)
                            } != 0 {
                        break '__b72;
                    }
                    if is_view != 0 {
                        if (0 == 0) as i32 != 0 && i_db == 1 {
                            code = 15;
                        } else { code = 17; }
                    } else if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                        code = 30;
                        z_arg2 =
                            unsafe {
                                (*unsafe {
                                                (*unsafe { sqlite3_get_v_table(db, p_tab) }).p_mod
                                            }).z_name
                            };
                    } else {
                        if (0 == 0) as i32 != 0 && i_db == 1 {
                            code = 13;
                        } else { code = 11; }
                    }
                    if unsafe {
                                sqlite3_auth_check(p_parse, code,
                                    unsafe { (*p_tab).z_name } as *const i8, z_arg2, z_db)
                            } != 0 {
                        break '__b72;
                    }
                    if unsafe {
                                sqlite3_auth_check(p_parse, 9,
                                    unsafe { (*p_tab).z_name } as *const i8, core::ptr::null(),
                                    z_db)
                            } != 0 {
                        break '__b72;
                    }
                }
                if table_may_not_be_dropped(db, unsafe { &*p_tab }) != 0 {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"table %s may not be dropped".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p_tab).z_name })
                    };
                    break '__b72;
                }
                if is_view != 0 &&
                        !(unsafe { (*p_tab).e_tab_type } as i32 == 2) as i32 != 0 {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"use DROP TABLE to delete table %s".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p_tab).z_name })
                    };
                    break '__b72;
                }
                if (is_view == 0) as i32 != 0 &&
                        unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"use DROP VIEW to delete view %s".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p_tab).z_name })
                    };
                    break '__b72;
                }

                /// Generate code to remove the table from the schema table
                ///* on disk.
                (v = unsafe { sqlite3_get_vdbe(p_parse) });
                if !(v).is_null() {
                    sqlite3_begin_write_operation(p_parse, 1, i_db);
                    if (is_view == 0) as i32 != 0 {
                        sqlite3_clear_stat_tables(p_parse, i_db,
                            c"tbl".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p_tab).z_name } as *const i8);
                        unsafe { sqlite3_fk_drop_table(p_parse, p_name, p_tab) };
                    }
                    sqlite3_code_drop_table(p_parse, p_tab, i_db, is_view);
                }
                break '__c72;
            }
            if !(false) { break '__b72; }
        }

        /// If pTab is a virtual table, call ViewGetColumnNames() to ensure
        ///* it is initialized.
        /// Ensure DROP TABLE is not used on a view, and DROP VIEW is not used
        ///* on a table.
        /// Generate code to remove the table from the schema table
        ///* on disk.
        sqlite3_src_list_delete(db, p_name);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_delete_table_generic(db: *mut Sqlite3,
    p_table: *mut ()) -> () {
    sqlite3_delete_table(db, p_table as *mut Table);
}

///* pArray is a pointer to an array of objects. Each object in the
///* array is szEntry bytes in size. This routine uses sqlite3DbRealloc()
///* to extend the array so that there is space for a new object at the end.
///*
///* When this function is called, *pnEntry contains the current size of
///* the array (in entries - so the allocation is ((*pnEntry) * szEntry) bytes
///* in total).
///*
///* If the realloc() is successful (i.e. if no OOM condition occurs), the
///* space allocated for the new object is zeroed, *pnEntry updated to
///* reflect the new size of the array and a pointer to the new allocation
///* returned. *pIdx is set to the index of the new array entry in this case.
///*
///* Otherwise, if the realloc() fails, *pIdx is set to -1, *pnEntry remains
///* unchanged and a copy of pArray returned.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_array_allocate(db: *mut Sqlite3,
    mut p_array: *mut (), sz_entry: i32, pn_entry: &mut i32, p_idx: &mut i32)
    -> *mut () {
    let mut z: *mut i8 = core::ptr::null_mut();
    let n: Sqlite3Int64 = { *p_idx = *pn_entry; *p_idx } as Sqlite3Int64;
    if n & n - 1 as Sqlite3Int64 == 0 as i64 {
        let sz: Sqlite3Int64 =
            if n == 0 as i64 {
                1 as Sqlite3Int64
            } else { 2 as Sqlite3Int64 * n };
        let p_new: *mut () =
            unsafe {
                sqlite3_db_realloc(db, p_array,
                    (sz * sz_entry as Sqlite3Int64) as u64)
            };
        if p_new == core::ptr::null_mut() { *p_idx = -1; return p_array; }
        p_array = p_new;
    }
    z = p_array as *mut i8;
    unsafe {
        memset(unsafe {
                    &raw mut *z.offset((n * sz_entry as Sqlite3Int64) as isize)
                } as *mut (), 0, sz_entry as u64)
    };
    { let __p = &mut *pn_entry; *__p += 1; *__p };
    return p_array;
}

///* Append a new element to the given IdList.  Create a new IdList if
///* need be.
///*
///* A new IdList is returned, or NULL if malloc() fails.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_id_list_append(p_parse: *mut Parse,
    mut p_list: *mut IdList, p_token: *mut Token) -> *mut IdList {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let mut i: i32 = 0;
    if p_list == core::ptr::null_mut() {
        p_list =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::offset_of!(IdList, a) as u64 +
                            1 as u64 * core::mem::size_of::<IdListItem>() as u64)
                } as *mut IdList;
        if p_list == core::ptr::null_mut() { return core::ptr::null_mut(); }
    } else {
        let mut p_new: *mut IdList = core::ptr::null_mut();
        p_new =
            unsafe {
                    sqlite3_db_realloc(db, p_list as *mut (),
                        core::mem::offset_of!(IdList, a) as u64 +
                            (unsafe { (*p_list).n_id } + 1) as u64 *
                                core::mem::size_of::<IdListItem>() as u64)
                } as *mut IdList;
        if p_new == core::ptr::null_mut() {
            sqlite3_id_list_delete(db, p_list);
            return core::ptr::null_mut();
        }
        p_list = p_new;
    }
    i =
        {
            let __p = unsafe { &mut (*p_list).n_id };
            let __t = *__p;
            *__p += 1;
            __t
        };
    unsafe {
        (*(unsafe { (*p_list).a.as_ptr() } as
                            *mut IdListItem).offset(i as isize)).z_name =
            sqlite3_name_from_token(db, p_token as *const Token)
    };
    if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 &&
            !(unsafe {
                            (*(unsafe { (*p_list).a.as_ptr() } as
                                            *mut IdListItem).offset(i as isize)).z_name
                        }).is_null() {
        unsafe {
            sqlite3_rename_token_map(p_parse,
                unsafe {
                            (*(unsafe { (*p_list).a.as_ptr() } as
                                            *mut IdListItem).offset(i as isize)).z_name
                        } as *mut () as *const (), p_token as *const Token)
        };
    }
    return p_list;
}

///* Return the index in pList of the identifier named zId.  Return -1
///* if not found.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_id_list_index(p_list: &IdList, z_name: *const i8)
    -> i32 {
    let mut i: i32 = 0;
    { let _ = 0; };
    {
        i = 0;
        '__b73: loop {
            if !(i < (*p_list).n_id) { break '__b73; }
            '__c73: loop {
                if unsafe {
                            sqlite3_str_i_cmp(unsafe {
                                        (*((*p_list).a.as_ptr() as
                                                        *mut IdListItem).offset(i as isize)).z_name
                                    } as *const i8, z_name)
                        } == 0 {
                    return i;
                }
                break '__c73;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return -1;
}

///* Expand the space allocated for the given SrcList object by
///* creating nExtra new slots beginning at iStart.  iStart is zero based.
///* New slots are zeroed.
///*
///* For example, suppose a SrcList initially contains two entries: A,B.
///* To append 3 new entries onto the end, do this:
///*
///*    sqlite3SrcListEnlarge(db, pSrclist, 3, 2);
///*
///* After the call above it would contain:  A, B, nil, nil, nil.
///* If the iStart argument had been 1 instead of 2, then the result
///* would have been:  A, nil, nil, nil, B.  To prepend the new slots,
///* the iStart value would be 0.  The result then would
///* be: nil, nil, nil, A, B.
///*
///* If a memory allocation fails or the SrcList becomes too large, leave
///* the original SrcList unchanged, return NULL, and leave an error message
///* in pParse.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_src_list_enlarge(p_parse: *mut Parse,
    mut p_src: *mut SrcList, n_extra: i32, i_start: i32) -> *mut SrcList {
    let mut i: i32 = 0;

    /// Sanity checking on calling parameters
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_src).n_src } as u32 + n_extra as u32 >
            unsafe { (*p_src).n_alloc } {
        let mut p_new: *mut SrcList = core::ptr::null_mut();
        let mut n_alloc: Sqlite3Int64 =
            2 as Sqlite3Int64 * unsafe { (*p_src).n_src } as Sqlite3Int64 +
                n_extra as Sqlite3Int64;
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        if unsafe { (*p_src).n_src } + n_extra >= 200 {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"too many FROM clause terms, max: %d".as_ptr() as *mut i8
                        as *const i8, 200)
            };
            return core::ptr::null_mut();
        }
        if n_alloc > 200 as i64 { n_alloc = 200 as Sqlite3Int64; }
        p_new =
            unsafe {
                    sqlite3_db_realloc(db, p_src as *mut (),
                        core::mem::offset_of!(SrcList, a) as u64 +
                            n_alloc as u64 * core::mem::size_of::<SrcItem>() as u64)
                } as *mut SrcList;
        if p_new == core::ptr::null_mut() {
            { let _ = 0; };
            return core::ptr::null_mut();
        }
        p_src = p_new;
        unsafe { (*p_src).n_alloc = n_alloc as u32 };
    }
    {
        i = unsafe { (*p_src).n_src } - 1;
        '__b74: loop {
            if !(i >= i_start) { break '__b74; }
            '__c74: loop {
                unsafe {
                    *(unsafe { (*p_src).a.as_ptr() } as
                                    *mut SrcItem).offset((i + n_extra) as isize) =
                        unsafe {
                            *(unsafe { (*p_src).a.as_ptr() } as
                                        *mut SrcItem).offset(i as isize)
                        }
                };
                break '__c74;
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
        }
    }
    unsafe { (*p_src).n_src += n_extra };

    /// Zero the newly allocated slots
    unsafe {
        memset(unsafe {
                    &raw mut *(unsafe { (*p_src).a.as_ptr() } as
                                    *mut SrcItem).offset(i_start as isize)
                } as *mut (), 0,
            core::mem::size_of::<SrcItem>() as u64 * n_extra as u64)
    };
    {
        i = i_start;
        '__b75: loop {
            if !(i < i_start + n_extra) { break '__b75; }
            '__c75: loop {
                unsafe {
                    (*(unsafe { (*p_src).a.as_ptr() } as
                                        *mut SrcItem).offset(i as isize)).i_cursor = -1
                };
                break '__c75;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }

    /// Return a pointer to the enlarged SrcList
    return p_src;
}

///* Append the contents of SrcList p2 to SrcList p1 and return the resulting
///* SrcList. Or, if an error occurs, return NULL. In all cases, p1 and p2
///* are deleted by this function.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_list_append_list(p_parse: *mut Parse,
    mut p1: *mut SrcList, p2: *mut SrcList) -> *mut SrcList {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if !(p2).is_null() {
        let n_old: i32 = unsafe { (*p1).n_src };
        let p_new: *mut SrcList =
            sqlite3_src_list_enlarge(p_parse, p1, unsafe { (*p2).n_src },
                n_old);
        if p_new == core::ptr::null_mut() {
            sqlite3_src_list_delete(unsafe { (*p_parse).db }, p2);
        } else {
            p1 = p_new;
            unsafe {
                memcpy(unsafe {
                            &raw mut *(unsafe { (*p1).a.as_ptr() } as
                                            *mut SrcItem).offset(n_old as isize)
                        } as *mut (),
                    unsafe { (*p2).a.as_ptr() } as *mut SrcItem as *const (),
                    unsafe { (*p2).n_src } as u64 *
                        core::mem::size_of::<SrcItem>() as u64)
            };
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                (*(unsafe { (*p1).a.as_ptr() } as
                                        *mut SrcItem).offset(0 as isize)).fg.jointype |=
                    (64 &
                            unsafe {
                                    (*(unsafe { (*p2).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).fg.jointype
                                } as i32) as u8
            };
            unsafe {
                sqlite3_db_free(unsafe { (*p_parse).db }, p2 as *mut ())
            };
        }
    }
    return p1;
}

///* Append a new table name to the given SrcList.  Create a new SrcList if
///* need be.  A new entry is created in the SrcList even if pTable is NULL.
///*
///* A SrcList is returned, or NULL if there is an OOM error or if the
///* SrcList grows to large.  The returned
///* SrcList might be the same as the SrcList that was input or it might be
///* a new one.  If an OOM error does occurs, then the prior value of pList
///* that is input to this routine is automatically freed.
///*
///* If pDatabase is not null, it means that the table has an optional
///* database name prefix.  Like this:  "database.table".  The pDatabase
///* points to the table name and the pTable points to the database name.
///* The SrcList.a[].zName field is filled with the table name which might
///* come from pTable (if pDatabase is NULL) or from pDatabase. 
///* SrcList.a[].zDatabase is filled with the database name from pTable,
///* or with NULL if no database is specified.
///*
///* In other words, if call like this:
///*
///*         sqlite3SrcListAppend(D,A,B,0);
///*
///* Then B is a table name and the database name is unspecified.  If called
///* like this:
///*
///*         sqlite3SrcListAppend(D,A,B,C);
///*
///* Then C is the table name and B is the database name.  If C is defined
///* then so is B.  In other words, we never have a case where:
///*
///*         sqlite3SrcListAppend(D,A,0,C);
///*
///* Both pTable and pDatabase are assumed to be quoted.  They are dequoted
///* before being added to the SrcList.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_src_list_append(p_parse: *mut Parse,
    mut p_list: *mut SrcList, p_table: *mut Token, mut p_database: *mut Token)
    -> *mut SrcList {
    unsafe {
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        { let _ = 0; };

        /// Cannot have C without B
        { let _ = 0; };
        { let _ = 0; };
        db = unsafe { (*p_parse).db };
        if p_list == core::ptr::null_mut() {
            p_list =
                unsafe {
                        sqlite3_db_malloc_raw_nn(unsafe { (*p_parse).db },
                            core::mem::offset_of!(SrcList, a) as u64 +
                                1 as u64 * core::mem::size_of::<SrcItem>() as u64)
                    } as *mut SrcList;
            if p_list == core::ptr::null_mut() {
                return core::ptr::null_mut();
            }
            unsafe { (*p_list).n_alloc = 1 as u32 };
            unsafe { (*p_list).n_src = 1 };
            unsafe {
                memset(unsafe {
                            &raw mut *(unsafe { (*p_list).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as isize)
                        } as *mut (), 0, core::mem::size_of::<SrcItem>() as u64)
            };
            unsafe {
                (*(unsafe { (*p_list).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)).i_cursor = -1
            };
        } else {
            let p_new: *mut SrcList =
                sqlite3_src_list_enlarge(p_parse, p_list, 1,
                    unsafe { (*p_list).n_src });
            if p_new == core::ptr::null_mut() {
                sqlite3_src_list_delete(db, p_list);
                return core::ptr::null_mut();
            } else { p_list = p_new; }
        }
        p_item =
            unsafe {
                &mut *(unsafe { (*p_list).a.as_ptr() } as
                                *mut SrcItem).offset((unsafe { (*p_list).n_src } - 1) as
                                isize)
            };
        if !(p_database).is_null() &&
                unsafe { (*p_database).z } == core::ptr::null() {
            p_database = core::ptr::null_mut();
        }
        { let _ = 0; };
        { let _ = 0; };
        if !(p_database).is_null() {
            unsafe {
                (*p_item).z_name =
                    sqlite3_name_from_token(db, p_database as *const Token)
            };
            unsafe {
                (*p_item).u4.z_database =
                    sqlite3_name_from_token(db, p_table as *const Token)
            };
        } else {
            unsafe {
                (*p_item).z_name =
                    sqlite3_name_from_token(db, p_table as *const Token)
            };
            unsafe { (*p_item).u4.z_database = core::ptr::null_mut() };
        }
        return p_list;
    }
}

///* Remove a Subquery from a SrcItem.  Return the associated Select object.
///* The returned Select becomes the responsibility of the caller.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_subquery_detach(db: *mut Sqlite3,
    p_item: &mut SrcItem) -> *mut Select {
    unsafe {
        let mut p_sel: *mut Select = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        p_sel = unsafe { (*(*p_item).u4.p_subq).p_select };
        unsafe { sqlite3_db_free(db, (*p_item).u4.p_subq as *mut ()) };
        (*p_item).u4.p_subq = core::ptr::null_mut();
        (*p_item).fg.set_is_subquery(0 as u32 as u32);
        return p_sel;
    }
}

///* Attach a Subquery object to pItem->uv.pSubq.  Set the
///* pSelect value but leave all the other values initialized
///* to zero.
///*
///* A copy of the Select object is made if dupSelect is true, and the
///* SrcItem takes responsibility for deleting the copy.  If dupSelect is
///* false, ownership of the Select passes to the SrcItem.  Either way,
///* the SrcItem will take responsibility for deleting the Select.
///*
///* When dupSelect is zero, that means the Select might get deleted right
///* away if there is an OOM error.  Beware.
///*
///* Return non-zero on success.  Return zero on an OOM error.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_item_attach_subquery(p_parse: &Parse,
    p_item: &mut SrcItem, mut p_select: *mut Select, dup_select: i32) -> i32 {
    unsafe {
        let mut p: *mut Subquery = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        if (*p_item).fg.fixed_schema() != 0 {
            (*p_item).u4.p_schema = core::ptr::null_mut();
            (*p_item).fg.set_fixed_schema(0 as u32 as u32);
        } else if (*p_item).u4.z_database != core::ptr::null_mut() {
            unsafe {
                sqlite3_db_free((*p_parse).db,
                    (*p_item).u4.z_database as *mut ())
            };
            (*p_item).u4.z_database = core::ptr::null_mut();
        }
        if dup_select != 0 {
            p_select =
                unsafe {
                    sqlite3_select_dup((*p_parse).db, p_select as *const Select,
                        0)
                };
            if p_select == core::ptr::null_mut() { return 0; }
        }
        p =
            {
                (*p_item).u4.p_subq =
                    unsafe {
                            sqlite3_db_malloc_raw_nn((*p_parse).db,
                                core::mem::size_of::<Subquery>() as u64)
                        } as *mut Subquery;
                (*p_item).u4.p_subq
            };
        if p == core::ptr::null_mut() {
            unsafe { sqlite3_select_delete((*p_parse).db, p_select) };
            return 0;
        }
        (*p_item).fg.set_is_subquery(1 as u32 as u32);
        unsafe { (*p).p_select = p_select };
        { let _ = 0; };
        unsafe {
            memset(unsafe {
                        (p as
                                *mut i8).add(core::mem::size_of::<*mut Select>() as usize)
                    } as *mut (), 0,
                core::mem::size_of::<Subquery>() as u64 -
                    core::mem::size_of::<*mut Select>() as u64)
        };
        return 1;
    }
}

///* This routine is called by the parser to add a new term to the
///* end of a growing FROM clause.  The "p" parameter is the part of
///* the FROM clause that has already been constructed.  "p" is NULL
///* if this is the first term of the FROM clause.  pTable and pDatabase
///* are the name of the table and database named in the FROM clause term.
///* pDatabase is NULL if the database name qualifier is missing - the
///* usual case.  If the term has an alias, then pAlias points to the
///* alias token.  If the term is a subquery, then pSubquery is the
///* SELECT statement that the subquery encodes.  The pTable and
///* pDatabase parameters are NULL for subqueries.  The pOn and pUsing
///* parameters are the content of the ON and USING clauses.
///*
///* Return a new SrcList which encodes is the FROM with the new
///* term added.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_list_append_from_term(p_parse: *mut Parse,
    mut p: *mut SrcList, p_table: *mut Token, p_database: *mut Token,
    p_alias: *mut Token, p_subquery: *mut Select, p_on_using: *mut OnOrUsing)
    -> *mut SrcList {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        '__b76: loop {
            '__c76: loop {
                let mut p_item: *mut SrcItem = core::ptr::null_mut();
                if (p).is_null() as i32 != 0 &&
                            p_on_using != core::ptr::null_mut() &&
                        (!(unsafe { (*p_on_using).p_on }).is_null() ||
                            !(unsafe { (*p_on_using).p_using }).is_null()) {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"a JOIN clause is required before %s".as_ptr() as *mut i8
                                as *const i8,
                            if !(unsafe { (*p_on_using).p_on }).is_null() {
                                c"ON".as_ptr() as *mut i8
                            } else { c"USING".as_ptr() as *mut i8 })
                    };
                    break '__b76;
                }
                p = sqlite3_src_list_append(p_parse, p, p_table, p_database);
                if p == core::ptr::null_mut() { break '__b76; }
                { let _ = 0; };
                p_item =
                    unsafe {
                        &mut *(unsafe { (*p).a.as_ptr() } as
                                        *mut SrcItem).offset((unsafe { (*p).n_src } - 1) as isize)
                    };
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 &&
                        !(unsafe { (*p_item).z_name }).is_null() {
                    let p_token: *const Token =
                        if !(p_database).is_null() &&
                                    !(unsafe { (*p_database).z }).is_null() {
                                p_database
                            } else { p_table } as *const Token;
                    unsafe {
                        sqlite3_rename_token_map(p_parse,
                            unsafe { (*p_item).z_name } as *const (),
                            p_token as *const Token)
                    };
                }
                { let _ = 0; };
                if unsafe { (*p_alias).n } != 0 {
                    unsafe {
                        (*p_item).z_alias =
                            sqlite3_name_from_token(db, p_alias as *const Token)
                    };
                }
                { let _ = 0; };
                if !(p_subquery).is_null() {
                    if sqlite3_src_item_attach_subquery(unsafe { &*p_parse },
                                unsafe { &mut *p_item }, p_subquery, 0) != 0 {
                        if unsafe { (*p_subquery).sel_flags } & 2048 as u32 != 0 {
                            unsafe { (*p_item).fg.set_is_nested_from(1 as u32 as u32) };
                        }
                    }
                }
                { let _ = 0; };
                { let _ = 0; };
                if p_on_using == core::ptr::null_mut() {
                    unsafe { (*p_item).u3.p_on = core::ptr::null_mut() };
                } else if !(unsafe { (*p_on_using).p_using }).is_null() {
                    unsafe { (*p_item).fg.set_is_using(1 as u32 as u32) };
                    unsafe {
                        (*p_item).u3.p_using = unsafe { (*p_on_using).p_using }
                    };
                } else {
                    unsafe {
                        (*p_item).u3.p_on = unsafe { (*p_on_using).p_on }
                    };
                }
                return p;
                break '__c76;
            }
            if !(false) { break '__b76; }
        }
        { let _ = 0; };
        unsafe { sqlite3_clear_on_or_using(db, p_on_using) };
        unsafe { sqlite3_select_delete(db, p_subquery) };
        return core::ptr::null_mut();
    }
}

///* Add an INDEXED BY or NOT INDEXED clause to the most recently added
///* element of the source-list passed as the second argument.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_src_list_indexed_by(p_parse: &Parse,
    p: *mut SrcList, p_indexed_by: *mut Token) -> () {
    unsafe {
        { let _ = 0; };
        if !(p).is_null() && unsafe { (*p_indexed_by).n } > 0 as u32 {
            let mut p_item: *mut SrcItem = core::ptr::null_mut();
            { let _ = 0; };
            p_item =
                unsafe {
                    &mut *(unsafe { (*p).a.as_ptr() } as
                                    *mut SrcItem).offset((unsafe { (*p).n_src } - 1) as isize)
                };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*p_indexed_by).n } == 1 as u32 &&
                    (unsafe { (*p_indexed_by).z }).is_null() as i32 != 0 {
                unsafe {

                    /// A "NOT INDEXED" clause was supplied. See parse.y
                    ///* construct "indexed_opt" for details.
                    (*p_item).fg.set_not_indexed(1 as u32 as u32)
                };
            } else {
                unsafe {
                    (*p_item).u1.z_indexed_by =
                        sqlite3_name_from_token((*p_parse).db,
                            p_indexed_by as *const Token)
                };
                unsafe { (*p_item).fg.set_is_indexed_by(1 as u32 as u32) };
                { let _ = 0; };
            }
        }
    }
}

///* Add the list of function arguments to the SrcList entry for a
///* table-valued-function.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_list_func_args(p_parse: &Parse, p: *mut SrcList,
    p_list: *mut ExprList) -> () {
    unsafe {
        if !(p).is_null() {
            let p_item: *mut SrcItem =
                unsafe {
                    &mut *(unsafe { (*p).a.as_ptr() } as
                                    *mut SrcItem).offset((unsafe { (*p).n_src } - 1) as isize)
                };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            unsafe { (*p_item).u1.p_func_arg = p_list };
            unsafe { (*p_item).fg.set_is_tab_func(1 as u32 as u32) };
        } else { unsafe { sqlite3_expr_list_delete((*p_parse).db, p_list) }; }
    }
}

///* When building up a FROM clause in the parser, the join operator
///* is initially attached to the left operand.  But the code generator
///* expects the join operator to be on the right operand.  This routine
///* Shifts all join operators from left to right for an entire FROM
///* clause.
///*
///* Example: Suppose the join is like this:
///*
///*           A natural cross join B
///*
///* The operator is "natural cross join".  The A and B operands are stored
///* in p->a[0] and p->a[1], respectively.  The parser initially stores the
///* operator with A.  This routine shifts that operator over to B.
///*
///* Additional changes:
///*
///*   *   All tables to the left of the right-most RIGHT JOIN are tagged with
///*       JT_LTORJ (mnemonic: Left Table Of Right Join) so that the
///*       code generator can easily tell that the table is part of
///*       the left operand of at least one RIGHT JOIN.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_list_shift_join_type(p_parse: *mut Parse,
    p: *mut SrcList) -> () {
    { let _ = p_parse; };
    if !(p).is_null() && unsafe { (*p).n_src } > 1 {
        let mut i: i32 = unsafe { (*p).n_src } - 1;
        let mut all_flags: u8 = 0 as u8;
        '__b77: loop {
            '__c77: loop {
                all_flags |=
                    {
                                unsafe {
                                    (*(unsafe { (*p).a.as_ptr() } as
                                                            *mut SrcItem).offset(i as isize)).fg.jointype =
                                        unsafe {
                                            (*(unsafe { (*p).a.as_ptr() } as
                                                                *mut SrcItem).offset((i - 1) as isize)).fg.jointype
                                        }
                                };
                                unsafe {
                                    (*(unsafe { (*p).a.as_ptr() } as
                                                        *mut SrcItem).offset(i as isize)).fg.jointype
                                }
                            } as i32 as u8;
                break '__c77;
            }
            if !({ let __p = &mut i; *__p -= 1; *__p } > 0) { break '__b77; }
        }
        unsafe {
            (*(unsafe { (*p).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)).fg.jointype = 0 as u8
        };
        if all_flags as i32 & 16 != 0 {
            {
                i = unsafe { (*p).n_src } - 1;
                '__b78: loop {
                    if !(i > 0 &&
                                    unsafe {
                                                    (*(unsafe { (*p).a.as_ptr() } as
                                                                        *mut SrcItem).offset(i as isize)).fg.jointype
                                                } as i32 & 16 == 0) {
                        break '__b78;
                    }
                    '__c78: loop { break '__c78; }
                    { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                }
            }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            { let _ = 0; };
            '__b79: loop {
                '__c79: loop {
                    unsafe {
                        (*(unsafe { (*p).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)).fg.jointype |= 64 as u8
                    };
                    break '__c79;
                }
                if !({ let __p = &mut i; *__p -= 1; *__p } >= 0) {
                    break '__b79;
                }
            }
        }
    }
}

///* This routine will drop an existing named index.  This routine
///* implements the DROP INDEX statement.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_drop_index(p_parse: *mut Parse,
    p_name: *mut SrcList, if_exists: i32) -> () {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        '__b80: loop {
            '__c80: loop {
                let mut p_index: *const Index = core::ptr::null();
                let mut v: *mut Vdbe = core::ptr::null_mut();
                let mut i_db: i32 = 0;
                if unsafe { (*db).malloc_failed } != 0 { break '__b80; }
                { let _ = 0; };

                /// Never called with prior non-OOM errors
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                if 0 != unsafe { sqlite3_read_schema(p_parse) } {
                    break '__b80;
                }
                p_index =
                    sqlite3_find_index(db,
                        unsafe {
                                (*(unsafe { (*p_name).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)).z_name
                            } as *const i8,
                        unsafe {
                                (*(unsafe { (*p_name).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).u4.z_database
                            } as *const i8);
                if p_index == core::ptr::null_mut() {
                    if (if_exists == 0) as i32 != 0 {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"no such index: %S".as_ptr() as *mut i8 as *const i8,
                                unsafe { (*p_name).a.as_ptr() } as *mut SrcItem)
                        };
                    } else {
                        sqlite3_code_verify_named_schema(p_parse,
                            unsafe {
                                    (*(unsafe { (*p_name).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).u4.z_database
                                } as *const i8);
                        sqlite3_force_not_read_only(p_parse);
                    }
                    unsafe { (*p_parse).set_check_schema(1 as Bft as u32) };
                    break '__b80;
                }
                if unsafe { (*p_index).idx_type() } as i32 != 0 {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"index associated with UNIQUE or PRIMARY KEY constraint cannot be dropped".as_ptr()
                                    as *mut i8 as *const i8, 0)
                    };
                    break '__b80;
                }
                i_db =
                    unsafe {
                        sqlite3_schema_to_index(db, unsafe { (*p_index).p_schema })
                    };
                { let _ = 0; };
                {
                    let mut code: i32 = 10;
                    let p_tab: *const Table =
                        unsafe { (*p_index).p_table } as *const Table;
                    let z_db: *const i8 =
                        unsafe {
                                (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                            } as *const i8;
                    let z_tab: *const i8 =
                        if (0 == 0) as i32 != 0 && i_db == 1 {
                                c"sqlite_temp_master".as_ptr() as *mut i8
                            } else { c"sqlite_master".as_ptr() as *mut i8 } as
                            *const i8;
                    if unsafe {
                                sqlite3_auth_check(p_parse, 9, z_tab, core::ptr::null(),
                                    z_db)
                            } != 0 {
                        break '__b80;
                    }
                    if (0 == 0) as i32 != 0 && i_db == 1 { code = 12; }
                    if unsafe {
                                sqlite3_auth_check(p_parse, code,
                                    unsafe { (*p_index).z_name } as *const i8,
                                    unsafe { (*p_tab).z_name } as *const i8, z_db)
                            } != 0 {
                        break '__b80;
                    }
                }

                /// Generate code to remove the index and from the schema table
                (v = unsafe { sqlite3_get_vdbe(p_parse) });
                if !(v).is_null() {
                    sqlite3_begin_write_operation(p_parse, 1, i_db);
                    unsafe {
                        sqlite3_nested_parse(p_parse,
                            c"DELETE FROM %Q.sqlite_master WHERE name=%Q AND type=\'index\'".as_ptr()
                                    as *mut i8 as *const i8,
                            unsafe {
                                (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                            }, unsafe { (*p_index).z_name })
                    };
                    sqlite3_clear_stat_tables(p_parse, i_db,
                        c"idx".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p_index).z_name } as *const i8);
                    sqlite3_change_cookie(unsafe { &*p_parse }, i_db);
                    destroy_root_page(p_parse,
                        unsafe { (*p_index).tnum } as i32, i_db);
                    unsafe {
                        sqlite3_vdbe_add_op4(v, 155, i_db, 0, 0,
                            unsafe { (*p_index).z_name } as *const i8, 0)
                    };
                }
                break '__c80;
            }
            if !(false) { break '__b80; }
        }

        /// Never called with prior non-OOM errors
        /// Generate code to remove the index and from the schema table
        sqlite3_src_list_delete(db, p_name);
    }
}

///* Return the preferred table name for system tables.  Translate legacy
///* names into the new preferred names, as appropriate.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_preferred_table_name(z_name: *const i8)
    -> *const i8 {
    if unsafe {
                sqlite3_strnicmp(z_name,
                    c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
            } == 0 {
        if unsafe {
                    sqlite3_str_i_cmp(unsafe { z_name.offset(7 as isize) },
                        unsafe {
                                &raw mut *(c"sqlite_master".as_ptr() as
                                                *mut i8).offset(7 as isize)
                            } as *const i8)
                } == 0 {
            return c"sqlite_schema".as_ptr() as *mut i8 as *const i8;
        }
        if unsafe {
                    sqlite3_str_i_cmp(unsafe { z_name.offset(7 as isize) },
                        unsafe {
                                &raw mut *(c"sqlite_temp_master".as_ptr() as
                                                *mut i8).offset(7 as isize)
                            } as *const i8)
                } == 0 {
            return c"sqlite_temp_schema".as_ptr() as *mut i8 as *const i8;
        }
    }
    return z_name;
}

///* Unlink the given table from the hash tables and the delete the
///* table structure with all its indices and foreign keys.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_unlink_and_delete_table(db: *mut Sqlite3, i_db: i32,
    z_tab_name: *const i8) -> () {
    unsafe {
        let mut p: *mut Table = core::ptr::null_mut();
        let mut p_db: *const Db = core::ptr::null();
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };

        /// Zero-length table names are allowed
        (p_db = unsafe { unsafe { (*db).a_db.offset(i_db as isize) } });
        p =
            unsafe {
                    sqlite3_hash_insert(unsafe {
                            &mut (*unsafe { (*p_db).p_schema }).tbl_hash
                        }, z_tab_name, core::ptr::null_mut())
                } as *mut Table;
        sqlite3_delete_table(db, p);
        unsafe { (*db).m_db_flags |= 1 as u32 };
    }
}

///* For the index called zIdxName which is found in the database iDb,
///* unlike that index from its Table then remove the index from
///* the index hash table and free all memory structures associated
///* with the index.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_unlink_and_delete_index(db: *mut Sqlite3, i_db: i32,
    z_idx_name: *const i8) -> () {
    unsafe {
        let mut p_index: *mut Index = core::ptr::null_mut();
        let mut p_hash: *mut Hash = core::ptr::null_mut();
        { let _ = 0; };
        p_hash =
            unsafe {
                &mut (*unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                }).idx_hash
            };
        p_index =
            unsafe {
                    sqlite3_hash_insert(p_hash, z_idx_name,
                        core::ptr::null_mut())
                } as *mut Index;
        if !(p_index).is_null() {
            if unsafe { (*unsafe { (*p_index).p_table }).p_index } == p_index
                {
                unsafe {
                    (*unsafe { (*p_index).p_table }).p_index =
                        unsafe { (*p_index).p_next }
                };
            } else {
                let mut p: *mut Index = core::ptr::null_mut();

                /// Justification of ALWAYS();  The index must be on the list of
                ///* indices.
                (p = unsafe { (*unsafe { (*p_index).p_table }).p_index });
                while !(p).is_null() && unsafe { (*p).p_next } != p_index {
                    p = unsafe { (*p).p_next };
                }
                if !(p).is_null() && unsafe { (*p).p_next } == p_index {
                    unsafe { (*p).p_next = unsafe { (*p_index).p_next } };
                }
            }
            sqlite3_free_index(db, p_index);
        }
        unsafe { (*db).m_db_flags |= 1 as u32 };
    }
}

///* Generate VDBE code for a BEGIN statement.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_begin_transaction(p_parse: *mut Parse, type_: i32)
    -> () {
    let mut db: *const Sqlite3 = core::ptr::null();
    let mut v: *mut Vdbe = core::ptr::null_mut();
    let mut i: i32 = 0;
    { let _ = 0; };
    db = unsafe { (*p_parse).db };
    { let _ = 0; };
    if unsafe {
                sqlite3_auth_check(p_parse, 22,
                    c"BEGIN".as_ptr() as *mut i8 as *const i8,
                    core::ptr::null(), core::ptr::null())
            } != 0 {
        return;
    }
    v = unsafe { sqlite3_get_vdbe(p_parse) };
    if (v).is_null() as i32 != 0 { return; }
    if type_ != 7 {
        {
            i = 0;
            '__b82: loop {
                if !(i < unsafe { (*db).n_db }) { break '__b82; }
                '__c82: loop {
                    let mut e_txn_type: i32 = 0;
                    let p_bt: *mut Btree =
                        unsafe { (*unsafe { (*db).a_db.offset(i as isize) }).p_bt };
                    if !(p_bt).is_null() &&
                            unsafe { sqlite3_btree_is_readonly(p_bt) } != 0 {
                        e_txn_type = 0;
                    } else if type_ == 9 {
                        e_txn_type = 2;
                    } else { e_txn_type = 1; }
                    unsafe { sqlite3_vdbe_add_op2(v, 2, i, e_txn_type) };
                    unsafe { sqlite3_vdbe_uses_btree(v, i) };
                    break '__c82;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    unsafe { sqlite3_vdbe_add_op0(v, 1) };
}

///* Generate VDBE code for a COMMIT or ROLLBACK statement.
///* Code for ROLLBACK is generated if eType==TK_ROLLBACK.  Otherwise
///* code is generated for a COMMIT.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_end_transaction(p_parse: *mut Parse, e_type: i32)
    -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    let mut is_rollback: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    is_rollback = (e_type == 12) as i32;
    if unsafe {
                sqlite3_auth_check(p_parse, 22,
                    if is_rollback != 0 {
                            c"ROLLBACK".as_ptr() as *mut i8
                        } else { c"COMMIT".as_ptr() as *mut i8 } as *const i8,
                    core::ptr::null(), core::ptr::null())
            } != 0 {
        return;
    }
    v = unsafe { sqlite3_get_vdbe(p_parse) };
    if !(v).is_null() {
        unsafe { sqlite3_vdbe_add_op2(v, 1, 1, is_rollback) };
    }
}

///* This function is called by the parser when it parses a command to create,
///* release or rollback an SQL savepoint.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_savepoint(p_parse: *mut Parse, op: i32,
    p_name: *mut Token) -> () {
    unsafe {
        let z_name: *mut i8 =
            sqlite3_name_from_token(unsafe { (*p_parse).db },
                p_name as *const Token);
        if !(z_name).is_null() {
            let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };
            { let _ = 0; };
            if (v).is_null() as i32 != 0 ||
                    unsafe {
                            sqlite3_auth_check(p_parse, 32, az[op as usize],
                                z_name as *const i8, core::ptr::null())
                        } != 0 {
                unsafe {
                    sqlite3_db_free(unsafe { (*p_parse).db }, z_name as *mut ())
                };
                return;
            }
            unsafe {
                sqlite3_vdbe_add_op4(v, 0, op, 0, 0, z_name as *const i8, -7)
            };
        }
    }
}

///* Code an OP_Halt due to non-unique rowid.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rowid_constraint(p_parse: *mut Parse, on_error: i32,
    p_tab: &Table) -> () {
    let mut z_msg: *mut i8 = core::ptr::null_mut();
    let mut rc: i32 = 0;
    if (*p_tab).i_p_key as i32 >= 0 {
        z_msg =
            unsafe {
                sqlite3_m_printf(unsafe { (*p_parse).db },
                    c"%s.%s".as_ptr() as *mut i8 as *const i8, (*p_tab).z_name,
                    unsafe {
                        (*(*p_tab).a_col.offset((*p_tab).i_p_key as
                                        isize)).z_cn_name
                    })
            };
        rc = 19 | 6 << 8;
    } else {
        z_msg =
            unsafe {
                sqlite3_m_printf(unsafe { (*p_parse).db },
                    c"%s.rowid".as_ptr() as *mut i8 as *const i8,
                    (*p_tab).z_name)
            };
        rc = 19 | 10 << 8;
    }
    sqlite3_halt_constraint(p_parse, rc, on_error, z_msg, -7 as i8, 2 as u8);
}

///* This routine is called to create a new foreign key on the table
///* currently under construction.  pFromCol determines which columns
///* in the current table point to the foreign key.  If pFromCol==0 then
///* connect the key to the last column inserted.  pTo is the name of
///* the table referred to (a.k.a the "parent" table).  pToCol is a list
///* of tables in the parent pTo table.  flags contains all
///* information about the conflict resolution algorithms specified
///* in the ON DELETE, ON UPDATE and ON INSERT clauses.
///*
///* An FKey structure is created and added to the table currently
///* under construction in the pParse->pNewTable field.
///*
///* The foreign key is set for IMMEDIATE processing.  A subsequent call
///* to sqlite3DeferForeignKey() might change this to DEFERRED.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_create_foreign_key(p_parse: *mut Parse,
    p_from_col: *mut ExprList, p_to: *mut Token, p_to_col: *mut ExprList,
    flags: i32) -> () {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_f_key: *mut FKey = core::ptr::null_mut();
        let mut p_next_to: *mut FKey = core::ptr::null_mut();
        let mut p: *mut Table = core::ptr::null_mut();
        let mut n_byte: i64 = 0 as i64;
        let mut i: i32 = 0;
        let mut n_col: i32 = 0;
        let mut z: *mut i8 = core::ptr::null_mut();
        let mut i_col: i32 = 0;
        let mut j: i32 = 0;
        let mut n: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s84:
                {
                match __state {
                    0 => { db = unsafe { (*p_parse).db }; __state = 3; }
                    2 => {
                        unsafe { sqlite3_db_free(db, p_f_key as *mut ()) };
                        __state = 90;
                    }
                    3 => { p_f_key = core::ptr::null_mut(); __state = 4; }
                    4 => { __state = 5; }
                    5 => { p = unsafe { (*p_parse).p_new_table }; __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { { let _ = 0; }; __state = 11; }
                    11 => {
                        if p == core::ptr::null_mut() ||
                                unsafe { (*p_parse).e_parse_mode } as i32 == 1 {
                            __state = 13;
                        } else { __state = 12; }
                    }
                    12 => {
                        if p_from_col == core::ptr::null_mut() {
                            __state = 15;
                        } else { __state = 16; }
                    }
                    13 => { __state = 2; }
                    14 => {
                        n_byte =
                            (core::mem::offset_of!(FKey, a_col) as u64 +
                                            n_col as u64 * core::mem::size_of::<SColMap>() as u64 +
                                        unsafe { (*p_to).n } as u64 + 1 as u64) as i64;
                        __state = 26;
                    }
                    15 => {
                        i_col = unsafe { (*p).n_col } as i32 - 1;
                        __state = 17;
                    }
                    16 => {
                        if !(p_to_col).is_null() &&
                                unsafe { (*p_to_col).n_expr } !=
                                    unsafe { (*p_from_col).n_expr } {
                            __state = 23;
                        } else { __state = 24; }
                    }
                    17 => {
                        if i_col < 0 { __state = 19; } else { __state = 18; }
                    }
                    18 => {
                        if !(p_to_col).is_null() &&
                                unsafe { (*p_to_col).n_expr } != 1 {
                            __state = 21;
                        } else { __state = 20; }
                    }
                    19 => { __state = 2; }
                    20 => { n_col = 1; __state = 14; }
                    21 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"foreign key on %s should reference only one column of table %T".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe {
                                    (*unsafe { (*p).a_col.offset(i_col as isize) }).z_cn_name
                                }, p_to)
                        };
                        __state = 22;
                    }
                    22 => { __state = 2; }
                    23 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"number of columns in foreign key does not match the number of columns in the referenced table".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 25;
                    }
                    24 => {
                        n_col = unsafe { (*p_from_col).n_expr };
                        __state = 14;
                    }
                    25 => { __state = 2; }
                    26 => {
                        if !(p_to_col).is_null() {
                            __state = 28;
                        } else { __state = 27; }
                    }
                    27 => {
                        p_f_key =
                            unsafe { sqlite3_db_malloc_zero(db, n_byte as u64) } as
                                *mut FKey;
                        __state = 32;
                    }
                    28 => { i = 0; __state = 29; }
                    29 => {
                        if i < unsafe { (*p_to_col).n_expr } {
                            __state = 30;
                        } else { __state = 27; }
                    }
                    30 => {
                        n_byte +=
                            (unsafe {
                                        sqlite3_strlen30(unsafe {
                                                    (*(unsafe { (*p_to_col).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i as isize)).z_e_name
                                                } as *const i8)
                                    } + 1) as i64;
                        __state = 31;
                    }
                    31 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 29;
                    }
                    32 => {
                        if p_f_key == core::ptr::null_mut() {
                            __state = 34;
                        } else { __state = 33; }
                    }
                    33 => { unsafe { (*p_f_key).p_from = p }; __state = 35; }
                    34 => { __state = 2; }
                    35 => { { let _ = 0; }; __state = 36; }
                    36 => {
                        unsafe {
                            (*p_f_key).p_next_from = unsafe { (*p).u.tab.p_f_key }
                        };
                        __state = 37;
                    }
                    37 => {
                        z =
                            unsafe {
                                    &raw mut *(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                    *mut SColMap).offset(n_col as isize)
                                } as *mut i8;
                        __state = 38;
                    }
                    38 => { unsafe { (*p_f_key).z_to = z }; __state = 39; }
                    39 => {
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                            __state = 41;
                        } else { __state = 40; }
                    }
                    40 => {
                        unsafe {
                            memcpy(z as *mut (), unsafe { (*p_to).z } as *const (),
                                unsafe { (*p_to).n } as u64)
                        };
                        __state = 42;
                    }
                    41 => {
                        unsafe {
                            sqlite3_rename_token_map(p_parse, z as *mut () as *const (),
                                p_to as *const Token)
                        };
                        __state = 40;
                    }
                    42 => {
                        unsafe { *z.add(unsafe { (*p_to).n } as usize) = 0 as i8 };
                        __state = 43;
                    }
                    43 => { unsafe { sqlite3_dequote(z) }; __state = 44; }
                    44 => {
                        {
                            let __n = unsafe { (*p_to).n } + 1 as u32;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        __state = 45;
                    }
                    45 => { unsafe { (*p_f_key).n_col = n_col }; __state = 46; }
                    46 => {
                        if p_from_col == core::ptr::null_mut() {
                            __state = 48;
                        } else { __state = 49; }
                    }
                    47 => {
                        if !(p_to_col).is_null() {
                            __state = 65;
                        } else { __state = 64; }
                    }
                    48 => {
                        unsafe {
                            (*(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                *mut SColMap).offset(0 as isize)).i_from =
                                unsafe { (*p).n_col } as i32 - 1
                        };
                        __state = 47;
                    }
                    49 => { i = 0; __state = 50; }
                    50 => {
                        if i < n_col { __state = 51; } else { __state = 47; }
                    }
                    51 => { __state = 53; }
                    52 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 50;
                    }
                    53 => { j = 0; __state = 55; }
                    54 => {
                        if j >= unsafe { (*p).n_col } as i32 {
                            __state = 61;
                        } else { __state = 60; }
                    }
                    55 => {
                        if j < unsafe { (*p).n_col } as i32 {
                            __state = 56;
                        } else { __state = 54; }
                    }
                    56 => {
                        if unsafe {
                                    sqlite3_str_i_cmp(unsafe {
                                                (*unsafe { (*p).a_col.offset(j as isize) }).z_cn_name
                                            } as *const i8,
                                        unsafe {
                                                (*(unsafe { (*p_from_col).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).z_e_name
                                            } as *const i8)
                                } == 0 {
                            __state = 58;
                        } else { __state = 57; }
                    }
                    57 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 55;
                    }
                    58 => {
                        unsafe {
                            (*(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                *mut SColMap).offset(i as isize)).i_from = j
                        };
                        __state = 59;
                    }
                    59 => { __state = 54; }
                    60 => {
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                            __state = 63;
                        } else { __state = 52; }
                    }
                    61 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"unknown column \"%s\" in foreign key definition".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe {
                                    (*(unsafe { (*p_from_col).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).z_e_name
                                })
                        };
                        __state = 62;
                    }
                    62 => { __state = 2; }
                    63 => {
                        unsafe {
                            sqlite3_rename_token_remap(p_parse,
                                unsafe {
                                        &raw mut *(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                        *mut SColMap).offset(i as isize)
                                    } as *const (),
                                unsafe {
                                        (*(unsafe { (*p_from_col).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).z_e_name
                                    } as *const ())
                        };
                        __state = 52;
                    }
                    64 => {
                        unsafe { (*p_f_key).is_deferred = 0 as u8 };
                        __state = 75;
                    }
                    65 => { i = 0; __state = 66; }
                    66 => {
                        if i < n_col { __state = 67; } else { __state = 64; }
                    }
                    67 => {
                        n =
                            unsafe {
                                sqlite3_strlen30(unsafe {
                                            (*(unsafe { (*p_to_col).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).z_e_name
                                        } as *const i8)
                            };
                        __state = 69;
                    }
                    68 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 66;
                    }
                    69 => {
                        unsafe {
                            (*(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                *mut SColMap).offset(i as isize)).z_col = z
                        };
                        __state = 70;
                    }
                    70 => {
                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                            __state = 72;
                        } else { __state = 71; }
                    }
                    71 => {
                        unsafe {
                            memcpy(z as *mut (),
                                unsafe {
                                        (*(unsafe { (*p_to_col).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).z_e_name
                                    } as *const (), n as u64)
                        };
                        __state = 73;
                    }
                    72 => {
                        unsafe {
                            sqlite3_rename_token_remap(p_parse, z as *const (),
                                unsafe {
                                        (*(unsafe { (*p_to_col).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).z_e_name
                                    } as *const ())
                        };
                        __state = 71;
                    }
                    73 => {
                        unsafe { *z.offset(n as isize) = 0 as i8 };
                        __state = 74;
                    }
                    74 => {
                        {
                            let __n = n + 1;
                            let __p = &mut z;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 68;
                    }
                    75 => {
                        unsafe {
                            (*p_f_key).a_action[0 as usize] = (flags & 255) as u8
                        };
                        __state = 76;
                    }
                    76 => {
                        unsafe {
                            (*p_f_key).a_action[1 as usize] = (flags >> 8 & 255) as u8
                        };
                        __state = 77;
                    }
                    77 => { { let _ = 0; }; __state = 78; }
                    78 => {
                        p_next_to =
                            unsafe {
                                    sqlite3_hash_insert(unsafe {
                                            &mut (*unsafe { (*p).p_schema }).fkey_hash
                                        }, unsafe { (*p_f_key).z_to } as *const i8,
                                        p_f_key as *mut ())
                                } as *mut FKey;
                        __state = 79;
                    }
                    79 => {
                        if p_next_to == p_f_key {
                            __state = 81;
                        } else { __state = 80; }
                    }
                    80 => {
                        if !(p_next_to).is_null() {
                            __state = 84;
                        } else { __state = 83; }
                    }
                    81 => { unsafe { sqlite3_oom_fault(db) }; __state = 82; }
                    82 => { __state = 2; }
                    83 => { { let _ = 0; }; __state = 87; }
                    84 => { { let _ = 0; }; __state = 85; }
                    85 => {
                        unsafe { (*p_f_key).p_next_to = p_next_to };
                        __state = 86;
                    }
                    86 => {
                        unsafe { (*p_next_to).p_prev_to = p_f_key };
                        __state = 83;
                    }
                    87 => {
                        unsafe { (*p).u.tab.p_f_key = p_f_key };
                        __state = 88;
                    }
                    88 => { p_f_key = core::ptr::null_mut(); __state = 89; }
                    89 => { __state = 2; }
                    90 => {
                        unsafe { sqlite3_expr_list_delete(db, p_from_col) };
                        __state = 91;
                    }
                    91 => {
                        unsafe { sqlite3_expr_list_delete(db, p_to_col) };
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

///* This routine is called when an INITIALLY IMMEDIATE or INITIALLY DEFERRED
///* clause is seen as part of a foreign key definition.  The isDeferred
///* parameter is 1 for INITIALLY DEFERRED and 0 for INITIALLY IMMEDIATE.
///* The behavior of the most recently created foreign key is adjusted
///* accordingly.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_defer_foreign_key(p_parse: &Parse, is_deferred: i32)
    -> () {
    unsafe {
        let mut p_tab: *const Table = core::ptr::null();
        let mut p_f_key: *mut FKey = core::ptr::null_mut();
        if { p_tab = (*p_parse).p_new_table; p_tab } == core::ptr::null_mut()
            {
            return;
        }
        if !(unsafe { (*p_tab).e_tab_type } as i32 == 0) as i32 != 0 {
            return;
        }
        if { p_f_key = unsafe { (*p_tab).u.tab.p_f_key }; p_f_key } ==
                core::ptr::null_mut() {
            return;
        }
        { let _ = 0; };

        /// EV: R-30323-21917
        unsafe { (*p_f_key).is_deferred = is_deferred as u8 };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_root_page_moved(db: &Sqlite3, i_db: i32,
    i_from: Pgno, i_to: Pgno) -> () {
    unsafe {
        let mut p_elem: *const HashElem = core::ptr::null();
        let mut p_hash: *const Hash = core::ptr::null();
        let mut p_db: *const Db = core::ptr::null();
        { let _ = 0; };
        p_db = unsafe { (*db).a_db.offset(i_db as isize) };
        p_hash = unsafe { &mut (*unsafe { (*p_db).p_schema }).tbl_hash };
        {
            p_elem = unsafe { (*p_hash).first };
            '__b85: loop {
                if !(!(p_elem).is_null()) { break '__b85; }
                '__c85: loop {
                    let p_tab: *mut Table =
                        unsafe { (*p_elem).data } as *mut Table;
                    if unsafe { (*p_tab).tnum } == i_from {
                        unsafe { (*p_tab).tnum = i_to };
                    }
                    break '__c85;
                }
                p_elem = unsafe { (*p_elem).next };
            }
        }
        p_hash = unsafe { &mut (*unsafe { (*p_db).p_schema }).idx_hash };
        {
            p_elem = unsafe { (*p_hash).first };
            '__b86: loop {
                if !(!(p_elem).is_null()) { break '__b86; }
                '__c86: loop {
                    let p_idx: *mut Index =
                        unsafe { (*p_elem).data } as *mut Index;
                    if unsafe { (*p_idx).tnum } == i_from {
                        unsafe { (*p_idx).tnum = i_to };
                    }
                    break '__c86;
                }
                p_elem = unsafe { (*p_elem).next };
            }
        }
    }
}

extern "C" fn collation_match(z_coll_1: *const i8, p_index_1: &Index) -> i32 {
    let mut i: i32 = 0;
    { let _ = 0; };
    {
        i = 0;
        '__b87: loop {
            if !(i < (*p_index_1).n_column as i32) { break '__b87; }
            '__c87: loop {
                let z: *const i8 =
                    unsafe { *(*p_index_1).az_coll.offset(i as isize) };
                { let _ = 0; };
                if 0 == unsafe { sqlite3_str_i_cmp(z, z_coll_1) } {
                    return 1;
                }
                break '__c87;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_reindex(p_parse: *mut Parse, p_name1: *mut Token,
    p_name2: *mut Token) -> () {
    unsafe {
        let mut z: *mut i8 = core::ptr::null_mut();
        /// Name of a table or index or collation
        let mut z_db: *const i8 = core::ptr::null();
        /// Name of the database
        let mut i_re_db: i32 = -1;
        /// The database index number
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        /// The database connection
        let mut p_obj_name: *mut Token = core::ptr::null_mut();
        /// Name of the table or index to be reindexed
        let mut b_match: i32 = 0;
        /// At least one name match
        let mut z_coll: *const i8 = core::ptr::null();
        /// Rebuild indexes using this collation
        let mut p_re_tab: *mut Table = core::ptr::null_mut();
        /// Rebuild all indexes of this table
        let mut p_re_index: *mut Index = core::ptr::null_mut();
        /// Rebuild this index
        let mut is_expr_idx: i32 = 0;
        /// Rebuild all expression indexes
        let mut b_all: i32 = 0;
        if 0 != unsafe { sqlite3_read_schema(p_parse) } { return; }
        if p_name1 == core::ptr::null_mut() {

            /// rebuild all indexes
            (b_match = 1);
            b_all = 1;
        } else if p_name2 == core::ptr::null_mut() ||
                unsafe { (*p_name2).z } == core::ptr::null() {
            { let _ = 0; };
            z =
                sqlite3_name_from_token(unsafe { (*p_parse).db },
                    p_name1 as *const Token);
            if z == core::ptr::null_mut() { return; }
        } else {
            i_re_db =
                sqlite3_two_part_name(p_parse, p_name1, p_name2,
                    &mut p_obj_name);
            if i_re_db < 0 { return; }
            z = sqlite3_name_from_token(db, p_obj_name as *const Token);
            if z == core::ptr::null_mut() { return; }
            z_db =
                unsafe {
                        (*unsafe {
                                    (*db).a_db.offset(i_re_db as isize)
                                }).z_db_s_name
                    } as *const i8;
        }
        if (b_all == 0) as i32 != 0 {
            if z_db == core::ptr::null() &&
                    unsafe {
                            sqlite3_str_i_cmp(z as *const i8,
                                c"expressions".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                is_expr_idx = 1;
                b_match = 1;
            }
            if z_db == core::ptr::null() &&
                    unsafe {
                            sqlite3_find_coll_seq(db, unsafe { (*db).enc },
                                z as *const i8, 0)
                        } != core::ptr::null_mut() {
                z_coll = z as *const i8;
                b_match = 1;
            }
            if z_coll == core::ptr::null() &&
                    {
                            p_re_tab =
                                sqlite3_find_table(unsafe { &*db }, z as *const i8, z_db);
                            p_re_tab
                        } != core::ptr::null_mut() {
                b_match = 1;
            }
            if z_coll == core::ptr::null() &&
                    {
                            p_re_index = sqlite3_find_index(db, z as *const i8, z_db);
                            p_re_index
                        } != core::ptr::null_mut() {
                b_match = 1;
            }
        }
        if b_match != 0 {
            let mut i_db: i32 = 0;
            let mut k: *mut HashElem = core::ptr::null_mut();
            let mut p_tab: *mut Table = core::ptr::null_mut();
            let mut p_idx: *mut Index = core::ptr::null_mut();
            let mut p_db: *const Db = core::ptr::null();
            {
                { i_db = 0; p_db = unsafe { (*db).a_db } };
                '__b88: loop {
                    if !(i_db < unsafe { (*db).n_db }) { break '__b88; }
                    '__c88: loop {
                        { let _ = 0; };
                        if i_re_db >= 0 && i_re_db != i_db { break '__c88; }
                        {
                            k =
                                unsafe {
                                    (*unsafe {
                                                    &mut (*unsafe { (*p_db).p_schema }).tbl_hash
                                                }).first
                                };
                            '__b89: loop {
                                if !(!(k).is_null()) { break '__b89; }
                                '__c89: loop {
                                    p_tab = unsafe { (*k).data } as *mut Table;
                                    if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                                        break '__c89;
                                    }
                                    {
                                        p_idx = unsafe { (*p_tab).p_index };
                                        '__b90: loop {
                                            if !(!(p_idx).is_null()) { break '__b90; }
                                            '__c90: loop {
                                                if b_all != 0 || p_tab == p_re_tab || p_idx == p_re_index ||
                                                            is_expr_idx != 0 && unsafe { (*p_idx).b_has_expr() } != 0 ||
                                                        z_coll != core::ptr::null() &&
                                                            collation_match(z_coll, unsafe { &*p_idx }) != 0 {
                                                    sqlite3_begin_write_operation(p_parse, 0, i_db);
                                                    sqlite3_refill_index(p_parse, p_idx, -1);
                                                }
                                                break '__c90;
                                            }
                                            p_idx = unsafe { (*p_idx).p_next };
                                        }
                                    }
                                    break '__c89;
                                }
                                k = unsafe { (*k).next };
                            }
                        }
                        break '__c88;
                    }
                    {
                        { let __p = &mut i_db; let __t = *__p; *__p += 1; __t };
                        {
                            let __p = &mut p_db;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
        } else {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"unable to identify the object to be reindexed".as_ptr() as
                            *mut i8 as *const i8)
            };
        }
        unsafe { sqlite3_db_free(db, z as *mut ()) };
        return;
    }
}

///* Table pTab is a virtual table.  If it the virtual table implementation
///* exists and has an xShadowName method, then loop over all other ordinary
///* tables within the same schema looking for shadow tables of pTab, and mark
///* any shadow tables seen using the TF_Shadow flag.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_mark_all_shadow_tables_of(db: &mut Sqlite3,
    p_tab: &Table) -> () {
    unsafe {
        let mut n_name: i32 = 0;
        /// Length of pTab->zName
        let mut p_mod: *const Module = core::ptr::null();
        /// Module for the virtual table
        let mut k: *const HashElem = core::ptr::null();

        /// For looping through the symbol table
        { let _ = 0; };
        p_mod =
            unsafe {
                    sqlite3_hash_find(&raw mut (*db).a_module as *const Hash,
                        unsafe { *(*p_tab).u.vtab.az_arg.offset(0 as isize) } as
                            *const i8)
                } as *mut Module;
        if p_mod == core::ptr::null_mut() { return; }
        if unsafe { (*p_mod).p_module } == core::ptr::null() { return; }
        if (unsafe { (*unsafe { (*p_mod).p_module }).i_version } as i32) < 3 {
            return;
        }
        if !unsafe { (*unsafe { (*p_mod).p_module }).x_shadow_name.is_some() }
                    as i32 != 0 {
            return;
        }
        { let _ = 0; };
        n_name = unsafe { sqlite3_strlen30((*p_tab).z_name as *const i8) };
        {
            k =
                unsafe {
                    (*unsafe { &mut (*(*p_tab).p_schema).tbl_hash }).first
                };
            '__b91: loop {
                if !(!(k).is_null()) { break '__b91; }
                '__c91: loop {
                    let p_other: *mut Table =
                        unsafe { (*k).data } as *mut Table;
                    { let _ = 0; };
                    if !(unsafe { (*p_other).e_tab_type } as i32 == 0) as i32 !=
                            0 {
                        break '__c91;
                    }
                    if unsafe { (*p_other).tab_flags } & 4096 as u32 != 0 {
                        break '__c91;
                    }
                    if unsafe {
                                        sqlite3_strnicmp(unsafe { (*p_other).z_name } as *const i8,
                                            (*p_tab).z_name as *const i8, n_name)
                                    } == 0 &&
                                unsafe {
                                            *unsafe { (*p_other).z_name.offset(n_name as isize) }
                                        } as i32 == '_' as i32 &&
                            unsafe {
                                    (unsafe {
                                            (*unsafe { (*p_mod).p_module }).x_shadow_name.unwrap()
                                        })(unsafe {
                                                unsafe {
                                                    unsafe {
                                                        (*p_other).z_name.offset(n_name as isize).offset(1 as isize)
                                                    }
                                                }
                                            } as *const i8)
                                } != 0 {
                        unsafe { (*p_other).tab_flags |= 4096 as u32 };
                    }
                    break '__c91;
                }
                k = unsafe { (*k).next };
            }
        }
    }
}

///* Create a new CTE object
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_cte_new(p_parse: &Parse, p_name: *mut Token,
    p_arglist: *mut ExprList, p_query: *mut Select, e_m10d: u8) -> *mut Cte {
    unsafe {
        let mut p_new: *mut Cte = core::ptr::null_mut();
        let db: *mut Sqlite3 = (*p_parse).db;
        p_new =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<Cte>() as u64)
                } as *mut Cte;
        { let _ = 0; };
        if unsafe { (*db).malloc_failed } != 0 {
            unsafe { sqlite3_expr_list_delete(db, p_arglist) };
            unsafe { sqlite3_select_delete(db, p_query) };
        } else {
            unsafe { (*p_new).p_select = p_query };
            unsafe { (*p_new).p_cols = p_arglist };
            unsafe {
                (*p_new).z_name =
                    sqlite3_name_from_token((*p_parse).db,
                        p_name as *const Token)
            };
            unsafe { (*p_new).e_m10d = e_m10d };
        }
        return p_new;
    }
}

///* Clear information from a Cte object, but do not deallocate storage
///* for the object itself.
extern "C" fn cte_clear(db: *mut Sqlite3, p_cte_1: &Cte) -> () {
    unsafe {
        { let _ = 0; };
        unsafe { sqlite3_expr_list_delete(db, (*p_cte_1).p_cols) };
        unsafe { sqlite3_select_delete(db, (*p_cte_1).p_select) };
        unsafe { sqlite3_db_free(db, (*p_cte_1).z_name as *mut ()) };
    }
}

///* Free the contents of the CTE object passed as the second argument.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_cte_delete(db: *mut Sqlite3, p_cte: *mut Cte)
    -> () {
    { let _ = 0; };
    cte_clear(db, unsafe { &*p_cte });
    unsafe { sqlite3_db_free(db, p_cte as *mut ()) };
}

///* This routine is invoked once per CTE by the parser while parsing a
///* WITH clause.  The CTE described by the third argument is added to
///* the WITH clause of the second argument.  If the second argument is
///* NULL, then a new WITH argument is created.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_with_add(p_parse: *mut Parse, p_with: *mut With,
    p_cte: *mut Cte) -> *mut With {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let mut p_new: *mut With = core::ptr::null_mut();
    let mut z_name: *mut i8 = core::ptr::null_mut();
    if p_cte == core::ptr::null_mut() { return p_with; }

    /// Check that the CTE name is unique within this WITH clause. If
    ///* not, store an error in the Parse structure.
    (z_name = unsafe { (*p_cte).z_name });
    if !(z_name).is_null() && !(p_with).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b92: loop {
                if !(i < unsafe { (*p_with).n_cte }) { break '__b92; }
                '__c92: loop {
                    if unsafe {
                                sqlite3_str_i_cmp(z_name as *const i8,
                                    unsafe {
                                            (*(unsafe { (*p_with).a.as_ptr() } as
                                                            *mut Cte).offset(i as isize)).z_name
                                        } as *const i8)
                            } == 0 {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"duplicate WITH table name: %s".as_ptr() as *mut i8 as
                                    *const i8, z_name)
                        };
                    }
                    break '__c92;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    if !(p_with).is_null() {
        p_new =
            unsafe {
                    sqlite3_db_realloc(db, p_with as *mut (),
                        core::mem::offset_of!(With, a) as u64 +
                            (unsafe { (*p_with).n_cte } + 1) as u64 *
                                core::mem::size_of::<Cte>() as u64)
                } as *mut With;
    } else {
        p_new =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::offset_of!(With, a) as u64 +
                            1 as u64 * core::mem::size_of::<Cte>() as u64)
                } as *mut With;
    }
    { let _ = 0; };
    if unsafe { (*db).malloc_failed } != 0 {
        sqlite3_cte_delete(db, p_cte);
        p_new = p_with;
    } else {
        unsafe {
            *(unsafe { (*p_new).a.as_ptr() } as
                            *mut Cte).offset({
                                let __p = unsafe { &mut (*p_new).n_cte };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            } as isize) = unsafe { core::ptr::read(p_cte) }
        };
        unsafe { sqlite3_db_free(db, p_cte as *mut ()) };
    }
    return p_new;
}

///* Free the contents of the With object passed as the second argument.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_with_delete(db: *mut Sqlite3, p_with: *mut With)
    -> () {
    if !(p_with).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b93: loop {
                if !(i < unsafe { (*p_with).n_cte }) { break '__b93; }
                '__c93: loop {
                    cte_clear(db,
                        unsafe {
                            &*(unsafe { (*p_with).a.as_ptr() } as
                                            *mut Cte).offset(i as isize)
                        });
                    break '__c93;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_db_free(db, p_with as *mut ()) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_with_delete_generic(db: *mut Sqlite3,
    p_with: *mut ()) -> () {
    sqlite3_with_delete(db, p_with as *mut With);
}

static flags_1: i32 = (2 | 4 | 16 | 8 | 512) as i32;

static a_code: [u8; 4] = [2 as u8, 4 as u8, 8 as u8, 6 as u8];

static null_row: [i8; 6] =
    [6 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8];

static a_val: [i16; 5] =
    [33 as i16, 32 as i16, 30 as i16, 28 as i16, 26 as i16];

static mut az_type: [*const i8; 6] =
    [c"".as_ptr() as *const i8, c" TEXT".as_ptr() as *const i8,
            c" NUM".as_ptr() as *const i8, c" INT".as_ptr() as *const i8,
            c" REAL".as_ptr() as *const i8, c" NUM".as_ptr() as *const i8];

static mut az: [*const i8; 3] =
    [c"BEGIN".as_ptr() as *const i8, c"RELEASE".as_ptr() as *const i8,
            c"ROLLBACK".as_ptr() as *const i8];

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
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_get_v_table(_: *mut Sqlite3, _: *mut Table)
    -> *mut VTable;
    fn sqlite3_autoincrement_begin(p_parse_1: *mut Parse)
    -> ();
    fn sqlite3_expr_code(_: *mut Parse, _: *mut Expr, _: i32)
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
    fn sqlite3_schema_clear(_: *mut ())
    -> ();
    fn sqlite3_vtab_unlock_list(_: *mut Sqlite3)
    -> ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
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
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_rename_token_map(_: *mut Parse, _: *const (), _: *const Token)
    -> *const ();
    static mut sqlite3Config: Sqlite3Config;
    fn strrchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_db_is_named(db: *mut Sqlite3, i_db_1: i32, z_name_1: *const i8)
    -> i32;
    static sqlite3_ctype_map: [u8; 0];
    static sqlite3_std_type_len: [u8; 0];
    static mut sqlite3_std_type: [*const i8; 0];
    static sqlite3_std_type_affinity: [i8; 0];
    fn sqlite3_column_index(p_tab_1: *mut Table, z_col_1: *const i8)
    -> i32;
    fn sqlite3_str_i_hash(_: *const i8)
    -> u8;
    static sqlite3_upper_to_lower: [u8; 0];
    fn sqlite3_get_int32(_: *const i8, _: *mut i32)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_expr_skip_collate(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_rename_token_remap(_: *mut Parse, p_to_1: *const (),
    p_from_1: *const ())
    -> ();
    fn sqlite3_delete_index_samples(_: *mut Sqlite3, _: *mut Index)
    -> ();
    fn sqlite3_src_list_lookup(_: *mut Parse, _: *mut SrcList)
    -> *mut Table;
    fn sqlite3_fix_init(_: *mut DbFixer, _: *mut Parse, _: i32, _: *const i8,
    _: *const Token)
    -> ();
    fn sqlite3_fix_src_list(_: *mut DbFixer, _: *mut SrcList)
    -> i32;
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_json_vtab_register(_: *mut Sqlite3, _: *const i8)
    -> *mut Module;
    fn sqlite3_vtab_eponymous_table_init(_: *mut Parse, _: *mut Module)
    -> i32;
    fn sqlite3_expr_list_check_length(_: *mut Parse, _: *mut ExprList,
    _: *const i8)
    -> ();
    fn sqlite3_resolve_self_reference(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut Expr, _: *mut ExprList)
    -> i32;
    static sqlite3_str_binary: [i8; 0];
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_log_est(_: u64)
    -> LogEst;
    fn sqlite3_key_info_alloc(_: *mut Sqlite3, _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_key_info_unref(_: *mut KeyInfo)
    -> ();
    fn sqlite3_key_info_ref(_: *mut KeyInfo)
    -> *mut KeyInfo;
    fn sqlite3_open_table(_: *mut Parse, i_cur_1: i32, i_db_1: i32,
    _: *mut Table, _: i32)
    -> ();
    fn sqlite3_generate_index_key(_: *mut Parse, _: *mut Index, _: i32,
    _: i32, _: i32, _: *mut i32, _: *mut Index, _: i32)
    -> i32;
    fn sqlite3_resolve_part_idx_label(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_str_accum_init(_: *mut StrAccum, _: *mut Sqlite3, _: *mut i8,
    _: i32, _: i32)
    -> ();
    fn sqlite3_str_accum_finish(_: *mut StrAccum)
    -> *mut i8;
    fn sqlite3_select_delete(_: *mut Sqlite3, _: *mut Select)
    -> ();
    fn sqlite3_fk_delete(_: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_vtab_clear(db: *mut Sqlite3, _: *mut Table)
    -> ();
    fn sqlite3_expr_is_constant_or_function(_: *mut Expr, _: u8)
    -> i32;
    fn sqlite3_expr_dup(_: *mut Sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_rename_expr_unmap(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_select(_: *mut Parse, _: *mut Select, _: *mut SelectDest)
    -> i32;
    fn sqlite3_table_affinity(_: *mut Vdbe, _: *mut Table, _: i32)
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
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
    fn sqlite3_fix_select(_: *mut DbFixer, _: *mut Select)
    -> i32;
    fn sqlite3_select_dup(_: *mut Sqlite3, _: *const Select, _: i32)
    -> *mut Select;
    fn sqlite3_expr_list_dup(_: *mut Sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_rename_exprlist_unmap(_: *mut Parse, _: *mut ExprList)
    -> ();
    fn sqlite3_vtab_call_connect(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_fk_drop_table(_: *mut Parse, _: *mut SrcList, _: *mut Table)
    -> ();
    fn sqlite3_trigger_list(_: *mut Parse, _: *mut Table)
    -> *mut Trigger;
    fn sqlite3_drop_trigger_ptr(_: *mut Parse, _: *mut Trigger)
    -> ();
    fn sqlite3_autoincrement_end(p_parse_1: *mut Parse)
    -> ();
    fn sqlite3_insert(_: *mut Parse, _: *mut SrcList, _: *mut Select,
    _: *mut IdList, _: i32, _: *mut Upsert)
    -> ();
    fn sqlite3_compute_generated_columns(_: *mut Parse, _: i32, _: *mut Table)
    -> ();
    fn sqlite3_clear_on_or_using(_: *mut Sqlite3, _: *mut OnOrUsing)
    -> ();
    fn sqlite3_indexed_by_lookup(_: *mut Parse, _: *mut SrcItem)
    -> i32;
    fn sqlite3_select_new(_: *mut Parse, _: *mut ExprList, _: *mut SrcList,
    _: *mut Expr, _: *mut ExprList, _: *mut Expr, _: *mut ExprList, _: u32,
    _: *mut Expr)
    -> *mut Select;
    fn sqlite3_select_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_select_check_on_clauses(p_parse_1: *mut Parse,
    p_select_1: *mut Select)
    -> ();
    fn sqlite3_is_read_only(_: *mut Parse, _: *mut Table, _: *mut Trigger)
    -> i32;
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
    fn sqlite3_vacuum(_: *mut Parse, _: *mut Token, _: *mut Expr)
    -> ();
    fn sqlite3_run_vacuum(_: *mut *mut i8, _: *mut Sqlite3, _: i32,
    _: *mut Sqlite3Value)
    -> i32;
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
    fn sqlite3_src_list_dup(_: *mut Sqlite3, _: *const SrcList, _: i32)
    -> *mut SrcList;
    fn sqlite3_id_list_dup(_: *mut Sqlite3, _: *const IdList)
    -> *mut IdList;
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
    fn sqlite3_safety_check_ok(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_safety_check_sick_or_ok(_: *mut Sqlite3)
    -> i32;
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
    fn sqlite3_triggers_exist(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut ExprList, p_mask_1: *mut i32)
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
    fn sqlite3_src_item_column_used(_: *mut SrcItem, _: i32)
    -> ();
    fn sqlite3_set_join_expr(_: *mut Expr, _: i32, _: u32)
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
    fn sqlite3_expr_skip_collate_and_likely(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_check_coll_seq(_: *mut Parse, _: *mut CollSeq)
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
    static mut sqlite3a_l_tb: *const u8;
    static mut sqlite3a_e_qb: *const u8;
    static mut sqlite3a_g_tb: *const u8;
    static mut sqlite3_builtin_functions: FuncDefHash;
    static sqlite3_oom_str: Sqlite3Str;
    static mut sqlite3_pending_byte: i32;
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
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
    -> i32;
    fn sqlite3_resolve_expr_list_names(_: *mut NameContext, _: *mut ExprList)
    -> i32;
    fn sqlite3_resolve_select_names(_: *mut Parse, _: *mut Select,
    _: *mut NameContext)
    -> ();
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
    fn sqlite3_get_coll_seq(_: *mut Parse, _: u8, _: *mut CollSeq,
    _: *const i8)
    -> *mut CollSeq;
    fn sqlite3_analyze(_: *mut Parse, _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_invoke_busy_handler(_: *mut BusyHandler)
    -> i32;
    fn sqlite3_analysis_load(_: *mut Sqlite3, i_db_1: i32)
    -> i32;
    fn sqlite3_register_like_functions(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_is_like_function(_: *mut Sqlite3, _: *mut Expr, _: *mut i32,
    _: *mut i8)
    -> i32;
    fn sqlite3_schema_get(_: *mut Sqlite3, _: *mut Btree)
    -> *mut Schema;
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
    fn sqlite3_vtab_savepoint(_: *mut Sqlite3, _: i32, _: i32)
    -> i32;
    fn sqlite3_vtab_import_errmsg(_: *mut Vdbe, _: *mut Sqlite3Vtab)
    -> ();
    fn sqlite3_vtab_create_module(_: *mut Sqlite3, _: *const i8,
    _: *const Sqlite3Module, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut Module;
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
    fn sqlite3_reprepare(_: *mut Vdbe)
    -> i32;
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
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
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
