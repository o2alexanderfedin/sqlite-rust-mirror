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
    AuthContext, Bitmask, Bitvec, BusyHandler, CollSeq, Column, Cte, DbFixer,
    Expr, ExprList, ExprListItem, ExprListItemS0, FKey, FpDecode, FuncDef,
    FuncDefHash, FuncDefU0, FuncDestructor, IdList, Index, KeyInfo, LogEst,
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

#[repr(C)]
#[derive(Copy, Clone)]
struct WindowRewrite {
    p_win: *mut Window,
    p_src: *mut SrcList,
    p_sub: *mut ExprList,
    p_tab: *mut Table,
    p_sub_select: *mut Select,
}

///* Unlink the Window object from the Select to which it is attached,
///* if it is attached.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_unlink_from_select(p: &mut Window) -> () {
    if !((*p).pp_this).is_null() {
        unsafe { *(*p).pp_this = (*p).p_next_win };
        if !((*p).p_next_win).is_null() {
            unsafe { (*(*p).p_next_win).pp_this = (*p).pp_this };
        }
        (*p).pp_this = core::ptr::null_mut();
    }
}

///* Free the Window object passed as the second argument.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_delete(db: *mut Sqlite3, p: *mut Window)
    -> () {
    if !(p).is_null() {
        sqlite3_window_unlink_from_select(unsafe { &mut *p });
        unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_filter }) };
        unsafe { sqlite3_expr_list_delete(db, unsafe { (*p).p_partition }) };
        unsafe { sqlite3_expr_list_delete(db, unsafe { (*p).p_order_by }) };
        unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_end }) };
        unsafe { sqlite3_expr_delete(db, unsafe { (*p).p_start }) };
        unsafe { sqlite3_db_free(db, unsafe { (*p).z_name } as *mut ()) };
        unsafe { sqlite3_db_free(db, unsafe { (*p).z_base } as *mut ()) };
        unsafe { sqlite3_db_free(db, p as *mut ()) };
    }
}

///* Free the linked list of Window objects starting at the second argument.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_list_delete(db: *mut Sqlite3,
    mut p: *mut Window) -> () {
    while !(p).is_null() {
        let p_next: *mut Window = unsafe { (*p).p_next_win };
        sqlite3_window_delete(db, p);
        p = p_next;
    }
}

///* The argument expression is an PRECEDING or FOLLOWING offset.  The
///* value should be a non-negative integer.  If the value is not a
///* constant, change it to NULL.  The fact that it is then a non-negative
///* integer will be caught later.  But it is important not to leave
///* variable values in the expression tree.
extern "C" fn sqlite3_window_offset_expr(p_parse_1: *mut Parse,
    mut p_expr_1: *mut Expr) -> *mut Expr {
    if 0 ==
            unsafe {
                sqlite3_expr_is_constant(core::ptr::null_mut(), p_expr_1)
            } {
        if unsafe { (*p_parse_1).e_parse_mode } as i32 >= 2 {
            unsafe { sqlite3_rename_expr_unmap(p_parse_1, p_expr_1) };
        }
        unsafe { sqlite3_expr_delete(unsafe { (*p_parse_1).db }, p_expr_1) };
        p_expr_1 =
            unsafe {
                sqlite3_expr_alloc(unsafe { (*p_parse_1).db }, 122,
                    core::ptr::null(), 0)
            };
    }
    return p_expr_1;
}

///* Allocate and return a new Window object describing a Window Definition.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_window_alloc(p_parse: *mut Parse, mut e_type: i32,
    e_start: i32, p_start: *mut Expr, e_end: i32, p_end: *mut Expr,
    mut e_exclude: u8) -> *mut Window {
    '__b1: loop {
        '__c1: loop {
            let mut p_win: *mut Window = core::ptr::null_mut();
            let mut b_implicit_frame: i32 = 0;

            /// Parser assures the following:
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            if e_type == 0 { b_implicit_frame = 1; e_type = 90; }
            if e_start == 86 && e_end == 89 ||
                    e_start == 87 && (e_end == 89 || e_end == 86) {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"unsupported frame specification".as_ptr() as *mut i8 as
                            *const i8)
                };
                break '__b1;
            }
            p_win =
                unsafe {
                        sqlite3_db_malloc_zero(unsafe { (*p_parse).db },
                            core::mem::size_of::<Window>() as u64)
                    } as *mut Window;
            if p_win == core::ptr::null_mut() { break '__b1; }
            unsafe { (*p_win).e_frm_type = e_type as u8 };
            unsafe { (*p_win).e_start = e_start as u8 };
            unsafe { (*p_win).e_end = e_end as u8 };
            if e_exclude as i32 == 0 &&
                    unsafe { (*unsafe { (*p_parse).db }).db_opt_flags } &
                            2 as u32 != 0 as u32 {
                e_exclude = 67 as u8;
            }
            unsafe { (*p_win).e_exclude = e_exclude };
            unsafe { (*p_win).b_implicit_frame = b_implicit_frame as u8 };
            unsafe {
                (*p_win).p_end = sqlite3_window_offset_expr(p_parse, p_end)
            };
            unsafe {
                (*p_win).p_start =
                    sqlite3_window_offset_expr(p_parse, p_start)
            };
            return p_win;
            break '__c1;
        }
        if !(false) { break '__b1; }
    }

    /// Parser assures the following:
    /// Additionally, the
    ///* starting boundary type may not occur earlier in the following list than
    ///* the ending boundary type:
    ///*
    ///*   UNBOUNDED PRECEDING
    ///*   <expr> PRECEDING
    ///*   CURRENT ROW
    ///*   <expr> FOLLOWING
    ///*   UNBOUNDED FOLLOWING
    ///*
    ///* The parser ensures that "UNBOUNDED PRECEDING" cannot be used as an ending
    ///* boundary, and than "UNBOUNDED FOLLOWING" cannot be used as a starting
    ///* frame boundary.
    unsafe { sqlite3_expr_delete(unsafe { (*p_parse).db }, p_end) };
    unsafe { sqlite3_expr_delete(unsafe { (*p_parse).db }, p_start) };
    return core::ptr::null_mut();
}

///* Attach window object pWin to expression p.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_attach(p_parse: *mut Parse, p: *mut Expr,
    p_win: *mut Window) -> () {
    unsafe {
        if !(p).is_null() {
            { let _ = 0; };
            { let _ = 0; };
            { let _ = 0; };
            unsafe { (*p).y.p_win = p_win };
            unsafe { (*p).flags |= (16777216 | 131072) as u32 };
            unsafe { (*p_win).p_owner = p };
            if unsafe { (*p).flags } & 4 as u32 != 0 &&
                    unsafe { (*p_win).e_frm_type } as i32 != 167 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"DISTINCT is not supported for window functions".as_ptr()
                                as *mut i8 as *const i8)
                };
            }
        } else { sqlite3_window_delete(unsafe { (*p_parse).db }, p_win); }
    }
}

///* Return 0 if the two window objects are identical, 1 if they are
///* different, or 2 if it cannot be determined if the objects are identical
///* or not. Identical window objects can be processed in a single scan.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_compare(p_parse: *const Parse,
    p1: *const Window, p2: *const Window, b_filter: i32) -> i32 {
    let mut res: i32 = 0;
    if p1 == core::ptr::null() || p2 == core::ptr::null() { return 1; }
    if unsafe { (*p1).e_frm_type } as i32 !=
            unsafe { (*p2).e_frm_type } as i32 {
        return 1;
    }
    if unsafe { (*p1).e_start } as i32 != unsafe { (*p2).e_start } as i32 {
        return 1;
    }
    if unsafe { (*p1).e_end } as i32 != unsafe { (*p2).e_end } as i32 {
        return 1;
    }
    if unsafe { (*p1).e_exclude } as i32 != unsafe { (*p2).e_exclude } as i32
        {
        return 1;
    }
    if unsafe {
                sqlite3_expr_compare(p_parse,
                    unsafe { (*p1).p_start } as *const Expr,
                    unsafe { (*p2).p_start } as *const Expr, -1)
            } != 0 {
        return 1;
    }
    if unsafe {
                sqlite3_expr_compare(p_parse,
                    unsafe { (*p1).p_end } as *const Expr,
                    unsafe { (*p2).p_end } as *const Expr, -1)
            } != 0 {
        return 1;
    }
    if {
                res =
                    unsafe {
                        sqlite3_expr_list_compare(unsafe { (*p1).p_partition } as
                                *const ExprList,
                            unsafe { (*p2).p_partition } as *const ExprList, -1)
                    };
                res
            } != 0 {
        return res;
    }
    if {
                res =
                    unsafe {
                        sqlite3_expr_list_compare(unsafe { (*p1).p_order_by } as
                                *const ExprList,
                            unsafe { (*p2).p_order_by } as *const ExprList, -1)
                    };
                res
            } != 0 {
        return res;
    }
    if b_filter != 0 {
        if {
                    res =
                        unsafe {
                            sqlite3_expr_compare(p_parse,
                                unsafe { (*p1).p_filter } as *const Expr,
                                unsafe { (*p2).p_filter } as *const Expr, -1)
                        };
                    res
                } != 0 {
            return res;
        }
    }
    return 0;
}

///* Possibly link window pWin into the list at pSel->pWin (window functions
///* to be processed as part of SELECT statement pSel). The window is linked
///* in if either (a) there are no other windows already linked to this
///* SELECT, or (b) the windows already linked use a compatible window frame.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_link(p_sel: *mut Select, p_win: *mut Window)
    -> () {
    unsafe {
        if !(p_sel).is_null() {
            if core::ptr::null_mut() == unsafe { (*p_sel).p_win } ||
                    0 ==
                        sqlite3_window_compare(core::ptr::null(),
                            unsafe { (*p_sel).p_win } as *const Window,
                            p_win as *const Window, 0) {
                unsafe { (*p_win).p_next_win = unsafe { (*p_sel).p_win } };
                if !(unsafe { (*p_sel).p_win }).is_null() {
                    unsafe {
                        (*unsafe { (*p_sel).p_win }).pp_this =
                            unsafe { &mut (*p_win).p_next_win }
                    };
                }
                unsafe { (*p_sel).p_win = p_win };
                unsafe { (*p_win).pp_this = unsafe { &mut (*p_sel).p_win } };
            } else {
                if unsafe {
                            sqlite3_expr_list_compare(unsafe { (*p_win).p_partition } as
                                    *const ExprList,
                                unsafe { (*unsafe { (*p_sel).p_win }).p_partition } as
                                    *const ExprList, -1)
                        } != 0 {
                    unsafe { (*p_sel).sel_flags |= 33554432 as u32 };
                }
            }
        }
    }
}

static nth_value_name: [i8; 10] =
    [110 as i8, 116 as i8, 104 as i8, 95 as i8, 118 as i8, 97 as i8,
            108 as i8, 117 as i8, 101 as i8, 0 as i8];

static first_value_name: [i8; 12] =
    [102 as i8, 105 as i8, 114 as i8, 115 as i8, 116 as i8, 95 as i8,
            118 as i8, 97 as i8, 108 as i8, 117 as i8, 101 as i8, 0 as i8];

static lead_name: [i8; 5] =
    [108 as i8, 101 as i8, 97 as i8, 100 as i8, 0 as i8];

static lag_name: [i8; 4] = [108 as i8, 97 as i8, 103 as i8, 0 as i8];

///* This is called by code in select.c before it calls sqlite3WhereBegin()
///* to begin iterating through the sub-query results. It is used to allocate
///* and initialize registers and cursors used by sqlite3WindowCodeStep().
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_window_code_init(p_parse: *mut Parse,
    p_select: &Select) -> () {
    unsafe {
        let mut p_win: *mut Window = core::ptr::null_mut();
        let mut n_eph_expr: i32 = 0;
        let mut p_m_win: *mut Window = core::ptr::null_mut();
        let mut v: *mut Vdbe = core::ptr::null_mut();
        { let _ = 0; };
        n_eph_expr =
            unsafe {
                (*unsafe {
                                (*unsafe {
                                                (*unsafe {
                                                                (*(unsafe { (*(*p_select).p_src).a.as_ptr() } as
                                                                                    *mut SrcItem).offset(0 as isize)).u4.p_subq
                                                            }).p_select
                                            }).p_e_list
                            }).n_expr
            };
        p_m_win = (*p_select).p_win;
        v = unsafe { sqlite3_get_vdbe(p_parse) };
        unsafe {
            sqlite3_vdbe_add_op2(v, 120, unsafe { (*p_m_win).i_eph_csr },
                n_eph_expr)
        };
        unsafe {
            sqlite3_vdbe_add_op2(v, 117, unsafe { (*p_m_win).i_eph_csr } + 1,
                unsafe { (*p_m_win).i_eph_csr })
        };
        unsafe {
            sqlite3_vdbe_add_op2(v, 117, unsafe { (*p_m_win).i_eph_csr } + 2,
                unsafe { (*p_m_win).i_eph_csr })
        };
        unsafe {
            sqlite3_vdbe_add_op2(v, 117, unsafe { (*p_m_win).i_eph_csr } + 3,
                unsafe { (*p_m_win).i_eph_csr })
        };
        if !(unsafe { (*p_m_win).p_partition }).is_null() {
            let n_expr: i32 =
                unsafe { (*unsafe { (*p_m_win).p_partition }).n_expr };
            unsafe { (*p_m_win).reg_part = unsafe { (*p_parse).n_mem } + 1 };
            unsafe { (*p_parse).n_mem += n_expr };
            unsafe {
                sqlite3_vdbe_add_op3(v, 77, 0, unsafe { (*p_m_win).reg_part },
                    unsafe { (*p_m_win).reg_part } + n_expr - 1)
            };
        }
        unsafe {
            (*p_m_win).reg_one =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                }
        };
        unsafe {
            sqlite3_vdbe_add_op2(v, 73, 1, unsafe { (*p_m_win).reg_one })
        };
        if unsafe { (*p_m_win).e_exclude } != 0 {
            unsafe {
                (*p_m_win).reg_start_rowid =
                    {
                        let __p = unsafe { &mut (*p_parse).n_mem };
                        *__p += 1;
                        *__p
                    }
            };
            unsafe {
                (*p_m_win).reg_end_rowid =
                    {
                        let __p = unsafe { &mut (*p_parse).n_mem };
                        *__p += 1;
                        *__p
                    }
            };
            unsafe {
                (*p_m_win).csr_app =
                    {
                        let __p = unsafe { &mut (*p_parse).n_tab };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    }
            };
            unsafe {
                sqlite3_vdbe_add_op2(v, 73, 1,
                    unsafe { (*p_m_win).reg_start_rowid })
            };
            unsafe {
                sqlite3_vdbe_add_op2(v, 73, 0,
                    unsafe { (*p_m_win).reg_end_rowid })
            };
            unsafe {
                sqlite3_vdbe_add_op2(v, 117, unsafe { (*p_m_win).csr_app },
                    unsafe { (*p_m_win).i_eph_csr })
            };
            return;
        }
        {
            p_win = p_m_win;
            '__b2: loop {
                if !(!(p_win).is_null()) { break '__b2; }
                '__c2: loop {
                    let p: *const FuncDef =
                        unsafe { (*p_win).p_w_func } as *const FuncDef;
                    if unsafe { (*p).func_flags } & 4096 as u32 != 0 &&
                            unsafe { (*p_win).e_start } as i32 != 91 {
                        /// The inline versions of min() and max() require a single ephemeral
                        ///* table and 3 registers. The registers are used as follows:
                        ///*
                        ///*   regApp+0: slot to copy min()/max() argument to for MakeRecord
                        ///*   regApp+1: integer value used to ensure keys are unique
                        ///*   regApp+2: output of MakeRecord
                        let mut p_list: *mut ExprList = core::ptr::null_mut();
                        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
                        { let _ = 0; };
                        p_list = unsafe { (*unsafe { (*p_win).p_owner }).x.p_list };
                        p_key_info =
                            unsafe {
                                sqlite3_key_info_from_expr_list(p_parse, p_list, 0, 0)
                            };
                        unsafe {
                            (*p_win).csr_app =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                }
                        };
                        unsafe {
                            (*p_win).reg_app = unsafe { (*p_parse).n_mem } + 1
                        };
                        unsafe { (*p_parse).n_mem += 3 };
                        if !(p_key_info).is_null() &&
                                unsafe {
                                            *unsafe {
                                                    (*unsafe { (*p_win).p_w_func }).z_name.offset(1 as isize)
                                                }
                                        } as i32 == 'i' as i32 {
                            { let _ = 0; };
                            unsafe {
                                *unsafe { (*p_key_info).a_sort_flags.offset(0 as isize) } =
                                    1 as u8
                            };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 120, unsafe { (*p_win).csr_app }, 2)
                        };
                        unsafe {
                            sqlite3_vdbe_append_p4(v, p_key_info as *mut (), -9)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 0,
                                unsafe { (*p_win).reg_app } + 1)
                        };
                    } else if unsafe { (*p).z_name } ==
                                &raw const nth_value_name[0 as usize] as *const i8 ||
                            unsafe { (*p).z_name } ==
                                &raw const first_value_name[0 as usize] as *const i8 {

                        /// Allocate two registers at pWin->regApp. These will be used to
                        ///* store the start and end index of the current frame.
                        unsafe {
                            (*p_win).reg_app = unsafe { (*p_parse).n_mem } + 1
                        };
                        unsafe {
                            (*p_win).csr_app =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                }
                        };
                        unsafe { (*p_parse).n_mem += 2 };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 117, unsafe { (*p_win).csr_app },
                                unsafe { (*p_m_win).i_eph_csr })
                        };
                    } else if unsafe { (*p).z_name } ==
                                &raw const lead_name[0 as usize] as *const i8 ||
                            unsafe { (*p).z_name } ==
                                &raw const lag_name[0 as usize] as *const i8 {
                        unsafe {
                            (*p_win).csr_app =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_tab };
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                }
                        };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 117, unsafe { (*p_win).csr_app },
                                unsafe { (*p_m_win).i_eph_csr })
                        };
                    }
                    break '__c2;
                }
                p_win = unsafe { (*p_win).p_next_win };
            }
        }
    }
}

///* A single instance of this structure is allocated on the stack by
///* sqlite3WindowCodeStep() and a pointer to it passed to the various helper
///* routines. This is to reduce the number of arguments required by each
///* helper function.
///*
///* regArg:
///*   Each window function requires an accumulator register (just as an
///*   ordinary aggregate function does). This variable is set to the first
///*   in an array of accumulator registers - one for each window function
///*   in the WindowCodeArg.pMWin list.
///*
///* eDelete:
///*   The window functions implementation sometimes caches the input rows
///*   that it processes in a temporary table. If it is not zero, this
///*   variable indicates when rows may be removed from the temp table (in
///*   order to reduce memory requirements - it would always be safe just
///*   to leave them there). Possible values for eDelete are:
///*
///*      WINDOW_RETURN_ROW:
///*        An input row can be discarded after it is returned to the caller.
///*
///*      WINDOW_AGGINVERSE:
///*        An input row can be discarded after the window functions xInverse()
///*        callbacks have been invoked in it.
///*
///*      WINDOW_AGGSTEP:
///*        An input row can be discarded after the window functions xStep()
///*        callbacks have been invoked in it.
///*
///* start,current,end
///*   Consider a window-frame similar to the following:
///*
///*     (ORDER BY a, b GROUPS BETWEEN 2 PRECEDING AND 2 FOLLOWING)
///*
///*   The windows functions implementation caches the input rows in a temp
///*   table, sorted by "a, b" (it actually populates the cache lazily, and
///*   aggressively removes rows once they are no longer required, but that's
///*   a mere detail). It keeps three cursors open on the temp table. One
///*   (current) that points to the next row to return to the query engine
///*   once its window function values have been calculated. Another (end)
///*   points to the next row to call the xStep() method of each window function
///*   on (so that it is 2 groups ahead of current). And a third (start) that
///*   points to the next row to call the xInverse() method of each window
///*   function on.
///*
///*   Each cursor (start, current and end) consists of a VDBE cursor
///*   (WindowCsrAndReg.csr) and an array of registers (starting at
///*   WindowCodeArg.reg) that always contains a copy of the peer values
///*   read from the corresponding cursor.
///*
///*   Depending on the window-frame in question, all three cursors may not
///*   be required. In this case both WindowCodeArg.csr and reg are set to
///*   0.
#[repr(C)]
#[derive(Copy, Clone)]
struct WindowCodeArg {
    p_parse: *mut Parse,
    p_m_win: *mut Window,
    p_vdbe: *mut Vdbe,
    addr_gosub: i32,
    reg_gosub: i32,
    reg_arg: i32,
    e_delete: i32,
    reg_rowid: i32,
    start: WindowCsrAndReg,
    current: WindowCsrAndReg,
    end: WindowCsrAndReg,
}

///* See comments above struct WindowCodeArg.
#[repr(C)]
#[derive(Copy, Clone)]
struct WindowCsrAndReg {
    csr: i32,
    reg: i32,
}

///* Return true if it can be determined at compile time that expression
///* pExpr evaluates to a value that, when cast to an integer, is greater
///* than zero. False otherwise.
///*
///* If an OOM error occurs, this function sets the Parse.db.mallocFailed
///* flag and returns zero.
extern "C" fn window_expr_gt_zero(p_parse_1: &Parse, p_expr_1: *const Expr)
    -> i32 {
    let mut ret: i32 = 0;
    let db: *mut Sqlite3 = (*p_parse_1).db;
    let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
    unsafe {
        sqlite3_value_from_expr(db, p_expr_1 as *const Expr,
            unsafe { (*db).enc }, 67 as u8, &mut p_val)
    };
    if !(p_val).is_null() && unsafe { sqlite3_value_int(p_val) } > 0 {
        ret = 1;
    }
    unsafe { sqlite3ValueFree(p_val) };
    return ret;
}

///* Return true if the current frame should be cached in the ephemeral table,
///* even if there are no xInverse() calls required.
extern "C" fn window_cache_frame(p_m_win_1: *mut Window) -> i32 {
    let mut p_win: *const Window = core::ptr::null();
    if unsafe { (*p_m_win_1).reg_start_rowid } != 0 { return 1; }
    {
        p_win = p_m_win_1;
        '__b3: loop {
            if !(!(p_win).is_null()) { break '__b3; }
            '__c3: loop {
                let p_func: *const FuncDef =
                    unsafe { (*p_win).p_w_func } as *const FuncDef;
                if unsafe { (*p_func).z_name } ==
                                    &raw const nth_value_name[0 as usize] as *const i8 ||
                                unsafe { (*p_func).z_name } ==
                                    &raw const first_value_name[0 as usize] as *const i8 ||
                            unsafe { (*p_func).z_name } ==
                                &raw const lead_name[0 as usize] as *const i8 ||
                        unsafe { (*p_func).z_name } ==
                            &raw const lag_name[0 as usize] as *const i8 {
                    return 1;
                }
                break '__c3;
            }
            p_win = unsafe { (*p_win).p_next_win };
        }
    }
    return 0;
}

///* Return the number of arguments passed to the window-function associated
///* with the object passed as the only argument to this function.
extern "C" fn window_arg_count(p_win_1: &Window) -> i32 {
    unsafe {
        let mut p_list: *const ExprList = core::ptr::null();
        { let _ = 0; };
        p_list = unsafe { (*(*p_win_1).p_owner).x.p_list } as *const ExprList;
        return if !(p_list).is_null() {
                unsafe { (*p_list).n_expr }
            } else { 0 };
    }
}

///* Generate code to set the accumulator register for each window function
///* in the linked list passed as the second argument to NULL. And perform
///* any equivalent initialization required by any built-in window functions
///* in the list.
extern "C" fn window_init_accum(p_parse_1: *mut Parse, p_m_win_1: *mut Window)
    -> i32 {
    let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse_1) };
    let mut reg_arg: i32 = 0;
    let mut n_arg: i32 = 0;
    let mut p_win: *mut Window = core::ptr::null_mut();
    {
        p_win = p_m_win_1;
        '__b4: loop {
            if !(!(p_win).is_null()) { break '__b4; }
            '__c4: loop {
                let p_func: *const FuncDef =
                    unsafe { (*p_win).p_w_func } as *const FuncDef;
                { let _ = 0; };
                unsafe {
                    sqlite3_vdbe_add_op2(v, 77, 0,
                        unsafe { (*p_win).reg_accum })
                };
                n_arg =
                    if n_arg > window_arg_count(unsafe { &*p_win }) {
                        n_arg
                    } else { window_arg_count(unsafe { &*p_win }) };
                if unsafe { (*p_m_win_1).reg_start_rowid } == 0 {
                    if unsafe { (*p_func).z_name } ==
                                &raw const nth_value_name[0 as usize] as *const i8 ||
                            unsafe { (*p_func).z_name } ==
                                &raw const first_value_name[0 as usize] as *const i8 {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 0, unsafe { (*p_win).reg_app })
                        };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 0,
                                unsafe { (*p_win).reg_app } + 1)
                        };
                    }
                    if unsafe { (*p_func).func_flags } & 4096 as u32 != 0 &&
                            unsafe { (*p_win).csr_app } != 0 {
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op1(v, 148, unsafe { (*p_win).csr_app })
                        };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 73, 0,
                                unsafe { (*p_win).reg_app } + 1)
                        };
                    }
                }
                break '__c4;
            }
            p_win = unsafe { (*p_win).p_next_win };
        }
    }
    reg_arg = unsafe { (*p_parse_1).n_mem } + 1;
    unsafe { (*p_parse_1).n_mem += n_arg };
    return reg_arg;
}

///* A "PRECEDING <expr>" (eCond==0) or "FOLLOWING <expr>" (eCond==1) or the
///* value of the second argument to nth_value() (eCond==2) has just been
///* evaluated and the result left in register reg. This function generates VM
///* code to check that the value is a non-negative integer and throws an
///* exception if it is not.
#[allow(unused_doc_comments)]
extern "C" fn window_check_value(p_parse_1: *mut Parse, reg: i32,
    e_cond_1: i32) -> () {
    unsafe {
        let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse_1) };
        let reg_zero: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
        { let _ = 0; };
        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_zero) };
        if e_cond_1 >= 3 {
            let reg_string: i32 = unsafe { sqlite3_get_temp_reg(p_parse_1) };
            unsafe {
                sqlite3_vdbe_add_op4(v, 118, 0, reg_string, 0,
                    c"".as_ptr() as *mut i8 as *const i8, -1)
            };
            unsafe {
                sqlite3_vdbe_add_op3(v, 58, reg_string,
                    unsafe { sqlite3_vdbe_current_addr(v) } + 2, reg)
            };
            unsafe { sqlite3_vdbe_change_p5(v, (67 | 16) as u16) };
            { let _ = 0; };
        } else {
            unsafe {
                sqlite3_vdbe_add_op2(v, 13, reg,
                    unsafe { sqlite3_vdbe_current_addr(v) } + 2)
            };
            { let _ = 0; };
        }
        unsafe {
            sqlite3_vdbe_add_op3(v, a_op[e_cond_1 as usize], reg_zero,
                unsafe { sqlite3_vdbe_current_addr(v) } + 2, reg)
        };
        unsafe { sqlite3_vdbe_change_p5(v, 67 as u16) };

        ///   the OP_Ge
        unsafe { sqlite3_may_abort(p_parse_1) };
        unsafe { sqlite3_vdbe_add_op2(v, 72, 1, 2) };
        unsafe {
            sqlite3_vdbe_append_p4(v, az_err[e_cond_1 as usize] as *mut (),
                -1)
        };
        unsafe { sqlite3_release_temp_reg(p_parse_1, reg_zero) };
    }
}

///* Generate VM code to invoke either xValue() (bFin==0) or xFinalize()
///* (bFin==1) for each window function in the linked list starting at
///* pMWin. Or, for built-in window-functions that do not use the standard
///* API, generate the equivalent VM code.
extern "C" fn window_agg_final(p: &WindowCodeArg, b_fin_1: i32) -> () {
    let p_parse: *mut Parse = (*p).p_parse;
    let p_m_win: *mut Window = (*p).p_m_win;
    let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };
    let mut p_win: *mut Window = core::ptr::null_mut();
    {
        p_win = p_m_win;
        '__b5: loop {
            if !(!(p_win).is_null()) { break '__b5; }
            '__c5: loop {
                if unsafe { (*p_m_win).reg_start_rowid } == 0 &&
                            unsafe { (*unsafe { (*p_win).p_w_func }).func_flags } &
                                    4096 as u32 != 0 && unsafe { (*p_win).e_start } as i32 != 91
                    {
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 77, 0,
                            unsafe { (*p_win).reg_result })
                    };
                    unsafe {
                        sqlite3_vdbe_add_op1(v, 32, unsafe { (*p_win).csr_app })
                    };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 96, unsafe { (*p_win).csr_app }, 0,
                            unsafe { (*p_win).reg_result })
                    };
                    unsafe {
                        sqlite3_vdbe_jump_here(v,
                            unsafe { sqlite3_vdbe_current_addr(v) } - 2)
                    };
                } else if unsafe { (*p_win).reg_app } != 0 {
                    { let _ = 0; };
                } else {
                    let n_arg: i32 = window_arg_count(unsafe { &*p_win });
                    if b_fin_1 != 0 {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 167, unsafe { (*p_win).reg_accum },
                                n_arg)
                        };
                        unsafe {
                            sqlite3_vdbe_append_p4(v,
                                unsafe { (*p_win).p_w_func } as *mut (), -8)
                        };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 82, unsafe { (*p_win).reg_accum },
                                unsafe { (*p_win).reg_result })
                        };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 77, 0,
                                unsafe { (*p_win).reg_accum })
                        };
                    } else {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 166, unsafe { (*p_win).reg_accum },
                                n_arg, unsafe { (*p_win).reg_result })
                        };
                        unsafe {
                            sqlite3_vdbe_append_p4(v,
                                unsafe { (*p_win).p_w_func } as *mut (), -8)
                        };
                    }
                }
                break '__c5;
            }
            p_win = unsafe { (*p_win).p_next_win };
        }
    }
}

///* Generate VM code to read the window frames peer values from cursor csr into
///* an array of registers starting at reg.
extern "C" fn window_read_peer_values(p: &WindowCodeArg, csr: i32, reg: i32)
    -> () {
    let p_m_win: *const Window = (*p).p_m_win as *const Window;
    let p_order_by: *const ExprList =
        unsafe { (*p_m_win).p_order_by } as *const ExprList;
    if !(p_order_by).is_null() {
        let v: *mut Vdbe = unsafe { sqlite3_get_vdbe((*p).p_parse) };
        let p_part: *const ExprList =
            unsafe { (*p_m_win).p_partition } as *const ExprList;
        let i_col_off: i32 =
            unsafe { (*p_m_win).n_buffer_col } +
                if !(p_part).is_null() {
                    unsafe { (*p_part).n_expr }
                } else { 0 };
        let mut i: i32 = 0;
        {
            i = 0;
            '__b6: loop {
                if !(i < unsafe { (*p_order_by).n_expr }) { break '__b6; }
                '__c6: loop {
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 96, csr, i_col_off + i, reg + i)
                    };
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

///* No-op implementations of xStep() and xFinalize().  Used as place-holders
///* for built-in window functions that never call those interfaces.
///*
///* The noopValueFunc() is called but is expected to do nothing.  The
///* noopStepFunc() is never called, and so it is marked with NO_TEST to
///* let the test coverage routine know not to expect this function to be
///* invoked.
#[allow(unused_doc_comments)]
extern "C" fn noop_step_func(p: *mut Sqlite3Context, n: i32,
    a: *mut *mut Sqlite3Value) -> () {

    ///NO_TEST
    { let _ = p; };

    ///NO_TEST
    ///NO_TEST
    { let _ = n; };

    ///NO_TEST
    ///NO_TEST
    ///NO_TEST
    { let _ = a; };

    ///NO_TEST
    ///NO_TEST
    ///NO_TEST
    ///NO_TEST
    { let _ = 0; };
}

///* Generate VM code to invoke either xStep() (if bInverse is 0) or
///* xInverse (if bInverse is non-zero) for each window function in the
///* linked list starting at pMWin. Or, for built-in window functions
///* that do not use the standard function API, generate the required
///* inline VM code.
///*
///* If argument csr is greater than or equal to 0, then argument reg is
///* the first register in an array of registers guaranteed to be large
///* enough to hold the array of arguments for each function. In this case
///* the arguments are extracted from the current row of csr into the
///* array of registers before invoking OP_AggStep or OP_AggInverse
///*
///* Or, if csr is less than zero, then the array of registers at reg is
///* already populated with all columns from the current row of the sub-query.
///*
///* If argument regPartSize is non-zero, then it is a register containing the
///* number of rows in the current partition.
#[allow(unused_doc_comments)]
extern "C" fn window_agg_step(p: &WindowCodeArg, p_m_win_1: *mut Window,
    csr: i32, b_inverse_1: i32, reg: i32) -> () {
    unsafe {
        let p_parse: *mut Parse = (*p).p_parse;
        let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };
        let mut p_win: *mut Window = core::ptr::null_mut();
        {
            p_win = p_m_win_1;
            '__b7: loop {
                if !(!(p_win).is_null()) { break '__b7; }
                '__c7: loop {
                    let p_func: *mut FuncDef = unsafe { (*p_win).p_w_func };
                    let mut reg_arg: i32 = 0;
                    let mut n_arg: i32 =
                        if unsafe { (*p_win).b_expr_args } != 0 {
                            0
                        } else { window_arg_count(unsafe { &*p_win }) };
                    let mut i: i32 = 0;
                    let mut addr_if: i32 = 0;
                    { let _ = 0; };

                    /// All OVER clauses in the same window function aggregate step must
                    ///* be the same.
                    { let _ = 0; };
                    {
                        i = 0;
                        '__b8: loop {
                            if !(i < n_arg) { break '__b8; }
                            '__c8: loop {
                                if i != 1 ||
                                        unsafe { (*p_func).z_name } !=
                                            &raw const nth_value_name[0 as usize] as *const i8 {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 96, csr,
                                            unsafe { (*p_win).i_arg_col } + i, reg + i)
                                    };
                                } else {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 96,
                                            unsafe { (*p_m_win_1).i_eph_csr },
                                            unsafe { (*p_win).i_arg_col } + i, reg + i)
                                    };
                                }
                                break '__c8;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    reg_arg = reg;
                    if !(unsafe { (*p_win).p_filter }).is_null() {
                        let mut reg_tmp: i32 = 0;
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        reg_tmp = unsafe { sqlite3_get_temp_reg(p_parse) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, csr,
                                unsafe { (*p_win).i_arg_col } + n_arg, reg_tmp)
                        };
                        addr_if =
                            unsafe { sqlite3_vdbe_add_op3(v, 17, reg_tmp, 0, 1) };
                        unsafe { sqlite3_release_temp_reg(p_parse, reg_tmp) };
                    }
                    if unsafe { (*p_m_win_1).reg_start_rowid } == 0 &&
                                unsafe { (*p_func).func_flags } & 4096 as u32 != 0 &&
                            unsafe { (*p_win).e_start } as i32 != 91 {
                        let addr_is_null: i32 =
                            unsafe { sqlite3_vdbe_add_op1(v, 51, reg_arg) };
                        if b_inverse_1 == 0 {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 88, unsafe { (*p_win).reg_app } + 1,
                                    1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 83, reg_arg,
                                    unsafe { (*p_win).reg_app })
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 99, unsafe { (*p_win).reg_app }, 2,
                                    unsafe { (*p_win).reg_app } + 2)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 140, unsafe { (*p_win).csr_app },
                                    unsafe { (*p_win).reg_app } + 2)
                            };
                        } else {
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 23, unsafe { (*p_win).csr_app },
                                    0, reg_arg, 1)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op1(v, 132, unsafe { (*p_win).csr_app })
                            };
                            unsafe {
                                sqlite3_vdbe_jump_here(v,
                                    unsafe { sqlite3_vdbe_current_addr(v) } - 2)
                            };
                        }
                        unsafe { sqlite3_vdbe_jump_here(v, addr_is_null) };
                    } else if unsafe { (*p_win).reg_app } != 0 {
                        { let _ = 0; };
                        { let _ = 0; };
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 88,
                                unsafe { (*p_win).reg_app } + 1 - b_inverse_1, 1)
                        };
                    } else if unsafe { (*p_func).x_s_func } !=
                            Some(noop_step_func) {
                        if unsafe { (*p_win).b_expr_args } != 0 {
                            let mut i_op: i32 = unsafe { sqlite3_vdbe_current_addr(v) };
                            let mut i_end: i32 = 0;
                            { let _ = 0; };
                            n_arg =
                                unsafe {
                                    (*unsafe { (*unsafe { (*p_win).p_owner }).x.p_list }).n_expr
                                };
                            reg_arg = unsafe { sqlite3_get_temp_range(p_parse, n_arg) };
                            unsafe {
                                sqlite3_expr_code_expr_list(p_parse,
                                    unsafe { (*unsafe { (*p_win).p_owner }).x.p_list }, reg_arg,
                                    0, 0 as u8)
                            };
                            {
                                i_end = unsafe { sqlite3_vdbe_current_addr(v) };
                                '__b9: loop {
                                    if !(i_op < i_end) { break '__b9; }
                                    '__c9: loop {
                                        let p_op: *mut VdbeOp =
                                            unsafe { sqlite3_vdbe_get_op(v, i_op) };
                                        if unsafe { (*p_op).opcode } as i32 == 96 &&
                                                unsafe { (*p_op).p1 } == unsafe { (*p_m_win_1).i_eph_csr } {
                                            unsafe { (*p_op).p1 = csr };
                                        }
                                        break '__c9;
                                    }
                                    { let __p = &mut i_op; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if unsafe { (*p_func).func_flags } & 32 as u32 != 0 {
                            let mut p_coll: *const CollSeq = core::ptr::null();
                            { let _ = 0; };
                            { let _ = 0; };
                            p_coll =
                                unsafe {
                                    sqlite3_expr_nn_coll_seq(p_parse,
                                        unsafe {
                                                (*(unsafe {
                                                                    (*unsafe {
                                                                                        (*unsafe { (*p_win).p_owner }).x.p_list
                                                                                    }).a.as_ptr()
                                                                } as *mut ExprListItem).offset(0 as isize)).p_expr
                                            } as *const Expr)
                                };
                            unsafe {
                                sqlite3_vdbe_add_op4(v, 87, 0, 0, 0, p_coll as *const i8,
                                    -2)
                            };
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v,
                                if b_inverse_1 != 0 { 163 } else { 164 }, b_inverse_1,
                                reg_arg, unsafe { (*p_win).reg_accum })
                        };
                        unsafe { sqlite3_vdbe_append_p4(v, p_func as *mut (), -8) };
                        unsafe { sqlite3_vdbe_change_p5(v, n_arg as u16) };
                        if unsafe { (*p_win).b_expr_args } != 0 {
                            unsafe {
                                sqlite3_release_temp_range(p_parse, reg_arg, n_arg)
                            };
                        }
                    }
                    if addr_if != 0 {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_if) };
                    }
                    break '__c7;
                }
                p_win = unsafe { (*p_win).p_next_win };
            }
        }
    }
}

///* Generate code to calculate the current values of all window functions in the
///* p->pMWin list by doing a full scan of the current window frame. Store the
///* results in the Window.regResult registers, ready to return the upper
///* layer.
#[allow(unused_doc_comments)]
extern "C" fn window_full_scan(p: *mut WindowCodeArg) -> () {
    let mut p_win: *const Window = core::ptr::null();
    let p_parse: *mut Parse = unsafe { (*p).p_parse };
    let p_m_win: *mut Window = unsafe { (*p).p_m_win };
    let v: *mut Vdbe = unsafe { (*p).p_vdbe };
    let mut reg_c_rowid: i32 = 0;
    /// Current rowid value
    let mut reg_c_peer: i32 = 0;
    /// Current peer values
    let mut reg_rowid: i32 = 0;
    /// AggStep rowid value
    let mut reg_peer: i32 = 0;
    /// AggStep peer values
    let mut n_peer: i32 = 0;
    let mut lbl_next: i32 = 0;
    let mut lbl_brk: i32 = 0;
    let mut addr_next: i32 = 0;
    let mut csr: i32 = 0;
    { let _ = 0; };
    csr = unsafe { (*p_m_win).csr_app };
    n_peer =
        if !(unsafe { (*p_m_win).p_order_by }).is_null() {
            unsafe { (*unsafe { (*p_m_win).p_order_by }).n_expr }
        } else { 0 };
    lbl_next = unsafe { sqlite3_vdbe_make_label(p_parse) };
    lbl_brk = unsafe { sqlite3_vdbe_make_label(p_parse) };
    reg_c_rowid = unsafe { sqlite3_get_temp_reg(p_parse) };
    reg_rowid = unsafe { sqlite3_get_temp_reg(p_parse) };
    if n_peer != 0 {
        reg_c_peer = unsafe { sqlite3_get_temp_range(p_parse, n_peer) };
        reg_peer = unsafe { sqlite3_get_temp_range(p_parse, n_peer) };
    }
    unsafe {
        sqlite3_vdbe_add_op2(v, 137, unsafe { (*p_m_win).i_eph_csr },
            reg_c_rowid)
    };
    window_read_peer_values(unsafe { &*p }, unsafe { (*p_m_win).i_eph_csr },
        reg_c_peer);
    {
        p_win = p_m_win;
        '__b10: loop {
            if !(!(p_win).is_null()) { break '__b10; }
            '__c10: loop {
                unsafe {
                    sqlite3_vdbe_add_op2(v, 77, 0,
                        unsafe { (*p_win).reg_accum })
                };
                break '__c10;
            }
            p_win = unsafe { (*p_win).p_next_win };
        }
    }
    unsafe {
        sqlite3_vdbe_add_op3(v, 23, csr, lbl_brk,
            unsafe { (*p_m_win).reg_start_rowid })
    };
    addr_next = unsafe { sqlite3_vdbe_current_addr(v) };
    unsafe { sqlite3_vdbe_add_op2(v, 137, csr, reg_rowid) };
    unsafe {
        sqlite3_vdbe_add_op3(v, 55, unsafe { (*p_m_win).reg_end_rowid },
            lbl_brk, reg_rowid)
    };
    if unsafe { (*p_m_win).e_exclude } as i32 == 86 {
        unsafe {
            sqlite3_vdbe_add_op3(v, 54, reg_c_rowid, lbl_next, reg_rowid)
        };
    } else if unsafe { (*p_m_win).e_exclude } as i32 != 67 {
        let mut addr: i32 = 0;
        let mut addr_eq: i32 = 0;
        let mut p_key_info: *mut KeyInfo = core::ptr::null_mut();
        if !(unsafe { (*p_m_win).p_order_by }).is_null() {
            p_key_info =
                unsafe {
                    sqlite3_key_info_from_expr_list(p_parse,
                        unsafe { (*p_m_win).p_order_by }, 0, 0)
                };
        }
        if unsafe { (*p_m_win).e_exclude } as i32 == 95 {
            addr_eq =
                unsafe {
                    sqlite3_vdbe_add_op3(v, 54, reg_c_rowid, 0, reg_rowid)
                };
        }
        if !(p_key_info).is_null() {
            window_read_peer_values(unsafe { &*p }, csr, reg_peer);
            unsafe {
                sqlite3_vdbe_add_op3(v, 92, reg_peer, reg_c_peer, n_peer)
            };
            unsafe { sqlite3_vdbe_append_p4(v, p_key_info as *mut (), -9) };
            addr = unsafe { sqlite3_vdbe_current_addr(v) } + 1;
            unsafe { sqlite3_vdbe_add_op3(v, 14, addr, lbl_next, addr) };
        } else { unsafe { sqlite3_vdbe_add_op2(v, 9, 0, lbl_next) }; }
        if addr_eq != 0 { unsafe { sqlite3_vdbe_jump_here(v, addr_eq) }; }
    }
    window_agg_step(unsafe { &*p }, p_m_win, csr, 0, unsafe { (*p).reg_arg });
    unsafe { sqlite3_vdbe_resolve_label(v, lbl_next) };
    unsafe { sqlite3_vdbe_add_op2(v, 40, csr, addr_next) };
    unsafe { sqlite3_vdbe_jump_here(v, addr_next - 1) };
    unsafe { sqlite3_vdbe_jump_here(v, addr_next + 1) };
    unsafe { sqlite3_release_temp_reg(p_parse, reg_rowid) };
    unsafe { sqlite3_release_temp_reg(p_parse, reg_c_rowid) };
    if n_peer != 0 {
        unsafe { sqlite3_release_temp_range(p_parse, reg_peer, n_peer) };
        unsafe { sqlite3_release_temp_range(p_parse, reg_c_peer, n_peer) };
    }
    window_agg_final(unsafe { &*p }, 1);
}

///* Invoke the sub-routine at regGosub (generated by code in select.c) to
///* return the current row of Window.iEphCsr. If all window functions are
///* aggregate window functions that use the standard API, a single
///* OP_Gosub instruction is all that this routine generates. Extra VM code
///* for per-row processing is only generated for the following built-in window
///* functions:
///*
///*   nth_value()
///*   first_value()
///*   lag()
///*   lead()
extern "C" fn window_return_one_row(p: *mut WindowCodeArg) -> () {
    unsafe {
        let p_m_win: *mut Window = unsafe { (*p).p_m_win };
        let v: *mut Vdbe = unsafe { (*p).p_vdbe };
        if unsafe { (*p_m_win).reg_start_rowid } != 0 {
            window_full_scan(p);
        } else {
            let p_parse: *mut Parse = unsafe { (*p).p_parse };
            let mut p_win: *const Window = core::ptr::null();
            {
                p_win = p_m_win;
                '__b11: loop {
                    if !(!(p_win).is_null()) { break '__b11; }
                    '__c11: loop {
                        let p_func: *const FuncDef =
                            unsafe { (*p_win).p_w_func } as *const FuncDef;
                        { let _ = 0; };
                        if unsafe { (*p_func).z_name } ==
                                    &raw const nth_value_name[0 as usize] as *const i8 ||
                                unsafe { (*p_func).z_name } ==
                                    &raw const first_value_name[0 as usize] as *const i8 {
                            let csr: i32 = unsafe { (*p_win).csr_app };
                            let lbl: i32 = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            let tmp_reg: i32 = unsafe { sqlite3_get_temp_reg(p_parse) };
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 77, 0,
                                    unsafe { (*p_win).reg_result })
                            };
                            if unsafe { (*p_func).z_name } ==
                                    &raw const nth_value_name[0 as usize] as *const i8 {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 96, unsafe { (*p_m_win).i_eph_csr },
                                        unsafe { (*p_win).i_arg_col } + 1, tmp_reg)
                                };
                                window_check_value(p_parse, tmp_reg, 2);
                            } else {
                                unsafe { sqlite3_vdbe_add_op2(v, 73, 1, tmp_reg) };
                            }
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 107, tmp_reg,
                                    unsafe { (*p_win).reg_app }, tmp_reg)
                            };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 55, unsafe { (*p_win).reg_app } + 1,
                                    lbl, tmp_reg)
                            };
                            unsafe { sqlite3_vdbe_add_op3(v, 30, csr, 0, tmp_reg) };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 96, csr,
                                    unsafe { (*p_win).i_arg_col },
                                    unsafe { (*p_win).reg_result })
                            };
                            unsafe { sqlite3_vdbe_resolve_label(v, lbl) };
                            unsafe { sqlite3_release_temp_reg(p_parse, tmp_reg) };
                        } else if unsafe { (*p_func).z_name } ==
                                    &raw const lead_name[0 as usize] as *const i8 ||
                                unsafe { (*p_func).z_name } ==
                                    &raw const lag_name[0 as usize] as *const i8 {
                            let n_arg: i32 =
                                unsafe {
                                    (*unsafe { (*unsafe { (*p_win).p_owner }).x.p_list }).n_expr
                                };
                            let csr: i32 = unsafe { (*p_win).csr_app };
                            let lbl: i32 = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            let tmp_reg_1: i32 =
                                unsafe { sqlite3_get_temp_reg(p_parse) };
                            let i_eph: i32 = unsafe { (*p_m_win).i_eph_csr };
                            if n_arg < 3 {
                                unsafe {
                                    sqlite3_vdbe_add_op2(v, 77, 0,
                                        unsafe { (*p_win).reg_result })
                                };
                            } else {
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 96, i_eph,
                                        unsafe { (*p_win).i_arg_col } + 2,
                                        unsafe { (*p_win).reg_result })
                                };
                            }
                            unsafe { sqlite3_vdbe_add_op2(v, 137, i_eph, tmp_reg_1) };
                            if n_arg < 2 {
                                let val: i32 =
                                    if unsafe { (*p_func).z_name } ==
                                            &raw const lead_name[0 as usize] as *const i8 {
                                        1
                                    } else { -1 };
                                unsafe { sqlite3_vdbe_add_op2(v, 88, tmp_reg_1, val) };
                            } else {
                                let op: i32 =
                                    if unsafe { (*p_func).z_name } ==
                                            &raw const lead_name[0 as usize] as *const i8 {
                                        107
                                    } else { 108 };
                                let tmp_reg2: i32 =
                                    unsafe { sqlite3_get_temp_reg(p_parse) };
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, 96, i_eph,
                                        unsafe { (*p_win).i_arg_col } + 1, tmp_reg2)
                                };
                                unsafe {
                                    sqlite3_vdbe_add_op3(v, op, tmp_reg2, tmp_reg_1, tmp_reg_1)
                                };
                                unsafe { sqlite3_release_temp_reg(p_parse, tmp_reg2) };
                            }
                            unsafe { sqlite3_vdbe_add_op3(v, 30, csr, lbl, tmp_reg_1) };
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 96, csr,
                                    unsafe { (*p_win).i_arg_col },
                                    unsafe { (*p_win).reg_result })
                            };
                            unsafe { sqlite3_vdbe_resolve_label(v, lbl) };
                            unsafe { sqlite3_release_temp_reg(p_parse, tmp_reg_1) };
                        }
                        break '__c11;
                    }
                    p_win = unsafe { (*p_win).p_next_win };
                }
            }
        }
        unsafe {
            sqlite3_vdbe_add_op2(v, 10, unsafe { (*p).reg_gosub },
                unsafe { (*p).addr_gosub })
        };
    }
}

///* regOld and regNew are each the first register in an array of size
///* pOrderBy->nExpr. This function generates code to compare the two
///* arrays of registers using the collation sequences and other comparison
///* parameters specified by pOrderBy.
///*
///* If the two arrays are not equal, the contents of regNew is copied to
///* regOld and control falls through. Otherwise, if the contents of the arrays
///* are equal, an OP_Goto is executed. The address of the OP_Goto is returned.
extern "C" fn window_if_new_peer(p_parse_1: *mut Parse,
    p_order_by_1: *mut ExprList, reg_new_1: i32, reg_old_1: i32, addr: i32)
    -> () {
    let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse_1) };
    if !(p_order_by_1).is_null() {
        let n_val: i32 = unsafe { (*p_order_by_1).n_expr };
        let p_key_info: *mut KeyInfo =
            unsafe {
                sqlite3_key_info_from_expr_list(p_parse_1, p_order_by_1, 0, 0)
            };
        unsafe { sqlite3_vdbe_add_op3(v, 92, reg_old_1, reg_new_1, n_val) };
        unsafe { sqlite3_vdbe_append_p4(v, p_key_info as *mut (), -9) };
        unsafe {
            sqlite3_vdbe_add_op3(v, 14,
                unsafe { sqlite3_vdbe_current_addr(v) } + 1, addr,
                unsafe { sqlite3_vdbe_current_addr(v) } + 1)
        };
        unsafe {
            sqlite3_vdbe_add_op3(v, 82, reg_new_1, reg_old_1, n_val - 1)
        };
    } else { unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr) }; }
}

///* This function is called as part of generating VM programs for RANGE
///* offset PRECEDING/FOLLOWING frame boundaries. Assuming "ASC" order for
///* the ORDER BY term in the window, and that argument op is OP_Ge, it generates
///* code equivalent to:
///*
///*   if( csr1.peerVal + regVal >= csr2.peerVal ) goto lbl;
///*
///* The value of parameter op may also be OP_Gt or OP_Le. In these cases the
///* operator in the above pseudo-code is replaced with ">" or "<=", respectively.
///*
///* If the sort-order for the ORDER BY term in the window is DESC, then the
///* comparison is reversed. Instead of adding regVal to csr1.peerVal, it is
///* subtracted. And the comparison operator is inverted to - ">=" becomes "<=",
///* ">" becomes "<", and so on. So, with DESC sort order, if the argument op
///* is OP_Ge, the generated code is equivalent to:
///*
///*   if( csr1.peerVal - regVal <= csr2.peerVal ) goto lbl;
///*
///* A special type of arithmetic is used such that if csr1.peerVal is not
///* a numeric type (real or integer), then the result of the addition
///* or subtraction is a a copy of csr1.peerVal.
#[allow(unused_doc_comments)]
extern "C" fn window_code_range_test(p: *mut WindowCodeArg, mut op: i32,
    csr1: i32, reg_val_1: i32, csr2: i32, lbl: i32) -> () {
    let p_parse: *mut Parse = unsafe { (*p).p_parse };
    let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };
    let p_order_by: *const ExprList =
        unsafe { (*unsafe { (*p).p_m_win }).p_order_by } as *const ExprList;
    /// ORDER BY clause for window
    let reg1: i32 = unsafe { sqlite3_get_temp_reg(p_parse) };
    /// Reg. for csr1.peerVal+regVal
    let reg2: i32 = unsafe { sqlite3_get_temp_reg(p_parse) };
    /// Reg. for csr2.peerVal
    let reg_string: i32 =
        { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
    /// Reg. for constant value ''
    let mut arith: i32 = 107;
    /// OP_Add or OP_Subtract
    let mut addr_ge: i32 = 0;
    /// Jump destination
    let addr_done: i32 = unsafe { sqlite3_vdbe_make_label(p_parse) };
    /// Address past OP_Ge
    let mut p_coll: *mut CollSeq = core::ptr::null_mut();

    /// Read the peer-value from each cursor into a register
    window_read_peer_values(unsafe { &*p }, csr1, reg1);
    window_read_peer_values(unsafe { &*p }, csr2, reg2);
    { let _ = 0; };
    { let _ = 0; };
    if unsafe {
                        (*(unsafe { (*p_order_by).a.as_ptr() } as
                                            *mut ExprListItem).offset(0 as isize)).fg.sort_flags
                    } as i32 & 1 != 0 {
        '__s12:
            {
            match op {
                58 => { op = 56; }
                55 => { op = 57; }
                _ => { { let _ = 0; }; op = 58; }
            }
        }
        arith = 108;
    }
    if unsafe {
                        (*(unsafe { (*p_order_by).a.as_ptr() } as
                                            *mut ExprListItem).offset(0 as isize)).fg.sort_flags
                    } as i32 & 2 != 0 {
        /// This block runs if reg1 contains a NULL.
        let addr: i32 = unsafe { sqlite3_vdbe_add_op1(v, 52, reg1) };
        '__s13:
            {
            match op {
                58 => { unsafe { sqlite3_vdbe_add_op2(v, 9, 0, lbl) }; }
                55 => { unsafe { sqlite3_vdbe_add_op2(v, 52, reg2, lbl) }; }
                56 => { unsafe { sqlite3_vdbe_add_op2(v, 51, reg2, lbl) }; }
                _ => { { let _ = 0; }; }
            }
        }
        unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr_done) };

        /// This block runs if reg1 is not NULL, but reg2 is.
        unsafe { sqlite3_vdbe_jump_here(v, addr) };
        unsafe {
            sqlite3_vdbe_add_op2(v, 51, reg2,
                if op == 55 || op == 58 { addr_done } else { lbl })
        };
    }

    /// Register reg1 currently contains csr1.peerVal (the peer-value from csr1).
    ///* This block adds (or subtracts for DESC) the numeric value in regVal
    ///* from it. Or, if reg1 is not numeric (it is a NULL, a text value or a blob),
    ///* then leave reg1 as it is. In pseudo-code, this is implemented as:
    ///*
    ///*   if( reg1>='' ) goto addrGe;
    ///*   reg1 = reg1 +/- regVal
    ///*   addrGe:
    ///*
    ///* Since all strings and blobs are greater-than-or-equal-to an empty string,
    ///* the add/subtract is skipped for these, as required. If reg1 is a NULL,
    ///* then the arithmetic is performed, but since adding or subtracting from
    ///* NULL is always NULL anyway, this case is handled as required too.
    unsafe {
        sqlite3_vdbe_add_op4(v, 118, 0, reg_string, 0,
            c"".as_ptr() as *mut i8 as *const i8, -1)
    };
    addr_ge = unsafe { sqlite3_vdbe_add_op3(v, 58, reg_string, 0, reg1) };
    if op == 58 && arith == 107 || op == 56 && arith == 108 {
        unsafe { sqlite3_vdbe_add_op3(v, op, reg2, lbl, reg1) };
    }
    unsafe { sqlite3_vdbe_add_op3(v, arith, reg_val_1, reg1, reg1) };
    unsafe { sqlite3_vdbe_jump_here(v, addr_ge) };

    /// Compare registers reg2 and reg1, taking the jump if required. Note that
    ///* control skips over this test if the BIGNULL flag is set and either
    ///* reg1 or reg2 contain a NULL value.
    unsafe { sqlite3_vdbe_add_op3(v, op, reg2, lbl, reg1) };
    p_coll =
        unsafe {
            sqlite3_expr_nn_coll_seq(p_parse,
                unsafe {
                        (*(unsafe { (*p_order_by).a.as_ptr() } as
                                        *mut ExprListItem).offset(0 as isize)).p_expr
                    } as *const Expr)
        };
    unsafe { sqlite3_vdbe_append_p4(v, p_coll as *mut (), -2) };
    unsafe { sqlite3_vdbe_change_p5(v, 128 as u16) };
    unsafe { sqlite3_vdbe_resolve_label(v, addr_done) };
    { let _ = 0; };
    unsafe { sqlite3_release_temp_reg(p_parse, reg1) };
    unsafe { sqlite3_release_temp_reg(p_parse, reg2) };
}

///* Helper function for sqlite3WindowCodeStep(). Each call to this function
///* generates VM code for a single RETURN_ROW, AGGSTEP or AGGINVERSE
///* operation. Refer to the header comment for sqlite3WindowCodeStep() for
///* details.
extern "C" fn window_code_op(p: *mut WindowCodeArg, op: i32,
    reg_countdown_1: i32, jump_on_eof_1: i32) -> i32 {
    let mut csr: i32 = 0;
    let mut reg: i32 = 0;
    let p_parse: *mut Parse = unsafe { (*p).p_parse };
    let p_m_win: *mut Window = unsafe { (*p).p_m_win };
    let mut ret: i32 = 0;
    let v: *mut Vdbe = unsafe { (*p).p_vdbe };
    let mut addr_continue: i32 = 0;
    let b_peer: i32 = (unsafe { (*p_m_win).e_frm_type } as i32 != 77) as i32;
    let lbl_done: i32 = unsafe { sqlite3_vdbe_make_label(p_parse) };
    let mut addr_next_range: i32 = 0;
    if op == 2 && unsafe { (*p_m_win).e_start } as i32 == 91 {
        { let _ = 0; };
        return 0;
    }
    if reg_countdown_1 > 0 {
        if unsafe { (*p_m_win).e_frm_type } as i32 == 90 {
            addr_next_range = unsafe { sqlite3_vdbe_current_addr(v) };
            { let _ = 0; };
            if op == 2 {
                if unsafe { (*p_m_win).e_start } as i32 == 87 {
                    window_code_range_test(p, 56, unsafe { (*p).current.csr },
                        reg_countdown_1, unsafe { (*p).start.csr }, lbl_done);
                } else {
                    window_code_range_test(p, 58, unsafe { (*p).start.csr },
                        reg_countdown_1, unsafe { (*p).current.csr }, lbl_done);
                }
            } else {
                window_code_range_test(p, 55, unsafe { (*p).end.csr },
                    reg_countdown_1, unsafe { (*p).current.csr }, lbl_done);
            }
        } else {
            unsafe {
                sqlite3_vdbe_add_op3(v, 61, reg_countdown_1, lbl_done, 1)
            };
        }
    }
    if op == 1 && unsafe { (*p_m_win).reg_start_rowid } == 0 {
        window_agg_final(unsafe { &*p }, 0);
    }
    addr_continue = unsafe { sqlite3_vdbe_current_addr(v) };
    if unsafe { (*p_m_win).e_start } as i32 ==
                    unsafe { (*p_m_win).e_end } as i32 && reg_countdown_1 != 0
            && unsafe { (*p_m_win).e_frm_type } as i32 == 90 {
        let reg_rowid1: i32 = unsafe { sqlite3_get_temp_reg(p_parse) };
        let reg_rowid2: i32 = unsafe { sqlite3_get_temp_reg(p_parse) };
        if op == 2 {
            unsafe {
                sqlite3_vdbe_add_op2(v, 137, unsafe { (*p).start.csr },
                    reg_rowid1)
            };
            unsafe {
                sqlite3_vdbe_add_op2(v, 137, unsafe { (*p).end.csr },
                    reg_rowid2)
            };
            unsafe {
                sqlite3_vdbe_add_op3(v, 58, reg_rowid2, lbl_done, reg_rowid1)
            };
        } else if unsafe { (*p).reg_rowid } != 0 {
            unsafe {
                sqlite3_vdbe_add_op2(v, 137, unsafe { (*p).end.csr },
                    reg_rowid1)
            };
            unsafe {
                sqlite3_vdbe_add_op3(v, 58, unsafe { (*p).reg_rowid },
                    lbl_done, reg_rowid1)
            };
        }
        unsafe { sqlite3_release_temp_reg(p_parse, reg_rowid1) };
        unsafe { sqlite3_release_temp_reg(p_parse, reg_rowid2) };
        { let _ = 0; };
    }
    '__s14:
        {
        match op {
            1 => {
                csr = unsafe { (*p).current.csr };
                reg = unsafe { (*p).current.reg };
                window_return_one_row(p);
            }
            2 => {
                csr = unsafe { (*p).start.csr };
                reg = unsafe { (*p).start.reg };
                if unsafe { (*p_m_win).reg_start_rowid } != 0 {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 88,
                            unsafe { (*p_m_win).reg_start_rowid }, 1)
                    };
                } else {
                    window_agg_step(unsafe { &*p }, p_m_win, csr, 1,
                        unsafe { (*p).reg_arg });
                }
            }
            _ => {
                { let _ = 0; };
                csr = unsafe { (*p).end.csr };
                reg = unsafe { (*p).end.reg };
                if unsafe { (*p_m_win).reg_start_rowid } != 0 {
                    { let _ = 0; };
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 88,
                            unsafe { (*p_m_win).reg_end_rowid }, 1)
                    };
                } else {
                    window_agg_step(unsafe { &*p }, p_m_win, csr, 0,
                        unsafe { (*p).reg_arg });
                }
            }
        }
    }
    if op == unsafe { (*p).e_delete } {
        unsafe { sqlite3_vdbe_add_op1(v, 132, csr) };
        unsafe { sqlite3_vdbe_change_p5(v, 2 as u16) };
    }
    if jump_on_eof_1 != 0 {
        unsafe {
            sqlite3_vdbe_add_op2(v, 40, csr,
                unsafe { sqlite3_vdbe_current_addr(v) } + 2)
        };
        ret = unsafe { sqlite3_vdbe_add_op0(v, 9) };
    } else {
        unsafe {
            sqlite3_vdbe_add_op2(v, 40, csr,
                unsafe { sqlite3_vdbe_current_addr(v) } + 1 + b_peer)
        };
        if b_peer != 0 { unsafe { sqlite3_vdbe_add_op2(v, 9, 0, lbl_done) }; }
    }
    if b_peer != 0 {
        let n_reg: i32 =
            if !(unsafe { (*p_m_win).p_order_by }).is_null() {
                unsafe { (*unsafe { (*p_m_win).p_order_by }).n_expr }
            } else { 0 };
        let reg_tmp: i32 =
            if n_reg != 0 {
                unsafe { sqlite3_get_temp_range(p_parse, n_reg) }
            } else { 0 };
        window_read_peer_values(unsafe { &*p }, csr, reg_tmp);
        window_if_new_peer(p_parse, unsafe { (*p_m_win).p_order_by }, reg_tmp,
            reg, addr_continue);
        unsafe { sqlite3_release_temp_range(p_parse, reg_tmp, n_reg) };
    }
    if addr_next_range != 0 {
        unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr_next_range) };
    }
    unsafe { sqlite3_vdbe_resolve_label(v, lbl_done) };
    return ret;
}

///* sqlite3WhereBegin() has already been called for the SELECT statement
///* passed as the second argument when this function is invoked. It generates
///* code to populate the Window.regResult register for each window function
///* and invoke the sub-routine at instruction addrGosub once for each row.
///* sqlite3WhereEnd() is always called before returning.
///*
///* This function handles several different types of window frames, which
///* require slightly different processing. The following pseudo code is
///* used to implement window frames of the form:
///*
///*   ROWS BETWEEN <expr1> PRECEDING AND <expr2> FOLLOWING
///*
///* Other window frame types use variants of the following:
///*
///*     ... loop started by sqlite3WhereBegin() ...
///*       if( new partition ){
///*         Gosub flush
///*       }
///*       Insert new row into eph table.
///*      
///*       if( first row of partition ){
///*         // Rewind three cursors, all open on the eph table.
///*         Rewind(csrEnd);
///*         Rewind(csrStart);
///*         Rewind(csrCurrent);
///*      
///*         regEnd = <expr2>          // FOLLOWING expression
///*         regStart = <expr1>        // PRECEDING expression
///*       }else{
///*         // First time this branch is taken, the eph table contains two
///*         // rows. The first row in the partition, which all three cursors
///*         // currently point to, and the following row.
///*         AGGSTEP
///*         if( (regEnd--)<=0 ){
///*           RETURN_ROW
///*           if( (regStart--)<=0 ){
///*             AGGINVERSE
///*           }
///*         }
///*       }
///*     }
///*     flush:
///*       AGGSTEP
///*       while( 1 ){
///*         RETURN ROW
///*         if( csrCurrent is EOF ) break;
///*         if( (regStart--)<=0 ){
///*           AggInverse(csrStart)
///*           Next(csrStart)
///*         }
///*       }
///*
///* The pseudo-code above uses the following shorthand:
///*
///*   AGGSTEP:    invoke the aggregate xStep() function for each window function
///*               with arguments read from the current row of cursor csrEnd, then
///*               step cursor csrEnd forward one row (i.e. sqlite3BtreeNext()).
///*
///*   RETURN_ROW: return a row to the caller based on the contents of the
///*               current row of csrCurrent and the current state of all
///*               aggregates. Then step cursor csrCurrent forward one row.
///*
///*   AGGINVERSE: invoke the aggregate xInverse() function for each window
///*               functions with arguments read from the current row of cursor
///*               csrStart. Then step csrStart forward one row.
///*
///* There are two other ROWS window frames that are handled significantly
///* differently from the above - "BETWEEN <expr> PRECEDING AND <expr> PRECEDING"
///* and "BETWEEN <expr> FOLLOWING AND <expr> FOLLOWING". These are special
///* cases because they change the order in which the three cursors (csrStart,
///* csrCurrent and csrEnd) iterate through the ephemeral table. Cases that
///* use UNBOUNDED or CURRENT ROW are much simpler variations on one of these
///* three.
///*
///*   ROWS BETWEEN <expr1> PRECEDING AND <expr2> PRECEDING
///*
///*     ... loop started by sqlite3WhereBegin() ...
///*       if( new partition ){
///*         Gosub flush
///*       }
///*       Insert new row into eph table.
///*       if( first row of partition ){
///*         Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
///*         regEnd = <expr2>
///*         regStart = <expr1>
///*       }else{
///*         if( (regEnd--)<=0 ){
///*           AGGSTEP
///*         }
///*         RETURN_ROW
///*         if( (regStart--)<=0 ){
///*           AGGINVERSE
///*         }
///*       }
///*     }
///*     flush:
///*       if( (regEnd--)<=0 ){
///*         AGGSTEP
///*       }
///*       RETURN_ROW
///*
///*
///*   ROWS BETWEEN <expr1> FOLLOWING AND <expr2> FOLLOWING
///*
///*   ... loop started by sqlite3WhereBegin() ...
///*     if( new partition ){
///*       Gosub flush
///*     }
///*     Insert new row into eph table.
///*     if( first row of partition ){
///*       Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
///*       regEnd = <expr2>
///*       regStart = regEnd - <expr1>
///*     }else{
///*       AGGSTEP
///*       if( (regEnd--)<=0 ){
///*         RETURN_ROW
///*       }
///*       if( (regStart--)<=0 ){
///*         AGGINVERSE
///*       }
///*     }
///*   }
///*   flush:
///*     AGGSTEP
///*     while( 1 ){
///*       if( (regEnd--)<=0 ){
///*         RETURN_ROW
///*         if( eof ) break;
///*       }
///*       if( (regStart--)<=0 ){
///*         AGGINVERSE
///*         if( eof ) break
///*       }
///*     }
///*     while( !eof csrCurrent ){
///*       RETURN_ROW
///*     }
///*
///* For the most part, the patterns above are adapted to support UNBOUNDED by
///* assuming that it is equivalent to "infinity PRECEDING/FOLLOWING" and
///* CURRENT ROW by assuming that it is equivalent to "0 PRECEDING/FOLLOWING".
///* This is optimized of course - branches that will never be taken and
///* conditions that are always true are omitted from the VM code. The only
///* exceptional case is:
///*
///*   ROWS BETWEEN <expr1> FOLLOWING AND UNBOUNDED FOLLOWING
///*
///*     ... loop started by sqlite3WhereBegin() ...
///*     if( new partition ){
///*       Gosub flush
///*     }
///*     Insert new row into eph table.
///*     if( first row of partition ){
///*       Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
///*       regStart = <expr1>
///*     }else{
///*       AGGSTEP
///*     }
///*   }
///*   flush:
///*     AGGSTEP
///*     while( 1 ){
///*       if( (regStart--)<=0 ){
///*         AGGINVERSE
///*         if( eof ) break
///*       }
///*       RETURN_ROW
///*     }
///*     while( !eof csrCurrent ){
///*       RETURN_ROW
///*     }
///*
///* Also requiring special handling are the cases:
///*
///*   ROWS BETWEEN <expr1> PRECEDING AND <expr2> PRECEDING
///*   ROWS BETWEEN <expr1> FOLLOWING AND <expr2> FOLLOWING
///*
///* when (expr1 < expr2). This is detected at runtime, not by this function.
///* To handle this case, the pseudo-code programs depicted above are modified
///* slightly to be:
///*
///*     ... loop started by sqlite3WhereBegin() ...
///*     if( new partition ){
///*       Gosub flush
///*     }
///*     Insert new row into eph table.
///*     if( first row of partition ){
///*       Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
///*       regEnd = <expr2>
///*       regStart = <expr1>
///*       if( regEnd < regStart ){
///*         RETURN_ROW
///*         delete eph table contents
///*         continue
///*       }
///*     ...
///*
///* The new "continue" statement in the above jumps to the next iteration
///* of the outer loop - the one started by sqlite3WhereBegin().
///*
///* The various GROUPS cases are implemented using the same patterns as
///* ROWS. The VM code is modified slightly so that:
///*
///*   1. The else branch in the main loop is only taken if the row just
///*      added to the ephemeral table is the start of a new group. In
///*      other words, it becomes:
///*
///*         ... loop started by sqlite3WhereBegin() ...
///*         if( new partition ){
///*           Gosub flush
///*         }
///*         Insert new row into eph table.
///*         if( first row of partition ){
///*           Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
///*           regEnd = <expr2>
///*           regStart = <expr1>
///*         }else if( new group ){
///*           ...
///*         }
///*       }
///*
///*   2. Instead of processing a single row, each RETURN_ROW, AGGSTEP or
///*      AGGINVERSE step processes the current row of the relevant cursor and
///*      all subsequent rows belonging to the same group.
///*
///* RANGE window frames are a little different again. As for GROUPS, the
///* main loop runs once per group only. And RETURN_ROW, AGGSTEP and AGGINVERSE
///* deal in groups instead of rows. As for ROWS and GROUPS, there are three
///* basic cases:
///*
///*   RANGE BETWEEN <expr1> PRECEDING AND <expr2> FOLLOWING
///*
///*     ... loop started by sqlite3WhereBegin() ...
///*       if( new partition ){
///*         Gosub flush
///*       }
///*       Insert new row into eph table.
///*       if( first row of partition ){
///*         Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
///*         regEnd = <expr2>
///*         regStart = <expr1>
///*       }else{
///*         AGGSTEP
///*         while( (csrCurrent.key + regEnd) < csrEnd.key ){
///*           RETURN_ROW
///*           while( csrStart.key + regStart) < csrCurrent.key ){
///*             AGGINVERSE
///*           }
///*         }
///*       }
///*     }
///*     flush:
///*       AGGSTEP
///*       while( 1 ){
///*         RETURN ROW
///*         if( csrCurrent is EOF ) break;
///*           while( csrStart.key + regStart) < csrCurrent.key ){
///*             AGGINVERSE
///*           }
///*         }
///*       }
///*
///* In the above notation, "csr.key" means the current value of the ORDER BY
///* expression (there is only ever 1 for a RANGE that uses an <expr> FOLLOWING
///* or <expr PRECEDING) read from cursor csr.
///*
///*   RANGE BETWEEN <expr1> PRECEDING AND <expr2> PRECEDING
///*
///*     ... loop started by sqlite3WhereBegin() ...
///*       if( new partition ){
///*         Gosub flush
///*       }
///*       Insert new row into eph table.
///*       if( first row of partition ){
///*         Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
///*         regEnd = <expr2>
///*         regStart = <expr1>
///*       }else{
///*         while( (csrEnd.key + regEnd) <= csrCurrent.key ){
///*           AGGSTEP
///*         }
///*         while( (csrStart.key + regStart) < csrCurrent.key ){
///*           AGGINVERSE
///*         }
///*         RETURN_ROW
///*       }
///*     }
///*     flush:
///*       while( (csrEnd.key + regEnd) <= csrCurrent.key ){
///*         AGGSTEP
///*       }
///*       while( (csrStart.key + regStart) < csrCurrent.key ){
///*         AGGINVERSE
///*       }
///*       RETURN_ROW
///*
///*   RANGE BETWEEN <expr1> FOLLOWING AND <expr2> FOLLOWING
///*
///*     ... loop started by sqlite3WhereBegin() ...
///*       if( new partition ){
///*         Gosub flush
///*       }
///*       Insert new row into eph table.
///*       if( first row of partition ){
///*         Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
///*         regEnd = <expr2>
///*         regStart = <expr1>
///*       }else{
///*         AGGSTEP
///*         while( (csrCurrent.key + regEnd) < csrEnd.key ){
///*           while( (csrCurrent.key + regStart) > csrStart.key ){
///*             AGGINVERSE
///*           }
///*           RETURN_ROW
///*         }
///*       }
///*     }
///*     flush:
///*       AGGSTEP
///*       while( 1 ){
///*         while( (csrCurrent.key + regStart) > csrStart.key ){
///*           AGGINVERSE
///*           if( eof ) break "while( 1 )" loop.
///*         }
///*         RETURN_ROW
///*       }
///*       while( !eof csrCurrent ){
///*         RETURN_ROW
///*       }
///*
///* The text above leaves out many details. Refer to the code and comments
///* below for a more complete picture.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_window_code_step(p_parse: *mut Parse, p: &Select,
    p_w_info: *mut WhereInfo, reg_gosub: i32, addr_gosub: i32) -> () {
    unsafe {
        let p_m_win: *mut Window = (*p).p_win;
        let p_order_by: *mut ExprList = unsafe { (*p_m_win).p_order_by };
        let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };
        let mut csr_write: i32 = 0;
        /// Cursor used to write to eph. table
        let csr_input: i32 =
            unsafe {
                (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                *mut SrcItem).offset(0 as isize)).i_cursor
            };
        /// Cursor of sub-select
        let n_input: i32 =
            unsafe {
                    (*unsafe {
                                    (*(unsafe { (*(*p).p_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)).p_s_tab
                                }).n_col
                } as i32;
        /// Number of cols returned by sub
        let mut i_input: i32 = 0;
        /// To iterate through sub cols
        let mut addr_ne: i32 = 0;
        /// Address of OP_Ne
        let mut addr_gosub_flush: i32 = 0;
        /// Address of OP_Gosub to flush:
        let mut addr_integer: i32 = 0;
        /// Address of OP_Integer
        let mut addr_empty: i32 = 0;
        /// Address of OP_Rewind in flush:
        let mut reg_new: i32 = 0;
        /// Array of registers holding new input row
        let mut reg_record: i32 = 0;
        /// regNew array in record form
        let mut reg_new_peer: i32 = 0;
        /// Peer values for new row (part of regNew)
        let mut reg_peer: i32 = 0;
        /// Peer values for current row
        let mut reg_flush_part: i32 = 0;
        /// Register for "Gosub flush_partition"
        let mut s: WindowCodeArg = unsafe { core::mem::zeroed() };
        /// Context object for sub-routines
        let mut lbl_where_end: i32 = 0;
        /// Label just before sqlite3WhereEnd() code
        let mut reg_start: i32 = 0;
        /// Value of <expr> PRECEDING
        let mut reg_end: i32 = 0;

        /// Value of <expr> FOLLOWING
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        lbl_where_end = unsafe { sqlite3_vdbe_make_label(p_parse) };

        /// Fill in the context object
        unsafe {
            memset(&raw mut s as *mut (), 0,
                core::mem::size_of::<WindowCodeArg>() as u64)
        };
        s.p_parse = p_parse;
        s.p_m_win = p_m_win;
        s.p_vdbe = v;
        s.reg_gosub = reg_gosub;
        s.addr_gosub = addr_gosub;
        s.current.csr = unsafe { (*p_m_win).i_eph_csr };
        csr_write = s.current.csr + 1;
        s.start.csr = s.current.csr + 2;
        s.end.csr = s.current.csr + 3;
        '__s15:
            {
            match unsafe { (*p_m_win).e_start } {
                87 => {
                    if unsafe { (*p_m_win).e_frm_type } as i32 != 90 &&
                            window_expr_gt_zero(unsafe { &*p_parse },
                                    unsafe { (*p_m_win).p_start } as *const Expr) != 0 {
                        s.e_delete = 1;
                    }
                }
                91 => {
                    if window_cache_frame(p_m_win) == 0 {
                        if unsafe { (*p_m_win).e_end } as i32 == 89 {
                            if unsafe { (*p_m_win).e_frm_type } as i32 != 90 &&
                                    window_expr_gt_zero(unsafe { &*p_parse },
                                            unsafe { (*p_m_win).p_end } as *const Expr) != 0 {
                                s.e_delete = 3;
                            }
                        } else { s.e_delete = 1; }
                    }
                }
                _ => { s.e_delete = 2; }
            }
        }

        /// Allocate registers for the array of values from the sub-query, the
        ///* same values in record form, and the rowid used to insert said record
        ///* into the ephemeral table.
        (reg_new = unsafe { (*p_parse).n_mem } + 1);
        unsafe { (*p_parse).n_mem += n_input };
        reg_record =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        s.reg_rowid =
            { let __p = unsafe { &mut (*p_parse).n_mem }; *__p += 1; *__p };
        if unsafe { (*p_m_win).e_start } as i32 == 89 ||
                unsafe { (*p_m_win).e_start } as i32 == 87 {
            reg_start =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                };
        }
        if unsafe { (*p_m_win).e_end } as i32 == 89 ||
                unsafe { (*p_m_win).e_end } as i32 == 87 {
            reg_end =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                };
        }
        if unsafe { (*p_m_win).e_frm_type } as i32 != 77 {
            let n_peer: i32 =
                if !(p_order_by).is_null() {
                    unsafe { (*p_order_by).n_expr }
                } else { 0 };
            reg_new_peer = reg_new + unsafe { (*p_m_win).n_buffer_col };
            if !(unsafe { (*p_m_win).p_partition }).is_null() {
                reg_new_peer +=
                    unsafe { (*unsafe { (*p_m_win).p_partition }).n_expr };
            }
            reg_peer = unsafe { (*p_parse).n_mem } + 1;
            unsafe { (*p_parse).n_mem += n_peer };
            s.start.reg = unsafe { (*p_parse).n_mem } + 1;
            unsafe { (*p_parse).n_mem += n_peer };
            s.current.reg = unsafe { (*p_parse).n_mem } + 1;
            unsafe { (*p_parse).n_mem += n_peer };
            s.end.reg = unsafe { (*p_parse).n_mem } + 1;
            unsafe { (*p_parse).n_mem += n_peer };
        }
        {
            i_input = 0;
            '__b16: loop {
                if !(i_input < n_input) { break '__b16; }
                '__c16: loop {
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 96, csr_input, i_input,
                            reg_new + i_input)
                    };
                    break '__c16;
                }
                { let __p = &mut i_input; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { sqlite3_vdbe_add_op3(v, 99, reg_new, n_input, reg_record) };
        if !(unsafe { (*p_m_win).p_partition }).is_null() {
            let mut addr: i32 = 0;
            let p_part: *mut ExprList = unsafe { (*p_m_win).p_partition };
            let n_part: i32 = unsafe { (*p_part).n_expr };
            let reg_new_part: i32 =
                reg_new + unsafe { (*p_m_win).n_buffer_col };
            let p_key_info: *mut KeyInfo =
                unsafe {
                    sqlite3_key_info_from_expr_list(p_parse, p_part, 0, 0)
                };
            reg_flush_part =
                {
                    let __p = unsafe { &mut (*p_parse).n_mem };
                    *__p += 1;
                    *__p
                };
            addr =
                unsafe {
                    sqlite3_vdbe_add_op3(v, 92, reg_new_part,
                        unsafe { (*p_m_win).reg_part }, n_part)
                };
            unsafe { sqlite3_vdbe_append_p4(v, p_key_info as *mut (), -9) };
            unsafe {
                sqlite3_vdbe_add_op3(v, 14, addr + 2, addr + 4, addr + 2)
            };
            addr_gosub_flush =
                unsafe { sqlite3_vdbe_add_op1(v, 10, reg_flush_part) };
            unsafe {
                sqlite3_vdbe_add_op3(v, 82, reg_new_part,
                    unsafe { (*p_m_win).reg_part }, n_part - 1)
            };
        }

        /// Insert the new row into the ephemeral table
        unsafe { sqlite3_vdbe_add_op2(v, 129, csr_write, s.reg_rowid) };
        unsafe {
            sqlite3_vdbe_add_op3(v, 130, csr_write, reg_record, s.reg_rowid)
        };
        addr_ne =
            unsafe {
                sqlite3_vdbe_add_op3(v, 53, unsafe { (*p_m_win).reg_one }, 0,
                    s.reg_rowid)
            };

        /// This block is run for the first row of each partition
        (s.reg_arg = window_init_accum(p_parse, p_m_win));
        if reg_start != 0 {
            unsafe {
                sqlite3_expr_code(p_parse, unsafe { (*p_m_win).p_start },
                    reg_start)
            };
            window_check_value(p_parse, reg_start,
                0 +
                    if unsafe { (*p_m_win).e_frm_type } as i32 == 90 {
                        3
                    } else { 0 });
        }
        if reg_end != 0 {
            unsafe {
                sqlite3_expr_code(p_parse, unsafe { (*p_m_win).p_end },
                    reg_end)
            };
            window_check_value(p_parse, reg_end,
                1 +
                    if unsafe { (*p_m_win).e_frm_type } as i32 == 90 {
                        3
                    } else { 0 });
        }
        if unsafe { (*p_m_win).e_frm_type } as i32 != 90 &&
                    unsafe { (*p_m_win).e_start } as i32 ==
                        unsafe { (*p_m_win).e_end } as i32 && reg_start != 0 {
            let op: i32 =
                if unsafe { (*p_m_win).e_start } as i32 == 87 {
                    58
                } else { 56 };
            let addr_ge: i32 =
                unsafe { sqlite3_vdbe_add_op3(v, op, reg_start, 0, reg_end) };

            ///   values previously checked
            window_agg_final(&s, 0);
            unsafe { sqlite3_vdbe_add_op1(v, 36, s.current.csr) };
            window_return_one_row(&mut s);
            unsafe { sqlite3_vdbe_add_op1(v, 148, s.current.csr) };
            unsafe { sqlite3_vdbe_add_op2(v, 9, 0, lbl_where_end) };
            unsafe { sqlite3_vdbe_jump_here(v, addr_ge) };
        }
        if unsafe { (*p_m_win).e_start } as i32 == 87 &&
                    unsafe { (*p_m_win).e_frm_type } as i32 != 90 &&
                reg_end != 0 {
            { let _ = 0; };
            unsafe {
                sqlite3_vdbe_add_op3(v, 108, reg_start, reg_end, reg_start)
            };
        }
        if unsafe { (*p_m_win).e_start } as i32 != 91 {
            unsafe { sqlite3_vdbe_add_op1(v, 36, s.start.csr) };
        }
        unsafe { sqlite3_vdbe_add_op1(v, 36, s.current.csr) };
        unsafe { sqlite3_vdbe_add_op1(v, 36, s.end.csr) };
        if reg_peer != 0 && !(p_order_by).is_null() {
            unsafe {
                sqlite3_vdbe_add_op3(v, 82, reg_new_peer, reg_peer,
                    unsafe { (*p_order_by).n_expr } - 1)
            };
            unsafe {
                sqlite3_vdbe_add_op3(v, 82, reg_peer, s.start.reg,
                    unsafe { (*p_order_by).n_expr } - 1)
            };
            unsafe {
                sqlite3_vdbe_add_op3(v, 82, reg_peer, s.current.reg,
                    unsafe { (*p_order_by).n_expr } - 1)
            };
            unsafe {
                sqlite3_vdbe_add_op3(v, 82, reg_peer, s.end.reg,
                    unsafe { (*p_order_by).n_expr } - 1)
            };
        }
        unsafe { sqlite3_vdbe_add_op2(v, 9, 0, lbl_where_end) };
        unsafe { sqlite3_vdbe_jump_here(v, addr_ne) };
        if reg_peer != 0 {
            window_if_new_peer(p_parse, p_order_by, reg_new_peer, reg_peer,
                lbl_where_end);
        }
        if unsafe { (*p_m_win).e_start } as i32 == 87 {
            window_code_op(&mut s, 3, 0, 0);
            if unsafe { (*p_m_win).e_end } as i32 != 91 {
                if unsafe { (*p_m_win).e_frm_type } as i32 == 90 {
                    let mut lbl: i32 =
                        unsafe { sqlite3_vdbe_make_label(p_parse) };
                    let addr_next: i32 =
                        unsafe { sqlite3_vdbe_current_addr(v) };
                    window_code_range_test(&mut s, 58, s.current.csr, reg_end,
                        s.end.csr, lbl);
                    window_code_op(&mut s, 2, reg_start, 0);
                    window_code_op(&mut s, 1, 0, 0);
                    unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr_next) };
                    unsafe { sqlite3_vdbe_resolve_label(v, lbl) };
                } else {
                    window_code_op(&mut s, 1, reg_end, 0);
                    window_code_op(&mut s, 2, reg_start, 0);
                }
            }
        } else if unsafe { (*p_m_win).e_end } as i32 == 89 {
            let b_rps: i32 =
                (unsafe { (*p_m_win).e_start } as i32 == 89 &&
                        unsafe { (*p_m_win).e_frm_type } as i32 == 90) as i32;
            window_code_op(&mut s, 3, reg_end, 0);
            if b_rps != 0 { window_code_op(&mut s, 2, reg_start, 0); }
            window_code_op(&mut s, 1, 0, 0);
            if (b_rps == 0) as i32 != 0 {
                window_code_op(&mut s, 2, reg_start, 0);
            }
        } else {
            let mut addr: i32 = 0;
            window_code_op(&mut s, 3, 0, 0);
            if unsafe { (*p_m_win).e_end } as i32 != 91 {
                if unsafe { (*p_m_win).e_frm_type } as i32 == 90 {
                    let mut lbl: i32 = 0;
                    addr = unsafe { sqlite3_vdbe_current_addr(v) };
                    if reg_end != 0 {
                        lbl = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        window_code_range_test(&mut s, 58, s.current.csr, reg_end,
                            s.end.csr, lbl);
                    }
                    window_code_op(&mut s, 1, 0, 0);
                    window_code_op(&mut s, 2, reg_start, 0);
                    if reg_end != 0 {
                        unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr) };
                        unsafe { sqlite3_vdbe_resolve_label(v, lbl) };
                    }
                } else {
                    if reg_end != 0 {
                        addr =
                            unsafe { sqlite3_vdbe_add_op3(v, 61, reg_end, 0, 1) };
                    }
                    window_code_op(&mut s, 1, 0, 0);
                    window_code_op(&mut s, 2, reg_start, 0);
                    if reg_end != 0 {
                        unsafe { sqlite3_vdbe_jump_here(v, addr) };
                    }
                }
            }
        }

        /// End of the main input loop
        unsafe { sqlite3_vdbe_resolve_label(v, lbl_where_end) };
        unsafe { sqlite3_where_end(p_w_info) };
        if !(unsafe { (*p_m_win).p_partition }).is_null() {
            addr_integer =
                unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_flush_part) };
            unsafe { sqlite3_vdbe_jump_here(v, addr_gosub_flush) };
        }
        s.reg_rowid = 0;
        addr_empty = unsafe { sqlite3_vdbe_add_op1(v, 36, csr_write) };
        if unsafe { (*p_m_win).e_end } as i32 == 89 {
            let b_rps_1: i32 =
                (unsafe { (*p_m_win).e_start } as i32 == 89 &&
                        unsafe { (*p_m_win).e_frm_type } as i32 == 90) as i32;
            window_code_op(&mut s, 3, reg_end, 0);
            if b_rps_1 != 0 { window_code_op(&mut s, 2, reg_start, 0); }
            window_code_op(&mut s, 1, 0, 0);
        } else if unsafe { (*p_m_win).e_start } as i32 == 87 {
            let mut addr_start: i32 = 0;
            let mut addr_break1: i32 = 0;
            let mut addr_break2: i32 = 0;
            let mut addr_break3: i32 = 0;
            window_code_op(&mut s, 3, 0, 0);
            if unsafe { (*p_m_win).e_frm_type } as i32 == 90 {
                addr_start = unsafe { sqlite3_vdbe_current_addr(v) };
                addr_break2 = window_code_op(&mut s, 2, reg_start, 1);
                addr_break1 = window_code_op(&mut s, 1, 0, 1);
            } else if unsafe { (*p_m_win).e_end } as i32 == 91 {
                addr_start = unsafe { sqlite3_vdbe_current_addr(v) };
                addr_break1 = window_code_op(&mut s, 1, reg_start, 1);
                addr_break2 = window_code_op(&mut s, 2, 0, 1);
            } else {
                { let _ = 0; };

                /// assert( regStart>=0 );
                ///* regEnd = regEnd - regStart;
                ///* regStart = 0;
                unsafe {
                    sqlite3_vdbe_add_op3(v, 108, reg_start, reg_end, reg_end)
                };
                unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_start) };
                addr_start = unsafe { sqlite3_vdbe_current_addr(v) };
                addr_break1 = window_code_op(&mut s, 1, reg_end, 1);
                addr_break2 = window_code_op(&mut s, 2, reg_start, 1);
            }
            unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr_start) };
            unsafe { sqlite3_vdbe_jump_here(v, addr_break2) };
            addr_start = unsafe { sqlite3_vdbe_current_addr(v) };
            addr_break3 = window_code_op(&mut s, 1, 0, 1);
            unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr_start) };
            unsafe { sqlite3_vdbe_jump_here(v, addr_break1) };
            unsafe { sqlite3_vdbe_jump_here(v, addr_break3) };
        } else {
            let mut addr_break: i32 = 0;
            let mut addr_start_1: i32 = 0;
            window_code_op(&mut s, 3, 0, 0);
            addr_start_1 = unsafe { sqlite3_vdbe_current_addr(v) };
            addr_break = window_code_op(&mut s, 1, 0, 1);
            window_code_op(&mut s, 2, reg_start, 0);
            unsafe { sqlite3_vdbe_add_op2(v, 9, 0, addr_start_1) };
            unsafe { sqlite3_vdbe_jump_here(v, addr_break) };
        }
        unsafe { sqlite3_vdbe_jump_here(v, addr_empty) };
        unsafe { sqlite3_vdbe_add_op1(v, 148, s.current.csr) };
        if !(unsafe { (*p_m_win).p_partition }).is_null() {
            if unsafe { (*p_m_win).reg_start_rowid } != 0 {
                unsafe {
                    sqlite3_vdbe_add_op2(v, 73, 1,
                        unsafe { (*p_m_win).reg_start_rowid })
                };
                unsafe {
                    sqlite3_vdbe_add_op2(v, 73, 0,
                        unsafe { (*p_m_win).reg_end_rowid })
                };
            }
            unsafe {
                sqlite3_vdbe_change_p1(v, addr_integer,
                    unsafe { sqlite3_vdbe_current_addr(v) })
            };
            unsafe { sqlite3_vdbe_add_op1(v, 69, reg_flush_part) };
        }
    }
}

extern "C" fn disallow_aggregates_in_order_by_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 169 &&
                unsafe { (*p_expr_1).p_agg_info } == core::ptr::null_mut() {
            { let _ = 0; };
            unsafe {
                sqlite3_error_msg(unsafe { (*p_walker_1).p_parse },
                    c"misuse of aggregate: %s()".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_expr_1).u.z_token })
            };
        }
        return 0;
    }
}

///* Append a copy of each expression in expression-list pAppend to
///* expression list pList. Return a pointer to the result list.
extern "C" fn expr_list_append_list(p_parse_1: *mut Parse,
    mut p_list_1: *mut ExprList, p_append_1: *const ExprList,
    b_int_to_null_1: i32) -> *mut ExprList {
    unsafe {
        if !(p_append_1).is_null() {
            let mut i: i32 = 0;
            let n_init: i32 =
                if !(p_list_1).is_null() {
                    unsafe { (*p_list_1).n_expr }
                } else { 0 };
            {
                i = 0;
                '__b17: loop {
                    if !(i < unsafe { (*p_append_1).n_expr }) { break '__b17; }
                    '__c17: loop {
                        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
                        let p_dup: *mut Expr =
                            unsafe {
                                sqlite3_expr_dup(db,
                                    unsafe {
                                            (*(unsafe { (*p_append_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        } as *const Expr, 0)
                            };
                        if unsafe { (*db).malloc_failed } != 0 {
                            unsafe { sqlite3_expr_delete(db, p_dup) };
                            break '__b17;
                        }
                        if b_int_to_null_1 != 0 {
                            let mut i_dummy: i32 = 0;
                            let mut p_sub: *mut Expr = core::ptr::null_mut();
                            p_sub =
                                unsafe { sqlite3_expr_skip_collate_and_likely(p_dup) };
                            if unsafe {
                                        sqlite3_expr_is_integer(p_sub as *const Expr, &mut i_dummy,
                                            core::ptr::null_mut())
                                    } != 0 {
                                unsafe { (*p_sub).op = 122 as u8 };
                                unsafe {
                                    (*p_sub).flags &= !(2048 | 268435456 | 536870912) as u32
                                };
                                unsafe { (*p_sub).u.z_token = core::ptr::null_mut() };
                            }
                        }
                        p_list_1 =
                            unsafe {
                                sqlite3_expr_list_append(p_parse_1, p_list_1, p_dup)
                            };
                        if !(p_list_1).is_null() {
                            unsafe {
                                (*(unsafe { (*p_list_1).a.as_ptr() } as
                                                        *mut ExprListItem).offset((n_init + i) as
                                                        isize)).fg.sort_flags =
                                    unsafe {
                                        (*(unsafe { (*p_append_1).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).fg.sort_flags
                                    }
                            };
                        }
                        break '__c17;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        return p_list_1;
    }
}

///* Callback function used by selectWindowRewriteEList(). If necessary,
///* this function appends to the output expression-list and updates
///* expression (*ppExpr) in place.
extern "C" fn select_window_rewrite_expr_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let p: *mut WindowRewrite = unsafe { (*p_walker_1).u.p_rewrite };
        let p_parse: *mut Parse = unsafe { (*p_walker_1).p_parse };
        { let _ = 0; };
        { let _ = 0; };
        if !(unsafe { (*p).p_sub_select }).is_null() {
            if unsafe { (*p_expr_1).op } as i32 != 168 {
                return 0;
            } else {
                let n_src: i32 = unsafe { (*unsafe { (*p).p_src }).n_src };
                let mut i: i32 = 0;
                {
                    i = 0;
                    '__b18: loop {
                        if !(i < n_src) { break '__b18; }
                        '__c18: loop {
                            if unsafe { (*p_expr_1).i_table } ==
                                    unsafe {
                                        (*(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                        *mut SrcItem).offset(i as isize)).i_cursor
                                    } {
                                break '__b18;
                            }
                            break '__c18;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if i == n_src { return 0; }
            }
        }
        '__s19:
            {
            match unsafe { (*p_expr_1).op } {
                172 => {
                    if !(unsafe { (*p_expr_1).flags } & 16777216 as u32 !=
                                        0 as u32) as i32 != 0 {
                        break '__s19;
                    } else {
                        let mut p_win: *mut Window = core::ptr::null_mut();
                        {
                            p_win = unsafe { (*p).p_win };
                            '__b20: loop {
                                if !(!(p_win).is_null()) { break '__b20; }
                                '__c20: loop {
                                    if unsafe { (*p_expr_1).y.p_win } == p_win {
                                        { let _ = 0; };
                                        return 1;
                                    }
                                    break '__c20;
                                }
                                p_win = unsafe { (*p_win).p_next_win };
                            }
                        }
                    }
                    {
                        let mut i_col: i32 = -1;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            return 2;
                        }
                        if !(unsafe { (*p).p_sub }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b21: loop {
                                    if !(i < unsafe { (*unsafe { (*p).p_sub }).n_expr }) {
                                        break '__b21;
                                    }
                                    '__c21: loop {
                                        if 0 ==
                                                unsafe {
                                                    sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p).p_sub }).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                                            } as *const Expr, p_expr_1 as *const Expr, -1)
                                                } {
                                            i_col = i;
                                            break '__b21;
                                        }
                                        break '__c21;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if i_col < 0 {
                            let p_dup: *mut Expr =
                                unsafe {
                                    sqlite3_expr_dup(unsafe { (*p_parse).db },
                                        p_expr_1 as *const Expr, 0)
                                };
                            if !(p_dup).is_null() &&
                                    unsafe { (*p_dup).op } as i32 == 169 {
                                unsafe { (*p_dup).op = 172 as u8 };
                            }
                            unsafe {
                                (*p).p_sub =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse, unsafe { (*p).p_sub },
                                            p_dup)
                                    }
                            };
                        }
                        if !(unsafe { (*p).p_sub }).is_null() {
                            let f: i32 =
                                (unsafe { (*p_expr_1).flags } & 512 as u32) as i32;
                            { let _ = 0; };
                            unsafe { (*p_expr_1).flags |= 134217728 as u32 };
                            unsafe {
                                sqlite3_expr_delete(unsafe { (*p_parse).db }, p_expr_1)
                            };
                            unsafe { (*p_expr_1).flags &= !(134217728 as u32) };
                            unsafe {
                                memset(p_expr_1 as *mut (), 0,
                                    core::mem::size_of::<Expr>() as u64)
                            };
                            unsafe { (*p_expr_1).op = 168 as u8 };
                            unsafe {
                                (*p_expr_1).i_column =
                                    if i_col < 0 {
                                            (unsafe { (*unsafe { (*p).p_sub }).n_expr }) - 1
                                        } else { i_col } as YnVar
                            };
                            unsafe {
                                (*p_expr_1).i_table =
                                    unsafe { (*unsafe { (*p).p_win }).i_eph_csr }
                            };
                            unsafe { (*p_expr_1).y.p_tab = unsafe { (*p).p_tab } };
                            unsafe { (*p_expr_1).flags = f as u32 };
                        }
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            return 2;
                        }
                        break '__s19;
                    }
                }
                179 => {
                    {
                        let mut i_col: i32 = -1;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            return 2;
                        }
                        if !(unsafe { (*p).p_sub }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b21: loop {
                                    if !(i < unsafe { (*unsafe { (*p).p_sub }).n_expr }) {
                                        break '__b21;
                                    }
                                    '__c21: loop {
                                        if 0 ==
                                                unsafe {
                                                    sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p).p_sub }).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                                            } as *const Expr, p_expr_1 as *const Expr, -1)
                                                } {
                                            i_col = i;
                                            break '__b21;
                                        }
                                        break '__c21;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if i_col < 0 {
                            let p_dup: *mut Expr =
                                unsafe {
                                    sqlite3_expr_dup(unsafe { (*p_parse).db },
                                        p_expr_1 as *const Expr, 0)
                                };
                            if !(p_dup).is_null() &&
                                    unsafe { (*p_dup).op } as i32 == 169 {
                                unsafe { (*p_dup).op = 172 as u8 };
                            }
                            unsafe {
                                (*p).p_sub =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse, unsafe { (*p).p_sub },
                                            p_dup)
                                    }
                            };
                        }
                        if !(unsafe { (*p).p_sub }).is_null() {
                            let f: i32 =
                                (unsafe { (*p_expr_1).flags } & 512 as u32) as i32;
                            { let _ = 0; };
                            unsafe { (*p_expr_1).flags |= 134217728 as u32 };
                            unsafe {
                                sqlite3_expr_delete(unsafe { (*p_parse).db }, p_expr_1)
                            };
                            unsafe { (*p_expr_1).flags &= !(134217728 as u32) };
                            unsafe {
                                memset(p_expr_1 as *mut (), 0,
                                    core::mem::size_of::<Expr>() as u64)
                            };
                            unsafe { (*p_expr_1).op = 168 as u8 };
                            unsafe {
                                (*p_expr_1).i_column =
                                    if i_col < 0 {
                                            (unsafe { (*unsafe { (*p).p_sub }).n_expr }) - 1
                                        } else { i_col } as YnVar
                            };
                            unsafe {
                                (*p_expr_1).i_table =
                                    unsafe { (*unsafe { (*p).p_win }).i_eph_csr }
                            };
                            unsafe { (*p_expr_1).y.p_tab = unsafe { (*p).p_tab } };
                            unsafe { (*p_expr_1).flags = f as u32 };
                        }
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            return 2;
                        }
                        break '__s19;
                    }
                }
                169 => {
                    {
                        let mut i_col: i32 = -1;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            return 2;
                        }
                        if !(unsafe { (*p).p_sub }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b21: loop {
                                    if !(i < unsafe { (*unsafe { (*p).p_sub }).n_expr }) {
                                        break '__b21;
                                    }
                                    '__c21: loop {
                                        if 0 ==
                                                unsafe {
                                                    sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p).p_sub }).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                                            } as *const Expr, p_expr_1 as *const Expr, -1)
                                                } {
                                            i_col = i;
                                            break '__b21;
                                        }
                                        break '__c21;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if i_col < 0 {
                            let p_dup: *mut Expr =
                                unsafe {
                                    sqlite3_expr_dup(unsafe { (*p_parse).db },
                                        p_expr_1 as *const Expr, 0)
                                };
                            if !(p_dup).is_null() &&
                                    unsafe { (*p_dup).op } as i32 == 169 {
                                unsafe { (*p_dup).op = 172 as u8 };
                            }
                            unsafe {
                                (*p).p_sub =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse, unsafe { (*p).p_sub },
                                            p_dup)
                                    }
                            };
                        }
                        if !(unsafe { (*p).p_sub }).is_null() {
                            let f: i32 =
                                (unsafe { (*p_expr_1).flags } & 512 as u32) as i32;
                            { let _ = 0; };
                            unsafe { (*p_expr_1).flags |= 134217728 as u32 };
                            unsafe {
                                sqlite3_expr_delete(unsafe { (*p_parse).db }, p_expr_1)
                            };
                            unsafe { (*p_expr_1).flags &= !(134217728 as u32) };
                            unsafe {
                                memset(p_expr_1 as *mut (), 0,
                                    core::mem::size_of::<Expr>() as u64)
                            };
                            unsafe { (*p_expr_1).op = 168 as u8 };
                            unsafe {
                                (*p_expr_1).i_column =
                                    if i_col < 0 {
                                            (unsafe { (*unsafe { (*p).p_sub }).n_expr }) - 1
                                        } else { i_col } as YnVar
                            };
                            unsafe {
                                (*p_expr_1).i_table =
                                    unsafe { (*unsafe { (*p).p_win }).i_eph_csr }
                            };
                            unsafe { (*p_expr_1).y.p_tab = unsafe { (*p).p_tab } };
                            unsafe { (*p_expr_1).flags = f as u32 };
                        }
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            return 2;
                        }
                        break '__s19;
                    }
                }
                168 => {
                    {
                        let mut i_col: i32 = -1;
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            return 2;
                        }
                        if !(unsafe { (*p).p_sub }).is_null() {
                            let mut i: i32 = 0;
                            {
                                i = 0;
                                '__b21: loop {
                                    if !(i < unsafe { (*unsafe { (*p).p_sub }).n_expr }) {
                                        break '__b21;
                                    }
                                    '__c21: loop {
                                        if 0 ==
                                                unsafe {
                                                    sqlite3_expr_compare(core::ptr::null(),
                                                        unsafe {
                                                                (*(unsafe { (*unsafe { (*p).p_sub }).a.as_ptr() } as
                                                                                *mut ExprListItem).offset(i as isize)).p_expr
                                                            } as *const Expr, p_expr_1 as *const Expr, -1)
                                                } {
                                            i_col = i;
                                            break '__b21;
                                        }
                                        break '__c21;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                        }
                        if i_col < 0 {
                            let p_dup: *mut Expr =
                                unsafe {
                                    sqlite3_expr_dup(unsafe { (*p_parse).db },
                                        p_expr_1 as *const Expr, 0)
                                };
                            if !(p_dup).is_null() &&
                                    unsafe { (*p_dup).op } as i32 == 169 {
                                unsafe { (*p_dup).op = 172 as u8 };
                            }
                            unsafe {
                                (*p).p_sub =
                                    unsafe {
                                        sqlite3_expr_list_append(p_parse, unsafe { (*p).p_sub },
                                            p_dup)
                                    }
                            };
                        }
                        if !(unsafe { (*p).p_sub }).is_null() {
                            let f: i32 =
                                (unsafe { (*p_expr_1).flags } & 512 as u32) as i32;
                            { let _ = 0; };
                            unsafe { (*p_expr_1).flags |= 134217728 as u32 };
                            unsafe {
                                sqlite3_expr_delete(unsafe { (*p_parse).db }, p_expr_1)
                            };
                            unsafe { (*p_expr_1).flags &= !(134217728 as u32) };
                            unsafe {
                                memset(p_expr_1 as *mut (), 0,
                                    core::mem::size_of::<Expr>() as u64)
                            };
                            unsafe { (*p_expr_1).op = 168 as u8 };
                            unsafe {
                                (*p_expr_1).i_column =
                                    if i_col < 0 {
                                            (unsafe { (*unsafe { (*p).p_sub }).n_expr }) - 1
                                        } else { i_col } as YnVar
                            };
                            unsafe {
                                (*p_expr_1).i_table =
                                    unsafe { (*unsafe { (*p).p_win }).i_eph_csr }
                            };
                            unsafe { (*p_expr_1).y.p_tab = unsafe { (*p).p_tab } };
                            unsafe { (*p_expr_1).flags = f as u32 };
                        }
                        if unsafe { (*unsafe { (*p_parse).db }).malloc_failed } != 0
                            {
                            return 2;
                        }
                        break '__s19;
                    }
                }
                _ => {}
            }
        }
        return 0;
    }
}

extern "C" fn select_window_rewrite_select_cb(p_walker_1: *mut Walker,
    p_select_1: *mut Select) -> i32 {
    unsafe {
        let p: *mut WindowRewrite = unsafe { (*p_walker_1).u.p_rewrite };
        let p_save: *mut Select = unsafe { (*p).p_sub_select };
        if p_save == p_select_1 {
            return 0;
        } else {
            unsafe { (*p).p_sub_select = p_select_1 };
            unsafe { sqlite3_walk_select(p_walker_1, p_select_1) };
            unsafe { (*p).p_sub_select = p_save };
        }
        return 1;
    }
}

///* Iterate through each expression in expression-list pEList. For each:
///*
///*   * TK_COLUMN,
///*   * aggregate function, or
///*   * window function with a Window object that is not a member of the
///*     Window list passed as the second argument (pWin).
///*
///* Append the node to output expression-list (*ppSub). And replace it
///* with a TK_COLUMN that reads the (N-1)th element of table
///* pWin->iEphCsr, where N is the number of elements in (*ppSub) after
///* appending the new one.
extern "C" fn select_window_rewrite_e_list(p_parse_1: *mut Parse,
    p_win_1: *mut Window, p_src_1: *mut SrcList, p_e_list_1: *mut ExprList,
    p_tab_1: *mut Table, pp_sub_1: &mut *mut ExprList) -> () {
    unsafe {
        let mut s_walker: Walker = unsafe { core::mem::zeroed() };
        let mut s_rewrite: WindowRewrite = unsafe { core::mem::zeroed() };
        { let _ = 0; };
        unsafe {
            memset(&raw mut s_walker as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        unsafe {
            memset(&raw mut s_rewrite as *mut (), 0,
                core::mem::size_of::<WindowRewrite>() as u64)
        };
        s_rewrite.p_sub = *pp_sub_1;
        s_rewrite.p_win = p_win_1;
        s_rewrite.p_src = p_src_1;
        s_rewrite.p_tab = p_tab_1;
        s_walker.p_parse = p_parse_1;
        s_walker.x_expr_callback = Some(select_window_rewrite_expr_cb);
        s_walker.x_select_callback = Some(select_window_rewrite_select_cb);
        s_walker.u.p_rewrite = &mut s_rewrite;
        {
            let _ =
                unsafe { sqlite3_walk_expr_list(&mut s_walker, p_e_list_1) };
        };
        *pp_sub_1 = s_rewrite.p_sub;
    }
}

///* When rewriting a query, if the new subquery in the FROM clause
///* contains TK_AGG_FUNCTION nodes that refer to an outer query,
///* then we have to increase the Expr->op2 values of those nodes
///* due to the extra subquery layer that was added.
///*
///* See also the incrAggDepth() routine in resolve.c
extern "C" fn sqlite3_window_extra_agg_func_depth(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    if unsafe { (*p_expr_1).op } as i32 == 169 &&
            unsafe { (*p_expr_1).op2 } as i32 >=
                unsafe { (*p_walker_1).walker_depth } {
        {
            let __p = unsafe { &mut (*p_expr_1).op2 };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    return 0;
}

///* If the SELECT statement passed as the second argument does not invoke
///* any SQL window functions, this function is a no-op. Otherwise, it
///* rewrites the SELECT statement so that window function xStep functions
///* are invoked in the correct order as described under "SELECT REWRITING"
///* at the top of this file.
#[unsafe(no_mangle)]
#[allow(unused_doc_comments)]
pub extern "C" fn sqlite3_window_rewrite(p_parse: *mut Parse, p: *mut Select)
    -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if !(unsafe { (*p).p_win }).is_null() &&
                        unsafe { (*p).p_prior } == core::ptr::null_mut() &&
                    unsafe { (*p).sel_flags } & 1048576 as u32 == 0 as u32 &&
                !(unsafe { (*p_parse).e_parse_mode } as i32 >= 2) as i32 != 0
            {
            let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            let mut p_sub: *mut Select = core::ptr::null_mut();
            /// The subquery
            let p_src: *mut SrcList = unsafe { (*p).p_src };
            let p_where: *mut Expr = unsafe { (*p).p_where };
            let p_group_by: *mut ExprList = unsafe { (*p).p_group_by };
            let p_having: *mut Expr = unsafe { (*p).p_having };
            let mut p_sort: *mut ExprList = core::ptr::null_mut();
            let mut p_sublist: *mut ExprList = core::ptr::null_mut();
            /// Expression list for sub-query
            let p_m_win: *mut Window = unsafe { (*p).p_win };
            /// Main window object
            let mut p_win: *mut Window = core::ptr::null_mut();
            /// Window object iterator
            let mut p_tab: *mut Table = core::ptr::null_mut();
            let mut w: Walker = unsafe { core::mem::zeroed() };
            let sel_flags: u32 = unsafe { (*p).sel_flags };
            p_tab =
                unsafe {
                        sqlite3_db_malloc_zero(db,
                            core::mem::size_of::<Table>() as u64)
                    } as *mut Table;
            if p_tab == core::ptr::null_mut() {
                return unsafe { sqlite3_error_to_parser(db, 7) };
            }
            unsafe { sqlite3_agg_info_persist_walker_init(&mut w, p_parse) };
            unsafe { sqlite3_walk_select(&mut w, p) };
            if unsafe { (*p).sel_flags } & 8 as u32 == 0 as u32 {
                w.x_expr_callback = Some(disallow_aggregates_in_order_by_cb);
                w.x_select_callback = None;
                unsafe {
                    sqlite3_walk_expr_list(&mut w, unsafe { (*p).p_order_by })
                };
            }
            unsafe { (*p).p_src = core::ptr::null_mut() };
            unsafe { (*p).p_where = core::ptr::null_mut() };
            unsafe { (*p).p_group_by = core::ptr::null_mut() };
            unsafe { (*p).p_having = core::ptr::null_mut() };
            unsafe { (*p).sel_flags &= !(8 as u32) };
            unsafe { (*p).sel_flags |= 1048576 as u32 };

            /// Create the ORDER BY clause for the sub-select. This is the concatenation
            ///* of the window PARTITION and ORDER BY clauses. Then, if this makes it
            ///* redundant, remove the ORDER BY from the parent SELECT.
            (p_sort =
                expr_list_append_list(p_parse, core::ptr::null_mut(),
                    unsafe { (*p_m_win).p_partition } as *const ExprList, 1));
            p_sort =
                expr_list_append_list(p_parse, p_sort,
                    unsafe { (*p_m_win).p_order_by } as *const ExprList, 1);
            if !(p_sort).is_null() && !(unsafe { (*p).p_order_by }).is_null()
                    &&
                    unsafe { (*unsafe { (*p).p_order_by }).n_expr } <=
                        unsafe { (*p_sort).n_expr } {
                let n_save: i32 = unsafe { (*p_sort).n_expr };
                unsafe {
                    (*p_sort).n_expr =
                        unsafe { (*unsafe { (*p).p_order_by }).n_expr }
                };
                if unsafe {
                            sqlite3_expr_list_compare(p_sort as *const ExprList,
                                unsafe { (*p).p_order_by } as *const ExprList, -1)
                        } == 0 {
                    unsafe {
                        sqlite3_expr_list_delete(db, unsafe { (*p).p_order_by })
                    };
                    unsafe { (*p).p_order_by = core::ptr::null_mut() };
                }
                unsafe { (*p_sort).n_expr = n_save };
            }

            /// Assign a cursor number for the ephemeral table used to buffer rows.
            ///* The OpenEphemeral instruction is coded later, after it is known how
            ///* many columns the table will have.
            unsafe {
                (*p_m_win).i_eph_csr =
                    {
                        let __p = unsafe { &mut (*p_parse).n_tab };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    }
            };
            unsafe { (*p_parse).n_tab += 3 };
            select_window_rewrite_e_list(p_parse, p_m_win, p_src,
                unsafe { (*p).p_e_list }, p_tab, &mut p_sublist);
            select_window_rewrite_e_list(p_parse, p_m_win, p_src,
                unsafe { (*p).p_order_by }, p_tab, &mut p_sublist);
            unsafe {
                (*p_m_win).n_buffer_col =
                    if !(p_sublist).is_null() {
                        unsafe { (*p_sublist).n_expr }
                    } else { 0 }
            };

            /// Append the PARTITION BY and ORDER BY expressions to the to the
            ///* sub-select expression list. They are required to figure out where
            ///* boundaries for partitions and sets of peer rows lie.
            (p_sublist =
                expr_list_append_list(p_parse, p_sublist,
                    unsafe { (*p_m_win).p_partition } as *const ExprList, 0));
            p_sublist =
                expr_list_append_list(p_parse, p_sublist,
                    unsafe { (*p_m_win).p_order_by } as *const ExprList, 0);
            {
                p_win = p_m_win;
                '__b22: loop {
                    if !(!(p_win).is_null()) { break '__b22; }
                    '__c22: loop {
                        let mut p_args: *mut ExprList = core::ptr::null_mut();
                        { let _ = 0; };
                        { let _ = 0; };
                        p_args = unsafe { (*unsafe { (*p_win).p_owner }).x.p_list };
                        if unsafe { (*unsafe { (*p_win).p_w_func }).func_flags } &
                                    1048576 as u32 != 0 {
                            select_window_rewrite_e_list(p_parse, p_m_win, p_src,
                                p_args, p_tab, &mut p_sublist);
                            unsafe {
                                (*p_win).i_arg_col =
                                    if !(p_sublist).is_null() {
                                        unsafe { (*p_sublist).n_expr }
                                    } else { 0 }
                            };
                            unsafe { (*p_win).b_expr_args = 1 as u8 };
                        } else {
                            unsafe {
                                (*p_win).i_arg_col =
                                    if !(p_sublist).is_null() {
                                        unsafe { (*p_sublist).n_expr }
                                    } else { 0 }
                            };
                            p_sublist =
                                expr_list_append_list(p_parse, p_sublist,
                                    p_args as *const ExprList, 0);
                        }
                        if !(unsafe { (*p_win).p_filter }).is_null() {
                            let p_filter: *mut Expr =
                                unsafe {
                                    sqlite3_expr_dup(db,
                                        unsafe { (*p_win).p_filter } as *const Expr, 0)
                                };
                            p_sublist =
                                unsafe {
                                    sqlite3_expr_list_append(p_parse, p_sublist, p_filter)
                                };
                        }
                        unsafe {
                            (*p_win).reg_accum =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        unsafe {
                            (*p_win).reg_result =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 77, 0,
                                unsafe { (*p_win).reg_accum })
                        };
                        break '__c22;
                    }
                    p_win = unsafe { (*p_win).p_next_win };
                }
            }
            if p_sublist == core::ptr::null_mut() {
                p_sublist =
                    unsafe {
                        sqlite3_expr_list_append(p_parse, core::ptr::null_mut(),
                            unsafe { sqlite3_expr_int32(db, 0) })
                    };
            }
            p_sub =
                unsafe {
                    sqlite3_select_new(p_parse, p_sublist, p_src, p_where,
                        p_group_by, p_having, p_sort, 0 as u32,
                        core::ptr::null_mut())
                };
            unsafe {
                (*p).p_src =
                    unsafe {
                        sqlite3_src_list_append(p_parse, core::ptr::null_mut(),
                            core::ptr::null_mut(), core::ptr::null_mut())
                    }
            };
            { let _ = 0; };
            if unsafe { (*p).p_src } == core::ptr::null_mut() {
                unsafe { sqlite3_select_delete(db, p_sub) };
            } else if unsafe {
                        sqlite3_src_item_attach_subquery(p_parse,
                            unsafe {
                                &mut *(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)
                            }, p_sub, 0)
                    } != 0 {
                let mut p_tab2: *mut Table = core::ptr::null_mut();
                unsafe {
                    (*(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                        *mut SrcItem).offset(0 as
                                        isize)).fg.set_is_correlated(1 as u32 as u32)
                };
                unsafe {
                    sqlite3_src_list_assign_cursors(p_parse,
                        unsafe { (*p).p_src })
                };
                unsafe { (*p_sub).sel_flags |= (64 | 134217728) as u32 };
                p_tab2 =
                    unsafe {
                        sqlite3_result_set_of_select(p_parse, p_sub, 64 as i8)
                    };
                unsafe { (*p_sub).sel_flags |= sel_flags & 8 as u32 };
                if p_tab2 == core::ptr::null_mut() {

                    /// Might actually be some other kind of error, but in that case
                    ///* pParse->nErr will be set, so if SQLITE_NOMEM is set, we will get
                    ///* the correct error message regardless.
                    (rc = 7);
                } else {
                    unsafe {
                        memcpy(p_tab as *mut (), p_tab2 as *const (),
                            core::mem::size_of::<Table>() as u64)
                    };
                    unsafe { (*p_tab).tab_flags |= 16384 as u32 };
                    unsafe {
                        (*(unsafe { (*unsafe { (*p).p_src }).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as isize)).p_s_tab = p_tab
                    };
                    p_tab = p_tab2;
                    unsafe {
                        memset(&raw mut w as *mut (), 0,
                            core::mem::size_of::<Walker>() as u64)
                    };
                    w.x_expr_callback =
                        Some(sqlite3_window_extra_agg_func_depth);
                    w.x_select_callback = Some(sqlite3_walker_depth_increase);
                    w.x_select_callback2 = Some(sqlite3_walker_depth_decrease);
                    unsafe { sqlite3_walk_select(&mut w, p_sub) };
                }
            }
            if unsafe { (*db).malloc_failed } != 0 { rc = 7; }

            /// Defer deleting the temporary table pTab because if an error occurred,
            ///* there could still be references to that table embedded in the
            ///* result-set or ORDER BY clause of the SELECT statement p.
            unsafe {
                sqlite3_parser_add_cleanup(p_parse, Some(sqlite3_db_free),
                    p_tab as *mut ())
            };
        }
        { let _ = 0; };
        return rc;
    }
}

extern "C" fn window_find(p_parse_1: *mut Parse, p_list_1: *mut Window,
    z_name_1: *const i8) -> *mut Window {
    let mut p: *mut Window = core::ptr::null_mut();
    {
        p = p_list_1;
        '__b23: loop {
            if !(!(p).is_null()) { break '__b23; }
            '__c23: loop {
                if unsafe {
                            sqlite3_str_i_cmp(unsafe { (*p).z_name } as *const i8,
                                z_name_1)
                        } == 0 {
                    break '__b23;
                }
                break '__c23;
            }
            p = unsafe { (*p).p_next_win };
        }
    }
    if p == core::ptr::null_mut() {
        unsafe {
            sqlite3_error_msg(p_parse_1,
                c"no such window: %s".as_ptr() as *mut i8 as *const i8,
                z_name_1)
        };
    }
    return p;
}

///* Window *pWin has just been created from a WINDOW clause. Token pBase
///* is the base window. Earlier windows from the same WINDOW clause are
///* stored in the linked list starting at pWin->pNextWin. This function
///* either updates *pWin according to the base specification, or else
///* leaves an error in pParse.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_chain(p_parse: *mut Parse,
    p_win: &mut Window, p_list: *mut Window) -> () {
    if !((*p_win).z_base).is_null() {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        let p_exist: *const Window =
            window_find(p_parse, p_list, (*p_win).z_base as *const i8) as
                *const Window;
        if !(p_exist).is_null() {
            let mut z_err: *const i8 = core::ptr::null();
            if !((*p_win).p_partition).is_null() {
                z_err = c"PARTITION clause".as_ptr() as *mut i8 as *const i8;
            } else if !(unsafe { (*p_exist).p_order_by }).is_null() &&
                    !((*p_win).p_order_by).is_null() {
                z_err = c"ORDER BY clause".as_ptr() as *mut i8 as *const i8;
            } else if unsafe { (*p_exist).b_implicit_frame } as i32 == 0 {
                z_err =
                    c"frame specification".as_ptr() as *mut i8 as *const i8;
            }
            if !(z_err).is_null() {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"cannot override %s of window: %s".as_ptr() as *mut i8 as
                            *const i8, z_err, (*p_win).z_base)
                };
            } else {
                (*p_win).p_partition =
                    unsafe {
                        sqlite3_expr_list_dup(db,
                            unsafe { (*p_exist).p_partition } as *const ExprList, 0)
                    };
                if !(unsafe { (*p_exist).p_order_by }).is_null() {
                    { let _ = 0; };
                    (*p_win).p_order_by =
                        unsafe {
                            sqlite3_expr_list_dup(db,
                                unsafe { (*p_exist).p_order_by } as *const ExprList, 0)
                        };
                }
                unsafe { sqlite3_db_free(db, (*p_win).z_base as *mut ()) };
                (*p_win).z_base = core::ptr::null_mut();
            }
        }
    }
}

///* Static names for the built-in window function names.  These static
///* names are used, rather than string literals, so that FuncDef objects
///* can be associated with a particular window function by direct
///* comparison of the zName pointer.  Example:
///*
///*       if( pFuncDef->zName==row_valueName ){ ... }
static row_number_name: [i8; 11] =
    [114 as i8, 111 as i8, 119 as i8, 95 as i8, 110 as i8, 117 as i8,
            109 as i8, 98 as i8, 101 as i8, 114 as i8, 0 as i8];

static dense_rank_name: [i8; 11] =
    [100 as i8, 101 as i8, 110 as i8, 115 as i8, 101 as i8, 95 as i8,
            114 as i8, 97 as i8, 110 as i8, 107 as i8, 0 as i8];

static rank_name: [i8; 5] =
    [114 as i8, 97 as i8, 110 as i8, 107 as i8, 0 as i8];

static percent_rank_name: [i8; 13] =
    [112 as i8, 101 as i8, 114 as i8, 99 as i8, 101 as i8, 110 as i8,
            116 as i8, 95 as i8, 114 as i8, 97 as i8, 110 as i8, 107 as i8,
            0 as i8];

static cume_dist_name: [i8; 10] =
    [99 as i8, 117 as i8, 109 as i8, 101 as i8, 95 as i8, 100 as i8,
            105 as i8, 115 as i8, 116 as i8, 0 as i8];

static ntile_name: [i8; 6] =
    [110 as i8, 116 as i8, 105 as i8, 108 as i8, 101 as i8, 0 as i8];

///* This function is called immediately after resolving the function name
///* for a window function within a SELECT statement. Argument pList is a
///* linked list of WINDOW definitions for the current SELECT statement.
///* Argument pFunc is the function definition just resolved and pWin
///* is the Window object representing the associated OVER clause. This
///* function updates the contents of pWin as follows:
///*
///*   * If the OVER clause referred to a named window (as in "max(x) OVER win"),
///*     search list pList for a matching WINDOW definition, and update pWin
///*     accordingly. If no such WINDOW clause can be found, leave an error
///*     in pParse.
///*
///*   * If the function is a built-in window function that requires the
///*     window to be coerced (see "BUILT-IN WINDOW FUNCTIONS" at the top
///*     of this file), pWin is updated here.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_update(p_parse: *mut Parse,
    p_list: *mut Window, p_win: *mut Window, p_func: *mut FuncDef) -> () {
    if !(unsafe { (*p_win).z_name }).is_null() &&
            unsafe { (*p_win).e_frm_type } as i32 == 0 {
        let p: *const Window =
            window_find(p_parse, p_list,
                    unsafe { (*p_win).z_name } as *const i8) as *const Window;
        if p == core::ptr::null_mut() { return; }
        unsafe {
            (*p_win).p_partition =
                unsafe {
                    sqlite3_expr_list_dup(unsafe { (*p_parse).db },
                        unsafe { (*p).p_partition } as *const ExprList, 0)
                }
        };
        unsafe {
            (*p_win).p_order_by =
                unsafe {
                    sqlite3_expr_list_dup(unsafe { (*p_parse).db },
                        unsafe { (*p).p_order_by } as *const ExprList, 0)
                }
        };
        unsafe {
            (*p_win).p_start =
                unsafe {
                    sqlite3_expr_dup(unsafe { (*p_parse).db },
                        unsafe { (*p).p_start } as *const Expr, 0)
                }
        };
        unsafe {
            (*p_win).p_end =
                unsafe {
                    sqlite3_expr_dup(unsafe { (*p_parse).db },
                        unsafe { (*p).p_end } as *const Expr, 0)
                }
        };
        unsafe { (*p_win).e_start = unsafe { (*p).e_start } };
        unsafe { (*p_win).e_end = unsafe { (*p).e_end } };
        unsafe { (*p_win).e_frm_type = unsafe { (*p).e_frm_type } };
        unsafe { (*p_win).e_exclude = unsafe { (*p).e_exclude } };
    } else { sqlite3_window_chain(p_parse, unsafe { &mut *p_win }, p_list); }
    if unsafe { (*p_win).e_frm_type } as i32 == 90 &&
                (!(unsafe { (*p_win).p_start }).is_null() ||
                    !(unsafe { (*p_win).p_end }).is_null()) &&
            (unsafe { (*p_win).p_order_by } == core::ptr::null_mut() ||
                unsafe { (*unsafe { (*p_win).p_order_by }).n_expr } != 1) {
        unsafe {
            sqlite3_error_msg(p_parse,
                c"RANGE with offset PRECEDING/FOLLOWING requires one ORDER BY expression".as_ptr()
                        as *mut i8 as *const i8)
        };
    } else if unsafe { (*p_func).func_flags } & 65536 as u32 != 0 {
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        if !(unsafe { (*p_win).p_filter }).is_null() {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"FILTER clause may only be used with aggregate window functions".as_ptr()
                            as *mut i8 as *const i8)
            };
        } else {
            let a_up: [WindowUpdateN12WindowUpdate; 8] =
                [WindowUpdateN12WindowUpdate {
                            z_func: row_number_name.as_ptr() as *const i8,
                            e_frm_type: 77,
                            e_start: 91,
                            e_end: 86,
                        },
                        WindowUpdateN12WindowUpdate {
                            z_func: dense_rank_name.as_ptr() as *const i8,
                            e_frm_type: 90,
                            e_start: 91,
                            e_end: 86,
                        },
                        WindowUpdateN12WindowUpdate {
                            z_func: rank_name.as_ptr() as *const i8,
                            e_frm_type: 90,
                            e_start: 91,
                            e_end: 86,
                        },
                        WindowUpdateN12WindowUpdate {
                            z_func: percent_rank_name.as_ptr() as *const i8,
                            e_frm_type: 93,
                            e_start: 86,
                            e_end: 91,
                        },
                        WindowUpdateN12WindowUpdate {
                            z_func: cume_dist_name.as_ptr() as *const i8,
                            e_frm_type: 93,
                            e_start: 87,
                            e_end: 91,
                        },
                        WindowUpdateN12WindowUpdate {
                            z_func: ntile_name.as_ptr() as *const i8,
                            e_frm_type: 77,
                            e_start: 86,
                            e_end: 91,
                        },
                        WindowUpdateN12WindowUpdate {
                            z_func: lead_name.as_ptr() as *const i8,
                            e_frm_type: 77,
                            e_start: 91,
                            e_end: 91,
                        },
                        WindowUpdateN12WindowUpdate {
                            z_func: lag_name.as_ptr() as *const i8,
                            e_frm_type: 77,
                            e_start: 91,
                            e_end: 86,
                        }];
            let mut i: i32 = 0;
            {
                i = 0;
                '__b24: loop {
                    if !(i <
                                    (core::mem::size_of::<[WindowUpdateN12WindowUpdate; 8]>() as
                                                u64 /
                                            core::mem::size_of::<WindowUpdateN12WindowUpdate>() as u64)
                                        as i32) {
                        break '__b24;
                    }
                    '__c24: loop {
                        if unsafe { (*p_func).z_name } == a_up[i as usize].z_func {
                            unsafe {
                                sqlite3_expr_delete(db, unsafe { (*p_win).p_start })
                            };
                            unsafe {
                                sqlite3_expr_delete(db, unsafe { (*p_win).p_end })
                            };
                            unsafe {
                                (*p_win).p_end =
                                    {
                                        unsafe { (*p_win).p_start = core::ptr::null_mut() };
                                        unsafe { (*p_win).p_start }
                                    }
                            };
                            unsafe {
                                (*p_win).e_frm_type = a_up[i as usize].e_frm_type as u8
                            };
                            unsafe {
                                (*p_win).e_start = a_up[i as usize].e_start as u8
                            };
                            unsafe { (*p_win).e_end = a_up[i as usize].e_end as u8 };
                            unsafe { (*p_win).e_exclude = 0 as u8 };
                            if unsafe { (*p_win).e_start } as i32 == 87 {
                                unsafe {
                                    (*p_win).p_start = unsafe { sqlite3_expr_int32(db, 1) }
                                };
                            }
                            break '__b24;
                        }
                        break '__c24;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
    }
    unsafe { (*p_win).p_w_func = p_func };
}

///* Allocate and return a duplicate of the Window object indicated by the
///* third argument. Set the Window.pOwner field of the new object to
///* pOwner.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_dup(db: *mut Sqlite3, p_owner: *mut Expr,
    p: *mut Window) -> *mut Window {
    let mut p_new: *mut Window = core::ptr::null_mut();
    if !(p).is_null() {
        p_new =
            unsafe {
                    sqlite3_db_malloc_zero(db,
                        core::mem::size_of::<Window>() as u64)
                } as *mut Window;
        if !(p_new).is_null() {
            unsafe {
                (*p_new).z_name =
                    unsafe {
                        sqlite3_db_str_dup(db, unsafe { (*p).z_name } as *const i8)
                    }
            };
            unsafe {
                (*p_new).z_base =
                    unsafe {
                        sqlite3_db_str_dup(db, unsafe { (*p).z_base } as *const i8)
                    }
            };
            unsafe {
                (*p_new).p_filter =
                    unsafe {
                        sqlite3_expr_dup(db,
                            unsafe { (*p).p_filter } as *const Expr, 0)
                    }
            };
            unsafe { (*p_new).p_w_func = unsafe { (*p).p_w_func } };
            unsafe {
                (*p_new).p_partition =
                    unsafe {
                        sqlite3_expr_list_dup(db,
                            unsafe { (*p).p_partition } as *const ExprList, 0)
                    }
            };
            unsafe {
                (*p_new).p_order_by =
                    unsafe {
                        sqlite3_expr_list_dup(db,
                            unsafe { (*p).p_order_by } as *const ExprList, 0)
                    }
            };
            unsafe { (*p_new).e_frm_type = unsafe { (*p).e_frm_type } };
            unsafe { (*p_new).e_end = unsafe { (*p).e_end } };
            unsafe { (*p_new).e_start = unsafe { (*p).e_start } };
            unsafe { (*p_new).e_exclude = unsafe { (*p).e_exclude } };
            unsafe { (*p_new).reg_result = unsafe { (*p).reg_result } };
            unsafe { (*p_new).reg_accum = unsafe { (*p).reg_accum } };
            unsafe { (*p_new).i_arg_col = unsafe { (*p).i_arg_col } };
            unsafe { (*p_new).i_eph_csr = unsafe { (*p).i_eph_csr } };
            unsafe { (*p_new).b_expr_args = unsafe { (*p).b_expr_args } };
            unsafe {
                (*p_new).p_start =
                    unsafe {
                        sqlite3_expr_dup(db, unsafe { (*p).p_start } as *const Expr,
                            0)
                    }
            };
            unsafe {
                (*p_new).p_end =
                    unsafe {
                        sqlite3_expr_dup(db, unsafe { (*p).p_end } as *const Expr,
                            0)
                    }
            };
            unsafe { (*p_new).p_owner = p_owner };
            unsafe {
                (*p_new).b_implicit_frame = unsafe { (*p).b_implicit_frame }
            };
        }
    }
    return p_new;
}

///* Return a copy of the linked list of Window objects passed as the
///* second argument.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_list_dup(db: *mut Sqlite3, p: *mut Window)
    -> *mut Window {
    let mut p_win: *mut Window = core::ptr::null_mut();
    let mut p_ret: *mut Window = core::ptr::null_mut();
    let mut pp: *mut *mut Window = &mut p_ret;
    {
        p_win = p;
        '__b25: loop {
            if !(!(p_win).is_null()) { break '__b25; }
            '__c25: loop {
                unsafe {
                    *pp = sqlite3_window_dup(db, core::ptr::null_mut(), p_win)
                };
                if unsafe { *pp } == core::ptr::null_mut() { break '__b25; }
                pp = unsafe { &mut (*unsafe { *pp }).p_next_win };
                break '__c25;
            }
            p_win = unsafe { (*p_win).p_next_win };
        }
    }
    return p_ret;
}

///* Implementation of built-in window function row_number(). Assumes that the
///* window frame has been coerced to:
///*
///*   ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
extern "C" fn row_number_step_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let p: *mut i64 =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<i64>() as i32)
            } as *mut i64;
    if !(p).is_null() {
        { let __p = unsafe { &mut *p }; let __t = *__p; *__p += 1; __t };
    }
    { let _ = n_arg_1; };
    { let _ = ap_arg_1; };
}

extern "C" fn row_number_value_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let p: *const i64 =
        unsafe {
                    sqlite3_aggregate_context(p_ctx_1,
                        core::mem::size_of::<i64>() as i32)
                } as *mut i64 as *const i64;
    unsafe {
        sqlite3_result_int64(p_ctx_1,
            if !(p).is_null() { unsafe { *p } } else { 0 as i64 })
    };
}

///* Context object type used by rank(), dense_rank(), percent_rank() and
///* cume_dist().
#[repr(C)]
#[derive(Copy, Clone)]
struct CallCount {
    n_value: i64,
    n_step: i64,
    n_total: i64,
}

///* Implementation of built-in window function dense_rank(). Assumes that
///* the window frame has been set to:
///*
///*   RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
extern "C" fn dense_rank_step_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    if !(p).is_null() { unsafe { (*p).n_step = 1 as i64 }; }
    { let _ = n_arg_1; };
    { let _ = ap_arg_1; };
}

extern "C" fn dense_rank_value_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    if !(p).is_null() {
        if unsafe { (*p).n_step } != 0 {
            {
                let __p = unsafe { &mut (*p).n_value };
                let __t = *__p;
                *__p += 1;
                __t
            };
            unsafe { (*p).n_step = 0 as i64 };
        }
        unsafe { sqlite3_result_int64(p_ctx_1, unsafe { (*p).n_value }) };
    }
}

///* Implementation of built-in window function rank(). Assumes that
///* the window frame has been set to:
///*
///*   RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
extern "C" fn rank_step_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    if !(p).is_null() {
        {
            let __p = unsafe { &mut (*p).n_step };
            let __t = *__p;
            *__p += 1;
            __t
        };
        if unsafe { (*p).n_value } == 0 as i64 {
            unsafe { (*p).n_value = unsafe { (*p).n_step } };
        }
    }
    { let _ = n_arg_1; };
    { let _ = ap_arg_1; };
}

extern "C" fn rank_value_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    if !(p).is_null() {
        unsafe { sqlite3_result_int64(p_ctx_1, unsafe { (*p).n_value }) };
        unsafe { (*p).n_value = 0 as i64 };
    }
}

///* Implementation of built-in window function percent_rank(). Assumes that
///* the window frame has been set to:
///*
///*   GROUPS BETWEEN CURRENT ROW AND UNBOUNDED FOLLOWING
extern "C" fn percent_rank_step_func(p_ctx_1: *mut Sqlite3Context,
    n_arg_1: i32, ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    { let _ = n_arg_1; };
    { let _ = 0; };
    { let _ = ap_arg_1; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    if !(p).is_null() {
        {
            let __p = unsafe { &mut (*p).n_total };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
}

extern "C" fn percent_rank_value_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    if !(p).is_null() {
        unsafe { (*p).n_value = unsafe { (*p).n_step } };
        if unsafe { (*p).n_total } > 1 as i64 {
            let r: f64 =
                unsafe { (*p).n_value } as f64 /
                    (unsafe { (*p).n_total } - 1 as i64) as f64;
            unsafe { sqlite3_result_double(p_ctx_1, r) };
        } else { unsafe { sqlite3_result_double(p_ctx_1, 0.0) }; }
    }
}

extern "C" fn percent_rank_inv_func(p_ctx_1: *mut Sqlite3Context,
    n_arg_1: i32, ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    { let _ = n_arg_1; };
    { let _ = 0; };
    { let _ = ap_arg_1; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    { let __p = unsafe { &mut (*p).n_step }; let __t = *__p; *__p += 1; __t };
}

///* Implementation of built-in window function cume_dist(). Assumes that
///* the window frame has been set to:
///*
///*   GROUPS BETWEEN 1 FOLLOWING AND UNBOUNDED FOLLOWING
extern "C" fn cume_dist_step_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    { let _ = n_arg_1; };
    { let _ = 0; };
    { let _ = ap_arg_1; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    if !(p).is_null() {
        {
            let __p = unsafe { &mut (*p).n_total };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
}

extern "C" fn cume_dist_value_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *const CallCount = core::ptr::null();
    p = unsafe { sqlite3_aggregate_context(p_ctx_1, 0) } as *mut CallCount;
    if !(p).is_null() {
        let r: f64 =
            unsafe { (*p).n_step } as f64 / unsafe { (*p).n_total } as f64;
        unsafe { sqlite3_result_double(p_ctx_1, r) };
    }
}

extern "C" fn cume_dist_inv_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut CallCount = core::ptr::null_mut();
    { let _ = n_arg_1; };
    { let _ = 0; };
    { let _ = ap_arg_1; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<CallCount>() as i32)
            } as *mut CallCount;
    { let __p = unsafe { &mut (*p).n_step }; let __t = *__p; *__p += 1; __t };
}

///* Context object for ntile() window function.
#[repr(C)]
#[derive(Copy, Clone)]
struct NtileCtx {
    n_total: i64,
    n_param: i64,
    i_row: i64,
}

///* Implementation of ntile(). This assumes that the window frame has
///* been coerced to:
///*
///*   ROWS CURRENT ROW AND UNBOUNDED FOLLOWING
extern "C" fn ntile_step_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut NtileCtx = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = n_arg_1; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<NtileCtx>() as i32)
            } as *mut NtileCtx;
    if !(p).is_null() {
        if unsafe { (*p).n_total } == 0 as i64 {
            unsafe {
                (*p).n_param =
                    unsafe {
                        sqlite3_value_int64(unsafe { *ap_arg_1.offset(0 as isize) })
                    }
            };
            if unsafe { (*p).n_param } <= 0 as i64 {
                unsafe {
                    sqlite3_result_error(p_ctx_1,
                        c"argument of ntile must be a positive integer".as_ptr() as
                                *mut i8 as *const i8, -1)
                };
            }
        }
        {
            let __p = unsafe { &mut (*p).n_total };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
}

extern "C" fn ntile_value_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *const NtileCtx = core::ptr::null();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<NtileCtx>() as i32)
            } as *mut NtileCtx;
    if !(p).is_null() && unsafe { (*p).n_param } > 0 as i64 {
        let n_size: i32 =
            (unsafe { (*p).n_total } / unsafe { (*p).n_param }) as i32;
        if n_size == 0 {
            unsafe {
                sqlite3_result_int64(p_ctx_1,
                    unsafe { (*p).i_row } + 1 as i64)
            };
        } else {
            let n_large: i64 =
                unsafe { (*p).n_total } -
                    unsafe { (*p).n_param } * n_size as i64;
            let i_small: i64 = n_large * (n_size + 1) as i64;
            let i_row: i64 = unsafe { (*p).i_row };
            { let _ = 0; };
            if i_row < i_small {
                unsafe {
                    sqlite3_result_int64(p_ctx_1,
                        1 as i64 + i_row / (n_size + 1) as i64)
                };
            } else {
                unsafe {
                    sqlite3_result_int64(p_ctx_1,
                        1 as i64 + n_large + (i_row - i_small) / n_size as i64)
                };
            }
        }
    }
}

extern "C" fn ntile_inv_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut NtileCtx = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = n_arg_1; };
    { let _ = ap_arg_1; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<NtileCtx>() as i32)
            } as *mut NtileCtx;
    { let __p = unsafe { &mut (*p).i_row }; let __t = *__p; *__p += 1; __t };
}

///* Context object for last_value() window function.
#[repr(C)]
#[derive(Copy, Clone)]
struct LastValueCtx {
    p_val: *mut Sqlite3Value,
    n_val: i32,
}

///* Implementation of last_value().
extern "C" fn last_value_step_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut LastValueCtx = core::ptr::null_mut();
    { let _ = n_arg_1; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<LastValueCtx>() as i32)
            } as *mut LastValueCtx;
    if !(p).is_null() {
        unsafe { sqlite3_value_free(unsafe { (*p).p_val }) };
        unsafe {
            (*p).p_val =
                unsafe {
                    sqlite3_value_dup(unsafe { *ap_arg_1.offset(0 as isize) } as
                            *const Sqlite3Value)
                }
        };
        if unsafe { (*p).p_val } == core::ptr::null_mut() {
            unsafe { sqlite3_result_error_nomem(p_ctx_1) };
        } else {
            {
                let __p = unsafe { &mut (*p).n_val };
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }
}

extern "C" fn last_value_finalize_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *mut LastValueCtx = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<LastValueCtx>() as i32)
            } as *mut LastValueCtx;
    if !(p).is_null() && !(unsafe { (*p).p_val }).is_null() {
        unsafe { sqlite3_result_value(p_ctx_1, unsafe { (*p).p_val }) };
        unsafe { sqlite3_value_free(unsafe { (*p).p_val }) };
        unsafe { (*p).p_val = core::ptr::null_mut() };
    }
}

extern "C" fn last_value_value_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *const LastValueCtx = core::ptr::null();
    p = unsafe { sqlite3_aggregate_context(p_ctx_1, 0) } as *mut LastValueCtx;
    if !(p).is_null() && !(unsafe { (*p).p_val }).is_null() {
        unsafe { sqlite3_result_value(p_ctx_1, unsafe { (*p).p_val }) };
    }
}

extern "C" fn last_value_inv_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut LastValueCtx = core::ptr::null_mut();
    { let _ = n_arg_1; };
    { let _ = ap_arg_1; };
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<LastValueCtx>() as i32)
            } as *mut LastValueCtx;
    if !(p).is_null() {
        {
            let __p = unsafe { &mut (*p).n_val };
            let __t = *__p;
            *__p -= 1;
            __t
        };
        if unsafe { (*p).n_val } == 0 {
            unsafe { sqlite3_value_free(unsafe { (*p).p_val }) };
            unsafe { (*p).p_val = core::ptr::null_mut() };
        }
    }
}

static last_value_name: [i8; 11] =
    [108 as i8, 97 as i8, 115 as i8, 116 as i8, 95 as i8, 118 as i8, 97 as i8,
            108 as i8, 117 as i8, 101 as i8, 0 as i8];

///* Implementation of built-in window function nth_value(). This
///* implementation is used in "slow mode" only - when the EXCLUDE clause
///* is not set to the default value "NO OTHERS".
#[repr(C)]
#[derive(Copy, Clone)]
struct NthValueCtx {
    n_step: i64,
    p_value: *mut Sqlite3Value,
}

extern "C" fn nth_value_step_func(p_ctx_1: *mut Sqlite3Context, n_arg_1: i32,
    ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut NthValueCtx = core::ptr::null_mut();
    let mut i_val: i64 = 0 as i64;
    let mut f_val: f64 = 0.0;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 { break; }
        '__s27:
            {
            match __state {
                0 => { __state = 3; }
                2 => {
                    unsafe {
                        sqlite3_result_error(p_ctx_1,
                            c"second argument to nth_value must be a positive integer".as_ptr()
                                    as *mut i8 as *const i8, -1)
                    };
                    __state = 1;
                }
                3 => {
                    p =
                        unsafe {
                                sqlite3_aggregate_context(p_ctx_1,
                                    core::mem::size_of::<NthValueCtx>() as i32)
                            } as *mut NthValueCtx;
                    __state = 4;
                }
                4 => {
                    if !(p).is_null() { __state = 6; } else { __state = 5; }
                }
                5 => { { let _ = n_arg_1; }; __state = 26; }
                6 => { __state = 7; }
                7 => {
                    '__s28:
                        {
                        match unsafe {
                                sqlite3_value_numeric_type(unsafe {
                                        *ap_arg_1.offset(1 as isize)
                                    })
                            } {
                            1 => { __state = 9; }
                            2 => { __state = 10; }
                            _ => { __state = 11; }
                        }
                    }
                }
                8 => {
                    if i_val <= 0 as i64 {
                        __state = 21;
                    } else { __state = 20; }
                }
                9 => {
                    i_val =
                        unsafe {
                            sqlite3_value_int64(unsafe { *ap_arg_1.offset(1 as isize) })
                        };
                    __state = 13;
                }
                10 => {
                    f_val =
                        unsafe {
                            sqlite3_value_double(unsafe {
                                    *ap_arg_1.offset(1 as isize)
                                })
                        };
                    __state = 16;
                }
                11 => { __state = 2; }
                12 => { __state = 9; }
                13 => { __state = 8; }
                14 => { __state = 10; }
                15 => { __state = 11; }
                16 => {
                    if unsafe { sqlite3_real_to_i64(f_val) } as f64 != f_val {
                        __state = 18;
                    } else { __state = 17; }
                }
                17 => { i_val = f_val as i64; __state = 19; }
                18 => { __state = 2; }
                19 => { __state = 8; }
                20 => {
                    {
                        let __p = unsafe { &mut (*p).n_step };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    __state = 22;
                }
                21 => { __state = 2; }
                22 => {
                    if i_val == unsafe { (*p).n_step } {
                        __state = 23;
                    } else { __state = 5; }
                }
                23 => {
                    unsafe {
                        (*p).p_value =
                            unsafe {
                                sqlite3_value_dup(unsafe { *ap_arg_1.offset(0 as isize) } as
                                        *const Sqlite3Value)
                            }
                    };
                    __state = 24;
                }
                24 => {
                    if (unsafe { (*p).p_value }).is_null() as i32 != 0 {
                        __state = 25;
                    } else { __state = 5; }
                }
                25 => {
                    unsafe { sqlite3_result_error_nomem(p_ctx_1) };
                    __state = 5;
                }
                26 => { { let _ = ap_arg_1; }; __state = 27; }
                27 => { return; }
                28 => { __state = 2; }
                _ => {}
            }
        }
    }
}

extern "C" fn nth_value_finalize_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *mut NthValueCtx = core::ptr::null_mut();
    p = unsafe { sqlite3_aggregate_context(p_ctx_1, 0) } as *mut NthValueCtx;
    if !(p).is_null() && !(unsafe { (*p).p_value }).is_null() {
        unsafe { sqlite3_result_value(p_ctx_1, unsafe { (*p).p_value }) };
        unsafe { sqlite3_value_free(unsafe { (*p).p_value }) };
        unsafe { (*p).p_value = core::ptr::null_mut() };
    }
}

extern "C" fn noop_value_func(p: *mut Sqlite3Context) -> () {
    { let _ = p; };
}

extern "C" fn first_value_step_func(p_ctx_1: *mut Sqlite3Context,
    n_arg_1: i32, ap_arg_1: *mut *mut Sqlite3Value) -> () {
    let mut p: *mut NthValueCtx = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<NthValueCtx>() as i32)
            } as *mut NthValueCtx;
    if !(p).is_null() && unsafe { (*p).p_value } == core::ptr::null_mut() {
        unsafe {
            (*p).p_value =
                unsafe {
                    sqlite3_value_dup(unsafe { *ap_arg_1.offset(0 as isize) } as
                            *const Sqlite3Value)
                }
        };
        if (unsafe { (*p).p_value }).is_null() as i32 != 0 {
            unsafe { sqlite3_result_error_nomem(p_ctx_1) };
        }
    }
    { let _ = n_arg_1; };
    { let _ = ap_arg_1; };
}

extern "C" fn first_value_finalize_func(p_ctx_1: *mut Sqlite3Context) -> () {
    let mut p: *mut NthValueCtx = core::ptr::null_mut();
    p =
        unsafe {
                sqlite3_aggregate_context(p_ctx_1,
                    core::mem::size_of::<NthValueCtx>() as i32)
            } as *mut NthValueCtx;
    if !(p).is_null() && !(unsafe { (*p).p_value }).is_null() {
        unsafe { sqlite3_result_value(p_ctx_1, unsafe { (*p).p_value }) };
        unsafe { sqlite3_value_free(unsafe { (*p).p_value }) };
        unsafe { (*p).p_value = core::ptr::null_mut() };
    }
}

///* Register those built-in window functions that are not also aggregates.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_functions() -> () {
    unsafe {
        unsafe {
            sqlite3_insert_builtin_funcs(&raw mut a_window_funcs[0 as usize]
                    as *mut FuncDef,
                (core::mem::size_of::<[FuncDef; 15]>() as u64 /
                        core::mem::size_of::<FuncDef>() as u64) as i32)
        };
    }
}

///* Attach PARTITION and ORDER BY clauses pPartition and pOrderBy to window
///* pWin. Also, if parameter pBase is not NULL, set pWin->zBase to the
///* equivalent nul-terminated string.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_window_assemble(p_parse: &Parse, p_win: *mut Window,
    p_partition: *mut ExprList, p_order_by: *mut ExprList, p_base: *mut Token)
    -> *mut Window {
    unsafe {
        if !(p_win).is_null() {
            unsafe { (*p_win).p_partition = p_partition };
            unsafe { (*p_win).p_order_by = p_order_by };
            if !(p_base).is_null() {
                unsafe {
                    (*p_win).z_base =
                        unsafe {
                            sqlite3_db_str_n_dup((*p_parse).db, unsafe { (*p_base).z },
                                unsafe { (*p_base).n } as u64)
                        }
                };
            }
        } else {
            unsafe { sqlite3_expr_list_delete((*p_parse).db, p_partition) };
            unsafe { sqlite3_expr_list_delete((*p_parse).db, p_order_by) };
        }
        return p_win;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct WindowUpdateN12WindowUpdate {
    z_func: *const i8,
    e_frm_type: i32,
    e_start: i32,
    e_end: i32,
}

static mut az_err: [*const i8; 5] =
    [c"frame starting offset must be a non-negative integer".as_ptr() as
                *const i8,
            c"frame ending offset must be a non-negative integer".as_ptr() as
                *const i8,
            c"second argument to nth_value must be a positive integer".as_ptr()
                as *const i8,
            c"frame starting offset must be a non-negative number".as_ptr() as
                *const i8,
            c"frame ending offset must be a non-negative number".as_ptr() as
                *const i8];

static mut a_op: [i32; 5] = [58, 58, 55, 58, 58];

static mut a_window_funcs: [FuncDef; 15] =
    [FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(row_number_step_func),
                x_finalize: Some(row_number_value_func),
                x_value: Some(row_number_value_func),
                x_inverse: Some(noop_step_func),
                z_name: row_number_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(dense_rank_step_func),
                x_finalize: Some(dense_rank_value_func),
                x_value: Some(dense_rank_value_func),
                x_inverse: Some(noop_step_func),
                z_name: dense_rank_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(rank_step_func),
                x_finalize: Some(rank_value_func),
                x_value: Some(rank_value_func),
                x_inverse: Some(noop_step_func),
                z_name: rank_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(percent_rank_step_func),
                x_finalize: Some(percent_rank_value_func),
                x_value: Some(percent_rank_value_func),
                x_inverse: Some(percent_rank_inv_func),
                z_name: percent_rank_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 0 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(cume_dist_step_func),
                x_finalize: Some(cume_dist_value_func),
                x_value: Some(cume_dist_value_func),
                x_inverse: Some(cume_dist_inv_func),
                z_name: cume_dist_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(ntile_step_func),
                x_finalize: Some(ntile_value_func),
                x_value: Some(ntile_value_func),
                x_inverse: Some(ntile_inv_func),
                z_name: ntile_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(last_value_step_func),
                x_finalize: Some(last_value_finalize_func),
                x_value: Some(last_value_value_func),
                x_inverse: Some(last_value_inv_func),
                z_name: last_value_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(nth_value_step_func),
                x_finalize: Some(nth_value_finalize_func),
                x_value: Some(noop_value_func),
                x_inverse: Some(noop_step_func),
                z_name: nth_value_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(first_value_step_func),
                x_finalize: Some(first_value_finalize_func),
                x_value: Some(noop_value_func),
                x_inverse: Some(noop_step_func),
                z_name: first_value_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(noop_step_func),
                x_finalize: Some(noop_value_func),
                x_value: Some(noop_value_func),
                x_inverse: Some(noop_step_func),
                z_name: lead_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(noop_step_func),
                x_finalize: Some(noop_value_func),
                x_value: Some(noop_value_func),
                x_inverse: Some(noop_step_func),
                z_name: lead_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 3 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(noop_step_func),
                x_finalize: Some(noop_value_func),
                x_value: Some(noop_value_func),
                x_inverse: Some(noop_step_func),
                z_name: lead_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 1 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(noop_step_func),
                x_finalize: Some(noop_value_func),
                x_value: Some(noop_value_func),
                x_inverse: Some(noop_step_func),
                z_name: lag_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(noop_step_func),
                x_finalize: Some(noop_value_func),
                x_value: Some(noop_value_func),
                x_inverse: Some(noop_step_func),
                z_name: lag_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 3 as i16,
                func_flags: (8388608 | 1 | 65536 | 0) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(noop_step_func),
                x_finalize: Some(noop_value_func),
                x_value: Some(noop_value_func),
                x_inverse: Some(noop_step_func),
                z_name: lag_name.as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
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
    fn sqlite3_select_pop_with(_: *mut Walker, _: *mut Select)
    -> ();
    fn sqlite3_multi_values(p_parse_1: *mut Parse, p_left_1: *mut Select,
    p_row_1: *mut ExprList)
    -> *mut Select;
    fn sqlite3_multi_values_end(p_parse_1: *mut Parse, p_val_1: *mut Select)
    -> ();
    fn sqlite3_expr_delete(_: *mut Sqlite3, _: *mut Expr)
    -> ();
    fn sqlite3_expr_list_delete(_: *mut Sqlite3, _: *mut ExprList)
    -> ();
    fn sqlite3_db_free(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_error_msg(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_db_malloc_zero(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_expr_is_constant(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_rename_expr_unmap(_: *mut Parse, _: *mut Expr)
    -> ();
    fn sqlite3_expr_alloc(_: *mut Sqlite3, _: i32, _: *const Token, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_compare(_: *const Parse, _: *const Expr, _: *const Expr,
    _: i32)
    -> i32;
    fn sqlite3_expr_list_compare(_: *const ExprList, _: *const ExprList,
    _: i32)
    -> i32;
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_key_info_from_expr_list(_: *mut Parse, _: *mut ExprList,
    _: i32, _: i32)
    -> *mut KeyInfo;
    fn sqlite3_value_from_expr(_: *mut Sqlite3, _: *const Expr, _: u8, _: u8,
    _: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3ValueFree(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_expr_code(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_get_temp_reg(_: *mut Parse)
    -> i32;
    fn sqlite3_may_abort(_: *mut Parse)
    -> ();
    fn sqlite3_release_temp_reg(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_get_temp_range(_: *mut Parse, _: i32)
    -> i32;
    fn sqlite3_expr_code_expr_list(_: *mut Parse, _: *mut ExprList, _: i32,
    _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_nn_coll_seq(p_parse_1: *mut Parse, p_expr_1: *const Expr)
    -> *mut CollSeq;
    fn sqlite3_release_temp_range(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_where_end(_: *mut WhereInfo)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_error_to_parser(_: *mut Sqlite3, _: i32)
    -> i32;
    fn sqlite3_agg_info_persist_walker_init(_: *mut Walker, _: *mut Parse)
    -> ();
    fn sqlite3_expr_dup(_: *mut Sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_skip_collate_and_likely(_: *mut Expr)
    -> *mut Expr;
    fn sqlite3_expr_is_integer(_: *const Expr, _: *mut i32, _: *mut Parse)
    -> i32;
    fn sqlite3_expr_list_append(_: *mut Parse, _: *mut ExprList, _: *mut Expr)
    -> *mut ExprList;
    fn sqlite3_expr_int32(_: *mut Sqlite3, _: i32)
    -> *mut Expr;
    fn sqlite3_select_new(_: *mut Parse, _: *mut ExprList, _: *mut SrcList,
    _: *mut Expr, _: *mut ExprList, _: *mut Expr, _: *mut ExprList, _: u32,
    _: *mut Expr)
    -> *mut Select;
    fn sqlite3_src_list_append(_: *mut Parse, _: *mut SrcList, _: *mut Token,
    _: *mut Token)
    -> *mut SrcList;
    fn sqlite3_select_delete(_: *mut Sqlite3, _: *mut Select)
    -> ();
    fn sqlite3_src_item_attach_subquery(_: *mut Parse, _: *mut SrcItem,
    _: *mut Select, _: i32)
    -> i32;
    fn sqlite3_src_list_assign_cursors(_: *mut Parse, _: *mut SrcList)
    -> ();
    fn sqlite3_result_set_of_select(_: *mut Parse, _: *mut Select, _: i8)
    -> *mut Table;
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn sqlite3_str_i_cmp(_: *const i8, _: *const i8)
    -> i32;
    fn sqlite3_expr_list_dup(_: *mut Sqlite3, _: *const ExprList, _: i32)
    -> *mut ExprList;
    fn sqlite3_db_str_dup(_: *mut Sqlite3, _: *const i8)
    -> *mut i8;
    fn sqlite3_real_to_i64(_: f64)
    -> i64;
    fn sqlite3_insert_builtin_funcs(_: *mut FuncDef, _: i32)
    -> ();
    fn sqlite3_db_str_n_dup(_: *mut Sqlite3, _: *const i8, _: u64)
    -> *mut i8;
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
    fn sqlite3_db_malloc_raw(_: *mut Sqlite3, _: u64)
    -> *mut ();
    fn sqlite3_db_malloc_raw_nn(_: *mut Sqlite3, _: u64)
    -> *mut ();
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
    fn sqlite3_clear_temp_reg_cache(_: *mut Parse)
    -> ();
    fn sqlite3_touch_register(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_expr(_: *mut Sqlite3, _: i32, _: *const i8)
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
    fn sqlite3_expr_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_expr_deferred_delete(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_expr_unmap_and_delete(_: *mut Parse, _: *mut Expr)
    -> ();
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
    fn sqlite3_subquery_delete(_: *mut Sqlite3, _: *mut Subquery)
    -> ();
    fn sqlite3_subquery_detach(_: *mut Sqlite3, _: *mut SrcItem)
    -> *mut Select;
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
    fn sqlite3_expr_compare_skip(_: *mut Expr, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_implies_expr(_: *const Parse, _: *const Expr,
    _: *const Expr, _: i32)
    -> i32;
    fn sqlite3_expr_implies_non_null_row(_: *mut Expr, _: i32, _: i32)
    -> i32;
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
    fn sqlite3_expr_is_constant_or_function(_: *mut Expr, _: u8)
    -> i32;
    fn sqlite3_expr_is_constant_or_group_by(_: *mut Parse, _: *mut Expr,
    _: *mut ExprList)
    -> i32;
    fn sqlite3_expr_is_single_table_constraint(_: *mut Expr,
    _: *const SrcList, _: i32, _: i32)
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
    fn sqlite3_src_list_dup(_: *mut Sqlite3, _: *const SrcList, _: i32)
    -> *mut SrcList;
    fn sqlite3_id_list_dup(_: *mut Sqlite3, _: *const IdList)
    -> *mut IdList;
    fn sqlite3_select_dup(_: *mut Sqlite3, _: *const Select, _: i32)
    -> *mut Select;
    fn sqlite3_function_search(_: i32, _: *const i8)
    -> *mut FuncDef;
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
