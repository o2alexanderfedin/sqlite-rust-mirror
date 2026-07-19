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
    BusyHandler, CollSeq, Column, Cte, CteUse, DbFixer, Expr, ExprList,
    ExprListItem, ExprListItemS0, FKey, FpDecode, FuncDef, FuncDefHash,
    FuncDestructor, IdList, IdListItem, Index, KeyInfo, LogEst, Module,
    NameContext, OnOrUsing, Parse, RowSet, SQLiteThread, Schema, Select,
    SelectDest, Sqlite3, Sqlite3Config, Sqlite3InitInfo, Sqlite3Str, SrcItem,
    SrcItemS0, SrcList, StrAccum, Subquery, Table, Token, Trigger,
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

#[repr(C)]
#[derive(Copy, Clone)]
struct WhereConst {
    p_parse: *mut Parse,
    p_oom_fault: *mut u8,
    n_const: i32,
    n_chng: i32,
    b_has_aff_blob: i32,
    m_exclude_on: u32,
    ap_expr: *mut *mut Expr,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CheckOnCtx {
    p_src: *mut SrcList,
    i_join: i32,
    b_func_arg: i32,
    p_parent: *mut CheckOnCtx,
}

///* Return a pointer to the right-most SELECT statement in a compound.
extern "C" fn find_rightmost(mut p: *mut Select) -> *mut Select {
    while !(unsafe { (*p).p_next }).is_null() { p = unsafe { (*p).p_next }; }
    return p;
}

///* If the SELECT passed as the second argument has an associated WITH
///* clause, pop it from the stack stored as part of the Parse object.
///*
///* This function is used as the xSelectCallback2() callback by
///* sqlite3SelectExpand() when walking a SELECT tree to resolve table
///* names and other FROM clause elements.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_pop_with(p_walker: *mut Walker,
    p: *mut Select) -> () {
    let p_parse: *mut Parse = unsafe { (*p_walker).p_parse };
    if !(unsafe { (*p_parse).p_with }).is_null() &&
            unsafe { (*p).p_prior } == core::ptr::null_mut() {
        let p_with: *const With =
            unsafe { (*find_rightmost(p)).p_with } as *const With;
        if p_with != core::ptr::null_mut() {
            { let _ = 0; };
            unsafe { (*p_parse).p_with = unsafe { (*p_with).p_outer } };
        }
    }
}

#[allow(unused_doc_comments)]
extern "C" fn column_type_impl(mut p_nc_1: *mut NameContext, p_expr_1: &Expr)
    -> *const i8 {
    unsafe {
        let mut z_type: *const i8 = core::ptr::null();
        let mut j: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        '__s1:
            {
            match (*p_expr_1).op {
                168 => {
                    {
                        /// The expression is a column. Locate the table the column is being
                        ///* extracted from in NameContext.pSrcList. This table may be real
                        ///* database table or a subquery.
                        let mut p_tab: *const Table = core::ptr::null();
                        /// Table structure column is extracted from
                        let mut p_s: *const Select = core::ptr::null();
                        /// Select the column is extracted from
                        let i_col: i32 = (*p_expr_1).i_column as i32;
                        while !(p_nc_1).is_null() && (p_tab).is_null() as i32 != 0 {
                            let p_tab_list: *const SrcList =
                                unsafe { (*p_nc_1).p_src_list } as *const SrcList;
                            {
                                j = 0;
                                '__b3: loop {
                                    if !(j < unsafe { (*p_tab_list).n_src } &&
                                                    unsafe {
                                                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                            *mut SrcItem).offset(j as isize)).i_cursor
                                                        } != (*p_expr_1).i_table) {
                                        break '__b3;
                                    }
                                    '__c3: loop { break '__c3; }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if j < unsafe { (*p_tab_list).n_src } {
                                p_tab =
                                    unsafe {
                                        (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                        *mut SrcItem).offset(j as isize)).p_s_tab
                                    };
                                if unsafe {
                                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                *mut SrcItem).offset(j as isize)).fg.is_subquery()
                                        } != 0 {
                                    p_s =
                                        unsafe {
                                            (*unsafe {
                                                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                                *mut SrcItem).offset(j as isize)).u4.p_subq
                                                        }).p_select
                                        };
                                } else { p_s = core::ptr::null_mut(); }
                            } else { p_nc_1 = unsafe { (*p_nc_1).p_next }; }
                        }
                        if p_tab == core::ptr::null_mut() {

                            /// At one time, code such as "SELECT new.x" within a trigger would
                            ///* cause this condition to run.  Since then, we have restructured how
                            ///* trigger code is generated and so this condition is no longer
                            ///* possible. However, it can still be true for statements like
                            ///* the following:
                            ///*
                            ///*   CREATE TABLE t1(col INTEGER);
                            ///*   SELECT (SELECT t1.col) FROM FROM t1;
                            ///*
                            ///* when columnType() is called on the expression "t1.col" in the
                            ///* sub-select. In this case, set the column type to NULL, even
                            ///* though it should really be "INTEGER".
                            ///*
                            ///* This is not a problem, as the column type of "t1.col" is never
                            ///* used. When columnType() is called on the expression
                            ///* "(SELECT t1.col)", the correct type is returned (see the TK_SELECT
                            ///* branch below.
                            break '__s1;
                        }
                        { let _ = 0; };
                        if !(p_s).is_null() {
                            if i_col < unsafe { (*unsafe { (*p_s).p_e_list }).n_expr }
                                    && ((0 == 0) as i32 != 0 || i_col >= 0) {
                                /// If iCol is less than zero, then the expression requests the
                                ///* rowid of the sub-select or view. This expression is legal (see
                                ///* test case misc2.2.2) - it always evaluates to NULL.
                                let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
                                let mut p: *mut Expr =
                                    unsafe {
                                        (*(unsafe { (*unsafe { (*p_s).p_e_list }).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i_col as isize)).p_expr
                                    };
                                s_nc.p_src_list = unsafe { (*p_s).p_src };
                                s_nc.p_next = p_nc_1;
                                s_nc.p_parse = unsafe { (*p_nc_1).p_parse };
                                z_type = column_type_impl(&mut s_nc, unsafe { &*p });
                            }
                        } else {

                            /// A real table or a CTE table
                            { let _ = 0; };
                            { let _ = 0; };
                            if i_col < 0 {
                                z_type = c"INTEGER".as_ptr() as *mut i8 as *const i8;
                            } else {
                                z_type =
                                    unsafe {
                                            sqlite3ColumnType(unsafe {
                                                    &mut *unsafe { (*p_tab).a_col.offset(i_col as isize) }
                                                }, core::ptr::null_mut())
                                        } as *const i8;
                            }
                        }
                        break '__s1;
                    }
                    {
                        /// The expression is a sub-select. Return the declaration type and
                        ///* origin info for the single column in the result set of the SELECT
                        ///* statement.
                        let mut s_nc_1: NameContext =
                            unsafe { core::mem::zeroed() };
                        let mut p_s_1: *const Select = core::ptr::null();
                        let mut p: *mut Expr = core::ptr::null_mut();
                        { let _ = 0; };
                        p_s_1 = (*p_expr_1).x.p_select;
                        p =
                            unsafe {
                                (*(unsafe { (*unsafe { (*p_s_1).p_e_list }).a.as_ptr() } as
                                                *mut ExprListItem).offset(0 as isize)).p_expr
                            };
                        s_nc_1.p_src_list = unsafe { (*p_s_1).p_src };
                        s_nc_1.p_next = p_nc_1;
                        s_nc_1.p_parse = unsafe { (*p_nc_1).p_parse };
                        z_type = column_type_impl(&mut s_nc_1, unsafe { &*p });
                        break '__s1;
                    }
                }
                139 => {
                    {
                        /// The expression is a sub-select. Return the declaration type and
                        ///* origin info for the single column in the result set of the SELECT
                        ///* statement.
                        let mut s_nc_1: NameContext =
                            unsafe { core::mem::zeroed() };
                        let mut p_s_1: *const Select = core::ptr::null();
                        let mut p: *mut Expr = core::ptr::null_mut();
                        { let _ = 0; };
                        p_s_1 = (*p_expr_1).x.p_select;
                        p =
                            unsafe {
                                (*(unsafe { (*unsafe { (*p_s_1).p_e_list }).a.as_ptr() } as
                                                *mut ExprListItem).offset(0 as isize)).p_expr
                            };
                        s_nc_1.p_src_list = unsafe { (*p_s_1).p_src };
                        s_nc_1.p_next = p_nc_1;
                        s_nc_1.p_parse = unsafe { (*p_nc_1).p_parse };
                        z_type = column_type_impl(&mut s_nc_1, unsafe { &*p });
                        break '__s1;
                    }
                }
                _ => {}
            }
        }
        return z_type;
    }
}

///* Generate code that will tell the VDBE the declaration types of columns
///* in the result set.
extern "C" fn generate_column_types(p_parse_1: *mut Parse,
    p_tab_list_1: *mut SrcList, p_e_list_1: &ExprList) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        s_nc.p_src_list = p_tab_list_1;
        s_nc.p_parse = p_parse_1;
        s_nc.p_next = core::ptr::null_mut();
        {
            i = 0;
            '__b4: loop {
                if !(i < (*p_e_list_1).n_expr) { break '__b4; }
                '__c4: loop {
                    let p: *mut Expr =
                        unsafe {
                            (*((*p_e_list_1).a.as_ptr() as
                                            *mut ExprListItem).offset(i as isize)).p_expr
                        };
                    let mut z_type: *const i8 = core::ptr::null();
                    z_type = column_type_impl(&mut s_nc, unsafe { &*p });
                    unsafe {
                        sqlite3_vdbe_set_col_name(v, i, 1, z_type,
                            Some(unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut ())
                                                -> ()>(-1 as isize as *const ())
                                }))
                    };
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

///* Compute the column names for a SELECT statement.
///*
///* The only guarantee that SQLite makes about column names is that if the
///* column has an AS clause assigning it a name, that will be the name used.
///* That is the only documented guarantee.  However, countless applications
///* developed over the years have made baseless assumptions about column names
///* and will break if those assumptions changes.  Hence, use extreme caution
///* when modifying this routine to avoid breaking legacy.
///*
///* See Also: sqlite3ColumnsFromExprList()
///*
///* The PRAGMA short_column_names and PRAGMA full_column_names settings are
///* deprecated.  The default setting is short=ON, full=OFF.  99.9% of all
///* applications should operate this way.  Nevertheless, we need to support the
///* other modes for legacy:
///*
///*    short=OFF, full=OFF:      Column name is the text of the expression has it
///*                              originally appears in the SELECT statement.  In
///*                              other words, the zSpan of the result expression.
///*
///*    short=ON, full=OFF:       (This is the default setting).  If the result
///*                              refers directly to a table column, then the
///*                              result column name is just the table column
///*                              name: COLUMN.  Otherwise use zSpan.
///*
///*    full=ON, short=ANY:       If the result refers directly to a table column,
///*                              then the result column name with the table name
///*                              prefix, ex: TABLE.COLUMN.  Otherwise use zSpan.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_generate_column_names(p_parse: *mut Parse,
    mut p_select: *mut Select) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
        let mut i: i32 = 0;
        let mut p_tab: *const Table = core::ptr::null();
        let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        let mut full_name: i32 = 0;
        /// TABLE.COLUMN if no AS clause and is a direct table ref
        let mut src_name: i32 = 0;
        if unsafe { (*p_parse).col_names_set() } != 0 { return; }
        while !(unsafe { (*p_select).p_prior }).is_null() {
            p_select = unsafe { (*p_select).p_prior };
        }
        p_tab_list = unsafe { (*p_select).p_src };
        p_e_list = unsafe { (*p_select).p_e_list };
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_parse).set_col_names_set(1 as Bft as u32) };
        full_name = (unsafe { (*db).flags } & 4 as u64 != 0 as u64) as i32;
        src_name =
            (unsafe { (*db).flags } & 64 as u64 != 0 as u64 || full_name != 0)
                as i32;
        unsafe {
            sqlite3_vdbe_set_num_cols(v, unsafe { (*p_e_list).n_expr })
        };
        {
            i = 0;
            '__b6: loop {
                if !(i < unsafe { (*p_e_list).n_expr }) { break '__b6; }
                '__c6: loop {
                    let p: *const Expr =
                        unsafe {
                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)).p_expr
                            } as *const Expr;
                    { let _ = 0; };
                    { let _ = 0; };

                    /// Agg processing has not run yet
                    { let _ = 0; };
                    if !(unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).z_e_name
                                        }).is_null() &&
                            unsafe {
                                        (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).fg.e_e_name()
                                    } as i32 == 0 {
                        /// An AS clause always takes first priority
                        let z_name: *const i8 =
                            unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).z_e_name
                                } as *const i8;
                        unsafe {
                            sqlite3_vdbe_set_col_name(v, i, 0, z_name as *const i8,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                    } else if src_name != 0 && unsafe { (*p).op } as i32 == 168
                        {
                        let mut z_col: *mut i8 = core::ptr::null_mut();
                        let mut i_col: i32 = unsafe { (*p).i_column } as i32;
                        p_tab = unsafe { (*p).y.p_tab };
                        { let _ = 0; };
                        if i_col < 0 { i_col = unsafe { (*p_tab).i_p_key } as i32; }
                        { let _ = 0; };
                        if i_col < 0 {
                            z_col = c"rowid".as_ptr() as *mut i8;
                        } else {
                            z_col =
                                unsafe {
                                    (*unsafe {
                                                (*p_tab).a_col.offset(i_col as isize)
                                            }).z_cn_name
                                };
                        }
                        if full_name != 0 {
                            let mut z_name_1: *const i8 = core::ptr::null();
                            z_name_1 =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"%s.%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab).z_name }, z_col)
                                };
                            unsafe {
                                sqlite3_vdbe_set_col_name(v, i, 0, z_name_1 as *const i8,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(sqlite3_row_set_clear as *const ())
                                        }))
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_set_col_name(v, i, 0, z_col as *const i8,
                                    Some(unsafe {
                                            core::mem::transmute::<*const (),
                                                    unsafe extern "C" fn(*mut ())
                                                        -> ()>(-1 as isize as *const ())
                                        }))
                            };
                        }
                    } else {
                        let mut z: *const i8 =
                            unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).z_e_name
                                } as *const i8;
                        z =
                            if z == core::ptr::null() {
                                    unsafe {
                                        sqlite3_m_printf(db,
                                            c"column%d".as_ptr() as *mut i8 as *const i8, i + 1)
                                    }
                                } else { unsafe { sqlite3_db_str_dup(db, z) } } as
                                *const i8;
                        unsafe {
                            sqlite3_vdbe_set_col_name(v, i, 0, z,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(sqlite3_row_set_clear as *const ())
                                    }))
                        };
                    }
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        generate_column_types(p_parse, p_tab_list, unsafe { &*p_e_list });
    }
}

///* Given an expression list (which is really the list of expressions
///* that form the result set of a SELECT statement) compute appropriate
///* column names for a table that would hold the expression list.
///*
///* All column names will be unique.
///*
///* Only the column names are computed.  Column.zType, Column.zColl,
///* and other fields of Column are zeroed.
///*
///* Return SQLITE_OK on success.  If a memory allocation error occurs,
///* store NULL in *paCol and 0 in *pnCol and return SQLITE_NOMEM.
///*
///* The only guarantee that SQLite makes about column names is that if the
///* column has an AS clause assigning it a name, that will be the name used.
///* That is the only documented guarantee.  However, countless applications
///* developed over the years have made baseless assumptions about column names
///* and will break if those assumptions changes.  Hence, use extreme caution
///* when modifying this routine to avoid breaking legacy.
///*
///* See Also: sqlite3GenerateColumnNames()
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_columns_from_expr_list(p_parse: *mut Parse,
    p_e_list: *mut ExprList, pn_col: &mut i16, pa_col: &mut *mut Column)
    -> i32 {
    unsafe {
        unsafe {
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            /// Database connection
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            /// Loop counters
            let mut cnt: u32 = 0 as u32;
            /// Index added to make the name unique
            let mut a_col: *mut Column = core::ptr::null_mut();
            let mut p_col: *mut Column = core::ptr::null_mut();
            /// For looping over result columns
            let mut n_col: i32 = 0;
            /// Number of columns in the result set
            let mut z_name: *mut i8 = core::ptr::null_mut();
            /// Column name
            let mut n_name: i32 = 0;
            /// Size of name in zName[]
            let mut ht: Hash = unsafe { core::mem::zeroed() };
            /// Hash table of column names
            let mut p_tab: *const Table = core::ptr::null();
            unsafe { sqlite3_hash_init(&mut ht) };
            if !(p_e_list).is_null() {
                n_col = unsafe { (*p_e_list).n_expr };
                a_col =
                    unsafe {
                            sqlite3_db_malloc_zero(db,
                                core::mem::size_of::<Column>() as u64 * n_col as u64)
                        } as *mut Column;
                if n_col > 32767 { n_col = 32767; }
            } else { n_col = 0; a_col = core::ptr::null_mut(); }
            { let _ = 0; };
            *pn_col = n_col as i16;
            *pa_col = a_col;
            {
                { i = 0; p_col = a_col };
                '__b7: loop {
                    if !(i < n_col &&
                                    (unsafe { (*p_parse).n_err } == 0) as i32 != 0) {
                        break '__b7;
                    }
                    '__c7: loop {
                        let p_x: *mut ExprListItem =
                            unsafe {
                                &mut *(unsafe { (*p_e_list).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)
                            };
                        let mut p_collide: *const ExprListItem = core::ptr::null();
                        if { z_name = unsafe { (*p_x).z_e_name }; z_name } !=
                                    core::ptr::null_mut() &&
                                unsafe { (*p_x).fg.e_e_name() } as i32 == 0
                            {} else {
                            let mut p_col_expr: *const Expr =
                                unsafe {
                                        sqlite3_expr_skip_collate_and_likely(unsafe {
                                                (*p_x).p_expr
                                            })
                                    } as *const Expr;
                            while p_col_expr != core::ptr::null_mut() &&
                                    unsafe { (*p_col_expr).op } as i32 == 142 {
                                p_col_expr = unsafe { (*p_col_expr).p_right };
                                { let _ = 0; };
                            }
                            if unsafe { (*p_col_expr).op } as i32 == 168 &&
                                        unsafe { (*p_col_expr).flags } &
                                                (16777216 | 33554432) as u32 == 0 as u32 &&
                                    unsafe { (*p_col_expr).y.p_tab } != core::ptr::null_mut() {
                                /// For columns use the column name name
                                let mut i_col: i32 =
                                    unsafe { (*p_col_expr).i_column } as i32;
                                p_tab = unsafe { (*p_col_expr).y.p_tab };
                                if i_col < 0 { i_col = unsafe { (*p_tab).i_p_key } as i32; }
                                z_name =
                                    if i_col >= 0 {
                                        unsafe {
                                            (*unsafe {
                                                        (*p_tab).a_col.offset(i_col as isize)
                                                    }).z_cn_name
                                        }
                                    } else { c"rowid".as_ptr() as *mut i8 };
                            } else if unsafe { (*p_col_expr).op } as i32 == 60 {
                                { let _ = 0; };
                                z_name = unsafe { (*p_col_expr).u.z_token };
                            } else {

                                /// Use the original text of the column expression as its name
                                { let _ = 0; };
                            }
                        }
                        if !(z_name).is_null() &&
                                (unsafe { sqlite3_is_true_or_false(z_name as *const i8) } ==
                                            0) as i32 != 0 {
                            z_name =
                                unsafe { sqlite3_db_str_dup(db, z_name as *const i8) };
                        } else {
                            z_name =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"column%d".as_ptr() as *mut i8 as *const i8, i + 1)
                                };
                        }

                        /// Make sure the column name is unique.  If the name is not unique,
                        ///* append an integer to the name so that it becomes unique.
                        (cnt = 0 as u32);
                        while !(z_name).is_null() &&
                                {
                                        p_collide =
                                            unsafe {
                                                    sqlite3_hash_find(&raw mut ht as *const Hash,
                                                        z_name as *const i8)
                                                } as *mut ExprListItem;
                                        p_collide
                                    } != core::ptr::null_mut() {
                            if unsafe { (*p_collide).fg.b_using_term() } != 0 {
                                unsafe { (*p_col).col_flags |= 1024 as u16 };
                            }
                            n_name = unsafe { sqlite3_strlen30(z_name as *const i8) };
                            if n_name > 0 {
                                {
                                    j = n_name - 1;
                                    '__b10: loop {
                                        if !(j > 0 &&
                                                        unsafe {
                                                                        *(sqlite3_ctype_map.as_ptr() as
                                                                                    *const u8).add(unsafe { *z_name.offset(j as isize) } as u8
                                                                                    as usize)
                                                                    } as i32 & 4 != 0) {
                                            break '__b10;
                                        }
                                        '__c10: loop { break '__c10; }
                                        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                    }
                                }
                                if unsafe { *z_name.offset(j as isize) } as i32 ==
                                        ':' as i32 {
                                    n_name = j;
                                }
                            }
                            z_name =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"%.*z:%u".as_ptr() as *mut i8 as *const i8, n_name, z_name,
                                        { let __p = &mut cnt; *__p += 1; *__p })
                                };
                            unsafe { sqlite3_progress_check(p_parse) };
                            if cnt > 3 as u32 {
                                unsafe {
                                    sqlite3_randomness(core::mem::size_of::<u32>() as i32,
                                        &raw mut cnt as *mut ())
                                };
                            }
                        }
                        unsafe { (*p_col).z_cn_name = z_name };
                        unsafe {
                            (*p_col).h_name =
                                unsafe { sqlite3_str_i_hash(z_name as *const i8) }
                        };
                        if unsafe { (*p_x).fg.b_no_expand() } != 0 {
                            unsafe { (*p_col).col_flags |= 1024 as u16 };
                        }
                        if !(z_name).is_null() &&
                                unsafe {
                                        sqlite3_hash_insert(&mut ht, z_name as *const i8,
                                            p_x as *mut ())
                                    } == p_x as *mut () {
                            unsafe { sqlite3_oom_fault(db) };
                        }
                        break '__c7;
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
            unsafe { sqlite3_hash_clear(&mut ht) };
            if unsafe { (*p_parse).n_err } != 0 {
                {
                    j = 0;
                    '__b11: loop {
                        if !(j < i) { break '__b11; }
                        '__c11: loop {
                            unsafe {
                                sqlite3_db_free(db,
                                    unsafe { (*a_col.offset(j as isize)).z_cn_name } as *mut ())
                            };
                            break '__c11;
                        }
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { sqlite3_db_free(db, a_col as *mut ()) };
                *pa_col = core::ptr::null_mut();
                *pn_col = 0 as i16;
                return unsafe { (*p_parse).rc };
            }
            return 0;
        }
    }
}

///* pTab is a transient Table object that represents a subquery of some
///* kind (maybe a parenthesized subquery in the FROM clause of a larger
///* query, or a VIEW, or a CTE).  This routine computes type information
///* for that Table object based on the Select object that implements the
///* subquery.  For the purposes of this routine, "type information" means:
///*
///*    *   The datatype name, as it might appear in a CREATE TABLE statement
///*    *   Which collating sequence to use for the column
///*    *   The affinity of the column
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_subquery_column_types(p_parse: *mut Parse,
    p_tab: &mut Table, mut p_select: *mut Select, aff: i8) -> () {
    unsafe {
        unsafe {
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            let mut p_col: *mut Column = core::ptr::null_mut();
            let mut p_coll: *const CollSeq = core::ptr::null();
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut p: *mut Expr = core::ptr::null_mut();
            let mut a: *const ExprListItem = core::ptr::null();
            let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*db).malloc_failed } != 0 ||
                    unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                return;
            }
            while !(unsafe { (*p_select).p_prior }).is_null() {
                p_select = unsafe { (*p_select).p_prior };
            }
            a =
                unsafe { (*unsafe { (*p_select).p_e_list }).a.as_ptr() } as
                    *mut ExprListItem;
            unsafe {
                memset(&raw mut s_nc as *mut (), 0,
                    core::mem::size_of::<NameContext>() as u64)
            };
            s_nc.p_src_list = unsafe { (*p_select).p_src };
            {
                { i = 0; p_col = (*p_tab).a_col };
                '__b13: loop {
                    if !(i < (*p_tab).n_col as i32) { break '__b13; }
                    '__c13: loop {
                        let mut z_type: *const i8 = core::ptr::null();
                        let mut n: i64 = 0 as i64;
                        let mut m: i32 = 0;
                        let mut p_s2: *mut Select = p_select;
                        (*p_tab).tab_flags |=
                            (unsafe { (*p_col).col_flags } as i32 & 98) as u32;
                        p = unsafe { (*a.offset(i as isize)).p_expr };

                        /// pCol->szEst = ... // Column size est for SELECT tables never used
                        unsafe {
                            (*p_col).affinity =
                                unsafe { sqlite3_expr_affinity(p as *const Expr) }
                        };
                        while unsafe { (*p_col).affinity } as i32 <= 64 &&
                                unsafe { (*p_s2).p_next } != core::ptr::null_mut() {
                            m |=
                                unsafe {
                                    sqlite3_expr_data_type(unsafe {
                                                (*(unsafe { (*unsafe { (*p_s2).p_e_list }).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                            } as *const Expr)
                                };
                            p_s2 = unsafe { (*p_s2).p_next };
                            unsafe {
                                (*p_col).affinity =
                                    unsafe {
                                        sqlite3_expr_affinity(unsafe {
                                                    (*(unsafe { (*unsafe { (*p_s2).p_e_list }).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i as isize)).p_expr
                                                } as *const Expr)
                                    }
                            };
                        }
                        if unsafe { (*p_col).affinity } as i32 <= 64 {
                            unsafe { (*p_col).affinity = aff };
                        }
                        if unsafe { (*p_col).affinity } as i32 >= 66 &&
                                (!(unsafe { (*p_s2).p_next }).is_null() || p_s2 != p_select)
                            {
                            {
                                p_s2 = unsafe { (*p_s2).p_next };
                                '__b15: loop {
                                    if !(!(p_s2).is_null()) { break '__b15; }
                                    '__c15: loop {
                                        m |=
                                            unsafe {
                                                sqlite3_expr_data_type(unsafe {
                                                            (*(unsafe { (*unsafe { (*p_s2).p_e_list }).a.as_ptr() } as
                                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                                        } as *const Expr)
                                            };
                                        break '__c15;
                                    }
                                    p_s2 = unsafe { (*p_s2).p_next };
                                }
                            }
                            if unsafe { (*p_col).affinity } as i32 == 66 && m & 1 != 0 {
                                unsafe { (*p_col).affinity = 65 as i8 };
                            } else if unsafe { (*p_col).affinity } as i32 >= 67 &&
                                    m & 2 != 0 {
                                unsafe { (*p_col).affinity = 65 as i8 };
                            }
                            if unsafe { (*p_col).affinity } as i32 >= 67 &&
                                    unsafe { (*p).op } as i32 == 36 {
                                unsafe { (*p_col).affinity = 70 as i8 };
                            }
                        }
                        z_type = column_type_impl(&mut s_nc, unsafe { &*p });
                        if z_type == core::ptr::null() ||
                                unsafe { (*p_col).affinity } as i32 !=
                                    unsafe {
                                            sqlite3_affinity_type(z_type, core::ptr::null_mut())
                                        } as i32 {
                            if unsafe { (*p_col).affinity } as i32 == 67 ||
                                    unsafe { (*p_col).affinity } as i32 == 70 {
                                z_type = c"NUM".as_ptr() as *mut i8 as *const i8;
                            } else {
                                z_type = core::ptr::null();
                                {
                                    j = 1;
                                    '__b16: loop {
                                        if !(j < 6) { break '__b16; }
                                        '__c16: loop {
                                            if unsafe {
                                                            *(sqlite3_std_type_affinity.as_ptr() as
                                                                        *const i8).offset(j as isize)
                                                        } as i32 == unsafe { (*p_col).affinity } as i32 {
                                                z_type =
                                                    unsafe {
                                                        *(sqlite3_std_type.as_ptr() as
                                                                    *mut *const i8).offset(j as isize)
                                                    };
                                                break '__b16;
                                            }
                                            break '__c16;
                                        }
                                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            }
                        }
                        if !(z_type).is_null() {
                            let k: i64 = unsafe { strlen(z_type) } as i64;
                            n =
                                unsafe {
                                        strlen(unsafe { (*p_col).z_cn_name } as *const i8)
                                    } as i64;
                            unsafe {
                                (*p_col).z_cn_name =
                                    unsafe {
                                            sqlite3_db_realloc_or_free(db,
                                                unsafe { (*p_col).z_cn_name } as *mut (),
                                                (n + k as i64 + 2 as i64) as u64)
                                        } as *mut i8
                            };
                            unsafe { (*p_col).col_flags &= !(4 | 512) as u16 };
                            if !(unsafe { (*p_col).z_cn_name }).is_null() {
                                unsafe {
                                    memcpy(unsafe {
                                                &raw mut *unsafe {
                                                            (*p_col).z_cn_name.offset((n + 1 as i64) as isize)
                                                        }
                                            } as *mut (), z_type as *const (), (k + 1 as i64) as u64)
                                };
                                unsafe { (*p_col).col_flags |= 4 as u16 };
                            }
                        }
                        p_coll =
                            unsafe { sqlite3_expr_coll_seq(p_parse, p as *const Expr) };
                        if !(p_coll).is_null() {
                            { let _ = 0; };
                            unsafe {
                                sqlite3_column_set_coll(db, p_col,
                                    unsafe { (*p_coll).z_name } as *const i8)
                            };
                        }
                        break '__c13;
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
            (*p_tab).sz_tab_row = 1 as LogEst;
        }
    }
}

///* Detect compound SELECT statements that use an ORDER BY clause with
///* an alternative collating sequence.
///*
///*    SELECT ... FROM t1 EXCEPT SELECT ... FROM t2 ORDER BY .. COLLATE ...
///*
///* These are rewritten as a subquery:
///*
///*    SELECT * FROM (SELECT ... FROM t1 EXCEPT SELECT ... FROM t2)
///*     ORDER BY ... COLLATE ...
///*
///* This transformation is necessary because the multiSelectByMerge() routine
///* above that generates the code for a compound SELECT with an ORDER BY clause
///* uses a merge algorithm that requires the same collating sequence on the
///* result columns as on the ORDER BY clause.  See ticket
///* http://sqlite.org/src/info/6709574d2a
///*
///* This transformation is only needed for EXCEPT, INTERSECT, and UNION.
///* The UNION ALL operator works fine with multiSelectByMerge() even when
///* there are COLLATE terms in the ORDER BY.
#[allow(unused_doc_comments)]
extern "C" fn convert_compound_select_to_subquery(p_walker_1: *mut Walker,
    p: *mut Select) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut p_new: *mut Select = core::ptr::null_mut();
        let mut p_x: *const Select = core::ptr::null();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut a: *const ExprListItem = core::ptr::null();
        let mut p_new_src: *mut SrcList = core::ptr::null_mut();
        let mut p_parse: *mut Parse = core::ptr::null_mut();
        let mut dummy: Token = unsafe { core::mem::zeroed() };
        if unsafe { (*p).p_prior } == core::ptr::null_mut() { return 0; }
        if unsafe { (*p).p_order_by } == core::ptr::null_mut() { return 0; }
        {
            p_x = p;
            '__b17: loop {
                if !(!(p_x).is_null() &&
                                (unsafe { (*p_x).op } as i32 == 136 ||
                                    unsafe { (*p_x).op } as i32 == 139)) {
                    break '__b17;
                }
                '__c17: loop { break '__c17; }
                p_x = unsafe { (*p_x).p_prior };
            }
        }
        if p_x == core::ptr::null_mut() { return 0; }
        a =
            unsafe { (*unsafe { (*p).p_order_by }).a.as_ptr() } as
                *mut ExprListItem;
        if unsafe { (*a.offset(0 as isize)).u.x.i_order_by_col } != 0 {
            return 0;
        }
        {
            i = unsafe { (*unsafe { (*p).p_order_by }).n_expr } - 1;
            '__b18: loop {
                if !(i >= 0) { break '__b18; }
                '__c18: loop {
                    if unsafe {
                                    (*unsafe { (*a.offset(i as isize)).p_expr }).flags
                                } & 512 as u32 != 0 {
                        break '__b18;
                    }
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        if i < 0 { return 0; }

        /// If we reach this point, that means the transformation is required.
        (p_parse = unsafe { (*p_walker_1).p_parse });
        db = unsafe { (*p_parse).db };
        p_new =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<Select>() as u64)
                } as *mut Select;
        if p_new == core::ptr::null_mut() { return 2; }
        unsafe {
            memset(&raw mut dummy as *mut (), 0,
                core::mem::size_of::<Token>() as u64)
        };
        p_new_src =
            unsafe {
                sqlite3_src_list_append_from_term(p_parse,
                    core::ptr::null_mut(), core::ptr::null_mut(),
                    core::ptr::null_mut(), &mut dummy, p_new,
                    core::ptr::null_mut())
            };
        { let _ = 0; };
        if unsafe { (*p_parse).n_err } != 0 {
            unsafe { sqlite3_src_list_delete(db, p_new_src) };
            return 2;
        }
        unsafe { *p_new = unsafe { core::ptr::read(p) } };
        unsafe { (*p).p_src = p_new_src };
        unsafe {
            (*p).p_e_list =
                unsafe {
                    sqlite3_expr_list_append(p_parse, core::ptr::null_mut(),
                        unsafe { sqlite3_expr(db, 180, core::ptr::null()) })
                }
        };
        unsafe { (*p).op = 139 as u8 };
        unsafe { (*p).p_where = core::ptr::null_mut() };
        unsafe { (*p_new).p_group_by = core::ptr::null_mut() };
        unsafe { (*p_new).p_having = core::ptr::null_mut() };
        unsafe { (*p_new).p_order_by = core::ptr::null_mut() };
        unsafe { (*p).p_prior = core::ptr::null_mut() };
        unsafe { (*p).p_next = core::ptr::null_mut() };
        unsafe { (*p).p_with = core::ptr::null_mut() };
        unsafe { (*p).p_win_defn = core::ptr::null_mut() };
        unsafe { (*p).sel_flags &= !(256 as u32) };
        { let _ = 0; };
        unsafe { (*p).sel_flags |= 65536 as u32 };
        { let _ = 0; };
        unsafe { (*unsafe { (*p_new).p_prior }).p_next = p_new };
        unsafe { (*p_new).p_limit = core::ptr::null_mut() };
        return 0;
    }
}

/// The code generator maintains a stack of active WITH clauses
///* with the inner-most WITH clause being at the top of the stack.
///*
///* This routine pushes the WITH clause passed as the second argument
///* onto the top of the stack. If argument bFree is true, then this
///* WITH clause will never be popped from the stack but should instead
///* be freed along with the Parse object. In other cases, when
///* bFree==0, the With object will be freed along with the SELECT
///* statement with which it is associated.
///*
///* This routine returns a copy of pWith.  Or, if bFree is true and
///* the pWith object is destroyed immediately due to an OOM condition,
///* then this routine return NULL.
///*
///* If bFree is true, do not continue to use the pWith pointer after
///* calling this routine,  Instead, use only the return value.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_with_push(p_parse: *mut Parse,
    mut p_with: *mut With, b_free: u8) -> *mut With {
    if !(p_with).is_null() {
        if b_free != 0 {
            p_with =
                unsafe {
                        sqlite3_parser_add_cleanup(p_parse,
                            Some(sqlite3_with_delete_generic), p_with as *mut ())
                    } as *mut With;
            if p_with == core::ptr::null_mut() {
                return core::ptr::null_mut();
            }
        }
        if unsafe { (*p_parse).n_err } == 0 {
            { let _ = 0; };
            unsafe { (*p_with).p_outer = unsafe { (*p_parse).p_with } };
            unsafe { (*p_parse).p_with = p_with };
        }
    }
    return p_with;
}

///* The SrcItem structure passed as the second argument represents a
///* sub-query in the FROM clause of a SELECT statement. This function
///* allocates and populates the SrcItem.pTab object. If successful,
///* SQLITE_OK is returned. Otherwise, if an OOM error is encountered,
///* SQLITE_NOMEM.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_expand_subquery(p_parse: *mut Parse,
    p_from: *mut SrcItem) -> i32 {
    unsafe {
        let mut p_sel: *const Select = core::ptr::null();
        let mut p_tab: *mut Table = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        p_sel = unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select };
        { let _ = 0; };
        unsafe {
            (*p_from).p_s_tab =
                {
                    p_tab =
                        unsafe {
                                sqlite3_db_malloc_zero(unsafe { (*p_parse).db },
                                    core::mem::size_of::<Table>() as u64)
                            } as *mut Table;
                    p_tab
                }
        };
        if p_tab == core::ptr::null_mut() { return 7; }
        unsafe { (*p_tab).n_tab_ref = 1 as u32 };
        if !(unsafe { (*p_from).z_alias }).is_null() {
            unsafe {
                (*p_tab).z_name =
                    unsafe {
                        sqlite3_db_str_dup(unsafe { (*p_parse).db },
                            unsafe { (*p_from).z_alias } as *const i8)
                    }
            };
        } else {
            unsafe {
                (*p_tab).z_name =
                    unsafe {
                        sqlite3_m_printf(unsafe { (*p_parse).db },
                            c"%!S".as_ptr() as *mut i8 as *const i8, p_from)
                    }
            };
        }
        while !(unsafe { (*p_sel).p_prior }).is_null() {
            p_sel = unsafe { (*p_sel).p_prior };
        }
        sqlite3_columns_from_expr_list(p_parse, unsafe { (*p_sel).p_e_list },
            unsafe { &mut (*p_tab).n_col }, unsafe { &mut (*p_tab).a_col });
        unsafe { (*p_tab).i_p_key = -1 as i16 };
        unsafe { (*p_tab).e_tab_type = 2 as u8 };
        unsafe { (*p_tab).n_row_log_est = 200 as LogEst };
        { let _ = 0; };

        /// The usual case - do not allow ROWID on a subquery
        unsafe { (*p_tab).tab_flags |= (16384 | 512) as u32 };
        return if unsafe { (*p_parse).n_err } != 0 { 1 } else { 0 };
    }
}

///* Argument pWith (which may be NULL) points to a linked list of nested
///* WITH contexts, from inner to outermost. If the table identified by
///* FROM clause element pItem is really a common-table-expression (CTE)
///* then return a pointer to the CTE definition for that table. Otherwise
///* return NULL.
///*
///* If a non-NULL value is returned, set *ppContext to point to the With
///* object that the returned CTE belongs to.
extern "C" fn search_with(p_with_1: *mut With, p_item_1: &SrcItem,
    pp_context_1: &mut *mut With) -> *mut Cte {
    let z_name: *const i8 = (*p_item_1).z_name as *const i8;
    let mut p: *mut With = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    {
        p = p_with_1;
        '__b20: loop {
            if !(!(p).is_null()) { break '__b20; }
            '__c20: loop {
                let mut i: i32 = 0;
                {
                    i = 0;
                    '__b21: loop {
                        if !(i < unsafe { (*p).n_cte }) { break '__b21; }
                        '__c21: loop {
                            if unsafe {
                                        sqlite3_str_i_cmp(z_name,
                                            unsafe {
                                                    (*(unsafe { (*p).a.as_ptr() } as
                                                                    *mut Cte).offset(i as isize)).z_name
                                                } as *const i8)
                                    } == 0 {
                                *pp_context_1 = p;
                                return unsafe {
                                        &mut *(unsafe { (*p).a.as_ptr() } as
                                                        *mut Cte).offset(i as isize)
                                    };
                            }
                            break '__c21;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if unsafe { (*p).b_view } != 0 { break '__b20; }
                break '__c20;
            }
            p = unsafe { (*p).p_outer };
        }
    }
    return core::ptr::null_mut();
}

///* Check to see if the FROM clause term pFrom has table-valued function
///* arguments.  If it does, leave an error message in pParse and return
///* non-zero, since pFrom is not allowed to be a table-valued function.
extern "C" fn cannot_be_function(p_parse_1: *mut Parse, p_from_1: &SrcItem)
    -> i32 {
    if (*p_from_1).fg.is_tab_func() != 0 {
        unsafe {
            sqlite3_error_msg(p_parse_1,
                c"\'%s\' is not a function".as_ptr() as *mut i8 as *const i8,
                (*p_from_1).z_name)
        };
        return 1;
    }
    return 0;
}

///* This function checks if argument pFrom refers to a CTE declared by
///* a WITH clause on the stack currently maintained by the parser (on the
///* pParse->pWith linked list).  And if currently processing a CTE
///* CTE expression, through routine checks to see if the reference is
///* a recursive reference to the CTE.
///*
///* If pFrom matches a CTE according to either of these two above, pFrom->pSTab
///* and other fields are populated accordingly.
///*
///* Return 0 if no match is found.
///* Return 1 if a match is found.
///* Return 2 if an error condition is detected.
#[allow(unused_doc_comments)]
extern "C" fn resolve_from_term_to_cte(p_parse_1: *mut Parse,
    p_walker_1: *mut Walker, p_from_1: *mut SrcItem) -> i32 {
    unsafe {
        let mut p_cte: *mut Cte = core::ptr::null_mut();
        /// Matched CTE (or NULL if no match)
        let mut p_with: *mut With = core::ptr::null_mut();

        /// The matching WITH
        { let _ = 0; };
        if unsafe { (*p_parse_1).p_with } == core::ptr::null_mut() {

            /// There are no WITH clauses in the stack.  No match is possible
            return 0;
        }
        if unsafe { (*p_parse_1).n_err } != 0 {

            /// Prior errors might have left pParse->pWith in a goofy state, so
            ///* go no further.
            return 0;
        }
        { let _ = 0; };
        if unsafe { (*p_from_1).fg.fixed_schema() } as i32 == 0 &&
                unsafe { (*p_from_1).u4.z_database } != core::ptr::null_mut()
            {

            /// The FROM term contains a schema qualifier (ex: main.t1) and so
            ///* it cannot possibly be a CTE reference.
            return 0;
        }
        if unsafe { (*p_from_1).fg.not_cte() } != 0 {

            /// The FROM term is specifically excluded from matching a CTE.
            ///*   (1)  It is part of a trigger that used to have zDatabase but had
            ///*        zDatabase removed by sqlite3FixTriggerStep().
            ///*   (2)  This is the first term in the FROM clause of an UPDATE.
            return 0;
        }
        p_cte =
            search_with(unsafe { (*p_parse_1).p_with }, unsafe { &*p_from_1 },
                &mut p_with);
        if !(p_cte).is_null() {
            let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
            let mut p_tab: *mut Table = core::ptr::null_mut();
            let mut p_e_list: *mut ExprList = core::ptr::null_mut();
            let mut p_sel: *mut Select = core::ptr::null_mut();
            let mut p_left: *const Select = core::ptr::null();
            /// Left-most SELECT statement
            let mut p_rec_term: *mut Select = core::ptr::null_mut();
            /// Left-most recursive term
            let mut b_may_recursive: i32 = 0;
            /// True if compound joined by UNION [ALL]
            let mut p_saved_with: *mut With = core::ptr::null_mut();
            /// Initial value of pParse->pWith
            let mut i_rec_tab: i32 = -1;
            /// Cursor for recursive table
            let mut p_cte_use: *mut CteUse = core::ptr::null_mut();
            if !(unsafe { (*p_cte).z_cte_err }).is_null() {
                unsafe {
                    sqlite3_error_msg(p_parse_1, unsafe { (*p_cte).z_cte_err },
                        unsafe { (*p_cte).z_name })
                };
                return 2;
            }
            if cannot_be_function(p_parse_1, unsafe { &*p_from_1 }) != 0 {
                return 2;
            }
            { let _ = 0; };
            p_tab =
                unsafe {
                        sqlite3_db_malloc_zero(db,
                            core::mem::size_of::<Table>() as u64)
                    } as *mut Table;
            if p_tab == core::ptr::null_mut() { return 2; }
            p_cte_use = unsafe { (*p_cte).p_use };
            if p_cte_use == core::ptr::null_mut() {
                unsafe {
                    (*p_cte).p_use =
                        {
                            p_cte_use =
                                unsafe {
                                        sqlite3_db_malloc_zero(db,
                                            core::mem::size_of::<CteUse>() as u64)
                                    } as *mut CteUse;
                            p_cte_use
                        }
                };
                if p_cte_use == core::ptr::null_mut() ||
                        unsafe {
                                sqlite3_parser_add_cleanup(p_parse_1, Some(sqlite3_db_free),
                                    p_cte_use as *mut ())
                            } == core::ptr::null_mut() {
                    unsafe { sqlite3_db_free(db, p_tab as *mut ()) };
                    return 2;
                }
                unsafe { (*p_cte_use).e_m10d = unsafe { (*p_cte).e_m10d } };
            }
            unsafe { (*p_from_1).p_s_tab = p_tab };
            unsafe { (*p_tab).n_tab_ref = 1 as u32 };
            unsafe {
                (*p_tab).z_name =
                    unsafe {
                        sqlite3_db_str_dup(db,
                            unsafe { (*p_cte).z_name } as *const i8)
                    }
            };
            unsafe { (*p_tab).i_p_key = -1 as i16 };
            unsafe { (*p_tab).n_row_log_est = 200 as LogEst };
            { let _ = 0; };
            unsafe { (*p_tab).tab_flags |= (16384 | 512) as u32 };
            unsafe {
                sqlite3_src_item_attach_subquery(p_parse_1, p_from_1,
                    unsafe { (*p_cte).p_select }, 1)
            };
            if unsafe { (*db).malloc_failed } != 0 { return 2; }
            { let _ = 0; };
            p_sel = unsafe { (*unsafe { (*p_from_1).u4.p_subq }).p_select };
            { let _ = 0; };
            unsafe { (*p_sel).sel_flags |= 67108864 as u32 };
            if unsafe { (*p_from_1).fg.is_indexed_by() } != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse_1,
                        c"no such index: \"%s\"".as_ptr() as *mut i8 as *const i8,
                        unsafe { (*p_from_1).u1.z_indexed_by })
                };
                return 2;
            }
            { let _ = 0; };
            unsafe { (*p_from_1).fg.set_is_cte(1 as u32 as u32) };
            unsafe { (*p_from_1).u2.p_cte_use = p_cte_use };
            {
                let __p = unsafe { &mut (*p_cte_use).n_use };
                let __t = *__p;
                *__p += 1;
                __t
            };

            /// Check if this is a recursive CTE.
            (p_rec_term = p_sel);
            b_may_recursive =
                (unsafe { (*p_sel).op } as i32 == 136 ||
                        unsafe { (*p_sel).op } as i32 == 135) as i32;
            while b_may_recursive != 0 &&
                    unsafe { (*p_rec_term).op } as i32 ==
                        unsafe { (*p_sel).op } as i32 {
                let mut i: i32 = 0;
                let p_src: *mut SrcList = unsafe { (*p_rec_term).p_src };
                { let _ = 0; };
                {
                    i = 0;
                    '__b23: loop {
                        if !(i < unsafe { (*p_src).n_src }) { break '__b23; }
                        '__c23: loop {
                            let p_item: *mut SrcItem =
                                unsafe {
                                    &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(i as isize)
                                };
                            if unsafe { (*p_item).z_name } != core::ptr::null_mut() &&
                                                (unsafe { (*p_item).fg.had_schema() } == 0) as i32 != 0 &&
                                            (unsafe { (*p_item).fg.is_subquery() } == 0) as i32 != 0 &&
                                        (unsafe { (*p_item).fg.fixed_schema() } != 0 ||
                                            unsafe { (*p_item).u4.z_database } == core::ptr::null_mut())
                                    &&
                                    0 ==
                                        unsafe {
                                            sqlite3_str_i_cmp(unsafe { (*p_item).z_name } as *const i8,
                                                unsafe { (*p_cte).z_name } as *const i8)
                                        } {
                                unsafe { (*p_item).p_s_tab = p_tab };
                                {
                                    let __p = unsafe { &mut (*p_tab).n_tab_ref };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                unsafe { (*p_item).fg.set_is_recursive(1 as u32 as u32) };
                                if unsafe { (*p_rec_term).sel_flags } & 8192 as u32 != 0 {
                                    unsafe {
                                        sqlite3_error_msg(p_parse_1,
                                            c"multiple references to recursive table: %s".as_ptr() as
                                                    *mut i8 as *const i8, unsafe { (*p_cte).z_name })
                                    };
                                    return 2;
                                }
                                unsafe { (*p_rec_term).sel_flags |= 8192 as u32 };
                                if i_rec_tab < 0 {
                                    i_rec_tab =
                                        {
                                            let __p = unsafe { &mut (*p_parse_1).n_tab };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                }
                                unsafe { (*p_item).i_cursor = i_rec_tab };
                            }
                            break '__c23;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if unsafe { (*p_rec_term).sel_flags } & 8192 as u32 ==
                        0 as u32 {
                    break;
                }
                p_rec_term = unsafe { (*p_rec_term).p_prior };
            }
            unsafe {
                (*p_cte).z_cte_err =
                    c"circular reference: %s".as_ptr() as *mut i8 as *const i8
            };
            p_saved_with = unsafe { (*p_parse_1).p_with };
            unsafe { (*p_parse_1).p_with = p_with };
            if unsafe { (*p_sel).sel_flags } & 8192 as u32 != 0 {
                let mut rc: i32 = 0;
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                unsafe { (*p_rec_term).p_with = unsafe { (*p_sel).p_with } };
                rc = unsafe { sqlite3_walk_select(p_walker_1, p_rec_term) };
                unsafe { (*p_rec_term).p_with = core::ptr::null_mut() };
                if rc != 0 {
                    unsafe { (*p_parse_1).p_with = p_saved_with };
                    return 2;
                }
            } else {
                if unsafe { sqlite3_walk_select(p_walker_1, p_sel) } != 0 {
                    unsafe { (*p_parse_1).p_with = p_saved_with };
                    return 2;
                }
            }
            unsafe { (*p_parse_1).p_with = p_with };
            {
                p_left = p_sel;
                '__b24: loop {
                    if !(!(unsafe { (*p_left).p_prior }).is_null()) {
                        break '__b24;
                    }
                    '__c24: loop { break '__c24; }
                    p_left = unsafe { (*p_left).p_prior };
                }
            }
            p_e_list = unsafe { (*p_left).p_e_list };
            if !(unsafe { (*p_cte).p_cols }).is_null() {
                if !(p_e_list).is_null() &&
                        unsafe { (*p_e_list).n_expr } !=
                            unsafe { (*unsafe { (*p_cte).p_cols }).n_expr } {
                    unsafe {
                        sqlite3_error_msg(p_parse_1,
                            c"table %s has %d values for %d columns".as_ptr() as *mut i8
                                as *const i8, unsafe { (*p_cte).z_name },
                            unsafe { (*p_e_list).n_expr },
                            unsafe { (*unsafe { (*p_cte).p_cols }).n_expr })
                    };
                    unsafe { (*p_parse_1).p_with = p_saved_with };
                    return 2;
                }
                p_e_list = unsafe { (*p_cte).p_cols };
            }
            sqlite3_columns_from_expr_list(p_parse_1, p_e_list,
                unsafe { &mut (*p_tab).n_col },
                unsafe { &mut (*p_tab).a_col });
            if b_may_recursive != 0 {
                if unsafe { (*p_sel).sel_flags } & 8192 as u32 != 0 {
                    unsafe {
                        (*p_cte).z_cte_err =
                            c"multiple recursive references: %s".as_ptr() as *mut i8 as
                                *const i8
                    };
                } else {
                    unsafe {
                        (*p_cte).z_cte_err =
                            c"recursive reference in a subquery: %s".as_ptr() as *mut i8
                                as *const i8
                    };
                }
                unsafe { sqlite3_walk_select(p_walker_1, p_sel) };
            }
            unsafe { (*p_cte).z_cte_err = core::ptr::null() };
            unsafe { (*p_parse_1).p_with = p_saved_with };
            return 1;
        }
        return 0;
    }
}

///* If the source-list item passed as an argument was augmented with an
///* INDEXED BY clause, then try to locate the specified index. If there
///* was such a clause and the named index cannot be found, return
///* SQLITE_ERROR and leave an error in pParse. Otherwise, populate
///* pFrom->pIndex and return SQLITE_OK.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_indexed_by_lookup(p_parse: *mut Parse,
    p_from: &mut SrcItem) -> i32 {
    unsafe {
        let p_tab: *const Table = (*p_from).p_s_tab as *const Table;
        let z_indexed_by: *mut i8 = (*p_from).u1.z_indexed_by;
        let mut p_idx: *mut Index = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        {
            p_idx = unsafe { (*p_tab).p_index };
            '__b25: loop {
                if !(!(p_idx).is_null() &&
                                unsafe {
                                        sqlite3_str_i_cmp(unsafe { (*p_idx).z_name } as *const i8,
                                            z_indexed_by as *const i8)
                                    } != 0) {
                    break '__b25;
                }
                '__c25: loop { break '__c25; }
                p_idx = unsafe { (*p_idx).p_next };
            }
        }
        if (p_idx).is_null() as i32 != 0 {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"no such index: %s".as_ptr() as *mut i8 as *const i8,
                    z_indexed_by, 0)
            };
            unsafe { (*p_parse).set_check_schema(1 as Bft as u32) };
            return 1;
        }
        { let _ = 0; };
        (*p_from).u2.p_ib_index = p_idx;
        return 0;
    }
}

///* Return the index of a column in a table.  Return -1 if the column
///* is not contained in the table.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_column_index(p_tab: &Table, z_col: *const i8)
    -> i32 {
    let mut i: i32 = 0;
    let mut h: u8 = 0 as u8;
    let mut a_col: *const Column = core::ptr::null();
    let mut n_col: i32 = 0;
    h = unsafe { sqlite3_str_i_hash(z_col) };
    a_col = (*p_tab).a_col as *const Column;
    n_col = (*p_tab).n_col as i32;

    /// See if the aHx gives us a lucky match
    (i =
        (*p_tab).a_hx[(h as u64 % core::mem::size_of::<[u8; 16]>() as u64) as
                    usize] as i32);
    { let _ = 0; };
    if unsafe { (*a_col.offset(i as isize)).h_name } as i32 == h as i32 &&
            unsafe {
                    sqlite3_str_i_cmp(unsafe {
                                (*a_col.offset(i as isize)).z_cn_name
                            } as *const i8, z_col)
                } == 0 {
        return i;
    }

    /// No lucky match from the hash table.  Do a full search.
    (i = 0);
    loop {
        if unsafe { (*a_col.offset(i as isize)).h_name } as i32 == h as i32 &&
                unsafe {
                        sqlite3_str_i_cmp(unsafe {
                                    (*a_col.offset(i as isize)).z_cn_name
                                } as *const i8, z_col)
                    } == 0 {
            return i;
        }
        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        if i >= n_col { break; }
    }
    return -1;
}

///* Mark a subquery result column as having been used.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_src_item_column_used(p_item: &SrcItem, i_col: i32)
    -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        if (*p_item).fg.is_nested_from() != 0 {
            let mut p_results: *mut ExprList = core::ptr::null_mut();
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            p_results =
                unsafe {
                    (*unsafe { (*(*p_item).u4.p_subq).p_select }).p_e_list
                };
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                (*(unsafe { (*p_results).a.as_ptr() } as
                                    *mut ExprListItem).offset(i_col as
                                    isize)).fg.set_b_used(1 as u32 as u32)
            };
        }
    }
}

///* Search the tables iStart..iEnd (inclusive) in pSrc, looking for a
///* table that has a column named zCol.  The search is left-to-right.
///* The first match found is returned.
///*
///* When found, set *piTab and *piCol to the table index and column index
///* of the matching column and return TRUE.
///*
///* If not found, return FALSE.
#[allow(unused_doc_comments)]
extern "C" fn table_and_column_index(p_src_1: &mut SrcList, i_start_1: i32,
    i_end_1: i32, z_col_1: *const i8, pi_tab_1: *mut i32, pi_col_1: &mut i32,
    b_ignore_hidden_1: i32) -> i32 {
    let mut i: i32 = 0;
    /// For looping over tables in pSrc
    let mut i_col: i32 = 0;

    /// Index of column matching zCol
    { let _ = 0; };
    { let _ = 0; };
    { let _ = 0; };
    {
        i = i_start_1;
        '__b27: loop {
            if !(i <= i_end_1) { break '__b27; }
            '__c27: loop {
                i_col =
                    sqlite3_column_index(unsafe {
                            &*unsafe {
                                        (*((*p_src_1).a.as_ptr() as
                                                        *mut SrcItem).offset(i as isize)).p_s_tab
                                    }
                        }, z_col_1);
                if i_col >= 0 &&
                        (b_ignore_hidden_1 == 0 ||
                            (unsafe {
                                                    (*unsafe {
                                                                    &mut *unsafe {
                                                                                (*unsafe {
                                                                                                    (*((*p_src_1).a.as_ptr() as
                                                                                                                    *mut SrcItem).offset(i as isize)).p_s_tab
                                                                                                }).a_col.offset(i_col as isize)
                                                                            }
                                                                }).col_flags
                                                } as i32 & 2 != 0) as i32 == 0) {
                    if !(pi_tab_1).is_null() {
                        sqlite3_src_item_column_used(unsafe {
                                &*((*p_src_1).a.as_ptr() as *mut SrcItem).offset(i as isize)
                            }, i_col);
                        unsafe { *pi_tab_1 = i };
                        *pi_col_1 = i_col;
                    }
                    return 1;
                }
                break '__c27;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 0;
}

///* Set the EP_OuterON property on all terms of the given expression.
///* And set the Expr.w.iJoin to iTable for every term in the
///* expression.
///*
///* The EP_OuterON property is used on terms of an expression to tell
///* the OUTER JOIN processing logic that this term is part of the
///* join restriction specified in the ON or USING clause and not a part
///* of the more general WHERE clause.  These terms are moved over to the
///* WHERE clause during join processing but we need to remember that they
///* originated in the ON or USING clause.
///*
///* The Expr.w.iJoin tells the WHERE clause processing that the
///* expression depends on table w.iJoin even if that table is not
///* explicitly mentioned in the expression.  That information is needed
///* for cases like this:
///*
///*    SELECT * FROM t1 LEFT JOIN t2 ON t1.a=t2.b AND t1.x=5
///*
///* The where clause needs to defer the handling of the t1.x=5
///* term until after the t2 loop of the join.  In that way, a
///* NULL t2 row will be inserted whenever t1.x!=5.  If we do not
///* defer the handling of t1.x=5, it will be processed immediately
///* after the t1 loop and rows with t1.x!=5 will never appear in
///* the output, which is incorrect.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_set_join_expr(mut p: *mut Expr, i_table: i32,
    join_flag: u32) -> () {
    unsafe {
        { let _ = 0; };
        while !(p).is_null() {
            unsafe { (*p).flags |= join_flag as u32 };
            { let _ = 0; };
            unsafe { (*p).w.i_join = i_table };
            if unsafe { (*p).flags } & 4096 as u32 == 0 as u32 {
                if !(unsafe { (*p).x.p_list }).is_null() {
                    let mut i: i32 = 0;
                    {
                        i = 0;
                        '__b29: loop {
                            if !(i < unsafe { (*unsafe { (*p).x.p_list }).n_expr }) {
                                break '__b29;
                            }
                            '__c29: loop {
                                sqlite3_set_join_expr(unsafe {
                                        (*(unsafe { (*unsafe { (*p).x.p_list }).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                    }, i_table, join_flag);
                                break '__c29;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
            }
            sqlite3_set_join_expr(unsafe { (*p).p_left }, i_table, join_flag);
            p = unsafe { (*p).p_right };
        }
    }
}

///* This routine processes the join information for a SELECT statement.
///*
///*   *  A NATURAL join is converted into a USING join.  After that, we
///*      do not need to be concerned with NATURAL joins and we only have
///*      think about USING joins.
///*
///*   *  ON and USING clauses result in extra terms being added to the
///*      WHERE clause to enforce the specified constraints.  The extra
///*      WHERE clause terms will be tagged with EP_OuterON or
///*      EP_InnerON so that we know that they originated in ON/USING.
///*
///* The terms of a FROM clause are contained in the Select.pSrc structure.
///* The left most table is the first entry in Select.pSrc.  The right-most
///* table is the last entry.  The join operator is held in the entry to
///* the right.  Thus entry 1 contains the join operator for the join between
///* entries 0 and 1.  Any ON or USING clauses associated with the join are
///* also attached to the right entry.
///*
///* This routine returns the number of errors encountered.
#[allow(unused_doc_comments)]
extern "C" fn sqlite3_process_join(p_parse_1: *mut Parse, p: &mut Select)
    -> i32 {
    unsafe {
        unsafe {
            let mut p_src: *mut SrcList = core::ptr::null_mut();
            /// All tables in the FROM clause
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            /// Loop counters
            let mut p_left: *mut SrcItem = core::ptr::null_mut();
            /// Left table being joined
            let mut p_right: *mut SrcItem = core::ptr::null_mut();

            /// Right table being joined
            (p_src = (*p).p_src);
            p_left =
                unsafe {
                    &mut *(unsafe { (*p_src).a.as_ptr() } as
                                    *mut SrcItem).offset(0 as isize)
                };
            p_right = unsafe { p_left.offset(1 as isize) };
            {
                i = 0;
                '__b30: loop {
                    if !(i < unsafe { (*p_src).n_src } - 1) { break '__b30; }
                    '__c30: loop {
                        let p_right_tab: *mut Table = unsafe { (*p_right).p_s_tab };
                        let mut join_type: u32 = 0 as u32;
                        if unsafe { (*p_left).p_s_tab } == core::ptr::null_mut() ||
                                p_right_tab == core::ptr::null_mut() {
                            break '__c30;
                        }
                        join_type =
                            if unsafe { (*p_right).fg.jointype } as i32 & 32 != 0 {
                                    1
                                } else { 2 } as u32;
                        if unsafe { (*p_right).fg.jointype } as i32 & 4 != 0 {
                            let mut p_using: *mut IdList = core::ptr::null_mut();
                            if unsafe { (*p_right).fg.is_using() } != 0 ||
                                    !(unsafe { (*p_right).u3.p_on }).is_null() {
                                unsafe {
                                    sqlite3_error_msg(p_parse_1,
                                        c"a NATURAL join may not have an ON or USING clause".as_ptr()
                                                as *mut i8 as *const i8, 0)
                                };
                                return 1;
                            }
                            {
                                j = 0;
                                '__b31: loop {
                                    if !(j < unsafe { (*p_right_tab).n_col } as i32) {
                                        break '__b31;
                                    }
                                    '__c31: loop {
                                        let mut z_name: *const i8 = core::ptr::null();
                                        if unsafe {
                                                            (*unsafe {
                                                                            &mut *unsafe { (*p_right_tab).a_col.offset(j as isize) }
                                                                        }).col_flags
                                                        } as i32 & 2 != 0 {
                                            break '__c31;
                                        }
                                        z_name =
                                            unsafe {
                                                (*unsafe {
                                                            (*p_right_tab).a_col.offset(j as isize)
                                                        }).z_cn_name
                                            };
                                        if table_and_column_index(unsafe { &mut *p_src }, 0, i,
                                                    z_name as *const i8, core::ptr::null_mut(), &mut 0, 1) != 0
                                            {
                                            p_using =
                                                unsafe {
                                                    sqlite3_id_list_append(p_parse_1, p_using,
                                                        core::ptr::null_mut())
                                                };
                                            if !(p_using).is_null() {
                                                { let _ = 0; };
                                                { let _ = 0; };
                                                unsafe {
                                                    (*(unsafe { (*p_using).a.as_ptr() } as
                                                                        *mut IdListItem).offset((unsafe { (*p_using).n_id } - 1) as
                                                                        isize)).z_name =
                                                        unsafe {
                                                            sqlite3_db_str_dup(unsafe { (*p_parse_1).db },
                                                                z_name as *const i8)
                                                        }
                                                };
                                            }
                                        }
                                        break '__c31;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if !(p_using).is_null() {
                                unsafe { (*p_right).fg.set_is_using(1 as u32 as u32) };
                                unsafe {
                                    (*p_right).fg.set_is_synth_using(1 as u32 as u32)
                                };
                                unsafe { (*p_right).u3.p_using = p_using };
                            }
                            if unsafe { (*p_parse_1).n_err } != 0 { return 1; }
                        }
                        if unsafe { (*p_right).fg.is_using() } != 0 {
                            let p_list: *const IdList =
                                unsafe { (*p_right).u3.p_using } as *const IdList;
                            let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
                            { let _ = 0; };
                            {
                                j = 0;
                                '__b32: loop {
                                    if !(j < unsafe { (*p_list).n_id }) { break '__b32; }
                                    '__c32: loop {
                                        let mut z_name_1: *mut i8 = core::ptr::null_mut();
                                        /// Name of the term in the USING clause
                                        let mut i_left: i32 = 0;
                                        /// Table on the left with matching column name
                                        let mut i_left_col: i32 = 0;
                                        /// Column number of matching column on the left
                                        let mut i_right_col: i32 = 0;
                                        /// Column number of matching column on the right
                                        let mut p_e1: *mut Expr = core::ptr::null_mut();
                                        /// Reference to the column on the LEFT of the join
                                        let mut p_e2: *mut Expr = core::ptr::null_mut();
                                        /// Reference to the column on the RIGHT of the join
                                        let mut p_eq: *mut Expr = core::ptr::null_mut();

                                        /// Equality constraint.  pE1 == pE2
                                        (z_name_1 =
                                            unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut IdListItem).offset(j as isize)).z_name
                                            });
                                        i_right_col =
                                            sqlite3_column_index(unsafe { &*p_right_tab },
                                                z_name_1 as *const i8);
                                        if i_right_col < 0 ||
                                                table_and_column_index(unsafe { &mut *p_src }, 0, i,
                                                        z_name_1 as *const i8, &mut i_left, &mut i_left_col,
                                                        unsafe { (*p_right).fg.is_synth_using() } as i32) == 0 {
                                            unsafe {
                                                sqlite3_error_msg(p_parse_1,
                                                    c"cannot join using column %s - column not present in both tables".as_ptr()
                                                            as *mut i8 as *const i8, z_name_1)
                                            };
                                            return 1;
                                        }
                                        p_e1 =
                                            unsafe {
                                                sqlite3_create_column_expr(db, p_src, i_left, i_left_col)
                                            };
                                        sqlite3_src_item_column_used(unsafe {
                                                &*(unsafe { (*p_src).a.as_ptr() } as
                                                                *mut SrcItem).offset(i_left as isize)
                                            }, i_left_col);
                                        if unsafe {
                                                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                    *mut SrcItem).offset(0 as isize)).fg.jointype
                                                            } as i32 & 64 != 0 && unsafe { (*p_parse_1).n_err } == 0 {
                                            /// This branch runs if the query contains one or more RIGHT or FULL
                                            ///* JOINs.  If only a single table on the left side of this join
                                            ///* contains the zName column, then this branch is a no-op.
                                            ///* But if there are two or more tables on the left side
                                            ///* of the join, construct a coalesce() function that gathers all
                                            ///* such tables.  Raise an error if more than one of those references
                                            ///* to zName is not also within a prior USING clause.
                                            ///*
                                            ///* We really ought to raise an error if there are two or more
                                            ///* non-USING references to zName on the left of an INNER or LEFT
                                            ///* JOIN.  But older versions of SQLite do not do that, so we avoid
                                            ///* adding a new error so as to not break legacy applications.
                                            let mut p_func_args: *mut ExprList = core::ptr::null_mut();
                                            { let _ = 0; };
                                            unsafe { (*p_e1).flags |= 2097152 as u32 };
                                            while table_and_column_index(unsafe { &mut *p_src },
                                                        i_left + 1, i, z_name_1 as *const i8, &mut i_left,
                                                        &mut i_left_col,
                                                        unsafe { (*p_right).fg.is_synth_using() } as i32) != 0 {
                                                if unsafe {
                                                                    (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                        *mut SrcItem).offset(i_left as isize)).fg.is_using()
                                                                } as i32 == 0 ||
                                                        unsafe {
                                                                sqlite3_id_list_index(unsafe {
                                                                        (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                            *mut SrcItem).offset(i_left as isize)).u3.p_using
                                                                    }, z_name_1 as *const i8)
                                                            } < 0 {
                                                    unsafe {
                                                        sqlite3_error_msg(p_parse_1,
                                                            c"ambiguous reference to %s in USING()".as_ptr() as *mut i8
                                                                as *const i8, z_name_1)
                                                    };
                                                    break;
                                                }
                                                p_func_args =
                                                    unsafe {
                                                        sqlite3_expr_list_append(p_parse_1, p_func_args, p_e1)
                                                    };
                                                p_e1 =
                                                    unsafe {
                                                        sqlite3_create_column_expr(db, p_src, i_left, i_left_col)
                                                    };
                                                sqlite3_src_item_column_used(unsafe {
                                                        &*(unsafe { (*p_src).a.as_ptr() } as
                                                                        *mut SrcItem).offset(i_left as isize)
                                                    }, i_left_col);
                                            }
                                            if !(p_func_args).is_null() {
                                                p_func_args =
                                                    unsafe {
                                                        sqlite3_expr_list_append(p_parse_1, p_func_args, p_e1)
                                                    };
                                                p_e1 =
                                                    unsafe {
                                                        sqlite3_expr_function(p_parse_1, p_func_args, &tk_coalesce,
                                                            0)
                                                    };
                                                if !(p_e1).is_null() {
                                                    unsafe { (*p_e1).aff_expr = 88 as i8 };
                                                }
                                            }
                                        } else if unsafe {
                                                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                    *mut SrcItem).offset((i + 1) as isize)).fg.jointype
                                                            } as i32 & 8 != 0 && unsafe { (*p_parse_1).n_err } == 0 {
                                            { let _ = 0; };
                                            unsafe { (*p_e1).flags |= 2097152 as u32 };
                                        }
                                        p_e2 =
                                            unsafe {
                                                sqlite3_create_column_expr(db, p_src, i + 1, i_right_col)
                                            };
                                        sqlite3_src_item_column_used(unsafe { &*p_right },
                                            i_right_col);
                                        p_eq = unsafe { sqlite3_p_expr(p_parse_1, 54, p_e1, p_e2) };
                                        { let _ = 0; };
                                        if !(p_eq).is_null() {
                                            unsafe { (*p_eq).flags |= join_type as u32 };
                                            { let _ = 0; };
                                            unsafe { (*p_eq).w.i_join = unsafe { (*p_e2).i_table } };
                                        }
                                        (*p).p_where =
                                            unsafe { sqlite3_expr_and(p_parse_1, (*p).p_where, p_eq) };
                                        break '__c32;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        } else if !(unsafe { (*p_right).u3.p_on }).is_null() {
                            sqlite3_set_join_expr(unsafe { (*p_right).u3.p_on },
                                unsafe { (*p_right).i_cursor }, join_type);
                            (*p).p_where =
                                unsafe {
                                    sqlite3_expr_and(p_parse_1, (*p).p_where,
                                        unsafe { (*p_right).u3.p_on })
                                };
                            unsafe { (*p_right).u3.p_on = core::ptr::null_mut() };
                            unsafe { (*p_right).fg.set_is_on(1 as u32 as u32) };
                            (*p).sel_flags |= 1073741824 as u32;
                        }
                        if unsafe { (*p_right).fg.is_tab_func() } != 0 &&
                                    join_type == 1 as u32 &&
                                !(unsafe { (*p_right).u1.p_func_arg }).is_null() {
                            (*p).sel_flags |= 1073741824 as u32;
                        }
                        break '__c30;
                    }
                    {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut p_right;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        {
                            let __p = &mut p_left;
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
}

///* Check the N SrcItem objects to the right of pBase.  (N might be zero!)
///* If any of those SrcItem objects have a USING clause containing zName
///* then return true.
///*
///* If N is zero, or none of the N SrcItem objects to the right of pBase
///* contains a USING clause, or if none of the USING clauses contain zName,
///* then return false.
extern "C" fn in_any_using_clause(z_name_1: *const i8,
    mut p_base_1: *const SrcItem, mut n_1: i32) -> i32 {
    unsafe {
        while n_1 > 0 {
            { let __p = &mut n_1; let __t = *__p; *__p -= 1; __t };
            {
                let __p = &mut p_base_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            if unsafe { (*p_base_1).fg.is_using() } as i32 == 0 { continue; }
            if unsafe { (*p_base_1).u3.p_using } == core::ptr::null_mut() {
                continue;
            }
            if unsafe {
                        sqlite3_id_list_index(unsafe { (*p_base_1).u3.p_using },
                            z_name_1)
                    } >= 0 {
                return 1;
            }
        }
        return 0;
    }
}

///* This routine is a Walker callback for "expanding" a SELECT statement.
///* "Expanding" means to do the following:
///*
///*    (1)  Make sure VDBE cursor numbers have been assigned to every
///*         element of the FROM clause.
///*
///*    (2)  Fill in the pTabList->a[].pTab fields in the SrcList that
///*         defines FROM clause.  When views appear in the FROM clause,
///*         fill pTabList->a[].pSelect with a copy of the SELECT statement
///*         that implements the view.  A copy is made of the view's SELECT
///*         statement so that we can freely modify or delete that statement
///*         without worrying about messing up the persistent representation
///*         of the view.
///*
///*    (3)  Add terms to the WHERE clause to accommodate the NATURAL keyword
///*         on joins and the ON and USING clause of joins.
///*
///*    (4)  Scan the list of columns in the result set (pEList) looking
///*         for instances of the "*" operator or the TABLE.* operator.
///*         If found, expand each "*" to be every column in every table
///*         and TABLE.* to be every column in TABLE.
///*
#[allow(unused_doc_comments)]
extern "C" fn select_expander(p_walker_1: *mut Walker, p: *mut Select)
    -> i32 {
    unsafe {
        let p_parse: *mut Parse = unsafe { (*p_walker_1).p_parse };
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut rc: i32 = 0;
        let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        let mut p_from: *mut SrcItem = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        let mut p_e: *const Expr = core::ptr::null();
        let mut p_right: *mut Expr = core::ptr::null_mut();
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        let sel_flags: u16 = unsafe { (*p).sel_flags } as u16;
        let mut elist_flags: u32 = 0 as u32;
        unsafe { (*p).sel_flags |= 64 as u32 };
        if unsafe { (*db).malloc_failed } != 0 { return 2; }
        { let _ = 0; };
        if sel_flags as i32 & 64 != 0 { return 1; }
        if unsafe { (*p_walker_1).e_code } != 0 {

            /// Renumber selId because it has been copied from a view
            unsafe {
                (*p).sel_id =
                    {
                            let __p = unsafe { &mut (*p_parse).n_select };
                            *__p += 1;
                            *__p
                        } as u32
            };
        }
        p_tab_list = unsafe { (*p).p_src };
        p_e_list = unsafe { (*p).p_e_list };
        if !(unsafe { (*p_parse).p_with }).is_null() &&
                unsafe { (*p).sel_flags } & 2097152 as u32 != 0 {
            if unsafe { (*p).p_with } == core::ptr::null_mut() {
                unsafe {
                    (*p).p_with =
                        unsafe {
                                sqlite3_db_malloc_zero(db,
                                    core::mem::offset_of!(With, a) as u64 +
                                        1 as u64 * core::mem::size_of::<Cte>() as u64)
                            } as *mut With
                };
                if unsafe { (*p).p_with } == core::ptr::null_mut() {
                    return 2;
                }
            }
            unsafe { (*unsafe { (*p).p_with }).b_view = 1 };
        }
        sqlite3_with_push(p_parse, unsafe { (*p).p_with }, 0 as u8);

        /// Make sure cursor numbers have been assigned to all entries in
        ///* the FROM clause of the SELECT statement.
        unsafe { sqlite3_src_list_assign_cursors(p_parse, p_tab_list) };
        {
            {
                i = 0;
                p_from = unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem
            };
            '__b35: loop {
                if !(i < unsafe { (*p_tab_list).n_src }) { break '__b35; }
                '__c35: loop {
                    let mut p_tab: *mut Table = core::ptr::null_mut();
                    { let _ = 0; };
                    if !(unsafe { (*p_from).p_s_tab }).is_null() {
                        break '__c35;
                    }
                    { let _ = 0; };
                    if unsafe { (*p_from).z_name } == core::ptr::null_mut() {
                        let mut p_sel: *mut Select = core::ptr::null_mut();
                        { let _ = 0; };
                        p_sel =
                            unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select };

                        /// A sub-query in the FROM clause of a SELECT
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { sqlite3_walk_select(p_walker_1, p_sel) } != 0 {
                            return 2;
                        }
                        if sqlite3_expand_subquery(p_parse, p_from) != 0 {
                            return 2;
                        }
                    } else if {
                                rc = resolve_from_term_to_cte(p_parse, p_walker_1, p_from);
                                rc
                            } != 0 {
                        if rc > 1 { return 2; }
                        p_tab = unsafe { (*p_from).p_s_tab };
                        { let _ = 0; };
                    } else {

                        /// An ordinary table or view name in the FROM clause
                        { let _ = 0; };
                        unsafe {
                            (*p_from).p_s_tab =
                                {
                                    p_tab =
                                        unsafe {
                                            sqlite3_locate_table_item(p_parse, 0 as u32, p_from)
                                        };
                                    p_tab
                                }
                        };
                        if p_tab == core::ptr::null_mut() { return 2; }
                        if unsafe { (*p_tab).n_tab_ref } >= 65535 as u32 {
                            unsafe {
                                sqlite3_error_msg(p_parse,
                                    c"too many references to \"%s\": max 65535".as_ptr() as
                                            *mut i8 as *const i8, unsafe { (*p_tab).z_name })
                            };
                            unsafe { (*p_from).p_s_tab = core::ptr::null_mut() };
                            return 2;
                        }
                        {
                            let __p = unsafe { &mut (*p_tab).n_tab_ref };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        if !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0
                                && cannot_be_function(p_parse, unsafe { &*p_from }) != 0 {
                            return 2;
                        }
                        if !(unsafe { (*p_tab).e_tab_type } as i32 == 0) as i32 != 0
                            {
                            let mut n_col: i16 = 0 as i16;
                            let e_code_orig: u8 = unsafe { (*p_walker_1).e_code } as u8;
                            if unsafe { sqlite3_view_get_column_names(p_parse, p_tab) }
                                    != 0 {
                                return 2;
                            }
                            { let _ = 0; };
                            if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                                if unsafe { (*db).flags } & 2147483648u32 as u64 == 0 as u64
                                        &&
                                        unsafe { (*p_tab).p_schema } !=
                                            unsafe {
                                                (*unsafe { (*db).a_db.offset(1 as isize) }).p_schema
                                            } {
                                    unsafe {
                                        sqlite3_error_msg(p_parse,
                                            c"access to view \"%s\" prohibited".as_ptr() as *mut i8 as
                                                *const i8, unsafe { (*p_tab).z_name })
                                    };
                                }
                                unsafe {
                                    sqlite3_src_item_attach_subquery(p_parse, p_from,
                                        unsafe { (*p_tab).u.view.p_select }, 1)
                                };
                            } else if unsafe { (*p_tab).e_tab_type } as i32 == 1 &&
                                            (unsafe { (*p_from).fg.from_ddl() } != 0 ||
                                                unsafe { (*p_parse).prep_flags } as i32 & 32 != 0) &&
                                        unsafe { (*p_tab).u.vtab.p } != core::ptr::null_mut() &&
                                    unsafe { (*unsafe { (*p_tab).u.vtab.p }).e_vtab_risk } as
                                            i32 >
                                        (unsafe { (*db).flags } & 128 as u64 != 0 as u64) as i32 {
                                unsafe {
                                    sqlite3_error_msg(p_parse,
                                        c"unsafe use of virtual table \"%s\"".as_ptr() as *mut i8 as
                                            *const i8, unsafe { (*p_tab).z_name })
                                };
                            }
                            { let _ = 0; };
                            n_col = unsafe { (*p_tab).n_col };
                            unsafe { (*p_tab).n_col = -1 as i16 };
                            unsafe { (*p_walker_1).e_code = 1 as u16 };
                            if unsafe { (*p_from).fg.is_subquery() } != 0 {
                                unsafe {
                                    sqlite3_walk_select(p_walker_1,
                                        unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select })
                                };
                            }
                            unsafe { (*p_walker_1).e_code = e_code_orig as u16 };
                            unsafe { (*p_tab).n_col = n_col };
                        }
                    }
                    if unsafe { (*p_from).fg.is_indexed_by() } != 0 &&
                            sqlite3_indexed_by_lookup(p_parse, unsafe { &mut *p_from })
                                != 0 {
                        return 2;
                    }
                    break '__c35;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_from;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }

        /// Process NATURAL keywords, and ON and USING clauses of joins.
        { let _ = 0; };
        if unsafe { (*p_parse).n_err } != 0 ||
                sqlite3_process_join(p_parse, unsafe { &mut *p }) != 0 {
            return 2;
        }
        {
            k = 0;
            '__b36: loop {
                if !(k < unsafe { (*p_e_list).n_expr }) { break '__b36; }
                '__c36: loop {
                    p_e =
                        unsafe {
                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                            *mut ExprListItem).offset(k as isize)).p_expr
                        };
                    if unsafe { (*p_e).op } as i32 == 180 { break '__b36; }
                    { let _ = 0; };
                    { let _ = 0; };
                    if unsafe { (*p_e).op } as i32 == 142 &&
                            unsafe { (*unsafe { (*p_e).p_right }).op } as i32 == 180 {
                        break '__b36;
                    }
                    elist_flags |= unsafe { (*p_e).flags };
                    break '__c36;
                }
                { let __p = &mut k; let __t = *__p; *__p += 1; __t };
            }
        }
        if k < unsafe { (*p_e_list).n_expr } {
            ///* If we get here it means the result set contains one or more "*"
            ///* operators that need to be expanded.  Loop through each expression
            ///* in the result set and expand them one by one.
            let a: *mut ExprListItem =
                unsafe { (*p_e_list).a.as_ptr() } as *mut ExprListItem;
            let mut p_new: *mut ExprList = core::ptr::null_mut();
            let flags: i32 =
                unsafe { (*unsafe { (*p_parse).db }).flags } as i32;
            let long_names: i32 = (flags & 4 != 0 && flags & 64 == 0) as i32;
            {
                k = 0;
                '__b37: loop {
                    if !(k < unsafe { (*p_e_list).n_expr }) { break '__b37; }
                    '__c37: loop {
                        p_e = unsafe { (*a.offset(k as isize)).p_expr };
                        elist_flags |= unsafe { (*p_e).flags };
                        p_right = unsafe { (*p_e).p_right };
                        { let _ = 0; };
                        if unsafe { (*p_e).op } as i32 != 180 &&
                                (unsafe { (*p_e).op } as i32 != 142 ||
                                    unsafe { (*p_right).op } as i32 != 180) {

                            /// This particular expression does not need to be expanded.
                            (p_new =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse, p_new,
                                        unsafe { (*a.offset(k as isize)).p_expr })
                                });
                            if !(p_new).is_null() {
                                unsafe {
                                    (*(unsafe { (*p_new).a.as_ptr() } as
                                                        *mut ExprListItem).offset((unsafe { (*p_new).n_expr } - 1)
                                                        as isize)).z_e_name =
                                        unsafe { (*a.offset(k as isize)).z_e_name }
                                };
                                unsafe {
                                    (*(unsafe { (*p_new).a.as_ptr() } as
                                                        *mut ExprListItem).offset((unsafe { (*p_new).n_expr } - 1)
                                                        as
                                                        isize)).fg.set_e_e_name(unsafe {
                                                (*a.offset(k as isize)).fg.e_e_name()
                                            } as u32)
                                };
                                unsafe {
                                    (*a.offset(k as isize)).z_e_name = core::ptr::null_mut()
                                };
                            }
                            unsafe {
                                (*a.offset(k as isize)).p_expr = core::ptr::null_mut()
                            };
                        } else {
                            /// This expression is a "*" or a "TABLE.*" and needs to be
                            ///* expanded.
                            let mut table_seen: i32 = 0;
                            /// Set to 1 when TABLE matches
                            let mut z_t_name: *mut i8 = core::ptr::null_mut();
                            /// text of name of TABLE
                            let mut i_err_ofst: i32 = 0;
                            if unsafe { (*p_e).op } as i32 == 142 {
                                { let _ = 0; };
                                { let _ = 0; };
                                { let _ = 0; };
                                z_t_name = unsafe { (*unsafe { (*p_e).p_left }).u.z_token };
                                { let _ = 0; };
                                i_err_ofst =
                                    unsafe { (*unsafe { (*p_e).p_right }).w.i_ofst };
                            } else {
                                { let _ = 0; };
                                i_err_ofst = unsafe { (*p_e).w.i_ofst };
                            }
                            {
                                {
                                    i = 0;
                                    p_from = unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem
                                };
                                '__b38: loop {
                                    if !(i < unsafe { (*p_tab_list).n_src }) { break '__b38; }
                                    '__c38: loop {
                                        let mut n_add: i32 = 0;
                                        /// Number of cols including rowid
                                        let p_tab_1: *mut Table = unsafe { (*p_from).p_s_tab };
                                        /// Table for this data source
                                        let mut p_nested_from: *mut ExprList =
                                            core::ptr::null_mut();
                                        /// Result-set of a nested FROM clause
                                        let mut z_tab_name: *mut i8 = core::ptr::null_mut();
                                        /// AS name for this data source
                                        let mut z_schema_name: *const i8 = core::ptr::null();
                                        /// Schema name for this data source
                                        let mut i_db: i32 = 0;
                                        /// Schema index for this data src
                                        let mut p_using: *mut IdList = core::ptr::null_mut();
                                        if { z_tab_name = unsafe { (*p_from).z_alias }; z_tab_name }
                                                == core::ptr::null_mut() {
                                            z_tab_name = unsafe { (*p_tab_1).z_name };
                                        }
                                        if unsafe { (*db).malloc_failed } != 0 { break '__b38; }
                                        { let _ = 0; };
                                        if unsafe { (*p_from).fg.is_nested_from() } != 0 {
                                            { let _ = 0; };
                                            { let _ = 0; };
                                            p_nested_from =
                                                unsafe {
                                                    (*unsafe {
                                                                    (*unsafe { (*p_from).u4.p_subq }).p_select
                                                                }).p_e_list
                                                };
                                            { let _ = 0; };
                                            { let _ = 0; };
                                            { let _ = 0; };
                                        } else {
                                            if !(z_t_name).is_null() &&
                                                    unsafe {
                                                            sqlite3_str_i_cmp(z_t_name as *const i8,
                                                                z_tab_name as *const i8)
                                                        } != 0 {
                                                break '__c38;
                                            }
                                            p_nested_from = core::ptr::null_mut();
                                            i_db =
                                                unsafe {
                                                    sqlite3_schema_to_index(db, unsafe { (*p_tab_1).p_schema })
                                                };
                                            z_schema_name =
                                                if i_db >= 0 {
                                                        unsafe {
                                                            (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                                        }
                                                    } else { c"*".as_ptr() as *mut i8 } as *const i8;
                                        }
                                        if i + 1 < unsafe { (*p_tab_list).n_src } &&
                                                    unsafe { (*p_from.offset(1 as isize)).fg.is_using() } != 0
                                                && sel_flags as i32 & 2048 != 0 {
                                            let mut ii: i32 = 0;
                                            p_using =
                                                unsafe { (*p_from.offset(1 as isize)).u3.p_using };
                                            {
                                                ii = 0;
                                                '__b39: loop {
                                                    if !(ii < unsafe { (*p_using).n_id }) { break '__b39; }
                                                    '__c39: loop {
                                                        let z_u_name: *const i8 =
                                                            unsafe {
                                                                    (*(unsafe { (*p_using).a.as_ptr() } as
                                                                                    *mut IdListItem).offset(ii as isize)).z_name
                                                                } as *const i8;
                                                        p_right = unsafe { sqlite3_expr(db, 60, z_u_name) };
                                                        unsafe {
                                                            sqlite3_expr_set_error_offset(p_right, i_err_ofst)
                                                        };
                                                        p_new =
                                                            unsafe {
                                                                sqlite3_expr_list_append(p_parse, p_new, p_right)
                                                            };
                                                        if !(p_new).is_null() {
                                                            let p_x: *mut ExprListItem =
                                                                unsafe {
                                                                    &mut *(unsafe { (*p_new).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset((unsafe { (*p_new).n_expr } - 1)
                                                                                    as isize)
                                                                };
                                                            { let _ = 0; };
                                                            unsafe {
                                                                (*p_x).z_e_name =
                                                                    unsafe {
                                                                        sqlite3_m_printf(db,
                                                                            c"..%s".as_ptr() as *mut i8 as *const i8, z_u_name)
                                                                    }
                                                            };
                                                            unsafe { (*p_x).fg.set_e_e_name(2 as u32 as u32) };
                                                            unsafe { (*p_x).fg.set_b_using_term(1 as u32 as u32) };
                                                        }
                                                        break '__c39;
                                                    }
                                                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                        } else { p_using = core::ptr::null_mut(); }
                                        n_add = unsafe { (*p_tab_1).n_col } as i32;
                                        if unsafe { (*p_tab_1).tab_flags } & 512 as u32 == 0 as u32
                                                && sel_flags as i32 & 2048 != 0 {
                                            { let __p = &mut n_add; let __t = *__p; *__p += 1; __t };
                                        }
                                        {
                                            j = 0;
                                            '__b40: loop {
                                                if !(j < n_add) { break '__b40; }
                                                '__c40: loop {
                                                    let mut z_name: *const i8 = core::ptr::null();
                                                    let mut p_x_1: *mut ExprListItem = core::ptr::null_mut();
                                                    if j == unsafe { (*p_tab_1).n_col } as i32 {
                                                        z_name = unsafe { sqlite3_rowid_alias(p_tab_1) };
                                                        if z_name == core::ptr::null() { break '__c40; }
                                                    } else {
                                                        z_name =
                                                            unsafe {
                                                                    (*unsafe { (*p_tab_1).a_col.offset(j as isize) }).z_cn_name
                                                                } as *const i8;
                                                        if !(p_nested_from).is_null() &&
                                                                unsafe {
                                                                            (*(unsafe { (*p_nested_from).a.as_ptr() } as
                                                                                                *mut ExprListItem).offset(j as isize)).fg.e_e_name()
                                                                        } as i32 == 3 {
                                                            break '__c40;
                                                        }
                                                        if !(z_t_name).is_null() && !(p_nested_from).is_null() &&
                                                                unsafe {
                                                                        sqlite3_match_e_name(unsafe {
                                                                                    &raw mut *(unsafe { (*p_nested_from).a.as_ptr() } as
                                                                                                    *mut ExprListItem).offset(j as isize)
                                                                                } as *const ExprListItem, core::ptr::null(),
                                                                            z_t_name as *const i8, core::ptr::null(),
                                                                            core::ptr::null_mut())
                                                                    } == 0 {
                                                            break '__c40;
                                                        }
                                                        if unsafe { (*p).sel_flags } & 131072 as u32 == 0 as u32 &&
                                                                unsafe {
                                                                                (*unsafe {
                                                                                                &mut *unsafe { (*p_tab_1).a_col.offset(j as isize) }
                                                                                            }).col_flags
                                                                            } as i32 & 2 != 0 {
                                                            break '__c40;
                                                        }
                                                        if unsafe {
                                                                                    (*unsafe { (*p_tab_1).a_col.offset(j as isize) }).col_flags
                                                                                } as i32 & 1024 != 0 && z_t_name == core::ptr::null_mut() &&
                                                                sel_flags as i32 & 2048 == 0 {
                                                            break '__c40;
                                                        }
                                                    }
                                                    { let _ = 0; };
                                                    table_seen = 1;
                                                    if i > 0 && z_t_name == core::ptr::null_mut() &&
                                                            sel_flags as i32 & 2048 == 0 {
                                                        if unsafe { (*p_from).fg.is_using() } != 0 &&
                                                                unsafe {
                                                                        sqlite3_id_list_index(unsafe { (*p_from).u3.p_using },
                                                                            z_name)
                                                                    } >= 0 {

                                                            /// In a join with a USING clause, omit columns in the
                                                            ///* using clause from the table on the right.
                                                            break '__c40;
                                                        }
                                                    }
                                                    p_right = unsafe { sqlite3_expr(db, 60, z_name) };
                                                    if unsafe { (*p_tab_list).n_src } > 1 &&
                                                                (unsafe { (*p_from).fg.jointype } as i32 & 64 == 0 ||
                                                                        sel_flags as i32 & 2048 != 0 ||
                                                                    (in_any_using_clause(z_name, p_from as *const SrcItem,
                                                                                    unsafe { (*p_tab_list).n_src } - i - 1) == 0) as i32 != 0)
                                                            || unsafe { (*p_parse).e_parse_mode } as i32 >= 2 {
                                                        let mut p_left: *mut Expr = core::ptr::null_mut();
                                                        p_left =
                                                            unsafe { sqlite3_expr(db, 60, z_tab_name as *const i8) };
                                                        p_expr =
                                                            unsafe { sqlite3_p_expr(p_parse, 142, p_left, p_right) };
                                                        if unsafe { (*p_parse).e_parse_mode } as i32 >= 2 &&
                                                                !(unsafe { (*p_e).p_left }).is_null() {
                                                            unsafe {
                                                                sqlite3_rename_token_remap(p_parse, p_left as *const (),
                                                                    unsafe { (*p_e).p_left } as *const ())
                                                            };
                                                        }
                                                        if !(z_schema_name).is_null() {
                                                            p_left = unsafe { sqlite3_expr(db, 60, z_schema_name) };
                                                            p_expr =
                                                                unsafe { sqlite3_p_expr(p_parse, 142, p_left, p_expr) };
                                                        }
                                                    } else { p_expr = p_right; }
                                                    unsafe {
                                                        sqlite3_expr_set_error_offset(p_expr, i_err_ofst)
                                                    };
                                                    p_new =
                                                        unsafe { sqlite3_expr_list_append(p_parse, p_new, p_expr) };
                                                    if p_new == core::ptr::null_mut() { break '__b40; }
                                                    p_x_1 =
                                                        unsafe {
                                                            &mut *(unsafe { (*p_new).a.as_ptr() } as
                                                                            *mut ExprListItem).offset((unsafe { (*p_new).n_expr } - 1)
                                                                            as isize)
                                                        };
                                                    { let _ = 0; };
                                                    if sel_flags as i32 & 2048 != 0 &&
                                                            !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 !=
                                                                0 {
                                                        if !(p_nested_from).is_null() &&
                                                                ((0 == 0) as i32 != 0 ||
                                                                    j < unsafe { (*p_nested_from).n_expr }) {
                                                            { let _ = 0; };
                                                            unsafe {
                                                                (*p_x_1).z_e_name =
                                                                    unsafe {
                                                                        sqlite3_db_str_dup(db,
                                                                            unsafe {
                                                                                    (*(unsafe { (*p_nested_from).a.as_ptr() } as
                                                                                                    *mut ExprListItem).offset(j as isize)).z_e_name
                                                                                } as *const i8)
                                                                    }
                                                            };
                                                        } else {
                                                            unsafe {
                                                                (*p_x_1).z_e_name =
                                                                    unsafe {
                                                                        sqlite3_m_printf(db,
                                                                            c"%s.%s.%s".as_ptr() as *mut i8 as *const i8, z_schema_name,
                                                                            z_tab_name, z_name)
                                                                    }
                                                            };
                                                        }
                                                        unsafe {
                                                            (*p_x_1).fg.set_e_e_name(if j ==
                                                                                unsafe { (*p_tab_1).n_col } as i32 {
                                                                            3
                                                                        } else { 2 } as u32 as u32)
                                                        };
                                                        if unsafe { (*p_from).fg.is_using() } != 0 &&
                                                                        unsafe {
                                                                                sqlite3_id_list_index(unsafe { (*p_from).u3.p_using },
                                                                                    z_name)
                                                                            } >= 0 ||
                                                                    !(p_using).is_null() &&
                                                                        unsafe { sqlite3_id_list_index(p_using, z_name) } >= 0 ||
                                                                j < unsafe { (*p_tab_1).n_col } as i32 &&
                                                                    unsafe {
                                                                                    (*unsafe { (*p_tab_1).a_col.offset(j as isize) }).col_flags
                                                                                } as i32 & 1024 != 0 {
                                                            unsafe { (*p_x_1).fg.set_b_no_expand(1 as u32 as u32) };
                                                        }
                                                    } else if long_names != 0 {
                                                        unsafe {
                                                            (*p_x_1).z_e_name =
                                                                unsafe {
                                                                    sqlite3_m_printf(db,
                                                                        c"%s.%s".as_ptr() as *mut i8 as *const i8, z_tab_name,
                                                                        z_name)
                                                                }
                                                        };
                                                        unsafe { (*p_x_1).fg.set_e_e_name(0 as u32 as u32) };
                                                    } else {
                                                        unsafe {
                                                            (*p_x_1).z_e_name =
                                                                unsafe { sqlite3_db_str_dup(db, z_name) }
                                                        };
                                                        unsafe { (*p_x_1).fg.set_e_e_name(0 as u32 as u32) };
                                                    }
                                                    break '__c40;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                        break '__c38;
                                    }
                                    {
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_from;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if (table_seen == 0) as i32 != 0 {
                                if !(z_t_name).is_null() {
                                    unsafe {
                                        sqlite3_error_msg(p_parse,
                                            c"no such table: %s".as_ptr() as *mut i8 as *const i8,
                                            z_t_name)
                                    };
                                } else {
                                    unsafe {
                                        sqlite3_error_msg(p_parse,
                                            c"no tables specified".as_ptr() as *mut i8 as *const i8)
                                    };
                                }
                            }
                        }
                        break '__c37;
                    }
                    { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { sqlite3_expr_list_delete(db, p_e_list) };
            unsafe { (*p).p_e_list = p_new };
        }
        if !(unsafe { (*p).p_e_list }).is_null() {
            if unsafe { (*unsafe { (*p).p_e_list }).n_expr } >
                    unsafe { (*db).a_limit[2 as usize] } {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"too many columns in result set".as_ptr() as *mut i8 as
                            *const i8)
                };
                return 2;
            }
            if elist_flags & (8 | 4194304) as u32 != 0 as u32 {
                unsafe { (*p).sel_flags |= 262144 as u32 };
            }
        }
        return 0;
    }
}

///* This routine "expands" a SELECT statement and all of its subqueries.
///* For additional information on what it means to "expand" a SELECT
///* statement, see the comment on the selectExpand worker callback above.
///*
///* Expanding a SELECT statement is the first step in processing a
///* SELECT statement.  The SELECT statement must be expanded before
///* name resolution is performed.
///*
///* If anything goes wrong, an error message is written into pParse.
///* The calling function can detect the problem by looking at pParse->nErr
///* and/or pParse->db->mallocFailed.
extern "C" fn sqlite3_select_expand(p_parse_1: *mut Parse,
    p_select_1: *mut Select) -> () {
    let mut w: Walker = unsafe { core::mem::zeroed() };
    w.x_expr_callback = Some(sqlite3_expr_walk_noop);
    w.p_parse = p_parse_1;
    if unsafe { (*p_parse_1).has_compound() } != 0 {
        w.x_select_callback = Some(convert_compound_select_to_subquery);
        w.x_select_callback2 = None;
        unsafe { sqlite3_walk_select(&mut w, p_select_1) };
    }
    w.x_select_callback = Some(select_expander);
    w.x_select_callback2 = Some(sqlite3_select_pop_with);
    w.e_code = 0 as u16;
    unsafe { sqlite3_walk_select(&mut w, p_select_1) };
}

///* This is a Walker.xSelectCallback callback for the sqlite3SelectTypeInfo()
///* interface.
///*
///* For each FROM-clause subquery, add Column.zType, Column.zColl, and
///* Column.affinity information to the Table structure that represents
///* the result set of that subquery.
///*
///* The Table structure that represents the result set was constructed
///* by selectExpander() but the type and collation and affinity information
///* was omitted at that point because identifiers had not yet been resolved.
///* This routine is called after identifier resolution.
#[allow(unused_doc_comments)]
extern "C" fn select_add_subquery_type_info(p_walker_1: *mut Walker,
    p: *mut Select) -> () {
    unsafe {
        let mut p_parse: *mut Parse = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
        let mut p_from: *const SrcItem = core::ptr::null();
        if unsafe { (*p).sel_flags } & 128 as u32 != 0 { return; }
        unsafe { (*p).sel_flags |= 128 as u32 };
        p_parse = unsafe { (*p_walker_1).p_parse };
        { let _ = 0; };
        p_tab_list = unsafe { (*p).p_src };
        {
            {
                i = 0;
                p_from = unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem
            };
            '__b41: loop {
                if !(i < unsafe { (*p_tab_list).n_src }) { break '__b41; }
                '__c41: loop {
                    let p_tab: *mut Table = unsafe { (*p_from).p_s_tab };
                    { let _ = 0; };
                    if unsafe { (*p_tab).tab_flags } & 16384 as u32 != 0 as u32
                            && unsafe { (*p_from).fg.is_subquery() } != 0 {
                        /// A sub-query in the FROM clause of a SELECT
                        let p_sel: *mut Select =
                            unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select };
                        sqlite3_subquery_column_types(p_parse,
                            unsafe { &mut *p_tab }, p_sel, 64 as i8);
                    }
                    break '__c41;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_from;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
    }
}

///* This routine adds datatype and collating sequence information to
///* the Table structures of all FROM-clause subqueries in a
///* SELECT statement.
///*
///* Use this routine after name resolution.
extern "C" fn sqlite3_select_add_type_info(p_parse_1: *mut Parse,
    p_select_1: *mut Select) -> () {
    let mut w: Walker = unsafe { core::mem::zeroed() };
    w.x_select_callback = Some(sqlite3_select_walk_noop);
    w.x_select_callback2 = Some(select_add_subquery_type_info);
    w.x_expr_callback = Some(sqlite3_expr_walk_noop);
    w.p_parse = p_parse_1;
    unsafe { sqlite3_walk_select(&mut w, p_select_1) };
}

///* This routine sets up a SELECT statement for processing.  The
///* following is accomplished:
///*
///*     *  VDBE Cursor numbers are assigned to all FROM-clause terms.
///*     *  Ephemeral Table objects are created for all FROM-clause subqueries.
///*     *  ON and USING clauses are shifted into WHERE statements
///*     *  Wildcards "*" and "TABLE.*" in result sets are expanded.
///*     *  Identifiers in expression are matched to tables.
///*
///* This routine acts recursively on all subqueries within the SELECT.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_prep(p_parse: *mut Parse, p: *mut Select,
    p_outer_nc: *mut NameContext) -> () {
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0 { return; }
    if unsafe { (*p).sel_flags } & 128 as u32 != 0 { return; }
    sqlite3_select_expand(p_parse, p);
    if unsafe { (*p_parse).n_err } != 0 { return; }
    unsafe { sqlite3_resolve_select_names(p_parse, p, p_outer_nc) };
    if unsafe { (*p_parse).n_err } != 0 { return; }
    sqlite3_select_add_type_info(p_parse, p);
}

///* Given a SELECT statement, generate a Table structure that describes
///* the result set of that SELECT.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_result_set_of_select(p_parse: *mut Parse,
    mut p_select: *mut Select, aff: i8) -> *mut Table {
    unsafe {
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        let mut saved_flags: u64 = 0 as u64;
        {
            let __p = unsafe { &mut (*p_parse).n_nest_sel };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if unsafe { (*p_parse).n_nest_sel } >=
                unsafe { (*db).a_limit[3 as usize] } {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"VIEWs and/or subqueries nested too deep".as_ptr() as
                            *mut i8 as *const i8)
            };
            return core::ptr::null_mut();
        }
        saved_flags = unsafe { (*db).flags };
        unsafe { (*db).flags &= !(4 as u64) };
        unsafe { (*db).flags |= 64 as u64 };
        sqlite3_select_prep(p_parse, p_select, core::ptr::null_mut());
        unsafe { (*db).flags = saved_flags };
        if unsafe { (*p_parse).n_err } != 0 { return core::ptr::null_mut(); }
        while !(unsafe { (*p_select).p_prior }).is_null() {
            p_select = unsafe { (*p_select).p_prior };
        }
        p_tab =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<Table>() as u64)
                } as *mut Table;
        if p_tab == core::ptr::null_mut() { return core::ptr::null_mut(); }
        unsafe { (*p_tab).n_tab_ref = 1 as u32 };
        unsafe { (*p_tab).z_name = core::ptr::null_mut() };
        unsafe { (*p_tab).n_row_log_est = 200 as LogEst };
        { let _ = 0; };
        sqlite3_columns_from_expr_list(p_parse,
            unsafe { (*p_select).p_e_list }, unsafe { &mut (*p_tab).n_col },
            unsafe { &mut (*p_tab).a_col });
        sqlite3_subquery_column_types(p_parse, unsafe { &mut *p_tab },
            p_select, aff);
        unsafe { (*p_tab).i_p_key = -1 as i16 };
        if unsafe { (*db).malloc_failed } != 0 {
            unsafe { sqlite3_delete_table(db, p_tab) };
            return core::ptr::null_mut();
        }
        {
            let __p = unsafe { &mut (*p_parse).n_nest_sel };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        { let _ = 0; };
        return p_tab;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct DistinctCtx {
    is_tnct: u8,
    e_tnct_type: u8,
    tab_tnct: i32,
    addr_tnct: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SortCtx {
    p_order_by: *mut ExprList,
    n_ob_sat: i32,
    i_e_cursor: i32,
    reg_return: i32,
    label_bk_out: i32,
    addr_sort_index: i32,
    label_done: i32,
    label_ob_lopt: i32,
    sort_flags: u8,
    p_deferred_row_load: *mut RowLoadInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RowLoadInfo {
    reg_result: i32,
    ecel_flags: u8,
}

///* Get a VDBE for the given parser context.  Create a new one if necessary.
///* If an error occurs, return NULL and leave a message in pParse.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_get_vdbe(p_parse: *mut Parse) -> *mut Vdbe {
    if !(unsafe { (*p_parse).p_vdbe }).is_null() {
        return unsafe { (*p_parse).p_vdbe };
    }
    if unsafe { (*p_parse).p_toplevel } == core::ptr::null_mut() &&
            unsafe { (*unsafe { (*p_parse).db }).db_opt_flags } & 8 as u32 ==
                0 as u32 {
        unsafe { (*p_parse).set_ok_const_factor(1 as Bft as u32) };
    }
    return unsafe { sqlite3_vdbe_create(p_parse) };
}

///* If any term of pSrc, or any SF_NestedFrom sub-query, is not the same
///* as pSrcItem but has the same alias as p0, then return true.
///* Otherwise return false.
extern "C" fn same_src_alias(p0: *mut SrcItem, p_src_1: &mut SrcList) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b43: loop {
                if !(i < (*p_src_1).n_src) { break '__b43; }
                '__c43: loop {
                    let p1: *mut SrcItem =
                        unsafe {
                            &mut *((*p_src_1).a.as_ptr() as
                                            *mut SrcItem).offset(i as isize)
                        };
                    if p1 == p0 { break '__c43; }
                    if unsafe { (*p0).p_s_tab } == unsafe { (*p1).p_s_tab } &&
                            0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { (*p0).z_alias } as *const i8,
                                        unsafe { (*p1).z_alias } as *const i8)
                                } {
                        return 1;
                    }
                    if unsafe { (*p1).fg.is_subquery() } != 0 &&
                                unsafe {
                                            (*unsafe {
                                                            (*unsafe { (*p1).u4.p_subq }).p_select
                                                        }).sel_flags
                                        } & 2048 as u32 != 0 as u32 &&
                            same_src_alias(p0,
                                    unsafe {
                                        &mut *unsafe {
                                                    (*unsafe { (*unsafe { (*p1).u4.p_subq }).p_select }).p_src
                                                }
                                    }) != 0 {
                        return 1;
                    }
                    break '__c43;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}

/// Undo the work of sqlite3SetJoinExpr().  This is used when a LEFT JOIN
///* is simplified into an ordinary JOIN, and when an ON expression is
///* "pushed down" into the WHERE clause of a subquery.
///*
///* Convert every term that is marked with EP_OuterON and w.iJoin==iTable into
///* an ordinary term that omits the EP_OuterON mark.  Or if iTable<0, then
///* just clear every EP_OuterON and EP_InnerON mark from the expression tree.
///*
///* If nullable is true, that means that Expr p might evaluate to NULL even
///* if it is a reference to a NOT NULL column.  This can happen, for example,
///* if the table that p references is on the left side of a RIGHT JOIN.
///* If nullable is true, then take care to not remove the EP_CanBeNull bit.
///* See forum thread https://sqlite.org/forum/forumpost/b40696f50145d21c
extern "C" fn unset_join_expr(mut p: *mut Expr, i_table_1: i32, nullable: i32)
    -> () {
    unsafe {
        while !(p).is_null() {
            if i_table_1 < 0 ||
                    unsafe { (*p).flags } & 1 as u32 != 0 as u32 &&
                        unsafe { (*p).w.i_join } == i_table_1 {
                unsafe { (*p).flags &= !((1 | 2) as u32) };
                if i_table_1 >= 0 { unsafe { (*p).flags |= 2 as u32 }; }
            }
            if unsafe { (*p).op } as i32 == 168 &&
                        unsafe { (*p).i_table } == i_table_1 &&
                    (nullable == 0) as i32 != 0 {
                unsafe { (*p).flags &= !(2097152 as u32) };
            }
            if unsafe { (*p).op } as i32 == 172 {
                { let _ = 0; };
                { let _ = 0; };
                if !(unsafe { (*p).x.p_list }).is_null() {
                    let mut i: i32 = 0;
                    {
                        i = 0;
                        '__b45: loop {
                            if !(i < unsafe { (*unsafe { (*p).x.p_list }).n_expr }) {
                                break '__b45;
                            }
                            '__c45: loop {
                                unset_join_expr(unsafe {
                                        (*(unsafe { (*unsafe { (*p).x.p_list }).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                    }, i_table_1, nullable);
                                break '__c45;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                }
            }
            unset_join_expr(unsafe { (*p).p_left }, i_table_1, nullable);
            p = unsafe { (*p).p_right };
        }
    }
}

///* Return true if any of the result-set columns in the compound query
///* have incompatible affinities on one or more arms of the compound.
extern "C" fn compound_has_different_affinities(p: &Select) -> i32 {
    unsafe {
        let mut ii: i32 = 0;
        let mut p_list: *const ExprList = core::ptr::null();
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        p_list = (*p).p_e_list;
        {
            ii = 0;
            '__b46: loop {
                if !(ii < unsafe { (*p_list).n_expr }) { break '__b46; }
                '__c46: loop {
                    let mut aff: i8 = 0 as i8;
                    let mut p_sub1: *const Select = core::ptr::null();
                    { let _ = 0; };
                    aff =
                        unsafe {
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*p_list).a.as_ptr() } as
                                                        *mut ExprListItem).offset(ii as isize)).p_expr
                                    } as *const Expr)
                        };
                    {
                        p_sub1 = (*p).p_prior;
                        '__b47: loop {
                            if !(!(p_sub1).is_null()) { break '__b47; }
                            '__c47: loop {
                                { let _ = 0; };
                                { let _ = 0; };
                                { let _ = 0; };
                                if unsafe {
                                                sqlite3_expr_affinity(unsafe {
                                                            (*(unsafe { (*unsafe { (*p_sub1).p_e_list }).a.as_ptr() } as
                                                                            *mut ExprListItem).offset(ii as isize)).p_expr
                                                        } as *const Expr)
                                            } as i32 != aff as i32 {
                                    return 1;
                                }
                                break '__c47;
                            }
                            p_sub1 = unsafe { (*p_sub1).p_prior };
                        }
                    }
                    break '__c46;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}

///* Assign new cursor numbers to each of the items in pSrc. For each
///* new cursor number assigned, set an entry in the aCsrMap[] array
///* to map the old cursor number to the new:
///*
///*     aCsrMap[iOld+1] = iNew;
///*
///* The array is guaranteed by the caller to be large enough for all
///* existing cursor numbers in pSrc.  aCsrMap[0] is the array size.
///*
///* If pSrc contains any sub-selects, call this routine recursively
///* on the FROM clause of each such sub-select, with iExcept set to -1.
extern "C" fn srclist_renumber_cursors(p_parse_1: *mut Parse,
    a_csr_map_1: *mut i32, p_src_1: &mut SrcList, i_except_1: i32) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        {
            { i = 0; p_item = (*p_src_1).a.as_ptr() as *mut SrcItem };
            '__b48: loop {
                if !(i < (*p_src_1).n_src) { break '__b48; }
                '__c48: loop {
                    if i != i_except_1 {
                        let mut p: *const Select = core::ptr::null();
                        { let _ = 0; };
                        if (unsafe { (*p_item).fg.is_recursive() } == 0) as i32 != 0
                                ||
                                unsafe {
                                        *a_csr_map_1.offset((unsafe { (*p_item).i_cursor } + 1) as
                                                    isize)
                                    } == 0 {
                            unsafe {
                                *a_csr_map_1.offset((unsafe { (*p_item).i_cursor } + 1) as
                                                isize) =
                                    {
                                        let __p = unsafe { &mut (*p_parse_1).n_tab };
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    }
                            };
                        }
                        unsafe {
                            (*p_item).i_cursor =
                                unsafe {
                                    *a_csr_map_1.offset((unsafe { (*p_item).i_cursor } + 1) as
                                                isize)
                                }
                        };
                        if unsafe { (*p_item).fg.is_subquery() } != 0 {
                            {
                                p = unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select };
                                '__b49: loop {
                                    if !(!(p).is_null()) { break '__b49; }
                                    '__c49: loop {
                                        srclist_renumber_cursors(p_parse_1, a_csr_map_1,
                                            unsafe { &mut *unsafe { (*p).p_src } }, -1);
                                        break '__c49;
                                    }
                                    p = unsafe { (*p).p_prior };
                                }
                            }
                        }
                    }
                    break '__c48;
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

///* *piCursor is a cursor number.  Change it if it needs to be mapped.
extern "C" fn renumber_cursor_do_mapping(p_walker_1: &Walker,
    pi_cursor_1: &mut i32) -> () {
    unsafe {
        let a_csr_map: *const i32 = (*p_walker_1).u.ai_col as *const i32;
        let i_csr: i32 = *pi_cursor_1;
        if i_csr < unsafe { *a_csr_map.offset(0 as isize) } &&
                unsafe { *a_csr_map.offset((i_csr + 1) as isize) } > 0 {
            *pi_cursor_1 = unsafe { *a_csr_map.offset((i_csr + 1) as isize) };
        }
    }
}

///* Expression walker callback used by renumberCursors() to update
///* Expr objects to match newly assigned cursor numbers.
extern "C" fn renumber_cursors_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let op: i32 = unsafe { (*p_expr_1).op } as i32;
        if op == 168 || op == 179 {
            renumber_cursor_do_mapping(unsafe { &*p_walker_1 },
                unsafe { &mut (*p_expr_1).i_table });
        }
        if unsafe { (*p_expr_1).flags } & 1 as u32 != 0 as u32 {
            renumber_cursor_do_mapping(unsafe { &*p_walker_1 },
                unsafe { &mut (*p_expr_1).w.i_join });
        }
        return 0;
    }
}

///* Assign a new cursor number to each cursor in the FROM clause (Select.pSrc)
///* of the SELECT statement passed as the second argument, and to each
///* cursor in the FROM clause of any FROM clause sub-selects, recursively.
///* Except, do not assign a new cursor number to the iExcept'th element in
///* the FROM clause of (*p). Update all expressions and other references
///* to refer to the new cursor numbers.
///*
///* Argument aCsrMap is an array that may be used for temporary working
///* space. Two guarantees are made by the caller:
///*
///*   * the array is larger than the largest cursor number used within the
///*     select statement passed as an argument, and
///*
///*   * the array entries for all cursor numbers that do *not* appear in
///*     FROM clauses of the select statement as described above are
///*     initialized to zero.
extern "C" fn renumber_cursors(p_parse_1: *mut Parse, p: *mut Select,
    i_except_1: i32, a_csr_map_1: *mut i32) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        srclist_renumber_cursors(p_parse_1, a_csr_map_1,
            unsafe { &mut *unsafe { (*p).p_src } }, i_except_1);
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.u.ai_col = a_csr_map_1;
        w.x_expr_callback = Some(renumber_cursors_cb);
        w.x_select_callback = Some(sqlite3_select_walk_noop);
        unsafe { sqlite3_walk_select(&mut w, p) };
    }
}

/// An instance of the SubstContext object describes an substitution edit
///* to be performed on a parse tree.
///*
///* All references to columns in table iTable are to be replaced by corresponding
///* expressions in pEList.
///*
///* ## About "isOuterJoin":
///*
///* The isOuterJoin column indicates that the replacement will occur into a
///* position in the parent that is NULL-able due to an OUTER JOIN.  Either the
///* target slot in the parent is the right operand of a LEFT JOIN, or one of
///* the left operands of a RIGHT JOIN.  In either case, we need to potentially
///* bypass the substituted expression with OP_IfNullRow.
///*
///* Suppose the original expression is an integer constant. Even though the table
///* has the nullRow flag set, because the expression is an integer constant,
///* it will not be NULLed out.  So instead, we insert an OP_IfNullRow opcode
///* that checks to see if the nullRow flag is set on the table.  If the nullRow
///* flag is set, then the value in the register is set to NULL and the original
///* expression is bypassed.  If the nullRow flag is not set, then the original
///* expression runs to populate the register.
///*
///* Example where this is needed:
///*
///*      CREATE TABLE t1(a INTEGER PRIMARY KEY, b INT);
///*      CREATE TABLE t2(x INT UNIQUE);
///*
///*      SELECT a,b,m,x FROM t1 LEFT JOIN (SELECT 59 AS m,x FROM t2) ON b=x;
///*
///* When the subquery on the right side of the LEFT JOIN is flattened, we
///* have to add OP_IfNullRow in front of the OP_Integer that implements the
///* "m" value of the subquery so that a NULL will be loaded instead of 59
///* when processing a non-matched row of the left.
#[repr(C)]
#[derive(Copy, Clone)]
struct SubstContext {
    p_parse: *mut Parse,
    i_table: i32,
    i_new_table: i32,
    is_outer_join: i32,
    n_sel_depth: i32,
    p_e_list: *mut ExprList,
    p_c_list: *mut ExprList,
}

///* If pSel is not part of a compound SELECT, return a pointer to its
///* expression list. Otherwise, return a pointer to the expression list
///* of the leftmost SELECT in the compound.
extern "C" fn find_leftmost_exprlist(mut p_sel_1: *const Select)
    -> *mut ExprList {
    unsafe {
        while !(unsafe { (*p_sel_1).p_prior }).is_null() {
            p_sel_1 = unsafe { (*p_sel_1).p_prior };
        }
        return unsafe { (*p_sel_1).p_e_list };
    }
}

///* Scan through the expression pExpr.  Replace every reference to
///* a column in table number iTable with a copy of the iColumn-th
///* entry in pEList.  (But leave references to the ROWID column
///* unchanged.)
///*
///* This routine is part of the flattening procedure.  A subquery
///* whose result set is defined by pEList appears as entry in the
///* FROM clause of a SELECT such that the VDBE cursor assigned to that
///* FORM clause entry is iTable.  This routine makes the necessary
///* changes to pExpr so that it refers directly to the source table
///* of the subquery rather the result set of the subquery.
extern "C" fn subst_expr(p_subst_1: *mut SubstContext,
    mut p_expr_1: *mut Expr) -> *mut Expr {
    unsafe {
        if p_expr_1 == core::ptr::null_mut() { return core::ptr::null_mut(); }
        if unsafe { (*p_expr_1).flags } & (1 | 2) as u32 != 0 as u32 &&
                unsafe { (*p_expr_1).w.i_join } ==
                    unsafe { (*p_subst_1).i_table } {
            unsafe {
                (*p_expr_1).w.i_join = unsafe { (*p_subst_1).i_new_table }
            };
        }
        if unsafe { (*p_expr_1).op } as i32 == 168 &&
                    unsafe { (*p_expr_1).i_table } ==
                        unsafe { (*p_subst_1).i_table } &&
                !(unsafe { (*p_expr_1).flags } & 32 as u32 != 0 as u32) as i32
                    != 0 {
            {
                let mut p_new: *mut Expr = core::ptr::null_mut();
                let mut i_column: i32 = 0;
                let mut p_copy: *mut Expr = core::ptr::null_mut();
                let mut if_null_row: Expr = unsafe { core::mem::zeroed() };
                i_column = unsafe { (*p_expr_1).i_column } as i32;
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                p_copy =
                    unsafe {
                        (*(unsafe { (*unsafe { (*p_subst_1).p_e_list }).a.as_ptr() }
                                        as *mut ExprListItem).offset(i_column as isize)).p_expr
                    };
                if unsafe { sqlite3_expr_is_vector(p_copy as *const Expr) } !=
                        0 {
                    unsafe {
                        sqlite3_vector_error_msg(unsafe { (*p_subst_1).p_parse },
                            p_copy)
                    };
                } else {
                    let db: *mut Sqlite3 =
                        unsafe { (*unsafe { (*p_subst_1).p_parse }).db };
                    if unsafe { (*p_subst_1).is_outer_join } != 0 &&
                            (unsafe { (*p_copy).op } as i32 != 168 ||
                                unsafe { (*p_copy).i_table } !=
                                    unsafe { (*p_subst_1).i_new_table }) {
                        unsafe {
                            memset(&raw mut if_null_row as *mut (), 0,
                                core::mem::size_of::<Expr>() as u64)
                        };
                        if_null_row.op = 179 as u8;
                        if_null_row.p_left = p_copy;
                        if_null_row.i_table = unsafe { (*p_subst_1).i_new_table };
                        if_null_row.i_column = -99 as YnVar;
                        if_null_row.flags = 262144 as u32;
                        p_copy = &mut if_null_row;
                    }
                    p_new =
                        unsafe { sqlite3_expr_dup(db, p_copy as *const Expr, 0) };
                    if unsafe { (*db).malloc_failed } != 0 {
                        unsafe { sqlite3_expr_delete(db, p_new) };
                        return p_expr_1;
                    }
                    if unsafe { (*p_subst_1).is_outer_join } != 0 {
                        unsafe { (*p_new).flags |= 2097152 as u32 };
                    }
                    if unsafe { (*p_new).op } as i32 == 171 {
                        unsafe {
                            (*p_new).u.i_value =
                                unsafe { sqlite3_expr_truth_value(p_new as *const Expr) }
                        };
                        unsafe { (*p_new).op = 156 as u8 };
                        unsafe { (*p_new).flags |= 2048 as u32 };
                    }
                    {
                        let p_nat: *mut CollSeq =
                            unsafe {
                                sqlite3_expr_coll_seq(unsafe { (*p_subst_1).p_parse },
                                    p_new as *const Expr)
                            };
                        let p_coll: *mut CollSeq =
                            unsafe {
                                sqlite3_expr_coll_seq(unsafe { (*p_subst_1).p_parse },
                                    unsafe {
                                            (*(unsafe { (*unsafe { (*p_subst_1).p_c_list }).a.as_ptr() }
                                                            as *mut ExprListItem).offset(i_column as isize)).p_expr
                                        } as *const Expr)
                            };
                        if p_nat != p_coll ||
                                unsafe { (*p_new).op } as i32 != 168 &&
                                    unsafe { (*p_new).op } as i32 != 114 {
                            p_new =
                                unsafe {
                                    sqlite3_expr_add_collate_string(unsafe {
                                                (*p_subst_1).p_parse
                                            } as *const Parse, p_new,
                                        if !(p_coll).is_null() {
                                                unsafe { (*p_coll).z_name }
                                            } else { c"BINARY".as_ptr() as *mut i8 } as *const i8)
                                };
                        }
                    }
                    unsafe { (*p_new).flags &= !(512 as u32) };
                    if unsafe { (*p_expr_1).flags } & (1 | 2) as u32 != 0 as u32
                        {
                        sqlite3_set_join_expr(p_new,
                            unsafe { (*p_expr_1).w.i_join },
                            unsafe { (*p_expr_1).flags } & (1 | 2) as u32);
                    }
                    unsafe { sqlite3_expr_delete(db, p_expr_1) };
                    p_expr_1 = p_new;
                }
            }
        } else {
            if unsafe { (*p_expr_1).op } as i32 == 179 &&
                    unsafe { (*p_expr_1).i_table } ==
                        unsafe { (*p_subst_1).i_table } {
                unsafe {
                    (*p_expr_1).i_table = unsafe { (*p_subst_1).i_new_table }
                };
            }
            if unsafe { (*p_expr_1).op } as i32 == 169 &&
                    unsafe { (*p_expr_1).op2 } as i32 >=
                        unsafe { (*p_subst_1).n_sel_depth } {
                {
                    let __p = unsafe { &mut (*p_expr_1).op2 };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
            unsafe {
                (*p_expr_1).p_left =
                    subst_expr(p_subst_1, unsafe { (*p_expr_1).p_left })
            };
            unsafe {
                (*p_expr_1).p_right =
                    subst_expr(p_subst_1, unsafe { (*p_expr_1).p_right })
            };
            if unsafe { (*p_expr_1).flags } & 4096 as u32 != 0 as u32 {
                subst_select(p_subst_1, unsafe { (*p_expr_1).x.p_select }, 1);
            } else {
                subst_expr_list(p_subst_1, unsafe { (*p_expr_1).x.p_list });
            }
            if unsafe { (*p_expr_1).flags } & 16777216 as u32 != 0 as u32 {
                let p_win: *mut Window = unsafe { (*p_expr_1).y.p_win };
                unsafe {
                    (*p_win).p_filter =
                        subst_expr(p_subst_1, unsafe { (*p_win).p_filter })
                };
                subst_expr_list(p_subst_1, unsafe { (*p_win).p_partition });
                subst_expr_list(p_subst_1, unsafe { (*p_win).p_order_by });
            }
        }
        return p_expr_1;
    }
}

/// Forward Declarations
extern "C" fn subst_expr_list(p_subst: *mut SubstContext,
    p_list: *mut ExprList) -> () {
    let mut i: i32 = 0;
    if p_list == core::ptr::null_mut() { return; }
    {
        i = 0;
        '__b51: loop {
            if !(i < unsafe { (*p_list).n_expr }) { break '__b51; }
            '__c51: loop {
                unsafe {
                    (*(unsafe { (*p_list).a.as_ptr() } as
                                        *mut ExprListItem).offset(i as isize)).p_expr =
                        subst_expr(p_subst,
                            unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                *mut ExprListItem).offset(i as isize)).p_expr
                            })
                };
                break '__c51;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn subst_select(p_subst: *mut SubstContext, mut p: *mut Select,
    do_prior: i32) -> () {
    unsafe {
        let mut p_src: *mut SrcList = core::ptr::null_mut();
        let mut p_item: *const SrcItem = core::ptr::null();
        let mut i: i32 = 0;
        if (p).is_null() as i32 != 0 { return; }
        {
            let __p = unsafe { &mut (*p_subst).n_sel_depth };
            let __t = *__p;
            *__p += 1;
            __t
        };
        '__b52: loop {
            '__c52: loop {
                subst_expr_list(p_subst, unsafe { (*p).p_e_list });
                subst_expr_list(p_subst, unsafe { (*p).p_group_by });
                subst_expr_list(p_subst, unsafe { (*p).p_order_by });
                unsafe {
                    (*p).p_having =
                        subst_expr(p_subst, unsafe { (*p).p_having })
                };
                unsafe {
                    (*p).p_where = subst_expr(p_subst, unsafe { (*p).p_where })
                };
                p_src = unsafe { (*p).p_src };
                { let _ = 0; };
                {
                    {
                        i = unsafe { (*p_src).n_src };
                        p_item = unsafe { (*p_src).a.as_ptr() } as *mut SrcItem
                    };
                    '__b53: loop {
                        if !(i > 0) { break '__b53; }
                        '__c53: loop {
                            if unsafe { (*p_item).fg.is_subquery() } != 0 {
                                subst_select(p_subst,
                                    unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select }, 1);
                            }
                            if unsafe { (*p_item).fg.is_tab_func() } != 0 {
                                subst_expr_list(p_subst,
                                    unsafe { (*p_item).u1.p_func_arg });
                            }
                            break '__c53;
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
                break '__c52;
            }
            if !(do_prior != 0 &&
                            { p = unsafe { (*p).p_prior }; p } != core::ptr::null_mut())
                {
                break '__b52;
            }
        }
        {
            let __p = unsafe { &mut (*p_subst).n_sel_depth };
            let __t = *__p;
            *__p -= 1;
            __t
        };
    }
}

///* pSelect is a SELECT statement and pSrcItem is one item in the FROM
///* clause of that SELECT.
///*
///* This routine scans the entire SELECT statement and recomputes the
///* pSrcItem->colUsed mask.
extern "C" fn recompute_columns_used_expr(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        if unsafe { (*p_expr_1).op } as i32 != 168 { return 0; }
        p_item = unsafe { (*p_walker_1).u.p_src_item };
        if unsafe { (*p_item).i_cursor } != unsafe { (*p_expr_1).i_table } {
            return 0;
        }
        if (unsafe { (*p_expr_1).i_column } as i32) < 0 { return 0; }
        unsafe {
            (*p_item).col_used |= unsafe { sqlite3_expr_col_used(p_expr_1) }
        };
        return 0;
    }
}

extern "C" fn recompute_columns_used(p_select_1: *mut Select,
    p_src_item_1: *mut SrcItem) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        if unsafe { (*p_src_item_1).p_s_tab } == core::ptr::null_mut() {
            return;
        }
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.x_expr_callback = Some(recompute_columns_used_expr);
        w.x_select_callback = Some(sqlite3_select_walk_noop);
        w.u.p_src_item = p_src_item_1;
        unsafe { (*p_src_item_1).col_used = 0 as Bitmask };
        unsafe { sqlite3_walk_select(&mut w, p_select_1) };
    }
}

///* Delete all the content of a Select structure.  Deallocate the structure
///* itself depending on the value of bFree
///*
///* If bFree==1, call sqlite3DbFree() on the p object.
///* If bFree==0, Leave the first Select object unfreed
extern "C" fn clear_select(db: *mut Sqlite3, mut p: *mut Select,
    mut b_free_1: i32) -> () {
    unsafe {
        { let _ = 0; };
        while !(p).is_null() {
            let p_prior: *mut Select = unsafe { (*p).p_prior };
            unsafe { sqlite3_expr_list_delete(db, unsafe { (*p).p_e_list }) };
            unsafe { sqlite3_src_list_delete(db, unsafe { (*p).p_src }) };
            unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_where }) };
            unsafe {
                sqlite3_expr_list_delete(db, unsafe { (*p).p_group_by })
            };
            unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_having }) };
            unsafe {
                sqlite3_expr_list_delete(db, unsafe { (*p).p_order_by })
            };
            unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_limit }) };
            if !(unsafe { (*p).p_with }).is_null() {
                unsafe { sqlite3_with_delete(db, unsafe { (*p).p_with }) };
            }
            if !(unsafe { (*p).p_win_defn }).is_null() {
                unsafe {
                    sqlite3_window_list_delete(db, unsafe { (*p).p_win_defn })
                };
            }
            while !(unsafe { (*p).p_win }).is_null() {
                { let _ = 0; };
                unsafe {
                    sqlite3_window_unlink_from_select(unsafe { (*p).p_win })
                };
            }
            if b_free_1 != 0 {
                unsafe { sqlite3_db_nn_free_nn(db, p as *mut ()) };
            }
            p = p_prior;
            b_free_1 = 1;
        }
    }
}

///* Delete the given Select structure and all of its substructures.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_delete(db: *mut Sqlite3, p: *mut Select)
    -> () {
    if !(p).is_null() { clear_select(db, p, 1); }
}

///* This routine attempts to flatten subqueries as a performance optimization.
///* This routine returns 1 if it makes changes and 0 if no flattening occurs.
///*
///* To understand the concept of flattening, consider the following
///* query:
///*
///*     SELECT a FROM (SELECT x+y AS a FROM t1 WHERE z<100) WHERE a>5
///*
///* The default way of implementing this query is to execute the
///* subquery first and store the results in a temporary table, then
///* run the outer query on that temporary table.  This requires two
///* passes over the data.  Furthermore, because the temporary table
///* has no indices, the WHERE clause on the outer query cannot be
///* optimized.
///*
///* This routine attempts to rewrite queries such as the above into
///* a single flat select, like this:
///*
///*     SELECT x+y AS a FROM t1 WHERE z<100 AND a>5
///*
///* The code generated for this simplification gives the same result
///* but only has to scan the data once.  And because indices might
///* exist on the table t1, a complete scan of the data might be
///* avoided.
///*
///* Flattening is subject to the following constraints:
///*
///*  (**)  We no longer attempt to flatten aggregate subqueries. Was:
///*        The subquery and the outer query cannot both be aggregates.
///*
///*  (**)  We no longer attempt to flatten aggregate subqueries. Was:
///*        (2) If the subquery is an aggregate then
///*        (2a) the outer query must not be a join and
///*        (2b) the outer query must not use subqueries
///*             other than the one FROM-clause subquery that is a candidate
///*             for flattening.  (This is due to ticket [2f7170d73bf9abf80]
///*             from 2015-02-09.)
///*
///*   (3)  If the subquery is the right operand of a LEFT JOIN then
///*        (3a) the subquery may not be a join
///*        (**) Was (3b): "the FROM clause of the subquery may not contain
///*             a virtual table"
///*        (**) Was: "The outer query may not have a GROUP BY." This case
///*             is now managed correctly
///*        (3d) the outer query may not be DISTINCT.
///*        See also (26) for restrictions on RIGHT JOIN.
///*
///*   (4)  The subquery can not be DISTINCT.
///*
///*  (**)  At one point restrictions (4) and (5) defined a subset of DISTINCT
///*        sub-queries that were excluded from this optimization. Restriction
///*        (4) has since been expanded to exclude all DISTINCT subqueries.
///*
///*  (**)  We no longer attempt to flatten aggregate subqueries.  Was:
///*        If the subquery is aggregate, the outer query may not be DISTINCT.
///*
///*   (7)  The subquery must have a FROM clause.  TODO:  For subqueries without
///*        A FROM clause, consider adding a FROM clause with the special
///*        table sqlite_once that consists of a single row containing a
///*        single NULL.
///*
///*   (8)  If the subquery uses LIMIT then the outer query may not be a join.
///*
///*   (9)  If the subquery uses LIMIT then the outer query may not be aggregate.
///*
///*  (**)  Restriction (10) was removed from the code on 2005-02-05 but we
///*        accidentally carried the comment forward until 2014-09-15.  Original
///*        constraint: "If the subquery is aggregate then the outer query
///*        may not use LIMIT."
///*
///*  (11)  The subquery and the outer query may not both have ORDER BY clauses.
///*
///*  (**)  Not implemented.  Subsumed into restriction (3).  Was previously
///*        a separate restriction deriving from ticket #350.
///*
///*  (13)  The subquery and outer query may not both use LIMIT.
///*
///*  (14)  The subquery may not use OFFSET.
///*
///*  (15)  If the outer query is part of a compound select, then the
///*        subquery may not use LIMIT.
///*        (See ticket #2339 and ticket [02a8e81d44]).
///*
///*  (16)  If the outer query is aggregate, then the subquery may not
///*        use ORDER BY.  (Ticket #2942)  This used to not matter
///*        until we introduced the group_concat() function. 
///*
///*  (17)  If the subquery is a compound select, then
///*        (17a) all compound operators must be a UNION ALL, and
///*        (17b) no terms within the subquery compound may be aggregate
///*              or DISTINCT, and
///*        (17c) every term within the subquery compound must have a FROM clause
///*        (17d) the outer query may not be
///*              (17d1) aggregate, or
///*              (17d2) DISTINCT
///*        (17e) the subquery may not contain window functions, and
///*        (17f) the subquery must not be the RHS of a LEFT JOIN.
///*        (17g) either the subquery is the first element of the outer
///*              query or there are no RIGHT or FULL JOINs in any arm
///*              of the subquery.  (This is a duplicate of condition (27b).)
///*        (17h) The corresponding result set expressions in all arms of the
///*              compound must have the same affinity.
///*
///*        The parent and sub-query may contain WHERE clauses. Subject to
///*        rules (11), (13) and (14), they may also contain ORDER BY,
///*        LIMIT and OFFSET clauses.  The subquery cannot use any compound
///*        operator other than UNION ALL because all the other compound
///*        operators have an implied DISTINCT which is disallowed by
///*        restriction (4).
///*
///*        Also, each component of the sub-query must return the same number
///*        of result columns. This is actually a requirement for any compound
///*        SELECT statement, but all the code here does is make sure that no
///*        such (illegal) sub-query is flattened. The caller will detect the
///*        syntax error and return a detailed message.
///*
///*  (18)  If the sub-query is a compound select, then all terms of the
///*        ORDER BY clause of the parent must be copies of a term returned
///*        by the parent query.
///*
///*  (19)  If the subquery uses LIMIT then the outer query may not
///*        have a WHERE clause.
///*
///*  (20)  If the sub-query is a compound select, then it must not use
///*        an ORDER BY clause.  Ticket #3773.  We could relax this constraint
///*        somewhat by saying that the terms of the ORDER BY clause must
///*        appear as unmodified result columns in the outer query.  But we
///*        have other optimizations in mind to deal with that case.
///*
///*  (21)  If the subquery uses LIMIT then the outer query may not be
///*        DISTINCT.  (See ticket [752e1646fc]).
///*
///*  (22)  The subquery may not be a recursive CTE.
///*
///*  (23)  If the outer query is a recursive CTE, then the sub-query may not be
///*        a compound query.  This restriction is because transforming the
///*        parent to a compound query confuses the code that handles
///*        recursive queries in multiSelect().
///*
///*  (**)  We no longer attempt to flatten aggregate subqueries.  Was:
///*        The subquery may not be an aggregate that uses the built-in min() or
///*        or max() functions.  (Without this restriction, a query like:
///*        "SELECT x FROM (SELECT max(y), x FROM t1)" would not necessarily
///*        return the value X for which Y was maximal.)
///*
///*  (25)  If either the subquery or the parent query contains a window
///*        function in the select list or ORDER BY clause, flattening
///*        is not attempted.
///*
///*  (26)  The subquery may not be the right operand of a RIGHT JOIN.
///*        See also (3) for restrictions on LEFT JOIN.
///*
///*  (27)  The subquery may not contain a FULL or RIGHT JOIN unless it
///*        is the first element of the parent query.  Two subcases:
///*        (27a) the subquery is not a compound query.
///*        (27b) the subquery is a compound query and the RIGHT JOIN occurs
///*              in any arm of the compound query.  (See also (17g).)
///*
///*  (28)  The subquery is not a MATERIALIZED CTE.  (This is handled
///*        in the caller before ever reaching this routine.)
///*
///*
///* In this routine, the "p" parameter is a pointer to the outer query.
///* The subquery is p->pSrc->a[iFrom].  isAgg is true if the outer query
///* uses aggregates.
///*
///* If flattening is not attempted, this routine is a no-op and returns 0.
///* If flattening is attempted this routine returns 1.
///*
///* All of the expression analysis must occur on both the outer query and
///* the subquery before this routine runs.
#[allow(unused_doc_comments)]
extern "C" fn flatten_subquery(p_parse_1: *mut Parse, p: *mut Select,
    i_from_1: i32, is_agg_1: i32) -> i32 {
    unsafe {
        let z_saved_auth_context: *const i8 =
            unsafe { (*p_parse_1).z_auth_context };
        let mut p_parent: *mut Select = core::ptr::null_mut();
        /// Current UNION ALL term of the other query
        let mut p_sub: *mut Select = core::ptr::null_mut();
        /// The inner query or "subquery"
        let mut p_sub1: *mut Select = core::ptr::null_mut();
        /// Pointer to the rightmost select in sub-query
        let mut p_src: *mut SrcList = core::ptr::null_mut();
        /// The FROM clause of the outer query
        let mut p_sub_src: *mut SrcList = core::ptr::null_mut();
        /// The FROM clause of the subquery
        let mut i_parent: i32 = 0;
        /// VDBE cursor number of the pSub result set temp table
        let mut i_new_parent: i32 = -1;
        /// Replacement table for iParent
        let mut is_outer_join: i32 = 0;
        /// True if pSub is the right side of a LEFT JOIN
        let mut i: i32 = 0;
        /// Loop counter
        let mut p_where: *mut Expr = core::ptr::null_mut();
        /// The WHERE clause
        let mut p_subitem: *mut SrcItem = core::ptr::null_mut();
        /// The subquery
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let mut w: Walker = unsafe { core::mem::zeroed() };
        /// Walker to persist agginfo data
        let mut a_csr_map: *mut i32 = core::ptr::null_mut();

        /// Check to see if flattening is permitted.  Return 0 if not.
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*db).db_opt_flags } & 1 as u32 != 0 as u32 { return 0; }
        p_src = unsafe { (*p).p_src };
        { let _ = 0; };
        p_subitem =
            unsafe {
                &mut *(unsafe { (*p_src).a.as_ptr() } as
                                *mut SrcItem).offset(i_from_1 as isize)
            };
        i_parent = unsafe { (*p_subitem).i_cursor };
        { let _ = 0; };
        p_sub = unsafe { (*unsafe { (*p_subitem).u4.p_subq }).p_select };
        { let _ = 0; };
        if !(unsafe { (*p).p_win }).is_null() ||
                !(unsafe { (*p_sub).p_win }).is_null() {
            return 0;
        }

        /// Restriction (25)
        (p_sub_src = unsafe { (*p_sub).p_src });
        { let _ = 0; };
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                !(unsafe { (*p).p_limit }).is_null() {
            return 0;
        }
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                !(unsafe { (*unsafe { (*p_sub).p_limit }).p_right }).is_null()
            {
            return 0;
        }
        if unsafe { (*p).sel_flags } & 256 as u32 != 0 as u32 &&
                !(unsafe { (*p_sub).p_limit }).is_null() {
            return 0;
        }
        if unsafe { (*p_sub_src).n_src } == 0 { return 0; }
        if unsafe { (*p_sub).sel_flags } & 1 as u32 != 0 { return 0; }
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                (unsafe { (*p_src).n_src } > 1 || is_agg_1 != 0) {
            return 0;
        }
        if !(unsafe { (*p).p_order_by }).is_null() &&
                !(unsafe { (*p_sub).p_order_by }).is_null() {
            return 0;
        }
        if is_agg_1 != 0 && !(unsafe { (*p_sub).p_order_by }).is_null() {
            return 0;
        }
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                !(unsafe { (*p).p_where }).is_null() {
            return 0;
        }
        if !(unsafe { (*p_sub).p_limit }).is_null() &&
                unsafe { (*p).sel_flags } & 1 as u32 != 0 as u32 {
            return 0;
        }
        if unsafe { (*p_sub).sel_flags } & 8192 as u32 != 0 { return 0; }
        if unsafe { (*p_subitem).fg.jointype } as i32 & (32 | 64) != 0 {
            if unsafe { (*p_sub_src).n_src } > 1 ||
                        unsafe { (*p).sel_flags } & 1 as u32 != 0 as u32 ||
                    unsafe { (*p_subitem).fg.jointype } as i32 & 16 != 0 {
                return 0;
            }
            is_outer_join = 1;
        }
        { let _ = 0; };
        if i_from_1 > 0 &&
                unsafe {
                                (*(unsafe { (*p_sub_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).fg.jointype
                            } as i32 & 64 != 0 {
            return 0;
        }

        /// Condition (28) is blocked by the caller
        { let _ = 0; };
        if !(unsafe { (*p_sub).p_prior }).is_null() {
            let mut ii: i32 = 0;
            if !(unsafe { (*p_sub).p_order_by }).is_null() { return 0; }
            if is_agg_1 != 0 ||
                        unsafe { (*p).sel_flags } & 1 as u32 != 0 as u32 ||
                    is_outer_join > 0 {
                return 0;
            }
            {
                p_sub1 = p_sub;
                '__b56: loop {
                    if !(!(p_sub1).is_null()) { break '__b56; }
                    '__c56: loop {
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        if unsafe { (*p_sub1).sel_flags } & (1 | 8) as u32 !=
                                            0 as u32 ||
                                        !(unsafe { (*p_sub1).p_prior }).is_null() &&
                                            unsafe { (*p_sub1).op } as i32 != 136 ||
                                    unsafe { (*unsafe { (*p_sub1).p_src }).n_src } < 1 ||
                                !(unsafe { (*p_sub1).p_win }).is_null() {
                            return 0;
                        }
                        if i_from_1 > 0 &&
                                unsafe {
                                                (*(unsafe { (*unsafe { (*p_sub1).p_src }).a.as_ptr() } as
                                                                    *mut SrcItem).offset(0 as isize)).fg.jointype
                                            } as i32 & 64 != 0 {

                            /// Without this restriction, the JT_LTORJ flag would end up being
                            ///* omitted on left-hand tables of the right join that is being
                            ///* flattened.
                            return 0;
                        }
                        break '__c56;
                    }
                    p_sub1 = unsafe { (*p_sub1).p_prior };
                }
            }
            if !(unsafe { (*p).p_order_by }).is_null() {
                {
                    ii = 0;
                    '__b57: loop {
                        if !(ii < unsafe { (*unsafe { (*p).p_order_by }).n_expr }) {
                            break '__b57;
                        }
                        '__c57: loop {
                            if unsafe {
                                            (*(unsafe { (*unsafe { (*p).p_order_by }).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                        } as i32 == 0 {
                                return 0;
                            }
                            break '__c57;
                        }
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            if unsafe { (*p).sel_flags } & 8192 as u32 != 0 { return 0; }
            if compound_has_different_affinities(unsafe { &*p_sub }) != 0 {
                return 0;
            }
            if unsafe { (*p_src).n_src } > 1 {
                if unsafe { (*p_parse_1).n_select } > 500 { return 0; }
                if unsafe { (*db).db_opt_flags } & 8388608 as u32 != 0 as u32
                    {
                    return 0;
                }
                a_csr_map =
                    unsafe {
                            sqlite3_db_malloc_zero(db,
                                (unsafe { (*p_parse_1).n_tab } as i64 + 1 as i64) as u64 *
                                    core::mem::size_of::<i32>() as u64)
                        } as *mut i32;
                if !(a_csr_map).is_null() {
                    unsafe {
                        *a_csr_map.offset(0 as isize) =
                            unsafe { (*p_parse_1).n_tab }
                    };
                }
            }
        }

        /// Authorize the subquery
        unsafe {
            (*p_parse_1).z_auth_context =
                unsafe { (*p_subitem).z_name } as *const i8
        };
        unsafe {
            sqlite3_auth_check(p_parse_1, 21, core::ptr::null(),
                core::ptr::null(), core::ptr::null())
        };
        unsafe { (*p_parse_1).z_auth_context = z_saved_auth_context };
        if unsafe { (*p_subitem).fg.is_subquery() } != 0 {
            p_sub1 = unsafe { sqlite3_subquery_detach(db, p_subitem) };
        } else { p_sub1 = core::ptr::null_mut(); }
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            sqlite3_db_free(db, unsafe { (*p_subitem).z_name } as *mut ())
        };
        unsafe {
            sqlite3_db_free(db, unsafe { (*p_subitem).z_alias } as *mut ())
        };
        unsafe { (*p_subitem).z_name = core::ptr::null_mut() };
        unsafe { (*p_subitem).z_alias = core::ptr::null_mut() };
        { let _ = 0; };
        {
            p_sub = unsafe { (*p_sub).p_prior };
            '__b58: loop {
                if !(!(p_sub).is_null()) { break '__b58; }
                '__c58: loop {
                    let mut p_new: *mut Select = core::ptr::null_mut();
                    let p_order_by: *mut ExprList = unsafe { (*p).p_order_by };
                    let p_limit: *mut Expr = unsafe { (*p).p_limit };
                    let p_prior: *mut Select = unsafe { (*p).p_prior };
                    let p_item_tab: *mut Table =
                        unsafe { (*p_subitem).p_s_tab };
                    unsafe { (*p_subitem).p_s_tab = core::ptr::null_mut() };
                    unsafe { (*p).p_order_by = core::ptr::null_mut() };
                    unsafe { (*p).p_prior = core::ptr::null_mut() };
                    unsafe { (*p).p_limit = core::ptr::null_mut() };
                    p_new =
                        unsafe { sqlite3_select_dup(db, p as *const Select, 0) };
                    unsafe { (*p).p_limit = p_limit };
                    unsafe { (*p).p_order_by = p_order_by };
                    unsafe { (*p).op = 136 as u8 };
                    unsafe { (*p_subitem).p_s_tab = p_item_tab };
                    if p_new == core::ptr::null_mut() {
                        unsafe { (*p).p_prior = p_prior };
                    } else {
                        unsafe {
                            (*p_new).sel_id =
                                {
                                        let __p = unsafe { &mut (*p_parse_1).n_select };
                                        *__p += 1;
                                        *__p
                                    } as u32
                        };
                        if !(a_csr_map).is_null() &&
                                unsafe { (*db).malloc_failed } as i32 == 0 {
                            renumber_cursors(p_parse_1, p_new, i_from_1, a_csr_map);
                        }
                        unsafe { (*p_new).p_prior = p_prior };
                        if !(p_prior).is_null() {
                            unsafe { (*p_prior).p_next = p_new };
                        }
                        unsafe { (*p_new).p_next = p };
                        unsafe { (*p).p_prior = p_new };
                    }
                    { let _ = 0; };
                    break '__c58;
                }
                p_sub = unsafe { (*p_sub).p_prior };
            }
        }
        unsafe { sqlite3_db_free(db, a_csr_map as *mut ()) };
        if unsafe { (*db).malloc_failed } != 0 {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            unsafe {
                sqlite3_src_item_attach_subquery(p_parse_1, p_subitem, p_sub1,
                    0)
            };
            return 1;
        }
        if unsafe { (*p_subitem).p_s_tab } != core::ptr::null_mut() {
            let p_tab_to_del: *mut Table = unsafe { (*p_subitem).p_s_tab };
            if unsafe { (*p_tab_to_del).n_tab_ref } == 1 as u32 {
                let p_toplevel: *mut Parse =
                    if !(unsafe { (*p_parse_1).p_toplevel }).is_null() {
                        unsafe { (*p_parse_1).p_toplevel }
                    } else { p_parse_1 };
                unsafe {
                    sqlite3_parser_add_cleanup(p_toplevel,
                        Some(sqlite3_delete_table_generic), p_tab_to_del as *mut ())
                };
            } else {
                {
                    let __p = unsafe { &mut (*p_tab_to_del).n_tab_ref };
                    let __t = *__p;
                    *__p -= 1;
                    __t
                };
            }
            unsafe { (*p_subitem).p_s_tab = core::ptr::null_mut() };
        }

        /// The following loop runs once for each term in a compound-subquery
        ///* flattening (as described above).  If we are doing a different kind
        ///* of flattening - a flattening other than a compound-subquery flattening -
        ///* then this loop only runs once.
        ///*
        ///* This loop moves all of the FROM elements of the subquery into the
        ///* the FROM clause of the outer query.  Before doing this, remember
        ///* the cursor number for the original outer query FROM element in
        ///* iParent.  The iParent cursor will never be used.  Subsequent code
        ///* will scan expressions looking for iParent references and replace
        ///* those references with expressions that resolve to the subquery FROM
        ///* elements we are now copying in.
        (p_sub = p_sub1);
        {
            p_parent = p;
            '__b59: loop {
                if !(!(p_parent).is_null()) { break '__b59; }
                '__c59: loop {
                    let mut n_sub_src: i32 = 0;
                    let jointype: u8 = unsafe { (*p_subitem).fg.jointype };
                    { let _ = 0; };
                    p_sub_src = unsafe { (*p_sub).p_src };

                    /// FROM clause of subquery
                    (n_sub_src = unsafe { (*p_sub_src).n_src });

                    /// Number of terms in subquery FROM clause
                    (p_src = unsafe { (*p_parent).p_src });
                    if n_sub_src > 1 {
                        p_src =
                            unsafe {
                                sqlite3_src_list_enlarge(p_parse_1, p_src, n_sub_src - 1,
                                    i_from_1 + 1)
                            };
                        if p_src == core::ptr::null_mut() { break '__b59; }
                        unsafe { (*p_parent).p_src = p_src };
                        p_subitem =
                            unsafe {
                                &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(i_from_1 as isize)
                            };
                    }

                    /// Transfer the FROM clause terms from the subquery into the
                    ///* outer query.
                    (i_new_parent =
                        unsafe {
                            (*(unsafe { (*p_sub_src).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as isize)).i_cursor
                        });
                    {
                        i = 0;
                        '__b60: loop {
                            if !(i < n_sub_src) { break '__b60; }
                            '__c60: loop {
                                let p_item: *mut SrcItem =
                                    unsafe {
                                        &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset((i + i_from_1) as isize)
                                    };
                                { let _ = 0; };
                                { let _ = 0; };
                                if unsafe { (*p_item).fg.is_using() } != 0 {
                                    unsafe {
                                        sqlite3_id_list_delete(db, unsafe { (*p_item).u3.p_using })
                                    };
                                }
                                unsafe {
                                    *p_item =
                                        unsafe {
                                            *(unsafe { (*p_sub_src).a.as_ptr() } as
                                                        *mut SrcItem).offset(i as isize)
                                        }
                                };
                                unsafe {
                                    (*p_item).fg.jointype |= (jointype as i32 & 64) as u8
                                };
                                unsafe {
                                    memset(unsafe {
                                                &raw mut *(unsafe { (*p_sub_src).a.as_ptr() } as
                                                                *mut SrcItem).offset(i as isize)
                                            } as *mut (), 0, core::mem::size_of::<SrcItem>() as u64)
                                };
                                break '__c60;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    unsafe {
                        (*p_subitem).fg.jointype |= jointype as i32 as u8
                    };
                    if !(unsafe { (*p_sub).p_order_by }).is_null() {
                        /// At this point, any non-zero iOrderByCol values indicate that the
                        ///* ORDER BY column expression is identical to the iOrderByCol'th
                        ///* expression returned by SELECT statement pSub. Since these values
                        ///* do not necessarily correspond to columns in SELECT statement pParent,
                        ///* zero them before transferring the ORDER BY clause.
                        ///*
                        ///* Not doing this may cause an error if a subsequent call to this
                        ///* function attempts to flatten a compound sub-query into pParent.
                        ///* See ticket [d11a6e908f].
                        let p_order_by_1: *mut ExprList =
                            unsafe { (*p_sub).p_order_by };
                        {
                            i = 0;
                            '__b61: loop {
                                if !(i < unsafe { (*p_order_by_1).n_expr }) {
                                    break '__b61;
                                }
                                '__c61: loop {
                                    unsafe {
                                        (*(unsafe { (*p_order_by_1).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col =
                                            0 as u16
                                    };
                                    break '__c61;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        { let _ = 0; };
                        unsafe { (*p_parent).p_order_by = p_order_by_1 };
                        unsafe { (*p_sub).p_order_by = core::ptr::null_mut() };
                    }
                    p_where = unsafe { (*p_sub).p_where };
                    unsafe { (*p_sub).p_where = core::ptr::null_mut() };
                    if is_outer_join > 0 {
                        { let _ = 0; };
                        sqlite3_set_join_expr(p_where, i_new_parent, 1 as u32);
                    }
                    if !(p_where).is_null() {
                        if !(unsafe { (*p_parent).p_where }).is_null() {
                            unsafe {
                                (*p_parent).p_where =
                                    unsafe {
                                        sqlite3_p_expr(p_parse_1, 44, p_where,
                                            unsafe { (*p_parent).p_where })
                                    }
                            };
                        } else { unsafe { (*p_parent).p_where = p_where }; }
                    }
                    if unsafe { (*db).malloc_failed } as i32 == 0 {
                        let mut x: SubstContext = unsafe { core::mem::zeroed() };
                        x.p_parse = p_parse_1;
                        x.i_table = i_parent;
                        x.i_new_table = i_new_parent;
                        x.is_outer_join = is_outer_join;
                        x.n_sel_depth = 0;
                        x.p_e_list = unsafe { (*p_sub).p_e_list };
                        x.p_c_list = find_leftmost_exprlist(p_sub as *const Select);
                        subst_select(&mut x, p_parent, 0);
                    }

                    /// The flattened query is a compound if either the inner or the
                    ///* outer query is a compound.
                    unsafe {
                        (*p_parent).sel_flags |=
                            unsafe { (*p_sub).sel_flags } & 256 as u32
                    };
                    { let _ = 0; };
                    if !(unsafe { (*p_sub).p_limit }).is_null() {
                        unsafe {
                            (*p_parent).p_limit = unsafe { (*p_sub).p_limit }
                        };
                        unsafe { (*p_sub).p_limit = core::ptr::null_mut() };
                    }
                    {
                        i = 0;
                        '__b62: loop {
                            if !(i < n_sub_src) { break '__b62; }
                            '__c62: loop {
                                recompute_columns_used(p_parent,
                                    unsafe {
                                        &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset((i + i_from_1) as isize)
                                    });
                                break '__c62;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__c59;
                }
                {
                    p_parent = unsafe { (*p_parent).p_prior };
                    p_sub = unsafe { (*p_sub).p_prior }
                };
            }
        }

        /// Finally, delete what is left of the subquery and return success.
        unsafe { sqlite3_agg_info_persist_walker_init(&mut w, p_parse_1) };
        unsafe { sqlite3_walk_select(&mut w, p_sub1) };
        sqlite3_select_delete(db, p_sub1);
        return 1;
    }
}

///* Add code to implement the OFFSET
extern "C" fn code_offset(v: *mut Vdbe, i_offset_1: i32, i_continue_1: i32)
    -> () {
    if i_offset_1 > 0 {
        unsafe { sqlite3_vdbe_add_op3(v, 61, i_offset_1, i_continue_1, 1) };
    }
}

///* This routine does the work of loading query data into an array of
///* registers so that it can be added to the sorter.
extern "C" fn inner_loop_load_row(p_parse_1: *mut Parse, p_select_1: &Select,
    p_info_1: &RowLoadInfo) -> () {
    unsafe {
        unsafe {
            sqlite3_expr_code_expr_list(p_parse_1, (*p_select_1).p_e_list,
                (*p_info_1).reg_result, 0, (*p_info_1).ecel_flags)
        };
    }
}

///* Add code that will check to make sure the array of registers starting at
///* iMem form a distinct entry. This is used by both "SELECT DISTINCT ..." and
///* distinct aggregates ("SELECT count(DISTINCT <expr>) ..."). Three strategies
///* are available. Which is used depends on the value of parameter eTnctType,
///* as follows:
///*
///*   WHERE_DISTINCT_UNORDERED/WHERE_DISTINCT_NOOP:
///*     Build an ephemeral table that contains all entries seen before and
///*     skip entries which have been seen before.
///*
///*     Parameter iTab is the cursor number of an ephemeral table that must
///*     be opened before the VM code generated by this routine is executed.
///*     The ephemeral cursor table is queried for a record identical to the
///*     record formed by the current array of registers. If one is found,
///*     jump to VM address addrRepeat. Otherwise, insert a new record into
///*     the ephemeral cursor and proceed.
///*
///*     The returned value in this case is a copy of parameter iTab.
///*
///*   WHERE_DISTINCT_ORDERED:
///*     In this case rows are being delivered sorted order. The ephemeral
///*     table is not required. Instead, the current set of values
///*     is compared against previous row. If they match, the new row
///*     is not distinct and control jumps to VM address addrRepeat. Otherwise,
///*     the VM program proceeds with processing the new row.
///*
///*     The returned value in this case is the register number of the first
///*     in an array of registers used to store the previous result row so that
///*     it can be compared to the next. The caller must ensure that this
///*     register is initialized to NULL.  (The fixDistinctOpenEph() routine
///*     will take care of this initialization.)
///*
///*   WHERE_DISTINCT_UNIQUE:
///*     In this case it has already been determined that the rows are distinct.
///*     No special action is required. The return value is zero.
///*
///* Parameter pEList is the list of expressions used to generated the
///* contents of each row. It is used by this routine to determine (a)
///* how many elements there are in the array of registers and (b) the
///* collation sequences that should be used for the comparisons if
///* eTnctType is WHERE_DISTINCT_ORDERED.
#[allow(unused_doc_comments)]
extern "C" fn code_distinct(p_parse_1: *mut Parse, e_tnct_type_1: i32,
    i_tab_1: i32, addr_repeat_1: i32, p_e_list_1: &ExprList, reg_elem_1: i32)
    -> i32 {
    let mut i_ret: i32 = 0;
    let n_result_col: i32 = (*p_e_list_1).n_expr;
    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
    '__s63:
        {
        match e_tnct_type_1 {
            2 => {
                {
                    let mut i: i32 = 0;
                    let mut i_jump: i32 = 0;
                    /// Jump destination
                    let mut reg_prev: i32 = 0;

                    /// Previous row content
                    /// Allocate space for the previous row
                    (i_ret =
                        { reg_prev = unsafe { (*p_parse_1).n_mem } + 1; reg_prev });
                    unsafe { (*p_parse_1).n_mem += n_result_col };
                    i_jump =
                        unsafe { sqlite3_vdbe_current_addr(v) } + n_result_col;
                    {
                        i = 0;
                        '__b64: loop {
                            if !(i < n_result_col) { break '__b64; }
                            '__c64: loop {
                                let p_coll: *const CollSeq =
                                    unsafe {
                                            sqlite3_expr_coll_seq(p_parse_1,
                                                unsafe {
                                                        (*((*p_e_list_1).a.as_ptr() as
                                                                        *mut ExprListItem).offset(i as isize)).p_expr
                                                    } as *const Expr)
                                        } as *const CollSeq;
                                if i < n_result_col - 1 {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 53, reg_elem_1 + i, i_jump,
                                            reg_prev + i)
                                    };
                                } else {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 54, reg_elem_1 + i, addr_repeat_1,
                                            reg_prev + i)
                                    };
                                }
                                unsafe {
                                    sqlite3_vdbe_change_p4(v, -1, p_coll as *const i8, -2)
                                };
                                unsafe { sqlite3_vdbe_change_p5(v, 128 as u16) };
                                break '__c64;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    { let _ = 0; };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 82, reg_elem_1, reg_prev,
                            n_result_col - 1)
                    };
                    break '__s63;
                }
                {

                    /// nothing to do
                    break '__s63;
                }
                {
                    let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 29, i_tab_1, addr_repeat_1,
                            reg_elem_1, n_result_col)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 99, reg_elem_1, n_result_col, r1)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 140, i_tab_1, r1, reg_elem_1,
                            n_result_col)
                    };
                    unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                    unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                    i_ret = i_tab_1;
                    break '__s63;
                }
            }
            1 => {
                {

                    /// nothing to do
                    break '__s63;
                }
                {
                    let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 29, i_tab_1, addr_repeat_1,
                            reg_elem_1, n_result_col)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 99, reg_elem_1, n_result_col, r1)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 140, i_tab_1, r1, reg_elem_1,
                            n_result_col)
                    };
                    unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                    unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                    i_ret = i_tab_1;
                    break '__s63;
                }
            }
            _ => {
                {
                    let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 29, i_tab_1, addr_repeat_1,
                            reg_elem_1, n_result_col)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 99, reg_elem_1, n_result_col, r1)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op4_int(v, 140, i_tab_1, r1, reg_elem_1,
                            n_result_col)
                    };
                    unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                    unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                    i_ret = i_tab_1;
                    break '__s63;
                }
            }
        }
    }
    return i_ret;
}

///* This routine runs after codeDistinct().  It makes necessary
///* adjustments to the OP_OpenEphemeral opcode that the codeDistinct()
///* routine made use of.  This processing must be done separately since
///* sometimes codeDistinct is called before the OP_OpenEphemeral is actually
///* laid down.
///*
///* WHERE_DISTINCT_NOOP:
///* WHERE_DISTINCT_UNORDERED:
///*
///*     No adjustments necessary.  This function is a no-op.
///*
///* WHERE_DISTINCT_UNIQUE:
///*
///*     The ephemeral table is not needed.  So change the
///*     OP_OpenEphemeral opcode into an OP_Noop.
///*
///* WHERE_DISTINCT_ORDERED:
///*
///*     The ephemeral table is not needed.  But we do need register
///*     iVal to be initialized to NULL.  So change the OP_OpenEphemeral
///*     into an OP_Null on the iVal register.
#[allow(unused_doc_comments)]
extern "C" fn fix_distinct_open_eph(p_parse_1: &Parse, e_tnct_type_1: i32,
    i_val_1: i32, i_open_eph_addr_1: i32) -> () {
    if (*p_parse_1).n_err == 0 && (e_tnct_type_1 == 1 || e_tnct_type_1 == 2) {
        let v: *mut Vdbe = (*p_parse_1).p_vdbe;
        unsafe { sqlite3_vdbe_change_to_noop(v, i_open_eph_addr_1) };
        if unsafe {
                        (*unsafe {
                                        sqlite3_vdbe_get_op(v, i_open_eph_addr_1 + 1)
                                    }).opcode
                    } as i32 == 190 {
            unsafe { sqlite3_vdbe_change_to_noop(v, i_open_eph_addr_1 + 1) };
        }
        if e_tnct_type_1 == 2 {
            /// Change the OP_OpenEphemeral to an OP_Null that sets the MEM_Cleared
            ///* bit on the first register of the previous value.  This will cause the
            ///* OP_Ne added in codeDistinct() to always fail on the first iteration of
            ///* the loop even if the first row is all NULLs.
            let p_op: *mut VdbeOp =
                unsafe { sqlite3_vdbe_get_op(v, i_open_eph_addr_1) };
            unsafe { (*p_op).opcode = 77 as u8 };
            unsafe { (*p_op).p1 = 1 };
            unsafe { (*p_op).p2 = i_val_1 };
        }
    }
}

///* Code the OP_MakeRecord instruction that generates the entry to be
///* added into the sorter.
///*
///* Return the register in which the result is stored.
extern "C" fn make_sorter_record(p_parse_1: *mut Parse, p_sort_1: &SortCtx,
    p_select_1: *mut Select, reg_base_1: i32, n_base_1: i32) -> i32 {
    let n_ob_sat: i32 = (*p_sort_1).n_ob_sat;
    let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
    let reg_out: i32 =
        { let __p = unsafe { &mut (*p_parse_1).n_mem }; *__p += 1; *__p };
    if !((*p_sort_1).p_deferred_row_load).is_null() {
        inner_loop_load_row(p_parse_1, unsafe { &*p_select_1 },
            unsafe { &*(*p_sort_1).p_deferred_row_load });
    }
    unsafe {
        sqlite3_vdbe_add_op3(v, 99, reg_base_1 + n_ob_sat,
            n_base_1 - n_ob_sat, reg_out)
    };
    return reg_out;
}

///* Allocate a KeyInfo object sufficient for an index of N key columns and
///* X extra columns.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_key_info_alloc(db: *mut Sqlite3, n: i32, x: i32)
    -> *mut KeyInfo {
    let n_extra: i32 =
        ((n + x) as u64 *
                (core::mem::size_of::<*mut CollSeq>() as u64 + 1 as u64)) as
            i32;
    let mut p: *mut KeyInfo = core::ptr::null_mut();
    { let _ = 0; };
    if n + x > 65535 {
        return unsafe { sqlite3_oom_fault(db) } as *mut KeyInfo;
    }
    p =
        unsafe {
                sqlite3_db_malloc_raw_nn(db,
                    core::mem::offset_of!(KeyInfo, a_coll) as u64 +
                            0 as u64 * core::mem::size_of::<*mut CollSeq>() as u64 +
                        n_extra as u64)
            } as *mut KeyInfo;
    if !(p).is_null() {
        unsafe {
            (*p).a_sort_flags =
                unsafe {
                        &raw mut *(unsafe { (*p).a_coll.as_ptr() } as
                                        *mut *mut CollSeq).offset((n + x) as isize)
                    } as *mut u8
        };
        unsafe { (*p).n_key_field = n as u16 };
        unsafe { (*p).n_all_field = (n + x) as u16 };
        unsafe { (*p).enc = unsafe { (*db).enc } };
        unsafe { (*p).db = db };
        unsafe { (*p).n_ref = 1 as u32 };
        unsafe {
            memset(unsafe { (*p).a_coll.as_ptr() } as *mut *mut CollSeq as
                    *mut (), 0, n_extra as u64)
        };
    } else { return unsafe { sqlite3_oom_fault(db) } as *mut KeyInfo; }
    return p;
}

///* Given an expression list, generate a KeyInfo structure that records
///* the collating sequence for each expression in that expression list.
///*
///* If the ExprList is an ORDER BY or GROUP BY clause then the resulting
///* KeyInfo structure is appropriate for initializing a virtual index to
///* implement that clause.  If the ExprList is the result set of a SELECT
///* then the KeyInfo structure is appropriate for initializing a virtual
///* index to implement a DISTINCT test.
///*
///* Space to hold the KeyInfo structure is obtained from malloc.  The calling
///* function is responsible for seeing that this structure is eventually
///* freed.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_key_info_from_expr_list(p_parse: *mut Parse,
    p_list: &ExprList, i_start: i32, n_extra: i32) -> *mut KeyInfo {
    let mut n_expr: i32 = 0;
    let mut p_info: *mut KeyInfo = core::ptr::null_mut();
    let mut p_item: *const ExprListItem = core::ptr::null();
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let mut i: i32 = 0;
    n_expr = (*p_list).n_expr;
    p_info = sqlite3_key_info_alloc(db, n_expr - i_start, n_extra + 1);
    if !(p_info).is_null() {
        { let _ = 0; };
        {
            {
                i = i_start;
                p_item =
                    unsafe {
                        ((*p_list).a.as_ptr() as
                                *mut ExprListItem).offset(i_start as isize)
                    }
            };
            '__b65: loop {
                if !(i < n_expr) { break '__b65; }
                '__c65: loop {
                    unsafe {
                        *(unsafe { (*p_info).a_coll.as_ptr() } as
                                        *mut *mut CollSeq).offset((i - i_start) as isize) =
                            unsafe {
                                sqlite3_expr_nn_coll_seq(p_parse,
                                    unsafe { (*p_item).p_expr } as *const Expr)
                            }
                    };
                    unsafe {
                        *unsafe {
                                    (*p_info).a_sort_flags.offset((i - i_start) as isize)
                                } = unsafe { (*p_item).fg.sort_flags }
                    };
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
    return p_info;
}

///* Generate code that will push the record in registers regData
///* through regData+nData-1 onto the sorter.
#[allow(unused_doc_comments)]
extern "C" fn push_onto_sorter(p_parse_1: *mut Parse, p_sort_1: *mut SortCtx,
    p_select_1: *mut Select, reg_data_1: i32, reg_orig_data_1: i32,
    n_data_1: i32, n_prefix_reg_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        /// Stmt under construction
        let b_seq: i32 =
            (unsafe { (*p_sort_1).sort_flags } as i32 & 1 == 0) as i32;
        let n_expr: i32 =
            unsafe { (*unsafe { (*p_sort_1).p_order_by }).n_expr };
        /// No. of ORDER BY terms
        let n_base: i32 = n_expr + b_seq + n_data_1;
        /// Fields in sorter record
        let mut reg_base: i32 = 0;
        /// Regs for sorter record
        let mut reg_record: i32 = 0;
        /// Assembled sorter record
        let n_ob_sat: i32 = unsafe { (*p_sort_1).n_ob_sat };
        /// ORDER BY terms to skip
        let mut op: i32 = 0;
        /// Opcode to add sorter record to sorter
        let mut i_limit: i32 = 0;
        /// LIMIT counter
        let mut i_skip: i32 = 0;

        /// End of the sorter insert loop
        { let _ = 0; };

        /// Three cases:
        ///*   (1) The data to be sorted has already been packed into a Record
        ///*       by a prior OP_MakeRecord.  In this case nData==1 and regData
        ///*       will be completely unrelated to regOrigData.
        ///*   (2) All output columns are included in the sort record.  In that
        ///*       case regData==regOrigData.
        ///*   (3) Some output columns are omitted from the sort record due to
        ///*       the SQLITE_ENABLE_SORTER_REFERENCES optimization, or due to the
        ///*       SQLITE_ECEL_OMITREF optimization, or due to the
        ///*       SortCtx.pDeferredRowLoad optimization.  In any of these cases
        ///*       regOrigData is 0 to prevent this routine from trying to copy
        ///*       values that might not yet exist.
        { let _ = 0; };
        if n_prefix_reg_1 != 0 {
            { let _ = 0; };
            reg_base = reg_data_1 - n_prefix_reg_1;
        } else {
            reg_base = unsafe { (*p_parse_1).n_mem } + 1;
            unsafe { (*p_parse_1).n_mem += n_base };
        }
        { let _ = 0; };
        i_limit =
            if unsafe { (*p_select_1).i_offset } != 0 {
                (unsafe { (*p_select_1).i_offset }) + 1
            } else { unsafe { (*p_select_1).i_limit } };
        unsafe {
            (*p_sort_1).label_done =
                unsafe { sqlite3_vdbe_make_label(p_parse_1) }
        };
        unsafe {
            sqlite3_expr_code_expr_list(p_parse_1,
                unsafe { (*p_sort_1).p_order_by }, reg_base, reg_orig_data_1,
                (1 | if reg_orig_data_1 != 0 { 4 } else { 0 }) as u8)
        };
        if b_seq != 0 {
            unsafe {
                sqlite3_vdbe_add_op2(v, 128,
                    unsafe { (*p_sort_1).i_e_cursor }, reg_base + n_expr)
            };
        }
        if n_prefix_reg_1 == 0 && n_data_1 > 0 {
            unsafe {
                sqlite3_expr_code_move(p_parse_1, reg_data_1,
                    reg_base + n_expr + b_seq, n_data_1)
            };
        }
        if n_ob_sat > 0 {
            let mut reg_prev_key: i32 = 0;
            /// The first nOBSat columns of the previous row
            let mut addr_first: i32 = 0;
            /// Address of the OP_IfNot opcode
            let mut addr_jmp: i32 = 0;
            /// Address of the OP_Jump opcode
            let mut p_op: *mut VdbeOp = core::ptr::null_mut();
            /// Opcode that opens the sorter
            let mut n_key: i32 = 0;
            /// Number of sorting key columns, including OP_Sequence
            let mut p_ki: *mut KeyInfo = core::ptr::null_mut();

            /// Original KeyInfo on the sorter table
            (reg_record =
                make_sorter_record(p_parse_1, unsafe { &*p_sort_1 },
                    p_select_1, reg_base, n_base));
            reg_prev_key = unsafe { (*p_parse_1).n_mem } + 1;
            unsafe { (*p_parse_1).n_mem += unsafe { (*p_sort_1).n_ob_sat } };
            n_key = n_expr - unsafe { (*p_sort_1).n_ob_sat } + b_seq;
            if b_seq != 0 {
                addr_first =
                    unsafe { sqlite3_vdbe_add_op1(v, 17, reg_base + n_expr) };
            } else {
                addr_first =
                    unsafe {
                        sqlite3_vdbe_add_op1(v, 122,
                            unsafe { (*p_sort_1).i_e_cursor })
                    };
            }
            unsafe {
                sqlite3_vdbe_add_op3(v, 92, reg_prev_key, reg_base,
                    unsafe { (*p_sort_1).n_ob_sat })
            };
            p_op =
                unsafe {
                    sqlite3_vdbe_get_op(v,
                        unsafe { (*p_sort_1).addr_sort_index })
                };
            if unsafe { (*unsafe { (*p_parse_1).db }).malloc_failed } != 0 {
                return;
            }
            unsafe { (*p_op).p2 = n_key + n_data_1 };
            p_ki = unsafe { (*p_op).p4.p_key_info };
            unsafe {
                memset(unsafe { (*p_ki).a_sort_flags } as *mut (), 0,
                    unsafe { (*p_ki).n_key_field } as u64)
            };

            /// Makes OP_Jump testable
            unsafe {
                sqlite3_vdbe_change_p4(v, -1, p_ki as *mut i8 as *const i8,
                    -9)
            };
            unsafe {
                (*p_op).p4.p_key_info =
                    sqlite3_key_info_from_expr_list(p_parse_1,
                        unsafe { &*unsafe { (*p_sort_1).p_order_by } }, n_ob_sat,
                        unsafe { (*p_ki).n_all_field } as i32 -
                                unsafe { (*p_ki).n_key_field } as i32 - 1)
            };
            p_op = core::ptr::null_mut();

            /// Ensure pOp not used after sqlite3VdbeAddOp3()
            (addr_jmp = unsafe { sqlite3_vdbe_current_addr(v) });
            unsafe {
                sqlite3_vdbe_add_op3(v, 14, addr_jmp + 1, 0, addr_jmp + 1)
            };
            unsafe {
                (*p_sort_1).label_bk_out =
                    unsafe { sqlite3_vdbe_make_label(p_parse_1) }
            };
            unsafe {
                (*p_sort_1).reg_return =
                    {
                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                        *__p += 1;
                        *__p
                    }
            };
            unsafe {
                sqlite3_vdbe_add_op2(v, 10, unsafe { (*p_sort_1).reg_return },
                    unsafe { (*p_sort_1).label_bk_out })
            };
            unsafe {
                sqlite3_vdbe_add_op1(v, 148,
                    unsafe { (*p_sort_1).i_e_cursor })
            };
            if i_limit != 0 {
                unsafe {
                    sqlite3_vdbe_add_op2(v, 17, i_limit,
                        unsafe { (*p_sort_1).label_done })
                };
            }
            unsafe { sqlite3_vdbe_jump_here(v, addr_first) };
            unsafe {
                sqlite3_expr_code_move(p_parse_1, reg_base, reg_prev_key,
                    unsafe { (*p_sort_1).n_ob_sat })
            };
            unsafe { sqlite3_vdbe_jump_here(v, addr_jmp) };
        }
        if i_limit != 0 {
            /// At this point the values for the new sorter entry are stored
            ///* in an array of registers. They need to be composed into a record
            ///* and inserted into the sorter if either (a) there are currently
            ///* less than LIMIT+OFFSET items or (b) the new record is smaller than
            ///* the largest record currently in the sorter. If (b) is true and there
            ///* are already LIMIT+OFFSET items in the sorter, delete the largest
            ///* entry before inserting the new one. This way there are never more
            ///* than LIMIT+OFFSET items in the sorter.
            ///*
            ///* If the new record does not need to be inserted into the sorter,
            ///* jump to the next iteration of the loop. If the pSort->labelOBLopt
            ///* value is not zero, then it is a label of where to jump.  Otherwise,
            ///* just bypass the row insert logic.  See the header comment on the
            ///* sqlite3WhereOrderByLimitOptLabel() function for additional info.
            let i_csr: i32 = unsafe { (*p_sort_1).i_e_cursor };
            unsafe {
                sqlite3_vdbe_add_op2(v, 62, i_limit,
                    unsafe { sqlite3_vdbe_current_addr(v) } + 4)
            };
            unsafe { sqlite3_vdbe_add_op2(v, 32, i_csr, 0) };
            i_skip =
                unsafe {
                    sqlite3_vdbe_add_op4_int(v, 41, i_csr, 0,
                        reg_base + n_ob_sat, n_expr - n_ob_sat)
                };
            unsafe { sqlite3_vdbe_add_op1(v, 132, i_csr) };
        }
        if reg_record == 0 {
            reg_record =
                make_sorter_record(p_parse_1, unsafe { &*p_sort_1 },
                    p_select_1, reg_base, n_base);
        }
        if unsafe { (*p_sort_1).sort_flags } as i32 & 1 != 0 {
            op = 141;
        } else { op = 140; }
        unsafe {
            sqlite3_vdbe_add_op4_int(v, op, unsafe { (*p_sort_1).i_e_cursor },
                reg_record, reg_base + n_ob_sat, n_base - n_ob_sat)
        };
        if i_skip != 0 {
            unsafe {
                sqlite3_vdbe_change_p2(v, i_skip,
                    if unsafe { (*p_sort_1).label_ob_lopt } != 0 {
                        unsafe { (*p_sort_1).label_ob_lopt }
                    } else { unsafe { sqlite3_vdbe_current_addr(v) } })
            };
        }
    }
}

///* This routine generates the code for the inside of the inner loop
///* of a SELECT.
///*
///* If srcTab is negative, then the p->pEList expressions
///* are evaluated in order to get the data for this row.  If srcTab is
///* zero or more, then data is pulled from srcTab and p->pEList is used only
///* to get the number of columns and the collation sequence for each column.
#[allow(unused_doc_comments)]
extern "C" fn select_inner_loop(p_parse_1: *mut Parse, p: *mut Select,
    src_tab_1: i32, mut p_sort_1: *mut SortCtx,
    p_distinct_1: *const DistinctCtx, p_dest_1: &mut SelectDest,
    i_continue_1: i32, i_break_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut has_distinct: i32 = 0;
        /// True if the DISTINCT keyword is present
        let e_dest: i32 = (*p_dest_1).e_dest as i32;
        /// How to dispose of results
        let i_parm: i32 = (*p_dest_1).i_sd_parm;
        /// First argument to disposal method
        let mut n_result_col: i32 = 0;
        /// Number of result columns
        let mut n_prefix_reg: i32 = 0;
        /// Number of extra registers before regResult
        let mut s_row_load_info: RowLoadInfo = unsafe { core::mem::zeroed() };
        /// Info for deferred row loading
        /// Usually, regResult is the first cell in an array of memory cells
        ///* containing the current result row. In this case regOrig is set to the
        ///* same value. However, if the results are being sent to the sorter, the
        ///* values for any expressions that are also part of the sort-key are omitted
        ///* from this array. In this case regOrig is set to zero.
        let mut reg_result: i32 = 0;
        /// Start of memory holding current results
        let mut reg_orig: i32 = 0;

        /// Start of memory holding full result (or 0)
        { let _ = 0; };
        { let _ = 0; };
        has_distinct =
            if !(p_distinct_1).is_null() {
                (unsafe { (*p_distinct_1).e_tnct_type }) as i32
            } else { 0 };
        if !(p_sort_1).is_null() &&
                unsafe { (*p_sort_1).p_order_by } == core::ptr::null_mut() {
            p_sort_1 = core::ptr::null_mut();
        }
        if p_sort_1 == core::ptr::null_mut() &&
                (has_distinct == 0) as i32 != 0 {
            { let _ = 0; };
            code_offset(v, unsafe { (*p).i_offset }, i_continue_1);
        }

        /// Pull the requested columns.
        (n_result_col = unsafe { (*unsafe { (*p).p_e_list }).n_expr });
        if (*p_dest_1).i_sdst == 0 {
            if !(p_sort_1).is_null() {
                n_prefix_reg =
                    unsafe { (*unsafe { (*p_sort_1).p_order_by }).n_expr };
                if (unsafe { (*p_sort_1).sort_flags } as i32 & 1 == 0) as i32
                        != 0 {
                    {
                        let __p = &mut n_prefix_reg;
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                }
                unsafe { (*p_parse_1).n_mem += n_prefix_reg };
            }
            (*p_dest_1).i_sdst = unsafe { (*p_parse_1).n_mem } + 1;
            unsafe { (*p_parse_1).n_mem += n_result_col };
        } else if (*p_dest_1).i_sdst + n_result_col >
                unsafe { (*p_parse_1).n_mem } {

            /// This is an error condition that can result, for example, when a SELECT
            ///* on the right-hand side of an INSERT contains more result columns than
            ///* there are columns in the table on the left.  The error will be caught
            ///* and reported later.  But we need to make sure enough memory is allocated
            ///* to avoid other spurious errors in the meantime.
            unsafe { (*p_parse_1).n_mem += n_result_col };
        }
        (*p_dest_1).n_sdst = n_result_col;
        reg_orig = { reg_result = (*p_dest_1).i_sdst; reg_result };
        if src_tab_1 >= 0 {
            {
                i = 0;
                '__b66: loop {
                    if !(i < n_result_col) { break '__b66; }
                    '__c66: loop {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, src_tab_1, i, reg_result + i)
                        };
                        break '__c66;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else if e_dest != 1 {
            /// If the destination is an EXISTS(...) expression, the actual
            ///* values returned by the SELECT are not required.
            let mut ecel_flags: u8 = 0 as u8;
            /// "ecel" is an abbreviation of "ExprCodeExprList"
            let mut p_e_list: *const ExprList = core::ptr::null();
            if e_dest == 8 || e_dest == 7 || e_dest == 11 {
                ecel_flags = 1 as u8;
            } else { ecel_flags = 0 as u8; }
            if !(p_sort_1).is_null() && has_distinct == 0 && e_dest != 10 &&
                    e_dest != 12 {

                /// For each expression in p->pEList that is a copy of an expression in
                ///* the ORDER BY clause (pSort->pOrderBy), set the associated
                ///* iOrderByCol value to one more than the index of the ORDER BY
                ///* expression within the sort-key that pushOntoSorter() will generate.
                ///* This allows the p->pEList field to be omitted from the sorted record,
                ///* saving space and CPU cycles.
                (ecel_flags |= (8 | 4) as u8);
                {
                    i = unsafe { (*p_sort_1).n_ob_sat };
                    '__b67: loop {
                        if !(i <
                                        unsafe { (*unsafe { (*p_sort_1).p_order_by }).n_expr }) {
                            break '__b67;
                        }
                        '__c67: loop {
                            let mut j: i32 = 0;
                            if {
                                        j =
                                            unsafe {
                                                    (*(unsafe {
                                                                                (*unsafe { (*p_sort_1).p_order_by }).a.as_ptr()
                                                                            } as
                                                                            *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                } as i32;
                                        j
                                    } > 0 {
                                unsafe {
                                    (*(unsafe { (*unsafe { (*p).p_e_list }).a.as_ptr() } as
                                                                *mut ExprListItem).offset((j - 1) as
                                                                isize)).u.x.i_order_by_col =
                                        (i + 1 - unsafe { (*p_sort_1).n_ob_sat }) as u16
                                };
                            }
                            break '__c67;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }

                /// Adjust nResultCol to account for columns that are omitted
                ///* from the sorter by the optimizations in this branch
                (p_e_list = unsafe { (*p).p_e_list });
                {
                    i = 0;
                    '__b68: loop {
                        if !(i < unsafe { (*p_e_list).n_expr }) { break '__b68; }
                        '__c68: loop {
                            if unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                        } as i32 > 0 {
                                {
                                    let __p = &mut n_result_col;
                                    let __t = *__p;
                                    *__p -= 1;
                                    __t
                                };
                                reg_orig = 0;
                            }
                            break '__c68;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                { let _ = 0; };
            }
            s_row_load_info.reg_result = reg_result;
            s_row_load_info.ecel_flags = ecel_flags;
            if unsafe { (*p).i_limit } != 0 && ecel_flags as i32 & 8 != 0 &&
                    n_prefix_reg > 0 {
                { let _ = 0; };
                { let _ = 0; };
                unsafe {
                    (*p_sort_1).p_deferred_row_load = &mut s_row_load_info
                };
                reg_orig = 0;
            } else {
                inner_loop_load_row(p_parse_1, unsafe { &*p },
                    &s_row_load_info);
            }
        }
        if has_distinct != 0 {
            let e_type: i32 = unsafe { (*p_distinct_1).e_tnct_type } as i32;
            let mut i_tab: i32 = unsafe { (*p_distinct_1).tab_tnct };
            { let _ = 0; };
            i_tab =
                code_distinct(p_parse_1, e_type, i_tab, i_continue_1,
                    unsafe { &*unsafe { (*p).p_e_list } }, reg_result);
            fix_distinct_open_eph(unsafe { &*p_parse_1 }, e_type, i_tab,
                unsafe { (*p_distinct_1).addr_tnct });
            if p_sort_1 == core::ptr::null_mut() {
                code_offset(v, unsafe { (*p).i_offset }, i_continue_1);
            }
        }
        '__s69:
            {
            match e_dest {
                6 => {
                    {
                        let mut r1: i32 =
                            unsafe {
                                sqlite3_get_temp_range(p_parse_1, n_prefix_reg + 1)
                            };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col,
                                r1 + n_prefix_reg)
                        };
                        if e_dest == 3 {
                            /// If the destination is DistFifo, then cursor (iParm+1) is open
                            ///* on an ephemeral index. If the current row is already present
                            ///* in the index, do not write it to the output. If not, add the
                            ///* current row to the index and proceed with writing it to the
                            ///* output table as well.
                            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) } + 4;
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, addr, r1, 0)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1, reg_result,
                                    n_result_col)
                            };
                            { let _ = 0; };
                        }
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, r1 + n_prefix_reg,
                                reg_orig, 1, n_prefix_reg);
                        } else {
                            let mut r2: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                            unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                            unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        }
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r1, n_prefix_reg + 1)
                        };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };

                            /// If the UPDATE FROM join is an aggregate that matches no rows, it
                            ///* might still be trying to return one row, because that is what
                            ///* aggregates do.  Don't record that empty row in the output table.
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {

                            /// At first glance you would think we could optimize out the
                            ///* ORDER BY in this case since the order of entries in the set
                            ///* does not matter.  But there might be a LIMIT clause, in which
                            ///* case the order does matter
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {

                                /// This occurs in cases where the SELECT had both a DISTINCT and
                                ///* an OFFSET clause.
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                3 => {
                    {
                        let mut r1: i32 =
                            unsafe {
                                sqlite3_get_temp_range(p_parse_1, n_prefix_reg + 1)
                            };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col,
                                r1 + n_prefix_reg)
                        };
                        if e_dest == 3 {
                            /// If the destination is DistFifo, then cursor (iParm+1) is open
                            ///* on an ephemeral index. If the current row is already present
                            ///* in the index, do not write it to the output. If not, add the
                            ///* current row to the index and proceed with writing it to the
                            ///* output table as well.
                            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) } + 4;
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, addr, r1, 0)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1, reg_result,
                                    n_result_col)
                            };
                            { let _ = 0; };
                        }
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, r1 + n_prefix_reg,
                                reg_orig, 1, n_prefix_reg);
                        } else {
                            let mut r2: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                            unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                            unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        }
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r1, n_prefix_reg + 1)
                        };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };

                            /// If the UPDATE FROM join is an aggregate that matches no rows, it
                            ///* might still be trying to return one row, because that is what
                            ///* aggregates do.  Don't record that empty row in the output table.
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {

                            /// At first glance you would think we could optimize out the
                            ///* ORDER BY in this case since the order of entries in the set
                            ///* does not matter.  But there might be a LIMIT clause, in which
                            ///* case the order does matter
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {

                                /// This occurs in cases where the SELECT had both a DISTINCT and
                                ///* an OFFSET clause.
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                12 => {
                    {
                        let mut r1: i32 =
                            unsafe {
                                sqlite3_get_temp_range(p_parse_1, n_prefix_reg + 1)
                            };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col,
                                r1 + n_prefix_reg)
                        };
                        if e_dest == 3 {
                            /// If the destination is DistFifo, then cursor (iParm+1) is open
                            ///* on an ephemeral index. If the current row is already present
                            ///* in the index, do not write it to the output. If not, add the
                            ///* current row to the index and proceed with writing it to the
                            ///* output table as well.
                            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) } + 4;
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, addr, r1, 0)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1, reg_result,
                                    n_result_col)
                            };
                            { let _ = 0; };
                        }
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, r1 + n_prefix_reg,
                                reg_orig, 1, n_prefix_reg);
                        } else {
                            let mut r2: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                            unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                            unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        }
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r1, n_prefix_reg + 1)
                        };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };

                            /// If the UPDATE FROM join is an aggregate that matches no rows, it
                            ///* might still be trying to return one row, because that is what
                            ///* aggregates do.  Don't record that empty row in the output table.
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {

                            /// At first glance you would think we could optimize out the
                            ///* ORDER BY in this case since the order of entries in the set
                            ///* does not matter.  But there might be a LIMIT clause, in which
                            ///* case the order does matter
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {

                                /// This occurs in cases where the SELECT had both a DISTINCT and
                                ///* an OFFSET clause.
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                10 => {
                    {
                        let mut r1: i32 =
                            unsafe {
                                sqlite3_get_temp_range(p_parse_1, n_prefix_reg + 1)
                            };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col,
                                r1 + n_prefix_reg)
                        };
                        if e_dest == 3 {
                            /// If the destination is DistFifo, then cursor (iParm+1) is open
                            ///* on an ephemeral index. If the current row is already present
                            ///* in the index, do not write it to the output. If not, add the
                            ///* current row to the index and proceed with writing it to the
                            ///* output table as well.
                            let addr: i32 = unsafe { sqlite3_vdbe_current_addr(v) } + 4;
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, addr, r1, 0)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1, reg_result,
                                    n_result_col)
                            };
                            { let _ = 0; };
                        }
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, r1 + n_prefix_reg,
                                reg_orig, 1, n_prefix_reg);
                        } else {
                            let mut r2: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                            unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                            unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        }
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r1, n_prefix_reg + 1)
                        };
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };

                            /// If the UPDATE FROM join is an aggregate that matches no rows, it
                            ///* might still be trying to return one row, because that is what
                            ///* aggregates do.  Don't record that empty row in the output table.
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {

                            /// At first glance you would think we could optimize out the
                            ///* ORDER BY in this case since the order of entries in the set
                            ///* does not matter.  But there might be a LIMIT clause, in which
                            ///* case the order does matter
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {

                                /// This occurs in cases where the SELECT had both a DISTINCT and
                                ///* an OFFSET clause.
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                13 => {
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else {
                            let i2: i32 = (*p_dest_1).i_sd_parm2;
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };

                            /// If the UPDATE FROM join is an aggregate that matches no rows, it
                            ///* might still be trying to return one row, because that is what
                            ///* aggregates do.  Don't record that empty row in the output table.
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 51, reg_result, i_break_1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, reg_result + (i2 < 0) as i32,
                                    n_result_col - (i2 < 0) as i32, r1)
                            };
                            if i2 < 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_result)
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result, i2)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {

                            /// At first glance you would think we could optimize out the
                            ///* ORDER BY in this case since the order of entries in the set
                            ///* does not matter.  But there might be a LIMIT clause, in which
                            ///* case the order does matter
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {

                                /// This occurs in cases where the SELECT had both a DISTINCT and
                                ///* an OFFSET clause.
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                9 => {
                    {
                        if !(p_sort_1).is_null() {

                            /// At first glance you would think we could optimize out the
                            ///* ORDER BY in this case since the order of entries in the set
                            ///* does not matter.  But there might be a LIMIT clause, in which
                            ///* case the order does matter
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm2 = 0;
                        } else {
                            let mut r1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            { let _ = 0; };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 99, reg_result, n_result_col, r1,
                                    (*p_dest_1).z_aff_sdst as *const i8, n_result_col)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_result,
                                    n_result_col)
                            };
                            if (*p_dest_1).i_sd_parm2 != 0 {
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                        reg_result, n_result_col)
                                };
                                unsafe {
                                    sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                        c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                                };
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        }
                        break '__s69;
                    }
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {

                                /// This occurs in cases where the SELECT had both a DISTINCT and
                                ///* an OFFSET clause.
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                1 => {
                    {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_parm) };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {

                                /// This occurs in cases where the SELECT had both a DISTINCT and
                                ///* an OFFSET clause.
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                8 => {
                    {
                        if !(p_sort_1).is_null() {
                            { let _ = 0; };
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                            (*p_dest_1).i_sd_parm = reg_result;
                        } else {
                            { let _ = 0; };
                            if reg_result != i_parm {

                                /// This occurs in cases where the SELECT had both a DISTINCT and
                                ///* an OFFSET clause.
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 82, reg_result, i_parm,
                                        n_result_col - 1)
                                };
                            }
                        }
                        break '__s69;
                    }
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                11 => {
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                7 => {
                    {
                        if !(p_sort_1).is_null() {
                            push_onto_sorter(p_parse_1, p_sort_1, p, reg_result,
                                reg_orig, n_result_col, n_prefix_reg);
                        } else if e_dest == 11 {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, reg_result, n_result_col)
                            };
                        }
                        break '__s69;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                4 => {
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                5 => {
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut addr_test: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        if e_dest == 4 {

                            /// If the destination is DistQueue, then cursor (iParm+1) is open
                            ///* on a second ephemeral index that holds all values every previously
                            ///* added to the queue.
                            (addr_test =
                                unsafe {
                                    sqlite3_vdbe_add_op4_int(v, 29, i_parm + 1, 0, reg_result,
                                        n_result_col)
                                });
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_result, n_result_col, r3)
                        };
                        if e_dest == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm + 1, r3) };
                            unsafe { sqlite3_vdbe_change_p5(v, 16 as u16) };
                        }
                        {
                            i = 0;
                            '__b70: loop {
                                if !(i < n_key) { break '__b70; }
                                '__c70: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            reg_result +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + i)
                                    };
                                    break '__c70;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 128, i_parm, r2 + n_key) };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, r2, n_key + 2)
                        };
                        if addr_test != 0 {
                            unsafe { sqlite3_vdbe_jump_here(v, addr_test) };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s69;
                    }
                    { { let _ = 0; }; break '__s69; }
                }
                _ => { { { let _ = 0; }; break '__s69; } }
            }
        }
        if p_sort_1 == core::ptr::null_mut() && unsafe { (*p).i_limit } != 0 {
            unsafe {
                sqlite3_vdbe_add_op2(v, 63, unsafe { (*p).i_limit },
                    i_break_1)
            };
        }
    }
}

///* Handle the special case of a compound-select that originates from a
///* VALUES clause.  By handling this as a special case, we avoid deep
///* recursion, and thus do not need to enforce the SQLITE_LIMIT_COMPOUND_SELECT
///* on a VALUES clause.
///*
///* Because the Select object originates from a VALUES clause:
///*   (1) There is no LIMIT or OFFSET or else there is a LIMIT of exactly 1
///*   (2) All terms are UNION ALL
///*   (3) There is no ORDER BY clause
///*
///* The "LIMIT of exactly 1" case of condition (1) comes about when a VALUES
///* clause occurs within scalar expression (ex: "SELECT (VALUES(1),(2),(3))").
///* The sqlite3CodeSubselect will have added the LIMIT 1 clause in tht case.
///* Since the limit is exactly 1, we only need to evaluate the left-most VALUES.
extern "C" fn multi_select_values(p_parse_1: *mut Parse, mut p: *mut Select,
    p_dest_1: *mut SelectDest) -> i32 {
    unsafe {
        let mut n_row: i32 = 1;
        let rc: i32 = 0;
        let b_show_all: i32 =
            (unsafe { (*p).p_limit } == core::ptr::null_mut()) as i32;
        { let _ = 0; };
        '__b71: loop {
            '__c71: loop {
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                if !(unsafe { (*p).p_win }).is_null() { return -1; }
                if unsafe { (*p).p_prior } == core::ptr::null_mut() {
                    break '__b71;
                }
                { let _ = 0; };
                p = unsafe { (*p).p_prior };
                n_row += b_show_all;
                break '__c71;
            }
        }
        unsafe {
            sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                c"SCAN %d CONSTANT ROW%s".as_ptr() as *mut i8 as *const i8,
                n_row,
                if n_row == 1 {
                    c"".as_ptr() as *mut i8
                } else { c"S".as_ptr() as *mut i8 })
        };
        while !(p).is_null() {
            select_inner_loop(p_parse_1, p, -1, core::ptr::null_mut(),
                core::ptr::null(), unsafe { &mut *p_dest_1 }, 1, 1);
            if (b_show_all == 0) as i32 != 0 { break; }
            unsafe { (*p).n_select_row = n_row as LogEst };
            p = unsafe { (*p).p_next };
        }
        return rc;
    }
}

///* Return true if the SELECT statement which is known to be the recursive
///* part of a recursive CTE still has its anchor terms attached.  If the
///* anchor terms have already been removed, then return false.
extern "C" fn has_anchor(mut p: *const Select) -> i32 {
    while !(p).is_null() &&
            unsafe { (*p).sel_flags } & 8192 as u32 != 0 as u32 {
        p = unsafe { (*p).p_prior };
    }
    return (p != core::ptr::null_mut()) as i32;
}

///* Compute the iLimit and iOffset fields of the SELECT based on the
///* pLimit expressions.  pLimit->pLeft and pLimit->pRight hold the expressions
///* that appear in the original SQL statement after the LIMIT and OFFSET
///* keywords.  Or NULL if those keywords are omitted. iLimit and iOffset
///* are the integer memory register numbers for counters used to compute
///* the limit and offset.  If there is no limit and/or offset, then
///* iLimit and iOffset are negative.
///*
///* This routine changes the values of iLimit and iOffset only if
///* a limit or offset is defined by pLimit->pLeft and pLimit->pRight.  iLimit
///* and iOffset should have been preset to appropriate default values (zero)
///* prior to calling this routine.
///*
///* The iOffset register (if it exists) is initialized to the value
///* of the OFFSET.  The iLimit register is initialized to LIMIT.  Register
///* iOffset+1 is initialized to LIMIT+OFFSET.
///*
///* Only if pLimit->pLeft!=0 do the limit registers get
///* redefined.  The UNION ALL operator uses this property to force
///* the reuse of the same limit and offset registers across multiple
///* SELECT statements.
#[allow(unused_doc_comments)]
extern "C" fn compute_limit_registers(p_parse_1: *mut Parse, p: &mut Select,
    i_break_1: i32) -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    let mut i_limit: i32 = 0;
    let mut i_offset: i32 = 0;
    let mut n: i32 = 0;
    let p_limit: *const Expr = (*p).p_limit as *const Expr;
    if (*p).i_limit != 0 { return; }
    if !(p_limit).is_null() {
        { let _ = 0; };
        { let _ = 0; };
        (*p).i_limit =
            {
                i_limit =
                    {
                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                        *__p += 1;
                        *__p
                    };
                i_limit
            };
        v = sqlite3_get_vdbe(p_parse_1);
        { let _ = 0; };
        if unsafe {
                    sqlite3_expr_is_integer(unsafe { (*p_limit).p_left } as
                            *const Expr, &mut n, p_parse_1)
                } != 0 {
            unsafe { sqlite3_vdbe_add_op2(v, 73, n, i_limit) };
            if n == 0 {
                unsafe { sqlite3_vdbe_goto(v, i_break_1) };
            } else if n >= 0 &&
                    (*p).n_select_row as i32 >
                        unsafe { sqlite3_log_est(n as u64) } as i32 {
                (*p).n_select_row = unsafe { sqlite3_log_est(n as u64) };
                (*p).sel_flags |= 16384 as u32;
            }
        } else {
            unsafe {
                sqlite3_expr_code(p_parse_1, unsafe { (*p_limit).p_left },
                    i_limit)
            };
            unsafe { sqlite3_vdbe_add_op1(v, 13, i_limit) };
            unsafe { sqlite3_vdbe_add_op2(v, 17, i_limit, i_break_1) };
        }
        if !(unsafe { (*p_limit).p_right }).is_null() {
            (*p).i_offset =
                {
                    i_offset =
                        {
                            let __p = unsafe { &mut (*p_parse_1).n_mem };
                            *__p += 1;
                            *__p
                        };
                    i_offset
                };
            {
                let __p = unsafe { &mut (*p_parse_1).n_mem };
                let __t = *__p;
                *__p += 1;
                __t
            };

            /// Allocate an extra register for limit+offset
            unsafe {
                sqlite3_expr_code(p_parse_1, unsafe { (*p_limit).p_right },
                    i_offset)
            };
            unsafe { sqlite3_vdbe_add_op1(v, 13, i_offset) };
            unsafe {
                sqlite3_vdbe_add_op3(v, 162, i_limit, i_offset + 1, i_offset)
            };
        }
    }
}

///* Initialize a SelectDest structure.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_dest_init(p_dest: &mut SelectDest,
    e_dest: i32, i_parm: i32) -> () {
    (*p_dest).e_dest = e_dest as u8;
    (*p_dest).i_sd_parm = i_parm;
    (*p_dest).i_sd_parm2 = 0;
    (*p_dest).z_aff_sdst = core::ptr::null_mut();
    (*p_dest).i_sdst = 0;
    (*p_dest).n_sdst = 0;
}

///* Return the appropriate collating sequence for the iCol-th column of
///* the result set for the compound-select statement "p".  Return NULL if
///* the column has no default collating sequence.
///*
///* The collating sequence for the compound select is taken from the
///* left-most term of the select that has a collating sequence.
extern "C" fn multi_select_coll_seq(p_parse_1: *mut Parse, p: &Select,
    i_col_1: i32) -> *mut CollSeq {
    unsafe {
        let mut p_ret: *mut CollSeq = core::ptr::null_mut();
        if !((*p).p_prior).is_null() {
            p_ret =
                multi_select_coll_seq(p_parse_1, unsafe { &*(*p).p_prior },
                    i_col_1);
        } else { p_ret = core::ptr::null_mut(); }
        { let _ = 0; };
        if p_ret == core::ptr::null_mut() &&
                i_col_1 < unsafe { (*(*p).p_e_list).n_expr } {
            p_ret =
                unsafe {
                    sqlite3_expr_coll_seq(p_parse_1,
                        unsafe {
                                (*(unsafe { (*(*p).p_e_list).a.as_ptr() } as
                                                *mut ExprListItem).offset(i_col_1 as isize)).p_expr
                            } as *const Expr)
                };
        }
        return p_ret;
    }
}

///* The select statement passed as the second parameter is a compound SELECT
///* with an ORDER BY clause. This function allocates and returns a KeyInfo
///* structure suitable for implementing the ORDER BY.
///*
///* Space to hold the KeyInfo structure is obtained from malloc. The calling
///* function is responsible for ensuring that this structure is eventually
///* freed.
extern "C" fn multi_select_by_merge_key_info(p_parse_1: *mut Parse,
    p: *mut Select, n_extra_1: i32) -> *mut KeyInfo {
    unsafe {
        let p_order_by: *mut ExprList = unsafe { (*p).p_order_by };
        let n_order_by: i32 =
            if p_order_by != core::ptr::null_mut() {
                unsafe { (*p_order_by).n_expr }
            } else { 0 };
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let p_ret: *mut KeyInfo =
            sqlite3_key_info_alloc(db, n_order_by + n_extra_1, 1);
        if !(p_ret).is_null() {
            let mut i: i32 = 0;
            {
                i = 0;
                '__b74: loop {
                    if !(i < n_order_by) { break '__b74; }
                    '__c74: loop {
                        let p_item: *const ExprListItem =
                            unsafe {
                                    &raw mut *(unsafe { (*p_order_by).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)
                                } as *const ExprListItem;
                        let p_term: *mut Expr = unsafe { (*p_item).p_expr };
                        let mut p_coll: *mut CollSeq = core::ptr::null_mut();
                        if unsafe { (*p_term).flags } & 512 as u32 != 0 {
                            p_coll =
                                unsafe {
                                    sqlite3_expr_coll_seq(p_parse_1, p_term as *const Expr)
                                };
                        } else {
                            p_coll =
                                multi_select_coll_seq(p_parse_1, unsafe { &*p },
                                    unsafe { (*p_item).u.x.i_order_by_col } as i32 - 1);
                            if p_coll == core::ptr::null_mut() {
                                p_coll = unsafe { (*db).p_dflt_coll };
                            }
                            unsafe {
                                (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).p_expr =
                                    unsafe {
                                        sqlite3_expr_add_collate_string(p_parse_1 as *const Parse,
                                            p_term, unsafe { (*p_coll).z_name } as *const i8)
                                    }
                            };
                        }
                        { let _ = 0; };
                        unsafe {
                            *(unsafe { (*p_ret).a_coll.as_ptr() } as
                                            *mut *mut CollSeq).offset(i as isize) = p_coll
                        };
                        unsafe {
                            *unsafe { (*p_ret).a_sort_flags.offset(i as isize) } =
                                unsafe {
                                    (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).fg.sort_flags
                                }
                        };
                        break '__c74;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return p_ret;
    }
}

///* This routine generates VDBE code to compute the content of a WITH RECURSIVE
///* query of the form:
///*
///*   <recursive-table> AS (<setup-query> UNION [ALL] <recursive-query>)
///*                         \___________/             \_______________/
///*                           p->pPrior                      p
///*
///*
///* There is exactly one reference to the recursive-table in the FROM clause
///* of recursive-query, marked with the SrcList->a[].fg.isRecursive flag.
///*
///* The setup-query runs once to generate an initial set of rows that go
///* into a Queue table.  Rows are extracted from the Queue table one by
///* one.  Each row extracted from Queue is output to pDest.  Then the single
///* extracted row (now in the iCurrent table) becomes the content of the
///* recursive-table for a recursive-query run.  The output of the recursive-query
///* is added back into the Queue table.  Then another row is extracted from Queue
///* and the iteration continues until the Queue table is empty.
///*
///* If the compound query operator is UNION then no duplicate rows are ever
///* inserted into the Queue table.  The iDistinct table keeps a copy of all rows
///* that have ever been inserted into Queue and causes duplicates to be
///* discarded.  If the operator is UNION ALL, then duplicates are allowed.
///*
///* If the query has an ORDER BY, then entries in the Queue table are kept in
///* ORDER BY order and the first entry is extracted for each cycle.  Without
///* an ORDER BY, the Queue table is just a FIFO.
///*
///* If a LIMIT clause is provided, then the iteration stops after LIMIT rows
///* have been output to pDest.  A LIMIT of zero means to output no rows and a
///* negative LIMIT means to output all rows.  If there is also an OFFSET clause
///* with a positive value, then the first OFFSET outputs are discarded rather
///* than being sent to pDest.  The LIMIT count does not begin until after OFFSET
///* rows have been skipped.
#[allow(unused_doc_comments)]
extern "C" fn generate_with_recursive_query(p_parse_1: *mut Parse,
    p: *mut Select, p_dest_1: *mut SelectDest) -> () {
    unsafe {
        let mut p_src: *const SrcList = core::ptr::null();
        /// The FROM clause of the recursive query
        let mut n_col: i32 = 0;
        /// Number of columns in the recursive table
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// The prepared statement under construction
        let mut p_setup: *mut Select = core::ptr::null_mut();
        /// The setup query
        let mut p_first_rec: *mut Select = core::ptr::null_mut();
        /// Left-most recursive term
        let mut addr_top: i32 = 0;
        /// Top of the loop
        let mut addr_cont: i32 = 0;
        let mut addr_break: i32 = 0;
        /// CONTINUE and BREAK addresses
        let mut i_current: i32 = 0;
        /// The Current table
        let mut reg_current: i32 = 0;
        /// Register holding Current table
        let mut i_queue: i32 = 0;
        /// The Queue table
        let mut i_distinct: i32 = 0;
        /// To ensure unique results if UNION
        let mut e_dest: i32 = 0;
        /// How to write to Queue
        let mut dest_queue: SelectDest = unsafe { core::mem::zeroed() };
        /// SelectDest targeting the Queue table
        let mut i: i32 = 0;
        /// Loop counter
        let mut rc: i32 = 0;
        /// Result code
        let mut p_order_by: *mut ExprList = core::ptr::null_mut();
        /// The ORDER BY clause
        let mut p_limit: *mut Expr = core::ptr::null_mut();
        /// Saved LIMIT and OFFSET
        let mut reg_limit: i32 = 0;
        let mut reg_offset: i32 = 0;
        /// Registers used by LIMIT and OFFSET
        /// Obtain authorization to do a recursive query
        /// Process the LIMIT and OFFSET clauses, if they exist
        /// 4 billion rows
        /// Locate the cursor number of the Current table
        /// Allocate cursors numbers for Queue and Distinct.  The cursor number for
        ///* the Distinct table must be exactly one greater than Queue in order
        ///* for the SRT_DistFifo and SRT_DistQueue destinations to work.
        /// Allocate cursors for Current, Queue, and Distinct.
        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
        /// Generate an ephemeral table used to enforce distinctness on the
        ///* output of the recursive part of the CTE.
        let mut p_key_info_1: *mut KeyInfo = core::ptr::null_mut();
        /// Collating sequence for the result set
        let mut ap_coll: *mut *mut CollSeq = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s76:
                {
                match __state {
                    0 => {
                        p_src = unsafe { (*p).p_src } as *const SrcList;
                        __state = 3;
                    }
                    2 => {
                        unsafe {
                            sqlite3_expr_list_delete(unsafe { (*p_parse_1).db },
                                unsafe { (*p).p_order_by })
                        };
                        __state = 109;
                    }
                    3 => {
                        n_col = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
                        __state = 4;
                    }
                    4 => { v = unsafe { (*p_parse_1).p_vdbe }; __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { i_current = 0; __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { i_distinct = 0; __state = 13; }
                    13 => { e_dest = 6; __state = 14; }
                    14 => { __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => { __state = 18; }
                    18 => { __state = 19; }
                    19 => { __state = 20; }
                    20 => {
                        if !(unsafe { (*p).p_win }).is_null() {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => {
                        if unsafe {
                                    sqlite3_auth_check(p_parse_1, 33, core::ptr::null(),
                                        core::ptr::null(), core::ptr::null())
                                } != 0 {
                            __state = 25;
                        } else { __state = 24; }
                    }
                    22 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"cannot use window functions in recursive queries".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                        __state = 23;
                    }
                    23 => { return; }
                    24 => {
                        addr_break = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        __state = 26;
                    }
                    25 => { return; }
                    26 => {
                        unsafe { (*p).n_select_row = 320 as LogEst };
                        __state = 27;
                    }
                    27 => {
                        compute_limit_registers(p_parse_1, unsafe { &mut *p },
                            addr_break);
                        __state = 28;
                    }
                    28 => { p_limit = unsafe { (*p).p_limit }; __state = 29; }
                    29 => { reg_limit = unsafe { (*p).i_limit }; __state = 30; }
                    30 => {
                        reg_offset = unsafe { (*p).i_offset };
                        __state = 31;
                    }
                    31 => {
                        unsafe { (*p).p_limit = core::ptr::null_mut() };
                        __state = 32;
                    }
                    32 => {
                        unsafe {
                            (*p).i_limit =
                                { unsafe { (*p).i_offset = 0 }; unsafe { (*p).i_offset } }
                        };
                        __state = 33;
                    }
                    33 => {
                        p_order_by = unsafe { (*p).p_order_by };
                        __state = 34;
                    }
                    34 => { i = 0; __state = 36; }
                    35 => {
                        i_queue =
                            {
                                let __p = unsafe { &mut (*p_parse_1).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 41;
                    }
                    36 => {
                        if i < unsafe { (*p_src).n_src } {
                            __state = 37;
                        } else { __state = 35; }
                    }
                    37 => {
                        if unsafe {
                                    (*(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset(i as isize)).fg.is_recursive()
                                } != 0 {
                            __state = 39;
                        } else { __state = 38; }
                    }
                    38 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 36;
                    }
                    39 => {
                        i_current =
                            unsafe {
                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)).i_cursor
                            };
                        __state = 40;
                    }
                    40 => { __state = 35; }
                    41 => {
                        if unsafe { (*p).op } as i32 == 135 {
                            __state = 43;
                        } else { __state = 44; }
                    }
                    42 => {
                        sqlite3_select_dest_init(&mut dest_queue, e_dest, i_queue);
                        __state = 46;
                    }
                    43 => {
                        e_dest = if !(p_order_by).is_null() { 4 } else { 3 };
                        __state = 45;
                    }
                    44 => {
                        e_dest = if !(p_order_by).is_null() { 5 } else { 6 };
                        __state = 42;
                    }
                    45 => {
                        i_distinct =
                            {
                                let __p = unsafe { &mut (*p_parse_1).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 42;
                    }
                    46 => {
                        reg_current =
                            {
                                let __p = unsafe { &mut (*p_parse_1).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 47;
                    }
                    47 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 123, i_current, reg_current, n_col)
                        };
                        __state = 48;
                    }
                    48 => {
                        if !(p_order_by).is_null() {
                            __state = 50;
                        } else { __state = 51; }
                    }
                    49 => { __state = 54; }
                    50 => {
                        p_key_info =
                            multi_select_by_merge_key_info(p_parse_1, p, 1);
                        __state = 52;
                    }
                    51 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 120, i_queue, n_col) };
                        __state = 49;
                    }
                    52 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 120, i_queue,
                                unsafe { (*p_order_by).n_expr } + 2, 0,
                                p_key_info as *mut i8 as *const i8, -9)
                        };
                        __state = 53;
                    }
                    53 => { dest_queue.p_order_by = p_order_by; __state = 49; }
                    54 => {
                        if i_distinct != 0 { __state = 56; } else { __state = 55; }
                    }
                    55 => {
                        unsafe { (*p).p_order_by = core::ptr::null_mut() };
                        __state = 71;
                    }
                    56 => { __state = 57; }
                    57 => { __state = 58; }
                    58 => { { let _ = 0; }; __state = 59; }
                    59 => { { let _ = 0; }; __state = 60; }
                    60 => {
                        n_col = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
                        __state = 61;
                    }
                    61 => {
                        p_key_info_1 =
                            sqlite3_key_info_alloc(unsafe { (*p_parse_1).db }, n_col,
                                1);
                        __state = 62;
                    }
                    62 => {
                        if !(p_key_info_1).is_null() {
                            __state = 63;
                        } else { __state = 64; }
                    }
                    63 => {
                        {
                            i = 0;
                            ap_coll =
                                unsafe { (*p_key_info_1).a_coll.as_ptr() } as
                                    *mut *mut CollSeq
                        };
                        __state = 66;
                    }
                    64 => { { let _ = 0; }; __state = 55; }
                    65 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 120, i_distinct, n_col, 0,
                                p_key_info_1 as *mut () as *const i8, -9)
                        };
                        __state = 55;
                    }
                    66 => {
                        if i < n_col { __state = 67; } else { __state = 65; }
                    }
                    67 => {
                        unsafe {
                            *ap_coll =
                                multi_select_coll_seq(p_parse_1, unsafe { &*p }, i)
                        };
                        __state = 69;
                    }
                    68 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut ap_coll;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 66;
                    }
                    69 => {
                        if core::ptr::null_mut() == unsafe { *ap_coll } {
                            __state = 70;
                        } else { __state = 68; }
                    }
                    70 => {
                        unsafe {
                            *ap_coll =
                                unsafe { (*unsafe { (*p_parse_1).db }).p_dflt_coll }
                        };
                        __state = 68;
                    }
                    71 => { p_first_rec = p; __state = 73; }
                    72 => {
                        p_setup = unsafe { (*p_first_rec).p_prior };
                        __state = 81;
                    }
                    73 => {
                        if p_first_rec != core::ptr::null_mut() {
                            __state = 74;
                        } else { __state = 72; }
                    }
                    74 => {
                        if unsafe { (*p_first_rec).sel_flags } & 8 as u32 != 0 {
                            __state = 77;
                        } else { __state = 76; }
                    }
                    75 => {
                        p_first_rec = unsafe { (*p_first_rec).p_prior };
                        __state = 73;
                    }
                    76 => {
                        unsafe { (*p_first_rec).op = 136 as u8 };
                        __state = 79;
                    }
                    77 => {
                        unsafe {
                            sqlite3_error_msg(p_parse_1,
                                c"recursive aggregate queries not supported".as_ptr() as
                                        *mut i8 as *const i8)
                        };
                        __state = 78;
                    }
                    78 => { __state = 2; }
                    79 => {
                        if unsafe { (*unsafe { (*p_first_rec).p_prior }).sel_flags }
                                    & 8192 as u32 == 0 as u32 {
                            __state = 80;
                        } else { __state = 75; }
                    }
                    80 => { __state = 72; }
                    81 => {
                        unsafe { (*p_setup).p_next = core::ptr::null_mut() };
                        __state = 82;
                    }
                    82 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                                c"SETUP".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 83;
                    }
                    83 => {
                        rc = sqlite3_select(p_parse_1, p_setup, &mut dest_queue);
                        __state = 84;
                    }
                    84 => { unsafe { (*p_setup).p_next = p }; __state = 85; }
                    85 => {
                        if rc != 0 { __state = 87; } else { __state = 86; }
                    }
                    86 => {
                        addr_top =
                            unsafe { sqlite3_vdbe_add_op2(v, 36, i_queue, addr_break) };
                        __state = 88;
                    }
                    87 => { __state = 2; }
                    88 => { __state = 89; }
                    89 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 138, i_current) };
                        __state = 90;
                    }
                    90 => {
                        if !(p_order_by).is_null() {
                            __state = 92;
                        } else { __state = 93; }
                    }
                    91 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 132, i_queue) };
                        __state = 94;
                    }
                    92 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_queue,
                                unsafe { (*p_order_by).n_expr } + 1, reg_current)
                        };
                        __state = 91;
                    }
                    93 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 136, i_queue, reg_current)
                        };
                        __state = 91;
                    }
                    94 => {
                        addr_cont = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        __state = 95;
                    }
                    95 => {
                        code_offset(v, reg_offset, addr_cont);
                        __state = 96;
                    }
                    96 => {
                        select_inner_loop(p_parse_1, p, i_current,
                            core::ptr::null_mut(), core::ptr::null(),
                            unsafe { &mut *p_dest_1 }, addr_cont, addr_break);
                        __state = 97;
                    }
                    97 => {
                        if reg_limit != 0 { __state = 99; } else { __state = 98; }
                    }
                    98 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_cont) };
                        __state = 101;
                    }
                    99 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 63, reg_limit, addr_break)
                        };
                        __state = 100;
                    }
                    100 => { __state = 98; }
                    101 => {
                        unsafe { (*p_first_rec).p_prior = core::ptr::null_mut() };
                        __state = 102;
                    }
                    102 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                                c"RECURSIVE STEP".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 103;
                    }
                    103 => {
                        sqlite3_select(p_parse_1, p, &mut dest_queue);
                        __state = 104;
                    }
                    104 => { { let _ = 0; }; __state = 105; }
                    105 => {
                        unsafe { (*p_first_rec).p_prior = p_setup };
                        __state = 106;
                    }
                    106 => {
                        unsafe { sqlite3_vdbe_goto(v, addr_top) };
                        __state = 107;
                    }
                    107 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_break) };
                        __state = 108;
                    }
                    108 => { __state = 2; }
                    109 => {
                        unsafe { (*p).p_order_by = p_order_by };
                        __state = 110;
                    }
                    110 => { unsafe { (*p).p_limit = p_limit }; __state = 111; }
                    111 => { return; }
                    _ => {}
                }
            }
        }
    }
}

///* Name of the connection operator, used for error messages.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_op_name(id: i32) -> *const i8 {
    let mut z: *mut i8 = core::ptr::null_mut();
    '__s77:
        {
        match id {
            136 => { z = c"UNION ALL".as_ptr() as *mut i8; }
            138 => { z = c"INTERSECT".as_ptr() as *mut i8; }
            137 => { z = c"EXCEPT".as_ptr() as *mut i8; }
            _ => { z = c"UNION".as_ptr() as *mut i8; }
        }
    }
    return z as *const i8;
}

///* Make a new pointer to a KeyInfo object
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_key_info_ref(p: *mut KeyInfo) -> *mut KeyInfo {
    if !(p).is_null() {
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p).n_ref };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    return p;
}

///* Code an output subroutine for a coroutine implementation of a
///* SELECT statement.
///*
///* The data to be output is contained in an array of pIn->nSdst registers
///* starting at register pIn->iSdst.  pDest is where the output should
///* be sent.
///*
///* regReturn is the number of the register holding the subroutine
///* return address.
///*
///* If regPrev>0 then it is the first register in a vector that
///* records the previous output.  mem[regPrev] is a flag that is false
///* if there has been no previous output.  If regPrev>0 then code is
///* generated to suppress duplicates.  pKeyInfo is used for comparing
///* keys.
///*
///* If the LIMIT found in p->iLimit is reached, jump immediately to
///* iBreak.
#[allow(unused_doc_comments)]
extern "C" fn generate_output_subroutine(p_parse_1: *mut Parse, p: &Select,
    p_in_1: &SelectDest, p_dest_1: &mut SelectDest, reg_return_1: i32,
    reg_prev_1: i32, p_key_info_1: *mut KeyInfo, i_break_1: i32) -> i32 {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i_continue: i32 = 0;
        let mut addr: i32 = 0;
        { let _ = 0; };
        addr = unsafe { sqlite3_vdbe_current_addr(v) };
        i_continue = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
        if reg_prev_1 != 0 {
            let mut addr1: i32 = 0;
            let mut addr2: i32 = 0;
            addr1 = unsafe { sqlite3_vdbe_add_op1(v, 17, reg_prev_1) };
            addr2 =
                unsafe {
                    sqlite3_vdbe_add_op4(v, 92, (*p_in_1).i_sdst,
                        reg_prev_1 + 1, (*p_in_1).n_sdst,
                        sqlite3_key_info_ref(p_key_info_1) as *mut i8 as *const i8,
                        -9)
                };
            unsafe {
                sqlite3_vdbe_add_op3(v, 14, addr2 + 2, i_continue, addr2 + 2)
            };
            unsafe { sqlite3_vdbe_jump_here(v, addr1) };
            unsafe {
                sqlite3_vdbe_add_op3(v, 82, (*p_in_1).i_sdst, reg_prev_1 + 1,
                    (*p_in_1).n_sdst - 1)
            };
            unsafe { sqlite3_vdbe_add_op2(v, 73, 1, reg_prev_1) };
        }
        if unsafe { (*unsafe { (*p_parse_1).db }).malloc_failed } != 0 {
            return 0;
        }

        /// Suppress the first OFFSET entries if there is an OFFSET clause
        code_offset(v, (*p).i_offset, i_continue);
        '__s78:
            {
            match (*p_dest_1).e_dest {
                6 => {
                    {
                        let mut r1: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let mut r2: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let i_parm: i32 = (*p_dest_1).i_sd_parm;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1)
                        };
                        if (*p_dest_1).e_dest as i32 == 3 {

                            /// If the destination is DistFifo, then cursor (iParm+1) is open
                            ///* on an ephemeral index that is used to enforce uniqueness on the
                            ///* total result.  At this point, we are processing the setup portion
                            ///* of the recursive CTE using the merge algorithm, so the results are
                            ///* guaranteed to be unique anyhow.  But we still need to populate the
                            ///* (iParm+1) cursor for use by the subsequent recursive phase.
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                        unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };

                        /// The LIMIT clause will jump out of the loop for us
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                3 => {
                    {
                        let mut r1: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let mut r2: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let i_parm: i32 = (*p_dest_1).i_sd_parm;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1)
                        };
                        if (*p_dest_1).e_dest as i32 == 3 {

                            /// If the destination is DistFifo, then cursor (iParm+1) is open
                            ///* on an ephemeral index that is used to enforce uniqueness on the
                            ///* total result.  At this point, we are processing the setup portion
                            ///* of the recursive CTE using the merge algorithm, so the results are
                            ///* guaranteed to be unique anyhow.  But we still need to populate the
                            ///* (iParm+1) cursor for use by the subsequent recursive phase.
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                        unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };

                        /// The LIMIT clause will jump out of the loop for us
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                12 => {
                    {
                        let mut r1: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let mut r2: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let i_parm: i32 = (*p_dest_1).i_sd_parm;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1)
                        };
                        if (*p_dest_1).e_dest as i32 == 3 {

                            /// If the destination is DistFifo, then cursor (iParm+1) is open
                            ///* on an ephemeral index that is used to enforce uniqueness on the
                            ///* total result.  At this point, we are processing the setup portion
                            ///* of the recursive CTE using the merge algorithm, so the results are
                            ///* guaranteed to be unique anyhow.  But we still need to populate the
                            ///* (iParm+1) cursor for use by the subsequent recursive phase.
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                        unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };

                        /// The LIMIT clause will jump out of the loop for us
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                10 => {
                    {
                        let mut r1: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let mut r2: i32 =
                            unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        let i_parm: i32 = (*p_dest_1).i_sd_parm;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1)
                        };
                        if (*p_dest_1).e_dest as i32 == 3 {

                            /// If the destination is DistFifo, then cursor (iParm+1) is open
                            ///* on an ephemeral index that is used to enforce uniqueness on the
                            ///* total result.  At this point, we are processing the setup portion
                            ///* of the recursive CTE using the merge algorithm, so the results are
                            ///* guaranteed to be unique anyhow.  But we still need to populate the
                            ///* (iParm+1) cursor for use by the subsequent recursive phase.
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm + 1, r1,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, r2) };
                        unsafe { sqlite3_vdbe_add_op3(v, 130, i_parm, r1, r2) };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r2) };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };

                        /// The LIMIT clause will jump out of the loop for us
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                1 => {
                    {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 1, (*p_dest_1).i_sd_parm)
                        };

                        /// The LIMIT clause will terminate the loop for us
                        break '__s78;
                    }
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };

                        /// The LIMIT clause will jump out of the loop for us
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                9 => {
                    {
                        let mut r1: i32 = 0;
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r1, (*p_dest_1).z_aff_sdst as *const i8,
                                (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, (*p_dest_1).i_sd_parm, r1,
                                (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        if (*p_dest_1).i_sd_parm2 > 0 {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 185, (*p_dest_1).i_sd_parm2, 0,
                                    (*p_in_1).i_sdst, (*p_in_1).n_sdst)
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"CREATE BLOOM FILTER".as_ptr() as *mut i8 as *const i8)
                            };
                        }
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        break '__s78;
                    }
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };

                        /// The LIMIT clause will jump out of the loop for us
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                8 => {
                    {
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sd_parm, (*p_in_1).n_sdst)
                        };

                        /// The LIMIT clause will jump out of the loop for us
                        break '__s78;
                    }
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                11 => {
                    {
                        if (*p_dest_1).i_sdst == 0 {
                            (*p_dest_1).i_sdst =
                                unsafe {
                                    sqlite3_get_temp_range(p_parse_1, (*p_in_1).n_sdst)
                                };
                            (*p_dest_1).n_sdst = (*p_in_1).n_sdst;
                        }
                        unsafe {
                            sqlite3_expr_code_move(p_parse_1, (*p_in_1).i_sdst,
                                (*p_dest_1).i_sdst, (*p_in_1).n_sdst)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                        };
                        break '__s78;
                    }
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                4 => {
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                5 => {
                    {
                        let mut n_key: i32 = 0;
                        let mut r1: i32 = 0;
                        let mut r2: i32 = 0;
                        let mut r3: i32 = 0;
                        let mut ii: i32 = 0;
                        let mut p_so: *const ExprList = core::ptr::null();
                        let i_parm_1: i32 = (*p_dest_1).i_sd_parm;
                        p_so = (*p_dest_1).p_order_by;
                        { let _ = 0; };
                        n_key = unsafe { (*p_so).n_expr };
                        r1 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        r2 =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_key + 2) };
                        r3 = r2 + n_key + 1;
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst, r3)
                        };
                        if (*p_dest_1).e_dest as i32 == 4 {
                            unsafe { sqlite3_vdbe_add_op2(v, 140, i_parm_1 + 1, r3) };
                        }
                        {
                            ii = 0;
                            '__b79: loop {
                                if !(ii < n_key) { break '__b79; }
                                '__c79: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op2(v, 83,
                                            (*p_in_1).i_sdst +
                                                    unsafe {
                                                            (*(unsafe { (*p_so).a.as_ptr() } as
                                                                                    *mut ExprListItem).offset(ii as isize)).u.x.i_order_by_col
                                                        } as i32 - 1, r2 + ii)
                                    };
                                    break '__c79;
                                }
                                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 128, i_parm_1, r2 + n_key)
                        };
                        unsafe { sqlite3_vdbe_add_op3(v, 99, r2, n_key + 2, r1) };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm_1, r1, r2,
                                n_key + 2)
                        };
                        unsafe { sqlite3_release_temp_reg(p_parse_1, r1) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, r2, n_key + 2)
                        };
                        break '__s78;
                    }
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                2 => {
                    { break '__s78; }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
                _ => {
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 86, (*p_in_1).i_sdst,
                                (*p_in_1).n_sdst)
                        };
                        break '__s78;
                    }
                }
            }
        }
        if (*p).i_limit != 0 {
            unsafe { sqlite3_vdbe_add_op2(v, 63, (*p).i_limit, i_break_1) };
        }

        /// Generate the subroutine return
        unsafe { sqlite3_vdbe_resolve_label(v, i_continue) };
        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_return_1) };
        return addr;
    }
}

///* Deallocate a KeyInfo object
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_key_info_unref(p: *mut KeyInfo) -> () {
    if !(p).is_null() {
        { let _ = 0; };
        { let _ = 0; };
        {
            let __p = unsafe { &mut (*p).n_ref };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p).n_ref } == 0 as u32 {
            unsafe {
                sqlite3_db_nn_free_nn(unsafe { (*p).db }, p as *mut ())
            };
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_delete_generic(db: *mut Sqlite3, p: *mut ())
    -> () {
    if !(p).is_null() { clear_select(db, p as *mut Select, 1); }
}

/// Forward references
#[allow(unused_doc_comments)]
extern "C" fn multi_select_by_merge(p_parse: *mut Parse, p: *mut Select,
    p_dest: *mut SelectDest) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        /// Loop counters
        let mut p_prior: *mut Select = core::ptr::null_mut();
        /// Another SELECT immediately to our left
        let mut p_split: *mut Select = core::ptr::null_mut();
        /// Left-most SELECT in the right-hand group
        let mut n_select: i32 = 0;
        /// Number of SELECT statements in the compound
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// Generate code to this VDBE
        let mut dest_a: SelectDest = unsafe { core::mem::zeroed() };
        /// Destination for coroutine A
        let mut dest_b: SelectDest = unsafe { core::mem::zeroed() };
        /// Destination for coroutine B
        let mut reg_addr_a: i32 = 0;
        /// Address register for select-A coroutine
        let mut reg_addr_b: i32 = 0;
        /// Address register for select-B coroutine
        let mut addr_select_a: i32 = 0;
        /// Address of the select-A coroutine
        let mut addr_select_b: i32 = 0;
        /// Address of the select-B coroutine
        let mut reg_out_a: i32 = 0;
        /// Address register for the output-A subroutine
        let mut reg_out_b: i32 = 0;
        /// Address register for the output-B subroutine
        let mut addr_out_a: i32 = 0;
        /// Address of the output-A subroutine
        let mut addr_out_b: i32 = 0;
        /// Address of the output-B subroutine
        let mut addr_eof_a: i32 = 0;
        /// Address of the select-A-exhausted subroutine
        let mut addr_eof_a_no_b: i32 = 0;
        /// Alternate addrEofA if B is uninitialized
        let mut addr_eof_b: i32 = 0;
        /// Address of the select-B-exhausted subroutine
        let mut addr_alt_b: i32 = 0;
        /// Address of the A<B subroutine
        let mut addr_aeq_b: i32 = 0;
        /// Address of the A==B subroutine
        let mut addr_agt_b: i32 = 0;
        /// Address of the A>B subroutine
        let mut reg_limit_a: i32 = 0;
        /// Limit register for select-A
        let mut reg_limit_b: i32 = 0;
        /// Limit register for select-A
        let mut reg_prev: i32 = 0;
        /// A range of registers to hold previous output
        let mut saved_limit: i32 = 0;
        /// Saved value of p->iLimit
        let mut saved_offset: i32 = 0;
        /// Saved value of p->iOffset
        let mut label_cmpr: i32 = 0;
        /// Label for the start of the merge algorithm
        let mut label_end: i32 = 0;
        /// Label for the end of the overall SELECT stmt
        let mut addr1: i32 = 0;
        /// Jump instructions that get retargeted
        let mut op: i32 = 0;
        /// One of TK_ALL, TK_UNION, TK_EXCEPT, TK_INTERSECT
        let mut p_key_dup: *mut KeyInfo = core::ptr::null_mut();
        /// Comparison information for duplicate removal
        let mut p_key_merge: *mut KeyInfo = core::ptr::null_mut();
        /// Comparison information for merging rows
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// Database connection
        let mut p_order_by: *mut ExprList = core::ptr::null_mut();
        /// The ORDER BY clause
        let mut n_order_by: i32 = 0;
        /// Number of terms in the ORDER BY clause
        let mut a_permute: *mut u32 = core::ptr::null_mut();

        /// Mapping from ORDER BY terms to result set columns
        { let _ = 0; };
        { let _ = 0; };

        /// "Managed" code needs this.  Ticket #3382.
        (db = unsafe { (*p_parse).db });
        v = unsafe { (*p_parse).p_vdbe };
        { let _ = 0; };

        /// Already thrown the error if VDBE alloc failed
        (label_end = unsafe { sqlite3_vdbe_make_label(p_parse) });
        label_cmpr = unsafe { sqlite3_vdbe_make_label(p_parse) };

        /// Patch up the ORDER BY clause
        (op = unsafe { (*p).op } as i32);
        { let _ = 0; };
        p_order_by = unsafe { (*p).p_order_by };
        { let _ = 0; };
        n_order_by = unsafe { (*p_order_by).n_expr };
        if op != 136 {
            {
                i = 1;
                '__b80: loop {
                    if !(unsafe { (*db).malloc_failed } as i32 == 0 &&
                                    i <= unsafe { (*unsafe { (*p).p_e_list }).n_expr }) {
                        break '__b80;
                    }
                    '__c80: loop {
                        let mut p_item: *const ExprListItem = core::ptr::null();
                        {
                            {
                                j = 0;
                                p_item =
                                    unsafe { (*p_order_by).a.as_ptr() } as *mut ExprListItem
                            };
                            '__b81: loop {
                                if !(j < n_order_by) { break '__b81; }
                                '__c81: loop {
                                    { let _ = 0; };
                                    { let _ = 0; };
                                    if unsafe { (*p_item).u.x.i_order_by_col } as i32 == i {
                                        break '__b81;
                                    }
                                    break '__c81;
                                }
                                {
                                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                    {
                                        let __p = &mut p_item;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                                };
                            }
                        }
                        if j == n_order_by {
                            let p_new: *mut Expr = unsafe { sqlite3_expr_int32(db, i) };
                            if p_new == core::ptr::null_mut() { return 7; }
                            unsafe {
                                (*p).p_order_by =
                                    {
                                        p_order_by =
                                            unsafe {
                                                sqlite3_expr_list_append(p_parse, p_order_by, p_new)
                                            };
                                        p_order_by
                                    }
                            };
                            if !(p_order_by).is_null() {
                                unsafe {
                                    (*(unsafe { (*p_order_by).a.as_ptr() } as
                                                                *mut ExprListItem).offset({
                                                                    let __p = &mut n_order_by;
                                                                    let __t = *__p;
                                                                    *__p += 1;
                                                                    __t
                                                                } as isize)).u.x.i_order_by_col = i as u16
                                };
                            }
                        }
                        break '__c80;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }

        /// Compute the comparison permutation and keyinfo that is used with
        ///* the permutation to determine if the next row of results comes
        ///* from selectA or selectB.  Also add literal collations to the
        ///* ORDER BY clause terms so that when selectA and selectB are
        ///* evaluated, they use the correct collation.
        (a_permute =
            unsafe {
                    sqlite3_db_malloc_raw_nn(db,
                        core::mem::size_of::<u32>() as u64 *
                            (n_order_by + 1) as u64)
                } as *mut u32);
        if !(a_permute).is_null() {
            let mut p_item_1: *const ExprListItem = core::ptr::null();
            let mut b_keep: i32 = 0;
            unsafe { *a_permute.offset(0 as isize) = n_order_by as u32 };
            {
                {
                    i = 1;
                    p_item_1 =
                        unsafe { (*p_order_by).a.as_ptr() } as *mut ExprListItem
                };
                '__b82: loop {
                    if !(i <= n_order_by) { break '__b82; }
                    '__c82: loop {
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        unsafe {
                            *a_permute.offset(i as isize) =
                                (unsafe { (*p_item_1).u.x.i_order_by_col } as i32 - 1) as
                                    u32
                        };
                        if unsafe { *a_permute.offset(i as isize) } !=
                                i as u32 - 1 as u32 {
                            b_keep = 1;
                        }
                        break '__c82;
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
            if b_keep == 0 {
                unsafe { sqlite3_db_free_nn(db, a_permute as *mut ()) };
                a_permute = core::ptr::null_mut();
            }
        }
        p_key_merge = multi_select_by_merge_key_info(p_parse, p, 1);
        if op == 136 {
            reg_prev = 0;
        } else {
            let n_expr: i32 = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
            { let _ = 0; };
            reg_prev = unsafe { (*p_parse).n_mem } + 1;
            unsafe { (*p_parse).n_mem += n_expr + 1 };
            unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_prev) };
            p_key_dup = sqlite3_key_info_alloc(db, n_expr, 1);
            if !(p_key_dup).is_null() {
                { let _ = 0; };
                {
                    i = 0;
                    '__b83: loop {
                        if !(i < n_expr) { break '__b83; }
                        '__c83: loop {
                            unsafe {
                                *(unsafe { (*p_key_dup).a_coll.as_ptr() } as
                                                *mut *mut CollSeq).offset(i as isize) =
                                    multi_select_coll_seq(p_parse, unsafe { &*p }, i)
                            };
                            unsafe {
                                *unsafe { (*p_key_dup).a_sort_flags.offset(i as isize) } =
                                    0 as u8
                            };
                            break '__c83;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
        }

        /// Separate the left and the right query from one another
        (n_select = 1);
        if (op == 136 || op == 135) &&
                unsafe { (*db).db_opt_flags } & 2097152 as u32 == 0 as u32 {
            {
                p_split = p;
                '__b84: loop {
                    if !(unsafe { (*p_split).p_prior } != core::ptr::null_mut()
                                    && unsafe { (*p_split).op } as i32 == op) {
                        break '__b84;
                    }
                    '__c84: loop {
                        { let __p = &mut n_select; let __t = *__p; *__p += 1; __t };
                        { let _ = 0; };
                        break '__c84;
                    }
                    p_split = unsafe { (*p_split).p_prior };
                }
            }
        }
        if n_select <= 3 {
            p_split = p;
        } else {
            p_split = p;
            {
                i = 2;
                '__b85: loop {
                    if !(i < n_select) { break '__b85; }
                    '__c85: loop {
                        p_split = unsafe { (*p_split).p_prior };
                        break '__c85;
                    }
                    i += 2;
                }
            }
        }
        p_prior = unsafe { (*p_split).p_prior };
        { let _ = 0; };
        unsafe { (*p_split).p_prior = core::ptr::null_mut() };
        unsafe { (*p_prior).p_next = core::ptr::null_mut() };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            (*p_prior).p_order_by =
                unsafe {
                    sqlite3_expr_list_dup(unsafe { (*p_parse).db },
                        p_order_by as *const ExprList, 0)
                }
        };
        unsafe {
            sqlite3_resolve_order_group_by(p_parse, p,
                unsafe { (*p).p_order_by },
                c"ORDER".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            sqlite3_resolve_order_group_by(p_parse, p_prior,
                unsafe { (*p_prior).p_order_by },
                c"ORDER".as_ptr() as *mut i8 as *const i8)
        };

        /// Compute the limit registers
        compute_limit_registers(p_parse, unsafe { &mut *p }, label_end);
        if unsafe { (*p).i_limit } != 0 && op == 136 {
            reg_limit_a =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                };
            reg_limit_b =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                };
            unsafe {
                sqlite3_vdbe_add_op2(v, 82,
                    if unsafe { (*p).i_offset } != 0 {
                        (unsafe { (*p).i_offset }) + 1
                    } else { unsafe { (*p).i_limit } }, reg_limit_a)
            };
            unsafe { sqlite3_vdbe_add_op2(v, 82, reg_limit_a, reg_limit_b) };
        } else { reg_limit_a = { reg_limit_b = 0; reg_limit_b }; }
        unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_limit }) };
        unsafe { (*p).p_limit = core::ptr::null_mut() };
        reg_addr_a =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        reg_addr_b =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        reg_out_a =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        reg_out_b =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        sqlite3_select_dest_init(&mut dest_a, 11, reg_addr_a);
        sqlite3_select_dest_init(&mut dest_b, 11, reg_addr_b);
        unsafe {
            sqlite3_vdbe_explain(p_parse, 1 as u8,
                c"MERGE (%s)".as_ptr() as *mut i8 as *const i8,
                sqlite3_select_op_name(unsafe { (*p).op } as i32))
        };

        /// Generate a coroutine to evaluate the SELECT statement to the
        ///* left of the compound operator - the "A" select.
        (addr_select_a = unsafe { sqlite3_vdbe_current_addr(v) } + 1);
        addr1 =
            unsafe {
                sqlite3_vdbe_add_op3(v, 11, reg_addr_a, 0, addr_select_a)
            };
        unsafe { (*p_prior).i_limit = reg_limit_a };
        unsafe {
            sqlite3_vdbe_explain(p_parse, 1 as u8,
                c"LEFT".as_ptr() as *mut i8 as *const i8)
        };
        sqlite3_select(p_parse, p_prior, &mut dest_a);
        unsafe { sqlite3_vdbe_end_coroutine(v, reg_addr_a) };
        unsafe { sqlite3_vdbe_jump_here(v, addr1) };

        /// Generate a coroutine to evaluate the SELECT statement on
        ///* the right - the "B" select
        (addr_select_b = unsafe { sqlite3_vdbe_current_addr(v) } + 1);
        addr1 =
            unsafe {
                sqlite3_vdbe_add_op3(v, 11, reg_addr_b, 0, addr_select_b)
            };
        saved_limit = unsafe { (*p).i_limit };
        saved_offset = unsafe { (*p).i_offset };
        unsafe { (*p).i_limit = reg_limit_b };
        unsafe { (*p).i_offset = 0 };
        unsafe {
            sqlite3_vdbe_explain(p_parse, 1 as u8,
                c"RIGHT".as_ptr() as *mut i8 as *const i8)
        };
        sqlite3_select(p_parse, p, &mut dest_b);
        unsafe { (*p).i_limit = saved_limit };
        unsafe { (*p).i_offset = saved_offset };
        unsafe { sqlite3_vdbe_end_coroutine(v, reg_addr_b) };
        addr_out_a =
            generate_output_subroutine(p_parse, unsafe { &*p }, &dest_a,
                unsafe { &mut *p_dest }, reg_out_a, reg_prev, p_key_dup,
                label_end);
        if op == 136 || op == 135 {
            addr_out_b =
                generate_output_subroutine(p_parse, unsafe { &*p }, &dest_b,
                    unsafe { &mut *p_dest }, reg_out_b, reg_prev, p_key_dup,
                    label_end);
        }
        sqlite3_key_info_unref(p_key_dup);
        if op == 137 || op == 138 {
            addr_eof_a_no_b = { addr_eof_a = label_end; addr_eof_a };
        } else {
            addr_eof_a =
                unsafe { sqlite3_vdbe_add_op2(v, 10, reg_out_b, addr_out_b) };
            addr_eof_a_no_b =
                unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_b, label_end) };
            unsafe { sqlite3_vdbe_goto(v, addr_eof_a) };
            unsafe {
                (*p).n_select_row =
                    unsafe {
                        sqlite3_log_est_add(unsafe { (*p).n_select_row },
                            unsafe { (*p_prior).n_select_row })
                    }
            };
        }
        if op == 138 {
            addr_eof_b = addr_eof_a;
            if unsafe { (*p).n_select_row } as i32 >
                    unsafe { (*p_prior).n_select_row } as i32 {
                unsafe {
                    (*p).n_select_row = unsafe { (*p_prior).n_select_row }
                };
            }
        } else {
            addr_eof_b =
                unsafe { sqlite3_vdbe_add_op2(v, 10, reg_out_a, addr_out_a) };
            unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_a, label_end) };
            unsafe { sqlite3_vdbe_goto(v, addr_eof_b) };
        }

        /// Generate code to handle the case of A<B
        (addr_alt_b =
            unsafe { sqlite3_vdbe_add_op2(v, 10, reg_out_a, addr_out_a) });
        unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_a, addr_eof_a) };
        unsafe { sqlite3_vdbe_goto(v, label_cmpr) };
        if op == 136 {
            addr_aeq_b = addr_alt_b;
        } else if op == 138 {
            addr_aeq_b = addr_alt_b;
            { let __p = &mut addr_alt_b; let __t = *__p; *__p += 1; __t };
        } else { addr_aeq_b = addr_alt_b + 1; }

        /// Generate code to handle the case of A>B
        (addr_agt_b = unsafe { sqlite3_vdbe_current_addr(v) });
        if op == 136 || op == 135 {
            unsafe { sqlite3_vdbe_add_op2(v, 10, reg_out_b, addr_out_b) };
            unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_b, addr_eof_b) };
            unsafe { sqlite3_vdbe_goto(v, label_cmpr) };
        } else {
            { let __p = &mut addr_agt_b; let __t = *__p; *__p += 1; __t };
        }

        /// This code runs once to initialize everything.
        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
        unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_a, addr_eof_a_no_b) };

        /// v---  Also the A>B case for EXCEPT and INTERSECT
        unsafe { sqlite3_vdbe_add_op2(v, 12, reg_addr_b, addr_eof_b) };
        if a_permute != core::ptr::null_mut() {
            unsafe {
                sqlite3_vdbe_add_op4(v, 91, 0, 0, 0,
                    a_permute as *mut i8 as *const i8, -15)
            };
        }
        unsafe { sqlite3_vdbe_resolve_label(v, label_cmpr) };
        unsafe {
            sqlite3_vdbe_add_op4(v, 92, dest_a.i_sdst, dest_b.i_sdst,
                n_order_by, p_key_merge as *mut i8 as *const i8, -9)
        };
        if a_permute != core::ptr::null_mut() {
            unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
        }
        unsafe {
            sqlite3_vdbe_add_op3(v, 14, addr_alt_b, addr_aeq_b, addr_agt_b)
        };

        /// Jump to the this point in order to terminate the query.
        unsafe { sqlite3_vdbe_resolve_label(v, label_end) };
        if !(unsafe { (*p_split).p_prior }).is_null() {
            unsafe {
                sqlite3_parser_add_cleanup(p_parse,
                    Some(sqlite3_select_delete_generic),
                    unsafe { (*p_split).p_prior } as *mut ())
            };
        }
        unsafe { (*p_split).p_prior = p_prior };
        unsafe { (*p_prior).p_next = p_split };
        unsafe {
            sqlite3_expr_list_delete(db, unsafe { (*p_prior).p_order_by })
        };
        unsafe { (*p_prior).p_order_by = core::ptr::null_mut() };

        ///TBD:  Insert subroutine calls to close cursors on incomplete
        ///*** subqueries ***
        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
        return (unsafe { (*p_parse).n_err } != 0) as i32;
    }
}

///* This routine is called to process a compound query form from
///* two or more separate queries using UNION, UNION ALL, EXCEPT, or
///* INTERSECT
///*
///* "p" points to the right-most of the two queries.  the query on the
///* left is p->pPrior.  The left query could also be a compound query
///* in which case this routine will be called recursively.
///*
///* The results of the total query are to be written into a destination
///* of type eDest with parameter iParm.
///*
///* Example 1:  Consider a three-way compound SQL statement.
///*
///*     SELECT a FROM t1 UNION SELECT b FROM t2 UNION SELECT c FROM t3
///*
///* This statement is parsed up as follows:
///*
///*     SELECT c FROM t3
///*      |
///*      `----->  SELECT b FROM t2
///*                |
///*                `------>  SELECT a FROM t1
///*
///* The arrows in the diagram above represent the Select.pPrior pointer.
///* So if this routine is called with p equal to the t3 query, then
///* pPrior will be the t2 query.  p->op will be TK_UNION in this case.
///*
///* Notice that because of the way SQLite parses compound SELECTs, the
///* individual selects always group from left to right.
#[allow(unused_doc_comments)]
extern "C" fn multi_select(p_parse_1: *mut Parse, p: *mut Select,
    p_dest_1: *mut SelectDest) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        /// Success code from a subroutine
        /// Another SELECT immediately to our left
        /// Generate code to this VDBE
        let mut dest: SelectDest = unsafe { core::mem::zeroed() };
        /// Alternative data destination
        let mut p_delete: *mut Select = core::ptr::null_mut();
        '__b86: loop {
            '__c86: loop {
                /// Success code from a subroutine
                let mut p_prior: *mut Select = core::ptr::null_mut();
                /// Another SELECT immediately to our left
                let mut v: *mut Vdbe = core::ptr::null_mut();
                /// Generate code to this VDBE
                /// Alternative data destination
                /// Chain of simple selects to delete
                let mut db: *mut Sqlite3 = core::ptr::null_mut();

                /// Database connection
                /// Make sure there is no ORDER BY or LIMIT clause on prior SELECTs.  Only
                ///* the last (right-most) SELECT in the series may have an ORDER BY or LIMIT.
                { let _ = 0; };

                /// Calling function guarantees this much
                { let _ = 0; };
                { let _ = 0; };
                db = unsafe { (*p_parse_1).db };
                p_prior = unsafe { (*p).p_prior };
                dest = unsafe { core::ptr::read(p_dest_1) };
                { let _ = 0; };
                { let _ = 0; };
                v = sqlite3_get_vdbe(p_parse_1);
                { let _ = 0; };
                if dest.e_dest as i32 == 10 {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 120, dest.i_sd_parm,
                            unsafe { (*unsafe { (*p).p_e_list }).n_expr })
                    };
                    dest.e_dest = 12 as u8;
                }
                if unsafe { (*p).sel_flags } & 1024 as u32 != 0 {
                    rc = multi_select_values(p_parse_1, p, &mut dest);
                    if rc >= 0 { break '__b86; }
                    rc = 0;
                }

                /// Make sure all SELECTs in the statement have the same number of elements
                ///* in their result sets.
                { let _ = 0; };
                { let _ = 0; };
                if unsafe { (*p).sel_flags } & 8192 as u32 != 0 as u32 &&
                        has_anchor(p as *const Select) != 0 {
                    generate_with_recursive_query(p_parse_1, p, &mut dest);
                } else if !(unsafe { (*p).p_order_by }).is_null() {

                    /// If the compound has an ORDER BY clause, then always use the merge
                    ///* algorithm.
                    return multi_select_by_merge(p_parse_1, p, p_dest_1);
                } else if unsafe { (*p).op } as i32 != 136 {
                    /// If the compound is EXCEPT, INTERSECT, or UNION (anything other than
                    ///* UNION ALL) then also always use the merge algorithm.  However, the
                    ///* multiSelectByMerge() routine requires that the compound have an
                    ///* ORDER BY clause, and it doesn't right now.  So invent one first.
                    let p_one: *mut Expr = unsafe { sqlite3_expr_int32(db, 1) };
                    unsafe {
                        (*p).p_order_by =
                            unsafe {
                                sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                                    p_one)
                            }
                    };
                    if unsafe { (*p_parse_1).n_err } != 0 { break '__b86; }
                    { let _ = 0; };
                    unsafe {
                        (*(unsafe { (*unsafe { (*p).p_order_by }).a.as_ptr() } as
                                                    *mut ExprListItem).offset(0 as isize)).u.x.i_order_by_col =
                            1 as u16
                    };
                    return multi_select_by_merge(p_parse_1, p, p_dest_1);
                } else {
                    /// For a UNION ALL compound without ORDER BY, simply run the left
                    ///* query, then run the right query
                    let mut addr: i32 = 0;
                    let mut n_limit: i32 = 0;
                    if unsafe { (*p_prior).p_prior } == core::ptr::null_mut() {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                                c"COMPOUND QUERY".as_ptr() as *mut i8 as *const i8)
                        };
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                                c"LEFT-MOST SUBQUERY".as_ptr() as *mut i8 as *const i8)
                        };
                    }
                    { let _ = 0; };
                    unsafe { (*p_prior).i_limit = unsafe { (*p).i_limit } };
                    unsafe { (*p_prior).i_offset = unsafe { (*p).i_offset } };
                    unsafe {
                        (*p_prior).p_limit =
                            unsafe {
                                sqlite3_expr_dup(db, unsafe { (*p).p_limit } as *const Expr,
                                    0)
                            }
                    };
                    rc = sqlite3_select(p_parse_1, p_prior, &mut dest);
                    unsafe {
                        sqlite3_expr_delete(db, unsafe { (*p_prior).p_limit })
                    };
                    unsafe { (*p_prior).p_limit = core::ptr::null_mut() };
                    if rc != 0 { break '__b86; }
                    unsafe { (*p).p_prior = core::ptr::null_mut() };
                    unsafe { (*p).i_limit = unsafe { (*p_prior).i_limit } };
                    unsafe { (*p).i_offset = unsafe { (*p_prior).i_offset } };
                    if unsafe { (*p).i_limit } != 0 {
                        addr =
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 17, unsafe { (*p).i_limit })
                            };
                        if unsafe { (*p).i_offset } != 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 162, unsafe { (*p).i_limit },
                                    unsafe { (*p).i_offset } + 1, unsafe { (*p).i_offset })
                            };
                        }
                    }
                    unsafe {
                        sqlite3_vdbe_explain(p_parse_1, 1 as u8,
                            c"UNION ALL".as_ptr() as *mut i8 as *const i8)
                    };
                    rc = sqlite3_select(p_parse_1, p, &mut dest);
                    p_delete = unsafe { (*p).p_prior };
                    unsafe { (*p).p_prior = p_prior };
                    unsafe {
                        (*p).n_select_row =
                            unsafe {
                                sqlite3_log_est_add(unsafe { (*p).n_select_row },
                                    unsafe { (*p_prior).n_select_row })
                            }
                    };
                    if !(unsafe { (*p).p_limit }).is_null() &&
                                    unsafe {
                                            sqlite3_expr_is_integer(unsafe {
                                                        (*unsafe { (*p).p_limit }).p_left
                                                    } as *const Expr, &mut n_limit, p_parse_1)
                                        } != 0 && n_limit > 0 &&
                            unsafe { (*p).n_select_row } as i32 >
                                unsafe { sqlite3_log_est(n_limit as u64) } as i32 {
                        unsafe {
                            (*p).n_select_row =
                                unsafe { sqlite3_log_est(n_limit as u64) }
                        };
                    }
                    if addr != 0 { unsafe { sqlite3_vdbe_jump_here(v, addr) }; }
                    if unsafe { (*p).p_next } == core::ptr::null_mut() {
                        unsafe { sqlite3_vdbe_explain_pop(p_parse_1) };
                    }
                }
                break '__c86;
            }
            if !(false) { break '__b86; }
        }

        /// Success code from a subroutine
        /// Another SELECT immediately to our left
        /// Generate code to this VDBE
        /// Alternative data destination
        /// Chain of simple selects to delete
        /// Database connection
        /// Make sure there is no ORDER BY or LIMIT clause on prior SELECTs.  Only
        ///* the last (right-most) SELECT in the series may have an ORDER BY or LIMIT.
        /// Calling function guarantees this much
        /// The VDBE already created by calling function
        /// Create the destination temporary table if necessary
        /// Special handling for a compound-select that originates as a VALUES clause.
        /// Make sure all SELECTs in the statement have the same number of elements
        ///* in their result sets.
        /// If the compound has an ORDER BY clause, then always use the merge
        ///* algorithm.
        /// If the compound is EXCEPT, INTERSECT, or UNION (anything other than
        ///* UNION ALL) then also always use the merge algorithm.  However, the
        ///* multiSelectByMerge() routine requires that the compound have an
        ///* ORDER BY clause, and it doesn't right now.  So invent one first.
        /// For a UNION ALL compound without ORDER BY, simply run the left
        ///* query, then run the right query
        /// Initialize to suppress harmless compiler warning
        unsafe { (*p_dest_1).i_sdst = dest.i_sdst };
        unsafe { (*p_dest_1).n_sdst = dest.n_sdst };
        unsafe { (*p_dest_1).i_sd_parm2 = dest.i_sd_parm2 };
        if !(p_delete).is_null() {
            unsafe {
                sqlite3_parser_add_cleanup(p_parse_1,
                    Some(sqlite3_select_delete_generic), p_delete as *mut ())
            };
        }
        return rc;
    }
}

///* Argument pWhere is the WHERE clause belonging to SELECT statement p. This
///* function attempts to transform expressions of the form:
///*
///*     EXISTS (SELECT ...)
///*
///* into joins. For example, given
///*
///*    CREATE TABLE sailors(sid INTEGER PRIMARY KEY, name TEXT);
///*    CREATE TABLE reserves(sid INT, day DATE, PRIMARY KEY(sid, day));
///*
///*    SELECT name FROM sailors AS S WHERE EXISTS (
///*      SELECT * FROM reserves AS R WHERE S.sid = R.sid AND R.day = '2022-10-25'
///*    );
///*
///* the SELECT statement may be transformed as follows:
///*
///*    SELECT name FROM sailors AS S, reserves AS R
///*      WHERE S.sid = R.sid AND R.day = '2022-10-25';
///*
///* **Approximately**.  Really, we have to ensure that the FROM-clause term
///* that was formerly inside the EXISTS is only executed once.  This is handled
///* by setting the SrcItem.fg.fromExists flag, which then causes code in
///* the where.c file to exit the corresponding loop after the first successful
///* match (if any).
#[allow(unused_doc_comments)]
extern "C" fn exists_to_join(p_parse_1: *mut Parse, p: *mut Select,
    p_where_1: *mut Expr) -> () {
    unsafe {
        if unsafe { (*p_parse_1).n_err } == 0 &&
                                p_where_1 != core::ptr::null_mut() &&
                            !(unsafe { (*p_where_1).flags } & (1 | 2) as u32 !=
                                            0 as u32) as i32 != 0 &&
                        unsafe { (*p).p_src } != core::ptr::null_mut() &&
                    unsafe { (*unsafe { (*p).p_src }).n_src } <
                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32
                &&
                (unsafe { (*p).p_limit } == core::ptr::null_mut() ||
                    unsafe { (*unsafe { (*p).p_limit }).p_right } ==
                        core::ptr::null_mut()) {
            if unsafe { (*p_where_1).op } as i32 == 44 {
                let p_right: *mut Expr = unsafe { (*p_where_1).p_right };
                exists_to_join(p_parse_1, p, unsafe { (*p_where_1).p_left });
                exists_to_join(p_parse_1, p, p_right);
            } else if unsafe { (*p_where_1).op } as i32 == 20 {
                let p_sub: *mut Select = unsafe { (*p_where_1).x.p_select };
                let p_sub_where: *mut Expr = unsafe { (*p_sub).p_where };
                if unsafe { (*unsafe { (*p_sub).p_src }).n_src } == 1 &&
                                    unsafe { (*p_sub).sel_flags } & 8 as u32 == 0 as u32 &&
                                (unsafe {
                                                (*(unsafe { (*unsafe { (*p_sub).p_src }).a.as_ptr() } as
                                                                    *mut SrcItem).offset(0 as isize)).fg.is_subquery()
                                            } == 0) as i32 != 0 &&
                            unsafe { (*p_sub).p_limit } == core::ptr::null_mut() &&
                        unsafe { (*p_sub).p_prior } == core::ptr::null_mut() {
                    /// Before combining the sub-select with the parent, renumber the 
                    ///* cursor used by the subselect. This is because the EXISTS expression
                    ///* might be a copy of another EXISTS expression from somewhere
                    ///* else in the tree, and in this case it is important that it use
                    ///* a unique cursor number.
                    let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
                    let a_csr_map: *mut i32 =
                        unsafe {
                                sqlite3_db_malloc_zero(db,
                                    (unsafe { (*p_parse_1).n_tab } + 2) as u64 *
                                        core::mem::size_of::<i32>() as u64)
                            } as *mut i32;
                    if a_csr_map == core::ptr::null_mut() { return; }
                    unsafe {
                        *a_csr_map.offset(0 as isize) =
                            unsafe { (*p_parse_1).n_tab } + 1
                    };
                    renumber_cursors(p_parse_1, p_sub, -1, a_csr_map);
                    unsafe { sqlite3_db_free(db, a_csr_map as *mut ()) };
                    unsafe {
                        memset(p_where_1 as *mut (), 0,
                            core::mem::size_of::<Expr>() as u64)
                    };
                    unsafe { (*p_where_1).op = 156 as u8 };
                    unsafe { (*p_where_1).u.i_value = 1 };
                    unsafe { (*p_where_1).flags |= 2048 as u32 };
                    { let _ = 0; };
                    unsafe {
                        (*(unsafe { (*unsafe { (*p_sub).p_src }).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as
                                            isize)).fg.set_from_exists(1 as u32 as u32)
                    };
                    unsafe {
                        (*p).p_src =
                            unsafe {
                                sqlite3_src_list_append_list(p_parse_1,
                                    unsafe { (*p).p_src }, unsafe { (*p_sub).p_src })
                            }
                    };
                    if !(p_sub_where).is_null() {
                        unsafe {
                            (*p).p_where =
                                unsafe {
                                    sqlite3_p_expr(p_parse_1, 44, unsafe { (*p).p_where },
                                        p_sub_where)
                                }
                        };
                        unsafe { (*p_sub).p_where = core::ptr::null_mut() };
                    }
                    unsafe { (*p_sub).p_src = core::ptr::null_mut() };
                    unsafe {
                        sqlite3_parser_add_cleanup(p_parse_1,
                            Some(sqlite3_select_delete_generic), p_sub as *mut ())
                    };
                }
            }
        }
    }
}

///* Add a new entry to the pConst object.  Except, do not add duplicate
///* pColumn entries.  Also, do not add if doing so would not be appropriate.
///*
///* The caller guarantees the pColumn is a column and pValue is a constant.
///* This routine has to do some additional checks before completing the
///* insert.
extern "C" fn const_insert(p_const_1: &mut WhereConst, p_column_1: *mut Expr,
    p_value_1: *mut Expr, p_expr_1: *const Expr) -> () {
    let mut i: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_column_1).flags } & 32 as u32 != 0 as u32 { return; }
    if unsafe { sqlite3_expr_affinity(p_value_1 as *const Expr) } as i32 != 0
        {
        return;
    }
    if (unsafe {
                        sqlite3_is_binary(unsafe {
                                    sqlite3_expr_compare_coll_seq((*p_const_1).p_parse,
                                        p_expr_1 as *const Expr)
                                } as *const CollSeq)
                    } == 0) as i32 != 0 {
        return;
    }
    {
        i = 0;
        '__b87: loop {
            if !(i < (*p_const_1).n_const) { break '__b87; }
            '__c87: loop {
                let p_e2: *const Expr =
                    unsafe { *(*p_const_1).ap_expr.offset((i * 2) as isize) } as
                        *const Expr;
                { let _ = 0; };
                if unsafe { (*p_e2).i_table } as i32 ==
                            unsafe { (*p_column_1).i_table } &&
                        unsafe { (*p_e2).i_column } as i32 ==
                            unsafe { (*p_column_1).i_column } as i32 {
                    return;
                }
                break '__c87;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    { let _ = 0; };
    if unsafe { sqlite3_expr_affinity(p_column_1 as *const Expr) } as i32 <=
            65 {
        (*p_const_1).b_has_aff_blob = 1;
    }
    { let __p = &mut (*p_const_1).n_const; let __t = *__p; *__p += 1; __t };
    (*p_const_1).ap_expr =
        unsafe {
                sqlite3_db_realloc_or_free(unsafe {
                        (*(*p_const_1).p_parse).db
                    }, (*p_const_1).ap_expr as *mut (),
                    ((*p_const_1).n_const * 2) as u64 *
                        core::mem::size_of::<*mut Expr>() as u64)
            } as *mut *mut Expr;
    if (*p_const_1).ap_expr == core::ptr::null_mut() {
        (*p_const_1).n_const = 0;
    } else {
        unsafe {
            *(*p_const_1).ap_expr.offset(((*p_const_1).n_const * 2 - 2) as
                            isize) = p_column_1
        };
        unsafe {
            *(*p_const_1).ap_expr.offset(((*p_const_1).n_const * 2 - 1) as
                            isize) = p_value_1
        };
    }
}

///* Find all terms of COLUMN=VALUE or VALUE=COLUMN in pExpr where VALUE
///* is a constant expression and where the term must be true because it
///* is part of the AND-connected terms of the expression.  For each term
///* found, add it to the pConst structure.
extern "C" fn find_const_in_where(p_const_1: *mut WhereConst,
    p_expr_1: *mut Expr) -> () {
    let mut p_right: *mut Expr = core::ptr::null_mut();
    let mut p_left: *mut Expr = core::ptr::null_mut();
    if p_expr_1 == core::ptr::null_mut() { return; }
    if unsafe { (*p_expr_1).flags } &
                unsafe { (*p_const_1).m_exclude_on } as u32 != 0 as u32 {
        return;
    }
    if unsafe { (*p_expr_1).op } as i32 == 44 {
        find_const_in_where(p_const_1, unsafe { (*p_expr_1).p_right });
        find_const_in_where(p_const_1, unsafe { (*p_expr_1).p_left });
        return;
    }
    if unsafe { (*p_expr_1).op } as i32 != 54 { return; }
    p_right = unsafe { (*p_expr_1).p_right };
    p_left = unsafe { (*p_expr_1).p_left };
    { let _ = 0; };
    { let _ = 0; };
    if unsafe { (*p_right).op } as i32 == 168 &&
            unsafe {
                    sqlite3_expr_is_constant(unsafe { (*p_const_1).p_parse },
                        p_left)
                } != 0 {
        const_insert(unsafe { &mut *p_const_1 }, p_right, p_left,
            p_expr_1 as *const Expr);
    }
    if unsafe { (*p_left).op } as i32 == 168 &&
            unsafe {
                    sqlite3_expr_is_constant(unsafe { (*p_const_1).p_parse },
                        p_right)
                } != 0 {
        const_insert(unsafe { &mut *p_const_1 }, p_left, p_right,
            p_expr_1 as *const Expr);
    }
}

///* This is a helper function for Walker callback propagateConstantExprRewrite().
///*
///* Argument pExpr is a candidate expression to be replaced by a value. If
///* pExpr is equivalent to one of the columns named in pWalker->u.pConst,
///* then overwrite it with the corresponding value. Except, do not do so
///* if argument bIgnoreAffBlob is non-zero and the affinity of pExpr
///* is SQLITE_AFF_BLOB.
#[allow(unused_doc_comments)]
extern "C" fn propagate_constant_expr_rewrite_one(p_const_1: &mut WhereConst,
    p_expr_1: *mut Expr, b_ignore_aff_blob_1: i32) -> i32 {
    let mut i: i32 = 0;
    if unsafe { *(*p_const_1).p_oom_fault.offset(0 as isize) } != 0 {
        return 1;
    }
    if unsafe { (*p_expr_1).op } as i32 != 168 { return 0; }
    if unsafe { (*p_expr_1).flags } &
                (32 as u32 | (*p_const_1).m_exclude_on) as u32 != 0 as u32 {
        return 0;
    }
    {
        i = 0;
        '__b88: loop {
            if !(i < (*p_const_1).n_const) { break '__b88; }
            '__c88: loop {
                let p_column: *mut Expr =
                    unsafe { *(*p_const_1).ap_expr.offset((i * 2) as isize) };
                if p_column == p_expr_1 { break '__c88; }
                if unsafe { (*p_column).i_table } !=
                        unsafe { (*p_expr_1).i_table } {
                    break '__c88;
                }
                if unsafe { (*p_column).i_column } as i32 !=
                        unsafe { (*p_expr_1).i_column } as i32 {
                    break '__c88;
                }
                { let _ = 0; };
                if b_ignore_aff_blob_1 != 0 &&
                        unsafe { sqlite3_expr_affinity(p_column as *const Expr) } as
                                i32 <= 65 {
                    break '__b88;
                }

                /// A match is found.  Add the EP_FixedCol property
                {
                    let __p = &mut (*p_const_1).n_chng;
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                unsafe { (*p_expr_1).flags &= !(8388608 as u32) };
                unsafe { (*p_expr_1).flags |= 32 as u32 };
                { let _ = 0; };
                unsafe {
                    (*p_expr_1).p_left =
                        unsafe {
                            sqlite3_expr_dup(unsafe { (*(*p_const_1).p_parse).db },
                                unsafe {
                                        *(*p_const_1).ap_expr.offset((i * 2 + 1) as isize)
                                    } as *const Expr, 0)
                        }
                };
                if unsafe {
                            (*unsafe { (*(*p_const_1).p_parse).db }).malloc_failed
                        } != 0 {
                    return 1;
                }
                break '__b88;
                break '__c88;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}

///* This is a Walker expression callback. pExpr is a node from the WHERE
///* clause of a SELECT statement. This function examines pExpr to see if
///* any substitutions based on the contents of pWalker->u.pConst should
///* be made to pExpr or its immediate children.
///*
///* A substitution is made if:
///*
///*   + pExpr is a column with an affinity other than BLOB that matches
///*     one of the columns in pWalker->u.pConst, or
///*
///*   + pExpr is a binary comparison operator (=, <=, >=, <, >) that
///*     uses an affinity other than TEXT and one of its immediate
///*     children is a column that matches one of the columns in
///*     pWalker->u.pConst.
extern "C" fn propagate_constant_expr_rewrite(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let p_const: *mut WhereConst = unsafe { (*p_walker_1).u.p_const };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_const).b_has_aff_blob } != 0 {
            if unsafe { (*p_expr_1).op } as i32 >= 54 &&
                        unsafe { (*p_expr_1).op } as i32 <= 58 ||
                    unsafe { (*p_expr_1).op } as i32 == 45 {
                propagate_constant_expr_rewrite_one(unsafe { &mut *p_const },
                    unsafe { (*p_expr_1).p_left }, 0);
                if unsafe {
                            *unsafe { (*p_const).p_oom_fault.offset(0 as isize) }
                        } != 0 {
                    return 1;
                }
                if unsafe {
                                sqlite3_expr_affinity(unsafe { (*p_expr_1).p_left } as
                                        *const Expr)
                            } as i32 != 66 {
                    propagate_constant_expr_rewrite_one(unsafe {
                            &mut *p_const
                        }, unsafe { (*p_expr_1).p_right }, 0);
                }
            }
        }
        return propagate_constant_expr_rewrite_one(unsafe { &mut *p_const },
                p_expr_1, unsafe { (*p_const).b_has_aff_blob });
    }
}

///* The WHERE-clause constant propagation optimization.
///*
///* If the WHERE clause contains terms of the form COLUMN=CONSTANT or
///* CONSTANT=COLUMN that are top-level AND-connected terms that are not
///* part of a ON clause from a LEFT JOIN, then throughout the query
///* replace all other occurrences of COLUMN with CONSTANT.
///*
///* For example, the query:
///*
///*      SELECT * FROM t1, t2, t3 WHERE t1.a=39 AND t2.b=t1.a AND t3.c=t2.b
///*
///* Is transformed into
///*
///*      SELECT * FROM t1, t2, t3 WHERE t1.a=39 AND t2.b=39 AND t3.c=39
///*
///* Return true if any transformations where made and false if not.
///*
///* Implementation note:  Constant propagation is tricky due to affinity
///* and collating sequence interactions.  Consider this example:
///*
///*    CREATE TABLE t1(a INT,b TEXT);
///*    INSERT INTO t1 VALUES(123,'0123');
///*    SELECT * FROM t1 WHERE a=123 AND b=a;
///*    SELECT * FROM t1 WHERE a=123 AND b=123;
///*
///* The two SELECT statements above should return different answers.  b=a
///* is always true because the comparison uses numeric affinity, but b=123
///* is false because it uses text affinity and '0123' is not the same as '123'.
///* To work around this, the expression tree is not actually changed from
///* "b=a" to "b=123" but rather the "a" in "b=a" is tagged with EP_FixedCol
///* and the "123" value is hung off of the pLeft pointer.  Code generator
///* routines know to generate the constant "123" instead of looking up the
///* column value.  Also, to avoid collation problems, this optimization is
///* only attempted if the "a=123" term uses the default BINARY collation.
///*
///* 2021-05-25 forum post 6a06202608: Another troublesome case is...
///*
///*    CREATE TABLE t1(x);
///*    INSERT INTO t1 VALUES(10.0);
///*    SELECT 1 FROM t1 WHERE x=10 AND x LIKE 10;
///*
///* The query should return no rows, because the t1.x value is '10.0' not '10'
///* and '10.0' is not LIKE '10'.  But if we are not careful, the first WHERE
///* term "x=10" will cause the second WHERE term to become "10 LIKE 10",
///* resulting in a false positive.  To avoid this, constant propagation for
///* columns with BLOB affinity is only allowed if the constant is used with
///* operators ==, <=, <, >=, >, or IS in a way that will cause the correct
///* type conversions to occur.  See logic associated with the bHasAffBlob flag
///* for details.
#[allow(unused_doc_comments)]
extern "C" fn propagate_constants(p_parse_1: *mut Parse, p: &Select) -> i32 {
    unsafe {
        let mut x: WhereConst = unsafe { core::mem::zeroed() };
        let mut w: Walker = unsafe { core::mem::zeroed() };
        let mut n_chng: i32 = 0;
        x.p_parse = p_parse_1;
        x.p_oom_fault =
            unsafe { &mut (*unsafe { (*p_parse_1).db }).malloc_failed };
        '__b89: loop {
            '__c89: loop {
                x.n_const = 0;
                x.n_chng = 0;
                x.ap_expr = core::ptr::null_mut();
                x.b_has_aff_blob = 0;
                if (*p).p_src != core::ptr::null_mut() &&
                            unsafe { (*(*p).p_src).n_src } > 0 &&
                        unsafe {
                                        (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                                            *mut SrcItem).offset(0 as isize)).fg.jointype
                                    } as i32 & 64 != 0 {

                    /// Do not propagate constants on any ON clause if there is a
                    ///* RIGHT JOIN anywhere in the query
                    (x.m_exclude_on = (2 | 1) as u32);
                } else {

                    /// Do not propagate constants through the ON clause of a LEFT JOIN
                    (x.m_exclude_on = 1 as u32);
                }
                find_const_in_where(&mut x, (*p).p_where);
                if x.n_const != 0 {
                    unsafe {
                        memset(&raw mut w as *mut (), 0,
                            core::mem::size_of::<Walker>() as u64)
                    };
                    w.p_parse = p_parse_1;
                    w.x_expr_callback = Some(propagate_constant_expr_rewrite);
                    w.x_select_callback = Some(sqlite3_select_walk_noop);
                    w.x_select_callback2 = None;
                    w.walker_depth = 0;
                    w.u.p_const = &mut x;
                    unsafe { sqlite3_walk_expr(&mut w, (*p).p_where) };
                    unsafe {
                        sqlite3_db_free(unsafe { (*x.p_parse).db },
                            x.ap_expr as *mut ())
                    };
                    n_chng += x.n_chng;
                }
                break '__c89;
            }
            if !(x.n_chng != 0) { break '__b89; }
        }
        return n_chng;
    }
}

///* Attempt to transform a query of the form
///*
///*    SELECT count(*) FROM (SELECT x FROM t1 UNION ALL SELECT y FROM t2)
///*
///* Into this:
///*
///*    SELECT (SELECT count(*) FROM t1)+(SELECT count(*) FROM t2)
///*
///* The transformation only works if all of the following are true:
///*
///*   *  The subquery is a UNION ALL of two or more terms
///*   *  The subquery does not have a LIMIT clause
///*   *  There is no WHERE or GROUP BY or HAVING clauses on the subqueries
///*   *  The outer query is a simple count(*) with no WHERE clause or other
///*      extraneous syntax.
///*   *  None of the subqueries are DISTINCT (forumpost/a860f5fb2e 2025-03-10)
///*
///* Return TRUE if the optimization is undertaken.
#[allow(unused_doc_comments)]
extern "C" fn count_of_view_optimization(p_parse_1: *mut Parse,
    p: &mut Select) -> i32 {
    unsafe {
        let mut p_sub: *mut Select = core::ptr::null_mut();
        let mut p_prior: *mut Select = core::ptr::null_mut();
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        let mut p_count: *mut Expr = core::ptr::null_mut();
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_from: *mut SrcItem = core::ptr::null_mut();
        if (*p).sel_flags & 8 as u32 == 0 as u32 { return 0; }
        if unsafe { (*(*p).p_e_list).n_expr } != 1 { return 0; }
        if !((*p).p_where).is_null() { return 0; }
        if !((*p).p_having).is_null() { return 0; }
        if !((*p).p_group_by).is_null() { return 0; }
        if !((*p).p_order_by).is_null() { return 0; }
        p_expr =
            unsafe {
                (*(unsafe { (*(*p).p_e_list).a.as_ptr() } as
                                *mut ExprListItem).offset(0 as isize)).p_expr
            };
        if unsafe { (*p_expr).op } as i32 != 169 { return 0; }

        /// Result is an aggregate
        { let _ = 0; };
        if unsafe {
                    sqlite3_stricmp(unsafe { (*p_expr).u.z_token } as *const i8,
                        c"count".as_ptr() as *mut i8 as *const i8)
                } != 0 {
            return 0;
        }

        /// Is count()
        { let _ = 0; };
        if unsafe { (*p_expr).x.p_list } != core::ptr::null_mut() {
            return 0;
        }
        if unsafe { (*(*p).p_src).n_src } != 1 { return 0; }
        if unsafe { (*p_expr).flags } & 16777216 as u32 != 0 as u32 {
            return 0;
        }

        /// Not a window function
        (p_from = unsafe { (*(*p).p_src).a.as_ptr() } as *mut SrcItem);
        if unsafe { (*p_from).fg.is_subquery() } as i32 == 0 { return 0; }

        /// FROM is a subquery
        (p_sub = unsafe { (*unsafe { (*p_from).u4.p_subq }).p_select });
        if unsafe { (*p_sub).p_prior } == core::ptr::null_mut() { return 0; }
        if unsafe { (*p_sub).sel_flags } & 67108864 as u32 != 0 { return 0; }
        '__b90: loop {
            '__c90: loop {
                if unsafe { (*p_sub).op } as i32 != 136 &&
                        !(unsafe { (*p_sub).p_prior }).is_null() {
                    return 0;
                }
                if !(unsafe { (*p_sub).p_where }).is_null() { return 0; }
                if !(unsafe { (*p_sub).p_limit }).is_null() { return 0; }
                if unsafe { (*p_sub).sel_flags } & (8 | 1) as u32 != 0 {
                    return 0;
                }
                { let _ = 0; };

                /// Due to the previous
                (p_sub = unsafe { (*p_sub).p_prior });
                break '__c90;
            }
            if !(!(p_sub).is_null()) { break '__b90; }
        }

        /// If we reach this point then it is OK to perform the transformation
        (db = unsafe { (*p_parse_1).db });
        p_count = p_expr;
        p_expr = core::ptr::null_mut();
        p_sub = unsafe { sqlite3_subquery_detach(db, p_from) };
        unsafe { sqlite3_src_list_delete(db, (*p).p_src) };
        (*p).p_src =
            unsafe {
                    sqlite3_db_malloc_zero(unsafe { (*p_parse_1).db },
                        core::mem::offset_of!(SrcList, a) as u64 +
                            core::mem::size_of::<SrcItem>() as u64)
                } as *mut SrcList;
        while !(p_sub).is_null() {
            let mut p_term: *mut Expr = core::ptr::null_mut();
            p_prior = unsafe { (*p_sub).p_prior };
            unsafe { (*p_sub).p_prior = core::ptr::null_mut() };
            unsafe { (*p_sub).p_next = core::ptr::null_mut() };
            unsafe { (*p_sub).sel_flags |= 8 as u32 };
            unsafe { (*p_sub).sel_flags &= !(256 as u32) };
            unsafe { (*p_sub).n_select_row = 0 as LogEst };
            unsafe {
                sqlite3_parser_add_cleanup(p_parse_1,
                    Some(sqlite3_expr_list_delete_generic),
                    unsafe { (*p_sub).p_e_list } as *mut ())
            };
            p_term =
                if !(p_prior).is_null() {
                    unsafe { sqlite3_expr_dup(db, p_count as *const Expr, 0) }
                } else { p_count };
            unsafe {
                (*p_sub).p_e_list =
                    unsafe {
                        sqlite3_expr_list_append(p_parse_1, core::ptr::null_mut(),
                            p_term)
                    }
            };
            p_term =
                unsafe {
                    sqlite3_p_expr(p_parse_1, 139, core::ptr::null_mut(),
                        core::ptr::null_mut())
                };
            unsafe { sqlite3_p_expr_add_select(p_parse_1, p_term, p_sub) };
            if p_expr == core::ptr::null_mut() {
                p_expr = p_term;
            } else {
                p_expr =
                    unsafe { sqlite3_p_expr(p_parse_1, 107, p_term, p_expr) };
            }
            p_sub = p_prior;
        }
        unsafe {
            (*(unsafe { (*(*p).p_e_list).a.as_ptr() } as
                                *mut ExprListItem).offset(0 as isize)).p_expr = p_expr
        };
        (*p).sel_flags &= !(8 as u32);
        return 1;
    }
}

///* This function is called to determine whether or not it is safe to
///* push WHERE clause expression pExpr down to FROM clause sub-query
///* pSubq, which contains at least one window function. Return 1
///* if it is safe and the expression should be pushed down, or 0
///* otherwise.
///*
///* It is only safe to push the expression down if it consists only
///* of constants and copies of expressions that appear in the PARTITION
///* BY clause of all window function used by the sub-query. It is safe
///* to filter out entire partitions, but not rows within partitions, as
///* this may change the results of the window functions.
///*
///* At the time this function is called it is guaranteed that
///*
///*   * the sub-query uses only one distinct window frame, and
///*   * that the window frame has a PARTITION BY clause.
extern "C" fn push_down_window_check(p_parse_1: *mut Parse, p_subq_1: &Select,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        return unsafe {
                sqlite3_expr_is_constant_or_group_by(p_parse_1, p_expr_1,
                    unsafe { (*(*p_subq_1).p_win).p_partition })
            };
    }
}

///* Make copies of relevant WHERE clause terms of the outer query into
///* the WHERE clause of subquery.  Example:
///*
///*    SELECT * FROM (SELECT a AS x, c-d AS y FROM t1) WHERE x=5 AND y=10;
///*
///* Transformed into:
///*
///*    SELECT * FROM (SELECT a AS x, c-d AS y FROM t1 WHERE a=5 AND c-d=10)
///*     WHERE x=5 AND y=10;
///*
///* The hope is that the terms added to the inner query will make it more
///* efficient.
///*
///* NAME AMBIGUITY
///*
///* This optimization is called the "WHERE-clause push-down optimization"
///* or sometimes the "predicate push-down optimization".
///*
///* Do not confuse this optimization with another unrelated optimization
///* with a similar name:  The "MySQL push-down optimization" causes WHERE
///* clause terms that can be evaluated using only the index and without
///* reference to the table are run first, so that if they are false,
///* unnecessary table seeks are avoided.
///*
///* RULES
///*
///* Do not attempt this optimization if:
///*
///*   (1) (** This restriction was removed on 2017-09-29.  We used to
///*           disallow this optimization for aggregate subqueries, but now
///*           it is allowed by putting the extra terms on the HAVING clause.
///*           The added HAVING clause is pointless if the subquery lacks
///*           a GROUP BY clause.  But such a HAVING clause is also harmless
///*           so there does not appear to be any reason to add extra logic
///*           to suppress it. **)
///*
///*   (2) The inner query is the recursive part of a common table expression.
///*
///*   (3) The inner query has a LIMIT clause (since the changes to the WHERE
///*       clause would change the meaning of the LIMIT).
///*
///*   (4) The inner query is the right operand of a LEFT JOIN and the
///*       expression to be pushed down does not come from the ON clause
///*       on that LEFT JOIN.
///*
///*   (5) The WHERE clause expression originates in the ON or USING clause
///*       of a LEFT JOIN where iCursor is not the right-hand table of that
///*       left join.  An example:
///*
///*           SELECT *
///*           FROM (SELECT 1 AS a1 UNION ALL SELECT 2) AS aa
///*           JOIN (SELECT 1 AS b2 UNION ALL SELECT 2) AS bb ON (a1=b2)
///*           LEFT JOIN (SELECT 8 AS c3 UNION ALL SELECT 9) AS cc ON (b2=2);
///*
///*       The correct answer is three rows:  (1,1,NULL),(2,2,8),(2,2,9).
///*       But if the (b2=2) term were to be pushed down into the bb subquery,
///*       then the (1,1,NULL) row would be suppressed.
///*
///*   (6) Window functions make things tricky as changes to the WHERE clause
///*       of the inner query could change the window over which window
///*       functions are calculated. Therefore, do not attempt the optimization
///*       if:
///*
///*     (6a) The inner query uses multiple incompatible window partitions.
///*
///*     (6b) The inner query is a compound and uses window-functions.
///*
///*     (6c) The WHERE clause does not consist entirely of constants and
///*          copies of expressions found in the PARTITION BY clause of
///*          all window-functions used by the sub-query. It is safe to
///*          filter out entire partitions, as this does not change the
///*          window over which any window-function is calculated.
///*
///*   (7) The inner query is a Common Table Expression (CTE) that should
///*       be materialized.  (This restriction is implemented in the calling
///*       routine.)
///*
///*   (8) If the subquery is a compound that uses UNION, INTERSECT,
///*       or EXCEPT, then all of the result set columns for all arms of
///*       the compound must use the BINARY collating sequence.
///*
///*   (9) All three of the following are true:
///*
///*       (9a) The WHERE clause expression originates in the ON or USING clause
///*            of a join (either an INNER or an OUTER join), and
///*
///*       (9b) The subquery is to the right of the ON/USING clause
///*
///*       (9c) There is a RIGHT JOIN (or FULL JOIN) in between the ON/USING
///*            clause and the subquery.
///*
///*       Without this restriction, the WHERE-clause push-down optimization
///*       might move the ON/USING filter expression from the left side of a
///*       RIGHT JOIN over to the right side, which leads to incorrect answers.
///*       See also restriction (6) in sqlite3ExprIsSingleTableConstraint().
///*
///*  (10) The inner query is not the right-hand table of a RIGHT JOIN.
///*
///*  (11) The subquery is not a VALUES clause
///*
///*  (12) The WHERE clause is not "rowid ISNULL" or the equivalent.  This
///*       case only comes up if SQLite is compiled using
///*       SQLITE_ALLOW_ROWID_IN_VIEW.
///*
///* Return 0 if no changes are made and non-zero if one or more WHERE clause
///* terms are duplicated into the subquery.
#[allow(unused_doc_comments)]
extern "C" fn push_down_where_terms(p_parse_1: *mut Parse,
    mut p_subq_1: *mut Select, mut p_where_1: *mut Expr,
    p_src_list_1: *mut SrcList, i_src_1: i32) -> i32 {
    unsafe {
        let mut p_new: *mut Expr = core::ptr::null_mut();
        let mut p_src: *const SrcItem = core::ptr::null();
        /// The subquery FROM term into which WHERE is pushed
        let mut n_chng: i32 = 0;
        p_src =
            unsafe {
                &mut *(unsafe { (*p_src_list_1).a.as_ptr() } as
                                *mut SrcItem).offset(i_src_1 as isize)
            };
        if p_where_1 == core::ptr::null_mut() { return 0; }
        if unsafe { (*p_subq_1).sel_flags } & (8192 | 33554432) as u32 != 0 {
            return 0;
        }
        if unsafe { (*p_src).fg.jointype } as i32 & (64 | 16) != 0 {
            return 0;
        }
        if !(unsafe { (*p_subq_1).p_prior }).is_null() {
            let mut p_sel: *const Select = core::ptr::null();
            let mut not_union_all: i32 = 0;
            {
                p_sel = p_subq_1;
                '__b92: loop {
                    if !(!(p_sel).is_null()) { break '__b92; }
                    '__c92: loop {
                        let op: u8 = unsafe { (*p_sel).op };
                        { let _ = 0; };
                        if op as i32 != 136 && op as i32 != 139 {
                            not_union_all = 1;
                        }
                        if !(unsafe { (*p_sel).p_win }).is_null() { return 0; }
                        break '__c92;
                    }
                    p_sel = unsafe { (*p_sel).p_prior };
                }
            }
            if not_union_all != 0 {
                {
                    p_sel = p_subq_1;
                    '__b93: loop {
                        if !(!(p_sel).is_null()) { break '__b93; }
                        '__c93: loop {
                            let mut ii: i32 = 0;
                            let p_list: *const ExprList =
                                unsafe { (*p_sel).p_e_list } as *const ExprList;
                            { let _ = 0; };
                            {
                                ii = 0;
                                '__b94: loop {
                                    if !(ii < unsafe { (*p_list).n_expr }) { break '__b94; }
                                    '__c94: loop {
                                        let p_coll: *const CollSeq =
                                            unsafe {
                                                    sqlite3_expr_coll_seq(p_parse_1,
                                                        unsafe {
                                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                                *const ExprListItem).offset(ii as isize)).p_expr
                                                            } as *const Expr)
                                                } as *const CollSeq;
                                        if (unsafe { sqlite3_is_binary(p_coll as *const CollSeq) }
                                                        == 0) as i32 != 0 {
                                            return 0;
                                        }
                                        break '__c94;
                                    }
                                    { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            break '__c93;
                        }
                        p_sel = unsafe { (*p_sel).p_prior };
                    }
                }
            }
        } else {
            if !(unsafe { (*p_subq_1).p_win }).is_null() &&
                    unsafe { (*unsafe { (*p_subq_1).p_win }).p_partition } ==
                        core::ptr::null_mut() {
                return 0;
            }
        }
        if unsafe { (*p_subq_1).p_limit } != core::ptr::null_mut() {
            return 0;
        }
        while unsafe { (*p_where_1).op } as i32 == 44 {
            n_chng +=
                push_down_where_terms(p_parse_1, p_subq_1,
                    unsafe { (*p_where_1).p_right }, p_src_list_1, i_src_1);
            p_where_1 = unsafe { (*p_where_1).p_left };
        }
        if unsafe {
                    sqlite3_expr_is_single_table_constraint(p_where_1,
                        p_src_list_1 as *const SrcList, i_src_1, 1)
                } != 0 {
            { let __p = &mut n_chng; let __t = *__p; *__p += 1; __t };
            unsafe { (*p_subq_1).sel_flags |= 16777216 as u32 };
            while !(p_subq_1).is_null() {
                let mut x: SubstContext = unsafe { core::mem::zeroed() };
                p_new =
                    unsafe {
                        sqlite3_expr_dup(unsafe { (*p_parse_1).db },
                            p_where_1 as *const Expr, 0)
                    };
                unset_join_expr(p_new, -1, 1);
                x.p_parse = p_parse_1;
                x.i_table = unsafe { (*p_src).i_cursor };
                x.i_new_table = unsafe { (*p_src).i_cursor };
                x.is_outer_join = 0;
                x.n_sel_depth = 0;
                x.p_e_list = unsafe { (*p_subq_1).p_e_list };
                x.p_c_list =
                    find_leftmost_exprlist(p_subq_1 as *const Select);
                p_new = subst_expr(&mut x, p_new);
                { let _ = 0; };
                if unsafe { (*p_parse_1).n_err } == 0 &&
                            unsafe { (*p_new).op } as i32 == 50 &&
                        unsafe { (*p_new).flags } & 4096 as u32 != 0 as u32 {
                    { let _ = 0; };
                    unsafe {
                        (*unsafe { (*p_new).x.p_select }).sel_flags |= 32 as u32
                    };
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    unsafe {
                        (*unsafe { (*p_where_1).x.p_select }).sel_flags |= 32 as u32
                    };
                }
                if !(unsafe { (*p_subq_1).p_win }).is_null() &&
                        0 ==
                            push_down_window_check(p_parse_1, unsafe { &*p_subq_1 },
                                p_new) {

                    /// Restriction 6c has prevented push-down in this case
                    unsafe {
                        sqlite3_expr_delete(unsafe { (*p_parse_1).db }, p_new)
                    };
                    { let __p = &mut n_chng; let __t = *__p; *__p -= 1; __t };
                    break;
                }
                if unsafe { (*p_subq_1).sel_flags } & 8 as u32 != 0 {
                    unsafe {
                        (*p_subq_1).p_having =
                            unsafe {
                                sqlite3_expr_and(p_parse_1, unsafe { (*p_subq_1).p_having },
                                    p_new)
                            }
                    };
                } else {
                    unsafe {
                        (*p_subq_1).p_where =
                            unsafe {
                                sqlite3_expr_and(p_parse_1, unsafe { (*p_subq_1).p_where },
                                    p_new)
                            }
                    };
                }
                p_subq_1 = unsafe { (*p_subq_1).p_prior };
            }
        }
        return n_chng;
    }
}

///* Check to see if a subquery contains result-set columns that are
///* never used.  If it does, change the value of those result-set columns
///* to NULL so that they do not cause unnecessary work to compute.
///*
///* Return the number of column that were changed to NULL.
#[allow(unused_doc_comments)]
extern "C" fn disable_unused_subquery_result_columns(p_item_1: &SrcItem)
    -> i32 {
    unsafe {
        let mut n_col: i32 = 0;
        let mut p_sub: *mut Select = core::ptr::null_mut();
        /// The subquery to be simplified
        let mut p_x: *mut Select = core::ptr::null_mut();
        /// For looping over compound elements of pSub
        let mut p_tab: *const Table = core::ptr::null();
        /// The table that describes the subquery
        let mut j: i32 = 0;
        /// Column number
        let mut n_chng: i32 = 0;
        /// Number of columns converted to NULL
        let mut col_used: Bitmask = 0 as Bitmask;

        /// Columns that may not be NULLed out
        { let _ = 0; };
        if (*p_item_1).fg.is_correlated() != 0 || (*p_item_1).fg.is_cte() != 0
            {
            return 0;
        }
        { let _ = 0; };
        p_tab = (*p_item_1).p_s_tab;
        { let _ = 0; };
        p_sub = unsafe { (*(*p_item_1).u4.p_subq).p_select };
        { let _ = 0; };
        {
            p_x = p_sub;
            '__b97: loop {
                if !(!(p_x).is_null()) { break '__b97; }
                '__c97: loop {
                    if unsafe { (*p_x).sel_flags } & (1 | 8) as u32 != 0 as u32
                        {
                        return 0;
                    }
                    if !(unsafe { (*p_x).p_prior }).is_null() &&
                            unsafe { (*p_x).op } as i32 != 136 {

                        /// This optimization does not work for compound subqueries that
                        ///* use UNION, INTERSECT, or EXCEPT.  Only UNION ALL is allowed.
                        return 0;
                    }
                    if !(unsafe { (*p_x).p_win }).is_null() {

                        /// This optimization does not work for subqueries that use window
                        ///* functions.
                        return 0;
                    }
                    break '__c97;
                }
                p_x = unsafe { (*p_x).p_prior };
            }
        }
        col_used = (*p_item_1).col_used;
        if !(unsafe { (*p_sub).p_order_by }).is_null() {
            let p_list: *const ExprList =
                unsafe { (*p_sub).p_order_by } as *const ExprList;
            {
                j = 0;
                '__b98: loop {
                    if !(j < unsafe { (*p_list).n_expr }) { break '__b98; }
                    '__c98: loop {
                        let mut i_col: u16 =
                            unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                        *mut ExprListItem).offset(j as isize)).u.x.i_order_by_col
                            };
                        if i_col as i32 > 0 {
                            { let __p = &mut i_col; let __t = *__p; *__p -= 1; __t };
                            col_used |=
                                (1 as Bitmask) <<
                                    if i_col as i32 >=
                                            (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 {
                                        (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                            1
                                    } else { i_col as i32 };
                        }
                        break '__c98;
                    }
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        n_col = unsafe { (*p_tab).n_col } as i32;
        {
            j = 0;
            '__b99: loop {
                if !(j < n_col) { break '__b99; }
                '__c99: loop {
                    let m: Bitmask =
                        if j <
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                    1 {
                            (1 as Bitmask) << j
                        } else {
                            (1 as Bitmask) <<
                                (core::mem::size_of::<Bitmask>() as u64 * 8 as u64) as i32 -
                                    1
                        };
                    if m & col_used != 0 as u64 { break '__c99; }
                    {
                        p_x = p_sub;
                        '__b100: loop {
                            if !(!(p_x).is_null()) { break '__b100; }
                            '__c100: loop {
                                let p_y: *mut Expr =
                                    unsafe {
                                        (*(unsafe { (*unsafe { (*p_x).p_e_list }).a.as_ptr() } as
                                                        *mut ExprListItem).offset(j as isize)).p_expr
                                    };
                                if unsafe { (*p_y).op } as i32 == 122 { break '__c100; }
                                unsafe { (*p_y).op = 122 as u8 };
                                unsafe { (*p_y).flags &= !((8192 | 524288) as u32) };
                                unsafe { (*p_x).sel_flags |= 16777216 as u32 };
                                { let __p = &mut n_chng; let __t = *__p; *__p += 1; __t };
                                break '__c100;
                            }
                            p_x = unsafe { (*p_x).p_prior };
                        }
                    }
                    break '__c99;
                }
                { let __p = &mut j; let __t = *__p; *__p += 1; __t };
            }
        }
        return n_chng;
    }
}

///* Check to see if the pThis entry of pTabList is a self-join of another view.
///* Search FROM-clause entries in the range of iFirst..iEnd, including iFirst
///* but stopping before iEnd.
///*
///* If pThis is a self-join, then return the SrcItem for the first other
///* instance of that view found.  If pThis is not a self-join then return 0.
#[allow(unused_doc_comments)]
extern "C" fn is_self_join_view(p_tab_list_1: &mut SrcList,
    p_this_1: &SrcItem, mut i_first_1: i32, i_end_1: i32) -> *mut SrcItem {
    unsafe {
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        let mut p_sel: *const Select = core::ptr::null();
        { let _ = 0; };
        p_sel = unsafe { (*(*p_this_1).u4.p_subq).p_select };
        { let _ = 0; };
        if unsafe { (*p_sel).sel_flags } & 16777216 as u32 != 0 {
            return core::ptr::null_mut();
        }
        while i_first_1 < i_end_1 {
            let mut p_s1: *const Select = core::ptr::null();
            p_item =
                unsafe {
                    &mut *((*p_tab_list_1).a.as_ptr() as
                                    *mut SrcItem).offset({
                                        let __p = &mut i_first_1;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize)
                };
            if (unsafe { (*p_item).fg.is_subquery() } == 0) as i32 != 0 {
                continue;
            }
            if unsafe { (*p_item).fg.via_coroutine() } != 0 { continue; }
            if unsafe { (*p_item).z_name } == core::ptr::null_mut() {
                continue;
            }
            { let _ = 0; };
            { let _ = 0; };
            if unsafe { (*unsafe { (*p_item).p_s_tab }).p_schema } !=
                    unsafe { (*(*p_this_1).p_s_tab).p_schema } {
                continue;
            }
            if unsafe {
                        sqlite3_stricmp(unsafe { (*p_item).z_name } as *const i8,
                            (*p_this_1).z_name as *const i8)
                    } != 0 {
                continue;
            }
            p_s1 = unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select };
            if unsafe { (*unsafe { (*p_item).p_s_tab }).p_schema } ==
                        core::ptr::null_mut() &&
                    unsafe { (*p_sel).sel_id } != unsafe { (*p_s1).sel_id } {

                /// The query flattener left two different CTE tables with identical
                ///* names in the same FROM clause.
                continue;
            }
            if unsafe { (*p_s1).sel_flags } & 16777216 as u32 != 0 {

                /// The view was modified by some other optimization such as
                ///* pushDownWhereTerms()
                continue;
            }
            return p_item;
        }
        return core::ptr::null_mut();
    }
}

///* Return TRUE (non-zero) if the i-th entry in the pTabList SrcList can
///* be implemented as a co-routine.  The i-th entry is guaranteed to be
///* a subquery.
///*
///* The subquery is implemented as a co-routine if all of the following are
///* true:
///*
///*    (1)  The subquery will likely be implemented in the outer loop of
///*         the query.  This will be the case if any one of the following
///*         conditions hold:
///*         (a)  The subquery is the only term in the FROM clause
///*         (b)  The subquery is the left-most term and a CROSS JOIN or similar
///*              requires it to be the outer loop
///*         (c)  All of the following are true:
///*                (i) The subquery is the left-most subquery in the FROM clause
///*               (ii) There is nothing that would prevent the subquery from
///*                    being used as the outer loop if the sqlite3WhereBegin()
///*                    routine nominates it to that position.
///*              (iii) The query is not a UPDATE ... FROM
///*    (2)  The subquery is not a CTE that should be materialized because
///*         (a) the AS MATERIALIZED keyword is used, or
///*         (b) the CTE is used multiple times and does not have the
///*             NOT MATERIALIZED keyword
///*    (3)  The subquery is not part of a left operand for a RIGHT JOIN
///*    (4)  The SQLITE_Coroutine optimization disable flag is not set
///*    (5)  The subquery is not self-joined
#[allow(unused_doc_comments)]
extern "C" fn from_clause_term_can_be_coroutine(p_parse_1: &Parse,
    p_tab_list_1: *mut SrcList, mut i: i32, sel_flags_1: i32) -> i32 {
    unsafe {
        let mut p_item: *mut SrcItem =
            unsafe {
                &mut *(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                *mut SrcItem).offset(i as isize)
            };
        if unsafe { (*p_item).fg.is_cte() } != 0 {
            let p_cte_use: *const CteUse =
                unsafe { (*p_item).u2.p_cte_use } as *const CteUse;
            if unsafe { (*p_cte_use).e_m10d } as i32 == 0 { return 0; }
            if unsafe { (*p_cte_use).n_use } as i32 >= 2 &&
                    unsafe { (*p_cte_use).e_m10d } as i32 != 2 {
                return 0;
            }
        }
        if unsafe {
                            (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)).fg.jointype
                        } as i32 & 64 != 0 {
            return 0;
        }
        if unsafe { (*(*p_parse_1).db).db_opt_flags } & 33554432 as u32 !=
                0 as u32 {
            return 0;
        }
        if is_self_join_view(unsafe { &mut *p_tab_list_1 },
                    unsafe { &*p_item }, i + 1,
                    unsafe { (*p_tab_list_1).n_src }) != core::ptr::null_mut() {
            return 0;
        }
        if i == 0 {
            if unsafe { (*p_tab_list_1).n_src } == 1 { return 1; }
            if unsafe {
                                (*(unsafe { (*p_tab_list_1).a.as_ptr() } as
                                                    *mut SrcItem).offset(1 as isize)).fg.jointype
                            } as i32 & 2 != 0 {
                return 1;
            }
            if sel_flags_1 & 268435456 != 0 { return 0; }

            /// (1c-iii)
            return 1;
        }
        if sel_flags_1 & 268435456 != 0 { return 0; }
        loop {
            if unsafe { (*p_item).fg.jointype } as i32 & (32 | 2) != 0 {
                return 0;
            }
            if i == 0 { break; }
            { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            {
                let __p = &mut p_item;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(-1) };
                __t
            };
            if unsafe { (*p_item).fg.is_subquery() } != 0 { return 0; }
        }
        return 1;
    }
}

///* If p2 exists and p1 and p2 have the same number of terms, then change
///* every term of p1 to have the same sort order as p2 and return true.
///*
///* If p2 is NULL or p1 and p2 are different lengths, then make no changes
///* and return false.
///*
///* p1 must be non-NULL.
extern "C" fn sqlite3_copy_sort_order(p1: &mut ExprList, p2: *const ExprList)
    -> i32 {
    { let _ = 0; };
    if !(p2).is_null() && (*p1).n_expr == unsafe { (*p2).n_expr } {
        let mut ii: i32 = 0;
        {
            ii = 0;
            '__b103: loop {
                if !(ii < (*p1).n_expr) { break '__b103; }
                '__c103: loop {
                    let mut sort_flags: u8 = 0 as u8;
                    sort_flags =
                        (unsafe {
                                        (*(unsafe { (*p2).a.as_ptr() } as
                                                            *mut ExprListItem).offset(ii as isize)).fg.sort_flags
                                    } as i32 & 1) as u8;
                    unsafe {
                        (*((*p1).a.as_ptr() as
                                                *mut ExprListItem).offset(ii as isize)).fg.sort_flags =
                            sort_flags
                    };
                    break '__c103;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        return 1;
    } else { return 0; }
}

///* Deallocate a single AggInfo object
extern "C" fn agginfo_free(db: *mut Sqlite3, p_arg_1: *mut ()) -> () {
    let p: *mut AggInfo = p_arg_1 as *mut AggInfo;
    unsafe { sqlite3_db_free(db, unsafe { (*p).a_col } as *mut ()) };
    unsafe { sqlite3_db_free(db, unsafe { (*p).a_func } as *mut ()) };
    unsafe { sqlite3_db_free_nn(db, p as *mut ()) };
}

///* sqlite3WalkExpr() callback used by havingToWhere().
///*
///* If the node passed to the callback is a TK_AND node, return
///* WRC_Continue to tell sqlite3WalkExpr() to iterate through child nodes.
///*
///* Otherwise, return WRC_Prune. In this case, also check if the
///* sub-expression matches the criteria for being moved to the WHERE
///* clause. If so, add it to the WHERE clause and replace the sub-expression
///* within the HAVING expression with a constant "1".
extern "C" fn having_to_where_expr_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 != 44 {
            let p_s: *mut Select = unsafe { (*p_walker_1).u.p_select };
            if unsafe {
                                sqlite3_expr_is_constant_or_group_by(unsafe {
                                        (*p_walker_1).p_parse
                                    }, p_expr_1, unsafe { (*p_s).p_group_by })
                            } != 0 &&
                        (unsafe { (*p_expr_1).flags } & (1 | 536870912) as u32 ==
                                    536870912 as u32) as i32 == 0 &&
                    unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
                let db: *mut Sqlite3 =
                    unsafe { (*unsafe { (*p_walker_1).p_parse }).db };
                let mut p_new: *mut Expr =
                    unsafe { sqlite3_expr_int32(db, 1) };
                if !(p_new).is_null() {
                    let p_where: *mut Expr = unsafe { (*p_s).p_where };
                    {
                        let t: Expr = unsafe { core::ptr::read(p_new) };
                        unsafe { *p_new = unsafe { core::ptr::read(p_expr_1) } };
                        unsafe { *p_expr_1 = t };
                    }
                    p_new =
                        unsafe {
                            sqlite3_expr_and(unsafe { (*p_walker_1).p_parse }, p_where,
                                p_new)
                        };
                    unsafe { (*p_s).p_where = p_new };
                    unsafe { (*p_walker_1).e_code = 1 as u16 };
                }
            }
            return 1;
        }
        return 0;
    }
}

///* Transfer eligible terms from the HAVING clause of a query, which is
///* processed after grouping, to the WHERE clause, which is processed before
///* grouping. For example, the query:
///*
///*   SELECT * FROM <tables> WHERE a=? GROUP BY b HAVING b=? AND c=?
///*
///* can be rewritten as:
///*
///*   SELECT * FROM <tables> WHERE a=? AND b=? GROUP BY b HAVING c=?
///*
///* A term of the HAVING expression is eligible for transfer if it consists
///* entirely of constants and expressions that are also GROUP BY terms that
///* use the "BINARY" collation sequence.
extern "C" fn having_to_where(p_parse_1: *mut Parse, p: *mut Select) -> () {
    unsafe {
        let mut s_walker: Walker = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut s_walker as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        s_walker.p_parse = p_parse_1;
        s_walker.x_expr_callback = Some(having_to_where_expr_cb);
        s_walker.u.p_select = p;
        unsafe { sqlite3_walk_expr(&mut s_walker, unsafe { (*p).p_having }) };
    }
}

///* The pFunc is the only aggregate function in the query.  Check to see
///* if the query is a candidate for the min/max optimization.
///*
///* If the query is a candidate for the min/max optimization, then set
///* *ppMinMax to be an ORDER BY clause to be used for the optimization
///* and return either WHERE_ORDERBY_MIN or WHERE_ORDERBY_MAX depending on
///* whether pFunc is a min() or max() function.
///*
///* If the query is not a candidate for the min/max optimization, return
///* WHERE_ORDERBY_NORMAL (which must be zero).
///*
///* This routine must be called after aggregate functions have been
///* located but before their arguments have been subjected to aggregate
///* analysis.
#[allow(unused_doc_comments)]
extern "C" fn min_max_query(db: *mut Sqlite3, p_func_1: &Expr,
    pp_min_max_1: &mut *mut ExprList) -> u8 {
    unsafe {
        let mut e_ret: i32 = 0;
        /// Return value
        let mut p_e_list: *const ExprList = core::ptr::null();
        /// Arguments to agg function
        let mut z_func: *const i8 = core::ptr::null();
        /// Name of aggregate function pFunc
        let mut p_order_by: *mut ExprList = core::ptr::null_mut();
        let mut sort_flags: u8 = 0 as u8;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        p_e_list = (*p_func_1).x.p_list;
        if p_e_list == core::ptr::null_mut() ||
                        unsafe { (*p_e_list).n_expr } != 1 ||
                    (*p_func_1).flags & 16777216 as u32 != 0 as u32 ||
                unsafe { (*db).db_opt_flags } & 65536 as u32 != 0 as u32 {
            return e_ret as u8;
        }
        { let _ = 0; };
        z_func = (*p_func_1).u.z_token as *const i8;
        if unsafe {
                    sqlite3_str_i_cmp(z_func,
                        c"min".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            e_ret = 1;
            if unsafe {
                        sqlite3_expr_can_be_null(unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(0 as isize)).p_expr
                                } as *const Expr)
                    } != 0 {
                sort_flags = 2 as u8;
            }
        } else if unsafe {
                    sqlite3_str_i_cmp(z_func,
                        c"max".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            e_ret = 2;
            sort_flags = 1 as u8;
        } else { return e_ret as u8; }
        *pp_min_max_1 =
            {
                p_order_by =
                    unsafe {
                        sqlite3_expr_list_dup(db, p_e_list as *const ExprList, 0)
                    };
                p_order_by
            };
        { let _ = 0; };
        if !(p_order_by).is_null() {
            unsafe {
                (*(unsafe { (*p_order_by).a.as_ptr() } as
                                        *mut ExprListItem).offset(0 as isize)).fg.sort_flags =
                    sort_flags
            };
        }
        return e_ret as u8;
    }
}

///* Analyze the arguments to aggregate functions.  Create new pAggInfo->aCol[]
///* entries for columns that are arguments to aggregate functions but which
///* are not otherwise used.
///*
///* The aCol[] entries in AggInfo prior to nAccumulator are columns that
///* are referenced outside of aggregate functions.  These might be columns
///* that are part of the GROUP by clause, for example.  Other database engines
///* would throw an error if there is a column reference that is not in the
///* GROUP BY clause and that is not part of an aggregate function argument.
///* But SQLite allows this.
///*
///* The aCol[] entries beginning with the aCol[nAccumulator] and following
///* are column references that are used exclusively as arguments to
///* aggregate functions.  This routine is responsible for computing
///* (or recomputing) those aCol[] entries.
extern "C" fn analyze_agg_func_args(p_agg_info_1: &AggInfo,
    p_nc_1: *mut NameContext) -> () {
    unsafe {
        let mut i: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        unsafe { (*p_nc_1).nc_flags |= 131072 };
        {
            i = 0;
            '__b104: loop {
                if !(i < (*p_agg_info_1).n_func) { break '__b104; }
                '__c104: loop {
                    let p_expr: *const Expr =
                        unsafe {
                                (*(*p_agg_info_1).a_func.offset(i as isize)).p_f_expr
                            } as *const Expr;
                    { let _ = 0; };
                    { let _ = 0; };
                    unsafe {
                        sqlite3_expr_analyze_agg_list(p_nc_1,
                            unsafe { (*p_expr).x.p_list })
                    };
                    if !(unsafe { (*p_expr).p_left }).is_null() {
                        { let _ = 0; };
                        { let _ = 0; };
                        unsafe {
                            sqlite3_expr_analyze_agg_list(p_nc_1,
                                unsafe { (*unsafe { (*p_expr).p_left }).x.p_list })
                        };
                    }
                    { let _ = 0; };
                    if unsafe { (*p_expr).flags } & 16777216 as u32 != 0 as u32
                        {
                        unsafe {
                            sqlite3_expr_analyze_aggregates(p_nc_1,
                                unsafe { (*unsafe { (*p_expr).y.p_win }).p_filter })
                        };
                    }
                    break '__c104;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { (*p_nc_1).nc_flags &= !131072 };
    }
}

///* An index on expressions is being used in the inner loop of an
///* aggregate query with a GROUP BY clause.  This routine attempts
///* to adjust the AggInfo object to take advantage of index and to
///* perhaps use the index as a covering index.
///*
extern "C" fn optimize_aggregate_use_of_indexed_expr(p_parse_1: *const Parse,
    p_select_1: *const Select, p_agg_info_1: *mut AggInfo,
    p_nc_1: *mut NameContext) -> () {
    unsafe {
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            (*p_agg_info_1).n_column =
                unsafe { (*p_agg_info_1).n_accumulator }
        };
        if unsafe { (*p_agg_info_1).n_sorting_column } > 0 as u32 {
            let mut mx: i32 =
                unsafe { (*unsafe { (*p_select_1).p_group_by }).n_expr } - 1;
            let mut j: i32 = 0;
            let mut k: i32 = 0;
            {
                j = 0;
                '__b105: loop {
                    if !(j < unsafe { (*p_agg_info_1).n_column }) {
                        break '__b105;
                    }
                    '__c105: loop {
                        k =
                            unsafe {
                                (*unsafe {
                                            (*p_agg_info_1).a_col.offset(j as isize)
                                        }).i_sorter_column
                            };
                        if k > mx { mx = k; }
                        break '__c105;
                    }
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                }
            }
            unsafe { (*p_agg_info_1).n_sorting_column = (mx + 1) as u32 };
        }
        analyze_agg_func_args(unsafe { &*p_agg_info_1 }, p_nc_1);
        { let _ = p_select_1; };
        { let _ = p_parse_1; };
    }
}

///* Allocate a block of registers so that there is one register for each
///* pAggInfo->aCol[] and pAggInfo->aFunc[] entry in pAggInfo.  The first
///* register in this block is stored in pAggInfo->iFirstReg.
///*
///* This routine may only be called once for each AggInfo object.  Prior
///* to calling this routine:
///*
///*     *  The aCol[] and aFunc[] arrays may be modified
///*     *  The AggInfoColumnReg() and AggInfoFuncReg() macros may not be used
///*
///* After calling this routine:
///*
///*     *  The aCol[] and aFunc[] arrays are fixed
///*     *  The AggInfoColumnReg() and AggInfoFuncReg() macros may be used
///*
extern "C" fn assign_aggregate_registers(p_parse_1: &mut Parse,
    p_agg_info_1: &mut AggInfo) -> () {
    { let _ = 0; };
    { let _ = 0; };
    (*p_agg_info_1).i_first_reg = (*p_parse_1).n_mem + 1;
    (*p_parse_1).n_mem += (*p_agg_info_1).n_column + (*p_agg_info_1).n_func;
}

///* Walker callback for aggregateConvertIndexedExprRefToColumn().
extern "C" fn aggregate_idx_epr_ref_to_col_callback(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut p_agg_info: *const AggInfo = core::ptr::null();
        let mut p_col: *const AggInfoCol = core::ptr::null();
        { let _ = p_walker_1; };
        if unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
            return 0;
        }
        if unsafe { (*p_expr_1).op } as i32 == 170 { return 0; }
        if unsafe { (*p_expr_1).op } as i32 == 169 { return 0; }
        if unsafe { (*p_expr_1).op } as i32 == 179 { return 0; }
        p_agg_info = unsafe { (*p_expr_1).p_agg_info };
        if unsafe { (*p_expr_1).i_agg } as i32 >=
                unsafe { (*p_agg_info).n_column } {
            return 0;
        }
        { let _ = 0; };
        p_col =
            unsafe {
                unsafe {
                    (*p_agg_info).a_col.offset(unsafe { (*p_expr_1).i_agg } as
                            isize)
                }
            };
        unsafe { (*p_expr_1).op = 170 as u8 };
        unsafe { (*p_expr_1).i_table = unsafe { (*p_col).i_table } };
        unsafe {
            (*p_expr_1).i_column = unsafe { (*p_col).i_column } as YnVar
        };
        unsafe { (*p_expr_1).flags &= !((8192 | 512 | 524288) as u32) };
        return 1;
    }
}

///* Convert every pAggInfo->aFunc[].pExpr such that any node within
///* those expressions that has pAppInfo set is changed into a TK_AGG_COLUMN
///* opcode.
extern "C" fn aggregate_convert_indexed_expr_ref_to_column(p_agg_info_1:
        &AggInfo) -> () {
    let mut i: i32 = 0;
    let mut w: Walker = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut w as *mut (), 0,
            core::mem::size_of::<Walker>() as u64)
    };
    w.x_expr_callback = Some(aggregate_idx_epr_ref_to_col_callback);
    {
        i = 0;
        '__b106: loop {
            if !(i < (*p_agg_info_1).n_func) { break '__b106; }
            '__c106: loop {
                unsafe {
                    sqlite3_walk_expr(&mut w,
                        unsafe {
                            (*(*p_agg_info_1).a_func.offset(i as isize)).p_f_expr
                        })
                };
                break '__c106;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

///* Generate code that will update the accumulator memory cells for an
///* aggregate based on the current cursor position.
///*
///* If regAcc is non-zero and there are no min() or max() aggregates
///* in pAggInfo, then only populate the pAggInfo->nAccumulator accumulator
///* registers if register regAcc contains 0. The caller will take care
///* of setting and clearing regAcc.
///*
///* For an ORDER BY aggregate, the actual accumulator memory cell update
///* is deferred until after all input rows have been received, so that they
///* can be run in the requested order.  In that case, instead of invoking
///* OP_AggStep to update the accumulator, just add the arguments that would
///* have been passed into OP_AggStep into the sorting ephemeral table
///* (along with the appropriate sort key).
#[allow(unused_doc_comments)]
extern "C" fn update_accumulator(p_parse_1: *mut Parse, reg_acc_1: i32,
    p_agg_info_1: &mut AggInfo, e_distinct_type_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut reg_hit: i32 = 0;
        let mut addr_hit_test: i32 = 0;
        let mut p_f: *mut AggInfoFunc = core::ptr::null_mut();
        let mut p_c: *const AggInfoCol = core::ptr::null();
        { let _ = 0; };
        if unsafe { (*p_parse_1).n_err } != 0 { return; }
        (*p_agg_info_1).direct_mode = 1 as u8;
        {
            { i = 0; p_f = (*p_agg_info_1).a_func };
            '__b107: loop {
                if !(i < (*p_agg_info_1).n_func) { break '__b107; }
                '__c107: loop {
                    let mut n_arg: i32 = 0;
                    let mut addr_next: i32 = 0;
                    let mut reg_agg: i32 = 0;
                    let mut reg_agg_sz: i32 = 0;
                    let mut reg_distinct: i32 = 0;
                    let mut p_list: *mut ExprList = core::ptr::null_mut();
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    p_list = unsafe { (*unsafe { (*p_f).p_f_expr }).x.p_list };
                    if unsafe { (*unsafe { (*p_f).p_f_expr }).flags } &
                                16777216 as u32 != 0 as u32 {
                        let p_filter: *mut Expr =
                            unsafe {
                                (*unsafe { (*unsafe { (*p_f).p_f_expr }).y.p_win }).p_filter
                            };
                        if (*p_agg_info_1).n_accumulator != 0 &&
                                    unsafe { (*unsafe { (*p_f).p_func }).func_flags } &
                                            32 as u32 != 0 && reg_acc_1 != 0 {
                            if reg_hit == 0 {
                                reg_hit =
                                    {
                                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                                        *__p += 1;
                                        *__p
                                    };
                            }

                            /// If this is the first row of the group (regAcc contains 0), clear the
                            ///* "magnet" register regHit so that the accumulator registers
                            ///* are populated if the FILTER clause jumps over the the
                            ///* invocation of min() or max() altogether. Or, if this is not
                            ///* the first row (regAcc contains 1), set the magnet register so that
                            ///* the accumulators are not populated unless the min()/max() is invoked
                            ///* and indicates that they should be.
                            unsafe { sqlite3_vdbe_add_op2(v, 82, reg_acc_1, reg_hit) };
                        }
                        addr_next = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        unsafe {
                            sqlite3_expr_if_false(p_parse_1, p_filter, addr_next, 16)
                        };
                    }
                    if unsafe { (*p_f).i_ob_tab } >= 0 {
                        /// Instead of invoking AggStep, we must push the arguments that would
                        ///* have been passed to AggStep onto the sorting table.
                        let mut jj: i32 = 0;
                        /// Registered used so far in building the record
                        let mut p_ob_list: *mut ExprList = core::ptr::null_mut();

                        /// The ORDER BY clause
                        { let _ = 0; };
                        n_arg = unsafe { (*p_list).n_expr };
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_ob_list =
                            unsafe {
                                (*unsafe { (*unsafe { (*p_f).p_f_expr }).p_left }).x.p_list
                            };
                        { let _ = 0; };
                        { let _ = 0; };
                        reg_agg_sz = unsafe { (*p_ob_list).n_expr };
                        if (unsafe { (*p_f).b_ob_unique } == 0) as i32 != 0 {
                            {
                                let __p = &mut reg_agg_sz;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        if unsafe { (*p_f).b_ob_payload } != 0 {
                            reg_agg_sz += n_arg;
                        }
                        if unsafe { (*p_f).b_use_subtype } != 0 {
                            reg_agg_sz += n_arg;
                        }
                        {
                            let __p = &mut reg_agg_sz;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };

                        /// One extra register to hold result of MakeRecord
                        (reg_agg =
                            unsafe { sqlite3_get_temp_range(p_parse_1, reg_agg_sz) });
                        reg_distinct = reg_agg;
                        unsafe {
                            sqlite3_expr_code_expr_list(p_parse_1, p_ob_list, reg_agg,
                                0, 1 as u8)
                        };
                        jj = unsafe { (*p_ob_list).n_expr };
                        if (unsafe { (*p_f).b_ob_unique } == 0) as i32 != 0 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 128, unsafe { (*p_f).i_ob_tab },
                                    reg_agg + jj)
                            };
                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { (*p_f).b_ob_payload } != 0 {
                            reg_distinct = reg_agg + jj;
                            unsafe {
                                sqlite3_expr_code_expr_list(p_parse_1, p_list, reg_distinct,
                                    0, 1 as u8)
                            };
                            jj += n_arg;
                        }
                        if unsafe { (*p_f).b_use_subtype } != 0 {
                            let mut kk: i32 = 0;
                            let reg_base: i32 =
                                if unsafe { (*p_f).b_ob_payload } != 0 {
                                    reg_distinct
                                } else { reg_agg };
                            {
                                kk = 0;
                                '__b108: loop {
                                    if !(kk < n_arg) { break '__b108; }
                                    '__c108: loop {
                                        unsafe {
                                            sqlite3_vdbe_add_op2(v, 183, reg_base + kk, reg_agg + jj)
                                        };
                                        break '__c108;
                                    }
                                    {
                                        { let __p = &mut kk; let __t = *__p; *__p += 1; __t };
                                        { let __p = &mut jj; let __t = *__p; *__p += 1; __t }
                                    };
                                }
                            }
                        }
                    } else if !(p_list).is_null() {
                        n_arg = unsafe { (*p_list).n_expr };
                        reg_agg =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_arg) };
                        reg_distinct = reg_agg;
                        unsafe {
                            sqlite3_expr_code_expr_list(p_parse_1, p_list, reg_agg, 0,
                                1 as u8)
                        };
                    } else { n_arg = 0; reg_agg = 0; }
                    if unsafe { (*p_f).i_distinct } >= 0 && !(p_list).is_null()
                        {
                        if addr_next == 0 {
                            addr_next = unsafe { sqlite3_vdbe_make_label(p_parse_1) };
                        }
                        unsafe {
                            (*p_f).i_distinct =
                                code_distinct(p_parse_1, e_distinct_type_1,
                                    unsafe { (*p_f).i_distinct }, addr_next,
                                    unsafe { &*p_list }, reg_distinct)
                        };
                    }
                    if unsafe { (*p_f).i_ob_tab } >= 0 {

                        /// Insert a new record into the ORDER BY table
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_agg, reg_agg_sz - 1,
                                reg_agg + reg_agg_sz - 1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, unsafe { (*p_f).i_ob_tab },
                                reg_agg + reg_agg_sz - 1, reg_agg, reg_agg_sz - 1)
                        };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, reg_agg, reg_agg_sz)
                        };
                    } else {
                        if unsafe { (*unsafe { (*p_f).p_func }).func_flags } &
                                    32 as u32 != 0 {
                            let mut p_coll: *mut CollSeq = core::ptr::null_mut();
                            let mut p_item: *const ExprListItem = core::ptr::null();
                            let mut j: i32 = 0;
                            { let _ = 0; };
                            {
                                {
                                    j = 0;
                                    p_item =
                                        unsafe { (*p_list).a.as_ptr() } as *mut ExprListItem
                                };
                                '__b109: loop {
                                    if !((p_coll).is_null() as i32 != 0 && j < n_arg) {
                                        break '__b109;
                                    }
                                    '__c109: loop {
                                        p_coll =
                                            unsafe {
                                                sqlite3_expr_coll_seq(p_parse_1,
                                                    unsafe { (*p_item).p_expr } as *const Expr)
                                            };
                                        break '__c109;
                                    }
                                    {
                                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                        {
                                            let __p = &mut p_item;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        }
                                    };
                                }
                            }
                            if (p_coll).is_null() as i32 != 0 {
                                p_coll =
                                    unsafe { (*unsafe { (*p_parse_1).db }).p_dflt_coll };
                            }
                            if reg_hit == 0 && (*p_agg_info_1).n_accumulator != 0 {
                                reg_hit =
                                    {
                                        let __p = unsafe { &mut (*p_parse_1).n_mem };
                                        *__p += 1;
                                        *__p
                                    };
                            }
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 87, reg_hit, 0, 0,
                                    p_coll as *mut i8 as *const i8, -2)
                            };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 164, 0, reg_agg,
                                (*p_agg_info_1).i_first_reg + (*p_agg_info_1).n_column + i)
                        };
                        unsafe {
                            sqlite3_vdbe_append_p4(v,
                                unsafe { (*p_f).p_func } as *mut (), -8)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, n_arg as u16) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, reg_agg, n_arg)
                        };
                    }
                    if addr_next != 0 {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_next) };
                    }
                    if unsafe { (*p_parse_1).n_err } != 0 { return; }
                    break '__c107;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_f;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        if reg_hit == 0 && (*p_agg_info_1).n_accumulator != 0 {
            reg_hit = reg_acc_1;
        }
        if reg_hit != 0 {
            addr_hit_test = unsafe { sqlite3_vdbe_add_op1(v, 16, reg_hit) };
        }
        {
            { i = 0; p_c = (*p_agg_info_1).a_col };
            '__b110: loop {
                if !(i < (*p_agg_info_1).n_accumulator) { break '__b110; }
                '__c110: loop {
                    unsafe {
                        sqlite3_expr_code(p_parse_1, unsafe { (*p_c).p_c_expr },
                            (*p_agg_info_1).i_first_reg + i)
                    };
                    if unsafe { (*p_parse_1).n_err } != 0 { return; }
                    break '__c110;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_c;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
        (*p_agg_info_1).direct_mode = 0 as u8;
        if addr_hit_test != 0 {
            unsafe { sqlite3_vdbe_jump_here_or_pop_inst(v, addr_hit_test) };
        }
    }
}

///* Invoke the OP_AggFinalize opcode for every aggregate function
///* in the AggInfo structure.
#[allow(unused_doc_comments)]
extern "C" fn finalize_agg_functions(p_parse_1: *mut Parse,
    p_agg_info_1: &AggInfo) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut p_f: *const AggInfoFunc = core::ptr::null();
        {
            { i = 0; p_f = (*p_agg_info_1).a_func };
            '__b111: loop {
                if !(i < (*p_agg_info_1).n_func) { break '__b111; }
                '__c111: loop {
                    let mut p_list: *const ExprList = core::ptr::null();
                    { let _ = 0; };
                    if unsafe { (*p_parse_1).n_err } != 0 { return; }
                    p_list = unsafe { (*unsafe { (*p_f).p_f_expr }).x.p_list };
                    if unsafe { (*p_f).i_ob_tab } >= 0 {
                        /// For an ORDER BY aggregate, calls to OP_AggStep were deferred.  Inputs
                        ///* were stored in emphermal table pF->iOBTab.  Here, we extract those
                        ///* inputs (in ORDER BY order) and make all calls to OP_AggStep
                        ///* before doing the OP_AggFinal call.
                        let mut i_top: i32 = 0;
                        /// Start of loop for extracting columns
                        let mut n_arg: i32 = 0;
                        /// Number of columns to extract
                        let mut n_key: i32 = 0;
                        /// Key columns to be skipped
                        let mut reg_agg: i32 = 0;
                        /// Extract into this array
                        let mut j: i32 = 0;

                        /// Loop counter
                        { let _ = 0; };
                        n_arg = unsafe { (*p_list).n_expr };
                        reg_agg =
                            unsafe { sqlite3_get_temp_range(p_parse_1, n_arg) };
                        if unsafe { (*p_f).b_ob_payload } as i32 == 0 {
                            n_key = 0;
                        } else {
                            { let _ = 0; };
                            { let _ = 0; };
                            { let _ = 0; };
                            n_key =
                                unsafe {
                                    (*unsafe {
                                                    (*unsafe { (*unsafe { (*p_f).p_f_expr }).p_left }).x.p_list
                                                }).n_expr
                                };
                            if (unsafe { (*p_f).b_ob_unique } == 0) as i32 != 0 {
                                { let __p = &mut n_key; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        i_top =
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 36, unsafe { (*p_f).i_ob_tab })
                            };
                        {
                            j = n_arg - 1;
                            '__b112: loop {
                                if !(j >= 0) { break '__b112; }
                                '__c112: loop {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 96, unsafe { (*p_f).i_ob_tab },
                                            n_key + j, reg_agg + j)
                                    };
                                    break '__c112;
                                }
                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                            }
                        }
                        if unsafe { (*p_f).b_use_subtype } != 0 {
                            let reg_subtype: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse_1) };
                            let i_base_col: i32 =
                                n_key + n_arg +
                                    (unsafe { (*p_f).b_ob_payload } as i32 == 0 &&
                                            unsafe { (*p_f).b_ob_unique } as i32 == 0) as i32;
                            {
                                j = n_arg - 1;
                                '__b113: loop {
                                    if !(j >= 0) { break '__b113; }
                                    '__c113: loop {
                                        unsafe {
                                            sqlite3_vdbe_add_op3(v, 96, unsafe { (*p_f).i_ob_tab },
                                                i_base_col + j, reg_subtype)
                                        };
                                        unsafe {
                                            sqlite3_vdbe_add_op2(v, 184, reg_subtype, reg_agg + j)
                                        };
                                        break '__c113;
                                    }
                                    { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                }
                            }
                            unsafe { sqlite3_release_temp_reg(p_parse_1, reg_subtype) };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 164, 0, reg_agg,
                                (*p_agg_info_1).i_first_reg + (*p_agg_info_1).n_column + i)
                        };
                        unsafe {
                            sqlite3_vdbe_append_p4(v,
                                unsafe { (*p_f).p_func } as *mut (), -8)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, n_arg as u16) };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 40, unsafe { (*p_f).i_ob_tab },
                                i_top + 1)
                        };
                        unsafe { sqlite3_vdbe_jump_here(v, i_top) };
                        unsafe {
                            sqlite3_release_temp_range(p_parse_1, reg_agg, n_arg)
                        };
                    }
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 167,
                            (*p_agg_info_1).i_first_reg + (*p_agg_info_1).n_column + i,
                            if !(p_list).is_null() {
                                unsafe { (*p_list).n_expr }
                            } else { 0 })
                    };
                    unsafe {
                        sqlite3_vdbe_append_p4(v,
                            unsafe { (*p_f).p_func } as *mut (), -8)
                    };
                    break '__c111;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_f;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
    }
}

///* Reset the aggregate accumulator.
///*
///* The aggregate accumulator is a set of memory cells that hold
///* intermediate results while calculating an aggregate.  This
///* routine generates code that stores NULLs in all of those memory
///* cells.
#[allow(unused_doc_comments)]
extern "C" fn reset_accumulator(p_parse_1: *mut Parse, p_agg_info_1: &AggInfo)
    -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let mut i: i32 = 0;
        let mut p_func: *mut AggInfoFunc = core::ptr::null_mut();
        let n_reg: i32 = (*p_agg_info_1).n_func + (*p_agg_info_1).n_column;
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if n_reg == 0 { return; }
        if unsafe { (*p_parse_1).n_err } != 0 { return; }
        unsafe {
            sqlite3_vdbe_add_op3(v, 77, 0, (*p_agg_info_1).i_first_reg,
                (*p_agg_info_1).i_first_reg + n_reg - 1)
        };
        {
            { p_func = (*p_agg_info_1).a_func; i = 0 };
            '__b114: loop {
                if !(i < (*p_agg_info_1).n_func) { break '__b114; }
                '__c114: loop {
                    if unsafe { (*p_func).i_distinct } >= 0 {
                        let p_e: *const Expr =
                            unsafe { (*p_func).p_f_expr } as *const Expr;
                        { let _ = 0; };
                        if unsafe { (*p_e).x.p_list } == core::ptr::null_mut() ||
                                unsafe { (*unsafe { (*p_e).x.p_list }).n_expr } != 1 {
                            unsafe {
                                sqlite3_error_msg(p_parse_1,
                                    c"DISTINCT aggregates must have exactly one argument".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                            unsafe { (*p_func).i_distinct = -1 };
                        } else {
                            let p_key_info: *mut KeyInfo =
                                sqlite3_key_info_from_expr_list(p_parse_1,
                                    unsafe { &*unsafe { (*p_e).x.p_list } }, 0, 0);
                            unsafe {
                                (*p_func).i_dist_addr =
                                    unsafe {
                                        sqlite3_vdbe_add_op4(v, 120,
                                            unsafe { (*p_func).i_distinct }, 0, 0,
                                            p_key_info as *mut i8 as *const i8, -9)
                                    }
                            };
                            unsafe {
                                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                    c"USE TEMP B-TREE FOR %s(DISTINCT)".as_ptr() as *mut i8 as
                                        *const i8, unsafe { (*unsafe { (*p_func).p_func }).z_name })
                            };
                        }
                    }
                    if unsafe { (*p_func).i_ob_tab } >= 0 {
                        let mut p_ob_list: *mut ExprList = core::ptr::null_mut();
                        let mut p_key_info_1: *mut KeyInfo = core::ptr::null_mut();
                        let mut n_extra: i32 = 0;
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        p_ob_list =
                            unsafe {
                                (*unsafe {
                                                    (*unsafe { (*p_func).p_f_expr }).p_left
                                                }).x.p_list
                            };
                        if (unsafe { (*p_func).b_ob_unique } == 0) as i32 != 0 {
                            { let __p = &mut n_extra; let __t = *__p; *__p += 1; __t };
                        }
                        if unsafe { (*p_func).b_ob_payload } != 0 {

                            /// extra columns for the function arguments
                            { let _ = 0; };
                            { let _ = 0; };
                            n_extra +=
                                unsafe {
                                    (*unsafe {
                                                    (*unsafe { (*p_func).p_f_expr }).x.p_list
                                                }).n_expr
                                };
                        }
                        if unsafe { (*p_func).b_use_subtype } != 0 {
                            n_extra +=
                                unsafe {
                                    (*unsafe {
                                                    (*unsafe { (*p_func).p_f_expr }).x.p_list
                                                }).n_expr
                                };
                        }
                        p_key_info_1 =
                            sqlite3_key_info_from_expr_list(p_parse_1,
                                unsafe { &*p_ob_list }, 0, n_extra);
                        if (unsafe { (*p_func).b_ob_unique } == 0) as i32 != 0 &&
                                unsafe { (*p_parse_1).n_err } == 0 {
                            {
                                let __p = unsafe { &mut (*p_key_info_1).n_key_field };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 120, unsafe { (*p_func).i_ob_tab },
                                unsafe { (*p_ob_list).n_expr } + n_extra, 0,
                                p_key_info_1 as *mut i8 as *const i8, -9)
                        };
                        unsafe {
                            sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                                c"USE TEMP B-TREE FOR %s(ORDER BY)".as_ptr() as *mut i8 as
                                    *const i8, unsafe { (*unsafe { (*p_func).p_func }).z_name })
                        };
                    }
                    break '__c114;
                }
                {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    {
                        let __p = &mut p_func;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }
                };
            }
        }
    }
}

///* The select statement passed as the first argument is an aggregate query.
///* The second argument is the associated aggregate-info object. This
///* function tests if the SELECT is of the form:
///*
///*   SELECT count(*) FROM <tbl>
///*
///* where table is a database table, not a sub-select or view. If the query
///* does match this pattern, then a pointer to the Table object representing
///* <tbl> is returned. Otherwise, NULL is returned.
///*
///* This routine checks to see if it is safe to use the count optimization.
///* A correct answer is still obtained (though perhaps more slowly) if
///* this routine returns NULL when it could have returned a table pointer.
///* But returning the pointer when NULL should have been returned can
///* result in incorrect answers and/or crashes.  So, when in doubt, return NULL.
extern "C" fn is_simple_count(p: &Select, p_agg_info_1: *mut AggInfo)
    -> *mut Table {
    unsafe {
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut p_expr: *const Expr = core::ptr::null();
        { let _ = 0; };
        if !((*p).p_where).is_null() ||
                                unsafe { (*(*p).p_e_list).n_expr } != 1 ||
                            unsafe { (*(*p).p_src).n_src } != 1 ||
                        unsafe {
                                (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).fg.is_subquery()
                            } != 0 || unsafe { (*p_agg_info_1).n_func } != 1 ||
                !((*p).p_having).is_null() {
            return core::ptr::null_mut();
        }
        p_tab =
            unsafe {
                (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).p_s_tab
            };
        { let _ = 0; };
        { let _ = 0; };
        if !(unsafe { (*p_tab).e_tab_type } as i32 == 0) as i32 != 0 {
            return core::ptr::null_mut();
        }
        p_expr =
            unsafe {
                (*(unsafe { (*(*p).p_e_list).a.as_ptr() } as
                                *mut ExprListItem).offset(0 as isize)).p_expr
            };
        { let _ = 0; };
        if unsafe { (*p_expr).op } as i32 != 169 {
            return core::ptr::null_mut();
        }
        if unsafe { (*p_expr).p_agg_info } != p_agg_info_1 {
            return core::ptr::null_mut();
        }
        if unsafe {
                        (*unsafe {
                                        (*unsafe {
                                                    (*p_agg_info_1).a_func.offset(0 as isize)
                                                }).p_func
                                    }).func_flags
                    } & 256 as u32 == 0 as u32 {
            return core::ptr::null_mut();
        }
        { let _ = 0; };
        if unsafe { (*p_expr).flags } & (4 | 16777216) as u32 != 0 as u32 {
            return core::ptr::null_mut();
        }
        return p_tab;
    }
}

extern "C" fn explain_simple_count(p_parse_1: *mut Parse, p_tab_1: &Table,
    p_idx_1: *const Index) -> () {
    if unsafe { (*p_parse_1).explain } as i32 == 2 {
        let b_cover: i32 =
            (p_idx_1 != core::ptr::null_mut() &&
                    ((*p_tab_1).tab_flags & 128 as u32 == 0 as u32 ||
                        !(unsafe { (*p_idx_1).idx_type() } as i32 == 2) as i32 !=
                            0)) as i32;
        unsafe {
            sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                c"SCAN %s%s%s".as_ptr() as *mut i8 as *const i8,
                (*p_tab_1).z_name,
                if b_cover != 0 {
                    c" USING COVERING INDEX ".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 },
                if b_cover != 0 {
                    unsafe { (*p_idx_1).z_name }
                } else { c"".as_ptr() as *mut i8 })
        };
    }
}

///* Unless an "EXPLAIN QUERY PLAN" command is being processed, this function
///* is a no-op. Otherwise, it adds a single row of output to the EQP result,
///* where the caption is of the form:
///*
///*   "USE TEMP B-TREE FOR xxx"
///*
///* where xxx is one of "DISTINCT", "ORDER BY" or "GROUP BY". Exactly which
///* is determined by the zUsage argument.
extern "C" fn explain_temp_table(p_parse_1: *mut Parse, z_usage_1: *const i8)
    -> () {
    unsafe {
        sqlite3_vdbe_explain(p_parse_1, 0 as u8,
            c"USE TEMP B-TREE FOR %s".as_ptr() as *mut i8 as *const i8,
            z_usage_1)
    };
}

///* If the inner loop was generated using a non-null pOrderBy argument,
///* then the results were placed in a sorter.  After the loop is terminated
///* we need to run the sorter and output the results.  The following
///* routine generates the code needed to do that.
#[allow(unused_doc_comments)]
extern "C" fn generate_sort_tail(p_parse_1: *mut Parse, p: &Select,
    p_sort_1: &SortCtx, mut n_column_1: i32, p_dest_1: &SelectDest) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        /// The prepared statement
        let addr_break: i32 = (*p_sort_1).label_done;
        /// Jump here to exit loop
        let addr_continue: i32 =
            unsafe { sqlite3_vdbe_make_label(p_parse_1) };
        /// Jump here for next cycle
        let mut addr: i32 = 0;
        /// Top of output loop. Jump for Next.
        let mut addr_once: i32 = 0;
        let mut i_tab: i32 = 0;
        let p_order_by: *const ExprList =
            (*p_sort_1).p_order_by as *const ExprList;
        let e_dest: i32 = (*p_dest_1).e_dest as i32;
        let i_parm: i32 = (*p_dest_1).i_sd_parm;
        let mut reg_row: i32 = 0;
        let mut reg_rowid: i32 = 0;
        let mut i_col: i32 = 0;
        let mut n_key: i32 = 0;
        /// Number of key columns in sorter record
        let mut i_sort_tab: i32 = 0;
        /// Sorter cursor to read from
        let mut i: i32 = 0;
        let mut b_seq: i32 = 0;
        /// True if sorter record includes seq. no.
        let n_ref_key: i32 = 0;
        let a_out_ex: *const ExprListItem =
            unsafe { (*(*p).p_e_list).a.as_ptr() } as *const ExprListItem;
        n_key = unsafe { (*p_order_by).n_expr } - (*p_sort_1).n_ob_sat;
        if (*p_sort_1).n_ob_sat == 0 || n_key == 1 {
            unsafe {
                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                    c"USE TEMP B-TREE FOR %sORDER BY".as_ptr() as *mut i8 as
                        *const i8,
                    if (*p_sort_1).n_ob_sat != 0 {
                        c"LAST TERM OF ".as_ptr() as *mut i8
                    } else { c"".as_ptr() as *mut i8 })
            };
        } else {
            unsafe {
                sqlite3_vdbe_explain(p_parse_1, 0 as u8,
                    c"USE TEMP B-TREE FOR LAST %d TERMS OF ORDER BY".as_ptr() as
                            *mut i8 as *const i8, n_key)
            };
        }
        { let _ = 0; };
        if (*p_sort_1).label_bk_out != 0 {
            unsafe {
                sqlite3_vdbe_add_op2(v, 10, (*p_sort_1).reg_return,
                    (*p_sort_1).label_bk_out)
            };
            unsafe { sqlite3_vdbe_goto(v, addr_break) };
            unsafe {
                sqlite3_vdbe_resolve_label(v, (*p_sort_1).label_bk_out)
            };
        }
        i_tab = (*p_sort_1).i_e_cursor;
        if e_dest == 7 || e_dest == 11 || e_dest == 8 {
            if e_dest == 8 && (*p).i_offset != 0 {
                unsafe { sqlite3_vdbe_add_op2(v, 77, 0, (*p_dest_1).i_sdst) };
            }
            reg_rowid = 0;
            reg_row = (*p_dest_1).i_sdst;
        } else {
            reg_rowid = unsafe { sqlite3_get_temp_reg(p_parse_1) };
            if e_dest == 10 || e_dest == 12 {
                reg_row = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                n_column_1 = 0;
            } else {
                reg_row =
                    unsafe { sqlite3_get_temp_range(p_parse_1, n_column_1) };
            }
        }
        if (*p_sort_1).sort_flags as i32 & 1 != 0 {
            let reg_sort_out: i32 =
                {
                    let __p = unsafe { &mut (*p_parse_1).n_mem };
                    *__p += 1;
                    *__p
                };
            i_sort_tab =
                {
                    let __p = unsafe { &mut (*p_parse_1).n_tab };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            if (*p_sort_1).label_bk_out != 0 {
                addr_once = unsafe { sqlite3_vdbe_add_op0(v, 15) };
            }
            unsafe {
                sqlite3_vdbe_add_op3(v, 123, i_sort_tab, reg_sort_out,
                    n_key + 1 + n_column_1 + n_ref_key)
            };
            if addr_once != 0 {
                unsafe { sqlite3_vdbe_jump_here(v, addr_once) };
            }
            addr =
                1 + unsafe { sqlite3_vdbe_add_op2(v, 34, i_tab, addr_break) };
            { let _ = 0; };
            unsafe {
                sqlite3_vdbe_add_op3(v, 135, i_tab, reg_sort_out, i_sort_tab)
            };
            b_seq = 0;
        } else {
            addr =
                1 + unsafe { sqlite3_vdbe_add_op2(v, 35, i_tab, addr_break) };
            code_offset(v, (*p).i_offset, addr_continue);
            i_sort_tab = i_tab;
            b_seq = 1;
            if (*p).i_offset > 0 {
                unsafe { sqlite3_vdbe_add_op2(v, 88, (*p).i_limit, -1) };
            }
        }
        {
            { i = 0; i_col = n_key + b_seq - 1 };
            '__b115: loop {
                if !(i < n_column_1) { break '__b115; }
                '__c115: loop {
                    if unsafe {
                                    (*a_out_ex.offset(i as isize)).u.x.i_order_by_col
                                } as i32 == 0 {
                        { let __p = &mut i_col; let __t = *__p; *__p += 1; __t };
                    }
                    break '__c115;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = n_column_1 - 1;
            '__b116: loop {
                if !(i >= 0) { break '__b116; }
                '__c116: loop {
                    {
                        let mut i_read: i32 = 0;
                        if unsafe {
                                    (*a_out_ex.offset(i as isize)).u.x.i_order_by_col
                                } != 0 {
                            i_read =
                                unsafe { (*a_out_ex.offset(i as isize)).u.x.i_order_by_col }
                                        as i32 - 1;
                        } else {
                            i_read =
                                { let __p = &mut i_col; let __t = *__p; *__p -= 1; __t };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_sort_tab, i_read, reg_row + i)
                        };
                    }
                    break '__c116;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        '__s117:
            {
            match e_dest {
                12 => {
                    {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_sort_tab, n_key + b_seq,
                                reg_row)
                        };
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, reg_rowid) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 130, i_parm, reg_row, reg_rowid)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, reg_row, n_column_1, reg_rowid,
                                (*p_dest_1).z_aff_sdst as *const i8, n_column_1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, reg_rowid, reg_row,
                                n_column_1)
                        };
                        break '__s117;
                    }
                    {

                        /// The LIMIT clause will terminate the loop for us
                        break '__s117;
                    }
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                10 => {
                    {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_sort_tab, n_key + b_seq,
                                reg_row)
                        };
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_parm, reg_rowid) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 130, i_parm, reg_row, reg_rowid)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, reg_row, n_column_1, reg_rowid,
                                (*p_dest_1).z_aff_sdst as *const i8, n_column_1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, reg_rowid, reg_row,
                                n_column_1)
                        };
                        break '__s117;
                    }
                    {

                        /// The LIMIT clause will terminate the loop for us
                        break '__s117;
                    }
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                9 => {
                    {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 99, reg_row, n_column_1, reg_rowid,
                                (*p_dest_1).z_aff_sdst as *const i8, n_column_1)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_parm, reg_rowid, reg_row,
                                n_column_1)
                        };
                        break '__s117;
                    }
                    {

                        /// The LIMIT clause will terminate the loop for us
                        break '__s117;
                    }
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                8 => {
                    {

                        /// The LIMIT clause will terminate the loop for us
                        break '__s117;
                    }
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                13 => {
                    {
                        let i2: i32 = (*p_dest_1).i_sd_parm2;
                        let r1: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_row + (i2 < 0) as i32,
                                n_column_1 - (i2 < 0) as i32, r1)
                        };
                        if i2 < 0 {
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 130, i_parm, r1, reg_row)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 140, i_parm, r1, reg_row, i2)
                            };
                        }
                        break '__s117;
                    }
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
                _ => {
                    {
                        { let _ = 0; };
                        if e_dest == 7 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 86, (*p_dest_1).i_sdst, n_column_1)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 12, (*p_dest_1).i_sd_parm)
                            };
                        }
                        break '__s117;
                    }
                }
            }
        }
        if reg_rowid != 0 {
            if e_dest == 9 {
                unsafe {
                    sqlite3_release_temp_range(p_parse_1, reg_row, n_column_1)
                };
            } else {
                unsafe { sqlite3_release_temp_reg(p_parse_1, reg_row) };
            }
            unsafe { sqlite3_release_temp_reg(p_parse_1, reg_rowid) };
        }

        /// The bottom of the loop
        unsafe { sqlite3_vdbe_resolve_label(v, addr_continue) };
        if (*p_sort_1).sort_flags as i32 & 1 != 0 {
            unsafe { sqlite3_vdbe_add_op2(v, 38, i_tab, addr) };
        } else { unsafe { sqlite3_vdbe_add_op2(v, 40, i_tab, addr) }; }
        if (*p_sort_1).reg_return != 0 {
            unsafe { sqlite3_vdbe_add_op1(v, 69, (*p_sort_1).reg_return) };
        }
        unsafe { sqlite3_vdbe_resolve_label(v, addr_break) };
    }
}

///* Generate byte-code for the SELECT statement given in the p argument. 
///*
///* The results are returned according to the SelectDest structure.
///* See comments in sqliteInt.h for further information.
///*
///* This routine returns the number of errors.  If any errors are
///* encountered, then an appropriate error message is left in
///* pParse->zErrMsg.
///*
///* This routine does NOT free the Select structure passed in.  The
///* calling function needs to do that.
///*
///* This is a long function.  The following is an outline of the processing
///* steps, with tags referencing various milestones:
///*
///*  *  Resolve names and similar preparation                tag-select-0100
///*  *  Scan of the FROM clause                              tag-select-0200
///*      +  OUTER JOIN strength reduction                      tag-select-0220
///*      +  Sub-query ORDER BY removal                         tag-select-0230
///*      +  Query flattening                                   tag-select-0240
///*  *  Separate subroutine for compound-SELECT              tag-select-0300
///*  *  WHERE-clause constant propagation                    tag-select-0330
///*  *  Count()-of-VIEW optimization                         tag-select-0350
///*  *  Scan of the FROM clause again                        tag-select-0400
///*      +  Authorize unreferenced tables                      tag-select-0410
///*      +  Predicate push-down optimization                   tag-select-0420
///*      +  Omit unused subquery columns optimization          tag-select-0440
///*      +  Generate code to implement subqueries              tag-select-0480
///*         -  Co-routines                                       tag-select-0482
///*         -  Reuse previously computed CTE                     tag-select-0484
///*         -  REuse previously computed VIEW                    tag-select-0486
///*         -  Materialize a VIEW or CTE                         tag-select-0488
///*  *  DISTINCT ORDER BY -> GROUP BY optimization           tag-select-0500
///*  *  Set up for ORDER BY                                  tag-select-0600
///*  *  Create output table                                  tag-select-0630
///*  *  Prepare registers for LIMIT                          tag-select-0650
///*  *  Setup for DISTINCT                                   tag-select-0680
///*  *  Generate code for non-aggregate and non-GROUP BY     tag-select-0700
///*  *  Generate code for aggregate and/or GROUP BY          tag-select-0800
///*      +  GROUP BY queries                                   tag-select-0810
///*      +  non-GROUP BY queries                               tag-select-0820
///*         -  Special case of count() w/o GROUP BY              tag-select-0821
///*         -  General case of non-GROUP BY aggregates           tag-select-0822
///*  *  Sort results, as needed                              tag-select-0900
///*  *  Internal self-checks                                 tag-select-1000
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_select(p_parse: *mut Parse, p: *mut Select,
    p_dest: *mut SelectDest) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        /// Loop counters
        let mut p_w_info: *mut WhereInfo = core::ptr::null_mut();
        /// Return from sqlite3WhereBegin()
        let mut v: *mut Vdbe = core::ptr::null_mut();
        /// The virtual machine under construction
        let mut is_agg: i32 = 0;
        /// True for select lists like "count(*)"
        let mut p_e_list: *mut ExprList = core::ptr::null_mut();
        /// List of columns to extract.
        let mut p_tab_list: *mut SrcList = core::ptr::null_mut();
        /// List of tables to select from
        let mut p_where: *mut Expr = core::ptr::null_mut();
        /// The WHERE clause.  May be NULL
        let mut p_group_by: *mut ExprList = core::ptr::null_mut();
        /// The GROUP BY clause.  May be NULL
        let mut p_having: *mut Expr = core::ptr::null_mut();
        /// The HAVING clause.  May be NULL
        let mut p_agg_info: *mut AggInfo = core::ptr::null_mut();
        /// Aggregate information
        let mut rc: i32 = 0;
        /// Value to return from this function
        let mut s_distinct: DistinctCtx = unsafe { core::mem::zeroed() };
        /// Info on how to code the DISTINCT keyword
        let mut s_sort: SortCtx = unsafe { core::mem::zeroed() };
        /// Info on how to code the ORDER BY clause
        let mut i_end: i32 = 0;
        /// Address of the end of the query
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        /// The database connection
        let mut p_min_max_order_by: *mut ExprList = core::ptr::null_mut();
        /// Added ORDER BY for min/max queries
        let mut min_max_flag: u8 = 0 as u8;
        /// Flag for min/max queries
        /// tag-select-0100
        /// All of these destinations are also able to ignore the ORDER BY clause
        /// If the SF_UFSrcCheck flag is set, then this function is being called
        ///* as part of populating the temp table for an UPDATE...FROM statement.
        ///* In this case, it is an error if the target object (pSrc->a[0]) name
        ///* or alias is duplicated within FROM clause (pSrc->a[1..n]). 
        ///*
        ///* Postgres disallows this case too. The reason is that some other
        ///* systems handle this case differently, and not all the same way,
        ///* which is just confusing. To avoid this, we follow PG's lead and
        ///* disallow it altogether.
        let mut p0: *mut SrcItem = core::ptr::null_mut();
        /// Clear the SF_UFSrcCheck flag. The check has already been performed,
        ///* and leaving this flag set can cause errors if a compound sub-query
        ///* in p->pSrc is flattened into this query and this function called
        ///* again as part of compound SELECT processing.
        /// SQLITE_OMIT_WINDOWFUNC
        /// Try to do various optimizations (flattening subqueries, and strength
        ///* reduction of join operators) in the FROM clause up into the main query
        ///* tag-select-0200
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        let mut p_sub: *mut Select = core::ptr::null_mut();
        let mut p_tab: *const Table = core::ptr::null();
        /// The expander should have already created transient Table objects
        ///* even for FROM clause elements such as subqueries that do not correspond
        ///* to a real table
        /// Try to simplify joins:
        ///*
        ///*      LEFT JOIN  ->  JOIN
        ///*     RIGHT JOIN  ->  JOIN
        ///*      FULL JOIN  ->  RIGHT JOIN
        ///*
        ///* If terms of the i-th table are used in the WHERE clause in such a
        ///* way that the i-th table cannot be the NULL row of a join, then
        ///* perform the appropriate simplification. This is called
        ///* "OUTER JOIN strength reduction" in the SQLite documentation.
        ///* tag-select-0220
        let mut p_i2: *mut SrcItem = core::ptr::null_mut();
        /// No further action if this term of the FROM clause is not a subquery
        /// Catch mismatch in the declared columns of a view and the number of
        ///* columns in the SELECT on the RHS
        /// Do not attempt the usual optimizations (flattening and ORDER BY
        ///* elimination) on a MATERIALIZED common table expression because
        ///* a MATERIALIZED common table expression is an optimization fence.
        /// Do not try to flatten an aggregate subquery.
        ///*
        ///* Flattening an aggregate subquery is only possible if the outer query
        ///* is not a join.  But if the outer query is not a join, then the subquery
        ///* will be implemented as a co-routine and there is no advantage to
        ///* flattening in that case.
        /// tag-select-0230:
        ///* If a FROM-clause subquery has an ORDER BY clause that is not
        ///* really doing anything, then delete it now so that it does not
        ///* interfere with query flattening.  See the discussion at
        ///* https://sqlite.org/forum/forumpost/2d76f2bcf65d256a
        ///*
        ///* Beware of these cases where the ORDER BY clause may not be safely
        ///* omitted:
        ///*
        ///*    (1)   There is also a LIMIT clause
        ///*    (2)   The subquery was added to help with window-function
        ///*          processing
        ///*    (3)   The subquery is in the FROM clause of an UPDATE
        ///*    (4)   The outer query uses an aggregate function other than
        ///*          the built-in count(), min(), or max().
        ///*    (5)   The ORDER BY isn't going to accomplish anything because
        ///*          one of:
        ///*            (a)  The outer query has a different ORDER BY clause
        ///*            (b)  The subquery is part of a join
        ///*          See forum post 062d576715d277c8
        ///*    (6)   The subquery is not a recursive CTE.  ORDER BY has a different
        ///*          meaning for recursive CTEs and this optimization does not
        ///*          apply.
        ///*
        ///* Also retain the ORDER BY if the OmitOrderBy optimization is disabled.
        /// Condition (5)
        /// Condition (1)
        /// (2) and (6)
        /// Condition (3) and (4)
        /// If the outer query contains a "complex" result set (that is,
        ///* if the result set of the outer query uses functions or subqueries)
        ///* and if the subquery contains an ORDER BY clause and if
        ///* it will be implemented as a co-routine, then do not flatten.  This
        ///* restriction allows SQL constructs like this:
        ///*
        ///*  SELECT expensive_function(x)
        ///*    FROM (SELECT x FROM tab ORDER BY y LIMIT 10);
        ///*
        ///* The expensive_function() is only computed on the 10 rows that
        ///* are output, rather than every row of the table.
        ///*
        ///* The requirement that the outer query have a complex result set
        ///* means that flattening does occur on simpler SQL constraints without
        ///* the expensive_function() like:
        ///*
        ///*  SELECT x FROM (SELECT x FROM tab ORDER BY y LIMIT 10);
        /// tag-select-0240
        /// This subquery can be absorbed into its parent.
        /// Handle compound SELECT statements using the separate multiSelect()
        ///* procedure.  tag-select-0300
        /// If there may be an "EXISTS (SELECT ...)" in the WHERE clause, attempt
        ///* to change it into a join.
        /// Do the WHERE-clause constant propagation optimization if this is
        ///* a join.  No need to spend time on this operation for non-join queries
        ///* as the equivalent optimization will be handled by query planner in
        ///* sqlite3WhereBegin().  tag-select-0330
        /// tag-select-0350
        /// Loop over all terms in the FROM clause and do two things for each term:
        ///*
        ///*   (1) Authorize unreferenced tables
        ///*   (2) Generate code for all sub-queries
        ///*
        ///* tag-select-0400
        let mut p_item_1: *mut SrcItem = core::ptr::null_mut();
        let mut p_prior: *const SrcItem = core::ptr::null();
        let mut dest: SelectDest = unsafe { core::mem::zeroed() };
        let mut p_subq: *mut Subquery = core::ptr::null_mut();
        let mut p_sub_1: *mut Select = core::ptr::null_mut();
        let mut z_saved_auth_context: *const i8 = core::ptr::null();
        /// Authorized unreferenced tables.  tag-select-0410
        ///*
        ///* Issue SQLITE_READ authorizations with a fake column name for any
        ///* tables that are referenced but from which no values are extracted.
        ///* Examples of where these kinds of null SQLITE_READ authorizations
        ///* would occur:
        ///*
        ///*     SELECT count(*) FROM t1;   -- SQLITE_READ t1.""
        ///*     SELECT t1.* FROM t1, t2;   -- SQLITE_READ t2.""
        ///*
        ///* The fake column name is an empty string.  It is possible for a table to
        ///* have a column named by the empty string, in which case there is no way to
        ///* distinguish between an unreferenced table and an actual reference to the
        ///* "" column. The original design was for the fake column name to be a NULL,
        ///* which would be unambiguous.  But legacy authorization callbacks might
        ///* assume the column name is non-NULL and segfault.  The use of an empty
        ///* string for the fake column name seems safer.
        let mut z_db: *const i8 = core::ptr::null();
        let mut i_db: i32 = 0;
        /// Generate code for all sub-queries in the FROM clause
        /// The code for a subquery should only be generated once.
        /// Increment Parse.nHeight by the height of the largest expression
        ///* tree referred to by this, the parent select. The child select
        ///* may contain expression trees of at most
        ///* (SQLITE_MAX_EXPR_DEPTH-Parse.nHeight) height. This is a bit
        ///* more conservative than necessary, but much easier than enforcing
        ///* an exact limit.
        /// Make copies of constant WHERE-clause terms in the outer query down
        ///* inside the subquery.  This can help the subquery to run more efficiently.
        ///* This is the "predicate push-down optimization".  tag-select-0420
        /// Convert unused result columns of the subquery into simple NULL
        ///* expressions, to avoid unneeded searching and computation.
        ///* tag-select-0440
        /// Generate byte-code to implement the subquery  tag-select-0480
        /// Implement a co-routine that will return a single row of the result
        ///* set on each invocation.  tag-select-0482
        let mut addr_top: i32 = 0;
        /// This is a CTE for which materialization code has already been
        ///* generated.  Invoke the subroutine to compute the materialization,
        ///* then make the pItem->iCursor be a copy of the ephemeral table that
        ///* holds the result of the materialization. tag-select-0484
        let mut p_cte_use: *const CteUse = core::ptr::null();
        /// This view has already been materialized by a prior entry in
        ///* this same FROM clause.  Reuse it.  tag-select-0486
        let mut p_prior_subq: *const Subquery = core::ptr::null();
        /// Materialize the view.  If the view is not correlated, generate a
        ///* subroutine to do the materialization so that subsequent uses of
        ///* the same view can reuse the materialization.  tag-select-0488
        let mut top_addr: i32 = 0;
        let mut once_addr: i32 = 0;
        /// If the subquery is not correlated and if we are not inside of
        ///* a trigger, then we only need to compute the value of the subquery
        ///* once.
        let mut p_cte_use_1: *mut CteUse = core::ptr::null_mut();
        /// Various elements of the SELECT copied into local variables for
        ///* convenience
        /// tag-select-0500
        ///*
        ///* If the query is DISTINCT with an ORDER BY but is not an aggregate, and
        ///* if the select-list is the same as the ORDER BY list, then this query
        ///* can be rewritten as a GROUP BY. In other words, this:
        ///*
        ///*     SELECT DISTINCT xyz FROM ... ORDER BY xyz
        ///*
        ///* is transformed to:
        ///*
        ///*     SELECT xyz FROM ... GROUP BY xyz ORDER BY xyz
        ///*
        ///* The second form is preferred as a single index (or temp-table) may be
        ///* used for both the ORDER BY and DISTINCT processing. As originally
        ///* written the query must use a temp-table for at least one of the ORDER
        ///* BY and DISTINCT, and an index or separate temp-table for the other.
        /// Notice that even thought SF_Distinct has been cleared from p->selFlags,
        ///* the sDistinct.isTnct is still set.  Hence, isTnct represents the
        ///* original setting of the SF_Distinct flag, not the current setting
        /// If there is an ORDER BY clause, then create an ephemeral index to
        ///* do the sorting.  But this sorting ephemeral index might end up
        ///* being unused if the data can be extracted in pre-sorted order.
        ///* If that is the case, then the OP_OpenEphemeral instruction will be
        ///* changed to an OP_Noop once we figure out that the sorting index is
        ///* not needed.  The sSort.addrSortIndex variable is used to facilitate
        ///* that change.  tag-select-0600
        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
        /// If the output is destined for a temporary table, open that table.
        ///* tag-select-0630
        /// Delete or NULL-out result columns that will never be used
        let mut ii: i32 = 0;
        /// Set the limiter.  tag-select-0650
        /// 4 billion rows
        /// Open an ephemeral index to use for the distinct set. tag-select-0680
        /// No aggregate functions and no GROUP BY clause.  tag-select-0700
        let mut wctrl_flags: u16 = 0 as u16;
        let mut p_win: *const Window = core::ptr::null();
        /// Main window object (or NULL)
        /// Begin the database scan.
        /// TUNING: For a UNION CTE, because UNION is implies DISTINCT,
        ///* reduce the estimated output row count by 8 (LogEst 30). 
        ///* Search for tag-20250414a to see other cases
        /// If sorting index that was created by a prior OP_OpenEphemeral
        ///* instruction ended up not being needed, then change the OP_OpenEphemeral
        ///* into an OP_Noop.
        let mut addr_gosub: i32 = 0;
        let mut i_cont: i32 = 0;
        let mut i_break: i32 = 0;
        let mut reg_gosub: i32 = 0;
        /// SQLITE_OMIT_WINDOWFUNC
        /// Use the standard inner loop.
        /// End the database scan loop.
        /// This case is for when there exist aggregate functions or a GROUP BY
        ///* clause or both.  tag-select-0800
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        /// Name context for processing aggregate information
        let mut i_a_mem: i32 = 0;
        /// First Mem address for storing current GROUP BY
        let mut i_b_mem: i32 = 0;
        /// First Mem address for previous GROUP BY
        let mut i_use_flag: i32 = 0;
        /// Mem address holding flag indicating that at least
        ///* one row of the input to the aggregator has been
        ///* processed
        let mut i_abort_flag: i32 = 0;
        /// Mem address which causes query abort if positive
        let mut group_by_sort: i32 = 0;
        /// Rows come from source in GROUP BY order
        let mut addr_end: i32 = 0;
        /// End of processing for this SELECT
        let mut sort_p_tab: i32 = 0;
        /// Pseudotable used to decode sorting results
        let mut sort_out: i32 = 0;
        /// Output register from the sorter
        let mut order_by_grp: i32 = 0;
        /// True if the GROUP BY and ORDER BY are the same
        /// Remove any and all aliases between the result set and the
        ///* GROUP BY clause.
        let mut k: i32 = 0;
        /// Loop counter
        let mut p_item_2: *mut ExprListItem = core::ptr::null_mut();
        /// For looping over expression in a list
        /// If there is both a GROUP BY and an ORDER BY clause and they are
        ///* identical, then it may be possible to disable the ORDER BY clause
        ///* on the grounds that the GROUP BY will cause elements to come out
        ///* in the correct order. It also may not - the GROUP BY might use a
        ///* database index that causes rows to be grouped together as required
        ///* but not actually sorted. Either way, record the fact that the
        ///* ORDER BY and GROUP BY clauses are the same by setting the orderByGrp
        ///* variable.
        /// Create a label to jump to when we want to abort the query
        /// Convert TK_COLUMN nodes into TK_AGG_COLUMN and make entries in
        ///* sAggInfo for all TK_AGG_FUNCTION nodes in expressions of the
        ///* SELECT statement.
        /// Processing for aggregates with GROUP BY is very different and
        ///* much more complex than aggregates without a GROUP BY.  tag-select-0810
        let mut p_key_info_1: *mut KeyInfo = core::ptr::null_mut();
        /// Keying information for the group by clause
        let mut addr1: i32 = 0;
        /// A-vs-B comparison jump
        let mut addr_output_row: i32 = 0;
        /// Start of subroutine that outputs a result row
        let mut reg_output_row: i32 = 0;
        /// Return address register for output subroutine
        let mut addr_set_abort: i32 = 0;
        /// Set the abort flag and return
        let mut addr_top_of_loop: i32 = 0;
        /// Top of the input loop
        let mut addr_sorting_idx: i32 = 0;
        /// The OP_OpenEphemeral for the sorting index
        let mut addr_reset: i32 = 0;
        /// Subroutine for resetting the accumulator
        let mut reg_reset: i32 = 0;
        /// Return address register for reset subroutine
        let mut p_distinct: *mut ExprList = core::ptr::null_mut();
        let mut dist_flag: u16 = 0 as u16;
        let mut e_dist: i32 = 0;
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        /// If there is a GROUP BY clause we might need a sorting index to
        ///* implement it.  Allocate that sorting index now.  If it turns out
        ///* that we do not need it after all, the OP_SorterOpen instruction
        ///* will be converted into a Noop.
        /// Initialize memory locations used by GROUP BY aggregate processing
        /// Begin a loop that will extract all source rows in GROUP BY order.
        ///* This might involve two separate loops with an OP_Sort in between, or
        ///* it might be a single loop that uses an index to extract information
        ///* in the right order to begin with.
        /// The optimizer is able to deliver rows in group by order so
        ///* we do not have to sort.  The OP_OpenEphemeral table will be
        ///* cancelled later because we still need to use the pKeyInfo
        /// Rows are coming out in undetermined order.  We have to push
        ///* each row into a sorting index, terminate the first loop,
        ///* then loop over the sorting index in order to get the output
        ///* in sorted order
        let mut reg_base: i32 = 0;
        let mut reg_record: i32 = 0;
        let mut n_col: i32 = 0;
        let mut n_group_by: i32 = 0;
        let mut p_col: *const AggInfoCol = core::ptr::null();
        /// If there are entries in pAgggInfo->aFunc[] that contain subexpressions
        ///* that are indexed (and that were previously identified and tagged
        ///* in optimizeAggregateUseOfIndexedExpr()) then those subexpressions
        ///* must now be converted into a TK_AGG_COLUMN node so that the value
        ///* is correctly pulled from the index rather than being recomputed.
        /// If the index or temporary table used by the GROUP BY sort
        ///* will naturally deliver rows in the order required by the ORDER BY
        ///* clause, cancel the ephemeral table open coded earlier.
        ///*
        ///* This is an optimization - the correct answer should result regardless.
        ///* Use the SQLITE_GroupByOrder flag with SQLITE_TESTCTRL_OPTIMIZER to
        ///* disable this optimization for testing purposes.
        /// Evaluate the current GROUP BY terms and store in b0, b1, b2...
        ///* (b0 is memory location iBMem+0, b1 is iBMem+1, and so forth)
        ///* Then compare the current GROUP BY terms against the GROUP BY terms
        ///* from the previous row currently stored in a0, a1, a2...
        let mut i_order_by_col: i32 = 0;
        let mut p_x: *mut Expr = core::ptr::null_mut();
        let mut p_base: *const Expr = core::ptr::null();
        /// Generate code that runs whenever the GROUP BY changes.
        ///* Changes in the GROUP BY are detected by the previous code
        ///* block.  If there were no changes, this block is skipped.
        ///*
        ///* This code copies current group by terms in b0,b1,b2,...
        ///* over to a0,a1,a2.  It then calls the output subroutine
        ///* and resets the aggregate accumulator registers in preparation
        ///* for the next GROUP BY batch.
        /// Update the aggregate accumulators based on the content of
        ///* the current row
        /// End of the loop
        /// Output the final row of result
        /// Jump over the subroutines
        /// Generate a subroutine that outputs a single row of the result
        ///* set.  This subroutine first looks at the iUseFlag.  If iUseFlag
        ///* is less than or equal to zero, the subroutine is a no-op.  If
        ///* the processing calls for the query to abort, this subroutine
        ///* increments the iAbortFlag memory location before returning in
        ///* order to signal the caller to abort.
        /// Generate a subroutine that will reset the group-by accumulator
        let mut p_f: *const AggInfoFunc = core::ptr::null();
        /// endif pGroupBy.  Begin aggregate queries without GROUP BY:
        /// Aggregate functions without GROUP BY. tag-select-0820
        let mut p_tab_1: *mut Table = core::ptr::null_mut();
        /// tag-select-0821
        ///*
        ///* If isSimpleCount() returns a pointer to a Table structure, then
        ///* the SQL statement is of the form:
        ///*
        ///*   SELECT count(*) FROM <tbl>
        ///*
        ///* where the Table structure returned represents table <tbl>.
        ///*
        ///* This statement is so common that it is optimized specially. The
        ///* OP_Count instruction is executed either on the intkey table that
        ///* contains the data for table <tbl> or on one of its indexes. It
        ///* is better to execute the op on an index, as indexes are almost
        ///* always spread across less pages than their corresponding tables.
        let mut i_db_1: i32 = 0;
        let mut i_csr: i32 = 0;
        /// Cursor to scan b-tree
        let mut p_idx: *mut Index = core::ptr::null_mut();
        /// Iterator variable
        let mut p_key_info_2: *mut KeyInfo = core::ptr::null_mut();
        /// Keyinfo for scanned index
        let mut p_best: *mut Index = core::ptr::null_mut();
        /// Best index found so far
        let mut i_root: Pgno = 0 as Pgno;
        /// Root page of scanned b-tree
        /// Search for the index that has the lowest scan cost.
        ///*
        ///* (2011-04-15) Do not do a full scan of an unordered index.
        ///*
        ///* (2013-10-03) Do not count the entries in a partial index.
        ///*
        ///* In practice the KeyInfo structure will not be used. It is only
        ///* passed to keep OP_OpenRead happy.
        /// Open a read-only cursor, execute the OP_Count, close the cursor.
        /// The general case of an aggregate query without GROUP BY
        ///* tag-select-0822
        let mut reg_acc: i32 = 0;
        /// "populate accumulators" flag
        let mut p_distinct_1: *mut ExprList = core::ptr::null_mut();
        let mut dist_flag_1: u16 = 0 as u16;
        let mut e_dist_1: i32 = 0;
        /// If there are accumulator registers but no min() or max() functions
        ///* without FILTER clauses, allocate register regAcc. Register regAcc
        ///* will contain 0 the first time the inner loop runs, and 1 thereafter.
        ///* The code generated by updateAccumulator() uses this to ensure
        ///* that the accumulator registers are (a) updated only once if
        ///* there are no min() or max functions or (b) always updated for the
        ///* first row visited by the aggregate, so that they are updated at
        ///* least once even if the FILTER clause means the min() or max()
        ///* function visits zero rows.
        /// This case runs if the aggregate has no GROUP BY clause.  The
        ///* processing is much simpler since there is only a single row
        ///* of output.
        /// If this query is a candidate for the min/max optimization, then
        ///* minMaxFlag will have been previously set to either
        ///* WHERE_ORDERBY_MIN or WHERE_ORDERBY_MAX and pMinMaxOrderBy will
        ///* be an appropriate ORDER BY expression for the optimization.
        let mut p_f_1: *const AggInfoFunc = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s119:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => { { let _ = 0; }; __state = 631; }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { p_e_list = core::ptr::null_mut(); __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { p_agg_info = core::ptr::null_mut(); __state = 12; }
                    12 => { rc = 1; __state = 13; }
                    13 => { __state = 14; }
                    14 => { __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => {
                        p_min_max_order_by = core::ptr::null_mut();
                        __state = 18;
                    }
                    18 => { __state = 19; }
                    19 => { db = unsafe { (*p_parse).db }; __state = 20; }
                    20 => { { let _ = 0; }; __state = 21; }
                    21 => { v = sqlite3_get_vdbe(p_parse); __state = 22; }
                    22 => {
                        if p == core::ptr::null_mut() ||
                                unsafe { (*p_parse).n_err } != 0 {
                            __state = 24;
                        } else { __state = 23; }
                    }
                    23 => { { let _ = 0; }; __state = 25; }
                    24 => { return 1; }
                    25 => {
                        if unsafe {
                                    sqlite3_auth_check(p_parse, 21, core::ptr::null(),
                                        core::ptr::null(), core::ptr::null())
                                } != 0 {
                            __state = 27;
                        } else { __state = 26; }
                    }
                    26 => { { let _ = 0; }; __state = 28; }
                    27 => { return 1; }
                    28 => { { let _ = 0; }; __state = 29; }
                    29 => { { let _ = 0; }; __state = 30; }
                    30 => { { let _ = 0; }; __state = 31; }
                    31 => {
                        if unsafe { (*p_dest).e_dest } as i32 <= 4 {
                            __state = 33;
                        } else { __state = 32; }
                    }
                    32 => {
                        sqlite3_select_prep(p_parse, p, core::ptr::null_mut());
                        __state = 39;
                    }
                    33 => { { let _ = 0; }; __state = 34; }
                    34 => {
                        if !(unsafe { (*p).p_order_by }).is_null() {
                            __state = 36;
                        } else { __state = 35; }
                    }
                    35 => {
                        unsafe { (*p).sel_flags &= !(1 as u32) };
                        __state = 32;
                    }
                    36 => {
                        unsafe {
                            sqlite3_parser_add_cleanup(p_parse,
                                Some(sqlite3_expr_list_delete_generic),
                                unsafe { (*p).p_order_by } as *mut ())
                        };
                        __state = 37;
                    }
                    37 => { __state = 38; }
                    38 => {
                        unsafe { (*p).p_order_by = core::ptr::null_mut() };
                        __state = 35;
                    }
                    39 => {
                        if unsafe { (*p_parse).n_err } != 0 {
                            __state = 41;
                        } else { __state = 40; }
                    }
                    40 => { { let _ = 0; }; __state = 42; }
                    41 => { __state = 2; }
                    42 => { { let _ = 0; }; __state = 43; }
                    43 => {
                        if unsafe { (*p).sel_flags } & 8388608 as u32 != 0 {
                            __state = 45;
                        } else { __state = 44; }
                    }
                    44 => {
                        if unsafe { (*p_dest).e_dest } as i32 == 7 {
                            __state = 51;
                        } else { __state = 50; }
                    }
                    45 => {
                        p0 =
                            unsafe {
                                &mut *(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)
                            };
                        __state = 46;
                    }
                    46 => {
                        if same_src_alias(p0,
                                    unsafe { &mut *unsafe { (*p).p_src } }) != 0 {
                            __state = 48;
                        } else { __state = 47; }
                    }
                    47 => {
                        unsafe { (*p).sel_flags &= !(8388608 as u32) };
                        __state = 44;
                    }
                    48 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"target object/alias may not appear in FROM clause: %s".as_ptr()
                                        as *mut i8 as *const i8,
                                if !(unsafe { (*p0).z_alias }).is_null() {
                                    unsafe { (*p0).z_alias }
                                } else { unsafe { (*unsafe { (*p0).p_s_tab }).z_name } })
                        };
                        __state = 49;
                    }
                    49 => { __state = 2; }
                    50 => {
                        if unsafe { sqlite3_window_rewrite(p_parse, p) } != 0 {
                            __state = 53;
                        } else { __state = 52; }
                    }
                    51 => {
                        sqlite3_generate_column_names(p_parse, p);
                        __state = 50;
                    }
                    52 => { p_tab_list = unsafe { (*p).p_src }; __state = 55; }
                    53 => { { let _ = 0; }; __state = 54; }
                    54 => { __state = 2; }
                    55 => {
                        is_agg =
                            (unsafe { (*p).sel_flags } & 8 as u32 != 0 as u32) as i32;
                        __state = 56;
                    }
                    56 => {
                        unsafe {
                            memset(&raw mut s_sort as *mut (), 0,
                                core::mem::size_of::<SortCtx>() as u64)
                        };
                        __state = 57;
                    }
                    57 => {
                        s_sort.p_order_by = unsafe { (*p).p_order_by };
                        __state = 58;
                    }
                    58 => { i = 0; __state = 60; }
                    59 => {
                        if !(unsafe { (*p).p_prior }).is_null() {
                            __state = 118;
                        } else { __state = 117; }
                    }
                    60 => {
                        if (unsafe { (*p).p_prior }).is_null() as i32 != 0 &&
                                i < unsafe { (*p_tab_list).n_src } {
                            __state = 61;
                        } else { __state = 59; }
                    }
                    61 => {
                        p_item =
                            unsafe {
                                &mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)
                            };
                        __state = 63;
                    }
                    62 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 60;
                    }
                    63 => {
                        p_sub =
                            if unsafe { (*p_item).fg.is_subquery() } != 0 {
                                unsafe { (*unsafe { (*p_item).u4.p_subq }).p_select }
                            } else { core::ptr::null_mut() };
                        __state = 64;
                    }
                    64 => {
                        p_tab = unsafe { (*p_item).p_s_tab } as *const Table;
                        __state = 65;
                    }
                    65 => { { let _ = 0; }; __state = 66; }
                    66 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & (8 | 64) != 0
                                    &&
                                    unsafe {
                                            sqlite3_expr_implies_non_null_row(unsafe { (*p).p_where },
                                                unsafe { (*p_item).i_cursor },
                                                unsafe { (*p_item).fg.jointype } as i32 & 64)
                                        } != 0 &&
                                unsafe { (*db).db_opt_flags } & 8192 as u32 == 0 as u32 {
                            __state = 68;
                        } else { __state = 67; }
                    }
                    67 => {
                        if p_sub == core::ptr::null_mut() {
                            __state = 94;
                        } else { __state = 93; }
                    }
                    68 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 8 != 0 {
                            __state = 70;
                        } else { __state = 69; }
                    }
                    69 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 64 != 0 {
                            __state = 76;
                        } else { __state = 67; }
                    }
                    70 => {
                        if unsafe { (*p_item).fg.jointype } as i32 & 16 != 0 {
                            __state = 71;
                        } else { __state = 72; }
                    }
                    71 => { __state = 73; }
                    72 => { __state = 74; }
                    73 => {
                        unsafe { (*p_item).fg.jointype &= !8 as u8 };
                        __state = 69;
                    }
                    74 => {
                        unsafe { (*p_item).fg.jointype &= !(8 | 32) as u8 };
                        __state = 75;
                    }
                    75 => {
                        unset_join_expr(unsafe { (*p).p_where },
                            unsafe { (*p_item).i_cursor }, 0);
                        __state = 69;
                    }
                    76 => { j = i + 1; __state = 78; }
                    77 => {
                        j = unsafe { (*p_tab_list).n_src } - 1;
                        __state = 88;
                    }
                    78 => {
                        if j < unsafe { (*p_tab_list).n_src } {
                            __state = 79;
                        } else { __state = 77; }
                    }
                    79 => {
                        p_i2 =
                            unsafe {
                                &mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(j as isize)
                            };
                        __state = 81;
                    }
                    80 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 78;
                    }
                    81 => {
                        if unsafe { (*p_i2).fg.jointype } as i32 & 16 != 0 {
                            __state = 82;
                        } else { __state = 80; }
                    }
                    82 => {
                        if unsafe { (*p_i2).fg.jointype } as i32 & 8 != 0 {
                            __state = 83;
                        } else { __state = 84; }
                    }
                    83 => { __state = 85; }
                    84 => { __state = 86; }
                    85 => {
                        unsafe { (*p_i2).fg.jointype &= !16 as u8 };
                        __state = 80;
                    }
                    86 => {
                        unsafe { (*p_i2).fg.jointype &= !(16 | 32) as u8 };
                        __state = 87;
                    }
                    87 => {
                        unset_join_expr(unsafe { (*p).p_where },
                            unsafe { (*p_i2).i_cursor }, 1);
                        __state = 80;
                    }
                    88 => { if j >= 0 { __state = 89; } else { __state = 67; } }
                    89 => {
                        unsafe {
                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                    *mut SrcItem).offset(j as isize)).fg.jointype &= !64 as u8
                        };
                        __state = 91;
                    }
                    90 => {
                        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                        __state = 88;
                    }
                    91 => {
                        if unsafe {
                                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                *mut SrcItem).offset(j as isize)).fg.jointype
                                        } as i32 & 16 != 0 {
                            __state = 92;
                        } else { __state = 90; }
                    }
                    92 => { __state = 67; }
                    93 => {
                        if unsafe { (*p_tab).n_col } as i32 !=
                                unsafe { (*unsafe { (*p_sub).p_e_list }).n_expr } {
                            __state = 96;
                        } else { __state = 95; }
                    }
                    94 => { __state = 62; }
                    95 => {
                        if unsafe { (*p_item).fg.is_cte() } != 0 &&
                                unsafe { (*unsafe { (*p_item).u2.p_cte_use }).e_m10d } as
                                        i32 == 0 {
                            __state = 99;
                        } else { __state = 98; }
                    }
                    96 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"expected %d columns for \'%s\' but got %d".as_ptr() as
                                        *mut i8 as *const i8, unsafe { (*p_tab).n_col } as i32,
                                unsafe { (*p_tab).z_name },
                                unsafe { (*unsafe { (*p_sub).p_e_list }).n_expr })
                        };
                        __state = 97;
                    }
                    97 => { __state = 2; }
                    98 => {
                        if unsafe { (*p_sub).sel_flags } & 8 as u32 != 0 as u32 {
                            __state = 101;
                        } else { __state = 100; }
                    }
                    99 => { __state = 62; }
                    100 => { { let _ = 0; }; __state = 102; }
                    101 => { __state = 62; }
                    102 => {
                        if unsafe { (*p_sub).p_order_by } != core::ptr::null_mut()
                                                &&
                                                (unsafe { (*p).p_order_by } != core::ptr::null_mut() ||
                                                    unsafe { (*p_tab_list).n_src } > 1) &&
                                            unsafe { (*p_sub).p_limit } == core::ptr::null_mut() &&
                                        unsafe { (*p_sub).sel_flags } & (134217728 | 8192) as u32 ==
                                            0 as u32 &&
                                    unsafe { (*p).sel_flags } & 134217728 as u32 == 0 as u32 &&
                                unsafe { (*db).db_opt_flags } & 262144 as u32 == 0 as u32 {
                            __state = 104;
                        } else { __state = 103; }
                    }
                    103 => {
                        if unsafe { (*p_sub).p_order_by } != core::ptr::null_mut()
                                        && i == 0 &&
                                    unsafe { (*p).sel_flags } & 262144 as u32 != 0 as u32 &&
                                (unsafe { (*p_tab_list).n_src } == 1 ||
                                    unsafe {
                                                    (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                                        *mut SrcItem).offset(1 as isize)).fg.jointype
                                                } as i32 & (32 | 2) != 0) {
                            __state = 108;
                        } else { __state = 107; }
                    }
                    104 => { __state = 105; }
                    105 => {
                        unsafe {
                            sqlite3_parser_add_cleanup(p_parse,
                                Some(sqlite3_expr_list_delete_generic),
                                unsafe { (*p_sub).p_order_by } as *mut ())
                        };
                        __state = 106;
                    }
                    106 => {
                        unsafe { (*p_sub).p_order_by = core::ptr::null_mut() };
                        __state = 103;
                    }
                    107 => {
                        if flatten_subquery(p_parse, p, i, is_agg) != 0 {
                            __state = 110;
                        } else { __state = 109; }
                    }
                    108 => { __state = 62; }
                    109 => {
                        p_tab_list = unsafe { (*p).p_src };
                        __state = 113;
                    }
                    110 => {
                        if unsafe { (*p_parse).n_err } != 0 {
                            __state = 112;
                        } else { __state = 111; }
                    }
                    111 => { i = -1; __state = 109; }
                    112 => { __state = 2; }
                    113 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 115;
                        } else { __state = 114; }
                    }
                    114 => {
                        if !(unsafe { (*p_dest).e_dest } as i32 <= 6) as i32 != 0 {
                            __state = 116;
                        } else { __state = 62; }
                    }
                    115 => { __state = 2; }
                    116 => {
                        s_sort.p_order_by = unsafe { (*p).p_order_by };
                        __state = 62;
                    }
                    117 => {
                        if unsafe { (*p_parse).b_has_exists() } != 0 &&
                                unsafe { (*db).db_opt_flags } & 1073741824 as u32 ==
                                    0 as u32 {
                            __state = 123;
                        } else { __state = 122; }
                    }
                    118 => {
                        rc = multi_select(p_parse, p, p_dest);
                        __state = 119;
                    }
                    119 => {
                        if unsafe { (*p).p_next } == core::ptr::null_mut() {
                            __state = 121;
                        } else { __state = 120; }
                    }
                    120 => { return rc; }
                    121 => {
                        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
                        __state = 120;
                    }
                    122 => {
                        if unsafe { (*p).p_where } != core::ptr::null_mut() &&
                                        unsafe { (*unsafe { (*p).p_where }).op } as i32 == 44 &&
                                    unsafe { (*db).db_opt_flags } & 32768 as u32 == 0 as u32 &&
                                propagate_constants(p_parse, unsafe { &*p }) != 0 {
                            __state = 126;
                        } else { __state = 127; }
                    }
                    123 => {
                        exists_to_join(p_parse, p, unsafe { (*p).p_where });
                        __state = 124;
                    }
                    124 => {
                        p_tab_list = unsafe { (*p).p_src };
                        __state = 122;
                    }
                    125 => {
                        if unsafe { (*db).db_opt_flags } & (1 | 512) as u32 ==
                                    0 as u32 &&
                                count_of_view_optimization(p_parse, unsafe { &mut *p }) != 0
                            {
                            __state = 129;
                        } else { __state = 128; }
                    }
                    126 => { __state = 125; }
                    127 => { __state = 125; }
                    128 => { i = 0; __state = 133; }
                    129 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 131;
                        } else { __state = 130; }
                    }
                    130 => {
                        p_tab_list = unsafe { (*p).p_src };
                        __state = 128;
                    }
                    131 => { __state = 2; }
                    132 => {
                        p_e_list = unsafe { (*p).p_e_list };
                        __state = 229;
                    }
                    133 => {
                        if i < unsafe { (*p_tab_list).n_src } {
                            __state = 134;
                        } else { __state = 132; }
                    }
                    134 => {
                        p_item_1 =
                            unsafe {
                                &mut *(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)
                            };
                        __state = 136;
                    }
                    135 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 133;
                    }
                    136 => { __state = 137; }
                    137 => { __state = 138; }
                    138 => { __state = 139; }
                    139 => { __state = 140; }
                    140 => { __state = 141; }
                    141 => {
                        if unsafe { (*p_item_1).col_used } == 0 as u64 &&
                                unsafe { (*p_item_1).z_name } != core::ptr::null_mut() {
                            __state = 143;
                        } else { __state = 142; }
                    }
                    142 => {
                        if unsafe { (*p_item_1).fg.is_subquery() } as i32 == 0 {
                            __state = 152;
                        } else { __state = 151; }
                    }
                    143 => { __state = 144; }
                    144 => {
                        if unsafe { (*p_item_1).fg.fixed_schema() } != 0 {
                            __state = 146;
                        } else { __state = 147; }
                    }
                    145 => {
                        unsafe {
                            sqlite3_auth_check(p_parse, 20,
                                unsafe { (*p_item_1).z_name } as *const i8,
                                c"".as_ptr() as *mut i8 as *const i8, z_db)
                        };
                        __state = 142;
                    }
                    146 => {
                        i_db =
                            unsafe {
                                sqlite3_schema_to_index(unsafe { (*p_parse).db },
                                    unsafe { (*p_item_1).u4.p_schema })
                            };
                        __state = 148;
                    }
                    147 => {
                        if unsafe { (*p_item_1).fg.is_subquery() } != 0 {
                            __state = 149;
                        } else { __state = 150; }
                    }
                    148 => {
                        z_db =
                            unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                } as *const i8;
                        __state = 145;
                    }
                    149 => { z_db = core::ptr::null(); __state = 145; }
                    150 => {
                        z_db = unsafe { (*p_item_1).u4.z_database } as *const i8;
                        __state = 145;
                    }
                    151 => {
                        p_subq = unsafe { (*p_item_1).u4.p_subq };
                        __state = 153;
                    }
                    152 => { __state = 135; }
                    153 => { { let _ = 0; }; __state = 154; }
                    154 => {
                        p_sub_1 = unsafe { (*p_subq).p_select };
                        __state = 155;
                    }
                    155 => {
                        if unsafe { (*p_subq).addr_fill_sub } != 0 {
                            __state = 157;
                        } else { __state = 156; }
                    }
                    156 => {
                        unsafe {
                            (*p_parse).n_height +=
                                unsafe { sqlite3_select_expr_height(p as *const Select) }
                        };
                        __state = 158;
                    }
                    157 => { __state = 135; }
                    158 => {
                        if unsafe { (*db).db_opt_flags } & 4096 as u32 == 0 as u32
                                    &&
                                    (unsafe { (*p_item_1).fg.is_cte() } as i32 == 0 ||
                                        unsafe { (*unsafe { (*p_item_1).u2.p_cte_use }).e_m10d } as
                                                    i32 != 0 &&
                                            unsafe { (*unsafe { (*p_item_1).u2.p_cte_use }).n_use } < 2)
                                &&
                                push_down_where_terms(p_parse, p_sub_1,
                                        unsafe { (*p).p_where }, p_tab_list, i) != 0 {
                            __state = 160;
                        } else { __state = 161; }
                    }
                    159 => {
                        if unsafe { (*db).db_opt_flags } & 67108864 as u32 ==
                                    0 as u32 &&
                                disable_unused_subquery_result_columns(unsafe {
                                            &*p_item_1
                                        }) != 0 {
                            __state = 163;
                        } else { __state = 162; }
                    }
                    160 => { { let _ = 0; }; __state = 159; }
                    161 => { __state = 159; }
                    162 => {
                        z_saved_auth_context = unsafe { (*p_parse).z_auth_context };
                        __state = 164;
                    }
                    163 => { __state = 162; }
                    164 => {
                        unsafe {
                            (*p_parse).z_auth_context =
                                unsafe { (*p_item_1).z_name } as *const i8
                        };
                        __state = 165;
                    }
                    165 => {
                        if from_clause_term_can_be_coroutine(unsafe { &*p_parse },
                                    p_tab_list, i, unsafe { (*p).sel_flags } as i32) != 0 {
                            __state = 167;
                        } else { __state = 168; }
                    }
                    166 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 227;
                        } else { __state = 226; }
                    }
                    167 => {
                        addr_top = unsafe { sqlite3_vdbe_current_addr(v) } + 1;
                        __state = 169;
                    }
                    168 => {
                        if unsafe { (*p_item_1).fg.is_cte() } != 0 &&
                                unsafe { (*unsafe { (*p_item_1).u2.p_cte_use }).addr_m9e } >
                                    0 {
                            __state = 183;
                        } else { __state = 184; }
                    }
                    169 => {
                        unsafe {
                            (*p_subq).reg_return =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 170;
                    }
                    170 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 11, unsafe { (*p_subq).reg_return },
                                0, addr_top)
                        };
                        __state = 171;
                    }
                    171 => { __state = 172; }
                    172 => {
                        unsafe { (*p_subq).addr_fill_sub = addr_top };
                        __state = 173;
                    }
                    173 => {
                        sqlite3_select_dest_init(&mut dest, 11,
                            unsafe { (*p_subq).reg_return });
                        __state = 174;
                    }
                    174 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 1 as u8,
                                c"CO-ROUTINE %!S".as_ptr() as *mut i8 as *const i8,
                                p_item_1)
                        };
                        __state = 175;
                    }
                    175 => {
                        sqlite3_select(p_parse, p_sub_1, &mut dest);
                        __state = 176;
                    }
                    176 => {
                        unsafe {
                            (*unsafe { (*p_item_1).p_s_tab }).n_row_log_est =
                                unsafe { (*p_sub_1).n_select_row }
                        };
                        __state = 177;
                    }
                    177 => {
                        unsafe {
                            (*p_item_1).fg.set_via_coroutine(1 as u32 as u32)
                        };
                        __state = 178;
                    }
                    178 => {
                        unsafe { (*p_subq).reg_result = dest.i_sdst };
                        __state = 179;
                    }
                    179 => {
                        unsafe {
                            sqlite3_vdbe_end_coroutine(v,
                                unsafe { (*p_subq).reg_return })
                        };
                        __state = 180;
                    }
                    180 => { __state = 181; }
                    181 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_top - 1) };
                        __state = 182;
                    }
                    182 => {
                        unsafe { sqlite3_clear_temp_reg_cache(p_parse) };
                        __state = 166;
                    }
                    183 => {
                        p_cte_use =
                            unsafe { (*p_item_1).u2.p_cte_use } as *const CteUse;
                        __state = 185;
                    }
                    184 => {
                        if {
                                    p_prior =
                                        is_self_join_view(unsafe { &mut *p_tab_list },
                                            unsafe { &*p_item_1 }, 0, i);
                                    p_prior
                                } != core::ptr::null_mut() {
                            __state = 190;
                        } else { __state = 191; }
                    }
                    185 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, unsafe { (*p_cte_use).reg_rtn },
                                unsafe { (*p_cte_use).addr_m9e })
                        };
                        __state = 186;
                    }
                    186 => {
                        if unsafe { (*p_item_1).i_cursor } !=
                                unsafe { (*p_cte_use).i_cur } {
                            __state = 188;
                        } else { __state = 187; }
                    }
                    187 => {
                        unsafe {
                            (*p_sub_1).n_select_row = unsafe { (*p_cte_use).n_row_est }
                        };
                        __state = 166;
                    }
                    188 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 117,
                                unsafe { (*p_item_1).i_cursor },
                                unsafe { (*p_cte_use).i_cur })
                        };
                        __state = 189;
                    }
                    189 => { __state = 187; }
                    190 => { __state = 192; }
                    191 => { __state = 199; }
                    192 => { { let _ = 0; }; __state = 193; }
                    193 => {
                        p_prior_subq = unsafe { (*p_prior).u4.p_subq };
                        __state = 194;
                    }
                    194 => { { let _ = 0; }; __state = 195; }
                    195 => {
                        if unsafe { (*p_prior_subq).addr_fill_sub } != 0 {
                            __state = 197;
                        } else { __state = 196; }
                    }
                    196 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 117,
                                unsafe { (*p_item_1).i_cursor },
                                unsafe { (*p_prior).i_cursor })
                        };
                        __state = 198;
                    }
                    197 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10,
                                unsafe { (*p_prior_subq).reg_return },
                                unsafe { (*p_prior_subq).addr_fill_sub })
                        };
                        __state = 196;
                    }
                    198 => {
                        unsafe {
                            (*p_sub_1).n_select_row =
                                unsafe {
                                    (*unsafe { (*p_prior_subq).p_select }).n_select_row
                                }
                        };
                        __state = 166;
                    }
                    199 => { once_addr = 0; __state = 200; }
                    200 => {
                        unsafe {
                            (*p_subq).reg_return =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 201;
                    }
                    201 => {
                        top_addr = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                        __state = 202;
                    }
                    202 => {
                        unsafe { (*p_subq).addr_fill_sub = top_addr + 1 };
                        __state = 203;
                    }
                    203 => {
                        unsafe {
                            (*p_item_1).fg.set_is_materialized(1 as u32 as u32)
                        };
                        __state = 204;
                    }
                    204 => {
                        if unsafe { (*p_item_1).fg.is_correlated() } as i32 == 0 {
                            __state = 206;
                        } else { __state = 207; }
                    }
                    205 => {
                        sqlite3_select_dest_init(&mut dest, 10,
                            unsafe { (*p_item_1).i_cursor });
                        __state = 210;
                    }
                    206 => {
                        once_addr = unsafe { sqlite3_vdbe_add_op0(v, 15) };
                        __state = 208;
                    }
                    207 => { __state = 205; }
                    208 => { __state = 209; }
                    209 => { __state = 205; }
                    210 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 1 as u8,
                                c"MATERIALIZE %!S".as_ptr() as *mut i8 as *const i8,
                                p_item_1)
                        };
                        __state = 211;
                    }
                    211 => {
                        sqlite3_select(p_parse, p_sub_1, &mut dest);
                        __state = 212;
                    }
                    212 => {
                        unsafe {
                            (*unsafe { (*p_item_1).p_s_tab }).n_row_log_est =
                                unsafe { (*p_sub_1).n_select_row }
                        };
                        __state = 213;
                    }
                    213 => {
                        if once_addr != 0 { __state = 215; } else { __state = 214; }
                    }
                    214 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 69, unsafe { (*p_subq).reg_return },
                                top_addr + 1)
                        };
                        __state = 216;
                    }
                    215 => {
                        unsafe { sqlite3_vdbe_jump_here(v, once_addr) };
                        __state = 214;
                    }
                    216 => { __state = 217; }
                    217 => { __state = 218; }
                    218 => {
                        unsafe { sqlite3_vdbe_jump_here(v, top_addr) };
                        __state = 219;
                    }
                    219 => {
                        unsafe { sqlite3_clear_temp_reg_cache(p_parse) };
                        __state = 220;
                    }
                    220 => {
                        if unsafe { (*p_item_1).fg.is_cte() } != 0 &&
                                unsafe { (*p_item_1).fg.is_correlated() } as i32 == 0 {
                            __state = 221;
                        } else { __state = 166; }
                    }
                    221 => {
                        p_cte_use_1 = unsafe { (*p_item_1).u2.p_cte_use };
                        __state = 222;
                    }
                    222 => {
                        unsafe {
                            (*p_cte_use_1).addr_m9e = unsafe { (*p_subq).addr_fill_sub }
                        };
                        __state = 223;
                    }
                    223 => {
                        unsafe {
                            (*p_cte_use_1).reg_rtn = unsafe { (*p_subq).reg_return }
                        };
                        __state = 224;
                    }
                    224 => {
                        unsafe {
                            (*p_cte_use_1).i_cur = unsafe { (*p_item_1).i_cursor }
                        };
                        __state = 225;
                    }
                    225 => {
                        unsafe {
                            (*p_cte_use_1).n_row_est =
                                unsafe { (*p_sub_1).n_select_row }
                        };
                        __state = 166;
                    }
                    226 => {
                        unsafe {
                            (*p_parse).n_height -=
                                unsafe { sqlite3_select_expr_height(p as *const Select) }
                        };
                        __state = 228;
                    }
                    227 => { __state = 2; }
                    228 => {
                        unsafe { (*p_parse).z_auth_context = z_saved_auth_context };
                        __state = 135;
                    }
                    229 => { p_where = unsafe { (*p).p_where }; __state = 230; }
                    230 => {
                        p_group_by = unsafe { (*p).p_group_by };
                        __state = 231;
                    }
                    231 => {
                        p_having = unsafe { (*p).p_having };
                        __state = 232;
                    }
                    232 => {
                        s_distinct.is_tnct =
                            (unsafe { (*p).sel_flags } & 1 as u32 != 0 as u32) as u8;
                        __state = 233;
                    }
                    233 => {
                        if unsafe { (*p).sel_flags } & (1 | 8) as u32 == 1 as u32 &&
                                            sqlite3_copy_sort_order(unsafe { &mut *p_e_list },
                                                    s_sort.p_order_by as *const ExprList) != 0 &&
                                        unsafe {
                                                sqlite3_expr_list_compare(p_e_list as *const ExprList,
                                                    s_sort.p_order_by as *const ExprList, -1)
                                            } == 0 &&
                                    unsafe { (*db).db_opt_flags } & 4 as u32 == 0 as u32 &&
                                unsafe { (*p).p_win } == core::ptr::null_mut() {
                            __state = 235;
                        } else { __state = 234; }
                    }
                    234 => {
                        if !(s_sort.p_order_by).is_null() {
                            __state = 246;
                        } else { __state = 247; }
                    }
                    235 => {
                        unsafe { (*p).sel_flags &= !(1 as u32) };
                        __state = 236;
                    }
                    236 => {
                        p_group_by =
                            {
                                unsafe {
                                    (*p).p_group_by =
                                        unsafe {
                                            sqlite3_expr_list_dup(db, p_e_list as *const ExprList, 0)
                                        }
                                };
                                unsafe { (*p).p_group_by }
                            };
                        __state = 237;
                    }
                    237 => {
                        if !(p_group_by).is_null() {
                            __state = 239;
                        } else { __state = 238; }
                    }
                    238 => {
                        unsafe { (*p).sel_flags |= 8 as u32 };
                        __state = 243;
                    }
                    239 => { i = 0; __state = 240; }
                    240 => {
                        if i < unsafe { (*p_group_by).n_expr } {
                            __state = 241;
                        } else { __state = 238; }
                    }
                    241 => {
                        unsafe {
                            (*(unsafe { (*p_group_by).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).u.x.i_order_by_col =
                                (i + 1) as u16
                        };
                        __state = 242;
                    }
                    242 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 240;
                    }
                    243 => { { let _ = 0; }; __state = 244; }
                    244 => { s_distinct.is_tnct = 2 as u8; __state = 234; }
                    245 => {
                        if unsafe { (*p_dest).e_dest } as i32 == 10 {
                            __state = 252;
                        } else { __state = 251; }
                    }
                    246 => { __state = 248; }
                    247 => { s_sort.addr_sort_index = -1; __state = 245; }
                    248 => {
                        p_key_info =
                            sqlite3_key_info_from_expr_list(p_parse,
                                unsafe { &*s_sort.p_order_by }, 0,
                                unsafe { (*p_e_list).n_expr });
                        __state = 249;
                    }
                    249 => {
                        s_sort.i_e_cursor =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 250;
                    }
                    250 => {
                        s_sort.addr_sort_index =
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 120, s_sort.i_e_cursor,
                                    unsafe { (*s_sort.p_order_by).n_expr } + 1 +
                                        unsafe { (*p_e_list).n_expr }, 0,
                                    p_key_info as *mut i8 as *const i8, -9)
                            };
                        __state = 245;
                    }
                    251 => {
                        i_end = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 266;
                    }
                    252 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 120, unsafe { (*p_dest).i_sd_parm },
                                unsafe { (*p_e_list).n_expr })
                        };
                        __state = 253;
                    }
                    253 => {
                        if unsafe { (*p).sel_flags } & 2048 as u32 != 0 {
                            __state = 254;
                        } else { __state = 251; }
                    }
                    254 => { __state = 255; }
                    255 => {
                        ii = unsafe { (*p_e_list).n_expr } - 1;
                        __state = 257;
                    }
                    256 => { ii = 0; __state = 262; }
                    257 => {
                        if ii > 0 &&
                                unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(ii as isize)).fg.b_used()
                                        } as i32 == 0 {
                            __state = 258;
                        } else { __state = 256; }
                    }
                    258 => {
                        unsafe {
                            sqlite3_expr_delete(db,
                                unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(ii as isize)).p_expr
                                })
                        };
                        __state = 260;
                    }
                    259 => {
                        { let __p = &mut ii; let __t = *__p; *__p -= 1; __t };
                        __state = 257;
                    }
                    260 => {
                        unsafe {
                            sqlite3_db_free(db,
                                unsafe {
                                        (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                        *mut ExprListItem).offset(ii as isize)).z_e_name
                                    } as *mut ())
                        };
                        __state = 261;
                    }
                    261 => {
                        {
                            let __p = unsafe { &mut (*p_e_list).n_expr };
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 259;
                    }
                    262 => {
                        if ii < unsafe { (*p_e_list).n_expr } {
                            __state = 263;
                        } else { __state = 251; }
                    }
                    263 => {
                        if unsafe {
                                        (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprListItem).offset(ii as isize)).fg.b_used()
                                    } as i32 == 0 {
                            __state = 265;
                        } else { __state = 264; }
                    }
                    264 => {
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        __state = 262;
                    }
                    265 => {
                        unsafe {
                            (*unsafe {
                                                (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(ii as isize)).p_expr
                                            }).op = 122 as u8
                        };
                        __state = 264;
                    }
                    266 => {
                        if unsafe { (*p).sel_flags } & 16384 as u32 == 0 as u32 {
                            __state = 268;
                        } else { __state = 267; }
                    }
                    267 => {
                        if !(unsafe { (*p).p_limit }).is_null() {
                            __state = 270;
                        } else { __state = 269; }
                    }
                    268 => {
                        unsafe { (*p).n_select_row = 320 as LogEst };
                        __state = 267;
                    }
                    269 => {
                        if unsafe { (*p).i_limit } == 0 &&
                                s_sort.addr_sort_index >= 0 {
                            __state = 272;
                        } else { __state = 271; }
                    }
                    270 => {
                        compute_limit_registers(p_parse, unsafe { &mut *p }, i_end);
                        __state = 269;
                    }
                    271 => {
                        if unsafe { (*p).sel_flags } & 1 as u32 != 0 {
                            __state = 275;
                        } else { __state = 276; }
                    }
                    272 => {
                        unsafe {
                            sqlite3_vdbe_change_opcode(v, s_sort.addr_sort_index,
                                121 as u8)
                        };
                        __state = 273;
                    }
                    273 => { s_sort.sort_flags |= 1 as u8; __state = 271; }
                    274 => {
                        if (is_agg == 0) as i32 != 0 &&
                                p_group_by == core::ptr::null_mut() {
                            __state = 281;
                        } else { __state = 282; }
                    }
                    275 => {
                        s_distinct.tab_tnct =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 277;
                    }
                    276 => { s_distinct.e_tnct_type = 0 as u8; __state = 274; }
                    277 => {
                        s_distinct.addr_tnct =
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 120, s_distinct.tab_tnct, 0, 0,
                                    sqlite3_key_info_from_expr_list(p_parse,
                                                unsafe { &*unsafe { (*p).p_e_list } }, 0, 0) as *mut i8 as
                                        *const i8, -9)
                            };
                        __state = 278;
                    }
                    278 => {
                        unsafe { sqlite3_vdbe_change_p5(v, 8 as u16) };
                        __state = 279;
                    }
                    279 => { s_distinct.e_tnct_type = 3 as u8; __state = 274; }
                    280 => {
                        if s_distinct.e_tnct_type as i32 == 3 {
                            __state = 625;
                        } else { __state = 624; }
                    }
                    281 => {
                        wctrl_flags =
                            (if s_distinct.is_tnct != 0 { 256 } else { 0 } as u32 |
                                    unsafe { (*p).sel_flags } & 16384 as u32) as u16;
                        __state = 283;
                    }
                    282 => { __state = 324; }
                    283 => {
                        p_win = unsafe { (*p).p_win } as *const Window;
                        __state = 284;
                    }
                    284 => {
                        if !(p_win).is_null() {
                            __state = 286;
                        } else { __state = 285; }
                    }
                    285 => { { let _ = 0; }; __state = 287; }
                    286 => {
                        unsafe { sqlite3_window_code_init(p_parse, p) };
                        __state = 285;
                    }
                    287 => { __state = 288; }
                    288 => {
                        p_w_info =
                            unsafe {
                                sqlite3_where_begin(p_parse, p_tab_list, p_where,
                                    s_sort.p_order_by, unsafe { (*p).p_e_list }, p, wctrl_flags,
                                    unsafe { (*p).n_select_row } as i32)
                            };
                        __state = 289;
                    }
                    289 => {
                        if p_w_info == core::ptr::null_mut() {
                            __state = 291;
                        } else { __state = 290; }
                    }
                    290 => {
                        if (unsafe { sqlite3_where_output_row_count(p_w_info) } as
                                        i32) < unsafe { (*p).n_select_row } as i32 {
                            __state = 293;
                        } else { __state = 292; }
                    }
                    291 => { __state = 2; }
                    292 => {
                        if s_distinct.is_tnct != 0 &&
                                unsafe { sqlite3_where_is_distinct(p_w_info) } != 0 {
                            __state = 297;
                        } else { __state = 296; }
                    }
                    293 => {
                        unsafe {
                            (*p).n_select_row =
                                unsafe { sqlite3_where_output_row_count(p_w_info) }
                        };
                        __state = 294;
                    }
                    294 => {
                        if unsafe { (*p_dest).e_dest } as i32 <= 4 &&
                                unsafe { (*p_dest).e_dest } as i32 >= 3 {
                            __state = 295;
                        } else { __state = 292; }
                    }
                    295 => {
                        unsafe { (*p).n_select_row -= 30 as LogEst };
                        __state = 292;
                    }
                    296 => {
                        if !(s_sort.p_order_by).is_null() {
                            __state = 299;
                        } else { __state = 298; }
                    }
                    297 => {
                        s_distinct.e_tnct_type =
                            unsafe { sqlite3_where_is_distinct(p_w_info) } as u8;
                        __state = 296;
                    }
                    298 => { __state = 303; }
                    299 => {
                        s_sort.n_ob_sat =
                            unsafe { sqlite3_where_is_ordered(p_w_info) };
                        __state = 300;
                    }
                    300 => {
                        s_sort.label_ob_lopt =
                            unsafe { sqlite3_where_order_by_limit_opt_label(p_w_info) };
                        __state = 301;
                    }
                    301 => {
                        if s_sort.n_ob_sat == unsafe { (*s_sort.p_order_by).n_expr }
                            {
                            __state = 302;
                        } else { __state = 298; }
                    }
                    302 => {
                        s_sort.p_order_by = core::ptr::null_mut();
                        __state = 298;
                    }
                    303 => {
                        if s_sort.addr_sort_index >= 0 &&
                                s_sort.p_order_by == core::ptr::null_mut() {
                            __state = 305;
                        } else { __state = 304; }
                    }
                    304 => { { let _ = 0; }; __state = 306; }
                    305 => {
                        unsafe {
                            sqlite3_vdbe_change_to_noop(v, s_sort.addr_sort_index)
                        };
                        __state = 304;
                    }
                    306 => {
                        if !(p_win).is_null() {
                            __state = 307;
                        } else { __state = 308; }
                    }
                    307 => {
                        addr_gosub = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 309;
                    }
                    308 => {
                        select_inner_loop(p_parse, p, -1, &mut s_sort,
                            &raw mut s_distinct as *const DistinctCtx,
                            unsafe { &mut *p_dest },
                            unsafe { sqlite3_where_continue_label(p_w_info) },
                            unsafe { sqlite3_where_break_label(p_w_info) });
                        __state = 322;
                    }
                    309 => {
                        i_cont = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 310;
                    }
                    310 => {
                        i_break = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 311;
                    }
                    311 => {
                        reg_gosub =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 312;
                    }
                    312 => {
                        unsafe {
                            sqlite3_window_code_step(p_parse, p, p_w_info, reg_gosub,
                                addr_gosub)
                        };
                        __state = 313;
                    }
                    313 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 9, 0, i_break) };
                        __state = 314;
                    }
                    314 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_gosub) };
                        __state = 315;
                    }
                    315 => { __state = 316; }
                    316 => { s_sort.label_ob_lopt = 0; __state = 317; }
                    317 => {
                        select_inner_loop(p_parse, p, -1, &mut s_sort,
                            &raw mut s_distinct as *const DistinctCtx,
                            unsafe { &mut *p_dest }, i_cont, i_break);
                        __state = 318;
                    }
                    318 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, i_cont) };
                        __state = 319;
                    }
                    319 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_gosub) };
                        __state = 320;
                    }
                    320 => { __state = 321; }
                    321 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, i_break) };
                        __state = 280;
                    }
                    322 => { __state = 323; }
                    323 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 280;
                    }
                    324 => { __state = 325; }
                    325 => { __state = 326; }
                    326 => { __state = 327; }
                    327 => { __state = 328; }
                    328 => { __state = 329; }
                    329 => { __state = 330; }
                    330 => { sort_p_tab = 0; __state = 331; }
                    331 => { sort_out = 0; __state = 332; }
                    332 => { order_by_grp = 0; __state = 333; }
                    333 => {
                        if !(p_group_by).is_null() {
                            __state = 335;
                        } else { __state = 336; }
                    }
                    334 => {
                        addr_end = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 352;
                    }
                    335 => { __state = 337; }
                    336 => { { let _ = 0; }; __state = 351; }
                    337 => { __state = 338; }
                    338 => {
                        {
                            k = unsafe { (*unsafe { (*p).p_e_list }).n_expr };
                            p_item_2 =
                                unsafe { (*unsafe { (*p).p_e_list }).a.as_ptr() } as
                                    *mut ExprListItem
                        };
                        __state = 340;
                    }
                    339 => {
                        {
                            k = unsafe { (*p_group_by).n_expr };
                            p_item_2 =
                                unsafe { (*p_group_by).a.as_ptr() } as *mut ExprListItem
                        };
                        __state = 344;
                    }
                    340 => {
                        if k > 0 { __state = 341; } else { __state = 339; }
                    }
                    341 => {
                        unsafe { (*p_item_2).u.x.i_alias = 0 as u16 };
                        __state = 342;
                    }
                    342 => {
                        {
                            { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p_item_2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 340;
                    }
                    343 => { { let _ = 0; }; __state = 347; }
                    344 => {
                        if k > 0 { __state = 345; } else { __state = 343; }
                    }
                    345 => {
                        unsafe { (*p_item_2).u.x.i_alias = 0 as u16 };
                        __state = 346;
                    }
                    346 => {
                        {
                            { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                            {
                                let __p = &mut p_item_2;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            }
                        };
                        __state = 344;
                    }
                    347 => {
                        if unsafe { (*p).n_select_row } as i32 > 66 {
                            __state = 349;
                        } else { __state = 348; }
                    }
                    348 => {
                        if sqlite3_copy_sort_order(unsafe { &mut *p_group_by },
                                        s_sort.p_order_by as *const ExprList) != 0 &&
                                unsafe {
                                        sqlite3_expr_list_compare(p_group_by as *const ExprList,
                                            s_sort.p_order_by as *const ExprList, -1)
                                    } == 0 {
                            __state = 350;
                        } else { __state = 334; }
                    }
                    349 => {
                        unsafe { (*p).n_select_row = 66 as LogEst };
                        __state = 348;
                    }
                    350 => { order_by_grp = 1; __state = 334; }
                    351 => {
                        unsafe { (*p).n_select_row = 0 as LogEst };
                        __state = 334;
                    }
                    352 => {
                        p_agg_info =
                            unsafe {
                                    sqlite3_db_malloc_zero(db,
                                        core::mem::size_of::<AggInfo>() as u64)
                                } as *mut AggInfo;
                        __state = 353;
                    }
                    353 => {
                        if !(p_agg_info).is_null() {
                            __state = 355;
                        } else { __state = 354; }
                    }
                    354 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 358;
                        } else { __state = 357; }
                    }
                    355 => {
                        unsafe {
                            sqlite3_parser_add_cleanup(p_parse, Some(agginfo_free),
                                p_agg_info as *mut ())
                        };
                        __state = 356;
                    }
                    356 => { __state = 354; }
                    357 => {
                        unsafe { (*p_agg_info).sel_id = unsafe { (*p).sel_id } };
                        __state = 359;
                    }
                    358 => { __state = 2; }
                    359 => {
                        unsafe {
                            memset(&raw mut s_nc as *mut (), 0,
                                core::mem::size_of::<NameContext>() as u64)
                        };
                        __state = 360;
                    }
                    360 => { s_nc.p_parse = p_parse; __state = 361; }
                    361 => { s_nc.p_src_list = p_tab_list; __state = 362; }
                    362 => { s_nc.u_nc.p_agg_info = p_agg_info; __state = 363; }
                    363 => {
                        unsafe {
                            (*p_agg_info).n_sorting_column =
                                if !(p_group_by).is_null() {
                                        unsafe { (*p_group_by).n_expr }
                                    } else { 0 } as u32
                        };
                        __state = 364;
                    }
                    364 => {
                        unsafe { (*p_agg_info).p_group_by = p_group_by };
                        __state = 365;
                    }
                    365 => {
                        unsafe {
                            sqlite3_expr_analyze_agg_list(&mut s_nc, p_e_list)
                        };
                        __state = 366;
                    }
                    366 => {
                        unsafe {
                            sqlite3_expr_analyze_agg_list(&mut s_nc, s_sort.p_order_by)
                        };
                        __state = 367;
                    }
                    367 => {
                        if !(p_having).is_null() {
                            __state = 369;
                        } else { __state = 368; }
                    }
                    368 => {
                        unsafe {
                            (*p_agg_info).n_accumulator =
                                unsafe { (*p_agg_info).n_column }
                        };
                        __state = 376;
                    }
                    369 => {
                        if !(p_group_by).is_null() {
                            __state = 371;
                        } else { __state = 370; }
                    }
                    370 => {
                        unsafe {
                            sqlite3_expr_analyze_aggregates(&mut s_nc, p_having)
                        };
                        __state = 368;
                    }
                    371 => { { let _ = 0; }; __state = 372; }
                    372 => { { let _ = 0; }; __state = 373; }
                    373 => { { let _ = 0; }; __state = 374; }
                    374 => { having_to_where(p_parse, p); __state = 375; }
                    375 => { p_where = unsafe { (*p).p_where }; __state = 370; }
                    376 => {
                        if unsafe { (*p).p_group_by } == core::ptr::null_mut() &&
                                    unsafe { (*p).p_having } == core::ptr::null_mut() &&
                                unsafe { (*p_agg_info).n_func } == 1 {
                            __state = 378;
                        } else { __state = 379; }
                    }
                    377 => {
                        analyze_agg_func_args(unsafe { &*p_agg_info }, &mut s_nc);
                        __state = 380;
                    }
                    378 => {
                        min_max_flag =
                            min_max_query(db,
                                unsafe {
                                    &*unsafe {
                                                (*unsafe {
                                                            (*p_agg_info).a_func.offset(0 as isize)
                                                        }).p_f_expr
                                            }
                                }, &mut p_min_max_order_by);
                        __state = 377;
                    }
                    379 => { min_max_flag = 0 as u8; __state = 377; }
                    380 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 382;
                        } else { __state = 381; }
                    }
                    381 => {
                        if !(p_group_by).is_null() {
                            __state = 384;
                        } else { __state = 385; }
                    }
                    382 => { __state = 2; }
                    383 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_end) };
                        __state = 280;
                    }
                    384 => { __state = 386; }
                    385 => { __state = 552; }
                    386 => { __state = 387; }
                    387 => { __state = 388; }
                    388 => { __state = 389; }
                    389 => { __state = 390; }
                    390 => { __state = 391; }
                    391 => { __state = 392; }
                    392 => { __state = 393; }
                    393 => { __state = 394; }
                    394 => {
                        p_distinct = core::ptr::null_mut();
                        __state = 395;
                    }
                    395 => { dist_flag = 0 as u16; __state = 396; }
                    396 => { e_dist = 0; __state = 397; }
                    397 => {
                        if unsafe { (*p_agg_info).n_func } == 1 &&
                                            unsafe {
                                                    (*unsafe {
                                                                (*p_agg_info).a_func.offset(0 as isize)
                                                            }).i_distinct
                                                } >= 0 &&
                                        unsafe {
                                                (*unsafe {
                                                            (*p_agg_info).a_func.offset(0 as isize)
                                                        }).p_f_expr
                                            } != core::ptr::null_mut() &&
                                    unsafe {
                                                (*unsafe {
                                                                (*unsafe {
                                                                            (*p_agg_info).a_func.offset(0 as isize)
                                                                        }).p_f_expr
                                                            }).flags
                                            } & 4096 as u32 == 0 as u32 &&
                                unsafe {
                                        (*unsafe {
                                                            (*unsafe {
                                                                        (*p_agg_info).a_func.offset(0 as isize)
                                                                    }).p_f_expr
                                                        }).x.p_list
                                    } != core::ptr::null_mut() {
                            __state = 399;
                        } else { __state = 398; }
                    }
                    398 => {
                        unsafe {
                            (*p_agg_info).sorting_idx =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                }
                        };
                        __state = 404;
                    }
                    399 => {
                        p_expr =
                            unsafe {
                                (*(unsafe {
                                                    (*unsafe {
                                                                        (*unsafe {
                                                                                            (*unsafe {
                                                                                                        (*p_agg_info).a_func.offset(0 as isize)
                                                                                                    }).p_f_expr
                                                                                        }).x.p_list
                                                                    }).a.as_ptr()
                                                } as *mut ExprListItem).offset(0 as isize)).p_expr
                            };
                        __state = 400;
                    }
                    400 => {
                        p_expr =
                            unsafe { sqlite3_expr_dup(db, p_expr as *const Expr, 0) };
                        __state = 401;
                    }
                    401 => {
                        p_distinct =
                            unsafe {
                                sqlite3_expr_list_dup(db, p_group_by as *const ExprList, 0)
                            };
                        __state = 402;
                    }
                    402 => {
                        p_distinct =
                            unsafe {
                                sqlite3_expr_list_append(p_parse, p_distinct, p_expr)
                            };
                        __state = 403;
                    }
                    403 => {
                        dist_flag =
                            if !(p_distinct).is_null() { 256 | 1024 } else { 0 } as u16;
                        __state = 398;
                    }
                    404 => {
                        p_key_info_1 =
                            sqlite3_key_info_from_expr_list(p_parse,
                                unsafe { &*p_group_by }, 0,
                                unsafe { (*p_agg_info).n_column });
                        __state = 405;
                    }
                    405 => {
                        addr_sorting_idx =
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 121,
                                    unsafe { (*p_agg_info).sorting_idx },
                                    unsafe { (*p_agg_info).n_sorting_column } as i32, 0,
                                    p_key_info_1 as *mut i8 as *const i8, -9)
                            };
                        __state = 406;
                    }
                    406 => {
                        i_use_flag =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 407;
                    }
                    407 => {
                        i_abort_flag =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 408;
                    }
                    408 => {
                        reg_output_row =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 409;
                    }
                    409 => {
                        addr_output_row =
                            unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 410;
                    }
                    410 => {
                        reg_reset =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 411;
                    }
                    411 => {
                        addr_reset = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 412;
                    }
                    412 => {
                        i_a_mem = unsafe { (*p_parse).n_mem } + 1;
                        __state = 413;
                    }
                    413 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_group_by).n_expr }
                        };
                        __state = 414;
                    }
                    414 => {
                        i_b_mem = unsafe { (*p_parse).n_mem } + 1;
                        __state = 415;
                    }
                    415 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_group_by).n_expr }
                        };
                        __state = 416;
                    }
                    416 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, i_abort_flag) };
                        __state = 417;
                    }
                    417 => { __state = 418; }
                    418 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 77, 0, i_a_mem,
                                i_a_mem + unsafe { (*p_group_by).n_expr } - 1)
                        };
                        __state = 419;
                    }
                    419 => {
                        unsafe {
                            sqlite3_expr_null_register_range(p_parse, i_a_mem,
                                unsafe { (*p_group_by).n_expr })
                        };
                        __state = 420;
                    }
                    420 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_reset, addr_reset)
                        };
                        __state = 421;
                    }
                    421 => { __state = 422; }
                    422 => {
                        p_w_info =
                            unsafe {
                                sqlite3_where_begin(p_parse, p_tab_list, p_where,
                                    p_group_by, p_distinct, p,
                                    (if s_distinct.is_tnct as i32 == 2 { 128 } else { 64 } |
                                                if order_by_grp != 0 { 512 } else { 0 } | dist_flag as i32)
                                        as u16, 0)
                            };
                        __state = 423;
                    }
                    423 => {
                        if p_w_info == core::ptr::null_mut() {
                            __state = 425;
                        } else { __state = 424; }
                    }
                    424 => {
                        if !(unsafe { (*p_parse).p_idx_epr }).is_null() {
                            __state = 428;
                        } else { __state = 427; }
                    }
                    425 => {
                        unsafe { sqlite3_expr_list_delete(db, p_distinct) };
                        __state = 426;
                    }
                    426 => { __state = 2; }
                    427 => {
                        assign_aggregate_registers(unsafe { &mut *p_parse },
                            unsafe { &mut *p_agg_info });
                        __state = 429;
                    }
                    428 => {
                        optimize_aggregate_use_of_indexed_expr(p_parse as
                                *const Parse, p as *const Select, p_agg_info, &mut s_nc);
                        __state = 427;
                    }
                    429 => {
                        e_dist = unsafe { sqlite3_where_is_distinct(p_w_info) };
                        __state = 430;
                    }
                    430 => { __state = 431; }
                    431 => {
                        if unsafe { sqlite3_where_is_ordered(p_w_info) } ==
                                unsafe { (*p_group_by).n_expr } {
                            __state = 433;
                        } else { __state = 434; }
                    }
                    432 => {
                        if !(unsafe { (*p_parse).p_idx_epr }).is_null() {
                            __state = 481;
                        } else { __state = 480; }
                    }
                    433 => { group_by_sort = 0; __state = 432; }
                    434 => { __state = 435; }
                    435 => { __state = 436; }
                    436 => { __state = 437; }
                    437 => { __state = 438; }
                    438 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 0 as u8,
                                c"USE TEMP B-TREE FOR %s".as_ptr() as *mut i8 as *const i8,
                                if s_distinct.is_tnct != 0 &&
                                        unsafe { (*p).sel_flags } & 1 as u32 == 0 as u32 {
                                    c"DISTINCT".as_ptr() as *mut i8
                                } else { c"GROUP BY".as_ptr() as *mut i8 })
                        };
                        __state = 439;
                    }
                    439 => { group_by_sort = 1; __state = 440; }
                    440 => {
                        n_group_by = unsafe { (*p_group_by).n_expr };
                        __state = 441;
                    }
                    441 => { n_col = n_group_by; __state = 442; }
                    442 => { j = n_group_by; __state = 443; }
                    443 => { i = 0; __state = 445; }
                    444 => {
                        reg_base =
                            unsafe { sqlite3_get_temp_range(p_parse, n_col) };
                        __state = 450;
                    }
                    445 => {
                        if i < unsafe { (*p_agg_info).n_column } {
                            __state = 446;
                        } else { __state = 444; }
                    }
                    446 => {
                        if unsafe {
                                    (*unsafe {
                                                (*p_agg_info).a_col.offset(i as isize)
                                            }).i_sorter_column
                                } >= j {
                            __state = 448;
                        } else { __state = 447; }
                    }
                    447 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 445;
                    }
                    448 => {
                        { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
                        __state = 449;
                    }
                    449 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 447;
                    }
                    450 => {
                        unsafe {
                            sqlite3_expr_code_expr_list(p_parse, p_group_by, reg_base,
                                0, 0 as u8)
                        };
                        __state = 451;
                    }
                    451 => { j = n_group_by; __state = 452; }
                    452 => {
                        unsafe { (*p_agg_info).direct_mode = 1 as u8 };
                        __state = 453;
                    }
                    453 => { i = 0; __state = 455; }
                    454 => {
                        unsafe { (*p_agg_info).direct_mode = 0 as u8 };
                        __state = 461;
                    }
                    455 => {
                        if i < unsafe { (*p_agg_info).n_column } {
                            __state = 456;
                        } else { __state = 454; }
                    }
                    456 => {
                        p_col =
                            unsafe { unsafe { (*p_agg_info).a_col.offset(i as isize) } }
                                as *const AggInfoCol;
                        __state = 458;
                    }
                    457 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 455;
                    }
                    458 => {
                        if unsafe { (*p_col).i_sorter_column } >= j {
                            __state = 459;
                        } else { __state = 457; }
                    }
                    459 => {
                        unsafe {
                            sqlite3_expr_code(p_parse, unsafe { (*p_col).p_c_expr },
                                j + reg_base)
                        };
                        __state = 460;
                    }
                    460 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 457;
                    }
                    461 => {
                        reg_record = unsafe { sqlite3_get_temp_reg(p_parse) };
                        __state = 462;
                    }
                    462 => { __state = 463; }
                    463 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_base, n_col, reg_record)
                        };
                        __state = 464;
                    }
                    464 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 141,
                                unsafe { (*p_agg_info).sorting_idx }, reg_record)
                        };
                        __state = 465;
                    }
                    465 => { __state = 466; }
                    466 => {
                        unsafe { sqlite3_release_temp_reg(p_parse, reg_record) };
                        __state = 467;
                    }
                    467 => {
                        unsafe {
                            sqlite3_release_temp_range(p_parse, reg_base, n_col)
                        };
                        __state = 468;
                    }
                    468 => { __state = 469; }
                    469 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 470;
                    }
                    470 => {
                        unsafe {
                            (*p_agg_info).sorting_idx_p_tab =
                                {
                                    sort_p_tab =
                                        {
                                            let __p = unsafe { &mut (*p_parse).n_tab };
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        };
                                    sort_p_tab
                                }
                        };
                        __state = 471;
                    }
                    471 => {
                        sort_out = unsafe { sqlite3_get_temp_reg(p_parse) };
                        __state = 472;
                    }
                    472 => { __state = 473; }
                    473 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 123, sort_p_tab, sort_out, n_col)
                        };
                        __state = 474;
                    }
                    474 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 34,
                                unsafe { (*p_agg_info).sorting_idx }, addr_end)
                        };
                        __state = 475;
                    }
                    475 => { __state = 476; }
                    476 => { __state = 477; }
                    477 => {
                        unsafe { (*p_agg_info).use_sorting_idx = 1 as u8 };
                        __state = 478;
                    }
                    478 => { __state = 479; }
                    479 => { __state = 432; }
                    480 => {
                        if order_by_grp != 0 &&
                                    unsafe { (*db).db_opt_flags } & 4 as u32 == 0 as u32 &&
                                (group_by_sort != 0 ||
                                    unsafe { sqlite3_where_is_sorted(p_w_info) } != 0) {
                            __state = 483;
                        } else { __state = 482; }
                    }
                    481 => {
                        aggregate_convert_indexed_expr_ref_to_column(unsafe {
                                &*p_agg_info
                            });
                        __state = 480;
                    }
                    482 => {
                        addr_top_of_loop = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 485;
                    }
                    483 => {
                        s_sort.p_order_by = core::ptr::null_mut();
                        __state = 484;
                    }
                    484 => {
                        unsafe {
                            sqlite3_vdbe_change_to_noop(v, s_sort.addr_sort_index)
                        };
                        __state = 482;
                    }
                    485 => {
                        if group_by_sort != 0 {
                            __state = 487;
                        } else { __state = 486; }
                    }
                    486 => { j = 0; __state = 489; }
                    487 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 135,
                                unsafe { (*p_agg_info).sorting_idx }, sort_out, sort_p_tab)
                        };
                        __state = 486;
                    }
                    488 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 92, i_a_mem, i_b_mem,
                                unsafe { (*p_group_by).n_expr },
                                sqlite3_key_info_ref(p_key_info_1) as *mut i8 as *const i8,
                                -9)
                        };
                        __state = 504;
                    }
                    489 => {
                        if j < unsafe { (*p_group_by).n_expr } {
                            __state = 490;
                        } else { __state = 488; }
                    }
                    490 => {
                        i_order_by_col =
                            unsafe {
                                    (*(unsafe { (*p_group_by).a.as_ptr() } as
                                                            *mut ExprListItem).offset(j as isize)).u.x.i_order_by_col
                                } as i32;
                        __state = 492;
                    }
                    491 => {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        __state = 489;
                    }
                    492 => {
                        if group_by_sort != 0 {
                            __state = 494;
                        } else { __state = 495; }
                    }
                    493 => {
                        if i_order_by_col != 0 {
                            __state = 497;
                        } else { __state = 491; }
                    }
                    494 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, sort_p_tab, j, i_b_mem + j)
                        };
                        __state = 493;
                    }
                    495 => {
                        unsafe { (*p_agg_info).direct_mode = 1 as u8 };
                        __state = 496;
                    }
                    496 => {
                        unsafe {
                            sqlite3_expr_code(p_parse,
                                unsafe {
                                    (*(unsafe { (*p_group_by).a.as_ptr() } as
                                                    *mut ExprListItem).offset(j as isize)).p_expr
                                }, i_b_mem + j)
                        };
                        __state = 493;
                    }
                    497 => {
                        p_x =
                            unsafe {
                                (*(unsafe { (*unsafe { (*p).p_e_list }).a.as_ptr() } as
                                                *mut ExprListItem).offset((i_order_by_col - 1) as
                                                isize)).p_expr
                            };
                        __state = 498;
                    }
                    498 => {
                        p_base =
                            unsafe { sqlite3_expr_skip_collate_and_likely(p_x) } as
                                *const Expr;
                        __state = 499;
                    }
                    499 => {
                        if p_base != core::ptr::null_mut() &&
                                unsafe { (*p_base).op } as i32 == 179 {
                            __state = 501;
                        } else { __state = 500; }
                    }
                    500 => {
                        if p_base != core::ptr::null_mut() &&
                                    unsafe { (*p_base).op } as i32 != 170 &&
                                unsafe { (*p_base).op } as i32 != 176 {
                            __state = 503;
                        } else { __state = 491; }
                    }
                    501 => { p_x = unsafe { (*p_base).p_left }; __state = 502; }
                    502 => {
                        p_base =
                            unsafe { sqlite3_expr_skip_collate_and_likely(p_x) };
                        __state = 499;
                    }
                    503 => {
                        unsafe { sqlite3_expr_to_register(p_x, i_a_mem + j) };
                        __state = 491;
                    }
                    504 => {
                        addr1 = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 505;
                    }
                    505 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 14, addr1 + 1, 0, addr1 + 1)
                        };
                        __state = 506;
                    }
                    506 => { __state = 507; }
                    507 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_output_row, addr_output_row)
                        };
                        __state = 508;
                    }
                    508 => { __state = 509; }
                    509 => {
                        unsafe {
                            sqlite3_expr_code_move(p_parse, i_b_mem, i_a_mem,
                                unsafe { (*p_group_by).n_expr })
                        };
                        __state = 510;
                    }
                    510 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 61, i_abort_flag, addr_end)
                        };
                        __state = 511;
                    }
                    511 => { __state = 512; }
                    512 => { __state = 513; }
                    513 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_reset, addr_reset)
                        };
                        __state = 514;
                    }
                    514 => { __state = 515; }
                    515 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
                        __state = 516;
                    }
                    516 => {
                        update_accumulator(p_parse, i_use_flag,
                            unsafe { &mut *p_agg_info }, e_dist);
                        __state = 517;
                    }
                    517 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_use_flag) };
                        __state = 518;
                    }
                    518 => { __state = 519; }
                    519 => {
                        if group_by_sort != 0 {
                            __state = 521;
                        } else { __state = 522; }
                    }
                    520 => {
                        unsafe { sqlite3_expr_list_delete(db, p_distinct) };
                        __state = 526;
                    }
                    521 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 38,
                                unsafe { (*p_agg_info).sorting_idx }, addr_top_of_loop)
                        };
                        __state = 523;
                    }
                    522 => { __state = 524; }
                    523 => { __state = 520; }
                    524 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 525;
                    }
                    525 => {
                        unsafe { sqlite3_vdbe_change_to_noop(v, addr_sorting_idx) };
                        __state = 520;
                    }
                    526 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 10, reg_output_row, addr_output_row)
                        };
                        __state = 527;
                    }
                    527 => { __state = 528; }
                    528 => {
                        unsafe { sqlite3_vdbe_goto(v, addr_end) };
                        __state = 529;
                    }
                    529 => {
                        addr_set_abort = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 530;
                    }
                    530 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, i_abort_flag) };
                        __state = 531;
                    }
                    531 => { __state = 532; }
                    532 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_output_row) };
                        __state = 533;
                    }
                    533 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_output_row) };
                        __state = 534;
                    }
                    534 => {
                        addr_output_row = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 535;
                    }
                    535 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 61, i_use_flag, addr_output_row + 2)
                        };
                        __state = 536;
                    }
                    536 => { __state = 537; }
                    537 => { __state = 538; }
                    538 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_output_row) };
                        __state = 539;
                    }
                    539 => {
                        finalize_agg_functions(p_parse, unsafe { &*p_agg_info });
                        __state = 540;
                    }
                    540 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, p_having,
                                addr_output_row + 1, 16)
                        };
                        __state = 541;
                    }
                    541 => {
                        select_inner_loop(p_parse, p, -1, &mut s_sort,
                            &raw mut s_distinct as *const DistinctCtx,
                            unsafe { &mut *p_dest }, addr_output_row + 1,
                            addr_set_abort);
                        __state = 542;
                    }
                    542 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_output_row) };
                        __state = 543;
                    }
                    543 => { __state = 544; }
                    544 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_reset) };
                        __state = 545;
                    }
                    545 => {
                        reset_accumulator(p_parse, unsafe { &*p_agg_info });
                        __state = 546;
                    }
                    546 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, i_use_flag) };
                        __state = 547;
                    }
                    547 => { __state = 548; }
                    548 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 69, reg_reset) };
                        __state = 549;
                    }
                    549 => {
                        if dist_flag as i32 != 0 && e_dist != 0 {
                            __state = 550;
                        } else { __state = 383; }
                    }
                    550 => {
                        p_f =
                            unsafe {
                                    unsafe { (*p_agg_info).a_func.offset(0 as isize) }
                                } as *const AggInfoFunc;
                        __state = 551;
                    }
                    551 => {
                        fix_distinct_open_eph(unsafe { &*p_parse }, e_dist,
                            unsafe { (*p_f).i_distinct },
                            unsafe { (*p_f).i_dist_addr });
                        __state = 383;
                    }
                    552 => {
                        if {
                                    p_tab_1 = is_simple_count(unsafe { &*p }, p_agg_info);
                                    p_tab_1
                                } != core::ptr::null_mut() {
                            __state = 554;
                        } else { __state = 555; }
                    }
                    553 => {
                        s_sort.p_order_by = core::ptr::null_mut();
                        __state = 622;
                    }
                    554 => {
                        i_db_1 =
                            unsafe {
                                    sqlite3_schema_to_index(unsafe { (*p_parse).db },
                                        unsafe { (*p_tab_1).p_schema })
                                } as i32;
                        __state = 556;
                    }
                    555 => { reg_acc = 0; __state = 581; }
                    556 => {
                        i_csr =
                            {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                } as i32;
                        __state = 557;
                    }
                    557 => { __state = 558; }
                    558 => {
                        p_key_info_2 = core::ptr::null_mut();
                        __state = 559;
                    }
                    559 => { p_best = core::ptr::null_mut(); __state = 560; }
                    560 => {
                        i_root = unsafe { (*p_tab_1).tnum };
                        __state = 561;
                    }
                    561 => {
                        unsafe { sqlite3_code_verify_schema(p_parse, i_db_1) };
                        __state = 562;
                    }
                    562 => {
                        unsafe {
                            sqlite3_table_lock(p_parse, i_db_1,
                                unsafe { (*p_tab_1).tnum }, 0 as u8,
                                unsafe { (*p_tab_1).z_name } as *const i8)
                        };
                        __state = 563;
                    }
                    563 => {
                        if !(unsafe { (*p_tab_1).tab_flags } & 128 as u32 ==
                                            0 as u32) as i32 != 0 {
                            __state = 565;
                        } else { __state = 564; }
                    }
                    564 => {
                        if (unsafe {
                                            (*(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).fg.not_indexed()
                                        } == 0) as i32 != 0 {
                            __state = 567;
                        } else { __state = 566; }
                    }
                    565 => {
                        p_best = unsafe { sqlite3_primary_key_index(p_tab_1) };
                        __state = 564;
                    }
                    566 => {
                        if !(p_best).is_null() {
                            __state = 573;
                        } else { __state = 572; }
                    }
                    567 => {
                        p_idx = unsafe { (*p_tab_1).p_index };
                        __state = 568;
                    }
                    568 => {
                        if !(p_idx).is_null() {
                            __state = 569;
                        } else { __state = 566; }
                    }
                    569 => {
                        if unsafe { (*p_idx).b_unordered() } as i32 == 0 &&
                                        (unsafe { (*p_idx).sz_idx_row } as i32) <
                                            unsafe { (*p_tab_1).sz_tab_row } as i32 &&
                                    unsafe { (*p_idx).p_part_idx_where } ==
                                        core::ptr::null_mut() &&
                                ((p_best).is_null() as i32 != 0 ||
                                    (unsafe { (*p_idx).sz_idx_row } as i32) <
                                        unsafe { (*p_best).sz_idx_row } as i32) {
                            __state = 571;
                        } else { __state = 570; }
                    }
                    570 => {
                        p_idx = unsafe { (*p_idx).p_next };
                        __state = 568;
                    }
                    571 => { p_best = p_idx; __state = 570; }
                    572 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 114, i_csr, i_root as i32,
                                i_db_1, 1)
                        };
                        __state = 575;
                    }
                    573 => {
                        i_root = unsafe { (*p_best).tnum };
                        __state = 574;
                    }
                    574 => {
                        p_key_info_2 =
                            unsafe { sqlite3_key_info_of_index(p_parse, p_best) };
                        __state = 572;
                    }
                    575 => {
                        if !(p_key_info_2).is_null() {
                            __state = 577;
                        } else { __state = 576; }
                    }
                    576 => {
                        assign_aggregate_registers(unsafe { &mut *p_parse },
                            unsafe { &mut *p_agg_info });
                        __state = 578;
                    }
                    577 => {
                        unsafe {
                            sqlite3_vdbe_change_p4(v, -1,
                                p_key_info_2 as *mut i8 as *const i8, -9)
                        };
                        __state = 576;
                    }
                    578 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 100, i_csr,
                                unsafe { (*p_agg_info).i_first_reg } +
                                        unsafe { (*p_agg_info).n_column } + 0)
                        };
                        __state = 579;
                    }
                    579 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 124, i_csr) };
                        __state = 580;
                    }
                    580 => {
                        explain_simple_count(p_parse, unsafe { &*p_tab_1 },
                            p_best as *const Index);
                        __state = 553;
                    }
                    581 => {
                        p_distinct_1 = core::ptr::null_mut();
                        __state = 582;
                    }
                    582 => { dist_flag_1 = 0 as u16; __state = 583; }
                    583 => { __state = 584; }
                    584 => {
                        if unsafe { (*p_agg_info).n_accumulator } != 0 {
                            __state = 586;
                        } else { __state = 587; }
                    }
                    585 => {
                        assign_aggregate_registers(unsafe { &mut *p_parse },
                            unsafe { &mut *p_agg_info });
                        __state = 600;
                    }
                    586 => { i = 0; __state = 589; }
                    587 => {
                        if unsafe { (*p_agg_info).n_func } == 1 &&
                                unsafe {
                                        (*unsafe {
                                                    (*p_agg_info).a_func.offset(0 as isize)
                                                }).i_distinct
                                    } >= 0 {
                            __state = 597;
                        } else { __state = 585; }
                    }
                    588 => {
                        if i == unsafe { (*p_agg_info).n_func } {
                            __state = 595;
                        } else { __state = 585; }
                    }
                    589 => {
                        if i < unsafe { (*p_agg_info).n_func } {
                            __state = 590;
                        } else { __state = 588; }
                    }
                    590 => {
                        if unsafe {
                                        (*unsafe {
                                                        (*unsafe {
                                                                    (*p_agg_info).a_func.offset(i as isize)
                                                                }).p_f_expr
                                                    }).flags
                                    } & 16777216 as u32 != 0 as u32 {
                            __state = 593;
                        } else { __state = 592; }
                    }
                    591 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 589;
                    }
                    592 => {
                        if unsafe {
                                        (*unsafe {
                                                        (*unsafe { (*p_agg_info).a_func.offset(i as isize) }).p_func
                                                    }).func_flags
                                    } & 32 as u32 != 0 {
                            __state = 594;
                        } else { __state = 591; }
                    }
                    593 => { __state = 591; }
                    594 => { __state = 588; }
                    595 => {
                        reg_acc =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 596;
                    }
                    596 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_acc) };
                        __state = 585;
                    }
                    597 => { { let _ = 0; }; __state = 598; }
                    598 => {
                        p_distinct_1 =
                            unsafe {
                                (*unsafe {
                                                    (*unsafe {
                                                                (*p_agg_info).a_func.offset(0 as isize)
                                                            }).p_f_expr
                                                }).x.p_list
                            };
                        __state = 599;
                    }
                    599 => {
                        dist_flag_1 =
                            if !(p_distinct_1).is_null() { 256 | 1024 } else { 0 } as
                                u16;
                        __state = 585;
                    }
                    600 => { { let _ = 0; }; __state = 601; }
                    601 => {
                        reset_accumulator(p_parse, unsafe { &*p_agg_info });
                        __state = 602;
                    }
                    602 => { { let _ = 0; }; __state = 603; }
                    603 => { { let _ = 0; }; __state = 604; }
                    604 => { __state = 605; }
                    605 => {
                        p_w_info =
                            unsafe {
                                sqlite3_where_begin(p_parse, p_tab_list, p_where,
                                    p_min_max_order_by, p_distinct_1, p,
                                    (min_max_flag as i32 | dist_flag_1 as i32) as u16, 0)
                            };
                        __state = 606;
                    }
                    606 => {
                        if p_w_info == core::ptr::null_mut() {
                            __state = 608;
                        } else { __state = 607; }
                    }
                    607 => { __state = 609; }
                    608 => { __state = 2; }
                    609 => {
                        e_dist_1 = unsafe { sqlite3_where_is_distinct(p_w_info) };
                        __state = 610;
                    }
                    610 => {
                        update_accumulator(p_parse, reg_acc,
                            unsafe { &mut *p_agg_info }, e_dist_1);
                        __state = 611;
                    }
                    611 => {
                        if e_dist_1 != 0 { __state = 613; } else { __state = 612; }
                    }
                    612 => {
                        if reg_acc != 0 { __state = 617; } else { __state = 616; }
                    }
                    613 => {
                        p_f_1 =
                            unsafe { (*p_agg_info).a_func } as *const AggInfoFunc;
                        __state = 614;
                    }
                    614 => {
                        if !(p_f_1).is_null() {
                            __state = 615;
                        } else { __state = 612; }
                    }
                    615 => {
                        fix_distinct_open_eph(unsafe { &*p_parse }, e_dist_1,
                            unsafe { (*p_f_1).i_distinct },
                            unsafe { (*p_f_1).i_dist_addr });
                        __state = 612;
                    }
                    616 => {
                        if min_max_flag != 0 {
                            __state = 619;
                        } else { __state = 618; }
                    }
                    617 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 1, reg_acc) };
                        __state = 616;
                    }
                    618 => { __state = 620; }
                    619 => {
                        unsafe { sqlite3_where_min_max_opt_early_out(v, p_w_info) };
                        __state = 618;
                    }
                    620 => {
                        unsafe { sqlite3_where_end(p_w_info) };
                        __state = 621;
                    }
                    621 => {
                        finalize_agg_functions(p_parse, unsafe { &*p_agg_info });
                        __state = 553;
                    }
                    622 => {
                        unsafe {
                            sqlite3_expr_if_false(p_parse, p_having, addr_end, 16)
                        };
                        __state = 623;
                    }
                    623 => {
                        select_inner_loop(p_parse, p, -1, core::ptr::null_mut(),
                            core::ptr::null(), unsafe { &mut *p_dest }, addr_end,
                            addr_end);
                        __state = 383;
                    }
                    624 => {
                        if !(s_sort.p_order_by).is_null() {
                            __state = 627;
                        } else { __state = 626; }
                    }
                    625 => {
                        explain_temp_table(p_parse,
                            c"DISTINCT".as_ptr() as *mut i8 as *const i8);
                        __state = 624;
                    }
                    626 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, i_end) };
                        __state = 629;
                    }
                    627 => { { let _ = 0; }; __state = 628; }
                    628 => {
                        generate_sort_tail(p_parse, unsafe { &*p }, &s_sort,
                            unsafe { (*p_e_list).n_expr }, unsafe { &*p_dest });
                        __state = 626;
                    }
                    629 => {
                        rc = (unsafe { (*p_parse).n_err } > 0) as i32;
                        __state = 630;
                    }
                    630 => { __state = 2; }
                    631 => { { let _ = 0; }; __state = 632; }
                    632 => {
                        unsafe { sqlite3_expr_list_delete(db, p_min_max_order_by) };
                        __state = 633;
                    }
                    633 => {
                        unsafe { sqlite3_vdbe_explain_pop(p_parse) };
                        __state = 634;
                    }
                    634 => { return rc; }
                    _ => {}
                }
            }
        }

        /// Loop counters
        /// Return from sqlite3WhereBegin()
        /// The virtual machine under construction
        /// True for select lists like "count(*)"
        /// List of columns to extract.
        /// List of tables to select from
        /// The WHERE clause.  May be NULL
        /// The GROUP BY clause.  May be NULL
        /// The HAVING clause.  May be NULL
        /// Aggregate information
        /// Value to return from this function
        /// Info on how to code the DISTINCT keyword
        /// Info on how to code the ORDER BY clause
        /// Address of the end of the query
        /// The database connection
        /// Added ORDER BY for min/max queries
        /// Flag for min/max queries
        /// tag-select-0100
        /// All of these destinations are also able to ignore the ORDER BY clause
        /// If the SF_UFSrcCheck flag is set, then this function is being called
        ///* as part of populating the temp table for an UPDATE...FROM statement.
        ///* In this case, it is an error if the target object (pSrc->a[0]) name
        ///* or alias is duplicated within FROM clause (pSrc->a[1..n]). 
        ///*
        ///* Postgres disallows this case too. The reason is that some other
        ///* systems handle this case differently, and not all the same way,
        ///* which is just confusing. To avoid this, we follow PG's lead and
        ///* disallow it altogether.
        /// Clear the SF_UFSrcCheck flag. The check has already been performed,
        ///* and leaving this flag set can cause errors if a compound sub-query
        ///* in p->pSrc is flattened into this query and this function called
        ///* again as part of compound SELECT processing.
        /// SQLITE_OMIT_WINDOWFUNC
        /// Try to do various optimizations (flattening subqueries, and strength
        ///* reduction of join operators) in the FROM clause up into the main query
        ///* tag-select-0200
        /// The expander should have already created transient Table objects
        ///* even for FROM clause elements such as subqueries that do not correspond
        ///* to a real table
        /// Try to simplify joins:
        ///*
        ///*      LEFT JOIN  ->  JOIN
        ///*     RIGHT JOIN  ->  JOIN
        ///*      FULL JOIN  ->  RIGHT JOIN
        ///*
        ///* If terms of the i-th table are used in the WHERE clause in such a
        ///* way that the i-th table cannot be the NULL row of a join, then
        ///* perform the appropriate simplification. This is called
        ///* "OUTER JOIN strength reduction" in the SQLite documentation.
        ///* tag-select-0220
        /// No further action if this term of the FROM clause is not a subquery
        /// Catch mismatch in the declared columns of a view and the number of
        ///* columns in the SELECT on the RHS
        /// Do not attempt the usual optimizations (flattening and ORDER BY
        ///* elimination) on a MATERIALIZED common table expression because
        ///* a MATERIALIZED common table expression is an optimization fence.
        /// Do not try to flatten an aggregate subquery.
        ///*
        ///* Flattening an aggregate subquery is only possible if the outer query
        ///* is not a join.  But if the outer query is not a join, then the subquery
        ///* will be implemented as a co-routine and there is no advantage to
        ///* flattening in that case.
        /// tag-select-0230:
        ///* If a FROM-clause subquery has an ORDER BY clause that is not
        ///* really doing anything, then delete it now so that it does not
        ///* interfere with query flattening.  See the discussion at
        ///* https://sqlite.org/forum/forumpost/2d76f2bcf65d256a
        ///*
        ///* Beware of these cases where the ORDER BY clause may not be safely
        ///* omitted:
        ///*
        ///*    (1)   There is also a LIMIT clause
        ///*    (2)   The subquery was added to help with window-function
        ///*          processing
        ///*    (3)   The subquery is in the FROM clause of an UPDATE
        ///*    (4)   The outer query uses an aggregate function other than
        ///*          the built-in count(), min(), or max().
        ///*    (5)   The ORDER BY isn't going to accomplish anything because
        ///*          one of:
        ///*            (a)  The outer query has a different ORDER BY clause
        ///*            (b)  The subquery is part of a join
        ///*          See forum post 062d576715d277c8
        ///*    (6)   The subquery is not a recursive CTE.  ORDER BY has a different
        ///*          meaning for recursive CTEs and this optimization does not
        ///*          apply.
        ///*
        ///* Also retain the ORDER BY if the OmitOrderBy optimization is disabled.
        /// Condition (5)
        /// Condition (1)
        /// (2) and (6)
        /// Condition (3) and (4)
        /// If the outer query contains a "complex" result set (that is,
        ///* if the result set of the outer query uses functions or subqueries)
        ///* and if the subquery contains an ORDER BY clause and if
        ///* it will be implemented as a co-routine, then do not flatten.  This
        ///* restriction allows SQL constructs like this:
        ///*
        ///*  SELECT expensive_function(x)
        ///*    FROM (SELECT x FROM tab ORDER BY y LIMIT 10);
        ///*
        ///* The expensive_function() is only computed on the 10 rows that
        ///* are output, rather than every row of the table.
        ///*
        ///* The requirement that the outer query have a complex result set
        ///* means that flattening does occur on simpler SQL constraints without
        ///* the expensive_function() like:
        ///*
        ///*  SELECT x FROM (SELECT x FROM tab ORDER BY y LIMIT 10);
        /// tag-select-0240
        /// This subquery can be absorbed into its parent.
        /// Handle compound SELECT statements using the separate multiSelect()
        ///* procedure.  tag-select-0300
        /// If there may be an "EXISTS (SELECT ...)" in the WHERE clause, attempt
        ///* to change it into a join.
        /// Do the WHERE-clause constant propagation optimization if this is
        ///* a join.  No need to spend time on this operation for non-join queries
        ///* as the equivalent optimization will be handled by query planner in
        ///* sqlite3WhereBegin().  tag-select-0330
        /// tag-select-0350
        /// Loop over all terms in the FROM clause and do two things for each term:
        ///*
        ///*   (1) Authorize unreferenced tables
        ///*   (2) Generate code for all sub-queries
        ///*
        ///* tag-select-0400
        /// Authorized unreferenced tables.  tag-select-0410
        ///*
        ///* Issue SQLITE_READ authorizations with a fake column name for any
        ///* tables that are referenced but from which no values are extracted.
        ///* Examples of where these kinds of null SQLITE_READ authorizations
        ///* would occur:
        ///*
        ///*     SELECT count(*) FROM t1;   -- SQLITE_READ t1.""
        ///*     SELECT t1.* FROM t1, t2;   -- SQLITE_READ t2.""
        ///*
        ///* The fake column name is an empty string.  It is possible for a table to
        ///* have a column named by the empty string, in which case there is no way to
        ///* distinguish between an unreferenced table and an actual reference to the
        ///* "" column. The original design was for the fake column name to be a NULL,
        ///* which would be unambiguous.  But legacy authorization callbacks might
        ///* assume the column name is non-NULL and segfault.  The use of an empty
        ///* string for the fake column name seems safer.
        /// Generate code for all sub-queries in the FROM clause
        /// The code for a subquery should only be generated once.
        /// Increment Parse.nHeight by the height of the largest expression
        ///* tree referred to by this, the parent select. The child select
        ///* may contain expression trees of at most
        ///* (SQLITE_MAX_EXPR_DEPTH-Parse.nHeight) height. This is a bit
        ///* more conservative than necessary, but much easier than enforcing
        ///* an exact limit.
        /// Make copies of constant WHERE-clause terms in the outer query down
        ///* inside the subquery.  This can help the subquery to run more efficiently.
        ///* This is the "predicate push-down optimization".  tag-select-0420
        /// Convert unused result columns of the subquery into simple NULL
        ///* expressions, to avoid unneeded searching and computation.
        ///* tag-select-0440
        /// Generate byte-code to implement the subquery  tag-select-0480
        /// Implement a co-routine that will return a single row of the result
        ///* set on each invocation.  tag-select-0482
        /// This is a CTE for which materialization code has already been
        ///* generated.  Invoke the subroutine to compute the materialization,
        ///* then make the pItem->iCursor be a copy of the ephemeral table that
        ///* holds the result of the materialization. tag-select-0484
        /// This view has already been materialized by a prior entry in
        ///* this same FROM clause.  Reuse it.  tag-select-0486
        /// Materialize the view.  If the view is not correlated, generate a
        ///* subroutine to do the materialization so that subsequent uses of
        ///* the same view can reuse the materialization.  tag-select-0488
        /// If the subquery is not correlated and if we are not inside of
        ///* a trigger, then we only need to compute the value of the subquery
        ///* once.
        /// Various elements of the SELECT copied into local variables for
        ///* convenience
        /// tag-select-0500
        ///*
        ///* If the query is DISTINCT with an ORDER BY but is not an aggregate, and
        ///* if the select-list is the same as the ORDER BY list, then this query
        ///* can be rewritten as a GROUP BY. In other words, this:
        ///*
        ///*     SELECT DISTINCT xyz FROM ... ORDER BY xyz
        ///*
        ///* is transformed to:
        ///*
        ///*     SELECT xyz FROM ... GROUP BY xyz ORDER BY xyz
        ///*
        ///* The second form is preferred as a single index (or temp-table) may be
        ///* used for both the ORDER BY and DISTINCT processing. As originally
        ///* written the query must use a temp-table for at least one of the ORDER
        ///* BY and DISTINCT, and an index or separate temp-table for the other.
        /// Notice that even thought SF_Distinct has been cleared from p->selFlags,
        ///* the sDistinct.isTnct is still set.  Hence, isTnct represents the
        ///* original setting of the SF_Distinct flag, not the current setting
        /// If there is an ORDER BY clause, then create an ephemeral index to
        ///* do the sorting.  But this sorting ephemeral index might end up
        ///* being unused if the data can be extracted in pre-sorted order.
        ///* If that is the case, then the OP_OpenEphemeral instruction will be
        ///* changed to an OP_Noop once we figure out that the sorting index is
        ///* not needed.  The sSort.addrSortIndex variable is used to facilitate
        ///* that change.  tag-select-0600
        /// If the output is destined for a temporary table, open that table.
        ///* tag-select-0630
        /// Delete or NULL-out result columns that will never be used
        /// Set the limiter.  tag-select-0650
        /// 4 billion rows
        /// Open an ephemeral index to use for the distinct set. tag-select-0680
        /// No aggregate functions and no GROUP BY clause.  tag-select-0700
        /// Main window object (or NULL)
        /// Begin the database scan.
        /// TUNING: For a UNION CTE, because UNION is implies DISTINCT,
        ///* reduce the estimated output row count by 8 (LogEst 30). 
        ///* Search for tag-20250414a to see other cases
        /// If sorting index that was created by a prior OP_OpenEphemeral
        ///* instruction ended up not being needed, then change the OP_OpenEphemeral
        ///* into an OP_Noop.
        /// SQLITE_OMIT_WINDOWFUNC
        /// Use the standard inner loop.
        /// End the database scan loop.
        /// This case is for when there exist aggregate functions or a GROUP BY
        ///* clause or both.  tag-select-0800
        /// Name context for processing aggregate information
        /// First Mem address for storing current GROUP BY
        /// First Mem address for previous GROUP BY
        /// Mem address holding flag indicating that at least
        ///* one row of the input to the aggregator has been
        ///* processed
        /// Mem address which causes query abort if positive
        /// Rows come from source in GROUP BY order
        /// End of processing for this SELECT
        /// Pseudotable used to decode sorting results
        /// Output register from the sorter
        /// True if the GROUP BY and ORDER BY are the same
        /// Remove any and all aliases between the result set and the
        ///* GROUP BY clause.
        /// Loop counter
        /// For looping over expression in a list
        /// If there is both a GROUP BY and an ORDER BY clause and they are
        ///* identical, then it may be possible to disable the ORDER BY clause
        ///* on the grounds that the GROUP BY will cause elements to come out
        ///* in the correct order. It also may not - the GROUP BY might use a
        ///* database index that causes rows to be grouped together as required
        ///* but not actually sorted. Either way, record the fact that the
        ///* ORDER BY and GROUP BY clauses are the same by setting the orderByGrp
        ///* variable.
        /// Create a label to jump to when we want to abort the query
        /// Convert TK_COLUMN nodes into TK_AGG_COLUMN and make entries in
        ///* sAggInfo for all TK_AGG_FUNCTION nodes in expressions of the
        ///* SELECT statement.
        /// Processing for aggregates with GROUP BY is very different and
        ///* much more complex than aggregates without a GROUP BY.  tag-select-0810
        /// Keying information for the group by clause
        /// A-vs-B comparison jump
        /// Start of subroutine that outputs a result row
        /// Return address register for output subroutine
        /// Set the abort flag and return
        /// Top of the input loop
        /// The OP_OpenEphemeral for the sorting index
        /// Subroutine for resetting the accumulator
        /// Return address register for reset subroutine
        /// If there is a GROUP BY clause we might need a sorting index to
        ///* implement it.  Allocate that sorting index now.  If it turns out
        ///* that we do not need it after all, the OP_SorterOpen instruction
        ///* will be converted into a Noop.
        /// Initialize memory locations used by GROUP BY aggregate processing
        /// Begin a loop that will extract all source rows in GROUP BY order.
        ///* This might involve two separate loops with an OP_Sort in between, or
        ///* it might be a single loop that uses an index to extract information
        ///* in the right order to begin with.
        /// The optimizer is able to deliver rows in group by order so
        ///* we do not have to sort.  The OP_OpenEphemeral table will be
        ///* cancelled later because we still need to use the pKeyInfo
        /// Rows are coming out in undetermined order.  We have to push
        ///* each row into a sorting index, terminate the first loop,
        ///* then loop over the sorting index in order to get the output
        ///* in sorted order
        /// If there are entries in pAgggInfo->aFunc[] that contain subexpressions
        ///* that are indexed (and that were previously identified and tagged
        ///* in optimizeAggregateUseOfIndexedExpr()) then those subexpressions
        ///* must now be converted into a TK_AGG_COLUMN node so that the value
        ///* is correctly pulled from the index rather than being recomputed.
        /// If the index or temporary table used by the GROUP BY sort
        ///* will naturally deliver rows in the order required by the ORDER BY
        ///* clause, cancel the ephemeral table open coded earlier.
        ///*
        ///* This is an optimization - the correct answer should result regardless.
        ///* Use the SQLITE_GroupByOrder flag with SQLITE_TESTCTRL_OPTIMIZER to
        ///* disable this optimization for testing purposes.
        /// Evaluate the current GROUP BY terms and store in b0, b1, b2...
        ///* (b0 is memory location iBMem+0, b1 is iBMem+1, and so forth)
        ///* Then compare the current GROUP BY terms against the GROUP BY terms
        ///* from the previous row currently stored in a0, a1, a2...
        /// Generate code that runs whenever the GROUP BY changes.
        ///* Changes in the GROUP BY are detected by the previous code
        ///* block.  If there were no changes, this block is skipped.
        ///*
        ///* This code copies current group by terms in b0,b1,b2,...
        ///* over to a0,a1,a2.  It then calls the output subroutine
        ///* and resets the aggregate accumulator registers in preparation
        ///* for the next GROUP BY batch.
        /// Update the aggregate accumulators based on the content of
        ///* the current row
        /// End of the loop
        /// Output the final row of result
        /// Jump over the subroutines
        /// Generate a subroutine that outputs a single row of the result
        ///* set.  This subroutine first looks at the iUseFlag.  If iUseFlag
        ///* is less than or equal to zero, the subroutine is a no-op.  If
        ///* the processing calls for the query to abort, this subroutine
        ///* increments the iAbortFlag memory location before returning in
        ///* order to signal the caller to abort.
        /// Generate a subroutine that will reset the group-by accumulator
        /// endif pGroupBy.  Begin aggregate queries without GROUP BY:
        /// Aggregate functions without GROUP BY. tag-select-0820
        /// tag-select-0821
        ///*
        ///* If isSimpleCount() returns a pointer to a Table structure, then
        ///* the SQL statement is of the form:
        ///*
        ///*   SELECT count(*) FROM <tbl>
        ///*
        ///* where the Table structure returned represents table <tbl>.
        ///*
        ///* This statement is so common that it is optimized specially. The
        ///* OP_Count instruction is executed either on the intkey table that
        ///* contains the data for table <tbl> or on one of its indexes. It
        ///* is better to execute the op on an index, as indexes are almost
        ///* always spread across less pages than their corresponding tables.
        /// Cursor to scan b-tree
        /// Iterator variable
        /// Keyinfo for scanned index
        /// Best index found so far
        /// Root page of scanned b-tree
        /// Search for the index that has the lowest scan cost.
        ///*
        ///* (2011-04-15) Do not do a full scan of an unordered index.
        ///*
        ///* (2013-10-03) Do not count the entries in a partial index.
        ///*
        ///* In practice the KeyInfo structure will not be used. It is only
        ///* passed to keep OP_OpenRead happy.
        /// Open a read-only cursor, execute the OP_Count, close the cursor.
        /// The general case of an aggregate query without GROUP BY
        ///* tag-select-0822
        /// "populate accumulators" flag
        /// If there are accumulator registers but no min() or max() functions
        ///* without FILTER clauses, allocate register regAcc. Register regAcc
        ///* will contain 0 the first time the inner loop runs, and 1 thereafter.
        ///* The code generated by updateAccumulator() uses this to ensure
        ///* that the accumulator registers are (a) updated only once if
        ///* there are no min() or max functions or (b) always updated for the
        ///* first row visited by the aggregate, so that they are updated at
        ///* least once even if the FILTER clause means the min() or max()
        ///* function visits zero rows.
        /// This case runs if the aggregate has no GROUP BY clause.  The
        ///* processing is much simpler since there is only a single row
        ///* of output.
        /// If this query is a candidate for the min/max optimization, then
        ///* minMaxFlag will have been previously set to either
        ///* WHERE_ORDERBY_MIN or WHERE_ORDERBY_MAX and pMinMaxOrderBy will
        ///* be an appropriate ORDER BY expression for the optimization.
        /// endif aggregate query
        /// If there is an ORDER BY clause, then we need to sort the results
        ///* and send them to the callback one by one.  tag-select-0900
        /// Jump here to skip this query
        /// The SELECT has been coded. If there is an error in the Parse structure,
        ///* set the return code to 1. Otherwise 0.
        /// Control jumps to here if an error is encountered above, or upon
        ///* successful coding of the SELECT.
        unreachable!();
    }
}

///* Allocate a new Select structure and return a pointer to that
///* structure.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_new(p_parse: *mut Parse,
    mut p_e_list: *mut ExprList, mut p_src: *mut SrcList, p_where: *mut Expr,
    p_group_by: *mut ExprList, p_having: *mut Expr, p_order_by: *mut ExprList,
    sel_flags: u32, p_limit: *mut Expr) -> *mut Select {
    unsafe {
        let mut p_new: *mut Select = core::ptr::null_mut();
        let mut p_allocated: *mut Select = core::ptr::null_mut();
        let mut standin: Select = unsafe { core::mem::zeroed() };
        p_allocated =
            {
                p_new =
                    unsafe {
                            sqlite3_db_malloc_raw_nn(unsafe { (*p_parse).db },
                                core::mem::size_of::<Select>() as u64)
                        } as *mut Select;
                p_new
            };
        if p_new == core::ptr::null_mut() {
            { let _ = 0; };
            p_new = &mut standin;
        }
        if p_e_list == core::ptr::null_mut() {
            p_e_list =
                unsafe {
                    sqlite3_expr_list_append(p_parse, core::ptr::null_mut(),
                        unsafe {
                            sqlite3_expr(unsafe { (*p_parse).db }, 180,
                                core::ptr::null())
                        })
                };
        }
        unsafe { (*p_new).p_e_list = p_e_list };
        unsafe { (*p_new).op = 139 as u8 };
        unsafe { (*p_new).sel_flags = sel_flags };
        unsafe { (*p_new).i_limit = 0 };
        unsafe { (*p_new).i_offset = 0 };
        unsafe {
            (*p_new).sel_id =
                {
                        let __p = unsafe { &mut (*p_parse).n_select };
                        *__p += 1;
                        *__p
                    } as u32
        };
        unsafe { (*p_new).n_select_row = 0 as LogEst };
        if p_src == core::ptr::null_mut() {
            p_src =
                unsafe {
                        sqlite3_db_malloc_zero(unsafe { (*p_parse).db },
                            core::mem::offset_of!(SrcList, a) as u64 +
                                core::mem::size_of::<SrcItem>() as u64)
                    } as *mut SrcList;
        }
        unsafe { (*p_new).p_src = p_src };
        unsafe { (*p_new).p_where = p_where };
        unsafe { (*p_new).p_group_by = p_group_by };
        unsafe { (*p_new).p_having = p_having };
        unsafe { (*p_new).p_order_by = p_order_by };
        unsafe { (*p_new).p_prior = core::ptr::null_mut() };
        unsafe { (*p_new).p_next = core::ptr::null_mut() };
        unsafe { (*p_new).p_limit = p_limit };
        unsafe { (*p_new).p_with = core::ptr::null_mut() };
        unsafe { (*p_new).p_win = core::ptr::null_mut() };
        unsafe { (*p_new).p_win_defn = core::ptr::null_mut() };
        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0 {
            clear_select(unsafe { (*p_parse).db }, p_new,
                (p_new != &raw mut standin as *mut Select) as i32);
            p_allocated = core::ptr::null_mut();
        } else { { let _ = 0; }; }
        return p_allocated;
    }
}

///* The xExpr callback for the search of invalid ON clause terms.
#[allow(unused_doc_comments)]
extern "C" fn select_check_on_clauses_expr(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let mut p_ctx: *mut CheckOnCtx =
            unsafe { (*p_walker_1).u.p_check_on_ctx };
        if unsafe { (*p_expr_1).flags } & 1 as u32 != 0 as u32 ||
                unsafe { (*p_expr_1).flags } & 2 as u32 != 0 as u32 &&
                    unsafe {
                                    (*(unsafe { (*unsafe { (*p_ctx).p_src }).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)).fg.jointype
                                } as i32 & 64 != 0 {

            /// If CheckOnCtx.iJoin is already set, then fall through and process
            ///* this expression node as normal. Or, if CheckOnCtx.iJoin is still 0,
            ///* set it to the cursor number of the RHS of the join to which this
            ///* ON expression was attached and then iterate through the entire 
            ///* expression.
            { let _ = 0; };
            if unsafe { (*p_ctx).i_join } == 0 {
                unsafe { (*p_ctx).i_join = unsafe { (*p_expr_1).w.i_join } };
                unsafe { sqlite3_walk_expr_nn(p_walker_1, p_expr_1) };
                unsafe { (*p_ctx).i_join = 0 };
                return 1;
            }
        }
        if unsafe { (*p_expr_1).op } as i32 == 168 {
            '__b120: loop {
                '__c120: loop {
                    let p_src: *const SrcList =
                        unsafe { (*p_ctx).p_src } as *const SrcList;
                    let n_src: i32 = unsafe { (*p_src).n_src };
                    let i_tab: i32 = unsafe { (*p_expr_1).i_table };
                    let mut ii: i32 = 0;
                    {
                        ii = 0;
                        '__b121: loop {
                            if !(ii < n_src &&
                                            unsafe {
                                                    (*(unsafe { (*p_src).a.as_ptr() } as
                                                                    *mut SrcItem).offset(ii as isize)).i_cursor
                                                } != i_tab) {
                                break '__b121;
                            }
                            '__c121: loop { break '__c121; }
                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if ii < n_src {
                        if unsafe { (*p_ctx).i_join } != 0 &&
                                i_tab > unsafe { (*p_ctx).i_join } {
                            unsafe {
                                sqlite3_error_msg(unsafe { (*p_walker_1).p_parse },
                                    c"%s references tables to its right".as_ptr() as *mut i8 as
                                        *const i8,
                                    if unsafe { (*p_ctx).b_func_arg } != 0 {
                                        c"table-function argument".as_ptr() as *mut i8
                                    } else { c"ON clause".as_ptr() as *mut i8 })
                            };
                            return 2;
                        }
                        break '__b120;
                    }
                    p_ctx = unsafe { (*p_ctx).p_parent };
                    break '__c120;
                }
                if !(!(p_ctx).is_null()) { break '__b120; }
            }
        }
        return 0;
    }
}

///* The xSelect callback for the search of invalid ON clause terms.
extern "C" fn select_check_on_clauses_select(p_walker_1: *mut Walker,
    p_select_1: *mut Select) -> i32 {
    unsafe {
        let p_ctx: *mut CheckOnCtx =
            unsafe { (*p_walker_1).u.p_check_on_ctx };
        if unsafe { (*p_select_1).p_src } == unsafe { (*p_ctx).p_src } ||
                unsafe { (*unsafe { (*p_select_1).p_src }).n_src } == 0 {
            return 0;
        } else {
            let mut s_ctx: CheckOnCtx = unsafe { core::mem::zeroed() };
            unsafe {
                memset(&raw mut s_ctx as *mut (), 0,
                    core::mem::size_of::<CheckOnCtx>() as u64)
            };
            s_ctx.p_src = unsafe { (*p_select_1).p_src };
            s_ctx.p_parent = p_ctx;
            unsafe { (*p_walker_1).u.p_check_on_ctx = &mut s_ctx };
            unsafe { sqlite3_walk_select(p_walker_1, p_select_1) };
            unsafe { (*p_walker_1).u.p_check_on_ctx = p_ctx };
            unsafe { (*p_select_1).sel_flags &= !1073741824 as u32 };
            return 1;
        }
    }
}

///* Check all ON clauses in pSelect to verify that they do not reference
///* columns to the right.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_select_check_on_clauses(p_parse: *mut Parse,
    p_select: &mut Select) -> () {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        let mut s_ctx: CheckOnCtx = unsafe { core::mem::zeroed() };
        let mut ii: i32 = 0;
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.p_parse = p_parse;
        w.x_expr_callback = Some(select_check_on_clauses_expr);
        w.x_select_callback = Some(select_check_on_clauses_select);
        w.u.p_check_on_ctx = &mut s_ctx;
        unsafe {
            memset(&raw mut s_ctx as *mut (), 0,
                core::mem::size_of::<CheckOnCtx>() as u64)
        };
        s_ctx.p_src = (*p_select).p_src;
        unsafe { sqlite3_walk_expr(&mut w, (*p_select).p_where) };
        (*p_select).sel_flags &= !1073741824 as u32;

        /// Check for any table-function args that are attached to virtual tables 
        ///* on the RHS of an outer join. They are subject to the same constraints
        ///* as ON clauses.
        (s_ctx.b_func_arg = 1);
        {
            ii = 0;
            '__b122: loop {
                if !(ii < unsafe { (*(*p_select).p_src).n_src }) {
                    break '__b122;
                }
                '__c122: loop {
                    let p_item: *const SrcItem =
                        unsafe {
                                &raw mut *(unsafe { (*(*p_select).p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(ii as isize)
                            } as *const SrcItem;
                    if unsafe { (*p_item).fg.is_tab_func() } != 0 &&
                            unsafe { (*p_item).fg.jointype } as i32 & 32 != 0 {
                        s_ctx.i_join = unsafe { (*p_item).i_cursor };
                        unsafe {
                            sqlite3_walk_expr_list(&mut w,
                                unsafe { (*p_item).u1.p_func_arg })
                        };
                    }
                    break '__c122;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

///* Given 1 to 3 identifiers preceding the JOIN keyword, determine the
///* type of join.  Return an integer constant that expresses that type
///* in terms of the following bit values:
///*
///*     JT_INNER
///*     JT_CROSS
///*     JT_OUTER
///*     JT_NATURAL
///*     JT_LEFT
///*     JT_RIGHT
///*
///* A full outer join is the combination of JT_LEFT and JT_RIGHT.
///*
///* If an illegal or unsupported join type is seen, then still return
///* a join type, but put an error in the pParse structure.
///*
///* These are the valid join types:
///*
///*
///*      pA       pB       pC               Return Value
///*     -------  -----    -----             ------------
///*     CROSS      -        -                 JT_CROSS
///*     INNER      -        -                 JT_INNER
///*     LEFT       -        -                 JT_LEFT|JT_OUTER
///*     LEFT     OUTER      -                 JT_LEFT|JT_OUTER
///*     RIGHT      -        -                 JT_RIGHT|JT_OUTER
///*     RIGHT    OUTER      -                 JT_RIGHT|JT_OUTER
///*     FULL       -        -                 JT_LEFT|JT_RIGHT|JT_OUTER
///*     FULL     OUTER      -                 JT_LEFT|JT_RIGHT|JT_OUTER
///*     NATURAL  INNER      -                 JT_NATURAL|JT_INNER
///*     NATURAL  LEFT       -                 JT_NATURAL|JT_LEFT|JT_OUTER
///*     NATURAL  LEFT     OUTER               JT_NATURAL|JT_LEFT|JT_OUTER
///*     NATURAL  RIGHT      -                 JT_NATURAL|JT_RIGHT|JT_OUTER
///*     NATURAL  RIGHT    OUTER               JT_NATURAL|JT_RIGHT|JT_OUTER
///*     NATURAL  FULL       -                 JT_NATURAL|JT_LEFT|JT_RIGHT
///*     NATURAL  FULL     OUTER               JT_NATRUAL|JT_LEFT|JT_RIGHT
///*
///* To preserve historical compatibly, SQLite also accepts a variety
///* of other non-standard and in many cases nonsensical join types.
///* This routine makes as much sense at it can from the nonsense join
///* type and returns a result.  Examples of accepted nonsense join types
///* include but are not limited to:
///*
///*          INNER CROSS JOIN        ->   same as JOIN
///*          NATURAL CROSS JOIN      ->   same as NATURAL JOIN
///*          OUTER LEFT JOIN         ->   same as LEFT JOIN
///*          LEFT NATURAL JOIN       ->   same as NATURAL LEFT JOIN
///*          LEFT RIGHT JOIN         ->   same as FULL JOIN
///*          RIGHT OUTER FULL JOIN   ->   same as FULL JOIN
///*          CROSS CROSS CROSS JOIN  ->   same as JOIN
///*
///* The only restrictions on the join type name are:
///*
///*    *   "INNER" cannot appear together with "OUTER", "LEFT", "RIGHT",
///*        or "FULL".
///*
///*    *   "CROSS" cannot appear together with "OUTER", "LEFT", "RIGHT,
///*        or "FULL".
///*
///*    *   If "OUTER" is present then there must also be one of
///*        "LEFT", "RIGHT", or "FULL"
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_join_type(p_parse: *mut Parse, p_a: *mut Token,
    p_b: *mut Token, p_c: *mut Token) -> i32 {
    unsafe {
        let mut jointype: i32 = 0;
        let mut ap_all: [*mut Token; 3] = [core::ptr::null_mut(); 3];
        let mut p: *mut Token = core::ptr::null_mut();
        /// Beginning of keyword text in zKeyText[]
        /// Length of the keyword in characters
        /// Join type mask
        /// (0) natural
        /// (1) left
        /// (2) outer
        /// (3) right
        /// (4) full
        /// (5) inner
        /// (6) cross
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        ap_all[0 as usize] = p_a;
        ap_all[1 as usize] = p_b;
        ap_all[2 as usize] = p_c;
        {
            i = 0;
            '__b123: loop {
                if !(i < 3 && !(ap_all[i as usize]).is_null()) {
                    break '__b123;
                }
                '__c123: loop {
                    p = ap_all[i as usize];
                    {
                        j = 0;
                        '__b124: loop {
                            if !(j <
                                            (core::mem::size_of::<[Sqlite3JoinTypeS0N20sqlite3JoinTypeS0; 7]>()
                                                        as u64 / 3) as i32) {
                                break '__b124;
                            }
                            '__c124: loop {
                                if unsafe { (*p).n } == a_keyword[j as usize].n_char as u32
                                        &&
                                        unsafe {
                                                sqlite3_strnicmp(unsafe { (*p).z } as *mut i8 as *const i8,
                                                    &z_key_text[a_keyword[j as usize].i as usize],
                                                    unsafe { (*p).n } as i32)
                                            } == 0 {
                                    jointype |= a_keyword[j as usize].code as i32;
                                    break '__b124;
                                }
                                break '__c124;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if j >=
                            (core::mem::size_of::<[Sqlite3JoinTypeS0N20sqlite3JoinTypeS0; 7]>()
                                        as u64 / 3) as i32 {
                        jointype |= 128;
                        break '__b123;
                    }
                    break '__c123;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if jointype & (1 | 32) == 1 | 32 || jointype & 128 != 0 ||
                jointype & (32 | 8 | 16) == 32 {
            let mut z_sp1: *const i8 = c" ".as_ptr() as *mut i8 as *const i8;
            let mut z_sp2: *const i8 = c" ".as_ptr() as *mut i8 as *const i8;
            if p_b == core::ptr::null_mut() {
                {
                    let __p = &mut z_sp1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            if p_c == core::ptr::null_mut() {
                {
                    let __p = &mut z_sp2;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"unknown join type: %T%s%T%s%T".as_ptr() as *mut i8 as
                        *const i8, p_a, z_sp1, p_b, z_sp2, p_c)
            };
            jointype = 1;
        }
        return jointype;
    }
}

///* Error message for when two or more terms of a compound select have different
///* size result sets.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_select_wrong_num_terms_error(p_parse: *mut Parse,
    p: &Select) -> () {
    if (*p).sel_flags & 512 as u32 != 0 {
        unsafe {
            sqlite3_error_msg(p_parse,
                c"all VALUES must have the same number of terms".as_ptr() as
                        *mut i8 as *const i8)
        };
    } else {
        unsafe {
            sqlite3_error_msg(p_parse,
                c"SELECTs to the left and right of %s do not have the same number of result columns".as_ptr()
                        as *mut i8 as *const i8,
                sqlite3_select_op_name((*p).op as i32))
        };
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlite3JoinTypeS0N20sqlite3JoinTypeS0 {
    i: u8,
    n_char: u8,
    code: u8,
}

static mut tk_coalesce: Token =
    Token { z: c"coalesce".as_ptr() as *const i8, n: 8 as u32 };

static z_key_text: [i8; 34] =
    [110 as i8, 97 as i8, 116 as i8, 117 as i8, 114 as i8, 97 as i8,
            108 as i8, 101 as i8, 102 as i8, 116 as i8, 111 as i8, 117 as i8,
            116 as i8, 101 as i8, 114 as i8, 105 as i8, 103 as i8, 104 as i8,
            116 as i8, 102 as i8, 117 as i8, 108 as i8, 108 as i8, 105 as i8,
            110 as i8, 110 as i8, 101 as i8, 114 as i8, 99 as i8, 114 as i8,
            111 as i8, 115 as i8, 115 as i8, 0 as i8];

static a_keyword: [Sqlite3JoinTypeS0N20sqlite3JoinTypeS0; 7] =
    [Sqlite3JoinTypeS0N20sqlite3JoinTypeS0 {
                i: 0 as u8,
                n_char: 7 as u8,
                code: 4 as u8,
            },
            Sqlite3JoinTypeS0N20sqlite3JoinTypeS0 {
                i: 6 as u8,
                n_char: 4 as u8,
                code: (8 | 32) as u8,
            },
            Sqlite3JoinTypeS0N20sqlite3JoinTypeS0 {
                i: 10 as u8,
                n_char: 5 as u8,
                code: 32 as u8,
            },
            Sqlite3JoinTypeS0N20sqlite3JoinTypeS0 {
                i: 14 as u8,
                n_char: 5 as u8,
                code: (16 | 32) as u8,
            },
            Sqlite3JoinTypeS0N20sqlite3JoinTypeS0 {
                i: 19 as u8,
                n_char: 4 as u8,
                code: (8 | 16 | 32) as u8,
            },
            Sqlite3JoinTypeS0N20sqlite3JoinTypeS0 {
                i: 23 as u8,
                n_char: 5 as u8,
                code: 1 as u8,
            },
            Sqlite3JoinTypeS0N20sqlite3JoinTypeS0 {
                i: 28 as u8,
                n_char: 5 as u8,
                code: (1 | 2) as u8,
            }];

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
    fn sqlite3_row_set_clear(_: *mut ())
    -> ();
    fn sqlite3_expr_skip_collate_and_likely(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_is_true_or_false(_: *const i8)
    -> u32;
    static sqlite3_ctype_map: [u8; 0];
    fn sqlite3_str_i_hash(_: *const i8)
    -> u8;
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn sqlite3_expr_affinity(p_expr_1: *const Expr)
    -> i8;
    fn sqlite3_expr_data_type(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_affinity_type(_: *const i8, _: *mut Column)
    -> i8;
    static sqlite3_std_type_affinity: [i8; 0];
    static mut sqlite3_std_type: [*const i8; 0];
    fn sqlite3_expr_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_src_list_append_from_term(_: *mut Parse, _: *mut SrcList,
    _: *mut Token, _: *mut Token, _: *mut Token, _: *mut Select,
    _: *mut OnOrUsing)
    -> *mut SrcList;
    fn sqlite3_src_list_delete(_: *mut Sqlite3, _: *mut SrcList)
    -> ();
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_with_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_src_list_assign_cursors(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_src_item_attach_subquery(_: *mut Parse, _: *mut SrcItem,
    _: *mut Select, _: i32)
    -> i32;
    fn sqlite3_locate_table_item(_: *mut Parse, flags: u32, _: *mut SrcItem)
    -> *mut Table;
    fn sqlite3_view_get_column_names(_: *mut Parse, _: *mut Table)
    -> i32;
    fn sqlite3_id_list_append(_: *mut Parse, _: *mut IdList, _: *mut Token)
    -> *mut IdList;
    fn sqlite3_create_column_expr(_: *mut Sqlite3, _: *mut SrcList, _: i32,
    _: i32)
    -> *mut Expr;
    fn sqlite3_id_list_index(_: *mut IdList, _: *const i8)
    -> i32;
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_expr_set_error_offset(_: *mut Expr, _: i32)
    -> ();
    fn sqlite3_rowid_alias(p_tab_1: *mut Table)
    -> *const i8;
    fn sqlite3_match_e_name(_: *const ExprListItem, _: *const i8,
    _: *const i8, _: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_rename_token_remap(_: *mut Parse, p_to_1: *const (),
    p_from_1: *const ())
    -> ();
    fn sqlite3_resolve_select_names(_: *mut Parse, _: *mut Select,
    _: *mut NameContext)
    -> ();
    fn sqlite3_delete_table(_: *mut Sqlite3, _: *mut Table)
    -> ();
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
    fn sqlite3_src_list_indexed_by(_: *mut Parse, _: *mut SrcList,
    _: *mut Token)
    -> ();
    fn sqlite3_src_list_func_args(_: *mut Parse, _: *mut SrcList,
    _: *mut ExprList)
    -> ();
    fn sqlite3_src_list_shift_join_type(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_id_list_delete(_: *mut Sqlite3, _: *mut IdList)
    -> ();
    fn sqlite3_clear_on_or_using(_: *mut Sqlite3, _: *mut OnOrUsing)
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
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_expr_implies_non_null_row(_: *mut Expr, _: i32, _: i32)
    -> i32;
    fn sqlite3_select_dup(_: *mut Sqlite3, _: *const Select, _: i32)
    -> *mut Select;
    fn sqlite3_expr_is_vector(p_expr_1: *const Expr)
    -> i32;
    fn sqlite3_vector_error_msg(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_expr_dup(_: *mut Sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_truth_value(_: *const Expr)
    -> i32;
    fn sqlite3_expr_add_collate_string(_: *const Parse, _: *mut Expr,
    _: *const i8)
    -> *mut Expr;
    fn sqlite3_expr_col_used(_: *mut Expr)
    -> Bitmask;
    fn sqlite3_agg_info_persist_walker_init(_: *mut Walker, _: *mut Parse)
    -> ();
    fn sqlite3_with_delete(_: *mut Sqlite3, _: *mut With)
    -> ();
    fn sqlite3_expr_code_expr_list(_: *mut Parse, _: *mut ExprList, _: i32,
    _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_code_move(_: *mut Parse, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_nn_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_expr_is_integer(_: *const Expr, _: *mut i32, _: *mut Parse)
    -> i32;
    fn sqlite3_log_est(_: u64)
    -> LogEst;
    fn sqlite3_expr_code(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_list_dup(_: *mut Sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_resolve_order_group_by(_: *mut Parse, _: *mut Select,
    _: *mut ExprList, _: *const i8)
    -> i32;
    fn sqlite3_log_est_add(_: LogEst, _: LogEst)
    -> LogEst;
    fn sqlite3_expr_is_constant(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_is_binary(_: *const CollSeq)
    -> i32;
    fn sqlite3_expr_compare_coll_seq(_: *mut Parse, _: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_select_expr_height(_: *const Select)
    -> i32;
    fn sqlite3_expr_is_single_table_constraint(_: *mut Expr,
    _: *const SrcList, _: i32, _: i32)
    -> i32;
    fn sqlite3_expr_is_constant_or_group_by(_: *mut Parse, _: *mut Expr,
    _: *mut ExprList)
    -> i32;
    fn sqlite3_expr_list_compare(_: *const ExprList, _: *const ExprList,
    _: i32)
    -> i32;
    fn sqlite3_where_begin(_: *mut Parse, _: *mut SrcList, _: *mut Expr,
    _: *mut ExprList, _: *mut ExprList, _: *mut Select, _: u16, _: i32)
    -> *mut WhereInfo;
    fn sqlite3_where_output_row_count(_: *mut WhereInfo)
    -> LogEst;
    fn sqlite3_where_is_distinct(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_is_ordered(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_order_by_limit_opt_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_continue_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_break_label(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_where_end(_: *mut WhereInfo)
    -> ();
    fn sqlite3_expr_analyze_agg_list(_: *mut NameContext, _: *mut ExprList)
    -> ();
    fn sqlite3_expr_analyze_aggregates(_: *mut NameContext, _: *mut Expr)
    -> ();
    fn sqlite3_expr_can_be_null(_: *const Expr)
    -> i32;
    fn sqlite3_expr_null_register_range(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_where_is_sorted(_: *mut WhereInfo)
    -> i32;
    fn sqlite3_expr_to_register(p_expr_1: *mut Expr, i_reg_1: i32)
    -> ();
    fn sqlite3_expr_if_false(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_code_verify_schema(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_key_info_of_index(_: *mut Parse, _: *mut Index)
    -> *mut KeyInfo;
    fn sqlite3_where_min_max_opt_early_out(_: *mut Vdbe, _: *mut WhereInfo)
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
    fn sqlite3_expr_code_generated_column(_: *mut Parse, _: *mut Table,
    _: *mut Column, _: i32)
    -> ();
    fn sqlite3_expr_code_copy(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_factorable(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_run_just_once(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_code_temp(_: *mut Parse, _: *mut Expr, _: *mut i32)
    -> i32;
    fn sqlite3_expr_code_target(_: *mut Parse, _: *mut Expr, _: i32)
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
    fn sqlite3_expr_implies_expr(_: *const Parse, _: *const Expr,
    _: *const Expr, _: i32)
    -> i32;
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
    fn sqlite3_expr_id_to_true_false(_: *mut Expr)
    -> i32;
    fn sqlite3_expr_is_constant_or_function(_: *mut Expr, _: u8)
    -> i32;
    fn sqlite3_expr_needs_no_affinity_change(_: *const Expr, _: i8)
    -> i32;
    fn sqlite3_expr_is_like_operator(_: *const Expr)
    -> i32;
    fn sqlite3_is_rowid(_: *const i8)
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
    fn sqlite3_may_abort(_: *mut Parse)
    -> ();
    fn sqlite3_halt_constraint(_: *mut Parse, _: i32, _: i32, _: *mut i8,
    _: i8, _: u8)
    -> ();
    fn sqlite3_unique_constraint(_: *mut Parse, _: i32, _: *mut Index)
    -> ();
    fn sqlite3_rowid_constraint(_: *mut Parse, _: i32, _: *mut Table)
    -> ();
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
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_set_text_encoding(db: *mut Sqlite3, _: u8)
    -> ();
    fn sqlite3_expr_coll_seq_match(_: *mut Parse, _: *const Expr,
    _: *const Expr)
    -> i32;
    fn sqlite3_expr_add_collate_token(p_parse_1: *const Parse, _: *mut Expr,
    _: *const Token, _: i32)
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
    fn sqlite3_code_rhs_of_in(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_code_subselect(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
    -> i32;
    fn sqlite3_resolve_expr_list_names(_: *mut NameContext, _: *mut ExprList)
    -> i32;
    fn sqlite3_resolve_self_reference(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut Expr, _: *mut ExprList)
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
    fn sqlite3_reprepare(_: *mut Vdbe)
    -> i32;
    fn sqlite3_expr_list_check_length(_: *mut Parse, _: *mut ExprList,
    _: *const i8)
    -> ();
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
    fn sqlite3_expr_check_height(_: *mut Parse, _: i32)
    -> i32;
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
    fn sqlite3_vector_field_subexpr(_: *mut Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_for_vector_field(_: *mut Parse, _: *mut Expr, _: i32,
    _: i32)
    -> *mut Expr;
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
struct WindowRewrite {
    _opaque: [u8; 0],
}
