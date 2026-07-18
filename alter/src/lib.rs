#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]

mod btree_h;
pub(crate) use crate::btree_h::*;
mod hash_h;
pub(crate) use crate::hash_h::*;
mod pager_h;
pub(crate) use crate::pager_h::*;
mod pcache_h;
pub(crate) use crate::pcache_h::*;
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
mod sqlite_int_h;
pub(crate) use crate::sqlite_int_h::*;
mod vdbe_h;
pub(crate) use crate::vdbe_h::*;

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
struct RenameToken {
    p: *const (),
    t: Token,
    p_next: *mut RenameToken,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RenameCtx {
    p_list: *mut RenameToken,
    n_list: i32,
    i_col: i32,
    p_tab: *mut Table,
    z_old: *const i8,
}

extern "C" fn rename_parse_sql(p: *mut Parse, z_db_1: *const i8,
    db: *mut Sqlite3, z_sql_1: *const i8, b_temp_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let mut flags: u64 = 0 as u64;
    unsafe { sqlite3_parse_object_init(p, db) };
    if z_sql_1 == core::ptr::null() { return 7; }
    if unsafe {
                sqlite3_strnicmp(z_sql_1,
                    c"CREATE ".as_ptr() as *mut i8 as *const i8, 7)
            } != 0 {
        return unsafe { sqlite3_corrupt_error(1168) };
    }
    if b_temp_1 != 0 {
        unsafe { (*db).init.i_db = 1 as u8 };
    } else {
        let i_db: i32 = unsafe { sqlite3_find_db_name(db, z_db_1) };
        { let _ = 0; };
        unsafe { (*db).init.i_db = i_db as u8 };
    }
    unsafe { (*p).e_parse_mode = 2 as u8 };
    unsafe { (*p).db = db };
    unsafe { (*p).n_query_loop = 1 as LogEst };
    flags = unsafe { (*db).flags };
    unsafe { (*db).flags |= (64 as u64) << 32 };
    rc = unsafe { sqlite3_run_parser(p, z_sql_1) };
    unsafe { (*db).flags = flags };
    if unsafe { (*db).malloc_failed } != 0 { rc = 7; }
    if rc == 0 &&
            (unsafe { (*p).p_new_table } == core::ptr::null_mut() &&
                    unsafe { (*p).p_new_index } == core::ptr::null_mut() &&
                unsafe { (*p).p_new_trigger } == core::ptr::null_mut()) {
        rc = unsafe { sqlite3_corrupt_error(1189) };
    }
    unsafe { (*db).init.i_db = 0 as u8 };
    return rc;
}

extern "C" fn rename_token_find(p_parse_1: *mut Parse,
    p_ctx_1: *mut RenameCtx, p_ptr_1: *const ()) -> *mut RenameToken {
    unsafe {
        let mut pp: *mut *mut RenameToken = core::ptr::null_mut();
        if p_ptr_1 == core::ptr::null() { return core::ptr::null_mut(); }
        {
            pp = unsafe { &mut (*p_parse_1).p_rename };
            '__b0: loop {
                if !(!(unsafe { *pp }).is_null()) { break '__b0; }
                '__c0: loop {
                    if unsafe { (*unsafe { *pp }).p } == p_ptr_1 {
                        let p_token: *mut RenameToken = unsafe { *pp };
                        if !(p_ctx_1).is_null() {
                            unsafe { *pp = unsafe { (*p_token).p_next } };
                            unsafe { (*p_token).p_next = unsafe { (*p_ctx_1).p_list } };
                            unsafe { (*p_ctx_1).p_list = p_token };
                            {
                                let __p = unsafe { &mut (*p_ctx_1).n_list };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                        return p_token;
                    }
                    break '__c0;
                }
                pp = unsafe { &mut (*unsafe { *pp }).p_next };
            }
        }
        return core::ptr::null_mut();
    }
}

extern "C" fn rename_column_expr_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let p: *mut RenameCtx = unsafe { (*p_walker_1).u.p_rename };
        if unsafe { (*p_expr_1).op } as i32 == 78 &&
                    unsafe { (*p_expr_1).i_column } as i32 ==
                        unsafe { (*p).i_col } &&
                unsafe { (*unsafe { (*p_walker_1).p_parse }).p_trigger_tab }
                    == unsafe { (*p).p_tab } {
            rename_token_find(unsafe { (*p_walker_1).p_parse }, p,
                p_expr_1 as *mut () as *const ());
        } else if unsafe { (*p_expr_1).op } as i32 == 168 &&
                        unsafe { (*p_expr_1).i_column } as i32 ==
                            unsafe { (*p).i_col } &&
                    unsafe { (*p_expr_1).flags } & (16777216 | 33554432) as u32
                        == 0 as u32 &&
                unsafe { (*p).p_tab } == unsafe { (*p_expr_1).y.p_tab } {
            rename_token_find(unsafe { (*p_walker_1).p_parse }, p,
                p_expr_1 as *mut () as *const ());
        }
        return 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rename_token_remap(p_parse: &Parse, p_to: *const (),
    p_from: *const ()) -> () {
    unsafe {
        let mut p: *mut RenameToken = core::ptr::null_mut();
        {
            p = (*p_parse).p_rename;
            '__b1: loop {
                if !(!(p).is_null()) { break '__b1; }
                '__c1: loop {
                    if unsafe { (*p).p } == p_from {
                        unsafe { (*p).p = p_to };
                        break '__b1;
                    }
                    break '__c1;
                }
                p = unsafe { (*p).p_next };
            }
        }
    }
}

extern "C" fn rename_unmap_expr_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let p_parse: *mut Parse = unsafe { (*p_walker_1).p_parse };
        sqlite3_rename_token_remap(unsafe { &*p_parse }, core::ptr::null(),
            p_expr_1 as *const ());
        if unsafe { (*p_expr_1).flags } & (16777216 | 33554432) as u32 ==
                0 as u32 {
            sqlite3_rename_token_remap(unsafe { &*p_parse },
                core::ptr::null(),
                unsafe { &raw mut (*p_expr_1).y.p_tab } as *const ());
        }
        return 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rename_exprlist_unmap(p_parse: *mut Parse,
    p_e_list: *mut ExprList) -> () {
    if !(p_e_list).is_null() {
        let mut i: i32 = 0;
        let mut s_walker: Walker = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut s_walker as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        s_walker.p_parse = p_parse;
        s_walker.x_expr_callback = Some(rename_unmap_expr_cb);
        unsafe { sqlite3_walk_expr_list(&mut s_walker, p_e_list) };
        {
            i = 0;
            '__b2: loop {
                if !(i < unsafe { (*p_e_list).n_expr }) { break '__b2; }
                '__c2: loop {
                    if unsafe {
                                    (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).fg.e_e_name()
                                } as i32 == 0 {
                        sqlite3_rename_token_remap(unsafe { &*p_parse },
                            core::ptr::null(),
                            unsafe {
                                        (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).z_e_name
                                    } as *mut () as *const ());
                    }
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

extern "C" fn rename_walk_with(p_walker_1: *mut Walker, p_select_1: &Select)
    -> () {
    unsafe {
        let p_with: *mut With = (*p_select_1).p_with;
        if !(p_with).is_null() {
            let p_parse: *mut Parse = unsafe { (*p_walker_1).p_parse };
            let mut i: i32 = 0;
            let mut p_copy: *mut With = core::ptr::null_mut();
            { let _ = 0; };
            if unsafe {
                            (*unsafe {
                                            (*(unsafe { (*p_with).a.as_ptr() } as
                                                            *mut Cte).offset(0 as isize)).p_select
                                        }).sel_flags
                        } & 64 as u32 == 0 as u32 {
                p_copy =
                    unsafe {
                        sqlite3_with_dup(unsafe { (*p_parse).db }, p_with)
                    };
                p_copy =
                    unsafe { sqlite3_with_push(p_parse, p_copy, 1 as u8) };
            }
            {
                i = 0;
                '__b3: loop {
                    if !(i < unsafe { (*p_with).n_cte }) { break '__b3; }
                    '__c3: loop {
                        let p: *mut Select =
                            unsafe {
                                (*(unsafe { (*p_with).a.as_ptr() } as
                                                *mut Cte).offset(i as isize)).p_select
                            };
                        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
                        unsafe {
                            memset(&raw mut s_nc as *mut (), 0,
                                core::mem::size_of::<NameContext>() as u64)
                        };
                        s_nc.p_parse = p_parse;
                        if !(p_copy).is_null() {
                            unsafe { sqlite3_select_prep(s_nc.p_parse, p, &mut s_nc) };
                        }
                        if unsafe { (*unsafe { (*s_nc.p_parse).db }).malloc_failed }
                                != 0 {
                            return;
                        }
                        unsafe { sqlite3_walk_select(p_walker_1, p) };
                        sqlite3_rename_exprlist_unmap(p_parse,
                            unsafe {
                                (*(unsafe { (*p_with).a.as_ptr() } as
                                                *mut Cte).offset(i as isize)).p_cols
                            });
                        break '__c3;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if !(p_copy).is_null() && unsafe { (*p_parse).p_with } == p_copy {
                unsafe { (*p_parse).p_with = unsafe { (*p_copy).p_outer } };
            }
        }
    }
}

extern "C" fn rename_column_select_cb(p_walker_1: *mut Walker, p: *mut Select)
    -> i32 {
    if unsafe { (*p).sel_flags } & (2097152 | 67108864) as u32 != 0 {
        return 1;
    }
    rename_walk_with(p_walker_1, unsafe { &*p });
    return 0;
}

extern "C" fn rename_column_parse_error(p_ctx_1: *mut Sqlite3Context,
    z_when_1: *const i8, p_type_1: *mut Sqlite3Value,
    p_object_1: *mut Sqlite3Value, p_parse_1: &Parse) -> () {
    let z_t: *const i8 = unsafe { sqlite3_value_text(p_type_1) } as *const i8;
    let z_n: *const i8 =
        unsafe { sqlite3_value_text(p_object_1) } as *const i8;
    let mut z_err: *mut i8 = core::ptr::null_mut();
    z_err =
        unsafe {
            sqlite3_m_printf((*p_parse_1).db,
                c"error in %s %s%s%s: %s".as_ptr() as *mut i8 as *const i8,
                z_t, z_n,
                if unsafe { *z_when_1.offset(0 as isize) } != 0 {
                    c" ".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 }, z_when_1,
                (*p_parse_1).z_err_msg)
        };
    unsafe { sqlite3_result_error(p_ctx_1, z_err as *const i8, -1) };
    unsafe { sqlite3_db_free((*p_parse_1).db, z_err as *mut ()) };
}

extern "C" fn rename_set_e_names(p_e_list_1: *mut ExprList, val: i32) -> () {
    { let _ = 0; };
    if !(p_e_list_1).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b4: loop {
                if !(i < unsafe { (*p_e_list_1).n_expr }) { break '__b4; }
                '__c4: loop {
                    { let _ = 0; };
                    unsafe {
                        (*(unsafe { (*p_e_list_1).a.as_ptr() } as
                                            *mut ExprListItem).offset(i as
                                            isize)).fg.set_e_e_name((val & 3) as u32 as u32)
                    };
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

extern "C" fn rename_resolve_trigger(p_parse_1: *mut Parse) -> i32 {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let p_new: *const Trigger =
            unsafe { (*p_parse_1).p_new_trigger } as *const Trigger;
        let mut p_step: *mut TriggerStep = core::ptr::null_mut();
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        let mut rc: i32 = 0;
        unsafe {
            memset(&raw mut s_nc as *mut (), 0,
                core::mem::size_of::<NameContext>() as u64)
        };
        s_nc.p_parse = p_parse_1;
        { let _ = 0; };
        unsafe {
            (*p_parse_1).p_trigger_tab =
                unsafe {
                    sqlite3_find_table(db,
                        unsafe { (*p_new).table } as *const i8,
                        unsafe {
                                (*unsafe {
                                            (*db).a_db.offset(unsafe {
                                                        sqlite3_schema_to_index(db,
                                                            unsafe { (*p_new).p_tab_schema })
                                                    } as isize)
                                        }).z_db_s_name
                            } as *const i8)
                }
        };
        unsafe { (*p_parse_1).e_trigger_op = unsafe { (*p_new).op } };
        if !(unsafe { (*p_parse_1).p_trigger_tab }).is_null() {
            rc =
                (unsafe {
                            sqlite3_view_get_column_names(p_parse_1,
                                unsafe { (*p_parse_1).p_trigger_tab })
                        } != 0) as i32;
        }
        if rc == 0 && !(unsafe { (*p_new).p_when }).is_null() {
            rc =
                unsafe {
                    sqlite3_resolve_expr_names(&mut s_nc,
                        unsafe { (*p_new).p_when })
                };
        }
        {
            p_step = unsafe { (*p_new).step_list };
            '__b5: loop {
                if !(rc == 0 && !(p_step).is_null()) { break '__b5; }
                '__c5: loop {
                    if !(unsafe { (*p_step).p_select }).is_null() {
                        unsafe {
                            sqlite3_select_prep(p_parse_1,
                                unsafe { (*p_step).p_select }, &mut s_nc)
                        };
                        if unsafe { (*p_parse_1).n_err } != 0 {
                            rc = unsafe { (*p_parse_1).rc };
                        }
                    }
                    if rc == 0 && !(unsafe { (*p_step).p_src }).is_null() {
                        let mut p_src: *mut SrcList =
                            unsafe {
                                sqlite3_src_list_dup(db,
                                    unsafe { (*p_step).p_src } as *const SrcList, 0)
                            };
                        if !(p_src).is_null() {
                            let p_sel: *mut Select =
                                unsafe {
                                    sqlite3_select_new(p_parse_1,
                                        unsafe { (*p_step).p_expr_list }, p_src,
                                        core::ptr::null_mut(), core::ptr::null_mut(),
                                        core::ptr::null_mut(), core::ptr::null_mut(), 0 as u32,
                                        core::ptr::null_mut())
                                };
                            if p_sel == core::ptr::null_mut() {
                                unsafe { (*p_step).p_expr_list = core::ptr::null_mut() };
                                p_src = core::ptr::null_mut();
                                rc = 7;
                            } else {
                                rename_set_e_names(unsafe { (*p_step).p_expr_list }, 1);
                                unsafe {
                                    sqlite3_select_prep(p_parse_1, p_sel, core::ptr::null_mut())
                                };
                                rename_set_e_names(unsafe { (*p_step).p_expr_list }, 0);
                                rc = if unsafe { (*p_parse_1).n_err } != 0 { 1 } else { 0 };
                                { let _ = 0; };
                                { let _ = 0; };
                                if !(unsafe { (*p_step).p_expr_list }).is_null() {
                                    unsafe { (*p_sel).p_e_list = core::ptr::null_mut() };
                                }
                                unsafe { (*p_sel).p_src = core::ptr::null_mut() };
                                unsafe { sqlite3_select_delete(db, p_sel) };
                            }
                            if !(unsafe { (*p_step).p_src }).is_null() {
                                let mut i: i32 = 0;
                                {
                                    i = 0;
                                    '__b6: loop {
                                        if !(i < unsafe { (*unsafe { (*p_step).p_src }).n_src } &&
                                                        rc == 0) {
                                            break '__b6;
                                        }
                                        '__c6: loop {
                                            let p: *const SrcItem =
                                                unsafe {
                                                        &raw mut *(unsafe {
                                                                            (*unsafe { (*p_step).p_src }).a.as_ptr()
                                                                        } as *mut SrcItem).offset(i as isize)
                                                    } as *const SrcItem;
                                            if unsafe { (*p).fg.is_subquery() } != 0 {
                                                { let _ = 0; };
                                                unsafe {
                                                    sqlite3_select_prep(p_parse_1,
                                                        unsafe { (*unsafe { (*p).u4.p_subq }).p_select },
                                                        core::ptr::null_mut())
                                                };
                                            }
                                            break '__c6;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            }
                            if unsafe { (*db).malloc_failed } != 0 { rc = 7; }
                            s_nc.p_src_list = p_src;
                            if rc == 0 && !(unsafe { (*p_step).p_where }).is_null() {
                                rc =
                                    unsafe {
                                        sqlite3_resolve_expr_names(&mut s_nc,
                                            unsafe { (*p_step).p_where })
                                    };
                            }
                            if rc == 0 {
                                rc =
                                    unsafe {
                                        sqlite3_resolve_expr_list_names(&mut s_nc,
                                            unsafe { (*p_step).p_expr_list })
                                    };
                            }
                            { let _ = 0; };
                            if !(unsafe { (*p_step).p_upsert }).is_null() && rc == 0 {
                                let p_upsert: *mut Upsert = unsafe { (*p_step).p_upsert };
                                unsafe { (*p_upsert).p_upsert_src = p_src };
                                s_nc.u_nc.p_upsert = p_upsert;
                                s_nc.nc_flags = 512;
                                rc =
                                    unsafe {
                                        sqlite3_resolve_expr_list_names(&mut s_nc,
                                            unsafe { (*p_upsert).p_upsert_target })
                                    };
                                if rc == 0 {
                                    let p_upsert_set: *mut ExprList =
                                        unsafe { (*p_upsert).p_upsert_set };
                                    rc =
                                        unsafe {
                                            sqlite3_resolve_expr_list_names(&mut s_nc, p_upsert_set)
                                        };
                                }
                                if rc == 0 {
                                    rc =
                                        unsafe {
                                            sqlite3_resolve_expr_names(&mut s_nc,
                                                unsafe { (*p_upsert).p_upsert_where })
                                        };
                                }
                                if rc == 0 {
                                    rc =
                                        unsafe {
                                            sqlite3_resolve_expr_names(&mut s_nc,
                                                unsafe { (*p_upsert).p_upsert_target_where })
                                        };
                                }
                                s_nc.nc_flags = 0;
                            }
                            s_nc.p_src_list = core::ptr::null_mut();
                            unsafe { sqlite3_src_list_delete(db, p_src) };
                        } else { rc = 7; }
                    }
                    break '__c5;
                }
                p_step = unsafe { (*p_step).p_next };
            }
        }
        return rc;
    }
}

extern "C" fn rename_column_elist_names(p_parse_1: *mut Parse,
    p_ctx_1: *mut RenameCtx, p_e_list_1: *const ExprList, z_old_1: *const i8)
    -> () {
    if !(p_e_list_1).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b7: loop {
                if !(i < unsafe { (*p_e_list_1).n_expr }) { break '__b7; }
                '__c7: loop {
                    let z_name: *const i8 =
                        unsafe {
                                (*(unsafe { (*p_e_list_1).a.as_ptr() } as
                                                *const ExprListItem).offset(i as isize)).z_e_name
                            } as *const i8;
                    if unsafe {
                                            (*(unsafe { (*p_e_list_1).a.as_ptr() } as
                                                                *const ExprListItem).offset(i as isize)).fg.e_e_name()
                                        } as i32 == 0 && z_name != core::ptr::null() &&
                            0 == unsafe { sqlite3_stricmp(z_name, z_old_1) } {
                        rename_token_find(p_parse_1, p_ctx_1, z_name as *const ());
                    }
                    break '__c7;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

extern "C" fn rename_column_idlist_names(p_parse_1: *mut Parse,
    p_ctx_1: *mut RenameCtx, p_id_list_1: *const IdList, z_old_1: *const i8)
    -> () {
    if !(p_id_list_1).is_null() {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b8: loop {
                if !(i < unsafe { (*p_id_list_1).n_id }) { break '__b8; }
                '__c8: loop {
                    let z_name: *const i8 =
                        unsafe {
                                (*(unsafe { (*p_id_list_1).a.as_ptr() } as
                                                *const IdListItem).offset(i as isize)).z_name
                            } as *const i8;
                    if 0 == unsafe { sqlite3_stricmp(z_name, z_old_1) } {
                        rename_token_find(p_parse_1, p_ctx_1, z_name as *const ());
                    }
                    break '__c8;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}

extern "C" fn rename_walk_trigger(p_walker_1: *mut Walker,
    p_trigger_1: &Trigger) -> () {
    unsafe {
        let mut p_step: *const TriggerStep = core::ptr::null();
        unsafe { sqlite3_walk_expr(p_walker_1, (*p_trigger_1).p_when) };
        {
            p_step = (*p_trigger_1).step_list;
            '__b9: loop {
                if !(!(p_step).is_null()) { break '__b9; }
                '__c9: loop {
                    unsafe {
                        sqlite3_walk_select(p_walker_1,
                            unsafe { (*p_step).p_select })
                    };
                    unsafe {
                        sqlite3_walk_expr(p_walker_1, unsafe { (*p_step).p_where })
                    };
                    unsafe {
                        sqlite3_walk_expr_list(p_walker_1,
                            unsafe { (*p_step).p_expr_list })
                    };
                    if !(unsafe { (*p_step).p_upsert }).is_null() {
                        let p_upsert: *const Upsert =
                            unsafe { (*p_step).p_upsert } as *const Upsert;
                        unsafe {
                            sqlite3_walk_expr_list(p_walker_1,
                                unsafe { (*p_upsert).p_upsert_target })
                        };
                        unsafe {
                            sqlite3_walk_expr_list(p_walker_1,
                                unsafe { (*p_upsert).p_upsert_set })
                        };
                        unsafe {
                            sqlite3_walk_expr(p_walker_1,
                                unsafe { (*p_upsert).p_upsert_where })
                        };
                        unsafe {
                            sqlite3_walk_expr(p_walker_1,
                                unsafe { (*p_upsert).p_upsert_target_where })
                        };
                    }
                    if !(unsafe { (*p_step).p_src }).is_null() {
                        let mut i: i32 = 0;
                        let p_src: *const SrcList =
                            unsafe { (*p_step).p_src } as *const SrcList;
                        {
                            i = 0;
                            '__b10: loop {
                                if !(i < unsafe { (*p_src).n_src }) { break '__b10; }
                                '__c10: loop {
                                    if unsafe {
                                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                                    *mut SrcItem).offset(i as isize)).fg.is_subquery()
                                            } != 0 {
                                        { let _ = 0; };
                                        unsafe {
                                            sqlite3_walk_select(p_walker_1,
                                                unsafe {
                                                    (*unsafe {
                                                                    (*(unsafe { (*p_src).a.as_ptr() } as
                                                                                        *mut SrcItem).offset(i as isize)).u4.p_subq
                                                                }).p_select
                                                })
                                        };
                                    }
                                    break '__c10;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                    break '__c9;
                }
                p_step = unsafe { (*p_step).p_next };
            }
        }
    }
}

extern "C" fn rename_column_token_next(p_ctx_1: &mut RenameCtx)
    -> *mut RenameToken {
    unsafe {
        let mut p_best: *mut RenameToken = (*p_ctx_1).p_list;
        let mut p_token: *mut RenameToken = core::ptr::null_mut();
        let mut pp: *mut *mut RenameToken = core::ptr::null_mut();
        {
            p_token = unsafe { (*p_best).p_next };
            '__b11: loop {
                if !(!(p_token).is_null()) { break '__b11; }
                '__c11: loop {
                    if unsafe { (*p_token).t.z } > unsafe { (*p_best).t.z } {
                        p_best = p_token;
                    }
                    break '__c11;
                }
                p_token = unsafe { (*p_token).p_next };
            }
        }
        {
            pp = &mut (*p_ctx_1).p_list;
            '__b12: loop {
                if !(unsafe { *pp } != p_best) { break '__b12; }
                '__c12: loop { break '__c12; }
                pp = unsafe { &mut (*unsafe { *pp }).p_next };
            }
        }
        unsafe { *pp = unsafe { (*p_best).p_next } };
        return p_best;
    }
}

extern "C" fn rename_edit_sql(p_ctx_1: *mut Sqlite3Context,
    p_rename_1: *mut RenameCtx, z_sql_1: *const i8, z_new_1: *const i8,
    b_quote_1: i32) -> i32 {
    unsafe {
        let n_new: i64 = unsafe { sqlite3_strlen30(z_new_1) } as i64;
        let n_sql: i64 = unsafe { sqlite3_strlen30(z_sql_1) } as i64;
        let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(p_ctx_1) };
        let mut rc: i32 = 0;
        let mut z_quot: *mut i8 = core::ptr::null_mut();
        let mut z_out: *mut i8 = core::ptr::null_mut();
        let mut n_quot: i64 = 0 as i64;
        let mut z_buf1: *mut i8 = core::ptr::null_mut();
        let mut z_buf2: *mut i8 = core::ptr::null_mut();
        if !(z_new_1).is_null() {
            z_quot =
                unsafe {
                    sqlite3_m_printf(db,
                        c"\"%w\" ".as_ptr() as *mut i8 as *const i8, z_new_1)
                };
            if z_quot == core::ptr::null_mut() {
                return 7;
            } else {
                n_quot =
                    (unsafe { sqlite3_strlen30(z_quot as *const i8) } - 1) as
                        i64;
            }
            { let _ = 0; };
            z_out =
                unsafe {
                        sqlite3_db_malloc_zero(db,
                            n_sql as u64 +
                                    unsafe { (*p_rename_1).n_list } as u64 * n_quot as u64 +
                                1 as u64)
                    } as *mut i8;
        } else {
            { let _ = 0; };
            z_out =
                unsafe {
                        sqlite3_db_malloc_zero(db,
                            (2 as u64 * n_sql as u64 + 1 as u64) * 3 as u64)
                    } as *mut i8;
            if !(z_out).is_null() {
                z_buf1 =
                    unsafe {
                        z_out.offset((n_sql * 2 as i64 + 1 as i64) as isize)
                    };
                z_buf2 =
                    unsafe {
                        z_out.offset((n_sql * 4 as i64 + 2 as i64) as isize)
                    };
            }
        }
        if !(z_out).is_null() {
            let mut n_out: i64 = n_sql;
            { let _ = 0; };
            unsafe {
                memcpy(z_out as *mut (), z_sql_1 as *const (), n_sql as u64)
            };
            while !(unsafe { (*p_rename_1).p_list }).is_null() {
                let mut i_off: i32 = 0;
                let mut n_replace: i64 = 0 as i64;
                let mut z_replace: *const i8 = core::ptr::null();
                let p_best: *mut RenameToken =
                    rename_column_token_next(unsafe { &mut *p_rename_1 });
                if !(z_new_1).is_null() {
                    if b_quote_1 == 0 &&
                            unsafe {
                                    sqlite3_is_id_char(unsafe {
                                            *(unsafe { (*p_best).t.z } as *mut u8)
                                        })
                                } != 0 {
                        n_replace = n_new;
                        z_replace = z_new_1;
                    } else {
                        n_replace = n_quot;
                        z_replace = z_quot as *const i8;
                        if unsafe {
                                        *unsafe {
                                                (*p_best).t.z.add(unsafe { (*p_best).t.n } as usize)
                                            }
                                    } as i32 == '\"' as i32 {
                            {
                                let __p = &mut n_replace;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        }
                    }
                } else {
                    unsafe {
                        memcpy(z_buf1 as *mut (),
                            unsafe { (*p_best).t.z } as *const (),
                            unsafe { (*p_best).t.n } as u64)
                    };
                    unsafe {
                        *z_buf1.add(unsafe { (*p_best).t.n } as usize) = 0 as i8
                    };
                    unsafe { sqlite3_dequote(z_buf1) };
                    { let _ = 0; };
                    unsafe {
                        sqlite3_snprintf((n_sql * 2 as i64) as i32, z_buf2,
                            c"%Q%s".as_ptr() as *mut i8 as *const i8, z_buf1,
                            if unsafe {
                                            *unsafe {
                                                    (*p_best).t.z.add(unsafe { (*p_best).t.n } as usize)
                                                }
                                        } as i32 == '\'' as i32 {
                                c" ".as_ptr() as *mut i8
                            } else { c"".as_ptr() as *mut i8 })
                    };
                    z_replace = z_buf2 as *const i8;
                    n_replace = unsafe { sqlite3_strlen30(z_replace) } as i64;
                }
                i_off =
                    unsafe { unsafe { (*p_best).t.z.offset_from(z_sql_1) } } as
                            i64 as i32;
                if unsafe { (*p_best).t.n } as i64 != n_replace {
                    unsafe {
                        memmove(unsafe {
                                    &raw mut *z_out.offset((i_off as i64 + n_replace) as isize)
                                } as *mut (),
                            unsafe {
                                    &raw mut *z_out.add((i_off as u32 +
                                                        unsafe { (*p_best).t.n }) as usize)
                                } as *const (),
                            (n_out - (i_off as u32 + unsafe { (*p_best).t.n }) as i64)
                                as u64)
                    };
                    n_out += n_replace - unsafe { (*p_best).t.n } as i64;
                    unsafe {
                        *z_out.offset(n_out as isize) = '\u{0}' as i32 as i8
                    };
                }
                unsafe {
                    memcpy(unsafe { &raw mut *z_out.offset(i_off as isize) } as
                            *mut (), z_replace as *const (), n_replace as u64)
                };
                unsafe { sqlite3_db_free(db, p_best as *mut ()) };
            }
            unsafe {
                sqlite3_result_text(p_ctx_1, z_out as *const i8, -1,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }))
            };
            unsafe { sqlite3_db_free(db, z_out as *mut ()) };
        } else { rc = 7; }
        unsafe { sqlite3_free(z_quot as *mut ()) };
        return rc;
    }
}

extern "C" fn rename_token_free(db: *mut Sqlite3, p_token_1: *mut RenameToken)
    -> () {
    let mut p_next: *mut RenameToken = core::ptr::null_mut();
    let mut p: *mut RenameToken = core::ptr::null_mut();
    {
        p = p_token_1;
        '__b14: loop {
            if !(!(p).is_null()) { break '__b14; }
            '__c14: loop {
                p_next = unsafe { (*p).p_next };
                unsafe { sqlite3_db_free(db, p as *mut ()) };
                break '__c14;
            }
            p = p_next;
        }
    }
}

extern "C" fn rename_parse_cleanup(p_parse_1: *mut Parse) -> () {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let mut p_idx: *mut Index = core::ptr::null_mut();
        if !(unsafe { (*p_parse_1).p_vdbe }).is_null() {
            unsafe { sqlite3_vdbe_finalize(unsafe { (*p_parse_1).p_vdbe }) };
        }
        unsafe {
            sqlite3_delete_table(db, unsafe { (*p_parse_1).p_new_table })
        };
        while { p_idx = unsafe { (*p_parse_1).p_new_index }; p_idx } !=
                core::ptr::null_mut() {
            unsafe { (*p_parse_1).p_new_index = unsafe { (*p_idx).p_next } };
            unsafe { sqlite3_free_index(db, p_idx) };
        }
        unsafe {
            sqlite3_delete_trigger(db, unsafe { (*p_parse_1).p_new_trigger })
        };
        unsafe {
            sqlite3_db_free(db, unsafe { (*p_parse_1).z_err_msg } as *mut ())
        };
        rename_token_free(db, unsafe { (*p_parse_1).p_rename });
        unsafe { sqlite3_parse_object_reset(p_parse_1) };
    }
}

extern "C" fn rename_column_func(context: *mut Sqlite3Context,
    not_used_1: i32, argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut s_ctx: RenameCtx = unsafe { core::mem::zeroed() };
        let mut z_sql: *const i8 = core::ptr::null();
        let mut z_db: *const i8 = core::ptr::null();
        let mut z_table: *const i8 = core::ptr::null();
        let mut i_col: i32 = 0;
        let mut z_new: *const i8 = core::ptr::null();
        let mut b_quote: i32 = 0;
        let mut b_temp: i32 = 0;
        let mut z_old: *const i8 = core::ptr::null();
        let mut rc: i32 = 0;
        let mut s_parse: Parse = unsafe { core::mem::zeroed() };
        let mut s_walker: Walker = unsafe { core::mem::zeroed() };
        let mut p_idx: *const Index = core::ptr::null();
        let mut i: i32 = 0;
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut x_auth:
                unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                    *const i8, *const i8) -> i32 =
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                            *const i8, *const i8) -> i32>(0 as *const ())
            };
        let mut p_select: *mut Select = core::ptr::null_mut();
        let mut b_fk_only: i32 = 0;
        let mut p_f_key: *mut FKey = core::ptr::null_mut();
        let mut p_expr: *mut Expr = core::ptr::null_mut();
        let mut p_step: *const TriggerStep = core::ptr::null();
        let mut p_target: *mut Table = core::ptr::null_mut();
        let mut p_upsert_set: *const ExprList = core::ptr::null();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s17:
                {
                match __state {
                    0 => {
                        db = unsafe { sqlite3_context_db_handle(context) };
                        __state = 3;
                    }
                    2 => {
                        if rc != 0 { __state = 116; } else { __state = 115; }
                    }
                    3 => { __state = 4; }
                    4 => {
                        z_sql =
                            unsafe {
                                    sqlite3_value_text(unsafe { *argv.offset(0 as isize) })
                                } as *const i8;
                        __state = 5;
                    }
                    5 => {
                        z_db =
                            unsafe {
                                    sqlite3_value_text(unsafe { *argv.offset(3 as isize) })
                                } as *const i8;
                        __state = 6;
                    }
                    6 => {
                        z_table =
                            unsafe {
                                    sqlite3_value_text(unsafe { *argv.offset(4 as isize) })
                                } as *const i8;
                        __state = 7;
                    }
                    7 => {
                        i_col =
                            unsafe {
                                sqlite3_value_int(unsafe { *argv.offset(5 as isize) })
                            };
                        __state = 8;
                    }
                    8 => {
                        z_new =
                            unsafe {
                                    sqlite3_value_text(unsafe { *argv.offset(6 as isize) })
                                } as *const i8;
                        __state = 9;
                    }
                    9 => {
                        b_quote =
                            unsafe {
                                sqlite3_value_int(unsafe { *argv.offset(7 as isize) })
                            };
                        __state = 10;
                    }
                    10 => {
                        b_temp =
                            unsafe {
                                sqlite3_value_int(unsafe { *argv.offset(8 as isize) })
                            };
                        __state = 11;
                    }
                    11 => { __state = 12; }
                    12 => { __state = 13; }
                    13 => { __state = 14; }
                    14 => { __state = 15; }
                    15 => { __state = 16; }
                    16 => { __state = 17; }
                    17 => { __state = 18; }
                    18 => { x_auth = unsafe { (*db).x_auth }; __state = 19; }
                    19 => { { let _ = not_used_1; }; __state = 20; }
                    20 => {
                        if z_sql == core::ptr::null() {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => {
                        if z_table == core::ptr::null() {
                            __state = 24;
                        } else { __state = 23; }
                    }
                    22 => { return; }
                    23 => {
                        if z_new == core::ptr::null() {
                            __state = 26;
                        } else { __state = 25; }
                    }
                    24 => { return; }
                    25 => {
                        if i_col < 0 { __state = 28; } else { __state = 27; }
                    }
                    26 => { return; }
                    27 => {
                        unsafe { sqlite3_btree_enter_all(db) };
                        __state = 29;
                    }
                    28 => { return; }
                    29 => {
                        p_tab = unsafe { sqlite3_find_table(db, z_table, z_db) };
                        __state = 30;
                    }
                    30 => {
                        if p_tab == core::ptr::null_mut() ||
                                i_col >= unsafe { (*p_tab).n_col } as i32 {
                            __state = 32;
                        } else { __state = 31; }
                    }
                    31 => {
                        z_old =
                            unsafe {
                                    (*unsafe {
                                                (*p_tab).a_col.offset(i_col as isize)
                                            }).z_cn_name
                                } as *const i8;
                        __state = 34;
                    }
                    32 => {
                        unsafe { sqlite3_btree_leave_all(db) };
                        __state = 33;
                    }
                    33 => { return; }
                    34 => {
                        unsafe {
                            memset(&raw mut s_ctx as *mut (), 0,
                                core::mem::size_of::<RenameCtx>() as u64)
                        };
                        __state = 35;
                    }
                    35 => {
                        s_ctx.i_col =
                            if i_col == unsafe { (*p_tab).i_p_key } as i32 {
                                -1
                            } else { i_col };
                        __state = 36;
                    }
                    36 => {
                        unsafe {
                            (*db).x_auth =
                                unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                                                *const i8, *const i8) -> i32>(0 as *const ())
                                }
                        };
                        __state = 37;
                    }
                    37 => {
                        rc =
                            rename_parse_sql(&mut s_parse, z_db, db, z_sql, b_temp);
                        __state = 38;
                    }
                    38 => {
                        unsafe {
                            memset(&raw mut s_walker as *mut (), 0,
                                core::mem::size_of::<Walker>() as u64)
                        };
                        __state = 39;
                    }
                    39 => { s_walker.p_parse = &mut s_parse; __state = 40; }
                    40 => {
                        s_walker.x_expr_callback = Some(rename_column_expr_cb);
                        __state = 41;
                    }
                    41 => {
                        s_walker.x_select_callback = Some(rename_column_select_cb);
                        __state = 42;
                    }
                    42 => { s_walker.u.p_rename = &mut s_ctx; __state = 43; }
                    43 => { s_ctx.p_tab = p_tab; __state = 44; }
                    44 => {
                        if rc != 0 { __state = 46; } else { __state = 45; }
                    }
                    45 => {
                        if !(s_parse.p_new_table).is_null() {
                            __state = 48;
                        } else { __state = 49; }
                    }
                    46 => { __state = 2; }
                    47 => { { let _ = 0; }; __state = 113; }
                    48 => {
                        if unsafe { (*s_parse.p_new_table).e_tab_type } as i32 == 2
                            {
                            __state = 50;
                        } else { __state = 51; }
                    }
                    49 => {
                        if !(s_parse.p_new_index).is_null() {
                            __state = 93;
                        } else { __state = 94; }
                    }
                    50 => {
                        p_select =
                            unsafe { (*s_parse.p_new_table).u.view.p_select };
                        __state = 52;
                    }
                    51 => {
                        if unsafe { (*s_parse.p_new_table).e_tab_type } as i32 == 0
                            {
                            __state = 60;
                        } else { __state = 47; }
                    }
                    52 => {
                        unsafe { (*p_select).sel_flags &= !(2097152 as u32) };
                        __state = 53;
                    }
                    53 => { s_parse.rc = 0; __state = 54; }
                    54 => {
                        unsafe {
                            sqlite3_select_prep(&mut s_parse, p_select,
                                core::ptr::null_mut())
                        };
                        __state = 55;
                    }
                    55 => {
                        rc =
                            if unsafe { (*db).malloc_failed } != 0 {
                                7
                            } else { s_parse.rc };
                        __state = 56;
                    }
                    56 => {
                        if rc == 0 { __state = 58; } else { __state = 57; }
                    }
                    57 => {
                        if rc != 0 { __state = 59; } else { __state = 47; }
                    }
                    58 => {
                        unsafe { sqlite3_walk_select(&mut s_walker, p_select) };
                        __state = 57;
                    }
                    59 => { __state = 2; }
                    60 => {
                        b_fk_only =
                            unsafe {
                                sqlite3_stricmp(z_table,
                                    unsafe { (*s_parse.p_new_table).z_name } as *const i8)
                            };
                        __state = 61;
                    }
                    61 => { __state = 62; }
                    62 => { s_ctx.p_tab = s_parse.p_new_table; __state = 63; }
                    63 => {
                        if b_fk_only == 0 { __state = 65; } else { __state = 64; }
                    }
                    64 => { { let _ = 0; }; __state = 83; }
                    65 => {
                        if i_col < unsafe { (*s_parse.p_new_table).n_col } as i32 {
                            __state = 67;
                        } else { __state = 66; }
                    }
                    66 => {
                        if s_ctx.i_col < 0 { __state = 69; } else { __state = 68; }
                    }
                    67 => {
                        rename_token_find(&mut s_parse, &mut s_ctx,
                            unsafe {
                                        (*unsafe {
                                                    (*s_parse.p_new_table).a_col.offset(i_col as isize)
                                                }).z_cn_name
                                    } as *mut () as *const ());
                        __state = 66;
                    }
                    68 => {
                        unsafe {
                            sqlite3_walk_expr_list(&mut s_walker,
                                unsafe { (*s_parse.p_new_table).p_check })
                        };
                        __state = 70;
                    }
                    69 => {
                        rename_token_find(&mut s_parse, &mut s_ctx,
                            unsafe { &raw mut (*s_parse.p_new_table).i_p_key } as
                                    *mut () as *const ());
                        __state = 68;
                    }
                    70 => {
                        p_idx = unsafe { (*s_parse.p_new_table).p_index };
                        __state = 72;
                    }
                    71 => { p_idx = s_parse.p_new_index; __state = 76; }
                    72 => {
                        if !(p_idx).is_null() {
                            __state = 73;
                        } else { __state = 71; }
                    }
                    73 => {
                        unsafe {
                            sqlite3_walk_expr_list(&mut s_walker,
                                unsafe { (*p_idx).a_col_expr })
                        };
                        __state = 74;
                    }
                    74 => { p_idx = unsafe { (*p_idx).p_next }; __state = 72; }
                    75 => { i = 0; __state = 79; }
                    76 => {
                        if !(p_idx).is_null() {
                            __state = 77;
                        } else { __state = 75; }
                    }
                    77 => {
                        unsafe {
                            sqlite3_walk_expr_list(&mut s_walker,
                                unsafe { (*p_idx).a_col_expr })
                        };
                        __state = 78;
                    }
                    78 => { p_idx = unsafe { (*p_idx).p_next }; __state = 76; }
                    79 => {
                        if i < unsafe { (*s_parse.p_new_table).n_col } as i32 {
                            __state = 80;
                        } else { __state = 64; }
                    }
                    80 => {
                        p_expr =
                            unsafe {
                                sqlite3_column_expr(s_parse.p_new_table,
                                    unsafe {
                                        &mut *unsafe {
                                                    (*s_parse.p_new_table).a_col.offset(i as isize)
                                                }
                                    })
                            };
                        __state = 82;
                    }
                    81 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 79;
                    }
                    82 => {
                        unsafe { sqlite3_walk_expr(&mut s_walker, p_expr) };
                        __state = 81;
                    }
                    83 => {
                        p_f_key = unsafe { (*s_parse.p_new_table).u.tab.p_f_key };
                        __state = 84;
                    }
                    84 => {
                        if !(p_f_key).is_null() {
                            __state = 85;
                        } else { __state = 47; }
                    }
                    85 => { i = 0; __state = 87; }
                    86 => {
                        p_f_key = unsafe { (*p_f_key).p_next_from };
                        __state = 84;
                    }
                    87 => {
                        if i < unsafe { (*p_f_key).n_col } {
                            __state = 88;
                        } else { __state = 86; }
                    }
                    88 => {
                        if b_fk_only == 0 &&
                                unsafe {
                                        (*(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                        *mut SColMap).offset(i as isize)).i_from
                                    } == i_col {
                            __state = 91;
                        } else { __state = 90; }
                    }
                    89 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 87;
                    }
                    90 => {
                        if 0 ==
                                    unsafe {
                                        sqlite3_stricmp(unsafe { (*p_f_key).z_to } as *const i8,
                                            z_table)
                                    } &&
                                0 ==
                                    unsafe {
                                        sqlite3_stricmp(unsafe {
                                                    (*(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                                    *mut SColMap).offset(i as isize)).z_col
                                                } as *const i8, z_old)
                                    } {
                            __state = 92;
                        } else { __state = 89; }
                    }
                    91 => {
                        rename_token_find(&mut s_parse, &mut s_ctx,
                            unsafe {
                                        &raw mut *(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                        *mut SColMap).offset(i as isize)
                                    } as *mut () as *const ());
                        __state = 90;
                    }
                    92 => {
                        rename_token_find(&mut s_parse, &mut s_ctx,
                            unsafe {
                                        (*(unsafe { (*p_f_key).a_col.as_ptr() } as
                                                        *mut SColMap).offset(i as isize)).z_col
                                    } as *mut () as *const ());
                        __state = 89;
                    }
                    93 => {
                        unsafe {
                            sqlite3_walk_expr_list(&mut s_walker,
                                unsafe { (*s_parse.p_new_index).a_col_expr })
                        };
                        __state = 95;
                    }
                    94 => { __state = 96; }
                    95 => {
                        unsafe {
                            sqlite3_walk_expr(&mut s_walker,
                                unsafe { (*s_parse.p_new_index).p_part_idx_where })
                        };
                        __state = 47;
                    }
                    96 => {
                        rc = rename_resolve_trigger(&mut s_parse);
                        __state = 97;
                    }
                    97 => {
                        if rc != 0 { __state = 99; } else { __state = 98; }
                    }
                    98 => {
                        p_step = unsafe { (*s_parse.p_new_trigger).step_list };
                        __state = 101;
                    }
                    99 => { __state = 2; }
                    100 => {
                        if s_parse.p_trigger_tab == p_tab {
                            __state = 112;
                        } else { __state = 111; }
                    }
                    101 => {
                        if !(p_step).is_null() {
                            __state = 102;
                        } else { __state = 100; }
                    }
                    102 => {
                        if !(unsafe { (*p_step).p_src }).is_null() {
                            __state = 104;
                        } else { __state = 103; }
                    }
                    103 => {
                        p_step = unsafe { (*p_step).p_next };
                        __state = 101;
                    }
                    104 => {
                        p_target =
                            unsafe {
                                sqlite3_locate_table_item(&mut s_parse, 0 as u32,
                                    unsafe {
                                        &mut *(unsafe { (*unsafe { (*p_step).p_src }).a.as_ptr() }
                                                        as *mut SrcItem).offset(0 as isize)
                                    })
                            };
                        __state = 105;
                    }
                    105 => {
                        if p_target == p_tab {
                            __state = 106;
                        } else { __state = 103; }
                    }
                    106 => {
                        if !(unsafe { (*p_step).p_upsert }).is_null() {
                            __state = 108;
                        } else { __state = 107; }
                    }
                    107 => {
                        rename_column_idlist_names(&mut s_parse, &mut s_ctx,
                            unsafe { (*p_step).p_id_list } as *const IdList, z_old);
                        __state = 110;
                    }
                    108 => {
                        p_upsert_set =
                            unsafe { (*unsafe { (*p_step).p_upsert }).p_upsert_set } as
                                *const ExprList;
                        __state = 109;
                    }
                    109 => {
                        rename_column_elist_names(&mut s_parse, &mut s_ctx,
                            p_upsert_set as *const ExprList, z_old);
                        __state = 107;
                    }
                    110 => {
                        rename_column_elist_names(&mut s_parse, &mut s_ctx,
                            unsafe { (*p_step).p_expr_list } as *const ExprList, z_old);
                        __state = 103;
                    }
                    111 => {
                        rename_walk_trigger(&mut s_walker,
                            unsafe { &*s_parse.p_new_trigger });
                        __state = 47;
                    }
                    112 => {
                        rename_column_idlist_names(&mut s_parse, &mut s_ctx,
                            unsafe { (*s_parse.p_new_trigger).p_columns } as
                                *const IdList, z_old);
                        __state = 111;
                    }
                    113 => {
                        rc =
                            rename_edit_sql(context, &mut s_ctx, z_sql, z_new, b_quote);
                        __state = 114;
                    }
                    114 => { __state = 2; }
                    115 => {
                        rename_parse_cleanup(&mut s_parse);
                        __state = 121;
                    }
                    116 => {
                        if rc == 1 && unsafe { sqlite3_writable_schema(db) } != 0 {
                            __state = 117;
                        } else { __state = 118; }
                    }
                    117 => {
                        unsafe {
                            sqlite3_result_value(context,
                                unsafe { *argv.offset(0 as isize) })
                        };
                        __state = 115;
                    }
                    118 => {
                        if !(s_parse.z_err_msg).is_null() {
                            __state = 119;
                        } else { __state = 120; }
                    }
                    119 => {
                        rename_column_parse_error(context,
                            c"".as_ptr() as *mut i8 as *const i8,
                            unsafe { *argv.offset(1 as isize) },
                            unsafe { *argv.offset(2 as isize) }, &s_parse);
                        __state = 115;
                    }
                    120 => {
                        unsafe { sqlite3_result_error_code(context, rc) };
                        __state = 115;
                    }
                    121 => {
                        rename_token_free(db, s_ctx.p_list);
                        __state = 122;
                    }
                    122 => { unsafe { (*db).x_auth = x_auth }; __state = 123; }
                    123 => {
                        unsafe { sqlite3_btree_leave_all(db) };
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

extern "C" fn rename_table_expr_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        let p: *mut RenameCtx = unsafe { (*p_walker_1).u.p_rename };
        if unsafe { (*p_expr_1).op } as i32 == 168 &&
                    unsafe { (*p_expr_1).flags } & (16777216 | 33554432) as u32
                        == 0 as u32 &&
                unsafe { (*p).p_tab } == unsafe { (*p_expr_1).y.p_tab } {
            rename_token_find(unsafe { (*p_walker_1).p_parse }, p,
                unsafe { &raw mut (*p_expr_1).y.p_tab } as *mut () as
                    *const ());
        }
        return 0;
    }
}

extern "C" fn rename_table_select_cb(p_walker_1: *mut Walker,
    p_select_1: *mut Select) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let p: *mut RenameCtx = unsafe { (*p_walker_1).u.p_rename };
        let p_src: *mut SrcList = unsafe { (*p_select_1).p_src };
        if unsafe { (*p_select_1).sel_flags } & (2097152 | 67108864) as u32 !=
                0 {
            return 1;
        }
        if p_src == core::ptr::null_mut() { { let _ = 0; }; return 2; }
        {
            i = 0;
            '__b18: loop {
                if !(i < unsafe { (*p_src).n_src }) { break '__b18; }
                '__c18: loop {
                    let p_item: *const SrcItem =
                        unsafe {
                                &raw mut *(unsafe { (*p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(i as isize)
                            } as *const SrcItem;
                    if unsafe { (*p_item).p_s_tab } == unsafe { (*p).p_tab } {
                        rename_token_find(unsafe { (*p_walker_1).p_parse }, p,
                            unsafe { (*p_item).z_name } as *const ());
                    }
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        rename_walk_with(p_walker_1, unsafe { &*p_select_1 });
        return 0;
    }
}

extern "C" fn rename_table_func(context: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
        let z_db: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        let z_input: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(3 as isize) }) }
                as *const i8;
        let z_old: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(4 as isize) }) }
                as *const i8;
        let z_new: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(5 as isize) }) }
                as *const i8;
        let b_temp: i32 =
            unsafe { sqlite3_value_int(unsafe { *argv.offset(6 as isize) }) };
        { let _ = not_used_1; };
        if !(z_input).is_null() && !(z_old).is_null() && !(z_new).is_null() {
            let mut s_parse: Parse = unsafe { core::mem::zeroed() };
            let mut rc: i32 = 0;
            let b_quote: i32 = 1;
            let mut s_ctx: RenameCtx = unsafe { core::mem::zeroed() };
            let mut s_walker: Walker = unsafe { core::mem::zeroed() };
            let x_auth:
                    unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                        *const i8, *const i8) -> i32 = unsafe { (*db).x_auth };
            unsafe {
                (*db).x_auth =
                    unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                                    *const i8, *const i8) -> i32>(0 as *const ())
                    }
            };
            unsafe { sqlite3_btree_enter_all(db) };
            unsafe {
                memset(&raw mut s_ctx as *mut (), 0,
                    core::mem::size_of::<RenameCtx>() as u64)
            };
            s_ctx.p_tab = unsafe { sqlite3_find_table(db, z_old, z_db) };
            unsafe {
                memset(&raw mut s_walker as *mut (), 0,
                    core::mem::size_of::<Walker>() as u64)
            };
            s_walker.p_parse = &mut s_parse;
            s_walker.x_expr_callback = Some(rename_table_expr_cb);
            s_walker.x_select_callback = Some(rename_table_select_cb);
            s_walker.u.p_rename = &mut s_ctx;
            rc = rename_parse_sql(&mut s_parse, z_db, db, z_input, b_temp);
            if rc == 0 {
                let is_legacy: i32 =
                    (unsafe { (*db).flags } & 67108864 as u64) as i32;
                if !(s_parse.p_new_table).is_null() {
                    let p_tab: *mut Table = s_parse.p_new_table;
                    if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                        if is_legacy == 0 {
                            let p_select: *mut Select =
                                unsafe { (*p_tab).u.view.p_select };
                            let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
                            unsafe {
                                memset(&raw mut s_nc as *mut (), 0,
                                    core::mem::size_of::<NameContext>() as u64)
                            };
                            s_nc.p_parse = &mut s_parse;
                            { let _ = 0; };
                            unsafe { (*p_select).sel_flags &= !(2097152 as u32) };
                            unsafe {
                                sqlite3_select_prep(&mut s_parse,
                                    unsafe { (*p_tab).u.view.p_select }, &mut s_nc)
                            };
                            if s_parse.n_err != 0 {
                                rc = s_parse.rc;
                            } else {
                                unsafe {
                                    sqlite3_walk_select(&mut s_walker,
                                        unsafe { (*p_tab).u.view.p_select })
                                };
                            }
                        }
                    } else {
                        if (is_legacy == 0 ||
                                    unsafe { (*db).flags } & 16384 as u64 != 0) &&
                                !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0 {
                            let mut p_f_key: *mut FKey = core::ptr::null_mut();
                            { let _ = 0; };
                            {
                                p_f_key = unsafe { (*p_tab).u.tab.p_f_key };
                                '__b19: loop {
                                    if !(!(p_f_key).is_null()) { break '__b19; }
                                    '__c19: loop {
                                        if unsafe {
                                                    sqlite3_stricmp(unsafe { (*p_f_key).z_to } as *const i8,
                                                        z_old)
                                                } == 0 {
                                            rename_token_find(&mut s_parse, &mut s_ctx,
                                                unsafe { (*p_f_key).z_to } as *mut () as *const ());
                                        }
                                        break '__c19;
                                    }
                                    p_f_key = unsafe { (*p_f_key).p_next_from };
                                }
                            }
                        }
                        if unsafe {
                                    sqlite3_stricmp(z_old,
                                        unsafe { (*p_tab).z_name } as *const i8)
                                } == 0 {
                            s_ctx.p_tab = p_tab;
                            if is_legacy == 0 {
                                unsafe {
                                    sqlite3_walk_expr_list(&mut s_walker,
                                        unsafe { (*p_tab).p_check })
                                };
                            }
                            rename_token_find(&mut s_parse, &mut s_ctx,
                                unsafe { (*p_tab).z_name } as *const ());
                        }
                    }
                } else if !(s_parse.p_new_index).is_null() {
                    rename_token_find(&mut s_parse, &mut s_ctx,
                        unsafe { (*s_parse.p_new_index).z_name } as *const ());
                    if is_legacy == 0 {
                        unsafe {
                            sqlite3_walk_expr(&mut s_walker,
                                unsafe { (*s_parse.p_new_index).p_part_idx_where })
                        };
                    }
                } else {
                    let p_trigger: *mut Trigger = s_parse.p_new_trigger;
                    let mut p_step: *const TriggerStep = core::ptr::null();
                    if 0 ==
                                unsafe {
                                    sqlite3_stricmp(unsafe { (*s_parse.p_new_trigger).table } as
                                            *const i8, z_old)
                                } &&
                            unsafe { (*s_ctx.p_tab).p_schema } ==
                                unsafe { (*p_trigger).p_tab_schema } {
                        rename_token_find(&mut s_parse, &mut s_ctx,
                            unsafe { (*s_parse.p_new_trigger).table } as *const ());
                    }
                    if is_legacy == 0 {
                        rc = rename_resolve_trigger(&mut s_parse);
                        if rc == 0 {
                            rename_walk_trigger(&mut s_walker, unsafe { &*p_trigger });
                            {
                                p_step = unsafe { (*p_trigger).step_list };
                                '__b20: loop {
                                    if !(!(p_step).is_null()) { break '__b20; }
                                    '__c20: loop {
                                        if !(unsafe { (*p_step).p_src }).is_null() {
                                            let mut i: i32 = 0;
                                            {
                                                i = 0;
                                                '__b21: loop {
                                                    if !(i < unsafe { (*unsafe { (*p_step).p_src }).n_src }) {
                                                        break '__b21;
                                                    }
                                                    '__c21: loop {
                                                        let p_item: *const SrcItem =
                                                            unsafe {
                                                                    &raw mut *(unsafe {
                                                                                        (*unsafe { (*p_step).p_src }).a.as_ptr()
                                                                                    } as *mut SrcItem).offset(i as isize)
                                                                } as *const SrcItem;
                                                        if 0 ==
                                                                unsafe {
                                                                    sqlite3_stricmp(unsafe { (*p_item).z_name } as *const i8,
                                                                        z_old)
                                                                } {
                                                            rename_token_find(&mut s_parse, &mut s_ctx,
                                                                unsafe { (*p_item).z_name } as *const ());
                                                        }
                                                        break '__c21;
                                                    }
                                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                                }
                                            }
                                        }
                                        break '__c20;
                                    }
                                    p_step = unsafe { (*p_step).p_next };
                                }
                            }
                        }
                    }
                }
            }
            if rc == 0 {
                rc =
                    rename_edit_sql(context, &mut s_ctx, z_input, z_new,
                        b_quote);
            }
            if rc != 0 {
                if rc == 1 && unsafe { sqlite3_writable_schema(db) } != 0 {
                    unsafe {
                        sqlite3_result_value(context,
                            unsafe { *argv.offset(3 as isize) })
                    };
                } else if !(s_parse.z_err_msg).is_null() {
                    rename_column_parse_error(context,
                        c"".as_ptr() as *mut i8 as *const i8,
                        unsafe { *argv.offset(1 as isize) },
                        unsafe { *argv.offset(2 as isize) }, &s_parse);
                } else { unsafe { sqlite3_result_error_code(context, rc) }; }
            }
            rename_parse_cleanup(&mut s_parse);
            rename_token_free(db, s_ctx.p_list);
            unsafe { sqlite3_btree_leave_all(db) };
            unsafe { (*db).x_auth = x_auth };
        }
        return;
    }
}

extern "C" fn rename_table_test(context: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
        let z_db: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        let z_input: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) }
                as *const i8;
        let b_temp: i32 =
            unsafe { sqlite3_value_int(unsafe { *argv.offset(4 as isize) }) };
        let is_legacy: i32 =
            (unsafe { (*db).flags } & 67108864 as u64) as i32;
        let z_when: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(5 as isize) }) }
                as *const i8;
        let b_no_dqs: i32 =
            unsafe { sqlite3_value_int(unsafe { *argv.offset(6 as isize) }) };
        let x_auth:
                unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                    *const i8, *const i8) -> i32 = unsafe { (*db).x_auth };
        unsafe {
            (*db).x_auth =
                unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                                *const i8, *const i8) -> i32>(0 as *const ())
                }
        };
        { let _ = not_used_1; };
        if !(z_db).is_null() && !(z_input).is_null() {
            let mut rc: i32 = 0;
            let mut s_parse: Parse = unsafe { core::mem::zeroed() };
            let flags: u64 = unsafe { (*db).flags };
            if b_no_dqs != 0 {
                unsafe { (*db).flags &= !(1073741824 | 536870912) as u64 };
            }
            rc = rename_parse_sql(&mut s_parse, z_db, db, z_input, b_temp);
            unsafe { (*db).flags = flags };
            if rc == 0 {
                if is_legacy == 0 && !(s_parse.p_new_table).is_null() &&
                        unsafe { (*s_parse.p_new_table).e_tab_type } as i32 == 2 {
                    let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
                    unsafe {
                        memset(&raw mut s_nc as *mut (), 0,
                            core::mem::size_of::<NameContext>() as u64)
                    };
                    s_nc.p_parse = &mut s_parse;
                    unsafe {
                        sqlite3_select_prep(&mut s_parse,
                            unsafe { (*s_parse.p_new_table).u.view.p_select },
                            &mut s_nc)
                    };
                    if s_parse.n_err != 0 { rc = s_parse.rc; }
                } else if !(s_parse.p_new_trigger).is_null() {
                    if is_legacy == 0 {
                        rc = rename_resolve_trigger(&mut s_parse);
                    }
                    if rc == 0 {
                        let i1: i32 =
                            unsafe {
                                sqlite3_schema_to_index(db,
                                    unsafe { (*s_parse.p_new_trigger).p_tab_schema })
                            };
                        let i2: i32 = unsafe { sqlite3_find_db_name(db, z_db) };
                        if i1 == i2 { unsafe { sqlite3_result_int(context, 1) }; }
                    }
                }
            }
            if rc != 0 && !(z_when).is_null() &&
                    (unsafe { sqlite3_writable_schema(db) } == 0) as i32 != 0 {
                rename_column_parse_error(context, z_when,
                    unsafe { *argv.offset(2 as isize) },
                    unsafe { *argv.offset(3 as isize) }, &s_parse);
            }
            rename_parse_cleanup(&mut s_parse);
        }
        unsafe { (*db).x_auth = x_auth };
    }
}

extern "C" fn get_constraint_token(z: *const u8, pi_token_1: &mut i32)
    -> i32 {
    let mut i_off: i32 = 0;
    let mut t: i32 = 0;
    '__b22: loop {
        '__c22: loop {
            i_off +=
                unsafe {
                        sqlite3_get_token(unsafe { &*z.offset(i_off as isize) },
                            &mut t)
                    } as i32;
            break '__c22;
        }
        if !(t == 184 || t == 185) { break '__b22; }
    }
    *pi_token_1 = t;
    if t == 22 {
        let mut n_nest: i32 = 1;
        while n_nest > 0 {
            i_off +=
                unsafe {
                        sqlite3_get_token(unsafe { &*z.offset(i_off as isize) },
                            &mut t)
                    } as i32;
            if t == 22 {
                { let __p = &mut n_nest; let __t = *__p; *__p += 1; __t };
            } else if t == 23 {
                t = 22;
                { let __p = &mut n_nest; let __t = *__p; *__p -= 1; __t };
            } else if t == 186 { break; }
        }
    }
    *pi_token_1 = t;
    return i_off;
}

extern "C" fn drop_column_func(context: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut i_schema: i32 = 0;
        let mut z_sql: *const i8 = core::ptr::null();
        let mut i_col: i32 = 0;
        let mut z_db: *const i8 = core::ptr::null();
        let mut rc: i32 = 0;
        let mut s_parse: Parse = unsafe { core::mem::zeroed() };
        let mut p_col: *mut RenameToken = core::ptr::null_mut();
        let mut p_tab: *const Table = core::ptr::null();
        let mut z_end: *const i8 = core::ptr::null();
        let mut z_new: *mut i8 = core::ptr::null_mut();
        let mut x_auth:
                unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                    *const i8, *const i8) -> i32 =
            unsafe {
                core::mem::transmute::<*const (),
                        unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                            *const i8, *const i8) -> i32>(0 as *const ())
            };
        let mut p_end: *const RenameToken = core::ptr::null();
        let mut e_tok: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s25:
                {
                match __state {
                    0 => {
                        db = unsafe { sqlite3_context_db_handle(context) };
                        __state = 3;
                    }
                    2 => { rename_parse_cleanup(&mut s_parse); __state = 40; }
                    3 => {
                        i_schema =
                            unsafe {
                                sqlite3_value_int(unsafe { *argv.offset(0 as isize) })
                            };
                        __state = 4;
                    }
                    4 => {
                        z_sql =
                            unsafe {
                                    sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
                                } as *const i8;
                        __state = 5;
                    }
                    5 => {
                        i_col =
                            unsafe {
                                sqlite3_value_int(unsafe { *argv.offset(2 as isize) })
                            };
                        __state = 6;
                    }
                    6 => {
                        z_db =
                            unsafe {
                                    (*unsafe {
                                                (*db).a_db.offset(i_schema as isize)
                                            }).z_db_s_name
                                } as *const i8;
                        __state = 7;
                    }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { __state = 10; }
                    10 => { __state = 11; }
                    11 => { __state = 12; }
                    12 => { z_new = core::ptr::null_mut(); __state = 13; }
                    13 => { x_auth = unsafe { (*db).x_auth }; __state = 14; }
                    14 => {
                        unsafe {
                            (*db).x_auth =
                                unsafe {
                                    core::mem::transmute::<*const (),
                                            unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                                                *const i8, *const i8) -> i32>(0 as *const ())
                                }
                        };
                        __state = 15;
                    }
                    15 => { { let _ = not_used_1; }; __state = 16; }
                    16 => {
                        rc =
                            rename_parse_sql(&mut s_parse, z_db, db, z_sql,
                                (i_schema == 1) as i32);
                        __state = 17;
                    }
                    17 => {
                        if rc != 0 { __state = 19; } else { __state = 18; }
                    }
                    18 => { p_tab = s_parse.p_new_table; __state = 20; }
                    19 => { __state = 2; }
                    20 => {
                        if p_tab == core::ptr::null_mut() ||
                                    unsafe { (*p_tab).n_col } as i32 == 1 ||
                                i_col >= unsafe { (*p_tab).n_col } as i32 {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => {
                        if i_col < unsafe { (*p_tab).n_col } as i32 - 1 {
                            __state = 25;
                        } else { __state = 26; }
                    }
                    22 => {
                        rc = unsafe { sqlite3_corrupt_error(2204) };
                        __state = 23;
                    }
                    23 => { __state = 2; }
                    24 => {
                        z_new =
                            unsafe {
                                sqlite3_m_printf(db,
                                    c"%.*s%s".as_ptr() as *mut i8 as *const i8,
                                    unsafe { unsafe { (*p_col).t.z.offset_from(z_sql) } } as
                                        i64, z_sql, z_end)
                            };
                        __state = 37;
                    }
                    25 => { __state = 27; }
                    26 => { __state = 30; }
                    27 => {
                        p_col =
                            rename_token_find(&mut s_parse, core::ptr::null_mut(),
                                unsafe {
                                            (*unsafe {
                                                        (*p_tab).a_col.offset(i_col as isize)
                                                    }).z_cn_name
                                        } as *mut () as *const ());
                        __state = 28;
                    }
                    28 => {
                        p_end =
                            rename_token_find(&mut s_parse, core::ptr::null_mut(),
                                unsafe {
                                            (*unsafe {
                                                        (*p_tab).a_col.offset((i_col + 1) as isize)
                                                    }).z_cn_name
                                        } as *mut () as *const ());
                        __state = 29;
                    }
                    29 => {
                        z_end = unsafe { (*p_end).t.z } as *const i8;
                        __state = 24;
                    }
                    30 => { { let _ = 0; }; __state = 31; }
                    31 => { { let _ = 0; }; __state = 32; }
                    32 => {
                        p_col =
                            rename_token_find(&mut s_parse, core::ptr::null_mut(),
                                unsafe {
                                            (*unsafe {
                                                        (*p_tab).a_col.offset((i_col - 1) as isize)
                                                    }).z_cn_name
                                        } as *mut () as *const ());
                        __state = 33;
                    }
                    33 => {
                        {
                            let __n =
                                get_constraint_token(unsafe { (*p_col).t.z } as *const u8,
                                    &mut e_tok);
                            let __p = unsafe { &mut (*p_col).t.z };
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        __state = 35;
                    }
                    34 => {
                        {
                            let __p = unsafe { &mut (*p_col).t.z };
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(-1) };
                            __t
                        };
                        __state = 36;
                    }
                    35 => {
                        if e_tok != 25 { __state = 33; } else { __state = 34; }
                    }
                    36 => {
                        z_end =
                            unsafe {
                                    &raw const *z_sql.offset(unsafe {
                                                        (*p_tab).u.tab.add_col_offset
                                                    } as isize)
                                } as *const i8;
                        __state = 24;
                    }
                    37 => {
                        unsafe {
                            sqlite3_result_text(context, z_new as *const i8, -1,
                                Some(unsafe {
                                        core::mem::transmute::<*const (),
                                                unsafe extern "C" fn(*mut ())
                                                    -> ()>(-1 as isize as *const ())
                                    }))
                        };
                        __state = 38;
                    }
                    38 => {
                        unsafe { sqlite3_free(z_new as *mut ()) };
                        __state = 39;
                    }
                    39 => { __state = 2; }
                    40 => { unsafe { (*db).x_auth = x_auth }; __state = 41; }
                    41 => { if rc != 0 { __state = 42; } else { __state = 1; } }
                    42 => {
                        unsafe { sqlite3_result_error_code(context, rc) };
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

extern "C" fn rename_quotefix_expr_cb(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 118 &&
                unsafe { (*p_expr_1).flags } & 128 as u32 != 0 {
            rename_token_find(unsafe { (*p_walker_1).p_parse },
                unsafe { (*p_walker_1).u.p_rename }, p_expr_1 as *const ());
        }
        return 0;
    }
}

extern "C" fn rename_quotefix_func(context: *mut Sqlite3Context,
    not_used_1: i32, argv: *mut *mut Sqlite3Value) -> () {
    unsafe {
        let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(context) };
        let z_db: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) }
                as *const i8;
        let z_input: *const i8 =
            unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) }
                as *const i8;
        let x_auth:
                unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                    *const i8, *const i8) -> i32 = unsafe { (*db).x_auth };
        unsafe {
            (*db).x_auth =
                unsafe {
                    core::mem::transmute::<*const (),
                            unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
                                *const i8, *const i8) -> i32>(0 as *const ())
                }
        };
        unsafe { sqlite3_btree_enter_all(db) };
        { let _ = not_used_1; };
        if !(z_db).is_null() && !(z_input).is_null() {
            let mut rc: i32 = 0;
            let mut s_parse: Parse = unsafe { core::mem::zeroed() };
            rc = rename_parse_sql(&mut s_parse, z_db, db, z_input, 0);
            if rc == 0 {
                let mut s_ctx: RenameCtx = unsafe { core::mem::zeroed() };
                let mut s_walker: Walker = unsafe { core::mem::zeroed() };
                unsafe {
                    memset(&raw mut s_ctx as *mut (), 0,
                        core::mem::size_of::<RenameCtx>() as u64)
                };
                unsafe {
                    memset(&raw mut s_walker as *mut (), 0,
                        core::mem::size_of::<Walker>() as u64)
                };
                s_walker.p_parse = &mut s_parse;
                s_walker.x_expr_callback = Some(rename_quotefix_expr_cb);
                s_walker.x_select_callback = Some(rename_column_select_cb);
                s_walker.u.p_rename = &mut s_ctx;
                if !(s_parse.p_new_table).is_null() {
                    if unsafe { (*s_parse.p_new_table).e_tab_type } as i32 == 2
                        {
                        let p_select: *mut Select =
                            unsafe { (*s_parse.p_new_table).u.view.p_select };
                        unsafe { (*p_select).sel_flags &= !(2097152 as u32) };
                        s_parse.rc = 0;
                        unsafe {
                            sqlite3_select_prep(&mut s_parse, p_select,
                                core::ptr::null_mut())
                        };
                        rc =
                            if unsafe { (*db).malloc_failed } != 0 {
                                7
                            } else { s_parse.rc };
                        if rc == 0 {
                            unsafe { sqlite3_walk_select(&mut s_walker, p_select) };
                        }
                    } else {
                        let mut i: i32 = 0;
                        unsafe {
                            sqlite3_walk_expr_list(&mut s_walker,
                                unsafe { (*s_parse.p_new_table).p_check })
                        };
                        {
                            i = 0;
                            '__b26: loop {
                                if !(i < unsafe { (*s_parse.p_new_table).n_col } as i32) {
                                    break '__b26;
                                }
                                '__c26: loop {
                                    unsafe {
                                        sqlite3_walk_expr(&mut s_walker,
                                            unsafe {
                                                sqlite3_column_expr(s_parse.p_new_table,
                                                    unsafe {
                                                        &mut *unsafe {
                                                                    (*s_parse.p_new_table).a_col.offset(i as isize)
                                                                }
                                                    })
                                            })
                                    };
                                    break '__c26;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                    }
                } else if !(s_parse.p_new_index).is_null() {
                    unsafe {
                        sqlite3_walk_expr_list(&mut s_walker,
                            unsafe { (*s_parse.p_new_index).a_col_expr })
                    };
                    unsafe {
                        sqlite3_walk_expr(&mut s_walker,
                            unsafe { (*s_parse.p_new_index).p_part_idx_where })
                    };
                } else {
                    rc = rename_resolve_trigger(&mut s_parse);
                    if rc == 0 {
                        rename_walk_trigger(&mut s_walker,
                            unsafe { &*s_parse.p_new_trigger });
                    }
                }
                if rc == 0 {
                    rc =
                        rename_edit_sql(context, &mut s_ctx, z_input,
                            core::ptr::null(), 0);
                }
                rename_token_free(db, s_ctx.p_list);
            }
            if rc != 0 {
                if unsafe { sqlite3_writable_schema(db) } != 0 && rc == 1 {
                    unsafe {
                        sqlite3_result_value(context,
                            unsafe { *argv.offset(1 as isize) })
                    };
                } else { unsafe { sqlite3_result_error_code(context, rc) }; }
            }
            rename_parse_cleanup(&mut s_parse);
        }
        unsafe { (*db).x_auth = x_auth };
        unsafe { sqlite3_btree_leave_all(db) };
    }
}

extern "C" fn skip_create_table(ctx: *mut Sqlite3Context, z_sql_1: *const u8,
    pi_off_1: &mut i32) -> i32 {
    let mut i_off: i32 = 0;
    if z_sql_1 == core::ptr::null() { return 1; }
    loop {
        let mut t: i32 = 0;
        i_off +=
            unsafe {
                    sqlite3_get_token(unsafe {
                            &*z_sql_1.offset(i_off as isize)
                        }, &mut t)
                } as i32;
        if t == 22 { break; }
        if t == 186 {
            unsafe {
                sqlite3_result_error_code(ctx,
                    unsafe { sqlite3_corrupt_error(2499) })
            };
            return 1;
        }
    }
    *pi_off_1 = i_off;
    return 0;
}

extern "C" fn get_whitespace(z: *const u8) -> i32 {
    let mut n_ret: i32 = 0;
    loop {
        let mut t: i32 = 0;
        let n: i32 =
            unsafe {
                    sqlite3_get_token(unsafe { &*z.offset(n_ret as isize) },
                        &mut t)
                } as i32;
        if t != 184 && t != 185 { break; }
        n_ret += n;
    }
    return n_ret;
}

extern "C" fn quoted_compare(ctx: *mut Sqlite3Context, t: i32,
    z_quote_1: *const u8, n_quote_1: i32, z_cmp_1: *const u8,
    p_res_1: &mut i32) -> i32 {
    let mut z_copy: *mut i8 = core::ptr::null_mut();
    if t == 186 { *p_res_1 = 1; return 0; }
    z_copy =
        unsafe { sqlite3_malloc_zero((n_quote_1 + 1) as u64) } as *mut i8;
    if z_copy == core::ptr::null_mut() {
        unsafe { sqlite3_result_error_nomem(ctx) };
        return 7;
    }
    unsafe {
        memcpy(z_copy as *mut (), z_quote_1 as *const (), n_quote_1 as u64)
    };
    unsafe { sqlite3_dequote(z_copy) };
    *p_res_1 =
        unsafe { sqlite3_stricmp(z_copy as *const i8, z_cmp_1 as *const i8) };
    unsafe { sqlite3_free(z_copy as *mut ()) };
    return 0;
}

extern "C" fn get_constraint(z: *const u8) -> i32 {
    let mut i_off: i32 = 0;
    let mut t: i32 = 0;
    loop {
        let n: i32 =
            get_constraint_token(unsafe { &*z.offset(i_off as isize) },
                &mut t);
        if t == 120 || t == 123 || t == 19 || t == 124 || t == 125 || t == 121
                                            || t == 114 || t == 126 || t == 133 || t == 23 || t == 25 ||
                        t == 186 || t == 24 || t == 96 {
            break;
        }
        i_off += n;
    }
    return i_off;
}

unsafe extern "C" fn error_m_printf(p_ctx_1: *mut Sqlite3Context,
    z_fmt_1: *const i8, mut __va0: ...) -> () {
    let db: *mut Sqlite3 = unsafe { sqlite3_context_db_handle(p_ctx_1) };
    let mut z_err: *mut i8 = core::ptr::null_mut();
    let mut ap: *mut i8 = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    z_err = unsafe { sqlite3_vm_printf(db, z_fmt_1, ap) };
    ();
    if !(z_err).is_null() {
        unsafe { sqlite3_result_error(p_ctx_1, z_err as *const i8, -1) };
        unsafe { sqlite3_db_free(db, z_err as *mut ()) };
    } else { unsafe { sqlite3_result_error_nomem(p_ctx_1) }; }
}

extern "C" fn drop_constraint_func(ctx: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let z_sql: *const u8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    let mut z_cons: *const u8 = core::ptr::null();
    let mut i_not_null: i32 = -1;
    let mut ii: i32 = 0;
    let mut i_off: i32 = 0;
    let mut i_start: i32 = 0;
    let mut i_end: i32 = 0;
    let mut z_new: *const i8 = core::ptr::null();
    let mut t: i32 = 0;
    let mut db: *mut Sqlite3 = core::ptr::null_mut();
    { let _ = not_used_1; };
    if z_sql == core::ptr::null() { return; }
    if skip_create_table(ctx, z_sql, &mut i_off) != 0 { return; }
    if unsafe { sqlite3_value_type(unsafe { *argv.offset(1 as isize) }) } == 1
        {
        i_not_null =
            unsafe { sqlite3_value_int(unsafe { *argv.offset(1 as isize) }) };
    } else {
        z_cons =
            unsafe {
                sqlite3_value_text(unsafe { *argv.offset(1 as isize) })
            };
    }
    {
        ii = 0;
        '__b30: loop {
            if !(i_end == 0) { break '__b30; }
            '__c30: loop {
                loop {
                    i_start = i_off;
                    i_off +=
                        get_constraint_token(unsafe {
                                &*z_sql.offset(i_off as isize)
                            }, &mut t);
                    if t == 120 && (!(z_cons).is_null() || i_not_null == ii) {
                        let mut n_tok: i32 = 0;
                        let mut cmp: i32 = 1;
                        i_off +=
                            get_whitespace(unsafe { &*z_sql.offset(i_off as isize) });
                        n_tok =
                            get_constraint_token(unsafe {
                                    &*z_sql.offset(i_off as isize)
                                }, &mut t);
                        if !(z_cons).is_null() {
                            if quoted_compare(ctx, t,
                                        unsafe { &*z_sql.offset(i_off as isize) }, n_tok, z_cons,
                                        &mut cmp) != 0 {
                                return;
                            }
                        }
                        i_off += n_tok;
                        n_tok =
                            get_constraint_token(unsafe {
                                    &*z_sql.offset(i_off as isize)
                                }, &mut t);
                        if t == 120 || t == 121 || t == 114 || t == 25 || t == 23 ||
                                    t == 96 || t == 24 {
                            t = 125;
                        } else {
                            i_off += n_tok;
                            i_off +=
                                get_constraint(unsafe { &*z_sql.offset(i_off as isize) });
                        }
                        if cmp == 0 || i_not_null >= 0 && t == 19 {
                            if t != 19 && t != 125 {
                                unsafe {
                                    error_m_printf(ctx,
                                        c"constraint may not be dropped: %s".as_ptr() as *mut i8 as
                                            *const i8, z_cons)
                                };
                                return;
                            }
                            i_end = i_off;
                            break;
                        }
                    } else if t == 19 && i_not_null == ii {
                        i_end =
                            i_off +
                                get_constraint(unsafe { &*z_sql.offset(i_off as isize) });
                        break;
                    } else if t == 23 || t == 186 {
                        i_end = -1;
                        break;
                    } else if t == 25 { break; }
                }
                break '__c30;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    if i_end <= 0 {
        if !(z_cons).is_null() {
            unsafe {
                error_m_printf(ctx,
                    c"no such constraint: %s".as_ptr() as *mut i8 as *const i8,
                    z_cons)
            };
        } else {
            unsafe {
                sqlite3_result_text(ctx, z_sql as *const i8, -1,
                    Some(unsafe {
                            core::mem::transmute::<*const (),
                                    unsafe extern "C" fn(*mut ())
                                        -> ()>(-1 as isize as *const ())
                        }))
            };
        }
    } else {
        let mut z_space: *const i8 = c" ".as_ptr() as *mut i8 as *const i8;
        i_end += get_whitespace(unsafe { &*z_sql.offset(i_end as isize) });
        unsafe {
            sqlite3_get_token(unsafe { &*z_sql.offset(i_end as isize) },
                &mut t)
        };
        if t == 23 || t == 25 {
            z_space = c"".as_ptr() as *mut i8 as *const i8;
            if unsafe { *z_sql.offset((i_start - 1) as isize) } as i32 ==
                    ',' as i32 {
                { let __p = &mut i_start; let __t = *__p; *__p -= 1; __t };
            }
        }
        db = unsafe { sqlite3_context_db_handle(ctx) };
        z_new =
            unsafe {
                sqlite3_m_printf(db,
                    c"%.*s%s%s".as_ptr() as *mut i8 as *const i8, i_start,
                    z_sql, z_space,
                    unsafe { &raw const *z_sql.offset(i_end as isize) } as
                        *const u8)
            };
        unsafe {
            sqlite3_result_text(ctx, z_new as *const i8, -1,
                Some(unsafe {
                        core::mem::transmute::<*const (),
                                unsafe extern "C" fn(*mut ())
                                    -> ()>(sqlite3_row_set_clear as *const ())
                    }))
        };
    }
}

extern "C" fn fail_constraint_func(ctx: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let z_text: *const i8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) } as
            *const i8;
    let err: i32 =
        unsafe { sqlite3_value_int(unsafe { *argv.offset(1 as isize) }) };
    { let _ = not_used_1; };
    unsafe { sqlite3_result_error(ctx, z_text, -1) };
    unsafe { sqlite3_result_error_code(ctx, err) };
}

extern "C" fn add_constraint_func(ctx: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let z_sql: *const u8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    let z_cons: *const i8 =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) } as
            *const i8;
    let i_col: i32 =
        unsafe { sqlite3_value_int(unsafe { *argv.offset(2 as isize) }) };
    let mut i_off: i32 = 0;
    let mut ii: i32 = 0;
    let mut p_new: *mut Sqlite3Str = core::ptr::null_mut();
    let mut t: i32 = 0;
    { let _ = not_used_1; };
    if skip_create_table(ctx, z_sql, &mut i_off) != 0 { return; }
    {
        ii = 0;
        '__b32: loop {
            if !(ii <= i_col || i_col < 0 && t != 23) { break '__b32; }
            '__c32: loop {
                i_off +=
                    get_constraint_token(unsafe {
                            &*z_sql.offset(i_off as isize)
                        }, &mut t);
                loop {
                    let n_tok: i32 =
                        get_constraint_token(unsafe {
                                &*z_sql.offset(i_off as isize)
                            }, &mut t);
                    if t == 25 || t == 23 { break; }
                    if t == 186 {
                        unsafe {
                            sqlite3_result_error_code(ctx,
                                unsafe { sqlite3_corrupt_error(2676) })
                        };
                        return;
                    }
                    i_off += n_tok;
                }
                break '__c32;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    i_off += get_whitespace(unsafe { &*z_sql.offset(i_off as isize) });
    p_new =
        unsafe { sqlite3_str_new(unsafe { sqlite3_context_db_handle(ctx) }) };
    unsafe { sqlite3_str_append(p_new, z_sql as *const i8, i_off) };
    if i_col < 0 {
        unsafe {
            sqlite3_str_append(p_new, c",".as_ptr() as *mut i8 as *const i8,
                1)
        };
    }
    unsafe {
        sqlite3_str_appendf(p_new, c" %s%s".as_ptr() as *mut i8 as *const i8,
            z_cons,
            unsafe { &raw const *z_sql.offset(i_off as isize) } as *const u8)
    };
    unsafe { sqlite3_result_str(ctx, p_new, 2) };
}

extern "C" fn find_constraint_func(ctx: *mut Sqlite3Context, not_used_1: i32,
    argv: *mut *mut Sqlite3Value) -> () {
    let mut z_sql: *const u8 = core::ptr::null();
    let mut z_cons: *const u8 = core::ptr::null();
    let mut i_off: i32 = 0;
    let mut t: i32 = 0;
    { let _ = not_used_1; };
    z_sql =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(0 as isize) }) };
    z_cons =
        unsafe { sqlite3_value_text(unsafe { *argv.offset(1 as isize) }) };
    if z_sql == core::ptr::null() || z_cons == core::ptr::null() { return; }
    while t != 22 && t != 186 {
        i_off +=
            unsafe {
                    sqlite3_get_token(unsafe { &*z_sql.offset(i_off as isize) },
                        &mut t)
                } as i32;
    }
    loop {
        i_off +=
            get_constraint_token(unsafe { &*z_sql.offset(i_off as isize) },
                &mut t);
        if t == 120 {
            let mut n_tok: i32 = 0;
            let mut cmp: i32 = 0;
            i_off +=
                get_whitespace(unsafe { &*z_sql.offset(i_off as isize) });
            n_tok =
                get_constraint_token(unsafe {
                        &*z_sql.offset(i_off as isize)
                    }, &mut t);
            if quoted_compare(ctx, t,
                        unsafe { &*z_sql.offset(i_off as isize) }, n_tok, z_cons,
                        &mut cmp) != 0 {
                return;
            }
            if cmp == 0 { unsafe { sqlite3_result_int(ctx, 1) }; return; }
        } else if t == 186 { break; }
    }
    unsafe { sqlite3_result_int(ctx, 0) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_functions() -> () {
    unsafe {
        unsafe {
            sqlite3_insert_builtin_funcs(&raw mut a_alter_table_funcs[0 as
                                usize] as *mut FuncDef,
                (core::mem::size_of::<[FuncDef; 9]>() as u64 /
                        core::mem::size_of::<FuncDef>() as u64) as i32)
        };
    }
}

extern "C" fn is_alterable_table(p_parse_1: *mut Parse, p_tab_1: &Table)
    -> i32 {
    if 0 ==
                    unsafe {
                        sqlite3_strnicmp((*p_tab_1).z_name as *const i8,
                            c"sqlite_".as_ptr() as *mut i8 as *const i8, 7)
                    } || (*p_tab_1).tab_flags & 32768 as u32 != 0 as u32 ||
            (*p_tab_1).tab_flags & 4096 as u32 != 0 as u32 &&
                unsafe {
                        sqlite3_read_only_shadow_tables(unsafe { (*p_parse_1).db })
                    } != 0 {
        unsafe {
            sqlite3_error_msg(p_parse_1,
                c"table %s may not be altered".as_ptr() as *mut i8 as
                    *const i8, (*p_tab_1).z_name)
        };
        return 1;
    }
    return 0;
}

extern "C" fn rename_reload_schema(p_parse_1: *mut Parse, i_db_1: i32,
    p5: u16) -> () {
    let v: *const Vdbe = unsafe { (*p_parse_1).p_vdbe } as *const Vdbe;
    if !(v).is_null() {
        unsafe { sqlite3_change_cookie(p_parse_1, i_db_1) };
        unsafe {
            sqlite3_vdbe_add_parse_schema_op(unsafe { (*p_parse_1).p_vdbe },
                i_db_1, core::ptr::null_mut(), p5)
        };
        if i_db_1 != 1 {
            unsafe {
                sqlite3_vdbe_add_parse_schema_op(unsafe {
                        (*p_parse_1).p_vdbe
                    }, 1, core::ptr::null_mut(), p5)
            };
        }
    }
}

extern "C" fn rename_test_schema(p_parse_1: *mut Parse, z_db_1: *const i8,
    b_temp_1: i32, z_when_1: *const i8, b_no_dqs_1: i32) -> () {
    unsafe { (*p_parse_1).set_col_names_set(1 as Bft as u32) };
    unsafe {
        sqlite3_nested_parse(p_parse_1,
            c"SELECT 1 FROM \"%w\".sqlite_master WHERE name NOT LIKE \'sqliteX_%%\' ESCAPE \'X\' AND sql NOT LIKE \'create virtual%%\' AND sqlite_rename_test(%Q, sql, type, name, %d, %Q, %d)=NULL ".as_ptr()
                    as *mut i8 as *const i8, z_db_1, z_db_1, b_temp_1, z_when_1,
            b_no_dqs_1)
    };
    if b_temp_1 == 0 {
        unsafe {
            sqlite3_nested_parse(p_parse_1,
                c"SELECT 1 FROM temp.sqlite_master WHERE name NOT LIKE \'sqliteX_%%\' ESCAPE \'X\' AND sql NOT LIKE \'create virtual%%\' AND sqlite_rename_test(%Q, sql, type, name, 1, %Q, %d)=NULL ".as_ptr()
                        as *mut i8 as *const i8, z_db_1, z_when_1, b_no_dqs_1)
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_rename_table(p_parse: *mut Parse,
    p_src: *mut SrcList, p_name: *mut Token) -> () {
    unsafe {
        let mut z_name: *mut i8 = core::ptr::null_mut();
        let db: *mut Sqlite3 = unsafe { (*p_parse).db };
        '__b36: loop {
            '__c36: loop {
                let mut i_db: i32 = 0;
                let mut z_db: *mut i8 = core::ptr::null_mut();
                let mut p_tab: *mut Table = core::ptr::null_mut();
                let mut n_tab_name: i32 = 0;
                let mut z_tab_name: *const i8 = core::ptr::null();
                let mut v: *mut Vdbe = core::ptr::null_mut();
                let mut p_v_tab: *const VTable = core::ptr::null();
                if unsafe { (*db).malloc_failed } != 0 { break '__b36; }
                { let _ = 0; };
                { let _ = 0; };
                p_tab =
                    unsafe {
                        sqlite3_locate_table_item(p_parse, 0 as u32,
                            unsafe {
                                &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)
                            })
                    };
                if (p_tab).is_null() as i32 != 0 { break '__b36; }
                i_db =
                    unsafe {
                        sqlite3_schema_to_index(unsafe { (*p_parse).db },
                            unsafe { (*p_tab).p_schema })
                    };
                z_db =
                    unsafe {
                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                    };
                z_name =
                    unsafe {
                        sqlite3_name_from_token(db, p_name as *const Token)
                    };
                if (z_name).is_null() as i32 != 0 { break '__b36; }
                if !(unsafe {
                                            sqlite3_find_table(db, z_name as *const i8,
                                                z_db as *const i8)
                                        }).is_null() ||
                            !(unsafe {
                                            sqlite3_find_index(db, z_name as *const i8,
                                                z_db as *const i8)
                                        }).is_null() ||
                        unsafe {
                                sqlite3_is_shadow_table_of(db, p_tab, z_name as *const i8)
                            } != 0 {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"there is already another table or index with this name: %s".as_ptr()
                                    as *mut i8 as *const i8, z_name)
                    };
                    break '__b36;
                }
                if 0 != is_alterable_table(p_parse, unsafe { &*p_tab }) {
                    break '__b36;
                }
                if 0 !=
                        unsafe {
                            sqlite3_check_object_name(p_parse, z_name as *const i8,
                                c"table".as_ptr() as *mut i8 as *const i8,
                                z_name as *const i8)
                        } {
                    break '__b36;
                }
                if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                    unsafe {
                        sqlite3_error_msg(p_parse,
                            c"view %s may not be altered".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p_tab).z_name })
                    };
                    break '__b36;
                }
                if unsafe {
                            sqlite3_auth_check(p_parse, 26, z_db as *const i8,
                                unsafe { (*p_tab).z_name } as *const i8, core::ptr::null())
                        } != 0 {
                    break '__b36;
                }
                if unsafe { sqlite3_view_get_column_names(p_parse, p_tab) } !=
                        0 {
                    break '__b36;
                }
                if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                    p_v_tab = unsafe { sqlite3_get_v_table(db, p_tab) };
                    if !unsafe {
                                        (*unsafe {
                                                            (*unsafe { (*p_v_tab).p_vtab }).p_module
                                                        }).x_rename.is_some()
                                    } as i32 != 0 {
                        p_v_tab = core::ptr::null_mut();
                    }
                }
                v = unsafe { sqlite3_get_vdbe(p_parse) };
                if v == core::ptr::null_mut() { break '__b36; }
                unsafe { sqlite3_may_abort(p_parse) };
                z_tab_name = unsafe { (*p_tab).z_name } as *const i8;
                n_tab_name = unsafe { sqlite3_utf8_char_len(z_tab_name, -1) };
                unsafe {
                    sqlite3_nested_parse(p_parse,
                        c"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_table(%Q, type, name, sql, %Q, %Q, %d) WHERE (type!=\'index\' OR tbl_name=%Q COLLATE nocase)AND   name NOT LIKE \'sqliteX_%%\' ESCAPE \'X\'".as_ptr()
                                as *mut i8 as *const i8, z_db, z_db, z_tab_name, z_name,
                        (i_db == 1) as i32, z_tab_name)
                };
                unsafe {
                    sqlite3_nested_parse(p_parse,
                        c"UPDATE %Q.sqlite_master SET tbl_name = %Q, name = CASE WHEN type=\'table\' THEN %Q WHEN name LIKE \'sqliteX_autoindex%%\' ESCAPE \'X\'      AND type=\'index\' THEN \'sqlite_autoindex_\' || %Q || substr(name,%d+18) ELSE name END WHERE tbl_name=%Q COLLATE nocase AND (type=\'table\' OR type=\'index\' OR type=\'trigger\');".as_ptr()
                                as *mut i8 as *const i8, z_db, z_name, z_name, z_name,
                        n_tab_name, z_tab_name)
                };
                if !(unsafe {
                                    sqlite3_find_table(db,
                                        c"sqlite_sequence".as_ptr() as *mut i8 as *const i8,
                                        z_db as *const i8)
                                }).is_null() {
                    unsafe {
                        sqlite3_nested_parse(p_parse,
                            c"UPDATE \"%w\".sqlite_sequence set name = %Q WHERE name = %Q".as_ptr()
                                    as *mut i8 as *const i8, z_db, z_name,
                            unsafe { (*p_tab).z_name })
                    };
                }
                if i_db != 1 {
                    unsafe {
                        sqlite3_nested_parse(p_parse,
                            c"UPDATE sqlite_temp_schema SET sql = sqlite_rename_table(%Q, type, name, sql, %Q, %Q, 1), tbl_name = CASE WHEN tbl_name=%Q COLLATE nocase AND   sqlite_rename_test(%Q, sql, type, name, 1, \'after rename\', 0) THEN %Q ELSE tbl_name END WHERE type IN (\'view\', \'trigger\')".as_ptr()
                                    as *mut i8 as *const i8, z_db, z_tab_name, z_name,
                            z_tab_name, z_db, z_name)
                    };
                }
                if !(p_v_tab).is_null() {
                    let i: i32 =
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            *__p += 1;
                            *__p
                        };
                    unsafe {
                        sqlite3_vdbe_load_string(v, i, z_name as *const i8)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op4(v, 179, i, 0, 0, p_v_tab as *const i8,
                            -12)
                    };
                }
                rename_reload_schema(p_parse, i_db, 1 as u16);
                rename_test_schema(p_parse, z_db as *const i8,
                    (i_db == 1) as i32,
                    c"after rename".as_ptr() as *mut i8 as *const i8, 0);
                break '__c36;
            }
            if !(false) { break '__b36; }
        }
        unsafe { sqlite3_src_list_delete(db, p_src) };
        unsafe { sqlite3_db_free(db, z_name as *mut ()) };
    }
}

extern "C" fn is_real_table(p_parse_1: *mut Parse, p_tab_1: &Table,
    i_op_1: i32) -> i32 {
    let mut z_type: *const i8 = core::ptr::null();
    if (*p_tab_1).e_tab_type as i32 == 2 {
        z_type = c"view".as_ptr() as *mut i8 as *const i8;
    }
    if (*p_tab_1).e_tab_type as i32 == 1 {
        z_type = c"virtual table".as_ptr() as *mut i8 as *const i8;
    }
    if !(z_type).is_null() {
        let az_msg: [*const i8; 3] =
            [c"rename columns of".as_ptr() as *const i8,
                    c"drop column from".as_ptr() as *const i8,
                    c"edit constraints of".as_ptr() as *const i8];
        { let _ = 0; };
        unsafe {
            sqlite3_error_msg(p_parse_1,
                c"cannot %s %s \"%s\"".as_ptr() as *mut i8 as *const i8,
                az_msg[i_op_1 as usize], z_type, (*p_tab_1).z_name)
        };
        return 1;
    }
    return 0;
}

extern "C" fn rename_fix_quotes(p_parse_1: *mut Parse, z_db_1: *const i8,
    b_temp_1: i32) -> () {
    unsafe {
        sqlite3_nested_parse(p_parse_1,
            c"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_quotefix(%Q, sql)WHERE name NOT LIKE \'sqliteX_%%\' ESCAPE \'X\' AND sql NOT LIKE \'create virtual%%\'".as_ptr()
                    as *mut i8 as *const i8, z_db_1, z_db_1)
    };
    if b_temp_1 == 0 {
        unsafe {
            sqlite3_nested_parse(p_parse_1,
                c"UPDATE temp.sqlite_master SET sql = sqlite_rename_quotefix(\'temp\', sql)WHERE name NOT LIKE \'sqliteX_%%\' ESCAPE \'X\' AND sql NOT LIKE \'create virtual%%\'".as_ptr()
                        as *mut i8 as *const i8)
        };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_rename_column(p_parse: *mut Parse,
    p_src: *mut SrcList, p_old: *mut Token, p_new: *mut Token) -> () {
    unsafe {
        unsafe {
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            let mut z_old: *mut i8 = core::ptr::null_mut();
            let mut z_new: *mut i8 = core::ptr::null_mut();
            '__b37: loop {
                '__c37: loop {
                    let mut p_tab: *mut Table = core::ptr::null_mut();
                    let mut i_col: i32 = 0;
                    let mut z_db: *const i8 = core::ptr::null();
                    let mut i_schema: i32 = 0;
                    let mut b_quote: i32 = 0;
                    p_tab =
                        unsafe {
                            sqlite3_locate_table_item(p_parse, 0 as u32,
                                unsafe {
                                    &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                    *mut SrcItem).offset(0 as isize)
                                })
                        };
                    if (p_tab).is_null() as i32 != 0 { break '__b37; }
                    if 0 != is_alterable_table(p_parse, unsafe { &*p_tab }) {
                        break '__b37;
                    }
                    if 0 != is_real_table(p_parse, unsafe { &*p_tab }, 0) {
                        break '__b37;
                    }
                    i_schema =
                        unsafe {
                            sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                        };
                    { let _ = 0; };
                    z_db =
                        unsafe {
                                (*unsafe {
                                            (*db).a_db.offset(i_schema as isize)
                                        }).z_db_s_name
                            } as *const i8;
                    if unsafe {
                                sqlite3_auth_check(p_parse, 26, z_db,
                                    unsafe { (*p_tab).z_name } as *const i8, core::ptr::null())
                            } != 0 {
                        break '__b37;
                    }
                    z_old =
                        unsafe {
                            sqlite3_name_from_token(db, p_old as *const Token)
                        };
                    if (z_old).is_null() as i32 != 0 { break '__b37; }
                    i_col =
                        unsafe { sqlite3_column_index(p_tab, z_old as *const i8) };
                    if i_col < 0 {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"no such column: \"%T\"".as_ptr() as *mut i8 as *const i8,
                                p_old)
                        };
                        break '__b37;
                    }
                    rename_test_schema(p_parse, z_db, (i_schema == 1) as i32,
                        c"".as_ptr() as *mut i8 as *const i8, 0);
                    rename_fix_quotes(p_parse, z_db, (i_schema == 1) as i32);
                    unsafe { sqlite3_may_abort(p_parse) };
                    z_new =
                        unsafe {
                            sqlite3_name_from_token(db, p_new as *const Token)
                        };
                    if (z_new).is_null() as i32 != 0 { break '__b37; }
                    { let _ = 0; };
                    b_quote =
                        unsafe {
                                    *(sqlite3_ctype_map.as_ptr() as
                                                *const u8).add(unsafe {
                                                        *unsafe { (*p_new).z.offset(0 as isize) }
                                                    } as u8 as usize)
                                } as i32 & 128;
                    unsafe {
                        sqlite3_nested_parse(p_parse,
                            c"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_column(sql, type, name, %Q, %Q, %d, %Q, %d, %d) WHERE name NOT LIKE \'sqliteX_%%\' ESCAPE \'X\'  AND (type != \'index\' OR tbl_name = %Q)".as_ptr()
                                    as *mut i8 as *const i8, z_db, z_db,
                            unsafe { (*p_tab).z_name }, i_col, z_new, b_quote,
                            (i_schema == 1) as i32, unsafe { (*p_tab).z_name })
                    };
                    unsafe {
                        sqlite3_nested_parse(p_parse,
                            c"UPDATE temp.sqlite_master SET sql = sqlite_rename_column(sql, type, name, %Q, %Q, %d, %Q, %d, 1) WHERE type IN (\'trigger\', \'view\')".as_ptr()
                                    as *mut i8 as *const i8, z_db, unsafe { (*p_tab).z_name },
                            i_col, z_new, b_quote)
                    };
                    rename_reload_schema(p_parse, i_schema, 1 as u16);
                    rename_test_schema(p_parse, z_db, (i_schema == 1) as i32,
                        c"after rename".as_ptr() as *mut i8 as *const i8, 1);
                    break '__c37;
                }
                if !(false) { break '__b37; }
            }
            unsafe { sqlite3_src_list_delete(db, p_src) };
            unsafe { sqlite3_db_free(db, z_old as *mut ()) };
            unsafe { sqlite3_db_free(db, z_new as *mut ()) };
            return;
        }
    }
}

extern "C" fn alter_find_table(p_parse_1: *mut Parse, p_src_1: *mut SrcList,
    pi_db_1: &mut i32, pz_db_1: &mut *const i8, b_auth_1: i32) -> *mut Table {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let mut p_tab: *mut Table = core::ptr::null_mut();
        { let _ = 0; };
        p_tab =
            unsafe {
                sqlite3_locate_table_item(p_parse_1, 0 as u32,
                    unsafe {
                        &mut *(unsafe { (*p_src_1).a.as_ptr() } as
                                        *mut SrcItem).offset(0 as isize)
                    })
            };
        if !(p_tab).is_null() {
            let i_db: i32 =
                unsafe {
                    sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                };
            *pz_db_1 =
                unsafe {
                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                    } as *const i8;
            *pi_db_1 = i_db;
            if 0 != is_real_table(p_parse_1, unsafe { &*p_tab }, 2) ||
                    0 != is_alterable_table(p_parse_1, unsafe { &*p_tab }) {
                p_tab = core::ptr::null_mut();
            }
        }
        if !(p_tab).is_null() && b_auth_1 != 0 {
            if unsafe {
                        sqlite3_auth_check(p_parse_1, 26, *pz_db_1,
                            unsafe { (*p_tab).z_name } as *const i8, core::ptr::null())
                    } != 0 {
                p_tab = core::ptr::null_mut();
            }
        }
        unsafe { sqlite3_src_list_delete(db, p_src_1) };
        return p_tab;
    }
}

extern "C" fn alter_find_col(p_parse_1: *mut Parse, mut p_tab_1: *mut Table,
    p_col_1: *const Token, pi_col_1: &mut i32) -> i32 {
    unsafe {
        let db: *mut Sqlite3 = unsafe { (*p_parse_1).db };
        let z_name: *mut i8 =
            unsafe { sqlite3_name_from_token(db, p_col_1 as *const Token) };
        let mut rc: i32 = 7;
        let mut i_col: i32 = -1;
        if !(z_name).is_null() {
            i_col =
                unsafe { sqlite3_column_index(p_tab_1, z_name as *const i8) };
            if i_col < 0 {
                unsafe {
                    sqlite3_error_msg(p_parse_1,
                        c"no such column: %s".as_ptr() as *mut i8 as *const i8,
                        z_name)
                };
                rc = 1;
            } else { rc = 0; }
        }
        if rc == 0 {
            let z_db: *const i8 =
                unsafe {
                        (*unsafe {
                                    (*db).a_db.offset(unsafe {
                                                sqlite3_schema_to_index(db, unsafe { (*p_tab_1).p_schema })
                                            } as isize)
                                }).z_db_s_name
                    } as *const i8;
            let z_col: *const i8 =
                unsafe {
                        (*unsafe {
                                    (*p_tab_1).a_col.offset(i_col as isize)
                                }).z_cn_name
                    } as *const i8;
            if unsafe {
                        sqlite3_auth_check(p_parse_1, 26, z_db,
                            unsafe { (*p_tab_1).z_name } as *const i8, z_col)
                    } != 0 {
                p_tab_1 = core::ptr::null_mut();
            }
        }
        unsafe { sqlite3_db_free(db, z_name as *mut ()) };
        *pi_col_1 = i_col;
        return rc;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_drop_constraint(p_parse: *mut Parse,
    p_src: *mut SrcList, p_cons: *mut Token, p_col: *mut Token) -> () {
    let db: *mut Sqlite3 = unsafe { (*p_parse).db };
    let mut p_tab: *mut Table = core::ptr::null_mut();
    let mut i_db: i32 = 0;
    let mut z_db: *const i8 = core::ptr::null();
    let mut z_arg: *mut i8 = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    p_tab =
        alter_find_table(p_parse, p_src, &mut i_db, &mut z_db,
            (p_cons != core::ptr::null_mut()) as i32);
    if (p_tab).is_null() as i32 != 0 { return; }
    if !(p_cons).is_null() {
        let z: *mut i8 =
            unsafe { sqlite3_name_from_token(db, p_cons as *const Token) };
        z_arg =
            unsafe {
                sqlite3_m_printf(db, c"%Q".as_ptr() as *mut i8 as *const i8,
                    z)
            };
        unsafe { sqlite3_db_free(db, z as *mut ()) };
    } else {
        let mut i_col: i32 = 0;
        if alter_find_col(p_parse, p_tab, p_col as *const Token, &mut i_col)
                != 0 {
            return;
        }
        z_arg =
            unsafe {
                sqlite3_m_printf(db, c"%d".as_ptr() as *mut i8 as *const i8,
                    i_col)
            };
    }
    unsafe {
        sqlite3_nested_parse(p_parse,
            c"UPDATE \"%w\".sqlite_master SET sql = sqlite_drop_constraint(sql, %s) WHERE type=\'table\' AND tbl_name=%Q COLLATE nocase".as_ptr()
                    as *mut i8 as *const i8, z_db, z_arg,
            unsafe { (*p_tab).z_name })
    };
    unsafe { sqlite3_db_free(db, z_arg as *mut ()) };
    rename_reload_schema(p_parse, i_db, 4 as u16);
}

extern "C" fn alter_rtrim_constraint(db: *mut Sqlite3, p_cons_1: *const i8,
    n_cons_1: i32) -> i32 {
    let z_tmp: *mut u8 =
        unsafe {
                sqlite3_m_printf(db, c"%.*s".as_ptr() as *mut i8 as *const i8,
                    n_cons_1, p_cons_1)
            } as *mut u8;
    let mut i_off: i32 = 0;
    let mut i_end: i32 = 0;
    if z_tmp == core::ptr::null_mut() { return 0; }
    loop {
        let mut t: i32 = 0;
        let n_token: i32 =
            unsafe {
                    sqlite3_get_token(unsafe {
                                &raw mut *z_tmp.offset(i_off as isize)
                            } as *const u8, &mut t)
                } as i32;
        if t == 186 { break; }
        if t != 184 &&
                (t != 185 ||
                    unsafe { *z_tmp.offset(i_off as isize) } as i32 !=
                        '-' as i32) {
            i_end = i_off + n_token;
        }
        i_off += n_token;
    }
    unsafe { sqlite3_db_free(db, z_tmp as *mut ()) };
    return i_end;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_add_constraint(p_parse: *mut Parse,
    p_src: *mut SrcList, p_first: &Token, p_name: *mut Token,
    z_expr: *const i8, n_expr: i32, p_expr: *mut Expr) -> () {
    unsafe {
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut i_db: i32 = 0;
        let mut z_db: *const i8 = core::ptr::null();
        let mut p_cons: *const i8 = core::ptr::null();
        let mut n_cons: i32 = 0;
        let mut rc: i32 = 0;
        { let _ = 0; };
        p_tab = alter_find_table(p_parse, p_src, &mut i_db, &mut z_db, 1);
        if (p_tab).is_null() as i32 != 0 {
            unsafe { sqlite3_expr_delete(unsafe { (*p_parse).db }, p_expr) };
            return;
        }
        rc =
            unsafe {
                sqlite3_resolve_self_reference(p_parse, p_tab, 4, p_expr,
                    core::ptr::null_mut())
            };
        unsafe { sqlite3_expr_delete(unsafe { (*p_parse).db }, p_expr) };
        if rc != 0 { return; }
        if !(p_name).is_null() {
            let z_name: *mut i8 =
                unsafe {
                    sqlite3_name_from_token(unsafe { (*p_parse).db },
                        p_name as *const Token)
                };
            unsafe {
                sqlite3_nested_parse(p_parse,
                    c"SELECT sqlite_fail(\'constraint %q already exists\', %d) FROM \"%w\".sqlite_master WHERE type=\'table\' AND tbl_name=%Q COLLATE nocase AND sqlite_find_constraint(sql, %Q)".as_ptr()
                            as *mut i8 as *const i8, z_name, 1, z_db,
                    unsafe { (*p_tab).z_name }, z_name)
            };
            unsafe {
                sqlite3_db_free(unsafe { (*p_parse).db }, z_name as *mut ())
            };
        }
        unsafe {
            sqlite3_nested_parse(p_parse,
                c"SELECT sqlite_fail(\'constraint failed\', %d) FROM %Q.%Q WHERE (%.*s) IS NOT TRUE".as_ptr()
                        as *mut i8 as *const i8, 19, z_db,
                unsafe { (*p_tab).z_name }, n_expr, z_expr)
        };
        p_cons = (*p_first).z;
        n_cons =
            alter_rtrim_constraint(unsafe { (*p_parse).db }, p_cons,
                unsafe {
                            unsafe { (*p_parse).s_last_token.z.offset_from(p_cons) }
                        } as i64 as i32);
        unsafe {
            sqlite3_nested_parse(p_parse,
                c"UPDATE \"%w\".sqlite_master SET sql = sqlite_add_constraint(sql, %.*Q, -1) WHERE type=\'table\' AND tbl_name=%Q COLLATE nocase".as_ptr()
                        as *mut i8 as *const i8, z_db, n_cons, p_cons,
                unsafe { (*p_tab).z_name })
        };
        rename_reload_schema(p_parse, i_db, 4 as u16);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_set_not_null(p_parse: *mut Parse,
    p_src: *mut SrcList, p_col: *mut Token, p_first: &Token) -> () {
    unsafe {
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut i_col: i32 = 0;
        let mut i_db: i32 = 0;
        let mut z_db: *const i8 = core::ptr::null();
        let mut p_cons: *const i8 = core::ptr::null();
        let mut n_cons: i32 = 0;
        { let _ = 0; };
        p_tab = alter_find_table(p_parse, p_src, &mut i_db, &mut z_db, 0);
        if (p_tab).is_null() as i32 != 0 { return; }
        if alter_find_col(p_parse, p_tab, p_col as *const Token, &mut i_col)
                != 0 {
            return;
        }
        p_cons = (*p_first).z;
        n_cons =
            alter_rtrim_constraint(unsafe { (*p_parse).db }, p_cons,
                unsafe {
                            unsafe { (*p_parse).s_last_token.z.offset_from(p_cons) }
                        } as i64 as i32);
        unsafe {
            sqlite3_nested_parse(p_parse,
                c"SELECT sqlite_fail(\'constraint failed\', %d) FROM %Q.%Q AS x WHERE x.%.*s IS NULL".as_ptr()
                        as *mut i8 as *const i8, 19, z_db,
                unsafe { (*p_tab).z_name }, unsafe { (*p_col).n } as i32,
                unsafe { (*p_col).z })
        };
        unsafe {
            sqlite3_nested_parse(p_parse,
                c"UPDATE \"%w\".sqlite_master SET sql = sqlite_add_constraint(sqlite_drop_constraint(sql, %d), %.*Q, %d) WHERE type=\'table\' AND tbl_name=%Q COLLATE nocase".as_ptr()
                        as *mut i8 as *const i8, z_db, i_col, n_cons, p_cons, i_col,
                unsafe { (*p_tab).z_name })
        };
        rename_reload_schema(p_parse, i_db, 4 as u16);
    }
}

extern "C" fn sqlite3_error_if_not_empty(p_parse_1: *mut Parse,
    z_db_1: *const i8, z_tab_1: *const i8, z_err_1: *const i8) -> () {
    unsafe {
        sqlite3_nested_parse(p_parse_1,
            c"SELECT raise(ABORT,%Q) FROM \"%w\".\"%w\"".as_ptr() as *mut i8
                as *const i8, z_err_1, z_db_1, z_tab_1)
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_finish_add_column(p_parse: *mut Parse,
    p_col_def: &mut Token) -> () {
    unsafe {
        unsafe {
            let mut p_new: *mut Table = core::ptr::null_mut();
            let mut p_tab: *const Table = core::ptr::null();
            let mut i_db: i32 = 0;
            let mut z_db: *const i8 = core::ptr::null();
            let mut z_tab: *const i8 = core::ptr::null();
            let mut z_col: *mut i8 = core::ptr::null_mut();
            let mut p_col: *mut Column = core::ptr::null_mut();
            let mut p_dflt: *const Expr = core::ptr::null();
            let mut db: *mut Sqlite3 = core::ptr::null_mut();
            let mut v: *mut Vdbe = core::ptr::null_mut();
            let mut r1: i32 = 0;
            db = unsafe { (*p_parse).db };
            { let _ = 0; };
            if unsafe { (*p_parse).n_err } != 0 { return; }
            { let _ = 0; };
            p_new = unsafe { (*p_parse).p_new_table };
            { let _ = 0; };
            { let _ = 0; };
            i_db =
                unsafe {
                    sqlite3_schema_to_index(db, unsafe { (*p_new).p_schema })
                };
            z_db =
                unsafe {
                        (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                    } as *const i8;
            z_tab =
                unsafe { unsafe { (*p_new).z_name.offset(16 as isize) } } as
                    *const i8;
            p_col =
                unsafe {
                    unsafe {
                        (*p_new).a_col.offset((unsafe { (*p_new).n_col } as i32 - 1)
                                as isize)
                    }
                };
            p_dflt = unsafe { sqlite3_column_expr(p_new, p_col) };
            p_tab = unsafe { sqlite3_find_table(db, z_tab, z_db) };
            { let _ = 0; };
            if unsafe {
                        sqlite3_auth_check(p_parse, 26, z_db,
                            unsafe { (*p_tab).z_name } as *const i8, core::ptr::null())
                    } != 0 {
                return;
            }
            if unsafe { (*p_col).col_flags } as i32 & 1 != 0 {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"Cannot add a PRIMARY KEY column".as_ptr() as *mut i8 as
                            *const i8)
                };
                return;
            }
            if !(unsafe { (*p_new).p_index }).is_null() {
                unsafe {
                    sqlite3_error_msg(p_parse,
                        c"Cannot add a UNIQUE column".as_ptr() as *mut i8 as
                            *const i8)
                };
                return;
            }
            if unsafe { (*p_col).col_flags } as i32 & 96 == 0 {
                { let _ = 0; };
                if !(p_dflt).is_null() &&
                        unsafe { (*unsafe { (*p_dflt).p_left }).op } as i32 == 122 {
                    p_dflt = core::ptr::null_mut();
                }
                { let _ = 0; };
                if unsafe { (*db).flags } & 16384 as u64 != 0 &&
                            !(unsafe { (*p_new).u.tab.p_f_key }).is_null() &&
                        !(p_dflt).is_null() {
                    sqlite3_error_if_not_empty(p_parse, z_db, z_tab,
                        c"Cannot add a REFERENCES column with non-NULL default value".as_ptr()
                                as *mut i8 as *const i8);
                }
                if unsafe { (*p_col).not_null() } != 0 &&
                        (p_dflt).is_null() as i32 != 0 {
                    sqlite3_error_if_not_empty(p_parse, z_db, z_tab,
                        c"Cannot add a NOT NULL column with default value NULL".as_ptr()
                                as *mut i8 as *const i8);
                }
                if !(p_dflt).is_null() {
                    let mut p_val: *mut Sqlite3Value = core::ptr::null_mut();
                    let mut rc: i32 = 0;
                    rc =
                        unsafe {
                            sqlite3_value_from_expr(db, p_dflt as *const Expr, 1 as u8,
                                65 as u8, &mut p_val)
                        };
                    { let _ = 0; };
                    if rc != 0 { { let _ = 0; }; return; }
                    if (p_val).is_null() as i32 != 0 {
                        sqlite3_error_if_not_empty(p_parse, z_db, z_tab,
                            c"Cannot add a column with non-constant default".as_ptr() as
                                    *mut i8 as *const i8);
                    }
                    unsafe { sqlite3ValueFree(p_val) };
                }
            } else if unsafe { (*p_col).col_flags } as i32 & 64 != 0 {
                sqlite3_error_if_not_empty(p_parse, z_db, z_tab,
                    c"cannot add a STORED column".as_ptr() as *mut i8 as
                        *const i8);
            }
            z_col =
                unsafe {
                    sqlite3_db_str_n_dup(db,
                        (*p_col_def).z as *mut i8 as *const i8,
                        (*p_col_def).n as u64)
                };
            if !(z_col).is_null() {
                let mut z_end: *mut i8 =
                    unsafe {
                        &mut *z_col.add(((*p_col_def).n - 1 as u32) as usize)
                    };
                while z_end > z_col &&
                        (unsafe { *z_end } as i32 == ';' as i32 ||
                            unsafe {
                                            *(sqlite3_ctype_map.as_ptr() as
                                                        *const u8).add(unsafe { *z_end } as u8 as usize)
                                        } as i32 & 1 != 0) {
                    unsafe {
                        *{
                                    let __p = &mut z_end;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(-1) };
                                    __t
                                } = '\u{0}' as i32 as i8
                    };
                }
                { let _ = 0; };
                { let _ = 0; };
                unsafe {
                    sqlite3_nested_parse(p_parse,
                        c"UPDATE \"%w\".sqlite_master SET sql = printf(\'%%.%ds, \',sql) || %Q || substr(sql,1+length(printf(\'%%.%ds\',sql))) WHERE type = \'table\' AND name = %Q".as_ptr()
                                as *mut i8 as *const i8, z_db,
                        unsafe { (*p_new).u.tab.add_col_offset }, z_col,
                        unsafe { (*p_new).u.tab.add_col_offset }, z_tab)
                };
                unsafe { sqlite3_db_free(db, z_col as *mut ()) };
            }
            v = unsafe { sqlite3_get_vdbe(p_parse) };
            if !(v).is_null() {
                r1 = unsafe { sqlite3_get_temp_reg(p_parse) };
                unsafe { sqlite3_vdbe_add_op3(v, 101, i_db, r1, 2) };
                unsafe { sqlite3_vdbe_uses_btree(v, i_db) };
                unsafe { sqlite3_vdbe_add_op2(v, 88, r1, -2) };
                unsafe {
                    sqlite3_vdbe_add_op2(v, 61, r1,
                        unsafe { sqlite3_vdbe_current_addr(v) } + 2)
                };
                unsafe { sqlite3_vdbe_add_op3(v, 102, i_db, 2, 3) };
                unsafe { sqlite3_release_temp_reg(p_parse, r1) };
                rename_reload_schema(p_parse, i_db, 3 as u16);
                if unsafe { (*p_new).p_check } != core::ptr::null_mut() ||
                            unsafe { (*p_col).not_null() } != 0 &&
                                unsafe { (*p_col).col_flags } as i32 & 96 != 0 ||
                        unsafe { (*p_tab).tab_flags } & 65536 as u32 != 0 as u32 {
                    unsafe {
                        sqlite3_nested_parse(p_parse,
                            c"SELECT CASE WHEN quick_check GLOB \'CHECK*\' THEN raise(ABORT,\'CHECK constraint failed\') WHEN quick_check GLOB \'non-* value in*\' THEN raise(ABORT,\'type mismatch on DEFAULT\') ELSE raise(ABORT,\'NOT NULL constraint failed\') END  FROM pragma_quick_check(%Q,%Q) WHERE quick_check GLOB \'CHECK*\' OR quick_check GLOB \'NULL*\' OR quick_check GLOB \'non-* value in*\'".as_ptr()
                                    as *mut i8 as *const i8, z_tab, z_db)
                    };
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_begin_add_column(p_parse: *mut Parse,
    p_src: *mut SrcList) -> () {
    unsafe {
        let mut p_new: *mut Table = core::ptr::null_mut();
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut i_db: i32 = 0;
        let mut i: i32 = 0;
        let mut n_alloc: i32 = 0;
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_col: *mut Column = core::ptr::null_mut();
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s41:
                {
                match __state {
                    0 => { __state = 3; }
                    2 => {
                        unsafe { sqlite3_src_list_delete(db, p_src) };
                        __state = 54;
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { db = unsafe { (*p_parse).db }; __state = 8; }
                    8 => { { let _ = 0; }; __state = 9; }
                    9 => { { let _ = 0; }; __state = 10; }
                    10 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 12;
                        } else { __state = 11; }
                    }
                    11 => {
                        p_tab =
                            unsafe {
                                sqlite3_locate_table_item(p_parse, 0 as u32,
                                    unsafe {
                                        &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)
                                    })
                            };
                        __state = 13;
                    }
                    12 => { __state = 2; }
                    13 => {
                        if (p_tab).is_null() as i32 != 0 {
                            __state = 15;
                        } else { __state = 14; }
                    }
                    14 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    15 => { __state = 2; }
                    16 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                            __state = 20;
                        } else { __state = 19; }
                    }
                    17 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"virtual tables may not be altered".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 18;
                    }
                    18 => { __state = 2; }
                    19 => {
                        if 0 != is_alterable_table(p_parse, unsafe { &*p_tab }) {
                            __state = 23;
                        } else { __state = 22; }
                    }
                    20 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"Cannot add a column to a view".as_ptr() as *mut i8 as
                                    *const i8)
                        };
                        __state = 21;
                    }
                    21 => { __state = 2; }
                    22 => {
                        unsafe { sqlite3_may_abort(p_parse) };
                        __state = 24;
                    }
                    23 => { __state = 2; }
                    24 => { { let _ = 0; }; __state = 25; }
                    25 => { { let _ = 0; }; __state = 26; }
                    26 => {
                        i_db =
                            unsafe {
                                sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                            };
                        __state = 27;
                    }
                    27 => {
                        p_new =
                            unsafe {
                                    sqlite3_db_malloc_zero(db,
                                        core::mem::size_of::<Table>() as u64)
                                } as *mut Table;
                        __state = 28;
                    }
                    28 => {
                        if (p_new).is_null() as i32 != 0 {
                            __state = 30;
                        } else { __state = 29; }
                    }
                    29 => {
                        unsafe { (*p_parse).p_new_table = p_new };
                        __state = 31;
                    }
                    30 => { __state = 2; }
                    31 => {
                        unsafe { (*p_new).n_tab_ref = 1 as u32 };
                        __state = 32;
                    }
                    32 => {
                        unsafe { (*p_new).n_col = unsafe { (*p_tab).n_col } };
                        __state = 33;
                    }
                    33 => { { let _ = 0; }; __state = 34; }
                    34 => {
                        n_alloc =
                            (unsafe { (*p_new).n_col } as i32 - 1) / 8 * 8 + 8;
                        __state = 35;
                    }
                    35 => { { let _ = 0; }; __state = 36; }
                    36 => {
                        unsafe {
                            (*p_new).a_col =
                                unsafe {
                                        sqlite3_db_malloc_zero(db,
                                            core::mem::size_of::<Column>() as u64 *
                                                n_alloc as u32 as u64)
                                    } as *mut Column
                        };
                        __state = 37;
                    }
                    37 => {
                        unsafe {
                            (*p_new).z_name =
                                unsafe {
                                    sqlite3_m_printf(db,
                                        c"sqlite_altertab_%s".as_ptr() as *mut i8 as *const i8,
                                        unsafe { (*p_tab).z_name })
                                }
                        };
                        __state = 38;
                    }
                    38 => {
                        if (unsafe { (*p_new).a_col }).is_null() as i32 != 0 ||
                                (unsafe { (*p_new).z_name }).is_null() as i32 != 0 {
                            __state = 40;
                        } else { __state = 39; }
                    }
                    39 => {
                        unsafe {
                            memcpy(unsafe { (*p_new).a_col } as *mut (),
                                unsafe { (*p_tab).a_col } as *const (),
                                core::mem::size_of::<Column>() as u64 *
                                    unsafe { (*p_new).n_col } as u64)
                        };
                        __state = 42;
                    }
                    40 => { { let _ = 0; }; __state = 41; }
                    41 => { __state = 2; }
                    42 => { i = 0; __state = 44; }
                    43 => { { let _ = 0; }; __state = 49; }
                    44 => {
                        if i < unsafe { (*p_new).n_col } as i32 {
                            __state = 45;
                        } else { __state = 43; }
                    }
                    45 => {
                        p_col =
                            unsafe { unsafe { (*p_new).a_col.offset(i as isize) } };
                        __state = 47;
                    }
                    46 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 44;
                    }
                    47 => {
                        unsafe {
                            (*p_col).z_cn_name =
                                unsafe {
                                    sqlite3_db_str_dup(db,
                                        unsafe { (*p_col).z_cn_name } as *const i8)
                                }
                        };
                        __state = 48;
                    }
                    48 => {
                        unsafe {
                            (*p_col).h_name =
                                unsafe {
                                    sqlite3_str_i_hash(unsafe { (*p_col).z_cn_name } as
                                            *const i8)
                                }
                        };
                        __state = 46;
                    }
                    49 => {
                        unsafe {
                            (*p_new).u.tab.p_dflt_list =
                                unsafe {
                                    sqlite3_expr_list_dup(db,
                                        unsafe { (*p_tab).u.tab.p_dflt_list } as *const ExprList, 0)
                                }
                        };
                        __state = 50;
                    }
                    50 => {
                        unsafe {
                            (*p_new).p_schema =
                                unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).p_schema
                                }
                        };
                        __state = 51;
                    }
                    51 => {
                        unsafe {
                            (*p_new).u.tab.add_col_offset =
                                unsafe { (*p_tab).u.tab.add_col_offset }
                        };
                        __state = 52;
                    }
                    52 => { { let _ = 0; }; __state = 53; }
                    53 => { __state = 2; }
                    54 => { return; }
                    _ => {}
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_alter_drop_column(p_parse: *mut Parse,
    p_src: *mut SrcList, p_name: *const Token) -> () {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut i_db: i32 = 0;
        let mut z_db: *const i8 = core::ptr::null();
        let mut z_col: *mut i8 = core::ptr::null_mut();
        let mut i_col: i32 = 0;
        let mut i: i32 = 0;
        let mut addr: i32 = 0;
        let mut reg: i32 = 0;
        let mut reg_rec: i32 = 0;
        let mut p_pk: *mut Index = core::ptr::null_mut();
        let mut n_field: i32 = 0;
        let mut i_cur: i32 = 0;
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let mut reg_out: i32 = 0;
        let mut i_pos: i32 = 0;
        let mut i_col_pos: i32 = 0;
        let mut aff: i8 = 0 as i8;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s43:
                {
                match __state {
                    0 => { db = unsafe { (*p_parse).db }; __state = 3; }
                    2 => {
                        unsafe { sqlite3_db_free(db, z_col as *mut ()) };
                        __state = 101;
                    }
                    3 => { __state = 4; }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { z_col = core::ptr::null_mut(); __state = 7; }
                    7 => { __state = 8; }
                    8 => { { let _ = 0; }; __state = 9; }
                    9 => { { let _ = 0; }; __state = 10; }
                    10 => {
                        if unsafe { (*db).malloc_failed } != 0 {
                            __state = 12;
                        } else { __state = 11; }
                    }
                    11 => {
                        p_tab =
                            unsafe {
                                sqlite3_locate_table_item(p_parse, 0 as u32,
                                    unsafe {
                                        &mut *(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset(0 as isize)
                                    })
                            };
                        __state = 13;
                    }
                    12 => { __state = 2; }
                    13 => {
                        if (p_tab).is_null() as i32 != 0 {
                            __state = 15;
                        } else { __state = 14; }
                    }
                    14 => {
                        if 0 != is_alterable_table(p_parse, unsafe { &*p_tab }) {
                            __state = 17;
                        } else { __state = 16; }
                    }
                    15 => { __state = 2; }
                    16 => {
                        if 0 != is_real_table(p_parse, unsafe { &*p_tab }, 1) {
                            __state = 19;
                        } else { __state = 18; }
                    }
                    17 => { __state = 2; }
                    18 => {
                        z_col = unsafe { sqlite3_name_from_token(db, p_name) };
                        __state = 20;
                    }
                    19 => { __state = 2; }
                    20 => {
                        if z_col == core::ptr::null_mut() {
                            __state = 22;
                        } else { __state = 21; }
                    }
                    21 => {
                        i_col =
                            unsafe { sqlite3_column_index(p_tab, z_col as *const i8) };
                        __state = 24;
                    }
                    22 => { { let _ = 0; }; __state = 23; }
                    23 => { __state = 2; }
                    24 => {
                        if i_col < 0 { __state = 26; } else { __state = 25; }
                    }
                    25 => {
                        if unsafe {
                                            (*unsafe {
                                                        (*p_tab).a_col.offset(i_col as isize)
                                                    }).col_flags
                                        } as i32 & (1 | 8) != 0 {
                            __state = 29;
                        } else { __state = 28; }
                    }
                    26 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"no such column: \"%T\"".as_ptr() as *mut i8 as *const i8,
                                p_name)
                        };
                        __state = 27;
                    }
                    27 => { __state = 2; }
                    28 => {
                        if unsafe { (*p_tab).n_col } as i32 <= 1 {
                            __state = 32;
                        } else { __state = 31; }
                    }
                    29 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot drop %s column: \"%s\"".as_ptr() as *mut i8 as
                                    *const i8,
                                if unsafe {
                                                    (*unsafe {
                                                                (*p_tab).a_col.offset(i_col as isize)
                                                            }).col_flags
                                                } as i32 & 1 != 0 {
                                    c"PRIMARY KEY".as_ptr() as *mut i8
                                } else { c"UNIQUE".as_ptr() as *mut i8 }, z_col)
                        };
                        __state = 30;
                    }
                    30 => { __state = 2; }
                    31 => {
                        i_db =
                            unsafe {
                                sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                            };
                        __state = 34;
                    }
                    32 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot drop column \"%s\": no other columns exist".as_ptr()
                                        as *mut i8 as *const i8, z_col)
                        };
                        __state = 33;
                    }
                    33 => { __state = 2; }
                    34 => { { let _ = 0; }; __state = 35; }
                    35 => {
                        z_db =
                            unsafe {
                                    (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                } as *const i8;
                        __state = 36;
                    }
                    36 => {
                        if unsafe {
                                    sqlite3_auth_check(p_parse, 26, z_db,
                                        unsafe { (*p_tab).z_name } as *const i8, z_col as *const i8)
                                } != 0 {
                            __state = 38;
                        } else { __state = 37; }
                    }
                    37 => {
                        rename_test_schema(p_parse, z_db, (i_db == 1) as i32,
                            c"".as_ptr() as *mut i8 as *const i8, 0);
                        __state = 39;
                    }
                    38 => { __state = 2; }
                    39 => {
                        rename_fix_quotes(p_parse, z_db, (i_db == 1) as i32);
                        __state = 40;
                    }
                    40 => {
                        unsafe {
                            sqlite3_nested_parse(p_parse,
                                c"UPDATE \"%w\".sqlite_master SET sql = sqlite_drop_column(%d, sql, %d) WHERE (type==\'table\' AND tbl_name=%Q COLLATE nocase)".as_ptr()
                                        as *mut i8 as *const i8, z_db, i_db, i_col,
                                unsafe { (*p_tab).z_name })
                        };
                        __state = 41;
                    }
                    41 => {
                        rename_reload_schema(p_parse, i_db, 2 as u16);
                        __state = 42;
                    }
                    42 => {
                        rename_test_schema(p_parse, z_db, (i_db == 1) as i32,
                            c"after drop column".as_ptr() as *mut i8 as *const i8, 1);
                        __state = 43;
                    }
                    43 => {
                        if unsafe { (*p_parse).n_err } == 0 &&
                                unsafe {
                                                (*unsafe {
                                                            (*p_tab).a_col.offset(i_col as isize)
                                                        }).col_flags
                                            } as i32 & 32 == 0 {
                            __state = 45;
                        } else { __state = 44; }
                    }
                    44 => { __state = 2; }
                    45 => { __state = 46; }
                    46 => { __state = 47; }
                    47 => { __state = 48; }
                    48 => { __state = 49; }
                    49 => { p_pk = core::ptr::null_mut(); __state = 50; }
                    50 => { n_field = 0; __state = 51; }
                    51 => { __state = 52; }
                    52 => {
                        v = unsafe { sqlite3_get_vdbe(p_parse) };
                        __state = 53;
                    }
                    53 => {
                        i_cur =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 54;
                    }
                    54 => {
                        unsafe {
                            sqlite3_open_table(p_parse, i_cur, i_db, p_tab, 116)
                        };
                        __state = 55;
                    }
                    55 => {
                        addr = unsafe { sqlite3_vdbe_add_op1(v, 36, i_cur) };
                        __state = 56;
                    }
                    56 => { __state = 57; }
                    57 => {
                        reg =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 58;
                    }
                    58 => {
                        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                            __state = 60;
                        } else { __state = 61; }
                    }
                    59 => {
                        reg_rec =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 69;
                    }
                    60 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 137, i_cur, reg) };
                        __state = 62;
                    }
                    61 => {
                        p_pk = unsafe { sqlite3_primary_key_index(p_tab) };
                        __state = 63;
                    }
                    62 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_tab).n_col } as i32
                        };
                        __state = 59;
                    }
                    63 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_pk).n_column } as i32
                        };
                        __state = 64;
                    }
                    64 => { i = 0; __state = 66; }
                    65 => {
                        n_field = unsafe { (*p_pk).n_key_col } as i32;
                        __state = 59;
                    }
                    66 => {
                        if i < unsafe { (*p_pk).n_key_col } as i32 {
                            __state = 67;
                        } else { __state = 65; }
                    }
                    67 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, i_cur, i, reg + i + 1)
                        };
                        __state = 68;
                    }
                    68 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 66;
                    }
                    69 => { i = 0; __state = 71; }
                    70 => {
                        if n_field == 0 { __state = 91; } else { __state = 90; }
                    }
                    71 => {
                        if i < unsafe { (*p_tab).n_col } as i32 {
                            __state = 72;
                        } else { __state = 70; }
                    }
                    72 => {
                        if i != i_col &&
                                unsafe {
                                                (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                            } as i32 & 32 == 0 {
                            __state = 74;
                        } else { __state = 73; }
                    }
                    73 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 71;
                    }
                    74 => { __state = 75; }
                    75 => {
                        if !(p_pk).is_null() {
                            __state = 77;
                        } else { __state = 78; }
                    }
                    76 => {
                        if i == unsafe { (*p_tab).i_p_key } as i32 {
                            __state = 84;
                        } else { __state = 85; }
                    }
                    77 => {
                        i_pos = unsafe { sqlite3_table_column_to_index(p_pk, i) };
                        __state = 79;
                    }
                    78 => { reg_out = reg + 1 + n_field; __state = 76; }
                    79 => {
                        i_col_pos =
                            unsafe { sqlite3_table_column_to_index(p_pk, i_col) };
                        __state = 80;
                    }
                    80 => {
                        if i_pos < unsafe { (*p_pk).n_key_col } as i32 {
                            __state = 82;
                        } else { __state = 81; }
                    }
                    81 => {
                        reg_out = reg + 1 + i_pos - (i_pos > i_col_pos) as i32;
                        __state = 76;
                    }
                    82 => { __state = 73; }
                    83 => {
                        { let __p = &mut n_field; let __t = *__p; *__p += 1; __t };
                        __state = 73;
                    }
                    84 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, reg_out) };
                        __state = 83;
                    }
                    85 => {
                        aff =
                            unsafe {
                                (*unsafe { (*p_tab).a_col.offset(i as isize) }).affinity
                            };
                        __state = 86;
                    }
                    86 => {
                        if aff as i32 == 69 { __state = 88; } else { __state = 87; }
                    }
                    87 => {
                        unsafe {
                            sqlite3_expr_code_get_column_of_table(v, p_tab, i_cur, i,
                                reg_out)
                        };
                        __state = 89;
                    }
                    88 => {
                        unsafe {
                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).affinity =
                                67 as i8
                        };
                        __state = 87;
                    }
                    89 => {
                        unsafe {
                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).affinity =
                                aff
                        };
                        __state = 83;
                    }
                    90 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg + 1, n_field, reg_rec)
                        };
                        __state = 94;
                    }
                    91 => {
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 92;
                    }
                    92 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, reg + 1) };
                        __state = 93;
                    }
                    93 => { n_field = 1; __state = 90; }
                    94 => {
                        if !(p_pk).is_null() {
                            __state = 96;
                        } else { __state = 97; }
                    }
                    95 => {
                        unsafe { sqlite3_vdbe_change_p5(v, 2 as u16) };
                        __state = 98;
                    }
                    96 => {
                        unsafe {
                            sqlite3_vdbe_add_op4_int(v, 140, i_cur, reg_rec, reg + 1,
                                unsafe { (*p_pk).n_key_col } as i32)
                        };
                        __state = 95;
                    }
                    97 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 130, i_cur, reg_rec, reg)
                        };
                        __state = 95;
                    }
                    98 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 40, i_cur, addr + 1) };
                        __state = 99;
                    }
                    99 => { __state = 100; }
                    100 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr) };
                        __state = 44;
                    }
                    101 => {
                        unsafe { sqlite3_src_list_delete(db, p_src) };
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rename_token_map(p_parse: &mut Parse,
    p_ptr: *const (), p_token: &Token) -> *const () {
    unsafe {
        let mut p_new: *mut RenameToken = core::ptr::null_mut();
        { let _ = 0; };
        if (*p_parse).e_parse_mode as i32 != 3 {
            p_new =
                unsafe {
                        sqlite3_db_malloc_zero((*p_parse).db,
                            core::mem::size_of::<RenameToken>() as u64)
                    } as *mut RenameToken;
            if !(p_new).is_null() {
                unsafe { (*p_new).p = p_ptr };
                unsafe { (*p_new).t = *p_token };
                unsafe { (*p_new).p_next = (*p_parse).p_rename };
                (*p_parse).p_rename = p_new;
            }
        }
        return p_ptr;
    }
}

extern "C" fn unmap_column_idlist_names(p_parse_1: *mut Parse,
    p_id_list_1: &IdList) -> () {
    let mut ii: i32 = 0;
    { let _ = 0; };
    {
        ii = 0;
        '__b44: loop {
            if !(ii < (*p_id_list_1).n_id) { break '__b44; }
            '__c44: loop {
                sqlite3_rename_token_remap(unsafe { &*p_parse_1 },
                    core::ptr::null(),
                    unsafe {
                            (*((*p_id_list_1).a.as_ptr() as
                                            *const IdListItem).offset(ii as isize)).z_name
                        } as *const ());
                break '__c44;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn rename_unmap_select_cb(p_walker_1: *mut Walker, p: *mut Select)
    -> i32 {
    unsafe {
        let p_parse: *mut Parse = unsafe { (*p_walker_1).p_parse };
        let mut i: i32 = 0;
        if unsafe { (*p_parse).n_err } != 0 { return 2; }
        if unsafe { (*p).sel_flags } & (2097152 | 67108864) as u32 != 0 {
            return 1;
        }
        if !(unsafe { (*p).p_e_list }).is_null() {
            let p_list: *mut ExprList = unsafe { (*p).p_e_list };
            {
                i = 0;
                '__b45: loop {
                    if !(i < unsafe { (*p_list).n_expr }) { break '__b45; }
                    '__c45: loop {
                        if !(unsafe {
                                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).z_e_name
                                            }).is_null() &&
                                unsafe {
                                            (*(unsafe { (*p_list).a.as_ptr() } as
                                                                *mut ExprListItem).offset(i as isize)).fg.e_e_name()
                                        } as i32 == 0 {
                            sqlite3_rename_token_remap(unsafe { &*p_parse },
                                core::ptr::null(),
                                unsafe {
                                            (*(unsafe { (*p_list).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).z_e_name
                                        } as *mut () as *const ());
                        }
                        break '__c45;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if !(unsafe { (*p).p_src }).is_null() {
            let p_src: *mut SrcList = unsafe { (*p).p_src };
            {
                i = 0;
                '__b46: loop {
                    if !(i < unsafe { (*p_src).n_src }) { break '__b46; }
                    '__c46: loop {
                        sqlite3_rename_token_remap(unsafe { &*p_parse },
                            core::ptr::null(),
                            unsafe {
                                        (*(unsafe { (*p_src).a.as_ptr() } as
                                                        *mut SrcItem).offset(i as isize)).z_name
                                    } as *mut () as *const ());
                        if unsafe {
                                        (*(unsafe { (*p_src).a.as_ptr() } as
                                                            *mut SrcItem).offset(i as isize)).fg.is_using()
                                    } as i32 == 0 {
                            unsafe {
                                sqlite3_walk_expr(p_walker_1,
                                    unsafe {
                                        (*(unsafe { (*p_src).a.as_ptr() } as
                                                            *mut SrcItem).offset(i as isize)).u3.p_on
                                    })
                            };
                        } else {
                            unmap_column_idlist_names(p_parse,
                                unsafe {
                                    &*unsafe {
                                                (*(unsafe { (*p_src).a.as_ptr() } as
                                                                    *mut SrcItem).offset(i as isize)).u3.p_using
                                            }
                                });
                        }
                        break '__c46;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        rename_walk_with(p_walker_1, unsafe { &*p });
        return 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_rename_expr_unmap(p_parse: *mut Parse,
    p_expr: *mut Expr) -> () {
    let e_mode: u8 = unsafe { (*p_parse).e_parse_mode };
    let mut s_walker: Walker = unsafe { core::mem::zeroed() };
    unsafe {
        memset(&raw mut s_walker as *mut (), 0,
            core::mem::size_of::<Walker>() as u64)
    };
    s_walker.p_parse = p_parse;
    s_walker.x_expr_callback = Some(rename_unmap_expr_cb);
    s_walker.x_select_callback = Some(rename_unmap_select_cb);
    unsafe { (*p_parse).e_parse_mode = 3 as u8 };
    unsafe { sqlite3_walk_expr(&mut s_walker, p_expr) };
    unsafe { (*p_parse).e_parse_mode = e_mode };
}

static mut a_alter_table_funcs: [FuncDef; 9] =
    [FuncDef {
                n_arg: 9 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(rename_column_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_rename_column".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 7 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(rename_table_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_rename_table".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 7 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(rename_table_test),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_rename_test".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 3 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(drop_column_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_drop_column".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(rename_quotefix_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_rename_quotefix".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(drop_constraint_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_drop_constraint".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(fail_constraint_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_fail".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 3 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(add_constraint_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_add_constraint".as_ptr() as *const i8,
                u: FuncDefU0 { p_hash: core::ptr::null_mut() },
            },
            FuncDef {
                n_arg: 2 as i16,
                func_flags: (8388608 | 262144 | 1 | 2048) as u32,
                p_user_data: core::ptr::null_mut(),
                p_next: core::ptr::null_mut(),
                x_s_func: Some(find_constraint_func),
                x_finalize: None,
                x_value: None,
                x_inverse: None,
                z_name: c"sqlite_find_constraint".as_ptr() as *const i8,
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
    static sqlite3_ctype_map: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3_builtin_functions: FuncDefHash;
    static sqlite3_oom_str: Sqlite3Str;
    static mut sqlite3_pending_byte: i32;
    fn sqlite3_root_page_moved(_: *mut Sqlite3, _: i32, _: Pgno, _: Pgno)
    -> ();
    fn sqlite3_reindex(_: *mut Parse, _: *mut Token, _: *mut Token)
    -> ();
    fn sqlite3_parse_object_init(_: *mut Parse, _: *mut Sqlite3)
    -> ();
    fn sqlite3_find_db_name(_: *mut Sqlite3, _: *const i8)
    -> i32;
    fn sqlite3_with_push(_: *mut Parse, _: *mut With, _: u8)
    -> *mut With;
    fn sqlite3_select_prep(_: *mut Parse, _: *mut Select, _: *mut NameContext)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
    -> i32;
    fn sqlite3_resolve_expr_list_names(_: *mut NameContext, _: *mut ExprList)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn memmove(__dst: *mut (), __src: *const (), __len: u64)
    -> *mut ();
    fn sqlite3_parse_object_reset(_: *mut Parse)
    -> ();
    fn sqlite3_get_token(_: *const u8, _: *mut i32)
    -> i64;
    fn sqlite3_is_shadow_table_of(_: *mut Sqlite3, _: *mut Table,
    _: *const i8)
    -> i32;
    fn sqlite3_read_only_shadow_tables(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_get_v_table(_: *mut Sqlite3, _: *mut Table)
    -> *mut VTable;
    fn sqlite3_nested_parse(_: *mut Parse, _: *const i8, ...)
    -> ();
    fn sqlite3_resolve_self_reference(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut Expr, _: *mut ExprList)
    -> i32;
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
    fn sqlite3_resolve_order_group_by(_: *mut Parse, _: *mut Select,
    _: *mut ExprList, _: *const i8)
    -> i32;
    fn sqlite3_column_default(_: *mut Vdbe, _: *mut Table, _: i32, _: i32)
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
    fn sqlite3_vtab_create_module(_: *mut Sqlite3, _: *const i8,
    _: *const Sqlite3Module, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> *mut Module;
    fn sqlite3_shadow_table_name(db: *mut Sqlite3, z_name_1: *const i8)
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
struct WhereConst {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct WindowRewrite {
    _opaque: [u8; 0],
}
