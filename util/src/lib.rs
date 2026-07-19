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
use crate::hash_h::Hash;
use crate::pager_h::{DbPage, Pager, Pgno};
use crate::pcache_h::{PCache, PgHdr};
use crate::sqlite3_h::{
    Sqlite3Backup, Sqlite3Blob, Sqlite3Context, Sqlite3File, Sqlite3Filename,
    Sqlite3IndexInfo, Sqlite3Int64, Sqlite3Module, Sqlite3Mutex,
    Sqlite3MutexMethods, Sqlite3PcachePage, Sqlite3RtreeGeometry,
    Sqlite3RtreeQueryInfo, Sqlite3Snapshot, Sqlite3Stmt, Sqlite3Uint64,
    Sqlite3Value, Sqlite3Vfs, Sqlite3Vtab, SqliteInt64,
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

type DarwinSizeT = u64;

type Uint128T = u128;

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

///* Internal function prototypes
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_i_cmp(z_left: *const i8, z_right: *const i8)
    -> i32 {
    unsafe {
        let mut a: *const u8 = core::ptr::null();
        let mut b: *const u8 = core::ptr::null();
        let mut c: i32 = 0;
        let mut x: i32 = 0;
        a = z_left as *mut u8;
        b = z_right as *mut u8;
        {
            '__b0: loop {
                '__c0: loop {
                    c = unsafe { *a } as i32;
                    x = unsafe { *b } as i32;
                    if c == x {
                        if c == 0 { break '__b0; }
                    } else {
                        c =
                            unsafe {
                                        *(sqlite3_upper_to_lower.as_ptr() as
                                                    *const u8).offset(c as isize)
                                    } as i32 -
                                unsafe {
                                        *(sqlite3_upper_to_lower.as_ptr() as
                                                    *const u8).offset(x as isize)
                                    } as i32;
                        if c != 0 { break '__b0; }
                    }
                    {
                        let __p = &mut a;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    {
                        let __p = &mut b;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                    break '__c0;
                }
            }
        }
        return c;
    }
}

///* CAPI3REF: String Comparison
///*
///* ^The [sqlite3_stricmp()] and [sqlite3_strnicmp()] APIs allow applications
///* and extensions to compare the contents of two buffers containing UTF-8
///* strings in a case-independent fashion, using the same definition of "case
///* independence" that SQLite uses internally when comparing identifiers.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_stricmp(z_left: *const i8, z_right: *const i8)
    -> i32 {
    if z_left == core::ptr::null() {
        return if !(z_right).is_null() { -1 } else { 0 };
    } else if z_right == core::ptr::null() { return 1; }
    return sqlite3_str_i_cmp(z_left, z_right);
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_strnicmp(z_left: *const i8, z_right: *const i8,
    mut n: i32) -> i32 {
    unsafe {
        let mut a: *const u8 = core::ptr::null();
        let mut b: *const u8 = core::ptr::null();
        if z_left == core::ptr::null() {
            return if !(z_right).is_null() { -1 } else { 0 };
        } else if z_right == core::ptr::null() { return 1; }
        a = z_left as *mut u8;
        b = z_right as *mut u8;
        while { let __p = &mut n; let __t = *__p; *__p -= 1; __t } > 0 &&
                    unsafe { *a } as i32 != 0 &&
                unsafe {
                            *(sqlite3_upper_to_lower.as_ptr() as
                                        *const u8).add(unsafe { *a } as usize)
                        } as i32 ==
                    unsafe {
                            *(sqlite3_upper_to_lower.as_ptr() as
                                        *const u8).add(unsafe { *b } as usize)
                        } as i32 {
            {
                let __p = &mut a;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            {
                let __p = &mut b;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        return if n < 0 {
                0
            } else {
                (unsafe {
                            *(sqlite3_upper_to_lower.as_ptr() as
                                        *const u8).add(unsafe { *a } as usize)
                        }) as i32 -
                    unsafe {
                            *(sqlite3_upper_to_lower.as_ptr() as
                                        *const u8).add(unsafe { *b } as usize)
                        } as i32
            };
    }
}

///* Compute a string length that is limited to what can be stored in
///* lower 30 bits of a 32-bit signed integer.
///*
///* The value returned will never be negative.  Nor will it ever be greater
///* than the actual length of the string.  For very long strings (greater
///* than 1GiB) the value returned might be less than the true string length.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_strlen30(z: *const i8) -> i32 {
    if z == core::ptr::null() { return 0; }
    return 1073741823 & unsafe { strlen(z) } as i32;
}

///* Return the declared type of a column.  Or return zDflt if the column
///* has no declared type.
///*
///* The column type is an extra string stored after the zero-terminator on
///* the column name if and only if the COLFLAG_HASTYPE flag is set.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3ColumnType(p_col: &Column, z_dflt: *mut i8)
    -> *mut i8 {
    unsafe {
        if (*p_col).col_flags as i32 & 4 != 0 {
            return unsafe {
                    unsafe {
                        (*p_col).z_cn_name.add(unsafe {
                                        strlen((*p_col).z_cn_name as *const i8)
                                    } as usize).offset(1 as isize)
                    }
                };
        } else if (*p_col).e_c_type() != 0 {
            { let _ = 0; };
            return unsafe {
                        *(sqlite3_std_type.as_ptr() as
                                    *mut *const i8).offset(((*p_col).e_c_type() as i32 - 1) as
                                    isize)
                    } as *mut i8;
        } else { return z_dflt; }
    }
}

///* Return true if the floating point value is Not a Number (NaN).
///*
///* Use the math library isnan() function if compiled with SQLITE_HAVE_ISNAN.
///* Otherwise, we have our own implementation that works on most systems.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_is_na_n(x: f64) -> i32 {
    let mut rc: i32 = 0;

    /// The value return
    (rc =
        if core::mem::size_of::<f64>() as u64 ==
                core::mem::size_of::<f32>() as u64 {
            unsafe { __inline_isnanf(x as f32) }
        } else {
            if core::mem::size_of::<f64>() as u64 ==
                    core::mem::size_of::<f64>() as u64 {
                unsafe { __inline_isnand(x as f64) }
            } else { unsafe { __inline_isnanl(x as f64) } }
        });
    return rc;
}

///* Return true if the floating point value is NaN or +Inf or -Inf.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_is_overflow(mut x: f64) -> i32 {
    let mut rc: i32 = 0;
    /// The value return
    let mut y: u64 = 0 as u64;
    unsafe {
        memcpy(&raw mut y as *mut (), &raw mut x as *const (),
            core::mem::size_of::<u64>() as u64)
    };
    rc = (y & (2047 as u64) << 52 == (2047 as u64) << 52) as i32;
    return rc;
}

///* Count leading zeros for a 64-bit unsigned integer.
extern "C" fn count_leading_zeros(m: u64) -> i32 {
    return m.leading_zeros() as i32;
}

extern "C" fn pwr2to10(p: i32) -> i32 { return p * 78913 >> 18; }

///* Two inputs are multiplied to get a 128-bit result.  Write the
///* lower 64-bits of the result into *pLo, and return the high-order
///* 64 bits.
extern "C" fn sqlite3_multiply128(a: u64, b: u64, p_lo_1: &mut u64) -> u64 {
    let r: Uint128T = a as Uint128T * b as Uint128T;
    *p_lo_1 = r as u64;
    return (r >> 64) as u64;
}

///* A is an unsigned 96-bit integer formed by (a<<32)+aLo.
///* B is an unsigned 64-bit integer.
///*
///* Compute the upper 96 bits of 160-bit result of A*B.
///*
///* Write ((A*B)>>64 & 0xffffffff) (the middle 32 bits of A*B)
///* into *pLo.  Return the upper 64 bits of A*B.
///*
///* The lower 64 bits of A*B are discarded.
extern "C" fn sqlite3_multiply160(a: u64, a_lo_1: u32, b: u64,
    p_lo_1: &mut u32) -> u64 {
    let mut r: Uint128T = a as Uint128T * b as Uint128T;
    r += a_lo_1 as Uint128T * b as Uint128T >> 32;
    *p_lo_1 = (r >> 32 & 4294967295u32 as Uint128T) as u32;
    return (r >> 64) as u64;
}

///* For any p between -348 and +347, return the integer part of
///*
///*    pow(10,p) * pow(2,63-pow10to2(p))
///*
///* Or, in other words, for any p in range, return the most significant
///* 64 bits of pow(10,p).  The pow(10,p) value is shifted left or right,
///* as appropriate so the most significant 64 bits fit exactly into a
///* 64-bit unsigned integer.
///*
///* Write into *pLo the next 32 significant bits of the answer after
///* the first 64.
///*
///* Algorithm:
///*
///* (1) For p between 0 and 26, return the value directly from the aBase[]
///*     lookup table.
///*
///* (2) For p outside the range 0 to 26, use aScale[] for the initial value
///*     then refine that result (if necessary) by a single multiplication
///*     against aBase[].
///*
///* The constant tables aBase[], aScale[], and aScaleLo[] are generated
///* by the C program at ../tool/mkfptab.c run with the --round option.
#[allow(unused_doc_comments)]
extern "C" fn power_of_ten(p: i32, p_lo_1: &mut u32) -> u64 {
    ///  0: 1.0e-351 << 1229
    ///  1: 1.0e-324 << 1140
    ///  2: 1.0e-297 << 1050
    ///  3: 1.0e-270 << 960
    ///  4: 1.0e-243 << 871
    ///  5: 1.0e-216 << 781
    ///  6: 1.0e-189 << 691
    ///  7: 1.0e-162 << 602
    ///  8: 1.0e-135 << 512
    ///  9: 1.0e-108 << 422
    /// 10: 1.0e-81 << 333
    /// 11: 1.0e-54 << 243
    /// 12: 1.0e-27 << 153
    /// 13: 1.0e-1 << 67 (special case)
    /// 14: 1.0e+27 >> 26
    /// 15: 1.0e+54 >> 116
    /// 16: 1.0e+81 >> 206
    /// 17: 1.0e+108 >> 295
    /// 18: 1.0e+135 >> 385
    /// 19: 1.0e+162 >> 475
    /// 20: 1.0e+189 >> 564
    /// 21: 1.0e+216 >> 654
    /// 22: 1.0e+243 >> 744
    /// 23: 1.0e+270 >> 833
    /// 24: 1.0e+297 >> 923
    /// 25: 1.0e+324 >> 1013
    let mut g: i32 = 0;
    let mut n: i32 = 0;
    let mut s: u64 = 0 as u64;
    let mut x: u64 = 0 as u64;
    let mut lo: u32 = 0 as u32;
    { let _ = 0; };
    if p < 0 {
        if p == -1 {
            *p_lo_1 = a_scale_lo[13 as usize] as u32;
            return a_scale[13 as usize] as u64;
        }
        g = p / 27;
        n = p % 27;
        if n != 0 {
            { let __p = &mut g; let __t = *__p; *__p -= 1; __t };
            n += 27;
        }
    } else if p < 27 {
        *p_lo_1 = 0 as u32;
        return a_base[p as usize] as u64;
    } else { g = p / 27; n = p % 27; }
    s = a_scale[(g + 13) as usize] as u64;
    if n == 0 { *p_lo_1 = a_scale_lo[(g + 13) as usize] as u32; return s; }
    x =
        sqlite3_multiply160(s, a_scale_lo[(g + 13) as usize],
            a_base[n as usize], &mut lo);
    if (1 as u64) << 63 & x == 0 as u64 {
        x = x << 1 | (lo >> 31 & 1 as u32) as u64;
        lo = lo << 1 | 1 as u32;
    }
    *p_lo_1 = lo;
    return x;
}

///* pow10to2(x) computes floor(log2(pow(10,x))).
///* pow2to10(y) computes floor(log10(pow(2,y))).
///*
///* Conceptually, pow10to2(p) converts a base-10 exponent p into
///* a corresponding base-2 exponent, and pow2to10(e) converts a base-2
///* exponent into a base-10 exponent.
///*
///* The conversions are based on the observation that:
///*
///*     ln(10.0)/ln(2.0) == 108853/32768     (approximately)
///*     ln(2.0)/ln(10.0) == 78913/262144     (approximately)
///*
///* These ratios are approximate, but they are accurate to 5 digits,
///* which is close enough for the usage here.  Right-shift is used
///* for division so that rounding of negative numbers happens in the
///* right direction.
extern "C" fn pwr10to2(p: i32) -> i32 { return p * 108853 >> 15; }

///* Given m and e, which represent a quantity r == m*pow(2,e),
///* return values *pD and *pP such that r == (*pD)*pow(10,*pP),
///* approximately.  *pD should contain at least n significant digits.
///*
///* The input m is required to have its highest bit set.  In other words,
///* m should be left-shifted, and e decremented, to maximize the value of m.
extern "C" fn sqlite3_fp2_convert10(m: u64, e: i32, n: i32, p_d_1: *mut u64,
    p_p_1: &mut i32) -> () {
    let mut p: i32 = 0;
    let mut h: u64 = 0 as u64;
    let mut d1: u64 = 0 as u64;
    let mut d2: u32 = 0 as u32;
    { let _ = 0; };
    p = n - 1 - pwr2to10(e + 63);
    h = sqlite3_multiply128(m, power_of_ten(p, &mut d2), &mut d1);
    { let _ = 0; };
    { let _ = 0; };
    if n == 18 {
        h >>= -(e + pwr10to2(p) + 2) as u64;
        unsafe { *p_d_1 = h + (h << 1 & 2 as u64) >> 1 };
    } else { unsafe { *p_d_1 = h >> -(e + pwr10to2(p) + 1) }; }
    *p_p_1 = -p;
}

///* Digit pairs used to convert a U64 or I64 into text, two digits
///* at a time.
#[repr(C)]
#[derive(Copy, Clone)]
union AnonU0 {
    a: [i8; 201],
    force_alignment: i16,
}

static sqlite3_digit_pairs: AnonU0 =
    AnonU0 {
        a: [48 as i8, 48 as i8, 48 as i8, 49 as i8, 48 as i8, 50 as i8,
                48 as i8, 51 as i8, 48 as i8, 52 as i8, 48 as i8, 53 as i8,
                48 as i8, 54 as i8, 48 as i8, 55 as i8, 48 as i8, 56 as i8,
                48 as i8, 57 as i8, 49 as i8, 48 as i8, 49 as i8, 49 as i8,
                49 as i8, 50 as i8, 49 as i8, 51 as i8, 49 as i8, 52 as i8,
                49 as i8, 53 as i8, 49 as i8, 54 as i8, 49 as i8, 55 as i8,
                49 as i8, 56 as i8, 49 as i8, 57 as i8, 50 as i8, 48 as i8,
                50 as i8, 49 as i8, 50 as i8, 50 as i8, 50 as i8, 51 as i8,
                50 as i8, 52 as i8, 50 as i8, 53 as i8, 50 as i8, 54 as i8,
                50 as i8, 55 as i8, 50 as i8, 56 as i8, 50 as i8, 57 as i8,
                51 as i8, 48 as i8, 51 as i8, 49 as i8, 51 as i8, 50 as i8,
                51 as i8, 51 as i8, 51 as i8, 52 as i8, 51 as i8, 53 as i8,
                51 as i8, 54 as i8, 51 as i8, 55 as i8, 51 as i8, 56 as i8,
                51 as i8, 57 as i8, 52 as i8, 48 as i8, 52 as i8, 49 as i8,
                52 as i8, 50 as i8, 52 as i8, 51 as i8, 52 as i8, 52 as i8,
                52 as i8, 53 as i8, 52 as i8, 54 as i8, 52 as i8, 55 as i8,
                52 as i8, 56 as i8, 52 as i8, 57 as i8, 53 as i8, 48 as i8,
                53 as i8, 49 as i8, 53 as i8, 50 as i8, 53 as i8, 51 as i8,
                53 as i8, 52 as i8, 53 as i8, 53 as i8, 53 as i8, 54 as i8,
                53 as i8, 55 as i8, 53 as i8, 56 as i8, 53 as i8, 57 as i8,
                54 as i8, 48 as i8, 54 as i8, 49 as i8, 54 as i8, 50 as i8,
                54 as i8, 51 as i8, 54 as i8, 52 as i8, 54 as i8, 53 as i8,
                54 as i8, 54 as i8, 54 as i8, 55 as i8, 54 as i8, 56 as i8,
                54 as i8, 57 as i8, 55 as i8, 48 as i8, 55 as i8, 49 as i8,
                55 as i8, 50 as i8, 55 as i8, 51 as i8, 55 as i8, 52 as i8,
                55 as i8, 53 as i8, 55 as i8, 54 as i8, 55 as i8, 55 as i8,
                55 as i8, 56 as i8, 55 as i8, 57 as i8, 56 as i8, 48 as i8,
                56 as i8, 49 as i8, 56 as i8, 50 as i8, 56 as i8, 51 as i8,
                56 as i8, 52 as i8, 56 as i8, 53 as i8, 56 as i8, 54 as i8,
                56 as i8, 55 as i8, 56 as i8, 56 as i8, 56 as i8, 57 as i8,
                57 as i8, 48 as i8, 57 as i8, 49 as i8, 57 as i8, 50 as i8,
                57 as i8, 51 as i8, 57 as i8, 52 as i8, 57 as i8, 53 as i8,
                57 as i8, 54 as i8, 57 as i8, 55 as i8, 57 as i8, 56 as i8,
                57 as i8, 57 as i8, 0 as i8],
    };

///* Return an IEEE754 floating point value that approximates d*pow(10,p).
///*
///* The (current) algorithm is adapted from the work of Ross Cox at
///* https://github.com/rsc/fpfmt
extern "C" fn sqlite3_fp10_convert2(d: u64, p: i32) -> f64 {
    let mut b: i32 = 0;
    let mut lp: i32 = 0;
    let mut e: i32 = 0;
    let mut adj: i32 = 0;
    let mut s: i32 = 0;
    let mut pwr10l: u32 = 0 as u32;
    let mut mid1: u32 = 0 as u32;
    let mut pwr10h: u64 = 0 as u64;
    let mut x: u64 = 0 as u64;
    let mut hi: u64 = 0 as u64;
    let mut lo: u64 = 0 as u64;
    let mut sticky: u64 = 0 as u64;
    let mut u: u64 = 0 as u64;
    let mut m: u64 = 0 as u64;
    let mut r: f64 = 0.0;
    if p < -348 { return 0.0; }
    if p > 347 { return f32::INFINITY as f64; }
    b = 64 - count_leading_zeros(d);
    lp = pwr10to2(p);
    e = 53 - b - lp;
    if e > 1074 { if e >= 1130 { return 0.0; } e = 1074; }
    s = -(e - (64 - b) + lp + 3);
    pwr10h = power_of_ten(p, &mut pwr10l);
    if pwr10l != 0 as u32 {
        { let __p = &mut pwr10h; let __t = *__p; *__p += 1; __t };
        pwr10l = !pwr10l;
    }
    x = d << 64 - b;
    hi = sqlite3_multiply128(x, pwr10h, &mut lo);
    mid1 = (lo >> 32) as u32;
    sticky = 1 as u64;
    if hi & ((1 as u64) << s) - 1 as u64 == 0 as u64 {
        let mid2: u32 =
            (sqlite3_multiply128(x, (pwr10l as u64) << 32, &mut lo) >> 32) as
                u32;
        sticky = (mid1 - mid2 > 1 as u32) as u64;
        hi -= (mid1 < mid2) as u64;
    }
    u = hi >> s | sticky;
    adj = (u >= ((1 as u64) << 55) - 2 as u64) as i32;
    if adj != 0 { u = u >> adj | u & 1 as u64; e -= adj; }
    m = u + 1 as u64 + (u >> 2 & 1 as u64) >> 2;
    if e <= -972 { return f32::INFINITY as f64; }
    if m & (1 as u64) << 52 != 0 as u64 {
        m = m & !((1 as u64) << 52) | ((1075 - e) as u64) << 52;
    }
    unsafe {
        memcpy(&raw mut r as *mut (), &raw mut m as *const (), 8 as u64)
    };
    return r;
}

///* Decode a floating-point value into an approximate decimal
///* representation.
///*
///* If iRound<=0 then round to -iRound significant digits to the
///* the right of the decimal point, or to a maximum of mxRound total
///* significant digits.
///*
///* If iRound>0 round to min(iRound,mxRound) significant digits total.
///*
///* mxRound must be positive.
///*
///* The significant digits of the decimal representation are
///* stored in p->z[] which is a often (but not always) a pointer
///* into the middle of p->zBuf[].  There are p->n significant digits.
///* The p->z[] array is *not* zero-terminated.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_fp_decode(p: &mut FpDecode, mut r: f64,
    mut i_round: i32, mx_round: i32) -> () {
    unsafe {
        let mut i: i32 = 0;
        /// Index into zBuf[] where to put next character
        let mut n: i32 = 0;
        /// Number of digits
        let mut v: u64 = 0 as u64;
        /// mantissa
        let mut e: i32 = 0;
        let mut exp: i32 = 0;
        /// Base-2 and base-10 exponent
        let mut z_buf: *mut i8 = core::ptr::null_mut();
        /// Local alias for p->zBuf
        let mut z: *mut i8 = core::ptr::null_mut();

        /// Local alias for p->z
        ((*p).is_special = 0 as i8);
        { let _ = 0; };
        if r < 0.0 {
            (*p).sign = '-' as i32 as i8;
            r = -r;
        } else if r == 0.0 {
            (*p).sign = '+' as i32 as i8;
            (*p).n = 1;
            (*p).i_dp = 1;
            (*p).z = c"0".as_ptr() as *mut i8;
            return;
        } else { (*p).sign = '+' as i32 as i8; }
        unsafe {
            memcpy(&raw mut v as *mut (), &raw mut r as *const (), 8 as u64)
        };
        e = (v >> 52 & 2047 as u64) as i32;
        if e == 2047 {
            (*p).is_special =
                (1 + (v != 9218868437227405312i64 as u64) as i32) as i8;
            (*p).n = 0;
            (*p).i_dp = 0;
            (*p).z = &raw mut (*p).z_buf[0 as usize] as *mut i8;
            return;
        }
        v &= 4503599627370495u64;
        if e == 0 {
            let nn: i32 = count_leading_zeros(v);
            v <<= nn as u64;
            e = -1074 - nn;
        } else { v = v << 11 | (1 as u64) << 63; e -= 1086; }
        sqlite3_fp2_convert10(v, e,
            if i_round <= 0 || i_round >= 18 { 18 } else { i_round + 1 },
            &mut v, &mut exp);

        /// Extract significant digits, start at the right-most slot in p->zBuf
        ///* and working back to the right.  "i" keeps track of the next slot in
        ///* which to store a digit.
        { let _ = 0; };
        { let _ = 0; };
        z_buf = &raw mut (*p).z_buf[0 as usize] as *mut i8;
        i = 20;
        while v >= 10 as u64 {
            let mut kk: i32 = (v % 100 as u64 * 2 as u64) as i32;
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                *(unsafe { &raw mut *z_buf.offset((i - 2) as isize) } as
                            *mut u16) =
                    unsafe {
                        *(&raw const sqlite3_digit_pairs.a[kk as usize] as *mut u16)
                    }
            };
            i -= 2;
            v /= 100 as u64;
        }
        if v != 0 {
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                *z_buf.offset({ let __p = &mut i; *__p -= 1; *__p } as isize)
                    = (v + '0' as i32 as u64) as i8
            };
        }

        /// SQLITE_AVOID_U64_DIVIDE
        { let _ = 0; };
        n = 20 - i;

        /// Total number of digits extracted
        { let _ = 0; };
        { let _ = 0; };
        (*p).i_dp = n + exp;
        if i_round <= 0 {
            i_round = (*p).i_dp - i_round;
            if i_round == 0 &&
                    unsafe { *z_buf.offset(i as isize) } as i32 >= '5' as i32 {
                i_round = 1;
                unsafe {
                    *z_buf.offset({ let __p = &mut i; *__p -= 1; *__p } as
                                    isize) = '0' as i32 as i8
                };
                { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                { let __p = &mut (*p).i_dp; let __t = *__p; *__p += 1; __t };
            }
        }
        z = unsafe { z_buf.offset(i as isize) };
        if i_round > 0 && (i_round < n || n > mx_round) {
            if i_round > mx_round { i_round = mx_round; }
            if i_round == 17 {
                if unsafe { *z.offset(15 as isize) } as i32 == '9' as i32 &&
                        unsafe { *z.offset(14 as isize) } as i32 == '9' as i32 {
                    let mut jj: i32 = 0;
                    let mut kk: i32 = 0;
                    let mut v2: u64 = 0 as u64;
                    {
                        jj = 14;
                        '__b3: loop {
                            if !(jj > 0 &&
                                            unsafe { *z.offset((jj - 1) as isize) } as i32 ==
                                                '9' as i32) {
                                break '__b3;
                            }
                            '__c3: loop { break '__c3; }
                            { let __p = &mut jj; let __t = *__p; *__p -= 1; __t };
                        }
                    }
                    if jj == 0 {
                        v2 = 1 as u64;
                    } else {
                        v2 =
                            (unsafe { *z.offset(0 as isize) } as i32 - '0' as i32) as
                                u64;
                        {
                            kk = 1;
                            '__b4: loop {
                                if !(kk < jj) { break '__b4; }
                                '__c4: loop {
                                    v2 =
                                        v2 * 10 as u64 + unsafe { *z.offset(kk as isize) } as u64 -
                                            '0' as i32 as u64;
                                    break '__c4;
                                }
                                { let __p = &mut kk; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        { let __p = &mut v2; let __t = *__p; *__p += 1; __t };
                    }
                    if r == sqlite3_fp10_convert2(v2, exp + n - jj) {
                        i_round = jj + 1;
                    }
                } else if (*p).i_dp >= n ||
                        unsafe { *z.offset(15 as isize) } as i32 == '0' as i32 &&
                                unsafe { *z.offset(14 as isize) } as i32 == '0' as i32 &&
                            unsafe { *z.offset(13 as isize) } as i32 == '0' as i32 {
                    let mut jj: i32 = 0;
                    let mut kk: i32 = 0;
                    let mut v2: u64 = 0 as u64;
                    { let _ = 0; };
                    {
                        jj = 13;
                        '__b5: loop {
                            if !(unsafe { *z.offset((jj - 1) as isize) } as i32 ==
                                            '0' as i32) {
                                break '__b5;
                            }
                            '__c5: loop { break '__c5; }
                            { let __p = &mut jj; let __t = *__p; *__p -= 1; __t };
                        }
                    }
                    v2 =
                        (unsafe { *z.offset(0 as isize) } as i32 - '0' as i32) as
                            u64;
                    {
                        kk = 1;
                        '__b6: loop {
                            if !(kk < jj) { break '__b6; }
                            '__c6: loop {
                                v2 =
                                    v2 * 10 as u64 + unsafe { *z.offset(kk as isize) } as u64 -
                                        '0' as i32 as u64;
                                break '__c6;
                            }
                            { let __p = &mut kk; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if r == sqlite3_fp10_convert2(v2, exp + n - jj) {
                        i_round = jj + 1;
                    }
                }
            }
            n = i_round;
            if unsafe { *z.offset(i_round as isize) } as i32 >= '5' as i32 {
                let mut j: i32 = i_round - 1;
                loop {
                    {
                        let __p = unsafe { &mut *z.offset(j as isize) };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    if unsafe { *z.offset(j as isize) } as i32 <= '9' as i32 {
                        break;
                    }
                    unsafe { *z.offset(j as isize) = '0' as i32 as i8 };
                    if j == 0 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(-1) };
                            __t
                        };
                        unsafe { *z.offset(0 as isize) = '1' as i32 as i8 };
                        { let __p = &mut n; let __t = *__p; *__p += 1; __t };
                        {
                            let __p = &mut (*p).i_dp;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        break;
                    } else {
                        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                    }
                }
            }
        }
        { let _ = 0; };
        while unsafe { *z.offset((n - 1) as isize) } as i32 == '0' as i32 {
            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
            { let _ = 0; };
        }
        (*p).n = n;
        (*p).z = z;
    }
}

///* Check for interrupts and invoke progress callback.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_progress_check(p: &mut Parse) -> () {
    unsafe {
        let db: *mut Sqlite3 = (*p).db;
        if unsafe {
                    std::sync::atomic::AtomicI32::from_ptr(unsafe {
                                    &raw mut (*db).u1.is_interrupted
                                } as *mut i32).load(std::sync::atomic::Ordering::Relaxed)
                } != 0 {
            { let __p = &mut (*p).n_err; let __t = *__p; *__p += 1; __t };
            (*p).rc = 9;
        }
        if unsafe { (*db).x_progress.is_some() } {
            if (*p).rc == 9 {
                (*p).n_progress_steps = 0 as u32;
            } else if {
                        let __p = &mut (*p).n_progress_steps;
                        *__p += 1;
                        *__p
                    } >= unsafe { (*db).n_progress_ops } {
                if unsafe {
                            (unsafe {
                                    (*db).x_progress.unwrap()
                                })(unsafe { (*db).p_progress_arg })
                        } != 0 {
                    {
                        let __p = &mut (*p).n_err;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    (*p).rc = 9;
                }
                (*p).n_progress_steps = 0 as u32;
            }
        }
    }
}

///* Add an error message to pParse->zErrMsg and increment pParse->nErr.
///*
///* This function should be used to report any error that occurs while
///* compiling an SQL statement (i.e. within sqlite3_prepare()). The
///* last thing the sqlite3_prepare() function does is copy the error
///* stored by this function into the database handle using sqlite3Error().
///* Functions sqlite3Error() or sqlite3ErrorWithMsg() should be used
///* during statement execution (sqlite3_step() etc.).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_error_msg(p_parse: &mut Parse,
    z_format: *const i8, mut __va0: ...) -> () {
    let mut z_msg: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    let db: *mut Sqlite3 = (*p_parse).db;
    { let _ = 0; };
    { let _ = 0; };
    unsafe { (*db).err_byte_offset = -2 };
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_msg = unsafe { sqlite3_vm_printf(db, z_format, ap) };
    ();
    if unsafe { (*db).err_byte_offset } < -1 {
        unsafe { (*db).err_byte_offset = -1 };
    }
    if unsafe { (*db).suppress_err } != 0 {
        unsafe { sqlite3_db_free(db, z_msg as *mut ()) };
        if unsafe { (*db).malloc_failed } != 0 {
            {
                let __p = &mut (*p_parse).n_err;
                let __t = *__p;
                *__p += 1;
                __t
            };
            (*p_parse).rc = 7;
        }
    } else {
        { let __p = &mut (*p_parse).n_err; let __t = *__p; *__p += 1; __t };
        unsafe { sqlite3_db_free(db, (*p_parse).z_err_msg as *mut ()) };
        (*p_parse).z_err_msg = z_msg;
        (*p_parse).rc = 1;
        (*p_parse).p_with = core::ptr::null_mut();
    }
}

///* If database connection db is currently parsing SQL, then transfer
///* error code errCode to that parser if the parser has not already
///* encountered some other kind of error.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_error_to_parser(db: *mut Sqlite3, err_code: i32)
    -> i32 {
    let mut p_parse: *mut Parse = core::ptr::null_mut();
    if db == core::ptr::null_mut() ||
            { p_parse = unsafe { (*db).p_parse }; p_parse } ==
                core::ptr::null_mut() {
        return err_code;
    }
    unsafe { (*p_parse).rc = err_code };
    {
        let __p = unsafe { &mut (*p_parse).n_err };
        let __t = *__p;
        *__p += 1;
        __t
    };
    return err_code;
}

///* Convert an SQL-style quoted string into a normal string by removing
///* the quote characters.  The conversion is done in-place.  If the
///* input does not begin with a quote character, then this routine
///* is a no-op.
///*
///* The input string must be zero-terminated.  A new zero-terminator
///* is added to the dequoted string.
///*
///* The return value is -1 if no dequoting occurs or the length of the
///* dequoted string, exclusive of the zero terminator, if dequoting does
///* occur.
///*
///* 2002-02-14: This routine is extended to remove MS-Access style
///* brackets from around identifiers.  For example:  "[a-b-c]" becomes
///* "a-b-c".
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_dequote(z: *mut i8) -> () {
    unsafe {
        let mut quote: i8 = 0 as i8;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        if z == core::ptr::null_mut() { return; }
        quote = unsafe { *z.offset(0 as isize) };
        if (unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(quote as u8 as usize)
                                } as i32 & 128 == 0) as i32 != 0 {
            return;
        }
        if quote as i32 == '[' as i32 { quote = ']' as i32 as i8; }
        {
            { i = 1; j = 0 };
            '__b9: loop {
                '__c9: loop {
                    { let _ = 0; };
                    if unsafe { *z.offset(i as isize) } as i32 == quote as i32 {
                        if unsafe { *z.offset((i + 1) as isize) } as i32 ==
                                quote as i32 {
                            unsafe {
                                *z.offset({
                                                    let __p = &mut j;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                } as isize) = quote
                            };
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        } else { break '__b9; }
                    } else {
                        unsafe {
                            *z.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) = unsafe { *z.offset(i as isize) }
                        };
                    }
                    break '__c9;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { *z.offset(j as isize) = 0 as i8 };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_dequote_expr(p: &mut Expr) -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        (*p).flags |=
            if unsafe { *(*p).u.z_token.offset(0 as isize) } as i32 ==
                        '\"' as i32 {
                    67108864 | 128
                } else { 67108864 } as u32;
        sqlite3_dequote((*p).u.z_token);
    }
}

///* If the input token p is quoted, try to adjust the token to remove
///* the quotes.  This is not always possible:
///*
///*     "abc"     ->   abc
///*     "ab""cd"  ->   (not possible because of the interior "")
///*
///* Remove the quotes if possible.  This is a optimization.  The overall
///* system should still return the correct answer even if this routine
///* is always a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_dequote_token(p: &mut Token) -> () {
    unsafe {
        unsafe {
            let mut i: u32 = 0 as u32;
            if (*p).n < 2 as u32 { return; }
            if (unsafe {
                                        *(sqlite3_ctype_map.as_ptr() as
                                                    *const u8).add(unsafe { *(*p).z.offset(0 as isize) } as u8
                                                    as usize)
                                    } as i32 & 128 == 0) as i32 != 0 {
                return;
            }
            {
                i = 1 as u32;
                '__b10: loop {
                    if !(i < (*p).n - 1 as u32) { break '__b10; }
                    '__c10: loop {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *(*p).z.add(i as usize) } as u8 as
                                                        usize)
                                        } as i32 & 128 != 0 {
                            return;
                        }
                        break '__c10;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            (*p).n -= 2 as u32;
            {
                let __p = &mut (*p).z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
    }
}

///* Translate a single byte of Hex into an integer.
///* This routine only works if h really is a valid hexadecimal
///* character:  0..9a..fA..F
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_hex_to_int(mut h: i32) -> u8 {
    { let _ = 0; };
    h += 9 * (1 & h >> 6);
    return (h & 15) as u8;
}

///* If zNum represents an integer that will fit in 32-bits, then set
///* *pValue to that integer and return true.  Otherwise return false.
///*
///* This routine accepts both decimal and hexadecimal notation for integers.
///*
///* Any non-numeric characters that following zNum are ignored.
///* This is different from sqlite3Atoi64() which requires the
///* input number to be zero-terminated.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_int32(mut z_num: *const i8, p_value: *mut i32)
    -> i32 {
    unsafe {
        let mut v: SqliteInt64 = 0 as SqliteInt64;
        let mut i: i32 = 0;
        let mut c: i32 = 0;
        let mut neg: i32 = 0;
        if unsafe { *z_num.offset(0 as isize) } as i32 == '-' as i32 {
            neg = 1;
            {
                let __p = &mut z_num;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else if unsafe { *z_num.offset(0 as isize) } as i32 == '+' as i32 {
            {
                let __p = &mut z_num;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else if unsafe { *z_num.offset(0 as isize) } as i32 == '0' as i32 &&
                    (unsafe { *z_num.offset(1 as isize) } as i32 == 'x' as i32
                        ||
                        unsafe { *z_num.offset(1 as isize) } as i32 == 'X' as i32)
                &&
                unsafe {
                                *(sqlite3_ctype_map.as_ptr() as
                                            *const u8).add(unsafe { *z_num.offset(2 as isize) } as u8 as
                                            usize)
                            } as i32 & 8 != 0 {
            let mut u: u32 = 0 as u32;
            {
                let __n = 2;
                let __p = &mut z_num;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            while unsafe { *z_num.offset(0 as isize) } as i32 == '0' as i32 {
                {
                    let __p = &mut z_num;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            {
                i = 0;
                '__b12: loop {
                    if !(i < 8 &&
                                    unsafe {
                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                *const u8).add(unsafe { *z_num.offset(i as isize) } as u8 as
                                                                usize)
                                                } as i32 & 8 != 0) {
                        break '__b12;
                    }
                    '__c12: loop {
                        u =
                            u * 16 as u32 +
                                sqlite3_hex_to_int(unsafe { *z_num.offset(i as isize) } as
                                            i32) as u32;
                        break '__c12;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if u & 2147483648u32 == 0 as u32 &&
                    unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe { *z_num.offset(i as isize) } as u8 as
                                                usize)
                                } as i32 & 8 == 0 {
                unsafe {
                    memcpy(p_value as *mut (), &raw mut u as *const (),
                        4 as u64)
                };
                return 1;
            } else { return 0; }
        }
        if (unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe { *z_num.offset(0 as isize) } as u8 as
                                                usize)
                                } as i32 & 4 == 0) as i32 != 0 {
            return 0;
        }
        while unsafe { *z_num.offset(0 as isize) } as i32 == '0' as i32 {
            {
                let __p = &mut z_num;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        {
            i = 0;
            '__b14: loop {
                if !(i < 11 &&
                                    {
                                            c =
                                                unsafe { *z_num.offset(i as isize) } as i32 - '0' as i32;
                                            c
                                        } >= 0 && c <= 9) {
                    break '__b14;
                }
                '__c14: loop {
                    v = v * 10 as SqliteInt64 + c as SqliteInt64;
                    break '__c14;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if i > 10 { return 0; }
        if v - neg as SqliteInt64 > 2147483647 as i64 { return 0; }
        if neg != 0 { v = -v; }
        unsafe { *p_value = v as i32 };
        return 1;
    }
}

///* Expression p is a QNUMBER (quoted number). Dequote the value in p->u.zToken
///* and set the type to INTEGER or FLOAT. "Quoted" integers or floats are those
///* that contain '_' characters that must be removed before further processing.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_dequote_number(p_parse: *mut Parse, p: *mut Expr)
    -> () {
    unsafe {
        unsafe {
            { let _ = 0; };
            if !(p).is_null() {
                let mut p_in: *const i8 =
                    unsafe { (*p).u.z_token } as *const i8;
                let mut p_out: *mut i8 = unsafe { (*p).u.z_token };
                let b_hex: i32 =
                    (unsafe { *p_in.offset(0 as isize) } as i32 == '0' as i32 &&
                            (unsafe { *p_in.offset(1 as isize) } as i32 == 'x' as i32 ||
                                unsafe { *p_in.offset(1 as isize) } as i32 == 'X' as i32))
                        as i32;
                let mut i_value: i32 = 0;
                { let _ = 0; };
                unsafe { (*p).op = 156 as u8 };
                '__b15: loop {
                    '__c15: loop {
                        if unsafe { *p_in } as i32 != '_' as i32 {
                            unsafe {
                                *{
                                            let __p = &mut p_out;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        } = unsafe { *p_in } as i8
                            };
                            if unsafe { *p_in } as i32 == 'e' as i32 ||
                                        unsafe { *p_in } as i32 == 'E' as i32 ||
                                    unsafe { *p_in } as i32 == '.' as i32 {
                                unsafe { (*p).op = 154 as u8 };
                            }
                        } else {
                            if b_hex == 0 &&
                                        ((unsafe {
                                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                                *const u8).add(unsafe { *p_in.offset(-1 as isize) } as u8 as
                                                                                usize)
                                                                } as i32 & 4 == 0) as i32 != 0 ||
                                            (unsafe {
                                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                                *const u8).add(unsafe { *p_in.offset(1 as isize) } as u8 as
                                                                                usize)
                                                                } as i32 & 4 == 0) as i32 != 0) ||
                                    b_hex == 1 &&
                                        ((unsafe {
                                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                                *const u8).add(unsafe { *p_in.offset(-1 as isize) } as u8 as
                                                                                usize)
                                                                } as i32 & 8 == 0) as i32 != 0 ||
                                            (unsafe {
                                                                    *(sqlite3_ctype_map.as_ptr() as
                                                                                *const u8).add(unsafe { *p_in.offset(1 as isize) } as u8 as
                                                                                usize)
                                                                } as i32 & 8 == 0) as i32 != 0) {
                                unsafe {
                                    sqlite3_error_msg(unsafe { &mut *p_parse },
                                        c"unrecognized token: \"%s\"".as_ptr() as *mut i8 as
                                            *const i8, unsafe { (*p).u.z_token })
                                };
                            }
                        }
                        break '__c15;
                    }
                    if !(unsafe {
                                        *{
                                                let __p = &mut p_in;
                                                let __t = *__p;
                                                *__p = unsafe { (*__p).offset(1) };
                                                __t
                                            }
                                    } != 0) {
                        break '__b15;
                    }
                }
                if b_hex != 0 { unsafe { (*p).op = 156 as u8 }; }
                if unsafe { (*p).op } as i32 == 156 &&
                        sqlite3_get_int32(unsafe { (*p).u.z_token } as *const i8,
                                &mut i_value) != 0 {
                    unsafe { (*p).u.i_value = i_value };
                    unsafe { (*p).flags |= 2048 as u32 };
                }
            }
        }
    }
}

///* Generate a Token object from a string
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_token_init(p: &mut Token, z: *mut i8) -> () {
    unsafe {
        (*p).z = z as *const i8;
        (*p).n = sqlite3_strlen30(z as *const i8) as u32;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_fault_sim(i_test: i32) -> i32 {
    unsafe {
        let x_callback: Option<unsafe extern "C" fn(i32) -> i32> =
            Some(sqlite3Config.x_test_callback);
        return if x_callback.is_some() {
                unsafe { x_callback.unwrap()(i_test) }
            } else { 0 };
    }
}

///* Log an error that is an API call on a connection pointer that should
///* not have been used.  The "type" of connection pointer is given as the
///* argument.  The zType is a word like "NULL" or "closed" or "invalid".
extern "C" fn log_bad_connection(z_type_1: *const i8) -> () {
    unsafe {
        sqlite3_log(21,
            c"API call with %s database connection pointer".as_ptr() as
                    *mut i8 as *const i8, z_type_1)
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_safety_check_sick_or_ok(db: &Sqlite3) -> i32 {
    let mut e_open_state: u8 = 0 as u8;
    e_open_state = (*db).e_open_state;
    if e_open_state as i32 != 186 && e_open_state as i32 != 118 &&
            e_open_state as i32 != 109 {
        log_bad_connection(c"invalid".as_ptr() as *mut i8 as *const i8);
        return 0;
    } else { return 1; }
}

///* Check to make sure we have a valid db pointer.  This test is not
///* foolproof but it does provide some measure of protection against
///* misuse of the interface such as passing in db pointers that are
///* NULL or which have been previously closed.  If this routine returns
///* 1 it means that the db pointer is valid and 0 if it should not be
///* dereferenced for any reason.  The calling function should invoke
///* SQLITE_MISUSE immediately.
///*
///* sqlite3SafetyCheckOk() requires that the db pointer be valid for
///* use.  sqlite3SafetyCheckSickOrOk() allows a db pointer that failed to
///* open properly and is not fit for general use but which can be
///* used as an argument to sqlite3_errmsg() or sqlite3_close().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_safety_check_ok(db: *mut Sqlite3) -> i32 {
    let mut e_open_state: u8 = 0 as u8;
    if db == core::ptr::null_mut() {
        log_bad_connection(c"NULL".as_ptr() as *mut i8 as *const i8);
        return 0;
    }
    e_open_state = unsafe { (*db).e_open_state };
    if e_open_state as i32 != 118 {
        if sqlite3_safety_check_sick_or_ok(unsafe { &*db }) != 0 {
            log_bad_connection(c"unopened".as_ptr() as *mut i8 as *const i8);
        }
        return 0;
    } else { return 1; }
}

///* Render an signed 64-bit integer as text.  Store the result in zOut[] and
///* return the length of the string that was stored, in bytes.  The value
///* returned does not include the zero terminator at the end of the output
///* string.
///*
///* The caller must ensure that zOut[] is at least 21 bytes in size.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_int64_to_text(v: i64, z_out: *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut x: u64 = 0 as u64;
        let mut u: Sqlite3Int64ToTextU0N24sqlite3Int64ToTextU0 =
            unsafe { core::mem::zeroed() };
        if v > 0 as i64 {
            x = v as u64;
        } else if v == 0 as i64 {
            unsafe { *z_out.offset(0 as isize) = '0' as i32 as i8 };
            unsafe { *z_out.offset(1 as isize) = 0 as i8 };
            return 1;
        } else {
            x =
                if v ==
                        -1 as i64 -
                            (4294967295u32 as i64 | (2147483647 as i64) << 32) {
                    (1 as u64) << 63
                } else { -v as u64 };
        }
        i = (core::mem::size_of::<[i8; 21]>() as u64 - 1 as u64) as i32;
        u.a[i as usize] = 0 as i8;
        while x >= 10 as u64 {
            let kk: i32 = (x % 100 as u64 * 2 as u64) as i32;
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                *(&raw mut u.a[(i - 2) as usize] as *mut u16) =
                    unsafe {
                        *(&raw const sqlite3_digit_pairs.a[kk as usize] as *mut u16)
                    }
            };
            i -= 2;
            x /= 100 as u64;
        }
        if x != 0 {
            u.a[{ let __p = &mut i; *__p -= 1; *__p } as usize] =
                (x + '0' as i32 as u64) as i8;
        }
        if v < 0 as i64 {
            u.a[{ let __p = &mut i; *__p -= 1; *__p } as usize] =
                '-' as i32 as i8;
        }
        unsafe {
            memcpy(z_out as *mut (), &raw mut u.a[i as usize] as *const (),
                core::mem::size_of::<[i8; 21]>() as u64 - i as u64)
        };
        return (core::mem::size_of::<[i8; 21]>() as u64 - 1 as u64 - i as u64)
                as i32;
    }
}

///* The string z[] is an text representation of a real number.
///* Convert this string to a double and write it into *pResult.
///*
///* z[] must be UTF-8 and zero-terminated.
///*
///* Return positive if the result is a valid real number (or integer) and
///* zero or negative if the string is empty or contains extraneous text.
///* Lower bits of the return value contain addition information about the
///* parse:
///*
///*   bit 0       =>   Set if any prefix of the input is valid.  Clear if
///*                    there is no prefix of the input that can be seen as
///*                    a valid floating point number.
///*   bit 1       =>   Set if the input contains a decimal point or eNNN
///*                    clause.  Zero if the input is an integer.
///*   bit 2       =>   The input is exactly 0.0, not an underflow from
///*                    some value near zero.
///*   bit 3       =>   Set if there are more than about 19 significant
///*                    digits in the input.
///*
///* If the input contains a syntax error but begins with text that might
///* be a valid number of some kind, then the result is negative.  The
///* result is only zero if no prefix of the input could be interpreted as
///* a number.
///*
///* Leading and trailing whitespace is ignored.  Valid numbers are in
///* one of the formats below:
///*
///*    [+-]digits[E[+-]digits]
///*    [+-]digits.[digits][E[+-]digits]
///*    [+-].digits[E[+-]digits]
///*
///* Algorithm sketch:  Compute an unsigned 64-bit integer s and a base-10
///* exponent d such that the value encoding by the input is s*pow(10,d).
///* Then invoke sqlite3Fp10Convert2() to calculated the closest possible
///* IEEE754 double.  The sign is added back afterwards, if the input string
///* starts with a "-".  The use of an unsigned 64-bit s mantissa means that
///* only about the first 19 significant digits of the input can contribute
///* to the result.  This can result in suboptimal rounding decisions when
///* correct rounding requires more than 19 input digits.  For example,
///* this routine renders "3500000000000000.2500001" as
///* 3500000000000000.0 instead of 3500000000000000.5 because the decision
///* to round up instead of using banker's rounding to round down is determined
///* by the 23rd significant digit, which this routine ignores. It is not
///* possible to do better without some kind of BigNum.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_ato_f(z_in: *const i8, p_result: &mut f64) -> i32 {
    unsafe {
        let mut z: *const u8 = core::ptr::null();
        let mut neg: i32 = 0;
        /// True for a negative value
        let mut s: u64 = 0 as u64;
        /// mantissa
        let mut d: i32 = 0;
        /// Value is s * pow(10,d)
        let mut m_state: i32 = 0;
        /// 1: digit seen 2: fp 4: hard-zero
        let mut v: u32 = 0 as u32;
        /// Value of a single digit
        /// if decimal point is present
        /// if exponent is present
        let mut esign: i32 = 0;
        /// get sign of exponent
        /// copy digits to exponent
        let mut exp: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s18:
                {
                match __state {
                    0 => { z = z_in as *const u8; __state = 4; }
                    2 => {
                        if {
                                    v =
                                        unsafe { *z.offset(0 as isize) } as u32 - '0' as i32 as u32;
                                    v
                                } < 10 as u32 {
                            __state = 11;
                        } else { __state = 12; }
                    }
                    3 => { m_state = 1; __state = 13; }
                    4 => { neg = 0; __state = 5; }
                    5 => { s = 0 as u64; __state = 6; }
                    6 => { d = 0; __state = 7; }
                    7 => { m_state = 0; __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 2; }
                    10 => {
                        if unsafe { *z } as i32 == '.' as i32 {
                            __state = 38;
                        } else { __state = 39; }
                    }
                    11 => { __state = 3; }
                    12 => {
                        if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                            __state = 24;
                        } else { __state = 25; }
                    }
                    13 => { s = v as u64; __state = 14; }
                    14 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 15;
                    }
                    15 => {
                        if {
                                    v =
                                        unsafe { *z.offset(0 as isize) } as u32 - '0' as i32 as u32;
                                    v
                                } < 10 as u32 {
                            __state = 16;
                        } else { __state = 10; }
                    }
                    16 => { s = s * 10 as u64 + v as u64; __state = 17; }
                    17 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 18;
                    }
                    18 => {
                        if s >=
                                ((4294967295u32 as u64 | (4294967295u32 as u64) << 32) -
                                        9 as u64) / 10 as u64 {
                            __state = 19;
                        } else { __state = 15; }
                    }
                    19 => { m_state = 9; __state = 20; }
                    20 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                                        usize)
                                        } as i32 & 4 != 0 {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => { __state = 10; }
                    22 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 23;
                    }
                    23 => {
                        { let __p = &mut d; let __t = *__p; *__p += 1; __t };
                        __state = 20;
                    }
                    24 => { neg = 1; __state = 26; }
                    25 => {
                        if unsafe { *z.offset(0 as isize) } as i32 == '+' as i32 {
                            __state = 29;
                        } else { __state = 30; }
                    }
                    26 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 27;
                    }
                    27 => {
                        if {
                                    v =
                                        unsafe { *z.offset(0 as isize) } as u32 - '0' as i32 as u32;
                                    v
                                } < 10 as u32 {
                            __state = 28;
                        } else { __state = 10; }
                    }
                    28 => { __state = 3; }
                    29 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 31;
                    }
                    30 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                                        usize)
                                        } as i32 & 1 != 0 {
                            __state = 33;
                        } else { __state = 34; }
                    }
                    31 => {
                        if {
                                    v =
                                        unsafe { *z.offset(0 as isize) } as u32 - '0' as i32 as u32;
                                    v
                                } < 10 as u32 {
                            __state = 32;
                        } else { __state = 10; }
                    }
                    32 => { __state = 3; }
                    33 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 36;
                    }
                    34 => { s = 0 as u64; __state = 10; }
                    35 => { __state = 2; }
                    36 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                                        usize)
                                        } as i32 & 1 != 0 {
                            __state = 33;
                        } else { __state = 35; }
                    }
                    37 => {
                        if unsafe { *z } as i32 == 'e' as i32 ||
                                unsafe { *z } as i32 == 'E' as i32 {
                            __state = 54;
                        } else { __state = 53; }
                    }
                    38 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 40;
                    }
                    39 => {
                        if m_state == 0 { __state = 51; } else { __state = 37; }
                    }
                    40 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                                        usize)
                                        } as i32 & 4 != 0 {
                            __state = 42;
                        } else { __state = 43; }
                    }
                    41 => { m_state |= 2; __state = 37; }
                    42 => { m_state |= 1; __state = 44; }
                    43 => {
                        if m_state == 0 { __state = 49; } else { __state = 41; }
                    }
                    44 => {
                        if s <
                                ((4294967295u32 as u64 | (4294967295u32 as u64) << 32) -
                                        9 as u64) / 10 as u64 {
                            __state = 46;
                        } else { __state = 47; }
                    }
                    45 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe {
                                                                *{
                                                                        let __p = &mut z;
                                                                        *__p = unsafe { (*__p).offset(1) };
                                                                        *__p
                                                                    }
                                                            } as u8 as usize)
                                        } as i32 & 4 != 0 {
                            __state = 44;
                        } else { __state = 41; }
                    }
                    46 => {
                        s =
                            s * 10 as u64 + unsafe { *z.offset(0 as isize) } as u64 -
                                '0' as i32 as u64;
                        __state = 48;
                    }
                    47 => { m_state = 11; __state = 45; }
                    48 => {
                        { let __p = &mut d; let __t = *__p; *__p -= 1; __t };
                        __state = 45;
                    }
                    49 => { *p_result = 0.0; __state = 50; }
                    50 => { return 0; }
                    51 => { *p_result = 0.0; __state = 52; }
                    52 => { return 0; }
                    53 => {
                        if s == 0 as u64 { __state = 72; } else { __state = 73; }
                    }
                    54 => { __state = 55; }
                    55 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 56;
                    }
                    56 => {
                        if unsafe { *z } as i32 == '-' as i32 {
                            __state = 58;
                        } else { __state = 59; }
                    }
                    57 => {
                        if {
                                    v =
                                        unsafe { *z.offset(0 as isize) } as u32 - '0' as i32 as u32;
                                    v
                                } < 10 as u32 {
                            __state = 63;
                        } else { __state = 64; }
                    }
                    58 => { esign = -1; __state = 60; }
                    59 => { esign = 1; __state = 61; }
                    60 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 57;
                    }
                    61 => {
                        if unsafe { *z } as i32 == '+' as i32 {
                            __state = 62;
                        } else { __state = 57; }
                    }
                    62 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 57;
                    }
                    63 => { exp = v as i32; __state = 65; }
                    64 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(-1) };
                            __t
                        };
                        __state = 53;
                    }
                    65 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 66;
                    }
                    66 => { m_state |= 2; __state = 67; }
                    67 => {
                        if {
                                    v =
                                        unsafe { *z.offset(0 as isize) } as u32 - '0' as i32 as u32;
                                    v
                                } < 10 as u32 {
                            __state = 69;
                        } else { __state = 68; }
                    }
                    68 => { d += esign * exp; __state = 53; }
                    69 => {
                        exp =
                            if exp < 10000 {
                                    (exp * 10) as u32 + v
                                } else { 10000 as u32 } as i32;
                        __state = 70;
                    }
                    70 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 67;
                    }
                    71 => {
                        if neg != 0 { __state = 76; } else { __state = 75; }
                    }
                    72 => { *p_result = 0.0; __state = 74; }
                    73 => {
                        *p_result = sqlite3_fp10_convert2(s, d);
                        __state = 71;
                    }
                    74 => { m_state |= 4; __state = 71; }
                    75 => { { let _ = 0; }; __state = 77; }
                    76 => { *p_result = -*p_result; __state = 75; }
                    77 => {
                        if unsafe { *z.offset(0 as isize) } as i32 == 0 {
                            __state = 79;
                        } else { __state = 78; }
                    }
                    78 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                                        usize)
                                        } as i32 & 1 != 0 {
                            __state = 81;
                        } else { __state = 80; }
                    }
                    79 => { return m_state; }
                    80 => { return (4294967280u32 | m_state as u32) as i32; }
                    81 => {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        __state = 83;
                    }
                    82 => {
                        if unsafe { *z.offset(0 as isize) } as i32 == 0 {
                            __state = 84;
                        } else { __state = 80; }
                    }
                    83 => {
                        if unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z } as u8 as usize)
                                        } as i32 & 1 != 0 {
                            __state = 81;
                        } else { __state = 82; }
                    }
                    84 => { return m_state; }
                    _ => {}
                }
            }
        }

        /// True for a negative value
        /// mantissa
        /// Value is s * pow(10,d)
        /// 1: digit seen 2: fp 4: hard-zero
        /// Value of a single digit
        /// if decimal point is present
        /// if exponent is present
        /// get sign of exponent
        /// copy digits to exponent
        /// Leave z[0] at 'e' or '+' or '-',
        ///* so that the return is 0 or -1
        /// Convert s*pow(10,d) into real
        /// return true if number and no extra non-whitespace characters after
        /// SQLITE_OMIT_FLOATING_POINT
        unreachable!();
    }
}

///* Try to convert z into an unsigned 32-bit integer.  Return true on
///* success and false if there is an error.
///*
///* Only decimal notation is accepted.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_u_int32(z: *const i8, p_i: &mut u32) -> i32 {
    unsafe {
        let mut v: u64 = 0 as u64;
        let mut i: i32 = 0;
        {
            i = 0;
            '__b19: loop {
                if !(unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z.offset(i as isize) } as u8 as
                                                        usize)
                                        } as i32 & 4 != 0) {
                    break '__b19;
                }
                '__c19: loop {
                    v =
                        v * 10 as u64 + unsafe { *z.offset(i as isize) } as u64 -
                            '0' as i32 as u64;
                    if v > 4294967296i64 as u64 { *p_i = 0 as u32; return 0; }
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if i == 0 || unsafe { *z.offset(i as isize) } as i32 != 0 {
            *p_i = 0 as u32;
            return 0;
        }
        *p_i = v as u32;
        return 1;
    }
}

///* Return a 32-bit integer value extracted from a string.  If the
///* string is not an integer, just return 0.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_atoi(z: *const i8) -> i32 {
    let mut x: i32 = 0;
    sqlite3_get_int32(z, &mut x);
    return x;
}

///* Convert an integer into a LogEst.  In other words, compute an
///* approximation for 10*log2(x).
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_log_est(mut x: u64) -> LogEst {
    unsafe {
        let mut y: LogEst = 40 as LogEst;
        if x < 8 as u64 {
            if x < 2 as u64 { return 0 as LogEst; }
            while x < 8 as u64 { y -= 10 as LogEst; x <<= 1 as u64; }
        } else {
            while x > 255 as u64 { y += 40 as LogEst; x >>= 4 as u64; }
            while x > 15 as u64 { y += 10 as LogEst; x >>= 1 as u64; }
        }
        return (a_1[(x & 7 as u64) as usize] as i32 + y as i32 - 10) as
                LogEst;
    }
}

///* Find (an approximate) sum of two LogEst values.  This computation is
///* not a simple "+" operator because LogEst is stored as a logarithmic
///* value.
///*
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_log_est_add(a: LogEst, b: LogEst) -> LogEst {
    if a as i32 >= b as i32 {
        if a as i32 > b as i32 + 49 { return a; }
        if a as i32 > b as i32 + 31 { return (a as i32 + 1) as LogEst; }
        return (a as i32 + x_1[(a as i32 - b as i32) as usize] as i32) as
                LogEst;
    } else {
        if b as i32 > a as i32 + 49 { return b; }
        if b as i32 > a as i32 + 31 { return (b as i32 + 1) as LogEst; }
        return (b as i32 + x_1[(b as i32 - a as i32) as usize] as i32) as
                LogEst;
    }
}

///* Convert a double into a LogEst
///* In other words, compute an approximation for 10*log2(x).
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_log_est_from_double(mut x: f64) -> LogEst {
    let mut a: u64 = 0 as u64;
    let mut e: LogEst = 0 as LogEst;
    { let _ = 0; };
    if x <= 1 as f64 { return 0 as LogEst; }
    if x <= 2000000000 as f64 { return sqlite3_log_est(x as u64); }
    unsafe {
        memcpy(&raw mut a as *mut (), &raw mut x as *const (), 8 as u64)
    };
    e = ((a >> 52) - 1022 as u64) as LogEst;
    return (e as i32 * 10) as LogEst;
}

///* Convert a LogEst into an integer.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_log_est_to_int(mut x: LogEst) -> u64 {
    let mut n: u64 = 0 as u64;
    n = (x as i32 % 10) as u64;
    x /= 10 as LogEst;
    if n >= 5 as u64 {
        n -= 2 as u64;
    } else if n >= 1 as u64 { n -= 1 as u64; }
    if x as i32 > 60 {
        return (4294967295u32 as i64 | (2147483647 as i64) << 32) as u64;
    }
    return if x as i32 >= 3 {
            (n + 8 as u64) << x as i32 - 3
        } else { n + 8 as u64 >> 3 - x as i32 };
}

///* Add a new name/number pair to a VList.  This might require that the
///* VList object be reallocated, so return the new VList.  If an OOM
///* error occurs, the original VList returned and the
///* db->mallocFailed flag is set.
///*
///* A VList is really just an array of integers.  To destroy a VList,
///* simply pass it to sqlite3DbFree().
///*
///* The first integer is the number of integers allocated for the whole
///* VList.  The second integer is the number of integers actually used.
///* Each name/number pair is encoded by subsequent groups of 3 or more
///* integers.
///*
///* Each name/number pair starts with two integers which are the numeric
///* value for the pair and the size of the name/number pair, respectively.
///* The text name overlays one or more following integers.  The text name
///* is always zero-terminated.
///*
///* Conceptually:
///*
///*    struct VList {
///*      int nAlloc;   // Number of allocated slots
///*      int nUsed;    // Number of used slots
///*      struct VListEntry {
///*        int iValue;    // Value for this entry
///*        int nSlot;     // Slots used by this entry
///*        // ... variable name goes here
///*      } a[0];
///*    }
///*
///* During code generation, pointers to the variable names within the
///* VList are taken.  When that happens, nAlloc is set to zero as an
///* indication that the VList may never again be enlarged, since the
///* accompanying realloc() would invalidate the pointers.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_v_list_add(db: *mut Sqlite3, mut p_in: *mut VList,
    z_name: *const i8, n_name: i32, i_val: i32) -> *mut VList {
    let mut n_int: i32 = 0;
    /// number of sizeof(int) objects needed for zName
    let mut z: *mut i8 = core::ptr::null_mut();
    /// Pointer to where zName will be stored
    let mut i: i32 = 0;

    /// Index in pIn[] where zName is stored
    (n_int = n_name / 4 + 3);
    { let _ = 0; };
    if p_in == core::ptr::null_mut() ||
            unsafe { *p_in.offset(1 as isize) } + n_int >
                unsafe { *p_in.offset(0 as isize) } {
        /// Enlarge the allocation
        let n_alloc: Sqlite3Int64 =
            if !(p_in).is_null() {
                    2 as Sqlite3Int64 *
                        unsafe { *p_in.offset(0 as isize) } as Sqlite3Int64
                } else { 10 as Sqlite3Int64 } + n_int as Sqlite3Int64;
        let p_out: *mut VList =
            unsafe {
                    sqlite3_db_realloc(db, p_in as *mut (),
                        n_alloc as u64 * core::mem::size_of::<i32>() as u64)
                } as *mut VList;
        if p_out == core::ptr::null_mut() { return p_in; }
        if p_in == core::ptr::null_mut() {
            unsafe { *p_out.offset(1 as isize) = 2 };
        }
        p_in = p_out;
        unsafe { *p_in.offset(0 as isize) = n_alloc as VList };
    }
    i = unsafe { *p_in.offset(1 as isize) };
    unsafe { *p_in.offset(i as isize) = i_val };
    unsafe { *p_in.offset((i + 1) as isize) = n_int };
    z = unsafe { &raw mut *p_in.offset((i + 2) as isize) } as *mut i8;
    unsafe { *p_in.offset(1 as isize) = i + n_int };
    { let _ = 0; };
    unsafe { memcpy(z as *mut (), z_name as *const (), n_name as u64) };
    unsafe { *z.offset(n_name as isize) = 0 as i8 };
    return p_in;
}

///* Return a pointer to the name of a variable in the given VList that
///* has the value iVal.  Or return a NULL if there is no such variable in
///* the list
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_v_list_num_to_name(p_in: *mut VList, i_val: i32)
    -> *const i8 {
    let mut i: i32 = 0;
    let mut mx: i32 = 0;
    if p_in == core::ptr::null_mut() { return core::ptr::null(); }
    mx = unsafe { *p_in.offset(1 as isize) };
    i = 2;
    '__b23: loop {
        '__c23: loop {
            if unsafe { *p_in.offset(i as isize) } == i_val {
                return unsafe { &raw mut *p_in.offset((i + 2) as isize) } as
                            *mut i8 as *const i8;
            }
            i += unsafe { *p_in.offset((i + 1) as isize) };
            break '__c23;
        }
        if !(i < mx) { break '__b23; }
    }
    return core::ptr::null();
}

///* Return the number of the variable named zName, if it is in VList.
///* or return 0 if there is no such variable.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_v_list_name_to_num(p_in: *mut VList,
    z_name: *const i8, n_name: i32) -> i32 {
    let mut i: i32 = 0;
    let mut mx: i32 = 0;
    if p_in == core::ptr::null_mut() { return 0; }
    mx = unsafe { *p_in.offset(1 as isize) };
    i = 2;
    '__b24: loop {
        '__c24: loop {
            let z: *const i8 =
                unsafe { &raw mut *p_in.offset((i + 2) as isize) } as
                    *const i8;
            if unsafe { strncmp(z, z_name, n_name as u64) } == 0 &&
                    unsafe { *z.offset(n_name as isize) } as i32 == 0 {
                return unsafe { *p_in.offset(i as isize) };
            }
            i += unsafe { *p_in.offset((i + 1) as isize) };
            break '__c24;
        }
        if !(i < mx) { break '__b24; }
    }
    return 0;
}

///* Write a 64-bit variable-length integer to memory starting at p[0].
///* The length of data write will be between 1 and 9 bytes.  The number
///* of bytes written is returned.
///*
///* A variable-length integer consists of the lower 7 bits of each byte
///* for all bytes that have the 8th bit set and one byte with the 8th
///* bit clear.  Except, if we get to the 9th byte, it stores the full
///* 8 bits and is the last byte.
extern "C" fn put_varint64(p: *mut u8, mut v: u64) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut buf: [u8; 10] = [0; 10];
    if v & (4278190080u32 as u64) << 32 != 0 {
        unsafe { *p.offset(8 as isize) = v as u8 };
        v >>= 8 as u64;
        {
            i = 7;
            '__b25: loop {
                if !(i >= 0) { break '__b25; }
                '__c25: loop {
                    unsafe {
                        *p.offset(i as isize) = (v & 127 as u64 | 128 as u64) as u8
                    };
                    v >>= 7 as u64;
                    break '__c25;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        return 9;
    }
    n = 0;
    '__b26: loop {
        '__c26: loop {
            buf[{ let __p = &mut n; let __t = *__p; *__p += 1; __t } as usize]
                = (v & 127 as u64 | 128 as u64) as u8;
            v >>= 7 as u64;
            break '__c26;
        }
        if !(v != 0 as u64) { break '__b26; }
    }
    buf[0 as usize] &= 127 as u8;
    { let _ = 0; };
    {
        { i = 0; j = n - 1 };
        '__b27: loop {
            if !(j >= 0) { break '__b27; }
            '__c27: loop {
                unsafe { *p.offset(i as isize) = buf[j as usize] };
                break '__c27;
            }
            {
                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                { let __p = &mut i; let __t = *__p; *__p += 1; __t }
            };
        }
    }
    return n;
}

///* Routines to read and write variable-length integers.  These used to
///* be defined locally, but now we use the varint routines in the util.c
///* file.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_put_varint(p: *mut u8, v: u64) -> i32 {
    if v <= 127 as u64 {
        unsafe { *p.offset(0 as isize) = (v & 127 as u64) as u8 };
        return 1;
    }
    if v <= 16383 as u64 {
        unsafe {
            *p.offset(0 as isize) = (v >> 7 & 127 as u64 | 128 as u64) as u8
        };
        unsafe { *p.offset(1 as isize) = (v & 127 as u64) as u8 };
        return 2;
    }
    return put_varint64(p, v);
}

///* Read a 64-bit variable-length integer from memory starting at p[0].
///* Return the number of bytes read.  The value is stored in *v.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_get_varint(mut p: *const u8, v: &mut u64) -> u8 {
    let mut a: u32 = 0 as u32;
    let mut b: u32 = 0 as u32;
    let mut s: u32 = 0 as u32;
    if unsafe { *(p as *mut i8).offset(0 as isize) } as i32 >= 0 {
        *v = unsafe { *p } as u64;
        return 1 as u8;
    }
    if unsafe { *(p as *mut i8).offset(1 as isize) } as i32 >= 0 {
        *v =
            (((unsafe { *p.offset(0 as isize) } as i32 & 127) as u32) << 7 |
                    unsafe { *p.offset(1 as isize) } as u32) as u64;
        return 2 as u8;
    }

    /// Verify that constants are precomputed correctly
    { let _ = 0; };
    { let _ = 0; };
    a = (unsafe { *p.offset(0 as isize) } as u32) << 14;
    b = unsafe { *p.offset(1 as isize) } as u32;
    {
        let __n = 2;
        let __p = &mut p;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    a |= unsafe { *p } as u32;
    if (a & 128 as u32 == 0) as i32 != 0 {
        a &= 2080895 as u32;
        b &= 127 as u32;
        b = b << 7;
        a |= b;
        *v = a as u64;
        return 3 as u8;
    }

    /// CSE1 from below
    (a &= 2080895 as u32);
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    b = b << 14;
    b |= unsafe { *p } as u32;
    if (b & 128 as u32 == 0) as i32 != 0 {
        b &= 2080895 as u32;

        /// moved CSE1 up */
        ///    /* a &= (0x7f<<14)|(0x7f);
        (a = a << 7);
        a |= b;
        *v = a as u64;
        return 4 as u8;
    }

    /// a: p0<<14 | p2 (masked) */
    ///  /* b: p1<<14 | p3 (unmasked) */
    ///  /* 1:save off p0<<21 | p1<<14 | p2<<7 | p3 (masked) */
    ///  /* moved CSE1 up */
    ///  /* a &= (0x7f<<14)|(0x7f);
    (b &= 2080895 as u32);
    s = a;

    /// s: p0<<14 | p2 (masked)
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    a = a << 14;
    a |= unsafe { *p } as u32;
    if (a & 128 as u32 == 0) as i32 != 0 {

        /// we can skip these cause they were (effectively) done above
        ///* while calculating s */
        ///    /* a &= (0x7f<<28)|(0x7f<<14)|(0x7f); */
        ///    /* b &= (0x7f<<14)|(0x7f);
        (b = b << 7);
        a |= b;
        s = s >> 18;
        *v = (s as u64) << 32 | a as u64;
        return 5 as u8;
    }

    /// 2:save off p0<<21 | p1<<14 | p2<<7 | p3 (masked)
    (s = s << 7);
    s |= b;

    /// s: p0<<21 | p1<<14 | p2<<7 | p3 (masked)
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    b = b << 14;
    b |= unsafe { *p } as u32;
    if (b & 128 as u32 == 0) as i32 != 0 {

        /// we can skip this cause it was (effectively) done above in calc'ing s */
        ///    /* b &= (0x7f<<28)|(0x7f<<14)|(0x7f);
        (a &= 2080895 as u32);
        a = a << 7;
        a |= b;
        s = s >> 18;
        *v = (s as u64) << 32 | a as u64;
        return 6 as u8;
    }
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    a = a << 14;
    a |= unsafe { *p } as u32;
    if (a & 128 as u32 == 0) as i32 != 0 {
        a &= 4028612735u32;
        b &= 2080895 as u32;
        b = b << 7;
        a |= b;
        s = s >> 11;
        *v = (s as u64) << 32 | a as u64;
        return 7 as u8;
    }

    /// CSE2 from below
    (a &= 2080895 as u32);
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    b = b << 14;
    b |= unsafe { *p } as u32;
    if (b & 128 as u32 == 0) as i32 != 0 {
        b &= 4028612735u32;

        /// moved CSE2 up */
        ///    /* a &= (0x7f<<14)|(0x7f);
        (a = a << 7);
        a |= b;
        s = s >> 4;
        *v = (s as u64) << 32 | a as u64;
        return 8 as u8;
    }
    {
        let __p = &mut p;
        let __t = *__p;
        *__p = unsafe { (*__p).offset(1) };
        __t
    };
    a = a << 15;
    a |= unsafe { *p } as u32;

    /// a: p4<<29 | p6<<15 | p8 (unmasked)
    /// moved CSE2 up */
    ///  /* a &= (0x7f<<29)|(0x7f<<15)|(0xff);
    (b &= 2080895 as u32);
    b = b << 8;
    a |= b;
    s = s << 4;
    b = unsafe { *p.offset(-4 as isize) } as u32;
    b &= 127 as u32;
    b = b >> 3;
    s |= b;
    *v = (s as u64) << 32 | a as u64;
    return 9 as u8;
}

///* Read a 32-bit variable-length integer from memory starting at p[0].
///* Return the number of bytes read.  The value is stored in *v.
///*
///* If the varint stored in p[0] is larger than can fit in a 32-bit unsigned
///* integer, then set *v to 0xffffffff.
///*
///* A MACRO version, getVarint32, is provided which inlines the
///* single-byte case.  All code should use the MACRO version as
///* this function assumes the single-byte case has already been handled.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_get_varint32(p: *const u8, v: &mut u32) -> u8 {
    let mut v64: u64 = 0 as u64;
    let mut n: u8 = 0 as u8;

    /// Assume that the single-byte case has already been handled by
    ///* the getVarint32() macro
    { let _ = 0; };
    if unsafe { *p.offset(1 as isize) } as i32 & 128 == 0 {

        /// This is the two-byte case
        (*v =
            ((unsafe { *p.offset(0 as isize) } as i32 & 127) << 7 |
                    unsafe { *p.offset(1 as isize) } as i32) as u32);
        return 2 as u8;
    }
    if unsafe { *p.offset(2 as isize) } as i32 & 128 == 0 {

        /// This is the three-byte case
        (*v =
            ((unsafe { *p.offset(0 as isize) } as i32 & 127) << 14 |
                        (unsafe { *p.offset(1 as isize) } as i32 & 127) << 7 |
                    unsafe { *p.offset(2 as isize) } as i32) as u32);
        return 3 as u8;
    }

    /// four or more bytes
    (n = sqlite3_get_varint(p, &mut v64));
    { let _ = 0; };
    if v64 & ((1 as u64) << 32) - 1 as u64 != v64 {
        *v = 4294967295u32;
    } else { *v = v64 as u32; }
    return n;
}

///* Return the number of bytes that will be needed to store the given
///* 64-bit integer.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_varint_len(mut v: u64) -> i32 {
    let mut i: i32 = 0;
    {
        i = 1;
        '__b28: loop {
            if !({ v >>= 7 as u64; v } != 0 as u64) { break '__b28; }
            '__c28: loop { { let _ = 0; }; break '__c28; }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return i;
}

///* Compare the 19-character string zNum against the text representation
///* value 2^63:  9223372036854775808.  Return negative, zero, or positive
///* if zNum is less than, equal to, or greater than the string.
///* Note that zNum must contain exactly 19 characters.
///*
///* Unlike memcmp() this routine is guaranteed to return the difference
///* in the values of the last digit if the only difference is in the
///* last digit.  So, for example,
///*
///*      compare2pow63("9223372036854775800", 1)
///*
///* will return -8.
#[allow(unused_doc_comments)]
extern "C" fn compare2pow63(z_num_1: *const i8, incr: i32) -> i32 {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    /// 012345678901234567
    let pow63: *const i8 =
        c"922337203685477580".as_ptr() as *mut i8 as *const i8;
    {
        i = 0;
        '__b29: loop {
            if !(c == 0 && i < 18) { break '__b29; }
            '__c29: loop {
                c =
                    (unsafe { *z_num_1.offset((i * incr) as isize) } as i32 -
                            unsafe { *pow63.offset(i as isize) } as i32) * 10;
                break '__c29;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if c == 0 {
        c =
            unsafe { *z_num_1.offset((18 * incr) as isize) } as i32 -
                '8' as i32;
    }
    return c;
}

///* Convert zNum to a 64-bit signed integer.  zNum must be decimal. This
///* routine does *not* accept hexadecimal notation.
///*
///* Returns:
///*
///*    -1    Not even a prefix of the input text looks like an integer
///*     0    Successful transformation.  Fits in a 64-bit signed integer.
///*     1    Excess non-space text after the integer value
///*     2    Integer too large for a 64-bit signed integer or is malformed
///*     3    Special case of 9223372036854775808
///*
///* length is the number of bytes in the string (bytes, not characters).
///* The string is not necessarily zero-terminated.  The encoding is
///* given by enc.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_atoi64(mut z_num: *const i8, p_num: &mut i64,
    mut length: i32, enc: u8) -> i32 {
    unsafe {
        let mut incr: i32 = 0;
        let mut u: u64 = 0 as u64;
        let mut neg: i32 = 0;
        /// assume positive
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut c: u32 = 0 as u32;
        let mut non_num: i32 = 0;
        /// True if input contains UTF16 with high byte non-zero
        let mut rc: i32 = 0;
        /// Baseline return code
        let mut z_start: *const i8 = core::ptr::null();
        let mut z_end: *const i8 = unsafe { z_num.offset(length as isize) };
        { let _ = 0; };
        if enc as i32 == 1 {
            incr = 1;
        } else {
            incr = 2;
            length &= !1;
            { let _ = 0; };
            {
                i = 3 - enc as i32;
                '__b30: loop {
                    if !(i < length &&
                                    unsafe { *z_num.offset(i as isize) } as i32 == 0) {
                        break '__b30;
                    }
                    '__c30: loop { break '__c30; }
                    i += 2;
                }
            }
            non_num = (i < length) as i32;
            z_end = unsafe { z_num.offset((i ^ 1) as isize) };
            {
                let __n = enc as i32 & 1;
                let __p = &mut z_num;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        while z_num < z_end &&
                unsafe {
                                *(sqlite3_ctype_map.as_ptr() as
                                            *const u8).add(unsafe { *z_num } as u8 as usize)
                            } as i32 & 1 != 0 {
            {
                let __n = incr;
                let __p = &mut z_num;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        if z_num < z_end {
            if unsafe { *z_num } as i32 == '-' as i32 {
                neg = 1;
                {
                    let __n = incr;
                    let __p = &mut z_num;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            } else if unsafe { *z_num } as i32 == '+' as i32 {
                {
                    let __n = incr;
                    let __p = &mut z_num;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            }
        }
        z_start = z_num;
        while z_num < z_end &&
                unsafe { *z_num.offset(0 as isize) } as i32 == '0' as i32 {
            {
                let __n = incr;
                let __p = &mut z_num;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        {
            i = 0;
            '__b33: loop {
                if !(unsafe { z_num.offset(i as isize) } < z_end &&
                                {
                                        c =
                                            unsafe { *z_num.offset(i as isize) } as u32 -
                                                '0' as i32 as u32;
                                        c
                                    } <= 9 as u32) {
                    break '__b33;
                }
                '__c33: loop { u = u * 10 as u64 + c as u64; break '__c33; }
                i += incr;
            }
        }
        if u > (4294967295u32 as i64 | (2147483647 as i64) << 32) as u64 {

            /// This test and assignment is needed only to suppress UB warnings
            ///* from clang and -fsanitize=undefined.  This test and assignment make
            ///* the code a little larger and slower, and no harm comes from omitting
            ///* them, but we must appease the undefined-behavior pharisees.
            (*p_num =
                if neg != 0 {
                    -1 as i64 -
                        (4294967295u32 as i64 | (2147483647 as i64) << 32)
                } else { 4294967295u32 as i64 | (2147483647 as i64) << 32 });
        } else if neg != 0 {
            *p_num = -(u as i64);
        } else { *p_num = u as i64; }
        rc = 0;
        if i == 0 && z_start == z_num {

            /// No digits
            (rc = -1);
        } else if non_num != 0 {

            /// UTF16 with high-order bytes non-zero
            (rc = 1);
        } else if unsafe { z_num.offset(i as isize) } < z_end {
            /// Extra bytes at the end
            let mut jj: i32 = i;
            '__b34: loop {
                '__c34: loop {
                    if (unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z_num.offset(jj as isize) } as u8
                                                            as usize)
                                            } as i32 & 1 == 0) as i32 != 0 {
                        rc = 1;

                        /// Extra non-space text after the integer
                        break '__b34;
                    }
                    jj += incr;
                    break '__c34;
                }
                if !(unsafe { z_num.offset(jj as isize) } < z_end) {
                    break '__b34;
                }
            }
        }
        if i < 19 * incr {

            /// Less than 19 digits, so we know that it fits in 64 bits
            { let _ = 0; };
            return rc;
        } else {

            /// zNum is a 19-digit numbers.  Compare it against 9223372036854775808.
            (j = if i > 19 * incr { 1 } else { compare2pow63(z_num, incr) });
            if j < 0 {

                /// zNum is less than 9223372036854775808 so it fits
                { let _ = 0; };
                return rc;
            } else {
                *p_num =
                    if neg != 0 {
                        -1 as i64 -
                            (4294967295u32 as i64 | (2147483647 as i64) << 32)
                    } else { 4294967295u32 as i64 | (2147483647 as i64) << 32 };
                if j > 0 {

                    /// zNum is greater than 9223372036854775808 so it overflows
                    return 2;
                } else {

                    /// zNum is exactly 9223372036854775808.  Fits if negative.  The
                    ///* special case 2 overflow if positive
                    { let _ = 0; };
                    return if neg != 0 { rc } else { 3 };
                }
            }
        }
    }
}

///* Transform a UTF-8 integer literal, in either decimal or hexadecimal,
///* into a 64-bit signed integer.  This routine accepts hexadecimal literals,
///* whereas sqlite3Atoi64() does not.
///*
///* Returns:
///*
///*     0    Successful transformation.  Fits in a 64-bit signed integer.
///*     1    Excess text after the integer value
///*     2    Integer too large for a 64-bit signed integer or is malformed
///*     3    Special case of 9223372036854775808
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_dec_or_hex_to_i64(z: *const i8, p_out: *mut i64)
    -> i32 {
    unsafe {
        if unsafe { *z.offset(0 as isize) } as i32 == '0' as i32 &&
                (unsafe { *z.offset(1 as isize) } as i32 == 'x' as i32 ||
                    unsafe { *z.offset(1 as isize) } as i32 == 'X' as i32) {
            let mut u: u64 = 0 as u64;
            let mut i: i32 = 0;
            let mut k: i32 = 0;
            {
                i = 2;
                '__b35: loop {
                    if !(unsafe { *z.offset(i as isize) } as i32 == '0' as i32)
                        {
                        break '__b35;
                    }
                    '__c35: loop { break '__c35; }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            {
                k = i;
                '__b36: loop {
                    if !(unsafe {
                                                *(sqlite3_ctype_map.as_ptr() as
                                                            *const u8).add(unsafe { *z.offset(k as isize) } as u8 as
                                                            usize)
                                            } as i32 & 8 != 0) {
                        break '__b36;
                    }
                    '__c36: loop {
                        u =
                            u * 16 as u64 +
                                sqlite3_hex_to_int(unsafe { *z.offset(k as isize) } as i32)
                                    as u64;
                        break '__c36;
                    }
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe {
                memcpy(p_out as *mut (), &raw mut u as *const (), 8 as u64)
            };
            if k - i > 16 { return 2; }
            if unsafe { *z.offset(k as isize) } as i32 != 0 { return 1; }
            return 0;
        } else {
            let mut n: i32 =
                (1073741823 as u64 &
                        unsafe {
                            strspn(z,
                                c"+- \n\t0123456789".as_ptr() as *mut i8 as *const i8)
                        }) as i32;
            if unsafe { *z.offset(n as isize) } != 0 {
                { let __p = &mut n; let __t = *__p; *__p += 1; __t };
            }
            return sqlite3_atoi64(z, unsafe { &mut *p_out }, n, 1 as u8);
        }
    }
}

///* Load the sqlite3.iSysErrno field if that is an appropriate thing
///* to do based on the SQLite error code in rc.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_system_error(db: &mut Sqlite3, mut rc: i32) -> () {
    if rc == 10 | 12 << 8 { return; }
    rc &= 255;
    if rc == 14 || rc == 10 {
        (*db).i_sys_errno = unsafe { sqlite3_os_get_last_error((*db).p_vfs) };
    }
}

///* Helper function for sqlite3Error() - called rarely.  Broken out into
///* a separate routine to avoid unnecessary register saves on entry to
///* sqlite3Error().
extern "C" fn sqlite3_error_finish(db: *mut Sqlite3, err_code: i32) -> () {
    if !(unsafe { (*db).p_err }).is_null() {
        unsafe { sqlite3_value_set_null(unsafe { (*db).p_err }) };
    }
    sqlite3_system_error(unsafe { &mut *db }, err_code);
}

///* Set the current error code to err_code and clear any prior error message.
///* Also set iSysErrno (by calling sqlite3System) if the err_code indicates
///* that would be appropriate.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_error(db: *mut Sqlite3, err_code: i32) -> () {
    { let _ = 0; };
    unsafe { (*db).err_code = err_code };
    if err_code != 0 || !(unsafe { (*db).p_err }).is_null() {
        sqlite3_error_finish(db, err_code);
    } else { unsafe { (*db).err_byte_offset = -1 }; }
}

///* Set the most recent error code and error string for the sqlite
///* handle "db". The error code is set to "err_code".
///*
///* If it is not NULL, string zFormat specifies the format of the
///* error string.  zFormat and any string tokens that follow it are
///* assumed to be encoded in UTF-8.
///*
///* To clear the most recent error for sqlite handle "db", sqlite3Error
///* should be called with err_code set to SQLITE_OK and zFormat set
///* to NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_error_with_msg(db: *mut Sqlite3,
    err_code: i32, z_format: *const i8, mut __va0: ...) -> () {
    { let _ = 0; };
    unsafe { (*db).err_code = err_code };
    sqlite3_system_error(unsafe { &mut *db }, err_code);
    if z_format == core::ptr::null() {
        sqlite3_error(db, err_code);
    } else if !(unsafe { (*db).p_err }).is_null() ||
            {
                    let __v = unsafe { sqlite3_value_new(db) };
                    unsafe { (*db).p_err = __v };
                    __v
                } != core::ptr::null_mut() {
        let mut z: *const i8 = core::ptr::null();
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z = unsafe { sqlite3_vm_printf(db, z_format, ap) };
        ();
        unsafe {
            sqlite3_value_set_str(unsafe { (*db).p_err }, -1, z as *const (),
                1 as u8,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(sqlite3_row_set_clear as *const ())
                    }))
        };
    }
}

///* The equivalent of sqlite3Error(db, SQLITE_OK).  Clear the error state
///* and error message.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_error_clear(db: &mut Sqlite3) -> () {
    { let _ = 0; };
    (*db).err_code = 0;
    (*db).err_byte_offset = -1;
    if !((*db).p_err).is_null() {
        unsafe { sqlite3_value_set_null((*db).p_err) };
    }
}

///* Convert a BLOB literal of the form "x'hhhhhh'" into its binary
///* value.  Return a pointer to its binary value.  Space to hold the
///* binary value has been obtained from malloc and must be freed by
///* the calling routine.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_hex_to_blob(db: *mut Sqlite3, z: *const i8,
    mut n: i32) -> *mut () {
    let mut z_blob: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    z_blob =
        unsafe { sqlite3_db_malloc_raw_nn(db, (n / 2 + 1) as u64) } as
            *mut i8;
    { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
    if !(z_blob).is_null() {
        {
            i = 0;
            '__b37: loop {
                if !(i < n) { break '__b37; }
                '__c37: loop {
                    unsafe {
                        *z_blob.offset((i / 2) as isize) =
                            ((sqlite3_hex_to_int(unsafe { *z.offset(i as isize) } as
                                                        i32) as i32) << 4 |
                                    sqlite3_hex_to_int(unsafe { *z.offset((i + 1) as isize) } as
                                                i32) as i32) as i8
                    };
                    break '__c37;
                }
                i += 2;
            }
        }
        unsafe { *z_blob.offset((i / 2) as isize) = 0 as i8 };
    }
    return z_blob as *mut ();
}

///* Attempt to add, subtract, or multiply the 64-bit signed value iB against
///* the other 64-bit signed integer at *pA and store the result in *pA.
///* Return 0 on success.  Or if the operation would have resulted in an
///* overflow, leave *pA unchanged and return 1.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_add_int64(p_a: &mut i64, i_b: i64) -> i32 {
    let i_a: i64 = *p_a;
    if i_b >= 0 as i64 {
        if i_a > 0 as i64 &&
                (4294967295u32 as i64 | (2147483647 as i64) << 32) - i_a < i_b
            {
            return 1;
        }
    } else {
        if i_a < 0 as i64 &&
                -(i_a + (4294967295u32 as i64 | (2147483647 as i64) << 32)) >
                    i_b + 1 as i64 {
            return 1;
        }
    }
    *p_a += i_b;
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_sub_int64(p_a: *mut i64, i_b: i64) -> i32 {
    if i_b == -1 as i64 - (4294967295u32 as i64 | (2147483647 as i64) << 32) {
        if unsafe { *p_a } >= 0 as i64 { return 1; }
        unsafe { *p_a -= i_b };
        return 0;
    } else { return sqlite3_add_int64(unsafe { &mut *p_a }, -i_b); }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_mul_int64(p_a: &mut i64, i_b: i64) -> i32 {
    let i_a: i64 = *p_a;
    if i_b > 0 as i64 {
        if i_a > (4294967295u32 as i64 | (2147483647 as i64) << 32) / i_b {
            return 1;
        }
        if i_a <
                (-1 as i64 -
                        (4294967295u32 as i64 | (2147483647 as i64) << 32)) / i_b {
            return 1;
        }
    } else if i_b < 0 as i64 {
        if i_a > 0 as i64 {
            if i_b <
                    (-1 as i64 -
                            (4294967295u32 as i64 | (2147483647 as i64) << 32)) / i_a {
                return 1;
            }
        } else if i_a < 0 as i64 {
            if i_b ==
                    -1 as i64 -
                        (4294967295u32 as i64 | (2147483647 as i64) << 32) {
                return 1;
            }
            if i_a ==
                    -1 as i64 -
                        (4294967295u32 as i64 | (2147483647 as i64) << 32) {
                return 1;
            }
            if -i_a >
                    (4294967295u32 as i64 | (2147483647 as i64) << 32) / -i_b {
                return 1;
            }
        }
    }
    *p_a = i_a * i_b;
    return 0;
}

///* Compute the absolute value of a 32-bit signed integer, if possible.  Or
///* if the integer has a value of -2147483648, return +2147483647
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_abs_int32(x: i32) -> i32 {
    if x >= 0 { return x; }
    if x == 2147483648u32 as i32 { return 2147483647; }
    return -x;
}

///* Compute an 8-bit hash on a string that is insensitive to case differences
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_str_i_hash(mut z: *const i8) -> u8 {
    unsafe {
        let mut h: u8 = 0 as u8;
        if z == core::ptr::null() { return 0 as u8; }
        while unsafe { *z.offset(0 as isize) } != 0 {
            h +=
                unsafe {
                            *(sqlite3_upper_to_lower.as_ptr() as
                                        *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                        usize)
                        } as i32 as u8;
            {
                let __p = &mut z;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        return h;
    }
}

///* Read or write a four-byte big-endian integer value.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get4byte(p: *const u8) -> u32 {
    return (unsafe { *p.offset(0 as isize) } as u32) << 24 |
                    ((unsafe { *p.offset(1 as isize) } as i32) << 16) as u32 |
                ((unsafe { *p.offset(2 as isize) } as i32) << 8) as u32 |
            unsafe { *p.offset(3 as isize) } as u32;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_put4byte(p: *mut u8, v: u32) -> () {
    unsafe { *p.offset(0 as isize) = (v >> 24) as u8 };
    unsafe { *p.offset(1 as isize) = (v >> 16) as u8 };
    unsafe { *p.offset(2 as isize) = (v >> 8) as u8 };
    unsafe { *p.offset(3 as isize) = v as u8 };
}

#[repr(C)]
#[derive(Copy, Clone)]
union Sqlite3Int64ToTextU0N24sqlite3Int64ToTextU0 {
    a: [i8; 21],
    force_alignment: u16,
}

static a_base: [u64; 27] =
    [9223372036854775808u64, 11529215046068469760u64, 14411518807585587200u64,
            18014398509481984000u64, 11258999068426240000u64,
            14073748835532800000u64, 17592186044416000000u64,
            10995116277760000000u64, 13743895347200000000u64,
            17179869184000000000u64, 10737418240000000000u64,
            13421772800000000000u64, 16777216000000000000u64,
            10485760000000000000u64, 13107200000000000000u64,
            16384000000000000000u64, 10240000000000000000u64,
            12800000000000000000u64, 16000000000000000000u64,
            10000000000000000000u64, 12500000000000000000u64,
            15625000000000000000u64, 9765625000000000000u64,
            12207031250000000000u64, 15258789062500000000u64,
            9536743164062500000u64, 11920928955078125000u64];

static a_scale: [u64; 26] =
    [9244100769003082158u64, 14934650266808366570u64, 12064114410120881697u64,
            9745314011399999080u64, 15744403932561434696u64,
            12718228212127407596u64, 10273702932711667006u64,
            16598062275523971834u64, 13407807929942597099u64,
            10830740992659433045u64, 17498005798264095394u64,
            14134776518227074636u64, 11417981541647679048u64,
            14757395258967641292u64, 14901161193847656250u64,
            12037062152420224081u64, 9723461371658033917u64,
            15709099088952724969u64, 12689709186578246116u64,
            10250665447337476733u64, 16560843210556190337u64,
            13377742608693866209u64, 10806454419566533849u64,
            17458768723248864463u64, 14103081061443981063u64,
            11392378155556871081u64];

static a_scale_lo: [u32; 26] =
    [542869869 as u32, 1376144557 as u32, 2938827448u32, 1517765799 as u32,
            2939790453u32, 3180165454u32, 1417589883 as u32, 213165475 as u32,
            2465418594u32, 980027385 as u32, 4209144473u32, 2862080332u32,
            2002690661 as u32, 3435973836u32, 0 as u32, 2576388278u32,
            1772103867 as u32, 3893260104u32, 1589665232 as u32,
            341348116 as u32, 2400610505u32, 1838497324 as u32,
            1253945104 as u32, 3160619833u32, 176566145 as u32,
            1812439746 as u32];

static mut a_1: [i16; 8] =
    [0 as i16, 2 as i16, 3 as i16, 5 as i16, 6 as i16, 7 as i16, 8 as i16,
            9 as i16];

static x_1: [u8; 32] =
    [10 as u8, 10 as u8, 9 as u8, 9 as u8, 8 as u8, 8 as u8, 7 as u8, 7 as u8,
            7 as u8, 6 as u8, 6 as u8, 6 as u8, 5 as u8, 5 as u8, 5 as u8,
            4 as u8, 4 as u8, 4 as u8, 4 as u8, 3 as u8, 3 as u8, 3 as u8,
            3 as u8, 3 as u8, 3 as u8, 2 as u8, 2 as u8, 2 as u8, 2 as u8,
            2 as u8, 2 as u8, 2 as u8];

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
    static sqlite3_upper_to_lower: [u8; 0];
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
    fn strlen(__s: *const i8)
    -> u64;
    static mut sqlite3_std_type: [*const i8; 0];
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
    fn __inline_isnanf(__x: f32)
    -> i32;
    fn __inline_isnand(__x: f64)
    -> i32;
    fn __inline_isnanl(__x: f64)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_m_printf(_: *mut Sqlite3, _: *const i8, ...)
    -> *mut i8;
    fn sqlite3_vm_printf(_: *mut Sqlite3, _: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_set_string(_: *mut *mut i8, _: *mut Sqlite3, _: *const i8)
    -> ();
    static sqlite3_ctype_map: [u8; 0];
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
    static mut sqlite3Config: Sqlite3Config;
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
    fn sqlite3_utf16_byte_len(p_data_1: *const (), n_byte_1: i32,
    n_char_1: i32)
    -> i32;
    fn sqlite3_utf8_char_len(p_data_1: *const i8, n_byte_1: i32)
    -> i32;
    fn sqlite3_utf8_read(_: *mut *const u8)
    -> u32;
    fn sqlite3_utf8_read_limited(_: *const u8, _: i32, _: *mut u32)
    -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
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
    fn strspn(__s: *const i8, __charset: *const i8)
    -> u64;
    fn sqlite3_value_set_null(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_value_new(_: *mut Sqlite3)
    -> *mut Sqlite3Value;
    fn sqlite3_value_set_str(_: *mut Sqlite3Value, _: i32, _: *const (),
    _: u8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
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
    fn sqlite3_get_boolean(z: *const i8, _: u8)
    -> u8;
    fn sqlite3ValueText(_: *mut Sqlite3Value, _: u8)
    -> *const ();
    fn sqlite3_value_is_of_class(_: *const Sqlite3Value,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3ValueBytes(_: *mut Sqlite3Value, _: u8)
    -> i32;
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_result_int_real(_: *mut Sqlite3Context)
    -> ();
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
    static mut sqlite3a_l_tb: *const u8;
    static mut sqlite3a_e_qb: *const u8;
    static mut sqlite3a_g_tb: *const u8;
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
    fn __builtin_clzll(_: u64)
    -> i32;
    fn __builtin_huge_valf()
    -> f32;
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
