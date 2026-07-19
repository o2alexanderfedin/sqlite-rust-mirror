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
    AggInfo, AggInfoCol, AggInfoFunc, AuthContext, Bft, Bitmask, Bitvec,
    BusyHandler, CollSeq, Column, Cte, DbFixer, Expr, ExprList, ExprListItem,
    ExprListItemS0, FKey, FpDecode, FuncDef, FuncDefHash, FuncDestructor,
    IdList, IdListItem, Index, IndexedExpr, KeyInfo, LogEst, Module,
    NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema, Select,
    SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str, SrcItem,
    SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
    TriggerStep, UnpackedRecord, Upsert, VList, VTable, Walker, WhereInfo,
    Window, With, YnVar,
};
use crate::vdbe_h::{Mem, SubProgram, SubrtnSig, Vdbe, VdbeOp, VdbeOpList};

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

/// Structure used to pass information throughout the Walker in order to
///* implement sqlite3ReferencesSrcList().
#[repr(C)]
#[derive(Copy, Clone)]
struct RefSrcList {
    db: *mut Sqlite3,
    p_ref: *mut SrcList,
    n_exclude: i64,
    ai_exclude: *mut i32,
}

///* An instance of the following structure is used by the tree walker
///* to determine if an expression can be evaluated by reference to the
///* index only, without having to do a search for the corresponding
///* table entry.  The IdxCover.pIdx field is the index.  IdxCover.iCur
///* is the cursor for the table.
#[repr(C)]
#[derive(Copy, Clone)]
struct IdxCover {
    p_idx: *mut Index,
    i_cur: i32,
}

///* This is a SELECT-node callback for the expression walker that
///* always "fails".  By "fail" in this case, we mean set
///* pWalker->eCode to zero and abort.
///*
///* This callback is used by multiple expression walkers.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_walk_fail(p_walker: *mut Walker,
    not_used: *mut Select) -> i32 {
    { let _ = not_used; };
    unsafe { (*p_walker).e_code = 0 as u16 };
    return 2;
}

///* Allocate a single new register for use to hold some intermediate result.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_temp_reg(p_parse: &mut Parse) -> i32 {
    if (*p_parse).n_temp_reg as i32 == 0 {
        return { let __p = &mut (*p_parse).n_mem; *__p += 1; *__p };
    }
    return (*p_parse).a_temp_reg[{
                    let __p = &mut (*p_parse).n_temp_reg;
                    *__p -= 1;
                    *__p
                } as usize];
}

///* Deallocate a register, making available for reuse for some other
///* purpose.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_release_temp_reg(p_parse: &mut Parse, i_reg: i32)
    -> () {
    if i_reg != 0 {
        if ((*p_parse).n_temp_reg as i32) <
                (core::mem::size_of::<[i32; 8]>() as u64 /
                        core::mem::size_of::<i32>() as u64) as i32 {
            (*p_parse).a_temp_reg[{
                            let __p = &mut (*p_parse).n_temp_reg;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        } as usize] = i_reg;
        }
    }
}

///* Allocate or deallocate a block of nReg consecutive registers.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_temp_range(p_parse: *mut Parse, n_reg: i32)
    -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    if n_reg == 1 { return sqlite3_get_temp_reg(unsafe { &mut *p_parse }); }
    i = unsafe { (*p_parse).i_range_reg };
    n = unsafe { (*p_parse).n_range_reg };
    if n_reg <= n {
        unsafe { (*p_parse).i_range_reg += n_reg };
        unsafe { (*p_parse).n_range_reg -= n_reg };
    } else {
        i = unsafe { (*p_parse).n_mem } + 1;
        unsafe { (*p_parse).n_mem += n_reg };
    }
    return i;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_release_temp_range(p_parse: *mut Parse, i_reg: i32,
    n_reg: i32) -> () {
    if n_reg == 1 {
        sqlite3_release_temp_reg(unsafe { &mut *p_parse }, i_reg);
        return;
    }
    if n_reg > unsafe { (*p_parse).n_range_reg } {
        unsafe { (*p_parse).n_range_reg = n_reg };
        unsafe { (*p_parse).i_range_reg = i_reg };
    }
}

///* Mark all temporary registers as being unavailable for reuse.
///*
///* Always invoke this procedure after coding a subroutine or co-routine
///* that might be invoked from other parts of the code, to ensure that
///* the sub/co-routine does not use registers in common with the code that
///* invokes the sub/co-routine.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_clear_temp_reg_cache(p_parse: &mut Parse) -> () {
    (*p_parse).n_temp_reg = 0 as u8;
    (*p_parse).n_range_reg = 0;
}

///* Make sure sufficient registers have been allocated so that
///* iReg is a valid register number.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_touch_register(p_parse: &mut Parse, i_reg: i32)
    -> () {
    if (*p_parse).n_mem < i_reg { (*p_parse).n_mem = i_reg; }
}

///* This routine is the core allocator for Expr nodes.
///*
///* Construct a new expression node and return a pointer to it.  Memory
///* for this node and for the pToken argument is a single allocation
///* obtained from sqlite3DbMalloc().  The calling function
///* is responsible for making sure the node eventually gets freed.
///*
///* If dequote is true, then the token (if it exists) is dequoted.
///* If dequote is false, no dequoting is performed.  The deQuote
///* parameter is ignored if pToken is NULL or if the token does not
///* appear to be quoted.  If the quotes were of the form "..." (double-quotes)
///* then the EP_DblQuoted flag is set on the expression node.
///*
///* Special case (tag-20240227-a):  If op==TK_INTEGER and pToken points to
///* a string that can be translated into a 32-bit integer, then the token is
///* not stored in u.zToken.  Instead, the integer values is written
///* into u.iValue and the EP_IntValue flag is set. No extra storage
///* is allocated to hold the integer text and the dequote flag is ignored.
///* See also tag-20240227-b.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_alloc(db: *mut Sqlite3, op: i32,
    p_token: *const Token, dequote: i32) -> *mut Expr {
    unsafe {
        unsafe {
            let mut p_new: *mut Expr = core::ptr::null_mut();
            let n_extra: i32 =
                if !(p_token).is_null() {
                        (unsafe { (*p_token).n }) + 1 as u32
                    } else { 0 as u32 } as i32;
            { let _ = 0; };
            p_new =
                unsafe {
                        sqlite3_db_malloc_raw_nn(db,
                            core::mem::size_of::<Expr>() as u64 + n_extra as u64)
                    } as *mut Expr;
            if !(p_new).is_null() {
                unsafe {
                    memset(p_new as *mut (), 0,
                        core::mem::size_of::<Expr>() as u64)
                };
                unsafe { (*p_new).op = op as u8 };
                unsafe { (*p_new).i_agg = -1 as i16 };
                if n_extra != 0 {
                    { let _ = 0; };
                    unsafe {
                        (*p_new).u.z_token =
                            unsafe { &raw mut *p_new.offset(1 as isize) } as *mut i8
                    };
                    { let _ = 0; };
                    if unsafe { (*p_token).n } != 0 {
                        unsafe {
                            memcpy(unsafe { (*p_new).u.z_token } as *mut (),
                                unsafe { (*p_token).z } as *const (),
                                unsafe { (*p_token).n } as u64)
                        };
                    }
                    unsafe {
                        *unsafe {
                                    (*p_new).u.z_token.add(unsafe { (*p_token).n } as usize)
                                } = 0 as i8
                    };
                    if dequote != 0 &&
                            unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe {
                                                                *unsafe { (*p_new).u.z_token.offset(0 as isize) }
                                                            } as u8 as usize)
                                        } as i32 & 128 != 0 {
                        unsafe { sqlite3_dequote_expr(p_new) };
                    }
                }
                unsafe { (*p_new).n_height = 1 };
            }
            return p_new;
        }
    }
}

///* Allocate a new expression node from a zero-terminated token that has
///* already been dequoted.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr(db: *mut Sqlite3, op: i32, z_token: *const i8)
    -> *mut Expr {
    unsafe {
        let mut x: Token = unsafe { core::mem::zeroed() };
        x.z = z_token;
        x.n = unsafe { sqlite3_strlen30(z_token) } as u32;
        return sqlite3_expr_alloc(db, op, &raw mut x as *const Token, 0);
    }
}

///* Allocate an expression for a 32-bit signed integer literal.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_int32(db: *mut Sqlite3, i_val: i32)
    -> *mut Expr {
    unsafe {
        let p_new: *mut Expr =
            unsafe {
                    sqlite3_db_malloc_raw_nn(db,
                        core::mem::size_of::<Expr>() as u64)
                } as *mut Expr;
        if !(p_new).is_null() {
            unsafe {
                memset(p_new as *mut (), 0,
                    core::mem::size_of::<Expr>() as u64)
            };
            unsafe { (*p_new).op = 156 as u8 };
            unsafe { (*p_new).i_agg = -1 as i16 };
            unsafe {
                (*p_new).flags =
                    (2048 | 8388608 |
                            if i_val != 0 { 268435456 } else { 536870912 }) as u32
            };
            unsafe { (*p_new).u.i_value = i_val };
            unsafe { (*p_new).n_height = 1 };
        }
        return p_new;
    }
}

///* Delete an entire expression list.
extern "C" fn expr_list_delete_nn(db: *mut Sqlite3, p_list_1: *mut ExprList)
    -> () {
    let mut i: i32 = unsafe { (*p_list_1).n_expr };
    let mut p_item: *const ExprListItem =
        unsafe { (*p_list_1).a.as_ptr() } as *const ExprListItem;
    { let _ = 0; };
    { let _ = 0; };
    '__b0: loop {
        '__c0: loop {
            sqlite3_expr_delete(db, unsafe { (*p_item).p_expr });
            if !(unsafe { (*p_item).z_e_name }).is_null() {
                unsafe {
                    sqlite3_db_nn_free_nn(db,
                        unsafe { (*p_item).z_e_name } as *mut ())
                };
            }
            {
                let __p = &mut p_item;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            break '__c0;
        }
        if !({ let __p = &mut i; *__p -= 1; *__p } > 0) { break '__b0; }
    }
    unsafe { sqlite3_db_nn_free_nn(db, p_list_1 as *mut ()) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_delete(db: *mut Sqlite3,
    p_list: *mut ExprList) -> () {
    if !(p_list).is_null() { expr_list_delete_nn(db, p_list); }
}

///* Recursively delete an expression tree.
#[allow(unused_doc_comments)]
extern "C" fn sqlite3_expr_delete_nn(db: *mut Sqlite3, mut p: *mut Expr)
    -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        loop {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if !(unsafe { (*p).flags } & (65536 | 8388608) as u32 != 0 as u32)
                        as i32 != 0 {

                /// The Expr.x union is never used at the same time as Expr.pRight
                { let _ = 0; };
                if !(unsafe { (*p).p_right }).is_null() {
                    { let _ = 0; };
                    sqlite3_expr_delete_nn(db, unsafe { (*p).p_right });
                } else if unsafe { (*p).flags } & 4096 as u32 != 0 as u32 {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_select_delete(db, unsafe { (*p).x.p_select })
                    };
                } else {
                    sqlite3_expr_list_delete(db, unsafe { (*p).x.p_list });
                    if unsafe { (*p).flags } & 16777216 as u32 != 0 as u32 {
                        unsafe {
                            sqlite3_window_delete(db, unsafe { (*p).y.p_win })
                        };
                    }
                }
                if !(unsafe { (*p).p_left }).is_null() &&
                        unsafe { (*p).op } as i32 != 178 {
                    let p_left: *mut Expr = unsafe { (*p).p_left };
                    if !(unsafe { (*p).flags } & 134217728 as u32 != 0 as u32)
                                    as i32 != 0 &&
                            !(unsafe { (*p_left).flags } & 134217728 as u32 != 0 as u32)
                                    as i32 != 0 {

                        /// Avoid unnecessary recursion on unary operators
                        unsafe { sqlite3_db_nn_free_nn(db, p as *mut ()) };
                        p = p_left;
                        continue;
                    } else { sqlite3_expr_delete_nn(db, p_left); }
                }
            }
            break;
        }
        if !(unsafe { (*p).flags } & 134217728 as u32 != 0 as u32) as i32 != 0
            {
            unsafe { sqlite3_db_nn_free_nn(db, p as *mut ()) };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_delete(db: *mut Sqlite3, p: *mut Expr) -> () {
    if !(p).is_null() { sqlite3_expr_delete_nn(db, p); }
}

///* Attach subtrees pLeft and pRight to the Expr node pRoot.
///*
///* If pRoot==NULL that means that a memory allocation error has occurred.
///* In that case, delete the subtrees pLeft and pRight.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_attach_subtrees(db: *mut Sqlite3,
    p_root: *mut Expr, p_left: *mut Expr, p_right: *mut Expr) -> () {
    if p_root == core::ptr::null_mut() {
        { let _ = 0; };
        sqlite3_expr_delete(db, p_left);
        sqlite3_expr_delete(db, p_right);
    } else {
        { let _ = 0; };
        { let _ = 0; };
        if !(p_right).is_null() {
            unsafe { (*p_root).p_right = p_right };
            unsafe {
                (*p_root).flags |=
                    (512 | 4194304 | 8) as u32 & unsafe { (*p_right).flags }
            };
            unsafe {
                (*p_root).n_height = unsafe { (*p_right).n_height } + 1
            };
        } else { unsafe { (*p_root).n_height = 1 }; }
        if !(p_left).is_null() {
            unsafe { (*p_root).p_left = p_left };
            unsafe {
                (*p_root).flags |=
                    (512 | 4194304 | 8) as u32 & unsafe { (*p_left).flags }
            };
            if unsafe { (*p_left).n_height } >= unsafe { (*p_root).n_height }
                {
                unsafe {
                    (*p_root).n_height = unsafe { (*p_left).n_height } + 1
                };
            }
        }
    }
}

///* Check that argument nHeight is less than or equal to the maximum
///* expression depth allowed. If it is not, leave an error message in
///* pParse.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_check_height(p_parse: *mut Parse,
    n_height: i32) -> i32 {
    let mut rc: i32 = 0;
    let mx_height: i32 =
        unsafe { (*unsafe { (*p_parse).db }).a_limit[3 as usize] };
    if n_height > mx_height {
        unsafe {
            sqlite3_error_msg(p_parse,
                c"Expression tree is too large (maximum depth %d)".as_ptr() as
                        *mut i8 as *const i8, mx_height)
        };
        rc = 1;
    }
    return rc;
}

///* Allocate an Expr node which joins as many as two subtrees.
///*
///* One or both of the subtrees can be NULL.  Return a pointer to the new
///* Expr node.  Or, if an OOM error occurs, set pParse->db->mallocFailed,
///* free the subtrees and return NULL.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_p_expr(p_parse: *mut Parse, op: i32,
    p_left: *mut Expr, p_right: *mut Expr) -> *mut Expr {
    let mut p: *mut Expr = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_db_malloc_raw_nn(unsafe { (*p_parse).db },
                    core::mem::size_of::<Expr>() as u64)
            } as *mut Expr;
    if !(p).is_null() {
        unsafe {
            memset(p as *mut (), 0, core::mem::size_of::<Expr>() as u64)
        };
        unsafe { (*p).op = (op & 255) as u8 };
        unsafe { (*p).i_agg = -1 as i16 };
        sqlite3_expr_attach_subtrees(unsafe { (*p_parse).db }, p, p_left,
            p_right);
        sqlite3_expr_check_height(p_parse, unsafe { (*p).n_height });
    } else {
        sqlite3_expr_delete(unsafe { (*p_parse).db }, p_left);
        sqlite3_expr_delete(unsafe { (*p_parse).db }, p_right);
    }
    return p;
}

/// The following three functions, heightOfExpr(), heightOfExprList()
///* and heightOfSelect(), are used to determine the maximum height
///* of any expression tree referenced by the structure passed as the
///* first argument.
///*
///* If this maximum height is greater than the current value pointed
///* to by pnHeight, the second parameter, then set *pnHeight to that
///* value.
extern "C" fn height_of_expr(p: *const Expr, pn_height_1: &mut i32) -> () {
    if !(p).is_null() {
        if unsafe { (*p).n_height } as i32 > *pn_height_1 {
            *pn_height_1 = unsafe { (*p).n_height } as i32;
        }
    }
}

extern "C" fn height_of_expr_list(p: *const ExprList, pn_height_1: *mut i32)
    -> () {
    if !(p).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b2: loop {
                if !(i < unsafe { (*p).n_expr }) { break '__b2; }
                '__c2: loop {
                    height_of_expr(unsafe {
                                (*(unsafe { (*p).a.as_ptr() } as
                                                *const ExprListItem).offset(i as isize)).p_expr
                            } as *const Expr, unsafe { &mut *pn_height_1 });
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

extern "C" fn height_of_select(p_select_1: *const Select,
    pn_height_1: *mut i32) -> () {
    unsafe {
        let mut p: *const Select = core::ptr::null();
        {
            p = p_select_1;
            '__b3: loop {
                if !(!(p).is_null()) { break '__b3; }
                '__c3: loop {
                    height_of_expr(unsafe { (*p).p_where } as *const Expr,
                        unsafe { &mut *pn_height_1 });
                    height_of_expr(unsafe { (*p).p_having } as *const Expr,
                        unsafe { &mut *pn_height_1 });
                    height_of_expr(unsafe { (*p).p_limit } as *const Expr,
                        unsafe { &mut *pn_height_1 });
                    height_of_expr_list(unsafe { (*p).p_e_list } as
                            *const ExprList, pn_height_1);
                    height_of_expr_list(unsafe { (*p).p_group_by } as
                            *const ExprList, pn_height_1);
                    height_of_expr_list(unsafe { (*p).p_order_by } as
                            *const ExprList, pn_height_1);
                    break '__c3;
                }
                p = unsafe { (*p).p_prior } as *const Select;
            }
        }
    }
}

///* Return the bitwise-OR of all Expr.flags fields in the given
///* ExprList.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_flags(p_list: &ExprList) -> u32 {
    let mut i: i32 = 0;
    let mut m: u32 = 0 as u32;
    { let _ = 0; };
    {
        i = 0;
        '__b4: loop {
            if !(i < (*p_list).n_expr) { break '__b4; }
            '__c4: loop {
                let p_expr: *const Expr =
                    unsafe {
                            (*((*p_list).a.as_ptr() as
                                            *const ExprListItem).offset(i as isize)).p_expr
                        } as *const Expr;
                { let _ = 0; };
                m |= unsafe { (*p_expr).flags };
                break '__c4;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return m;
}

///* Set the Expr.nHeight variable in the structure passed as an
///* argument. An expression with no children, Expr.pList or
///* Expr.pSelect member has a height of 1. Any other expression
///* has a height equal to the maximum height of any other
///* referenced Expr plus one.
///*
///* Also propagate EP_Propagate flags up from Expr.x.pList to Expr.flags,
///* if appropriate.
extern "C" fn expr_set_height(p: &mut Expr) -> () {
    unsafe {
        let mut n_height: i32 =
            if !((*p).p_left).is_null() {
                unsafe { (*(*p).p_left).n_height }
            } else { 0 };
        if !((*p).p_right).is_null() &&
                unsafe { (*(*p).p_right).n_height } > n_height {
            n_height = unsafe { (*(*p).p_right).n_height };
        }
        if (*p).flags & 4096 as u32 != 0 as u32 {
            height_of_select((*p).x.p_select as *const Select, &mut n_height);
        } else if !((*p).x.p_list).is_null() {
            height_of_expr_list((*p).x.p_list as *const ExprList,
                &mut n_height);
            (*p).flags |=
                (512 | 4194304 | 8) as u32 &
                    sqlite3_expr_list_flags(unsafe { &*(*p).x.p_list });
        }
        (*p).n_height = n_height + 1;
    }
}

///* Set the Expr.nHeight variable using the exprSetHeight() function. If
///* the height is greater than the maximum allowed expression depth,
///* leave an error in pParse.
///*
///* Also propagate all EP_Propagate flags from the Expr.x.pList into
///* Expr.flags.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_set_height_and_flags(p_parse: *mut Parse,
    p: *mut Expr) -> () {
    if unsafe { (*p_parse).n_err } != 0 { return; }
    expr_set_height(unsafe { &mut *p });
    sqlite3_expr_check_height(p_parse, unsafe { (*p).n_height });
}

///* Add pSelect to the Expr.x.pSelect field.  Or, if pExpr is NULL (due
///* do a memory allocation failure) then delete the pSelect object.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_p_expr_add_select(p_parse: *mut Parse,
    p_expr: *mut Expr, p_select: *mut Select) -> () {
    unsafe {
        if !(p_expr).is_null() {
            unsafe { (*p_expr).x.p_select = p_select };
            unsafe { (*p_expr).flags |= (4096 | 4194304) as u32 };
            sqlite3_expr_set_height_and_flags(p_parse, p_expr);
        } else {
            { let _ = 0; };
            unsafe {
                sqlite3_select_delete(unsafe { (*p_parse).db }, p_select)
            };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_delete_generic(db: *mut Sqlite3, p: *mut ())
    -> () {
    if !(p).is_null() { sqlite3_expr_delete_nn(db, p as *mut Expr); }
}

///* Arrange to cause pExpr to be deleted when the pParse is deleted.
///* This is similar to sqlite3ExprDelete() except that the delete is
///* deferred until the pParse is deleted.
///*
///* The pExpr might be deleted immediately on an OOM error.
///*
///* Return 0 if the delete was successfully deferred.  Return non-zero
///* if the delete happened immediately because of an OOM.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_deferred_delete(p_parse: *mut Parse,
    p_expr: *mut Expr) -> i32 {
    return (core::ptr::null_mut() ==
                unsafe {
                    sqlite3_parser_add_cleanup(p_parse,
                        Some(sqlite3_expr_delete_generic), p_expr as *mut ())
                }) as i32;
}

///* Join two expressions using an AND operator.  If either expression is
///* NULL, then just return the other expression.
///*
///* If one side or the other of the AND is known to be false, and neither side
///* is part of an ON clause, then instead of returning an AND expression,
///* just return a constant expression with a value of false.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_and(p_parse: *mut Parse, p_left: *mut Expr,
    p_right: *mut Expr) -> *mut Expr {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    if p_left == core::ptr::null_mut() {
        return p_right;
    } else if p_right == core::ptr::null_mut() {
        return p_left;
    } else {
        let f: u32 = unsafe { (*p_left).flags } | unsafe { (*p_right).flags };
        if f & (1 | 2 | 536870912 | 8) as u32 == 536870912 as u32 &&
                !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 != 0
            {
            sqlite3_expr_deferred_delete(p_parse, p_left);
            sqlite3_expr_deferred_delete(p_parse, p_right);
            return sqlite3_expr_int32(db, 0);
        } else { return sqlite3_p_expr(p_parse, 44, p_left, p_right); }
    }
}

///* If pExpr is an AND or OR expression, try to simplify it by eliminating
///* terms that are always true or false.  Return the simplified expression.
///* Or return the original expression if no simplification is possible.
///*
///* Examples:
///*
///*     (x<10) AND true                =>   (x<10)
///*     (x<10) AND false               =>   false
///*     (x<10) AND (y=22 OR false)     =>   (x<10) AND (y=22)
///*     (x<10) AND (y=22 OR true)      =>   (x<10)
///*     (y=22) OR true                 =>   true
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_simplified_and_or(mut p_expr: *mut Expr)
    -> *mut Expr {
    { let _ = 0; };
    if unsafe { (*p_expr).op } as i32 == 44 ||
            unsafe { (*p_expr).op } as i32 == 43 {
        let p_right: *mut Expr =
            sqlite3_expr_simplified_and_or(unsafe { (*p_expr).p_right });
        let p_left: *mut Expr =
            sqlite3_expr_simplified_and_or(unsafe { (*p_expr).p_left });
        if unsafe { (*p_left).flags } & (1 | 268435456) as u32 ==
                    268435456 as u32 ||
                unsafe { (*p_right).flags } & (1 | 536870912) as u32 ==
                    536870912 as u32 {
            p_expr =
                if unsafe { (*p_expr).op } as i32 == 44 {
                    p_right
                } else { p_left };
        } else if unsafe { (*p_right).flags } & (1 | 268435456) as u32 ==
                    268435456 as u32 ||
                unsafe { (*p_left).flags } & (1 | 536870912) as u32 ==
                    536870912 as u32 {
            p_expr =
                if unsafe { (*p_expr).op } as i32 == 44 {
                    p_left
                } else { p_right };
        }
    }
    return p_expr;
}

///* Construct a new expression node for a function with multiple
///* arguments.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_function(p_parse: *mut Parse,
    p_list: *mut ExprList, p_token: *const Token, e_distinct: i32)
    -> *mut Expr {
    unsafe {
        let mut p_new: *mut Expr = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        { let _ = 0; };
        p_new = sqlite3_expr_alloc(db, 172, p_token, 1);
        if p_new == core::ptr::null_mut() {
            sqlite3_expr_list_delete(db, p_list);

            /// Avoid memory leak when malloc fails
            return core::ptr::null_mut();
        }
        { let _ = 0; };
        unsafe {
            (*p_new).w.i_ofst =
                unsafe {
                            unsafe {
                                (*p_token).z.offset_from(unsafe { (*p_parse).z_tail })
                            }
                        } as i64 as i32
        };
        if !(p_list).is_null() &&
                    unsafe { (*p_list).n_expr } >
                        unsafe { (*unsafe { (*p_parse).db }).a_limit[6 as usize] }
                && (unsafe { (*p_parse).nested } == 0) as i32 != 0 {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"too many arguments on function %T".as_ptr() as *mut i8 as
                        *const i8, p_token)
            };
        }
        unsafe { (*p_new).x.p_list = p_list };
        unsafe { (*p_new).flags |= 8 as u32 };
        { let _ = 0; };
        sqlite3_expr_set_height_and_flags(p_parse, p_new);
        if e_distinct == 1 { unsafe { (*p_new).flags |= 4 as u32 }; }
        return p_new;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_delete_generic(db: *mut Sqlite3,
    p_list: *mut ()) -> () {
    if !(p_list).is_null() {
        expr_list_delete_nn(db, p_list as *mut ExprList);
    }
}

///* Report an error when attempting to use an ORDER BY clause within
///* the arguments of a non-aggregate function.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_order_by_aggregate_error(p_parse: *mut Parse,
    p: *mut Expr) -> () {
    unsafe {
        sqlite3_error_msg(p_parse,
            c"ORDER BY may not be used with non-aggregate %#T()".as_ptr() as
                    *mut i8 as *const i8, p)
    };
}

///* Attach an ORDER BY clause to a function call.
///*
///*     functionname( arguments ORDER BY sortlist )
///*     \_____________________/          \______/
///*             pExpr                    pOrderBy
///*
///* The ORDER BY clause is inserted into a new Expr node of type TK_ORDER
///* and added to the Expr.pLeft field of the parent TK_FUNCTION node.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_add_function_order_by(p_parse: *mut Parse,
    p_expr: *mut Expr, p_order_by: *mut ExprList) -> () {
    unsafe {
        let mut p_ob: *mut Expr = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        if p_order_by == core::ptr::null_mut() { { let _ = 0; }; return; }
        if p_expr == core::ptr::null_mut() {
            { let _ = 0; };
            sqlite3_expr_list_delete(db, p_order_by);
            return;
        }
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_expr).x.p_list } == core::ptr::null_mut() ||
                unsafe { (*unsafe { (*p_expr).x.p_list }).n_expr } == 0 {

            /// Ignore ORDER BY on zero-argument aggregates
            unsafe {
                sqlite3_parser_add_cleanup(p_parse,
                    Some(sqlite3_expr_list_delete_generic),
                    p_order_by as *mut ())
            };
            return;
        }
        if unsafe { (*p_expr).flags } & 16777216 as u32 != 0 as u32 &&
                unsafe { (*unsafe { (*p_expr).y.p_win }).e_frm_type } as i32
                    != 167 {
            sqlite3_expr_order_by_aggregate_error(p_parse, p_expr);
            sqlite3_expr_list_delete(db, p_order_by);
            return;
        }
        if unsafe { (*p_order_by).n_expr } >
                unsafe { (*db).a_limit[2 as usize] } {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"too many terms in ORDER BY clause".as_ptr() as *mut i8 as
                        *const i8)
            };
            sqlite3_expr_list_delete(db, p_order_by);
            return;
        }
        p_ob = sqlite3_expr_alloc(db, 146, core::ptr::null(), 0);
        if p_ob == core::ptr::null_mut() {
            sqlite3_expr_list_delete(db, p_order_by);
            return;
        }
        unsafe { (*p_ob).x.p_list = p_order_by };
        { let _ = 0; };
        unsafe { (*p_expr).p_left = p_ob };
        unsafe { (*p_ob).flags |= 131072 as u32 };
    }
}

///* Check to see if a function is usable according to current access
///* rules:
///*
///*    SQLITE_FUNC_DIRECT    -     Only usable from top-level SQL
///*
///*    SQLITE_FUNC_UNSAFE    -     Usable if TRUSTED_SCHEMA or from
///*                                top-level SQL
///*
///* If the function is not usable, create an error.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_function_usable(p_parse: *mut Parse,
    p_expr: *const Expr, p_def: &FuncDef) -> () {
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_expr).flags } & 1073741824 as u32 != 0 as u32 ||
            unsafe { (*p_parse).prep_flags } as i32 & 32 != 0 {
        if (*p_def).func_flags & 524288 as u32 != 0 as u32 ||
                unsafe { (*unsafe { (*p_parse).db }).flags } & 128 as u64 ==
                    0 as u64 {

            /// Functions prohibited in triggers and views if:
            ///*     (1) tagged with SQLITE_DIRECTONLY
            ///*     (2) not tagged with SQLITE_INNOCUOUS (which means it
            ///*         is tagged with SQLITE_FUNC_UNSAFE) and
            ///*         SQLITE_DBCONFIG_TRUSTED_SCHEMA is off (meaning
            ///*         that the schema is possibly tainted).
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"unsafe use of %#T()".as_ptr() as *mut i8 as *const i8,
                    p_expr)
            };
        }
    }
}

///* Assign a variable number to an expression that encodes a wildcard
///* in the original SQL statement. 
///*
///* Wildcards consisting of a single "?" are assigned the next sequential
///* variable number.
///*
///* Wildcards of the form "?nnn" are assigned the number "nnn".  We make
///* sure "nnn" is not too big to avoid a denial of service attack when
///* the SQL statement comes from an external source.
///*
///* Wildcards of the form ":aaa", "@aaa", or "$aaa" are assigned the same number
///* as the previous instance of the same wildcard.  Or if this is the first
///* instance of the wildcard, the next sequential variable number is
///* assigned.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_assign_var_number(p_parse: *mut Parse,
    p_expr: *mut Expr, n: u32) -> () {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        let mut z: *const i8 = core::ptr::null();
        let mut x: YnVar = 0 as YnVar;
        if p_expr == core::ptr::null_mut() { return; }
        { let _ = 0; };
        z = unsafe { (*p_expr).u.z_token } as *const i8;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { *z.offset(1 as isize) } as i32 == 0 {

            /// Wildcard of the form "?".  Assign the next variable number
            { let _ = 0; };
            x =
                {
                        let __p = unsafe { &mut (*p_parse).n_var };
                        *__p += 1;
                        *__p
                    } as YnVar;
        } else {
            let mut do_add: i32 = 0;
            if unsafe { *z.offset(0 as isize) } as i32 == '?' as i32 {
                /// Wildcard of the form "?nnn".  Convert "nnn" to an integer and
                ///* use it as the variable number
                let mut i: i64 = 0 as i64;
                let mut b_ok: i32 = 0;
                if n == 2 as u32 {

                    ///OPTIMIZATION-IF-TRUE
                    (i =
                        (unsafe { *z.offset(1 as isize) } as i32 - '0' as i32) as
                            i64);

                    /// The common case of ?N for a single digit N
                    (b_ok = 1);
                } else {
                    b_ok =
                        (0 ==
                                unsafe {
                                    sqlite3_atoi64(unsafe { &*z.offset(1 as isize) }, &mut i,
                                        (n - 1 as u32) as i32, 1 as u8)
                                }) as i32;
                }
                if b_ok == 0 || i < 1 as i64 ||
                        i > unsafe { (*db).a_limit[9 as usize] } as i64 {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"variable number must be between ?1 and ?%d".as_ptr() as
                                    *mut i8 as *const i8, unsafe { (*db).a_limit[9 as usize] })
                    };
                    unsafe {
                        sqlite3_record_error_offset_of_expr(unsafe {
                                (*p_parse).db
                            }, p_expr as *const Expr)
                    };
                    return;
                }
                x = i as YnVar;
                if x as i32 > unsafe { (*p_parse).n_var } as i32 {
                    unsafe { (*p_parse).n_var = x as i32 as YnVar };
                    do_add = 1;
                } else if unsafe {
                            sqlite3_v_list_num_to_name(unsafe { (*p_parse).p_v_list },
                                x as i32)
                        } == core::ptr::null() {
                    do_add = 1;
                }
            } else {

                /// Wildcards like ":aaa", "$aaa" or "@aaa".  Reuse the same variable
                ///* number as the prior appearance of the same name, or if the name
                ///* has never appeared before, reuse the same variable number
                (x =
                    unsafe {
                            sqlite3_v_list_name_to_num(unsafe { (*p_parse).p_v_list },
                                z, n as i32)
                        } as YnVar);
                if x as i32 == 0 {
                    x =
                        {
                                let __p = unsafe { &mut (*p_parse).n_var };
                                *__p += 1;
                                *__p
                            } as YnVar;
                    do_add = 1;
                }
            }
            if do_add != 0 {
                unsafe {
                    (*p_parse).p_v_list =
                        unsafe {
                            sqlite3_v_list_add(db, unsafe { (*p_parse).p_v_list }, z,
                                n as i32, x as i32)
                        }
                };
            }
        }
        unsafe { (*p_expr).i_column = x };
        if x as i32 > unsafe { (*db).a_limit[9 as usize] } {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"too many SQL variables".as_ptr() as *mut i8 as *const i8)
            };
            unsafe {
                sqlite3_record_error_offset_of_expr(unsafe { (*p_parse).db },
                    p_expr as *const Expr)
            };
        }
    }
}

/// Invoke sqlite3RenameExprUnmap() and sqlite3ExprDelete() on the
///* expression.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_unmap_and_delete(p_parse: *mut Parse,
    p: *mut Expr) -> () {
    if !(p).is_null() {
        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
            unsafe { sqlite3_rename_expr_unmap(p_parse, p) };
        }
        sqlite3_expr_delete_nn(unsafe { (*p_parse).db }, p);
    }
}

///* Add a new element to the end of an expression list.  If pList is
///* initially NULL, then create a new expression list.
///*
///* The pList argument must be either NULL or a pointer to an ExprList
///* obtained from a prior call to sqlite3ExprListAppend().
///*
///* If a memory allocation error occurs, the entire list is freed and
///* NULL is returned.  If non-NULL is returned, then it is guaranteed
///* that the new entry was successfully appended.
static mut zero_item: ExprListItem =
    ExprListItem {
        p_expr: core::ptr::null_mut(),
        z_e_name: core::ptr::null_mut(),
        fg: unsafe { core::mem::zeroed() },
        u: unsafe { core::mem::zeroed() },
    };

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_append_new(db: *mut Sqlite3,
    p_expr_1: *mut Expr) -> *mut ExprList {
    unsafe {
        let mut p_item: *mut ExprListItem = core::ptr::null_mut();
        let mut p_list: *mut ExprList = core::ptr::null_mut();
        p_list =
            unsafe {
                    sqlite3_db_malloc_raw_nn(db,
                        core::mem::offset_of!(ExprList, a) as u64 +
                            4 as u64 * core::mem::size_of::<ExprListItem>() as u64)
                } as *mut ExprList;
        if p_list == core::ptr::null_mut() {
            sqlite3_expr_delete(db, p_expr_1);
            return core::ptr::null_mut();
        }
        unsafe { (*p_list).n_alloc = 4 };
        unsafe { (*p_list).n_expr = 1 };
        p_item =
            unsafe {
                &mut *(unsafe { (*p_list).a.as_ptr() } as
                                *mut ExprListItem).offset(0 as isize)
            };
        unsafe { *p_item = zero_item };
        unsafe { (*p_item).p_expr = p_expr_1 };
        return p_list;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_append_grow(db: *mut Sqlite3,
    mut p_list_1: *mut ExprList, p_expr_1: *mut Expr) -> *mut ExprList {
    unsafe {
        let mut p_item: *mut ExprListItem = core::ptr::null_mut();
        let mut p_new: *mut ExprList = core::ptr::null_mut();
        unsafe { (*p_list_1).n_alloc *= 2 };
        p_new =
            unsafe {
                    sqlite3_db_realloc(db, p_list_1 as *mut (),
                        core::mem::offset_of!(ExprList, a) as u64 +
                            unsafe { (*p_list_1).n_alloc } as u64 *
                                core::mem::size_of::<ExprListItem>() as u64)
                } as *mut ExprList;
        if p_new == core::ptr::null_mut() {
            sqlite3_expr_list_delete(db, p_list_1);
            sqlite3_expr_delete(db, p_expr_1);
            return core::ptr::null_mut();
        } else { p_list_1 = p_new; }
        p_item =
            unsafe {
                &mut *(unsafe { (*p_list_1).a.as_ptr() } as
                                *mut ExprListItem).offset({
                                    let __p = unsafe { &mut (*p_list_1).n_expr };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
            };
        unsafe { *p_item = zero_item };
        unsafe { (*p_item).p_expr = p_expr_1 };
        return p_list_1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_append(p_parse: &Parse,
    p_list: *mut ExprList, p_expr: *mut Expr) -> *mut ExprList {
    unsafe {
        let mut p_item: *mut ExprListItem = core::ptr::null_mut();
        if p_list == core::ptr::null_mut() {
            return sqlite3_expr_list_append_new((*p_parse).db, p_expr);
        }
        if unsafe { (*p_list).n_alloc } < unsafe { (*p_list).n_expr } + 1 {
            return sqlite3_expr_list_append_grow((*p_parse).db, p_list,
                    p_expr);
        }
        p_item =
            unsafe {
                &mut *(unsafe { (*p_list).a.as_ptr() } as
                                *mut ExprListItem).offset({
                                    let __p = unsafe { &mut (*p_list).n_expr };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as isize)
            };
        unsafe { *p_item = zero_item };
        unsafe { (*p_item).p_expr = p_expr };
        return p_list;
    }
}

///* If the expression passed as the only argument is of type TK_VECTOR
///* return the number of expressions in the vector. Or, if the expression
///* is a sub-select, return the number of columns in the sub-select. For
///* any other type of expression, return 1.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_vector_size(p_expr: &Expr) -> i32 {
    unsafe {
        let mut op: u8 = (*p_expr).op as u8;
        if op as i32 == 176 { op = (*p_expr).op2 as u8; }
        if op as i32 == 177 {
            { let _ = 0; };
            return unsafe { (*(*p_expr).x.p_list).n_expr };
        } else if op as i32 == 139 {
            { let _ = 0; };
            return unsafe {
                    (*unsafe { (*(*p_expr).x.p_select).p_e_list }).n_expr
                };
        } else { return 1; }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct EdupBuf {
    z_alloc: *mut u8,
}

///* The dupedExpr*Size() routines each return the number of bytes required
///* to store a copy of an expression or expression tree.  They differ in
///* how much of the tree is measured.
///*
///*     dupedExprStructSize()     Size of only the Expr structure
///*     dupedExprNodeSize()       Size of Expr + space for token
///*     dupedExprSize()           Expr + token + subtree components
///*
///**************************************************************************
///*
///* The dupedExprStructSize() function returns two values OR-ed together: 
///* (1) the space required for a copy of the Expr structure only and
///* (2) the EP_xxx flags that indicate what the structure size should be.
///* The return values is always one of:
///*
///*      EXPR_FULLSIZE
///*      EXPR_REDUCEDSIZE   | EP_Reduced
///*      EXPR_TOKENONLYSIZE | EP_TokenOnly
///*
///* The size of the structure can be found by masking the return value
///* of this routine with 0xfff.  The flags can be found by masking the
///* return value with EP_Reduced|EP_TokenOnly.
///*
///* Note that with flags==EXPRDUP_REDUCE, this routines works on full-size
///* (unreduced) Expr objects as they or originally constructed by the parser.
///* During expression analysis, extra information is computed and moved into
///* later parts of the Expr object and that extra information might get chopped
///* off if the expression is reduced.  Note also that it does not work to
///* make an EXPRDUP_REDUCE copy of a reduced expression.  It is only legal
///* to reduce a pristine expression tree from the parser.  The implementation
///* of dupedExprStructSize() contain multiple assert() statements that attempt
///* to enforce this constraint.
#[allow(unused_doc_comments)]
extern "C" fn duped_expr_struct_size(p: &Expr, flags: i32) -> i32 {
    unsafe {
        let mut n_size: i32 = 0;
        { let _ = 0; };

        /// Only one flag value allowed
        { let _ = 0; };
        { let _ = 0; };
        if 0 == flags || (*p).flags & 131072 as u32 != 0 as u32 {
            n_size = core::mem::size_of::<Expr>() as i32;
        } else {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if !((*p).p_left).is_null() || !((*p).x.p_list).is_null() {
                n_size =
                    (core::mem::offset_of!(Expr, i_table) as u64 | 16384 as u64)
                        as i32;
            } else {
                { let _ = 0; };
                n_size =
                    (core::mem::offset_of!(Expr, p_left) as u64 | 65536 as u64)
                        as i32;
            }
        }
        return n_size;
    }
}

///* This function returns the space in bytes required to store the copy
///* of the Expr structure and a copy of the Expr.u.zToken string (if that
///* string is defined.)
extern "C" fn duped_expr_node_size(p: *const Expr, flags: i32) -> i32 {
    unsafe {
        let mut n_byte: i32 =
            duped_expr_struct_size(unsafe { &*p }, flags) & 4095;
        if !(unsafe { (*p).flags } & 2048 as u32 != 0 as u32) as i32 != 0 &&
                !(unsafe { (*p).u.z_token }).is_null() {
            n_byte +=
                ((unsafe { strlen(unsafe { (*p).u.z_token } as *const i8) } &
                            1073741823 as u64) + 1 as u64) as i32;
        }
        return n_byte + 7 & !7;
    }
}

///* Return the number of bytes required to create a duplicate of the
///* expression passed as the first argument.
///*
///* The value returned includes space to create a copy of the Expr struct
///* itself and the buffer referred to by Expr.u.zToken, if any.
///*
///* The return value includes space to duplicate all Expr nodes in the
///* tree formed by Expr.pLeft and Expr.pRight, but not any other
///* substructure such as Expr.x.pList, Expr.x.pSelect, and Expr.y.pWin.
extern "C" fn duped_expr_size(p: *const Expr) -> i32 {
    let mut n_byte: i32 = 0;
    { let _ = 0; };
    n_byte = duped_expr_node_size(p, 1);
    if !(unsafe { (*p).p_left }).is_null() {
        n_byte += duped_expr_size(unsafe { (*p).p_left } as *const Expr);
    }
    if !(unsafe { (*p).p_right }).is_null() {
        n_byte += duped_expr_size(unsafe { (*p).p_right } as *const Expr);
    }
    { let _ = 0; };
    return n_byte;
}

///* Return the number of bytes allocated for the expression structure
///* passed as the first argument. This is always one of EXPR_FULLSIZE,
///* EXPR_REDUCEDSIZE or EXPR_TOKENONLYSIZE.
extern "C" fn expr_struct_size(p: &Expr) -> i32 {
    if (*p).flags & 65536 as u32 != 0 as u32 {
        return core::mem::offset_of!(Expr, p_left) as i32;
    }
    if (*p).flags & 16384 as u32 != 0 as u32 {
        return core::mem::offset_of!(Expr, i_table) as i32;
    }
    return core::mem::size_of::<Expr>() as i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_dup(db: *mut Sqlite3, p: *const ExprList,
    flags: i32) -> *mut ExprList {
    let mut p_new: *mut ExprList = core::ptr::null_mut();
    let mut p_item: *mut ExprListItem = core::ptr::null_mut();
    let mut p_old_item: *const ExprListItem = core::ptr::null();
    let mut i: i32 = 0;
    let mut p_prior_select_col_old: *mut Expr = core::ptr::null_mut();
    let mut p_prior_select_col_new: *mut Expr = core::ptr::null_mut();
    { let _ = 0; };
    if p == core::ptr::null() { return core::ptr::null_mut(); }
    p_new =
        unsafe {
                sqlite3_db_malloc_raw_nn(db,
                    unsafe { sqlite3_db_malloc_size(db, p as *const ()) } as
                        u64)
            } as *mut ExprList;
    if p_new == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe { (*p_new).n_expr = unsafe { (*p).n_expr } as i32 };
    unsafe { (*p_new).n_alloc = unsafe { (*p).n_alloc } as i32 };
    p_item = unsafe { (*p_new).a.as_ptr() } as *mut ExprListItem;
    p_old_item = unsafe { (*p).a.as_ptr() } as *const ExprListItem;
    {
        i = 0;
        '__b5: loop {
            if !(i < unsafe { (*p).n_expr }) { break '__b5; }
            '__c5: loop {
                let p_old_expr: *const Expr =
                    unsafe { (*p_old_item).p_expr } as *const Expr;
                let mut p_new_expr: *mut Expr = core::ptr::null_mut();
                unsafe {
                    (*p_item).p_expr =
                        sqlite3_expr_dup(db, p_old_expr as *const Expr, flags)
                };
                if !(p_old_expr).is_null() &&
                            unsafe { (*p_old_expr).op } as i32 == 178 &&
                        { p_new_expr = unsafe { (*p_item).p_expr }; p_new_expr } !=
                            core::ptr::null_mut() {
                    if !(unsafe { (*p_new_expr).p_right }).is_null() {
                        p_prior_select_col_old = unsafe { (*p_old_expr).p_right };
                        p_prior_select_col_new = unsafe { (*p_new_expr).p_right };
                        unsafe {
                            (*p_new_expr).p_left = unsafe { (*p_new_expr).p_right }
                        };
                    } else {
                        if unsafe { (*p_old_expr).p_left } != p_prior_select_col_old
                            {
                            p_prior_select_col_old = unsafe { (*p_old_expr).p_left };
                            p_prior_select_col_new =
                                sqlite3_expr_dup(db, p_prior_select_col_old as *const Expr,
                                    flags);
                            unsafe { (*p_new_expr).p_right = p_prior_select_col_new };
                        }
                        unsafe { (*p_new_expr).p_left = p_prior_select_col_new };
                    }
                }
                unsafe {
                    (*p_item).z_e_name =
                        unsafe {
                            sqlite3_db_str_dup(db,
                                unsafe { (*p_old_item).z_e_name } as *const i8)
                        }
                };
                unsafe { (*p_item).fg = unsafe { (*p_old_item).fg } };
                unsafe { (*p_item).u = unsafe { (*p_old_item).u } };
                break '__c5;
            }
            {
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_item;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
                {
                    let __p = &mut p_old_item;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                }
            };
        }
    }
    return p_new;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_id_list_dup(db: *mut Sqlite3, p: *const IdList)
    -> *mut IdList {
    let mut p_new: *mut IdList = core::ptr::null_mut();
    let mut i: i32 = 0;
    { let _ = 0; };
    if p == core::ptr::null() { return core::ptr::null_mut(); }
    p_new =
        unsafe {
                sqlite3_db_malloc_raw_nn(db,
                    core::mem::offset_of!(IdList, a) as u64 +
                        unsafe { (*p).n_id } as u64 *
                            core::mem::size_of::<IdListItem>() as u64)
            } as *mut IdList;
    if p_new == core::ptr::null_mut() { return core::ptr::null_mut(); }
    unsafe { (*p_new).n_id = unsafe { (*p).n_id } as i32 };
    {
        i = 0;
        '__b6: loop {
            if !(i < unsafe { (*p).n_id }) { break '__b6; }
            '__c6: loop {
                let p_new_item: *mut IdListItem =
                    unsafe {
                        &mut *(unsafe { (*p_new).a.as_ptr() } as
                                        *mut IdListItem).offset(i as isize)
                    };
                let p_old_item: *const IdListItem =
                    unsafe {
                        &*(unsafe { (*p).a.as_ptr() } as
                                        *const IdListItem).offset(i as isize)
                    };
                unsafe {
                    (*p_new_item).z_name =
                        unsafe {
                            sqlite3_db_str_dup(db,
                                unsafe { (*p_old_item).z_name } as *const i8)
                        }
                };
                break '__c6;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return p_new;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_list_dup(db: *mut Sqlite3, p: *const SrcList,
    flags: i32) -> *mut SrcList {
    unsafe {
        let mut p_new: *mut SrcList = core::ptr::null_mut();
        let mut i: i32 = 0;
        { let _ = 0; };
        if p == core::ptr::null() { return core::ptr::null_mut(); }
        p_new =
            unsafe {
                    sqlite3_db_malloc_raw_nn(db,
                        core::mem::offset_of!(SrcList, a) as u64 +
                            unsafe { (*p).n_src } as u64 *
                                core::mem::size_of::<SrcItem>() as u64)
                } as *mut SrcList;
        if p_new == core::ptr::null_mut() { return core::ptr::null_mut(); }
        unsafe {
            (*p_new).n_src =
                {
                        let __v = unsafe { (*p).n_src } as u32;
                        unsafe { (*p_new).n_alloc = __v };
                        __v
                    } as i32
        };
        {
            i = 0;
            '__b7: loop {
                if !(i < unsafe { (*p).n_src }) { break '__b7; }
                '__c7: loop {
                    let p_new_item: *mut SrcItem =
                        unsafe {
                            &mut *(unsafe { (*p_new).a.as_ptr() } as
                                            *mut SrcItem).offset(i as isize)
                        };
                    let p_old_item: *const SrcItem =
                        unsafe {
                            &*(unsafe { (*p).a.as_ptr() } as
                                            *const SrcItem).offset(i as isize)
                        };
                    let mut p_tab: *mut Table = core::ptr::null_mut();
                    unsafe { (*p_new_item).fg = unsafe { (*p_old_item).fg } };
                    if unsafe { (*p_old_item).fg.is_subquery() } != 0 {
                        let mut p_new_subq: *mut Subquery =
                            unsafe {
                                    sqlite3_db_malloc_raw(db,
                                        core::mem::size_of::<Subquery>() as u64)
                                } as *mut Subquery;
                        if p_new_subq == core::ptr::null_mut() {
                            { let _ = 0; };
                            unsafe {
                                (*p_new_item).fg.set_is_subquery(0 as u32 as u32)
                            };
                        } else {
                            unsafe {
                                memcpy(p_new_subq as *mut (),
                                    unsafe { (*p_old_item).u4.p_subq } as *const (),
                                    core::mem::size_of::<Subquery>() as u64)
                            };
                            unsafe {
                                (*p_new_subq).p_select =
                                    sqlite3_select_dup(db,
                                        unsafe { (*p_new_subq).p_select } as *const Select, flags)
                            };
                            if unsafe { (*p_new_subq).p_select } ==
                                    core::ptr::null_mut() {
                                unsafe { sqlite3_db_free(db, p_new_subq as *mut ()) };
                                p_new_subq = core::ptr::null_mut();
                                unsafe {
                                    (*p_new_item).fg.set_is_subquery(0 as u32 as u32)
                                };
                            }
                        }
                        unsafe { (*p_new_item).u4.p_subq = p_new_subq };
                    } else if unsafe { (*p_old_item).fg.fixed_schema() } != 0 {
                        unsafe {
                            (*p_new_item).u4.p_schema =
                                unsafe { (*p_old_item).u4.p_schema }
                        };
                    } else {
                        unsafe {
                            (*p_new_item).u4.z_database =
                                unsafe {
                                    sqlite3_db_str_dup(db,
                                        unsafe { (*p_old_item).u4.z_database } as *const i8)
                                }
                        };
                    }
                    unsafe {
                        (*p_new_item).z_name =
                            unsafe {
                                sqlite3_db_str_dup(db,
                                    unsafe { (*p_old_item).z_name } as *const i8)
                            }
                    };
                    unsafe {
                        (*p_new_item).z_alias =
                            unsafe {
                                sqlite3_db_str_dup(db,
                                    unsafe { (*p_old_item).z_alias } as *const i8)
                            }
                    };
                    unsafe {
                        (*p_new_item).i_cursor =
                            unsafe { (*p_old_item).i_cursor } as i32
                    };
                    if unsafe { (*p_new_item).fg.is_indexed_by() } != 0 {
                        unsafe {
                            (*p_new_item).u1.z_indexed_by =
                                unsafe {
                                    sqlite3_db_str_dup(db,
                                        unsafe { (*p_old_item).u1.z_indexed_by } as *const i8)
                                }
                        };
                    } else if unsafe { (*p_new_item).fg.is_tab_func() } != 0 {
                        unsafe {
                            (*p_new_item).u1.p_func_arg =
                                sqlite3_expr_list_dup(db,
                                    unsafe { (*p_old_item).u1.p_func_arg } as *const ExprList,
                                    flags)
                        };
                    } else {
                        unsafe {
                            (*p_new_item).u1.n_row =
                                unsafe { (*p_old_item).u1.n_row } as u32
                        };
                    }
                    unsafe { (*p_new_item).u2 = unsafe { (*p_old_item).u2 } };
                    if unsafe { (*p_new_item).fg.is_cte() } != 0 {
                        {
                            let __p =
                                unsafe {
                                    &mut (*unsafe { (*p_new_item).u2.p_cte_use }).n_use
                                };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                    p_tab =
                        {
                            unsafe {
                                (*p_new_item).p_s_tab = unsafe { (*p_old_item).p_s_tab }
                            };
                            unsafe { (*p_new_item).p_s_tab }
                        };
                    if !(p_tab).is_null() {
                        {
                            let __p = unsafe { &mut (*p_tab).n_tab_ref };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                    }
                    if unsafe { (*p_old_item).fg.is_using() } != 0 {
                        { let _ = 0; };
                        unsafe {
                            (*p_new_item).u3.p_using =
                                sqlite3_id_list_dup(db,
                                    unsafe { (*p_old_item).u3.p_using } as *const IdList)
                        };
                    } else {
                        unsafe {
                            (*p_new_item).u3.p_on =
                                sqlite3_expr_dup(db,
                                    unsafe { (*p_old_item).u3.p_on } as *const Expr, flags)
                        };
                    }
                    unsafe {
                        (*p_new_item).col_used =
                            unsafe { (*p_old_item).col_used } as Bitmask
                    };
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return p_new;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_with_dup(db: *mut Sqlite3, p: *mut With)
    -> *mut With {
    unsafe {
        let mut p_ret: *mut With = core::ptr::null_mut();
        if !(p).is_null() {
            let n_byte: Sqlite3Int64 =
                (core::mem::offset_of!(With, a) as u64 +
                        unsafe { (*p).n_cte } as u64 *
                            core::mem::size_of::<Cte>() as u64) as Sqlite3Int64;
            p_ret =
                unsafe { sqlite3_db_malloc_zero(db, n_byte as u64) } as
                    *mut With;
            if !(p_ret).is_null() {
                let mut i: i32 = 0;
                unsafe { (*p_ret).n_cte = unsafe { (*p).n_cte } };
                {
                    i = 0;
                    '__b8: loop {
                        if !(i < unsafe { (*p).n_cte }) { break '__b8; }
                        '__c8: loop {
                            unsafe {
                                (*(unsafe { (*p_ret).a.as_ptr() } as
                                                    *mut Cte).offset(i as isize)).p_select =
                                    sqlite3_select_dup(db,
                                        unsafe {
                                                (*(unsafe { (*p).a.as_ptr() } as
                                                                *mut Cte).offset(i as isize)).p_select
                                            } as *const Select, 0)
                            };
                            unsafe {
                                (*(unsafe { (*p_ret).a.as_ptr() } as
                                                    *mut Cte).offset(i as isize)).p_cols =
                                    sqlite3_expr_list_dup(db,
                                        unsafe {
                                                (*(unsafe { (*p).a.as_ptr() } as
                                                                *mut Cte).offset(i as isize)).p_cols
                                            } as *const ExprList, 0)
                            };
                            unsafe {
                                (*(unsafe { (*p_ret).a.as_ptr() } as
                                                    *mut Cte).offset(i as isize)).z_name =
                                    unsafe {
                                        sqlite3_db_str_dup(db,
                                            unsafe {
                                                    (*(unsafe { (*p).a.as_ptr() } as
                                                                    *mut Cte).offset(i as isize)).z_name
                                                } as *const i8)
                                    }
                            };
                            unsafe {
                                (*(unsafe { (*p_ret).a.as_ptr() } as
                                                    *mut Cte).offset(i as isize)).e_m10d =
                                    unsafe {
                                        (*(unsafe { (*p).a.as_ptr() } as
                                                        *mut Cte).offset(i as isize)).e_m10d
                                    }
                            };
                            break '__c8;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        }
        return p_ret;
    }
}

///* The gatherSelectWindows() procedure and its helper routine
///* gatherSelectWindowsCallback() are used to scan all the expressions
///* an a newly duplicated SELECT statement and gather all of the Window
///* objects found there, assembling them onto the linked list at Select->pWin.
extern "C" fn gather_select_windows_callback(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 172 &&
                unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
            let p_select: *mut Select = unsafe { (*p_walker_1).u.p_select };
            let p_win: *mut Window = unsafe { (*p_expr_1).y.p_win };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            unsafe { sqlite3_window_link(p_select, p_win) };
        }
        return 0;
    }
}

extern "C" fn gather_select_windows_select_callback(p_walker_1: *mut Walker,
    p: *mut Select) -> i32 {
    unsafe {
        return if p == unsafe { (*p_walker_1).u.p_select } { 0 } else { 1 };
    }
}

extern "C" fn gather_select_windows(p: *mut Select) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        w.x_expr_callback = Some(gather_select_windows_callback);
        w.x_select_callback = Some(gather_select_windows_select_callback);
        w.x_select_callback2 = None;
        w.p_parse = core::ptr::null_mut();
        w.u.p_select = p;
        unsafe { sqlite3_walk_select(&mut w, p) };
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_select_dup(db: *mut Sqlite3, p_dup: *const Select,
    flags: i32) -> *mut Select {
    unsafe {
        let mut p_ret: *mut Select = core::ptr::null_mut();
        let mut p_next: *mut Select = core::ptr::null_mut();
        let mut pp: *mut *mut Select = &mut p_ret;
        let mut p: *const Select = core::ptr::null();
        { let _ = 0; };
        {
            p = p_dup;
            '__b9: loop {
                if !(!(p).is_null()) { break '__b9; }
                '__c9: loop {
                    let p_new: *mut Select =
                        unsafe {
                                sqlite3_db_malloc_raw_nn(db,
                                    core::mem::size_of::<Select>() as u64)
                            } as *mut Select;
                    if p_new == core::ptr::null_mut() { break '__b9; }
                    unsafe {
                        (*p_new).p_e_list =
                            sqlite3_expr_list_dup(db,
                                unsafe { (*p).p_e_list } as *const ExprList, flags)
                    };
                    unsafe {
                        (*p_new).p_src =
                            sqlite3_src_list_dup(db,
                                unsafe { (*p).p_src } as *const SrcList, flags)
                    };
                    unsafe {
                        (*p_new).p_where =
                            sqlite3_expr_dup(db, unsafe { (*p).p_where } as *const Expr,
                                flags)
                    };
                    unsafe {
                        (*p_new).p_group_by =
                            sqlite3_expr_list_dup(db,
                                unsafe { (*p).p_group_by } as *const ExprList, flags)
                    };
                    unsafe {
                        (*p_new).p_having =
                            sqlite3_expr_dup(db,
                                unsafe { (*p).p_having } as *const Expr, flags)
                    };
                    unsafe {
                        (*p_new).p_order_by =
                            sqlite3_expr_list_dup(db,
                                unsafe { (*p).p_order_by } as *const ExprList, flags)
                    };
                    unsafe { (*p_new).op = unsafe { (*p).op } as u8 };
                    unsafe { (*p_new).p_next = p_next };
                    unsafe { (*p_new).p_prior = core::ptr::null_mut() };
                    unsafe {
                        (*p_new).p_limit =
                            sqlite3_expr_dup(db, unsafe { (*p).p_limit } as *const Expr,
                                flags)
                    };
                    unsafe { (*p_new).i_limit = 0 };
                    unsafe { (*p_new).i_offset = 0 };
                    unsafe {
                        (*p_new).sel_flags = unsafe { (*p).sel_flags } as u32
                    };
                    unsafe {
                        (*p_new).n_select_row =
                            unsafe { (*p).n_select_row } as LogEst
                    };
                    unsafe {
                        (*p_new).p_with =
                            sqlite3_with_dup(db, unsafe { (*p).p_with })
                    };
                    unsafe { (*p_new).p_win = core::ptr::null_mut() };
                    unsafe {
                        (*p_new).p_win_defn =
                            unsafe {
                                sqlite3_window_list_dup(db, unsafe { (*p).p_win_defn })
                            }
                    };
                    if !(unsafe { (*p).p_win }).is_null() &&
                            unsafe { (*db).malloc_failed } as i32 == 0 {
                        gather_select_windows(p_new);
                    }
                    unsafe { (*p_new).sel_id = unsafe { (*p).sel_id } as u32 };
                    if unsafe { (*db).malloc_failed } != 0 {

                        /// Any prior OOM might have left the Select object incomplete.
                        ///* Delete the whole thing rather than allow an incomplete Select
                        ///* to be used by the code generator.
                        unsafe { (*p_new).p_next = core::ptr::null_mut() };
                        unsafe { sqlite3_select_delete(db, p_new) };
                        break '__b9;
                    }
                    unsafe { *pp = p_new };
                    pp = unsafe { &mut (*p_new).p_prior };
                    p_next = p_new;
                    break '__c9;
                }
                p = unsafe { (*p).p_prior } as *const Select;
            }
        }
        return p_ret;
    }
}

///* This function is similar to sqlite3ExprDup(), except that if pEdupBuf
///* is not NULL then it points to memory that can be used to store a copy
///* of the input Expr p together with its p->u.zToken (if any).  pEdupBuf
///* is updated with the new buffer tail prior to returning.
#[allow(unused_doc_comments)]
extern "C" fn expr_dup(db: *mut Sqlite3, p: *const Expr, dup_flags_1: i32,
    p_edup_buf_1: *mut EdupBuf) -> *mut Expr {
    unsafe {
        let mut p_new: *mut Expr = core::ptr::null_mut();
        /// Value to return
        let mut s_edup_buf: EdupBuf = unsafe { core::mem::zeroed() };
        /// Memory space from which to build Expr object
        let mut static_flag: u32 = 0 as u32;
        /// EP_Static if space not obtained from malloc
        let mut n_token: i32 = -1;

        /// Space needed for p->u.zToken.  -1 means unknown
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if !(p_edup_buf_1).is_null() {
            s_edup_buf.z_alloc = unsafe { (*p_edup_buf_1).z_alloc };
            static_flag = 134217728 as u32;
            { let _ = 0; };
            { let _ = 0; };
        } else {
            let mut n_alloc: i32 = 0;
            if dup_flags_1 != 0 {
                n_alloc = duped_expr_size(p);
            } else if !(unsafe { (*p).flags } & 2048 as u32 != 0 as u32) as
                            i32 != 0 && !(unsafe { (*p).u.z_token }).is_null() {
                n_token =
                    ((unsafe { strlen(unsafe { (*p).u.z_token } as *const i8) }
                                & 1073741823 as u64) + 1 as u64) as i32;
                n_alloc =
                    (core::mem::size_of::<Expr>() as u64 + n_token as u64 +
                                7 as u64 & !7 as u64) as i32;
            } else {
                n_token = 0;
                n_alloc =
                    (core::mem::size_of::<Expr>() as u64 + 7 as u64 & !7 as u64)
                        as i32;
            }
            { let _ = 0; };
            s_edup_buf.z_alloc =
                unsafe { sqlite3_db_malloc_raw_nn(db, n_alloc as u64) } as
                    *mut u8;
            static_flag = 0 as u32;
        }
        p_new = s_edup_buf.z_alloc as *mut Expr;
        { let _ = 0; };
        if !(p_new).is_null() {
            /// Set nNewSize to the size allocated for the structure pointed to
            ///* by pNew. This is either EXPR_FULLSIZE, EXPR_REDUCEDSIZE or
            ///* EXPR_TOKENONLYSIZE. nToken is set to the number of bytes consumed
            ///* by the copy of the p->u.zToken string (if any).
            let n_struct_size: u32 =
                duped_expr_struct_size(unsafe { &*p }, dup_flags_1) as u32;
            let mut n_new_size: i32 = (n_struct_size & 4095 as u32) as i32;
            if n_token < 0 {
                if !(unsafe { (*p).flags } & 2048 as u32 != 0 as u32) as i32
                            != 0 && !(unsafe { (*p).u.z_token }).is_null() {
                    n_token =
                        unsafe {
                                sqlite3_strlen30(unsafe { (*p).u.z_token } as *const i8)
                            } + 1;
                } else { n_token = 0; }
            }
            if dup_flags_1 != 0 {
                { let _ = 0; };
                { let _ = 0; };
                unsafe {
                    memcpy(s_edup_buf.z_alloc as *mut (), p as *const (),
                        n_new_size as u64)
                };
            } else {
                let n_size: u32 = expr_struct_size(unsafe { &*p }) as u32;
                { let _ = 0; };
                unsafe {
                    memcpy(s_edup_buf.z_alloc as *mut (), p as *const (),
                        n_size as u64)
                };
                if (n_size as u64) < core::mem::size_of::<Expr>() as u64 {
                    unsafe {
                        memset(unsafe {
                                    &raw mut *s_edup_buf.z_alloc.add(n_size as usize)
                                } as *mut (), 0,
                            core::mem::size_of::<Expr>() as u64 - n_size as u64)
                    };
                }
                n_new_size = core::mem::size_of::<Expr>() as i32;
            }

            /// Set the EP_Reduced, EP_TokenOnly, and EP_Static flags appropriately.
            unsafe { (*p_new).flags &= !(16384 | 65536 | 134217728) as u32 };
            unsafe {
                (*p_new).flags |= n_struct_size & (16384 | 65536) as u32
            };
            unsafe { (*p_new).flags |= static_flag };
            if dup_flags_1 != 0 {}

            /// Copy the p->u.zToken string, if any.
            { let _ = 0; };
            if n_token > 0 {
                let z_token: *mut i8 =
                    {
                        unsafe {
                            (*p_new).u.z_token =
                                unsafe {
                                        &raw mut *s_edup_buf.z_alloc.offset(n_new_size as isize)
                                    } as *mut i8
                        };
                        unsafe { (*p_new).u.z_token }
                    };
                unsafe {
                    memcpy(z_token as *mut (),
                        unsafe { (*p).u.z_token } as *const (), n_token as u64)
                };
                n_new_size += n_token;
            }
            {
                let __n = n_new_size + 7 & !7;
                let __p = &mut s_edup_buf.z_alloc;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            if (unsafe { (*p).flags } | unsafe { (*p_new).flags }) &
                        (65536 | 8388608) as u32 == 0 as u32 {
                if unsafe { (*p).flags } & 4096 as u32 != 0 as u32 {
                    unsafe {
                        (*p_new).x.p_select =
                            sqlite3_select_dup(db,
                                unsafe { (*p).x.p_select } as *const Select, dup_flags_1)
                    };
                } else {
                    unsafe {
                        (*p_new).x.p_list =
                            sqlite3_expr_list_dup(db,
                                unsafe { (*p).x.p_list } as *const ExprList,
                                if unsafe { (*p).op } as i32 != 146 {
                                    dup_flags_1
                                } else { 0 })
                    };
                }
                if unsafe { (*p).flags } & 16777216 as u32 != 0 as u32 {
                    unsafe {
                        (*p_new).y.p_win =
                            unsafe {
                                sqlite3_window_dup(db, p_new, unsafe { (*p).y.p_win })
                            }
                    };
                    { let _ = 0; };
                }
                if dup_flags_1 != 0 {
                    if unsafe { (*p).op } as i32 == 178 {
                        unsafe { (*p_new).p_left = unsafe { (*p).p_left } };
                        { let _ = 0; };
                    } else {
                        unsafe {
                            (*p_new).p_left =
                                if !(unsafe { (*p).p_left }).is_null() {
                                    expr_dup(db, unsafe { (*p).p_left } as *const Expr, 1,
                                        &mut s_edup_buf)
                                } else { core::ptr::null_mut() }
                        };
                    }
                    unsafe {
                        (*p_new).p_right =
                            if !(unsafe { (*p).p_right }).is_null() {
                                expr_dup(db, unsafe { (*p).p_right } as *const Expr, 1,
                                    &mut s_edup_buf)
                            } else { core::ptr::null_mut() }
                    };
                } else {
                    if unsafe { (*p).op } as i32 == 178 {
                        unsafe { (*p_new).p_left = unsafe { (*p).p_left } };
                        { let _ = 0; };
                    } else {
                        unsafe {
                            (*p_new).p_left =
                                sqlite3_expr_dup(db, unsafe { (*p).p_left } as *const Expr,
                                    0)
                        };
                    }
                    unsafe {
                        (*p_new).p_right =
                            sqlite3_expr_dup(db, unsafe { (*p).p_right } as *const Expr,
                                0)
                    };
                }
            }
        }
        if !(p_edup_buf_1).is_null() {
            unsafe {
                memcpy(p_edup_buf_1 as *mut (),
                    &raw mut s_edup_buf as *const (),
                    core::mem::size_of::<EdupBuf>() as u64)
            };
        }
        { let _ = 0; };
        return p_new;
    }
}

///* The following group of routines make deep copies of expressions,
///* expression lists, ID lists, and select statements.  The copies can
///* be deleted (by being passed to their respective ...Delete() routines)
///* without effecting the originals.
///*
///* The expression list, ID, and source lists return by sqlite3ExprListDup(),
///* sqlite3IdListDup(), and sqlite3SrcListDup() can not be further expanded
///* by subsequent calls to sqlite*ListAppend() routines.
///*
///* Any tables that the SrcList might point to are not duplicated.
///*
///* The flags parameter contains a combination of the EXPRDUP_XXX flags.
///* If the EXPRDUP_REDUCE flag is set, then the structure returned is a
///* truncated version of the usual Expr structure that will be stored as
///* part of the in-memory representation of the database schema.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_dup(db: *mut Sqlite3, p: *const Expr,
    flags: i32) -> *mut Expr {
    { let _ = 0; };
    return if !(p).is_null() {
            expr_dup(db, p, flags, core::ptr::null_mut())
        } else { core::ptr::null_mut() };
}

///* Compute and return a new Expr object which when passed to
///* sqlite3ExprCode() will generate all necessary code to compute
///* the iField-th column of the vector expression pVector.
///*
///* It is ok for pVector to be a scalar (as long as iField==0). 
///* In that case, this routine works like sqlite3ExprDup().
///*
///* The caller owns the returned Expr object and is responsible for
///* ensuring that the returned value eventually gets freed.
///*
///* The caller retains ownership of pVector.  If pVector is a TK_SELECT,
///* then the returned object will reference pVector and so pVector must remain
///* valid for the life of the returned object.  If pVector is a TK_VECTOR
///* or a scalar expression, then it can be deleted as soon as this routine
///* returns.
///*
///* A trick to cause a TK_SELECT pVector to be deleted together with
///* the returned Expr object is to attach the pVector to the pRight field
///* of the returned TK_SELECT_COLUMN Expr object.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_for_vector_field(p_parse: *mut Parse,
    mut p_vector: *mut Expr, i_field: i32, n_field: i32) -> *mut Expr {
    unsafe {
        let mut p_ret: *mut Expr = core::ptr::null_mut();
        if unsafe { (*p_vector).op } as i32 == 139 {
            { let _ = 0; };

            /// The TK_SELECT_COLUMN Expr node:
            ///*
            ///* pLeft:           pVector containing TK_SELECT.  Not deleted.
            ///* pRight:          not used.  But recursively deleted.
            ///* iColumn:         Index of a column in pVector
            ///* iTable:          0 or the number of columns on the LHS of an assignment
            ///* pLeft->iTable:   First in an array of register holding result, or 0
            ///*                  if the result is not yet computed.
            ///*
            ///* sqlite3ExprDelete() specifically skips the recursive delete of
            ///* pLeft on TK_SELECT_COLUMN nodes.  But pRight is followed, so pVector
            ///* can be attached to pRight to cause this node to take ownership of
            ///* pVector.  Typically there will be multiple TK_SELECT_COLUMN nodes
            ///* with the same pLeft pointer to the pVector, but only one of them
            ///* will own the pVector.
            (p_ret =
                sqlite3_p_expr(p_parse, 178, core::ptr::null_mut(),
                    core::ptr::null_mut()));
            if !(p_ret).is_null() {
                unsafe { (*p_ret).flags |= 131072 as u32 };
                unsafe { (*p_ret).i_table = n_field };
                unsafe { (*p_ret).i_column = i_field as YnVar };
                unsafe { (*p_ret).p_left = p_vector };
            }
        } else {
            if unsafe { (*p_vector).op } as i32 == 177 {
                let mut pp_vector: *mut *mut Expr = core::ptr::null_mut();
                { let _ = 0; };
                pp_vector =
                    unsafe {
                        &mut (*(unsafe {
                                                (*unsafe { (*p_vector).x.p_list }).a.as_ptr()
                                            } as *mut ExprListItem).offset(i_field as isize)).p_expr
                    };
                p_vector = unsafe { *pp_vector };
                if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {

                    /// This must be a vector UPDATE inside a trigger
                    unsafe { *pp_vector = core::ptr::null_mut() };
                    return p_vector;
                }
            }
            p_ret =
                sqlite3_expr_dup(unsafe { (*p_parse).db },
                    p_vector as *const Expr, 0);
        }
        return p_ret;
    }
}

///* pColumns and pExpr form a vector assignment which is part of the SET
///* clause of an UPDATE statement.  Like this:
///*
///*        (a,b,c) = (expr1,expr2,expr3)
///* Or:    (a,b,c) = (SELECT x,y,z FROM ....)
///*
///* For each term of the vector assignment, append new entries to the
///* expression list pList.  In the case of a subquery on the RHS, append
///* TK_SELECT_COLUMN expressions.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_list_append_vector(p_parse: *mut Parse,
    mut p_list: *mut ExprList, p_columns: *mut IdList, mut p_expr: *mut Expr)
    -> *mut ExprList {
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut i_first: i32 = 0;
    /// pColumns can only be NULL due to an OOM but an OOM will cause an
    ///* exit prior to this routine being invoked
    /// If the RHS is a vector, then we can immediately check to see that
    ///* the size of the RHS and LHS match.  But if the RHS is a SELECT,
    ///* wildcards ("*") in the result set of the SELECT must be expanded before
    ///* we can do the size check, so defer the size check until code generation.
    let mut p_sub_expr: *mut Expr = core::ptr::null_mut();
    let mut p_first: *mut Expr = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s11:
            {
            match __state {
                0 => { db = unsafe { (*p_parse).db }; __state = 3; }
                2 => {
                    sqlite3_expr_unmap_and_delete(p_parse, p_expr);
                    __state = 33;
                }
                3 => { __state = 4; }
                4 => { __state = 5; }
                5 => {
                    i_first =
                        if !(p_list).is_null() {
                            unsafe { (*p_list).n_expr }
                        } else { 0 };
                    __state = 6;
                }
                6 => {
                    if p_columns == core::ptr::null_mut() {
                        __state = 8;
                    } else { __state = 7; }
                }
                7 => {
                    if p_expr == core::ptr::null_mut() {
                        __state = 10;
                    } else { __state = 9; }
                }
                8 => { __state = 2; }
                9 => {
                    if unsafe { (*p_expr).op } as i32 != 139 &&
                            unsafe { (*p_columns).n_id } !=
                                { n = sqlite3_expr_vector_size(unsafe { &*p_expr }); n } {
                        __state = 12;
                    } else { __state = 11; }
                }
                10 => { __state = 2; }
                11 => { i = 0; __state = 15; }
                12 => {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"%d columns assigned %d values".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p_columns).n_id }, n)
                    };
                    __state = 13;
                }
                13 => { __state = 2; }
                14 => {
                    if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 &&
                                unsafe { (*p_expr).op } as i32 == 139 &&
                            p_list != core::ptr::null_mut() {
                        __state = 27;
                    } else { __state = 26; }
                }
                15 => {
                    if i < unsafe { (*p_columns).n_id } {
                        __state = 16;
                    } else { __state = 14; }
                }
                16 => {
                    p_sub_expr =
                        sqlite3_expr_for_vector_field(p_parse, p_expr, i,
                            unsafe { (*p_columns).n_id });
                    __state = 18;
                }
                17 => {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    __state = 15;
                }
                18 => { { let _ = 0; }; __state = 19; }
                19 => {
                    if p_sub_expr == core::ptr::null_mut() {
                        __state = 21;
                    } else { __state = 20; }
                }
                20 => {
                    p_list =
                        sqlite3_expr_list_append(unsafe { &*p_parse }, p_list,
                            p_sub_expr);
                    __state = 22;
                }
                21 => { __state = 17; }
                22 => {
                    if !(p_list).is_null() {
                        __state = 23;
                    } else { __state = 17; }
                }
                23 => { { let _ = 0; }; __state = 24; }
                24 => {
                    unsafe {
                        (*(unsafe { (*p_list).a.as_ptr() } as
                                            *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                            as isize)).z_e_name =
                            unsafe {
                                (*(unsafe { (*p_columns).a.as_ptr() } as
                                                *mut IdListItem).offset(i as isize)).z_name
                            }
                    };
                    __state = 25;
                }
                25 => {
                    unsafe {
                        (*(unsafe { (*p_columns).a.as_ptr() } as
                                            *mut IdListItem).offset(i as isize)).z_name =
                            core::ptr::null_mut()
                    };
                    __state = 17;
                }
                26 => { __state = 2; }
                27 => {
                    p_first =
                        unsafe {
                            (*(unsafe { (*p_list).a.as_ptr() } as
                                            *mut ExprListItem).offset(i_first as isize)).p_expr
                        };
                    __state = 28;
                }
                28 => { { let _ = 0; }; __state = 29; }
                29 => { { let _ = 0; }; __state = 30; }
                30 => {
                    unsafe { (*p_first).p_right = p_expr };
                    __state = 31;
                }
                31 => { p_expr = core::ptr::null_mut(); __state = 32; }
                32 => {
                    unsafe {
                        (*p_first).i_table = unsafe { (*p_columns).n_id }
                    };
                    __state = 26;
                }
                33 => {
                    unsafe { sqlite3_id_list_delete(db, p_columns) };
                    __state = 34;
                }
                34 => { return p_list; }
                _ => {}
            }
        }
    }

    /// pColumns can only be NULL due to an OOM but an OOM will cause an
    ///* exit prior to this routine being invoked
    /// If the RHS is a vector, then we can immediately check to see that
    ///* the size of the RHS and LHS match.  But if the RHS is a SELECT,
    ///* wildcards ("*") in the result set of the SELECT must be expanded before
    ///* we can do the size check, so defer the size check until code generation.
    /// Store the SELECT statement in pRight so it will be deleted when
    ///* sqlite3ExprListDelete() is called
    /// Remember the size of the LHS in iTable so that we can check that
    ///* the RHS and LHS sizes match during code generation.
    unreachable!();
}

///* Expression list pEList is a list of vector values. This function
///* converts the contents of pEList to a VALUES(...) Select statement
///* returning 1 row for each element of the list. For example, the
///* expression list:
///*
///*   ( (1,2), (3,4) (5,6) )
///*
///* is translated to the equivalent of:
///*
///*   VALUES(1,2), (3,4), (5,6)
///*
///* Each of the vector values in pEList must contain exactly nElem terms.
///* If a list element that is not a vector or does not contain nElem terms,
///* an error message is left in pParse.
///*
///* This is used as part of processing IN(...) expressions with a list
///* of vectors on the RHS. e.g. "... IN ((1,2), (3,4), (5,6))".
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_to_values(p_parse: *mut Parse,
    n_elem: i32, p_e_list: *mut ExprList) -> *mut Select {
    unsafe {
        let mut ii: i32 = 0;
        let mut p_ret: *mut Select = core::ptr::null_mut();
        { let _ = 0; };
        {
            ii = 0;
            '__b12: loop {
                if !(ii < unsafe { (*p_e_list).n_expr }) { break '__b12; }
                '__c12: loop {
                    let mut p_sel: *mut Select = core::ptr::null_mut();
                    let p_expr: *mut Expr =
                        unsafe {
                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                            *mut ExprListItem).offset(ii as isize)).p_expr
                        };
                    let mut n_expr_elem: i32 = 0;
                    if unsafe { (*p_expr).op } as i32 == 177 {
                        { let _ = 0; };
                        n_expr_elem =
                            unsafe { (*unsafe { (*p_expr).x.p_list }).n_expr };
                    } else { n_expr_elem = 1; }
                    if n_expr_elem != n_elem {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"IN(...) element has %d term%s - expected %d".as_ptr() as
                                        *mut i8 as *const i8, n_expr_elem,
                                if n_expr_elem > 1 {
                                    c"s".as_ptr() as *mut i8
                                } else { c"".as_ptr() as *mut i8 }, n_elem)
                        };
                        break '__b12;
                    }
                    { let _ = 0; };
                    p_sel =
                        unsafe {
                            sqlite3_select_new(p_parse, unsafe { (*p_expr).x.p_list },
                                core::ptr::null_mut(), core::ptr::null_mut(),
                                core::ptr::null_mut(), core::ptr::null_mut(),
                                core::ptr::null_mut(), 512 as u32, core::ptr::null_mut())
                        };
                    unsafe { (*p_expr).x.p_list = core::ptr::null_mut() };
                    if !(p_sel).is_null() {
                        if !(p_ret).is_null() {
                            unsafe { (*p_sel).op = 136 as u8 };
                            unsafe { (*p_sel).p_prior = p_ret };
                        }
                        p_ret = p_sel;
                    }
                    break '__c12;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        if !(p_ret).is_null() && !(unsafe { (*p_ret).p_prior }).is_null() {
            unsafe { (*p_ret).sel_flags |= 1024 as u32 };
        }
        sqlite3_expr_list_delete(unsafe { (*p_parse).db }, p_e_list);
        return p_ret;
    }
}

///* Set the sort order for the last element on the given ExprList.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_set_sort_order(p: *mut ExprList,
    mut i_sort_order: i32, e_nulls: i32) -> () {
    let mut p_item: *mut ExprListItem = core::ptr::null_mut();
    if p == core::ptr::null_mut() { return; }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    p_item =
        unsafe {
            &mut *(unsafe { (*p).a.as_ptr() } as
                            *mut ExprListItem).offset((unsafe { (*p).n_expr } - 1) as
                            isize)
        };
    { let _ = 0; };
    if i_sort_order == -1 { i_sort_order = 0; }
    unsafe { (*p_item).fg.sort_flags = i_sort_order as u8 };
    if e_nulls != -1 {
        unsafe { (*p_item).fg.set_b_nulls(1 as u32 as u32) };
        if i_sort_order != e_nulls {
            unsafe { (*p_item).fg.sort_flags |= 2 as u8 };
        }
    }
}

///* Set the ExprList.a[].zEName element of the most recently added item
///* on the expression list.
///*
///* pList might be NULL following an OOM error.  But pName should never be
///* NULL.  If a memory allocation fails, the pParse->db->mallocFailed flag
///* is set.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_list_set_name(p_parse: *mut Parse,
    p_list: *mut ExprList, p_name: *const Token, dequote: i32) -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if !(p_list).is_null() {
            let mut p_item: *mut ExprListItem = core::ptr::null_mut();
            { let _ = 0; };
            p_item =
                unsafe {
                    &mut *(unsafe { (*p_list).a.as_ptr() } as
                                    *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                    as isize)
                };
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                (*p_item).z_e_name =
                    unsafe {
                        sqlite3_db_str_n_dup(unsafe { (*p_parse).db },
                            unsafe { (*p_name).z }, unsafe { (*p_name).n } as u64)
                    }
            };
            if dequote != 0 {

                /// If dequote==0, then pName->z does not point to part of a DDL
                ///* statement handled by the parser. And so no token need be added
                ///* to the token-map.
                unsafe { sqlite3_dequote(unsafe { (*p_item).z_e_name }) };
                if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                    unsafe {
                        sqlite3_rename_token_map(p_parse,
                            unsafe { (*p_item).z_e_name } as *const (), p_name)
                    };
                }
            }
        }
    }
}

///* Set the ExprList.a[].zSpan element of the most recently added item
///* on the expression list.
///*
///* pList might be NULL following an OOM error.  But pSpan should never be
///* NULL.  If a memory allocation fails, the pParse->db->mallocFailed flag
///* is set.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_set_span(p_parse: &Parse,
    p_list: *mut ExprList, z_start: *const i8, z_end: *const i8) -> () {
    let db: *mut Sqlite3 = (*p_parse).db;
    { let _ = 0; };
    if !(p_list).is_null() {
        let p_item: *mut ExprListItem =
            unsafe {
                &mut *(unsafe { (*p_list).a.as_ptr() } as
                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                as isize)
            };
        { let _ = 0; };
        if unsafe { (*p_item).z_e_name } == core::ptr::null_mut() {
            unsafe {
                (*p_item).z_e_name =
                    unsafe { sqlite3_db_span_dup(db, z_start, z_end) }
            };
            unsafe { (*p_item).fg.set_e_e_name(1 as u32 as u32) };
        }
    }
}

///* Clear both elements of an OnOrUsing object
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_clear_on_or_using(db: *mut Sqlite3,
    p: *mut OnOrUsing) -> () {
    unsafe {
        if p == core::ptr::null_mut()
            {} else if !(unsafe { (*p).p_on }).is_null() {
            sqlite3_expr_delete_nn(db, unsafe { (*p).p_on });
        } else if !(unsafe { (*p).p_using }).is_null() {
            unsafe { sqlite3_id_list_delete(db, unsafe { (*p).p_using }) };
        }
    }
}

///* Expression pVar is guaranteed to be an SQL variable. pExpr may be any
///* type of expression.
///*
///* If pExpr is a simple SQL value - an integer, real, string, blob
///* or NULL value - then the VDBE currently being prepared is configured
///* to re-prepare each time a new value is bound to variable pVar.
///*
///* Additionally, if pExpr is a simple SQL value and the value is the
///* same as that currently bound to variable pVar, non-zero is returned.
///* Otherwise, if the values are not the same or if pExpr is not a simple
///* SQL value, zero is returned.
///*
///* If the SQLITE_EnableQPSG flag is set on the database connection, then
///* this routine always returns false.
extern "C" fn expr_compare_variable(p_parse_1: &Parse, p_var_1: &Expr,
    p_expr_1: *const Expr) -> i32 {
    let mut res: i32 = 2;
    let mut i_var: i32 = 0;
    let mut p_l: *mut Sqlite3Value = core::ptr::null_mut();
    let mut p_r: *mut Sqlite3Value = core::ptr::null_mut();
    if unsafe { (*p_expr_1).op } as i32 == 157 &&
            (*p_var_1).i_column as i32 ==
                unsafe { (*p_expr_1).i_column } as i32 {
        return 0;
    }
    if unsafe { (*(*p_parse_1).db).flags } & 8388608 as u64 != 0 as u64 {
        return 2;
    }
    unsafe {
        sqlite3_value_from_expr((*p_parse_1).db, p_expr_1, 1 as u8, 65 as u8,
            &mut p_r)
    };
    if !(p_r).is_null() {
        i_var = (*p_var_1).i_column as i32;
        unsafe { sqlite3_vdbe_set_varmask((*p_parse_1).p_vdbe, i_var) };
        p_l =
            unsafe {
                sqlite3_vdbe_get_bound_value((*p_parse_1).p_reprepare, i_var,
                    65 as u8)
            };
        if !(p_l).is_null() {
            if unsafe { sqlite3_value_type(p_l) } == 3 {
                unsafe { sqlite3_value_text(p_l) };
            }
            res =
                if unsafe {
                            sqlite3_mem_compare(p_l as *const Mem, p_r as *const Mem,
                                core::ptr::null())
                        } != 0 {
                    2
                } else { 0 };
        }
        unsafe { sqlite3ValueFree(p_r) };
        unsafe { sqlite3ValueFree(p_l) };
    }
    return res;
}

///* Compare two ExprList objects.  Return 0 if they are identical, 1
///* if they are certainly different, or 2 if it is not possible to
///* determine if they are identical or not.
///*
///* If any subelement of pB has Expr.iTable==(-1) then it is allowed
///* to compare equal to an equivalent element in pA with Expr.iTable==iTab.
///*
///* This routine might return non-zero for equivalent ExprLists.  The
///* only consequence will be disabled optimizations.  But this routine
///* must never return 0 if the two ExprList objects are different, or
///* a malfunction will result.
///*
///* Two NULL pointers are considered to be the same.  But a NULL pointer
///* always differs from a non-NULL pointer.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_compare(p_a: *const ExprList,
    p_b: *const ExprList, i_tab: i32) -> i32 {
    let mut i: i32 = 0;
    if p_a == core::ptr::null() && p_b == core::ptr::null() { return 0; }
    if p_a == core::ptr::null() || p_b == core::ptr::null() { return 1; }
    if unsafe { (*p_a).n_expr } as i32 != unsafe { (*p_b).n_expr } {
        return 1;
    }
    {
        i = 0;
        '__b13: loop {
            if !(i < unsafe { (*p_a).n_expr }) { break '__b13; }
            '__c13: loop {
                let mut res: i32 = 0;
                let p_expr_a: *const Expr =
                    unsafe {
                            (*(unsafe { (*p_a).a.as_ptr() } as
                                            *const ExprListItem).offset(i as isize)).p_expr
                        } as *const Expr;
                let p_expr_b: *const Expr =
                    unsafe {
                            (*(unsafe { (*p_b).a.as_ptr() } as
                                            *const ExprListItem).offset(i as isize)).p_expr
                        } as *const Expr;
                if unsafe {
                                (*(unsafe { (*p_a).a.as_ptr() } as
                                                    *const ExprListItem).offset(i as isize)).fg.sort_flags
                            } as i32 !=
                        unsafe {
                                (*(unsafe { (*p_b).a.as_ptr() } as
                                                    *const ExprListItem).offset(i as isize)).fg.sort_flags
                            } as i32 {
                    return 1;
                }
                if {
                            res =
                                sqlite3_expr_compare(core::ptr::null(),
                                    p_expr_a as *const Expr, p_expr_b as *const Expr, i_tab);
                            res
                        } != 0 {
                    return res;
                }
                break '__c13;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Do a deep comparison of two expression trees.  Return 0 if the two
///* expressions are completely identical.  Return 1 if they differ only
///* by a COLLATE operator at the top level.  Return 2 if there are differences
///* other than the top-level COLLATE operator.
///*
///* If any subelement of pB has Expr.iTable==(-1) then it is allowed
///* to compare equal to an equivalent element in pA with Expr.iTable==iTab.
///*
///* The pA side might be using TK_REGISTER.  If that is the case and pB is
///* not using TK_REGISTER but is otherwise equivalent, then still return 0.
///*
///* Sometimes this routine will return 2 even if the two expressions
///* really are equivalent.  If we cannot prove that the expressions are
///* identical, we return 2 just to be safe.  So if this routine
///* returns 2, then you do not really know for certain if the two
///* expressions are the same.  But if you get a 0 or 1 return, then you
///* can be sure the expressions are the same.  In the places where
///* this routine is used, it does not hurt to get an extra 2 - that
///* just might result in some slightly slower code.  But returning
///* an incorrect 0 or 1 could lead to a malfunction.
///*
///* If pParse is not NULL and SQLITE_EnableQPSG is off then TK_VARIABLE
///* terms in pA with bindings in pParse->pReprepare can be matched against
///* literals in pB.  The pParse->pVdbe->expmask bitmask is updated for
///* each variable referenced.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_compare(p_parse: *const Parse,
    p_a: *const Expr, p_b: *const Expr, i_tab: i32) -> i32 {
    unsafe {
        let mut combined_flags: u32 = 0 as u32;
        if p_a == core::ptr::null() || p_b == core::ptr::null() {
            return if p_b == p_a { 0 } else { 2 };
        }
        if !(p_parse).is_null() && unsafe { (*p_a).op } as i32 == 157 {
            return expr_compare_variable(unsafe { &*p_parse },
                    unsafe { &*p_a }, p_b);
        }
        combined_flags =
            unsafe { (*p_a).flags } | unsafe { (*p_b).flags } as u32;
        if combined_flags & 2048 as u32 != 0 {
            if unsafe { (*p_a).flags } & unsafe { (*p_b).flags } as u32 &
                            2048 as u32 != 0 as u32 &&
                    unsafe { (*p_a).u.i_value } as i32 ==
                        unsafe { (*p_b).u.i_value } {
                return 0;
            }
            return 2;
        }
        if unsafe { (*p_a).op } as i32 != unsafe { (*p_b).op } as i32 ||
                unsafe { (*p_a).op } as i32 == 72 {
            if unsafe { (*p_a).op } as i32 == 114 &&
                    sqlite3_expr_compare(p_parse,
                            unsafe { (*p_a).p_left } as *const Expr, p_b, i_tab) < 2 {
                return 1;
            }
            if unsafe { (*p_b).op } as i32 == 114 &&
                    sqlite3_expr_compare(p_parse, p_a,
                            unsafe { (*p_b).p_left } as *const Expr, i_tab) < 2 {
                return 1;
            }
            if unsafe { (*p_a).op } as i32 == 170 &&
                            unsafe { (*p_b).op } as i32 == 168 &&
                        (unsafe { (*p_b).i_table } as i32) < 0 &&
                    unsafe { (*p_a).i_table } as i32 == i_tab
                {} else { return 2; }
        }
        { let _ = 0; };
        { let _ = 0; };
        if !(unsafe { (*p_a).u.z_token }).is_null() {
            if unsafe { (*p_a).op } as i32 == 172 ||
                    unsafe { (*p_a).op } as i32 == 169 {
                if unsafe {
                            sqlite3_str_i_cmp(unsafe { (*p_a).u.z_token } as *const i8,
                                unsafe { (*p_b).u.z_token } as *const i8)
                        } != 0 {
                    return 2;
                }
                { let _ = 0; };
                if (unsafe { (*p_a).flags } & 16777216 as u32 != 0 as u32) as
                            i32 !=
                        (unsafe { (*p_b).flags } & 16777216 as u32 != 0 as u32) as
                            i32 {
                    return 2;
                }
                if unsafe { (*p_a).flags } & 16777216 as u32 != 0 as u32 {
                    if unsafe {
                                sqlite3_window_compare(p_parse,
                                    unsafe { (*p_a).y.p_win } as *const Window,
                                    unsafe { (*p_b).y.p_win } as *const Window, 1)
                            } != 0 {
                        return 2;
                    }
                }
            } else if unsafe { (*p_a).op } as i32 == 122 {
                return 0;
            } else if unsafe { (*p_a).op } as i32 == 114 {
                if unsafe {
                            sqlite3_stricmp(unsafe { (*p_a).u.z_token } as *const i8,
                                unsafe { (*p_b).u.z_token } as *const i8)
                        } != 0 {
                    return 2;
                }
            } else if unsafe { (*p_b).u.z_token } != core::ptr::null_mut() &&
                            unsafe { (*p_a).op } as i32 != 168 &&
                        unsafe { (*p_a).op } as i32 != 170 &&
                    unsafe {
                            strcmp(unsafe { (*p_a).u.z_token } as *const i8,
                                unsafe { (*p_b).u.z_token } as *const i8)
                        } != 0 {
                return 2;
            }
        }
        if unsafe { (*p_a).flags } & (4 | 1024) as u32 !=
                unsafe { (*p_b).flags } & (4 | 1024) as u32 {
            return 2;
        }
        if combined_flags & 65536 as u32 == 0 as u32 {
            if combined_flags & 4096 as u32 != 0 { return 2; }
            if combined_flags & 32 as u32 == 0 as u32 &&
                    sqlite3_expr_compare(p_parse,
                            unsafe { (*p_a).p_left } as *const Expr,
                            unsafe { (*p_b).p_left } as *const Expr, i_tab) != 0 {
                return 2;
            }
            if sqlite3_expr_compare(p_parse,
                        unsafe { (*p_a).p_right } as *const Expr,
                        unsafe { (*p_b).p_right } as *const Expr, i_tab) != 0 {
                return 2;
            }
            if sqlite3_expr_list_compare(unsafe { (*p_a).x.p_list } as
                            *const ExprList,
                        unsafe { (*p_b).x.p_list } as *const ExprList, i_tab) != 0 {
                return 2;
            }
            if unsafe { (*p_a).op } as i32 != 118 &&
                        unsafe { (*p_a).op } as i32 != 171 &&
                    combined_flags & 16384 as u32 == 0 as u32 {
                if unsafe { (*p_a).i_column } as i32 !=
                        unsafe { (*p_b).i_column } as i32 {
                    return 2;
                }
                if unsafe { (*p_a).op2 } as i32 !=
                            unsafe { (*p_b).op2 } as i32 &&
                        unsafe { (*p_a).op } as i32 == 175 {
                    return 2;
                }
                if unsafe { (*p_a).op } as i32 != 50 &&
                            unsafe { (*p_a).i_table } as i32 !=
                                unsafe { (*p_b).i_table } &&
                        unsafe { (*p_a).i_table } as i32 != i_tab {
                    return 2;
                }
            }
        }
        return 0;
    }
}

///* Return the affinity character for a single column of a table.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_table_column_affinity(p_tab: &Table, i_col: i32)
    -> i8 {
    if i_col < 0 || i_col >= (*p_tab).n_col as i32 { return 68 as i8; }
    return unsafe { (*(*p_tab).a_col.offset(i_col as isize)).affinity };
}

///* Return the 'affinity' of the expression pExpr if any.
///*
///* If pExpr is a column, a reference to a column via an 'AS' alias,
///* or a sub-select with a column as the return value, then the
///* affinity of that column is returned. Otherwise, 0x00 is returned,
///* indicating no affinity for the expression.
///*
///* i.e. the WHERE clause expressions in the following statements all
///* have an affinity:
///*
///* CREATE TABLE t1(a);
///* SELECT * FROM t1 WHERE a;
///* SELECT a AS b FROM t1 WHERE b;
///* SELECT * FROM t1 WHERE (select a from t1);
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_affinity(mut p_expr: *const Expr) -> i8 {
    unsafe {
        let mut op: i32 = 0;
        op = unsafe { (*p_expr).op } as i32;
        loop {
            if op == 168 ||
                    op == 170 &&
                        unsafe { (*p_expr).y.p_tab } != core::ptr::null_mut() {
                { let _ = 0; };
                { let _ = 0; };
                return sqlite3_table_column_affinity(unsafe {
                            &*unsafe { (*p_expr).y.p_tab }
                        }, unsafe { (*p_expr).i_column } as i32);
            }
            if op == 139 {
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                return sqlite3_expr_affinity(unsafe {
                                (*(unsafe {
                                                    (*unsafe {
                                                                        (*unsafe { (*p_expr).x.p_select }).p_e_list
                                                                    }).a.as_ptr()
                                                } as *mut ExprListItem).offset(0 as isize)).p_expr
                            } as *const Expr);
            }
            if op == 36 {
                { let _ = 0; };
                return unsafe {
                        sqlite3_affinity_type(unsafe { (*p_expr).u.z_token } as
                                *const i8, core::ptr::null_mut())
                    };
            }
            if op == 178 {
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                return sqlite3_expr_affinity(unsafe {
                                (*(unsafe {
                                                    (*unsafe {
                                                                        (*unsafe {
                                                                                        (*unsafe { (*p_expr).p_left }).x.p_select
                                                                                    }).p_e_list
                                                                    }).a.as_ptr()
                                                } as
                                                *mut ExprListItem).offset(unsafe { (*p_expr).i_column } as
                                                isize)).p_expr
                            } as *const Expr);
            }
            if op == 177 ||
                    op == 172 && unsafe { (*p_expr).aff_expr } as i32 == 88 {
                { let _ = 0; };
                return sqlite3_expr_affinity(unsafe {
                                (*(unsafe { (*unsafe { (*p_expr).x.p_list }).a.as_ptr() } as
                                                *mut ExprListItem).offset(0 as isize)).p_expr
                            } as *const Expr);
            }
            if unsafe { (*p_expr).flags } & (8192 | 262144) as u32 != 0 as u32
                {
                { let _ = 0; };
                p_expr = unsafe { (*p_expr).p_left } as *const Expr;
                op = unsafe { (*p_expr).op } as i32;
                continue;
            }
            if op != 176 { break; }
            op = unsafe { (*p_expr).op2 } as i32;
            if op == 176 { break; }
        }
        return unsafe { (*p_expr).aff_expr } as i8;
    }
}

///* Expression Node callback for sqlite3ExprCanReturnSubtype().  If
///* pExpr is able to return a subtype, set pWalker->eCode and abort
///* the search.  If pExpr can never return a subtype, prune search.
///*
///* The only expressions that can return a subtype are:
///*
///*    1.  A function
///*    2.  The no-op "+" operator
///*    3.  A CASE...END expression
///*    4.  A CAST() expression
///*    5.  A "expr COLLATE colseq" expression.
///*
///* For any other kind of expression, prune the search.
///*
///* For case 1, the expression can yield a subtype if the function has
///* the SQLITE_RESULT_SUBTYPE property.  Functions can also return
///* a subtype (via sqlite3_result_value()) if any of the arguments can
///* return a subtype.
///*
///* In all cases 1 through 5, the expression might also return a subtype
///* if any operand can return a subtype.
extern "C" fn expr_node_can_return_subtype(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut n: i32 = 0;
        let mut p_def: *const FuncDef = core::ptr::null();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        if unsafe { (*p_expr_1).op } as i32 == 158 ||
                        unsafe { (*p_expr_1).op } as i32 == 173 ||
                    unsafe { (*p_expr_1).op } as i32 == 114 ||
                unsafe { (*p_expr_1).op } as i32 == 36 {
            return 0;
        }
        if unsafe { (*p_expr_1).op } as i32 != 172 { return 1; }
        { let _ = 0; };
        db = unsafe { (*unsafe { (*p_walker_1).p_parse }).db };
        n =
            if !(unsafe { (*p_expr_1).x.p_list }).is_null() {
                unsafe { (*unsafe { (*p_expr_1).x.p_list }).n_expr }
            } else { 0 };
        p_def =
            unsafe {
                sqlite3_find_function(db,
                    unsafe { (*p_expr_1).u.z_token } as *const i8, n,
                    unsafe { (*db).enc }, 0 as u8)
            };
        if p_def == core::ptr::null_mut() ||
                unsafe { (*p_def).func_flags } & 16777216 as u32 != 0 as u32 {
            unsafe { (*p_walker_1).e_code = 1 as u16 };
            return 2;
        }
        return 0;
    }
}

///* Return TRUE if expression pExpr is able to return a subtype.
///*
///* A TRUE return does not guarantee that a subtype will be returned.
///* It only indicates that a subtype return is possible.  False positives
///* are acceptable as they only disable an optimization.  False negatives,
///* on the other hand, can lead to incorrect answers.
extern "C" fn sqlite3_expr_can_return_subtype(p_parse_1: *mut Parse,
    p_expr_1: *mut Expr) -> i32 {
    let mut w: Walker = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut w as *mut (), 0,
            core::mem::size_of::<Walker>() as u64)
    };
    w.p_parse = p_parse_1;
    w.x_expr_callback = Some(expr_node_can_return_subtype);
    unsafe { sqlite3_walk_expr(&mut w, p_expr_1) };
    return w.e_code as i32;
}

///* Check to see if pExpr is one of the indexed expressions on pParse->pIdxEpr.
///* If it is, then resolve the expression by reading from the index and
///* return the register into which the value has been read.  If pExpr is
///* not an indexed expression, then return negative.
#[allow(unused_doc_comments)]
extern "C" fn sqlite3_indexed_expr_lookup(p_parse_1: *mut Parse,
    p_expr_1: *mut Expr, target: i32) -> i32 {
    let mut p: *mut IndexedExpr = core::ptr::null_mut();
    let mut v: *mut Vdbe = core::ptr::null_mut();
    {
        p = unsafe { (*p_parse_1).p_idx_epr };
        '__b15: loop {
            if !(!(p).is_null()) { break '__b15; }
            '__c15: loop {
                let mut expr_aff: u8 = 0 as u8;
                let mut i_data_cur: i32 = unsafe { (*p).i_data_cur };
                if i_data_cur < 0 { break '__c15; }
                if unsafe { (*p_parse_1).i_self_tab } != 0 {
                    if unsafe { (*p).i_data_cur } !=
                            unsafe { (*p_parse_1).i_self_tab } - 1 {
                        break '__c15;
                    }
                    i_data_cur = -1;
                }
                if sqlite3_expr_compare(core::ptr::null(),
                            p_expr_1 as *const Expr,
                            unsafe { (*p).p_expr } as *const Expr, i_data_cur) != 0 {
                    break '__c15;
                }
                { let _ = 0; };
                expr_aff =
                    sqlite3_expr_affinity(p_expr_1 as *const Expr) as u8;
                if expr_aff as i32 <= 65 && unsafe { (*p).aff } as i32 != 65
                            || expr_aff as i32 == 66 && unsafe { (*p).aff } as i32 != 66
                        || expr_aff as i32 >= 67 && unsafe { (*p).aff } as i32 != 67
                    {

                    /// Affinity mismatch on a generated column
                    break '__c15;
                }
                if unsafe { (*p_expr_1).flags } & 2147483648u32 as u32 !=
                            0 as u32 &&
                        sqlite3_expr_can_return_subtype(p_parse_1, p_expr_1) != 0 {
                    break '__c15;
                }
                v = unsafe { (*p_parse_1).p_vdbe };
                { let _ = 0; };
                if unsafe { (*p).b_maybe_null_row } != 0 {
                    /// If the index is on a NULL row due to an outer join, then we
                    ///* cannot extract the value from the index.  The value must be
                    ///* computed using the original expression.
                    let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 20, unsafe { (*p).i_idx_cur },
                            addr + 3, target)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 96, unsafe { (*p).i_idx_cur },
                            unsafe { (*p).i_idx_col }, target)
                    };
                    unsafe { sqlite3_vdbe_goto(v, 0) };
                    p = unsafe { (*p_parse_1).p_idx_epr };
                    unsafe { (*p_parse_1).p_idx_epr = core::ptr::null_mut() };
                    sqlite3_expr_code(p_parse_1, p_expr_1, target);
                    unsafe { (*p_parse_1).p_idx_epr = p };
                    unsafe { sqlite3_vdbe_jump_here(v, addr + 2) };
                } else {
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 96, unsafe { (*p).i_idx_cur },
                            unsafe { (*p).i_idx_col }, target)
                    };
                }
                return target;
                break '__c15;
            }
            p = unsafe { (*p).p_ie_next };
        }
    }
    return -1;
}

///* Generate code that will compute the value of generated column pCol
///* and store the result in register regOut
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_generated_column(p_parse: *mut Parse,
    p_tab: *mut Table, p_col: *mut Column, reg_out: i32) -> () {
    let mut i_addr: i32 = 0;
    let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
    let n_err: i32 = unsafe { (*p_parse).n_err };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_parse).i_self_tab } > 0 {
        i_addr =
            unsafe {
                sqlite3_vdbe_add_op3(v, 20,
                    unsafe { (*p_parse).i_self_tab } - 1, 0, reg_out)
            };
    } else { i_addr = 0; }
    sqlite3_expr_code_copy(p_parse,
        unsafe { sqlite3_column_expr(p_tab, p_col) }, reg_out);
    if unsafe { (*p_col).col_flags } as i32 & 32 != 0 &&
            unsafe { (*p_tab).tab_flags } & 65536 as u32 != 0 as u32 {
        let p3: i32 =
            2 +
                unsafe { p_col.offset_from(unsafe { (*p_tab).a_col }) } as i64
                    as i32;
        unsafe {
            sqlite3_vdbe_add_op4(v, 97, reg_out, 1, p3,
                p_tab as *mut i8 as *const i8, -5)
        };
    } else if unsafe { (*p_col).affinity } as i32 >= 66 {
        unsafe {
            sqlite3_vdbe_add_op4(v, 98, reg_out, 1, 0,
                unsafe { &raw mut (*p_col).affinity } as *const i8, 1)
        };
    }
    if i_addr != 0 { unsafe { sqlite3_vdbe_jump_here(v, i_addr) }; }
    if unsafe { (*p_parse).n_err } > n_err {
        unsafe { (*unsafe { (*p_parse).db }).err_byte_offset = -1 };
    }
}

///* Expression pExpr is guaranteed to be a TK_COLUMN or equivalent. This
///* function checks the Parse.pIdxPartExpr list to see if this column
///* can be replaced with a constant value. If so, it generates code to
///* put the constant value in a register (ideally, but not necessarily, 
///* register iTarget) and returns the register number.
///*
///* Or, if the TK_COLUMN cannot be replaced by a constant, zero is 
///* returned.
extern "C" fn expr_partidx_expr_lookup(p_parse_1: *mut Parse, p_expr_1: &Expr,
    i_target_1: i32) -> i32 {
    let mut p: *mut IndexedExpr = core::ptr::null_mut();
    {
        p = unsafe { (*p_parse_1).p_idx_part_expr };
        '__b16: loop {
            if !(!(p).is_null()) { break '__b16; }
            '__c16: loop {
                if (*p_expr_1).i_column as i32 == unsafe { (*p).i_idx_col } &&
                        (*p_expr_1).i_table == unsafe { (*p).i_data_cur } {
                    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
                    let mut addr: i32 = 0;
                    let mut ret: i32 = 0;
                    if unsafe { (*p).b_maybe_null_row } != 0 {
                        addr =
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 20, unsafe { (*p).i_idx_cur })
                            };
                    }
                    ret =
                        sqlite3_expr_code_target(p_parse_1, unsafe { (*p).p_expr },
                            i_target_1);
                    unsafe {
                        sqlite3_vdbe_add_op4(unsafe { (*p_parse_1).p_vdbe }, 98,
                            ret, 1, 0, unsafe { &raw mut (*p).aff } as *const i8, 1)
                    };
                    if addr != 0 {
                        unsafe { sqlite3_vdbe_jump_here(v, addr) };
                        unsafe { sqlite3_vdbe_change_p3(v, addr, ret) };
                    }
                    return ret;
                }
                break '__c16;
            }
            p = unsafe { (*p).p_ie_next };
        }
    }
    return 0;
}

///* Generate code to extract the value of the iCol-th column of a table.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_get_column_of_table(v: *mut Vdbe,
    p_tab: *mut Table, i_tab_cur: i32, i_col: i32, reg_out: i32) -> () {
    let mut p_col: *mut Column = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if i_col < 0 || i_col == unsafe { (*p_tab).i_p_key } as i32 {
        unsafe { sqlite3_vdbe_add_op2(v, 137, i_tab_cur, reg_out) };
    } else {
        let mut op: i32 = 0;
        let mut x: i32 = 0;
        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
            op = 178;
            x = i_col;
        } else if unsafe {
                            (*{
                                            p_col =
                                                unsafe { unsafe { (*p_tab).a_col.offset(i_col as isize) } };
                                            p_col
                                        }).col_flags
                        } as i32 & 32 != 0 {
            let p_parse: *mut Parse = unsafe { sqlite3_vdbe_parser(v) };
            if unsafe { (*p_col).col_flags } as i32 & 256 != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"generated column loop on \"%s\"".as_ptr() as *mut i8 as
                            *const i8, unsafe { (*p_col).z_cn_name })
                };
            } else {
                let saved_self_tab: i32 = unsafe { (*p_parse).i_self_tab };
                unsafe { (*p_col).col_flags |= 256 as u16 };
                unsafe { (*p_parse).i_self_tab = i_tab_cur + 1 };
                sqlite3_expr_code_generated_column(p_parse, p_tab, p_col,
                    reg_out);
                unsafe { (*p_parse).i_self_tab = saved_self_tab };
                unsafe { (*p_col).col_flags &= !256 as u16 };
            }
            return;
        } else if !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32) as
                    i32 != 0 {
            x =
                unsafe {
                    sqlite3_table_column_to_index(unsafe {
                            sqlite3_primary_key_index(p_tab)
                        }, i_col)
                };
            op = 96;
        } else {
            x =
                unsafe {
                        sqlite3_table_column_to_storage(p_tab, i_col as i16)
                    } as i32;
            op = 96;
        }
        unsafe { sqlite3_vdbe_add_op3(v, op, i_tab_cur, x, reg_out) };
        unsafe { sqlite3_column_default(v, p_tab, i_col, reg_out) };
    }
}

///* Generate code that will extract the iColumn-th column from
///* table pTab and store the column value in register iReg.
///*
///* There must be an open cursor to pTab in iTable when this routine
///* is called.  If iColumn<0 then code is generated that extracts the rowid.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_get_column(p_parse: &Parse,
    p_tab: *mut Table, i_column: i32, i_table: i32, i_reg: i32, p5: u8)
    -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    sqlite3_expr_code_get_column_of_table((*p_parse).p_vdbe, p_tab, i_table,
        i_column, i_reg);
    if p5 != 0 {
        let p_op: *mut VdbeOp =
            unsafe { sqlite3_vdbe_get_last_op((*p_parse).p_vdbe) };
        if unsafe { (*p_op).opcode } as i32 == 96 {
            unsafe { (*p_op).p5 = p5 as u16 };
        }
        if unsafe { (*p_op).opcode } as i32 == 178 {
            unsafe { (*p_op).p5 = (p5 as i32 & 1) as u16 };
        }
    }
    return i_reg;
}

///* Generate an instruction that will put the floating point
///* value described by z[0..n-1] into register iMem.
///*
///* The z[] string will probably not be zero-terminated.  But the
///* z[n] character is guaranteed to be something that does not look
///* like the continuation of the number.
extern "C" fn code_real(v: *mut Vdbe, z: *const i8, negate_flag_1: i32,
    i_mem_1: i32) -> () {
    if z != core::ptr::null() {
        let mut value: f64 = 0.0;
        unsafe { sqlite3_ato_f(z, &mut value) };
        { let _ = 0; };
        if negate_flag_1 != 0 { value = -value; }
        unsafe {
            sqlite3_vdbe_add_op4_dup8(v, 154, 0, i_mem_1, 0,
                &raw mut value as *mut u8 as *const u8, -13)
        };
    }
}

///* Generate an instruction that will put the integer describe by
///* text z[0..n-1] into register iMem.
///*
///* Expr.u.zToken is always UTF8 and zero-terminated.
extern "C" fn code_integer(p_parse_1: *mut Parse, p_expr_1: *mut Expr,
    neg_flag_1: i32, i_mem_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        if unsafe { (*p_expr_1).flags } & 2048 as u32 != 0 {
            let mut i: i32 = unsafe { (*p_expr_1).u.i_value };
            { let _ = 0; };
            if neg_flag_1 != 0 { i = -i; }
            unsafe { sqlite3_vdbe_add_op2(v, 73, i, i_mem_1) };
        } else {
            let mut c: i32 = 0;
            let mut value: i64 = 0 as i64;
            let z: *const i8 = unsafe { (*p_expr_1).u.z_token } as *const i8;
            { let _ = 0; };
            c = unsafe { sqlite3_dec_or_hex_to_i64(z, &mut value) };
            if c == 3 && (neg_flag_1 == 0) as i32 != 0 || c == 2 ||
                    neg_flag_1 != 0 &&
                        value ==
                            -1 as i64 -
                                (4294967295u32 as i64 | (2147483647 as i64) << 32) {
                if unsafe {
                            sqlite3_strnicmp(z, c"0x".as_ptr() as *mut i8 as *const i8,
                                2)
                        } == 0 {
                    unsafe {
                        sqlite3_error_msg(p_parse_1,
                            c"hex literal too big: %s%#T".as_ptr() as *mut i8 as
                                *const i8,
                            if neg_flag_1 != 0 {
                                c"-".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 }, p_expr_1)
                    };
                } else { code_real(v, z, neg_flag_1, i_mem_1); }
            } else {
                if neg_flag_1 != 0 {
                    value =
                        if c == 3 {
                            -1 as i64 -
                                (4294967295u32 as i64 | (2147483647 as i64) << 32)
                        } else { -value };
                }
                unsafe {
                    sqlite3_vdbe_add_op4_dup8(v, 74, 0, i_mem_1, 0,
                        &raw mut value as *mut u8 as *const u8, -14)
                };
            }
        }
    }
}

///* Skip over any TK_COLLATE operators and/or any unlikely()
///* or likelihood() or likely() functions at the root of an
///* expression.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_skip_collate_and_likely(mut p_expr: *mut Expr)
    -> *mut Expr {
    unsafe {
        while !(p_expr).is_null() &&
                unsafe { (*p_expr).flags } & (8192 | 524288) as u32 !=
                    0 as u32 {
            if unsafe { (*p_expr).flags } & 524288 as u32 != 0 as u32 {
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                p_expr =
                    unsafe {
                        (*(unsafe { (*unsafe { (*p_expr).x.p_list }).a.as_ptr() } as
                                        *mut ExprListItem).offset(0 as isize)).p_expr
                    };
            } else if unsafe { (*p_expr).op } as i32 == 114 {
                p_expr = unsafe { (*p_expr).p_left };
            } else { break; }
        }
        return p_expr;
    }
}

///* The argument must be a TK_TRUEFALSE Expr node.  Return 1 if it is TRUE
///* and 0 if it is FALSE.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_truth_value(mut p_expr: *const Expr) -> i32 {
    unsafe {
        p_expr =
            sqlite3_expr_skip_collate_and_likely(p_expr as *mut Expr) as
                *const Expr;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        return (unsafe { *unsafe { (*p_expr).u.z_token.offset(4 as isize) } }
                        as i32 == 0) as i32;
    }
}

///* Return true if expression pExpr is a vector, or false otherwise.
///*
///* A vector is defined as any expression that results in two or more
///* columns of result.  Every TK_VECTOR node is an vector because the
///* parser will not generate a TK_VECTOR with fewer than two entries.
///* But a TK_SELECT might be either a vector or a scalar. It is only
///* considered a vector if it has two or more result columns.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_is_vector(p_expr: *const Expr) -> i32 {
    return (sqlite3_expr_vector_size(unsafe { &*p_expr }) > 1) as i32;
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_code_subselect(p_parse: *mut Parse,
    p_expr: &mut Expr) -> i32 {
    unsafe {
        let mut addr_once: i32 = 0;
        /// Address of OP_Once at top of subroutine
        let mut r_reg: i32 = 0;
        /// Register storing resulting
        let mut p_sel: *mut Select = core::ptr::null_mut();
        /// SELECT statement to encode
        let mut dest: SelectDest = unsafe { core::mem::zeroed() };
        /// How to deal with SELECT result
        let mut n_reg: i32 = 0;
        /// Registers to allocate
        let mut p_limit: *mut Expr = core::ptr::null_mut();
        /// New limit expression
        let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
        { let _ = 0; };
        if unsafe { (*p_parse).n_err } != 0 { return 0; }
        { let _ = 0; };
        { let _ = 0; };
        p_sel = (*p_expr).x.p_select;
        if (*p_expr).flags & 33554432 as u32 != 0 as u32 {
            unsafe {
                sqlite3_vdbe_explain(p_parse, 0 as u8,
                    c"REUSE SUBQUERY %d".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p_sel).sel_id })
            };
            { let _ = 0; };
            unsafe {
                sqlite3_vdbe_add_op2(v, 10, (*p_expr).y.sub.reg_return,
                    (*p_expr).y.sub.i_addr)
            };
            return (*p_expr).i_table;
        }

        /// Begin coding the subroutine
        { let _ = 0; };
        { let _ = 0; };
        (*p_expr).flags |= 33554432 as u32;
        (*p_expr).y.sub.reg_return =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        (*p_expr).y.sub.i_addr =
            unsafe {
                    sqlite3_vdbe_add_op2(v, 76, 0, (*p_expr).y.sub.reg_return)
                } + 1;
        if !((*p_expr).flags & 64 as u32 != 0 as u32) as i32 != 0 {
            addr_once = unsafe { sqlite3_vdbe_add_op0(v, 15) };
        }

        /// For a SELECT, generate code to put the values for all columns of
        ///* the first row into an array of registers and return the index of
        ///* the first register.
        ///*
        ///* If this is an EXISTS, write an integer 0 (not exists) or 1 (exists)
        ///* into a register and return that register number.
        ///*
        ///* In both cases, the query is augmented with "LIMIT 1".  Any
        ///* preexisting limit is discarded in place of the new LIMIT 1.
        unsafe {
            sqlite3_vdbe_explain(p_parse, 1 as u8,
                c"%sSCALAR SUBQUERY %d".as_ptr() as *mut i8 as *const i8,
                if addr_once != 0 {
                    c"".as_ptr() as *mut i8
                } else { c"CORRELATED ".as_ptr() as *mut i8 },
                unsafe { (*p_sel).sel_id })
        };
        n_reg =
            if (*p_expr).op as i32 == 139 {
                unsafe { (*unsafe { (*p_sel).p_e_list }).n_expr }
            } else { 1 };
        unsafe {
            sqlite3_select_dest_init(&mut dest, 0,
                unsafe { (*p_parse).n_mem } + 1)
        };
        unsafe { (*p_parse).n_mem += n_reg };
        if (*p_expr).op as i32 == 139 {
            dest.e_dest = 8 as u8;
            if unsafe { (*p_sel).sel_flags } & 1 as u32 != 0 &&
                        !(unsafe { (*p_sel).p_limit }).is_null() &&
                    !(unsafe {
                                    (*unsafe { (*p_sel).p_limit }).p_right
                                }).is_null() {

                /// If there is both a DISTINCT and an OFFSET clause, then allocate
                ///* a separate dest.iSdst array for sqlite3Select() and other
                ///* routines to populate. In this case results will be copied over
                ///* into the dest.iSDParm array only after OFFSET processing. This
                ///* ensures that in the case where OFFSET excludes all rows, the
                ///* dest.iSDParm array is not left populated with the contents of the
                ///* last row visited - it should be all NULLs if all rows were
                ///* excluded by OFFSET.
                (dest.i_sdst = unsafe { (*p_parse).n_mem } + 1);
                unsafe { (*p_parse).n_mem += n_reg };
            } else { dest.i_sdst = dest.i_sd_parm; }
            dest.n_sdst = n_reg;
            unsafe {
                sqlite3_vdbe_add_op3(v, 77, 0, dest.i_sd_parm,
                    unsafe { (*p_parse).n_mem })
            };
        } else {
            dest.e_dest = 1 as u8;
            unsafe { sqlite3_vdbe_add_op2(v, 73, 0, dest.i_sd_parm) };
        }
        if !(unsafe { (*p_sel).p_limit }).is_null() {
            /// The subquery already has a limit.  If the pre-existing limit X is 
            ///* not already integer value 1 or 0, then make the new limit X<>0 so that
            ///* the new limit is either 1 or 0
            let p_left: *mut Expr =
                unsafe { (*unsafe { (*p_sel).p_limit }).p_left };
            if (unsafe { (*p_left).flags } & 2048 as u32 != 0 as u32) as i32
                        == 0 ||
                    unsafe { (*p_left).u.i_value } != 1 &&
                        unsafe { (*p_left).u.i_value } != 0 {
                let db: *mut Sqlite3 = unsafe { (*p_parse).db };
                p_limit = sqlite3_expr_int32(db, 0);
                if !(p_limit).is_null() {
                    unsafe { (*p_limit).aff_expr = 67 as i8 };
                    p_limit =
                        sqlite3_p_expr(p_parse, 53,
                            sqlite3_expr_dup(db, p_left as *const Expr, 0), p_limit);
                }
                sqlite3_expr_deferred_delete(p_parse, p_left);
                unsafe { (*unsafe { (*p_sel).p_limit }).p_left = p_limit };
            }
        } else {

            /// If there is no pre-existing limit add a limit of 1
            (p_limit = sqlite3_expr_int32(unsafe { (*p_parse).db }, 1));
            unsafe {
                (*p_sel).p_limit =
                    sqlite3_p_expr(p_parse, 149, p_limit, core::ptr::null_mut())
            };
        }
        unsafe { (*p_sel).i_limit = 0 };
        if unsafe { sqlite3_select(p_parse, p_sel, &mut dest) } != 0 {
            (*p_expr).op2 = (*p_expr).op;
            (*p_expr).op = 182 as u8;
            return 0;
        }
        (*p_expr).i_table = { r_reg = dest.i_sd_parm; r_reg };
        if addr_once != 0 { unsafe { sqlite3_vdbe_jump_here(v, addr_once) }; }

        /// Subroutine return
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            sqlite3_vdbe_add_op3(v, 69, (*p_expr).y.sub.reg_return,
                (*p_expr).y.sub.i_addr, 1)
        };
        sqlite3_clear_temp_reg_cache(unsafe { &mut *p_parse });
        return r_reg;
    }
}

///* If expression pExpr is of type TK_SELECT, generate code to evaluate
///* it. Return the register in which the result is stored (or, if the
///* sub-select returns more than one column, the first in an array
///* of registers in which the result is stored).
///*
///* If pExpr is not a TK_SELECT expression, return 0.
extern "C" fn expr_code_subselect(p_parse_1: *mut Parse, p_expr_1: *mut Expr)
    -> i32 {
    let mut reg: i32 = 0;
    if unsafe { (*p_expr_1).op } as i32 == 139 {
        reg = sqlite3_code_subselect(p_parse_1, unsafe { &mut *p_expr_1 });
    }
    return reg;
}

///* Return a pointer to a subexpression of pVector that is the i-th
///* column of the vector (numbered starting with 0).  The caller must
///* ensure that i is within range.
///*
///* If pVector is really a scalar (and "scalar" here includes subqueries
///* that return a single column!) then return pVector unmodified.
///*
///* pVector retains ownership of the returned subexpression.
///*
///* If the vector is a (SELECT ...) then the expression returned is
///* just the expression for the i-th term of the result set, and may
///* not be ready for evaluation because the table cursor has not yet
///* been positioned.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vector_field_subexpr(p_vector: *mut Expr, i: i32)
    -> *mut Expr {
    unsafe {
        { let _ = 0; };
        if sqlite3_expr_is_vector(p_vector as *const Expr) != 0 {
            { let _ = 0; };
            if unsafe { (*p_vector).op } as i32 == 139 ||
                    unsafe { (*p_vector).op2 } as i32 == 139 {
                { let _ = 0; };
                return unsafe {
                        (*(unsafe {
                                            (*unsafe {
                                                                (*unsafe { (*p_vector).x.p_select }).p_e_list
                                                            }).a.as_ptr()
                                        } as *mut ExprListItem).offset(i as isize)).p_expr
                    };
            } else {
                { let _ = 0; };
                return unsafe {
                        (*(unsafe { (*unsafe { (*p_vector).x.p_list }).a.as_ptr() }
                                        as *mut ExprListItem).offset(i as isize)).p_expr
                    };
            }
        }
        return p_vector;
    }
}

///* pExpr is a TK_FUNCTION node.  Try to determine whether or not the
///* function is a constant function.  A function is constant if all of
///* the following are true:
///*
///*    (1)  It is a scalar function (not an aggregate or window function)
///*    (2)  It has either the SQLITE_FUNC_CONSTANT or SQLITE_FUNC_SLOCHNG
///*         property.
///*    (3)  All of its arguments are constants
///*
///* This routine sets pWalker->eCode to 0 if pExpr is not a constant.
///* It makes no changes to pWalker->eCode if pExpr is constant.  In
///* every case, it returns WRC_Abort.
///*
///* Called as a service subroutine from exprNodeIsConstant().
#[allow(unused_doc_comments)]
extern "C" fn expr_node_is_constant_function(p_walker_1: *mut Walker,
    p_expr_1: &Expr) -> i32 {
    unsafe {
        let mut n: i32 = 0;
        /// Number of arguments
        let mut p_list: *mut ExprList = core::ptr::null_mut();
        /// List of arguments
        let mut p_def: *const FuncDef = core::ptr::null();
        /// The function
        let mut db: *mut Sqlite3 = core::ptr::null_mut();

        /// The database
        { let _ = 0; };
        if (*p_expr_1).flags & 65536 as u32 != 0 as u32 ||
                { p_list = (*p_expr_1).x.p_list; p_list } ==
                    core::ptr::null_mut() {
            n = 0;
        } else {
            n = unsafe { (*p_list).n_expr };
            unsafe { sqlite3_walk_expr_list(p_walker_1, p_list) };
            if unsafe { (*p_walker_1).e_code } as i32 == 0 { return 2; }
        }
        db = unsafe { (*unsafe { (*p_walker_1).p_parse }).db };
        p_def =
            unsafe {
                sqlite3_find_function(db, (*p_expr_1).u.z_token as *const i8,
                    n, unsafe { (*db).enc }, 0 as u8)
            };
        if p_def == core::ptr::null_mut() ||
                        unsafe { (*p_def).x_finalize.is_some() } ||
                    unsafe { (*p_def).func_flags } & (2048 | 8192) as u32 ==
                        0 as u32 || (*p_expr_1).flags & 16777216 as u32 != 0 as u32
            {
            unsafe { (*p_walker_1).e_code = 0 as u16 };
            return 2;
        }
        return 1;
    }
}

///* Check the input string to see if it is "true" or "false" (in any case).
///*
///*       If the string is....           Return
///*         "true"                         EP_IsTrue
///*         "false"                        EP_IsFalse
///*         anything else                  0
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_is_true_or_false(z_in: *const i8) -> u32 {
    if unsafe {
                sqlite3_str_i_cmp(z_in,
                    c"true".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return 268435456 as u32;
    }
    if unsafe {
                sqlite3_str_i_cmp(z_in,
                    c"false".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return 536870912 as u32;
    }
    return 0 as u32;
}

///* If the input expression is an ID with the name "true" or "false"
///* then convert it into an TK_TRUEFALSE term.  Return non-zero if
///* the conversion happened, and zero if the expression is unaltered.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_id_to_true_false(p_expr: &mut Expr) -> i32 {
    unsafe {
        let mut v: u32 = 0 as u32;
        { let _ = 0; };
        if !((*p_expr).flags & (67108864 | 2048) as u32 != 0 as u32) as i32 !=
                    0 &&
                {
                        v =
                            sqlite3_is_true_or_false((*p_expr).u.z_token as *const i8);
                        v
                    } != 0 as u32 {
            (*p_expr).op = 171 as u8;
            (*p_expr).flags |= v as u32;
            return 1;
        }
        return 0;
    }
}

///* These routines are Walker callbacks used to check expressions to
///* see if they are "constant" for some definition of constant.  The
///* Walker.eCode value determines the type of "constant" we are looking
///* for.
///*
///* These callback routines are used to implement the following:
///*
///*     sqlite3ExprIsConstant()                  pWalker->eCode==1
///*     sqlite3ExprIsConstantNotJoin()           pWalker->eCode==2
///*     sqlite3ExprIsTableConstant()             pWalker->eCode==3
///*     sqlite3ExprIsConstantOrFunction()        pWalker->eCode==4 or 5
///*
///* In all cases, the callbacks set Walker.eCode=0 and abort if the expression
///* is found to not be a constant.
///*
///* The sqlite3ExprIsConstantOrFunction() is used for evaluating DEFAULT
///* expressions in a CREATE TABLE statement.  The Walker.eCode value is 5
///* when parsing an existing schema out of the sqlite_schema table and 4
///* when processing a new CREATE TABLE statement.  A bound parameter raises
///* an error for new statements, but is silently converted
///* to NULL for existing schemas.  This allows sqlite_schema tables that
///* contain a bound parameter because they were generated by older versions
///* of SQLite to be parsed by newer versions of SQLite without raising a
///* malformed schema error.
#[allow(unused_doc_comments)]
extern "C" fn expr_node_is_constant(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        { let _ = 0; };
        if unsafe { (*p_walker_1).e_code } as i32 == 2 &&
                unsafe { (*p_expr_1).flags } & 1 as u32 != 0 as u32 {
            unsafe { (*p_walker_1).e_code = 0 as u16 };
            return 2;
        }
        '__s18:
            {
            match unsafe { (*p_expr_1).op } {
                172 => {
                    if (unsafe { (*p_walker_1).e_code } as i32 >= 4 ||
                                unsafe { (*p_expr_1).flags } & 1048576 as u32 != 0 as u32)
                            &&
                            !(unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                            0 as u32) as i32 != 0 {
                        if unsafe { (*p_walker_1).e_code } as i32 == 5 {
                            unsafe { (*p_expr_1).flags |= 1073741824 as u32 };
                        }
                        return 0;
                    } else if !(unsafe { (*p_walker_1).p_parse }).is_null() {
                        return expr_node_is_constant_function(p_walker_1,
                                unsafe { &*p_expr_1 });
                    } else {
                        unsafe { (*p_walker_1).e_code = 0 as u16 };
                        return 2;
                    }
                    if sqlite3_expr_id_to_true_false(unsafe { &mut *p_expr_1 })
                            != 0 {
                        return 1;
                    }
                    if unsafe { (*p_expr_1).flags } & 32 as u32 != 0 as u32 &&
                            unsafe { (*p_walker_1).e_code } as i32 != 2 {
                        return 0;
                    }
                    if unsafe { (*p_walker_1).e_code } as i32 == 3 &&
                            unsafe { (*p_expr_1).i_table } ==
                                unsafe { (*p_walker_1).u.i_cur } {
                        return 0;
                    }
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                60 => {
                    if sqlite3_expr_id_to_true_false(unsafe { &mut *p_expr_1 })
                            != 0 {
                        return 1;
                    }
                    if unsafe { (*p_expr_1).flags } & 32 as u32 != 0 as u32 &&
                            unsafe { (*p_walker_1).e_code } as i32 != 2 {
                        return 0;
                    }
                    if unsafe { (*p_walker_1).e_code } as i32 == 3 &&
                            unsafe { (*p_expr_1).i_table } ==
                                unsafe { (*p_walker_1).u.i_cur } {
                        return 0;
                    }
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                168 => {
                    if unsafe { (*p_expr_1).flags } & 32 as u32 != 0 as u32 &&
                            unsafe { (*p_walker_1).e_code } as i32 != 2 {
                        return 0;
                    }
                    if unsafe { (*p_walker_1).e_code } as i32 == 3 &&
                            unsafe { (*p_expr_1).i_table } ==
                                unsafe { (*p_walker_1).u.i_cur } {
                        return 0;
                    }
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                169 => {
                    if unsafe { (*p_expr_1).flags } & 32 as u32 != 0 as u32 &&
                            unsafe { (*p_walker_1).e_code } as i32 != 2 {
                        return 0;
                    }
                    if unsafe { (*p_walker_1).e_code } as i32 == 3 &&
                            unsafe { (*p_expr_1).i_table } ==
                                unsafe { (*p_walker_1).u.i_cur } {
                        return 0;
                    }
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                170 => {
                    if unsafe { (*p_expr_1).flags } & 32 as u32 != 0 as u32 &&
                            unsafe { (*p_walker_1).e_code } as i32 != 2 {
                        return 0;
                    }
                    if unsafe { (*p_walker_1).e_code } as i32 == 3 &&
                            unsafe { (*p_expr_1).i_table } ==
                                unsafe { (*p_walker_1).u.i_cur } {
                        return 0;
                    }
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                179 => {
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                176 => {
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                142 => {
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                72 => {
                    unsafe { (*p_walker_1).e_code = 0 as u16 };
                    return 2;
                }
                157 => {
                    if unsafe { (*p_walker_1).e_code } as i32 == 5 {

                        /// Silently convert bound parameters that appear inside of CREATE
                        ///* statements into a NULL when parsing the CREATE statement text out
                        ///* of the sqlite_schema table
                        unsafe { (*p_expr_1).op = 122 as u8 };
                    } else if unsafe { (*p_walker_1).e_code } as i32 == 4 {

                        /// A bound parameter in a CREATE statement that originates from
                        ///* sqlite3_prepare() causes an error
                        unsafe { (*p_walker_1).e_code = 0 as u16 };
                        return 2;
                    }
                    return 0;
                }
                _ => { return 0; }
            }
        }
    }
}

extern "C" fn expr_is_const(p_parse_1: *mut Parse, p: *mut Expr,
    init_flag_1: i32) -> i32 {
    let mut w: Walker = unsafe { core::mem::zeroed() };
    w.e_code = init_flag_1 as u16;
    w.p_parse = p_parse_1;
    w.x_expr_callback = Some(expr_node_is_constant);
    w.x_select_callback = Some(sqlite3_select_walk_fail);
    unsafe { sqlite3_walk_expr(&mut w, p) };
    return w.e_code as i32;
}

///* Walk an expression tree.  Return non-zero if
///*
///*   (1) the expression is constant, and
///*   (2) the expression does not originate in the ON or USING clause
///*       of a LEFT JOIN, and
///*   (3) the expression does not contain any EP_FixedCol TK_COLUMN
///*       operands created by the constant propagation optimization.
///*
///* When this routine returns true, it indicates that the expression
///* can be added to the pParse->pConstExpr list and evaluated once when
///* the prepared statement starts up.  See sqlite3ExprCodeRunJustOnce().
extern "C" fn sqlite3_expr_is_constant_not_join(p_parse_1: *mut Parse,
    p: *mut Expr) -> i32 {
    return expr_is_const(p_parse_1, p, 2);
}

///* Generate code that will evaluate expression pExpr just one time
///* per prepared statement execution.
///*
///* If the expression uses functions (that might throw an exception) then
///* guard them with an OP_Once opcode to ensure that the code is only executed
///* once. If no functions are involved, then factor the code out and put it at
///* the end of the prepared statement in the initialization section.
///*
///* If regDest>0 then the result is always stored in that register and the
///* result is not reusable.  If regDest<0 then this routine is free to
///* store the value wherever it wants.  The register where the expression
///* is stored is returned.  When regDest<0, two identical expressions might
///* code to the same register, if they do not contain function calls and hence
///* are factored out into the initialization section at the end of the
///* prepared statement.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_run_just_once(p_parse: *mut Parse,
    mut p_expr: *mut Expr, mut reg_dest: i32) -> i32 {
    unsafe {
        let mut p: *mut ExprList = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        p = unsafe { (*p_parse).p_const_expr };
        if reg_dest < 0 && !(p).is_null() {
            let mut p_item: *const ExprListItem = core::ptr::null();
            let mut i: i32 = 0;
            {
                {
                    p_item = unsafe { (*p).a.as_ptr() } as *mut ExprListItem;
                    i = unsafe { (*p).n_expr }
                };
                '__b19: loop {
                    if !(i > 0) { break '__b19; }
                    '__c19: loop {
                        if unsafe { (*p_item).fg.reusable() } != 0 &&
                                sqlite3_expr_compare(core::ptr::null(),
                                        unsafe { (*p_item).p_expr } as *const Expr,
                                        p_expr as *const Expr, -1) == 0 {
                            return unsafe { (*p_item).u.i_const_expr_reg };
                        }
                        break '__c19;
                    }
                    {
                        {
                            let __p = &mut p_item;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t }
                    };
                }
            }
        }
        p_expr =
            sqlite3_expr_dup(unsafe { (*p_parse).db }, p_expr as *const Expr,
                0);
        if p_expr != core::ptr::null_mut() &&
                unsafe { (*p_expr).flags } & 8 as u32 != 0 as u32 {
            let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
            let mut addr: i32 = 0;
            { let _ = 0; };
            addr = unsafe { sqlite3_vdbe_add_op0(v, 15) };
            unsafe { (*p_parse).set_ok_const_factor(0 as Bft as u32) };
            if (unsafe { (*unsafe { (*p_parse).db }).malloc_failed } == 0) as
                        i32 != 0 {
                if reg_dest < 0 {
                    reg_dest =
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            *__p += 1;
                            *__p
                        };
                }
                sqlite3_expr_code(p_parse, p_expr, reg_dest);
            }
            unsafe { (*p_parse).set_ok_const_factor(1 as Bft as u32) };
            sqlite3_expr_delete(unsafe { (*p_parse).db }, p_expr);
            unsafe { sqlite3_vdbe_jump_here(v, addr) };
        } else {
            p = sqlite3_expr_list_append(unsafe { &*p_parse }, p, p_expr);
            if !(p).is_null() {
                let p_item_1: *mut ExprListItem =
                    unsafe {
                        &mut *(unsafe { (*p).a.as_ptr() } as
                                        *mut ExprListItem).offset((unsafe { (*p).n_expr } - 1) as
                                        isize)
                    };
                unsafe {
                    (*p_item_1).fg.set_reusable((reg_dest < 0) as u32 as u32)
                };
                if reg_dest < 0 {
                    reg_dest =
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            *__p += 1;
                            *__p
                        };
                }
                unsafe { (*p_item_1).u.i_const_expr_reg = reg_dest };
            }
            unsafe { (*p_parse).p_const_expr = p };
        }
        return reg_dest;
    }
}

///* Generate code to evaluate an expression and store the results
///* into a register.  Return the register number where the results
///* are stored.
///*
///* If the register is a temporary register that can be deallocated,
///* then write its number into *pReg.  If the result register is not
///* a temporary, then set *pReg to zero.
///*
///* If pExpr is a constant, then this routine might generate this
///* code to fill the register in the initialization section of the
///* VDBE program, in order to factor it out of the evaluation loop.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_temp(p_parse: *mut Parse,
    mut p_expr: *mut Expr, p_reg: &mut i32) -> i32 {
    let mut r2: i32 = 0;
    p_expr = sqlite3_expr_skip_collate_and_likely(p_expr);
    if unsafe { (*p_parse).ok_const_factor() } != 0 &&
                    p_expr != core::ptr::null_mut() &&
                unsafe { (*p_expr).op } as i32 != 176 &&
            sqlite3_expr_is_constant_not_join(p_parse, p_expr) != 0 {
        *p_reg = 0;
        r2 = sqlite3_expr_code_run_just_once(p_parse, p_expr, -1);
    } else {
        let r1: i32 = sqlite3_get_temp_reg(unsafe { &mut *p_parse });
        r2 = sqlite3_expr_code_target(p_parse, p_expr, r1);
        if r2 == r1 {
            *p_reg = r1;
        } else {
            sqlite3_release_temp_reg(unsafe { &mut *p_parse }, r1);
            *p_reg = 0;
        }
    }
    return r2;
}

///* Argument pVector points to a vector expression - either a TK_VECTOR
///* or TK_SELECT that returns more than one column. This function returns
///* the register number of a register that contains the value of
///* element iField of the vector.
///*
///* If pVector is a TK_SELECT expression, then code for it must have
///* already been generated using the exprCodeSubselect() routine. In this
///* case parameter regSelect should be the first in an array of registers
///* containing the results of the sub-select.
///*
///* If pVector is of type TK_VECTOR, then code for the requested field
///* is generated. In this case (*pRegFree) may be set to the number of
///* a temporary register to be freed by the caller before returning.
///*
///* Before returning, output parameter (*ppExpr) is set to point to the
///* Expr object corresponding to element iElem of the vector.
extern "C" fn expr_vector_register(p_parse_1: *mut Parse,
    p_vector_1: *mut Expr, i_field_1: i32, reg_select_1: i32,
    pp_expr_1: &mut *mut Expr, p_reg_free_1: *mut i32) -> i32 {
    unsafe {
        let op: u8 = unsafe { (*p_vector_1).op };
        { let _ = 0; };
        if op as i32 == 176 {
            *pp_expr_1 = sqlite3_vector_field_subexpr(p_vector_1, i_field_1);
            return unsafe { (*p_vector_1).i_table } + i_field_1;
        }
        if op as i32 == 139 {
            { let _ = 0; };
            *pp_expr_1 =
                unsafe {
                    (*(unsafe {
                                        (*unsafe {
                                                            (*unsafe { (*p_vector_1).x.p_select }).p_e_list
                                                        }).a.as_ptr()
                                    } as *mut ExprListItem).offset(i_field_1 as isize)).p_expr
                };
            return reg_select_1 + i_field_1;
        }
        if op as i32 == 177 {
            { let _ = 0; };
            *pp_expr_1 =
                unsafe {
                    (*(unsafe {
                                        (*unsafe { (*p_vector_1).x.p_list }).a.as_ptr()
                                    } as *mut ExprListItem).offset(i_field_1 as isize)).p_expr
                };
            return sqlite3_expr_code_temp(p_parse_1, *pp_expr_1,
                    unsafe { &mut *p_reg_free_1 });
        }
        return 0;
    }
}

///* Return the collation sequence for the expression pExpr. If
///* there is no defined collating sequence, return NULL.
///*
///* See also: sqlite3ExprNNCollSeq()
///*
///* The sqlite3ExprNNCollSeq() works the same exact that it returns the
///* default collation if pExpr has no defined collation.
///*
///* The collating sequence might be determined by a COLLATE operator
///* or by the presence of a column with a defined collating sequence.
///* COLLATE operators take first precedence.  Left operands take
///* precedence over right operands.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_coll_seq(p_parse: *mut Parse,
    p_expr: *const Expr) -> *mut CollSeq {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        let mut p_coll: *mut CollSeq = core::ptr::null_mut();
        let mut p: *const Expr = p_expr;
        while !(p).is_null() {
            let mut op: i32 = unsafe { (*p).op } as i32;
            if op == 176 { op = unsafe { (*p).op2 } as i32; }
            if op == 170 && unsafe { (*p).y.p_tab } != core::ptr::null_mut()
                        || op == 168 || op == 78 {
                let mut j: i32 = 0;
                { let _ = 0; };
                { let _ = 0; };
                if { j = unsafe { (*p).i_column } as i32; j } >= 0 {
                    let z_coll: *const i8 =
                        unsafe {
                            sqlite3_column_coll(unsafe {
                                    &mut *unsafe {
                                                (*unsafe { (*p).y.p_tab }).a_col.offset(j as isize)
                                            }
                                })
                        };
                    p_coll =
                        unsafe {
                            sqlite3_find_coll_seq(db, unsafe { (*db).enc }, z_coll, 0)
                        };
                }
                break;
            }
            if op == 36 || op == 173 {
                p = unsafe { (*p).p_left } as *const Expr;
                continue;
            }
            if op == 177 || op == 172 && unsafe { (*p).aff_expr } as i32 == 88
                {
                { let _ = 0; };
                p =
                    unsafe {
                            (*(unsafe { (*unsafe { (*p).x.p_list }).a.as_ptr() } as
                                            *mut ExprListItem).offset(0 as isize)).p_expr
                        } as *const Expr;
                continue;
            }
            if op == 114 {
                { let _ = 0; };
                p_coll =
                    unsafe {
                        sqlite3_get_coll_seq(p_parse, unsafe { (*db).enc },
                            core::ptr::null_mut(),
                            unsafe { (*p).u.z_token } as *const i8)
                    };
                break;
            }
            if unsafe { (*p).flags } & 512 as u32 != 0 {
                if !(unsafe { (*p).p_left }).is_null() &&
                        unsafe { (*unsafe { (*p).p_left }).flags } & 512 as u32 !=
                            0 as u32 {
                    p = unsafe { (*p).p_left } as *const Expr;
                } else {
                    let mut p_next: *const Expr =
                        unsafe { (*p).p_right } as *const Expr;

                    /// The Expr.x union is never used at the same time as Expr.pRight
                    { let _ = 0; };
                    if unsafe { (*p).flags } & 4096 as u32 == 0 as u32 &&
                                unsafe { (*p).x.p_list } != core::ptr::null_mut() &&
                            (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                        let mut i: i32 = 0;
                        {
                            i = 0;
                            '__b21: loop {
                                if !(i < unsafe { (*unsafe { (*p).x.p_list }).n_expr }) {
                                    break '__b21;
                                }
                                '__c21: loop {
                                    if unsafe {
                                                    (*unsafe {
                                                                    (*(unsafe { (*unsafe { (*p).x.p_list }).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).p_expr
                                                                }).flags
                                                } & 512 as u32 != 0 as u32 {
                                        p_next =
                                            unsafe {
                                                (*(unsafe { (*unsafe { (*p).x.p_list }).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                            };
                                        break '__b21;
                                    }
                                    break '__c21;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                    p = p_next as *const Expr;
                }
            } else { break; }
        }
        if unsafe { sqlite3_check_coll_seq(p_parse, p_coll) } != 0 {
            p_coll = core::ptr::null_mut();
        }
        return p_coll;
    }
}

///* Return a pointer to the collation sequence that should be used by
///* a binary comparison operator comparing pLeft and pRight.
///*
///* If the left hand expression has a collating sequence type, then it is
///* used. Otherwise the collation sequence for the right hand expression
///* is used, or the default (BINARY) if neither expression has a collating
///* type.
///*
///* Argument pRight (but not pLeft) may be a null pointer. In this case,
///* it is not considered.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_binary_compare_coll_seq(p_parse: *mut Parse,
    p_left: *const Expr, p_right: *const Expr) -> *mut CollSeq {
    let mut p_coll: *mut CollSeq = core::ptr::null_mut();
    { let _ = 0; };
    if unsafe { (*p_left).flags } & 512 as u32 != 0 {
        p_coll = sqlite3_expr_coll_seq(p_parse, p_left);
    } else if !(p_right).is_null() &&
            unsafe { (*p_right).flags } & 512 as u32 != 0 as u32 {
        p_coll = sqlite3_expr_coll_seq(p_parse, p_right);
    } else {
        p_coll = sqlite3_expr_coll_seq(p_parse, p_left);
        if (p_coll).is_null() as i32 != 0 {
            p_coll = sqlite3_expr_coll_seq(p_parse, p_right);
        }
    }
    return p_coll;
}

///* pExpr is an operand of a comparison operator.  aff2 is the
///* type affinity of the other operand.  This routine returns the
///* type affinity that should be used for the comparison operator.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_compare_affinity(p_expr: *const Expr, aff2: i8)
    -> i8 {
    let aff1: i8 = sqlite3_expr_affinity(p_expr);
    if aff1 as i32 > 64 && aff2 as i32 > 64 {
        if aff1 as i32 >= 67 || aff2 as i32 >= 67 {
            return 67 as i8;
        } else { return 65 as i8; }
    } else {

        /// One side is a column, the other is not. Use the columns affinity.
        { let _ = 0; };
        return (if aff1 as i32 <= 64 { aff2 as i32 } else { aff1 as i32 } |
                    64) as i8;
    }
}

///* Return the P5 value that should be used for a binary comparison
///* opcode (OP_Eq, OP_Ge etc.) used to compare pExpr1 and pExpr2.
extern "C" fn binary_compare_p5(p_expr1_1: *const Expr,
    p_expr2_1: *const Expr, jump_if_null_1: i32) -> u8 {
    let mut aff: u8 = sqlite3_expr_affinity(p_expr2_1) as i8 as u8;
    aff =
        (sqlite3_compare_affinity(p_expr1_1, aff as i8) as u8 as i32 |
                jump_if_null_1 as u8 as i32) as u8;
    return aff;
}

///* Generate code for a comparison operator.
extern "C" fn code_compare(p_parse_1: *mut Parse, p_left_1: *const Expr,
    p_right_1: *const Expr, opcode: i32, in1: i32, in2: i32, dest: i32,
    jump_if_null_1: i32, is_commuted_1: i32) -> i32 {
    let mut p5: i32 = 0;
    let mut addr: i32 = 0;
    let mut p4: *mut CollSeq = core::ptr::null_mut();
    if unsafe { (*p_parse_1).n_err } != 0 { return 0; }
    if is_commuted_1 != 0 {
        p4 =
            sqlite3_binary_compare_coll_seq(p_parse_1,
                p_right_1 as *const Expr, p_left_1 as *const Expr);
    } else {
        p4 =
            sqlite3_binary_compare_coll_seq(p_parse_1,
                p_left_1 as *const Expr, p_right_1 as *const Expr);
    }
    p5 =
        binary_compare_p5(p_left_1 as *const Expr, p_right_1 as *const Expr,
                jump_if_null_1) as i32;
    addr =
        unsafe {
            sqlite3_vdbe_add_op4(unsafe { (*p_parse_1).p_vdbe }, opcode, in2,
                dest, in1, p4 as *mut () as *const i8, -2)
        };
    unsafe {
        sqlite3_vdbe_change_p5(unsafe { (*p_parse_1).p_vdbe }, p5 as u16)
    };
    return addr;
}

///* Expression pExpr is a comparison between two vector values. Compute
///* the result of the comparison (1, 0, or NULL) and write that
///* result into register dest.
///*
///* The caller must satisfy the following preconditions:
///*
///*    if pExpr->op==TK_IS:      op==TK_EQ and p5==SQLITE_NULLEQ
///*    if pExpr->op==TK_ISNOT:   op==TK_NE and p5==SQLITE_NULLEQ
///*    otherwise:                op==pExpr->op and p5==0
extern "C" fn code_vector_compare(p_parse_1: *mut Parse, p_expr_1: &Expr,
    dest: i32, op: u8, p5: u8) -> () {
    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
    let p_left: *mut Expr = (*p_expr_1).p_left;
    let p_right: *mut Expr = (*p_expr_1).p_right;
    let n_left: i32 = sqlite3_expr_vector_size(unsafe { &*p_left });
    let mut i: i32 = 0;
    let mut reg_left: i32 = 0;
    let mut reg_right: i32 = 0;
    let mut opx: u8 = op;
    let mut addr_cmp: i32 = 0;
    let addr_done: i32 = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
    let is_commuted: i32 =
        ((*p_expr_1).flags & 1024 as u32 != 0 as u32) as i32;
    { let _ = 0; };
    if unsafe { (*p_parse_1).n_err } != 0 { return; }
    if n_left != sqlite3_expr_vector_size(unsafe { &*p_right }) {
        unsafe {
            sqlite3_error_msg(p_parse_1,
                c"row value misused".as_ptr() as *mut i8 as *const i8)
        };
        return;
    }
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if op as i32 == 56 { opx = 57 as u8; }
    if op as i32 == 58 { opx = 55 as u8; }
    if op as i32 == 53 { opx = 54 as u8; }
    reg_left = expr_code_subselect(p_parse_1, p_left);
    reg_right = expr_code_subselect(p_parse_1, p_right);
    unsafe { sqlite3_vdbe_add_op2(v, 73, 1, dest) };
    {
        i = 0;
        '__b22: loop {
            if !(1 != 0) { break '__b22; }
            '__c22: loop {
                let mut reg_free1: i32 = 0;
                let mut reg_free2: i32 = 0;
                let mut p_l: *mut Expr = core::ptr::null_mut();
                let mut p_r: *mut Expr = core::ptr::null_mut();
                let mut r1: i32 = 0;
                let mut r2: i32 = 0;
                { let _ = 0; };
                if addr_cmp != 0 {
                    unsafe { sqlite3_vdbe_jump_here(v, addr_cmp) };
                }
                r1 =
                    expr_vector_register(p_parse_1, p_left, i, reg_left,
                        &mut p_l, &mut reg_free1);
                r2 =
                    expr_vector_register(p_parse_1, p_right, i, reg_right,
                        &mut p_r, &mut reg_free2);
                addr_cmp = unsafe { sqlite3_vdbe_current_addr(v) };
                code_compare(p_parse_1, p_l as *const Expr,
                    p_r as *const Expr, opx as i32, r1, r2, addr_done,
                    p5 as i32, is_commuted);
                sqlite3_release_temp_reg(unsafe { &mut *p_parse_1 },
                    reg_free1);
                sqlite3_release_temp_reg(unsafe { &mut *p_parse_1 },
                    reg_free2);
                if (opx as i32 == 57 || opx as i32 == 55) && i < n_left - 1 {
                    addr_cmp = unsafe { sqlite3_vdbe_add_op0(v, 59) };
                }
                if p5 as i32 == 128 {
                    unsafe { sqlite3_vdbe_add_op2(v, 73, 0, dest) };
                } else {
                    unsafe { sqlite3_vdbe_add_op3(v, 94, r1, dest, r2) };
                }
                if i == n_left - 1 { break '__b22; }
                if opx as i32 == 54 {
                    unsafe { sqlite3_vdbe_add_op2(v, 52, dest, addr_done) };
                } else {
                    { let _ = 0; };
                    unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr_done) };
                    if i == n_left - 2 { opx = op; }
                }
                break '__c22;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { sqlite3_vdbe_jump_here(v, addr_cmp) };
    unsafe { sqlite3_vdbe_resolve_label(v, addr_done) };
    if op as i32 == 53 { unsafe { sqlite3_vdbe_add_op2(v, 19, dest, dest) }; }
}

///* Return true if it might be advantageous to compute the right operand
///* of expression pExpr first, before the left operand.
///*
///* Normally the left operand is computed before the right operand.  But if
///* the left operand contains a subquery and the right does not, then it
///* might be more efficient to compute the right operand first.
extern "C" fn expr_eval_rhs_first(p_expr_1: &Expr) -> i32 {
    if unsafe { (*(*p_expr_1).p_left).flags } & 4194304 as u32 != 0 as u32 &&
            !(unsafe { (*(*p_expr_1).p_right).flags } & 4194304 as u32 !=
                            0 as u32) as i32 != 0 {
        return 1;
    } else { return 0; }
}

///* Return FALSE if there is no chance that the expression can be NULL.
///*
///* If the expression might be NULL or if the expression is too complex
///* to tell return TRUE. 
///*
///* This routine is used as an optimization, to skip OP_IsNull opcodes
///* when we know that a value cannot be NULL.  Hence, a false positive
///* (returning TRUE when in fact the expression can never be NULL) might
///* be a small performance hit but is otherwise harmless.  On the other
///* hand, a false negative (returning FALSE when the result could be NULL)
///* will likely result in an incorrect answer.  So when in doubt, return
///* TRUE.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_can_be_null(mut p: *const Expr) -> i32 {
    unsafe {
        let mut op: u8 = 0 as u8;
        { let _ = 0; };
        while unsafe { (*p).op } as i32 == 173 ||
                unsafe { (*p).op } as i32 == 174 {
            p = unsafe { (*p).p_left } as *const Expr;
            { let _ = 0; };
        }
        op = unsafe { (*p).op } as u8;
        if op as i32 == 176 { op = unsafe { (*p).op2 } as u8; }
        '__s24:
            {
            match op {
                156 => {
                    return 0;
                    { let _ = 0; };
                    return (unsafe { (*p).flags } & 2097152 as u32 != 0 as u32
                                    || unsafe { (*p).y.p_tab } == core::ptr::null_mut() ||
                                unsafe { (*p).i_column } as i32 >= 0 &&
                                            unsafe { (*unsafe { (*p).y.p_tab }).a_col } !=
                                                core::ptr::null_mut() &&
                                        (unsafe { (*p).i_column } as i32) <
                                            unsafe { (*unsafe { (*p).y.p_tab }).n_col } as i32 &&
                                    unsafe {
                                                (*unsafe {
                                                            (*unsafe {
                                                                                (*p).y.p_tab
                                                                            }).a_col.offset(unsafe { (*p).i_column } as isize)
                                                        }).not_null()
                                            } as i32 == 0) as i32;
                    return 1;
                }
                118 => {
                    return 0;
                    { let _ = 0; };
                    return (unsafe { (*p).flags } & 2097152 as u32 != 0 as u32
                                    || unsafe { (*p).y.p_tab } == core::ptr::null_mut() ||
                                unsafe { (*p).i_column } as i32 >= 0 &&
                                            unsafe { (*unsafe { (*p).y.p_tab }).a_col } !=
                                                core::ptr::null_mut() &&
                                        (unsafe { (*p).i_column } as i32) <
                                            unsafe { (*unsafe { (*p).y.p_tab }).n_col } as i32 &&
                                    unsafe {
                                                (*unsafe {
                                                            (*unsafe {
                                                                                (*p).y.p_tab
                                                                            }).a_col.offset(unsafe { (*p).i_column } as isize)
                                                        }).not_null()
                                            } as i32 == 0) as i32;
                    return 1;
                }
                154 => {
                    return 0;
                    { let _ = 0; };
                    return (unsafe { (*p).flags } & 2097152 as u32 != 0 as u32
                                    || unsafe { (*p).y.p_tab } == core::ptr::null_mut() ||
                                unsafe { (*p).i_column } as i32 >= 0 &&
                                            unsafe { (*unsafe { (*p).y.p_tab }).a_col } !=
                                                core::ptr::null_mut() &&
                                        (unsafe { (*p).i_column } as i32) <
                                            unsafe { (*unsafe { (*p).y.p_tab }).n_col } as i32 &&
                                    unsafe {
                                                (*unsafe {
                                                            (*unsafe {
                                                                                (*p).y.p_tab
                                                                            }).a_col.offset(unsafe { (*p).i_column } as isize)
                                                        }).not_null()
                                            } as i32 == 0) as i32;
                    return 1;
                }
                155 => {
                    return 0;
                    { let _ = 0; };
                    return (unsafe { (*p).flags } & 2097152 as u32 != 0 as u32
                                    || unsafe { (*p).y.p_tab } == core::ptr::null_mut() ||
                                unsafe { (*p).i_column } as i32 >= 0 &&
                                            unsafe { (*unsafe { (*p).y.p_tab }).a_col } !=
                                                core::ptr::null_mut() &&
                                        (unsafe { (*p).i_column } as i32) <
                                            unsafe { (*unsafe { (*p).y.p_tab }).n_col } as i32 &&
                                    unsafe {
                                                (*unsafe {
                                                            (*unsafe {
                                                                                (*p).y.p_tab
                                                                            }).a_col.offset(unsafe { (*p).i_column } as isize)
                                                        }).not_null()
                                            } as i32 == 0) as i32;
                    return 1;
                }
                168 => {
                    { let _ = 0; };
                    return (unsafe { (*p).flags } & 2097152 as u32 != 0 as u32
                                    || unsafe { (*p).y.p_tab } == core::ptr::null_mut() ||
                                unsafe { (*p).i_column } as i32 >= 0 &&
                                            unsafe { (*unsafe { (*p).y.p_tab }).a_col } !=
                                                core::ptr::null_mut() &&
                                        (unsafe { (*p).i_column } as i32) <
                                            unsafe { (*unsafe { (*p).y.p_tab }).n_col } as i32 &&
                                    unsafe {
                                                (*unsafe {
                                                            (*unsafe {
                                                                                (*p).y.p_tab
                                                                            }).a_col.offset(unsafe { (*p).i_column } as isize)
                                                        }).not_null()
                                            } as i32 == 0) as i32;
                    return 1;
                }
                _ => { return 1; }
            }
        }
    }
}

///* Compute the two operands of a binary operator.
///*
///* If either operand contains a subquery, then the code strives to
///* compute the operand containing the subquery second.  If the other
///* operand evalutes to NULL, then a jump is made.  The address of the
///* IsNull operand that does this jump is returned.  The caller can use
///* this to optimize the computation so as to avoid doing the potentially
///* expensive subquery.
///*
///* If no optimization opportunities exist, return 0.
#[allow(unused_doc_comments)]
extern "C" fn expr_compute_operands(p_parse_1: *mut Parse,
    p_expr_1: *mut Expr, p_r1_1: &mut i32, p_r2_1: &mut i32,
    p_free1_1: *mut i32, p_free2_1: *mut i32) -> i32 {
    let mut addr_is_null: i32 = 0;
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
    { let _ = 0; };
    if expr_eval_rhs_first(unsafe { &*p_expr_1 }) != 0 &&
            sqlite3_expr_can_be_null(unsafe { (*p_expr_1).p_right } as
                        *const Expr) != 0 {
        r2 =
            sqlite3_expr_code_temp(p_parse_1, unsafe { (*p_expr_1).p_right },
                unsafe { &mut *p_free2_1 });
        addr_is_null = unsafe { sqlite3_vdbe_add_op1(v, 51, r2) };
    } else {
        r2 = 0;

        /// Silence a false-positive uninit-var warning in MSVC
        (addr_is_null = 0);
    }
    r1 =
        sqlite3_expr_code_temp(p_parse_1, unsafe { (*p_expr_1).p_left },
            unsafe { &mut *p_free1_1 });
    if addr_is_null == 0 {
        if unsafe { (*unsafe { (*p_expr_1).p_right }).flags } & 4194304 as u32
                    != 0 as u32 &&
                sqlite3_expr_can_be_null(unsafe { (*p_expr_1).p_left } as
                            *const Expr) != 0 {
            addr_is_null = unsafe { sqlite3_vdbe_add_op1(v, 51, r1) };
        }
        r2 =
            sqlite3_expr_code_temp(p_parse_1, unsafe { (*p_expr_1).p_right },
                unsafe { &mut *p_free2_1 });
    }
    *p_r1_1 = r1;
    *p_r2_1 = r2;
    return addr_is_null;
}

///* Generate code that evaluates an AND or OR operator leaving a
///* boolean result in a register.  pExpr is the AND/OR expression.
///* Store the result in the "target" register.  Use short-circuit
///* evaluation to avoid computing both operands, if possible.
///*
///* The code generated might require the use of a temporary register.
///* If it does, then write the number of that temporary register
///* into *pTmpReg.  If not, leave *pTmpReg unchanged.
#[allow(unused_doc_comments)]
extern "C" fn expr_code_target_and_or(p_parse_1: *mut Parse,
    p_expr_1: *mut Expr, target: i32, p_tmp_reg_1: *mut i32) -> i32 {
    let mut op: i32 = 0;
    /// The opcode.  TK_AND or TK_OR
    let mut skip_op: i32 = 0;
    /// Opcode for the branch that skips one operand
    let mut addr_skip: i32 = 0;
    /// Branch instruction that skips one of the operands
    let mut reg_ss: i32 = 0;
    /// Register holding computed operand when other omitted
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    /// Registers for left and right operands, respectively
    let mut p_alt: *mut Expr = core::ptr::null_mut();
    /// Alternative, simplified expression
    let mut v: *mut Vdbe = core::ptr::null_mut();

    /// statement being coded
    { let _ = 0; };
    op = unsafe { (*p_expr_1).op } as i32;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    v = unsafe { (*p_parse_1).p_vdbe };
    p_alt = sqlite3_expr_simplified_and_or(p_expr_1);
    if p_alt != p_expr_1 {
        r1 = sqlite3_expr_code_target(p_parse_1, p_alt, target);
        unsafe { sqlite3_vdbe_add_op3(v, 44, r1, r1, target) };
        return target;
    }
    skip_op = if op == 44 { 17 } else { 16 };
    if expr_eval_rhs_first(unsafe { &*p_expr_1 }) != 0 {

        /// Compute the right operand first.  Skip the computation of the left
        ///* operand if the right operand fully determines the result
        (r2 =
            {
                reg_ss =
                    sqlite3_expr_code_target(p_parse_1,
                        unsafe { (*p_expr_1).p_right }, target);
                reg_ss
            });
        addr_skip = unsafe { sqlite3_vdbe_add_op1(v, skip_op, r2) };
        r1 =
            sqlite3_expr_code_temp(p_parse_1, unsafe { (*p_expr_1).p_left },
                unsafe { &mut *p_tmp_reg_1 });
    } else {

        /// Compute the left operand first
        (r1 =
            sqlite3_expr_code_target(p_parse_1, unsafe { (*p_expr_1).p_left },
                target));
        if unsafe { (*unsafe { (*p_expr_1).p_right }).flags } & 4194304 as u32
                != 0 as u32 {

            /// Skip over the computation of the right operand if the right
            ///* operand is a subquery and the left operand completely determines
            ///* the result
            (reg_ss = r1);
            addr_skip = unsafe { sqlite3_vdbe_add_op1(v, skip_op, r1) };
        } else { addr_skip = { reg_ss = 0; reg_ss }; }
        r2 =
            sqlite3_expr_code_temp(p_parse_1, unsafe { (*p_expr_1).p_right },
                unsafe { &mut *p_tmp_reg_1 });
    }
    unsafe { sqlite3_vdbe_add_op3(v, op, r2, r1, target) };
    if addr_skip != 0 {
        unsafe {
            sqlite3_vdbe_add_op2(v, 9, 0,
                unsafe { sqlite3_vdbe_current_addr(v) } + 2)
        };
        unsafe { sqlite3_vdbe_jump_here(v, addr_skip) };
        unsafe { sqlite3_vdbe_add_op3(v, 43, reg_ss, reg_ss, target) };
    }
    return target;
}

///* If the last opcode is a OP_Copy, then set the do-not-merge flag (p5)
///* so that a subsequent copy will not be merged into this one.
extern "C" fn set_do_not_merge_flag_on_copy(v: *mut Vdbe) -> () {
    if unsafe { (*unsafe { sqlite3_vdbe_get_last_op(v) }).opcode } as i32 ==
            82 {
        unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
    }
}

///* Return non-zero if Expr p can only be true if pNN is not NULL.
///*
///* Or if seenNot is true, return non-zero if Expr p can only be
///* non-NULL if pNN is not NULL
extern "C" fn expr_implies_not_null(p_parse_1: *const Parse, p: *const Expr,
    p_nn_1: *const Expr, i_tab_1: i32, mut seen_not_1: i32) -> i32 {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if sqlite3_expr_compare(p_parse_1, p, p_nn_1, i_tab_1) == 0 {
            return (unsafe { (*p_nn_1).op } as i32 != 122) as i32;
        }
        '__s25:
            {
            match unsafe { (*p).op } {
                50 => {
                    {
                        if seen_not_1 != 0 &&
                                unsafe { (*p).flags } & 4096 as u32 != 0 as u32 {
                            return 0;
                        }
                        { let _ = 0; };
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        let mut p_list: *const ExprList = core::ptr::null();
                        { let _ = 0; };
                        p_list = unsafe { (*p).x.p_list };
                        { let _ = 0; };
                        { let _ = 0; };
                        if seen_not_1 != 0 { return 0; }
                        if expr_implies_not_null(p_parse_1,
                                        unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                            } as *const Expr, p_nn_1, i_tab_1, 1) != 0 ||
                                expr_implies_not_null(p_parse_1,
                                        unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(1 as isize)).p_expr
                                            } as *const Expr, p_nn_1, i_tab_1, 1) != 0 {
                            return 1;
                        }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                49 => {
                    {
                        let mut p_list: *const ExprList = core::ptr::null();
                        { let _ = 0; };
                        p_list = unsafe { (*p).x.p_list };
                        { let _ = 0; };
                        { let _ = 0; };
                        if seen_not_1 != 0 { return 0; }
                        if expr_implies_not_null(p_parse_1,
                                        unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                            } as *const Expr, p_nn_1, i_tab_1, 1) != 0 ||
                                expr_implies_not_null(p_parse_1,
                                        unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(1 as isize)).p_expr
                                            } as *const Expr, p_nn_1, i_tab_1, 1) != 0 {
                            return 1;
                        }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                54 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                53 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                57 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                56 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                55 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                58 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                107 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                108 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                104 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                105 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                106 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                112 => {
                    seen_not_1 = 1;
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                109 => {
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                111 => {
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                103 => {
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                110 => {
                    {
                        if expr_implies_not_null(p_parse_1,
                                    unsafe { (*p).p_right } as *const Expr, p_nn_1, i_tab_1,
                                    seen_not_1) != 0 {
                            return 1;
                        }
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                181 => {
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                114 => {
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                173 => {
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                174 => {
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1,
                                seen_not_1);
                    }
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                175 => {
                    {
                        if seen_not_1 != 0 { return 0; }
                        if unsafe { (*p).op2 } as i32 != 45 { return 0; }
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                115 => {
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                19 => {
                    {
                        return expr_implies_not_null(p_parse_1,
                                unsafe { (*p).p_left } as *const Expr, p_nn_1, i_tab_1, 1);
                    }
                }
                _ => {}
            }
        }
        return 0;
    }
}

///* If the expression p codes a constant integer that is small enough
///* to fit in a 32-bit integer, return 1 and put the value of the integer
///* in *pValue.  If the expression is not an integer or if it is too big
///* to fit in a signed 32-bit integer, return 0 and leave *pValue unchanged.
///*
///* If the pParse pointer is provided, then allow the expression p to be
///* a parameter (TK_VARIABLE) that is bound to an integer.
///* But if pParse is NULL, then p must be a pure integer literal.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_is_integer(p: *const Expr, p_value: *mut i32,
    p_parse: *mut Parse) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if p == core::ptr::null() { return 0; }

        /// Used to only happen following on OOM
        /// If an expression is an integer literal that fits in a signed 32-bit
        ///* integer, then the EP_IntValue flag will have already been set
        { let _ = 0; };
        if unsafe { (*p).flags } & 2048 as u32 != 0 {
            unsafe { *p_value = unsafe { (*p).u.i_value } as i32 };
            return 1;
        }
        '__s26:
            {
            match unsafe { (*p).op } {
                173 => {
                    {
                        rc =
                            sqlite3_expr_is_integer(unsafe { (*p).p_left } as
                                    *const Expr, p_value, core::ptr::null_mut());
                        break '__s26;
                    }
                    {
                        let mut v: i32 = 0;
                        if sqlite3_expr_is_integer(unsafe { (*p).p_left } as
                                        *const Expr, &mut v, core::ptr::null_mut()) != 0 {
                            { let _ = 0; };
                            unsafe { *p_value = -v };
                            rc = 1;
                        }
                        break '__s26;
                    }
                    {
                        let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
                        if p_parse == core::ptr::null_mut() { break '__s26; }
                        if unsafe { (*p_parse).p_vdbe } == core::ptr::null_mut() {
                            break '__s26;
                        }
                        if unsafe { (*unsafe { (*p_parse).db }).flags } &
                                    8388608 as u64 != 0 as u64 {
                            break '__s26;
                        }
                        unsafe {
                            sqlite3_vdbe_set_varmask(unsafe { (*p_parse).p_vdbe },
                                unsafe { (*p).i_column } as i32)
                        };
                        p_val =
                            unsafe {
                                sqlite3_vdbe_get_bound_value(unsafe {
                                        (*p_parse).p_reprepare
                                    }, unsafe { (*p).i_column } as i32, 65 as u8)
                            };
                        if !(p_val).is_null() {
                            if unsafe { sqlite3_value_type(p_val) } == 1 {
                                let vv: Sqlite3Int64 =
                                    unsafe { sqlite3_value_int64(p_val) };
                                if vv == vv & 2147483647 as Sqlite3Int64 {

                                    /// non-negative numbers only
                                    unsafe { *p_value = vv as i32 };

                                    /// non-negative numbers only
                                    (rc = 1);
                                }
                            }
                            unsafe { sqlite3ValueFree(p_val) };
                        }
                        break '__s26;
                    }
                }
                174 => {
                    {
                        let mut v: i32 = 0;
                        if sqlite3_expr_is_integer(unsafe { (*p).p_left } as
                                        *const Expr, &mut v, core::ptr::null_mut()) != 0 {
                            { let _ = 0; };
                            unsafe { *p_value = -v };
                            rc = 1;
                        }
                        break '__s26;
                    }
                    {
                        let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
                        if p_parse == core::ptr::null_mut() { break '__s26; }
                        if unsafe { (*p_parse).p_vdbe } == core::ptr::null_mut() {
                            break '__s26;
                        }
                        if unsafe { (*unsafe { (*p_parse).db }).flags } &
                                    8388608 as u64 != 0 as u64 {
                            break '__s26;
                        }
                        unsafe {
                            sqlite3_vdbe_set_varmask(unsafe { (*p_parse).p_vdbe },
                                unsafe { (*p).i_column } as i32)
                        };
                        p_val =
                            unsafe {
                                sqlite3_vdbe_get_bound_value(unsafe {
                                        (*p_parse).p_reprepare
                                    }, unsafe { (*p).i_column } as i32, 65 as u8)
                            };
                        if !(p_val).is_null() {
                            if unsafe { sqlite3_value_type(p_val) } == 1 {
                                let vv: Sqlite3Int64 =
                                    unsafe { sqlite3_value_int64(p_val) };
                                if vv == vv & 2147483647 as Sqlite3Int64 {

                                    /// non-negative numbers only
                                    unsafe { *p_value = vv as i32 };

                                    /// non-negative numbers only
                                    (rc = 1);
                                }
                            }
                            unsafe { sqlite3ValueFree(p_val) };
                        }
                        break '__s26;
                    }
                }
                157 => {
                    {
                        let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
                        if p_parse == core::ptr::null_mut() { break '__s26; }
                        if unsafe { (*p_parse).p_vdbe } == core::ptr::null_mut() {
                            break '__s26;
                        }
                        if unsafe { (*unsafe { (*p_parse).db }).flags } &
                                    8388608 as u64 != 0 as u64 {
                            break '__s26;
                        }
                        unsafe {
                            sqlite3_vdbe_set_varmask(unsafe { (*p_parse).p_vdbe },
                                unsafe { (*p).i_column } as i32)
                        };
                        p_val =
                            unsafe {
                                sqlite3_vdbe_get_bound_value(unsafe {
                                        (*p_parse).p_reprepare
                                    }, unsafe { (*p).i_column } as i32, 65 as u8)
                            };
                        if !(p_val).is_null() {
                            if unsafe { sqlite3_value_type(p_val) } == 1 {
                                let vv: Sqlite3Int64 =
                                    unsafe { sqlite3_value_int64(p_val) };
                                if vv == vv & 2147483647 as Sqlite3Int64 {

                                    /// non-negative numbers only
                                    unsafe { *p_value = vv as i32 };

                                    /// non-negative numbers only
                                    (rc = 1);
                                }
                            }
                            unsafe { sqlite3ValueFree(p_val) };
                        }
                        break '__s26;
                    }
                }
                _ => {}
            }
        }
        return rc;
    }
}

///* Return true if the boolean value of the expression is always either
///* FALSE or NULL.
extern "C" fn sqlite3_expr_is_not_true(p_expr_1: *const Expr) -> i32 {
    let mut v: i32 = 0;
    if unsafe { (*p_expr_1).op } as i32 == 122 { return 1; }
    if unsafe { (*p_expr_1).op } as i32 == 171 &&
            sqlite3_expr_truth_value(p_expr_1 as *const Expr) == 0 {
        return 1;
    }
    v = 1;
    if sqlite3_expr_is_integer(p_expr_1 as *const Expr, &mut v,
                    core::ptr::null_mut()) != 0 && v == 0 {
        return 1;
    }
    return 0;
}

///* Return true if the expression is one of the following:
///*
///*    CASE WHEN x THEN y END
///*    CASE WHEN x THEN y ELSE NULL END
///*    CASE WHEN x THEN y ELSE false END
///*    iif(x,y)
///*    iif(x,y,NULL)
///*    iif(x,y,false)
extern "C" fn sqlite3_expr_is_iif(db: *mut Sqlite3, p_expr_1: &Expr) -> i32 {
    unsafe {
        let mut p_list: *const ExprList = core::ptr::null();
        if (*p_expr_1).op as i32 == 172 {
            let z: *const i8 = (*p_expr_1).u.z_token as *const i8;
            let mut p_def: *const FuncDef = core::ptr::null();
            if unsafe { *z.offset(0 as isize) } as i32 != 'i' as i32 &&
                    unsafe { *z.offset(0 as isize) } as i32 != 'I' as i32 {
                return 0;
            }
            if (*p_expr_1).x.p_list == core::ptr::null_mut() { return 0; }
            p_def =
                unsafe {
                    sqlite3_find_function(db, z,
                        unsafe { (*(*p_expr_1).x.p_list).n_expr },
                        unsafe { (*db).enc }, 0 as u8)
                };
            if p_def == core::ptr::null_mut() { return 0; }
            if unsafe { (*p_def).func_flags } & 4194304 as u32 == 0 as u32 {
                return 0;
            }
            if unsafe { (*p_def).p_user_data } as i64 as i32 != 5 {
                return 0;
            }
        } else if (*p_expr_1).op as i32 == 158 {
            if (*p_expr_1).p_left != core::ptr::null_mut() { return 0; }
        } else { return 0; }
        p_list = (*p_expr_1).x.p_list;
        { let _ = 0; };
        if unsafe { (*p_list).n_expr } == 2 { return 1; }
        if unsafe { (*p_list).n_expr } == 3 &&
                sqlite3_expr_is_not_true(unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                *mut ExprListItem).offset(2 as isize)).p_expr
                            } as *const Expr) != 0 {
            return 1;
        }
        return 0;
    }
}

///* Return true if we can prove the pE2 will always be true if pE1 is
///* true.  Return false if we cannot complete the proof or if pE2 might
///* be false.  Examples:
///*
///*     pE1: x==5        pE2: x==5             Result: true
///*     pE1: x>0         pE2: x==5             Result: false
///*     pE1: x=21        pE2: x=21 OR y=43     Result: true
///*     pE1: x!=123      pE2: x IS NOT NULL    Result: true
///*     pE1: x!=?1       pE2: x IS NOT NULL    Result: true
///*     pE1: x IS NULL   pE2: x IS NOT NULL    Result: false
///*     pE1: x IS ?2     pE2: x IS NOT NULL    Result: false
///*     pE1: iif(x,y)    pE2: x                Result: true
///*     PE1: iif(x,y,0)  pE2: x                Result: true
///*
///* When comparing TK_COLUMN nodes between pE1 and pE2, if pE2 has
///* Expr.iTable<0 then assume a table number given by iTab.
///*
///* If pParse is not NULL, then the values of bound variables in pE1 are
///* compared against literal values in pE2 and pParse->pVdbe->expmask is
///* modified to record which bound variables are referenced.  If pParse
///* is NULL, then false will be returned if pE1 contains any bound variables.
///*
///* When in doubt, return false.  Returning true might give a performance
///* improvement.  Returning false might cause a performance reduction, but
///* it will always give the correct answer and is hence always safe.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_implies_expr(p_parse: *const Parse,
    p_e1: *const Expr, p_e2: *const Expr, i_tab: i32) -> i32 {
    unsafe {
        if sqlite3_expr_compare(p_parse, p_e1, p_e2, i_tab) == 0 { return 1; }
        if unsafe { (*p_e2).op } as i32 == 43 &&
                (sqlite3_expr_implies_expr(p_parse, p_e1,
                            unsafe { (*p_e2).p_left } as *const Expr, i_tab) != 0 ||
                    sqlite3_expr_implies_expr(p_parse, p_e1,
                            unsafe { (*p_e2).p_right } as *const Expr, i_tab) != 0) {
            return 1;
        }
        if unsafe { (*p_e2).op } as i32 == 52 &&
                expr_implies_not_null(p_parse, p_e1,
                        unsafe { (*p_e2).p_left } as *const Expr, i_tab, 0) != 0 {
            return 1;
        }
        if sqlite3_expr_is_iif(unsafe { (*p_parse).db }, unsafe { &*p_e1 }) !=
                0 {
            return sqlite3_expr_implies_expr(p_parse,
                    unsafe {
                            (*(unsafe { (*unsafe { (*p_e1).x.p_list }).a.as_ptr() } as
                                            *mut ExprListItem).offset(0 as isize)).p_expr
                        } as *const Expr, p_e2, i_tab);
        }
        return 0;
    }
}

/// This is a helper function to impliesNotNullRow().  In this routine,
///* set pWalker->eCode to one only if *both* of the input expressions
///* separately have the implies-not-null-row property.
extern "C" fn both_imply_not_null_row(p_walker_1: *mut Walker,
    p_e1_1: *mut Expr, p_e2_1: *mut Expr) -> () {
    if unsafe { (*p_walker_1).e_code } as i32 == 0 {
        unsafe { sqlite3_walk_expr(p_walker_1, p_e1_1) };
        if unsafe { (*p_walker_1).e_code } != 0 {
            unsafe { (*p_walker_1).e_code = 0 as u16 };
            unsafe { sqlite3_walk_expr(p_walker_1, p_e2_1) };
        }
    }
}

///* This is the Expr node callback for sqlite3ExprImpliesNonNullRow().
///* If the expression node requires that the table at pWalker->iCur
///* have one or more non-NULL column, then set pWalker->eCode to 1 and abort.
///*
///* pWalker->mWFlags is non-zero if this inquiry is being undertaking on
///* behalf of a RIGHT JOIN (or FULL JOIN).  That makes a difference when
///* evaluating terms in the ON clause of an inner join.
///*
///* This routine controls an optimization.  False positives (setting
///* pWalker->eCode to 1 when it should not be) are deadly, but false-negatives
///* (never setting pWalker->eCode) is a harmless missed optimization.
#[allow(unused_doc_comments)]
extern "C" fn implies_not_null_row(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).flags } & 1 as u32 != 0 as u32 { return 1; }
        if unsafe { (*p_expr_1).flags } & 2 as u32 != 0 as u32 &&
                unsafe { (*p_walker_1).m_w_flags } != 0 {

            /// If iCur is used in an inner-join ON clause to the left of a
            ///* RIGHT JOIN, that does *not* mean that the table must be non-null.
            ///* But it is difficult to check for that condition precisely.
            ///* To keep things simple, any use of iCur from any inner-join is
            ///* ignored while attempting to simplify a RIGHT JOIN.
            return 1;
        }
        '__s27:
            {
            match unsafe { (*p_expr_1).op } {
                46 => { return 1; }
                51 => { return 1; }
                52 => { return 1; }
                45 => { return 1; }
                177 => { return 1; }
                172 => { return 1; }
                175 => { return 1; }
                158 => { return 1; }
                168 => {
                    if unsafe { (*p_walker_1).u.i_cur } ==
                            unsafe { (*p_expr_1).i_table } {
                        unsafe { (*p_walker_1).e_code = 1 as u16 };
                        return 2;
                    }
                    return 1;
                }
                43 => {
                    both_imply_not_null_row(p_walker_1,
                        unsafe { (*p_expr_1).p_left },
                        unsafe { (*p_expr_1).p_right });
                    return 1;
                }
                44 => {
                    both_imply_not_null_row(p_walker_1,
                        unsafe { (*p_expr_1).p_left },
                        unsafe { (*p_expr_1).p_right });
                    return 1;
                }
                50 => {
                    if unsafe { (*p_expr_1).flags } & 4096 as u32 == 0 as u32 &&
                            unsafe { (*unsafe { (*p_expr_1).x.p_list }).n_expr } > 0 {
                        unsafe {
                            sqlite3_walk_expr(p_walker_1, unsafe { (*p_expr_1).p_left })
                        };
                    }
                    return 1;
                }
                49 => {
                    { let _ = 0; };
                    { let _ = 0; };
                    unsafe {
                        sqlite3_walk_expr(p_walker_1, unsafe { (*p_expr_1).p_left })
                    };
                    both_imply_not_null_row(p_walker_1,
                        unsafe {
                            (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                            as *mut ExprListItem).offset(0 as isize)).p_expr
                        },
                        unsafe {
                            (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                            as *mut ExprListItem).offset(1 as isize)).p_expr
                        });
                    return 1;
                }
                54 => {
                    {
                        let p_left: *const Expr =
                            unsafe { (*p_expr_1).p_left } as *const Expr;
                        let p_right: *const Expr =
                            unsafe { (*p_expr_1).p_right } as *const Expr;

                        /// The y.pTab=0 assignment in wherecode.c always happens after the
                        ///* impliesNotNullRow() test
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_left).op } as i32 == 168 &&
                                        unsafe { (*p_left).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_left).y.p_tab }).e_tab_type } as i32
                                        == 1 ||
                                unsafe { (*p_right).op } as i32 == 168 &&
                                        unsafe { (*p_right).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_right).y.p_tab }).e_tab_type } as
                                            i32 == 1 {
                            return 1;
                        }
                    }
                    return 0;
                }
                53 => {
                    {
                        let p_left: *const Expr =
                            unsafe { (*p_expr_1).p_left } as *const Expr;
                        let p_right: *const Expr =
                            unsafe { (*p_expr_1).p_right } as *const Expr;

                        /// The y.pTab=0 assignment in wherecode.c always happens after the
                        ///* impliesNotNullRow() test
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_left).op } as i32 == 168 &&
                                        unsafe { (*p_left).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_left).y.p_tab }).e_tab_type } as i32
                                        == 1 ||
                                unsafe { (*p_right).op } as i32 == 168 &&
                                        unsafe { (*p_right).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_right).y.p_tab }).e_tab_type } as
                                            i32 == 1 {
                            return 1;
                        }
                    }
                    return 0;
                }
                57 => {
                    {
                        let p_left: *const Expr =
                            unsafe { (*p_expr_1).p_left } as *const Expr;
                        let p_right: *const Expr =
                            unsafe { (*p_expr_1).p_right } as *const Expr;

                        /// The y.pTab=0 assignment in wherecode.c always happens after the
                        ///* impliesNotNullRow() test
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_left).op } as i32 == 168 &&
                                        unsafe { (*p_left).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_left).y.p_tab }).e_tab_type } as i32
                                        == 1 ||
                                unsafe { (*p_right).op } as i32 == 168 &&
                                        unsafe { (*p_right).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_right).y.p_tab }).e_tab_type } as
                                            i32 == 1 {
                            return 1;
                        }
                    }
                    return 0;
                }
                56 => {
                    {
                        let p_left: *const Expr =
                            unsafe { (*p_expr_1).p_left } as *const Expr;
                        let p_right: *const Expr =
                            unsafe { (*p_expr_1).p_right } as *const Expr;

                        /// The y.pTab=0 assignment in wherecode.c always happens after the
                        ///* impliesNotNullRow() test
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_left).op } as i32 == 168 &&
                                        unsafe { (*p_left).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_left).y.p_tab }).e_tab_type } as i32
                                        == 1 ||
                                unsafe { (*p_right).op } as i32 == 168 &&
                                        unsafe { (*p_right).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_right).y.p_tab }).e_tab_type } as
                                            i32 == 1 {
                            return 1;
                        }
                    }
                    return 0;
                }
                55 => {
                    {
                        let p_left: *const Expr =
                            unsafe { (*p_expr_1).p_left } as *const Expr;
                        let p_right: *const Expr =
                            unsafe { (*p_expr_1).p_right } as *const Expr;

                        /// The y.pTab=0 assignment in wherecode.c always happens after the
                        ///* impliesNotNullRow() test
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_left).op } as i32 == 168 &&
                                        unsafe { (*p_left).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_left).y.p_tab }).e_tab_type } as i32
                                        == 1 ||
                                unsafe { (*p_right).op } as i32 == 168 &&
                                        unsafe { (*p_right).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_right).y.p_tab }).e_tab_type } as
                                            i32 == 1 {
                            return 1;
                        }
                    }
                    return 0;
                }
                58 => {
                    {
                        let p_left: *const Expr =
                            unsafe { (*p_expr_1).p_left } as *const Expr;
                        let p_right: *const Expr =
                            unsafe { (*p_expr_1).p_right } as *const Expr;

                        /// The y.pTab=0 assignment in wherecode.c always happens after the
                        ///* impliesNotNullRow() test
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_left).op } as i32 == 168 &&
                                        unsafe { (*p_left).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_left).y.p_tab }).e_tab_type } as i32
                                        == 1 ||
                                unsafe { (*p_right).op } as i32 == 168 &&
                                        unsafe { (*p_right).y.p_tab } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_right).y.p_tab }).e_tab_type } as
                                            i32 == 1 {
                            return 1;
                        }
                    }
                    return 0;
                }
                _ => { return 0; }
            }
        }
    }
}

///* Return true (non-zero) if expression p can only be true if at least
///* one column of table iTab is non-null.  In other words, return true
///* if expression p will always be NULL or false if every column of iTab
///* is NULL.
///*
///* False negatives are acceptable.  In other words, it is ok to return
///* zero even if expression p will never be true of every column of iTab
///* is NULL.  A false negative is merely a missed optimization opportunity.
///*
///* False positives are not allowed, however.  A false positive may result
///* in an incorrect answer.
///*
///* Terms of p that are marked with EP_OuterON (and hence that come from
///* the ON or USING clauses of OUTER JOINS) are excluded from the analysis.
///*
///* This routine is used to check if a LEFT JOIN can be converted into
///* an ordinary JOIN.  The p argument is the WHERE clause.  If the WHERE
///* clause requires that some column of the right table of the LEFT JOIN
///* be non-NULL, then the LEFT JOIN can be safely converted into an
///* ordinary join.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_implies_non_null_row(mut p: *mut Expr,
    i_tab: i32, is_rj: i32) -> i32 {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        p = sqlite3_expr_skip_collate_and_likely(p);
        if p == core::ptr::null_mut() { return 0; }
        if unsafe { (*p).op } as i32 == 52 {
            p = unsafe { (*p).p_left };
        } else {
            while unsafe { (*p).op } as i32 == 44 {
                if sqlite3_expr_implies_non_null_row(unsafe { (*p).p_left },
                            i_tab, is_rj) != 0 {
                    return 1;
                }
                p = unsafe { (*p).p_right };
            }
        }
        w.x_expr_callback = Some(implies_not_null_row);
        w.x_select_callback = None;
        w.x_select_callback2 = None;
        w.e_code = 0 as u16;
        w.m_w_flags = (is_rj != 0) as u16;
        w.u.i_cur = i_tab;
        unsafe { sqlite3_walk_expr(&mut w, p) };
        return w.e_code as i32;
    }
}

///* Generate code to implement special SQL functions that are implemented
///* in-line rather than by using the usual callbacks.
#[allow(unused_doc_comments)]
extern "C" fn expr_code_inline_function(p_parse_1: *mut Parse,
    p_farg_1: *mut ExprList, i_func_id_1: i32, mut target: i32) -> i32 {
    unsafe {
        let mut n_farg: i32 = 0;
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        { let _ = 0; };
        { let _ = 0; };
        n_farg = unsafe { (*p_farg_1).n_expr };
        { let _ = 0; };
        '__s29:
            {
            match i_func_id_1 {
                0 => {
                    {
                        /// Attempt a direct implementation of the built-in COALESCE() and
                        ///* IFNULL() functions.  This avoids unnecessary evaluation of
                        ///* arguments past the first non-NULL argument.
                        let end_coalesce: i32 =
                            unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        let mut i: i32 = 0;
                        { let _ = 0; };
                        sqlite3_expr_code(p_parse_1,
                            unsafe {
                                (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                *mut ExprListItem).offset(0 as isize)).p_expr
                            }, target);
                        {
                            i = 1;
                            '__b30: loop {
                                if !(i < n_farg) { break '__b30; }
                                '__c30: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 52, target, end_coalesce)
                                    };
                                    sqlite3_expr_code(p_parse_1,
                                        unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        }, target);
                                    break '__c30;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        set_do_not_merge_flag_on_copy(v);
                        unsafe { sqlite3_vdbe_resolve_label(v, end_coalesce) };
                        break '__s29;
                    }
                    {
                        let mut case_expr: Expr = unsafe { core::mem::zeroed() };
                        unsafe {
                            memset(&raw mut case_expr as *mut (), 0,
                                core::mem::size_of::<Expr>() as u64)
                        };
                        case_expr.op = 158 as u8;
                        case_expr.x.p_list = p_farg_1;
                        return sqlite3_expr_code_target(p_parse_1, &mut case_expr,
                                target);
                    }
                    {

                        /// The UNLIKELY() function is a no-op.  The result is the value
                        ///* of the first argument.
                        { let _ = 0; };
                        target =
                            sqlite3_expr_code_target(p_parse_1,
                                unsafe {
                                    (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                    *mut ExprListItem).offset(0 as isize)).p_expr
                                }, target);
                        break '__s29;
                    }
                    {

                        /// Compare two expressions using sqlite3ExprCompare()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_compare(core::ptr::null(),
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {

                        /// Compare two expressions using sqlite3ExprImpliesExpr()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_implies_expr(p_parse_1 as *const Parse,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {
                        /// Result of sqlite3ExprImpliesNonNullRow()
                        let mut p_a1: *const Expr = core::ptr::null();
                        { let _ = 0; };
                        p_a1 =
                            unsafe {
                                (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                *mut ExprListItem).offset(1 as isize)).p_expr
                            };
                        if unsafe { (*p_a1).op } as i32 == 168 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 73,
                                    sqlite3_expr_implies_non_null_row(unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        }, unsafe { (*p_a1).i_table }, 1), target)
                            };
                        } else {
                            unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        }
                        break '__s29;
                    }
                    {
                        /// The AFFINITY() function evaluates to a string that describes
                        ///* the type affinity of the argument.  This is used for testing of
                        ///* the SQLite type logic.
                        let az_aff: [*const i8; 6] =
                            [c"blob".as_ptr() as *const i8,
                                    c"text".as_ptr() as *const i8,
                                    c"numeric".as_ptr() as *const i8,
                                    c"integer".as_ptr() as *const i8,
                                    c"real".as_ptr() as *const i8,
                                    c"flexnum".as_ptr() as *const i8];
                        let mut aff: i8 = 0 as i8;
                        { let _ = 0; };
                        aff =
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).p_expr
                                    } as *const Expr);
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_load_string(v, target,
                                if aff as i32 <= 64 {
                                    c"none".as_ptr() as *mut i8 as *const i8
                                } else { az_aff[(aff as i32 - 65) as usize] })
                        };
                        break '__s29;
                    }
                }
                5 => {
                    {
                        let mut case_expr: Expr = unsafe { core::mem::zeroed() };
                        unsafe {
                            memset(&raw mut case_expr as *mut (), 0,
                                core::mem::size_of::<Expr>() as u64)
                        };
                        case_expr.op = 158 as u8;
                        case_expr.x.p_list = p_farg_1;
                        return sqlite3_expr_code_target(p_parse_1, &mut case_expr,
                                target);
                    }
                    {

                        /// The UNLIKELY() function is a no-op.  The result is the value
                        ///* of the first argument.
                        { let _ = 0; };
                        target =
                            sqlite3_expr_code_target(p_parse_1,
                                unsafe {
                                    (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                    *mut ExprListItem).offset(0 as isize)).p_expr
                                }, target);
                        break '__s29;
                    }
                    {

                        /// Compare two expressions using sqlite3ExprCompare()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_compare(core::ptr::null(),
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {

                        /// Compare two expressions using sqlite3ExprImpliesExpr()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_implies_expr(p_parse_1 as *const Parse,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {
                        /// Result of sqlite3ExprImpliesNonNullRow()
                        let mut p_a1: *const Expr = core::ptr::null();
                        { let _ = 0; };
                        p_a1 =
                            unsafe {
                                (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                *mut ExprListItem).offset(1 as isize)).p_expr
                            };
                        if unsafe { (*p_a1).op } as i32 == 168 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 73,
                                    sqlite3_expr_implies_non_null_row(unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        }, unsafe { (*p_a1).i_table }, 1), target)
                            };
                        } else {
                            unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        }
                        break '__s29;
                    }
                    {
                        /// The AFFINITY() function evaluates to a string that describes
                        ///* the type affinity of the argument.  This is used for testing of
                        ///* the SQLite type logic.
                        let az_aff: [*const i8; 6] =
                            [c"blob".as_ptr() as *const i8,
                                    c"text".as_ptr() as *const i8,
                                    c"numeric".as_ptr() as *const i8,
                                    c"integer".as_ptr() as *const i8,
                                    c"real".as_ptr() as *const i8,
                                    c"flexnum".as_ptr() as *const i8];
                        let mut aff: i8 = 0 as i8;
                        { let _ = 0; };
                        aff =
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).p_expr
                                    } as *const Expr);
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_load_string(v, target,
                                if aff as i32 <= 64 {
                                    c"none".as_ptr() as *mut i8 as *const i8
                                } else { az_aff[(aff as i32 - 65) as usize] })
                        };
                        break '__s29;
                    }
                }
                3 => {
                    {

                        /// Compare two expressions using sqlite3ExprCompare()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_compare(core::ptr::null(),
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {

                        /// Compare two expressions using sqlite3ExprImpliesExpr()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_implies_expr(p_parse_1 as *const Parse,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {
                        /// Result of sqlite3ExprImpliesNonNullRow()
                        let mut p_a1: *const Expr = core::ptr::null();
                        { let _ = 0; };
                        p_a1 =
                            unsafe {
                                (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                *mut ExprListItem).offset(1 as isize)).p_expr
                            };
                        if unsafe { (*p_a1).op } as i32 == 168 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 73,
                                    sqlite3_expr_implies_non_null_row(unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        }, unsafe { (*p_a1).i_table }, 1), target)
                            };
                        } else {
                            unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        }
                        break '__s29;
                    }
                    {
                        /// The AFFINITY() function evaluates to a string that describes
                        ///* the type affinity of the argument.  This is used for testing of
                        ///* the SQLite type logic.
                        let az_aff: [*const i8; 6] =
                            [c"blob".as_ptr() as *const i8,
                                    c"text".as_ptr() as *const i8,
                                    c"numeric".as_ptr() as *const i8,
                                    c"integer".as_ptr() as *const i8,
                                    c"real".as_ptr() as *const i8,
                                    c"flexnum".as_ptr() as *const i8];
                        let mut aff: i8 = 0 as i8;
                        { let _ = 0; };
                        aff =
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).p_expr
                                    } as *const Expr);
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_load_string(v, target,
                                if aff as i32 <= 64 {
                                    c"none".as_ptr() as *mut i8 as *const i8
                                } else { az_aff[(aff as i32 - 65) as usize] })
                        };
                        break '__s29;
                    }
                }
                2 => {
                    {

                        /// Compare two expressions using sqlite3ExprImpliesExpr()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_implies_expr(p_parse_1 as *const Parse,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {
                        /// Result of sqlite3ExprImpliesNonNullRow()
                        let mut p_a1: *const Expr = core::ptr::null();
                        { let _ = 0; };
                        p_a1 =
                            unsafe {
                                (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                *mut ExprListItem).offset(1 as isize)).p_expr
                            };
                        if unsafe { (*p_a1).op } as i32 == 168 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 73,
                                    sqlite3_expr_implies_non_null_row(unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        }, unsafe { (*p_a1).i_table }, 1), target)
                            };
                        } else {
                            unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        }
                        break '__s29;
                    }
                    {
                        /// The AFFINITY() function evaluates to a string that describes
                        ///* the type affinity of the argument.  This is used for testing of
                        ///* the SQLite type logic.
                        let az_aff: [*const i8; 6] =
                            [c"blob".as_ptr() as *const i8,
                                    c"text".as_ptr() as *const i8,
                                    c"numeric".as_ptr() as *const i8,
                                    c"integer".as_ptr() as *const i8,
                                    c"real".as_ptr() as *const i8,
                                    c"flexnum".as_ptr() as *const i8];
                        let mut aff: i8 = 0 as i8;
                        { let _ = 0; };
                        aff =
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).p_expr
                                    } as *const Expr);
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_load_string(v, target,
                                if aff as i32 <= 64 {
                                    c"none".as_ptr() as *mut i8 as *const i8
                                } else { az_aff[(aff as i32 - 65) as usize] })
                        };
                        break '__s29;
                    }
                }
                1 => {
                    {
                        /// Result of sqlite3ExprImpliesNonNullRow()
                        let mut p_a1: *const Expr = core::ptr::null();
                        { let _ = 0; };
                        p_a1 =
                            unsafe {
                                (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                *mut ExprListItem).offset(1 as isize)).p_expr
                            };
                        if unsafe { (*p_a1).op } as i32 == 168 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 73,
                                    sqlite3_expr_implies_non_null_row(unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        }, unsafe { (*p_a1).i_table }, 1), target)
                            };
                        } else {
                            unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        }
                        break '__s29;
                    }
                    {
                        /// The AFFINITY() function evaluates to a string that describes
                        ///* the type affinity of the argument.  This is used for testing of
                        ///* the SQLite type logic.
                        let az_aff: [*const i8; 6] =
                            [c"blob".as_ptr() as *const i8,
                                    c"text".as_ptr() as *const i8,
                                    c"numeric".as_ptr() as *const i8,
                                    c"integer".as_ptr() as *const i8,
                                    c"real".as_ptr() as *const i8,
                                    c"flexnum".as_ptr() as *const i8];
                        let mut aff: i8 = 0 as i8;
                        { let _ = 0; };
                        aff =
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).p_expr
                                    } as *const Expr);
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_load_string(v, target,
                                if aff as i32 <= 64 {
                                    c"none".as_ptr() as *mut i8 as *const i8
                                } else { az_aff[(aff as i32 - 65) as usize] })
                        };
                        break '__s29;
                    }
                }
                4 => {
                    {
                        /// The AFFINITY() function evaluates to a string that describes
                        ///* the type affinity of the argument.  This is used for testing of
                        ///* the SQLite type logic.
                        let az_aff: [*const i8; 6] =
                            [c"blob".as_ptr() as *const i8,
                                    c"text".as_ptr() as *const i8,
                                    c"numeric".as_ptr() as *const i8,
                                    c"integer".as_ptr() as *const i8,
                                    c"real".as_ptr() as *const i8,
                                    c"flexnum".as_ptr() as *const i8];
                        let mut aff: i8 = 0 as i8;
                        { let _ = 0; };
                        aff =
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).p_expr
                                    } as *const Expr);
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_load_string(v, target,
                                if aff as i32 <= 64 {
                                    c"none".as_ptr() as *mut i8 as *const i8
                                } else { az_aff[(aff as i32 - 65) as usize] })
                        };
                        break '__s29;
                    }
                }
                _ => {
                    {

                        /// The UNLIKELY() function is a no-op.  The result is the value
                        ///* of the first argument.
                        { let _ = 0; };
                        target =
                            sqlite3_expr_code_target(p_parse_1,
                                unsafe {
                                    (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                    *mut ExprListItem).offset(0 as isize)).p_expr
                                }, target);
                        break '__s29;
                    }
                    {

                        /// Compare two expressions using sqlite3ExprCompare()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_compare(core::ptr::null(),
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {

                        /// Compare two expressions using sqlite3ExprImpliesExpr()
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_implies_expr(p_parse_1 as *const Parse,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(1 as isize)).p_expr
                                        } as *const Expr, -1), target)
                        };
                        break '__s29;
                    }
                    {
                        /// Result of sqlite3ExprImpliesNonNullRow()
                        let mut p_a1: *const Expr = core::ptr::null();
                        { let _ = 0; };
                        p_a1 =
                            unsafe {
                                (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                *mut ExprListItem).offset(1 as isize)).p_expr
                            };
                        if unsafe { (*p_a1).op } as i32 == 168 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 73,
                                    sqlite3_expr_implies_non_null_row(unsafe {
                                            (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        }, unsafe { (*p_a1).i_table }, 1), target)
                            };
                        } else {
                            unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        }
                        break '__s29;
                    }
                    {
                        /// The AFFINITY() function evaluates to a string that describes
                        ///* the type affinity of the argument.  This is used for testing of
                        ///* the SQLite type logic.
                        let az_aff: [*const i8; 6] =
                            [c"blob".as_ptr() as *const i8,
                                    c"text".as_ptr() as *const i8,
                                    c"numeric".as_ptr() as *const i8,
                                    c"integer".as_ptr() as *const i8,
                                    c"real".as_ptr() as *const i8,
                                    c"flexnum".as_ptr() as *const i8];
                        let mut aff: i8 = 0 as i8;
                        { let _ = 0; };
                        aff =
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_farg_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).p_expr
                                    } as *const Expr);
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_load_string(v, target,
                                if aff as i32 <= 64 {
                                    c"none".as_ptr() as *mut i8 as *const i8
                                } else { az_aff[(aff as i32 - 65) as usize] })
                        };
                        break '__s29;
                    }
                }
            }
        }
        return target;
    }
}

///* Walk an expression tree.  Return non-zero if the expression is constant
///* or return zero if the expression involves variables or function calls.
///*
///* For the purposes of this function, a double-quoted string (ex: "abc")
///* is considered a variable but a single-quoted string (ex: 'abc') is
///* a constant.
///*
///* The pParse parameter may be NULL.  But if it is NULL, there is no way
///* to determine if function calls are constant or not, and hence all
///* function calls will be considered to be non-constant.  If pParse is
///* not NULL, then a function call might be constant, depending on the
///* function and on its parameters.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_is_constant(p_parse: *mut Parse, p: *mut Expr)
    -> i32 {
    return expr_is_const(p_parse, p, 1);
}

///* Generate code that pushes the value of every element of the given
///* expression list into a sequence of registers beginning at target.
///*
///* Return the number of elements evaluated.  The number returned will
///* usually be pList->nExpr but might be reduced if SQLITE_ECEL_OMITREF
///* is defined.
///*
///* The SQLITE_ECEL_DUP flag prevents the arguments from being
///* filled using OP_SCopy.  OP_Copy must be used instead.
///*
///* The SQLITE_ECEL_FACTOR argument allows constant arguments to be
///* factored out into initialization code.
///*
///* The SQLITE_ECEL_REF flag means that expressions in the list with
///* ExprList.a[].u.x.iOrderByCol>0 have already been evaluated and stored
///* in registers at srcReg, and so the value can be copied from there.
///* If SQLITE_ECEL_OMITREF is also set, then the values with u.x.iOrderByCol>0
///* are simply omitted rather than being copied from srcReg.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_code_expr_list(p_parse: *mut Parse,
    p_list: &mut ExprList, target: i32, src_reg: i32, mut flags: u8) -> i32 {
    unsafe {
        let mut p_item: *const ExprListItem = core::ptr::null();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut n: i32 = 0;
        let copy_op: u8 = if flags as i32 & 1 != 0 { 82 } else { 83 } as u8;
        let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };

        /// Never gets this far otherwise
        (n = (*p_list).n_expr);
        if (unsafe { (*p_parse).ok_const_factor() } == 0) as i32 != 0 {
            flags &= !2 as u8;
        }
        {
            { p_item = (*p_list).a.as_ptr() as *mut ExprListItem; i = 0 };
            '__b31: loop {
                if !(i < n) { break '__b31; }
                '__c31: loop {
                    let p_expr: *mut Expr = unsafe { (*p_item).p_expr };
                    if flags as i32 & 4 != 0 &&
                            { j = unsafe { (*p_item).u.x.i_order_by_col } as i32; j } >
                                0 {
                        if flags as i32 & 8 != 0 {
                            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, copy_op as i32, j + src_reg - 1,
                                    target + i)
                            };
                        }
                    } else if flags as i32 & 2 != 0 &&
                            sqlite3_expr_is_constant_not_join(p_parse, p_expr) != 0 {
                        sqlite3_expr_code_run_just_once(p_parse, p_expr,
                            target + i);
                    } else {
                        let in_reg: i32 =
                            sqlite3_expr_code_target(p_parse, p_expr, target + i);
                        if in_reg != target + i {
                            let mut p_op: *mut VdbeOp = core::ptr::null_mut();
                            if copy_op as i32 == 82 &&
                                                unsafe {
                                                            (*{
                                                                            p_op = unsafe { sqlite3_vdbe_get_last_op(v) };
                                                                            p_op
                                                                        }).opcode
                                                        } as i32 == 82 &&
                                            unsafe { (*p_op).p1 } + unsafe { (*p_op).p3 } + 1 == in_reg
                                        &&
                                        unsafe { (*p_op).p2 } + unsafe { (*p_op).p3 } + 1 ==
                                            target + i && unsafe { (*p_op).p5 } as i32 == 0 {
                                {
                                    let __p = unsafe { &mut (*p_op).p3 };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op2(v, copy_op as i32, in_reg, target + i)
                                };
                            }
                        }
                    }
                    break '__c31;
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
        return n;
    }
}

///* Load the Parse object passed as the first argument with an error
///* message of the form:
///*
///*   "sub-select returns N columns - expected M"
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_subselect_error(p_parse_1: *mut Parse,
    n_actual_1: i32, n_expect_1: i32) -> () {
    if unsafe { (*p_parse_1).n_err } == 0 {
        let z_fmt: *const i8 =
            c"sub-select returns %d columns - expected %d".as_ptr() as *mut i8
                as *const i8;
        unsafe {
            sqlite3_error_msg(p_parse_1, z_fmt, n_actual_1, n_expect_1)
        };
    }
}

///* Expression pExpr is a vector that has been used in a context where
///* it is not permitted. If pExpr is a sub-select vector, this routine
///* loads the Parse object with a message of the form:
///*
///*   "sub-select returns N columns - expected 1"
///*
///* Or, if it is a regular scalar vector:
///*
///*   "row value misused"
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_vector_error_msg(p_parse: *mut Parse, p_expr: &Expr)
    -> () {
    unsafe {
        if (*p_expr).flags & 4096 as u32 != 0 as u32 {
            sqlite3_subselect_error(p_parse,
                unsafe {
                    (*unsafe { (*(*p_expr).x.p_select).p_e_list }).n_expr
                }, 1);
        } else {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"row value misused".as_ptr() as *mut i8 as *const i8)
            };
        }
    }
}

///* Expr pIn is an IN(...) expression. This function checks that the
///* sub-select on the RHS of the IN() operator has the same number of
///* columns as the vector on the LHS. Or, if the RHS of the IN() is not
///* a sub-query, that the LHS is a vector of size 1.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_check_in(p_parse: *mut Parse, p_in: &Expr)
    -> i32 {
    unsafe {
        let n_vector: i32 =
            sqlite3_expr_vector_size(unsafe { &*(*p_in).p_left });
        if (*p_in).flags & 4096 as u32 != 0 as u32 &&
                (unsafe { (*unsafe { (*p_parse).db }).malloc_failed } == 0) as
                        i32 != 0 {
            if n_vector !=
                    unsafe {
                        (*unsafe { (*(*p_in).x.p_select).p_e_list }).n_expr
                    } {
                sqlite3_subselect_error(p_parse,
                    unsafe {
                        (*unsafe { (*(*p_in).x.p_select).p_e_list }).n_expr
                    }, n_vector);
                return 1;
            }
        } else if n_vector != 1 {
            sqlite3_vector_error_msg(p_parse, unsafe { &*(*p_in).p_left });
            return 1;
        }
        return 0;
    }
}

///* Argument pExpr is an (?, ?...) IN(...) expression. This
///* function allocates and returns a nul-terminated string containing
///* the affinities to be used for each column of the comparison.
///*
///* It is the responsibility of the caller to ensure that the returned
///* string is eventually freed using sqlite3DbFree().
extern "C" fn expr_in_affinity(p_parse_1: &Parse, p_expr_1: &Expr)
    -> *mut i8 {
    unsafe {
        let p_left: *mut Expr = (*p_expr_1).p_left;
        let n_val: i32 = sqlite3_expr_vector_size(unsafe { &*p_left });
        let p_select: *const Select =
            if (*p_expr_1).flags & 4096 as u32 != 0 as u32 {
                    (*p_expr_1).x.p_select
                } else { core::ptr::null_mut() } as *const Select;
        let mut z_ret: *mut i8 = core::ptr::null_mut();
        { let _ = 0; };
        z_ret =
            unsafe {
                    sqlite3_db_malloc_raw((*p_parse_1).db,
                        (1 as i64 + n_val as i64) as u64)
                } as *mut i8;
        if !(z_ret).is_null() {
            let mut i: i32 = 0;
            {
                i = 0;
                '__b32: loop {
                    if !(i < n_val) { break '__b32; }
                    '__c32: loop {
                        let p_a: *const Expr =
                            sqlite3_vector_field_subexpr(p_left, i) as *const Expr;
                        let a: i8 = sqlite3_expr_affinity(p_a as *const Expr);
                        if !(p_select).is_null() {
                            unsafe {
                                *z_ret.offset(i as isize) =
                                    sqlite3_compare_affinity(unsafe {
                                                (*(unsafe { (*unsafe { (*p_select).p_e_list }).a.as_ptr() }
                                                                as *mut ExprListItem).offset(i as isize)).p_expr
                                            } as *const Expr, a)
                            };
                        } else { unsafe { *z_ret.offset(i as isize) = a }; }
                        break '__c32;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { *z_ret.offset(n_val as isize) = '\u{0}' as i32 as i8 };
        }
        return z_ret;
    }
}

#[allow(unused_doc_comments)]
extern "C" fn is_candidate_for_in_opt(p_x_1: &Expr) -> *mut Select {
    unsafe {
        let mut p: *mut Select = core::ptr::null_mut();
        let mut p_src: *const SrcList = core::ptr::null();
        let mut p_e_list: *const ExprList = core::ptr::null();
        let mut p_tab: *const Table = core::ptr::null();
        let mut i: i32 = 0;
        if !((*p_x_1).flags & 4096 as u32 != 0 as u32) as i32 != 0 {
            return core::ptr::null_mut();
        }
        if (*p_x_1).flags & 64 as u32 != 0 as u32 {
            return core::ptr::null_mut();
        }

        /// Correlated subq
        (p = (*p_x_1).x.p_select);
        if !(unsafe { (*p).p_prior }).is_null() {
            return core::ptr::null_mut();
        }
        if unsafe { (*p).sel_flags } & (1 | 8) as u32 != 0 {
            return core::ptr::null_mut();
        }
        { let _ = 0; };
        if !(unsafe { (*p).p_limit }).is_null() {
            return core::ptr::null_mut();
        }
        if !(unsafe { (*p).p_where }).is_null() {
            return core::ptr::null_mut();
        }

        /// Has no WHERE clause
        (p_src = unsafe { (*p).p_src });
        { let _ = 0; };
        if unsafe { (*p_src).n_src } != 1 { return core::ptr::null_mut(); }
        if unsafe {
                    (*(unsafe { (*p_src).a.as_ptr() } as
                                        *mut SrcItem).offset(0 as isize)).fg.is_subquery()
                } != 0 {
            return core::ptr::null_mut();
        }

        /// FROM is not a subquery or view
        (p_tab =
            unsafe {
                (*(unsafe { (*p_src).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).p_s_tab
            });
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
            return core::ptr::null_mut();
        }

        /// FROM clause not a virtual table
        (p_e_list = unsafe { (*p).p_e_list });
        { let _ = 0; };
        {
            i = 0;
            '__b33: loop {
                if !(i < unsafe { (*p_e_list).n_expr }) { break '__b33; }
                '__c33: loop {
                    let p_res: *const Expr =
                        unsafe {
                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)).p_expr
                            } as *const Expr;
                    if unsafe { (*p_res).op } as i32 != 168 {
                        return core::ptr::null_mut();
                    }
                    { let _ = 0; };
                    break '__c33;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return p;
    }
}

///* Generate code that checks the left-most column of index table iCur to see if
///* it contains any NULL entries.  Cause the register at regHasNull to be set
///* to a non-NULL value if iCur contains no NULLs.  Cause register regHasNull
///* to be set to NULL if iCur contains one or more NULL values.
extern "C" fn sqlite3_set_has_null_flag(v: *mut Vdbe, i_cur_1: i32,
    reg_has_null_1: i32) -> () {
    let mut addr1: i32 = 0;
    unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_has_null_1) };
    addr1 = unsafe { sqlite3_vdbe_add_op1(v, 36, i_cur_1) };
    unsafe { sqlite3_vdbe_add_op3(v, 96, i_cur_1, 0, reg_has_null_1) };
    unsafe { sqlite3_vdbe_change_p5(v, 128 as u16) };
    unsafe { sqlite3_vdbe_jump_here(v, addr1) };
}

///* The argument is an IN operator with a list (not a subquery) on the
///* right-hand side.  Return TRUE if that list is constant.
extern "C" fn sqlite3_in_rhs_is_constant(p_parse_1: *mut Parse,
    p_in_1: *mut Expr) -> i32 {
    let mut p_lhs: *mut Expr = core::ptr::null_mut();
    let mut res: i32 = 0;
    { let _ = 0; };
    p_lhs = unsafe { (*p_in_1).p_left };
    unsafe { (*p_in_1).p_left = core::ptr::null_mut() };
    res = sqlite3_expr_is_constant(p_parse_1, p_in_1);
    unsafe { (*p_in_1).p_left = p_lhs };
    return res;
}

///* Scan all previously generated bytecode looking for an OP_BeginSubrtn
///* that is compatible with pExpr.  If found, add the y.sub values
///* to pExpr and return true.  If not found, return false.
extern "C" fn find_compatible_in_rhs_subrtn(p_parse_1: &Parse,
    p_expr_1: &mut Expr, p_new_sig_1: *const SubrtnSig) -> i32 {
    unsafe {
        let mut p_op: *mut VdbeOp = core::ptr::null_mut();
        let mut p_end: *mut VdbeOp = core::ptr::null_mut();
        let mut p_sig: *const SubrtnSig = core::ptr::null();
        let mut v: *mut Vdbe = core::ptr::null_mut();
        if p_new_sig_1 == core::ptr::null_mut() { return 0; }
        if (*p_parse_1).m_subrtn_sig as i32 &
                    1 << (unsafe { (*p_new_sig_1).sel_id } & 7) == 0 {
            return 0;
        }
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        v = (*p_parse_1).p_vdbe;
        { let _ = 0; };
        p_op = unsafe { sqlite3_vdbe_get_op(v, 1) };
        p_end = unsafe { sqlite3_vdbe_get_last_op(v) };
        {
            '__b34: loop {
                if !(p_op < p_end) { break '__b34; }
                '__c34: loop {
                    if unsafe { (*p_op).p4type } as i32 != -18 { break '__c34; }
                    { let _ = 0; };
                    p_sig = unsafe { (*p_op).p4.p_subrtn_sig };
                    { let _ = 0; };
                    if (unsafe { (*p_sig).b_complete } == 0) as i32 != 0 {
                        break '__c34;
                    }
                    if unsafe { (*p_new_sig_1).sel_id } !=
                            unsafe { (*p_sig).sel_id } {
                        break '__c34;
                    }
                    if unsafe {
                                strcmp(unsafe { (*p_new_sig_1).z_aff } as *const i8,
                                    unsafe { (*p_sig).z_aff } as *const i8)
                            } != 0 {
                        break '__c34;
                    }
                    (*p_expr_1).y.sub.i_addr = unsafe { (*p_sig).i_addr };
                    (*p_expr_1).y.sub.reg_return =
                        unsafe { (*p_sig).reg_return };
                    (*p_expr_1).i_table = unsafe { (*p_sig).i_table };
                    (*p_expr_1).flags |= 33554432 as u32;
                    return 1;
                    break '__c34;
                }
                {
                    let __p = &mut p_op;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
        return 0;
    }
}

///* Generate code that will construct an ephemeral table containing all terms
///* in the RHS of an IN operator.  The IN operator can be in either of two
///* forms:
///*
///*     x IN (4,5,11)              -- IN operator with list on right-hand side
///*     x IN (SELECT a FROM b)     -- IN operator with subquery on the right
///*
///* The pExpr parameter is the IN operator.  The cursor number for the
///* constructed ephemeral table is returned.  The first time the ephemeral
///* table is computed, the cursor number is also stored in pExpr->iTable,
///* however the cursor number returned might not be the same, as it might
///* have been duplicated using OP_OpenDup.
///*
///* If the LHS expression ("x" in the examples) is a column value, or
///* the SELECT statement returns a column value, then the affinity of that
///* column is used to build the index keys. If both 'x' and the
///* SELECT... statement are columns, then numeric affinity is used
///* if either column has NUMERIC or INTEGER affinity. If neither
///* 'x' nor the SELECT... statement are columns, then numeric affinity
///* is used.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_code_rhs_of_in(p_parse: *mut Parse,
    p_expr: *mut Expr, i_tab: i32, allow_bloom: i32) -> () {
    unsafe {
        let mut addr_once: i32 = 0;
        /// Address of the OP_Once instruction at top
        let mut addr: i32 = 0;
        /// Address of OP_OpenEphemeral instruction
        let mut p_left: *mut Expr = core::ptr::null_mut();
        /// the LHS of the IN operator
        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
        /// Key information
        let mut n_val: i32 = 0;
        /// Size of vector pLeft
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// The prepared statement under construction
        let mut p_sig: *mut SubrtnSig = core::ptr::null_mut();

        /// Signature for this subroutine
        (v = unsafe { (*p_parse).p_vdbe });
        { let _ = 0; };
        if !(unsafe { (*p_expr).flags } & 64 as u32 != 0 as u32) as i32 != 0
                && unsafe { (*p_parse).i_self_tab } == 0 {

            /// Reuse of the RHS is allowed
            ///*
            ///* Compute a signature for the RHS of the IN operator to facility
            ///* finding and reusing prior instances of the same IN operator.
            { let _ = 0; };
            if unsafe { (*p_expr).flags } & 4096 as u32 != 0 as u32 &&
                    unsafe { (*unsafe { (*p_expr).x.p_select }).sel_flags } &
                            2 as u32 == 0 as u32 {
                p_sig =
                    unsafe {
                            sqlite3_db_malloc_raw_nn(unsafe { (*p_parse).db },
                                core::mem::size_of::<SubrtnSig>() as u64)
                        } as *mut SubrtnSig;
                if !(p_sig).is_null() {
                    unsafe {
                        (*p_sig).sel_id =
                            unsafe { (*unsafe { (*p_expr).x.p_select }).sel_id } as i32
                    };
                    unsafe {
                        (*p_sig).z_aff =
                            expr_in_affinity(unsafe { &*p_parse }, unsafe { &*p_expr })
                    };
                }
            }
            if unsafe { (*p_expr).flags } & 33554432 as u32 != 0 as u32 ||
                    find_compatible_in_rhs_subrtn(unsafe { &*p_parse },
                            unsafe { &mut *p_expr }, p_sig as *const SubrtnSig) != 0 {
                addr_once = unsafe { sqlite3_vdbe_add_op0(v, 15) };
                if unsafe { (*p_expr).flags } & 4096 as u32 != 0 as u32 {
                    unsafe {
                        sqlite3_vdbe_explain(p_parse, 0 as u8,
                            c"REUSE LIST SUBQUERY %d".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*unsafe { (*p_expr).x.p_select }).sel_id })
                    };
                }
                { let _ = 0; };
                unsafe {
                    sqlite3_vdbe_add_op2(v, 10,
                        unsafe { (*p_expr).y.sub.reg_return },
                        unsafe { (*p_expr).y.sub.i_addr })
                };
                { let _ = 0; };
                unsafe {
                    sqlite3_vdbe_add_op2(v, 117, i_tab,
                        unsafe { (*p_expr).i_table })
                };
                unsafe { sqlite3_vdbe_jump_here(v, addr_once) };
                if !(p_sig).is_null() {
                    unsafe {
                        sqlite3_db_free(unsafe { (*p_parse).db },
                            unsafe { (*p_sig).z_aff } as *mut ())
                    };
                    unsafe {
                        sqlite3_db_free(unsafe { (*p_parse).db }, p_sig as *mut ())
                    };
                }
                return;
            }

            /// Begin coding the subroutine
            { let _ = 0; };
            unsafe { (*p_expr).flags |= 33554432 as u32 };
            { let _ = 0; };
            unsafe {
                (*p_expr).y.sub.reg_return =
                    {
                        let __p = unsafe { &mut (*p_parse).n_mem };
                        *__p += 1;
                        *__p
                    }
            };
            unsafe {
                (*p_expr).y.sub.i_addr =
                    unsafe {
                            sqlite3_vdbe_add_op2(v, 76, 0,
                                unsafe { (*p_expr).y.sub.reg_return })
                        } + 1
            };
            if !(p_sig).is_null() {
                unsafe { (*p_sig).b_complete = 0 as u8 };
                unsafe {
                    (*p_sig).i_addr = unsafe { (*p_expr).y.sub.i_addr }
                };
                unsafe {
                    (*p_sig).reg_return = unsafe { (*p_expr).y.sub.reg_return }
                };
                unsafe { (*p_sig).i_table = i_tab };
                unsafe {
                    (*p_parse).m_subrtn_sig =
                        (1 << (unsafe { (*p_sig).sel_id } & 7)) as u8
                };
                unsafe {
                    sqlite3_vdbe_change_p4(v, -1, p_sig as *const i8, -18)
                };
            }
            addr_once = unsafe { sqlite3_vdbe_add_op0(v, 15) };
        }

        /// Check to see if this is a vector IN operator
        (p_left = unsafe { (*p_expr).p_left });
        n_val = sqlite3_expr_vector_size(unsafe { &*p_left });

        /// Construct the ephemeral table that will contain the content of
        ///* RHS of the IN operator.
        unsafe { (*p_expr).i_table = i_tab };
        addr =
            unsafe {
                sqlite3_vdbe_add_op2(v, 120, unsafe { (*p_expr).i_table },
                    n_val)
            };
        p_key_info =
            unsafe {
                sqlite3_key_info_alloc(unsafe { (*p_parse).db }, n_val, 1)
            };
        if unsafe { (*p_expr).flags } & 4096 as u32 != 0 as u32 {
            /// Case 1:     expr IN (SELECT ...)
            ///*
            ///* Generate code to write the results of the select into the temporary
            ///* table allocated and opened above.
            let p_select: *mut Select = unsafe { (*p_expr).x.p_select };
            let p_e_list: *const ExprList =
                unsafe { (*p_select).p_e_list } as *const ExprList;
            unsafe {
                sqlite3_vdbe_explain(p_parse, 1 as u8,
                    c"%sLIST SUBQUERY %d".as_ptr() as *mut i8 as *const i8,
                    if addr_once != 0 {
                        c"".as_ptr() as *mut i8
                    } else { c"CORRELATED ".as_ptr() as *mut i8 },
                    unsafe { (*p_select).sel_id })
            };
            if unsafe { (*p_e_list).n_expr } == n_val {
                let mut p_copy: *mut Select = core::ptr::null_mut();
                let mut dest: SelectDest = unsafe { core::mem::zeroed() };
                let mut i: i32 = 0;
                let mut rc: i32 = 0;
                let mut addr_bloom: i32 = 0;
                unsafe { sqlite3_select_dest_init(&mut dest, 9, i_tab) };
                dest.z_aff_sdst =
                    expr_in_affinity(unsafe { &*p_parse }, unsafe { &*p_expr });
                unsafe { (*p_select).i_limit = 0 };
                if addr_once != 0 && allow_bloom != 0 &&
                        unsafe { (*unsafe { (*p_parse).db }).db_opt_flags } &
                                524288 as u32 == 0 as u32 {
                    let reg_bloom: i32 =
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            *__p += 1;
                            *__p
                        };
                    addr_bloom =
                        unsafe { sqlite3_vdbe_add_op2(v, 79, 10000, reg_bloom) };
                    dest.i_sd_parm2 = reg_bloom;
                }

                /// Caused by OOM in sqlite3KeyInfoAlloc()
                (p_copy =
                    sqlite3_select_dup(unsafe { (*p_parse).db },
                        p_select as *const Select, 0));
                rc =
                    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                        {
                        1
                    } else {
                        unsafe { sqlite3_select(p_parse, p_copy, &mut dest) }
                    };
                unsafe {
                    sqlite3_select_delete(unsafe { (*p_parse).db }, p_copy)
                };
                unsafe {
                    sqlite3_db_free(unsafe { (*p_parse).db },
                        dest.z_aff_sdst as *mut ())
                };
                if addr_bloom != 0 {

                    /// Remember that location of the Bloom filter in the P3 operand
                    ///* of the OP_Once that began this subroutine. tag-202407032019
                    unsafe {
                        (*unsafe { sqlite3_vdbe_get_op(v, addr_once) }).p3 =
                            dest.i_sd_parm2
                    };
                    if dest.i_sd_parm2 == 0 {

                        /// If the Bloom filter won't actually be used, keep it small
                        unsafe {
                            (*unsafe { sqlite3_vdbe_get_op(v, addr_bloom) }).p1 = 10
                        };
                    }
                }
                if rc != 0 {
                    unsafe { sqlite3_key_info_unref(p_key_info) };
                    return;
                }
                { let _ = 0; };

                /// OOM will cause exit after sqlite3Select()
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                {
                    i = 0;
                    '__b35: loop {
                        if !(i < n_val) { break '__b35; }
                        '__c35: loop {
                            let p: *const Expr =
                                sqlite3_vector_field_subexpr(p_left, i) as *const Expr;
                            unsafe {
                                *(unsafe { (*p_key_info).a_coll.as_ptr() } as
                                                *mut *mut CollSeq).offset(i as isize) =
                                    sqlite3_binary_compare_coll_seq(p_parse, p as *const Expr,
                                        unsafe {
                                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                            } as *const Expr)
                            };
                            break '__c35;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        } else if unsafe { (*p_expr).x.p_list } != core::ptr::null_mut() {
            /// Case 2:     expr IN (exprlist)
            ///*
            ///* For each expression, build an index key from the evaluation and
            ///* store it in the temporary table. If <expr> is a column, then use
            ///* that columns affinity when building index keys. If <expr> is not
            ///* a column, use numeric affinity.
            let mut affinity: i8 = 0 as i8;
            /// Affinity of the LHS of the IN
            let mut i: i32 = 0;
            let p_list: *mut ExprList = unsafe { (*p_expr).x.p_list };
            let mut p_item: *const ExprListItem = core::ptr::null();
            let mut r1: i32 = 0;
            let mut r2: i32 = 0;
            affinity = sqlite3_expr_affinity(p_left as *const Expr);
            if affinity as i32 <= 64 {
                affinity = 65 as i8;
            } else if affinity as i32 == 69 { affinity = 67 as i8; }
            if !(p_key_info).is_null() {
                { let _ = 0; };
                unsafe {
                    *(unsafe { (*p_key_info).a_coll.as_ptr() } as
                                    *mut *mut CollSeq).offset(0 as isize) =
                        sqlite3_expr_coll_seq(p_parse,
                            unsafe { (*p_expr).p_left } as *const Expr)
                };
            }

            /// Loop through each expression in <exprlist>.
            (r1 = sqlite3_get_temp_reg(unsafe { &mut *p_parse }));
            r2 = sqlite3_get_temp_reg(unsafe { &mut *p_parse });
            {
                {
                    i = unsafe { (*p_list).n_expr };
                    p_item =
                        unsafe { (*p_list).a.as_ptr() } as *mut ExprListItem
                };
                '__b36: loop {
                    if !(i > 0) { break '__b36; }
                    '__c36: loop {
                        let p_e2: *mut Expr = unsafe { (*p_item).p_expr };
                        if addr_once != 0 &&
                                (sqlite3_expr_is_constant(p_parse, p_e2) == 0) as i32 != 0 {
                            unsafe { sqlite3_vdbe_change_to_noop(v, addr_once - 1) };
                            unsafe { sqlite3_vdbe_change_to_noop(v, addr_once) };
                            unsafe { (*p_expr).flags &= !(33554432 as u32) };
                            addr_once = 0;
                        }

                        /// Evaluate the expression and insert it into the temp table
                        sqlite3_expr_code(p_parse, p_e2, r1);
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, r1, 1, r2,
                                &raw mut affinity as *const i8, 1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_tab, r2, r1, 1)
                        };
                        break '__c36;
                    }
                    {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        {
                            let __p = &mut p_item;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                    };
                }
            }
            sqlite3_release_temp_reg(unsafe { &mut *p_parse }, r1);
            sqlite3_release_temp_reg(unsafe { &mut *p_parse }, r2);
        }
        if !(p_sig).is_null() { unsafe { (*p_sig).b_complete = 1 as u8 }; }
        if !(p_key_info).is_null() {
            unsafe {
                sqlite3_vdbe_change_p4(v, addr,
                    p_key_info as *mut () as *const i8, -9)
            };
        }
        if addr_once != 0 {
            unsafe { sqlite3_vdbe_add_op1(v, 138, i_tab) };
            unsafe { sqlite3_vdbe_jump_here(v, addr_once) };

            /// Subroutine return
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                sqlite3_vdbe_add_op3(v, 69,
                    unsafe { (*p_expr).y.sub.reg_return },
                    unsafe { (*p_expr).y.sub.i_addr }, 1)
            };
            sqlite3_clear_temp_reg_cache(unsafe { &mut *p_parse });
        }
    }
}

#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_find_in_index(p_parse: *mut Parse, p_x: *mut Expr,
    in_flags: u32, mut pr_rhs_has_null: *mut i32, ai_map: *mut i32,
    pi_tab: &mut i32) -> i32 {
    unsafe {
        let mut p: *const Select = core::ptr::null();
        /// SELECT to the right of IN operator
        let mut e_type: i32 = 0;
        /// Type of RHS table. IN_INDEX_*
        let mut i_tab: i32 = 0;
        /// Cursor of the RHS table
        let mut must_be_unique: i32 = 0;
        /// True if RHS must be unique
        let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };

        /// Virtual machine being coded
        { let _ = 0; };
        must_be_unique = (in_flags & 4 as u32 != 0 as u32) as i32;
        i_tab =
            {
                let __p = unsafe { &mut (*p_parse).n_tab };
                let __t = *__p;
                *__p += 1;
                __t
            };
        if !(pr_rhs_has_null).is_null() &&
                unsafe { (*p_x).flags } & 4096 as u32 != 0 as u32 {
            let mut i: i32 = 0;
            let p_e_list: *const ExprList =
                unsafe { (*unsafe { (*p_x).x.p_select }).p_e_list } as
                    *const ExprList;
            {
                i = 0;
                '__b37: loop {
                    if !(i < unsafe { (*p_e_list).n_expr }) { break '__b37; }
                    '__c37: loop {
                        if sqlite3_expr_can_be_null(unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        } as *const Expr) != 0 {
                            break '__b37;
                        }
                        break '__c37;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if i == unsafe { (*p_e_list).n_expr } {
                pr_rhs_has_null = core::ptr::null_mut();
            }
        }
        if unsafe { (*p_parse).n_err } == 0 &&
                { p = is_candidate_for_in_opt(unsafe { &*p_x }); p } !=
                    core::ptr::null_mut() {
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            /// Database connection
            let mut p_tab: *mut Table = core::ptr::null_mut();
            /// Table <table>.
            let mut i_db: i32 = 0;
            /// Database idx for pTab
            let p_e_list_1: *const ExprList =
                unsafe { (*p).p_e_list } as *const ExprList;
            let n_expr: i32 = unsafe { (*p_e_list_1).n_expr };
            { let _ = 0; };

            /// Because of isCandidateForInOpt(p)
            { let _ = 0; };

            /// Because of isCandidateForInOpt(p)
            { let _ = 0; };

            /// Because of isCandidateForInOpt(p)
            (p_tab =
                unsafe {
                    (*(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)).p_s_tab
                });

            /// Code an OP_Transaction and OP_TableLock for <table>.
            (i_db =
                unsafe {
                    sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                });
            { let _ = 0; };
            unsafe { sqlite3_code_verify_schema(p_parse, i_db) };
            unsafe {
                sqlite3_table_lock(p_parse, i_db, unsafe { (*p_tab).tnum },
                    0 as u8, unsafe { (*p_tab).z_name } as *const i8)
            };
            { let _ = 0; };
            if n_expr == 1 &&
                    (unsafe {
                                    (*unsafe {
                                                    (*(unsafe { (*p_e_list_1).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(0 as isize)).p_expr
                                                }).i_column
                                } as i32) < 0 {
                /// The "x IN (SELECT rowid FROM table)" case
                let i_addr: i32 = unsafe { sqlite3_vdbe_add_op0(v, 15) };
                unsafe {
                    sqlite3_open_table(p_parse, i_tab, i_db, p_tab, 114)
                };
                e_type = 1;
                unsafe {
                    sqlite3_vdbe_explain(p_parse, 0 as u8,
                        c"USING ROWID SEARCH ON TABLE %s FOR IN-OPERATOR".as_ptr()
                                as *mut i8 as *const i8, unsafe { (*p_tab).z_name })
                };
                unsafe { sqlite3_vdbe_jump_here(v, i_addr) };
            } else {
                let mut p_idx: *mut Index = core::ptr::null_mut();
                /// Iterator variable
                let mut affinity_ok: i32 = 1;
                let mut i: i32 = 0;
                {
                    i = 0;
                    '__b38: loop {
                        if !(i < n_expr && affinity_ok != 0) { break '__b38; }
                        '__c38: loop {
                            let p_lhs: *const Expr =
                                sqlite3_vector_field_subexpr(unsafe { (*p_x).p_left }, i) as
                                    *const Expr;
                            let i_col: i32 =
                                unsafe {
                                        (*unsafe {
                                                        (*(unsafe { (*p_e_list_1).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                                    }).i_column
                                    } as i32;
                            let idxaff: i8 =
                                sqlite3_table_column_affinity(unsafe { &*p_tab }, i_col);
                            /// RHS table
                            let cmpaff: i8 =
                                sqlite3_compare_affinity(p_lhs as *const Expr, idxaff);
                            '__s39:
                                {
                                match cmpaff {
                                    65 => {}
                                    66 => { { let _ = 0; }; }
                                    _ => { affinity_ok = (idxaff as i32 >= 67) as i32; }
                                }
                            }
                            break '__c38;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if affinity_ok != 0 {
                    {
                        p_idx = unsafe { (*p_tab).p_index };
                        '__b40: loop {
                            if !(!(p_idx).is_null() && e_type == 0) { break '__b40; }
                            '__c40: loop {
                                let mut col_used: Bitmask = 0 as Bitmask;
                                /// Columns of the index used
                                let mut m_col: Bitmask = 0 as Bitmask;
                                if (unsafe { (*p_idx).n_column } as i32) < n_expr {
                                    break '__c40;
                                }
                                if unsafe { (*p_idx).p_part_idx_where } !=
                                        core::ptr::null_mut() {
                                    break '__c40;
                                }
                                if unsafe { (*p_idx).n_column } as i32 >=
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                            1 {
                                    break '__c40;
                                }
                                if must_be_unique != 0 {
                                    if unsafe { (*p_idx).n_key_col } as i32 > n_expr ||
                                            unsafe { (*p_idx).n_column } as i32 > n_expr &&
                                                !(unsafe { (*p_idx).on_error } as i32 != 0) as i32 != 0 {
                                        break '__c40;
                                    }
                                }
                                col_used = 0 as Bitmask;
                                {
                                    i = 0;
                                    '__b41: loop {
                                        if !(i < n_expr) { break '__b41; }
                                        '__c41: loop {
                                            let p_lhs_1: *const Expr =
                                                sqlite3_vector_field_subexpr(unsafe { (*p_x).p_left }, i) as
                                                    *const Expr;
                                            let p_rhs: *const Expr =
                                                unsafe {
                                                        (*(unsafe { (*p_e_list_1).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                                    } as *const Expr;
                                            let p_req: *const CollSeq =
                                                sqlite3_binary_compare_coll_seq(p_parse,
                                                        p_lhs_1 as *const Expr, p_rhs as *const Expr) as
                                                    *const CollSeq;
                                            let mut j: i32 = 0;
                                            {
                                                j = 0;
                                                '__b42: loop {
                                                    if !(j < n_expr) { break '__b42; }
                                                    '__c42: loop {
                                                        if unsafe {
                                                                        *unsafe { (*p_idx).ai_column.offset(j as isize) }
                                                                    } as i32 != unsafe { (*p_rhs).i_column } as i32 {
                                                            break '__c42;
                                                        }
                                                        { let _ = 0; };
                                                        if p_req != core::ptr::null_mut() &&
                                                                unsafe {
                                                                        sqlite3_str_i_cmp(unsafe { (*p_req).z_name } as *const i8,
                                                                            unsafe { *unsafe { (*p_idx).az_coll.offset(j as isize) } })
                                                                    } != 0 {
                                                            break '__c42;
                                                        }
                                                        break '__b42;
                                                        break '__c42;
                                                    }
                                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                            if j == n_expr { break '__b41; }
                                            m_col = (1 as Bitmask) << j;
                                            if m_col & col_used != 0 { break '__b41; }

                                            /// Each column used only once
                                            (col_used |= m_col);
                                            if !(ai_map).is_null() {
                                                unsafe { *ai_map.offset(i as isize) = j };
                                            }
                                            break '__c41;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                { let _ = 0; };
                                { let _ = 0; };
                                if col_used == ((1 as Bitmask) << n_expr) - 1 as Bitmask {
                                    /// If we reach this point, that means the index pIdx is usable
                                    let i_addr_1: i32 = unsafe { sqlite3_vdbe_add_op0(v, 15) };
                                    unsafe {
                                        sqlite3_vdbe_explain(p_parse, 0 as u8,
                                            c"USING INDEX %s FOR IN-OPERATOR".as_ptr() as *mut i8 as
                                                *const i8, unsafe { (*p_idx).z_name })
                                    };
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 114, i_tab,
                                            unsafe { (*p_idx).tnum } as i32, i_db)
                                    };
                                    unsafe { sqlite3_vdbe_set_p4_key_info(p_parse, p_idx) };
                                    { let _ = 0; };
                                    e_type =
                                        3 +
                                            unsafe {
                                                    *unsafe { (*p_idx).a_sort_order.offset(0 as isize) }
                                                } as i32;
                                    if !(pr_rhs_has_null).is_null() {
                                        unsafe {
                                            *pr_rhs_has_null =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                                    *__p += 1;
                                                    *__p
                                                }
                                        };
                                        if n_expr == 1 {
                                            sqlite3_set_has_null_flag(v, i_tab,
                                                unsafe { *pr_rhs_has_null });
                                        }
                                    }
                                    unsafe { sqlite3_vdbe_jump_here(v, i_addr_1) };
                                }
                                break '__c40;
                            }
                            p_idx = unsafe { (*p_idx).p_next };
                        }
                    }
                }
            }
        }
        if e_type == 0 && in_flags & 1 as u32 != 0 &&
                    unsafe { (*p_x).flags } & 4096 as u32 == 0 as u32 &&
                ((sqlite3_in_rhs_is_constant(p_parse, p_x) == 0) as i32 != 0
                    || unsafe { (*unsafe { (*p_x).x.p_list }).n_expr } <= 2) {
            {
                let __p = unsafe { &mut (*p_parse).n_tab };
                let __t = *__p;
                *__p -= 1;
                __t
            };

            /// Back out the allocation of the unused cursor
            (i_tab = -1);

            /// Cursor is not allocated
            (e_type = 5);
        }
        if e_type == 0 {
            /// Could not find an existing table or index to use as the RHS b-tree.
            ///* We will have to generate an ephemeral table to do the job.
            let saved_n_query_loop: u32 =
                unsafe { (*p_parse).n_query_loop } as u32;
            let mut r_may_have_null: i32 = 0;
            let mut bloom_ok: i32 = (in_flags & 2 as u32 != 0 as u32) as i32;
            e_type = 2;
            if in_flags & 4 as u32 != 0 {
                unsafe { (*p_parse).n_query_loop = 0 as LogEst };
            } else if !(pr_rhs_has_null).is_null() {
                unsafe {
                    *pr_rhs_has_null =
                        {
                            r_may_have_null =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                };
                            r_may_have_null
                        }
                };
            }
            { let _ = 0; };
            if (bloom_ok == 0) as i32 != 0 &&
                        unsafe { (*p_x).flags } & 4096 as u32 != 0 as u32 &&
                    unsafe { (*unsafe { (*p_x).x.p_select }).sel_flags } &
                            32 as u32 != 0 as u32 {
                bloom_ok = 1;
            }
            sqlite3_code_rhs_of_in(p_parse, p_x, i_tab, bloom_ok);
            if r_may_have_null != 0 {
                sqlite3_set_has_null_flag(v, i_tab, r_may_have_null);
            }
            unsafe { (*p_parse).n_query_loop = saved_n_query_loop as LogEst };
        }
        if !(ai_map).is_null() && e_type != 3 && e_type != 4 {
            let mut i: i32 = 0;
            let mut n: i32 = 0;
            n =
                sqlite3_expr_vector_size(unsafe {
                        &*unsafe { (*p_x).p_left }
                    });
            {
                i = 0;
                '__b43: loop {
                    if !(i < n) { break '__b43; }
                    '__c43: loop {
                        unsafe { *ai_map.offset(i as isize) = i };
                        break '__c43;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        *pi_tab = i_tab;
        return e_type;
    }
}

///* Generate code that will evaluate expression pExpr and store the
///* results in register target.  The results are guaranteed to appear
///* in register target.  If the expression is constant, then this routine
///* might choose to code the expression at initialization time.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_factorable(p_parse: *mut Parse,
    p_expr: *mut Expr, target: i32) -> () {
    if unsafe { (*p_parse).ok_const_factor() } != 0 &&
            sqlite3_expr_is_constant_not_join(p_parse, p_expr) != 0 {
        sqlite3_expr_code_run_just_once(p_parse, p_expr, target);
    } else { sqlite3_expr_code_copy(p_parse, p_expr, target); }
}

///* Evaluate an expression (either a vector or a scalar expression) and store
///* the result in contiguous temporary registers.  Return the index of
///* the first register used to store the result.
///*
///* If the returned result register is a temporary scalar, then also write
///* that register number into *piFreeable.  If the returned result register
///* is not a temporary or if the expression is a vector set *piFreeable
///* to 0.
extern "C" fn expr_code_vector(p_parse: *mut Parse, p: *mut Expr,
    pi_freeable: *mut i32) -> i32 {
    unsafe {
        let mut i_result: i32 = 0;
        let n_result: i32 = sqlite3_expr_vector_size(unsafe { &*p });
        if n_result == 1 {
            i_result =
                sqlite3_expr_code_temp(p_parse, p,
                    unsafe { &mut *pi_freeable });
        } else {
            unsafe { *pi_freeable = 0 };
            if unsafe { (*p).op } as i32 == 139 {
                i_result =
                    sqlite3_code_subselect(p_parse, unsafe { &mut *p });
            } else {
                let mut i: i32 = 0;
                i_result = unsafe { (*p_parse).n_mem } + 1;
                unsafe { (*p_parse).n_mem += n_result };
                { let _ = 0; };
                {
                    i = 0;
                    '__b44: loop {
                        if !(i < n_result) { break '__b44; }
                        '__c44: loop {
                            sqlite3_expr_code_factorable(p_parse,
                                unsafe {
                                    (*(unsafe { (*unsafe { (*p).x.p_list }).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).p_expr
                                }, i + i_result);
                            break '__c44;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        }
        return i_result;
    }
}

///* Generate code for an IN expression.
///*
///*      x IN (SELECT ...)
///*      x IN (value, value, ...)
///*
///* The left-hand side (LHS) is a scalar or vector expression.  The
///* right-hand side (RHS) is an array of zero or more scalar values, or a
///* subquery.  If the RHS is a subquery, the number of result columns must
///* match the number of columns in the vector on the LHS.  If the RHS is
///* a list of values, the LHS must be a scalar.
///*
///* The IN operator is true if the LHS value is contained within the RHS.
///* The result is false if the LHS is definitely not in the RHS.  The
///* result is NULL if the presence of the LHS in the RHS cannot be
///* determined due to NULLs.
///*
///* This routine generates code that jumps to destIfFalse if the LHS is not
///* contained within the RHS.  If due to NULLs we cannot determine if the LHS
///* is contained in the RHS then jump to destIfNull.  If the LHS is contained
///* within the RHS then fall through.
///*
///* See the separate in-operator.md documentation file in the canonical
///* SQLite source tree for additional information.
#[allow(unused_doc_comments)]
extern "C" fn sqlite3_expr_code_in(p_parse_1: *mut Parse, p_expr_1: *mut Expr,
    dest_if_false_1: i32, dest_if_null_1: i32) -> () {
    unsafe {
        let mut r_rhs_has_null: i32 = 0;
        /// Register that is true if RHS contains NULL values
        let mut e_type: i32 = 0;
        /// Type of the RHS
        let mut r_lhs: i32 = 0;
        /// Register(s) holding the LHS values
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// Statement under construction
        let mut ai_map: *mut i32 = core::ptr::null_mut();
        /// Map from vector field to index column
        let mut z_aff: *mut i8 = core::ptr::null_mut();
        /// Affinity string for comparisons
        let mut n_vector: i32 = 0;
        /// Size of vectors for this IN operator
        let mut i_dummy: i32 = 0;
        /// Dummy parameter to exprCodeVector()
        let mut p_left: *mut Expr = core::ptr::null_mut();
        /// The LHS of the IN operator
        let mut i: i32 = 0;
        /// loop counter
        let mut dest_step2: i32 = 0;
        /// Where to jump when NULLs seen in step 2
        let mut dest_step6: i32 = 0;
        /// Start of code for Step 6
        let mut addr_truth_op: i32 = 0;
        /// Address of opcode that determines the IN is true
        let mut dest_not_null: i32 = 0;
        /// Jump here if a comparison is not true in step 6
        let mut addr_top: i32 = 0;
        /// Top of the step-6 loop
        let mut i_tab: i32 = 0;
        /// Index to use
        let mut ok_const_factor: u8 = 0 as u8;
        /// Attempt to compute the RHS. After this step, if anything other than
        ///* IN_INDEX_NOOP is returned, the table opened with cursor iTab
        ///* contains the values that make up the RHS. If IN_INDEX_NOOP is returned,
        ///* the RHS has not yet been coded.
        /// OOM detected prior to this routine
        /// Code the LHS, the <expr> from "<expr> IN (...)". If the LHS is a
        ///* vector, then it is stored in an array of nVector registers starting
        ///* at r1.
        ///*
        ///* sqlite3FindInIndex() might have reordered the fields of the LHS vector
        ///* so that the fields are in the same order as an existing index.   The
        ///* aiMap[] array contains a mapping from the original LHS field order to
        ///* the field order that matches the RHS index.
        ///*
        ///* Avoid factoring the LHS of the IN(...) expression out of the loop,
        ///* even if it is constant, as OP_Affinity may be used on the register
        ///* by code generated below.
        /// If sqlite3FindInIndex() did not find or create an index that is
        ///* suitable for evaluating the IN operator, then evaluate using a
        ///* sequence of comparisons.
        ///*
        ///* This is step (1) in the in-operator.md optimized algorithm.
        let mut p_list: *const ExprList = core::ptr::null();
        let mut p_coll: *mut CollSeq = core::ptr::null_mut();
        let mut label_ok: i32 = 0;
        let mut r2: i32 = 0;
        let mut reg_to_free: i32 = 0;
        let mut reg_ck_null: i32 = 0;
        let mut ii: i32 = 0;
        let mut op: i32 = 0;
        let mut op__1: i32 = 0;
        /// If this IN operator will use an index, then the order of columns in the
        ///* vector might be different from the order in the index.  In that case,
        ///* we need to reorder the LHS values to be in index order.  Run Affinity
        ///* before reordering the columns, so that the affinity is correct.
        /// Are LHS fields reordered?
        /// Need to reorder the LHS fields according to aiMap
        let mut r_lhs_orig: i32 = 0;
        /// Step 2: Check to see if the LHS contains any NULL columns.  If the
        ///* LHS does contain NULLs then the result must be either FALSE or NULL.
        ///* We will then skip the binary search of the RHS.
        let mut p: *const Expr = core::ptr::null();
        /// Step 3.  The LHS is now known to be non-NULL.  Do the binary search
        ///* of the RHS using the LHS as a probe.  If found, the result is
        ///* true.
        /// In this case, the RHS is the ROWID of table b-tree and so we also
        ///* know that the RHS is non-NULL.  Hence, we combine steps 3 and 4
        ///* into a single opcode.
        /// Return True
        /// Combine Step 3 and Step 5 into a single opcode
        let mut p_op: *const VdbeOp = core::ptr::null();
        /// tag-202407032019
        /// Ordinary Step 3, for the case where FALSE and NULL are distinct
        /// Step 4.  If the RHS is known to be non-NULL and we did not find
        ///* an match on the search above, then the result must be FALSE.
        /// Step 5.  If we do not care about the difference between NULL and
        ///* FALSE, then just return false.
        /// Step 6: Loop through rows of the RHS.  Compare each row to the LHS.
        ///* If any comparison is NULL, then the result is NULL.  If all
        ///* comparisons are FALSE then the final result is FALSE.
        ///*
        ///* For a scalar LHS, it is sufficient to check just the first row
        ///* of the RHS.
        /// For nVector==1, combine steps 6 and 7 by immediately returning
        ///* FALSE if the first comparison is not NULL
        let mut p__1: *const Expr = core::ptr::null();
        let mut p_coll_1: *mut CollSeq = core::ptr::null_mut();
        let mut r3: i32 = 0;
        let mut p_rhs: *const Expr = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s46:
                {
                match __state {
                    0 => { r_rhs_has_null = 0; __state = 4; }
                    2 => { __state = 162; }
                    3 => {
                        unsafe {
                            sqlite3_db_free(unsafe { (*p_parse_1).db },
                                ai_map as *mut ())
                        };
                        __state = 163;
                    }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { ai_map = core::ptr::null_mut(); __state = 8; }
                    8 => { z_aff = core::ptr::null_mut(); __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { __state = 14; }
                    14 => { dest_step6 = 0; __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => { __state = 18; }
                    18 => { i_tab = 0; __state = 19; }
                    19 => {
                        ok_const_factor =
                            unsafe { (*p_parse_1).ok_const_factor() } as u8;
                        __state = 20;
                    }
                    20 => { { let _ = 0; }; __state = 21; }
                    21 => {
                        p_left = unsafe { (*p_expr_1).p_left };
                        __state = 22;
                    }
                    22 => {
                        if sqlite3_expr_check_in(p_parse_1, unsafe { &*p_expr_1 })
                                != 0 {
                            __state = 24;
                        } else { __state = 23; }
                    }
                    23 => {
                        z_aff =
                            expr_in_affinity(unsafe { &*p_parse_1 },
                                unsafe { &*p_expr_1 });
                        __state = 25;
                    }
                    24 => { return; }
                    25 => {
                        n_vector =
                            sqlite3_expr_vector_size(unsafe {
                                    &*unsafe { (*p_expr_1).p_left }
                                });
                        __state = 26;
                    }
                    26 => {
                        ai_map =
                            unsafe {
                                    sqlite3_db_malloc_zero(unsafe { (*p_parse_1).db },
                                        n_vector as u64 * core::mem::size_of::<i32>() as u64)
                                } as *mut i32;
                        __state = 27;
                    }
                    27 => {
                        if unsafe { (*unsafe { (*p_parse_1).db }).malloc_failed } !=
                                0 {
                            __state = 29;
                        } else { __state = 28; }
                    }
                    28 => { v = unsafe { (*p_parse_1).p_vdbe }; __state = 30; }
                    29 => { __state = 3; }
                    30 => { { let _ = 0; }; __state = 31; }
                    31 => { __state = 32; }
                    32 => {
                        e_type =
                            sqlite3_find_in_index(p_parse_1, p_expr_1, (2 | 1) as u32,
                                if dest_if_false_1 == dest_if_null_1 {
                                    core::ptr::null_mut()
                                } else { &mut r_rhs_has_null }, ai_map, &mut i_tab);
                        __state = 33;
                    }
                    33 => { { let _ = 0; }; __state = 34; }
                    34 => { { let _ = 0; }; __state = 35; }
                    35 => {
                        unsafe {
                            (*p_parse_1).set_ok_const_factor(0 as Bft as u32)
                        };
                        __state = 36;
                    }
                    36 => {
                        r_lhs = expr_code_vector(p_parse_1, p_left, &mut i_dummy);
                        __state = 37;
                    }
                    37 => {
                        unsafe {
                            (*p_parse_1).set_ok_const_factor(ok_const_factor as Bft as
                                    u32)
                        };
                        __state = 38;
                    }
                    38 => {
                        if e_type == 5 { __state = 40; } else { __state = 39; }
                    }
                    39 => {
                        if e_type != 1 { __state = 82; } else { __state = 81; }
                    }
                    40 => { __state = 41; }
                    41 => { __state = 42; }
                    42 => {
                        label_ok = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        __state = 43;
                    }
                    43 => { __state = 44; }
                    44 => { reg_ck_null = 0; __state = 45; }
                    45 => { __state = 46; }
                    46 => { { let _ = 0; }; __state = 47; }
                    47 => { { let _ = 0; }; __state = 48; }
                    48 => {
                        p_list = unsafe { (*p_expr_1).x.p_list };
                        __state = 49;
                    }
                    49 => {
                        p_coll =
                            sqlite3_expr_coll_seq(p_parse_1,
                                unsafe { (*p_expr_1).p_left } as *const Expr);
                        __state = 50;
                    }
                    50 => {
                        if dest_if_null_1 != dest_if_false_1 {
                            __state = 52;
                        } else { __state = 51; }
                    }
                    51 => { ii = 0; __state = 55; }
                    52 => {
                        reg_ck_null =
                            sqlite3_get_temp_reg(unsafe { &mut *p_parse_1 });
                        __state = 53;
                    }
                    53 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 103, r_lhs, r_lhs, reg_ck_null)
                        };
                        __state = 51;
                    }
                    54 => {
                        if reg_ck_null != 0 { __state = 76; } else { __state = 75; }
                    }
                    55 => {
                        if ii < unsafe { (*p_list).n_expr } {
                            __state = 56;
                        } else { __state = 54; }
                    }
                    56 => {
                        r2 =
                            sqlite3_expr_code_temp(p_parse_1,
                                unsafe {
                                    (*(unsafe { (*p_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(ii as isize)).p_expr
                                }, &mut reg_to_free);
                        __state = 58;
                    }
                    57 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 55;
                    }
                    58 => {
                        if reg_ck_null != 0 &&
                                sqlite3_expr_can_be_null(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(ii as isize)).p_expr
                                            } as *const Expr) != 0 {
                            __state = 60;
                        } else { __state = 59; }
                    }
                    59 => {
                        sqlite3_release_temp_reg(unsafe { &mut *p_parse_1 },
                            reg_to_free);
                        __state = 61;
                    }
                    60 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 103, reg_ck_null, r2, reg_ck_null)
                        };
                        __state = 59;
                    }
                    61 => {
                        if ii < unsafe { (*p_list).n_expr } - 1 ||
                                dest_if_null_1 != dest_if_false_1 {
                            __state = 62;
                        } else { __state = 63; }
                    }
                    62 => {
                        op = if r_lhs != r2 { 54 } else { 52 };
                        __state = 64;
                    }
                    63 => {
                        op__1 = if r_lhs != r2 { 53 } else { 51 };
                        __state = 70;
                    }
                    64 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, op, r_lhs, label_ok, r2,
                                p_coll as *mut () as *const i8, -2)
                        };
                        __state = 65;
                    }
                    65 => { __state = 66; }
                    66 => { __state = 67; }
                    67 => { __state = 68; }
                    68 => { __state = 69; }
                    69 => {
                        unsafe {
                            sqlite3_vdbe_change_p5(v,
                                unsafe { *z_aff.offset(0 as isize) } as u16)
                        };
                        __state = 57;
                    }
                    70 => { { let _ = 0; }; __state = 71; }
                    71 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, op__1, r_lhs, dest_if_false_1, r2,
                                p_coll as *mut () as *const i8, -2)
                        };
                        __state = 72;
                    }
                    72 => { __state = 73; }
                    73 => { __state = 74; }
                    74 => {
                        unsafe {
                            sqlite3_vdbe_change_p5(v,
                                (unsafe { *z_aff.offset(0 as isize) } as i32 | 16) as u16)
                        };
                        __state = 57;
                    }
                    75 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, label_ok) };
                        __state = 79;
                    }
                    76 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 51, reg_ck_null, dest_if_null_1)
                        };
                        __state = 77;
                    }
                    77 => { __state = 78; }
                    78 => {
                        unsafe { sqlite3_vdbe_goto(v, dest_if_false_1) };
                        __state = 75;
                    }
                    79 => {
                        sqlite3_release_temp_reg(unsafe { &mut *p_parse_1 },
                            reg_ck_null);
                        __state = 80;
                    }
                    80 => { __state = 2; }
                    81 => {
                        if dest_if_null_1 == dest_if_false_1 {
                            __state = 97;
                        } else { __state = 98; }
                    }
                    82 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 98, r_lhs, n_vector, 0,
                                z_aff as *const i8, n_vector)
                        };
                        __state = 83;
                    }
                    83 => { i = 0; __state = 85; }
                    84 => {
                        if i != n_vector { __state = 88; } else { __state = 81; }
                    }
                    85 => {
                        if i < n_vector &&
                                unsafe { *ai_map.offset(i as isize) } == i {
                            __state = 86;
                        } else { __state = 84; }
                    }
                    86 => { __state = 87; }
                    87 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 85;
                    }
                    88 => { r_lhs_orig = r_lhs; __state = 89; }
                    89 => {
                        r_lhs = sqlite3_get_temp_range(p_parse_1, n_vector);
                        __state = 90;
                    }
                    90 => { i = 0; __state = 92; }
                    91 => {
                        sqlite3_release_temp_reg(unsafe { &mut *p_parse_1 },
                            r_lhs_orig);
                        __state = 81;
                    }
                    92 => {
                        if i < n_vector { __state = 93; } else { __state = 91; }
                    }
                    93 => { __state = 95; }
                    94 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 92;
                    }
                    95 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 82, r_lhs_orig + i,
                                r_lhs + unsafe { *ai_map.offset(i as isize) }, 0)
                        };
                        __state = 94;
                    }
                    96 => { i = 0; __state = 100; }
                    97 => { dest_step2 = dest_if_false_1; __state = 96; }
                    98 => {
                        dest_step2 =
                            {
                                dest_step6 = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                                dest_step6
                            };
                        __state = 96;
                    }
                    99 => {
                        if e_type == 1 { __state = 110; } else { __state = 111; }
                    }
                    100 => {
                        if i < n_vector { __state = 101; } else { __state = 99; }
                    }
                    101 => {
                        p =
                            sqlite3_vector_field_subexpr(unsafe { (*p_expr_1).p_left },
                                    i) as *const Expr;
                        __state = 103;
                    }
                    102 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 100;
                    }
                    103 => {
                        if unsafe { (*p_parse_1).n_err } != 0 {
                            __state = 105;
                        } else { __state = 104; }
                    }
                    104 => {
                        if sqlite3_expr_can_be_null(p as *const Expr) != 0 {
                            __state = 106;
                        } else { __state = 102; }
                    }
                    105 => { __state = 3; }
                    106 => { __state = 107; }
                    107 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 51,
                                r_lhs + unsafe { *ai_map.offset(i as isize) }, dest_step2)
                        };
                        __state = 108;
                    }
                    108 => { __state = 102; }
                    109 => {
                        if r_rhs_has_null != 0 && n_vector == 1 {
                            __state = 128;
                        } else { __state = 127; }
                    }
                    110 => { { let _ = 0; }; __state = 112; }
                    111 => {
                        if dest_if_false_1 == dest_if_null_1 {
                            __state = 116;
                        } else { __state = 115; }
                    }
                    112 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 30, i_tab, dest_if_false_1, r_lhs)
                        };
                        __state = 113;
                    }
                    113 => { __state = 114; }
                    114 => {
                        addr_truth_op = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                        __state = 109;
                    }
                    115 => {
                        addr_truth_op =
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_tab, 0, r_lhs, n_vector)
                            };
                        __state = 126;
                    }
                    116 => {
                        if unsafe { (*p_expr_1).flags } & 33554432 as u32 !=
                                0 as u32 {
                            __state = 118;
                        } else { __state = 117; }
                    }
                    117 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 28, i_tab, dest_if_false_1,
                                r_lhs, n_vector)
                        };
                        __state = 124;
                    }
                    118 => {
                        p_op =
                            unsafe {
                                    sqlite3_vdbe_get_op(v, unsafe { (*p_expr_1).y.sub.i_addr })
                                } as *const VdbeOp;
                        __state = 119;
                    }
                    119 => { { let _ = 0; }; __state = 120; }
                    120 => {
                        if unsafe { (*p_op).p3 } as i32 > 0 {
                            __state = 121;
                        } else { __state = 117; }
                    }
                    121 => { { let _ = 0; }; __state = 122; }
                    122 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 66, unsafe { (*p_op).p3 },
                                dest_if_false_1, r_lhs, n_vector)
                        };
                        __state = 123;
                    }
                    123 => { __state = 117; }
                    124 => { __state = 125; }
                    125 => { __state = 2; }
                    126 => { __state = 109; }
                    127 => {
                        if dest_if_false_1 == dest_if_null_1 {
                            __state = 131;
                        } else { __state = 130; }
                    }
                    128 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 52, r_rhs_has_null, dest_if_false_1)
                        };
                        __state = 129;
                    }
                    129 => { __state = 127; }
                    130 => {
                        if dest_step6 != 0 {
                            __state = 133;
                        } else { __state = 132; }
                    }
                    131 => {
                        unsafe { sqlite3_vdbe_goto(v, dest_if_false_1) };
                        __state = 130;
                    }
                    132 => {
                        addr_top =
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 36, i_tab, dest_if_false_1)
                            };
                        __state = 134;
                    }
                    133 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, dest_step6) };
                        __state = 132;
                    }
                    134 => { __state = 135; }
                    135 => {
                        if n_vector > 1 { __state = 137; } else { __state = 138; }
                    }
                    136 => { i = 0; __state = 140; }
                    137 => {
                        dest_not_null =
                            unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        __state = 136;
                    }
                    138 => { dest_not_null = dest_if_false_1; __state = 136; }
                    139 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 9, 0, dest_if_null_1) };
                        __state = 155;
                    }
                    140 => {
                        if i < n_vector { __state = 141; } else { __state = 139; }
                    }
                    141 => { __state = 143; }
                    142 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 140;
                    }
                    143 => { __state = 144; }
                    144 => {
                        r3 = sqlite3_get_temp_reg(unsafe { &mut *p_parse_1 });
                        __state = 145;
                    }
                    145 => {
                        p__1 = sqlite3_vector_field_subexpr(p_left, i);
                        __state = 146;
                    }
                    146 => {
                        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                            __state = 148;
                        } else { __state = 149; }
                    }
                    147 => { __state = 151; }
                    148 => {
                        p_rhs =
                            unsafe {
                                    (*(unsafe {
                                                        (*unsafe {
                                                                            (*unsafe { (*p_expr_1).x.p_select }).p_e_list
                                                                        }).a.as_ptr()
                                                    } as *mut ExprListItem).offset(i as isize)).p_expr
                                } as *const Expr;
                        __state = 150;
                    }
                    149 => {
                        p_coll_1 =
                            sqlite3_expr_coll_seq(p_parse_1, p__1 as *const Expr);
                        __state = 147;
                    }
                    150 => {
                        p_coll_1 =
                            sqlite3_binary_compare_coll_seq(p_parse_1,
                                p__1 as *const Expr, p_rhs as *const Expr);
                        __state = 147;
                    }
                    151 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_tab,
                                unsafe { *ai_map.offset(i as isize) }, r3)
                        };
                        __state = 152;
                    }
                    152 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 53,
                                r_lhs + unsafe { *ai_map.offset(i as isize) },
                                dest_not_null, r3, p_coll_1 as *mut () as *const i8, -2)
                        };
                        __state = 153;
                    }
                    153 => { __state = 154; }
                    154 => {
                        sqlite3_release_temp_reg(unsafe { &mut *p_parse_1 }, r3);
                        __state = 142;
                    }
                    155 => {
                        if n_vector > 1 { __state = 157; } else { __state = 156; }
                    }
                    156 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_truth_op) };
                        __state = 161;
                    }
                    157 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, dest_not_null) };
                        __state = 158;
                    }
                    158 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 40, i_tab, addr_top + 1) };
                        __state = 159;
                    }
                    159 => { __state = 160; }
                    160 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 9, 0, dest_if_false_1) };
                        __state = 156;
                    }
                    161 => { __state = 2; }
                    162 => { __state = 3; }
                    163 => {
                        unsafe {
                            sqlite3_db_free(unsafe { (*p_parse_1).db },
                                z_aff as *mut ())
                        };
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

///* Convert a scalar expression node to a TK_REGISTER referencing
///* register iReg.  The caller must ensure that iReg already contains
///* the correct value for the expression.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_to_register(p_expr: *mut Expr, i_reg: i32)
    -> () {
    let p: *mut Expr = sqlite3_expr_skip_collate_and_likely(p_expr);
    if p == core::ptr::null_mut() { return; }
    if unsafe { (*p).op } as i32 == 176 {
        { let _ = 0; };
    } else {
        unsafe { (*p).op2 = unsafe { (*p).op } };
        unsafe { (*p).op = 176 as u8 };
        unsafe { (*p).i_table = i_reg };
        unsafe { (*p).flags &= !(8192 as u32) };
    }
}

/// Forward declarations
#[allow(unused_doc_comments)]
extern "C" fn expr_code_between(p_parse: *mut Parse, p_expr: &Expr, dest: i32,
    x_jump:
        Option<unsafe extern "C" fn(*mut Parse, *mut Expr, i32, i32) -> ()>,
    jump_if_null: i32) -> () {
    unsafe {
        let mut expr_and: Expr = unsafe { core::mem::zeroed() };
        /// The AND operator in  x>=y AND x<=z
        let mut comp_left: Expr = unsafe { core::mem::zeroed() };
        /// The  x>=y  term
        let mut comp_right: Expr = unsafe { core::mem::zeroed() };
        /// The  x<=z  term
        let mut reg_free1: i32 = 0;
        /// Temporary use register
        let mut p_del: *mut Expr = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        unsafe {
            memset(&raw mut comp_left as *mut (), 0,
                core::mem::size_of::<Expr>() as u64)
        };
        unsafe {
            memset(&raw mut comp_right as *mut (), 0,
                core::mem::size_of::<Expr>() as u64)
        };
        unsafe {
            memset(&raw mut expr_and as *mut (), 0,
                core::mem::size_of::<Expr>() as u64)
        };
        { let _ = 0; };
        p_del = sqlite3_expr_dup(db, (*p_expr).p_left as *const Expr, 0);
        if unsafe { (*db).malloc_failed } as i32 == 0 {
            expr_and.op = 44 as u8;
            expr_and.p_left = &mut comp_left;
            expr_and.p_right = &mut comp_right;
            comp_left.op = 58 as u8;
            comp_left.p_left = p_del;
            comp_left.p_right =
                unsafe {
                    (*(unsafe { (*(*p_expr).x.p_list).a.as_ptr() } as
                                    *mut ExprListItem).offset(0 as isize)).p_expr
                };
            comp_right.op = 56 as u8;
            comp_right.p_left = p_del;
            comp_right.p_right =
                unsafe {
                    (*(unsafe { (*(*p_expr).x.p_list).a.as_ptr() } as
                                    *mut ExprListItem).offset(1 as isize)).p_expr
                };
            sqlite3_expr_to_register(p_del,
                expr_code_vector(p_parse, p_del, &mut reg_free1));
            if x_jump.is_some() {
                unsafe {
                    x_jump.unwrap()(p_parse, &mut expr_and, dest, jump_if_null)
                };
            } else {

                /// Mark the expression is being from the ON or USING clause of a join
                ///* so that the sqlite3ExprCodeTarget() routine will not attempt to move
                ///* it into the Parse.pConstExpr list.  We should use a new bit for this,
                ///* for clarity, but we are out of bits in the Expr.flags field so we
                ///* have to reuse the EP_OuterON bit.  Bummer.
                unsafe { (*p_del).flags |= 1 as u32 };
                sqlite3_expr_code_target(p_parse, &mut expr_and, dest);
            }
            sqlite3_release_temp_reg(unsafe { &mut *p_parse }, reg_free1);
        }
        sqlite3_expr_delete(db, p_del);
    }
}

///* Generate code for a boolean expression such that a jump is made
///* to the label "dest" if the expression is true but execution
///* continues straight thru if the expression is false.
///*
///* If the expression evaluates to NULL (neither true nor false), then
///* take the jump if the jumpIfNull flag is SQLITE_JUMPIFNULL.
///*
///* This code depends on the fact that certain token values (ex: TK_EQ)
///* are the same as opcode values (ex: OP_Eq) that implement the corresponding
///* operation.  Special comments in vdbe.c and the mkopcodeh.awk script in
///* the make process cause these values to align.  Assert()s in the code
///* below verify that the numbers are aligned correctly.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_if_true(p_parse: *mut Parse, p_expr: *mut Expr,
    dest: i32, mut jump_if_null: i32) -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    let mut op: i32 = 0;
    let mut reg_free1: i32 = 0;
    let mut reg_free2: i32 = 0;
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    /// Existence of VDBE checked by caller
    /// No way this can happen
    let mut p_alt: *mut Expr = core::ptr::null_mut();
    let mut p_first: *mut Expr = core::ptr::null_mut();
    let mut p_second: *mut Expr = core::ptr::null_mut();
    let mut d2: i32 = 0;
    let mut is_not: i32 = 0;
    /// IS NOT TRUE or IS NOT FALSE
    let mut is_true: i32 = 0;
    /// IS TRUE or IS NOT TRUE
    /// no break
    let mut addr_is_null: i32 = 0;
    let mut dest_if_false: i32 = 0;
    let mut dest_if_null: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s48:
            {
            match __state {
                0 => { v = unsafe { (*p_parse).p_vdbe }; __state = 3; }
                2 => {
                    if unsafe { (*p_expr).flags } & (1 | 268435456) as u32 ==
                            268435456 as u32 {
                        __state = 133;
                    } else { __state = 134; }
                }
                3 => { op = 0; __state = 4; }
                4 => { reg_free1 = 0; __state = 5; }
                5 => { reg_free2 = 0; __state = 6; }
                6 => { __state = 7; }
                7 => { { let _ = 0; }; __state = 8; }
                8 => {
                    if v == core::ptr::null_mut() {
                        __state = 10;
                    } else { __state = 9; }
                }
                9 => {
                    if p_expr == core::ptr::null_mut() {
                        __state = 12;
                    } else { __state = 11; }
                }
                10 => { return; }
                11 => { { let _ = 0; }; __state = 13; }
                12 => { return; }
                13 => { op = unsafe { (*p_expr).op } as i32; __state = 14; }
                14 => {
                    '__s49:
                        {
                        match op {
                            44 => { __state = 16; }
                            43 => { __state = 17; }
                            19 => { __state = 18; }
                            175 => { __state = 19; }
                            45 => { __state = 20; }
                            46 => { __state = 21; }
                            57 => { __state = 22; }
                            56 => { __state = 23; }
                            55 => { __state = 24; }
                            58 => { __state = 25; }
                            53 => { __state = 26; }
                            54 => { __state = 27; }
                            51 => { __state = 28; }
                            52 => { __state = 29; }
                            49 => { __state = 30; }
                            50 => { __state = 31; }
                            _ => { __state = 32; }
                        }
                    }
                }
                15 => {
                    sqlite3_release_temp_reg(unsafe { &mut *p_parse },
                        reg_free1);
                    __state = 141;
                }
                16 => { __state = 17; }
                17 => {
                    p_alt = sqlite3_expr_simplified_and_or(p_expr);
                    __state = 35;
                }
                18 => { __state = 55; }
                19 => { __state = 58; }
                20 => { __state = 21; }
                21 => { __state = 68; }
                22 => { __state = 23; }
                23 => { __state = 24; }
                24 => { __state = 25; }
                25 => { __state = 26; }
                26 => { __state = 27; }
                27 => { __state = 74; }
                28 => { __state = 29; }
                29 => { { let _ = 0; }; __state = 111; }
                30 => { __state = 124; }
                31 => {
                    dest_if_false = unsafe { sqlite3_vdbe_make_label(p_parse) };
                    __state = 127;
                }
                32 => { __state = 2; }
                33 => { __state = 16; }
                34 => { __state = 53; }
                35 => {
                    if p_alt != p_expr { __state = 37; } else { __state = 38; }
                }
                36 => { __state = 15; }
                37 => {
                    sqlite3_expr_if_true(p_parse, p_alt, dest, jump_if_null);
                    __state = 36;
                }
                38 => { __state = 39; }
                39 => {
                    if expr_eval_rhs_first(unsafe { &*p_expr }) != 0 {
                        __state = 41;
                    } else { __state = 42; }
                }
                40 => { if op == 44 { __state = 45; } else { __state = 46; } }
                41 => {
                    p_first = unsafe { (*p_expr).p_right };
                    __state = 43;
                }
                42 => { p_first = unsafe { (*p_expr).p_left }; __state = 44; }
                43 => {
                    p_second = unsafe { (*p_expr).p_left };
                    __state = 40;
                }
                44 => {
                    p_second = unsafe { (*p_expr).p_right };
                    __state = 40;
                }
                45 => {
                    d2 = unsafe { sqlite3_vdbe_make_label(p_parse) };
                    __state = 47;
                }
                46 => { __state = 51; }
                47 => { __state = 48; }
                48 => {
                    sqlite3_expr_if_false(p_parse, p_first, d2,
                        jump_if_null ^ 16);
                    __state = 49;
                }
                49 => {
                    sqlite3_expr_if_true(p_parse, p_second, dest, jump_if_null);
                    __state = 50;
                }
                50 => {
                    unsafe { sqlite3_vdbe_resolve_label(v, d2) };
                    __state = 36;
                }
                51 => {
                    sqlite3_expr_if_true(p_parse, p_first, dest, jump_if_null);
                    __state = 52;
                }
                52 => {
                    sqlite3_expr_if_true(p_parse, p_second, dest, jump_if_null);
                    __state = 36;
                }
                53 => { __state = 18; }
                54 => { __state = 19; }
                55 => {
                    sqlite3_expr_if_false(p_parse, unsafe { (*p_expr).p_left },
                        dest, jump_if_null);
                    __state = 56;
                }
                56 => { __state = 15; }
                57 => { __state = 20; }
                58 => { __state = 59; }
                59 => { __state = 60; }
                60 => {
                    is_not = (unsafe { (*p_expr).op2 } as i32 == 46) as i32;
                    __state = 61;
                }
                61 => {
                    is_true =
                        sqlite3_expr_truth_value(unsafe { (*p_expr).p_right } as
                                *const Expr);
                    __state = 62;
                }
                62 => { __state = 63; }
                63 => { __state = 64; }
                64 => {
                    if is_true ^ is_not != 0 {
                        __state = 66;
                    } else { __state = 67; }
                }
                65 => { __state = 15; }
                66 => {
                    sqlite3_expr_if_true(p_parse, unsafe { (*p_expr).p_left },
                        dest, if is_not != 0 { 16 } else { 0 });
                    __state = 65;
                }
                67 => {
                    sqlite3_expr_if_false(p_parse, unsafe { (*p_expr).p_left },
                        dest, if is_not != 0 { 16 } else { 0 });
                    __state = 65;
                }
                68 => { __state = 69; }
                69 => { op = if op == 45 { 54 } else { 53 }; __state = 70; }
                70 => { jump_if_null = 128; __state = 71; }
                71 => { __state = 72; }
                72 => { __state = 22; }
                73 => { __state = 109; }
                74 => {
                    if sqlite3_expr_is_vector(unsafe { (*p_expr).p_left } as
                                    *const Expr) != 0 {
                        __state = 76;
                    } else { __state = 75; }
                }
                75 => {
                    if unsafe { (*p_expr).flags } & 4194304 as u32 != 0 as u32
                            && jump_if_null != 128 {
                        __state = 78;
                    } else { __state = 79; }
                }
                76 => { __state = 2; }
                77 => {
                    code_compare(p_parse,
                        unsafe { (*p_expr).p_left } as *const Expr,
                        unsafe { (*p_expr).p_right } as *const Expr, op, r1, r2,
                        dest, jump_if_null,
                        (unsafe { (*p_expr).flags } & 1024 as u32 != 0 as u32) as
                            i32);
                    __state = 82;
                }
                78 => {
                    addr_is_null =
                        expr_compute_operands(p_parse, p_expr, &mut r1, &mut r2,
                            &mut reg_free1, &mut reg_free2);
                    __state = 77;
                }
                79 => {
                    r1 =
                        sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                            &mut reg_free1);
                    __state = 80;
                }
                80 => {
                    r2 =
                        sqlite3_expr_code_temp(p_parse,
                            unsafe { (*p_expr).p_right }, &mut reg_free2);
                    __state = 81;
                }
                81 => { addr_is_null = 0; __state = 77; }
                82 => { { let _ = 0; }; __state = 83; }
                83 => { __state = 84; }
                84 => { __state = 85; }
                85 => { { let _ = 0; }; __state = 86; }
                86 => { __state = 87; }
                87 => { __state = 88; }
                88 => { { let _ = 0; }; __state = 89; }
                89 => { __state = 90; }
                90 => { __state = 91; }
                91 => { { let _ = 0; }; __state = 92; }
                92 => { __state = 93; }
                93 => { __state = 94; }
                94 => { { let _ = 0; }; __state = 95; }
                95 => { __state = 96; }
                96 => { __state = 97; }
                97 => { __state = 98; }
                98 => { { let _ = 0; }; __state = 99; }
                99 => { __state = 100; }
                100 => { __state = 101; }
                101 => { __state = 102; }
                102 => { __state = 103; }
                103 => { __state = 104; }
                104 => {
                    if addr_is_null != 0 {
                        __state = 106;
                    } else { __state = 105; }
                }
                105 => { __state = 15; }
                106 => {
                    if jump_if_null != 0 {
                        __state = 107;
                    } else { __state = 108; }
                }
                107 => {
                    unsafe { sqlite3_vdbe_change_p2(v, addr_is_null, dest) };
                    __state = 105;
                }
                108 => {
                    unsafe { sqlite3_vdbe_jump_here(v, addr_is_null) };
                    __state = 105;
                }
                109 => { __state = 28; }
                110 => { __state = 122; }
                111 => { __state = 112; }
                112 => { { let _ = 0; }; __state = 113; }
                113 => { __state = 114; }
                114 => {
                    r1 =
                        sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                            &mut reg_free1);
                    __state = 115;
                }
                115 => { { let _ = 0; }; __state = 116; }
                116 => {
                    if reg_free1 != 0 { __state = 118; } else { __state = 117; }
                }
                117 => {
                    unsafe { sqlite3_vdbe_add_op2(v, op, r1, dest) };
                    __state = 119;
                }
                118 => {
                    unsafe { sqlite3_vdbe_typeof_column(v, r1) };
                    __state = 117;
                }
                119 => { __state = 120; }
                120 => { __state = 121; }
                121 => { __state = 15; }
                122 => { __state = 30; }
                123 => { __state = 31; }
                124 => {
                    expr_code_between(p_parse, unsafe { &*p_expr }, dest,
                        Some(sqlite3_expr_if_true), jump_if_null);
                    __state = 125;
                }
                125 => { __state = 15; }
                126 => { __state = 32; }
                127 => {
                    dest_if_null =
                        if jump_if_null != 0 { dest } else { dest_if_false };
                    __state = 128;
                }
                128 => {
                    sqlite3_expr_code_in(p_parse, p_expr, dest_if_false,
                        dest_if_null);
                    __state = 129;
                }
                129 => {
                    unsafe { sqlite3_vdbe_goto(v, dest) };
                    __state = 130;
                }
                130 => {
                    unsafe { sqlite3_vdbe_resolve_label(v, dest_if_false) };
                    __state = 131;
                }
                131 => { __state = 15; }
                132 => { __state = 15; }
                133 => {
                    unsafe { sqlite3_vdbe_goto(v, dest) };
                    __state = 132;
                }
                134 => {
                    if unsafe { (*p_expr).flags } & (1 | 536870912) as u32 ==
                            536870912 as u32 {
                        __state = 135;
                    } else { __state = 136; }
                }
                135 => { __state = 132; }
                136 => {
                    r1 =
                        sqlite3_expr_code_temp(p_parse, p_expr, &mut reg_free1);
                    __state = 137;
                }
                137 => {
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 16, r1, dest,
                            (jump_if_null != 0) as i32)
                    };
                    __state = 138;
                }
                138 => { __state = 139; }
                139 => { __state = 140; }
                140 => { __state = 132; }
                141 => {
                    sqlite3_release_temp_reg(unsafe { &mut *p_parse },
                        reg_free2);
                    __state = 1;
                }
                _ => {}
            }
        }
    }
}

///* Generate code for a boolean expression such that a jump is made
///* to the label "dest" if the expression is false but execution
///* continues straight thru if the expression is true.
///*
///* If the expression evaluates to NULL (neither true nor false) then
///* jump if jumpIfNull is SQLITE_JUMPIFNULL or fall through if jumpIfNull
///* is 0.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_if_false(p_parse: *mut Parse,
    p_expr: *mut Expr, dest: i32, mut jump_if_null: i32) -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    let mut op: i32 = 0;
    let mut reg_free1: i32 = 0;
    let mut reg_free2: i32 = 0;
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    /// Existence of VDBE checked by caller
    /// The value of pExpr->op and op are related as follows:
    ///*
    ///*       pExpr->op            op
    ///*       ---------          ----------
    ///*       TK_ISNULL          OP_NotNull
    ///*       TK_NOTNULL         OP_IsNull
    ///*       TK_NE              OP_Eq
    ///*       TK_EQ              OP_Ne
    ///*       TK_GT              OP_Le
    ///*       TK_LE              OP_Gt
    ///*       TK_GE              OP_Lt
    ///*       TK_LT              OP_Ge
    ///*
    ///* For other values of pExpr->op, op is undefined and unused.
    ///* The value of TK_ and OP_ constants are arranged such that we
    ///* can compute the mapping above using the following expression.
    ///* Assert()s verify that the computation is correct.
    /// Verify correct alignment of TK_ and OP_ constants
    let mut p_alt: *mut Expr = core::ptr::null_mut();
    let mut p_first: *mut Expr = core::ptr::null_mut();
    let mut p_second: *mut Expr = core::ptr::null_mut();
    let mut d2: i32 = 0;
    let mut is_not: i32 = 0;
    /// IS NOT TRUE or IS NOT FALSE
    let mut is_true: i32 = 0;
    /// IS TRUE or IS NOT TRUE
    /// IS TRUE and IS NOT FALSE
    /// IS FALSE and IS NOT TRUE
    /// no break
    let mut addr_is_null: i32 = 0;
    let mut dest_if_null: i32 = 0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s51:
            {
            match __state {
                0 => { v = unsafe { (*p_parse).p_vdbe }; __state = 3; }
                2 => {
                    if unsafe { (*p_expr).flags } & (1 | 536870912) as u32 ==
                            536870912 as u32 {
                        __state = 139;
                    } else { __state = 140; }
                }
                3 => { op = 0; __state = 4; }
                4 => { reg_free1 = 0; __state = 5; }
                5 => { reg_free2 = 0; __state = 6; }
                6 => { __state = 7; }
                7 => { { let _ = 0; }; __state = 8; }
                8 => {
                    if v == core::ptr::null_mut() {
                        __state = 10;
                    } else { __state = 9; }
                }
                9 => {
                    if p_expr == core::ptr::null_mut() {
                        __state = 12;
                    } else { __state = 11; }
                }
                10 => { return; }
                11 => { { let _ = 0; }; __state = 13; }
                12 => { return; }
                13 => {
                    op =
                        (unsafe { (*p_expr).op } as i32 + (51 & 1) ^ 1) - (51 & 1);
                    __state = 14;
                }
                14 => { { let _ = 0; }; __state = 15; }
                15 => { { let _ = 0; }; __state = 16; }
                16 => { { let _ = 0; }; __state = 17; }
                17 => { { let _ = 0; }; __state = 18; }
                18 => { { let _ = 0; }; __state = 19; }
                19 => { { let _ = 0; }; __state = 20; }
                20 => { { let _ = 0; }; __state = 21; }
                21 => { { let _ = 0; }; __state = 22; }
                22 => {
                    '__s52:
                        {
                        match unsafe { (*p_expr).op } {
                            44 => { __state = 24; }
                            43 => { __state = 25; }
                            19 => { __state = 26; }
                            175 => { __state = 27; }
                            45 => { __state = 28; }
                            46 => { __state = 29; }
                            57 => { __state = 30; }
                            56 => { __state = 31; }
                            55 => { __state = 32; }
                            58 => { __state = 33; }
                            53 => { __state = 34; }
                            54 => { __state = 35; }
                            51 => { __state = 36; }
                            52 => { __state = 37; }
                            49 => { __state = 38; }
                            50 => { __state = 39; }
                            _ => { __state = 40; }
                        }
                    }
                }
                23 => {
                    sqlite3_release_temp_reg(unsafe { &mut *p_parse },
                        reg_free1);
                    __state = 147;
                }
                24 => { __state = 25; }
                25 => {
                    p_alt = sqlite3_expr_simplified_and_or(p_expr);
                    __state = 43;
                }
                26 => { __state = 63; }
                27 => { __state = 66; }
                28 => { __state = 29; }
                29 => { __state = 76; }
                30 => { __state = 31; }
                31 => { __state = 32; }
                32 => { __state = 33; }
                33 => { __state = 34; }
                34 => { __state = 35; }
                35 => { __state = 82; }
                36 => { __state = 37; }
                37 => {
                    r1 =
                        sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                            &mut reg_free1);
                    __state = 119;
                }
                38 => { __state = 130; }
                39 => {
                    if jump_if_null != 0 {
                        __state = 134;
                    } else { __state = 135; }
                }
                40 => { __state = 2; }
                41 => { __state = 24; }
                42 => { __state = 61; }
                43 => {
                    if p_alt != p_expr { __state = 45; } else { __state = 46; }
                }
                44 => { __state = 23; }
                45 => {
                    sqlite3_expr_if_false(p_parse, p_alt, dest, jump_if_null);
                    __state = 44;
                }
                46 => { __state = 47; }
                47 => {
                    if expr_eval_rhs_first(unsafe { &*p_expr }) != 0 {
                        __state = 49;
                    } else { __state = 50; }
                }
                48 => {
                    if unsafe { (*p_expr).op } as i32 == 44 {
                        __state = 53;
                    } else { __state = 54; }
                }
                49 => {
                    p_first = unsafe { (*p_expr).p_right };
                    __state = 51;
                }
                50 => { p_first = unsafe { (*p_expr).p_left }; __state = 52; }
                51 => {
                    p_second = unsafe { (*p_expr).p_left };
                    __state = 48;
                }
                52 => {
                    p_second = unsafe { (*p_expr).p_right };
                    __state = 48;
                }
                53 => { __state = 55; }
                54 => {
                    d2 = unsafe { sqlite3_vdbe_make_label(p_parse) };
                    __state = 57;
                }
                55 => {
                    sqlite3_expr_if_false(p_parse, p_first, dest, jump_if_null);
                    __state = 56;
                }
                56 => {
                    sqlite3_expr_if_false(p_parse, p_second, dest,
                        jump_if_null);
                    __state = 44;
                }
                57 => { __state = 58; }
                58 => {
                    sqlite3_expr_if_true(p_parse, p_first, d2,
                        jump_if_null ^ 16);
                    __state = 59;
                }
                59 => {
                    sqlite3_expr_if_false(p_parse, p_second, dest,
                        jump_if_null);
                    __state = 60;
                }
                60 => {
                    unsafe { sqlite3_vdbe_resolve_label(v, d2) };
                    __state = 44;
                }
                61 => { __state = 26; }
                62 => { __state = 27; }
                63 => {
                    sqlite3_expr_if_true(p_parse, unsafe { (*p_expr).p_left },
                        dest, jump_if_null);
                    __state = 64;
                }
                64 => { __state = 23; }
                65 => { __state = 28; }
                66 => { __state = 67; }
                67 => { __state = 68; }
                68 => {
                    is_not = (unsafe { (*p_expr).op2 } as i32 == 46) as i32;
                    __state = 69;
                }
                69 => {
                    is_true =
                        sqlite3_expr_truth_value(unsafe { (*p_expr).p_right } as
                                *const Expr);
                    __state = 70;
                }
                70 => { __state = 71; }
                71 => { __state = 72; }
                72 => {
                    if is_true ^ is_not != 0 {
                        __state = 74;
                    } else { __state = 75; }
                }
                73 => { __state = 23; }
                74 => {
                    sqlite3_expr_if_false(p_parse, unsafe { (*p_expr).p_left },
                        dest, if is_not != 0 { 0 } else { 16 });
                    __state = 73;
                }
                75 => {
                    sqlite3_expr_if_true(p_parse, unsafe { (*p_expr).p_left },
                        dest, if is_not != 0 { 0 } else { 16 });
                    __state = 73;
                }
                76 => { __state = 77; }
                77 => {
                    op =
                        if unsafe { (*p_expr).op } as i32 == 45 { 53 } else { 54 };
                    __state = 78;
                }
                78 => { jump_if_null = 128; __state = 79; }
                79 => { __state = 80; }
                80 => { __state = 30; }
                81 => { __state = 117; }
                82 => {
                    if sqlite3_expr_is_vector(unsafe { (*p_expr).p_left } as
                                    *const Expr) != 0 {
                        __state = 84;
                    } else { __state = 83; }
                }
                83 => {
                    if unsafe { (*p_expr).flags } & 4194304 as u32 != 0 as u32
                            && jump_if_null != 128 {
                        __state = 86;
                    } else { __state = 87; }
                }
                84 => { __state = 2; }
                85 => {
                    code_compare(p_parse,
                        unsafe { (*p_expr).p_left } as *const Expr,
                        unsafe { (*p_expr).p_right } as *const Expr, op, r1, r2,
                        dest, jump_if_null,
                        (unsafe { (*p_expr).flags } & 1024 as u32 != 0 as u32) as
                            i32);
                    __state = 90;
                }
                86 => {
                    addr_is_null =
                        expr_compute_operands(p_parse, p_expr, &mut r1, &mut r2,
                            &mut reg_free1, &mut reg_free2);
                    __state = 85;
                }
                87 => {
                    r1 =
                        sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                            &mut reg_free1);
                    __state = 88;
                }
                88 => {
                    r2 =
                        sqlite3_expr_code_temp(p_parse,
                            unsafe { (*p_expr).p_right }, &mut reg_free2);
                    __state = 89;
                }
                89 => { addr_is_null = 0; __state = 85; }
                90 => { { let _ = 0; }; __state = 91; }
                91 => { __state = 92; }
                92 => { __state = 93; }
                93 => { { let _ = 0; }; __state = 94; }
                94 => { __state = 95; }
                95 => { __state = 96; }
                96 => { { let _ = 0; }; __state = 97; }
                97 => { __state = 98; }
                98 => { __state = 99; }
                99 => { { let _ = 0; }; __state = 100; }
                100 => { __state = 101; }
                101 => { __state = 102; }
                102 => { { let _ = 0; }; __state = 103; }
                103 => { __state = 104; }
                104 => { __state = 105; }
                105 => { __state = 106; }
                106 => { { let _ = 0; }; __state = 107; }
                107 => { __state = 108; }
                108 => { __state = 109; }
                109 => { __state = 110; }
                110 => { __state = 111; }
                111 => { __state = 112; }
                112 => {
                    if addr_is_null != 0 {
                        __state = 114;
                    } else { __state = 113; }
                }
                113 => { __state = 23; }
                114 => {
                    if jump_if_null != 0 {
                        __state = 115;
                    } else { __state = 116; }
                }
                115 => {
                    unsafe { sqlite3_vdbe_change_p2(v, addr_is_null, dest) };
                    __state = 113;
                }
                116 => {
                    unsafe { sqlite3_vdbe_jump_here(v, addr_is_null) };
                    __state = 113;
                }
                117 => { __state = 36; }
                118 => { __state = 128; }
                119 => { { let _ = 0; }; __state = 120; }
                120 => {
                    if reg_free1 != 0 { __state = 122; } else { __state = 121; }
                }
                121 => {
                    unsafe { sqlite3_vdbe_add_op2(v, op, r1, dest) };
                    __state = 123;
                }
                122 => {
                    unsafe { sqlite3_vdbe_typeof_column(v, r1) };
                    __state = 121;
                }
                123 => { __state = 124; }
                124 => { __state = 125; }
                125 => { __state = 126; }
                126 => { __state = 127; }
                127 => { __state = 23; }
                128 => { __state = 38; }
                129 => { __state = 39; }
                130 => {
                    expr_code_between(p_parse, unsafe { &*p_expr }, dest,
                        Some(sqlite3_expr_if_false), jump_if_null);
                    __state = 131;
                }
                131 => { __state = 23; }
                132 => { __state = 40; }
                133 => { __state = 23; }
                134 => {
                    sqlite3_expr_code_in(p_parse, p_expr, dest, dest);
                    __state = 133;
                }
                135 => {
                    dest_if_null = unsafe { sqlite3_vdbe_make_label(p_parse) };
                    __state = 136;
                }
                136 => {
                    sqlite3_expr_code_in(p_parse, p_expr, dest, dest_if_null);
                    __state = 137;
                }
                137 => {
                    unsafe { sqlite3_vdbe_resolve_label(v, dest_if_null) };
                    __state = 133;
                }
                138 => { __state = 23; }
                139 => {
                    unsafe { sqlite3_vdbe_goto(v, dest) };
                    __state = 138;
                }
                140 => {
                    if unsafe { (*p_expr).flags } & (1 | 268435456) as u32 ==
                            268435456 as u32 {
                        __state = 141;
                    } else { __state = 142; }
                }
                141 => { __state = 138; }
                142 => {
                    r1 =
                        sqlite3_expr_code_temp(p_parse, p_expr, &mut reg_free1);
                    __state = 143;
                }
                143 => {
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 17, r1, dest,
                            (jump_if_null != 0) as i32)
                    };
                    __state = 144;
                }
                144 => { __state = 145; }
                145 => { __state = 146; }
                146 => { __state = 138; }
                147 => {
                    sqlite3_release_temp_reg(unsafe { &mut *p_parse },
                        reg_free2);
                    __state = 1;
                }
                _ => {}
            }
        }
    }
}

///* Generate code into the current Vdbe to evaluate the given
///* expression.  Attempt to store the results in register "target".
///* Return the register where results are stored.
///*
///* With this routine, there is no guarantee that results will
///* be stored in target.  The result might be stored in some other
///* register if it is convenient to do so.  The calling function
///* must check the return code and move the results to the desired
///* register.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_code_target(p_parse: *mut Parse,
    mut p_expr: *mut Expr, target: i32) -> i32 {
    unsafe {
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// The VM under construction
        let mut op: i32 = 0;
        /// The opcode being coded
        let mut in_reg: i32 = 0;
        /// Results stored in register inReg
        let mut reg_free1: i32 = 0;
        /// If non-zero free this temporary register
        let mut reg_free2: i32 = 0;
        /// If non-zero free this temporary register
        let mut r1: i32 = 0;
        let mut r2: i32 = 0;
        /// Various register numbers
        let mut temp_x: Expr = unsafe { core::mem::zeroed() };
        /// Temporary expression node
        let mut p5: i32 = 0;
        let mut p_agg_info: *const AggInfo = core::ptr::null();
        let mut p_col: *const AggInfoCol = core::ptr::null();
        /// Happens when the left table of a RIGHT JOIN is null and
        ///* is using an expression index
        let mut p_tab: *const Table = core::ptr::null();
        /// No comment added
        /// This case happens when the argument to an aggregate function
        ///* is rewritten by aggregateConvertIndexedExprRefToColumn()
        /// Otherwise, fall thru into the TK_COLUMN case */
        ///      /* no break
        let mut i_tab: i32 = 0;
        let mut i_reg: i32 = 0;
        /// This COLUMN expression is really a constant due to WHERE clause
        ///* constraints, and that constant is coded by the pExpr->pLeft
        ///* expression.  However, make sure the constant has the correct
        ///* datatype by applying the Affinity of the table column to the
        ///* constant.
        let mut aff: i32 = 0;
        /// Other columns in the same row for CHECK constraints or
        ///* generated columns or for inserting into partial index.
        ///* The row is unpacked into registers beginning at
        ///* 0-(pParse->iSelfTab).  The rowid (if any) is in a register
        ///* immediately prior to the first column.
        let mut p_col_1: *mut Column = core::ptr::null_mut();
        let mut p_tab_1: *mut Table = core::ptr::null_mut();
        let mut i_src: i32 = 0;
        let mut i_col: i32 = 0;
        /// SQLITE_OMIT_GENERATED_COLUMNS
        /// Coding an expression that is part of an index where column names
        ///* in the index refer to the table to which the index belongs
        /// Set a range of registers to NULL.  pExpr->y.nReg registers starting
        ///* with target
        /// Make NULL the default case so that if a bug causes an illegal
        ///* Expr node to be passed into this function, it will be handled
        ///* sanely and not crash.  But keep the assert() to bring the problem
        ///* to the attention of the developers.
        let mut n: i32 = 0;
        let mut z: *const i8 = core::ptr::null();
        let mut z_blob: *const i8 = core::ptr::null();
        /// Expressions of the form:   CAST(pLeft AS token)
        /// SQLITE_OMIT_CAST
        /// no break
        let mut p_left: *mut Expr = core::ptr::null_mut();
        let mut addr_is_null: i32 = 0;
        let mut addr_is_null_1: i32 = 0;
        let mut p_left_1: *mut Expr = core::ptr::null_mut();
        let mut is_true: i32 = 0;
        /// IS TRUE or IS NOT TRUE
        let mut b_normal: i32 = 0;
        /// IS TRUE or IS FALSE
        let mut addr: i32 = 0;
        let mut p_info: *const AggInfo = core::ptr::null();
        let mut p_farg: *mut ExprList = core::ptr::null_mut();
        /// List of function arguments
        let mut n_farg: i32 = 0;
        /// Number of function arguments
        let mut p_def: *mut FuncDef = core::ptr::null_mut();
        /// The function definition object
        let mut z_id: *const i8 = core::ptr::null();
        /// The function name
        let mut const_mask: u32 = 0 as u32;
        /// Mask of function arguments that are constant
        let mut i: i32 = 0;
        /// Loop counter
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// The database connection
        let mut enc: u8 = 0 as u8;
        /// The text encoding used by this database
        let mut p_coll: *mut CollSeq = core::ptr::null_mut();
        /// A collating sequence
        /// SQL functions can be expensive. So try to avoid running them
        ///* multiple times if we know they always give the same result
        /// For length() and typeof() and octet_length() functions,
        ///* set the P5 parameter to the OP_Column opcode to OPFLAG_LENGTHARG
        ///* or OPFLAG_TYPEOFARG or OPFLAG_BYTELENARG respectively, to avoid
        ///* unnecessary data loading.
        let mut expr_op: u8 = 0 as u8;
        /// Possibly overload the function if the first argument is
        ///* a virtual table column.
        ///*
        ///* For infix functions (LIKE, GLOB, REGEXP, and MATCH) use the
        ///* second argument, not the first, as the argument to test to
        ///* see if it is a column in a virtual table.  This is done because
        ///* the left operand of infix functions (the operand we want to
        ///* control overloading) ends up as the second argument to the
        ///* function.  The expression "A glob B" is equivalent to
        ///* "glob(B,A).  We want to use the A in "A glob B" to test
        ///* for function overloading.  But we use the B term in "glob(B,A)".
        let mut n_col: i32 = 0;
        let mut n__1: i32 = 0;
        let mut p_left_2: *mut Expr = core::ptr::null_mut();
        let mut dest_if_false: i32 = 0;
        let mut dest_if_null: i32 = 0;
        /// SQLITE_OMIT_SUBQUERY
        ///*    x BETWEEN y AND z
        ///*
        ///* This is equivalent to
        ///*
        ///*    x>=y AND x<=z
        ///*
        ///* X is stored in pExpr->pLeft.
        ///* Y is stored in pExpr->pList->a[0].pExpr.
        ///* Z is stored in pExpr->pList->a[1].pExpr.
        /// A TK_COLLATE Expr node without the EP_Collate tag is a so-called
        ///* "SOFT-COLLATE" that is added to constraints that are pushed down
        ///* from outer queries into sub-queries by the WHERE-clause push-down
        ///* optimization. Clear subtypes as subtypes may not cross a subquery
        ///* boundary.
        /// 2018-04-28: Prevent deep recursion.
        /// 2018-04-28: Prevent deep recursion. OSSFuzz.
        /// If the opcode is TK_TRIGGER, then the expression is a reference
        ///* to a column in the new.* or old.* pseudo-tables available to
        ///* trigger programs. In this case Expr.iTable is set to 1 for the
        ///* new.* pseudo-table, or 0 for the old.* pseudo-table. Expr.iColumn
        ///* is set to the column of the pseudo-table to read, or to -1 to
        ///* read the rowid field.
        ///*
        ///* The expression is implemented using an OP_Param opcode. The p1
        ///* parameter is set to 0 for an old.rowid reference, or to (i+1)
        ///* to reference another column of the old.* pseudo-table, where
        ///* i is the index of the column. For a new.rowid reference, p1 is
        ///* set to (n+1), where n is the number of columns in each pseudo-table.
        ///* For a reference to any other column in the new.* pseudo-table, p1
        ///* is set to (n+2+i), where n and i are as defined previously. For
        ///* example, if the table on which triggers are being fired is
        ///* declared as:
        ///*
        ///*   CREATE TABLE t1(a, b);
        ///*
        ///* Then p1 is interpreted as follows:
        ///*
        ///*   p1==0   ->    old.rowid     p1==3   ->    new.rowid
        ///*   p1==1   ->    old.a         p1==4   ->    new.a
        ///*   p1==2   ->    old.b         p1==5   ->    new.b
        let mut p_tab_2: *mut Table = core::ptr::null_mut();
        let mut i_col_1: i32 = 0;
        let mut p1: i32 = 0;
        /// If the column has REAL affinity, it may currently be stored as an
        ///* integer. Use OP_RealAffinity to make sure it is really real.
        ///*
        ///* EVIDENCE-OF: R-60985-57662 SQLite will convert the value back to
        ///* floating point when extracting it from the record.
        /// TK_IF_NULL_ROW Expr nodes are inserted ahead of expressions
        ///* that derive from the right-hand table of a LEFT JOIN.  The
        ///* Expr.iTable value is the table number for the right-hand table.
        ///* The expression is only evaluated if that table is not currently
        ///* on a LEFT JOIN NULL row.
        let mut addr_inr: i32 = 0;
        let mut ok_const_factor: u8 = 0 as u8;
        let mut p_agg_info_1: *const AggInfo = core::ptr::null();
        /// The OP_IfNullRow opcode above can overwrite the result register with
        ///* NULL.  So we have to ensure that the result register is not a value
        ///* that is suppose to be a constant.  Two defenses are needed:
        ///*   (1)  Temporarily disable factoring of constant expressions
        ///*   (2)  Make sure the computed value really is stored in register
        ///*        "target" and not someplace else.
        /// note (1) above
        ///* Form A:
        ///*   CASE x WHEN e1 THEN r1 WHEN e2 THEN r2 ... WHEN eN THEN rN ELSE y END
        ///*
        ///* Form B:
        ///*   CASE WHEN e1 THEN r1 WHEN e2 THEN r2 ... WHEN eN THEN rN ELSE y END
        ///*
        ///* Form A is can be transformed into the equivalent form B as follows:
        ///*   CASE WHEN x=e1 THEN r1 WHEN x=e2 THEN r2 ...
        ///*        WHEN x=eN THEN rN ELSE y END
        ///*
        ///* X (if it exists) is in pExpr->pLeft.
        ///* Y is in the last element of pExpr->x.pList if pExpr->x.pList->nExpr is
        ///* odd.  The Y is also optional.  If the number of elements in x.pList
        ///* is even, then Y is omitted and the "otherwise" result is NULL.
        ///* Ei is in pExpr->pList->a[i*2] and Ri is pExpr->pList->a[i*2+1].
        ///*
        ///* The result of the expression is the Ri for the first matching Ei,
        ///* or if there is no matching Ei, the ELSE term Y, or if there is
        ///* no ELSE term, NULL.
        let mut end_label: i32 = 0;
        /// GOTO label for end of CASE stmt
        let mut next_case: i32 = 0;
        /// GOTO label for next WHEN clause
        let mut n_expr: i32 = 0;
        /// 2x number of WHEN terms
        let mut i__1: i32 = 0;
        /// Loop counter
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        /// List of WHEN terms
        let mut a_listelem: *const ExprListItem = core::ptr::null();
        /// Array of WHEN terms
        let mut op_compare: Expr = unsafe { core::mem::zeroed() };
        /// The X==Ei expression
        let mut p_x: *const Expr = core::ptr::null();
        /// The X expression
        let mut p_test: *mut Expr = core::ptr::null_mut();
        /// X==Ei (form A) or just Ei (form B)
        let mut p_del: *mut Expr = core::ptr::null_mut();
        let mut db__1: *mut Sqlite3 = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s54:
                {
                match __state {
                    0 => { v = unsafe { (*p_parse).p_vdbe }; __state = 3; }
                    2 => {
                        if p_expr == core::ptr::null_mut() {
                            __state = 14;
                        } else { __state = 15; }
                    }
                    3 => { __state = 4; }
                    4 => { in_reg = target; __state = 5; }
                    5 => { reg_free1 = 0; __state = 6; }
                    6 => { reg_free2 = 0; __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { p5 = 0; __state = 10; }
                    10 => { { let _ = 0; }; __state = 11; }
                    11 => { { let _ = 0; }; __state = 12; }
                    12 => { __state = 2; }
                    13 => { { let _ = 0; }; __state = 19; }
                    14 => { op = 122; __state = 13; }
                    15 => {
                        if unsafe { (*p_parse).p_idx_epr } != core::ptr::null_mut()
                                    &&
                                    !(unsafe { (*p_expr).flags } & 8388608 as u32 != 0 as u32)
                                            as i32 != 0 &&
                                {
                                        r1 = sqlite3_indexed_expr_lookup(p_parse, p_expr, target);
                                        r1
                                    } >= 0 {
                            __state = 16;
                        } else { __state = 17; }
                    }
                    16 => { return r1; }
                    17 => { { let _ = 0; }; __state = 18; }
                    18 => { op = unsafe { (*p_expr).op } as i32; __state = 13; }
                    19 => {
                        '__s55:
                            {
                            match op {
                                170 => { __state = 21; }
                                168 => { __state = 22; }
                                156 => { __state = 23; }
                                171 => { __state = 24; }
                                154 => { __state = 25; }
                                118 => { __state = 26; }
                                83 => { __state = 27; }
                                155 => { __state = 29; }
                                157 => { __state = 30; }
                                176 => { __state = 31; }
                                36 => { __state = 32; }
                                45 => { __state = 33; }
                                46 => { __state = 34; }
                                57 => { __state = 35; }
                                56 => { __state = 36; }
                                55 => { __state = 37; }
                                58 => { __state = 38; }
                                53 => { __state = 39; }
                                54 => { __state = 40; }
                                44 => { __state = 41; }
                                43 => { __state = 42; }
                                107 => { __state = 43; }
                                109 => { __state = 44; }
                                108 => { __state = 45; }
                                111 => { __state = 46; }
                                103 => { __state = 47; }
                                104 => { __state = 48; }
                                110 => { __state = 49; }
                                105 => { __state = 50; }
                                106 => { __state = 51; }
                                112 => { __state = 52; }
                                174 => { __state = 53; }
                                115 => { __state = 54; }
                                19 => { __state = 55; }
                                175 => { __state = 56; }
                                51 => { __state = 57; }
                                52 => { __state = 58; }
                                169 => { __state = 59; }
                                172 => { __state = 60; }
                                20 => { __state = 61; }
                                139 => { __state = 62; }
                                178 => { __state = 63; }
                                50 => { __state = 64; }
                                49 => { __state = 65; }
                                114 => { __state = 66; }
                                181 => { __state = 67; }
                                173 => { __state = 68; }
                                78 => { __state = 69; }
                                177 => { __state = 70; }
                                179 => { __state = 71; }
                                158 => { __state = 72; }
                                72 => { __state = 73; }
                                _ => { __state = 28; }
                            }
                        }
                    }
                    20 => {
                        sqlite3_release_temp_reg(unsafe { &mut *p_parse },
                            reg_free1);
                        __state = 541;
                    }
                    21 => {
                        p_agg_info =
                            unsafe { (*p_expr).p_agg_info } as *const AggInfo;
                        __state = 76;
                    }
                    22 => {
                        i_tab = unsafe { (*p_expr).i_table };
                        __state = 102;
                    }
                    23 => {
                        code_integer(p_parse, p_expr, 0, target);
                        __state = 153;
                    }
                    24 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73,
                                sqlite3_expr_truth_value(p_expr as *const Expr), target)
                        };
                        __state = 155;
                    }
                    25 => { { let _ = 0; }; __state = 157; }
                    26 => { { let _ = 0; }; __state = 160; }
                    27 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 77, 0, target,
                                target + unsafe { (*p_expr).y.n_reg } - 1)
                        };
                        __state = 163;
                    }
                    28 => { { let _ = 0; }; __state = 165; }
                    29 => { __state = 168; }
                    30 => { { let _ = 0; }; __state = 180; }
                    31 => { return unsafe { (*p_expr).i_table }; }
                    32 => {
                        sqlite3_expr_code(p_parse, unsafe { (*p_expr).p_left },
                            target);
                        __state = 186;
                    }
                    33 => { __state = 34; }
                    34 => {
                        op = if op == 45 { 54 } else { 53 };
                        __state = 190;
                    }
                    35 => { __state = 36; }
                    36 => { __state = 37; }
                    37 => { __state = 38; }
                    38 => { __state = 39; }
                    39 => { __state = 40; }
                    40 => {
                        p_left = unsafe { (*p_expr).p_left };
                        __state = 194;
                    }
                    41 => { __state = 42; }
                    42 => {
                        in_reg =
                            expr_code_target_and_or(p_parse, p_expr, target,
                                &mut reg_free1);
                        __state = 233;
                    }
                    43 => { __state = 44; }
                    44 => { __state = 45; }
                    45 => { __state = 46; }
                    46 => { __state = 47; }
                    47 => { __state = 48; }
                    48 => { __state = 49; }
                    49 => { __state = 50; }
                    50 => { __state = 51; }
                    51 => { __state = 52; }
                    52 => { __state = 236; }
                    53 => {
                        p_left_1 = unsafe { (*p_expr).p_left };
                        __state = 270;
                    }
                    54 => { __state = 55; }
                    55 => { { let _ = 0; }; __state = 288; }
                    56 => { __state = 297; }
                    57 => { __state = 58; }
                    58 => { __state = 307; }
                    59 => {
                        p_info = unsafe { (*p_expr).p_agg_info } as *const AggInfo;
                        __state = 322;
                    }
                    60 => { __state = 328; }
                    61 => { __state = 62; }
                    62 => { __state = 400; }
                    63 => { __state = 410; }
                    64 => {
                        dest_if_false = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 420;
                    }
                    65 => {
                        expr_code_between(p_parse, unsafe { &*p_expr }, target,
                            None, 0);
                        __state = 429;
                    }
                    66 => {
                        if !(unsafe { (*p_expr).flags } & 512 as u32 != 0 as u32) as
                                    i32 != 0 {
                            __state = 431;
                        } else { __state = 432; }
                    }
                    67 => { __state = 68; }
                    68 => {
                        p_expr = unsafe { (*p_expr).p_left };
                        __state = 439;
                    }
                    69 => { __state = 442; }
                    70 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"row value misused".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 458;
                    }
                    71 => { __state = 460; }
                    72 => { __state = 479; }
                    73 => { { let _ = 0; }; __state = 529; }
                    74 => { __state = 21; }
                    75 => { __state = 100; }
                    76 => { __state = 77; }
                    77 => { { let _ = 0; }; __state = 78; }
                    78 => { { let _ = 0; }; __state = 79; }
                    79 => {
                        if unsafe { (*p_expr).i_agg } as i32 >=
                                unsafe { (*p_agg_info).n_column } {
                            __state = 81;
                        } else { __state = 80; }
                    }
                    80 => {
                        p_col =
                            unsafe {
                                unsafe {
                                    (*p_agg_info).a_col.offset(unsafe { (*p_expr).i_agg } as
                                            isize)
                                }
                            };
                        __state = 83;
                    }
                    81 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        __state = 82;
                    }
                    82 => { __state = 20; }
                    83 => {
                        if (unsafe { (*p_agg_info).direct_mode } == 0) as i32 != 0 {
                            __state = 85;
                        } else { __state = 86; }
                    }
                    84 => { __state = 75; }
                    85 => {
                        return unsafe { (*p_agg_info).i_first_reg } +
                                unsafe { (*p_expr).i_agg } as i32;
                    }
                    86 => {
                        if unsafe { (*p_agg_info).use_sorting_idx } != 0 {
                            __state = 87;
                        } else { __state = 88; }
                    }
                    87 => {
                        p_tab = unsafe { (*p_col).p_tab } as *const Table;
                        __state = 89;
                    }
                    88 => {
                        if unsafe { (*p_expr).y.p_tab } == core::ptr::null_mut() {
                            __state = 98;
                        } else { __state = 84; }
                    }
                    89 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96,
                                unsafe { (*p_agg_info).sorting_idx_p_tab },
                                unsafe { (*p_col).i_sorter_column }, target)
                        };
                        __state = 90;
                    }
                    90 => {
                        if p_tab == core::ptr::null_mut() {
                            __state = 92;
                        } else { __state = 93; }
                    }
                    91 => { return target; }
                    92 => { __state = 91; }
                    93 => {
                        if unsafe { (*p_col).i_column } < 0 {
                            __state = 94;
                        } else { __state = 95; }
                    }
                    94 => { __state = 91; }
                    95 => { __state = 96; }
                    96 => {
                        if unsafe {
                                        (*unsafe {
                                                    (*p_tab).a_col.offset(unsafe { (*p_col).i_column } as isize)
                                                }).affinity
                                    } as i32 == 69 {
                            __state = 97;
                        } else { __state = 91; }
                    }
                    97 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 89, target) };
                        __state = 91;
                    }
                    98 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, unsafe { (*p_expr).i_table },
                                unsafe { (*p_expr).i_column } as i32, target)
                        };
                        __state = 99;
                    }
                    99 => { return target; }
                    100 => { __state = 22; }
                    101 => { __state = 23; }
                    102 => { __state = 103; }
                    103 => {
                        if unsafe { (*p_expr).flags } & 32 as u32 != 0 as u32 {
                            __state = 105;
                        } else { __state = 104; }
                    }
                    104 => {
                        if i_tab < 0 { __state = 117; } else { __state = 118; }
                    }
                    105 => { __state = 106; }
                    106 => {
                        i_reg =
                            sqlite3_expr_code_target(p_parse,
                                unsafe { (*p_expr).p_left }, target);
                        __state = 107;
                    }
                    107 => { { let _ = 0; }; __state = 108; }
                    108 => { { let _ = 0; }; __state = 109; }
                    109 => {
                        aff =
                            sqlite3_table_column_affinity(unsafe {
                                        &*unsafe { (*p_expr).y.p_tab }
                                    }, unsafe { (*p_expr).i_column } as i32) as i32;
                        __state = 110;
                    }
                    110 => {
                        if aff > 65 { __state = 112; } else { __state = 111; }
                    }
                    111 => { return i_reg; }
                    112 => { __state = 113; }
                    113 => { { let _ = 0; }; __state = 114; }
                    114 => { { let _ = 0; }; __state = 115; }
                    115 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 98, i_reg, 1, 0,
                                &z_aff_1[((aff - 'B' as i32) * 2) as usize], -1)
                        };
                        __state = 111;
                    }
                    116 => { { let _ = 0; }; __state = 149; }
                    117 => {
                        if unsafe { (*p_parse).i_self_tab } < 0 {
                            __state = 119;
                        } else { __state = 120; }
                    }
                    118 => {
                        if !(unsafe { (*p_parse).p_idx_part_expr }).is_null() &&
                                0 !=
                                    {
                                        r1 =
                                            expr_partidx_expr_lookup(p_parse, unsafe { &*p_expr },
                                                target);
                                        r1
                                    } {
                            __state = 148;
                        } else { __state = 116; }
                    }
                    119 => { __state = 121; }
                    120 => {
                        i_tab = unsafe { (*p_parse).i_self_tab } - 1;
                        __state = 116;
                    }
                    121 => { __state = 122; }
                    122 => { __state = 123; }
                    123 => {
                        i_col = unsafe { (*p_expr).i_column } as i32;
                        __state = 124;
                    }
                    124 => { { let _ = 0; }; __state = 125; }
                    125 => {
                        p_tab_1 = unsafe { (*p_expr).y.p_tab };
                        __state = 126;
                    }
                    126 => { { let _ = 0; }; __state = 127; }
                    127 => { { let _ = 0; }; __state = 128; }
                    128 => { { let _ = 0; }; __state = 129; }
                    129 => {
                        if i_col < 0 { __state = 131; } else { __state = 130; }
                    }
                    130 => {
                        p_col_1 =
                            unsafe {
                                unsafe { (*p_tab_1).a_col.offset(i_col as isize) }
                            };
                        __state = 132;
                    }
                    131 => { return -1 - unsafe { (*p_parse).i_self_tab }; }
                    132 => { __state = 133; }
                    133 => {
                        i_src =
                            unsafe {
                                        sqlite3_table_column_to_storage(p_tab_1, i_col as i16)
                                    } as i32 - unsafe { (*p_parse).i_self_tab };
                        __state = 134;
                    }
                    134 => {
                        if unsafe { (*p_col_1).col_flags } as i32 & 96 != 0 {
                            __state = 135;
                        } else { __state = 136; }
                    }
                    135 => {
                        if unsafe { (*p_col_1).col_flags } as i32 & 256 != 0 {
                            __state = 138;
                        } else { __state = 137; }
                    }
                    136 => {
                        if unsafe { (*p_col_1).affinity } as i32 == 69 {
                            __state = 144;
                        } else { __state = 145; }
                    }
                    137 => {
                        unsafe { (*p_col_1).col_flags |= 256 as u16 };
                        __state = 140;
                    }
                    138 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"generated column loop on \"%s\"".as_ptr() as *mut i8 as
                                    *const i8, unsafe { (*p_col_1).z_cn_name })
                        };
                        __state = 139;
                    }
                    139 => { return 0; }
                    140 => {
                        if unsafe { (*p_col_1).col_flags } as i32 & 128 != 0 {
                            __state = 142;
                        } else { __state = 141; }
                    }
                    141 => {
                        unsafe { (*p_col_1).col_flags &= !(256 | 128) as u16 };
                        __state = 143;
                    }
                    142 => {
                        sqlite3_expr_code_generated_column(p_parse, p_tab_1,
                            p_col_1, i_src);
                        __state = 141;
                    }
                    143 => { return i_src; }
                    144 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 83, i_src, target) };
                        __state = 146;
                    }
                    145 => { return i_src; }
                    146 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 89, target) };
                        __state = 147;
                    }
                    147 => { return target; }
                    148 => { return r1; }
                    149 => { { let _ = 0; }; __state = 150; }
                    150 => {
                        i_reg =
                            sqlite3_expr_code_get_column(unsafe { &*p_parse },
                                unsafe { (*p_expr).y.p_tab },
                                unsafe { (*p_expr).i_column } as i32, i_tab, target,
                                unsafe { (*p_expr).op2 });
                        __state = 151;
                    }
                    151 => { return i_reg; }
                    152 => { __state = 24; }
                    153 => { return target; }
                    154 => { __state = 25; }
                    155 => { return target; }
                    156 => { __state = 26; }
                    157 => {
                        code_real(v, unsafe { (*p_expr).u.z_token } as *const i8, 0,
                            target);
                        __state = 158;
                    }
                    158 => { return target; }
                    159 => { __state = 27; }
                    160 => {
                        unsafe {
                            sqlite3_vdbe_load_string(v, target,
                                unsafe { (*p_expr).u.z_token } as *const i8)
                        };
                        __state = 161;
                    }
                    161 => { return target; }
                    162 => { __state = 28; }
                    163 => { return target; }
                    164 => { __state = 29; }
                    165 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        __state = 166;
                    }
                    166 => { return target; }
                    167 => { __state = 30; }
                    168 => { __state = 169; }
                    169 => { __state = 170; }
                    170 => { { let _ = 0; }; __state = 171; }
                    171 => { { let _ = 0; }; __state = 172; }
                    172 => { { let _ = 0; }; __state = 173; }
                    173 => {
                        z =
                            unsafe { unsafe { (*p_expr).u.z_token.offset(2 as isize) } }
                                as *const i8;
                        __state = 174;
                    }
                    174 => {
                        n = unsafe { sqlite3_strlen30(z) } - 1;
                        __state = 175;
                    }
                    175 => { { let _ = 0; }; __state = 176; }
                    176 => {
                        z_blob =
                            unsafe {
                                    sqlite3_hex_to_blob(unsafe { sqlite3_vdbe_db(v) }, z, n)
                                } as *mut i8;
                        __state = 177;
                    }
                    177 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 79, n / 2, target, 0,
                                z_blob as *const i8, -7)
                        };
                        __state = 178;
                    }
                    178 => { return target; }
                    179 => { __state = 31; }
                    180 => { { let _ = 0; }; __state = 181; }
                    181 => { { let _ = 0; }; __state = 182; }
                    182 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 80,
                                unsafe { (*p_expr).i_column } as i32, target)
                        };
                        __state = 183;
                    }
                    183 => { return target; }
                    184 => { __state = 32; }
                    185 => { __state = 33; }
                    186 => { { let _ = 0; }; __state = 187; }
                    187 => { { let _ = 0; }; __state = 188; }
                    188 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 90, target,
                                unsafe {
                                        sqlite3_affinity_type(unsafe { (*p_expr).u.z_token } as
                                                *const i8, core::ptr::null_mut())
                                    } as i32)
                        };
                        __state = 189;
                    }
                    189 => { return in_reg; }
                    190 => { p5 = 128; __state = 191; }
                    191 => { __state = 192; }
                    192 => { __state = 35; }
                    193 => { __state = 231; }
                    194 => { addr_is_null = 0; __state = 195; }
                    195 => {
                        if sqlite3_expr_is_vector(p_left as *const Expr) != 0 {
                            __state = 197;
                        } else { __state = 198; }
                    }
                    196 => { __state = 20; }
                    197 => {
                        code_vector_compare(p_parse, unsafe { &*p_expr }, target,
                            op as u8, p5 as u8);
                        __state = 196;
                    }
                    198 => {
                        if unsafe { (*p_expr).flags } & 4194304 as u32 != 0 as u32
                                && p5 != 128 {
                            __state = 200;
                        } else { __state = 201; }
                    }
                    199 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, in_reg) };
                        __state = 203;
                    }
                    200 => {
                        addr_is_null =
                            expr_compute_operands(p_parse, p_expr, &mut r1, &mut r2,
                                &mut reg_free1, &mut reg_free2);
                        __state = 199;
                    }
                    201 => {
                        r1 =
                            sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                                &mut reg_free1);
                        __state = 202;
                    }
                    202 => {
                        r2 =
                            sqlite3_expr_code_temp(p_parse,
                                unsafe { (*p_expr).p_right }, &mut reg_free2);
                        __state = 199;
                    }
                    203 => {
                        code_compare(p_parse, p_left as *const Expr,
                            unsafe { (*p_expr).p_right } as *const Expr, op, r1, r2,
                            unsafe { sqlite3_vdbe_current_addr(v) } + 2, p5,
                            (unsafe { (*p_expr).flags } & 1024 as u32 != 0 as u32) as
                                i32);
                        __state = 204;
                    }
                    204 => { { let _ = 0; }; __state = 205; }
                    205 => { __state = 206; }
                    206 => { __state = 207; }
                    207 => { { let _ = 0; }; __state = 208; }
                    208 => { __state = 209; }
                    209 => { __state = 210; }
                    210 => { { let _ = 0; }; __state = 211; }
                    211 => { __state = 212; }
                    212 => { __state = 213; }
                    213 => { { let _ = 0; }; __state = 214; }
                    214 => { __state = 215; }
                    215 => { __state = 216; }
                    216 => { { let _ = 0; }; __state = 217; }
                    217 => { __state = 218; }
                    218 => { __state = 219; }
                    219 => { { let _ = 0; }; __state = 220; }
                    220 => { __state = 221; }
                    221 => { __state = 222; }
                    222 => {
                        if p5 == 128 { __state = 224; } else { __state = 225; }
                    }
                    223 => { __state = 230; }
                    224 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, in_reg) };
                        __state = 223;
                    }
                    225 => {
                        unsafe { sqlite3_vdbe_add_op3(v, 94, r1, in_reg, r2) };
                        __state = 226;
                    }
                    226 => {
                        if addr_is_null != 0 {
                            __state = 227;
                        } else { __state = 223; }
                    }
                    227 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 9, 0,
                                unsafe { sqlite3_vdbe_current_addr(v) } + 2)
                        };
                        __state = 228;
                    }
                    228 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_is_null) };
                        __state = 229;
                    }
                    229 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, in_reg) };
                        __state = 223;
                    }
                    230 => { __state = 196; }
                    231 => { __state = 41; }
                    232 => { __state = 234; }
                    233 => { __state = 20; }
                    234 => { __state = 43; }
                    235 => { __state = 268; }
                    236 => { { let _ = 0; }; __state = 237; }
                    237 => { __state = 238; }
                    238 => { { let _ = 0; }; __state = 239; }
                    239 => { __state = 240; }
                    240 => { { let _ = 0; }; __state = 241; }
                    241 => { __state = 242; }
                    242 => { { let _ = 0; }; __state = 243; }
                    243 => { __state = 244; }
                    244 => { { let _ = 0; }; __state = 245; }
                    245 => { __state = 246; }
                    246 => { { let _ = 0; }; __state = 247; }
                    247 => { __state = 248; }
                    248 => { { let _ = 0; }; __state = 249; }
                    249 => { __state = 250; }
                    250 => { { let _ = 0; }; __state = 251; }
                    251 => { __state = 252; }
                    252 => { { let _ = 0; }; __state = 253; }
                    253 => { __state = 254; }
                    254 => {
                        if unsafe { (*p_expr).flags } & 4194304 as u32 != 0 as u32 {
                            __state = 256;
                        } else { __state = 257; }
                    }
                    255 => {
                        unsafe { sqlite3_vdbe_add_op3(v, op, r2, r1, target) };
                        __state = 260;
                    }
                    256 => {
                        addr_is_null_1 =
                            expr_compute_operands(p_parse, p_expr, &mut r1, &mut r2,
                                &mut reg_free1, &mut reg_free2);
                        __state = 255;
                    }
                    257 => {
                        r1 =
                            sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                                &mut reg_free1);
                        __state = 258;
                    }
                    258 => {
                        r2 =
                            sqlite3_expr_code_temp(p_parse,
                                unsafe { (*p_expr).p_right }, &mut reg_free2);
                        __state = 259;
                    }
                    259 => { addr_is_null_1 = 0; __state = 255; }
                    260 => { __state = 261; }
                    261 => { __state = 262; }
                    262 => {
                        if addr_is_null_1 != 0 {
                            __state = 264;
                        } else { __state = 263; }
                    }
                    263 => { __state = 20; }
                    264 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 9, 0,
                                unsafe { sqlite3_vdbe_current_addr(v) } + 2)
                        };
                        __state = 265;
                    }
                    265 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_is_null_1) };
                        __state = 266;
                    }
                    266 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        __state = 267;
                    }
                    267 => { __state = 263; }
                    268 => { __state = 53; }
                    269 => { __state = 54; }
                    270 => { { let _ = 0; }; __state = 271; }
                    271 => {
                        if unsafe { (*p_left_1).op } as i32 == 156 {
                            __state = 273;
                        } else { __state = 274; }
                    }
                    272 => { __state = 20; }
                    273 => {
                        code_integer(p_parse, p_left_1, 1, target);
                        __state = 275;
                    }
                    274 => {
                        if unsafe { (*p_left_1).op } as i32 == 154 {
                            __state = 276;
                        } else { __state = 277; }
                    }
                    275 => { return target; }
                    276 => { { let _ = 0; }; __state = 278; }
                    277 => { temp_x.op = 156 as u8; __state = 280; }
                    278 => {
                        code_real(v, unsafe { (*p_left_1).u.z_token } as *const i8,
                            1, target);
                        __state = 279;
                    }
                    279 => { return target; }
                    280 => {
                        temp_x.flags = (2048 | 65536) as u32;
                        __state = 281;
                    }
                    281 => { temp_x.u.i_value = 0; __state = 282; }
                    282 => { __state = 283; }
                    283 => {
                        r1 =
                            sqlite3_expr_code_temp(p_parse, &mut temp_x,
                                &mut reg_free1);
                        __state = 284;
                    }
                    284 => {
                        r2 =
                            sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                                &mut reg_free2);
                        __state = 285;
                    }
                    285 => {
                        unsafe { sqlite3_vdbe_add_op3(v, 108, r2, r1, target) };
                        __state = 286;
                    }
                    286 => { __state = 272; }
                    287 => { __state = 295; }
                    288 => { __state = 289; }
                    289 => { { let _ = 0; }; __state = 290; }
                    290 => { __state = 291; }
                    291 => {
                        r1 =
                            sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                                &mut reg_free1);
                        __state = 292;
                    }
                    292 => { __state = 293; }
                    293 => {
                        unsafe { sqlite3_vdbe_add_op2(v, op, r1, in_reg) };
                        __state = 294;
                    }
                    294 => { __state = 20; }
                    295 => { __state = 56; }
                    296 => { __state = 57; }
                    297 => { __state = 298; }
                    298 => {
                        r1 =
                            sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                                &mut reg_free1);
                        __state = 299;
                    }
                    299 => { __state = 300; }
                    300 => {
                        is_true =
                            sqlite3_expr_truth_value(unsafe { (*p_expr).p_right } as
                                    *const Expr);
                        __state = 301;
                    }
                    301 => {
                        b_normal = (unsafe { (*p_expr).op2 } as i32 == 45) as i32;
                        __state = 302;
                    }
                    302 => { __state = 303; }
                    303 => { __state = 304; }
                    304 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 93, r1, in_reg,
                                (is_true == 0) as i32 as i32, is_true ^ b_normal)
                        };
                        __state = 305;
                    }
                    305 => { __state = 20; }
                    306 => { __state = 320; }
                    307 => { { let _ = 0; }; __state = 308; }
                    308 => { __state = 309; }
                    309 => { { let _ = 0; }; __state = 310; }
                    310 => { __state = 311; }
                    311 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, target) };
                        __state = 312;
                    }
                    312 => {
                        r1 =
                            sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                                &mut reg_free1);
                        __state = 313;
                    }
                    313 => { __state = 314; }
                    314 => {
                        addr = unsafe { sqlite3_vdbe_add_op1(v, op, r1) };
                        __state = 315;
                    }
                    315 => { __state = 316; }
                    316 => { __state = 317; }
                    317 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, target) };
                        __state = 318;
                    }
                    318 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr) };
                        __state = 319;
                    }
                    319 => { __state = 20; }
                    320 => { __state = 59; }
                    321 => { __state = 60; }
                    322 => {
                        if p_info == core::ptr::null_mut() ||
                                    (unsafe { (*p_expr).i_agg } as i32) < 0 ||
                                unsafe { (*p_expr).i_agg } as i32 >=
                                    unsafe { (*p_info).n_func } {
                            __state = 324;
                        } else { __state = 325; }
                    }
                    323 => { __state = 20; }
                    324 => { { let _ = 0; }; __state = 326; }
                    325 => {
                        return unsafe { (*p_info).i_first_reg } +
                                    unsafe { (*p_info).n_column } +
                                unsafe { (*p_expr).i_agg } as i32;
                    }
                    326 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"misuse of aggregate: %#T()".as_ptr() as *mut i8 as
                                    *const i8, p_expr)
                        };
                        __state = 323;
                    }
                    327 => { __state = 61; }
                    328 => { __state = 329; }
                    329 => { __state = 330; }
                    330 => { __state = 331; }
                    331 => { const_mask = 0 as u32; __state = 332; }
                    332 => { __state = 333; }
                    333 => { db = unsafe { (*p_parse).db }; __state = 334; }
                    334 => { enc = unsafe { (*db).enc }; __state = 335; }
                    335 => { p_coll = core::ptr::null_mut(); __state = 336; }
                    336 => {
                        if unsafe { (*p_expr).flags } & 16777216 as u32 != 0 as u32
                            {
                            __state = 338;
                        } else { __state = 337; }
                    }
                    337 => {
                        if unsafe { (*p_parse).ok_const_factor() } != 0 &&
                                sqlite3_expr_is_constant_not_join(p_parse, p_expr) != 0 {
                            __state = 340;
                        } else { __state = 339; }
                    }
                    338 => {
                        return unsafe {
                                (*unsafe { (*p_expr).y.p_win }).reg_result
                            };
                    }
                    339 => { { let _ = 0; }; __state = 341; }
                    340 => {
                        return sqlite3_expr_code_run_just_once(p_parse, p_expr, -1);
                    }
                    341 => { { let _ = 0; }; __state = 342; }
                    342 => {
                        p_farg = unsafe { (*p_expr).x.p_list };
                        __state = 343;
                    }
                    343 => {
                        n_farg =
                            if !(p_farg).is_null() {
                                unsafe { (*p_farg).n_expr }
                            } else { 0 };
                        __state = 344;
                    }
                    344 => { { let _ = 0; }; __state = 345; }
                    345 => {
                        z_id = unsafe { (*p_expr).u.z_token } as *const i8;
                        __state = 346;
                    }
                    346 => {
                        p_def =
                            unsafe {
                                sqlite3_find_function(db, z_id, n_farg, enc, 0 as u8)
                            };
                        __state = 347;
                    }
                    347 => {
                        if p_def == core::ptr::null_mut() ||
                                    unsafe { (*p_def).x_finalize.is_some() } ||
                                unsafe { (*p_def).func_flags } & 262144 as u32 != 0 as u32
                                        && (unsafe { (*p_parse).nested } == 0) as i32 != 0 &&
                                    unsafe { (*db).m_db_flags } & 32 as u32 == 0 as u32 {
                            __state = 349;
                        } else { __state = 348; }
                    }
                    348 => {
                        if unsafe { (*p_def).func_flags } & 4194304 as u32 !=
                                    0 as u32 && p_farg != core::ptr::null_mut() {
                            __state = 352;
                        } else { __state = 353; }
                    }
                    349 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"unknown function: %#T()".as_ptr() as *mut i8 as *const i8,
                                p_expr)
                        };
                        __state = 350;
                    }
                    350 => { __state = 20; }
                    351 => { i = 0; __state = 358; }
                    352 => { { let _ = 0; }; __state = 354; }
                    353 => {
                        if unsafe { (*p_def).func_flags } &
                                    (524288 | 2097152) as u32 != 0 {
                            __state = 356;
                        } else { __state = 351; }
                    }
                    354 => { { let _ = 0; }; __state = 355; }
                    355 => {
                        return expr_code_inline_function(p_parse, p_farg,
                                unsafe { (*p_def).p_user_data } as i64 as i32, target);
                    }
                    356 => {
                        sqlite3_expr_function_usable(p_parse, p_expr as *const Expr,
                            unsafe { &*p_def });
                        __state = 351;
                    }
                    357 => {
                        if !(p_farg).is_null() {
                            __state = 366;
                        } else { __state = 367; }
                    }
                    358 => {
                        if i < n_farg { __state = 359; } else { __state = 357; }
                    }
                    359 => {
                        if i < 32 &&
                                sqlite3_expr_is_constant(p_parse,
                                        unsafe {
                                            (*(unsafe { (*p_farg).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        }) != 0 {
                            __state = 362;
                        } else { __state = 361; }
                    }
                    360 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 358;
                    }
                    361 => {
                        if unsafe { (*p_def).func_flags } & 32 as u32 != 0 as u32 &&
                                (p_coll).is_null() as i32 != 0 {
                            __state = 364;
                        } else { __state = 360; }
                    }
                    362 => { __state = 363; }
                    363 => { const_mask |= (1 as u32) << i; __state = 361; }
                    364 => {
                        p_coll =
                            sqlite3_expr_coll_seq(p_parse,
                                unsafe {
                                        (*(unsafe { (*p_farg).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                    } as *const Expr);
                        __state = 360;
                    }
                    365 => {
                        if n_farg >= 2 &&
                                unsafe { (*p_expr).flags } & 256 as u32 != 0 as u32 {
                            __state = 387;
                        } else { __state = 388; }
                    }
                    366 => {
                        if const_mask != 0 {
                            __state = 369;
                        } else { __state = 370; }
                    }
                    367 => { r1 = 0; __state = 365; }
                    368 => {
                        if unsafe { (*p_def).func_flags } & (64 | 128) as u32 !=
                                0 as u32 {
                            __state = 373;
                        } else { __state = 372; }
                    }
                    369 => {
                        r1 = unsafe { (*p_parse).n_mem } + 1;
                        __state = 371;
                    }
                    370 => {
                        r1 = sqlite3_get_temp_range(p_parse, n_farg);
                        __state = 368;
                    }
                    371 => {
                        unsafe { (*p_parse).n_mem += n_farg };
                        __state = 368;
                    }
                    372 => {
                        sqlite3_expr_code_expr_list(p_parse,
                            unsafe { &mut *p_farg }, r1, 0, 2 as u8);
                        __state = 365;
                    }
                    373 => { __state = 374; }
                    374 => { { let _ = 0; }; __state = 375; }
                    375 => { { let _ = 0; }; __state = 376; }
                    376 => {
                        expr_op =
                            unsafe {
                                (*unsafe {
                                                (*(unsafe { (*p_farg).a.as_ptr() } as
                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                            }).op
                            };
                        __state = 377;
                    }
                    377 => {
                        if expr_op as i32 == 168 || expr_op as i32 == 170 {
                            __state = 378;
                        } else { __state = 372; }
                    }
                    378 => { { let _ = 0; }; __state = 379; }
                    379 => { { let _ = 0; }; __state = 380; }
                    380 => { { let _ = 0; }; __state = 381; }
                    381 => { { let _ = 0; }; __state = 382; }
                    382 => { __state = 383; }
                    383 => { __state = 384; }
                    384 => { __state = 385; }
                    385 => {
                        unsafe {
                            (*unsafe {
                                                (*(unsafe { (*p_farg).a.as_ptr() } as
                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                            }).op2 = (unsafe { (*p_def).func_flags } & 192 as u32) as u8
                        };
                        __state = 372;
                    }
                    386 => {
                        if unsafe { (*p_def).func_flags } & 32 as u32 != 0 {
                            __state = 391;
                        } else { __state = 390; }
                    }
                    387 => {
                        p_def =
                            unsafe {
                                sqlite3_vtab_overload_function(db, p_def, n_farg,
                                    unsafe {
                                        (*(unsafe { (*p_farg).a.as_ptr() } as
                                                        *mut ExprListItem).offset(1 as isize)).p_expr
                                    })
                            };
                        __state = 386;
                    }
                    388 => {
                        if n_farg > 0 { __state = 389; } else { __state = 386; }
                    }
                    389 => {
                        p_def =
                            unsafe {
                                sqlite3_vtab_overload_function(db, p_def, n_farg,
                                    unsafe {
                                        (*(unsafe { (*p_farg).a.as_ptr() } as
                                                        *mut ExprListItem).offset(0 as isize)).p_expr
                                    })
                            };
                        __state = 386;
                    }
                    390 => {
                        unsafe {
                            sqlite3_vdbe_add_function_call(p_parse, const_mask as i32,
                                r1, target, n_farg, p_def as *const FuncDef,
                                unsafe { (*p_expr).op2 } as i32)
                        };
                        __state = 394;
                    }
                    391 => {
                        if (p_coll).is_null() as i32 != 0 {
                            __state = 393;
                        } else { __state = 392; }
                    }
                    392 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 87, 0, 0, 0,
                                p_coll as *mut i8 as *const i8, -2)
                        };
                        __state = 390;
                    }
                    393 => {
                        p_coll = unsafe { (*db).p_dflt_coll };
                        __state = 392;
                    }
                    394 => {
                        if n_farg != 0 { __state = 396; } else { __state = 395; }
                    }
                    395 => { return target; }
                    396 => {
                        if const_mask == 0 as u32 {
                            __state = 397;
                        } else { __state = 398; }
                    }
                    397 => {
                        sqlite3_release_temp_range(p_parse, r1, n_farg);
                        __state = 395;
                    }
                    398 => { __state = 395; }
                    399 => { __state = 408; }
                    400 => { __state = 401; }
                    401 => { __state = 402; }
                    402 => {
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            __state = 404;
                        } else { __state = 405; }
                    }
                    403 => { __state = 20; }
                    404 => { return 0; }
                    405 => {
                        if op == 139 &&
                                    unsafe { (*p_expr).flags } & 4096 as u32 != 0 as u32 &&
                                {
                                        n_col =
                                            unsafe {
                                                (*unsafe {
                                                                (*unsafe { (*p_expr).x.p_select }).p_e_list
                                                            }).n_expr
                                            };
                                        n_col
                                    } != 1 {
                            __state = 406;
                        } else { __state = 407; }
                    }
                    406 => {
                        sqlite3_subselect_error(p_parse, n_col, 1);
                        __state = 403;
                    }
                    407 => {
                        return sqlite3_code_subselect(p_parse,
                                unsafe { &mut *p_expr });
                    }
                    408 => { __state = 63; }
                    409 => { __state = 64; }
                    410 => {
                        p_left_2 = unsafe { (*p_expr).p_left };
                        __state = 411;
                    }
                    411 => {
                        if unsafe { (*p_left_2).i_table } == 0 ||
                                unsafe { (*p_parse).within_rj_subrtn } as i32 >
                                    unsafe { (*p_left_2).op2 } as i32 {
                            __state = 413;
                        } else { __state = 412; }
                    }
                    412 => { { let _ = 0; }; __state = 415; }
                    413 => {
                        unsafe {
                            (*p_left_2).i_table =
                                sqlite3_code_subselect(p_parse, unsafe { &mut *p_left_2 })
                        };
                        __state = 414;
                    }
                    414 => {
                        unsafe {
                            (*p_left_2).op2 = unsafe { (*p_parse).within_rj_subrtn }
                        };
                        __state = 412;
                    }
                    415 => {
                        n__1 = sqlite3_expr_vector_size(unsafe { &*p_left_2 });
                        __state = 416;
                    }
                    416 => {
                        if unsafe { (*p_expr).i_table } != n__1 {
                            __state = 418;
                        } else { __state = 417; }
                    }
                    417 => {
                        return unsafe { (*p_left_2).i_table } +
                                unsafe { (*p_expr).i_column } as i32;
                    }
                    418 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"%d columns assigned %d values".as_ptr() as *mut i8 as
                                    *const i8, unsafe { (*p_expr).i_table }, n__1)
                        };
                        __state = 417;
                    }
                    419 => { __state = 65; }
                    420 => {
                        dest_if_null = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 421;
                    }
                    421 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        __state = 422;
                    }
                    422 => {
                        sqlite3_expr_code_in(p_parse, p_expr, dest_if_false,
                            dest_if_null);
                        __state = 423;
                    }
                    423 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, target) };
                        __state = 424;
                    }
                    424 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, dest_if_false) };
                        __state = 425;
                    }
                    425 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 88, target, 0) };
                        __state = 426;
                    }
                    426 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, dest_if_null) };
                        __state = 427;
                    }
                    427 => { return target; }
                    428 => { __state = 66; }
                    429 => { return target; }
                    430 => { __state = 437; }
                    431 => { { let _ = 0; }; __state = 433; }
                    432 => {
                        p_expr = unsafe { (*p_expr).p_left };
                        __state = 436;
                    }
                    433 => {
                        sqlite3_expr_code(p_parse, unsafe { (*p_expr).p_left },
                            target);
                        __state = 434;
                    }
                    434 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 182, target) };
                        __state = 435;
                    }
                    435 => { return target; }
                    436 => { __state = 2; }
                    437 => { __state = 67; }
                    438 => { __state = 440; }
                    439 => { __state = 2; }
                    440 => { __state = 69; }
                    441 => { __state = 70; }
                    442 => { __state = 443; }
                    443 => { __state = 444; }
                    444 => { { let _ = 0; }; __state = 445; }
                    445 => {
                        p_tab_2 = unsafe { (*p_expr).y.p_tab };
                        __state = 446;
                    }
                    446 => {
                        i_col_1 = unsafe { (*p_expr).i_column } as i32;
                        __state = 447;
                    }
                    447 => {
                        p1 =
                            unsafe { (*p_expr).i_table } *
                                        (unsafe { (*p_tab_2).n_col } as i32 + 1) + 1 +
                                unsafe {
                                        sqlite3_table_column_to_storage(p_tab_2, i_col_1 as i16)
                                    } as i32;
                        __state = 448;
                    }
                    448 => { { let _ = 0; }; __state = 449; }
                    449 => { { let _ = 0; }; __state = 450; }
                    450 => { { let _ = 0; }; __state = 451; }
                    451 => { { let _ = 0; }; __state = 452; }
                    452 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 159, p1, target) };
                        __state = 453;
                    }
                    453 => { __state = 454; }
                    454 => {
                        if i_col_1 >= 0 &&
                                unsafe {
                                            (*unsafe {
                                                        (*p_tab_2).a_col.offset(i_col_1 as isize)
                                                    }).affinity
                                        } as i32 == 69 {
                            __state = 456;
                        } else { __state = 455; }
                    }
                    455 => { __state = 20; }
                    456 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 89, target) };
                        __state = 455;
                    }
                    457 => { __state = 71; }
                    458 => { __state = 20; }
                    459 => { __state = 72; }
                    460 => {
                        ok_const_factor =
                            unsafe { (*p_parse).ok_const_factor() } as u8;
                        __state = 461;
                    }
                    461 => {
                        p_agg_info_1 =
                            unsafe { (*p_expr).p_agg_info } as *const AggInfo;
                        __state = 462;
                    }
                    462 => {
                        if !(p_agg_info_1).is_null() {
                            __state = 464;
                        } else { __state = 463; }
                    }
                    463 => {
                        addr_inr =
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 20, unsafe { (*p_expr).i_table }, 0,
                                    target)
                            };
                        __state = 472;
                    }
                    464 => { { let _ = 0; }; __state = 465; }
                    465 => {
                        if (unsafe { (*p_agg_info_1).direct_mode } == 0) as i32 != 0
                            {
                            __state = 467;
                        } else { __state = 466; }
                    }
                    466 => {
                        if unsafe {
                                    (*unsafe { (*p_expr).p_agg_info }).use_sorting_idx
                                } != 0 {
                            __state = 469;
                        } else { __state = 463; }
                    }
                    467 => {
                        in_reg =
                            unsafe { (*p_agg_info_1).i_first_reg } +
                                unsafe { (*p_expr).i_agg } as i32;
                        __state = 468;
                    }
                    468 => { __state = 20; }
                    469 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96,
                                unsafe { (*p_agg_info_1).sorting_idx_p_tab },
                                unsafe {
                                    (*unsafe {
                                                (*p_agg_info_1).a_col.offset(unsafe { (*p_expr).i_agg } as
                                                        isize)
                                            }).i_sorter_column
                                }, target)
                        };
                        __state = 470;
                    }
                    470 => { in_reg = target; __state = 471; }
                    471 => { __state = 20; }
                    472 => {
                        unsafe { (*p_parse).set_ok_const_factor(0 as Bft as u32) };
                        __state = 473;
                    }
                    473 => {
                        sqlite3_expr_code(p_parse, unsafe { (*p_expr).p_left },
                            target);
                        __state = 474;
                    }
                    474 => { { let _ = 0; }; __state = 475; }
                    475 => {
                        unsafe {
                            (*p_parse).set_ok_const_factor(ok_const_factor as Bft as
                                    u32)
                        };
                        __state = 476;
                    }
                    476 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_inr) };
                        __state = 477;
                    }
                    477 => { __state = 20; }
                    478 => { __state = 73; }
                    479 => { __state = 480; }
                    480 => { __state = 481; }
                    481 => { __state = 482; }
                    482 => { __state = 483; }
                    483 => { __state = 484; }
                    484 => { __state = 485; }
                    485 => { __state = 486; }
                    486 => { p_test = core::ptr::null_mut(); __state = 487; }
                    487 => { p_del = core::ptr::null_mut(); __state = 488; }
                    488 => { db__1 = unsafe { (*p_parse).db }; __state = 489; }
                    489 => { { let _ = 0; }; __state = 490; }
                    490 => { { let _ = 0; }; __state = 491; }
                    491 => {
                        p_e_list = unsafe { (*p_expr).x.p_list };
                        __state = 492;
                    }
                    492 => {
                        a_listelem =
                            unsafe { (*p_e_list).a.as_ptr() } as *mut ExprListItem;
                        __state = 493;
                    }
                    493 => {
                        n_expr = unsafe { (*p_e_list).n_expr };
                        __state = 494;
                    }
                    494 => {
                        end_label = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 495;
                    }
                    495 => {
                        if { p_x = unsafe { (*p_expr).p_left }; p_x } !=
                                core::ptr::null_mut() {
                            __state = 497;
                        } else { __state = 496; }
                    }
                    496 => { i__1 = 0; __state = 510; }
                    497 => {
                        p_del = sqlite3_expr_dup(db__1, p_x as *const Expr, 0);
                        __state = 498;
                    }
                    498 => {
                        if unsafe { (*db__1).malloc_failed } != 0 {
                            __state = 500;
                        } else { __state = 499; }
                    }
                    499 => { __state = 502; }
                    500 => { sqlite3_expr_delete(db__1, p_del); __state = 501; }
                    501 => { __state = 20; }
                    502 => {
                        sqlite3_expr_to_register(p_del,
                            expr_code_vector(p_parse, p_del, &mut reg_free1));
                        __state = 503;
                    }
                    503 => { __state = 504; }
                    504 => {
                        unsafe {
                            memset(&raw mut op_compare as *mut (), 0,
                                core::mem::size_of::<Expr>() as u64)
                        };
                        __state = 505;
                    }
                    505 => { op_compare.op = 54 as u8; __state = 506; }
                    506 => { op_compare.p_left = p_del; __state = 507; }
                    507 => { p_test = &mut op_compare; __state = 508; }
                    508 => { reg_free1 = 0; __state = 496; }
                    509 => {
                        if n_expr & 1 != 0 {
                            __state = 524;
                        } else { __state = 525; }
                    }
                    510 => {
                        if i__1 < n_expr - 1 {
                            __state = 511;
                        } else { __state = 509; }
                    }
                    511 => {
                        if !(p_x).is_null() {
                            __state = 514;
                        } else { __state = 515; }
                    }
                    512 => { i__1 = i__1 + 2; __state = 510; }
                    513 => {
                        next_case = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 517;
                    }
                    514 => { { let _ = 0; }; __state = 516; }
                    515 => {
                        p_test =
                            unsafe { (*a_listelem.offset(i__1 as isize)).p_expr };
                        __state = 513;
                    }
                    516 => {
                        op_compare.p_right =
                            unsafe { (*a_listelem.offset(i__1 as isize)).p_expr };
                        __state = 513;
                    }
                    517 => { __state = 518; }
                    518 => {
                        sqlite3_expr_if_false(p_parse, p_test, next_case, 16);
                        __state = 519;
                    }
                    519 => { __state = 520; }
                    520 => {
                        sqlite3_expr_code(p_parse,
                            unsafe { (*a_listelem.offset((i__1 + 1) as isize)).p_expr },
                            target);
                        __state = 521;
                    }
                    521 => {
                        unsafe { sqlite3_vdbe_goto(v, end_label) };
                        __state = 522;
                    }
                    522 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, next_case) };
                        __state = 512;
                    }
                    523 => { sqlite3_expr_delete(db__1, p_del); __state = 526; }
                    524 => {
                        sqlite3_expr_code(p_parse,
                            unsafe {
                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprListItem).offset((n_expr - 1) as isize)).p_expr
                            }, target);
                        __state = 523;
                    }
                    525 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, target) };
                        __state = 523;
                    }
                    526 => { set_do_not_merge_flag_on_copy(v); __state = 527; }
                    527 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, end_label) };
                        __state = 528;
                    }
                    528 => { __state = 20; }
                    529 => {
                        if (unsafe { (*p_parse).p_trigger_tab }).is_null() as i32 !=
                                    0 && (unsafe { (*p_parse).nested } == 0) as i32 != 0 {
                            __state = 531;
                        } else { __state = 530; }
                    }
                    530 => {
                        if unsafe { (*p_expr).aff_expr } as i32 == 2 {
                            __state = 534;
                        } else { __state = 533; }
                    }
                    531 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"RAISE() may only be used within a trigger-program".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 532;
                    }
                    532 => { return 0; }
                    533 => { { let _ = 0; }; __state = 535; }
                    534 => {
                        unsafe { sqlite3_may_abort(p_parse) };
                        __state = 533;
                    }
                    535 => {
                        if unsafe { (*p_expr).aff_expr } as i32 == 4 {
                            __state = 537;
                        } else { __state = 538; }
                    }
                    536 => { __state = 20; }
                    537 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 72, 0, 4) };
                        __state = 539;
                    }
                    538 => {
                        r1 =
                            sqlite3_expr_code_temp(p_parse, unsafe { (*p_expr).p_left },
                                &mut reg_free1);
                        __state = 540;
                    }
                    539 => { __state = 536; }
                    540 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 72,
                                if !(unsafe { (*p_parse).p_trigger_tab }).is_null() {
                                    19 | 7 << 8
                                } else { 1 }, unsafe { (*p_expr).aff_expr } as i32, r1)
                        };
                        __state = 536;
                    }
                    541 => {
                        sqlite3_release_temp_reg(unsafe { &mut *p_parse },
                            reg_free2);
                        __state = 542;
                    }
                    542 => { return in_reg; }
                    _ => {}
                }
            }
        }

        /// The VM under construction
        /// The opcode being coded
        /// Results stored in register inReg
        /// If non-zero free this temporary register
        /// If non-zero free this temporary register
        /// Various register numbers
        /// Temporary expression node
        /// Happens when the left table of a RIGHT JOIN is null and
        ///* is using an expression index
        /// No comment added
        /// This case happens when the argument to an aggregate function
        ///* is rewritten by aggregateConvertIndexedExprRefToColumn()
        /// Otherwise, fall thru into the TK_COLUMN case */
        ///      /* no break
        /// This COLUMN expression is really a constant due to WHERE clause
        ///* constraints, and that constant is coded by the pExpr->pLeft
        ///* expression.  However, make sure the constant has the correct
        ///* datatype by applying the Affinity of the table column to the
        ///* constant.
        /// Other columns in the same row for CHECK constraints or
        ///* generated columns or for inserting into partial index.
        ///* The row is unpacked into registers beginning at
        ///* 0-(pParse->iSelfTab).  The rowid (if any) is in a register
        ///* immediately prior to the first column.
        /// SQLITE_OMIT_GENERATED_COLUMNS
        /// Coding an expression that is part of an index where column names
        ///* in the index refer to the table to which the index belongs
        /// Set a range of registers to NULL.  pExpr->y.nReg registers starting
        ///* with target
        /// Make NULL the default case so that if a bug causes an illegal
        ///* Expr node to be passed into this function, it will be handled
        ///* sanely and not crash.  But keep the assert() to bring the problem
        ///* to the attention of the developers.
        /// Expressions of the form:   CAST(pLeft AS token)
        /// SQLITE_OMIT_CAST
        /// no break
        /// IS TRUE or IS NOT TRUE
        /// IS TRUE or IS FALSE
        /// List of function arguments
        /// Number of function arguments
        /// The function definition object
        /// The function name
        /// Mask of function arguments that are constant
        /// Loop counter
        /// The database connection
        /// The text encoding used by this database
        /// A collating sequence
        /// SQL functions can be expensive. So try to avoid running them
        ///* multiple times if we know they always give the same result
        /// For length() and typeof() and octet_length() functions,
        ///* set the P5 parameter to the OP_Column opcode to OPFLAG_LENGTHARG
        ///* or OPFLAG_TYPEOFARG or OPFLAG_BYTELENARG respectively, to avoid
        ///* unnecessary data loading.
        /// Possibly overload the function if the first argument is
        ///* a virtual table column.
        ///*
        ///* For infix functions (LIKE, GLOB, REGEXP, and MATCH) use the
        ///* second argument, not the first, as the argument to test to
        ///* see if it is a column in a virtual table.  This is done because
        ///* the left operand of infix functions (the operand we want to
        ///* control overloading) ends up as the second argument to the
        ///* function.  The expression "A glob B" is equivalent to
        ///* "glob(B,A).  We want to use the A in "A glob B" to test
        ///* for function overloading.  But we use the B term in "glob(B,A)".
        /// SQLITE_OMIT_SUBQUERY
        ///*    x BETWEEN y AND z
        ///*
        ///* This is equivalent to
        ///*
        ///*    x>=y AND x<=z
        ///*
        ///* X is stored in pExpr->pLeft.
        ///* Y is stored in pExpr->pList->a[0].pExpr.
        ///* Z is stored in pExpr->pList->a[1].pExpr.
        /// A TK_COLLATE Expr node without the EP_Collate tag is a so-called
        ///* "SOFT-COLLATE" that is added to constraints that are pushed down
        ///* from outer queries into sub-queries by the WHERE-clause push-down
        ///* optimization. Clear subtypes as subtypes may not cross a subquery
        ///* boundary.
        /// 2018-04-28: Prevent deep recursion.
        /// 2018-04-28: Prevent deep recursion. OSSFuzz.
        /// If the opcode is TK_TRIGGER, then the expression is a reference
        ///* to a column in the new.* or old.* pseudo-tables available to
        ///* trigger programs. In this case Expr.iTable is set to 1 for the
        ///* new.* pseudo-table, or 0 for the old.* pseudo-table. Expr.iColumn
        ///* is set to the column of the pseudo-table to read, or to -1 to
        ///* read the rowid field.
        ///*
        ///* The expression is implemented using an OP_Param opcode. The p1
        ///* parameter is set to 0 for an old.rowid reference, or to (i+1)
        ///* to reference another column of the old.* pseudo-table, where
        ///* i is the index of the column. For a new.rowid reference, p1 is
        ///* set to (n+1), where n is the number of columns in each pseudo-table.
        ///* For a reference to any other column in the new.* pseudo-table, p1
        ///* is set to (n+2+i), where n and i are as defined previously. For
        ///* example, if the table on which triggers are being fired is
        ///* declared as:
        ///*
        ///*   CREATE TABLE t1(a, b);
        ///*
        ///* Then p1 is interpreted as follows:
        ///*
        ///*   p1==0   ->    old.rowid     p1==3   ->    new.rowid
        ///*   p1==1   ->    old.a         p1==4   ->    new.a
        ///*   p1==2   ->    old.b         p1==5   ->    new.b
        /// If the column has REAL affinity, it may currently be stored as an
        ///* integer. Use OP_RealAffinity to make sure it is really real.
        ///*
        ///* EVIDENCE-OF: R-60985-57662 SQLite will convert the value back to
        ///* floating point when extracting it from the record.
        /// TK_IF_NULL_ROW Expr nodes are inserted ahead of expressions
        ///* that derive from the right-hand table of a LEFT JOIN.  The
        ///* Expr.iTable value is the table number for the right-hand table.
        ///* The expression is only evaluated if that table is not currently
        ///* on a LEFT JOIN NULL row.
        /// The OP_IfNullRow opcode above can overwrite the result register with
        ///* NULL.  So we have to ensure that the result register is not a value
        ///* that is suppose to be a constant.  Two defenses are needed:
        ///*   (1)  Temporarily disable factoring of constant expressions
        ///*   (2)  Make sure the computed value really is stored in register
        ///*        "target" and not someplace else.
        /// note (1) above
        ///* Form A:
        ///*   CASE x WHEN e1 THEN r1 WHEN e2 THEN r2 ... WHEN eN THEN rN ELSE y END
        ///*
        ///* Form B:
        ///*   CASE WHEN e1 THEN r1 WHEN e2 THEN r2 ... WHEN eN THEN rN ELSE y END
        ///*
        ///* Form A is can be transformed into the equivalent form B as follows:
        ///*   CASE WHEN x=e1 THEN r1 WHEN x=e2 THEN r2 ...
        ///*        WHEN x=eN THEN rN ELSE y END
        ///*
        ///* X (if it exists) is in pExpr->pLeft.
        ///* Y is in the last element of pExpr->x.pList if pExpr->x.pList->nExpr is
        ///* odd.  The Y is also optional.  If the number of elements in x.pList
        ///* is even, then Y is omitted and the "otherwise" result is NULL.
        ///* Ei is in pExpr->pList->a[i*2] and Ri is pExpr->pList->a[i*2+1].
        ///*
        ///* The result of the expression is the Ri for the first matching Ei,
        ///* or if there is no matching Ei, the ELSE term Y, or if there is
        ///* no ELSE term, NULL.
        /// GOTO label for end of CASE stmt
        /// GOTO label for next WHEN clause
        /// 2x number of WHEN terms
        /// Loop counter
        /// List of WHEN terms
        /// Array of WHEN terms
        /// The X==Ei expression
        /// The X expression
        /// X==Ei (form A) or just Ei (form B)
        /// Ticket b351d95f9cd5ef17e9d9dbae18f5ca8611190001:
        ///* The value in regFree1 might get SCopy-ed into the file result.
        ///* So make sure that the regFree1 register is not reused for other
        ///* purposes and possibly overwritten.
        unreachable!();
    }
}

///* Generate code that will evaluate expression pExpr and store the
///* results in register target.  The results are guaranteed to appear
///* in register target.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code(p_parse: *mut Parse, p_expr: *mut Expr,
    target: i32) -> () {
    let mut in_reg: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_parse).p_vdbe } == core::ptr::null_mut() { return; }
    in_reg = sqlite3_expr_code_target(p_parse, p_expr, target);
    if in_reg != target {
        let mut op: u8 = 0 as u8;
        let p_x: *const Expr =
            sqlite3_expr_skip_collate_and_likely(p_expr) as *const Expr;
        if !(p_x).is_null() &&
                (unsafe { (*p_x).flags } & 4194304 as u32 != 0 as u32 ||
                    unsafe { (*p_x).op } as i32 == 176) {
            op = 82 as u8;
        } else { op = 83 as u8; }
        unsafe {
            sqlite3_vdbe_add_op2(unsafe { (*p_parse).p_vdbe }, op as i32,
                in_reg, target)
        };
    }
}

///* Make a transient copy of expression pExpr and then code it using
///* sqlite3ExprCode().  This routine works just like sqlite3ExprCode()
///* except that the input expression is guaranteed to be unchanged.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_copy(p_parse: *mut Parse,
    mut p_expr: *mut Expr, target: i32) -> () {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    p_expr = sqlite3_expr_dup(db, p_expr as *const Expr, 0);
    if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
        sqlite3_expr_code(p_parse, p_expr, target);
    }
    sqlite3_expr_delete(db, p_expr);
}

/// Generate code that will load into register regOut a value that is
///* appropriate for the iIdxCol-th column of index pIdx.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_load_index_column(p_parse: *mut Parse,
    p_idx: &Index, i_tab_cur: i32, i_idx_col: i32, reg_out: i32) -> () {
    let i_tab_col: i16 =
        unsafe { *(*p_idx).ai_column.offset(i_idx_col as isize) };
    if i_tab_col as i32 == -2 {
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_parse).i_self_tab = i_tab_cur + 1 };
        sqlite3_expr_code_copy(p_parse,
            unsafe {
                (*(unsafe { (*(*p_idx).a_col_expr).a.as_ptr() } as
                                *mut ExprListItem).offset(i_idx_col as isize)).p_expr
            }, reg_out);
        unsafe { (*p_parse).i_self_tab = 0 };
    } else {
        sqlite3_expr_code_get_column_of_table(unsafe { (*p_parse).p_vdbe },
            (*p_idx).p_table, i_tab_cur, i_tab_col as i32, reg_out);
    }
}

///* Generate code to move content from registers iFrom...iFrom+nReg-1
///* over to iTo..iTo+nReg-1.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_code_move(p_parse: &Parse, i_from: i32,
    i_to: i32, n_reg: i32) -> () {
    unsafe {
        sqlite3_vdbe_add_op3((*p_parse).p_vdbe, 81, i_from, i_to, n_reg)
    };
}

///* Make arrangements to invoke OP_Null on a range of registers
///* during initialization.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_null_register_range(p_parse: *mut Parse,
    i_reg: i32, n_reg: i32) -> () {
    unsafe {
        let ok_const_factor: u8 =
            unsafe { (*p_parse).ok_const_factor() } as u8;
        let mut t: Expr = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut t as *mut (), 0,
                core::mem::size_of::<Expr>() as u64)
        };
        t.op = 83 as u8;
        t.y.n_reg = n_reg;
        unsafe { (*p_parse).set_ok_const_factor(1 as Bft as u32) };
        sqlite3_expr_code_run_just_once(p_parse, &mut t, i_reg);
        unsafe {
            (*p_parse).set_ok_const_factor(ok_const_factor as Bft as u32)
        };
    }
}

///* Like sqlite3ExprIfFalse() except that a copy is made of pExpr before
///* code generation, and that copy is deleted after code generation. This
///* ensures that the original pExpr is unchanged.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_if_false_dup(p_parse: *mut Parse,
    p_expr: *mut Expr, dest: i32, jump_if_null: i32) -> () {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let p_copy: *mut Expr = sqlite3_expr_dup(db, p_expr as *const Expr, 0);
    if unsafe { (*db).malloc_failed } as i32 == 0 {
        sqlite3_expr_if_false(p_parse, p_copy, dest, jump_if_null);
    }
    sqlite3_expr_delete(db, p_copy);
}

///* Skip over any TK_COLLATE operators.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_skip_collate(mut p_expr: *mut Expr)
    -> *mut Expr {
    while !(p_expr).is_null() &&
            unsafe { (*p_expr).flags } & 8192 as u32 != 0 as u32 {
        { let _ = 0; };
        p_expr = unsafe { (*p_expr).p_left };
    }
    return p_expr;
}

///* Like sqlite3ExprCompare() except COLLATE operators at the top-level
///* are ignored.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_compare_skip(p_a: *mut Expr, p_b: *mut Expr,
    i_tab: i32) -> i32 {
    return sqlite3_expr_compare(core::ptr::null(),
            sqlite3_expr_skip_collate(p_a) as *const Expr,
            sqlite3_expr_skip_collate(p_b) as *const Expr, i_tab);
}

///* This is a Walker expression node callback.
///*
///* For Expr nodes that contain pAggInfo pointers, make sure the AggInfo
///* object that is referenced does not refer directly to the Expr.  If
///* it does, make a copy.  This is done because the pExpr argument is
///* subject to change.
///*
///* The copy is scheduled for deletion using the sqlite3ExprDeferredDelete()
///* which builds on the sqlite3ParserAddCleanup() mechanism.
extern "C" fn agginfo_persist_expr_cb(p_walker_1: *mut Walker,
    mut p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if !(unsafe { (*p_expr_1).flags } & (65536 | 16384) as u32 !=
                                0 as u32) as i32 != 0 &&
                unsafe { (*p_expr_1).p_agg_info } != core::ptr::null_mut() {
            let p_agg_info: *const AggInfo =
                unsafe { (*p_expr_1).p_agg_info } as *const AggInfo;
            let i_agg: i32 = unsafe { (*p_expr_1).i_agg } as i32;
            let p_parse: *mut Parse = unsafe { (*p_walker_1).p_parse };
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            { let _ = 0; };
            if unsafe { (*p_expr_1).op } as i32 != 169 {
                if i_agg < unsafe { (*p_agg_info).n_column } &&
                        unsafe {
                                (*unsafe {
                                            (*p_agg_info).a_col.offset(i_agg as isize)
                                        }).p_c_expr
                            } == p_expr_1 {
                    p_expr_1 = sqlite3_expr_dup(db, p_expr_1 as *const Expr, 0);
                    if !(p_expr_1).is_null() &&
                            (sqlite3_expr_deferred_delete(p_parse, p_expr_1) == 0) as
                                    i32 != 0 {
                        unsafe {
                            (*unsafe {
                                            (*p_agg_info).a_col.offset(i_agg as isize)
                                        }).p_c_expr = p_expr_1
                        };
                    }
                }
            } else {
                { let _ = 0; };
                if i_agg < unsafe { (*p_agg_info).n_func } &&
                        unsafe {
                                (*unsafe {
                                            (*p_agg_info).a_func.offset(i_agg as isize)
                                        }).p_f_expr
                            } == p_expr_1 {
                    p_expr_1 = sqlite3_expr_dup(db, p_expr_1 as *const Expr, 0);
                    if !(p_expr_1).is_null() &&
                            (sqlite3_expr_deferred_delete(p_parse, p_expr_1) == 0) as
                                    i32 != 0 {
                        unsafe {
                            (*unsafe {
                                            (*p_agg_info).a_func.offset(i_agg as isize)
                                        }).p_f_expr = p_expr_1
                        };
                    }
                }
            }
        }
        return 0;
    }
}

///* Initialize a Walker object so that will persist AggInfo entries referenced
///* by the tree that is walked.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_agg_info_persist_walker_init(p_walker: *mut Walker,
    p_parse: *mut Parse) -> () {
    unsafe {
        memset(p_walker as *mut (), 0, core::mem::size_of::<Walker>() as u64)
    };
    unsafe { (*p_walker).p_parse = p_parse };
    unsafe { (*p_walker).x_expr_callback = Some(agginfo_persist_expr_cb) };
    unsafe { (*p_walker).x_select_callback = Some(sqlite3_select_walk_noop) };
}

///* Add a new element to the pAggInfo->aCol[] array.  Return the index of
///* the new element.  Return a negative number if malloc fails.
extern "C" fn add_agg_info_column(db: *mut Sqlite3, p_info_1: &mut AggInfo)
    -> i32 {
    let mut i: i32 = 0;
    (*p_info_1).a_col =
        unsafe {
                sqlite3_array_allocate(db, (*p_info_1).a_col as *mut (),
                    core::mem::size_of::<AggInfoCol>() as i32,
                    &mut (*p_info_1).n_column, &mut i)
            } as *mut AggInfoCol;
    return i;
}

///* Search the AggInfo object for an aCol[] entry that has iTable and iColumn.
///* Return the index in aCol[] of the entry that describes that column.
///*
///* If no prior entry is found, create a new one and return -1.  The
///* new column will have an index of pAggInfo->nColumn-1.
#[allow(unused_doc_comments)]
extern "C" fn find_or_create_agg_info_column(p_parse_1: *mut Parse,
    p_agg_info_1: *mut AggInfo, p_expr_1: *mut Expr) -> () {
    unsafe {
        let mut p_col: *mut AggInfoCol = core::ptr::null_mut();
        let mut k: i32 = 0;
        let mut mx_term: i32 = 0;
        /// OOM on resize
        let mut j: i32 = 0;
        let mut n: i32 = 0;
        let mut p_gb: *mut ExprList = core::ptr::null_mut();
        let mut p_term: *const ExprListItem = core::ptr::null();
        let mut p_e: *const Expr = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s58:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => { __state = 44; }
                    3 => { __state = 4; }
                    4 => {
                        mx_term =
                            unsafe {
                                (*unsafe { (*p_parse_1).db }).a_limit[2 as usize]
                            };
                        __state = 5;
                    }
                    5 => { { let _ = 0; }; __state = 6; }
                    6 => { { let _ = 0; }; __state = 7; }
                    7 => {
                        p_col = unsafe { (*p_agg_info_1).a_col };
                        __state = 8;
                    }
                    8 => { k = 0; __state = 10; }
                    9 => {
                        k =
                            add_agg_info_column(unsafe { (*p_parse_1).db },
                                unsafe { &mut *p_agg_info_1 });
                        __state = 16;
                    }
                    10 => {
                        if k < unsafe { (*p_agg_info_1).n_column } {
                            __state = 11;
                        } else { __state = 9; }
                    }
                    11 => {
                        if unsafe { (*p_col).p_c_expr } == p_expr_1 {
                            __state = 14;
                        } else { __state = 13; }
                    }
                    12 => {
                        {
                            { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut p_col;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 10;
                    }
                    13 => {
                        if unsafe { (*p_col).i_table } ==
                                        unsafe { (*p_expr_1).i_table } &&
                                    unsafe { (*p_col).i_column } ==
                                        unsafe { (*p_expr_1).i_column } as i32 &&
                                unsafe { (*p_expr_1).op } as i32 != 179 {
                            __state = 15;
                        } else { __state = 12; }
                    }
                    14 => { return; }
                    15 => { __state = 2; }
                    16 => { if k < 0 { __state = 18; } else { __state = 17; } }
                    17 => {
                        if k > mx_term { __state = 21; } else { __state = 20; }
                    }
                    18 => { { let _ = 0; }; __state = 19; }
                    19 => { return; }
                    20 => {
                        p_col =
                            unsafe {
                                unsafe { (*p_agg_info_1).a_col.offset(k as isize) }
                            };
                        __state = 23;
                    }
                    21 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"more than %d aggregate terms".as_ptr() as *mut i8 as
                                    *const i8, mx_term)
                        };
                        __state = 22;
                    }
                    22 => { k = mx_term; __state = 20; }
                    23 => { { let _ = 0; }; __state = 24; }
                    24 => {
                        unsafe { (*p_col).p_tab = unsafe { (*p_expr_1).y.p_tab } };
                        __state = 25;
                    }
                    25 => {
                        unsafe {
                            (*p_col).i_table = unsafe { (*p_expr_1).i_table }
                        };
                        __state = 26;
                    }
                    26 => {
                        unsafe {
                            (*p_col).i_column = unsafe { (*p_expr_1).i_column } as i32
                        };
                        __state = 27;
                    }
                    27 => {
                        unsafe { (*p_col).i_sorter_column = -1 };
                        __state = 28;
                    }
                    28 => {
                        unsafe { (*p_col).p_c_expr = p_expr_1 };
                        __state = 29;
                    }
                    29 => {
                        if !(unsafe { (*p_agg_info_1).p_group_by }).is_null() &&
                                unsafe { (*p_expr_1).op } as i32 != 179 {
                            __state = 31;
                        } else { __state = 30; }
                    }
                    30 => {
                        if unsafe { (*p_col).i_sorter_column } < 0 {
                            __state = 43;
                        } else { __state = 42; }
                    }
                    31 => { __state = 32; }
                    32 => {
                        p_gb = unsafe { (*p_agg_info_1).p_group_by };
                        __state = 33;
                    }
                    33 => {
                        p_term =
                            unsafe { (*p_gb).a.as_ptr() } as *mut ExprListItem as
                                *const ExprListItem;
                        __state = 34;
                    }
                    34 => { n = unsafe { (*p_gb).n_expr }; __state = 35; }
                    35 => { j = 0; __state = 36; }
                    36 => { if j < n { __state = 37; } else { __state = 30; } }
                    37 => {
                        p_e = unsafe { (*p_term).p_expr } as *const Expr;
                        __state = 39;
                    }
                    38 => {
                        {
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut p_term;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 36;
                    }
                    39 => {
                        if unsafe { (*p_e).op } as i32 == 168 &&
                                    unsafe { (*p_e).i_table } == unsafe { (*p_expr_1).i_table }
                                &&
                                unsafe { (*p_e).i_column } as i32 ==
                                    unsafe { (*p_expr_1).i_column } as i32 {
                            __state = 40;
                        } else { __state = 38; }
                    }
                    40 => {
                        unsafe { (*p_col).i_sorter_column = j };
                        __state = 41;
                    }
                    41 => { __state = 30; }
                    42 => { __state = 2; }
                    43 => {
                        unsafe {
                            (*p_col).i_sorter_column =
                                {
                                        let __p = unsafe { &mut (*p_agg_info_1).n_sorting_column };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as i32
                        };
                        __state = 42;
                    }
                    44 => { { let _ = 0; }; __state = 45; }
                    45 => {
                        unsafe { (*p_expr_1).p_agg_info = p_agg_info_1 };
                        __state = 46;
                    }
                    46 => {
                        if unsafe { (*p_expr_1).op } as i32 == 168 {
                            __state = 48;
                        } else { __state = 47; }
                    }
                    47 => { { let _ = 0; }; __state = 49; }
                    48 => {
                        unsafe { (*p_expr_1).op = 170 as u8 };
                        __state = 47;
                    }
                    49 => {
                        unsafe { (*p_expr_1).i_agg = k as i16 };
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

///* Add a new element to the pAggInfo->aFunc[] array.  Return the index of
///* the new element.  Return a negative number if malloc fails.
extern "C" fn add_agg_info_func(db: *mut Sqlite3, p_info_1: &mut AggInfo)
    -> i32 {
    let mut i: i32 = 0;
    (*p_info_1).a_func =
        unsafe {
                sqlite3_array_allocate(db, (*p_info_1).a_func as *mut (),
                    core::mem::size_of::<AggInfoFunc>() as i32,
                    &mut (*p_info_1).n_func, &mut i)
            } as *mut AggInfoFunc;
    return i;
}

///* This is the xExprCallback for a tree walker.  It is used to
///* implement sqlite3ExprAnalyzeAggregates().  See sqlite3ExprAnalyzeAggregates
///* for additional information.
#[allow(unused_doc_comments)]
extern "C" fn analyze_aggregate(p_walker_1: *mut Walker, p_expr_1: *mut Expr)
    -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let p_nc: *const NameContext =
            unsafe { (*p_walker_1).u.p_nc } as *const NameContext;
        let p_parse: *mut Parse = unsafe { (*p_nc).p_parse };
        let p_src_list: *mut SrcList = unsafe { (*p_nc).p_src_list };
        let p_agg_info: *mut AggInfo = unsafe { (*p_nc).u_nc.p_agg_info };
        { let _ = 0; };
        { let _ = 0; };
        '__s59:
            {
            match unsafe { (*p_expr_1).op } {
                179 => {
                    {
                        if p_src_list != core::ptr::null_mut() {
                            let mut p_item: *const SrcItem =
                                unsafe { (*p_src_list).a.as_ptr() } as *const SrcItem;
                            {
                                i = 0;
                                '__b62: loop {
                                    if !(i < unsafe { (*p_src_list).n_src }) { break '__b62; }
                                    '__c62: loop {
                                        { let _ = 0; };
                                        if unsafe { (*p_expr_1).i_table } ==
                                                unsafe { (*p_item).i_cursor } {
                                            find_or_create_agg_info_column(p_parse, p_agg_info,
                                                p_expr_1);
                                            break '__b62;
                                        }
                                        break '__c62;
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
                        return 0;
                    }
                    {
                        if unsafe { (*p_nc).nc_flags } & 131072 == 0 &&
                                    unsafe { (*p_walker_1).walker_depth } ==
                                        unsafe { (*p_expr_1).op2 } as i32 &&
                                unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
                            /// Check to see if pExpr is a duplicate of another aggregate
                            ///* function that is already in the pAggInfo structure
                            let mut p_item_1: *mut AggInfoFunc =
                                unsafe { (*p_agg_info).a_func };
                            let mx_term: i32 =
                                unsafe { (*unsafe { (*p_parse).db }).a_limit[2 as usize] };
                            { let _ = 0; };
                            {
                                i = 0;
                                '__b63: loop {
                                    if !(i < unsafe { (*p_agg_info).n_func }) { break '__b63; }
                                    '__c63: loop {
                                        if unsafe { (*p_item_1).p_f_expr } == p_expr_1 {
                                            break '__b63;
                                        }
                                        if sqlite3_expr_compare(core::ptr::null(),
                                                    unsafe { (*p_item_1).p_f_expr } as *const Expr,
                                                    p_expr_1 as *const Expr, -1) == 0 {
                                            break '__b63;
                                        }
                                        break '__c63;
                                    }
                                    {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_item_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if i > mx_term {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"more than %d aggregate terms".as_ptr() as *mut i8 as
                                            *const i8, mx_term)
                                };
                                i = mx_term;
                                { let _ = 0; };
                            } else if i >= unsafe { (*p_agg_info).n_func } {
                                /// pExpr is original.  Make a new entry in pAggInfo->aFunc[]
                                let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                                i =
                                    add_agg_info_func(unsafe { (*p_parse).db },
                                        unsafe { &mut *p_agg_info });
                                if i >= 0 {
                                    let mut n_arg: i32 = 0;
                                    { let _ = 0; };
                                    p_item_1 =
                                        unsafe {
                                            unsafe { (*p_agg_info).a_func.offset(i as isize) }
                                        };
                                    unsafe { (*p_item_1).p_f_expr = p_expr_1 };
                                    { let _ = 0; };
                                    n_arg =
                                        if !(unsafe { (*p_expr_1).x.p_list }).is_null() {
                                            unsafe { (*unsafe { (*p_expr_1).x.p_list }).n_expr }
                                        } else { 0 };
                                    unsafe {
                                        (*p_item_1).p_func =
                                            unsafe {
                                                sqlite3_find_function(unsafe { (*p_parse).db },
                                                    unsafe { (*p_expr_1).u.z_token } as *const i8, n_arg, enc,
                                                    0 as u8)
                                            }
                                    };
                                    { let _ = 0; };
                                    if !(unsafe { (*p_expr_1).p_left }).is_null() &&
                                            unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                    32 as u32 == 0 as u32 {
                                        /// The NEEDCOLL test above causes any ORDER BY clause on
                                        ///* aggregate min() or max() to be ignored.
                                        let mut p_ob_list: *const ExprList = core::ptr::null();
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        unsafe {
                                            (*p_item_1).i_ob_tab =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                        p_ob_list =
                                            unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        if unsafe { (*p_ob_list).n_expr } == 1 && n_arg == 1 &&
                                                sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*p_ob_list).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr,
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                                as *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr, 0) == 0 {
                                            unsafe { (*p_item_1).b_ob_payload = 0 as u8 };
                                            unsafe {
                                                (*p_item_1).b_ob_unique =
                                                    (unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32) as u8
                                            };
                                        } else { unsafe { (*p_item_1).b_ob_payload = 1 as u8 }; }
                                        unsafe {
                                            (*p_item_1).b_use_subtype =
                                                (unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                            1048576 as u32 != 0 as u32) as u8
                                        };
                                    } else { unsafe { (*p_item_1).i_ob_tab = -1 }; }
                                    if unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32 &&
                                            (unsafe { (*p_item_1).b_ob_unique } == 0) as i32 != 0 {
                                        unsafe {
                                            (*p_item_1).i_distinct =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                    } else { unsafe { (*p_item_1).i_distinct = -1 }; }
                                }
                            }

                            /// Make pExpr point to the appropriate pAggInfo->aFunc[] entry
                            { let _ = 0; };
                            { let _ = 0; };
                            unsafe { (*p_expr_1).i_agg = i as i16 };
                            unsafe { (*p_expr_1).p_agg_info = p_agg_info };
                            return 1;
                        } else { return 0; }
                    }
                }
                170 => {
                    {
                        if p_src_list != core::ptr::null_mut() {
                            let mut p_item: *const SrcItem =
                                unsafe { (*p_src_list).a.as_ptr() } as *const SrcItem;
                            {
                                i = 0;
                                '__b62: loop {
                                    if !(i < unsafe { (*p_src_list).n_src }) { break '__b62; }
                                    '__c62: loop {
                                        { let _ = 0; };
                                        if unsafe { (*p_expr_1).i_table } ==
                                                unsafe { (*p_item).i_cursor } {
                                            find_or_create_agg_info_column(p_parse, p_agg_info,
                                                p_expr_1);
                                            break '__b62;
                                        }
                                        break '__c62;
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
                        return 0;
                    }
                    {
                        if unsafe { (*p_nc).nc_flags } & 131072 == 0 &&
                                    unsafe { (*p_walker_1).walker_depth } ==
                                        unsafe { (*p_expr_1).op2 } as i32 &&
                                unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
                            /// Check to see if pExpr is a duplicate of another aggregate
                            ///* function that is already in the pAggInfo structure
                            let mut p_item_1: *mut AggInfoFunc =
                                unsafe { (*p_agg_info).a_func };
                            let mx_term: i32 =
                                unsafe { (*unsafe { (*p_parse).db }).a_limit[2 as usize] };
                            { let _ = 0; };
                            {
                                i = 0;
                                '__b63: loop {
                                    if !(i < unsafe { (*p_agg_info).n_func }) { break '__b63; }
                                    '__c63: loop {
                                        if unsafe { (*p_item_1).p_f_expr } == p_expr_1 {
                                            break '__b63;
                                        }
                                        if sqlite3_expr_compare(core::ptr::null(),
                                                    unsafe { (*p_item_1).p_f_expr } as *const Expr,
                                                    p_expr_1 as *const Expr, -1) == 0 {
                                            break '__b63;
                                        }
                                        break '__c63;
                                    }
                                    {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_item_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if i > mx_term {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"more than %d aggregate terms".as_ptr() as *mut i8 as
                                            *const i8, mx_term)
                                };
                                i = mx_term;
                                { let _ = 0; };
                            } else if i >= unsafe { (*p_agg_info).n_func } {
                                /// pExpr is original.  Make a new entry in pAggInfo->aFunc[]
                                let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                                i =
                                    add_agg_info_func(unsafe { (*p_parse).db },
                                        unsafe { &mut *p_agg_info });
                                if i >= 0 {
                                    let mut n_arg: i32 = 0;
                                    { let _ = 0; };
                                    p_item_1 =
                                        unsafe {
                                            unsafe { (*p_agg_info).a_func.offset(i as isize) }
                                        };
                                    unsafe { (*p_item_1).p_f_expr = p_expr_1 };
                                    { let _ = 0; };
                                    n_arg =
                                        if !(unsafe { (*p_expr_1).x.p_list }).is_null() {
                                            unsafe { (*unsafe { (*p_expr_1).x.p_list }).n_expr }
                                        } else { 0 };
                                    unsafe {
                                        (*p_item_1).p_func =
                                            unsafe {
                                                sqlite3_find_function(unsafe { (*p_parse).db },
                                                    unsafe { (*p_expr_1).u.z_token } as *const i8, n_arg, enc,
                                                    0 as u8)
                                            }
                                    };
                                    { let _ = 0; };
                                    if !(unsafe { (*p_expr_1).p_left }).is_null() &&
                                            unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                    32 as u32 == 0 as u32 {
                                        /// The NEEDCOLL test above causes any ORDER BY clause on
                                        ///* aggregate min() or max() to be ignored.
                                        let mut p_ob_list: *const ExprList = core::ptr::null();
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        unsafe {
                                            (*p_item_1).i_ob_tab =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                        p_ob_list =
                                            unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        if unsafe { (*p_ob_list).n_expr } == 1 && n_arg == 1 &&
                                                sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*p_ob_list).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr,
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                                as *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr, 0) == 0 {
                                            unsafe { (*p_item_1).b_ob_payload = 0 as u8 };
                                            unsafe {
                                                (*p_item_1).b_ob_unique =
                                                    (unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32) as u8
                                            };
                                        } else { unsafe { (*p_item_1).b_ob_payload = 1 as u8 }; }
                                        unsafe {
                                            (*p_item_1).b_use_subtype =
                                                (unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                            1048576 as u32 != 0 as u32) as u8
                                        };
                                    } else { unsafe { (*p_item_1).i_ob_tab = -1 }; }
                                    if unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32 &&
                                            (unsafe { (*p_item_1).b_ob_unique } == 0) as i32 != 0 {
                                        unsafe {
                                            (*p_item_1).i_distinct =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                    } else { unsafe { (*p_item_1).i_distinct = -1 }; }
                                }
                            }

                            /// Make pExpr point to the appropriate pAggInfo->aFunc[] entry
                            { let _ = 0; };
                            { let _ = 0; };
                            unsafe { (*p_expr_1).i_agg = i as i16 };
                            unsafe { (*p_expr_1).p_agg_info = p_agg_info };
                            return 1;
                        } else { return 0; }
                    }
                }
                168 => {
                    {
                        if p_src_list != core::ptr::null_mut() {
                            let mut p_item: *const SrcItem =
                                unsafe { (*p_src_list).a.as_ptr() } as *const SrcItem;
                            {
                                i = 0;
                                '__b62: loop {
                                    if !(i < unsafe { (*p_src_list).n_src }) { break '__b62; }
                                    '__c62: loop {
                                        { let _ = 0; };
                                        if unsafe { (*p_expr_1).i_table } ==
                                                unsafe { (*p_item).i_cursor } {
                                            find_or_create_agg_info_column(p_parse, p_agg_info,
                                                p_expr_1);
                                            break '__b62;
                                        }
                                        break '__c62;
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
                        return 0;
                    }
                    {
                        if unsafe { (*p_nc).nc_flags } & 131072 == 0 &&
                                    unsafe { (*p_walker_1).walker_depth } ==
                                        unsafe { (*p_expr_1).op2 } as i32 &&
                                unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
                            /// Check to see if pExpr is a duplicate of another aggregate
                            ///* function that is already in the pAggInfo structure
                            let mut p_item_1: *mut AggInfoFunc =
                                unsafe { (*p_agg_info).a_func };
                            let mx_term: i32 =
                                unsafe { (*unsafe { (*p_parse).db }).a_limit[2 as usize] };
                            { let _ = 0; };
                            {
                                i = 0;
                                '__b63: loop {
                                    if !(i < unsafe { (*p_agg_info).n_func }) { break '__b63; }
                                    '__c63: loop {
                                        if unsafe { (*p_item_1).p_f_expr } == p_expr_1 {
                                            break '__b63;
                                        }
                                        if sqlite3_expr_compare(core::ptr::null(),
                                                    unsafe { (*p_item_1).p_f_expr } as *const Expr,
                                                    p_expr_1 as *const Expr, -1) == 0 {
                                            break '__b63;
                                        }
                                        break '__c63;
                                    }
                                    {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_item_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if i > mx_term {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"more than %d aggregate terms".as_ptr() as *mut i8 as
                                            *const i8, mx_term)
                                };
                                i = mx_term;
                                { let _ = 0; };
                            } else if i >= unsafe { (*p_agg_info).n_func } {
                                /// pExpr is original.  Make a new entry in pAggInfo->aFunc[]
                                let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                                i =
                                    add_agg_info_func(unsafe { (*p_parse).db },
                                        unsafe { &mut *p_agg_info });
                                if i >= 0 {
                                    let mut n_arg: i32 = 0;
                                    { let _ = 0; };
                                    p_item_1 =
                                        unsafe {
                                            unsafe { (*p_agg_info).a_func.offset(i as isize) }
                                        };
                                    unsafe { (*p_item_1).p_f_expr = p_expr_1 };
                                    { let _ = 0; };
                                    n_arg =
                                        if !(unsafe { (*p_expr_1).x.p_list }).is_null() {
                                            unsafe { (*unsafe { (*p_expr_1).x.p_list }).n_expr }
                                        } else { 0 };
                                    unsafe {
                                        (*p_item_1).p_func =
                                            unsafe {
                                                sqlite3_find_function(unsafe { (*p_parse).db },
                                                    unsafe { (*p_expr_1).u.z_token } as *const i8, n_arg, enc,
                                                    0 as u8)
                                            }
                                    };
                                    { let _ = 0; };
                                    if !(unsafe { (*p_expr_1).p_left }).is_null() &&
                                            unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                    32 as u32 == 0 as u32 {
                                        /// The NEEDCOLL test above causes any ORDER BY clause on
                                        ///* aggregate min() or max() to be ignored.
                                        let mut p_ob_list: *const ExprList = core::ptr::null();
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        unsafe {
                                            (*p_item_1).i_ob_tab =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                        p_ob_list =
                                            unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        if unsafe { (*p_ob_list).n_expr } == 1 && n_arg == 1 &&
                                                sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*p_ob_list).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr,
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                                as *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr, 0) == 0 {
                                            unsafe { (*p_item_1).b_ob_payload = 0 as u8 };
                                            unsafe {
                                                (*p_item_1).b_ob_unique =
                                                    (unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32) as u8
                                            };
                                        } else { unsafe { (*p_item_1).b_ob_payload = 1 as u8 }; }
                                        unsafe {
                                            (*p_item_1).b_use_subtype =
                                                (unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                            1048576 as u32 != 0 as u32) as u8
                                        };
                                    } else { unsafe { (*p_item_1).i_ob_tab = -1 }; }
                                    if unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32 &&
                                            (unsafe { (*p_item_1).b_ob_unique } == 0) as i32 != 0 {
                                        unsafe {
                                            (*p_item_1).i_distinct =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                    } else { unsafe { (*p_item_1).i_distinct = -1 }; }
                                }
                            }

                            /// Make pExpr point to the appropriate pAggInfo->aFunc[] entry
                            { let _ = 0; };
                            { let _ = 0; };
                            unsafe { (*p_expr_1).i_agg = i as i16 };
                            unsafe { (*p_expr_1).p_agg_info = p_agg_info };
                            return 1;
                        } else { return 0; }
                    }
                }
                169 => {
                    {
                        if unsafe { (*p_nc).nc_flags } & 131072 == 0 &&
                                    unsafe { (*p_walker_1).walker_depth } ==
                                        unsafe { (*p_expr_1).op2 } as i32 &&
                                unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
                            /// Check to see if pExpr is a duplicate of another aggregate
                            ///* function that is already in the pAggInfo structure
                            let mut p_item_1: *mut AggInfoFunc =
                                unsafe { (*p_agg_info).a_func };
                            let mx_term: i32 =
                                unsafe { (*unsafe { (*p_parse).db }).a_limit[2 as usize] };
                            { let _ = 0; };
                            {
                                i = 0;
                                '__b63: loop {
                                    if !(i < unsafe { (*p_agg_info).n_func }) { break '__b63; }
                                    '__c63: loop {
                                        if unsafe { (*p_item_1).p_f_expr } == p_expr_1 {
                                            break '__b63;
                                        }
                                        if sqlite3_expr_compare(core::ptr::null(),
                                                    unsafe { (*p_item_1).p_f_expr } as *const Expr,
                                                    p_expr_1 as *const Expr, -1) == 0 {
                                            break '__b63;
                                        }
                                        break '__c63;
                                    }
                                    {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_item_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if i > mx_term {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"more than %d aggregate terms".as_ptr() as *mut i8 as
                                            *const i8, mx_term)
                                };
                                i = mx_term;
                                { let _ = 0; };
                            } else if i >= unsafe { (*p_agg_info).n_func } {
                                /// pExpr is original.  Make a new entry in pAggInfo->aFunc[]
                                let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                                i =
                                    add_agg_info_func(unsafe { (*p_parse).db },
                                        unsafe { &mut *p_agg_info });
                                if i >= 0 {
                                    let mut n_arg: i32 = 0;
                                    { let _ = 0; };
                                    p_item_1 =
                                        unsafe {
                                            unsafe { (*p_agg_info).a_func.offset(i as isize) }
                                        };
                                    unsafe { (*p_item_1).p_f_expr = p_expr_1 };
                                    { let _ = 0; };
                                    n_arg =
                                        if !(unsafe { (*p_expr_1).x.p_list }).is_null() {
                                            unsafe { (*unsafe { (*p_expr_1).x.p_list }).n_expr }
                                        } else { 0 };
                                    unsafe {
                                        (*p_item_1).p_func =
                                            unsafe {
                                                sqlite3_find_function(unsafe { (*p_parse).db },
                                                    unsafe { (*p_expr_1).u.z_token } as *const i8, n_arg, enc,
                                                    0 as u8)
                                            }
                                    };
                                    { let _ = 0; };
                                    if !(unsafe { (*p_expr_1).p_left }).is_null() &&
                                            unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                    32 as u32 == 0 as u32 {
                                        /// The NEEDCOLL test above causes any ORDER BY clause on
                                        ///* aggregate min() or max() to be ignored.
                                        let mut p_ob_list: *const ExprList = core::ptr::null();
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        unsafe {
                                            (*p_item_1).i_ob_tab =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                        p_ob_list =
                                            unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        if unsafe { (*p_ob_list).n_expr } == 1 && n_arg == 1 &&
                                                sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*p_ob_list).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr,
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                                as *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr, 0) == 0 {
                                            unsafe { (*p_item_1).b_ob_payload = 0 as u8 };
                                            unsafe {
                                                (*p_item_1).b_ob_unique =
                                                    (unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32) as u8
                                            };
                                        } else { unsafe { (*p_item_1).b_ob_payload = 1 as u8 }; }
                                        unsafe {
                                            (*p_item_1).b_use_subtype =
                                                (unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                            1048576 as u32 != 0 as u32) as u8
                                        };
                                    } else { unsafe { (*p_item_1).i_ob_tab = -1 }; }
                                    if unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32 &&
                                            (unsafe { (*p_item_1).b_ob_unique } == 0) as i32 != 0 {
                                        unsafe {
                                            (*p_item_1).i_distinct =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                    } else { unsafe { (*p_item_1).i_distinct = -1 }; }
                                }
                            }

                            /// Make pExpr point to the appropriate pAggInfo->aFunc[] entry
                            { let _ = 0; };
                            { let _ = 0; };
                            unsafe { (*p_expr_1).i_agg = i as i16 };
                            unsafe { (*p_expr_1).p_agg_info = p_agg_info };
                            return 1;
                        } else { return 0; }
                    }
                }
                _ => {
                    {
                        let mut p_i_epr: *const IndexedExpr = core::ptr::null();
                        let mut tmp: Expr = unsafe { core::mem::zeroed() };
                        { let _ = 0; };
                        if unsafe { (*p_nc).nc_flags } & 131072 == 0 {
                            break '__s59;
                        }
                        if unsafe { (*p_parse).p_idx_epr } == core::ptr::null_mut()
                            {
                            break '__s59;
                        }
                        {
                            p_i_epr = unsafe { (*p_parse).p_idx_epr };
                            '__b60: loop {
                                if !(!(p_i_epr).is_null()) { break '__b60; }
                                '__c60: loop {
                                    let i_data_cur: i32 = unsafe { (*p_i_epr).i_data_cur };
                                    if i_data_cur < 0 { break '__c60; }
                                    if sqlite3_expr_compare(core::ptr::null(),
                                                p_expr_1 as *const Expr,
                                                unsafe { (*p_i_epr).p_expr } as *const Expr, i_data_cur) ==
                                            0 {
                                        break '__b60;
                                    }
                                    break '__c60;
                                }
                                p_i_epr = unsafe { (*p_i_epr).p_ie_next };
                            }
                        }
                        if p_i_epr == core::ptr::null_mut() { break '__s59; }
                        if !(unsafe { (*p_expr_1).flags } &
                                                (16777216 | 33554432) as u32 == 0 as u32) as i32 != 0 {
                            break '__s59;
                        }
                        {
                            i = 0;
                            '__b61: loop {
                                if !(i < unsafe { (*p_src_list).n_src }) { break '__b61; }
                                '__c61: loop {
                                    if unsafe {
                                                (*(unsafe { (*p_src_list).a.as_ptr() } as
                                                                *mut SrcItem).offset(i as isize)).i_cursor
                                            } == unsafe { (*p_i_epr).i_data_cur } {
                                        break '__b61;
                                    }
                                    break '__c61;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        if i >= unsafe { (*p_src_list).n_src } { break '__s59; }
                        if unsafe { (*p_expr_1).p_agg_info } !=
                                core::ptr::null_mut() {
                            break '__s59;
                        }
                        if unsafe { (*p_parse).n_err } != 0 { return 2; }

                        /// If we reach this point, it means that expression pExpr can be
                        ///* translated into a reference to an index column as described by
                        ///* pIEpr.
                        unsafe {
                            memset(&raw mut tmp as *mut (), 0,
                                core::mem::size_of::<Expr>() as u64)
                        };
                        tmp.op = 170 as u8;
                        tmp.i_table = unsafe { (*p_i_epr).i_idx_cur };
                        tmp.i_column = unsafe { (*p_i_epr).i_idx_col } as YnVar;
                        find_or_create_agg_info_column(p_parse, p_agg_info,
                            &mut tmp);
                        if unsafe { (*p_parse).n_err } != 0 { return 2; }
                        { let _ = 0; };
                        { let _ = 0; };
                        unsafe {
                            (*unsafe {
                                            (*p_agg_info).a_col.offset(tmp.i_agg as isize)
                                        }).p_c_expr = p_expr_1
                        };
                        unsafe { (*p_expr_1).p_agg_info = p_agg_info };
                        unsafe { (*p_expr_1).i_agg = tmp.i_agg };
                        return 1;
                    }
                    {
                        if p_src_list != core::ptr::null_mut() {
                            let mut p_item: *const SrcItem =
                                unsafe { (*p_src_list).a.as_ptr() } as *const SrcItem;
                            {
                                i = 0;
                                '__b62: loop {
                                    if !(i < unsafe { (*p_src_list).n_src }) { break '__b62; }
                                    '__c62: loop {
                                        { let _ = 0; };
                                        if unsafe { (*p_expr_1).i_table } ==
                                                unsafe { (*p_item).i_cursor } {
                                            find_or_create_agg_info_column(p_parse, p_agg_info,
                                                p_expr_1);
                                            break '__b62;
                                        }
                                        break '__c62;
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
                        return 0;
                    }
                    {
                        if unsafe { (*p_nc).nc_flags } & 131072 == 0 &&
                                    unsafe { (*p_walker_1).walker_depth } ==
                                        unsafe { (*p_expr_1).op2 } as i32 &&
                                unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
                            /// Check to see if pExpr is a duplicate of another aggregate
                            ///* function that is already in the pAggInfo structure
                            let mut p_item_1: *mut AggInfoFunc =
                                unsafe { (*p_agg_info).a_func };
                            let mx_term: i32 =
                                unsafe { (*unsafe { (*p_parse).db }).a_limit[2 as usize] };
                            { let _ = 0; };
                            {
                                i = 0;
                                '__b63: loop {
                                    if !(i < unsafe { (*p_agg_info).n_func }) { break '__b63; }
                                    '__c63: loop {
                                        if unsafe { (*p_item_1).p_f_expr } == p_expr_1 {
                                            break '__b63;
                                        }
                                        if sqlite3_expr_compare(core::ptr::null(),
                                                    unsafe { (*p_item_1).p_f_expr } as *const Expr,
                                                    p_expr_1 as *const Expr, -1) == 0 {
                                            break '__b63;
                                        }
                                        break '__c63;
                                    }
                                    {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_item_1;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if i > mx_term {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"more than %d aggregate terms".as_ptr() as *mut i8 as
                                            *const i8, mx_term)
                                };
                                i = mx_term;
                                { let _ = 0; };
                            } else if i >= unsafe { (*p_agg_info).n_func } {
                                /// pExpr is original.  Make a new entry in pAggInfo->aFunc[]
                                let enc: u8 = unsafe { (*unsafe { (*p_parse).db }).enc };
                                i =
                                    add_agg_info_func(unsafe { (*p_parse).db },
                                        unsafe { &mut *p_agg_info });
                                if i >= 0 {
                                    let mut n_arg: i32 = 0;
                                    { let _ = 0; };
                                    p_item_1 =
                                        unsafe {
                                            unsafe { (*p_agg_info).a_func.offset(i as isize) }
                                        };
                                    unsafe { (*p_item_1).p_f_expr = p_expr_1 };
                                    { let _ = 0; };
                                    n_arg =
                                        if !(unsafe { (*p_expr_1).x.p_list }).is_null() {
                                            unsafe { (*unsafe { (*p_expr_1).x.p_list }).n_expr }
                                        } else { 0 };
                                    unsafe {
                                        (*p_item_1).p_func =
                                            unsafe {
                                                sqlite3_find_function(unsafe { (*p_parse).db },
                                                    unsafe { (*p_expr_1).u.z_token } as *const i8, n_arg, enc,
                                                    0 as u8)
                                            }
                                    };
                                    { let _ = 0; };
                                    if !(unsafe { (*p_expr_1).p_left }).is_null() &&
                                            unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                    32 as u32 == 0 as u32 {
                                        /// The NEEDCOLL test above causes any ORDER BY clause on
                                        ///* aggregate min() or max() to be ignored.
                                        let mut p_ob_list: *const ExprList = core::ptr::null();
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        unsafe {
                                            (*p_item_1).i_ob_tab =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                        p_ob_list =
                                            unsafe { (*unsafe { (*p_expr_1).p_left }).x.p_list };
                                        { let _ = 0; };
                                        { let _ = 0; };
                                        if unsafe { (*p_ob_list).n_expr } == 1 && n_arg == 1 &&
                                                sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*p_ob_list).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr,
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p_expr_1).x.p_list }).a.as_ptr() }
                                                                                as *mut ExprListItem).offset(0 as isize)).p_expr
                                                            } as *const Expr, 0) == 0 {
                                            unsafe { (*p_item_1).b_ob_payload = 0 as u8 };
                                            unsafe {
                                                (*p_item_1).b_ob_unique =
                                                    (unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32) as u8
                                            };
                                        } else { unsafe { (*p_item_1).b_ob_payload = 1 as u8 }; }
                                        unsafe {
                                            (*p_item_1).b_use_subtype =
                                                (unsafe { (*unsafe { (*p_item_1).p_func }).func_flags } &
                                                            1048576 as u32 != 0 as u32) as u8
                                        };
                                    } else { unsafe { (*p_item_1).i_ob_tab = -1 }; }
                                    if unsafe { (*p_expr_1).flags } & 4 as u32 != 0 as u32 &&
                                            (unsafe { (*p_item_1).b_ob_unique } == 0) as i32 != 0 {
                                        unsafe {
                                            (*p_item_1).i_distinct =
                                                {
                                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                }
                                        };
                                    } else { unsafe { (*p_item_1).i_distinct = -1 }; }
                                }
                            }

                            /// Make pExpr point to the appropriate pAggInfo->aFunc[] entry
                            { let _ = 0; };
                            { let _ = 0; };
                            unsafe { (*p_expr_1).i_agg = i as i16 };
                            unsafe { (*p_expr_1).p_agg_info = p_agg_info };
                            return 1;
                        } else { return 0; }
                    }
                }
            }
        }
        return 0;
    }
}

///* Analyze the pExpr expression looking for aggregate functions and
///* for variables that need to be added to AggInfo object that pNC->pAggInfo
///* points to.  Additional entries are made on the AggInfo object as
///* necessary.
///*
///* This routine should only be called after the expression has been
///* analyzed by sqlite3ResolveExprNames().
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_analyze_aggregates(p_nc: *mut NameContext,
    p_expr: *mut Expr) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        w.x_expr_callback = Some(analyze_aggregate);
        w.x_select_callback = Some(sqlite3_walker_depth_increase);
        w.x_select_callback2 = Some(sqlite3_walker_depth_decrease);
        w.walker_depth = 0;
        w.u.p_nc = p_nc;
        w.p_parse = core::ptr::null_mut();
        { let _ = 0; };
        unsafe { sqlite3_walk_expr(&mut w, p_expr) };
    }
}

///* Call sqlite3ExprAnalyzeAggregates() for every expression in an
///* expression list.  Return the number of errors.
///*
///* If an error is found, the analysis is cut short.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_analyze_agg_list(p_nc: *mut NameContext,
    p_list: *mut ExprList) -> () {
    let mut p_item: *const ExprListItem = core::ptr::null();
    let mut i: i32 = 0;
    if !(p_list).is_null() {
        {
            {
                p_item = unsafe { (*p_list).a.as_ptr() } as *mut ExprListItem;
                i = 0
            };
            '__b64: loop {
                if !(i < unsafe { (*p_list).n_expr }) { break '__b64; }
                '__c64: loop {
                    sqlite3_expr_analyze_aggregates(p_nc,
                        unsafe { (*p_item).p_expr });
                    break '__c64;
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

///* Check to see if there are references to columns in table
///* pWalker->u.pIdxCover->iCur can be satisfied using the index
///* pWalker->u.pIdxCover->pIdx.
extern "C" fn expr_idx_cover(p_walker_1: *mut Walker, p_expr_1: *mut Expr)
    -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 168 &&
                    unsafe { (*p_expr_1).i_table } ==
                        unsafe { (*unsafe { (*p_walker_1).u.p_idx_cover }).i_cur }
                &&
                unsafe {
                        sqlite3_table_column_to_index(unsafe {
                                (*unsafe { (*p_walker_1).u.p_idx_cover }).p_idx
                            }, unsafe { (*p_expr_1).i_column } as i32)
                    } < 0 {
            unsafe { (*p_walker_1).e_code = 1 as u16 };
            return 2;
        }
        return 0;
    }
}

///* Determine if an index pIdx on table with cursor iCur contains will
///* the expression pExpr.  Return true if the index does cover the
///* expression and false if the pExpr expression references table columns
///* that are not found in the index pIdx.
///*
///* An index covering an expression means that the expression can be
///* evaluated using only the index and without having to lookup the
///* corresponding table entry.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_covered_by_index(p_expr: *mut Expr, i_cur: i32,
    p_idx: *mut Index) -> i32 {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        let mut xcov: IdxCover = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        xcov.i_cur = i_cur;
        xcov.p_idx = p_idx;
        w.x_expr_callback = Some(expr_idx_cover);
        w.u.p_idx_cover = &mut xcov;
        unsafe { sqlite3_walk_expr(&mut w, p_expr) };
        return (w.e_code == 0) as i32 as i32;
    }
}

/// This is the Walker EXPR callback for sqlite3ReferencesSrcList().
///*
///* Set the 0x01 bit of pWalker->eCode if there is a reference to any
///* of the tables shown in RefSrcList.pRef.
///*
///* Set the 0x02 bit of pWalker->eCode if there is a reference to a
///* table is in neither RefSrcList.pRef nor RefSrcList.aiExclude.
extern "C" fn expr_ref_to_src_list(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 168 ||
                unsafe { (*p_expr_1).op } as i32 == 170 {
            let mut i: i32 = 0;
            let p: *const RefSrcList =
                unsafe { (*p_walker_1).u.p_ref_src_list } as
                    *const RefSrcList;
            let p_src: *const SrcList =
                unsafe { (*p).p_ref } as *const SrcList;
            let n_src: i32 =
                if !(p_src).is_null() {
                    unsafe { (*p_src).n_src }
                } else { 0 };
            {
                i = 0;
                '__b65: loop {
                    if !(i < n_src) { break '__b65; }
                    '__c65: loop {
                        if unsafe { (*p_expr_1).i_table } ==
                                unsafe {
                                    (*(unsafe { (*p_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(i as isize)).i_cursor
                                } {
                            unsafe { (*p_walker_1).e_code |= 1 as u16 };
                            return 0;
                        }
                        break '__c65;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            {
                i = 0;
                '__b66: loop {
                    if !((i as i64) < unsafe { (*p).n_exclude } &&
                                    unsafe { *unsafe { (*p).ai_exclude.offset(i as isize) } } !=
                                        unsafe { (*p_expr_1).i_table }) {
                        break '__b66;
                    }
                    '__c66: loop { break '__c66; }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if i as i64 >= unsafe { (*p).n_exclude } {
                unsafe { (*p_walker_1).e_code |= 2 as u16 };
            }
        }
        return 0;
    }
}

///* Walker SELECT callbacks for sqlite3ReferencesSrcList().
///*
///* When entering a new subquery on the pExpr argument, add all FROM clause
///* entries for that subquery to the exclude list.
///*
///* When leaving the subquery, remove those entries from the exclude list.
extern "C" fn select_ref_enter(p_walker_1: *mut Walker,
    p_select_1: *mut Select) -> i32 {
    unsafe {
        let p: *mut RefSrcList = unsafe { (*p_walker_1).u.p_ref_src_list };
        let p_src: *const SrcList =
            unsafe { (*p_select_1).p_src } as *const SrcList;
        let mut i: i64 = 0 as i64;
        let mut j: i64 = 0 as i64;
        let mut pi_new: *mut i32 = core::ptr::null_mut();
        if unsafe { (*p_src).n_src } == 0 { return 0; }
        j = unsafe { (*p).n_exclude };
        unsafe { (*p).n_exclude += unsafe { (*p_src).n_src } as i64 };
        pi_new =
            unsafe {
                    sqlite3_db_realloc(unsafe { (*p).db },
                        unsafe { (*p).ai_exclude } as *mut (),
                        unsafe { (*p).n_exclude } as u64 *
                            core::mem::size_of::<i32>() as u64)
                } as *mut i32;
        if pi_new == core::ptr::null_mut() {
            unsafe { (*p).n_exclude = 0 as i64 };
            return 2;
        } else { unsafe { (*p).ai_exclude = pi_new }; }
        {
            i = 0 as i64;
            '__b67: loop {
                if !(i < unsafe { (*p_src).n_src } as i64) { break '__b67; }
                '__c67: loop {
                    unsafe {
                        *unsafe { (*p).ai_exclude.offset(j as isize) } =
                            unsafe {
                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)).i_cursor
                            }
                    };
                    break '__c67;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        return 0;
    }
}

extern "C" fn select_ref_leave(p_walker_1: *mut Walker,
    p_select_1: *mut Select) -> () {
    unsafe {
        let p: *mut RefSrcList = unsafe { (*p_walker_1).u.p_ref_src_list };
        let p_src: *const SrcList =
            unsafe { (*p_select_1).p_src } as *const SrcList;
        if unsafe { (*p).n_exclude } != 0 {
            { let _ = 0; };
            unsafe { (*p).n_exclude -= unsafe { (*p_src).n_src } as i64 };
        }
    }
}

///* Check to see if pExpr references any tables in pSrcList.
///* Possible return values:
///*
///*    1         pExpr does references a table in pSrcList.
///*
///*    0         pExpr references some table that is not defined in either
///*              pSrcList or in subqueries of pExpr itself.
///*
///*   -1         pExpr only references no tables at all, or it only
///*              references tables defined in subqueries of pExpr itself.
///*
///* As currently used, pExpr is always an aggregate function call.  That
///* fact is exploited for efficiency.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_references_src_list(p_parse: &Parse, p_expr: &Expr,
    p_src_list: *mut SrcList) -> i32 {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        let mut x: RefSrcList = unsafe { core::mem::zeroed() };
        { let _ = 0; };
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        unsafe {
            memset(&raw mut x as *mut (), 0,
                core::mem::size_of::<RefSrcList>() as u64)
        };
        w.x_expr_callback = Some(expr_ref_to_src_list);
        w.x_select_callback = Some(select_ref_enter);
        w.x_select_callback2 = Some(select_ref_leave);
        w.u.p_ref_src_list = &mut x;
        x.db = (*p_parse).db;
        x.p_ref = p_src_list;
        { let _ = 0; };
        { let _ = 0; };
        unsafe { sqlite3_walk_expr_list(&mut w, (*p_expr).x.p_list) };
        if !((*p_expr).p_left).is_null() {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                sqlite3_walk_expr_list(&mut w,
                    unsafe { (*(*p_expr).p_left).x.p_list })
            };
        }
        if (*p_expr).flags & 16777216 as u32 != 0 as u32 {
            unsafe {
                sqlite3_walk_expr(&mut w,
                    unsafe { (*(*p_expr).y.p_win).p_filter })
            };
        }
        if !(x.ai_exclude).is_null() {
            unsafe {
                sqlite3_db_nn_free_nn((*p_parse).db, x.ai_exclude as *mut ())
            };
        }
        if w.e_code as i32 & 1 != 0 {
            return 1;
        } else if w.e_code != 0 { return 0; } else { return -1; }
    }
}

///* Walk an expression tree for the DEFAULT field of a column definition
///* in a CREATE TABLE statement.  Return non-zero if the expression is
///* acceptable for use as a DEFAULT.  That is to say, return non-zero if
///* the expression is constant or a function call with constant arguments.
///* Return and 0 if there are any variables.
///*
///* isInit is true when parsing from sqlite_schema.  isInit is false when
///* processing a new CREATE TABLE statement.  When isInit is true, parameters
///* (such as ? or $abc) in the expression are converted into NULL.  When
///* isInit is false, parameters raise an error.  Parameters should not be
///* allowed in a CREATE TABLE statement, but some legacy versions of SQLite
///* allowed it, so we need to support it when reading sqlite_schema for
///* backwards compatibility.
///*
///* If isInit is true, set EP_FromDDL on every TK_FUNCTION node.
///*
///* For the purposes of this function, a double-quoted string (ex: "abc")
///* is considered a variable but a single-quoted string (ex: 'abc') is
///* a constant.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_is_constant_or_function(p: *mut Expr,
    is_init: u8) -> i32 {
    { let _ = 0; };
    return expr_is_const(core::ptr::null_mut(), p, 4 + is_init as i32);
}

///* Return the collation sequence for the expression pExpr. If
///* there is no defined collating sequence, return a pointer to the
///* default collation sequence.
///*
///* See also: sqlite3ExprCollSeq()
///*
///* The sqlite3ExprCollSeq() routine works the same except that it
///* returns NULL if there is no defined collation.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_nn_coll_seq(p_parse: *mut Parse,
    p_expr: *const Expr) -> *mut CollSeq {
    let mut p: *mut CollSeq = sqlite3_expr_coll_seq(p_parse, p_expr);
    if p == core::ptr::null_mut() {
        p = unsafe { (*unsafe { (*p_parse).db }).p_dflt_coll };
    }
    { let _ = 0; };
    return p;
}

///* sqlite3WalkExpr() callback used by sqlite3ExprIsConstantOrGroupBy().
extern "C" fn expr_node_is_constant_or_group_by(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let p_group_by: *const ExprList =
            unsafe { (*p_walker_1).u.p_group_by } as *const ExprList;
        let mut i: i32 = 0;
        {
            i = 0;
            '__b68: loop {
                if !(i < unsafe { (*p_group_by).n_expr }) { break '__b68; }
                '__c68: loop {
                    let p: *const Expr =
                        unsafe {
                                (*(unsafe { (*p_group_by).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)).p_expr
                            } as *const Expr;
                    if sqlite3_expr_compare(core::ptr::null(),
                                p_expr_1 as *const Expr, p as *const Expr, -1) < 2 {
                        let p_coll: *const CollSeq =
                            sqlite3_expr_nn_coll_seq(unsafe { (*p_walker_1).p_parse },
                                    p as *const Expr) as *const CollSeq;
                        if unsafe { sqlite3_is_binary(p_coll as *const CollSeq) } !=
                                0 {
                            return 1;
                        }
                    }
                    break '__c68;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
            unsafe { (*p_walker_1).e_code = 0 as u16 };
            return 2;
        }
        return expr_node_is_constant(p_walker_1, p_expr_1);
    }
}

///* Walk the expression tree passed as the first argument. Return non-zero
///* if the expression consists entirely of constants or copies of terms
///* in pGroupBy that sort with the BINARY collation sequence.
///*
///* This routine is used to determine if a term of the HAVING clause can
///* be promoted into the WHERE clause.  In order for such a promotion to work,
///* the value of the HAVING clause term must be the same for all members of
///* a "group".  The requirement that the GROUP BY term must be BINARY
///* assumes that no other collating sequence will have a finer-grained
///* grouping than binary.  In other words (A=B COLLATE binary) implies
///* A=B in every other collating sequence.  The requirement that the
///* GROUP BY be BINARY is stricter than necessary.  It would also work
///* to promote HAVING clauses that use the same alternative collating
///* sequence as the GROUP BY term, but that is much harder to check,
///* alternative collating sequences are uncommon, and this is only an
///* optimization, so we take the easy way out and simply require the
///* GROUP BY to use the BINARY collating sequence.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_is_constant_or_group_by(p_parse: *mut Parse,
    p: *mut Expr, p_group_by: *mut ExprList) -> i32 {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        w.e_code = 1 as u16;
        w.x_expr_callback = Some(expr_node_is_constant_or_group_by);
        w.x_select_callback = None;
        w.u.p_group_by = p_group_by;
        w.p_parse = p_parse;
        unsafe { sqlite3_walk_expr(&mut w, p) };
        return w.e_code as i32;
    }
}

///* This routine examines sub-SELECT statements as an expression is being
///* walked as part of sqlite3ExprIsTableConstant().  Sub-SELECTs are considered
///* constant as long as they are uncorrelated - meaning that they do not
///* contain any terms from outer contexts.
extern "C" fn expr_select_walk_table_constant(p_walker_1: *mut Walker,
    p_select_1: *mut Select) -> i32 {
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_select_1).sel_flags } & 536870912 as u32 != 0 as u32 {
        unsafe { (*p_walker_1).e_code = 0 as u16 };
        return 2;
    }
    return 1;
}

///* Walk an expression tree.  Return non-zero if the expression is constant
///* for any single row of the table with cursor iCur.  In other words, the
///* expression must not refer to any non-deterministic function nor any
///* table other than iCur.
///*
///* Consider uncorrelated subqueries to be constants if the bAllowSubq
///* parameter is true.
extern "C" fn sqlite3_expr_is_table_constant(p: *mut Expr, i_cur_1: i32,
    b_allow_subq_1: i32) -> i32 {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        w.e_code = 3 as u16;
        w.p_parse = core::ptr::null_mut();
        w.x_expr_callback = Some(expr_node_is_constant);
        if b_allow_subq_1 != 0 {
            w.x_select_callback = Some(expr_select_walk_table_constant);
        } else { w.x_select_callback = Some(sqlite3_select_walk_fail); }
        w.u.i_cur = i_cur_1;
        unsafe { sqlite3_walk_expr(&mut w, p) };
        return w.e_code as i32;
    }
}

///* Check pExpr to see if it is an constraint on the single data source
///* pSrc = &pSrcList->a[iSrc].  In other words, check to see if pExpr
///* constrains pSrc but does not depend on any other tables or data
///* sources anywhere else in the query.  Return true (non-zero) if pExpr
///* is a constraint on pSrc only.
///*
///* This is an optimization.  False negatives will perhaps cause slower
///* queries, but false positives will yield incorrect answers.  So when in
///* doubt, return 0.
///*
///* To be an single-source constraint, the following must be true:
///*
///*   (1)  pExpr cannot refer to any table other than pSrc->iCursor.
///*
///*   (2a) pExpr cannot use subqueries unless the bAllowSubq parameter is
///*        true and the subquery is non-correlated
///*
///*   (2b) pExpr cannot use non-deterministic functions.
///*
///*   (3)  pSrc cannot be part of the left operand for a RIGHT JOIN.
///*        (Is there some way to relax this constraint?)
///*
///*   (4)  If pSrc is the right operand of a LEFT JOIN, then...
///*         (4a)  pExpr must come from an ON clause..
///*         (4b)  and specifically the ON clause associated with the LEFT JOIN.
///*
///*   (5)  If pSrc is the right operand of a LEFT JOIN or the left
///*        operand of a RIGHT JOIN, then pExpr must be from the WHERE
///*        clause, not an ON clause.
///*
///*   (6) Either:
///*
///*       (6a) pExpr does not originate in an ON or USING clause, or
///*
///*       (6b) The ON or USING clause from which pExpr is derived is
///*            not to the left of a RIGHT JOIN (or FULL JOIN).
///*
///*       Without this restriction, accepting pExpr as a single-table
///*       constraint might move the the ON/USING filter expression
///*       from the left side of a RIGHT JOIN over to the right side,
///*       which leads to incorrect answers.  See also restriction (9)
///*       on push-down.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_is_single_table_constraint(p_expr: *mut Expr,
    p_src_list: &SrcList, i_src: i32, b_allow_subq: i32) -> i32 {
    unsafe {
        let p_src: *const SrcItem =
            unsafe {
                &*((*p_src_list).a.as_ptr() as
                                *const SrcItem).offset(i_src as isize)
            };
        if unsafe { (*p_src).fg.jointype } as i32 & 64 != 0 { return 0; }
        if unsafe { (*p_src).fg.jointype } as i32 & 8 != 0 {
            if !(unsafe { (*p_expr).flags } & 1 as u32 != 0 as u32) as i32 !=
                    0 {
                return 0;
            }
            if unsafe { (*p_expr).w.i_join } != unsafe { (*p_src).i_cursor } {
                return 0;
            }
        } else {
            if unsafe { (*p_expr).flags } & 1 as u32 != 0 as u32 { return 0; }
        }
        if unsafe { (*p_expr).flags } & (1 | 2) as u32 != 0 as u32 &&
                unsafe {
                                (*((*p_src_list).a.as_ptr() as
                                                    *const SrcItem).offset(0 as isize)).fg.jointype
                            } as i32 & 64 != 0 {
            let mut jj: i32 = 0;
            {
                jj = 0;
                '__b69: loop {
                    if !(jj < i_src) { break '__b69; }
                    '__c69: loop {
                        if unsafe { (*p_expr).w.i_join } ==
                                unsafe {
                                    (*((*p_src_list).a.as_ptr() as
                                                    *const SrcItem).offset(jj as isize)).i_cursor
                                } {
                            if unsafe {
                                                (*((*p_src_list).a.as_ptr() as
                                                                    *const SrcItem).offset(jj as isize)).fg.jointype
                                            } as i32 & 64 != 0 {
                                return 0;
                            }
                            break '__b69;
                        }
                        break '__c69;
                    }
                    { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                }
            }
        }

        /// Rules (1), (2a), and (2b) handled by the following:
        return sqlite3_expr_is_table_constant(p_expr,
                unsafe { (*p_src).i_cursor }, b_allow_subq);
    }
}

///* Return TRUE if the given expression is a constant which would be
///* unchanged by OP_Affinity with the affinity given in the second
///* argument.
///*
///* This routine is used to determine if the OP_Affinity operation
///* can be omitted.  When in doubt return FALSE.  A false negative
///* is harmless.  A false positive, however, can result in the wrong
///* answer.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_needs_no_affinity_change(mut p: *const Expr,
    aff: i8) -> i32 {
    let mut op: u8 = 0 as u8;
    let mut unary_minus: i32 = 0;
    if aff as i32 == 65 { return 1; }
    while unsafe { (*p).op } as i32 == 173 || unsafe { (*p).op } as i32 == 174
        {
        if unsafe { (*p).op } as i32 == 174 { unary_minus = 1; }
        p = unsafe { (*p).p_left } as *const Expr;
    }
    op = unsafe { (*p).op } as u8;
    if op as i32 == 176 { op = unsafe { (*p).op2 } as u8; }
    '__s71:
        {
        match op {
            156 => {
                { return (aff as i32 >= 67) as i32; }
                { return (aff as i32 >= 67) as i32; }
                {
                    return ((unary_minus == 0) as i32 != 0 && aff as i32 == 66)
                            as i32;
                }
                { return (unary_minus == 0) as i32 as i32; }
                {
                    { let _ = 0; };

                    /// p cannot be part of a CHECK constraint
                    return (aff as i32 >= 67 &&
                                (unsafe { (*p).i_column } as i32) < 0) as i32;
                }
                { return 0; }
            }
            154 => {
                { return (aff as i32 >= 67) as i32; }
                {
                    return ((unary_minus == 0) as i32 != 0 && aff as i32 == 66)
                            as i32;
                }
                { return (unary_minus == 0) as i32 as i32; }
                {
                    { let _ = 0; };

                    /// p cannot be part of a CHECK constraint
                    return (aff as i32 >= 67 &&
                                (unsafe { (*p).i_column } as i32) < 0) as i32;
                }
                { return 0; }
            }
            118 => {
                {
                    return ((unary_minus == 0) as i32 != 0 && aff as i32 == 66)
                            as i32;
                }
                { return (unary_minus == 0) as i32 as i32; }
                {
                    { let _ = 0; };

                    /// p cannot be part of a CHECK constraint
                    return (aff as i32 >= 67 &&
                                (unsafe { (*p).i_column } as i32) < 0) as i32;
                }
                { return 0; }
            }
            155 => {
                { return (unary_minus == 0) as i32 as i32; }
                {
                    { let _ = 0; };

                    /// p cannot be part of a CHECK constraint
                    return (aff as i32 >= 67 &&
                                (unsafe { (*p).i_column } as i32) < 0) as i32;
                }
                { return 0; }
            }
            168 => {
                {
                    { let _ = 0; };

                    /// p cannot be part of a CHECK constraint
                    return (aff as i32 >= 67 &&
                                (unsafe { (*p).i_column } as i32) < 0) as i32;
                }
                { return 0; }
            }
            _ => { { return 0; } }
        }
    }
}

///* Return TRUE if the given string is a row-id column name.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_is_rowid(z: *const i8) -> i32 {
    if unsafe {
                sqlite3_str_i_cmp(z,
                    c"_ROWID_".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return 1;
    }
    if unsafe {
                sqlite3_str_i_cmp(z,
                    c"ROWID".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return 1;
    }
    if unsafe {
                sqlite3_str_i_cmp(z, c"OID".as_ptr() as *mut i8 as *const i8)
            } == 0 {
        return 1;
    }
    return 0;
}

///* Return a pointer to a buffer containing a usable rowid alias for table
///* pTab. An alias is usable if there is not an explicit user-defined column 
///* of the same name.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rowid_alias(p_tab: *mut Table) -> *const i8 {
    let az_opt: [*const i8; 3] =
        [c"_ROWID_".as_ptr() as *const i8, c"ROWID".as_ptr() as *const i8,
                c"OID".as_ptr() as *const i8];
    let mut ii: i32 = 0;
    { let _ = 0; };
    {
        ii = 0;
        '__b72: loop {
            if !(ii <
                            (core::mem::size_of::<[*const i8; 3]>() as u64 /
                                    core::mem::size_of::<*const i8>() as u64) as i32) {
                break '__b72;
            }
            '__c72: loop {
                if unsafe { sqlite3_column_index(p_tab, az_opt[ii as usize]) }
                        < 0 {
                    return az_opt[ii as usize];
                }
                break '__c72;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return core::ptr::null();
}

///* pExpr is a comparison operator.  Return the type affinity that should
///* be applied to both operands prior to doing the comparison.
extern "C" fn comparison_affinity(p_expr_1: &Expr) -> i8 {
    unsafe {
        let mut aff: i8 = 0 as i8;
        { let _ = 0; };
        { let _ = 0; };
        aff = sqlite3_expr_affinity((*p_expr_1).p_left as *const Expr);
        if !((*p_expr_1).p_right).is_null() {
            aff =
                sqlite3_compare_affinity((*p_expr_1).p_right as *const Expr,
                    aff);
        } else if (*p_expr_1).flags & 4096 as u32 != 0 as u32 {
            aff =
                sqlite3_compare_affinity(unsafe {
                            (*(unsafe {
                                                (*unsafe { (*(*p_expr_1).x.p_select).p_e_list }).a.as_ptr()
                                            } as *mut ExprListItem).offset(0 as isize)).p_expr
                        } as *const Expr, aff);
        } else if aff as i32 == 0 { aff = 65 as i8; }
        return aff;
    }
}

///* pExpr is a comparison expression, eg. '=', '<', IN(...) etc.
///* idx_affinity is the affinity of an indexed column. Return true
///* if the index with affinity idx_affinity may be used to implement
///* the comparison in pExpr.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_index_affinity_ok(p_expr: *const Expr,
    idx_affinity: i8) -> i32 {
    let aff: i8 = comparison_affinity(unsafe { &*p_expr });
    if (aff as i32) < 66 { return 1; }
    if aff as i32 == 66 { return (idx_affinity as i32 == 66) as i32; }
    return (idx_affinity as i32 >= 67) as i32;
}

///* Make a guess at all the possible datatypes of the result that could
///* be returned by an expression.  Return a bitmask indicating the answer:
///*
///*     0x01         Numeric
///*     0x02         Text
///*     0x04         Blob
///*
///* If the expression must return NULL, then 0x00 is returned.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expr_data_type(mut p_expr: *const Expr) -> i32 {
    unsafe {
        while !(p_expr).is_null() {
            '__s74:
                {
                match unsafe { (*p_expr).op } {
                    114 => {
                        {
                            p_expr = unsafe { (*p_expr).p_left } as *const Expr;
                            break '__s74;
                        }
                        { p_expr = core::ptr::null(); break '__s74; }
                        { return 2; }
                        { return 4; }
                        { return 6; }
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    179 => {
                        {
                            p_expr = unsafe { (*p_expr).p_left } as *const Expr;
                            break '__s74;
                        }
                        { p_expr = core::ptr::null(); break '__s74; }
                        { return 2; }
                        { return 4; }
                        { return 6; }
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    173 => {
                        {
                            p_expr = unsafe { (*p_expr).p_left } as *const Expr;
                            break '__s74;
                        }
                        { p_expr = core::ptr::null(); break '__s74; }
                        { return 2; }
                        { return 4; }
                        { return 6; }
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    122 => {
                        { p_expr = core::ptr::null(); break '__s74; }
                        { return 2; }
                        { return 4; }
                        { return 6; }
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    118 => {
                        { return 2; }
                        { return 4; }
                        { return 6; }
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    155 => {
                        { return 4; }
                        { return 6; }
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    112 => {
                        { return 6; }
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    157 => {
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    169 => {
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    172 => {
                        { return 7; }
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    168 => {
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    170 => {
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    139 => {
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    36 => {
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    178 => {
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    177 => {
                        {
                            let aff: i32 = sqlite3_expr_affinity(p_expr) as i32;
                            if aff >= 67 { return 5; }
                            if aff == 66 { return 6; }
                            return 7;
                        }
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    158 => {
                        {
                            let mut res: i32 = 0;
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_expr).x.p_list } as *const ExprList;
                            { let _ = 0; };
                            { let _ = 0; };
                            {
                                ii = 1;
                                '__b75: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b75; }
                                    '__c75: loop {
                                        res |=
                                            sqlite3_expr_data_type(unsafe {
                                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                                    } as *const Expr);
                                        break '__c75;
                                    }
                                    ii += 2;
                                }
                            }
                            if unsafe { (*p_list).n_expr } % 2 != 0 {
                                res |=
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset((unsafe { (*p_list).n_expr } - 1)
                                                                as isize)).p_expr
                                            } as *const Expr);
                            }
                            return res;
                        }
                        { return 1; }
                    }
                    _ => { { return 1; } }
                }
            }
        }

        /// End of while(pExpr)
        return 0;
    }
}

///* Return TRUE if the two expressions have equivalent collating sequences.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_coll_seq_match(p_parse: *mut Parse,
    p_e1: *const Expr, p_e2: *const Expr) -> i32 {
    let p_coll1: *mut CollSeq = sqlite3_expr_nn_coll_seq(p_parse, p_e1);
    let p_coll2: *mut CollSeq = sqlite3_expr_nn_coll_seq(p_parse, p_e2);
    { let _ = 0; };
    return (p_coll1 == p_coll2) as i32;
}

///* Set the collating sequence for expression pExpr to be the collating
///* sequence named by pToken.   Return a pointer to a new Expr node that
///* implements the COLLATE operator.
///*
///* If a memory allocation error occurs, that fact is recorded in pParse->db
///* and the pExpr parameter is returned unchanged.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_add_collate_token(p_parse: &Parse,
    mut p_expr: *mut Expr, p_coll_name: *const Token, dequote: i32)
    -> *mut Expr {
    unsafe {
        if unsafe { (*p_coll_name).n } as u32 > 0 as u32 {
            let p_new: *mut Expr =
                sqlite3_expr_alloc((*p_parse).db, 114, p_coll_name, dequote);
            if !(p_new).is_null() {
                unsafe { (*p_new).p_left = p_expr };
                unsafe { (*p_new).flags |= (512 | 8192) as u32 };
                p_expr = p_new;
            }
        }
        return p_expr;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_add_collate_string(p_parse: *const Parse,
    p_expr: *mut Expr, z_c: *const i8) -> *mut Expr {
    let mut s: Token = unsafe { core::mem::zeroed() };
    { let _ = 0; };
    unsafe { sqlite3_token_init(&mut s, z_c as *mut i8) };
    return sqlite3_expr_add_collate_token(unsafe { &*p_parse }, p_expr,
            &raw mut s as *const Token, 0);
}

///* If the expression list pEList contains more than iLimit elements,
///* leave an error message in pParse.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_list_check_length(p_parse: *mut Parse,
    p_e_list: *mut ExprList, z_object: *const i8) -> () {
    let mx: i32 = unsafe { (*unsafe { (*p_parse).db }).a_limit[2 as usize] };
    if !(p_e_list).is_null() && unsafe { (*p_e_list).n_expr } > mx {
        unsafe {
            sqlite3_error_msg(p_parse,
                c"too many columns in %s".as_ptr() as *mut i8 as *const i8,
                z_object)
        };
    }
}

/// Expression p is a comparison operator.  Return a collation sequence
///* appropriate for the comparison operator.
///*
///* This is normally just a wrapper around sqlite3BinaryCompareCollSeq().
///* However, if the OP_Commuted flag is set, then the order of the operands
///* is reversed in the sqlite3BinaryCompareCollSeq() call so that the
///* correct collating sequence is found.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_compare_coll_seq(p_parse: *mut Parse, p: &Expr)
    -> *mut CollSeq {
    if (*p).flags & 1024 as u32 != 0 as u32 {
        return sqlite3_binary_compare_coll_seq(p_parse,
                (*p).p_right as *const Expr, (*p).p_left as *const Expr);
    } else {
        return sqlite3_binary_compare_coll_seq(p_parse,
                (*p).p_left as *const Expr, (*p).p_right as *const Expr);
    }
}

///* Return the maximum height of any expression tree referenced
///* by the select statement passed as an argument.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_expr_height(p: *const Select) -> i32 {
    let mut n_height: i32 = 0;
    height_of_select(p, &mut n_height);
    return n_height;
}

///* Set the error offset for an Expr node, if possible.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_set_error_offset(p_expr: *mut Expr,
    i_ofst: i32) -> () {
    unsafe {
        if p_expr == core::ptr::null_mut() { return; }
        if unsafe { (*p_expr).flags } & (2 | 1) as u32 != 0 as u32 { return; }
        unsafe { (*p_expr).w.i_ofst = i_ofst };
    }
}

static z_aff_1: [i8; 10] =
    [66 as i8, 0 as i8, 67 as i8, 0 as i8, 68 as i8, 0 as i8, 69 as i8,
            0 as i8, 70 as i8, 0 as i8];

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
    static sqlite3_ctype_map: [u8; 0];
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_select_delete(_: *mut Sqlite3, _: *mut Select)
    -> ();
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_atoi64(_: *const i8, _: *mut i64, _: i32, _: u8)
    -> i32;
    fn sqlite3_record_error_offset_of_expr(_: *mut Sqlite3, _: *const Expr)
    -> ();
    fn sqlite3_v_list_num_to_name(_: *mut VList, _: i32)
    -> *const i8;
    fn sqlite3_v_list_name_to_num(_: *mut VList, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_v_list_add(_: *mut Sqlite3, _: *mut VList, _: *const i8,
    _: i32, _: i32)
    -> *mut VList;
    fn sqlite3_rename_expr_unmap(_: *mut Parse, _: *mut Expr)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn sqlite3_id_list_delete(_: *mut Sqlite3, _: *mut IdList)
    -> ();
    fn sqlite3_select_new(_: *mut Parse, _: *mut ExprList, _: *mut SrcList,
    _: *mut Expr, _: *mut ExprList, _: *mut Expr, _: *mut ExprList, _: u32,
    _: *mut Expr)
    -> *mut Select;
    fn sqlite3_rename_token_map(_: *mut Parse, _: *const (), _: *const Token)
    -> *const ();
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
    fn sqlite3_value_from_expr(_: *mut Sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn sqlite3_affinity_type(_: *const i8, _: *mut Column)
    -> i8;
    fn sqlite3_find_function(_: *mut Sqlite3, _: *const i8, _: i32, _: u8,
    _: u8)
    -> *mut FuncDef;
    fn sqlite3_column_default(_: *mut Vdbe, _: *mut Table, _: i32, _: i32)
    -> ();
    fn sqlite3_dec_or_hex_to_i64(_: *const i8, _: *mut i64)
    -> i32;
    fn sqlite3_ato_f(z: *const i8, _: *mut f64)
    -> i32;
    fn sqlite3_hex_to_blob(_: *mut Sqlite3, z: *const i8, n: i32)
    -> *mut ();
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_find_coll_seq(_: *mut Sqlite3, enc: u8, _: *const i8, _: i32)
    -> *mut CollSeq;
    fn sqlite3_get_coll_seq(_: *mut Parse, _: u8, _: *mut CollSeq,
    _: *const i8)
    -> *mut CollSeq;
    fn sqlite3_check_coll_seq(_: *mut Parse, _: *mut CollSeq)
    -> i32;
    fn sqlite3_vtab_overload_function(_: *mut Sqlite3, _: *mut FuncDef,
    n_arg_1: i32, _: *mut Expr)
    -> *mut FuncDef;
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_code_verify_schema(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_key_info_alloc(_: *mut Sqlite3, _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_key_info_unref(_: *mut KeyInfo)
    -> ();
    fn sqlite3_may_abort(_: *mut Parse)
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
    fn sqlite3_is_binary(_: *const CollSeq)
    -> i32;
    fn sqlite3_expr_is_like_operator(_: *const Expr)
    -> i32;
    fn sqlite3_column_index(p_tab_1: *mut Table, z_col_1: *const i8)
    -> i32;
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
    fn sqlite3_function_search(_: i32, _: *const i8)
    -> *mut FuncDef;
    fn sqlite3_insert_builtin_funcs(_: *mut FuncDef, _: i32)
    -> ();
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
    fn sqlite3_error_with_msg(_: *mut Sqlite3, _: i32, _: *const i8, ...)
    -> ();
    fn sqlite3_error(_: *mut Sqlite3, _: i32)
    -> ();
    fn sqlite3_error_clear(_: *mut Sqlite3)
    -> ();
    fn sqlite3_system_error(_: *mut Sqlite3, _: i32)
    -> ();
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
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_set_text_encoding(db: *mut Sqlite3, _: u8)
    -> ();
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
    fn sqlite3_nested_parse(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_expire_prepared_statements(_: *mut Sqlite3, _: i32)
    -> ();
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
    fn sqlite3_alter_finish_add_column(_: *mut Parse, _: *mut Token)
    -> ();
    fn sqlite3_alter_begin_add_column(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_alter_drop_column(_: *mut Parse, _: *mut SrcList,
    _: *const Token)
    -> ();
    fn sqlite3_rename_token_remap(_: *mut Parse, p_to_1: *const (),
    p_from_1: *const ())
    -> ();
    fn sqlite3_rename_exprlist_unmap(_: *mut Parse, _: *mut ExprList)
    -> ();
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
    fn sqlite3_create_column_expr(_: *mut Sqlite3, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_record_error_byte_offset(_: *mut Sqlite3, _: *const i8)
    -> ();
    fn sqlite3_backup_restart(_: *mut Sqlite3Backup)
    -> ();
    fn sqlite3_backup_update(_: *mut Sqlite3Backup, _: Pgno, _: *const u8)
    -> ();
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
    fn sqlite3_journal_open(_: *mut Sqlite3Vfs, _: *const i8,
    _: *mut Sqlite3File, _: i32, _: i32)
    -> i32;
    fn sqlite3_journal_size(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_journal_is_in_memory(p: *mut Sqlite3File)
    -> i32;
    fn sqlite3_mem_journal_open(_: *mut Sqlite3File)
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

#[repr(C)]
#[derive(Copy, Clone)]
struct RecN0 {
    _opaque: [u8; 0],
}
