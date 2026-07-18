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
extern "C" fn expr_list_is_constant(p_parse_1: *mut Parse, p_row_1: &ExprList)
    -> i32 {
    let mut ii: i32 = 0;
    {
        ii = 0;
        '__b0: loop {
            if !(ii < (*p_row_1).n_expr) { break '__b0; }
            '__c0: loop {
                if 0 ==
                        unsafe {
                            sqlite3_expr_is_constant(p_parse_1,
                                unsafe {
                                    (*((*p_row_1).a.as_ptr() as
                                                    *mut ExprListItem).offset(ii as isize)).p_expr
                                })
                        } {
                    return 0;
                }
                break '__c0;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}
extern "C" fn expr_list_is_no_affinity(p_parse_1: *mut Parse,
    p_row_1: *mut ExprList) -> i32 {
    let mut ii: i32 = 0;
    if expr_list_is_constant(p_parse_1, unsafe { &*p_row_1 }) == 0 {
        return 0;
    }
    {
        ii = 0;
        '__b1: loop {
            if !(ii < unsafe { (*p_row_1).n_expr }) { break '__b1; }
            '__c1: loop {
                let p_expr: *const Expr =
                    unsafe {
                            (*(unsafe { (*p_row_1).a.as_ptr() } as
                                            *mut ExprListItem).offset(ii as isize)).p_expr
                        } as *const Expr;
                { let _ = 0; };
                { let _ = 0; };
                if 0 !=
                        unsafe { sqlite3_expr_affinity(p_expr as *const Expr) } as
                            i32 {
                    return 0;
                }
                break '__c1;
            }
            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
        }
    }
    return 1;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_multi_values_end(p_parse: &Parse,
    p_val: *mut Select) -> () {
    unsafe {
        if !(p_val).is_null() &&
                unsafe { (*unsafe { (*p_val).p_src }).n_src } > 0 {
            let p_item: *const SrcItem =
                unsafe {
                        &raw mut *(unsafe {
                                            (*unsafe { (*p_val).p_src }).a.as_ptr()
                                        } as *mut SrcItem).offset(0 as isize)
                    } as *const SrcItem;
            { let _ = 0; };
            if unsafe { (*p_item).fg.is_subquery() } != 0 {
                unsafe {
                    sqlite3_vdbe_end_coroutine((*p_parse).p_vdbe,
                        unsafe { (*unsafe { (*p_item).u4.p_subq }).reg_return })
                };
                unsafe {
                    sqlite3_vdbe_jump_here((*p_parse).p_vdbe,
                        unsafe { (*unsafe { (*p_item).u4.p_subq }).addr_fill_sub } -
                            1)
                };
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_multi_values(p_parse: *mut Parse,
    mut p_left: *mut Select, p_row: *mut ExprList) -> *mut Select {
    unsafe {
        if unsafe { (*p_parse).b_has_with() } != 0 ||
                            unsafe { (*unsafe { (*p_parse).db }).init.busy } != 0 ||
                        expr_list_is_constant(p_parse, unsafe { &*p_row }) == 0 ||
                    unsafe { (*unsafe { (*p_left).p_src }).n_src } == 0 &&
                        expr_list_is_no_affinity(p_parse,
                                unsafe { (*p_left).p_e_list }) == 0 ||
                unsafe { (*p_parse).e_parse_mode } as i32 != 0 {
            let mut p_select: *mut Select = core::ptr::null_mut();
            let mut f: i32 = 512 | 1024;
            if unsafe { (*unsafe { (*p_left).p_src }).n_src } != 0 {
                sqlite3_multi_values_end(unsafe { &*p_parse }, p_left);
                f = 512;
            } else if !(unsafe { (*p_left).p_prior }).is_null() {
                f = (f as u32 & unsafe { (*p_left).sel_flags }) as i32;
            }
            p_select =
                unsafe {
                    sqlite3_select_new(p_parse, p_row, core::ptr::null_mut(),
                        core::ptr::null_mut(), core::ptr::null_mut(),
                        core::ptr::null_mut(), core::ptr::null_mut(), f as u32,
                        core::ptr::null_mut())
                };
            unsafe { (*p_left).sel_flags &= !(1024 as u32) };
            if !(p_select).is_null() {
                unsafe { (*p_select).op = 136 as u8 };
                unsafe { (*p_select).p_prior = p_left };
                p_left = p_select;
            }
        } else {
            let mut p: *mut SrcItem = core::ptr::null_mut();
            if unsafe { (*unsafe { (*p_left).p_src }).n_src } == 0 {
                let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p_parse) };
                let p_ret: *mut Select =
                    unsafe {
                        sqlite3_select_new(p_parse, core::ptr::null_mut(),
                            core::ptr::null_mut(), core::ptr::null_mut(),
                            core::ptr::null_mut(), core::ptr::null_mut(),
                            core::ptr::null_mut(), 0 as u32, core::ptr::null_mut())
                    };
                if unsafe { (*unsafe { (*p_parse).db }).m_db_flags } &
                            16 as u32 == 0 as u32 {
                    unsafe { sqlite3_read_schema(p_parse) };
                }
                if !(p_ret).is_null() {
                    let mut dest: SelectDest = unsafe { core::mem::zeroed() };
                    let mut p_subq: *mut Subquery = core::ptr::null_mut();
                    unsafe { (*unsafe { (*p_ret).p_src }).n_src = 1 };
                    unsafe { (*p_ret).p_prior = unsafe { (*p_left).p_prior } };
                    unsafe { (*p_ret).op = unsafe { (*p_left).op } };
                    if !(unsafe { (*p_ret).p_prior }).is_null() {
                        unsafe { (*p_ret).sel_flags |= 512 as u32 };
                    }
                    unsafe { (*p_left).p_prior = core::ptr::null_mut() };
                    unsafe { (*p_left).op = 139 as u8 };
                    { let _ = 0; };
                    { let _ = 0; };
                    p =
                        unsafe {
                            &mut *(unsafe { (*unsafe { (*p_ret).p_src }).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as isize)
                        };
                    unsafe { (*p).fg.set_via_coroutine(1 as u32 as u32) };
                    unsafe { (*p).i_cursor = -1 };
                    { let _ = 0; };
                    unsafe { (*p).u1.n_row = 2 as u32 };
                    if unsafe {
                                sqlite3_src_item_attach_subquery(p_parse, p, p_left, 0)
                            } != 0 {
                        p_subq = unsafe { (*p).u4.p_subq };
                        unsafe {
                            (*p_subq).addr_fill_sub =
                                unsafe { sqlite3_vdbe_current_addr(v) } + 1
                        };
                        unsafe {
                            (*p_subq).reg_return =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 11, unsafe { (*p_subq).reg_return },
                                0, unsafe { (*p_subq).addr_fill_sub })
                        };
                        unsafe {
                            sqlite3_select_dest_init(&mut dest, 11,
                                unsafe { (*p_subq).reg_return })
                        };
                        dest.i_sdst = unsafe { (*p_parse).n_mem } + 3;
                        dest.n_sdst =
                            unsafe { (*unsafe { (*p_left).p_e_list }).n_expr };
                        unsafe { (*p_parse).n_mem += 2 + dest.n_sdst };
                        unsafe { (*p_left).sel_flags |= 1024 as u32 };
                        unsafe { sqlite3_select(p_parse, p_left, &mut dest) };
                        unsafe { (*p_subq).reg_result = dest.i_sdst };
                        { let _ = 0; };
                    }
                    p_left = p_ret;
                }
            } else {
                p =
                    unsafe {
                        &mut *(unsafe { (*unsafe { (*p_left).p_src }).a.as_ptr() }
                                        as *mut SrcItem).offset(0 as isize)
                    };
                { let _ = 0; };
                {
                    let __p = unsafe { &mut (*p).u1.n_row };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
            if unsafe { (*p_parse).n_err } == 0 {
                let mut p_subq_1: *const Subquery = core::ptr::null();
                { let _ = 0; };
                { let _ = 0; };
                p_subq_1 = unsafe { (*p).u4.p_subq };
                { let _ = 0; };
                { let _ = 0; };
                { let _ = 0; };
                if unsafe {
                            (*unsafe {
                                            (*unsafe { (*p_subq_1).p_select }).p_e_list
                                        }).n_expr
                        } != unsafe { (*p_row).n_expr } {
                    unsafe {
                        sqlite3_select_wrong_num_terms_error(p_parse,
                            unsafe { (*p_subq_1).p_select })
                    };
                } else {
                    unsafe {
                        sqlite3_expr_code_expr_list(p_parse, p_row,
                            unsafe { (*p_subq_1).reg_result }, 0, 0 as u8)
                    };
                    unsafe {
                        sqlite3_vdbe_add_op1(unsafe { (*p_parse).p_vdbe }, 12,
                            unsafe { (*p_subq_1).reg_return })
                    };
                }
            }
            unsafe {
                sqlite3_expr_list_delete(unsafe { (*p_parse).db }, p_row)
            };
        }
        return p_left;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_open_table(p_parse: *mut Parse, i_cur: i32,
    i_db: i32, p_tab: *mut Table, opcode: i32) -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    { let _ = 0; };
    { let _ = 0; };
    v = unsafe { (*p_parse).p_vdbe };
    { let _ = 0; };
    if (unsafe { (*unsafe { (*p_parse).db }).no_shared_cache } == 0) as i32 !=
            0 {
        unsafe {
            sqlite3_table_lock(p_parse, i_db, unsafe { (*p_tab).tnum },
                if opcode == 116 { 1 } else { 0 } as u8,
                unsafe { (*p_tab).z_name } as *const i8)
        };
    }
    if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
        unsafe {
            sqlite3_vdbe_add_op4_int(v, opcode, i_cur,
                unsafe { (*p_tab).tnum } as i32, i_db,
                unsafe { (*p_tab).n_nv_col } as i32)
        };
    } else {
        let p_pk: *mut Index = unsafe { sqlite3_primary_key_index(p_tab) };
        { let _ = 0; };
        { let _ = 0; };
        unsafe {
            sqlite3_vdbe_add_op3(v, opcode, i_cur,
                unsafe { (*p_pk).tnum } as i32, i_db)
        };
        unsafe { sqlite3_vdbe_set_p4_key_info(p_parse, p_pk) };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_autoincrement_begin(p_parse: *mut Parse) -> () {
    unsafe {
        let mut p: *const AutoincInfo = core::ptr::null();
        let db: *const Sqlite3 = unsafe { (*p_parse).db } as *const Sqlite3;
        let mut p_db: *const Db = core::ptr::null();
        let mut mem_id: i32 = 0;
        let v: *mut Vdbe = unsafe { (*p_parse).p_vdbe };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        {
            p = unsafe { (*p_parse).p_ainc };
            '__b2: loop {
                if !(!(p).is_null()) { break '__b2; }
                '__c2: loop {
                    let mut a_op: *mut VdbeOp = core::ptr::null_mut();
                    p_db =
                        unsafe {
                            unsafe { (*db).a_db.offset(unsafe { (*p).i_db } as isize) }
                        };
                    mem_id = unsafe { (*p).reg_ctr };
                    { let _ = 0; };
                    sqlite3_open_table(p_parse, 0, unsafe { (*p).i_db },
                        unsafe { (*unsafe { (*p_db).p_schema }).p_seq_tab }, 114);
                    unsafe {
                        sqlite3_vdbe_load_string(v, mem_id - 1,
                            unsafe { (*unsafe { (*p).p_tab }).z_name } as *const i8)
                    };
                    a_op =
                        unsafe {
                            sqlite3_vdbe_add_op_list(v,
                                (core::mem::size_of::<[VdbeOpList; 12]>() as u64 /
                                        core::mem::size_of::<VdbeOpList>() as u64) as i32,
                                &raw const auto_inc[0 as usize] as *const VdbeOpList,
                                i_ln_1)
                        };
                    if a_op == core::ptr::null_mut() { break '__b2; }
                    unsafe { (*a_op.offset(0 as isize)).p2 = mem_id };
                    unsafe { (*a_op.offset(0 as isize)).p3 = mem_id + 2 };
                    unsafe { (*a_op.offset(2 as isize)).p3 = mem_id };
                    unsafe { (*a_op.offset(3 as isize)).p1 = mem_id - 1 };
                    unsafe { (*a_op.offset(3 as isize)).p3 = mem_id };
                    unsafe { (*a_op.offset(3 as isize)).p5 = 16 as u16 };
                    unsafe { (*a_op.offset(4 as isize)).p2 = mem_id + 1 };
                    unsafe { (*a_op.offset(5 as isize)).p3 = mem_id };
                    unsafe { (*a_op.offset(6 as isize)).p1 = mem_id };
                    unsafe { (*a_op.offset(7 as isize)).p2 = mem_id + 2 };
                    unsafe { (*a_op.offset(7 as isize)).p1 = mem_id };
                    unsafe { (*a_op.offset(10 as isize)).p2 = mem_id };
                    if unsafe { (*p_parse).n_tab } == 0 {
                        unsafe { (*p_parse).n_tab = 1 };
                    }
                    break '__c2;
                }
                p = unsafe { (*p).p_next };
            }
        }
    }
}
extern "C" fn auto_increment_end(p_parse_1: *mut Parse) -> () {
    unsafe {
        let mut p: *const AutoincInfo = core::ptr::null();
        let v: *mut Vdbe = unsafe { (*p_parse_1).p_vdbe };
        let db: *const Sqlite3 = unsafe { (*p_parse_1).db } as *const Sqlite3;
        { let _ = 0; };
        { let _ = 0; };
        {
            p = unsafe { (*p_parse_1).p_ainc };
            '__b3: loop {
                if !(!(p).is_null()) { break '__b3; }
                '__c3: loop {
                    let mut a_op: *mut VdbeOp = core::ptr::null_mut();
                    let p_db: *const Db =
                        unsafe {
                                &raw mut *unsafe {
                                            (*db).a_db.offset(unsafe { (*p).i_db } as isize)
                                        }
                            } as *const Db;
                    let mut i_rec: i32 = 0;
                    let mem_id: i32 = unsafe { (*p).reg_ctr };
                    i_rec = unsafe { sqlite3_get_temp_reg(p_parse_1) };
                    { let _ = 0; };
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 56, mem_id + 2,
                            unsafe { sqlite3_vdbe_current_addr(v) } + 7, mem_id)
                    };
                    sqlite3_open_table(p_parse_1, 0, unsafe { (*p).i_db },
                        unsafe { (*unsafe { (*p_db).p_schema }).p_seq_tab }, 116);
                    a_op =
                        unsafe {
                            sqlite3_vdbe_add_op_list(v,
                                (core::mem::size_of::<[VdbeOpList; 5]>() as u64 /
                                        core::mem::size_of::<VdbeOpList>() as u64) as i32,
                                &raw const auto_inc_end[0 as usize] as *const VdbeOpList,
                                i_ln_2)
                        };
                    if a_op == core::ptr::null_mut() { break '__b3; }
                    unsafe { (*a_op.offset(0 as isize)).p1 = mem_id + 1 };
                    unsafe { (*a_op.offset(1 as isize)).p2 = mem_id + 1 };
                    unsafe { (*a_op.offset(2 as isize)).p1 = mem_id - 1 };
                    unsafe { (*a_op.offset(2 as isize)).p3 = i_rec };
                    unsafe { (*a_op.offset(3 as isize)).p2 = i_rec };
                    unsafe { (*a_op.offset(3 as isize)).p3 = mem_id + 1 };
                    unsafe { (*a_op.offset(3 as isize)).p5 = 8 as u16 };
                    unsafe { sqlite3_release_temp_reg(p_parse_1, i_rec) };
                    break '__c3;
                }
                p = unsafe { (*p).p_next };
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_autoincrement_end(p_parse: *mut Parse) -> () {
    if unsafe { (*p_parse).uses_ainc() } != 0 { auto_increment_end(p_parse); }
}
extern "C" fn xfer_compatible_index(p_dest_1: &Index, p_src_1: &Index)
    -> i32 {
    let mut i: i32 = 0;
    { let _ = 0; };
    { let _ = 0; };
    if (*p_dest_1).n_key_col as i32 != (*p_src_1).n_key_col as i32 ||
            (*p_dest_1).n_column as i32 != (*p_src_1).n_column as i32 {
        return 0;
    }
    if (*p_dest_1).on_error as i32 != (*p_src_1).on_error as i32 { return 0; }
    {
        i = 0;
        '__b4: loop {
            if !(i < (*p_src_1).n_key_col as i32) { break '__b4; }
            '__c4: loop {
                if unsafe { *(*p_src_1).ai_column.offset(i as isize) } as i32
                        !=
                        unsafe { *(*p_dest_1).ai_column.offset(i as isize) } as i32
                    {
                    return 0;
                }
                if unsafe { *(*p_src_1).ai_column.offset(i as isize) } as i32
                        == -2 {
                    { let _ = 0; };
                    if unsafe {
                                sqlite3_expr_compare(core::ptr::null(),
                                    unsafe {
                                            (*(unsafe { (*(*p_src_1).a_col_expr).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        } as *const Expr,
                                    unsafe {
                                            (*(unsafe { (*(*p_dest_1).a_col_expr).a.as_ptr() } as
                                                            *mut ExprListItem).offset(i as isize)).p_expr
                                        } as *const Expr, -1)
                            } != 0 {
                        return 0;
                    }
                }
                if unsafe { *(*p_src_1).a_sort_order.offset(i as isize) } as
                            i32 !=
                        unsafe { *(*p_dest_1).a_sort_order.offset(i as isize) } as
                            i32 {
                    return 0;
                }
                if unsafe {
                            sqlite3_stricmp(unsafe {
                                    *(*p_src_1).az_coll.offset(i as isize)
                                }, unsafe { *(*p_dest_1).az_coll.offset(i as isize) })
                        } != 0 {
                    return 0;
                }
                break '__c4;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if unsafe {
                sqlite3_expr_compare(core::ptr::null(),
                    (*p_src_1).p_part_idx_where as *const Expr,
                    (*p_dest_1).p_part_idx_where as *const Expr, -1)
            } != 0 {
        return 0;
    }
    return 1;
}
extern "C" fn xfer_check_rowid(p_walk_1: *mut Walker, p_expr_1: *mut Expr)
    -> i32 {
    if unsafe { (*p_expr_1).op } as i32 == 168 &&
            (unsafe { (*p_expr_1).i_column } as i32) < 0 {
        unsafe { (*p_walk_1).e_code = 1 as u16 };
        return 2;
    } else { return 0; }
}
extern "C" fn xfer_compatible_check(p_dest_1: &Table, p_src_1: &Table)
    -> i32 {
    if unsafe {
                sqlite3_expr_list_compare((*p_src_1).p_check as
                        *const ExprList, (*p_dest_1).p_check as *const ExprList, -1)
            } != 0 {
        return 0;
    }
    if ((*p_dest_1).i_p_key as i32) < 0 {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.x_expr_callback = Some(xfer_check_rowid);
        unsafe { sqlite3_walk_expr_list(&mut w, (*p_dest_1).p_check) };
        if w.e_code != 0 { return 0; }
    }
    return 1;
}
extern "C" fn auto_inc_begin(p_parse_1: *mut Parse, i_db_1: i32,
    p_tab_1: *mut Table) -> i32 {
    unsafe {
        let mut mem_id: i32 = 0;
        { let _ = 0; };
        if unsafe { (*p_tab_1).tab_flags } & 8 as u32 != 0 as u32 &&
                unsafe { (*unsafe { (*p_parse_1).db }).m_db_flags } & 4 as u32
                    == 0 as u32 {
            let p_toplevel: *mut Parse =
                if !(unsafe { (*p_parse_1).p_toplevel }).is_null() {
                    unsafe { (*p_parse_1).p_toplevel }
                } else { p_parse_1 };
            let mut p_info: *mut AutoincInfo = core::ptr::null_mut();
            let p_seq_tab: *const Table =
                unsafe {
                        (*unsafe {
                                        (*unsafe {
                                                    (*unsafe { (*p_parse_1).db }).a_db.offset(i_db_1 as isize)
                                                }).p_schema
                                    }).p_seq_tab
                    } as *const Table;
            if p_seq_tab == core::ptr::null_mut() ||
                            !(unsafe { (*p_seq_tab).tab_flags } & 128 as u32 ==
                                            0 as u32) as i32 != 0 ||
                        unsafe { (*p_seq_tab).e_tab_type } as i32 == 1 ||
                    unsafe { (*p_seq_tab).n_col } as i32 != 2 {
                {
                    let __p = unsafe { &mut (*p_parse_1).n_err };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                unsafe { (*p_parse_1).rc = 11 | 2 << 8 };
                return 0;
            }
            if unsafe { (*p_toplevel).uses_ainc() } as i32 == 0 {
                unsafe { (*p_toplevel).p_ainc = core::ptr::null_mut() };
            }
            p_info = unsafe { (*p_toplevel).p_ainc };
            while !(p_info).is_null() && unsafe { (*p_info).p_tab } != p_tab_1
                {
                p_info = unsafe { (*p_info).p_next };
            }
            if p_info == core::ptr::null_mut() {
                p_info =
                    unsafe {
                            sqlite3_db_malloc_raw_nn(unsafe { (*p_parse_1).db },
                                core::mem::size_of::<AutoincInfo>() as u64)
                        } as *mut AutoincInfo;
                unsafe {
                    sqlite3_parser_add_cleanup(p_toplevel,
                        Some(sqlite3_db_free), p_info as *mut ())
                };
                if unsafe { (*unsafe { (*p_parse_1).db }).malloc_failed } != 0
                    {
                    return 0;
                }
                unsafe { (*p_info).p_next = unsafe { (*p_toplevel).p_ainc } };
                unsafe { (*p_toplevel).p_ainc = p_info };
                unsafe { (*p_toplevel).set_uses_ainc(1 as Bft as u32) };
                unsafe { (*p_info).p_tab = p_tab_1 };
                unsafe { (*p_info).i_db = i_db_1 };
                {
                    let __p = unsafe { &mut (*p_toplevel).n_mem };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
                unsafe {
                    (*p_info).reg_ctr =
                        {
                            let __p = unsafe { &mut (*p_toplevel).n_mem };
                            *__p += 1;
                            *__p
                        }
                };
                unsafe { (*p_toplevel).n_mem += 2 };
            }
            mem_id = unsafe { (*p_info).reg_ctr };
        }
        return mem_id;
    }
}
extern "C" fn auto_inc_step(p_parse_1: &Parse, mem_id_1: i32,
    reg_rowid_1: i32) -> () {
    if mem_id_1 > 0 {
        unsafe {
            sqlite3_vdbe_add_op2((*p_parse_1).p_vdbe, 161, mem_id_1,
                reg_rowid_1)
        };
    }
}
extern "C" fn xfer_optimization(p_parse: *mut Parse, p_dest: *mut Table,
    p_select: &Select, mut on_error: i32, i_db_dest: i32) -> i32 {
    unsafe {
        unsafe {
            let db: *mut Sqlite3 = unsafe { (*p_parse).db };
            let mut p_e_list: *const ExprList = core::ptr::null();
            let mut p_src: *mut Table = core::ptr::null_mut();
            let mut p_src_idx: *mut Index = core::ptr::null_mut();
            let mut p_dest_idx: *mut Index = core::ptr::null_mut();
            let mut p_item: *mut SrcItem = core::ptr::null_mut();
            let mut i: i32 = 0;
            let mut i_db_src: i32 = 0;
            let mut i_src: i32 = 0;
            let mut i_dest: i32 = 0;
            let mut addr1: i32 = 0;
            let mut addr2: i32 = 0;
            let mut empty_dest_test: i32 = 0;
            let mut empty_src_test: i32 = 0;
            let mut v: *mut Vdbe = core::ptr::null_mut();
            let mut reg_autoinc: i32 = 0;
            let mut dest_has_unique_idx: i32 = 0;
            let mut reg_data: i32 = 0;
            let mut reg_rowid: i32 = 0;
            { let _ = 0; };
            if !(unsafe { (*p_parse).p_with }).is_null() ||
                    !((*p_select).p_with).is_null() {
                return 0;
            }
            if unsafe { (*p_dest).e_tab_type } as i32 == 1 { return 0; }
            if on_error == 11 {
                if unsafe { (*p_dest).i_p_key } as i32 >= 0 {
                    on_error = unsafe { (*p_dest).key_conf } as i32;
                }
                if on_error == 11 { on_error = 2; }
            }
            { let _ = 0; };
            if unsafe { (*(*p_select).p_src).n_src } != 1 { return 0; }
            if unsafe {
                        (*(unsafe { (*(*p_select).p_src).a.as_ptr() } as
                                            *mut SrcItem).offset(0 as isize)).fg.is_subquery()
                    } != 0 {
                return 0;
            }
            if !((*p_select).p_where).is_null() { return 0; }
            if !((*p_select).p_order_by).is_null() { return 0; }
            if !((*p_select).p_group_by).is_null() { return 0; }
            if !((*p_select).p_limit).is_null() { return 0; }
            if !((*p_select).p_prior).is_null() { return 0; }
            if (*p_select).sel_flags & 1 as u32 != 0 { return 0; }
            p_e_list = (*p_select).p_e_list;
            { let _ = 0; };
            if unsafe { (*p_e_list).n_expr } != 1 { return 0; }
            { let _ = 0; };
            if unsafe {
                            (*unsafe {
                                            (*(unsafe { (*p_e_list).a.as_ptr() } as
                                                            *mut ExprListItem).offset(0 as isize)).p_expr
                                        }).op
                        } as i32 != 180 {
                return 0;
            }
            p_item =
                unsafe { (*(*p_select).p_src).a.as_ptr() } as *mut SrcItem;
            p_src =
                unsafe {
                    sqlite3_locate_table_item(p_parse, 0 as u32, p_item)
                };
            if p_src == core::ptr::null_mut() { return 0; }
            if unsafe { (*p_src).tnum } == unsafe { (*p_dest).tnum } &&
                    unsafe { (*p_src).p_schema } ==
                        unsafe { (*p_dest).p_schema } {
                return 0;
            }
            if (unsafe { (*p_dest).tab_flags } & 128 as u32 == 0 as u32) as
                        i32 !=
                    (unsafe { (*p_src).tab_flags } & 128 as u32 == 0 as u32) as
                        i32 {
                return 0;
            }
            if !(unsafe { (*p_src).e_tab_type } as i32 == 0) as i32 != 0 {
                return 0;
            }
            if unsafe { (*p_dest).n_col } as i32 !=
                    unsafe { (*p_src).n_col } as i32 {
                return 0;
            }
            if unsafe { (*p_dest).i_p_key } as i32 !=
                    unsafe { (*p_src).i_p_key } as i32 {
                return 0;
            }
            if unsafe { (*p_dest).tab_flags } & 65536 as u32 != 0 as u32 {
                if unsafe { (*p_src).tab_flags } & 65536 as u32 == 0 as u32 {
                    return 0;
                }
                {
                    i = 0;
                    '__b6: loop {
                        if !(i < unsafe { (*p_dest).n_col } as i32) { break '__b6; }
                        '__c6: loop {
                            let e_dest_type: u32 =
                                unsafe {
                                        (*unsafe { (*p_dest).a_col.offset(i as isize) }).e_c_type()
                                    } as u32;
                            let e_src_type: u32 =
                                unsafe {
                                        (*unsafe { (*p_src).a_col.offset(i as isize) }).e_c_type()
                                    } as u32;
                            if e_dest_type == 1 as u32 { break '__c6; }
                            if e_dest_type == e_src_type { break '__c6; }
                            if e_dest_type == 3 as u32 && e_src_type == 4 as u32 {
                                break '__c6;
                            }
                            if e_dest_type == 4 as u32 && e_src_type == 3 as u32 {
                                break '__c6;
                            }
                            return 0;
                            break '__c6;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            {
                i = 0;
                '__b7: loop {
                    if !(i < unsafe { (*p_dest).n_col } as i32) { break '__b7; }
                    '__c7: loop {
                        let p_dest_col: *mut Column =
                            unsafe {
                                &mut *unsafe { (*p_dest).a_col.offset(i as isize) }
                            };
                        let p_src_col: *mut Column =
                            unsafe {
                                &mut *unsafe { (*p_src).a_col.offset(i as isize) }
                            };
                        if unsafe { (*p_dest_col).col_flags } as i32 & 96 !=
                                unsafe { (*p_src_col).col_flags } as i32 & 96 {
                            return 0;
                        }
                        if unsafe { (*p_dest_col).col_flags } as i32 & 96 != 0 {
                            if unsafe {
                                        sqlite3_expr_compare(core::ptr::null(),
                                            unsafe { sqlite3_column_expr(p_src, p_src_col) } as
                                                *const Expr,
                                            unsafe { sqlite3_column_expr(p_dest, p_dest_col) } as
                                                *const Expr, -1)
                                    } != 0 {
                                return 0;
                            }
                        }
                        if unsafe { (*p_dest_col).affinity } as i32 !=
                                unsafe { (*p_src_col).affinity } as i32 {
                            return 0;
                        }
                        if unsafe {
                                    sqlite3_stricmp(unsafe { sqlite3_column_coll(p_dest_col) },
                                        unsafe { sqlite3_column_coll(p_src_col) })
                                } != 0 {
                            return 0;
                        }
                        if unsafe { (*p_dest_col).not_null() } != 0 &&
                                (unsafe { (*p_src_col).not_null() } == 0) as i32 != 0 {
                            return 0;
                        }
                        if unsafe { (*p_dest_col).col_flags } as i32 & 96 == 0 &&
                                i > 0 {
                            let p_dest_expr: *const Expr =
                                unsafe { sqlite3_column_expr(p_dest, p_dest_col) } as
                                    *const Expr;
                            let p_src_expr: *const Expr =
                                unsafe { sqlite3_column_expr(p_src, p_src_col) } as
                                    *const Expr;
                            { let _ = 0; };
                            { let _ = 0; };
                            { let _ = 0; };
                            { let _ = 0; };
                            if (p_dest_expr == core::ptr::null_mut()) as i32 !=
                                        (p_src_expr == core::ptr::null_mut()) as i32 ||
                                    p_dest_expr != core::ptr::null_mut() &&
                                        unsafe {
                                                strcmp(unsafe { (*p_dest_expr).u.z_token } as *const i8,
                                                    unsafe { (*p_src_expr).u.z_token } as *const i8)
                                            } != 0 {
                                return 0;
                            }
                        }
                        break '__c7;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            {
                p_dest_idx = unsafe { (*p_dest).p_index };
                '__b8: loop {
                    if !(!(p_dest_idx).is_null()) { break '__b8; }
                    '__c8: loop {
                        if unsafe { (*p_dest_idx).on_error } as i32 != 0 {
                            dest_has_unique_idx = 1;
                        }
                        {
                            p_src_idx = unsafe { (*p_src).p_index };
                            '__b9: loop {
                                if !(!(p_src_idx).is_null()) { break '__b9; }
                                '__c9: loop {
                                    if xfer_compatible_index(unsafe { &*p_dest_idx },
                                                unsafe { &*p_src_idx }) != 0 {
                                        break '__b9;
                                    }
                                    break '__c9;
                                }
                                p_src_idx = unsafe { (*p_src_idx).p_next };
                            }
                        }
                        if p_src_idx == core::ptr::null_mut() { return 0; }
                        if unsafe { (*p_src_idx).tnum } ==
                                        unsafe { (*p_dest_idx).tnum } &&
                                    unsafe { (*p_src).p_schema } ==
                                        unsafe { (*p_dest).p_schema } &&
                                unsafe { sqlite3_fault_sim(411) } == 0 {
                            return 0;
                        }
                        break '__c8;
                    }
                    p_dest_idx = unsafe { (*p_dest_idx).p_next };
                }
            }
            if !(unsafe { (*p_dest).p_check }).is_null() &&
                        unsafe { (*db).m_db_flags } & 4 as u32 == 0 as u32 &&
                    (xfer_compatible_check(unsafe { &*p_dest },
                                    unsafe { &*p_src }) == 0) as i32 != 0 {
                return 0;
            }
            { let _ = 0; };
            if unsafe { (*db).flags } & 16384 as u64 != 0 as u64 &&
                    unsafe { (*p_dest).u.tab.p_f_key } != core::ptr::null_mut()
                {
                return 0;
            }
            if unsafe { (*db).flags } & (1 as u64) << 32 != 0 as u64 {
                return 0;
            }
            if unsafe { (*db).x_auth.is_some() } {
                let i_db: i32 =
                    unsafe {
                        sqlite3_schema_to_index(db, unsafe { (*p_src).p_schema })
                    };
                if unsafe {
                            sqlite3_auth_check(p_parse, 21, core::ptr::null(),
                                core::ptr::null(), core::ptr::null())
                        } != 0 {
                    return 0;
                }
                {
                    i = 0;
                    '__b10: loop {
                        if !(i < unsafe { (*p_src).n_col } as i32) { break '__b10; }
                        '__c10: loop {
                            let p_src_col_1: *const Column =
                                unsafe {
                                        &raw mut *unsafe { (*p_src).a_col.offset(i as isize) }
                                    } as *const Column;
                            if unsafe {
                                        sqlite3_auth_read_col(p_parse,
                                            unsafe { (*p_src).z_name } as *const i8,
                                            unsafe { (*p_src_col_1).z_cn_name } as *const i8, i_db)
                                    } != 0 {
                                return 0;
                            }
                            break '__c10;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
            }
            i_db_src =
                unsafe {
                    sqlite3_schema_to_index(db, unsafe { (*p_src).p_schema })
                };
            v = unsafe { sqlite3_get_vdbe(p_parse) };
            unsafe { sqlite3_code_verify_schema(p_parse, i_db_src) };
            i_src =
                {
                    let __p = unsafe { &mut (*p_parse).n_tab };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            i_dest =
                {
                    let __p = unsafe { &mut (*p_parse).n_tab };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            reg_autoinc = auto_inc_begin(p_parse, i_db_dest, p_dest);
            reg_data = unsafe { sqlite3_get_temp_reg(p_parse) };
            unsafe { sqlite3_vdbe_add_op2(v, 77, 0, reg_data) };
            reg_rowid = unsafe { sqlite3_get_temp_reg(p_parse) };
            sqlite3_open_table(p_parse, i_dest, i_db_dest, p_dest, 116);
            { let _ = 0; };
            if unsafe { (*db).m_db_flags } & 4 as u32 == 0 as u32 &&
                    ((unsafe { (*p_dest).i_p_key } as i32) < 0 &&
                                unsafe { (*p_dest).p_index } != core::ptr::null_mut() ||
                            dest_has_unique_idx != 0 || on_error != 2 && on_error != 1)
                {
                addr1 = unsafe { sqlite3_vdbe_add_op2(v, 36, i_dest, 0) };
                empty_dest_test = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                unsafe { sqlite3_vdbe_jump_here(v, addr1) };
            }
            if unsafe { (*p_src).tab_flags } & 128 as u32 == 0 as u32 {
                let mut ins_flags: u8 = 0 as u8;
                sqlite3_open_table(p_parse, i_src, i_db_src, p_src, 114);
                empty_src_test =
                    unsafe { sqlite3_vdbe_add_op2(v, 36, i_src, 0) };
                if unsafe { (*p_dest).i_p_key } as i32 >= 0 {
                    addr1 =
                        unsafe { sqlite3_vdbe_add_op2(v, 137, i_src, reg_rowid) };
                    if unsafe { (*db).m_db_flags } & 4 as u32 == 0 as u32 {
                        addr2 =
                            unsafe {
                                sqlite3_vdbe_add_op3(v, 31, i_dest, 0, reg_rowid)
                            };
                        unsafe {
                            sqlite3_rowid_constraint(p_parse, on_error, p_dest)
                        };
                        unsafe { sqlite3_vdbe_jump_here(v, addr2) };
                    }
                    auto_inc_step(unsafe { &*p_parse }, reg_autoinc, reg_rowid);
                } else if unsafe { (*p_dest).p_index } ==
                            core::ptr::null_mut() &&
                        (unsafe { (*db).m_db_flags } & 8 as u32 == 0) as i32 != 0 {
                    addr1 =
                        unsafe { sqlite3_vdbe_add_op2(v, 129, i_dest, reg_rowid) };
                } else {
                    addr1 =
                        unsafe { sqlite3_vdbe_add_op2(v, 137, i_src, reg_rowid) };
                    { let _ = 0; };
                }
                if unsafe { (*db).m_db_flags } & 4 as u32 != 0 {
                    unsafe { sqlite3_vdbe_add_op1(v, 139, i_dest) };
                    ins_flags = (8 | 16 | 128) as u8;
                } else { ins_flags = (1 | 32 | 8 | 128) as u8; }
                {
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 131, i_dest, i_src, reg_rowid)
                    };
                }
                unsafe {
                    sqlite3_vdbe_add_op3(v, 130, i_dest, reg_data, reg_rowid)
                };
                if unsafe { (*db).m_db_flags } & 4 as u32 == 0 as u32 {
                    unsafe {
                        sqlite3_vdbe_change_p4(v, -1,
                            p_dest as *mut i8 as *const i8, -5)
                    };
                }
                unsafe { sqlite3_vdbe_change_p5(v, ins_flags as u16) };
                unsafe { sqlite3_vdbe_add_op2(v, 40, i_src, addr1) };
                unsafe { sqlite3_vdbe_add_op2(v, 124, i_src, 0) };
                unsafe { sqlite3_vdbe_add_op2(v, 124, i_dest, 0) };
            } else {
                unsafe {
                    sqlite3_table_lock(p_parse, i_db_dest,
                        unsafe { (*p_dest).tnum }, 1 as u8,
                        unsafe { (*p_dest).z_name } as *const i8)
                };
                unsafe {
                    sqlite3_table_lock(p_parse, i_db_src,
                        unsafe { (*p_src).tnum }, 0 as u8,
                        unsafe { (*p_src).z_name } as *const i8)
                };
            }
            {
                p_dest_idx = unsafe { (*p_dest).p_index };
                '__b11: loop {
                    if !(!(p_dest_idx).is_null()) { break '__b11; }
                    '__c11: loop {
                        let mut idx_ins_flags: u8 = 0 as u8;
                        {
                            p_src_idx = unsafe { (*p_src).p_index };
                            '__b12: loop {
                                if !(!(p_src_idx).is_null()) { break '__b12; }
                                '__c12: loop {
                                    if xfer_compatible_index(unsafe { &*p_dest_idx },
                                                unsafe { &*p_src_idx }) != 0 {
                                        break '__b12;
                                    }
                                    break '__c12;
                                }
                                p_src_idx = unsafe { (*p_src_idx).p_next };
                            }
                        }
                        { let _ = 0; };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 114, i_src,
                                unsafe { (*p_src_idx).tnum } as i32, i_db_src)
                        };
                        unsafe { sqlite3_vdbe_set_p4_key_info(p_parse, p_src_idx) };
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 116, i_dest,
                                unsafe { (*p_dest_idx).tnum } as i32, i_db_dest)
                        };
                        unsafe {
                            sqlite3_vdbe_set_p4_key_info(p_parse, p_dest_idx)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
                        addr1 = unsafe { sqlite3_vdbe_add_op2(v, 36, i_src, 0) };
                        if unsafe { (*db).m_db_flags } & 4 as u32 != 0 {
                            {
                                i = 0;
                                '__b13: loop {
                                    if !(i < unsafe { (*p_src_idx).n_column } as i32) {
                                        break '__b13;
                                    }
                                    '__c13: loop {
                                        let z_coll: *const i8 =
                                            unsafe {
                                                *unsafe { (*p_src_idx).az_coll.offset(i as isize) }
                                            };
                                        if unsafe {
                                                    sqlite3_stricmp(sqlite3_str_binary.as_ptr() as *const i8,
                                                        z_coll)
                                                } != 0 {
                                            break '__b13;
                                        }
                                        break '__c13;
                                    }
                                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                }
                            }
                            if i == unsafe { (*p_src_idx).n_column } as i32 {
                                idx_ins_flags = (16 | 128) as u8;
                                unsafe { sqlite3_vdbe_add_op1(v, 139, i_dest) };
                                unsafe { sqlite3_vdbe_add_op2(v, 131, i_dest, i_src) };
                            }
                        } else if !(unsafe { (*p_src).tab_flags } & 128 as u32 ==
                                                0 as u32) as i32 != 0 &&
                                unsafe { (*p_dest_idx).idx_type() } as i32 == 2 {
                            idx_ins_flags |= 1 as u8;
                        }
                        if idx_ins_flags as i32 != 16 | 128 {
                            unsafe { sqlite3_vdbe_add_op3(v, 136, i_src, reg_data, 1) };
                            if unsafe { (*db).m_db_flags } & 4 as u32 == 0 as u32 &&
                                        !(unsafe { (*p_dest).tab_flags } & 128 as u32 == 0 as u32)
                                                as i32 != 0 &&
                                    unsafe { (*p_dest_idx).idx_type() } as i32 == 2 {}
                        }
                        unsafe { sqlite3_vdbe_add_op2(v, 140, i_dest, reg_data) };
                        unsafe {
                            sqlite3_vdbe_change_p5(v, (idx_ins_flags as i32 | 8) as u16)
                        };
                        unsafe { sqlite3_vdbe_add_op2(v, 40, i_src, addr1 + 1) };
                        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
                        unsafe { sqlite3_vdbe_add_op2(v, 124, i_src, 0) };
                        unsafe { sqlite3_vdbe_add_op2(v, 124, i_dest, 0) };
                        break '__c11;
                    }
                    p_dest_idx = unsafe { (*p_dest_idx).p_next };
                }
            }
            if empty_src_test != 0 {
                unsafe { sqlite3_vdbe_jump_here(v, empty_src_test) };
            }
            unsafe { sqlite3_release_temp_reg(p_parse, reg_rowid) };
            unsafe { sqlite3_release_temp_reg(p_parse, reg_data) };
            if empty_dest_test != 0 {
                sqlite3_autoincrement_end(p_parse);
                unsafe { sqlite3_vdbe_add_op2(v, 72, 0, 0) };
                unsafe { sqlite3_vdbe_jump_here(v, empty_dest_test) };
                unsafe { sqlite3_vdbe_add_op2(v, 124, i_dest, 0) };
                return 0;
            } else { return 1; }
        }
    }
}
extern "C" fn reads_table(p: *mut Parse, i_db_1: i32, p_tab_1: *mut Table)
    -> i32 {
    unsafe {
        let v: *mut Vdbe = unsafe { sqlite3_get_vdbe(p) };
        let mut i: i32 = 0;
        let i_end: i32 = unsafe { sqlite3_vdbe_current_addr(v) };
        let p_v_tab: *mut VTable =
            if unsafe { (*p_tab_1).e_tab_type } as i32 == 1 {
                unsafe { sqlite3_get_v_table(unsafe { (*p).db }, p_tab_1) }
            } else { core::ptr::null_mut() };
        {
            i = 1;
            '__b14: loop {
                if !(i < i_end) { break '__b14; }
                '__c14: loop {
                    let p_op: *const VdbeOp =
                        unsafe { sqlite3_vdbe_get_op(v, i) } as *const VdbeOp;
                    { let _ = 0; };
                    if unsafe { (*p_op).opcode } as i32 == 114 &&
                            unsafe { (*p_op).p3 } == i_db_1 {
                        let mut p_index: *const Index = core::ptr::null();
                        let tnum: Pgno = unsafe { (*p_op).p2 } as Pgno;
                        if tnum == unsafe { (*p_tab_1).tnum } { return 1; }
                        {
                            p_index = unsafe { (*p_tab_1).p_index };
                            '__b15: loop {
                                if !(!(p_index).is_null()) { break '__b15; }
                                '__c15: loop {
                                    if tnum == unsafe { (*p_index).tnum } { return 1; }
                                    break '__c15;
                                }
                                p_index = unsafe { (*p_index).p_next };
                            }
                        }
                    }
                    if unsafe { (*p_op).opcode } as i32 == 175 &&
                            unsafe { (*p_op).p4.p_vtab } == p_v_tab {
                        { let _ = 0; };
                        { let _ = 0; };
                        return 1;
                    }
                    break '__c14;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_open_table_and_indices(p_parse: *mut Parse,
    p_tab: *mut Table, op: i32, mut p5: u8, mut i_base: i32,
    a_to_open: *mut u8, pi_data_cur: &mut i32, pi_idx_cur: &mut i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut i_db: i32 = 0;
        let mut i_data_cur: i32 = 0;
        let mut p_idx: *mut Index = core::ptr::null_mut();
        let mut v: *mut Vdbe = core::ptr::null_mut();
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        { let _ = 0; };
        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
            *pi_data_cur = { *pi_idx_cur = -999; *pi_idx_cur };
            return 0;
        }
        i_db =
            unsafe {
                sqlite3_schema_to_index(unsafe { (*p_parse).db },
                    unsafe { (*p_tab).p_schema })
            };
        v = unsafe { (*p_parse).p_vdbe };
        { let _ = 0; };
        if i_base < 0 { i_base = unsafe { (*p_parse).n_tab }; }
        i_data_cur =
            { let __p = &mut i_base; let __t = *__p; *__p += 1; __t };
        *pi_data_cur = i_data_cur;
        if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 &&
                (a_to_open == core::ptr::null_mut() ||
                    unsafe { *a_to_open.offset(0 as isize) } != 0) {
            sqlite3_open_table(p_parse, i_data_cur, i_db, p_tab, op);
        } else if unsafe { (*unsafe { (*p_parse).db }).no_shared_cache } as
                    i32 == 0 {
            unsafe {
                sqlite3_table_lock(p_parse, i_db, unsafe { (*p_tab).tnum },
                    (op == 116) as u8, unsafe { (*p_tab).z_name } as *const i8)
            };
        }
        *pi_idx_cur = i_base;
        {
            { i = 0; p_idx = unsafe { (*p_tab).p_index } };
            '__b16: loop {
                if !(!(p_idx).is_null()) { break '__b16; }
                '__c16: loop {
                    let i_idx_cur: i32 =
                        { let __p = &mut i_base; let __t = *__p; *__p += 1; __t };
                    { let _ = 0; };
                    if unsafe { (*p_idx).idx_type() } as i32 == 2 &&
                            !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32) as
                                    i32 != 0 {
                        *pi_data_cur = i_idx_cur;
                        p5 = 0 as u8;
                    }
                    if a_to_open == core::ptr::null_mut() ||
                            unsafe { *a_to_open.offset((i + 1) as isize) } != 0 {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, op, i_idx_cur,
                                unsafe { (*p_idx).tnum } as i32, i_db)
                        };
                        unsafe { sqlite3_vdbe_set_p4_key_info(p_parse, p_idx) };
                        unsafe { sqlite3_vdbe_change_p5(v, p5 as u16) };
                    }
                    break '__c16;
                }
                {
                    p_idx = unsafe { (*p_idx).p_next };
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                };
            }
        }
        if i_base > unsafe { (*p_parse).n_tab } {
            unsafe { (*p_parse).n_tab = i_base };
        }
        return i;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_table_affinity_str(db: *mut Sqlite3, p_tab: &Table)
    -> *mut i8 {
    let mut z_col_aff: *mut i8 = core::ptr::null_mut();
    z_col_aff =
        unsafe {
                sqlite3_db_malloc_raw(db, ((*p_tab).n_col as i32 + 1) as u64)
            } as *mut i8;
    if !(z_col_aff).is_null() {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        {
            i = { j = 0; j };
            '__b17: loop {
                if !(i < (*p_tab).n_col as i32) { break '__b17; }
                '__c17: loop {
                    if unsafe { (*(*p_tab).a_col.offset(i as isize)).col_flags }
                                    as i32 & 32 == 0 {
                        unsafe {
                            *z_col_aff.offset({
                                                let __p = &mut j;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as isize) =
                                unsafe { (*(*p_tab).a_col.offset(i as isize)).affinity }
                        };
                    }
                    break '__c17;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        '__b18: loop {
            '__c18: loop {
                unsafe {
                    *z_col_aff.offset({
                                        let __p = &mut j;
                                        let __t = *__p;
                                        *__p -= 1;
                                        __t
                                    } as isize) = 0 as i8
                };
                break '__c18;
            }
            if !(j >= 0 &&
                            unsafe { *z_col_aff.offset(j as isize) } as i32 <= 65) {
                break '__b18;
            }
        }
    }
    return z_col_aff;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_table_affinity(v: *mut Vdbe, p_tab: *mut Table,
    i_reg: i32) -> () {
    let mut i: i32 = 0;
    let mut z_col_aff: *mut i8 = core::ptr::null_mut();
    if unsafe { (*p_tab).tab_flags } & 65536 as u32 != 0 {
        if i_reg == 0 {
            let mut p_prev: *mut VdbeOp = core::ptr::null_mut();
            let mut p3: i32 = 0;
            unsafe { sqlite3_vdbe_append_p4(v, p_tab as *mut (), -5) };
            p_prev = unsafe { sqlite3_vdbe_get_last_op(v) };
            { let _ = 0; };
            { let _ = 0; };
            unsafe { (*p_prev).opcode = 97 as u8 };
            p3 = unsafe { (*p_prev).p3 };
            unsafe { (*p_prev).p3 = 0 };
            unsafe {
                sqlite3_vdbe_add_op3(v, 99, unsafe { (*p_prev).p1 },
                    unsafe { (*p_prev).p2 }, p3)
            };
        } else {
            unsafe {
                sqlite3_vdbe_add_op2(v, 97, i_reg,
                    unsafe { (*p_tab).n_nv_col } as i32)
            };
            unsafe { sqlite3_vdbe_append_p4(v, p_tab as *mut (), -5) };
        }
        return;
    }
    z_col_aff = unsafe { (*p_tab).z_col_aff };
    if z_col_aff == core::ptr::null_mut() {
        z_col_aff =
            sqlite3_table_affinity_str(core::ptr::null_mut(),
                unsafe { &*p_tab });
        if (z_col_aff).is_null() as i32 != 0 {
            unsafe { sqlite3_oom_fault(unsafe { sqlite3_vdbe_db(v) }) };
            return;
        }
        unsafe { (*p_tab).z_col_aff = z_col_aff };
    }
    { let _ = 0; };
    i =
        (unsafe { strlen(z_col_aff as *const i8) } & 1073741823 as u64) as
            i32;
    if i != 0 {
        if i_reg != 0 {
            unsafe {
                sqlite3_vdbe_add_op4(v, 98, i_reg, i, 0,
                    z_col_aff as *const i8, i)
            };
        } else {
            { let _ = 0; };
            unsafe {
                sqlite3_vdbe_change_p4(v, -1, z_col_aff as *const i8, i)
            };
        }
    }
}
extern "C" fn expr_column_flag_union(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 168 &&
                unsafe { (*p_expr_1).i_column } as i32 >= 0 {
            { let _ = 0; };
            unsafe {
                (*p_walker_1).e_code |=
                    unsafe {
                                (*unsafe {
                                            (*unsafe {
                                                                (*p_walker_1).u.p_tab
                                                            }).a_col.offset(unsafe { (*p_expr_1).i_column } as isize)
                                        }).col_flags
                            } as i32 as u16
            };
        }
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_compute_generated_columns(p_parse: *mut Parse,
    i_reg_store: i32, p_tab: *mut Table) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut w: Walker = unsafe { core::mem::zeroed() };
        let mut p_redo: *const Column = core::ptr::null();
        let mut e_progress: i32 = 0;
        let mut p_op: *mut VdbeOp = core::ptr::null_mut();
        { let _ = 0; };
        sqlite3_table_affinity(unsafe { (*p_parse).p_vdbe }, p_tab,
            i_reg_store);
        if unsafe { (*p_tab).tab_flags } & 64 as u32 != 0 as u32 {
            p_op =
                unsafe {
                    sqlite3_vdbe_get_last_op(unsafe { (*p_parse).p_vdbe })
                };
            if unsafe { (*p_op).opcode } as i32 == 98 {
                let mut ii: i32 = 0;
                let mut jj: i32 = 0;
                let z_p4: *mut i8 = unsafe { (*p_op).p4.z };
                { let _ = 0; };
                { let _ = 0; };
                {
                    ii = { jj = 0; jj };
                    '__b19: loop {
                        if !(unsafe { *z_p4.offset(jj as isize) } != 0) {
                            break '__b19;
                        }
                        '__c19: loop {
                            if unsafe {
                                                (*unsafe { (*p_tab).a_col.offset(ii as isize) }).col_flags
                                            } as i32 & 32 != 0 {
                                break '__c19;
                            }
                            if unsafe {
                                                (*unsafe { (*p_tab).a_col.offset(ii as isize) }).col_flags
                                            } as i32 & 64 != 0 {
                                unsafe { *z_p4.offset(jj as isize) = 64 as i8 };
                            }
                            { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                            break '__c19;
                        }
                        { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                    }
                }
            } else if unsafe { (*p_op).opcode } as i32 == 97 {
                unsafe { (*p_op).p3 = 1 };
            }
        }
        {
            i = 0;
            '__b20: loop {
                if !(i < unsafe { (*p_tab).n_col } as i32) { break '__b20; }
                '__c20: loop {
                    if unsafe {
                                        (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                    } as i32 & 96 != 0 {
                        unsafe {
                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags |=
                                128 as u16
                        };
                    }
                    break '__c20;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        w.u.p_tab = p_tab;
        w.x_expr_callback = Some(expr_column_flag_union);
        w.x_select_callback = None;
        w.x_select_callback2 = None;
        unsafe { (*p_parse).i_self_tab = -i_reg_store };
        '__b21: loop {
            '__c21: loop {
                e_progress = 0;
                p_redo = core::ptr::null_mut();
                {
                    i = 0;
                    '__b22: loop {
                        if !(i < unsafe { (*p_tab).n_col } as i32) { break '__b22; }
                        '__c22: loop {
                            let p_col: *mut Column =
                                unsafe { unsafe { (*p_tab).a_col.offset(i as isize) } };
                            if unsafe { (*p_col).col_flags } as i32 & 128 != 0 {
                                let mut x: i32 = 0;
                                unsafe { (*p_col).col_flags |= 256 as u16 };
                                w.e_code = 0 as u16;
                                unsafe {
                                    sqlite3_walk_expr(&mut w,
                                        unsafe { sqlite3_column_expr(p_tab, p_col) })
                                };
                                unsafe { (*p_col).col_flags &= !256 as u16 };
                                if w.e_code as i32 & 128 != 0 {
                                    p_redo = p_col;
                                    break '__c22;
                                }
                                e_progress = 1;
                                { let _ = 0; };
                                x =
                                    unsafe { sqlite3_table_column_to_storage(p_tab, i as i16) }
                                            as i32 + i_reg_store;
                                unsafe {
                                    sqlite3_expr_code_generated_column(p_parse, p_tab, p_col, x)
                                };
                                unsafe { (*p_col).col_flags &= !128 as u16 };
                            }
                            break '__c22;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                break '__c21;
            }
            if !(!(p_redo).is_null() && e_progress != 0) { break '__b21; }
        }
        if !(p_redo).is_null() {
            unsafe {
                sqlite3_error_msg(p_parse,
                    c"generated column loop on \"%s\"".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_redo).z_cn_name })
            };
        }
        unsafe { (*p_parse).i_self_tab = 0 };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IndexIterator {
    e_type: i32,
    i: i32,
    u: IndexIteratorU0,
}
#[repr(C)]
#[derive(Copy, Clone)]
union IndexIteratorU0 {
    lx: IndexIteratorU0S0,
    ax: IndexIteratorU0S1,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IndexIteratorU0S0 {
    p_idx: *mut Index,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IndexIteratorU0S1 {
    n_idx: i32,
    a_idx: *mut IndexListTerm,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct IndexListTerm {
    p: *mut Index,
    ix: i32,
}
extern "C" fn check_constraint_expr_node(p_walker_1: *mut Walker,
    p_expr_1: *mut Expr) -> i32 {
    unsafe {
        if unsafe { (*p_expr_1).op } as i32 == 168 {
            { let _ = 0; };
            if unsafe { (*p_expr_1).i_column } as i32 >= 0 {
                if unsafe {
                            *unsafe {
                                    (*p_walker_1).u.ai_col.offset(unsafe {
                                                (*p_expr_1).i_column
                                            } as isize)
                                }
                        } >= 0 {
                    unsafe { (*p_walker_1).e_code |= 1 as u16 };
                }
            } else { unsafe { (*p_walker_1).e_code |= 2 as u16 }; }
        }
        return 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_expr_references_updated_column(p_expr: *mut Expr,
    ai_chng: *mut i32, chng_rowid: i32) -> i32 {
    unsafe {
        let mut w: Walker = unsafe { core::mem::zeroed() };
        unsafe {
            memset(&raw mut w as *mut (), 0,
                core::mem::size_of::<Walker>() as u64)
        };
        w.e_code = 0 as u16;
        w.x_expr_callback = Some(check_constraint_expr_node);
        w.u.ai_col = ai_chng;
        unsafe { sqlite3_walk_expr(&mut w, p_expr) };
        if (chng_rowid == 0) as i32 != 0 { w.e_code &= !2 as u16; }
        return (w.e_code as i32 != 0) as i32;
    }
}
extern "C" fn index_iterator_first(p_iter_1: &IndexIterator, p_ix_1: &mut i32)
    -> *mut Index {
    unsafe {
        { let _ = 0; };
        if (*p_iter_1).e_type != 0 {
            *p_ix_1 =
                unsafe { (*(*p_iter_1).u.ax.a_idx.offset(0 as isize)).ix };
            return unsafe { (*(*p_iter_1).u.ax.a_idx.offset(0 as isize)).p };
        } else { *p_ix_1 = 0; return (*p_iter_1).u.lx.p_idx; }
    }
}
extern "C" fn index_iterator_next(p_iter_1: &mut IndexIterator,
    p_ix_1: &mut i32) -> *mut Index {
    unsafe {
        if (*p_iter_1).e_type != 0 {
            let i: i32 = { let __p = &mut (*p_iter_1).i; *__p += 1; *__p };
            if i >= (*p_iter_1).u.ax.n_idx {
                *p_ix_1 = i;
                return core::ptr::null_mut();
            }
            *p_ix_1 =
                unsafe { (*(*p_iter_1).u.ax.a_idx.offset(i as isize)).ix };
            return unsafe { (*(*p_iter_1).u.ax.a_idx.offset(i as isize)).p };
        } else {
            { let __p = &mut *p_ix_1; *__p += 1; *__p };
            (*p_iter_1).u.lx.p_idx =
                unsafe { (*(*p_iter_1).u.lx.p_idx).p_next };
            return (*p_iter_1).u.lx.p_idx;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_generate_constraint_checks(p_parse: *mut Parse,
    p_tab: *mut Table, a_reg_idx: *mut i32, i_data_cur: i32, i_idx_cur: i32,
    reg_new_data: i32, reg_old_data: i32, pk_chng: u8, mut override_error: u8,
    ignore_dest: i32, pb_may_replace: &mut i32, ai_chng: *mut i32,
    mut p_upsert: *mut Upsert) -> () {
    unsafe {
        unsafe {
            let mut v: *mut Vdbe = core::ptr::null_mut();
            let mut p_idx: *mut Index = core::ptr::null_mut();
            let mut p_pk: *mut Index = core::ptr::null_mut();
            let mut db: *mut Sqlite3 = core::ptr::null_mut();
            let mut i: i32 = 0;
            let mut ix: i32 = 0;
            let mut n_col: i32 = 0;
            let mut on_error: i32 = 0;
            let mut seen_replace: i32 = 0;
            let mut n_pk_field: i32 = 0;
            let mut p_upsert_clause: *mut Upsert = core::ptr::null_mut();
            let mut is_update: u8 = 0 as u8;
            let mut b_affinity_done: u8 = 0 as u8;
            let mut upsert_ipk_return: i32 = 0;
            let mut upsert_ipk_delay: i32 = 0;
            let mut ipk_top: i32 = 0;
            let mut ipk_bottom: i32 = 0;
            let mut reg_trig_cnt: i32 = 0;
            let mut addr_recheck: i32 = 0;
            let mut lbl_recheck_ok: i32 = 0;
            let mut p_trigger: *mut Trigger = core::ptr::null_mut();
            let mut n_replace_trig: i32 = 0;
            let mut s_idx_iter: IndexIterator =
                unsafe { core::mem::zeroed() };
            is_update = (reg_old_data != 0) as u8;
            db = unsafe { (*p_parse).db };
            v = unsafe { (*p_parse).p_vdbe };
            { let _ = 0; };
            { let _ = 0; };
            n_col = unsafe { (*p_tab).n_col } as i32;
            if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                p_pk = core::ptr::null_mut();
                n_pk_field = 1;
            } else {
                p_pk = unsafe { sqlite3_primary_key_index(p_tab) };
                n_pk_field = unsafe { (*p_pk).n_key_col } as i32;
            }
            if unsafe { (*p_tab).tab_flags } & 2048 as u32 != 0 {
                let mut b2nd_pass: i32 = 0;
                let mut n_seen_replace: i32 = 0;
                let mut n_generated: i32 = 0;
                loop {
                    {
                        i = 0;
                        '__b24: loop {
                            if !(i < n_col) { break '__b24; }
                            '__c24: loop {
                                let mut i_reg: i32 = 0;
                                let p_col: *mut Column =
                                    unsafe {
                                        &mut *unsafe { (*p_tab).a_col.offset(i as isize) }
                                    };
                                let mut is_generated: i32 = 0;
                                on_error = unsafe { (*p_col).not_null() } as i32;
                                if on_error == 0 { break '__c24; }
                                if i == unsafe { (*p_tab).i_p_key } as i32 { break '__c24; }
                                is_generated = unsafe { (*p_col).col_flags } as i32 & 96;
                                if is_generated != 0 && (b2nd_pass == 0) as i32 != 0 {
                                    {
                                        let __p = &mut n_generated;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                    break '__c24;
                                }
                                if !(ai_chng).is_null() &&
                                            unsafe { *ai_chng.offset(i as isize) } < 0 &&
                                        (is_generated == 0) as i32 != 0 {
                                    break '__c24;
                                }
                                if override_error as i32 != 11 {
                                    on_error = override_error as i32;
                                } else if on_error == 11 { on_error = 2; }
                                if on_error == 5 {
                                    if b2nd_pass != 0 || unsafe { (*p_col).i_dflt } as i32 == 0
                                        {
                                        on_error = 2;
                                    } else { { let _ = 0; }; }
                                } else if b2nd_pass != 0 && (is_generated == 0) as i32 != 0
                                    {
                                    break '__c24;
                                }
                                { let _ = 0; };
                                i_reg =
                                    unsafe { sqlite3_table_column_to_storage(p_tab, i as i16) }
                                                as i32 + reg_new_data + 1;
                                '__s25:
                                    {
                                    match on_error {
                                        5 => {
                                            {
                                                let addr1: i32 =
                                                    unsafe { sqlite3_vdbe_add_op1(v, 52, i_reg) };
                                                { let _ = 0; };
                                                {
                                                    let __p = &mut n_seen_replace;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                };
                                                unsafe {
                                                    sqlite3_expr_code_copy(p_parse,
                                                        unsafe { sqlite3_column_expr(p_tab, p_col) }, i_reg)
                                                };
                                                unsafe { sqlite3_vdbe_jump_here(v, addr1) };
                                                break '__s25;
                                            }
                                            unsafe { sqlite3_may_abort(p_parse) };
                                            {
                                                let z_msg: *mut i8 =
                                                    unsafe {
                                                        sqlite3_m_printf(db,
                                                            c"%s.%s".as_ptr() as *mut i8 as *const i8,
                                                            unsafe { (*p_tab).z_name }, unsafe { (*p_col).z_cn_name })
                                                    };
                                                unsafe {
                                                    sqlite3_vdbe_add_op3(v, 71, 19 | 5 << 8, on_error, i_reg)
                                                };
                                                unsafe { sqlite3_vdbe_append_p4(v, z_msg as *mut (), -7) };
                                                unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
                                                break '__s25;
                                            }
                                            {
                                                { let _ = 0; };
                                                unsafe { sqlite3_vdbe_add_op2(v, 51, i_reg, ignore_dest) };
                                                break '__s25;
                                            }
                                        }
                                        2 => {
                                            unsafe { sqlite3_may_abort(p_parse) };
                                            {
                                                let z_msg: *mut i8 =
                                                    unsafe {
                                                        sqlite3_m_printf(db,
                                                            c"%s.%s".as_ptr() as *mut i8 as *const i8,
                                                            unsafe { (*p_tab).z_name }, unsafe { (*p_col).z_cn_name })
                                                    };
                                                unsafe {
                                                    sqlite3_vdbe_add_op3(v, 71, 19 | 5 << 8, on_error, i_reg)
                                                };
                                                unsafe { sqlite3_vdbe_append_p4(v, z_msg as *mut (), -7) };
                                                unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
                                                break '__s25;
                                            }
                                            {
                                                { let _ = 0; };
                                                unsafe { sqlite3_vdbe_add_op2(v, 51, i_reg, ignore_dest) };
                                                break '__s25;
                                            }
                                        }
                                        1 => {
                                            {
                                                let z_msg: *mut i8 =
                                                    unsafe {
                                                        sqlite3_m_printf(db,
                                                            c"%s.%s".as_ptr() as *mut i8 as *const i8,
                                                            unsafe { (*p_tab).z_name }, unsafe { (*p_col).z_cn_name })
                                                    };
                                                unsafe {
                                                    sqlite3_vdbe_add_op3(v, 71, 19 | 5 << 8, on_error, i_reg)
                                                };
                                                unsafe { sqlite3_vdbe_append_p4(v, z_msg as *mut (), -7) };
                                                unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
                                                break '__s25;
                                            }
                                            {
                                                { let _ = 0; };
                                                unsafe { sqlite3_vdbe_add_op2(v, 51, i_reg, ignore_dest) };
                                                break '__s25;
                                            }
                                        }
                                        3 => {
                                            {
                                                let z_msg: *mut i8 =
                                                    unsafe {
                                                        sqlite3_m_printf(db,
                                                            c"%s.%s".as_ptr() as *mut i8 as *const i8,
                                                            unsafe { (*p_tab).z_name }, unsafe { (*p_col).z_cn_name })
                                                    };
                                                unsafe {
                                                    sqlite3_vdbe_add_op3(v, 71, 19 | 5 << 8, on_error, i_reg)
                                                };
                                                unsafe { sqlite3_vdbe_append_p4(v, z_msg as *mut (), -7) };
                                                unsafe { sqlite3_vdbe_change_p5(v, 1 as u16) };
                                                break '__s25;
                                            }
                                            {
                                                { let _ = 0; };
                                                unsafe { sqlite3_vdbe_add_op2(v, 51, i_reg, ignore_dest) };
                                                break '__s25;
                                            }
                                        }
                                        _ => {
                                            {
                                                { let _ = 0; };
                                                unsafe { sqlite3_vdbe_add_op2(v, 51, i_reg, ignore_dest) };
                                                break '__s25;
                                            }
                                        }
                                    }
                                }
                                break '__c24;
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if n_generated == 0 && n_seen_replace == 0 { break; }
                    if b2nd_pass != 0 { break; }
                    b2nd_pass = 1;
                    if n_seen_replace > 0 &&
                            unsafe { (*p_tab).tab_flags } & 96 as u32 != 0 as u32 {
                        sqlite3_compute_generated_columns(p_parse, reg_new_data + 1,
                            p_tab);
                    }
                }
            }
            if !(unsafe { (*p_tab).p_check }).is_null() &&
                    unsafe { (*db).flags } & 512 as u64 == 0 as u64 {
                let p_check: *const ExprList =
                    unsafe { (*p_tab).p_check } as *const ExprList;
                unsafe { (*p_parse).i_self_tab = -(reg_new_data + 1) };
                on_error =
                    if override_error as i32 != 11 {
                        override_error as i32
                    } else { 2 };
                {
                    i = 0;
                    '__b26: loop {
                        if !(i < unsafe { (*p_check).n_expr }) { break '__b26; }
                        '__c26: loop {
                            let mut all_ok: i32 = 0;
                            let mut p_copy: *mut Expr = core::ptr::null_mut();
                            let p_expr: *mut Expr =
                                unsafe {
                                    (*(unsafe { (*p_check).a.as_ptr() } as
                                                    *mut ExprListItem).offset(i as isize)).p_expr
                                };
                            if !(ai_chng).is_null() &&
                                    (sqlite3_expr_references_updated_column(p_expr, ai_chng,
                                                    pk_chng as i32) == 0) as i32 != 0 {
                                break '__c26;
                            }
                            if b_affinity_done as i32 == 0 {
                                sqlite3_table_affinity(v, p_tab, reg_new_data + 1);
                                b_affinity_done = 1 as u8;
                            }
                            all_ok = unsafe { sqlite3_vdbe_make_label(p_parse) };
                            p_copy =
                                unsafe { sqlite3_expr_dup(db, p_expr as *const Expr, 0) };
                            if (unsafe { (*db).malloc_failed } == 0) as i32 != 0 {
                                unsafe {
                                    sqlite3_expr_if_true(p_parse, p_copy, all_ok, 16)
                                };
                            }
                            unsafe { sqlite3_expr_delete(db, p_copy) };
                            if on_error == 4 {
                                unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                            } else {
                                let z_name: *mut i8 =
                                    unsafe {
                                        (*(unsafe { (*p_check).a.as_ptr() } as
                                                        *mut ExprListItem).offset(i as isize)).z_e_name
                                    };
                                { let _ = 0; };
                                if on_error == 5 { on_error = 2; }
                                unsafe {
                                    sqlite3_halt_constraint(p_parse, 19 | 1 << 8, on_error,
                                        z_name, 0 as i8, 3 as u8)
                                };
                            }
                            unsafe { sqlite3_vdbe_resolve_label(v, all_ok) };
                            break '__c26;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                unsafe { (*p_parse).i_self_tab = 0 };
            }
            s_idx_iter.e_type = 0;
            s_idx_iter.i = 0;
            s_idx_iter.u.ax.a_idx = core::ptr::null_mut();
            s_idx_iter.u.lx.p_idx = unsafe { (*p_tab).p_index };
            if !(p_upsert).is_null() {
                if unsafe { (*p_upsert).p_upsert_target } ==
                        core::ptr::null_mut() {
                    { let _ = 0; };
                    if unsafe { (*p_upsert).is_do_update } as i32 == 0 {
                        override_error = 4 as u8;
                        p_upsert = core::ptr::null_mut();
                    } else { override_error = 6 as u8; }
                } else if unsafe { (*p_tab).p_index } != core::ptr::null_mut()
                    {
                    let mut n_idx: i32 = 0;
                    let mut jj: i32 = 0;
                    let mut n_byte: u64 = 0 as u64;
                    let mut p_term: *const Upsert = core::ptr::null();
                    let mut b_used: *mut u8 = core::ptr::null_mut();
                    {
                        { n_idx = 0; p_idx = unsafe { (*p_tab).p_index } };
                        '__b27: loop {
                            if !(!(p_idx).is_null()) { break '__b27; }
                            '__c27: loop { { let _ = 0; }; break '__c27; }
                            {
                                p_idx = unsafe { (*p_idx).p_next };
                                { let __p = &mut n_idx; let __t = *__p; *__p += 1; __t }
                            };
                        }
                    }
                    s_idx_iter.e_type = 1;
                    s_idx_iter.u.ax.n_idx = n_idx;
                    n_byte =
                        ((core::mem::size_of::<IndexListTerm>() as u64 + 1 as u64) *
                                    n_idx as u64 + n_idx as u64) as u64;
                    s_idx_iter.u.ax.a_idx =
                        unsafe { sqlite3_db_malloc_zero(db, n_byte) } as
                            *mut IndexListTerm;
                    if s_idx_iter.u.ax.a_idx == core::ptr::null_mut() {
                        return;
                    }
                    b_used =
                        unsafe {
                                &raw mut *s_idx_iter.u.ax.a_idx.offset(n_idx as isize)
                            } as *mut u8;
                    unsafe {
                        (*p_upsert).p_to_free = s_idx_iter.u.ax.a_idx as *mut ()
                    };
                    {
                        { i = 0; p_term = p_upsert };
                        '__b28: loop {
                            if !(!(p_term).is_null()) { break '__b28; }
                            '__c28: loop {
                                if unsafe { (*p_term).p_upsert_target } ==
                                        core::ptr::null_mut() {
                                    break '__b28;
                                }
                                if unsafe { (*p_term).p_upsert_idx } ==
                                        core::ptr::null_mut() {
                                    break '__c28;
                                }
                                jj = 0;
                                p_idx = unsafe { (*p_tab).p_index };
                                while p_idx != core::ptr::null_mut() &&
                                        p_idx != unsafe { (*p_term).p_upsert_idx } {
                                    p_idx = unsafe { (*p_idx).p_next };
                                    { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
                                }
                                if unsafe { *b_used.offset(jj as isize) } != 0 {
                                    break '__c28;
                                }
                                unsafe { *b_used.offset(jj as isize) = 1 as u8 };
                                unsafe {
                                    (*s_idx_iter.u.ax.a_idx.offset(i as isize)).p = p_idx
                                };
                                unsafe {
                                    (*s_idx_iter.u.ax.a_idx.offset(i as isize)).ix = jj
                                };
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                break '__c28;
                            }
                            p_term = unsafe { (*p_term).p_next_upsert };
                        }
                    }
                    {
                        { jj = 0; p_idx = unsafe { (*p_tab).p_index } };
                        '__b30: loop {
                            if !(!(p_idx).is_null()) { break '__b30; }
                            '__c30: loop {
                                if unsafe { *b_used.offset(jj as isize) } != 0 {
                                    break '__c30;
                                }
                                unsafe {
                                    (*s_idx_iter.u.ax.a_idx.offset(i as isize)).p = p_idx
                                };
                                unsafe {
                                    (*s_idx_iter.u.ax.a_idx.offset(i as isize)).ix = jj
                                };
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                break '__c30;
                            }
                            {
                                p_idx = unsafe { (*p_idx).p_next };
                                { let __p = &mut jj; let __t = *__p; *__p += 1; __t }
                            };
                        }
                    }
                    { let _ = 0; };
                }
            }
            if unsafe { (*db).flags } & (8192 | 16384) as u64 == 0 as u64 {
                p_trigger = core::ptr::null_mut();
                reg_trig_cnt = 0;
            } else {
                if unsafe { (*db).flags } & 8192 as u64 != 0 {
                    p_trigger =
                        unsafe {
                            sqlite3_triggers_exist(p_parse, p_tab, 129,
                                core::ptr::null_mut(), core::ptr::null_mut())
                        };
                    reg_trig_cnt =
                        (p_trigger != core::ptr::null_mut() ||
                                unsafe {
                                        sqlite3_fk_required(p_parse, p_tab, core::ptr::null_mut(),
                                            0)
                                    } != 0) as i32;
                } else {
                    p_trigger = core::ptr::null_mut();
                    reg_trig_cnt =
                        unsafe {
                            sqlite3_fk_required(p_parse, p_tab, core::ptr::null_mut(),
                                0)
                        };
                }
                if reg_trig_cnt != 0 {
                    reg_trig_cnt =
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            *__p += 1;
                            *__p
                        };
                    unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_trig_cnt) };
                    lbl_recheck_ok =
                        unsafe { sqlite3_vdbe_make_label(p_parse) };
                    addr_recheck = lbl_recheck_ok;
                }
            }
            if pk_chng != 0 && p_pk == core::ptr::null_mut() {
                let addr_rowid_ok: i32 =
                    unsafe { sqlite3_vdbe_make_label(p_parse) };
                on_error = unsafe { (*p_tab).key_conf } as i32;
                if override_error as i32 != 11 {
                    on_error = override_error as i32;
                } else if on_error == 11 { on_error = 2; }
                if !(p_upsert).is_null() {
                    p_upsert_clause =
                        unsafe {
                            sqlite3_upsert_of_index(p_upsert, core::ptr::null_mut())
                        };
                    if p_upsert_clause != core::ptr::null_mut() {
                        if unsafe { (*p_upsert_clause).is_do_update } as i32 == 0 {
                            on_error = 4;
                        } else { on_error = 6; }
                    }
                    if p_upsert_clause != p_upsert {
                        upsert_ipk_delay = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                    }
                }
                if on_error == 5 && on_error != override_error as i32 &&
                            !(unsafe { (*p_tab).p_index }).is_null() &&
                        (upsert_ipk_delay == 0) as i32 != 0 {
                    ipk_top = unsafe { sqlite3_vdbe_add_op0(v, 9) } + 1;
                }
                if is_update != 0 {
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 54, reg_new_data, addr_rowid_ok,
                            reg_old_data)
                    };
                    unsafe { sqlite3_vdbe_change_p5(v, 144 as u16) };
                }
                unsafe {
                    sqlite3_vdbe_add_op3(v, 31, i_data_cur, addr_rowid_ok,
                        reg_new_data)
                };
                '__s31:
                    {
                    match on_error {
                        1 => {
                            {
                                unsafe {
                                    sqlite3_rowid_constraint(p_parse, on_error, p_tab)
                                };
                                break '__s31;
                            }
                            {
                                if reg_trig_cnt != 0 {
                                    unsafe { sqlite3_multi_write(p_parse) };
                                    unsafe {
                                        sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                            i_data_cur, i_idx_cur, reg_new_data, 1 as i16, 0 as u8,
                                            5 as u8, 1 as u8, -1)
                                    };
                                    unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                    {
                                        let __p = &mut n_replace_trig;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                } else {
                                    if !(unsafe { (*p_tab).p_index }).is_null() {
                                        unsafe { sqlite3_multi_write(p_parse) };
                                        unsafe {
                                            sqlite3_generate_row_index_delete(p_parse, p_tab,
                                                i_data_cur, i_idx_cur, core::ptr::null_mut(), -1)
                                        };
                                    }
                                }
                                seen_replace = 1;
                                break '__s31;
                            }
                            {
                                unsafe {
                                    sqlite3_upsert_do_update(p_parse, p_upsert, p_tab,
                                        core::ptr::null_mut(), i_data_cur)
                                };
                            }
                            {
                                unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                break '__s31;
                            }
                        }
                        2 => {
                            {
                                unsafe {
                                    sqlite3_rowid_constraint(p_parse, on_error, p_tab)
                                };
                                break '__s31;
                            }
                            {
                                if reg_trig_cnt != 0 {
                                    unsafe { sqlite3_multi_write(p_parse) };
                                    unsafe {
                                        sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                            i_data_cur, i_idx_cur, reg_new_data, 1 as i16, 0 as u8,
                                            5 as u8, 1 as u8, -1)
                                    };
                                    unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                    {
                                        let __p = &mut n_replace_trig;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                } else {
                                    if !(unsafe { (*p_tab).p_index }).is_null() {
                                        unsafe { sqlite3_multi_write(p_parse) };
                                        unsafe {
                                            sqlite3_generate_row_index_delete(p_parse, p_tab,
                                                i_data_cur, i_idx_cur, core::ptr::null_mut(), -1)
                                        };
                                    }
                                }
                                seen_replace = 1;
                                break '__s31;
                            }
                            {
                                unsafe {
                                    sqlite3_upsert_do_update(p_parse, p_upsert, p_tab,
                                        core::ptr::null_mut(), i_data_cur)
                                };
                            }
                            {
                                unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                break '__s31;
                            }
                        }
                        3 => {
                            {
                                unsafe {
                                    sqlite3_rowid_constraint(p_parse, on_error, p_tab)
                                };
                                break '__s31;
                            }
                            {
                                if reg_trig_cnt != 0 {
                                    unsafe { sqlite3_multi_write(p_parse) };
                                    unsafe {
                                        sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                            i_data_cur, i_idx_cur, reg_new_data, 1 as i16, 0 as u8,
                                            5 as u8, 1 as u8, -1)
                                    };
                                    unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                    {
                                        let __p = &mut n_replace_trig;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                } else {
                                    if !(unsafe { (*p_tab).p_index }).is_null() {
                                        unsafe { sqlite3_multi_write(p_parse) };
                                        unsafe {
                                            sqlite3_generate_row_index_delete(p_parse, p_tab,
                                                i_data_cur, i_idx_cur, core::ptr::null_mut(), -1)
                                        };
                                    }
                                }
                                seen_replace = 1;
                                break '__s31;
                            }
                            {
                                unsafe {
                                    sqlite3_upsert_do_update(p_parse, p_upsert, p_tab,
                                        core::ptr::null_mut(), i_data_cur)
                                };
                            }
                            {
                                unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                break '__s31;
                            }
                        }
                        5 => {
                            {
                                if reg_trig_cnt != 0 {
                                    unsafe { sqlite3_multi_write(p_parse) };
                                    unsafe {
                                        sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                            i_data_cur, i_idx_cur, reg_new_data, 1 as i16, 0 as u8,
                                            5 as u8, 1 as u8, -1)
                                    };
                                    unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                    {
                                        let __p = &mut n_replace_trig;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                } else {
                                    if !(unsafe { (*p_tab).p_index }).is_null() {
                                        unsafe { sqlite3_multi_write(p_parse) };
                                        unsafe {
                                            sqlite3_generate_row_index_delete(p_parse, p_tab,
                                                i_data_cur, i_idx_cur, core::ptr::null_mut(), -1)
                                        };
                                    }
                                }
                                seen_replace = 1;
                                break '__s31;
                            }
                            {
                                unsafe {
                                    sqlite3_upsert_do_update(p_parse, p_upsert, p_tab,
                                        core::ptr::null_mut(), i_data_cur)
                                };
                            }
                            {
                                unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                break '__s31;
                            }
                        }
                        6 => {
                            {
                                unsafe {
                                    sqlite3_upsert_do_update(p_parse, p_upsert, p_tab,
                                        core::ptr::null_mut(), i_data_cur)
                                };
                            }
                            {
                                unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                break '__s31;
                            }
                        }
                        4 => {
                            {
                                unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                break '__s31;
                            }
                        }
                        _ => {
                            { on_error = 2; }
                            {
                                unsafe {
                                    sqlite3_rowid_constraint(p_parse, on_error, p_tab)
                                };
                                break '__s31;
                            }
                            {
                                if reg_trig_cnt != 0 {
                                    unsafe { sqlite3_multi_write(p_parse) };
                                    unsafe {
                                        sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                            i_data_cur, i_idx_cur, reg_new_data, 1 as i16, 0 as u8,
                                            5 as u8, 1 as u8, -1)
                                    };
                                    unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                    {
                                        let __p = &mut n_replace_trig;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    };
                                } else {
                                    if !(unsafe { (*p_tab).p_index }).is_null() {
                                        unsafe { sqlite3_multi_write(p_parse) };
                                        unsafe {
                                            sqlite3_generate_row_index_delete(p_parse, p_tab,
                                                i_data_cur, i_idx_cur, core::ptr::null_mut(), -1)
                                        };
                                    }
                                }
                                seen_replace = 1;
                                break '__s31;
                            }
                            {
                                unsafe {
                                    sqlite3_upsert_do_update(p_parse, p_upsert, p_tab,
                                        core::ptr::null_mut(), i_data_cur)
                                };
                            }
                            {
                                unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                break '__s31;
                            }
                        }
                    }
                }
                unsafe { sqlite3_vdbe_resolve_label(v, addr_rowid_ok) };
                if !(p_upsert).is_null() && p_upsert_clause != p_upsert {
                    upsert_ipk_return = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                } else if ipk_top != 0 {
                    ipk_bottom = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                    unsafe { sqlite3_vdbe_jump_here(v, ipk_top - 1) };
                }
            }
            {
                p_idx = index_iterator_first(&s_idx_iter, &mut ix);
                '__b32: loop {
                    if !(!(p_idx).is_null()) { break '__b32; }
                    '__c32: loop {
                        let mut reg_idx: i32 = 0;
                        let mut reg_r: i32 = 0;
                        let mut i_this_cur: i32 = 0;
                        let mut addr_unique_ok: i32 = 0;
                        let mut addr_conflict_ck: i32 = 0;
                        if unsafe { *a_reg_idx.offset(ix as isize) } == 0 {
                            break '__c32;
                        }
                        if !(p_upsert).is_null() {
                            p_upsert_clause =
                                unsafe { sqlite3_upsert_of_index(p_upsert, p_idx) };
                            if upsert_ipk_delay != 0 && p_upsert_clause == p_upsert {
                                unsafe { sqlite3_vdbe_jump_here(v, upsert_ipk_delay) };
                            }
                        }
                        addr_unique_ok =
                            unsafe { sqlite3_vdbe_make_label(p_parse) };
                        if b_affinity_done as i32 == 0 {
                            sqlite3_table_affinity(v, p_tab, reg_new_data + 1);
                            b_affinity_done = 1 as u8;
                        }
                        i_this_cur = i_idx_cur + ix;
                        if !(unsafe { (*p_idx).p_part_idx_where }).is_null() {
                            unsafe {
                                sqlite3_vdbe_add_op2(v, 77, 0,
                                    unsafe { *a_reg_idx.offset(ix as isize) })
                            };
                            unsafe { (*p_parse).i_self_tab = -(reg_new_data + 1) };
                            unsafe {
                                sqlite3_expr_if_false_dup(p_parse,
                                    unsafe { (*p_idx).p_part_idx_where }, addr_unique_ok, 16)
                            };
                            unsafe { (*p_parse).i_self_tab = 0 };
                        }
                        reg_idx = unsafe { *a_reg_idx.offset(ix as isize) } + 1;
                        {
                            i = 0;
                            '__b33: loop {
                                if !(i < unsafe { (*p_idx).n_column } as i32) {
                                    break '__b33;
                                }
                                '__c33: loop {
                                    let i_field: i32 =
                                        unsafe { *unsafe { (*p_idx).ai_column.offset(i as isize) } }
                                            as i32;
                                    let mut x: i32 = 0;
                                    if i_field == -2 {
                                        unsafe { (*p_parse).i_self_tab = -(reg_new_data + 1) };
                                        unsafe {
                                            sqlite3_expr_code_copy(p_parse,
                                                unsafe {
                                                    (*(unsafe { (*unsafe { (*p_idx).a_col_expr }).a.as_ptr() }
                                                                    as *mut ExprListItem).offset(i as isize)).p_expr
                                                }, reg_idx + i)
                                        };
                                        unsafe { (*p_parse).i_self_tab = 0 };
                                    } else if i_field == -1 ||
                                            i_field == unsafe { (*p_tab).i_p_key } as i32 {
                                        x = reg_new_data;
                                        unsafe { sqlite3_vdbe_add_op2(v, 84, x, reg_idx + i) };
                                    } else {
                                        x =
                                            unsafe {
                                                            sqlite3_table_column_to_storage(p_tab, i_field as i16)
                                                        } as i32 + reg_new_data + 1;
                                        unsafe { sqlite3_vdbe_add_op2(v, 83, x, reg_idx + i) };
                                    }
                                    break '__c33;
                                }
                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            }
                        }
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_idx,
                                unsafe { (*p_idx).n_column } as i32,
                                unsafe { *a_reg_idx.offset(ix as isize) })
                        };
                        if is_update != 0 && p_pk == p_idx && pk_chng as i32 == 0 {
                            unsafe { sqlite3_vdbe_resolve_label(v, addr_unique_ok) };
                            break '__c32;
                        }
                        on_error = unsafe { (*p_idx).on_error } as i32;
                        if on_error == 0 {
                            unsafe { sqlite3_vdbe_resolve_label(v, addr_unique_ok) };
                            break '__c32;
                        }
                        if override_error as i32 != 11 {
                            on_error = override_error as i32;
                        } else if on_error == 11 { on_error = 2; }
                        if !(p_upsert_clause).is_null() {
                            if unsafe { (*p_upsert_clause).is_do_update } as i32 == 0 {
                                on_error = 4;
                            } else { on_error = 6; }
                        }
                        { let _ = 0; };
                        if ix == 0 &&
                                                unsafe { (*p_idx).p_next } == core::ptr::null_mut() &&
                                            p_pk == p_idx && on_error == 5 &&
                                    (0 as u64 == unsafe { (*db).flags } & 8192 as u64 ||
                                        core::ptr::null_mut() ==
                                            unsafe {
                                                sqlite3_triggers_exist(p_parse, p_tab, 129,
                                                    core::ptr::null_mut(), core::ptr::null_mut())
                                            }) &&
                                (0 as u64 == unsafe { (*db).flags } & 16384 as u64 ||
                                    core::ptr::null_mut() == unsafe { (*p_tab).u.tab.p_f_key }
                                        &&
                                        core::ptr::null_mut() ==
                                            unsafe { sqlite3_fk_references(p_tab) }) {
                            unsafe { sqlite3_vdbe_resolve_label(v, addr_unique_ok) };
                            break '__c32;
                        }
                        addr_conflict_ck =
                            unsafe {
                                sqlite3_vdbe_add_op4_int(v, 27, i_this_cur, addr_unique_ok,
                                    reg_idx, unsafe { (*p_idx).n_key_col } as i32)
                            };
                        reg_r =
                            if p_idx == p_pk {
                                reg_idx
                            } else {
                                unsafe { sqlite3_get_temp_range(p_parse, n_pk_field) }
                            };
                        if is_update != 0 || on_error == 5 {
                            if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                                unsafe { sqlite3_vdbe_add_op2(v, 144, i_this_cur, reg_r) };
                                if is_update != 0 {
                                    unsafe {
                                        sqlite3_vdbe_add_op3(v, 54, reg_r, addr_unique_ok,
                                            reg_old_data)
                                    };
                                    unsafe { sqlite3_vdbe_change_p5(v, 144 as u16) };
                                }
                            } else {
                                let mut x: i32 = 0;
                                if p_idx != p_pk {
                                    {
                                        i = 0;
                                        '__b34: loop {
                                            if !(i < unsafe { (*p_pk).n_key_col } as i32) {
                                                break '__b34;
                                            }
                                            '__c34: loop {
                                                { let _ = 0; };
                                                x =
                                                    unsafe {
                                                        sqlite3_table_column_to_index(p_idx,
                                                            unsafe { *unsafe { (*p_pk).ai_column.offset(i as isize) } }
                                                                as i32)
                                                    };
                                                unsafe {
                                                    sqlite3_vdbe_add_op3(v, 96, i_this_cur, x, reg_r + i)
                                                };
                                                break '__c34;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                }
                                if is_update != 0 {
                                    let mut addr_jump: i32 =
                                        unsafe { sqlite3_vdbe_current_addr(v) } +
                                            unsafe { (*p_pk).n_key_col } as i32;
                                    let mut op: i32 = 53;
                                    let reg_cmp: i32 =
                                        if unsafe { (*p_idx).idx_type() } as i32 == 2 {
                                            reg_idx
                                        } else { reg_r };
                                    {
                                        i = 0;
                                        '__b35: loop {
                                            if !(i < unsafe { (*p_pk).n_key_col } as i32) {
                                                break '__b35;
                                            }
                                            '__c35: loop {
                                                let p4: *const i8 =
                                                    unsafe {
                                                                sqlite3_locate_coll_seq(p_parse,
                                                                    unsafe { *unsafe { (*p_pk).az_coll.offset(i as isize) } })
                                                            } as *mut i8 as *const i8;
                                                x =
                                                    unsafe { *unsafe { (*p_pk).ai_column.offset(i as isize) } }
                                                        as i32;
                                                { let _ = 0; };
                                                if i == unsafe { (*p_pk).n_key_col } as i32 - 1 {
                                                    addr_jump = addr_unique_ok;
                                                    op = 54;
                                                }
                                                x =
                                                    unsafe { sqlite3_table_column_to_storage(p_tab, x as i16) }
                                                        as i32;
                                                unsafe {
                                                    sqlite3_vdbe_add_op4(v, op, reg_old_data + 1 + x, addr_jump,
                                                        reg_cmp + i, p4 as *const i8, -2)
                                                };
                                                unsafe { sqlite3_vdbe_change_p5(v, 144 as u16) };
                                                break '__c35;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                }
                            }
                        }
                        { let _ = 0; };
                        '__s36:
                            {
                            match on_error {
                                1 => {
                                    {
                                        unsafe {
                                            sqlite3_unique_constraint(p_parse, on_error, p_idx)
                                        };
                                        break '__s36;
                                    }
                                    {
                                        unsafe {
                                            sqlite3_upsert_do_update(p_parse, p_upsert, p_tab, p_idx,
                                                i_idx_cur + ix)
                                        };
                                    }
                                    {
                                        unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                        break '__s36;
                                    }
                                    {
                                        let mut n_conflict_ck: i32 = 0;
                                        { let _ = 0; };
                                        n_conflict_ck =
                                            unsafe { sqlite3_vdbe_current_addr(v) } - addr_conflict_ck;
                                        { let _ = 0; };
                                        if reg_trig_cnt != 0 {
                                            unsafe { sqlite3_multi_write(p_parse) };
                                            {
                                                let __p = &mut n_replace_trig;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            };
                                        }
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 169, i_data_cur) };
                                        }
                                        unsafe {
                                            sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                                i_data_cur, i_idx_cur, reg_r, n_pk_field as i16, 0 as u8,
                                                5 as u8, if p_idx == p_pk { 1 } else { 0 } as u8,
                                                i_this_cur)
                                        };
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 170, i_data_cur) };
                                        }
                                        if reg_trig_cnt != 0 {
                                            let mut addr_bypass: i32 = 0;
                                            unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                            addr_bypass = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                                            unsafe { sqlite3_vdbe_resolve_label(v, lbl_recheck_ok) };
                                            lbl_recheck_ok =
                                                unsafe { sqlite3_vdbe_make_label(p_parse) };
                                            if !(unsafe { (*p_idx).p_part_idx_where }).is_null() {
                                                unsafe {
                                                    sqlite3_vdbe_add_op2(v, 51, reg_idx - 1, lbl_recheck_ok)
                                                };
                                            }
                                            while n_conflict_ck > 0 {
                                                let mut x: VdbeOp = unsafe { core::mem::zeroed() };
                                                x =
                                                    unsafe {
                                                        core::ptr::read(unsafe {
                                                                sqlite3_vdbe_get_op(v, addr_conflict_ck)
                                                            })
                                                    };
                                                if x.opcode as i32 != 144 {
                                                    let mut p2: i32 = 0;
                                                    let mut z_p4: *const i8 = core::ptr::null();
                                                    if unsafe {
                                                                        *(sqlite3_opcode_property.as_ptr() as
                                                                                    *const u8).add(x.opcode as usize)
                                                                    } as i32 & 1 != 0 {
                                                        p2 = lbl_recheck_ok;
                                                    } else { p2 = x.p2; }
                                                    z_p4 =
                                                        if x.p4type as i32 == -3 {
                                                                x.p4.i as i64 as *mut ()
                                                            } else { x.p4.z as *mut () } as *const i8;
                                                    unsafe {
                                                        sqlite3_vdbe_add_op4(v, x.opcode as i32, x.p1, p2, x.p3,
                                                            z_p4, x.p4type as i32)
                                                    };
                                                    unsafe { sqlite3_vdbe_change_p5(v, x.p5) };
                                                }
                                                {
                                                    let __p = &mut n_conflict_ck;
                                                    let __t = *__p;
                                                    *__p -= 1;
                                                    __t
                                                };
                                                {
                                                    let __p = &mut addr_conflict_ck;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                };
                                            }
                                            unsafe { sqlite3_unique_constraint(p_parse, 2, p_idx) };
                                            unsafe { sqlite3_vdbe_jump_here(v, addr_bypass) };
                                        }
                                        seen_replace = 1;
                                        break '__s36;
                                    }
                                }
                                2 => {
                                    {
                                        unsafe {
                                            sqlite3_unique_constraint(p_parse, on_error, p_idx)
                                        };
                                        break '__s36;
                                    }
                                    {
                                        unsafe {
                                            sqlite3_upsert_do_update(p_parse, p_upsert, p_tab, p_idx,
                                                i_idx_cur + ix)
                                        };
                                    }
                                    {
                                        unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                        break '__s36;
                                    }
                                    {
                                        let mut n_conflict_ck: i32 = 0;
                                        { let _ = 0; };
                                        n_conflict_ck =
                                            unsafe { sqlite3_vdbe_current_addr(v) } - addr_conflict_ck;
                                        { let _ = 0; };
                                        if reg_trig_cnt != 0 {
                                            unsafe { sqlite3_multi_write(p_parse) };
                                            {
                                                let __p = &mut n_replace_trig;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            };
                                        }
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 169, i_data_cur) };
                                        }
                                        unsafe {
                                            sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                                i_data_cur, i_idx_cur, reg_r, n_pk_field as i16, 0 as u8,
                                                5 as u8, if p_idx == p_pk { 1 } else { 0 } as u8,
                                                i_this_cur)
                                        };
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 170, i_data_cur) };
                                        }
                                        if reg_trig_cnt != 0 {
                                            let mut addr_bypass: i32 = 0;
                                            unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                            addr_bypass = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                                            unsafe { sqlite3_vdbe_resolve_label(v, lbl_recheck_ok) };
                                            lbl_recheck_ok =
                                                unsafe { sqlite3_vdbe_make_label(p_parse) };
                                            if !(unsafe { (*p_idx).p_part_idx_where }).is_null() {
                                                unsafe {
                                                    sqlite3_vdbe_add_op2(v, 51, reg_idx - 1, lbl_recheck_ok)
                                                };
                                            }
                                            while n_conflict_ck > 0 {
                                                let mut x: VdbeOp = unsafe { core::mem::zeroed() };
                                                x =
                                                    unsafe {
                                                        core::ptr::read(unsafe {
                                                                sqlite3_vdbe_get_op(v, addr_conflict_ck)
                                                            })
                                                    };
                                                if x.opcode as i32 != 144 {
                                                    let mut p2: i32 = 0;
                                                    let mut z_p4: *const i8 = core::ptr::null();
                                                    if unsafe {
                                                                        *(sqlite3_opcode_property.as_ptr() as
                                                                                    *const u8).add(x.opcode as usize)
                                                                    } as i32 & 1 != 0 {
                                                        p2 = lbl_recheck_ok;
                                                    } else { p2 = x.p2; }
                                                    z_p4 =
                                                        if x.p4type as i32 == -3 {
                                                                x.p4.i as i64 as *mut ()
                                                            } else { x.p4.z as *mut () } as *const i8;
                                                    unsafe {
                                                        sqlite3_vdbe_add_op4(v, x.opcode as i32, x.p1, p2, x.p3,
                                                            z_p4, x.p4type as i32)
                                                    };
                                                    unsafe { sqlite3_vdbe_change_p5(v, x.p5) };
                                                }
                                                {
                                                    let __p = &mut n_conflict_ck;
                                                    let __t = *__p;
                                                    *__p -= 1;
                                                    __t
                                                };
                                                {
                                                    let __p = &mut addr_conflict_ck;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                };
                                            }
                                            unsafe { sqlite3_unique_constraint(p_parse, 2, p_idx) };
                                            unsafe { sqlite3_vdbe_jump_here(v, addr_bypass) };
                                        }
                                        seen_replace = 1;
                                        break '__s36;
                                    }
                                }
                                3 => {
                                    {
                                        unsafe {
                                            sqlite3_unique_constraint(p_parse, on_error, p_idx)
                                        };
                                        break '__s36;
                                    }
                                    {
                                        unsafe {
                                            sqlite3_upsert_do_update(p_parse, p_upsert, p_tab, p_idx,
                                                i_idx_cur + ix)
                                        };
                                    }
                                    {
                                        unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                        break '__s36;
                                    }
                                    {
                                        let mut n_conflict_ck: i32 = 0;
                                        { let _ = 0; };
                                        n_conflict_ck =
                                            unsafe { sqlite3_vdbe_current_addr(v) } - addr_conflict_ck;
                                        { let _ = 0; };
                                        if reg_trig_cnt != 0 {
                                            unsafe { sqlite3_multi_write(p_parse) };
                                            {
                                                let __p = &mut n_replace_trig;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            };
                                        }
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 169, i_data_cur) };
                                        }
                                        unsafe {
                                            sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                                i_data_cur, i_idx_cur, reg_r, n_pk_field as i16, 0 as u8,
                                                5 as u8, if p_idx == p_pk { 1 } else { 0 } as u8,
                                                i_this_cur)
                                        };
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 170, i_data_cur) };
                                        }
                                        if reg_trig_cnt != 0 {
                                            let mut addr_bypass: i32 = 0;
                                            unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                            addr_bypass = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                                            unsafe { sqlite3_vdbe_resolve_label(v, lbl_recheck_ok) };
                                            lbl_recheck_ok =
                                                unsafe { sqlite3_vdbe_make_label(p_parse) };
                                            if !(unsafe { (*p_idx).p_part_idx_where }).is_null() {
                                                unsafe {
                                                    sqlite3_vdbe_add_op2(v, 51, reg_idx - 1, lbl_recheck_ok)
                                                };
                                            }
                                            while n_conflict_ck > 0 {
                                                let mut x: VdbeOp = unsafe { core::mem::zeroed() };
                                                x =
                                                    unsafe {
                                                        core::ptr::read(unsafe {
                                                                sqlite3_vdbe_get_op(v, addr_conflict_ck)
                                                            })
                                                    };
                                                if x.opcode as i32 != 144 {
                                                    let mut p2: i32 = 0;
                                                    let mut z_p4: *const i8 = core::ptr::null();
                                                    if unsafe {
                                                                        *(sqlite3_opcode_property.as_ptr() as
                                                                                    *const u8).add(x.opcode as usize)
                                                                    } as i32 & 1 != 0 {
                                                        p2 = lbl_recheck_ok;
                                                    } else { p2 = x.p2; }
                                                    z_p4 =
                                                        if x.p4type as i32 == -3 {
                                                                x.p4.i as i64 as *mut ()
                                                            } else { x.p4.z as *mut () } as *const i8;
                                                    unsafe {
                                                        sqlite3_vdbe_add_op4(v, x.opcode as i32, x.p1, p2, x.p3,
                                                            z_p4, x.p4type as i32)
                                                    };
                                                    unsafe { sqlite3_vdbe_change_p5(v, x.p5) };
                                                }
                                                {
                                                    let __p = &mut n_conflict_ck;
                                                    let __t = *__p;
                                                    *__p -= 1;
                                                    __t
                                                };
                                                {
                                                    let __p = &mut addr_conflict_ck;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                };
                                            }
                                            unsafe { sqlite3_unique_constraint(p_parse, 2, p_idx) };
                                            unsafe { sqlite3_vdbe_jump_here(v, addr_bypass) };
                                        }
                                        seen_replace = 1;
                                        break '__s36;
                                    }
                                }
                                6 => {
                                    {
                                        unsafe {
                                            sqlite3_upsert_do_update(p_parse, p_upsert, p_tab, p_idx,
                                                i_idx_cur + ix)
                                        };
                                    }
                                    {
                                        unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                        break '__s36;
                                    }
                                    {
                                        let mut n_conflict_ck: i32 = 0;
                                        { let _ = 0; };
                                        n_conflict_ck =
                                            unsafe { sqlite3_vdbe_current_addr(v) } - addr_conflict_ck;
                                        { let _ = 0; };
                                        if reg_trig_cnt != 0 {
                                            unsafe { sqlite3_multi_write(p_parse) };
                                            {
                                                let __p = &mut n_replace_trig;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            };
                                        }
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 169, i_data_cur) };
                                        }
                                        unsafe {
                                            sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                                i_data_cur, i_idx_cur, reg_r, n_pk_field as i16, 0 as u8,
                                                5 as u8, if p_idx == p_pk { 1 } else { 0 } as u8,
                                                i_this_cur)
                                        };
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 170, i_data_cur) };
                                        }
                                        if reg_trig_cnt != 0 {
                                            let mut addr_bypass: i32 = 0;
                                            unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                            addr_bypass = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                                            unsafe { sqlite3_vdbe_resolve_label(v, lbl_recheck_ok) };
                                            lbl_recheck_ok =
                                                unsafe { sqlite3_vdbe_make_label(p_parse) };
                                            if !(unsafe { (*p_idx).p_part_idx_where }).is_null() {
                                                unsafe {
                                                    sqlite3_vdbe_add_op2(v, 51, reg_idx - 1, lbl_recheck_ok)
                                                };
                                            }
                                            while n_conflict_ck > 0 {
                                                let mut x: VdbeOp = unsafe { core::mem::zeroed() };
                                                x =
                                                    unsafe {
                                                        core::ptr::read(unsafe {
                                                                sqlite3_vdbe_get_op(v, addr_conflict_ck)
                                                            })
                                                    };
                                                if x.opcode as i32 != 144 {
                                                    let mut p2: i32 = 0;
                                                    let mut z_p4: *const i8 = core::ptr::null();
                                                    if unsafe {
                                                                        *(sqlite3_opcode_property.as_ptr() as
                                                                                    *const u8).add(x.opcode as usize)
                                                                    } as i32 & 1 != 0 {
                                                        p2 = lbl_recheck_ok;
                                                    } else { p2 = x.p2; }
                                                    z_p4 =
                                                        if x.p4type as i32 == -3 {
                                                                x.p4.i as i64 as *mut ()
                                                            } else { x.p4.z as *mut () } as *const i8;
                                                    unsafe {
                                                        sqlite3_vdbe_add_op4(v, x.opcode as i32, x.p1, p2, x.p3,
                                                            z_p4, x.p4type as i32)
                                                    };
                                                    unsafe { sqlite3_vdbe_change_p5(v, x.p5) };
                                                }
                                                {
                                                    let __p = &mut n_conflict_ck;
                                                    let __t = *__p;
                                                    *__p -= 1;
                                                    __t
                                                };
                                                {
                                                    let __p = &mut addr_conflict_ck;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                };
                                            }
                                            unsafe { sqlite3_unique_constraint(p_parse, 2, p_idx) };
                                            unsafe { sqlite3_vdbe_jump_here(v, addr_bypass) };
                                        }
                                        seen_replace = 1;
                                        break '__s36;
                                    }
                                }
                                4 => {
                                    {
                                        unsafe { sqlite3_vdbe_goto(v, ignore_dest) };
                                        break '__s36;
                                    }
                                    {
                                        let mut n_conflict_ck: i32 = 0;
                                        { let _ = 0; };
                                        n_conflict_ck =
                                            unsafe { sqlite3_vdbe_current_addr(v) } - addr_conflict_ck;
                                        { let _ = 0; };
                                        if reg_trig_cnt != 0 {
                                            unsafe { sqlite3_multi_write(p_parse) };
                                            {
                                                let __p = &mut n_replace_trig;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            };
                                        }
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 169, i_data_cur) };
                                        }
                                        unsafe {
                                            sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                                i_data_cur, i_idx_cur, reg_r, n_pk_field as i16, 0 as u8,
                                                5 as u8, if p_idx == p_pk { 1 } else { 0 } as u8,
                                                i_this_cur)
                                        };
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 170, i_data_cur) };
                                        }
                                        if reg_trig_cnt != 0 {
                                            let mut addr_bypass: i32 = 0;
                                            unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                            addr_bypass = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                                            unsafe { sqlite3_vdbe_resolve_label(v, lbl_recheck_ok) };
                                            lbl_recheck_ok =
                                                unsafe { sqlite3_vdbe_make_label(p_parse) };
                                            if !(unsafe { (*p_idx).p_part_idx_where }).is_null() {
                                                unsafe {
                                                    sqlite3_vdbe_add_op2(v, 51, reg_idx - 1, lbl_recheck_ok)
                                                };
                                            }
                                            while n_conflict_ck > 0 {
                                                let mut x: VdbeOp = unsafe { core::mem::zeroed() };
                                                x =
                                                    unsafe {
                                                        core::ptr::read(unsafe {
                                                                sqlite3_vdbe_get_op(v, addr_conflict_ck)
                                                            })
                                                    };
                                                if x.opcode as i32 != 144 {
                                                    let mut p2: i32 = 0;
                                                    let mut z_p4: *const i8 = core::ptr::null();
                                                    if unsafe {
                                                                        *(sqlite3_opcode_property.as_ptr() as
                                                                                    *const u8).add(x.opcode as usize)
                                                                    } as i32 & 1 != 0 {
                                                        p2 = lbl_recheck_ok;
                                                    } else { p2 = x.p2; }
                                                    z_p4 =
                                                        if x.p4type as i32 == -3 {
                                                                x.p4.i as i64 as *mut ()
                                                            } else { x.p4.z as *mut () } as *const i8;
                                                    unsafe {
                                                        sqlite3_vdbe_add_op4(v, x.opcode as i32, x.p1, p2, x.p3,
                                                            z_p4, x.p4type as i32)
                                                    };
                                                    unsafe { sqlite3_vdbe_change_p5(v, x.p5) };
                                                }
                                                {
                                                    let __p = &mut n_conflict_ck;
                                                    let __t = *__p;
                                                    *__p -= 1;
                                                    __t
                                                };
                                                {
                                                    let __p = &mut addr_conflict_ck;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                };
                                            }
                                            unsafe { sqlite3_unique_constraint(p_parse, 2, p_idx) };
                                            unsafe { sqlite3_vdbe_jump_here(v, addr_bypass) };
                                        }
                                        seen_replace = 1;
                                        break '__s36;
                                    }
                                }
                                _ => {
                                    {
                                        let mut n_conflict_ck: i32 = 0;
                                        { let _ = 0; };
                                        n_conflict_ck =
                                            unsafe { sqlite3_vdbe_current_addr(v) } - addr_conflict_ck;
                                        { let _ = 0; };
                                        if reg_trig_cnt != 0 {
                                            unsafe { sqlite3_multi_write(p_parse) };
                                            {
                                                let __p = &mut n_replace_trig;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            };
                                        }
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 169, i_data_cur) };
                                        }
                                        unsafe {
                                            sqlite3_generate_row_delete(p_parse, p_tab, p_trigger,
                                                i_data_cur, i_idx_cur, reg_r, n_pk_field as i16, 0 as u8,
                                                5 as u8, if p_idx == p_pk { 1 } else { 0 } as u8,
                                                i_this_cur)
                                        };
                                        if !(p_trigger).is_null() && is_update != 0 {
                                            unsafe { sqlite3_vdbe_add_op1(v, 170, i_data_cur) };
                                        }
                                        if reg_trig_cnt != 0 {
                                            let mut addr_bypass: i32 = 0;
                                            unsafe { sqlite3_vdbe_add_op2(v, 88, reg_trig_cnt, 1) };
                                            addr_bypass = unsafe { sqlite3_vdbe_add_op0(v, 9) };
                                            unsafe { sqlite3_vdbe_resolve_label(v, lbl_recheck_ok) };
                                            lbl_recheck_ok =
                                                unsafe { sqlite3_vdbe_make_label(p_parse) };
                                            if !(unsafe { (*p_idx).p_part_idx_where }).is_null() {
                                                unsafe {
                                                    sqlite3_vdbe_add_op2(v, 51, reg_idx - 1, lbl_recheck_ok)
                                                };
                                            }
                                            while n_conflict_ck > 0 {
                                                let mut x: VdbeOp = unsafe { core::mem::zeroed() };
                                                x =
                                                    unsafe {
                                                        core::ptr::read(unsafe {
                                                                sqlite3_vdbe_get_op(v, addr_conflict_ck)
                                                            })
                                                    };
                                                if x.opcode as i32 != 144 {
                                                    let mut p2: i32 = 0;
                                                    let mut z_p4: *const i8 = core::ptr::null();
                                                    if unsafe {
                                                                        *(sqlite3_opcode_property.as_ptr() as
                                                                                    *const u8).add(x.opcode as usize)
                                                                    } as i32 & 1 != 0 {
                                                        p2 = lbl_recheck_ok;
                                                    } else { p2 = x.p2; }
                                                    z_p4 =
                                                        if x.p4type as i32 == -3 {
                                                                x.p4.i as i64 as *mut ()
                                                            } else { x.p4.z as *mut () } as *const i8;
                                                    unsafe {
                                                        sqlite3_vdbe_add_op4(v, x.opcode as i32, x.p1, p2, x.p3,
                                                            z_p4, x.p4type as i32)
                                                    };
                                                    unsafe { sqlite3_vdbe_change_p5(v, x.p5) };
                                                }
                                                {
                                                    let __p = &mut n_conflict_ck;
                                                    let __t = *__p;
                                                    *__p -= 1;
                                                    __t
                                                };
                                                {
                                                    let __p = &mut addr_conflict_ck;
                                                    let __t = *__p;
                                                    *__p += 1;
                                                    __t
                                                };
                                            }
                                            unsafe { sqlite3_unique_constraint(p_parse, 2, p_idx) };
                                            unsafe { sqlite3_vdbe_jump_here(v, addr_bypass) };
                                        }
                                        seen_replace = 1;
                                        break '__s36;
                                    }
                                }
                            }
                        }
                        unsafe { sqlite3_vdbe_resolve_label(v, addr_unique_ok) };
                        if reg_r != reg_idx {
                            unsafe {
                                sqlite3_release_temp_range(p_parse, reg_r, n_pk_field)
                            };
                        }
                        if !(p_upsert_clause).is_null() && upsert_ipk_return != 0 &&
                                unsafe { sqlite3_upsert_next_is_ipk(p_upsert_clause) } != 0
                            {
                            unsafe { sqlite3_vdbe_goto(v, upsert_ipk_delay + 1) };
                            unsafe { sqlite3_vdbe_jump_here(v, upsert_ipk_return) };
                            upsert_ipk_return = 0;
                        }
                        break '__c32;
                    }
                    p_idx = index_iterator_next(&mut s_idx_iter, &mut ix);
                }
            }
            if ipk_top != 0 {
                unsafe { sqlite3_vdbe_goto(v, ipk_top) };
                { let _ = 0; };
                unsafe { sqlite3_vdbe_jump_here(v, ipk_bottom) };
            }
            { let _ = 0; };
            if n_replace_trig != 0 {
                unsafe {
                    sqlite3_vdbe_add_op2(v, 17, reg_trig_cnt, lbl_recheck_ok)
                };
                if (p_pk).is_null() as i32 != 0 {
                    if is_update != 0 {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 54, reg_new_data, addr_recheck,
                                reg_old_data)
                        };
                        unsafe { sqlite3_vdbe_change_p5(v, 144 as u16) };
                    }
                    unsafe {
                        sqlite3_vdbe_add_op3(v, 31, i_data_cur, addr_recheck,
                            reg_new_data)
                    };
                    unsafe { sqlite3_rowid_constraint(p_parse, 2, p_tab) };
                } else { unsafe { sqlite3_vdbe_goto(v, addr_recheck) }; }
                unsafe { sqlite3_vdbe_resolve_label(v, lbl_recheck_ok) };
            }
            if unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32 {
                let reg_rec: i32 = unsafe { *a_reg_idx.offset(ix as isize) };
                unsafe {
                    sqlite3_vdbe_add_op3(v, 99, reg_new_data + 1,
                        unsafe { (*p_tab).n_nv_col } as i32, reg_rec)
                };
                if (b_affinity_done == 0) as i32 != 0 {
                    sqlite3_table_affinity(v, p_tab, 0);
                }
            }
            *pb_may_replace = seen_replace;
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_complete_insertion(p_parse: &Parse,
    p_tab: *mut Table, i_data_cur: i32, i_idx_cur: i32, reg_new_data: i32,
    a_reg_idx: *mut i32, update_flags: i32, append_bias: i32,
    use_seek_result: i32) -> () {
    let mut v: *mut Vdbe = core::ptr::null_mut();
    let mut p_idx: *const Index = core::ptr::null();
    let mut pik_flags: u8 = 0 as u8;
    let mut i: i32 = 0;
    { let _ = 0; };
    v = (*p_parse).p_vdbe;
    { let _ = 0; };
    { let _ = 0; };
    {
        { i = 0; p_idx = unsafe { (*p_tab).p_index } };
        '__b38: loop {
            if !(!(p_idx).is_null()) { break '__b38; }
            '__c38: loop {
                { let _ = 0; };
                if unsafe { *a_reg_idx.offset(i as isize) } == 0 {
                    break '__c38;
                }
                if !(unsafe { (*p_idx).p_part_idx_where }).is_null() ||
                        update_flags != 0 && unsafe { (*p_idx).b_has_expr() } != 0 {
                    unsafe {
                        sqlite3_vdbe_add_op2(v, 51,
                            unsafe { *a_reg_idx.offset(i as isize) },
                            unsafe { sqlite3_vdbe_current_addr(v) } + 2)
                    };
                }
                pik_flags = if use_seek_result != 0 { 16 } else { 0 } as u8;
                if unsafe { (*p_idx).idx_type() } as i32 == 2 &&
                        !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32) as
                                i32 != 0 {
                    pik_flags |= 1 as u8;
                    pik_flags |= (update_flags & 2) as u8;
                    if update_flags == 0 {}
                }
                unsafe {
                    sqlite3_vdbe_add_op4_int(v, 140, i_idx_cur + i,
                        unsafe { *a_reg_idx.offset(i as isize) },
                        unsafe { *a_reg_idx.offset(i as isize) } + 1,
                        if unsafe { (*p_idx).uniq_not_null() } != 0 {
                            (unsafe { (*p_idx).n_key_col }) as i32
                        } else { (unsafe { (*p_idx).n_column }) as i32 })
                };
                unsafe { sqlite3_vdbe_change_p5(v, pik_flags as u16) };
                break '__c38;
            }
            {
                p_idx = unsafe { (*p_idx).p_next };
                { let __p = &mut i; let __t = *__p; *__p += 1; __t }
            };
        }
    }
    if !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32) as i32 != 0 {
        return;
    }
    if (*p_parse).nested != 0 {
        pik_flags = 0 as u8;
    } else {
        pik_flags = 1 as u8;
        pik_flags |= if update_flags != 0 { update_flags } else { 32 } as u8;
    }
    if append_bias != 0 { pik_flags |= 8 as u8; }
    if use_seek_result != 0 { pik_flags |= 16 as u8; }
    unsafe {
        sqlite3_vdbe_add_op3(v, 130, i_data_cur,
            unsafe { *a_reg_idx.offset(i as isize) }, reg_new_data)
    };
    if ((*p_parse).nested == 0) as i32 != 0 {
        unsafe { sqlite3_vdbe_append_p4(v, p_tab as *mut (), -5) };
    }
    unsafe { sqlite3_vdbe_change_p5(v, pik_flags as u16) };
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_insert(p_parse: *mut Parse,
    p_tab_list: *mut SrcList, mut p_select: *mut Select,
    p_column: *mut IdList, on_error: i32, p_upsert: *mut Upsert) -> () {
    unsafe {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut p_tab: *mut Table = core::ptr::null_mut();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut v: *mut Vdbe = core::ptr::null_mut();
        let mut p_idx: *const Index = core::ptr::null();
        let mut n_column: i32 = 0;
        let mut n_hidden: i32 = 0;
        let mut i_data_cur: i32 = 0;
        let mut i_idx_cur: i32 = 0;
        let mut ipk_column: i32 = 0;
        let mut end_of_loop: i32 = 0;
        let mut src_tab: i32 = 0;
        let mut addr_ins_top: i32 = 0;
        let mut addr_cont: i32 = 0;
        let mut dest: SelectDest = unsafe { core::mem::zeroed() };
        let mut i_db: i32 = 0;
        let mut use_temp_table: u8 = 0 as u8;
        let mut append_flag: u8 = 0 as u8;
        let mut without_rowid: u8 = 0 as u8;
        let mut b_id_list_in_order: u8 = 0 as u8;
        let mut p_list: *mut ExprList = core::ptr::null_mut();
        let mut i_reg_store: i32 = 0;
        let mut reg_from_select: i32 = 0;
        let mut reg_autoinc: i32 = 0;
        let mut reg_row_count: i32 = 0;
        let mut reg_ins: i32 = 0;
        let mut reg_rowid: i32 = 0;
        let mut reg_data: i32 = 0;
        let mut a_reg_idx: *mut i32 = core::ptr::null_mut();
        let mut a_tab_col_map: *mut i32 = core::ptr::null_mut();
        let mut is_view: i32 = 0;
        let mut p_trigger: *mut Trigger = core::ptr::null_mut();
        let mut tmask: i32 = 0;
        let mut rc: i32 = 0;
        let mut p_item: *mut SrcItem = core::ptr::null_mut();
        let mut p_subq: *const Subquery = core::ptr::null();
        let mut addr_top: i32 = 0;
        let mut reg_yield: i32 = 0;
        let mut reg_rec: i32 = 0;
        let mut reg_temp_rowid: i32 = 0;
        let mut addr_l: i32 = 0;
        let mut s_nc: NameContext = unsafe { core::mem::zeroed() };
        let mut n_idx: i32 = 0;
        let mut p_nx: *mut Upsert = core::ptr::null_mut();
        let mut k: i32 = 0;
        let mut col_flags: u32 = 0 as u32;
        let mut p_x: *mut Expr = core::ptr::null_mut();
        let mut y: i32 = 0;
        let mut reg_cols: i32 = 0;
        let mut addr1: i32 = 0;
        let mut p_ipk: *const Expr = core::ptr::null();
        let mut addr1__1: i32 = 0;
        let mut p_v_tab: *const i8 = core::ptr::null();
        let mut is_replace: i32 = 0;
        let mut b_use_seek: i32 = 0;
        let mut __state: i32 = 0;
        loop {
            if __state == 1 { break; }
            '__s40:
                {
                match __state {
                    0 => { __state = 4; }
                    2 => {
                        if unsafe { (*p_parse).nested } as i32 == 0 &&
                                unsafe { (*p_parse).p_trigger_tab } == core::ptr::null_mut()
                            {
                            __state = 374;
                        } else { __state = 373; }
                    }
                    3 => {
                        unsafe { sqlite3_src_list_delete(db, p_tab_list) };
                        __state = 377;
                    }
                    4 => { __state = 5; }
                    5 => { __state = 6; }
                    6 => { __state = 7; }
                    7 => { __state = 8; }
                    8 => { __state = 9; }
                    9 => { n_hidden = 0; __state = 10; }
                    10 => { i_data_cur = 0; __state = 11; }
                    11 => { i_idx_cur = 0; __state = 12; }
                    12 => { ipk_column = -1; __state = 13; }
                    13 => { __state = 14; }
                    14 => { src_tab = 0; __state = 15; }
                    15 => { addr_ins_top = 0; __state = 16; }
                    16 => { addr_cont = 0; __state = 17; }
                    17 => { __state = 18; }
                    18 => { __state = 19; }
                    19 => { use_temp_table = 0 as u8; __state = 20; }
                    20 => { append_flag = 0 as u8; __state = 21; }
                    21 => { __state = 22; }
                    22 => { __state = 23; }
                    23 => { p_list = core::ptr::null_mut(); __state = 24; }
                    24 => { __state = 25; }
                    25 => { reg_from_select = 0; __state = 26; }
                    26 => { reg_autoinc = 0; __state = 27; }
                    27 => { reg_row_count = 0; __state = 28; }
                    28 => { __state = 29; }
                    29 => { __state = 30; }
                    30 => { __state = 31; }
                    31 => { a_reg_idx = core::ptr::null_mut(); __state = 32; }
                    32 => {
                        a_tab_col_map = core::ptr::null_mut();
                        __state = 33;
                    }
                    33 => { __state = 34; }
                    34 => { __state = 35; }
                    35 => { __state = 36; }
                    36 => { db = unsafe { (*p_parse).db }; __state = 37; }
                    37 => { { let _ = 0; }; __state = 38; }
                    38 => {
                        if unsafe { (*p_parse).n_err } != 0 {
                            __state = 40;
                        } else { __state = 39; }
                    }
                    39 => { { let _ = 0; }; __state = 41; }
                    40 => { __state = 3; }
                    41 => { dest.i_sd_parm = 0; __state = 42; }
                    42 => {
                        if !(p_select).is_null() &&
                                    unsafe { (*p_select).sel_flags } & 512 as u32 != 0 as u32 &&
                                unsafe { (*p_select).p_prior } == core::ptr::null_mut() {
                            __state = 44;
                        } else { __state = 43; }
                    }
                    43 => { { let _ = 0; }; __state = 48; }
                    44 => {
                        p_list = unsafe { (*p_select).p_e_list };
                        __state = 45;
                    }
                    45 => {
                        unsafe { (*p_select).p_e_list = core::ptr::null_mut() };
                        __state = 46;
                    }
                    46 => {
                        unsafe { sqlite3_select_delete(db, p_select) };
                        __state = 47;
                    }
                    47 => { p_select = core::ptr::null_mut(); __state = 43; }
                    48 => {
                        p_tab =
                            unsafe { sqlite3_src_list_lookup(p_parse, p_tab_list) };
                        __state = 49;
                    }
                    49 => {
                        if p_tab == core::ptr::null_mut() {
                            __state = 51;
                        } else { __state = 50; }
                    }
                    50 => {
                        i_db =
                            unsafe {
                                sqlite3_schema_to_index(db, unsafe { (*p_tab).p_schema })
                            };
                        __state = 52;
                    }
                    51 => { __state = 3; }
                    52 => { { let _ = 0; }; __state = 53; }
                    53 => {
                        if unsafe {
                                    sqlite3_auth_check(p_parse, 18,
                                        unsafe { (*p_tab).z_name } as *const i8, core::ptr::null(),
                                        unsafe {
                                                (*unsafe { (*db).a_db.offset(i_db as isize) }).z_db_s_name
                                            } as *const i8)
                                } != 0 {
                            __state = 55;
                        } else { __state = 54; }
                    }
                    54 => {
                        without_rowid =
                            !(unsafe { (*p_tab).tab_flags } & 128 as u32 == 0 as u32) as
                                    i32 as u8;
                        __state = 56;
                    }
                    55 => { __state = 3; }
                    56 => {
                        p_trigger =
                            unsafe {
                                sqlite3_triggers_exist(p_parse, p_tab, 128,
                                    core::ptr::null_mut(), &mut tmask)
                            };
                        __state = 57;
                    }
                    57 => {
                        is_view =
                            (unsafe { (*p_tab).e_tab_type } as i32 == 2) as i32;
                        __state = 58;
                    }
                    58 => { { let _ = 0; }; __state = 59; }
                    59 => {
                        if unsafe { sqlite3_view_get_column_names(p_parse, p_tab) }
                                != 0 {
                            __state = 61;
                        } else { __state = 60; }
                    }
                    60 => {
                        if unsafe {
                                    sqlite3_is_read_only(p_parse, p_tab, p_trigger)
                                } != 0 {
                            __state = 63;
                        } else { __state = 62; }
                    }
                    61 => { __state = 3; }
                    62 => {
                        v = unsafe { sqlite3_get_vdbe(p_parse) };
                        __state = 64;
                    }
                    63 => { __state = 3; }
                    64 => {
                        if v == core::ptr::null_mut() {
                            __state = 66;
                        } else { __state = 65; }
                    }
                    65 => {
                        if unsafe { (*p_parse).nested } as i32 == 0 {
                            __state = 68;
                        } else { __state = 67; }
                    }
                    66 => { __state = 3; }
                    67 => {
                        unsafe {
                            sqlite3_begin_write_operation(p_parse,
                                (!(p_select).is_null() || !(p_trigger).is_null()) as i32,
                                i_db)
                        };
                        __state = 69;
                    }
                    68 => {
                        unsafe { sqlite3_vdbe_count_changes(v) };
                        __state = 67;
                    }
                    69 => {
                        if p_column == core::ptr::null_mut() &&
                                        p_select != core::ptr::null_mut() &&
                                    p_trigger == core::ptr::null_mut() &&
                                xfer_optimization(p_parse, p_tab, unsafe { &*p_select },
                                        on_error, i_db) != 0 {
                            __state = 71;
                        } else { __state = 70; }
                    }
                    70 => {
                        reg_autoinc = auto_inc_begin(p_parse, i_db, p_tab);
                        __state = 74;
                    }
                    71 => { { let _ = 0; }; __state = 72; }
                    72 => { { let _ = 0; }; __state = 73; }
                    73 => { __state = 2; }
                    74 => {
                        reg_rowid =
                            { reg_ins = unsafe { (*p_parse).n_mem } + 1; reg_ins };
                        __state = 75;
                    }
                    75 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_tab).n_col } as i32 + 1
                        };
                        __state = 76;
                    }
                    76 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                            __state = 78;
                        } else { __state = 77; }
                    }
                    77 => { reg_data = reg_rowid + 1; __state = 80; }
                    78 => {
                        {
                            let __p = &mut reg_rowid;
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 79;
                    }
                    79 => {
                        {
                            let __p = unsafe { &mut (*p_parse).n_mem };
                            let __t = *__p;
                            *__p += 1;
                            __t
                        };
                        __state = 77;
                    }
                    80 => {
                        b_id_list_in_order =
                            (unsafe { (*p_tab).tab_flags } & (1024 | 64) as u32 ==
                                    0 as u32) as u8;
                        __state = 81;
                    }
                    81 => {
                        if !(p_column).is_null() {
                            __state = 83;
                        } else { __state = 82; }
                    }
                    82 => {
                        if !(p_select).is_null() {
                            __state = 108;
                        } else { __state = 109; }
                    }
                    83 => {
                        a_tab_col_map =
                            unsafe {
                                    sqlite3_db_malloc_zero(db,
                                        unsafe { (*p_tab).n_col } as u64 *
                                            core::mem::size_of::<i32>() as u64)
                                } as *mut i32;
                        __state = 84;
                    }
                    84 => {
                        if a_tab_col_map == core::ptr::null_mut() {
                            __state = 86;
                        } else { __state = 85; }
                    }
                    85 => { i = 0; __state = 87; }
                    86 => { __state = 3; }
                    87 => {
                        if i < unsafe { (*p_column).n_id } {
                            __state = 88;
                        } else { __state = 82; }
                    }
                    88 => {
                        j =
                            unsafe {
                                sqlite3_column_index(p_tab,
                                    unsafe {
                                            (*(unsafe { (*p_column).a.as_ptr() } as
                                                            *mut IdListItem).offset(i as isize)).z_name
                                        } as *const i8)
                            };
                        __state = 90;
                    }
                    89 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 87;
                    }
                    90 => { if j >= 0 { __state = 91; } else { __state = 92; } }
                    91 => {
                        if unsafe { *a_tab_col_map.offset(j as isize) } == 0 {
                            __state = 94;
                        } else { __state = 93; }
                    }
                    92 => {
                        if unsafe {
                                        sqlite3_is_rowid(unsafe {
                                                    (*(unsafe { (*p_column).a.as_ptr() } as
                                                                    *mut IdListItem).offset(i as isize)).z_name
                                                } as *const i8)
                                    } != 0 && (without_rowid == 0) as i32 != 0 {
                            __state = 102;
                        } else { __state = 103; }
                    }
                    93 => { if i != j { __state = 96; } else { __state = 95; } }
                    94 => {
                        unsafe { *a_tab_col_map.offset(j as isize) = i + 1 };
                        __state = 93;
                    }
                    95 => {
                        if j == unsafe { (*p_tab).i_p_key } as i32 {
                            __state = 98;
                        } else { __state = 97; }
                    }
                    96 => { b_id_list_in_order = 0 as u8; __state = 95; }
                    97 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(j as isize) }).col_flags
                                        } as i32 & (64 | 32) != 0 {
                            __state = 100;
                        } else { __state = 89; }
                    }
                    98 => { ipk_column = i; __state = 99; }
                    99 => { { let _ = 0; }; __state = 97; }
                    100 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot INSERT into generated column \"%s\"".as_ptr() as
                                        *mut i8 as *const i8,
                                unsafe {
                                    (*unsafe { (*p_tab).a_col.offset(j as isize) }).z_cn_name
                                })
                        };
                        __state = 101;
                    }
                    101 => { __state = 3; }
                    102 => { ipk_column = i; __state = 104; }
                    103 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"table %S has no column named %s".as_ptr() as *mut i8 as
                                    *const i8,
                                unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem,
                                unsafe {
                                    (*(unsafe { (*p_column).a.as_ptr() } as
                                                    *mut IdListItem).offset(i as isize)).z_name
                                })
                        };
                        __state = 105;
                    }
                    104 => { b_id_list_in_order = 0 as u8; __state = 89; }
                    105 => {
                        unsafe { (*p_parse).set_check_schema(1 as Bft as u32) };
                        __state = 106;
                    }
                    106 => { __state = 3; }
                    107 => {
                        if p_column == core::ptr::null_mut() && n_column > 0 {
                            __state = 171;
                        } else { __state = 170; }
                    }
                    108 => { __state = 110; }
                    109 => { __state = 161; }
                    110 => {
                        if unsafe { (*unsafe { (*p_select).p_src }).n_src } == 1 &&
                                    unsafe {
                                            (*(unsafe { (*unsafe { (*p_select).p_src }).a.as_ptr() } as
                                                                *mut SrcItem).offset(0 as isize)).fg.via_coroutine()
                                        } != 0 &&
                                unsafe { (*p_select).p_prior } == core::ptr::null_mut() {
                            __state = 112;
                        } else { __state = 113; }
                    }
                    111 => {
                        if !(p_trigger).is_null() ||
                                reads_table(p_parse, i_db, p_tab) != 0 {
                            __state = 144;
                        } else { __state = 143; }
                    }
                    112 => {
                        p_item =
                            unsafe {
                                &mut *(unsafe { (*unsafe { (*p_select).p_src }).a.as_ptr() }
                                                as *mut SrcItem).offset(0 as isize)
                            };
                        __state = 114;
                    }
                    113 => { __state = 127; }
                    114 => { __state = 115; }
                    115 => { { let _ = 0; }; __state = 116; }
                    116 => {
                        p_subq = unsafe { (*p_item).u4.p_subq };
                        __state = 117;
                    }
                    117 => {
                        dest.i_sd_parm = unsafe { (*p_subq).reg_return };
                        __state = 118;
                    }
                    118 => {
                        reg_from_select = unsafe { (*p_subq).reg_result };
                        __state = 119;
                    }
                    119 => { { let _ = 0; }; __state = 120; }
                    120 => { { let _ = 0; }; __state = 121; }
                    121 => {
                        n_column =
                            unsafe {
                                (*unsafe {
                                                (*unsafe { (*p_subq).p_select }).p_e_list
                                            }).n_expr
                            };
                        __state = 122;
                    }
                    122 => {
                        unsafe {
                            sqlite3_vdbe_explain(p_parse, 0 as u8,
                                c"SCAN %S".as_ptr() as *mut i8 as *const i8, p_item)
                        };
                        __state = 123;
                    }
                    123 => {
                        if b_id_list_in_order != 0 &&
                                n_column == unsafe { (*p_tab).n_col } as i32 {
                            __state = 124;
                        } else { __state = 111; }
                    }
                    124 => { reg_data = reg_from_select; __state = 125; }
                    125 => { reg_rowid = reg_data - 1; __state = 126; }
                    126 => {
                        reg_ins =
                            reg_rowid -
                                if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                                    1
                                } else { 0 };
                        __state = 111;
                    }
                    127 => {
                        reg_yield =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 128;
                    }
                    128 => {
                        addr_top = unsafe { sqlite3_vdbe_current_addr(v) } + 1;
                        __state = 129;
                    }
                    129 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 11, reg_yield, 0, addr_top)
                        };
                        __state = 130;
                    }
                    130 => {
                        unsafe {
                            sqlite3_select_dest_init(&mut dest, 11, reg_yield)
                        };
                        __state = 131;
                    }
                    131 => {
                        dest.i_sdst =
                            if b_id_list_in_order != 0 { reg_data } else { 0 };
                        __state = 132;
                    }
                    132 => {
                        dest.n_sdst = unsafe { (*p_tab).n_col } as i32;
                        __state = 133;
                    }
                    133 => {
                        rc =
                            unsafe { sqlite3_select(p_parse, p_select, &mut dest) };
                        __state = 134;
                    }
                    134 => { reg_from_select = dest.i_sdst; __state = 135; }
                    135 => { { let _ = 0; }; __state = 136; }
                    136 => {
                        if rc != 0 || unsafe { (*p_parse).n_err } != 0 {
                            __state = 138;
                        } else { __state = 137; }
                    }
                    137 => { { let _ = 0; }; __state = 139; }
                    138 => { __state = 3; }
                    139 => {
                        unsafe { sqlite3_vdbe_end_coroutine(v, reg_yield) };
                        __state = 140;
                    }
                    140 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_top - 1) };
                        __state = 141;
                    }
                    141 => { { let _ = 0; }; __state = 142; }
                    142 => {
                        n_column =
                            unsafe { (*unsafe { (*p_select).p_e_list }).n_expr };
                        __state = 111;
                    }
                    143 => {
                        if use_temp_table != 0 {
                            __state = 145;
                        } else { __state = 107; }
                    }
                    144 => { use_temp_table = 1 as u8; __state = 143; }
                    145 => { __state = 146; }
                    146 => { __state = 147; }
                    147 => { __state = 148; }
                    148 => {
                        src_tab =
                            {
                                let __p = unsafe { &mut (*p_parse).n_tab };
                                let __t = *__p;
                                *__p += 1;
                                __t
                            };
                        __state = 149;
                    }
                    149 => {
                        reg_rec = unsafe { sqlite3_get_temp_reg(p_parse) };
                        __state = 150;
                    }
                    150 => {
                        reg_temp_rowid = unsafe { sqlite3_get_temp_reg(p_parse) };
                        __state = 151;
                    }
                    151 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 120, src_tab, n_column) };
                        __state = 152;
                    }
                    152 => {
                        addr_l =
                            unsafe { sqlite3_vdbe_add_op1(v, 12, dest.i_sd_parm) };
                        __state = 153;
                    }
                    153 => { __state = 154; }
                    154 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 99, reg_from_select, n_column,
                                reg_rec)
                        };
                        __state = 155;
                    }
                    155 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 129, src_tab, reg_temp_rowid)
                        };
                        __state = 156;
                    }
                    156 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 130, src_tab, reg_rec,
                                reg_temp_rowid)
                        };
                        __state = 157;
                    }
                    157 => {
                        unsafe { sqlite3_vdbe_goto(v, addr_l) };
                        __state = 158;
                    }
                    158 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_l) };
                        __state = 159;
                    }
                    159 => {
                        unsafe { sqlite3_release_temp_reg(p_parse, reg_rec) };
                        __state = 160;
                    }
                    160 => {
                        unsafe {
                            sqlite3_release_temp_reg(p_parse, reg_temp_rowid)
                        };
                        __state = 107;
                    }
                    161 => {
                        unsafe {
                            memset(&raw mut s_nc as *mut (), 0,
                                core::mem::size_of::<NameContext>() as u64)
                        };
                        __state = 162;
                    }
                    162 => { s_nc.p_parse = p_parse; __state = 163; }
                    163 => { src_tab = -1; __state = 164; }
                    164 => { { let _ = 0; }; __state = 165; }
                    165 => {
                        if !(p_list).is_null() {
                            __state = 166;
                        } else { __state = 167; }
                    }
                    166 => {
                        n_column = unsafe { (*p_list).n_expr };
                        __state = 168;
                    }
                    167 => { n_column = 0; __state = 107; }
                    168 => {
                        if unsafe {
                                    sqlite3_resolve_expr_list_names(&mut s_nc, p_list)
                                } != 0 {
                            __state = 169;
                        } else { __state = 107; }
                    }
                    169 => { __state = 3; }
                    170 => {
                        if p_column != core::ptr::null_mut() &&
                                n_column != unsafe { (*p_column).n_id } {
                            __state = 195;
                        } else { __state = 194; }
                    }
                    171 => {
                        ipk_column = unsafe { (*p_tab).i_p_key } as i32;
                        __state = 172;
                    }
                    172 => {
                        if ipk_column >= 0 &&
                                unsafe { (*p_tab).tab_flags } & 96 as u32 != 0 as u32 {
                            __state = 174;
                        } else { __state = 173; }
                    }
                    173 => { { let _ = 0; }; __state = 183; }
                    174 => { __state = 175; }
                    175 => { __state = 176; }
                    176 => { i = ipk_column - 1; __state = 177; }
                    177 => {
                        if i >= 0 { __state = 178; } else { __state = 173; }
                    }
                    178 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                        } as i32 & 96 != 0 {
                            __state = 180;
                        } else { __state = 179; }
                    }
                    179 => {
                        { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
                        __state = 177;
                    }
                    180 => { __state = 181; }
                    181 => { __state = 182; }
                    182 => {
                        {
                            let __p = &mut ipk_column;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 179;
                    }
                    183 => { { let _ = 0; }; __state = 184; }
                    184 => { { let _ = 0; }; __state = 185; }
                    185 => {
                        if unsafe { (*p_tab).tab_flags } & (96 | 2) as u32 !=
                                0 as u32 {
                            __state = 187;
                        } else { __state = 186; }
                    }
                    186 => {
                        if n_column != unsafe { (*p_tab).n_col } as i32 - n_hidden {
                            __state = 192;
                        } else { __state = 170; }
                    }
                    187 => { i = 0; __state = 188; }
                    188 => {
                        if i < unsafe { (*p_tab).n_col } as i32 {
                            __state = 189;
                        } else { __state = 186; }
                    }
                    189 => {
                        if unsafe {
                                            (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                        } as i32 & 98 != 0 {
                            __state = 191;
                        } else { __state = 190; }
                    }
                    190 => {
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                        __state = 188;
                    }
                    191 => {
                        { let __p = &mut n_hidden; let __t = *__p; *__p += 1; __t };
                        __state = 190;
                    }
                    192 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"table %S has %d columns but %d values were supplied".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { (*p_tab_list).a.as_ptr() } as *mut SrcItem,
                                unsafe { (*p_tab).n_col } as i32 - n_hidden, n_column)
                        };
                        __state = 193;
                    }
                    193 => { __state = 3; }
                    194 => {
                        if unsafe { (*db).flags } & (1 as u64) << 32 != 0 as u64 &&
                                        (unsafe { (*p_parse).nested } == 0) as i32 != 0 &&
                                    (unsafe { (*p_parse).p_trigger_tab }).is_null() as i32 != 0
                                && (unsafe { (*p_parse).b_returning() } == 0) as i32 != 0 {
                            __state = 198;
                        } else { __state = 197; }
                    }
                    195 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"%d values for %d columns".as_ptr() as *mut i8 as
                                    *const i8, n_column, unsafe { (*p_column).n_id })
                        };
                        __state = 196;
                    }
                    196 => { __state = 3; }
                    197 => {
                        if (is_view == 0) as i32 != 0 {
                            __state = 201;
                        } else { __state = 200; }
                    }
                    198 => {
                        reg_row_count =
                            {
                                let __p = unsafe { &mut (*p_parse).n_mem };
                                *__p += 1;
                                *__p
                            };
                        __state = 199;
                    }
                    199 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, 0, reg_row_count) };
                        __state = 197;
                    }
                    200 => {
                        if !(p_upsert).is_null() {
                            __state = 214;
                        } else { __state = 213; }
                    }
                    201 => { __state = 202; }
                    202 => {
                        n_idx =
                            sqlite3_open_table_and_indices(p_parse, p_tab, 116, 0 as u8,
                                -1, core::ptr::null_mut(), &mut i_data_cur, &mut i_idx_cur);
                        __state = 203;
                    }
                    203 => {
                        a_reg_idx =
                            unsafe {
                                    sqlite3_db_malloc_raw_nn(db,
                                        core::mem::size_of::<i32>() as u64 * (n_idx + 2) as u64)
                                } as *mut i32;
                        __state = 204;
                    }
                    204 => {
                        if a_reg_idx == core::ptr::null_mut() {
                            __state = 206;
                        } else { __state = 205; }
                    }
                    205 => {
                        { i = 0; p_idx = unsafe { (*p_tab).p_index } };
                        __state = 208;
                    }
                    206 => { __state = 3; }
                    207 => {
                        unsafe {
                            *a_reg_idx.offset(i as isize) =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 200;
                    }
                    208 => {
                        if i < n_idx { __state = 209; } else { __state = 207; }
                    }
                    209 => { { let _ = 0; }; __state = 211; }
                    210 => {
                        {
                            p_idx = unsafe { (*p_idx).p_next };
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t }
                        };
                        __state = 208;
                    }
                    211 => {
                        unsafe {
                            *a_reg_idx.offset(i as isize) =
                                {
                                    let __p = unsafe { &mut (*p_parse).n_mem };
                                    *__p += 1;
                                    *__p
                                }
                        };
                        __state = 212;
                    }
                    212 => {
                        unsafe {
                            (*p_parse).n_mem += unsafe { (*p_idx).n_column } as i32
                        };
                        __state = 210;
                    }
                    213 => {
                        if use_temp_table != 0 {
                            __state = 235;
                        } else { __state = 236; }
                    }
                    214 => { __state = 215; }
                    215 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                            __state = 217;
                        } else { __state = 216; }
                    }
                    216 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 2 {
                            __state = 220;
                        } else { __state = 219; }
                    }
                    217 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"UPSERT not implemented for virtual table \"%s\"".as_ptr()
                                        as *mut i8 as *const i8, unsafe { (*p_tab).z_name })
                        };
                        __state = 218;
                    }
                    218 => { __state = 3; }
                    219 => {
                        if unsafe {
                                    sqlite3_has_explicit_nulls(p_parse,
                                        unsafe { (*p_upsert).p_upsert_target })
                                } != 0 {
                            __state = 223;
                        } else { __state = 222; }
                    }
                    220 => {
                        unsafe {
                            sqlite3_error_msg(p_parse,
                                c"cannot UPSERT a view".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 221;
                    }
                    221 => { __state = 3; }
                    222 => {
                        unsafe {
                            (*(unsafe { (*p_tab_list).a.as_ptr() } as
                                                *mut SrcItem).offset(0 as isize)).i_cursor = i_data_cur
                        };
                        __state = 224;
                    }
                    223 => { __state = 3; }
                    224 => { p_nx = p_upsert; __state = 225; }
                    225 => {
                        unsafe { (*p_nx).p_upsert_src = p_tab_list };
                        __state = 227;
                    }
                    226 => {
                        if p_nx != core::ptr::null_mut() {
                            __state = 225;
                        } else { __state = 213; }
                    }
                    227 => {
                        unsafe { (*p_nx).reg_data = reg_data };
                        __state = 228;
                    }
                    228 => {
                        unsafe { (*p_nx).i_data_cur = i_data_cur };
                        __state = 229;
                    }
                    229 => {
                        unsafe { (*p_nx).i_idx_cur = i_idx_cur };
                        __state = 230;
                    }
                    230 => {
                        if !(unsafe { (*p_nx).p_upsert_target }).is_null() {
                            __state = 232;
                        } else { __state = 231; }
                    }
                    231 => {
                        p_nx = unsafe { (*p_nx).p_next_upsert };
                        __state = 226;
                    }
                    232 => {
                        if unsafe {
                                    sqlite3_upsert_analyze_target(p_parse, p_tab_list, p_nx,
                                        p_upsert)
                                } != 0 {
                            __state = 233;
                        } else { __state = 231; }
                    }
                    233 => { __state = 3; }
                    234 => { n_hidden = 0; __state = 244; }
                    235 => {
                        addr_ins_top =
                            unsafe { sqlite3_vdbe_add_op1(v, 36, src_tab) };
                        __state = 237;
                    }
                    236 => {
                        if !(p_select).is_null() {
                            __state = 239;
                        } else { __state = 234; }
                    }
                    237 => { __state = 238; }
                    238 => {
                        addr_cont = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 234;
                    }
                    239 => { __state = 240; }
                    240 => {
                        addr_ins_top =
                            {
                                addr_cont =
                                    unsafe { sqlite3_vdbe_add_op1(v, 12, dest.i_sd_parm) };
                                addr_cont
                            };
                        __state = 241;
                    }
                    241 => { __state = 242; }
                    242 => {
                        if ipk_column >= 0 {
                            __state = 243;
                        } else { __state = 234; }
                    }
                    243 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 82, reg_from_select + ipk_column,
                                reg_rowid)
                        };
                        __state = 234;
                    }
                    244 => { i_reg_store = reg_data; __state = 245; }
                    245 => { { let _ = 0; }; __state = 246; }
                    246 => { i = 0; __state = 248; }
                    247 => {
                        end_of_loop = unsafe { sqlite3_vdbe_make_label(p_parse) };
                        __state = 288;
                    }
                    248 => {
                        if i < unsafe { (*p_tab).n_col } as i32 {
                            __state = 249;
                        } else { __state = 247; }
                    }
                    249 => { __state = 251; }
                    250 => {
                        {
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            {
                                let __p = &mut i_reg_store;
                                let __t = *__p;
                                *__p += 1;
                                __t
                            }
                        };
                        __state = 248;
                    }
                    251 => { __state = 252; }
                    252 => { { let _ = 0; }; __state = 253; }
                    253 => {
                        if i == unsafe { (*p_tab).i_p_key } as i32 {
                            __state = 255;
                        } else { __state = 254; }
                    }
                    254 => {
                        if {
                                        col_flags =
                                            unsafe {
                                                    (*unsafe { (*p_tab).a_col.offset(i as isize) }).col_flags
                                                } as u32;
                                        col_flags
                                    } & 98 as u32 != 0 as u32 {
                            __state = 258;
                        } else { __state = 257; }
                    }
                    255 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 78, i_reg_store) };
                        __state = 256;
                    }
                    256 => { __state = 250; }
                    257 => {
                        if !(p_column).is_null() {
                            __state = 270;
                        } else { __state = 271; }
                    }
                    258 => {
                        { let __p = &mut n_hidden; let __t = *__p; *__p += 1; __t };
                        __state = 259;
                    }
                    259 => {
                        if col_flags & 32 as u32 != 0 as u32 {
                            __state = 260;
                        } else { __state = 261; }
                    }
                    260 => {
                        {
                            let __p = &mut i_reg_store;
                            let __t = *__p;
                            *__p -= 1;
                            __t
                        };
                        __state = 262;
                    }
                    261 => {
                        if col_flags & 64 as u32 != 0 as u32 {
                            __state = 263;
                        } else { __state = 264; }
                    }
                    262 => { __state = 250; }
                    263 => {
                        if tmask & 1 != 0 { __state = 266; } else { __state = 265; }
                    }
                    264 => {
                        if p_column == core::ptr::null_mut() {
                            __state = 267;
                        } else { __state = 257; }
                    }
                    265 => { __state = 250; }
                    266 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 78, i_reg_store) };
                        __state = 265;
                    }
                    267 => {
                        unsafe {
                            sqlite3_expr_code_factorable(p_parse,
                                unsafe {
                                    sqlite3_column_expr(p_tab,
                                        unsafe {
                                            &mut *unsafe { (*p_tab).a_col.offset(i as isize) }
                                        })
                                }, i_reg_store)
                        };
                        __state = 268;
                    }
                    268 => { __state = 250; }
                    269 => {
                        if use_temp_table != 0 {
                            __state = 280;
                        } else { __state = 281; }
                    }
                    270 => {
                        j = unsafe { *a_tab_col_map.offset(i as isize) };
                        __state = 272;
                    }
                    271 => {
                        if n_column == 0 { __state = 277; } else { __state = 278; }
                    }
                    272 => { { let _ = 0; }; __state = 273; }
                    273 => {
                        if j == 0 { __state = 275; } else { __state = 274; }
                    }
                    274 => { k = j - 1; __state = 269; }
                    275 => {
                        unsafe {
                            sqlite3_expr_code_factorable(p_parse,
                                unsafe {
                                    sqlite3_column_expr(p_tab,
                                        unsafe {
                                            &mut *unsafe { (*p_tab).a_col.offset(i as isize) }
                                        })
                                }, i_reg_store)
                        };
                        __state = 276;
                    }
                    276 => { __state = 250; }
                    277 => {
                        unsafe {
                            sqlite3_expr_code_factorable(p_parse,
                                unsafe {
                                    sqlite3_column_expr(p_tab,
                                        unsafe {
                                            &mut *unsafe { (*p_tab).a_col.offset(i as isize) }
                                        })
                                }, i_reg_store)
                        };
                        __state = 279;
                    }
                    278 => { k = i - n_hidden; __state = 269; }
                    279 => { __state = 250; }
                    280 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, src_tab, k, i_reg_store)
                        };
                        __state = 250;
                    }
                    281 => {
                        if !(p_select).is_null() {
                            __state = 282;
                        } else { __state = 283; }
                    }
                    282 => {
                        if reg_from_select != reg_data {
                            __state = 284;
                        } else { __state = 250; }
                    }
                    283 => {
                        p_x =
                            unsafe {
                                (*(unsafe { (*p_list).a.as_ptr() } as
                                                *mut ExprListItem).offset(k as isize)).p_expr
                            };
                        __state = 285;
                    }
                    284 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 83, reg_from_select + k,
                                i_reg_store)
                        };
                        __state = 250;
                    }
                    285 => {
                        y =
                            unsafe {
                                sqlite3_expr_code_target(p_parse, p_x, i_reg_store)
                            };
                        __state = 286;
                    }
                    286 => {
                        if y != i_reg_store {
                            __state = 287;
                        } else { __state = 250; }
                    }
                    287 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v,
                                if unsafe { (*p_x).flags } & 4194304 as u32 != 0 as u32 {
                                    82
                                } else { 83 }, y, i_reg_store)
                        };
                        __state = 250;
                    }
                    288 => {
                        if tmask & 1 != 0 { __state = 290; } else { __state = 289; }
                    }
                    289 => {
                        if (is_view == 0) as i32 != 0 {
                            __state = 316;
                        } else { __state = 315; }
                    }
                    290 => {
                        reg_cols =
                            unsafe {
                                sqlite3_get_temp_range(p_parse,
                                    unsafe { (*p_tab).n_col } as i32 + 1)
                            };
                        __state = 291;
                    }
                    291 => {
                        if ipk_column < 0 { __state = 293; } else { __state = 294; }
                    }
                    292 => { { let _ = 0; }; __state = 306; }
                    293 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, -1, reg_cols) };
                        __state = 292;
                    }
                    294 => { __state = 295; }
                    295 => { { let _ = 0; }; __state = 296; }
                    296 => {
                        if use_temp_table != 0 {
                            __state = 298;
                        } else { __state = 299; }
                    }
                    297 => {
                        addr1 = unsafe { sqlite3_vdbe_add_op1(v, 52, reg_cols) };
                        __state = 301;
                    }
                    298 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, src_tab, ipk_column, reg_cols)
                        };
                        __state = 297;
                    }
                    299 => { { let _ = 0; }; __state = 300; }
                    300 => {
                        unsafe {
                            sqlite3_expr_code(p_parse,
                                unsafe {
                                    (*(unsafe { (*p_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(ipk_column as isize)).p_expr
                                }, reg_cols)
                        };
                        __state = 297;
                    }
                    301 => { __state = 302; }
                    302 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 73, -1, reg_cols) };
                        __state = 303;
                    }
                    303 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr1) };
                        __state = 304;
                    }
                    304 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 13, reg_cols) };
                        __state = 305;
                    }
                    305 => { __state = 292; }
                    306 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 82, reg_rowid + 1, reg_cols + 1,
                                unsafe { (*p_tab).n_nv_col } as i32 - 1)
                        };
                        __state = 307;
                    }
                    307 => {
                        if unsafe { (*p_tab).tab_flags } & 96 as u32 != 0 {
                            __state = 309;
                        } else { __state = 308; }
                    }
                    308 => {
                        if (is_view == 0) as i32 != 0 {
                            __state = 313;
                        } else { __state = 312; }
                    }
                    309 => { __state = 310; }
                    310 => { __state = 311; }
                    311 => {
                        sqlite3_compute_generated_columns(p_parse, reg_cols + 1,
                            p_tab);
                        __state = 308;
                    }
                    312 => {
                        unsafe {
                            sqlite3_code_row_trigger(p_parse, p_trigger, 128,
                                core::ptr::null_mut(), 1, p_tab,
                                reg_cols - unsafe { (*p_tab).n_col } as i32 - 1, on_error,
                                end_of_loop)
                        };
                        __state = 314;
                    }
                    313 => {
                        sqlite3_table_affinity(v, p_tab, reg_cols + 1);
                        __state = 312;
                    }
                    314 => {
                        unsafe {
                            sqlite3_release_temp_range(p_parse, reg_cols,
                                unsafe { (*p_tab).n_col } as i32 + 1)
                        };
                        __state = 289;
                    }
                    315 => {
                        if reg_row_count != 0 {
                            __state = 361;
                        } else { __state = 360; }
                    }
                    316 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                            __state = 318;
                        } else { __state = 317; }
                    }
                    317 => {
                        if ipk_column >= 0 {
                            __state = 320;
                        } else { __state = 321; }
                    }
                    318 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, reg_ins) };
                        __state = 317;
                    }
                    319 => {
                        auto_inc_step(unsafe { &*p_parse }, reg_autoinc, reg_rowid);
                        __state = 345;
                    }
                    320 => {
                        if use_temp_table != 0 {
                            __state = 323;
                        } else { __state = 324; }
                    }
                    321 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 ||
                                without_rowid != 0 {
                            __state = 342;
                        } else { __state = 343; }
                    }
                    322 => {
                        if (append_flag == 0) as i32 != 0 {
                            __state = 331;
                        } else { __state = 319; }
                    }
                    323 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 96, src_tab, ipk_column, reg_rowid)
                        };
                        __state = 322;
                    }
                    324 => {
                        if !(p_select).is_null() {
                            __state = 325;
                        } else { __state = 326; }
                    }
                    325 => { __state = 322; }
                    326 => {
                        p_ipk =
                            unsafe {
                                    (*(unsafe { (*p_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(ipk_column as isize)).p_expr
                                } as *const Expr;
                        __state = 327;
                    }
                    327 => {
                        if unsafe { (*p_ipk).op } as i32 == 122 &&
                                !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0 {
                            __state = 328;
                        } else { __state = 329; }
                    }
                    328 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 129, i_data_cur, reg_rowid,
                                reg_autoinc)
                        };
                        __state = 330;
                    }
                    329 => {
                        unsafe {
                            sqlite3_expr_code(p_parse,
                                unsafe {
                                    (*(unsafe { (*p_list).a.as_ptr() } as
                                                    *mut ExprListItem).offset(ipk_column as isize)).p_expr
                                }, reg_rowid)
                        };
                        __state = 322;
                    }
                    330 => { append_flag = 1 as u8; __state = 322; }
                    331 => { __state = 332; }
                    332 => {
                        if !(unsafe { (*p_tab).e_tab_type } as i32 == 1) as i32 != 0
                            {
                            __state = 334;
                        } else { __state = 335; }
                    }
                    333 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 13, reg_rowid) };
                        __state = 341;
                    }
                    334 => {
                        addr1__1 =
                            unsafe { sqlite3_vdbe_add_op1(v, 52, reg_rowid) };
                        __state = 336;
                    }
                    335 => {
                        addr1__1 = unsafe { sqlite3_vdbe_current_addr(v) };
                        __state = 339;
                    }
                    336 => { __state = 337; }
                    337 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 129, i_data_cur, reg_rowid,
                                reg_autoinc)
                        };
                        __state = 338;
                    }
                    338 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr1__1) };
                        __state = 333;
                    }
                    339 => {
                        unsafe {
                            sqlite3_vdbe_add_op2(v, 51, reg_rowid, addr1__1 + 2)
                        };
                        __state = 340;
                    }
                    340 => { __state = 333; }
                    341 => { __state = 319; }
                    342 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 77, 0, reg_rowid) };
                        __state = 319;
                    }
                    343 => {
                        unsafe {
                            sqlite3_vdbe_add_op3(v, 129, i_data_cur, reg_rowid,
                                reg_autoinc)
                        };
                        __state = 344;
                    }
                    344 => { append_flag = 1 as u8; __state = 319; }
                    345 => {
                        if unsafe { (*p_tab).tab_flags } & 96 as u32 != 0 {
                            __state = 347;
                        } else { __state = 346; }
                    }
                    346 => {
                        if unsafe { (*p_tab).e_tab_type } as i32 == 1 {
                            __state = 348;
                        } else { __state = 349; }
                    }
                    347 => {
                        sqlite3_compute_generated_columns(p_parse, reg_rowid + 1,
                            p_tab);
                        __state = 346;
                    }
                    348 => {
                        p_v_tab =
                            unsafe { sqlite3_get_v_table(db, p_tab) } as *const i8;
                        __state = 350;
                    }
                    349 => { is_replace = 0; __state = 354; }
                    350 => {
                        unsafe { sqlite3_vtab_make_writable(p_parse, p_tab) };
                        __state = 351;
                    }
                    351 => {
                        unsafe {
                            sqlite3_vdbe_add_op4(v, 7, 1,
                                unsafe { (*p_tab).n_col } as i32 + 2, reg_ins, p_v_tab, -12)
                        };
                        __state = 352;
                    }
                    352 => {
                        unsafe {
                            sqlite3_vdbe_change_p5(v,
                                if on_error == 11 { 2 } else { on_error } as u16)
                        };
                        __state = 353;
                    }
                    353 => {
                        unsafe { sqlite3_may_abort(p_parse) };
                        __state = 315;
                    }
                    354 => { __state = 355; }
                    355 => {
                        sqlite3_generate_constraint_checks(p_parse, p_tab,
                            a_reg_idx, i_data_cur, i_idx_cur, reg_ins, 0,
                            (ipk_column >= 0) as u8, on_error as u8, end_of_loop,
                            &mut is_replace, core::ptr::null_mut(), p_upsert);
                        __state = 356;
                    }
                    356 => {
                        if unsafe { (*db).flags } & 16384 as u64 != 0 {
                            __state = 358;
                        } else { __state = 357; }
                    }
                    357 => {
                        b_use_seek =
                            (is_replace == 0 ||
                                    (unsafe { sqlite3_vdbe_has_sub_program(v) } == 0) as i32 !=
                                        0) as i32;
                        __state = 359;
                    }
                    358 => {
                        unsafe {
                            sqlite3_fk_check(p_parse, p_tab, 0, reg_ins,
                                core::ptr::null_mut(), 0)
                        };
                        __state = 357;
                    }
                    359 => {
                        sqlite3_complete_insertion(unsafe { &*p_parse }, p_tab,
                            i_data_cur, i_idx_cur, reg_ins, a_reg_idx, 0,
                            append_flag as i32, b_use_seek);
                        __state = 315;
                    }
                    360 => {
                        if !(p_trigger).is_null() {
                            __state = 363;
                        } else { __state = 362; }
                    }
                    361 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 88, reg_row_count, 1) };
                        __state = 360;
                    }
                    362 => {
                        unsafe { sqlite3_vdbe_resolve_label(v, end_of_loop) };
                        __state = 364;
                    }
                    363 => {
                        unsafe {
                            sqlite3_code_row_trigger(p_parse, p_trigger, 128,
                                core::ptr::null_mut(), 2, p_tab,
                                reg_data - 2 - unsafe { (*p_tab).n_col } as i32, on_error,
                                end_of_loop)
                        };
                        __state = 362;
                    }
                    364 => {
                        if use_temp_table != 0 {
                            __state = 366;
                        } else { __state = 367; }
                    }
                    365 => { __state = 2; }
                    366 => {
                        unsafe { sqlite3_vdbe_add_op2(v, 40, src_tab, addr_cont) };
                        __state = 368;
                    }
                    367 => {
                        if !(p_select).is_null() {
                            __state = 371;
                        } else { __state = 365; }
                    }
                    368 => { __state = 369; }
                    369 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_ins_top) };
                        __state = 370;
                    }
                    370 => {
                        unsafe { sqlite3_vdbe_add_op1(v, 124, src_tab) };
                        __state = 365;
                    }
                    371 => {
                        unsafe { sqlite3_vdbe_goto(v, addr_cont) };
                        __state = 372;
                    }
                    372 => {
                        unsafe { sqlite3_vdbe_jump_here(v, addr_ins_top) };
                        __state = 365;
                    }
                    373 => {
                        if reg_row_count != 0 {
                            __state = 376;
                        } else { __state = 375; }
                    }
                    374 => {
                        sqlite3_autoincrement_end(p_parse);
                        __state = 373;
                    }
                    375 => { __state = 3; }
                    376 => {
                        unsafe {
                            sqlite3_code_change_count(v, reg_row_count,
                                c"rows inserted".as_ptr() as *mut i8 as *const i8)
                        };
                        __state = 375;
                    }
                    377 => {
                        unsafe { sqlite3_expr_list_delete(db, p_list) };
                        __state = 378;
                    }
                    378 => {
                        unsafe { sqlite3_upsert_delete(db, p_upsert) };
                        __state = 379;
                    }
                    379 => {
                        unsafe { sqlite3_select_delete(db, p_select) };
                        __state = 380;
                    }
                    380 => {
                        if !(p_column).is_null() {
                            __state = 382;
                        } else { __state = 381; }
                    }
                    381 => {
                        if !(a_reg_idx).is_null() {
                            __state = 384;
                        } else { __state = 1; }
                    }
                    382 => {
                        unsafe { sqlite3_id_list_delete(db, p_column) };
                        __state = 383;
                    }
                    383 => {
                        unsafe { sqlite3_db_free(db, a_tab_col_map as *mut ()) };
                        __state = 381;
                    }
                    384 => {
                        unsafe { sqlite3_db_nn_free_nn(db, a_reg_idx as *mut ()) };
                        __state = 1;
                    }
                    _ => {}
                }
            }
        }
    }
}
extern "C" fn compute_index_aff_str(db: *mut Sqlite3, p_idx_1: &mut Index)
    -> *const i8 {
    let mut n: i32 = 0;
    let p_tab: *const Table = (*p_idx_1).p_table as *const Table;
    (*p_idx_1).z_col_aff =
        unsafe {
                sqlite3_db_malloc_raw(core::ptr::null_mut(),
                    ((*p_idx_1).n_column as i32 + 1) as u64)
            } as *mut i8;
    if ((*p_idx_1).z_col_aff).is_null() as i32 != 0 {
        unsafe { sqlite3_oom_fault(db) };
        return core::ptr::null();
    }
    {
        n = 0;
        '__b41: loop {
            if !(n < (*p_idx_1).n_column as i32) { break '__b41; }
            '__c41: loop {
                let x: i16 =
                    unsafe { *(*p_idx_1).ai_column.offset(n as isize) };
                let mut aff: i8 = 0 as i8;
                if x as i32 >= 0 {
                    aff =
                        unsafe {
                            (*unsafe { (*p_tab).a_col.offset(x as isize) }).affinity
                        };
                } else if x as i32 == -1 {
                    aff = 68 as i8;
                } else {
                    { let _ = 0; };
                    { let _ = 0; };
                    { let _ = 0; };
                    aff =
                        unsafe {
                            sqlite3_expr_affinity(unsafe {
                                        (*(unsafe { (*(*p_idx_1).a_col_expr).a.as_ptr() } as
                                                        *mut ExprListItem).offset(n as isize)).p_expr
                                    } as *const Expr)
                        };
                }
                if (aff as i32) < 65 { aff = 65 as i8; }
                if aff as i32 > 67 { aff = 67 as i8; }
                unsafe { *(*p_idx_1).z_col_aff.offset(n as isize) = aff };
                break '__c41;
            }
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *(*p_idx_1).z_col_aff.offset(n as isize) = 0 as i8 };
    return (*p_idx_1).z_col_aff as *const i8;
}
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_index_affinity_str(db: *mut Sqlite3,
    p_idx: *mut Index) -> *const i8 {
    if (unsafe { (*p_idx).z_col_aff }).is_null() as i32 != 0 {
        return compute_index_aff_str(db, unsafe { &mut *p_idx });
    }
    return unsafe { (*p_idx).z_col_aff } as *const i8;
}
static i_ln_1: i32 = 0 as i32;
static auto_inc: [VdbeOpList; 12] =
    [VdbeOpList { opcode: 77 as u8, p1: 0 as i8, p2: 0 as i8, p3: 0 as i8 },
            VdbeOpList {
                opcode: 36 as u8,
                p1: 0 as i8,
                p2: 10 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 96 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 53 as u8,
                p1: 0 as i8,
                p2: 9 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 137 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 96 as u8,
                p1: 0 as i8,
                p2: 1 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 88 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 82 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 9 as u8,
                p1: 0 as i8,
                p2: 11 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 40 as u8,
                p1: 0 as i8,
                p2: 2 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 73 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 124 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            }];
static i_ln_2: i32 = 0 as i32;
static auto_inc_end: [VdbeOpList; 5] =
    [VdbeOpList { opcode: 52 as u8, p1: 0 as i8, p2: 2 as i8, p3: 0 as i8 },
            VdbeOpList {
                opcode: 129 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 99 as u8,
                p1: 0 as i8,
                p2: 2 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 130 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
            },
            VdbeOpList {
                opcode: 124 as u8,
                p1: 0 as i8,
                p2: 0 as i8,
                p3: 0 as i8,
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
    fn sqlite3_expr_is_constant(_: *mut Parse, _: *mut Expr)
    -> i32;
    fn sqlite3_expr_affinity(p_expr_1: *const Expr)
    -> i8;
    fn sqlite3_select_new(_: *mut Parse, _: *mut ExprList, _: *mut SrcList,
    _: *mut Expr, _: *mut ExprList, _: *mut Expr, _: *mut ExprList, _: u32,
    _: *mut Expr)
    -> *mut Select;
    fn sqlite3_get_vdbe(_: *mut Parse)
    -> *mut Vdbe;
    fn sqlite3_read_schema(p_parse_1: *mut Parse)
    -> i32;
    fn sqlite3_src_item_attach_subquery(_: *mut Parse, _: *mut SrcItem,
    _: *mut Select, _: i32)
    -> i32;
    fn sqlite3_select_dest_init(_: *mut SelectDest, _: i32, _: i32)
    -> ();
    fn sqlite3_select(_: *mut Parse, _: *mut Select, _: *mut SelectDest)
    -> i32;
    fn sqlite3_select_wrong_num_terms_error(p_parse_1: *mut Parse,
    p: *mut Select)
    -> ();
    fn sqlite3_expr_code_expr_list(_: *mut Parse, _: *mut ExprList, _: i32,
    _: i32, _: u8)
    -> i32;
    fn sqlite3_expr_list_delete(_: *mut Sqlite3, _: *mut ExprList)
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
    fn sqlite3_table_lock(_: *mut Parse, _: i32, _: Pgno, _: u8, _: *const i8)
    -> ();
    fn sqlite3_src_list_delete(_: *mut Sqlite3, _: *mut SrcList)
    -> ();
    fn sqlite3_select_delete(_: *mut Sqlite3, _: *mut Select)
    -> ();
    fn sqlite3_src_list_lookup(_: *mut Parse, _: *mut SrcList)
    -> *mut Table;
    fn sqlite3_schema_to_index(db: *mut Sqlite3, _: *mut Schema)
    -> i32;
    fn sqlite3_auth_check(_: *mut Parse, _: i32, _: *const i8, _: *const i8,
    _: *const i8)
    -> i32;
    fn sqlite3_triggers_exist(_: *mut Parse, _: *mut Table, _: i32,
    _: *mut ExprList, p_mask_1: *mut i32)
    -> *mut Trigger;
    fn sqlite3_is_read_only(_: *mut Parse, _: *mut Table, _: *mut Trigger)
    -> i32;
    fn sqlite3_begin_write_operation(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_locate_table_item(_: *mut Parse, flags: u32, _: *mut SrcItem)
    -> *mut Table;
    fn sqlite3_expr_compare(_: *const Parse, _: *const Expr, _: *const Expr,
    _: i32)
    -> i32;
    fn sqlite3_expr_list_compare(_: *const ExprList, _: *const ExprList,
    _: i32)
    -> i32;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn sqlite3_auth_read_col(_: *mut Parse, _: *const i8, _: *const i8,
    _: i32)
    -> i32;
    fn sqlite3_code_verify_schema(_: *mut Parse, _: i32)
    -> ();
    fn sqlite3_parser_add_cleanup(_: *mut Parse,
    _: Option<unsafe extern "C" fn(*mut Sqlite3, *mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_rowid_constraint(_: *mut Parse, _: i32, _: *mut Table)
    -> ();
    static sqlite3_str_binary: [i8; 0];
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn sqlite3_column_index(p_tab_1: *mut Table, z_col_1: *const i8)
    -> i32;
    fn sqlite3_is_rowid(_: *const i8)
    -> i32;
    fn sqlite3_get_v_table(_: *mut Sqlite3, _: *mut Table)
    -> *mut VTable;
    fn sqlite3_resolve_expr_list_names(_: *mut NameContext, _: *mut ExprList)
    -> i32;
    fn sqlite3_has_explicit_nulls(_: *mut Parse, _: *mut ExprList)
    -> i32;
    fn sqlite3_upsert_analyze_target(_: *mut Parse, _: *mut SrcList,
    _: *mut Upsert, _: *mut Upsert)
    -> i32;
    fn sqlite3_expr_code_factorable(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_code_target(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_code(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_oom_fault(_: *mut Sqlite3)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn sqlite3_expr_code_generated_column(_: *mut Parse, _: *mut Table,
    _: *mut Column, _: i32)
    -> ();
    fn sqlite3_code_row_trigger(_: *mut Parse, _: *mut Trigger, _: i32,
    _: *mut ExprList, _: i32, _: *mut Table, _: i32, _: i32, _: i32)
    -> ();
    fn sqlite3_vtab_make_writable(_: *mut Parse, _: *mut Table)
    -> ();
    fn sqlite3_may_abort(_: *mut Parse)
    -> ();
    fn sqlite3_expr_code_copy(_: *mut Parse, _: *mut Expr, _: i32)
    -> ();
    fn sqlite3_expr_dup(_: *mut Sqlite3, _: *const Expr, _: i32)
    -> *mut Expr;
    fn sqlite3_expr_if_true(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_halt_constraint(_: *mut Parse, _: i32, _: i32, _: *mut i8,
    _: i8, _: u8)
    -> ();
    fn sqlite3_fk_required(_: *mut Parse, _: *mut Table, _: *mut i32, _: i32)
    -> i32;
    fn sqlite3_upsert_of_index(_: *mut Upsert, _: *mut Index)
    -> *mut Upsert;
    fn sqlite3_multi_write(_: *mut Parse)
    -> ();
    fn sqlite3_generate_row_delete(_: *mut Parse, _: *mut Table,
    _: *mut Trigger, _: i32, _: i32, _: i32, _: i16, _: u8, _: u8, _: u8,
    _: i32)
    -> ();
    fn sqlite3_generate_row_index_delete(_: *mut Parse, _: *mut Table, _: i32,
    _: i32, _: *mut i32, _: i32)
    -> ();
    fn sqlite3_upsert_do_update(_: *mut Parse, _: *mut Upsert, _: *mut Table,
    _: *mut Index, _: i32)
    -> ();
    fn sqlite3_expr_if_false_dup(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
    -> ();
    fn sqlite3_fk_references(_: *mut Table)
    -> *mut FKey;
    fn sqlite3_locate_coll_seq(p_parse_1: *mut Parse, z_name_1: *const i8)
    -> *mut CollSeq;
    fn sqlite3_unique_constraint(_: *mut Parse, _: i32, _: *mut Index)
    -> ();
    static sqlite3_opcode_property: [u8; 0];
    fn sqlite3_upsert_next_is_ipk(_: *mut Upsert)
    -> i32;
    fn sqlite3_fk_check(_: *mut Parse, _: *mut Table, _: i32, _: i32,
    _: *mut i32, _: i32)
    -> ();
    fn sqlite3_code_change_count(_: *mut Vdbe, _: i32, _: *const i8)
    -> ();
    fn sqlite3_upsert_delete(_: *mut Sqlite3, _: *mut Upsert)
    -> ();
    fn sqlite3_id_list_delete(_: *mut Sqlite3, _: *mut IdList)
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
    fn sqlite3_select_delete_generic(_: *mut Sqlite3, _: *mut ())
    -> ();
    fn sqlite3_select_check_on_clauses(p_parse_1: *mut Parse,
    p_select_1: *mut Select)
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
    fn sqlite3_expr_code_run_just_once(_: *mut Parse, _: *mut Expr, _: i32)
    -> i32;
    fn sqlite3_expr_null_register_range(_: *mut Parse, _: i32, _: i32)
    -> ();
    fn sqlite3_expr_code_temp(_: *mut Parse, _: *mut Expr, _: *mut i32)
    -> i32;
    fn sqlite3_expr_if_false(_: *mut Parse, _: *mut Expr, _: i32, _: i32)
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
    fn sqlite3_expr_compare_skip(_: *mut Expr, _: *mut Expr, _: i32)
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
    fn sqlite3_expr_is_integer(_: *const Expr, _: *mut i32, _: *mut Parse)
    -> i32;
    fn sqlite3_expr_can_be_null(_: *const Expr)
    -> i32;
    fn sqlite3_expr_needs_no_affinity_change(_: *const Expr, _: i8)
    -> i32;
    fn sqlite3_expr_is_like_operator(_: *const Expr)
    -> i32;
    fn sqlite3_rowid_alias(p_tab_1: *mut Table)
    -> *const i8;
    fn sqlite3_generate_index_key(_: *mut Parse, _: *mut Index, _: i32,
    _: i32, _: i32, _: *mut i32, _: *mut Index, _: i32)
    -> i32;
    fn sqlite3_resolve_part_idx_label(_: *mut Parse, _: i32)
    -> ();
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
    fn sqlite3_trigger_list(_: *mut Parse, _: *mut Table)
    -> *mut Trigger;
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
    fn sqlite3_auth_context_push(_: *mut Parse, _: *mut AuthContext,
    _: *const i8)
    -> ();
    fn sqlite3_auth_context_pop(_: *mut AuthContext)
    -> ();
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
    fn sqlite3_two_part_name(_: *mut Parse, _: *mut Token, _: *mut Token,
    _: *mut *mut Token)
    -> i32;
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
    fn sqlite3_match_e_name(_: *const ExprListItem, _: *const i8,
    _: *const i8, _: *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_expr_col_used(_: *mut Expr)
    -> Bitmask;
    fn sqlite3_str_i_hash(_: *const i8)
    -> u8;
    fn sqlite3_resolve_expr_names(_: *mut NameContext, _: *mut Expr)
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
    fn sqlite3_upsert_dup(_: *mut Sqlite3, _: *mut Upsert)
    -> *mut Upsert;
    fn sqlite3_fk_drop_table(_: *mut Parse, _: *mut SrcList, _: *mut Table)
    -> ();
    fn sqlite3_fk_actions(_: *mut Parse, _: *mut Table, _: *mut ExprList,
    _: i32, _: *mut i32, _: i32)
    -> ();
    fn sqlite3_fk_oldmask(_: *mut Parse, _: *mut Table)
    -> u32;
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
#[repr(C)]
#[derive(Copy, Clone)]
struct WindowRewrite {
    _opaque: [u8; 0],
}